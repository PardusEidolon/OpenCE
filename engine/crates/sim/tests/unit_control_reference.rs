//! The engine's unit control against the game's: the flags set and asked,
//! the mouth moved, the units AI drives and guns, the seats filled, the
//! animation states that are busy and a script's weapon test of
//! tools/engine_unit_control_reference.py must answer as the game's code
//! answers.

use std::error::Error;

use opence_sim::unit_control::{mouth_aperture, ridden_by_ai, seat_filled, UnitFlags};
use opence_sim::unit_inventory::{scripting_has_weapon, UnitInventory};
use opence_sim::unit_states::{busy, UnitState};

const REFERENCE: &str = include_str!("unit_control_reference.txt");

type Failure = Box<dyn Error>;

fn number(field: &str) -> Result<i64, Failure> {
    Ok(field.parse()?)
}

/// `None` for the game's -1 (or any negative), otherwise the value.
fn index(field: &str) -> Result<Option<u32>, Failure> {
    let value = number(field)?;
    Ok(if value < 0 {
        None
    } else {
        Some(u32::try_from(value)?)
    })
}

fn seat(field: &str) -> Result<Option<i16>, Failure> {
    let value: i16 = field.parse()?;
    Ok((value >= 0).then_some(value))
}

fn bit(value: bool) -> u8 {
    u8::from(value)
}

/// The case's fields before ` | ` (without its kind) and after it, up to
/// ` -> `; with no ` | `, none after.
fn parts(line: &str) -> Result<(Vec<&str>, Vec<&str>), Failure> {
    let (asked, _) = line.rsplit_once(" -> ").ok_or("no `->`")?;
    let (head, middle) = asked.split_once(" |").unwrap_or((asked, ""));
    Ok((
        head.split(' ').skip(1).collect(),
        middle.split_whitespace().collect(),
    ))
}

/// Each `a:b:...` field's values.
fn fields<'a>(middle: &[&'a str]) -> Vec<Vec<&'a str>> {
    middle
        .iter()
        .map(|field| field.split(':').collect())
        .collect()
}

fn case(line: &str) -> Result<String, Failure> {
    let kind = line.split(' ').next().ok_or("an empty line")?;
    let (head, middle) = parts(line)?;
    Ok(match (kind, head.as_slice()) {
        ("flags", [flags, way, value, unit]) => {
            let mut unit_flags = UnitFlags(flags.parse()?);
            let value = *value == "1";
            let unit = index(unit)?;
            match *way {
                "0" => unit_flags.set_controllable(value),
                "1" => unit_flags.set_possessed(value),
                // a wish of no unit changes nothing
                "2" if unit.is_some() => unit_flags.desire_flashlight(value),
                "3" => unit_flags.stop_running_blindly(),
                _ => {}
            }
            let flashlight = unit.is_some() && unit_flags.flashlight_on();
            format!(
                "{} {} {}",
                unit_flags.0,
                bit(unit_flags.controllable()),
                bit(flashlight)
            )
        }
        ("mouth", [current, wanted]) => {
            let current = f32::from_bits(current.parse()?);
            let wanted = f32::from_bits(wanted.parse()?);
            mouth_aperture(current, wanted).to_bits().to_string()
        }
        ("riders", []) => {
            let units = fields(&middle)
                .into_iter()
                .map(|values| match values.as_slice() {
                    [actor, driver, gunner] => Ok((index(actor)?, index(driver)?, index(gunner)?)),
                    _ => Err(Failure::from("a rider")),
                })
                .collect::<Result<Vec<_>, _>>()?;
            let has_actor = |unit: u32| {
                usize::try_from(unit)
                    .ok()
                    .and_then(|unit| units.get(unit))
                    .is_some_and(|(actor, _, _)| actor.is_some())
            };
            units
                .iter()
                .map(|(actor, driver, gunner)| {
                    format!(
                        "{}{}",
                        bit(ridden_by_ai(actor.is_some(), driver.map(has_actor))),
                        bit(ridden_by_ai(actor.is_some(), gunner.map(has_actor)))
                    )
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
        ("seat_filled", [unit, asked]) => {
            let riders = fields(&middle)
                .into_iter()
                .map(|values| match values.as_slice() {
                    [parent, parent_seat] => Ok((index(parent)?, seat(parent_seat)?)),
                    _ => Err(Failure::from("a rider")),
                })
                .collect::<Result<Vec<_>, _>>()?;
            bit(seat_filled(riders, index(unit)?, seat(asked)?)).to_string()
        }
        ("busy", [state]) => {
            let state = usize::try_from(number(state)?)
                .ok()
                .and_then(UnitState::from_index);
            bit(busy(state)).to_string()
        }
        ("scripting_has_weapon", [unit, definition]) => {
            let mut inventory: UnitInventory<i64> = UnitInventory::default();
            for (slot, field) in middle.iter().enumerate() {
                let definition = number(field)?;
                *inventory.weapons.get_mut(slot).ok_or("too many slots")? =
                    (definition >= 0).then_some(definition);
            }
            let definition = number(definition)?;
            bit(scripting_has_weapon(
                index(unit)?.map(|_| &inventory),
                (definition >= 0).then_some(definition),
                |weapon| *weapon,
            ))
            .to_string()
        }
        _ => return Err(format!("an unknown case `{line}`").into()),
    })
}

#[test]
fn the_unit_control_answers_as_the_games_code_answers() -> Result<(), Failure> {
    let mut cases = 0;
    let mut wrong = Vec::new();
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (_, expected) = line.rsplit_once(" -> ").ok_or("no `->`")?;
        let got = case(line)?;
        if got != expected {
            wrong.push(format!("{line}\n  the engine: {got}"));
        }
        cases += 1;
    }
    assert!(cases > 10_000, "only {cases} cases");
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
