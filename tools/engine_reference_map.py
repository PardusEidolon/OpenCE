"""The reference map: each function of the original that the engine ports,
and the trace that checks it or the reason it has none.

    python3 tools/engine_reference_map.py          # writes the map into docs/engine.md
    python3 tools/engine_reference_map.py --check  # fails if the map or a trace is stale

The engine names the function of source/ it ports in backticks in its doc
comments (`unit_find_nearby_seat`). Those of the ported systems' crates
(AI, particles, animation, sound, scripts and gameplay: sim, physics, audio,
script, campaign) that source/ defines are the ported functions. A function
is traced when a tools/engine_*_reference.py script compiles it into its
trace; the rest are listed in REASONS with why they have none. --check
fails when a ported function is neither, when a reason is left for one that
is traced or no longer ported, when the table in docs/engine.md is not the
one written here, and when any trace is stale (each script's own --check).
"""

import argparse
import importlib.util
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOCS = ROOT / "docs" / "engine.md"
CRATES = ["sim", "physics", "audio", "script", "campaign"]
BEGIN = "<!-- reference map: tools/engine_reference_map.py writes this table -->"
END = "<!-- end of the reference map -->"

# The systems, by the directory of source/ a function is in.
SYSTEMS = {
    "ai": "AI",
    "effects": "particles",
    "models": "animation",
    "first_person": "animation",
    "sound": "sound",
    "hs": "scripts",
}

