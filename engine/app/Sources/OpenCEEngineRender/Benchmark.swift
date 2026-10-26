// The benchmark: one of the engine's benchmark paths (devgulch, Blood
// Gulch, Blood Gulch with sixteen players: crates/ffi/src/benchmark.rs)
// played and drawn offscreen for a set time, as fast as the GPU goes, with
// up to three frames in flight as the window has. Each frame asks its
// caller for the world at that time. A frame's time is the time between
// the ends of two frames' GPU work, so it counts whichever of the CPU and
// the GPU is slower.

import Foundation
import Metal
import OpenCEEngineCore
import QuartzCore
import simd

/// The benchmark's frame times and what they were drawn at.
public struct BenchmarkResult: Sendable {
    public let width: Int
    public let height: Int
    public let seconds: Double
    /// Each frame's time, in milliseconds.
    public let frameTimes: [Double]
    /// Each frame's GPU time (from its first command to its last), in milliseconds.
    public let gpuTimes: [Double]

    /// The `percentile` (0 to 100) of `times`, by the nearest rank.
    public static func percentile(_ percentile: Double, of times: [Double]) -> Double {
        guard !times.isEmpty else { return 0 }
        let sorted = times.sorted()
        let rank = Int((percentile / 100 * Double(sorted.count)).rounded(.up)) - 1
        return sorted[min(max(rank, 0), sorted.count - 1)]
    }

    public var p50: Double { Self.percentile(50, of: frameTimes) }
    public var p99: Double { Self.percentile(99, of: frameTimes) }
    public var gpuP50: Double { Self.percentile(50, of: gpuTimes) }
    public var gpuP99: Double { Self.percentile(99, of: gpuTimes) }

    /// The frame times' report (the engine's: crates/ffi/src/benchmark.rs),
    /// whose hitches are frames over twice `targetMs`.
    public func report(targetMs: Double) -> BenchmarkReport {
        benchmarkReport(frameTimes: frameTimes, targetMs: targetMs)
    }

    /// The result of `path` as JSON, for tools/test_engine_benchmark.py.
    public func json(path: String, targetMs: Double) -> String {
        func number(_ value: Double) -> String { String(format: "%.3f", value) }
        let report = report(targetMs: targetMs)
        return "{\"path\": \"\(path)\", \"width\": \(width), \"height\": \(height), "
            + "\"seconds\": \(number(seconds)), \"frames\": \(report.frames), "
            + "\"mean_ms\": \(number(report.meanMs)), \"p50_ms\": \(number(report.p50Ms)), "
            + "\"p99_ms\": \(number(report.p99Ms)), \"p999_ms\": \(number(report.p999Ms)), "
            + "\"hitches\": \(report.hitches), \"pacing_ms\": \(number(report.pacingMs)), "
            + "\"gpu_p50_ms\": \(number(gpuP50)), \"gpu_p99_ms\": \(number(gpuP99))}"
    }
}

/// The frames' ends, gathered from the command buffers' completion handlers.
private final class FrameClock: @unchecked Sendable {
    private let lock = NSLock()
    private var lastEnd: CFTimeInterval?
    private(set) var frameTimes: [Double] = []
    private(set) var gpuTimes: [Double] = []

    func completed(start: CFTimeInterval, end: CFTimeInterval) {
        lock.lock()
        defer { lock.unlock() }
        if let lastEnd {
            frameTimes.append((end - lastEnd) * 1000)
        }
        gpuTimes.append((end - start) * 1000)
        lastEnd = end
    }

    var results: ([Double], [Double]) {
        lock.lock()
        defer { lock.unlock() }
        return (frameTimes, gpuTimes)
    }
}

extension LevelRenderer {
    /// Draws for `seconds` at `width` by `height`, after `warmUp` frames
    /// that are not counted (the pipelines' and the temporal history's
    /// first use). Before each frame `world` is given the seconds since the
    /// counting began (none while warming up) and sets the renderer's
    /// lights, decals and particles, giving back the camera and the objects.
    public func benchmark(
        width: Int, height: Int, seconds: Double, warmUp: Int = 60, queue: MTLCommandQueue,
        world: (_ elapsed: Double?) throws -> (camera: ViewCamera, objects: [DrawnObject])
    ) throws -> BenchmarkResult {
        let descriptor = MTLTextureDescriptor.texture2DDescriptor(
            pixelFormat: outputFormat, width: width,
            height: height, mipmapped: false)
        descriptor.usage = [.renderTarget, .shaderRead]
        descriptor.storageMode = .private
        let inFlight = 3
        var targets: [MTLTexture] = []
        for _ in 0..<inFlight {
            guard let target = device.makeTexture(descriptor: descriptor) else { throw FrameCaptureError.resources }
            targets.append(target)
        }
        let slots = DispatchSemaphore(value: inFlight)
        let clock = FrameClock()
        var frame = 0
        var started: CFTimeInterval?
        var last: MTLCommandBuffer?
        while true {
            let now = CACurrentMediaTime()
            if frame == warmUp {
                last?.waitUntilCompleted()
                started = CACurrentMediaTime()
            }
            let elapsed = started.map { now - $0 } ?? 0
            if started != nil, elapsed >= seconds { break }
            // (the world before the slot: a throw leaves the slots whole)
            let (camera, objects) = try world(started.map { _ in elapsed })
            slots.wait()
            guard let commands = queue.makeCommandBuffer() else {
                slots.signal()
                throw FrameCaptureError.resources
            }
            let counted = started != nil
            commands.addCompletedHandler { buffer in
                if counted {
                    clock.completed(start: buffer.gpuStartTime, end: buffer.gpuEndTime)
                }
                slots.signal()
            }
            encode(into: commands, target: targets[frame % inFlight], camera: camera, objects: objects)
            commands.commit()
            last = commands
            frame += 1
        }
        // (every frame's handler has run once its slot is back)
        // (and given back: a semaphore freed below its first value traps)
        for _ in 0..<inFlight { slots.wait() }
        for _ in 0..<inFlight { slots.signal() }
        let (frameTimes, gpuTimes) = clock.results
        return BenchmarkResult(
            width: width, height: height, seconds: seconds, frameTimes: frameTimes,
            gpuTimes: gpuTimes)
    }
}
