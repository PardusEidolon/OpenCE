//! A particle's life as the game's particles.c lives it: the choices
//! `particle_new` makes for it (whether it plays backwards and is
//! mirrored, how long it lives, how long a frame shows, the sequence and
//! frame it begins on), the sequences it begins with, loops through and
//! ends with (`particle_next_sequence`), the frames it steps through
//! (`particle_next_frame`, `particle_update_frame_time`), and when
//! `particles_update` lets it die. A particle with final sequences lives
//! past its lifespan until they are played; one whose sequences run out
//! dies when they do.
//!
//! It draws from a [`RandomSeed`], as the game draws from its local
//! seed, so tools/engine_particle_reference.py's trace of the game's code
//! is met draw for draw (ffi/tests/particle_reference.rs).

use opence_content::particle::Particle;
use opence_sim::random::RandomSeed;

/// The seconds of a particle's life every local player shares: beyond
/// them, its life is split among them (`PARTICLE_SHARED_LIFESPAN`).
pub const SHARED_LIFESPAN: f32 = 0.7;
/// The game's ticks a second, by which `particles_update` counts how many
/// ticks an update spans for the particles that step once a frame.
const TICKS_PER_SECOND: f32 = 30.0;
/// No sequence (`NONE`).
pub const NO_SEQUENCE: i32 = -1;
/// The most frames one update steps through. The game's loop has no
/// bound, and a frame shown no time at all would hold it forever; no
/// particle the game's tools make shows over a thousand frames a tick.
const MOST_FRAMES_AN_UPDATE: u32 = 4096;

/// Which of its sequences a particle takes next (`_particle_state_*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// It takes one of its initial sequences.
    Initial,
    /// It takes one of its looping sequences.
    Looping,
    /// It is looping: it takes another while it lives.
    StillLooping,
    /// It takes one of its final sequences.
    Final,
    /// It has played them all.
    Done,
}

impl Stage {
    /// The stage after this one (`particle->state++`).
    fn next(self) -> Self {
        match self {
            Self::Initial => Self::Looping,
            Self::Looping | Self::StillLooping => Self::Final,
            Self::Final | Self::Done => Self::Done,
        }
    }
}

/// How many updates span `dt` seconds for the particles that step once a
/// frame (`particles_update`'s `particles_update_ticks`), the fraction of
/// a tick left over kept for the next.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Clock {
    leftover: f32,
}

impl Clock {
    /// The ticks `dt` seconds bring, with what was left over.
    pub fn ticks(&mut self, dt: f32) -> i32 {
        self.leftover += dt * TICKS_PER_SECOND;
        let ticks = self.leftover as i32;
        self.leftover -= ticks as f32;
        ticks
    }
}

/// A particle's age and animation.
#[derive(Clone, Debug, PartialEq)]
pub struct Life {
    /// The seconds it has lived.
    pub age: f32,
    /// The seconds it lives (longer, if it has final sequences to play).
    pub lifespan: f32,
    /// Which of its sequences it takes next.
    pub stage: Stage,
    /// The sequence of its bitmap it shows ([`NO_SEQUENCE`] once dead).
    pub sequence: i32,
    /// The sprite of that sequence it shows (-1 before its first).
    pub frame: i32,
    /// The seconds it has shown it (-1 before its first).
    pub frame_time: f32,
    /// The seconds a frame shows (`f32::MAX` if it does not animate);
    /// longer each time it meets the world.
    pub frame_span: f32,
    /// It plays its frames backwards.
    pub backwards: bool,
    /// Its picture is mirrored across and up and down.
    pub mirrors: [bool; 2],
}

/// The sprites in sequence `sequence` of `sprites`, a bitmap's counts.
fn sprites_in(sprites: &[i32], sequence: i32) -> i32 {
    usize::try_from(sequence)
        .ok()
        .and_then(|index| sprites.get(index))
        .copied()
        .unwrap_or(0)
}

