//! The scripts' cinematics, camera, HUD and object natives, as
//! `hs_library_cinematics.c`, `camera_scripting.c`, `hud_messaging.c` and
//! `hs_library_external.c` keep them: a cinematic letterboxes and titles
//! until it stops, the scripted camera moves between its points over the
//! ticks asked, the HUD timer counts down only while it runs, and the
//! objects are teleported, turned, made and destroyed by name and seen
//! within an angle (skipped where there is no game data).

use std::error::Error;
use std::sync::Arc;

use opence_campaign::Mission;
use opence_import::testing::stock_level_unheld;
use opence_script::Value;
use opence_sim::input::TickInput;
use opence_sim::objects::Difficulty;
use opence_sim::{Simulation, Vec3};

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

/// The mission of `map` with its player, if the map is here.
fn mission(map: &str) -> Result<Option<Mission>> {
    let Some(stock) = stock_level_unheld(map)? else {
        return Ok(None);
    };
    let level = Arc::new(stock);
    let mut sim = Simulation::empty();
    sim.spawn_player(&level, 0)?;
    Ok(Some(Mission::new(sim, level, Difficulty::Normal)))
}

/// Runs `ticks` of the scripts' 30 Hz: two of the simulation's each.
fn steps(mission: &mut Mission, ticks: u32) {
    for _ in 0..2 * ticks {
        mission.step(TickInput::default());
    }
}

fn close(a: Vec3, b: Vec3) -> bool {
    (a - b).length() < 1.0e-3
}

#[test]
fn a_cinematic_letterboxes_titles_and_skips_until_it_stops() -> Result {
    let Some(mut mission) = mission("a10")? else {
        return Ok(()); // (no game data here)
    };
    mission.call("cinematic_start", &[]);
    assert!(mission.cinematic().playing && mission.cinematic().letterbox);
    mission.call("cinematic_show_letterbox", &[Value::Boolean(false)]);
    assert!(!mission.cinematic().letterbox, "letterbox hidden");
    mission.call("cinematic_set_title", &[Value::Index(0)]);
    assert_eq!(mission.cinematic().title, Some(0));
    mission.call("cinematic_skip_start_internal", &[]);
    assert!(mission.cinematic().skippable, "may be skipped");
    mission.call("cinematic_skip_stop_internal", &[]);
    assert!(!mission.cinematic().skippable, "may no longer be skipped");
    mission.call("cinematic_stop", &[]);
    let cinematic = mission.cinematic();
    assert!(!cinematic.playing && !cinematic.letterbox && cinematic.title.is_none());
    Ok(())
}

#[test]
fn the_scripted_camera_moves_between_its_points_over_the_ticks_asked() -> Result {
    let Some(mut mission) = mission("a10")? else {
        return Ok(()); // (no game data here)
    };
    let points = mission.level().scenario.campaign.camera_points.clone();
    let [first, second, ..] = &points[..] else {
        return Err("fewer than two camera points".into());
    };
    let (from, to) = (Vec3::from(first.position), Vec3::from(second.position));
    assert_eq!(mission.cinematic_camera(), None, "the player's camera");

    mission.call("camera_control", &[Value::Boolean(true)]);
    mission.call("camera_set", &[Value::Index(0), Value::Short(0)]);
    let (at, ..) = mission.cinematic_camera().ok_or("no camera")?;
    assert!(close(at, from), "at its first point at once");

    mission.call("camera_set", &[Value::Index(1), Value::Short(30)]);
    assert_eq!(mission.call("camera_time", &[]), Some(Value::Short(30)));
    steps(&mut mission, 15);
    assert_eq!(mission.call("camera_time", &[]), Some(Value::Short(15)));
    let (at, ..) = mission.cinematic_camera().ok_or("no camera")?;
    assert!(close(at, from.lerp(to, 0.5)), "halfway at half its ticks");
    steps(&mut mission, 15);
    let (at, _, fov) = mission.cinematic_camera().ok_or("no camera")?;
    assert!(
        close(at, to) && fov == second.field_of_view,
        "there at the end"
    );

    let players = mission.call("players", &[]).ok_or("no players")?;
    let player = mission.sim().player().ok_or("no player")?.handle();
    let held = Value::Index(mission.index_of(player));
    mission.call(
        "camera_set_relative",
        &[Value::Index(1), Value::Short(0), held],
    );
    let origin = mission.sim().transform(player).ok_or("no player")?.position;
    let (at, ..) = mission.cinematic_camera().ok_or("no camera")?;
    assert!(close(at, origin + to), "measured from the object");

    mission.call("camera_set_first_person", std::slice::from_ref(&players));
    assert_eq!(mission.cinematic_camera(), None, "the first person's again");
    mission.call("camera_set", &[Value::Index(0), Value::Short(0)]);
    mission.call("camera_set_dead", &[players]);
    assert_eq!(mission.cinematic_camera(), None, "the dead camera's");
    mission.call("camera_control", &[Value::Boolean(false)]);
    assert_eq!(mission.cinematic_camera(), None);
    Ok(())
}

