//! The benchmark's paths and its report.
//!
//! A path is a level and its players' inputs, one per tick, made from the
//! tick and the player's place alone, so that every run replays the same
//! camera and the same world: devgulch and Blood Gulch with the player
//! alone, and Blood Gulch with sixteen. The app draws the frames
//! (`--benchmark`), and the report here turns their times into the
//! mean, the percentiles, the hitches and the pacing, and finds the
//! regressions against a run kept before.

use opence_sim::input::{Buttons, TickInput};
use opence_sim::units::Radians;

/// The benchmark's paths.
#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum BenchmarkPath {
    /// devgulch, the player alone.
    Devgulch,
    /// Blood Gulch, the player alone.
    BloodGulch,
    /// Blood Gulch with sixteen players.
    BloodGulch16,
}

impl BenchmarkPath {
    /// Every path, in the order the benchmark runs them.
    pub const ALL: [Self; 3] = [Self::Devgulch, Self::BloodGulch, Self::BloodGulch16];

    /// The path's name, as `--path` takes it and the report shows it.
    pub fn name(self) -> &'static str {
        match self {
            Self::Devgulch => "devgulch",
            Self::BloodGulch => "bloodgulch",
            Self::BloodGulch16 => "bloodgulch16",
        }
    }

    /// The path named `name`.
    pub fn named(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|path| path.name() == name)
    }

    /// The map it plays.
    pub fn map(self) -> &'static str {
        match self {
            Self::Devgulch => "devgulch",
            Self::BloodGulch | Self::BloodGulch16 => "bloodgulch",
        }
    }

    /// Its players, the local one first.
    pub fn players(self) -> usize {
        match self {
            Self::Devgulch | Self::BloodGulch => 1,
            Self::BloodGulch16 => 16,
        }
    }
}

/// The path's name.
#[uniffi::export]
pub fn benchmark_path_name(path: BenchmarkPath) -> String {
    path.name().into()
}

/// The path named `name` (devgulch, bloodgulch, bloodgulch16).
#[uniffi::export]
pub fn benchmark_path_named(name: String) -> Option<BenchmarkPath> {
    BenchmarkPath::named(&name)
}

/// The map a path plays.
#[uniffi::export]
pub fn benchmark_path_map(path: BenchmarkPath) -> String {
    path.map().into()
}

/// A path being played: each tick's inputs for its players.
#[derive(Debug)]
pub(crate) struct Run {
    tick: u32,
    inputs: Vec<TickInput>,
}

// (a phrase of the script is two seconds; the turns are radians a tick)
const PHRASE: u32 = 120;
const TURN: f32 = 0.012;
const LOOK: f32 = 0.002;

impl Run {
    pub(crate) fn new() -> Self {
        Self {
            tick: 0,
            inputs: Vec::new(),
        }
    }

    /// The inputs of the next tick for `players` players, in their order.
    pub(crate) fn inputs(&mut self, players: usize) -> &[TickInput] {
        let tick = self.tick;
        self.tick = self.tick.wrapping_add(1);
        self.inputs.clear();
        self.inputs
            .extend((0..players).map(|player| scripted(tick, player)));
        &self.inputs
    }
}

/// Player `player`'s input at `tick`: runs forward and strafes, turning
/// one way then the other, looks up and down, fires in bursts, jumps now
/// and then and throws a grenade once a phrase; each player is a phrase's
/// part later than the one before, so they spread out.
pub(crate) fn scripted(tick: u32, player: usize) -> TickInput {
    let offset = u32::try_from(player).unwrap_or(0).wrapping_mul(53);
    let at = tick.wrapping_add(offset);
    let phrase = at / PHRASE;
    let within = at % PHRASE;
    let mut buttons = Buttons::default();
    // (a third of each phrase firing, a jump at its middle, a grenade at
    // every fourth phrase's end)
    if within < PHRASE / 3 {
        buttons.0 |= Buttons::FIRE.0;
    }
    if within == PHRASE / 2 {
        buttons.0 |= Buttons::JUMP.0;
    }
    if phrase % 4 == 3 && within == PHRASE - 1 {
        buttons.0 |= Buttons::GRENADE.0;
    }
    let sign = |odd: bool| if odd { -1.0 } else { 1.0 };
    TickInput {
        buttons,
        forward: 127,
        right: match (phrase % 3, phrase % 2) {
            (0, _) => 0,
            (_, 1) => -90,
            _ => 90,
        },
        up: 0,
        yaw: Radians(TURN * sign(phrase % 2 == 1)),
        pitch: Radians(LOOK * sign((at / (PHRASE / 2)) % 2 == 1)),
    }
}

