//! The engine's seeds draw the game's numbers: tools/engine_random_reference.py
//! runs real_math.h's random helpers (`random_range`, `real_random_range`,
//! `random_direction3d`, their local seed's twins and the rest) over the
//! game's seed and its local seed, and each line is drawn again from a
//! `RandomSeed` holding the seed before it. The engine's local seeds (the
//! effects', the sounds') are `RandomSeed`s of their own, so the local
//! helpers are its methods too.

use glam::Vec3;
use opence_sim::random::RandomSeed;

const REFERENCE: &str = include_str!("random_reference.txt");

type Failure = Box<dyn std::error::Error>;

fn number<T: std::str::FromStr>(field: Option<&&str>) -> Result<T, Failure>
where
    T::Err: std::error::Error + 'static,
{
    Ok(field.ok_or("a short line")?.parse::<T>()?)
}

fn vector(fields: &[&str]) -> Result<Vec3, Failure> {
    let [i, j, k] = fields else {
        return Err(format!("a vector of {} numbers", fields.len()).into());
    };
    Ok(Vec3::new(i.parse()?, j.parse()?, k.parse()?))
}

#[test]
fn each_helper_draws_the_games_numbers() -> Result<(), Failure> {
    let (mut compared, mut wrong) = (0, Vec::new());
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.first() == Some(&"locked") {
            continue;
        }
        let mut seed = RandomSeed(number(fields.get(1))?);
        let after: u32 = number(fields.last())?;
        let drawn = &fields[3..fields.len() - 1];
        let same = match fields.get(2).copied() {
            Some("random") => u32::from(seed.next_u16()) == number::<u32>(drawn.first())?,
            Some("range") => {
                let (lower, upper) = (number(drawn.first())?, number(drawn.get(1))?);
                seed.next_range(lower, upper) == number::<i16>(drawn.get(2))?
            }
            // (`random() > 0x8000`)
            Some("boolean") => u8::from(seed.next_u16() > 0x8000) == number::<u8>(drawn.first())?,
            Some("real") => seed.next_real() == number::<f32>(drawn.first())?,
            Some("real_range") => {
                let (lower, upper) = (number(drawn.first())?, number(drawn.get(1))?);
                seed.next_real_range(lower, upper) == number::<f32>(drawn.get(2))?
            }
            Some("direction") => seed.next_direction() == vector(drawn)?,
            Some("cone") => {
                let axis = vector(&drawn[..3])?;
                let (inner, outer) = (number(drawn.get(3))?, number(drawn.get(4))?);
                seed.vector_in_cone(axis, inner, outer) == vector(&drawn[5..])?
            }
            _ => return Err(format!("a line the test does not know: {line}").into()),
        };
        compared += 1;
        if !same || seed.0 != after {
            wrong.push(format!("{line}: the engine's seed after is {}", seed.0));
        }
    }
    assert!(compared >= 3000, "{compared} lines");
    assert!(
        wrong.is_empty(),
        "{} of {compared} differ:\n{}",
        wrong.len(),
        wrong[..wrong.len().min(20)].join("\n")
    );
    Ok(())
}

#[test]
fn a_locked_seed_still_draws_while_no_game_engine_runs() -> Result<(), Failure> {
    let line = REFERENCE
        .lines()
        .find(|line| line.starts_with("locked"))
        .ok_or("no locked line")?;
    let fields: Vec<&str> = line.split_whitespace().collect();
    let mut seed = RandomSeed(number(fields.get(1))?);
    assert_eq!(seed.next_range(0, 100), number::<i16>(fields.get(2))?);
    assert_eq!(seed.0, number::<u32>(fields.get(3))?);
    Ok(())
}
