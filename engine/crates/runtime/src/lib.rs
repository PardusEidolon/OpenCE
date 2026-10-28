//! The game's runtime: `source/` built as `game.wasm` (tools/engine_wasm.py),
//! compiled by Wasmtime with Cranelift and run inside the macOS app.
//!
//! A [`Runtime`] holds the engine and, when given a folder, Wasmtime's cache
//! of compiled modules, keyed by Wasmtime's version, the compiler's settings
//! and the CPU's features, so the game compiles once per machine and build.
//! [`Runtime::load`] instantiates a module with a stub for every function it
//! imports: each stub records its call and returns zeros, until the platform
//! supplies the real ones. `proc_exit` ends the call with [`Error::Exit`],
//! and `fd_write` keeps what is written, per descriptor ([`Game::output`]),
//! so the reference harnesses run as WASI commands ([`Game::run_command`]).
//! A call budget ([`Game::set_call_budget`]) ends a game that, on stubs,
//! never returns.
//!
//! The module and its memory are untrusted: a trap is an error, never a
//! panic, and the memory is read and written only through checked ranges.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

mod audio;
pub mod benchmark;
mod config;
mod controls;
mod d3d;
pub mod files;
mod input;
pub mod memory;
mod net;
mod profile;
mod resources;
mod space;
mod system;
mod textures;
mod threads;

use space::Space;
use wasmtime::{
    Cache, CacheConfig, Caller, Config, Engine, Extern, ExternType, Instance, Linker, Memory,
    Module, SharedMemory, Store,
};

/// a wasm value, as the game's functions take and return them
pub use wasmtime::Val;

pub use audio::{Speakers, SPEAKER_RATE, SPEAKER_TARGET_FRAMES};
pub use controls::{keystroke_flags, virtual_key, Controls};
pub use profile::{Span, SLEEP};

/// the import that ends the game, with its exit code
const PROC_EXIT: &str = "wasi_snapshot_preview1.proc_exit";

/// the import that writes to a descriptor, kept in [`Record::outputs`]
const FD_WRITE: &str = "wasi_snapshot_preview1.fd_write";

/// WASI's errno for an address outside the memory
const EFAULT: i32 = 21;

/// WASI's errno for a count too large for its type
const EOVERFLOW: i32 = 61;

/// the calls a game keeps until [`Game::set_record_limit`] says otherwise
const DEFAULT_RECORD_LIMIT: usize = 65_536;

/// the address space a 32-bit module can reach: the whole of it is reserved,
/// so the Xbox's window (0x80000000 to 0xA0000000) needs no bounds checks
const MEMORY_RESERVATION: u64 = 1 << 32;

/// what went wrong
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// the game called `proc_exit` with this code
    #[error("the game exited with code {0}")]
    Exit(i32),
    /// the game made as many import calls as its budget allowed
    #[error("the game used its budget of {0} import calls")]
    Budget(u64),
    /// the game made an import call after its deadline
    #[error("the game ran past its deadline")]
    Deadline,
    /// the game trapped (or a stub failed)
    #[error("the game trapped: {0}")]
    Trap(String),
    /// the module exports no function of this name
    #[error("the module exports no function {0}")]
    NoExport(String),
    /// the module could not be compiled
    #[error("the module could not be compiled: {0}")]
    Compile(String),
    /// the module imports something the runtime cannot supply
    #[error("the module imports {0}, which the runtime cannot supply")]
    Import(String),
    /// a read or write outside the memory
    #[error("{len} bytes at {address:#x} are outside the memory of {size} bytes")]
    OutOfBounds {
        /// where the access began
        address: u64,
        /// how many bytes it covered
        len: u64,
        /// the memory's size
        size: u64,
    },
    /// the export returned something other than what was asked for
    #[error("{0} returned {1}")]
    Result(String, String),
    /// Wasmtime could not be set up
    #[error("the runtime could not be set up: {0}")]
    Setup(String),
    /// the memory could not grow to this many bytes
    #[error("the memory could not grow to {0:#x} bytes")]
    Grow(u64),
}

