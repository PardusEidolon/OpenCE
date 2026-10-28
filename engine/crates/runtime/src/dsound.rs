//! DirectSound, as the game's sound imports reach it. The streams are the
//! Linux port's (`port/linux/src/dsound_sdl.c`): each takes up to 64
//! packets, plays them out in turn, and completes each through the game's
//! callback, or its event; a flush completes them at once. The stream
//! objects are the module's own (`port/wasmtime/dsound.c`), each its
//! vtable's pointer alone, and this module keeps their state by address.
//!
//! Until the mixer reaches Core Audio, a stream plays in silence: a packet
//! finishes when its samples would have played at the stream's frequency,
//! and `DirectSoundDoWork` completes the packets that have. The device's,
//! the listener's and the buffers' calls answer as the Linux port's do.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use wasmtime::{Caller, Val};

use super::memory::{arg, call_game, give, Handler};
use super::{with_space, Record};

/// DirectSound's results
const DS_OK: u32 = 0;
const E_OUTOFMEMORY: u32 = 0x8007_000E;
const DSERR_INVALIDCALL: u32 = 0x8878_0032;

/// a packet's statuses, as it is completed
const STATUS_SUCCESS: u32 = 0;
const STATUS_PENDING: u32 = 0x8000_000A;
const STATUS_FLUSHED: u32 = 0x8000_4004;

/// how many packets a stream holds, as the Linux port's
const MAXIMUM_PACKETS: usize = 64;
/// `GetStatus`' flag: the stream takes another packet
const ACCEPT_INPUT_DATA: u32 = 1;
/// `XMEDIAINFO`'s flags: a fixed sample size, taken asynchronously
const FIXED_SAMPLE_SIZE_INPUT_ASYNC: u32 = 0x5;

/// the Xbox's ADPCM: its format tag, and a channel's block of 64 samples
const WAVE_FORMAT_XBOX_ADPCM: u16 = 0x69;
const ADPCM_BLOCK_BYTES: u32 = 36;
const ADPCM_BLOCK_SAMPLES: u64 = 64;

/// `DSSTREAMDESC`: its callback and context, and its format's pointer
const DESCRIPTION_BYTES: u32 = 24;
/// `WAVEFORMATEX` up to its bits a sample
const FORMAT_BYTES: u32 = 16;
/// `XMEDIAPACKET`
const PACKET_BYTES: u32 = 24;
/// `DSCAPS`: the free 2D and 3D voices and buffer entries, and the memory
const CAPS: [u32; 4] = [64, 64, 2047, 0];
/// the bytes the device and each buffer object take in the game's memory
const OBJECT_BYTES: u32 = 16;

const NANOSECONDS: u64 = 1_000_000_000;

/// DirectSound's state: the streams by address, the device, and the buffers
#[derive(Default)]
pub(crate) struct Sound {
    streams: BTreeMap<u32, Stream>,
    device: Option<u32>,
    buffers: BTreeSet<u32>,
}

/// a stream: its references, callback, format and packets
struct Stream {
    references: u32,
    callback: u32,
    context: u32,
    adpcm: bool,
    channels: u32,
    bytes_per_frame: u32,
    rate: u32,
    frequency: u32,
    packets: VecDeque<Packet>,
    /// when its last packet finishes, in nanoseconds since the game was loaded
    end: u64,
}

/// a packet a stream holds
#[derive(Clone, Copy)]
struct Packet {
    size: u32,
    completed: u32,
    status: u32,
    /// the callback's context, or the event to set
    context: u32,
    /// when it finishes
    end: u64,
}

/// a packet to complete, with its stream's callback
struct Completion {
    callback: u32,
    context: u32,
    packet: Packet,
}

impl Stream {
    /// the nanoseconds `size` bytes take to play
    fn duration(&self, size: u32) -> u64 {
        let frames = if self.adpcm {
            u64::from(size / (ADPCM_BLOCK_BYTES * self.channels)) * ADPCM_BLOCK_SAMPLES
        } else {
            u64::from(size / self.bytes_per_frame)
        };
        match self.frequency {
            0 => 0,
            frequency => frames.saturating_mul(NANOSECONDS) / u64::from(frequency),
        }
    }

