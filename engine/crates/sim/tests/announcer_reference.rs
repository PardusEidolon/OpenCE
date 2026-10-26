//! The engine's multiplayer announcer against the game: the 480 games of
//! tools/engine_announcer_reference.py (Slayer, Capture the Flag, King of
//! the Hill and Oddball, with and without teams, with tables of sounds of
//! random lengths, some missing, or none) are played again with the
//! engine's `Announcer` and `starting_sound`: each sound must play on the
//! game's tick it played on in the game, and no other.

use std::error::Error;

use opence_content::GameType;
use opence_sim::announcer::{starting_sound, Announcer, MultiplayerSound, MultiplayerSounds};
use opence_sim::game::Variant;

const REFERENCE: &str = include_str!("announcer_reference.txt");

type Failure = Box<dyn Error>;

/// A game type by Halo's number for it.
fn game_type(number: &str) -> Result<GameType, Failure> {
    Ok(match number {
        "1" => GameType::CaptureTheFlag,
        "2" => GameType::Slayer,
        "3" => GameType::Oddball,
        "4" => GameType::King,
        _ => return Err(format!("an unknown game type: {number}").into()),
    })
}

/// A game as the reference plays it.
#[derive(Default)]
struct Case {
    variant: Option<Variant>,
    information: bool,
    sounds: Vec<Option<u32>>,
    /// The sounds asked for, by game tick, in order.
    calls: Vec<(u32, u8)>,
    /// The sounds played, by game tick, in order.
    plays: Vec<(u32, u8)>,
}

impl Case {
    /// Plays the game's calls through the engine's announcer, stepping it
    /// once a game tick before that tick's calls: the sounds played.
    fn play(&self, ticks: u32) -> Result<Vec<(u32, u8)>, Failure> {
        let variant = self.variant.as_ref().ok_or("no game")?;
        let sounds = MultiplayerSounds(if self.information {
            self.sounds.clone()
        } else {
            Vec::new()
        });
        let mut announcer = Announcer::new();
        let mut played = Vec::new();
        if let Some(sound) = starting_sound(variant) {
            if let Some(now) = announcer.play(sound, &sounds) {
                played.push((0, now.0));
            }
        }
        let mut calls = self.calls.iter().peekable();
        for tick in 1..=ticks {
            if let Some(now) = announcer.update(&sounds) {
                played.push((tick, now.0));
            }
            while let Some(&&(at, sound)) = calls.peek() {
                if at != tick {
                    break;
                }
                calls.next();
                if let Some(now) = announcer.play(MultiplayerSound(sound), &sounds) {
                    played.push((tick, now.0));
                }
            }
        }
        Ok(played)
    }
}

#[test]
fn the_announcer_plays_what_the_games_code_plays_when_it_plays_it() -> Result<(), Failure> {
    let mut case = Case::default();
    let (mut games, mut plays, mut started) = (0, 0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["game", kind, teams, information] => {
                case = Case {
                    variant: Some(Variant {
                        game_type: game_type(kind)?,
                        teams: *teams == "1",
                        ..Variant::slayer()
                    }),
                    information: *information == "1",
                    ..Case::default()
                };
            }
            ["sound", index, length] => {
                let length: u32 = length.parse()?;
                case.sounds.push((*index != "-1").then_some(length));
            }
            ["call", tick, sound] => case.calls.push((tick.parse()?, sound.parse()?)),
            ["play", tick, sound] => case.plays.push((tick.parse()?, sound.parse()?)),
            ["end", ticks] => {
                let played = case.play(ticks.parse()?)?;
                assert_eq!(played, case.plays, "game {games}");
                games += 1;
                plays += played.len();
                let starting = case.variant.as_ref().and_then(starting_sound);
                started += usize::from(
                    played
                        .iter()
                        .any(|&(_, s)| Some(s) == starting.map(|s| s.0)),
                );
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!(games, 480);
    assert!(plays > 5000, "{plays} sounds played");
    assert!(started > 150, "{started} games announced their type");
    Ok(())
}

#[test]
fn a_game_announces_its_type_when_its_first_two_seconds_are_up() {
    let sounds = MultiplayerSounds(vec![Some(1000); 43]);
    let mut announcer = Announcer::new();
    let slayer = starting_sound(&Variant::slayer());
    assert_eq!(slayer, Some(MultiplayerSound::SLAYER));
    assert_eq!(slayer.and_then(|s| announcer.play(s, &sounds)), None);
    let played: Vec<u32> = (1..=200)
        .filter(|_| announcer.update(&sounds).is_some())
        .collect();
    assert_eq!(played, [60]);
}
