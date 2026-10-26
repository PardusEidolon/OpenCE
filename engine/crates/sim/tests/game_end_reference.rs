//! When the engine's rules end a game against the game's: the cases of
//! tools/engine_game_end_reference.py (`game_engine_should_end_game` over
//! every game of up to five players, with and without teams, each on
//! either team, alive or dead, with and without a minute's time limit)
//! are set up again through `Rules`, played for as long, and the engine
//! must end each game exactly when the game's code does.
//!
//! The game's timers are in its 30 Hz ticks, the engine's in its own 60 Hz
//! ones: each game tick is two of the engine's.

use std::error::Error;

use opence_sim::clock::TICKS_PER_SECOND;
use opence_sim::game::{Rules, Variant};

const REFERENCE: &str = include_str!("game_end_reference.txt");

/// The engine's ticks to a game tick.
const TICKS_PER_GAME_TICK: u32 = TICKS_PER_SECOND / 30;

type Failure = Box<dyn Error>;

fn number(fields: &[&str], at: usize) -> Result<u32, Failure> {
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

#[test]
fn games_end_when_the_games_code_ends_them() -> Result<(), Failure> {
    let (mut games, mut ended) = (0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.first() != Some(&"game") {
            return Err(format!("an unknown line: {line}").into());
        }
        let minutes = number(&fields, 2)?;
        let mut rules = Rules::new(
            Variant {
                teams: number(&fields, 1)? != 0,
                time_limit_ticks: (minutes > 0).then_some(minutes * 60 * TICKS_PER_SECOND),
                ..Variant::slayer()
            },
            &[],
        );
        let end = fields
            .iter()
            .position(|&field| field == "end")
            .ok_or("no end")?;
        for player in &fields[4..end] {
            let (team, state) = player.split_at(player.len() - 1);
            let index = rules.join(team.parse()?);
            if state == "d" {
                rules.player_killed(index, None, [0.0; 3]);
            }
        }
        for _ in 0..number(&fields, 3)? * TICKS_PER_GAME_TICK {
            rules.step(&[]);
        }
        let expected = number(&fields, end + 1)? != 0;
        assert_eq!(rules.should_end(), expected, "{line}");
        if number(&fields, 3)? > 0 {
            assert_eq!(rules.is_over(), expected, "{line}");
        }
        games += 1;
        ended += usize::from(expected);
    }
    assert_eq!(games, 5712);
    assert!(ended > 1000, "{ended} ended");
    Ok(())
}
