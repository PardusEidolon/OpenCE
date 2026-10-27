//! A unit running blindly and burning to death (units.c): the run it
//! starts on, from its actor's way or its own facing and turned at random,
//! the way it wanders each tick, and its flaming death, begun and then
//! dealt.
//!
//! What they do to the world and ask of it is the caller's, a
//! [`FlameWorld`]; what they draw at random is from the game's
//! [`RandomSeed`].
//!
//! Each is checked against the game's code by
//! tests/unit_flaming_reference.rs.

use glam::Vec3;

use crate::random::{rotate_about_axis, RandomSeed};
use crate::unit_control::UnitFlags;
use crate::unit_life::DamageFlags;

/// The game's pi (`_pi`).
const PI: f32 = core::f32::consts::PI;

/// The most a run blindly turns its turning in a tick, in radians
/// (1.2 degrees).
const TURN_ACCELERATION: f32 = 0.020_943_95;

/// A turning's room, in radians, to the share of a tick's turning it
/// allows (one over 3.6 degrees).
const TURN_ROOM: f32 = 15.915_494;

/// `angle` in degrees in radians, as the game reckons it, in floats
/// (`DEGREES_TO_RADIANS`).
fn degrees(angle: f32) -> f32 {
    angle * PI / 180.0
}

/// The game's sine and cosine, through the double library's (`sine`,
/// `cosine`).
fn sine(angle: f32) -> f32 {
    libm::sin(f64::from(angle)) as f32
}

fn cosine(angle: f32) -> f32 {
    libm::cos(f64::from(angle)) as f32
}

/// The world a burning unit is in, as units.c asks of it and acts on it.
pub trait FlameWorld {
    /// Drops the weapon in the unit's hand, as it dies
    /// (`unit_drop_current_weapon`, with its delete).
    fn drop_current_weapon(&mut self);
    /// The game globals' flaming death damage effect, if any
    /// (`falling_damage[0].flaming_death_damage_effect`).
    fn flaming_death_effect(&self) -> Option<u32>;
    /// The object's owners, if it is still there (`object_try_and_get`).
    fn owners(&self, object: u32) -> Option<Owners>;
    /// Deals the damage to the unit, whose damage flags it may kill
    /// (`object_cause_damage`, with no node, region or material and no
    /// normal).
    fn cause_damage(&mut self, damage: FlameDamage, damage_flags: &mut DamageFlags);
}

/// An object's owners, as its object datum keeps them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Owners {
    /// The player owning it (`owner_player_index`).
    pub player: Option<u32>,
    /// The object owning it (`owner_object_index`).
    pub object: Option<u32>,
    /// Its team (`owner_team_index`).
    pub team: Option<i16>,
}

/// The damage a flaming death deals: its effect and whose it is, the rest
/// of the game's damage data as `damage_data_new` leaves it (no material
/// or location, a scale and multiplier of one).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlameDamage {
    /// The damage effect (`definition_index`).
    pub effect: u32,
    /// Whose it is.
    pub owners: Owners,
}

/// A unit's run blindly: where it heads, about its facing, and how fast
/// that turns (`run_blindly_angle`, `run_blindly_angle_delta`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RunBlind {
    /// The angle it runs at, in radians.
    pub angle: f32,
    /// How much the angle turns a tick.
    pub delta: f32,
}

/// What a unit's burning keeps of it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Burning {
    /// The unit's flags.
    pub flags: UnitFlags,
    /// Its damage flags.
    pub damage_flags: DamageFlags,
    /// Its run blindly.
    pub run: RunBlind,
    /// The ticks until it dies of its burning, or none
    /// (`flaming_death_delay`).
    pub delay: u8,
    /// Who set it burning (`flaming_death_attacker_object_index`).
    pub attacker: Option<u32>,
}

impl Burning {
    /// Starts it running blindly, unless it is (`unit_start_running_blindly`):
    /// straight along the way its actor gives (`actor_way`, the actor's
    /// running blind vector) or along its `forward` facing,
    /// turned at random up to 25 or 100 degrees.
    pub fn start_running_blindly(
        &mut self,
        actor_way: Option<Vec3>,
        forward: Vec3,
        random: &mut RandomSeed,
    ) {
        if self.flags.has(UnitFlags::RUNNING_BLINDLY) {
            return;
        }
        self.flags.set(UnitFlags::RUNNING_BLINDLY, true);
        let range = if actor_way.is_some() {
            self.run.angle = 0.0;
            degrees(25.0)
        } else {
            let mut yaw = libm::atan2(f64::from(forward.y), f64::from(forward.x)) as f32;
            if yaw > PI {
                yaw -= 2.0 * PI;
            }
            self.run.angle = yaw;
            degrees(100.0)
        };
        self.run.angle += random.next_real_range(-range, range);
    }

