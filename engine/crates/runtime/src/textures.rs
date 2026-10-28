//! Direct3D's textures and their surfaces: created, locked and described as
//! the Linux port does (`port/linux/src/d3d8_resources.c`), with the
//! layout in memory of `port/linux/src/xbox_textures.c` and the header of
//! the project's own `port/include/xdk/xdk_pdb.h`; and the texture and
//! palette each stage draws with.
//!
//! A texture is a `D3DBaseTexture`: Common, Data and Lock, then Format and
//! Size. A swizzled texture's Format holds the log2 of its width, height
//! and depth and its levels, and its Size is 0; a linear one's Size holds
//! its pitch, width and height, and it has one level. The levels lie one
//! after another, and each face of a cube is padded to 128 bytes.

use wasmtime::{Caller, Val};

use super::d3d::STAGE_COUNT;
use super::memory::{arg, give, Handler};
use super::resources::{create, field, malloc, put, virtual_data, Data, E_OUTOFMEMORY, S_OK};
use super::Record;

/// `Common`'s type and creation fields
const D3DCOMMON_TYPE_TEXTURE: u32 = 0x0004_0000;
const D3DCOMMON_TYPE_SURFACE: u32 = 0x0005_0000;
const D3DCOMMON_D3DCREATED: u32 = 0x0100_0000;

/// `Format`'s fields
const D3DFORMAT_DMACHANNEL_A: u32 = 0x0000_0001;
const D3DFORMAT_CUBEMAP: u32 = 0x0000_0004;
const D3DFORMAT_DIMENSION_MASK: u32 = 0x0000_00F0;
const D3DFORMAT_DIMENSION_SHIFT: u32 = 4;
const D3DFORMAT_FORMAT_MASK: u32 = 0x0000_FF00;
const D3DFORMAT_FORMAT_SHIFT: u32 = 8;
const D3DFORMAT_MIPMAP_MASK: u32 = 0x000F_0000;
const D3DFORMAT_MIPMAP_SHIFT: u32 = 16;
const D3DFORMAT_USIZE_MASK: u32 = 0x00F0_0000;
const D3DFORMAT_USIZE_SHIFT: u32 = 20;
const D3DFORMAT_VSIZE_MASK: u32 = 0x0F00_0000;
const D3DFORMAT_VSIZE_SHIFT: u32 = 24;
const D3DFORMAT_PSIZE_MASK: u32 = 0xF000_0000;
const D3DFORMAT_PSIZE_SHIFT: u32 = 28;

/// `Size`'s fields
const D3DSIZE_WIDTH_MASK: u32 = 0x0000_0FFF;
const D3DSIZE_HEIGHT_MASK: u32 = 0x00FF_F000;
const D3DSIZE_HEIGHT_SHIFT: u32 = 12;
const D3DSIZE_PITCH_MASK: u32 = 0xFF00_0000;
const D3DSIZE_PITCH_SHIFT: u32 = 24;

/// how a linear texture's rows and a cube's faces are aligned
const D3DTEXTURE_PITCH_ALIGNMENT: u32 = 64;
const D3DTEXTURE_CUBEFACE_ALIGNMENT: u32 = 128;

/// a cube's faces
const CUBE_FACES: u32 = 6;

/// `D3DRTYPE_SURFACE` and `D3DMULTISAMPLE_NONE`, for a level's description
const D3DRTYPE_SURFACE: u32 = 1;
const D3DMULTISAMPLE_NONE: u32 = 0x0011;

