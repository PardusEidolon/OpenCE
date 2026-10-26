//! The game's physics variables (source/physics/physics_variables.c): a
//! speed that gains or loses toward a limit, and a position between two
//! limits that it moves, clamped or wrapped round, as the vehicles' speeds,
//! slides and turns are.

/// A speed's limits and how fast it changes
/// (`physics_variable_speed_parameters`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpeedParameters {
    /// The fastest forward, as a share of the change's size.
    pub positive: f32,
    /// The fastest backward.
    pub negative: f32,
    /// What it gains a step while it speeds up.
    pub acceleration: f32,
    /// What it loses a step while it slows.
    pub deceleration: f32,
}

/// A position's limits, high then low, and its speed's parameters, as the
/// game lays them out for `physics_variable_update`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Variable {
    /// The highest and the lowest position.
    pub limits: [f32; 2],
    /// How its speed changes.
    pub speed: SpeedParameters,
}

/// A speed sped up toward `delta`'s side by its size
/// (`physics_variable_speed_update`): it slows first when moving the other
/// way, and stops at its limit.
pub fn speed_update(speed: &mut f32, parameters: SpeedParameters, delta: f32) {
    let magnitude = delta.abs();
    let acceleration = magnitude * parameters.acceleration;
    let deceleration = magnitude * parameters.deceleration;
    if delta > 0.0 {
        if *speed <= -deceleration {
            *speed += deceleration;
        } else if *speed >= 0.0 {
            *speed += acceleration;
        } else {
            *speed = (*speed / deceleration + 1.0) * acceleration;
        }
        // (`MIN`, which keeps the limit when the speed is not a number)
        let limit = magnitude * parameters.positive;
        *speed = if *speed < limit { *speed } else { limit };
    } else if delta < 0.0 {
        if *speed >= deceleration {
            *speed -= deceleration;
        } else if *speed <= 0.0 {
            *speed -= acceleration;
        } else {
            *speed = (*speed / deceleration - 1.0) * acceleration;
        }
        let limit = -magnitude * parameters.negative;
        *speed = if limit > *speed { limit } else { *speed };
    }
}

/// A speed moved toward `target`, stopping there
/// (`physics_variable_speed_update_seek`); whether it is there.
pub fn speed_update_seek(
    speed: &mut f32,
    parameters: SpeedParameters,
    target: f32,
    delta: f32,
) -> bool {
    if *speed > target {
        speed_update(speed, parameters, -delta);
        if *speed <= target {
            *speed = target;
            return true;
        }
        false
    } else if *speed < target {
        speed_update(speed, parameters, delta);
        if *speed >= target {
            *speed = target;
            return true;
        }
        false
    } else {
        true
    }
}

/// A position moved by `delta`, clamped to its limits or wrapped round
/// them (`physics_variable_position_update`).
pub fn position_update(position: &mut f32, limits: [f32; 2], wrap: bool, delta: f32) {
    *position += delta;
    if *position < limits[1] {
        if wrap {
            *position += limits[0] - limits[1];
        } else {
            *position = limits[1];
        }
    } else if *position > limits[0] {
        if wrap {
            *position -= limits[0] - limits[1];
        } else {
            *position = limits[0];
        }
    }
}

/// The way from `position` to `target`, 1, -1 or 0, the short way round
/// when it wraps (`physics_variable_position_get_seek_direction`).
pub fn seek_direction(limits: [f32; 2], position: f32, wrap: bool, target: f32) -> f32 {
    let mut direction = target - position;
    if direction != 0.0 {
        if wrap && direction.abs() > (limits[0] - limits[1]) * 0.5 {
            direction = -direction;
        }
        direction = if direction > 0.0 { 1.0 } else { -1.0 };
    }
    direction
}

/// A position moved toward `target` by `delta`, stopping there
/// (`physics_variable_position_update_seek`); whether it is there.
pub fn position_update_seek(
    position: &mut f32,
    limits: [f32; 2],
    wrap: bool,
    target: f32,
    delta: f32,
) -> bool {
    let direction = seek_direction(limits, *position, wrap, target);
    if direction != 0.0 {
        position_update(position, limits, wrap, direction * delta);
        if seek_direction(limits, *position, wrap, target) == direction {
            return false;
        }
    }
    *position = target;
    true
}

/// A position moved by its speed, the speed first sped up by `delta`
/// (`physics_variable_update`).
pub fn update(position: &mut f32, speed: &mut f32, variable: Variable, wrap: bool, delta: f32) {
    speed_update(speed, variable.speed, delta);
    position_update(position, variable.limits, wrap, *speed);
}

/// A position moved by its speed toward `target`, stopping there with no
/// speed left (`physics_variable_update_seek`); whether it is there.
pub fn update_seek(
    position: &mut f32,
    speed: &mut f32,
    variable: Variable,
    wrap: bool,
    target: f32,
    delta: f32,
) -> bool {
    let direction = seek_direction(variable.limits, *position, wrap, target);
    if direction != 0.0 {
        speed_update(speed, variable.speed, direction * delta);
        position_update(position, variable.limits, wrap, *speed);
        if seek_direction(variable.limits, *position, wrap, target) == direction {
            return false;
        }
    }
    *position = target;
    *speed = 0.0;
    true
}