    /// Starts it burning to death, set burning by `attacker`
    /// (`unit_start_flaming_to_death`): it drops its weapon, is no longer
    /// dead and can't be hurt, and, unless it burns already, is given
    /// 60 to 149 ticks to burn and starts running blindly.
    pub fn start_flaming_to_death(
        &mut self,
        attacker: Option<u32>,
        actor_way: Option<Vec3>,
        forward: Vec3,
        world: &mut impl FlameWorld,
        random: &mut RandomSeed,
    ) {
        world.drop_current_weapon();
        self.flags.set(UnitFlags::IGNORE_HARD_PINGS, true);
        self.damage_flags.set(DamageFlags::DEAD, false);
        self.damage_flags.set(DamageFlags::CANNOT_TAKE_DAMAGE, true);
        if self.delay == 0 {
            let delay = random.next_range(60, 150).clamp(1, 255);
            self.delay = u8::try_from(delay).unwrap_or(u8::MAX);
            self.attacker = attacker;
            self.start_running_blindly(actor_way, forward, random);
        }
    }

    /// It dies of its burning (`unit_flame_to_death`): it stops running
    /// blindly, can be hurt again and is dealt the game's flaming death,
    /// as its attacker's if it is still there. If that leaves it alive,
    /// it dies by an act of god.
    pub fn flame_to_death(&mut self, world: &mut impl FlameWorld) {
        self.flags.stop_running_blindly();
        self.flags.set(UnitFlags::IGNORE_HARD_PINGS, false);
        self.damage_flags
            .set(DamageFlags::CANNOT_TAKE_DAMAGE, false);
        if let Some(effect) = world.flaming_death_effect() {
            let mut owners = Owners::default();
            if let Some(attacker) = self.attacker {
                if let Some(theirs) = world.owners(attacker) {
                    owners = Owners {
                        object: theirs.object.or(Some(attacker)),
                        ..theirs
                    };
                }
            }
            world.cause_damage(FlameDamage { effect, owners }, &mut self.damage_flags);
        }
        if !self.damage_flags.has(DamageFlags::DEAD) {
            self.damage_flags.set(DamageFlags::DIE_ACT_OF_GOD, true);
        }
    }
}

impl RunBlind {
    /// The way a unit running blindly runs this tick (`unit_running_blind`):
    /// its actor's way (`actor_way`) or forward, turned by its angle, its
    /// turning turned at random within what keeps it within 45 degrees of
    /// its actor's way and turning no faster than 12 degrees a tick.
    pub fn run(&mut self, actor_way: Option<Vec3>, random: &mut RandomSeed) -> Vec3 {
        let way = actor_way.unwrap_or(Vec3::X);
        let mut positive_allowed: f32 = 1.0;
        let mut negative_allowed: f32 = 1.0;
        if actor_way.is_some() {
            let negative_room = degrees(45.0) - self.angle;
            let positive_room = degrees(45.0) + self.angle;
            negative_allowed = min(negative_allowed, negative_room / degrees(13.5));
            positive_allowed = min(positive_allowed, positive_room / degrees(13.5));
        }
        let negative_room = degrees(12.0) - self.delta;
        let positive_room = degrees(12.0) + self.delta;
        negative_allowed = min(negative_allowed, negative_room * TURN_ROOM);
        positive_allowed = min(positive_allowed, positive_room * TURN_ROOM);
        let acceleration = if negative_allowed < positive_allowed {
            if negative_allowed < -1.0 {
                -TURN_ACCELERATION
            } else {
                let allowed = min(1.0, negative_allowed);
                random.next_real_range(-TURN_ACCELERATION, TURN_ACCELERATION * allowed)
            }
        } else if positive_allowed < -1.0 {
            TURN_ACCELERATION
        } else {
            let allowed = min(1.0, positive_allowed);
            random.next_real_range(-TURN_ACCELERATION * allowed, TURN_ACCELERATION)
        };
        self.delta += acceleration;
        self.angle += self.delta;
        if self.angle < -PI {
            self.angle += 2.0 * PI;
        } else if self.angle > PI {
            self.angle -= 2.0 * PI;
        }
        rotate_about_axis(way, Vec3::Z, sine(self.angle), cosine(self.angle))
    }
}

/// The game's `MIN`: `a` unless it is more than `b` (so a NaN gives `a`).
fn min(a: f32, b: f32) -> f32 {
    if a > b {
        b
    } else {
        a
    }
}
