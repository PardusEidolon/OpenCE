import Metal
import XCTest

import OpenCEEngineCore
import OpenCEEngineKit
@testable import OpenCEEngineRender

/// Dynamic resolution: the controller settles on a synthetic cost without
/// hunting and steps back up when the load falls, the renderer steps
/// between its scales without making anything in play, and on Blood Gulch
/// at 4K it holds a 90 Hz frame.
final class DynamicResolutionTests: XCTestCase {
    override func tearDown() {
        HitchLog.shared.playing = false
        HitchLog.shared.reset()
        super.tearDown()
    }

    /// `frames` frames of `cost` at the controller's scale, with a ±5%
    /// deterministic wobble; the number of changes.
    func run(
        _ controller: inout DynamicResolution, frames: Int, cost: (Float) -> Double
    ) -> Int {
        var changes = 0
        for frame in 0..<frames {
            let wobble = 1 + 0.05 * sin(Double(frame) * 0.7)
            if controller.record(milliseconds: cost(controller.scale) * wobble) { changes += 1 }
        }
        return changes
    }

    func heavy(_ scale: Float) -> Double { 3 + 10 * Double(scale * scale) }
    func light(_ scale: Float) -> Double { 1 + 4 * Double(scale * scale) }

    func testTheControllerSettlesWithoutHunting() {
        var controller = DynamicResolution(targetMilliseconds: 10, maximum: 1)
        XCTAssertEqual(controller.steps, [1, 0.67, 0.5])
        let changes = run(&controller, frames: 3000, cost: heavy)
        // (13 ms at 1, 7.5 at 0.67: the step above is predicted too long)
        XCTAssertEqual(controller.scale, 0.67)
        XCTAssertLessThanOrEqual(changes, 2)
    }

    func testTheControllerStepsBackUpWhenTheLoadFalls() {
        var controller = DynamicResolution(targetMilliseconds: 10, maximum: 1)
        _ = run(&controller, frames: 600, cost: heavy)
        XCTAssertLessThan(controller.scale, 1)
        let changes = run(&controller, frames: 600, cost: light)
        XCTAssertEqual(controller.scale, 1)
        XCTAssertLessThanOrEqual(changes, 2)
        // (and stays)
        XCTAssertEqual(run(&controller, frames: 1200, cost: light), 0)
    }

    func testTheStepsAreBelowTheMaximum() {
        XCTAssertEqual(DynamicResolution(targetMilliseconds: 10, maximum: 0.8).steps, [0.8, 0.67, 0.5])
        XCTAssertEqual(DynamicResolution(targetMilliseconds: 10, maximum: 0.67).steps, [0.67, 0.5])
        XCTAssertEqual(DynamicResolution(targetMilliseconds: 10, maximum: 0.2).steps, [0.5])
        XCTAssertEqual(DynamicResolution(targetMilliseconds: 10, maximum: .nan).steps, [1, 0.67, 0.5])
    }

    func testInvalidTimesAreIgnored() {
        var controller = DynamicResolution(targetMilliseconds: 10, maximum: 1)
        let start = controller
        for time in [Double.nan, .infinity, -1, 0] {
            for _ in 0..<100 { XCTAssertFalse(controller.record(milliseconds: time)) }
        }
        XCTAssertEqual(controller, start)
    }

