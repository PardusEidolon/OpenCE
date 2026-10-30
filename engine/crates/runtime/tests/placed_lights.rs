//! The dynamic lights the game submits each frame (`rasterizer_lights`)
//! reach the host: game.wasm's device sends them as one LIGHTS command
//! before each PRESENT, every light checked. Blood Gulch is played for a
//! time and the frames' commands are read. Needs game.wasm (or the one
//! `OPENCE_GAME_WASM` names) and the user's maps (assets/maps); skips itself
//! without them.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use opence_render::Command;
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime};

/// what the frames' commands held
#[derive(Default, Debug, Clone, Copy)]
struct Seen {
    presents: usize,
    lights: usize,
    /// a PRESENT without a LIGHTS before it since the last one
    unlit_presents: usize,
    most: usize,
    lit_frames: usize,
    pending: bool,
}

struct Lights {
    seen: Arc<Mutex<Seen>>,
}

impl Plugin for Lights {
    fn name(&self) -> &str {
        "lights"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Render]
    }

    fn on(&mut self, event: &Event<'_>, _view: &View<'_>) {
        let Event::Render(commands) = event else {
            return;
        };
        let Ok(mut seen) = self.seen.lock() else {
            return;
        };
        for command in *commands {
            match command {
                Command::Lights { lights } => {
                    seen.lights += 1;
                    seen.pending = true;
                    seen.most = seen.most.max(lights.len());
                    seen.lit_frames += usize::from(!lights.is_empty());
                }
                Command::Present => {
                    seen.presents += 1;
                    seen.unlit_presents += usize::from(!seen.pending);
                    seen.pending = false;
                }
                _ => {}
            }
        }
    }
}

#[test]
fn each_frame_sends_its_lights_before_it_is_shown() -> Result<(), Error> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let wasm = std::env::var_os("OPENCE_GAME_WASM")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("build/engine/wasm/game.wasm"));
    let (Ok(bytes), true) = (
        std::fs::read(&wasm),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs game.wasm and assets/maps/bloodgulch.map");
        return Ok(());
    };
    let saves = std::env::temp_dir().join(format!("opence-lights-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&saves);
    let cache = wasm.parent().map(|folder| folder.join("cache"));
    let runtime = Runtime::new(cache.as_deref())?;
    let mut game = runtime.load(&bytes)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    game.set_init_commands("map_name bloodgulch\n");
    let seen = Arc::new(Mutex::new(Seen::default()));
    game.add_plugin(Box::new(Lights {
        seen: Arc::clone(&seen),
    }));
    game.set_deadline(Some(Instant::now() + Duration::from_secs(40)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    assert!(matches!(ran, Err(Error::Deadline)), "{ran:?}");
    let seen = seen.lock().map(|seen| *seen).unwrap_or_default();
    eprintln!("lights: {seen:?}");
    assert!(seen.presents > 0, "{seen:?}");
    assert_eq!(seen.lights, seen.presents, "{seen:?}");
    assert_eq!(seen.unlit_presents, 0, "{seen:?}");
    // a refused LIGHTS never reaches a plugin, and the game sends at most
    // MAXIMUM_LIGHTS_PER_WINDOW
    assert!(seen.most <= 128, "{seen:?}");
    Ok(())
}
