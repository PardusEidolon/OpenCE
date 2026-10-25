//! Collision with the world on bloodgulch: a placed piece of scenery is in
//! the level's collision (the original's vehicle and biped collision tests
//! take scenery with the structure), so a Warthog driven at it is held
//! back and a player walking into it is turned aside or climbs its flank,
//! as far as the biped's slope allows, rather than passing through; and a
//! Warthog driven at a crate shoves it ahead. The simulation is
//! deterministic, so each run with scenery is compared with the same run
//! on open ground, which it would match exactly if the scenery were not
//! there. Passes without assets/maps/bloodgulch.map.

use std::error::Error;

use opence_content::{Level, ObjectKind, TagRef, VehicleSeat};
use opence_import::testing::stock_level;
use opence_sim::input::{Buttons, TickInput};
use opence_sim::objects::{ObjectHandle, ObjectSpawn};
use opence_sim::units::Radians;
use opence_sim::{clock, Quat, Simulation, Transform, Vec3};

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

const WARTHOG: &str = "vehicles\\warthog\\warthog";

fn bloodgulch() -> Result<Option<Level>> {
    let Some(stock) = stock_level("bloodgulch")? else {
        return Ok(None); // (no game data here)
    };
    Ok(Some(Level::clone(&stock)))
}

/// The scenery definition whose collision is widest at its narrowest, and
/// at least two units across and two tall (bloodgulch's large grey
/// boulder): its tag, how far it reaches from its origin across the ground,
/// and how far its foot is below its origin.
fn wide_scenery(level: &Level) -> Result<(TagRef, f32, f32)> {
    level
        .objects
        .iter()
        .filter(|object| object.kind == ObjectKind::Scenery)
        .filter_map(|object| {
            let collision = level.collision_model(object.collision_model.as_ref()?)?;
            let points: Vec<Vec3> = collision
                .nodes
                .iter()
                .flat_map(|node| &node.bsps)
                .flat_map(|bsp| &bsp.vertices)
                .map(|vertex| Vec3::from_array(vertex.point))
                .collect();
            let low = points.iter().copied().reduce(Vec3::min)?;
            let high = points.iter().copied().reduce(Vec3::max)?;
            let size = high - low;
            let across = size.x.min(size.y);
            (size.z > 2.0 && across > 2.0).then(|| {
                let reach = low.truncate().abs().max(high.truncate().abs()).length();
                (object.tag.clone(), reach, -low.z, across)
            })
        })
        .max_by(|a, b| a.3.total_cmp(&b.3))
        .map(|(tag, reach, foot, _)| (tag, reach, foot))
        .ok_or_else(|| "no wide scenery".into())
}

fn place(
    simulation: &mut Simulation,
    kind: ObjectKind,
    tag: TagRef,
    position: Vec3,
    yaw: f32,
) -> Result<ObjectHandle> {
    Ok(simulation.spawn(ObjectSpawn {
        kind,
        tag,
        transform: Transform {
            position,
            rotation: Quat::from_rotation_z(yaw),
            scale: 1.0,
        },
        name: None,
        permutation: None,
        team: None,
    })?)
}

/// The player settled at the first start, in the driver's seat of a
/// Warthog: the simulation, the Warthog, and the way the driver looks.
fn driving(level: &Level) -> Result<(Simulation, ObjectHandle, Vec3)> {
    let mut simulation = Simulation::empty();
    simulation.spawn_player(level, 0)?;
    for _ in 0..90 {
        simulation.step(TickInput::default());
    }
    let (feet, yaw) = {
        let player = simulation.player().ok_or("no player")?;
        (player.mover().position, player.look().0 .0)
    };
    let tag = level
        .objects
        .iter()
        .filter(|object| object.kind == ObjectKind::Vehicle)
        .map(|object| object.tag.clone())
        .find(|tag| tag.path.eq_ignore_ascii_case(WARTHOG))
        .ok_or("no warthog")?;
    let turn = Quat::from_rotation_z(yaw);
    let warthog = place(
        &mut simulation,
        ObjectKind::Vehicle,
        tag,
        feet + turn * Vec3::new(5.0, 0.0, 0.6),
        yaw,
    )?;
    for _ in 0..2 * clock::TICKS_PER_SECOND {
        simulation.step(TickInput::default());
    }
    let driver = level
        .objects
        .iter()
        .find(|object| object.tag.path.eq_ignore_ascii_case(WARTHOG))
        .and_then(|object| object.vehicle.as_ref())
        .and_then(|vehicle| {
            vehicle
                .seats
                .iter()
                .position(|seat| seat.flags & VehicleSeat::DRIVER != 0)
        })
        .ok_or("no driver's seat")?;
    for _ in 0..10 * clock::TICKS_PER_SECOND {
        let (Some(player), Some(target)) = (
            simulation.player(),
            simulation.rides().entrance(warthog, driver),
        ) else {
            return Err("no player or seat".into());
        };
        let to = target - player.mover().position;
        let distance = to.truncate().length();
        if distance < 0.2 {
            break;
        }
        let heading = libm::atan2f(to.y, to.x);
        let turn = libm::remainderf(heading - player.look().0 .0, std::f32::consts::TAU);
        simulation.step(TickInput {
            yaw: Radians(turn),
            forward: if distance < 0.5 { 40 } else { 127 },
            ..TickInput::default()
        });
    }
    simulation.step(TickInput::default());
    simulation.step(TickInput {
        buttons: Buttons::ACTION,
        ..TickInput::default()
    });
    for _ in 0..clock::TICKS_PER_SECOND {
        simulation.step(TickInput::default());
    }
    let player = simulation.player().ok_or("no player")?;
    if player.seated().map(|seated| seated.seat) != Some(driver) {
        return Err("the player never took the driver's seat".into());
    }
    let yaw = player.look().0 .0;
    Ok((
        simulation,
        warthog,
        Vec3::new(libm::cosf(yaw), libm::sinf(yaw), 0.0),
    ))
}

