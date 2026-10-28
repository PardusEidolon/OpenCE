//! Direct3D's device, as the game creates it on its way to its main loop:
//! the device and its two surfaces (the back buffer and the depth buffer),
//! the default render and texture states, the device's capabilities, its
//! transforms and its viewport. Each follows the Linux port
//! (`port/linux/src/d3d8_gl.c`), with the values and layouts of the
//! project's own headers (`port/include/xdk/xdk_d3d8.h`, `xdk_pdb.h`).
//!
//! The render and texture states live in game.wasm (`D3D__RenderState`,
//! `D3D__TextureState`, `port/wasmtime/platform_data.c`), since the SDK's
//! inline functions read and write them directly; the device writes their
//! defaults through the addresses the module exports. The device and its
//! surfaces are blocks from the game's own malloc. A surface holds no
//! pixels: the frame is drawn by Metal, not into the game's memory.

use wasmtime::{Caller, Extern, Val};

use super::memory::{arg, call_game, give, Handler};
use super::{with_space, Record};

/// `S_OK`
const S_OK: u32 = 0;
/// `D3DERR_NOTFOUND`
const D3DERR_NOTFOUND: u32 = 0x8876_0866;

/// the transforms the device keeps (`D3DTS_MAX`), each a 4×4 matrix of
/// floats
const TRANSFORMS: usize = 10;
const MATRIX_BYTES: u32 = 64;
/// a `D3DSurface`: Common, Data, Lock, Format, Size and Parent
const SURFACE_BYTES: u32 = 24;
/// a `D3DVIEWPORT8`: X, Y, Width, Height, MinZ and MaxZ
const VIEWPORT_BYTES: u32 = 24;
/// a `D3DCAPS8`'s 53 words
const CAPS_WORDS: usize = 53;

/// the render states the device sets (`D3DRS_*`, and how many there are)
const RENDER_STATES: u32 = 144;
const D3DRS_ZFUNC: u32 = 57;
const D3DRS_ALPHAFUNC: u32 = 58;
const D3DRS_SRCBLEND: u32 = 62;
const D3DRS_DESTBLEND: u32 = 63;
const D3DRS_ZWRITEENABLE: u32 = 64;
const D3DRS_COLORWRITEENABLE: u32 = 67;
const D3DRS_STENCILZFAIL: u32 = 68;
const D3DRS_STENCILPASS: u32 = 69;
const D3DRS_STENCILFUNC: u32 = 70;
const D3DRS_STENCILMASK: u32 = 72;
const D3DRS_STENCILWRITEMASK: u32 = 73;
const D3DRS_BLENDOP: u32 = 74;
const D3DRS_FILLMODE: u32 = 119;
const D3DRS_ZENABLE: u32 = 123;
const D3DRS_STENCILFAIL: u32 = 125;
const D3DRS_FRONTFACE: u32 = 126;
const D3DRS_CULLMODE: u32 = 127;

/// the texture states the device sets, per stage (`D3DTSS_*`, and how many
/// a stage has and how many stages there are)
const TEXTURE_STATES: u32 = 32;
pub(crate) const STAGES: u32 = 4;
const D3DTSS_ADDRESSU: u32 = 10;
const D3DTSS_ADDRESSV: u32 = 11;
const D3DTSS_ADDRESSW: u32 = 12;
const D3DTSS_MAGFILTER: u32 = 13;
const D3DTSS_MINFILTER: u32 = 14;
const D3DTSS_MAXANISOTROPY: u32 = 18;

/// the values the defaults take
const D3DCMP_LESSEQUAL: u32 = 515;
const D3DCMP_ALWAYS: u32 = 519;
const D3DCOLORWRITEENABLE_ALL: u32 = 0x0101_0101;
const D3DBLEND_ZERO: u32 = 0;
const D3DBLEND_ONE: u32 = 1;
const D3DBLENDOP_ADD: u32 = 32774;
const D3DCULL_CCW: u32 = 2305;
const D3DFRONT_CW: u32 = 2304;
const D3DFILL_SOLID: u32 = 6914;
const D3DSTENCILOP_KEEP: u32 = 7680;
const D3DTADDRESS_WRAP: u32 = 1;
const D3DTEXF_POINT: u32 = 1;

