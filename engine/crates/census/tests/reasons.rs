//! The real tree's reasons: what the census classes as done by a library,
//! and as the editor and tools.

mod common;

use opence_census::classes::Class;
use opence_census::Census;

type Failure = Box<dyn std::error::Error>;

/// The functions of `files` that are neither Replaced nor in `kept`.
fn not_replaced(census: &Census, files: &[&str], kept: &[&str]) -> Vec<String> {
    census
        .functions
        .iter()
        .filter(|f| files.contains(&f.file.as_str()))
        .filter(|f| f.class != Some(Class::Replaced) && !kept.contains(&f.name.as_str()))
        .map(|f| format!("{} ({}): {:?}", f.name, f.file, f.class))
        .collect()
}

#[test]
fn the_collision_bsp_walk_is_done_by_jolt() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        eprintln!("skipped: no source/");
        return Ok(());
    };
    let census = Census::read(&root)?;
    let left = not_replaced(
        &census,
        &[
            "physics/collision_bsp.c",
            "physics/bsp2d.c",
            "physics/bsp3d.c",
        ],
        // (the point test is the engine's own, matched to the original)
        &["bsp3d_test_point"],
    );
    assert!(left.is_empty(), "{left:#?}");
    Ok(())
}

/// The original's debug functions outside its debug files, each a debug
/// draw, a console command or a debug pick.
const DEBUG: &[&str] = &[
    "render_debug_recording",
    "device_render_debug",
    "players_debug_render",
    "debug_player_teleport",
    "hs_parse_debug_string",
    "hs_evaluate_debug_string",
    "render_debug_profile_stall_tick",
    "render_debug_profile",
    "ui_widget_debug_show_path",
    "random_seed_debug_log",
    "damage_render_debug",
    "render_debug_object_damage",
    "object_type_render_debug",
    "debug_pvs",
    "biped_render_debug",
    "units_debug_get_next_unit",
    "units_debug_get_closest_unit",
    "unit_debug_ninja_rope",
    "unit_render_debug",
    "vehicle_render_debug",
    "debug_sound_classes_set_distances_evaluate",
    "ai_debug_vocalize_evaluate",
    "debug_sound_classes_enable_evaluate",
    "ui_debug_targets_enabled",
    "ui_debug_draw_outline",
    "ui_debug_draw_mark",
    "ui_debug_draw_targets",
    "ui_debug_set_mark",
    "main_frame_rate_debug",
    "object_render_debug",
];

#[test]
fn the_console_and_the_debug_functions_are_the_tools() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        eprintln!("skipped: no source/");
        return Ok(());
    };
    let census = Census::read(&root)?;
    let mut left = not_replaced(
        &census,
        &["main/console.c", "interface/terminal.c", "game/cheats.c"],
        &[],
    );
    left.extend(
        census
            .functions
            .iter()
            .filter(|f| DEBUG.contains(&f.name.as_str()) && f.class != Some(Class::Replaced))
            .map(|f| format!("{}: {:?}", f.name, f.class)),
    );
    assert!(left.is_empty(), "{left:#?}");
    Ok(())
}
