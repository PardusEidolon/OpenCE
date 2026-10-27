# The coverage census

Every function of the original (`source/`), by system, with its class and
the evidence for it. `opence census --write` writes this file from
`engine/census/census.toml` and the reference traces; do not edit it.
docs/engine.md's "The coverage census" says what each class needs.

| System | Ported | Matched | Native | Partial | Replaced | Missing | Owed |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `ai` | 42 (1744 B) | 14 (1600 B) | | 17 (18128 B) | 91 (5616 B) | 795 (269178 B) | 0 |
| `bink` | | | | | 24 (3968 B) | | 0 |
| `bitmaps` | | 1 (224 B) | | | 209 (63904 B) | | 0 |
| `bungie_net` | | | | | 105 (18992 B) | | 0 |
| `cache` | | | | | 187 (24784 B) | | 0 |
| `camera` | 3 (192 B) | 18 (8144 B) | | 4 (624 B) | 17 (2448 B) | 79 (25136 B) | 0 |
| `cseries` | | 1 (16 B) | 28 (3168 B) | | 111 (18656 B) | | 0 |
| `cutscene` | | 8 (288 B) | | 2 (672 B) | 7 (1072 B) | 48 (8496 B) | 0 |
| `devices` | 7 (944 B) | 18 (3856 B) | | | 1 (432 B) | 34 (1952 B) | 0 |
| `dialogs` | | | | | 5 (1328 B) | | 0 |
| `editor` | | | | | 9 (0 B) | | 0 |
| `effects` | 1 (48 B) | 87 (33136 B) | | 2 (2176 B) | 13 (7344 B) | 87 (13904 B) | 0 |
| `game` | 47 (5264 B) | 15 (1616 B) | | 28 (13936 B) | 61 (4384 B) | 565 (78880 B) | 0 |
| `hs` | 2 (384 B) | 42 (7984 B) | | 8 (1776 B) | 11 (2880 B) | 576 (53968 B) | 0 |
| `input` | | | | | 46 (7216 B) | 1 (0 B) | 0 |
| `interface` | 6 (1408 B) | 96 (33424 B) | | 19 (6112 B) | 104 (21432 B) | 452 (94304 B) | 0 |
| `items` | 9 (4080 B) | 17 (4976 B) | | 8 (14480 B) | 1 (16 B) | 102 (12496 B) | 0 |
| `main` | | | | 1 (1648 B) | 34 (5440 B) | 91 (8752 B) | 0 |
| `math` | 44 (5552 B) | 6 (256 B) | 70 (4720 B) | | 25 (4016 B) | 64 (8624 B) | 0 |
| `memory` | | 2 (336 B) | 128 (19856 B) | | 102 (15104 B) | | 0 |
| `models` | 17 (8928 B) | 12 (4880 B) | | 1 (464 B) | 7 (1616 B) | 11 (1680 B) | 0 |
| `networking` | | | | | 288 (49824 B) | | 0 |
| `objects` | 3 (160 B) | 42 (5536 B) | | 17 (4336 B) | 59 (13424 B) | 226 (28128 B) | 0 |
| `physics` | | 14 (2672 B) | | 22 (12592 B) | 64 (25040 B) | 45 (6928 B) | 0 |
| `rasterizer` | | 2 (544 B) | | 1 (2240 B) | 777 (182416 B) | | 0 |
| `render` | 2 (592 B) | 2 (240 B) | | | 115 (37520 B) | | 0 |
| `saved games` | | | | 2 (96 B) | 31 (3712 B) | 104 (21184 B) | 0 |
| `scenario` | | 11 (4000 B) | | 2 (320 B) | 23 (3136 B) | 21 (2128 B) | 0 |
| `shaders` | | | | | 21 (2624 B) | | 0 |
| `shell` | | | | | 11 (528 B) | 2 (0 B) | 0 |
| `sound` | 8 (1312 B) | 60 (15168 B) | | 5 (672 B) | 56 (2832 B) | 43 (6976 B) | 0 |
| `structures` | | 2 (0 B) | | | 98 (27952 B) | 26 (3760 B) | 0 |
| `tag_files` | | | 43 (4288 B) | | 5 (0 B) | | 0 |
| `text` | | 5 (1536 B) | 63 (6864 B) | 4 (400 B) | 19 (5744 B) | 21 (2368 B) | 0 |
| `tool` | | | | | 27 (8752 B) | | 0 |
| `units` | 71 (11440 B) | 50 (19024 B) | | 31 (25952 B) | 9 (1616 B) | 168 (49488 B) | 0 |
| **all** | 262 (42048 B) | 525 (149456 B) | 332 (38896 B) | 174 (106624 B) | 2773 (575768 B) | 3561 (698330 B) | 0 |

## `ai`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `vector_from_points3d` | ai/action_alert.c | Ported |  | `tools/engine_math_reference.py` |
| `magnitude_squared3d` | ai/action_alert.c | Ported |  | `tools/engine_math_reference.py` |
| `distance_squared3d` | ai/action_alert.c | Ported |  | `tools/engine_math_reference.py` |
| `real_random_range` | ai/action_alert.c | Matched | audio/src/choice.rs:43 | sim/tests/random_reference.rs (trace) |
| `square_root` | ai/action_charge.c | Ported |  | `tools/engine_animation_reference.py` |
| `scale_vector2d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `magnitude_squared2d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `magnitude2d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `normalize2d` | ai/action_charge.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `dot_product2d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `point_from_line3d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `scale_vector3d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `magnitude3d` | ai/action_charge.c | Ported |  | `tools/engine_item_reference.py` |
| `normalize3d` | ai/action_charge.c | Ported |  | `tools/engine_item_reference.py` |
| `dot_product3d` | ai/action_charge.c | Ported | sim/src/vehicle.rs:828 | `tools/engine_handle_action_reference.py` |
| `subtract_vectors3d` | ai/action_charge.c | Ported |  | `tools/engine_physics_math_reference.py` |
| `real_random` | ai/action_charge.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `vector_from_points2d` | ai/action_obey.c | Ported |  | `tools/engine_math_reference.py` |
| `cross_product3d` | ai/action_obey.c | Ported |  | `tools/engine_animation_reference.py` |
| `negate_vector3d` | ai/action_obey.c | Ported |  | `tools/engine_math_reference.py` |
| `random_range` | ai/action_obey.c | Ported |  | `tools/engine_random_reference.py` |
| `point_from_line2d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `distance_squared2d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `distance2d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `negate_vector2d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `distance3d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `actor_mode_name` | ai/actions.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `actor_action_debug_color` | ai/actions.c | Replaced | dev/src/draw.rs | the editor and tools |
| `set_real_vector2d` | ai/actions.c | Ported |  | `tools/engine_math_reference.py` |
| `set_real_vector3d` | ai/actions.c | Ported |  | `tools/engine_math_reference.py` |
| `actor_action_handle_surprise` | ai/actions.c | Partial | sim/src/ai_impulse.rs:6 | sim/src/ai_impulse.rs (measure); leaves out the surprise level and vehicle passenger tests, the surprise's communication event, the wild fire and the bursts held back, the new target taken from what surprised it, the facing it keeps when the way is unknown: the desired facing rather than the current one (the campaign always knows the way) |
| `actor_action_try_to_dive` | ai/actions.c | Partial | sim/src/ai_impulse.rs:5 | sim/src/ai_impulse.rs (measure); leaves out the evasion direction tried against the world's collision and its ledges (actor_move_try_evasion_direction), a dive off a ledge |
| `fast_ftol` | ai/actor_combat.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `sine` | ai/actor_combat.c | Ported |  | `tools/engine_math_extra_reference.py` |
| `cosine` | ai/actor_combat.c | Ported |  | `tools/engine_math_extra_reference.py` |
| `tangent` | ai/actor_combat.c | Ported |  | `tools/engine_math_reference.py` |
| `cross_product2d` | ai/actor_combat.c | Ported |  | `tools/engine_math_reference.py` |
| `add_vectors3d` | ai/actor_combat.c | Ported |  | `tools/engine_item_reference.py` |
| `random_boolean` | ai/actor_combat.c | Ported |  | `tools/engine_random_reference.py` |
| `valid_real` | ai/actor_combat.c | Ported |  | `tools/engine_math_reference.py` |
| `firing_position_store_evaluation_debug` | ai/actor_firing_position.c | Replaced | dev/src/draw.rs | the editor and tools |
| `actor_select_firing_position` | ai/actor_firing_position.c | Partial | campaign/src/actors.rs:541 | campaign/tests/actors.rs (measure); leaves out the pre- and post-evaluators, discarded positions, line of sight from each position, the firing position groups |
| `arccosine` | ai/actor_moving.c | Ported |  | `tools/engine_math_reference.py` |
| `midpoint3d` | ai/actor_moving.c | Ported |  | `tools/engine_math_reference.py` |
| `arctangent` | ai/actor_perception.c | Ported |  | `tools/engine_math_reference.py` |
| `actor_perception_update` | ai/actor_perception.c | Partial | campaign/src/actors.rs:510 | campaign/tests/actors.rs (measure); leaves out props and their knowledge states, acknowledgement and orphans, danger zones, the target weights, searching and uncovering |
| `actor_activation_debug_color` | ai/actors.c | Replaced | dev/src/draw.rs | the editor and tools |
| `actor_new` | ai/actors.c | Partial | campaign/src/actors.rs:307 | campaign/tests/actors.rs (measure); leaves out the actor's variants and their weapons, its perception and memory |
| `actor_kill` | ai/actors.c | Partial | campaign/src/actors.rs:395 | campaign/tests/actors.rs (measure); leaves out swarms, a silent death (ai_kill_silent kills as ai_kill does), the actor left in the AI's list once its unit is dead |
| `actor_erase` | ai/actors.c | Partial | campaign/src/actors.rs:385 | campaign/tests/actors.rs (measure); leaves out swarms, actor_died's notice to its encounter |
| `actor_update` | ai/actors.c | Partial | campaign/src/actors.rs:600 | campaign/src/actors.rs (measure); leaves out actions and their state machines, emotions, swarms, dormancy |
| `actor_place` | ai/actors.c | Partial | campaign/src/actors.rs:306 | campaign/tests/actors.rs (measure); leaves out swarms, the starting location's initial state and animation, mounted weapons, major upgrades |
| `actors_update` | ai/actors.c | Partial | campaign/src/actors.rs:403 | campaign/tests/actors.rs (measure); leaves out actions and their state machines, emotions, swarms, dormancy and activation, props |
| `ai_place` | ai/ai.c | Matched | campaign/src/actors.rs:221 | campaign/tests/actors.rs (measure) |
| `ai_erase` | ai/ai.c | Matched | campaign/src/ai.rs:223 | campaign/tests/actors.rs (measure) |
| `ai_update` | ai/ai.c | Partial | campaign/src/actors.rs:403 | campaign/tests/actors.rs (measure); leaves out inactive encounters and swarms released, team status, the AI's spatial effects, the conversations' own update |
| `ai_handle_editing` | ai/ai.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `code_000309a0` | ai/ai.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `ai_conversation_status` | ai/ai_communication.c | Partial | campaign/src/conversations.rs:19 | campaign/tests/natives.rs (measure); leaves out status 1, made but not yet begun, status 5, unable to begin |
| `ai_conversation_line` | ai/ai_communication.c | Matched | campaign/src/conversations.rs:36 | campaign/tests/natives.rs (measure) |
| `ai_conversation_advance` | ai/ai_communication.c | Matched | content/src/campaign.rs:103 | campaign/src/conversations.rs (measure) |
| `ai_conversation_line_perform` | ai/ai_communication.c | Partial | campaign/src/conversations.rs:241 | campaign/src/conversations.rs (measure); leaves out the speaker's animation, the line's subtitle |
| `ai_conversation_stop` | ai/ai_communication.c | Matched | campaign/src/conversations.rs:349 | campaign/tests/natives.rs (measure) |
| `ai_conversation_begin` | ai/ai_communication.c | Partial | campaign/src/conversations.rs:117 | campaign/src/conversations.rs (measure); leaves out the participants chosen by distance and by sight |
| `ai_conversation` | ai/ai_communication.c | Matched | content/src/campaign.rs:63 | campaign/tests/natives.rs (measure) |
| `ai_conversation_update` | ai/ai_communication.c | Partial | campaign/src/conversations.rs:170 | campaign/src/conversations.rs (measure); leaves out the participants' gestures and looking at each other |
| `ai_profile_initialize` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_dispose` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_initialize_for_new_map` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_dispose_from_old_map` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_update` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_display` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_sample_encounters` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_change_render_spray` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_sample_actors` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_sample_units` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_sample_swarms` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_sample_props` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_sample_swarm_actors` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_sample_swarm_components` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_draw_string` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_render_encounters` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_render_actors` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_render_swarms` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_render_props` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_render_collisions` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_render_paths` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_render_spray` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_render` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_scripting_place` | ai/ai_script.c | Matched | campaign/src/ai.rs | campaign/tests/actors.rs (measure) |
| `ai_scripting_erase` | ai/ai_script.c | Matched | campaign/src/ai.rs | campaign/tests/actors.rs (measure) |
| `ai_scripting_conversation` | ai/ai_script.c | Matched | campaign/src/conversations.rs | campaign/tests/natives.rs (measure) |
| `ai_scripting_conversation_stop` | ai/ai_script.c | Matched | campaign/src/conversations.rs | campaign/tests/natives.rs (measure) |
| `ai_scripting_conversation_advance` | ai/ai_script.c | Matched | campaign/src/conversations.rs | campaign/src/conversations.rs (measure) |
| `ai_scripting_conversation_line` | ai/ai_script.c | Matched | campaign/src/conversations.rs | campaign/tests/natives.rs (measure) |
| `ai_scripting_conversation_status` | ai/ai_script.c | Matched | campaign/src/conversations.rs | campaign/tests/natives.rs (measure) |
| `encounter_create` | ai/encounters.c | Partial | campaign/src/actors.rs:265 | campaign/tests/actors.rs (measure); leaves out the encounter's firing positions and their states, the squads placed by delay or by count |
| `encounters_create_for_new_map` | ai/encounters.c | Partial | campaign/src/actors.rs:264 | campaign/tests/actors.rs (measure); leaves out activation cluster bit vectors, firing position owner indices, the encounter's own state (respawn, timers) |
| `render_debug_path` | ai/path_obstacle_avoidance.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_obstacle_path` | ai/path_obstacle_avoidance.c | Replaced | dev/src/draw.rs | the editor and tools |
| `set_real_point2d` | ai/path_obstacles.c | Ported |  | `tools/engine_math_reference.py` |
| `rotate_vector2d` | ai/path_obstacles.c | Ported |  | `tools/engine_math_reference.py` |
| `point_in_circle` | ai/path_obstacles.c | Ported |  | `tools/engine_math_reference.py` |
| `point_in_sphere` | ai/path_obstacles.c | Ported |  | `tools/engine_math_reference.py` |
| `render_debug_obstacles` | ai/path_obstacles.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_initialize` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_dispose` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_dispose_from_old_map` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_clear_storage` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_actor_deleted` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_get_last_path` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_highlight_cluster` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_drawstack_setup` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_drawstack` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_path_node` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_highlight_unit` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_surface` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_actor` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_path_nodes` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_path_storage` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_sound_point_set` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_lineoffire_new` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_lineoffire_success` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_lineofsight_reset` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_idle_look_clear` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_lineoffire_addpill` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_idle_look_addprop` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_communication_toggle_bits` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_communication_suppress` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_communication_ignore` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_communication_focus` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_lineofsight_findpoint` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_lineofsight_storeray` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_lineofsight` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_vocalize` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_speak_list` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_speak` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_describe_actor` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_select_encounter` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_select_actor` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_initialize_for_new_map` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_update` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_change_selected_actor` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_change_selected_encounter` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_teleport_to` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_get_path_storage` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_lineoffire` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_ballistic_lineoffire` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_lineofsight` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_aiming_validity` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_idle_look` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_spatial_effects` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_select_this_actor` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_speech` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_vehicles_enterable` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_path` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_paths_failed` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_all_actors` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_encounter` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_speech_update` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_path_storage_update` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_render_path_line` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_debug_get_this_actor` | ai/ai_debug.c | Replaced | dev/src/draw.rs | the editor and tools |

795 Missing (781 reached from `main_loop`), by file:

- ai/action_alert.c: `action_alert_setup`, `action_alert_begin`, `action_alert_control`, `action_alert_flush_position_indices`, `action_alert_flush_structure_indices`, `action_alert_update`, `action_alert_next_position`, `action_alert_perform`
- ai/action_avoid.c: `action_avoid_setup`, `action_avoid_begin`, `action_avoid_end`, `action_avoid_perform`, `action_avoid_update`, `action_avoid_control`
- ai/action_charge.c: `action_charge_begin`, `action_charge_find_target_range`, `action_charge_update`, `action_charge_control`, `action_charge_is_leaping`, `collision_test_line`, `action_charge_perform`, `action_charge_valid_melee_destination`, `action_charge_setup`
- ai/action_converse.c: `action_converse_setup`, `action_converse_begin`, `action_converse_perform`, `action_converse_update`, `action_converse_control`, `action_converse_replace_prop`, `actor_conversation_control`, `actor_conversation_end`, `action_converse_end`
- ai/action_fight.c: `action_fight_setup`, `action_fight_begin`, `action_fight_end`, `action_fight_update`, `action_fight_control`, `action_fight_perform`
- ai/action_flee.c: `action_flee_end`, `action_flee_flush_position_indices`, `action_flee_modify_color`, `action_flee_find_flee_position`, `action_flee_current_position_exposed`, `action_flee_replace_prop`, `action_flee_blind_panic`, `action_flee_setup`, `action_flee_begin`, `action_flee_update`, `action_flee_control`, `action_flee_at_flee_position`, `action_flee_perform`
- ai/action_guard.c: `action_guard_setup_current_position`, `action_guard_setup_find_position`, `action_guard_setup_postcombat`, `action_guard_begin`, `action_guard_end`, `action_guard_speak_post_combat`, `action_guard_update`, `action_guard_flush_position_indices`, `action_guard_flush_structure_indices`, `action_guard_modify_color`, `action_guard_replace_prop`, `action_guard_setup_from_combat_transition`, `action_guard_setup_from_fleeing`, `action_guard_perform`, `action_guard_control`
- ai/action_obey.c: `vehicle_possibility_qsort`, `action_obey_command_end`, `action_obey_individual_setup`, `action_obey_individual_flush_command_indices`, `action_obey_individual_begin`, `action_obey_individual_advance`, `action_obey_individual_end`, `action_obey_individuals_iterate`, `action_obey_command_list_setup`, `action_obey_flush_command_indices`, `action_obey_flush_structure_indices`, `action_obey_advance_command_list`, `action_obey_begin`, `action_obey_end`, `action_obey_describe_command`, `action_obey_directmovement_update_facing`, `action_obey_command_begin`, `action_obey_command_perform`, `action_obey_individual_perform`, `action_obey_individual_update`, `action_obey_update`, `action_obey_perform`, `action_obey_control`
- ai/action_search.c: `action_search_setup_target`, `action_search_setup_pursuit`, `action_search_setup_undirected`, `action_search_update`, `action_search_flush_position_indices`, `action_search_flush_structure_indices`, `action_search_control`, `action_search_begin`, `action_search_perform`
- ai/action_sleep.c: `action_sleep_control`
- ai/action_uncover.c: `action_uncover_setup_target`, `action_uncover_setup_pursuit`, `action_uncover_perform`, `action_uncover_control`, `action_uncover_flush_position_indices`, `action_uncover_flush_structure_indices`, `action_uncover_modify_color`, `action_uncover_begin`, `action_uncover_update`
- ai/action_vehicle.c: `object_get_bounding_sphere`, `action_vehicle_begin`, `action_vehicle_update`, `action_vehicle_flush_structure_indices`, `action_vehicle_control`, `action_vehicle_desirable`, `action_vehicle_evaluate_seat`, `action_vehicle_find_destination`, `action_vehicle_setup_specific`, `action_vehicle_perform`, `action_vehicle_find_impromptu_seat`, `action_vehicle_setup_impromptu`
- ai/action_wait.c: `action_wait_begin`, `action_wait_perform`, `action_wait_control`, `action_wait_setup`, `action_wait_update`
- ai/actions.c: `encounter_get_squad`, `actor_action_perform`, `actor_action_update`, `actor_action_control`, `actor_action_replace_prop`, `actor_action_flush_position_indices`, `actor_action_flush_structure_indices`, `actor_action_handle_panic_from_surprise`, `actor_action_handle_panic_from_damage`, `actor_action_handle_panic_from_burning_to_death`, `actor_action_handle_panic_from_attached_projectiles`, `actor_action_handle_panic_from_attached_melee_attackers`, `actor_action_handle_berserking_from_attacking_mode`, `actor_action_handle_berserking_from_proximity`, `actor_action_handle_berserking_from_damage`, `actor_action_deny_transition`, `actor_action_allowed_to_enter_vehicle`, `actor_action_handle_vehicle_exit`, `actor_action_allow_cover_seeking`, `actor_action_determine_pursuit_options`, `actor_action_can_stop_guarding`, `actor_action_can_stop_conversing`, `actor_action_change`, `actor_action_test_grenade`, `actor_action_try_to_seek_cover`, `actor_action_try_to_panic`, `actor_action_try_to_enter_vehicle`, `actor_get_pursuit_location`, `actor_pursuit_consider_nearby_actor`, `actor_action_name`, `actor_action_class`, `actor_action_get_default_state`, `point_to_line_distance3d`, `actor_action_set_default_state`, `actor_action_handle_initial_action`, `actor_action_handle_pending_command_list`, `actor_action_handle_panic_transition`, `actor_action_handle_berserking_from_attached_projectiles`, `actor_action_handle_vehicle_entry`, `actor_action_find_escape_from_danger`, `actor_action_handle_active_cover_seeking`, `actor_action_handle_combat_selection`, `actor_action_handle_lost_contact`, `actor_action_handle_done_fleeing`, `actor_action_handle_combat_status`, `actor_action_handle_combat_failure`, `actor_action_handle_exit_pursuit`, `actor_action_try_to_throw_grenade`, `actor_action_consider_grenade`, `actor_action_try_to_evade`, `actors_searching_same_position`, `actor_pursuit_find_nearby_actors`, `actor_action_handle_berserk_transition`, `actor_action_handle_combat_transition`, `actor_action_handle_grenade_throwing`, `actor_action_handle_evasion`, `actor_action_handle_danger_avoidance`
- ai/actor_combat.c: `actor_combat_enable_special_fire_situation`, `actor_combat_fire_wildly`, `actor_combat_disable_bursts`, `actor_firing_blindly`, `actor_combat_currently_firing_burst`, `actor_get_weapon_definition`, `actor_get_weapon_vector`, `actor_combat_get_firing_variant_definition`, `actor_combat_get_firing_parameters`, `actor_get_grenade_definition`, `actor_combat_find_nearby_target`, `actor_start_pause`, `actor_start_first_burst_delay`, `actor_combat_reaim_grenade`, `actor_combat_build_grenade_trajectory`, `actor_combat_find_grenade_target`, `actor_combat_check_collateral_damage`, `actor_combat_plan_grenade_trajectory`, `valid_realcmp`, `valid_real_normal3d`, `actor_combat_allow_special_fire_situation`, `actor_aim_projectile`, `actor_start_burst`, `actor_combat_retarget_grenade`, `actor_aim_grenade`, `actor_combat_update`
- ai/actor_firing_position.c: `firing_position_reject`, `pre_evaluator_guard`, `pre_evaluator_combatmove`, `post_evaluator_global`, `post_evaluator_pursuit`, `post_evaluator_hide`, `post_evaluator_uncover`, `post_evaluator_attack`, `firing_position_pre_evaluate`, `firing_position_post_evaluate`, `firing_positions_get_post_evaluation_bound`, `firing_position_compare`, `actor_get_firing_position_group`, `actor_clear_discarded_firing_positions`, `actor_discard_firing_position`, `actor_firing_position_discarded`, `pre_evaluator_global`, `pre_evaluator_pursuit`, `pre_evaluator_panic`, `firing_position_compute_line_of_sight`, `firing_position_forced_evaluation`, `actor_nearby_firing_positions`, `actor_active_select_firing_position`, `actor_change_firing_position`, `pre_evaluator_attack`
- ai/actor_looking.c: `actor_look_secondary_stop`, `actor_look_compute_prop_interest`, `actor_look_get_looking_definition`, `actor_look_secondary`, `actor_look_valid_aim_vector`, `actor_look_valid_look_vector`, `actor_looking_test_validity`, `actor_look_idle_find_prop`, `actor_look_idle_timer`, `actor_look_find_random_vector`, `valid_real_normal2d`, `actor_look_decode_direction`, `actor_look_idle_new_major_direction`, `actor_look_idle_new_minor_direction`, `actor_look_update`, `actor_look_affect_movement`
- ai/actor_moving.c: `actor_move_keep_moving_past_destination`, `actor_move_animation_busy`, `actor_path_clear`, `actor_path_has_path`, `actor_path_at_destination`, `actor_path_get_destination_firing_position_index`, `actor_path_input_new`, `actor_test_destination`, `actor_get_stopping_distances`, `actor_move_animation_impulse`, `actor_move_force_stop`, `actor_move_try_evasion_vector`, `actor_move_try_evasion_direction`, `actor_aim_jump`, `actor_move_avoidance_setup`, `actor_move_test_avoidance_vector`, `actor_move_vector_avoidance_find_direction`, `actor_move_transform_avoidance_vector`, `actor_move_get_avoidance_direction`, `actor_move_initialize`, `actor_path_3d_available`, `actor_move_vector_avoidance`, `actor_move_calculate_movement`, `actor_move_calculate_controlled_by_aiming`, `actor_path_refresh`, `actor_destination_update`, `actor_move_to_point`, `actor_move_to_move_position`, `actor_move_to_firing_position`, `actor_move_to_prop`, `actor_move_calculate_free`, `actor_move_update`, `actor_move_halt`, `actor_move_halt_at_firing_position`
- ai/actor_perception.c: `actor_perception_acknowledge`, `actor_get_perception_knowledge`, `actor_get_vision_distances`, `actor_emotion_assess_unopposable_danger`, `actor_emotion_get_unopposable_enemy`, `actor_perception_desire_prop`, `actor_perception_find_prop_pathfinding_location`, `actor_perception_find_killer_prop_index`, `actor_perception_find_recent_damaging_prop_index`, `actor_perception_forget_recent_damage`, `actor_perception_retreat_successful`, `actor_compute_prop_unopposable`, `actor_compute_prop_target_weight`, `actor_situation_update_target_status`, `actor_situation_combat_status_update`, `actor_situation_update`, `actor_situation_try_new_target`, `actor_perception_friend_prop_is_attacking`, `actor_perception_aiming_vector_test_blockage`, `actor_emotion_flee_with_friends`, `actor_emotion_unopposable_retreat`, `actor_perception_unit_from_swarm`, `actor_perception_assess_vehicle_danger`, `actor_berserk`, `actor_visibility_at_point`, `actor_audibility_at_point`, `actor_perception_find_sense_position`, `actor_perception_refresh_test_object`, `prop_position_refresh`, `actor_perception_refresh`, `actor_perception_refresh_danger_zone`, `actor_expected_acknowledgement`, `actor_perception_unreachable`, `actor_perception_tried_to_uncover`, `actor_perception_tried_to_search`, `actor_perception_abandoned_search`, `actor_emotion_update`, `actor_perception_become_acknowledged`, `prop_status_refresh`, `actor_perception_qsort_compare_optional_props`, `actor_perception_create_orphan_from_friend`, `actor_perception_assess_suicide_danger`, `actor_perception_midpoint3d`
- ai/actor_stimulus.c: `actor_stimulus_clear`, `actor_stimulus_combat`, `actor_stimulus_surprise`, `actor_stimulus_suspicion`, `actor_stimulus_prop_sighted`, `actor_stimulus_enter_combat_found_body`, `actor_stimulus_enter_combat_perceived_enemy`, `actor_stimulus_enter_combat_friend_in_combat`, `actor_stimulus_bumped`, `actor_stimulus_environmental_noise`, `actor_stimulus_heard_shooting`, `actor_stimulus_was_surprised`, `actor_stimulus_maneuvering`, `actor_stimulus_vehicle_eviction`, `actor_stimulus_abandon_stationary_facing`, `actor_stimulus_prop_acknowledged`, `actor_stimulus_damage`, `actor_stimulus_weapon_impact`, `actor_stimulus_weapon_detonation`, `actor_stimulus_prop_just_killed`, `actor_stimulus_prop_fleeing`, `actor_stimulus_noticed_danger_zone`
- ai/actor_type_carrier.c: `carrier_decide_action`
- ai/actor_type_crew.c: `crew_decide_action`
- ai/actor_type_elite.c: `elite_decide_action`
- ai/actor_type_engineer.c: `engineer_decide_action`
- ai/actor_type_flood.c: `flood_decide_action`, `actor_type_flood_desire_shamble`
- ai/actor_type_grunt.c: `grunt_decide_action`
- ai/actor_type_hunter.c: `hunter_decide_action`
- ai/actor_type_infection.c: `infection_decide_action`, `infection_wander_pause_time`, `infection_wander_move_time`, `infection_swarm_control`, `infection_swarm_aim_jump`
- ai/actor_type_jackal.c: `jackal_decide_action`
- ai/actor_type_marine.c: `marine_decide_action`
- ai/actor_type_mounted_weapon.c: `mounted_weapon_decide_action`
- ai/actor_type_sentinel.c: `sentinel_decide_action`
- ai/actor_types.c: `actor_type_definition_get`, `actor_types_initialize`, `actor_type_get_name`, `actor_type_get_race`, `actor_type_get_when_to_search_at_target`, `actor_type_get_when_to_pursue`, `actor_type_get_when_to_search_pursuit`, `actor_type_get_pursuit_controller`, `actor_type_get_swarm`, `actor_type_initialize`, `actor_type_decide_action`, `actor_type_swarm_control`, `actor_type_swarm_aim_jump`
- ai/actors.c: `actors_initialize`, `actors_dispose`, `actors_initialize_for_new_map`, `actors_dispose_from_old_map`, `actor_set_team`, `actor_verify_unit_activation`, `actor_verify_activation`, `actor_detach_from_unit`, `actor_swarm_detach_from_unit`, `actor_swarm_cache_delete`, `actor_swarm_component_refresh`, `actor_is_noncombat`, `actor_in_combat`, `actor_is_fighting`, `actor_attacking_target`, `actor_is_leaping`, `actor_get_weapon`, `actor_has_ranged_weapon`, `actor_target_unit_index`, `actor_derive_target_information`, `actor_switch_props`, `actor_flush_position_indices`, `actor_flush_structure_indices`, `actor_has_unlimited_grenades`, `actor_handle_damage`, `actor_freeze_unit`, `actor_freeze`, `actors_freeze`, `actor_update_begin`, `actors_move_randomly`, `actor_change_encounter`, `actor_update_end`, `actor_get_timeslice`, `actor_clear_output`, `actor_input_update`, `actor_find_pathfinding_location`, `actor_destination_tolerance`, `actor_input_sample_position`, `actor_clear_orders`, `actor_decision_loop`, `actors_handle_spatial_effect`, `actor_handle_communication`, `actor_unit_control_crouch`, `actor_unit_control_jump`, `actor_unit_control_primary_trigger`, `actor_unit_control_secondary_trigger`, `actor_unit_control_throw_grenade`, `actor_unit_control_exact_facing`, `actor_unit_control_stop_animation_impulse`, `actor_customize_unit`, `actor_set_dormant`, `actor_swarm_component_setup`, `actor_delete_props`, `actor_delete`, `actor_swarm_cache_new`, `actor_get_running_blind_vector`, `actor_died`, `actor_swarm_unit_died`, `actor_braindead`, `actor_handle_unit_effect`, `actor_set_active`, `actor_swarm_attach_unit`, `actor_randomly_control_unit`, `actor_general_update`, `actors_handle_unit_effect`, `actor_unit_control`, `actor_attach_unit`, `actor_create_for_unit`, `actors_spawn_from_unit`
- ai/ai.c: `ai_initialize`, `ai_dispose`, `ai_dispose_from_old_map`, `ai_globals_ai_active`, `ai_globals_dialogue_triggers_enabled`, `ai_globals_grenades_enabled`, `ai_get_major_upgrade_chance`, `ai_adjust_damage`, `ai_release_inactive_swarms`, `compare_potentially_releasable_entities`, `ai_find_inactive_encounters`, `ai_release_inactive_encounters`, `ai_try_vehicle_eviction`, `ai_get_responsible_unit`, `ai_handle_death`, `ai_handle_killing_spree`, `ai_handle_allegiance_status_changed`, `ai_handle_allegiance_broken_notification`, `ai_update_team_status`, `ai_handle_bump`, `ai_handle_damage`, `ai_flush_spatial_effects`, `ai_create_mounted_weapons_for_unit`, `ai_handle_deleted_object`, `ai_handle_unit_effect`, `ai_handle_enter_vehicle`, `ai_handle_exit_vehicle`, `ai_generate_line_of_fire_pill`, `ai_disconnect_from_structure_bsp`, `ai_reconnect_to_structure_bsp`, `ai_get_race_from_team_index`, `ai_initialize_for_new_map`, `ai_consider_major_upgrade`, `ai_find_line_of_fire_friend_pills`, `ai_place_pending_mounted_weapons`, `ai_test_line_of_fire`, `ai_test_line_of_sight`, `ai_test_ballistic_line_of_fire`, `ai_enemies_can_see_player`, `ai_enemies_attacking_player`, `ai_handle_spatial_effect`
- ai/ai_communication.c: `ai_communication_initialize`, `ai_communication_dispose`, `ai_communication_initialize_for_new_map`, `ai_communication_dispose_from_old_map`, `ai_communication_get_type_name`, `ai_communication_get_type_by_name`, `ai_communication_packet_new`, `reply_filter_close`, `reply_filter_not_close`, `reply_filter_searching`, `reply_filter_same_platoon`, `reply_filter_fighting`, `reply_filter_fighting_close`, `reply_filter_same_target`, `reply_filter_no_certain_target`, `reply_filter_flee_leader`, `ai_communication_consider_speech`, `actor_communication_team`, `ai_communication_look_secondary_at_unit`, `ai_communication_look_secondary_at_object`, `ai_conversation_finish`, `ai_conversation_new`, `ai_conversation_line_begin`, `ai_conversation_line_end`, `actor_reset_idle_vocalization_timer`, `actor_communication_update`, `ai_communication_handle_received_looking`, `ai_communication_update_speech_timers`, `ai_communication_get_player_rating`, `ai_conversation_actor_deleted`, `ai_conversation_unit_died`, `ai_conversation_find_participant`, `ai_communication_started`, `ai_communication_notify`, `ai_communication_actor_talk_weight`, `ai_communication_find_specific_actor_to_talk`, `ai_communication_find_global_actor_to_talk`, `ai_communication_find_actor_to_reply_to_player`, `ai_communication_finished`, `ai_communication_event`
- ai/ai_scenario_definitions.c: `scenario_get_encounter_by_name`, `encounter_definition_get_squad_by_name`, `encounter_definition_get_platoon_by_name`, `choose_random_array_element`
- ai/ai_script.c: `encounter_get_platoon`, `ai_script_initialize`, `ai_script_dispose`, `ai_script_initialize_for_new_map`, `ai_script_dispose_from_old_map`, `ai_index_from_string`, `ai_index_to_string`, `ai_index_platoon_iterator_new`, `ai_index_platoon_iterator_next`, `ai_index_squad_iterator_new`, `ai_index_squad_iterator_next`, `ai_index_actor_iterator_new`, `ai_index_actor_iterator_next`, `object_list_from_ai_reference`, `ai_scripting_attach_unit`, `ai_scripting_attach_units`, `ai_scripting_detach_unit`, `ai_scripting_detach_units`, `ai_scripting_kill_internal`, `ai_scripting_kill`, `ai_scripting_kill_silent`, `ai_scripting_erase_all`, `ai_scripting_deselect`, `ai_scripting_select`, `ai_scripting_spawn_actor`, `ai_scripting_set_respawn`, `ai_scripting_set_deaf`, `ai_scripting_set_blind`, `ai_scripting_magically_see_unit`, `ai_scripting_magically_see_units`, `ai_scripting_timer_start`, `ai_scripting_timer_expire`, `ai_scripting_count_internal`, `ai_scripting_swarm_count`, `ai_scripting_nonswarm_count`, `ai_scripting_living_count`, `ai_scripting_living_fraction`, `ai_scripting_strength`, `ai_scripting_is_attacking`, `ai_scripting_attack`, `ai_scripting_defend`, `ai_scripting_maneuver`, `ai_scripting_maneuver_enable`, `ai_scripting_migrate_find_target_squad`, `ai_scripting_migrate_internal`, `ai_scripting_migrate`, `ai_scripting_migrate_by_unit_internal`, `ai_scripting_migrate_by_unit`, `ai_scripting_migrate_and_speak`, `ai_scripting_allegiance`, `ai_scripting_allegiance_remove`, `ai_scripting_allegiance_broken`, `ai_scripting_vehicle_candidate_qsort`, `ai_scripting_going_to_vehicle`, `ai_scripting_exit_vehicle`, `ai_scripting_braindead`, `ai_scripting_braindead_by_unit`, `ai_scripting_ignore`, `ai_scripting_prefer_target`, `ai_scripting_teleport_starting_location_private`, `ai_scripting_teleport_starting_location_if_unsupported`, `ai_scripting_teleport_starting_location`, `ai_scripting_try_to_fight_nothing`, `ai_scripting_try_to_fight`, `ai_scripting_try_to_fight_player`, `ai_scripting_allow_charge`, `ai_scripting_command_list`, `ai_scripting_command_list_by_unit`, `ai_scripting_command_list_advance`, `ai_scripting_command_list_advance_by_unit`, `ai_scripting_command_list_status_internal`, `ai_scripting_command_list_status`, `ai_scripting_free`, `ai_scripting_free_units`, `ai_scripting_attach_free`, `ai_scripting_force_active`, `ai_scripting_force_active_by_unit`, `ai_scripting_set_return_state`, `ai_scripting_set_current_state`, `ai_scripting_assess_status`, `ai_scripting_status`, `ai_scripting_reconnect`, `ai_scripting_playfight`, `ai_scripting_vehicle_encounter`, `ai_scripting_find_vehicle_enterable`, `ai_scripting_vehicle_enterable_distance`, `ai_scripting_vehicle_enterable_team`, `ai_scripting_vehicle_enterable_actor_type`, `ai_scripting_vehicle_enterable_actors`, `ai_scripting_vehicle_enterable_disable`, `ai_scripting_look_at_object`, `ai_scripting_stop_looking`, `ai_scripting_automatic_migration_target`, `ai_scripting_follow_target_disable`, `ai_scripting_follow_target_players`, `ai_scripting_follow_target_unit`, `ai_scripting_follow_target_ai`, `ai_scripting_follow_distance`, `ai_scripting_link_activation`, `ai_scripting_berserk`, `ai_scripting_set_team`, `ai_scripting_allow_dormant`, `ai_scripting_magically_see_encounter`, `ai_scripting_magically_see_players`, `ai_scripting_retreat`, `ai_scripting_go_to_vehicle_internal`, `ai_scripting_go_to_vehicle`, `ai_scripting_go_to_vehicle_override`, `ai_scripting_renew`
- ai/encounters.c: `encounters_initialize`, `encounters_dispose`, `encounters_dispose_from_old_map`, `encounter_compute_activation_cluster_bit_vector`, `encounter_detach_actor`, `encounter_attach_unit`, `encounterless_attach_actor`, `encounterless_detach_actor`, `encounter_get_by_name`, `encounter_iterator_new`, `encounter_iterator_next`, `encounter_actor_iterator_new`, `encounter_actor_iterator_next`, `encounter_actor_iterator_prev`, `actor_iterator_new`, `actor_iterator_next`, `encounter_clear_pursuit`, `encounter_find_pursuit`, `encounter_modify_pursuit_desires`, `encounter_determine_pursuit_availability`, `squad_reset_starting_locations`, `encounter_definition_fit`, `encounter_new`, `encounterless_activate`, `encounter_activate`, `encounter_deactivate`, `encounter_link_activation`, `squad_get_actor_type`, `encounter_post_combat_add_possibility`, `encounter_stand_down`, `encounter_post_combat_select_random_behavior`, `encounter_post_combat`, `encounter_set_blind`, `encounter_set_deaf`, `encounter_squad_timer_expire`, `encounter_update_timers`, `encounter_test_rule`, `encounters_initialize_for_new_map`, `encounters_unit_died`, `encounter_verify_firing_position_owner_actor_indices`, `encounter_build_firing_position_owner_actor_indices`, `encounter_mark_examined_pursuit_position`, `encounter_pursuit_position_already_examined`, `encounter_get_actor_starting_location`, `encounter_force_activate`, `encounter_force_deactivate`, `encounter_update_respawn`, `encounter_update_platoons`, `encounter_update_follow`, `encounter_spawn_actor`, `encounter_set_respawn`, `encounter_control_actors`, `encounter_update_squads`, `encounterless_deactivate`, `encounter_attach_actor`, `encounter_update_status`, `encounters_update_dirty_status`, `encounters_test_activation`, `encounters_update`
- ai/path.c: `paths_initialize`, `paths_dispose`, `paths_initialize_for_new_map`, `paths_dispose_from_old_map`, `path_input_new`, `path_input_set_target_object`, `path_input_set_start`, `path_input_set_attractor`, `path_input_set_search_bounds`, `path_state_new`, `path_state_destination`, `path_state_reset`, `path_heap_verify`, `path_heap_bubble_up`, `path_heap_bubble_down`, `path_heap_pop_cheapest_node`, `path_heap_insert`, `surface_is_broken`, `path_get_node`, `path_node_from_hash_table`, `path_3d_available`, `path_3d_build_path`, `path_state_approach_point`, `path_state_build_path`, `path_state_begin`, `closest_available_point_on_surface`, `build_path_edges_for_surface`, `closest_point_to_attractor`, `path_attractor_weight`, `path_state_estimated_distance`, `path_state_traverse`, `path_state_find`
- ai/path_obstacle_avoidance.c: `obstacles_get_disc`, `obstacle_from_disc`, `path_get_step`, `path_get_step_index`, `heap_parent_index`, `heap_left_index`, `heap_right_index`, `heap_cost`, `error_heap`, `heap_verify`, `heap_up`, `heap_down`, `heap_insert`, `heap_remove`, `valid_real_point2d`, `path_add_step`, `path_new`, `path_test_pill2d`, `path_add_steps`, `path_iterate`, `path_find`, `path_avoid_obstacles`
- ai/path_obstacles.c: `obstacles_new`, `project_point3d`, `circle_intersect_ray`, `circle_tangents`, `obstacles_add_disc`, `obstacles_get_discs_in_sphere`, `obstacles_test_circle`, `obstacles_test_pill`, `obstacles_disc_tangents`, `obstacles_disc_neighborhood`, `obstacles_recompute`
- ai/path_smoothing.c: `surface_is_walkable`, `find_tangent_point`, `find_avoidance_point`, `choose_turning_point`, `find_turning_point`, `path_smooth`
- ai/path_structure_bsp.c: `structure_test_ray2d`, `structure_surfaces_are_equivalent`, `clip_empty_interval_by_solid_interval`, `structure_test_line2d`, `structure_surface_index_from_point`, `structure_test_pill2d`, `path_pathfinding_surface`
- ai/props.c: `props_initialize`, `props_dispose`, `props_initialize_for_new_map`, `props_dispose_from_old_map`, `prop_add`, `prop_new_blank`, `prop_remove`, `prop_iterator_new`, `prop_iterator_next`, `prop_new_unacknowledged`, `prop_setup_orphan`, `prop_orphan_transition`, `prop_orphan_from_friend`, `prop_orphan_update_information`, `prop_delete`, `prop_get_active_by_unit_index`, `prop_get_base_by_unit_index`

## `bink`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `bink_draw_frame` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_active` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_ui_rendering_inhibited` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_in_progress` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_free_texture_cache` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_get_memory_available` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_alloc_permanent` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_alloc` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_decompress_audio_frame` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_free` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `is_all_bink_memory_free` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_decompress_video_frame` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_query_analog_controller_buttons` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5c60` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5c80` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_initialize` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5cc0` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5d20` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_stop` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_dispose` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_update__internal` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_render` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_start` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_update` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |

## `bitmaps`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `ColorToFcolor` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `FcolorToColor` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `ColorToRGB` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `RGBToColor` | bitmaps/s3tc/s3tc.c | Replaced |  | through its caller `DecodeBlockRGB__single_pixel` |
| `Square3x3` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `Quantize` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `ClipExtrema` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `AllSame` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `EncodeBlockRGBColorKey` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `DecodeBlockRGB` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `DecodeBlockRGB__single_pixel` | bitmaps/s3tc/s3tc.c | Replaced |  | through its caller `DecodeBlockAlpha3__single_pixel` |
| `DecodeBlockAlpha4` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `DecodeBlockAlpha4__single_pixel` | bitmaps/s3tc/s3tc.c | Replaced |  | through its caller `bitmap_2d_get_pixel` |
| `DecodeBlockAlpha3` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `DecodeBlockAlpha3__single_pixel` | bitmaps/s3tc/s3tc.c | Replaced |  | through its caller `bitmap_2d_get_pixel` |
| `EncodeBlockRGB` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `EncodeBlockAlpha4` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `EncodeBlockAlpha3` | bitmaps/s3tc/s3tc.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_initialize_line` | bitmaps/bitmap_drawing.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer |
| `bitmap_step_line` | bitmaps/bitmap_drawing.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer |
| `bitmap_fill_rectangle` | bitmaps/bitmap_drawing.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer |
| `bitmap_copy` | bitmaps/bitmap_drawing.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer |
| `bitmap_tile_and_bevel_rectangle` | bitmaps/bitmap_drawing.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer |
| `bitmap_draw_line` | bitmaps/bitmap_drawing.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer |
| `bitmap_frame_rectangle` | bitmaps/bitmap_drawing.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer |
| `extract_initialize` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_find_sequence_bounds` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_warn_about_horizontal_border` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_plateless_cube_map` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_find_bitmap_bounds` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_get_bitmap_format` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_pixels_to_mipmap` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_pixels_from_mipmap` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_build_texture_pages_by_sequence` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_build_debug_plate` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_mipmaps_to_bitmap` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_add_bitmap` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_3d_textures` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_cube_maps` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_sprites` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_bitmap` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_without_sequences` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_sequence` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `extract_sequences` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmaps_extract` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmaps_extract_from_plate` | bitmaps/bitmap_extract.c | Replaced | import/src/lib.rs | the editor and tools |
| `postprocess_bitmap` | bitmaps/bitmap_group.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `delete_bitmap` | bitmaps/bitmap_group.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `postprocess_bitmap_group` | bitmaps/bitmap_group.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `bitmap_group_try_and_get_bitmap` | bitmaps/bitmap_group.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `bitmap_group_get_bitmap_from_sequence` | bitmaps/bitmap_group.c | Matched | content/src/bitmap.rs:30 | ffi/src/hud/sprite_reference.rs (trace); one lookup (Bitmap::sequence_image) for the HUD's static elements and meters and the UI's widget backgrounds and list headers; the UI clamped a frame past the last, where the game wraps it |
| `bitmap_group_add_bitmap` | bitmaps/bitmap_group.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `bitmap_fill` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_alpha_to_rgb` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_clone` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_2d_shrink` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_3d_shrink` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_cm_shrink` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_fade` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_2d_smooth` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_3d_smooth` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_cm_smooth` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_2d_sharpen` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_3d_sharpen` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_cm_sharpen` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_2d_alpha_bleed` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_3d_alpha_bleed` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_cm_alpha_bleed` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_cm_height_map` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_2d_compress_to_mipmap` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_3d_compress_to_mipmap` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_cm_compress_to_mipmap` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_2d_uncompress_from_mipmap` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_3d_uncompress_from_mipmap` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_cm_uncompress_from_mipmap` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `real_rgb_color_brightness` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `rgb_color_to_hsv_color` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `hsv_color_to_rgb_color` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `real_rgb_color_to_real_hsv_color` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `real_hsv_color_to_real_rgb_color` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `argb_color_to_real_argb_color` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `rgb_color_to_real_rgb_color` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `pixel32_to_real_argb_color` | bitmaps/bitmap_utilities.c | Replaced |  | through its caller `cinematic_render` |
| `pixel32_to_real_rgb_color` | bitmaps/bitmap_utilities.c | Replaced |  | through its caller `custom_render_nav_point` |
| `valid_real_rgb_color` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_shrink` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_smooth` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_sharpen` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_alpha_bleed` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_2d_height_map` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_3d_height_map` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_2d_vector_map` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_3d_vector_map` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_cm_vector_map` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_compress_to_mipmap` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_uncompress_from_mipmap` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `rgb_colors_interpolate` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `rgb_colors_interpolate_and_scale` | bitmaps/bitmap_utilities.c | Replaced |  | through its caller `lights_preprocess_scene` |
| `bitmap_height_map` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_vector_map` | bitmaps/bitmap_utilities.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_type_get_string` | bitmaps/bitmaps.c | Replaced |  | through its caller `rasterizer_set_texture` |
| `bitmap_format_get_string` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_format_get_bits_per_pixel` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_changed` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_delete` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_2d_address` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_3d_address` | bitmaps/bitmaps.c | Replaced |  | through its caller `bitmap_mipmap_address` |
| `bitmap_cube_map_address` | bitmaps/bitmaps.c | Replaced |  | through its caller `bitmap_mipmap_address` |
| `bitmap_mipmap_address` | bitmaps/bitmaps.c | Replaced |  | through its caller `bitmap_2d_get_pixel` |
| `bitmap_format_to_a8r8g8b8` | bitmaps/bitmaps.c | Replaced |  | through its caller `bitmap_2d_get_pixel` |
| `bitmap_byte_swap_pixels` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `palette_find_closest_match` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_format_type_valid_width` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_format_type_valid_height` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_format_type_valid_depth` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_verify` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_rebuild` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_get_max_mipmap_count` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_mipmap_get_width` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_mipmap_get_height` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_mipmap_get_depth` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_mipmap_get_pixel_count` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_mipmap_get_pixel_data_size` | bitmaps/bitmaps.c | Replaced |  | through its caller `rasterizer_bitmap_2d_changed` |
| `bitmap_mipmap_get_row_pitch` | bitmaps/bitmaps.c | Replaced |  | through its caller `rasterizer_xbox_bitmap_get_pixel_data_size` |
| `bitmap_2d_get_pixel` | bitmaps/bitmaps.c | Replaced |  | through its caller `sample_diffuse_texture` |
| `bitmap_get_pixel_count` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_get_pixel_data_size` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_2d_new` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_3d_new` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_cube_map_new` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_3d_slice_extract` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_3d_slice_insert` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_cube_map_face_extract` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_cube_map_face_insert` | bitmaps/bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `bitmap_quantitize_read_row` | bitmaps/bitmaps_quantitize.c | Replaced | import/src/lib.rs | the editor and tools |
| `row_dither` | bitmaps/bitmaps_quantitize.c | Replaced | import/src/lib.rs | the editor and tools |
| `bitmap_quantitize` | bitmaps/bitmaps_quantitize.c | Replaced | import/src/lib.rs | the editor and tools |
| `targa_export` | bitmaps/targa_file.c | Replaced | import/src/lib.rs | the editor and tools |
| `tiff_get_bounds` | bitmaps/tiff_file.c | Replaced | import/src/lib.rs | the editor and tools |
| `tiff_export` | bitmaps/tiff_file.c | Replaced | import/src/lib.rs | the editor and tools |
| `tiff_import` | bitmaps/tiff_file.c | Replaced | import/src/lib.rs | the editor and tools |
| `DECLARE1` | bitmaps/libtiff/mkg3states.c | Replaced | import/src/lib.rs | the editor and tools |
| `DECLARE3` | bitmaps/libtiff/mkg3states.c | Replaced | import/src/lib.rs | the editor and tools |
| `DECLARE2` | bitmaps/libtiff/mkg3states.c | Replaced | import/src/lib.rs | the editor and tools |
| `build_null_mode_tables` | bitmaps/libtiff/mkg3states.c | Replaced | import/src/lib.rs | the editor and tools |
| `build_horiz_mode_tables` | bitmaps/libtiff/mkg3states.c | Replaced | import/src/lib.rs | the editor and tools |
| `build_uncomp_mode_tables` | bitmaps/libtiff/mkg3states.c | Replaced | import/src/lib.rs | the editor and tools |
| `findScheme` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFNoEncode` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFNoRowEncode` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFNoStripEncode` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFNoTileEncode` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFNoDecode` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFNoRowDecode` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFNoStripDecode` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFNoTileDecode` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFSetCompressionScheme` | bitmaps/libtiff/tif_compress.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteDirectory` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteNormalTag` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFSetupShortLong` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteRational` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWritePerSampleShorts` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFSetupShortPair` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteShortTable` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteString` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteShortArray` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteLongArray` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteRationalArray` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteFloatArray` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteJPEGQTables` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteJPEGCTables` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteTransferFunction` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWriteData` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFLinkDirectory` | bitmaps/libtiff/tif_dirwrite.c | Replaced | import/src/lib.rs | the editor and tools |
| `defaultHandler` | bitmaps/libtiff/tif_error.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFSetErrorHandler` | bitmaps/libtiff/tif_error.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFError` | bitmaps/libtiff/tif_error.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFInitOrder` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `getMode` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFOpen` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFFdOpen` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFScanlineSize` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFFileName` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFFileno` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFGetMode` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFIsTiled` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFCurrentRow` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFCurrentDirectory` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFCurrentStrip` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFCurrentTile` | bitmaps/libtiff/tif_open.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFReadScanline` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFSeek` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFReadEncodedStrip` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFReadRawStrip` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFReadRawStrip1` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFFillStrip` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFReadTile` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFReadEncodedTile` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFReadRawTile` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFReadRawTile1` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFFillTile` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFReadBufferSetup` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFStartStrip` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFStartTile` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFCheckRead` | bitmaps/libtiff/tif_read.c | Replaced | import/src/lib.rs | the editor and tools |
| `ThunderDecode` | bitmaps/libtiff/tif_thunder.c | Replaced | import/src/lib.rs | the editor and tools |
| `ThunderDecodeRow` | bitmaps/libtiff/tif_thunder.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFInitThunderScan` | bitmaps/libtiff/tif_thunder.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFSetWarningHandler` | bitmaps/libtiff/tif_warning.c | Replaced | import/src/lib.rs | the editor and tools |
| `TIFFWarning` | bitmaps/libtiff/tif_warning.c | Replaced | import/src/lib.rs | the editor and tools |

## `bungie_net`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `add64` | bungie_net/common/64bit_math.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `negate64` | bungie_net/common/64bit_math.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `subtract64` | bungie_net/common/64bit_math.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `multiply64` | bungie_net/common/64bit_math.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `divide64` | bungie_net/common/64bit_math.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `key_agreement_get_packet_type` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `key_agreement_decode_packet` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `key_agreement_encode_packet` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `key_agreement_build_message` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `build_initiate_key_agreement_message` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `build_finalize_key_agreement_message` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `is_message_encryption_key_message` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `initiate_key_exchange` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `complete_key_exchange` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `initialize_key_agreement_packets` | bungie_net/common/key_agreement.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `reversible_crypt` | bungie_net/common/message_encryption.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `tea_encipher` | bungie_net/common/message_encryption.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `tea_decipher` | bungie_net/common/message_encryption.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `message_encrypt` | bungie_net/common/message_encryption.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `message_decrypt` | bungie_net/common/message_encryption.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `build_message_header` | bungie_net/common/message_header.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `byte_swap_message_header` | bungie_net/common/message_header.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `create_message` | bungie_net/common/message_header.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `compare_ulongs_descending` | bungie_net/common/prime_numbers.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `primegen` | bungie_net/common/prime_numbers.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `randomprime` | bungie_net/common/prime_numbers.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `probable_prime64` | bungie_net/common/prime_numbers.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `x_exp_y_mod_n` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `generate_diffie_hellman_public_key` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `generate_diffie_hellman_private_key` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `generate_key_parameters` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `generate_public_key` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `generate_private_key` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `randomrange` | bungie_net/common/random_numbers.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `randomrange64` | bungie_net/common/random_numbers.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `get_thread_from_pool` | bungie_net/common/thread_win32.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `get_mutex_from_pool` | bungie_net/common/thread_win32.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `create_thread` | bungie_net/common/thread_win32.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `thread_has_exited` | bungie_net/common/thread_win32.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `dispose_thread` | bungie_net/common/thread_win32.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `create_mutex` | bungie_net/common/thread_win32.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `take_mutex` | bungie_net/common/thread_win32.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `release_mutex` | bungie_net/common/thread_win32.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `dispose_mutex` | bungie_net/common/thread_win32.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `create_transport_address` | bungie_net/network/transport_address.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `delete_transport_address` | bungie_net/network/transport_address.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_address_equivalent` | bungie_net/network/transport_address.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_address_to_string` | bungie_net/network/transport_address.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_error_to_string` | bungie_net/network/transport_address.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `net_startup_debug` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `get_next_available_set_array_index` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_push_key` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_pop_key` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_get_nonce` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_nonce_is_equal` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_is_nonce` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_client_stop` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_get_xnaddr` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_get_key_id` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_get_key` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_initialize` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_dispose` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_network_available` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `create_endpoint_set` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `delete_endpoint_set` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `poll_ep_array_compare_proc` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `poll_endpoint_set` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `add_endpoint_to_set` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `remove_endpoint_from_set` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `rewind_endpoint_set` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `get_next_endpoint_from_set` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `count_endpoints_in_set` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_server_initialize` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_server_terminate` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `transport_client_start` | bungie_net/network/transport_endpoint_set_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `add_connect_thread` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `mark_connection_thread_as_terminated` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `connection_thread_list_maintenance` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `create_transport_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `get_endpoint_type` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `read_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `write_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `endpoint_readable` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `endpoint_writeable` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `endpoint_connected` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `endpoint_listening` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `endpoint_blocking` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `get_endpoint_error` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `endpoint_equivalent` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `winsock_error_to_string` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `create_socket` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `get_endpoint_address` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `set_endpoint_blocking` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `bind_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `connect_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `disconnect_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `connect_async_thread_proc` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `connect_endpoint_async` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `cancel_connect_process` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `listen_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `accept_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `read_from_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `write_to_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `delete_transport_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `reject_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Replaced | net/src/transport.rs | networking and Xbox Live |

## `cache`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `cache_files_map_directory` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_tags_unload` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_files_open` | cache/cache_files.c | Replaced |  | through its caller `shell_initialize` |
| `tag_files_close` | cache/cache_files.c | Replaced |  | through its caller `shell_dispose` |
| `tag_groups_checksum` | cache/cache_files.c | Replaced |  | through its caller `network_game_generate_join_game_token` |
| `cache_files_get_checksum` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_loaded` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_enable_writes` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_disable_writes` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_block_resize` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_data_resize` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_block_add_element` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_block_delete_element` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_load` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_unload` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_file_get_path` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_reference_set` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_iterator_new` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_iterator_next` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_get_tag_instance` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_file_header_verify` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_give_time_to_precache` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_tags_load` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_structure_bsp_load` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_structure_bsp_unload` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_get` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_get_name` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_get_group_tag` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_copy_stop_requested` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_run_decompression` |
| `cache_copy_buffer_size` | cache/cache_files_decompress_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_copy_set_priority` | cache/cache_files_decompress_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_copy_compressed_file_complete` | cache/cache_files_decompress_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_copy_begin` | cache/cache_files_decompress_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_copy_queue_end` | cache/cache_files_decompress_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_copy_initialize_zlib` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_dispose_zlib` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_compressed_alloc` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize` |
| `cache_copy_compressed_free` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize` |
| `cache_copy_initialize_read_buffers` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_initialize_file_data` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_block_on_raw_read` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize_read_data` |
| `cache_copy_block_on_raw_write` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize_read_data` |
| `acquire_read_request` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_run_decompression` |
| `cache_copy_read_buffer_size` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize_read_buffers` |
| `cache_copy_get_read_buffer` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read_internal` |
| `cache_copy_get_write_buffer` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_write_internal` |
| `cache_copy_write_buffer_size` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_write_internal` |
| `any_bit_vector_flag_set` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_wait_for_async_io` |
| `cache_copy_wait_for_async_io` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_set_flag` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_FileIOCompletionRoutine` |
| `cache_copy_get_flags` | cache/cache_files_decompress_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `decompressor_reset_timing` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `decompressor_timer_start` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read` |
| `decompressor_timer_stop` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read` |
| `cache_copy_print_timing` | cache/cache_files_decompress_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_copy_yield` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_run_decompression` |
| `cache_copy_get_status` | cache/cache_files_decompress_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_copy_end` | cache/cache_files_decompress_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_copy_FileIOCompletionRoutine` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read` |
| `cache_copy_issue_read` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read_internal` |
| `cache_copy_issue_write` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_write_internal` |
| `cache_copy_issue_read_raw` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize_read_data` |
| `cache_copy_issue_write_raw` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize_read_data` |
| `cache_copy_issue_read_internal` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read_by_index` |
| `cache_copy_issue_read_request` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_release_read_request` |
| `cache_copy_issue_read_by_index` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_initial_reads` |
| `cache_copy_release_read_request` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_run_decompression` |
| `cache_copy_issue_write_internal` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_update_write_buffers` |
| `cache_copy_initialize_read_data` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_issue_initial_reads` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_update_write_buffers` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_run_decompression` |
| `cache_copy_run_decompression` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `simple_cache_copy_thread` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize` |
| `cache_copy_initialize` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_files_initialize` |
| `cache_files_dispose` | cache/cache_files_windows.c | Replaced |  | through its caller `tag_files_close` |
| `cached_map_issue_async_request` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_block_on_async_request` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_request_get` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_request_next_free_index` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_requests_flush` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_precache_set_priority` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_precache_in_progress` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_precache_is_copying_map` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_precache_map_queue_end` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_file_get` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_file_invalidate` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_file_get_handle` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_file_get_size` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_file_get_path` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_file_windows_thread_wake` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_file_read_io_completion_routine` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_file_windows_thread_proc` |
| `cache_file_blocking_io_completion_routine` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_file_get_map_path` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `IDirect3DVertexBuffer8_IsBusy` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `IDirect3DVertexBuffer8_BlockUntilNotBusy` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `IDirect3DVertexBuffer8_Register` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `IDirect3DIndexBuffer8_IsBusy` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `IDirect3DIndexBuffer8_BlockUntilNotBusy` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_file_close` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_file_read` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_file_promote_read` | cache/cache_files_windows.c | Replaced |  | through its caller `_texture_cache_bitmap_get_hardware_format` |
| `cache_file_read_header_from_dvd` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_issue_async_read` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_issue_async_write` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_open_cache_files` |
| `cache_file_block_until_not_busy` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tags_header_register_vertex_and_index_buffers` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tags_header_deregister_vertex_and_index_buffers` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `structure_bsp_header_register_vertex_buffers` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `structure_bsp_header_deregister_vertex_buffers` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_files_delete` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_open_cache_files` |
| `cache_files_verify_language` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_initialize` |
| `cache_files_precache_map_status` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_file_set_modification_date` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_file_read_header` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_files_find_map` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cached_map_files_find_free_map` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_file_windows_thread_proc` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_file_windows_thread_create` |
| `cache_file_open` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_open_cache_files` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_initialize` |
| `cache_files_precache_map_loaded` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_precache_map_begin` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_precache_map_end` | cache/cache_files_windows.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_file_windows_thread_create` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_initialize` |
| `cache_files_initialize` | cache/cache_files_windows.c | Replaced |  | through its caller `tag_files_open` |
| `physical_memory_allocate` | cache/physical_memory_map.c | Replaced |  | through its caller `main` |
| `physical_memory_verify` | cache/physical_memory_map.c | Replaced |  | through its caller `shell_platform_verify` |
| `physical_memory_free` | cache/physical_memory_map.c | Replaced | import/src/lib.rs | Xbox-only hardware |
| `physical_memory_get_game_state_base_address` | cache/physical_memory_map.c | Replaced |  | through its caller `game_state_allocate_buffer` |
| `physical_memory_get_tag_cache_base_address` | cache/physical_memory_map.c | Replaced | import/src/lib.rs | Xbox-only hardware |
| `physical_memory_get_texture_cache_base_address` | cache/physical_memory_map.c | Replaced |  | through its caller `texture_cache_new` |
| `physical_memory_get_sound_cache_base_address` | cache/physical_memory_map.c | Replaced |  | through its caller `dsound_channel_queue_packet` |
| `predicted_resources_sound_precache` | cache/predicted_resources.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `predicted_resources_precache` | cache/predicted_resources.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `sound_cache_delete` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_open` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_idle` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_sound_new` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_sound_delete` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_sound_finished` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_sound_hardware_lock` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_sound_hardware_unlock` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_locked_block_proc` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_delete_block_proc` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `cache_block_get_sound_permutation_name` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_start_loading_sound` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_new` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_flush` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_close` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `_sound_cache_sound_request` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `render_inverse_transform_screen_point` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `sound_cache_debug_render` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `texture_cache_delete` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_open` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_idle` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_bitmap_new` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_bitmap_delete` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_steal_memory` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_return_memory` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_name_block_proc` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `bitmap_format_to_d3d_format` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `bitmap_format_to_d3d_linear_format` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `compare` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `IDirect3DDevice8_IsBusy` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `IDirect3DDevice8_KickPushBuffer` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `IDirect3DBaseTexture8_IsBusy` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `IDirect3DBaseTexture8_Register` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_flush` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_locked_block_proc` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_delete_block_proc` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_initialize_hardware_format` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `render_inverse_transform_screen_point` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_new` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_close` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_start_loading_bitmap` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `texture_cache_debug_render` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `_texture_cache_bitmap_get_hardware_format` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |
| `cache_empty_tag_instance` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_set_tag_instances` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_build_region` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_multiplayer_region` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_map_plays_multiplayer` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_show_multiplayer_unavailable` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `cache_files_map_version` | cache/cache_files.c | Replaced |  | through its caller `network_game_server_change_map_name` |
| `cache_files_map_present` | cache/cache_files.c | Replaced |  | through its caller `network_game_client_game_settings_updated` |
| `tag_index_is_group` | cache/cache_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `sound_cache_sound_get_cache_sound` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `texture_cache_bitmap_valid` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |

## `camera`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `real_local_random_range` | camera/bored_camera.c | Ported |  | `tools/engine_particle_reference.py` |
| `scripted_camera_enable` | camera/camera_scripting.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `scripted_camera_set_animation` | camera/camera_scripting.c | Matched | campaign/src/cinematic.rs | campaign/tests/cutscenes.rs (measure) |
| `scripted_camera_set_first_person` | camera/camera_scripting.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `scripted_camera_set_dead` | camera/camera_scripting.c | Partial | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure); leaves out the dead camera's own path: the scripts hand the camera back to the player |
| `scripted_camera_set` | camera/camera_scripting.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `scripted_camera_set_camera_point_relative` | camera/camera_scripting.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `scripted_camera_time` | camera/camera_scripting.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `scripted_camera_update` | camera/camera_scripting.c | Matched | campaign/src/cinematic.rs:122 | campaign/tests/natives.rs (measure) |
| `dead_camera_new` | camera/dead_camera.c | Matched | sim/src/dead_camera.rs:46 | sim/tests/death_camera.rs (measure) |
| `dead_camera_update` | camera/dead_camera.c | Matched | sim/src/lib.rs:1447 | sim/tests/death_camera.rs (measure) |
| `director_desired_perspective` | camera/director.c | Matched | sim/src/seats.rs:1238 | sim/tests/seats.rs (measure) |
| `director_script_camera` | camera/director.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `editor_camera_new` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_get_focus` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_set_focus` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_set_position` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_bump_speed` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_get_speed` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_use_roll` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_get_unit_focus` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_set_mode` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_get_mode` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_get_scripted` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_get_field_of_view` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_move_to_point` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_set_position_and_roll` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_set_unit_focus` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_update` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `editor_camera_set_scripted` | camera/editor_flying_camera.c | Replaced | app/Sources/OpenCEEngineKit/FlyControls.swift | the editor and tools |
| `first_person_camera_new` | camera/first_person_camera.c | Partial | sim/src/player.rs | sim/tests/player.rs (measure); leaves out the camera's own smoothing: the engine puts the camera at the eyes each tick |
| `first_person_camera_update` | camera/first_person_camera.c | Partial | sim/src/player.rs | sim/tests/player.rs (measure); leaves out the camera's own smoothing: the engine puts the camera at the eyes each tick |
| `following_camera_new` | camera/following_camera.c | Matched | sim/src/player.rs | sim/tests/seats.rs (measure) |
| `arcsine` | camera/following_camera.c | Ported |  | `tools/engine_math_reference.py` |
| `uniform_cubic_spline` | camera/following_camera.c | Matched | sim/src/player.rs:1696 | sim/src/player.rs (measure) |
| `uniform_cubic_spline_vector3d` | camera/following_camera.c | Ported |  | `tools/engine_wind_reference.py` |
| `camera_track_splut` | camera/following_camera.c | Matched | sim/src/player.rs:1662 | sim/src/player.rs (measure) |
| `following_camera_deterministic` | camera/following_camera.c | Matched | sim/src/player.rs | sim/tests/seats.rs (measure) |
| `following_camera_update` | camera/following_camera.c | Matched | sim/src/player.rs:788 | sim/tests/seats.rs (measure) |
| `observer_up_from_forward` | camera/observer.c | Matched | sim/src/dead_camera.rs:95 | sim/src/dead_camera.rs (measure) |
| `observer_update_polynomial` | camera/observer.c | Matched | sim/src/dead_camera.rs:9 | sim/src/dead_camera.rs (measure) |
| `observer_update` | camera/observer.c | Partial | sim/src/player.rs:759 | sim/tests/player.rs (measure); leaves out the observer's dynamics and its springs, the cutscenes' cameras |

79 Missing (66 reached from `main_loop`), by file:

- camera/bored_camera.c: `bored_camera_new`, `bored_camera_shot_threshold_milliseconds`, `bored_camera_shot_duration_milliseconds`, `is_bored`, `is_still_bored`, `valid_real_vector3d`, `valid_real_vector3d_axes2`, `bored_camera_update`
- camera/camera_scripting.c: `scripted_camera_object_is_first_person_camera`, `scripted_camera_set_absolute`, `scripted_camera_set_camera_point_absolute`, `scripted_camera_next_camera_point`, `scripted_camera_object_relative_to`
- camera/dead_camera.c: `player_has_allies`, `player_get_next_player_with_a_unit`
- camera/director.c: `director_get`, `director_initialize`, `director_dispose`, `director_inhibit_facing`, `director_inhibit_input`, `director_inhibited_facing`, `director_inhibited_input`, `director_set_mode`, `director_save_camera`, `director_get_perspective`, `director_set_camera`, `director_initialize_variables`, `director_process_variables`, `director_dispose_from_old_map`, `director_load_camera`, `director_rotate_cameras`, `director_camera_deterministic`, `director_choose_game_perspective`, `director_choose_camera_game`, `director_choose_camera_editor`, `director_choose_camera_script_camera_record`, `director_update_controls`, `director_choose_camera`, `director_initialize_for_new_map`, `director_update`, `director_initialize_for_saved_game`
- camera/editor_flying_camera.c: `editor_camera_flying_update`, `editor_camera_orbiting_update`, `translate_orbiting_to_flying`, `translate_flying_to_orbiting`
- camera/first_person_camera.c: `first_person_camera_deterministic`, `first_person_camera_for_unit_and_vector`, `first_person_camera_fake`
- camera/flying_camera.c: `flying_camera_new`, `flying_camera_new_from_point_and_vector`, `flying_camera_update`
- camera/following_camera.c: `unit_camera_get`
- camera/observer.c: `observer_get`, `observer_initialize`, `observer_clear`, `observer_initialize_for_new_map`, `observer_dispose_from_old_map`, `observer_get_camera`, `observer_command_has_finished`, `observer_reconnect_to_structure_bsp`, `observer_update_accelerations`, `observer_update_velocities`, `observer_obsolete_position`, `observer_collision_test_with_t`, `observer_collision_test_differential`, `observer_update_command`, `observer_set_camera`, `observer_apply_rotational_displacement`, `observer_update_positions`, `observer_rotational_displacement`, `observer_find_displacement`, `observer_check_penetration`, `observer_postcheck`, `observer_update_displacements`, `observer_pass_time`
- camera/orbiting_camera.c: `orbiting_camera_new`, `orbiting_camera_update`
- camera/static_camera.c: `static_camera_new`, `static_camera_update`

## `cseries`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `cseries_initialize` | cseries/cseries.c | Replaced |  | through its caller `shell_initialize` |
| `cseries_dispose` | cseries/cseries.c | Replaced |  | through its caller `shell_dispose` |
| `string_to_tag` | cseries/cseries.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_to_string` | cseries/cseries.c | Replaced |  | through its caller `tag_get` |
| `strnlen` | cseries/cseries.c | Replaced |  | through its caller `data_encode_string` |
| `strnupr` | cseries/cseries.c | Native |  | `String` |
| `strnlwr` | cseries/cseries.c | Native |  | `String` |
| `strupr` | cseries/cseries.c | Native |  | `String` |
| `strlwr` | cseries/cseries.c | Native |  | `String` |
| `csprintf` | cseries/cseries.c | Native |  | `String` |
| `display_assert` | cseries/cseries.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `csmemcmp` | cseries/cseries.c | Native |  | `copy_from_slice` |
| `csmemmove` | cseries/cseries.c | Native |  | `copy_from_slice` |
| `csmemset` | cseries/cseries.c | Native |  | `copy_from_slice` |
| `csstrcat` | cseries/cseries.c | Native |  | `String` |
| `csstrcmp` | cseries/cseries.c | Native |  | `String` |
| `csstrncat` | cseries/cseries.c | Native |  | `String` |
| `csstrncmp` | cseries/cseries.c | Native |  | `String` |
| `csstrncpy` | cseries/cseries.c | Native |  | `String` |
| `csstrtok` | cseries/cseries.c | Native |  | `String` |
| `csstrlen` | cseries/cseries.c | Native |  | `String` |
| `csstrcpy` | cseries/cseries.c | Native |  | `String` |
| `csmemcpy` | cseries/cseries.c | Native |  | `copy_from_slice` |
| `csstrcasecmp` | cseries/cseries.c | Native |  | `String` |
| `stristr` | cseries/cseries.c | Native |  | `String` |
| `string_hash` | cseries/cseries.c | Native |  | `std::collections::BTreeMap` |
| `display_debug_string` | cseries/cseries_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `system_exit` | cseries/cseries_windows.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | Xbox-only hardware |
| `system_unique_identifier_get` | cseries/cseries_windows.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | Xbox-only hardware |
| `system_unique_identifiers_equal` | cseries/cseries_windows.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | Xbox-only hardware |
| `system_milliseconds` | cseries/cseries_windows.c | Native |  | `std::time::Instant` |
| `system_seconds` | cseries/cseries_windows.c | Replaced |  | through its caller `get_number_suitable_for_initializing_random_seed` |
| `system_get_user_name` | cseries/cseries_windows.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | Xbox-only hardware |
| `system_calloc` | cseries/cseries_windows.c | Native |  | `Vec` |
| `system_malloc` | cseries/cseries_windows.c | Native |  | `Vec` |
| `system_free` | cseries/cseries_windows.c | Native |  | `Vec` |
| `system_realloc` | cseries/cseries_windows.c | Native |  | `Vec` |
| `system_get_used_memory_size` | cseries/cseries_windows.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `system_memory_information_get` | cseries/cseries_windows.c | Replaced |  | through its caller `render_debug_profile` |
| `system_show_wait_cursor` | cseries/cseries_windows.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | Xbox-only hardware |
| `system_alert` | cseries/cseries_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `system_kill_screen_saver` | cseries/cseries_windows.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | Xbox-only hardware |
| `exception_code_get_string` | cseries/cseries_windows.c | Replaced |  | through its caller `generic_exception_filter` |
| `generic_exception_filter` | cseries/cseries_windows.c | Replaced |  | through its caller `main` |
| `debug_memory_manager_initialize` | cseries/debug_memory.c | Replaced |  | through its caller `cseries_initialize` |
| `debug_check_memory_globals` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_memory_header_checksum` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `compare_file_pointer_totals` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_check_pointer_header` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `check_memory_status` | cseries/debug_memory.c | Replaced |  | through its caller `scenario_load` |
| `local_random` | cseries/debug_memory.c | Matched | sim/src/random.rs | sim/tests/random_reference.rs (trace) |
| `debug_check_pointer_overrun` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_memory_fill_with_random` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_memory_add_pointer` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_memory_remove_pointer` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_check_memory` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_dump_memory_for_file` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_dump_memory_by_file` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_malloc` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_free` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_realloc` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `debug_dump_memory` | cseries/debug_memory.c | Replaced | ffi/src/allocations.rs | the editor and tools |
| `errors_dispose` | cseries/errors.c | Replaced |  | through its caller `shell_dispose` |
| `errors_output_to_debug_file` | cseries/errors.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `errors_overflow_suppression_enable` | cseries/errors.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `error_get` | cseries/errors.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `write_to_error_file` | cseries/errors.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `write_to_debug_file` | cseries/errors.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `errors_initialize` | cseries/errors.c | Replaced |  | through its caller `shell_initialize` |
| `error` | cseries/errors.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `errors_handle` | cseries/errors.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `errors_clear` | cseries/errors.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_initialize` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_internal_step` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_rasterizer_stats` | cseries/profile.c | Replaced |  | through its caller `rasterizer_profile_frame_end` |
| `profile_timesection_inherit` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_seconds_elapsed` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_lapsed_frames` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_lapsed_msec` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `find_profile_section` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_enter_private` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_exit_private` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_describe_frame` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_timesection_begin` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_timesection_end` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `compare_profile_sections` | cseries/profile.c | Replaced |  | through its caller `profile_dump` |
| `profile_dump` | cseries/profile.c | Replaced |  | through its caller `profile_dump_to_file` |
| `profile_dump_to_file` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_dump_frame` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_dump_frame_stop` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `string_has_prefix` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_sections_activation` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_sections_activate` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_sections_deactivate` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_find_frame_value` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_find_game_value` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_frame_get_value` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_frame_iterator_new` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_frame_iterator_next` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_frame_get_messages` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_frame_get_stalls` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_rasterizer_stalls` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_timesection_begin_now` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_timesection_end_now` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_tick_start` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_tick_end` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_render_start` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_render_end` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_render_window_start` | cseries/profile.c | Replaced |  | through its caller `render_frame_pregame` |
| `profile_render_window_end` | cseries/profile.c | Replaced |  | through its caller `render_frame_pregame` |
| `profile_texture_start` | cseries/profile.c | Replaced |  | through its caller `rasterizer_set_texture_bitmap_data` |
| `profile_texture_end` | cseries/profile.c | Replaced |  | through its caller `rasterizer_set_texture_bitmap_data` |
| `profile_frame_start` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_frame_end` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_idle_start` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_idle_end` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `progress_new` | cseries/progress.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `progress_update` | cseries/progress.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `shortsort_2byte` | cseries/sort.c | Native |  | `sort_by` |
| `shortsort_4byte` | cseries/sort.c | Native |  | `sort_by` |
| `qsort_2byte` | cseries/sort.c | Replaced |  | through its caller `rasterizer_frame_statistics_count_dynamic_vertices` |
| `qsort_4byte` | cseries/sort.c | Native |  | `sort_by` |
| `stack_walk_global_function_offset` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `stack_walk_disregard_symbol_names` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `symbol_sort_proc` | cseries/stack_walk_windows.c | Replaced |  | through its caller `load_symbol_table` |
| `free_symbol_table` | cseries/stack_walk_windows.c | Replaced |  | through its caller `load_symbol_table` |
| `symbol_name_from_address` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `base_address_from_symbol_name` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `is_valid_ebp` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `walk_up` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `initialize_stack_walk` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `walk_stack_context` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `walk_stack` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `stack_walk_dispose` | cseries/stack_walk_windows.c | Replaced |  | through its caller `errors_dispose` |
| `stack_walk_with_context` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `load_symbol_table` | cseries/stack_walk_windows.c | Replaced |  | through its caller `stack_walk_initialize` |
| `stack_walk_initialize` | cseries/stack_walk_windows.c | Replaced |  | through its caller `errors_initialize` |
| `stack_walk` | cseries/stack_walk_windows.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `reset_error_state` | cseries/errors.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `profile_append` | cseries/profile.c | Replaced | dev/src/logbook.rs | the editor and tools |

## `cutscene`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `cinematic_start` | cutscene/cinematics.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `cinematic_can_be_skipped` | cutscene/cinematics.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `cinematic_skip_start` | cutscene/cinematics.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `cinematic_skip_stop` | cutscene/cinematics.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `cinematic_show_letterbox` | cutscene/cinematics.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `cinematic_suppress_bsp_object_creation` | cutscene/cinematics.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `cinematic_stop` | cutscene/cinematics.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `cinematic_in_progress` | cutscene/cinematics.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `cinematic_set_title` | cutscene/cinematics.c | Matched | campaign/src/cinematic.rs | campaign/tests/natives.rs (measure) |
| `byte_swap_recording` | cutscene/recorded_animation_definitions.c | Replaced | import/src/lib.rs | maps imported to a typed format; byte order is settled when the map is imported |
| `recorded_animation_byteswap_unit_control` | cutscene/recorded_animation_initialize.c | Replaced | import/src/lib.rs | maps imported to a typed format; byte order is settled when the map is imported |
| `recorded_animation_write_unit_control` | cutscene/recorded_animation_initialize.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `byte_swap_recording_stream` | cutscene/recorded_animation_playback.c | Replaced | import/src/lib.rs | maps imported to a typed format; byte order is settled when the map is imported |
| `byte_swap_recording_stream_v1` | cutscene/recorded_animation_playback_v1.c | Replaced | import/src/lib.rs | maps imported to a typed format; byte order is settled when the map is imported |
| `recorded_animation_verify` | cutscene/recorded_animations.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `render_debug_recording` | cutscene/recorded_animations.c | Replaced | dev/src/draw.rs | the editor and tools |
| `recorded_animation_play_internal` | cutscene/recorded_animations.c | Partial | campaign/src/units.rs:157 | campaign/tests/natives.rs (measure); leaves out the recorded controls played back: the engine waits out the recording's length, the recording's version checks, the units in vehicles |

48 Missing (41 reached from `main_loop`), by file:

- cutscene/cinematics.c: `cinematic_initialize`, `cinematic_dispose`, `cinematic_initialize_for_new_map`, `cinematic_dispose_from_old_map`, `draw_quad`, `cinematic_force_title`, `cinematic_set_title_delayed`, `cinematic_render`
- cutscene/recorded_animation_definitions.c: `scenario_get_animation_by_name`
- cutscene/recorded_animation_initialize.c: `recorded_animation_initialize_unit_control`
- cutscene/recorded_animation_playback.c: `apply_animation_state`, `apply_aiming_speed`, `apply_control_flags`, `apply_weapon_index`, `apply_throttle`, `update_controller_char`, `update_controller_short`, `uncompress_vector_from_controller`, `apply_vector_char_difference`, `apply_vector_short_difference`, `recorded_animation_initialize_event_stream`, `recorded_animation_initialize_event_stream_with_size`, `recorded_animation_apply_event_stream`
- cutscene/recorded_animation_playback_v1.c: `apply_facing_vector`, `apply_aiming_vector`, `apply_looking_vector`, `apply_angle_vector`, `apply_multi_vector`, `code_00082c90`, `code_00082d50`, `code_00082e10`, `code_00082ed0`, `code_00082fe0`, `recorded_animation_initialize_event_stream_v1`, `recorded_animation_apply_event_stream_v1`
- cutscene/recorded_animations.c: `recorded_animations_initialize`, `recorded_animations_dispose`, `recorded_animations_dispose_from_old_map`, `recorded_animations_clear_debug_storage`, `recorded_animations_update`, `recorded_animation_controlling_unit`, `get_controlling_thread`, `recorded_animations_initialize_for_new_map`, `recorded_animation_kill`, `recorded_animation_get_time_left`, `recorded_animation_play`, `recorded_animation_play_and_delete`, `recorded_animation_play_and_hover`

## `devices`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `machine_place` | devices/device_machines.c | Matched | campaign/src/devices.rs:219 | campaign/tests/devices_reference.rs (trace) |
| `machine_update` | devices/device_machines.c | Matched | campaign/src/devices.rs:414 | campaign/tests/devices_reference.rs (trace) |
| `device_new` | devices/devices.c | Ported | campaign/src/devices.rs:218 | `tools/engine_devices_reference.py` |
| `device_export_function_values` | devices/devices.c | Matched | sim/src/device_animation.rs:5 | sim/src/device_animation.rs (measure) |
| `device_preprocess_node_orientations` | devices/devices.c | Matched | sim/src/device_animation.rs:6 | sim/src/device_animation.rs (measure) |
| `device_get_position` | devices/devices.c | Matched | campaign/src/devices.rs | campaign/tests/devices_reference.rs (trace) |
| `device_get_power` | devices/devices.c | Matched | campaign/src/devices.rs | campaign/tests/devices_reference.rs (trace) |
| `device_set_never_appears_locked` | devices/devices.c | Matched | campaign/src/devices.rs | campaign/tests/devices_reference.rs (trace) |
| `device_group_set_actual_value` | devices/devices.c | Ported | campaign/src/devices.rs:321 | `tools/engine_devices_reference.py` |
| `device_one_sided_set` | devices/devices.c | Matched | campaign/src/devices.rs | campaign/tests/devices_reference.rs (trace) |
| `device_operates_automatically_set` | devices/devices.c | Matched | campaign/src/devices.rs | campaign/tests/devices_reference.rs (trace) |
| `device_group_change_only_once_more_set` | devices/devices.c | Matched | campaign/src/devices.rs:341 | campaign/tests/devices_reference.rs (trace) |
| `device_group_get_value` | devices/devices.c | Matched | campaign/src/devices.rs:288 | campaign/tests/devices_reference.rs (trace) |
| `device_group_new` | devices/devices.c | Ported | campaign/src/devices.rs:142 | `tools/engine_devices_reference.py` |
| `device_group_delete` | devices/devices.c | Ported | campaign/src/devices.rs:168 | `tools/engine_devices_reference.py` |
| `create_initial_device_groups` | devices/devices.c | Matched | campaign/src/devices.rs:131 | campaign/tests/devices_reference.rs (trace) |
| `devices_initialize_for_new_map` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_delete` | devices/devices.c | Ported | campaign/src/devices.rs:280 | `tools/engine_devices_reference.py` |
| `device_update` | devices/devices.c | Matched | campaign/src/devices.rs:414 | campaign/tests/devices_reference.rs (trace) |
| `device_render_debug` | devices/devices.c | Replaced | dev/src/draw.rs | the editor and tools |
| `device_group_set_desired_value` | devices/devices.c | Matched | campaign/src/devices.rs:296 | campaign/tests/devices_reference.rs (trace) |
| `device_set_actual_position` | devices/devices.c | Matched | campaign/src/devices.rs:361 | campaign/tests/devices_reference.rs (trace) |
| `device_add_scenario_information` | devices/devices.c | Ported | campaign/src/devices.rs:219 | `tools/engine_devices_reference.py` |
| `device_set_desired_position` | devices/devices.c | Matched | campaign/src/devices.rs:350 | campaign/tests/devices_reference.rs (trace) |
| `device_set_power` | devices/devices.c | Matched | campaign/src/devices.rs:371 | campaign/tests/devices_reference.rs (trace) |
| `device_group_change` | devices/devices.c | Matched | campaign/src/devices.rs:296 | campaign/tests/devices_reference.rs (trace) |

34 Missing (33 reached from `main_loop`), by file:

- devices/device_controls.c: `controls_initialize`, `controls_dispose`, `controls_initialize_for_new_map`, `controls_dispose_from_old_map`, `control_place`, `control_new`, `control_delete`, `control_update`, `control_toggle`, `control_touched`, `control_destroyed`
- devices/device_light_fixtures.c: `light_fixtures_initialize`, `light_fixtures_dispose`, `light_fixtures_initialize_for_new_map`, `light_fixtures_dispose_from_old_map`, `light_fixture_place`, `light_fixture_new`, `light_fixture_delete`, `light_fixture_update`
- devices/device_machines.c: `machines_initialize`, `machines_dispose`, `machines_initialize_for_new_map`, `machines_dispose_from_old_map`, `machine_new`, `machine_delete`, `machine_bumped`, `machine_try_to_open_with_damage`
- devices/devices.c: `devices_initialize`, `devices_dispose`, `devices_dispose_from_old_map`, `device_touched`, `device_can_change_position`, `device_effect_new`, `device_frontfacing`

## `dialogs`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `edit_text_fix_selection` | dialogs/edit_text.c | Replaced | app/Sources/OpenCEEngineKit/Shell/LineEditor.swift | the editor and tools |
| `edit_text_selection_reset` | dialogs/edit_text.c | Replaced | app/Sources/OpenCEEngineKit/Shell/LineEditor.swift | the editor and tools |
| `edit_text_get_selection_indices` | dialogs/edit_text.c | Replaced | app/Sources/OpenCEEngineKit/Shell/LineEditor.swift | the editor and tools |
| `edit_text_new` | dialogs/edit_text.c | Replaced | app/Sources/OpenCEEngineKit/Shell/LineEditor.swift | the editor and tools |
| `edit_text_handle_key` | dialogs/edit_text.c | Replaced | app/Sources/OpenCEEngineKit/Shell/LineEditor.swift | the editor and tools |

## `editor`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `editor_render` | editor/editor_stubs.c | Replaced | dev/src/lib.rs | the editor and tools |
| `game_in_editor` | editor/editor_stubs.c | Replaced | dev/src/lib.rs | the editor and tools |
| `editor_preprocess_rendered_object` | editor/editor_stubs.c | Replaced | dev/src/lib.rs | the editor and tools |
| `editor_should_exit` | editor/editor_stubs.c | Replaced | dev/src/lib.rs | the editor and tools |
| `editor_initialize` | editor/editor_stubs.c | Replaced | dev/src/lib.rs | the editor and tools |
| `editor_dispose` | editor/editor_stubs.c | Replaced | dev/src/lib.rs | the editor and tools |
| `editor_update` | editor/editor_stubs.c | Replaced | dev/src/lib.rs | the editor and tools |
| `editor_initialize_for_new_map` | editor/editor_stubs.c | Replaced | dev/src/lib.rs | the editor and tools |
| `editor_dispose_from_old_map` | editor/editor_stubs.c | Replaced | dev/src/lib.rs | the editor and tools |

## `effects`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `contrail_scale_value` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:32 | ffi/src/particles/contrails.rs (measure) |
| `contrail_delete` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | ffi/src/particles/contrails.rs (measure) |
| `contrail_compute_new_point_count` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | ffi/src/particles/contrails.rs (measure) |
| `local_random_range` | effects/contrails.c | Matched | sim/src/random.rs | sim/tests/random_reference.rs (trace); the particles draw on a xorshift of their own (ffi/src/particles.rs), not the game's local seed; the animation and world track owns that |
| `local_random_vector_in_cone3d` | effects/contrails.c | Matched | sim/src/random.rs | sim/tests/random_reference.rs (trace); the particles draw on a xorshift of their own (ffi/src/particles.rs), not the game's local seed; the animation and world track owns that |
| `contrail_scale_random_value` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:42 | ffi/src/particles/contrails.rs (measure) |
| `contrail_next_frame` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:252 | ffi/src/particles/contrails.rs (measure) |
| `contrail_add_points` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:298 | ffi/src/particles/contrails.rs (measure) |
| `contrail_update_points` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:122 | ffi/src/particles/contrails.rs (measure) |
| `contrail_new` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | ffi/src/particles/contrails.rs (measure) |
| `contrails_update` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | ffi/src/particles/contrails.rs (measure) |
| `decal_sprite_get_bounds` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `render_debug_decals` | effects/decals.c | Replaced | dev/src/draw.rs | the editor and tools |
| `projection_from_vector3d` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `projection_sign_from_vector3d` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `project_point2d` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `triple_product3d` | effects/decals.c | Matched | sim/src/vehicle_animation.rs:73 | sim/src/vehicle_animation.rs (measure) |
| `plane2d_from_points` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `plane3d_distance_to_point` | effects/decals.c | Ported |  | `tools/engine_math_reference.py` |
| `bsp3d_get_plane_from_designator` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `decal_update` | effects/decals.c | Matched | ffi/src/dynamic_decals.rs | ffi/src/dynamic_decals.rs (measure) |
| `decal_insert` | effects/decals.c | Matched | ffi/src/dynamic_decals.rs | ffi/src/dynamic_decals.rs (measure) |
| `decals_update` | effects/decals.c | Matched | ffi/src/dynamic_decals.rs:4 | ffi/src/dynamic_decals.rs (measure) |
| `decal_delete` | effects/decals.c | Matched | ffi/src/dynamic_decals.rs | ffi/src/dynamic_decals.rs (measure) |
| `decal_projection_create` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `decal_clip_to_surface` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `decal_new_from_collision` | effects/decals.c | Matched | ffi/src/decals.rs:179 | ffi/src/decals.rs (measure) |
| `decal_new` | effects/decals.c | Matched | ffi/src/decals.rs:144 | ffi/src/decals.rs (measure) |
| `effect_delete` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_scale` | effects/effects.c | Matched | content/src/weapon.rs:602 | sim/src/effects.rs (measure) |
| `effect_get_random_seed` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_real_random_range` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_random_angular_velocity` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_allowed_by_environment` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_part_allowed_by_disposition` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_set_event` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_build_location` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_location_get_next_instance` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `real_local_random` | effects/effects.c | Matched | sim/src/random.rs | sim/tests/random_reference.rs (trace) |
| `local_random_direction3d` | effects/effects.c | Matched | sim/src/random.rs | sim/tests/random_reference.rs (trace) |
| `effect_stop` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_random_translational_velocity` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_allocate` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_build_locations` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_generate_particles` | effects/effects.c | Partial | ffi/src/particles.rs | ffi/src/particles.rs (measure); leaves out a particle's camera mode: first person only, third person only or both (camera_mode) |
| `effect_generate_part` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_marker_list_get_marker` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_generate_parts` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_marker_list_get_markers_by_name` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_update` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_new_looping` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_new_from_object` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_new_attached_from_markers` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `effect_new_unattached_from_markers` | effects/effects.c | Matched | sim/src/effects.rs:64 | sim/src/effects.rs (measure) |
| `effects_update` | effects/effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `material_effect_new` | effects/material_effects.c | Matched | ffi/src/sound_events.rs:210 | ffi/src/sound_events.rs (measure) |
| `material_effect_new_from_point` | effects/material_effects.c | Matched | sim/src/effects.rs | sim/src/effects.rs (measure) |
| `particle_system_orphan` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_delete` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_next_type_state_index` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_next_particle_state_index` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_update_default` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_new_particle_default` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_update_particle_default` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_update_explosion` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_new_particles` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `randomize_particle_variables` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_update` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_render` | effects/particle_systems.c | Replaced | app/Sources/OpenCEEngineRender/ParticlePass.swift | the enhanced lighting and Metal renderer |
| `particle_system_new_particle_explosion` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_new_particle_jet` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_initialize` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_systems_update` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_systems_render` | effects/particle_systems.c | Replaced | app/Sources/OpenCEEngineRender/ParticlePass.swift | the enhanced lighting and Metal renderer |
| `particle_system_new_unattached` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_system_new_attached` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | ffi/src/particles/systems.rs (measure) |
| `particle_delete` | effects/particles.c | Matched | ffi/src/particles.rs | ffi/src/particles.rs (measure) |
| `new_particle_is_visible` | effects/particles.c | Matched | ffi/src/particles.rs | ffi/src/particles.rs (measure) |
| `particle_get_radius` | effects/particles.c | Matched | content/src/particle.rs:155 | ffi/tests/particle_reference.rs (trace); its radius through a particle's life, unclamped as the game's; the engine falls back to the definition's first radius where the game's would not be a number |
| `particle_effect_new` | effects/particles.c | Matched | ffi/src/particles.rs | ffi/src/particles.rs (measure) |
| `particle_die` | effects/particles.c | Matched | ffi/src/particles.rs | ffi/src/particles.rs (measure) |
| `particle_next_sequence` | effects/particles.c | Matched | ffi/src/particle_life.rs:5 | ffi/tests/particle_reference.rs (trace) |
| `particle_next_frame` | effects/particles.c | Matched | ffi/src/particle_life.rs:6 | ffi/tests/particle_reference.rs (trace); a bitmap sequence with images but no sprites counts its images (the game's sprite count would be none, and its particles would die at once) |
| `particle_update_frame_time` | effects/particles.c | Matched | ffi/src/particle_life.rs:6 | ffi/tests/particle_reference.rs (trace); steps at most 4096 frames an update, where the game's loop has no bound and would hold forever on a frame shown no time |
| `particle_update_physics` | effects/particles.c | Matched | ffi/src/particles.rs:502 | ffi/src/particles.rs (measure) |
| `particle_new` | effects/particles.c | Matched | ffi/src/particle_life.rs:2 | ffi/tests/particle_reference.rs (trace) |
| `particles_update` | effects/particles.c | Partial | ffi/src/particle_life.rs:7 | ffi/tests/particle_reference.rs (trace); leaves out the deletion of particles left undrawn for a few frames (the engine draws every live particle), the physics, which particle_update_physics owes; a particle's age, death past its life and its frames, as particle_life::Life::update; the engine ticks particles at 30 Hz |
| `weather_particle_system_get` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_get_type` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_type_delete_particle` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_wrap_point` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_new` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_delete` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_new_particle` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_box_offset_from_point3d` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_update_physics` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_build_clipping_planes` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_transform_clip_planes_to_box` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_polyhedra_find` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_update_particle_count` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_update` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | ffi/src/particles/weather.rs (measure) |
| `weather_particle_system_render` | effects/weather_particle_systems.c | Replaced | app/Sources/OpenCEEngineRender/ParticlePass.swift | the enhanced lighting and Metal renderer |
| `weather_particle_systems_render` | effects/weather_particle_systems.c | Replaced | app/Sources/OpenCEEngineRender/ParticlePass.swift | the enhanced lighting and Metal renderer |

87 Missing (87 reached from `main_loop`), by file:

- effects/contrails.c: `contrails_initialize_for_new_map`, `contrails_dispose_from_old_map`, `contrails_dispose`, `contrails_disconnect_from_structure_bsp`, `contrails_reconnect_to_structure_bsp`, `contrail_verify`, `contrails_initialize`, `contrail_owner_collision`
- effects/decals.c: `decal_check`, `decal_set_first_decal_index`, `decals_initialize`, `decals_initialize_for_new_map`, `decals_dispose_from_old_map`, `decals_dispose`, `decals_unlock`, `decal_get_first_decal_index`, `decal_new_from_media_collision`, `plane3d_from_point_and_normal`, `plane3d_negate`, `real_a_rgb_color_to_pixel32`, `decal_reinsert`, `decals_reconnect_to_structure_bsp`, `decals_disconnect_from_structure_bsp`, `decals_delete_permanent_from_cluster`
- effects/effects.c: `effects_initialize`, `effects_initialize_for_new_map`, `effects_dispose_from_old_map`, `effects_dispose`, `effects_object_is_corpse`, `effects_stop_on_first_person_weapon`, `effects_information_get`, `effects_disconnect_from_structure_bsp`, `effect_get_node_matrix`, `effect_evaluate_function_integral`, `effects_reconnect_to_structure_bsp`, `dangerous_effects_near_player`, `impulse_effect_initialize`, `effects_start_on_first_person_weapon`
- effects/material_effects.c: `material_effect_visible`
- effects/particle_systems.c: `particle_systems_initialize`, `particle_systems_initialize_for_new_map`, `particle_systems_dispose`, `particle_systems_disconnect_from_structure_bsp`, `particle_systems_reconnect_to_structure_bsp`, `particle_systems_dispose_from_old_map`
- effects/particles.c: `particles_initialize`, `particles_initialize_for_new_map`, `particles_dispose_from_old_map`, `particles_dispose`, `particles_stop_on_first_person_weapon`, `particles_disconnect_from_structure_bsp`, `particles_reconnect_to_structure_bsp`, `valid_real_point3d`, `valid_real_argb_color`
- effects/player_effects.c: `player_effect_get`, `player_effect_initialize`, `player_effect_dispose`, `player_effect_initialize_for_new_map`, `player_effect_dispose_from_old_map`, `player_effect_add_continuous_effect`, `scripted_player_effect_set_rotation`, `scripted_player_effect_set_rumble`, `player_telefrag_effect_stop`, `player_effect_screen_fade_in`, `player_effect_screen_fade_out`, `player_effect_get_damage_indicators`, `player_effect_clear_damage_indicators`, `effect_scale_factor`, `player_effect_update_screen_flash`, `player_effect_update_camera_shake`, `effect_scale_value`, `player_effect_update`, `player_effect_continuous_refresh`, `scripted_player_effect_set_translation`, `scripted_player_effect_start`, `scripted_player_effect_stop`, `player_effect_screen_flash`, `player_telefrag_effect_start`, `player_effect_get_screen_flash`, `get_shake_matrix`, `player_effect_get_camera_effect_matrix`, `player_effect_update_camera_impulse`, `player_effect_start`
- effects/weather_particle_systems.c: `weather_particle_systems_initialize`, `weather_particle_systems_initialize_for_new_map`, `weather_particle_systems_dispose_from_old_map`, `weather_particle_systems_dispose`

## `game`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `reciprocal_square_root` | game/aim_assist.c | Ported |  | `tools/engine_animation_reference.py` |
| `limit3d` | game/aim_assist.c | Ported |  | `tools/engine_math_reference.py` |
| `set_real_euler_angles2d` | game/aim_assist.c | Ported |  | `tools/engine_math_reference.py` |
| `cheats_initialize` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheats_dispose` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheats_dispose_from_old_map` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheats_update` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheats_load` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheat_active_camouflage_local_player` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheat_player_index` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheats_initialize_for_new_map` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheat_teleport_to_camera` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheat_active_camouflage` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheat_objects` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheat_all_weapons` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheat_all_powerups` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `cheat_all_vehicles` | game/cheats.c | Replaced | dev/src/control.rs | the editor and tools |
| `game_tick` | game/game.c | Partial | sim/src/game.rs:225 | sim/tests/game_tick_reference.rs (trace); leaves out the game's order: the scripts run before the actors, units and weapons, the actors think before the units move, and the effects run after the simulation, the antennas, rumble, the editor, the movement and placement traces and the network client's actors, which the engine's tick does not update |
| `game_initial_pulse` | game/game.c | Matched | sim/src/game.rs:863 | sim/tests/game_engine_reference.rs (trace); the game engine's trace compiles it to add its players; the start it calls is traced in sim/tests/announcer_reference.rs |
| `game_all_quiet` | game/game.c | Partial | sim/src/safety.rs:1 | sim/tests/safety_reference.rs (trace); leaves out the dangers themselves, but for a dead player: enemies that see a player, projectiles, items, effects and vehicles near one, units throwing or dying, a player in the air |
| `game_safe_to_save` | game/game.c | Partial | sim/src/safety.rs:2 | sim/tests/safety_reference.rs (trace); leaves out the dangers themselves, but for a dead player: enemies that see a player, projectiles, items, effects and vehicles near one, units throwing or dying, a player in the air |
| `game_safe_to_speak` | game/game.c | Partial | sim/src/safety.rs:2 | sim/tests/safety_reference.rs (trace); leaves out the dangers themselves, but for a dead player: enemies that see a player, projectiles, items, effects and vehicles near one, units throwing or dying, a player in the air |
| `game_is_cooperative` | game/game.c | Matched | sim/src/safety.rs:54 | sim/tests/safety_reference.rs (trace); the engine counts its players, the game the players it spawns |
| `set_random_seed` | game/game.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `game_engine_get_team_score` | game/game_engine.c | Matched | sim/src/game.rs:348 | sim/tests/game_engine_reference.rs (trace) |
| `initialize_player_multiplayer_data` | game/game_engine.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `game_engine_game_starting` | game/game_engine.c | Partial | sim/src/game.rs:863 | sim/tests/announcer_reference.rs (trace); leaves out importing the globals' multiplayer sounds (the data track), playing the sound (the scripts and sounds track), game_engine_build_lighting (replaced by the engine's lighting); the announcer's queue (game_engine_multiplayer_sounds.c) and the game types' starts are traced with it in sim/src/announcer.rs; Race's start is not in the sources |
| `players_in_game` | game/game_engine.c | Ported |  | `tools/engine_game_end_reference.py` |
| `game_engine_handle_client_message` | game/game_engine.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_handle_server_message` | game/game_engine.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_update_purge` | game/game_engine.c | Partial | sim/src/game.rs:623 | sim/tests/item_spawn_reference.rs (trace); leaves out the purge of dropped weapons and dead bipeds, which the units' own garbage handles, can_delete_item's flags |
| `item_collection_get_total` | game/game_engine.c | Ported | sim/src/game.rs:635 | `tools/engine_item_spawn_reference.py` |
| `nearby_vehicle` | game/game_engine.c | Ported | sim/src/game.rs:677 | `tools/engine_spawn_reference.py` |
| `game_engine_get_variant` | game/game_engine.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `game_engine_has_teams` | game/game_engine.c | Ported |  | `tools/engine_announcer_reference.py` |
| `game_engine_player_has_flag` | game/game_engine.c | Ported |  | `tools/engine_ctf_reference.py` |
| `game_engine_test_flag` | game/game_engine.c | Ported |  | `tools/engine_spawn_reference.py` |
| `build_game_variant_slayer` | game/game_engine.c | Matched | sim/src/game.rs:73 | sim/tests/game_variant_reference.rs (trace) |
| `build_game_variant_team_slayer` | game/game_engine.c | Matched | sim/src/game.rs:88 | sim/tests/game_variant_reference.rs (trace) |
| `build_game_variant_ctf` | game/game_engine.c | Matched | sim/src/game.rs:99 | sim/tests/game_variant_reference.rs (trace) |
| `game_engine_flag_reset` | game/game_engine.c | Ported | sim/src/game.rs:229 | `tools/engine_ctf_reference.py` |
| `multiple_teams_alive` | game/game_engine.c | Ported | sim/src/game.rs:593 | `tools/engine_game_end_reference.py` |
| `game_engine_should_end_game` | game/game_engine.c | Partial | sim/src/game.rs:590 | sim/tests/game_end_reference.rs (trace); leaves out lives, the odd man out, players who quit |
| `random_item` | game/game_engine.c | Matched | sim/src/game.rs:643 | sim/tests/item_spawn_reference.rs (trace) |
| `match_game_type` | game/game_engine.c | Matched | content/src/level.rs:464 | sim/tests/spawn_reference.rs (trace) |
| `game_engine_update_item_spawn` | game/game_engine.c | Partial | sim/src/game.rs:629 | sim/tests/item_spawn_reference.rs (trace); leaves out the placement's created at rest flag, the game type's no map weapons, Custom Edition's facing |
| `game_engine_get_distance_rating_for_spawn` | game/game_engine.c | Ported | sim/src/game.rs:688 | `tools/engine_spawn_reference.py` |
| `game_engine_get_friendly_bonus` | game/game_engine.c | Ported | sim/src/game.rs:725 | `tools/engine_spawn_reference.py` |
| `default_starting_location_rate_function` | game/game_engine.c | Partial | sim/src/game.rs:744 | sim/tests/spawn_reference.rs (trace); leaves out Capture the Flag's assault rating: the engine has no assault option |
| `game_engine_get_starting_location_rating` | game/game_engine.c | Partial | sim/src/game.rs:743 | sim/tests/spawn_reference.rs (trace); leaves out nearby_vehicle's objects_in_sphere: the engine tests every vehicle's bounding sphere, not the sphere's clusters' first sixteen objects |
| `game_engine_initialize_for_new_map` | game/game_engine.c | Partial | sim/src/game.rs:871 | sim/tests/announcer_reference.rs (trace); leaves out the custom game engine's own initialisation, the map's check and the resources' prediction, the goals, the vehicles' homes and the network state, which the engine's game makes afresh; the announcer's trace compiles it to empty the queue; the next team's reset is traced in sim/tests/game_engine_reference.rs |
| `game_engine_player_added` | game/game_engine.c | Partial | sim/src/game.rs:290 | sim/tests/game_engine_reference.rs (trace); leaves out a network client's own team choice, the multiplayer message as a player joins |
| `game_engine_update` | game/game_engine.c | Partial | sim/src/game.rs:951 | sim/tests/game_engine_reference.rs (trace); leaves out the oddball, king, race and juggernaut variants, the multiplayer sound, no shields, invisibility and teleporters, the vehicles' respawn, the postgame's sound and its killing of the players; slayer's update and the kills scored are traced; CTF (ctf_reference), the game's end (game_end_reference) and the items' respawn (item_spawn_reference) are each traced on their own |
| `game_engine_player_killed` | game/game_engine.c | Partial | sim/src/game.rs:377 | sim/tests/game_engine_reference.rs (trace); leaves out the oddball, king, race and juggernaut variants, the game's announcer and score messages, the PC version's gametype options (auto team balance, the friendly fire penalty), off in stock |
| `ctf_engine_game_starting` | game/game_engine_ctf.c | Ported | sim/src/announcer.rs:167 | `tools/engine_announcer_reference.py` |
| `ctf_award_capture` | game/game_engine_ctf.c | Matched | sim/src/game.rs | sim/tests/ctf_reference.rs (trace) |
| `ctf_flag_failure_sound` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_find_flag_carrier` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_engine_allow_pick_up` | game/game_engine_ctf.c | Ported | sim/src/game.rs:477 | `tools/engine_ctf_reference.py` |
| `ctf_engine_player_killed_player` | game/game_engine_ctf.c | Partial | sim/src/game.rs:378 | sim/tests/ctf_reference.rs (trace); leaves out the oddball, king, race and juggernaut variants, the game's announcer and score messages |
| `ctf_sound_update_warning` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_set_flag_warning` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_get_score` | game/game_engine_ctf.c | Matched | sim/src/game.rs:348 | sim/tests/ctf_reference.rs (trace) |
| `ctf_test_flag` | game/game_engine_ctf.c | Ported |  | `tools/engine_spawn_reference.py` |
| `ctf_reset_flag` | game/game_engine_ctf.c | Matched | sim/src/game.rs | sim/tests/ctf_reference.rs (trace) |
| `ctf_player_drop_flag` | game/game_engine_ctf.c | Matched | sim/src/game.rs | sim/tests/ctf_reference.rs (trace) |
| `ctf_position_near_flag` | game/game_engine_ctf.c | Ported | sim/src/game.rs:236 | `tools/engine_ctf_reference.py` |
| `ctf_engine_player_update` | game/game_engine_ctf.c | Ported | sim/src/game.rs:476 | `tools/engine_ctf_reference.py` |
| `ctf_engine_weapon_update` | game/game_engine_ctf.c | Ported | sim/src/game.rs:476 | `tools/engine_ctf_reference.py` |
| `ctf_engine_update` | game/game_engine_ctf.c | Matched | sim/src/game.rs:446 | sim/tests/ctf_reference.rs (trace) |
| `ctf_weapon_pickup` | game/game_engine_ctf.c | Ported | sim/src/game.rs:477 | `tools/engine_ctf_reference.py` |
| `ctf_engine_starting_location_rating` | game/game_engine_ctf.c | Ported |  | `tools/engine_spawn_reference.py` |
| `king_engine_game_starting` | game/game_engine_king.c | Ported | sim/src/announcer.rs:167 | `tools/engine_announcer_reference.py` |
| `_game_engine_play_multiplayer_sound` | game/game_engine_multiplayer_sounds.c | Ported |  | `tools/engine_announcer_reference.py` |
| `push_queued_sound` | game/game_engine_multiplayer_sounds.c | Ported |  | `tools/engine_announcer_reference.py` |
| `game_engine_update_multiplayer_sound` | game/game_engine_multiplayer_sounds.c | Ported | sim/src/announcer.rs:147 | `tools/engine_announcer_reference.py` |
| `get_sound_length_in_ticks` | game/game_engine_multiplayer_sounds.c | Ported | sim/src/announcer.rs:65 | `tools/engine_announcer_reference.py` |
| `game_engine_play_multiplayer_sound` | game/game_engine_multiplayer_sounds.c | Ported | sim/src/announcer.rs:125 | `tools/engine_announcer_reference.py` |
| `game_engine_intialize_queued_sounds` | game/game_engine_multiplayer_sounds.c | Ported | sim/src/announcer.rs:112 | `tools/engine_announcer_reference.py` |
| `oddball_engine_game_starting` | game/game_engine_oddball.c | Ported | sim/src/announcer.rs:168 | `tools/engine_announcer_reference.py` |
| `oddball_test_flag` | game/game_engine_oddball.c | Ported |  | `tools/engine_spawn_reference.py` |
| `slayer_engine_player_added` | game/game_engine_slayer.c | Ported | sim/src/game.rs:290 | `tools/engine_game_engine_reference.py` |
| `slayer_engine_game_starting` | game/game_engine_slayer.c | Ported | sim/src/announcer.rs:166 | `tools/engine_announcer_reference.py` |
| `slayer_engine_update` | game/game_engine_slayer.c | Matched | sim/src/game.rs:446 | sim/tests/game_engine_reference.rs (trace) |
| `slayer_engine_adjust_score` | game/game_engine_slayer.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `slayer_get_score` | game/game_engine_slayer.c | Matched | sim/src/game.rs:348 | sim/tests/game_engine_reference.rs (trace) |
| `slayer_test_flag` | game/game_engine_slayer.c | Ported |  | `tools/engine_spawn_reference.py` |
| `slayer_engine_player_killed_player` | game/game_engine_slayer.c | Partial | sim/src/game.rs:377 | sim/tests/game_engine_reference.rs (trace); leaves out the oddball, king, race and juggernaut variants, the game's announcer and score messages, kill in order's targets and the speed for the score |
| `game_statistics_record_kill` | game/game_statistics.c | Partial | sim/src/game.rs:379 | sim/tests/game_engine_reference.rs (trace); leaves out assists, multiple kills and kills in a row, the attackers' damage and credit |
| `game_predicting` | game/game_time.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_time_get_speed` | game/game_time.c | Ported |  | `tools/engine_game_time_reference.py` |
| `game_time_statistics_new` | game/game_time.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `game_time_statistics_frame` | game/game_time.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `game_time_update` | game/game_time.c | Partial | sim/src/clock.rs:2 | sim/tests/game_time_reference.rs (trace); leaves out Halo's 30 Hz tick: the engine's simulation runs at 60 Hz, with the cap doubled to 14, a film's and a network game's caps, the server's time and a client's held clock; traced at the game's 30 Hz and cap of seven, tick for tick and the leftover to the bit |
| `limit2d` | game/player_control.c | Ported |  | `tools/engine_math_reference.py` |
| `interpolate_scalar` | game/player_control.c | Ported |  | `tools/engine_facing_reference.py` |
| `player_control_modify_desired_angles` | game/player_control.c | Partial | sim/src/facing.rs:2 | sim/tests/facing_reference.rs (trace); leaves out a seat's yaw limits and a unit camera's pitch limits, eased in, the vehicle's look levelled (autolevel) |
| `handle_one_player_input` | game/player_control.c | Partial | sim/src/player.rs:584 | sim/tests/facing_reference.rs (trace); leaves out choosing the weapon and grenade, zoom, the vehicle's look levelled (autolevel), debug rotation and the ninja rope, the network client's queue, a seat's and a unit camera's own limits; the look's turn, through player_control_modify_desired_angles |
| `player_control_update` | game/player_control.c | Partial | sim/src/player.rs:584 | sim/tests/facing_reference.rs (trace); leaves out choosing the weapon and grenade, zoom, the vehicle's look levelled (autolevel), debug rotation and the ninja rope, the network client's queue, a seat's and a unit camera's own limits; the look's turn, through player_control_modify_desired_angles |
| `update_client_delete` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `update_client_get_maximum_actions` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `player_new_queue` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `rumble_initialize` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_dispose` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_initialize_for_new_map` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_player_set_scripted_values` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_player_set_scale` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_player_impulse` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_player_clear` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_clear_all_now` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_player_continuous` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_dispose_from_old_map` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_calculate` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `rumble_update` | game/player_rumble.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `unit_should_autopick_weapon` | game/players.c | Ported | sim/src/pickup.rs:6 | `tools/engine_pickup_reference.py` |
| `random_direction3d` | game/players.c | Ported |  | `tools/engine_random_reference.py` |
| `player_new` | game/players.c | Partial | sim/src/lib.rs:765 | sim/tests/game_engine_reference.rs (trace); leaves out the network's player slots, the player's profile and colours, its name, the squad, cluster and aim assist it starts without; a new player has no unit and is added to the game as game_engine_player_added adds it (the game engine trace); the rest of its datum the engine does not hold |
| `find_best_starting_location_index` | game/players.c | Ported | sim/src/game.rs:771 | `tools/engine_spawn_reference.py` |
| `player_spawn` | game/players.c | Partial | sim/src/game.rs:317 | sim/tests/spawn_reference.rs (trace); leaves out the starting equipment's profiles, a campaign respawn's saved unit, a co-op respawn behind its teammate, the unit's colours; its start chosen as the game chooses it (find_best_starting_location_index's trace); the unit placed there and armed from the globals |
| `player_handle_action` | game/players.c | Partial | sim/src/seats.rs:805 | sim/tests/handle_action_reference.rs (trace); leaves out the action's other uses (devices, powerups), the eviction of an AI from a seat, unit_can_enter_seat's own checks; a vehicle turned back over from the side or end the game chooses (vehicle::Approach::of), 3000 flips |
| `players_debug_render` | game/players.c | Replaced | dev/src/draw.rs | the editor and tools |
| `debug_player_teleport` | game/players.c | Replaced | dev/src/control.rs | the editor and tools |
| `players_update_before_game` | game/players.c | Partial | sim/src/lib.rs:394 | sim/tests/game_tick_reference.rs (trace); leaves out the network's prediction, the players' respawn timers, co-op's stranded players rescued, the actions' checks (its asserts); only its place in the tick is traced (game_tick); its body is the game loop track's |
| `player_examine_nearby_item` | game/players.c | Partial | sim/src/pickup.rs:2 | sim/tests/pickup_reference.rs (trace); leaves out picking up equipment and powerups, the grenades' pickup, the action's priority among the objects nearby (player_set_action_result); taken, swapped or left as the game chooses (unit_should_autopick_weapon, unit_approve_weapon_swap), 4000 cases; the engine carries two weapons where the game's places are four |
| `player_examine_nearby_objects` | game/players.c | Partial | sim/src/hud.rs:318 | sim/tests/pickup_reference.rs (trace); leaves out the objects found by objects_in_sphere (the engine looks within the pickup radius), the nearby bipeds and devices; the weapons examined as the game examines them (player_examine_nearby_item's trace); the vehicles' seats are offered by sim/src/seats.rs |
| `players_update_after_game` | game/players.c | Partial | campaign/src/lib.rs:475 | sim/tests/game_tick_reference.rs (trace); leaves out the telefrag, the powerups' timers, double speed, co-op's wait for the host; only its place in the tick is traced (game_tick); the BSP switch and the end of a level are checked by campaign/tests/flow.rs; its body is the game loop track's |
| `game_engine_network_player_score` | game/game_engine.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_write_network_state` | game/game_engine.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_read_network_state` | game/game_engine.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_ctf_write_network_state` | game/game_engine_ctf.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_ctf_read_network_state` | game/game_engine_ctf.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_king_write_network_state` | game/game_engine_king.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `king_client_score_sounds` | game/game_engine_king.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_king_read_network_state` | game/game_engine_king.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `oddball_client_score_sounds` | game/game_engine_oddball.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_oddball_write_network_state` | game/game_engine_oddball.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_oddball_read_network_state` | game/game_engine_oddball.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_race_write_network_state` | game/game_engine_race.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_race_read_network_state` | game/game_engine_race.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_slayer_write_network_state` | game/game_engine_slayer.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `game_engine_slayer_read_network_state` | game/game_engine_slayer.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `distributed_action_valid` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `distributed_add_new_ticks` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `update_server_handle_distributed_input` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `update_server_ticked_update_number` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `update_server_update_actions` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `update_client_handle_relayed_action` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `update_client_distributed_input` | game/player_queues_new.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `network_player_attach_unit` | game/players.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `network_player_show_pickup` | game/players.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `network_player_detach_unit` | game/players.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |

565 Missing (549 reached from `main_loop`), by file:

- game/aim_assist.c: `compute_attenuation`, `compute_composite_attenuation`, `unit_get_aim_assist_parameters`, `compare_targets`, `aim_assist_clear_line_of_sight`, `object_compute_autoaim_target`, `aim_assist_compute_target`, `autoaim_compute_target`, `find_aim_assist_targets_recursive`, `find_aim_assist_targets`, `aim_assist`, `player_aim_projectile`, `local_player_aim_assist`
- game/game.c: `game_initialize`, `game_dispose`, `game_precache_new_map`, `game_map_loading_in_progress`, `game_unload`, `game_dispose_from_old_map`, `game_frame`, `remove_quitting_players_from_game`, `game_options_new`, `game_options_verify`, `game_set_players_are_double_speed`, `game_players_are_double_speed`, `game_difficulty_level_set`, `game_difficulty_level_get`, `game_difficulty_level_get_ignore_easy`, `game_set_game_variant`, `game_set_game_engine_index`, `game_load`, `game_initialize_for_new_map`, `game_set_game_variant_from_name`, `game_set_game_variant_options`, `game_variant_options_get`
- game/game_allegiance.c: `game_allegiance_initialize`, `game_allegiance_dispose`, `game_allegiance_initialize_for_new_map`, `game_allegiance_dispose_from_old_map`, `game_team_is_enemy`, `game_team_is_ally`, `game_team_ally_status_changed`, `game_allegiance_get_incidents`, `game_allegiance_provoke`, `game_allegiance_notify_change`, `game_allegiance_broken`, `game_allegiance_update`, `game_allegiance_create`, `game_allegiance_remove`, `game_allegiance_incident`, `game_allegiance_set_teams`
- game/game_engine.c: `game_globals_get_weapon`, `linear_to_non_linear_alpha`, `game_engine_dispose`, `game_engine_build_lighting`, `game_engine_dispose_from_old_map`, `game_engine_game_ending`, `game_engine_statistics_append`, `sort_statistic_buffer`, `sort_statistic_buffer_ranking`, `is_place_tied`, `place_get_position`, `get_place_string`, `drawline`, `get_selected_color`, `get_postgame_hilite_colors`, `find_closest_player_callback`, `game_engine_post_rasterize_objects`, `can_delete_item`, `update_weapon_inventory`, `game_engine_update_weapons`, `game_engine_load_stage`, `game_engine_playlist_begin`, `game_engine_get_current_stage`, `game_engine_end_game`, `game_engine_allow_pick_up`, `game_engine_player_damaged_player`, `game_engine_player_is_odd_man_out`, `game_engine_player_is_out_of_lives`, `game_engine_should_spawn_player`, `game_engine_player_get_team_index`, `game_engine_prespawn_player_update`, `game_engine_running`, `game_engine_can_score`, `game_engine_force_single_screen`, `get_blink_alpha`, `game_engine_press_start_to_begin`, `game_engine_rasterize_message`, `game_engine_picking_up`, `goal_matches_player`, `game_engine_player_get_custom_motion_sensor_positions`, `get_flag_definition_index`, `get_ball_definition_index`, `game_engine_switch_to_postgame`, `game_engine_get_goal_in_use`, `game_engine_get_goal_position`, `game_engine_set_goal_position`, `game_engine_clear_goal_position`, `game_engine_render_nav_points`, `game_engine_infinite_grenades_internal`, `game_engine_infinite_grenades`, `game_engine_display_team_indicators`, `game_engine_has_shield`, `weapon_definition_index_to_list_index`, `list_index_to_weapon_definition_index`, `game_engine_remap_vehicle`, `game_engine_remap_weapon`, `game_engine_man_out`, `game_engine_state_message`, `game_engine_player_get_change_color`, `game_engine_player_depower_active_camo`, `get_place_name`, `game_engine_get_place`, `game_engine_hud_draw_motion_sensor`, `game_engine_test_trait`, `netgame_flag_verify_no_team_duplicates`, `netgame_flag_verify_team_range`, `game_engine_playlist_next`, `build_game_variant_slayer_pro`, `build_game_variant_elimination`, `build_game_variant_phantoms`, `build_game_variant_endurance`, `build_game_variant_rockets`, `build_game_variant_snipers`, `build_game_variant_oddball`, `build_game_variant_team_oddball`, `build_game_variant_reverse_tag`, `build_game_variant_accumulation`, `build_game_variant_juggernaut`, `build_game_variant_stalker`, `build_game_variant_king`, `build_game_variant_king_pro`, `build_game_variant_crazy_king`, `build_game_variant_team_king`, `build_game_variant_ctf_pro`, `build_game_variant_invasion`, `build_game_variant_iron_ctf`, `build_game_variant_race`, `build_game_variant_rally`, `build_game_variant_team_race`, `build_game_variant_team_rally`, `game_engine_override_map_name`, `game_engine_override_game_variant`, `rasterize_in_game_score_draw_line`, `game_engine_hud_draw_messages`, `game_engine_player_has_stealth_weapon`, `game_engine_weapon_fired`, `test_any_gamepad_button`, `ticks_to_unicode_time_string`, `game_engine_variant_cleanup`, `game_engine_allow_pause`, `game_engine_predict_resources`, `game_engine_draw_object_in_motion_sensor`, `game_engine_allow_dynamic_lighting`, `game_engine_allow_integrated_lights`, `game_engine_force_autopickup`, `game_engine_initialize`, `team_has_players`, `adjust_score_for_ranking`, `populate_statistic_buffer`, `game_engine_get_player_place`, `postgame_statistic_get_rating`, `select_players_to_display`, `find_closest_player_index`, `internal_rasterize_target_name`, `internal_rasterize_score`, `game_engine_get_type`, `game_engine_update_player_no_shield`, `game_engine_update_player_always_invis`, `game_engine_update_non_deterministic`, `multiplayer_message_internal`, `multiplayer_message`, `game_show_score_one_player`, `game_show_score_team`, `game_show_score_you_ally_enemy`, `game_show_score_extended`, `game_show_score`, `find_netgame_flags`, `find_netgame_flag`, `handle_custom_starting_equipment`, `game_engine_postspawn_player_update`, `game_engine_get_damage_multiplier`, `game_engine_update_teleporter`, `game_engine_get_variant_by_name`, `game_engine_remap_equipment`, `game_engine_remap_object_definition`, `game_engine_get_state_message`, `game_engine_did_player_win_default`, `game_engine_did_player_win`, `game_engine_did_team_win`, `netgame_flag_verify_team_exists`, `netgame_verify_spawn_points`, `netgame_verify_equipment`, `game_engine_verify_current_map`, `game_engine_playlist_initialize`, `game_engine_generate_title_string`, `game_engine_post_rasterize_post_game`, `game_engine_nonplayer_post_rasterize`, `game_engine_rasterize_in_game_score`, `game_engine_post_rasterize_in_game`, `game_engine_post_rasterize`, `scoreboard_background_color`, `scoreboard_draw_row`, `scoreboard_team_columns`, `game_engine_scoreboard_closed`, `scoreboard_menu_items`, `scoreboard_menu_first_row`, `scoreboard_rectangle`, `scoreboard_point_in`, `scoreboard_menu_pick`, `game_engine_rasterize_scoreboard`, `game_engine_rasterize_voice_speakers`, `game_variant_options_default`, `game_engine_vehicle_empty`, `game_engine_vehicle_home_on_pad`, `game_engine_vehicle_home_follow`, `game_engine_vehicle_home_current`, `game_engine_update_vehicle_respawn`, `game_engine_predict_multiplayer_vehicle`, `game_engine_variant_vehicle_type`, `game_engine_nearest_team`, `game_engine_vehicle_placement_allowed`
- game/game_engine_ctf.c: `ctf_create_flag_object`, `ctf_engine_dispose`, `create_the_flag`, `ctf_single_flag_what_is_up_message`, `ctf_engine_dispose_from_old_map`, `ctf_engine_player_added`, `ctf_engine_game_ending`, `ctf_engine_statistics_append`, `ctf_engine_handle_client_message`, `ctf_engine_handle_server_message`, `ctf_engine_pregame_post_rasterize`, `ctf_engine_post_rasterize`, `ctf_engine_player_damaged_player`, `ctf_engine_display_score`, `ctf_engine_prespawn_player_update`, `ctf_state_message_update_warning`, `ctf_weapon_drop`, `ctf_get_score_string`, `ctf_get_score_header_string`, `ctf_get_team_score_string`, `ctf_engine_initialize_for_new_map`, `ctf_show_grab`, `ctf_show_touch_return`, `ctf_show_untouched_return`, `ctf_client_flag_failure_update`
- game/game_engine_king.c: `king_engine_dispose`, `point3d_to_point2d`, `find_hill`, `king_engine_dispose_from_old_map`, `king_engine_player_added`, `king_engine_game_ending`, `king_engine_statistics_append`, `king_engine_handle_client_message`, `king_engine_handle_server_message`, `king_engine_pregame_post_rasterize`, `player_inside_hill`, `king_engine_player_update`, `king_calculate_hill_state`, `king_engine_player_damaged_player`, `king_engine_player_killed_player`, `king_engine_display_score`, `king_engine_prespawn_player_update`, `king_get_score`, `render_dynamic_quad_initialize`, `render_dynamic_quad`, `king_get_score_string`, `king_get_score_header_string`, `king_get_team_score_string`, `king_engine_goal_matches_player`, `find_next_hill`, `king_engine_initialize_for_new_map`, `king_engine_post_rasterize`, `king_engine_update`
- game/game_engine_oddball.c: `oddball_engine_dispose`, `oddball_engine_dispose_from_old_map`, `oddball_engine_player_added`, `oddball_engine_game_ending`, `oddball_engine_statistics_append`, `oddball_engine_handle_client_message`, `oddball_engine_handle_server_message`, `oddball_engine_pregame_post_rasterize`, `oddball_engine_post_rasterize`, `oddball_add_score`, `oddball_add_time_with_ball`, `player_ball_count`, `oddball_engine_player_damaged_player`, `player_has_ball`, `ball_available`, `oddball_engine_display_score`, `oddball_engine_prespawn_player_update`, `oddball_weapon_drop`, `oddball_get_score`, `oddball_ball_transfer_by_killing`, `accumulate_score_by_time`, `terminator_scoring_rules`, `oddball_test_trait`, `oddball_get_score_string`, `oddball_get_score_header_string`, `oddball_get_team_score_string`, `find_position_for_ball`, `create_the_ball`, `oddball_engine_initialize_for_new_map`, `reset_ball`, `update_ball_ownership`, `oddball_engine_player_update`, `oddball_engine_weapon_update`, `oddball_engine_update`, `oddball_engine_player_killed_player`, `oddball_weapon_pickup`
- game/game_engine_race.c: `race_engine_dispose`, `delete_race_vehicles`, `race_get_vehicle_to_spawn`, `race_flags_make_unique`, `race_engine_dispose_from_old_map`, `race_engine_player_added`, `race_engine_game_ending`, `race_engine_game_starting`, `race_engine_statistics_append`, `race_engine_handle_client_message`, `race_engine_handle_server_message`, `race_engine_pregame_post_rasterize`, `race_engine_post_rasterize`, `race_complete_lap`, `can_touch_team`, `race_engine_weapon_update`, `race_team_can_win_game`, `build_player_speeds`, `race_engine_player_damaged_player`, `race_engine_player_killed_player`, `race_engine_display_score`, `race_engine_prespawn_player_update`, `race_goal_matches_player`, `count_bits_32`, `race_engine_get_score`, `race_get_score_string`, `race_get_score_header_string`, `race_get_team_score_string`, `race_engine_did_player_win`, `find_closest_vehicle`, `spawn_race_vehicles`, `new_rally_flag`, `race_touch_flag`, `race_engine_player_update`, `race_engine_update`, `race_engine_initialize_for_new_map`
- game/game_engine_slayer.c: `target_is_eligible`, `slayer_engine_dispose`, `slayer_engine_initialize_for_new_map`, `slayer_engine_dispose_from_old_map`, `slayer_engine_game_ending`, `slayer_engine_statistics_append`, `slayer_engine_handle_client_message`, `slayer_engine_handle_server_message`, `slayer_engine_pregame_post_rasterize`, `slayer_engine_post_rasterize`, `slayer_engine_allow_pick_up`, `slayer_engine_player_damaged_player`, `update_speed_for_score`, `slayer_engine_prespawn_player_update`, `slayer_get_score_string`, `slayer_get_score_header_string`, `slayer_get_team_score_string`, `find_next_target`, `slayer_engine_display_score`, `slayer_player_update`
- game/game_engine_stub.c: `stub_engine_dispose`, `stub_engine_initialize_for_new_map`, `stub_engine_dispose_from_old_map`, `stub_engine_player_added`, `stub_engine_game_ending`, `stub_engine_game_starting`, `stub_engine_statistics_append`, `stub_engine_handle_client_message`, `stub_engine_handle_server_message`, `stub_engine_pregame_post_rasterize`, `stub_engine_post_rasterize`, `stub_engine_update`, `stub_engine_allow_pick_up`, `stub_engine_player_damaged_player`, `stub_engine_player_killed_player`
- game/game_globals.c: `material_get_name`, `game_difficulty_get_value_by_difficulty`, `game_difficulty_get_value`, `game_difficulty_get_team_value`
- game/game_statistics.c: `game_statistics_start`, `game_statistics_stop`, `game_statistics_record_damage`
- game/game_time.c: `game_time_initialized`, `game_time_initialize`, `game_time_initialize_for_new_map`, `game_time_dispose_from_old_map`, `game_time_dispose`, `game_time_end`, `game_time_get`, `game_time_get_elapsed`, `local_time_get`, `local_time_get_elapsed`, `game_in_progress`, `game_time_get_paused`, `game_time_set_paused`, `game_time_set_speed`, `game_time_start`, `game_time_set_distributed`, `game_time_get_tick_fraction`, `game_time_held`
- game/player_control.c: `player_control_get`, `player_control_initialize`, `player_control_dispose`, `player_control_dispose_from_old_map`, `player_control_camera_control_is_active`, `scripted_player_control_set_camera_control`, `player_control_inhibit_buttons`, `evaluate_piecewise_linear_function`, `player_action_clear`, `player_control_get_aiming_unit_index`, `player_control_get_target_object_index`, `player_control_get_field_of_view`, `player_control_get_unit_camera_info`, `player_control_get_unit_index`, `player_control_get_desired_weapon`, `player_control_get_autoaim_level`, `unit_get_local_player_index`, `players_unzoom_all`, `player_control_unzoom`, `player_control_get_zoom_level`, `player_control_action_test_reset`, `player_control_action_test_accept`, `player_control_action_test_back`, `player_control_action_test_action`, `player_control_action_test_jump`, `player_control_action_test_primary_trigger`, `player_control_action_test_grenade_trigger`, `player_control_action_test_zoom`, `player_control_action_test_move_relative_all_directions`, `player_control_action_test_look_relative_all_directions`, `player_control_action_test_look_relative_left`, `player_control_action_test_look_relative_right`, `player_control_action_test_look_relative_up`, `player_control_action_test_look_relative_down`, `player_control_action_test_check_reset_input_blob`, `signed_angular_difference`, `player_control_set_facing`, `player_control_new_unit`, `get_local_player_input_blob`, `player_control_get_facing_angles`, `player_control_get_facing_direction`, `player_control_set_desired_weapon`, `player_control_initialize_for_new_map`, `player_control_permanent_impulse`, `player_control_action_test_note`
- game/player_queues_new.c: `update_server_add_player`, `update_client_new`, `update_client_start`, `update_client_add_player`, `update_client_queue`, `update_client_queue_push`, `update_client_build_client_update`, `update_server_get_update`, `update_client_get_update`, `update_server_new`, `update_server_delete`, `update_server_start`, `update_server_build_server_update`, `update_client_dequeue`, `update_client_get_maximum_possible_server_time`, `update_server_handle_client_update`, `update_client_handle_server_update`, `update_queues_reset_and_fill_with_lies`, `update_server_next_update`, `update_client_local_ticks`, `update_client_local_player_index`, `update_client_dequeue_distributed`, `update_queues_distributed_reset`, `update_server_take_local_actions`
- game/players.c: `players_initialize`, `players_initialize_for_new_map`, `players_dispose_from_old_map`, `players_dispose`, `machine_get_player_list`, `machine_add_player`, `local_player_exists`, `find_unused_local_player_index`, `player_delete`, `players_get_respawn_failure`, `local_player_get_player_index`, `local_player_set_player_index`, `local_player_count`, `local_player_get_next`, `player_index_from_unit_index`, `player_died`, `players_are_all_dead`, `players_set_local_player_unit`, `players_get_combined_pvs_local`, `players_get_combined_pvs`, `player_input_enable`, `player_input_enabled`, `any_player_is_in_the_air`, `any_player_is_dead`, `is_player_in_trigger`, `player_pseudo_kill`, `player_control_fix_for_loaded_game_state`, `player_get_starting_location_count`, `player_get_starting_location`, `placement_data_set_change_color`, `create_weapon`, `players_compute_combined_pvs`, `players_compute_local_player_count`, `player_examine_nearby_unit`, `player_reset_action_result`, `player_handle_weapon_swap`, `player_over_shield_screen_effect`, `player_active_camo_screen_effect`, `player_health_pack_screen_effect`, `player_powerup_on`, `player_powerup_additional`, `player_powerup_off`, `players_handle_deleted_object`, `valid_real_vector2d`, `player_add_equipment`, `player_aiming_vector_from_facing`, `player_teleport_internal`, `player_teleport`, `player_set_action_result`, `player_handle_powerup`, `player_handle_powerup_minor`, `player_update_powerups`, `players_respawn_coop`, `player_teleport_on_bsp_switch`, `players_reconnect_to_structure_bsp`, `player_examine_nearby_vehicle`, `player_examine_nearby_device`, `player_handle_powerup_equipment`, `network_player_log_idle_action`, `players_coop_extras_held`, `players_coop_may_spawn`, `players_coop_waiting_to_start`, `players_coop_danger_near`, `players_coop_unit_in_structure`, `players_coop_unit_grounded`, `players_coop_unit_safe`, `players_coop_unit_where`, `players_coop_session_count`, `players_coop_use_majority`, `players_coop_unit_spawnable`, `players_coop_near_count`, `players_coop_respawn_unit`, `players_coop_spawn_location`, `players_respawn_network_coop`, `player_spawnable_beside`, `players_checkpoint_position`, `players_coop_unit_grounded_anywhere`, `players_coop_unit_stranded`, `players_coop_rescue_stranded`, `players_coop_note_on_foot`, `players_coop_room_to_spawn`, `player_place_beside_teammate`, `players_coop_respawn_beside`, `players_note_checkpoint`, `players_respawn_at_checkpoint`, `player_position_switches_bsp`, `players_coop_bsp_switch_ready`, `players_coop_bsp_switch_allowed`, `player_name_clean`

## `hs`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `debug_sound_classes_set_distances_evaluate` | hs/hs.c | Replaced | dev/src/script.rs | the editor and tools |
| `ai_debug_vocalize_evaluate` | hs/hs.c | Replaced | dev/src/script.rs | the editor and tools |
| `debug_sound_classes_enable_evaluate` | hs/hs.c | Replaced | dev/src/script.rs | the editor and tools |
| `hs_scenario_merge` | hs/hs.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `hs_recompile` | hs/hs.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `hs_hack` | hs/hs.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `hs_parse_debug_string` | hs/hs_compile.c | Replaced | dev/src/script.rs | the editor and tools |
| `hs_compile_expression` | hs/hs_compile.c | Partial | script/src/console.rs:166 | script/tests/console.rs (measure); leaves out the scenario's scripts (compiled at import), the compiler's errors as the original words them |
| `hs_compile` | hs/hs_compile.c | Partial | script/src/console.rs | script/tests/console.rs (measure); leaves out the scenario's source text: maps carry their scripts compiled, so only the console's line is compiled |
| `hs_not` | hs/hs_library_external.c | Matched | campaign/src/game.rs | campaign/tests/natives.rs (measure) |
| `hs_print` | hs/hs_library_external.c | Matched | campaign/src/game.rs | campaign/tests/natives.rs (measure) |
| `hs_players` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/natives.rs (measure) |
| `hs_objects_can_see_object` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/natives.rs (measure) |
| `hs_objects_can_see_flag` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/natives.rs (measure) |
| `hs_object_create` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `hs_object_destroy` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `hs_object_destroy_all` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/natives.rs (measure) |
| `hs_object_create_containing` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/natives.rs (measure) |
| `hs_object_destroy_containing` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/natives.rs (measure) |
| `hs_object_create_anew` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `hs_object_create_anew_containing` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/natives.rs (measure) |
| `hs_object_teleport` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/natives.rs (measure) |
| `hs_object_set_facing` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | campaign/tests/natives.rs (measure) |
| `hs_wake` | hs/hs_runtime.c | Matched | script/src/lib.rs:469 | script/tests/runtime.rs (measure) |
| `hs_long_to_boolean` | hs/hs_runtime.c | Partial | script/src/value.rs | script/tests/script_cast_reference.rs (trace); leaves out the decompilation's n==0, which makes zero true; the engine takes zero as false until the user's maps settle it |
| `hs_short_to_boolean` | hs/hs_runtime.c | Partial | script/src/value.rs | script/tests/script_cast_reference.rs (trace); leaves out the decompilation's n==0, which makes zero true; the engine takes zero as false until the user's maps settle it |
| `hs_string_to_boolean` | hs/hs_runtime.c | Partial | script/src/value.rs | script/tests/script_cast_reference.rs (trace); leaves out the decompilation's n==0, which makes zero true; the engine takes zero as false until the user's maps settle it |
| `hs_data_to_void` | hs/hs_runtime.c | Matched | script/src/value.rs | script/tests/script_cast_reference.rs (trace) |
| `hs_short_to_real` | hs/hs_runtime.c | Matched | script/src/value.rs | script/tests/script_cast_reference.rs (trace) |
| `hs_long_to_real` | hs/hs_runtime.c | Matched | script/src/value.rs | script/tests/script_cast_reference.rs (trace) |
| `hs_enum_to_real` | hs/hs_runtime.c | Partial | script/src/value.rs | script/tests/script_cast_reference.rs (trace); leaves out the one hs_enum_to_real adds to the index, the enum rows of the table, one place early |
| `hs_real_to_short` | hs/hs_runtime.c | Matched | script/src/value.rs | script/tests/script_cast_reference.rs (trace) |
| `hs_real_to_long` | hs/hs_runtime.c | Matched | script/src/value.rs | script/tests/script_cast_reference.rs (trace) |
| `hs_long_to_short` | hs/hs_runtime.c | Matched | script/src/value.rs | script/tests/script_cast_reference.rs (trace) |
| `hs_object_name_to_object_list` | hs/hs_runtime.c | Matched | script/src/value.rs | script/tests/script_cast_reference.rs (trace) |
| `hs_object_to_object_list` | hs/hs_runtime.c | Matched | script/src/value.rs | script/tests/script_cast_reference.rs (trace) |
| `hs_object_type_can_cast` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_can_cast` | hs/hs_runtime.c | Ported | import/src/script_tables.rs:881 | `tools/engine_script_cast_reference.py` |
| `hs_cast` | hs/hs_runtime.c | Partial | script/src/value.rs:88 | script/tests/script_cast_reference.rs (trace); leaves out typecasting_procedures as the decompilation miscounts it: the ai, enum and object rows each a place earlier than the last |
| `render_debug_scripting` | hs/hs_runtime.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_trigger_volumes` | hs/hs_runtime.c | Replaced | dev/src/draw.rs | the editor and tools |
| `hs_return` | hs/hs_runtime.c | Matched | script/src/lib.rs:227 | script/tests/runtime.rs (measure) |
| `hs_global_evaluate` | hs/hs_runtime.c | Matched | script/src/lib.rs:979 | script/tests/runtime.rs (measure) |
| `hs_evaluate` | hs/hs_runtime.c | Matched | script/src/lib.rs:609 | script/tests/runtime.rs (measure) |
| `hs_arguments_evaluate` | hs/hs_runtime.c | Matched | script/src/lib.rs:609 | script/tests/runtime.rs (measure) |
| `hs_evaluate_begin` | hs/hs_runtime.c | Matched | script/src/lib.rs:728 | script/tests/runtime.rs (measure) |
| `hs_evaluate_begin_random` | hs/hs_runtime.c | Partial | script/src/lib.rs:728 | script/tests/runtime.rs (measure); leaves out the game's own random numbers: the shuffle draws on the runtime's xorshift |
| `hs_evaluate_if` | hs/hs_runtime.c | Matched | script/src/lib.rs:729 | script/tests/runtime.rs (measure) |
| `hs_evaluate_set` | hs/hs_runtime.c | Matched | script/src/lib.rs:729 | script/tests/runtime.rs (measure) |
| `hs_evaluate_logical` | hs/hs_runtime.c | Matched | script/src/lib.rs:729 | script/tests/runtime.rs (measure) |
| `hs_evaluate_arithmetic` | hs/hs_runtime.c | Matched | script/src/lib.rs:730 | script/tests/runtime.rs (measure) |
| `hs_evaluate_equality` | hs/hs_runtime.c | Matched | script/src/lib.rs:730 | script/tests/runtime.rs (measure) |
| `hs_evaluate_inequality` | hs/hs_runtime.c | Matched | script/src/lib.rs:731 | script/tests/runtime.rs (measure) |
| `hs_evaluate_sleep` | hs/hs_runtime.c | Matched | script/src/lib.rs:890 | script/tests/runtime.rs (measure) |
| `hs_evaluate_sleep_until` | hs/hs_runtime.c | Matched | script/src/lib.rs:927 | script/tests/runtime.rs (measure) |
| `hs_evaluate_inspect` | hs/hs_runtime.c | Matched | script/src/lib.rs:731 | script/tests/runtime.rs (measure) |
| `hs_evaluate_debug_string` | hs/hs_runtime.c | Replaced | dev/src/script.rs | the editor and tools |
| `hs_thread_main` | hs/hs_runtime.c | Matched | script/src/lib.rs:571 | script/tests/runtime.rs (measure) |
| `hs_runtime_initialize_for_new_map` | hs/hs_runtime.c | Matched | script/src/lib.rs:307 | script/tests/runtime.rs (measure) |
| `hs_runtime_update` | hs/hs_runtime.c | Matched | script/src/lib.rs:395 | script/tests/runtime.rs (measure) |
| `hs_runtime_evaluate` | hs/hs_runtime.c | Matched | script/src/lib.rs:478 | script/tests/runtime.rs (measure) |
| `hs_syntax_data_byte_swap` | hs/hs_scenario_definitions.c | Replaced | import/src/lib.rs | maps imported to a typed format; byte order is settled when the map is imported |
| `hs_type_default_value` | hs/hs_runtime.c | Matched | script/src/value.rs:33 | script/tests/values.rs (measure) |

576 Missing (571 reached from `main_loop`), by file:

- hs/hs.c: `hs_sv_say`, `hs_sv_end_game`, `hs_sv_say_evaluate`, `hs_sv_end_game_evaluate`, `hs_mods_reload_evaluate`, `hs_mods_enable_evaluate`, `hs_mods_list_evaluate`, `hs_scenario_syntax_data_valid`, `hs_syntax_node_refusal`, `hs_scenario_functions_check`, `hs_scenario_script_disabled`, `hs_scenario_global_initializer_disabled`, `hs_allocate`, `hs_compile_source_error`, `hs_compile_source`, `hs_rebuild_source_file`, `hs_rebuild_source`, `hs_get_function_parameters_string`, `hs_get_function_documentation_string`, `hs_tokens_enumerate_add_string`, `hs_enumerate_block_data`, `hs_enumerate_scenario_data`, `hs_enumerate_from_string_list`, `hs_enumerate_special_form_names`, `hs_enumerate_script_type_names`, `hs_enumerate_type_names`, `hs_enumerate_function_names`, `hs_enumerate_script_names`, `hs_enumerate_variable_names`, `hs_enumerate_ai_names`, `hs_enumerate_ai_command_list_names`, `hs_enumerate_starting_profile_names`, `hs_enumerate_conversation_names`, `hs_enumerate_object_names`, `hs_enumerate_trigger_volume_names`, `hs_enumerate_cutscene_flag_names`, `hs_enumerate_cutscene_camera_point_names`, `hs_enumerate_cutscene_title_names`, `hs_enumerate_cutscene_recording_names`, `hs_enumerate_navpoints`, `hs_enumerate_hud_messages`, `alphabetize_file_references`, `alphabetize`, `main_saving_map_evaluate`, `game_state_reverted_evaluate`, `hs_object_destroy_all_evaluate`, `numeric_countdown_timer_stop_evaluate`, `numeric_countdown_timer_restart_evaluate`, `objects_dump_memory_evaluate`, `garbage_collect_now_evaluate`, `object_pvs_clear_evaluate`, `breakable_surfaces_reset_evaluate`, `cheat_all_powerups_evaluate`, `cheat_all_weapons_evaluate`, `cheat_all_vehicles_evaluate`, `cheat_teleport_to_camera_evaluate`, `cheat_active_camouflage_evaluate`, `scripting_magic_melee_attack_evaluate`, `cheats_load_evaluate`, `ai_scripting_erase_all_evaluate`, `ai_scripting_deselect_evaluate`, `ai_scripting_reconnect_evaluate`, `director_save_camera_evaluate`, `director_load_camera_evaluate`, `players_unzoom_all_evaluate`, `player_control_action_test_reset_evaluate`, `main_reset_map_evaluate`, `main_print_version_evaluate`, `main_set_game_connection_to_film_playback_evaluate`, `texture_cache_flush_evaluate`, `sound_cache_flush_evaluate`, `debug_dump_memory_evaluate`, `debug_dump_memory_by_file_evaluate`, `profile_initialize_evaluate`, `ai_profile_change_render_spray_evaluate`, `ai_debug_sound_point_set_evaluate`, `cinematic_start_evaluate`, `cinematic_stop_evaluate`, `cinematic_skip_start_evaluate`, `cinematic_skip_stop_evaluate`, `attract_mode_start_evaluate`, `main_won_map_evaluate`, `main_lost_map_evaluate`, `main_save_map_safe_evaluate`, `main_save_cancel_evaluate`, `main_save_map_no_timeout_evaluate`, `main_save_map_nonsafe_evaluate`, `main_revert_map_evaluate`, `main_load_core_evaluate`, `main_load_core_at_startup_evaluate`, `main_save_core_evaluate`, `scripted_hud_restart_flashing_evaluate`, `terminal_clear_evaluate`, `structure_lens_flares_place_evaluate`, `scripted_hud_messages_clear_evaluate`, `scripted_hud_time_code_reset_evaluate`, `rasterizer_decals_flush_evaluate`, `rasterizer_fps_accumulate_evaluate`, `rasterizer_lights_reset_for_new_map_evaluate`, `rasterizer_screen_effect_stop_evaluate`, `enumerate_memory_units_test_evaluate`, `saved_game_files_delete_all_custom_profiles_evaluate`, `player_ui_fast_setup_network_server_evaluate`, `player_ui_activate_all_solo_levels_evaluate`, `network_game_client_request_immediate_start_evaluate`, `hs_doc_evaluate`, `tag_groups_dump_memory_evaluate`, `hs_players_evaluate`, `game_time_get_evaluate`, `scripted_camera_time_evaluate`, `game_difficulty_level_get_ignore_easy_evaluate`, `game_difficulty_level_get_evaluate`, `global_structure_bsp_index_get_evaluate`, `scripted_hud_get_timer_ticks_evaluate`, `object_list_count_evaluate`, `numeric_countdown_timer_get_evaluate`, `recorded_animation_get_time_left_evaluate`, `scenery_get_animation_time_evaluate`, `unit_get_custom_animation_time_evaluate`, `unit_scripting_get_grenade_count_evaluate`, `ai_scripting_command_list_status_evaluate`, `ai_scripting_going_to_vehicle_evaluate`, `ai_scripting_living_count_evaluate`, `ai_scripting_swarm_count_evaluate`, `ai_scripting_nonswarm_count_evaluate`, `ai_scripting_status_evaluate`, `ai_scripting_conversation_line_evaluate`, `ai_scripting_conversation_status_evaluate`, `vehicle_scripting_load_magic_evaluate`, `vehicle_scripting_unload_evaluate`, `unit_scripting_unit_riders_evaluate`, `unit_scripting_unit_driver_evaluate`, `unit_scripting_unit_gunner_evaluate`, `object_list_from_ai_reference_evaluate`, `scripted_sound_time_evaluate`, `hs_object_destroy_evaluate`, `recorded_animation_kill_evaluate`, `object_cannot_take_damage_evaluate`, `object_can_take_damage_evaluate`, `hs_objects_predict_evaluate`, `object_definition_predict_evaluate`, `object_pvs_set_object_evaluate`, `object_pvs_activate_evaluate`, `unit_open_evaluate`, `unit_close_evaluate`, `unit_kill_evaluate`, `unit_kill_silent_evaluate`, `unit_stop_custom_animation_evaluate`, `unit_scripting_exit_vehicle_evaluate`, `unit_scripting_doesnt_drop_items_evaluate`, `ai_scripting_free_evaluate`, `ai_scripting_free_units_evaluate`, `ai_scripting_detach_unit_evaluate`, `ai_scripting_detach_units_evaluate`, `ai_scripting_place_evaluate`, `ai_scripting_kill_evaluate`, `ai_scripting_kill_silent_evaluate`, `ai_scripting_erase_evaluate`, `ai_scripting_select_evaluate`, `ai_scripting_spawn_actor_evaluate`, `ai_scripting_magically_see_players_evaluate`, `ai_scripting_timer_start_evaluate`, `ai_scripting_timer_expire_evaluate`, `ai_scripting_attack_evaluate`, `ai_scripting_defend_evaluate`, `ai_scripting_retreat_evaluate`, `hs_object_create_evaluate`, `hs_object_create_anew_evaluate`, `object_pvs_set_camera_point_evaluate`, `cheat_active_camouflage_local_player_evaluate`, `breakable_surfaces_enable_evaluate`, `render_effects_evaluate`, `ai_globals_ai_active_evaluate`, `ai_globals_dialogue_triggers_enabled_evaluate`, `ai_globals_grenades_enabled_evaluate`, `hs_print_evaluate`, `hs_object_create_containing_evaluate`, `hs_object_create_anew_containing_evaluate`, `hs_object_destroy_containing_evaluate`, `hs_objects_delete_by_definition_evaluate`, `scripting_set_magic_base_seat_evaluate`, `object_set_ranged_attack_inhibited_evaluate`, `object_set_melee_attack_inhibited_evaluate`, `object_scripting_set_collideable_evaluate`, `unit_scripting_can_blink_evaluate`, `unit_aim_without_turning_evaluate`, `unit_set_enterable_by_player_evaluate`, `unit_scripting_impervious_evaluate`, `unit_scripting_suspended_evaluate`, `units_set_desired_flashlight_state_evaluate`, `unit_set_desired_flashlight_state_evaluate`, `device_set_never_appears_locked_evaluate`, `device_one_sided_set_evaluate`, `device_operates_automatically_set_evaluate`, `ai_scripting_set_respawn_evaluate`, `ai_scripting_set_deaf_evaluate`, `ai_scripting_set_blind_evaluate`, `hs_damage_object_evaluate`, `objects_scripting_detach_evaluate`, `ai_scripting_attach_unit_evaluate`, `ai_scripting_attach_units_evaluate`, `ai_scripting_attach_free_evaluate`, `ai_scripting_magically_see_encounter_evaluate`, `ai_scripting_magically_see_unit_evaluate`, `ai_scripting_magically_see_units_evaluate`, `hs_object_teleport_evaluate`, `hs_object_set_facing_evaluate`, `hs_effect_new_evaluate`, `hs_damage_new_evaluate`, `numeric_countdown_timer_set_evaluate`, `unit_scripting_set_emotion_animation_evaluate`, `unit_scripting_set_seat_evaluate`, `device_group_change_only_once_more_set_evaluate`, `unit_set_emotion_evaluate`, `unit_scripting_enter_vehicle_evaluate`, `hs_teleport_players_not_in_trigger_volume_evaluate`, `hs_object_set_shield_evaluate`, `hs_object_set_permutation_evaluate`, `hs_effect_new_from_object_marker_evaluate`, `hs_objects_can_see_object_evaluate`, `hs_objects_can_see_flag_evaluate`, `hs_sound_set_gain_evaluate`, `objects_scripting_set_scale_evaluate`, `objects_scripting_attach_evaluate`, `object_beautify_evaluate`, `scenery_animation_start_evaluate`, `scenery_animation_start_at_frame_evaluate`, `unit_scripting_set_maximum_vitality_evaluate`, `units_scripting_set_maximum_vitality_evaluate`, `unit_scripting_set_current_vitality_evaluate`, `units_scripting_set_current_vitality_evaluate`, `device_set_power_evaluate`, `device_set_desired_position_evaluate`, `device_set_actual_position_evaluate`, `device_group_set_desired_value_evaluate`, `device_group_set_actual_value_evaluate`, `ai_scripting_vehicle_enterable_distance_evaluate`, `ai_scripting_follow_distance_evaluate`, `player_effect_screen_fade_in_evaluate`, `player_effect_screen_fade_out_evaluate`, `cinematic_set_title_delayed_evaluate`, `scripted_sound_new_evaluate`, `scripted_looping_sound_start_evaluate`, `scripted_looping_sound_set_scale_evaluate`, `debug_sound_classes_set_wet_evaluate`, `sound_class_set_gain_evaluate`, `hud_unit_activate_nav_point_with_flag_evaluate`, `hud_unit_activate_nav_point_with_object_evaluate`, `hud_activate_team_nav_point_with_flag_evaluate`, `hud_activate_team_nav_point_with_object_evaluate`, `scripted_player_effect_set_translation_evaluate`, `scripted_player_effect_set_rotation_evaluate`, `scripted_player_effect_set_rumble_evaluate`, `scripted_player_effect_start_evaluate`, `rasterizer_model_ambient_reflection_tint_evaluate`, `rasterizer_script_screen_effect_set_value_evaluate`, `rasterizer_screen_effect_set_convolution_evaluate`, `rasterizer_screen_effect_set_filter_evaluate`, `rasterizer_screen_effect_set_filter_desaturation_tint_evaluate`, `rasterizer_screen_effect_set_video_evaluate`, `hs_object_list_get_element_evaluate`, `hs_sound_get_gain_evaluate`, `unit_scripting_get_health_evaluate`, `unit_scripting_get_shield_evaluate`, `device_get_power_evaluate`, `device_get_position_evaluate`, `device_group_get_value_evaluate`, `ai_scripting_living_fraction_evaluate`, `ai_scripting_strength_evaluate`, `ai_scripting_maneuver_evaluate`, `ai_scripting_maneuver_enable_evaluate`, `ai_scripting_migrate_evaluate`, `radiosity_hack_start_evaluate`, `radiosity_hack_save_evaluate`, `radiosity_hack_find_point_evaluate`, `ai_scripting_migrate_and_speak_evaluate`, `ai_scripting_migrate_by_unit_evaluate`, `ai_scripting_allegiance_evaluate`, `ai_scripting_allegiance_remove_evaluate`, `ai_scripting_go_to_vehicle_evaluate`, `ai_scripting_go_to_vehicle_override_evaluate`, `ai_scripting_exit_vehicle_evaluate`, `ai_scripting_braindead_evaluate`, `ai_scripting_braindead_by_unit_evaluate`, `ai_scripting_ignore_evaluate`, `ai_scripting_prefer_target_evaluate`, `ai_scripting_teleport_starting_location_evaluate`, `ai_scripting_teleport_starting_location_if_unsupported_evaluate`, `ai_scripting_renew_evaluate`, `ai_scripting_try_to_fight_nothing_evaluate`, `ai_scripting_try_to_fight_evaluate`, `ai_scripting_try_to_fight_player_evaluate`, `ai_scripting_command_list_evaluate`, `ai_scripting_command_list_by_unit_evaluate`, `ai_scripting_command_list_advance_evaluate`, `ai_scripting_command_list_advance_by_unit_evaluate`, `ai_scripting_force_active_evaluate`, `ai_scripting_force_active_by_unit_evaluate`, `ai_scripting_set_return_state_evaluate`, `ai_scripting_set_current_state_evaluate`, `ai_scripting_playfight_evaluate`, `ai_scripting_vehicle_encounter_evaluate`, `ai_scripting_vehicle_enterable_team_evaluate`, `ai_scripting_vehicle_enterable_actor_type_evaluate`, `ai_scripting_vehicle_enterable_actors_evaluate`, `ai_scripting_vehicle_enterable_disable_evaluate`, `ai_scripting_look_at_object_evaluate`, `ai_scripting_stop_looking_evaluate`, `ai_scripting_automatic_migration_target_evaluate`, `ai_scripting_follow_target_disable_evaluate`, `ai_scripting_follow_target_players_evaluate`, `ai_scripting_follow_target_unit_evaluate`, `ai_scripting_follow_target_ai_evaluate`, `ai_scripting_conversation_stop_evaluate`, `ai_scripting_conversation_advance_evaluate`, `ai_scripting_link_activation_evaluate`, `ai_scripting_berserk_evaluate`, `ai_scripting_set_team_evaluate`, `ai_scripting_allow_charge_evaluate`, `ai_scripting_allow_dormant_evaluate`, `director_script_camera_evaluate`, `scripted_camera_set_absolute_evaluate`, `scripted_camera_set_evaluate`, `scripted_camera_set_animation_evaluate`, `scripted_camera_set_first_person_evaluate`, `scripted_camera_set_dead_evaluate`, `game_time_set_speed_evaluate`, `game_set_game_variant_from_name_evaluate`, `player_input_enable_evaluate`, `player_control_action_test_jump_evaluate`, `player_control_action_test_primary_trigger_evaluate`, `player_control_action_test_grenade_trigger_evaluate`, `player_control_action_test_zoom_evaluate`, `player_control_action_test_action_evaluate`, `player_control_action_test_accept_evaluate`, `player_control_action_test_back_evaluate`, `player_control_action_test_look_relative_up_evaluate`, `player_control_action_test_look_relative_down_evaluate`, `player_control_action_test_look_relative_left_evaluate`, `player_control_action_test_look_relative_right_evaluate`, `player_control_action_test_look_relative_all_directions_evaluate`, `player_control_action_test_move_relative_all_directions_evaluate`, `main_set_difficulty_evaluate`, `main_crash_evaluate`, `debug_dump_memory_for_file_evaluate`, `profile_dump_to_file_evaluate`, `profile_sections_activate_evaluate`, `profile_sections_deactivate_evaluate`, `profile_graph_toggle_evaluate`, `debug_pvs_evaluate`, `ai_debug_teleport_to_evaluate`, `ai_debug_speak_evaluate`, `ai_debug_speak_list_evaluate`, `cinematic_show_letterbox_evaluate`, `cinematic_set_title_evaluate`, `cinematic_suppress_bsp_object_creation_evaluate`, `main_load_core_name_evaluate`, `main_load_core_name_at_startup_evaluate`, `main_save_core_name_evaluate`, `main_skip_evaluate`, `scripted_sound_stop_evaluate`, `scripted_foley_predict_evaluate`, `scripted_looping_sound_stop_evaluate`, `scripted_looping_sound_set_alternate_evaluate`, `sound_enable_evaluate`, `vehicle_hover_evaluate`, `scripted_hud_set_flashing_state_evaluate`, `hud_unit_deactivate_nav_point_with_flag_evaluate`, `hud_unit_deactivate_nav_point_with_object_evaluate`, `hud_deactivate_team_nav_point_with_flag_evaluate`, `hud_deactivate_team_nav_point_with_object_evaluate`, `errors_overflow_suppression_enable_evaluate`, `scripted_player_effect_stop_evaluate`, `scripted_hud_show_health_evaluate`, `scripted_hud_blink_health_evaluate`, `scripted_hud_show_shield_evaluate`, `scripted_hud_blink_shield_evaluate`, `scripted_hud_show_motion_sensor_evaluate`, `scripted_hud_blink_motion_sensor_evaluate`, `scripted_hud_show_crosshair_evaluate`, `scripted_hud_set_state_message_evaluate`, `scripted_hud_set_objective_evaluate`, `scripted_hud_set_timer_time_evaluate`, `scripted_hud_set_timer_warning_cutoff_evaluate`, `scripted_hud_set_timer_position_evaluate`, `scripted_hud_show_timer_evaluate`, `scripted_hud_pause_timer_evaluate`, `hs_dispose`, `hs_update`, `hs_node_gc`, `hs_function_get`, `hs_find_script_by_name`, `hs_find_tag_reference_by_index`, `hs_global_external_get`, `hs_global_get_type`, `hs_global_get_name`, `hs_find_global_by_name`, `hs_find_function_by_name`, `scripted_hud_time_code_show_evaluate`, `scripted_hud_time_code_start_evaluate`, `rasterizer_screen_effect_start_evaluate`, `rasterizer_set_near_clip_distance_evaluate`, `player0_look_invert_pitch_evaluate`, `player0_look_pitch_is_inverted_evaluate`, `player0_joystick_set_is_normal_evaluate`, `ui_widget_debug_show_path_evaluate`, `display_scenario_help_evaluate`, `xbox_set_machine_name_evaluate`, `hs_help_evaluate`, `hs_not_evaluate`, `scenario_trigger_volume_test_object_evaluate`, `hs_trigger_volume_test_objects_any_evaluate`, `hs_trigger_volume_test_objects_all_evaluate`, `recorded_animation_play_evaluate`, `recorded_animation_play_and_delete_evaluate`, `recorded_animation_play_and_hover_evaluate`, `lights_enable_evaluate`, `unit_start_user_animation_evaluate`, `unit_scripting_start_user_animation_list_evaluate`, `unit_custom_animation_at_frame_evaluate`, `unit_is_playing_custom_animation_evaluate`, `hs_tokens_enumerate`, `unit_scripting_vehicle_test_seat_list_evaluate`, `unit_scripting_vehicle_test_seat_evaluate`, `unit_scripting_has_weapon_evaluate`, `unit_scripting_has_weapon_readied_evaluate`, `unit_solo_player_integrated_night_vision_is_active_evaluate`, `unit_get_current_flashlight_state_evaluate`, `ai_scripting_is_attacking_evaluate`, `hs_evaluate_by_name`, `ai_scripting_conversation_evaluate`, `ai_scripting_allegiance_broken_evaluate`, `scripted_player_control_set_camera_control_evaluate`, `hs_scenario_postprocess`, `hs_initialize_for_new_map`, `hs_dispose_from_old_map`, `hs_help`, `hs_doc`, `scripted_show_hud_evaluate`, `scripted_show_hud_help_text_evaluate`, `hs_initialize`, `hs_compile_and_evaluate`, `hs_recompile_evaluate`, `random_range_evaluate`, `real_random_range_evaluate`, `hs_compile_and_evaluate_command`
- hs/hs_compile.c: `hs_parse_cond_recursive`, `hs_get_parameter_indices`, `hs_compile_initialize`, `hs_compile_add_source`, `hs_verify_source_offset`, `hs_find_string_constant`, `character_in_list`, `hs_parse_variable`, `hs_parse_call_predicate`, `hs_parse_boolean`, `hs_parse_real`, `hs_parse_integer`, `hs_parse_string`, `hs_parse_script`, `hs_parse_tag_reference`, `hs_parse_enum`, `hs_parse_scenario_datum`, `hs_parse_trigger_volume`, `hs_parse_cutscene_flag`, `hs_parse_cutscene_camera_point`, `hs_parse_cutscene_title`, `hs_parse_cutscene_recording`, `hs_parse_device_group`, `hs_parse_ai`, `hs_parse_ai_command_list`, `hs_parse_starting_profile`, `hs_parse_conversation`, `hs_parse_object_name`, `hs_parse_object`, `hs_parse_navpoint`, `hs_parse_hud_message`, `hs_parse_object_list`, `string_list_find`, `hs_concatenate_string_constant`, `hs_add_global`, `hs_add_script`, `hs_tokenize_primitive`, `hs_parse_primitive`, `hs_parse_nonprimitive`, `hs_postprocess_passthrough_arguments`, `hs_compile_postprocess`, `hs_concatenate_expression`, `hs_tokenize`, `hs_tokenize_nonprimitive`, `hs_parse`, `hs_macro_function_parse`, `hs_parse_begin`, `hs_parse_if`, `hs_parse_cond`, `hs_parse_set`, `hs_parse_logical`, `hs_parse_arithmetic`, `hs_parse_equality`, `hs_parse_inequality`, `hs_parse_sleep`, `hs_parse_sleep_until`, `hs_parse_wake`, `hs_parse_inspect`, `hs_parse_object_cast_up`, `hs_compile_finish`, `hs_compile_dispose`, `skip_whitespace`
- hs/hs_library_external.c: `hs_trigger_volume_test_objects`, `hs_unit_can_see_object`, `hs_unit_can_see_flag`, `object_is_or_contains_player`, `hs_object_destroy_by_name`, `hs_object_iterate_names_containing`, `hs_object_list_get_element`, `hs_object_set_shield`, `hs_object_set_permutation`, `hs_objects_predict`, `hs_objects_delete_by_definition`, `hs_effect_new`, `hs_effect_new_from_object_marker`, `hs_damage_new`, `hs_damage_object`, `hs_sound_get_gain_reference`, `hs_sound_get_gain`, `hs_sound_set_gain`, `hs_trigger_volume_test_objects_all`, `hs_trigger_volume_test_objects_any`, `hs_object_orient`, `hs_teleport_players_not_in_trigger_volume`
- hs/hs_runtime.c: `hs_syntax_nth`, `hs_inspect_boolean`, `hs_inspect_real`, `hs_inspect_short_integer`, `hs_inspect_long_integer`, `hs_inspect_string`, `hs_inspect_enum`, `hs_runtime_initialize`, `hs_runtime_dispose_from_old_map`, `hs_runtime_dispose`, `expression_get_function_name`, `hs_thread_new`, `hs_thread_delete`, `hs_thread_format`, `hs_stack_push`, `hs_stack_pop`, `hs_stack_allocate`, `hs_find_thread_by_script`, `hs_find_thread_by_name`, `hs_global_reconcile_read`, `hs_global_reconcile_write`, `script_error`, `hs_runtime_get_executing_thread_name`, `hs_wake_by_name`, `hs_evaluate_wake`, `hs_script_evaluate`, `hs_macro_function_evaluate`, `hs_evaluate_object_cast_up`, `hs_runtime_waiting_on_call`
- hs/object_lists.c: `reference_list_new`, `reference_list_get_next_datum_index`, `reference_list_add`, `reference_list_delete`, `object_lists_initialize`, `object_lists_dispose`, `object_lists_initialize_for_new_map`, `object_lists_dispose_from_old_map`, `object_list_new`, `object_list_delete`, `object_list_add`, `object_list_get_next`, `object_list_add_reference`, `object_list_remove_reference`, `object_list_gc`, `object_list_count`, `object_list_get_first`

## `input`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `input_abstraction_dispose` | input/input_abstraction.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_abstraction_reset_controller_detection_timer` | input/input_abstraction.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_abstraction_get_local_player_preferences` | input/input_abstraction.c | Replaced |  | through its caller `draw_string_and_hack_in_icons` |
| `input_abstraction_update_local_player_preferences` | input/input_abstraction.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_abstraction_get_input_state` | input/input_abstraction.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_abstraction_update_device_changes` | input/input_abstraction.c | Replaced |  | through its caller `input_get_device_states` |
| `keyboard_hold_ticks` | input/input_abstraction.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `keyboard_controls_update` | input/input_abstraction.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_abstraction_initialize` | input/input_abstraction.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_abstraction_update` | input/input_abstraction.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `fix_dead_zone` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `update_ticks` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `update_hold_ticks` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_dispose` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_activate` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_deactivate` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_flush` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_key_is_down` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_get_key` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_get_mouse_state` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_suppress` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_mouse_button_is_down` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_has_gamepad` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_get_gamepad_state` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_set_gamepad_rumbler_state` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_vertical_blank_interrupt` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `acquire_input_mutex` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `release_input_mutex` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_update_analog_button_state` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_get_device_states` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_update_keyboard_devices` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_get_raw_data_string` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_update` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_frame_begin` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_frame_end` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_flush_rumble` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_initialize` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `HATCleanup` | input/HaloAutoTest.c | Replaced |  | through its caller `input_dispose` |
| `HATReadMain` | input/HaloAutoTest.c | Replaced |  | through its caller `HATInit` |
| `HATRawRead` | input/HaloAutoTest.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `HATRawLoopRead` | input/HaloAutoTest.c | Replaced |  | through its caller `HATRun` |
| `HATRawWrite` | input/HaloAutoTest.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `HATInit` | input/HaloAutoTest.c | Replaced |  | through its caller `input_initialize` |
| `HATRun` | input/HaloAutoTest.c | Replaced |  | through its caller `input_update` |
| `set_default_game_input_preferences` | input/input_abstraction.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_keyboard_thread` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |

1 Missing (1 reached from `main_loop`), by file:

- input/input_abstraction.c: `local_player_is_piloting_aircraft`

## `interface`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `scripted_show_hud` | interface/hud.c | Matched | campaign/src/game.rs | campaign/tests/natives.rs (measure) |
| `scripted_show_hud_help_text` | interface/hud.c | Matched | campaign/src/game.rs | campaign/tests/natives.rs (measure) |
| `hud_show_action_response` | interface/hud.c | Matched | ffi/src/hud_text.rs:52 | ffi/tests/hud_text.rs (measure) |
| `hud_draw_screen` | interface/hud.c | Partial | ffi/src/hud.rs | ffi/tests/hud.rs (measure); leaves out the damage indicators, nav points, player names, timer, weapon overlays and messages: the engine draws the unit's and weapon's interfaces, the grenades and the motion sensor |
| `hud_globals_get_scale` | interface/hud_draw.c | Matched | ffi/src/hud.rs | ffi/tests/hud.rs (measure) |
| `hud_retrieve_bitmap_and_bounding_rect` | interface/hud_draw.c | Matched | ffi/src/hud/sprite_reference.rs:4 | ffi/src/hud/sprite_reference.rs (trace) |
| `fast_ftol_C` | interface/hud_draw.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `hud_calculate_point` | interface/hud_draw.c | Matched | ffi/src/hud.rs:168 | ffi/tests/hud.rs (measure) |
| `get_flash_color` | interface/hud_draw.c | Matched | ffi/src/hud.rs:180 | ffi/tests/hud.rs (measure) |
| `hud_draw_bitmap_direct` | interface/hud_draw.c | Matched | ffi/src/hud.rs | ffi/tests/hud.rs (measure) |
| `hud_draw_meter` | interface/hud_draw.c | Matched | ffi/src/hud.rs:316 | ffi/tests/hud.rs (measure) |
| `hud_draw_numbers` | interface/hud_draw.c | Matched | ffi/src/hud.rs:357 | ffi/tests/hud.rs (measure) |
| `hud_draw_bitmap` | interface/hud_draw.c | Matched | ffi/src/hud.rs | ffi/tests/hud.rs (measure) |
| `hud_draw_static_element` | interface/hud_draw.c | Matched | ffi/src/hud.rs:298 | ffi/tests/hud.rs (measure) |
| `render_state_text` | interface/hud_messaging.c | Matched | ffi/src/hud_text.rs | ffi/tests/hud_text.rs (measure) |
| `render_state_bitmap` | interface/hud_messaging.c | Matched | ffi/src/hud.rs:257 | ffi/tests/hud.rs (measure) |
| `scripted_hud_set_objective` | interface/hud_messaging.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `scripted_hud_set_timer_time` | interface/hud_messaging.c | Matched | campaign/src/game.rs | campaign/tests/natives.rs (measure) |
| `scripted_hud_set_timer_warning_cutoff` | interface/hud_messaging.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `scripted_hud_set_timer_position` | interface/hud_messaging.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `scripted_hud_show_timer` | interface/hud_messaging.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `scripted_hud_pause_timer` | interface/hud_messaging.c | Matched | campaign/src/game.rs | campaign/tests/natives.rs (measure) |
| `scripted_hud_get_timer_ticks` | interface/hud_messaging.c | Matched | campaign/src/game.rs | campaign/tests/natives.rs (measure) |
| `hud_set_state_message` | interface/hud_messaging.c | Matched | ffi/src/hud_text.rs | ffi/tests/hud_text.rs (measure) |
| `hud_set_state_message_icon` | interface/hud_messaging.c | Matched | ffi/src/hud_text.rs | ffi/tests/hud_text.rs (measure) |
| `hud_set_state_message_text` | interface/hud_messaging.c | Matched | ffi/src/hud_text.rs | ffi/tests/hud_text.rs (measure) |
| `hud_messaging_update` | interface/hud_messaging.c | Partial | ffi/src/hud_text.rs:53 | ffi/tests/hud_text.rs (measure); leaves out the objective and help messages, the list of messages and their ageing, split screen |
| `scripted_hud_show_health` | interface/hud_unit.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `scripted_hud_blink_health` | interface/hud_unit.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `scripted_hud_show_shield` | interface/hud_unit.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `scripted_hud_blink_shield` | interface/hud_unit.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `scripted_hud_show_motion_sensor` | interface/hud_unit.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `scripted_hud_blink_motion_sensor` | interface/hud_unit.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `hud_render_unit_interface` | interface/hud_unit.c | Matched | ffi/src/hud.rs:82 | ffi/tests/hud.rs (measure) |
| `scripted_hud_show_crosshair` | interface/hud_weapon.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `get_crosshair_state` | interface/hud_weapon.c | Matched | ffi/src/hud.rs | ffi/tests/hud.rs (measure) |
| `render_grenade_hud` | interface/hud_weapon.c | Matched | ffi/src/hud.rs:623 | ffi/tests/hud.rs (measure) |
| `crosshairs_draw` | interface/hud_weapon.c | Matched | ffi/src/hud.rs | ffi/tests/hud.rs (measure) |
| `render_weapon_hud` | interface/hud_weapon.c | Matched | ffi/src/hud.rs | ffi/tests/hud.rs (measure) |
| `hud_render_weapon_interface` | interface/hud_weapon.c | Matched | ffi/src/hud.rs | ffi/tests/hud.rs (measure) |
| `blip_type_get` | interface/motion_sensor.c | Matched | sim/src/hud.rs | sim/src/hud.rs (measure) |
| `motion_sensor_blip_set_type_and_size` | interface/motion_sensor.c | Matched | sim/src/hud.rs | sim/src/hud.rs (measure) |
| `blip_size_get` | interface/motion_sensor.c | Matched | sim/src/hud.rs:67 | sim/src/hud.rs (measure) |
| `should_track_object` | interface/motion_sensor.c | Matched | sim/src/hud.rs | sim/src/hud.rs (measure) |
| `should_draw_object` | interface/motion_sensor.c | Matched | sim/src/hud.rs | sim/src/hud.rs (measure) |
| `render_blip` | interface/motion_sensor.c | Matched | sim/src/hud.rs:171 | sim/src/hud.rs (measure) |
| `motion_sensor_update` | interface/motion_sensor.c | Matched | sim/src/hud.rs:122 | sim/src/hud.rs (measure) |
| `update_motion_sensor` | interface/motion_sensor.c | Matched | sim/src/hud.rs | sim/src/hud.rs (measure) |
| `render_motion_sensor` | interface/motion_sensor.c | Matched | sim/src/hud.rs | sim/src/hud.rs (measure) |
| `motion_sensor_tick` | interface/motion_sensor.c | Matched | sim/src/hud.rs | sim/src/hud.rs (measure) |
| `first_person_weapon_start_interpolation` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:543 | `tools/engine_first_person_reference.py` |
| `first_person_weapon_set_state` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:197 | `tools/engine_first_person_reference.py` |
| `first_person_weapon_animation_fits` | interface/first_person_weapons.c | Matched | sim/src/first_person.rs:88 | sim/tests/first_person.rs (measure) |
| `first_person_weapon_build_node_matrices` | interface/first_person_weapons.c | Matched | sim/src/first_person.rs:160 | sim/tests/first_person.rs (measure) |
| `first_person_weapon_message` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:633 | `tools/engine_first_person_reference.py` |
| `first_person_weapon_switch_weapons` | interface/first_person_weapons.c | Matched | sim/src/first_person_states.rs:616 | sim/src/first_person_states.rs (measure) |
| `first_person_weapon_next_state` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:686 | `tools/engine_first_person_reference.py` |
| `first_person_weapon_update` | interface/first_person_weapons.c | Matched | sim/src/first_person_states.rs:751 | sim/src/first_person_states.rs (measure) |
| `model_build_remapping_table_for_animation_graph` | interface/first_person_weapons.c | Matched | sim/src/first_person.rs:75 | sim/tests/first_person.rs (measure) |
| `first_person_weapon_state_from_weapon_message` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:315 | `tools/engine_first_person_reference.py` |
| `first_person_animation_type_from_weapon_state` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:166 | `tools/engine_first_person_reference.py` |
| `first_person_weapons_update` | interface/first_person_weapons.c | Partial | sim/src/lib.rs:1312 | sim/tests/first_person.rs (measure); leaves out the first-person animations' blending, the weapon's sway |
| `render_debug_profile_stall_tick` | interface/interface.c | Replaced | dev/src/profile.rs | the editor and tools |
| `render_debug_profile` | interface/interface.c | Replaced | dev/src/profile.rs | the editor and tools |
| `interface_draw_bitmap_modulated` | interface/interface.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `interface_draw_bitmap_modulated_p32` | interface/interface.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `overhead_map_post_rasterize` | interface/overhead_map.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `player0_look_pitch_is_inverted` | interface/player_ui.c | Partial | campaign/src/game.rs | campaign/tests/natives.rs (measure); leaves out the player's profile: it answers a constant, the mouse's own setting standing in |
| `player0_joystick_set_is_normal` | interface/player_ui.c | Partial | campaign/src/game.rs | campaign/tests/natives.rs (measure); leaves out the player's profile: it answers a constant, the mouse's own setting standing in |
| `player0_look_invert_pitch` | interface/player_ui.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `code_000d0ec0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d0f10` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d0f20` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d0f40` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1020` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1030` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d10e0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d10f0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `tgaLoad` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `progress_bar_is_stuff_ready` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1160` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1380` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1390` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d14c0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1660` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1670` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1680` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d16a0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d16e0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d16f0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `gen_cloud_coord` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `gen_mask_coord` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `progress_bar_enable` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1ba0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `IDirect3DTexture8_Release` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1bc0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1bd0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `code_000d1bf0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `D3DXMatrixIdentity` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `progress_bar_render` | interface/progress_bar.c | Partial | ui/src/loading.rs:211 | ui/tests/loading.rs (measure); leaves out the original's drawing at its low resolution (the light is drawn at the screen's) |
| `code_000d23b0` | interface/progress_bar.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `terminal_initialize` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_dispose` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_remove_line` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_clear` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_gets_active` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_gets_begin` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_gets_end` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_update_input` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_update_output` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_draw` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_new_line` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_update` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `terminal_printf` | interface/terminal.c | Replaced | app/Sources/OpenCEEngineKit/Shell/ShellOverlay.swift | the editor and tools |
| `event_controller_index_compatible_with_widget` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widget_debug_show_path` | interface/ui_widget.c | Replaced | dev/src/draw.rs | the editor and tools |
| `widget_instance_count_children` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_get_nth_child` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widgets_active` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widgets_active_for_local_player` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `draw_bitmap_in_rect` | interface/ui_widget.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `widget_instance_get_topmost_parent` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_get_child_index_from_parent` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `pop_widget` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_start_main_menu_music` | interface/ui_widget.c | Matched | ffi/src/ui_host.rs:62 | ffi/tests/menu_music.rs (measure) |
| `dispose_widget_stack` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widget_delete_children_recursive` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widget_launch_widget` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_find_by_tag_index_recursive` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_give_focus_directly` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_give_focus_by_tag` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widget_add_child` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `push_widget` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_can_receive_events` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_set_focused_child_by_index` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_go_back_to_previous` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `event_handler_dispatch` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widget_load_children_recursive` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_initialize` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `remap_sticks_for_local_player` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `should_flip_sticks_for_local_player` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `search_and_replace` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_render_text_box` | interface/ui_widget.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `widget_instance_render_spinner_list` | interface/ui_widget.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `get_ui_rgb_white` | interface/ui_widget.c | Matched | ui/src/draw.rs:16 | ui/tests/draw.rs (measure) |
| `get_ui_argb_white` | interface/ui_widget.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `ui_debug_targets_enabled` | interface/ui_widget.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ui_widget_delete` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widgets_close_all` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widgets_close_all_for_local_player` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widgets_delete_history` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_widgets_pop_stack` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_play_audio_feedback_sound` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_debug_draw_outline` | interface/ui_widget.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ui_debug_draw_mark` | interface/ui_widget.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ui_debug_draw_targets` | interface/ui_widget.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ui_debug_set_mark` | interface/ui_widget.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ui_mouse_press` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_child_index` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_list_shows_several` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `widget_event_function_list_widget_goto_next_item` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_event_function_list_widget_goto_previous_item` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `ui_mouse_key_button` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_widget_is_item` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_note_list_step` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `render_ui_widgets` | interface/ui_widget.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `ui_mouse_note_slot_band` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_note_target` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_widget_load_by_name_or_tag` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `display_scenario_help` | interface/ui_widget.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `ui_mouse_merge_setting_rows` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_widen_values` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `process_ui_widgets` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `begin_music_fade_out` | interface/ui_widget_event_handler_functions.c | Matched | ffi/src/ui_host.rs:651 | ffi/tests/menu_music.rs (measure) |
| `main_menu_initialize` | interface/ui_widget_event_handler_functions.c | Matched | ffi/src/ui_host.rs:651 | ffi/tests/menu_music.rs (measure) |
| `virtual_keyboard_initialize` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_dispose` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_launch` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_active` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_last_exit_saved_text` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_tab_left` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_tab_right` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_tab_up` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_tab_down` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_cancel` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_get_character` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_get_current_character` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_render_internal` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_free_space_in_text_buffer` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_backspace` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_close` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_render` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_select` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_process_internal` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_process` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `get_sprite_clip_rect` | interface/hud_draw.c | Matched | ffi/src/hud/sprite_reference.rs:4 | ffi/src/hud/sprite_reference.rs (trace) |
| `hud_calculate_bitmap_bounds` | interface/hud_draw.c | Matched | ffi/src/hud.rs:206 | ffi/tests/hud.rs (measure) |
| `widget_instance_can_handle_events` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_get_tail_child_widget` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_find_by_tag_index` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_get_cumulative_alpha_modifier` | interface/ui_widget.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `ui_mouse_label_text_right` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_fit_button_targets` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_target_at` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_widget_has_focus` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_give_focus` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_list_directions` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_step_list_to_slot` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_wheel_widget` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_wheel_skips_child` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_wheel_room` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_menu` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_mouse_menus_active` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_debug_log_click` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_debug_log_keyboard_tap` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `ui_widgets_process_mouse` | interface/ui_widget.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `widget_instance_render_recursive` | interface/ui_widget.c | Matched | ui/src/draw.rs:3 | ui/tests/draw.rs (measure) |
| `spinner_list_update` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_tab_to_next_valid_widget` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/pointer.rs (measure) |
| `widget_instance_tab_to_previous_valid_widget` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/pointer.rs (measure) |
| `widget_takes_any_controller` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_takes_events_of_controller` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `widget_instance_process_one_event_recursive` | interface/ui_widget.c | Matched | ui/src/lib.rs | ui/tests/tree.rs (measure) |
| `virtual_keyboard_target_rectangles` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |
| `virtual_keyboard_click` | interface/virtual_keyboard.c | Replaced | ui/src/pointer.rs | keyboard and mouse only |

452 Missing (435 reached from `main_loop`), by file:

- interface/attract_mode.c: `attract_mode_should_start`, `attract_mode_reset_timer`, `attract_mode_get_localized_movie_path`, `attract_mode_start`, `game_end_credits_start`
- interface/event_manager.c: `event_manager_initialize`, `event_manager_dispose`, `event_manager_flush`, `event_manager_suppress`, `get_next_event`, `event_manager_time_of_last_event`, `event_manager_post_button`, `event_manager_update`, `queue_event`
- interface/first_person_weapons.c: `animation_update_render_only`, `first_person_weapons_initialize`, `first_person_weapons_dispose`, `first_person_weapons_initialize_for_new_map`, `first_person_weapons_dispose_from_old_map`, `first_person_weapon_set_visibility`, `first_person_weapon_predict`, `first_person_weapon_new_unit`, `first_person_weapon_draw`, `first_person_weapon_get_local_index`, `first_person_weapon_get_marker_by_name`, `first_person_weapon_center_flashlight`, `first_person_weapon_adjust_light`, `first_person_weapon_get_node_matrix`, `first_person_weapon_render_update`, `first_person_weapon_get_marker_by_name_render`, `weapon_play_first_person_weapon_sound`, `model_remap_node_matrices_to_match_animation_graph`, `first_person_weapon_message_from_unit`, `first_person_weapon_message_from_weapon`, `first_person_weapon_get`, `first_person_weapon_index_from_weapon_index`, `first_person_weapon_index_from_unit_index`
- interface/hud.c: `weapon_state_is_depleted`, `hud_initialize`, `hud_dispose`, `hud_initialize_for_new_map`, `hud_dispose_from_old_map`, `hud_update`, `get_object_icon_text_index`, `hud_get_item_string`, `hud_picked_up_grenade`, `hud_picked_up_ammunition`, `hud_picked_up_weapon`, `hud_picked_up_powerup`, `temporary_hud_draw_reticle`, `hud_load`, `hud_autosave`, `hud_draw_friendly_indicator`, `hud_draw_players`, `hud_player_names_setting`, `hud_player_name_scale`, `hud_player_name_in_sight`, `hud_player_name_enemy_range`, `hud_draw_player_name`, `hud_draw_player_names`, `temporary_hud_draw`
- interface/hud_draw.c: `get_return_eip`, `check_stack_buffer`, `hud_draw_multitexture_overlay_get_current_weapon_definition`, `hud_multitexture_overlay_blend_function`, `hud_draw_multitexture_overlay`, `real_argb_color_to_pixel32`, `real_rgb_color_to_pixel32`, `real_alpha_intensity_to_pixel32`, `get_flash_duration`, `hud_multitexture_overlays_follow_zoom`, `hud_number_shows_only_when_zoomed`, `hud_zoomed_layout_begin`, `hud_draw_weapon_overlays`, `hud_zoomed_layout_end`, `hud_draw_bitmap_with_meter`
- interface/hud_messaging.c: `hud_messaging_initialize`, `hud_messaging_initialize_for_new_map`, `hud_messaging_dispose_from_old_map`, `hud_messaging_dispose`, `scripted_hud_set_state_message`, `scripted_hud_set_flashing_state`, `scripted_hud_restart_flashing`, `scripted_hud_time_code_show`, `scripted_hud_time_code_start`, `scripted_hud_time_code_reset`, `hud_render_timer`, `hud_enable_custom_state_message`, `hud_set_state_text`, `hud_messaging_get_objective`, `find_free_hud_message`, `compare_messages`, `scripted_hud_messages_clear`, `hud_get_font_index`, `hud_get_text_color`, `hud_messaging_globals_update`, `hud_print_message`, `hud_add_item_message`, `hud_broadcast_team_message`
- interface/hud_nav_points.c: `find_nav_point`, `get_nav_point_datum`, `hud_initialize_nav_points`, `hud_initialize_nav_points_for_new_map`, `hud_dispose_nav_points_from_old_map`, `hud_dispose_nav_points`, `hud_activate_nav_point`, `hud_activate_nav_point_with_game_engine_flag`, `hud_activate_nav_point_with_flag`, `hud_activate_nav_point_with_object`, `hud_activate_team_nav_point`, `hud_activate_team_nav_point_with_game_engine_flag`, `hud_activate_team_nav_point_with_flag`, `hud_activate_team_nav_point_with_object`, `hud_activate_global_nav_point`, `hud_activate_global_nav_point_with_game_engine_flag`, `hud_deactivate_nav_point`, `hud_deactivate_nav_point_with_game_engine_flag`, `hud_deactivate_nav_point_with_flag`, `hud_deactivate_nav_point_with_object`, `hud_deactivate_team_nav_point`, `hud_deactivate_team_nav_point_with_flag`, `hud_deactivate_team_nav_point_with_object`, `hud_unit_activate_nav_point_with_flag`, `hud_unit_activate_nav_point_with_object`, `hud_unit_deactivate_nav_point_with_flag`, `hud_unit_deactivate_nav_point_with_object`, `hud_get_nav_point_render_type`, `custom_render_nav_point`, `hud_render_nav_points`, `hud_update_nav_point_local_player`, `hud_update_nav_points`
- interface/hud_sounds.c: `hud_play_sound`
- interface/hud_unit.c: `unit_hud_outline_mapper_tick`, `unit_hud_shield_meter_mapper_tick`, `unit_hud_shield_meter_mapper_init`, `initialize_hud_state`, `get_hud_state`, `hud_initialize_unit_interface`, `hud_initialize_unit_interface_for_new_map`, `hud_dispose_unit_interface_from_old_map`, `hud_dispose_unit_interface`, `hud_play_unit_sounds`, `hud_fix_unit_data`, `hud_update_unit_local_player`, `hud_render_damage_indicators`, `hud_tick_shield`, `hud_update_unit`
- interface/hud_weapon.c: `hud_initialize_weapon_interface`, `hud_initialize_weapon_interface_for_new_map`, `hud_dispose_weapon_interface_from_old_map`, `hud_dispose_weapon_interface`, `get_hud_state`, `play_weapon_hud_sounds`, `strip_path_name`, `hud_update_weapon_local_player`, `hud_fix_weapon_data`, `hud_update_weapon`
- interface/interface.c: `interface_initialize`, `interface_dispose_from_old_map`, `interface_dispose`, `interface_get_tag_index`, `interface_get_real_argb_color`, `interface_set_bitmap_text_draw_mode`, `interface_get_weapon_hud_index`, `interface_draw_screen`, `profile_graph_toggle`, `interface_splitscreen_render`, `interface_initialize_for_new_map`, `interface_get_rgb_color`, `interface_draw_fullscreen_overlays`, `interface_draw_bitmap`
- interface/marketing_and_strategic_business_development.c: `xbox_demos_available`, `xbox_demos_launch`, `xbox_dashboard_launch`, `clean_up_for_image_launch`
- interface/motion_sensor.c: `tiny_point2d_set`, `tiny_point2d_get`, `get_motion_sensor_data`, `motion_sensor_initialize`, `motion_sensor_dispose`, `motion_sensor_initialize_for_new_map`, `motion_sensor_dispose_from_old_map`, `blip_begin`, `motion_sensor_draw_screen`
- interface/overhead_map.c: `overhead_map_initialize`, `overhead_map_initialize_for_new_map`, `overhead_map_dispose_from_old_map`
- interface/player_ui.c: `player_ui_dispose`, `player_ui_reset_single_player_local_player_controllers`, `player_ui_set_single_player_local_player_controller`, `player_ui_get_single_player_local_player_controller`, `player_ui_get_single_player_local_player_from_controller`, `player_ui_local_player_joined_multiplayer_game`, `player_ui_local_player_wants_to_play_multiplayer`, `player_ui_clear_multiplayer_autojoin_for_local_player`, `player_ui_autojoin_players_to_next_multiplayer_game`, `player_ui_clear_multiplayer_variant`, `player_ui_get_active_player_profile`, `player_ui_get_active_player_profile_index`, `player_ui_get_last_single_player_level_played`, `player_ui_set_game_variant`, `player_ui_game_variant_specified`, `player_ui_rumble_disabled`, `player_ui_autolevel_enabled`, `player_ui_get_path_to_local_player_profile_directory`, `player_ui_remember_player1_profile`, `player_ui_get_player1_last_used_profile_index`, `player_ui_fast_setup_network_server`, `player_ui_edit_profile_is_default_profile`, `player_ui_edit_profile_name_is_dirty`, `player_ui_prompt_user_to_rename_edit_profile`, `player_ui_get_edit_player_profile`, `player_ui_get_edit_playlist_profile`, `player_ui_edit_profile_is_dirty`, `player_ui_activate_all_solo_levels`, `hud_message_to_all`, `generate_default_player_profile`, `set_local_player_controls_from_player_profile`, `clear_profile_edit_data`, `reset_local_player_profile`, `player_ui_initialize`, `player_ui_clear_multiplayer_joins`, `player_ui_set_active_player_profile`, `player_ui_begin_editing_profile`, `player_ui_save_profile`, `player_ui_end_editing_profile`, `player_ui_get_game_variant_options`, `player_ui_saved_game_file_removed`
- interface/progress_bar.c: `progress_bar_load_loading_texture`, `tgaLoadHeader`, `tgaLoadImageData`, `progress_bar_initialize`, `progress_bar_dispose`, `progress_bar_begin`, `progress_bar_end`, `progress_bar_is_active`, `to_screen`, `generate_gravy_rect`, `progress_bar_create_noise_texture`, `draw_layer_int`, `draw_layer`, `draw_initial_layer`, `this_is_awful`, `do_convoluation_coords`, `draw_fade_layer`, `SetRenderStateSmart`, `SetTextureStageStateSmart`, `progress_bar_create_mask_texture`, `progress_bar_eachframe`, `progress_bar_make_stuff_ready`, `progress_bar_display`
- interface/ui_widget.c: `display_error_text_when_main_menu_loaded`, `pool_alloc`, `set_ui_plasma_effect_color`, `ui_widgets_initialize`, `ui_widgets_safe_to_load`, `ui_widgets_set_fade_value`, `ui_widget_realloc`, `widget_free`, `ui_widgets_inhibit_processing`, `pool_free`, `widget_instance_set_visibility_recursive`, `main_menu_active`, `main_menu_is_active`, `main_menu_screen_is_active`, `ui_set_next_level`, `ui_widget_load_progress_widget`, `filesystem_check_thread_is_active`, `display_error_when_main_menu_loaded`, `display_error_deferred`, `display_error_abort_to_dashboard_deferred`, `ui_stop_main_menu_music`, `ui_main_menu_music_active`, `ui_widgets_disable_pause_game`, `play_sound_tag`, `transition_to_game_in_progress`, `get_icon_type`, `filesystem_initialization_thread_proc`, `perform_filesystem_initialization`, `widget_instance_reload_recursive`, `ui_widget_reload_by_tag`, `widget_instance_text_box_is_focused`, `string_has_icons_to_draw`, `modulate_pixel32_by_real_alpha`, `main_screen_shell_begin_fade`, `draw_string_and_hack_in_icons`, `ui_widgets_dispose`, `render_ui_widgets_postgame`, `main_screen_shell_load`, `network_game_reset_to_pregame_ui`, `display_error`, `display_error_abort_to_dashboard`, `display_error_damaged_media`, `display_errors_deferred_until_cinematic_stop`, `compute_offset_coordinate`, `spinner_string_list_extra_count`, `ui_widget_spinner_own_item_count`, `ui_widget_spinner_extra_description`, `spinner_string_list_get_string`, `widget_instance_render_column_list`, `widget_instance_update_animation_parameters`, `column_list_update`, `ui_check_for_pause_game`
- interface/ui_widget_event_handler_functions.c: `ui_widget_event_handler_function_invoke`, `widget_event_function_null`, `reset_last_player1_profile_index`, `new_campaign_decision`, `create_and_begin_editing_new_gametype_profile`, `network_game_join_game_from_server_list`, `pause_game_restart_at_checkpoint`, `pause_game_restart_level`, `pause_game_quit_to_main_menu`, `coop_game_initialize`, `multiplayer_type_menu_initialize`, `cancel_profile_delete`, `run_xdemos`, `single_player_reset_controller_choices`, `exit_to_xbox_dashboard`, `clear_multiplayer_player_joins`, `network_server_list_dispose`, `network_game_cancel`, `multiplayer_pick_quick_start_play_stage`, `multiplayer_level_list_dispose`, `playlist_profile_end_editing`, `player_profile_end_editing`, `switch_from_main_menu_to_single_player`, `network_game_server_accept_connections`, `network_game_server_defer_game_start`, `network_game_server_allow_game_start`, `disable_widget_if_no_xdemos`, `pop_history_stack_once`, `solo_level_dispose_list`, `start_new_game`, `multiplayer_profiles_list_dispose`, `player_profiles_list_dispose`, `player_profile_color_picker_menu_dispose`, `network_game_server_list_initialize`, `delete_playlist_profile_final`, `player_wants_to_join_multiplayer_game`, `playlist_profile_change_name`, `player_profile_change_name`, `network_game_remove_local_player`, `delete_player_profile_final`, `single_player_set_player1_controller_choice`, `netgame_unjoin_player`, `close_calling_widget_if_not_editing_profile`, `new_campaign_if_no_custom_player_profiles_exist`, `difficulty_set`, `display_error_if_no_network_connection`, `split_screen_game_initialize`, `single_player_set_player2_controller_choice`, `player_profile_save_changes`, `start_network_game_if_no_advertised_servers`, `delete_player_profile_request`, `player_profile_color_picker_select_color`, `playlist_profile_set_game_engine`, `multiplayer_game_swap_teams`, `network_game_start_faster`, `network_game_start_slower`, `create_and_begin_editing_new_player_profile`, `multiplayer_level_list_initialize`, `playlist_profile_begin_editing`, `player_profile_begin_editing`, `delete_playlist_profile_request`, `player_profile_color_picker_menu_initialize`, `difficulty_menu_initialize`, `playlist_profile_initialize_game_engine`, `playlist_profile_save_changes`, `multiplayer_profiles_list_initialize`, `ui_widget_function_denied`, `new_campaign_chosen`, `network_game_start_new_server`, `playlist_profile_initialize_name`, `playlist_profile_change_item_options`, `playlist_profile_change_indicator_options`, `playlist_profile_change_ctf_rules`, `playlist_profile_change_racing_rules`, `playlist_profile_change_koth_rules`, `playlist_profile_change_slayer_rules`, `player_profile_set_for_game_1wide`, `player_profile_initialize_controller_settings`, `solo_level_initialize_list_coop`, `player_profile_change_advanced_controller_settings`, `solo_level_set_next_map_name`, `netgame_join_player`, `player_profiles_list_initialize`, `player_profile_set_for_game_3wide`, `playlist_profile_initialize_indicator_options`, `playlist_profile_initialize_item_options`, `player_profile_initialize_advanced_controller_settings`, `player_profile_change_controller_settings`, `playlist_profile_initialize_racing_rules`, `playlist_profile_initialize_slayer_rules`, `playlist_profile_initialize_ctf_rules`, `playlist_profile_initialize_koth_rules`, `playlist_profile_initialize_oddball_rules`, `playlist_profile_initialize_player_options`, `playlist_profile_change_oddball_rules`, `playlist_profile_change_player_options`, `multiplayer_level_select`, `multiplayer_profile_set_for_game`, `solo_level_initialize_list_single_player`
- interface/ui_widget_game_data_input_functions.c: `ui_widget_game_data_function_invoke`, `widget_function_null`, `settings_menu_update_extended_description`, `playlist_settings_menu_update_extended_description`, `playlist_gametype_select_menu_update_extended_description`, `multiplayer_type_menu_update_extended_description`, `difficulty_select_menu_update_extended_description`, `server_list_menu_update`, `network_pregame_status_screen_update`, `splitscreen_pregame_status_screen_update`, `netgame_prejoin_players`, `set_textbox_to_build_number`, `solo_game_objective_text`, `main_menu_animation_fakery`, `get_active_player_profile_display_name`, `get_editable_player_profile_display_name`, `get_editable_playlist_profile_display_name`, `get_active_player_profile_color_index`, `player_profile_edit_select_menu_update_extended_description`, `game_options_menu_update_text_desc`, `game_options_menu_update_pic_desc`, `multiplayer_game_set_text_box_for_map_name`, `multiplayer_game_set_text_box_for_game_ruleset`, `multiplayer_game_set_text_box_for_teams_noteams`, `multiplayer_game_set_text_box_for_score_limit`, `multiplayer_game_set_text_box_for_score_limit_type`, `multiplayer_game_set_bitmap_for_map`, `multiplayer_game_set_bitmap_for_ruleset`, `multiplayer_game_set_text_box_for_number_of_players`, `multiplayer_edit_profile_set_ruleset_textbox_string_index`, `system_link_status_check`, `multiplayer_game_directions`, `teams_no_teams_mp_game_bitmap_update`, `warn_if_difficulty_will_nuke_saved_game`, `dim_if_no_system_link_cable`, `spinner_list_3wide_determine_displayed_item_indices`, `player_profile_update_cache_for_nwide_list`, `variant_profile_update_cache_for_nwide_list`, `mutliplayer_settings_select_list_update_displayed_items`, `player_profile_3wide_list_update`, `player_profile_1wide_list_update`, `solo_level_select_list_update_displayed_items`, `player_profile_color_picker_update`, `mp_level_select_list_update_displayed_items`, `list_indices_sort_proc`, `filter_invalid_list_indices`
- interface/ui_widget_group.c: `ui_load_tags_for_scenario`
- interface/ui_widget_text_search_and_replace_functions.c: `ui_widget_search_and_replace_invoke`, `widget_replace_function_null`, `widget_controller`

## `items`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `equipment_place` | items/equipment.c | Matched | import/src/level.rs:1046 | sim/tests/item_spawn_reference.rs (trace) |
| `equipment_handle_pickup` | items/equipment.c | Matched | sim/src/player.rs:1253 | sim/tests/weapons.rs (measure) |
| `verify_item_location` | items/items.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `item_adjust_for_angular_velocity_change` | items/items.c | Ported | sim/src/item_physics.rs:393 | `tools/engine_item_reference.py` |
| `item_accelerate` | items/items.c | Ported | sim/src/forces.rs:211 | `tools/engine_item_reference.py` |
| `item_align_to_normal_and_point` | items/items.c | Ported | sim/src/item_physics.rs:376 | `tools/engine_item_reference.py` |
| `item_update` | items/items.c | Ported | sim/src/game.rs:229 | `tools/engine_item_reference.py` |
| `projectile_calculate_deceleration_from_distances` | items/projectiles.c | Matched | sim/src/projectile.rs:737 | sim/tests/projectile_deceleration_reference.rs (trace) |
| `projectile_calculate_deceleration` | items/projectiles.c | Matched | sim/src/projectile.rs:3 | sim/tests/projectile_deceleration_reference.rs (trace) |
| `random_vector_in_cone3d` | items/projectiles.c | Matched | sim/src/random.rs | sim/tests/random_reference.rs (trace) |
| `projectile_collision_test_line` | items/projectiles.c | Matched | sim/src/projectile.rs | sim/src/projectile.rs (measure) |
| `projectile_detonate` | items/projectiles.c | Partial | sim/src/effects.rs:178 | sim/src/projectile.rs (measure); leaves out the super detonation, made when enough projectiles stick in one object (super_detonation) |
| `projectile_new` | items/projectiles.c | Matched | sim/src/projectile.rs | sim/src/projectile.rs (measure) |
| `projectile_collision` | items/projectiles.c | Partial | sim/src/effects.rs:177 | sim/src/effects.rs (measure); leaves out the effect a projectile makes as it starts to detonate (detonation_started), a material response's potential effect (potential_effect), the effects scaled by the impact's damage or angle (scale_effects_by) |
| `projectile_update` | items/projectiles.c | Partial | sim/src/projectile.rs:71 | sim/tests/projectile_deceleration_reference.rs (trace); leaves out the turning of a guided projectile towards its target (guided_angular_velocity) |
| `weapon_place` | items/weapons.c | Matched | import/src/level.rs:1046 | sim/tests/item_spawn_reference.rs (trace) |
| `weapon_magazine_finish_reload` | items/weapons.c | Matched | sim/src/weapon.rs:584 | sim/src/weapon.rs (measure) |
| `weapon_trigger_recover` | items/weapons.c | Matched | sim/src/weapon.rs:489 | sim/src/weapon.rs (measure) |
| `weapon_magazine_start_reload` | items/weapons.c | Matched | sim/src/weapon.rs:224 | sim/src/weapon.rs (measure) |
| `power` | items/weapons.c | Ported |  | `tools/engine_math_reference.py` |
| `random` | items/weapons.c | Ported |  | `tools/engine_random_reference.py` |
| `weapon_new` | items/weapons.c | Matched | sim/src/weapon.rs:170 | sim/src/weapon.rs (measure) |
| `weapon_export_function_values` | items/weapons.c | Partial | sim/src/functions.rs:156 | sim/src/functions.rs (measure); leaves out the charged illumination exported while charged (charged_illumination) |
| `weapon_trigger_can_fire_again` | items/weapons.c | Matched | sim/src/weapon.rs:424 | sim/src/weapon.rs (measure) |
| `weapon_effect_new` | items/weapons.c | Matched | sim/src/lib.rs:594 | sim/tests/weapons.rs (measure) |
| `weapon_ready` | items/weapons.c | Partial | sim/src/weapon.rs:208 | sim/src/weapon.rs (measure); leaves out weapon_reset of its triggers and magazines, the ready time taken from the first-person animation's length |
| `projectile_distribute` | items/weapons.c | Matched | sim/src/projectile.rs | sim/src/projectile.rs (measure) |
| `trigger_create_projectiles` | items/weapons.c | Partial | sim/src/projectile.rs | sim/src/projectile.rs (measure); leaves out the tracers, one every so many rounds (rounds_between_tracers), the inner cone of the projectiles' error (projectile_error_inner_cone_angle) |
| `weapon_trigger_fire` | items/weapons.c | Partial | sim/src/weapon.rs:496 | sim/src/weapon.rs (measure); leaves out the alternate shots loaded at most (maximum_alternate_shots_loaded), the detonation of a weapon past its heat threshold (heat_detonation_threshold) |
| `weapon_trigger_begin_firing` | items/weapons.c | Matched | sim/src/weapon.rs:440 | sim/src/weapon.rs (measure) |
| `weapon_trigger_release_charge` | items/weapons.c | Matched | sim/src/weapon.rs:466 | sim/src/weapon.rs (measure) |
| `weapon_update` | items/weapons.c | Partial | sim/src/weapon.rs:2 | sim/tests/weapons.rs (measure); leaves out the alternate shots loaded at most (maximum_alternate_shots_loaded), the rate of fire past which a trigger's barrels blur (blurred_rate_of_fire) |
| `item_rested_surface_index` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `item_rested_surface` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `item_set_rested_surface` | items/items.c | Ported |  | `tools/engine_item_reference.py` |

102 Missing (95 reached from `main_loop`), by file:

- items/equipment.c: `equipment_definition_handle_pickup`
- items/garbage.c: `garbage_update`, `garbage_new`
- items/items.c: `object_get_type`, `items_initialize`, `items_initialize_for_new_map`, `items_dispose_from_old_map`, `items_dispose`, `item_new`, `item_delete`, `dangerous_items_near_player`, `item_in_unit_inventory`, `item_get_position_even_if_in_inventory`, `item_detonate`, `valid_real_vector3d_axes3`, `valid_real_matrix4x3`
- items/projectiles.c: `projectiles_initialize`, `projectiles_initialize_for_new_map`, `projectiles_dispose_from_old_map`, `projectiles_dispose`, `projectile_kill_tracer`, `projectiles_delete_all`, `projectile_delete`, `projectile_set_target_object_index`, `projectile_make_tracer`, `projectile_get_ballistic_acceleration`, `projectile_estimate_time_to_target`, `dangerous_projectiles_near_player`, `projectile_handle_deleted_object`, `projectile_adjust_for_angular_velocity_change`, `projectile_export_function_values`, `projectile_handle_parent_destroyed`, `projectile_aim_ballistic`, `projectile_aim_linear`, `projectile_aim`, `projectile_set_action`, `projectile_effect_new`, `projectile_accelerate`
- items/weapon_definitions.c: `weapon_definition_get_damage_potential`
- items/weapons.c: `animation_convert_frame_to_pal`, `animation_key_frame_index`, `animation_update`, `animation_choose_random_permutation`, `weapons_initialize`, `weapons_initialize_for_new_map`, `weapons_dispose_from_old_map`, `weapons_dispose`, `weapon_preprocess_node_orientations`, `weapon_get_label`, `weapon_set_integrated_light_power`, `weapon_estimate_time_to_target`, `weapon_can_be_fired`, `weapon_useful`, `weapon_compute_movement_penalty`, `weapon_melee_attack`, `weapon_must_be_readied`, `weapon_is_flag`, `weapon_prevents_grenade_throwing`, `weapon_get_first_person_animation_time`, `weapon_overcharged`, `weapon_precache_projectile_trails`, `weapon_magazine_start_chamber`, `weapon_magazine_finish_chamber`, `weapon_trigger_fully_charged`, `weapon_trigger_idle`, `weapon_trigger_locked`, `weapon_trigger_finish_tracking`, `weapon_state_next`, `weapon_trigger_get`, `weapon_magazine_get`, `weapon_trigger_count`, `weapon_trigger_get_charged_fraction`, `weapon_busy`, `weapon_magazine_state_change_ok`, `weapon_get_effect_object_index`, `weapon_get_owner_object_index`, `weapon_set_total_rounds`, `weapon_delete`, `weapon_handle_potential_inventory_item`, `weapon_owner_update`, `weapon_build_weapon_interface_state`, `weapon_reloading`, `weapon_rotate_zoom_level`, `weapon_get_zoom_magnification`, `weapon_get_field_of_view`, `weapon_prevents_melee_attack`, `weapon_get_projectile_owner_object_index`, `weapon_magazine_idle`, `weapon_effect_looping_new`, `weapon_detonate`, `weapon_trigger_change_state`, `weapon_state_key_frame`, `weapon_magazine_state_interruptable`, `weapon_reset`, `weapon_state_interruptable`, `weapon_set_current_amount`, `weapon_put_away`, `weapon_aim`, `weapon_stop_reload`, `weapon_set_state`, `weapon_trigger_overload`, `weapon_trigger_overcharged`

## `main`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `console_initialize` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_initialize_for_new_map` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_open` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_close` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_is_active` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_printf` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_warning` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_process_command` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_get_text_to_autocomplete` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_complete` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_startup` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_dispose` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `console_update` | main/console.c | Replaced | dev/src/script.rs | the editor and tools |
| `gamepad_button_is_down` | main/main.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only; pad events are filtered out |
| `main_menu_unload` | main/main.c | Replaced |  | through its caller `network_game_create_game_objects` |
| `main_get_window_count` | main/main.c | Replaced |  | through its caller `_rasterizer_detail_objects_begin` |
| `main_native_error_tail` | main/main.c | Replaced |  | through its caller `halt_and_catch_fire` |
| `main_frame_rate_debug` | main/main.c | Replaced | dev/src/profile.rs | the editor and tools |
| `main_taking_screenshot` | main/main.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `main_movie_start` | main/main.c | Replaced | ui/src/loading.rs | Bink; no Bink movies; the loading screen stands in |
| `main_framerate_render` | main/main.c | Replaced |  | through its caller `interface_draw_fullscreen_overlays` |
| `halt_and_catch_fire` | main/main.c | Replaced |  | through its caller `main` |
| `main_loop` | main/main.c | Partial | ffi/src/lib.rs:1133 | ffi/src/lib.rs (measure); leaves out the menu and the shell, switching maps, the network connection, the console's frame |
| `debug_keys_initialize` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_keys_dispose` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_keys_update` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_key_select_actor` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_key_select_next_encounter` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_key_select_prev_encounter` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_key_select_next_actor` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_key_select_prev_actor` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_key_show_actor_spray` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_key_profile_reset` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_key_erase_all_actors` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_key_play_animation` | main/debug_keys.c | Replaced | dev/src/control.rs | the editor and tools |

91 Missing (91 reached from `main_loop`), by file:

- main/main.c: `main_get_seconds_elapsed`, `game_connection_set`, `game_connection`, `main_disallow_persistent_storage`, `main_set_map_name`, `main_defer_map_map_change`, `main_set_multiplayer_map_name`, `main_get_map_name`, `main_get_multiplayer_map_name`, `main_set_difficulty`, `main_get_difficulty`, `sort_desired_local_player_controllers`, `create_local_players`, `main_reset_map`, `main_revert_map`, `main_skip_cinematic`, `main_save_map_nonsafe`, `main_saving_map`, `main_save_cancel`, `main_save_map_safe`, `main_won_map`, `main_lost_map`, `main_respawn`, `main_save_core`, `main_save_core_name`, `main_load_core`, `main_load_core_at_startup`, `main_load_core_name`, `main_load_core_name_at_startup`, `main_switch_structure_bsp`, `main_skip`, `main_queue_map_name`, `main_queue_map_private`, `main_goto_main_menu`, `main_menu_precache_resources`, `main_menu_ensure_player_queues_exist`, `main_menu_fade_active`, `main_menu_switch_to_single_player`, `main_set_game_connection_to_film_playback`, `main_get_solo_level_from_name`, `main_get_current_solo_level`, `main_get_solo_level_name`, `main_run_demos`, `screenshot_record`, `compute_window_bounds`, `compute_subframe_counts`, `main_new_map`, `main_change_map_name`, `main_native_build_label`, `main_coop_host`, `main_revert_map_private`, `main_skip_cinematic_private`, `main_skip_private`, `main_saving_map_private`, `main_save_map_private`, `main_switch_to_structure_bsp_private`, `main_death_timer_expired`, `main_lost_map_private`, `main_respawn_private`, `main_load_core_private`, `main_save_core_private`, `main_run_demos_private`, `main_framerate_throttle_enabled`, `main_rasterizer_throttle`, `main_won_map_private`, `main_movie_stop`, `main_stop_time`, `main_start_time`, `main_crash`, `main_print_version`, `main_vertical_blank_interrupt_handler`, `main_save_current_solo_map`, `main_load_last_solo_map`, `main_save_map_no_timeout`, `main_load_ui_scenario`, `main_menu_load`, `main_roll_credits`, `main_pregame_render`, `set_window_camera_values`, `main_present_frame`, `main_exit`, `main_reset_time`, `main_update_time_unthrottled`, `main_loop_of_death`, `main_update_time`, `main_setup_connection`, `main_initialize_time`, `main_reset_map_private`, `screenshot_render`, `main_game_render`, `main_loop_iteration`

## `math`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `geosphere_dispose` | math/geometry.c | Ported |  | `tools/engine_math_reference.py` |
| `convex_hull3d_verify` | math/geometry.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `plane2d_distance_to_point` | math/geometry.c | Native |  | `glam::Vec2` |
| `vector_intersect_plane2d` | math/geometry.c | Replaced |  | through its caller `convex_polygon2d_clip_to_plane` |
| `subdivide_triangle` | math/geometry.c | Ported | sim/src/random.rs:178 | `tools/engine_math_reference.py` |
| `get_face_vertex` | math/geometry.c | Ported | sim/src/random.rs:193 | `tools/engine_math_reference.py` |
| `convex_hull2d_verify` | math/geometry.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `convex_hull2d_test_point_indexed` | math/geometry.c | Replaced |  | through its caller `build_structure_lens_flares` |
| `convex_polygon2d_clip_to_plane` | math/geometry.c | Replaced |  | through its caller `convex_hull2d_intersect` |
| `convex_polygon2d_verify` | math/geometry.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `convex_polygon3d_clip_to_plane` | math/geometry.c | Replaced |  | through its caller `portal_hull_from_points` |
| `convex_polygon3d_verify` | math/geometry.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `get_edge_vertex` | math/geometry.c | Ported | sim/src/random.rs:224 | `tools/engine_math_reference.py` |
| `convex_hull2d_intersect` | math/geometry.c | Replaced |  | through its caller `structure_visibility_find_mirror` |
| `calculate_vertex` | math/geometry.c | Ported | sim/src/random.rs:255 | `tools/engine_math_reference.py` |
| `geosphere_new` | math/geometry.c | Ported | sim/src/random.rs:5 | `tools/engine_math_reference.py` |
| `matrix4x3_identity` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_transpose` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_inverse` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_scale` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_translation` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_rotation_from_axis_and_angle` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_rotation_from_quaternion` | math/matrix_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `matrix4x3_from_point_and_quaternion` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_from_orientation` | math/matrix_math.c | Ported | sim/src/animation.rs:104 | `tools/engine_animation_reference.py` |
| `matrix4x3_to_point_and_vectors` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_transform_point` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_transform_vector` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_transform_normal` | math/matrix_math.c | Ported | sim/src/animation.rs:169 | `tools/engine_math_reference.py` |
| `matrix4x3_inverse_transform_point` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_inverse_transform_vector` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_inverse_transform_normal` | math/matrix_math.c | Ported | sim/src/animation_blend.rs:178 | `tools/engine_math_reference.py` |
| `matrix4x3_multiply` | math/matrix_math.c | Ported | sim/src/animation.rs:131 | `tools/engine_animation_reference.py` |
| `matrix3x3_determinant` | math/matrix_math.c | Native |  | `glam::Mat3` |
| `matrix3x3_transpose` | math/matrix_math.c | Native |  | `glam::Mat3` |
| `matrix3x3_inverse` | math/matrix_math.c | Native |  | `glam::Mat3` |
| `matrix3x3_from_axis_and_angle` | math/matrix_math.c | Native |  | `glam::Mat3` |
| `matrix3x3_multiply` | math/matrix_math.c | Native | sim/src/vehicle.rs:934 | `glam::Mat3` |
| `matrix3x3_transform_vector` | math/matrix_math.c | Native |  | `glam::Mat3` |
| `matrix4x3_rotation_from_vectors` | math/matrix_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `matrix4x3_rotation_from_angles` | math/matrix_math.c | Matched | sim/src/objects.rs:400 | sim/tests/math_extra_reference.rs (trace) |
| `matrix4x3_rotation_to_angles` | math/matrix_math.c | Native |  | `glam::Quat` |
| `matrix4x3_rotation_to_quaternion` | math/matrix_math.c | Native |  | `glam::Quat` |
| `matrix4x3_from_point_and_vectors` | math/matrix_math.c | Ported | sim/src/animation.rs:93 | `tools/engine_animation_reference.py` |
| `vector_from_matrices4x3` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_transform_plane` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix4x3_inverse_transform_plane` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `matrix3x3_from_forward_and_up` | math/matrix_math.c | Native |  | `glam::Mat3` |
| `matrix3x3_rotation_to_quaternion` | math/matrix_math.c | Native | sim/src/vehicle.rs:966 | `glam::Quat` |
| `matrix4x3_from_plane` | math/matrix_math.c | Native |  | `glam::Affine3A` |
| `periodic_functions_dispose` | math/periodic_functions.c | Replaced |  | through its caller `real_math_dispose` |
| `periodic_function_evaluate` | math/periodic_functions.c | Ported | sim/src/periodic.rs:165 | `tools/engine_cause_damage_reference.py` |
| `transition_function_evaluate` | math/periodic_functions.c | Ported | sim/src/damage.rs:37 | `tools/engine_cause_damage_reference.py` |
| `transition_function_build_table` | math/periodic_functions.c | Ported | sim/src/periodic.rs:199 | `tools/engine_cause_damage_reference.py` |
| `periodic_function_build_table` | math/periodic_functions.c | Ported | sim/src/periodic.rs:93 | `tools/engine_cause_damage_reference.py` |
| `periodic_functions_initialize` | math/periodic_functions.c | Ported | sim/src/periodic.rs:45 | `tools/engine_cause_damage_reference.py` |
| `lock_global_random_seed` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `unlock_global_random_seed` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `get_global_random_seed_address` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `get_random_seed` | math/random_math.c | Matched | sim/src/random.rs | sim/tests/random_reference.rs (trace) |
| `get_global_local_random_seed_address` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `random_seed_debug_log` | math/random_math.c | Replaced | dev/src/logbook.rs | the editor and tools |
| `get_number_suitable_for_initializing_random_seed` | math/random_math.c | Replaced |  | through its caller `random_math_initialize` |
| `random_math_initialize` | math/random_math.c | Replaced |  | through its caller `real_math_initialize` |
| `random_math_dispose` | math/random_math.c | Replaced |  | through its caller `real_math_dispose` |
| `real_seed_random` | math/random_math.c | Ported | audio/src/choice.rs:30 | `tools/engine_cause_damage_reference.py` |
| `real_seed_random_range` | math/random_math.c | Ported | ffi/src/decals.rs:126 | `tools/engine_math_reference.py` |
| `seed_random` | math/random_math.c | Ported | audio/src/choice.rs:24 | `tools/engine_cause_damage_reference.py` |
| `seed_random_range` | math/random_math.c | Ported | audio/src/choice.rs:36 | `tools/engine_cause_damage_reference.py` |
| `direction3d_from_table` | math/random_math.c | Ported |  | `tools/engine_math_reference.py` |
| `seed_random_direction3d` | math/random_math.c | Matched | sim/src/random.rs:55 | sim/tests/random_reference.rs (trace) |
| `seed_random_vector_in_cone3d` | math/random_math.c | Ported | sim/src/random.rs:66 | `tools/engine_math_reference.py` |
| `vector_tree_new` | math/vector_tree.c | Native |  | `std::collections::BTreeMap` |
| `vector_tree_delete` | math/vector_tree.c | Native |  | `std::collections::BTreeMap` |
| `vectors_equal` | math/vector_tree.c | Native |  | `std::collections::BTreeMap` |
| `vector_tree_add_node` | math/vector_tree.c | Native |  | `std::collections::BTreeMap` |
| `vector_tree_find` | math/vector_tree.c | Native |  | `std::collections::BTreeMap` |
| `set_rectangle2d` | math/integer_math.c | Replaced |  | through its caller `rasterizer_draw_string` |
| `set_point2d` | math/integer_math.c | Native |  | `min` |
| `offset_point2d` | math/integer_math.c | Native |  | `min` |
| `rectangle2d_width` | math/integer_math.c | Replaced |  | through its caller `draw_string` |
| `rectangle2d_height` | math/integer_math.c | Native |  | `min` |
| `inset_rectangle2d` | math/integer_math.c | Native |  | `min` |
| `offset_rectangle2d` | math/integer_math.c | Replaced |  | through its caller `bink_draw_frame` |
| `adjust_rectangle2d` | math/integer_math.c | Native |  | `min` |
| `intersect_rectangles2d` | math/integer_math.c | Native |  | `min` |
| `rectangle2d_hull_from_rectangles2d` | math/integer_math.c | Native |  | `min` |
| `point2d_in_rectangle2d` | math/integer_math.c | Native |  | `min` |
| `interior_rectangle2d` | math/integer_math.c | Native |  | `min` |
| `equal_rectangle2d` | math/integer_math.c | Native |  | `min` |
| `equal_point2d` | math/integer_math.c | Native |  | `min` |
| `floor_log2` | math/integer_math.c | Replaced |  | through its caller `bitmap_swizzle_vector2d` |
| `ceiling_log2` | math/integer_math.c | Native |  | `next_power_of_two` |
| `floor_power2` | math/integer_math.c | Native |  | `next_power_of_two` |
| `ceiling_power2` | math/integer_math.c | Native |  | `next_power_of_two` |
| `bit_vector_and` | math/integer_math.c | Native |  | `u32` |
| `bit_vector_or` | math/integer_math.c | Native |  | `u32` |
| `bit_vector_not` | math/integer_math.c | Replaced |  | through its caller `ai_debug_communication_toggle_bits` |
| `scale_rectangle2d` | math/integer_math.c | Native |  | `min` |
| `periodic_function_build_variable_period_x_table` | math/periodic_functions.c | Ported | sim/src/periodic.rs:73 | `tools/engine_cause_damage_reference.py` |
| `magnitude_squared3d_right_associated` | math/real_math.c | Native |  | `glam::Vec3` |
| `real_math_initialize` | math/real_math.c | Replaced |  | through its caller `shell_initialize` |
| `real_math_dispose` | math/real_math.c | Replaced |  | through its caller `shell_dispose` |
| `real_math_reset_precision` | math/real_math.c | Native |  | `libm::sqrtf` |
| `perpendicular2d` | math/real_math.c | Native |  | `glam::Vec2` |
| `perpendicular3d` | math/real_math.c | Native |  | `glam::Vec3` |
| `perpendicular4d` | math/real_math.c | Native |  | `glam::Vec3` |
| `rotate_vector_about_axis` | math/real_math.c | Ported | ffi/src/sprite.rs:102 | `tools/engine_item_reference.py` |
| `vectors_interpolate` | math/real_math.c | Replaced |  | through its caller `get_flash_color` |
| `points_interpolate` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `scalars_interpolate` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `scalars_interpolate_and_clamp_0_to_1` | math/real_math.c | Replaced |  | through its caller `rasterizer_screen_effect_get_cinematic_parameters` |
| `component_vectors_from_normal3d` | math/real_math.c | Ported |  | `tools/engine_physics_math_reference.py` |
| `quaternions_multiply` | math/real_math.c | Ported | sim/src/animation.rs:221 | `tools/engine_animation_reference.py` |
| `quaternions_interpolate` | math/real_math.c | Ported | sim/src/animation.rs:208 | `tools/engine_animation_reference.py` |
| `quaternion_transform_point` | math/real_math.c | Ported |  | `tools/engine_item_reference.py` |
| `fast_vector_intersects_sphere` | math/real_math.c | Ported |  | `tools/engine_collision_reference.py` |
| `point_in_triangle3d` | math/real_math.c | Replaced |  | through its caller `structure_render_surface_from_point_and_leaf` |
| `fast_normalize2d` | math/real_math.c | Native |  | `glam::Vec2` |
| `fast_normalize3d` | math/real_math.c | Native |  | `glam::Vec3` |
| `cross_product_magnitude3d` | math/real_math.c | Native |  | `glam::Vec3` |
| `dequantize_byte_to_real` | math/real_math.c | Ported | sim/src/vehicle_animation.rs:189 | `tools/engine_math_extra_reference.py` |
| `quantize_real_to_byte_lower_bound` | math/real_math.c | Matched | sim/src/vehicle_animation.rs:171 | sim/tests/math_extra_reference.rs (trace) |
| `signed_angle_between_vectors2d` | math/real_math.c | Native |  | `glam::Vec2` |
| `angle_between_vectors2d` | math/real_math.c | Native |  | `glam::Vec2` |
| `angle_between_vectors3d` | math/real_math.c | Ported | sim/src/vehicle.rs:1114 | `tools/engine_math_reference.py` |
| `angle_between_normals3d` | math/real_math.c | Native |  | `glam::Vec3` |
| `yaw_vectors` | math/real_math.c | Ported | sim/src/vehicle.rs:893 | `tools/engine_vehicle_physics_reference.py` |
| `pitch_vectors` | math/real_math.c | Ported | sim/src/vehicle.rs:903 | `tools/engine_vehicle_physics_reference.py` |
| `fast_normals_interpolate` | math/real_math.c | Native |  | `glam::Vec3` |
| `normals_interpolate` | math/real_math.c | Native |  | `glam::Vec3` |
| `reflect_vector3d` | math/real_math.c | Native |  | `glam::Vec3` |
| `quaternion_normalize` | math/real_math.c | Ported | sim/src/animation.rs:190 | `tools/engine_animation_reference.py` |
| `quaternion_from_angle_and_vector` | math/real_math.c | Native |  | `glam::Quat` |
| `quaternion_to_angle_and_vector` | math/real_math.c | Native | sim/src/vehicle.rs:1003 | `glam::Quat` |
| `quaternions_interpolate_and_normalize` | math/real_math.c | Ported | sim/src/animation.rs:202 | `tools/engine_animation_reference.py` |
| `orientations_interpolate` | math/real_math.c | Replaced |  | through its caller `first_person_weapon_build_node_matrices` |
| `euler_angles2d_from_vector3d` | math/real_math.c | Ported | sim/src/animation_blend.rs:179 | `tools/engine_math_reference.py` |
| `vector3d_from_angle` | math/real_math.c | Native |  | `libm::sinf` |
| `vector3d_from_euler_angles2d` | math/real_math.c | Matched | content/src/level.rs:375 | sim/tests/math_extra_reference.rs (trace) |
| `point_to_line_distance_squared3d` | math/real_math.c | Native |  | `glam::Vec3` |
| `vector_to_line_distance_squared3d` | math/real_math.c | Native |  | `glam::Vec3` |
| `accelerate_to_position` | math/real_math.c | Matched | sim/src/device_animation.rs:3 | sim/tests/math_extra_reference.rs (trace); sim/src/first_person_states.rs has a copy for the weapons, which never ask for a periodic range; the trace checks both |
| `valid_real_sine_cosine` | math/real_math.c | Native |  | `libm::sinf` |
| `set_real_point3d` | math/real_math.h | Native |  | `glam::Vec3` |

64 Missing (18 reached from `main_loop`), by file:

- math/geometry.c: `vector_intersect_plane3d`, `build_torus`, `convex_hull2d`, `convex_hull2d_test_circle`, `convex_hull2d_test_point`, `convex_hull2d_area`, `convex_hull2d_perimeter`, `convex_hull2d_test_vector`, `convex_hull3d_begin`, `convex_hull3d_expand`, `convex_hull3d`, `convex_hull3d_test_point`, `convex_hull3d_test_vector`, `points_dimension2d`
- math/integer_math.c: `integer_square_root`
- math/matrix_math.c: `valid_real_plane3d`
- math/periodic_functions.c: `@periodic_function_build_variable_period_x_table`
- math/probability.c: `factorial`, `permutations`, `combinations`, `permute`, `combine`
- math/random_math.c: `seed_random_orientation`
- math/real_math.c: `component_vectors_from_direction3d`, `vectors3d_from_euler_angles3d`, `fast_vector_intersection_with_sphere`, `point_in_rectangle2d`, `point_in_rectangle3d`, `circle_intersects_rectangle2d`, `point_in_cone2d`, `point_in_cone3d`, `point_in_sector2d`, `point_in_sector3d`, `circle_intersects_cone2d`, `circle_intersects_sector2d`, `sphere_intersects_cone3d`, `sphere_intersects_sector3d`, `point_in_triangle2d`, `circle_intersects_triangle2d`, `pill_intersects_triangle2d`, `pill_intersects_rectangle2d`, `sphere_intersects_rectangle3d`, `sphere_intersects_triangle3d`, `pill_intersects_triangle3d`, `vector_intersects_rectangle2d`, `vector_intersects_pill2d`, `vector_intersects_pill3d`, `vector_intersects_triangle2d`, `vector_intersects_triangle3d`, `vector_intersects_rectangle3d`, `pill_intersects_rectangle3d`, `quantize_real_to_byte_upper_bound`, `quantize_real_to_byte_rectangle3d`, `pin_normal_to_cone3d`, `refract_vector3d`, `point_in_pill2d`, `sphere_test_vector3d`, `pill_test_vector3d`, `point_from_planes3d`, `line_from_planes3d`, `angular_accelerate_to_position`, `accelerate_to_velocity3d`, `accerate_to_position3d`, `accelerate_to_velocity`

## `memory`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `dynamic_array_new` | memory/array.c | Native |  | `Vec` |
| `dynamic_array_resize` | memory/array.c | Native |  | `Vec` |
| `dynamic_array_delete` | memory/array.c | Native |  | `Vec` |
| `dynamic_array_add_element` | memory/array.c | Native |  | `Vec` |
| `dynamic_array_get_element` | memory/array.c | Native |  | `Vec` |
| `dynamic_array_delete_element` | memory/array.c | Native |  | `Vec` |
| `static_array_new` | memory/array.c | Native |  | `Vec` |
| `static_array_resize` | memory/array.c | Native |  | `Vec` |
| `static_array_add_element` | memory/array.c | Native |  | `Vec` |
| `static_array_get_element` | memory/array.c | Native |  | `Vec` |
| `static_array_delete_element` | memory/array.c | Native |  | `Vec` |
| `byte_swap_memory` | memory/byte_swapping.c | Replaced |  | through its caller `data_decode_memory` |
| `_byte_swap_data` | memory/byte_swapping.c | Native |  | `from_le_bytes` |
| `byte_swap_codes_size` | memory/byte_swapping.c | Native |  | `from_le_bytes` |
| `byte_swap_data` | memory/byte_swapping.c | Replaced |  | through its caller `data_packet_group_append_packet_header` |
| `byte_swap_data_explicit` | memory/byte_swapping.c | Native |  | `from_le_bytes` |
| `circular_queue_reset` | memory/circular_queue.c | Replaced |  | through its caller `network_client_unreliable_connection_read` |
| `circular_queue_verify` | memory/circular_queue.c | Replaced |  | through its caller `circular_queue_delete` |
| `circular_queue_new` | memory/circular_queue.c | Replaced |  | through its caller `network_connection_create_client_from_endpoint` |
| `circular_queue_delete` | memory/circular_queue.c | Replaced |  | through its caller `network_connection_delete` |
| `circular_queue_size` | memory/circular_queue.c | Replaced |  | through its caller `circular_queue_dequeue_data` |
| `circular_queue_free_space` | memory/circular_queue.c | Replaced |  | through its caller `network_connection_idle` |
| `circular_queue_queue_data` | memory/circular_queue.c | Replaced |  | through its caller `network_connection_idle` |
| `circular_queue_dequeue_data` | memory/circular_queue.c | Replaced |  | through its caller `network_client_reliable_connection_read` |
| `crc_new` | memory/crc.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `build_crc_table` | memory/crc.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `crc_checksum_buffer` | memory/crc.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `data_allocation_size` | memory/data.c | Native |  | `Vec` |
| `data_initialize` | memory/data.c | Native |  | `Vec` |
| `datum_try_and_get` | memory/data.c | Native |  | `Vec` |
| `datum_get` | memory/data.c | Native |  | `Vec` |
| `data_verify` | memory/data.c | Native |  | `Vec` |
| `datum_initialize` | memory/data.c | Native |  | `Vec` |
| `data_new` | memory/data.c | Native |  | `Vec` |
| `data_dispose` | memory/data.c | Native |  | `Vec` |
| `data_make_invalid` | memory/data.c | Native |  | `Vec` |
| `datum_new_at_index` | memory/data.c | Native |  | `Vec` |
| `datum_new` | memory/data.c | Matched | ffi/src/particles/slots.rs:3 | sim/tests/data_reference.rs (trace) |
| `datum_delete` | memory/data.c | Native |  | `Vec` |
| `data_delete_all` | memory/data.c | Native |  | `Vec` |
| `data_iterator_new` | memory/data.c | Native |  | `Vec` |
| `data_iterator_next` | memory/data.c | Native |  | `Vec` |
| `data_next_index` | memory/data.c | Matched | ffi/src/particles/slots.rs:4 | sim/tests/data_reference.rs (trace) |
| `data_prev_index` | memory/data.c | Replaced |  | through its caller `ai_debug_change_selected_encounter` |
| `data_compact` | memory/data.c | Native |  | `Vec` |
| `data_make_valid` | memory/data.c | Native |  | `Vec` |
| `data_compress` | memory/data_compress.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `data_decompressed_size` | memory/data_compress.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `data_decompress` | memory/data_compress.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `data_encode_new` | memory/data_encoding.c | Replaced |  | through its caller `data_packet_encode` |
| `data_encode_memory` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_encode` |
| `data_encode_integer` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_encode` |
| `data_encode_structures` | memory/data_encoding.c | Replaced | net/src/wire.rs | networking and Xbox Live |
| `data_encode_array` | memory/data_encoding.c | Replaced | net/src/wire.rs | networking and Xbox Live |
| `data_encode_string` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_encode` |
| `data_decode_new` | memory/data_encoding.c | Replaced |  | through its caller `data_packet_decode` |
| `data_decode_structures` | memory/data_encoding.c | Replaced | net/src/wire.rs | networking and Xbox Live |
| `data_decode_memory` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_decode` |
| `data_decode_byte` | memory/data_encoding.c | Replaced |  | through its caller `data_decode_integer` |
| `data_decode_short` | memory/data_encoding.c | Replaced |  | through its caller `data_decode_integer` |
| `data_decode_long` | memory/data_encoding.c | Replaced |  | through its caller `data_decode_integer` |
| `data_decode_int64` | memory/data_encoding.c | Replaced | net/src/wire.rs | networking and Xbox Live |
| `data_decode_integer` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_decode` |
| `data_decode_array` | memory/data_encoding.c | Replaced | net/src/wire.rs | networking and Xbox Live |
| `data_decode_string` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_decode` |
| `data_packet_group_initialize` | memory/data_packet_groups.c | Replaced |  | through its caller `initialize_network_game_packets` |
| `data_packet_groups_get_error` | memory/data_packet_groups.c | Replaced | net/src/wire.rs | networking and Xbox Live |
| `data_packet_group_decode_packet` | memory/data_packet_groups.c | Replaced |  | through its caller `decode_network_game_message` |
| `data_packet_group_append_packet_header` | memory/data_packet_groups.c | Replaced |  | through its caller `data_packet_group_encode_packet` |
| `data_packet_group_encode_packet` | memory/data_packet_groups.c | Replaced |  | through its caller `encode_network_game_message` |
| `_data_packet_verify` | memory/data_packets.c | Replaced | net/src/wire.rs | networking and Xbox Live |
| `_data_packet_encode` | memory/data_packets.c | Replaced | net/src/wire.rs | networking and Xbox Live |
| `_data_packet_decode` | memory/data_packets.c | Replaced | net/src/wire.rs | networking and Xbox Live |
| `data_packet_verify` | memory/data_packets.c | Replaced |  | through its caller `data_packet_decode` |
| `data_packet_encode` | memory/data_packets.c | Replaced |  | through its caller `data_packet_group_encode_packet` |
| `data_packet_decode` | memory/data_packets.c | Replaced |  | through its caller `data_packet_group_decode_packet` |
| `hashtable_new` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `hashtable_set_user_data` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `hashtable_delete` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `default_hash_function` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `hashtable_search` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `hashtable_get` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `hashtable_remove` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `hashtable_capacious_put` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `hashtable_grow` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `hashtable_put` | memory/hashtable.c | Native |  | `std::collections::BTreeMap` |
| `lra_full` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_default_update_proc` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_default_delete_proc` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_block_delete` | memory/lra_cache.c | Native |  | `Vec` |
| `verify_lra_cache_block` | memory/lra_cache.c | Native |  | `Vec` |
| `verify_lra_cache` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_new` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_dispose` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_flush` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_free` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_lock` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_unlock` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_block_offset` | memory/lra_cache.c | Native |  | `Vec` |
| `lra_allocate` | memory/lra_cache.c | Native |  | `Vec` |
| `lrar_default_lock_proc` | memory/lrar_cache.c | Native |  | `Vec` |
| `lrar_default_unlock_proc` | memory/lrar_cache.c | Native |  | `Vec` |
| `lrar_unlock_block` | memory/lrar_cache.c | Native |  | `Vec` |
| `verify_lrar_cache_block` | memory/lrar_cache.c | Native |  | `Vec` |
| `verify_lrar_cache` | memory/lrar_cache.c | Native |  | `Vec` |
| `lrar_new` | memory/lrar_cache.c | Native |  | `Vec` |
| `lrar_dispose` | memory/lrar_cache.c | Native |  | `Vec` |
| `get_lrar_cache_block` | memory/lrar_cache.c | Native |  | `Vec` |
| `lrar_flush` | memory/lrar_cache.c | Native |  | `Vec` |
| `lrar_allocate` | memory/lrar_cache.c | Native |  | `Vec` |
| `lrar_block_address` | memory/lrar_cache.c | Native |  | `Vec` |
| `lrar_deallocate` | memory/lrar_cache.c | Native |  | `Vec` |
| `lru_default_new_block_proc` | memory/lru_cache.c | Native |  | `Vec` |
| `lru_default_purge_block_proc` | memory/lru_cache.c | Native |  | `Vec` |
| `get_lru_cache_block_offset` | memory/lru_cache.c | Native |  | `Vec` |
| `verify_lru_cache_block` | memory/lru_cache.c | Native |  | `Vec` |
| `verify_lru_cache` | memory/lru_cache.c | Native |  | `Vec` |
| `lru_new` | memory/lru_cache.c | Native |  | `Vec` |
| `lru_dispose` | memory/lru_cache.c | Native |  | `Vec` |
| `lru_flush` | memory/lru_cache.c | Native |  | `Vec` |
| `lru_free_blocks` | memory/lru_cache.c | Native |  | `Vec` |
| `lru_allocate` | memory/lru_cache.c | Native |  | `Vec` |
| `lru_lock` | memory/lru_cache.c | Native |  | `Vec` |
| `lru_unlock` | memory/lru_cache.c | Native |  | `Vec` |
| `lru_touch` | memory/lru_cache.c | Native |  | `Vec` |
| `lruv_allocation_size` | memory/lruv_cache.c | Replaced |  | through its caller `game_state_lruv_cache_new` |
| `lruv_update_function_pointers` | memory/lruv_cache.c | Replaced |  | through its caller `_rasterizer_decals_update_function_pointers` |
| `lruv_has_locked_proc` | memory/lruv_cache.c | Replaced |  | through its caller `rasterizer_decal_vertices_purge_proc` |
| `lruv_cache_bytes_to_pages` | memory/lruv_cache.c | Replaced |  | through its caller `lruv_block_new` |
| `lruv_cache_verify` | memory/lruv_cache.c | Replaced |  | through its caller `lruv_block_delete` |
| `lruv_initialize` | memory/lruv_cache.c | Replaced |  | through its caller `game_state_lruv_cache_new` |
| `lruv_delete` | memory/lruv_cache.c | Replaced |  | through its caller `_rasterizer_decals_dispose` |
| `lruv_idle` | memory/lruv_cache.c | Replaced |  | through its caller `rasterizer_decal_vertices_begin_update` |
| `lruv_block_delete` | memory/lruv_cache.c | Replaced |  | through its caller `_rasterizer_decal_vertices_delete` |
| `lruv_block_touch` | memory/lruv_cache.c | Replaced |  | through its caller `_sound_cache_sound_request` |
| `lruv_block_get_address` | memory/lruv_cache.c | Replaced |  | through its caller `_rasterizer_decal_vertices_lock` |
| `lruv_block_touched` | memory/lruv_cache.c | Replaced |  | through its caller `texture_cache_debug_render` |
| `lruv_cache_get_page_usage` | memory/lruv_cache.c | Replaced |  | through its caller `sound_cache_debug_render` |
| `lruv_resize` | memory/lruv_cache.c | Replaced |  | through its caller `texture_cache_return_memory` |
| `lruv_debug_to_file` | memory/lruv_cache.c | Replaced |  | through its caller `_texture_cache_bitmap_get_hardware_format` |
| `lruv_new` | memory/lruv_cache.c | Replaced |  | through its caller `sound_cache_new` |
| `lruv_flush` | memory/lruv_cache.c | Replaced |  | through its caller `_rasterizer_decals_dispose_from_old_map` |
| `lruv_block_new` | memory/lruv_cache.c | Replaced |  | through its caller `_rasterizer_decal_vertices_new` |
| `memory_pool_allocation_size` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_initialize` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_get_free_size` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_get_used_size` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_get_contiguous_free_size` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_block_compute_actual_size` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_block_new` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_verify` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_block_get` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_new` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_delete` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_block_allocate` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_block_free` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_compact` | memory/memory_pool.c | Native |  | `Vec` |
| `memory_pool_block_reallocate` | memory/memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_reset` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `memory_block_set_size_and_index` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `memory_block_get_real_size` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `memory_block_get_user_size` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `memory_block_get_pool_index` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_free_space_at_end_of_pool` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_find_first_unused_memory_block` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_set_next_block_index` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_find_space_between_blocks` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `memory_block_valid` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `memory_block_lock` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `memory_block_unlock` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `memory_block_is_locked` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `memory_block_get_user_address` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_compact` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_valid_block` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_dispose_block` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_lock_block` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_unlock_block` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_allocate` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `dispose_handle` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `lock_handle` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `unlock_handle` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `dispose_pointer` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_new_block` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_new_block_clear` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `stack_memory_pool_resize_block` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `pool_new_handle` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `pool_new_handle_clear` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `pool_resize_handle` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `pool_new_pointer` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `pool_new_pointer_clear` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `pool_resize_pointer` | memory/stack_memory_pool.c | Native |  | `Vec` |
| `texture_page_fraction_used` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_verify` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_new` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_delete` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_textures_begin` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_texture_get` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `qsort_texture_indexes` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_resort` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_texture_new` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_textures_cancel` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_textures_end` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_texture_delete` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `texture_page_resize` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `data_usable` | memory/data.c | Native |  | `Vec` |
| `texture_page_verify_and_return` | memory/texture_page.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | the enhanced lighting and Metal renderer |
| `match_init` | memory/zlib/contrib/asm386/gvmat32c.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `longest_match` | memory/zlib/contrib/asm386/gvmat32c.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `setcompressionlevel` | memory/zlib/contrib/iostream/zfstream.h | Replaced | import/src/lib.rs | maps imported to a typed format |
| `setcompressionstrategy` | memory/zlib/contrib/iostream/zfstream.h | Replaced | import/src/lib.rs | maps imported to a typed format |
| `read` | memory/zlib/contrib/iostream2/zstream.h | Replaced | import/src/lib.rs | maps imported to a typed format |
| `zstringlen` | memory/zlib/contrib/iostream2/zstream.h | Replaced | import/src/lib.rs | maps imported to a typed format |
| `write` | memory/zlib/contrib/iostream2/zstream.h | Replaced | import/src/lib.rs | maps imported to a typed format |
| `allocate_new_datablock` | memory/zlib/contrib/minizip/zip.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `OF` | memory/zlib/contrib/untgz/untgz.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `getoct` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `tar` |
| `strtime` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `tar` |
| `ExprMatch` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `matchname` |
| `makedir` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `tar` |
| `matchname` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `tar` |
| `tar` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `main` |
| `help` | memory/zlib/contrib/untgz/untgz.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `make_crc_table` | memory/zlib/crc32.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `get_crc_table` | memory/zlib/crc32.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `crc32` | memory/zlib/crc32.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `maketree` | memory/zlib/maketree.c | Replaced |  | through its caller `main` |
| `send_bits` | memory/zlib/trees.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tr_static_init` | memory/zlib/trees.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `gen_trees_header` | memory/zlib/trees.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `zlibVersion` | memory/zlib/zutil.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `zcalloc` | memory/zlib/zutil.c | Replaced |  | through its caller `main` |
| `zcfree` | memory/zlib/zutil.c | Replaced |  | through its caller `main` |

## `models`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `animation_get_frame_data` | models/model_animation_definitions.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_loop_frame_index` | models/model_animations.c | Matched | sim/src/animation.rs | sim/src/animation.rs (measure) |
| `animation_second_key_frame_index` | models/model_animations.c | Matched | sim/src/animation.rs | sim/src/animation.rs (measure) |
| `animation_sound_frame_index` | models/model_animations.c | Matched | sim/src/animation.rs | sim/src/animation.rs (measure) |
| `animation_is_compressed` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `build_damage_animation_index` | models/model_animations.c | Matched | content/src/object.rs:522 | sim/src/animation_graph.rs (measure) |
| `animation_set_frame_size` | models/model_animations.c | Replaced | import/src/object.rs | maps imported to a typed format; the tool's, when it builds an animation: nothing in the game calls it, and the importer reads what it made |
| `quaternion_decompress_8byte` | models/model_animations.c | Ported | sim/src/animation.rs:260 | `tools/engine_animation_reference.py` |
| `quaternion_decompress_6byte` | models/model_animations.c | Ported | sim/src/animation.rs:272 | `tools/engine_animation_reference.py` |
| `quaternion_compress_8byte` | models/model_animations.c | Replaced | import/src/object.rs | maps imported to a typed format; the tool's, when it builds an animation: nothing in the game calls it, and the importer reads what it made |
| `quaternion_compress_6byte` | models/model_animations.c | Replaced | import/src/object.rs | maps imported to a typed format; the tool's, when it builds an animation: nothing in the game calls it, and the importer reads what it made |
| `animation_graph_node_matrices_from_orientations` | models/model_animations.c | Ported | sim/src/animation.rs:1146 | `tools/engine_animation_reference.py` |
| `interpolate_node_orientations` | models/model_animations.c | Ported | sim/src/animation.rs:1122 | `tools/engine_animation_reference.py` |
| `animation_keyframe_search` | models/model_animations.c | Ported | sim/src/animation.rs:522 | `tools/engine_animation_reference.py` |
| `animation_choose_random_permutation_internal` | models/model_animations.c | Matched | sim/src/animation.rs:1227 | sim/tests/animation_permutation_reference.rs (trace) |
| `animation_get_keyframe_rotation` | models/model_animations.c | Ported | sim/src/animation.rs:448 | `tools/engine_animation_reference.py` |
| `animation_get_keyframe_translation` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_get_keyframe_scale` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_update_internal` | models/model_animations.c | Matched | sim/src/animation.rs:1252 | sim/src/animation.rs (measure) |
| `animation_get_node_orientations` | models/model_animations.c | Ported | sim/src/animation.rs:694 | `tools/engine_animation_reference.py` |
| `replacement_animation_apply` | models/model_animations.c | Ported | sim/src/animation.rs:726 | `tools/engine_animation_reference.py` |
| `overlay_animation_apply` | models/model_animations.c | Ported | sim/src/animation.rs:739 | `tools/engine_animation_reference.py` |
| `overlay_animation_apply_scaled` | models/model_animations.c | Ported | sim/src/animation.rs:844 | `tools/engine_animation_reference.py` |
| `overlay_animation_apply_continuous` | models/model_animations.c | Ported | sim/src/animation.rs:880 | `tools/engine_animation_reference.py` |
| `overlay_animation_apply_continuous_scaled` | models/model_animations.c | Matched | sim/src/animation.rs:893 | sim/src/animation.rs (measure) |
| `aiming_screen_apply` | models/model_animations.c | Ported | sim/src/animation.rs:762 | `tools/engine_animation_reference.py` |
| `render_model_parts` | models/models.c | Replaced | app/Sources/OpenCEEngineRender/ModelPass.swift | the enhanced lighting and Metal renderer |
| `model_interpolate_node_orientations` | models/models.c | Matched | sim/src/animation.rs | sim/tests/animation_reference.rs (trace) |
| `model_get_node_orientations` | models/models.c | Ported | sim/src/animation.rs:683 | `tools/engine_animation_reference.py` |
| `model_get_node_matrices` | models/models.c | Matched | sim/src/animation.rs | sim/tests/animation_reference.rs (trace) |
| `model_node_matrices_from_orientations` | models/models.c | Matched | sim/src/seats/model_reference.rs:2 | sim/src/seats/model_reference.rs (trace); the nodes' matrices match the game's bit for bit over six trees at sixty poses, with bad links and a loop; a node the walk never reaches keeps what its matrix had in the game and is the identity in the engine |
| `model_find_marker` | models/models.c | Matched | sim/src/seats/model_reference.rs:3 | sim/src/seats/model_reference.rs (trace); the engine finds a marker by its name in any case, searching in turn where the game halves the sorted list: the same marker for the tools' sorted, unique names (traced through model_get_marker_by_name); a list out of order could find differently |
| `model_get_default_inverse_matrix` | models/models.c | Replaced | ffi/src/render_model.rs | the enhanced lighting and Metal renderer |
| `model_geometry_part_build_tangent_matrices` | models/models.c | Replaced | import/src/object.rs | maps imported to a typed format |
| `render_model` | models/models.c | Matched | ffi/src/render_model.rs:67 | ffi/src/render_model.rs (measure) |
| `model_get_marker_by_name` | models/models.c | Partial | sim/src/seats/model_reference.rs:3 | sim/src/seats/model_reference.rs (trace); leaves out the marker's region and permutation: every instance of the marker is taken, whichever permutation is drawn, the mirrored flag, which turns the marker's left the other way, the most markers the caller has room for: every instance is taken; asked without permutations and unmirrored, the markers' matrices match the game's bit for bit over 960 lookups, markers on nodes the model does not have skipped as the game skips them |
| `model_build_tangent_matrices` | models/models.c | Replaced | import/src/object.rs | maps imported to a typed format |

11 Missing (9 reached from `main_loop`), by file:

- models/model_animation_definitions.c: `animation_get_frame_info`, `animation_list_get_string`
- models/model_animations.c: `animation_get_x_offsets`, `quaternion_decompress_6byte_renormalized`, `animation_graph_get_animation_by_name`, `animation_frame_get_xy_translation`, `inverse_kinematics_adjust_matrices`, `animation_get_root_matrix`, `animation_get_root_velocity`
- models/models.c: `model_find_node`, `model_geometry_fix_transparent_part_links`

## `networking`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `check_networking_and_generate_error` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_dispose` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_keep_alive` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_state` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_initiate_join_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_set_machine` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_machine` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_machine_index` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_available_games` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_error` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_seconds_to_game_start` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_write` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_address_matches_server` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_game_out_of_sync` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_ponged` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_accepted_into_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_game_settings_updated` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `unstrip_player_index` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_game_has_started` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_game_update` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_add_player_to_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_switch_to_postgame` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_switch_to_pregame` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_connection` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_remote_server_address` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_server_has_started_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_next_update_number` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_client_get_oos` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_add_player` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_update_local_player_data` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_request_start_time_change` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_countdown_timer_update` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_advertised_game_is_valid` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `add_advertised_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_set_error` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_update_precache_status` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_process_incoming_messages` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_leave_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_request_remove_player` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_remove_player` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_new_advertised_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_game_shutdown` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_reset` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_idle_searching` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_idle_joining` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_idle_pregame` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_idle_ingame` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_idle_postgame` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_create` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_idle` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_rejected_by_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_game_advertise` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_pong` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_machine_accepted` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_machine_rejected` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_game_settings_update` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_pregame_countdown` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_pregame_keep_alive` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_postgame_keep_alive` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_begin_game` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_graceful_game_exit_pregame` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_game_update` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_add_player_ingame` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_remove_player_ingame` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_game_over` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_switch_to_pregame` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message_server_graceful_game_exit_postgame` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_handle_message` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_initialize` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_server_allow_client_connections` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_connected` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_get_address` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_connect` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_set_connection_rejection_procedure` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_server_accept_client_connection` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_active` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_going_stale` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_client_unreliable_connection_read` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_log_traffic_event` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_keep_alive` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_delete` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_write` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_server_close_client_connection` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_create_client_from_endpoint` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_client_reliable_connection_read` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_idle_client_reliable_endpoint` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_new` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_read` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_disconnect` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_idle_server_reliable_endpoint` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_idle` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_is_active` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_set_number_of_games_played` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_set_random_seed` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_get_game` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_player_is_local` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_accept_remote_connections` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_should_accept_remote_connections` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_is_splitscreen_local` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_set_quickstart_local` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_is_quickstart_local` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `global_network_game_server_get` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `dispose_global_network_game_server` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_start_frame` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `global_network_game_client_get` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `create_global_network_game_client` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `dispose_global_network_game_client` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_start_frame` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_end_frame` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_get_local_machine_index` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_local_player_quit` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_abort` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_all_local_players_have_quit` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_request_immediate_start` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_get_number_of_games_played` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_get_random_seed` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `create_global_network_game_server` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_invalidate_player` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_add_machine` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_update_machine` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `xbox_set_machine_name` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_generate_local_machine_name` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_end_and_load_ui` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_reset_for_next_round` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_assign_players_to_team` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_player_is_valid` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_invalidate_machine` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_add_player` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `compare_network_players` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_spawn_player` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_player_is_valid` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_invalidate` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_update_player` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_remove_player` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_create_game_objects` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_remove_machine` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_get_random_player_name` | networking/network_game_ui.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `initialize_network_game_packets` | networking/network_messages.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_event` | networking/network_messages.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `encode_network_game_message` | networking/network_messages.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `create_network_game_message` | networking/network_messages.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `decode_network_game_message` | networking/network_messages.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `countdown_timer_update` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `countdown_timer_get_time_remaining` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `countdown_timer_increment` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `countdown_timer_decrement` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `countdown_timer_set_time_remaining` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_set_game_name` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_game_name` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_state` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_open_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_game_is_open` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_game_is_valid` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_player_quit_messages_ingame` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_start_network_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_switch_to_postgame` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_graceful_shutdown` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_client_machine_is_joined_to_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_accept_client_machine_into_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `player_name_is_unique` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `get_unique_random_name` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `get_unique_random_color` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_add_player_to_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_remove_player_from_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_adjust_machine_settings` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_all_machines_have_loaded` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_client_machine_game_loading_complete` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_client_machine_is_precached` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_client_update_packet` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_switch_machine_from_postgame_to_pregame` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_update_ticks` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_queue_player_for_addition` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_begin_game_start_countdown` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `server_needs_more_teams` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `server_has_a_player_on_each_machine` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `server_has_enough_machines` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `server_ok_to_countdown` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_invalidate_network_machine` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_generate_join_game_token` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_client_machine` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_connection` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_client_connection` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_machine_connection` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_client_machine_at_index` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_client_machine_at_address` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_game_can_start` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_pause_countdown` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_change_map_name` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_change_game_variant` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_ingame_addition_count` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_kick_machine` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_rejection_message` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_reject_connection_game_is_full` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_machine_may_add_player_ingame` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_last_refusal_code` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_accepts_late_joins` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_client_machine_count` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `dump_network_game_data` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_dump` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_remove_client_machine_from_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_remove_machine_from_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_update_countdown` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_client_machine_is_loaded` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_machine_has_waiting_players` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_dispose` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_idle` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_reset_to_pregame` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_create` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_write` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_machine_is_loading_late` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_message_to_client_machine` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_game_settings_to_client_machine` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_server_machines` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_client_machine_in_game` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_server_send_to_machine` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_message_to_machine` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_message_to_all_machines` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_player_joined_info_ingame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_game_data_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_server_send_to_machine_reliably` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_server_send_to_all_reliably` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_game_settings_to_all_machines` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_flush_game_data_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_queue_client_player` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_broadcast_game_search` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_ping` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_join_game_request` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_add_player_request_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_datagram` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_client_message` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_console_initialize` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_console_dispose` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_console_print` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_client_write` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_console_process` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_network_available` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_network_lost` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_map_name_is_valid` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_process_messages` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_process_last_messages` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_advertised_game_compatible` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_join_first_available_game` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_set_team` | networking/network_client_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_client_receive_game_settings_piece` | networking/network_client_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_write_reliable` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_flush_reliable` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_connection_datagram_size` | networking/network_connection.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_client_send` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_client_send_reliably` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_distributed_client` | networking/network_game_globals.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_player_slot_reusable` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_player_slot_held` | networking/network_game_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_machine_has_players` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_refuse_late_joiner` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_same_player` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_player_among` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_start_late_joiner` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_restore_held_players` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_discard_held_players` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_queue_held_players` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_keep_late_joiners_alive` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_late_joiner_loaded` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_drop_client_machine` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_client_machine_heard` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_client_machine_is_local` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_countdown_started` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_setup_game_from_playlist` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_add_new_client` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_public_endpoint` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_client_machines` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_idle_postgame_tasks` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_have_all_machines_have_precached` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_idle_pregame_tasks` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_remove_player_request_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_settings_request` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_player_settings_request` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_game_start_request` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_graceful_game_exit_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_map_is_precached_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_loaded` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_add_player_request_ingame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_remove_player_request_ingame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_remove_player_request_postgame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_switch_to_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_client_disconnect` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `process_telnet_client_buffer` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |

## `objects`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `lightning_render` | objects/widgets/lightning.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `damage_render_debug` | objects/damage.c | Replaced | dev/src/draw.rs | the editor and tools |
| `object_initialize_vitality` | objects/damage.c | Matched | sim/src/damage.rs:113 | campaign/tests/unit_vitality_reference.rs (trace) |
| `object_get_maximum_body_vitality` | objects/damage.c | Matched | sim/src/damage.rs | sim/src/damage.rs (measure) |
| `object_get_maximum_shield_vitality` | objects/damage.c | Matched | sim/src/damage.rs | sim/src/damage.rs (measure) |
| `damage_data_new` | objects/damage.c | Matched | sim/src/damage.rs | sim/tests/cause_damage_reference.rs (trace) |
| `object_can_take_damage` | objects/damage.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `object_cannot_take_damage` | objects/damage.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `object_set_ranged_attack_inhibited` | objects/damage.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `object_set_melee_attack_inhibited` | objects/damage.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `object_deplete_shield` | objects/damage.c | Matched | sim/src/damage.rs:94 | campaign/tests/unit_vitality_reference.rs (trace) |
| `object_damage_shield` | objects/damage.c | Partial | sim/src/damage.rs:259 | sim/tests/cause_damage_reference.rs (trace); leaves out shield over-charge and the current and recent shield damage (coverage_units), the umbrella shield and the network (coverage_units) |
| `object_damage_aftermath` | objects/damage.c | Matched | sim/src/forces.rs:2 | sim/tests/forces.rs (measure) |
| `render_debug_object_damage` | objects/damage.c | Replaced | dev/src/draw.rs | the editor and tools |
| `object_deplete_body` | objects/damage.c | Matched | sim/src/damage.rs:101 | campaign/tests/unit_vitality_reference.rs (trace) |
| `object_destroy` | objects/damage.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `object_damage_body` | objects/damage.c | Partial | sim/src/damage.rs:259 | sim/tests/cause_damage_reference.rs (trace); leaves out region byte damage and the current and recent body damage (coverage_units), the friendly difficulty divide, game-engine multipliers and deathless players (coverage_units) |
| `object_cause_damage` | objects/damage.c | Partial | sim/src/damage.rs:258 | sim/tests/cause_damage_reference.rs (trace); leaves out shield over-charge, the friendly difficulty divide and the player team's falling ignore-difficulty (coverage_units), negligible damage, the current and recent damage and the region byte damage: the engine keeps body points (coverage_units), damage to the dead: the engine stops where the game goes on (coverage_units), effects, deathless players, the body destroyed threshold and only-hurt-while-occupied (coverage_units), vehicles, children, the umbrella shield, AI adjustment, game-engine multipliers, network and player effects (coverage_units) |
| `object_damage_update` | objects/damage.c | Matched | sim/src/damage.rs | sim/src/damage.rs (measure) |
| `area_of_effect_cause_damage_to_object` | objects/damage.c | Matched | sim/src/damage.rs:5 | sim/src/damage.rs (measure) |
| `area_of_effect_cause_damage` | objects/damage.c | Matched | sim/src/damage.rs | sim/tests/forces.rs (measure) |
| `texture_cache_bitmap_load` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `shade_scalar` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `shade_vector2d` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `shade_vector3d` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `sample_lightmap` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `sample_diffuse_texture` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `should_render_lights` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_initialize` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_dispose` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_initialize_for_new_map` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_dispose_from_old_map` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_enable` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_delete` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_build_cluster_array` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `object_get_self_illumination` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_particle` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_disconnect_from_map` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_disconnect_from_structure_bsp` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_attenuation` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `brighten_real_rgb_color` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `cluster_get_first_light` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `cluster_get_next_light` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_marker_begin` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_unmarked` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_mark` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_marker_end` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `render_debug_light` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_queue_lens_flare` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `find_point_lights_for_object_in_cluster` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `build_distant_lights` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_compute_bounding_sphere` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_get_bounding_sphere` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_render_diffuse` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_render_specular` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_illumination_at_point` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_prepare_for_object_dynamic` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_distant_lighting_at_point` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_reconnect_to_map` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_reconnect_to_structure_bsp` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_new` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `light_new_unattached` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_preprocess_scene` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `lights_prepare_for_object_static` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `object_shadow_get_object` | objects/object_shadows.c | Replaced | app/Sources/OpenCEEngineRender/ShadowPass.swift | the enhanced lighting and Metal renderer |
| `object_build_shadow_recursive` | objects/object_shadows.c | Replaced | app/Sources/OpenCEEngineRender/ShadowPass.swift | the enhanced lighting and Metal renderer |
| `object_build_shadow` | objects/object_shadows.c | Replaced | app/Sources/OpenCEEngineRender/ShadowPass.swift | the enhanced lighting and Metal renderer |
| `object_type_render_debug` | objects/object_types.c | Replaced | dev/src/draw.rs | the editor and tools |
| `object_types_disconnect_from_structure_bsp` | objects/object_types.c | Matched | sim/src/objects.rs:469 | sim/src/objects.rs (measure) |
| `object_types_place_objects` | objects/object_types.c | Matched | sim/src/objects.rs:314 | sim/src/objects.rs (measure) |
| `object_types_place_all` | objects/object_types.c | Matched | sim/src/objects.rs:381 | sim/src/objects.rs (measure) |
| `object_names_postprocess` | objects/object_types.c | Ported | content/src/campaign.rs:16 | `tools/engine_object_names_reference.py` |
| `scenery_place` | objects/scenery.c | Matched | sim/src/scenery.rs | sim/src/objects.rs (measure) |
| `scenery_new` | objects/scenery.c | Matched | sim/src/animation.rs:1368 | sim/src/animation.rs (measure) |
| `scenery_update` | objects/scenery.c | Matched | sim/src/scenery.rs | sim/src/animation.rs (measure) |
| `scenery_animation_start_private` | objects/scenery.c | Matched | campaign/src/units.rs | campaign/tests/cutscenes.rs (measure) |
| `scenery_animation_start` | objects/scenery.c | Matched | campaign/src/units.rs | campaign/tests/cutscenes.rs (measure) |
| `scenery_animation_start_at_frame` | objects/scenery.c | Matched | campaign/src/units.rs | campaign/tests/cutscenes.rs (measure) |
| `damage_set_network_state` | objects/damage.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `damage_replay_player_effect` | objects/damage.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `damage_replay_aftermath` | objects/damage.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `damage_replay_kill` | objects/damage.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `damage_replay_static` | objects/damage.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `damage_get_network_state` | objects/damage.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `find_point_light_for_object` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `object_add_scenario_permutation` | objects/objects.c | Matched | sim/src/objects.rs | sim/src/objects.rs (measure) |
| `garbage_collect_now` | objects/objects.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `object_pvs_set_object` | objects/objects.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `object_pvs_set_camera_point` | objects/objects.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `object_pvs_clear` | objects/objects.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `object_beautify` | objects/objects.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `object_header_new` | objects/objects.c | Ported |  | `tools/engine_data_reference.py` |
| `object_header_delete` | objects/objects.c | Ported |  | `tools/engine_data_reference.py` |
| `object_scripting_set_collideable` | objects/objects.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `objects_place` | objects/objects.c | Matched | sim/src/objects.rs | sim/src/objects.rs (measure) |
| `object_start_interpolation` | objects/objects.c | Matched | campaign/src/units.rs:57 | campaign/tests/cutscenes.rs (measure) |
| `object_get_function_value` | objects/objects.c | Matched | ffi/src/particles/contrails.rs:56 | ffi/src/particles/contrails.rs (measure) |
| `object_index_from_name_index` | objects/objects.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `objects_disconnect_from_structure_bsp` | objects/objects.c | Matched | sim/src/objects.rs | sim/src/objects.rs (measure) |
| `object_pvs_activate` | objects/objects.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `objects_scripting_set_scale` | objects/objects.c | Partial | campaign/src/objects.rs | campaign/tests/objects.rs (measure); leaves out the frames asked for: the scale is taken at once |
| `object_delete` | objects/objects.c | Matched | campaign/src/lib.rs:265 | campaign/tests/objects.rs (measure) |
| `object_detach` | objects/objects.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `objects_reconnect_to_structure_bsp` | objects/objects.c | Matched | sim/src/objects.rs | sim/src/objects.rs (measure) |
| `objects_paparazzi` | objects/objects.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |
| `object_export_function_values` | objects/objects.c | Matched | sim/src/functions.rs:228 | sim/src/functions.rs (measure) |
| `object_compute_node_matrices` | objects/objects.c | Partial | sim/src/player.rs:811 | sim/tests/animation_reference.rs (measure); leaves out its bounding sphere's centre for objects other than bipeds |
| `object_choose_random_change_colors` | objects/objects.c | Matched | ffi/src/lib.rs:511 | ffi/src/lib.rs (measure) |
| `objects_scripting_detach` | objects/objects.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `object_render_debug` | objects/objects.c | Replaced | dev/src/draw.rs | the editor and tools |
| `object_set_position` | objects/objects.c | Matched | campaign/src/objects.rs | campaign/tests/cutscenes.rs (measure) |
| `object_new` | objects/objects.c | Matched | ffi/src/effects.rs:117 | sim/src/animation.rs (measure) |
| `object_new_from_scenario` | objects/objects.c | Matched | sim/src/objects.rs:362 | sim/src/objects.rs (measure) |
| `object_attach_to_marker` | objects/objects.c | Partial | campaign/src/objects.rs | campaign/tests/objects.rs (measure); leaves out the parent's markers: a child is carried at its parent's origin |
| `object_new_by_name` | objects/objects.c | Matched | import/src/level.rs:1097 | campaign/tests/objects.rs (measure) |
| `objects_scripting_attach` | objects/objects.c | Partial | campaign/src/objects.rs | campaign/tests/objects.rs (measure); leaves out the parent's and child's markers: a child is carried at its parent's origin |
| `object_delete_immediately` | objects/objects.c | Matched | sim/src/objects.rs | sim/src/objects.rs (measure) |
| `object_name_list_lookup` | objects/objects.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `object_choose_random_region_permutations` | objects/objects.c | Matched | sim/src/objects.rs | sim/src/objects.rs (measure) |
| `object_delete_recursive` | objects/objects.c | Partial | sim/src/objects.rs | sim/src/objects.rs (measure); leaves out its children's deletion: the engine's attached objects are kept by the campaign, which drops them when either goes |
| `object_compute_function_values` | objects/objects.c | Matched | sim/src/functions.rs:1 | sim/src/functions.rs (measure) |

226 Missing (221 reached from `main_loop`), by file:

- objects/damage.c: `damage_initialize`, `damage_dispose`, `damage_initialize_for_new_map`, `damage_dispose_from_old_map`, `object_restore_body`, `object_double_charge_shield`, `object_destroy_notify_children`, `get_player_index_from_object_or_parents`, `damage_effect_new_on_object`, `object_permutation_shield_regions`, `object_get_actual_body_vitality`, `object_get_actual_shield_vitality`, `damage_effect_new_at_location`, `object_destroy_region`, `damage_kill_object_for_player`
- objects/object_deleted_procs.c: `object_deleted_procs_call`
- objects/object_types.c: `object_type_definition_get`, `object_type_get_datum_size`, `object_type_get_name`, `object_types_initialize`, `object_types_dispose`, `object_types_initialize_for_new_map`, `object_types_dispose_from_old_map`, `object_type_adjust_placement`, `object_type_new`, `object_type_place`, `object_type_delete`, `object_type_update`, `object_type_export_function_values`, `object_type_handle_deleted_object`, `object_type_handle_region_destroyed`, `object_type_handle_parent_destroyed`, `object_type_preprocess_node_orientations`, `object_type_postprocess_node_matrices`, `object_type_reset`, `object_type_disconnect_from_structure_bsp`, `object_type_notify_impulse_sound`, `object_definition_index_to_object_type`, `scenario_get_object_type_scenario_datums`, `scenario_get_object_type_scenario_palette`, `object_types_reconnect_to_structure_bsp`, `object_type_synchronize`
- objects/objects.c: `cluster_get_first_noncollideable_object`, `cluster_get_next_noncollideable_object`, `cluster_get_first_collideable_object`, `cluster_get_next_collideable_object`, `object_get_next_cluster`, `object_try_and_get_and_verify_type`, `object_get_and_verify_type`, `object_iterator_new`, `object_iterator_next`, `object_get_ultimate_parent`, `object_set_object_index_for_name_index`, `objects_fix_for_deleted_object`, `object_set_garbage`, `objects_information_get`, `objects_get_activating_cluster_index`, `object_definition_predict`, `object_predict`, `object_header_block_get`, `object_header_block_allocate`, `object_marker_begin`, `object_marker_end`, `object_unmarked_function`, `object_mark_function`, `find_objects_from_point_vector`, `objects_dump_memory`, `objects_initialize`, `objects_initialize_for_new_map`, `objects_dispose_from_old_map`, `objects_dispose`, `object_activate`, `object_deactivate`, `object_reset`, `object_placement_data_new`, `object_disconnect_from_map`, `object_get_first_cluster`, `object_get_node_matrices`, `object_get_attachment_marker_name`, `object_has_node`, `object_set_automatic_deactivation`, `object_set_visibility`, `object_get_velocities`, `object_get_location`, `object_offset_interpolation`, `object_permute_region`, `objects_in_clusters_by_indices`, `object_visible_to_any_player`, `object_delete_initial_recursive`, `object_reconnect_to_map`, `object_get_node_matrix`, `object_get_marker_by_name`, `object_align_marker_to_matrix`, `object_get_origin`, `object_get_orientation`, `object_get_world_matrix`, `object_inverse_kinematics`, `objects_in_sphere`, `object_postprocess_node_matrices`, `attachments_new`, `object_translate`, `object_attach_to_node`, `object_force_inside_bsp`, `object_update`, `object_compute_node_matrices_recursive`, `active_garbage_limit`, `objects_garbage_collection`, `objects_update`, `objects_memory_compact`, `object_connect_lights`, `object_name_list_allocate`, `object_name_list_free`, `object_name_list_clear`, `object_name_list_new`, `object_claim_scenario_name`, `object_name_list_delete`, `recursive_object_adder`, `sort_dumps`, `object_add_to_dump`, `object_dump_write`, `object_find_region_permutations_available_with_variant`, `object_select_random_region_permutations_by_variant`, `object_determine_variant_number`, `object_add_to_list`, `object_remove_from_list`, `attachments_delete`, `object_create_attachments`, `object_delete_attachments`, `object_compute_change_colors`
- objects/placeholder_definitions.c: `placeholder_initialize`, `placeholder_initialize_for_new_map`, `placeholder_dispose_from_old_map`, `placeholder_dispose`, `placeholder_place`, `placeholder_new`, `placeholder_delete`
- objects/scenery.c: `scenery_initialize`, `scenery_initialize_for_new_map`, `scenery_dispose_from_old_map`, `scenery_dispose`, `scenery_delete`, `scenery_get_animation_time`
- objects/widgets/antenna.c: `antennas_initialize`, `antennas_initialize_for_new_map`, `antennas_dispose_from_old_map`, `antennas_dispose`, `antenna_new`, `antenna_delete`, `antenna_update_attachment`, `antenna_render_proper`, `antenna_update`, `antenna_render`, `antennas_update`, `antenna_record_shape`
- objects/widgets/flags.c: `flag_datum_get_vertex`, `flag_datum_get_cell`, `flags_initialize`, `flags_initialize_for_new_map`, `flags_dispose_from_old_map`, `flags_dispose`, `flag_tesselate_region`, `flag_delete`, `flag_update_attachment`, `flag_set_attachment_shape`, `flag_set_trailing_shape`, `flag_update`, `flag_render_proper`, `flag_new`, `flag_render`, `flags_update`
- objects/widgets/glow.c: `glow_dispose`, `glow_initialize_for_new_map`, `glow_dispose_from_old_map`, `glow_new`, `glow_delete`, `glow_trailing_particle_update_color`, `glow_trailing_particle_update_size`, `glow_trailing_particle_update_velocity`, `glow_trailing_particle_update_position`, `glow_trailing_particle_age`, `glow_normal_particle_update_color`, `glow_normal_particle_update_size`, `glow_particle_new`, `point_from_parametric_line`, `glow_render`, `nonuniform_cubic_spline`, `nonuniform_cubic_spline_vector3d`, `glow_initialize`, `glow_normal_particle_new`, `get_particle_world_position`, `glow_normal_particle_update_position`, `glow_particles_initialize`, `glow_trailing_particle_new`, `glow_update`, `glow_submit`, `glow_time_wrap_down`, `glow_time_wrap_up`
- objects/widgets/light_volumes.c: `light_volumes_initialize`, `light_volumes_dispose`, `light_volumes_initialize_for_new_map`, `light_volumes_dispose_from_old_map`, `light_volume_new`, `light_volume_delete`, `light_volume_interpolate_frames`, `pow1`, `light_volume_render`, `light_volume_submit`
- objects/widgets/lightning.c: `lightnings_initialize`, `lightnings_dispose`, `lightnings_initialize_for_new_map`, `lightnings_dispose_from_old_map`, `lightning_new`, `lightning_delete`, `lightning_offset_marker_position`, `lightning_submit`
- objects/widgets/widgets.c: `tag_group_to_widget_type`, `widget_type_definition_get`, `widgets_initialize`, `widgets_initialize_for_new_map`, `widgets_dispose_from_old_map`, `widgets_dispose`, `widgets_new`, `widgets_delete`, `widgets_need_lighting`, `widgets_render`, `widgets_update`

## `physics`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `bsp2d_test_point` | physics/bsp2d.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `bsp3d_test_point` | physics/bsp3d.c | Matched | content/src/level.rs:908 | physics/tests/collision_reference.rs (trace) |
| `bsp3d_clip_line_to_leaves` | physics/bsp3d.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `bsp3d_clip_polygon_to_leaves_recursive` | physics/bsp3d.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `bsp3d_clip_polygon_to_leaves` | physics/bsp3d.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_edge_count` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_polygon` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `add_feature` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `render_debug_collision_vertex` | physics/collision_bsp.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_edge` | physics/collision_bsp.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_surface` | physics/collision_bsp.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_bsp` | physics/collision_bsp.c | Replaced | dev/src/draw.rs | the editor and tools |
| `collision_edge_length` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_perimeter` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_area` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_project_point2d` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_test_point2d` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_find_closest_point2d` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_test_line2d` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_test_sphere` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_bsp_valid_surface_index` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_bsp3d_enter_node` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_test_point` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `bsp2d_test_sphere_recursive` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `sphere_test_vector` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_bsp_test_pill_new_recursive` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_bsp_test_pill_new` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `bsp3d_test_sphere_recursive` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_leaf_test_vector` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `pill_test_vector` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_bsp_test_sphere` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_bsp_test_vector` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_bsp_test_vector_recursive` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_surface_test_pill` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_bsp_test_pill` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `collision_debug_render` | physics/collision_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `collision_log_compare_overall_usage` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_initialize` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_enable` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_store_period` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_begin_period` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_continue_period` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_end_period` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_format_usage` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_render` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_get_current_user` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_start_time` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_end_time` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_usage` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_display` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_test_sphere` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ |
| `object_test_point` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/world.rs (measure); leaves out the original's own pill, sphere and point solver: the engine's physics world (physics/src/lib.rs) moves and tests them |
| `collision_test_point` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ |
| `object_test_vector` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/world.rs (measure); leaves out the original's own pill, sphere and point solver: the engine's physics world (physics/src/lib.rs) moves and tests them |
| `collision_test_vector` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ |
| `collision_test_pill` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ, the original's pill_test_vector (collision_bsp.c) takes the negative of the half-b term, so its roots are for moving backward; the trace keeps the original's answers and the engine agrees away from edges, to be checked against the xbe |
| `object_get_features_in_sphere` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/world.rs (measure); leaves out the original's own pill, sphere and point solver: the engine's physics world (physics/src/lib.rs) moves and tests them |
| `collision_get_features_in_sphere` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ, which surfaces, edges and vertices are listed: the trace checks only whether a sphere touches |
| `collision_fix_pill_nudge_collision` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/movement.rs (measure); leaves out the original's own pill, sphere and point solver: the engine's physics world (physics/src/lib.rs) moves and tests them |
| `collision_fix_pill` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/movement.rs (measure); leaves out the original's own pill, sphere and point solver: the engine's physics world (physics/src/lib.rs) moves and tests them |
| `collision_move_point` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/world.rs (measure); leaves out the original's own pill, sphere and point solver: the engine's physics world (physics/src/lib.rs) moves and tests them |
| `collision_move_pill` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/movement.rs (measure); leaves out the original's own pill, sphere and point solver: the engine's physics world (physics/src/lib.rs) moves and tests them |
| `collision_move_sphere` | physics/collisions.c | Partial | physics/src/lib.rs | physics/tests/world.rs (measure); leaves out the original's own pill, sphere and point solver: the engine's physics world (physics/src/lib.rs) moves and tests them |
| `pin_fraction` | physics/physics.c | Matched | sim/src/vehicle.rs:713 | sim/tests/physics_math_reference.rs (trace) |
| `physics_get_features_in_sphere` | physics/physics.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ, which surfaces, edges and vertices are listed: the trace checks only whether a sphere touches |
| `get_material_type` | physics/physics.c | Matched | physics/src/ground.rs:3 | physics/src/ground.rs (measure) |
| `render_debug_physics` | physics/physics.c | Replaced | dev/src/draw.rs | the editor and tools |
| `physics_instance_new` | physics/physics.c | Replaced | physics/src/lib.rs | a library does the job; the same rigid bodies falling, rolling and coming to rest: Jolt integrates the dynamic bodies (physics/src/lib.rs) with the masses and shapes of the game's physics tags |
| `physics_test_point` | physics/physics.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ |
| `physics_test_vector` | physics/physics.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ |
| `friction_evaluate` | physics/physics.c | Matched | sim/src/vehicle.rs:1543 | sim/tests/physics_math_reference.rs (trace) |
| `physics_compute_new` | physics/physics.c | Replaced | physics/src/lib.rs | a library does the job; the same rigid bodies falling, rolling and coming to rest: Jolt integrates the dynamic bodies (physics/src/lib.rs) with the masses and shapes of the game's physics tags |
| `physics_compute_biped_collision` | physics/physics.c | Matched | sim/src/seats.rs:344 | sim/tests/forces.rs (measure) |
| `physics_compute_vehicle_collision` | physics/physics.c | Partial | physics/src/lib.rs | sim/tests/world_collision.rs (measure); leaves out the vehicle's collision model: its mass points stand for it |
| `physics_compute_unit_collisions` | physics/physics.c | Partial | physics/src/lib.rs | sim/tests/world_collision.rs (measure); leaves out unit against unit on foot: only vehicles strike bipeds |
| `physics_update_new` | physics/physics.c | Replaced | physics/src/lib.rs | a library does the job; the same rigid bodies falling, rolling and coming to rest: Jolt integrates the dynamic bodies (physics/src/lib.rs) with the masses and shapes of the game's physics tags |
| `physics_update` | physics/physics.c | Replaced | physics/src/lib.rs | a library does the job; the same rigid bodies falling, rolling and coming to rest: Jolt integrates the dynamic bodies (physics/src/lib.rs) with the masses and shapes of the game's physics tags |
| `physics_variable_position_update` | physics/physics_variables.c | Matched | sim/src/physics_variables.rs:89 | sim/tests/physics_variable_reference.rs (trace) |
| `physics_variable_speed_update` | physics/physics_variables.c | Matched | sim/src/physics_variables.rs:31 | sim/tests/physics_variable_reference.rs (trace); the limits taken as MIN and MAX take them, a speed that is not a number given the limit |
| `physics_variable_speed_update_seek` | physics/physics_variables.c | Matched | sim/src/physics_variables.rs:62 | sim/tests/physics_variable_reference.rs (trace) |
| `physics_variable_update` | physics/physics_variables.c | Matched | sim/src/physics_variables.rs:21 | sim/tests/physics_variable_reference.rs (trace) |
| `physics_variable_position_get_seek_direction` | physics/physics_variables.c | Matched | sim/src/physics_variables.rs:108 | sim/tests/physics_variable_reference.rs (trace) |
| `physics_variable_position_update_seek` | physics/physics_variables.c | Matched | sim/src/physics_variables.rs:121 | sim/tests/physics_variable_reference.rs (trace) |
| `physics_variable_update_seek` | physics/physics_variables.c | Matched | sim/src/physics_variables.rs:148 | sim/tests/physics_variable_reference.rs (trace) |
| `bsp3d_plane_distance_to_point` | physics/bsp3d.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `bsp3d_polygon_plane_distance_to_point` | physics/bsp3d.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `bsp2d_test_pill_recursive` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `bsp3d_test_pill_recursive` | physics/collision_bsp.c | Replaced | physics/src/lib.rs | a library does the job; the same floors, walls and line of sight: Jolt answers the queries (physics/src/lib.rs) on a mesh of the same collision BSP surfaces (physics/src/mesh.rs); the Partial entries of collision_fix_pill and the queries say where an edge can differ |
| `render_debug_collision_features` | physics/collision_features.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_sphere` | physics/collision_features.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_cylinder` | physics/collision_features.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_prism` | physics/collision_features.c | Replaced | dev/src/draw.rs | the editor and tools |
| `collision_model_test_sphere` | physics/collision_models.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ, an object's model BSP: the trace walks a level's collision BSP, which is the same walk |
| `collision_model_test_point` | physics/collision_models.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ, an object's model BSP: the trace walks a level's collision BSP, which is the same walk |
| `collision_model_test_vector` | physics/collision_models.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ, an object's model BSP: the trace walks a level's collision BSP, which is the same walk |
| `collision_model_test_pill` | physics/collision_models.c | Partial | physics/src/lib.rs | physics/tests/collision_reference.rs (trace); leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ, an object's model BSP: the trace walks a level's collision BSP, which is the same walk |
| `render_debug_collision_model` | physics/collision_models.c | Replaced | dev/src/draw.rs | the editor and tools |
| `point_physics_definition_get_mass` | physics/point_physics.c | Matched | sim/src/point_physics.rs:44 | sim/tests/physics_math_reference.rs (trace) |
| `point_physics_update` | physics/point_physics.c | Matched | sim/src/point_physics.rs:1 | sim/src/point_physics.rs (measure) |
| `render_debug_point_physics` | physics/point_physics.c | Replaced | dev/src/draw.rs | the editor and tools |

45 Missing (43 reached from `main_loop`), by file:

- physics/breakable_surfaces.c: `breakable_surface_plane_distance`, `breakable_surface_get`, `breakable_surfaces_initialize`, `breakable_surfaces_dispose`, `breakable_surfaces_initialize_for_new_map`, `breakable_surfaces_dispose_from_old_map`, `breakable_surfaces_enable`, `breakable_surfaces_reset`, `breakable_surface_flags_get`, `breakable_surface_extant`, `breakable_surface_damage`, `breakable_surface_damage_area_of_effect`, `breakable_surface_effect`
- physics/collision_features.c: `collision_features_new`, `collision_features_from_point`, `collision_features_from_line`, `collision_features_from_polygon`, `collision_features_from_vertex`, `collision_features_from_edge`, `collision_features_from_surface`, `collision_bsp_get_features_in_sphere`, `collision_features_test_point`, `collision_features_test_vector`, `collision_sphere_test_point`, `collision_cylinder_test_point`, `collision_prism_test_point`, `collision_sphere_test_vector`, `collision_cylinder_test_vector`, `collision_prism_test_vector`
- physics/collision_models.c: `collision_model_instance_new`, `collision_model_get_features_in_sphere`
- physics/collisions.c: `collision_model_get_material_type`, `collision_test_vector_exit`, `collision_test_pill_new`, `clip_position_to_plane`, `clip_velocity_to_plane`, `clip_position_to_line`, `clip_velocity_to_line`, `collision_cluster_fog_plane`
- physics/physics.c: `compute_ground_plane`, `rotate_vectors3d_by_angular_velocity`, `physics_update_old`
- physics/point_physics.c: `point_physics_initialize_for_new_map`, `point_physics_dispose_from_old_map`, `point_physics_definition_interpolate`

## `rasterizer`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `Direct3D_Release` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3D8_Release` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3D8_CreateDevice` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_BeginScene` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_EndScene` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_GetRenderState` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_GetTextureStageState` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Release` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_GetDeviceCaps` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Present` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_GetBackBuffer` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreateTexture` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreateVolumeTexture` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreateCubeTexture` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreatePalette` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderTarget` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_preinitialize__fill_you_up_with_the_devils_cock` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_GetDepthStencilSurface` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_BeginScene` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_EndScene` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Clear` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetViewport` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_get_bitmap_default_hardware_format` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_GetRenderState` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTexture` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetPalette` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_GetTextureStageState` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShader` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetShaderConstantMode` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetStreamSource` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetIndices` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetPixelShaderProgram` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVerticalBlankCallback` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_reset_state` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_spin_begin` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_spin_end` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_windows_begin` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_window_get_fog` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DTexture_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_windows_end` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_frame_end` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DTexture8_GetSurfaceLevel` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DTexture8_LockRect` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DTexture8_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DVolumeTexture_UnlockBox` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dispose` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_set_vblank_callback` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVolumeTexture8_LockBox` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVolumeTexture8_UnlockBox` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_texture_bitmap_data` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_texture_direct` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DCubeTexture_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_texture_direct_non_blocking` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_texture` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DCubeTexture8_LockRect` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DCubeTexture8_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_texture_non_blocking` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_get_target` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DPalette_Unlock` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DPalette8_Lock` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DPalette8_Unlock` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_vertex_shader` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_pixel_shader` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_model_skinning` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DSurface_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DSurface8_Release` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_model_lighting_point_light` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DSurface8_GetDesc` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DSurface8_LockRect` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DSurface8_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_model_lighting_distant_light` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_model_lighting` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_frustum_z` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_get_stencil_mode` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `SetupSmartStates` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_filthy_bitmap_defaults_initialize` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_initialize` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_frame_begin` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_window_set_fog` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_present` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_framebuffer_blend_function` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_target` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_target_as_texture` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_secondary_render_target_debug` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_stencil_mode` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_window_begin` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_window_end` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `real_alpha_to_pixel32` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_active_camouflage_set_visibility` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_active_camouflage_cache_primary_render_target` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_active_camouflage_draw` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decal_vertices_purge_proc` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decal_vertices_locked_proc` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_update_function_pointers` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_initialize_for_new_map` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_dispose_from_old_map` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_flush` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decal_vertices_begin_update` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decal_vertices_end_update` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decal_vertices_new` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_DrawPrimitive` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decal_vertices_delete` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetStreamSource` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_end` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData4ub` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DVertexBuffer_Unlock` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Release` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Register` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `IDirect3DVertexBuffer8_Lock` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Unlock` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_initialize` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_dispose` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decal_vertices_lock` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decal_vertices_unlock` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_begin` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_draw` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `detail_object_build_vertices` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014b990` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014b9e0` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014ba00` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_detail_objects_initialize` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014bb90` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014bdb0` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014be10` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014be20` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014be30` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_detail_objects_end` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014be50` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014be80` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014be90` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014bea0` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014bec0` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_detail_objects_dispose` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_detail_objects_begin` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_detail_objects_rebuild_vertices` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_detail_objects_draw` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `draw_primitives_vertex_buffer_valid` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `draw_primitives_triangle_buffer_valid` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_geometry_initialize` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_geometry_begin` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_geometry_end` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_triangles_new` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `draw_primitives_dynamic_triangle_count_valid` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `dynamic_vertex_group_get_d3d_vertex_buffer` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_triangles_delete` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_vertices_new` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014ceb0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014cec0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_vertices_get_type` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014cf80` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_vertices_delete` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_dynamic_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_dynamic_triangles_dynamic_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_dynamic_triangles_static_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_dynamic_triangles_static_vertices2` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_static_triangles_dynamic_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_static_triangles_static_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014e180` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014e190` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014e1a0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014e1c0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014e1d0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014e1e0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014e1f0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014e200` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014e220` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_geometry_dispose` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_triangles_lock` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_triangles_unlock` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_vertices_lock` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_vertices_unlock` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_hud_begin` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_hud_end` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_lit_geometry_draw` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_screen_geometry_add_multitexture_params_to_base` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_screen_geometry_draw` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexDataColor` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_unlit_geometry_draw` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `submit_screen_vertex` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_psuedo_dynamic_screen_quad_draw` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTexture` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_lightmap_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_lightmaps_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_diffuse_light_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_diffuse_lights_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_diffuse_textures_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_light_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_lights_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_lightmap_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_lightmaps_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflection_lightmap_mask_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflection_lightmap_mask_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflection_lightmap_masks_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflection_mirrors_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflection_mirrors_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflections_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflections_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_transparent_geometry_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_transparent_geometry_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_lightmaps_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_lightmap_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_lightmap_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_diffuse_lights_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_diffuse_light_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_diffuse_light_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_diffuse_textures_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_diffuse_texture_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_lights_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_spot_light_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_light_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_light_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_lightmaps_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_lightmap_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_specular_lightmap_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflection_lightmap_masks_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflection_lightmap_mask_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflection_mirror_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_reflection_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_transparent_geometry_submit` | rasterizer/xbox/rasterizer_xbox_environment.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `previous_camera_matrix_for_window` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00155180` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_001551d0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_001551f0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_initialize` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_window_begin` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_window_end` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_dispose` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_001554a0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00155500` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_fog_draw` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_fog_end` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_fog_screen_wind_get_vector` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_001556c0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_001556d0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_001556e0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_model_submit` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_model_end` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `set_real_vector4d` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `local_random_boolean` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_is_active` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_wind_update` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_fog_begin` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_fog_screen_begin` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_fog_screen_draw` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_fog_screen_end` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_model_begin` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_error` | rasterizer/xbox/rasterizer_xbox_errors.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreateTexture` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreateVolumeTexture` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreateCubeTexture` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_bitmap_new` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DBaseTexture8_Release` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DTexture_UnlockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DTexture8_LockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DTexture8_UnlockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DVolumeTexture_UnlockBox` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVolumeTexture8_LockBox` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVolumeTexture8_UnlockBox` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DCubeTexture_UnlockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DCubeTexture8_LockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DCubeTexture8_UnlockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_bitmap_2d_changed` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_bitmap_3d_changed` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_bitmap_cm_changed` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_bitmap_delete` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_bitmap_changed` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreateVertexBuffer` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreateIndexBuffer` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00158450` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00158460` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DVertexBuffer_Unlock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Release` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Lock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Unlock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DIndexBuffer_Lock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DIndexBuffer_Unlock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DIndexBuffer8_Release` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DIndexBuffer8_Lock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DIndexBuffer8_Unlock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_vertex_buffer_new` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_vertex_buffer_delete` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_triangle_buffer_new` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_triangle_buffer_delete` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_project_billboard` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData4f` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_sun_glow_copy_source` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_sun_glow_convolve` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_sun_glow_draw` | rasterizer/xbox/rasterizer_xbox_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `set_environment_shader_pixel_shader` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_models_begin` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_model_end` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_models_end` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_model_ambient_reflection_tint` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_model_draw_environment_shader` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_model_begin` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_model_transparent_geometry_submit` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_model_draw` | rasterizer/xbox/rasterizer_xbox_models.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d220` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d3d0` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d420` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d640` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d6a0` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d6b0` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d6d0` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d700` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d710` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015d720` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_hud_motion_sensor_blip_begin` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_hud_motion_sensor_blip_draw` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_hud_motion_sensor_blip_end` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015e2b0` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015e460` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015e4b0` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015e6d0` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015e730` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_plasma_energy_draw` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `profile_assert` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `callback_function` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `frame_callback_function` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_enabled` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_initialize` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_frame_begin` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_window_begin` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_profile_enable` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_begin` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_end` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_get_string` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_query` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_query_pushbuffer` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_frame_end` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_window_end` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_dispose` | rasterizer/xbox/rasterizer_xbox_profile.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `vector2d_reciprocal` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015f8d0` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015f920` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015f9b0` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015fbd0` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015fc30` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015fc40` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015fc60` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015fc70` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0015fc80` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effect_set_texture_transforms` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_screen_effect` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_screen_flash` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_shadows_begin` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_shadow_model_begin` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_shadow_model_end` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_shadow_end` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_shadows_end` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_shadow_convolve` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_shadow_begin` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_shadow_model_draw` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_environment_shadow_draw` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00162ea0` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00163050` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_001630a0` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_001632c0` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00163320` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_end` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00163340` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00163360` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00163370` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00163380` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_begin` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_draw_character` | rasterizer/xbox/rasterizer_xbox_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00163b40` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00163cf0` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_CreateVertexBuffer` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_group_draw__internal` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00163ee0` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00164100` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00164160` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00164170` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_BeginVisibilityTest` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_EndVisibilityTest` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_GetVisibilityTestResult` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DVertexBuffer_Unlock` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Release` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Lock` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Unlock` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_groups_end` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `subtract_vectors4d` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `offset_vector4d` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_initialize_aux_buffer` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_dispose_aux_buffer` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_groups_begin` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_group_draw` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00168070` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_initialize.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00168090` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_initialize.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_vertex_shaders_initialize` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_initialize.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_vertex_shaders_dispose` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_initialize.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00168350` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00168360` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00168370` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00168380` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_vertex_shader_permutation` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_water_set_visibility_for_frame` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_water_set_visibility_for_window` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_water_get_visibility_for_window` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_water_build_bumpmap` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_water_draw` | rasterizer/xbox/rasterizer_xbox_water.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_project_billboard` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_BeginVisibilityTest` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_EndVisibilityTest` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_GetVisibilityTestResult` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData4f` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_SetVertexDataColor` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_end` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_get_occlusion_test_result` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_submit` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_begin` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_set_texture` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_set_tint_factor` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_set_zbuffer_enable` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_draw_sprite2d` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_draw_sprite3d` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_widget_submit_occlusion_test` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_map_verify` | rasterizer/xbox/shader_transparent_chicago_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_transparent_chicago_create` | rasterizer/xbox/shader_transparent_chicago_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_initialize` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_reset_state` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_windows_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_window_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_window_get_fog` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_window_set_fog` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_window_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_windows_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_present` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dispose` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_vblank_callback` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_profile_enable` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_triangles_new` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_triangles_lock` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_triangles_unlock` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_triangles_delete` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_vertices_new` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_vertices_get_type` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_vertices_lock` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_vertices_unlock` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_vertices_delete` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_line` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_triangle` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_begin_screenspace` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_line_screenspace` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_linestrip_screenspace` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_end_screenspace` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_initialize` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_update_function_pointers` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_initialize_for_new_map` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_dispose_from_old_map` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_flush` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_dispose` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decal_vertices_new` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decal_vertices_lock` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decal_vertices_unlock` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decal_vertices_delete` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_detail_objects_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_detail_objects_rebuild_vertices` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_detail_objects_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_detail_objects_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effect` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_flash` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_model_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_model_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_model_transparent_geometry_submit` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_model_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_models_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_lightmaps_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_lightmap_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_lightmap_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_lightmap_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_lightmaps_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_diffuse_lights_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_diffuse_light_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_diffuse_light_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_diffuse_light_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_diffuse_lights_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_shadows_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_shadow_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_shadow_model_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_shadow_model_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_shadow_model_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_shadow_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_shadow_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_shadows_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_diffuse_textures_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_diffuse_texture_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_diffuse_textures_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_lights_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_light_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_light_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_light_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_lights_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_lightmaps_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_lightmap_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_lightmap_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_lightmap_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_specular_lightmaps_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflection_lightmap_masks_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflection_lightmap_mask_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflection_lightmap_mask_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflection_lightmap_mask_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflection_lightmap_masks_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflection_mirrors_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflection_mirror_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflection_mirrors_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflections_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflection_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_reflections_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_transparent_geometry_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_transparent_geometry_submit` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_transparent_geometry_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_wind_get_vector` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_environment_fog_screen_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_hud_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_hud_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_unlit_geometry_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_lit_geometry_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_screen_geometry_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_screen_geometry_add_multitexture_params_to_base` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_psuedo_dynamic_screen_quad_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_submit` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_set_texture` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_set_tint_factor` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_set_zbuffer_enable` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_draw_sprite2d` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_draw_sprite3d` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_submit_occlusion_test` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_get_occlusion_test_result` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_hud_motion_sensor_blip_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_hud_motion_sensor_blip_draw` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_hud_motion_sensor_blip_end` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_point` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_immediate_vector` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_models_begin` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_model_vertices` | rasterizer/rasterizer.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effects_time` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effects_initialize` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effects_initialize_for_new_map` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effects_dispose_from_old_map` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effects_dispose` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_script_screen_effect_set_value` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_script_screen_effect_get_value` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effect_start` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effect_set_convolution` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effect_set_filter` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effect_set_filter_desaturation_tint` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effect_set_video` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effect_stop` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_effect_get_cinematic_parameters` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_set_near_clip_distance` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_get_near_clip_distance` | rasterizer/rasterizer_cinematics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_new_primitive` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_initialize` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_begin` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_end` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_dispose` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_test` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_compare_primitives` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_draw` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_line_shaded` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_triangle_shaded` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_line` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_triangle` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_statistics_initialize` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_statistics_begin` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_statistics_get_fps` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_fps_accumulate` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `eat_my_shorts` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_statistics_count_static_vertices` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_statistics_count_dynamic_vertices` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_statistics_draw` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_statistics_end` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_statistics_dispose` | rasterizer/rasterizer_frame_statistics.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `uncompress_int8_to_real` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `uncompress_int16_to_real` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `uncompress_int32_to_real_vector3d` | rasterizer/rasterizer_geometry.c | Matched | import/src/level.rs:1943 | sim/tests/math_extra_reference.rs (trace) |
| `rasterizer_geometry_get_vertex_size` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_geometry_byte_swap_vertices` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_geometry_uncompress_vertices` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_geometry_stripify` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `environment_vertex_compressed_get_point` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `environment_vertex_compressed_get_normal` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `environment_vertex_compressed_get_texcoord` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `environment_lightmap_vertex_compressed_get_incident_radiosity` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `environment_lightmap_vertex_compressed_get_texcoord` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `compress_real_to_int8` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `compress_real_to_int8_clamp` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `compress_real_to_int16` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `compress_real_to_int16_clamp` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `compress_real_vector3d_to_int32` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `compress_real_vector3d_to_int32_clamp` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_geometry_compress_vertices` | rasterizer/rasterizer_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `screenshot_in_progress` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `lens_flare_parameters_get` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `lens_flare_occlusion_test_results_get` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_lights_reset_for_new_map` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_lights_begin_for_new_frame` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_lights_begin` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_light_submit` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_lights_end` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `lens_flare_evaluate_corona_rotation_function` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_lens_flare_submit` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_lens_flare_submit_for_cluster` | rasterizer/rasterizer_lights.c | Matched | import/src/level.rs:1679 | ffi/src/camera.rs (measure) |
| `rasterizer_lens_flares_submit_occlusion_tests` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_lens_flares_draw` | rasterizer/rasterizer_lights.c | Partial | ffi/src/camera.rs:3 | ffi/src/camera.rs (measure); leaves out the animation of each reflection (lens_flare_reflection.animation_flags) |
| `lock_rasterizer_text_data` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `unlock_rasterizer_text_data` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_cache_initialize` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_set_shadow_color` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_cache_flush` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_cache_dispose` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_character` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_character_with_dropshadow` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_hires_glyph` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_hires_character` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_hires_character_with_dropshadow` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `hardware_character_cache_get_bitmap` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_string` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_unicode_string` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_initialize` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_begin` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_new_group` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_new_group2` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_get_groups2` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_next_group` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_get_groups` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_get_group_from_presorted_index` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_get_group_presorted_index` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_get_group_pending_status` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_set_group_pending_status` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_get_primary_vertex_type` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_end` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_dispose` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_stop` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_sort_internal` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `group_sorted_indices_cmpfn` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_sort_external` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_geometry_draw` | rasterizer/rasterizer_transparent_geometry.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_initialize_for_new_map` | rasterizer/common/rasterizer_common.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dispose_from_old_map` | rasterizer/common/rasterizer_common.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_frame_update` | rasterizer/common/rasterizer_common.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_memory_pool_initialize` | rasterizer/rasterizer_memory_pool.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_memory_pool_begin` | rasterizer/rasterizer_memory_pool.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_memory_alloc` | rasterizer/rasterizer_memory_pool.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_memory_alloc_const` | rasterizer/rasterizer_memory_pool.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_memory_pool_end` | rasterizer/rasterizer_memory_pool.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_memory_pool_dispose` | rasterizer/rasterizer_memory_pool.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_get_max_mipmap_count` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_get_pixel_data_size` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `bitmap_swizzle_vector2d` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_swizzle2d_byte` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_swizzle2d_word` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_swizzle2d_long` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_swizzle3d_byte` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_swizzle3d_word` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_swizzle3d_long` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_swizzle` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_xbox_bitmap_rebuild_hardware_format` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `bitmap_swizzle_vector3d` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `compute_swizzle_masks` | rasterizer/rasterizer_swizzle.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `hardware_character_cache_get_origin` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `hardware_character_padding` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `flush_hardware_character` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `cache_hardware_format_character` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_transparent_encloses` | rasterizer/rasterizer_transparent_enclosure.h | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_screen_width_update` | rasterizer/xbox/rasterizer_xbox.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_drawing_begin` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_debug_drawing_end` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_debug_immediate_begin` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_debug_immediate_end` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_debug_immediate_line` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_debug_immediate_triangle` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_debug_immediate_begin_screenspace` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_debug_immediate_end_screenspace` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_debug_immediate_line_screenspace` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_debug_immediate_linestrip_screenspace` | rasterizer/xbox/rasterizer_xbox_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decal_blend_function_commutes` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_sort_run` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_decals_draw_order` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_shadow_convolve_pass` | rasterizer/xbox/rasterizer_xbox_shadows.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_widget_batch_sprites` | rasterizer/xbox/rasterizer_xbox_widgets.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_stage_color_input` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_stage_color_output` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_stage_color_output_flags` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_stage_alpha_input` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_stage_alpha_output` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_stage_alpha_output_flags` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_stage_verify` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shader_transparent_generic_create` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |

## `render`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `render_effects` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_initialize` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_initialize_for_new_map` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_dispose_from_old_map` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_dispose` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_nonplayer_frame` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_frame_pregame` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_frame_present` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_location_visible` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rendered_cluster_get` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_window` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_player_frame` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_frame` | render/render.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_warn_once` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_new` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_hack_frustum_z` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_build_frustum_bounds` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_frustum_sphere_diameter_in_pixels` | render/render_cameras.c | Matched | ffi/src/render_model/size_reference.rs:3 | ffi/src/render_model/size_reference.rs (trace); the frustum's vertical scale built in the game's order (a double-precision tangent, then its reciprocal across half the height) |
| `render_frustum_cube_view_fraction` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_frustum_get_projection_bounds` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_view_to_screen` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_screen_to_view` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_screen_to_world` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_get_adjusted_field_of_view_tangent` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_build_clipped_frustum_bounds` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_triangle_frontfacing` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_frustum_build_point_flags` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_frustum_triangle_visible` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_frustum_cube_visible` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_frustum_sphere_visible` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_world_to_screen` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_debug_frustum` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_mirror` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_camera_build_frustum` | render/render_cameras.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `contrail_fade` | render/render_contrails.c | Matched | ffi/src/particles/contrails/fade_reference.rs:2 | ffi/src/particles/contrails/fade_reference.rs (trace); a camera on the strip's point is not traced: the game divides by zero |
| `render_contrail` | render/render_contrails.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_contrails` | render/render_contrails.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_contrails_ground_mapped` | render/render_contrails.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_contrails_media_mapped` | render/render_contrails.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_contrails_normal` | render/render_contrails.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_triangle` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_quadrilateral` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_polygon` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_add_cache_string` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_circle_points` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_height_matrix` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_pill_points` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_add_cache_entry` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_point` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_line` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_vector` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_tick` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_line_offset` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_matrix` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_sphere` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_cylinder` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_pill` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_box` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_polygon_edges` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_string` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_string_at_point` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_camera` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_player` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_structure` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_bsp` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_input` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_structure_decals` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_point2d` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_line2d` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_vector2d` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_circle` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_vectors` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_quaternion` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_box2d_outline` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug_box_outline` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_debug` | render/render_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_media_initialize_for_new_map` | render/render_media.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_media_dispose_from_old_map` | render/render_media.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_media_build_render_target` | render/render_media.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_media` | render/render_media.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `object_get_render_bounding_sphere` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_objects_initialize` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_objects_initialize_for_new_map` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_objects_dispose_from_old_map` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_objects_dispose` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `object_is_first_person_camera` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `find_rendered_objects` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `object_get_level_of_detail_pixels` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_object_list` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `interpolate_real_argb_color` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `interpolate_normal` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_object_shadow_begin` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `shadow_volume_plane_pair` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_object_shadow_end` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `object_render_state_refresh` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `object_get_cached_render_state` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_object` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `object_get_cached_render_lighting` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `process_rendered_objects` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017bbc0` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_objects` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_object_shadows` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `local_player_is_first_person` | render/render_particles.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `compare_rendered_particles` | render/render_particles.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_particles` | render/render_particles.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_sky` | render/render_sky.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_transform_origin_and_direction` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_compute_scale` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_prepare_for_window` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_get_group` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprites_begin` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprites_end` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_compute_basis` | render/render_sprite.c | Ported | ffi/src/sprite.rs:1 | `tools/engine_sprite_reference.py` |
| `build_sprite_compute_vertex_fade` | render/render_sprite.c | Ported | ffi/src/sprite.rs:2 | `tools/engine_sprite_reference.py` |
| `build_sprite` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_rotational` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `triangle_strip_iterator_new` | render/triangle_strips.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `triangle_strip_iterator_get_triangle` | render/triangle_strips.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_detail_objects` | render/render_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |

## `saved games`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `game_state_save` | saved games/game_state.c | Partial | campaign/src/checkpoint.rs:61 | campaign/tests/flow.rs (measure); leaves out the memory image: the saved game holds the inputs, replayed from the mission's start |
| `game_state_revert` | saved games/game_state.c | Partial | campaign/src/checkpoint.rs:87 | campaign/tests/flow.rs (measure); leaves out the memory image: the saved game holds the inputs, replayed from the mission's start |
| `game_state_gpu_malloc` | saved games/game_state.c | Replaced |  | through its caller `_rasterizer_decals_initialize` |
| `game_state_lruv_cache_new` | saved games/game_state.c | Replaced |  | through its caller `_rasterizer_decals_initialize` |
| `game_state_initialize` | saved games/game_state.c | Replaced |  | through its caller `shell_initialize` |
| `game_state_allocate_buffer` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_free_buffer` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_create_or_open_file` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_close_file` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_write_to_file` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_read_from_file` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_write_core` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_read_core_header` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_read_core` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_get_persistent_storage_filename` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_get_persistent_storage_path` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `delete_persistent_storage` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_open_persistent_storage` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_read_header_from_persistent_storage` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_write_to_persistent_storage` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_read_from_persistent_storage` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `game_state_create_persistent_storage` | saved games/game_state_xbox.c | Replaced | sim/src/lib.rs | Xbox-only hardware |
| `player_profile_get_random_good_color` | saved games/player_profile.c | Replaced |  | through its caller `get_unique_random_color` |
| `player_profile_get_random_color` | saved games/player_profile.c | Replaced |  | through its caller `get_unique_random_color` |
| `saved_game_file_name_unique` | saved games/saved_game_files.c | Replaced |  | through its caller `virtual_keyboard_select` |
| `game_state_image_contains` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_reference_valid` |
| `game_state_image_pointer` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_data_valid` |
| `game_state_image_refuse` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_accept` |
| `game_state_image_data_valid` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_accept` |
| `game_state_image_reference_valid` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_memory_pool_valid` |
| `game_state_image_memory_pool_valid` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_accept` |
| `game_state_image_lruv_cache_valid` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_accept` |
| `game_state_image_accept` | saved games/game_state.c | Replaced |  | through its caller `game_state_read_core` |

104 Missing (99 reached from `main_loop`), by file:

- saved games/game_state.c: `dummy`, `game_state_call_before_save_procs`, `game_state_call_before_load_procs`, `game_state_call_after_load_procs`, `game_state_dispose`, `game_state_initialize_for_new_map`, `game_state_dispose_from_old_map`, `game_state_save_to_persistent_storage`, `game_state_test_persistent_storage`, `game_state_save_core`, `game_state_reverted`, `game_state_header_valid`, `game_state_allocation_record`, `game_state_note_event`, `game_state_malloc`, `game_state_data_new`, `game_state_memory_pool_new`, `game_state_try_and_load_from_persistent_storage`, `game_state_load_core`, `game_state_set_revert_time`, `game_state_allocation_new`, `game_state_data_arrays_new_map`, `game_state_check_data_arrays`
- saved games/player_profile.c: `player_profiles_dispose`, `player_profiles_enumerate_available_to_local_player_index`, `player_profile_delete`, `player_profile_get_from_path`, `player_profile_number_of_available_primary_colors`, `player_profile_get_rgb_color`, `player_profile_get_highest_completed_solo_level`, `player_profile_get_enclosing_directory_path`, `build_default_profile`, `player_profile_read`, `player_profile_write_thread_proc`, `player_profile_new`, `player_profile_get`, `player_profile_get_argb_color`, `player_profile_create_default_profiles_on_disk`, `player_profile_write`, `player_profiles_initialize`, `player_profile_save`, `player_profile_save_last_level_played`, `player_profile_save_level_completed`
- saved games/playlist_profile.c: `playlist_profiles_initialize`, `playlist_profiles_dispose`, `playlist_profile_new`, `playlist_profile_delete`, `playlist_profile_get_from_path`, `playlist_profile_get_display_name`, `playlist_profile_number_of_default_profiles_on_disk`, `playlist_profile_create_default_profiles_on_disk`, `playlist_profile_read`, `playlist_profile_write_thread_proc`, `playlist_profiles_enumerate_available_to_local_player_index`, `playlist_profile_get`, `playlist_profile_write`, `playlist_profile_save`, `playlist_profile_options_from_block`, `playlist_profile_options_to_block`
- saved games/saved_game_files.c: `saved_game_files_dispose`, `saved_game_file_close`, `saved_game_file_get_type`, `saved_game_files_notify_memory_units_changed`, `saved_game_file_get_useable_untitled_profile_name`, `saved_game_files_take_mutex`, `saved_game_files_release_mutex`, `saved_game_perform_file_system_checks`, `saved_game_file_remember_player1_last_used_profile_directory`, `saved_game_file_retrieve_player1_last_used_profile_directory`, `saved_game_file_remember_last_used_multiplayer_variant_directory`, `saved_game_file_retrieve_last_used_multiplayer_variant_directory`, `saved_game_file_remember_last_used_multiplayer_map`, `saved_game_file_retrieve_last_used_multiplayer_map`, `saved_game_file_generate_checksum`, `find_and_create_directory_if_necessary`, `enumerate_saved_game_files_start`, `enumerate_saved_game_files_end`, `enumerate_saved_game_file`, `enumerate_mapfile_start`, `enumerate_mapfile_end`, `enumerate_saved_game_file_from_mapfile`, `saved_game_files_take_mapfile_mutex`, `saved_game_files_release_mapfile_mutex`, `set_nth_entry_in_mapfile`, `enumerate_memory_units`, `saved_game_files_initialize`, `saved_game_file_find_profile_index_for_directory_path`, `enumerate_default_playlist_profiles`, `enumerate_default_player_profiles`, `count_enumerated_profiles_in_mapfile`, `build_saved_game_file_index`, `append_entry_to_mapfile`, `remove_nth_entry_in_mapfile`, `saved_game_file_get_display_name`, `delete_enumerated_saved_game_file`, `saved_game_file_open`, `synchronize_metadata_display_name_with_profile_name`, `saved_game_file_get_path_to_enclosing_directory`, `enumerate_default_profiles`, `saved_game_files_delete_all_custom_profiles`, `get_nth_entry_in_mapfile`, `saved_game_files_enumerate_available_to_local_player_index`, `create_enumerated_saved_game_file`, `enumerate_memory_units_test`

## `scenario`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `scenario_unload` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `global_scenario_get` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `global_scenario_try_and_get` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `global_structure_bsp_get` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `global_collision_bsp_get` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `global_bsp3d_get` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_get_game_globals` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `global_structure_bsp_tag_index_get` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `default_material_definition_get` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_material_definition_get` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_fog_at_point` | scenario/scenario.c | Replaced | ffi/src/camera.rs | the enhanced lighting and Metal renderer |
| `scenario_illumination_at_point` | scenario/scenario.c | Replaced | ffi/src/camera.rs | the enhanced lighting and Metal renderer |
| `scenario_get_sky_definition_index` | scenario/scenario.c | Replaced |  | through its caller `scenario_get_sky` |
| `scenario_get_sky` | scenario/scenario.c | Replaced |  | through its caller `render_sky` |
| `scenario_fog_region_get_fog_index` | scenario/scenario.c | Replaced | ffi/src/camera.rs | the enhanced lighting and Metal renderer |
| `scenario_switch_structure_bsp` | scenario/scenario.c | Matched | sim/src/lib.rs | sim/src/objects.rs (measure) |
| `scenario_reload_structure_bsp_if_necessary` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_get_structure_reference_index_from_tag_index` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_trigger_volume_test_point` | scenario/scenario.c | Matched | content/src/campaign.rs:143 | content/tests/scenario_reference.rs (trace) |
| `scenario_trigger_volume_test_object` | scenario/scenario.c | Matched | campaign/src/objects.rs | campaign/tests/objects.rs (measure) |
| `scenario_debug_to_file` | scenario/scenario.c | Replaced |  | through its caller `_texture_cache_bitmap_get_hardware_format` |
| `global_structure_bsp_index_get` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_load` | scenario/scenario.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `scenario_get_fog_region_index` | scenario/scenario.c | Replaced | ffi/src/camera.rs | the enhanced lighting and Metal renderer |
| `scenario_location_water_depth` | scenario/scenario.c | Matched | content/src/level.rs:752 | content/src/level.rs (measure) |
| `scenario_get_sound_environment` | scenario/scenario.c | Matched | ffi/src/sound_world/sound_environment_reference.rs:5 | ffi/src/sound_world/sound_environment_reference.rs (trace); one local player; the scenario's fog regions and water boundary are not in the engine, and the game glides once a frame where the engine glides once a tick (the same at 30 frames a second) |
| `interpolate_real_rgb_color` | scenario/scenario.c | Replaced |  | through its caller `object_render_state_refresh` |
| `scenario_get_atmospheric_fog` | scenario/scenario.c | Replaced |  | through its caller `render_player_frame` |
| `wind_dispose_from_old_map` | scenario/wind.c | Matched | ffi/src/particles/wind/wind_reference.rs:7 | ffi/src/particles/wind/wind_reference.rs (trace) |
| `wind_variance_get` | scenario/wind.c | Matched | ffi/src/particles/wind/wind_reference.rs:2 | ffi/src/particles/wind/wind_reference.rs (trace) |
| `wind_update` | scenario/wind.c | Matched | ffi/src/particles/wind/wind_reference.rs:2 | ffi/src/particles/wind/wind_reference.rs (trace) |
| `scenario_get_current_from_weather_palette` | scenario/wind.c | Matched | ffi/src/particles/wind/wind_reference.rs:3 | ffi/src/particles/wind/wind_reference.rs (trace) |
| `wind_variance_initialize` | scenario/wind.c | Matched | ffi/src/particles/wind/wind_reference.rs:2 | ffi/src/particles/wind/wind_reference.rs (trace) |
| `wind_initialize_for_new_map` | scenario/wind.c | Matched | ffi/src/particles/wind/wind_reference.rs:8 | ffi/src/particles/wind/wind_reference.rs (trace); its zeroed globals and its noise are the trace's start |
| `scenario_get_current` | scenario/wind.c | Partial | import/src/level/wind_weather_reference.rs:2 | import/src/level/wind_weather_reference.rs (trace); leaves out the water's current (the force-water flag, or no flags in water), which nothing in the engine asks for, a fog plane's region's weather below a plane of mist: the engine keeps the cluster's |
| `scenario_get_wind` | scenario/wind.c | Partial | import/src/level/wind_weather_reference.rs:2 | import/src/level/wind_weather_reference.rs (trace); leaves out a fog plane's region's weather below a plane of mist: the engine keeps the cluster's |

21 Missing (17 reached from `main_loop`), by file:

- scenario/multiplayer_scenario_description.c: `multiplayer_scenario_description_get_list`, `map_name_from_multiplayer_scenario_description_item`
- scenario/scenario.c: `scenario_call_disconnect_from_structure_bsp_procs`, `scenario_call_reconnect_to_structure_bsp_procs`, `scenario_initialize`, `scenario_initialize_for_new_map`, `scenario_dispose_from_old_map`, `scenario_frame_update`, `scenario_location_award_bonus`, `scenario_location_deafening`, `scenario_leaf_index_from_point`, `scenario_test_pvs`, `scenario_test_pas`, `scenario_location_potentially_visible_local`, `scenario_location_potentially_visible`, `scenario_object_name_index_from_string`, `scenario_location_from_point`, `scenario_location_from_line`, `scenario_ensure_point_within_world`, `scenario_location_underwater`
- scenario/wind.c: `scenario_get_water_current`

## `shaders`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `shader_get_and_verify_type` | shaders/shader_definitions.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_get_vertex_shader_permutation` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_is_mirror` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_is_destructable` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_is_double_sided` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_is_decal` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_is_water_decal` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_ignores_effect` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_type_is_transparent` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_type_is_lightmapped` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_type_is_vertex_lit` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_type_is_valid_for_environment` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_type_is_valid_for_model` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_type_is_valid_for_modifier` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_environment_texture_animation_evaluate` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `numeric_countdown_timer_set` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `numeric_countdown_timer_get` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `numeric_countdown_timer_stop` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `numeric_countdown_timer_restart` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `numeric_countdown_timer_update` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |
| `shader_texture_animation_evaluate` | shaders/shaders.c | Replaced | app/Sources/OpenCEEngineRender/ShaderSources.swift | the enhanced lighting and Metal renderer |

## `shell`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `shell_platform_initialize` | shell/shell_xbox.c | Replaced | app/Sources/OpenCEEngine/main.swift | Xbox-only hardware |
| `shell_platform_verify` | shell/shell_xbox.c | Replaced | app/Sources/OpenCEEngine/main.swift | Xbox-only hardware |
| `shell_platform_dispose` | shell/shell_xbox.c | Replaced | app/Sources/OpenCEEngine/main.swift | Xbox-only hardware |
| `shell_screen_pause` | shell/shell_xbox.c | Replaced | app/Sources/OpenCEEngine/main.swift | Xbox-only hardware |
| `shell_get_command_line` | shell/shell_xbox.c | Replaced | app/Sources/OpenCEEngine/main.swift | Xbox-only hardware |
| `shell_idle` | shell/shell_xbox.c | Replaced | app/Sources/OpenCEEngine/main.swift | Xbox-only hardware |
| `fuck_code_in_the_eye` | shell/shell_xbox.c | Replaced | app/Sources/OpenCEEngine/main.swift | Xbox-only hardware |
| `main` | shell/shell_xbox.c | Replaced | app/Sources/OpenCEEngine/main.swift | Xbox-only hardware |
| `shell_initialize` | shell/shell.c | Replaced |  | through its caller `main` |
| `shell_dispose` | shell/shell.c | Replaced |  | through its caller `main` |
| `shell_running_import_tool` | shell/shell.c | Replaced | dev/src/logbook.rs | the editor and tools; a debug check, statistic or editor hook of the original; the dev tools stand in |

2 Missing (1 reached from `main_loop`), by file:

- shell/shell.c: `shell_application_is_paused`, `shell_application_pause`

## `sound`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `sound_file_is_aiff` | sound/sound_import/sound_aiff.c | Replaced | import/src/lib.rs | the editor and tools |
| `sound_file_aiff_info_get` | sound/sound_import/sound_aiff.c | Replaced | import/src/lib.rs | the editor and tools |
| `sound_file_aiff_raw_data_get` | sound/sound_import/sound_aiff.c | Replaced | import/src/lib.rs | the editor and tools |
| `sound_file_aiff_format` | sound/sound_import/sound_aiff.c | Replaced | import/src/lib.rs | the editor and tools |
| `sound_file_info_get` | sound/sound_import/sound_import.c | Replaced | import/src/lib.rs | the editor and tools |
| `sound_raw_sample_data_get` | sound/sound_import/sound_import.c | Replaced | import/src/lib.rs | the editor and tools |
| `sound_file_is_wave` | sound/sound_import/sound_wave.c | Replaced | import/src/lib.rs | the editor and tools |
| `sound_file_wave_info_get` | sound/sound_import/sound_wave.c | Replaced | import/src/lib.rs | the editor and tools |
| `sound_file_wave_raw_data_get` | sound/sound_import/sound_wave.c | Replaced | import/src/lib.rs | the editor and tools |
| `sound_file_wave_format` | sound/sound_import/sound_wave.c | Replaced | import/src/lib.rs | the editor and tools |
| `game_looping_sound_new` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | ffi/src/sound_world.rs (measure) |
| `game_looping_sound_delete` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | ffi/src/sound_world.rs (measure) |
| `unattached_impulse_sound_new` | sound/game_sound.c | Matched | ffi/src/sound_events.rs | ffi/src/sound_events.rs (measure) |
| `unspatialized_impulse_sound_new` | sound/game_sound.c | Matched | sim/src/player.rs:1305 | ffi/tests/every_sound.rs (measure) |
| `scripted_sound_time` | sound/game_sound.c | Matched | campaign/src/sounds.rs:181 | campaign/tests/sounds.rs (measure) |
| `scripted_sound_stop` | sound/game_sound.c | Matched | campaign/src/sounds.rs | campaign/tests/sounds.rs (measure) |
| `scripted_looping_sound_set_alternate` | sound/game_sound.c | Matched | campaign/src/sounds.rs | campaign/tests/sounds.rs (measure) |
| `unattached_looping_sound_start` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | ffi/src/sound_world.rs (measure) |
| `unattached_looping_sound_stop` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | ffi/src/sound_world.rs (measure) |
| `scripted_looping_sound_stop_internal` | sound/game_sound.c | Matched | campaign/src/sounds.rs | campaign/tests/sounds.rs (measure) |
| `track_object_impulse_sound` | sound/game_sound.c | Matched | ffi/src/sound_events.rs | ffi/src/sound_events.rs (measure) |
| `obstruction_cache_slot` | sound/game_sound.c | Ported |  | `tools/engine_sound_obstruction_reference.py` |
| `location_potentially_audible` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | ffi/src/sound_world.rs (measure) |
| `update_potentially_audible_looping_sound` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | ffi/src/sound_world.rs (measure) |
| `object_impulse_sound_new` | sound/game_sound.c | Matched | ffi/src/sound_events.rs | ffi/src/sound_events.rs (measure) |
| `scripted_sound_new` | sound/game_sound.c | Matched | campaign/src/sounds.rs | campaign/tests/sounds.rs (measure) |
| `scripted_looping_sound_stop` | sound/game_sound.c | Matched | campaign/src/sounds.rs | campaign/tests/sounds.rs (measure) |
| `game_sound_update` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | ffi/src/sound_world.rs (measure) |
| `compute_sound_obstruction` | sound/game_sound.c | Matched | ffi/src/sound_world/sound_obstruction_reference.rs:3 | ffi/src/sound_world/sound_obstruction_reference.rs (trace); the obstruction cache is the old port's and has nothing to match; a source at the listener in its own cluster is shut away where the game's occlusion is not a number |
| `scripted_looping_sound_start` | sound/game_sound.c | Matched | campaign/src/sounds.rs | campaign/tests/sounds.rs (measure) |
| `compress_ima_adpcm_audio_data` | sound/ima_adpcm.c | Replaced | dev/src/control.rs | the editor and tools |
| `decompress_ima_adpcm_audio_data` | sound/ima_adpcm.c | Matched | audio/src/decode.rs | audio/src/decode.rs (measure) |
| `byte_swap_bungie_ima_adpcm_header` | sound/ima_adpcm.c | Replaced | import/src/lib.rs | maps imported to a typed format; byte order is settled when the map is imported |
| `sound_class_get` | sound/sound_classes.c | Matched | audio/src/classes.rs | audio/tests/sound_class_reference.rs (trace) |
| `sound_classes_initialize` | sound/sound_classes.c | Ported |  | `tools/engine_sound_class_reference.py` |
| `sound_class_datum_get` | sound/sound_classes.c | Matched | audio/src/classes.rs | audio/tests/sound_class_reference.rs (trace) |
| `debug_sound_classes_enable` | sound/sound_classes.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_sound_classes_set_distances` | sound/sound_classes.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_sound_classes_set_wet` | sound/sound_classes.c | Replaced | dev/src/control.rs | the editor and tools |
| `sound_classes_initialize_for_new_map` | sound/sound_classes.c | Ported |  | `tools/engine_sound_class_reference.py` |
| `sound_classes_update` | sound/sound_classes.c | Matched | audio/src/classes.rs:579 | audio/tests/sound_class_reference.rs (trace) |
| `sound_class_get_gain` | sound/sound_classes.c | Matched | audio/src/classes.rs | audio/tests/sound_class_reference.rs (trace) |
| `sound_class_set_gain` | sound/sound_classes.c | Matched | audio/src/classes.rs:4 | audio/tests/sound_class_reference.rs (trace) |
| `sound_definition_get_maximum_distance` | sound/sound_definitions.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `sound_definition_get_minimum_distance` | sound/sound_definitions.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `sound_definition_find_pitch_range_by_pitch` | sound/sound_definitions.c | Ported | audio/src/choice.rs:80 | `tools/engine_sound_choice_reference.py` |
| `try_to_reset_permutations` | sound/sound_definitions.c | Ported | audio/src/choice.rs:134 | `tools/engine_sound_choice_reference.py` |
| `sound_definition_next_permutation` | sound/sound_definitions.c | Ported | audio/src/choice.rs:163 | `tools/engine_sound_choice_reference.py` |
| `sound_is_active` | sound/sound_manager.c | Matched | audio/src/mixer.rs | audio/src/mixer.rs (measure) |
| `sound_scripted_dialog_is_playing` | sound/sound_manager.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `sound_manager_set_sound_environment` | sound/sound_manager.c | Matched | audio/src/mixer.rs | ffi/src/sound_world/sound_environment_reference.rs (trace) |
| `sound_scale_value` | sound/sound_manager.c | Ported | audio/src/choice.rs:50 | `tools/engine_sound_choice_reference.py` |
| `sound_definition_is_playable` | sound/sound_manager.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `listener_get` | sound/sound_manager.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `sound_definition_promote` | sound/sound_manager.c | Matched | audio/src/choice.rs | audio/tests/sound_choice_reference.rs (trace) |
| `sound_manager_master_gain` | sound/sound_manager.c | Matched | audio/src/player.rs:236 | audio/src/player.rs (measure) |
| `sound_travel_milliseconds` | sound/sound_manager.c | Matched | audio/src/mixer.rs | audio/src/mixer.rs (measure) |
| `refresh_sound` | sound/sound_manager.c | Matched | audio/src/mixer.rs | audio/src/mixer.rs (measure) |
| `sound_channel_summary_build` | sound/sound_manager.c | Matched | audio/src/player.rs:299 | audio/src/player.rs (measure) |
| `channel_set_properties_hardware` | sound/sound_manager.c | Partial | audio/src/mixer.rs | audio/src/mixer.rs (measure); leaves out the sound card's hardware voices: the engine mixes every channel itself |
| `track_loop_track_sound` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | audio/src/player/looping.rs (measure) |
| `track_loop_impulse_sound` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | audio/src/player/looping.rs (measure) |
| `looping_sound_find` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | audio/src/player/looping.rs (measure) |
| `looping_sound_new` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | audio/src/player/looping.rs (measure) |
| `limit_pitch` | sound/sound_manager.c | Matched | audio/src/choice.rs:68 | audio/tests/sound_choice_reference.rs (trace) |
| `render_debug_sound` | sound/sound_manager.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_looping_sound` | sound/sound_manager.c | Replaced | dev/src/draw.rs | the editor and tools |
| `sound_scale_random_value` | sound/sound_manager.c | Ported | audio/src/choice.rs:56 | `tools/engine_sound_choice_reference.py` |
| `sound_calculate_fade` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | audio/src/player/looping.rs (measure) |
| `sound_start_fade` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | audio/src/player/looping.rs (measure) |
| `sound_stop` | sound/sound_manager.c | Matched | audio/src/mixer.rs | audio/src/mixer.rs (measure) |
| `source_distance` | sound/sound_manager.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `source_distance_squared` | sound/sound_manager.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `sound_preempts_sound` | sound/sound_manager.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `sound_find_best_channel` | sound/sound_manager.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `sound_find_channel` | sound/sound_manager.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `sound_find_like_channel` | sound/sound_manager.c | Matched | audio/src/player.rs:299 | audio/src/player.rs (measure) |
| `update_channel_for_impulse_sound` | sound/sound_manager.c | Partial | audio/src/mixer.rs | audio/src/mixer.rs (measure); leaves out the sound's cone: its inner and outer angles and outer gain (inner_cone_angle, outer_cone_angle, outer_cone_gain) |
| `update_channel_for_looping_sound` | sound/sound_manager.c | Partial | audio/src/mixer.rs | audio/src/mixer.rs (measure); leaves out the sound's cone: its inner and outer angles and outer gain (inner_cone_angle, outer_cone_angle, outer_cone_gain), the pitch's bend per second at most and its natural pitch (maximum_bend_per_second, natural_pitch) |
| `process_looping_sounds` | sound/sound_manager.c | Matched | audio/src/player/looping.rs:1 | audio/src/player/looping.rs (measure) |
| `sound_stop_all` | sound/sound_manager.c | Matched | audio/src/mixer.rs | audio/src/mixer.rs (measure) |
| `refresh_sounds` | sound/sound_manager.c | Matched | audio/src/mixer.rs | audio/src/mixer.rs (measure) |
| `update_channels` | sound/sound_manager.c | Matched | audio/src/mixer.rs | audio/src/mixer.rs (measure) |
| `refresh_listener` | sound/sound_manager.c | Matched | audio/src/player.rs | audio/src/player.rs (measure) |
| `sound_new_impulse` | sound/sound_manager.c | Matched | audio/src/player.rs:1 | audio/src/player.rs (measure) |
| `sound_refresh_looping` | sound/sound_manager.c | Matched | audio/src/player/looping.rs:1 | audio/src/player/looping.rs (measure) |
| `detail_sound_random_offset` | sound/sound_manager.c | Matched | audio/src/player/looping.rs:686 | audio/src/player/looping.rs (measure) |
| `sound_render` | sound/sound_manager.c | Matched | audio/src/mixer.rs | audio/src/mixer.rs (measure) |
| `read_sound_preferences` | sound/sound_preferences.c | Partial | ffi/src/settings.rs | ffi/tests/settings.rs (measure); leaves out the original's preferences file: the volumes are kept with the app's settings |
| `write_sound_preferences` | sound/sound_preferences.c | Partial | ffi/src/settings.rs | ffi/tests/settings.rs (measure); leaves out the original's preferences file: the volumes are kept with the app's settings |
| `sound_samples_per_second` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_volume_from_gain` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_frequency_from_pitch` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_angle_from_angle` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_occlusion_from_occlusion` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_obstruction_from_obstruction` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_get` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_virtual_get_state` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_virtual_stop` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_channel_update` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_begin_scene` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_dispose` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_end_scene` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_set_paused` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_flush` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_set_listener_properties` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_initialize` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_virtual_set_location` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_virtual_set_properties` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_virtual_queue_sound` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_channel_set_I3DL2_properties` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `virtual_channel_new` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_fix_rear_speakers` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_initialize_channel` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_channel_callback` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_channel_set_location` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_channel_set_properties` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_channel_queue_packet` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_channel_fill` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_channel_queue_sound` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `interrupt_time_error` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_virtual_remap` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_virtual_touch` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_channel_stopped` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `dsound_error` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `channel_stop` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `channel_get` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `virtual_channel_get` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |
| `channel_get_state` | sound/sound_dsound_xbox.c | Replaced | app/Sources/OpenCEEngineKit/SoundOutput.swift | Xbox-only hardware |

43 Missing (37 reached from `main_loop`), by file:

- sound/game_sound.c: `game_sound_initialize`, `game_sound_dispose`, `game_sound_initialize_for_new_map`, `game_sound_clear`, `game_sound_restore`, `scripted_foley_predict`, `scripted_looping_sound_set_scale`, `game_sound_set_mouth_aperture`, `compute_combined_pas`, `looping_sound_definition_is_music`, `game_sound_dispose_from_old_map`, `scripted_music_stop_all`
- sound/sound_classes.c: `sound_classes_dispose_from_old_map`, `sound_classes_dispose`
- sound/sound_definitions.c: `sound_permutation_get_mouth_aperture`, `sound_permutation_get_real_mouth_aperture`
- sound/sound_manager.c: `sound_valid_for_channel`, `current_platform_definition`, `sound_initialize_for_new_map`, `sound_dispose`, `sound_pause`, `sound_render_time`, `sound_reconnect_to_structure_bsp`, `sound_try_and_get`, `sound_enable`, `sound_update_time`, `sound_delete`, `channel_queue_sound`, `sound_set_definition_begin`, `sound_set_definition_end`, `sound_initialize`, `sound_stop_impulse`, `sound_stop_impulse_by_source_and_definition`, `source_audible`, `prioritize_sounds`, `code_001be6a0`, `code_001be890`, `code_001bebf0`, `sound_idle`, `code_001bee50`, `sound_dispose_from_old_map`
- sound/sound_scenery.c: `sound_scenery_new`, `sound_scenery_delete`

## `structures`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `node_stack_push` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `node_stack_pop` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `node_stack_read` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_delete` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `find_like_crossing` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `map_leaf_find_face_on_node` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_close_portal` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_leaf_is_closed` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_family_mark` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `render_debug_leaf_portal` | structures/leaf_map.c | Replaced |  | through its caller `render_debug_leaf_portals` |
| `render_debug_leaf_portals` | structures/leaf_map.c | Replaced |  | through its caller `structure_render_preprocess` |
| `leaf_map_build_portal_from_leaves` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_get_leaf_bounds` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_closure` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_leaf_spans_polygon` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_face_get_vertex3d` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `render_debug_leaf_faces` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `normalize_three_dee` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `intersect_planes3d` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_build_portals_from_leaf` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_build_leaf_face_for_leaf_on_node` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_build_portals` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_build_leaf_faces_for_leaf` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_build_leaf_faces` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `leaf_map_initialize_from_bsp` | structures/leaf_map.c | Replaced | import/src/level.rs | maps imported to a typed format |
| `calculate_world_from_cell_index_and_offset` | structures/structure_detail_objects.c | Replaced |  | through its caller `render_debug_detail_objects` |
| `get_local_player_datum` | structures/structure_detail_objects.c | Replaced |  | through its caller `render_debug_detail_objects` |
| `structure_detail_objects_initialize` | structures/structure_detail_objects.c | Replaced | ffi/src/render_level.rs | the enhanced lighting and Metal renderer |
| `structure_detail_objects_dispose_from_old_map` | structures/structure_detail_objects.c | Replaced | ffi/src/render_level.rs | the enhanced lighting and Metal renderer |
| `structure_detail_objects_dispose` | structures/structure_detail_objects.c | Replaced | ffi/src/render_level.rs | the enhanced lighting and Metal renderer |
| `structure_detail_objects_flush` | structures/structure_detail_objects.c | Replaced | ffi/src/render_level.rs | the enhanced lighting and Metal renderer |
| `detail_object_offset` | structures/structure_detail_objects.c | Replaced | ffi/src/render_level.rs | the enhanced lighting and Metal renderer |
| `key_compare_cells_lower_bound` | structures/structure_detail_objects.c | Replaced |  | through its caller `get_lower_bound_cell` |
| `key_compare_cells_upper_bound` | structures/structure_detail_objects.c | Replaced |  | through its caller `get_upper_bound_cell` |
| `get_lower_bound_cell` | structures/structure_detail_objects.c | Replaced |  | through its caller `structure_render_detail_objects` |
| `get_upper_bound_cell` | structures/structure_detail_objects.c | Replaced |  | through its caller `structure_render_detail_objects` |
| `dot_product4d` | structures/structure_detail_objects.c | Replaced | ffi/src/render_level.rs | the enhanced lighting and Metal renderer |
| `structure_detail_objects_initialize_for_new_map` | structures/structure_detail_objects.c | Replaced | ffi/src/render_level.rs | the enhanced lighting and Metal renderer |
| `structure_render_detail_objects` | structures/structure_detail_objects.c | Replaced |  | through its caller `render_window` |
| `render_debug_detail_objects` | structures/structure_detail_objects.c | Replaced |  | through its caller `render_debug` |
| `compare_temp_markers` | structures/structure_lens_flares.c | Replaced | ffi/src/lights.rs | the enhanced lighting and Metal renderer |
| `cluster_index_from_point` | structures/structure_lens_flares.c | Replaced | ffi/src/lights.rs | the enhanced lighting and Metal renderer |
| `build_structure_lens_flares` | structures/structure_lens_flares.c | Replaced | ffi/src/lights.rs | the enhanced lighting and Metal renderer |
| `structure_lens_flares_place` | structures/structure_lens_flares.c | Replaced | ffi/src/lights.rs | the enhanced lighting and Metal renderer |
| `structure_render_set_fog_offset` | structures/structure_render.c | Replaced |  | through its caller `structure_get_planar_fog` |
| `compare_surface_indices` | structures/structure_render.c | Replaced |  | through its caller `structure_render_sort_surface_indices` |
| `structure_render_dynamic_triangles_from_bitvector` | structures/structure_render.c | Replaced |  | through its caller `structure_render_build_dynamic_triangles` |
| `structure_render_dynamic_triangles_from_indices` | structures/structure_render.c | Replaced |  | through its caller `structure_render_build_dynamic_triangles` |
| `structure_render_build_dynamic_triangles` | structures/structure_render.c | Replaced |  | through its caller `structure_render_diffuse_light` |
| `structure_render_pass` | structures/structure_render.c | Replaced |  | through its caller `structure_render_diffuse_light` |
| `structure_render_preprocess` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_lightmaps` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_diffuse_texture` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_specular_lightmaps` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_reflection_lightmap_masks` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_reflection_mirrors` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_reflections` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_transparent_geometry` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_fog` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_fog_screen` | structures/structure_render.c | Replaced |  | through its caller `render_window` |
| `structure_render_specular_light` | structures/structure_render.c | Replaced |  | through its caller `lights_render_specular` |
| `structure_render_diffuse_light` | structures/structure_render.c | Replaced |  | through its caller `lights_render_diffuse` |
| `structure_render_shadow` | structures/structure_render.c | Replaced |  | through its caller `render_object_shadow_end` |
| `structure_decals_initialize` | structures/structure_runtime_decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `structure_decals_initialize_for_new_map` | structures/structure_runtime_decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `structure_decals_reconnect_to_structure_bsp` | structures/structure_runtime_decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `structure_decals_disconnect_from_structure_bsp` | structures/structure_runtime_decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `structure_decals_dispose_from_old_map` | structures/structure_runtime_decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `structure_decals_dispose` | structures/structure_runtime_decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `structure_decals_update` | structures/structure_runtime_decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `debug_pvs` | structures/structure_visibility.c | Replaced | dev/src/draw.rs | the editor and tools |
| `structure_visibility_find_camera` | structures/structure_visibility.c | Replaced |  | through its caller `render_player_frame` |
| `structure_visibility_traverse_subclusters` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_compute` |
| `structure_visibility_traverse_surface_lists` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_compute` |
| `bounding_rectangles_intersect` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_build_surfaces_traverse_clusters` |
| `planes_intersect_rectangle` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_build_surfaces_traverse_clusters` |
| `structure_visibility_find_objects` | structures/structure_visibility.c | Replaced |  | through its caller `find_rendered_objects` |
| `grow_clipping_rectangle_by_portal_hull` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_traverse_cluster` |
| `structure_visibility_render_debug_portal_hull` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_traverse_cluster` |
| `dequantize_byte_to_real_rectangle3d` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_build_surfaces_traverse_leaf` |
| `structure_visibility_build_surfaces_traverse_clusters` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_build_surfaces` |
| `structure_visibility_build_surfaces_traverse_leaf` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_build_surfaces_traverse_node` |
| `portal_hull_from_points` | structures/structure_visibility.c | Replaced |  | through its caller `portal_hull_from_portal` |
| `portal_hull_from_portal` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_traverse_cluster` |
| `points_within_distance` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_traverse_cluster` |
| `structure_visibility_find_mirror` | structures/structure_visibility.c | Replaced |  | through its caller `render_player_frame` |
| `structure_visibility_build_surfaces_traverse_node` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_build_surfaces` |
| `structure_visibility_traverse_cluster` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_find_clusters` |
| `structure_visibility_build_surfaces` | structures/structure_visibility.c | Replaced |  | through its caller `structure_render_diffuse_light` |
| `structure_visibility_find_clusters` | structures/structure_visibility.c | Replaced |  | through its caller `structure_visibility_compute` |
| `structure_visibility_compute` | structures/structure_visibility.c | Replaced |  | through its caller `render_window` |
| `structure_render_surface_from_point_and_leaf` | structures/structures.c | Replaced |  | through its caller `structure_test_vector` |
| `structure_get_planar_fog_definition_index` | structures/structures.c | Replaced |  | through its caller `structure_get_planar_fog` |
| `structure_test_vector` | structures/structures.c | Replaced |  | through its caller `light_particle` |
| `structure_get_planar_fog` | structures/structures.c | Replaced |  | through its caller `render_player_frame` |
| `render_debug_fog_planes` | structures/structures.c | Replaced |  | through its caller `render_debug` |
| `structure_bsp_get_cluster_pvs` | structures/structure_bsp_definitions.c | Matched | content/src/level.rs:799 | ffi/src/sound_world/sound_obstruction_reference.rs (trace) |
| `vertex_type_from_shader_tag` | structures/structure_bsp_definitions.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `structure_bsp_get_cluster_encoded_sound_distance` | structures/structure_bsp_definitions.c | Matched | content/src/level.rs:777 | ffi/src/sound_world/sound_environment_reference.rs (trace) |
| `structure_visibility_dequantize_byte_to_real` | structures/structure_visibility.c | Replaced |  | through its caller `dequantize_byte_to_real_rectangle3d` |

26 Missing (23 reached from `main_loop`), by file:

- structures/cluster_partitions.c: `reference_list_remove`, `reference_list_copy`, `cluster_partition_new`, `cluster_partition_make_valid`, `cluster_partition_make_invalid`, `cluster_partition_delete`, `cluster_partition_get_next_datum`, `cluster_partition_get_first_cluster`, `cluster_partition_get_next_cluster`, `cluster_partition_copy`, `cluster_partition_get_first_reference`, `cluster_partition_reconnect`, `cluster_partition_disconnect`, `cluster_partition_get_first_datum`
- structures/structures.c: `structures_initialize`, `structures_initialize_for_new_map`, `structures_dispose_from_old_map`, `structures_dispose`, `structure_cluster_marker_begin`, `structure_cluster_unmarked`, `structure_cluster_mark`, `structure_cluster_marker_end`, `sphere_intersects_cluster_portal`, `structure_clusters_in_sphere_recursive`, `structure_clusters_in_cone`, `structure_clusters_in_sphere`

## `tag_files`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `file_location_is_valid` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_compare_last_modification_dates` | tag_files/files_windows.c | Native |  | `std::fs` |
| `find_files_start` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_path_add_name` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_path_add_extension` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_path_remove_name` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_path_split` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_location_get_full_path` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_read_only` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_error` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_create` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_delete` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_exists` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_rename` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_open` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_close` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_get_position` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_set_position` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_get_eof` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_set_eof` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_read` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_write` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_read_from_position` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_write_to_position` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_get_last_modification_date` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_get_size` | tag_files/files_windows.c | Native |  | `std::fs` |
| `find_files_next` | tag_files/files_windows.c | Native |  | `std::fs` |
| `file_location_set_volume` | tag_files/files.c | Native |  | `std::fs` |
| `file_reference_create` | tag_files/files.c | Native |  | `std::fs` |
| `find_files` | tag_files/files.c | Native |  | `std::fs` |
| `file_read_into_memory` | tag_files/files.c | Native |  | `std::fs` |
| `file_printf` | tag_files/files.c | Native |  | `std::fs` |
| `file_reference_get_info` | tag_files/files.c | Native |  | `std::fs` |
| `file_reference_copy` | tag_files/files.c | Native |  | `std::fs` |
| `file_reference_add_directory` | tag_files/files.c | Native |  | `std::fs` |
| `file_reference_set_name` | tag_files/files.c | Native |  | `std::fs` |
| `file_reference_get_location` | tag_files/files.c | Native |  | `std::fs` |
| `file_reference_get_name` | tag_files/files.c | Native |  | `std::fs` |
| `file_references_equal` | tag_files/files.c | Native |  | `std::fs` |
| `file_reference_create_from_path` | tag_files/files.c | Native |  | `std::fs` |
| `directory_create_or_delete_contents` | tag_files/files.c | Native |  | `std::fs` |
| `datastore_read` | tag_files/files.c | Native |  | `std::fs` |
| `datastore_write` | tag_files/files.c | Native |  | `std::fs` |
| `tag_name_strip_path` | tag_files/tag_files.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_empty_data` | tag_files/tag_groups.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_data_get_pointer` | tag_files/tag_groups.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_block_get_element_with_size` | tag_files/tag_groups.c | Replaced | import/src/lib.rs | maps imported to a typed format |
| `tag_block_get_element_fast` | tag_files/tag_groups.h | Replaced | import/src/lib.rs | maps imported to a typed format |

## `text`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `text_bounds_draw_character` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string_compute_bounds` |
| `draw_string_get_string` | text/draw_string.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `draw_string_set_tab_stops` | text/draw_string.c | Partial | ui/src/draw.rs | ui/tests/draw.rs (measure); leaves out tab stops: the interface's strings are laid out without them |
| `draw_string_set_indents` | text/draw_string.c | Replaced |  | through its caller `draw_string_and_hack_in_icons` |
| `draw_string_set_color` | text/draw_string.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `draw_string_get_color` | text/draw_string.c | Replaced |  | through its caller `draw_string_and_hack_in_icons` |
| `draw_string_set_font` | text/draw_string.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `draw_string_set_format` | text/draw_string.c | Matched | ui/src/draw.rs | ui/tests/draw.rs (measure) |
| `draw_string_set_draw_mode` | text/draw_string.c | Partial | ui/src/draw.rs | ui/tests/draw.rs (measure); leaves out the draw modes other than plain text |
| `draw_string_set_highlight` | text/draw_string.c | Partial | ui/src/draw.rs | ui/tests/draw.rs (measure); leaves out the highlighted run of characters |
| `bitmap_draw_character` | text/draw_string.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `styled_font_get` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string_compute_bounds` |
| `parse_string_new` | text/draw_string.c | Replaced |  | through its caller `draw_string` |
| `parse_string` | text/draw_string.c | Replaced |  | through its caller `draw_string` |
| `parse_unicode_string` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string` |
| `draw_string_partial` | text/draw_string.c | Replaced |  | through its caller `draw_string` |
| `draw_unicode_string_partial` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string` |
| `draw_string` | text/draw_string.c | Matched | ui/src/draw.rs:200 | ui/tests/draw.rs (measure) |
| `draw_unicode_string` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string_compute_bounds` |
| `draw_string_compute_bounds` | text/draw_string.c | Partial | ui/src/draw.rs | ui/tests/draw.rs (measure); leaves out the bounds of a string measured on its own: the engine lays text out within its widget's bounds |
| `draw_unicode_string_compute_bounds` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string_capital_middle` |
| `bitmap_draw_string` | text/draw_string.c | Replaced | app/Sources/OpenCEEngineRender/UiPass.swift | the enhanced lighting and Metal renderer; drawn by the Metal renderer |
| `font_get_character_by_ascii_code` | text/font_group.c | Replaced |  | through its caller `draw_string` |
| `align_to_character` | text/international_strings.c | Replaced |  | through its caller `edit_text_fix_selection` |
| `character_in_pattern` | text/international_strings.c | Replaced |  | through its caller `parse_string` |
| `umemchr` | text/unicode.c | Native |  | `String` |
| `umemcpy` | text/unicode.c | Native |  | `String` |
| `umemcmp` | text/unicode.c | Native |  | `String` |
| `umemmove` | text/unicode.c | Native |  | `String` |
| `umemset` | text/unicode.c | Native |  | `String` |
| `ustrcpy` | text/unicode.c | Replaced |  | through its caller `virtual_keyboard_select` |
| `ustrcat` | text/unicode.c | Native |  | `String` |
| `ustrnlen` | text/unicode.c | Native |  | `String` |
| `ustrchr` | text/unicode.c | Native |  | `String` |
| `ustrcoll` | text/unicode.c | Native |  | `String` |
| `ustrcspn` | text/unicode.c | Native |  | `String` |
| `ustrncat` | text/unicode.c | Native |  | `String` |
| `ustrpbrk` | text/unicode.c | Native |  | `String` |
| `ustrrchr` | text/unicode.c | Native |  | `String` |
| `ustrspn` | text/unicode.c | Native |  | `String` |
| `ustrstr` | text/unicode.c | Replaced |  | through its caller `search_and_replace` |
| `ustrtok` | text/unicode.c | Native |  | `String` |
| `ustrxfrm` | text/unicode.c | Native |  | `String` |
| `ustrlwr` | text/unicode.c | Native |  | `String` |
| `ustrupr` | text/unicode.c | Native |  | `String` |
| `ustrnlwr` | text/unicode.c | Native |  | `String` |
| `ustrnupr` | text/unicode.c | Native |  | `String` |
| `ustrcasecmp` | text/unicode.c | Native |  | `String` |
| `ustrncasecmp` | text/unicode.c | Native |  | `String` |
| `uisalpha` | text/unicode.c | Native |  | `String` |
| `uisupper` | text/unicode.c | Native |  | `String` |
| `uislower` | text/unicode.c | Native |  | `String` |
| `uisdigit` | text/unicode.c | Native |  | `String` |
| `uisxdigit` | text/unicode.c | Native |  | `String` |
| `uisspace` | text/unicode.c | Native |  | `String` |
| `uispunct` | text/unicode.c | Native |  | `String` |
| `uisalnum` | text/unicode.c | Native |  | `String` |
| `uisprint` | text/unicode.c | Native |  | `String` |
| `uisgraph` | text/unicode.c | Native |  | `String` |
| `uiscntrl` | text/unicode.c | Native |  | `String` |
| `utoupper` | text/unicode.c | Native |  | `String` |
| `utolower` | text/unicode.c | Native |  | `String` |
| `ufgetc` | text/unicode.c | Native |  | `String` |
| `ufputc` | text/unicode.c | Native |  | `String` |
| `uungetc` | text/unicode.c | Native |  | `String` |
| `ufgets` | text/unicode.c | Native |  | `String` |
| `ufputs` | text/unicode.c | Native |  | `String` |
| `ugets` | text/unicode.c | Native |  | `String` |
| `uputs` | text/unicode.c | Native |  | `String` |
| `ufprintf` | text/unicode.c | Native |  | `String` |
| `uprintf` | text/unicode.c | Native |  | `String` |
| `uvfprintf` | text/unicode.c | Native |  | `String` |
| `uvprintf` | text/unicode.c | Native |  | `String` |
| `uvsnprintf` | text/unicode.c | Native |  | `String` |
| `uvsprintf` | text/unicode.c | Native |  | `String` |
| `ufdopen` | text/unicode.c | Native |  | `String` |
| `ufopen` | text/unicode.c | Native |  | `String` |
| `ufclose` | text/unicode.c | Native |  | `String` |
| `ufreopen` | text/unicode.c | Native |  | `String` |
| `uperror` | text/unicode.c | Native |  | `String` |
| `upopen` | text/unicode.c | Native |  | `String` |
| `uremove` | text/unicode.c | Native |  | `String` |
| `utmpnam` | text/unicode.c | Native |  | `String` |
| `ustrtol` | text/unicode.c | Native |  | `String` |
| `ustrtoul` | text/unicode.c | Native |  | `String` |
| `ustrtod` | text/unicode.c | Native |  | `String` |
| `uatoi` | text/unicode.c | Native |  | `String` |
| `uctime` | text/unicode.c | Native |  | `String` |
| `uasctime` | text/unicode.c | Native |  | `String` |
| `ascii_to_wide` | text/unicode.c | Replaced |  | through its caller `set_textbox_to_build_number` |
| `ustrerror` | text/unicode.c | Native |  | `String` |

21 Missing (19 reached from `main_loop`), by file:

- text/draw_string.c: `draw_string_initialize`, `draw_string_initialize_for_new_map`, `draw_string_dispose_from_old_map`, `draw_string_dispose`, `text_pick_draw_character`, `draw_string_pick`
- text/international_strings.c: `set_language_code`, `double_byte_character`, `get_next_character`, `get_previous_character`
- text/text_group.c: `string_list_get_string`, `unicode_string_list_get_string`, `fallback_string`
- text/unicode.c: `ustrcmp`, `ustrlen`, `ustrncmp`, `ustrncpy`, `usnprintf`, `usprintf`, `wide_to_ascii`, `ustring_format_takes`

## `tool`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `connected_geometry_new` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `connected_geometry_delete` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `connected_geometry_group_recursive` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `connected_geometry_find_or_add_vertex` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `connected_geometry_find_or_add_edge` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `plane3d_from_points` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `connected_geometry_add_triangle` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `triangle_coplanar` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `connected_geometry_add_intermediate_triangle` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `connected_geometry_group_coplanar` | tool/connected_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_file_is_open` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_initialize` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_dispose` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_set_name` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_line` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_triangle` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_polygon` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_polygon_list` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_polygon_mesh__textured_with_no_import_scale` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_comment` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_rectangle3d` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_bounded_line` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_bounded_triangle` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_bounded_polygon` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_set_transform` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_point` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |
| `error_geometry_bounded_point` | tool/error_geometry.c | Replaced | bake/src/lib.rs | the editor and tools |

## `units`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `biped_start_limp_body_physics` | units/bipeds.c | Matched | sim/src/unit_states.rs:456 | sim/src/unit_states.rs (measure) |
| `biped_falling_damage` | units/bipeds.c | Partial | sim/src/damage.rs | sim/tests/biped_fall_reference.rs (trace); leaves out the jetpack cheat, erasing a biped that fell out of the world |
| `biped_start_landing` | units/bipeds.c | Matched | physics/src/movement.rs | physics/tests/biped_landing_reference.rs (trace) |
| `biped_render_debug` | units/bipeds.c | Replaced | dev/src/draw.rs | the editor and tools |
| `biped_jump` | units/bipeds.c | Partial | physics/src/movement.rs | physics/tests/movement.rs (measure); leaves out the stun penalty, the super jump, the actor's aim-jump |
| `biped_make_footstep` | units/bipeds.c | Partial | ffi/src/sound_events.rs:97 | sim/tests/player.rs (measure); leaves out the landing's footsteps two ticks into its recovery: the engine makes them as it lands, the cinematic's silence |
| `biped_update_airborne` | units/bipeds.c | Partial | physics/src/movement.rs | physics/tests/movement.rs (measure); leaves out turning while airborne, the airborne animation states |
| `biped_update_landing` | units/bipeds.c | Partial | sim/src/lib.rs:1756 | sim/tests/player.rs (measure); leaves out the land-hard and land-soft states, which the unit animator chooses from the landing |
| `biped_update_jumping` | units/bipeds.c | Partial | physics/src/movement.rs | physics/tests/movement.rs (measure); leaves out the jetpack cheat |
| `biped_update_physics` | units/bipeds.c | Partial | sim/src/player.rs:598 | physics/tests/movement.rs (measure); leaves out flying, the collision with up to 16 planes, replaced by the character controller |
| `biped_accelerate` | units/bipeds.c | Partial | sim/src/forces.rs:213 | sim/src/player.rs (measure); leaves out the spin it gives a dead, flying or climbing biped (the ragdolls throw the dead), turning a biped's facing to the push |
| `biped_update_turning` | units/bipeds.c | Partial | sim/src/unit_states.rs:789 | sim/src/unit_states.rs (measure); leaves out the turning speed it uses while moving (moving_turning_speed) |
| `biped_update_moving` | units/bipeds.c | Partial | physics/src/movement.rs | physics/tests/movement.rs (measure); leaves out flying bipeds, animation-driven movement, elevators and support surfaces, the random speed increase, simple creatures, climbing |
| `biped_update` | units/bipeds.c | Partial | sim/src/unit_animator.rs:1 | sim/src/unit_states.rs (measure); leaves out its physics, landing and jumping, done by the player's mover (physics/src/movement.rs) |
| `dialogue_get_vocalization_name` | units/dialogue_definitions.c | Ported | sim/src/unit_dialogue.rs:239 | `tools/engine_unit_tables_reference.py` |
| `dialogue_get_vocalization_type_by_name` | units/dialogue_definitions.c | Ported | sim/src/unit_dialogue.rs:246 | `tools/engine_unit_tables_reference.py` |
| `unit_definition_get_active_hud_index` | units/unit_definitions.c | Ported | sim/src/unit_definition.rs:26 | `tools/engine_unit_tables_reference.py` |
| `unit_definition_get_seat_active_hud_index` | units/unit_definitions.c | Ported | sim/src/unit_definition.rs:27 | `tools/engine_unit_tables_reference.py` |
| `unit_get_speech_priority_name` | units/unit_dialogue.c | Ported | sim/src/unit_dialogue.rs:251 | `tools/engine_unit_tables_reference.py` |
| `unit_get_speech_priority_by_name` | units/unit_dialogue.c | Ported | sim/src/unit_dialogue.rs:260 | `tools/engine_unit_tables_reference.py` |
| `unit_scripting_set_maximum_vitality` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs:136 | campaign/tests/unit_vitality_reference.rs (trace) |
| `units_scripting_set_maximum_vitality` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | campaign/tests/unit_vitality_reference.rs (trace) |
| `unit_scripting_set_current_vitality` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | campaign/tests/unit_vitality_reference.rs (trace) |
| `units_scripting_set_current_vitality` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | campaign/tests/unit_vitality_reference.rs (trace) |
| `unit_scripting_get_health` | units/unit_scripting_commands.c | Matched | sim/src/damage.rs:153 | campaign/tests/unit_vitality_reference.rs (trace) |
| `unit_scripting_get_shield` | units/unit_scripting_commands.c | Matched | sim/src/damage.rs:163 | campaign/tests/unit_vitality_reference.rs (trace) |
| `unit_scripting_get_grenade_count` | units/unit_scripting_commands.c | Partial | campaign/src/units.rs:279 | campaign/tests/unit_commands_reference.rs (trace); leaves out units that are not players: the engine's AI carries no inventory (coverage_units) |
| `unit_scripting_impervious` | units/unit_scripting_commands.c | Partial | campaign/src/units.rs:240 | campaign/tests/unit_commands_reference.rs (trace); leaves out the damage push and the flinch it stops: the engine's damage has neither yet (coverage_units) |
| `unit_scripting_start_user_animation_list` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs:318 | campaign/tests/unit_vitality_reference.rs (trace) |
| `unit_scripting_has_weapon` | units/unit_scripting_commands.c | Ported | sim/src/unit_inventory.rs:282 | `tools/engine_unit_control_reference.py` |
| `unit_scripting_has_weapon_readied` | units/unit_scripting_commands.c | Partial | campaign/src/units.rs:265 | campaign/tests/unit_commands_reference.rs (trace); leaves out units that are not players: the engine's AI carries no inventory (coverage_units) |
| `unit_kill` | units/units.c | Matched | script/src/lib.rs:508 | script/tests/runtime.rs (measure) |
| `units_update` | units/units.c | Partial | sim/src/lib.rs:393 | sim/tests/player.rs (measure); leaves out the units' network updates, the units' order in the object list: the engine runs the players, then the actors |
| `unit_get_seat_entrance_point` | units/units.c | Matched | sim/src/seats.rs:63 | sim/tests/seats.rs (measure) |
| `unit_get_zoom_level` | units/units.c | Ported | sim/src/unit_inventory.rs:209 | `tools/engine_unit_inventory_reference.py` |
| `unit_animation_state_interruptable` | units/units.c | Ported | sim/src/unit_states.rs:220 | `tools/engine_unit_state_reference.py` |
| `unit_animation_busy` | units/units.c | Matched | sim/src/unit_states.rs:242 | sim/src/unit_states.rs (measure) |
| `unit_animation_overlay_action_loops` | units/units.c | Ported | sim/src/unit_states.rs:292 | `tools/engine_unit_state_reference.py` |
| `unit_animation_state_loops` | units/units.c | Ported | sim/src/unit_states.rs:266 | `tools/engine_unit_state_reference.py` |
| `unit_animation_aiming_screen` | units/units.c | Ported | sim/src/animation_blend.rs:167 | through `unit_animation_state_get_aiming_screen_index`'s trace; checked through unit_animation_state_get_aiming_screen_index's trace |
| `unit_animation_state_get_aiming_screen_index` | units/units.c | Ported | sim/src/unit_states.rs:314 | `tools/engine_unit_state_reference.py` |
| `unit_animation_state_can_be_entered_without_animation` | units/units.c | Ported | sim/src/unit_states.rs:205 | `tools/engine_unit_state_reference.py` |
| `unit_animation_compute_interpolation_frame_count` | units/units.c | Ported | sim/src/unit_states.rs:300 | `tools/engine_unit_state_reference.py` |
| `unit_destroy` | units/units.c | Matched | sim/src/unit_states.rs:453 | sim/src/unit_states.rs (measure) |
| `unit_get_camera_position` | units/units.c | Matched | sim/src/seats.rs:71 | sim/tests/seats.rs (measure) |
| `unit_animation_impulse_get_index` | units/units.c | Matched | sim/src/ai_impulse.rs:3 | sim/src/ai_impulse.rs (measure) |
| `unit_can_play_animation_impulse` | units/units.c | Matched | sim/src/ai_impulse.rs:3 | sim/src/ai_impulse.rs (measure) |
| `unit_test_animation_impulse` | units/units.c | Matched | sim/src/ai_impulse.rs:131 | sim/src/ai_impulse.rs (measure) |
| `unit_inventory_next_grenade` | units/units.c | Ported | sim/src/unit_inventory.rs:125 | `tools/engine_unit_inventory_reference.py` |
| `unit_controllable` | units/units.c | Ported | sim/src/unit_control.rs:44 | `tools/engine_unit_control_reference.py` |
| `unit_set_controllable` | units/units.c | Ported | sim/src/unit_control.rs:49 | `tools/engine_unit_control_reference.py` |
| `unit_set_possessed` | units/units.c | Ported | sim/src/unit_control.rs:54 | `tools/engine_unit_control_reference.py` |
| `unit_is_busy` | units/units.c | Ported | sim/src/unit_states.rs:242 | `tools/engine_unit_control_reference.py` |
| `unit_scripting_suspended` | units/units.c | Matched | sim/src/lib.rs:1198 | sim/tests/suspended.rs (measure) |
| `unit_scripting_can_blink` | units/units.c | Ported | sim/src/unit_control.rs:81 | `tools/engine_unit_seats_reference.py` |
| `unit_scripting_doesnt_drop_items` | units/units.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `unit_scripting_vehicle_test_seat_list` | units/units.c | Matched | campaign/src/units.rs:457 | campaign/tests/unit_commands_reference.rs (trace) |
| `unit_scripting_vehicle_test_seat` | units/units.c | Ported | sim/src/unit_definition.rs:50 | `tools/engine_unit_seats_reference.py` |
| `unit_scripting_unit_riders` | units/units.c | Ported | sim/src/unit_control.rs:119 | `tools/engine_unit_seats_reference.py` |
| `unit_scripting_unit_driver` | units/units.c | Ported | sim/src/unit_control.rs:109 | `tools/engine_unit_seats_reference.py` |
| `unit_scripting_unit_gunner` | units/units.c | Ported | sim/src/unit_control.rs:109 | `tools/engine_unit_seats_reference.py` |
| `units_debug_get_next_unit` | units/units.c | Replaced | dev/src/control.rs | the editor and tools |
| `units_debug_get_closest_unit` | units/units.c | Replaced | dev/src/control.rs | the editor and tools |
| `unit_debug_ninja_rope` | units/units.c | Replaced | dev/src/control.rs | the editor and tools |
| `unit_driven_by_ai` | units/units.c | Ported | sim/src/unit_control.rs:140 | `tools/engine_unit_control_reference.py` |
| `unit_gunned_by_ai` | units/units.c | Ported | sim/src/unit_control.rs:140 | `tools/engine_unit_control_reference.py` |
| `unit_set_desired_flashlight_state` | units/units.c | Ported | sim/src/unit_control.rs:61 | `tools/engine_unit_control_reference.py` |
| `unit_get_current_flashlight_state` | units/units.c | Ported | sim/src/unit_control.rs:71 | `tools/engine_unit_control_reference.py` |
| `unit_detach_from_parent` | units/units.c | Matched | sim/src/vehicle_animation.rs:337 | sim/tests/seats.rs (measure) |
| `unit_seat_filled` | units/units.c | Ported | sim/src/unit_control.rs:149 | `tools/engine_unit_control_reference.py` |
| `unit_seat_is_driver` | units/units.c | Ported | sim/src/unit_definition.rs:14 | `tools/engine_unit_tables_reference.py` |
| `unit_seat_is_gunner` | units/units.c | Ported | sim/src/unit_definition.rs:15 | `tools/engine_unit_tables_reference.py` |
| `unit_seat_allow_noncombatants` | units/units.c | Ported | sim/src/unit_definition.rs:15 | `tools/engine_unit_tables_reference.py` |
| `unit_add_grenade_type_to_inventory` | units/units.c | Ported | sim/src/unit_inventory.rs:156 | `tools/engine_unit_inventory_reference.py` |
| `unit_has_weapon_definition_index` | units/units.c | Ported | sim/src/unit_inventory.rs:174 | `tools/engine_unit_control_reference.py` |
| `unit_first_free_weapon_index` | units/units.c | Ported | sim/src/unit_inventory.rs:148 | `tools/engine_unit_inventory_reference.py` |
| `unit_get_weapon_count` | units/units.c | Ported |  | `tools/engine_pickup_reference.py` |
| `unit_approve_weapon_pickup` | units/units.c | Ported | sim/src/pickup.rs:69 | `tools/engine_pickup_reference.py` |
| `unit_get_grenade_count` | units/units.c | Ported | sim/src/unit_inventory.rs:189 | `tools/engine_unit_inventory_reference.py` |
| `unit_get_current_grenade_type` | units/units.c | Ported | sim/src/unit_inventory.rs:196 | `tools/engine_unit_inventory_reference.py` |
| `unit_throw_grenade_release` | units/units.c | Matched | sim/src/player.rs:1017 | sim/tests/grenades.rs (measure) |
| `unit_seat_update` | units/units.c | Matched | sim/src/vehicle_animation.rs:5 | sim/src/vehicle_animation.rs (measure) |
| `base_seat_label_get` | units/units.c | Ported | sim/src/unit_states.rs:367 | `tools/engine_unit_inventory_reference.py` |
| `seat_label_to_base_seat_index` | units/units.c | Ported | sim/src/unit_states.rs:372 | `tools/engine_unit_inventory_reference.py` |
| `base_weapon_label_get` | units/units.c | Ported | sim/src/unit_states.rs:364 | `tools/engine_unit_inventory_reference.py` |
| `unit_animation_update` | units/units.c | Partial | ffi/src/sound_events.rs:71 | sim/src/unit_animator.rs (measure); leaves out the sounds of a unit's own animation keys other than its footsteps |
| `unit_get_weapon` | units/units.c | Ported | sim/src/unit_inventory.rs:77 | `tools/engine_unit_inventory_reference.py` |
| `unit_melee_sound` | units/units.c | Matched | sim/src/player.rs:1184 | sim/tests/damage.rs (measure) |
| `unit_set_enterable_by_player` | units/units.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `unit_set_emotion` | units/units.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `unit_get_custom_animation_time` | units/units.c | Matched | campaign/src/units.rs:362 | campaign/tests/unit_commands_reference.rs (trace) |
| `unit_set_mouth_aperture` | units/units.c | Ported | sim/src/unit_control.rs:133 | `tools/engine_unit_control_reference.py` |
| `unit_inventory_get_must_be_readied_weapon` | units/units.c | Ported | sim/src/unit_inventory.rs:202 | `tools/engine_unit_inventory_reference.py` |
| `unit_has_weapon` | units/units.c | Ported | sim/src/unit_inventory.rs:180 | `tools/engine_unit_tables_reference.py` |
| `unit_has_weapon_with_flag` | units/units.c | Ported | sim/src/unit_inventory.rs:174 | `tools/engine_unit_inventory_reference.py` |
| `unit_stop_running_blindly` | units/units.c | Ported | sim/src/unit_control.rs:76 | `tools/engine_unit_control_reference.py` |
| `unit_set_or_test_seat_and_weapon_label` | units/units.c | Matched | sim/src/animation.rs:1396 | sim/src/unit_states.rs (measure) |
| `unit_render_debug` | units/units.c | Replaced | dev/src/draw.rs | the editor and tools |
| `unit_animation_set_state` | units/units.c | Matched | sim/src/unit_states.rs:3 | sim/src/unit_states.rs (measure) |
| `unit_find_nearby_seat` | units/units.c | Matched | sim/src/seats.rs:4 | sim/tests/seats.rs (measure) |
| `unit_inventory_get_weapon` | units/units.c | Ported | sim/src/unit_inventory.rs:77 | `tools/engine_unit_inventory_reference.py` |
| `unit_set_actively_controlled` | units/units.c | Ported | sim/src/unit_control.rs:90 | `tools/engine_unit_seats_reference.py` |
| `vehicle_scripting_find_available_seats` | units/units.c | Ported | sim/src/unit_definition.rs:103 | `tools/engine_unit_seats_reference.py` |
| `unit_open` | units/units.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `unit_close` | units/units.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `unit_overcharged` | units/units.c | Ported | sim/src/unit_inventory.rs:225 | `tools/engine_unit_seats_reference.py` |
| `units_set_desired_flashlight_state` | units/units.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `unit_get_seat_label` | units/units.c | Ported | sim/src/unit_definition.rs:39 | `tools/engine_unit_seats_reference.py` |
| `unit_approve_weapon_swap` | units/units.c | Ported | sim/src/pickup.rs:10 | `tools/engine_pickup_reference.py` |
| `unit_weapon_next_index` | units/units.c | Ported | sim/src/unit_inventory.rs:83 | `tools/engine_unit_inventory_reference.py` |
| `unit_get_current_weapon_label` | units/units.c | Ported | sim/src/unit_inventory.rs:231 | `tools/engine_unit_seats_reference.py` |
| `unit_scripting_set_seat` | units/units.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `unit_cause_melee_damage` | units/units.c | Matched | sim/src/unit_states.rs:450 | sim/src/unit_states.rs (measure) |
| `unit_stop_custom_animation` | units/units.c | Partial | campaign/src/units.rs | campaign/tests/unit_commands_reference.rs (trace); leaves out the unit's return to its idle animation (coverage_units) |
| `unit_custom_animation_at_frame` | units/units.c | Matched | sim/src/animation.rs:1479 | sim/src/animation.rs (measure) |
| `unit_preprocess_node_orientations` | units/units.c | Matched | sim/src/animation_blend.rs:5 | sim/src/animation_blend.rs (measure) |
| `unit_update_animation` | units/units.c | Matched | sim/src/unit_animator.rs:1 | sim/src/unit_animator.rs (measure) |
| `unit_unzoom` | units/units.c | Ported | sim/src/unit_inventory.rs:245 | `tools/engine_unit_seats_reference.py` |
| `unit_get_zoom_magnification` | units/units.c | Ported | sim/src/unit_inventory.rs:220 | `tools/engine_unit_seats_reference.py` |
| `unit_integrated_night_vision_is_active` | units/units.c | Ported | sim/src/unit_inventory.rs:237 | `tools/engine_unit_seats_reference.py` |
| `unit_start_animation_impulse` | units/units.c | Matched | sim/src/ai_impulse.rs:4 | sim/src/ai_impulse.rs (measure) |
| `unit_inventory_next_weapon` | units/units.c | Ported | sim/src/unit_inventory.rs:83 | `tools/engine_unit_inventory_reference.py` |
| `unit_solo_player_integrated_night_vision_is_active` | units/units.c | Ported | sim/src/unit_inventory.rs:264 | `tools/engine_unit_seats_reference.py` |
| `unit_new` | units/units.c | Partial | sim/src/objects.rs:247 | sim/tests/player.rs (measure); leaves out the unit's seats and its weapons, its animation graph's state |
| `unit_enter_seat` | units/units.c | Matched | sim/src/unit_states.rs:591 | sim/tests/seats.rs (measure) |
| `unit_exit_seat_end` | units/units.c | Matched | sim/src/lib.rs:575 | sim/tests/seats.rs (measure) |
| `unit_scripting_enter_vehicle` | units/units.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `unit_try_and_exit_seat` | units/units.c | Matched | sim/src/unit_states.rs:623 | sim/tests/seats.rs (measure) |
| `unit_update` | units/units.c | Partial | sim/src/lib.rs:416 | sim/tests/player.rs (measure); leaves out the unit's shield and health recharge, done by damage.rs, its powerups' timers, its looking at what it hears |
| `unit_scripting_exit_vehicle` | units/units.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `vehicle_causes_collision_damage` | units/vehicles.c | Matched | sim/src/damage.rs | sim/tests/world_collision.rs (measure) |
| `vehicle_hover` | units/vehicles.c | Partial | campaign/src/presented.rs | campaign/tests/script_signatures_reference.rs (trace); leaves out what it does: the engine keeps it only as an event, which nothing acts on yet |
| `compute_airborne_ticks` | units/vehicles.c | Matched | sim/src/vehicle.rs | sim/tests/vehicle_physics_reference.rs (trace) |
| `vehicle_is_flipped` | units/vehicles.c | Matched | sim/src/vehicle.rs:69 | sim/tests/vehicle_physics_reference.rs (trace) |
| `update_alien_fighter_physics_new` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `set_real_quaternion` | units/vehicles.c | Ported |  | `tools/engine_animation_reference.py` |
| `vehicle_new` | units/vehicles.c | Matched | sim/src/objects.rs:345 | sim/src/objects.rs (measure) |
| `vehicle_preprocess_node_orientations` | units/vehicles.c | Matched | sim/src/vehicle_animation.rs:3 | sim/tests/seats.rs (measure) |
| `vehicle_accelerate` | units/vehicles.c | Matched | sim/src/forces.rs:212 | sim/tests/forces.rs (measure) |
| `vehicle_render_debug` | units/vehicles.c | Replaced | dev/src/draw.rs | the editor and tools |
| `update_alien_fighter_physics_old` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `slowly_stop_vehicle` | units/vehicles.c | Matched | sim/src/vehicle.rs | sim/tests/vehicle_physics_reference.rs (trace) |
| `compute_acceleration` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `create_ghost_effect` | units/vehicles.c | Matched | sim/src/seats.rs:1281 | sim/src/seats.rs (measure) |
| `create_pelican_effect` | units/vehicles.c | Matched | sim/src/seats.rs:1282 | sim/src/seats.rs (measure) |
| `update_suspension` | units/vehicles.c | Partial | sim/src/vehicle_animation.rs:4 | sim/src/vehicle_animation.rs (measure); leaves out the suspension's sound (suspension_sound) |
| `create_slipping_effects` | units/vehicles.c | Matched | sim/src/vehicle.rs | sim/tests/vehicle_slip_reference.rs (trace) |
| `update_human_tank_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | sim/tests/vehicle_physics_reference.rs (trace) |
| `vehicle_export_function_values` | units/vehicles.c | Ported | sim/src/functions.rs:222 | `tools/engine_vehicle_function_reference.py` |
| `vehicle_cross_product3d_target` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `vehicle_dot_product3d_target` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `vehicle_triple_product3d_target` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `vehicle_update` | units/vehicles.c | Partial | sim/src/vehicle.rs | sim/tests/vehicle_physics_reference.rs (trace); leaves out the upending torque, the speed, slide and turn seek before the controllers |
| `unarmed_melee_weapon_definition_index` | units/units.c | Ported | sim/src/unit_inventory.rs:275 | `tools/engine_unit_seats_reference.py` |
| `unit_network_add_weapon` | units/units.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `unit_network_drop_weapon` | units/units.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `unit_network_forget_weapon` | units/units.c | Replaced | net/src/snapshot.rs | networking and Xbox Live; the original's network state and prediction; the engine sends snapshots |
| `update_human_boat_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | sim/tests/vehicle_physics_reference.rs (trace) |
| `update_human_jeep_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | sim/tests/vehicle_physics_reference.rs (trace) |
| `update_alien_scout_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | sim/tests/vehicle_physics_reference.rs (trace) |
| `update_alien_fighter_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | sim/tests/vehicle_physics_reference.rs (trace) |

168 Missing (159 reached from `main_loop`), by file:

- units/biped_limp_noodle.c: `biped_limp_noodle_get_max_relaxation_iterations`, `biped_limp_noodle_valid_joint_rotation`, `biped_limp_noodle_move_relax_and_constrain_positions`, `validate_real_vector3d_axes3`, `biped_limp_noodle_adjust_orientations`, `biped_limp_noodle_relax_nodes_onto_environment`
- units/bipeds.c: `animation_left_foot_frame_index`, `animation_right_foot_frame_index`, `bipeds_initialize`, `bipeds_initialize_for_new_map`, `bipeds_dispose_from_old_map`, `bipeds_dispose`, `biped_place`, `biped_delete`, `biped_reset`, `biped_disconnect_from_structure_bsp`, `biped_get_physics_pill`, `biped_stop_melee_attack`, `biped_stop_limp_body_physics`, `biped_bumped_object`, `biped_flying_through_air`, `biped_apply_rotation`, `biped_falling_danger`, `biped_adjust_placement`, `biped_export_function_values`, `biped_get_sight_position`, `biped_get_autoaim_pill`, `biped_fix_position`, `biped_find_ground_surface`, `biped_approximate_surface_index`, `biped_find_pathfinding_surface_index`, `biped_exit_seat_end`, `biped_vehicle_speech`, `biped_update_dead`, `biped_try_to_make_footsteps`, `biped_find_nearby_support_surface`, `biped_verify_object_vectors`, `biped_update_slipping`, `biped_build_flying_axes`, `biped_snap_facing`, `biped_new`, `biped_preprocess_node_orientations`, `biped_check_discard`
- units/unit_dialogue.c: `unit_test_speech`, `unit_is_speaking`, `unit_dialogue_determine_variant`, `unit_describe_speech`, `unit_lose_speech`, `unit_speak`, `unit_notify_impulse_sound`, `unit_make_damage_sound`, `unit_scream`, `unit_find_dialogue_variant`, `unit_dialogue_setup`, `unit_dialogue_update`
- units/units.c: `units_initialize`, `units_initialize_for_new_map`, `units_dispose_from_old_map`, `units_dispose`, `unit_kill_silent`, `unit_kill_no_statistics`, `unit_delete`, `unit_export_function_values`, `unit_persistent_control`, `unit_get_melee_range_and_ticks`, `unit_get_animation_frames_remaining`, `unit_euler_axis_doplan`, `unit_animation_weapon_ik`, `unit_animation_vehicle_ik`, `unit_animation_start_action`, `unit_animation_start_overlay_action`, `unit_can_enter_seat`, `unit_impulse`, `unit_handle_weapon_state_change`, `unit_record_damage`, `unit_test_spawning`, `unit_get_head_position`, `unit_estimate_position`, `unit_get_center_of_mass`, `unit_get_aiming_unit_index`, `unit_get_aiming_vector`, `unit_get_looking_vector`, `unit_get_facing_vector`, `unit_scripting_set_emotion_animation`, `unit_shield_sapping_update`, `unit_set_user_animation`, `any_unit_is_dangerous`, `unit_can_see_point`, `unit_update_driver_and_gunner`, `unit_get_current_equipment`, `unit_add_grenade_to_inventory`, `unit_add_equipment_to_inventory`, `unit_delete_current_equipment`, `unit_delete_all_weapons`, `unit_throw_grenade_move_to_hand`, `unit_set_animation`, `unit_refresh_illumination`, `unit_drop_item`, `unit_drop_grenades`, `unit_drop_inventory_weapons`, `unit_handle_region_destroyed`, `unit_cause_continuous_melee_damage`, `unit_aim_without_turning`, `unit_is_playing_custom_animation`, `unit_start_user_animation`, `unit_start_running_blindly`, `unit_flame_to_death`, `unit_flying_through_air`, `unit_euler_axis_buildplan`, `unit_euler_axis_couple`, `unit_adjust_projectile_ray`, `unit_abort_animation`, `unit_clip_to_aiming_bounds`, `unit_set_seat`, `unit_drop_current_equipment`, `unit_can_use_weapon`, `unit_drop_current_weapon`, `scripting_set_magic_base_seat`, `unit_handle_deleted_object`, `unit_cause_player_melee_damage`, `unit_align_facing`, `unit_start_flaming_to_death`, `unit_running_blind`, `unit_vectors_are_valid`, `unit_verify_vectors`, `unit_control`, `unit_postprocess_node_matrices`, `unit_euler_aiming_update`, `unit_has_animation_to_enter_seat`, `unit_ping_animation`, `unit_melee_attack_begin`, `unit_leap_begin`, `unit_unsuspecting`, `unit_add_weapon_to_inventory`, `unit_ready_desired_weapon`, `unit_throw_grenade_begin`, `scripting_magic_melee_attack`, `unit_impact_melee_damage`, `unit_place`, `unit_add_initial_weapons`, `unit_adjust_for_seat_change`, `unit_died`, `vehicle_scripting_load_magic`, `unit_damage_aftermath`, `vehicle_scripting_unload`, `unit_animation_enhanced`, `unit_grenade_throw_ease`, `unit_weapon_class_base_animation`, `unit_animation_pose_at_phase`, `unit_node_orientations_blend`, `unit_grenade_throw_keep_legs_moving`
- units/vehicles.c: `vehicles_initialize`, `vehicles_initialize_for_new_map`, `vehicles_dispose_from_old_map`, `vehicles_dispose`, `vehicle_place`, `vehicle_delete`, `vehicle_build_update`, `vehicle_parse_update`, `vehicle_reset`, `vehicle_find_pathfinding_surface_index`, `vehicle_dot_product3d_test`, `update_human_plane_physics`, `create_crashing_effects`, `update_turret_physics`, `vehicle_moving_near_any_player`, `vehicle_stuck`, `vehicle_mass_points_fit`