impl Life {
    /// A new particle's life (`particle_new`'s part of it), for `particle`
    /// whose bitmap's sequences have `sprites` sprites, among `players`
    /// local players: dead from the start (see [`Life::alive`]) if it has
    /// no sequence to show, when it dies as the game's does.
    pub fn new(
        particle: &Particle,
        sprites: &[i32],
        players: u16,
        random: &mut RandomSeed,
    ) -> Self {
        let flags = particle.flags;
        let mut backwards = false;
        let mut mirrors = [false; 2];
        // the game ors a random word's bit into the particle's own flags,
        // whose bits are backwards (0), at rest, u mirror (2), v mirror (3)
        if flags & Particle::CAN_ANIMATE_BACKWARDS != 0 {
            backwards = random.next_u16() & 1 != 0;
        }
        if flags & Particle::RANDOM_HORIZONTAL_MIRRORING != 0 {
            mirrors[0] = random.next_u16() & 1 << 2 != 0;
        }
        if flags & Particle::RANDOM_VERTICAL_MIRRORING != 0 {
            mirrors[1] = random.next_u16() & 1 << 3 != 0;
        }
        let [shortest, longest] = particle.lifespan;
        let mut lifespan = random.next_real_range(shortest, longest);
        if lifespan > SHARED_LIFESPAN {
            lifespan = (lifespan - SHARED_LIFESPAN) / f32::from(players.max(1)) + SHARED_LIFESPAN;
        }
        let [slowest, fastest] = particle.fps;
        let frame_span = if fastest != 0.0 {
            1.0 / random.next_real_range(slowest, fastest)
        } else {
            f32::MAX
        };
        let mut life = Self {
            age: 0.0,
            lifespan,
            stage: Stage::Initial,
            sequence: NO_SEQUENCE,
            frame: -1,
            frame_time: -1.0,
            frame_span,
            backwards,
            mirrors,
        };
        if life.next_sequence(particle, sprites, random) {
            let count = sprites_in(sprites, life.sequence);
            life.frame = if flags & Particle::ANIMATION_STARTS_ON_RANDOM_FRAME != 0 {
                let from = if backwards { 1 } else { -1 };
                from + i32::from(random.next_range(0, i16::try_from(count).unwrap_or(i16::MAX)))
            } else if backwards {
                count
            } else {
                -1
            };
        }
        life
    }

    /// It still lives: it has a sequence to show.
    pub fn alive(&self) -> bool {
        self.sequence != NO_SEQUENCE
    }

    /// Its radius by `particle`'s radius over its life, for a radius of 1
    /// at emission (`particle_get_radius`): past its lifespan, as it plays
    /// its final sequences, the radius goes on the way it went.
    pub fn radius(&self, particle: &Particle) -> f32 {
        let [birth, death] = particle.radius;
        (death - birth) * (self.age / self.lifespan) + birth
    }

    /// One update of `dt` seconds, `ticks` of them for the particles that
    /// step once a frame, at rest or not (`particles_update`'s, up to its
    /// physics): false if it died, which it does as the game's does, with
    /// its death effect. Its physics comes after, while it lives.
    pub fn update(
        &mut self,
        particle: &Particle,
        sprites: &[i32],
        (dt, ticks): (f32, i32),
        at_rest: bool,
        random: &mut RandomSeed,
    ) -> bool {
        let was_new = self.age == 0.0;
        self.age += dt;
        if self.age >= self.lifespan && !was_new && particle.final_sequences == 0 {
            return false;
        }
        self.update_frame_time(particle, sprites, (dt, ticks), at_rest, random)
    }

    /// Shows its frames for `dt` seconds more (`particle_update_frame_time`).
    fn update_frame_time(
        &mut self,
        particle: &Particle,
        sprites: &[i32],
        (mut dt, ticks): (f32, i32),
        at_rest: bool,
        random: &mut RandomSeed,
    ) -> bool {
        if particle.flags & Particle::ANIMATION_STOPS_AT_REST != 0 && at_rest {
            return true;
        }
        if particle.flags & Particle::ANIMATE_ONCE_PER_FRAME != 0 {
            if dt != 0.0 && ticks > 0 {
                return self.next_frame(particle, sprites, random);
            }
            return true;
        }
        let mut alive = true;
        if self.frame_time == -1.0 {
            alive = self.next_frame(particle, sprites, random);
            self.frame_time = 0.0;
        }
        let mut frames = 0;
        while dt > 0.0 && alive && frames < MOST_FRAMES_AN_UPDATE {
            let remaining = self.frame_span - self.frame_time;
            if remaining <= dt {
                alive = self.next_frame(particle, sprites, random);
                dt -= remaining;
                frames += 1;
            } else {
                self.frame_time += dt;
                break;
            }
        }
        alive
    }

    /// Steps to its next frame, and to its next sequence at the end of
    /// one (`particle_next_frame`).
    fn next_frame(
        &mut self,
        particle: &Particle,
        sprites: &[i32],
        random: &mut RandomSeed,
    ) -> bool {
        self.frame_time = 0.0;
        if self.backwards {
            if self.frame > 0 {
                self.frame -= 1;
                return true;
            }
            let alive = self.next_sequence(particle, sprites, random);
            if alive {
                self.frame = sprites_in(sprites, self.sequence) - 1;
            }
            alive
        } else {
            if self.frame + 1 < sprites_in(sprites, self.sequence) {
                self.frame += 1;
                return true;
            }
            let alive = self.next_sequence(particle, sprites, random);
            self.frame = 0;
            alive
        }
    }

