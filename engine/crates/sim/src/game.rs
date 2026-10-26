//! Game types (`game_engine.c`, `game_engine_ctf.c`, `game_engine_slayer.c`):
//! the rules of Slayer and Capture the Flag, the scores, the players'
//! respawn timers, the flags and the scenario's netgame equipment.
//!
//! [`Rules`] are the rules alone, stepped with where each player stands;
//! [`Game`] plays them over a [`Simulation`], reading its kills, standing
//! its players again at the level's starts and placing its flags and
//! equipment.

use opence_content::{GameType, Level, NetgameFlagKind, ObjectKind, TagRef};

use crate::clock::TICKS_PER_SECOND;
use crate::objects::{ObjectHandle, ObjectSpawn};
use crate::random::RandomSeed;
use crate::{Quat, Simulation, Transform, Vec3};

/// Halo's ticks a second, which its game options count in.
const GAME_TICKS_PER_SECOND: u32 = 30;

/// The shortest wait to respawn: 3 seconds (`game_engine.c`).
pub const MINIMUM_RESPAWN_TICKS: u32 = 3 * TICKS_PER_SECOND;

/// How near a player comes to a flag, or a carrier to its own stand, to
/// touch it.
pub const FLAG_TOUCH_RADIUS: f32 = 1.0;

/// How long a dropped flag lies untouched before it goes home: 17 seconds
/// of the game's ticks (`CTF_FLAG_RETURN_TIME`); it goes home on the tick
/// after.
pub const FLAG_RETURN_TICKS: u32 = 17 * GAME_TICKS_PER_SECOND;

/// The engine's steps to one of the game's ticks, on which the flags' rules
/// run.
const STEPS_PER_GAME_TICK: u32 = TICKS_PER_SECOND / GAME_TICKS_PER_SECOND;

/// The seconds between a placement's items when neither it nor its item
/// collection says.
pub const DEFAULT_EQUIPMENT_SECONDS: u16 = 30;

/// Halo's game ticks as the engine's.
fn engine_ticks(game_ticks: u32) -> u32 {
    game_ticks * TICKS_PER_SECOND / GAME_TICKS_PER_SECOND
}

/// A game variant: the game type and its options.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Variant {
    /// The game type.
    pub game_type: GameType,
    /// Whether players play in teams.
    pub teams: bool,
    /// The score that wins.
    pub score_to_win: i32,
    /// How long the game lasts, if it has a limit.
    pub time_limit_ticks: Option<u32>,
    /// How long a dead player waits to respawn (at least
    /// [`MINIMUM_RESPAWN_TICKS`]).
    pub respawn_ticks: u32,
    /// How much longer each death makes a player's next wait, up to five
    /// times as long, and each kill of an enemy makes it shorter again.
    pub respawn_growth_ticks: u32,
    /// How much longer after a suicide or a betrayal.
    pub suicide_penalty_ticks: u32,
    /// Whether a team's own flag must be carried home rather than touched.
    pub flag_must_reset: bool,
    /// Whether a team's own flag must be home for it to score.
    pub flag_at_home_to_score: bool,
}

impl Variant {
    /// Slayer, every player for themself, as Halo's default.
    /// (`build_game_variant_slayer`)
    pub fn slayer() -> Self {
        Self {
            game_type: GameType::Slayer,
            teams: false,
            score_to_win: 15,
            time_limit_ticks: None,
            respawn_ticks: 0,
            respawn_growth_ticks: 0,
            suicide_penalty_ticks: engine_ticks(300),
            flag_must_reset: false,
            flag_at_home_to_score: false,
        }
    }

    /// Team Slayer, as Halo's default (`build_game_variant_team_slayer`):
    /// to 50, with ten seconds to wait before a respawn.
    pub fn team_slayer() -> Self {
        Self {
            teams: true,
            score_to_win: 50,
            respawn_ticks: engine_ticks(300),
            ..Self::slayer()
        }
    }

