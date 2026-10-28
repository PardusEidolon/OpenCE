//! What the Gearbox PC port lost, measured in the original's own code: each
//! effect of the Xbox rasterizer has its switch among the original's
//! console globals (`rasterizer_debug_options`,
//! rasterizer/rasterizer_console_vars.h), so Blood Gulch is played once as
//! it is and once with each effect switched off, the time stepped a 60th a
//! frame (`debug.frame_clock`) so that frame N is the same moment in every
//! run, and the draws each frame's commands make are counted. An effect
//! the game draws in Blood Gulch draws less with its switch off; the
//! catalogue (docs/pc_port.md) records what each one measured. Needs
//! game.wasm and the user's maps (assets/maps); skips itself without them.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use opence_render::Command;
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime};

/// the effects measured: the original's console global that switches each
/// off, and what it draws
const EFFECTS: &[(&str, &str)] = &[
    ("rasterizer_environment_lightmaps", "the lightmaps"),
    ("rasterizer_environment_decals", "the environment's decals"),
    ("rasterizer_environment_specular_lights", "specular lights"),
    (
        "rasterizer_environment_specular_lightmaps",
        "specular lightmaps",
    ),
    ("rasterizer_environment_reflections", "bumped reflections"),
    (
        "rasterizer_environment_transparents",
        "transparent geometry",
    ),
    ("rasterizer_environment_fog", "the environment's fog"),
    ("rasterizer_environment_fog_screen", "the fog screen"),
    ("rasterizer_fog_plane", "planar fog"),
    ("rasterizer_fog_atmosphere", "atmospheric fog"),
    ("rasterizer_water", "the water"),
    ("rasterizer_lens_flares", "lens flares"),
    ("rasterizer_lens_flares_occlusion", "lens flare occlusion"),
    ("rasterizer_detail_objects", "detail objects"),
    ("rasterizer_models", "the models"),
    ("rasterizer_model_transparents", "the models' transparents"),
    ("rasterizer_bump_mapping", "bump mapping"),
    ("rasterizer_shadows_convolution", "the shadows' softening"),
    ("rasterizer_hud_motion_sensor", "the motion sensor"),
    ("rasterizer_active_camouflage", "active camouflage"),
    ("rasterizer_plasma_energy", "plasma energy"),
    ("rasterizer_screen_flashes", "screen flashes"),
    ("rasterizer_screen_effects", "screen effects"),
];

/// the effects Blood Gulch draws in the frames played, each found to draw
/// less with its switch off
const DRAWN_IN_BLOOD_GULCH: &[&str] = &[
    "rasterizer_environment_lightmaps",
    "rasterizer_environment_decals",
    "rasterizer_environment_reflections",
    "rasterizer_environment_transparents",
    "rasterizer_environment_fog",
    "rasterizer_models",
    "rasterizer_model_transparents",
    "rasterizer_shadows_convolution",
    "rasterizer_hud_motion_sensor",
];

/// how long each run plays: the load and some seconds of play
const PLAYED: Duration = Duration::from_secs(50);

/// runs at once
const AT_ONCE: usize = 4;

/// each frame's draws and the vertices (or indices) they draw
type Frames = Vec<(u64, u64)>;

/// counts each frame's draws
struct Draws {
    frames: Arc<Mutex<Frames>>,
}

