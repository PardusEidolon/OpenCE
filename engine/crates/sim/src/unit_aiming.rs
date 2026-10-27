//! The planner a unit turns its aim and look with (units.c's euler aiming):
//! each euler axis's error from where it is wished to point, with the
//! axis's speed, is planned as an acceleration, a coast and a deceleration
//! that bring it to rest on the wish within the unit's limits, the two
//! axes' plans are coupled so that they end together, and each tick the
//! plan is followed for a tick.
//!
//! With them are the unit's aiming and looking vectors: the questions of
//! them, the check that they are all unit normals, and their clip to the
//! screens an animation lets the unit aim and look in.
//!
//! Each is checked against the game's code by
//! tests/unit_aiming_plan_reference.rs and
//! tests/unit_aiming_bounds_reference.rs. The arithmetic is the game's, in
//! its order and in floats, so that a plan is the same float as the game's.

use crate::animation_blend;

/// An axis's plan (`struct unit_acceleration_plan`): from its error and
/// speed, an acceleration for a time, a coast and a deceleration for a time.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AccelerationPlan {
    /// The axis is on its wish and still, and needs no plan (`at_rest`).
    pub at_rest: bool,
    /// The error the plan starts from (`initial_position`).
    pub initial_position: f32,
    /// The speed the plan starts from (`initial_velocity`).
    pub initial_velocity: f32,
    /// The first part's acceleration (`acceleration`).
    pub acceleration: f32,
    /// The first part's time, in ticks (`acceleration_time`).
    pub acceleration_time: f32,
    /// The coast's time between them, in ticks (`coast_time`).
    pub coast_time: f32,
    /// The last part's acceleration (`deceleration`).
    pub deceleration: f32,
    /// The last part's time, in ticks (`deceleration_time`).
    pub deceleration_time: f32,
}

/// The game's `MIN`: `a` when it is less, else `b`, NaN included.
fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

/// Whether time is left: more than none, which a NaN is not.
fn positive(time: f32) -> bool {
    time > 0.0
}

impl AccelerationPlan {
    /// Plans an axis from its error `position` and speed `velocity` to
    /// rest at none, at most `maximum_acceleration` and, when it is
    /// positive, at most `maximum_velocity` (`unit_euler_axis_buildplan`).
    ///
    /// An error and speed under 0.001 are at rest. A speed that carries the
    /// axis past its wish before it could stop is planned as its mirror and
    /// mirrored back; one too slow to reach it is a deceleration alone.
    pub fn build(
        position: f32,
        velocity: f32,
        maximum_velocity: f32,
        maximum_acceleration: f32,
    ) -> Self {
        let mut plan = Self {
            at_rest: position.abs() < 0.001 && velocity.abs() < 0.001,
            initial_position: position,
            initial_velocity: velocity,
            ..Self::default()
        };
        if plan.at_rest {
            return plan;
        }

        let time = velocity.abs() / maximum_acceleration;
        let moving_positive = velocity > 0.0;
        let half_time = time * 0.5;
        if half_time * velocity * 0.5 + position < 0.0 {
            let mut plan =
                Self::build(-position, -velocity, maximum_velocity, maximum_acceleration);
            plan.initial_position *= -1.0;
            plan.initial_velocity *= -1.0;
            plan.acceleration *= -1.0;
            plan.deceleration *= -1.0;
            return plan;
        }

        let stopping_position = velocity * 0.5 * time + position;
        if stopping_position < 0.0 {
            plan.deceleration = plan.initial_velocity * plan.initial_velocity
                / (plan.initial_position + plan.initial_position);
            plan.deceleration_time = -(plan.initial_velocity / plan.deceleration);
            return plan;
        }

        let acceleration_time = if moving_positive {
            libm::sqrtf(stopping_position / maximum_acceleration)
        } else {
            let negative_acceleration = -maximum_acceleration;
            let velocity = velocity + velocity;
            let discriminant =
                velocity * velocity - negative_acceleration * stopping_position * 4.0;
            let second_root = libm::sqrtf(discriminant);
            let first_root =
                (-velocity - second_root) / (negative_acceleration + negative_acceleration);
            let second_time =
                (second_root - velocity) / (negative_acceleration + negative_acceleration);
            if first_root >= 0.0 && (second_time < 0.0 || first_root < second_time) {
                first_root
            } else if 0.0 > second_time {
                0.0
            } else {
                second_time
            }
        };

        // the time it accelerates for, held to the speed limit
        let actual_time = if maximum_velocity > 0.0 {
            let speed = if moving_positive {
                maximum_velocity
            } else {
                maximum_velocity + plan.initial_velocity
            };
            let maximum_velocity_time = speed / maximum_acceleration;
            // the game's MAX(0, t), which keeps a NaN
            let maximum_velocity_time = if 0.0 > maximum_velocity_time {
                0.0
            } else {
                maximum_velocity_time
            };
            min(acceleration_time, maximum_velocity_time)
        } else {
            acceleration_time
        };

        plan.acceleration = -maximum_acceleration;
        plan.deceleration = maximum_acceleration;
        if moving_positive {
            plan.acceleration_time = actual_time + time;
            plan.deceleration_time = actual_time;
        } else {
            plan.acceleration_time = actual_time;
            plan.deceleration_time = actual_time + time;
        }

        if actual_time < acceleration_time {
            let coasting_velocity =
                plan.acceleration * plan.acceleration_time + plan.initial_velocity;
            let cut = acceleration_time - actual_time;
            let distance = cut * coasting_velocity * 2.0 - cut * cut * maximum_acceleration;
            plan.coast_time = distance / coasting_velocity;
        }
        plan
    }

