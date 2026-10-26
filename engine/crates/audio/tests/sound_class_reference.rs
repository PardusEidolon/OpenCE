//! The engine's sound classes are the game's: tools/engine_sound_class_reference.py
//! runs sound_classes.c's table, `sound_class_get` (an index out of range
//! pinned to the table's ends) and the fades `sound_class_set_gain` sets and
//! `sound_classes_update` steps, and the engine's classes and gains are
//! checked line by line against it.

use opence_audio::{class, ClassGains, SOUND_CLASS_COUNT};

const REFERENCE: &str = include_str!("sound_class_reference.txt");

type Failure = Box<dyn std::error::Error>;

fn field<T: std::str::FromStr>(fields: &[&str], at: usize) -> Result<T, Failure>
where
    T::Err: std::error::Error + 'static,
{
    Ok(fields.get(at).ok_or("a short line")?.parse::<T>()?)
}

#[test]
fn each_class_is_the_games() -> Result<(), Failure> {
    let mut compared = 0;
    for line in REFERENCE.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let used = match fields.first() {
            Some(&"class") => true,
            Some(&"unused") => false,
            _ => continue,
        };
        let asked: i32 = field(&fields, 1)?;
        let given: usize = field(&fields, 2)?;
        let engine = class(asked);
        assert!(
            std::ptr::eq(engine, class(i32::try_from(given)?)),
            "{line}: the engine pins it elsewhere"
        );
        assert_eq!(
            u32::from(engine.maximum_per_definition),
            field::<u32>(&fields, 3)?,
            "{line}"
        );
        assert_eq!(
            u32::from(engine.maximum_per_object),
            field::<u32>(&fields, 4)?,
            "{line}"
        );
        assert_eq!(
            u32::from(engine.preemption_ms),
            field::<u32>(&fields, 5)?,
            "{line}"
        );
        assert_eq!(u8::from(engine.speech), field::<u8>(&fields, 6)?, "{line}");
        assert_eq!(
            u32::from(engine.priority),
            field::<u32>(&fields, 7)?,
            "{line}"
        );
        // (cache miss mode 1: a sound waits for its samples)
        assert_eq!(
            u8::from(engine.waits_for_samples),
            field::<u8>(&fields, 8)?,
            "{line}"
        );
        assert_eq!(engine.wet_gain, field::<f32>(&fields, 9)?, "{line}");
        assert_eq!(
            engine.minimum_distance,
            field::<f32>(&fields, 10)?,
            "{line}"
        );
        assert_eq!(
            engine.maximum_distance,
            field::<f32>(&fields, 11)?,
            "{line}"
        );
        assert_eq!(engine.unknown_gain, field::<f32>(&fields, 12)?, "{line}");
        assert_eq!(engine.unknown_scale, field::<f32>(&fields, 13)?, "{line}");
        // (no class is disabled but by the debug commands)
        assert_eq!(fields.get(14), Some(&"0"), "{line}");
        let name = if used {
            *fields.get(15).ok_or("no name")?
        } else {
            ""
        };
        assert_eq!(engine.name, name, "{line}");
        compared += 1;
    }
    // (each of the 51, and three either side)
    assert_eq!(compared, SOUND_CLASS_COUNT + 6);
    Ok(())
}

#[test]
fn the_fades_step_as_the_games() -> Result<(), Failure> {
    let mut gains = ClassGains::default();
    let mut compared = 0;
    for line in REFERENCE.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.first() {
            Some(&"set") => {
                let name = *fields.get(1).ok_or("no name")?;
                gains.set(name, field(&fields, 2)?, field(&fields, 3)?);
            }
            Some(&"update") => gains.update(field(&fields, 1)?),
            Some(&"gains") => {
                for (index, expected) in fields[1..].iter().enumerate() {
                    let expected: f32 = expected.parse()?;
                    let index = i32::try_from(index)?;
                    assert_eq!(
                        gains.gain(index),
                        expected,
                        "class {index} after line {compared}"
                    );
                }
                assert_eq!(fields.len() - 1, SOUND_CLASS_COUNT);
                compared += 1;
            }
            _ => {}
        }
    }
    assert!(compared > 400, "{compared} lines of gains");
    Ok(())
}