/// a value passed to an import; floats are kept as their bits, so that a
/// recorded call compares exactly, NaNs included
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// an i32
    I32(i32),
    /// an i64
    I64(i64),
    /// an f32's bits
    F32(u32),
    /// an f64's bits
    F64(u64),
    /// a v128's bits
    V128(u128),
    /// a reference of any kind
    Ref,
}

impl From<&Val> for Value {
    fn from(value: &Val) -> Self {
        match value {
            Val::I32(v) => Self::I32(*v),
            Val::I64(v) => Self::I64(*v),
            Val::F32(bits) => Self::F32(*bits),
            Val::F64(bits) => Self::F64(*bits),
            Val::V128(v) => Self::V128(v.as_u128()),
            _ => Self::Ref,
        }
    }
}

/// one recorded call of an import
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// the import's index in the module's imports ([`Game::import_name`])
    pub import: usize,
    /// its arguments
    pub args: Vec<Value>,
}

/// the exit code `proc_exit` carries out of the game as a trap
#[derive(Debug)]
struct Exit(i32);

impl std::fmt::Display for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "proc_exit({})", self.0)
    }
}

impl std::error::Error for Exit {}

/// the call budget, carried out of the game as a trap
#[derive(Debug)]
struct Budget(u64);

impl std::fmt::Display for Budget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a budget of {} import calls", self.0)
    }
}

impl std::error::Error for Budget {}

/// the deadline, carried out of the game as a trap
#[derive(Debug)]
struct Deadline;

impl std::fmt::Display for Deadline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a deadline")
    }
}

impl std::error::Error for Deadline {}

/// the game's memory: its own, or shared between its threads
#[derive(Clone)]
enum Mem {
    Own(Memory),
    Shared(SharedMemory),
}

/// what the game's threads share
struct Shared {
    /// the Xbox's contiguous window, as `XPhysicalAlloc` hands it out
    window: Mutex<memory::Window>,
    /// the game's folders, open files and last error
    files: Mutex<files::Files>,
    /// the game's threads, the objects they wait on and the clocks
    threads: threads::Threads,
    /// Direct3D's device, once the game has created it
    d3d: Mutex<d3d::Device>,
    /// the game's settings, as it reads and writes them
    settings: Mutex<config::Settings>,
    /// the keyboard and mouse, and what the game has opened of them
    input: Mutex<input::Input>,
    /// the mixed sound, on its way to the speakers
    audio: audio::Output,
    /// the profile sections' spans, while profiling is on
    profile: Mutex<profile::Profile>,
}

/// the record the stubs keep, in the store
struct Record {
    /// each import's "module.name", in the module's order
    names: Vec<String>,
    /// how many times each import was called
    counts: Vec<u64>,
    /// the first calls, up to `limit`
    calls: Vec<Call>,
    limit: usize,
    /// the import calls made so far, and how many may be made
    made: u64,
    budget: Option<u64>,
    /// when the game's import calls end ([`Game::set_deadline`])
    deadline: Option<std::time::Instant>,
    /// what `fd_write` wrote, per descriptor
    outputs: BTreeMap<i32, Vec<u8>>,
    /// the game's memory, once there is one (until then, its export)
    memory: Option<Mem>,
    /// what this thread shares with the game's others
    shared: Arc<Shared>,
    /// this thread's id (0 for the game's first)
    thread: u32,
    /// the handle of the thread `CreateThread` is starting
    pending: Option<u32>,
}

impl Record {
    /// a record for the game's thread `thread`, sharing this one's memory
    /// and state (the thread's own calls are not counted or budgeted)
    fn for_thread(&self, thread: u32) -> Self {
        Self {
            names: self.names.clone(),
            counts: vec![0; self.names.len()],
            calls: Vec::new(),
            limit: 0,
            made: 0,
            budget: None,
            deadline: None,
            outputs: BTreeMap::new(),
            memory: self.memory.clone(),
            shared: Arc::clone(&self.shared),
            thread,
            pending: None,
        }
    }

