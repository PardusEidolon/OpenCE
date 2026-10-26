//! When the engine's netgame items stand and go against the game's: the run
//! of tools/engine_item_spawn_reference.py (`game_engine_update_purge` and
//! `game_engine_update_item_spawn` over 6000 of the game's ticks, a few
//! placements and collections, and a player now and then taking the oldest
//! item) is played again with the engine's `item_respawn_period`,
//! `random_item` and `LyingItems`, as `Game` plays them each of the game's
//! ticks, and must purge, stand and choose the same items at the same ticks.
//!
//! A placement's spawn time is its own, else its collection's, as the
//! importer gives it: a negative one repeats as its size does.

use std::collections::BTreeSet;
use std::error::Error;

use opence_sim::game::{item_respawn_period, random_item, LyingItems};
use opence_sim::random::RandomSeed;

const REFERENCE: &str = include_str!("item_spawn_reference.txt");

type Failure = Box<dyn Error>;

/// A collection: its spawn time and each permutation's weight and item.
struct Collection {
    spawn_time: i16,
    permutations: Vec<(f32, i32)>,
}

/// The seconds a placement's items stand apart, as the importer folds them.
fn seconds(placement: i16, collection: i16) -> Option<u16> {
    [placement, collection]
        .into_iter()
        .map(i16::unsigned_abs)
        .find(|&seconds| seconds > 0)
}

#[test]
fn items_stand_and_go_when_the_games_code_has_them() -> Result<(), Failure> {
    let mut collections = Vec::new();
    let mut placements = Vec::new();
    let mut expected = Vec::new();
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["collection", _, spawn_time, permutations @ ..] => {
                let permutations = permutations
                    .iter()
                    .map(|permutation| {
                        let (weight, item) = permutation.split_once(':').ok_or("no item")?;
                        Ok((weight.parse()?, item.parse()?))
                    })
                    .collect::<Result<_, Failure>>()?;
                collections.push(Collection {
                    spawn_time: spawn_time.parse()?,
                    permutations,
                });
            }
            ["placement", _, spawn_time, collection] => {
                let collection: usize = collection.parse()?;
                let spawn_time: i16 = spawn_time.parse()?;
                let of = collections.get(collection).ok_or("no collection")?;
                placements.push((seconds(spawn_time, of.spawn_time), collection));
            }
            ["purge" | "spawn" | "take", ..] => expected.push(line.to_owned()),
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }

    let mut seed = RandomSeed::default();
    let mut lying = LyingItems::<i32>::default();
    let mut taken = BTreeSet::new();
    let mut next = 0;
    let mut played = Vec::new();
    for now in 0..=6000_u32 {
        for item in lying.purge(now, |item| taken.contains(&item)) {
            played.push(format!("purge {now} {item}"));
        }
        for &(seconds, collection) in &placements {
            let period = item_respawn_period(seconds);
            if !now.is_multiple_of(period) {
                continue;
            }
            let permutations = &collections[collection].permutations;
            let weights: Vec<f32> = permutations.iter().map(|&(weight, _)| weight).collect();
            let Some(&(_, item)) =
                random_item(&mut seed, &weights).and_then(|chosen| permutations.get(chosen))
            else {
                continue;
            };
            if item < 0 {
                continue;
            }
            played.push(format!("spawn {now} {next} {item}"));
            lying.stood(next, now, period);
            next += 1;
        }
        if now % 437 == 5 {
            if let Some(oldest) = lying.items().find(|item| !taken.contains(item)) {
                taken.insert(oldest);
                played.push(format!("take {now} {oldest}"));
            }
        }
    }
    assert_eq!(played, expected);
    assert!(
        expected
            .iter()
            .filter(|line| line.starts_with("purge"))
            .count()
            > 20
    );
    Ok(())
}