    /// Capture the Flag, as Halo's default (`build_game_variant_ctf`).
    pub fn ctf() -> Self {
        Self {
            game_type: GameType::CaptureTheFlag,
            teams: true,
            score_to_win: 3,
            time_limit_ticks: None,
            respawn_ticks: engine_ticks(300),
            respawn_growth_ticks: 0,
            suicide_penalty_ticks: engine_ticks(150),
            flag_must_reset: false,
            flag_at_home_to_score: false,
        }
    }
}

/// A player in the game: its team, its score and whether it waits to
/// respawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Contestant {
    /// Its team (in a game without teams, its own).
    pub team: u16,
    /// Its score.
    pub score: i32,
    /// The players it killed.
    pub kills: u32,
    /// The times it died.
    pub deaths: u32,
    /// The times it killed itself.
    pub suicides: u32,
    /// The times it killed a teammate.
    pub betrayals: u32,
    /// Whether it is alive.
    pub alive: bool,
    /// How long it still waits to respawn.
    pub respawn_ticks: u32,
    /// How much longer than the variant's its next wait is, for the
    /// respawn growth.
    pub respawn_penalty_ticks: u32,
}

/// Where a flag is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlagState {
    /// On its stand.
    Home,
    /// Carried by a player.
    Carried(usize),
    /// Lying where its carrier fell.
    Dropped {
        /// Where.
        at: [f32; 3],
        /// How many of the game's ticks it has lain there untouched.
        untouched_ticks: u32,
    },
}

/// A team's flag.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Flag {
    /// Its team.
    pub team: u16,
    /// Its stand.
    pub stand: [f32; 3],
    /// Where it is.
    pub state: FlagState,
}

impl Flag {
    /// Where it is now, unless a player carries it.
    pub fn position(&self) -> Option<[f32; 3]> {
        match self.state {
            FlagState::Home => Some(self.stand),
            FlagState::Carried(_) => None,
            FlagState::Dropped { at, .. } => Some(at),
        }
    }
}

/// Something that happened in the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEvent {
    /// A player died, killed by a player (perhaps itself) or by nothing.
    Killed {
        /// The player who died.
        victim: usize,
        /// The player who killed it.
        killer: Option<usize>,
    },
    /// A player's wait is over: it stands at a start again.
    Respawn(usize),
    /// A player took an enemy's flag.
    FlagTaken {
        /// The flag.
        flag: usize,
        /// The player.
        by: usize,
    },
    /// A flag went home.
    FlagReturned(usize),
    /// A flag's carrier dropped it.
    FlagDropped(usize),
    /// A player carried an enemy's flag to its own stand.
    Captured {
        /// The flag.
        flag: usize,
        /// The player.
        by: usize,
    },
    /// The game is over.
    GameOver,
}

/// The rules of a game.
#[derive(Clone, Debug, PartialEq)]
pub struct Rules {
    variant: Variant,
    contestants: Vec<Contestant>,
    flags: Vec<Flag>,
    ticks: u32,
    over: bool,
    events: Vec<GameEvent>,
    /// Where each player stood at the last of the game's ticks, which its
    /// update sees before the objects move (`game_tick`).
    standing: Vec<Option<[f32; 3]>>,
    /// The last of the game's ticks each flag was held at the objects'
    /// update, or sent home: its untouched time counts from then
    /// (`item_update`, `game_engine_flag_reset`).
    owned: Vec<u32>,
}

/// Whether `a` is nearer than `radius` to `b` (`ctf_position_near_flag`).
fn within(a: [f32; 3], b: [f32; 3], radius: f32) -> bool {
    let d = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    d[0] * d[0] + d[1] * d[1] + d[2] * d[2] < radius * radius
}

