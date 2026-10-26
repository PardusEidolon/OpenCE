//! The dead player's camera (`dead_camera.c`, through `observer.c`): from
//! where the player last saw, it looks at its body's bounding sphere from
//! half a unit away, then eases out over three seconds to a distance and a
//! look down at it drawn at random, which it keeps for as long as the
//! player is dead.
//!
//! The original commands its observer each frame, and the observer moves
//! each parameter to its command along a quintic over the command's time
//! left (`observer_update_polynomial`). Commanded from rest to a target
//! that stays put, that curve is the minimum-jerk one, `10u³ - 15u⁴ +
//! 6u⁵` of the way at `u` of the time, which is what is used here.

use glam::{Quat, Vec3};

use crate::random::RandomSeed;
use crate::{clock, Transform};

/// How long it eases out, in seconds (`dead_camera_constants.dead_timer`).
pub(crate) const DEAD_TIMER: f32 = 3.0;

/// How far it starts from the body (the first command's `depth`).
const START_DEPTH: f32 = 0.5;

/// The nearest and furthest it may end from the body.
const DISTANCE: (f32, f32) = (2.0, 6.0);

/// The least and most it may end looking down, in radians (27 and 63
/// degrees).
const PITCH: (f32, f32) = (0.471_238_94, 1.099_557_4);

/// A dead player's camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DeadCamera {
    /// The way the camera looked as the player died.
    start: Vec3,
    /// The way it ends looking.
    forward: Vec3,
    /// How far it ends from the body.
    distance: f32,
    /// The seconds since the player died.
    elapsed: f32,
}

impl DeadCamera {
    /// The camera of a player who died seeing from `camera`, drawing its
    /// distance and look from `random` (`dead_camera_new`).
    pub(crate) fn new(camera: &Transform, random: &mut RandomSeed) -> Self {
        let distance = random.next_real_range(DISTANCE.0, DISTANCE.1);
        let yaw = random.next_real_range(0.0, std::f32::consts::TAU);
        let pitch = -random.next_real_range(PITCH.0, PITCH.1);
        Self {
            start: camera.rotation * Vec3::X,
            forward: euler_forward(yaw, pitch),
            distance,
            elapsed: 0.0,
        }
    }

    /// The camera looking at `focus`, stopped short of what `reach` says
    /// stands between: the fraction of the way back from `focus` it may
    /// go.
    pub(crate) fn place(&self, focus: Vec3, reach: impl FnOnce(Vec3, Vec3) -> f32) -> Transform {
        let along = minimum_jerk(self.elapsed / DEAD_TIMER);
        let forward = (self.start + (self.forward - self.start) * along)
            .try_normalize()
            .unwrap_or(self.forward);
        let depth = START_DEPTH + (self.distance - START_DEPTH) * along;
        let back = -forward * depth;
        Transform {
            position: focus + back * reach(focus, back),
            rotation: look_along(forward),
            scale: 1.0,
        }
    }

    /// A tick on.
    pub(crate) fn step(&mut self) {
        self.elapsed = (self.elapsed + clock::TICK.0).min(DEAD_TIMER);
    }
}

/// The way along a minimum-jerk move at `u` of its time.
fn minimum_jerk(u: f32) -> f32 {
    let u = u.clamp(0.0, 1.0);
    u * u * u * (10.0 + u * (-15.0 + u * 6.0))
}

/// The way `yaw` and `pitch` look (`vector3d_from_euler_angles2d`).
fn euler_forward(yaw: f32, pitch: f32) -> Vec3 {
    let (sine, cosine) = (libm::sinf(pitch), libm::cosf(pitch));
    Vec3::new(libm::cosf(yaw) * cosine, libm::sinf(yaw) * cosine, sine)
}

/// The turn that looks along `forward`, its up as near the world's as it
/// can be (`observer_up_from_forward`).
fn look_along(forward: Vec3) -> Quat {
    let yaw = libm::atan2f(forward.y, forward.x);
    let pitch = libm::asinf(forward.z.clamp(-1.0, 1.0));
    Quat::from_rotation_z(yaw) * Quat::from_rotation_y(-pitch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_eases_from_rest_to_rest() {
        assert_eq!(minimum_jerk(0.0), 0.0);
        assert!((minimum_jerk(0.5) - 0.5).abs() < 1e-6);
        assert!((minimum_jerk(1.0) - 1.0).abs() < 1e-6);
        // (no speed at either end)
        assert!(minimum_jerk(0.01) < 1e-4);
        assert!(1.0 - minimum_jerk(0.99) < 1e-4);
    }

    #[test]
    fn it_looks_as_it_was_turned() {
        let forward = euler_forward(1.0, -0.8);
        let turned = look_along(forward) * Vec3::X;
        assert!(turned.distance(forward) < 1e-5, "{turned} {forward}");
    }
}