/// a `D3DSurface`: a texture's header, then its Parent
const SURFACE_WORDS: usize = 3;

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "D3DDevice_CreateTexture" => create_texture,
        "D3DDevice_CreateCubeTexture" => create_cube_texture,
        "D3DDevice_CreateVolumeTexture" => create_volume_texture,
        "D3DTexture_LockRect" => texture_lock_rect,
        "D3DCubeTexture_LockRect" => cube_texture_lock_rect,
        "D3DVolumeTexture_LockBox" => volume_texture_lock_box,
        "D3DTexture_GetLevelDesc" => texture_get_level_desc,
        "D3DTexture_GetSurfaceLevel" => texture_get_surface_level,
        "D3DSurface_GetDesc" => surface_get_desc,
        "D3DSurface_LockRect" => surface_lock_rect,
        "D3DDevice_SetTexture" => set_texture,
        "D3DDevice_SetPalette" => set_palette,
        _ => return None,
    };
    Some(handler)
}

/// what a format's texels are: their bytes (a 4 by 4 block's, if
/// compressed), whether the format is linear, and whether it is compressed
#[derive(Clone, Copy)]
struct Texels {
    bytes: u32,
    linear: bool,
    compressed: bool,
}

/// the texels of the format `format` (`D3DFMT_*`); A8R8G8B8's for one the
/// game never uses
fn texels(format: u32) -> Texels {
    let (bytes, linear) = match format {
        0x00 | 0x01 | 0x0B | 0x19 => (1, false),
        0x13 | 0x1B | 0x1F => (1, true),
        0x02..=0x05 | 0x1A | 0x27..=0x29 | 0x2C | 0x2D | 0x32 | 0x38 | 0x39 => (2, false),
        0x10 | 0x11 | 0x16 | 0x17 | 0x1C | 0x1D | 0x20 | 0x24 | 0x25 | 0x30 | 0x31 | 0x35
        | 0x37 | 0x3D | 0x3E => (2, true),
        0x12 | 0x1E | 0x2E | 0x2F | 0x36 | 0x3F..=0x41 => (4, true),
        0x0C => (8, false),
        0x0E | 0x0F => (16, false),
        _ => (4, false),
    };
    Texels {
        bytes,
        linear,
        compressed: matches!(format, 0x0C | 0x0E | 0x0F),
    }
}

/// floor(log2(value)), and 0 for 0
fn floor_log2(value: u32) -> u32 {
    value.checked_ilog2().unwrap_or(0)
}

/// `base` at mip level `level`: halved each level, but never below 1
fn level_dimension(base: u32, level: u32) -> u32 {
    base.checked_shr(level).unwrap_or(0).max(1)
}

/// a texture as its Format and Size describe it (`xgpu_texture_describe`)
#[derive(Clone, Copy)]
struct Description {
    /// `D3DFMT_*`
    format: u32,
    texels: Texels,
    width: u32,
    height: u32,
    depth: u32,
    levels: u32,
    cube_map: bool,
    /// not swizzled, and addressed by rows of `pitch` bytes
    linear: bool,
    pitch: u32,
}

impl Description {
    /// the texture whose Format is `format` and Size `size`
    fn of(format: u32, size: u32) -> Self {
        let kind = (format & D3DFORMAT_FORMAT_MASK) >> D3DFORMAT_FORMAT_SHIFT;
        let texels = texels(kind);
        let mut description = if size != 0 {
            Self {
                format: kind,
                texels,
                width: (size & D3DSIZE_WIDTH_MASK) + 1,
                height: ((size & D3DSIZE_HEIGHT_MASK) >> D3DSIZE_HEIGHT_SHIFT) + 1,
                depth: 1,
                levels: 1,
                cube_map: false,
                linear: true,
                pitch: (((size & D3DSIZE_PITCH_MASK) >> D3DSIZE_PITCH_SHIFT) + 1)
                    * D3DTEXTURE_PITCH_ALIGNMENT,
            }
        } else {
            let width = 1 << ((format & D3DFORMAT_USIZE_MASK) >> D3DFORMAT_USIZE_SHIFT);
            Self {
                format: kind,
                texels,
                width,
                height: 1 << ((format & D3DFORMAT_VSIZE_MASK) >> D3DFORMAT_VSIZE_SHIFT),
                depth: 1 << ((format & D3DFORMAT_PSIZE_MASK) >> D3DFORMAT_PSIZE_SHIFT),
                levels: ((format & D3DFORMAT_MIPMAP_MASK) >> D3DFORMAT_MIPMAP_SHIFT).max(1),
                cube_map: false,
                linear: texels.linear,
                pitch: width * texels.bytes,
            }
        };
        description.cube_map = format & D3DFORMAT_CUBEMAP != 0;
        if (format & D3DFORMAT_DIMENSION_MASK) >> D3DFORMAT_DIMENSION_SHIFT != 3 {
            description.depth = 1;
        }
        description
    }