impl Rules {
    /// A game of `variant`, whose flags stand at `stands` (a team and its
    /// stand each), with no players yet.
    pub fn new(variant: Variant, stands: &[(u16, [f32; 3])]) -> Self {
        let flags = if variant.game_type == GameType::CaptureTheFlag {
            stands
                .iter()
                .map(|&(team, stand)| Flag {
                    team,
                    stand,
                    state: FlagState::Home,
                })
                .collect()
        } else {
            Vec::new()
        };
        let owned = vec![0; flags.len()];
        Self {
            variant,
            contestants: Vec::new(),
            flags,
            ticks: 0,
            over: false,
            events: Vec::new(),
            standing: Vec::new(),
            owned,
        }
    }

    /// The variant.
    pub fn variant(&self) -> &Variant {
        &self.variant
    }

    /// Adds a player, alive, on `team` (ignored without teams: each
    /// player is its own); its index is returned.
    pub fn join(&mut self, team: u16) -> usize {
        let index = self.contestants.len();
        let team = if self.variant.teams {
            team
        } else {
            u16::try_from(index).unwrap_or(u16::MAX)
        };
        self.contestants.push(Contestant {
            team,
            alive: true,
            ..Contestant::default()
        });
        index
    }

    /// The players.
    pub fn contestants(&self) -> &[Contestant] {
        &self.contestants
    }

    /// The flags.
    pub fn flags(&self) -> &[Flag] {
        &self.flags
    }

    /// How long the game has lasted.
    pub fn ticks(&self) -> u32 {
        self.ticks
    }

    /// Whether the game is over.
    pub fn is_over(&self) -> bool {
        self.over
    }

    /// A team's score: its players' scores in Slayer, its captures in
    /// Capture the Flag. Without teams, a player's.
    /// (`slayer_get_score`, `ctf_get_score`, `game_engine_get_team_score`)
    pub fn team_score(&self, team: u16) -> i32 {
        self.contestants
            .iter()
            .filter(|contestant| contestant.team == team)
            .map(|contestant| contestant.score)
            .sum()
    }

    /// The leading team (without teams, player), if one leads alone.
    pub fn leader(&self) -> Option<u16> {
        let mut teams: Vec<u16> = self.contestants.iter().map(|c| c.team).collect();
        teams.sort_unstable();
        teams.dedup();
        let mut scores: Vec<(i32, u16)> = teams
            .into_iter()
            .map(|team| (self.team_score(team), team))
            .collect();
        scores.sort_unstable_by_key(|&(score, _)| std::cmp::Reverse(score));
        match scores.as_slice() {
            [(best, team), (next, _), ..] if best > next => Some(*team),
            [(_, team)] => Some(*team),
            _ => None,
        }
    }

    /// Player `victim` died where it stood, at `at`, killed by `killer`
    /// (itself for a suicide; `None` for a fall or the level): its killer
    /// scores, it drops a flag it carried and waits to respawn.
    /// (`game_engine_player_killed`, `slayer_engine_player_killed_player`,
    /// `ctf_engine_player_killed_player`)
    /// (the original's `game_statistics_record_kill`, in part)
    pub fn player_killed(&mut self, victim: usize, killer: Option<usize>, at: [f32; 3]) {
        let Some(dead) = self.contestants.get(victim).copied() else {
            return;
        };
        if !dead.alive || self.over {
            return;
        }
        let killer = killer.filter(|&killer| killer < self.contestants.len());
        let friendly = match killer {
            Some(killer) => killer == victim || self.contestants[killer].team == dead.team,
            None => false,
        };
        // (a death that is not another player's kill costs the penalty)
        let penalty = killer.is_none() || friendly;
        if let Some(killer) = killer {
            let slayer = self.variant.game_type == GameType::Slayer;
            let scorer = &mut self.contestants[killer];
            if killer == victim {
                scorer.suicides += 1;
            } else if friendly {
                scorer.betrayals += 1;
            } else {
                scorer.kills += 1;
            }
            if slayer {
                scorer.score += if friendly { -1 } else { 1 };
            }
        }
        let mut wait = dead.respawn_penalty_ticks + self.variant.respawn_ticks;
        let growth = self.variant.respawn_growth_ticks;
        if growth > 0 {
            let contestant = &mut self.contestants[victim];
            contestant.respawn_penalty_ticks =
                (contestant.respawn_penalty_ticks + growth).min(growth * 5);
            if let Some(killer) = killer.filter(|_| !penalty) {
                let scorer = &mut self.contestants[killer];
                scorer.respawn_penalty_ticks = scorer.respawn_penalty_ticks.saturating_sub(growth);
            }
        }
        if penalty {
            wait += self.variant.suicide_penalty_ticks;
        }
        let contestant = &mut self.contestants[victim];
        contestant.alive = false;
        contestant.deaths += 1;
        contestant.respawn_ticks = wait.max(MINIMUM_RESPAWN_TICKS);
        self.events.push(GameEvent::Killed { victim, killer });
        let now = self.game_ticks();
        for (index, flag) in self.flags.iter_mut().enumerate() {
            if flag.state == FlagState::Carried(victim) {
                flag.state = FlagState::Dropped {
                    at,
                    untouched_ticks: now.saturating_sub(self.owned[index]),
                };
                self.events.push(GameEvent::FlagDropped(index));
            }
        }
        self.check_score();
    }