    /// Follows the plan for `delta_time` ticks from error `position` and
    /// speed `velocity`, answering whether it is complete, with the error
    /// and speed it leaves; a plan at rest, or no time, leaves them as they
    /// are (`unit_euler_axis_doplan`).
    pub fn follow(&self, delta_time: f32, position: f32, velocity: f32) -> (bool, f32, f32) {
        let mut position = position;
        let mut velocity = velocity;
        if self.at_rest {
            return (true, position, velocity);
        }
        let mut remaining = delta_time;
        if !positive(remaining) {
            return (false, position, velocity);
        }
        if self.acceleration_time > 0.0 {
            let time = min(remaining, self.acceleration_time);
            position += (time * self.acceleration * 0.5 + velocity) * time;
            velocity += time * self.acceleration;
            remaining -= time;
        }
        if !positive(remaining) {
            return (false, position, velocity);
        }
        if self.coast_time > 0.0 {
            let time = min(remaining, self.coast_time);
            position += velocity * time;
            remaining -= time;
        }
        if !positive(remaining) {
            return (false, position, velocity);
        }
        if self.deceleration_time > 0.0 {
            let time = min(remaining, self.deceleration_time);
            position += (0.5 * time * self.deceleration + velocity) * time;
            velocity += time * self.deceleration;
            remaining -= time;
        }
        (positive(remaining), position, velocity)
    }

    /// The plan's whole time: its deceleration's, its coast's and its
    /// acceleration's, in the game's order.
    fn total_time(&self) -> f32 {
        self.deceleration_time + self.coast_time + self.acceleration_time
    }
}

/// Couples two axes' plans so that they end together: the one that ends
/// first, when it accelerates, accelerates and decelerates for less and
/// coasts for longer (`unit_euler_axis_couple`, which is also given the
/// speed limit and asks it nothing).
pub fn couple(
    first: &mut AccelerationPlan,
    second: &mut AccelerationPlan,
    maximum_acceleration: f32,
) {
    if first.at_rest || second.at_rest {
        return;
    }
    let first_total = first.total_time();
    let second_total = second.total_time();
    let (plan, extension) = if first.acceleration_time > 0.0 && first_total < second_total {
        (first, second_total - first_total)
    } else if second.acceleration_time > 0.0 && second_total < first_total {
        (second, first_total - second_total)
    } else {
        return;
    };

    let time_factor = (extension + plan.coast_time) * maximum_acceleration;
    let peak_velocity = plan.acceleration_time * plan.acceleration + plan.initial_velocity;
    let discriminant =
        time_factor * time_factor - (-extension * peak_velocity.abs()) * maximum_acceleration * 4.0;
    let adjustment =
        (libm::sqrtf(discriminant) - time_factor) / (maximum_acceleration + maximum_acceleration);
    let adjustment = min(
        adjustment,
        min(plan.acceleration_time, plan.deceleration_time),
    );
    if !positive(adjustment) {
        return;
    }

    let acceleration_time = plan.acceleration_time - adjustment;
    let peak_velocity = acceleration_time * plan.acceleration + plan.initial_velocity;
    plan.acceleration_time = acceleration_time;
    plan.deceleration_time -= adjustment;
    plan.coast_time = (peak_velocity + peak_velocity + adjustment * plan.acceleration) * adjustment
        / peak_velocity;
}

/// A unit's frame and the vectors it faces, aims and looks along, as
/// `struct unit_datum` holds them.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UnitAim {
    /// Its object's forward (`object.forward`).
    pub forward: [f32; 3],
    /// Its object's up (`object.up`).
    pub up: [f32; 3],
    /// Where it wishes to face (`desired_facing_vector`).
    pub desired_facing: [f32; 3],
    /// Where it wishes to aim (`desired_aiming_vector`).
    pub desired_aiming: [f32; 3],
    /// Where it wishes to look (`desired_looking_vector`).
    pub desired_looking: [f32; 3],
    /// Where it aims (`aiming_vector`).
    pub aiming: [f32; 3],
    /// Where it looks (`looking_vector`).
    pub looking: [f32; 3],
}

