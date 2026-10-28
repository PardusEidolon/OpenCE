//! The game's render calls kept as one buffer per frame
//! (`opence_runtime::Frames`): each drawing or state call, with a copy of
//! what its pointer points at, in the order the game made it, and the
//! render and texture states whole before a draw whose states changed;
//! `Present` ends the frame.

use opence_runtime::{Error, Frame, Game, Runtime, Val, STATES};

/// the module: some of the device's calls, its two state arrays, and a
/// page for what the calls point at
const MODULE: &str = r#"
(module
  (import "env" "D3DDevice_Clear" (func $clear (param i32 i32 i32 i32 f32 i32)))
  (import "env" "D3DDevice_DrawIndexedVertices" (func $indexed (param i32 i32 i32)))
  (import "env" "D3DDevice_DrawVertices" (func $draw (param i32 i32 i32)))
  (import "env" "D3DDevice_SetTransform" (func $transform (param i32 i32)))
  (import "env" "D3DDevice_SetStreamSource" (func $stream (param i32 i32 i32)))
  (import "env" "D3DDevice_Present" (func $present (param i32 i32 i32 i32)))
  (import "env" "D3DDevice_GetBackBuffer" (func $back (param i32 i32 i32)))
  (memory (export "memory") 1)
  (global (export "D3D__RenderState") i32 (i32.const 0x2000))
  (global (export "D3D__TextureState") i32 (i32.const 0x2400))
  (func (export "clear") (param i32 i32 i32 i32 f32 i32)
    (call $clear (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4) (local.get 5)))
  (func (export "indexed") (param i32 i32 i32)
    (call $indexed (local.get 0) (local.get 1) (local.get 2)))
  (func (export "draw") (param i32 i32 i32)
    (call $draw (local.get 0) (local.get 1) (local.get 2)))
  (func (export "transform") (param i32 i32)
    (call $transform (local.get 0) (local.get 1)))
  (func (export "stream") (param i32 i32 i32)
    (call $stream (local.get 0) (local.get 1) (local.get 2)))
  (func (export "present")
    (call $present (i32.const 0) (i32.const 0) (i32.const 0) (i32.const 0)))
  (func (export "back")
    (call $back (i32.const 0) (i32.const 0) (i32.const 0x3000))))
"#;

const DATA: u32 = 0x1000;
const RENDER_STATE: u64 = 0x2000;

fn game() -> Result<Game, Error> {
    let bytes = wat::parse_str(MODULE).unwrap_or_default();
    Runtime::new(None)?.load(&bytes)
}

fn i32s(values: &[u32]) -> Vec<Val> {
    values
        .iter()
        .map(|value| Val::I32(value.cast_signed()))
        .collect()
}

fn call(game: &mut Game, name: &str, values: &[u32]) -> Result<(), Error> {
    game.call_with(name, &i32s(values)).map(|_| ())
}

fn present(game: &mut Game) -> Result<(), Error> {
    call(game, "present", &[])
}

/// each command as (name, args, data)
fn listed(frame: &Frame) -> Vec<(&'static str, Vec<u32>, Vec<u8>)> {
    frame
        .commands()
        .map(|command| (command.name(), command.args.to_vec(), command.data.to_vec()))
        .collect()
}

fn names(frame: &Frame) -> Vec<&'static str> {
    frame.commands().map(|command| command.name()).collect()
}

#[test]
fn nothing_is_handed_over_before_the_game_presents() -> Result<(), Error> {
    let mut game = game()?;
    let frames = game.frames();
    call(&mut game, "stream", &[0, 0x8000, 32])?;
    assert!(frames.take().is_none());
    assert_eq!(frames.presented(), 0);
    Ok(())
}

#[test]
fn a_frame_keeps_its_calls_in_order_and_ends_with_present() -> Result<(), Error> {
    let mut game = game()?;
    let frames = game.frames();
    call(&mut game, "stream", &[0, 0x8000, 32])?;
    call(&mut game, "draw", &[5, 0, 3])?;
    present(&mut game)?;
    let frame = frames.take().ok_or(Error::Import("no frame".into()))?;
    assert_eq!(frame.number, 0);
    assert_eq!(
        names(&frame),
        [
            "D3DDevice_SetStreamSource",
            "states",
            "D3DDevice_DrawVertices",
            "D3DDevice_Present"
        ]
    );
    let listed = listed(&frame);
    assert_eq!(listed[0].1, [0, 0x8000, 32]);
    assert_eq!(listed[2].1, [5, 0, 3]);
    Ok(())
}

#[test]
fn calls_the_frame_does_not_draw_with_are_not_kept() -> Result<(), Error> {
    let mut game = game()?;
    let frames = game.frames();
    game.call_with("back", &[])?;
    present(&mut game)?;
    let frame = frames.take().ok_or(Error::Import("no frame".into()))?;
    assert_eq!(names(&frame), ["D3DDevice_Present"]);
    Ok(())
}