#[test]
fn the_hud_timer_counts_down_only_while_it_runs() -> Result {
    let Some(mut mission) = mission("a10")? else {
        return Ok(()); // (no game data here)
    };
    mission.call("show_hud", &[Value::Boolean(false)]);
    mission.call("show_hud_help_text", &[Value::Boolean(false)]);
    assert!(!mission.flow().hud && !mission.flow().help_text);

    mission.call("hud_set_timer_time", &[Value::Short(1), Value::Short(30)]);
    assert_eq!(
        mission.call("hud_get_timer_ticks", &[]),
        Some(Value::Short(90 * 30))
    );
    steps(&mut mission, 10);
    assert_eq!(mission.flow().timer, 90 * 30, "paused from the start");
    mission.call("pause_hud_timer", &[Value::Boolean(false)]);
    steps(&mut mission, 10);
    assert_eq!(mission.flow().timer, 90 * 30 - 10, "runs");
    mission.call("pause_hud_timer", &[Value::Boolean(true)]);
    steps(&mut mission, 10);
    assert_eq!(mission.flow().timer, 90 * 30 - 10, "paused again");

    assert_eq!(
        mission.call("not", &[Value::Boolean(true)]),
        Some(Value::Boolean(false))
    );
    mission.call("print", &[Value::String("hello".into())]);
    assert_eq!(mission.take_printed(), ["hello"]);
    assert_eq!(
        mission.call("player0_look_pitch_is_inverted", &[]),
        Some(Value::Boolean(false))
    );
    assert_eq!(
        mission.call("player0_joystick_set_is_normal", &[]),
        Some(Value::Boolean(true))
    );
    Ok(())
}

#[test]
fn objects_are_teleported_turned_seen_and_made_and_destroyed_by_name() -> Result {
    let Some(mut mission) = mission("a10")? else {
        return Ok(()); // (no game data here)
    };
    let flags = mission.level().scenario.campaign.cutscene_flags.clone();
    let first = flags.first().ok_or("no cutscene flags")?;
    let player = mission.sim().player().ok_or("no player")?.handle();
    let held = Value::Index(mission.index_of(player));

    mission.call("object_teleport", &[held.clone(), Value::Index(0)]);
    let at = mission.sim().transform(player).ok_or("no player")?;
    assert!(
        close(at.position, Vec3::from(first.position)),
        "at the flag"
    );

    // Seen by the players within the angle of where they look.
    let players = mission.call("players", &[]).ok_or("no players")?;
    let (yaw, pitch) = mission.sim().player().ok_or("no player")?.look();
    let look = Vec3::new(
        libm::cosf(yaw.0) * libm::cosf(pitch.0),
        libm::sinf(yaw.0) * libm::cosf(pitch.0),
        libm::sinf(pitch.0),
    );
    let eye = mission.sim().transform(player).ok_or("no player")?.position;
    for (index, flag) in flags.iter().enumerate() {
        let toward = (Vec3::from(flag.position) - eye).normalize_or_zero();
        let flag_index = Value::Index(i32::try_from(index)?);
        for degrees in [45.0_f32, 90.0, 180.0] {
            let seen = toward != Vec3::ZERO && look.dot(toward) >= libm::cosf(degrees.to_radians());
            let answer = mission.call(
                "objects_can_see_flag",
                &[players.clone(), flag_index.clone(), Value::Real(degrees)],
            );
            assert_eq!(
                answer,
                Some(Value::Boolean(seen)),
                "flag {index} at {degrees}"
            );
        }
    }

    // Made and destroyed by a part of their names, the players never.
    let names = mission.level().scenario.campaign.object_names.clone();
    let part = names
        .iter()
        .enumerate()
        .find_map(|(name, text)| {
            let made = mission.create_named(name, false)?;
            mission.remove(made);
            Some(text.clone())
        })
        .ok_or("no name to make")?;
    let containing: Vec<usize> = (0..names.len())
        .filter(|&n| names[n].contains(&part))
        .collect();
    mission.call("object_create_containing", &[Value::String(part.clone())]);
    assert!(
        containing.iter().any(|&n| mission.named(n).is_some()),
        "made"
    );
    mission.call("object_destroy_containing", &[Value::String(part.clone())]);
    assert!(
        containing.iter().all(|&n| mission.named(n).is_none()),
        "destroyed"
    );
    mission.call("object_create_anew_containing", &[Value::String(part)]);
    assert!(
        containing.iter().any(|&n| mission.named(n).is_some()),
        "made anew"
    );
    mission.call("object_destroy_all", &[]);
    assert!(
        containing.iter().all(|&n| mission.named(n).is_none()),
        "all destroyed"
    );
    assert!(mission.sim().object(player).is_some(), "but not the player");
    Ok(())
}

