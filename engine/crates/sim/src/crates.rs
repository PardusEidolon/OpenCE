//! Crates: boxes dropped into the player's physics world by the test map's
//! physics station, each falling, sliding and tumbling until it lies still.
//! They are not objects: nothing but the physics and the debug drawing
//! sees them.

use glam::{Quat, Vec3};
use opence_physics::{BodyId, Material, Motion, Pose, Shape, World as PhysicsWorld};

/// Slower than this, in world units a second, a crate is still.
pub const REST_SPEED: f32 = 0.05;
/// The ticks a crate is that slow for before it lies still.
pub const REST_TICKS: u32 = 15;
/// The most crates at once.
pub const CRATES_MAX: usize = 64;
/// A crate's mass for each cubic world unit it holds, in kilograms.
const DENSITY: f32 = 400.0;

/// A crate.
#[derive(Clone, Debug, PartialEq)]
pub struct Crate {
    body: BodyId,
    /// Where it is.
    pub pose: Pose,
    /// Half its size along each of its axes, in world units.
    pub half_extents: Vec3,
    /// How fast it moves, in world units a second.
    pub speed: f32,
    /// Whether the physics has put it to sleep.
    pub asleep: bool,
    /// Its mass, in kilograms.
    mass: f32,
    /// The ticks it has been slow for.
    still: u32,
}

impl Crate {
    /// Whether it lies still: asleep, or slow for a moment.
    pub fn resting(&self) -> bool {
        self.asleep || self.still >= REST_TICKS
    }
}

/// The crates of a game.
#[derive(Clone, Debug, Default)]
pub struct Crates {
    crates: Vec<Crate>,
}

impl Crates {
    /// The crates, the first dropped first.
    pub fn all(&self) -> &[Crate] {
        &self.crates
    }

    /// Whether any crate still moves (the physics steps for it).
    pub fn moving(&self) -> bool {
        self.crates.iter().any(|one| !one.resting())
    }

    /// Drops a crate of `half_extents` at `position`, turned by
    /// `rotation`; false if there are already [`CRATES_MAX`] or the
    /// physics refuses its body.
    pub fn drop_crate(
        &mut self,
        physics: &mut PhysicsWorld,
        position: Vec3,
        rotation: Quat,
        half_extents: Vec3,
    ) -> bool {
        if self.crates.len() >= CRATES_MAX || !half_extents.cmpgt(Vec3::ZERO).all() {
            return false;
        }
        let pose = Pose { position, rotation };
        let mass =
            (DENSITY * 8.0 * half_extents.x * half_extents.y * half_extents.z).clamp(1.0, 5000.0);
        let Ok(body) = physics.add_body(
            Shape::Box { half_extents },
            pose,
            Motion::Dynamic,
            mass,
            Material {
                friction: 0.5,
                restitution: 0.2,
            },
        ) else {
            return false;
        };
        self.crates.push(Crate {
            body,
            pose,
            half_extents,
            speed: 0.0,
            asleep: false,
            mass,
            still: 0,
        });
        true
    }

    /// Removes every crate.
    pub fn clear(&mut self, physics: &mut PhysicsWorld) {
        for one in self.crates.drain(..) {
            physics.remove_body(one.body);
        }
    }

    /// Forgets every crate, whose bodies went with their physics world.
    pub fn forget(&mut self) {
        self.crates.clear();
    }

    /// Pushes each crate that one of `pushers` (a sphere's centre, radius
    /// and velocity in world units a second: a vehicle's mass points)
    /// reaches into, so that it goes at least as fast as the sphere away
    /// from it across the ground: a vehicle shoves a crate ahead rather
    /// than stopping at it.
    /// Whether any was pushed.
    pub fn push_by(&mut self, physics: &mut PhysicsWorld, pushers: &[(Vec3, f32, Vec3)]) -> bool {
        let mut pushed = false;
        for one in &mut self.crates {
            let (position, rotation) = (one.pose.position, one.pose.rotation);
            for &(centre, radius, velocity) in pushers {
                // (the crate's nearest point to the sphere's centre)
                let local = rotation.inverse() * (centre - position);
                let nearest = local.clamp(-one.half_extents, one.half_extents);
                let apart = local - nearest;
                let distance = apart.length();
                if distance >= radius {
                    continue;
                }
                // (out of the crate towards the sphere, across the ground:
                // a wheel rolling onto one would press it into the level's
                // mesh, which holds it only by its surface; from inside it,
                // against the way the sphere goes)
                let out = if distance > 0.0 {
                    rotation * apart
                } else {
                    -velocity
                };
                let Some(normal) = Vec3::new(out.x, out.y, 0.0).try_normalize() else {
                    continue;
                };
                let point = position + rotation * nearest;
                let Ok((linear, angular)) = physics.body_velocity(one.body) else {
                    continue;
                };
                let at = linear + angular.cross(point - position);
                let short = (velocity - at).dot(-normal);
                if short <= 0.0 {
                    continue;
                }
                if physics
                    .push_body(one.body, -normal * (one.mass * short), point)
                    .is_ok()
                {
                    one.still = 0;
                    one.asleep = false;
                    pushed = true;
                }
            }
        }
        pushed
    }

    /// After the physics steps: each crate's pose and speed.
    pub fn settle(&mut self, physics: &PhysicsWorld) {
        for one in &mut self.crates {
            if let Ok(pose) = physics.body_pose(one.body) {
                one.pose = pose;
            }
            one.speed = physics
                .body_velocity(one.body)
                .map_or(0.0, |(linear, _)| linear.length());
            one.asleep = !physics.body_awake(one.body).unwrap_or(false);
            one.still = if one.speed < REST_SPEED {
                one.still.saturating_add(1)
            } else {
                0
            };
        }
    }
}
