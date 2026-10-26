//! The engine's collision queries against the game's: the collision BSP
//! the game's `collision_bsp_test_vector`, `collision_bsp_test_sphere`,
//! `collision_bsp_test_pill` and `bsp3d_test_point` ran over (a room with
//! a floating pillar, tools/engine_collision_reference.py) is rebuilt from
//! the trace, made into the level's mesh, and every line, sphere and moving
//! sphere the game cast is cast again through Jolt, and every point looked
//! up with `CollisionBsp::leaf_at`.
//!
//! Points must name the game's leaf exactly. A line or a moving sphere
//! that starts in the open must hit when the game's does, as far along
//! (to a thousandth of a unit), on the game's surface's plane. A sphere
//! in the open must touch the level when the game's touches a surface.
//! Casts that start inside solid, or that meet the level within a
//! hundredth of a unit of an edge (where which of two surfaces is first is
//! a matter of rounding), are left out, and counted.
//!
//! A moving sphere the engine's stops on an edge the game's passes through
//! is counted apart: collision_bsp.c's `pill_test_vector` solves for where
//! a sphere meets an edge's capsule with its linear term's sign turned
//! over (real_math.c's `pill_test_vector3d` has it the right way), so the
//! game's roots are the ones the sphere would meet moving backward, and a
//! sphere moving toward an edge past the faces' ends never meets it. Which
//! the xbe does is still to be measured; until then the engine meets the
//! edge.

use std::error::Error;

use glam::Vec3;
use opence_content::level::Bsp2dReference;
use opence_content::level::{
    Bsp3dNode, CollisionEdge, CollisionLeaf, CollisionSurface, CollisionVertex,
};
use opence_content::CollisionBsp;
use opence_physics::{Layers, LevelMesh, Pose, Shape, World, WorldSettings};

const REFERENCE: &str = include_str!("collision_reference.txt");

type Failure = Box<dyn Error>;

/// The reference's lines of one kind, split into fields after the kind.
fn lines(kind: &str) -> impl Iterator<Item = Vec<&'static str>> + '_ {
    REFERENCE.lines().filter_map(move |line| {
        let mut fields = line.split_whitespace();
        (fields.next() == Some(kind)).then(|| fields.collect())
    })
}

