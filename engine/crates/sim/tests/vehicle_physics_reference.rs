//! The vehicle controllers hand the physics update what the game's do:
//! tools/engine_vehicle_physics_reference.py runs vehicles.c's jeep, tank,
//! boat, scout and fighter controllers, its count of the ticks off the
//! ground and its slowing to a stop over random vehicles (with the wrong
//! number of powered mass points now and then, and facings straight up and
//! down), and the engine's are run on the same and compared: the vehicle
//! after, the powered mass points' push and turn, the force and torque,
//! and whether the vehicle lay flipped before (`vehicle_is_flipped`).

use glam::{Vec2, Vec3};
use opence_content::{FrictionType, Group, MassPoint, Physics, PoweredMassPoint, TagRef, Vehicle};
use opence_sim::vehicle::{
    boat_setup, compute_airborne_ticks, fighter_setup, jeep_setup, scout_hover, scout_setup,
    slowly_stop, tank_setup, ControllerSetup, VehicleControls, VehicleState,
};

const REFERENCE: &str = include_str!("vehicle_physics_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// The fields of a line, read in order.
struct Fields<'a> {
    fields: std::slice::Iter<'a, &'a str>,
}

impl Fields<'_> {
    fn text(&mut self) -> Result<&str, Failure> {
        Ok(self.fields.next().ok_or("a short line")?)
    }

    fn real(&mut self) -> Result<f32, Failure> {
        Ok(self.text()?.parse()?)
    }

    fn whole(&mut self) -> Result<i64, Failure> {
        Ok(self.text()?.parse()?)
    }

    fn vector(&mut self) -> Result<Vec3, Failure> {
        Ok(Vec3::new(self.real()?, self.real()?, self.real()?))
    }
}

/// A physics of `powered` powered mass points and the given mass points'
/// powered indices, its sizes as the trace's.
fn physics(radius: f32, mass: f32, moments: Vec3, powered: usize, points: &[i64]) -> Physics {
    let powered_point = PoweredMassPoint {
        name: String::new(),
        flags: 0,
        antigrav_strength: 0.0,
        antigrav_offset: 0.0,
        antigrav_height: 0.0,
        antigrav_damp_fraction: 0.0,
        antigrav_normal_k1: 0.0,
        antigrav_normal_k0: 0.0,
    };
    Physics {
        tag: TagRef {
            group: Group(*b"phys"),
            path: String::new(),
        },
        radius,
        moment_scale: 1.0,
        mass,
        center_of_mass: [0.0; 3],
        density: 1.0,
        gravity_scale: 1.0,
        ground_friction: 0.2,
        ground_depth: 0.15,
        ground_damp_fraction: 0.05,
        ground_normal_k1: 0.7,
        ground_normal_k0: 0.5,
        water_friction: 0.05,
        water_depth: 0.25,
        water_density: 1.0,
        air_friction: 0.001,
        xx_moment: moments.x,
        yy_moment: moments.y,
        zz_moment: moments.z,
        inertial_matrix: None,
        powered_mass_points: vec![powered_point; powered],
        mass_points: points
            .iter()
            .map(|&index| MassPoint {
                name: String::new(),
                powered_mass_point: u16::try_from(index).ok(),
                model_node: None,
                flags: 0,
                relative_mass: 1.0,
                mass: 1.0,
                relative_density: 1.0,
                density: 1.0,
                position: [0.0; 3],
                forward: [1.0, 0.0, 0.0],
                up: [0.0, 0.0, 1.0],
                friction_type: FrictionType::Point,
                friction_parallel_scale: 1.0,
                friction_perpendicular_scale: 1.0,
                radius: 0.1,
            })
            .collect(),
    }
}

/// How far the engine's may be from the game's: a millionth of the larger
/// of the two (or of one), where the worst measured is about a quarter of
/// that, from the C's maths library's sines and square roots.
fn near(engine: f32, game: f32) -> bool {
    if engine.is_nan() || game.is_nan() {
        return engine.is_nan() && game.is_nan();
    }
    let scale = engine.abs().max(game.abs()).max(1.0);
    (engine - game).abs() <= scale * 1.0e-6
}