    /// the bytes of level `level`
    fn level_bytes(&self, level: u32) -> u32 {
        let width = level_dimension(self.width, level);
        let height = level_dimension(self.height, level);
        let depth = level_dimension(self.depth, level);
        if self.texels.compressed {
            width.div_ceil(4) * height.div_ceil(4) * self.texels.bytes * depth
        } else if self.linear {
            self.pitch.saturating_mul(height)
        } else {
            width * height * depth * self.texels.bytes
        }
    }

    /// where level `level` starts, from the face's first byte
    fn level_offset(&self, level: u32) -> u32 {
        (0..level.min(self.levels)).fold(0, |offset: u32, index| {
            offset.saturating_add(self.level_bytes(index))
        })
    }

    /// the bytes of one face, its levels included (a cube's padded)
    fn face_size(&self) -> u32 {
        let size = self.level_offset(self.levels);
        if self.cube_map {
            size.saturating_add(D3DTEXTURE_CUBEFACE_ALIGNMENT - 1)
                & !(D3DTEXTURE_CUBEFACE_ALIGNMENT - 1)
        } else {
            size
        }
    }

    /// the bytes from one row of level `level` to the next (of blocks, if
    /// compressed)
    fn level_pitch(&self, level: u32) -> u32 {
        if self.linear {
            self.pitch
        } else if self.texels.compressed {
            level_dimension(self.width, level).div_ceil(4) * self.texels.bytes
        } else {
            level_dimension(self.width, level) * self.texels.bytes
        }
    }

    /// the bytes of level `level` as one surface, as `D3DSURFACE_DESC`'s
    /// Size has it
    fn level_size(&self, level: u32) -> u32 {
        let height = level_dimension(self.height, level);
        let rows = if self.texels.compressed {
            height.div_ceil(4)
        } else {
            height
        };
        self.level_pitch(level).saturating_mul(rows)
    }
}

/// the description of the texture or surface at `resource`
fn describe(caller: &mut Caller<'_, Record>, resource: u32) -> wasmtime::Result<Description> {
    Ok(Description::of(
        field(caller, resource, 12)?,
        field(caller, resource, 16)?,
    ))
}

/// a linear texture's pitch: its rows' bytes, aligned
fn linear_pitch(width: u32, texels: Texels) -> u32 {
    width
        .saturating_mul(texels.bytes)
        .saturating_add(D3DTEXTURE_PITCH_ALIGNMENT - 1)
        & !(D3DTEXTURE_PITCH_ALIGNMENT - 1)
}