/// A run's frame times, summed up.
#[derive(Clone, Debug, PartialEq, uniffi::Record)]
pub struct BenchmarkReport {
    /// The frames counted.
    pub frames: u32,
    /// The mean frame time, in milliseconds.
    pub mean_ms: f64,
    /// The median.
    pub p50_ms: f64,
    /// The 99th percentile.
    pub p99_ms: f64,
    /// The 99.9th percentile.
    pub p999_ms: f64,
    /// The frames over twice the target.
    pub hitches: u32,
    /// The frame pacing: the root mean square of the change from one
    /// frame's time to the next's, in milliseconds (0 when every frame
    /// takes as long as the one before).
    pub pacing_ms: f64,
}

/// The `percentile` (0 to 100) of `sorted`, by the nearest rank, as the
/// app's benchmark has always taken it.
fn percentile(percentile: f64, sorted: &[f64]) -> f64 {
    let Some(&last) = sorted.last() else {
        return 0.0;
    };
    // (less a hair, as 99.9% of 1000 is a hair over 999 in floating point)
    let rank = libm::ceil(percentile / 100.0 * sorted.len() as f64 - 1e-9) as usize;
    sorted.get(rank.saturating_sub(1)).copied().unwrap_or(last)
}

/// The report of `frame_times` (milliseconds, in the order drawn), whose
/// hitches are the frames over twice `target_ms`.
#[uniffi::export]
pub fn benchmark_report(frame_times: Vec<f64>, target_ms: f64) -> BenchmarkReport {
    let times: Vec<f64> = frame_times.into_iter().filter(|t| t.is_finite()).collect();
    let mut sorted = times.clone();
    sorted.sort_by(f64::total_cmp);
    let count = times.len();
    let mean = if count == 0 {
        0.0
    } else {
        times.iter().sum::<f64>() / count as f64
    };
    let changes: Vec<f64> = times.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let pacing = if changes.is_empty() {
        0.0
    } else {
        libm::sqrt(changes.iter().map(|change| change * change).sum::<f64>() / changes.len() as f64)
    };
    BenchmarkReport {
        frames: u32::try_from(count).unwrap_or(u32::MAX),
        mean_ms: mean,
        p50_ms: percentile(50.0, &sorted),
        p99_ms: percentile(99.0, &sorted),
        p999_ms: percentile(99.9, &sorted),
        hitches: u32::try_from(times.iter().filter(|&&t| t > 2.0 * target_ms).count())
            .unwrap_or(u32::MAX),
        pacing_ms: pacing,
    }
}

/// The share a measure may grow by from the run kept before.
pub const REGRESSION: f64 = 0.05;