/// Drives `seconds` forward: how far the Warthog went along `look`, and
/// where its centre was each tick.
fn drive(
    simulation: &mut Simulation,
    warthog: ObjectHandle,
    look: Vec3,
    seconds: u32,
) -> Result<(f32, Vec<Vec3>)> {
    let start = simulation.transform(warthog).ok_or("no warthog")?.position;
    let mut path = Vec::new();
    for _ in 0..seconds * clock::TICKS_PER_SECOND {
        simulation.step(TickInput {
            forward: 127,
            ..TickInput::default()
        });
        path.push(simulation.transform(warthog).ok_or("no warthog")?.position);
    }
    let end = simulation.transform(warthog).ok_or("no warthog")?.position;
    Ok(((end - start).dot(look), path))
}

/// Where `path` first leaves `open` by more than a hundredth of a unit, as
/// a distance along `look` from `start`, and the most it leaves it by.
fn parting(path: &[Vec3], open: &[Vec3], start: Vec3, look: Vec3) -> (Option<f32>, f32) {
    let apart = |(a, b): (&Vec3, &Vec3)| (*a - *b).length();
    let first = path
        .iter()
        .zip(open)
        .find(|pair| apart(*pair) > 0.01)
        .map(|(point, _)| (*point - start).dot(look));
    let most = path.iter().zip(open).map(apart).fold(0.0, f32::max);
    (first, most)
}

/// The lowest height on `path` within `reach` of `at` along `look` (where
/// scenery standing at `at` meets the ground the way went over).
fn lowest(path: &[Vec3], start: Vec3, look: Vec3, at: f32, reach: f32) -> Result<f32> {
    path.iter()
        .filter(|point| ((**point - start).dot(look) - at).abs() <= reach)
        .map(|point| point.z)
        .reduce(f32::min)
        .ok_or_else(|| "the way never got there".into())
}

#[test]
fn a_warthog_driven_at_scenery_is_held_back_by_it() -> Result {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    let (scenery, reach, foot) = wide_scenery(&level)?;
    // (on open ground it gets well past where the scenery will stand)
    let (mut open, warthog, look) = driving(&level)?;
    let ahead = 10.0 + reach;
    let start = open.transform(warthog).ok_or("no warthog")?.position;
    let (free, path) = drive(&mut open, warthog, look, 5)?;
    assert!(free > ahead + reach, "it went {free} on open ground");
    let centre = lowest(&path, start, look, ahead, reach)?;

    let (mut blocked, warthog, look) = driving(&level)?;
    let at = blocked.transform(warthog).ok_or("no warthog")?.position + look * ahead;
    // (its foot a unit below the lowest the Warthog's centre went beside
    // it, down in the ground)
    let at = Vec3::new(at.x, at.y, centre + foot - 1.0);
    place(&mut blocked, ObjectKind::Scenery, scenery, at, 0.0)?;
    let (went, way) = drive(&mut blocked, warthog, look, 5)?;
    let (first, _) = parting(&way, &path, start, look);
    // (it meets the scenery's near side, and loses at least its width)
    let met = first.ok_or("the scenery never touched it")?;
    assert!(
        met < ahead,
        "it first touched the scenery {met} along, past its middle {ahead} ahead"
    );
    assert!(
        went < free - 2.0 * reach,
        "it went {went} against {free} on open ground, through the scenery"
    );
    Ok(())
}

