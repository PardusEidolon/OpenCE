# Plan: Coverage 2 of 11, Evidence for the owed

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: The fake world
- [x] Task: Write a reference test that compiles `object_new` and `object_delete` against the fake world and fails on an empty trace. 602a3326
- [x] Task: Build the shared fake world: object table, tag instances, a small synthetic map. bb93668a
- [ ] Task: Port the existing reference scripts that need a world onto it, keeping their traces unchanged.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: The fake world' (Protocol in workflow.md) (automated)

## Phase 2: The top of the owed list
- [ ] Task: Traces for `data_next_index`, `datum_new`, `object_new` and `object_delete`.
- [ ] Task: Traces for `real_random_range` and the other random helpers.
- [ ] Task: Traces for `hs_evaluate`, `hs_return` and `sound_class_get`.
- [ ] Task: A trace for `game_tick` over a short run of the fake world.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: The top of the owed list' (Protocol in workflow.md) (automated)

## Phase 3: The rest by system
- [ ] Task: Work the owed list down by caller count, one commit per system, fixing or recording each divergence.
- [ ] Task: The AI owed entries, on the fake world with actors, props and the map.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: The rest by system' (Protocol in workflow.md) (automated)

## Phase 4: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: The report' (Protocol in workflow.md) (automated)
