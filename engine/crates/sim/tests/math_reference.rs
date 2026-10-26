//! The vector maths the engine ports by itself (the angle between two
//! vectors, a vector's yaw and pitch in a frame, and a frame turning a
//! normal, and the random directions) against the game's own
//! (tools/engine_math_reference.py writes the table from
//! source/math/real_math.c, matrix_math.c, geometry.c and random_math.c),
//! and real_math.h's helpers of one operation each against the glam and
//! libm primitives the engine uses in their place.

use glam::{Vec2, Vec3};
use opence_sim::animation::NodeMatrix;
use opence_sim::animation_blend::relative_angles;
use opence_sim::random::{direction_table, RandomSeed};
use opence_sim::real_math::{limit2d, limit3d};
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

/// How many arguments each of real_math.h's one-operation helpers takes in
/// the reference.
const HELPERS: &[(&str, usize)] = &[
    ("set_real_euler_angles2d", 2),
    ("arcsine", 1),
    ("tangent", 1),
    ("power", 2),
    ("valid_real", 1),
    ("interpolate_scalar", 3),
    ("dot_product2d", 4),
    ("cross_product2d", 4),
    ("magnitude_squared2d", 2),
    ("magnitude2d", 2),
    ("scale_vector2d", 3),
    ("negate_vector2d", 2),
    ("negate_vector3d", 3),
    ("vector_from_points2d", 4),
    ("vector_from_points3d", 6),
    ("distance_squared2d", 4),
    ("distance2d", 4),
    ("distance_squared3d", 6),
    ("distance3d", 6),
    ("midpoint3d", 6),
    ("point_from_line2d", 5),
    ("point_from_line3d", 7),
    ("point_in_circle", 5),
    ("point_in_sphere", 7),
    ("rotate_vector2d", 4),
    ("limit2d", 3),
    ("limit3d", 4),
    ("plane3d_distance_to_point", 7),
];

/// What the engine's primitive gives for a helper's `arguments` (as many
/// as `HELPERS` says).
fn primitive(name: &str, a: &[f32]) -> Vec<f32> {
    let flag = |b: bool| f32::from(u8::from(b));
    let v2 = |at: usize| Vec2::new(a[at], a[at + 1]);
    let v3 = |at: usize| Vec3::new(a[at], a[at + 1], a[at + 2]);
    match name {
        "set_real_euler_angles2d" => vec![a[1], a[0]],
        // (the game's are the double functions, rounded: libm's asinf is
        // a unit in the last place out for one argument in twenty)
        "arcsine" => vec![libm::asin(f64::from(a[0])) as f32],
        "tangent" => vec![libm::tan(f64::from(a[0])) as f32],
        "power" => vec![libm::pow(f64::from(a[0]), f64::from(a[1])) as f32],
        "valid_real" => vec![flag(a[0].is_finite())],
        "interpolate_scalar" => vec![a[0] + (a[1] - a[0]).clamp(-a[2], a[2])],
        "dot_product2d" => vec![v2(0).dot(v2(2))],
        "cross_product2d" => vec![v2(0).perp_dot(v2(2))],
        "magnitude_squared2d" => vec![v2(0).length_squared()],
        "magnitude2d" => vec![v2(0).length()],
        "scale_vector2d" => (v2(0) * a[2]).to_array().to_vec(),
        "negate_vector2d" => (-v2(0)).to_array().to_vec(),
        "negate_vector3d" => (-v3(0)).to_array().to_vec(),
        "vector_from_points2d" => (v2(2) - v2(0)).to_array().to_vec(),
        "vector_from_points3d" => (v3(3) - v3(0)).to_array().to_vec(),
        "distance_squared2d" => vec![v2(0).distance_squared(v2(2))],
        "distance2d" => vec![v2(0).distance(v2(2))],
        "distance_squared3d" => vec![v3(0).distance_squared(v3(3))],
        "distance3d" => vec![v3(0).distance(v3(3))],
        "midpoint3d" => v3(0).midpoint(v3(3)).to_array().to_vec(),
        "point_from_line2d" => (v2(0) + v2(2) * a[4]).to_array().to_vec(),
        "point_from_line3d" => (v3(0) + v3(3) * a[6]).to_array().to_vec(),
        "point_in_circle" => vec![flag(v2(0).distance_squared(v2(2)) <= a[4] * a[4])],
        "point_in_sphere" => vec![flag(v3(0).distance_squared(v3(3)) <= a[6] * a[6])],
        "rotate_vector2d" => Vec2::new(a[3], a[2]).rotate(v2(0)).to_array().to_vec(),
        // (ported: glam's clamp_length_max rounds differently)
        "limit2d" => {
            let (limited, limited2) = limit2d(v2(0), a[2]);
            let [i, j] = limited2.to_array();
            vec![flag(limited), i, j]
        }
        "limit3d" => {
            let (limited, limited3) = limit3d(v3(0), a[3]);
            let [i, j, k] = limited3.to_array();
            vec![flag(limited), i, j, k]
        }
        "plane3d_distance_to_point" => vec![v3(0).dot(v3(4)) - a[3]],
        _ => Vec::new(),
    }
}

#[test]
fn the_one_operation_helpers_are_the_primitives() -> Result<(), Failure> {
    for &(name, arguments) in HELPERS {
        let (mut compared, mut wrong) = (0, Vec::new());
        for values in lines(name) {
            let values = values?;
            let (given, expected) = values.split_at(arguments.min(values.len()));
            let got = primitive(name, given);
            compared += 1;
            let same = got.len() == expected.len()
                && got
                    .iter()
                    .zip(expected)
                    .all(|(g, e)| g.to_bits() == e.to_bits());
            if !same {
                wrong.push(format!("{name}{given:?}: {got:?}, the game's {expected:?}"));
            }
        }
        report(name, compared, &wrong);
    }
    Ok(())
}
