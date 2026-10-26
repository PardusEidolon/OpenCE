//! The fixed tick, and how far the display is between the last two ticks
//! (`game_time_update`).
//!
//! The game turns each frame's time into whole ticks as it does: the
//! frame's seconds and the leftover of the last frame, times the rate the
//! game's speed sets, floored; at most a cap of ticks a frame, the time past
//! the cap thrown away; and what is short of a whole tick kept, in seconds,
//! for the next frame. The game ticks 30 times a second with a cap of 7;
//! the engine ticks 60 times a second with a cap of 14, the same seven
//! game ticks.

use crate::units::Seconds;

/// The simulation's rate.
pub const TICKS_PER_SECOND: u32 = 60;
/// One tick.
pub const TICK: Seconds = Seconds(1.0 / TICKS_PER_SECOND as f32);

/// The most ticks one frame runs: after a long stall (a breakpoint, the app
/// in the background) the world slows down rather than running a burst of
/// ticks that each make the next frame later still. The game's seven
/// ticks of a local game, at the engine's rate.
pub const MAX_TICKS_PER_FRAME: u32 = 14;

/// The most ticks the game counts in one frame before its cap
/// (`SOME_LARGE_NUMBER_OF_TICKS`).
const SOME_LARGE_NUMBER_OF_TICKS: f32 = 1000.0;

/// Turns the display's frame times into whole ticks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Clock {
    /// The ticks a second at the game's normal speed.
    ticks_per_second: f32,
    /// The most ticks a frame runs.
    cap: u32,
    /// The time short of a whole tick, in seconds (`leftover_dt`).
    leftover: f32,
}

impl Default for Clock {
    fn default() -> Self {
        Self::at_rate(TICKS_PER_SECOND, MAX_TICKS_PER_FRAME)
    }
}

/// What a frame should do.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    /// The ticks to run now.
    pub ticks: u32,
    /// Where the display is between the tick before the last (0) and the
    /// last (1).
    pub alpha: f32,
}

impl Clock {
    /// A clock of `ticks_per_second` that runs at most `cap` ticks a frame,
    /// throwing away the time past it, as a local game's does.
    pub fn at_rate(ticks_per_second: u32, cap: u32) -> Self {
        Self {
            ticks_per_second: ticks_per_second as f32,
            cap,
            leftover: 0.0,
        }
    }

    /// Adds a frame's time at the game's `speed` (1 normal, 0 paused) and
    /// says how many ticks to run (`game_time_update`). A frame time or
    /// speed that is not a finite, non-negative number runs nothing.
    pub fn advance(&mut self, frame_seconds: f64, speed: f64) -> Frame {
        let usable = |x: f64| x.is_finite() && x >= 0.0;
        let (frame_seconds, speed) = if usable(frame_seconds) && usable(speed) {
            (frame_seconds as f32, speed as f32)
        } else {
            (0.0, 0.0)
        };
        let ticks_per_second = speed * self.ticks_per_second;
        let mut ticks = 0;
        if ticks_per_second > 0.0 {
            let mut game_time = frame_seconds + self.leftover;
            let ticks_real = libm::floorf(game_time * ticks_per_second);
            // (finite and non-negative, at most a thousand)
            ticks = ticks_real.min(SOME_LARGE_NUMBER_OF_TICKS) as u32;
            if ticks > self.cap {
                ticks = self.cap;
                game_time = ticks_real / ticks_per_second;
            }
            self.leftover = (game_time - ticks_real / ticks_per_second).max(0.0);
        }
        Frame {
            ticks,
            // (a fraction just short of a tick rounds to 1.0; the display
            // is never at the next tick yet)
            alpha: (self.leftover * self.ticks_per_second).min(1.0 - f32::EPSILON / 2.0),
        }
    }

