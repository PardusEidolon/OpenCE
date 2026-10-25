//! A grenade thrown on Blood Gulch is drawn every frame from the throw to
//! its detonation, in the first person and the third: the object the throw
//! makes is dressed in its model the frame it is made (needs assets/maps;
//! passes without it).

use std::path::Path;
use std::sync::Arc;

use opence_ffi::{Engine, InputState, LevelGeometry};
use opence_sim::input::Buttons;
use serde_json::Value;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run(engine: &Engine, line: &str) -> Result<Value> {
    let output = engine.run_command(line.into());
    if !output.ok {
        return Err(format!("{line}: {}", output.text).into());
    }
    Ok(serde_json::from_str(&output.json)?)
}

fn still(buttons: u32) -> InputState {
    InputState {
        buttons,
        forward: 0,
        right: 0,
        up: 0,
        yaw: 0.0,
        pitch: 0.0,
    }
}

/// Blood Gulch, alone, the player settled at the first start; none
/// without game data.
fn blood_gulch() -> Result<Option<Arc<Engine>>> {
    let map = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/maps/bloodgulch.map");
    if !map.exists() {
        return Ok(None);
    }
    let cache = opence_import::testing::cache_folder();
    let geometry = LevelGeometry::load(
        map.to_string_lossy().into(),
        cache.to_string_lossy().into(),
        0,
    )?;
    let engine = Engine::new();
    engine.start_level(geometry, 1)?;
    engine.spawn_player(0)?;
    for _ in 0..30 {
        engine.advance(1.0 / 30.0)?;
    }
    Ok(Some(engine))
}

/// Throws a grenade and watches it to its detonation: the frames it was
/// in the world, and those of them it was not drawn in.
fn throw(engine: &Engine) -> Result<(u32, Vec<u32>)> {
    let (mut present, mut undrawn) = (0, Vec::new());
    for frame in 0..300 {
        let buttons = if frame < 3 { Buttons::GRENADE.0 } else { 0 };
        engine.set_input(still(buttons));
        engine.advance(1.0 / 30.0)?;
        let rows = run(engine, "objects projectile")?;
        let rows = rows.as_array().map(Vec::as_slice).unwrap_or_default();
        if rows.is_empty() {
            if present > 0 {
                break; // (it has detonated)
            }
            continue;
        }
        present += 1;
        if rows.iter().any(|row| row["drawn"] != true) {
            undrawn.push(frame);
        }
    }
    engine.set_input(still(0));
    Ok((present, undrawn))
}

#[test]
fn a_thrown_grenade_is_drawn_from_the_throw_to_its_detonation() -> Result {
    let Some(engine) = blood_gulch()? else {
        return Ok(()); // (no game data here)
    };
    for third_person in [false, true] {
        engine.set_third_person(third_person);
        let (present, undrawn) = throw(&engine)?;
        let view = if third_person { "third" } else { "first" };
        assert!(
            present > 10,
            "in the {view} person the grenade flew {present} frames"
        );
        assert!(
            undrawn.is_empty(),
            "in the {view} person the grenade was not drawn in frames {undrawn:?} of {present}"
        );
    }
    Ok(())
}
