//! The engine's remaining owed maths are the game's:
//! tools/engine_math_extra_reference.py runs real_math.c's
//! `vector3d_from_euler_angles2d`, `accelerate_to_position` and
//! `quantize_real_to_byte_lower_bound`, matrix_math.c's
//! `matrix4x3_rotation_from_angles` and rasterizer_geometry.c's
//! `uncompress_int32_to_real_vector3d`, and each line is made again by the
//! engine's own and compared to the bit.

use opence_content::SkyLight;
use opence_sim::{device_animation, first_person_states, objects, vehicle_animation};

const REFERENCE: &str = include_str!("math_extra_reference.txt");

type Failure = Box<dyn std::error::Error>;

fn reals(fields: &[&str]) -> Result<Vec<f32>, Failure> {
    Ok(fields
        .iter()
        .map(|field| field.parse::<f32>())
        .collect::<Result<_, _>>()?)
}

/// Each line of a kind, split after its name.
fn lines<'a>(kind: &'a str) -> impl Iterator<Item = Vec<&'a str>> + 'a {
    REFERENCE
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(move |fields| fields.first() == Some(&kind))
        .map(|fields| fields[1..].to_vec())
}

#[test]
fn a_vector_from_a_yaw_and_a_pitch_is_the_games() -> Result<(), Failure> {
    let mut compared = 0;
    for fields in lines("euler") {
        let [yaw, pitch, i, j, k] = reals(&fields)?[..] else {
            return Err("a short line".into());
        };
        let light = SkyLight {
            lens_flare: None,
            flags: 0,
            color: [1.0; 3],
            power: 1.0,
            test_distance: 0.0,
            direction: [yaw, pitch],
            diameter: 0.0,
        };
        assert_eq!(light.towards(), [i, j, k], "yaw {yaw} pitch {pitch}");
        compared += 1;
    }
    assert_eq!(compared, 300);
    Ok(())
}

#[test]
fn a_rotation_from_three_angles_is_the_games() -> Result<(), Failure> {
    let mut compared = 0;
    for fields in lines("rotation") {
        let values = reals(&fields)?;
        let (angles, rows) = values.split_at(3);
        let matrix = objects::rotation_matrix_from_angles([angles[0], angles[1], angles[2]]);
        assert_eq!(matrix.to_cols_array()[..], rows[..], "angles {angles:?}");
        compared += 1;
    }
    assert_eq!(compared, 300);
    Ok(())
}

/// A run of ticks: its start, then each tick's position, velocity and
/// whether it arrived.
struct Run {
    start: Vec<f32>,
    periodic: bool,
    ticks: Vec<(f32, f32, bool)>,
}

fn runs() -> Result<Vec<Run>, Failure> {
    let mut runs: Vec<Run> = Vec::new();
    for line in REFERENCE.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.first() {
            Some(&"accelerate") => runs.push(Run {
                start: reals(fields.get(2..9).ok_or("a short line")?)?,
                periodic: fields.get(9) == Some(&"1"),
                ticks: Vec::new(),
            }),
            Some(&"tick") => {
                let values = reals(fields.get(1..3).ok_or("a short line")?)?;
                runs.last_mut().ok_or("a tick before a run")?.ticks.push((
                    values[0],
                    values[1],
                    fields.get(3) == Some(&"1"),
                ));
            }
            _ => {}
        }
    }
    Ok(runs)
}

#[test]
fn the_devices_move_towards_a_position_as_the_game() -> Result<(), Failure> {
    let runs = runs()?;
    assert_eq!(runs.len(), 120);
    for (index, run) in runs.iter().enumerate() {
        let [mut position, mut velocity, target, maximum_velocity, acceleration, minimum, maximum] =
            run.start[..]
        else {
            return Err("a short run".into());
        };
        for (tick, expected) in run.ticks.iter().enumerate() {
            // (the device's version names the game's fourth and fifth
            // arguments the other way, as its doc says)
            let arrived = device_animation::accelerate_to_position(
                &mut position,
                &mut velocity,
                target,
                maximum_velocity,
                acceleration,
                minimum,
                maximum,
                run.periodic,
            );
            assert_eq!(
                (position, velocity, arrived),
                *expected,
                "run {index} tick {tick}"
            );
        }
    }
    Ok(())
}

#[test]
fn the_first_person_weapons_move_towards_a_position_as_the_game() -> Result<(), Failure> {
    let mut compared = 0;
    for (index, run) in runs()?.iter().enumerate().filter(|(_, run)| !run.periodic) {
        let [mut position, mut velocity, target, maximum_velocity, acceleration, minimum, maximum] =
            run.start[..]
        else {
            return Err("a short run".into());
        };
        for (tick, expected) in run.ticks.iter().enumerate() {
            let arrived = first_person_states::accelerate_to_position(
                &mut position,
                &mut velocity,
                target,
                maximum_velocity,
                acceleration,
                minimum,
                maximum,
            );
            assert_eq!(
                (position, velocity, arrived),
                *expected,
                "run {index} tick {tick}"
            );
        }
        compared += 1;
    }
    assert_eq!(compared, 80);
    Ok(())
}

#[test]
fn a_real_quantizes_to_the_byte_below_as_the_games() -> Result<(), Failure> {
    let mut compared = 0;
    for fields in lines("quantize") {
        let values = reals(fields.get(..3).ok_or("a short line")?)?;
        let expected: u8 = fields.get(3).ok_or("a short line")?.parse()?;
        assert_eq!(
            vehicle_animation::quantize_lower_bound(values[0], values[1], values[2]),
            expected,
            "{fields:?}"
        );
        compared += 1;
    }
    assert!(compared > 900, "{compared} values");
    Ok(())
}

#[test]
fn a_packed_unit_vector_unpacks_as_the_games() -> Result<(), Failure> {
    let mut compared = 0;
    for fields in lines("unpack") {
        let packed: u32 = fields.first().ok_or("a short line")?.parse()?;
        let vector = reals(fields.get(1..4).ok_or("a short line")?)?;
        assert_eq!(
            opence_import::level::unit_vector(packed)[..],
            vector[..],
            "{packed:#x}"
        );
        compared += 1;
    }
    assert_eq!(compared, 300);
    Ok(())
}
