//! The game's render calls, kept as one buffer per frame for Metal to draw
//! (the native port's A4). Each call that changes what is drawn or draws
//! (the device's `Set*` calls, `Clear`, `Begin`, the vertex data, `End`
//! and the `Draw*` calls) is kept as it was made: its arguments, and a copy
//! of what its pointer points at (the rectangles, the matrix, the viewport,
//! the indices, the constants, the pixel shader), since the game reuses that
//! memory before the frame is drawn. `Present` ends the frame and hands it
//! to [`Frames`].
//!
//! The SDK's inline functions write most render and texture states straight
//! into `D3D__RenderState` and `D3D__TextureState`, so no call says they
//! changed: before each draw whose states differ from the last kept, the
//! frame keeps both arrays whole ([`STATES`]). Vertex buffers, index
//! buffers and textures stay in the game's memory, named by address, as
//! the Xbox's GPU read them there.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, PoisonError};

use wasmtime::{Caller, Val};

use super::plugins::{self, Event};
use super::{with_space, Record, Shared};

/// The calls a frame keeps, by the index its commands give ([`Op`]).
pub const CALLS: &[&str] = &[
    "D3DDevice_Begin",
    "D3DDevice_BeginVisibilityTest",
    "D3DDevice_Clear",
    "D3DDevice_DrawIndexedVertices",
    "D3DDevice_DrawVertices",
    "D3DDevice_End",
    "D3DDevice_EndVisibilityTest",
    "D3DDevice_InsertCallback",
    "D3DDevice_LoadVertexShader",
    "D3DDevice_Present",
    "D3DDevice_SelectVertexShader",
    "D3DDevice_SetIndices",
    "D3DDevice_SetPalette",
    "D3DDevice_SetPixelShaderProgram",
    "D3DDevice_SetRenderStateNotInline",
    "D3DDevice_SetRenderState_BackFillMode",
    "D3DDevice_SetRenderState_CullMode",
    "D3DDevice_SetRenderState_Deferred",
    "D3DDevice_SetRenderState_DoNotCullUncompressed",
    "D3DDevice_SetRenderState_Dxt1NoiseEnable",
    "D3DDevice_SetRenderState_EdgeAntiAlias",
    "D3DDevice_SetRenderState_FillMode",
    "D3DDevice_SetRenderState_FogColor",
    "D3DDevice_SetRenderState_FrontFace",
    "D3DDevice_SetRenderState_LineWidth",
    "D3DDevice_SetRenderState_LogicOp",
    "D3DDevice_SetRenderState_MultiSampleAntiAlias",
    "D3DDevice_SetRenderState_MultiSampleMask",
    "D3DDevice_SetRenderState_MultiSampleType",
    "D3DDevice_SetRenderState_NormalizeNormals",
    "D3DDevice_SetRenderState_OcclusionCullEnable",
    "D3DDevice_SetRenderState_PSTextureModes",
    "D3DDevice_SetRenderState_RopZCmpAlwaysRead",
    "D3DDevice_SetRenderState_RopZRead",
    "D3DDevice_SetRenderState_ShadowFunc",
    "D3DDevice_SetRenderState_Simple",
    "D3DDevice_SetRenderState_StencilCullEnable",
    "D3DDevice_SetRenderState_StencilEnable",
    "D3DDevice_SetRenderState_StencilFail",
    "D3DDevice_SetRenderState_TextureFactor",
    "D3DDevice_SetRenderState_TwoSidedLighting",
    "D3DDevice_SetRenderState_VertexBlend",
    "D3DDevice_SetRenderState_YuvEnable",
    "D3DDevice_SetRenderState_ZBias",
    "D3DDevice_SetRenderState_ZEnable",
    "D3DDevice_SetRenderTarget",
    "D3DDevice_SetShaderConstantMode",
    "D3DDevice_SetStreamSource",
    "D3DDevice_SetTexture",
    "D3DDevice_SetTextureState_BorderColor",
    "D3DDevice_SetTextureState_BumpEnv",
    "D3DDevice_SetTextureState_ColorKeyColor",
    "D3DDevice_SetTextureState_Deferred",
    "D3DDevice_SetTextureState_TexCoordIndex",
    "D3DDevice_SetTransform",
    "D3DDevice_SetVertexData2f",
    "D3DDevice_SetVertexData2s",
    "D3DDevice_SetVertexData4f",
    "D3DDevice_SetVertexData4ub",
    "D3DDevice_SetVertexDataColor",
    "D3DDevice_SetVertexShader",
    "D3DDevice_SetVertexShaderConstant",
    "D3DDevice_SetViewport",
];

/// A command's call: an index into [`CALLS`], or [`STATES`].
pub type Op = u16;

/// The command that keeps the render and texture states whole: its data
/// is `D3D__RenderState`'s 144 words, then `D3D__TextureState`'s 4 × 32.
pub const STATES: Op = 0xFFFF;

