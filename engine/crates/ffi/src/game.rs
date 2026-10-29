//! The original game, built as game.wasm, run for the app
//! (`opence_runtime`): loaded and compiled ahead of time, run on its own
//! thread, its keyboard and mouse handed to it from the app's, and each
//! render batch it submits handed to the app's renderer ([`RenderTarget`])
//! as a [`RenderFrame`].
//!
//! A frame carries its bytes with it: the renderer never reads the game's
//! memory. The bytes each upload, library and buffer write names follow one
//! another in the order of the commands ([`opence_runtime::payload`]), so
//! the renderer takes each command's `length` of them in turn. A texture
//! the renderer reads back is answered as a [`RenderReadback`], which the
//! runtime writes only where the batch's ReadTexture names
//! ([`opence_runtime::readback_fits`]).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use opence_render::{Command, DecodedBatch};
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{
    control, keystroke_flags, payload, virtual_key, Console, Controls, Game, Readback, RenderSink,
    Runtime, Speakers, SPEAKER_RATE,
};

/// One render batch as the renderer carries it out: the commands that
/// passed the check, and the bytes their uploads name, in order.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct RenderFrame {
    /// the batch's commands that passed
    pub commands: Vec<Command>,
    /// the bytes of each UploadBuffer, UploadTexture, CreateLibrary and
    /// WriteBuffer, one after another, each its command's `length`
    pub data: Vec<u8>,
}

/// Bytes the renderer read back from a texture, for the game's memory at
/// `address`, as a ReadTexture named it.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RenderReadback {
    /// where in the game's memory
    pub address: u32,
    /// the bytes
    pub bytes: Vec<u8>,
}

/// The app's renderer (Swift and Metal).
#[uniffi::export(with_foreign)]
pub trait RenderTarget: Send + Sync {
    /// Carries out `frame`'s commands; answers the textures it read back.
    fn submit(&self, frame: RenderFrame) -> Vec<RenderReadback>;
}

/// The runtime's sink for a [`RenderTarget`].
struct TargetSink(Arc<dyn RenderTarget>);

impl RenderSink for TargetSink {
    fn submit(&mut self, batch: &DecodedBatch, memory: &View<'_>) -> Vec<Readback> {
        // (the validator checked every range against the memory, so a
        // payload is missing only if the memory shrank, which wasm's cannot)
        let Some(data) = payload(batch, memory) else {
            return Vec::new();
        };
        let frame = RenderFrame {
            commands: batch.commands.clone(),
            data,
        };
        self.0
            .submit(frame)
            .into_iter()
            .map(|readback| Readback {
                address: readback.address,
                bytes: readback.bytes,
            })
            .collect()
    }
}

/// The runtime's render sink that hands each batch to `target`.
#[must_use]
pub fn render_sink(target: Arc<dyn RenderTarget>) -> Box<dyn RenderSink> {
    Box::new(TargetSink(target))
}

/// Counts the game's ticks: none while it loads or sits at its menus, so
/// the app can tell loading from play.
struct TickCounter(Arc<AtomicU64>);

impl Plugin for TickCounter {
    fn name(&self) -> &str {
        "ticks"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Tick]
    }

    fn on(&mut self, _: &Event<'_>, _: &View<'_>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

/// Where the game and its files are, and what it draws to.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct WasmGameOptions {
    /// game.wasm
    pub wasm: String,
    /// the game's data folder (its maps)
    pub data: String,
    /// its saves folder
    pub saves: String,
    /// the folder of compiled modules, or none to compile each time
    pub cache: Option<String>,
    /// the drawable's width in pixels
    pub width: u32,
    /// and its height
    pub height: u32,
    /// the map to start on, or none for the main menu
    pub map: Option<String>,
    /// the seconds to run for once started (a headless run), or none to
    /// run until the game quits
    pub seconds: Option<u32>,
    /// settings for the game, each `section.key=value` as the settings
    /// file would have it
    pub settings: Vec<String>,
    /// the folder to record a map audit's run into ([`crate::audit`]),
    /// its keys and mouse played by the audit's schedule, or none
    pub audit: Option<String>,
}

/// Why the game could not be loaded or started.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, uniffi::Error)]
pub enum WasmGameError {
    /// game.wasm could not be read, compiled or prepared
    #[error("the game could not be loaded: {reason}")]
    Load {
        /// what went wrong
        reason: String,
    },
    /// the game has already been started
    #[error("the game has already been started")]
    Started,
    /// the control socket could not be bound
    #[error("the control socket could not be bound: {reason}")]
    Control {
        /// what went wrong
        reason: String,
    },
}

