# The coverage census

Every function of the original (`source/`), by system, with its class and
the evidence for it. `opence census --write` writes this file from
`engine/census/census.toml` and the reference traces; do not edit it.
docs/engine.md's "The coverage census" says what each class needs.

| System | Ported | Matched | Native | Partial | Replaced | Missing | Owed |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `ai` | | 19 (3408 B) | | 6 (11296 B) | 103 (3744 B) | 951 (277818 B) | 25 |
| `bink` | | | | | 33 (3968 B) | | 0 |
| `bitmaps` | | 1 (224 B) | | | 15 (5424 B) | 197 (58480 B) | 1 |
| `bungie_net` | | | | | 109 (18992 B) | | 0 |
| `cache` | | | | | 102 (9680 B) | 122 (15104 B) | 0 |
| `camera` | | 9 (5392 B) | | | | 137 (31152 B) | 9 |
| `cseries` | | | | | 34 (7488 B) | 117 (14352 B) | 0 |
| `cutscene` | | | | | | 81 (10528 B) | 0 |
| `devices` | | 17 (3728 B) | | | | 43 (3456 B) | 17 |
| `dialogs` | | | | | | 6 (1328 B) | 0 |
| `editor` | | | | | 9 (0 B) | | 0 |
| `effects` | | 89 (35312 B) | | | 13 (7344 B) | 88 (13952 B) | 89 |
| `game` | | 23 (5280 B) | | 10 (3296 B) | 12 (1488 B) | 675 (94016 B) | 33 |
| `hs` | | 47 (8304 B) | | 2 (848 B) | 2 (1472 B) | 692 (56368 B) | 49 |
| `input` | | | | | 42 (4064 B) | 14 (3152 B) | 0 |
| `interface` | 6 (0 B) | 24 (11056 B) | | | | 884 (145624 B) | 24 |
| `items` | 7 (4048 B) | 24 (9024 B) | | | | 151 (22976 B) | 24 |
| `main` | | | | | 15 (768 B) | 141 (15072 B) | 0 |
| `math` | 36 (4096 B) | 5 (240 B) | | | 18 (3072 B) | 157 (15760 B) | 5 |
| `memory` | | 2 (336 B) | | | 61 (9136 B) | 196 (25824 B) | 2 |
| `models` | 17 (6080 B) | 12 (4880 B) | | 1 (464 B) | 7 (1616 B) | 16 (4528 B) | 13 |
| `networking` | | | | | 313 (49824 B) | | 0 |
| `objects` | | 47 (9776 B) | | 12 (96 B) | 49 (12832 B) | 241 (28880 B) | 59 |
| `physics` | | 14 (1072 B) | | 26 (16832 B) | 27 (7568 B) | 98 (21760 B) | 40 |
| `rasterizer` | | 3 (2784 B) | | | 793 (182416 B) | | 3 |
| `render` | 2 (112 B) | 2 (240 B) | | | 134 (38000 B) | | 2 |
| `saved games` | | | | 2 (96 B) | 30 (3616 B) | 117 (21280 B) | 2 |
| `scenario` | | 2 (608 B) | | | 5 (1456 B) | 50 (7520 B) | 2 |
| `shaders` | | | | | 21 (2624 B) | | 0 |
| `shell` | | | | | 10 (528 B) | 3 (0 B) | 0 |
| `sound` | 5 (672 B) | 63 (6864 B) | | 2 (32 B) | 55 (2624 B) | 93 (16768 B) | 65 |
| `structures` | | 1 (0 B) | | | 55 (15968 B) | 71 (15744 B) | 1 |
| `tag_files` | | | | | | 49 (4288 B) | 0 |
| `text` | | 1 (912 B) | | | 15 (4240 B) | 96 (11760 B) | 1 |
| `tool` | | | | | 27 (8752 B) | | 0 |
| `units` | 8 (1824 B) | 67 (19968 B) | | 14 (1008 B) | | 278 (84720 B) | 81 |
| **all** | 81 (16832 B) | 472 (129408 B) | | 75 (33968 B) | 2109 (408704 B) | 5764 (1022210 B) | 547 |

