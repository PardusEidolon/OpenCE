//! The engine's Capture the Flag rules against the game's: every game of
//! tools/engine_ctf_reference.py (players walking a grid of quarter metres
//! for the flags and the stands, dying and coming back) is played again
//! through [`Rules`], two engine steps to each of the game's ticks, and
//! after every tick the flags, the team scores, each player's captures and
//! whether the game is over must be what the game's code left.

use std::collections::VecDeque;
use std::error::Error;

use opence_sim::clock::TICKS_PER_SECOND;
use opence_sim::game::{FlagState, Rules, Variant};

const REFERENCE: &str = include_str!("ctf_reference.txt");

/// The engine's steps to one of the game's ticks.
const STEPS_PER_GAME_TICK: u32 = TICKS_PER_SECOND / 30;

type Failure = Box<dyn Error>;

/// A flag as the trace writes it.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Seen {
    Home,
    Carried(usize),
    Dropped(f32, f32),
}

/// What the game's code left after a tick.
#[derive(Clone, Debug, PartialEq)]
struct State {
    flags: [Seen; 2],
    teams: [i32; 2],
    captures: Vec<i32>,
    over: bool,
}

/// One game: its options, its players and what happened in it.
#[derive(Default)]
struct Played {
    header: Vec<i32>,
    joins: Vec<(u16, [i32; 2])>,
    goes: VecDeque<(usize, [i32; 2])>,
    kills: Vec<(u32, usize)>,
    spawns: Vec<(u32, usize, [i32; 2])>,
    states: Vec<(u32, State)>,
    end: u32,
}

fn integers(words: &[&str]) -> Result<Vec<i32>, Failure> {
    Ok(words
        .iter()
        .map(|word| word.parse())
        .collect::<Result<_, _>>()?)
}

fn index(value: i32) -> Result<usize, Failure> {
    Ok(usize::try_from(value)?)
}

fn tick(value: i32) -> Result<u32, Failure> {
    Ok(u32::try_from(value)?)
}

fn state(words: &[&str], players: usize) -> Result<(u32, State), Failure> {
    let at = tick(words[0].parse()?)?;
    let mut rest = &words[1..];
    let mut flags = [Seen::Home; 2];
    for flag in &mut flags {
        match rest {
            ["home", more @ ..] => {
                rest = more;
            }
            ["carried", p, more @ ..] => {
                *flag = Seen::Carried(p.parse()?);
                rest = more;
            }
            ["dropped", x, y, more @ ..] => {
                *flag = Seen::Dropped(x.parse()?, y.parse()?);
                rest = more;
            }
            _ => return Err(format!("a flag I cannot read: {words:?}").into()),
        }
    }
    let numbers = integers(rest)?;
    if numbers.len() != 3 + players {
        return Err(format!("a state of {} numbers: {words:?}", numbers.len()).into());
    }
    Ok((
        at,
        State {
            flags,
            teams: [numbers[0], numbers[1]],
            captures: numbers[2..2 + players].to_vec(),
            over: numbers[2 + players] != 0,
        },
    ))
}

