//! The engine's slowing of a projectile against the game's: the 4,000
//! definitions of tools/engine_projectile_deceleration_reference.py (random
//! speeds and damage ranges, in the air or under water) are slowed again
//! through `Deceleration::of`, and each deceleration, damage distance and
//! timer must be the game's, to the bit.

use std::error::Error;

use opence_sim::projectile::Deceleration;

const REFERENCE: &str = include_str!("projectile_deceleration_reference.txt");

type Failure = Box<dyn Error>;

/// An f32 written as its bits in hex.
fn float(bits: &str) -> Result<f32, Failure> {
    Ok(f32::from_bits(u32::from_str_radix(bits, 16)?))
}

#[test]
fn a_projectile_slows_as_the_games_projectiles_slow() -> Result<(), Failure> {
    let (mut cases, mut slowing, mut held) = (0, 0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let ["case", initial, last, air_lower, air_upper, water_lower, water_upper, underwater, before, deceleration, distance, timer, delta] =
            fields.as_slice()
        else {
            return Err(format!("an unknown line: {line}").into());
        };
        let got = Deceleration::of(
            float(initial)?,
            float(last)?,
            [float(air_lower)?, float(air_upper)?],
            [float(water_lower)?, float(water_upper)?],
            *underwater == "1",
        );
        // (a timer it does not set is left as it was)
        let got_timer = got.timer.unwrap_or(float(before)?);
        assert_eq!(
            [
                got.deceleration.to_bits(),
                got.maximum_damage_distance.to_bits(),
                got_timer.to_bits(),
                got.timer_delta.to_bits(),
            ],
            [
                float(deceleration)?.to_bits(),
                float(distance)?.to_bits(),
                float(timer)?.to_bits(),
                float(delta)?.to_bits(),
            ],
            "{line}"
        );
        cases += 1;
        slowing += usize::from(got.deceleration != 0.0);
        held += usize::from(got.timer.is_none());
    }
    assert_eq!(cases, 4000);
    assert!(
        slowing > 1500 && held > 1000,
        "{slowing} slowing, {held} held"
    );
    Ok(())
}
