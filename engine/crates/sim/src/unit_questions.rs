//! The game's questions of its units, bipeds and vehicles: which unit aims
//! for a rider, whether a unit is dangerous or unsuspecting, the markers of
//! its head and body, the frames left of its animation, an animation's foot
//! frames, a biped's sight position and its limp body stopped, and whether
//! a vehicle moves near a player (units.c, bipeds.c and vehicles.c).
//!
//! The vector sums are written out term by term, as the game's are, so
//! that they round as the game's do.
//!
//! Each is checked against the game's code by
//! tests/unit_questions_reference.rs.

use glam::Vec3;

use crate::unit_definition::BASE_SEAT_LABELS;
use crate::unit_states::UnitState;

/// A seat whose rider is not drawn (`_unit_seat_invisible_bit`).
pub const SEAT_INVISIBLE: u32 = 1 << 0;
/// A seat whose rider aims the unit's gun (`_unit_seat_gunner_bit`).
pub const SEAT_GUNNER: u32 = 1 << 3;

/// The unit that aims for `unit`: the unit it rides, if its seat is
/// invisible or a gunner's, otherwise itself (`unit_get_aiming_unit_index`).
/// `riding` is the unit it rides and its seat's flags.
pub fn aiming_unit(unit: u32, riding: Option<(u32, u32)>) -> u32 {
    match riding {
        Some((parent, seat_flags)) if seat_flags & (SEAT_INVISIBLE | SEAT_GUNNER) != 0 => parent,
        _ => unit,
    }
}

/// The last stage of a grenade's throw (`_unit_grenade_throw_ending`).
pub const GRENADE_THROW_ENDING: i8 = 3;
/// An animation that moves the unit by none of its own translation
/// (`_unit_animation_ignore_translation_bit`).
pub const ANIMATION_IGNORE_TRANSLATION: u16 = 1 << 2;

/// Whether a unit is dangerous to be near: throwing a grenade not yet at
/// its end, or dying with its animation's translation
/// (`any_unit_is_dangerous`, for one unit).
pub fn is_dangerous(
    state: Option<UnitState>,
    grenade_throw_state: i8,
    animation_flags: u16,
) -> bool {
    match state {
        Some(UnitState::ThrowGrenade) => grenade_throw_state != GRENADE_THROW_ENDING,
        Some(UnitState::Dying | UnitState::DyingAirborne) => {
            animation_flags & ANIMATION_IGNORE_TRANSLATION == 0
        }
        _ => false,
    }
}

/// A sum of products term by term, in the game's order.
fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// A unit definition whose units are never unsuspecting
/// (`_unit_never_unsuspecting_bit`).
pub const UNIT_NEVER_UNSUSPECTING: u32 = 1 << 16;

/// What whether a biped is unsuspecting depends on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Suspicion<'a> {
    /// Its definition is never unsuspecting (`_unit_never_unsuspecting_bit`).
    pub never_unsuspecting: bool,
    /// The centre of its bounding sphere.
    pub center: Vec3,
    /// The way it looks.
    pub looking: Vec3,
    /// Its seat label (`unit_get_seat_label`).
    pub seat_label: &'a str,
}

/// Whether a unit is unsuspecting of `point`: only a biped can be, if its
/// definition allows, when it looks away from the point or sleeps
/// (`unit_unsuspecting`). `biped` is none for no unit or one not a biped.
pub fn unsuspecting(biped: Option<&Suspicion<'_>>, point: Vec3) -> bool {
    biped.is_some_and(|biped| {
        !biped.never_unsuspecting
            && (dot(biped.center - point, biped.looking) > 0.0
                || biped.seat_label == BASE_SEAT_LABELS[0])
    })
}

/// The marker a unit's head is at (`unit_get_head_position`).
pub const HEAD_MARKER: &str = "head";
/// The marker a unit's centre of mass is at (`unit_get_center_of_mass`).
pub const BODY_MARKER: &str = "body";

/// The frames left of a unit's animation, of `frame_count`, at
/// `frame_index`, in the game's short (`unit_get_animation_frames_remaining`).
pub fn animation_frames_remaining(frame_count: i16, frame_index: i16) -> i16 {
    frame_count.wrapping_sub(frame_index)
}

/// An animation's foot frame as the game returns it: its byte, as a signed
/// char (`animation_left_foot_frame_index`, `animation_right_foot_frame_index`).
pub fn foot_frame_index(frame: u8) -> i8 {
    i8::from_le_bytes([frame])
}

/// How a biped's sight position is estimated (`_unit_estimate_*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Estimate {
    /// From where it is, at its own crouch.
    None,
    /// From a body position, standing.
    HeadStanding,
    /// From a body position, crouching.
    HeadCrouching,
    /// From a body position, at the gun's offset along a facing.
    GunPosition,
}