    /// The time short of a whole tick, in seconds.
    pub fn leftover(&self) -> f32 {
        self.leftover
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(frames: &[f64]) -> (u32, f32) {
        let mut clock = Clock::default();
        let mut ticks = 0;
        let mut alpha = 0.0;
        for &frame in frames {
            let step = clock.advance(frame, 1.0);
            ticks += step.ticks;
            alpha = step.alpha;
        }
        (ticks, alpha)
    }

    #[test]
    fn a_second_of_frames_at_any_rate_is_sixty_ticks() {
        for rate in [30.0, 60.0, 120.0, 144.0, 240.0] {
            let frames = vec![1.0 / rate; rate as usize];
            let (ticks, alpha) = run(&frames);
            // (the last tick lands within rounding of the frame's end)
            assert!(
                ticks == 60 || (ticks == 59 && alpha > 0.999),
                "{rate} Hz: {ticks} ticks, alpha {alpha}"
            );
        }
    }

    #[test]
    fn alpha_is_the_fraction_of_a_tick_left_over() {
        let mut clock = Clock::default();
        let frame = clock.advance(1.5 / 60.0, 1.0);
        assert_eq!(frame.ticks, 1);
        assert!((frame.alpha - 0.5).abs() < 1e-5);
        let frame = clock.advance(0.25 / 60.0, 1.0);
        assert_eq!(frame.ticks, 0);
        assert!((frame.alpha - 0.75).abs() < 1e-5);
    }

    #[test]
    fn uneven_frames_keep_alpha_in_range_and_lose_little_time() {
        // a frame pattern from a display that misses deadlines
        let pattern = [0.004, 0.021, 0.0083, 0.0167, 0.033, 0.001, 0.012];
        let mut clock = Clock::default();
        let mut ticks = 0u32;
        let mut total = 0.0;
        for i in 0..7000 {
            let seconds = pattern[i % pattern.len()];
            total += seconds;
            let frame = clock.advance(seconds, 1.0);
            assert!((0.0..1.0).contains(&frame.alpha), "alpha {}", frame.alpha);
            ticks += frame.ticks;
        }
        // (the leftover is kept in f32 seconds, as the game keeps it: a
        // frame's rounding is a few millionths of a tick)
        let expected = total * 60.0;
        let counted = f64::from(ticks) + f64::from(clock.leftover() * 60.0);
        assert!(
            (counted - expected).abs() < 0.05,
            "{counted} for {expected}"
        );
    }

    #[test]
    fn a_stall_runs_at_most_the_cap_and_throws_the_rest_away() {
        let mut clock = Clock::default();
        assert_eq!(
            clock.advance(10.0, 1.0),
            Frame {
                ticks: MAX_TICKS_PER_FRAME,
                alpha: 0.0
            }
        );
    }

    #[test]
    fn a_frame_of_exactly_the_cap_keeps_its_leftover() {
        // (the game caps only above the cap)
        let mut clock = Clock::at_rate(30, 7);
        let frame = clock.advance(7.5 / 30.0, 1.0);
        assert_eq!(frame.ticks, 7);
        assert!((frame.alpha - 0.5).abs() < 1e-4, "{}", frame.alpha);
    }

    #[test]
    fn the_games_speed_scales_the_rate() {
        let mut clock = Clock::default();
        assert_eq!(clock.advance(0.1, 0.5).ticks, 3);
        assert_eq!(clock.advance(0.1, 0.0).ticks, 0);
        assert_eq!(clock.advance(0.1, 2.0).ticks, 12);
    }

    #[test]
    fn nonsense_frame_times_run_nothing() {
        let mut clock = Clock::default();
        for seconds in [f64::NAN, f64::INFINITY, -1.0, 0.0] {
            assert_eq!(clock.advance(seconds, 1.0).ticks, 0);
        }
        assert_eq!(clock.advance(1.0, f64::NAN).ticks, 0);
    }

    #[test]
    fn alpha_stays_below_one() {
        let mut clock = Clock::default();
        for _ in 0..100_000 {
            let frame = clock.advance(1.0 / 144.0, 1.0);
            assert!((0.0..1.0).contains(&frame.alpha), "{}", frame.alpha);
        }
    }
}
