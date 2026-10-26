//! The dead player's camera on bloodgulch, as `dead_camera.c` places it:
//! once the player dies, the camera looks at its body's bounding sphere
//! from half a unit away, then over three seconds eases out to between
//! two and six units, looking down at it by 27 to 63 degrees, and keeps
//! looking at it as the body falls. Passes without
//! assets/maps/bloodgulch.map.

use std::error::Error;
use std::sync::Arc;

use opence_content::{Level, ObjectKind};
use opence_import::testing::stock_level;
use opence_sim::animation::{AnimationPlayer, NodeMatrices};
use opence_sim::damage::Vitality;
use opence_sim::input::{Buttons, TickInput};
use opence_sim::objects::ObjectHandle;
use opence_sim::projectile::Detonation;
use opence_sim::units::Radians;
use opence_sim::{clock, Simulation, Vec3};

fn bloodgulch() -> Result<Option<Level>, Box<dyn Error>> {
    let Some(stock) = stock_level("bloodgulch")? else {
        return Ok(None); // (no game data here)
    };
    Ok(Some(Level::clone(&stock)))
}

/// The player settled at the first start, and the detonation of a frag
/// it threw (a grenade's explosion to script).
fn settled(level: &Level) -> Result<(Simulation, Detonation), Box<dyn Error>> {
    let mut simulation = Simulation::empty();
    simulation.spawn_player(level, 0)?;
    simulation.step(TickInput {
        yaw: Radians(std::f32::consts::PI),
        ..TickInput::default()
    });
    for _ in 0..90 {
        simulation.step(TickInput::default());
    }
    assert!(simulation.give_grenades(0, 1));
    simulation.step(TickInput {
        buttons: Buttons::GRENADE,
        ..TickInput::default()
    });
    for _ in 0..8 * clock::TICKS_PER_SECOND {
        simulation.step(TickInput::default());
        if let Some(detonation) = simulation.detonations().first() {
            let detonation = detonation.clone();
            return Ok((simulation, detonation));
        }
    }
    Err("the frag never detonated".into())
}

/// The centre of `object`'s bounding sphere, from its root node
/// (`bounding_sphere_center`).
fn centre(
    simulation: &Simulation,
    level: &Level,
    object: ObjectHandle,
) -> Result<Vec3, Box<dyn Error>> {
    let tag = &simulation.object(object).ok_or("no object")?.tag;
    let offset = level.object(tag).ok_or("no definition")?.bounding_offset;
    let nodes = simulation
        .component::<NodeMatrices>(object)
        .ok_or("no nodes")?;
    let root = nodes.current.first().ok_or("no root node")?;
    Ok(Vec3::from_array(root.transform_point(offset)))
}

/// Animates the player's biped as the app does.
fn animate(simulation: &mut Simulation, level: &Level) -> Result<(), Box<dyn Error>> {
    let body = simulation.player().ok_or("no player")?.handle();
    let tag = simulation.object(body).ok_or("no object")?.tag.clone();
    let definition = level.object(&tag).ok_or("no definition")?;
    let model = definition
        .model
        .as_ref()
        .and_then(|model| level.model_index(model))
        .and_then(|model| level.models.get(model))
        .ok_or("no model")?;
    let graph = definition
        .animation_graph
        .as_ref()
        .and_then(|graph| level.animation_graph(graph))
        .ok_or("no animations")?;
    let player = AnimationPlayer::for_object(
        Arc::new(graph.clone()),
        Some(Arc::new(model.clone())),
        ObjectKind::Biped,
        0.5,
    );
    assert!(simulation.animate_unit(body, player));
    for _ in 0..10 {
        simulation.step(TickInput::default());
    }
    Ok(())
}

fn dead(simulation: &Simulation, object: ObjectHandle) -> bool {
    simulation
        .component::<Vitality>(object)
        .is_some_and(|vitality| vitality.dead)
}

/// How far `point` is from the camera's line of sight, and how far along
/// it.
fn sighted(simulation: &Simulation, point: Vec3) -> (f32, f32) {
    let camera = simulation.camera().transform;
    let forward = camera.rotation * Vec3::X;
    let along = (point - camera.position).dot(forward);
    let off = (point - camera.position - forward * along).length();
    (off, along)
}