/// the surfaces' formats
const D3DFMT_LIN_A8R8G8B8: u32 = 18;
const D3DFMT_LIN_D24S8: u32 = 46;
/// a surface's `Common`: its type, and the device's own reference
const D3DCOMMON_TYPE_SURFACE: u32 = 0x0005_0000;
/// how a surface's `Format` and `Size` words are packed
const D3DFORMAT_DMACHANNEL_A: u32 = 0x1;
const D3DFORMAT_DIMENSION_SHIFT: u32 = 4;
const D3DFORMAT_FORMAT_SHIFT: u32 = 8;
const D3DSIZE_HEIGHT_SHIFT: u32 = 12;
const D3DSIZE_PITCH_SHIFT: u32 = 24;
const D3DTEXTURE_PITCH_ALIGNMENT: u32 = 64;
/// both surfaces' texels are four bytes
const TEXEL_BYTES: u32 = 4;
/// the size a device takes when the game names none
const DEFAULT_SIZE: (u32, u32) = (640, 480);
/// the game's screen: always the Xbox's 480 lines, and 640 to 1920 columns
/// as the display's shape asks (`port/linux/src/d3d8_gl.c`)
const SCREEN_HEIGHT: u32 = 480;
const SCREEN_WIDTHS: (u32, u32) = (640, 1920);

/// The viewport the game last set (`D3DVIEWPORT8`), in pixels and depth.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Viewport {
    /// the left column
    pub x: u32,
    /// the top row
    pub y: u32,
    /// the width, in columns
    pub width: u32,
    /// the height, in rows
    pub height: u32,
    /// the nearest depth
    pub min_z: f32,
    /// the farthest depth
    pub max_z: f32,
}

/// the device, once the game has created it
#[derive(Default)]
pub(crate) struct Device {
    /// the device the game was given, and its back and depth buffers
    created: Option<Surfaces>,
    /// each transform, as the game's bytes
    transforms: Vec<[u8; MATRIX_BYTES as usize]>,
    /// the viewport
    pub(crate) viewport: Viewport,
    /// the display the screen is shown on, in pixels, if there is one
    pub(crate) display: Option<(u32, u32)>,
    /// the texture each stage draws with (`D3DDevice_SetTexture`)
    pub(crate) textures: [u32; STAGE_COUNT],
    /// the palette each stage's indexed texture reads (`D3DDevice_SetPalette`)
    pub(crate) palettes: [u32; STAGE_COUNT],
}

/// the texture stages, as an array's length
pub(crate) const STAGE_COUNT: usize = STAGES as usize;

impl Device {
    /// The screen's columns (its lines are always 480) and how many of the
    /// display's pixels each of its pixels takes, across and down: the
    /// display's shape at 480 lines, even, within 640 to 1920 columns; a
    /// display of another shape keeps the picture's and is letterboxed, at
    /// the smaller scale (`screen_mode_choose`).
    pub(crate) fn screen(&self) -> (u32, [f32; 2]) {
        let Some((display_width, display_height)) = self.display.filter(|&(w, h)| w > 0 && h > 0)
        else {
            return (SCREEN_WIDTHS.0, [1.0, 1.0]);
        };
        let wanted = (u64::from(SCREEN_HEIGHT) * u64::from(display_width)
            + u64::from(display_height / 2))
            / u64::from(display_height);
        let wanted = u32::try_from(wanted).unwrap_or(u32::MAX);
        let width = if wanted < SCREEN_WIDTHS.0 {
            SCREEN_WIDTHS.0
        } else if wanted > SCREEN_WIDTHS.1 {
            SCREEN_WIDTHS.1
        } else {
            wanted & !1
        };
        let mut scale = [
            display_width as f32 / width as f32,
            display_height as f32 / SCREEN_HEIGHT as f32,
        ];
        if wanted != width && wanted & !1 != width {
            let smaller = scale[0].min(scale[1]);
            scale = [smaller, smaller];
        }
        (width, scale)
    }
}