    /// the Xbox's window, held while the guard lives
    fn window(&self) -> MutexGuard<'_, memory::Window> {
        self.shared
            .window
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// the game's files, held while the guard lives
    fn files(&self) -> MutexGuard<'_, files::Files> {
        self.shared
            .files
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// Direct3D's device, held while the guard lives
    fn device(&self) -> MutexGuard<'_, d3d::Device> {
        self.shared
            .d3d
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// the game's settings, held while the guard lives
    fn settings(&self) -> MutexGuard<'_, config::Settings> {
        self.shared
            .settings
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// the keyboard and mouse, held while the guard lives
    fn input(&self) -> MutexGuard<'_, input::Input> {
        self.shared
            .input
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// the profile sections, held while the guard lives
    fn profile(&self) -> MutexGuard<'_, profile::Profile> {
        self.shared
            .profile
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

/// runs `use_space` on the caller's memory, with its record beside it
///
/// # Errors
/// If the game has no memory yet and exports none.
fn with_space<R>(
    caller: &mut Caller<'_, Record>,
    use_space: impl FnOnce(&mut Space<'_>, &mut Record) -> R,
) -> wasmtime::Result<R> {
    let memory = match caller.data().memory.clone() {
        Some(memory) => memory,
        None => caller
            .get_export("memory")
            .and_then(Extern::into_memory)
            .map(Mem::Own)
            .ok_or_else(|| wasmtime::format_err!("the game has no memory"))?,
    };
    Ok(match memory {
        Mem::Own(memory) => {
            let (data, record) = memory.data_and_store_mut(&mut *caller);
            use_space(&mut Space::own(data), record)
        }
        Mem::Shared(memory) => use_space(&mut Space::shared(memory.data()), caller.data_mut()),
    })
}

/// `fd_write(fd, iovs, count, written)`: gathers the `count` iovecs at
/// `iovs` from the caller's memory, stores how many bytes they hold at
/// `written` and keeps them; returns WASI's errno, [`EFAULT`] (with nothing
/// written) if any of it is outside the memory
fn fd_write(caller: &mut Caller<'_, Record>, params: &[Val]) -> i32 {
    let [Val::I32(fd), Val::I32(iovs), Val::I32(count), Val::I32(written)] = params else {
        return EFAULT;
    };
    // (the module's pointers are unsigned)
    let (iovs, count, written) = (
        iovs.cast_unsigned(),
        count.cast_unsigned(),
        written.cast_unsigned(),
    );
    let gathered = with_space(caller, |space, _| {
        let mut bytes = Vec::new();
        for index in 0..count {
            let base = index
                .checked_mul(8)
                .and_then(|offset| iovs.checked_add(offset))
                .ok_or(EFAULT)?;
            let chunk = space
                .get(base)
                .zip(base.checked_add(4).and_then(|at| space.get(at)))
                .and_then(|(address, len)| space.read(address, len))
                .ok_or(EFAULT)?;
            bytes.extend(chunk);
        }
        let total = u32::try_from(bytes.len()).map_err(|_| EOVERFLOW)?;
        if !space.put(written, total) {
            return Err(EFAULT);
        }
        Ok(bytes)
    });
    match gathered {
        Ok(Ok(bytes)) => {
            caller
                .data_mut()
                .outputs
                .entry(*fd)
                .or_default()
                .extend(bytes);
            0
        }
        Ok(Err(errno)) => errno,
        Err(_) => EFAULT,
    }
}

/// Whether the platform answers the import "module.name" (rather than
/// leaving it to a stub that returns zeros).
#[must_use]
pub fn answers(name: &str) -> bool {
    name == PROC_EXIT
        || name == FD_WRITE
        || threads::handler(name).is_some()
        || memory::handler(name).is_some()
        || system::handler(name).is_some()
        || d3d::handler(name).is_some()
        || resources::handler(name).is_some()
        || textures::handler(name).is_some()
        || config::handler(name).is_some()
        || input::handler(name).is_some()
        || audio::handler(name).is_some()
        || net::handler(name).is_some()
        || profile::handler(name).is_some()
        || files::service(name).is_some()
}

/// Wasmtime, set up for the game
pub struct Runtime {
    engine: Engine,
    cache: Option<Cache>,
}

impl Runtime {
    /// A runtime that caches compiled modules in `cache` (none if `None`).
    ///
    /// # Errors
    /// [`Error::Setup`] if Wasmtime or its cache cannot be set up.
    pub fn new(cache: Option<&Path>) -> Result<Self, Error> {
        let mut config = Config::new();
        config
            .memory_reservation(MEMORY_RESERVATION)
            .wasm_threads(true)
            .shared_memory(true)
            // the simulation must be deterministic: every NaN the game makes
            // has the same bits on every machine
            .cranelift_nan_canonicalization(true)
            // and relaxed SIMD, whose results may differ by machine (a fused
            // multiply-add or not), is refused: the game is built without it
            .wasm_relaxed_simd(false);
        let cache = match cache {
            Some(folder) => {
                let mut settings = CacheConfig::new();
                settings.with_directory(folder);
                Some(Cache::new(settings).map_err(|error| Error::Setup(format!("{error:#}")))?)
            }
            None => None,
        };
        config.cache(cache.clone());
        let engine = Engine::new(&config).map_err(|error| Error::Setup(format!("{error:#}")))?;
        Ok(Self { engine, cache })
    }

    /// Wasmtime's engine, as the runtime set it up: what the platform layer
    /// shares between the game's threads, with their shared memory.
    #[must_use]
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// How many times a compiled module was found in the cache.
    #[must_use]
    pub fn cache_hits(&self) -> usize {
        self.cache.as_ref().map_or(0, Cache::cache_hits)
    }

    /// Compiles `wasm` (or finds it in the cache) and instantiates it with a
    /// recording stub for every function it imports.
    ///
    /// # Errors
    /// [`Error::Compile`] if the module is malformed or invalid,
    /// [`Error::Import`] if it imports anything but functions and one
    /// shared memory, and
    /// [`Error::Trap`] if instantiating it traps.
    pub fn load(&self, wasm: &[u8]) -> Result<Game, Error> {
        let module = Module::new(&self.engine, wasm)
            .map_err(|error| Error::Compile(format!("{error:#}")))?;
        let mut linker = Linker::new(&self.engine);
        let mut names = Vec::new();
        // (a threaded game imports its memory, shared, and exports none)
        let mut shared = None;
        for (index, import) in module.imports().enumerate() {
            let name = format!("{}.{}", import.module(), import.name());
            let ty = match import.ty() {
                ExternType::Func(ty) => ty,
                ExternType::Memory(ty) if ty.is_shared() && shared.is_none() => {
                    let memory = SharedMemory::new(&self.engine, ty)
                        .map_err(|error| Error::Import(format!("{name}: {error:#}")))?;
                    shared = Some((
                        import.module().to_string(),
                        import.name().to_string(),
                        memory,
                    ));
                    names.push(name);
                    continue;
                }
                _ => return Err(Error::Import(name)),
            };
            let exits = name == PROC_EXIT;
            let writes = name == FD_WRITE;
            let threads = threads::handler(&name);
            let serves = memory::handler(&name)
                .or_else(|| system::handler(&name))
                .or_else(|| d3d::handler(&name))
                .or_else(|| resources::handler(&name))
                .or_else(|| textures::handler(&name))
                .or_else(|| config::handler(&name))
                .or_else(|| input::handler(&name))
                .or_else(|| audio::handler(&name))
                .or_else(|| net::handler(&name))
                .or_else(|| profile::handler(&name));
            let files_service = files::service(&name);
            linker
                .func_new(
                    import.module(),
                    import.name(),
                    ty.clone(),
                    move |mut caller: Caller<'_, Record>, params, results| {
                        let record: &mut Record = caller.data_mut();
                        if let Some(budget) = record.budget {
                            if record.made >= budget {
                                return Err(wasmtime::Error::new(Budget(budget)));
                            }
                        }
                        if record
                            .deadline
                            .is_some_and(|deadline| std::time::Instant::now() >= deadline)
                        {
                            return Err(wasmtime::Error::new(Deadline));
                        }
                        record.made += 1;
                        if let Some(count) = record.counts.get_mut(index) {
                            *count += 1;
                        }
                        if record.calls.len() < record.limit {
                            record.calls.push(Call {
                                import: index,
                                args: params.iter().map(Value::from).collect(),
                            });
                        }
                        if exits {
                            let code = match params.first() {
                                Some(Val::I32(code)) => *code,
                                _ => -1,
                            };
                            return Err(wasmtime::Error::new(Exit(code)));
                        }
                        if writes {
                            let errno = fd_write(&mut caller, params);
                            if let Some(result) = results.first_mut() {
                                *result = Val::I32(errno);
                            }
                            return Ok(());
                        }
                        if let Some(serve) = threads {
                            return serve(&mut caller, params, results);
                        }
                        if let Some(serve) = serves {
                            return serve(&mut caller, params, results);
                        }
                        if let Some(service) = files_service {
                            return files::serve(&mut caller, service, params, results);
                        }
                        for (result, ty) in results.iter_mut().zip(ty.results()) {
                            *result = Val::default_for_ty(&ty).ok_or_else(|| {
                                wasmtime::format_err!("no zero value for a result of type {ty}")
                            })?;
                        }
                        Ok(())
                    },
                )
                .map_err(|error| Error::Import(format!("{name}: {error:#}")))?;
            names.push(name);
        }
        let counts = vec![0; names.len()];
        let mut store = Store::new(
            &self.engine,
            Record {
                names,
                counts,
                calls: Vec::new(),
                limit: DEFAULT_RECORD_LIMIT,
                made: 0,
                budget: None,
                deadline: None,
                outputs: BTreeMap::new(),
                memory: shared
                    .as_ref()
                    .map(|(_, _, memory)| Mem::Shared(memory.clone())),
                shared: Arc::new(Shared {
                    window: Mutex::new(memory::Window::new()),
                    files: Mutex::new(files::Files::default()),
                    threads: threads::Threads::new(),
                    d3d: Mutex::new(d3d::Device::default()),
                    settings: Mutex::new(config::Settings::new(|name| std::env::var(name).ok())),
                    input: Mutex::new(input::Input::default()),
                    audio: audio::Output::default(),
                    profile: Mutex::new(profile::Profile::default()),
                }),
                thread: 0,
                pending: None,
            },
        );
        if let Some((module_name, name, memory)) = &shared {
            linker
                .define(&store, module_name, name, memory.clone())
                .map_err(|error| Error::Import(format!("{module_name}.{name}: {error:#}")))?;
        }
        let instance = if shared.is_some() {
            // (each new thread instantiates the game again, over its memory)
            let pre = linker.instantiate_pre(&module).map_err(called)?;
            store.data().shared.threads.set_spawner(pre.clone());
            pre.instantiate(&mut store).map_err(called)?
        } else {
            linker.instantiate(&mut store, &module).map_err(called)?
        };
        let memory = match shared {
            Some((_, _, memory)) => Some(Mem::Shared(memory)),
            None => instance.get_memory(&mut store, "memory").map(Mem::Own),
        };
        store.data_mut().memory.clone_from(&memory);
        let functions = module
            .exports()
            .filter(|export| matches!(export.ty(), ExternType::Func(_)))
            .map(|export| export.name().to_string())
            .collect();
        Ok(Game {
            store,
            instance,
            memory,
            functions,
        })
    }
}

impl Drop for Game {
    /// ends the waits of the game's threads, so they end with it
    fn drop(&mut self) {
        self.store.data().shared.threads.close();
    }
}

/// what a failed call means: the game's exit, or a trap
fn called(error: wasmtime::Error) -> Error {
    if let Some(Exit(code)) = error.downcast_ref::<Exit>() {
        return Error::Exit(*code);
    }
    if error.downcast_ref::<Deadline>().is_some() {
        return Error::Deadline;
    }
    match error.downcast_ref::<Budget>() {
        Some(Budget(budget)) => Error::Budget(*budget),
        None => Error::Trap(format!("{error:?}")),
    }
}

/// an instantiated module
pub struct Game {
    store: Store<Record>,
    instance: Instance,
    memory: Option<Mem>,
    /// the names of the functions it exports
    functions: Vec<String>,
}

impl Game {
    /// Calls the exported function `name` with no arguments and returns its
    /// results.
    ///
    /// # Errors
    /// [`Error::NoExport`] if there is no such function, [`Error::Exit`] if
    /// the game exits during the call and [`Error::Trap`] if it traps.
    pub fn call_results(&mut self, name: &str) -> Result<Vec<Val>, Error> {
        self.call_with(name, &[])
    }

