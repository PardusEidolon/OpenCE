# Plan: The lighting, extended from the original's own

Every task is test first (`workflow.md`). Each phase ends with its
automated checks: the workspace tests, golden frames, the measurements
against stock, `census --check` and `nix flake check`. The phases run to
the end without manual stops, except the user's approval of golden frames.

## Phase 1: The switches and the measurements [checkpoint: 2e2a3ac6]
- [x] Task: A switch per extension, with stock as a value, in the settings, the dev panel and the `opence` tool. e68ca99a
  - The native app has no dev panel now: each switch is a `lighting.*` setting, set by the settings file, `--setting`, the control socket's `setting` request and `opence setting`. A value not drawn yet draws stock and says so once in the log.
- [x] Task: The sweeps against stock: seams, shadow edges, brightness and colour per region, on every map. 68ee430f
  - 161 runs over 23 maps, none failed; 51 findings, all on switches that still draw stock, so they are run-to-run noise (map_audit Phase 3 fixes the determinism first).
- [x] Task: Conductor - User Manual Verification 'Phase 1: Switches' (Protocol in workflow.md) (automated)

## Phase 2: Placed lights
- [x] Task: Read the placed and dynamic lights from the map. 9edea209
    - Placed and dynamic lights both go through rasterizer_lights; game.wasm sends them as LIGHTS (op 30) before PRESENT. Idle Blood Gulch submits 0 lights.
- [ ] Task: Their shadows and light on the world and the models.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Placed lights' (Protocol in workflow.md) (automated)

## Phase 3: Bounce and probes
- [ ] Task: Bake bounce light and probes from the lightmaps and lights, behind the loading screen, cached by the map's hash.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Bounce and probes' (Protocol in workflow.md) (automated)

## Phase 4: The sky and atmosphere
- [ ] Task: Fog and sky from the map's tags with scattering, matched to the original's colours.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Atmosphere' (Protocol in workflow.md) (automated)

## Phase 5: Ray tracing
- [ ] Task: Ray traced shadows and reflections, with the rasterized fallback chosen by the quality setting.
- [ ] Task: p99 ≤ 8.33 ms at 4K ultra on every map.
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Ray tracing' (Protocol in workflow.md) (automated)

## Phase 6: The report
- [ ] Task: The user's approval of the golden frames.
- [ ] Task: Write the report from the data.
- [ ] Task: Conductor - User Manual Verification 'Phase 6: The report' (Protocol in workflow.md) (automated)
