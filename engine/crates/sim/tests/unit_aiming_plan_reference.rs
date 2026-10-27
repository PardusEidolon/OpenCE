//! The unit's euler axis planner against the game's own: every plan built,
//! followed and coupled in tests/unit_aiming_plan_reference.txt, which
//! tools/engine_unit_aiming_plan_reference.py records from source/units/units.c,
//! must be the same, float for float.

use opence_sim::unit_aiming::{couple, AccelerationPlan};

const REFERENCE: &str = include_str!("unit_aiming_plan_reference.txt");

/// The floats of a line, from their bits.
fn floats(words: &[&str]) -> Vec<f32> {
    words
        .iter()
        .map(|word| f32::from_bits(word.parse().unwrap_or(u32::MAX)))
        .collect()
}

/// A plan from its eight words.
fn plan(words: &[&str]) -> AccelerationPlan {
    let values = floats(&words[1..8]);
    AccelerationPlan {
        at_rest: words[0] == "1",
        initial_position: values[0],
        initial_velocity: values[1],
        acceleration: values[2],
        acceleration_time: values[3],
        coast_time: values[4],
        deceleration: values[5],
        deceleration_time: values[6],
    }
}

/// A plan's eight words, as the trace prints them.
fn printed(plan: &AccelerationPlan) -> String {
    format!(
        "{} {} {} {} {} {} {} {}",
        u8::from(plan.at_rest),
        plan.initial_position.to_bits(),
        plan.initial_velocity.to_bits(),
        plan.acceleration.to_bits(),
        plan.acceleration_time.to_bits(),
        plan.coast_time.to_bits(),
        plan.deceleration.to_bits(),
        plan.deceleration_time.to_bits(),
    )
}

/// What the engine answers to a line's question.
fn answer(asked: &str) -> String {
    let words: Vec<&str> = asked.split_whitespace().collect();
    match words[0] {
        "build" => {
            let values = floats(&words[1..5]);
            printed(&AccelerationPlan::build(
                values[0], values[1], values[2], values[3],
            ))
        }
        "do" => {
            let values = floats(&words[9..12]);
            let (complete, position, velocity) =
                plan(&words[1..9]).follow(values[0], values[1], values[2]);
            format!(
                "{} {} {}",
                u8::from(complete),
                position.to_bits(),
                velocity.to_bits()
            )
        }
        "couple" => {
            let limits = floats(&words[1..3]);
            let mut first = plan(&words[3..11]);
            let mut second = plan(&words[11..19]);
            couple(&mut first, &mut second, limits[1]);
            format!("{} {}", printed(&first), printed(&second))
        }
        other => format!("unknown case {other}"),
    }
}

#[test]
fn the_aiming_planner_is_the_games() {
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
fn a_plan_from_rest_is_complete_at_once() {
    let plan = AccelerationPlan::build(0.0005, -0.0005, 1.0, 1.0);
    assert!(plan.at_rest);
    assert_eq!(plan.follow(1.0, 0.0005, -0.0005), (true, 0.0005, -0.0005));
}