    /// takes its packets, which finish at `before` or earlier (all if None)
    fn take(&mut self, before: Option<u64>) -> Vec<Completion> {
        let mut taken = Vec::new();
        while let Some(packet) = self.packets.front().copied() {
            if before.is_some_and(|now| packet.end > now) {
                break;
            }
            self.packets.pop_front();
            taken.push(Completion {
                callback: self.callback,
                context: self.context,
                packet,
            });
        }
        if before.is_none() {
            self.end = 0;
        }
        taken
    }
}

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "halo_dsound_stream_create" => stream_create,
        "halo_dsound_stream_add_ref" => stream_add_ref,
        "halo_dsound_stream_release" => stream_release,
        "halo_dsound_stream_get_info" => stream_get_info,
        "halo_dsound_stream_get_status" => stream_get_status,
        "halo_dsound_stream_process" => stream_process,
        "halo_dsound_stream_flush" => stream_flush,
        "halo_dsound_stream_discontinuity" => ok,
        "IDirectSoundStream_SetFrequency" => stream_set_frequency,
        "IDirectSoundStream_SetConeAngles"
        | "IDirectSoundStream_SetConeOrientation"
        | "IDirectSoundStream_SetConeOutsideVolume"
        | "IDirectSoundStream_SetI3DL2Source"
        | "IDirectSoundStream_SetMaxDistance"
        | "IDirectSoundStream_SetMinDistance"
        | "IDirectSoundStream_SetMixBinVolumes"
        | "IDirectSoundStream_SetMixBins"
        | "IDirectSoundStream_SetMode"
        | "IDirectSoundStream_SetPosition"
        | "IDirectSoundStream_SetVelocity"
        | "IDirectSoundStream_SetVolume"
        | "dsound_sdl_stream_set_stereo_position" => ok,
        "DirectSoundCreate" => create,
        "DirectSoundDoWork" => do_work,
        "DirectSoundGetStreamVoiceStatus" => stream_voice_status,
        "DirectSoundStopStream" => stream_flush,
        "DirectSoundUseFullHRTF" => ok,
        "IDirectSound_GetCaps" => get_caps,
        "IDirectSound_GetSpeakerConfig" => get_speaker_config,
        "IDirectSound_DownloadEffectsImage" => download_effects_image,
        "IDirectSound_CreateSoundBuffer" => create_sound_buffer,
        "DirectSoundCreateBuffer" => create_buffer,
        "IDirectSound_Release"
        | "IDirectSound_CommitDeferredSettings"
        | "IDirectSound_SetDistanceFactor"
        | "IDirectSound_SetI3DL2Listener"
        | "IDirectSound_SetMixBinHeadroom"
        | "IDirectSound_SetOrientation"
        | "IDirectSound_SetPosition"
        | "IDirectSound_SetRolloffFactor"
        | "IDirectSound_SetVelocity" => ok,
        "IDirectSoundBuffer_Release" => buffer_release,
        "IDirectSoundBuffer_Play"
        | "IDirectSoundBuffer_SetBufferData"
        | "IDirectSoundBuffer_SetCurrentPosition"
        | "IDirectSoundBuffer_SetLoopRegion"
        | "IDirectSoundBuffer_SetPitch"
        | "IDirectSoundBuffer_SetVolume"
        | "IDirectSoundBuffer_Stop" => ok,
        _ => return None,
    };
    Some(handler)
}

/// a call that needs nothing done: DS_OK
fn ok(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, DS_OK);
    Ok(())
}

/// the bytes at `address`, or an error naming `what`
fn read(
    caller: &mut Caller<'_, Record>,
    address: u32,
    len: u32,
    what: &str,
) -> wasmtime::Result<Vec<u8>> {
    with_space(caller, |space, _| space.read(address, len))?
        .ok_or_else(|| wasmtime::format_err!("the {what} at {address:#x} is outside the memory"))
}

/// writes `words` at `address`, or an error naming `what`
fn write(
    caller: &mut Caller<'_, Record>,
    address: u32,
    words: &[u32],
    what: &str,
) -> wasmtime::Result<()> {
    let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
    if with_space(caller, |space, _| space.write(address, &bytes))? {
        Ok(())
    } else {
        Err(wasmtime::format_err!(
            "the {what} at {address:#x} is outside the memory"
        ))
    }
}

/// the little-endian word at `offset` of `bytes`
fn word_at(bytes: &[u8], offset: usize) -> u32 {
    bytes
        .get(offset..offset + 4)
        .and_then(|word| word.try_into().ok())
        .map_or(0, u32::from_le_bytes)
}

/// the little-endian half-word at `offset` of `bytes`
fn half_at(bytes: &[u8], offset: usize) -> u16 {
    bytes
        .get(offset..offset + 2)
        .and_then(|half| half.try_into().ok())
        .map_or(0, u16::from_le_bytes)
}

/// completes each packet with `status`: its status and the size completed
/// written where it asks, then its stream's callback called, or its event
/// set. The streams are not held, since the callback refills its stream.
fn complete(
    caller: &mut Caller<'_, Record>,
    completions: Vec<Completion>,
    status: u32,
) -> wasmtime::Result<()> {
    for Completion {
        callback,
        context,
        packet,
    } in completions
    {
        let size = if status == STATUS_SUCCESS {
            packet.size
        } else {
            0
        };
        if packet.completed != 0 {
            write(caller, packet.completed, &[size], "packet's completed size")?;
        }
        if packet.status != 0 {
            write(caller, packet.status, &[status], "packet's status")?;
        }
        if callback != 0 {
            let args = [callback, context, packet.context, status]
                .map(|value| Val::I32(value.cast_signed()));
            call_game(caller, "halo_dsound_complete", &args)?;
        } else if packet.context != 0 {
            caller.data().shared.threads.set_event(packet.context);
        }
    }
    Ok(())
}

