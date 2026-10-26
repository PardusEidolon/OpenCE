//! The engine's default variants against the game's: each of
//! tools/engine_game_variant_reference.py's `build_game_variant_*` results
//! must be what the engine's [`Variant`] holds, its times in the game's
//! ticks turned into the engine's.

use std::error::Error;

use opence_content::GameType;
use opence_sim::clock::TICKS_PER_SECOND;
use opence_sim::game::Variant;

const REFERENCE: &str = include_str!("game_variant_reference.txt");

type Failure = Box<dyn Error>;

/// The game's ticks as the engine's.
fn engine_ticks(game_ticks: u32) -> u32 {
    game_ticks * TICKS_PER_SECOND / 30
}

/// The variant the trace names, as the engine builds it.
fn engine_variant(name: &str) -> Result<Variant, Failure> {
    match name {
        "slayer" => Ok(Variant::slayer()),
        "team_slayer" => Ok(Variant::team_slayer()),
        "ctf" => Ok(Variant::ctf()),
        _ => Err(format!("a variant I do not know: {name}").into()),
    }
}

#[test]
fn the_default_variants_are_the_games() -> Result<(), Failure> {
    let mut seen = 0;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let words: Vec<&str> = line.split_whitespace().collect();
        let [name, rest @ ..] = &words[..] else {
            return Err(format!("a line I cannot read: {line}").into());
        };
        let numbers: Vec<i32> = rest
            .iter()
            .map(|word| word.parse())
            .collect::<Result<_, _>>()?;
        let [ctf, teams, score, respawn, growth, suicide, must_reset, at_home] = numbers[..] else {
            return Err(format!("a line I cannot read: {line}").into());
        };
        let ticks =
            |value: i32| -> Result<u32, Failure> { Ok(engine_ticks(u32::try_from(value)?)) };
        let game = Variant {
            game_type: if ctf != 0 {
                GameType::CaptureTheFlag
            } else {
                GameType::Slayer
            },
            teams: teams != 0,
            score_to_win: score,
            time_limit_ticks: None,
            respawn_ticks: ticks(respawn)?,
            respawn_growth_ticks: ticks(growth)?,
            suicide_penalty_ticks: ticks(suicide)?,
            flag_must_reset: must_reset != 0,
            flag_at_home_to_score: at_home != 0,
        };
        assert_eq!(engine_variant(name)?, game, "{name}");
        seen += 1;
    }
    assert_eq!(seen, 3);
    Ok(())
}
