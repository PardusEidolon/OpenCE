//! The renderer blends each object from where it was at the tick before
//! to where it is: one put somewhere (a teleport) is drawn there from the
//! first frame, not blended across the level from where it was.

use std::error::Error;

use opence_content::{devmap, Level};
use opence_import::testing::stock_level;
use opence_sim::input::TickInput;
use opence_sim::{PreviousTransform, Simulation, Vec3};

fn test_map() -> Result<Option<Level>, Box<dyn Error>> {
    let Some(stock) = stock_level("bloodgulch")? else {
        return Ok(None); // (no game data here)
    };
    let donor = Level::clone(&stock);
    Ok(Some(devmap::level(Some(&donor))))
}

#[test]
fn a_teleported_player_is_not_blended_from_where_it_was() -> Result<(), Box<dyn Error>> {
    let Some(level) = test_map()? else {
        return Ok(());
    };
    let mut simulation = Simulation::empty();
    simulation.spawn_player(&level, 0)?;
    for _ in 0..30 {
        simulation.step(TickInput::default());
    }
    let handle = simulation.players()[0].handle();
    let Some(before) = simulation.transform(handle) else {
        return Err("the player has no object".into());
    };
    let to = before.position + Vec3::new(20.0, 0.0, 0.0);
    assert!(simulation.teleport_player(0, to));
    simulation.step(TickInput::default());
    let (Some(now), Some(PreviousTransform(then))) = (
        simulation.transform(handle),
        simulation.component::<PreviousTransform>(handle),
    ) else {
        return Err("the player has no object".into());
    };
    assert!(now.position.distance(to) < 0.5, "{} {to}", now.position);
    assert_eq!(then, now);
    // (and walking on from there, it is blended again)
    let forward = TickInput {
        forward: 127,
        ..TickInput::default()
    };
    for _ in 0..10 {
        simulation.step(forward);
    }
    let (Some(now), Some(PreviousTransform(then))) = (
        simulation.transform(handle),
        simulation.component::<PreviousTransform>(handle),
    ) else {
        return Err("the player has no object".into());
    };
    assert!(now.position.distance(then.position) > 0.0);
    assert!(now.position.distance(then.position) < 0.5);
    Ok(())
}
