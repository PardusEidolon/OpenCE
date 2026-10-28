//! The game run as game.wasm for the app (`opence_ffi::game`): its render
//! batches handed to the app's renderer ([`RenderTarget`]) as frames, each
//! its commands and the bytes its uploads name, one after another; the
//! textures the renderer reads back written where the game asked.

use std::sync::{Arc, Mutex, PoisonError};

use opence_ffi::game::{
    render_sink, RenderFrame, RenderReadback, RenderTarget, WasmGame, WasmGameOptions,
};
use opence_render::{op, Command, PixelFormat, TextureKind, MAGIC, USAGE_RENDER_TARGET, VERSION};
use opence_runtime::{Error, Runtime, Val};

/// where the test writes its batches
const BATCH: u32 = 0x100;

/// a batch of `commands`, each its op and fields, with its header
fn batch(commands: &[(u32, &[u32])]) -> Vec<u8> {
    let mut words = vec![MAGIC, VERSION, 0, 0];
    for (code, fields) in commands {
        let length = u32::try_from(fields.len() + 1).unwrap_or(0);
        words.push(code | (length << 16));
        words.extend_from_slice(fields);
    }
    let size = u32::try_from(words.len() * 4).unwrap_or(0);
    words[2] = size;
    words[3] = u32::try_from(commands.len()).unwrap_or(0);
    words.iter().flat_map(|word| word.to_le_bytes()).collect()
}

/// a module that submits the batch it is given
const DEVICE: &str = r#"
(module
  (import "env" "host_render_submit" (func $submit (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "submit") (param i32 i32) (result i32) (call $submit (local.get 0) (local.get 1))))
"#;

/// A renderer that keeps each frame and answers with `readbacks`.
#[derive(Default)]
struct Target {
    frames: Mutex<Vec<RenderFrame>>,
    readbacks: Vec<RenderReadback>,
}

impl RenderTarget for Target {
    fn submit(&self, frame: RenderFrame) -> Vec<RenderReadback> {
        self.frames
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(frame);
        self.readbacks.clone()
    }
}

fn frames(target: &Target) -> Vec<RenderFrame> {
    std::mem::take(&mut *target.frames.lock().unwrap_or_else(PoisonError::into_inner))
}

fn submit(game: &mut opence_runtime::Game, bytes: &[u8]) -> Result<Option<i32>, Error> {
    game.write(u64::from(BATCH), bytes)?;
    let size = i32::try_from(bytes.len()).unwrap_or(0);
    let results = game.call_with("submit", &[Val::I32(BATCH.cast_signed()), Val::I32(size)])?;
    Ok(results.first().and_then(Val::i32))
}

#[test]
fn a_frame_is_the_batch_s_commands_and_the_bytes_its_uploads_name() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&wat::parse_str(DEVICE).unwrap_or_default())?;
    let target = Arc::new(Target::default());
    game.set_render_sink(render_sink(Arc::clone(&target) as Arc<dyn RenderTarget>));
    game.write(0x800, b"abcdefgh")?;
    game.write(0x900, b"xy")?;
    let bytes = batch(&[
        (op::CREATE_BUFFER, &[1, 8]),
        (op::UPLOAD_BUFFER, &[1, 0, 0x800, 8]),
        (op::WRITE_BUFFER, &[1, 2, 0x900, 2]),
    ]);
    assert_eq!(submit(&mut game, &bytes)?, Some(0));
    let frames = frames(&target);
    let [frame] = frames.as_slice() else {
        panic!("the renderer was handed {} frames", frames.len());
    };
    assert_eq!(frame.commands.len(), 3);
    assert_eq!(
        frame.commands.get(1),
        Some(&Command::UploadBuffer {
            id: 1,
            offset: 0,
            address: 0x800,
            length: 8
        })
    );
    assert_eq!(frame.data, b"abcdefghxy");
    Ok(())
}

