//! The vector helpers of source/math/real_math.h that glam has no exact
//! primitive for, ported as the game writes them. (glam's
//! `clamp_length_max` scales by `max / length` after dividing, and rounds
//! differently from `limit2d` for one vector in four.)

use glam::{Vec2, Vec3};

/// The original's `limit2d`: `vector` shortened to `length` when it is
/// longer, and whether it was.
pub fn limit2d(vector: Vec2, length: f32) -> (bool, Vec2) {
    let dot = vector.x * vector.x + vector.y * vector.y;
    if dot <= length * length {
        return (false, vector);
    }
    (true, vector * (length / libm::sqrtf(dot)))
}

/// The original's `limit3d`: `vector` shortened to `length` when it is
/// longer, and whether it was.
pub fn limit3d(vector: Vec3, length: f32) -> (bool, Vec3) {
    let dot = vector.x * vector.x + vector.y * vector.y + vector.z * vector.z;
    if dot > length * length {
        return (true, vector * (length * (1.0 / libm::sqrtf(dot))));
    }
    (false, vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shorter_vector_is_left_alone() {
        assert_eq!(
            limit2d(Vec2::new(3.0, 4.0), 5.0),
            (false, Vec2::new(3.0, 4.0))
        );
        assert_eq!(
            limit3d(Vec3::new(0.0, 3.0, 4.0), 6.0),
            (false, Vec3::new(0.0, 3.0, 4.0))
        );
    }

    #[test]
    fn a_longer_vector_is_shortened() {
        assert_eq!(
            limit2d(Vec2::new(3.0, 4.0), 2.5),
            (true, Vec2::new(1.5, 2.0))
        );
        assert_eq!(
            limit3d(Vec3::new(0.0, 3.0, 4.0), 2.5),
            (true, Vec3::new(0.0, 1.5, 2.0))
        );
    }
}
