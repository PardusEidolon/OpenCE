//! The benchmark's paths replay alike: two runs of a path from the same
//! level reach the same world at every second, with the path's players
//! moving (needs assets/maps; passes without it).

use std::path::Path;
use std::sync::Arc;

use opence_ffi::benchmark::BenchmarkPath;
use opence_ffi::{Engine, LevelGeometry};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The path's level, loaded; none without game data.
fn level(path: BenchmarkPath) -> Result<Option<Arc<LevelGeometry>>> {
    let maps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/maps");
    if !maps.join("bloodgulch.map").exists() {
        return Ok(None);
    }
    let cache = Path::new(env!("CARGO_TARGET_TMPDIR")).join("benchmark_paths");
    std::fs::create_dir_all(&cache)?;
    let map = maps.join(format!("{}.map", path.map()));
    Ok(Some(LevelGeometry::load(
        map.to_string_lossy().into(),
        cache.to_string_lossy().into(),
        0,
    )?))
}

/// The world's hash at each of `seconds` seconds of `path`, and the
/// players.
fn run(
    geometry: &Arc<LevelGeometry>,
    path: BenchmarkPath,
    seconds: u32,
) -> Result<(Vec<u64>, u32)> {
    let engine = Engine::new();
    engine.start_level(geometry.clone(), 1)?;
    let players = engine.start_benchmark(path)?;
    let mut hashes = vec![engine.world_hash()];
    for _ in 0..seconds {
        for _ in 0..60 {
            engine.step()?;
        }
        hashes.push(engine.world_hash());
    }
    Ok((hashes, players))
}

fn replays_alike(path: BenchmarkPath, seconds: u32) -> Result {
    let Some(geometry) = level(path)? else {
        return Ok(());
    };
    let (first, players) = run(&geometry, path, seconds)?;
    let (second, _) = run(&geometry, path, seconds)?;
    assert_eq!(players as usize, path.players());
    assert_eq!(first, second, "{} replayed differently", path.name());
    // (the world changes from second to second: the players move)
    for pair in first.windows(2) {
        assert_ne!(pair[0], pair[1], "{} stood still", path.name());
    }
    Ok(())
}

#[test]
fn blood_gulch_with_sixteen_players_replays_alike() -> Result {
    replays_alike(BenchmarkPath::BloodGulch16, 10)
}

#[test]
fn blood_gulch_replays_alike() -> Result {
    replays_alike(BenchmarkPath::BloodGulch, 10)
}

#[test]
fn devgulch_replays_alike() -> Result {
    replays_alike(BenchmarkPath::Devgulch, 10)
}
