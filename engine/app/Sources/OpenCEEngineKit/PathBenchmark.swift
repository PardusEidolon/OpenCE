// A benchmark path played through a loaded level and drawn offscreen: the
// one the --benchmark command and the tier tests both measure.
import Metal
import OpenCEEngineCore
import OpenCEEngineRender

extension LoadedLevel {
    /// Plays `path` through the level for `seconds`, drawing each frame at
    /// `width` by `height`, after `warmUp` frames not counted. The game runs
    /// at the frames' own pace, a tick a frame while warming up, so that
    /// each second counted plays the same second of the path and a frame
    /// carries only the ticks the game would give it.
    public func benchmark(
        path: BenchmarkPath, width: Int, height: Int, seconds: Double, warmUp: Int = 60,
        queue: MTLCommandQueue
    ) throws -> BenchmarkResult {
        let engine = Engine()
        let snapshots = SnapshotReader(engine: engine)
        _ = try engine.startLevel(geometry: geometry, difficulty: 1)
        _ = try engine.startBenchmark(path: path)
        let renderer = renderer
        var played = 0.0
        return try renderer.benchmark(
            width: width, height: height, seconds: seconds, warmUp: warmUp, queue: queue
        ) { elapsed in
            let now = elapsed ?? 0
            _ = try engine.advance(frameSeconds: elapsed == nil ? 1.0 / 60 : max(now - played, 0))
            played = now
            let frame = snapshots.acquire()
            renderer.time = Float((Double(frame.tick) + Double(frame.alpha)) / 60)
            renderer.lights = frame.drawnLights()
            renderer.laidDecals = Array(frame.decals)
            renderer.laidParticles = Array(frame.particles)
            return (ViewCamera(frame.camera, alpha: frame.alpha), frame.drawnObjects())
        }
    }
}