    /// Calls the exported function `name` with `args` and returns its
    /// results.
    ///
    /// # Errors
    /// As [`Game::call_results`], and [`Error::Trap`] if `args` do not fit
    /// the function's parameters.
    pub fn call_with(&mut self, name: &str, args: &[Val]) -> Result<Vec<Val>, Error> {
        let func = self
            .instance
            .get_func(&mut self.store, name)
            .ok_or_else(|| Error::NoExport(name.to_string()))?;
        let ty = func.ty(&self.store);
        let mut results: Vec<Val> = ty
            .results()
            .map(|ty| Val::default_for_ty(&ty).unwrap_or(Val::I32(0)))
            .collect();
        func.call(&mut self.store, args, &mut results)
            .map_err(called)?;
        Ok(results)
    }

    /// Calls the exported function `name`, dropping any results.
    ///
    /// # Errors
    /// As [`Game::call_results`].
    pub fn call(&mut self, name: &str) -> Result<(), Error> {
        self.call_results(name).map(drop)
    }

    /// Calls the exported function `name`, which returns one i32.
    ///
    /// # Errors
    /// As [`Game::call_results`], and [`Error::Result`] if it returns
    /// anything else.
    pub fn call_i32(&mut self, name: &str) -> Result<i32, Error> {
        match self.call_results(name)?.as_slice() {
            [Val::I32(value)] => Ok(*value),
            other => Err(Error::Result(name.to_string(), format!("{other:?}"))),
        }
    }

