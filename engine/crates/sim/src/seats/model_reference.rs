//! A model's nodes and markers are the game's: tools/engine_model_reference.py
//! runs models.c's `model_node_matrices_from_orientations`,
//! `model_find_marker` and `model_get_marker_by_name` over six models
//! (trees of 1 to 70 nodes, one with links the model does not have and a
//! loop) at sixty poses, asking each for markers by sixteen names, and the
//! engine's node matrices and marker lookup are asked the same and compared
//! bit for bit.

use std::collections::BTreeMap;

use opence_content::{AnimationNode, Group, MarkerInstance, Model, ModelMarker, TagRef};

use super::marker_matrices;
use crate::animation::{node_matrices, NodeMatrix, Orientation};

const REFERENCE: &str = include_str!("../../tests/model_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// Each line of a kind, split after its name.
fn lines(kind: &str) -> impl Iterator<Item = Vec<&str>> + '_ {
    REFERENCE
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(move |fields| fields.first() == Some(&kind))
        .map(|fields| fields[1..].to_vec())
}

fn integer(fields: &[&str], at: usize) -> Result<i32, Failure> {
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

fn reals<const N: usize>(fields: &[&str], from: usize) -> Result<[f32; N], Failure> {
    let mut values = [0.0; N];
    for (offset, value) in values.iter_mut().enumerate() {
        *value = fields.get(from + offset).ok_or("a short line")?.parse()?;
    }
    Ok(values)
}

/// A link of the game's (a short, `NONE` for none), as the engine imports it.
fn link(fields: &[&str], at: usize) -> Result<Option<u16>, Failure> {
    let value = integer(fields, at)?;
    Ok(if value == -1 {
        None
    } else {
        Some(u16::try_from(value)?)
    })
}

fn matrix(fields: &[&str], from: usize) -> Result<NodeMatrix, Failure> {
    let [scale, fi, fj, fk, li, lj, lk, ui, uj, uk, x, y, z] = reals::<13>(fields, from)?;
    Ok(NodeMatrix {
        forward: [fi, fj, fk],
        left: [li, lj, lk],
        up: [ui, uj, uk],
        position: [x, y, z],
        scale,
    })
}

/// The trace's models: their nodes' links, and their markers as the
/// engine imports them.
fn models() -> Result<BTreeMap<i32, (Vec<AnimationNode>, Model)>, Failure> {
    let mut models = BTreeMap::new();
    for fields in lines("model") {
        models.insert(
            integer(&fields, 0)?,
            (
                Vec::new(),
                Model {
                    tag: TagRef {
                        group: Group(*b"mod2"),
                        path: String::new(),
                    },
                    flags: 0,
                    node_list_checksum: 0,
                    detail_cutoffs: [0.0; 5],
                    base_map_scale: [1.0; 2],
                    nodes: Vec::new(),
                    markers: Vec::new(),
                    regions: Vec::new(),
                    geometries: Vec::new(),
                    shaders: Vec::new(),
                },
            ),
        );
    }
    for fields in lines("node") {
        let (links, _) = models
            .get_mut(&integer(&fields, 0)?)
            .ok_or("no such model")?;
        links.push(AnimationNode {
            name: String::new(),
            parent: link(&fields, 2)?,
            first_child: link(&fields, 3)?,
            next_sibling: link(&fields, 4)?,
        });
    }
    for fields in lines("instance") {
        let (_, model) = models
            .get_mut(&integer(&fields, 0)?)
            .ok_or("no such model")?;
        let name = *fields.get(1).ok_or("a short line")?;
        if model
            .markers
            .last()
            .is_none_or(|marker| marker.name != name)
        {
            model.markers.push(ModelMarker {
                name: name.into(),
                instances: Vec::new(),
            });
        }
        let marker = model.markers.last_mut().ok_or("no marker")?;
        let [x, y, z, i, j, k, w] = reals::<7>(&fields, 5)?;
        marker.instances.push(MarkerInstance {
            region: u8::try_from(integer(&fields, 2)?)?,
            permutation: u8::try_from(integer(&fields, 3)?)?,
            node: u8::try_from(integer(&fields, 4)?)?,
            translation: [x, y, z],
            rotation: [i, j, k, w],
        });
    }
    Ok(models)
}

#[test]
fn a_models_nodes_and_markers_are_the_games() -> Result<(), Failure> {
    let models = models()?;
    let asked: BTreeMap<i32, &str> = lines("asked")
        .map(|fields| {
            let name = *fields.get(1).ok_or("a short line")?;
            Ok((integer(&fields, 0)?, if name == "-" { "" } else { name }))
        })
        .collect::<Result<_, Failure>>()?;
    let (mut nodes, mut unreached, mut markers) = (0, 0, 0);
    for pose in lines("pose") {
        let trial = integer(&pose, 0)?;
        let (links, model) = models.get(&integer(&pose, 1)?).ok_or("no such model")?;
        let [x, y, z, fi, fj, fk, ui, uj, uk] = reals::<9>(&pose, 2)?;
        let root = NodeMatrix::from_point_and_vectors([x, y, z], [fi, fj, fk], [ui, uj, uk]);
        let orientations = lines("orientation")
            .filter(|fields| integer(fields, 0).is_ok_and(|of| of == trial))
            .map(|fields| {
                let [i, j, k, w, tx, ty, tz, scale] = reals::<8>(&fields, 2)?;
                Ok(Orientation {
                    rotation: [i, j, k, w],
                    translation: [tx, ty, tz],
                    scale,
                })
            })
            .collect::<Result<Vec<_>, Failure>>()?;
        let matrices = node_matrices(links, &orientations, &root);
        // (a node the game's walk never reaches keeps whatever its matrix
        // had: the engine's is the identity, and the trace leaves it so)
        let mut expected = vec![NodeMatrix::IDENTITY; links.len()];
        for fields in
            lines("matrix").filter(|fields| integer(fields, 0).is_ok_and(|of| of == trial))
        {
            let node = usize::try_from(integer(&fields, 1)?)?;
            *expected.get_mut(node).ok_or("no such node")? = matrix(&fields, 2)?;
            nodes += 1;
        }
        unreached += lines("unreached")
            .filter(|fields| integer(fields, 0).is_ok_and(|of| of == trial))
            .count();
        assert_eq!(matrices, expected, "trial {trial}'s nodes");

        for ask in lines("ask").filter(|fields| integer(fields, 0).is_ok_and(|of| of == trial)) {
            let index = integer(&ask, 2)?;
            let name = asked.get(&index).ok_or("no such name")?;
            let expected = lines("found")
                .filter(|fields| {
                    integer(fields, 0).is_ok_and(|of| of == trial)
                        && integer(fields, 1).is_ok_and(|of| of == index)
                })
                .map(|fields| matrix(&fields, 4))
                .collect::<Result<Vec<_>, Failure>>()?;
            assert_eq!(
                usize::try_from(integer(&ask, 4)?)?,
                expected.len(),
                "a short trace"
            );
            assert_eq!(
                marker_matrices(model, &matrices, name),
                expected,
                "trial {trial}'s marker {name:?}"
            );
            markers += 1;
        }
    }
    assert_eq!((nodes + unreached, markers), (1240, 960));
    assert!(unreached > 0 && unreached < 100, "{unreached} unreached");
    Ok(())
}