    /// One tick, with where each player stands (`None` for the dead and
    /// the absent): the respawn timers count down, players touch flags
    /// and carry them home, and dropped flags go home in time. What
    /// happened since the last step is returned.
    /// (`slayer_engine_update`, `ctf_engine_update`)
    pub fn step(&mut self, positions: &[Option<[f32; 3]>]) -> Vec<GameEvent> {
        if !self.over {
            self.ticks += 1;
            for (index, contestant) in self.contestants.iter_mut().enumerate() {
                if contestant.alive {
                    continue;
                }
                contestant.respawn_ticks = contestant.respawn_ticks.saturating_sub(1);
                if contestant.respawn_ticks == 0 {
                    contestant.alive = true;
                    self.events.push(GameEvent::Respawn(index));
                }
            }
            self.step_flags(positions);
            self.check_score();
            if !self.over && self.should_end() {
                self.over = true;
                self.events.push(GameEvent::GameOver);
            }
        }
        std::mem::take(&mut self.events)
    }

    /// The flags' rules, on the game's ticks: a flag lying untouched too
    /// long goes home, the carriers standing at their own stands capture
    /// (where they stood before this tick's moves, as the game's update
    /// sees them), the game may end, and then the players where they stand
    /// now take the enemy's flags and return their own, the lowest player
    /// first and then the lowest flag.
    /// (`ctf_engine_weapon_update`, `ctf_engine_player_update`,
    /// `ctf_engine_update`, `ctf_engine_allow_pick_up`, `ctf_weapon_pickup`)
    fn step_flags(&mut self, positions: &[Option<[f32; 3]>]) {
        if !self.ticks.is_multiple_of(STEPS_PER_GAME_TICK) {
            return;
        }
        let now = self.game_ticks();
        for (index, flag) in self.flags.iter_mut().enumerate() {
            match &mut flag.state {
                FlagState::Carried(_) => continue,
                FlagState::Dropped {
                    untouched_ticks, ..
                } => *untouched_ticks = now.saturating_sub(self.owned[index]),
                FlagState::Home => {}
            }
            // (one at home goes home again, unseen)
            if now.saturating_sub(self.owned[index]) > FLAG_RETURN_TICKS {
                if flag.state != FlagState::Home {
                    flag.state = FlagState::Home;
                    self.events.push(GameEvent::FlagReturned(index));
                }
                self.owned[index] = now;
            }
        }
        let standing = std::mem::take(&mut self.standing);
        for (carrier, at) in standing.iter().enumerate() {
            let Some(at) = self.living(carrier, at) else {
                continue;
            };
            let Some(carried) = self.carried_by(carrier) else {
                continue;
            };
            let team = self.contestants[carrier].team;
            let Some(home) = self.flags.iter().find(|own| own.team == team) else {
                continue;
            };
            let at_home = !self.variant.flag_at_home_to_score || home.state == FlagState::Home;
            if within(at, home.stand, FLAG_TOUCH_RADIUS) && at_home {
                self.flags[carried].state = FlagState::Home;
                self.owned[carried] = now;
                self.contestants[carrier].score += 1;
                self.events.push(GameEvent::Captured {
                    flag: carried,
                    by: carrier,
                });
            }
        }
        self.check_score();
        self.standing = positions.to_vec();
        if self.over {
            return;
        }
        for (index, flag) in self.flags.iter().enumerate() {
            if matches!(flag.state, FlagState::Carried(_)) {
                self.owned[index] = now;
            }
        }
        for (player, at) in positions.iter().enumerate() {
            let Some(at) = self.living(player, at) else {
                continue;
            };
            if self.carried_by(player).is_some() {
                continue;
            }
            let team = self.contestants[player].team;
            for index in 0..self.flags.len() {
                let flag = self.flags[index];
                let Some(position) = flag.position() else {
                    continue;
                };
                if !within(at, position, FLAG_TOUCH_RADIUS) {
                    continue;
                }
                if flag.team != team {
                    self.flags[index].state = FlagState::Carried(player);
                    self.events.push(GameEvent::FlagTaken {
                        flag: index,
                        by: player,
                    });
                    break;
                }
                // (a team's own flag at home is not to be picked up; one
                // lying elsewhere goes home at its touch, unless it must
                // be carried)
                if flag.state != FlagState::Home && !self.variant.flag_must_reset {
                    self.flags[index].state = FlagState::Home;
                    self.owned[index] = now;
                    self.events.push(GameEvent::FlagReturned(index));
                }
            }
        }
    }

