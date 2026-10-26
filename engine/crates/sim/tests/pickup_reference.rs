//! The engine's choice of what a player does with a weapon at its feet
//! against the game's: the 4000 cases of tools/engine_pickup_reference.py
//! (random inventories of up to four weapons of five kinds, with random
//! flags and ages, the trigger held or not, a CTF flag forced or not) must
//! each choose as the game chose.

use std::error::Error;

use opence_sim::pickup::{choose, Choice, Seen};

const REFERENCE: &str = include_str!("pickup_reference.txt");

type Failure = Box<dyn Error>;

/// A weapon written as `kind/age`, the age as f32 bits in hex; `None` for
/// an empty place (kind -1).
fn weapon(field: &str, flags: &[u32]) -> Result<Option<Seen>, Failure> {
    let (kind, age) = field.split_once('/').ok_or("a weapon without an age")?;
    let kind: i64 = kind.parse()?;
    if kind < 0 {
        return Ok(None);
    }
    let kind = usize::try_from(kind)?;
    Ok(Some(Seen {
        kind,
        flags: *flags.get(kind).ok_or("a kind without flags")?,
        age: f32::from_bits(u32::from_str_radix(age, 16)?),
    }))
}

#[test]
fn the_engine_chooses_as_the_game_chooses() -> Result<(), Failure> {
    let mut cases = 0;
    let mut chosen = [0; 3];
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let ["case", current, trigger, forced, places @ .., expected] = fields.as_slice() else {
            return Err(format!("an unknown line: {line}").into());
        };
        let (weapons, flags) = places.split_at(5);
        let flags = flags
            .iter()
            .map(|flags| u32::from_str_radix(flags, 16))
            .collect::<Result<Vec<_>, _>>()?;
        let (carried, lying) = weapons.split_at(4);
        let carried = carried
            .iter()
            .map(|field| weapon(field, &flags))
            .collect::<Result<Vec<_>, _>>()?;
        let lying = weapon(lying.first().ok_or("no weapon lying")?, &flags)?
            .ok_or("the lying weapon is empty")?;
        let current = usize::try_from(current.parse::<i64>()?).ok();
        let choice = choose(&carried, current, &lying, *trigger == "1", *forced == "1");
        let expected = match *expected {
            "take" => Choice::Take,
            "add" => Choice::Add,
            "swap" => Choice::Swap,
            "leave" => Choice::Leave,
            other => return Err(format!("an unknown choice: {other}").into()),
        };
        assert_eq!(choice, expected, "{line}");
        cases += 1;
        match choice {
            Choice::Take => chosen[0] += 1,
            Choice::Swap => chosen[1] += 1,
            Choice::Leave => chosen[2] += 1,
            Choice::Add => {}
        }
    }
    assert_eq!(cases, 4000);
    assert!(chosen.iter().all(|&count| count > 100), "{chosen:?}");
    Ok(())
}
