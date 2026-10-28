//! Direct3D's resources the game creates for itself: palettes and vertex
//! and index buffers, with their locks, and every resource's register,
//! release and busy calls. Each follows the Linux port
//! (`port/linux/src/d3d8_resources.c`), with the layouts of the project's
//! own header (`port/include/xdk/xdk_d3d8.h`).
//!
//! A resource is a `D3DResource` (Common, Data and Lock) from the game's
//! malloc. A palette's or a vertex buffer's data is a block of the Xbox
//! window and its Data the block's physical address, as the Xbox's GPU
//! reads it; an index buffer's data is ordinary memory from malloc and its
//! Data virtual, since the CPU, not the GPU, reads indices.

use wasmtime::{Caller, Val};

use super::memory::{
    arg, call_game, give, window_block, Handler, ANYWHERE, PAGE_READWRITE, WINDOW_BASE,
};
use super::{with_space, Record};

/// `S_OK`
pub(crate) const S_OK: u32 = 0;
/// `E_OUTOFMEMORY`
pub(crate) const E_OUTOFMEMORY: u32 = 0x8007_000E;

/// a `D3DResource`'s Common, Data and Lock
const RESOURCE_BYTES: u32 = 12;
/// `Common`'s reference count, type and whether Direct3D created it
const D3DCOMMON_REFCOUNT_MASK: u32 = 0xFFFF;
const D3DCOMMON_TYPE_MASK: u32 = 0x0007_0000;
const D3DCOMMON_TYPE_VERTEXBUFFER: u32 = 0x0000_0000;
const D3DCOMMON_TYPE_INDEXBUFFER: u32 = 0x0001_0000;
const D3DCOMMON_TYPE_PALETTE: u32 = 0x0003_0000;
const D3DCOMMON_D3DCREATED: u32 = 0x0100_0000;
/// where a palette's `Common` keeps its `D3DPALETTESIZE`
const D3DPALETTE_COMMON_PALETTESIZE_SHIFT: u32 = 30;
/// a palette's colours, whatever its size: the Linux port's, and the most
/// any palette holds
const PALETTE_BYTES: u32 = 256 * 4;
/// how the window's blocks for resources are aligned
pub(crate) const D3DTEXTURE_ALIGNMENT: u32 = 128;

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "D3DDevice_CreatePalette" => create_palette,
        "D3DPalette_Lock" => palette_lock,
        "D3DDevice_CreateVertexBuffer" => create_vertex_buffer,
        "D3DVertexBuffer_Lock" => vertex_buffer_lock,
        "D3DDevice_CreateIndexBuffer" => create_index_buffer,
        "D3DResource_Register" => register,
        "D3DResource_Release" => release,
        "D3DResource_IsBusy" => is_busy,
        "D3DResource_BlockUntilNotBusy" => block_until_not_busy,
        _ => return None,
    };
    Some(handler)
}

/// where the data of a resource whose Data is `data` is, to the CPU: NULL
/// for none, else in the window
pub(crate) fn virtual_data(data: u32) -> u32 {
    if data == 0 {
        0
    } else {
        data | WINDOW_BASE
    }
}

/// the game's malloc of `size` bytes; 0 if it has none
pub(crate) fn malloc(caller: &mut Caller<'_, Record>, size: u32) -> wasmtime::Result<u32> {
    call_game(caller, "malloc", &[Val::I32(size.cast_signed())])
}

/// the game's free of `block`
fn free(caller: &mut Caller<'_, Record>, block: u32) -> wasmtime::Result<()> {
    call_game(caller, "free", &[Val::I32(block.cast_signed())]).map(drop)
}

/// where a resource's data comes from
#[derive(Clone, Copy)]
pub(crate) enum Data {
    /// a block of the window, its Data physical
    Window,
    /// a cleared block from malloc, its Data virtual
    Heap,
}

/// a new resource with `common` and `size` bytes of data, and the words
/// `after` its Lock (a texture's Format and Size), its address written at
/// `returned`; S_OK, or E_OUTOFMEMORY (with nothing kept) if there is no
/// room for either
pub(crate) fn create(
    caller: &mut Caller<'_, Record>,
    common: u32,
    size: u32,
    from: Data,
    after: &[u32],
    returned: u32,
) -> wasmtime::Result<u32> {
    let data = match from {
        Data::Window => window_block(caller, size, ANYWHERE, D3DTEXTURE_ALIGNMENT, PAGE_READWRITE)?
            .map(|block| block & !WINDOW_BASE),
        Data::Heap => match malloc(caller, size)? {
            0 => None,
            block => with_space(caller, |space, _| space.fill(block, size, 0))?.then_some(block),
        },
    };
    let Some(data) = data else {
        return Ok(E_OUTOFMEMORY);
    };
    let bytes = u32::try_from(after.len()).map_or(u32::MAX, |words| {
        RESOURCE_BYTES.saturating_add(words.saturating_mul(4))
    });
    let resource = malloc(caller, bytes)?;
    if resource == 0 {
        free_data(caller, common, data)?;
        return Ok(E_OUTOFMEMORY);
    }
    let written = with_space(caller, |space, _| {
        space.put(resource, common)
            && space.put(resource.wrapping_add(4), data)
            && space.put(resource.wrapping_add(8), 0)
            && (0_u32..)
                .zip(after)
                .all(|(index, &word)| space.put(resource.wrapping_add(12 + 4 * index), word))
            && space.put(returned, resource)
    })?;
    if !written {
        return Err(wasmtime::format_err!(
            "a resource's pointer {returned:#x} is outside the memory"
        ));
    }
    Ok(S_OK)
}

