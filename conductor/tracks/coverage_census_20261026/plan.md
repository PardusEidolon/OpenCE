# Plan: Coverage census

Every task is test first (`workflow.md`). Each phase ends with an automated
check: the whole workspace's tests, clippy, `nix flake check` and the
census check. The phases run to the end without manual stops. The one
manual check is at the end, with the report.

## Phase 1: The census crate [checkpoint: 6f67c558]
- [x] Task: The crate and source/'s symbols 10518229
  - [x] Tests on a fake source/ tree: headers parsed into address, size and name; malformed headers refused, not panicked on
  - [x] engine/crates/census, using bounds-checked readers and passing the workspace lints
  - [x] On the real tree: 6,747 functions in 1,611,122 bytes (the 6,962 counted the headers' data too)
- [x] Task: Citations from Rust and Swift be343e52
  - [x] Tests on fake crates and fake Swift Sources: backtick names in doc comments, excluding tests/
  - [x] Collected from all crates and engine/app/Sources; the reference map's 159 reproduced as the floor (236 cited)
- [x] Task: The call graph and reachability cd7ed869
  - [x] Tests on a fake tree: calls, calls through function tables, and entry points
  - [x] Entry points for a campaign game and a multiplayer game (both run through `main_loop`); the reached set recorded
- [x] Task: `opence census` and the flake check 9cec1a32
  - [x] The cli subcommand (summary, --check, --write)
  - [x] source/'s headers and the app's Sources visible to the crate's test in nix flake check
- [x] Task: Conductor - User Manual Verification 'Phase 1: The census crate' (Protocol in workflow.md) (automated) 6f67c558

## Phase 2: The classes and their requirements [checkpoint: f7f0d4ad]
- [x] Task: engine/census.toml and the six classes 71ea835c
  - [x] Tests: one failing case per requirement (Ported without a trace, Matched with only an engine test, Native without a primitive, Partial with nothing left out, Replaced without a listed reason, a function with no class)
  - [x] The schema: rules by directory, file and name, entries for single functions, and the fixed list of Replaced reasons
- [x] Task: The reference map moved in ffd8b56d
  - [x] Its REASONS moved to census.toml, its table rewritten from the census, tools/engine_reference_map.py removed
  - [x] `--check` runs the trace scripts' own --check; the cargo test checks that each cited trace exists
- [x] Task: The docs ffd8b56d
  - [x] docs/census.md (every function, by system) and the summary table in docs/engine.md
  - [x] Staleness test
- [x] Task: Replaced by rule f28dc8db
  - [x] Directory rules for rasterizer, render, shaders, bink, bungie_net, networking, editor, tool and the Xbox-only files, each with its stand-in named
  - [x] Replaced through a caller, from reachability
- [x] Task: Conductor - User Manual Verification 'Phase 2: The classes and their requirements' (Protocol in workflow.md) (automated) f7f0d4ad

## Phase 3: The walk through the systems
Each task reads the Rust and Swift sides, adds the citations they lack, and
classes every function of its directories with its evidence. Partial
entries list what they leave out.
- [x] Task: AI (ai) ac315e46
- [ ] Task: Scripts (hs)
- [ ] Task: The game loop (game, main, shell, saved games)
- [ ] Task: Objects (objects, units, items, devices)
- [ ] Task: Physics (physics)
- [ ] Task: Effects (effects)
- [ ] Task: Sound (sound)
- [ ] Task: Animation (models, first_person)
- [ ] Task: Cameras and cutscenes (camera, cutscene)
- [ ] Task: The interface, HUD, UI and input (interface, dialogs, text, strings, input)
- [ ] Task: The world (scenario, structures, bitmaps, cache, tag_files)
- [ ] Task: Helpers (math, cseries, memory) as Native, checked against the math reference trace and the fixed-order rule
- [ ] Task: Conductor - User Manual Verification 'Phase 3: The walk through the systems' (Protocol in workflow.md) (automated; deferred to the end)

## Phase 4: Tag fields and load-time fix-ups
- [ ] Task: The layout from source/
  - [ ] Tests on a fake *_definitions.h
  - [ ] clang's offsetof on every tag structure, written to engine/census/tag_layout.tsv; --check regenerates and compares it
- [ ] Task: Fields the original reads
  - [ ] Tests: field accesses through each definition's pointers
  - [ ] Marks for every field
- [ ] Task: Fields the importer keeps
  - [ ] Tests first: the importer's snapshots and the reference traces unchanged
  - [ ] The readers register named offsets in place of bare constants; the census reads the registration
- [ ] Task: Fields the engine uses
  - [ ] Typed fields read outside the importer, marked
- [ ] Task: Load-time fix-ups
  - [ ] The original's postprocess and defaults listed per tag group
  - [ ] Each compared with the importer, and the ones that differ recorded
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Tag fields and load-time fix-ups' (Protocol in workflow.md) (automated; deferred to the end)

## Phase 5: Scripts and sounds
- [ ] Task: Script functions and globals
  - [ ] Every one in the original's tables, marked wired or not, with the maps' calls counted where the user's maps are present (skipped without them)
- [ ] Task: Sounds
  - [ ] Every sound tag, and whether play asks for it, measured on devgulch and the campaign where the maps are present
  - [ ] The per-object sound limit recorded with what it needs (sound requests that know their object)
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Scripts and sounds' (Protocol in workflow.md) (automated; deferred to the end)

## Phase 6: The report
- [ ] Task: The numbers
  - [ ] Per system and class, in counts and bytes; tag-field gaps; fix-ups that differ; scripts and sounds
  - [ ] The Missing, Partial and evidence-less items, ranked by reach and by how much of play depends on them
- [ ] Task: The report Artifact and the proposed completion track
  - [ ] The phases of the coverage-completion track proposed from the ranking, to become a new track after your review
- [ ] Task: Conductor - User Manual Verification 'Phase 6: The report' (Protocol in workflow.md) (the one manual check: you review the report)
