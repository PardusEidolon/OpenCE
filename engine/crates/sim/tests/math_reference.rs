//! The vector maths the engine ports by itself (the angle between two
//! vectors, a vector's yaw and pitch in a frame, and a frame turning a
//! normal, and the random directions) against the game's own
//! (tools/engine_math_reference.py writes the table from
//! source/math/real_math.c, matrix_math.c, geometry.c and random_math.c).

use glam::Vec3;
use opence_sim::animation::NodeMatrix;
use opence_sim::animation_blend::relative_angles;
use opence_sim::random::{direction_table, RandomSeed};
use opence_sim::vehicle::angle_between;

const REFERENCE: &str = include_str!("math_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// The numbers after the first word of the lines whose first word is
/// `kind`.
fn lines(kind: &str) -> impl Iterator<Item = Result<Vec<f32>, Failure>> + '_ {
    REFERENCE
        .lines()
        .filter(move |line| line.split_whitespace().next() == Some(kind))
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|field| Ok(field.parse::<f32>()?))
                .collect()
        })
}

fn report(kind: &str, compared: usize, wrong: &[String]) {
    assert!(compared >= 400, "the reference has the {kind} lines");
    assert!(
        wrong.is_empty(),
        "{} of {compared} {kind} values differ from the game's:\n{}",
        wrong.len(),
        wrong
            .iter()
            .take(20)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn the_angles_between_vectors_are_the_games() -> Result<(), Failure> {
    let (mut compared, mut wrong) = (0, Vec::new());
    for values in lines("angle") {
        let values = values?;
        let [ai, aj, ak, bi, bj, bk, expected] = values[..] else {
            return Err(format!("an angle line of {} numbers", values.len()).into());
        };
        let (a, b) = (Vec3::new(ai, aj, ak), Vec3::new(bi, bj, bk));
        let got = angle_between(a, b);
        compared += 1;
        if got != expected {
            wrong.push(format!("between {a} and {b}: {got}, the game's {expected}"));
        }
    }
    report("angle", compared, &wrong);
    Ok(())
}

#[test]
fn the_frames_are_the_games() -> Result<(), Failure> {
    let (mut compared, mut wrong) = (0, Vec::new());
    for values in lines("frame") {
        let values = values?;
        let [fi, fj, fk, ui, uj, uk, vi, vj, vk, yaw, pitch, ti, tj, tk] = values[..] else {
            return Err(format!("a frame line of {} numbers", values.len()).into());
        };
        let (forward, up, vector) = ([fi, fj, fk], [ui, uj, uk], [vi, vj, vk]);
        let angles = relative_angles(forward, up, vector);
        let turned =
            NodeMatrix::from_point_and_vectors([0.0; 3], forward, up).transform_vector(vector);
        compared += 1;
        if angles != (yaw, pitch) {
            wrong.push(format!(
                "{vector:?} in {forward:?} {up:?}: {angles:?}, the game's ({yaw}, {pitch})"
            ));
        }
        if turned != [ti, tj, tk] {
            wrong.push(format!("{vector:?} turned by {forward:?} {up:?}: {turned:?}, the game's [{ti}, {tj}, {tk}]"));
        }
    }
    report("frame", compared, &wrong);
    Ok(())
}

#[test]
fn the_direction_table_is_the_games() -> Result<(), Failure> {
    let table = direction_table();
    let (mut compared, mut wrong) = (0, Vec::new());
    for (index, values) in lines("direction").enumerate() {
        let values = values?;
        let expected: [f32; 3] = values[..].try_into()?;
        let got = table.get(index).copied();
        compared += 1;
        if got != Some(expected) {
            wrong.push(format!(
                "direction {index}: {got:?}, the game's {expected:?}"
            ));
        }
    }
    assert_eq!(compared, table.len(), "the table's size is the game's");
    report("direction", compared, &wrong);
    Ok(())
}

#[test]
fn the_vectors_in_cones_are_the_games() -> Result<(), Failure> {
    let (mut compared, mut wrong) = (0, Vec::new());
    for line in REFERENCE.lines().filter(|line| line.starts_with("cone ")) {
        let fields: Vec<&str> = line.split_whitespace().skip(1).collect();
        let [before, ai, aj, ak, inner, outer, ri, rj, rk, after] = fields[..] else {
            return Err(format!("a cone line of {} fields", fields.len()).into());
        };
        let number = |field: &str| field.parse::<f32>();
        let axis = Vec3::new(number(ai)?, number(aj)?, number(ak)?);
        let expected = [number(ri)?, number(rj)?, number(rk)?];
        let mut seed = RandomSeed(before.parse()?);
        let got = seed
            .vector_in_cone(axis, number(inner)?, number(outer)?)
            .to_array();
        compared += 1;
        if got != expected || seed.0 != after.parse::<u32>()? {
            wrong.push(format!(
                "{axis} in {inner}..{outer} from {before}: {got:?} and {}, the game's {expected:?} and {after}",
                seed.0
            ));
        }
    }
    report("cone", compared, &wrong);
    Ok(())
}