/// The player settled at the first start, walking `seconds` the way they
/// look, with `scenery` (if any) placed: the way they
/// look, where they started, and where their feet were each tick.
fn walk(
    level: &Level,
    seconds: u32,
    scenery: Option<(TagRef, Vec3)>,
) -> Result<(Vec3, Vec3, Vec<Vec3>)> {
    let mut simulation = Simulation::empty();
    simulation.spawn_player(level, 0)?;
    for _ in 0..90 {
        simulation.step(TickInput::default());
    }
    let (feet, yaw) = {
        let player = simulation.player().ok_or("no player")?;
        (player.mover().position, player.look().0 .0)
    };
    let look = Vec3::new(libm::cosf(yaw), libm::sinf(yaw), 0.0);
    if let Some((tag, at)) = scenery {
        place(&mut simulation, ObjectKind::Scenery, tag, at, 0.0)?;
    }
    let mut path = Vec::new();
    for _ in 0..seconds * clock::TICKS_PER_SECOND {
        simulation.step(TickInput {
            forward: 127,
            ..TickInput::default()
        });
        path.push(simulation.player().ok_or("no player")?.mover().position);
    }
    Ok((look, feet, path))
}

#[test]
fn a_player_walking_into_scenery_goes_aside_or_over_it() -> Result {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    let (scenery, reach, foot) = wide_scenery(&level)?;
    // (on open ground they get well past where the scenery will stand, and
    // the ground there, which falls away ahead, is found)
    let ahead = 3.0 + reach;
    let (look, feet, path) = walk(&level, 8, None)?;
    let free = path.last().map_or(0.0, |end| (*end - feet).dot(look));
    assert!(free > ahead + reach, "they walked {free} on open ground");
    let ground = lowest(&path, feet, look, ahead, reach)?;

    let at = feet + look * ahead;
    // (its foot half a unit down in the ground)
    let at = Vec3::new(at.x, at.y, ground + foot - 0.5);
    let (_, _, way) = walk(&level, 8, Some((scenery, at)))?;
    let (first, most) = parting(&way, &path, feet, look);
    // (they meet its near side, and are moved off their way by more than
    // a unit: aside, or up its flank)
    let met = first.ok_or("the scenery never touched them")?;
    assert!(
        met < ahead,
        "they first touched the scenery {met} along, past its middle {ahead} ahead"
    );
    assert!(
        most > 1.0,
        "they kept within {most} of their way on open ground, through the scenery"
    );
    Ok(())
}

#[test]
fn a_warthog_driven_at_a_crate_shoves_it_ahead() -> Result {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    let (mut simulation, warthog, look) = driving(&level)?;
    let start = simulation.transform(warthog).ok_or("no warthog")?.position;
    // (dropped well above the ground, which rises ahead, and come to rest)
    assert!(simulation.drop_crate(
        start + look * 8.0 + Vec3::Z * 4.0,
        Quat::IDENTITY,
        Vec3::splat(0.5)
    ));
    for _ in 0..3 * clock::TICKS_PER_SECOND {
        simulation.step(TickInput::default());
    }
    let dropped = simulation
        .crates()
        .all()
        .first()
        .ok_or("no crate")?
        .pose
        .position;
    drive(&mut simulation, warthog, look, 4)?;
    let crate_ = simulation.crates().all().first().ok_or("no crate")?;
    let shoved = (crate_.pose.position - dropped).dot(look);
    assert!(
        shoved > 3.0,
        "the crate went {shoved} along the way the warthog drove"
    );
    Ok(())
}

#[test]
fn a_warthog_driven_at_a_biped_knocks_it_aside_and_hurts_it() -> Result {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    let unit = level
        .globals
        .multiplayer_unit
        .clone()
        .ok_or("no multiplayer unit")?;
    let (mut simulation, warthog, look) = driving(&level)?;
    let driver = simulation.player().ok_or("no player")?.handle();
    let start = simulation.transform(warthog).ok_or("no warthog")?.position;
    // (standing in its way, dropped from above the Warthog's centre and
    // come to rest)
    let at = start + look * 8.0 + Vec3::Z;
    let (index, biped) = simulation.add_actor(&level, &unit, None, (at, 0.0), None)?;
    for _ in 0..clock::TICKS_PER_SECOND {
        simulation.step(TickInput::default());
    }
    let stood = simulation
        .actors()
        .get(index)
        .ok_or("no actor")?
        .mover()
        .position;
    let mut hurt = false;
    for _ in 0..5 * clock::TICKS_PER_SECOND {
        simulation.step(TickInput {
            forward: 127,
            ..TickInput::default()
        });
        hurt |= simulation
            .damage_events()
            .iter()
            .any(|event| event.object == biped && event.owner == Some(driver));
    }
    let moved = simulation
        .actors()
        .get(index)
        .ok_or("no actor")?
        .mover()
        .position
        - stood;
    // (thrown off where it stood, and hurt by the driver, as
    // `physics_compute_vehicle_collision` has it: the hurt kills it, so it
    // goes no further than the first shoves throw it)
    assert!(moved.length() > 1.0, "the biped moved only {moved:?}");
    assert!(hurt, "the biped was never hurt by the driver");
    Ok(())
}