    /// Runs the module as a WASI command: calls `_start`, and returns the
    /// code it gave `proc_exit`, or 0 if it returned.
    ///
    /// # Errors
    /// [`Error::NoExport`] if it has no `_start`, and [`Error::Trap`] if it
    /// traps.
    pub fn run_command(&mut self) -> Result<i32, Error> {
        match self.call("_start") {
            Ok(()) => Ok(0),
            Err(Error::Exit(code)) => Ok(code),
            Err(error) => Err(error),
        }
    }

    /// What the module has written to descriptor `fd` with `fd_write`.
    #[must_use]
    pub fn output(&self, fd: i32) -> &[u8] {
        self.store
            .data()
            .outputs
            .get(&fd)
            .map_or(&[], Vec::as_slice)
    }

    /// The calls recorded so far, oldest first, up to the record's limit.
    #[must_use]
    pub fn calls(&self) -> &[Call] {
        &self.store.data().calls
    }

    /// Keeps at most `limit` calls from now on (every call is still
    /// counted).
    pub fn set_record_limit(&mut self, limit: usize) {
        let record = self.store.data_mut();
        record.limit = limit;
        record.calls.truncate(limit);
    }

    /// Ends the game with [`Error::Budget`] when it makes an import call
    /// after `budget` of them (counted from the start; `None`, the default,
    /// for no budget).
    pub fn set_call_budget(&mut self, budget: Option<u64>) {
        self.store.data_mut().budget = budget;
    }