/// where the device and its surfaces are in the game's memory
#[derive(Clone, Copy)]
struct Surfaces {
    device: u32,
    back_buffer: u32,
    depth_buffer: u32,
}

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "Direct3DCreate8" => direct3d_create8,
        "Direct3D_SetPushBufferSize" => set_push_buffer_size,
        "Direct3D_CreateDevice" => create_device,
        "D3DDevice_GetBackBuffer" => get_back_buffer,
        "D3DDevice_GetDepthStencilSurface" => get_depth_stencil_surface,
        "D3DDevice_GetDeviceCaps" => get_device_caps,
        "D3DDevice_SetTransform" => set_transform,
        "D3DDevice_GetTransform" => get_transform,
        "D3DDevice_SetViewport" => set_viewport,
        "D3DDevice_Present" => present,
        "D3DDevice_Release" => release,
        "D3DDevice_BeginVisibilityTest" | "D3DDevice_EndVisibilityTest" => visibility_test,
        "D3DDevice_GetVisibilityTestResult" => visibility_test_result,
        "halo_screen_width" | "halo_screen_commit" => screen_width,
        "halo_screen_scale" => screen_scale,
        _ => return None,
    };
    Some(handler)
}

/// `Direct3DCreate8(version)`: Direct3D itself, which the game only checks
/// is there
fn direct3d_create8(
    _: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    give(results, 1);
    Ok(())
}

/// `Direct3D_SetPushBufferSize(size, kickoff)`: there is no push buffer;
/// each frame's calls go to Metal as one buffer
fn set_push_buffer_size(
    _: &mut Caller<'_, Record>,
    _: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    Ok(())
}

/// a surface's `Format` and `Size` words, at `width` by `height`
fn surface_words(format: u32, width: u32, height: u32) -> (u32, u32) {
    let alignment = D3DTEXTURE_PITCH_ALIGNMENT;
    let pitch = width.saturating_mul(TEXEL_BYTES).div_ceil(alignment).max(1);
    let format = (format << D3DFORMAT_FORMAT_SHIFT)
        | (2 << D3DFORMAT_DIMENSION_SHIFT)
        | D3DFORMAT_DMACHANNEL_A;
    let size = ((pitch - 1) << D3DSIZE_PITCH_SHIFT)
        | ((height.saturating_sub(1) & 0xFFF) << D3DSIZE_HEIGHT_SHIFT)
        | (width.saturating_sub(1) & 0xFFF);
    (format, size)
}

/// the address the module exports as the global `name`
fn exported_address(caller: &mut Caller<'_, Record>, name: &str) -> wasmtime::Result<u32> {
    let global = caller
        .get_export(name)
        .and_then(Extern::into_global)
        .ok_or_else(|| wasmtime::format_err!("the game exports no {name}"))?;
    match global.get(&mut *caller) {
        Val::I32(address) => Ok(address.cast_unsigned()),
        _ => Err(wasmtime::format_err!("{name} is not an address")),
    }
}

/// states as (index, value) pairs
type States = Vec<(u32, u32)>;

/// the render states and the texture states the device starts with
fn default_states() -> (States, States) {
    let render = vec![
        (D3DRS_ZENABLE, 1),
        (D3DRS_ZWRITEENABLE, 1),
        (D3DRS_ZFUNC, D3DCMP_LESSEQUAL),
        (D3DRS_COLORWRITEENABLE, D3DCOLORWRITEENABLE_ALL),
        (D3DRS_SRCBLEND, D3DBLEND_ONE),
        (D3DRS_DESTBLEND, D3DBLEND_ZERO),
        (D3DRS_BLENDOP, D3DBLENDOP_ADD),
        (D3DRS_CULLMODE, D3DCULL_CCW),
        (D3DRS_FRONTFACE, D3DFRONT_CW),
        (D3DRS_FILLMODE, D3DFILL_SOLID),
        (D3DRS_ALPHAFUNC, D3DCMP_ALWAYS),
        (D3DRS_STENCILFUNC, D3DCMP_ALWAYS),
        (D3DRS_STENCILMASK, 0xff),
        (D3DRS_STENCILWRITEMASK, 0xff),
        (D3DRS_STENCILFAIL, D3DSTENCILOP_KEEP),
        (D3DRS_STENCILZFAIL, D3DSTENCILOP_KEEP),
        (D3DRS_STENCILPASS, D3DSTENCILOP_KEEP),
    ];
    let per_stage = [
        (D3DTSS_ADDRESSU, D3DTADDRESS_WRAP),
        (D3DTSS_ADDRESSV, D3DTADDRESS_WRAP),
        (D3DTSS_ADDRESSW, D3DTADDRESS_WRAP),
        (D3DTSS_MAGFILTER, D3DTEXF_POINT),
        (D3DTSS_MINFILTER, D3DTEXF_POINT),
        (D3DTSS_MAXANISOTROPY, 1),
    ];
    let texture = (0..STAGES)
        .flat_map(|stage| {
            per_stage
                .iter()
                .map(move |&(index, value)| (stage * TEXTURE_STATES + index, value))
        })
        .collect();
    (render, texture)
}

