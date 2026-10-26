//! Where the engine respawns a multiplayer player against the game: the
//! 4000 games of tools/engine_spawn_reference.py (Capture the Flag, Slayer,
//! Oddball and King of the Hill, with and without teams, a few players with
//! units or none, starts of several game types and either team, and
//! vehicles and other objects about) are rated again with the engine's
//! `starting_location_rating`, and a start chosen with
//! `best_starting_location` from the same random seed: each start's
//! rating, the start chosen and the seed after must be the game's.

use std::error::Error;

use opence_content::{GameType, GameTypes};
use opence_sim::game::{
    best_starting_location, starting_location_rating, vehicle_near, Neighbour, StartView, Variant,
};
use opence_sim::random::RandomSeed;

const REFERENCE: &str = include_str!("spawn_reference.txt");

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

fn point(fields: &[&str]) -> Result<[f32; 3], Failure> {
    let [x, y, z] = fields else {
        return Err("not a point".into());
    };
    Ok([x.parse()?, y.parse()?, z.parse()?])
}

/// A game as the reference sets it up.
struct Case {
    variant: Variant,
    team: u16,
    seed: u32,
    players: Vec<Neighbour>,
    vehicles: Vec<([f32; 3], f32)>,
    starts: Vec<(u16, GameTypes, [f32; 3], f32)>,
}

impl Case {
    /// Rates the starts and chooses one as the engine does: the ratings and
    /// the start chosen, with the seed after.
    fn play(&self) -> (Vec<f32>, Option<usize>, u32) {
        let ratings: Vec<f32> = self
            .starts
            .iter()
            .map(|&(team, game_types, position, _)| {
                let view = StartView {
                    position,
                    team,
                    for_game_type: game_types.matches(self.variant.game_type),
                    vehicle_near: vehicle_near(&self.vehicles, position),
                };
                starting_location_rating(&self.variant, self.team, &view, &self.players)
            })
            .collect();
        let mut seed = RandomSeed(self.seed);
        let chosen = best_starting_location(&mut seed, ratings.iter().copied());
        (ratings, chosen, seed.0)
    }
}

#[test]
fn respawns_choose_the_start_the_games_code_chooses() -> Result<(), Failure> {
    let mut case: Option<Case> = None;
    let (mut games, mut chosen_some, mut rated) = (0, 0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["game", kind, teams, team, _, seed] => {
                case = Some(Case {
                    variant: Variant {
                        game_type: game_type(kind)?,
                        teams: *teams == "1",
                        ..Variant::slayer()
                    },
                    team: team.parse()?,
                    seed: seed.parse()?,
                    players: Vec::new(),
                    vehicles: Vec::new(),
                    starts: Vec::new(),
                });
            }
            ["player", team, position @ ..] => {
                let game = case.as_mut().ok_or("a player before its game")?;
                game.players.push(Neighbour {
                    team: team.parse()?,
                    position: (position != ["-"]).then(|| point(position)).transpose()?,
                });
            }
            ["object", vehicle, centre @ .., radius] => {
                let game = case.as_mut().ok_or("an object before its game")?;
                if *vehicle == "1" {
                    game.vehicles.push((point(centre)?, radius.parse()?));
                }
            }
            ["start", team, a, b, c, d, x, y, z, rating] => {
                let game = case.as_mut().ok_or("a start before its game")?;
                let game_types = GameTypes([a.parse()?, b.parse()?, c.parse()?, d.parse()?]);
                game.starts.push((
                    team.parse()?,
                    game_types,
                    point(&[x, y, z])?,
                    rating.parse()?,
                ));
            }
            ["chosen", chosen, seed] => {
                let game = case.take().ok_or("a choice before its game")?;
                let (ratings, engine_chosen, engine_seed) = game.play();
                let expected: Vec<f32> = game.starts.iter().map(|start| start.3).collect();
                let bits =
                    |ratings: &[f32]| ratings.iter().map(|r| r.to_bits()).collect::<Vec<_>>();
                assert_eq!(bits(&ratings), bits(&expected), "game {games}");
                let chosen: i32 = chosen.parse()?;
                assert_eq!(
                    engine_chosen,
                    usize::try_from(chosen).ok(),
                    "game {games}: {ratings:?}"
                );
                assert_eq!(engine_seed, seed.parse::<u32>()?, "game {games}");
                games += 1;
                chosen_some += usize::from(chosen >= 0);
                rated += expected.iter().filter(|&&rating| rating > 0.0).count();
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!(games, 4000);
    assert!(chosen_some > 2500, "{chosen_some} chose a start");
    assert!(rated > 5000, "{rated} starts rated");
    Ok(())
}
