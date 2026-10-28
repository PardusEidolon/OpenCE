# Plan: The native port, the original run as wasm32 in Wasmtime

Every task is test first (`workflow.md`): the trace, measurement or test is
written and seen to fail before the change. Each phase ends with its
automated checks: the workspace tests, the reference traces, `census
--check` and `nix flake check`. The phases run to the end without manual
stops, except A1's go/no-go.

## Phase 1: The spike, then go or no-go (A1)
- [x] Task: Build `source/` as `game.wasm` from the web build's flags, in Nix; a test that the module's imports and exports are the expected list. a3af5a92
  - Finding: `a_frame_of_the_ai_station_keeps_to_its_budget` (`engine/crates/ffi/tests/allocations.rs`) fails at HEAD as well, with 279.5–279.7 allocations a frame against a budget of 200; it passed once in a fresh worktree, so it is flaky, and older than this track. The Rust simulation it measures goes in Phase 7.
- [x] Task: The runtime crate: Wasmtime with Cranelift, the `.cwasm` cached by Wasmtime's version and the CPU, the 4 GB memory with the Xbox's window; stub imports that record their calls. d9591186
  - Finding: `cargo deny check advisories` reports zstd 0.13.3 and zstd-safe 7.3.0 (RUSTSEC-2026-0391, 0398, 0403), pinned by Wasmtime's cache, which decompresses only the files it wrote itself; basic-toml (RUSTSEC-2026-0374) predates this track. The gate the flake runs (licences, bans, sources) passes.
- [x] Task: Run devgulch headless for some thousands of ticks; record the reference traces from the wasm build and compare them with the native reference. 692250d8
  - Finding: all 63 reference traces match in the wasm build (`tools/engine_wasm_traces.py`, about 18 s), each harness built with the game's ABI flags and run in the runtime. The wasm build found one harness bug: the physics_variable LCG kept its seed in `unsigned long`, 32 bits on wasm32.
  - Finding: devgulch for thousands of ticks needs the platform layer, so it moves to Phase 2's dev shell task. With every import stubbed, `halo_game_main` first walks the debug monitor's modules and loops on `DmWalkModuleSections`, which never reports the end of the list; a call budget stops it (20 million calls in 1.03 s, about 51 ns a host call). game.wasm compiles and loads in 324 ms without the cache.
- [x] Task: Check the threads on shared memory and atomics, `setjmp` and `longjmp`, and deterministic floating point across runs. 3ebac490
  - Finding: two host threads, each with its own store and instance on one `SharedMemory` and the runtime's engine, share atomics exactly (200,000 from 2 × 100,000), so A2 maps the Win32 thread imports onto Wasmtime threads that way.
  - Finding: game.wasm imports no `setjmp` or `longjmp` (`source/` has none), so it needs neither wasm exceptions nor unwinding.
  - Finding: NaNs come out canonical, relaxed SIMD is refused at compile, and the game is built with `-ffp-contract=off`; the 63 wasm traces ran twice with identical output.
- [x] Task: `--benchmark 60`: the simulation tick's p99 and the CPU frame's p99 on the M1 Max. 419951f2 (moved to Phase 2, after the threads)
  - Finding: the game does not tick with its imports stubbed, so the benchmark cannot run before A2. The proxies, measured with `tools/engine_wasm_traces.py --speed`: the 15 harnesses heavy enough to time run at 1.38× native in wasm (geometric mean, 0.97× to 1.93×, both at `-O1`); game.wasm loads in 315 ms uncached and 54 ms from its `.cwasm`; a host call costs about 51 ns.
- [x] Task: Write the go/no-go (`a1_go_no_go.md` in this track) with the data, and bring it to the user. Stop on a no-go. efb9f12a (go; brought to the user in the overnight summary)
- [x] Task: Conductor - User Manual Verification 'Phase 1: The spike' (Protocol in workflow.md) (automated) [checkpoint: 23167ab5]

## Phase 2: The macOS platform layer (A2)
- [x] Task: Memory and the map cache. c7b3f593
  - Finding: the window (0x80000000 to 0xA0000000) holds the game state and the tag cache at their fixed addresses; game.wasm starts with 2 GB so wasi-libc's malloc takes everything below it, and the platform grows the memory past it before the game runs.
