//! The engine's unit seats and current weapon against the game's: the seat
//! labels, the riders, the seats tested and found, the controlled and blink
//! flags, the weapon in hand's questions, the unzoom and the unarmed melee
//! weapon of tools/engine_unit_seats_reference.py must answer as the
//! game's code answers.

use std::error::Error;

use opence_content::VehicleSeat;
use opence_sim::unit_control::{scripting_crew, seat_filled, seated_riders, Crew, UnitFlags};
use opence_sim::unit_definition::{available_seats, seat_label, test_seat, SeatDesire};
use opence_sim::unit_inventory::{solo_player_night_vision, unarmed_melee_weapon, UnitInventory};

const REFERENCE: &str = include_str!("unit_seats_reference.txt");

/// The game's `_object_dead_bit` in a unit's damage flags.
const DEAD: u32 = 1 << 2;
/// weapon_definitions.h's `_weapon_enables_unit_integrated_night_vision_bit`.
const NIGHT_VISION: u32 = 1 << 14;
/// The fake world's weapon labels, by weapon.
const WEAPON_LABELS: [&str; 8] = [
    "pistol", "rifle", "", "needler", "plasma", "rocket", "sniper", "flag",
];
/// The fake world's first listed weapon definition.
const FIRST_LISTED_WEAPON: i32 = 77;

type Failure = Box<dyn Error>;

/// `None` for the game's -1 (or any negative), otherwise the value.
fn index(field: &str) -> Result<Option<usize>, Failure> {
    optional(field.parse()?)
}

/// `None` for a negative value, otherwise the value.
fn optional(value: i64) -> Result<Option<usize>, Failure> {
    Ok(if value < 0 {
        None
    } else {
        Some(usize::try_from(value)?)
    })
}

fn short(field: &str) -> Result<Option<i16>, Failure> {
    let value: i16 = field.parse()?;
    Ok((value >= 0).then_some(value))
}

fn bit(value: bool) -> u8 {
    u8::from(value)
}

fn quoted(field: &str) -> Result<&str, Failure> {
    Ok(field
        .strip_prefix('\'')
        .and_then(|field| field.strip_suffix('\''))
        .ok_or("an unquoted name")?)
}

fn none_or(value: Option<usize>) -> String {
    value.map_or("-1".into(), |value| value.to_string())
}

/// A seat as the trace prints it: its flags and its quoted label.
fn seat(field: &str) -> Result<(u32, &str), Failure> {
    let (flags, label) = field.split_once(':').ok_or("a seat with no label")?;
    Ok((flags.parse()?, quoted(label)?))
}

/// A slot's weapon as the trace prints it: none, or the weapon, its
/// definition's flags and its zoom-out sound.
#[derive(Clone, Copy, PartialEq)]
struct Weapon {
    index: usize,
    flags: u32,
    sound: Option<usize>,
}

/// Unit 0's inventory from the current weapon, zoom and desired zoom and
/// its four slots.
fn inventory(head: &[&str], slots: &[&str]) -> Result<UnitInventory<Weapon>, Failure> {
    let [current, zoom, desired] = head else {
        return Err("an inventory with no current weapon and zoom".into());
    };
    let mut weapons = [None; 4];
    for (slot, field) in weapons.iter_mut().zip(slots) {
        if *field != "-1" {
            let values: Vec<&str> = field.split(':').collect();
            let [weapon, flags, sound] = values.as_slice() else {
                return Err("a weapon with no flags or sound".into());
            };
            *slot = Some(Weapon {
                index: weapon.parse()?,
                flags: flags.parse()?,
                sound: index(sound)?,
            });
        }
    }
    let zoom_of = |field: &str| -> Result<Option<u8>, Failure> {
        Ok(index(field)?.map(u8::try_from).transpose()?)
    };
    Ok(UnitInventory {
        weapons,
        current_weapon: index(current)?,
        current_zoom: zoom_of(zoom)?,
        desired_zoom: zoom_of(desired)?,
        ..UnitInventory::default()
    })
}

fn night_vision(weapon: &Weapon) -> bool {
    weapon.flags & NIGHT_VISION != 0
}

