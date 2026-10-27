//! The AI's actors on the campaign's maps: every encounter placed stands
//! its actors on the ground, armed and on a team, and an enemy put before
//! the player sees it, turns to it and shoots it (skipped where there is
//! no game data).

use std::error::Error;
use std::sync::Arc;

use opence_campaign::{AiReference, Mission};
use opence_content::Level;
use opence_import::testing::stock_level_unheld;
use opence_script::Value;
use opence_sim::damage::Vitality;
use opence_sim::input::TickInput;
use opence_sim::objects::Difficulty;
use opence_sim::{Simulation, Vec3};

/// A body no enemy's fire takes in the ticks a test runs.
const STURDY: f32 = 1.0e6;

/// The ticks each enemy is given to see the player and hurt it: five
/// seconds, as one may first walk round a boulder or a friend in its way
/// (with the scenery solid, an enemy no longer walks through a rock to
/// fire at the player from beside it).
const TRIED_TICKS: u32 = 300;

const MAPS: [&str; 10] = [
    "a10", "a30", "a50", "b30", "b40", "c10", "c20", "c40", "d20", "d40",
];

/// The mission of `map` at legendary with every encounter placed, if the
/// map is here.
fn mission(map: &str) -> Result<Option<Mission>, Box<dyn Error>> {
    let Some(stock) = stock_level_unheld(map)? else {
        return Ok(None);
    };
    placed(&Arc::new(stock)).map(Some)
}

/// `level`'s mission at legendary with every encounter placed.
fn placed(level: &Arc<Level>) -> Result<Mission, Box<dyn Error>> {
    let encounters = level.scenario.campaign.encounters.len();
    let mut sim = Simulation::empty();
    sim.spawn_player(level, 0)?;
    let mut mission = Mission::new(sim, Arc::clone(level), Difficulty::Legendary);
    for e in 0..encounters {
        mission.place_ai(AiReference::Encounter(u16::try_from(e)?));
    }
    Ok(mission)
}

#[test]
fn every_encounter_places_its_actors() -> Result<(), Box<dyn Error>> {
    for map in MAPS {
        let Some(mission) = mission(map)? else {
            continue;
        };
        assert!(!mission.actors().is_empty(), "{map}: no actors");
        assert_eq!(
            mission.actors().len(),
            mission.sim().actors().len(),
            "{map}"
        );
        for (actor, player) in mission.actors().iter().zip(mission.sim().actors()) {
            assert_eq!(actor.unit, player.handle(), "{map}");
            assert!(actor.team > 0, "{map}: no team");
            assert!(mission.sim().object(actor.unit).is_some(), "{map}");
        }
    }
    Ok(())
}

#[test]
fn an_enemy_before_the_player_shoots_it() -> Result<(), Box<dyn Error>> {
    let mut missed = Vec::new();
    for map in MAPS {
        let Some(stock) = stock_level_unheld(map)? else {
            continue;
        };
        let level = Arc::new(stock);
        let enemies: Vec<usize> = {
            let mission = placed(&level)?;
            (0..mission.actors().len())
                .filter(|&i| ![1, 2].contains(&mission.actors()[i].team))
                .collect()
        };
        // (each enemy tried in turn, in a mission of its own, until one
        // hurts the player: one tried after another would meet the others
        // drawn to where the player stood before, in its line of fire)
        let mut shot = false;
        for index in enemies.into_iter().take(24) {
            if shoots(&mut placed(&level)?, index) {
                shot = true;
                break;
            }
        }
        if !shot {
            missed.push(map);
        }
    }
    assert!(missed.is_empty(), "no enemy shot the player on {missed:?}");
    Ok(())
}

/// Whether actor `index` of `mission` targets the player, put a little
/// ahead of it, and hurts it.
fn shoots(mission: &mut Mission, index: usize) -> bool {
    mission.step(TickInput::default());
    let Some(player) = mission.sim().players().first().map(|p| p.handle()) else {
        return false;
    };
    // (the player too strong to die, so that no ragdoll takes its body)
    let Some(mut sturdy) = mission.sim().component::<Vitality>(player) else {
        return false;
    };
    sturdy.body = STURDY;
    let Some(actor) = mission.sim().actors().get(index) else {
        return false;
    };
    let (yaw, _) = actor.look();
    let ahead = Vec3::new(libm::cosf(yaw.0), libm::sinf(yaw.0), 0.0);
    let at = actor.mover().position + ahead * 4.0 + Vec3::Z * 0.1;
    let unit = mission.actors()[index].unit;
    mission.sim_mut().insert(player, sturdy);
    mission.sim_mut().teleport_player(0, at);
    let structure = mission.flow().bsp;
    // (a teleport into a switching volume loads a structure the enemy may
    // not stand on: that enemy is passed over)
    mission.step(TickInput::default());
    if mission.flow().bsp != structure {
        return false;
    }
    let (mut targeted, mut hurt) = (false, false);
    for _ in 1..TRIED_TICKS {
        mission.step(TickInput::default());
        targeted |= mission.actors().get(index).and_then(|a| a.target) == Some(player);
        hurt |= mission
            .sim()
            .damage_events()
            .iter()
            .any(|event| event.object == player && event.owner == Some(unit));
    }
    targeted && hurt
}

#[test]
fn a_placed_encounter_is_counted_killed_and_erased() -> Result<(), Box<dyn Error>> {
    let Some(stock) = stock_level_unheld("a10")? else {
        return Ok(()); // (no game data here)
    };
    let level = Arc::new(stock);
    let mut sim = Simulation::empty();
    sim.spawn_player(&level, 0)?;
    let mut mission = Mission::new(sim, Arc::clone(&level), Difficulty::Normal);
    let living =
        |mission: &mut Mission, e: i32| mission.call("ai_living_count", &[Value::Index(e)]);
    // The first encounter that places anyone at normal.
    let mut placed = None;
    for e in 0..i32::try_from(level.scenario.campaign.encounters.len())? {
        mission.call("ai_place", &[Value::Index(e)]);
        if !mission.actors().is_empty() {
            placed = Some(e);
            break;
        }
    }
    let e = placed.ok_or("no encounter places anyone")?;
    let units: Vec<_> = mission.actors().iter().map(|a| a.unit).collect();
    let placed_count = i16::try_from(units.len())?;
    assert_eq!(living(&mut mission, e), Some(Value::Short(placed_count)));

    mission.call("ai_kill", &[Value::Index(e)]);
    assert_eq!(living(&mut mission, e), Some(Value::Short(0)), "killed");
    for &unit in &units {
        let vitality: Vitality = mission.sim().component(unit).ok_or("no vitality")?;
        assert!(vitality.dead, "the unit dies");
    }

    mission.call("ai_erase", &[Value::Index(e)]);
    assert!(mission.actors().is_empty(), "the actors are erased");
    assert!(mission.sim().actors().is_empty());
    for &unit in &units {
        assert!(mission.sim().object(unit).is_none(), "with their units");
    }
    Ok(())
}