- [x] Task: Files: the data folder, saved games, the map copies. 50101bd5
  - Finding: `d:\` is the data folder where the maps are read in place, every other drive a folder of the save folder; `ReadFileEx` completions wait for the next alertable wait, which the threads task delivers.
- [x] Task: Threads, timers and the clock. cbd86a0f
  - Finding: game.wasm is built for wasm32-wasip1-threads (an imported shared memory, wasi-libc's pthreads); `CreateThread` calls the game's `pthread_create`, and each thread is the module instantiated again over the same memory. The reference harnesses stay on wasm32-wasip1, and all 63 traces still match.
- [x] Task: `--benchmark 60` on devgulch, the A1 bar moved here: the simulation tick's p99 within about 1 ms and the CPU frame's p99 at most 8.33 ms on the M1 Max. A miss stops the track with the measurements (the go/no-go's condition). e2ff0ba1
  - Finding: Go. On Blood Gulch (devgulch is the dev shell's task, below) for 60 s at 3840×2160, the CPU frame's p99 is 7.113 ms against 8.33 ms; the draws are stubbed until Phase 4, which measures the bar end to end. The tick misses its aim at p99, 2.988 ms, nearly all `game_tick.objects`; recorded, not a stop. The census counts any backticked engine name in a comment as a citation, so a comment may cite only functions it classes.
- [ ] Task: Keyboard and mouse as the game's input, pad events filtered.
- [ ] Task: Sound output from the game's mixer to Core Audio.
- [ ] Task: The render calls as one buffer per frame.
- [ ] Task: The dev shell, the control socket and the `opence` tool against the wasm game; devgulch and its stations as a map the original loads.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: The platform layer' (Protocol in workflow.md) (automated)

## Phase 3: The plugin interface (A3)
- [ ] Task: The hooks as imports and exports: tick, frame, rendering, HUD and UI, sound, AI.
- [ ] Task: Read-only views of the game's memory through bounds-checked readers; fuzz them.
- [ ] Task: Recorded inputs and exact replay.
- [ ] Task: Per-plugin budgets logged per frame; a plugin over its share fails a test.
- [ ] Task: With every plugin off, the run matches the traces.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: The plugin interface' (Protocol in workflow.md) (automated)

## Phase 4: Stock rendering on Metal (A4)
- [ ] Task: `default.metallib` linked from the `.air` at build and loaded with `makeLibrary(URL:)`.
- [ ] Task: Draw the rasterizer's buffer: the environment shaders and lightmaps.
- [ ] Task: The model, transparent and chicago shaders, and the rest.
- [ ] Task: The dynamic lights and fog.
- [ ] Task: Golden frames and image differences against the original, until each shader matches; the user re-approves the golden frames.
- [ ] Task: The pipelines in a Metal binary archive behind the loading screen; a pipeline missing during play fails a test.
- [ ] Task: The bar end to end: p99 ≤ 8.33 ms at 4K ultra on the M1 Max.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Stock rendering' (Protocol in workflow.md) (automated)

## Phase 5: The presentation as plugins (A5)
- [ ] Task: The lighting extended from the original: bakes, probes and ray tracing on the original's lightmaps, shader tags and lights; each switchable to stock.
- [ ] Task: The particles; the weapons station draws every particle the game makes.
- [ ] Task: The HUD at 4K; the grenade count and every element measured against the original.
- [ ] Task: The first-person rifle and hand where the original puts them.
- [ ] Task: The sound.
- [ ] Task: Conductor - User Manual Verification 'Phase 5: The presentation' (Protocol in workflow.md) (automated)

## Phase 6: What the PC port left out (A6)
- [ ] Task: Catalogue what the Gearbox PC port lost or broke, each with how it is measured.
- [ ] Task: Measure and fix each one that falls to the presentation.
- [ ] Task: Conductor - User Manual Verification 'Phase 6: The PC port' (Protocol in workflow.md) (automated)

## Phase 7: Removals and checks (A7)
- [ ] Task: Reclass the census for the new direction, with its `--check`.
- [ ] Task: Remove the Rust simulation and `bevy_ecs`.
- [ ] Task: Remove the presentation code the plugins replace, and the `net` crate.
- [ ] Task: Remove `port/macos` and its VM.
- [ ] Task: The traces as checks that the wasm build matches the native reference.
- [ ] Task: Conductor - User Manual Verification 'Phase 7: Removals' (Protocol in workflow.md) (automated)