/// How the game ended, and what it drew.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct GameEnd {
    /// why it stopped, if not by returning from its main
    pub error: Option<String>,
    /// render batches checked and handed on
    pub batches: u64,
    /// their commands that passed
    pub commands: u64,
    /// their commands refused
    pub refused: u64,
    /// batches refused whole
    pub refused_batches: u64,
    /// the first refusals, as text
    pub refusals: Vec<String>,
    /// whether it stopped because its seconds were up
    pub timed_out: bool,
    /// each frame's CPU time in milliseconds from the first tick on, less
    /// its throttle and sleeps (`opence_runtime::benchmark::frames`); none
    /// unless [`WasmGame::profile`] was asked
    pub frame_ms: Vec<f64>,
}

/// the game's thread: it hands the game back, with how it stopped
type Running = JoinHandle<(Game, Result<(), opence_runtime::Error>)>;

/// The game, loaded; [`WasmGame::start`] runs it on its own thread.
#[derive(uniffi::Object)]
pub struct WasmGame {
    /// the game until it is started
    game: Mutex<Option<Game>>,
    /// the seconds it runs for once started
    seconds: Option<Duration>,
    /// its thread, which hands the game back when its main returns
    thread: Mutex<Option<Running>>,
    controls: Controls,
    console: Console,
    /// the console, keyboard and mouse the control socket drives
    driven: control::Driven,
    speakers: Speakers,
    /// the game's ticks so far ([`TickCounter`])
    ticks: Arc<AtomicU64>,
}

#[uniffi::export]
impl WasmGame {
    /// Loads game.wasm (compiled ahead of time, from the cache when it is
    /// there) and prepares its memory, folders and drawable.
    ///
    /// # Errors
    /// [`WasmGameError::Load`] if the module cannot be read, compiled or
    /// prepared.
    #[uniffi::constructor]
    pub fn new(options: WasmGameOptions) -> Result<Arc<Self>, WasmGameError> {
        let fail = |reason: String| WasmGameError::Load { reason };
        let wasm = std::fs::read(&options.wasm)
            .map_err(|error| fail(format!("{}: {error}", options.wasm)))?;
        let cache = options.cache.as_ref().map(PathBuf::from);
        let runtime = Runtime::new(cache.as_deref()).map_err(|error| fail(error.to_string()))?;
        let mut game = runtime
            .load(&wasm)
            .map_err(|error| fail(error.to_string()))?;
        game.prepare(
            &PathBuf::from(&options.data),
            &PathBuf::from(&options.saves),
            (options.width, options.height),
        )
        .map_err(|error| fail(error.to_string()))?;
        game.set_tone(crate::loading::chant_tone(Path::new(&options.data)).as_ref())
            .map_err(|error| fail(error.to_string()))?;
        for setting in &options.settings {
            let set = setting
                .split_once('=')
                .is_some_and(|(name, text)| game.set_setting(name, text));
            if !set {
                return Err(fail(format!("no setting {setting}")));
            }
        }
        if let Some(map) = &options.map {
            game.set_init_commands(&format!("map_name {map}\n"));
        }
        let ticks = Arc::new(AtomicU64::new(0));
        game.add_plugin(Box::new(TickCounter(Arc::clone(&ticks))));
        if let Some(folder) = &options.audit {
            let audit = crate::audit::Audit::new(Path::new(folder), game.controls())
                .map_err(|error| fail(format!("{folder}: {error}")))?;
            game.add_plugin(Box::new(audit));
        }
        let controls = game.controls();
        let console = game.console();
        let driven = control::Driven::of(&mut game);
        let speakers = game.speakers();
        let seconds = options
            .seconds
            .map(|seconds| Duration::from_secs(u64::from(seconds)));
        Ok(Arc::new(Self {
            game: Mutex::new(Some(game)),
            seconds,
            thread: Mutex::new(None),
            controls,
            console,
            driven,
            speakers,
            ticks,
        }))
    }

    /// Runs the game's main on its own thread, its render batches handed
    /// to `target`.
    ///
    /// # Errors
    /// [`WasmGameError::Started`] if it has been started already.
    pub fn start(&self, target: Arc<dyn RenderTarget>) -> Result<(), WasmGameError> {
        let mut game = self
            .game
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
            .ok_or(WasmGameError::Started)?;
        game.set_render_sink(render_sink(target));
        game.set_deadline(self.seconds.map(|seconds| Instant::now() + seconds));
        let thread = std::thread::Builder::new()
            .name("game".to_owned())
            .spawn(move || {
                let ended = game.call("halo_game_main").map(|_| ());
                (game, ended)
            })
            .map_err(|error| WasmGameError::Load {
                reason: error.to_string(),
            })?;
        *self.thread.lock().unwrap_or_else(PoisonError::into_inner) = Some(thread);
        Ok(())
    }

