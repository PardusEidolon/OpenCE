//! The engine's Slayer rules for a death against the game's: the games of
//! tools/engine_game_engine_reference.py (`game_engine_player_killed` with
//! slayer's scorer, over every mix of respawn time, respawn growth and
//! suicide penalty, with and without teams) are played again through
//! `Rules`: each player added (`game_engine_player_added`) must play on the
//! game's team for it, and after every kill the dead's wait to respawn, every player's
//! score and respawn penalty and every team's score must be the game's.
//!
//! The game's timers are in its 30 Hz ticks, the engine's in its own 60 Hz
//! ones: each game tick is two of the engine's.

use std::error::Error;

use opence_sim::clock::TICKS_PER_SECOND;
use opence_sim::game::{Rules, Variant};

const REFERENCE: &str = include_str!("game_engine_reference.txt");

/// The engine's ticks to a game tick.
const TICKS_PER_GAME_TICK: u32 = TICKS_PER_SECOND / 30;

type Failure = Box<dyn Error>;

fn number(fields: &[&str], at: usize) -> Result<i64, Failure> {
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

fn ticks(fields: &[&str], at: usize) -> Result<u32, Failure> {
    Ok(u32::try_from(number(fields, at)?)? * TICKS_PER_GAME_TICK)
}

fn player(fields: &[&str], at: usize) -> Result<Option<usize>, Failure> {
    let index = number(fields, at)?;
    if index < 0 {
        return Ok(None);
    }
    Ok(Some(usize::try_from(index)?))
}

#[test]
fn deaths_cost_the_games_waits_and_scores() -> Result<(), Failure> {
    let (mut games, mut kills) = (0, 0);
    let mut rules = Rules::new(Variant::slayer(), &[]);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.first().copied() {
            Some("game") => {
                rules = Rules::new(
                    Variant {
                        teams: number(&fields, 1)? != 0,
                        score_to_win: i32::MAX,
                        respawn_ticks: ticks(&fields, 2)?,
                        respawn_growth_ticks: ticks(&fields, 3)?,
                        suicide_penalty_ticks: ticks(&fields, 4)?,
                        ..Variant::slayer()
                    },
                    &[],
                );
                games += 1;
            }
            Some("join") => {
                let team = u16::try_from(number(&fields, 2)?)?;
                let index = rules.player_added();
                assert_eq!(index, usize::try_from(number(&fields, 1)?)?);
                assert_eq!(rules.contestants()[index].team, team, "game {games}");
            }
            Some("wait") => {
                for _ in 0..ticks(&fields, 1)? {
                    rules.step(&[]);
                }
            }
            Some("kill") => {
                for _ in 0..ticks(&fields, 1)? {
                    rules.step(&[]);
                }
                let dead = player(&fields, 2)?.ok_or("no one died")?;
                rules.player_killed(dead, player(&fields, 3)?, [0.0; 3]);
                let contestants = rules.contestants();
                assert_eq!(
                    contestants[dead].respawn_ticks,
                    ticks(&fields, 4)?,
                    "game {games}: {line}"
                );
                for (index, contestant) in contestants.iter().enumerate() {
                    let field = fields.get(5 + index).ok_or("a short line")?;
                    let (score, penalty) = field.split_once('/').ok_or("no penalty")?;
                    assert_eq!(
                        contestant.score,
                        score.parse::<i32>()?,
                        "game {games}: player {index}'s score: {line}"
                    );
                    assert_eq!(
                        contestant.respawn_penalty_ticks,
                        penalty.parse::<u32>()? * TICKS_PER_GAME_TICK,
                        "game {games}: player {index}'s penalty: {line}"
                    );
                }
                let teams = fields
                    .iter()
                    .position(|&field| field == "teams")
                    .ok_or("no teams")?;
                for (team, score) in fields[teams + 1..].iter().enumerate() {
                    assert_eq!(
                        rules.team_score(u16::try_from(team)?),
                        score.parse::<i32>()?,
                        "game {games}: team {team}'s score: {line}"
                    );
                }
                kills += 1;
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!(games, 54);
    assert!(kills >= 1500, "{kills} kills");
    Ok(())
}
