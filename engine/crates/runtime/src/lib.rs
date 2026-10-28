//! The game's runtime: `source/` built as `game.wasm` (tools/engine_wasm.py),
//! compiled by Wasmtime with Cranelift and run inside the macOS app.
//!
//! A [`Runtime`] holds the engine and, when given a folder, Wasmtime's cache
//! of compiled modules, keyed by Wasmtime's version, the compiler's settings
//! and the CPU's features, so the game compiles once per machine and build.
//! [`Runtime::load`] instantiates a module with a stub for every function it
//! imports: each stub records its call and returns zeros, until the platform
//! supplies the real ones. `proc_exit` ends the call with [`Error::Exit`].
//!
//! The module and its memory are untrusted: a trap is an error, never a
//! panic, and the memory is read and written only through checked ranges.

use std::path::Path;

use wasmtime::{
    Cache, CacheConfig, Config, Engine, ExternType, Instance, Linker, Memory, Module, Store, Val,
};

/// the import that ends the game, with its exit code
const PROC_EXIT: &str = "wasi_snapshot_preview1.proc_exit";

/// the calls a game keeps until [`Game::set_record_limit`] says otherwise
const DEFAULT_RECORD_LIMIT: usize = 65_536;

/// the address space a 32-bit module can reach: the whole of it is reserved,
/// so the Xbox's window (0x80000000 to 0x88000000) needs no bounds checks
const MEMORY_RESERVATION: u64 = 1 << 32;

/// what went wrong
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// the game called `proc_exit` with this code
    #[error("the game exited with code {0}")]
    Exit(i32),
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

/// the record the stubs keep, in the store
struct Record {
    /// each import's "module.name", in the module's order
    names: Vec<String>,
    /// how many times each import was called
    counts: Vec<u64>,
    /// the first calls, up to `limit`
    calls: Vec<Call>,
    limit: usize,
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
            .cranelift_nan_canonicalization(true);
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
    /// [`Error::Import`] if it imports anything but functions, and
    /// [`Error::Trap`] if instantiating it traps.
    pub fn load(&self, wasm: &[u8]) -> Result<Game, Error> {
        let module = Module::new(&self.engine, wasm)
            .map_err(|error| Error::Compile(format!("{error:#}")))?;
        let mut linker = Linker::new(&self.engine);
        let mut names = Vec::new();
        for (index, import) in module.imports().enumerate() {
            let name = format!("{}.{}", import.module(), import.name());
            let ExternType::Func(ty) = import.ty() else {
                return Err(Error::Import(name));
            };
            let exits = name == PROC_EXIT;
            linker
                .func_new(
                    import.module(),
                    import.name(),
                    ty.clone(),
                    move |mut caller, params, results| {
                        let record: &mut Record = caller.data_mut();
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
            },
        );
        let instance = linker.instantiate(&mut store, &module).map_err(called)?;
        let memory = instance.get_memory(&mut store, "memory");
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

/// what a failed call means: the game's exit, or a trap
fn called(error: wasmtime::Error) -> Error {
    match error.downcast_ref::<Exit>() {
        Some(Exit(code)) => Error::Exit(*code),
        None => Error::Trap(format!("{error:?}")),
    }
}

/// an instantiated module
pub struct Game {
    store: Store<Record>,
    instance: Instance,
    memory: Option<Memory>,
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
        let func = self
            .instance
            .get_func(&mut self.store, name)
            .ok_or_else(|| Error::NoExport(name.to_string()))?;
        let ty = func.ty(&self.store);
        let mut results: Vec<Val> = ty
            .results()
            .map(|ty| Val::default_for_ty(&ty).unwrap_or(Val::I32(0)))
            .collect();
        func.call(&mut self.store, &[], &mut results)
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

    /// The size of the exported memory in bytes (0 if there is none).
    #[must_use]
    pub fn memory_size(&self) -> u64 {
        self.memory
            .map_or(0, |memory| memory.data_size(&self.store) as u64)
    }

    /// The memory's bytes from `address` to `address + len`, checked
    /// against its size.
    fn range(&self, address: u64, len: u64) -> Result<std::ops::Range<usize>, Error> {
        let size = self.memory_size();
        let outside = Error::OutOfBounds { address, len, size };
        let end = address
            .checked_add(len)
            .filter(|end| *end <= size)
            .ok_or(outside)?;
        let start =
            usize::try_from(address).map_err(|_| Error::OutOfBounds { address, len, size })?;
        let end = usize::try_from(end).map_err(|_| Error::OutOfBounds { address, len, size })?;
        Ok(start..end)
    }

    /// Reads `len` bytes of the memory at `address`.
    ///
    /// # Errors
    /// [`Error::OutOfBounds`] if any of them is outside the memory.
    pub fn read(&self, address: u64, len: u64) -> Result<Vec<u8>, Error> {
        let range = self.range(address, len)?;
        let memory = self.memory.ok_or(Error::OutOfBounds {
            address,
            len,
            size: 0,
        })?;
        memory
            .data(&self.store)
            .get(range)
            .map(<[u8]>::to_vec)
            .ok_or(Error::OutOfBounds {
                address,
                len,
                size: 0,
            })
    }

    /// Writes `bytes` to the memory at `address`.
    ///
    /// # Errors
    /// [`Error::OutOfBounds`] if any of them would be outside the memory.
    pub fn write(&mut self, address: u64, bytes: &[u8]) -> Result<(), Error> {
        let len = bytes.len() as u64;
        let range = self.range(address, len)?;
        let memory = self.memory.ok_or(Error::OutOfBounds {
            address,
            len,
            size: 0,
        })?;
        let target = memory
            .data_mut(&mut self.store)
            .get_mut(range)
            .ok_or(Error::OutOfBounds {
                address,
                len,
                size: 0,
            })?;
        target.copy_from_slice(bytes);
        Ok(())
    }
}