#[test]
fn the_vehicle_controllers_are_the_games() -> Result<(), Failure> {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    let mut worst = 0.0f32;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (case, result) = line.split_once(" -> ").ok_or("a line with no result")?;
        let case: Vec<&str> = case.split_whitespace().collect();
        let result: Vec<&str> = result.split_whitespace().collect();
        let mut fields = Fields {
            fields: case.iter(),
        };
        let kind = fields.text()?.to_owned();
        let mut state = VehicleState {
            position: fields.vector()?,
            forward: fields.vector()?,
            up: fields.vector()?,
            velocity: fields.vector()?,
            angular_velocity: fields.vector()?,
            speed: fields.real()?,
            turn: fields.real()?,
            wheel: fields.real()?,
            treads: [fields.real()?, fields.real()?],
            hover: fields.real()?,
            thrust: fields.real()?,
            airborne_ticks: u8::try_from(fields.whole()?)?,
            on_ground_ticks: u8::try_from(fields.whole()?)?,
            stop_time: u8::try_from(fields.whole()?)?,
            brake: fields.whole()? != 0,
            ..VehicleState::default()
        };
        let throttle = Vec2::new(fields.real()?, fields.real()?);
        let facing = fields.vector()?;
        let seat_power = fields.real()?;
        let controls = VehicleControls {
            throttle,
            facing,
            seat_power: [seat_power, 0.0],
            ai: fields.whole()? != 0,
            ..VehicleControls::default()
        };
        let water_depth = fields.real()?;
        let steering = fields.real()?;
        let vehicle = Vehicle {
            maximum_forward_speed: fields.real()?,
            maximum_reverse_speed: fields.real()?,
            speed_acceleration: fields.real()?,
            speed_deceleration: fields.real()?,
            maximum_left_turn: fields.real()?,
            turn_rate: fields.real()?,
            fixed_gun_pitch: fields.real()?,
            wheel_circumference: fields.real()?,
            ..Vehicle::default()
        };
        let radius = fields.real()?;
        let mass = fields.real()?;
        let moments = fields.vector()?;
        let powered = usize::try_from(fields.whole()?)?;
        let count = usize::try_from(fields.whole()?)?;
        let mut indices = Vec::with_capacity(count);
        for _ in 0..count {
            let (index, flags) = fields.text()?.split_once(':').ok_or("a mass point")?;
            indices.push(index.parse::<i64>()?);
            state.points.push(flags.parse()?);
        }
        let physics = physics(radius, mass, moments, powered, &indices);
        let flipped = if state.is_flipped() { 1.0 } else { 0.0 };

        let setup = match kind.as_str() {
            "jeep" => Some(jeep_setup(&mut state, &vehicle, &physics)),
            "tank" => Some(tank_setup(&mut state, &vehicle, &physics)),
            "boat" => Some(boat_setup(&state, &physics)),
            "scout" => {
                let setup =
                    scout_setup(&state, &vehicle, &physics, &controls, steering, water_depth);
                // (the game's physics update hands back what it was given)
                scout_hover(&mut state, &physics);
                Some(setup)
            }
            "fighter" => Some(fighter_setup(&mut state, &vehicle, &physics, &controls)),
            "airborne" => {
                compute_airborne_ticks(&mut state);
                None
            }
            "stop" => {
                slowly_stop(&mut state);
                None
            }
            other => return Err(format!("an unknown kind {other}").into()),
        };
        let setup = setup.unwrap_or(ControllerSetup {
            powered: None,
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        });

        let mut engine = vec![];
        for v in [
            state.position,
            state.forward,
            state.up,
            state.velocity,
            state.angular_velocity,
        ] {
            engine.extend(v.to_array());
        }
        engine.extend([
            state.wheel,
            state.treads[0],
            state.treads[1],
            state.hover,
            state.thrust,
            f32::from(state.airborne_ticks),
            f32::from(state.on_ground_ticks),
            f32::from(state.stop_time),
            if setup.powered.is_some() { 1.0 } else { 0.0 },
        ]);
        for index in 0..3 {
            match &setup.powered {
                Some(points) => {
                    let point = &points[index];
                    engine.extend([
                        point.ground_friction_velocity,
                        point.water_friction_velocity,
                        point.water_lift,
                        point.antigrav_fraction,
                    ]);
                    engine.extend(point.rotation.to_array());
                }
                None => engine.extend([0.0; 8]),
            }
        }
        engine.extend(setup.force.to_array());
        engine.extend(setup.torque.to_array());
        engine.push(flipped);

        if engine.len() != result.len() {
            return Err(format!("{line}: {} values, not {}", engine.len(), result.len()).into());
        }
        for (at, (engine, game)) in engine.iter().zip(&result).enumerate() {
            let game: f32 = game.parse()?;
            if !near(*engine, game) {
                return Err(format!(
                    "{line}\nvalue {at}: the engine's {engine}, the game's {game}"
                )
                .into());
            }
            if engine.is_finite() && game.is_finite() {
                worst = worst.max((engine - game).abs() / engine.abs().max(game.abs()).max(1.0));
            }
        }
        *counts.entry(kind).or_default() += 1;
    }
    println!("the worst relative error: {worst:e}");
    assert_eq!(
        counts.into_iter().collect::<Vec<_>>(),
        [
            ("airborne".to_owned(), 200),
            ("boat".to_owned(), 400),
            ("fighter".to_owned(), 400),
            ("jeep".to_owned(), 400),
            ("scout".to_owned(), 400),
            ("stop".to_owned(), 200),
            ("tank".to_owned(), 400),
        ]
    );
    Ok(())
}