## `ai`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `real_random_range` | ai/action_alert.c | Matched | audio/src/choice.rs:43 | owed: the engine cites it, with no check against the original yet |
| `actor_action_debug_color` | ai/actions.c | Replaced | dev/src/draw.rs | the editor and tools |
| `actor_action_handle_surprise` | ai/actions.c | Matched | sim/src/ai_impulse.rs:6 | owed: AI; reads the object, tag and map world, which the fake worlds do not hold (the actor and its prop); Phase 11's divergences |
| `actor_action_try_to_dive` | ai/actions.c | Matched | sim/src/ai_impulse.rs:5 | owed: AI; reads the object, tag and map world, which the fake worlds do not hold (the actor and its prop); Phase 11's divergences |
| `firing_position_store_evaluation_debug` | ai/actor_firing_position.c | Replaced | dev/src/draw.rs | the editor and tools |
| `actor_select_firing_position` | ai/actor_firing_position.c | Partial | campaign/src/actors.rs:542 | leaves out the pre- and post-evaluators, discarded positions, line of sight from each position, the firing position groups; owed: the campaign's rewrite of the AI; no check against the original yet (Phase 11's divergences) |
| `actor_perception_update` | ai/actor_perception.c | Partial | campaign/src/actors.rs:511 | leaves out props and their knowledge states, acknowledgement and orphans, danger zones, the target weights, searching and uncovering; owed: the campaign's rewrite of the AI; no check against the original yet (Phase 11's divergences) |
| `actor_activation_debug_color` | ai/actors.c | Replaced | dev/src/draw.rs | the editor and tools |
| `actor_kill` | ai/actors.c | Matched | campaign/src/actors.rs:393 | owed: the campaign's rewrite of the AI; no check against the original yet |
| `actor_erase` | ai/actors.c | Matched | campaign/src/actors.rs:383 | owed: the campaign's rewrite of the AI; no check against the original yet |
| `actor_place` | ai/actors.c | Partial | campaign/src/actors.rs:305 | leaves out swarms, the starting location's initial state and animation, mounted weapons, major upgrades; owed: the campaign's rewrite of the AI; no check against the original yet (Phase 11's divergences) |
| `actors_update` | ai/actors.c | Partial | campaign/src/actors.rs:404 | leaves out actions and their state machines, emotions, swarms, dormancy and activation, props; owed: the campaign's rewrite of the AI; no check against the original yet (Phase 11's divergences) |
| `ai_place` | ai/ai.c | Matched | campaign/src/actors.rs:221 | owed: the engine cites it, with no check against the original yet |
| `ai_erase` | ai/ai.c | Matched | campaign/src/ai.rs:223 | owed: a script function the campaign wires; no check against the original yet |
| `ai_update` | ai/ai.c | Partial | campaign/src/actors.rs:404 | leaves out inactive encounters and swarms released, team status, the AI's spatial effects, the conversations' own update; owed: the campaign's rewrite of the AI; no check against the original yet (Phase 11's divergences) |
| `ai_conversation_status` | ai/ai_communication.c | Matched | campaign/src/conversations.rs:19 | owed: AI; reads the object, tag and map world, which the fake worlds do not hold (the scenario's conversations); campaign/tests/flow.rs |
| `ai_conversation_line` | ai/ai_communication.c | Matched | campaign/src/conversations.rs:36 | owed: AI; reads the object, tag and map world, which the fake worlds do not hold (the scenario's conversations); campaign/tests/flow.rs |
| `ai_conversation_advance` | ai/ai_communication.c | Matched | content/src/campaign.rs:98 | owed: the engine cites it, with no check against the original yet |
| `ai_conversation_stop` | ai/ai_communication.c | Matched | campaign/src/conversations.rs:346 | owed: a script function the campaign wires; no check against the original yet |
| `ai_conversation` | ai/ai_communication.c | Matched | content/src/campaign.rs:58 | owed: the engine cites it, with no check against the original yet |
| `ai_profile_initialize` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_dispose` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_initialize_for_new_map` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_dispose_from_old_map` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_update` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_display` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00041e70` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_change_render_spray` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00041f40` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00041f50` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00041f60` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00041fb0` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00041fc0` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00041fd0` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00042020` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00042030` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00042090` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00042100` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00042160` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_000421f0` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_00042260` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `code_000422c0` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_profile_render` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
| `ai_scripting_place` | ai/ai_script.c | Matched | campaign/src/ai.rs | owed: a script function the campaign wires; no check against the original yet |
| `ai_scripting_erase` | ai/ai_script.c | Matched | campaign/src/ai.rs | owed: a script function the campaign wires; no check against the original yet |
| `ai_scripting_conversation` | ai/ai_script.c | Matched | campaign/src/conversations.rs | owed: a script function the campaign wires; no check against the original yet |
| `ai_scripting_conversation_stop` | ai/ai_script.c | Matched | campaign/src/conversations.rs | owed: a script function the campaign wires; no check against the original yet |
| `ai_scripting_conversation_advance` | ai/ai_script.c | Matched | campaign/src/conversations.rs | owed: a script function the campaign wires; no check against the original yet |
| `ai_scripting_conversation_line` | ai/ai_script.c | Matched | campaign/src/conversations.rs | owed: a script function the campaign wires; no check against the original yet |
| `ai_scripting_conversation_status` | ai/ai_script.c | Matched | campaign/src/conversations.rs | owed: a script function the campaign wires; no check against the original yet |
| `encounters_create_for_new_map` | ai/encounters.c | Partial | campaign/src/actors.rs:264 | leaves out activation cluster bit vectors, firing position owner indices, the encounter's own state (respawn, timers); owed: the campaign's rewrite of the AI; no check against the original yet (Phase 11's divergences) |
| `render_debug_path` | ai/path_obstacle_avoidance.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_obstacle_path` | ai/path_obstacle_avoidance.c | Replaced | dev/src/draw.rs | the editor and tools |
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
| `ai_profile_sample_encounters` | ai/ai_profile.c | Replaced | dev/src/draw.rs | the editor and tools |
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

951 Missing (726 reached from `main_loop`), by file:

- ai/action_alert.c: `action_alert_setup`, `action_alert_begin`, `action_alert_control`, `action_alert_flush_position_indices`, `action_alert_flush_structure_indices`, `vector_from_points3d`, `magnitude_squared3d`, `distance_squared3d`, `action_alert_update`, `action_alert_next_position`, `action_alert_perform`
- ai/action_avoid.c: `action_avoid_setup`, `action_avoid_begin`, `action_avoid_end`, `action_avoid_perform`, `action_avoid_update`, `action_avoid_control`
- ai/action_charge.c: `action_charge_begin`, `action_charge_find_target_range`, `action_charge_update`, `action_charge_control`, `action_charge_is_leaping`, `square_root`, `scale_vector2d`, `magnitude_squared2d`, `magnitude2d`, `normalize2d`, `dot_product2d`, `point_from_line3d`, `scale_vector3d`, `magnitude3d`, `normalize3d`, `dot_product3d`, `subtract_vectors3d`, `real_random`, `collision_test_line`, `action_charge_perform`, `action_charge_valid_melee_destination`, `action_charge_setup`
- ai/action_converse.c: `action_converse_setup`, `action_converse_begin`, `action_converse_perform`, `action_converse_update`, `action_converse_control`, `action_converse_replace_prop`, `actor_conversation_control`, `actor_conversation_end`, `action_converse_end`
- ai/action_fight.c: `action_fight_setup`, `action_fight_begin`, `action_fight_end`, `action_fight_update`, `action_fight_control`, `action_fight_perform`
- ai/action_flee.c: `action_flee_end`, `action_flee_flush_position_indices`, `action_flee_modify_color`, `code_00003210`, `code_00003490`, `action_flee_replace_prop`, `action_flee_blind_panic`, `action_flee_setup`, `action_flee_begin`, `action_flee_update`, `action_flee_control`, `code_000039e0`, `action_flee_perform`, `action_flee_find_flee_position`, `action_flee_current_position_exposed`, `action_flee_at_flee_position`
- ai/action_guard.c: `action_guard_setup_current_position`, `action_guard_setup_find_position`, `action_guard_setup_postcombat`, `action_guard_begin`, `action_guard_end`, `action_guard_speak_post_combat`, `action_guard_update`, `action_guard_flush_position_indices`, `action_guard_flush_structure_indices`, `action_guard_modify_color`, `action_guard_replace_prop`, `action_guard_setup_from_combat_transition`, `action_guard_setup_from_fleeing`, `action_guard_perform`, `action_guard_control`
- ai/action_obey.c: `code_00004f70`, `code_00004fb0`, `code_000051e0`, `code_00005250`, `code_00005290`, `code_000052e0`, `code_00005300`, `code_00005350`, `action_obey_command_list_setup`, `action_obey_flush_command_indices`, `action_obey_flush_structure_indices`, `action_obey_advance_command_list`, `action_obey_begin`, `action_obey_end`, `action_obey_describe_command`, `vector_from_points2d`, `cross_product3d`, `negate_vector3d`, `random_range`, `code_00005f70`, `code_000060c0`, `code_000071a0`, `code_00007720`, `code_00007840`, `action_obey_update`, `action_obey_perform`, `action_obey_control`, `vehicle_possibility_qsort`, `action_obey_command_end`, `action_obey_individual_setup`, `action_obey_individual_flush_command_indices`, `action_obey_individual_begin`, `action_obey_individual_advance`, `action_obey_individual_end`, `action_obey_individuals_iterate`, `action_obey_directmovement_update_facing`, `action_obey_command_begin`, `action_obey_command_perform`, `action_obey_individual_perform`, `action_obey_individual_update`
- ai/action_search.c: `action_search_setup_target`, `action_search_setup_pursuit`, `action_search_setup_undirected`, `action_search_update`, `action_search_flush_position_indices`, `action_search_flush_structure_indices`, `action_search_control`, `action_search_begin`, `action_search_perform`
- ai/action_sleep.c: `action_sleep_control`
- ai/action_uncover.c: `action_uncover_setup_target`, `action_uncover_setup_pursuit`, `action_uncover_perform`, `action_uncover_control`, `action_uncover_flush_position_indices`, `action_uncover_flush_structure_indices`, `action_uncover_modify_color`, `action_uncover_begin`, `action_uncover_update`
- ai/action_vehicle.c: `object_get_bounding_sphere`, `action_vehicle_begin`, `action_vehicle_update`, `action_vehicle_flush_structure_indices`, `action_vehicle_control`, `point_from_line2d`, `distance_squared2d`, `distance2d`, `negate_vector2d`, `distance3d`, `code_000093b0`, `code_000094c0`, `code_00009890`, `action_vehicle_setup_specific`, `action_vehicle_perform`, `code_0000a1b0`, `action_vehicle_setup_impromptu`, `action_vehicle_desirable`, `action_vehicle_evaluate_seat`, `action_vehicle_find_destination`, `action_vehicle_find_impromptu_seat`
- ai/action_wait.c: `action_wait_begin`, `action_wait_perform`, `action_wait_control`, `action_wait_setup`, `action_wait_update`
- ai/actions.c: `encounter_get_squad`, `actor_action_perform`, `actor_action_update`, `actor_action_control`, `actor_action_replace_prop`, `actor_action_flush_position_indices`, `actor_action_flush_structure_indices`, `actor_action_handle_panic_from_surprise`, `actor_action_handle_panic_from_damage`, `actor_action_handle_panic_from_burning_to_death`, `actor_action_handle_panic_from_attached_projectiles`, `actor_action_handle_panic_from_attached_melee_attackers`, `actor_action_handle_berserking_from_attacking_mode`, `actor_action_handle_berserking_from_proximity`, `actor_action_handle_berserking_from_damage`, `actor_action_deny_transition`, `actor_action_allowed_to_enter_vehicle`, `actor_action_handle_vehicle_exit`, `actor_action_allow_cover_seeking`, `actor_action_determine_pursuit_options`, `actor_action_can_stop_guarding`, `actor_action_can_stop_conversing`, `actor_action_change`, `actor_action_test_grenade`, `actor_action_try_to_seek_cover`, `actor_action_try_to_panic`, `actor_action_try_to_enter_vehicle`, `actor_get_pursuit_location`, `actor_pursuit_consider_nearby_actor`, `actor_action_name`, `actor_mode_name`, `actor_action_class`, `actor_action_get_default_state`, `set_real_vector2d`, `set_real_vector3d`, `point_to_line_distance3d`, `actor_action_set_default_state`, `actor_action_handle_initial_action`, `actor_action_handle_pending_command_list`, `actor_action_handle_panic_transition`, `actor_action_handle_berserking_from_attached_projectiles`, `actor_action_handle_vehicle_entry`, `actor_action_find_escape_from_danger`, `actor_action_handle_active_cover_seeking`, `actor_action_handle_combat_selection`, `actor_action_handle_lost_contact`, `actor_action_handle_done_fleeing`, `actor_action_handle_combat_status`, `actor_action_handle_combat_failure`, `actor_action_handle_exit_pursuit`, `actor_action_try_to_throw_grenade`, `actor_action_consider_grenade`, `actor_action_try_to_evade`, `actors_searching_same_position`, `actor_pursuit_find_nearby_actors`, `actor_action_handle_berserk_transition`, `actor_action_handle_combat_transition`, `actor_action_handle_grenade_throwing`, `actor_action_handle_evasion`, `actor_action_handle_danger_avoidance`
- ai/actor_combat.c: `code_0000f590`, `actor_combat_fire_wildly`, `actor_combat_disable_bursts`, `actor_firing_blindly`, `actor_combat_currently_firing_burst`, `actor_get_weapon_definition`, `actor_get_weapon_vector`, `actor_combat_get_firing_variant_definition`, `code_0000f880`, `code_0000f920`, `fast_ftol`, `sine`, `cosine`, `tangent`, `cross_product2d`, `add_vectors3d`, `random_boolean`, `valid_real`, `actor_combat_find_nearby_target`, `code_0000fba0`, `code_0000fc50`, `actor_combat_reaim_grenade`, `actor_combat_build_grenade_trajectory`, `actor_combat_find_grenade_target`, `actor_combat_check_collateral_damage`, `actor_combat_plan_grenade_trajectory`, `valid_realcmp`, `valid_real_normal3d`, `code_00010620`, `actor_aim_projectile`, `code_000109a0`, `actor_combat_retarget_grenade`, `actor_aim_grenade`, `actor_combat_update`, `actor_combat_enable_special_fire_situation`, `actor_combat_get_firing_parameters`, `actor_get_grenade_definition`, `actor_start_pause`, `actor_start_first_burst_delay`, `actor_combat_allow_special_fire_situation`, `actor_start_burst`
- ai/actor_firing_position.c: `firing_position_reject`, `pre_evaluator_guard`, `pre_evaluator_combatmove`, `post_evaluator_global`, `post_evaluator_pursuit`, `post_evaluator_hide`, `post_evaluator_uncover`, `post_evaluator_attack`, `firing_position_pre_evaluate`, `firing_position_post_evaluate`, `firing_positions_get_post_evaluation_bound`, `firing_position_compare`, `actor_get_firing_position_group`, `actor_clear_discarded_firing_positions`, `actor_discard_firing_position`, `actor_firing_position_discarded`, `pre_evaluator_global`, `pre_evaluator_pursuit`, `pre_evaluator_panic`, `code_00013db0`, `code_00013f80`, `actor_nearby_firing_positions`, `actor_active_select_firing_position`, `actor_change_firing_position`, `pre_evaluator_attack`, `firing_position_compute_line_of_sight`, `firing_position_forced_evaluation`
- ai/actor_looking.c: `actor_look_secondary_stop`, `actor_look_compute_prop_interest`, `actor_look_get_looking_definition`, `actor_look_secondary`, `actor_look_valid_aim_vector`, `actor_look_valid_look_vector`, `actor_looking_test_validity`, `actor_look_idle_find_prop`, `actor_look_idle_timer`, `actor_look_find_random_vector`, `valid_real_normal2d`, `actor_look_decode_direction`, `actor_look_idle_new_major_direction`, `actor_look_idle_new_minor_direction`, `actor_look_update`, `actor_look_affect_movement`
- ai/actor_moving.c: `actor_move_keep_moving_past_destination`, `actor_move_animation_busy`, `actor_path_clear`, `actor_path_has_path`, `actor_path_at_destination`, `actor_path_get_destination_firing_position_index`, `actor_path_input_new`, `arccosine`, `midpoint3d`, `actor_test_destination`, `actor_get_stopping_distances`, `actor_move_animation_impulse`, `actor_move_force_stop`, `actor_move_try_evasion_vector`, `actor_move_try_evasion_direction`, `actor_aim_jump`, `code_000193f0`, `code_00019630`, `code_00019920`, `actor_move_transform_avoidance_vector`, `actor_move_get_avoidance_direction`, `actor_move_initialize`, `actor_path_3d_available`, `code_00019e40`, `code_0001a0c0`, `code_0001a390`, `actor_path_refresh`, `actor_destination_update`, `actor_move_to_point`, `actor_move_to_move_position`, `actor_move_to_firing_position`, `actor_move_to_prop`, `code_0001c0b0`, `actor_move_update`, `actor_move_halt`, `actor_move_halt_at_firing_position`, `actor_move_avoidance_setup`, `actor_move_test_avoidance_vector`, `actor_move_vector_avoidance_find_direction`, `actor_move_vector_avoidance`, `actor_move_calculate_movement`, `actor_move_calculate_controlled_by_aiming`, `actor_move_calculate_free`
- ai/actor_perception.c: `actor_perception_acknowledge`, `actor_get_perception_knowledge`, `actor_get_vision_distances`, `code_0001dbc0`, `code_0001dc00`, `actor_perception_desire_prop`, `actor_perception_find_prop_pathfinding_location`, `actor_perception_find_killer_prop_index`, `arctangent`, `actor_perception_find_recent_damaging_prop_index`, `actor_perception_forget_recent_damage`, `actor_perception_retreat_successful`, `actor_compute_prop_unopposable`, `actor_compute_prop_target_weight`, `actor_situation_update_target_status`, `actor_situation_combat_status_update`, `actor_situation_update`, `actor_situation_try_new_target`, `actor_perception_friend_prop_is_attacking`, `actor_perception_aiming_vector_test_blockage`, `actor_emotion_flee_with_friends`, `code_0001f470`, `code_0001f4f0`, `code_0001f560`, `actor_berserk`, `actor_visibility_at_point`, `actor_audibility_at_point`, `actor_perception_find_sense_position`, `code_00020210`, `prop_position_refresh`, `code_00020780`, `actor_perception_refresh_danger_zone`, `actor_expected_acknowledgement`, `actor_perception_unreachable`, `actor_perception_tried_to_uncover`, `actor_perception_tried_to_search`, `actor_perception_abandoned_search`, `actor_emotion_update`, `actor_perception_become_acknowledged`, `prop_status_refresh`, `code_000228b0`, `actor_perception_create_orphan_from_friend`, `code_00023290`, `actor_perception_midpoint3d`, `actor_emotion_assess_unopposable_danger`, `actor_emotion_get_unopposable_enemy`, `actor_emotion_unopposable_retreat`, `actor_perception_unit_from_swarm`, `actor_perception_assess_vehicle_danger`, `actor_perception_refresh_test_object`, `actor_perception_refresh`, `actor_perception_qsort_compare_optional_props`, `actor_perception_assess_suicide_danger`
- ai/actor_stimulus.c: `actor_stimulus_clear`, `actor_stimulus_combat`, `actor_stimulus_surprise`, `actor_stimulus_suspicion`, `actor_stimulus_prop_sighted`, `actor_stimulus_enter_combat_found_body`, `actor_stimulus_enter_combat_perceived_enemy`, `actor_stimulus_enter_combat_friend_in_combat`, `actor_stimulus_bumped`, `actor_stimulus_environmental_noise`, `actor_stimulus_heard_shooting`, `actor_stimulus_was_surprised`, `actor_stimulus_maneuvering`, `actor_stimulus_vehicle_eviction`, `actor_stimulus_abandon_stationary_facing`, `actor_stimulus_prop_acknowledged`, `actor_stimulus_damage`, `actor_stimulus_weapon_impact`, `actor_stimulus_weapon_detonation`, `actor_stimulus_prop_just_killed`, `actor_stimulus_prop_fleeing`, `actor_stimulus_noticed_danger_zone`
- ai/actor_type_carrier.c: `carrier_decide_action`
- ai/actor_type_crew.c: `crew_decide_action`
- ai/actor_type_elite.c: `elite_decide_action`
- ai/actor_type_engineer.c: `engineer_decide_action`
- ai/actor_type_flood.c: `flood_decide_action`, `actor_type_flood_desire_shamble`
- ai/actor_type_grunt.c: `grunt_decide_action`
- ai/actor_type_hunter.c: `hunter_decide_action`
- ai/actor_type_infection.c: `infection_decide_action`, `code_000273b0`, `code_00027410`, `code_00027470`, `infection_swarm_aim_jump`, `infection_wander_pause_time`, `infection_wander_move_time`, `infection_swarm_control`
- ai/actor_type_jackal.c: `jackal_decide_action`
- ai/actor_type_marine.c: `marine_decide_action`
- ai/actor_type_mounted_weapon.c: `mounted_weapon_decide_action`
- ai/actor_type_sentinel.c: `sentinel_decide_action`
- ai/actor_types.c: `actor_type_definition_get`, `actor_types_initialize`, `actor_type_get_name`, `actor_type_get_race`, `actor_type_get_when_to_search_at_target`, `actor_type_get_when_to_pursue`, `actor_type_get_when_to_search_pursuit`, `actor_type_get_pursuit_controller`, `actor_type_get_swarm`, `actor_type_initialize`, `actor_type_decide_action`, `actor_type_swarm_control`, `actor_type_swarm_aim_jump`
- ai/actors.c: `actors_initialize`, `actors_dispose`, `actors_initialize_for_new_map`, `actors_dispose_from_old_map`, `actor_set_team`, `code_00029230`, `actor_verify_activation`, `actor_detach_from_unit`, `actor_swarm_detach_from_unit`, `actor_swarm_cache_delete`, `code_000296c0`, `actor_is_noncombat`, `actor_in_combat`, `actor_is_fighting`, `actor_attacking_target`, `actor_is_leaping`, `actor_get_weapon`, `actor_has_ranged_weapon`, `actor_target_unit_index`, `actor_derive_target_information`, `actor_switch_props`, `actor_flush_position_indices`, `actor_flush_structure_indices`, `actor_has_unlimited_grenades`, `actor_handle_damage`, `code_00029df0`, `code_00029e70`, `actors_freeze`, `code_00029f50`, `actors_move_randomly`, `actor_change_encounter`, `code_0002a120`, `code_0002a150`, `code_0002a160`, `code_0002a200`, `actor_find_pathfinding_location`, `actor_destination_tolerance`, `actor_input_sample_position`, `code_0002a460`, `code_0002a4a0`, `actors_handle_spatial_effect`, `actor_handle_communication`, `actor_unit_control_crouch`, `actor_unit_control_jump`, `actor_unit_control_primary_trigger`, `actor_unit_control_secondary_trigger`, `actor_unit_control_throw_grenade`, `actor_unit_control_exact_facing`, `actor_unit_control_stop_animation_impulse`, `actor_new`, `actor_customize_unit`, `actor_set_dormant`, `code_0002b160`, `actor_delete_props`, `actor_delete`, `actor_swarm_cache_new`, `actor_get_running_blind_vector`, `actor_died`, `actor_swarm_unit_died`, `actor_braindead`, `actor_handle_unit_effect`, `actor_set_active`, `actor_swarm_attach_unit`, `code_0002c000`, `code_0002c230`, `actors_handle_unit_effect`, `code_0002cdb0`, `actor_attach_unit`, `code_0002d290`, `actor_create_for_unit`, `actors_spawn_from_unit`, `actor_verify_unit_activation`, `actor_swarm_component_refresh`, `actor_freeze_unit`, `actor_freeze`, `actor_update_begin`, `actor_update_end`, `actor_get_timeslice`, `actor_clear_output`, `actor_input_update`, `actor_clear_orders`, `actor_decision_loop`, `actor_swarm_component_setup`, `actor_randomly_control_unit`, `actor_general_update`, `actor_unit_control`, `actor_update`
- ai/ai.c: `ai_initialize`, `ai_dispose`, `ai_dispose_from_old_map`, `ai_globals_ai_active`, `ai_globals_dialogue_triggers_enabled`, `ai_globals_grenades_enabled`, `ai_get_major_upgrade_chance`, `ai_adjust_damage`, `ai_release_inactive_swarms`, `code_0002e110`, `ai_find_inactive_encounters`, `ai_release_inactive_encounters`, `ai_try_vehicle_eviction`, `ai_get_responsible_unit`, `ai_handle_death`, `ai_handle_killing_spree`, `ai_handle_allegiance_status_changed`, `ai_handle_allegiance_broken_notification`, `ai_update_team_status`, `ai_handle_bump`, `ai_handle_damage`, `code_0002eb80`, `ai_create_mounted_weapons_for_unit`, `ai_handle_deleted_object`, `ai_handle_unit_effect`, `ai_handle_enter_vehicle`, `ai_handle_exit_vehicle`, `code_0002f050`, `ai_disconnect_from_structure_bsp`, `ai_reconnect_to_structure_bsp`, `ai_get_race_from_team_index`, `ai_initialize_for_new_map`, `ai_consider_major_upgrade`, `code_0002f9d0`, `code_0002fa30`, `ai_test_line_of_fire`, `ai_test_line_of_sight`, `ai_test_ballistic_line_of_fire`, `ai_handle_editing`, `code_000309a0`, `ai_enemies_can_see_player`, `ai_enemies_attacking_player`, `ai_handle_spatial_effect`, `compare_potentially_releasable_entities`, `ai_flush_spatial_effects`, `ai_generate_line_of_fire_pill`, `ai_find_line_of_fire_friend_pills`, `ai_place_pending_mounted_weapons`
- ai/ai_communication.c: `ai_communication_initialize`, `ai_communication_dispose`, `ai_communication_initialize_for_new_map`, `ai_communication_dispose_from_old_map`, `ai_communication_get_type_name`, `ai_communication_get_type_by_name`, `ai_communication_packet_new`, `reply_filter_close`, `reply_filter_not_close`, `reply_filter_searching`, `reply_filter_same_platoon`, `reply_filter_fighting`, `reply_filter_fighting_close`, `reply_filter_same_target`, `reply_filter_no_certain_target`, `reply_filter_flee_leader`, `ai_communication_consider_speech`, `actor_communication_team`, `ai_communication_look_secondary_at_unit`, `ai_communication_look_secondary_at_object`, `ai_conversation_finish`, `ai_conversation_new`, `ai_conversation_line_begin`, `ai_conversation_line_perform`, `ai_conversation_line_end`, `actor_reset_idle_vocalization_timer`, `actor_communication_update`, `ai_communication_handle_received_looking`, `ai_communication_update_speech_timers`, `ai_communication_get_player_rating`, `ai_conversation_actor_deleted`, `ai_conversation_unit_died`, `ai_conversation_find_participant`, `ai_communication_started`, `ai_communication_notify`, `ai_communication_actor_talk_weight`, `ai_communication_find_specific_actor_to_talk`, `ai_communication_find_global_actor_to_talk`, `ai_conversation_begin`, `ai_communication_find_actor_to_reply_to_player`, `ai_communication_finished`, `ai_conversation_update`, `ai_communication_event`
- ai/ai_scenario_definitions.c: `scenario_get_encounter_by_name`, `encounter_definition_get_squad_by_name`, `encounter_definition_get_platoon_by_name`, `choose_random_array_element`
- ai/ai_script.c: `encounter_get_platoon`, `ai_script_initialize`, `ai_script_dispose`, `ai_script_initialize_for_new_map`, `ai_script_dispose_from_old_map`, `ai_index_from_string`, `ai_index_to_string`, `ai_index_platoon_iterator_new`, `ai_index_platoon_iterator_next`, `ai_index_squad_iterator_new`, `ai_index_squad_iterator_next`, `ai_index_actor_iterator_new`, `ai_index_actor_iterator_next`, `object_list_from_ai_reference`, `ai_scripting_attach_unit`, `ai_scripting_attach_units`, `ai_scripting_detach_unit`, `ai_scripting_detach_units`, `ai_scripting_kill_internal`, `ai_scripting_kill`, `ai_scripting_kill_silent`, `ai_scripting_erase_all`, `ai_scripting_deselect`, `ai_scripting_select`, `ai_scripting_spawn_actor`, `ai_scripting_set_respawn`, `ai_scripting_set_deaf`, `ai_scripting_set_blind`, `ai_scripting_magically_see_unit`, `ai_scripting_magically_see_units`, `ai_scripting_timer_start`, `ai_scripting_timer_expire`, `ai_scripting_count_internal`, `ai_scripting_swarm_count`, `ai_scripting_nonswarm_count`, `ai_scripting_living_count`, `ai_scripting_living_fraction`, `ai_scripting_strength`, `ai_scripting_is_attacking`, `ai_scripting_attack`, `ai_scripting_defend`, `ai_scripting_maneuver`, `ai_scripting_maneuver_enable`, `code_00044010`, `code_00044440`, `ai_scripting_migrate`, `code_00044a30`, `ai_scripting_migrate_by_unit`, `ai_scripting_migrate_and_speak`, `ai_scripting_allegiance`, `ai_scripting_allegiance_remove`, `ai_scripting_allegiance_broken`, `ai_scripting_vehicle_candidate_qsort`, `ai_scripting_going_to_vehicle`, `ai_scripting_exit_vehicle`, `ai_scripting_braindead`, `ai_scripting_braindead_by_unit`, `ai_scripting_ignore`, `ai_scripting_prefer_target`, `code_000452d0`, `ai_scripting_teleport_starting_location_if_unsupported`, `ai_scripting_teleport_starting_location`, `ai_scripting_try_to_fight_nothing`, `ai_scripting_try_to_fight`, `ai_scripting_try_to_fight_player`, `ai_scripting_allow_charge`, `ai_scripting_command_list`, `ai_scripting_command_list_by_unit`, `ai_scripting_command_list_advance`, `ai_scripting_command_list_advance_by_unit`, `code_000459a0`, `ai_scripting_command_list_status`, `ai_scripting_free`, `ai_scripting_free_units`, `ai_scripting_attach_free`, `ai_scripting_force_active`, `ai_scripting_force_active_by_unit`, `ai_scripting_set_return_state`, `ai_scripting_set_current_state`, `ai_scripting_assess_status`, `ai_scripting_status`, `ai_scripting_reconnect`, `ai_scripting_playfight`, `ai_scripting_vehicle_encounter`, `ai_scripting_find_vehicle_enterable`, `ai_scripting_vehicle_enterable_distance`, `ai_scripting_vehicle_enterable_team`, `ai_scripting_vehicle_enterable_actor_type`, `ai_scripting_vehicle_enterable_actors`, `ai_scripting_vehicle_enterable_disable`, `ai_scripting_look_at_object`, `ai_scripting_stop_looking`, `ai_scripting_automatic_migration_target`, `ai_scripting_follow_target_disable`, `ai_scripting_follow_target_players`, `ai_scripting_follow_target_unit`, `ai_scripting_follow_target_ai`, `ai_scripting_follow_distance`, `ai_scripting_link_activation`, `ai_scripting_berserk`, `ai_scripting_set_team`, `ai_scripting_allow_dormant`, `ai_scripting_magically_see_encounter`, `ai_scripting_magically_see_players`, `ai_scripting_retreat`, `ai_scripting_go_to_vehicle_internal`, `ai_scripting_go_to_vehicle`, `ai_scripting_go_to_vehicle_override`, `ai_scripting_renew`, `ai_scripting_migrate_find_target_squad`, `ai_scripting_migrate_internal`, `ai_scripting_migrate_by_unit_internal`, `ai_scripting_teleport_starting_location_private`, `ai_scripting_command_list_status_internal`
- ai/encounters.c: `encounters_initialize`, `encounters_dispose`, `encounters_dispose_from_old_map`, `encounter_compute_activation_cluster_bit_vector`, `encounter_detach_actor`, `encounter_attach_unit`, `encounterless_attach_actor`, `encounterless_detach_actor`, `encounter_get_by_name`, `encounter_iterator_new`, `encounter_iterator_next`, `encounter_actor_iterator_new`, `encounter_actor_iterator_next`, `encounter_actor_iterator_prev`, `actor_iterator_new`, `actor_iterator_next`, `code_00048260`, `code_000482b0`, `encounter_modify_pursuit_desires`, `encounter_determine_pursuit_availability`, `code_000486c0`, `code_00048790`, `code_00048a20`, `encounterless_activate`, `code_00048b00`, `code_00048b50`, `encounter_link_activation`, `code_00048cb0`, `code_00048d50`, `encounter_stand_down`, `code_000492d0`, `code_00049360`, `encounter_set_blind`, `encounter_set_deaf`, `encounter_squad_timer_expire`, `code_000494e0`, `code_000495e0`, `encounters_initialize_for_new_map`, `encounters_unit_died`, `encounter_verify_firing_position_owner_actor_indices`, `encounter_build_firing_position_owner_actor_indices`, `encounter_mark_examined_pursuit_position`, `encounter_pursuit_position_already_examined`, `encounter_get_actor_starting_location`, `encounter_force_activate`, `encounter_force_deactivate`, `code_0004a140`, `code_0004a250`, `code_0004aa10`, `encounter_spawn_actor`, `encounter_set_respawn`, `code_0004acf0`, `code_0004afb0`, `code_0004b0f0`, `encounter_attach_actor`, `encounter_update_status`, `encounters_update_dirty_status`, `encounter_create`, `code_0004c270`, `encounters_update`, `encounter_clear_pursuit`, `encounter_find_pursuit`, `squad_reset_starting_locations`, `encounter_definition_fit`, `encounter_new`, `encounter_activate`, `encounter_deactivate`, `squad_get_actor_type`, `encounter_post_combat_add_possibility`, `encounter_post_combat_select_random_behavior`, `encounter_post_combat`, `encounter_update_timers`, `encounter_test_rule`, `encounter_update_respawn`, `encounter_update_platoons`, `encounter_update_follow`, `encounter_control_actors`, `encounter_update_squads`, `encounterless_deactivate`, `encounters_test_activation`
- ai/path.c: `paths_initialize`, `paths_dispose`, `paths_initialize_for_new_map`, `paths_dispose_from_old_map`, `path_input_new`, `path_input_set_target_object`, `path_input_set_start`, `path_input_set_attractor`, `path_input_set_search_bounds`, `path_state_new`, `path_state_destination`, `code_0004c770`, `code_0004c7b0`, `code_0004c7c0`, `code_0004c9a0`, `code_0004cbd0`, `code_0004ccf0`, `code_0004cd70`, `path_get_node`, `path_node_from_hash_table`, `path_3d_available`, `path_3d_build_path`, `path_state_approach_point`, `path_state_build_path`, `code_0004d5f0`, `code_0004d840`, `code_0004d8b0`, `closest_point_to_attractor`, `path_attractor_weight`, `path_state_estimated_distance`, `code_0004ddb0`, `path_state_find`, `path_state_reset`, `path_heap_verify`, `path_heap_bubble_up`, `path_heap_bubble_down`, `path_heap_pop_cheapest_node`, `path_heap_insert`, `surface_is_broken`, `path_state_begin`, `closest_available_point_on_surface`, `build_path_edges_for_surface`, `path_state_traverse`
- ai/path_obstacle_avoidance.c: `obstacles_get_disc`, `obstacle_from_disc`, `path_get_step`, `path_get_step_index`, `heap_parent_index`, `heap_left_index`, `heap_right_index`, `heap_cost`, `error_heap`, `heap_verify`, `heap_up`, `heap_down`, `heap_insert`, `heap_remove`, `valid_real_point2d`, `path_add_step`, `path_new`, `path_test_pill2d`, `path_add_steps`, `path_iterate`, `path_find`, `path_avoid_obstacles`
- ai/path_obstacles.c: `obstacles_new`, `set_real_point2d`, `rotate_vector2d`, `project_point3d`, `point_in_circle`, `point_in_sphere`, `circle_intersect_ray`, `circle_tangents`, `obstacles_add_disc`, `obstacles_get_discs_in_sphere`, `obstacles_test_circle`, `obstacles_test_pill`, `obstacles_disc_tangents`, `obstacles_disc_neighborhood`, `obstacles_recompute`
- ai/path_smoothing.c: `surface_is_walkable`, `find_tangent_point`, `find_avoidance_point`, `choose_turning_point`, `find_turning_point`, `path_smooth`
- ai/path_structure_bsp.c: `structure_test_ray2d`, `structure_surfaces_are_equivalent`, `clip_empty_interval_by_solid_interval`, `structure_test_line2d`, `structure_surface_index_from_point`, `structure_test_pill2d`, `path_pathfinding_surface`
- ai/props.c: `props_initialize`, `props_dispose`, `props_initialize_for_new_map`, `props_dispose_from_old_map`, `prop_add`, `prop_new_blank`, `prop_remove`, `prop_iterator_new`, `prop_iterator_next`, `prop_new_unacknowledged`, `prop_setup_orphan`, `prop_orphan_transition`, `prop_orphan_from_friend`, `prop_orphan_update_information`, `prop_delete`, `prop_get_active_by_unit_index`, `prop_get_base_by_unit_index`

## `bink`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `code_001b53f0` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_active` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_ui_rendering_inhibited` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_playback_in_progress` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5480` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5570` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b55a0` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5790` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_decompress_audio_frame` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5860` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5bf0` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5c00` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `code_001b5c50` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
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
| `bink_draw_frame` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_free_texture_cache` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_get_memory_available` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_alloc_permanent` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_alloc` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_free` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `is_all_bink_memory_free` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_decompress_video_frame` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |
| `bink_query_analog_controller_buttons` | bink/bink_playback.c | Replaced | app/Sources/OpenCEEngineMenu/Launcher.swift | Bink |

## `bitmaps`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `RGBToColor` | bitmaps/s3tc/s3tc.c | Replaced |  | through its caller `DecodeBlockRGB__single_pixel` |
| `DecodeBlockRGB__single_pixel` | bitmaps/s3tc/s3tc.c | Replaced |  | through its caller `DecodeBlockAlpha3__single_pixel` |
| `DecodeBlockAlpha4__single_pixel` | bitmaps/s3tc/s3tc.c | Replaced |  | through its caller `bitmap_2d_get_pixel` |
| `DecodeBlockAlpha3__single_pixel` | bitmaps/s3tc/s3tc.c | Replaced |  | through its caller `bitmap_2d_get_pixel` |
| `bitmap_group_get_bitmap_from_sequence` | bitmaps/bitmap_group.c | Matched | ffi/src/hud.rs:657 | owed: the engine cites it, with no check against the original yet |
| `pixel32_to_real_argb_color` | bitmaps/bitmap_utilities.c | Replaced |  | through its caller `cinematic_render` |
| `pixel32_to_real_rgb_color` | bitmaps/bitmap_utilities.c | Replaced |  | through its caller `custom_render_nav_point` |
| `rgb_colors_interpolate_and_scale` | bitmaps/bitmap_utilities.c | Replaced |  | through its caller `lights_preprocess_scene` |
| `bitmap_type_get_string` | bitmaps/bitmaps.c | Replaced |  | through its caller `rasterizer_set_texture` |
| `bitmap_3d_address` | bitmaps/bitmaps.c | Replaced |  | through its caller `bitmap_mipmap_address` |
| `bitmap_cube_map_address` | bitmaps/bitmaps.c | Replaced |  | through its caller `bitmap_mipmap_address` |
| `bitmap_mipmap_address` | bitmaps/bitmaps.c | Replaced |  | through its caller `bitmap_2d_get_pixel` |
| `bitmap_format_to_a8r8g8b8` | bitmaps/bitmaps.c | Replaced |  | through its caller `bitmap_2d_get_pixel` |
| `bitmap_mipmap_get_pixel_data_size` | bitmaps/bitmaps.c | Replaced |  | through its caller `rasterizer_bitmap_2d_changed` |
| `bitmap_mipmap_get_row_pitch` | bitmaps/bitmaps.c | Replaced |  | through its caller `rasterizer_xbox_bitmap_get_pixel_data_size` |
| `bitmap_2d_get_pixel` | bitmaps/bitmaps.c | Replaced |  | through its caller `sample_diffuse_texture` |

197 Missing (29 reached from `main_loop`), by file:

- bitmaps/bitmap_drawing.c: `bitmap_initialize_line`, `bitmap_step_line`, `bitmap_fill_rectangle`, `bitmap_copy`, `bitmap_tile_and_bevel_rectangle`, `bitmap_draw_line`, `bitmap_frame_rectangle`
- bitmaps/bitmap_extract.c: `extract_initialize`, `extract_find_sequence_bounds`, `extract_warn_about_horizontal_border`, `extract_plateless_cube_map`, `extract_find_bitmap_bounds`, `extract_get_bitmap_format`, `extract_pixels_to_mipmap`, `extract_pixels_from_mipmap`, `extract_build_texture_pages_by_sequence`, `extract_build_debug_plate`, `extract_mipmaps_to_bitmap`, `extract_add_bitmap`, `extract_3d_textures`, `extract_cube_maps`, `extract_sprites`, `extract_bitmap`, `extract_without_sequences`, `extract_sequence`, `extract_sequences`, `bitmaps_extract`, `bitmaps_extract_from_plate`
- bitmaps/bitmap_group.c: `code_00065210`, `code_00065220`, `code_00065240`, `bitmap_group_try_and_get_bitmap`, `bitmap_group_add_bitmap`, `postprocess_bitmap`, `delete_bitmap`, `postprocess_bitmap_group`
- bitmaps/bitmap_utilities.c: `bitmap_fill`, `bitmap_alpha_to_rgb`, `bitmap_clone`, `bitmap_2d_shrink`, `bitmap_3d_shrink`, `bitmap_cm_shrink`, `bitmap_fade`, `bitmap_2d_smooth`, `bitmap_3d_smooth`, `bitmap_cm_smooth`, `bitmap_2d_sharpen`, `bitmap_3d_sharpen`, `bitmap_cm_sharpen`, `bitmap_2d_alpha_bleed`, `bitmap_3d_alpha_bleed`, `bitmap_cm_alpha_bleed`, `bitmap_cm_height_map`, `bitmap_2d_compress_to_mipmap`, `bitmap_3d_compress_to_mipmap`, `bitmap_cm_compress_to_mipmap`, `bitmap_2d_uncompress_from_mipmap`, `bitmap_3d_uncompress_from_mipmap`, `bitmap_cm_uncompress_from_mipmap`, `real_rgb_color_brightness`, `rgb_color_to_hsv_color`, `hsv_color_to_rgb_color`, `real_rgb_color_to_real_hsv_color`, `real_hsv_color_to_real_rgb_color`, `argb_color_to_real_argb_color`, `rgb_color_to_real_rgb_color`, `valid_real_rgb_color`, `bitmap_shrink`, `bitmap_smooth`, `bitmap_sharpen`, `bitmap_alpha_bleed`, `bitmap_2d_height_map`, `bitmap_3d_height_map`, `bitmap_2d_vector_map`, `bitmap_3d_vector_map`, `bitmap_cm_vector_map`, `bitmap_compress_to_mipmap`, `bitmap_uncompress_from_mipmap`, `rgb_colors_interpolate`, `bitmap_height_map`, `bitmap_vector_map`
- bitmaps/bitmaps.c: `bitmap_format_get_string`, `bitmap_format_get_bits_per_pixel`, `bitmap_changed`, `bitmap_delete`, `bitmap_2d_address`, `bitmap_byte_swap_pixels`, `palette_find_closest_match`, `bitmap_format_type_valid_width`, `bitmap_format_type_valid_height`, `bitmap_format_type_valid_depth`, `bitmap_verify`, `bitmap_rebuild`, `bitmap_get_max_mipmap_count`, `bitmap_mipmap_get_width`, `bitmap_mipmap_get_height`, `bitmap_mipmap_get_depth`, `bitmap_mipmap_get_pixel_count`, `bitmap_get_pixel_count`, `bitmap_get_pixel_data_size`, `bitmap_2d_new`, `bitmap_3d_new`, `bitmap_cube_map_new`, `bitmap_3d_slice_extract`, `bitmap_3d_slice_insert`, `bitmap_cube_map_face_extract`, `bitmap_cube_map_face_insert`
- bitmaps/bitmaps_quantitize.c: `bitmap_quantitize_read_row`, `row_dither`, `bitmap_quantitize`
- bitmaps/libtiff/mkg3states.c: `DECLARE1`, `DECLARE3`, `DECLARE2`, `build_null_mode_tables`, `build_horiz_mode_tables`, `build_uncomp_mode_tables`
- bitmaps/libtiff/tif_compress.c: `findScheme`, `TIFFNoEncode`, `TIFFNoRowEncode`, `TIFFNoStripEncode`, `TIFFNoTileEncode`, `TIFFNoDecode`, `TIFFNoRowDecode`, `TIFFNoStripDecode`, `TIFFNoTileDecode`, `TIFFSetCompressionScheme`
- bitmaps/libtiff/tif_dirwrite.c: `TIFFWriteDirectory`, `TIFFWriteNormalTag`, `TIFFSetupShortLong`, `TIFFWriteRational`, `TIFFWritePerSampleShorts`, `TIFFSetupShortPair`, `TIFFWriteShortTable`, `TIFFWriteString`, `TIFFWriteShortArray`, `TIFFWriteLongArray`, `TIFFWriteRationalArray`, `TIFFWriteFloatArray`, `TIFFWriteJPEGQTables`, `TIFFWriteJPEGCTables`, `TIFFWriteTransferFunction`, `TIFFWriteData`, `TIFFLinkDirectory`
- bitmaps/libtiff/tif_error.c: `defaultHandler`, `TIFFSetErrorHandler`, `TIFFError`
- bitmaps/libtiff/tif_open.c: `TIFFInitOrder`, `getMode`, `TIFFOpen`, `TIFFFdOpen`, `TIFFScanlineSize`, `TIFFFileName`, `TIFFFileno`, `TIFFGetMode`, `TIFFIsTiled`, `TIFFCurrentRow`, `TIFFCurrentDirectory`, `TIFFCurrentStrip`, `TIFFCurrentTile`
- bitmaps/libtiff/tif_read.c: `TIFFReadScanline`, `TIFFSeek`, `TIFFReadEncodedStrip`, `TIFFReadRawStrip`, `TIFFReadRawStrip1`, `TIFFFillStrip`, `TIFFReadTile`, `TIFFReadEncodedTile`, `TIFFReadRawTile`, `TIFFReadRawTile1`, `TIFFFillTile`, `TIFFReadBufferSetup`, `TIFFStartStrip`, `TIFFStartTile`, `TIFFCheckRead`
- bitmaps/libtiff/tif_thunder.c: `ThunderDecode`, `ThunderDecodeRow`, `TIFFInitThunderScan`
- bitmaps/libtiff/tif_warning.c: `TIFFSetWarningHandler`, `TIFFWarning`
- bitmaps/s3tc/s3tc.c: `ColorToFcolor`, `FcolorToColor`, `ColorToRGB`, `Square3x3`, `Quantize`, `ClipExtrema`, `AllSame`, `EncodeBlockRGBColorKey`, `DecodeBlockRGB`, `DecodeBlockAlpha4`, `DecodeBlockAlpha3`, `EncodeBlockRGB`, `EncodeBlockAlpha4`, `EncodeBlockAlpha3`
- bitmaps/targa_file.c: `targa_export`
- bitmaps/tiff_file.c: `tiff_get_bounds`, `tiff_export`, `tiff_import`

## `bungie_net`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `add64` | bungie_net/common/64bit_math.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `code_0006e650` | bungie_net/common/64bit_math.c | Replaced | net/src/transport.rs | networking and Xbox Live |
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
| `code_0006f630` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `code_0006f700` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `code_0006f780` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
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
| `negate64` | bungie_net/common/64bit_math.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `x_exp_y_mod_n` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `generate_diffie_hellman_public_key` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |
| `generate_diffie_hellman_private_key` | bungie_net/common/public_key_crypt.c | Replaced | net/src/transport.rs | networking and Xbox Live |

## `cache`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `tag_files_open` | cache/cache_files.c | Replaced |  | through its caller `shell_initialize` |
| `tag_files_close` | cache/cache_files.c | Replaced |  | through its caller `shell_dispose` |
| `tag_groups_checksum` | cache/cache_files.c | Replaced |  | through its caller `network_game_generate_join_game_token` |
| `acquire_read_request` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_run_decompression` |
| `cache_copy_FileIOCompletionRoutine` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read` |
| `cache_copy_initialize` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_files_initialize` |
| `cache_files_dispose` | cache/cache_files_windows.c | Replaced |  | through its caller `tag_files_close` |
| `cache_file_read_io_completion_routine` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_file_windows_thread_proc` |
| `cache_file_promote_read` | cache/cache_files_windows.c | Replaced |  | through its caller `_texture_cache_bitmap_get_hardware_format` |
| `cached_map_issue_async_write` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_open_cache_files` |
| `cached_map_files_delete` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_open_cache_files` |
| `cache_files_verify_language` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_initialize` |
| `cache_file_windows_thread_proc` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_file_windows_thread_create` |
| `cache_files_open_cache_files` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_initialize` |
| `cache_file_windows_thread_create` | cache/cache_files_windows.c | Replaced |  | through its caller `cache_files_initialize` |
| `cache_files_initialize` | cache/cache_files_windows.c | Replaced |  | through its caller `tag_files_open` |
| `physical_memory_allocate` | cache/physical_memory_map.c | Replaced |  | through its caller `main` |
| `physical_memory_verify` | cache/physical_memory_map.c | Replaced |  | through its caller `shell_platform_verify` |
| `physical_memory_get_game_state_base_address` | cache/physical_memory_map.c | Replaced |  | through its caller `game_state_allocate_buffer` |
| `physical_memory_get_texture_cache_base_address` | cache/physical_memory_map.c | Replaced |  | through its caller `texture_cache_new` |
| `physical_memory_get_sound_cache_base_address` | cache/physical_memory_map.c | Replaced |  | through its caller `dsound_channel_queue_packet` |
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
| `cache_files_map_version` | cache/cache_files.c | Replaced |  | through its caller `network_game_server_change_map_name` |
| `cache_files_map_present` | cache/cache_files.c | Replaced |  | through its caller `network_game_client_game_settings_updated` |
| `cache_copy_stop_requested` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_run_decompression` |
| `cache_copy_initialize_zlib` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_dispose_zlib` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_compressed_alloc` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize` |
| `cache_copy_compressed_free` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize` |
| `cache_copy_initialize_read_buffers` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_initialize_file_data` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_block_on_raw_read` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize_read_data` |
| `cache_copy_block_on_raw_write` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize_read_data` |
| `cache_copy_read_buffer_size` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_initialize_read_buffers` |
| `cache_copy_get_read_buffer` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read_internal` |
| `cache_copy_get_write_buffer` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_write_internal` |
| `cache_copy_write_buffer_size` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_write_internal` |
| `any_bit_vector_flag_set` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_wait_for_async_io` |
| `cache_copy_wait_for_async_io` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `cache_copy_set_flag` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_FileIOCompletionRoutine` |
| `decompressor_reset_timing` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `simple_cache_copy_thread` |
| `decompressor_timer_start` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read` |
| `decompressor_timer_stop` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_issue_read` |
| `cache_copy_yield` | cache/cache_files_decompress_windows.c | Replaced |  | through its caller `cache_copy_run_decompression` |
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
| `sound_cache_sound_get_cache_sound` | cache/xbox_sound_cache.c | Replaced | audio/src/player.rs | Xbox-only hardware |
| `texture_cache_bitmap_valid` | cache/xbox_texture_cache.c | Replaced | app/Sources/OpenCEEngineRender/LoadedLevel.swift | Xbox-only hardware |

122 Missing (69 reached from `main_loop`), by file:

- cache/cache_files.c: `cache_files_map_directory`, `scenario_tags_unload`, `cache_files_get_checksum`, `tag_loaded`, `cache_files_enable_writes`, `cache_files_disable_writes`, `tag_block_resize`, `tag_data_resize`, `tag_block_add_element`, `tag_block_delete_element`, `tag_load`, `tag_unload`, `tag_file_get_path`, `tag_reference_set`, `tag_iterator_new`, `tag_iterator_next`, `cache_get_tag_instance`, `cache_file_header_verify`, `cache_files_give_time_to_precache`, `scenario_tags_load`, `scenario_structure_bsp_load`, `scenario_structure_bsp_unload`, `tag_get`, `tag_get_name`, `tag_get_group_tag`, `cache_empty_tag_instance`, `cache_files_set_tag_instances`, `cache_files_build_region`, `cache_files_multiplayer_region`, `cache_files_map_plays_multiplayer`, `cache_files_show_multiplayer_unavailable`, `tag_index_is_group`
- cache/cache_files_decompress_windows.c: `code_001a9c10`, `cache_copy_buffer_size`, `cache_copy_set_priority`, `cache_copy_compressed_file_complete`, `cache_copy_begin`, `cache_copy_queue_end`, `code_001a9fe0`, `code_001aa010`, `code_001aa040`, `code_001aa0a0`, `code_001aa0f0`, `code_001aa1a0`, `code_001aa290`, `code_001aa310`, `code_001aa420`, `code_001aa430`, `code_001aa480`, `code_001aa4d0`, `code_001aa510`, `code_001aa540`, `code_001aa5e0`, `code_001aa610`, `code_001aa620`, `code_001aa640`, `code_001aa650`, `code_001aa680`, `code_001aa780`, `cache_copy_get_status`, `cache_copy_end`, `code_001aab70`, `code_001aacb0`, `code_001aadf0`, `code_001aae00`, `code_001aae10`, `code_001aaf10`, `code_001aaf30`, `code_001aaf80`, `code_001ab020`, `code_001ab1b0`, `code_001ab280`, `code_001ab350`, `code_001ab540`, `code_001ab880`, `cache_copy_get_flags`, `cache_copy_print_timing`
- cache/cache_files_windows.c: `cached_map_issue_async_request`, `cached_map_block_on_async_request`, `cache_request_get`, `cache_request_next_free_index`, `cache_requests_flush`, `cache_files_precache_set_priority`, `cache_files_precache_in_progress`, `cache_files_precache_is_copying_map`, `cache_files_precache_map_queue_end`, `cached_map_file_get`, `cached_map_file_invalidate`, `cached_map_file_get_handle`, `cached_map_file_get_size`, `cached_map_file_get_path`, `cache_file_windows_thread_wake`, `cache_file_blocking_io_completion_routine`, `cache_file_get_map_path`, `IDirect3DVertexBuffer8_IsBusy`, `IDirect3DVertexBuffer8_BlockUntilNotBusy`, `IDirect3DVertexBuffer8_Register`, `IDirect3DIndexBuffer8_IsBusy`, `IDirect3DIndexBuffer8_BlockUntilNotBusy`, `cache_file_close`, `cache_file_read`, `cache_file_read_header_from_dvd`, `cached_map_issue_async_read`, `cache_file_block_until_not_busy`, `tags_header_register_vertex_and_index_buffers`, `tags_header_deregister_vertex_and_index_buffers`, `structure_bsp_header_register_vertex_buffers`, `structure_bsp_header_deregister_vertex_buffers`, `cache_files_precache_map_status`, `cached_map_file_set_modification_date`, `cached_map_file_read_header`, `cached_map_files_find_map`, `cached_map_files_find_free_map`, `cache_file_open`, `cache_files_precache_map_loaded`, `cache_files_precache_map_begin`, `cache_files_precache_map_end`
- cache/physical_memory_map.c: `physical_memory_free`, `physical_memory_get_tag_cache_base_address`
- cache/predicted_resources.c: `code_001ad870`, `predicted_resources_precache`, `predicted_resources_sound_precache`

## `camera`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `scripted_camera_update` | camera/camera_scripting.c | Matched | campaign/src/cinematic.rs:122 | owed: cutscene cameras; reads the object, tag and map world, which the fake worlds do not hold; campaign/tests/cutscenes.rs |
| `dead_camera_new` | camera/dead_camera.c | Matched | sim/src/dead_camera.rs:46 | owed: the engine cites it, with no check against the original yet |
| `dead_camera_update` | camera/dead_camera.c | Matched | sim/src/lib.rs:1399 | owed: the engine cites it, with no check against the original yet |
| `director_desired_perspective` | camera/director.c | Matched | sim/src/seats.rs:1223 | owed: the camera; reads the object, tag and map world, which the fake worlds do not hold (the unit's seat); Phase 14's death camera |
| `uniform_cubic_spline` | camera/following_camera.c | Matched | sim/src/player.rs:1631 | owed: the engine cites it, with no check against the original yet |
| `following_camera_update` | camera/following_camera.c | Matched | sim/src/player.rs:765 | owed: the camera; reads the object, tag and map world, which the fake worlds do not hold; Phase 14's death camera |
| `observer_up_from_forward` | camera/observer.c | Matched | sim/src/dead_camera.rs:95 | owed: the engine cites it, with no check against the original yet |
| `camera_track_splut` | camera/following_camera.c | Matched | sim/src/player.rs:1597 | owed: cutscene cameras; reads the object, tag and map world, which the fake worlds do not hold (the camera track tag); campaign/tests/cutscenes.rs |
| `observer_update_polynomial` | camera/observer.c | Matched | sim/src/dead_camera.rs:9 | owed: the engine cites it, with no check against the original yet |

137 Missing (77 reached from `main_loop`), by file:

- camera/bored_camera.c: `bored_camera_new`, `code_00073000`, `code_00073020`, `is_bored`, `is_still_bored`, `real_local_random_range`, `valid_real_vector3d`, `valid_real_vector3d_axes2`, `bored_camera_update`, `bored_camera_shot_threshold_milliseconds`, `bored_camera_shot_duration_milliseconds`
- camera/camera_scripting.c: `scripted_camera_enable`, `scripted_camera_set_animation`, `scripted_camera_set_first_person`, `scripted_camera_set_dead`, `scripted_camera_object_is_first_person_camera`, `scripted_camera_set`, `scripted_camera_set_absolute`, `scripted_camera_set_camera_point_relative`, `scripted_camera_set_camera_point_absolute`, `scripted_camera_next_camera_point`, `scripted_camera_object_relative_to`, `scripted_camera_time`
- camera/dead_camera.c: `player_has_allies`, `player_get_next_player_with_a_unit`
- camera/director.c: `director_get`, `director_initialize`, `director_dispose`, `director_inhibit_facing`, `director_inhibit_input`, `director_inhibited_facing`, `director_inhibited_input`, `director_set_mode`, `director_save_camera`, `director_get_perspective`, `director_set_camera`, `director_initialize_variables`, `director_process_variables`, `director_dispose_from_old_map`, `director_load_camera`, `director_rotate_cameras`, `director_camera_deterministic`, `director_choose_game_perspective`, `director_script_camera`, `director_choose_camera_game`, `director_choose_camera_editor`, `director_choose_camera_script_camera_record`, `director_update_controls`, `director_choose_camera`, `director_initialize_for_new_map`, `director_update`, `director_initialize_for_saved_game`
- camera/editor_flying_camera.c: `editor_camera_new`, `editor_camera_get_focus`, `editor_camera_set_focus`, `editor_camera_set_position`, `editor_camera_bump_speed`, `editor_camera_get_speed`, `editor_camera_use_roll`, `editor_camera_get_unit_focus`, `editor_camera_set_mode`, `editor_camera_get_mode`, `editor_camera_get_scripted`, `code_00076270`, `editor_camera_get_field_of_view`, `editor_camera_move_to_point`, `editor_camera_set_position_and_roll`, `editor_camera_set_unit_focus`, `editor_camera_update`, `editor_camera_set_scripted`, `code_00076870`, `code_000768d0`, `code_00076e50`, `editor_camera_flying_update`, `editor_camera_orbiting_update`, `translate_orbiting_to_flying`, `translate_flying_to_orbiting`
- camera/first_person_camera.c: `first_person_camera_new`, `first_person_camera_deterministic`, `code_000773c0`, `first_person_camera_fake`, `first_person_camera_update`, `first_person_camera_for_unit_and_vector`
- camera/flying_camera.c: `flying_camera_new`, `flying_camera_new_from_point_and_vector`, `flying_camera_update`
- camera/following_camera.c: `following_camera_new`, `code_00077f20`, `arcsine`, `uniform_cubic_spline_vector3d`, `code_00078120`, `following_camera_deterministic`, `unit_camera_get`
- camera/observer.c: `code_00078970`, `observer_initialize`, `code_000789c0`, `observer_initialize_for_new_map`, `observer_dispose_from_old_map`, `observer_get_camera`, `observer_command_has_finished`, `observer_reconnect_to_structure_bsp`, `code_00078cd0`, `code_00078ea0`, `observer_obsolete_position`, `code_00079180`, `code_00079200`, `code_000792e0`, `observer_set_camera`, `code_000796d0`, `code_00079ae0`, `code_0007a080`, `code_0007a6a0`, `code_0007a7c0`, `code_0007aab0`, `code_0007ab20`, `code_0007b360`, `code_0007b3b0`, `observer_update`, `observer_get`, `observer_clear`, `observer_update_accelerations`, `observer_update_velocities`, `observer_collision_test_with_t`, `observer_collision_test_differential`, `observer_update_command`, `observer_apply_rotational_displacement`, `observer_update_positions`, `observer_rotational_displacement`, `observer_find_displacement`, `observer_check_penetration`, `observer_postcheck`, `observer_update_displacements`, `observer_pass_time`
- camera/orbiting_camera.c: `orbiting_camera_new`, `orbiting_camera_update`
- camera/static_camera.c: `static_camera_new`, `static_camera_update`

## `cseries`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `cseries_initialize` | cseries/cseries.c | Replaced |  | through its caller `shell_initialize` |
| `cseries_dispose` | cseries/cseries.c | Replaced |  | through its caller `shell_dispose` |
| `strnlen` | cseries/cseries.c | Replaced |  | through its caller `data_encode_string` |
| `system_seconds` | cseries/cseries_windows.c | Replaced |  | through its caller `get_number_suitable_for_initializing_random_seed` |
| `system_memory_information_get` | cseries/cseries_windows.c | Replaced |  | through its caller `render_debug_profile` |
| `exception_code_get_string` | cseries/cseries_windows.c | Replaced |  | through its caller `generic_exception_filter` |
| `generic_exception_filter` | cseries/cseries_windows.c | Replaced |  | through its caller `main` |
| `debug_memory_manager_initialize` | cseries/debug_memory.c | Replaced |  | through its caller `cseries_initialize` |
| `debug_check_memory` | cseries/debug_memory.c | Replaced |  | through its caller `debug_dump_memory_for_file` |
| `debug_dump_memory_for_file` | cseries/debug_memory.c | Replaced |  | through its caller `debug_dump_memory` |
| `debug_dump_memory` | cseries/debug_memory.c | Replaced |  | through its caller `cseries_dispose` |
| `errors_dispose` | cseries/errors.c | Replaced |  | through its caller `shell_dispose` |
| `errors_initialize` | cseries/errors.c | Replaced |  | through its caller `shell_initialize` |
| `profile_initialize` | cseries/profile.c | Replaced |  | through its caller `cseries_initialize` |
| `profile_rasterizer_stats` | cseries/profile.c | Replaced |  | through its caller `rasterizer_profile_frame_end` |
| `compare_profile_sections` | cseries/profile.c | Replaced |  | through its caller `profile_dump` |
| `profile_dump` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_find_frame_value` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_frame_get_value` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_frame_iterator_new` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_frame_iterator_next` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_frame_get_messages` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_frame_get_stalls` | cseries/profile.c | Replaced |  | through its caller `render_debug_profile` |
| `profile_render_window_start` | cseries/profile.c | Replaced |  | through its caller `render_frame_pregame` |
| `profile_render_window_end` | cseries/profile.c | Replaced |  | through its caller `render_frame_pregame` |
| `profile_texture_start` | cseries/profile.c | Replaced |  | through its caller `rasterizer_set_texture_bitmap_data` |
| `profile_texture_end` | cseries/profile.c | Replaced |  | through its caller `rasterizer_set_texture_bitmap_data` |
| `qsort_2byte` | cseries/sort.c | Replaced |  | through its caller `rasterizer_frame_statistics_count_dynamic_vertices` |
| `symbol_sort_proc` | cseries/stack_walk_windows.c | Replaced |  | through its caller `load_symbol_table` |
| `free_symbol_table` | cseries/stack_walk_windows.c | Replaced |  | through its caller `load_symbol_table` |
| `stack_walk_dispose` | cseries/stack_walk_windows.c | Replaced |  | through its caller `errors_dispose` |
| `load_symbol_table` | cseries/stack_walk_windows.c | Replaced |  | through its caller `stack_walk_initialize` |
| `stack_walk_initialize` | cseries/stack_walk_windows.c | Replaced |  | through its caller `errors_initialize` |
| `shortsort_2byte` | cseries/sort.c | Replaced |  | through its caller `qsort_2byte` |

117 Missing (73 reached from `main_loop`), by file:

- cseries/cseries.c: `string_to_tag`, `tag_to_string`, `strnupr`, `strnlwr`, `strupr`, `strlwr`, `csprintf`, `display_assert`, `csmemcmp`, `csmemmove`, `csmemset`, `csstrcat`, `csstrcmp`, `csstrncat`, `csstrncmp`, `csstrncpy`, `csstrtok`, `csstrlen`, `csstrcpy`, `csmemcpy`, `csstrcasecmp`, `stristr`, `string_hash`
- cseries/cseries_windows.c: `display_debug_string`, `system_exit`, `system_unique_identifier_get`, `system_unique_identifiers_equal`, `system_milliseconds`, `system_get_user_name`, `system_calloc`, `system_malloc`, `system_free`, `system_realloc`, `system_get_used_memory_size`, `system_show_wait_cursor`, `system_alert`, `system_kill_screen_saver`
- cseries/debug_memory.c: `code_0007ccf0`, `code_0007cd40`, `code_0007cd90`, `code_0007cdc0`, `check_memory_status`, `local_random`, `code_0007ce40`, `code_0007cf50`, `code_0007cfc0`, `code_0007d060`, `debug_dump_memory_by_file`, `debug_malloc`, `debug_free`, `debug_realloc`, `debug_check_memory_globals`, `debug_memory_header_checksum`, `compare_file_pointer_totals`, `debug_check_pointer_header`, `debug_check_pointer_overrun`, `debug_memory_fill_with_random`, `debug_memory_add_pointer`, `debug_memory_remove_pointer`
- cseries/errors.c: `errors_output_to_debug_file`, `errors_overflow_suppression_enable`, `error_get`, `write_to_error_file`, `code_0007d9d0`, `error`, `errors_handle`, `errors_clear`, `write_to_debug_file`, `reset_error_state`
- cseries/profile.c: `profile_internal_step`, `profile_timesection_inherit`, `profile_seconds_elapsed`, `profile_lapsed_frames`, `profile_lapsed_msec`, `find_profile_section`, `profile_enter_private`, `profile_exit_private`, `profile_describe_frame`, `profile_timesection_begin`, `profile_timesection_end`, `profile_dump_to_file`, `profile_dump_frame`, `profile_dump_frame_stop`, `string_has_prefix`, `profile_sections_activation`, `profile_sections_activate`, `profile_sections_deactivate`, `profile_find_game_value`, `profile_rasterizer_stalls`, `profile_timesection_begin_now`, `profile_timesection_end_now`, `profile_tick_start`, `profile_tick_end`, `profile_render_start`, `profile_render_end`, `profile_frame_start`, `profile_frame_end`, `profile_idle_start`, `profile_idle_end`, `profile_append`
- cseries/progress.c: `progress_new`, `progress_update`
- cseries/sort.c: `code_00080360`, `code_000803c0`, `qsort_4byte`, `shortsort_4byte`
- cseries/stack_walk_windows.c: `stack_walk_global_function_offset`, `stack_walk_disregard_symbol_names`, `symbol_name_from_address`, `base_address_from_symbol_name`, `is_valid_ebp`, `walk_up`, `initialize_stack_walk`, `walk_stack_context`, `walk_stack`, `stack_walk_with_context`, `stack_walk`

## `cutscene`


81 Missing (40 reached from `main_loop`), by file:

- cutscene/cinematics.c: `cinematic_initialize`, `cinematic_dispose`, `cinematic_initialize_for_new_map`, `cinematic_dispose_from_old_map`, `cinematic_start`, `cinematic_can_be_skipped`, `cinematic_skip_start`, `cinematic_skip_stop`, `cinematic_show_letterbox`, `draw_quad`, `cinematic_force_title`, `cinematic_suppress_bsp_object_creation`, `cinematic_stop`, `cinematic_in_progress`, `cinematic_set_title_delayed`, `cinematic_render`, `cinematic_set_title`
- cutscene/recorded_animation_definitions.c: `code_00081cd0`, `scenario_get_animation_by_name`, `byte_swap_recording`
- cutscene/recorded_animation_initialize.c: `recorded_animation_byteswap_unit_control`, `recorded_animation_initialize_unit_control`, `recorded_animation_write_unit_control`
- cutscene/recorded_animation_playback.c: `code_00081ef0`, `code_00081f80`, `code_00082010`, `code_000820a0`, `code_00082130`, `code_000821d0`, `code_00082210`, `code_00082250`, `code_00082290`, `code_00082490`, `recorded_animation_initialize_event_stream`, `recorded_animation_initialize_event_stream_with_size`, `recorded_animation_apply_event_stream`, `byte_swap_recording_stream`, `apply_animation_state`, `apply_aiming_speed`, `apply_control_flags`, `apply_weapon_index`, `apply_throttle`, `update_controller_char`, `update_controller_short`, `uncompress_vector_from_controller`, `apply_vector_char_difference`, `apply_vector_short_difference`
- cutscene/recorded_animation_playback_v1.c: `code_00082910`, `code_000829c0`, `code_00082a70`, `code_00082b20`, `code_00082bd0`, `code_00082c90`, `code_00082d50`, `code_00082e10`, `code_00082ed0`, `code_00082fe0`, `recorded_animation_initialize_event_stream_v1`, `recorded_animation_apply_event_stream_v1`, `byte_swap_recording_stream_v1`, `apply_facing_vector`, `apply_aiming_vector`, `apply_looking_vector`, `apply_angle_vector`, `apply_multi_vector`
- cutscene/recorded_animations.c: `recorded_animations_initialize`, `recorded_animations_dispose`, `recorded_animations_dispose_from_old_map`, `recorded_animations_clear_debug_storage`, `recorded_animations_update`, `recorded_animation_verify`, `recorded_animation_controlling_unit`, `get_controlling_thread`, `render_debug_recording`, `recorded_animations_initialize_for_new_map`, `recorded_animation_kill`, `recorded_animation_play_internal`, `recorded_animation_get_time_left`, `recorded_animation_play`, `recorded_animation_play_and_delete`, `recorded_animation_play_and_hover`

## `devices`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `machine_update` | devices/device_machines.c | Matched | campaign/src/devices.rs:170 | owed: the engine cites it, with no check against the original yet |
| `device_export_function_values` | devices/devices.c | Matched | sim/src/device_animation.rs:5 | owed: devices; reads the object, tag and map world, which the fake worlds do not hold |
| `device_preprocess_node_orientations` | devices/devices.c | Matched | sim/src/device_animation.rs:6 | owed: devices; reads the object, tag and map world, which the fake worlds do not hold (the device's model) |
| `device_get_position` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_get_power` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_set_never_appears_locked` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_one_sided_set` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_operates_automatically_set` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_group_change_only_once_more_set` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_group_get_value` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `create_initial_device_groups` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_update` | devices/devices.c | Matched | campaign/src/devices.rs:170 | owed: the engine cites it, with no check against the original yet |
| `device_group_set_desired_value` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_set_actual_position` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_set_desired_position` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_set_power` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |
| `device_group_change` | devices/devices.c | Matched | campaign/src/devices.rs | owed: the engine's own tests (campaign/src/devices.rs); no check against the original yet |

43 Missing (42 reached from `main_loop`), by file:

- devices/device_controls.c: `controls_initialize`, `controls_dispose`, `controls_initialize_for_new_map`, `controls_dispose_from_old_map`, `control_place`, `control_new`, `control_delete`, `control_update`, `control_toggle`, `control_touched`, `control_destroyed`
- devices/device_light_fixtures.c: `light_fixtures_initialize`, `light_fixtures_dispose`, `light_fixtures_initialize_for_new_map`, `light_fixtures_dispose_from_old_map`, `light_fixture_place`, `light_fixture_new`, `light_fixture_delete`, `light_fixture_update`
- devices/device_machines.c: `machines_initialize`, `machines_dispose`, `machines_initialize_for_new_map`, `machines_dispose_from_old_map`, `machine_place`, `machine_new`, `machine_delete`, `machine_bumped`, `machine_try_to_open_with_damage`
- devices/devices.c: `devices_initialize`, `devices_dispose`, `devices_dispose_from_old_map`, `device_new`, `device_group_set_actual_value`, `device_touched`, `device_can_change_position`, `device_effect_new`, `device_group_new`, `device_group_delete`, `devices_initialize_for_new_map`, `device_delete`, `device_render_debug`, `device_add_scenario_information`, `device_frontfacing`

## `dialogs`


6 Missing (5 reached from `main_loop`), by file:

- dialogs/edit_text.c: `code_00085920`, `edit_text_selection_reset`, `edit_text_get_selection_indices`, `edit_text_new`, `edit_text_handle_key`, `edit_text_fix_selection`

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
| `contrail_scale_value` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:32 | owed: the engine cites it, with no check against the original yet |
| `contrail_delete` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | owed: the engine's own tests (ffi/src/particles/contrails.rs); no check against the original yet |
| `contrail_compute_new_point_count` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | owed: the engine's own tests (ffi/src/particles/contrails.rs); no check against the original yet |
| `local_random_range` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | owed: the engine's own tests (ffi/src/particles/contrails.rs); no check against the original yet |
| `local_random_vector_in_cone3d` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | owed: the engine's own tests (ffi/src/particles/contrails.rs); no check against the original yet |
| `contrail_scale_random_value` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:42 | owed: the engine cites it, with no check against the original yet |
| `contrail_next_frame` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:252 | owed: the engine cites it, with no check against the original yet |
| `contrail_add_points` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:298 | owed: the engine cites it, with no check against the original yet |
| `contrail_update_points` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs:122 | owed: the engine cites it, with no check against the original yet |
| `contrail_new` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | owed: the engine's own tests (ffi/src/particles/contrails.rs); no check against the original yet |
| `contrails_update` | effects/contrails.c | Matched | ffi/src/particles/contrails.rs | owed: the engine's own tests (ffi/src/particles/contrails.rs); no check against the original yet |
| `decal_sprite_get_bounds` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `render_debug_decals` | effects/decals.c | Replaced | dev/src/draw.rs | the editor and tools |
| `projection_from_vector3d` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `projection_sign_from_vector3d` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `project_point2d` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `triple_product3d` | effects/decals.c | Matched | sim/src/vehicle_animation.rs:73 | owed: the engine cites it, with no check against the original yet |
| `plane2d_from_points` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `bsp3d_get_plane_from_designator` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `decal_update` | effects/decals.c | Matched | ffi/src/dynamic_decals.rs | owed: the engine's own tests (ffi/src/dynamic_decals.rs); no check against the original yet |
| `decal_insert` | effects/decals.c | Matched | ffi/src/dynamic_decals.rs | owed: the engine's own tests (ffi/src/dynamic_decals.rs); no check against the original yet |
| `decals_update` | effects/decals.c | Matched | ffi/src/dynamic_decals.rs:4 | owed: the engine cites it, with no check against the original yet |
| `decal_delete` | effects/decals.c | Matched | ffi/src/dynamic_decals.rs | owed: the engine's own tests (ffi/src/dynamic_decals.rs); no check against the original yet |
| `decal_projection_create` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `decal_clip_to_surface` | effects/decals.c | Replaced | app/Sources/OpenCEEngineRender/DecalPass.swift | the enhanced lighting and Metal renderer |
| `decal_new_from_collision` | effects/decals.c | Matched | ffi/src/decals.rs:179 | owed: the engine cites it, with no check against the original yet |
| `decal_new` | effects/decals.c | Matched | ffi/src/decals.rs:144 | owed: the engine cites it, with no check against the original yet |
| `effect_delete` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_scale` | effects/effects.c | Matched | content/src/weapon.rs:602 | owed: the engine cites it, with no check against the original yet |
| `effect_get_random_seed` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_real_random_range` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_random_angular_velocity` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_allowed_by_environment` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_part_allowed_by_disposition` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_set_event` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_build_location` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_location_get_next_instance` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `real_local_random` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `local_random_direction3d` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_stop` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_random_translational_velocity` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_allocate` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_build_locations` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_generate_particles` | effects/effects.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `effect_generate_part` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_marker_list_get_marker` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_generate_parts` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_marker_list_get_markers_by_name` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_update` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_new_looping` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_new_from_object` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_new_attached_from_markers` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `effect_new_unattached_from_markers` | effects/effects.c | Matched | sim/src/effects.rs:64 | owed: particles; reads the object, tag and map world, which the fake worlds do not hold (the effect tag); Phase 12's explosions |
| `effects_update` | effects/effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `material_effect_new` | effects/material_effects.c | Matched | ffi/src/sound_events.rs:210 | owed: the engine cites it, with no check against the original yet |
| `material_effect_new_from_point` | effects/material_effects.c | Matched | sim/src/effects.rs | owed: the engine's own tests (sim/src/effects.rs); no check against the original yet |
| `particle_system_orphan` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_delete` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_next_type_state_index` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_next_particle_state_index` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_update_default` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_new_particle_default` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_update_particle_default` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_update_explosion` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_new_particles` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `randomize_particle_variables` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_update` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_render` | effects/particle_systems.c | Replaced | app/Sources/OpenCEEngineRender/ParticlePass.swift | the enhanced lighting and Metal renderer |
| `particle_system_new_particle_explosion` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_new_particle_jet` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_initialize` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_systems_update` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_systems_render` | effects/particle_systems.c | Replaced | app/Sources/OpenCEEngineRender/ParticlePass.swift | the enhanced lighting and Metal renderer |
| `particle_system_new_unattached` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_system_new_attached` | effects/particle_systems.c | Matched | ffi/src/particles/systems.rs | owed: the engine's own tests (ffi/src/particles/systems.rs); no check against the original yet |
| `particle_delete` | effects/particles.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `new_particle_is_visible` | effects/particles.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `particle_get_radius` | effects/particles.c | Matched | content/src/particle.rs:155 | owed: the engine cites it, with no check against the original yet |
| `particle_effect_new` | effects/particles.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `particle_die` | effects/particles.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `particle_next_sequence` | effects/particles.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `particle_next_frame` | effects/particles.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `particle_update_frame_time` | effects/particles.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `particle_update_physics` | effects/particles.c | Matched | ffi/src/particles.rs:514 | owed: the engine cites it, with no check against the original yet |
| `particle_new` | effects/particles.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `particles_update` | effects/particles.c | Matched | ffi/src/particles.rs | owed: the engine's own tests (ffi/src/particles.rs); no check against the original yet |
| `weather_particle_system_get` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_get_type` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_type_delete_particle` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_wrap_point` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_new` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_delete` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_new_particle` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_box_offset_from_point3d` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_update_physics` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_build_clipping_planes` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_transform_clip_planes_to_box` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_polyhedra_find` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_update_particle_count` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_update` | effects/weather_particle_systems.c | Matched | ffi/src/particles/weather.rs | owed: the engine's own tests (ffi/src/particles/weather.rs); no check against the original yet |
| `weather_particle_system_render` | effects/weather_particle_systems.c | Replaced | app/Sources/OpenCEEngineRender/ParticlePass.swift | the enhanced lighting and Metal renderer |
| `weather_particle_systems_render` | effects/weather_particle_systems.c | Replaced | app/Sources/OpenCEEngineRender/ParticlePass.swift | the enhanced lighting and Metal renderer |

88 Missing (87 reached from `main_loop`), by file:

- effects/contrails.c: `contrails_initialize_for_new_map`, `contrails_dispose_from_old_map`, `contrails_dispose`, `contrails_disconnect_from_structure_bsp`, `contrails_reconnect_to_structure_bsp`, `contrail_verify`, `contrails_initialize`, `contrail_owner_collision`
- effects/decals.c: `decal_check`, `decal_set_first_decal_index`, `decals_initialize`, `decals_initialize_for_new_map`, `decals_dispose_from_old_map`, `decals_dispose`, `decals_unlock`, `decal_get_first_decal_index`, `decal_new_from_media_collision`, `plane3d_from_point_and_normal`, `plane3d_negate`, `plane3d_distance_to_point`, `real_a_rgb_color_to_pixel32`, `decal_reinsert`, `decals_reconnect_to_structure_bsp`, `decals_disconnect_from_structure_bsp`, `decals_delete_permanent_from_cluster`
- effects/effects.c: `effects_initialize`, `effects_initialize_for_new_map`, `effects_dispose_from_old_map`, `effects_dispose`, `effects_object_is_corpse`, `effects_stop_on_first_person_weapon`, `effects_information_get`, `effects_disconnect_from_structure_bsp`, `effect_get_node_matrix`, `effect_evaluate_function_integral`, `effects_reconnect_to_structure_bsp`, `dangerous_effects_near_player`, `impulse_effect_initialize`, `effects_start_on_first_person_weapon`
- effects/material_effects.c: `material_effect_visible`
- effects/particle_systems.c: `particle_systems_initialize`, `particle_systems_initialize_for_new_map`, `particle_systems_dispose`, `particle_systems_disconnect_from_structure_bsp`, `particle_systems_reconnect_to_structure_bsp`, `particle_systems_dispose_from_old_map`
- effects/particles.c: `particles_initialize`, `particles_initialize_for_new_map`, `particles_dispose_from_old_map`, `particles_dispose`, `particles_stop_on_first_person_weapon`, `particles_disconnect_from_structure_bsp`, `particles_reconnect_to_structure_bsp`, `valid_real_point3d`, `valid_real_argb_color`
- effects/player_effects.c: `player_effect_get`, `player_effect_initialize`, `player_effect_dispose`, `player_effect_initialize_for_new_map`, `player_effect_dispose_from_old_map`, `player_effect_add_continuous_effect`, `scripted_player_effect_set_rotation`, `scripted_player_effect_set_rumble`, `player_telefrag_effect_stop`, `player_effect_screen_fade_in`, `player_effect_screen_fade_out`, `player_effect_get_damage_indicators`, `player_effect_clear_damage_indicators`, `effect_scale_factor`, `player_effect_update_screen_flash`, `player_effect_update_camera_shake`, `effect_scale_value`, `player_effect_update`, `player_effect_continuous_refresh`, `scripted_player_effect_set_translation`, `scripted_player_effect_start`, `scripted_player_effect_stop`, `player_effect_screen_flash`, `player_telefrag_effect_start`, `player_effect_get_screen_flash`, `get_shake_matrix`, `player_effect_get_camera_effect_matrix`, `player_effect_update_camera_impulse`, `player_effect_start`
- effects/weather_particle_systems.c: `weather_particle_systems_initialize`, `weather_particle_systems_initialize_for_new_map`, `weather_particle_systems_dispose_from_old_map`, `weather_particle_systems_dispose`

## `game`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `game_tick` | game/game.c | Matched | sim/src/lib.rs:382 | owed: the engine's own tests (sim/tests); no check against the original yet |
| `game_all_quiet` | game/game.c | Partial | campaign/src/game.rs | leaves out the game's own test: the engine's always passes; owed: the engine's own tests (campaign/src/game.rs); no check against the original yet |
| `game_safe_to_save` | game/game.c | Partial | campaign/src/game.rs | leaves out the game's own test: the engine's always passes; owed: the engine's own tests (campaign/src/game.rs); no check against the original yet |
| `game_safe_to_speak` | game/game.c | Partial | campaign/src/game.rs | leaves out the game's own test: the engine's always passes; owed: the engine's own tests (campaign/src/game.rs); no check against the original yet |
| `game_is_cooperative` | game/game.c | Matched | campaign/src/game.rs | owed: the engine's own tests (campaign/src/game.rs); no check against the original yet |
| `game_engine_get_team_score` | game/game_engine.c | Matched | sim/src/game.rs:282 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `game_engine_game_starting` | game/game_engine.c | Matched | sim/src/game.rs:526 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `build_game_variant_slayer` | game/game_engine.c | Matched | sim/src/game.rs:64 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `build_game_variant_team_slayer` | game/game_engine.c | Matched | sim/src/game.rs:78 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `build_game_variant_ctf` | game/game_engine.c | Matched | sim/src/game.rs:86 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `game_engine_should_end_game` | game/game_engine.c | Matched | sim/src/game.rs | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `random_item` | game/game_engine.c | Matched | sim/src/game.rs | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `match_game_type` | game/game_engine.c | Matched | content/src/level.rs:460 | owed: the engine cites it, with no check against the original yet |
| `game_engine_update_item_spawn` | game/game_engine.c | Matched | sim/src/game.rs:740 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `default_starting_location_rate_function` | game/game_engine.c | Partial | sim/src/game.rs | leaves out the rating's weights: the engine takes the spawn farthest from its living enemies; owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `game_engine_get_starting_location_rating` | game/game_engine.c | Partial | sim/src/game.rs:658 | leaves out the rating's weights: the engine takes the spawn farthest from its living enemies; owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `game_engine_player_added` | game/game_engine.c | Matched | sim/src/game.rs | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `game_engine_update` | game/game_engine.c | Partial | sim/src/game.rs:596 | leaves out the oddball, king, race and juggernaut variants, the game's announcer and score messages; owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `game_engine_player_killed` | game/game_engine.c | Partial | sim/src/game.rs:311 | leaves out the oddball, king, race and juggernaut variants, the game's announcer and score messages; owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `ctf_award_capture` | game/game_engine_ctf.c | Matched | sim/src/game.rs | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `ctf_engine_player_killed_player` | game/game_engine_ctf.c | Partial | sim/src/game.rs:312 | leaves out the oddball, king, race and juggernaut variants, the game's announcer and score messages; owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `ctf_get_score` | game/game_engine_ctf.c | Matched | sim/src/game.rs:282 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `ctf_reset_flag` | game/game_engine_ctf.c | Matched | sim/src/game.rs | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `ctf_player_drop_flag` | game/game_engine_ctf.c | Matched | sim/src/game.rs | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `ctf_engine_update` | game/game_engine_ctf.c | Matched | sim/src/game.rs:368 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `slayer_engine_update` | game/game_engine_slayer.c | Matched | sim/src/game.rs:368 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `slayer_get_score` | game/game_engine_slayer.c | Matched | sim/src/game.rs:282 | owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `slayer_engine_player_killed_player` | game/game_engine_slayer.c | Partial | sim/src/game.rs:311 | leaves out the oddball, king, race and juggernaut variants, the game's announcer and score messages; owed: the engine's own tests (sim/tests/game.rs); no check against the original yet |
| `game_time_update` | game/game_time.c | Partial | sim/src/clock.rs:35 | leaves out Halo's 30 Hz tick: the engine's simulation runs at 60 Hz; owed: the engine's own tests (sim/src/clock.rs); no check against the original yet |
| `handle_one_player_input` | game/player_control.c | Matched | sim/src/player.rs:578 | owed: the engine's own tests (sim/tests/player.rs); no check against the original yet |
| `player_control_update` | game/player_control.c | Matched | sim/src/player.rs:578 | owed: the engine's own tests (sim/tests/player.rs); no check against the original yet |
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
| `player_examine_nearby_objects` | game/players.c | Matched | sim/src/hud.rs:318 | owed: the action button; reads the object, tag and map world, which the fake worlds do not hold |
| `players_update_after_game` | game/players.c | Matched | campaign/src/lib.rs:474 | owed: the end of a level; reads the object, tag and map world, which the fake worlds do not hold; campaign/tests/flow.rs |

675 Missing (590 reached from `main_loop`), by file:

- game/aim_assist.c: `compute_attenuation`, `compute_composite_attenuation`, `unit_get_aim_assist_parameters`, `compare_targets`, `reciprocal_square_root`, `limit3d`, `set_real_euler_angles2d`, `aim_assist_clear_line_of_sight`, `object_compute_autoaim_target`, `aim_assist_compute_target`, `autoaim_compute_target`, `find_aim_assist_targets_recursive`, `find_aim_assist_targets`, `aim_assist`, `player_aim_projectile`, `local_player_aim_assist`
- game/cheats.c: `cheats_initialize`, `cheats_dispose`, `cheats_dispose_from_old_map`, `cheats_update`, `cheats_load`, `cheat_active_camouflage_local_player`, `cheat_player_index`, `cheats_initialize_for_new_map`, `cheat_teleport_to_camera`, `cheat_active_camouflage`, `cheat_objects`, `cheat_all_weapons`, `cheat_all_powerups`, `cheat_all_vehicles`
- game/game.c: `game_initialize`, `game_dispose`, `game_precache_new_map`, `game_map_loading_in_progress`, `game_unload`, `game_dispose_from_old_map`, `game_frame`, `remove_quitting_players_from_game`, `game_options_new`, `game_options_verify`, `game_initial_pulse`, `game_set_players_are_double_speed`, `game_players_are_double_speed`, `game_difficulty_level_set`, `game_difficulty_level_get`, `game_difficulty_level_get_ignore_easy`, `game_set_game_variant`, `game_set_game_engine_index`, `set_random_seed`, `game_load`, `game_initialize_for_new_map`, `game_set_game_variant_from_name`, `game_set_game_variant_options`, `game_variant_options_get`
- game/game_allegiance.c: `game_allegiance_initialize`, `game_allegiance_dispose`, `game_allegiance_initialize_for_new_map`, `game_allegiance_dispose_from_old_map`, `game_team_is_enemy`, `game_team_is_ally`, `game_team_ally_status_changed`, `game_allegiance_get_incidents`, `game_allegiance_provoke`, `game_allegiance_notify_change`, `game_allegiance_broken`, `game_allegiance_update`, `game_allegiance_create`, `game_allegiance_remove`, `game_allegiance_incident`, `game_allegiance_set_teams`
- game/game_engine.c: `game_globals_get_weapon`, `linear_to_non_linear_alpha`, `game_engine_dispose`, `initialize_player_multiplayer_data`, `game_engine_build_lighting`, `game_engine_dispose_from_old_map`, `game_engine_game_ending`, `players_in_game`, `game_engine_statistics_append`, `game_engine_handle_client_message`, `game_engine_handle_server_message`, `sort_statistic_buffer`, `sort_statistic_buffer_ranking`, `is_place_tied`, `place_get_position`, `get_place_string`, `drawline`, `get_selected_color`, `get_postgame_hilite_colors`, `find_closest_player_callback`, `game_engine_post_rasterize_objects`, `can_delete_item`, `game_engine_update_purge`, `update_weapon_inventory`, `game_engine_update_weapons`, `item_collection_get_total`, `game_engine_load_stage`, `game_engine_playlist_begin`, `game_engine_get_current_stage`, `game_engine_end_game`, `game_engine_allow_pick_up`, `game_engine_player_damaged_player`, `game_engine_player_is_odd_man_out`, `game_engine_player_is_out_of_lives`, `game_engine_should_spawn_player`, `game_engine_player_get_team_index`, `game_engine_prespawn_player_update`, `game_engine_running`, `game_engine_can_score`, `game_engine_force_single_screen`, `get_blink_alpha`, `game_engine_press_start_to_begin`, `nearby_vehicle`, `game_engine_rasterize_message`, `game_engine_picking_up`, `goal_matches_player`, `game_engine_player_get_custom_motion_sensor_positions`, `get_flag_definition_index`, `get_ball_definition_index`, `game_engine_switch_to_postgame`, `game_engine_get_variant`, `game_engine_get_goal_in_use`, `game_engine_get_goal_position`, `game_engine_set_goal_position`, `game_engine_clear_goal_position`, `game_engine_render_nav_points`, `game_engine_infinite_grenades_internal`, `game_engine_infinite_grenades`, `game_engine_has_teams`, `game_engine_display_team_indicators`, `game_engine_has_shield`, `weapon_definition_index_to_list_index`, `list_index_to_weapon_definition_index`, `game_engine_remap_vehicle`, `game_engine_remap_weapon`, `game_engine_man_out`, `game_engine_state_message`, `game_engine_player_get_change_color`, `game_engine_player_has_flag`, `game_engine_player_depower_active_camo`, `get_place_name`, `game_engine_get_place`, `game_engine_hud_draw_motion_sensor`, `game_engine_test_flag`, `game_engine_test_trait`, `netgame_flag_verify_no_team_duplicates`, `netgame_flag_verify_team_range`, `game_engine_playlist_next`, `build_game_variant_slayer_pro`, `build_game_variant_elimination`, `build_game_variant_phantoms`, `build_game_variant_endurance`, `build_game_variant_rockets`, `build_game_variant_snipers`, `build_game_variant_oddball`, `build_game_variant_team_oddball`, `build_game_variant_reverse_tag`, `build_game_variant_accumulation`, `build_game_variant_juggernaut`, `build_game_variant_stalker`, `build_game_variant_king`, `build_game_variant_king_pro`, `build_game_variant_crazy_king`, `build_game_variant_team_king`, `build_game_variant_ctf_pro`, `build_game_variant_invasion`, `build_game_variant_iron_ctf`, `build_game_variant_race`, `build_game_variant_rally`, `build_game_variant_team_race`, `build_game_variant_team_rally`, `game_engine_override_map_name`, `game_engine_override_game_variant`, `rasterize_in_game_score_draw_line`, `game_engine_hud_draw_messages`, `game_engine_player_has_stealth_weapon`, `game_engine_weapon_fired`, `test_any_gamepad_button`, `ticks_to_unicode_time_string`, `game_engine_flag_reset`, `game_engine_variant_cleanup`, `game_engine_allow_pause`, `game_engine_predict_resources`, `game_engine_draw_object_in_motion_sensor`, `game_engine_allow_dynamic_lighting`, `game_engine_allow_integrated_lights`, `game_engine_force_autopickup`, `game_engine_initialize`, `multiple_teams_alive`, `team_has_players`, `adjust_score_for_ranking`, `populate_statistic_buffer`, `game_engine_get_player_place`, `postgame_statistic_get_rating`, `select_players_to_display`, `find_closest_player_index`, `internal_rasterize_target_name`, `internal_rasterize_score`, `game_engine_get_type`, `game_engine_update_player_no_shield`, `game_engine_update_player_always_invis`, `game_engine_update_non_deterministic`, `multiplayer_message_internal`, `multiplayer_message`, `game_show_score_one_player`, `game_show_score_team`, `game_show_score_you_ally_enemy`, `game_show_score_extended`, `game_show_score`, `find_netgame_flags`, `find_netgame_flag`, `handle_custom_starting_equipment`, `game_engine_postspawn_player_update`, `game_engine_get_damage_multiplier`, `game_engine_update_teleporter`, `game_engine_get_distance_rating_for_spawn`, `game_engine_get_friendly_bonus`, `game_engine_get_variant_by_name`, `game_engine_remap_equipment`, `game_engine_remap_object_definition`, `game_engine_get_state_message`, `game_engine_did_player_win_default`, `game_engine_did_player_win`, `game_engine_did_team_win`, `netgame_flag_verify_team_exists`, `netgame_verify_spawn_points`, `netgame_verify_equipment`, `game_engine_verify_current_map`, `game_engine_playlist_initialize`, `game_engine_initialize_for_new_map`, `game_engine_generate_title_string`, `game_engine_post_rasterize_post_game`, `game_engine_nonplayer_post_rasterize`, `game_engine_rasterize_in_game_score`, `game_engine_post_rasterize_in_game`, `game_engine_post_rasterize`, `scoreboard_background_color`, `scoreboard_draw_row`, `scoreboard_team_columns`, `game_engine_scoreboard_closed`, `scoreboard_menu_items`, `scoreboard_menu_first_row`, `scoreboard_rectangle`, `scoreboard_point_in`, `scoreboard_menu_pick`, `game_engine_rasterize_scoreboard`, `game_engine_rasterize_voice_speakers`, `game_variant_options_default`, `game_engine_vehicle_empty`, `game_engine_vehicle_home_on_pad`, `game_engine_vehicle_home_follow`, `game_engine_vehicle_home_current`, `game_engine_update_vehicle_respawn`, `game_engine_predict_multiplayer_vehicle`, `game_engine_variant_vehicle_type`, `game_engine_nearest_team`, `game_engine_vehicle_placement_allowed`, `game_engine_network_player_score`, `game_engine_write_network_state`, `game_engine_read_network_state`
- game/game_engine_ctf.c: `ctf_create_flag_object`, `ctf_engine_dispose`, `create_the_flag`, `ctf_single_flag_what_is_up_message`, `ctf_engine_dispose_from_old_map`, `ctf_engine_player_added`, `ctf_engine_game_ending`, `ctf_engine_game_starting`, `ctf_engine_statistics_append`, `ctf_engine_handle_client_message`, `ctf_engine_handle_server_message`, `ctf_engine_pregame_post_rasterize`, `ctf_engine_post_rasterize`, `ctf_flag_failure_sound`, `ctf_find_flag_carrier`, `ctf_engine_allow_pick_up`, `ctf_engine_player_damaged_player`, `ctf_engine_display_score`, `ctf_engine_prespawn_player_update`, `ctf_state_message_update_warning`, `ctf_sound_update_warning`, `ctf_set_flag_warning`, `ctf_weapon_drop`, `ctf_test_flag`, `ctf_get_score_string`, `ctf_get_score_header_string`, `ctf_get_team_score_string`, `ctf_engine_initialize_for_new_map`, `ctf_position_near_flag`, `ctf_engine_player_update`, `ctf_engine_weapon_update`, `ctf_weapon_pickup`, `ctf_engine_starting_location_rating`, `ctf_show_grab`, `ctf_show_touch_return`, `ctf_show_untouched_return`, `ctf_client_flag_failure_update`, `game_engine_ctf_write_network_state`, `game_engine_ctf_read_network_state`
- game/game_engine_king.c: `king_engine_dispose`, `point3d_to_point2d`, `find_hill`, `king_engine_dispose_from_old_map`, `king_engine_player_added`, `king_engine_game_ending`, `king_engine_game_starting`, `king_engine_statistics_append`, `king_engine_handle_client_message`, `king_engine_handle_server_message`, `king_engine_pregame_post_rasterize`, `player_inside_hill`, `king_engine_player_update`, `king_calculate_hill_state`, `king_engine_player_damaged_player`, `king_engine_player_killed_player`, `king_engine_display_score`, `king_engine_prespawn_player_update`, `king_get_score`, `render_dynamic_quad_initialize`, `render_dynamic_quad`, `king_get_score_string`, `king_get_score_header_string`, `king_get_team_score_string`, `king_engine_goal_matches_player`, `find_next_hill`, `king_engine_initialize_for_new_map`, `king_engine_post_rasterize`, `king_engine_update`, `game_engine_king_write_network_state`, `king_client_score_sounds`, `game_engine_king_read_network_state`
- game/game_engine_multiplayer_sounds.c: `code_000a1460`, `code_000a14c0`, `game_engine_update_multiplayer_sound`, `code_000a1530`, `game_engine_play_multiplayer_sound`, `game_engine_intialize_queued_sounds`, `_game_engine_play_multiplayer_sound`, `push_queued_sound`, `get_sound_length_in_ticks`
- game/game_engine_oddball.c: `oddball_engine_dispose`, `oddball_engine_dispose_from_old_map`, `oddball_engine_player_added`, `oddball_engine_game_ending`, `oddball_engine_game_starting`, `oddball_engine_statistics_append`, `oddball_engine_handle_client_message`, `oddball_engine_handle_server_message`, `oddball_engine_pregame_post_rasterize`, `oddball_engine_post_rasterize`, `oddball_add_score`, `oddball_add_time_with_ball`, `player_ball_count`, `oddball_engine_player_damaged_player`, `player_has_ball`, `ball_available`, `oddball_engine_display_score`, `oddball_engine_prespawn_player_update`, `oddball_weapon_drop`, `oddball_get_score`, `oddball_ball_transfer_by_killing`, `accumulate_score_by_time`, `terminator_scoring_rules`, `oddball_test_flag`, `oddball_test_trait`, `oddball_get_score_string`, `oddball_get_score_header_string`, `oddball_get_team_score_string`, `find_position_for_ball`, `create_the_ball`, `oddball_engine_initialize_for_new_map`, `reset_ball`, `update_ball_ownership`, `oddball_engine_player_update`, `oddball_engine_weapon_update`, `oddball_engine_update`, `oddball_engine_player_killed_player`, `oddball_weapon_pickup`, `oddball_client_score_sounds`, `game_engine_oddball_write_network_state`, `game_engine_oddball_read_network_state`
- game/game_engine_race.c: `race_engine_dispose`, `delete_race_vehicles`, `race_get_vehicle_to_spawn`, `race_flags_make_unique`, `race_engine_dispose_from_old_map`, `race_engine_player_added`, `race_engine_game_ending`, `race_engine_game_starting`, `race_engine_statistics_append`, `race_engine_handle_client_message`, `race_engine_handle_server_message`, `race_engine_pregame_post_rasterize`, `race_engine_post_rasterize`, `race_complete_lap`, `can_touch_team`, `race_engine_weapon_update`, `race_team_can_win_game`, `build_player_speeds`, `race_engine_player_damaged_player`, `race_engine_player_killed_player`, `race_engine_display_score`, `race_engine_prespawn_player_update`, `race_goal_matches_player`, `count_bits_32`, `race_engine_get_score`, `race_get_score_string`, `race_get_score_header_string`, `race_get_team_score_string`, `race_engine_did_player_win`, `find_closest_vehicle`, `spawn_race_vehicles`, `new_rally_flag`, `race_touch_flag`, `race_engine_player_update`, `race_engine_update`, `race_engine_initialize_for_new_map`, `game_engine_race_write_network_state`, `game_engine_race_read_network_state`
- game/game_engine_slayer.c: `target_is_eligible`, `slayer_engine_dispose`, `slayer_engine_initialize_for_new_map`, `slayer_engine_dispose_from_old_map`, `slayer_engine_player_added`, `slayer_engine_game_ending`, `slayer_engine_game_starting`, `slayer_engine_statistics_append`, `slayer_engine_handle_client_message`, `slayer_engine_handle_server_message`, `slayer_engine_pregame_post_rasterize`, `slayer_engine_post_rasterize`, `slayer_engine_allow_pick_up`, `slayer_engine_player_damaged_player`, `update_speed_for_score`, `slayer_engine_adjust_score`, `slayer_engine_prespawn_player_update`, `slayer_test_flag`, `slayer_get_score_string`, `slayer_get_score_header_string`, `slayer_get_team_score_string`, `find_next_target`, `slayer_engine_display_score`, `slayer_player_update`, `game_engine_slayer_write_network_state`, `game_engine_slayer_read_network_state`
- game/game_engine_stub.c: `stub_engine_dispose`, `stub_engine_initialize_for_new_map`, `stub_engine_dispose_from_old_map`, `stub_engine_player_added`, `stub_engine_game_ending`, `stub_engine_game_starting`, `stub_engine_statistics_append`, `stub_engine_handle_client_message`, `stub_engine_handle_server_message`, `stub_engine_pregame_post_rasterize`, `stub_engine_post_rasterize`, `stub_engine_update`, `stub_engine_allow_pick_up`, `stub_engine_player_damaged_player`, `stub_engine_player_killed_player`
- game/game_globals.c: `material_get_name`, `code_000a4850`, `game_difficulty_get_value`, `game_difficulty_get_team_value`, `game_difficulty_get_value_by_difficulty`
- game/game_statistics.c: `game_statistics_start`, `game_statistics_stop`, `game_statistics_record_damage`, `game_statistics_record_kill`
- game/game_time.c: `game_time_initialized`, `game_time_initialize`, `game_time_initialize_for_new_map`, `game_time_dispose_from_old_map`, `game_time_dispose`, `game_time_end`, `game_time_get`, `game_time_get_elapsed`, `local_time_get`, `local_time_get_elapsed`, `game_predicting`, `game_in_progress`, `game_time_get_paused`, `game_time_set_paused`, `game_time_get_speed`, `game_time_set_speed`, `game_time_statistics_new`, `game_time_statistics_frame`, `game_time_start`, `game_time_set_distributed`, `game_time_get_tick_fraction`, `game_time_held`
- game/player_control.c: `player_control_get`, `player_control_initialize`, `player_control_dispose`, `player_control_dispose_from_old_map`, `player_control_camera_control_is_active`, `scripted_player_control_set_camera_control`, `player_control_inhibit_buttons`, `evaluate_piecewise_linear_function`, `player_action_clear`, `player_control_get_aiming_unit_index`, `player_control_get_target_object_index`, `player_control_get_field_of_view`, `player_control_get_unit_camera_info`, `player_control_get_unit_index`, `player_control_get_desired_weapon`, `player_control_get_autoaim_level`, `unit_get_local_player_index`, `players_unzoom_all`, `player_control_unzoom`, `player_control_get_zoom_level`, `player_control_action_test_reset`, `player_control_action_test_accept`, `player_control_action_test_back`, `player_control_action_test_action`, `player_control_action_test_jump`, `player_control_action_test_primary_trigger`, `player_control_action_test_grenade_trigger`, `player_control_action_test_zoom`, `player_control_action_test_move_relative_all_directions`, `player_control_action_test_look_relative_all_directions`, `player_control_action_test_look_relative_left`, `player_control_action_test_look_relative_right`, `player_control_action_test_look_relative_up`, `player_control_action_test_look_relative_down`, `player_control_action_test_check_reset_input_blob`, `signed_angular_difference`, `limit2d`, `interpolate_scalar`, `player_control_set_facing`, `player_control_new_unit`, `get_local_player_input_blob`, `player_control_get_facing_angles`, `player_control_get_facing_direction`, `player_control_set_desired_weapon`, `player_control_modify_desired_angles`, `player_control_initialize_for_new_map`, `player_control_permanent_impulse`, `player_control_action_test_note`
- game/player_queues_new.c: `update_server_add_player`, `update_client_new`, `update_client_delete`, `update_client_start`, `update_client_add_player`, `update_client_queue`, `update_client_queue_push`, `update_client_get_maximum_actions`, `update_client_build_client_update`, `player_new_queue`, `update_server_get_update`, `update_client_get_update`, `update_server_new`, `update_server_delete`, `update_server_start`, `update_server_build_server_update`, `update_client_dequeue`, `update_client_get_maximum_possible_server_time`, `update_server_handle_client_update`, `update_client_handle_server_update`, `update_queues_reset_and_fill_with_lies`, `update_server_next_update`, `update_client_local_ticks`, `update_client_local_player_index`, `update_client_dequeue_distributed`, `update_queues_distributed_reset`, `update_server_take_local_actions`, `distributed_action_valid`, `distributed_add_new_ticks`, `update_server_handle_distributed_input`, `update_server_ticked_update_number`, `update_server_update_actions`, `update_client_handle_relayed_action`, `update_client_distributed_input`
- game/players.c: `players_initialize`, `players_initialize_for_new_map`, `players_dispose_from_old_map`, `players_dispose`, `machine_get_player_list`, `machine_add_player`, `local_player_exists`, `find_unused_local_player_index`, `player_delete`, `players_get_respawn_failure`, `local_player_get_player_index`, `local_player_set_player_index`, `local_player_count`, `local_player_get_next`, `player_index_from_unit_index`, `player_died`, `players_are_all_dead`, `players_set_local_player_unit`, `players_get_combined_pvs_local`, `players_get_combined_pvs`, `player_input_enable`, `player_input_enabled`, `any_player_is_in_the_air`, `any_player_is_dead`, `is_player_in_trigger`, `player_pseudo_kill`, `player_control_fix_for_loaded_game_state`, `player_get_starting_location_count`, `player_get_starting_location`, `placement_data_set_change_color`, `create_weapon`, `players_compute_combined_pvs`, `players_compute_local_player_count`, `player_examine_nearby_unit`, `unit_should_autopick_weapon`, `player_reset_action_result`, `player_handle_weapon_swap`, `player_over_shield_screen_effect`, `player_active_camo_screen_effect`, `player_health_pack_screen_effect`, `player_powerup_on`, `player_powerup_additional`, `player_powerup_off`, `players_handle_deleted_object`, `random_direction3d`, `valid_real_vector2d`, `player_new`, `player_add_equipment`, `player_aiming_vector_from_facing`, `player_teleport_internal`, `player_teleport`, `find_best_starting_location_index`, `player_spawn`, `player_set_action_result`, `player_handle_action`, `player_handle_powerup`, `player_handle_powerup_minor`, `player_update_powerups`, `players_debug_render`, `debug_player_teleport`, `players_respawn_coop`, `player_teleport_on_bsp_switch`, `players_reconnect_to_structure_bsp`, `player_examine_nearby_vehicle`, `player_examine_nearby_device`, `player_handle_powerup_equipment`, `players_update_before_game`, `player_examine_nearby_item`, `network_player_attach_unit`, `network_player_show_pickup`, `network_player_log_idle_action`, `network_player_detach_unit`, `players_coop_extras_held`, `players_coop_may_spawn`, `players_coop_waiting_to_start`, `players_coop_danger_near`, `players_coop_unit_in_structure`, `players_coop_unit_grounded`, `players_coop_unit_safe`, `players_coop_unit_where`, `players_coop_session_count`, `players_coop_use_majority`, `players_coop_unit_spawnable`, `players_coop_near_count`, `players_coop_respawn_unit`, `players_coop_spawn_location`, `players_respawn_network_coop`, `player_spawnable_beside`, `players_checkpoint_position`, `players_coop_unit_grounded_anywhere`, `players_coop_unit_stranded`, `players_coop_rescue_stranded`, `players_coop_note_on_foot`, `players_coop_room_to_spawn`, `player_place_beside_teammate`, `players_coop_respawn_beside`, `players_note_checkpoint`, `players_respawn_at_checkpoint`, `player_position_switches_bsp`, `players_coop_bsp_switch_ready`, `players_coop_bsp_switch_allowed`, `player_name_clean`

## `hs`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `hs_compile` | hs/hs_compile.c | Partial | script/src/console.rs | leaves out the scenario's source text: maps carry their scripts compiled, so only the console's line is compiled; owed: the console's compiler (script/src/console.rs); no check against the original yet |
| `hs_not` | hs/hs_library_external.c | Matched | campaign/src/game.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_print` | hs/hs_library_external.c | Matched | campaign/src/game.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_players` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_objects_can_see_object` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_objects_can_see_flag` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_object_create` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_object_destroy` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_object_destroy_all` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_object_create_containing` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_object_destroy_containing` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_object_create_anew` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_object_create_anew_containing` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_object_teleport` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_object_set_facing` | hs/hs_library_external.c | Matched | campaign/src/objects.rs | owed: the script function's wrapper, which the campaign wires; no check against the original yet |
| `hs_wake` | hs/hs_runtime.c | Matched | script/src/lib.rs:452 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_long_to_boolean` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_short_to_boolean` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_string_to_boolean` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_data_to_void` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_short_to_real` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_long_to_real` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_enum_to_real` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_real_to_short` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_real_to_long` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_long_to_short` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_object_name_to_object_list` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_object_to_object_list` | hs/hs_runtime.c | Matched | script/src/value.rs | owed: one of hs_cast's conversions, in script/src/value.rs's cast; no check against the original yet |
| `hs_cast` | hs/hs_runtime.c | Matched | script/src/value.rs:88 | owed: the engine cites it, with no check against the original yet |
| `render_debug_scripting` | hs/hs_runtime.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_trigger_volumes` | hs/hs_runtime.c | Replaced | dev/src/draw.rs | the editor and tools |
| `hs_return` | hs/hs_runtime.c | Matched | script/src/lib.rs:210 | owed: the engine cites it, with no check against the original yet |
| `hs_global_evaluate` | hs/hs_runtime.c | Matched | script/src/lib.rs:962 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate` | hs/hs_runtime.c | Matched | script/src/lib.rs:592 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_arguments_evaluate` | hs/hs_runtime.c | Matched | script/src/lib.rs:592 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_begin` | hs/hs_runtime.c | Matched | script/src/lib.rs:711 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_begin_random` | hs/hs_runtime.c | Partial | script/src/lib.rs:711 | leaves out the game's own random numbers: the shuffle draws on the runtime's xorshift; owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_if` | hs/hs_runtime.c | Matched | script/src/lib.rs:712 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_set` | hs/hs_runtime.c | Matched | script/src/lib.rs:712 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_logical` | hs/hs_runtime.c | Matched | script/src/lib.rs:712 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_arithmetic` | hs/hs_runtime.c | Matched | script/src/lib.rs:713 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_equality` | hs/hs_runtime.c | Matched | script/src/lib.rs:713 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_inequality` | hs/hs_runtime.c | Matched | script/src/lib.rs:714 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_sleep` | hs/hs_runtime.c | Matched | script/src/lib.rs:873 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_sleep_until` | hs/hs_runtime.c | Matched | script/src/lib.rs:910 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_evaluate_inspect` | hs/hs_runtime.c | Matched | script/src/lib.rs:714 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_thread_main` | hs/hs_runtime.c | Matched | script/src/lib.rs:554 | owed: scripts; reads the object, tag and map world, which the fake worlds do not hold (the scenario's compiled scripts); script/tests |
| `hs_runtime_initialize_for_new_map` | hs/hs_runtime.c | Matched | script/src/lib.rs:290 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_runtime_update` | hs/hs_runtime.c | Matched | script/src/lib.rs:378 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_runtime_evaluate` | hs/hs_runtime.c | Matched | script/src/lib.rs:461 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |
| `hs_type_default_value` | hs/hs_runtime.c | Matched | script/src/value.rs:33 | owed: the engine's runtime (script/src/lib.rs); no check against the original yet |

692 Missing (196 reached from `main_loop`), by file:

- hs/hs.c: `code_000ad290`, `code_000ad2e0`, `code_000ad320`, `code_000ad340`, `code_000ad380`, `code_000ad3d0`, `code_000ad420`, `code_000ad470`, `code_000ad4b0`, `code_000ad4f0`, `code_000ad530`, `code_000ad570`, `code_000ad5b0`, `code_000ad5f0`, `code_000ad610`, `code_000ad650`, `code_000ad690`, `code_000ad6d0`, `code_000ad710`, `code_000ad750`, `code_000ad7a0`, `code_000ad7e0`, `code_000ad820`, `code_000ad860`, `code_000ad8a0`, `code_000ad8f0`, `code_000ad940`, `code_000ad980`, `code_000ad9c0`, `code_000ada00`, `code_000ada40`, `code_000ada90`, `code_000adab0`, `code_000adad0`, `code_000adb10`, `code_000adb60`, `code_000adbb0`, `code_000adc00`, `code_000adc40`, `code_000adc90`, `code_000adcd0`, `code_000add10`, `code_000add30`, `code_000add70`, `code_000addb0`, `code_000addf0`, `code_000ade30`, `code_000ade50`, `code_000ade90`, `code_000aded0`, `code_000adf10`, `code_000adf50`, `code_000adf90`, `code_000adfd0`, `code_000ae010`, `code_000ae030`, `code_000ae070`, `code_000ae0c0`, `code_000ae110`, `code_000ae150`, `code_000ae1a0`, `code_000ae1e0`, `code_000ae220`, `code_000ae260`, `code_000ae2a0`, `code_000ae2e0`, `code_000ae320`, `code_000ae370`, `code_000ae3b0`, `code_000ae400`, `code_000ae450`, `code_000ae4b0`, `code_000ae500`, `code_000ae540`, `code_000ae580`, `code_000ae5c0`, `code_000ae600`, `code_000ae650`, `code_000ae6a0`, `code_000ae6e0`, `code_000ae720`, `code_000ae770`, `code_000ae7c0`, `code_000ae810`, `code_000ae860`, `code_000ae8b0`, `code_000ae900`, `code_000ae940`, `code_000ae980`, `code_000ae9a0`, `code_000ae9e0`, `code_000aea20`, `code_000aea60`, `code_000aeaa0`, `code_000aeae0`, `code_000aeb30`, `code_000aeb80`, `code_000aebd0`, `code_000aec10`, `code_000aec50`, `code_000aec90`, `code_000aecc0`, `code_000aed00`, `code_000aed40`, `code_000aed90`, `code_000aedd0`, `code_000aee10`, `code_000aee50`, `code_000aeea0`, `code_000aeee0`, `code_000aef20`, `code_000aef60`, `code_000aefb0`, `code_000aeff0`, `code_000af030`, `code_000af070`, `code_000af0b0`, `code_000af0d0`, `code_000af0f0`, `code_000af110`, `code_000af130`, `code_000af150`, `code_000af170`, `code_000af1b0`, `code_000af1d0`, `code_000af210`, `code_000af250`, `code_000af290`, `code_000af2d0`, `code_000af310`, `code_000af350`, `code_000af390`, `code_000af3d0`, `code_000af410`, `code_000af450`, `code_000af490`, `code_000af4d0`, `code_000af510`, `code_000af550`, `code_000af570`, `code_000af5b0`, `code_000af5d0`, `code_000af610`, `code_000af650`, `code_000af690`, `code_000af6d0`, `code_000af710`, `code_000af750`, `code_000af790`, `code_000af7d0`, `code_000af810`, `code_000af850`, `code_000af890`, `code_000af8d0`, `code_000af910`, `code_000af950`, `code_000af990`, `code_000af9d0`, `code_000afa10`, `code_000afa50`, `code_000afa90`, `code_000afad0`, `code_000afb10`, `code_000afb50`, `code_000afb90`, `code_000afbd0`, `code_000afc10`, `code_000afc50`, `code_000afc90`, `code_000afcd0`, `code_000afd10`, `code_000afd50`, `code_000afd90`, `code_000afdd0`, `code_000afe10`, `code_000afe50`, `code_000afe90`, `code_000afed0`, `code_000aff10`, `code_000aff50`, `code_000aff90`, `code_000affd0`, `code_000b0010`, `code_000b0050`, `code_000b0070`, `code_000b00b0`, `code_000b00f0`, `code_000b0130`, `code_000b0170`, `code_000b01b0`, `code_000b01f0`, `code_000b0230`, `code_000b0270`, `code_000b02b0`, `code_000b02f0`, `code_000b0330`, `code_000b0370`, `code_000b03b0`, `code_000b03f0`, `code_000b0430`, `code_000b0470`, `code_000b04b0`, `code_000b04f0`, `code_000b0530`, `code_000b0570`, `code_000b05b0`, `code_000b0600`, `code_000b0650`, `code_000b06a0`, `code_000b06f0`, `code_000b0730`, `code_000b0770`, `code_000b07c0`, `code_000b0810`, `code_000b0850`, `code_000b08a0`, `code_000b08f0`, `code_000b0940`, `code_000b0990`, `code_000b09e0`, `code_000b0a20`, `code_000b0a60`, `code_000b0aa0`, `code_000b0ae0`, `code_000b0b20`, `code_000b0b60`, `code_000b0b90`, `code_000b0bb0`, `code_000b0bd0`, `code_000b0c10`, `code_000b0c50`, `code_000b0c70`, `code_000b0ca0`, `code_000b0cd0`, `code_000b0cf0`, `code_000b0d30`, `code_000b0d80`, `code_000b0da0`, `code_000b0dd0`, `code_000b0e00`, `code_000b0e30`, `code_000b0e60`, `code_000b0e90`, `code_000b0ec0`, `code_000b0ef0`, `code_000b0f20`, `code_000b0f50`, `code_000b0f80`, `code_000b0fb0`, `code_000b0fe0`, `code_000b1010`, `code_000b1050`, `code_000b1090`, `code_000b10b0`, `code_000b10f0`, `code_000b1130`, `code_000b1170`, `code_000b11b0`, `code_000b11f0`, `code_000b1220`, `code_000b1240`, `code_000b1260`, `code_000b1280`, `code_000b12a0`, `code_000b12c0`, `code_000b12e0`, `code_000b1320`, `code_000b1340`, `code_000b1360`, `code_000b13a0`, `code_000b13e0`, `code_000b1420`, `code_000b1460`, `radiosity_hack_start_evaluate`, `radiosity_hack_save_evaluate`, `radiosity_hack_find_point_evaluate`, `code_000b1500`, `code_000b1520`, `code_000b1540`, `code_000b1580`, `code_000b15c0`, `code_000b1600`, `code_000b1640`, `code_000b1690`, `code_000b16e0`, `code_000b1700`, `code_000b1720`, `code_000b1740`, `code_000b1760`, `code_000b17a0`, `code_000b17e0`, `code_000b1820`, `code_000b1860`, `code_000b1880`, `code_000b18a0`, `code_000b18c0`, `code_000b18f0`, `code_000b1920`, `code_000b1950`, `code_000b1980`, `code_000b19a0`, `code_000b19c0`, `code_000b19e0`, `code_000b1a00`, `code_000b1a30`, `code_000b1a50`, `code_000b1a70`, `code_000b1a90`, `code_000b1ad0`, `code_000b1b10`, `code_000b1b30`, `code_000b1b70`, `code_000b1bb0`, `code_000b1be0`, `code_000b1c20`, `code_000b1c60`, `code_000b1ca0`, `code_000b1ce0`, `code_000b1d20`, `code_000b1d60`, `code_000b1da0`, `code_000b1de0`, `code_000b1e20`, `code_000b1e70`, `code_000b1eb0`, `code_000b1ef0`, `code_000b1f30`, `code_000b1f70`, `code_000b1fc0`, `code_000b2010`, `code_000b2050`, `code_000b2070`, `code_000b20c0`, `code_000b2110`, `code_000b2160`, `code_000b21b0`, `code_000b21f0`, `code_000b2230`, `code_000b2270`, `code_000b22b0`, `code_000b22d0`, `code_000b2310`, `code_000b2330`, `code_000b2380`, `code_000b23d0`, `code_000b2410`, `code_000b2450`, `code_000b2490`, `code_000b24d0`, `code_000b2510`, `code_000b2550`, `code_000b2590`, `code_000b25d0`, `code_000b2610`, `code_000b2650`, `code_000b2670`, `code_000b26b0`, `code_000b26f0`, `code_000b2730`, `code_000b2770`, `code_000b27c0`, `code_000b2800`, `code_000b2840`, `code_000b2870`, `code_000b28b0`, `code_000b28f0`, `code_000b2910`, `code_000b2930`, `code_000b2950`, `code_000b29a0`, `code_000b29c0`, `code_000b2a00`, `code_000b2a40`, `code_000b2aa0`, `code_000b2b00`, `code_000b2b50`, `code_000b2b90`, `code_000b2bb0`, `code_000b2bf0`, `code_000b2c10`, `code_000b2c30`, `code_000b2c50`, `code_000b2c70`, `code_000b2cb0`, `code_000b2ce0`, `code_000b2d10`, `code_000b2d50`, `code_000b2d90`, `code_000b2db0`, `hs_scenario_merge`, `code_000b2f00`, `hs_dispose`, `hs_recompile`, `hs_update`, `hs_node_gc`, `hs_function_get`, `hs_find_script_by_name`, `hs_find_tag_reference_by_index`, `hs_global_external_get`, `hs_global_get_type`, `hs_global_get_name`, `hs_find_global_by_name`, `hs_find_function_by_name`, `code_000b33b0`, `code_000b33d0`, `code_000b3450`, `code_000b3490`, `code_000b34d0`, `code_000b3500`, `code_000b3520`, `code_000b3550`, `code_000b3580`, `code_000b35e0`, `code_000b3610`, `code_000b36c0`, `code_000b36f0`, `code_000b3720`, `code_000b3750`, `code_000b3780`, `code_000b37b0`, `code_000b37e0`, `code_000b3810`, `code_000b3840`, `code_000b3870`, `code_000b38a0`, `code_000b38e0`, `hs_tokens_enumerate`, `code_000b3a00`, `code_000b3b10`, `code_000b3b60`, `code_000b3ca0`, `code_000b3d10`, `code_000b3de0`, `code_000b3e80`, `hs_evaluate_by_name`, `code_000b3ee0`, `code_000b3f00`, `code_000b3f50`, `hs_scenario_postprocess`, `hs_initialize_for_new_map`, `hs_dispose_from_old_map`, `hs_help`, `hs_doc`, `hs_hack`, `code_000b4390`, `code_000b43b0`, `hs_initialize`, `hs_compile_and_evaluate`, `hs_sv_say`, `hs_sv_end_game`, `hs_scenario_syntax_data_valid`, `hs_syntax_node_refusal`, `hs_scenario_functions_check`, `hs_scenario_script_disabled`, `hs_scenario_global_initializer_disabled`, `hs_allocate`, `hs_compile_source_error`, `hs_compile_source`, `hs_rebuild_source_file`, `hs_rebuild_source`, `hs_get_function_parameters_string`, `hs_get_function_documentation_string`, `hs_tokens_enumerate_add_string`, `hs_enumerate_block_data`, `hs_enumerate_scenario_data`, `hs_enumerate_from_string_list`, `hs_enumerate_special_form_names`, `hs_enumerate_script_type_names`, `hs_enumerate_type_names`, `hs_enumerate_function_names`, `hs_enumerate_script_names`, `hs_enumerate_variable_names`, `hs_enumerate_ai_names`, `hs_enumerate_ai_command_list_names`, `hs_enumerate_starting_profile_names`, `hs_enumerate_conversation_names`, `hs_enumerate_object_names`, `hs_enumerate_trigger_volume_names`, `hs_enumerate_cutscene_flag_names`, `hs_enumerate_cutscene_camera_point_names`, `hs_enumerate_cutscene_title_names`, `hs_enumerate_cutscene_recording_names`, `hs_enumerate_navpoints`, `hs_enumerate_hud_messages`, `alphabetize_file_references`, `alphabetize`, `hs_objects_can_see_object_evaluate`, `hs_objects_can_see_flag_evaluate`, `objects_scripting_set_scale_evaluate`, `unit_scripting_set_maximum_vitality_evaluate`, `units_scripting_set_maximum_vitality_evaluate`, `unit_scripting_set_current_vitality_evaluate`, `units_scripting_set_current_vitality_evaluate`, `device_set_desired_position_evaluate`, `device_group_set_desired_value_evaluate`, `player_effect_screen_fade_in_evaluate`, `player_effect_screen_fade_out_evaluate`, `debug_sound_classes_set_distances_evaluate`, `sound_class_set_gain_evaluate`, `scripted_player_effect_set_translation_evaluate`, `scripted_player_effect_set_rotation_evaluate`, `scripted_player_effect_set_rumble_evaluate`, `scripted_player_effect_start_evaluate`, `rasterizer_model_ambient_reflection_tint_evaluate`, `rasterizer_screen_effect_set_convolution_evaluate`, `rasterizer_screen_effect_set_filter_evaluate`, `rasterizer_screen_effect_set_filter_desaturation_tint_evaluate`, `hs_object_list_get_element_evaluate`, `hs_sound_get_gain_evaluate`, `ai_debug_vocalize_evaluate`, `debug_sound_classes_enable_evaluate`, `hs_recompile_evaluate`, `random_range_evaluate`, `real_random_range_evaluate`, `hs_compile_and_evaluate_command`
- hs/hs_compile.c: `code_000b46b0`, `code_000b4970`, `hs_compile_initialize`, `code_000b4ad0`, `hs_verify_source_offset`, `code_000b4b70`, `code_000b4bc0`, `code_000b4be0`, `code_000b4d00`, `code_000b4dc0`, `code_000b4ef0`, `code_000b4fe0`, `code_000b5100`, `code_000b5180`, `code_000b5230`, `code_000b5300`, `code_000b54d0`, `code_000b55d0`, `code_000b5640`, `code_000b56b0`, `code_000b5720`, `code_000b5790`, `code_000b5800`, `code_000b5870`, `code_000b5920`, `code_000b5990`, `code_000b5a00`, `code_000b5a70`, `code_000b5bb0`, `code_000b5c50`, `code_000b5ce0`, `code_000b5d70`, `code_000b5dd0`, `code_000b5e10`, `code_000b5ea0`, `code_000b6130`, `code_000b6560`, `code_000b6650`, `code_000b6740`, `code_000b6860`, `hs_compile_postprocess`, `code_000b6eb0`, `code_000b6f80`, `code_000b7040`, `hs_parse`, `hs_macro_function_parse`, `hs_parse_begin`, `hs_parse_if`, `hs_parse_cond`, `hs_parse_set`, `hs_parse_logical`, `hs_parse_arithmetic`, `hs_parse_equality`, `hs_parse_inequality`, `hs_parse_sleep`, `hs_parse_sleep_until`, `hs_parse_wake`, `hs_parse_inspect`, `hs_parse_object_cast_up`, `hs_parse_debug_string`, `hs_compile_expression`, `hs_compile_finish`, `hs_compile_dispose`, `hs_parse_cond_recursive`, `hs_get_parameter_indices`, `hs_compile_add_source`, `hs_find_string_constant`, `character_in_list`, `hs_parse_variable`, `hs_parse_call_predicate`, `hs_parse_boolean`, `hs_parse_real`, `hs_parse_integer`, `hs_parse_string`, `hs_parse_script`, `hs_parse_tag_reference`, `hs_parse_enum`, `hs_parse_scenario_datum`, `hs_parse_trigger_volume`, `hs_parse_cutscene_flag`, `hs_parse_cutscene_camera_point`, `hs_parse_cutscene_title`, `hs_parse_cutscene_recording`, `hs_parse_device_group`, `hs_parse_ai`, `hs_parse_ai_command_list`, `hs_parse_starting_profile`, `hs_parse_conversation`, `hs_parse_object_name`, `hs_parse_object`, `hs_parse_navpoint`, `hs_parse_hud_message`, `hs_parse_object_list`, `string_list_find`, `hs_concatenate_string_constant`, `hs_add_global`, `hs_add_script`, `hs_tokenize_primitive`, `hs_parse_primitive`, `hs_parse_nonprimitive`, `hs_postprocess_passthrough_arguments`, `hs_concatenate_expression`, `hs_tokenize`, `hs_tokenize_nonprimitive`, `skip_whitespace`
- hs/hs_library_external.c: `hs_trigger_volume_test_objects`, `hs_unit_can_see_object`, `hs_unit_can_see_flag`, `object_is_or_contains_player`, `hs_object_destroy_by_name`, `hs_object_iterate_names_containing`, `hs_object_list_get_element`, `hs_object_set_shield`, `hs_object_set_permutation`, `hs_objects_predict`, `hs_objects_delete_by_definition`, `hs_effect_new`, `hs_effect_new_from_object_marker`, `hs_damage_new`, `hs_damage_object`, `hs_sound_get_gain_reference`, `hs_sound_get_gain`, `hs_sound_set_gain`, `hs_trigger_volume_test_objects_all`, `hs_trigger_volume_test_objects_any`, `hs_object_orient`, `hs_teleport_players_not_in_trigger_volume`
- hs/hs_runtime.c: `hs_syntax_nth`, `hs_inspect_boolean`, `hs_inspect_real`, `hs_inspect_short_integer`, `hs_inspect_long_integer`, `hs_inspect_string`, `hs_inspect_enum`, `hs_runtime_initialize`, `hs_runtime_dispose_from_old_map`, `hs_runtime_dispose`, `expression_get_function_name`, `hs_thread_new`, `hs_thread_delete`, `hs_thread_format`, `hs_stack_push`, `hs_stack_pop`, `hs_stack_allocate`, `hs_find_thread_by_script`, `hs_find_thread_by_name`, `hs_object_type_can_cast`, `hs_can_cast`, `hs_global_reconcile_read`, `hs_global_reconcile_write`, `script_error`, `hs_runtime_get_executing_thread_name`, `hs_wake_by_name`, `hs_evaluate_wake`, `hs_script_evaluate`, `hs_macro_function_evaluate`, `hs_evaluate_object_cast_up`, `hs_evaluate_debug_string`, `hs_runtime_waiting_on_call`
- hs/hs_scenario_definitions.c: `hs_syntax_data_byte_swap`
- hs/object_lists.c: `reference_list_new`, `reference_list_get_next_datum_index`, `reference_list_add`, `reference_list_delete`, `object_lists_initialize`, `object_lists_dispose`, `object_lists_initialize_for_new_map`, `object_lists_dispose_from_old_map`, `object_list_new`, `object_list_delete`, `object_list_add`, `object_list_get_next`, `object_list_add_reference`, `object_list_remove_reference`, `object_list_gc`, `object_list_count`, `object_list_get_first`

## `input`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `input_abstraction_get_local_player_preferences` | input/input_abstraction.c | Replaced |  | through its caller `draw_string_and_hack_in_icons` |
| `input_abstraction_update_device_changes` | input/input_abstraction.c | Replaced |  | through its caller `input_get_device_states` |
| `fix_dead_zone` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `update_ticks` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `code_000be800` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
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
| `code_000bebb0` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `code_000bebc0` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `code_000bebd0` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `code_000bf0a0` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `code_000bf150` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_get_raw_data_string` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_update` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_frame_begin` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_frame_end` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `code_000bf540` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_initialize` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `HATCleanup` | input/HaloAutoTest.c | Replaced |  | through its caller `input_dispose` |
| `HATReadMain` | input/HaloAutoTest.c | Replaced |  | through its caller `HATInit` |
| `HATRawLoopRead` | input/HaloAutoTest.c | Replaced |  | through its caller `HATRun` |
| `HATInit` | input/HaloAutoTest.c | Replaced |  | through its caller `input_initialize` |
| `HATRun` | input/HaloAutoTest.c | Replaced |  | through its caller `input_update` |
| `update_hold_ticks` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `acquire_input_mutex` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `release_input_mutex` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_update_analog_button_state` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_get_device_states` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_update_keyboard_devices` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_flush_rumble` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |
| `input_keyboard_thread` | input/input_xbox.c | Replaced | app/Sources/OpenCEEngineKit/PlayControls.swift | keyboard and mouse only |

14 Missing (10 reached from `main_loop`), by file:

- input/HaloAutoTest.c: `HATRawRead`, `HATRawWrite`
- input/input_abstraction.c: `input_abstraction_dispose`, `input_abstraction_reset_controller_detection_timer`, `input_abstraction_update_local_player_preferences`, `input_abstraction_get_input_state`, `code_000bdc60`, `code_000bdd40`, `input_abstraction_initialize`, `input_abstraction_update`, `keyboard_hold_ticks`, `keyboard_controls_update`, `local_player_is_piloting_aircraft`, `set_default_game_input_preferences`

## `interface`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `hud_calculate_point` | interface/hud_draw.c | Matched | ffi/src/hud.rs:168 | owed: the engine cites it, with no check against the original yet |
| `get_flash_color` | interface/hud_draw.c | Matched | ffi/src/hud.rs:180 | owed: the engine cites it, with no check against the original yet |
| `hud_draw_meter` | interface/hud_draw.c | Matched | ffi/src/hud.rs:319 | owed: the engine cites it, with no check against the original yet |
| `hud_draw_numbers` | interface/hud_draw.c | Matched | ffi/src/hud.rs:361 | owed: the engine cites it, with no check against the original yet |
| `hud_draw_static_element` | interface/hud_draw.c | Matched | ffi/src/hud.rs:300 | owed: the engine cites it, with no check against the original yet |
| `render_state_bitmap` | interface/hud_messaging.c | Matched | ffi/src/hud.rs:258 | owed: the engine cites it, with no check against the original yet |
| `hud_render_unit_interface` | interface/hud_unit.c | Matched | ffi/src/hud.rs:82 | owed: the engine cites it, with no check against the original yet |
| `blip_size_get` | interface/motion_sensor.c | Matched | sim/src/hud.rs:67 | owed: the motion sensor's blips; checked by sim/tests/hud.rs |
| `render_blip` | interface/motion_sensor.c | Matched | sim/src/hud.rs:171 | owed: the engine cites it, with no check against the original yet |
| `motion_sensor_update` | interface/motion_sensor.c | Matched | sim/src/hud.rs:122 | owed: the motion sensor; reads the object, tag and map world, which the fake worlds do not hold; sim/tests/hud.rs |
| `ui_start_main_menu_music` | interface/ui_widget.c | Matched | ffi/src/ui_host.rs:62 | owed: the engine cites it, with no check against the original yet |
| `get_ui_rgb_white` | interface/ui_widget.c | Matched | ui/src/draw.rs:16 | owed: the engine cites it, with no check against the original yet |
| `first_person_weapon_start_interpolation` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:543 | `tools/engine_first_person_reference.py` |
| `first_person_weapon_set_state` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:197 | `tools/engine_first_person_reference.py` |
| `first_person_weapon_animation_fits` | interface/first_person_weapons.c | Matched | sim/src/first_person.rs:88 | owed: reads the object, tag and map world, which the fake worlds do not hold (the first-person model's nodes); sim/tests/first_person.rs |
| `first_person_weapon_build_node_matrices` | interface/first_person_weapons.c | Matched | sim/src/first_person.rs:160 | owed: reads the object, tag and map world, which the fake worlds do not hold (the first-person model's nodes); sim/tests/first_person.rs |
| `first_person_weapon_message` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:633 | `tools/engine_first_person_reference.py` |
| `first_person_weapon_switch_weapons` | interface/first_person_weapons.c | Matched | sim/src/first_person_states.rs:616 | owed: the engine cites it, with no check against the original yet |
| `first_person_weapon_next_state` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:686 | `tools/engine_first_person_reference.py` |
| `first_person_weapon_update` | interface/first_person_weapons.c | Matched | sim/src/first_person_states.rs:751 | owed: reads the object, tag and map world, which the fake worlds do not hold (the weapon tag); its states are first_person_reference's |
| `model_build_remapping_table_for_animation_graph` | interface/first_person_weapons.c | Matched | sim/src/first_person.rs:75 | owed: reads the object, tag and map world, which the fake worlds do not hold (the model's and graph's node names) |
| `first_person_weapon_state_from_weapon_message` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:315 | `tools/engine_first_person_reference.py` |
| `first_person_animation_type_from_weapon_state` | interface/first_person_weapons.c | Ported | sim/src/first_person_states.rs:166 | `tools/engine_first_person_reference.py` |
| `hud_show_action_response` | interface/hud.c | Matched | ffi/src/hud_text.rs:52 | owed: the HUD's messages; reads the object, tag and map world, which the fake worlds do not hold |
| `get_sprite_clip_rect` | interface/hud_draw.c | Matched | ffi/src/hud.rs:658 | owed: the engine cites it, with no check against the original yet |
| `hud_calculate_bitmap_bounds` | interface/hud_draw.c | Matched | ffi/src/hud.rs:206 | owed: the engine cites it, with no check against the original yet |
| `render_grenade_hud` | interface/hud_weapon.c | Matched | ffi/src/hud.rs:624 | owed: the engine cites it, with no check against the original yet |
| `widget_instance_render_recursive` | interface/ui_widget.c | Matched | ui/src/draw.rs:3 | owed: the engine cites it, with no check against the original yet |
| `begin_music_fade_out` | interface/ui_widget_event_handler_functions.c | Matched | ffi/src/ui_host.rs:651 | owed: the engine cites it, with no check against the original yet |
| `main_menu_initialize` | interface/ui_widget_event_handler_functions.c | Matched | ffi/src/ui_host.rs:651 | owed: the engine cites it, with no check against the original yet |

884 Missing (547 reached from `main_loop`), by file:

- interface/attract_mode.c: `attract_mode_should_start`, `attract_mode_reset_timer`, `attract_mode_get_localized_movie_path`, `attract_mode_start`, `game_end_credits_start`
- interface/event_manager.c: `event_manager_initialize`, `event_manager_dispose`, `event_manager_flush`, `event_manager_suppress`, `get_next_event`, `event_manager_time_of_last_event`, `code_000cb850`, `event_manager_update`, `event_manager_post_button`, `queue_event`
- interface/first_person_weapons.c: `animation_update_render_only`, `first_person_weapons_initialize`, `first_person_weapons_dispose`, `first_person_weapons_initialize_for_new_map`, `first_person_weapons_dispose_from_old_map`, `code_000cbd30`, `code_000cbdf0`, `code_000cbf00`, `code_000cc020`, `code_000cc060`, `code_000cc100`, `code_000cc1b0`, `code_000cc290`, `code_000cc2f0`, `code_000cc330`, `first_person_weapon_draw`, `first_person_weapon_get_local_index`, `first_person_weapon_get_marker_by_name`, `first_person_weapon_center_flashlight`, `first_person_weapon_adjust_light`, `first_person_weapon_get_node_matrix`, `code_000cca00`, `code_000ccab0`, `first_person_weapon_render_update`, `first_person_weapon_get_marker_by_name_render`, `code_000cd100`, `code_000cd3b0`, `code_000cd610`, `code_000cd670`, `first_person_weapon_message_from_unit`, `first_person_weapon_message_from_weapon`, `code_000cd920`, `code_000cda90`, `first_person_weapons_update`, `first_person_weapon_set_visibility`, `first_person_weapon_predict`, `first_person_weapon_new_unit`, `weapon_play_first_person_weapon_sound`, `model_remap_node_matrices_to_match_animation_graph`, `first_person_weapon_get`, `first_person_weapon_index_from_weapon_index`, `first_person_weapon_index_from_unit_index`
- interface/hud.c: `code_000bf660`, `hud_initialize`, `hud_dispose`, `hud_initialize_for_new_map`, `hud_dispose_from_old_map`, `scripted_show_hud`, `scripted_show_hud_help_text`, `hud_update`, `code_000bf840`, `code_000bf870`, `hud_get_item_string`, `hud_picked_up_grenade`, `hud_picked_up_ammunition`, `hud_picked_up_weapon`, `hud_picked_up_powerup`, `code_000c0020`, `hud_load`, `hud_autosave`, `hud_draw_friendly_indicator`, `hud_draw_players`, `code_000c0430`, `hud_draw_screen`, `weapon_state_is_depleted`, `get_object_icon_text_index`, `temporary_hud_draw_reticle`, `hud_player_names_setting`, `hud_player_name_scale`, `hud_player_name_in_sight`, `hud_player_name_enemy_range`, `hud_draw_player_name`, `hud_draw_player_names`, `temporary_hud_draw`
- interface/hud_draw.c: `get_return_eip`, `check_stack_buffer`, `code_000c0920`, `hud_globals_get_scale`, `hud_retrieve_bitmap_and_bounding_rect`, `code_000c0c30`, `code_000c0e10`, `fast_ftol_C`, `real_argb_color_to_pixel32`, `real_rgb_color_to_pixel32`, `real_alpha_intensity_to_pixel32`, `get_flash_duration`, `code_000c1920`, `code_000c1b40`, `code_000c2420`, `hud_draw_bitmap_direct`, `hud_draw_bitmap`, `hud_draw_weapon_overlays`, `hud_draw_multitexture_overlay_get_current_weapon_definition`, `hud_multitexture_overlay_blend_function`, `hud_draw_multitexture_overlay`, `hud_multitexture_overlays_follow_zoom`, `hud_number_shows_only_when_zoomed`, `hud_zoomed_layout_begin`, `hud_zoomed_layout_end`, `hud_draw_bitmap_with_meter`
- interface/hud_messaging.c: `render_state_text`, `hud_messaging_initialize`, `hud_messaging_initialize_for_new_map`, `hud_messaging_dispose_from_old_map`, `hud_messaging_dispose`, `scripted_hud_set_state_message`, `scripted_hud_set_flashing_state`, `scripted_hud_restart_flashing`, `scripted_hud_set_objective`, `scripted_hud_set_timer_time`, `scripted_hud_set_timer_warning_cutoff`, `scripted_hud_set_timer_position`, `scripted_hud_show_timer`, `scripted_hud_pause_timer`, `scripted_hud_get_timer_ticks`, `scripted_hud_time_code_show`, `scripted_hud_time_code_start`, `scripted_hud_time_code_reset`, `hud_render_timer`, `hud_set_state_message`, `hud_set_state_message_icon`, `hud_set_state_message_text`, `hud_enable_custom_state_message`, `hud_set_state_text`, `hud_messaging_get_objective`, `find_free_hud_message`, `compare_messages`, `scripted_hud_messages_clear`, `hud_get_font_index`, `hud_get_text_color`, `hud_messaging_globals_update`, `hud_print_message`, `hud_add_item_message`, `hud_broadcast_team_message`, `hud_messaging_update`
- interface/hud_nav_points.c: `find_nav_point`, `get_nav_point_datum`, `hud_initialize_nav_points`, `hud_initialize_nav_points_for_new_map`, `hud_dispose_nav_points_from_old_map`, `hud_dispose_nav_points`, `code_000c53b0`, `hud_activate_nav_point_with_game_engine_flag`, `hud_activate_nav_point_with_flag`, `hud_activate_nav_point_with_object`, `code_000c5500`, `hud_activate_team_nav_point_with_game_engine_flag`, `hud_activate_team_nav_point_with_flag`, `hud_activate_team_nav_point_with_object`, `code_000c5600`, `hud_activate_global_nav_point_with_game_engine_flag`, `code_000c56a0`, `hud_deactivate_nav_point_with_game_engine_flag`, `hud_deactivate_nav_point_with_flag`, `hud_deactivate_nav_point_with_object`, `code_000c5770`, `hud_deactivate_team_nav_point_with_flag`, `hud_deactivate_team_nav_point_with_object`, `hud_unit_activate_nav_point_with_flag`, `hud_unit_activate_nav_point_with_object`, `hud_unit_deactivate_nav_point_with_flag`, `hud_unit_deactivate_nav_point_with_object`, `hud_get_nav_point_render_type`, `custom_render_nav_point`, `hud_render_nav_points`, `code_000c61d0`, `hud_update_nav_points`, `hud_activate_nav_point`, `hud_activate_team_nav_point`, `hud_activate_global_nav_point`, `hud_deactivate_nav_point`, `hud_deactivate_team_nav_point`, `hud_update_nav_point_local_player`
- interface/hud_sounds.c: `hud_play_sound`
- interface/hud_unit.c: `unit_hud_outline_mapper_tick`, `unit_hud_shield_meter_mapper_tick`, `unit_hud_shield_meter_mapper_init`, `initialize_hud_state`, `get_hud_state`, `hud_initialize_unit_interface`, `hud_initialize_unit_interface_for_new_map`, `hud_dispose_unit_interface_from_old_map`, `hud_dispose_unit_interface`, `scripted_hud_show_health`, `scripted_hud_blink_health`, `scripted_hud_show_shield`, `scripted_hud_blink_shield`, `scripted_hud_show_motion_sensor`, `scripted_hud_blink_motion_sensor`, `hud_play_unit_sounds`, `hud_fix_unit_data`, `hud_update_unit_local_player`, `hud_render_damage_indicators`, `hud_tick_shield`, `hud_update_unit`
- interface/hud_weapon.c: `hud_initialize_weapon_interface`, `hud_initialize_weapon_interface_for_new_map`, `hud_dispose_weapon_interface_from_old_map`, `hud_dispose_weapon_interface`, `scripted_hud_show_crosshair`, `get_hud_state`, `get_crosshair_state`, `code_000c8020`, `code_000c8070`, `strip_path_name`, `code_000c8370`, `hud_fix_weapon_data`, `code_000c8ce0`, `code_000c92a0`, `hud_update_weapon`, `hud_render_weapon_interface`, `play_weapon_hud_sounds`, `hud_update_weapon_local_player`, `crosshairs_draw`, `render_weapon_hud`
- interface/interface.c: `interface_initialize`, `interface_dispose_from_old_map`, `interface_dispose`, `interface_get_tag_index`, `interface_get_real_argb_color`, `interface_set_bitmap_text_draw_mode`, `interface_get_weapon_hud_index`, `interface_draw_screen`, `profile_graph_toggle`, `render_debug_profile_stall_tick`, `render_debug_profile`, `interface_splitscreen_render`, `interface_initialize_for_new_map`, `interface_get_rgb_color`, `interface_draw_fullscreen_overlays`, `interface_draw_bitmap`, `interface_draw_bitmap_modulated`, `interface_draw_bitmap_modulated_p32`
- interface/marketing_and_strategic_business_development.c: `xbox_demos_available`, `xbox_demos_launch`, `xbox_dashboard_launch`, `clean_up_for_image_launch`
- interface/motion_sensor.c: `tiny_point2d_set`, `tiny_point2d_get`, `blip_type_get`, `motion_sensor_blip_set_type_and_size`, `get_motion_sensor_data`, `motion_sensor_initialize`, `motion_sensor_dispose`, `motion_sensor_initialize_for_new_map`, `motion_sensor_dispose_from_old_map`, `should_track_object`, `blip_begin`, `should_draw_object`, `update_motion_sensor`, `render_motion_sensor`, `motion_sensor_draw_screen`, `motion_sensor_tick`
- interface/overhead_map.c: `overhead_map_initialize`, `overhead_map_initialize_for_new_map`, `overhead_map_dispose_from_old_map`, `overhead_map_post_rasterize`
- interface/player_ui.c: `player_ui_dispose`, `player_ui_reset_single_player_local_player_controllers`, `player_ui_set_single_player_local_player_controller`, `player_ui_get_single_player_local_player_controller`, `player_ui_get_single_player_local_player_from_controller`, `player_ui_local_player_joined_multiplayer_game`, `player_ui_local_player_wants_to_play_multiplayer`, `player_ui_clear_multiplayer_autojoin_for_local_player`, `player_ui_autojoin_players_to_next_multiplayer_game`, `player_ui_clear_multiplayer_variant`, `player_ui_get_active_player_profile`, `player_ui_get_active_player_profile_index`, `player_ui_get_last_single_player_level_played`, `player_ui_set_game_variant`, `player_ui_game_variant_specified`, `player_ui_rumble_disabled`, `player_ui_autolevel_enabled`, `player_ui_get_path_to_local_player_profile_directory`, `player_ui_remember_player1_profile`, `player_ui_get_player1_last_used_profile_index`, `player_ui_fast_setup_network_server`, `player_ui_edit_profile_is_default_profile`, `player_ui_edit_profile_name_is_dirty`, `player_ui_prompt_user_to_rename_edit_profile`, `player_ui_get_edit_player_profile`, `player_ui_get_edit_playlist_profile`, `player_ui_edit_profile_is_dirty`, `player_ui_activate_all_solo_levels`, `hud_message_to_all`, `player0_look_pitch_is_inverted`, `player0_joystick_set_is_normal`, `generate_default_player_profile`, `set_local_player_controls_from_player_profile`, `clear_profile_edit_data`, `reset_local_player_profile`, `player_ui_initialize`, `player_ui_clear_multiplayer_joins`, `player_ui_set_active_player_profile`, `player_ui_begin_editing_profile`, `player_ui_save_profile`, `player_ui_end_editing_profile`, `player0_look_invert_pitch`, `player_ui_get_game_variant_options`, `player_ui_saved_game_file_removed`
- interface/progress_bar.c: `code_000d0d10`, `code_000d0ec0`, `code_000d0f10`, `code_000d0f20`, `code_000d0f40`, `tgaLoadHeader`, `code_000d1020`, `code_000d1030`, `tgaLoadImageData`, `code_000d10e0`, `code_000d10f0`, `tgaLoad`, `progress_bar_is_stuff_ready`, `code_000d1160`, `code_000d1380`, `code_000d1390`, `progress_bar_initialize`, `progress_bar_dispose`, `progress_bar_begin`, `code_000d14c0`, `progress_bar_end`, `progress_bar_is_active`, `to_screen`, `generate_gravy_rect`, `code_000d1660`, `code_000d1670`, `code_000d1680`, `code_000d16a0`, `progress_bar_create_noise_texture`, `code_000d16e0`, `code_000d16f0`, `gen_cloud_coord`, `gen_mask_coord`, `draw_layer_int`, `draw_layer`, `draw_initial_layer`, `this_is_awful`, `do_convoluation_coords`, `draw_fade_layer`, `progress_bar_enable`, `code_000d1ba0`, `IDirect3DTexture8_Release`, `code_000d1bc0`, `code_000d1bd0`, `code_000d1bf0`, `D3DXMatrixIdentity`, `SetRenderStateSmart`, `SetTextureStageStateSmart`, `progress_bar_render`, `code_000d23b0`, `progress_bar_create_mask_texture`, `progress_bar_eachframe`, `progress_bar_make_stuff_ready`, `progress_bar_display`, `progress_bar_load_loading_texture`
- interface/terminal.c: `terminal_initialize`, `terminal_dispose`, `code_000d2b90`, `terminal_clear`, `terminal_gets_active`, `terminal_gets_begin`, `terminal_gets_end`, `code_000d2d00`, `code_000d2dc0`, `terminal_draw`, `code_000d30c0`, `terminal_update`, `terminal_printf`, `terminal_remove_line`, `terminal_update_input`, `terminal_update_output`, `terminal_new_line`
- interface/ui_widget.c: `code_000d32c0`, `code_000d32e0`, `event_controller_index_compatible_with_widget`, `set_ui_plasma_effect_color`, `ui_widgets_initialize`, `ui_widgets_safe_to_load`, `ui_widgets_set_fade_value`, `ui_widget_debug_show_path`, `widget_instance_count_children`, `widget_instance_get_nth_child`, `ui_widget_realloc`, `widget_free`, `ui_widgets_active`, `ui_widgets_active_for_local_player`, `ui_widgets_inhibit_processing`, `code_000d35e0`, `draw_bitmap_in_rect`, `widget_instance_get_topmost_parent`, `widget_instance_get_child_index_from_parent`, `widget_instance_set_visibility_recursive`, `main_menu_active`, `main_menu_is_active`, `main_menu_screen_is_active`, `ui_set_next_level`, `ui_widget_load_progress_widget`, `filesystem_check_thread_is_active`, `code_000d3c10`, `display_error_when_main_menu_loaded`, `display_error_deferred`, `display_error_abort_to_dashboard_deferred`, `ui_stop_main_menu_music`, `ui_main_menu_music_active`, `ui_widgets_disable_pause_game`, `code_000d3e70`, `code_000d3ef0`, `code_000d3f50`, `code_000d3f80`, `code_000d4010`, `code_000d4090`, `code_000d40e0`, `code_000d4100`, `code_000d4180`, `code_000d41c0`, `code_000d4200`, `code_000d4250`, `code_000d43f0`, `code_000d4460`, `code_000d44c0`, `remap_sticks_for_local_player`, `code_000d4650`, `code_000d4680`, `code_000d4690`, `code_000d46a0`, `code_000d4810`, `code_000d48e0`, `code_000d4900`, `code_000d4ad0`, `code_000d4b00`, `code_000d4b40`, `code_000d4b60`, `code_000d4bc0`, `get_ui_argb_white`, `code_000d4d10`, `modulate_pixel32_by_real_alpha`, `ui_widget_delete`, `ui_widgets_close_all`, `ui_widgets_close_all_for_local_player`, `ui_widgets_delete_history`, `ui_widgets_pop_stack`, `main_screen_shell_begin_fade`, `ui_play_audio_feedback_sound`, `code_000d5290`, `code_000d5530`, `draw_string_and_hack_in_icons`, `code_000d58c0`, `code_000d5bd0`, `code_000d6010`, `code_000d6060`, `code_000d6100`, `ui_widgets_dispose`, `widget_event_function_list_widget_goto_next_item`, `widget_event_function_list_widget_goto_previous_item`, `code_000d6650`, `code_000d6b40`, `code_000d6e30`, `render_ui_widgets_postgame`, `render_ui_widgets`, `code_000d7290`, `code_000d7480`, `ui_widget_load_by_name_or_tag`, `main_screen_shell_load`, `network_game_reset_to_pregame_ui`, `display_error`, `display_error_abort_to_dashboard`, `display_error_damaged_media`, `display_errors_deferred_until_cinematic_stop`, `display_scenario_help`, `code_000d8800`, `code_000d8aa0`, `process_ui_widgets`, `compute_offset_coordinate`, `display_error_text_when_main_menu_loaded`, `pool_alloc`, `pool_free`, `pop_widget`, `dispose_widget_stack`, `ui_widget_delete_children_recursive`, `ui_widget_launch_widget`, `widget_instance_can_handle_events`, `widget_instance_find_by_tag_index_recursive`, `widget_instance_give_focus_directly`, `widget_instance_give_focus_by_tag`, `play_sound_tag`, `widget_instance_get_tail_child_widget`, `ui_widget_add_child`, `push_widget`, `widget_instance_can_receive_events`, `widget_instance_set_focused_child_by_index`, `widget_instance_go_back_to_previous`, `widget_instance_find_by_tag_index`, `event_handler_dispatch`, `ui_widget_load_children_recursive`, `widget_instance_initialize`, `transition_to_game_in_progress`, `get_icon_type`, `should_flip_sticks_for_local_player`, `filesystem_initialization_thread_proc`, `perform_filesystem_initialization`, `widget_instance_reload_recursive`, `ui_widget_reload_by_tag`, `widget_instance_get_cumulative_alpha_modifier`, `widget_instance_text_box_is_focused`, `string_has_icons_to_draw`, `search_and_replace`, `widget_instance_render_text_box`, `widget_instance_render_spinner_list`, `ui_debug_targets_enabled`, `ui_debug_draw_outline`, `ui_debug_draw_mark`, `ui_debug_draw_targets`, `ui_debug_set_mark`, `ui_mouse_press`, `ui_mouse_child_index`, `ui_mouse_list_shows_several`, `ui_mouse_key_button`, `ui_mouse_widget_is_item`, `ui_mouse_note_list_step`, `ui_mouse_note_slot_band`, `ui_mouse_note_target`, `ui_mouse_merge_setting_rows`, `ui_mouse_widen_values`, `ui_mouse_label_text_right`, `ui_mouse_fit_button_targets`, `ui_mouse_target_at`, `ui_mouse_widget_has_focus`, `ui_mouse_give_focus`, `ui_mouse_list_directions`, `ui_mouse_step_list_to_slot`, `ui_mouse_wheel_widget`, `ui_mouse_wheel_skips_child`, `ui_mouse_wheel_room`, `ui_mouse_menu`, `ui_mouse_menus_active`, `ui_debug_log_click`, `ui_debug_log_keyboard_tap`, `ui_widgets_process_mouse`, `spinner_string_list_extra_count`, `ui_widget_spinner_own_item_count`, `ui_widget_spinner_extra_description`, `spinner_string_list_get_string`, `widget_instance_render_column_list`, `widget_instance_update_animation_parameters`, `spinner_list_update`, `column_list_update`, `widget_instance_tab_to_next_valid_widget`, `widget_instance_tab_to_previous_valid_widget`, `widget_takes_any_controller`, `widget_takes_events_of_controller`, `widget_instance_process_one_event_recursive`, `ui_check_for_pause_game`
- interface/ui_widget_event_handler_functions.c: `ui_widget_event_handler_function_invoke`, `code_000d9020`, `reset_last_player1_profile_index`, `code_000d9040`, `code_000d91e0`, `code_000d9210`, `code_000d9350`, `code_000d93b0`, `code_000d93e0`, `code_000d93f0`, `code_000d9400`, `code_000d9410`, `code_000d9430`, `code_000d9480`, `code_000d94c0`, `code_000d9550`, `code_000d9750`, `code_000d9770`, `code_000d9790`, `code_000d9800`, `code_000d9810`, `code_000d9850`, `code_000d9860`, `code_000d9880`, `code_000d9970`, `code_000d9990`, `code_000d9b60`, `code_000d9cc0`, `code_000d9cf0`, `code_000d9f90`, `code_000da080`, `code_000da190`, `code_000da2f0`, `code_000da320`, `code_000da4e0`, `code_000da640`, `code_000da780`, `code_000da7a0`, `code_000da880`, `code_000da8d0`, `code_000dac70`, `code_000dae90`, `code_000db1f0`, `code_000db770`, `code_000dba40`, `code_000dbfc0`, `code_000dc2e0`, `code_000dc4d0`, `code_000dc590`, `code_000dc630`, `code_000dc9c0`, `code_000dcbf0`, `code_000dcf40`, `code_000dd450`, `code_000dd730`, `code_000ddc80`, `code_000ddf90`, `code_000de190`, `code_000de250`, `code_000de360`, `code_000de390`, `code_000de490`, `code_000de5c0`, `code_000de5e0`, `code_000de630`, `code_000de6b0`, `code_000de890`, `code_000deb70`, `code_000ded40`, `code_000df080`, `code_000df0d0`, `code_000df0f0`, `code_000df200`, `code_000df340`, `code_000df390`, `code_000df3d0`, `code_000df3e0`, `code_000df560`, `code_000df650`, `code_000df6f0`, `code_000df790`, `code_000df7b0`, `code_000df7d0`, `code_000df7f0`, `code_000df810`, `code_000df820`, `code_000df830`, `code_000df880`, `code_000df8f0`, `code_000df950`, `code_000df9d0`, `code_000dfb50`, `code_000dfba0`, `code_000dfbb0`, `code_000dfc40`, `code_000dfda0`, `code_000dfdc0`, `code_000dfea0`, `code_000dfec0`, `code_000dff10`, `widget_event_function_null`, `new_campaign_decision`, `create_and_begin_editing_new_gametype_profile`, `network_game_join_game_from_server_list`, `pause_game_restart_at_checkpoint`, `pause_game_restart_level`, `pause_game_quit_to_main_menu`, `coop_game_initialize`, `multiplayer_type_menu_initialize`, `cancel_profile_delete`, `run_xdemos`, `single_player_reset_controller_choices`, `exit_to_xbox_dashboard`, `clear_multiplayer_player_joins`, `network_server_list_dispose`, `network_game_cancel`, `multiplayer_pick_quick_start_play_stage`, `multiplayer_level_list_dispose`, `playlist_profile_end_editing`, `player_profile_end_editing`, `switch_from_main_menu_to_single_player`, `network_game_server_accept_connections`, `network_game_server_defer_game_start`, `network_game_server_allow_game_start`, `disable_widget_if_no_xdemos`, `pop_history_stack_once`, `solo_level_dispose_list`, `start_new_game`, `multiplayer_profiles_list_dispose`, `player_profiles_list_dispose`, `player_profile_color_picker_menu_dispose`, `network_game_server_list_initialize`, `delete_playlist_profile_final`, `player_wants_to_join_multiplayer_game`, `playlist_profile_change_name`, `player_profile_change_name`, `network_game_remove_local_player`, `delete_player_profile_final`, `single_player_set_player1_controller_choice`, `netgame_unjoin_player`, `close_calling_widget_if_not_editing_profile`, `new_campaign_if_no_custom_player_profiles_exist`, `difficulty_set`, `display_error_if_no_network_connection`, `split_screen_game_initialize`, `single_player_set_player2_controller_choice`, `player_profile_save_changes`, `start_network_game_if_no_advertised_servers`, `delete_player_profile_request`, `player_profile_color_picker_select_color`, `playlist_profile_set_game_engine`, `multiplayer_game_swap_teams`, `network_game_start_faster`, `network_game_start_slower`, `create_and_begin_editing_new_player_profile`, `multiplayer_level_list_initialize`, `playlist_profile_begin_editing`, `player_profile_begin_editing`, `delete_playlist_profile_request`, `player_profile_color_picker_menu_initialize`, `difficulty_menu_initialize`, `playlist_profile_initialize_game_engine`, `playlist_profile_save_changes`, `multiplayer_profiles_list_initialize`, `ui_widget_function_denied`, `new_campaign_chosen`, `network_game_start_new_server`, `playlist_profile_initialize_name`, `playlist_profile_change_item_options`, `playlist_profile_change_indicator_options`, `playlist_profile_change_ctf_rules`, `playlist_profile_change_racing_rules`, `playlist_profile_change_koth_rules`, `playlist_profile_change_slayer_rules`, `player_profile_set_for_game_1wide`, `player_profile_initialize_controller_settings`, `solo_level_initialize_list_coop`, `player_profile_change_advanced_controller_settings`, `solo_level_set_next_map_name`, `netgame_join_player`, `player_profiles_list_initialize`, `player_profile_set_for_game_3wide`, `playlist_profile_initialize_indicator_options`, `playlist_profile_initialize_item_options`, `player_profile_initialize_advanced_controller_settings`, `player_profile_change_controller_settings`, `playlist_profile_initialize_racing_rules`, `playlist_profile_initialize_slayer_rules`, `playlist_profile_initialize_ctf_rules`, `playlist_profile_initialize_koth_rules`, `playlist_profile_initialize_oddball_rules`, `playlist_profile_initialize_player_options`, `playlist_profile_change_oddball_rules`, `playlist_profile_change_player_options`, `multiplayer_level_select`, `multiplayer_profile_set_for_game`, `solo_level_initialize_list_single_player`
- interface/ui_widget_game_data_input_functions.c: `ui_widget_game_data_function_invoke`, `code_000e0210`, `code_000e0220`, `code_000e0330`, `code_000e03e0`, `code_000e0490`, `code_000e0510`, `code_000e0620`, `code_000e06b0`, `code_000e0e90`, `code_000e1650`, `code_000e1b10`, `code_000e1c30`, `code_000e1ce0`, `code_000e1e10`, `code_000e1ea0`, `code_000e1fd0`, `code_000e2060`, `code_000e2110`, `code_000e21c0`, `code_000e2280`, `code_000e2310`, `code_000e24d0`, `code_000e25e0`, `code_000e2650`, `code_000e26e0`, `code_000e2790`, `code_000e2950`, `code_000e2a00`, `code_000e2aa0`, `code_000e2b50`, `code_000e2b80`, `code_000e2cc0`, `code_000e2d10`, `code_000e2dc0`, `code_000e2e10`, `code_000e2ec0`, `code_000e2fd0`, `code_000e30e0`, `code_000e3110`, `code_000e3140`, `code_000e3400`, `code_000e3990`, `code_000e3e60`, `code_000e42e0`, `code_000e4470`, `widget_function_null`, `settings_menu_update_extended_description`, `playlist_settings_menu_update_extended_description`, `playlist_gametype_select_menu_update_extended_description`, `multiplayer_type_menu_update_extended_description`, `difficulty_select_menu_update_extended_description`, `server_list_menu_update`, `network_pregame_status_screen_update`, `splitscreen_pregame_status_screen_update`, `netgame_prejoin_players`, `set_textbox_to_build_number`, `solo_game_objective_text`, `main_menu_animation_fakery`, `get_active_player_profile_display_name`, `get_editable_player_profile_display_name`, `get_editable_playlist_profile_display_name`, `get_active_player_profile_color_index`, `player_profile_edit_select_menu_update_extended_description`, `game_options_menu_update_text_desc`, `game_options_menu_update_pic_desc`, `multiplayer_game_set_text_box_for_map_name`, `multiplayer_game_set_text_box_for_game_ruleset`, `multiplayer_game_set_text_box_for_teams_noteams`, `multiplayer_game_set_text_box_for_score_limit`, `multiplayer_game_set_text_box_for_score_limit_type`, `multiplayer_game_set_bitmap_for_map`, `multiplayer_game_set_bitmap_for_ruleset`, `multiplayer_game_set_text_box_for_number_of_players`, `multiplayer_edit_profile_set_ruleset_textbox_string_index`, `system_link_status_check`, `multiplayer_game_directions`, `teams_no_teams_mp_game_bitmap_update`, `warn_if_difficulty_will_nuke_saved_game`, `dim_if_no_system_link_cable`, `spinner_list_3wide_determine_displayed_item_indices`, `player_profile_update_cache_for_nwide_list`, `variant_profile_update_cache_for_nwide_list`, `mutliplayer_settings_select_list_update_displayed_items`, `player_profile_3wide_list_update`, `player_profile_1wide_list_update`, `solo_level_select_list_update_displayed_items`, `player_profile_color_picker_update`, `mp_level_select_list_update_displayed_items`, `list_indices_sort_proc`, `filter_invalid_list_indices`
- interface/ui_widget_group.c: `ui_load_tags_for_scenario`
- interface/ui_widget_text_search_and_replace_functions.c: `ui_widget_search_and_replace_invoke`, `code_000e4a90`, `code_000e4aa0`, `widget_replace_function_null`, `widget_controller`
- interface/virtual_keyboard.c: `virtual_keyboard_initialize`, `virtual_keyboard_dispose`, `virtual_keyboard_launch`, `virtual_keyboard_active`, `virtual_keyboard_last_exit_saved_text`, `virtual_keyboard_tab_left`, `virtual_keyboard_tab_right`, `virtual_keyboard_tab_up`, `virtual_keyboard_tab_down`, `virtual_keyboard_cancel`, `virtual_keyboard_get_character`, `virtual_keyboard_get_current_character`, `virtual_keyboard_render_internal`, `virtual_keyboard_free_space_in_text_buffer`, `virtual_keyboard_backspace`, `virtual_keyboard_close`, `virtual_keyboard_render`, `virtual_keyboard_select`, `virtual_keyboard_process_internal`, `virtual_keyboard_process`, `virtual_keyboard_target_rectangles`, `virtual_keyboard_click`

## `items`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `equipment_place` | items/equipment.c | Matched | import/src/level.rs:1040 | owed: the engine cites it, with no check against the original yet |
| `equipment_handle_pickup` | items/equipment.c | Matched | sim/src/player.rs:1221 | owed: items; reads the object, tag and map world, which the fake worlds do not hold |
| `item_adjust_for_angular_velocity_change` | items/items.c | Ported | sim/src/item_physics.rs:393 | `tools/engine_item_reference.py` |
| `item_accelerate` | items/items.c | Ported | sim/src/forces.rs:211 | `tools/engine_item_reference.py` |
| `item_align_to_normal_and_point` | items/items.c | Ported | sim/src/item_physics.rs:376 | `tools/engine_item_reference.py` |
| `item_update` | items/items.c | Ported | sim/src/item_physics.rs:1 | `tools/engine_item_reference.py` |
| `random_vector_in_cone3d` | items/projectiles.c | Matched | sim/src/projectile.rs | owed: the engine's own tests (sim/src/projectile.rs); no check against the original yet |
| `projectile_new` | items/projectiles.c | Matched | sim/src/projectile.rs | owed: the engine's own tests (sim/src/projectile.rs); no check against the original yet |
| `projectile_update` | items/projectiles.c | Matched | sim/src/projectile.rs:70 | owed: the engine cites it, with no check against the original yet |
| `weapon_place` | items/weapons.c | Matched | import/src/level.rs:1040 | owed: the engine cites it, with no check against the original yet |
| `weapon_new` | items/weapons.c | Matched | sim/src/weapon.rs:170 | owed: the engine cites it, with no check against the original yet |
| `weapon_export_function_values` | items/weapons.c | Matched | sim/src/functions.rs:156 | owed: the engine cites it, with no check against the original yet |
| `weapon_update` | items/weapons.c | Matched | sim/src/weapon.rs:2 | owed: the engine cites it, with no check against the original yet |
| `item_rested_surface_index` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `item_rested_surface` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `item_set_rested_surface` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `projectile_calculate_deceleration_from_distances` | items/projectiles.c | Matched | sim/src/projectile.rs | owed: the engine's own tests (sim/src/projectile.rs); no check against the original yet |
| `projectile_calculate_deceleration` | items/projectiles.c | Matched | sim/src/projectile.rs | owed: the engine's own tests (sim/src/projectile.rs); no check against the original yet |
| `projectile_collision_test_line` | items/projectiles.c | Matched | sim/src/projectile.rs | owed: the engine's own tests (sim/src/projectile.rs); no check against the original yet |
| `projectile_detonate` | items/projectiles.c | Matched | sim/src/effects.rs:178 | owed: the engine cites it, with no check against the original yet |
| `projectile_collision` | items/projectiles.c | Matched | sim/src/effects.rs:177 | owed: the engine cites it, with no check against the original yet |
| `weapon_magazine_finish_reload` | items/weapons.c | Matched | sim/src/weapon.rs:583 | owed: weapons; reads the object, tag and map world, which the fake worlds do not hold (the weapon tag) |
| `weapon_trigger_recover` | items/weapons.c | Matched | sim/src/weapon.rs:488 | owed: weapons; reads the object, tag and map world, which the fake worlds do not hold (the weapon tag) |
| `weapon_magazine_start_reload` | items/weapons.c | Matched | sim/src/weapon.rs:223 | owed: weapons; reads the object, tag and map world, which the fake worlds do not hold (the weapon tag) |
| `weapon_trigger_can_fire_again` | items/weapons.c | Matched | sim/src/weapon.rs:423 | owed: weapons; reads the object, tag and map world, which the fake worlds do not hold (the weapon tag) |
| `weapon_effect_new` | items/weapons.c | Matched | sim/src/lib.rs:579 | owed: weapons; reads the object, tag and map world, which the fake worlds do not hold; Phase 12's effects |
| `projectile_distribute` | items/weapons.c | Matched | sim/src/projectile.rs | owed: the engine's own tests (sim/src/projectile.rs); no check against the original yet |
| `trigger_create_projectiles` | items/weapons.c | Matched | sim/src/projectile.rs | owed: the engine's own tests (sim/src/projectile.rs); no check against the original yet |
| `weapon_trigger_fire` | items/weapons.c | Matched | sim/src/weapon.rs:495 | owed: weapons; reads the object, tag and map world, which the fake worlds do not hold (the weapon tag); Phase 12 |
| `weapon_trigger_begin_firing` | items/weapons.c | Matched | sim/src/weapon.rs:439 | owed: weapons; reads the object, tag and map world, which the fake worlds do not hold (the weapon tag) |
| `weapon_trigger_release_charge` | items/weapons.c | Matched | sim/src/weapon.rs:465 | owed: weapons; reads the object, tag and map world, which the fake worlds do not hold (the weapon tag) |

151 Missing (97 reached from `main_loop`), by file:

- items/equipment.c: `equipment_definition_handle_pickup`
- items/garbage.c: `garbage_update`, `garbage_new`
- items/items.c: `object_get_type`, `items_initialize`, `items_initialize_for_new_map`, `items_dispose_from_old_map`, `items_dispose`, `item_new`, `item_delete`, `verify_item_location`, `dangerous_items_near_player`, `item_in_unit_inventory`, `item_get_position_even_if_in_inventory`, `item_detonate`, `valid_real_vector3d_axes3`, `valid_real_matrix4x3`
- items/projectiles.c: `projectiles_initialize`, `projectiles_initialize_for_new_map`, `projectiles_dispose_from_old_map`, `projectiles_dispose`, `projectile_kill_tracer`, `projectiles_delete_all`, `projectile_delete`, `projectile_set_target_object_index`, `projectile_make_tracer`, `projectile_get_ballistic_acceleration`, `projectile_estimate_time_to_target`, `dangerous_projectiles_near_player`, `projectile_handle_deleted_object`, `code_000e7630`, `code_000e7650`, `projectile_export_function_values`, `code_000e7790`, `projectile_handle_parent_destroyed`, `projectile_aim_ballistic`, `projectile_aim_linear`, `projectile_aim`, `code_000e7d80`, `code_000e7e30`, `code_000e7f10`, `code_000e8110`, `projectile_accelerate`, `code_000e88c0`, `projectile_adjust_for_angular_velocity_change`, `projectile_set_action`, `projectile_effect_new`
- items/weapon_definitions.c: `weapon_definition_get_damage_potential`
- items/weapons.c: `animation_convert_frame_to_pal`, `animation_key_frame_index`, `animation_update`, `animation_choose_random_permutation`, `weapons_initialize`, `weapons_initialize_for_new_map`, `weapons_dispose_from_old_map`, `weapons_dispose`, `weapon_preprocess_node_orientations`, `weapon_get_label`, `weapon_set_integrated_light_power`, `weapon_estimate_time_to_target`, `weapon_can_be_fired`, `weapon_useful`, `weapon_compute_movement_penalty`, `weapon_melee_attack`, `weapon_must_be_readied`, `weapon_is_flag`, `weapon_prevents_grenade_throwing`, `weapon_get_first_person_animation_time`, `weapon_overcharged`, `code_000eab30`, `code_000eab80`, `code_000eabd0`, `code_000eac20`, `code_000eac60`, `code_000eac90`, `code_000eacd0`, `code_000ead20`, `code_000eadb0`, `code_000eaea0`, `code_000eaef0`, `code_000eafe0`, `code_000eb050`, `code_000eb090`, `code_000eb120`, `code_000eb1a0`, `code_000eb1f0`, `code_000eb210`, `code_000eb230`, `weapon_set_total_rounds`, `power`, `random`, `weapon_delete`, `weapon_handle_potential_inventory_item`, `weapon_owner_update`, `weapon_build_weapon_interface_state`, `weapon_reloading`, `weapon_rotate_zoom_level`, `weapon_get_zoom_magnification`, `weapon_get_field_of_view`, `weapon_prevents_melee_attack`, `code_000ec1a0`, `code_000ec300`, `code_000ec3e0`, `code_000ec4a0`, `code_000ec520`, `code_000ec5e0`, `code_000ec670`, `code_000ec6d0`, `code_000ec730`, `code_000ec8c0`, `code_000ec960`, `weapon_set_current_amount`, `weapon_ready`, `weapon_put_away`, `weapon_aim`, `weapon_stop_reload`, `code_000ecd30`, `code_000ecd80`, `code_000ed4a0`, `code_000edc60`, `code_000eded0`, `code_000edfa0`, `code_000ee0a0`, `weapon_precache_projectile_trails`, `weapon_magazine_start_chamber`, `weapon_magazine_finish_chamber`, `weapon_trigger_fully_charged`, `weapon_trigger_idle`, `weapon_trigger_locked`, `weapon_trigger_finish_tracking`, `weapon_state_next`, `weapon_trigger_get`, `weapon_magazine_get`, `weapon_trigger_count`, `weapon_trigger_get_charged_fraction`, `weapon_busy`, `weapon_magazine_state_change_ok`, `weapon_get_effect_object_index`, `weapon_get_owner_object_index`, `weapon_get_projectile_owner_object_index`, `weapon_magazine_idle`, `weapon_effect_looping_new`, `weapon_detonate`, `weapon_trigger_change_state`, `weapon_state_key_frame`, `weapon_magazine_state_interruptable`, `weapon_reset`, `weapon_state_interruptable`, `weapon_set_state`, `weapon_trigger_overload`, `weapon_trigger_overcharged`

## `main`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `main_menu_unload` | main/main.c | Replaced |  | through its caller `network_game_create_game_objects` |
| `main_get_window_count` | main/main.c | Replaced |  | through its caller `_rasterizer_detail_objects_begin` |
| `main_framerate_render` | main/main.c | Replaced |  | through its caller `interface_draw_fullscreen_overlays` |
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

141 Missing (91 reached from `main_loop`), by file:

- main/console.c: `console_initialize`, `console_initialize_for_new_map`, `console_open`, `console_close`, `console_is_active`, `console_printf`, `console_warning`, `code_000eede0`, `code_000eee50`, `code_000eee90`, `console_startup`, `console_dispose`, `console_update`, `console_process_command`, `console_get_text_to_autocomplete`, `console_complete`
- main/main.c: `main_get_seconds_elapsed`, `gamepad_button_is_down`, `game_connection_set`, `game_connection`, `main_disallow_persistent_storage`, `main_set_map_name`, `main_defer_map_map_change`, `main_set_multiplayer_map_name`, `main_get_map_name`, `main_get_multiplayer_map_name`, `main_set_difficulty`, `main_get_difficulty`, `sort_desired_local_player_controllers`, `code_000ef8e0`, `main_reset_map`, `main_revert_map`, `main_skip_cinematic`, `main_save_map_nonsafe`, `main_saving_map`, `main_save_cancel`, `main_save_map_safe`, `main_won_map`, `main_lost_map`, `main_respawn`, `main_save_core`, `main_save_core_name`, `main_load_core`, `main_load_core_at_startup`, `main_load_core_name`, `main_load_core_name_at_startup`, `main_switch_structure_bsp`, `main_skip`, `main_queue_map_name`, `code_000efde0`, `main_goto_main_menu`, `main_menu_precache_resources`, `main_menu_ensure_player_queues_exist`, `main_menu_fade_active`, `main_menu_switch_to_single_player`, `main_set_game_connection_to_film_playback`, `main_get_solo_level_from_name`, `main_get_current_solo_level`, `main_get_solo_level_name`, `main_run_demos`, `code_000f00b0`, `compute_window_bounds`, `code_000f0350`, `code_000f0420`, `code_000f05a0`, `code_000f05c0`, `code_000f05f0`, `code_000f06a0`, `code_000f06c0`, `code_000f07a0`, `code_000f07d0`, `code_000f0810`, `code_000f0850`, `code_000f08b0`, `code_000f08d0`, `code_000f08f0`, `code_000f0930`, `code_000f0940`, `code_000f0b60`, `code_000f0bb0`, `main_framerate_throttle_enabled`, `code_000f0bf0`, `main_rasterizer_throttle`, `code_000f13a0`, `main_taking_screenshot`, `main_movie_start`, `main_movie_stop`, `main_stop_time`, `main_start_time`, `main_crash`, `main_print_version`, `main_vertical_blank_interrupt_handler`, `main_save_current_solo_map`, `main_load_last_solo_map`, `main_save_map_no_timeout`, `main_load_ui_scenario`, `main_menu_load`, `main_roll_credits`, `main_pregame_render`, `set_window_camera_values`, `main_present_frame`, `code_000f1c20`, `code_000f1ce0`, `code_000f1d30`, `halt_and_catch_fire`, `main_loop_of_death`, `code_000f24a0`, `main_loop`, `create_local_players`, `main_queue_map_private`, `screenshot_record`, `compute_subframe_counts`, `main_new_map`, `main_change_map_name`, `main_native_build_label`, `main_native_error_tail`, `main_coop_host`, `main_revert_map_private`, `main_skip_cinematic_private`, `main_skip_private`, `main_saving_map_private`, `main_save_map_private`, `main_switch_to_structure_bsp_private`, `main_death_timer_expired`, `main_lost_map_private`, `main_respawn_private`, `main_load_core_private`, `main_save_core_private`, `main_run_demos_private`, `main_frame_rate_debug`, `main_won_map_private`, `main_exit`, `main_reset_time`, `main_update_time_unthrottled`, `main_update_time`, `main_setup_connection`, `main_initialize_time`, `main_reset_map_private`, `screenshot_render`, `main_game_render`, `main_loop_iteration`

## `math`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `geosphere_dispose` | math/geometry.c | Ported |  | `tools/engine_math_reference.py` |
| `vector_intersect_plane2d` | math/geometry.c | Replaced |  | through its caller `convex_polygon2d_clip_to_plane` |
| `convex_polygon2d_clip_to_plane` | math/geometry.c | Replaced |  | through its caller `convex_hull2d_intersect` |
| `convex_polygon3d_clip_to_plane` | math/geometry.c | Replaced |  | through its caller `portal_hull_from_points` |
| `convex_hull2d_intersect` | math/geometry.c | Replaced |  | through its caller `structure_visibility_find_mirror` |
| `geosphere_new` | math/geometry.c | Ported | sim/src/random.rs:5 | `tools/engine_math_reference.py` |
| `matrix4x3_rotation_from_quaternion` | math/matrix_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `matrix4x3_from_orientation` | math/matrix_math.c | Ported | sim/src/animation.rs:104 | `tools/engine_animation_reference.py` |
| `matrix4x3_transform_normal` | math/matrix_math.c | Ported | sim/src/animation.rs:169 | `tools/engine_math_reference.py` |
| `matrix4x3_inverse_transform_normal` | math/matrix_math.c | Ported | sim/src/animation_blend.rs:178 | `tools/engine_math_reference.py` |
| `matrix4x3_multiply` | math/matrix_math.c | Ported | sim/src/animation.rs:131 | `tools/engine_animation_reference.py` |
| `matrix4x3_rotation_from_vectors` | math/matrix_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `matrix4x3_rotation_from_angles` | math/matrix_math.c | Matched | sim/src/objects.rs:399 | owed: rotations; checked through the item trace's object matrices |
| `matrix4x3_from_point_and_vectors` | math/matrix_math.c | Ported | sim/src/animation.rs:93 | `tools/engine_animation_reference.py` |
| `periodic_functions_dispose` | math/periodic_functions.c | Replaced |  | through its caller `real_math_dispose` |
| `periodic_function_evaluate` | math/periodic_functions.c | Ported | sim/src/periodic.rs:165 | `tools/engine_periodic_reference.py` |
| `transition_function_evaluate` | math/periodic_functions.c | Ported | sim/src/damage.rs:37 | `tools/engine_periodic_reference.py` |
| `transition_function_build_table` | math/periodic_functions.c | Ported | sim/src/periodic.rs:199 | `tools/engine_periodic_reference.py` |
| `periodic_function_build_table` | math/periodic_functions.c | Ported | sim/src/periodic.rs:93 | `tools/engine_periodic_reference.py` |
| `periodic_functions_initialize` | math/periodic_functions.c | Ported | sim/src/periodic.rs:45 | `tools/engine_periodic_reference.py` |
| `get_number_suitable_for_initializing_random_seed` | math/random_math.c | Replaced |  | through its caller `random_math_initialize` |
| `random_math_initialize` | math/random_math.c | Replaced |  | through its caller `real_math_initialize` |
| `random_math_dispose` | math/random_math.c | Replaced |  | through its caller `real_math_dispose` |
| `real_seed_random` | math/random_math.c | Ported | audio/src/choice.rs:30 | `tools/engine_item_reference.py` |
| `real_seed_random_range` | math/random_math.c | Ported | ffi/src/decals.rs:126 | `tools/engine_math_reference.py` |
| `seed_random` | math/random_math.c | Ported | audio/src/choice.rs:24 | `tools/engine_math_reference.py` |
| `seed_random_range` | math/random_math.c | Ported | audio/src/choice.rs:36 | `tools/engine_math_reference.py` |
| `direction3d_from_table` | math/random_math.c | Ported |  | `tools/engine_math_reference.py` |
| `seed_random_direction3d` | math/random_math.c | Matched | sim/src/random.rs:55 | owed: the engine cites it, with no check against the original yet |
| `seed_random_vector_in_cone3d` | math/random_math.c | Ported | sim/src/random.rs:66 | `tools/engine_math_reference.py` |
| `subdivide_triangle` | math/geometry.c | Ported | sim/src/random.rs:178 | `tools/engine_math_reference.py` |
| `get_face_vertex` | math/geometry.c | Ported | sim/src/random.rs:193 | `tools/engine_math_reference.py` |
| `get_edge_vertex` | math/geometry.c | Ported | sim/src/random.rs:224 | `tools/engine_math_reference.py` |
| `calculate_vertex` | math/geometry.c | Ported | sim/src/random.rs:255 | `tools/engine_math_reference.py` |
| `set_rectangle2d` | math/integer_math.c | Replaced |  | through its caller `rasterizer_draw_string` |
| `rectangle2d_width` | math/integer_math.c | Replaced |  | through its caller `draw_string` |
| `offset_rectangle2d` | math/integer_math.c | Replaced |  | through its caller `bink_draw_frame` |
| `bit_vector_not` | math/integer_math.c | Replaced |  | through its caller `ai_debug_communication_toggle_bits` |
| `periodic_function_build_variable_period_x_table` | math/periodic_functions.c | Ported | sim/src/periodic.rs:73 | `tools/engine_periodic_reference.py` |
| `real_math_initialize` | math/real_math.c | Replaced |  | through its caller `shell_initialize` |
| `real_math_dispose` | math/real_math.c | Replaced |  | through its caller `shell_dispose` |
| `rotate_vector_about_axis` | math/real_math.c | Ported | ffi/src/sprite.rs:102 | `tools/engine_item_reference.py` |
| `vectors_interpolate` | math/real_math.c | Replaced |  | through its caller `get_flash_color` |
| `points_interpolate` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `scalars_interpolate` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `scalars_interpolate_and_clamp_0_to_1` | math/real_math.c | Replaced |  | through its caller `rasterizer_screen_effect_get_cinematic_parameters` |
| `component_vectors_from_normal3d` | math/real_math.c | Ported |  | `tools/engine_vehicle_function_reference.py` |
| `quaternions_multiply` | math/real_math.c | Ported | sim/src/animation.rs:221 | `tools/engine_animation_reference.py` |
| `quaternions_interpolate` | math/real_math.c | Ported | sim/src/animation.rs:208 | `tools/engine_animation_reference.py` |
| `quaternion_transform_point` | math/real_math.c | Ported |  | `tools/engine_item_reference.py` |
| `point_in_triangle3d` | math/real_math.c | Replaced |  | through its caller `structure_render_surface_from_point_and_leaf` |
| `quantize_real_to_byte_lower_bound` | math/real_math.c | Matched | sim/src/vehicle_animation.rs:171 | owed: the engine's own tests (sim/tests/vehicle_traces.rs); no check against the original yet; only from 0 to 1, where the original's step down to the byte below the value is the floor of value * 255 |
| `angle_between_vectors3d` | math/real_math.c | Ported | sim/src/vehicle.rs:917 | `tools/engine_math_reference.py` |
| `quaternion_normalize` | math/real_math.c | Ported | sim/src/animation.rs:190 | `tools/engine_animation_reference.py` |
| `quaternions_interpolate_and_normalize` | math/real_math.c | Ported | sim/src/animation.rs:202 | `tools/engine_animation_reference.py` |
| `orientations_interpolate` | math/real_math.c | Replaced |  | through its caller `first_person_weapon_build_node_matrices` |
| `euler_angles2d_from_vector3d` | math/real_math.c | Ported | sim/src/animation_blend.rs:179 | `tools/engine_math_reference.py` |
| `vector3d_from_euler_angles2d` | math/real_math.c | Matched | content/src/level.rs:375 | owed: the engine cites it, with no check against the original yet |
| `accelerate_to_position` | math/real_math.c | Matched | sim/src/device_animation.rs:3 | owed: a first-person weapon's sway; checked through first_person_weapon_update's trace |

157 Missing (65 reached from `main_loop`), by file:

- math/geometry.c: `convex_hull3d_verify`, `plane2d_distance_to_point`, `vector_intersect_plane3d`, `code_000f5050`, `build_torus`, `code_000f54a0`, `convex_hull2d`, `convex_hull2d_verify`, `convex_hull2d_test_circle`, `convex_hull2d_test_point`, `convex_hull2d_test_point_indexed`, `convex_hull2d_area`, `convex_hull2d_perimeter`, `convex_hull2d_test_vector`, `convex_polygon2d_verify`, `convex_polygon3d_verify`, `convex_hull3d_begin`, `convex_hull3d_expand`, `convex_hull3d`, `convex_hull3d_test_point`, `convex_hull3d_test_vector`, `code_000f76e0`, `code_000f7a90`, `code_000f7c20`, `points_dimension2d`
- math/integer_math.c: `set_point2d`, `offset_point2d`, `rectangle2d_height`, `inset_rectangle2d`, `adjust_rectangle2d`, `intersect_rectangles2d`, `rectangle2d_hull_from_rectangles2d`, `point2d_in_rectangle2d`, `interior_rectangle2d`, `equal_rectangle2d`, `equal_point2d`, `floor_log2`, `ceiling_log2`, `floor_power2`, `ceiling_power2`, `integer_square_root`, `bit_vector_and`, `bit_vector_or`, `scale_rectangle2d`
- math/matrix_math.c: `matrix4x3_identity`, `matrix4x3_transpose`, `matrix4x3_inverse`, `matrix4x3_scale`, `matrix4x3_translation`, `matrix4x3_rotation_from_axis_and_angle`, `matrix4x3_from_point_and_quaternion`, `matrix4x3_to_point_and_vectors`, `matrix4x3_transform_point`, `matrix4x3_transform_vector`, `matrix4x3_inverse_transform_point`, `matrix4x3_inverse_transform_vector`, `matrix3x3_determinant`, `matrix3x3_transpose`, `matrix3x3_inverse`, `matrix3x3_from_axis_and_angle`, `matrix3x3_multiply`, `matrix3x3_transform_vector`, `matrix4x3_rotation_to_angles`, `matrix4x3_rotation_to_quaternion`, `vector_from_matrices4x3`, `matrix4x3_transform_plane`, `matrix4x3_inverse_transform_plane`, `matrix3x3_from_forward_and_up`, `matrix3x3_rotation_to_quaternion`, `valid_real_plane3d`, `matrix4x3_from_plane`
- math/periodic_functions.c: `@periodic_function_build_variable_period_x_table`
- math/probability.c: `factorial`, `permutations`, `combinations`, `permute`, `combine`
- math/random_math.c: `lock_global_random_seed`, `unlock_global_random_seed`, `get_global_random_seed_address`, `get_random_seed`, `get_global_local_random_seed_address`, `random_seed_debug_log`, `seed_random_orientation`
- math/real_math.c: `magnitude_squared3d_right_associated`, `real_math_reset_precision`, `perpendicular2d`, `perpendicular3d`, `perpendicular4d`, `component_vectors_from_direction3d`, `vectors3d_from_euler_angles3d`, `fast_vector_intersects_sphere`, `fast_vector_intersection_with_sphere`, `point_in_rectangle2d`, `point_in_rectangle3d`, `circle_intersects_rectangle2d`, `point_in_cone2d`, `point_in_cone3d`, `point_in_sector2d`, `point_in_sector3d`, `circle_intersects_cone2d`, `circle_intersects_sector2d`, `sphere_intersects_cone3d`, `sphere_intersects_sector3d`, `point_in_triangle2d`, `circle_intersects_triangle2d`, `pill_intersects_triangle2d`, `pill_intersects_rectangle2d`, `sphere_intersects_rectangle3d`, `sphere_intersects_triangle3d`, `pill_intersects_triangle3d`, `vector_intersects_rectangle2d`, `vector_intersects_pill2d`, `vector_intersects_pill3d`, `vector_intersects_triangle2d`, `vector_intersects_triangle3d`, `vector_intersects_rectangle3d`, `pill_intersects_rectangle3d`, `fast_normalize2d`, `fast_normalize3d`, `cross_product_magnitude3d`, `dequantize_byte_to_real`, `quantize_real_to_byte_upper_bound`, `quantize_real_to_byte_rectangle3d`, `signed_angle_between_vectors2d`, `angle_between_vectors2d`, `angle_between_normals3d`, `yaw_vectors`, `pitch_vectors`, `fast_normals_interpolate`, `normals_interpolate`, `pin_normal_to_cone3d`, `reflect_vector3d`, `refract_vector3d`, `quaternion_from_angle_and_vector`, `quaternion_to_angle_and_vector`, `vector3d_from_angle`, `point_in_pill2d`, `point_to_line_distance_squared3d`, `vector_to_line_distance_squared3d`, `sphere_test_vector3d`, `pill_test_vector3d`, `point_from_planes3d`, `line_from_planes3d`, `angular_accelerate_to_position`, `accelerate_to_velocity3d`, `accerate_to_position3d`, `accelerate_to_velocity`, `valid_real_sine_cosine`
- math/real_math.h: `set_real_point3d`
- math/vector_tree.c: `vector_tree_new`, `vector_tree_delete`, `code_00100040`, `code_00100080`, `vector_tree_find`, `vectors_equal`, `vector_tree_add_node`

## `memory`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `byte_swap_memory` | memory/byte_swapping.c | Replaced |  | through its caller `data_decode_memory` |
| `byte_swap_data` | memory/byte_swapping.c | Replaced |  | through its caller `data_packet_group_append_packet_header` |
| `circular_queue_reset` | memory/circular_queue.c | Replaced |  | through its caller `network_client_unreliable_connection_read` |
| `circular_queue_verify` | memory/circular_queue.c | Replaced |  | through its caller `circular_queue_delete` |
| `circular_queue_new` | memory/circular_queue.c | Replaced |  | through its caller `network_connection_create_client_from_endpoint` |
| `circular_queue_delete` | memory/circular_queue.c | Replaced |  | through its caller `network_connection_delete` |
| `circular_queue_size` | memory/circular_queue.c | Replaced |  | through its caller `circular_queue_dequeue_data` |
| `circular_queue_free_space` | memory/circular_queue.c | Replaced |  | through its caller `network_connection_idle` |
| `circular_queue_queue_data` | memory/circular_queue.c | Replaced |  | through its caller `network_connection_idle` |
| `circular_queue_dequeue_data` | memory/circular_queue.c | Replaced |  | through its caller `network_client_reliable_connection_read` |
| `datum_new` | memory/data.c | Matched | ffi/src/particles/slots.rs:3 | owed: the engine cites it, with no check against the original yet |
| `data_next_index` | memory/data.c | Matched | ffi/src/particles/slots.rs:4 | owed: the engine cites it, with no check against the original yet |
| `data_prev_index` | memory/data.c | Replaced |  | through its caller `ai_debug_change_selected_encounter` |
| `data_encode_new` | memory/data_encoding.c | Replaced |  | through its caller `data_packet_encode` |
| `data_encode_memory` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_encode` |
| `data_encode_integer` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_encode` |
| `data_encode_string` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_encode` |
| `data_decode_new` | memory/data_encoding.c | Replaced |  | through its caller `data_packet_decode` |
| `data_decode_memory` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_decode` |
| `data_decode_byte` | memory/data_encoding.c | Replaced |  | through its caller `data_decode_integer` |
| `data_decode_short` | memory/data_encoding.c | Replaced |  | through its caller `data_decode_integer` |
| `data_decode_long` | memory/data_encoding.c | Replaced |  | through its caller `data_decode_integer` |
| `data_decode_integer` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_decode` |
| `data_decode_string` | memory/data_encoding.c | Replaced |  | through its caller `_data_packet_decode` |
| `data_packet_group_initialize` | memory/data_packet_groups.c | Replaced |  | through its caller `initialize_network_game_packets` |
| `data_packet_group_decode_packet` | memory/data_packet_groups.c | Replaced |  | through its caller `decode_network_game_message` |
| `data_packet_group_append_packet_header` | memory/data_packet_groups.c | Replaced |  | through its caller `data_packet_group_encode_packet` |
| `data_packet_group_encode_packet` | memory/data_packet_groups.c | Replaced |  | through its caller `encode_network_game_message` |
| `data_packet_verify` | memory/data_packets.c | Replaced |  | through its caller `data_packet_decode` |
| `data_packet_encode` | memory/data_packets.c | Replaced |  | through its caller `data_packet_group_encode_packet` |
| `data_packet_decode` | memory/data_packets.c | Replaced |  | through its caller `data_packet_group_decode_packet` |
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
| `_byte_swap_data` | memory/byte_swapping.c | Replaced |  | through its caller `byte_swap_data` |
| `_data_packet_verify` | memory/data_packets.c | Replaced |  | through its caller `_data_packet_decode` |
| `_data_packet_encode` | memory/data_packets.c | Replaced |  | through its caller `data_packet_encode` |
| `_data_packet_decode` | memory/data_packets.c | Replaced |  | through its caller `data_packet_decode` |
| `getoct` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `tar` |
| `strtime` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `tar` |
| `ExprMatch` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `matchname` |
| `makedir` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `tar` |
| `matchname` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `tar` |
| `tar` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `main` |
| `help` | memory/zlib/contrib/untgz/untgz.c | Replaced |  | through its caller `main` |
| `maketree` | memory/zlib/maketree.c | Replaced |  | through its caller `main` |
| `zcalloc` | memory/zlib/zutil.c | Replaced |  | through its caller `main` |
| `zcfree` | memory/zlib/zutil.c | Replaced |  | through its caller `main` |

196 Missing (61 reached from `main_loop`), by file:

- memory/array.c: `dynamic_array_new`, `dynamic_array_resize`, `dynamic_array_delete`, `dynamic_array_add_element`, `dynamic_array_get_element`, `dynamic_array_delete_element`, `static_array_new`, `static_array_resize`, `static_array_add_element`, `static_array_get_element`, `static_array_delete_element`
- memory/byte_swapping.c: `code_00108010`, `byte_swap_codes_size`, `byte_swap_data_explicit`
- memory/crc.c: `crc_new`, `build_crc_table`, `crc_checksum_buffer`
- memory/data.c: `data_allocation_size`, `data_initialize`, `datum_try_and_get`, `datum_get`, `data_verify`, `datum_initialize`, `data_new`, `data_dispose`, `data_make_invalid`, `datum_new_at_index`, `datum_delete`, `data_delete_all`, `data_iterator_new`, `data_iterator_next`, `data_compact`, `data_make_valid`, `data_usable`
- memory/data_compress.c: `data_compress`, `data_decompressed_size`, `data_decompress`
- memory/data_encoding.c: `data_encode_structures`, `data_encode_array`, `data_decode_structures`, `data_decode_int64`, `data_decode_array`
- memory/data_packet_groups.c: `data_packet_groups_get_error`
- memory/data_packets.c: `code_0010a5f0`, `code_0010a7c0`, `code_0010aac0`
- memory/hashtable.c: `hashtable_new`, `hashtable_set_user_data`, `hashtable_delete`, `default_hash_function`, `code_0010b270`, `hashtable_get`, `hashtable_remove`, `code_0010b630`, `hashtable_grow`, `hashtable_put`, `hashtable_search`, `hashtable_capacious_put`
- memory/lra_cache.c: `lra_full`, `lra_default_update_proc`, `lra_default_delete_proc`, `lra_block_delete`, `verify_lra_cache_block`, `verify_lra_cache`, `lra_new`, `lra_dispose`, `lra_flush`, `lra_free`, `lra_lock`, `lra_unlock`, `lra_block_offset`, `lra_allocate`
- memory/lrar_cache.c: `lrar_default_lock_proc`, `lrar_default_unlock_proc`, `lrar_unlock_block`, `verify_lrar_cache_block`, `verify_lrar_cache`, `lrar_new`, `lrar_dispose`, `get_lrar_cache_block`, `lrar_flush`, `lrar_allocate`, `lrar_block_address`, `lrar_deallocate`
- memory/lru_cache.c: `lru_default_new_block_proc`, `lru_default_purge_block_proc`, `get_lru_cache_block_offset`, `verify_lru_cache_block`, `verify_lru_cache`, `lru_new`, `lru_dispose`, `lru_flush`, `lru_free_blocks`, `lru_allocate`, `lru_lock`, `lru_unlock`, `lru_touch`
- memory/memory_pool.c: `memory_pool_allocation_size`, `memory_pool_initialize`, `memory_pool_get_free_size`, `memory_pool_get_used_size`, `memory_pool_get_contiguous_free_size`, `memory_pool_block_compute_actual_size`, `memory_pool_block_new`, `memory_pool_verify`, `memory_pool_block_get`, `memory_pool_new`, `memory_pool_delete`, `memory_pool_block_allocate`, `memory_pool_block_free`, `memory_pool_compact`, `memory_pool_block_reallocate`
- memory/stack_memory_pool.c: `stack_memory_pool_reset`, `code_0010e270`, `code_0010e2b0`, `code_0010e2e0`, `code_0010e330`, `code_0010e360`, `code_0010e3e0`, `code_0010e430`, `stack_memory_pool_find_space_between_blocks`, `code_0010e510`, `code_0010e5b0`, `code_0010e5f0`, `code_0010e630`, `code_0010e670`, `code_0010e6a0`, `code_0010e770`, `code_0010e7f0`, `code_0010e890`, `code_0010e960`, `code_0010ea00`, `dispose_handle`, `lock_handle`, `unlock_handle`, `dispose_pointer`, `code_0010eef0`, `code_0010ef10`, `code_0010ef70`, `pool_new_handle`, `pool_new_handle_clear`, `pool_resize_handle`, `pool_new_pointer`, `pool_new_pointer_clear`, `pool_resize_pointer`, `memory_block_set_size_and_index`, `memory_block_get_real_size`, `memory_block_get_user_size`, `memory_block_get_pool_index`, `stack_memory_pool_free_space_at_end_of_pool`, `stack_memory_pool_find_first_unused_memory_block`, `stack_memory_pool_set_next_block_index`, `memory_block_valid`, `memory_block_lock`, `memory_block_unlock`, `memory_block_is_locked`, `memory_block_get_user_address`, `stack_memory_pool_compact`, `stack_memory_pool_valid_block`, `stack_memory_pool_dispose_block`, `stack_memory_pool_lock_block`, `stack_memory_pool_unlock_block`, `stack_memory_pool_allocate`, `stack_memory_pool_new_block`, `stack_memory_pool_new_block_clear`, `stack_memory_pool_resize_block`
- memory/texture_page.c: `texture_page_fraction_used`, `texture_page_verify`, `texture_page_new`, `texture_page_delete`, `texture_page_textures_begin`, `texture_page_texture_get`, `qsort_texture_indexes`, `texture_page_resort`, `texture_page_texture_new`, `texture_page_textures_cancel`, `texture_page_textures_end`, `texture_page_texture_delete`, `texture_page_resize`, `texture_page_verify_and_return`
- memory/zlib/contrib/asm386/gvmat32c.c: `match_init`, `longest_match`
- memory/zlib/contrib/iostream/zfstream.h: `setcompressionlevel`, `setcompressionstrategy`
- memory/zlib/contrib/iostream2/zstream.h: `read`, `zstringlen`, `write`
- memory/zlib/contrib/minizip/zip.c: `allocate_new_datablock`
- memory/zlib/contrib/untgz/untgz.c: `OF`
- memory/zlib/crc32.c: `make_crc_table`, `get_crc_table`, `crc32`
- memory/zlib/trees.c: `send_bits`, `tr_static_init`, `gen_trees_header`
- memory/zlib/zutil.c: `zlibVersion`

## `models`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `animation_get_frame_data` | models/model_animation_definitions.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_loop_frame_index` | models/model_animations.c | Matched | sim/src/animation.rs | owed: the engine's own tests (sim/tests/stock_animations.rs); no check against the original yet |
| `animation_second_key_frame_index` | models/model_animations.c | Matched | sim/src/animation.rs | owed: the engine's own tests (sim/tests/stock_animations.rs); no check against the original yet |
| `animation_sound_frame_index` | models/model_animations.c | Matched | sim/src/animation.rs | owed: the engine's own tests (sim/tests/stock_animations.rs); no check against the original yet |
| `build_damage_animation_index` | models/model_animations.c | Matched | content/src/object.rs:519 | owed: the damage animations; reads the object, tag and map world, which the fake worlds do not hold (the animation graph) |
| `animation_set_frame_size` | models/model_animations.c | Replaced | import/src/object.rs | maps imported to a typed format; the tool's, when it builds an animation: nothing in the game calls it, and the importer reads what it made |
| `quaternion_decompress_8byte` | models/model_animations.c | Ported | sim/src/animation.rs:260 | `tools/engine_animation_reference.py` |
| `quaternion_decompress_6byte` | models/model_animations.c | Ported | sim/src/animation.rs:272 | `tools/engine_animation_reference.py` |
| `quaternion_compress_8byte` | models/model_animations.c | Replaced | import/src/object.rs | maps imported to a typed format; the tool's, when it builds an animation: nothing in the game calls it, and the importer reads what it made |
| `quaternion_compress_6byte` | models/model_animations.c | Replaced | import/src/object.rs | maps imported to a typed format; the tool's, when it builds an animation: nothing in the game calls it, and the importer reads what it made |
| `animation_graph_node_matrices_from_orientations` | models/model_animations.c | Ported | sim/src/animation.rs:1146 | `tools/engine_animation_reference.py` |
| `interpolate_node_orientations` | models/model_animations.c | Ported | sim/src/animation.rs:1122 | `tools/engine_animation_reference.py` |
| `animation_choose_random_permutation_internal` | models/model_animations.c | Matched | sim/src/animation.rs:1227 | owed: draws from the game's random seed; checked through the animation trace's permutation weights |
| `animation_update_internal` | models/model_animations.c | Matched | sim/src/animation.rs:1252 | owed: reads the object, tag and map world, which the fake worlds do not hold (the animation tag's frames); checked by the animation trace's frame counts |
| `animation_get_node_orientations` | models/model_animations.c | Ported | sim/src/animation.rs:694 | `tools/engine_animation_reference.py` |
| `replacement_animation_apply` | models/model_animations.c | Ported | sim/src/animation.rs:726 | `tools/engine_animation_reference.py` |
| `overlay_animation_apply` | models/model_animations.c | Ported | sim/src/animation.rs:739 | `tools/engine_animation_reference.py` |
| `overlay_animation_apply_scaled` | models/model_animations.c | Ported | sim/src/animation.rs:844 | `tools/engine_animation_reference.py` |
| `overlay_animation_apply_continuous` | models/model_animations.c | Ported | sim/src/animation.rs:880 | `tools/engine_animation_reference.py` |
| `overlay_animation_apply_continuous_scaled` | models/model_animations.c | Matched | sim/src/animation.rs:893 | owed: checked through the animation trace's overlays |
| `aiming_screen_apply` | models/model_animations.c | Ported | sim/src/animation.rs:762 | `tools/engine_animation_reference.py` |
| `render_model_parts` | models/models.c | Replaced | app/Sources/OpenCEEngineRender/ModelPass.swift | the enhanced lighting and Metal renderer |
| `model_interpolate_node_orientations` | models/models.c | Matched | sim/src/animation.rs | owed: the engine's own tests (sim/tests/stock_animations.rs); no check against the original yet |
| `model_get_node_orientations` | models/models.c | Ported | sim/src/animation.rs:683 | `tools/engine_animation_reference.py` |
| `model_get_node_matrices` | models/models.c | Matched | sim/src/animation.rs | owed: the engine's own tests (sim/tests/stock_animations.rs); no check against the original yet |
| `model_node_matrices_from_orientations` | models/models.c | Matched | sim/src/animation.rs | owed: the engine's own tests (sim/tests/stock_animations.rs); no check against the original yet |
| `model_find_marker` | models/models.c | Matched | sim/src/seats.rs | owed: the engine's own tests (sim/src/seats.rs); no check against the original yet |
| `model_get_default_inverse_matrix` | models/models.c | Replaced | ffi/src/render_model.rs | the enhanced lighting and Metal renderer |
| `model_geometry_part_build_tangent_matrices` | models/models.c | Replaced | import/src/object.rs | maps imported to a typed format |
| `render_model` | models/models.c | Matched | ffi/src/render_model.rs:67 | owed: the engine cites it, with no check against the original yet |
| `model_get_marker_by_name` | models/models.c | Partial | sim/src/seats.rs | leaves out the marker's region and permutation: every instance of the marker is taken, whichever permutation is drawn; owed: the engine's own tests (sim/src/seats.rs); no check against the original yet |
| `model_build_tangent_matrices` | models/models.c | Replaced | import/src/object.rs | maps imported to a typed format |
| `animation_is_compressed` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_keyframe_search` | models/model_animations.c | Ported | sim/src/animation.rs:522 | `tools/engine_animation_reference.py` |
| `animation_get_keyframe_rotation` | models/model_animations.c | Ported | sim/src/animation.rs:448 | `tools/engine_animation_reference.py` |
| `animation_get_keyframe_translation` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_get_keyframe_scale` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |

16 Missing (9 reached from `main_loop`), by file:

- models/model_animation_definitions.c: `animation_get_frame_info`, `animation_list_get_string`
- models/model_animations.c: `code_0010fe70`, `animation_get_x_offsets`, `quaternion_decompress_6byte_renormalized`, `animation_graph_get_animation_by_name`, `code_00110560`, `animation_frame_get_xy_translation`, `inverse_kinematics_adjust_matrices`, `code_00110b80`, `code_00110e90`, `code_00111190`, `animation_get_root_matrix`, `animation_get_root_velocity`
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
| `code_0011af00` | networking/network_messages.c | Replaced | net/src/lib.rs | networking and Xbox Live |
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
| `code_0011d0d0` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011d240` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_rejection_message` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_reject_connection_game_is_full` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011d3b0` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011d400` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011d470` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_get_client_machine_count` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `dump_network_game_data` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_dump` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_remove_client_machine_from_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_remove_machine_from_game` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_update_countdown` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011ddd0` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011dfa0` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_dispose` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_idle` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_reset_to_pregame` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_create` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011e890` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011e920` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011e9c0` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011ea50` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011eae0` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011eb80` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011ec20` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_message_to_machine` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_message_to_all_machines` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_player_joined_info_ingame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_game_data_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011eee0` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011f120` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011f1e0` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011f5d0` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011f690` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011f750` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011f830` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011f900` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_0011f9d0` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_datagram` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_client_message` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_console_initialize` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_console_dispose` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_console_print` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `code_001203c0` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
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
| `encode_network_game_message` | networking/network_messages.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_ingame_addition_count` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_kick_machine` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_machine_may_add_player_ingame` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_last_refusal_code` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_accepts_late_joins` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_client_machine_is_loaded` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_machine_has_waiting_players` | networking/network_server_manager.c | Replaced | net/src/lib.rs | networking and Xbox Live |
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
| `network_game_server_write` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_machine_is_loading_late` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_message_to_client_machine` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_game_settings_to_client_machine` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_server_machines` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_client_machine_in_game` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_server_send_to_machine` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_server_send_to_machine_reliably` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_distributed_server_send_to_all_reliably` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_send_game_settings_to_all_machines` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_flush_game_data_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_queue_client_player` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_broadcast_game_search` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_ping` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_join_game_request` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `network_game_server_handle_message_client_add_player_request_pregame` | networking/network_server_message_handler.c | Replaced | net/src/lib.rs | networking and Xbox Live |
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
| `telnet_client_write` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `telnet_client_disconnect` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |
| `process_telnet_client_buffer` | networking/telnet_console.c | Replaced | net/src/lib.rs | networking and Xbox Live |

## `objects`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `object_initialize_vitality` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `object_get_maximum_body_vitality` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `object_get_maximum_shield_vitality` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `damage_data_new` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `object_can_take_damage` | objects/damage.c | Matched | campaign/src/objects.rs | owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `object_cannot_take_damage` | objects/damage.c | Matched | campaign/src/objects.rs | owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `object_set_ranged_attack_inhibited` | objects/damage.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `object_set_melee_attack_inhibited` | objects/damage.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `object_deplete_shield` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `object_damage_shield` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `object_damage_aftermath` | objects/damage.c | Matched | sim/src/forces.rs:2 | owed: damage; reads the object, tag and map world, which the fake worlds do not hold; Phase 12 |
| `object_deplete_body` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `object_destroy` | objects/damage.c | Matched | campaign/src/objects.rs | owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `object_damage_body` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `object_cause_damage` | objects/damage.c | Matched | sim/src/damage.rs:179 | owed: damage; reads the object, tag and map world, which the fake worlds do not hold; Phase 12 |
| `object_damage_update` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `area_of_effect_cause_damage_to_object` | objects/damage.c | Matched | sim/src/damage.rs:5 | owed: damage; reads the object, tag and map world, which the fake worlds do not hold; Phase 12's explosions |
| `area_of_effect_cause_damage` | objects/damage.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
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
| `code_0012b870` | objects/object_shadows.c | Replaced | app/Sources/OpenCEEngineRender/ShadowPass.swift | the enhanced lighting and Metal renderer |
| `code_0012b880` | objects/object_shadows.c | Replaced | app/Sources/OpenCEEngineRender/ShadowPass.swift | the enhanced lighting and Metal renderer |
| `object_build_shadow` | objects/object_shadows.c | Replaced | app/Sources/OpenCEEngineRender/ShadowPass.swift | the enhanced lighting and Metal renderer |
| `object_types_disconnect_from_structure_bsp` | objects/object_types.c | Matched | sim/src/objects.rs:456 | owed: placement; reads the object, tag and map world, which the fake worlds do not hold |
| `object_types_place_objects` | objects/object_types.c | Matched | sim/src/objects.rs:313 | owed: placement; reads the object, tag and map world, which the fake worlds do not hold; tools/engine_placement_traces.py records its results |
| `object_types_place_all` | objects/object_types.c | Matched | sim/src/objects.rs:380 | owed: placement; reads the object, tag and map world, which the fake worlds do not hold; tools/engine_placement_traces.py records its results |
| `scenery_place` | objects/scenery.c | Matched | sim/src/scenery.rs | owed: the engine's own tests (sim/src/scenery.rs); no check against the original yet |
| `scenery_new` | objects/scenery.c | Matched | sim/src/animation.rs:1368 | owed: the engine cites it, with no check against the original yet |
| `scenery_update` | objects/scenery.c | Matched | sim/src/scenery.rs | owed: the engine's own tests (sim/src/scenery.rs); no check against the original yet |
| `scenery_animation_start_private` | objects/scenery.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `scenery_animation_start` | objects/scenery.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `scenery_animation_start_at_frame` | objects/scenery.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `find_point_light_for_object` | objects/object_lights.c | Replaced | app/Sources/OpenCEEngineRender/LightPass.swift | the enhanced lighting and Metal renderer |
| `object_shadow_get_object` | objects/object_shadows.c | Replaced | app/Sources/OpenCEEngineRender/ShadowPass.swift | the enhanced lighting and Metal renderer |
| `object_build_shadow_recursive` | objects/object_shadows.c | Replaced | app/Sources/OpenCEEngineRender/ShadowPass.swift | the enhanced lighting and Metal renderer |
| `object_add_scenario_permutation` | objects/objects.c | Matched | sim/src/objects.rs | owed: the engine's own tests (sim/src/objects.rs); no check against the original yet |
| `garbage_collect_now` | objects/objects.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `object_pvs_set_object` | objects/objects.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `object_pvs_set_camera_point` | objects/objects.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `object_pvs_clear` | objects/objects.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `object_beautify` | objects/objects.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `object_scripting_set_collideable` | objects/objects.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `objects_place` | objects/objects.c | Matched | sim/src/objects.rs | owed: the engine's own tests (sim/src/objects.rs); no check against the original yet |
| `object_start_interpolation` | objects/objects.c | Matched | campaign/src/units.rs:55 | owed: interpolation; reads the object, tag and map world, which the fake worlds do not hold; Phase 12's impacts |
| `object_get_function_value` | objects/objects.c | Matched | ffi/src/particles/contrails.rs:56 | owed: the engine cites it, with no check against the original yet |
| `object_index_from_name_index` | objects/objects.c | Matched | campaign/src/objects.rs | owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `objects_disconnect_from_structure_bsp` | objects/objects.c | Matched | sim/src/objects.rs | owed: the engine's own tests (sim/src/objects.rs); no check against the original yet |
| `object_pvs_activate` | objects/objects.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `objects_scripting_set_scale` | objects/objects.c | Partial | campaign/src/objects.rs | leaves out the frames asked for: the scale is taken at once; owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `object_delete` | objects/objects.c | Matched | sim/src/objects.rs | owed: the engine's own tests (sim/src/objects.rs); no check against the original yet |
| `object_detach` | objects/objects.c | Matched | campaign/src/objects.rs | owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `objects_reconnect_to_structure_bsp` | objects/objects.c | Matched | sim/src/objects.rs | owed: the engine's own tests (sim/src/objects.rs); no check against the original yet |
| `object_export_function_values` | objects/objects.c | Matched | sim/src/functions.rs:228 | owed: object functions; reads the object, tag and map world, which the fake worlds do not hold; Phase 13's animations driven by their functions |
| `object_compute_node_matrices` | objects/objects.c | Matched | sim/src/player.rs:788 | owed: the engine cites it, with no check against the original yet |
| `object_choose_random_change_colors` | objects/objects.c | Matched | ffi/src/lib.rs:510 | owed: the engine cites it, with no check against the original yet |
| `objects_scripting_detach` | objects/objects.c | Matched | campaign/src/objects.rs | owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `object_set_position` | objects/objects.c | Matched | campaign/src/objects.rs | owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `object_new` | objects/objects.c | Matched | sim/src/animation.rs:1362 | owed: the engine cites it, with no check against the original yet |
| `object_new_from_scenario` | objects/objects.c | Matched | sim/src/objects.rs:361 | owed: placement; reads the object, tag and map world, which the fake worlds do not hold; tools/engine_placement_traces.py records its results |
| `object_attach_to_marker` | objects/objects.c | Partial | campaign/src/objects.rs | leaves out the parent's markers: a child is attached at the parent's origin; owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `object_new_by_name` | objects/objects.c | Matched | campaign/src/objects.rs | owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `objects_scripting_attach` | objects/objects.c | Partial | campaign/src/objects.rs | leaves out the parent's markers: a child is attached at the parent's origin; owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `object_delete_immediately` | objects/objects.c | Matched | sim/src/objects.rs | owed: the engine's own tests (sim/src/objects.rs); no check against the original yet |
| `object_name_list_lookup` | objects/objects.c | Matched | campaign/src/objects.rs | owed: the engine's own tests (campaign/src/objects.rs); no check against the original yet |
| `object_choose_random_region_permutations` | objects/objects.c | Matched | sim/src/objects.rs | owed: the engine's own tests (sim/src/objects.rs); no check against the original yet |
| `object_delete_recursive` | objects/objects.c | Matched | sim/src/objects.rs | owed: the engine's own tests (sim/src/objects.rs); no check against the original yet |
| `object_compute_function_values` | objects/objects.c | Matched | sim/src/functions.rs:1 | owed: object functions; reads the object, tag and map world, which the fake worlds do not hold; Phase 13's animations driven by their functions |

241 Missing (219 reached from `main_loop`), by file:

- objects/damage.c: `damage_initialize`, `damage_dispose`, `damage_initialize_for_new_map`, `damage_dispose_from_old_map`, `damage_render_debug`, `object_restore_body`, `object_double_charge_shield`, `object_destroy_notify_children`, `get_player_index_from_object_or_parents`, `damage_effect_new_on_object`, `object_permutation_shield_regions`, `object_get_actual_body_vitality`, `object_get_actual_shield_vitality`, `damage_effect_new_at_location`, `render_debug_object_damage`, `object_destroy_region`, `damage_set_network_state`, `damage_replay_player_effect`, `damage_replay_aftermath`, `damage_replay_kill`, `damage_replay_static`, `damage_get_network_state`, `damage_kill_object_for_player`
- objects/object_deleted_procs.c: `object_deleted_procs_call`
- objects/object_types.c: `object_type_definition_get`, `object_type_get_datum_size`, `object_type_get_name`, `object_types_initialize`, `object_types_dispose`, `object_types_initialize_for_new_map`, `object_types_dispose_from_old_map`, `object_type_adjust_placement`, `object_type_new`, `object_type_place`, `object_type_delete`, `object_type_update`, `object_type_export_function_values`, `object_type_handle_deleted_object`, `object_type_handle_region_destroyed`, `object_type_handle_parent_destroyed`, `object_type_preprocess_node_orientations`, `object_type_postprocess_node_matrices`, `object_type_reset`, `object_type_disconnect_from_structure_bsp`, `object_type_render_debug`, `object_type_notify_impulse_sound`, `object_definition_index_to_object_type`, `scenario_get_object_type_scenario_datums`, `scenario_get_object_type_scenario_palette`, `object_names_postprocess`, `object_types_reconnect_to_structure_bsp`, `object_type_synchronize`
- objects/objects.c: `cluster_get_first_noncollideable_object`, `cluster_get_next_noncollideable_object`, `cluster_get_first_collideable_object`, `cluster_get_next_collideable_object`, `object_get_next_cluster`, `object_try_and_get_and_verify_type`, `object_get_and_verify_type`, `object_iterator_new`, `object_iterator_next`, `object_get_ultimate_parent`, `object_set_object_index_for_name_index`, `objects_fix_for_deleted_object`, `object_set_garbage`, `objects_information_get`, `objects_get_activating_cluster_index`, `object_definition_predict`, `object_predict`, `object_header_block_get`, `object_header_new`, `object_header_delete`, `object_header_block_allocate`, `object_marker_begin`, `object_marker_end`, `object_unmarked_function`, `object_mark_function`, `find_objects_from_point_vector`, `objects_dump_memory`, `objects_initialize`, `objects_initialize_for_new_map`, `objects_dispose_from_old_map`, `objects_dispose`, `object_activate`, `object_deactivate`, `object_reset`, `object_placement_data_new`, `object_disconnect_from_map`, `object_get_first_cluster`, `object_get_node_matrices`, `object_get_attachment_marker_name`, `object_has_node`, `object_set_automatic_deactivation`, `object_set_visibility`, `object_get_velocities`, `object_get_location`, `object_offset_interpolation`, `object_permute_region`, `objects_in_clusters_by_indices`, `object_visible_to_any_player`, `object_delete_initial_recursive`, `object_reconnect_to_map`, `object_get_node_matrix`, `object_get_marker_by_name`, `object_align_marker_to_matrix`, `object_get_origin`, `object_get_orientation`, `object_get_world_matrix`, `object_inverse_kinematics`, `objects_in_sphere`, `objects_paparazzi`, `object_postprocess_node_matrices`, `object_render_debug`, `attachments_new`, `object_translate`, `object_attach_to_node`, `object_force_inside_bsp`, `object_update`, `object_compute_node_matrices_recursive`, `active_garbage_limit`, `objects_garbage_collection`, `objects_update`, `objects_memory_compact`, `object_connect_lights`, `object_name_list_allocate`, `object_name_list_free`, `object_name_list_clear`, `object_name_list_new`, `object_claim_scenario_name`, `object_name_list_delete`, `recursive_object_adder`, `sort_dumps`, `object_add_to_dump`, `object_dump_write`, `object_find_region_permutations_available_with_variant`, `object_select_random_region_permutations_by_variant`, `object_determine_variant_number`, `object_add_to_list`, `object_remove_from_list`, `attachments_delete`, `object_create_attachments`, `object_delete_attachments`, `object_compute_change_colors`
- objects/placeholder_definitions.c: `placeholder_initialize`, `placeholder_initialize_for_new_map`, `placeholder_dispose_from_old_map`, `placeholder_dispose`, `placeholder_place`, `placeholder_new`, `placeholder_delete`
- objects/scenery.c: `scenery_initialize`, `scenery_initialize_for_new_map`, `scenery_dispose_from_old_map`, `scenery_dispose`, `scenery_delete`, `scenery_get_animation_time`
- objects/widgets/antenna.c: `antennas_initialize`, `antennas_initialize_for_new_map`, `antennas_dispose_from_old_map`, `antennas_dispose`, `antenna_new`, `antenna_delete`, `antenna_update_attachment`, `antenna_render_proper`, `antenna_update`, `antenna_render`, `antennas_update`, `antenna_record_shape`
- objects/widgets/flags.c: `flag_datum_get_vertex`, `flag_datum_get_cell`, `flags_initialize`, `flags_initialize_for_new_map`, `flags_dispose_from_old_map`, `flags_dispose`, `flag_tesselate_region`, `flag_delete`, `flag_update_attachment`, `flag_set_attachment_shape`, `flag_set_trailing_shape`, `flag_update`, `flag_render_proper`, `flag_new`, `flag_render`, `flags_update`
- objects/widgets/glow.c: `glow_dispose`, `glow_initialize_for_new_map`, `glow_dispose_from_old_map`, `glow_new`, `glow_delete`, `glow_trailing_particle_update_color`, `glow_trailing_particle_update_size`, `glow_trailing_particle_update_velocity`, `glow_trailing_particle_update_position`, `glow_trailing_particle_age`, `glow_normal_particle_update_color`, `glow_normal_particle_update_size`, `glow_particle_new`, `point_from_parametric_line`, `glow_render`, `nonuniform_cubic_spline`, `nonuniform_cubic_spline_vector3d`, `glow_initialize`, `glow_normal_particle_new`, `get_particle_world_position`, `glow_normal_particle_update_position`, `glow_particles_initialize`, `glow_trailing_particle_new`, `glow_update`, `glow_submit`, `glow_time_wrap_down`, `glow_time_wrap_up`
- objects/widgets/light_volumes.c: `light_volumes_initialize`, `light_volumes_dispose`, `light_volumes_initialize_for_new_map`, `light_volumes_dispose_from_old_map`, `light_volume_new`, `light_volume_delete`, `light_volume_interpolate_frames`, `pow1`, `light_volume_render`, `light_volume_submit`
- objects/widgets/lightning.c: `lightnings_initialize`, `lightnings_dispose`, `lightnings_initialize_for_new_map`, `lightnings_dispose_from_old_map`, `lightning_new`, `lightning_delete`, `lightning_render`, `lightning_offset_marker_position`, `lightning_submit`
- objects/widgets/widgets.c: `tag_group_to_widget_type`, `widget_type_definition_get`, `widgets_initialize`, `widgets_initialize_for_new_map`, `widgets_dispose_from_old_map`, `widgets_dispose`, `widgets_new`, `widgets_delete`, `widgets_need_lighting`, `widgets_render`, `widgets_update`

## `physics`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `bsp3d_test_point` | physics/bsp3d.c | Matched | content/src/level.rs:878 | owed: the engine cites it, with no check against the original yet |
| `render_debug_collision_vertex` | physics/collision_bsp.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_edge` | physics/collision_bsp.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_surface` | physics/collision_bsp.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_bsp` | physics/collision_bsp.c | Replaced | dev/src/draw.rs | the editor and tools |
| `collision_debug_render` | physics/collision_debug.c | Replaced | dev/src/draw.rs | the editor and tools |
| `collision_log_compare_overall_usage` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_initialize` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `collision_log_enable` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `code_0013c8d0` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
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
| `collision_test_sphere` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `object_test_point` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_test_point` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `object_test_vector` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_test_vector` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_test_pill` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `object_get_features_in_sphere` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_get_features_in_sphere` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_fix_pill_nudge_collision` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_fix_pill` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_move_point` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_move_pill` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_move_sphere` | physics/collisions.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `pin_fraction` | physics/physics.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `physics_get_features_in_sphere` | physics/physics.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `render_debug_physics` | physics/physics.c | Replaced | dev/src/draw.rs | the editor and tools |
| `physics_instance_new` | physics/physics.c | Partial | physics/src/lib.rs | leaves out the original's rigid-body integrator: Jolt moves the dynamic bodies; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `physics_test_point` | physics/physics.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `physics_test_vector` | physics/physics.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `physics_compute_new` | physics/physics.c | Partial | physics/src/lib.rs | leaves out the original's rigid-body integrator: Jolt moves the dynamic bodies; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `physics_update_new` | physics/physics.c | Partial | physics/src/lib.rs | leaves out the original's rigid-body integrator: Jolt moves the dynamic bodies; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `physics_update` | physics/physics.c | Partial | physics/src/lib.rs | leaves out the original's rigid-body integrator: Jolt moves the dynamic bodies; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `physics_variable_position_update` | physics/physics_variables.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `physics_variable_speed_update` | physics/physics_variables.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `physics_variable_speed_update_seek` | physics/physics_variables.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `physics_variable_update` | physics/physics_variables.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `physics_variable_position_update_seek` | physics/physics_variables.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `physics_variable_update_seek` | physics/physics_variables.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `render_debug_collision_features` | physics/collision_features.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_sphere` | physics/collision_features.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_cylinder` | physics/collision_features.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_collision_prism` | physics/collision_features.c | Replaced | dev/src/draw.rs | the editor and tools |
| `collision_model_test_sphere` | physics/collision_models.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_model_test_point` | physics/collision_models.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_model_test_vector` | physics/collision_models.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `collision_model_test_pill` | physics/collision_models.c | Partial | physics/src/lib.rs | leaves out the original's collision BSP walk: Jolt answers the query on a mesh made from the collision BSP, so what is hit at an edge, and in what order, can differ; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `render_debug_collision_model` | physics/collision_models.c | Replaced | dev/src/draw.rs | the editor and tools |
| `collision_log_store_period` | physics/collision_usage.c | Replaced | dev/src/profile.rs | the editor and tools |
| `get_material_type` | physics/physics.c | Matched | physics/src/ground.rs:3 | owed: the ground's material; reads the object, tag and map world, which the fake worlds do not hold (the structure's collision) |
| `friction_evaluate` | physics/physics.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `physics_compute_biped_collision` | physics/physics.c | Matched | sim/src/seats.rs:344 | owed: physics; Jolt stands for the original's collision; tools/engine_movement_traces.py |
| `physics_compute_vehicle_collision` | physics/physics.c | Partial | physics/src/lib.rs | leaves out the original's rigid-body integrator: Jolt moves the dynamic bodies; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `physics_compute_unit_collisions` | physics/physics.c | Partial | physics/src/lib.rs | leaves out the original's rigid-body integrator: Jolt moves the dynamic bodies; owed: the engine's own tests (physics/tests/world.rs); no check against the original yet |
| `physics_variable_position_get_seek_direction` | physics/physics_variables.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `point_physics_definition_get_mass` | physics/point_physics.c | Matched | sim/src/point_physics.rs | owed: the engine's own tests (sim/src/point_physics.rs); no check against the original yet |
| `point_physics_update` | physics/point_physics.c | Matched | sim/src/point_physics.rs:1 | owed: particles; reads the object, tag and map world, which the fake worlds do not hold (the point physics tag); Phase 12's particles |
| `render_debug_point_physics` | physics/point_physics.c | Replaced | dev/src/draw.rs | the editor and tools |

98 Missing (64 reached from `main_loop`), by file:

- physics/breakable_surfaces.c: `breakable_surface_plane_distance`, `breakable_surface_get`, `breakable_surfaces_initialize`, `breakable_surfaces_dispose`, `breakable_surfaces_initialize_for_new_map`, `breakable_surfaces_dispose_from_old_map`, `breakable_surfaces_enable`, `breakable_surfaces_reset`, `breakable_surface_flags_get`, `breakable_surface_extant`, `breakable_surface_damage`, `breakable_surface_damage_area_of_effect`, `breakable_surface_effect`
- physics/bsp2d.c: `bsp2d_test_point`
- physics/bsp3d.c: `bsp3d_clip_line_to_leaves`, `code_00136900`, `bsp3d_clip_polygon_to_leaves`, `bsp3d_plane_distance_to_point`, `bsp3d_polygon_plane_distance_to_point`, `bsp3d_clip_polygon_to_leaves_recursive`
- physics/collision_bsp.c: `collision_surface_edge_count`, `collision_surface_polygon`, `add_feature`, `collision_edge_length`, `collision_surface_perimeter`, `collision_surface_area`, `collision_surface_project_point2d`, `collision_surface_test_point2d`, `collision_surface_find_closest_point2d`, `collision_surface_test_line2d`, `collision_surface_test_sphere`, `code_00137a90`, `code_00137bc0`, `code_00137c90`, `bsp2d_test_sphere_recursive`, `code_00137fd0`, `code_00138160`, `collision_bsp_test_pill_new`, `bsp3d_test_sphere_recursive`, `code_00138700`, `code_00138a20`, `collision_bsp_test_sphere`, `collision_bsp_test_vector`, `code_00138dc0`, `code_00138ed0`, `collision_bsp_test_pill`, `collision_bsp_valid_surface_index`, `collision_bsp3d_enter_node`, `collision_surface_test_point`, `sphere_test_vector`, `collision_bsp_test_pill_new_recursive`, `collision_leaf_test_vector`, `pill_test_vector`, `collision_bsp_test_vector_recursive`, `collision_surface_test_pill`, `bsp2d_test_pill_recursive`, `bsp3d_test_pill_recursive`
- physics/collision_features.c: `collision_features_new`, `collision_features_from_point`, `collision_features_from_line`, `collision_features_from_polygon`, `collision_features_from_vertex`, `collision_features_from_edge`, `collision_features_from_surface`, `collision_bsp_get_features_in_sphere`, `collision_features_test_point`, `collision_features_test_vector`, `collision_sphere_test_point`, `collision_cylinder_test_point`, `collision_prism_test_point`, `collision_sphere_test_vector`, `collision_cylinder_test_vector`, `collision_prism_test_vector`
- physics/collision_models.c: `collision_model_instance_new`, `collision_model_get_features_in_sphere`
- physics/collisions.c: `collision_model_get_material_type`, `collision_test_vector_exit`, `collision_test_pill_new`, `clip_position_to_plane`, `clip_velocity_to_plane`, `clip_position_to_line`, `clip_velocity_to_line`, `collision_cluster_fog_plane`
- physics/physics.c: `code_00140090`, `code_001404d0`, `code_00140620`, `code_001412a0`, `code_00141710`, `code_00141ba0`, `code_00141d20`, `code_00142690`, `compute_ground_plane`, `rotate_vectors3d_by_angular_velocity`, `physics_update_old`
- physics/physics_variables.c: `code_00143f40`
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
| `IDirect3DVertexBuffer8_Register` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Lock` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `IDirect3DVertexBuffer8_Unlock` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_initialize` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_dispose` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decal_vertices_lock` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decal_vertices_unlock` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_begin` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_decals_draw` | rasterizer/xbox/rasterizer_xbox_decals.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014b7e0` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `code_0014c870` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014c890` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_geometry_initialize` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_geometry_begin` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_dynamic_geometry_end` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `_rasterizer_dynamic_triangles_new` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014ccd0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0014cd00` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `code_00154fd0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `code_0015f720` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `code_0016d980` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `uncompress_int32_to_real_vector3d` | rasterizer/rasterizer_geometry.c | Matched | import/src/level.rs:1803 | owed: the engine cites it, with no check against the original yet |
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
| `rasterizer_lens_flare_submit_for_cluster` | rasterizer/rasterizer_lights.c | Matched | import/src/level.rs:1539 | owed: the engine cites it, with no check against the original yet |
| `rasterizer_lens_flares_submit_occlusion_tests` | rasterizer/rasterizer_lights.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_lens_flares_draw` | rasterizer/rasterizer_lights.c | Matched | ffi/src/camera.rs:3 | owed: the engine cites it, with no check against the original yet |
| `code_00172e80` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00172e90` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_cache_initialize` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_set_shadow_color` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_cache_flush` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_text_cache_dispose` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00172fa0` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00172fc0` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00173070` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_001730d0` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00173450` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00173540` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `rasterizer_debug_compare_primitives` | rasterizer/rasterizer_debug.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `lock_rasterizer_text_data` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `unlock_rasterizer_text_data` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_character` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_character_with_dropshadow` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_hires_glyph` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_hires_character` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `rasterizer_draw_hires_character_with_dropshadow` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `hardware_character_cache_get_bitmap` | rasterizer/rasterizer_text.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `detail_object_build_vertices` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `draw_primitives_vertex_buffer_valid` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `draw_primitives_triangle_buffer_valid` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `draw_primitives_dynamic_triangle_count_valid` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `dynamic_vertex_group_get_d3d_vertex_buffer` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `previous_camera_matrix_for_window` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `vector2d_reciprocal` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `render_frustum_sphere_diameter_in_pixels` | render/render_cameras.c | Matched | ffi/src/render_model.rs:84 | owed: the engine cites it, with no check against the original yet |
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
| `contrail_fade` | render/render_contrails.c | Matched | ffi/src/particles/contrails.rs:481 | owed: the engine cites it, with no check against the original yet |
| `code_00177860` | render/render_contrails.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_00178000` | render/render_contrails.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `code_0017a860` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017a8d0` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017a980` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017a9e0` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017ae60` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017af00` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017afd0` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017b080` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017b180` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017b1e0` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017b4b0` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017b7d0` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `object_get_cached_render_lighting` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017b950` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017bbc0` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_objects` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_object_shadows` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `local_player_is_first_person` | render/render_particles.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `compare_rendered_particles` | render/render_particles.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_particles` | render/render_particles.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_sky` | render/render_sky.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017c760` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017c890` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_prepare_for_window` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017c990` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprites_begin` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprites_end` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `code_0017cce0` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_compute_vertex_fade` | render/render_sprite.c | Ported | ffi/src/sprite.rs:2 | `tools/engine_sprite_reference.py` |
| `build_sprite` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_rotational` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `triangle_strip_iterator_new` | render/triangle_strips.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `triangle_strip_iterator_get_triangle` | render/triangle_strips.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_contrail` | render/render_contrails.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_contrails` | render/render_contrails.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `render_detail_objects` | render/render_detail_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
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
| `process_rendered_objects` | render/render_objects.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_transform_origin_and_direction` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_compute_scale` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_get_group` | render/render_sprite.c | Replaced | app/Sources/OpenCEEngineRender/LevelRenderer.swift | the enhanced lighting and Metal renderer |
| `build_sprite_compute_basis` | render/render_sprite.c | Ported | ffi/src/sprite.rs:1 | `tools/engine_sprite_reference.py` |

## `saved games`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `game_state_save` | saved games/game_state.c | Partial | campaign/src/checkpoint.rs:61 | leaves out the memory image: the saved game holds the inputs, replayed from the mission's start; owed: the engine's own tests (campaign/src/checkpoint.rs); no check against the original yet |
| `game_state_revert` | saved games/game_state.c | Partial | campaign/src/checkpoint.rs:87 | leaves out the memory image: the saved game holds the inputs, replayed from the mission's start; owed: the engine's own tests (campaign/src/checkpoint.rs); no check against the original yet |
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
| `game_state_image_contains` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_reference_valid` |
| `game_state_image_pointer` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_data_valid` |
| `game_state_image_refuse` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_accept` |
| `game_state_image_data_valid` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_accept` |
| `game_state_image_reference_valid` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_memory_pool_valid` |
| `game_state_image_memory_pool_valid` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_accept` |
| `game_state_image_lruv_cache_valid` | saved games/game_state.c | Replaced |  | through its caller `game_state_image_accept` |
| `game_state_image_accept` | saved games/game_state.c | Replaced |  | through its caller `game_state_read_core` |

117 Missing (97 reached from `main_loop`), by file:

- saved games/game_state.c: `dummy`, `code_001af260`, `code_001af270`, `code_001af280`, `game_state_dispose`, `game_state_initialize_for_new_map`, `game_state_dispose_from_old_map`, `game_state_save_to_persistent_storage`, `game_state_test_persistent_storage`, `game_state_save_core`, `game_state_reverted`, `game_state_header_valid`, `game_state_allocation_record`, `code_001af6c0`, `game_state_malloc`, `game_state_data_new`, `game_state_memory_pool_new`, `game_state_try_and_load_from_persistent_storage`, `game_state_load_core`, `game_state_call_before_save_procs`, `game_state_call_before_load_procs`, `game_state_call_after_load_procs`, `game_state_note_event`, `game_state_set_revert_time`, `game_state_allocation_new`, `game_state_data_arrays_new_map`, `game_state_check_data_arrays`
- saved games/player_profile.c: `player_profiles_dispose`, `player_profiles_enumerate_available_to_local_player_index`, `player_profile_delete`, `player_profile_get_from_path`, `player_profile_number_of_available_primary_colors`, `player_profile_get_rgb_color`, `player_profile_get_highest_completed_solo_level`, `player_profile_get_enclosing_directory_path`, `build_default_profile`, `player_profile_read`, `player_profile_write_thread_proc`, `player_profile_new`, `player_profile_get`, `player_profile_get_argb_color`, `player_profile_create_default_profiles_on_disk`, `player_profile_write`, `player_profiles_initialize`, `player_profile_save`, `player_profile_save_last_level_played`, `player_profile_save_level_completed`
- saved games/playlist_profile.c: `playlist_profiles_initialize`, `playlist_profiles_dispose`, `playlist_profile_new`, `playlist_profile_delete`, `playlist_profile_get_from_path`, `playlist_profile_get_display_name`, `playlist_profile_number_of_default_profiles_on_disk`, `playlist_profile_create_default_profiles_on_disk`, `playlist_profile_read`, `playlist_profile_write_thread_proc`, `playlist_profiles_enumerate_available_to_local_player_index`, `playlist_profile_get`, `playlist_profile_write`, `playlist_profile_save`, `playlist_profile_options_from_block`, `playlist_profile_options_to_block`
- saved games/saved_game_files.c: `saved_game_files_dispose`, `saved_game_file_close`, `saved_game_file_get_type`, `saved_game_files_notify_memory_units_changed`, `saved_game_file_get_useable_untitled_profile_name`, `saved_game_files_take_mutex`, `saved_game_files_release_mutex`, `saved_game_perform_file_system_checks`, `saved_game_file_name_unique`, `saved_game_file_remember_player1_last_used_profile_directory`, `saved_game_file_retrieve_player1_last_used_profile_directory`, `saved_game_file_remember_last_used_multiplayer_variant_directory`, `saved_game_file_retrieve_last_used_multiplayer_variant_directory`, `saved_game_file_remember_last_used_multiplayer_map`, `saved_game_file_retrieve_last_used_multiplayer_map`, `saved_game_file_generate_checksum`, `find_and_create_directory_if_necessary`, `enumerate_saved_game_files_start`, `enumerate_saved_game_files_end`, `enumerate_saved_game_file`, `enumerate_mapfile_start`, `enumerate_mapfile_end`, `enumerate_saved_game_file_from_mapfile`, `code_001b3100`, `code_001b31d0`, `code_001b31f0`, `code_001b3200`, `saved_game_files_initialize`, `saved_game_file_find_profile_index_for_directory_path`, `enumerate_default_playlist_profiles`, `enumerate_default_player_profiles`, `code_001b3930`, `code_001b3b20`, `append_entry_to_mapfile`, `code_001b3ee0`, `saved_game_file_get_display_name`, `delete_enumerated_saved_game_file`, `saved_game_file_open`, `synchronize_metadata_display_name_with_profile_name`, `saved_game_file_get_path_to_enclosing_directory`, `enumerate_default_profiles`, `saved_game_files_delete_all_custom_profiles`, `code_001b4b00`, `saved_game_files_enumerate_available_to_local_player_index`, `create_enumerated_saved_game_file`, `enumerate_memory_units_test`, `saved_game_files_take_mapfile_mutex`, `saved_game_files_release_mapfile_mutex`, `set_nth_entry_in_mapfile`, `enumerate_memory_units`, `count_enumerated_profiles_in_mapfile`, `build_saved_game_file_index`, `remove_nth_entry_in_mapfile`, `get_nth_entry_in_mapfile`

## `scenario`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `scenario_get_sky_definition_index` | scenario/scenario.c | Replaced |  | through its caller `scenario_get_sky` |
| `scenario_get_sky` | scenario/scenario.c | Replaced |  | through its caller `render_sky` |
| `scenario_trigger_volume_test_point` | scenario/scenario.c | Matched | content/src/campaign.rs:138 | owed: the engine cites it, with no check against the original yet |
| `scenario_debug_to_file` | scenario/scenario.c | Replaced |  | through its caller `_texture_cache_bitmap_get_hardware_format` |
| `scenario_location_water_depth` | scenario/scenario.c | Matched | content/src/level.rs:741 | owed: water; reads the object, tag and map world, which the fake worlds do not hold (the structure's fog planes) |
| `interpolate_real_rgb_color` | scenario/scenario.c | Replaced |  | through its caller `object_render_state_refresh` |
| `scenario_get_atmospheric_fog` | scenario/scenario.c | Replaced |  | through its caller `render_player_frame` |

50 Missing (42 reached from `main_loop`), by file:

- scenario/multiplayer_scenario_description.c: `multiplayer_scenario_description_get_list`, `map_name_from_multiplayer_scenario_description_item`
- scenario/scenario.c: `scenario_call_disconnect_from_structure_bsp_procs`, `scenario_call_reconnect_to_structure_bsp_procs`, `scenario_initialize`, `scenario_initialize_for_new_map`, `scenario_dispose_from_old_map`, `scenario_frame_update`, `scenario_unload`, `global_scenario_get`, `global_scenario_try_and_get`, `global_structure_bsp_get`, `global_collision_bsp_get`, `global_bsp3d_get`, `scenario_get_game_globals`, `global_structure_bsp_tag_index_get`, `scenario_location_award_bonus`, `default_material_definition_get`, `scenario_material_definition_get`, `scenario_location_deafening`, `scenario_fog_at_point`, `scenario_illumination_at_point`, `scenario_leaf_index_from_point`, `scenario_test_pvs`, `scenario_test_pas`, `scenario_location_potentially_visible_local`, `scenario_location_potentially_visible`, `scenario_object_name_index_from_string`, `scenario_fog_region_get_fog_index`, `scenario_switch_structure_bsp`, `scenario_reload_structure_bsp_if_necessary`, `scenario_get_structure_reference_index_from_tag_index`, `scenario_trigger_volume_test_object`, `global_structure_bsp_index_get`, `scenario_load`, `scenario_location_from_point`, `scenario_location_from_line`, `scenario_ensure_point_within_world`, `scenario_get_fog_region_index`, `scenario_location_underwater`, `scenario_get_sound_environment`
- scenario/wind.c: `wind_dispose_from_old_map`, `wind_variance_get`, `wind_update`, `scenario_get_current_from_weather_palette`, `wind_variance_initialize`, `wind_initialize_for_new_map`, `scenario_get_current`, `scenario_get_wind`, `scenario_get_water_current`

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

3 Missing (1 reached from `main_loop`), by file:

- shell/shell.c: `shell_application_is_paused`, `shell_application_pause`, `shell_running_import_tool`

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
| `game_looping_sound_new` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | owed: the engine's own tests (ffi/src/sound_world.rs); no check against the original yet |
| `game_looping_sound_delete` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | owed: the engine's own tests (ffi/src/sound_world.rs); no check against the original yet |
| `unattached_impulse_sound_new` | sound/game_sound.c | Matched | ffi/src/sound_events.rs | owed: the engine's own tests (ffi/src/sound_events.rs); no check against the original yet |
| `unspatialized_impulse_sound_new` | sound/game_sound.c | Matched | sim/src/player.rs:1275 | owed: the engine cites it, with no check against the original yet |
| `scripted_sound_time` | sound/game_sound.c | Matched | campaign/src/sounds.rs:181 | owed: sound; reads the object, tag and map world, which the fake worlds do not hold (the sound tag); campaign/tests/sounds.rs |
| `scripted_sound_stop` | sound/game_sound.c | Matched | campaign/src/sounds.rs | owed: the engine's own tests (campaign/src/sounds.rs); no check against the original yet |
| `scripted_looping_sound_set_alternate` | sound/game_sound.c | Matched | campaign/src/sounds.rs | owed: the engine's own tests (campaign/src/sounds.rs); no check against the original yet |
| `unattached_looping_sound_start` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | owed: the engine's own tests (ffi/src/sound_world.rs); no check against the original yet |
| `unattached_looping_sound_stop` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | owed: the engine's own tests (ffi/src/sound_world.rs); no check against the original yet |
| `track_object_impulse_sound` | sound/game_sound.c | Matched | ffi/src/sound_events.rs | owed: the engine's own tests (ffi/src/sound_events.rs); no check against the original yet |
| `object_impulse_sound_new` | sound/game_sound.c | Matched | ffi/src/sound_events.rs | owed: the engine's own tests (ffi/src/sound_events.rs); no check against the original yet |
| `scripted_sound_new` | sound/game_sound.c | Matched | campaign/src/sounds.rs | owed: the engine's own tests (campaign/src/sounds.rs); no check against the original yet |
| `scripted_looping_sound_stop` | sound/game_sound.c | Matched | campaign/src/sounds.rs | owed: the engine's own tests (campaign/src/sounds.rs); no check against the original yet |
| `game_sound_update` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | owed: the engine's own tests (ffi/src/sound_world.rs); no check against the original yet |
| `compute_sound_obstruction` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | owed: the engine's own tests (ffi/src/sound_world.rs); no check against the original yet |
| `scripted_looping_sound_start` | sound/game_sound.c | Matched | campaign/src/sounds.rs | owed: the engine's own tests (campaign/src/sounds.rs); no check against the original yet |
| `compress_ima_adpcm_audio_data` | sound/ima_adpcm.c | Replaced | dev/src/control.rs | the editor and tools |
| `decompress_ima_adpcm_audio_data` | sound/ima_adpcm.c | Matched | audio/src/decode.rs | owed: the engine's own tests (audio/src/decode.rs); no check against the original yet |
| `sound_class_get` | sound/sound_classes.c | Matched | audio/src/classes.rs | owed: the engine's own tests (audio/src/classes.rs); no check against the original yet |
| `sound_class_datum_get` | sound/sound_classes.c | Matched | audio/src/classes.rs | owed: the engine's own tests (audio/src/classes.rs); no check against the original yet |
| `debug_sound_classes_enable` | sound/sound_classes.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_sound_classes_set_distances` | sound/sound_classes.c | Replaced | dev/src/control.rs | the editor and tools |
| `debug_sound_classes_set_wet` | sound/sound_classes.c | Replaced | dev/src/control.rs | the editor and tools |
| `sound_classes_update` | sound/sound_classes.c | Matched | audio/src/classes.rs:579 | owed: the sound classes; reads the object, tag and map world, which the fake worlds do not hold (the sound manager); Phase 15 |
| `sound_class_get_gain` | sound/sound_classes.c | Matched | audio/src/classes.rs | owed: the engine's own tests (audio/src/classes.rs); no check against the original yet |
| `sound_class_set_gain` | sound/sound_classes.c | Matched | audio/src/classes.rs:4 | owed: the sound classes; reads the object, tag and map world, which the fake worlds do not hold (the sound manager); Phase 15 |
| `sound_definition_get_maximum_distance` | sound/sound_definitions.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `sound_definition_get_minimum_distance` | sound/sound_definitions.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `sound_definition_find_pitch_range_by_pitch` | sound/sound_definitions.c | Ported | audio/src/choice.rs:80 | `tools/engine_sound_choice_reference.py` |
| `try_to_reset_permutations` | sound/sound_definitions.c | Ported | audio/src/choice.rs:134 | `tools/engine_sound_choice_reference.py` |
| `sound_definition_next_permutation` | sound/sound_definitions.c | Ported | audio/src/choice.rs:163 | `tools/engine_sound_choice_reference.py` |
| `sound_is_active` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `sound_scripted_dialog_is_playing` | sound/sound_manager.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `sound_manager_set_sound_environment` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `sound_stop_all` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `sound_new_impulse` | sound/sound_manager.c | Matched | audio/src/player.rs:1 | owed: sound; reads the object, tag and map world, which the fake worlds do not hold; Phase 15 |
| `sound_refresh_looping` | sound/sound_manager.c | Matched | audio/src/player/looping.rs:1 | owed: sound; reads the object, tag and map world, which the fake worlds do not hold; Phase 15 |
| `sound_render` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `read_sound_preferences` | sound/sound_preferences.c | Partial | ffi/src/settings.rs | leaves out the original's preferences file: the volumes are kept with the app's settings; owed: the engine's own tests (ffi/src/settings.rs); no check against the original yet |
| `write_sound_preferences` | sound/sound_preferences.c | Partial | ffi/src/settings.rs | leaves out the original's preferences file: the volumes are kept with the app's settings; owed: the engine's own tests (ffi/src/settings.rs); no check against the original yet |
| `scripted_looping_sound_stop_internal` | sound/game_sound.c | Matched | campaign/src/sounds.rs | owed: the engine's own tests (campaign/src/sounds.rs); no check against the original yet |
| `location_potentially_audible` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | owed: the engine's own tests (ffi/src/sound_world.rs); no check against the original yet |
| `update_potentially_audible_looping_sound` | sound/game_sound.c | Matched | ffi/src/sound_world.rs | owed: the engine's own tests (ffi/src/sound_world.rs); no check against the original yet |
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
| `sound_scale_value` | sound/sound_manager.c | Ported | audio/src/choice.rs:50 | `tools/engine_sound_choice_reference.py` |
| `sound_definition_is_playable` | sound/sound_manager.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `listener_get` | sound/sound_manager.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `sound_definition_promote` | sound/sound_manager.c | Matched | audio/src/choice.rs | owed: the engine's own tests (audio/src/choice.rs); no check against the original yet |
| `sound_manager_master_gain` | sound/sound_manager.c | Matched | audio/src/player.rs:236 | owed: the sound manager's gain; reads the object, tag and map world, which the fake worlds do not hold; Phase 15 |
| `sound_travel_milliseconds` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `refresh_sound` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `sound_channel_summary_build` | sound/sound_manager.c | Matched | audio/src/player.rs:299 | owed: the mixer's channels; reads the object, tag and map world, which the fake worlds do not hold (the sound manager); Phase 15 |
| `channel_set_properties_hardware` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `track_loop_track_sound` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | owed: the engine's own tests (audio/src/player/looping.rs); no check against the original yet |
| `track_loop_impulse_sound` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | owed: the engine's own tests (audio/src/player/looping.rs); no check against the original yet |
| `looping_sound_find` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | owed: the engine's own tests (audio/src/player/looping.rs); no check against the original yet |
| `looping_sound_new` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | owed: the engine's own tests (audio/src/player/looping.rs); no check against the original yet |
| `limit_pitch` | sound/sound_manager.c | Matched | audio/src/choice.rs:68 | owed: the engine cites it, with no check against the original yet |
| `render_debug_sound` | sound/sound_manager.c | Replaced | dev/src/draw.rs | the editor and tools |
| `render_debug_looping_sound` | sound/sound_manager.c | Replaced | dev/src/draw.rs | the editor and tools |
| `sound_scale_random_value` | sound/sound_manager.c | Ported | audio/src/choice.rs:56 | `tools/engine_sound_choice_reference.py` |
| `sound_calculate_fade` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | owed: the engine's own tests (audio/src/player/looping.rs); no check against the original yet |
| `sound_start_fade` | sound/sound_manager.c | Matched | audio/src/player/looping.rs | owed: the engine's own tests (audio/src/player/looping.rs); no check against the original yet |
| `sound_stop` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `source_distance` | sound/sound_manager.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `source_distance_squared` | sound/sound_manager.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `sound_preempts_sound` | sound/sound_manager.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `sound_find_best_channel` | sound/sound_manager.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `sound_find_channel` | sound/sound_manager.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `sound_find_like_channel` | sound/sound_manager.c | Matched | audio/src/player.rs:299 | owed: the mixer's channels; reads the object, tag and map world, which the fake worlds do not hold (the sound manager); Phase 15 |
| `update_channel_for_impulse_sound` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `update_channel_for_looping_sound` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `process_looping_sounds` | sound/sound_manager.c | Matched | audio/src/player/looping.rs:1 | owed: sound; reads the object, tag and map world, which the fake worlds do not hold; Phase 15 |
| `refresh_sounds` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `update_channels` | sound/sound_manager.c | Matched | audio/src/mixer.rs | owed: the engine's own tests (audio/src/mixer.rs); no check against the original yet |
| `refresh_listener` | sound/sound_manager.c | Matched | audio/src/player.rs | owed: the engine's own tests (audio/src/player.rs); no check against the original yet |
| `detail_sound_random_offset` | sound/sound_manager.c | Matched | audio/src/player/looping.rs:686 | owed: a looping sound's details; draws from the game's random seed; Phase 15 |

93 Missing (37 reached from `main_loop`), by file:

- sound/game_sound.c: `game_sound_initialize`, `game_sound_dispose`, `game_sound_initialize_for_new_map`, `game_sound_clear`, `game_sound_restore`, `scripted_foley_predict`, `scripted_looping_sound_set_scale`, `code_001b7290`, `game_sound_set_mouth_aperture`, `code_001b7630`, `code_001b7720`, `code_001b7790`, `code_001b7800`, `code_001b7860`, `game_sound_dispose_from_old_map`, `obstruction_cache_slot`, `compute_combined_pas`, `looping_sound_definition_is_music`, `scripted_music_stop_all`
- sound/ima_adpcm.c: `byte_swap_bungie_ima_adpcm_header`
- sound/sound_classes.c: `sound_classes_initialize`, `sound_classes_dispose_from_old_map`, `sound_classes_dispose`, `sound_classes_initialize_for_new_map`
- sound/sound_definitions.c: `sound_permutation_get_mouth_aperture`, `sound_permutation_get_real_mouth_aperture`
- sound/sound_manager.c: `sound_valid_for_channel`, `current_platform_definition`, `sound_initialize_for_new_map`, `sound_dispose`, `sound_pause`, `sound_render_time`, `sound_reconnect_to_structure_bsp`, `sound_try_and_get`, `sound_enable`, `code_001bb4c0`, `code_001bb4f0`, `code_001bb510`, `code_001bb570`, `code_001bb5b0`, `code_001bb5f0`, `code_001bb690`, `code_001bb6c0`, `code_001bb710`, `code_001bb730`, `code_001bb820`, `code_001bba20`, `code_001bbaa0`, `code_001bbb40`, `code_001bbc30`, `code_001bbcb0`, `code_001bbcf0`, `code_001bbde0`, `code_001bbe00`, `code_001bbf30`, `code_001bbf80`, `code_001bbfe0`, `code_001bc0a0`, `sound_initialize`, `code_001bc3b0`, `code_001bc3e0`, `code_001bc550`, `code_001bc6d0`, `code_001bc790`, `code_001bc860`, `code_001bc970`, `code_001bca70`, `code_001bcc80`, `code_001bcdb0`, `code_001bce80`, `sound_stop_impulse`, `sound_stop_impulse_by_source_and_definition`, `code_001bd090`, `code_001bd180`, `code_001bd3a0`, `code_001bd540`, `code_001bd720`, `code_001be4b0`, `code_001be6a0`, `code_001be890`, `code_001bebf0`, `sound_idle`, `code_001bee50`, `sound_dispose_from_old_map`, `sound_update_time`, `sound_delete`, `channel_queue_sound`, `sound_set_definition_begin`, `sound_set_definition_end`, `source_audible`, `prioritize_sounds`
- sound/sound_scenery.c: `sound_scenery_new`, `sound_scenery_delete`

## `structures`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `render_debug_leaf_portal` | structures/leaf_map.c | Replaced |  | through its caller `render_debug_leaf_portals` |
| `render_debug_leaf_portals` | structures/leaf_map.c | Replaced |  | through its caller `structure_render_preprocess` |
| `calculate_world_from_cell_index_and_offset` | structures/structure_detail_objects.c | Replaced |  | through its caller `render_debug_detail_objects` |
| `get_local_player_datum` | structures/structure_detail_objects.c | Replaced |  | through its caller `render_debug_detail_objects` |
| `key_compare_cells_lower_bound` | structures/structure_detail_objects.c | Replaced |  | through its caller `get_lower_bound_cell` |
| `key_compare_cells_upper_bound` | structures/structure_detail_objects.c | Replaced |  | through its caller `get_upper_bound_cell` |
| `get_lower_bound_cell` | structures/structure_detail_objects.c | Replaced |  | through its caller `structure_render_detail_objects` |
| `get_upper_bound_cell` | structures/structure_detail_objects.c | Replaced |  | through its caller `structure_render_detail_objects` |
| `structure_render_detail_objects` | structures/structure_detail_objects.c | Replaced |  | through its caller `render_window` |
| `render_debug_detail_objects` | structures/structure_detail_objects.c | Replaced |  | through its caller `render_debug` |
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
| `structure_bsp_get_cluster_encoded_sound_distance` | structures/structure_bsp_definitions.c | Matched | content/src/level.rs:766 | owed: the engine cites it, with no check against the original yet |
| `structure_visibility_dequantize_byte_to_real` | structures/structure_visibility.c | Replaced |  | through its caller `dequantize_byte_to_real_rectangle3d` |

71 Missing (36 reached from `main_loop`), by file:

- structures/cluster_partitions.c: `reference_list_remove`, `reference_list_copy`, `cluster_partition_new`, `cluster_partition_make_valid`, `cluster_partition_make_invalid`, `cluster_partition_delete`, `cluster_partition_get_next_datum`, `cluster_partition_get_first_cluster`, `cluster_partition_get_next_cluster`, `cluster_partition_copy`, `cluster_partition_get_first_reference`, `cluster_partition_reconnect`, `cluster_partition_disconnect`, `cluster_partition_get_first_datum`
- structures/leaf_map.c: `node_stack_push`, `node_stack_pop`, `node_stack_read`, `leaf_map_delete`, `find_like_crossing`, `map_leaf_find_face_on_node`, `leaf_map_close_portal`, `leaf_map_leaf_is_closed`, `leaf_map_family_mark`, `leaf_map_build_portal_from_leaves`, `leaf_map_get_leaf_bounds`, `leaf_map_closure`, `leaf_map_leaf_spans_polygon`, `leaf_face_get_vertex3d`, `render_debug_leaf_faces`, `normalize_three_dee`, `intersect_planes3d`, `leaf_map_build_portals_from_leaf`, `leaf_map_build_leaf_face_for_leaf_on_node`, `leaf_map_build_portals`, `leaf_map_build_leaf_faces_for_leaf`, `leaf_map_build_leaf_faces`, `leaf_map_initialize_from_bsp`
- structures/structure_bsp_definitions.c: `structure_bsp_get_cluster_pvs`, `vertex_type_from_shader_tag`
- structures/structure_detail_objects.c: `structure_detail_objects_initialize`, `structure_detail_objects_dispose_from_old_map`, `structure_detail_objects_dispose`, `structure_detail_objects_flush`, `detail_object_offset`, `dot_product4d`, `structure_detail_objects_initialize_for_new_map`
- structures/structure_lens_flares.c: `compare_temp_markers`, `cluster_index_from_point`, `build_structure_lens_flares`, `structure_lens_flares_place`
- structures/structure_runtime_decals.c: `structure_decals_initialize`, `structure_decals_initialize_for_new_map`, `structure_decals_reconnect_to_structure_bsp`, `structure_decals_disconnect_from_structure_bsp`, `structure_decals_dispose_from_old_map`, `structure_decals_dispose`, `structure_decals_update`
- structures/structure_visibility.c: `debug_pvs`
- structures/structures.c: `structures_initialize`, `structures_initialize_for_new_map`, `structures_dispose_from_old_map`, `structures_dispose`, `structure_cluster_marker_begin`, `structure_cluster_unmarked`, `structure_cluster_mark`, `structure_cluster_marker_end`, `sphere_intersects_cluster_portal`, `code_00188200`, `structure_clusters_in_cone`, `structure_clusters_in_sphere`, `structure_clusters_in_sphere_recursive`

## `tag_files`


49 Missing (35 reached from `main_loop`), by file:

- tag_files/files.c: `file_location_set_volume`, `file_reference_create`, `find_files`, `file_read_into_memory`, `file_printf`, `file_reference_get_info`, `file_reference_copy`, `file_reference_add_directory`, `file_reference_set_name`, `file_reference_get_location`, `file_reference_get_name`, `file_references_equal`, `file_reference_create_from_path`, `directory_create_or_delete_contents`, `datastore_read`, `datastore_write`
- tag_files/files_windows.c: `file_location_is_valid`, `file_compare_last_modification_dates`, `find_files_start`, `file_path_add_name`, `file_path_add_extension`, `file_path_remove_name`, `file_path_split`, `file_location_get_full_path`, `file_read_only`, `code_00189ca0`, `file_create`, `file_delete`, `file_exists`, `file_rename`, `file_open`, `file_close`, `file_get_position`, `file_set_position`, `file_get_eof`, `file_set_eof`, `file_read`, `file_write`, `file_read_from_position`, `file_write_to_position`, `file_get_last_modification_date`, `file_get_size`, `find_files_next`, `file_error`
- tag_files/tag_files.c: `tag_name_strip_path`
- tag_files/tag_groups.c: `tag_empty_data`, `tag_data_get_pointer`, `tag_block_get_element_with_size`
- tag_files/tag_groups.h: `tag_block_get_element_fast`

## `text`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `text_bounds_draw_character` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string_compute_bounds` |
| `draw_string_set_indents` | text/draw_string.c | Replaced |  | through its caller `draw_string_and_hack_in_icons` |
| `draw_string_get_color` | text/draw_string.c | Replaced |  | through its caller `draw_string_and_hack_in_icons` |
| `styled_font_get` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string_compute_bounds` |
| `parse_string_new` | text/draw_string.c | Replaced |  | through its caller `draw_string` |
| `parse_string` | text/draw_string.c | Replaced |  | through its caller `draw_string` |
| `parse_unicode_string` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string` |
| `draw_string_partial` | text/draw_string.c | Replaced |  | through its caller `draw_string` |
| `draw_unicode_string_partial` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string` |
| `draw_string` | text/draw_string.c | Matched | ui/src/draw.rs:208 | owed: the engine cites it, with no check against the original yet |
| `draw_unicode_string` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string_compute_bounds` |
| `draw_unicode_string_compute_bounds` | text/draw_string.c | Replaced |  | through its caller `draw_unicode_string_capital_middle` |
| `font_get_character_by_ascii_code` | text/font_group.c | Replaced |  | through its caller `draw_string` |
| `character_in_pattern` | text/international_strings.c | Replaced |  | through its caller `parse_string` |
| `ustrstr` | text/unicode.c | Replaced |  | through its caller `search_and_replace` |
| `ascii_to_wide` | text/unicode.c | Replaced |  | through its caller `set_textbox_to_build_number` |

96 Missing (26 reached from `main_loop`), by file:

- text/draw_string.c: `draw_string_initialize`, `draw_string_initialize_for_new_map`, `draw_string_dispose_from_old_map`, `draw_string_dispose`, `text_pick_draw_character`, `draw_string_get_string`, `draw_string_set_tab_stops`, `draw_string_set_color`, `draw_string_set_font`, `draw_string_set_format`, `draw_string_set_draw_mode`, `draw_string_set_highlight`, `bitmap_draw_character`, `draw_string_compute_bounds`, `draw_string_pick`, `bitmap_draw_string`
- text/international_strings.c: `set_language_code`, `double_byte_character`, `get_next_character`, `get_previous_character`, `align_to_character`
- text/text_group.c: `string_list_get_string`, `unicode_string_list_get_string`, `fallback_string`
- text/unicode.c: `umemchr`, `umemcpy`, `umemcmp`, `umemmove`, `umemset`, `ustrcpy`, `ustrcat`, `ustrcmp`, `ustrlen`, `ustrnlen`, `ustrchr`, `ustrcoll`, `ustrcspn`, `ustrncat`, `ustrncmp`, `ustrncpy`, `ustrpbrk`, `ustrrchr`, `ustrspn`, `ustrtok`, `ustrxfrm`, `ustrlwr`, `ustrupr`, `ustrnlwr`, `ustrnupr`, `ustrcasecmp`, `ustrncasecmp`, `uisalpha`, `uisupper`, `uislower`, `uisdigit`, `uisxdigit`, `uisspace`, `uispunct`, `uisalnum`, `uisprint`, `uisgraph`, `uiscntrl`, `utoupper`, `utolower`, `ufgetc`, `ufputc`, `uungetc`, `ufgets`, `ufputs`, `ugets`, `uputs`, `ufprintf`, `uprintf`, `usnprintf`, `usprintf`, `uvfprintf`, `uvprintf`, `uvsnprintf`, `uvsprintf`, `ufdopen`, `ufopen`, `ufclose`, `ufreopen`, `uperror`, `upopen`, `uremove`, `utmpnam`, `ustrtol`, `ustrtoul`, `ustrtod`, `uatoi`, `uctime`, `uasctime`, `wide_to_ascii`, `ustrerror`, `ustring_format_takes`

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
| `biped_start_limp_body_physics` | units/bipeds.c | Matched | sim/src/unit_states.rs:436 | owed: death; reads the object, tag and map world, which the fake worlds do not hold; Phase 14's ragdoll |
| `biped_accelerate` | units/bipeds.c | Matched | sim/src/forces.rs:213 | owed: the engine cites it, with no check against the original yet |
| `biped_update` | units/bipeds.c | Matched | sim/src/unit_animator.rs:1 | owed: the engine cites it, with no check against the original yet |
| `unit_scripting_set_maximum_vitality` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `units_scripting_set_maximum_vitality` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_scripting_set_current_vitality` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `units_scripting_set_current_vitality` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_scripting_get_health` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_scripting_get_shield` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_scripting_get_grenade_count` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_scripting_impervious` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_scripting_start_user_animation_list` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_scripting_has_weapon_readied` | units/unit_scripting_commands.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_kill` | units/units.c | Matched | script/src/lib.rs:491 | owed: the engine cites it, with no check against the original yet |
| `unit_get_seat_entrance_point` | units/units.c | Matched | sim/src/seats.rs:63 | owed: seats; reads the object, tag and map world, which the fake worlds do not hold (the seat's marker) |
| `unit_animation_state_interruptable` | units/units.c | Ported | sim/src/unit_states.rs:220 | `tools/engine_unit_state_reference.py` |
| `unit_animation_busy` | units/units.c | Matched | sim/src/unit_states.rs:242 | owed: the engine cites it, with no check against the original yet |
| `unit_animation_overlay_action_loops` | units/units.c | Ported | sim/src/unit_states.rs:292 | `tools/engine_unit_state_reference.py` |
| `unit_animation_state_loops` | units/units.c | Ported | sim/src/unit_states.rs:266 | `tools/engine_unit_state_reference.py` |
| `unit_animation_aiming_screen` | units/units.c | Ported | sim/src/animation_blend.rs:167 | through `unit_animation_state_get_aiming_screen_index`'s trace; checked through unit_animation_state_get_aiming_screen_index's trace |
| `unit_animation_state_get_aiming_screen_index` | units/units.c | Ported | sim/src/unit_states.rs:314 | `tools/engine_unit_state_reference.py` |
| `unit_animation_state_can_be_entered_without_animation` | units/units.c | Ported | sim/src/unit_states.rs:205 | `tools/engine_unit_state_reference.py` |
| `unit_animation_compute_interpolation_frame_count` | units/units.c | Ported | sim/src/unit_states.rs:300 | `tools/engine_unit_state_reference.py` |
| `unit_destroy` | units/units.c | Matched | sim/src/unit_states.rs:433 | owed: the engine cites it, with no check against the original yet |
| `unit_get_camera_position` | units/units.c | Matched | sim/src/seats.rs:71 | owed: seats; reads the object, tag and map world, which the fake worlds do not hold (the seat's camera marker) |
| `unit_animation_impulse_get_index` | units/units.c | Matched | sim/src/ai_impulse.rs:3 | owed: AI's impulses; reads the object, tag and map world, which the fake worlds do not hold (the animation graph) |
| `unit_can_play_animation_impulse` | units/units.c | Matched | sim/src/ai_impulse.rs:3 | owed: AI's impulses; reads the object, tag and map world, which the fake worlds do not hold (the animation graph) |
| `unit_test_animation_impulse` | units/units.c | Matched | sim/src/ai_impulse.rs:131 | owed: AI's impulses; reads the object, tag and map world, which the fake worlds do not hold (the animation graph) |
| `unit_scripting_suspended` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `unit_scripting_doesnt_drop_items` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `unit_scripting_vehicle_test_seat_list` | units/units.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_detach_from_parent` | units/units.c | Matched | sim/src/vehicle_animation.rs:315 | owed: seats; reads the object, tag and map world, which the fake worlds do not hold; Phase 13's getting in and out |
| `unit_throw_grenade_release` | units/units.c | Matched | sim/src/player.rs:993 | owed: grenades; reads the object, tag and map world, which the fake worlds do not hold; Phase 12's thrown grenades |
| `unit_seat_update` | units/units.c | Matched | sim/src/vehicle_animation.rs:5 | owed: seats; reads the object, tag and map world, which the fake worlds do not hold; Phase 13 |
| `unit_animation_update` | units/units.c | Matched | ffi/src/sound_events.rs:71 | owed: the engine cites it, with no check against the original yet |
| `unit_melee_sound` | units/units.c | Matched | sim/src/player.rs:1160 | owed: the engine cites it, with no check against the original yet |
| `unit_set_enterable_by_player` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `unit_set_emotion` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `unit_get_custom_animation_time` | units/units.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_set_or_test_seat_and_weapon_label` | units/units.c | Matched | sim/src/animation.rs:1396 | owed: checked through the animation trace's labels |
| `unit_animation_set_state` | units/units.c | Matched | sim/src/unit_states.rs:3 | owed: checked through the unit state trace's rules |
| `unit_find_nearby_seat` | units/units.c | Matched | sim/src/seats.rs:4 | owed: seats; reads the object, tag and map world, which the fake worlds do not hold; Phase 13's getting in and out |
| `unit_open` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `unit_close` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `units_set_desired_flashlight_state` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `unit_scripting_set_seat` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `unit_cause_melee_damage` | units/units.c | Matched | sim/src/unit_states.rs:430 | owed: damage; reads the object, tag and map world, which the fake worlds do not hold; Phase 12 |
| `unit_stop_custom_animation` | units/units.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_custom_animation_at_frame` | units/units.c | Matched | sim/src/animation.rs:1479 | owed: scripts' custom animations; reads the object, tag and map world, which the fake worlds do not hold |
| `unit_preprocess_node_orientations` | units/units.c | Matched | sim/src/animation_blend.rs:5 | owed: checked through the animation trace's aiming blends |
| `unit_update_animation` | units/units.c | Matched | sim/src/unit_animator.rs:1 | owed: checked through the unit state trace's rules |
| `unit_start_animation_impulse` | units/units.c | Matched | sim/src/ai_impulse.rs:4 | owed: AI's impulses; reads the object, tag and map world, which the fake worlds do not hold (the animation graph) |
| `unit_solo_player_integrated_night_vision_is_active` | units/units.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_enter_seat` | units/units.c | Matched | sim/src/unit_states.rs:571 | owed: seats; reads the object, tag and map world, which the fake worlds do not hold; Phase 13's getting in and out |
| `unit_exit_seat_end` | units/units.c | Matched | sim/src/lib.rs:560 | owed: seats; reads the object, tag and map world, which the fake worlds do not hold; Phase 13's getting in and out |
| `unit_scripting_enter_vehicle` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `vehicle_scripting_load_magic` | units/units.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_try_and_exit_seat` | units/units.c | Matched | sim/src/unit_states.rs:603 | owed: the engine cites it, with no check against the original yet |
| `vehicle_scripting_unload` | units/units.c | Matched | campaign/src/units.rs | owed: the engine's own tests (campaign/src/units.rs); no check against the original yet |
| `unit_scripting_exit_vehicle` | units/units.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `vehicle_causes_collision_damage` | units/vehicles.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `vehicle_hover` | units/vehicles.c | Partial | campaign/src/presented.rs | leaves out what it does: the engine keeps it only as an event, which nothing acts on yet; owed: the engine's own tests (campaign/src/lib.rs); no check against the original yet |
| `vehicle_is_flipped` | units/vehicles.c | Matched | sim/src/vehicle.rs:68 | owed: vehicles; reads the object, tag and map world, which the fake worlds do not hold (the vehicle's physics); Phase 13 |
| `vehicle_new` | units/vehicles.c | Matched | sim/src/objects.rs:344 | owed: the engine cites it, with no check against the original yet |
| `vehicle_preprocess_node_orientations` | units/vehicles.c | Matched | sim/src/vehicle_animation.rs:3 | owed: vehicles; reads the object, tag and map world, which the fake worlds do not hold (the vehicle's model); Phase 13's wheels and tracks |
| `vehicle_accelerate` | units/vehicles.c | Matched | sim/src/forces.rs:212 | owed: the engine cites it, with no check against the original yet |
| `vehicle_export_function_values` | units/vehicles.c | Ported | sim/src/functions.rs:222 | `tools/engine_vehicle_function_reference.py` |
| `vehicle_update` | units/vehicles.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `biped_falling_damage` | units/bipeds.c | Matched | sim/src/damage.rs | owed: the engine's own tests (sim/src/damage.rs); no check against the original yet |
| `biped_start_landing` | units/bipeds.c | Matched | physics/src/movement.rs | owed: the engine's own tests (physics/src/movement.rs); no check against the original yet |
| `biped_jump` | units/bipeds.c | Matched | physics/src/movement.rs | owed: the engine's own tests (physics/src/movement.rs); no check against the original yet |
| `biped_make_footstep` | units/bipeds.c | Matched | ffi/src/sound_events.rs:97 | owed: the engine cites it, with no check against the original yet |
| `biped_update_airborne` | units/bipeds.c | Partial | physics/src/movement.rs | leaves out the original's collision: the pill is moved by the physics library's character controller; owed: the engine's own tests (physics/src/movement.rs); no check against the original yet |
| `biped_update_landing` | units/bipeds.c | Matched | sim/src/lib.rs:1700 | owed: the engine cites it, with no check against the original yet |
| `biped_update_jumping` | units/bipeds.c | Matched | physics/src/movement.rs | owed: the engine's own tests (physics/src/movement.rs); no check against the original yet |
| `biped_update_physics` | units/bipeds.c | Partial | physics/src/movement.rs | leaves out the original's collision: the pill is moved by the physics library's character controller; owed: the engine's own tests (physics/src/movement.rs); no check against the original yet |
| `biped_update_turning` | units/bipeds.c | Matched | sim/src/unit_states.rs:769 | owed: a biped's turning; reads the object, tag and map world, which the fake worlds do not hold; Phase 11's divergences |
| `biped_update_moving` | units/bipeds.c | Partial | physics/src/movement.rs | leaves out the original's collision: the pill is moved by the physics library's character controller; owed: the engine's own tests (physics/src/movement.rs); no check against the original yet |
| `compute_airborne_ticks` | units/vehicles.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `slowly_stop_vehicle` | units/vehicles.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `create_ghost_effect` | units/vehicles.c | Matched | sim/src/seats.rs:1266 | owed: a vehicle's effects; reads the object, tag and map world, which the fake worlds do not hold; Phase 13's effects on the move |
| `create_pelican_effect` | units/vehicles.c | Matched | sim/src/seats.rs:1267 | owed: a vehicle's effects; reads the object, tag and map world, which the fake worlds do not hold; Phase 13's effects on the move |
| `update_suspension` | units/vehicles.c | Matched | sim/src/vehicle_animation.rs:4 | owed: the engine cites it, with no check against the original yet |
| `create_slipping_effects` | units/vehicles.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `update_human_tank_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `update_human_boat_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `update_human_jeep_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `update_alien_scout_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |
| `update_alien_fighter_physics` | units/vehicles.c | Matched | sim/src/vehicle.rs | owed: the engine's own tests (sim/src/vehicle.rs); no check against the original yet |

278 Missing (212 reached from `main_loop`), by file:

- units/biped_limp_noodle.c: `biped_limp_noodle_get_max_relaxation_iterations`, `biped_limp_noodle_valid_joint_rotation`, `biped_limp_noodle_move_relax_and_constrain_positions`, `validate_real_vector3d_axes3`, `biped_limp_noodle_adjust_orientations`, `biped_limp_noodle_relax_nodes_onto_environment`
- units/bipeds.c: `animation_left_foot_frame_index`, `animation_right_foot_frame_index`, `bipeds_initialize`, `bipeds_initialize_for_new_map`, `bipeds_dispose_from_old_map`, `bipeds_dispose`, `biped_place`, `biped_delete`, `biped_reset`, `biped_disconnect_from_structure_bsp`, `biped_get_physics_pill`, `biped_stop_melee_attack`, `biped_stop_limp_body_physics`, `code_001902b0`, `code_001903a0`, `code_00190450`, `biped_flying_through_air`, `code_00190670`, `code_00190780`, `biped_adjust_placement`, `biped_export_function_values`, `biped_get_sight_position`, `biped_get_autoaim_pill`, `biped_fix_position`, `biped_render_debug`, `biped_find_ground_surface`, `biped_approximate_surface_index`, `biped_find_pathfinding_surface_index`, `biped_exit_seat_end`, `code_001916e0`, `code_00191820`, `code_001919d0`, `code_00191b00`, `code_00191cb0`, `biped_find_nearby_support_surface`, `biped_verify_object_vectors`, `code_00192170`, `code_001922d0`, `code_00192380`, `code_00192400`, `biped_build_flying_axes`, `code_001927b0`, `biped_snap_facing`, `biped_new`, `biped_preprocess_node_orientations`, `code_001944c0`, `code_00194b70`, `code_00195b60`, `biped_bumped_object`, `biped_apply_rotation`, `biped_falling_danger`, `biped_vehicle_speech`, `biped_update_dead`, `biped_try_to_make_footsteps`, `biped_update_slipping`, `biped_check_discard`
- units/dialogue_definitions.c: `dialogue_get_vocalization_name`, `dialogue_get_vocalization_type_by_name`
- units/unit_definitions.c: `unit_definition_get_active_hud_index`, `unit_definition_get_seat_active_hud_index`
- units/unit_dialogue.c: `unit_test_speech`, `unit_is_speaking`, `unit_dialogue_determine_variant`, `unit_get_speech_priority_name`, `unit_get_speech_priority_by_name`, `unit_describe_speech`, `code_00196700`, `unit_speak`, `unit_notify_impulse_sound`, `unit_make_damage_sound`, `unit_scream`, `code_00196f30`, `code_00197010`, `unit_dialogue_update`, `unit_lose_speech`, `unit_find_dialogue_variant`, `unit_dialogue_setup`
- units/unit_scripting_commands.c: `unit_scripting_has_weapon`
- units/units.c: `units_initialize`, `units_initialize_for_new_map`, `units_dispose_from_old_map`, `units_dispose`, `unit_kill_silent`, `unit_kill_no_statistics`, `unit_delete`, `units_update`, `unit_export_function_values`, `unit_persistent_control`, `unit_get_melee_range_and_ticks`, `unit_get_animation_frames_remaining`, `unit_euler_axis_doplan`, `unit_get_zoom_level`, `unit_animation_weapon_ik`, `unit_animation_vehicle_ik`, `unit_animation_start_action`, `unit_animation_start_overlay_action`, `unit_can_enter_seat`, `unit_impulse`, `unit_handle_weapon_state_change`, `unit_record_damage`, `unit_test_spawning`, `unit_get_head_position`, `unit_estimate_position`, `unit_get_center_of_mass`, `unit_get_aiming_unit_index`, `unit_get_aiming_vector`, `unit_get_looking_vector`, `unit_get_facing_vector`, `unit_inventory_next_grenade`, `unit_controllable`, `unit_set_controllable`, `unit_set_possessed`, `unit_is_busy`, `unit_scripting_set_emotion_animation`, `unit_scripting_can_blink`, `unit_scripting_vehicle_test_seat`, `unit_scripting_unit_riders`, `unit_scripting_unit_driver`, `unit_scripting_unit_gunner`, `unit_shield_sapping_update`, `units_debug_get_next_unit`, `units_debug_get_closest_unit`, `unit_debug_ninja_rope`, `unit_set_user_animation`, `any_unit_is_dangerous`, `unit_can_see_point`, `unit_driven_by_ai`, `unit_gunned_by_ai`, `unit_set_desired_flashlight_state`, `unit_get_current_flashlight_state`, `unit_seat_filled`, `unit_seat_is_driver`, `unit_seat_is_gunner`, `unit_seat_allow_noncombatants`, `unit_update_driver_and_gunner`, `unit_get_current_equipment`, `unit_add_grenade_to_inventory`, `unit_add_grenade_type_to_inventory`, `unit_add_equipment_to_inventory`, `unit_delete_current_equipment`, `unit_delete_all_weapons`, `unit_has_weapon_definition_index`, `unit_first_free_weapon_index`, `unit_get_weapon_count`, `unit_approve_weapon_pickup`, `unit_get_grenade_count`, `unit_get_current_grenade_type`, `unit_throw_grenade_move_to_hand`, `base_seat_label_get`, `seat_label_to_base_seat_index`, `base_weapon_label_get`, `unit_set_animation`, `unit_refresh_illumination`, `unit_get_weapon`, `unit_drop_item`, `unit_drop_grenades`, `unit_drop_inventory_weapons`, `unit_handle_region_destroyed`, `unit_cause_continuous_melee_damage`, `unit_aim_without_turning`, `unit_is_playing_custom_animation`, `unit_start_user_animation`, `unit_set_mouth_aperture`, `unit_inventory_get_must_be_readied_weapon`, `unit_has_weapon`, `unit_has_weapon_with_flag`, `unit_start_running_blindly`, `unit_stop_running_blindly`, `unit_flame_to_death`, `unit_flying_through_air`, `unit_euler_axis_buildplan`, `unit_euler_axis_couple`, `unit_adjust_projectile_ray`, `unit_render_debug`, `unit_abort_animation`, `unit_clip_to_aiming_bounds`, `unit_inventory_get_weapon`, `unit_set_actively_controlled`, `vehicle_scripting_find_available_seats`, `unit_overcharged`, `unit_set_seat`, `unit_get_seat_label`, `unit_drop_current_equipment`, `unit_can_use_weapon`, `unit_approve_weapon_swap`, `unit_weapon_next_index`, `unit_drop_current_weapon`, `unit_get_current_weapon_label`, `scripting_set_magic_base_seat`, `unit_handle_deleted_object`, `unit_cause_player_melee_damage`, `unit_align_facing`, `unit_start_flaming_to_death`, `unit_running_blind`, `unit_vectors_are_valid`, `unit_verify_vectors`, `unit_control`, `unit_postprocess_node_matrices`, `unit_euler_aiming_update`, `unit_has_animation_to_enter_seat`, `unit_unzoom`, `unit_get_zoom_magnification`, `unit_integrated_night_vision_is_active`, `unit_ping_animation`, `unit_inventory_next_weapon`, `unit_melee_attack_begin`, `unit_leap_begin`, `unit_unsuspecting`, `unit_add_weapon_to_inventory`, `unit_ready_desired_weapon`, `unit_throw_grenade_begin`, `scripting_magic_melee_attack`, `unit_impact_melee_damage`, `unit_place`, `unit_add_initial_weapons`, `unit_adjust_for_seat_change`, `unit_new`, `unit_died`, `unit_update`, `unit_damage_aftermath`, `unit_animation_enhanced`, `unarmed_melee_weapon_definition_index`, `unit_grenade_throw_ease`, `unit_weapon_class_base_animation`, `unit_animation_pose_at_phase`, `unit_node_orientations_blend`, `unit_grenade_throw_keep_legs_moving`, `unit_network_add_weapon`, `unit_network_drop_weapon`, `unit_network_forget_weapon`
- units/vehicles.c: `vehicles_initialize`, `vehicles_initialize_for_new_map`, `vehicles_dispose_from_old_map`, `vehicles_dispose`, `vehicle_place`, `vehicle_delete`, `vehicle_build_update`, `vehicle_parse_update`, `code_001a4f50`, `code_001a4fa0`, `set_real_quaternion`, `vehicle_reset`, `vehicle_render_debug`, `vehicle_find_pathfinding_surface_index`, `code_001a5810`, `code_001a58e0`, `code_001a5a30`, `code_001a5b40`, `code_001a5e50`, `code_001a6290`, `code_001a6590`, `code_001a6710`, `code_001a6910`, `code_001a6ba0`, `code_001a6dc0`, `code_001a70e0`, `vehicle_moving_near_any_player`, `vehicle_stuck`, `code_001a7ac0`, `code_001a7e60`, `code_001a8800`, `update_alien_fighter_physics_new`, `update_alien_fighter_physics_old`, `compute_acceleration`, `vehicle_dot_product3d_test`, `update_human_plane_physics`, `create_crashing_effects`, `update_turret_physics`, `vehicle_cross_product3d_target`, `vehicle_dot_product3d_target`, `vehicle_triple_product3d_target`, `vehicle_mass_points_fit`
