//! The engine's save and speech predicates against the game's: every one
//! of the 256 combinations of the eight dangers in
//! tools/engine_safety_reference.py must be quiet, safe to save and safe to
//! speak exactly when the game's own `game_all_quiet`, `game_safe_to_save`
//! and `game_safe_to_speak` say so, and a game cooperative exactly when
//! `game_is_cooperative` says so.

use std::error::Error;

use opence_sim::safety::{is_cooperative, Dangers};

const REFERENCE: &str = include_str!("safety_reference.txt");

type Failure = Box<dyn Error>;

/// The dangers of the trace's bits, in its order.
fn dangers(bits: u32) -> Dangers {
    let bit = |n: u32| bits >> n & 1 == 1;
    Dangers {
        enemies_see_player: bit(0),
        projectiles_near_player: bit(1),
        items_near_player: bit(2),
        effects_near_player: bit(3),
        unit_is_dangerous: bit(4),
        player_in_the_air: bit(5),
        player_dead: bit(6),
        vehicle_moving_near_player: bit(7),
    }
}

#[test]
fn the_predicates_say_what_the_games_say() -> Result<(), Failure> {
    let (mut combinations, mut counts) = (0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["dangers", bits, quiet, save, speak] => {
                let dangers = dangers(bits.parse()?);
                let said = [
                    dangers.all_quiet(),
                    dangers.safe_to_save(),
                    dangers.safe_to_speak(),
                ];
                assert_eq!(said, [*quiet == "1", *save == "1", *speak == "1"], "{line}");
                combinations += 1;
            }
            ["players", players, cooperative] => {
                assert_eq!(
                    is_cooperative(players.parse()?),
                    *cooperative == "1",
                    "{line}"
                );
                counts += 1;
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!((combinations, counts), (256, 5));
    Ok(())
}
