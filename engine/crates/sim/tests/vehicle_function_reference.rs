//! A vehicle's function values against the game's own, over a spread of
//! vehicles and their states (tools/engine_vehicle_function_reference.py
//! writes the table from source/units/vehicles.c's
//! `vehicle_export_function_values`).

use glam::Vec3;
use opence_content::Vehicle;
use opence_sim::vehicle::{export_function_value, VehicleState};

const REFERENCE: &str = include_str!("vehicle_function_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// The vehicle and its state a line gives, its mode and the game's value
/// (none where it exports nothing).
fn case(values: &[f32]) -> Result<(Vehicle, VehicleState, i16, Option<f32>), Failure> {
    let [forward, reverse, left_turn, right_turn, left_slide, right_slide, circumference, speed, slide, turn, wheel, left_tread, right_tread, thrust, hover, airborne, vehicle_flags, object_flags, vx, vy, vz, fx, fy, fz, ux, uy, uz, mode, value] =
        values
    else {
        return Err(format!("a line of {} numbers", values.len()).into());
    };
    let vehicle = Vehicle {
        maximum_forward_speed: *forward,
        maximum_reverse_speed: *reverse,
        maximum_left_turn: *left_turn,
        maximum_right_turn: *right_turn,
        maximum_left_slide: *left_slide,
        maximum_right_slide: *right_slide,
        wheel_circumference: *circumference,
        ..Vehicle::default()
    };
    let mut state = VehicleState::default();
    let (vehicle_flags, object_flags) = (*vehicle_flags as u32, *object_flags as u32);
    state.speed = *speed;
    state.slide = *slide;
    state.turn = *turn;
    state.wheel = *wheel;
    state.treads = [*left_tread, *right_tread];
    state.thrust = *thrust;
    state.hover = *hover;
    state.airborne_ticks = *airborne as u8;
    state.crouch = vehicle_flags & 1 << 2 != 0;
    state.brake = vehicle_flags & 1 << 3 != 0;
    // (on the ground; on, partly and wholly under water)
    state.on_ground = object_flags & 1 << 1 != 0;
    state.in_water = object_flags & (1 << 2 | 1 << 3) != 0;
    state.under_water = object_flags & 1 << 4 != 0;
    state.velocity = Vec3::new(*vx, *vy, *vz);
    state.forward = Vec3::new(*fx, *fy, *fz);
    state.up = Vec3::new(*ux, *uy, *uz);
    let value = (*value != -1.0).then_some(*value);
    Ok((vehicle, state, *mode as i16, value))
}

#[test]
fn a_vehicle_exports_the_games_function_values() -> Result<(), Failure> {
    let mut compared = 0;
    let mut wrong = Vec::new();
    for line in REFERENCE.lines().filter(|line| line.starts_with("export ")) {
        let values = line
            .split_whitespace()
            .skip(1)
            .map(|field| field.parse::<f32>())
            .collect::<Result<Vec<_>, _>>()?;
        let (vehicle, state, mode, expected) = case(&values)?;
        let got = export_function_value(mode, &state, &vehicle);
        compared += 1;
        let same = match (got, expected) {
            (None, None) => true,
            // (where the game divides nothing by nothing it gives not a
            // number; the engine takes that part as 0)
            (Some(got), Some(expected)) if expected.is_nan() => (0.0..=1.0).contains(&got),
            (Some(got), Some(expected)) => (got - expected).abs() <= 1e-6,
            _ => false,
        };
        if !same {
            wrong.push(format!(
                "mode {mode}: {got:?}, the game's {expected:?} ({line})"
            ));
        }
    }
    assert!(compared >= 600, "the reference has its export lines");
    assert!(
        wrong.is_empty(),
        "{} of {compared} values differ from the game's:\n{}",
        wrong.len(),
        wrong
            .iter()
            .take(20)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(())
}
