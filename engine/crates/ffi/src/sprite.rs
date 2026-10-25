//! How a sprite faces, as render_sprite.c's `build_sprite_compute_basis`
//! lays it out and `build_sprite_compute_vertex_fade` fades it: the axes its
//! corners lie along and how much of it shows as it turns edge on. The app's
//! particle shader (`particle.metal`) is a transcription of these; the
//! sprite reference test holds them to the game's own functions.
//!
//! The game works in the camera's space; these work in the world's, which
//! gives the same axes, since the camera's space is only turned from it.

use opence_sim::Vec3;

use crate::snapshot::{
    PARTICLE_FACING_PARALLEL, PARTICLE_FACING_PERPENDICULAR, PARTICLE_FACING_SCREEN,
    PARTICLE_FADE_SHIFT, PARTICLE_MIRROR_U, PARTICLE_MIRROR_V,
};

/// Below this length the game leaves a vector as it is when it normalizes
/// it (`_real_epsilon`).
const EPSILON: f32 = 0.0001;

/// How a sprite faces (a particle's `sprite orientation`, a particle
/// system's `sprite render mode` and a weather system's `render mode`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    /// Facing the screen.
    Screen,
    /// Lying along its direction, turned about it to the camera.
    Parallel,
    /// Lying across its direction, the direction its normal.
    Perpendicular,
}

impl Orientation {
    /// The orientation a tag's field names (the game's three; anything
    /// else faces the screen).
    pub fn from_tag(value: i16) -> Self {
        match value {
            1 => Orientation::Parallel,
            2 => Orientation::Perpendicular,
            _ => Orientation::Screen,
        }
    }
}

/// `ParticleSnapshot::modes`' facing for a sprite of `orientation` along
/// `direction`, fading by its shader's `fade_mode` and its picture
/// mirrored across and up and down by `mirrors`. A sprite with no direction
/// faces the screen.
pub fn facing(
    orientation: Orientation,
    direction: Vec3,
    fade_mode: i16,
    mirrors: [bool; 2],
) -> u32 {
    let facing = match orientation {
        _ if direction == Vec3::ZERO => PARTICLE_FACING_SCREEN,
        Orientation::Screen => PARTICLE_FACING_SCREEN,
        Orientation::Parallel => PARTICLE_FACING_PARALLEL,
        Orientation::Perpendicular => PARTICLE_FACING_PERPENDICULAR,
    };
    let fade = match fade_mode {
        1 | 2 if facing != PARTICLE_FACING_SCREEN => (fade_mode as u32) << PARTICLE_FADE_SHIFT,
        _ => 0,
    };
    let [u, v] = mirrors;
    facing | fade | if u { PARTICLE_MIRROR_U } else { 0 } | if v { PARTICLE_MIRROR_V } else { 0 }
}

/// `normalize3d`: `v` made one long, or left as it is when it is too short.
fn normalize(v: Vec3) -> Vec3 {
    let length = libm::sqrtf(v.dot(v));
    if length < EPSILON {
        v
    } else {
        v * (1.0 / length)
    }
}

/// The two axes a sprite at `at`, seen from `eye`, lays its corners along
/// (its width's and its height's), for a sprite lying along or across its
/// `direction`. Facing the screen, they are the camera's right and up,
/// which the shader takes from the view; these give zeros.
pub fn basis(orientation: Orientation, eye: Vec3, at: Vec3, direction: Vec3) -> (Vec3, Vec3) {
    match orientation {
        Orientation::Screen => (Vec3::ZERO, Vec3::ZERO),
        Orientation::Parallel => {
            let x = normalize(direction);
            (x, normalize((at - eye).cross(x)))
        }
        Orientation::Perpendicular => {
            // (across the world's up, or, for a direction nearly along it,
            // across the world's left)
            let up = Vec3::Z;
            let along = direction.dot(up);
            let reference = if along * along > direction.dot(direction) * 0.99 {
                Vec3::Y
            } else {
                up
            };
            let x = normalize(direction.cross(reference));
            let normal = normalize(direction);
            // (`rotate_vector_about_axis` by a quarter turn back)
            (x, normal * normal.dot(x) + x.cross(normal))
        }
    }
}

/// How much of a sprite at `at`, its face's normal `normal`, shows from
/// `eye` by its shader's fade mode: all of it for none (0), the more the
/// more it faces the camera (1), or the more the more it is edge on (2).
pub fn fade(mode: u8, eye: Vec3, at: Vec3, normal: Vec3) -> f32 {
    if mode == 0 {
        return 1.0;
    }
    let offset = at - eye;
    let facing = libm::fabsf(normal.dot(offset) / libm::sqrtf(offset.dot(offset)));
    if mode == 2 {
        1.0 - facing
    } else {
        facing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_parallel_sprite_lies_along_its_direction_and_faces_the_camera() {
        let (x, y) = basis(
            Orientation::Parallel,
            Vec3::ZERO,
            Vec3::new(5.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 3.0),
        );
        assert_eq!(x, Vec3::Z);
        // (its face's normal points back along the view)
        assert!(x.cross(y).dot(Vec3::X).abs() > 0.999, "{x} {y}");
    }

    #[test]
    fn a_perpendicular_sprite_lies_across_its_direction() {
        let direction = Vec3::new(1.0, 2.0, 0.5);
        let (x, y) = basis(Orientation::Perpendicular, Vec3::ZERO, Vec3::X, direction);
        assert!(x.dot(direction).abs() < 1e-6 && y.dot(direction).abs() < 1e-5);
        // (its face looks back along it, as the game's does)
        assert!((x.cross(y).normalize() + direction.normalize()).length() < 1e-5);
        // (straight up, it lies across the world's left instead)
        let (x, _) = basis(Orientation::Perpendicular, Vec3::ZERO, Vec3::X, Vec3::Z);
        assert_eq!(x, Vec3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn the_fade_modes_fade_toward_and_away_from_edge_on() {
        let (eye, at) = (Vec3::ZERO, Vec3::new(2.0, 0.0, 0.0));
        assert_eq!(fade(0, eye, at, Vec3::Z), 1.0);
        assert_eq!(fade(1, eye, at, Vec3::X), 1.0);
        assert_eq!(fade(1, eye, at, Vec3::Z), 0.0);
        assert_eq!(fade(2, eye, at, Vec3::Z), 1.0);
    }

    #[test]
    fn a_facing_packs_its_fade_and_mirrors() {
        let up = Vec3::Z;
        assert_eq!(
            facing(Orientation::Screen, up, 1, [false; 2]),
            PARTICLE_FACING_SCREEN
        );
        assert_eq!(
            facing(Orientation::Parallel, up, 2, [true, false]),
            PARTICLE_FACING_PARALLEL | 2 << PARTICLE_FADE_SHIFT | PARTICLE_MIRROR_U
        );
        assert_eq!(
            facing(Orientation::Perpendicular, up, 0, [false, true]),
            PARTICLE_FACING_PERPENDICULAR | PARTICLE_MIRROR_V
        );
        // (with no direction, it faces the screen and does not fade)
        assert_eq!(
            facing(Orientation::Parallel, Vec3::ZERO, 1, [false; 2]),
            PARTICLE_FACING_SCREEN
        );
    }

    #[test]
    fn the_tags_orientations_are_the_games_three() {
        assert_eq!(Orientation::from_tag(0), Orientation::Screen);
        assert_eq!(Orientation::from_tag(1), Orientation::Parallel);
        assert_eq!(Orientation::from_tag(2), Orientation::Perpendicular);
        assert_eq!(Orientation::from_tag(3), Orientation::Screen);
    }
}