#[test]
fn a_texture_read_back_is_written_where_its_command_names() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&wat::parse_str(DEVICE).unwrap_or_default())?;
    let target = Arc::new(Target {
        frames: Mutex::default(),
        readbacks: vec![
            RenderReadback {
                address: 0xa00,
                bytes: vec![1, 2, 3, 4],
            },
            // (no command reads back here: refused)
            RenderReadback {
                address: 0xb00,
                bytes: vec![9],
            },
        ],
    });
    game.set_render_sink(render_sink(Arc::clone(&target) as Arc<dyn RenderTarget>));
    let plane = TextureKind::Plane.word();
    let rgba8 = PixelFormat::Rgba8.word();
    let bytes = batch(&[
        (
            op::CREATE_TEXTURE,
            &[1, plane, rgba8, 1, 1, 1, 1, USAGE_RENDER_TARGET, 1],
        ),
        (op::READ_TEXTURE, &[1, 0, 0, 0, 0, 1, 1, 0xa00, 4, 4]),
    ]);
    assert_eq!(submit(&mut game, &bytes)?, Some(0));
    assert_eq!(game.read(0xa00, 4)?, vec![1, 2, 3, 4]);
    assert_eq!(game.read(0xb00, 1)?, vec![0]);
    assert_eq!(game.render_refusals().len(), 1);
    Ok(())
}

/// a game whose main submits one batch, already in its memory, and returns
const MAIN: &str = r#"
(module
  (import "env" "host_render_submit" (func $submit (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 0x100) "BATCH")
  (func (export "halo_game_main") (drop (call $submit (i32.const 0x100) (i32.const SIZE)))))
"#;

#[test]
fn the_game_runs_on_its_own_thread_and_draws_to_the_target() {
    let bytes = batch(&[(op::CREATE_BUFFER, &[1, 64])]);
    let escaped: String = bytes.iter().map(|byte| format!("\\{byte:02x}")).collect();
    let text = MAIN
        .replace("BATCH", &escaped)
        .replace("SIZE", &bytes.len().to_string());
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("wasm_game");
    let _ = std::fs::create_dir_all(folder.join("saves"));
    let wasm = folder.join("game.wasm");
    let _ = std::fs::write(&wasm, wat::parse_str(&text).unwrap_or_default());
    let display = |path: &std::path::Path| path.display().to_string();
    let game = WasmGame::new(WasmGameOptions {
        wasm: display(&wasm),
        data: display(&folder),
        saves: display(&folder.join("saves")),
        cache: None,
        width: 1280,
        height: 720,
        map: None,
        seconds: None,
    });
    let game = match game {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    let target = Arc::new(Target::default());
    assert!(game
        .start(Arc::clone(&target) as Arc<dyn RenderTarget>)
        .is_ok());
    // a second start is refused: the game is running
    assert!(game
        .start(Arc::clone(&target) as Arc<dyn RenderTarget>)
        .is_err());
    let end = game.wait();
    assert_eq!(end.error, None);
    assert_eq!((end.batches, end.commands, end.refused), (1, 1, 0));
    assert_eq!(
        frames(&target)
            .iter()
            .map(|frame| frame.commands.clone())
            .collect::<Vec<_>>(),
        [vec![Command::CreateBuffer { id: 1, length: 64 }]]
    );
}

/// a game whose main never returns (it submits nothing, over and over)
const FOREVER: &str = r#"
(module
  (import "env" "host_render_submit" (func $submit (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "halo_game_main") (loop $again (drop (call $submit (i32.const 0) (i32.const 0))) (br $again))))
"#;

#[test]
fn a_game_given_seconds_ends_when_they_are_up() {
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("wasm_game_seconds");
    let _ = std::fs::create_dir_all(folder.join("saves"));
    let wasm = folder.join("game.wasm");
    let _ = std::fs::write(&wasm, wat::parse_str(FOREVER).unwrap_or_default());
    let display = |path: &std::path::Path| path.display().to_string();
    let game = match WasmGame::new(WasmGameOptions {
        wasm: display(&wasm),
        data: display(&folder),
        saves: display(&folder.join("saves")),
        cache: None,
        width: 640,
        height: 480,
        map: None,
        seconds: Some(1),
    }) {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    let started = std::time::Instant::now();
    assert!(game
        .start(Arc::new(Target::default()) as Arc<dyn RenderTarget>)
        .is_ok());
    let end = game.wait();
    // (its time running out is how it was meant to end, not an error)
    assert_eq!(end.error, None);
    assert!(end.timed_out);
    assert!(started.elapsed() < std::time::Duration::from_secs(10));
}
