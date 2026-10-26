//! The wind is the game's: tools/engine_wind_reference.py runs wind.c's
//! `wind_variance_initialize`, `wind_update`, `wind_variance_get` and
//! `scenario_get_current_from_weather_palette` over a palette of three
//! entries for 200 ticks, handing them a recorded run of random directions
//! and coins, and the engine's winds are handed the same draws and
//! compared with it to the bit.

use std::collections::VecDeque;

use opence_content::{Group, Level, StructureBsp, TagRef, WeatherPalette, Wind};
use opence_sim::Vec3;

use super::{State, Winds};

const REFERENCE: &str = include_str!("../../../tests/wind_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// The trace's points, which the air is asked for at.
const POINTS: [[f32; 3]; 5] = [
    [0.0, 0.0, 0.0],
    [0.3, 0.6, 0.2],
    [-12.5, 40.25, -3.0],
    [1000.0, -2500.5, 77.7],
    [1_048_576.0, -3_000_000.0, 0.0625],
];

fn tag(path: &str) -> TagRef {
    TagRef {
        group: Group(*b"wind"),
        path: path.into(),
    }
}

/// The trace's map: two winds (their variation areas yaw first, as the
/// tag holds them) and a palette of three entries, the middle without.
fn map() -> (Level, StructureBsp) {
    let mut level = opence_content::devmap::level(None);
    level.winds = vec![
        Wind {
            tag: tag("gusty"),
            velocity: [1.0, 4.0],
            variation_area: [0.6, 0.2],
            local_variation_weight: 0.4,
            local_variation_rate: 1.5,
            damping: 0.25,
        },
        Wind {
            tag: tag("steady"),
            velocity: [0.5, 0.75],
            variation_area: [0.1, 0.9],
            local_variation_weight: 0.0,
            local_variation_rate: 0.5,
            damping: 0.6,
        },
    ];
    let mut bsp = level.structure_bsps[0].clone();
    bsp.weather_palette = vec![
        WeatherPalette {
            system: None,
            wind_tag: Some(tag("gusty")),
            wind: [1.0, 2.0, 0.5, 3.0],
        },
        WeatherPalette {
            system: None,
            wind_tag: None,
            wind: [0.0, 1.0, 0.0, 9.0],
        },
        WeatherPalette {
            system: None,
            wind_tag: Some(tag("steady")),
            wind: [-0.3, 0.1, -0.8, 1.25],
        },
    ];
    (level, bsp)
}

fn reals(fields: &[&str]) -> Result<Vec<f32>, Failure> {
    Ok(fields
        .iter()
        .map(|field| field.parse::<f32>())
        .collect::<Result<_, _>>()?)
}

fn vector(fields: &[&str]) -> Result<Vec3, Failure> {
    let values = reals(fields)?;
    match values[..] {
        [x, y, z] => Ok(Vec3::new(x, y, z)),
        _ => Err("not a vector".into()),
    }
}

/// Each line of a kind, split after its name.
fn lines(kind: &str) -> impl Iterator<Item = Vec<&str>> + '_ {
    REFERENCE
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(move |fields| fields.first() == Some(&kind))
        .map(|fields| fields[1..].to_vec())
}

/// The noise the trace's directions make.
fn winds() -> Result<Winds, Failure> {
    let mut directions = lines("direction")
        .map(|fields| vector(&fields))
        .collect::<Result<VecDeque<_>, _>>()?;
    assert_eq!(directions.len(), 24);
    Ok(Winds::from_directions(|| {
        directions.pop_front().unwrap_or(Vec3::ZERO)
    }))
}

#[test]
fn the_noise_is_the_games() -> Result<(), Failure> {
    let winds = winds()?;
    let mut compared = 0;
    for fields in lines("variance") {
        let axis: usize = fields.first().ok_or("a short line")?.parse()?;
        let sample: usize = fields.get(1).ok_or("a short line")?.parse()?;
        let expected = vector(fields.get(2..5).ok_or("a short line")?)?;
        assert_eq!(
            winds.variance[axis][sample], expected,
            "axis {axis} sample {sample}"
        );
        compared += 1;
    }
    assert_eq!(compared, 192);
    Ok(())
}

#[test]
fn the_winds_wander_and_blow_as_the_games() -> Result<(), Failure> {
    let (level, bsp) = map();
    let mut winds = winds()?;
    let mut coins = lines("coin")
        .map(|fields| Ok(fields.first() == Some(&"1")))
        .collect::<Result<VecDeque<bool>, Failure>>()?;
    let states: Vec<Vec<&str>> = lines("state").collect();
    let currents: Vec<Vec<&str>> = lines("current").collect();
    let (mut compared_states, mut compared_currents) = (0, 0);
    for tick in 1..=200 {
        winds.tick_with(&level, &bsp, || coins.pop_front().unwrap_or(false));
        let tick_field = tick.to_string();
        for fields in states
            .iter()
            .filter(|fields| fields.first() == Some(&tick_field.as_str()))
        {
            let entry: usize = fields.get(1).ok_or("a short line")?.parse()?;
            let values = reals(fields.get(3..10).ok_or("a short line")?)?;
            let expected = State {
                valid: fields.get(2) == Some(&"1"),
                velocity_variance: values[0],
                angular_variance: [values[1], values[2]],
                velocity: values[3],
                velocity3d: Vec3::new(values[4], values[5], values[6]),
                ..State::default()
            };
            let state = winds.states.get(entry).ok_or("no such state")?;
            let actual = State {
                weight: 0.0,
                rate: 0.0,
                damping: 0.0,
                ..*state
            };
            assert_eq!(actual, expected, "tick {tick} entry {entry}");
            compared_states += 1;
        }
        for fields in currents
            .iter()
            .filter(|fields| fields.first() == Some(&tick_field.as_str()))
        {
            let entry: i32 = fields.get(1).ok_or("a short line")?.parse()?;
            let point: usize = fields.get(2).ok_or("a short line")?.parse()?;
            let flags: u32 = fields.get(3).ok_or("a short line")?.parse()?;
            let expected = vector(fields.get(4..7).ok_or("a short line")?)?;
            let actual = winds.current(
                usize::try_from(entry).ok(),
                Vec3::from_array(POINTS[point]),
                flags & 1 != 0,
                flags & 2 != 0,
            );
            assert_eq!(
                actual, expected,
                "tick {tick} entry {entry} point {point} flags {flags}"
            );
            compared_currents += 1;
        }
    }
    assert!(coins.is_empty(), "{} coins left", coins.len());
    assert_eq!(compared_states, 600);
    assert_eq!(compared_currents, 20 * 5 * 5 * 4);
    Ok(())
}