/// the bytes of `D3D__RenderState` and `D3D__TextureState`
const RENDER_STATE_BYTES: u32 = 144 * 4;
const TEXTURE_STATE_BYTES: u32 = 4 * 32 * 4;

/// how many finished frames wait for the drawing side; older are dropped
const FRAMES_KEPT: usize = 2;
/// the words and bytes a frame may hold before it is dropped as runaway
/// (a game that never presents)
const FRAME_LIMIT: usize = 1 << 26;

/// what a call's pointer points at: the argument holding the pointer, and
/// its bytes, fixed or a count argument times a size
#[derive(Clone, Copy)]
enum Data {
    None,
    Fixed {
        pointer: usize,
        bytes: u32,
    },
    Counted {
        pointer: usize,
        count: usize,
        size: u32,
    },
}

/// what the call `name` points at
fn data_of(name: &str) -> Data {
    match name {
        // (count, rectangles, flags, color, z, stencil): D3DRECTs
        "D3DDevice_Clear" => Data::Counted {
            pointer: 1,
            count: 0,
            size: 16,
        },
        // (type, count, indices): 16-bit indices
        "D3DDevice_DrawIndexedVertices" => Data::Counted {
            pointer: 2,
            count: 1,
            size: 2,
        },
        // (register, constants, count): four floats each
        "D3DDevice_SetVertexShaderConstant" => Data::Counted {
            pointer: 1,
            count: 2,
            size: 16,
        },
        // (state, matrix): a D3DMATRIX
        "D3DDevice_SetTransform" => Data::Fixed {
            pointer: 1,
            bytes: 64,
        },
        // (viewport): a D3DVIEWPORT8
        "D3DDevice_SetViewport" => Data::Fixed {
            pointer: 0,
            bytes: 24,
        },
        // (definition): a D3DPIXELSHADERDEF's 60 words
        "D3DDevice_SetPixelShaderProgram" => Data::Fixed {
            pointer: 0,
            bytes: 240,
        },
        _ => Data::None,
    }
}

/// the calls that draw, before which the states are kept
fn draws(op: Op) -> bool {
    matches!(
        CALLS.get(usize::from(op)).copied(),
        Some("D3DDevice_DrawVertices" | "D3DDevice_DrawIndexedVertices" | "D3DDevice_End")
    )
}

/// The op of the import "module.name", if a frame keeps its calls.
pub(crate) fn op(name: &str) -> Option<Op> {
    let call = name.strip_prefix("env.")?;
    CALLS
        .iter()
        .position(|kept| *kept == call)
        .and_then(|index| Op::try_from(index).ok())
}

/// One frame's render calls, in the order the game made them.
#[derive(Clone, Debug, Default)]
pub struct Frame {
    /// the frames presented before this one
    pub number: u64,
    /// each command: its op, its argument count, then where its data
    /// starts and how long it is in `data`, then its arguments' bits
    words: Vec<u32>,
    /// the commands' data, one after another
    data: Vec<u8>,
}

/// One render call in a [`Frame`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Command<'a> {
    /// the call ([`CALLS`]) or [`STATES`]
    pub op: Op,
    /// its arguments' bits, as the game passed them (floats as their bits)
    pub args: &'a [u32],
    /// a copy of what its pointer pointed at, if anything
    pub data: &'a [u8],
}

impl Command<'_> {
    /// The call's name, or "states" for [`STATES`].
    #[must_use]
    pub fn name(&self) -> &'static str {
        if self.op == STATES {
            return "states";
        }
        CALLS.get(usize::from(self.op)).copied().unwrap_or("")
    }
}

impl Frame {
    /// The frame's commands, in order.
    pub fn commands(&self) -> impl Iterator<Item = Command<'_>> {
        let mut rest = self.words.as_slice();
        std::iter::from_fn(move || {
            let [op, count, start, len, tail @ ..] = rest else {
                return None;
            };
            let count = usize::try_from(*count).ok()?;
            let args = tail.get(..count)?;
            rest = tail.get(count..)?;
            let start = usize::try_from(*start).ok()?;
            let end = start.checked_add(usize::try_from(*len).ok()?)?;
            Some(Command {
                op: Op::try_from(*op).ok()?,
                args,
                data: self.data.get(start..end)?,
            })
        })
    }

    /// How many commands the frame holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.commands().count()
    }

    /// Whether the frame holds no commands.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    fn push(&mut self, op: Op, args: &[u32], data: &[u8]) {
        let (Ok(count), Ok(start), Ok(len)) = (
            u32::try_from(args.len()),
            u32::try_from(self.data.len()),
            u32::try_from(data.len()),
        ) else {
            return;
        };
        self.words.extend([u32::from(op), count, start, len]);
        self.words.extend_from_slice(args);
        self.data.extend_from_slice(data);
    }

    fn runaway(&self) -> bool {
        self.words.len() > FRAME_LIMIT || self.data.len() > FRAME_LIMIT
    }
}

