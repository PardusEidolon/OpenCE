//! The plugin hooks (`opence_runtime::plugins`): the game's own seams, its
//! profile sections, its `Present` and its mixer's output, are where the
//! runtime hands each plugin the moment and a read-only view of the game's
//! memory. A plugin is called only for the hooks it asks for, in the order
//! the plugins were added.

use std::sync::{Arc, Mutex, PoisonError};

use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Game, Runtime, Val};

/// the module: the sections the hooks follow, `Present`, the mixer's call,
/// and a word for the view to read
const MODULE: &str = r#"
(module
  (import "env" "halo_profile_enter" (func $enter (param i32)))
  (import "env" "halo_profile_exit" (func $exit (param i32)))
  (import "env" "D3DDevice_Clear" (func $clear (param i32 i32 i32 i32 f32 i32)))
  (import "env" "D3DDevice_Present" (func $present (param i32 i32 i32 i32)))
  (import "env" "halo_audio_output" (func $output (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 16) "game_tick.player_effect\00")
  (data (i32.const 48) "game_tick.ai\00")
  (data (i32.const 64) "game_tick.hud\00")
  (data (i32.const 80) "ui_update\00")
  (data (i32.const 96) "throttle\00")
  (data (i32.const 112) "game_tick.units\00")
  (data (i32.const 256) "\2a\00\00\00")
  (func (export "section") (param i32)
    (call $enter (local.get 0)) (call $exit (local.get 0)))
  (func (export "clear")
    (call $clear (i32.const 0) (i32.const 0) (i32.const 1) (i32.const 0) (f32.const 1) (i32.const 0)))
  (func (export "present")
    (call $present (i32.const 0) (i32.const 0) (i32.const 0) (i32.const 0)))
  (func (export "output") (param i32 i32) (result i32)
    (call $output (local.get 0) (local.get 1))))
"#;

const TICK: i32 = 16;
const AI: i32 = 48;
const HUD: i32 = 64;
const UI: i32 = 80;
const FRAME: i32 = 96;
const UNITS: i32 = 112;
const WORD: u32 = 256;

fn game() -> Result<Game, Error> {
    let bytes = wat::parse_str(MODULE).unwrap_or_default();
    Runtime::new(None)?.load(&bytes)
}

fn section(game: &mut Game, name: i32) -> Result<(), Error> {
    game.call_with("section", &[Val::I32(name)]).map(|_| ())
}

/// what a plugin saw: its name, the hook, and what the event carried
type Seen = Arc<Mutex<Vec<String>>>;

/// a plugin that writes down each event it is handed, with the word at
/// [`WORD`] as its view reads it
struct Witness {
    name: &'static str,
    hooks: Vec<Hook>,
    seen: Seen,
}

impl Plugin for Witness {
    fn name(&self) -> &str {
        self.name
    }

    fn hooks(&self) -> &[Hook] {
        &self.hooks
    }

    fn on(&mut self, event: &Event<'_>, view: &View<'_>) {
        let word = view.u32(WORD).unwrap_or(0);
        let what = match event {
            Event::Render(frame) => format!("render {} calls", frame.len()),
            Event::Sound(samples) => format!("sound {}", samples.len()),
            other => format!("{:?}", other.hook()),
        };
        self.seen
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(format!("{} {what} {word}", self.name));
    }
}

fn witness(name: &'static str, hooks: &[Hook], seen: &Seen) -> Box<dyn Plugin> {
    Box::new(Witness {
        name,
        hooks: hooks.to_vec(),
        seen: Arc::clone(seen),
    })
}

fn seen(seen: &Seen) -> Vec<String> {
    std::mem::take(&mut *seen.lock().unwrap_or_else(PoisonError::into_inner))
}

#[test]
fn each_section_the_hooks_follow_calls_the_plugins_that_ask_for_it() -> Result<(), Error> {
    let mut game = game()?;
    let log = Seen::default();
    game.add_plugin(witness("all", &Hook::ALL, &log));
    for name in [TICK, AI, HUD, UI, FRAME, UNITS] {
        section(&mut game, name)?;
    }
    assert_eq!(
        seen(&log),
        [
            "all Tick 42",
            "all Ai 42",
            "all Hud 42",
            "all Ui 42",
            "all Frame 42"
        ]
    );
    Ok(())
}

#[test]
fn a_plugin_hears_only_its_hooks_in_the_order_the_plugins_were_added() -> Result<(), Error> {
    let mut game = game()?;
    let log = Seen::default();
    game.add_plugin(witness("first", &[Hook::Tick], &log));
    game.add_plugin(witness("second", &[Hook::Tick, Hook::Ui], &log));
    section(&mut game, UI)?;
    section(&mut game, TICK)?;
    assert_eq!(
        seen(&log),
        ["second Ui 42", "first Tick 42", "second Tick 42"]
    );
    Ok(())
}

#[test]
fn present_hands_the_render_hook_the_frame_it_ends() -> Result<(), Error> {
    let mut game = game()?;
    let log = Seen::default();
    game.add_plugin(witness("draw", &[Hook::Render], &log));
    game.call("clear")?;
    game.call("present")?;
    assert_eq!(seen(&log), ["draw render 2 calls 42"]);
    // the frame still goes to the app as before
    assert_eq!(game.frames().take().map(|frame| frame.len()), Some(2));
    Ok(())
}

#[test]
fn the_mixers_output_hands_the_sound_hook_its_samples() -> Result<(), Error> {
    let mut game = game()?;
    let log = Seen::default();
    game.add_plugin(witness("ears", &[Hook::Sound], &log));
    game.call_with("output", &[Val::I32(0x1000), Val::I32(64)])?;
    // 64 stereo frames
    assert_eq!(seen(&log), ["ears sound 128 42"]);
    Ok(())
}

#[test]
fn with_no_plugin_the_sections_run_as_before() -> Result<(), Error> {
    let mut game = game()?;
    section(&mut game, TICK)?;
    game.set_profiling(true);
    section(&mut game, TICK)?;
    let names: Vec<String> = game
        .take_profile_spans()
        .into_iter()
        .map(|span| span.name)
        .collect();
    assert_eq!(names, ["game_tick.player_effect"]);
    Ok(())
}

#[test]
fn the_view_reads_in_bounds_and_answers_none_outside() -> Result<(), Error> {
    struct Edges(Seen);
    impl Plugin for Edges {
        fn name(&self) -> &str {
            "edges"
        }
        fn hooks(&self) -> &[Hook] {
            &[Hook::Tick]
        }
        fn on(&mut self, _: &Event<'_>, view: &View<'_>) {
            let last = view.len().saturating_sub(4);
            let answers = [
                view.u32(WORD).is_some(),
                view.u32(u32::try_from(last).unwrap_or(0)).is_some(),
                view.u32(u32::try_from(last + 1).unwrap_or(0)).is_none(),
                view.bytes(u32::MAX, 2).is_none(),
                view.bytes(WORD, 4) == Some(vec![0x2a, 0, 0, 0]),
            ];
            self.0
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(format!("{answers:?}"));
        }
    }
    let mut game = game()?;
    let log = Seen::default();
    game.add_plugin(Box::new(Edges(Arc::clone(&log))));
    section(&mut game, TICK)?;
    assert_eq!(seen(&log), ["[true, true, true, true, true]"]);
    Ok(())
}

/// a plugin that takes `busy` at each tick, with `share` as its share of a
/// frame
struct Busy {
    name: &'static str,
    busy: std::time::Duration,
    share: std::time::Duration,
}

impl Plugin for Busy {
    fn name(&self) -> &str {
        self.name
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Tick]
    }

    fn share(&self) -> std::time::Duration {
        self.share
    }

    fn on(&mut self, _: &Event<'_>, _: &View<'_>) {
        let start = std::time::Instant::now();
        while start.elapsed() < self.busy {
            std::hint::spin_loop();
        }
    }
}