impl UnitAim {
    /// Where it faces: its forward (`unit_get_facing_vector`).
    pub fn facing(&self) -> [f32; 3] {
        self.forward
    }

    /// Where it aims (`unit_get_aiming_vector`).
    pub fn aiming(&self) -> [f32; 3] {
        self.aiming
    }

    /// Where it looks (`unit_get_looking_vector`).
    pub fn looking(&self) -> [f32; 3] {
        self.looking
    }

    /// Whether its wishes, its aim and its look are unit normals and its
    /// forward and up a frame (`unit_vectors_are_valid`).
    pub fn vectors_are_valid(&self) -> bool {
        valid_normal(self.desired_facing)
            && valid_normal(self.desired_aiming)
            && valid_normal(self.desired_looking)
            && valid_axes(self.forward, self.up)
            && valid_normal(self.aiming)
            && valid_normal(self.looking)
    }

    /// Faces it along `alignment` on the level, with the world's up, unless
    /// it is attached to a parent (`unit_align_facing`).
    pub fn align_facing(&mut self, attached: bool, alignment: [f32; 2]) {
        if !attached {
            self.forward = [alignment[0], alignment[1], 0.0];
            self.up = [0.0, 0.0, 1.0];
        }
    }
}

/// Whether `a` is within a thousandth of `b` (`valid_realcmp`).
fn valid_compare(a: f32, b: f32) -> bool {
    let difference = a - b;
    difference.is_finite() && difference.abs() < 0.001
}

/// Whether `n` is a unit normal (`valid_real_normal3d`).
fn valid_normal(n: [f32; 3]) -> bool {
    valid_compare(n[0] * n[0] + n[1] * n[1] + n[2] * n[2], 1.0)
}

/// Whether `forward` and `up` are unit normals at right angles
/// (`valid_real_vector3d_axes2`).
fn valid_axes(forward: [f32; 3], up: [f32; 3]) -> bool {
    valid_normal(forward)
        && valid_normal(up)
        && valid_compare(
            forward[0] * up[0] + forward[1] * up[1] + forward[2] * up[2],
            0.0,
        )
}

/// `vector` clipped to a screen of yaw and pitch in the unit's frame,
/// `[x0, x1, y0, y1]` (`unit_clip_to_aiming_bounds`): none when there is no
/// screen or the vector is within it, else the vector at the screen's edge.
/// The yaw is clamped before the pitch, and the clipped vector is rebuilt
/// from its angles, as the game's is.
pub fn clip_to_aiming_bounds(
    forward: [f32; 3],
    up: [f32; 3],
    screen: Option<[f32; 4]>,
    vector: [f32; 3],
) -> Option<[f32; 3]> {
    let [x0, x1, y0, y1] = screen?;
    let (mut yaw, mut pitch) = animation_blend::relative_angles(forward, up, vector);
    let mut clipped = false;
    if yaw < x0 {
        yaw = x0;
        clipped = true;
    } else if yaw > x1 {
        yaw = x1;
        clipped = true;
    }
    if pitch < y0 {
        pitch = y0;
        clipped = true;
    } else if pitch > y1 {
        pitch = y1;
        clipped = true;
    }
    if !clipped {
        return None;
    }
    // (the game's `sine` and `cosine` take the double's and round it)
    let sine = |angle: f32| libm::sin(f64::from(angle)) as f32;
    let cosine = |angle: f32| libm::cos(f64::from(angle)) as f32;
    let cosine_pitch = cosine(pitch);
    let (i, j, k) = (
        cosine(yaw) * cosine_pitch,
        sine(yaw) * cosine_pitch,
        sine(pitch),
    );
    let left = [
        up[1] * forward[2] - up[2] * forward[1],
        up[2] * forward[0] - up[0] * forward[2],
        up[0] * forward[1] - up[1] * forward[0],
    ];
    Some(std::array::from_fn(|axis| {
        i * forward[axis] + j * left[axis] + k * up[axis]
    }))
}

/// The unit state a script's custom animation plays in
/// (`_unit_state_user_animation`).
pub const STATE_USER_ANIMATION: i16 = 28;

/// Whether a unit plays a script's custom animation: its state, none of no
/// unit (`unit_is_playing_custom_animation`).
pub fn is_playing_custom_animation(state: Option<i16>) -> bool {
    state == Some(STATE_USER_ANIMATION)
}
