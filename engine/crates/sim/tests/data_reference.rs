//! The engine's object table takes slots, is walked and fills as the
//! game's does: tools/engine_data_reference.py runs the game's own data
//! array (`datum_new`, `datum_delete`, `data_next_index`) and object header
//! table through objects made and deleted, and the engine is run through
//! the same.

use opence_content::{Group, ObjectKind, TagRef};
use opence_sim::objects::{ObjectHandle, ObjectSpawn};
use opence_sim::{Simulation, Transform};

const REFERENCE: &str = include_str!("data_reference.txt");

type Failure = Box<dyn std::error::Error>;

fn rock() -> ObjectSpawn {
    ObjectSpawn {
        kind: ObjectKind::Scenery,
        tag: TagRef {
            group: Group(*b"scen"),
            path: "scenery\\rock".into(),
        },
        transform: Transform::default(),
        name: None,
        permutation: None,
        team: None,
    }
}

fn slot(field: &str) -> Result<u32, Failure> {
    Ok(field.parse()?)
}

#[test]
fn the_trace_is_there() {
    let steps = REFERENCE.lines().filter(|l| l.starts_with("walk")).count();
    assert!(steps > 1000, "{steps} steps");
}

#[test]
fn each_object_takes_the_slot_the_games_takes() -> Result<(), Failure> {
    let mut simulation = Simulation::empty();
    // (each slot's handle, as the engine gave it)
    let mut held: Vec<Option<ObjectHandle>> = Vec::new();
    let mut deleted: Vec<ObjectHandle> = Vec::new();
    let mut lines = 0;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["new", expected] => {
                let handle = simulation.spawn(rock())?;
                assert_eq!(handle.index, slot(expected)?, "{line}");
                let index = handle.index as usize;
                if held.len() <= index {
                    held.resize(index + 1, None);
                }
                held[index] = Some(handle);
            }
            ["delete", expected] => {
                let index = slot(expected)? as usize;
                let handle = held
                    .get_mut(index)
                    .and_then(Option::take)
                    .ok_or("the engine holds nothing there")?;
                assert!(simulation.remove(handle), "{line}");
                assert!(!simulation.remove(handle), "removed twice: {line}");
                deleted.push(handle);
            }
            ["stale", expected, taken] => {
                let old = deleted.last().copied().ok_or("no delete before")?;
                assert_eq!(old.index, slot(expected)?, "{line}");
                // (the engine's handle of a deleted object is refused)
                assert_eq!(taken, &"0", "{line}");
                assert!(!simulation.remove(old), "{line}");
            }
            ["walk", slots @ ..] => {
                let walked: Vec<u32> = simulation.objects().iter().map(|h| h.index).collect();
                let expected = slots
                    .iter()
                    .map(|s| slot(s))
                    .collect::<Result<Vec<_>, _>>()?;
                assert_eq!(walked, expected, "line {lines}");
            }
            ["refused", index] => {
                // (the engine's objects take no memory from a pool that can run out)
                assert_eq!(index, &"-1", "{line}");
            }
            ["full", made, total] => {
                let mut filled = 0;
                while simulation.spawn(rock()).is_ok() {
                    filled += 1;
                }
                assert_eq!(filled, made.parse::<usize>()?, "{line}");
                assert_eq!(
                    simulation.objects().len(),
                    total.parse::<usize>()?,
                    "{line}"
                );
            }
            _ => return Err(format!("a line the test does not know: {line}").into()),
        }
        lines += 1;
    }
    assert!(lines > 3000, "{lines} lines");
    Ok(())
}