/// `halo_dsound_stream_create(stream, description)`: the stream at
/// `stream`, as `description` has it, with one reference
fn stream_create(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (stream, description) = (arg(params, 0), arg(params, 1));
    let description = read(
        caller,
        description,
        DESCRIPTION_BYTES,
        "stream's description",
    )?;
    let format = word_at(&description, 8);
    if format == 0 {
        give(results, DSERR_INVALIDCALL);
        return Ok(());
    }
    let format = read(caller, format, FORMAT_BYTES, "stream's format")?;
    let adpcm = half_at(&format, 0) == WAVE_FORMAT_XBOX_ADPCM;
    let channels = if half_at(&format, 2) == 2 { 2 } else { 1 };
    let rate = word_at(&format, 4);
    let bits = u32::from(half_at(&format, 14));
    let bytes_per_frame = (channels * bits / 8).max(1);
    caller.data().sound().streams.insert(
        stream,
        Stream {
            references: 1,
            callback: word_at(&description, 12),
            context: word_at(&description, 16),
            adpcm,
            channels,
            bytes_per_frame,
            rate,
            frequency: rate,
            packets: VecDeque::new(),
            end: 0,
        },
    );
    give(results, DS_OK);
    Ok(())
}

/// `AddRef(stream)`: its references now
fn stream_add_ref(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let references = caller
        .data()
        .sound()
        .streams
        .get_mut(&arg(params, 0))
        .map_or(0, |stream| {
            stream.references += 1;
            stream.references
        });
    give(results, references);
    Ok(())
}

