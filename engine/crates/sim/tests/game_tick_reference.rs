//! The engine's tick against the game's: tools/engine_game_tick_reference.py
//! runs game.c's `game_tick` with each system it updates a stand-in that
//! says its name, and this checks the engine's tick against that order.
//!
//! The engine's tick is not one function: `run_ticks` (ffi/src/lib.rs) runs
//! the campaign's `Campaign::play` (the scripts on a game tick, then
//! `Mission::step`: the actors think, the simulation steps the players and
//! the actors, their first-person weapons, then the recorded animations end),
//! then the effects, the lights, the decals and the particles. `ENGINE` is
//! that order, by the game's names; where it differs from the game's is
//! `OUT_OF_ORDER`, which the game loop track puts in the game's order.

use std::collections::BTreeSet;

use opence_sim::clock::TICK;

const REFERENCE: &str = include_str!("game_tick_reference.txt");

/// The systems a tick of the game alone updates, in the game's order.
const GAME: [&str; 27] = [
    "game_state_check_data_arrays",
    "lights_port_recover",
    "cheats_network_client_enforce",
    "remove_quitting_players_from_game",
    "game_allegiance_update",
    "units_update",
    "ai_update",
    "players_update_before_game",
    "effects_update",
    "antennas_update",
    "lock_global_random_seed",
    "rumble_update",
    "first_person_weapons_update",
    "unlock_global_random_seed",
    "game_engine_update",
    "editor_update",
    "hs_update",
    "recorded_animations_update",
    "objects_update",
    "physics_jolt_update",
    "players_update_after_game",
    "movement_trace_update",
    "placement_trace_update",
    "hud_update",
    "player_effect_update",
    "game_state_check_data_arrays",
    "lights_port_recover",
];

/// The game's systems the engine's tick updates, in the engine's order.
const ENGINE: [&str; 6] = [
    // (campaign's `Campaign::play`, on a game tick)
    "hs_update",
    // (`Mission::step`: `actors::think`, then the simulation's step)
    "ai_update",
    "units_update",
    "first_person_weapons_update",
    "recorded_animations_update",
    // (ffi's `run_ticks`, after the mission)
    "effects_update",
];

/// The pairs the engine updates in the other order from the game's, the
/// game's first: the scripts run before the actors, the units and the
/// weapons rather than after them, the actors think before the units move,
/// and the effects come after the whole simulation.
const OUT_OF_ORDER: [(&str, &str); 7] = [
    ("ai_update", "hs_update"),
    ("units_update", "hs_update"),
    ("first_person_weapons_update", "hs_update"),
    ("effects_update", "hs_update"),
    ("units_update", "ai_update"),
    ("effects_update", "first_person_weapons_update"),
    ("effects_update", "recorded_animations_update"),
];

type Failure = Box<dyn std::error::Error>;

/// A tick of the trace, by its kind: the systems it updates, with the
/// seconds those that are timed are given.
fn tick(kind: &str) -> Result<Vec<(String, Option<f32>)>, Failure> {
    let mut systems = None;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let mut fields = line.split_whitespace();
        let name = fields.next().unwrap_or_default();
        match (name, &mut systems) {
            ("tick", None) if fields.next() == Some(kind) => systems = Some(Vec::new()),
            ("end", Some(_)) => break,
            (_, Some(systems)) => {
                let seconds = fields.next().map(str::parse).transpose()?;
                systems.push((name.to_owned(), seconds));
            }
            _ => {}
        }
    }
    Ok(systems.ok_or("no such tick")?)
}

fn names(systems: &[(String, Option<f32>)]) -> Vec<&str> {
    systems.iter().map(|(name, _)| name.as_str()).collect()
}

#[test]
fn the_game_ticks_in_the_order_recorded() -> Result<(), Failure> {
    assert_eq!(names(&tick("alone")?), GAME);
    Ok(())
}

#[test]
fn a_game_tick_lasts_two_of_the_engines() -> Result<(), Failure> {
    // (the engine's simulation ticks at 60 Hz, the game's at 30, or at 60
    // when the players are at double speed)
    let engine = TICK.0;
    for (kind, seconds) in [("alone", engine * 2.0), ("double_speed", engine)] {
        let timed: Vec<f32> = tick(kind)?.into_iter().filter_map(|(_, s)| s).collect();
        assert_eq!(timed.len(), 3, "{kind}");
        assert!(
            timed.iter().all(|given| *given == seconds),
            "{kind}: {timed:?}"
        );
    }
    Ok(())
}

#[test]
fn a_client_leaves_the_actors_and_the_scripts_to_the_host() -> Result<(), Failure> {
    let distributed = tick("distributed_client")?;
    let distributed = names(&distributed);
    assert!(!distributed.contains(&"ai_update"));
    assert!(distributed.contains(&"network_actors_drive"));
    assert!(distributed.contains(&"hs_update"));
    assert!(!names(&tick("coop_client")?).contains(&"hs_update"));
    Ok(())
}

#[test]
fn the_engine_ticks_out_of_the_games_order_only_where_recorded() {
    let place = |name: &str| GAME.iter().position(|n| *n == name);
    let mut differ = BTreeSet::new();
    for (i, first) in ENGINE.iter().enumerate() {
        for second in &ENGINE[i + 1..] {
            // (the engine updates `first` before `second`)
            assert!(place(first).is_some() && place(second).is_some());
            if place(first) > place(second) {
                differ.insert((*second, *first));
            }
        }
    }
    let expected: BTreeSet<(&str, &str)> = OUT_OF_ORDER.into_iter().collect();
    assert_eq!(differ, expected);
}