/// a block of `size` bytes from the game's malloc
fn allocate(caller: &mut Caller<'_, Record>, size: u32) -> wasmtime::Result<u32> {
    match call_game(caller, "malloc", &[Val::I32(size.cast_signed())])? {
        0 => Err(wasmtime::format_err!(
            "the game's malloc has no room for the device"
        )),
        block => Ok(block),
    }
}

/// `Direct3D_CreateDevice(adapter, type, window, behavior, presentation,
/// device)`: the device at the presentation's back buffer size (640×480 if
/// it names none), with its surfaces, identity transforms, a viewport over
/// the whole screen and the default states; S_OK. A second call returns the
/// same device.
fn create_device(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (presentation, returned) = (arg(params, 4), arg(params, 5));
    let existing = caller.data().device().created;
    let surfaces = match existing {
        Some(surfaces) => surfaces,
        None => {
            let size = with_space(caller, |space, _| {
                (presentation != 0)
                    .then(|| {
                        space
                            .get(presentation)
                            .zip(space.get(presentation.wrapping_add(4)))
                    })
                    .flatten()
            })?;
            let (width, height) = match size {
                Some((width, height)) if width != 0 && height != 0 => (width, height),
                _ => DEFAULT_SIZE,
            };
            let surfaces = Surfaces {
                device: allocate(caller, 16)?,
                back_buffer: allocate(caller, SURFACE_BYTES)?,
                depth_buffer: allocate(caller, SURFACE_BYTES)?,
            };
            let render_state = exported_address(caller, "D3D__RenderState")?;
            let texture_state = exported_address(caller, "D3D__TextureState")?;
            let (render, texture) = default_states();
            let written = with_space(caller, |space, _| {
                let mut written = space.fill(surfaces.device, 16, 0);
                for (surface, format) in [
                    (surfaces.back_buffer, D3DFMT_LIN_A8R8G8B8),
                    (surfaces.depth_buffer, D3DFMT_LIN_D24S8),
                ] {
                    let (format, size) = surface_words(format, width, height);
                    written &= space.fill(surface, SURFACE_BYTES, 0)
                        && space.put(surface, D3DCOMMON_TYPE_SURFACE | 1)
                        && space.put(surface.wrapping_add(12), format)
                        && space.put(surface.wrapping_add(16), size);
                }
                written &= space.fill(render_state, RENDER_STATES * 4, 0)
                    && space.fill(texture_state, STAGES * TEXTURE_STATES * 4, 0);
                for (index, value) in render {
                    written &= space.put(render_state.wrapping_add(index * 4), value);
                }
                for (index, value) in texture {
                    written &= space.put(texture_state.wrapping_add(index * 4), value);
                }
                written
            })?;
            if !written {
                return Err(wasmtime::format_err!(
                    "the device's memory is outside the game's"
                ));
            }
            let mut device = caller.data().device();
            let mut identity = [0; MATRIX_BYTES as usize];
            for diagonal in [0, 5, 10, 15] {
                identity[diagonal * 4..diagonal * 4 + 4].copy_from_slice(&1.0_f32.to_le_bytes());
            }
            device.transforms = vec![identity; TRANSFORMS];
            device.viewport = Viewport {
                width,
                height,
                max_z: 1.0,
                ..Viewport::default()
            };
            device.created = Some(surfaces);
            surfaces
        }
    };
    with_space(caller, |space, _| space.put(returned, surfaces.device))?;
    give(results, S_OK);
    Ok(())
}

