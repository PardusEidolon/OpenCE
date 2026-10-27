# Plan: Coverage 3 of 11, Data fidelity

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: Strides and fix-ups
- [x] Task: Write the failing test comparing every importer stride with `tag_layout.tsv`. 31ff985d
- [x] Task: Fix the strides, starting with the `game_globals` difficulty block. 31ff985d
- [x] Task: Reference traces for the four differing fix-ups, then match them. 13cb5ef9 68c1fd7d (the script fix-up is checked against every stock map's stored nodes rather than a compiled trace)
- [x] Task: Conductor - User Manual Verification 'Phase 1: Strides and fix-ups' (Protocol in workflow.md) (automated): 1410 tests pass, census --check passes, treefmt clean

## Phase 2: Layouts from a second source
- [x] Task: Write tests for `vehicle_definition` and each shader subtype's layout against a synthetic tag.
- [x] Task: Fill the layouts from format documentation and check them on the user's maps where present.
- [x] Task: Fuzz targets for the new readers.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Layouts from a second source' (Protocol in workflow.md) (automated)

## Phase 3: Fields
- [ ] Task: Wire or class each of the 516 kept-but-unused fields, one commit per tag group.
- [ ] Task: Trace each of the 151 untraced fields.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Fields' (Protocol in workflow.md) (automated)

## Phase 4: Cache and bitmaps
- [ ] Task: Port, match or class the cache and bitmaps functions.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Cache and bitmaps' (Protocol in workflow.md) (automated)

## Phase 5: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 5: The report' (Protocol in workflow.md) (automated)