    /// Ends the game with [`Error::Deadline`] when it makes an import call
    /// at or after `deadline` (`None`, the default, for none): a benchmark
    /// runs the game for a time, not a count.
    pub fn set_deadline(&mut self, deadline: Option<std::time::Instant>) {
        self.store.data_mut().deadline = deadline;
    }

    /// Turns the timing of the game's profile sections on (from now) or
    /// off ([`profile`]'s module documentation).
    pub fn set_profiling(&mut self, on: bool) {
        self.store.data().profile().set(on);
    }

    /// The profile sections' spans ended since they were last taken, in
    /// the order they ended.
    pub fn take_profile_spans(&mut self) -> Vec<Span> {
        self.store.data().profile().take()
    }

    /// Sets the game's data folder (`d:\`, the maps) and save folder (the
    /// other drives); until then no path is found.
    pub fn set_folders(&mut self, data: &std::path::Path, saves: &std::path::Path) {
        self.store.data().files().set_folders(data, saves);
    }

    /// Sets the console commands the game runs as it starts, as its
    /// `d:\init.txt` ([`files::Files::set_init_commands`]).
    pub fn set_init_commands(&mut self, text: &str) {
        self.store.data().files().set_init_commands(text);
    }

    /// Makes the game ready to run `halo_game_main`: its data and save
    /// folders ([`Game::set_folders`]), its memory grown past the Xbox's
    /// window, where the game state and the tag cache are placed
    /// (`memory::WINDOW_BASE`), and the display its screen is shown on
    /// ([`Game::set_display`]).
    ///
    /// # Errors
    /// [`Error::Grow`] if the memory cannot grow past the window.
    pub fn prepare(
        &mut self,
        data: &std::path::Path,
        saves: &std::path::Path,
        display: (u32, u32),
    ) -> Result<(), Error> {
        self.set_folders(data, saves);
        self.grow_memory_to(u64::from(memory::WINDOW_BASE) + u64::from(memory::WINDOW_SIZE))?;
        self.set_display(display.0, display.1);
        Ok(())
    }