impl Estimate {
    /// The estimate the game numbers `index`.
    pub fn from_index(index: usize) -> Option<Self> {
        [
            Self::None,
            Self::HeadStanding,
            Self::HeadCrouching,
            Self::GunPosition,
        ]
        .get(index)
        .copied()
    }
}

/// What a biped's sight position depends on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BipedSight {
    /// Its origin.
    pub origin: Vec3,
    /// How far it crouches, from 0 to 1.
    pub crouch: f32,
    /// Its definition's camera height standing.
    pub standing_camera_height: f32,
    /// Its definition's camera height crouching.
    pub crouching_camera_height: f32,
}

impl BipedSight {
    /// Where the biped sees from (`biped_get_sight_position`): its origin
    /// or `estimated`, moved by the gun's offset along `facing` and to its
    /// left, or raised to its camera's height at the estimate's crouch.
    pub fn position(
        &self,
        mode: Estimate,
        estimated: Vec3,
        facing: Vec3,
        gun_offset: Vec3,
    ) -> Vec3 {
        let mut sight = if mode == Estimate::None {
            self.origin
        } else {
            estimated
        };
        if mode == Estimate::GunPosition {
            let left = Vec3::new(-facing.y, facing.x, 0.0);
            let forward = gun_offset.x;
            sight.x += facing.x * forward;
            sight.y += facing.y * forward;
            sight.z += facing.z * forward;
            let sideways = gun_offset.y;
            sight.x += left.x * sideways;
            sight.y += left.y * sideways;
            sight.z += left.z * sideways;
            sight.z += gun_offset.z;
        } else {
            let crouch = match mode {
                Estimate::HeadStanding => 0.0,
                Estimate::HeadCrouching => 1.0,
                _ => self.crouch,
            };
            sight.z += (1.0 - crouch) * self.standing_camera_height
                + crouch * self.crouching_camera_height;
        }
        sight
    }
}

/// A biped definition whose bodies go limp (`_biped_uses_limp_body_physics_bit`).
pub const BIPED_USES_LIMP_BODY_PHYSICS: u32 = 1 << 9;
/// A biped whose body is limp now (`_biped_limp_body_physics_active_bit`).
pub const BIPED_LIMP_BODY_PHYSICS_ACTIVE: u32 = 1 << 5;
/// An object whose node matrices are left as they are
/// (`_object_do_not_recompute_node_matrices_bit`).
pub const OBJECT_DO_NOT_RECOMPUTE_NODE_MATRICES: u32 = 1 << 23;

/// Stops a biped's limp body: if its definition's bodies go limp and its
/// is, its nodes are computed again and it is limp no longer
/// (`biped_stop_limp_body_physics`).
pub fn stop_limp_body_physics(
    definition_flags: u32,
    biped_flags: &mut u32,
    object_flags: &mut u32,
) {
    if definition_flags & BIPED_USES_LIMP_BODY_PHYSICS != 0
        && *biped_flags & BIPED_LIMP_BODY_PHYSICS_ACTIVE != 0
    {
        *object_flags &= !OBJECT_DO_NOT_RECOMPUTE_NODE_MATRICES;
        *biped_flags &= !BIPED_LIMP_BODY_PHYSICS_ACTIVE;
    }
}

/// The dot product the vehicles' code tests with (`vehicle_dot_product3d_test`).
pub fn vehicle_dot_product(a: Vec3, b: Vec3) -> f32 {
    dot(a, b)
}

/// The squared speed a vehicle moves at, at least, to be moving: a
/// thirtieth of a world unit a frame (`vehicle_moving_near_any_player`).
pub const MOVING_SPEED_SQUARED: f32 = ((1.0 / 30.0) * (1.0 / 30.0)) as f32;
/// The squared distance a vehicle is near a player within.
pub const NEAR_DISTANCE_SQUARED: f32 = 100.0;

/// Whether a vehicle moves near any local player's unit
/// (`vehicle_moving_near_any_player`): `players` are the centres of the
/// local players' units that ride nothing, `vehicles` each vehicle's
/// centre and velocity, in the world's order. A unit that rides nothing
/// rides no vehicle, so the game's test that the unit is not in the
/// vehicle always passes.
pub fn vehicle_moving_near_any_player(
    players: &[Vec3],
    vehicles: impl IntoIterator<Item = (Vec3, Vec3)>,
) -> bool {
    if players.is_empty() {
        return false;
    }
    vehicles.into_iter().any(|(center, velocity)| {
        players.iter().any(|&player| {
            let offset = center - player;
            dot(offset, offset) < NEAR_DISTANCE_SQUARED
                && dot(velocity, velocity) >= MOVING_SPEED_SQUARED
        })
    })
}