    func testTheSettingsMakeAControllerOnlyForTemporal() throws {
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice(), "no Metal device")
        let geometry = try LevelGeometry.devmap(donor: nil, cacheFolder: "")
        let on = VideoSettings(antiAliasing: .temporal, renderScale: 0.8, dynamicResolution: true)
        let renderer = try LevelRenderer(device: device, geometry: geometry, settings: on)
        XCTAssertEqual(renderer.dynamic?.steps, [0.8, 0.67, 0.5])
        renderer.settings = VideoSettings(antiAliasing: .native, dynamicResolution: true)
        XCTAssertNil(renderer.dynamic)
        renderer.settings = VideoSettings(antiAliasing: .temporal)
        XCTAssertNil(renderer.dynamic)
    }

    /// After the load's preparation, long frames step the renderer down
    /// through each scale and short ones back up, drawing each at its own
    /// size, and nothing is made while it plays.
    func testTheRendererStepsBetweenScalesWithoutMakingAnything() throws {
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice(), "no Metal device")
        try XCTSkipUnless(TemporalScaling.isSupported(device), "no MetalFX temporal scaler")
        let queue = try XCTUnwrap(device.makeCommandQueue())
        let geometry = try LevelGeometry.devmap(donor: nil, cacheFolder: "")
        let settings = VideoSettings(
            antiAliasing: .temporal, renderScale: 1, dynamicResolution: true, frameTargetMilliseconds: 10)
        let renderer = try LevelRenderer(device: device, geometry: geometry, settings: settings)
        let (width, height) = (320, 180)
        let start = geometry.startView()
        try LoadedLevel.prepare(renderer, width: width, height: height, from: start)
        let descriptor = MTLTextureDescriptor.texture2DDescriptor(
            pixelFormat: renderer.outputFormat, width: width, height: height, mipmapped: false)
        descriptor.usage = [.renderTarget]
        descriptor.storageMode = .private
        let target = try XCTUnwrap(device.makeTexture(descriptor: descriptor))
        let camera = ViewCamera(
            position: [start.x, start.y, start.z], yaw: start.yaw, pitch: start.pitch,
            verticalFov: 70 * .pi / 180)
        HitchLog.shared.reset()
        HitchLog.shared.playing = true
        var seen: [Float] = []
        func play(milliseconds: Double, until scale: Float) throws {
            for _ in 0..<400 {
                renderer.gpuFrameTimes.add(milliseconds: milliseconds)
                let commands = try XCTUnwrap(queue.makeCommandBuffer())
                renderer.encode(into: commands, target: target, camera: camera)
                commands.commit()
                commands.waitUntilCompleted()
                let drawn = try XCTUnwrap(renderer.dynamic?.scale)
                let size = settings.inputSize(width: width, height: height, scale: drawn)
                XCTAssertTrue(renderer.lastInputSize == size, "\(renderer.lastInputSize) at \(drawn)")
                if seen.last != drawn { seen.append(drawn) }
                if drawn == scale { return }
            }
            XCTFail("never reached \(scale): \(seen)")
        }
        try play(milliseconds: 100, until: 0.5)
        try play(milliseconds: 0.1, until: 1)
        HitchLog.shared.playing = false
        XCTAssertEqual(seen, [1, 0.67, 0.5, 0.67, 1])
        XCTAssertEqual(HitchLog.shared.take(), [])
    }

    /// Blood Gulch's benchmark path at 4K, held to 90 Hz: about 13 ms a
    /// frame at full scale, so it settles below it and meets the target
    /// (skipped without assets/maps).
    func testBloodGulchAt4KHoldsTheTarget() throws {
        let maps = StockLevel.root.appendingPathComponent("assets/maps")
        guard FileManager.default.fileExists(atPath: maps.appendingPathComponent("bloodgulch.map").path) else {
            throw XCTSkip("no assets/maps/bloodgulch.map")
        }
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice(), "no Metal device")
        try XCTSkipUnless(TemporalScaling.isSupported(device), "no MetalFX temporal scaler")
        let queue = try XCTUnwrap(device.makeCommandQueue())
        let cache = StockLevel.root.appendingPathComponent("build/engine/cache")
        try FileManager.default.createDirectory(at: cache, withIntermediateDirectories: true)
        let size = (width: 3840, height: 2160)
        let target = 1000.0 / 90
        let level = try LoadedLevel(
            map: maps.appendingPathComponent("bloodgulch.map").path, cacheFolder: cache.path, device: device,
            settings: VideoSettings(
                antiAliasing: .temporal, renderScale: 1, dynamicResolution: true,
                frameTargetMilliseconds: target),
            bakeAtLoad: .cachedOnly, prepare: size)
        let engine = Engine()
        let snapshots = SnapshotReader(engine: engine)
        _ = try engine.startLevel(geometry: level.geometry, difficulty: 1)
        _ = try engine.startBenchmark(path: .bloodGulch)
        let renderer = level.renderer
        HitchLog.shared.reset()
        // (a long warm-up: the controller's windows settle first)
        let result = try renderer.benchmark(
            width: size.width, height: size.height, seconds: 4, warmUp: 360, queue: queue
        ) { elapsed in
            HitchLog.shared.playing = elapsed != nil
            _ = try engine.advance(frameSeconds: 1.0 / 60)
            let frame = snapshots.acquire()
            renderer.time = Float((Double(frame.tick) + Double(frame.alpha)) / 60)
            renderer.lights = frame.drawnLights()
            renderer.laidDecals = Array(frame.decals)
            renderer.laidParticles = Array(frame.particles)
            return (ViewCamera(frame.camera, alpha: frame.alpha), frame.drawnObjects())
        }
        HitchLog.shared.playing = false
        let report = result.report(targetMs: target)
        let scale = try XCTUnwrap(renderer.dynamic?.scale)
        print(
            "dynamic resolution, Blood Gulch 4K at 90 Hz: scale \(scale), mean \(report.meanMs) ms, "
                + "p99 \(report.p99Ms) ms")
        XCTAssertLessThan(scale, 1)
        XCTAssertLessThanOrEqual(report.meanMs, target * 1.05)
        XCTAssertEqual(HitchLog.shared.take(), [])
    }
}