impl Plugin for Draws {
    fn name(&self) -> &str {
        "draws"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Render]
    }

    fn on(&mut self, event: &Event<'_>, _view: &View<'_>) {
        let Event::Render(commands) = event else {
            return;
        };
        let (mut draws, mut drawn) = (0u64, 0u64);
        for command in *commands {
            match *command {
                Command::Draw {
                    vertices,
                    instances,
                    ..
                } => {
                    draws += 1;
                    drawn += u64::from(vertices) * u64::from(instances.max(1));
                }
                Command::DrawIndexed {
                    indices, instances, ..
                } => {
                    draws += 1;
                    drawn += u64::from(indices) * u64::from(instances.max(1));
                }
                _ => {}
            }
        }
        if let Ok(mut frames) = self.frames.lock() {
            frames.push((draws, drawn));
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Blood Gulch played with `off` switched off (or as it is), and its
/// frames' draws
fn play(wasm: &[u8], off: Option<&str>) -> Result<Frames, Error> {
    let root = root();
    let saves = std::env::temp_dir().join(format!(
        "opence-pc-port-{}-{}",
        std::process::id(),
        off.unwrap_or("stock")
    ));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(wasm)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    assert!(game.set_setting("debug.frame_clock", "true"));
    let mut commands = String::new();
    if let Some(global) = off {
        let _ = writeln!(commands, "{global} false");
    }
    commands.push_str("map_name bloodgulch\n");
    game.set_init_commands(&commands);
    let frames = Arc::new(Mutex::new(Frames::new()));
    game.add_plugin(Box::new(Draws {
        frames: Arc::clone(&frames),
    }));
    game.set_deadline(Some(Instant::now() + PLAYED));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    let frames = frames
        .lock()
        .map(|frames| frames.clone())
        .unwrap_or_default();
    Ok(frames)
}

/// the draws and vertices over the frames both runs drew
fn totals(stock: &Frames, off: &Frames) -> ((u64, u64), (u64, u64)) {
    let frames = stock.len().min(off.len());
    let sum = |run: &Frames| {
        run.iter()
            .take(frames)
            .fold((0, 0), |(draws, drawn), (d, v)| (draws + d, drawn + v))
    };
    (sum(stock), sum(off))
}

#[test]
fn each_effect_blood_gulch_draws_draws_less_with_its_switch_off() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/bloodgulch.map");
        return Ok(());
    };
    let wasm = Arc::new(wasm);
    let mut runs: Vec<Option<&str>> = vec![None];
    runs.extend(EFFECTS.iter().map(|(global, _)| Some(*global)));
    let mut played: Vec<(Option<&str>, Frames)> = Vec::new();
    for batch in runs.chunks(AT_ONCE) {
        let handles: Vec<_> = batch
            .iter()
            .map(|off| {
                let wasm = Arc::clone(&wasm);
                let off = off.map(str::to_owned);
                std::thread::spawn(move || play(&wasm, off.as_deref()))
            })
            .collect();
        for (off, handle) in batch.iter().zip(handles) {
            let frames = handle.join().unwrap_or_else(|_| Ok(Frames::new()))?;
            played.push((*off, frames));
        }
    }
    let Some((None, stock)) = played.first().cloned() else {
        panic!("no stock run");
    };
    assert!(
        stock.iter().any(|(draws, _)| *draws > 0),
        "the stock run drew nothing"
    );
    let mut table = String::from("global\tframes\tdraws on\tdraws off\tdrawn on\tdrawn off\n");
    let mut less = Vec::new();
    for (off, frames) in played.iter().skip(1) {
        let Some(global) = off else { continue };
        let ((draws_on, drawn_on), (draws_off, drawn_off)) = totals(&stock, frames);
        let _ = writeln!(
            table,
            "{global}\t{}\t{draws_on}\t{draws_off}\t{drawn_on}\t{drawn_off}",
            stock.len().min(frames.len())
        );
        // a switched-off run draws a few more than the stock one (about 43
        // draws over the run, the console echoing the init command), so an
        // effect counts as drawn when switching it off saves at least half a
        // draw a frame
        let frames = stock.len().min(frames.len()) as u64;
        if draws_on >= draws_off + frames / 2 {
            less.push(*global);
        }
    }
    eprintln!("{table}");
    let _ = std::fs::create_dir_all(root.join("build/engine"));
    let _ = std::fs::write(root.join("build/engine/pc_port.tsv"), &table);
    for global in DRAWN_IN_BLOOD_GULCH {
        assert!(
            less.contains(global),
            "{global} drew no less switched off:\n{table}"
        );
    }
    Ok(())
}
