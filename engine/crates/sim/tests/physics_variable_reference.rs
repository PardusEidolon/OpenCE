//! The engine's physics variables against the game's: every call of
//! tools/engine_physics_variable_reference.py (the speed's update and seek,
//! the position's update, seek and seek direction, and the position moved
//! by its speed, over runs of parameters, limits, wraps and targets) is
//! made again with the same arguments, and must leave what the game's
//! left, to the bit.

use std::error::Error;

use opence_sim::physics_variables::{
    position_update, position_update_seek, seek_direction, speed_update, speed_update_seek, update,
    update_seek, SpeedParameters, Variable,
};

const REFERENCE: &str = include_str!("physics_variable_reference.txt");

type Failure = Box<dyn Error>;

/// A line's numbers, after its name.
fn numbers(line: &str) -> Result<Vec<f32>, Failure> {
    Ok(line
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .collect::<Result<_, _>>()?)
}

fn parameters(at: &[f32]) -> SpeedParameters {
    SpeedParameters {
        positive: at[0],
        negative: at[1],
        acceleration: at[2],
        deceleration: at[3],
    }
}

/// The same bits (a zero's sign included).
fn assert_bits(engine: f32, game: f32, what: &str, line: &str) {
    assert_eq!(
        engine.to_bits(),
        game.to_bits(),
        "{what}: {engine} for {game}: {line}"
    );
}

#[test]
fn physics_variables_move_as_the_games_do() -> Result<(), Failure> {
    let mut calls = [0usize; 7];
    let mut there = 0;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let n = numbers(line)?;
        let name = line.split_whitespace().next().unwrap_or_default();
        match (name, n.as_slice()) {
            ("speed", [p @ .., speed, delta, after]) if p.len() == 4 => {
                let mut s = *speed;
                speed_update(&mut s, parameters(p), *delta);
                assert_bits(s, *after, "speed", line);
                calls[0] += 1;
            }
            ("speed_seek", [p @ .., speed, target, delta, after, done]) if p.len() == 4 => {
                let mut s = *speed;
                let reached = speed_update_seek(&mut s, parameters(p), *target, *delta);
                assert_bits(s, *after, "speed", line);
                assert_eq!(reached, *done != 0.0, "there: {line}");
                there += usize::from(reached);
                calls[1] += 1;
            }
            ("direction", [high, low, wrap, position, target, direction]) => {
                let found = seek_direction([*high, *low], *position, *wrap != 0.0, *target);
                assert_bits(found, *direction, "direction", line);
                calls[2] += 1;
            }
            ("position", [high, low, wrap, position, delta, after]) => {
                let mut p = *position;
                position_update(&mut p, [*high, *low], *wrap != 0.0, *delta);
                assert_bits(p, *after, "position", line);
                calls[3] += 1;
            }
            ("position_seek", [high, low, wrap, position, target, delta, after, done]) => {
                let mut p = *position;
                let reached =
                    position_update_seek(&mut p, [*high, *low], *wrap != 0.0, *target, *delta);
                assert_bits(p, *after, "position", line);
                assert_eq!(reached, *done != 0.0, "there: {line}");
                there += usize::from(reached);
                calls[4] += 1;
            }
            (
                "update",
                [high, low, p @ .., wrap, position, speed, delta, position_after, speed_after],
            ) if p.len() == 4 => {
                let variable = Variable {
                    limits: [*high, *low],
                    speed: parameters(p),
                };
                let (mut at, mut s) = (*position, *speed);
                update(&mut at, &mut s, variable, *wrap != 0.0, *delta);
                assert_bits(at, *position_after, "position", line);
                assert_bits(s, *speed_after, "speed", line);
                calls[5] += 1;
            }
            (
                "update_seek",
                [high, low, p @ .., wrap, position, speed, target, delta, position_after, speed_after, done],
            ) if p.len() == 4 => {
                let variable = Variable {
                    limits: [*high, *low],
                    speed: parameters(p),
                };
                let (mut at, mut s) = (*position, *speed);
                let reached = update_seek(&mut at, &mut s, variable, *wrap != 0.0, *target, *delta);
                assert_bits(at, *position_after, "position", line);
                assert_bits(s, *speed_after, "speed", line);
                assert_eq!(reached, *done != 0.0, "there: {line}");
                there += usize::from(reached);
                calls[6] += 1;
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert!(calls.iter().all(|&count| count == 1800), "{calls:?}");
    assert!(there >= 250, "{there} reached their targets");
    Ok(())
}
