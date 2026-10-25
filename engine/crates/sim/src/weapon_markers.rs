//! Where a weapon's effects are made: at its markers, as
//! `weapon_effect_new` makes them at the weapon's (its firing effect's
//! `primary trigger` and `primary ejection`). The local player's
//! first-person weapon is posed from the camera, so its effects are at its
//! first-person model's markers; anyone else's weapon is held at its grip
//! marker in the unit's hand marker, so its effects are at its own model's
//! markers there, in its default pose.

use std::collections::BTreeMap;
use std::sync::Arc;

use glam::{Mat3, Quat, Vec3};
use opence_content::{Level, Model, TagRef};

use crate::animation::{root_matrix, NodeMatrix};
use crate::first_person::FirstPersonModel;
use crate::seats::{default_pose, marker_matrix};
use crate::Transform;

/// A weapon's model held in the hands: the model and its default pose.
#[derive(Clone, Debug)]
struct Held {
    model: Arc<Model>,
    pose: Vec<NodeMatrix>,
}

/// The markers of a level's weapons, and its effects' locations.
#[derive(Clone, Debug, Default)]
pub struct WeaponMarkers {
    /// Each weapon's model, by the weapon's tag.
    held: BTreeMap<TagRef, Held>,
    /// The first-person models (the weapons' and the hands'), by their
    /// index in the level's models.
    first_person: BTreeMap<usize, Arc<Model>>,
    /// Each effect's locations, the names of markers, by its tag.
    locations: BTreeMap<TagRef, Vec<String>>,
}

impl WeaponMarkers {
    /// The markers of `level`'s weapons.
    pub fn for_level(level: &Level) -> Self {
        let model = |tag: Option<&TagRef>| {
            let index = level.model_index(tag?)?;
            Some((index, level.models.get(index)?))
        };
        let mut markers = WeaponMarkers::default();
        if let Some((index, hands)) = model(level.globals.first_person_hands.as_ref()) {
            markers.first_person.insert(index, Arc::new(hands.clone()));
        }
        for object in &level.objects {
            let Some(weapon) = object.weapon.as_ref() else {
                continue;
            };
            if let Some((_, held)) = model(object.model.as_ref()) {
                markers.held.insert(
                    object.tag.clone(),
                    Held {
                        model: Arc::new(held.clone()),
                        pose: default_pose(held),
                    },
                );
            }
            if let Some((index, first_person)) = model(weapon.first_person_model.as_ref()) {
                markers
                    .first_person
                    .insert(index, Arc::new(first_person.clone()));
            }
        }
        markers.locations = level
            .effects
            .iter()
            .map(|effect| (effect.tag.clone(), effect.locations.clone()))
            .collect();
        markers
    }

    /// The locations of `effect`, the names of markers.
    pub fn locations(&self, effect: &TagRef) -> &[String] {
        self.locations.get(effect).map_or(&[], Vec::as_slice)
    }

    /// The marker `name` of the first-person `models` (the weapon's before
    /// the hands'), posed from the camera `then` and moved with it to
    /// `now`.
    pub fn first_person(
        &self,
        models: &[FirstPersonModel],
        then: &Transform,
        now: &Transform,
        name: &str,
    ) -> Option<NodeMatrix> {
        let marker = models.iter().rev().find_map(|posed| {
            let model = self.first_person.get(&posed.model)?;
            marker_matrix(model, &posed.current, name)
        })?;
        let local = inverse(&root_matrix(then)).multiply(&marker);
        Some(root_matrix(now).multiply(&local))
    }

    /// The marker `name` of `weapon` held at its marker `grip` (its origin,
    /// for none) in `hand`.
    pub fn held(
        &self,
        weapon: &TagRef,
        hand: &NodeMatrix,
        grip: &str,
        name: &str,
    ) -> Option<NodeMatrix> {
        let held = self.held.get(weapon)?;
        let marker = marker_matrix(&held.model, &held.pose, name)?;
        let grip = match grip {
            "" => NodeMatrix::IDENTITY,
            grip => marker_matrix(&held.model, &held.pose, grip)?,
        };
        Some(hand.multiply(&inverse(&grip)).multiply(&marker))
    }
}

/// The matrix that undoes `matrix`: its turn back, its scale inverted and
/// its point taken away.
pub fn inverse(matrix: &NodeMatrix) -> NodeMatrix {
    let scale = if matrix.scale == 0.0 {
        1.0
    } else {
        1.0 / matrix.scale
    };
    let turn = Mat3::from_cols(
        Vec3::from_array(matrix.forward),
        Vec3::from_array(matrix.left),
        Vec3::from_array(matrix.up),
    )
    .transpose();
    let position = turn * Vec3::from_array(matrix.position) * -scale;
    NodeMatrix {
        forward: turn.x_axis.to_array(),
        left: turn.y_axis.to_array(),
        up: turn.z_axis.to_array(),
        position: position.to_array(),
        scale,
    }
}

