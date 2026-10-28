//! Direct3D's resources as the platform answers them
//! (`opence_runtime`'s resources module): palettes and vertex and index
//! buffers, created, locked, registered and released as the Linux port
//! does (`port/linux/src/d3d8_resources.c`). Their data lives in the Xbox
//! window and their `Data` holds its physical address; an index buffer's
//! is ordinary memory, and virtual.

use opence_runtime::{answers, Error, Runtime, Val};

/// the module: each import re-exported, memory that covers the window,
/// and a malloc that bumps from 0x1000 and counts its frees
const MODULE: &str = r#"
(module
  (import "env" "D3DDevice_CreatePalette" (func $palette (param i32 i32) (result i32)))
  (import "env" "D3DPalette_Lock" (func $palette_lock (param i32 i32 i32)))
  (import "env" "D3DDevice_CreateVertexBuffer" (func $vertices (param i32 i32 i32 i32 i32) (result i32)))
  (import "env" "D3DVertexBuffer_Lock" (func $vertices_lock (param i32 i32 i32 i32 i32)))
  (import "env" "D3DDevice_CreateIndexBuffer" (func $indices (param i32 i32 i32 i32 i32) (result i32)))
  (import "env" "D3DResource_Register" (func $register (param i32 i32)))
  (import "env" "D3DResource_Release" (func $release (param i32) (result i32)))
  (import "env" "D3DResource_IsBusy" (func $busy (param i32) (result i32)))
  (import "env" "D3DResource_BlockUntilNotBusy" (func $block (param i32)))
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
  (func (export "palette") (param i32 i32) (result i32) (call $palette (local.get 0) (local.get 1)))
  (func (export "palette_lock") (param i32 i32 i32)
    (call $palette_lock (local.get 0) (local.get 1) (local.get 2)))
  (func (export "vertices") (param i32 i32 i32 i32 i32) (result i32)
    (call $vertices (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "vertices_lock") (param i32 i32 i32 i32 i32)
    (call $vertices_lock (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "indices") (param i32 i32 i32 i32 i32) (result i32)
    (call $indices (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "register") (param i32 i32) (call $register (local.get 0) (local.get 1)))
  (func (export "release") (param i32) (result i32) (call $release (local.get 0)))
  (func (export "busy") (param i32) (result i32) (call $busy (local.get 0)))
  (func (export "block") (param i32) (call $block (local.get 0))))
"#;

/// where the test has the imports return their pointers
const RETURNED: u32 = 0x100;

/// the window, and how a physical address in it becomes virtual
const WINDOW_BASE: u32 = 0x8000_0000;

/// `Common`'s fields
const TYPE_VERTEXBUFFER: u32 = 0x0000_0000;
const TYPE_INDEXBUFFER: u32 = 0x0001_0000;
const TYPE_PALETTE: u32 = 0x0003_0000;
const D3DCREATED: u32 = 0x0100_0000;
/// `D3DPALETTE_256` and `D3DPALETTE_32`
const PALETTE_256: u32 = 0;
const PALETTE_32: u32 = 3;

/// the module, whose memory covers the window
fn game() -> Result<opence_runtime::Game, Error> {
    game_of(MODULE)
}

fn game_of(module: &str) -> Result<opence_runtime::Game, Error> {
    let bytes = wat::parse_str(module).unwrap_or_default();
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

/// a resource's Common, Data and Lock
fn fields(game: &opence_runtime::Game, resource: u32) -> Result<[u32; 3], Error> {
    Ok([
        word(game, resource)?,
        word(game, resource + 4)?,
        word(game, resource + 8)?,
    ])
}

#[test]
fn a_palette_holds_256_colours_in_the_window_whatever_its_size() -> Result<(), Error> {
    let mut game = game()?;
    for size in [PALETTE_256, PALETTE_32] {
        assert_eq!(call(&mut game, "palette", &[size, RETURNED])?, 0);
        let palette = word(&game, RETURNED)?;
        assert_ne!(palette, 0);
        let [common, data, lock] = fields(&game, palette)?;
        assert_eq!(common, TYPE_PALETTE | D3DCREATED | 1 | (size << 30));
        assert_ne!(data, 0);
        assert!(data < WINDOW_BASE, "physical: {data:#x}");
        assert_eq!(lock, 0);
        // the lock is the data's virtual address, and all 256 colours write
        call(&mut game, "palette_lock", &[palette, RETURNED, 0])?;
        let colours = word(&game, RETURNED)?;
        assert_eq!(colours, data | WINDOW_BASE);
        game.write(u64::from(colours), &[0x5A; 1024])?;
        assert_eq!(game.read(u64::from(colours) + 1020, 4)?, [0x5A; 4]);
    }
    Ok(())
}

#[test]
fn a_vertex_buffer_locks_at_an_offset_into_its_data() -> Result<(), Error> {
    let mut game = game()?;
    assert_eq!(call(&mut game, "vertices", &[4096, 0, 0, 0, RETURNED])?, 0);
    let buffer = word(&game, RETURNED)?;
    let [common, data, _] = fields(&game, buffer)?;
    assert_eq!(common, TYPE_VERTEXBUFFER | D3DCREATED | 1);
    assert!(data != 0 && data < WINDOW_BASE);
    call(&mut game, "vertices_lock", &[buffer, 64, 128, RETURNED, 0])?;
    assert_eq!(word(&game, RETURNED)?, (data | WINDOW_BASE) + 64);
    // a buffer of no length still has its byte
    assert_eq!(call(&mut game, "vertices", &[0, 0, 0, 0, RETURNED])?, 0);
    assert_ne!(fields(&game, word(&game, RETURNED)?)?[1], 0);
    // a buffer whose Data is NULL locks to NULL
    game.write(0x200, &[0; 12])?;
    game.write(u64::from(RETURNED), &[0xFF; 4])?;
    call(&mut game, "vertices_lock", &[0x200, 0, 0, RETURNED, 0])?;
    assert_eq!(word(&game, RETURNED)?, 0);
    Ok(())
}

#[test]
fn an_index_buffer_s_data_is_ordinary_memory() -> Result<(), Error> {
    let mut game = game()?;
    assert_eq!(call(&mut game, "indices", &[600, 0, 0, 0, RETURNED])?, 0);
    let buffer = word(&game, RETURNED)?;
    let [common, data, _] = fields(&game, buffer)?;
    assert_eq!(common, TYPE_INDEXBUFFER | D3DCREATED | 1);
    assert!(data != 0 && data < WINDOW_BASE);
    // cleared, as calloc's
    assert!(game
        .read(u64::from(data), 600)?
        .iter()
        .all(|&byte| byte == 0));
    Ok(())
}

#[test]
fn without_the_window_a_resource_is_out_of_memory() -> Result<(), Error> {
    let small = MODULE.replace(
        "(memory (export \"memory\") 40960)",
        "(memory (export \"memory\") 1)",
    );
    let mut game = game_of(&small)?;
    let out_of_memory = 0x8007_000E;
    assert_eq!(
        call(&mut game, "palette", &[PALETTE_256, RETURNED])?,
        out_of_memory
    );
    assert_eq!(
        call(&mut game, "vertices", &[64, 0, 0, 0, RETURNED])?,
        out_of_memory
    );
    Ok(())
}

#[test]
fn registering_makes_a_resource_s_offset_physical() -> Result<(), Error> {
    let mut game = game()?;
    // the decals' buffer: a virtual address in the window, and no base
    let mut resource = Vec::new();
    for field in [1, 0x8123_4000_u32, 0] {
        resource.extend_from_slice(&field.to_le_bytes());
    }
    game.write(0x200, &resource)?;
    call(&mut game, "register", &[0x200, 0])?;
    assert_eq!(word(&game, 0x204)?, 0x0123_4000);
    // a texture of a map: an offset into the block at the base
    game.write(0x204, &0x40_u32.to_le_bytes())?;
    call(&mut game, "register", &[0x200, 0x8200_0000])?;
    assert_eq!(word(&game, 0x204)?, 0x0200_0040);
    Ok(())
}

#[test]
fn releasing_frees_what_direct3d_created_at_its_last_reference() -> Result<(), Error> {
    let mut game = game()?;
    call(&mut game, "palette", &[PALETTE_256, RETURNED])?;
    let palette = word(&game, RETURNED)?;
    let data = fields(&game, palette)?[1];
    // a second reference, which the first release leaves
    game.write(
        u64::from(palette),
        &(TYPE_PALETTE | D3DCREATED | 2).to_le_bytes(),
    )?;
    assert_eq!(call(&mut game, "release", &[palette])?, 1);
    assert_eq!(call(&mut game, "frees", &[])?, 0);
    assert_eq!(call(&mut game, "release", &[palette])?, 0);
    // the header went back to malloc, and the data's page to the window:
    // the next palette is given it again
    assert_eq!(call(&mut game, "frees", &[])?, 1);
    call(&mut game, "palette", &[PALETTE_256, RETURNED])?;
    assert_eq!(fields(&game, word(&game, RETURNED)?)?[1], data);
    // an index buffer frees its data to malloc too
    call(&mut game, "indices", &[64, 0, 0, 0, RETURNED])?;
    let buffer = word(&game, RETURNED)?;
    assert_eq!(call(&mut game, "release", &[buffer])?, 0);
    assert_eq!(call(&mut game, "frees", &[])?, 3);
    // the game's own resources are never freed, and a count of 0 stays 0
    game.write(0x200, &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])?;
    assert_eq!(call(&mut game, "release", &[0x200])?, 0);
    assert_eq!(call(&mut game, "release", &[0x200])?, 0);
    assert_eq!(call(&mut game, "frees", &[])?, 3);
    Ok(())
}

#[test]
fn no_resource_is_ever_busy() -> Result<(), Error> {
    let mut game = game()?;
    assert_eq!(call(&mut game, "busy", &[0x200])?, 0);
    call(&mut game, "block", &[0x200])?;
    Ok(())
}

#[test]
fn the_platform_answers_the_resources() {
    for name in [
        "env.D3DDevice_CreatePalette",
        "env.D3DPalette_Lock",
        "env.D3DDevice_CreateVertexBuffer",
        "env.D3DVertexBuffer_Lock",
        "env.D3DDevice_CreateIndexBuffer",
        "env.D3DResource_Register",
        "env.D3DResource_Release",
        "env.D3DResource_IsBusy",
        "env.D3DResource_BlockUntilNotBusy",
    ] {
        assert!(answers(name), "{name}");
    }
}
