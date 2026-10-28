//! Direct3D's device as the platform creates it (`opence_runtime::d3d`):
//! the device and its surfaces, the default render and texture states
//! written where the game keeps them, the device's capabilities and its
//! transforms. Each follows the Linux port (`port/linux/src/d3d8_gl.c`).

use opence_runtime::{answers, Error, Runtime, Val};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// where the module keeps the render states and the texture states, as
/// game.wasm exports their addresses
const RENDER_STATE: u32 = 1024;
const TEXTURE_STATE: u32 = 2048;

/// the module: each import re-exported, the state arrays' addresses, and a
/// malloc that bumps from 8192
const DEVICE: &str = r#"
(module
  (import "env" "Direct3DCreate8" (func $create8 (param i32) (result i32)))
  (import "env" "Direct3D_SetPushBufferSize" (func $push (param i32 i32)))
  (import "env" "Direct3D_CreateDevice" (func $create (param i32 i32 i32 i32 i32 i32) (result i32)))
  (import "env" "D3DDevice_GetBackBuffer" (func $back (param i32 i32 i32)))
  (import "env" "D3DDevice_GetDepthStencilSurface" (func $depth (param i32) (result i32)))
  (import "env" "D3DDevice_GetDeviceCaps" (func $caps (param i32)))
  (import "env" "D3DDevice_SetTransform" (func $set (param i32 i32)))
  (import "env" "D3DDevice_GetTransform" (func $get (param i32 i32)))
  (import "env" "D3DDevice_SetViewport" (func $viewport (param i32)))
  (memory (export "memory") 1)
  (global (export "D3D__RenderState") i32 (i32.const 1024))
  (global (export "D3D__TextureState") i32 (i32.const 2048))
  (global $next (mut i32) (i32.const 8192))
  (func (export "malloc") (param i32) (result i32)
    (global.get $next)
    (global.set $next (i32.add (global.get $next) (i32.and (i32.add (local.get 0) (i32.const 15)) (i32.const -16)))))
  (func (export "create8") (param i32) (result i32) (call $create8 (local.get 0)))
  (func (export "push") (param i32 i32) (call $push (local.get 0) (local.get 1)))
  (func (export "create") (param i32 i32 i32 i32 i32 i32) (result i32)
    (call $create (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4) (local.get 5)))
  (func (export "back") (param i32 i32 i32) (call $back (local.get 0) (local.get 1) (local.get 2)))
  (func (export "depth") (param i32) (result i32) (call $depth (local.get 0)))
  (func (export "caps") (param i32) (call $caps (local.get 0)))
  (func (export "set") (param i32 i32) (call $set (local.get 0) (local.get 1)))
  (func (export "get") (param i32 i32) (call $get (local.get 0) (local.get 1)))
  (func (export "viewport") (param i32) (call $viewport (local.get 0))))
"#;

/// where the test puts the presentation parameters and the pointers the
/// imports return
const PRESENTATION: u32 = 256;
const RETURNED: u32 = 512;

/// D3DERR_NOTFOUND, as the game reads it back
const NOT_FOUND: i32 = 0x8876_0866_u32 as i32;

fn first(results: &[Val]) -> Option<i32> {
    results.first().and_then(Val::i32)
}

fn word(game: &opence_runtime::Game, address: u32) -> Result<u32, Error> {
    let bytes = game.read(u64::from(address), 4)?;
    let mut word = [0; 4];
    word.copy_from_slice(&bytes);
    Ok(u32::from_le_bytes(word))
}

fn float(game: &opence_runtime::Game, address: u32) -> Result<f32, Error> {
    Ok(f32::from_bits(word(game, address)?))
}

fn i32_of(value: u32) -> Val {
    Val::I32(value.cast_signed())
}

/// a game whose device has been created at `width` by `height`
fn created(width: u32, height: u32) -> Result<opence_runtime::Game, Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    let mut parameters = Vec::new();
    for value in [width, height] {
        parameters.extend(value.to_le_bytes());
    }
    parameters.resize(13 * 4, 0);
    game.write(u64::from(PRESENTATION), &parameters)?;
    let args = [0, 1, 0, 0x40, PRESENTATION, RETURNED].map(i32_of);
    assert_eq!(first(&game.call_with("create", &args)?), Some(0));
    Ok(game)
}