/// adds the caller's reference to the surface at `surface` and stores its
/// address at `result`
fn hand_out(caller: &mut Caller<'_, Record>, surface: u32, result: u32) -> wasmtime::Result<()> {
    with_space(caller, |space, _| {
        if let Some(common) = space.get(surface) {
            space.put(surface, common.wrapping_add(1));
        }
        space.put(result, surface);
    })
}

/// `D3DDevice_GetBackBuffer(index, type, result)`: the back buffer, with a
/// reference the caller releases
fn get_back_buffer(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let result = arg(params, 2);
    let created = caller.data().device().created;
    match created {
        Some(surfaces) => hand_out(caller, surfaces.back_buffer, result),
        None => with_space(caller, |space, _| {
            space.put(result, 0);
        }),
    }
}

/// `D3DDevice_GetDepthStencilSurface(result)`: the depth buffer, with a
/// reference the caller releases; D3DERR_NOTFOUND (and NULL) before there
/// is a device
fn get_depth_stencil_surface(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let result = arg(params, 0);
    let created = caller.data().device().created;
    match created {
        Some(surfaces) => {
            hand_out(caller, surfaces.depth_buffer, result)?;
            give(results, S_OK);
        }
        None => {
            with_space(caller, |space, _| space.put(result, 0))?;
            give(results, D3DERR_NOTFOUND);
        }
    }
    Ok(())
}

/// the `D3DCAPS8` the device reports, word by word: the Linux port's
fn caps() -> [u32; CAPS_WORDS] {
    let mut caps = [0; CAPS_WORDS];
    for (index, value) in [
        // DeviceType: D3DDEVTYPE_HAL
        (0, 1),
        // MaxTextureWidth, MaxTextureHeight, MaxVolumeExtent,
        // MaxTextureRepeat, MaxTextureAspectRatio, MaxAnisotropy
        (22, 4096),
        (23, 4096),
        (24, 512),
        (25, 8192),
        (26, 4096),
        (27, 4),
        // MaxTextureBlendStages, MaxSimultaneousTextures
        (37, 4),
        (38, 4),
        // MaxActiveLights, MaxVertexBlendMatrices
        (40, 8),
        (42, 4),
        // MaxPointSize
        (44, 64.0_f32.to_bits()),
        // MaxPrimitiveCount, MaxVertexIndex, MaxStreams, MaxStreamStride
        (45, 0xfffff),
        (46, 0xffff),
        (47, 16),
        (48, 255),
        // VertexShaderVersion 1.1, MaxVertexShaderConst, PixelShaderVersion
        // 1.1, MaxPixelShaderValue
        (49, 0xFFFE_0101),
        (50, 192),
        (51, 0xFFFF_0101),
        (52, 1.0_f32.to_bits()),
    ] {
        if let Some(word) = caps.get_mut(index) {
            *word = value;
        }
    }
    caps
}

/// `D3DDevice_GetDeviceCaps(caps)`: what the device can do
fn get_device_caps(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let address = arg(params, 0);
    let bytes: Vec<u8> = caps().iter().flat_map(|word| word.to_le_bytes()).collect();
    with_space(caller, |space, _| space.write(address, &bytes))?;
    Ok(())
}

/// the transform `state` names, if the device keeps one
fn transform_index(params: &[Val]) -> Option<usize> {
    usize::try_from(arg(params, 0))
        .ok()
        .filter(|&index| index < TRANSFORMS)
}

/// `D3DDevice_SetTransform(state, matrix)`: keeps the matrix (one past
/// D3DTS_MAX is ignored)
fn set_transform(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let Some(index) = transform_index(params) else {
        return Ok(());
    };
    let matrix = with_space(caller, |space, _| space.read(arg(params, 1), MATRIX_BYTES))?;
    if let Some(bytes) = matrix {
        let mut device = caller.data().device();
        let mut kept = [0; MATRIX_BYTES as usize];
        kept.copy_from_slice(&bytes);
        if let Some(transform) = device.transforms.get_mut(index) {
            *transform = kept;
        }
    }
    Ok(())
}

