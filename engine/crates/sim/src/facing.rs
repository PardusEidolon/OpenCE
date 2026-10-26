//! Where a player on foot looks, turned by the mouse as the game turns it
//! (`player_control_modify_desired_angles`): the yaw kept within a turn,
//! from zero up to a whole turn, and the pitch within 85.5 degrees of level.
//!
//! The game eases a seat's or a unit camera's own limits in at a
//! 256th of a half turn a tick and levels the look of a moving vehicle;
//! a biped's camera has no limits of its own, so on foot the limits are
//! the 85.5 degrees from the start.

use std::f32::consts::PI;

/// The steepest a player looks up or down from level, in radians.
pub const MAX_PITCH: f32 = 85.5 * PI / 180.0;

/// A whole turn, as the game writes it (`_pi * 2.f`).
const TURN: f32 = PI * 2.0;

/// The most whole turns a yaw is brought back by a turn at a time, as the
/// game does; past it (a yaw no mouse turns to) the remainder is taken at
/// once.
const MOST_TURNS: f32 = 64.0;

/// `yaw` brought within `0.0..=TURN` a turn at a time, as the game brings
/// it; a yaw that is not a finite number is zero.
pub fn wrap_yaw(yaw: f32) -> f32 {
    if !yaw.is_finite() {
        return 0.0;
    }
    let mut yaw = if yaw.abs() > MOST_TURNS * TURN {
        yaw.rem_euclid(TURN)
    } else {
        yaw
    };
    while yaw < 0.0 {
        yaw += TURN;
    }
    while yaw > TURN {
        yaw -= TURN;
    }
    yaw
}

/// The look `(yaw, pitch)` turned by `delta_yaw` and `delta_pitch`: the yaw
/// first, brought within a turn, then the pitch, held within
/// [`MAX_PITCH`]. A delta that is not a finite number does not turn it.
pub fn turn(yaw: f32, pitch: f32, delta_yaw: f32, delta_pitch: f32) -> (f32, f32) {
    let delta = |d: f32| if d.is_finite() { d } else { 0.0 };
    let yaw = wrap_yaw(yaw + delta(delta_yaw));
    let pitch = pitch + delta(delta_pitch);
    // (as the game's PIN, a NaN stays a NaN)
    let pitch = pitch.clamp(-MAX_PITCH, MAX_PITCH);
    (yaw, pitch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_yaw_is_kept_within_a_turn_from_zero() {
        assert_eq!(wrap_yaw(-0.5), -0.5 + TURN);
        assert_eq!(wrap_yaw(TURN + 0.25), TURN + 0.25 - TURN);
        assert_eq!(wrap_yaw(TURN), TURN);
        assert_eq!(wrap_yaw(0.0), 0.0);
    }

    #[test]
    fn the_pitch_stops_at_eighty_five_and_a_half_degrees() {
        assert_eq!(turn(0.0, 1.4, 0.0, 0.5).1, MAX_PITCH);
        assert_eq!(turn(0.0, -1.4, 0.0, -0.5).1, -MAX_PITCH);
        assert!((MAX_PITCH.to_degrees() - 85.5).abs() < 1e-4);
    }

    #[test]
    fn nonsense_neither_turns_nor_hangs() {
        assert_eq!(turn(1.0, 0.5, f32::NAN, f32::INFINITY), (1.0, 0.5));
        assert_eq!(wrap_yaw(f32::INFINITY), 0.0);
        let far = wrap_yaw(1.0e30);
        assert!((0.0..=TURN).contains(&far));
    }
}
