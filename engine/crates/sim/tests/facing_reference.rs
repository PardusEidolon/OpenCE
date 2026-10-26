//! The engine's facing against the game's: the 32 runs of
//! tools/engine_facing_reference.py (mouse-sized turns, flicks and turns of
//! several whole turns, from a random yaw) are turned again through
//! `facing::turn`, and each look must be the game's, to the bit.

use std::error::Error;

use opence_sim::facing;

const REFERENCE: &str = include_str!("facing_reference.txt");

type Failure = Box<dyn Error>;

/// An f32 written as its bits in hex.
fn float(bits: &str) -> Result<f32, Failure> {
    Ok(f32::from_bits(u32::from_str_radix(bits, 16)?))
}

#[test]
fn a_look_turns_as_the_games_look_turns() -> Result<(), Failure> {
    let (mut look, mut runs, mut turns, mut stopped) = ((0.0, 0.0), 0, 0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["run", yaw] => {
                look = (float(yaw)?, 0.0);
                runs += 1;
            }
            ["turn", yaw, pitch, after_yaw, after_pitch] => {
                look = facing::turn(look.0, look.1, float(yaw)?, float(pitch)?);
                assert_eq!(
                    (look.0.to_bits(), look.1.to_bits()),
                    (float(after_yaw)?.to_bits(), float(after_pitch)?.to_bits()),
                    "run {runs}: {line}"
                );
                turns += 1;
                stopped += usize::from(look.1.abs() == facing::MAX_PITCH);
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!((runs, turns), (32, 32 * 200));
    assert!(stopped > 500, "{stopped} looks at the limit");
    Ok(())
}