#[test]
fn creating_the_device_returns_it_and_sets_the_default_states() -> Result<(), Error> {
    let mut game = created(1280, 720)?;
    assert_ne!(
        first(&game.call_with("create8", &[Val::I32(0x0120)])?),
        Some(0)
    );
    game.call_with("push", &[Val::I32(0x40000), Val::I32(0x8000)])?;
    assert_ne!(word(&game, RETURNED)?, 0, "a device");
    let state = |index: u32| word(&game, RENDER_STATE + index * 4);
    // ZENABLE, ZWRITEENABLE, ZFUNC LESSEQUAL, COLORWRITEENABLE_ALL
    assert_eq!(state(123)?, 1);
    assert_eq!(state(64)?, 1);
    assert_eq!(state(57)?, 515);
    assert_eq!(state(67)?, 0x0101_0101);
    // SRCBLEND ONE, DESTBLEND ZERO, BLENDOP ADD, CULLMODE CCW, FRONTFACE CW,
    // FILLMODE SOLID
    assert_eq!(state(62)?, 1);
    assert_eq!(state(63)?, 0);
    assert_eq!(state(74)?, 32774);
    assert_eq!(state(127)?, 2305);
    assert_eq!(state(126)?, 2304);
    assert_eq!(state(119)?, 6914);
    // ALPHAFUNC and STENCILFUNC ALWAYS, the stencil's masks and its ops KEEP
    assert_eq!(state(58)?, 519);
    assert_eq!(state(70)?, 519);
    assert_eq!(state(72)?, 0xff);
    assert_eq!(state(73)?, 0xff);
    for op in [125, 68, 69] {
        assert_eq!(state(op)?, 7680);
    }
    // every stage wraps, filters by point and has one level of anisotropy
    for stage in 0..4 {
        let texture = |index: u32| word(&game, TEXTURE_STATE + (stage * 32 + index) * 4);
        for (index, value) in [(10, 1), (11, 1), (12, 1), (13, 1), (14, 1), (18, 1)] {
            assert_eq!(texture(index)?, value, "stage {stage} state {index}");
        }
    }
    assert_eq!(game.call_count("env.Direct3D_CreateDevice"), 1);
    Ok(())
}

#[test]
fn the_back_buffer_and_depth_surface_describe_the_screen() -> Result<(), Error> {
    let mut game = created(1280, 720)?;
    game.call_with("back", &[Val::I32(0), Val::I32(0), i32_of(RETURNED)])?;
    let back = word(&game, RETURNED)?;
    // a surface the device owns, with the caller's reference
    assert_eq!(word(&game, back)?, 0x0005_0000 | 2);
    // LIN_A8R8G8B8, two dimensions, DMA channel A
    assert_eq!(word(&game, back + 12)?, (18 << 8) | (2 << 4) | 1);
    // a pitch of 5120 bytes (in 64s, less one), 720 rows and 1280 columns
    assert_eq!(word(&game, back + 16)?, (79 << 24) | (719 << 12) | 1279);

    assert_eq!(
        first(&game.call_with("depth", &[i32_of(RETURNED + 4)])?),
        Some(0)
    );
    let depth = word(&game, RETURNED + 4)?;
    assert_ne!(depth, back);
    assert_eq!(word(&game, depth)?, 0x0005_0000 | 2);
    assert_eq!(word(&game, depth + 12)?, (46 << 8) | (2 << 4) | 1);
    assert_eq!(word(&game, depth + 16)?, (79 << 24) | (719 << 12) | 1279);
    Ok(())
}

#[test]
fn without_a_device_there_is_no_depth_surface() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    assert_eq!(
        first(&game.call_with("depth", &[i32_of(RETURNED)])?),
        Some(NOT_FOUND)
    );
    assert_eq!(word(&game, RETURNED)?, 0);
    Ok(())
}

#[test]
fn a_device_made_without_a_size_is_640_by_480() -> Result<(), Error> {
    let mut game = created(0, 0)?;
    game.call_with("back", &[Val::I32(0), Val::I32(0), i32_of(RETURNED)])?;
    let back = word(&game, RETURNED)?;
    // 2560 bytes a row is 40 64s
    assert_eq!(word(&game, back + 16)?, (39 << 24) | (479 << 12) | 639);
    Ok(())
}

