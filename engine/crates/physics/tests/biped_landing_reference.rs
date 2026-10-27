//! A biped recovers from its landings as the game's do:
//! tools/engine_biped_landing_reference.py runs bipeds.c's
//! `biped_start_landing` over random landings with random biped tags (some
//! with an empty range of soft or hard landings), and the engine's
//! `landing_recovery` is run on the same and compared: whether it lands
//! soft, hard or not at all, and for how many of the game's ticks.

use opence_content::Biped;
use opence_physics::{landing_recovery, Landing};

const REFERENCE: &str = include_str!("biped_landing_reference.txt");

type Failure = Box<dyn std::error::Error>;

#[test]
fn the_biped_landings_are_the_games() -> Result<(), Failure> {
    let (mut soft, mut hard, mut none) = (0, 0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (case, result) = line.split_once(" -> ").ok_or("a line with no result")?;
        let case: Vec<f32> = case
            .split_whitespace()
            .skip(1)
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let [soft_time, hard_time, minimum_soft, minimum_hard, maximum_hard, velocity] = case[..]
        else {
            return Err(format!("{line}: a case of {} fields", case.len()).into());
        };
        let biped = Biped {
            maximum_soft_landing_time: soft_time,
            maximum_hard_landing_time: hard_time,
            minimum_soft_landing_velocity: minimum_soft,
            minimum_hard_landing_velocity: minimum_hard,
            maximum_hard_landing_velocity: maximum_hard,
            ..Biped::default()
        };
        let engine = landing_recovery(&biped, velocity);
        let result: Vec<&str> = result.split_whitespace().collect();
        let game = match result.as_slice() {
            ["none"] => None,
            ["0", ticks] => Some((Landing::Soft, ticks.parse()?)),
            ["1", ticks] => Some((Landing::Hard, ticks.parse()?)),
            _ => return Err(format!("{line}: an unknown result").into()),
        };
        if engine != game {
            return Err(format!("{line}\nthe engine's landing is {engine:?}").into());
        }
        match engine {
            Some((Landing::Soft, _)) => soft += 1,
            Some((Landing::Hard, _)) => hard += 1,
            None => none += 1,
        }
    }
    assert_eq!((soft, hard, none), (409, 3097, 494));
    Ok(())
}