/// What `current` lost against `baseline`: each of the mean, the p99 and
/// the p99.9 more than 5% slower, and any hitch more, one line each.
#[uniffi::export]
pub fn benchmark_regressions(baseline: BenchmarkReport, current: BenchmarkReport) -> Vec<String> {
    let mut lost = Vec::new();
    for (name, before, now) in [
        ("mean", baseline.mean_ms, current.mean_ms),
        ("p99", baseline.p99_ms, current.p99_ms),
        ("p99.9", baseline.p999_ms, current.p999_ms),
    ] {
        if now > before * (1.0 + REGRESSION) {
            lost.push(format!(
                "{name} {now:.3} ms is {:.1}% over {before:.3} ms",
                (now / before - 1.0) * 100.0
            ));
        }
    }
    if current.hitches > baseline.hitches {
        lost.push(format!(
            "{} hitches, {} before",
            current.hitches, baseline.hitches
        ));
    }
    lost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paths_are_named_and_counted() {
        for path in BenchmarkPath::ALL {
            assert_eq!(BenchmarkPath::named(path.name()), Some(path));
        }
        assert_eq!(BenchmarkPath::BloodGulch16.players(), 16);
        assert_eq!(BenchmarkPath::Devgulch.map(), "devgulch");
        assert_eq!(BenchmarkPath::named("a10"), None);
    }

    #[test]
    fn the_script_is_the_same_every_run_and_moves_fires_and_jumps() {
        let mut first = Run::new();
        let mut second = Run::new();
        let mut fired = 0;
        let mut jumped = 0;
        let mut threw = 0;
        for _ in 0..3600 {
            let a = first.inputs(16).to_vec();
            assert_eq!(a, second.inputs(16));
            assert_eq!(a.len(), 16);
            fired += usize::from(a[0].buttons.contains(Buttons::FIRE));
            jumped += usize::from(a[0].buttons.contains(Buttons::JUMP));
            threw += usize::from(a[0].buttons.contains(Buttons::GRENADE));
            assert_eq!(a[0].forward, 127);
        }
        assert_eq!(fired, 3600 / 3);
        assert_eq!(jumped, 30);
        assert!(threw >= 7, "{threw} grenades");
        // (the players are not in step)
        assert_ne!(scripted(0, 0), scripted(0, 1));
    }

    #[test]
    fn the_report_has_the_mean_percentiles_hitches_and_pacing() {
        // (999 frames of 8 ms, one of 20: a hitch at a target of 8.33)
        let mut times = vec![8.0; 999];
        times.insert(500, 20.0);
        let report = benchmark_report(times, 8.33);
        assert_eq!(report.frames, 1000);
        assert!((report.mean_ms - 8.012).abs() < 1e-9, "{}", report.mean_ms);
        assert_eq!(report.p50_ms, 8.0);
        assert_eq!(report.p99_ms, 8.0);
        assert_eq!(report.p999_ms, 8.0);
        assert_eq!(report.hitches, 1);
        // (two changes of 12 over 999)
        assert!((report.pacing_ms - libm::sqrt(288.0 / 999.0)).abs() < 1e-9);
        let steady = benchmark_report(vec![5.0; 100], 8.33);
        assert_eq!(steady.pacing_ms, 0.0);
        assert_eq!(steady.hitches, 0);
        assert_eq!(benchmark_report(Vec::new(), 8.33).frames, 0);
    }

    #[test]
    fn the_p999_finds_the_slowest_of_a_thousand() {
        let mut times: Vec<f64> = (1..=1000).map(f64::from).collect();
        times.reverse();
        let report = benchmark_report(times, 1000.0);
        assert_eq!(report.p50_ms, 500.0);
        assert_eq!(report.p99_ms, 990.0);
        assert_eq!(report.p999_ms, 999.0);
    }

    #[test]
    fn a_regression_over_five_percent_fails() {
        let baseline = benchmark_report(vec![8.0; 100], 8.33);
        let within = benchmark_report(vec![8.0 * 1.04; 100], 8.33);
        assert!(benchmark_regressions(baseline.clone(), within).is_empty());
        let over = benchmark_report(vec![8.0 * 1.06; 100], 8.33);
        let lost = benchmark_regressions(baseline.clone(), over);
        assert_eq!(lost.len(), 3, "{lost:?}");
        assert!(lost[0].starts_with("mean"), "{lost:?}");
        let mut hitched = vec![8.0; 100];
        hitched[50] = 30.0;
        let lost = benchmark_regressions(baseline, benchmark_report(hitched, 8.33));
        assert!(lost.iter().any(|line| line.contains("hitches")), "{lost:?}");
    }
}
