//! The engine's unit inventory against the game's: the weapon and grenade
//! it picks, the free slot, the grenades added, the weapons it has, the
//! weapon in a slot, the zoom level and the base seat and weapon labels of
//! tools/engine_unit_inventory_reference.py must answer as the game's code
//! answers.

use std::error::Error;

use opence_sim::unit_inventory::{UnitInventory, WEAPONS_PER_UNIT};
use opence_sim::unit_states::{base_seat_index, base_seat_label, base_weapon_label};

const REFERENCE: &str = include_str!("unit_inventory_reference.txt");

type Failure = Box<dyn Error>;

/// A weapon in a slot, as the trace draws it.
#[derive(Clone, Copy, Debug)]
struct Weapon {
    /// Its object (the slot's index and one).
    object: i64,
    usable: bool,
    must_be_readied: bool,
    definition: i64,
    flags: u32,
}

fn number(field: &str) -> Result<i64, Failure> {
    Ok(field.parse()?)
}

/// `None` for the game's -1, otherwise the index.
fn index(field: &str) -> Result<Option<usize>, Failure> {
    let value = number(field)?;
    Ok(if value < 0 {
        None
    } else {
        Some(usize::try_from(value)?)
    })
}

fn answer(value: Option<usize>) -> String {
    value.map_or_else(|| String::from("-1"), |value| value.to_string())
}

/// The case's fields before ` | `, after it up to ` -> `, and after that.
fn parts(line: &str) -> Result<(Vec<&str>, Vec<&str>, &str), Failure> {
    let (head, rest) = line.split_once(" | ").ok_or("no `|`")?;
    let (middle, expected) = rest.split_once(" -> ").ok_or("no `->`")?;
    Ok((
        head.split(' ').skip(1).collect(),
        middle.split(' ').collect(),
        expected,
    ))
}

/// The unit's four slots, as the trace prints them.
fn slots(fields: &[&str]) -> Result<UnitInventory<Weapon>, Failure> {
    let mut inventory = UnitInventory::default();
    if fields.len() != WEAPONS_PER_UNIT {
        return Err("not four slots".into());
    }
    for (slot, field) in fields.iter().enumerate() {
        let values: Vec<&str> = field.split(':').collect();
        match values.as_slice() {
            ["-1", last_used] => inventory.last_used[slot] = last_used.parse()?,
            [usable, must, last_used, definition, flags] => {
                inventory.last_used[slot] = last_used.parse()?;
                inventory.weapons[slot] = Some(Weapon {
                    object: i64::try_from(slot)? + 1,
                    usable: *usable == "1",
                    must_be_readied: *must == "1",
                    definition: number(definition)?,
                    flags: flags.parse()?,
                });
            }
            _ => return Err(format!("a slot `{field}`").into()),
        }
    }
    Ok(inventory)
}

fn grenades(fields: &[&str]) -> Result<[i8; 2], Failure> {
    let [first, second] = fields else {
        return Err("not two grenade counts".into());
    };
    Ok([first.parse()?, second.parse()?])
}

fn case(line: &str) -> Result<String, Failure> {
    let kind = line.split(' ').next().ok_or("an empty line")?;
    if let Some(rest) = line.strip_prefix("seat_label ") {
        let (asked, _) = rest.split_once(" -> ").ok_or("no `->`")?;
        return Ok(base_seat_label(asked.parse()?)
            .ok_or("no label")?
            .to_string());
    }
    if let Some(rest) = line.strip_prefix("seat_index '") {
        let (asked, _) = rest.split_once("' -> ").ok_or("no `->`")?;
        return Ok(answer(base_seat_index(asked)));
    }
    if line.starts_with("weapon_label ") {
        return Ok(base_weapon_label(0).ok_or("no label")?.to_string());
    }
    let (head, middle, _) = parts(line)?;
    Ok(match kind {
        "next_weapon" => {
            let [current, delta] = head.as_slice() else {
                return Err("a next weapon's head".into());
            };
            let inventory = slots(&middle)?;
            answer(inventory.next_weapon(
                index(current)?,
                delta.parse()?,
                |weapon| weapon.usable,
                |weapon| weapon.must_be_readied,
            ))
        }
        "next_grenade" => {
            let [current, delta] = head.as_slice() else {
                return Err("a next grenade's head".into());
            };
            let inventory: UnitInventory<Weapon> = UnitInventory {
                grenades: grenades(&middle)?,
                ..UnitInventory::default()
            };
            answer(inventory.next_grenade(index(current)?, delta.parse()?))
        }
        "first_free" => answer(slots(&middle)?.first_free_weapon()),
        "add_grenades" => {
            let [kind, count, current, desired] = head.as_slice() else {
                return Err("an add grenades' head".into());
            };
            let mut inventory: UnitInventory<Weapon> = UnitInventory {
                current_grenade: index(current)?,
                desired_grenade: index(desired)?,
                grenades: grenades(&middle)?,
                ..UnitInventory::default()
            };
            let added = inventory.add_grenades(kind.parse()?, count.parse()?);
            format!(
                "{added} {} {} {} {}",
                inventory.grenades[0],
                inventory.grenades[1],
                answer(inventory.current_grenade),
                answer(inventory.desired_grenade)
            )
        }
        "has" => {
            let [definition, flag] = head.as_slice() else {
                return Err("a has' head".into());
            };
            let definition = number(definition)?;
            let flag: u32 = flag.parse()?;
            let inventory = slots(&middle)?;
            format!(
                "{} {} {}",
                u8::from(inventory.has_weapon(|weapon| weapon.definition == definition)),
                u8::from(inventory.has_weapon(|weapon| weapon.flags & (1 << flag) != 0)),
                answer(inventory.must_be_readied_weapon(|weapon| weapon.must_be_readied))
            )
        }
        "grenade_count" => {
            let [kind, current] = head.as_slice() else {
                return Err("a grenade count's head".into());
            };
            let inventory: UnitInventory<Weapon> = UnitInventory {
                current_grenade: index(current)?,
                grenades: grenades(&middle)?,
                ..UnitInventory::default()
            };
            let kind = number(kind)?;
            let kind = if kind < 0 {
                None
            } else {
                Some(i16::try_from(kind)?)
            };
            format!(
                "{} {}",
                inventory.grenade_count(kind),
                answer(inventory.current_grenade_type())
            )
        }
        "weapon" => {
            let [asked, zoom] = head.as_slice() else {
                return Err("a weapon's head".into());
            };
            let mut inventory = slots(&middle)?;
            let zoom = number(zoom)?;
            inventory.current_zoom = if zoom < 0 {
                None
            } else {
                Some(u8::try_from(zoom)?)
            };
            let object = inventory
                .weapon(index(asked)?)
                .map_or(-1, |weapon| weapon.object);
            let zoom = inventory.zoom_level().map_or(-1, i64::from);
            format!("{object} {object} {zoom}")
        }
        _ => return Err(format!("an unknown case `{kind}`").into()),
    })
}

#[test]
fn the_unit_inventory_answers_as_the_games_code_answers() -> Result<(), Failure> {
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
    assert!(cases > 12_000, "only {cases} cases");
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