    /// Times the game's profile sections from its start, for
    /// [`GameEnd::frame_ms`]; false once it has been started.
    pub fn profile(&self) -> bool {
        let mut game = self.game.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(game) = game.as_mut() else {
            return false;
        };
        game.set_profiling(true);
        true
    }

    /// Waits for the game's main to return, and answers how it ended.
    pub fn wait(&self) -> GameEnd {
        let thread = self
            .thread
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        let ended = match thread {
            Some(thread) => thread
                .join()
                .map_err(|_| "the game's thread panicked".to_owned()),
            None => Err("the game was not started".to_owned()),
        };
        match ended {
            Ok((mut game, ended)) => {
                let counts = game.render_counts();
                let frame_ms = opence_runtime::benchmark::frames(&game.take_profile_spans())
                    .into_iter()
                    .map(|ns| Duration::from_nanos(ns).as_secs_f64() * 1000.0)
                    .collect();
                let timed_out = matches!(ended, Err(opence_runtime::Error::Deadline));
                GameEnd {
                    error: ended
                        .err()
                        .filter(|_| !timed_out)
                        .map(|error| error.to_string()),
                    batches: counts.batches,
                    commands: counts.commands,
                    refused: counts.refused,
                    refused_batches: counts.refused_batches,
                    refusals: game.render_refusals(),
                    timed_out,
                    frame_ms,
                }
            }
            Err(error) => GameEnd {
                error: Some(error),
                batches: 0,
                commands: 0,
                refused: 0,
                refused_batches: 0,
                refusals: Vec::new(),
                timed_out: false,
                frame_ms: Vec::new(),
            },
        }
    }

    /// A key of the Mac's keyboard (its virtual key code) going down or up,
    /// with its character (0 if none) and the modifiers held; a key the
    /// game has no Windows key for is dropped.
    pub fn mac_key(
        &self,
        code: u16,
        character: u8,
        control: bool,
        shift: bool,
        alt: bool,
        down: bool,
    ) {
        if let Some(vk) = virtual_key(code) {
            self.controls
                .key(vk, character, keystroke_flags(control, shift, alt), down);
        }
    }

    /// The mouse moving by `dx` and `dy` pixels, right and down.
    pub fn mouse_motion(&self, dx: f32, dy: f32) {
        self.controls.mouse_motion(dx, dy);
    }

    /// A mouse button going down or up: 1 the left, 2 the middle, 3 the
    /// right, 4 and on the side buttons.
    pub fn mouse_button(&self, button: u8, down: bool) {
        self.controls.mouse_button(button, down);
    }

    /// Hands the game's console `line` (`debug.telnet_console` on) and
    /// gives what it writes back within `milliseconds`; none if the
    /// console is not listening.
    pub fn ask(&self, line: String, milliseconds: u32) -> Option<String> {
        self.console
            .ask(&line, Duration::from_millis(u64::from(milliseconds)))
    }

    /// Serves the control socket at `path` (readable by this user alone),
    /// each line a client sends handed to the game's console as `ask`
    /// does and each structured request to its keyboard and mouse, so the
    /// `opence` tool drives the game the app plays.
    ///
    /// # Errors
    /// [`WasmGameError::Control`] if the socket cannot be bound there.
    pub fn listen(&self, path: String) -> Result<(), WasmGameError> {
        let listener =
            control::bind(std::path::Path::new(&path)).map_err(|error| WasmGameError::Control {
                reason: format!("{path}: {error}"),
            })?;
        control::serve(listener, self.driven.clone());
        Ok(())
    }

    /// The game's ticks so far: none while it loads or sits at its menus.
    pub fn ticks(&self) -> u64 {
        self.ticks.load(Ordering::Relaxed)
    }

    /// The wheel turning by `notches`, up positive.
    pub fn mouse_wheel(&self, notches: f32) {
        self.controls.mouse_wheel(notches);
    }

    /// The game's speakers, for the app's audio device to pull its mixed
    /// sound from.
    pub fn speakers(&self) -> Arc<GameSpeakers> {
        Arc::new(GameSpeakers(self.speakers.clone()))
    }
}

/// The mixed sound of a running game.wasm (its own mixer's, 48 kHz
/// stereo), pulled on the audio device's thread.
#[derive(uniffi::Object)]
pub struct GameSpeakers(Speakers);

#[uniffi::export]
impl GameSpeakers {
    /// The sound's sample rate.
    pub fn sample_rate(&self) -> u32 {
        SPEAKER_RATE
    }

    /// The next `frames` frames, interleaved stereo: what the game has
    /// mixed, then silence. At most a second is pulled at once.
    pub fn pull(&self, frames: u32) -> Vec<f32> {
        let frames = frames.min(SPEAKER_RATE) as usize;
        let mut samples = vec![0.0; frames * 2];
        self.0.pull(&mut samples);
        samples
    }
}
