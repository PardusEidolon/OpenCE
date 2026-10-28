//! The lighting's switch to stock: `display.per_pixel_lighting` lights the
//! game's model lighting programs for each pixel, from the original's own
//! lights (the ambient light, two distant lights and two point lights,
//! nv2a_psh.c `model_lighting`), and off, as it is by default, the models
//! are lit at each vertex as on the Xbox. Blood Gulch is played for a time
//! each way, and the shader libraries the device makes are read: off, none
//! of them lights a pixel; on, some do. Needs game.wasm and the user's maps
//! (assets/maps); skips itself without them.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use opence_render::Command;
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime};

/// the shader libraries made, and how many of them light each pixel
#[derive(Default, Debug, Clone, Copy)]
struct Libraries {
    made: usize,
    lit: usize,
}

/// reads the source of each library the frame's commands make
struct Shaders {
    seen: Arc<Mutex<Libraries>>,
}

impl Plugin for Shaders {
    fn name(&self) -> &str {
        "shaders"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Render]
    }

    fn on(&mut self, event: &Event<'_>, view: &View<'_>) {
        let Event::Render(commands) = event else {
            return;
        };
        let Ok(mut seen) = self.seen.lock() else {
            return;
        };
        for command in *commands {
            if let Command::CreateLibrary {
                address, length, ..
            } = *command
            {
                seen.made += 1;
                let source = view.bytes(address, length).unwrap_or_default();
                if source.windows(15).any(|word| word == b"model_lighting(") {
                    seen.lit += 1;
                }
            }
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Blood Gulch played for 60 seconds with `per_pixel` as the setting (or
/// left as it is), and the libraries its device made; `None` without the
/// game or the map
fn libraries(per_pixel: Option<&str>) -> Result<Option<Libraries>, Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/bloodgulch.map");
        return Ok(None);
    };
    let saves = std::env::temp_dir().join(format!(
        "opence-lighting-{}-{}",
        std::process::id(),
        per_pixel.unwrap_or("default")
    ));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    if let Some(text) = per_pixel {
        assert!(game.set_setting("display.per_pixel_lighting", text));
    }
    game.set_init_commands("map_name bloodgulch\n");
    let seen = Arc::new(Mutex::new(Libraries::default()));
    game.add_plugin(Box::new(Shaders {
        seen: Arc::clone(&seen),
    }));
    game.set_deadline(Some(Instant::now() + Duration::from_secs(60)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    let seen = seen.lock().map(|seen| *seen).unwrap_or_default();
    eprintln!("lighting {per_pixel:?}: {seen:?}");
    Ok(Some(seen))
}

#[test]
fn by_default_the_models_are_lit_at_each_vertex_as_on_the_xbox() -> Result<(), Error> {
    let Some(seen) = libraries(None)? else {
        return Ok(());
    };
    assert!(seen.made > 0, "no shader library was made");
    assert_eq!(seen.lit, 0, "{seen:?}");
    Ok(())
}

#[test]
fn per_pixel_lighting_lights_the_models_from_the_original_lights() -> Result<(), Error> {
    let Some(seen) = libraries(Some("true"))? else {
        return Ok(());
    };
    assert!(seen.lit > 0, "{seen:?}");
    assert!(seen.lit < seen.made, "{seen:?}");
    Ok(())
}