# Why a ported function has no trace of its own.
WORLD = "reads the object, tag and map world, which the fake worlds do not hold"
REASONS = {
    "accelerate_to_position": "a first-person weapon's sway; checked through first_person_weapon_update's trace",
    "actor_action_handle_surprise": "AI; " + WORLD + " (the actor and its prop); Phase 11's divergences",
    "actor_action_try_to_dive": "AI; " + WORLD + " (the actor and its prop); Phase 11's divergences",
    "ai_conversation_line": "AI; " + WORLD + " (the scenario's conversations); campaign/tests/flow.rs",
    "ai_conversation_status": "AI; " + WORLD + " (the scenario's conversations); campaign/tests/flow.rs",
    "animation_choose_random_permutation_internal": "draws from the game's random seed; checked through the animation trace's permutation weights",
    "animation_update_internal": WORLD + " (the animation tag's frames); checked by the animation trace's frame counts",
    "area_of_effect_cause_damage_to_object": "damage; " + WORLD + "; Phase 12's explosions",
    "biped_start_limp_body_physics": "death; " + WORLD + "; Phase 14's ragdoll",
    "biped_update_turning": "a biped's turning; " + WORLD + "; Phase 11's divergences",
    "blip_size_get": "the motion sensor's blips; checked by sim/tests/hud.rs",
    "build_damage_animation_index": "the damage animations; " + WORLD + " (the animation graph)",
    "camera_track_splut": "cutscene cameras; " + WORLD + " (the camera track tag); campaign/tests/cutscenes.rs",
    "create_ghost_effect": "a vehicle's effects; " + WORLD + "; Phase 13's effects on the move",
    "create_pelican_effect": "a vehicle's effects; " + WORLD + "; Phase 13's effects on the move",
    "detail_sound_random_offset": "a looping sound's details; draws from the game's random seed; Phase 15",
    "device_export_function_values": "devices; " + WORLD,
    "device_preprocess_node_orientations": "devices; " + WORLD + " (the device's model)",
    "director_desired_perspective": "the camera; " + WORLD + " (the unit's seat); Phase 14's death camera",
    "effect_new_unattached_from_markers": "particles; " + WORLD + " (the effect tag); Phase 12's explosions",
    "equipment_handle_pickup": "items; " + WORLD,
    "first_person_weapon_animation_fits": WORLD + " (the first-person model's nodes); sim/tests/first_person.rs",
    "first_person_weapon_build_node_matrices": WORLD + " (the first-person model's nodes); sim/tests/first_person.rs",
    "first_person_weapon_update": WORLD + " (the weapon tag); its states are first_person_reference's",
    "following_camera_update": "the camera; " + WORLD + "; Phase 14's death camera",
    "game_difficulty_get_team_value": "the difficulty table; " + WORLD + " (the globals tag)",
    "get_material_type": "the ground's material; " + WORLD + " (the structure's collision)",
    "hs_thread_main": "scripts; " + WORLD + " (the scenario's compiled scripts); script/tests",
    "hud_show_action_response": "the HUD's messages; " + WORLD,
    "matrix4x3_rotation_from_angles": "rotations; checked through the item trace's object matrices",
    "model_build_remapping_table_for_animation_graph": WORLD + " (the model's and graph's node names)",
    "motion_sensor_update": "the motion sensor; " + WORLD + "; sim/tests/hud.rs",
    "object_cause_damage": "damage; " + WORLD + "; Phase 12",
    "object_compute_function_values": "object functions; "
    + WORLD
    + "; Phase 13's animations driven by their functions",
    "object_damage_aftermath": "damage; " + WORLD + "; Phase 12",
    "object_export_function_values": "object functions; " + WORLD + "; Phase 13's animations driven by their functions",
    "object_new_from_scenario": "placement; " + WORLD + "; tools/engine_placement_traces.py records its results",
    "object_start_interpolation": "interpolation; " + WORLD + "; Phase 12's impacts",
    "object_types_disconnect_from_structure_bsp": "placement; " + WORLD,
    "object_types_place_all": "placement; " + WORLD + "; tools/engine_placement_traces.py records its results",
    "object_types_place_objects": "placement; " + WORLD + "; tools/engine_placement_traces.py records its results",
    "overlay_animation_apply_continuous_scaled": "checked through the animation trace's overlays",
    "physics_compute_biped_collision": "physics; Jolt stands for the original's collision; tools/engine_movement_traces.py",
    "player_examine_nearby_objects": "the action button; " + WORLD,
    "players_update_after_game": "the end of a level; " + WORLD + "; campaign/tests/flow.rs",
    "point_physics_update": "particles; " + WORLD + " (the point physics tag); Phase 12's particles",
    "process_looping_sounds": "sound; " + WORLD + "; Phase 15",
    "scenario_location_water_depth": "water; " + WORLD + " (the structure's fog planes)",
    "scripted_camera_update": "cutscene cameras; " + WORLD + "; campaign/tests/cutscenes.rs",
    "scripted_sound_time": "sound; " + WORLD + " (the sound tag); campaign/tests/sounds.rs",
    "sound_channel_summary_build": "the mixer's channels; " + WORLD + " (the sound manager); Phase 15",
    "sound_class_set_gain": "the sound classes; " + WORLD + " (the sound manager); Phase 15",
    "sound_classes_update": "the sound classes; " + WORLD + " (the sound manager); Phase 15",
    "sound_find_like_channel": "the mixer's channels; " + WORLD + " (the sound manager); Phase 15",
    "sound_manager_master_gain": "the sound manager's gain; " + WORLD + "; Phase 15",
    "sound_new_impulse": "sound; " + WORLD + "; Phase 15",
    "sound_refresh_looping": "sound; " + WORLD + "; Phase 15",
    "unit_animation_aiming_screen": "checked through unit_animation_state_get_aiming_screen_index's trace",
    "unit_animation_get_fitting": WORLD + " (the unit's model and graph)",
    "unit_animation_impulse_get_index": "AI's impulses; " + WORLD + " (the animation graph)",
    "unit_animation_set_state": "checked through the unit state trace's rules",
    "unit_can_play_animation_impulse": "AI's impulses; " + WORLD + " (the animation graph)",
    "unit_cause_melee_damage": "damage; " + WORLD + "; Phase 12",
    "unit_custom_animation_at_frame": "scripts' custom animations; " + WORLD,
    "unit_detach_from_parent": "seats; " + WORLD + "; Phase 13's getting in and out",
    "unit_enter_seat": "seats; " + WORLD + "; Phase 13's getting in and out",
    "unit_exit_seat_end": "seats; " + WORLD + "; Phase 13's getting in and out",
    "unit_find_nearby_seat": "seats; " + WORLD + "; Phase 13's getting in and out",
    "unit_get_camera_position": "seats; " + WORLD + " (the seat's camera marker)",
    "unit_get_seat_entrance_point": "seats; " + WORLD + " (the seat's marker)",
    "unit_preprocess_node_orientations": "checked through the animation trace's aiming blends",
    "unit_seat_update": "seats; " + WORLD + "; Phase 13",
    "unit_set_or_test_seat_and_weapon_label": "checked through the animation trace's labels",
    "unit_start_animation_impulse": "AI's impulses; " + WORLD + " (the animation graph)",
    "unit_test_animation_impulse": "AI's impulses; " + WORLD + " (the animation graph)",
    "unit_throw_grenade_release": "grenades; " + WORLD + "; Phase 12's thrown grenades",
    "unit_update_animation": "checked through the unit state trace's rules",
    "vehicle_is_flipped": "vehicles; " + WORLD + " (the vehicle's physics); Phase 13",
    "vehicle_preprocess_node_orientations": "vehicles; "
    + WORLD
    + " (the vehicle's model); Phase 13's wheels and tracks",
    "weapon_effect_new": "weapons; " + WORLD + "; Phase 12's effects",
    "weapon_magazine_finish_reload": "weapons; " + WORLD + " (the weapon tag)",
    "weapon_magazine_start_reload": "weapons; " + WORLD + " (the weapon tag)",
    "weapon_trigger_begin_firing": "weapons; " + WORLD + " (the weapon tag)",
    "weapon_trigger_can_fire_again": "weapons; " + WORLD + " (the weapon tag)",
    "weapon_trigger_fire": "weapons; " + WORLD + " (the weapon tag); Phase 12",
    "weapon_trigger_recover": "weapons; " + WORLD + " (the weapon tag)",
    "weapon_trigger_release_charge": "weapons; " + WORLD + " (the weapon tag)",
}