#[test]
fn an_object_is_turned_to_a_flags_facing() -> Result {
    let Some(mut mission) = mission("a10")? else {
        return Ok(()); // (no game data here)
    };
    let names = mission.level().scenario.campaign.object_names.len();
    let made = (0..names)
        .find_map(|name| mission.create_named(name, false))
        .ok_or("no name to make")?;
    let flag = mission
        .level()
        .scenario
        .campaign
        .cutscene_flags
        .first()
        .cloned()
        .ok_or("no cutscene flags")?;
    let before = mission.sim().transform(made).ok_or("not made")?;
    let held = Value::Index(mission.index_of(made));
    mission.call("object_set_facing", &[held, Value::Index(0)]);
    let after = mission.sim().transform(made).ok_or("gone")?;
    assert!(
        close(after.position, before.position),
        "it stays where it is"
    );
    let [yaw, pitch] = flag.facing;
    let facing = after.rotation * Vec3::X;
    let expected = Vec3::new(
        libm::cosf(yaw) * libm::cosf(pitch),
        libm::sinf(yaw) * libm::cosf(pitch),
        libm::sinf(pitch),
    );
    assert!(
        (facing - expected).length() < 1.0e-3,
        "faces the flag's way"
    );

    // The players see the object as they would a point where it stands.
    let player = mission.sim().player().ok_or("no player")?.handle();
    let (yaw, pitch) = mission.sim().player().ok_or("no player")?.look();
    let look = Vec3::new(
        libm::cosf(yaw.0) * libm::cosf(pitch.0),
        libm::sinf(yaw.0) * libm::cosf(pitch.0),
        libm::sinf(pitch.0),
    );
    let eye = mission.sim().transform(player).ok_or("no player")?.position;
    let toward = (after.position - eye).normalize_or_zero();
    let players = mission.call("players", &[]).ok_or("no players")?;
    let seen_object = Value::Index(mission.index_of(made));
    for degrees in [10.0_f32, 90.0, 180.0] {
        let seen = toward != Vec3::ZERO && look.dot(toward) >= libm::cosf(degrees.to_radians());
        assert_eq!(
            mission.call(
                "objects_can_see_object",
                &[players.clone(), seen_object.clone(), Value::Real(degrees)],
            ),
            Some(Value::Boolean(seen)),
            "seen at {degrees}"
        );
    }
    Ok(())
}

#[test]
fn a_recording_plays_for_its_length_until_it_is_killed() -> Result {
    let Some(mut mission) = mission("a10")? else {
        return Ok(()); // (no game data here)
    };
    let recordings = mission.level().scenario.campaign.recordings.clone();
    let length = recordings
        .first()
        .map(|r| r.length_in_ticks)
        .ok_or("no recordings")?;
    let player = mission.sim().player().ok_or("no player")?.handle();
    let held = Value::Index(mission.index_of(player));
    let time = |mission: &mut Mission| mission.call("recording_time", std::slice::from_ref(&held));

    assert_eq!(time(&mut mission), Some(Value::Short(0)), "none playing");
    let missing = Value::Index(i32::try_from(recordings.len())?);
    assert_eq!(
        mission.call("recording_play", &[held.clone(), missing]),
        Some(Value::Boolean(false)),
        "a recording the scenario does not have"
    );
    assert_eq!(
        mission.call("recording_play", &[held.clone(), Value::Index(0)]),
        Some(Value::Boolean(true))
    );
    let full = length;
    assert_eq!(time(&mut mission), Some(Value::Short(full)));
    steps(&mut mission, 5);
    assert_eq!(
        time(&mut mission),
        Some(Value::Short(full - 5)),
        "counts down"
    );
    mission.call("recording_kill", std::slice::from_ref(&held));
    assert_eq!(time(&mut mission), Some(Value::Short(0)), "killed");
    Ok(())
}
