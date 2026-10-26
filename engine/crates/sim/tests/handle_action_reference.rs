//! The engine's choice of where a player turning an overturned vehicle back
//! over pushes it from against the game's: the 3000 flips of
//! tools/engine_handle_action_reference.py (vehicles facing anywhere, a
//! third of them on a nose or a tail or at the 45 degrees between, and the
//! player anywhere around them) must each take the game's approach.

use std::error::Error;

use glam::Vec3;
use opence_sim::vehicle::Approach;

const REFERENCE: &str = include_str!("handle_action_reference.txt");

type Failure = Box<dyn Error>;

fn vector(fields: &[&str]) -> Result<Vec3, Failure> {
    let [x, y, z] = fields else {
        return Err("a vector without three parts".into());
    };
    let real = |field: &str| u32::from_str_radix(field, 16).map(f32::from_bits);
    Ok(Vec3::new(real(x)?, real(y)?, real(z)?))
}

#[test]
fn the_engine_approaches_as_the_game_approaches() -> Result<(), Failure> {
    let mut flips = 0;
    let mut approaches = [0; 4];
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let ["flip", reals @ .., approach] = fields.as_slice() else {
            return Err(format!("an unknown line: {line}").into());
        };
        let [forward, vehicle, player] = [0, 3, 6].map(|at| reals.get(at..at + 3));
        let (Some(forward), Some(vehicle), Some(player)) = (forward, vehicle, player) else {
            return Err(format!("a flip without its vectors: {line}").into());
        };
        let expected = match *approach {
            "1" => Approach::Right,
            "2" => Approach::Left,
            "3" => Approach::NoseUp,
            "4" => Approach::NoseDown,
            other => return Err(format!("an unknown approach: {other}").into()),
        };
        let chosen = Approach::of(vector(player)?, vector(vehicle)?, vector(forward)?);
        assert_eq!(chosen, expected, "{line}");
        flips += 1;
        approaches[chosen as usize] += 1;
    }
    assert_eq!(flips, 3000);
    assert!(
        approaches.iter().all(|&count| count > 300),
        "{approaches:?}"
    );
    Ok(())
}