    /// How many of the game's ticks the game has lasted.
    fn game_ticks(&self) -> u32 {
        self.ticks / STEPS_PER_GAME_TICK
    }

    /// Where player `index` stands, if it is alive and stands anywhere.
    fn living(&self, index: usize, at: &Option<[f32; 3]>) -> Option<[f32; 3]> {
        self.contestants
            .get(index)
            .filter(|contestant| contestant.alive)
            .and(*at)
    }

    /// The flag player `index` carries.
    fn carried_by(&self, index: usize) -> Option<usize> {
        self.flags
            .iter()
            .position(|flag| flag.state == FlagState::Carried(index))
    }

    /// Whether the game should end: fewer than two teams have a player
    /// in it, or its time limit is up (`game_engine_should_end_game`).
    /// A game of one player or none goes on. Every player counts as in
    /// the game: the engine has no lives, no odd man out and no quitting
    /// (`multiple_teams_alive`).
    pub fn should_end(&self) -> bool {
        let mut teams = self.contestants.iter().map(|contestant| contestant.team);
        let multiple_teams = self.contestants.len() <= 1
            || teams
                .next()
                .is_some_and(|first| teams.any(|team| team != first));
        let time_up = self
            .variant
            .time_limit_ticks
            .is_some_and(|limit| limit > 0 && self.ticks >= limit);
        !multiple_teams || time_up
    }

    fn check_score(&mut self) {
        if self.over {
            return;
        }
        let won = self
            .contestants
            .iter()
            .any(|contestant| self.team_score(contestant.team) >= self.variant.score_to_win);
        if won {
            self.over = true;
            self.events.push(GameEvent::GameOver);
        }
    }
}

/// The game's ticks an item a placement stood lies untaken before it goes
/// (`game_engine_update_purge`), which a placement's new item is given less
/// of so that it lasts one period.
const ITEM_PURGE_GAME_TICKS: i64 = 30 * 30;

/// The game's ticks between a placement's items: its spawn time, else its
/// collection's (as the importer gives it), else 30 seconds.
/// (`game_engine_update_item_spawn`)
pub fn item_respawn_period(seconds: Option<u16>) -> u32 {
    u32::from(seconds.unwrap_or(DEFAULT_EQUIPMENT_SECONDS)) * 30
}

