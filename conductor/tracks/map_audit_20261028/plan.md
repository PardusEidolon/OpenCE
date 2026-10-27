# Plan: The map and cutscene audit, every map played against stock

Every task is test first (`workflow.md`). Each phase ends with its
automated checks: the workspace tests, the reference traces, `census
--check` and `nix flake check`. The phases run to the end without manual
stops.

## Phase 1: The runs
- [ ] Task: Record inputs that play each map to its end, and play them headless through the `opence` tool.
- [ ] Task: Play each cutscene in full.
- [ ] Task: Record frames at fixed ticks, the sound mix, the script events, the frame times and the faults, under the cache folder.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: The runs' (Protocol in workflow.md) (automated)

## Phase 2: Comparing with stock
- [ ] Task: Play the same inputs with the stock rendering and sound.
- [ ] Task: Image differences per region, the sound's loudness and spectrum, the script events in order; thresholds set from repeated stock runs.
- [ ] Task: List the findings with their map, tick, frames and measurement.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Comparing' (Protocol in workflow.md) (automated)

## Phase 3: Fixing and iterating
- [ ] Task: Trace each finding to its cause and fix it, each with a failing test first.
- [ ] Task: Run every map and cutscene again until they pass, or list what remains with its reason.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Fixing' (Protocol in workflow.md) (automated)

## Phase 4: The report
- [ ] Task: Write the report from the data: the findings, their causes and fixes, what remains, and p99 per map.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: The report' (Protocol in workflow.md) (automated)
