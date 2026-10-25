//! Where a weapon's effects start on bloodgulch: at its markers, the
//! first-person model's in the first person and the held weapon's in the
//! third, and the casings leave from the ejection port. Passes without
//! assets/maps/bloodgulch.map.

use std::error::Error;
use std::sync::Arc;

use opence_content::Level;
use opence_import::testing::stock_level;
use opence_sim::animation::{self, AnimationPlayer};
use opence_sim::debris::Ejection;
use opence_sim::input::{Buttons, TickInput};
use opence_sim::player::View;
use opence_sim::{Simulation, Vec3};

fn bloodgulch() -> Result<Option<Level>, Box<dyn Error>> {
    let Some(stock) = stock_level("bloodgulch")? else {
        return Ok(None); // (no game data here)
    };
    Ok(Some(Level::clone(&stock)))
}

/// The player settled at the first start, holding the assault rifle, its
/// biped animated as the app animates it.
fn settled(level: &Level) -> Result<Simulation, Box<dyn Error>> {
    let mut simulation = Simulation::empty();
    simulation.spawn_player(level, 0)?;
    let player = simulation.player().ok_or("no player")?;
    let (handle, unit) = (player.handle(), player.unit().clone());
    let definition = level.object(&unit).ok_or("no biped")?;
    let graph = definition.animation_graph.as_ref().ok_or("no graph")?;
    let graph = Arc::new(level.animation_graph(graph).ok_or("no graph")?.clone());
    let model = definition
        .model
        .as_ref()
        .and_then(|model| level.model_index(model))
        .map(|model| Arc::new(level.models[model].clone()));
    let standing = animation::standing_animation(&graph, true);
    assert!(simulation.animate_unit(handle, AnimationPlayer::new(graph, model, standing)));
    for _ in 0..90 {
        simulation.step(TickInput::default());
    }
    Ok(simulation)
}

/// The rifle's first shot's firing effect.
fn first_shot(simulation: &mut Simulation) -> Result<Ejection, Box<dyn Error>> {
    simulation.step(TickInput {
        buttons: Buttons::FIRE,
        ..TickInput::default()
    });
    simulation
        .effects()
        .iter()
        .find(|ejection| location(ejection, "primary trigger").is_some())
        .cloned()
        .ok_or_else(|| "no firing effect at the trigger".into())
}

fn location(ejection: &Ejection, name: &str) -> Option<Vec3> {
    ejection
        .locations
        .iter()
        .find(|(location, _, _)| location == name)
        .map(|&(_, position, _)| position)
}

#[test]
fn the_first_person_rifle_fires_from_its_muzzle() -> Result<(), Box<dyn Error>> {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    let mut simulation = settled(&level)?;
    let shot = first_shot(&mut simulation)?;
    let camera = simulation.camera().transform;
    let (eye, ahead) = (camera.position, camera.rotation * Vec3::X);
    let trigger = location(&shot, "primary trigger").ok_or("no trigger")?;
    let ejection = location(&shot, "primary ejection").ok_or("no ejection port")?;
    assert_eq!(shot.position, trigger);
    for at in [trigger, ejection] {
        assert!(at.distance(eye) < 1.0, "{} from the eye", at.distance(eye));
        assert!((at - eye).dot(ahead) > 0.0, "{at} behind the eye {eye}");
    }
    // (the muzzle is ahead of the port it ejects from)
    assert!(
        (trigger - eye).dot(ahead) > (ejection - eye).dot(ahead) + 0.05,
        "the trigger {trigger} and the port {ejection}"
    );
    Ok(())
}

#[test]
fn the_third_person_rifle_fires_from_its_hands() -> Result<(), Box<dyn Error>> {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    let mut simulation = settled(&level)?;
    assert!(simulation.set_player_view(View::ThirdPerson));
    simulation.step(TickInput::default());
    let shot = first_shot(&mut simulation)?;
    let player = simulation.player().ok_or("no player")?;
    let (feet, (yaw, _)) = (player.mover().position, player.look());
    let ahead = Vec3::new(libm::cosf(yaw.0), libm::sinf(yaw.0), 0.0);
    let trigger = location(&shot, "primary trigger").ok_or("no trigger")?;
    let ejection = location(&shot, "primary ejection").ok_or("no ejection port")?;
    for at in [trigger, ejection] {
        // (in the hands: off the ground, by the biped)
        assert!(
            at.distance(feet) < 1.5,
            "{} from the feet",
            at.distance(feet)
        );
        assert!(at.z > feet.z + 0.2, "{at} by the feet {feet}");
    }
    assert!(
        (trigger - feet).dot(ahead) > (ejection - feet).dot(ahead),
        "the trigger {trigger} and the port {ejection}"
    );
    Ok(())
}

#[test]
fn casings_leave_from_the_ejection_port() -> Result<(), Box<dyn Error>> {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    let mut simulation = settled(&level)?;
    let shot = first_shot(&mut simulation)?;
    let port = location(&shot, "primary ejection").ok_or("no ejection port")?;
    let casing = simulation
        .debris()
        .pieces()
        .first()
        .ok_or("no casing")?
        .pose
        .position;
    // (a tick's flight from the port)
    assert!(
        casing.distance(port) < 0.25,
        "{} from the port",
        casing.distance(port)
    );
    Ok(())
}
