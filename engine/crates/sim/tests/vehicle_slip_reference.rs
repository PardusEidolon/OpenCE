//! A vehicle's mass points slip as the game's do:
//! tools/engine_vehicle_slip_reference.py runs vehicles.c's
//! `create_slipping_effects` over random mass points (on the ground and off
//! it, sliding faster and slower than a slip and not at all, metallic and
//! not), and the engine's `slip` is run on the same and compared: whether
//! it slips, and the effect's kind, material, position, normal and scale.

use glam::Vec3;
use opence_sim::vehicle::slip;

const REFERENCE: &str = include_str!("vehicle_slip_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// The game's material effect kinds: a tyre's slip and a tread's.
const SLIP: i64 = 9;
const METALLIC_SLIP: i64 = 10;

/// How far the engine's may be from the game's: a millionth of the larger
/// of the two (or of one); the maths is the same, operation for operation.
fn near(engine: f32, game: f32) -> bool {
    let scale = engine.abs().max(game.abs()).max(1.0);
    (engine - game).abs() <= scale * 1.0e-6
}

fn reals(fields: &[&str]) -> Result<Vec<f32>, Failure> {
    Ok(fields
        .iter()
        .map(|field| field.parse())
        .collect::<Result<_, _>>()?)
}

#[test]
fn the_vehicle_slips_are_the_games() -> Result<(), Failure> {
    let (mut slips, mut none) = (0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (case, result) = line.split_once(" -> ").ok_or("a line with no result")?;
        let case: Vec<&str> = case.split_whitespace().collect();
        let [_, on_ground, rest @ ..] = case.as_slice() else {
            return Err(format!("{line}: a short case").into());
        };
        let [px, py, pz, vi, vj, vk, ni, nj, nk, material, depth, radius, flags] = rest else {
            return Err(format!("{line}: a case of {} fields", rest.len()).into());
        };
        let [px, py, pz, vi, vj, vk, ni, nj, nk, depth, radius] =
            reals(&[px, py, pz, vi, vj, vk, ni, nj, nk, depth, radius])?[..]
        else {
            return Err(format!("{line}: a case's reals").into());
        };
        let material: i16 = material.parse()?;
        let metallic = flags.parse::<u32>()? & 1 != 0;

        // (the engine asks only of the mass points on the ground)
        let engine = if *on_ground == "1" {
            slip(
                Vec3::new(px, py, pz),
                Vec3::new(ni, nj, nk),
                depth,
                radius,
                Vec3::new(vi, vj, vk),
                material,
                metallic,
            )
        } else {
            None
        };

        let result: Vec<&str> = result.split_whitespace().collect();
        match (engine, result.as_slice()) {
            (None, ["none"]) => none += 1,
            (Some(engine), [kind, material, values @ ..]) => {
                let kind: i64 = kind.parse()?;
                let wanted = if engine.metallic { METALLIC_SLIP } else { SLIP };
                if kind != wanted || material.parse::<i16>()? != engine.material {
                    return Err(format!("{line}\nthe engine's slip is {engine:?}").into());
                }
                let game = reals(values)?;
                let mut ours = vec![];
                ours.extend(engine.position.to_array());
                ours.extend(engine.normal.to_array());
                ours.push(engine.scale);
                if ours.len() != game.len() {
                    return Err(format!("{line}: {} values, not {}", ours.len(), game.len()).into());
                }
                for (at, (ours, game)) in ours.iter().zip(&game).enumerate() {
                    if !near(*ours, *game) {
                        return Err(format!(
                            "{line}\nvalue {at}: the engine's {ours}, the game's {game}"
                        )
                        .into());
                    }
                }
                slips += 1;
            }
            (engine, _) => {
                return Err(format!("{line}\nthe engine's slip is {engine:?}").into());
            }
        }
    }
    assert_eq!((slips, none), (1582, 418));
    Ok(())
}
