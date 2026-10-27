//! The unit's aiming and looking vectors against the game's own: every
//! question, check, alignment and clip in
//! tests/unit_aiming_bounds_reference.txt, which
//! tools/engine_unit_aiming_bounds_reference.py records from
//! source/units/units.c, must be the same, float for float.

use opence_sim::unit_aiming::{clip_to_aiming_bounds, is_playing_custom_animation, UnitAim};
use opence_sim::unit_control::{aim_without_turning, UnitFlags};

const REFERENCE: &str = include_str!("unit_aiming_bounds_reference.txt");

/// The vector of three words, from their bits.
fn vector(words: &[&str]) -> [f32; 3] {
    std::array::from_fn(|axis| f32::from_bits(words[axis].parse().unwrap_or(u32::MAX)))
}

/// A vector's three words, as the trace prints them.
fn printed(vector: [f32; 3]) -> String {
    format!(
        "{} {} {}",
        vector[0].to_bits(),
        vector[1].to_bits(),
        vector[2].to_bits()
    )
}

/// What the engine answers to a line's question.
fn answer(asked: &str) -> String {
    let words: Vec<&str> = asked.split_whitespace().collect();
    let bit = |index: usize| words[index] == "1";
    match words[0] {
        "valid" => {
            let aim = UnitAim {
                desired_facing: vector(&words[1..]),
                desired_aiming: vector(&words[4..]),
                desired_looking: vector(&words[7..]),
                forward: vector(&words[10..]),
                up: vector(&words[13..]),
                aiming: vector(&words[16..]),
                looking: vector(&words[19..]),
            };
            u8::from(aim.vectors_are_valid()).to_string()
        }
        "clip" => {
            let (forward, up) = (vector(&words[3..]), vector(&words[6..]));
            let bounds: [f32; 4] = std::array::from_fn(|edge| {
                f32::from_bits(words[9 + edge].parse().unwrap_or(u32::MAX))
            });
            let screen = bit(2).then_some(bounds);
            let asked = vector(&words[13..]);
            match clip_to_aiming_bounds(forward, up, screen, asked) {
                Some(clipped) => format!("1 {}", printed(clipped)),
                None => format!("0 {}", printed(asked)),
            }
        }
        "get" => {
            let aim = UnitAim {
                forward: vector(&words[1..]),
                looking: vector(&words[4..]),
                aiming: vector(&words[7..]),
                ..UnitAim::default()
            };
            format!(
                "{} {} {}",
                printed(aim.facing()),
                printed(aim.looking()),
                printed(aim.aiming())
            )
        }
        "align" => {
            let alignment = vector(&words[2..]);
            let mut aim = UnitAim {
                forward: vector(&words[4..]),
                up: vector(&words[7..]),
                ..UnitAim::default()
            };
            aim.align_facing(bit(1), [alignment[0], alignment[1]]);
            format!("{} {}", printed(aim.forward), printed(aim.up))
        }
        "aim" => {
            let mut flags = UnitFlags(words[2].parse().unwrap_or(0));
            aim_without_turning(bit(1).then_some(&mut flags), bit(3));
            flags.0.to_string()
        }
        "custom" => {
            let state = words[2].parse().unwrap_or(-1);
            u8::from(is_playing_custom_animation(bit(1).then_some(state))).to_string()
        }
        other => format!("unknown case {other}"),
    }
}

#[test]
fn the_unit_aiming_vectors_are_the_games() {
    let mut cases = 0;
    let mut wrong = Vec::new();
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let Some((asked, expected)) = line.rsplit_once(" -> ") else {
            continue;
        };
        cases += 1;
        let got = answer(asked);
        if got != expected && wrong.len() < 10 {
            wrong.push(format!("{asked}\n  game:   {expected}\n  engine: {got}"));
        }
    }
    assert!(cases > 50_000, "only {cases} cases");
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn a_vector_inside_the_screen_is_not_clipped() {
    let ahead = [1.0, 0.0, 0.0];
    let up = [0.0, 0.0, 1.0];
    assert_eq!(
        clip_to_aiming_bounds(ahead, up, Some([-1.0, 1.0, -1.0, 1.0]), ahead),
        None
    );
    assert_eq!(
        clip_to_aiming_bounds(ahead, up, None, [0.0, 1.0, 0.0]),
        None
    );
}