/// An item collection's weights' total, a whole number again after each
/// weight is added (`item_collection_get_total`).
pub fn item_collection_total(weights: impl IntoIterator<Item = f32>) -> i32 {
    weights
        .into_iter()
        .fold(0, |total, weight| (total as f32 + weight) as i32)
}

/// Which of a collection's permutations a placement stands, by weight
/// (`random_item`): a whole number below the total, from which each weight
/// is taken in turn until it falls below zero. None when the weights run
/// out first.
pub fn random_item(seed: &mut RandomSeed, weights: &[f32]) -> Option<usize> {
    let total = item_collection_total(weights.iter().copied());
    let mut remaining = i32::from(seed.next_range(0, total as i16));
    for (index, &weight) in weights.iter().enumerate() {
        remaining = (remaining as f32 - weight) as i32;
        if remaining < 0 {
            return Some(index);
        }
    }
    None
}

/// The items the placements stood that still lie where they were put, with
/// the game tick each was last owned at: one is purged a period after it
/// was stood, as the next one stands (`game_engine_update_purge`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LyingItems<H> {
    lying: Vec<(H, i64)>,
}

impl<H> Default for LyingItems<H> {
    fn default() -> Self {
        Self { lying: Vec::new() }
    }
}

impl<H: Copy> LyingItems<H> {
    /// An item stood at game tick `now` by a placement of `period` ticks.
    pub fn stood(&mut self, item: H, now: u32, period: u32) {
        let owned = i64::from(now) + i64::from(period) - ITEM_PURGE_GAME_TICKS;
        self.lying.push((item, owned));
    }

    /// At game tick `now`, forgets the items that were `taken` and returns
    /// those that lay untouched too long, oldest first, to delete.
    pub fn purge(&mut self, now: u32, taken: impl Fn(H) -> bool) -> Vec<H> {
        let cutoff = i64::from(now) - ITEM_PURGE_GAME_TICKS;
        let mut purged = Vec::new();
        self.lying.retain(|&(item, owned)| {
            if taken(item) {
                false
            } else if owned < cutoff {
                purged.push(item);
                false
            } else {
                true
            }
        });
        purged
    }

    /// The items lying there.
    pub fn items(&self) -> impl Iterator<Item = H> + '_ {
        self.lying.iter().map(|&(item, _)| item)
    }
}

/// A game played over a [`Simulation`].
#[derive(Clone, Debug)]
pub struct Game {
    rules: Rules,
    flag_objects: Vec<Option<ObjectHandle>>,
    /// The netgame equipment placements in play.
    placements: Vec<usize>,
    items: LyingItems<ObjectHandle>,
    steps: u32,
    seed: RandomSeed,
}

impl Game {
    /// Starts a game of `variant` on `level` with the simulation's players
    /// (alternating between teams 0 and 1 in a game with teams): its
    /// flags go to their stands; its equipment stands at its placements
    /// with the first step.
    /// (`game_engine_game_starting`)
    pub fn start(simulation: &mut Simulation, level: &Level, variant: Variant) -> Self {
        let stands: Vec<(u16, [f32; 3])> = level
            .scenario
            .netgame_flags
            .iter()
            .filter(|flag| flag.kind == NetgameFlagKind::CtfFlag)
            .map(|flag| (flag.team, flag.position))
            .collect();
        let mut rules = Rules::new(variant, &stands);
        for index in 0..simulation.players().len() {
            rules.join(u16::try_from(index % 2).unwrap_or(0));
        }
        let mut game = Self {
            flag_objects: vec![None; rules.flags().len()],
            rules,
            placements: level
                .scenario
                .netgame_equipment
                .iter()
                .enumerate()
                .filter(|(_, placement)| placement.game_types.matches(variant.game_type))
                .map(|(placement, _)| placement)
                .collect(),
            items: LyingItems::default(),
            steps: 0,
            seed: RandomSeed::default(),
        };
        for flag in 0..game.flag_objects.len() {
            game.place_flag(simulation, level, flag);
        }
        game
    }

