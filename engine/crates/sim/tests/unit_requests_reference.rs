//! The engine's small requests of a unit against the game's: its custom
//! animation stopped, its animation aborted, opened or closed, its emotion
//! and seat set, the magic animations' base seat, whether a player may
//! enter it, whether it drops its items and how many grenades it holds, as
//! tools/engine_unit_requests_reference.py records the game's code doing
//! them.

use std::error::Error;

use opence_sim::unit_control::{doesnt_drop_items, UnitFlags};
use opence_sim::unit_inventory::{scripting_grenade_count, UnitInventory};
use opence_sim::unit_scripting::{
    abort_animation, magic_base_seat, open, scripting_set_seat, set_emotion, stop_custom_animation,
    StateRequest,
};
use opence_sim::unit_states::UnitState;

const REFERENCE: &str = include_str!("unit_requests_reference.txt");

type Failure = Box<dyn Error>;

/// Whether the game's unit index is one (not -1).
fn unit(field: &str) -> Result<bool, Failure> {
    Ok(field.parse::<i32>()? >= 0)
}

/// The state set and the network notes sent, as the trace writes them.
fn request(request: Option<StateRequest>) -> String {
    request.map_or_else(
        || "-1 0".to_owned(),
        |request| {
            format!(
                "{} {}",
                request.state.index(),
                u8::from(request.network_note)
            )
        },
    )
}

/// The label between the trace's quotes.
fn label(asked: &str) -> Result<&str, Failure> {
    let start = asked.find('\'').ok_or("no label")?;
    let end = asked.rfind('\'').ok_or("no label")?;
    Ok(&asked[start + 1..end])
}

fn game_seat(seat: Option<usize>) -> Result<i64, Failure> {
    Ok(seat.map_or(Ok(-1), i64::try_from)?)
}

fn case(line: &str) -> Result<String, Failure> {
    let (asked, _) = line.rsplit_once(" -> ").ok_or("no `->`")?;
    let fields: Vec<&str> = asked.split(' ').collect();
    Ok(match fields.as_slice() {
        ["stop_custom", which, state] => {
            let state = usize::try_from(state.parse::<i32>()?)
                .ok()
                .and_then(UnitState::from_index);
            request(stop_custom_animation(
                unit(which)?.then_some(state).flatten(),
            ))
        }
        ["abort"] => request(Some(abort_animation())),
        ["open", which, opens] => request(unit(which)?.then(|| open(*opens == "1"))),
        ["emotion", which, old, emotion] => {
            let mut index: i8 = old.parse()?;
            let frames = set_emotion(unit(which)?.then_some(&mut index), emotion.parse()?);
            format!("{index} {}", frames.map_or(-1, i32::from))
        }
        ["set_seat", which, old, ..] => {
            let mut seat: i8 = old.parse()?;
            scripting_set_seat(unit(which)?.then_some(&mut seat), label(asked)?);
            seat.to_string()
        }
        ["magic_base_seat", ..] => game_seat(magic_base_seat(label(asked)?))?.to_string(),
        ["enterable", which, flags, enterable] => {
            let mut flags = UnitFlags(flags.parse()?);
            if unit(which)? {
                flags.set_enterable_by_player(*enterable == "1");
            }
            flags.0.to_string()
        }
        ["doesnt_drop", "|", objects @ ..] => {
            let mut objects = objects
                .iter()
                .map(|object| {
                    let (is_unit, flags) = object.split_once(':').ok_or("no `:`")?;
                    Ok((is_unit == "1", UnitFlags(flags.parse()?)))
                })
                .collect::<Result<Vec<_>, Failure>>()?;
            doesnt_drop_items(
                objects
                    .iter_mut()
                    .map(|(is_unit, flags)| is_unit.then_some(flags)),
            );
            let flags: Vec<String> = objects
                .iter()
                .map(|(_, flags)| flags.0.to_string())
                .collect();
            flags.join(" ")
        }
        ["grenade_count", _, "|", first, second, is_unit] => {
            let inventory: UnitInventory<u32> = UnitInventory {
                grenades: [first.parse()?, second.parse()?],
                ..UnitInventory::default()
            };
            scripting_grenade_count((*is_unit == "1").then_some(&inventory)).to_string()
        }
        _ => return Err(format!("an unknown case: {line}").into()),
    })
}

#[test]
fn the_unit_requests_are_the_games() -> Result<(), Failure> {
    let mut cases = 0;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (_, expected) = line.rsplit_once(" -> ").ok_or("no `->`")?;
        assert_eq!(case(line)?, expected, "{line}");
        cases += 1;
    }
    assert!(cases > 4900, "only {cases} cases");
    Ok(())
}

#[test]
fn only_a_custom_animation_is_stopped() {
    assert_eq!(stop_custom_animation(Some(UnitState::Idle)), None);
    assert_eq!(
        stop_custom_animation(Some(UnitState::UserAnimation)),
        Some(StateRequest {
            state: UnitState::Idle,
            network_note: true
        })
    );
}
