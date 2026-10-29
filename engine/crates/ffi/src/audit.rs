//! The map audit's recorder (`--audit DIR`): a plugin that plays a map
//! by a fixed schedule of keys and mouse, and writes what the game's own
//! code did into the audit's folder.
//!
//! - `events.tsv`: each script thread of the game state's "hs thread"
//!   array (hs_runtime.c) as it starts and ends, by tick;
//! - `sound.tsv`: each second of the mixed sound from the map's first tick,
//!   its loudness in dBFS and its energy in octave bands from 63 Hz to
//!   16 kHz.
//!
//! The frames are taken by the app, which draws them. The game's random
//! seed is fixed, so two runs of a map differ only by the timing of their
//! frames; the schedule is keyed to the tick so that it does not add more.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::views::{GAME_STATE_ADDRESS, GAME_STATE_SIZE};
use opence_runtime::{Controls, SPEAKER_RATE};

/// the hs thread datum's type (`byte type`) and script index
/// (`long script_index`), after its 16-bit identifier and a flags byte
const THREAD_TYPE: u32 = 2;
const THREAD_SCRIPT: u32 = 4;

/// the octave bands' centres, in hertz
pub const BANDS: [f64; 9] = [
    63.0, 125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0,
];

/// A script thread as the audit tells them apart: its type
/// (`_hs_thread_type_script` 0, `_global_initialize` 1,
/// `_console_command` 2) and its script.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Thread {
    /// the thread's type
    pub kind: u8,
    /// its script's index, or -1 for none
    pub script: i32,
}

/// The lines of `events.tsv` for the threads that ended and started
/// between `before` and `now` (each keyed by its datum index), in the
/// datums' order: those that ended first.
#[must_use]
pub fn thread_events(
    tick: u64,
    before: &BTreeMap<u32, Thread>,
    now: &BTreeMap<u32, Thread>,
) -> Vec<String> {
    let line =
        |what: &str, thread: &Thread| format!("{tick}\t{what}\t{}\t{}", thread.script, thread.kind);
    let ended = before
        .iter()
        .filter(|(index, thread)| now.get(index) != Some(thread))
        .map(|(_, thread)| line("end", thread));
    let started = now
        .iter()
        .filter(|(index, thread)| before.get(index) != Some(thread))
        .map(|(_, thread)| line("start", thread));
    ended.chain(started).collect()
}

/// A band-pass filter (the RBJ cookbook's, 0 dB at its centre), one
/// octave wide.
#[derive(Debug, Clone)]
struct Band {
    b0: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    x: [f64; 2],
    y: [f64; 2],
}

impl Band {
    fn new(centre: f64, rate: f64) -> Self {
        let w0 = std::f64::consts::TAU * centre.min(rate * 0.45) / rate;
        let alpha = libm::sin(w0) * libm::sinh(std::f64::consts::LN_2 / 2.0 * w0 / libm::sin(w0));
        let a0 = 1.0 + alpha;
        Self {
            b0: alpha / a0,
            b2: -alpha / a0,
            a1: -2.0 * libm::cos(w0) / a0,
            a2: (1.0 - alpha) / a0,
            x: [0.0; 2],
            y: [0.0; 2],
        }
    }

    fn filter(&mut self, sample: f64) -> f64 {
        let out =
            self.b0 * sample + self.b2 * self.x[1] - self.a1 * self.y[0] - self.a2 * self.y[1];
        self.x = [sample, self.x[0]];
        self.y = [out, self.y[0]];
        out
    }
}

/// The sound's loudness, a second at a time: fed the mixed frames
/// (interleaved stereo), it gives a line of `sound.tsv` for each whole
/// second.
#[derive(Debug, Clone)]
pub struct Loudness {
    bands: Vec<Band>,
    /// the second's sums of squares: the whole, then each band's
    sums: Vec<f64>,
    frames: u32,
    seconds: u64,
}

impl Default for Loudness {
    fn default() -> Self {
        let rate = f64::from(SPEAKER_RATE);
        Self {
            bands: BANDS
                .iter()
                .map(|&centre| Band::new(centre, rate))
                .collect(),
            sums: vec![0.0; BANDS.len() + 1],
            frames: 0,
            seconds: 0,
        }
    }
}