#[test]
fn the_capabilities_are_the_linux_port_s() -> Result<(), Error> {
    let mut game = created(640, 480)?;
    let caps = 4096;
    game.write(u64::from(caps), &[0xAA; 212])?;
    game.call_with("caps", &[i32_of(caps)])?;
    let at = |offset: u32| word(&game, caps + offset);
    // HAL, then every capability bit cleared
    assert_eq!(at(0)?, 1);
    for offset in (4..88).step_by(4) {
        assert_eq!(at(offset)?, 0, "offset {offset}");
    }
    for (offset, value) in [
        (88, 4096),
        (92, 4096),
        (96, 512),
        (100, 8192),
        (104, 4096),
        (108, 4),
        (148, 4),
        (152, 4),
        (160, 8),
        (168, 4),
        (180, 0xfffff),
        (184, 0xffff),
        (188, 16),
        (192, 255),
        (196, 0xFFFE_0101),
        (200, 192),
        (204, 0xFFFF_0101),
    ] {
        assert_eq!(at(offset)?, value, "offset {offset}");
    }
    assert!((float(&game, caps + 176)? - 64.0).abs() < f32::EPSILON);
    assert!((float(&game, caps + 208)? - 1.0).abs() < f32::EPSILON);
    Ok(())
}

#[test]
fn transforms_start_as_the_identity_and_keep_what_is_set() -> Result<(), Error> {
    let mut game = created(640, 480)?;
    let matrix = 4096;
    game.call_with("get", &[Val::I32(6), i32_of(matrix)])?;
    for index in 0..16 {
        let expected = if index % 5 == 0 { 1.0 } else { 0.0 };
        assert!((float(&game, matrix + index * 4)? - expected).abs() < f32::EPSILON);
    }
    let set: Vec<u8> = (0..16_u8)
        .flat_map(|value| f32::from(value).to_le_bytes())
        .collect();
    game.write(u64::from(matrix), &set)?;
    game.call_with("set", &[Val::I32(1), i32_of(matrix)])?;
    game.write(u64::from(matrix), &[0; 64])?;
    game.call_with("get", &[Val::I32(1), i32_of(matrix)])?;
    assert_eq!(game.read(u64::from(matrix), 64)?, set);
    // a transform past D3DTS_MAX is ignored, and nothing is written
    game.write(u64::from(matrix), &[0x55; 64])?;
    game.call_with("get", &[Val::I32(10), i32_of(matrix)])?;
    assert_eq!(game.read(u64::from(matrix), 64)?, vec![0x55; 64]);
    game.call_with("set", &[Val::I32(10), i32_of(matrix)])?;
    // and a viewport is kept
    game.call_with("viewport", &[i32_of(matrix)])?;
    assert_eq!(game.call_count("env.D3DDevice_SetViewport"), 1);
    Ok(())
}

#[test]
fn the_platform_answers_direct3d_s_creation() {
    for name in [
        "env.Direct3DCreate8",
        "env.Direct3D_SetPushBufferSize",
        "env.Direct3D_CreateDevice",
        "env.D3DDevice_GetBackBuffer",
        "env.D3DDevice_GetDepthStencilSurface",
        "env.D3DDevice_GetDeviceCaps",
        "env.D3DDevice_SetTransform",
        "env.D3DDevice_GetTransform",
        "env.D3DDevice_SetViewport",
    ] {
        assert!(answers(name), "{name}");
    }
    assert!(!answers("env.D3DDevice_DrawVertices"));
}

/// the screen's imports, re-exported
const SCREEN: &str = r#"
(module
  (import "env" "halo_screen_width" (func $width (result i32)))
  (import "env" "halo_screen_scale" (func $scale (result f32)))
  (import "env" "D3DDevice_Present" (func $present (param i32 i32 i32 i32)))
  (import "env" "D3DDevice_Release" (func $release (result i32)))
  (import "env" "halo_screen_commit" (func $commit (result i32)))
  (memory (export "memory") 1)
  (func (export "width") (result i32) (call $width))
  (func (export "commit") (result i32) (call $commit))
  (func (export "scale") (result f32) (call $scale))
  (func (export "present") (call $present (i32.const 0) (i32.const 0) (i32.const 0) (i32.const 0)))
  (func (export "release") (result i32) (call $release)))
"#;

