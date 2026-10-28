//! Direct3D's textures as the platform answers them (`opence_runtime`'s
//! textures module): created, locked and described as the Linux port does
//! (`port/linux/src/d3d8_resources.c`, with the layout of
//! `port/linux/src/xbox_textures.c`). A texture's header is Common, Data,
//! Lock, Format and Size; its data lives in the Xbox window, each mip level
//! after the one before, and a cube's faces each padded to 128 bytes.

use opence_runtime::{answers, Error, Runtime, Val};

/// the module: each import re-exported, memory that covers the window,
/// and a malloc that bumps from 0x1000 and counts its frees
const MODULE: &str = r#"
(module
  (import "env" "D3DDevice_CreateTexture" (func $texture (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (import "env" "D3DDevice_CreateCubeTexture" (func $cube (param i32 i32 i32 i32 i32 i32) (result i32)))
  (import "env" "D3DDevice_CreateVolumeTexture"
    (func $volume (param i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (import "env" "D3DTexture_LockRect" (func $lock (param i32 i32 i32 i32 i32)))
  (import "env" "D3DCubeTexture_LockRect" (func $cube_lock (param i32 i32 i32 i32 i32 i32)))
  (import "env" "D3DVolumeTexture_LockBox" (func $volume_lock (param i32 i32 i32 i32 i32)))
  (import "env" "D3DTexture_GetLevelDesc" (func $level_desc (param i32 i32 i32)))
  (import "env" "D3DTexture_GetSurfaceLevel" (func $surface_level (param i32 i32 i32) (result i32)))
  (import "env" "D3DSurface_GetDesc" (func $surface_desc (param i32 i32)))
  (import "env" "D3DSurface_LockRect" (func $surface_lock (param i32 i32 i32 i32)))
  (import "env" "D3DDevice_SetTexture" (func $set_texture (param i32 i32)))
  (import "env" "D3DDevice_SetPalette" (func $set_palette (param i32 i32)))
  (import "env" "D3DResource_Release" (func $release (param i32) (result i32)))
  (memory (export "memory") 40960)
  (global $next (mut i32) (i32.const 0x1000))
  (global $frees (mut i32) (i32.const 0))
  (func (export "malloc") (param $size i32) (result i32)
    (local $block i32)
    (local.set $block (global.get $next))
    (global.set $next
      (i32.and (i32.add (i32.add (global.get $next) (local.get $size)) (i32.const 15))
               (i32.const -16)))
    (local.get $block))
  (func (export "free") (param i32)
    (global.set $frees (i32.add (global.get $frees) (i32.const 1))))
  (func (export "frees") (result i32) (global.get $frees))
  (func (export "texture") (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (call $texture (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4) (local.get 5)
      (local.get 6)))
  (func (export "cube") (param i32 i32 i32 i32 i32 i32) (result i32)
    (call $cube (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4) (local.get 5)))
  (func (export "volume") (param i32 i32 i32 i32 i32 i32 i32 i32) (result i32)
    (call $volume (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4) (local.get 5)
      (local.get 6) (local.get 7)))
  (func (export "lock") (param i32 i32 i32 i32 i32)
    (call $lock (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "cube_lock") (param i32 i32 i32 i32 i32 i32)
    (call $cube_lock (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4) (local.get 5)))
  (func (export "volume_lock") (param i32 i32 i32 i32 i32)
    (call $volume_lock (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "level_desc") (param i32 i32 i32)
    (call $level_desc (local.get 0) (local.get 1) (local.get 2)))
  (func (export "surface_level") (param i32 i32 i32) (result i32)
    (call $surface_level (local.get 0) (local.get 1) (local.get 2)))
  (func (export "surface_desc") (param i32 i32) (call $surface_desc (local.get 0) (local.get 1)))
  (func (export "surface_lock") (param i32 i32 i32 i32)
    (call $surface_lock (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "set_texture") (param i32 i32) (call $set_texture (local.get 0) (local.get 1)))
  (func (export "set_palette") (param i32 i32) (call $set_palette (local.get 0) (local.get 1)))
  (func (export "release") (param i32) (result i32) (call $release (local.get 0))))
"#;

/// where the test has the imports return their pointers and descriptions
const RETURNED: u32 = 0x100;
/// where the test keeps a RECT or a D3DBOX
const AREA: u32 = 0x180;

/// the window, and how a physical address in it becomes virtual
const WINDOW_BASE: u32 = 0x8000_0000;

/// `Common`'s fields
const TYPE_TEXTURE: u32 = 0x0004_0000;
const TYPE_SURFACE: u32 = 0x0005_0000;
const D3DCREATED: u32 = 0x0100_0000;

/// `D3DFORMAT_*`'s fields, and the formats the tests use
const DMACHANNEL_A: u32 = 1;
const CUBEMAP: u32 = 4;
const A4R4G4B4: u32 = 0x04;
const A8R8G8B8: u32 = 0x06;
const DXT1: u32 = 0x0C;
const LIN_A8R8G8B8: u32 = 0x12;

/// `D3DRTYPE_SURFACE` and `D3DMULTISAMPLE_NONE`
const RTYPE_SURFACE: u32 = 1;
const MULTISAMPLE_NONE: u32 = 0x11;

fn game() -> Result<opence_runtime::Game, Error> {
    let bytes = wat::parse_str(MODULE).unwrap_or_default();
    Runtime::new(None)?.load(&bytes)
}

/// calls `name` with these unsigned arguments; its one result, or 0
fn call(game: &mut opence_runtime::Game, name: &str, args: &[u32]) -> Result<u32, Error> {
    let args: Vec<Val> = args.iter().map(|arg| Val::I32(arg.cast_signed())).collect();
    Ok(match game.call_with(name, &args)?.as_slice() {
        [Val::I32(value)] => value.cast_unsigned(),
        [] => 0,
        other => panic!("{name} returned {other:?}"),
    })
}

fn word(game: &opence_runtime::Game, address: u32) -> Result<u32, Error> {
    let bytes = game.read(u64::from(address), 4)?;
    let mut word = [0; 4];
    word.copy_from_slice(&bytes);
    Ok(u32::from_le_bytes(word))
}

/// `count` words from `address`
fn words<const N: usize>(game: &opence_runtime::Game, address: u32) -> Result<[u32; N], Error> {
    let mut words = [0; N];
    for (index, word_at) in words.iter_mut().enumerate() {
        *word_at = word(game, address + 4 * index as u32)?;
    }
    Ok(words)
}

/// writes these words from `address`
fn put(game: &mut opence_runtime::Game, address: u32, words: &[u32]) -> Result<(), Error> {
    let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
    game.write(u64::from(address), &bytes)
}

/// a new texture, by `name` and its arguments (the result pointer last)
fn create(game: &mut opence_runtime::Game, name: &str, args: &[u32]) -> Result<u32, Error> {
    let mut args = args.to_vec();
    args.push(RETURNED);
    assert_eq!(call(game, name, &args)?, 0, "{name}{args:?}");
    word(game, RETURNED)
}

#[test]
fn a_linear_texture_s_size_word_holds_its_pitch_and_size() -> Result<(), Error> {
    let mut game = game()?;
    // the screen's copy: 320 by 240, linear A8R8G8B8, as the rasterizer makes it
    let texture = create(&mut game, "texture", &[320, 240, 1, 1, LIN_A8R8G8B8, 0])?;
    let [common, data, lock, format, size] = words::<5>(&game, texture)?;
    assert_eq!(common, TYPE_TEXTURE | D3DCREATED | 1);
    assert!(data != 0 && data < WINDOW_BASE, "physical: {data:#x}");
    assert_eq!(lock, 0);
    assert_eq!(
        format,
        (LIN_A8R8G8B8 << 8) | (1 << 16) | (2 << 4) | DMACHANNEL_A
    );
    assert_eq!(size, ((1280 / 64 - 1) << 24) | (239 << 12) | 319);
    // locked whole, and at a rectangle: rows of the pitch, texels of 4 bytes
    call(&mut game, "lock", &[texture, 0, RETURNED, 0, 0])?;
    assert_eq!(words::<2>(&game, RETURNED)?, [1280, data | WINDOW_BASE]);
    put(&mut game, AREA, &[2, 3, 10, 10])?;
    call(&mut game, "lock", &[texture, 0, RETURNED, AREA, 0])?;
    assert_eq!(
        words::<2>(&game, RETURNED)?,
        [1280, (data | WINDOW_BASE) + 3 * 1280 + 2 * 4]
    );
    // its last row is in the window: 1280 by 240 bytes are its own
    game.write(u64::from(data | WINDOW_BASE) + 1280 * 240 - 4, &[0x5A; 4])?;
    Ok(())
}

#[test]
fn a_swizzled_texture_keeps_its_levels_one_after_another() -> Result<(), Error> {
    let mut game = game()?;
    // no levels asked for: all of them, down to 1 by 1
    let texture = create(&mut game, "texture", &[64, 32, 0, 0, A8R8G8B8, 0])?;
    let [_, data, _, format, size] = words::<5>(&game, texture)?;
    let levels = 7;
    assert_eq!(
        format,
        (5 << 24) | (6 << 20) | (levels << 16) | (A8R8G8B8 << 8) | (2 << 4) | DMACHANNEL_A
    );
    assert_eq!(size, 0);
    // level 2 is 16 by 8, after 64 by 32 and 32 by 16
    call(&mut game, "lock", &[texture, 2, RETURNED, 0, 0])?;
    assert_eq!(
        words::<2>(&game, RETURNED)?,
        [16 * 4, (data | WINDOW_BASE) + 8192 + 2048]
    );
    // and its description: 8 by 4 at level 3
    call(&mut game, "level_desc", &[texture, 3, RETURNED])?;
    assert_eq!(
        words::<7>(&game, RETURNED)?,
        [
            A8R8G8B8,
            RTYPE_SURFACE,
            0,
            8 * 4 * 4,
            MULTISAMPLE_NONE,
            8,
            4
        ]
    );
    // a texture asked for more levels than it has keeps its own
    let texture = create(&mut game, "texture", &[4, 4, 9, 0, A8R8G8B8, 0])?;
    assert_eq!((word(&game, texture + 12)? >> 16) & 0xF, 3);
    Ok(())
}

#[test]
fn a_texture_s_surface_is_its_level_with_the_texture_as_parent() -> Result<(), Error> {
    let mut game = game()?;
    let texture = create(&mut game, "texture", &[64, 32, 0, 0, A8R8G8B8, 0])?;
    let [_, data, _, format, _] = words::<5>(&game, texture)?;
    assert_eq!(
        call(&mut game, "surface_level", &[texture, 1, RETURNED])?,
        0
    );
    let surface = word(&game, RETURNED)?;
    let [common, surface_data, lock, surface_format, size, parent] = words::<6>(&game, surface)?;
    // not D3DCREATED: releasing it never frees the texture's data
    assert_eq!(common, TYPE_SURFACE | 1);
    assert_eq!(surface_data, data + 8192);
    assert_eq!(lock, 0);
    assert_eq!(
        surface_format,
        (format & 0x0000_FFFF) | (4 << 24) | (5 << 20) | (1 << 16)
    );
    assert_eq!(size, 0);
    assert_eq!(parent, texture);
    call(&mut game, "surface_desc", &[surface, RETURNED])?;
    assert_eq!(
        words::<7>(&game, RETURNED)?,
        [
            A8R8G8B8,
            RTYPE_SURFACE,
            0,
            32 * 4 * 16,
            MULTISAMPLE_NONE,
            32,
            16
        ]
    );
    call(&mut game, "surface_lock", &[surface, RETURNED, 0, 0])?;
    assert_eq!(
        words::<2>(&game, RETURNED)?,
        [32 * 4, (data | WINDOW_BASE) + 8192]
    );
    // and the screen copy's surface keeps its linear size word
    let screen = create(&mut game, "texture", &[320, 240, 1, 1, LIN_A8R8G8B8, 0])?;
    call(&mut game, "surface_level", &[screen, 0, RETURNED])?;
    let surface = word(&game, RETURNED)?;
    assert_eq!(word(&game, surface + 16)?, word(&game, screen + 16)?);
    assert_eq!(word(&game, surface + 4)?, word(&game, screen + 4)?);
    Ok(())
}

#[test]
fn a_compressed_texture_locks_by_blocks() -> Result<(), Error> {
    let mut game = game()?;
    let texture = create(&mut game, "texture", &[16, 16, 1, 0, DXT1, 0])?;
    let data = word(&game, texture + 4)? | WINDOW_BASE;
    // rows of 4 blocks of 8 bytes; a rectangle at (4, 8) is block (1, 2)
    put(&mut game, AREA, &[4, 8, 8, 12])?;
    call(&mut game, "lock", &[texture, 0, RETURNED, AREA, 0])?;
    assert_eq!(words::<2>(&game, RETURNED)?, [32, data + 2 * 32 + 8]);
    call(&mut game, "level_desc", &[texture, 0, RETURNED])?;
    assert_eq!(word(&game, RETURNED + 12)?, 32 * 4);
    Ok(())
}

#[test]
fn a_cube_s_faces_are_each_padded() -> Result<(), Error> {
    let mut game = game()?;
    let cube = create(&mut game, "cube", &[4, 1, 0, A4R4G4B4, 1])?;
    let [common, data, _, format, size] = words::<5>(&game, cube)?;
    assert_eq!(common, TYPE_TEXTURE | D3DCREATED | 1);
    assert_eq!(
        format,
        (2 << 24) | (2 << 20) | (1 << 16) | (A4R4G4B4 << 8) | (2 << 4) | CUBEMAP | DMACHANNEL_A
    );
    assert_eq!(size, 0);
    // each face's 32 bytes take 128
    call(&mut game, "cube_lock", &[cube, 2, 0, RETURNED, 0, 0])?;
    assert_eq!(
        words::<2>(&game, RETURNED)?,
        [8, (data | WINDOW_BASE) + 2 * 128]
    );
    game.write(u64::from(data | WINDOW_BASE) + 6 * 128 - 4, &[0x5A; 4])?;
    Ok(())
}

#[test]
fn a_volume_locks_by_rows_and_slices() -> Result<(), Error> {
    let mut game = game()?;
    let volume = create(&mut game, "volume", &[4, 4, 4, 1, 0, A4R4G4B4, 1])?;
    let [_, data, _, format, _] = words::<5>(&game, volume)?;
    assert_eq!(
        format,
        (2 << 28) | (2 << 24) | (2 << 20) | (1 << 16) | (A4R4G4B4 << 8) | (3 << 4) | DMACHANNEL_A
    );
    call(&mut game, "volume_lock", &[volume, 0, RETURNED, 0, 0])?;
    assert_eq!(words::<3>(&game, RETURNED)?, [8, 32, data | WINDOW_BASE]);
    // a box at (1, 2, 3)
    put(&mut game, AREA, &[1, 2, 2, 3, 3, 4])?;
    call(&mut game, "volume_lock", &[volume, 0, RETURNED, AREA, 0])?;
    assert_eq!(
        words::<3>(&game, RETURNED)?,
        [8, 32, (data | WINDOW_BASE) + 3 * 32 + 2 * 8 + 2]
    );
    Ok(())
}

#[test]
fn a_texture_with_no_data_locks_to_null() -> Result<(), Error> {
    let mut game = game()?;
    // a map's texture before it is registered: no Data
    let format = (2 << 24) | (2 << 20) | (1 << 16) | (A8R8G8B8 << 8) | (2 << 4) | DMACHANNEL_A;
    put(&mut game, 0x200, &[TYPE_TEXTURE | 1, 0, 0, format, 0])?;
    put(&mut game, RETURNED, &[0xFFFF_FFFF, 0xFFFF_FFFF])?;
    call(&mut game, "lock", &[0x200, 0, RETURNED, 0, 0])?;
    assert_eq!(words::<2>(&game, RETURNED)?, [16, 0]);
    Ok(())
}

#[test]
fn releasing_a_texture_frees_its_data_and_header() -> Result<(), Error> {
    let mut game = game()?;
    let texture = create(&mut game, "texture", &[64, 64, 1, 0, A8R8G8B8, 0])?;
    let data = word(&game, texture + 4)?;
    assert_eq!(call(&mut game, "release", &[texture])?, 0);
    assert_eq!(call(&mut game, "frees", &[])?, 1);
    // the window's block is the next texture's
    let texture = create(&mut game, "texture", &[64, 64, 1, 0, A8R8G8B8, 0])?;
    assert_eq!(word(&game, texture + 4)?, data);
    Ok(())
}

#[test]
fn the_device_keeps_each_stage_s_texture_and_palette() -> Result<(), Error> {
    let mut game = game()?;
    call(&mut game, "set_texture", &[1, 0x2000])?;
    call(&mut game, "set_palette", &[1, 0x3000])?;
    call(&mut game, "set_palette", &[0, 0x4000])?;
    // a stage past the fourth is ignored
    call(&mut game, "set_texture", &[4, 0x5000])?;
    assert_eq!(game.stage(0), Some((0, 0x4000)));
    assert_eq!(game.stage(1), Some((0x2000, 0x3000)));
    assert_eq!(game.stage(4), None);
    Ok(())
}

#[test]
fn the_platform_answers_the_textures() {
    for name in [
        "env.D3DDevice_CreateTexture",
        "env.D3DDevice_CreateCubeTexture",
        "env.D3DDevice_CreateVolumeTexture",
        "env.D3DTexture_LockRect",
        "env.D3DCubeTexture_LockRect",
        "env.D3DVolumeTexture_LockBox",
        "env.D3DTexture_GetLevelDesc",
        "env.D3DTexture_GetSurfaceLevel",
        "env.D3DSurface_GetDesc",
        "env.D3DSurface_LockRect",
        "env.D3DDevice_SetTexture",
        "env.D3DDevice_SetPalette",
    ] {
        assert!(answers(name), "{name}");
    }
}