fn busy(name: &'static str, busy_us: u64, share_us: u64) -> Box<dyn Plugin> {
    Box::new(Busy {
        name,
        busy: std::time::Duration::from_micros(busy_us),
        share: std::time::Duration::from_micros(share_us),
    })
}

#[test]
fn each_plugins_time_is_logged_per_frame() -> Result<(), Error> {
    let mut game = game()?;
    game.add_plugin(busy("slow", 2_000, 10_000));
    game.add_plugin(busy("quick", 0, 10_000));
    for _ in 0..3 {
        // two ticks in the frame: their times add up
        section(&mut game, TICK)?;
        section(&mut game, TICK)?;
        section(&mut game, FRAME)?;
    }
    let frames = game.take_plugin_frames();
    assert_eq!(frames.len(), 3);
    for frame in &frames {
        let names: Vec<&str> = frame.iter().map(|time| time.name.as_str()).collect();
        assert_eq!(names, ["slow", "quick"]);
        assert!(frame[0].nanos >= 4_000_000, "{frame:?}");
        assert!(frame[1].nanos < frame[0].nanos, "{frame:?}");
        assert_eq!(frame[0].share, 10_000_000);
    }
    // taken: the log starts again
    assert!(game.take_plugin_frames().is_empty());
    Ok(())
}

#[test]
fn a_plugin_over_its_share_is_named() -> Result<(), Error> {
    let mut game = game()?;
    game.add_plugin(busy("greedy", 3_000, 1_000));
    game.add_plugin(busy("modest", 0, 1_000));
    section(&mut game, TICK)?;
    section(&mut game, FRAME)?;
    let over = opence_runtime::plugins::over_share(&game.take_plugin_frames());
    let names: Vec<&str> = over.iter().map(|(_, time)| time.name.as_str()).collect();
    assert_eq!(names, ["greedy"]);
    assert_eq!(over[0].0, 0);
    Ok(())
}

#[test]
fn with_no_plugin_no_frame_is_logged() -> Result<(), Error> {
    let mut game = game()?;
    section(&mut game, TICK)?;
    section(&mut game, FRAME)?;
    assert!(game.take_plugin_frames().is_empty());
    Ok(())
}

/// The hooks' inputs come from the game: the mixer's address and count,
/// and the render calls' arguments. Whatever they are, a plugin is handed
/// only what is inside the memory, or the import fails; nothing panics.
#[test]
fn the_hooks_inputs_from_the_game_never_panic() -> Result<(), Error> {
    let mut game = game()?;
    let log = Seen::default();
    game.add_plugin(witness("all", &Hook::ALL, &log));
    let mut state: u64 = 0x005E_ED0F_0AC3;
    let mut next = move || {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    };
    for _ in 0..512 {
        let address = (next() >> 32) as i32;
        let frames = (next() >> 48) as i32;
        let heard = game.call_with("output", &[Val::I32(address), Val::I32(frames)]);
        let inside = u64::from(address as u32) + u64::from(frames as u32) * 8 <= 65_536;
        assert_eq!(heard.is_ok(), inside, "{address:#x} {frames}");
        let name = (next() % 512) as i32;
        let _ = game.call_with("section", &[Val::I32(name)]);
    }
    let _ = seen(&log);
    Ok(())
}