/// frees the data at `data` of a resource whose Common is `common`
fn free_data(caller: &mut Caller<'_, Record>, common: u32, data: u32) -> wasmtime::Result<()> {
    if common & D3DCOMMON_TYPE_MASK == D3DCOMMON_TYPE_INDEXBUFFER {
        free(caller, data)
    } else {
        if data != 0 {
            caller.data().window().free(virtual_data(data));
        }
        Ok(())
    }
}

/// writes `value` at the pointer `at`, which the game passed
pub(crate) fn put(caller: &mut Caller<'_, Record>, at: u32, value: u32) -> wasmtime::Result<()> {
    if with_space(caller, |space, _| space.put(at, value))? {
        Ok(())
    } else {
        Err(wasmtime::format_err!(
            "the pointer {at:#x} is outside the memory"
        ))
    }
}

/// a resource's word at `offset`
pub(crate) fn field(
    caller: &mut Caller<'_, Record>,
    resource: u32,
    offset: u32,
) -> wasmtime::Result<u32> {
    let address = resource.wrapping_add(offset);
    with_space(caller, |space, _| space.get(address))?
        .ok_or_else(|| wasmtime::format_err!("the resource {resource:#x} is outside the memory"))
}

/// `D3DDevice_CreatePalette(size, &palette)`: a palette of 256 colours,
/// whichever size it is (the Linux port's)
fn create_palette(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (size, returned) = (arg(params, 0), arg(params, 1));
    let common = D3DCOMMON_TYPE_PALETTE
        | D3DCOMMON_D3DCREATED
        | 1
        | (size << D3DPALETTE_COMMON_PALETTESIZE_SHIFT);
    let result = create(caller, common, PALETTE_BYTES, Data::Window, &[], returned)?;
    give(results, result);
    Ok(())
}

/// `D3DPalette_Lock(palette, &colors, flags)`: its colours, where they are
fn palette_lock(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let (palette, colors) = (arg(params, 0), arg(params, 1));
    let data = field(caller, palette, 4)?;
    put(caller, colors, virtual_data(data))
}

/// `D3DDevice_CreateVertexBuffer(length, usage, fvf, pool, &buffer)`: a
/// buffer of `length` bytes (one, if none)
fn create_vertex_buffer(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (length, returned) = (arg(params, 0), arg(params, 4));
    let common = D3DCOMMON_TYPE_VERTEXBUFFER | D3DCOMMON_D3DCREATED | 1;
    let result = create(caller, common, length.max(1), Data::Window, &[], returned)?;
    give(results, result);
    Ok(())
}

/// `D3DVertexBuffer_Lock(buffer, offset, size, &data, flags)`: the data
/// from `offset`, whatever the flags (nothing else reads it meanwhile)
fn vertex_buffer_lock(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let (buffer, offset, at) = (arg(params, 0), arg(params, 1), arg(params, 3));
    let data = virtual_data(field(caller, buffer, 4)?);
    let locked = if data == 0 {
        0
    } else {
        data.wrapping_add(offset)
    };
    put(caller, at, locked)
}

/// `D3DDevice_CreateIndexBuffer(length, usage, format, pool, &buffer)`
fn create_index_buffer(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (length, returned) = (arg(params, 0), arg(params, 4));
    let common = D3DCOMMON_TYPE_INDEXBUFFER | D3DCOMMON_D3DCREATED | 1;
    let result = create(caller, common, length.max(1), Data::Heap, &[], returned)?;
    give(results, result);
    Ok(())
}

/// `D3DResource_Register(resource, base)`: Data, an offset from `base`,
/// becomes the physical address it names
fn register(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let (resource, base) = (arg(params, 0), arg(params, 1));
    let data = field(caller, resource, 4)?;
    put(
        caller,
        resource.wrapping_add(4),
        base.wrapping_add(data) & !WINDOW_BASE,
    )
}

/// `D3DResource_Release(resource)`: one reference fewer; at the last, what
/// Direct3D created is freed. The references left.
fn release(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let resource = arg(params, 0);
    let common = field(caller, resource, 0)?;
    let count = (common & D3DCOMMON_REFCOUNT_MASK).saturating_sub(1);
    if common & D3DCOMMON_REFCOUNT_MASK != 0 {
        put(
            caller,
            resource,
            (common & !D3DCOMMON_REFCOUNT_MASK) | count,
        )?;
        if count == 0 && common & D3DCOMMON_D3DCREATED != 0 {
            let data = field(caller, resource, 4)?;
            free_data(caller, common, data)?;
            free(caller, resource)?;
        }
    }
    give(results, count);
    Ok(())
}

/// `D3DResource_IsBusy(resource)`: FALSE, since the GPU never holds the
/// game's memory
fn is_busy(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, 0);
    Ok(())
}

/// `D3DResource_BlockUntilNotBusy(resource)`
fn block_until_not_busy(
    _: &mut Caller<'_, Record>,
    _: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    Ok(())
}