/// What the engine answers of a case, as the trace prints it after ` ->`.
fn answer(asked: &str) -> Result<String, Failure> {
    let (head, middle) = asked.split_once(" |").unwrap_or((asked, ""));
    let mut head = head.split(' ');
    let kind = head.next().ok_or("an empty line")?;
    let head: Vec<&str> = head.collect();
    let middle: Vec<&str> = middle.split_whitespace().collect();
    Ok(match (kind, head.as_slice()) {
        ("seat_label", [parent, _, base]) => {
            let (asked_seat, seats) = middle.split_last().ok_or("no seat asked")?;
            let labels = seats
                .iter()
                .map(|field| Ok(seat(field)?.1))
                .collect::<Result<Vec<_>, Failure>>()?;
            let riding = match (index(parent)?, index(asked_seat)?) {
                (Some(_), Some(asked_seat)) => Some((labels.as_slice(), asked_seat)),
                _ => None,
            };
            let label = seat_label(riding, base.parse()?).ok_or("no label")?;
            format!(" '{label}'")
        }
        ("riders", [unit]) => {
            let (first_child, units) = middle.split_last().ok_or("no first child")?;
            let units = units
                .iter()
                .map(|field| {
                    field
                        .split(':')
                        .map(|value| Ok(value.parse()?))
                        .collect::<Result<Vec<i64>, Failure>>()
                })
                .collect::<Result<Vec<_>, Failure>>()?;
            let field = |unit: usize, at: usize| -> Result<i64, Failure> {
                Ok(*units
                    .get(unit)
                    .and_then(|fields| fields.get(at))
                    .ok_or("a unit past the six")?)
            };
            let unit = index(unit)?;
            let crew = unit
                .map(|unit| -> Result<Crew<usize>, Failure> {
                    Ok(Crew {
                        driver: optional(field(unit, 2)?)?,
                        gunner: optional(field(unit, 3)?)?,
                    })
                })
                .transpose()?;
            let crew = scripting_crew(crew.as_ref());
            // the unit's children, from its first along each next
            let mut children = Vec::new();
            let mut child = index(first_child)?;
            while let Some(at) = child {
                // biped or vehicle (`_object_mask_unit`)
                let is_unit = field(at, 0)? < 2;
                let seat = i16::try_from(field(at, 1)?)?;
                children.push((at, is_unit, (seat >= 0).then_some(seat)));
                child = optional(field(at, 4)?)?;
            }
            let riders = seated_riders(unit.map(|_| children));
            let listed = riders.as_ref().map_or(String::new(), |riders| {
                riders
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            });
            format!(
                " {} {} [{listed}]{}",
                none_or(crew.driver),
                none_or(crew.gunner),
                if riders.is_none() { " none" } else { "" }
            )
        }
        ("test_seat", [vehicle, unit, name, parent, unit_seat]) => {
            let labels = middle
                .iter()
                .map(|field| Ok(seat(field)?.1))
                .collect::<Result<Vec<_>, Failure>>()?;
            let vehicle = index(vehicle)?;
            let in_vehicle =
                vehicle.is_some() && index(unit)?.is_some() && index(parent)? == vehicle;
            let unit_seat = if in_vehicle { short(unit_seat)? } else { None };
            format!(" {}", bit(test_seat(&labels, quoted(name)?, unit_seat)))
        }
        ("available", [name, desire, maximum]) => {
            let split = middle
                .iter()
                .position(|field| *field == "|")
                .ok_or("no riders")?;
            let (seats, riders) = middle.split_at(split);
            let seats = seats
                .iter()
                .map(|field| seat(field))
                .collect::<Result<Vec<_>, Failure>>()?;
            let riders = riders[1..]
                .iter()
                .map(|field| {
                    let (parent, rider_seat) =
                        field.split_once(':').ok_or("a rider with no seat")?;
                    Ok((index(parent)?, short(rider_seat)?))
                })
                .collect::<Result<Vec<_>, Failure>>()?;
            // the vehicle itself rides nothing
            let riders: Vec<_> = std::iter::once((None, None)).chain(riders).collect();
            let desire = short(desire)?
                .map(SeatDesire::from_index)
                .map(|desire| desire.ok_or("a desire past them"));
            let found = available_seats(
                seats,
                quoted(name)?,
                desire.transpose()?,
                maximum.parse()?,
                |seat| {
                    let seat = i16::try_from(seat).ok();
                    seat_filled(riders.iter().copied(), Some(0), seat)
                },
            );
            found.iter().map(|seat| format!(" {seat}")).collect()
        }
        ("controlled", [flags, wanted, actor, swarm, player, damage, blinker, can_blink]) => {
            let mut unit_flags = UnitFlags(flags.parse()?);
            let someone = [actor, swarm, player]
                .into_iter()
                .any(|field| *field != "-1");
            let dead = damage.parse::<u32>()? & DEAD != 0;
            unit_flags.set_actively_controlled(*wanted == "1", someone, dead);
            let mut calls = String::new();
            for weapon in &middle {
                if *weapon != "-1" {
                    calls.push_str(&format!(" held {weapon}"));
                }
            }
            calls.push_str(" seated 0");
            if *blinker != "-1" {
                unit_flags.set_can_blink(*can_blink == "1");
            }
            format!("{calls} = {}", unit_flags.0)
        }
        ("weapon", [zoom_level, inventory_head @ ..]) => {
            let inventory = inventory(inventory_head, &middle)?;
            let zoom_level = f32::from(zoom_level.parse::<i16>()?);
            let magnification = inventory.zoom_magnification(|weapon| {
                // the fake world's `(weapon + 1) * 1.5 + zoom level`
                f32::from(u16::try_from(weapon.index + 1).unwrap_or(0)) * 1.5 + zoom_level
            });
            let label = inventory.current_weapon_label(|weapon| WEAPON_LABELS[weapon.index]);
            format!(
                " {} {} '{label}' {}",
                magnification.to_bits(),
                bit(inventory.overcharged(|weapon| weapon.index % 3 == 0)),
                bit(inventory.night_vision_active(night_vision))
            )
        }
        ("solo", [count, unit, inventory_head @ ..]) => {
            let inventory = inventory(inventory_head, &middle)?;
            let unit = index(unit)?.map(|_| &inventory);
            let active = solo_player_night_vision(count.parse()?, unit, night_vision);
            format!(" {}", bit(active))
        }
        ("unzoom", [player, inventory_head @ ..]) => {
            let mut inventory = inventory(inventory_head, &middle)?;
            let local_player = index(player)?.is_some();
            let sound = inventory.unzoom(local_player, |weapon| weapon.sound);
            let played = sound.map_or(String::new(), |sound| format!(" sound {sound}"));
            // the caller's night vision power, turned to 0
            format!(
                "{played} unzoomed 0 = {} {} {}",
                none_or(inventory.current_zoom.map(usize::from)),
                none_or(inventory.desired_zoom.map(usize::from)),
                0.0f32.to_bits()
            )
        }
        ("unarmed", [globals, listed]) => {
            let list: Vec<i32> = if *listed == "1" {
                vec![FIRST_LISTED_WEAPON]
            } else {
                Vec::new()
            };
            let globals = (*globals == "1").then_some(list.as_slice());
            let weapon = unarmed_melee_weapon(globals).unwrap_or(-1);
            format!(" {weapon}")
        }
        _ => return Err(format!("an unknown case: {asked}").into()),
    })
}

#[test]
fn the_engine_answers_as_the_game_does() -> Result<(), Failure> {
    let mut cases = 0;
    let mut wrong = Vec::new();
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (asked, expected) = line.rsplit_once(" ->").ok_or("no `->`")?;
        let got = answer(asked)?;
        if got != expected {
            wrong.push(format!("{line}\n  the engine:{got}"));
        }
        cases += 1;
    }
    assert!(cases > 18_000, "only {cases} cases");
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

#[test]
fn a_seat_flag_desire_matches_the_content_flags() {
    let seats = [
        (VehicleSeat::DRIVER, "driver"),
        (VehicleSeat::GUNNER, "gunner"),
    ];
    assert_eq!(
        available_seats(seats, "", Some(SeatDesire::Gunner), 4, |_| false),
        [1]
    );
}