#[test]
fn a_call_keeps_a_copy_of_what_it_points_at() -> Result<(), Error> {
    let mut game = game()?;
    let frames = game.frames();
    let matrix: Vec<u8> = (0..64).collect();
    game.write(u64::from(DATA), &matrix)?;
    call(&mut game, "transform", &[2, DATA])?;
    // (the game reuses the memory before the frame is drawn)
    game.write(u64::from(DATA), &[0xEE; 64])?;
    let indices: Vec<u8> = [7_u16, 8, 9].iter().flat_map(|i| i.to_le_bytes()).collect();
    game.write(u64::from(DATA + 0x100), &indices)?;
    call(&mut game, "indexed", &[4, 3, DATA + 0x100])?;
    present(&mut game)?;
    let frame = frames.take().ok_or(Error::Import("no frame".into()))?;
    let listed = listed(&frame);
    assert_eq!(listed[0].0, "D3DDevice_SetTransform");
    assert_eq!(listed[0].2, matrix);
    let draw = listed
        .iter()
        .find(|(name, ..)| *name == "D3DDevice_DrawIndexedVertices")
        .ok_or(Error::Import("no draw".into()))?;
    assert_eq!(draw.2, indices);
    Ok(())
}

#[test]
fn clear_keeps_its_rectangles_and_its_float_s_bits() -> Result<(), Error> {
    let mut game = game()?;
    let frames = game.frames();
    let rects: Vec<u8> = (0..32).collect();
    game.write(u64::from(DATA), &rects)?;
    let args = [
        Val::I32(2),
        Val::I32(DATA.cast_signed()),
        Val::I32(7),
        Val::I32(0x0011_2233),
        Val::F32(0.5_f32.to_bits()),
        Val::I32(0),
    ];
    game.call_with("clear", &args)?;
    // (a null pointer keeps nothing)
    let args = [
        Val::I32(0),
        Val::I32(0),
        Val::I32(7),
        Val::I32(0),
        Val::F32(1.0_f32.to_bits()),
        Val::I32(0),
    ];
    game.call_with("clear", &args)?;
    present(&mut game)?;
    let frame = frames.take().ok_or(Error::Import("no frame".into()))?;
    let listed = listed(&frame);
    assert_eq!(listed[0].1, [2, DATA, 7, 0x0011_2233, 0.5_f32.to_bits(), 0]);
    assert_eq!(listed[0].2, rects);
    assert!(listed[1].2.is_empty());
    Ok(())
}

#[test]
fn a_pointer_outside_the_memory_traps() -> Result<(), Error> {
    let mut game = game()?;
    assert!(call(&mut game, "transform", &[2, 0xFFFF_FFF0]).is_err());
    Ok(())
}

#[test]
fn the_states_are_kept_before_a_draw_only_when_they_changed() -> Result<(), Error> {
    let mut game = game()?;
    let frames = game.frames();
    game.write(RENDER_STATE, &[1, 0, 0, 0])?;
    call(&mut game, "draw", &[5, 0, 3])?;
    call(&mut game, "draw", &[5, 3, 3])?;
    // (an inline SetRenderState writes the array, with no call)
    game.write(RENDER_STATE, &[2, 0, 0, 0])?;
    call(&mut game, "draw", &[5, 6, 3])?;
    present(&mut game)?;
    let frame = frames.take().ok_or(Error::Import("no frame".into()))?;
    assert_eq!(
        names(&frame),
        [
            "states",
            "D3DDevice_DrawVertices",
            "D3DDevice_DrawVertices",
            "states",
            "D3DDevice_DrawVertices",
            "D3DDevice_Present"
        ]
    );
    let states: Vec<_> = frame
        .commands()
        .filter(|command| command.op == STATES)
        .collect();
    assert_eq!(states[0].data.len(), 144 * 4 + 4 * 32 * 4);
    assert_eq!(states[0].data[0], 1);
    assert_eq!(states[1].data[0], 2);
    Ok(())
}

#[test]
fn each_frame_starts_with_the_states_whole() -> Result<(), Error> {
    let mut game = game()?;
    let frames = game.frames();
    call(&mut game, "draw", &[5, 0, 3])?;
    present(&mut game)?;
    let _older = frames.take();
    call(&mut game, "draw", &[5, 0, 3])?;
    present(&mut game)?;
    let frame = frames.take().ok_or(Error::Import("no frame".into()))?;
    assert_eq!(frame.number, 1);
    assert_eq!(names(&frame)[0], "states");
    Ok(())
}

#[test]
fn the_newest_frame_is_taken_and_older_ones_dropped() -> Result<(), Error> {
    let mut game = game()?;
    let frames = game.frames();
    for _ in 0..5 {
        present(&mut game)?;
    }
    let frame = frames.take().ok_or(Error::Import("no frame".into()))?;
    assert_eq!(frame.number, 4);
    assert!(frames.take().is_none());
    assert_eq!(frames.presented(), 5);
    assert_eq!(frames.dropped(), 4);
    Ok(())
}
