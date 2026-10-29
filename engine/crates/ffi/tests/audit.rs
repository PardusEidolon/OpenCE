//! The map audit's recorder: the script threads' starts and ends, the
//! sound's loudness a second at a time, the schedule of keys and mouse,
//! and a run of the first campaign map recorded through it (which needs
//! game.wasm and the user's maps, and skips itself without them).

use std::collections::BTreeMap;
use std::path::Path;

use opence_ffi::audit::{schedule, thread_events, Audit, Held, Loudness, Thread, BANDS};
use opence_runtime::plugins::{Event, Plugin, View};

#[test]
fn threads_that_end_and_start_are_written_by_tick() {
    let thread = |kind, script| Thread { kind, script };
    let before = BTreeMap::from([(0, thread(0, 3)), (1, thread(1, -1)), (2, thread(0, 7))]);
    // 1 ended, 2's datum was reused by another script, 4 started
    let now = BTreeMap::from([(0, thread(0, 3)), (2, thread(0, 8)), (4, thread(2, -1))]);
    assert_eq!(
        thread_events(90, &before, &now),
        [
            "90\tend\t-1\t1",
            "90\tend\t7\t0",
            "90\tstart\t8\t0",
            "90\tstart\t-1\t2",
        ]
    );
    assert!(thread_events(91, &now, &now).is_empty());
}

/// a second of a sine at `hertz` with `amplitude`, interleaved stereo
fn sine(hertz: f64, amplitude: f64) -> Vec<f32> {
    (0..48_000)
        .flat_map(|frame| {
            let sample =
                amplitude * libm::sin(std::f64::consts::TAU * hertz * f64::from(frame) / 48_000.0);
            let sample = sample as f32;
            [sample, sample]
        })
        .collect()
}

/// the columns of a line of sound.tsv
fn columns(line: &str) -> Vec<f64> {
    line.split('\t')
        .filter_map(|column| column.parse().ok())
        .collect()
}

#[test]
fn each_second_of_sound_is_its_loudness_and_its_octave_bands() {
    let mut loudness = Loudness::default();
    assert_eq!(Loudness::header().split('\t').count(), 2 + BANDS.len());
    // half a second gives nothing yet
    let tone = sine(1000.0, 0.5);
    assert!(loudness.feed(&tone[..48_000]).is_empty());
    let lines = loudness.feed(&tone[48_000..]);
    assert_eq!(lines.len(), 1);
    let line = columns(&lines[0]);
    assert_eq!(line[0], 0.0);
    // a sine of 0.5 is 0.5/√2 RMS: -9.03 dBFS
    assert!((line[1] - -9.03).abs() < 0.05, "{line:?}");
    // loudest in the 1 kHz band, and well down an octave and more away
    let bands = &line[2..];
    let loudest = bands
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .map(|(index, _)| index);
    assert_eq!(loudest, Some(4));
    assert!(
        bands[4] - bands[2] > 10.0 && bands[4] - bands[6] > 10.0,
        "{bands:?}"
    );
    // silence is the floor
    let silent = loudness.feed(&vec![0.0; 96_000]);
    assert_eq!(columns(&silent[0])[..2], [1.0, -120.0]);
}

#[test]
fn the_schedule_waits_for_the_opening_then_walks_turns_jumps_and_fires() {
    assert!((0..300).all(|tick| schedule(tick) == (Held::default(), 0.0)));
    let held: Vec<_> = (300..300 + 8 * 120).map(schedule).collect();
    assert!(held.iter().any(|(held, _)| held.forward));
    assert!(held.iter().any(|(held, _)| held.back));
    assert!(held.iter().any(|(held, _)| held.left));
    assert!(held.iter().any(|(held, _)| held.right));
    assert!(held.iter().any(|(held, _)| held.jump));
    assert!(held.iter().any(|(held, _)| held.fire));
    assert!(held.iter().any(|&(_, turn)| turn > 0.0));
    assert!(held.iter().any(|&(_, turn)| turn < 0.0));
    // it repeats, by tick alone
    assert_eq!(schedule(300 + 17), schedule(300 + 17 + 8 * 120));
}

#[test]
fn a_run_of_the_first_map_records_its_scripts_and_its_sound() -> Result<(), opence_runtime::Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/a10.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/a10.map");
        return Ok(());
    };
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join("audit_a10");
    let saves = Path::new(env!("CARGO_TARGET_TMPDIR")).join("audit_saves");
    let _ = std::fs::remove_dir_all(&folder);
    let _ = std::fs::create_dir_all(&saves);
    let runtime = opence_runtime::Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    // the sound before the map's first tick (the load's) is left out, and
    // counted from the tick on
    let early = Path::new(env!("CARGO_TARGET_TMPDIR")).join("audit_early");
    {
        let mut audit = Audit::new(&early, game.controls())
            .map_err(|error| opence_runtime::Error::Setup(error.to_string()))?;
        let tone = sine(1000.0, 0.5);
        View::over(&mut [0; 64], |view| {
            audit.on(&Event::Sound(&tone), view);
            audit.on(&Event::Tick, view);
            audit.on(&Event::Sound(&tone), view);
        });
    }
    let early = std::fs::read_to_string(early.join("sound.tsv")).unwrap_or_default();
    assert_eq!(early.lines().count(), 2, "{early}");
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    game.set_init_commands("map_name a10\n");
    let audit = Audit::new(&folder, game.controls())
        .map_err(|error| opence_runtime::Error::Setup(error.to_string()))?;
    game.add_plugin(Box::new(audit));
    game.set_deadline(Some(
        std::time::Instant::now() + std::time::Duration::from_secs(45),
    ));
    let ran = game.call("halo_game_main");
    drop(game);
    assert!(
        matches!(ran, Err(opence_runtime::Error::Deadline)),
        "the game ended before its deadline: {ran:?}"
    );
    let events = std::fs::read_to_string(folder.join("events.tsv")).unwrap_or_default();
    let sound = std::fs::read_to_string(folder.join("sound.tsv")).unwrap_or_default();
    eprintln!(
        "a10: {} script events, {} seconds of sound",
        events.lines().count() - 1,
        sound.lines().count() - 1
    );
    // the opening's scripts start, and the mix is heard
    assert!(
        events.lines().any(|line| line.contains("\tstart\t")),
        "{events}"
    );
    assert!(sound.lines().count() > 5, "{sound}");
    assert!(
        sound
            .lines()
            .skip(1)
            .any(|line| columns(line).get(1).is_some_and(|&dbfs| dbfs > -60.0)),
        "{sound}"
    );
    Ok(())
}