/// a new texture (`create_texture`): linear if its format is, with one
/// level, else swizzled with `levels` levels (all of them, for 0 or too
/// many); its address written at `returned`
fn new_texture(
    caller: &mut Caller<'_, Record>,
    (width, height, depth): (u32, u32, u32),
    levels: u32,
    format: u32,
    cube_map: bool,
    returned: u32,
) -> wasmtime::Result<u32> {
    let texels = texels(format);
    let most = floor_log2(width.max(height)) + 1;
    let levels = if levels == 0 || levels > most {
        most
    } else {
        levels
    };
    let kind = (format << D3DFORMAT_FORMAT_SHIFT) & D3DFORMAT_FORMAT_MASK;
    let (format_word, size_word) = if texels.linear {
        let pitch = linear_pitch(width, texels);
        (
            kind | (1 << D3DFORMAT_MIPMAP_SHIFT)
                | (2 << D3DFORMAT_DIMENSION_SHIFT)
                | D3DFORMAT_DMACHANNEL_A,
            (((pitch / D3DTEXTURE_PITCH_ALIGNMENT).saturating_sub(1) << D3DSIZE_PITCH_SHIFT)
                & D3DSIZE_PITCH_MASK)
                | ((height.saturating_sub(1) << D3DSIZE_HEIGHT_SHIFT) & D3DSIZE_HEIGHT_MASK)
                | (width.saturating_sub(1) & D3DSIZE_WIDTH_MASK),
        )
    } else {
        let dimension = if depth > 1 { 3 } else { 2 };
        (
            ((floor_log2(depth) << D3DFORMAT_PSIZE_SHIFT) & D3DFORMAT_PSIZE_MASK)
                | ((floor_log2(height) << D3DFORMAT_VSIZE_SHIFT) & D3DFORMAT_VSIZE_MASK)
                | ((floor_log2(width) << D3DFORMAT_USIZE_SHIFT) & D3DFORMAT_USIZE_MASK)
                | kind
                | ((levels << D3DFORMAT_MIPMAP_SHIFT) & D3DFORMAT_MIPMAP_MASK)
                | (dimension << D3DFORMAT_DIMENSION_SHIFT)
                | if cube_map { D3DFORMAT_CUBEMAP } else { 0 }
                | D3DFORMAT_DMACHANNEL_A,
            0,
        )
    };
    let faces = if cube_map { CUBE_FACES } else { 1 };
    let size = Description::of(format_word, size_word)
        .face_size()
        .saturating_mul(faces);
    let common = D3DCOMMON_TYPE_TEXTURE | D3DCOMMON_D3DCREATED | 1;
    create(
        caller,
        common,
        size,
        Data::Window,
        &[format_word, size_word],
        returned,
    )
}

/// `D3DDevice_CreateTexture(width, height, levels, usage, format, pool, &texture)`
fn create_texture(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let size = (arg(params, 0), arg(params, 1), 1);
    let result = new_texture(
        caller,
        size,
        arg(params, 2),
        arg(params, 4),
        false,
        arg(params, 6),
    )?;
    give(results, result);
    Ok(())
}

/// `D3DDevice_CreateCubeTexture(edge, levels, usage, format, pool, &texture)`
fn create_cube_texture(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let edge = arg(params, 0);
    let result = new_texture(
        caller,
        (edge, edge, 1),
        arg(params, 1),
        arg(params, 3),
        true,
        arg(params, 5),
    )?;
    give(results, result);
    Ok(())
}

/// `D3DDevice_CreateVolumeTexture(width, height, depth, levels, usage,
/// format, pool, &texture)`
fn create_volume_texture(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let size = (arg(params, 0), arg(params, 1), arg(params, 2));
    let result = new_texture(
        caller,
        size,
        arg(params, 3),
        arg(params, 5),
        false,
        arg(params, 7),
    )?;
    give(results, result);
    Ok(())
}

/// the words at `area`, a RECT's or a D3DBOX's, which the game passed
fn area<const N: usize>(caller: &mut Caller<'_, Record>, area: u32) -> wasmtime::Result<[u32; N]> {
    let mut words = [0; N];
    for (index, word) in (0_u32..).zip(words.iter_mut()) {
        *word = field(caller, area, 4 * index)?;
    }
    Ok(words)
}