DEFINITION = re.compile(r"^(?:static |__inline )?[\w *]+?\n?\b(\w+)\(\s*[^;{]*\)\s*\{", re.M)
CITED = re.compile(r"`([a-z][a-z0-9]*(?:_[a-z0-9]+){2,})`")


def defined():
    """Each function source/ defines, and the file it is in (the first, for one defined twice)."""
    files = {}
    for path in sorted((ROOT / "source").rglob("*.c")):
        text = path.read_text(encoding="latin-1")
        for name in DEFINITION.findall(text):
            files.setdefault(name, path.relative_to(ROOT).as_posix())
    return files


def ported(files):
    """Each function of source/ the ported crates name, and the first place that names it."""
    places = {}
    for crate in CRATES:
        for path in sorted((ROOT / "engine" / "crates" / crate).rglob("*.rs")):
            for number, line in enumerate(path.read_text().splitlines(), 1):
                for name in CITED.findall(line):
                    if name in files:
                        places.setdefault(name, f"{path.relative_to(ROOT / 'engine' / 'crates').as_posix()}:{number}")
    return places


def scripts():
    """The reference scripts, by their name."""
    return {path.stem: path for path in sorted((ROOT / "tools").glob("engine_*_reference.py"))}


def load(path):
    """A reference script as a module."""
    spec = importlib.util.spec_from_file_location(path.stem, path)
    if spec is None or spec.loader is None:
        raise SystemExit(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def traced():
    """Each function a reference script compiles into its trace, and the script."""
    traces = {}
    for name, path in scripts().items():
        for function in DEFINITION.findall(load(path).source_file()):
            traces.setdefault(function, name)
    return traces


def system(source_file):
    """The system a function of `source_file` belongs to."""
    return SYSTEMS.get(source_file.split("/")[1], "gameplay")


def table(files, places, traces):
    """The map, as the table in docs/engine.md."""
    rows = [
        BEGIN,
        "| Function | System | Source | Ported in | Trace, or why none |",
        "| --- | --- | --- | --- | --- |",
    ]
    for name in sorted(places, key=lambda name: (system(files[name]), name)):
        check = f"`tools/{traces[name]}.py`" if name in traces else REASONS.get(name, "(none)")
        rows.append(f"| `{name}` | {system(files[name])} | {files[name]} | {places[name]} | {check} |")
    rows.append(END)
    return "\n".join(rows)


def problems(files, places, traces):
    """What is wrong with the map: ported functions with neither a trace nor a reason, and stale reasons."""
    found = []
    for name in sorted(places):
        if name not in traces and name not in REASONS:
            found.append(f"{name} ({places[name]}) has no trace and no reason")
    for name in sorted(REASONS):
        if name in traces:
            found.append(f"{name} is traced by {traces[name]}: its reason is stale")
        elif name not in places:
            found.append(f"{name} is no longer ported: its reason is stale")
    return found


def stale_traces():
    """The reference scripts whose trace is not the one their code writes."""
    stale = []
    for name, path in scripts().items():
        result = subprocess.run([sys.executable, str(path), "--check"], capture_output=True, text=True)
        if result.returncode:
            stale.append(f"{name}: {(result.stderr or result.stdout).strip()}")
    return stale


def written(docs, map_table):
    """docs/engine.md with the map in it."""
    if BEGIN not in docs or END not in docs:
        raise SystemExit(f"docs/engine.md has no reference map ({BEGIN})")
    start = docs.index(BEGIN)
    return docs[:start] + map_table + docs[docs.index(END) + len(END) :]


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the map or a trace is stale")
    arguments = parser.parse_args()
    files = defined()
    places = ported(files)
    traces = traced()
    found = problems(files, places, traces)
    docs = DOCS.read_text()
    map_table = table(files, places, traces)
    if arguments.check:
        if written(docs, map_table) != docs:
            found.append("docs/engine.md's reference map is stale: run tools/engine_reference_map.py")
        found += stale_traces()
        if found:
            raise SystemExit("the reference map:\n  " + "\n  ".join(found))
        covered = sum(name in traces for name in places)
        print(f"the reference map holds {len(places)} ported functions, {covered} traced")
        return
    if found:
        raise SystemExit("the reference map:\n  " + "\n  ".join(found))
    DOCS.write_text(written(docs, map_table))
    print(f"wrote the reference map into docs/engine.md ({len(places)} functions)")


if __name__ == "__main__":
    main()