fn integer(fields: &[&str], at: usize) -> Result<i64, Failure> {
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

fn real(fields: &[&str], at: usize) -> Result<f32, Failure> {
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

fn reals<const N: usize>(fields: &[&str], from: usize) -> Result<[f32; N], Failure> {
    let mut values = [0.0; N];
    for (index, value) in values.iter_mut().enumerate() {
        *value = real(fields, from + index)?;
    }
    Ok(values)
}

fn index(fields: &[&str], at: usize) -> Result<u32, Failure> {
    Ok(u32::try_from(integer(fields, at)?)?)
}

/// The game's world: the BSP the trace was made over.
fn level() -> Result<CollisionBsp, Failure> {
    let mut bsp = CollisionBsp::default();
    for fields in lines("plane") {
        bsp.planes.push(reals(&fields, 1)?);
    }
    for fields in lines("vertex") {
        bsp.vertices.push(CollisionVertex {
            point: reals(&fields, 1)?,
            first_edge: index(&fields, 4)?,
        });
    }
    for fields in lines("edge") {
        bsp.edges.push(CollisionEdge {
            vertices: [index(&fields, 1)?, index(&fields, 2)?],
            edges: [index(&fields, 3)?, index(&fields, 4)?],
            surfaces: [Some(index(&fields, 5)?), Some(index(&fields, 6)?)],
        });
    }
    for fields in lines("surface") {
        bsp.surfaces.push(CollisionSurface {
            plane: i32::try_from(integer(&fields, 1)?)?,
            first_edge: index(&fields, 2)?,
            flags: 0,
            breakable_surface: 0,
            material: None,
        });
    }
    for fields in lines("node") {
        bsp.bsp3d_nodes.push(Bsp3dNode {
            plane: i32::try_from(integer(&fields, 1)?)?,
            children: [
                i32::try_from(integer(&fields, 2)?)?,
                i32::try_from(integer(&fields, 3)?)?,
            ],
        });
    }
    for fields in lines("leaf") {
        bsp.leaves.push(CollisionLeaf {
            flags: u16::try_from(integer(&fields, 1)?)?,
            bsp2d_reference_count: u16::try_from(integer(&fields, 2)?)?,
            first_bsp2d_reference: index(&fields, 3)?,
        });
    }
    for fields in lines("reference") {
        bsp.bsp2d_references.push(Bsp2dReference {
            plane: i32::try_from(integer(&fields, 1)?)?,
            node: i32::try_from(integer(&fields, 2)?)?,
        });
    }
    Ok(bsp)
}

fn world(bsp: &CollisionBsp) -> Result<World, Failure> {
    let mut world = World::new(WorldSettings::default())?;
    world.set_level(&LevelMesh::from_collision(bsp))?;
    Ok(world)
}

/// How near `point` is to any of the level's edges.
fn edge_distance(bsp: &CollisionBsp, point: Vec3) -> f32 {
    bsp.edges
        .iter()
        .filter_map(|edge| {
            let a = Vec3::from_array(bsp.vertices.get(edge.vertices[0] as usize)?.point);
            let b = Vec3::from_array(bsp.vertices.get(edge.vertices[1] as usize)?.point);
            let along = ((point - a).dot(b - a) / (b - a).length_squared()).clamp(0.0, 1.0);
            Some(point.distance(a + (b - a) * along))
        })
        .fold(f32::INFINITY, f32::min)
}

/// Whether a point is in the open: in one of the BSP's leaves.
fn open(bsp: &CollisionBsp, point: Vec3) -> bool {
    bsp.leaf_at(point.to_array()).is_some()
}

#[test]
fn points_name_the_games_leaves() -> Result<(), Failure> {
    let bsp = level()?;
    let mut count = 0;
    for fields in lines("point") {
        let point: [f32; 3] = reals(&fields, 1)?;
        let leaf = integer(&fields, 4)?;
        let expected = u32::try_from(leaf).ok();
        assert_eq!(bsp.leaf_at(point), expected, "point {}", fields[0]);
        count += 1;
    }
    assert_eq!(count, 1000);
    Ok(())
}

#[test]
fn lines_meet_the_games_surfaces() -> Result<(), Failure> {
    let bsp = level()?;
    let world = world(&bsp)?;
    let (mut compared, mut in_solid, mut at_edges) = (0, 0, 0);
    for fields in lines("vector") {
        let point = Vec3::from_array(reals(&fields, 1)?);
        let vector = Vec3::from_array(reals(&fields, 4)?);
        let hit = integer(&fields, 7)? != 0;
        if !open(&bsp, point) {
            in_solid += 1;
            continue;
        }
        let engine = world.cast_ray(point, vector, Layers::LEVEL)?;
        if hit {
            let t = real(&fields, 8)?;
            let plane = bsp.planes[index(&fields, 9)? as usize];
            if edge_distance(&bsp, point + vector * t) < 0.01 {
                at_edges += 1;
                continue;
            }
            let engine = engine.ok_or_else(|| format!("line {}: the game's hits", fields[0]))?;
            let length = vector.length();
            assert!(
                (engine.fraction - t).abs() * length < 1e-3,
                "line {}: at {} of the way, the game's at {t}",
                fields[0],
                engine.fraction
            );
            let normal = Vec3::new(plane[0], plane[1], plane[2]);
            assert!(
                engine.normal.dot(normal) > 0.9999,
                "line {}: normal {} on the game's plane {normal}",
                fields[0],
                engine.normal
            );
        } else {
            let end = point + vector;
            if engine.is_some_and(|hit| edge_distance(&bsp, hit.point) < 0.01)
                || !open(&bsp, end) && edge_distance(&bsp, end) < 0.01
            {
                at_edges += 1;
                continue;
            }
            assert!(
                engine.is_none(),
                "line {}: the game's misses, the engine's hits {engine:?}",
                fields[0]
            );
        }
        compared += 1;
    }
    eprintln!("lines: {compared} compared, {in_solid} starting in solid, {at_edges} at edges");
    assert!(
        compared >= 600 && at_edges <= 10,
        "{compared} compared, {at_edges} at edges"
    );
    Ok(())
}

#[test]
fn spheres_touch_where_the_games_do() -> Result<(), Failure> {
    let bsp = level()?;
    let world = world(&bsp)?;
    let (mut compared, mut in_solid, mut grazing) = (0, 0, 0);
    for fields in lines("sphere") {
        let center = Vec3::from_array(reals(&fields, 1)?);
        let radius = real(&fields, 4)?;
        let hit = integer(&fields, 5)? != 0;
        if !open(&bsp, center) {
            in_solid += 1;
            continue;
        }
        let engine = world.sphere_contact(center, radius, Layers::LEVEL)?;
        if engine.is_some_and(|contact| contact.depth < 1e-3) {
            grazing += 1;
            continue;
        }
        assert_eq!(
            engine.is_some(),
            hit,
            "sphere {}: the game's {hit}, the engine's {engine:?}",
            fields[0]
        );
        compared += 1;
    }
    eprintln!("spheres: {compared} compared, {in_solid} centred in solid, {grazing} grazing");
    assert!(
        compared >= 600 && grazing <= 10,
        "{compared} compared, {grazing} grazing"
    );
    Ok(())
}

#[test]
fn moving_spheres_meet_the_games_surfaces() -> Result<(), Failure> {
    let bsp = level()?;
    let world = world(&bsp)?;
    let (mut compared, mut starting_in, mut at_edges, mut edges_passed) = (0, 0, 0, 0);
    for fields in lines("pill") {
        let point = Vec3::from_array(reals(&fields, 1)?);
        let vector = Vec3::from_array(reals(&fields, 4)?);
        let radius = real(&fields, 7)?;
        let hit = integer(&fields, 8)? != 0;
        if !open(&bsp, point)
            || world
                .sphere_contact(point, radius, Layers::LEVEL)?
                .is_some()
        {
            starting_in += 1;
            continue;
        }
        let engine = world.cast_shape(
            Shape::Sphere { radius },
            Pose::at(point),
            vector,
            Layers::LEVEL,
        )?;
        if hit {
            let t = real(&fields, 9)?;
            let normal = Vec3::from_array(reals(&fields, 10)?);
            let engine = engine.ok_or_else(|| format!("pill {}: the game's hits", fields[0]))?;
            if edge_distance(&bsp, engine.point) < 0.01 {
                at_edges += 1;
                continue;
            }
            assert!(
                (engine.fraction - t).abs() * vector.length() < 1e-3,
                "pill {}: at {} of the way, the game's at {t}",
                fields[0],
                engine.fraction
            );
            assert!(
                engine.normal.dot(normal) > 0.9999,
                "pill {}: normal {} on the game's plane {normal}",
                fields[0],
                engine.normal
            );
        } else if engine.is_some_and(|hit| edge_distance(&bsp, hit.point) < 0.01) {
            edges_passed += 1;
            continue;
        } else {
            assert!(
                engine.is_none(),
                "pill {}: the game's misses, the engine's hits {engine:?}",
                fields[0]
            );
        }
        compared += 1;
    }
    eprintln!(
        "moving spheres: {compared} compared, {starting_in} starting in, {at_edges} at edges, \
         {edges_passed} edges the game's passes through"
    );
    assert!(
        compared >= 300 && at_edges <= 20 && edges_passed <= 20,
        "{compared} compared, {at_edges} at edges, {edges_passed} edges passed"
    );
    Ok(())
}
