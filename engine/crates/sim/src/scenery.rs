//! Scenery in the level's collision: each placed scenery object's
//! collision model, at its default pose, joined to its structure BSP's
//! mesh in the players' physics world. The original tests the scenery with
//! the structure (`_collision_test_for_vehicles_flags` and the bipeds'
//! have its bit): a vehicle's mass points and a biped stop at it. Scenery
//! does not move, so the mesh is built again only when what is placed
//! changes (the level's start, a switch of structure BSP).

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_ecs::prelude::World as Ecs;
use glam::Vec3;
use opence_content::{CollisionBsp, Level, ObjectKind, TagRef};
use opence_physics::{LevelMesh, PhysicsError, World as PhysicsWorld};

use crate::animation::NodeMatrix;
use crate::cached::Cached;
use crate::objects::{Object, ObjectHandle};
use crate::seats::default_pose;
use crate::Transform;

/// What [`SceneryCollision::update`] reads of each object.
type Placed = (&'static ObjectHandle, &'static Object, &'static Transform);

/// A scenery definition's collision: each BSP with the node it moves with,
/// from the object's origin.
#[derive(Clone, Debug, PartialEq)]
struct Piece {
    bsp: CollisionBsp,
    node: NodeMatrix,
}

/// The level's scenery as collision.
#[derive(Clone, Debug, Default)]
pub struct SceneryCollision {
    /// Each scenery definition's pieces.
    pieces: BTreeMap<TagRef, Arc<Vec<Piece>>>,
    /// The current structure BSP's mesh.
    structure: Option<Arc<LevelMesh>>,
    /// The scenery the mesh was last built with, in the order of their
    /// handles (none: it is to be built).
    built: Option<Vec<(ObjectHandle, TagRef, Transform)>>,
    /// The objects (a kept query).
    placed: Cached<Placed>,
}

impl SceneryCollision {
    /// `level`'s scenery definitions' collision, and no structure yet.
    pub fn for_level(level: &Level) -> SceneryCollision {
        let pieces = level
            .objects
            .iter()
            .filter(|object| object.kind == ObjectKind::Scenery)
            .filter_map(|object| {
                let collision = level.collision_model(object.collision_model.as_ref()?)?;
                let model = object
                    .model
                    .as_ref()
                    .and_then(|tag| level.model_index(tag))
                    .and_then(|index| level.models.get(index));
                let pose = model.map(default_pose).unwrap_or_default();
                let pieces: Vec<Piece> = collision
                    .nodes
                    .iter()
                    .enumerate()
                    .flat_map(|(index, node)| {
                        // (its model's node of the same name, else the same
                        // index, else the origin)
                        let node_index = model
                            .and_then(|model| {
                                model.nodes.iter().position(|other| other.name == node.name)
                            })
                            .unwrap_or(index);
                        let matrix = pose
                            .get(node_index)
                            .copied()
                            .unwrap_or(NodeMatrix::IDENTITY);
                        node.bsps.iter().map(move |bsp| Piece {
                            bsp: bsp.clone(),
                            node: matrix,
                        })
                    })
                    .collect();
                (!pieces.is_empty()).then(|| (object.tag.clone(), Arc::new(pieces)))
            })
            .collect();
        SceneryCollision {
            pieces,
            structure: None,
            built: None,
            placed: Cached::default(),
        }
    }

    /// Takes `level`'s structure BSP `bsp` as the mesh's base; the mesh is
    /// built again at the next update.
    pub fn set_bsp(&mut self, level: &Level, bsp: usize) {
        self.structure = level
            .structure_bsps
            .get(bsp)
            .map(|structure| Arc::new(LevelMesh::from_collision(&structure.collision)));
        self.built = None;
    }

    /// The scenery placed that has collision, in the order of handles.
    fn placed(&mut self, world: &mut Ecs) -> Vec<(ObjectHandle, TagRef, Transform)> {
        let mut placed: Vec<(ObjectHandle, TagRef, Transform)> = self
            .placed
            .get(world)
            .iter(world)
            .filter(|(_, object, _)| object.kind == ObjectKind::Scenery)
            .filter(|(_, object, _)| self.pieces.contains_key(&object.tag))
            .map(|(&handle, object, &transform)| (handle, object.tag.clone(), transform))
            .collect();
        placed.sort_by_key(|(handle, _, _)| *handle);
        placed
    }

    /// Whether the scenery placed has changed since the mesh was built
    /// (looked at without allocating, as it is every tick).
    pub fn stale(&mut self, world: &mut Ecs) -> bool {
        if self.structure.is_none() {
            return false;
        }
        let Some(built) = &self.built else {
            return true;
        };
        let mut seen = 0;
        for (handle, object, transform) in self.placed.get(world).iter(world) {
            if object.kind != ObjectKind::Scenery || !self.pieces.contains_key(&object.tag) {
                continue;
            }
            seen += 1;
            let same = built
                .binary_search_by_key(handle, |(other, _, _)| *other)
                .ok()
                .and_then(|index| built.get(index))
                .is_some_and(|(_, tag, placed)| *tag == object.tag && placed == transform);
            if !same {
                return true;
            }
        }
        seen != built.len()
    }

    /// Builds the mesh again into each of `worlds` if the scenery placed
    /// has changed: whether it was built.
    pub fn update<'a>(
        &mut self,
        world: &mut Ecs,
        worlds: impl IntoIterator<Item = &'a mut PhysicsWorld>,
    ) -> Result<bool, PhysicsError> {
        let Some(structure) = self.structure.clone() else {
            return Ok(false);
        };
        let placed = self.placed(world);
        if self.built.as_ref() == Some(&placed) {
            return Ok(false);
        }
        let mesh = self.mesh(&structure, &placed);
        for physics in worlds {
            physics.set_level(&mesh)?;
        }
        self.built = Some(placed);
        Ok(true)
    }

    /// The structure's mesh with the scenery of `placed` in it.
    fn mesh(
        &self,
        structure: &LevelMesh,
        placed: &[(ObjectHandle, TagRef, Transform)],
    ) -> LevelMesh {
        let mut mesh = structure.clone();
        for (_, tag, transform) in placed {
            let Some(pieces) = self.pieces.get(tag) else {
                continue;
            };
            for piece in pieces.iter() {
                mesh.append_placed(&piece.bsp, |point| {
                    let local = Vec3::from_array(piece.node.transform_point(point));
                    (transform.position + transform.rotation * (local * transform.scale)).to_array()
                });
            }
        }
        mesh
    }
}
