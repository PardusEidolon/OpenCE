# Project Tracks

This file tracks all major tracks for the project.

---


- [x] **Track: The game in a macOS window: a Swift app, a Metal renderer, native input and audio**
  *Link: [./tracks/macos_window_metal_20261018/](./tracks/macos_window_metal_20261018/)*

- [x] **Track: Ragdolls, dismemberment and physics debris with Jolt Physics in the macOS app**
  *Link: [./tracks/ragdolls_jolt_20261020/](./tracks/ragdolls_jolt_20261020/)*

- [x] **Track: A new native engine in Rust and Swift: no 32-bit VM, maps imported to a typed format, a modern Metal renderer, and the game rebuilt in seven milestones**
  *Link: [./tracks/native_engine_20261021/](./tracks/native_engine_20261021/)*

- [x] **Track: The developer shell, engine hooks and a test map: a terminal-like overlay, a control socket and the opence tool, debug drawing, live shader reload, a test driver, the devmap, and placement checks against the reference game**
  *Link: [./tracks/dev_tooling_20261021/](./tracks/dev_tooling_20261021/)*

- [x] **Track: Lighting, shaders, decals and effects drawn: a physically based shader model for every Halo shader class, re-baked bounced light with AO and probes, dynamic lights and shadows, lens flares, decals and particles drawn**
  *Link: [./tracks/lighting_shaders_20261021/](./tracks/lighting_shaders_20261021/)*

- [x] **Track: The Halo UI from the user's ui.map, a launcher with only Play, Settings and Exit, and the Xbox loading screen between campaign maps**
  *Link: [./tracks/halo_ui_20261021/](./tracks/halo_ui_20261021/)*

- [x] **Track: The complete animation set for bipeds, weapons, the first person, vehicles, AI, devices and cutscenes**
  *Link: [./tracks/animation_20261021/](./tracks/animation_20261021/)*

- [x] **Track: Complete physics: objects as mass-point bodies, collisions, forces, grenades, vehicles and devices**
  *Link: [./tracks/physics_20261021/](./tracks/physics_20261021/)*

- [x] **Track: The complete particle system: particles and their physics, particle systems, effects, contrails, weather and wind**
  *Link: [./tracks/particles_20261021/](./tracks/particles_20261021/)*

- [x] **Track: Sound: every sound tag, looping sounds, music, dialogue, sound environments and the scripts' waits on sounds**
  *Link: [./tracks/sound_20261021/](./tracks/sound_20261021/)*

- [ ] **Track: Coverage of every map, a cutscene audit against the reference game, the fixes, and a review of the engine**
  *Link: [./tracks/coverage_audit_20261021/](./tracks/coverage_audit_20261021/)*

- [ ] **Track: Placed lights: one light component with Baked, Stationary and Movable mobility, ambient as probe overrides, placed from the dev panel and kept in a per-map overlay**
  *Link: [./tracks/placed_lights_20261025/](./tracks/placed_lights_20261025/)*

- [ ] **Track: Modern lighting and atmosphere: a procedural sky blended with the original, a day-night cycle, volumetric fog and clouds, contact shadows and occlusion, emissive area lights, reflections and glass**
  *Link: [./tracks/atmosphere_20261023/](./tracks/atmosphere_20261023/)*

- [ ] **Track: Weather, water and a world that reacts: weather systems, wet and snowy surfaces, ground marks and decals, dirt and blood on bodies and models, and simulated water with a new water shader**
  *Link: [./tracks/living_world_20261023/](./tracks/living_world_20261023/)*

- [ ] **Track: Decision models for the Covenant: a commander layer over Halo's AI with local models such as kev-4b, host-only and recorded**
  *Link: [./tracks/ai_decisions_20261023/](./tracks/ai_decisions_20261023/)*

- [~] **Track: The menus, launcher and loading screen redesigned in the original's design language, designed first in a Claude design artifact**
  *Link: [./tracks/menu_redesign_20261024/](./tracks/menu_redesign_20261024/)*

- [x] **Track: Coverage census: every function of the original, every tag field and its fix-ups, the scripts and the sounds, classed against the engine with the evidence each class asks for, kept current by a Rust census check**
  *Link: [./tracks/coverage_census_20261026/](./tracks/coverage_census_20261026/)*

- [x] **Track: Coverage: classify and cite: cite the engine's existing equivalents, class the small helpers as Native, add the library reason, name the address-only functions, and a ratchet that stops the counts rising**
  *Link: [./tracks/coverage_classify_20261026/](./tracks/coverage_classify_20261026/)*

- [x] **Track: Coverage: evidence for the owed: reference traces for the 675 functions the census says owe evidence, starting with the datum allocators, random ranges, the script evaluator and the game tick, and a fake world that holds objects, tags and a map**
  *Link: [./tracks/coverage_evidence_20261026/](./tracks/coverage_evidence_20261026/)*

- [x] **Track: Coverage: data fidelity: every tag field, stride and load-time fix-up matched to the original, vehicle_definition and the shader subtypes from a second source, and the kept-but-unused and untraced fields wired or justified**
  *Link: [./tracks/coverage_data_20261026/](./tracks/coverage_data_20261026/)*

- [~] **Track: Coverage: units, players' effects and the object systems: unit update and damage, biped movement, player effects, objects, items, devices, effects and physics, ported and traced**
  *Link: [./tracks/coverage_units_20261026/](./tracks/coverage_units_20261026/)*
  *Paused 27 October 2026, with the coverage tracks after it, while the hybrid (source/'s simulation compiled natively under the Rust engine) is tried.*

- [ ] **Track: Coverage: the game loop, flow and saves: the main loop iteration, winning a map and the next level, the game engine's update, players and respawn, saved games and the shell**
  *Link: [./tracks/coverage_game_loop_20261026/](./tracks/coverage_game_loop_20261026/)*

- [ ] **Track: Coverage: animation and the world's queries: frame info, root motion and inverse kinematics, models, cluster partitions, the PVS and PAS tests, scenario locations, structures and the geometry maths**
  *Link: [./tracks/coverage_world_animation_20261026/](./tracks/coverage_world_animation_20261026/)*

- [ ] **Track: Coverage: the interface and HUD: interface_draw_screen and hud_draw_screen in full, the HUD's script natives, the UI widgets, text and unicode, input and dialogs**
  *Link: [./tracks/coverage_interface_20261026/](./tracks/coverage_interface_20261026/)*

- [ ] **Track: Coverage: cameras and cutscenes: the director, the observer, the bored, static, orbiting and flying cameras, recorded animations, the letterbox and the title**
  *Link: [./tracks/coverage_cameras_20261026/](./tracks/coverage_cameras_20261026/)*

- [ ] **Track: Coverage: scripts and sounds: the script system, every allowed native and global, sound requests that carry their object, per-object limits, the mouth aperture, and play sweeps of each campaign map**
  *Link: [./tracks/coverage_scripts_sounds_20261026/](./tracks/coverage_scripts_sounds_20261026/)*

- [ ] **Track: Coverage: AI core: actor update, the decide-action and action-handle families, perception, firing positions, placement, props, knowledge, emotions, swarms and dormancy, ported from source/ and traced**
  *Link: [./tracks/coverage_ai_20261026/](./tracks/coverage_ai_20261026/)*

- [ ] **Track: Coverage: the 100% gate: the ratchet at zero in every system, a final census and measurement sweep against the stock game, and the report that hands over to features**
  *Link: [./tracks/coverage_complete_20261026/](./tracks/coverage_complete_20261026/)*