/// a mean square in dBFS, floored at -120
fn decibels(mean_square: f64) -> f64 {
    if mean_square <= 1e-12 {
        -120.0
    } else {
        10.0 * mean_square.log10()
    }
}

impl Loudness {
    /// The header of `sound.tsv`.
    #[must_use]
    pub fn header() -> String {
        let mut header = "second\tdbfs".to_owned();
        for centre in BANDS {
            let _ = write!(header, "\tb{centre}");
        }
        header
    }

    /// Takes `samples` (interleaved stereo) and gives the lines of the
    /// seconds they complete.
    pub fn feed(&mut self, samples: &[f32]) -> Vec<String> {
        let mut lines = Vec::new();
        for frame in samples.as_chunks::<2>().0 {
            let mono = (f64::from(frame[0]) + f64::from(frame[1])) / 2.0;
            self.sums[0] += mono * mono;
            for (band, sum) in self.bands.iter_mut().zip(&mut self.sums[1..]) {
                let out = band.filter(mono);
                *sum += out * out;
            }
            self.frames += 1;
            if self.frames == SPEAKER_RATE {
                let count = f64::from(self.frames);
                let mut line = format!("{}", self.seconds);
                for sum in &self.sums {
                    let _ = write!(line, "\t{:.2}", decibels(sum / count));
                }
                lines.push(line);
                self.sums.iter_mut().for_each(|sum| *sum = 0.0);
                self.frames = 0;
                self.seconds += 1;
            }
        }
        lines
    }
}

/// What the schedule holds down at a tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Held {
    /// forward (W)
    pub forward: bool,
    /// back (S)
    pub back: bool,
    /// strafe left (A)
    pub left: bool,
    /// strafe right (D)
    pub right: bool,
    /// jump (space)
    pub jump: bool,
    /// fire (the left button)
    pub fire: bool,
}

/// the schedule's segments, each this many ticks (two seconds)
const SEGMENT: u64 = 120;

/// The schedule: what is held at `tick`, and how far the mouse moves, in
/// pixels right. It walks, turns, strafes, jumps and fires in a loop of
/// eight segments, so that each map is crossed, looked around and shot
/// in; the first five seconds are left alone for the map's opening.
#[must_use]
pub fn schedule(tick: u64) -> (Held, f32) {
    if tick < 300 {
        return (Held::default(), 0.0);
    }
    let tick = tick - 300;
    let within = tick % SEGMENT;
    let pulse = within % 30 < 4;
    match (tick / SEGMENT) % 8 {
        0 => (
            Held {
                forward: true,
                ..Held::default()
            },
            0.0,
        ),
        1 => (
            Held {
                forward: true,
                fire: pulse,
                ..Held::default()
            },
            6.0,
        ),
        2 => (
            Held {
                left: true,
                jump: pulse,
                ..Held::default()
            },
            0.0,
        ),
        3 => (
            Held {
                forward: true,
                ..Held::default()
            },
            -6.0,
        ),
        4 => (
            Held {
                fire: within % 20 < 10,
                ..Held::default()
            },
            3.0,
        ),
        5 => (
            Held {
                right: true,
                forward: true,
                ..Held::default()
            },
            0.0,
        ),
        6 => (
            Held {
                back: true,
                jump: pulse,
                ..Held::default()
            },
            -3.0,
        ),
        _ => (
            Held {
                forward: true,
                fire: pulse,
                ..Held::default()
            },
            12.0,
        ),
    }
}

/// the Windows keys of [`Held`]'s keys, with their characters
fn keys(held: Held) -> [(u8, u8, bool); 5] {
    [
        (b'W', b'w', held.forward),
        (b'S', b's', held.back),
        (b'A', b'a', held.left),
        (b'D', b'd', held.right),
        (0x20, b' ', held.jump),
    ]
}

/// The audit's plugin: it applies [`schedule`] through the game's
/// controls each tick, and writes `events.tsv` and `sound.tsv` into its
/// folder as it goes.
pub struct Audit {
    folder: PathBuf,
    controls: Controls,
    tick: u64,
    held: Held,
    array: Option<u32>,
    threads: BTreeMap<u32, Thread>,
    loudness: Loudness,
    events: Option<std::io::BufWriter<std::fs::File>>,
    sound: Option<std::io::BufWriter<std::fs::File>>,
}

