//! The engine's unit vitality commands against the game's: the 6,000
//! units of tools/engine_unit_vitality_reference.py, each set to a current
//! or a maximum vitality or read for its health or shield, must be left
//! and answered as the game's unit_scripting_commands.c leaves and answers
//! them, and its 3,000 animation lists must start the units the game's
//! starts and answer as it does.

use std::error::Error;

use opence_campaign::start_in_turn;
use opence_sim::damage::Vitality;

const REFERENCE: &str = include_str!("unit_vitality_reference.txt");

type Failure = Box<dyn Error>;

fn real(field: &str) -> Result<f32, Failure> {
    Ok(f32::from_bits(u32::from_str_radix(field, 16)?))
}

/// A unit's vitality and its maximums, from six fields.
fn unit(fields: &[&str]) -> Result<(Vitality, [f32; 2]), Failure> {
    let [dead, depleted, most_body, most_shield, body, shield] = fields else {
        return Err("a unit without its six fields".into());
    };
    let vitality = Vitality {
        body: real(body)?,
        shield: real(shield)?,
        stun_ticks: 0,
        shield_depleted: *depleted == "1",
        dead: *dead == "1",
        region_damage: Vec::new(),
        regions_destroyed: Vec::new(),
    };
    Ok((vitality, [real(most_body)?, real(most_shield)?]))
}

/// A unit as the game prints it.
fn printed(vitality: &Vitality, maximums: [f32; 2]) -> String {
    format!(
        "{} {} {:08x} {:08x} {:08x} {:08x}",
        u8::from(vitality.dead),
        u8::from(vitality.shield_depleted),
        maximums[0].to_bits(),
        maximums[1].to_bits(),
        vitality.body.to_bits(),
        vitality.shield.to_bits()
    )
}

#[test]
fn the_engine_sets_and_reads_vitality_as_the_game_does() -> Result<(), Failure> {
    let mut counts = [0usize; 5];
    let (mut killed, mut short_circuited) = (0, 0);
    for (number, line) in REFERENCE.lines().enumerate() {
        if line.starts_with('#') {
            continue;
        }
        let at = number + 1;
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            [command @ ("current" | "maximum"), body, shield, rest @ ..] if rest.len() == 12 => {
                let (mut vitality, mut maximums) = unit(&rest[..6])?;
                let was_dead = vitality.dead;
                let (body, shield) = (real(body)?, real(shield)?);
                if *command == "current" {
                    counts[0] += 1;
                    vitality.set_current(body, shield, maximums);
                } else {
                    counts[1] += 1;
                    if !vitality.dead {
                        vitality.set_maximums([body, shield]);
                        maximums = [body, shield];
                    }
                }
                killed += usize::from(vitality.dead && !was_dead);
                assert_eq!(
                    printed(&vitality, maximums),
                    rest[6..].join(" "),
                    "line {at}"
                );
            }
            [command @ ("health" | "shield"), rest @ ..] if rest.len() == 7 => {
                let (vitality, _) = unit(&rest[..6])?;
                let answer = if *command == "health" {
                    counts[2] += 1;
                    vitality.script_health()
                } else {
                    counts[3] += 1;
                    vitality.script_shield()
                };
                assert_eq!(answer.to_bits(), real(rest[6])?.to_bits(), "line {at}");
            }
            ["list", rest @ ..] => {
                counts[4] += 1;
                let [entries @ .., "->", asked, answer] = rest else {
                    return Err(format!("line {at}: a list without its answer").into());
                };
                let entries = entries
                    .iter()
                    .map(|entry| {
                        let (is_unit, starts) = entry.split_once(':').ok_or("a list entry")?;
                        Ok((is_unit == "1", starts == "1"))
                    })
                    .collect::<Result<Vec<_>, Failure>>()?;
                let mut called = 0u32;
                let started =
                    start_in_turn(entries.iter().enumerate(), |(index, &(is_unit, starts))| {
                        is_unit.then(|| {
                            called |= 1 << index;
                            starts
                        })
                    });
                short_circuited += usize::from(
                    entries.iter().any(|&(u, _)| u)
                        && called.count_ones() < entries.iter().filter(|&&(u, _)| u).count() as u32,
                );
                assert_eq!(format!("{called:x}"), *asked, "line {at}");
                assert_eq!(u8::from(started).to_string(), *answer, "line {at}");
            }
            _ => return Err(format!("line {at}: an unknown line").into()),
        }
    }
    assert_eq!(counts.iter().sum::<usize>(), 9_000);
    assert!(counts.iter().all(|&count| count > 1_000), "{counts:?}");
    assert!(
        killed > 100 && short_circuited > 200,
        "{killed} {short_circuited}"
    );
    Ok(())
}
