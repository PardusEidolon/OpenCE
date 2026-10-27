//! The engine's small physics sums against the game's: the cases of
//! tools/engine_physics_math_reference.py (a pin's fraction, a mass
//! point's friction by type and a point's mass) must give the game's
//! answers to the bit.

use std::error::Error;

use glam::Vec3;
use opence_content::{FrictionType, PointPhysics};
use opence_sim::point_physics::definition_mass;
use opence_sim::vehicle::{friction_evaluate, pin_fraction};

const REFERENCE: &str = include_str!("physics_math_reference.txt");

type Failure = Box<dyn Error>;

fn real(field: &str) -> Result<f32, Failure> {
    Ok(f32::from_bits(u32::from_str_radix(field, 16)?))
}

fn vector(fields: &[&str]) -> Result<Vec3, Failure> {
    let [x, y, z] = fields else {
        return Err("a vector of three".into());
    };
    Ok(Vec3::new(real(x)?, real(y)?, real(z)?))
}

fn bits(v: Vec3) -> [u32; 3] {
    [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]
}

#[test]
fn the_engine_sums_physics_as_the_game_sums_it() -> Result<(), Failure> {
    let (mut pins, mut frictions, mut masses) = (0, [0usize; 4], 0);
    for (number, line) in REFERENCE.lines().enumerate() {
        if line.starts_with('#') {
            continue;
        }
        let at = number + 1;
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["pin", value, begin, end, fraction] => {
                let got = pin_fraction(real(value)?, real(begin)?, real(end)?);
                assert_eq!(
                    got.to_bits(),
                    real(fraction)?.to_bits(),
                    "line {at}: {line}"
                );
                pins += 1;
            }
            ["friction", kind, parallel, perpendicular, rest @ ..] if rest.len() == 12 => {
                let index: usize = kind.parse()?;
                let kind = *FrictionType::ALL.get(index).ok_or("a friction type")?;
                let got = friction_evaluate(
                    vector(&rest[0..3])?,
                    kind,
                    real(parallel)?,
                    real(perpendicular)?,
                    vector(&rest[3..6])?,
                    vector(&rest[6..9])?,
                );
                assert_eq!(bits(got), bits(vector(&rest[9..12])?), "line {at}: {line}");
                if let Some(count) = frictions.get_mut(index) {
                    *count += 1;
                }
            }
            ["mass", over, radius, mass] => {
                let physics = PointPhysics {
                    mass_over_radius_cubed: real(over)?,
                    ..PointPhysics::default()
                };
                let got = definition_mass(&physics, real(radius)?);
                assert_eq!(got.to_bits(), real(mass)?.to_bits(), "line {at}: {line}");
                masses += 1;
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!(
        (pins, frictions.iter().sum::<usize>(), masses),
        (3000, 3000, 2000)
    );
    assert!(frictions.iter().all(|&count| count > 500), "{frictions:?}");
    Ok(())
}
