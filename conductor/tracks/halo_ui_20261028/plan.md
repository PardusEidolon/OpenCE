# Plan: The Halo UI rewrite, a plugin drawn at 4K from `ui.map`

Every task is test first (`workflow.md`). Each phase ends with its
automated checks: the workspace tests, golden frames, `census --check` and
`nix flake check`. The phases run to the end without manual stops, except
the design approval, which the user gives.

## Phase 1: Design
- [x] Task: Redraw the existing mockups as static prototypes in a design artifact, without the launcher boards; add the pause, post-game and loading screens. (canvas v43, https://claude.ai/artifact/MqqSyPQ8J4pcW7i9LwRbk1)
- [ ] Task: The user's approval of every screen. (waiting on the user: the drawing against the prototypes, Phase 3 tasks 1, 3 and 4, waits on it)
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Design' (Protocol in workflow.md)

## Phase 2: Reading the game's UI [checkpoint: e230b4d1]
- [x] Task: Read the widget tree from the game's memory through the read-only views; fuzz the reader. fde71a8e
- [x] Task: Signed distance field fonts from the font tags, built behind the loading screen. 3327ed7d
- [x] Task: Conductor - User Manual Verification 'Phase 2: Reading the UI' (Protocol in workflow.md) (automated)

## Phase 3: Drawing and input [checkpoint: 12d45c9c]
- [ ] Task: Draw the layout at 4K over the background scene; golden frames of the main menu. (blocked: waits on the user's approval of the Phase 1 prototypes)
- [x] Task: Keyboard navigation and the mouse mapped onto it, driven by tests through the control socket. d4998cee
- [ ] Task: Every other screen, each against its approved prototype. (blocked: waits on the user's approval of the Phase 1 prototypes)
- [ ] Task: The pause menu, the post-game screens and the HUD's text. (blocked: waits on the user's approval of the Phase 1 prototypes)
- [x] Task: Conductor - User Manual Verification 'Phase 3: Drawing and input' (Protocol in workflow.md) (automated)

## Phase 4: The loading screen
- [ ] Task: Draw the Xbox loading screen at 4K from the original's code.
- [x] Task: The chant from the user's xbe, silent without it. cfdcf9b2
- [ ] Task: No hitch or black frame into the first frame of the next map.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: The loading screen' (Protocol in workflow.md) (automated)