/// locks face `face`'s level `level` of the texture or surface at
/// `resource` into the `D3DLOCKED_RECT` at `locked`, from the RECT at
/// `rectangle` if there is one (`lock_level`)
fn lock_level(
    caller: &mut Caller<'_, Record>,
    resource: u32,
    (face, level): (u32, u32),
    locked: u32,
    rectangle: u32,
) -> wasmtime::Result<()> {
    let description = describe(caller, resource)?;
    let pitch = description.level_pitch(level);
    let mut bits = virtual_data(field(caller, resource, 4)?);
    if bits != 0 {
        let start = face
            .wrapping_mul(description.face_size())
            .wrapping_add(description.level_offset(level));
        bits = bits.wrapping_add(start);
        if rectangle != 0 {
            let [left, top] = area::<2>(caller, rectangle)?;
            let width = level_dimension(description.width, level);
            bits = if description.texels.compressed {
                let block = pitch / width.div_ceil(4);
                bits.wrapping_add((top / 4).wrapping_mul(pitch))
                    .wrapping_add((left / 4).wrapping_mul(block))
            } else {
                let texel = pitch / width;
                bits.wrapping_add(top.wrapping_mul(pitch))
                    .wrapping_add(left.wrapping_mul(texel))
            };
        }
    }
    put(caller, locked, pitch)?;
    put(caller, locked.wrapping_add(4), bits)
}

/// `D3DTexture_LockRect(texture, level, &locked, rectangle, flags)`
fn texture_lock_rect(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    lock_level(
        caller,
        arg(params, 0),
        (0, arg(params, 1)),
        arg(params, 2),
        arg(params, 3),
    )
}

/// `D3DCubeTexture_LockRect(texture, face, level, &locked, rectangle, flags)`
fn cube_texture_lock_rect(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let face_level = (arg(params, 1), arg(params, 2));
    lock_level(
        caller,
        arg(params, 0),
        face_level,
        arg(params, 3),
        arg(params, 4),
    )
}

/// `D3DSurface_LockRect(surface, &locked, rectangle, flags)`
fn surface_lock_rect(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    lock_level(
        caller,
        arg(params, 0),
        (0, 0),
        arg(params, 1),
        arg(params, 2),
    )
}

/// `D3DVolumeTexture_LockBox(texture, level, &locked, box, flags)`: rows
/// and slices of level `level`, from the D3DBOX at `box` if there is one
fn volume_texture_lock_box(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let (volume, level, locked, area_at) = (
        arg(params, 0),
        arg(params, 1),
        arg(params, 2),
        arg(params, 3),
    );
    let description = describe(caller, volume)?;
    let row_pitch = description.level_pitch(level);
    let slice = row_pitch.saturating_mul(level_dimension(description.height, level));
    let mut bits = virtual_data(field(caller, volume, 4)?);
    if bits != 0 {
        bits = bits.wrapping_add(description.level_offset(level));
        if area_at != 0 {
            let [left, top, _, _, front] = area::<5>(caller, area_at)?;
            let texel = row_pitch / level_dimension(description.width, level);
            bits = bits
                .wrapping_add(front.wrapping_mul(slice))
                .wrapping_add(top.wrapping_mul(row_pitch))
                .wrapping_add(left.wrapping_mul(texel));
        }
    }
    put(caller, locked, row_pitch)?;
    put(caller, locked.wrapping_add(4), slice)?;
    put(caller, locked.wrapping_add(8), bits)
}

/// writes level `level` of the texture or surface at `resource` as the
/// `D3DSURFACE_DESC` at `at` (`describe_level`)
fn describe_level(
    caller: &mut Caller<'_, Record>,
    resource: u32,
    level: u32,
    at: u32,
) -> wasmtime::Result<()> {
    let description = describe(caller, resource)?;
    let words = [
        description.format,
        D3DRTYPE_SURFACE,
        0,
        description.level_size(level),
        D3DMULTISAMPLE_NONE,
        level_dimension(description.width, level),
        level_dimension(description.height, level),
    ];
    for (index, word) in (0_u32..).zip(words) {
        put(caller, at.wrapping_add(4 * index), word)?;
    }
    Ok(())
}

