//! A suspended unit holds still, as `biped_update_physics` holds a biped
//! whose unit is suspended (`unit_suspended`): lifted far above
//! bloodgulch's ground, the player stays where it was put, still and not
//! falling, and falls when it is let go. Passes without
//! assets/maps/bloodgulch.map.

use std::error::Error;

use opence_import::testing::stock_level;
use opence_sim::input::TickInput;
use opence_sim::{clock, Simulation, Vec3};

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn a_suspended_player_holds_still_in_the_air_and_falls_when_let_go() -> Result {
    let Some(level) = stock_level("bloodgulch")? else {
        return Ok(()); // (no game data here)
    };
    let mut simulation = Simulation::empty();
    let handle = simulation.spawn_player(&level, 0)?;
    for _ in 0..30 {
        simulation.step(TickInput::default());
    }
    let ground = simulation.player().ok_or("no player")?.mover().position;
    let lifted = ground + Vec3::new(0.0, 0.0, 30.0);
    assert!(simulation.teleport_player(0, lifted));
    assert!(simulation.suspend(handle, true));
    for _ in 0..2 * clock::TICKS_PER_SECOND {
        simulation.step(TickInput::default());
    }
    let player = simulation.player().ok_or("no player")?;
    assert!((player.mover().position - lifted).length() < 1.0e-3);
    assert_eq!(player.mover().velocity, Vec3::ZERO);
    assert!(!player.mover().airborne);

    assert!(simulation.suspend(handle, false));
    for _ in 0..clock::TICKS_PER_SECOND / 2 {
        simulation.step(TickInput::default());
    }
    let player = simulation.player().ok_or("no player")?;
    assert!(player.mover().position.z < lifted.z - 0.5);
    let stranger = opence_sim::objects::ObjectHandle {
        index: 999,
        salt: 7,
    };
    assert!(!simulation.suspend(stranger, true));
    Ok(())
}
