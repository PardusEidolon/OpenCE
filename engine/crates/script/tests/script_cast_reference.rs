//! The engine's casts are the game's: tools/engine_script_cast_reference.py
//! runs hs_runtime.c's `hs_can_cast` over every pair of types and `hs_cast`
//! (what `hs_return` hands back to the expression that called a function or
//! a script) over values of each type it lets cast, and each cast is made
//! again by the engine's `cast` on a world with the trace's stand-ins.

use std::collections::BTreeSet;

use opence_content::ValueType;
use opence_script::{cast, Value, World};

const REFERENCE: &str = include_str!("script_cast_reference.txt");

/// The trace's strings, which it passes by index.
const STRINGS: [&str; 3] = ["", "hello", "0"];

type Failure = Box<dyn std::error::Error>;

/// The trace's stand-ins: object names 0 and 1 name objects 50 and 51, a
/// list is numbered from 100, and an encounter but the first has two units.
#[derive(Default)]
struct Traced {
    lists: Vec<Vec<i32>>,
}

impl World for Traced {
    fn object_of_name(&mut self, name: i32) -> i32 {
        if (0..2).contains(&name) {
            50 + name
        } else {
            -1
        }
    }

    fn object_list_of(&mut self, objects: &[i32]) -> i32 {
        if objects.first().is_none_or(|object| *object == -1) {
            return -1;
        }
        self.lists.push(objects.to_vec());
        99 + i32::try_from(self.lists.len()).unwrap_or(i32::MAX)
    }

    fn object_list_of_ai(&mut self, ai: i32) -> i32 {
        if ai == 0 {
            return -1;
        }
        self.object_list_of(&[70 + ai, 80 + ai])
    }
}

fn field<T: std::str::FromStr>(fields: &[&str], at: usize) -> Result<T, Failure>
where
    T::Err: std::error::Error + 'static,
{
    Ok(fields.get(at).ok_or("a short line")?.parse::<T>()?)
}

fn value_type(index: i16) -> Result<ValueType, Failure> {
    Ok(ValueType::from_index(index).ok_or("no such type")?)
}

/// A value of a type, as the game holds it in a long.
fn value_of(value_type: ValueType, raw: i32) -> Result<Value, Failure> {
    // (the game's long holds a short, an index or a real's bits as they are)
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(match value_type {
        ValueType::Void => Value::Void,
        ValueType::Boolean => Value::Boolean(raw & 0xff != 0),
        ValueType::Real => Value::Real(f32::from_bits(raw as u32)),
        ValueType::Short => Value::Short(raw as i16),
        ValueType::Long => Value::Long(raw),
        ValueType::String => {
            Value::String((*STRINGS.get(usize::try_from(raw)?).ok_or("no such string")?).to_owned())
        }
        _ => Value::Index(raw),
    })
}

/// A value as the game would hold it in a long, for a type that keeps a
/// short in its low half (and nothing above it).
fn raw_of(value: &Value) -> i32 {
    // (a real's bits, a short's low half, as the game's long holds them)
    #[allow(clippy::cast_possible_wrap)]
    match value {
        Value::Void | Value::Tag(None) | Value::String(_) => 0,
        Value::Boolean(value) => i32::from(*value),
        Value::Real(value) => value.to_bits() as i32,
        Value::Short(value) => i32::from(*value as u16),
        Value::Long(value) | Value::Index(value) => *value,
        Value::Tag(Some(_)) => -1,
    }
}

/// The pairs whose casts the engine leaves as the types say, where the
/// decompilation's reading differs (docs/census.md has them, and the
/// scripts and sounds track measures them against the user's own maps):
///
/// - number and string to boolean: `hs_long_to_boolean` reads `n==0`, so
///   zero, an empty string and a real of 0 would be true; the engine takes
///   them as false, as every other reader of a script value does;
/// - `typecasting_procedures` holds four enum rows to real where there are
///   five enum types, five object rows to a list where there are six object
///   types and six object name rows, and its ai row one early: each run of
///   entries sits one more place early than the last, which is a table
///   miscounted rather than a different set of types, so the engine casts
///   the ai, each enum and each object as its own type (and an enum to a
///   real as its index, where `hs_enum_to_real` adds one).
const DIFFERS: [(i16, i16); 16] = [
    // (real, short, long and string to boolean)
    (6, 5),
    (7, 5),
    (8, 5),
    (9, 5),
    // (device group to a list, where the ai's entry is)
    (16, 23),
    // (damage effect, object definition, difficulty and team to a real)
    (30, 6),
    (31, 6),
    (32, 6),
    (33, 6),
    // (three enums to a list, and four objects to none)
    (34, 23),
    (35, 23),
    (36, 23),
    (39, 23),
    (40, 23),
    (41, 23),
    (42, 23),
];

#[test]
fn each_cast_is_the_games() -> Result<(), Failure> {
    let mut world = Traced::default();
    let (mut compared, mut differ) = (0, BTreeSet::new());
    for line in REFERENCE.lines().filter(|line| line.starts_with("cast")) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let (actual, desired) = (field::<i16>(&fields, 1)?, field::<i16>(&fields, 2)?);
        let raw: i32 = field(&fields, 3)?;
        let expected: i32 = field(&fields, 4)?;
        let (from, to) = (value_type(actual)?, value_type(desired)?);
        let lists = world.lists.len();
        let result = cast(&mut world, value_of(from, raw)?, from, to);
        let same = if fields.get(5) == Some(&"list") {
            // (a new list, whatever its number, holding these objects)
            let objects: Vec<i32> = fields[6..]
                .iter()
                .map(|f| f.parse())
                .collect::<Result<_, _>>()?;
            result != Value::Index(-1)
                && world.lists.len() == lists + 1
                && world.lists.last() == Some(&objects)
        } else if from == to {
            // (a value cast to its own type is handed back as it is)
            result == value_of(from, expected)?
        } else {
            raw_of(&result) == expected
        };
        compared += 1;
        if !same {
            differ.insert((actual, desired));
        }
    }
    assert!(compared > 500, "{compared} casts");
    let expected: BTreeSet<(i16, i16)> = DIFFERS.into_iter().collect();
    assert_eq!(differ, expected, "the pairs that differ from the game's");
    Ok(())
}

#[test]
fn each_pair_the_game_casts_is_compared() -> Result<(), Failure> {
    let pairs = |kind: &str| -> Result<BTreeSet<(i16, i16)>, Failure> {
        REFERENCE
            .lines()
            .filter(|line| line.starts_with(kind))
            .map(|line| {
                let fields: Vec<&str> = line.split_whitespace().collect();
                Ok((field(&fields, 1)?, field(&fields, 2)?))
            })
            .collect()
    };
    // (a passthrough is whatever its value already is, so none is cast)
    let castable: BTreeSet<(i16, i16)> = pairs("can")?
        .into_iter()
        .filter(|(actual, _)| *actual != 3)
        .collect();
    assert!(castable.len() > 130, "{} pairs", castable.len());
    assert_eq!(pairs("cast")?, castable);
    Ok(())
}
