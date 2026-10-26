//! The census report's named equivalents: functions of the original the
//! engine has its own version of but did not cite, now cited and classed.

mod common;

use opence_census::classes::Class;
use opence_census::Census;

type Failure = Box<dyn std::error::Error>;

/// The equivalents, each with the engine's place for it.
const EQUIVALENTS: &[&str] = &[
    "actor_new",
    "actor_update",
    "ai_conversation_begin",
    "ai_conversation_line_perform",
    "ai_conversation_update",
    "encounter_create",
    "first_person_weapons_update",
    "game_statistics_record_kill",
    "hs_compile_expression",
    "hud_messaging_update",
    "main_loop",
    "observer_update",
    "player_examine_nearby_item",
    "player_handle_action",
    "player_new",
    "player_spawn",
    "players_update_before_game",
    "progress_bar_render",
    "recorded_animation_play_internal",
    "unit_new",
    "unit_update",
    "units_update",
    "weapon_ready",
];

#[test]
fn the_named_equivalents_are_cited_and_classed() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        return Ok(());
    };
    let census = Census::read(&root)?;
    let mut lacking = Vec::new();
    for name in EQUIVALENTS {
        match census.function(name) {
            None => lacking.push(format!("{name}: not in the census")),
            Some(function) if function.cited.is_empty() => {
                lacking.push(format!("{name}: not cited"));
            }
            Some(function) if matches!(function.class, None | Some(Class::Missing)) => {
                lacking.push(format!("{name}: still Missing"));
            }
            Some(_) => {}
        }
    }
    assert!(lacking.is_empty(), "{lacking:#?}");
    Ok(())
}
