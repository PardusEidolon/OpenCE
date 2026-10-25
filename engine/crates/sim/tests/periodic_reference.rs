//! The periodic and transition functions and the seeded range against the
//! game's own, over their inputs (tools/engine_periodic_reference.py writes
//! the table from source/math/periodic_functions.c and random_math.c).

use opence_sim::damage::transition;
use opence_sim::periodic::evaluate;
use opence_sim::random::RandomSeed;

const REFERENCE: &str = include_str!("periodic_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// The numbers after a line's first word.
fn numbers(line: &str) -> Result<Vec<f32>, Failure> {
    line.split_whitespace()
        .skip(1)
        .map(|field| Ok(field.parse::<f32>()?))
        .collect()
}

/// The lines whose first word is `kind`.
fn lines(kind: &str) -> impl Iterator<Item = &'static str> + '_ {
    REFERENCE
        .lines()
        .filter(move |line| line.split_whitespace().next() == Some(kind))
}

/// Each of a function's values against the game's, from function -1 on;
/// the mismatches, as lines to print.
fn compare(kind: &str, engine: impl Fn(i16, f32) -> f32) -> Result<(usize, Vec<String>), Failure> {
    let mut compared = 0;
    let mut wrong = Vec::new();
    for line in lines(kind) {
        let values = numbers(line)?;
        let (input, expected) = values.split_first().ok_or("an empty line")?;
        for (function, &expected) in (-1i16..).zip(expected) {
            let got = engine(function, *input);
            compared += 1;
            if (got - expected).abs() > 1e-6 {
                wrong.push(format!(
                    "{kind} {function} at {input}: {got}, the game's {expected}"
                ));
            }
        }
    }
    Ok((compared, wrong))
}

fn report(kind: &str, compared: usize, wrong: &[String]) {
    assert!(compared > 1000, "the reference has the {kind} lines");
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
fn the_periodic_functions_are_the_games() -> Result<(), Failure> {
    let (compared, wrong) = compare("periodic", evaluate)?;
    report("periodic", compared, &wrong);
    Ok(())
}

#[test]
fn the_transition_functions_are_the_games() -> Result<(), Failure> {
    let (compared, wrong) = compare("transition", transition)?;
    report("transition", compared, &wrong);
    Ok(())
}

#[test]
fn the_seeded_range_is_the_games() -> Result<(), Failure> {
    let mut seed = RandomSeed(0xDEAD_BEEF);
    let mut draws = 0;
    for line in lines("range") {
        let fields: Vec<i64> = line
            .split_whitespace()
            .skip(1)
            .map(|field| field.parse::<i64>())
            .collect::<Result<_, _>>()?;
        let [lower, upper, value, after] = fields[..] else {
            return Err(format!("a range of {} numbers", fields.len()).into());
        };
        let got = seed.next_range(i16::try_from(lower)?, i16::try_from(upper)?);
        assert_eq!(
            i64::from(got),
            value,
            "the range {lower}..{upper}, draw {draws}"
        );
        assert_eq!(i64::from(seed.0), after, "the seed after draw {draws}");
        draws += 1;
    }
    assert_eq!(draws, 300);
    Ok(())
}
