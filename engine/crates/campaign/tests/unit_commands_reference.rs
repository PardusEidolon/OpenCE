//! The engine's small unit commands against the game's: the grenade
//! counts, readied weapons, custom animation times, imperviousness and seat lists of
//! tools/engine_unit_commands_reference.py must answer as the game's code
//! answers.

use std::collections::BTreeSet;
use std::error::Error;

use opence_campaign::{
    custom_animation_time, grenade_total, seat_rider_listed, set_impervious, weapon_readied,
};

const REFERENCE: &str = include_str!("unit_commands_reference.txt");

type Failure = Box<dyn Error>;

fn number(field: &str) -> Result<i64, Failure> {
    Ok(field.parse()?)
}

/// A seat list's case: the vehicle, the seat asked for, the labels, the
/// objects (is a unit, parent, seat) and the list, as the line gives them.
fn seats(fields: &[&str]) -> Result<bool, Failure> {
    let [vehicle, asked, rest @ ..] = fields else {
        return Err("a seat list without its vehicle".into());
    };
    let mut parts = rest.split(|field| *field == "|");
    let labels: Vec<&str> = parts.next().ok_or("no labels")?.to_vec();
    let objects = parts.next().ok_or("no objects")?;
    let list = parts.next().ok_or("no list")?;
    let list: Vec<usize> = list
        .iter()
        .map(|field| field.parse())
        .collect::<Result<_, _>>()?;
    if *vehicle == "-1" {
        return Ok(false);
    }
    // the vehicle is object 0; the others are objects 1 on, in order
    let mut riders = Vec::new();
    for (at, object) in objects.iter().enumerate() {
        let mut values = object.split(':');
        let unit = values.next().ok_or("is a unit")? == "1";
        let parent = number(values.next().ok_or("a parent")?)?;
        let seat: usize = values.next().ok_or("a seat")?.parse()?;
        if unit && parent == 0 {
            riders.push((at + 1, seat));
        }
    }
    Ok(seat_rider_listed(&labels, &riders, asked, &list))
}

/// An imperviousness case: the objects (is a unit, impervious) and the
/// list; the answer is each object's flag after the command.
fn impervious(on: &str, fields: &[&str]) -> Result<String, Failure> {
    let mut parts = fields.split(|field| *field == "|");
    let objects = parts.next().ok_or("no objects")?;
    let list = parts.next().ok_or("no list")?;
    let mut units = BTreeSet::new();
    let mut set = BTreeSet::new();
    for (at, object) in objects.iter().enumerate() {
        let (unit, flag) = object.split_once(':').ok_or("an object")?;
        if unit == "1" {
            units.insert(at);
        }
        if flag == "1" {
            set.insert(at);
        }
    }
    let list: Vec<usize> = list
        .iter()
        .map(|field| field.parse())
        .collect::<Result<_, _>>()?;
    set_impervious(
        &mut set,
        list.into_iter().filter(|object| units.contains(object)),
        on == "1",
    );
    // scenery keeps whatever flag it had; the engine sets only units
    let after: Vec<String> = objects
        .iter()
        .enumerate()
        .map(|(at, object)| {
            if units.contains(&at) {
                u8::from(set.contains(&at)).to_string()
            } else {
                object
                    .split_once(':')
                    .map_or("0", |(_, flag)| flag)
                    .to_string()
            }
        })
        .collect();
    Ok(after.join(" "))
}

#[test]
fn the_engine_answers_the_unit_commands_as_the_game_does() -> Result<(), Failure> {
    let (mut cases, mut listed, mut ticking) = (0, 0, 0);
    for (number_of_line, line) in REFERENCE.lines().enumerate() {
        if line.starts_with('#') {
            continue;
        }
        let at = number_of_line + 1;
        let fields: Vec<&str> = line.split_whitespace().collect();
        let (got, expected) = match fields.as_slice() {
            ["grenades", unit, is_unit, counts @ .., answer] => {
                let counts: Vec<i16> =
                    counts.iter().map(|c| c.parse()).collect::<Result<_, _>>()?;
                let total = if *unit != "-1" && *is_unit == "1" {
                    grenade_total(&counts)
                } else {
                    0
                };
                (total.to_string(), answer.to_string())
            }
            ["readied", unit, current, slots @ .., asked, answer] => {
                let slots: Vec<i64> = slots.iter().map(|s| number(s)).collect::<Result<_, _>>()?;
                let held = usize::try_from(number(current)?)
                    .ok()
                    .and_then(|slot| slots.get(slot))
                    .filter(|definition| **definition != -1)
                    .filter(|_| *unit != "-1");
                let asked = Some(number(asked)?).filter(|definition| *definition != -1);
                let readied = weapon_readied(held.copied(), asked);
                (u8::from(readied).to_string(), answer.to_string())
            }
            ["time", unit, playing, frames, frame, stopped, playing_after, answer] => {
                // the engine forgets a stopped animation; an animation
                // left playing has its frames left
                let playing = *unit != "-1" && *playing == "1" && *stopped == "0";
                let left = u64::try_from(number(frames)? - number(frame)?)?;
                let time = custom_animation_time(playing.then_some(left));
                ticking += usize::from(time > 0);
                let after = u8::from(playing || (*unit == "-1" && *playing_after == "1"));
                assert_eq!(after.to_string(), *playing_after, "line {at}: {line}");
                (time.to_string(), answer.to_string())
            }
            ["impervious", on, rest @ ..] => {
                let arrow = rest
                    .iter()
                    .position(|field| *field == "->")
                    .ok_or("no answer")?;
                let (case, answer) = rest.split_at(arrow);
                let answer = answer.get(1..).ok_or("no answer")?.join(" ");
                (impervious(on, case)?, answer)
            }
            ["seats", rest @ ..] => {
                let (rest, answer) = rest.split_at(rest.len().saturating_sub(2));
                let [arrow, answer] = answer else {
                    return Err(format!("line {at}: no answer").into());
                };
                assert_eq!(*arrow, "->");
                let found = seats(rest)?;
                listed += usize::from(found);
                (u8::from(found).to_string(), answer.to_string())
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        };
        assert_eq!(got, expected, "line {at}: {line}");
        cases += 1;
    }
    assert_eq!(cases, 10000);
    assert!(listed > 300, "{listed}");
    assert!(ticking > 800, "{ticking}");
    Ok(())
}
