//! The engine's unit name tables and definition lookups against the game's:
//! the vocalization and speech priority names both ways, the seat flags,
//! the HUD a screen shows and whether a unit holds a weapon, of
//! tools/engine_unit_tables_reference.py, must answer as the game's code
//! answers.

use std::error::Error;

use opence_content::VehicleSeat;
use opence_sim::unit_definition::{active_hud, seat_is};
use opence_sim::unit_dialogue::{
    speech_priority_by_name, speech_priority_name, vocalization_by_name, vocalization_name,
};
use opence_sim::unit_inventory::UnitInventory;

const REFERENCE: &str = include_str!("unit_tables_reference.txt");

type Failure = Box<dyn Error>;

/// The game's answer for a name: `<error>` for none.
fn name(name: Option<&str>) -> String {
    name.unwrap_or("<error>").to_string()
}

/// `-1` for none, otherwise the index.
fn index(value: Option<usize>) -> String {
    value.map_or_else(|| String::from("-1"), |value| value.to_string())
}

/// The name a lookup asks for, between its quotes.
fn quoted(rest: &str) -> Result<&str, Failure> {
    let rest = rest.strip_prefix('\'').ok_or("no opening quote")?;
    Ok(rest.strip_suffix('\'').ok_or("no closing quote")?)
}

/// The numbers of a case's list, after its `|`.
fn numbers<T: std::str::FromStr>(list: &str) -> Result<Vec<T>, Failure>
where
    T::Err: Error + 'static,
{
    Ok(list
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?)
}

/// The HUD of `huds` a screen shows, as the trace names it.
fn hud(huds: &[i64], multiplayer: &str) -> Result<String, Failure> {
    let chosen = active_hud(huds.len(), multiplayer != "0");
    Ok(chosen
        .and_then(|chosen| huds.get(chosen))
        .map_or_else(|| String::from("-1"), ToString::to_string))
}

fn case(asked: &str) -> Result<String, Failure> {
    let (kind, rest) = asked.split_once(' ').ok_or("no case")?;
    let (head, list) = rest.split_once(" |").unwrap_or((rest, ""));
    let fields: Vec<&str> = head.split(' ').collect();
    Ok(match (kind, fields.as_slice()) {
        ("vocalization", [vocalization, abbreviated]) => name(vocalization_name(
            vocalization.parse()?,
            *abbreviated == "1",
        )),
        ("vocalization_index", _) => index(vocalization_by_name(quoted(rest)?)),
        ("priority", [priority]) => name(speech_priority_name(priority.parse()?)),
        ("priority_index", _) => speech_priority_by_name(quoted(rest)?).to_string(),
        ("seat", [seat]) => {
            let flags: Vec<u32> = numbers(list)?;
            let seat: i16 = seat.parse()?;
            let answers = [
                VehicleSeat::DRIVER,
                VehicleSeat::GUNNER,
                VehicleSeat::ALLOWS_NONCOMBATANTS,
            ]
            .map(|flag| u8::from(seat_is(&flags, seat, flag)).to_string());
            answers.join(" ")
        }
        ("hud", [multiplayer]) | ("seat_hud", [_, multiplayer]) => {
            hud(&numbers(list)?, multiplayer)?
        }
        ("has_weapon", [weapon]) => {
            let slots: Vec<i64> = numbers(list)?;
            let mut inventory = UnitInventory::default();
            for (slot, object) in inventory.weapons.iter_mut().zip(slots) {
                *slot = (object >= 0).then_some(object);
            }
            let weapon: i64 = weapon.parse()?;
            let weapon = (weapon >= 0).then_some(weapon);
            u8::from(inventory.holds(weapon.as_ref())).to_string()
        }
        _ => return Err(format!("an unknown case `{asked}`").into()),
    })
}

#[test]
fn the_unit_tables_answer_as_the_games_code_answers() -> Result<(), Failure> {
    let mut cases = 0;
    let mut wrong = Vec::new();
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (asked, expected) = line.rsplit_once(" -> ").ok_or("no `->`")?;
        let got = case(asked)?;
        if got != expected {
            wrong.push(format!("{line}\n  the engine: {got}"));
        }
        cases += 1;
    }
    assert!(cases > 4_500, "only {cases} cases");
    assert!(
        wrong.is_empty(),
        "{} of {cases} differ:\n{}",
        wrong.len(),
        wrong
            .iter()
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(())
}