/// a file of the folder with its header written, or none if it cannot be
fn table(folder: &Path, name: &str, header: &str) -> Option<std::io::BufWriter<std::fs::File>> {
    let mut file = std::io::BufWriter::new(std::fs::File::create(folder.join(name)).ok()?);
    writeln!(file, "{header}").ok()?;
    Some(file)
}

impl Audit {
    /// The audit of a run, into `folder` (made if missing), driving the
    /// game through `controls`.
    ///
    /// # Errors
    /// If the folder or its files cannot be made.
    pub fn new(folder: &Path, controls: Controls) -> std::io::Result<Self> {
        std::fs::create_dir_all(folder)?;
        let missing = || std::io::Error::other(format!("{}: not writable", folder.display()));
        let events =
            table(folder, "events.tsv", "tick\tevent\tscript\ttype").ok_or_else(missing)?;
        let sound = table(folder, "sound.tsv", &Loudness::header()).ok_or_else(missing)?;
        Ok(Self {
            folder: folder.to_owned(),
            controls,
            tick: 0,
            held: Held::default(),
            array: None,
            threads: BTreeMap::new(),
            loudness: Loudness::default(),
            events: Some(events),
            sound: Some(sound),
        })
    }

    /// The folder it writes into.
    #[must_use]
    pub fn folder(&self) -> &Path {
        &self.folder
    }

    /// the live script threads
    fn threads(&mut self, view: &View<'_>) -> BTreeMap<u32, Thread> {
        if self.array.is_none() {
            self.array = view
                .find_data_array(GAME_STATE_ADDRESS, GAME_STATE_SIZE, "hs thread")
                .map(|(address, _)| address);
        }
        let Some(array) = self.array.and_then(|address| view.data_array(address)) else {
            return BTreeMap::new();
        };
        array
            .datums(view)
            .into_iter()
            .filter_map(|(index, address)| {
                let kind = view
                    .bytes(address.checked_add(THREAD_TYPE)?, 1)?
                    .first()
                    .copied()?;
                let script = view.u32(address.checked_add(THREAD_SCRIPT)?)?;
                Some((
                    index,
                    Thread {
                        kind,
                        script: i32::from_ne_bytes(script.to_ne_bytes()),
                    },
                ))
            })
            .collect()
    }

    fn drive(&mut self) {
        let (held, turn) = schedule(self.tick);
        for ((vk, ascii, now), (_, _, was)) in keys(held).into_iter().zip(keys(self.held)) {
            if now != was {
                self.controls.key(vk, ascii, 0, now);
            }
        }
        if held.fire != self.held.fire {
            self.controls.mouse_button(1, held.fire);
        }
        if turn != 0.0 {
            self.controls.mouse_motion(turn, 0.0);
        }
        self.held = held;
    }

    fn tick(&mut self, view: &View<'_>) {
        self.drive();
        let now = self.threads(view);
        let lines = thread_events(self.tick, &self.threads, &now);
        self.threads = now;
        if let Some(events) = &mut self.events {
            if lines.iter().any(|line| writeln!(events, "{line}").is_err()) {
                self.events = None;
            }
        }
        self.tick += 1;
        // (flushed each second, so that a run cut short keeps what it saw)
        if self.tick.is_multiple_of(60) {
            for file in [&mut self.events, &mut self.sound].into_iter().flatten() {
                let _ = file.flush();
            }
        }
    }

    fn sound(&mut self, samples: &[f32]) {
        // (the sound is counted from the map's first tick: the load's length
        // differs from run to run, and would shift every second after it)
        if self.tick == 0 {
            return;
        }
        let lines = self.loudness.feed(samples);
        if let Some(sound) = &mut self.sound {
            if lines.iter().any(|line| writeln!(sound, "{line}").is_err()) {
                self.sound = None;
            }
        }
    }
}

impl Drop for Audit {
    fn drop(&mut self) {
        for file in [&mut self.events, &mut self.sound].into_iter().flatten() {
            let _ = file.flush();
        }
    }
}

impl Plugin for Audit {
    fn name(&self) -> &str {
        "audit"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Tick, Hook::Sound]
    }

    fn on(&mut self, event: &Event<'_>, view: &View<'_>) {
        match event {
            Event::Tick => self.tick(view),
            Event::Sound(samples) => self.sound(samples),
            _ => {}
        }
    }
}