    /// Adds a player the simulation added after the game began.
    pub fn join(&mut self, team: u16) -> usize {
        self.rules.join(team)
    }

    /// The rules and the scores.
    pub fn rules(&self) -> &Rules {
        &self.rules
    }

    /// The rules, to kill players by hand.
    pub fn rules_mut(&mut self) -> &mut Rules {
        &mut self.rules
    }

    /// The flags' objects, by flag.
    pub fn flag_objects(&self) -> &[Option<ObjectHandle>] {
        &self.flag_objects
    }

    /// The items the placements stood that still lie where they were put.
    pub fn equipment_objects(&self) -> Vec<ObjectHandle> {
        self.items.items().collect()
    }

    /// One tick, after the simulation's: the kills it saw are scored, the
    /// rules step, the players whose wait is over stand at a start, and
    /// the flags and the taken equipment are put back (`game_engine_update`).
    pub fn step(&mut self, simulation: &mut Simulation, level: &Level) -> Vec<GameEvent> {
        let handles: Vec<ObjectHandle> = simulation.players().iter().map(|p| p.handle()).collect();
        let player_of = |handle: ObjectHandle| handles.iter().position(|&h| h == handle);
        let kills: Vec<(usize, Option<usize>)> = simulation
            .damage_events()
            .iter()
            .filter(|event| event.outcome.killed)
            .filter_map(|event| {
                let victim = player_of(event.object)?;
                Some((victim, event.owner.and_then(player_of)))
            })
            .collect();
        for (victim, killer) in kills {
            let at = position(simulation, handles[victim]).unwrap_or([0.0; 3]);
            self.rules.player_killed(victim, killer, at);
        }
        let positions: Vec<Option<[f32; 3]>> = handles
            .iter()
            .enumerate()
            .map(|(index, &handle)| {
                let alive = self.rules.contestants().get(index).is_some_and(|c| c.alive);
                alive.then(|| position(simulation, handle)).flatten()
            })
            .collect();
        let events = self.rules.step(&positions);
        for event in &events {
            match *event {
                GameEvent::Respawn(index) => self.respawn(simulation, level, index),
                GameEvent::FlagTaken { flag, .. }
                | GameEvent::FlagDropped(flag)
                | GameEvent::FlagReturned(flag)
                | GameEvent::Captured { flag, .. } => self.place_flag(simulation, level, flag),
                GameEvent::Killed { .. } | GameEvent::GameOver => {}
            }
        }
        if self.steps.is_multiple_of(STEPS_PER_GAME_TICK) {
            self.update_items(simulation, level, self.steps / STEPS_PER_GAME_TICK);
        }
        self.steps += 1;
        events
    }

    /// One of the game's ticks of its items at game tick `now`: the
    /// untouched are purged, then each placement whose period comes round
    /// stands an item chosen by weight (`game_engine_update_purge`,
    /// `game_engine_update_item_spawn`).
    fn update_items(&mut self, simulation: &mut Simulation, level: &Level, now: u32) {
        for item in self
            .items
            .purge(now, |item| simulation.object(item).is_none())
        {
            simulation.remove(item);
        }
        for &index in &self.placements {
            let Some(placement) = level.scenario.netgame_equipment.get(index) else {
                continue;
            };
            let period = item_respawn_period(placement.spawn_seconds);
            if !now.is_multiple_of(period) {
                continue;
            }
            let weights: Vec<f32> = placement.items.iter().map(|item| item.weight).collect();
            let Some(tag) = random_item(&mut self.seed, &weights)
                .and_then(|chosen| placement.items.get(chosen))
                .and_then(|choice| choice.item.clone())
            else {
                continue;
            };
            let team = (placement.team != 0).then_some(placement.team);
            if let Some(item) = spawn_item(
                simulation,
                level,
                tag,
                placement.position,
                placement.facing,
                team,
            ) {
                self.items.stood(item, now, period);
            }
        }
    }