#[test]
fn the_dead_players_camera_eases_out_and_follows_its_body() -> Result<(), Box<dyn Error>> {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    let (mut simulation, grenade) = settled(&level)?;
    animate(&mut simulation, &level)?;
    let body = simulation.player().ok_or("no player")?.handle();
    let eyes = simulation.camera().transform;

    // (scripted frags at its feet until it dies)
    for _ in 0..5 {
        if dead(&simulation, body) {
            break;
        }
        let feet = simulation.player().ok_or("no player")?.mover().position;
        simulation.detonate(Detonation {
            point: feet + Vec3::new(0.3, 0.0, 0.1),
            attached: None,
            attached_damage: None,
            ..grenade.clone()
        });
        simulation.step(TickInput::default());
    }
    assert!(dead(&simulation, body), "the frags never killed it");
    assert!(simulation.ragdolls().of(body).is_some(), "no ragdoll");
    let died = centre(&simulation, &level, body)?;

    // (the tick it died: half a unit from the body, still looking as its
    // eyes looked)
    let (off, along) = sighted(&simulation, centre(&simulation, &level, body)?);
    let forward = simulation.camera().transform.rotation * Vec3::X;
    assert!(off < 1e-3, "{off}");
    assert!((along - 0.5).abs() < 1e-3, "{along}");
    assert!(forward.dot(eyes.rotation * Vec3::X) > 0.99, "{forward}");

    // (three seconds out, ever further and never past six; the dead turn
    // nothing, however they press)
    let mut last = along;
    for tick in 0..3 * clock::TICKS_PER_SECOND {
        simulation.step(TickInput {
            yaw: Radians(0.1),
            pitch: Radians(0.1),
            ..TickInput::default()
        });
        let (off, along) = sighted(&simulation, centre(&simulation, &level, body)?);
        assert!(off < 1e-3, "tick {tick}: {off} off the body");
        assert!(along >= last - 1e-4, "tick {tick}: {along} after {last}");
        assert!(along <= 6.0 + 1e-3, "tick {tick}: {along}");
        last = along;
    }
    let settled = simulation.camera().transform;
    let forward = settled.rotation * Vec3::X;
    let down = -libm::asinf(forward.z.clamp(-1.0, 1.0));
    assert!(
        (0.471_238_94 - 1e-3..=1.099_557_4 + 1e-3).contains(&down),
        "pitch {down}"
    );
    // (at its distance, unless the ground stood in the way)
    assert!(last <= 6.0 + 1e-3 && last > 0.5, "{last}");
    assert!(settled.position.z > centre(&simulation, &level, body)?.z);

    // (ten seconds more: it keeps that look, and its sight on the body as
    // the body moves)
    let mut moved = centre(&simulation, &level, body)?.distance(died);
    for tick in 0..10 * clock::TICKS_PER_SECOND {
        simulation.step(TickInput::default());
        let at = centre(&simulation, &level, body)?;
        moved = moved.max(at.distance(died));
        let (off, along) = sighted(&simulation, at);
        assert!(off < 1e-3, "tick {tick}: {off} off the body");
        assert!(along <= last + 1e-3, "tick {tick}: {along} past {last}");
        let now = simulation.camera().transform.rotation * Vec3::X;
        assert!(now.dot(forward) > 0.9999, "tick {tick}: turned to {now}");
    }
    // (the blast threw it: there was a body to follow)
    assert!(moved > 0.25, "the body moved only {moved}");
    Ok(())
}

#[test]
fn a_respawned_player_sees_from_its_eyes_again() -> Result<(), Box<dyn Error>> {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    let (mut simulation, _) = settled(&level)?;
    animate(&mut simulation, &level)?;
    let body = simulation.player().ok_or("no player")?.handle();
    assert!(simulation.kill(body));
    for _ in 0..clock::TICKS_PER_SECOND {
        simulation.step(TickInput::default());
    }
    let (off, along) = sighted(&simulation, centre(&simulation, &level, body)?);
    assert!(off < 1e-3 && along > 0.5, "{off} {along}");

    simulation
        .respawn_player(0, &level, 0, None)
        .ok_or("no player")??;
    simulation.step(TickInput::default());
    let player = simulation.player().ok_or("no player")?;
    let eyes = player.mover().position;
    let camera = simulation.camera().transform.position;
    assert!(
        (camera.x - eyes.x).abs() < 1e-3 && (camera.y - eyes.y).abs() < 1e-3,
        "{camera} {eyes}"
    );
    assert!(
        camera.z > eyes.z && camera.z < eyes.z + 1.0,
        "{camera} {eyes}"
    );
    Ok(())
}