    /// Sets the display the game's screen is shown on, in pixels: the
    /// screen takes its shape (until then, the Xbox's 640 by 480).
    pub fn set_display(&mut self, width: u32, height: u32) {
        self.store.data().device().display = Some((width, height));
    }

    /// A key going down or up: its Windows virtual key, its character (0
    /// if none) and its keystroke flags (shift, control, alt and so on).
    pub fn key(&mut self, vk: u8, ascii: u8, flags: u8, down: bool) {
        self.store.data().input().key(vk, ascii, flags, down);
    }

    /// The mouse moving by `dx` and `dy` pixels, right and down.
    pub fn mouse_motion(&mut self, dx: f32, dy: f32) {
        self.store.data().input().motion(dx, dy);
    }

    /// A mouse button going down or up, numbered as SDL numbers them: 1
    /// the left, 2 the middle, 3 the right, 4 and on the side buttons.
    pub fn mouse_button(&mut self, button: u8, down: bool) {
        self.store.data().input().button(button, down);
    }

    /// The mouse's wheel turning by `notches`, up positive.
    pub fn mouse_wheel(&mut self, notches: f32) {
        self.store.data().input().wheel(notches);
    }

    /// The screenshots the player has asked for since the last call.
    pub fn take_screenshots(&mut self) -> u32 {
        self.store.data().input().take_screenshots()
    }

    /// The game's keyboard and mouse, to hand input to from another thread
    /// while `halo_game_main` keeps this game busy.
    #[must_use]
    pub fn controls(&self) -> Controls {
        Controls::new(Arc::clone(&self.store.data().shared))
    }

    /// The game's speakers, to pull its mixed sound from on the audio
    /// device's thread while `halo_game_main` keeps this game busy.
    #[must_use]
    pub fn speakers(&self) -> Speakers {
        Speakers::new(Arc::clone(&self.store.data().shared))
    }

    /// The texture and palette the game last set for the texture `stage`,
    /// or None past the device's four stages.
    #[must_use]
    pub fn stage(&self, stage: usize) -> Option<(u32, u32)> {
        let device = self.store.data().device();
        Some((*device.textures.get(stage)?, *device.palettes.get(stage)?))
    }

    /// What each of the game's threads that trapped said, oldest first.
    #[must_use]
    pub fn thread_faults(&self) -> Vec<String> {
        self.store.data().shared.threads.faults()
    }

    /// The completion routines `ReadFileEx` and `WriteFileEx` queued, oldest
    /// first, for the game's next alertable wait.
    #[must_use]
    pub fn completions(&self) -> Vec<files::Completion> {
        self.store.data().files().completions().to_vec()
    }

