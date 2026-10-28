//! DirectSound as the platform answers the game's sound imports
//! (`opence_runtime`'s dsound module), as the Linux port does
//! (`port/linux/src/dsound_sdl.c`): a stream takes up to 64 packets, plays
//! them out in real time and completes each through the game's callback
//! (`halo_dsound_complete`), a flush completes them at once, and the
//! device's and buffers' calls answer as a device with nothing to say.

use std::time::{Duration, Instant};

use opence_runtime::{answers, Error, Runtime, Val};

/// the module: each import re-exported, a bump `malloc`, and a
/// `halo_dsound_complete` that logs its arguments at LOG
const MODULE: &str = r#"
(module
  (import "env" "halo_dsound_stream_create" (func $create (param i32 i32) (result i32)))
  (import "env" "halo_dsound_stream_add_ref" (func $add_ref (param i32) (result i32)))
  (import "env" "halo_dsound_stream_release" (func $release (param i32) (result i32)))
  (import "env" "halo_dsound_stream_get_info" (func $get_info (param i32 i32) (result i32)))
  (import "env" "halo_dsound_stream_get_status" (func $get_status (param i32 i32) (result i32)))
  (import "env" "halo_dsound_stream_process" (func $process (param i32 i32 i32) (result i32)))
  (import "env" "halo_dsound_stream_discontinuity" (func $discontinuity (param i32) (result i32)))
  (import "env" "halo_dsound_stream_flush" (func $flush (param i32) (result i32)))
  (import "env" "IDirectSoundStream_SetFrequency" (func $set_frequency (param i32 i32) (result i32)))
  (import "env" "DirectSoundCreate" (func $ds_create (param i32 i32 i32) (result i32)))
  (import "env" "DirectSoundDoWork" (func $do_work))
  (import "env" "DirectSoundGetStreamVoiceStatus" (func $voice_status (param i32) (result i32)))
  (import "env" "DirectSoundStopStream" (func $stop (param i32)))
  (import "env" "IDirectSound_GetCaps" (func $caps (param i32 i32) (result i32)))
  (import "env" "IDirectSound_GetSpeakerConfig" (func $speakers (param i32 i32) (result i32)))
  (import "env" "IDirectSound_DownloadEffectsImage" (func $effects (param i32 i32 i32 i32 i32) (result i32)))
  (import "env" "IDirectSound_CreateSoundBuffer" (func $buffer (param i32 i32 i32 i32) (result i32)))
  (import "env" "IDirectSoundBuffer_Play" (func $play (param i32 i32 i32 i32) (result i32)))
  (import "env" "IDirectSoundBuffer_Release" (func $buffer_release (param i32) (result i32)))
  (import "env" "CreateEventA" (func $event (param i32 i32 i32 i32) (result i32)))
  (import "env" "WaitForSingleObject" (func $wait (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (global $heap (mut i32) (i32.const 0x8000))
  (func (export "malloc") (param i32) (result i32)
    (local $at i32)
    (local.set $at (global.get $heap))
    (global.set $heap (i32.add (global.get $heap) (i32.and (i32.add (local.get 0) (i32.const 15)) (i32.const -16))))
    (local.get $at))
  (func (export "halo_dsound_complete") (param i32 i32 i32 i32)
    (local $at i32)
    (local.set $at (i32.add (i32.const 0x1010) (i32.shl (i32.load (i32.const 0x1000)) (i32.const 4))))
    (i32.store (local.get $at) (local.get 0))
    (i32.store offset=4 (local.get $at) (local.get 1))
    (i32.store offset=8 (local.get $at) (local.get 2))
    (i32.store offset=12 (local.get $at) (local.get 3))
    (i32.store (i32.const 0x1000) (i32.add (i32.load (i32.const 0x1000)) (i32.const 1))))
  (func (export "create") (param i32 i32) (result i32) (call $create (local.get 0) (local.get 1)))
  (func (export "add_ref") (param i32) (result i32) (call $add_ref (local.get 0)))
  (func (export "release") (param i32) (result i32) (call $release (local.get 0)))
  (func (export "get_info") (param i32 i32) (result i32) (call $get_info (local.get 0) (local.get 1)))
  (func (export "get_status") (param i32 i32) (result i32) (call $get_status (local.get 0) (local.get 1)))
  (func (export "process") (param i32 i32 i32) (result i32)
    (call $process (local.get 0) (local.get 1) (local.get 2)))
  (func (export "discontinuity") (param i32) (result i32) (call $discontinuity (local.get 0)))
  (func (export "flush") (param i32) (result i32) (call $flush (local.get 0)))
  (func (export "set_frequency") (param i32 i32) (result i32) (call $set_frequency (local.get 0) (local.get 1)))
  (func (export "ds_create") (param i32 i32 i32) (result i32)
    (call $ds_create (local.get 0) (local.get 1) (local.get 2)))
  (func (export "do_work") (call $do_work))
  (func (export "voice_status") (param i32) (result i32) (call $voice_status (local.get 0)))
  (func (export "stop") (param i32) (call $stop (local.get 0)))
  (func (export "caps") (param i32 i32) (result i32) (call $caps (local.get 0) (local.get 1)))
  (func (export "speakers") (param i32 i32) (result i32) (call $speakers (local.get 0) (local.get 1)))
  (func (export "effects") (param i32 i32 i32 i32 i32) (result i32)
    (call $effects (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "buffer") (param i32 i32 i32 i32) (result i32)
    (call $buffer (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "play") (param i32 i32 i32 i32) (result i32)
    (call $play (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "buffer_release") (param i32) (result i32) (call $buffer_release (local.get 0)))
  (func (export "event") (param i32 i32 i32 i32) (result i32)
    (call $event (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "wait") (param i32 i32) (result i32) (call $wait (local.get 0) (local.get 1))))
"#;

/// where the test lays out a stream's description, its format and packets
const DESCRIPTION: u32 = 0x300;
const FORMAT: u32 = 0x340;
const INFO: u32 = 0x380;
const STATUS: u32 = 0x3A0;
const PACKET: u32 = 0x400;
const PACKET_STATUS: u32 = 0x440;
const PACKET_COMPLETED: u32 = 0x444;
const SAMPLES: u32 = 0x2000;
/// the stream object, as the game's shim allocates it
const STREAM: u32 = 0x500;
const OTHER_STREAM: u32 = 0x510;
/// the callback and its context the description names
const CALLBACK: u32 = 7;
const CONTEXT: u32 = 3;
const PACKET_CONTEXT: u32 = 0x77;
/// the completions' log: a count, then (callback, context, packet's, status)
const LOG: u32 = 0x1000;
const OUT: u32 = 0x600;

const WAVE_FORMAT_PCM: u16 = 1;
const WAVE_FORMAT_XBOX_ADPCM: u16 = 0x69;
const ADPCM_BLOCK: u32 = 36;

const DS_OK: u32 = 0;
const E_OUTOFMEMORY: u32 = 0x8007_000E;
const DSERR_INVALIDCALL: u32 = 0x8878_0032;
const STATUS_SUCCESS: u32 = 0;
const STATUS_PENDING: u32 = 0x8000_000A;
const STATUS_FLUSHED: u32 = 0x8000_4004;
const ACCEPT_INPUT_DATA: u32 = 1;
const MAXIMUM_PACKETS: u32 = 64;
const WAIT_OBJECT_0: u32 = 0;

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
    let read = game.read(u64::from(address), 4)?;
    let mut bytes = [0; 4];
    bytes.copy_from_slice(&read);
    Ok(u32::from_le_bytes(bytes))
}

fn put(game: &mut opence_runtime::Game, address: u32, value: u32) -> Result<(), Error> {
    game.write(u64::from(address), &value.to_le_bytes())
}

/// a stream's description at DESCRIPTION and its `WAVEFORMATEX` at
/// FORMAT, completing through `callback`
fn describe(
    game: &mut opence_runtime::Game,
    tag: u16,
    channels: u16,
    rate: u32,
    callback: u32,
) -> Result<(), Error> {
    let bits: u16 = if tag == WAVE_FORMAT_PCM { 16 } else { 4 };
    let mut format = Vec::new();
    format.extend_from_slice(&tag.to_le_bytes());
    format.extend_from_slice(&channels.to_le_bytes());
    format.extend_from_slice(&rate.to_le_bytes());
    format.extend_from_slice(&0_u32.to_le_bytes());
    format.extend_from_slice(&0_u16.to_le_bytes());
    format.extend_from_slice(&bits.to_le_bytes());
    format.extend_from_slice(&0_u16.to_le_bytes());
    game.write(u64::from(FORMAT), &format)?;
    for (offset, value) in [
        (0, 0),
        (4, 4),
        (8, FORMAT),
        (12, callback),
        (16, CONTEXT),
        (20, 0),
    ] {
        put(game, DESCRIPTION + offset, value)?;
    }
    Ok(())
}

/// a packet at PACKET of `size` bytes, with or without the status and
/// completed size to report to, and `context` (or the event)
fn packet(
    game: &mut opence_runtime::Game,
    size: u32,
    report: bool,
    context: u32,
) -> Result<(), Error> {
    let (completed, status) = if report {
        (PACKET_COMPLETED, PACKET_STATUS)
    } else {
        (0, 0)
    };
    for (offset, value) in [
        (0, SAMPLES),
        (4, size),
        (8, completed),
        (12, status),
        (16, context),
        (20, 0),
    ] {
        put(game, PACKET + offset, value)?;
    }
    put(game, PACKET_STATUS, 0xFFFF_FFFF)?;
    put(game, PACKET_COMPLETED, 0xFFFF_FFFF)
}

/// the completions logged so far: (callback, context, packet's, status)
fn completions(game: &opence_runtime::Game) -> Result<Vec<[u32; 4]>, Error> {
    let count = word(game, LOG)?;
    (0..count)
        .map(|index| {
            let at = LOG + 0x10 + index * 16;
            Ok([
                word(game, at)?,
                word(game, at + 4)?,
                word(game, at + 8)?,
                word(game, at + 12)?,
            ])
        })
        .collect()
}

/// an ADPCM stereo stream at STREAM, at 22050 Hz
fn stereo_stream(game: &mut opence_runtime::Game) -> Result<(), Error> {
    describe(game, WAVE_FORMAT_XBOX_ADPCM, 2, 22050, CALLBACK)?;
    assert_eq!(call(game, "create", &[STREAM, DESCRIPTION])?, DS_OK);
    Ok(())
}

#[test]
fn the_platform_answers_directsound() {
    for name in [
        "env.halo_dsound_stream_create",
        "env.halo_dsound_stream_process",
        "env.halo_dsound_stream_flush",
        "env.DirectSoundCreate",
        "env.DirectSoundDoWork",
        "env.DirectSoundGetStreamVoiceStatus",
        "env.IDirectSound_GetCaps",
        "env.IDirectSound_SetOrientation",
        "env.IDirectSoundBuffer_Play",
        "env.IDirectSoundStream_SetPosition",
        "env.dsound_sdl_stream_set_stereo_position",
    ] {
        assert!(answers(name), "{name}");
    }
}

#[test]
fn a_stream_says_what_it_takes() -> Result<(), Error> {
    let mut game = game()?;
    stereo_stream(&mut game)?;
    assert_eq!(call(&mut game, "get_info", &[STREAM, INFO])?, DS_OK);
    assert_eq!(
        word(&game, INFO)?,
        0x5,
        "a fixed sample size, taken asynchronously"
    );
    assert_eq!(word(&game, INFO + 4)?, ADPCM_BLOCK * 2, "a block a channel");
    assert_eq!(word(&game, INFO + 8)?, 0);

    describe(&mut game, WAVE_FORMAT_PCM, 1, 44100, CALLBACK)?;
    assert_eq!(
        call(&mut game, "create", &[OTHER_STREAM, DESCRIPTION])?,
        DS_OK
    );
    assert_eq!(call(&mut game, "get_info", &[OTHER_STREAM, INFO])?, DS_OK);
    assert_eq!(word(&game, INFO + 4)?, 2, "a 16-bit sample");

    assert_eq!(call(&mut game, "get_status", &[STREAM, STATUS])?, DS_OK);
    assert_eq!(word(&game, STATUS)?, ACCEPT_INPUT_DATA);
    Ok(())
}

#[test]
fn a_stream_holds_sixty_four_packets() -> Result<(), Error> {
    let mut game = game()?;
    stereo_stream(&mut game)?;
    packet(&mut game, ADPCM_BLOCK * 2 * 1000, true, PACKET_CONTEXT)?;
    for _ in 0..MAXIMUM_PACKETS {
        assert_eq!(call(&mut game, "process", &[STREAM, PACKET, 0])?, DS_OK);
    }
    assert_eq!(word(&game, PACKET_STATUS)?, STATUS_PENDING);
    assert_eq!(word(&game, PACKET_COMPLETED)?, 0);
    assert_eq!(call(&mut game, "get_status", &[STREAM, STATUS])?, DS_OK);
    assert_eq!(word(&game, STATUS)?, 0, "a full stream takes no more");
    assert_eq!(
        call(&mut game, "process", &[STREAM, PACKET, 0])?,
        E_OUTOFMEMORY
    );
    assert_eq!(call(&mut game, "voice_status", &[STREAM])?, 1);
    Ok(())
}

#[test]
fn a_flush_completes_every_packet_at_once() -> Result<(), Error> {
    let mut game = game()?;
    stereo_stream(&mut game)?;
    packet(&mut game, ADPCM_BLOCK * 2 * 1000, true, PACKET_CONTEXT)?;
    for _ in 0..3 {
        assert_eq!(call(&mut game, "process", &[STREAM, PACKET, 0])?, DS_OK);
    }
    assert_eq!(call(&mut game, "flush", &[STREAM])?, DS_OK);
    assert_eq!(
        completions(&game)?,
        vec![[CALLBACK, CONTEXT, PACKET_CONTEXT, STATUS_FLUSHED]; 3]
    );
    assert_eq!(word(&game, PACKET_STATUS)?, STATUS_FLUSHED);
    assert_eq!(
        word(&game, PACKET_COMPLETED)?,
        0,
        "nothing of it was played"
    );
    assert_eq!(call(&mut game, "voice_status", &[STREAM])?, 0);

    // stopping the stream is a flush too
    assert_eq!(call(&mut game, "process", &[STREAM, PACKET, 0])?, DS_OK);
    call(&mut game, "stop", &[STREAM])?;
    assert_eq!(completions(&game)?.len(), 4);
    assert_eq!(call(&mut game, "voice_status", &[STREAM])?, 0);
    Ok(())
}

#[test]
fn a_packet_plays_out_in_real_time() -> Result<(), Error> {
    let mut game = game()?;
    stereo_stream(&mut game)?;
    // ten blocks of 64 samples at 22050 Hz: 29 ms
    let size = ADPCM_BLOCK * 2 * 10;
    packet(&mut game, size, true, PACKET_CONTEXT)?;
    let start = Instant::now();
    assert_eq!(call(&mut game, "process", &[STREAM, PACKET, 0])?, DS_OK);
    call(&mut game, "do_work", &[])?;
    assert!(completions(&game)?.is_empty(), "it has not played yet");
    while completions(&game)?.is_empty() && start.elapsed() < Duration::from_secs(5) {
        std::thread::sleep(Duration::from_millis(2));
        call(&mut game, "do_work", &[])?;
    }
    assert!(start.elapsed() >= Duration::from_millis(29));
    assert_eq!(
        completions(&game)?,
        vec![[CALLBACK, CONTEXT, PACKET_CONTEXT, STATUS_SUCCESS]]
    );
    assert_eq!(word(&game, PACKET_STATUS)?, STATUS_SUCCESS);
    assert_eq!(word(&game, PACKET_COMPLETED)?, size);
    assert_eq!(call(&mut game, "voice_status", &[STREAM])?, 0);
    Ok(())
}

#[test]
fn a_stream_without_a_callback_sets_the_packets_event() -> Result<(), Error> {
    let mut game = game()?;
    describe(&mut game, WAVE_FORMAT_PCM, 1, 44100, 0)?;
    assert_eq!(call(&mut game, "create", &[STREAM, DESCRIPTION])?, DS_OK);
    let event = call(&mut game, "event", &[0, 1, 0, 0])?;
    assert_ne!(event, 0);
    packet(&mut game, 2, false, event)?;
    assert_eq!(call(&mut game, "process", &[STREAM, PACKET, 0])?, DS_OK);
    assert_eq!(call(&mut game, "flush", &[STREAM])?, DS_OK);
    assert!(completions(&game)?.is_empty());
    assert_eq!(call(&mut game, "wait", &[event, 0])?, WAIT_OBJECT_0);
    Ok(())
}

#[test]
fn a_stream_lives_while_it_is_referred_to() -> Result<(), Error> {
    let mut game = game()?;
    stereo_stream(&mut game)?;
    assert_eq!(call(&mut game, "add_ref", &[STREAM])?, 2);
    assert_eq!(call(&mut game, "release", &[STREAM])?, 1);
    assert_eq!(call(&mut game, "set_frequency", &[STREAM, 0])?, DS_OK);
    assert_eq!(call(&mut game, "discontinuity", &[STREAM])?, DS_OK);
    packet(&mut game, ADPCM_BLOCK * 2 * 1000, true, PACKET_CONTEXT)?;
    assert_eq!(call(&mut game, "process", &[STREAM, PACKET, 0])?, DS_OK);
    // releasing it flushes what it holds
    assert_eq!(call(&mut game, "release", &[STREAM])?, 0);
    assert_eq!(completions(&game)?.len(), 1);
    assert_eq!(
        call(&mut game, "process", &[STREAM, PACKET, 0])?,
        DSERR_INVALIDCALL
    );
    assert_eq!(
        call(&mut game, "get_status", &[OTHER_STREAM, STATUS])?,
        DSERR_INVALIDCALL
    );
    Ok(())
}

#[test]
fn the_device_answers_as_the_linux_port() -> Result<(), Error> {
    let mut game = game()?;
    assert_eq!(call(&mut game, "ds_create", &[0, OUT, 0])?, DS_OK);
    let device = word(&game, OUT)?;
    assert_ne!(device, 0);

    assert_eq!(call(&mut game, "caps", &[device, OUT])?, DS_OK);
    assert_eq!(
        [
            word(&game, OUT)?,
            word(&game, OUT + 4)?,
            word(&game, OUT + 8)?,
            word(&game, OUT + 12)?
        ],
        [64, 64, 2047, 0]
    );
    put(&mut game, OUT, 0xFFFF_FFFF)?;
    assert_eq!(call(&mut game, "speakers", &[device, OUT])?, DS_OK);
    assert_eq!(word(&game, OUT)?, 0, "stereo");
    put(&mut game, OUT, 0xFFFF_FFFF)?;
    assert_eq!(call(&mut game, "effects", &[device, 0, 0, 0, OUT])?, DS_OK);
    assert_eq!(word(&game, OUT)?, 0, "no effects image");

    assert_eq!(
        call(&mut game, "buffer", &[device, DESCRIPTION, OUT, 0])?,
        DS_OK
    );
    let buffer = word(&game, OUT)?;
    assert_ne!(buffer, 0);
    assert_eq!(call(&mut game, "play", &[buffer, 0, 0, 0])?, DS_OK);
    assert_eq!(call(&mut game, "buffer_release", &[buffer])?, DS_OK);
    Ok(())
}