fn screen(display: Option<(u32, u32)>) -> Result<(i32, f32), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(SCREEN))?;
    if let Some((width, height)) = display {
        game.set_display(width, height);
    }
    let width = first(&game.call_with("width", &[])?).unwrap_or_default();
    let scale = game
        .call_with("scale", &[])?
        .first()
        .and_then(Val::f32)
        .unwrap_or_default();
    Ok((width, scale))
}

#[test]
fn without_a_display_the_screen_is_the_xbox_s() -> Result<(), Error> {
    assert_eq!(screen(None)?, (640, 1.0));
    Ok(())
}

#[test]
fn the_screen_keeps_480_lines_and_takes_the_display_s_shape() -> Result<(), Error> {
    // 16:9 at 4K: 853 columns, even, and the larger of the two scales
    let (width, scale) = screen(Some((3840, 2160)))?;
    assert_eq!(width, 852);
    assert!((scale - 3840.0 / 852.0).abs() < 1e-5, "{scale}");
    // 4:3 is the Xbox's own, a narrow display keeps 640, a very wide one 1920
    assert_eq!(screen(Some((1280, 960)))?, (640, 2.0));
    assert_eq!(screen(Some((960, 1080)))?.0, 640);
    assert_eq!(screen(Some((10000, 480)))?.0, 1920);
    Ok(())
}

#[test]
fn a_frame_commits_the_screen_s_width_as_the_display_now_has_it() -> Result<(), Error> {
    // rasterizer_screen_width_update takes this width for the screen's
    // bounds after each frame: 0 would leave the camera an empty viewport
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(SCREEN))?;
    assert_eq!(first(&game.call_with("commit", &[])?), Some(640));
    game.set_display(3840, 2160);
    assert_eq!(first(&game.call_with("commit", &[])?), Some(852));
    assert_eq!(first(&game.call_with("width", &[])?), Some(852));
    assert!(answers("env.halo_screen_commit"));
    Ok(())
}

#[test]
fn presenting_and_releasing_the_device_are_answered() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(SCREEN))?;
    game.call_with("present", &[])?;
    game.call_with("present", &[])?;
    assert_eq!(game.call_count("env.D3DDevice_Present"), 2);
    // the device is never really released (the Linux port's answer)
    assert_eq!(first(&game.call_with("release", &[])?), Some(1));
    for name in [
        "env.halo_screen_width",
        "env.halo_screen_scale",
        "env.D3DDevice_Present",
        "env.D3DDevice_Release",
    ] {
        assert!(answers(name), "{name}");
    }
    Ok(())
}

/// the visibility tests' imports, re-exported, with the result and its
/// time stamp at 0x100 and 0x108 marked as NONE
const VISIBILITY: &str = r#"
(module
  (import "env" "D3DDevice_BeginVisibilityTest" (func $begin))
  (import "env" "D3DDevice_EndVisibilityTest" (func $end (param i32) (result i32)))
  (import "env" "D3DDevice_GetVisibilityTestResult" (func $get (param i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "begin") (call $begin))
  (func (export "end") (param i32) (result i32) (call $end (local.get 0)))
  (func (export "get") (param i32 i32 i32) (result i32)
    (call $get (local.get 0) (local.get 1) (local.get 2)))
  (data (i32.const 0x100) "\ff\ff\ff\ff\00\00\00\00\ff\ff\ff\ff\ff\ff\ff\ff"))
"#;

#[test]
fn a_visibility_test_counts_no_samples_until_the_renderer_draws() -> Result<(), Error> {
    // the lens flares' occlusion asserts that the result was written
    // (rasterizer_xbox_widgets.c): with no query pending, the Linux port
    // writes 0 samples and a 0 time stamp
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(VISIBILITY))?;
    game.call_with("begin", &[])?;
    assert_eq!(first(&game.call_with("end", &[Val::I32(3)])?), Some(0));
    assert_eq!(
        first(&game.call_with("get", &[Val::I32(3), Val::I32(0x100), Val::I32(0x108)])?),
        Some(0)
    );
    assert_eq!(game.read(0x100, 16)?, vec![0; 16]);
    // null pointers are not written
    assert_eq!(
        first(&game.call_with("get", &[Val::I32(3), Val::I32(0), Val::I32(0)])?),
        Some(0)
    );
    for name in [
        "env.D3DDevice_BeginVisibilityTest",
        "env.D3DDevice_EndVisibilityTest",
        "env.D3DDevice_GetVisibilityTestResult",
    ] {
        assert!(answers(name), "{name}");
    }
    Ok(())
}