/// `D3DTexture_GetLevelDesc(texture, level, &description)`
fn texture_get_level_desc(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    describe_level(caller, arg(params, 0), arg(params, 1), arg(params, 2))
}

/// `D3DSurface_GetDesc(surface, &description)`
fn surface_get_desc(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    describe_level(caller, arg(params, 0), 0, arg(params, 1))
}

/// `D3DTexture_GetSurfaceLevel(texture, level, &surface)`: a new surface
/// over level `level`, with the texture as its Parent. It is not
/// D3DCREATED, as the Linux port's is not, so releasing it frees nothing.
fn texture_get_surface_level(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (texture, level, returned) = (arg(params, 0), arg(params, 1), arg(params, 2));
    let description = describe(caller, texture)?;
    let (data, format, size) = (
        field(caller, texture, 4)?,
        field(caller, texture, 12)?,
        field(caller, texture, 16)?,
    );
    let surface = malloc(caller, 4 * (SURFACE_WORDS as u32 + 3))?;
    if surface == 0 {
        give(results, E_OUTOFMEMORY);
        return Ok(());
    }
    let sizes =
        D3DFORMAT_USIZE_MASK | D3DFORMAT_VSIZE_MASK | D3DFORMAT_PSIZE_MASK | D3DFORMAT_MIPMAP_MASK;
    let width = level_dimension(description.width, level);
    let height = level_dimension(description.height, level);
    let words = [
        D3DCOMMON_TYPE_SURFACE | 1,
        data.wrapping_add(description.level_offset(level)),
        0,
        (format & !sizes)
            | ((floor_log2(width) << D3DFORMAT_USIZE_SHIFT) & D3DFORMAT_USIZE_MASK)
            | ((floor_log2(height) << D3DFORMAT_VSIZE_SHIFT) & D3DFORMAT_VSIZE_MASK)
            | (1 << D3DFORMAT_MIPMAP_SHIFT),
        size,
        texture,
    ];
    for (index, word) in (0_u32..).zip(words) {
        put(caller, surface.wrapping_add(4 * index), word)?;
    }
    put(caller, returned, surface)?;
    give(results, S_OK);
    Ok(())
}

/// sets `stage`'s entry of the device's `pick`ed table to `value`, if the
/// stage is one of the four
fn set_stage(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    pick: fn(&mut super::d3d::Device) -> &mut [u32; STAGE_COUNT],
) -> wasmtime::Result<()> {
    let (stage, value) = (arg(params, 0), arg(params, 1));
    let mut device = caller.data().device();
    if let Some(entry) = usize::try_from(stage)
        .ok()
        .and_then(|stage| pick(&mut device).get_mut(stage))
    {
        *entry = value;
    }
    Ok(())
}

/// `D3DDevice_SetTexture(stage, texture)`
fn set_texture(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    set_stage(caller, params, |device| &mut device.textures)
}

/// `D3DDevice_SetPalette(stage, palette)`
fn set_palette(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    set_stage(caller, params, |device| &mut device.palettes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_format_s_texels_follow_the_linux_port_s_table() {
        // A8R8G8B8, its linear form, DXT1 and DXT5, P8, and one unknown
        for (format, bytes, linear, compressed) in [
            (0x06, 4, false, false),
            (0x12, 4, true, false),
            (0x0C, 8, false, true),
            (0x0F, 16, false, true),
            (0x0B, 1, false, false),
            (0x42, 4, false, false),
        ] {
            let texels = texels(format);
            assert_eq!(
                (texels.bytes, texels.linear, texels.compressed),
                (bytes, linear, compressed),
                "{format:#x}"
            );
        }
    }

    #[test]
    fn a_level_is_never_smaller_than_one() {
        assert_eq!(level_dimension(64, 3), 8);
        assert_eq!(level_dimension(4, 5), 1);
        assert_eq!(level_dimension(4, 40), 1);
        assert_eq!(floor_log2(0), 0);
        assert_eq!(floor_log2(320), 8);
    }
}
