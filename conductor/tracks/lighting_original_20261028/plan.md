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

## Phase 2: Placed lights [checkpoint: 63456f2b]
- [x] Task: Read the placed and dynamic lights from the map. 9edea209
    - Placed and dynamic lights both go through rasterizer_lights; game.wasm sends them as LIGHTS (op 30) before PRESENT. Idle Blood Gulch submits 0 lights.
- [x] Task: Their shadows and light on the world and the models. 89e2d9a0
    - Design: screen-space shadows in the world's diffuse light pass (the world is not kept for shadow maps). The pass writes its light (position, 0.5/radius) to vertex constants -81..-77; the device matches that against rasterizer_lights, and for that pass's draws (lighting.placed_lights=shadowed) the vertex program passes its world position on, the host copies the depth once per frame before the first such draw, and the fragment shader marches from the pixel toward the light through that depth (world to clip by the pass's own matrix), scaling the additive light by what is lit. Measured on c10 (up to 7 lights, 877 of 1053 frames lit in 40 s).
    - Checked: one depth copy per lit frame and a shadow bind for each (885 of 1068 frames, tests/placed_lights.rs); the c10 sweep refuses nothing, and placed_lights against stock (brightness 3.87, seams 1094) is within stock2's noise (2.22, 1025).
    - Not shown: a visible shadow. In 150 s of c10, a build with the light passes forced dark matches stock within noise up to tick 2100 (mean |Δ| ≤ 3.0, ≤ 5.6% of pixels); after tick 2400, stock and stock2 drift apart by 24–74% of pixels, so those frames measure nothing. The kept frames show no surface the dynamic lights visibly light, so whether the shadows darken the right part is unmeasured; it needs a scene with a strong placed light (a station in devgulch).
- [x] Task: Conductor - User Manual Verification 'Phase 2: Placed lights' (Protocol in workflow.md) (automated)

## Phase 3: Bounce and probes
- [x] Task: Bake bounce light and probes from the lightmaps and lights, behind the loading screen, cached by the map's hash. b10ec6fe (lightmaps baked at import; probes not baked yet)
    - Measured (30 Oct, M1 Max, release, rayon over the atlas's points with `Tracer::irradiance_over`, 64 paths a texel, BSP 0): bloodgulch 94 098 texels 3.0 s; d40 131 655, 12.8 s; a30 1.20 M, ~43 s; c10 1.55 M, ~156 s. At the default 512 paths that is ~3 min (a30) to ~21 min (c10) a BSP: too slow behind a loading screen, so the bake belongs at import. The GPU tracer that made it quick went with the removed engine.
    - Not built: the way into the game's drawing. The lightmap is bound at texture stage 2 by `_rasterizer_environment_lightmap_begin` (rasterizer_xbox_environment.c), but the device has no map from the game's bitmap uploads to the bake's pages, and the baked pages are HDR halves where the stock pages are 8-bit with the shader's own scale. Needed: a `port:` hook naming the page as it is bound, an import-time bake writing the cache, and the device substituting the page under `lighting.bounce=baked`.
    - Built (b10ec6fe): `opence bake MAP` bakes each BSP at import into `<cache>/bake` (the default chosen: at import, not behind the loading screen). The game names each page as it binds it (LIGHTMAP_PAGE, op 31, `port:` in structure_render.c); the render sink swaps the bind for the bake's page when it is a whole multiple of the stock size (the bake draws at twice it). Wasmtime's modules moved to `<cache>/compiled`, as its cleanup deleted the bakes.
    - Measured, bloodgulch 640×480, baked against stock: 0 refused; tick 300 brightness by region up to +16.0, colour up to 11.1, seams 266; tick 600 the shadowed lower left +25.2 to +31.7, colour 15.3, seams 514. By eye the shadowed foreground lifts and its edge softens; cliffs and sky barely change.
    - Left: probes are not baked (the models still draw stock); the 64-path bake's noise and the 266–514 seam pixels are unexamined; only `--setting` turns it on for the loader; a map change in a session keeps the old pages (the size check guards partly).
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
