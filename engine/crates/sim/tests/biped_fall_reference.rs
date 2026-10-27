//! A biped's falls hurt it as the game's do:
//! tools/engine_biped_fall_reference.py runs bipeds.c's
//! `biped_falling_damage` over random landings and falls (immune, flying
//! and dead now and then), and the engine's `fall` is run on the same, its
//! velocities a second where the game's are a tick, and compared: whether
//! it hurts, with which damage and at what scale.

use opence_content::FallingDamage;
use opence_sim::damage::{fall, Fall};

const REFERENCE: &str = include_str!("biped_fall_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// The game's ticks a second, which its velocities are in.
const TICKS_PER_SECOND: f32 = 30.0;

#[test]
fn the_biped_falls_are_the_games() -> Result<(), Failure> {
    let (mut landings, mut greatest, mut none) = (0, 0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (case, result) = line.split_once(" -> ").ok_or("a line with no result")?;
        let case: Vec<&str> = case.split_whitespace().skip(1).collect();
        let [minimum, maximum, fastest, landing, upward, immune, flying, dead] = case[..] else {
            return Err(format!("{line}: a case of {} fields", case.len()).into());
        };
        let second = |field: &str| Ok::<_, Failure>(field.parse::<f32>()? * TICKS_PER_SECOND);
        let falling = FallingDamage {
            minimum_damage_velocity: second(minimum)?,
            maximum_damage_velocity: second(maximum)?,
            maximum_falling_velocity: second(fastest)?,
            falling_damage: None,
            maximum_distance_damage: None,
            vehicle_collision_damage: None,
        };
        let engine = fall(
            &falling,
            second(landing)?,
            second(upward)?,
            immune == "1",
            flying == "1",
            dead == "1",
        );
        let result: Vec<&str> = result.split_whitespace().collect();
        match (engine, result.as_slice()) {
            (None, ["none"]) => none += 1,
            (Some(Fall::Greatest), ["1", "1"]) => greatest += 1,
            // (the scale's from velocities a second, not a tick: within a
            // millionth)
            (Some(Fall::Landing(scale)), ["0", game])
                if (scale - game.parse::<f32>()?).abs() <= 1.0e-6 =>
            {
                landings += 1;
            }
            (engine, _) => return Err(format!("{line}\nthe engine's fall is {engine:?}").into()),
        }
    }
    assert_eq!((landings, greatest, none), (2012, 654, 1334));
    Ok(())
}
