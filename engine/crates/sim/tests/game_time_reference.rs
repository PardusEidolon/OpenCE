//! The engine's clock against the game's: the 64 runs of
//! tools/engine_game_time_reference.py (frames of random lengths, stalls and
//! frames of nothing, at the game's speeds) are played again through a
//! `Clock` at the game's 30 ticks a second and cap of seven: each frame
//! must run the ticks it ran in the game and leave the same time over, to
//! the bit.

use std::error::Error;

use opence_sim::clock::Clock;

const REFERENCE: &str = include_str!("game_time_reference.txt");

type Failure = Box<dyn Error>;

/// An f32 written as its bits in hex.
fn float(bits: &str) -> Result<f32, Failure> {
    Ok(f32::from_bits(u32::from_str_radix(bits, 16)?))
}

#[test]
fn the_clock_runs_the_ticks_the_games_clock_runs() -> Result<(), Failure> {
    let (mut clock, mut speed) = (Clock::at_rate(30, 7), 1.0);
    let (mut runs, mut frames, mut capped) = (0, 0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["run", bits] => {
                speed = float(bits)?;
                clock = Clock::at_rate(30, 7);
                runs += 1;
            }
            ["frame", seconds, ticks, leftover] => {
                let frame = clock.advance(f64::from(float(seconds)?), f64::from(speed));
                let ticks: u32 = ticks.parse()?;
                assert_eq!(frame.ticks, ticks, "run {runs}: {line}");
                assert_eq!(
                    clock.leftover().to_bits(),
                    float(leftover)?.to_bits(),
                    "run {runs}: {line}"
                );
                frames += 1;
                capped += usize::from(ticks == 7);
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!((runs, frames), (64, 64 * 160));
    assert!(capped > 500, "{capped} frames at the cap");
    Ok(())
}
