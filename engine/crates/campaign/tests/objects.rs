//! The scripts' objects, as `hs_library_objects.c` keeps them: a named
//! object is made once and made anew only when asked, destroyed but never
//! a player's, scaled, attached to a parent that carries it and let go,
//! and kept from harm while it cannot take damage (skipped where there is
//! no game data).

use std::error::Error;
use std::sync::Arc;

use opence_campaign::Mission;
use opence_import::testing::stock_level_unheld;
use opence_script::Value;
use opence_sim::damage::Vitality;
use opence_sim::input::TickInput;
use opence_sim::objects::Difficulty;
use opence_sim::{clock, Simulation, Vec3};

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

/// The mission of `map` with its player, if the map is here.
fn mission(map: &str) -> Result<Option<Mission>> {
    let Some(stock) = stock_level_unheld(map)? else {
        return Ok(None);
    };
    let level = Arc::new(stock);
    let mut sim = Simulation::empty();
    sim.spawn_player(&level, 0)?;
    Ok(Some(Mission::new(sim, level, Difficulty::Normal)))
}

/// The first two scenario object names `create_named` can make.
fn two_names(mission: &mut Mission) -> Option<(usize, usize)> {
    let mut made = Vec::new();
    for name in 0..mission.level().scenario.campaign.object_names.len() {
        if mission.named(name).is_some() {
            continue;
        }
        if let Some(handle) = mission.create_named(name, false) {
            mission.remove(handle);
            made.push(name);
        }
        if let [first, second] = made[..] {
            return Some((first, second));
        }
    }
    None
}

#[test]
fn a_named_object_is_made_once_scaled_attached_and_destroyed() -> Result {
    let Some(mut mission) = mission("a10")? else {
        return Ok(()); // (no game data here)
    };
    let (parent_name, child_name) = two_names(&mut mission).ok_or("no names to make")?;
    let name = |n: usize| Value::Index(i32::try_from(n).unwrap_or(-1));

    mission.call("object_create", &[name(parent_name)]);
    let parent = mission.named(parent_name).ok_or("not made")?;
    mission.call("object_create", &[name(parent_name)]);
    assert_eq!(mission.named(parent_name), Some(parent), "made again");
    mission.call("object_create_anew", &[name(parent_name)]);
    let parent = mission.named(parent_name).ok_or("not made anew")?;
    mission.call("object_create", &[name(child_name)]);
    let child = mission.named(child_name).ok_or("not made")?;

    let held = Value::Index(mission.index_of(parent));
    mission.call(
        "object_set_scale",
        &[held.clone(), Value::Real(2.5), Value::Short(0)],
    );
    assert_eq!(mission.sim().transform(parent).map(|t| t.scale), Some(2.5));

    let carried = Value::Index(mission.index_of(child));
    mission.call(
        "objects_attach",
        &[
            held.clone(),
            Value::String(String::new()),
            carried.clone(),
            Value::String(String::new()),
        ],
    );
    let at = |mission: &Mission, handle| mission.sim().transform(handle).map(|t| t.position);
    assert_eq!(
        at(&mission, child),
        at(&mission, parent),
        "put on its parent"
    );
    let mut moved = mission.sim().transform(parent).ok_or("no parent")?;
    moved.position += Vec3::new(3.0, 0.0, 0.0);
    mission.sim_mut().place(parent, moved);
    mission.step(TickInput::default());
    let parent_at = at(&mission, parent).ok_or("no parent")?;
    let child_at = at(&mission, child).ok_or("no child")?;
    assert!((child_at - parent_at).length() < 1.0e-3, "carried along");

    mission.call("objects_detach", &[held.clone(), carried.clone()]);
    moved.position += Vec3::new(3.0, 0.0, 0.0);
    mission.sim_mut().place(parent, moved);
    mission.step(TickInput::default());
    let left = at(&mission, child).ok_or("no child")?;
    assert!((left - child_at).length() < 1.0e-3, "let go");

    mission.call("object_destroy", &[carried]);
    assert_eq!(mission.named(child_name), None);
    let player = mission.sim().player().ok_or("no player")?.handle();
    let player_held = Value::Index(mission.index_of(player));
    mission.call("object_destroy", &[player_held]);
    assert!(
        mission.sim().object(player).is_some(),
        "a player is never destroyed"
    );
    Ok(())
}

#[test]
fn an_object_that_cannot_take_damage_survives_a_fall() -> Result {
    let Some(mut mission) = mission("bloodgulch")? else {
        return Ok(()); // (no game data here)
    };
    for _ in 0..30 {
        mission.step(TickInput::default());
    }
    let player = mission.sim().player().ok_or("no player")?.handle();
    let ground = mission.sim().player().ok_or("no player")?.mover().position;
    let fall = |mission: &mut Mission| -> Result<f32> {
        mission
            .sim_mut()
            .teleport_player(0, ground + Vec3::new(0.0, 0.0, 12.0));
        for _ in 0..3 * clock::TICKS_PER_SECOND {
            mission.step(TickInput::default());
        }
        let vitality: Vitality = mission.sim().component(player).ok_or("no vitality")?;
        Ok(vitality.body)
    };
    let players = mission.call("players", &[]).ok_or("no players")?;
    mission.call("object_cannot_take_damage", std::slice::from_ref(&players));
    assert_eq!(fall(&mut mission)?, 1.0, "kept from harm");
    mission.call("object_can_take_damage", &[players]);
    assert!(fall(&mut mission)? < 1.0, "hurt by the fall");
    Ok(())
}

#[test]
fn a_player_is_inside_a_trigger_volume_only_while_standing_in_it() -> Result {
    let Some(mut mission) = mission("a10")? else {
        return Ok(()); // (no game data here)
    };
    // The middle of the first volume tall and wide enough to stand in.
    let volumes = mission.level().scenario.campaign.trigger_volumes.clone();
    let (index, middle) = volumes
        .iter()
        .enumerate()
        .find_map(|(index, volume)| {
            let [x, y, z] = volume.extents;
            let forward = Vec3::from(volume.forward);
            let up = Vec3::from(volume.up);
            let left = up.cross(forward);
            let middle = Vec3::from(volume.position)
                + forward * (x / 2.0)
                + left * (y / 2.0)
                + up * (z / 2.0);
            (x > 2.0 && y > 2.0 && z > 3.0 && volume.contains(middle.to_array()))
                .then_some((index, middle))
        })
        .ok_or("no volume to stand in")?;
    let player = mission.sim().player().ok_or("no player")?.handle();
    let held = Value::Index(mission.index_of(player));
    let volume = Value::Index(i32::try_from(index)?);
    let test = |mission: &mut Mission, at: Vec3| {
        mission
            .sim_mut()
            .teleport_player(0, at - Vec3::new(0.0, 0.0, 0.5));
        mission.step(TickInput::default());
        mission.call("volume_test_object", &[volume.clone(), held.clone()])
    };
    assert_eq!(
        test(&mut mission, middle),
        Some(Value::Boolean(true)),
        "inside"
    );
    let away = middle + Vec3::new(0.0, 0.0, 200.0);
    assert_eq!(
        test(&mut mission, away),
        Some(Value::Boolean(false)),
        "outside"
    );
    let missing = Value::Index(i32::try_from(volumes.len())?);
    assert_eq!(
        mission.call("volume_test_object", &[missing, held.clone()]),
        Some(Value::Boolean(false)),
        "a volume the scenario does not have"
    );
    Ok(())
}