    /// Stands player `index` at the start for the game type and its team
    /// farthest from its living enemies.
    /// (`game_engine_get_starting_location_rating`)
    fn respawn(&mut self, simulation: &mut Simulation, level: &Level, index: usize) {
        let Some(contestant) = self.rules.contestants().get(index).copied() else {
            return;
        };
        let variant = *self.rules.variant();
        let enemies: Vec<[f32; 3]> = simulation
            .players()
            .iter()
            .enumerate()
            .filter(|&(other, _)| other != index)
            .filter(|&(other, _)| {
                self.rules
                    .contestants()
                    .get(other)
                    .is_some_and(|c| c.alive && c.team != contestant.team)
            })
            .filter_map(|(_, player)| position(simulation, player.handle()))
            .collect();
        let starts = &level.scenario.player_starts;
        let fits = |team_matters: bool| {
            starts
                .iter()
                .enumerate()
                .filter(move |(_, start)| start.game_types.matches(variant.game_type))
                .filter(move |(_, start)| !team_matters || start.team == contestant.team)
                .map(|(at, _)| at)
        };
        let candidates: Vec<usize> = if variant.teams && fits(true).next().is_some() {
            fits(true).collect()
        } else if fits(false).next().is_some() {
            fits(false).collect()
        } else {
            (0..starts.len()).collect()
        };
        let nearest_enemy = |start: usize| {
            let at = starts[start].position;
            enemies
                .iter()
                .map(|enemy| {
                    let d = [at[0] - enemy[0], at[1] - enemy[1], at[2] - enemy[2]];
                    d[0] * d[0] + d[1] * d[1] + d[2] * d[2]
                })
                .fold(f32::INFINITY, f32::min)
        };
        let chosen = if enemies.is_empty() {
            // (no one to avoid: a random one of them)
            let pick = usize::from(self.seed.next_u16()) % candidates.len().max(1);
            candidates.get(pick).copied()
        } else {
            candidates
                .iter()
                .copied()
                .max_by(|&a, &b| nearest_enemy(a).total_cmp(&nearest_enemy(b)))
        };
        if let Some(start) = chosen {
            let team = variant.teams.then_some(contestant.team);
            let _ = simulation.respawn_player(index, level, start, team);
        }
    }

    /// Puts flag `flag`'s object where the flag is (none while carried).
    fn place_flag(&mut self, simulation: &mut Simulation, level: &Level, flag: usize) {
        if let Some(handle) = self.flag_objects.get_mut(flag).and_then(Option::take) {
            simulation.remove(handle);
        }
        let Some(tag) = level.globals.multiplayer_flag.clone() else {
            return;
        };
        let Some(state) = self.rules.flags().get(flag).copied() else {
            return;
        };
        let Some(at) = state.position() else {
            return;
        };
        let spawned = spawn_item(simulation, level, tag, at, 0.0, Some(state.team));
        if let Some(slot) = self.flag_objects.get_mut(flag) {
            *slot = spawned;
        }
    }
}

/// Where an object stands, as an array.
fn position(simulation: &Simulation, handle: ObjectHandle) -> Option<[f32; 3]> {
    simulation
        .transform(handle)
        .map(|transform| transform.position.to_array())
}

/// Spawns an item (a weapon or equipment) at `at`, facing `facing`.
fn spawn_item(
    simulation: &mut Simulation,
    level: &Level,
    tag: TagRef,
    at: [f32; 3],
    facing: f32,
    team: Option<u16>,
) -> Option<ObjectHandle> {
    let kind = level.object(&tag)?.kind;
    if !matches!(kind, ObjectKind::Weapon | ObjectKind::Equipment) {
        return None;
    }
    simulation
        .spawn(ObjectSpawn {
            kind,
            tag,
            transform: Transform {
                position: Vec3::from_array(at),
                rotation: Quat::from_rotation_z(facing),
                scale: 1.0,
            },
            name: None,
            permutation: None,
            team,
        })
        .ok()
}