/// `matrix`'s point and turn.
pub fn frame(matrix: &NodeMatrix) -> (Vec3, Quat) {
    let turn = Mat3::from_cols(
        Vec3::from_array(matrix.forward),
        Vec3::from_array(matrix.left),
        Vec3::from_array(matrix.up),
    );
    (
        Vec3::from_array(matrix.position),
        Quat::from_mat3(&turn).normalize(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use opence_content::{Group, MarkerInstance, ModelMarker, ModelNode};

    /// A model of one node at the origin, with markers on it at points,
    /// unturned.
    fn model(markers: &[(&str, [f32; 3])]) -> Model {
        Model {
            tag: TagRef {
                group: Group(*b"mod2"),
                path: "m".into(),
            },
            flags: 0,
            node_list_checksum: 0,
            detail_cutoffs: [0.0; 5],
            base_map_scale: [1.0; 2],
            nodes: vec![ModelNode {
                name: "frame".into(),
                parent: None,
                first_child: None,
                next_sibling: None,
                translation: [0.0; 3],
                rotation: [0.0, 0.0, 0.0, 1.0],
                distance_from_parent: 0.0,
                default_inverse_scale: 1.0,
                default_inverse: [[0.0; 3]; 4],
            }],
            markers: markers
                .iter()
                .map(|&(name, translation)| ModelMarker {
                    name: name.into(),
                    instances: vec![MarkerInstance {
                        region: 0,
                        permutation: 0,
                        node: 0,
                        translation,
                        rotation: [0.0, 0.0, 0.0, 1.0],
                    }],
                })
                .collect(),
            regions: Vec::new(),
            geometries: Vec::new(),
            shaders: Vec::new(),
        }
    }

    fn near(a: [f32; 3], b: [f32; 3]) -> bool {
        Vec3::from_array(a).distance(Vec3::from_array(b)) < 1e-5
    }

    #[test]
    fn an_inverse_undoes_its_matrix() {
        let matrix =
            NodeMatrix::from_point_and_vectors([1.0, -2.0, 3.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let both = inverse(&matrix).multiply(&matrix);
        assert!(near(both.transform_point([0.3, 0.4, 0.5]), [0.3, 0.4, 0.5]));
    }

    #[test]
    fn a_held_weapons_marker_is_from_its_grip_in_the_hand() {
        let tag = TagRef {
            group: Group(*b"weap"),
            path: "rifle".into(),
        };
        let weapon = model(&[
            ("grip", [0.1, 0.0, 0.0]),
            ("primary trigger", [0.6, 0.0, 0.05]),
        ]);
        let markers = WeaponMarkers {
            held: BTreeMap::from([(
                tag.clone(),
                Held {
                    pose: default_pose(&weapon),
                    model: Arc::new(weapon),
                },
            )]),
            ..WeaponMarkers::default()
        };
        // (a hand at (5, 0, 1), facing along y)
        let hand =
            NodeMatrix::from_point_and_vectors([5.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let trigger = markers
            .held(&tag, &hand, "grip", "primary trigger")
            .map(|matrix| matrix.position);
        assert!(
            trigger.is_some_and(|at| near(at, [5.0, 0.5, 1.05])),
            "{trigger:?}"
        );
        let ungripped = markers
            .held(&tag, &hand, "", "primary trigger")
            .map(|matrix| matrix.position);
        assert!(
            ungripped.is_some_and(|at| near(at, [5.0, 0.6, 1.05])),
            "{ungripped:?}"
        );
        assert!(markers.held(&tag, &hand, "grip", "smoke").is_none());
    }

    #[test]
    fn a_first_person_marker_moves_with_the_camera() {
        let hands = model(&[("primary trigger", [9.0, 9.0, 9.0])]);
        let rifle = model(&[("primary trigger", [10.5, 0.0, -0.1])]);
        let markers = WeaponMarkers {
            first_person: BTreeMap::from([(0, Arc::new(hands)), (1, Arc::new(rifle))]),
            ..WeaponMarkers::default()
        };
        let posed = |model| FirstPersonModel {
            model,
            previous: vec![NodeMatrix::IDENTITY],
            current: vec![NodeMatrix::IDENTITY],
        };
        // (posed from a camera at x = 10, which has since turned to face y)
        let then = Transform {
            position: Vec3::new(10.0, 0.0, 0.0),
            ..Transform::default()
        };
        let now = Transform {
            rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            ..then
        };
        let trigger = markers
            .first_person(&[posed(0), posed(1)], &then, &now, "primary trigger")
            .map(|matrix| matrix.position);
        // (the rifle's, not the hands', half a unit ahead and below)
        assert!(
            trigger.is_some_and(|at| near(at, [10.0, 0.5, -0.1])),
            "{trigger:?}"
        );
    }

    #[test]
    fn a_frame_keeps_its_point_and_turn() {
        let turn = Quat::from_rotation_z(0.7) * Quat::from_rotation_y(0.2);
        let matrix = NodeMatrix::from_point_and_vectors(
            [1.0, 2.0, 3.0],
            (turn * Vec3::X).to_array(),
            (turn * Vec3::Z).to_array(),
        );
        let (point, rotation) = frame(&matrix);
        assert_eq!(point, Vec3::new(1.0, 2.0, 3.0));
        assert!(rotation.angle_between(turn) < 1e-4);
    }
}