/// `Release(stream)`: its references left; at none, its packets flushed
/// and the stream gone (the module frees its object)
fn stream_release(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let address = arg(params, 0);
    let (references, flushed) = {
        let mut sound = caller.data().sound();
        let Some(stream) = sound.streams.get_mut(&address) else {
            give(results, 0);
            return Ok(());
        };
        stream.references = stream.references.saturating_sub(1);
        let references = stream.references;
        let flushed = if references == 0 {
            sound
                .streams
                .remove(&address)
                .map(|mut stream| stream.take(None))
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        (references, flushed)
    };
    complete(caller, flushed, STATUS_FLUSHED)?;
    give(results, references);
    Ok(())
}

/// `GetInfo(stream, information)`: what a packet of it takes
fn stream_get_info(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (address, information) = (arg(params, 0), arg(params, 1));
    let input = caller.data().sound().streams.get(&address).map(|stream| {
        if stream.adpcm {
            ADPCM_BLOCK_BYTES * stream.channels
        } else {
            stream.bytes_per_frame
        }
    });
    let Some(input) = input else {
        give(results, DSERR_INVALIDCALL);
        return Ok(());
    };
    write(
        caller,
        information,
        &[FIXED_SAMPLE_SIZE_INPUT_ASYNC, input, 0, 0],
        "stream's information",
    )?;
    give(results, DS_OK);
    Ok(())
}

/// `GetStatus(stream, status)`: whether it takes another packet
fn stream_get_status(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (address, status) = (arg(params, 0), arg(params, 1));
    let count = caller
        .data()
        .sound()
        .streams
        .get(&address)
        .map(|stream| stream.packets.len());
    let Some(count) = count else {
        give(results, DSERR_INVALIDCALL);
        return Ok(());
    };
    let flags = if count < MAXIMUM_PACKETS {
        ACCEPT_INPUT_DATA
    } else {
        0
    };
    write(caller, status, &[flags], "stream's status")?;
    give(results, DS_OK);
    Ok(())
}

/// `Process(stream, input, output)`: the packet `input` queued behind the
/// stream's others, pending
fn stream_process(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (address, input) = (arg(params, 0), arg(params, 1));
    let bytes = read(caller, input, PACKET_BYTES, "packet")?;
    let now = caller.data().shared.threads.elapsed();
    let queued = {
        let mut sound = caller.data().sound();
        match sound.streams.get_mut(&address) {
            None => Err(DSERR_INVALIDCALL),
            Some(stream) if stream.packets.len() == MAXIMUM_PACKETS => Err(E_OUTOFMEMORY),
            Some(stream) => {
                let size = word_at(&bytes, 4);
                let end = stream.end.max(now).saturating_add(stream.duration(size));
                stream.end = end;
                let packet = Packet {
                    size,
                    completed: word_at(&bytes, 8),
                    status: word_at(&bytes, 12),
                    context: word_at(&bytes, 16),
                    end,
                };
                stream.packets.push_back(packet);
                Ok(packet)
            }
        }
    };
    match queued {
        Err(error) => give(results, error),
        Ok(packet) => {
            if packet.status != 0 {
                write(caller, packet.status, &[STATUS_PENDING], "packet's status")?;
            }
            if packet.completed != 0 {
                write(caller, packet.completed, &[0], "packet's completed size")?;
            }
            give(results, DS_OK);
        }
    }
    Ok(())
}

/// `Flush(stream)`, and `DirectSoundStopStream(stream)`: every packet it
/// holds completed at once, flushed
fn stream_flush(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let flushed = caller
        .data()
        .sound()
        .streams
        .get_mut(&arg(params, 0))
        .map(|stream| stream.take(None));
    let Some(flushed) = flushed else {
        give(results, DSERR_INVALIDCALL);
        return Ok(());
    };
    complete(caller, flushed, STATUS_FLUSHED)?;
    give(results, DS_OK);
    Ok(())
}

/// `SetFrequency(stream, frequency)`: 0 is the stream's own rate
fn stream_set_frequency(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let found = caller
        .data()
        .sound()
        .streams
        .get_mut(&arg(params, 0))
        .map(|stream| {
            stream.frequency = match arg(params, 1) {
                0 => stream.rate,
                frequency => frequency,
            };
        });
    give(
        results,
        if found.is_some() {
            DS_OK
        } else {
            DSERR_INVALIDCALL
        },
    );
    Ok(())
}

/// `DirectSoundGetStreamVoiceStatus(stream)`: whether it holds a packet
fn stream_voice_status(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let active = caller
        .data()
        .sound()
        .streams
        .get(&arg(params, 0))
        .is_some_and(|stream| !stream.packets.is_empty());
    give(results, u32::from(active));
    Ok(())
}

/// `DirectSoundDoWork()`: every packet that has played completed
fn do_work(caller: &mut Caller<'_, Record>, _: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    let now = caller.data().shared.threads.elapsed();
    let finished: Vec<Completion> = caller
        .data()
        .sound()
        .streams
        .values_mut()
        .flat_map(|stream| stream.take(Some(now)))
        .collect();
    complete(caller, finished, STATUS_SUCCESS)
}

/// a new object of `OBJECT_BYTES` in the game's memory
fn object(caller: &mut Caller<'_, Record>) -> wasmtime::Result<u32> {
    let address = call_game(caller, "malloc", &[Val::I32(OBJECT_BYTES.cast_signed())])?;
    if address == 0 {
        return Err(wasmtime::format_err!(
            "the game has no memory for a sound object"
        ));
    }
    write(caller, address, &[0; 4], "sound object")?;
    Ok(address)
}

/// `DirectSoundCreate(device, result, outer)`: the one device
fn create(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let result = arg(params, 1);
    let made = caller.data().sound().device;
    let device = match made {
        Some(device) => device,
        None => {
            let device = object(caller)?;
            caller.data().sound().device = Some(device);
            device
        }
    };
    write(caller, result, &[device], "device's result")?;
    give(results, DS_OK);
    Ok(())
}

/// `GetCaps(device, caps)`: the Linux port's voices and memory
fn get_caps(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    write(caller, arg(params, 1), &CAPS, "device's caps")?;
    give(results, DS_OK);
    Ok(())
}

/// `GetSpeakerConfig(device, config)`: stereo
fn get_speaker_config(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    write(caller, arg(params, 1), &[0], "speaker config")?;
    give(results, DS_OK);
    Ok(())
}

/// `DownloadEffectsImage(device, image, size, locations, description)`:
/// no effects image
fn download_effects_image(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let description = arg(params, 4);
    if description != 0 {
        write(caller, description, &[0], "effects image's description")?;
    }
    give(results, DS_OK);
    Ok(())
}

/// a buffer, written to `result`
fn new_buffer(
    caller: &mut Caller<'_, Record>,
    result: u32,
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let buffer = object(caller)?;
    caller.data().sound().buffers.insert(buffer);
    write(caller, result, &[buffer], "buffer's result")?;
    give(results, DS_OK);
    Ok(())
}

/// `CreateSoundBuffer(device, description, result, outer)`: a buffer that
/// plays nothing
fn create_sound_buffer(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    new_buffer(caller, arg(params, 2), results)
}

/// `DirectSoundCreateBuffer(description, result)`: a buffer that plays
/// nothing
fn create_buffer(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    new_buffer(caller, arg(params, 1), results)
}

/// `Release(buffer)`
fn buffer_release(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    caller.data().sound().buffers.remove(&arg(params, 0));
    give(results, DS_OK);
    Ok(())
}