    /// The "module.name" of import `index` (empty if there is none).
    #[must_use]
    pub fn import_name(&self, index: usize) -> String {
        self.store
            .data()
            .names
            .get(index)
            .cloned()
            .unwrap_or_default()
    }

    /// How many functions the module imports.
    #[must_use]
    pub fn import_count(&self) -> usize {
        self.store.data().names.len()
    }

    /// How many times the import "module.name" was called.
    #[must_use]
    pub fn call_count(&self, name: &str) -> u64 {
        let record = self.store.data();
        record
            .names
            .iter()
            .position(|import| import == name)
            .and_then(|index| record.counts.get(index))
            .copied()
            .unwrap_or(0)
    }

    /// Whether the module exports a function `name`.
    #[must_use]
    pub fn has_export(&self, name: &str) -> bool {
        self.functions.iter().any(|function| function == name)
    }

    /// The size of the game's memory in bytes (0 if there is none).
    #[must_use]
    pub fn memory_size(&self) -> u64 {
        match &self.memory {
            Some(Mem::Own(memory)) => memory.data_size(&self.store) as u64,
            Some(Mem::Shared(memory)) => memory.data_size() as u64,
            None => 0,
        }
    }

    /// Grows the game's memory to at least `bytes` (rounded up to whole
    /// wasm pages); does nothing if it is that large already.
    ///
    /// # Errors
    /// [`Error::Grow`] if there is no memory, or it cannot grow that far
    /// (its maximum, or the host's).
    pub fn grow_memory_to(&mut self, bytes: u64) -> Result<(), Error> {
        match self.memory.clone().ok_or(Error::Grow(bytes))? {
            Mem::Own(memory) => {
                let want = bytes.div_ceil(memory.page_size(&self.store));
                let have = memory.size(&self.store);
                if want > have {
                    memory
                        .grow(&mut self.store, want - have)
                        .map_err(|_| Error::Grow(bytes))?;
                }
            }
            Mem::Shared(memory) => {
                let want = bytes.div_ceil(memory.page_size());
                let have = memory.size();
                if want > have {
                    memory.grow(want - have).map_err(|_| Error::Grow(bytes))?;
                }
            }
        }
        Ok(())
    }

    /// what an access of `len` bytes at `address` outside the memory is
    fn outside(&self, address: u64, len: u64) -> Error {
        Error::OutOfBounds {
            address,
            len,
            size: self.memory_size(),
        }
    }

    /// Reads `len` bytes of the memory at `address`.
    ///
    /// # Errors
    /// [`Error::OutOfBounds`] if any of them is outside the memory.
    pub fn read(&self, address: u64, len: u64) -> Result<Vec<u8>, Error> {
        let (Ok(at), Ok(count)) = (u32::try_from(address), u32::try_from(len)) else {
            return Err(self.outside(address, len));
        };
        let read = match &self.memory {
            Some(Mem::Own(memory)) => {
                let (start, end) = (at as usize, (at as usize).checked_add(count as usize));
                end.and_then(|end| memory.data(&self.store).get(start..end))
                    .map(<[u8]>::to_vec)
            }
            Some(Mem::Shared(memory)) => Space::shared(memory.data()).read(at, count),
            None => None,
        };
        read.ok_or_else(|| self.outside(address, len))
    }

    /// Writes `bytes` to the memory at `address`.
    ///
    /// # Errors
    /// [`Error::OutOfBounds`] if any of them would be outside the memory.
    pub fn write(&mut self, address: u64, bytes: &[u8]) -> Result<(), Error> {
        let len = bytes.len() as u64;
        let Ok(at) = u32::try_from(address) else {
            return Err(self.outside(address, len));
        };
        let written = match self.memory.clone() {
            Some(Mem::Own(memory)) => Space::own(memory.data_mut(&mut self.store)).write(at, bytes),
            Some(Mem::Shared(memory)) => Space::shared(memory.data()).write(at, bytes),
            None => false,
        };
        if written {
            Ok(())
        } else {
            Err(self.outside(address, len))
        }
    }
}