fn games() -> Result<Vec<Played>, Failure> {
    let mut games: Vec<Played> = Vec::new();
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.first() == Some(&"game") {
            games.push(Played {
                header: integers(&words[1..])?,
                ..Played::default()
            });
            continue;
        }
        let Some(game) = games.last_mut() else {
            return Err(format!("a line before a game: {line}").into());
        };
        let n = |at: usize| -> Result<i32, Failure> { Ok(words.get(at).ok_or("short")?.parse()?) };
        match words.first().copied() {
            Some("join") => game.joins.push((u16::try_from(n(1)?)?, [n(2)?, n(3)?])),
            Some("go") => game.goes.push_back((index(n(1)?)?, [n(2)?, n(3)?])),
            Some("kill") => game.kills.push((tick(n(1)?)?, index(n(2)?)?)),
            Some("spawn") => game
                .spawns
                .push((tick(n(1)?)?, index(n(2)?)?, [n(3)?, n(4)?])),
            Some("state") => {
                let players = game.joins.len();
                game.states.push(state(&words[1..], players)?);
            }
            Some("end") => game.end = tick(n(1)?)?,
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    Ok(games)
}

/// A place on the grid, in metres.
fn metres(at: [i32; 2]) -> [f32; 3] {
    // (quarter metres: exact in an f32)
    let quarter = |value: i32| value as f32 * 0.25;
    [quarter(at[0]), quarter(at[1]), 0.0]
}

fn engine_state(rules: &Rules) -> State {
    let seen = |team: usize| match rules.flags()[team].state {
        FlagState::Home => Seen::Home,
        FlagState::Carried(player) => Seen::Carried(player),
        FlagState::Dropped { at, .. } => Seen::Dropped(at[0], at[1]),
    };
    State {
        flags: [seen(0), seen(1)],
        teams: [rules.team_score(0), rules.team_score(1)],
        captures: rules.contestants().iter().map(|c| c.score).collect(),
        over: rules.is_over(),
    }
}

fn play(number: usize, game: &mut Played) -> Result<usize, Failure> {
    let [_, must_reset, at_home, score_to_win, rx, ry, bx, by] = game.header[..] else {
        return Err(format!("game {number}: a header I cannot read").into());
    };
    let variant = Variant {
        score_to_win,
        flag_must_reset: must_reset != 0,
        flag_at_home_to_score: at_home != 0,
        // (the fake world's players come back when it says)
        respawn_ticks: 0,
        suicide_penalty_ticks: 0,
        ..Variant::ctf()
    };
    let mut rules = Rules::new(variant, &[(0, metres([rx, ry])), (1, metres([bx, by]))]);
    let mut at = Vec::new();
    let mut goal = Vec::new();
    let mut alive = Vec::new();
    for &(team, place) in &game.joins {
        rules.join(team);
        at.push(place);
        let (player, target) = game.goes.pop_front().ok_or("no goal")?;
        if player != at.len() - 1 {
            return Err(format!("game {number}: a goal for {player} at the join").into());
        }
        goal.push(target);
        alive.push(true);
    }
    let mut expected = game.states.iter().peekable();
    let mut last = None;
    let mut checked = 0;
    for t in 1..=game.end {
        for &(_, player) in game.kills.iter().filter(|kill| kill.0 == t) {
            rules.player_killed(player, None, metres(at[player]));
            alive[player] = false;
        }
        for &(_, player, place) in game.spawns.iter().filter(|spawn| spawn.0 == t) {
            at[player] = place;
            alive[player] = true;
            let (who, target) = game.goes.pop_front().ok_or("no goal")?;
            if who != player {
                return Err(format!("game {number}: a goal for {who} at {player}'s spawn").into());
            }
            goal[player] = target;
        }
        for _ in 1..STEPS_PER_GAME_TICK {
            rules.step(&[]);
        }
        // (the players walk before the game's pickups, after its update)
        for player in 0..at.len() {
            if !alive[player] {
                continue;
            }
            if at[player] == goal[player] {
                let (who, target) = game.goes.pop_front().ok_or("no goal")?;
                if who != player {
                    return Err(format!("game {number}: a goal for {who} at {player}'s").into());
                }
                goal[player] = target;
            }
            for axis in 0..2 {
                at[player][axis] += (goal[player][axis] - at[player][axis]).signum();
            }
        }
        let positions: Vec<Option<[f32; 3]>> = (0..at.len())
            .map(|player| alive[player].then(|| metres(at[player])))
            .collect();
        rules.step(&positions);
        while let Some((_, state)) = expected.next_if(|(when, _)| *when <= t) {
            last = Some(state);
        }
        let engine = engine_state(&rules);
        if Some(&engine) != last {
            return Err(format!(
                "game {number}, tick {t}: the engine has {engine:?}, the game {last:?}"
            )
            .into());
        }
        checked += 1;
    }
    if !game.goes.is_empty() {
        return Err(format!("game {number}: {} goals left over", game.goes.len()).into());
    }
    Ok(checked)
}

#[test]
fn capture_the_flag_plays_as_the_games_does() -> Result<(), Failure> {
    let mut games = games()?;
    let mut ticks = 0;
    let mut captures = 0;
    let mut over = 0;
    for (number, game) in games.iter_mut().enumerate() {
        ticks += play(number, game)?;
        if let Some((_, state)) = game.states.last() {
            captures += state.teams.iter().sum::<i32>();
            over += usize::from(state.over);
        }
    }
    assert_eq!(games.len(), 60);
    assert!(ticks > 90_000, "{ticks} ticks");
    assert!(captures >= 100, "{captures} captures");
    assert!(over >= 40, "{over} games won");
    Ok(())
}
