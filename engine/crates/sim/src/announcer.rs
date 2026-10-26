//! The multiplayer announcer (`game_engine_multiplayer_sounds.c`): the
//! game's sounds for its events, queued so that one finishes before the
//! next begins, and the sound each game type plays as it starts.
//!
//! The queue counts the game's ticks (30 a second). Its first entry is
//! the sound playing, or a two-second wait as a game begins; a queued
//! sound waits its longest permutation and five ticks more, and a sound
//! that is not queued (a countdown, a hill moving) plays at once over
//! whatever plays.

use opence_content::GameType;

use crate::game::Variant;

/// One of the globals' multiplayer information sounds, by its index there
/// (`multiplayer_information_sound`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MultiplayerSound(pub u8);

impl MultiplayerSound {
    /// "Play ball!", as a game of Oddball begins.
    pub const PLAY_BALL: Self = Self(0);
    /// "Game over".
    pub const GAME_OVER: Self = Self(1);
    /// "Oddball".
    pub const ODDBALL: Self = Self(19);
    /// "Race".
    pub const RACE: Self = Self(20);
    /// "Slayer".
    pub const SLAYER: Self = Self(21);
    /// "Capture the Flag".
    pub const CAPTURE_THE_FLAG: Self = Self(22);
    /// "Team King of the Hill".
    pub const TEAM_KING_OF_THE_HILL: Self = Self(32);
    /// "Team Slayer".
    pub const TEAM_SLAYER: Self = Self(35);
    /// "King of the Hill".
    pub const KING_OF_THE_HILL: Self = Self(36);
    /// The last of the sounds, which nothing plays.
    pub const TING: Self = Self(43);

    /// Whether it waits for the sounds before it rather than playing at
    /// once: all but the countdowns, the teleporter, a flag's failure, the
    /// respawn and the hill's moves (`sound_is_queueable`). Ting, past the
    /// end of the game's table, is never played.
    pub fn is_queueable(self) -> bool {
        !matches!(self.0, 26..=31 | 42..)
    }
}

/// The globals' multiplayer information sounds, as long as each one's
/// longest permutation in milliseconds, or `None` where the globals name
/// no sound. Empty when the globals have no multiplayer information.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MultiplayerSounds(pub Vec<Option<u32>>);

impl MultiplayerSounds {
    /// Whether `sound` is one the globals name, which plays when its turn
    /// comes (`_game_engine_play_multiplayer_sound`).
    pub fn has(&self, sound: MultiplayerSound) -> bool {
        matches!(self.0.get(usize::from(sound.0)), Some(Some(_)))
    }

    /// How many of the game's ticks `sound` lasts, rounded down, or 0 for
    /// a sound the globals do not name (`get_sound_length_in_ticks`).
    pub fn length_in_ticks(&self, sound: MultiplayerSound) -> u32 {
        match self.0.get(usize::from(sound.0)) {
            Some(Some(milliseconds)) => {
                let ticks = i64::from(*milliseconds) * i64::from(GAME_TICKS_PER_SECOND) / 1000;
                u32::try_from(ticks).unwrap_or(u32::MAX)
            }
            _ => 0,
        }
    }
}

/// The game's ticks a second, which the queue counts.
const GAME_TICKS_PER_SECOND: u32 = 30;

/// How many sounds the queue holds, the one playing among them
/// (`MAXIMUM_QUEUED_MULTIPLAYER_SOUNDS`).
pub const MAXIMUM_QUEUED_SOUNDS: usize = 5;

/// The wait before a game's first sound: two seconds.
const STARTING_DELAY_TICKS: u32 = 2 * GAME_TICKS_PER_SECOND;

/// The ticks a queued sound waits past its own length.
const QUEUED_GAP_TICKS: u32 = 5;

/// A sound in the queue and the game's ticks left until the next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Queued {
    /// The sound, or `None` for the wait as a game begins.
    sound: Option<MultiplayerSound>,
    delay_ticks: u32,
}

/// The queue of the announcer's sounds (`mp_sound_queue`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Announcer {
    queue: Vec<Queued>,
}

impl Default for Announcer {
    fn default() -> Self {
        Self::new()
    }
}

impl Announcer {
    /// The queue as a game begins: a two-second wait before anything
    /// queued plays (`game_engine_intialize_queued_sounds`).
    pub fn new() -> Self {
        Self {
            queue: vec![Queued {
                sound: None,
                delay_ticks: STARTING_DELAY_TICKS,
            }],
        }
    }

    /// Asks for `sound`: a queued sound waits its turn, dropped when the
    /// queue is full, and plays now if nothing else is queued; any other
    /// plays now. The sound to play now is returned, if the globals name
    /// it (`game_engine_play_multiplayer_sound`).
    pub fn play(
        &mut self,
        sound: MultiplayerSound,
        sounds: &MultiplayerSounds,
    ) -> Option<MultiplayerSound> {
        if sound.is_queueable() {
            if self.queue.len() < MAXIMUM_QUEUED_SOUNDS {
                self.queue.push(Queued {
                    sound: Some(sound),
                    delay_ticks: sounds.length_in_ticks(sound) + QUEUED_GAP_TICKS,
                });
            }
            if self.queue.len() != 1 {
                return None;
            }
        }
        sounds.has(sound).then_some(sound)
    }

    /// One of the game's ticks: when the first entry's time is up the next
    /// begins, and the sound to play now is returned, if the globals name
    /// it (`game_engine_update_multiplayer_sound`).
    pub fn update(&mut self, sounds: &MultiplayerSounds) -> Option<MultiplayerSound> {
        let first = self.queue.first_mut()?;
        first.delay_ticks = first.delay_ticks.wrapping_sub(1);
        if first.delay_ticks != 0 {
            return None;
        }
        self.queue.remove(0);
        let next = self.queue.first()?.sound?;
        sounds.has(next).then_some(next)
    }

    /// How many sounds are queued, the one playing among them.
    pub fn queued(&self) -> usize {
        self.queue.len()
    }
}

/// The sound a game of `variant` plays as it starts: its type's name, or
/// its team type's, and nothing for Oddball (`slayer_engine_game_starting`,
/// `ctf_engine_game_starting`, `king_engine_game_starting`,
/// `oddball_engine_game_starting`). Race's is not in the sources, so Race
/// plays nothing.
pub fn starting_sound(variant: &Variant) -> Option<MultiplayerSound> {
    match variant.game_type {
        GameType::Slayer if variant.teams => Some(MultiplayerSound::TEAM_SLAYER),
        GameType::Slayer => Some(MultiplayerSound::SLAYER),
        GameType::CaptureTheFlag => Some(MultiplayerSound::CAPTURE_THE_FLAG),
        GameType::King if variant.teams => Some(MultiplayerSound::TEAM_KING_OF_THE_HILL),
        GameType::King => Some(MultiplayerSound::KING_OF_THE_HILL),
        GameType::Oddball | GameType::Race => None,
    }
}