/// the frame being kept, the frames finished and the states last kept
#[derive(Default)]
pub(crate) struct Recorder {
    current: Frame,
    finished: VecDeque<Frame>,
    presented: u64,
    dropped: u64,
    /// where `D3D__RenderState` and `D3D__TextureState` are
    states_at: Option<(u32, u32)>,
    /// the states the current frame last kept
    states: Vec<u8>,
}

/// the bits of an argument, floats as theirs
fn bits(value: &Val) -> u32 {
    match value {
        Val::I32(value) => value.cast_unsigned(),
        Val::F32(bits) => *bits,
        _ => 0,
    }
}

/// Keeps the call `op` with `params` in the current frame; `Present` ends
/// the frame.
pub(crate) fn record(
    caller: &mut Caller<'_, Record>,
    op: Op,
    params: &[Val],
) -> wasmtime::Result<()> {
    let args: Vec<u32> = params.iter().map(bits).collect();
    let name = CALLS.get(usize::from(op)).copied().unwrap_or("");
    let data = match data_of(name) {
        Data::None => None,
        Data::Fixed { pointer, bytes } => Some((pointer, Some(bytes))),
        Data::Counted {
            pointer,
            count,
            size,
        } => Some((
            pointer,
            args.get(count).and_then(|count| count.checked_mul(size)),
        )),
    };
    let data = match data {
        Some((pointer, len)) => {
            let address = args.get(pointer).copied().unwrap_or(0);
            let len = len.ok_or_else(|| wasmtime::format_err!("{name}'s data is too long"))?;
            if address == 0 || len == 0 {
                Vec::new()
            } else {
                with_space(caller, |space, _| space.read(address, len))?.ok_or_else(|| {
                    wasmtime::format_err!("{name}'s data at {address:#x} is outside the memory")
                })?
            }
        }
        None => Vec::new(),
    };
    let states = if draws(op) { states(caller)? } else { None };
    let shared = Arc::clone(&caller.data().shared);
    let mut recorder = shared.render.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(states) = states {
        if states != recorder.states {
            recorder.current.push(STATES, &[], &states);
            recorder.states = states;
        }
    }
    recorder.current.push(op, &args, &data);
    if name == "D3DDevice_Present" {
        let number = recorder.presented;
        let mut frame = std::mem::take(&mut recorder.current);
        frame.number = number;
        recorder.presented += 1;
        recorder.states.clear();
        // the render hook sees the frame before the app can take it, with
        // the recorder free while the plugins run
        drop(recorder);
        plugins::fire(caller, &Event::Render(&frame))?;
        let mut recorder = shared.render.lock().unwrap_or_else(PoisonError::into_inner);
        recorder.finished.push_back(frame);
        while recorder.finished.len() > FRAMES_KEPT {
            recorder.finished.pop_front();
            recorder.dropped += 1;
        }
    } else if recorder.current.runaway() {
        recorder.current = Frame::default();
        recorder.states.clear();
        recorder.dropped += 1;
    }
    Ok(())
}

/// the render and texture states as they are now, if the game exports them
fn states(caller: &mut Caller<'_, Record>) -> wasmtime::Result<Option<Vec<u8>>> {
    let known = caller
        .data()
        .shared
        .render
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .states_at;
    let at = match known {
        Some(at) => at,
        None => {
            let (Ok(render), Ok(texture)) = (
                super::d3d::exported_address(caller, "D3D__RenderState"),
                super::d3d::exported_address(caller, "D3D__TextureState"),
            ) else {
                return Ok(None);
            };
            caller
                .data()
                .shared
                .render
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .states_at = Some((render, texture));
            (render, texture)
        }
    };
    with_space(caller, |space, _| {
        let mut states = space.read(at.0, RENDER_STATE_BYTES)?;
        states.extend(space.read(at.1, TEXTURE_STATE_BYTES)?);
        Some(states)
    })
}

/// The frames a running game presents, to take on the drawing thread;
/// each clone is the same frames.
#[derive(Clone)]
pub struct Frames {
    shared: Arc<Shared>,
}

impl Frames {
    pub(crate) fn new(shared: Arc<Shared>) -> Self {
        Self { shared }
    }

    fn recorder(&self) -> std::sync::MutexGuard<'_, Recorder> {
        self.shared
            .render
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// The newest frame the game has presented and nobody has taken, if
    /// any; the older ones waiting are dropped.
    #[must_use]
    pub fn take(&self) -> Option<Frame> {
        let mut recorder = self.recorder();
        let newest = recorder.finished.pop_back();
        let older = recorder.finished.len();
        recorder.finished.clear();
        recorder.dropped += u64::try_from(older).unwrap_or(u64::MAX);
        newest
    }

    /// How many frames the game has presented.
    #[must_use]
    pub fn presented(&self) -> u64 {
        self.recorder().presented
    }

    /// How many presented frames were dropped untaken (or as runaway).
    #[must_use]
    pub fn dropped(&self) -> u64 {
        self.recorder().dropped
    }
}

/// the recorder's lock, shared with the game's threads
pub(crate) type Render = Mutex<Recorder>;