/// `D3DDevice_GetTransform(state, matrix)`: the matrix last set (the
/// identity before any)
fn get_transform(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let Some(index) = transform_index(params) else {
        return Ok(());
    };
    let kept = caller.data().device().transforms.get(index).copied();
    if let Some(bytes) = kept {
        with_space(caller, |space, _| space.write(arg(params, 1), &bytes))?;
    }
    Ok(())
}

/// `D3DDevice_SetViewport(viewport)`: keeps the viewport, for the frame
fn set_viewport(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let address = arg(params, 0);
    let words = with_space(caller, |space, _| {
        (0..VIEWPORT_BYTES / 4)
            .map(|index| space.get(address.wrapping_add(index * 4)))
            .collect::<Option<Vec<u32>>>()
    })?;
    if let Some([x, y, width, height, min_z, max_z]) = words.as_deref() {
        caller.data().device().viewport = Viewport {
            x: *x,
            y: *y,
            width: *width,
            height: *height,
            min_z: f32::from_bits(*min_z),
            max_z: f32::from_bits(*max_z),
        };
    }
    Ok(())
}

/// `D3DDevice_Present(source, destination, ...)`: the frame is done; the
/// render buffer each frame hands Metal is the native port's next task
fn present(_: &mut Caller<'_, Record>, _: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    Ok(())
}

/// `D3DDevice_Release()`: the device lives as long as the game, so a
/// reference always remains (the Linux port's answer)
fn release(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, 1);
    Ok(())
}

/// `D3DDevice_BeginVisibilityTest()` and `D3DDevice_EndVisibilityTest(index)`:
/// nothing is drawn yet to count, so there is nothing to start or end
fn visibility_test(
    _: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    give(results, 0);
    Ok(())
}

/// `D3DDevice_GetVisibilityTestResult(index, result, time_stamp)`: no
/// samples and a 0 time stamp, as the Linux port answers a test with no
/// query pending; the lens flares' occlusion asserts that the result is
/// written (`rasterizer_xbox_widgets.c`)
fn visibility_test_result(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (result, time_stamp) = (arg(params, 1), arg(params, 2));
    with_space(caller, |space, _| {
        if result != 0 {
            space.write(result, &[0; 4]);
        }
        if time_stamp != 0 {
            space.write(time_stamp, &[0; 8]);
        }
    })?;
    give(results, 0);
    Ok(())
}

/// `halo_screen_width()`: the screen's columns; and `halo_screen_commit()`,
/// the same after each frame, so that a change to the display applies from
/// the next (`rasterizer_screen_width_update`, the Linux port's
/// `port/linux/src/d3d8_gl.c`)
fn screen_width(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (width, _) = caller.data().device().screen();
    give(results, width);
    Ok(())
}

/// `halo_screen_scale()`: the display's pixels for each of the screen's,
/// the larger of the two ways (the screen effects' convolutions)
fn screen_scale(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (_, [across, down]) = caller.data().device().screen();
    if let Some(result) = results.first_mut() {
        *result = Val::F32(across.max(down).to_bits());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_surface_s_words_pack_its_pitch_and_size() {
        assert_eq!(
            surface_words(D3DFMT_LIN_A8R8G8B8, 1280, 720),
            ((18 << 8) | (2 << 4) | 1, (79 << 24) | (719 << 12) | 1279)
        );
        // a row that is not a whole number of 64s is padded to one
        assert_eq!(surface_words(D3DFMT_LIN_D24S8, 17, 1).1, (1 << 24) | 16);
    }

    #[test]
    fn the_caps_are_53_words_with_the_versions_last() {
        let caps = caps();
        assert_eq!(caps.len() * 4, 212);
        assert_eq!(caps.get(49..52), Some(&[0xFFFE_0101, 192, 0xFFFF_0101][..]));
    }
}