    /// Takes its next sequence, of those its stage chooses from: false if
    /// there is none, and it dies (`particle_next_sequence`).
    fn next_sequence(
        &mut self,
        particle: &Particle,
        sprites: &[i32],
        random: &mut RandomSeed,
    ) -> bool {
        let first = i32::from(particle.first_sequence);
        let initial = i32::from(particle.initial_sequences);
        let looping = i32::from(particle.looping_sequences);
        let last = i32::from(particle.final_sequences);
        self.sequence = NO_SEQUENCE;
        if self.stage == Stage::Initial {
            if initial > 0 {
                self.sequence = first + i32::from(random.next_range(0, particle.initial_sequences));
            }
            self.stage = self.stage.next();
        }
        if self.sequence == NO_SEQUENCE && self.stage == Stage::Looping
            || self.stage == Stage::StillLooping
        {
            self.stage = Stage::StillLooping;
            if self.age < self.lifespan && looping > 0 {
                self.sequence =
                    initial + first + i32::from(random.next_range(0, particle.looping_sequences));
            } else {
                self.stage = self.stage.next();
            }
        }
        if self.sequence == NO_SEQUENCE && self.stage == Stage::Final {
            if last > 0 {
                self.sequence = looping
                    + initial
                    + first
                    + i32::from(random.next_range(0, particle.final_sequences));
            }
            self.stage = self.stage.next();
        }
        let count = i32::try_from(sprites.len()).unwrap_or(i32::MAX);
        if self.sequence == NO_SEQUENCE || count == 0 {
            self.sequence = NO_SEQUENCE;
            return false;
        }
        self.sequence = self.sequence.clamp(0, count - 1);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opence_content::{Group, TagRef};

    fn particle() -> Particle {
        Particle::new(TagRef {
            group: Group(*b"part"),
            path: "smoke".into(),
        })
    }

    #[test]
    fn a_particle_with_no_more_sequences_dies_when_its_last_ends() {
        let mut smoke = particle();
        smoke.fps = [30.0, 30.0];
        smoke.lifespan = [10.0, 10.0];
        let sprites = [3];
        let mut random = RandomSeed::default();
        let mut life = Life::new(&smoke, &sprites, 1, &mut random);
        assert!(life.alive());
        let mut updates = 0;
        while life.update(&smoke, &sprites, (1.0 / 30.0, 1), false, &mut random) {
            updates += 1;
        }
        // its three frames, then no looping or final sequence to take
        assert_eq!(updates, 2, "{life:?}");
    }

    #[test]
    fn a_particle_with_final_sequences_lives_past_its_lifespan() {
        let mut spark = particle();
        spark.fps = [30.0, 30.0];
        spark.lifespan = [0.1, 0.1];
        spark.looping_sequences = 1;
        spark.final_sequences = 1;
        let sprites = [2, 2, 8];
        let mut random = RandomSeed::default();
        let mut life = Life::new(&spark, &sprites, 1, &mut random);
        let mut sequences = vec![life.sequence];
        while life.update(&spark, &sprites, (1.0 / 30.0, 1), false, &mut random) {
            if sequences.last() != Some(&life.sequence) {
                sequences.push(life.sequence);
            }
        }
        assert_eq!(sequences.first(), Some(&0));
        assert_eq!(sequences.last(), Some(&2));
        assert!(life.age > life.lifespan, "{life:?}");
    }

    #[test]
    fn a_particle_with_no_sprites_dies_at_birth() {
        let life = Life::new(&particle(), &[], 1, &mut RandomSeed::default());
        assert!(!life.alive());
    }

    #[test]
    fn a_frame_shown_no_time_does_not_hold_an_update_forever() {
        let mut stuck = particle();
        stuck.fps = [1e30, 1e30];
        stuck.lifespan = [10.0, 10.0];
        stuck.looping_sequences = 1;
        let sprites = [1, 1];
        let mut random = RandomSeed::default();
        let mut life = Life::new(&stuck, &sprites, 1, &mut random);
        assert!(life.frame_span < 1e-20);
        assert!(life.update(&stuck, &sprites, (1.0 / 30.0, 1), false, &mut random));
    }
}
