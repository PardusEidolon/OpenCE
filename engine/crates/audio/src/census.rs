//! What the player made of the sounds asked of it, by class, for the dev
//! panel's audio view and the object inspector: the voices playing, the
//! sounds culled (skipped by their skip fraction or a promotion, out of
//! hearing, silent at their scale) and those that failed (no pitch range or
//! permutation to play, samples that would not decode). It keeps the same
//! counts for each sound, and the paths asked for that the level has no
//! sound of, so that a sweep of the level can list what never plays.

use std::collections::BTreeMap;

use crate::mixer::{Mixer, VoiceId};

/// Why a sound asked for did not play.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dropped {
    /// Skipped, out of hearing or silent.
    Culled,
    /// Nothing to play, or samples that would not decode.
    Failed,
}

/// A voice playing, and what plays on it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Playing {
    /// The mixer's voice.
    pub voice: VoiceId,
    /// The sound, in the bank's.
    pub sound: usize,
    /// Its sound class.
    pub class: u16,
    /// Where it plays from; `None` for one heard everywhere alike.
    pub position: Option<[f32; 3]>,
    /// When it started, on the player's clock, in milliseconds.
    pub started_ms: f32,
}

/// One class's sounds: the voices playing now, and the sounds culled and
/// failed since the level started.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClassCount {
    /// The sound class.
    pub class: u16,
    /// Its voices playing.
    pub playing: u32,
    /// Its sounds culled.
    pub culled: u32,
    /// Its sounds that failed.
    pub failed: u32,
}

/// One sound's voices started, and its sounds culled and failed, since
/// the level started.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SoundCount {
    /// The sound, in the bank's.
    pub sound: usize,
    /// Its voices started.
    pub played: u32,
    /// Its sounds culled.
    pub culled: u32,
    /// Its sounds that failed.
    pub failed: u32,
}

/// The voices playing and the sounds dropped, kept by the player.
#[derive(Debug, Default)]
pub struct Census {
    playing: Vec<Playing>,
    /// Each class's culled and failed.
    dropped: BTreeMap<u16, [u32; 2]>,
    /// Each sound's started, culled and failed.
    sounds: BTreeMap<usize, SoundCount>,
    /// The paths asked for that name no sound of the level's, and how
    /// often.
    unknown: BTreeMap<String, u32>,
}

impl Census {
    pub(crate) fn played(&mut self, playing: Playing) {
        let count = self.sound(playing.sound);
        count.played = count.played.saturating_add(1);
        self.playing.push(playing);
    }

    /// A sound queued on a voice already counted (a looping track's start
    /// or stop on its loop's voice).
    pub(crate) fn queued(&mut self, sound: usize) {
        let count = self.sound(sound);
        count.played = count.played.saturating_add(1);
    }

    pub(crate) fn dropped(&mut self, sound: usize, class: u16, why: Dropped) {
        let counts = self.dropped.entry(class).or_default();
        let count = &mut counts[usize::from(why == Dropped::Failed)];
        *count = count.saturating_add(1);
        let count = self.sound(sound);
        let count = match why {
            Dropped::Culled => &mut count.culled,
            Dropped::Failed => &mut count.failed,
        };
        *count = count.saturating_add(1);
    }

    pub(crate) fn unknown(&mut self, path: &str) {
        let count = self.unknown.entry(path.to_ascii_lowercase()).or_default();
        *count = count.saturating_add(1);
    }

    fn sound(&mut self, sound: usize) -> &mut SoundCount {
        self.sounds.entry(sound).or_insert(SoundCount {
            sound,
            ..SoundCount::default()
        })
    }

    /// Every sound asked for since the level started, in the bank's order.
    pub fn sounds(&self) -> impl Iterator<Item = &SoundCount> {
        self.sounds.values()
    }

    /// The paths asked for that name no sound of the level's, with how
    /// often each was, in order.
    pub fn unknown_paths(&self) -> impl Iterator<Item = (&str, u32)> {
        self.unknown
            .iter()
            .map(|(path, &count)| (path.as_str(), count))
    }

    /// Forgets the voices that ended.
    pub(crate) fn prune(&mut self, mixer: &Mixer) {
        self.playing
            .retain(|playing| mixer.is_playing(playing.voice));
    }

    /// The voices playing (as of the last prune).
    pub fn playing(&self) -> &[Playing] {
        &self.playing
    }

    /// Every class with a sound playing, culled or failed, in the classes'
    /// order.
    pub fn counts(&self) -> Vec<ClassCount> {
        let mut counts: BTreeMap<u16, ClassCount> = self
            .dropped
            .iter()
            .map(|(&class, &[culled, failed])| {
                (
                    class,
                    ClassCount {
                        class,
                        playing: 0,
                        culled,
                        failed,
                    },
                )
            })
            .collect();
        for playing in &self.playing {
            let count = counts.entry(playing.class).or_insert(ClassCount {
                class: playing.class,
                ..ClassCount::default()
            });
            count.playing = count.playing.saturating_add(1);
        }
        counts.into_values().collect()
    }
}
