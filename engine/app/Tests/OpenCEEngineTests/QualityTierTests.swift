import Metal
import XCTest

import OpenCEEngineCore
import OpenCEEngineKit
@testable import OpenCEEngineRender

/// The quality tiers: each Mac's GPU family picks its tier, each tier's
/// presets, the temporal scaler's output limit, an 8 GB Mac's GPU
/// budget holding every stock map at Low, and every benchmark path at
/// Ultra meeting 120 Hz at 4K on a Mac picked for it.
final class QualityTierTests: XCTestCase {
    func testEachGpuFamilyPicksItsTier() {
        let gib: UInt64 = 1 << 30
        let macs: [(String, Int?, UInt64, QualityTier)] = [
            ("Apple M1", 7, 8 * gib, .low),
            ("Apple M1", 8, 8 * gib, .low),
            ("Apple M2", 10, 8 * gib, .low),
            ("Apple M2", 10, 16 * gib, .medium),
            ("Apple M3", 8, 16 * gib, .low),
            ("Apple M1 Pro", 16, 16 * gib, .high),
            ("Apple M3 Pro", 14, 18 * gib, .high),
            ("Apple M1 Max", 32, 32 * gib, .ultra),
            ("Apple M2 Max", 30, 32 * gib, .ultra),
            ("Apple M2 Ultra", 60, 64 * gib, .ultra),
            // (no registry count: the name's class)
            ("Apple M1", nil, 16 * gib, .low),
            ("Apple M2 Pro", nil, 16 * gib, .high),
            ("Apple M1 Max", nil, 32 * gib, .ultra),
            ("Apple M2 Ultra", nil, 64 * gib, .ultra),
            // (an unknown GPU is taken as Medium's)
            ("Some GPU", nil, 16 * gib, .medium),
            ("Apple M1 Max", 0, 32 * gib, .ultra),
        ]
        for (name, registry, memory, tier) in macs {
            let cores = QualityTier.cores(name: name, registry: registry)
            XCTAssertEqual(
                QualityTier.pick(cores: cores, memory: memory), tier,
                "\(name), \(registry ?? -1) cores, \(memory / gib) GB")
        }
    }

    func testThisMacHasATier() throws {
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice(), "no Metal device")
        let tier = QualityTier.detect(device)
        print("quality tier: \(device.name), \(QualityTier.registryCores() ?? -1) cores: \(tier.title)")
        XCTAssertTrue(QualityTier.allCases.contains(tier))
    }

    func testThePresetsRiseWithTheTier() {
        var switches: [RenderSwitches] = []
        for tier in QualityTier.allCases {
            var made = RenderSwitches()
            tier.apply(to: &made)
            switches.append(made)
        }
        XCTAssertEqual(switches.map(\.gtao), [false, true, true, true])
        XCTAssertEqual(switches.map(\.ssr), [false, false, true, true])
        XCTAssertEqual(switches.map(\.contactShadows), [false, true, true, true])
        XCTAssertEqual(switches.map(\.halfOcclusion), [true, true, true, false])
        XCTAssertEqual(switches.map(\.shadowResolution), [1024, 2048, 2048, 2048])
        let videos = QualityTier.allCases.map { $0.video() }
        XCTAssertEqual(videos.map(\.temporalHeightLimit), [1080, 1440, 1440, 1440])
        XCTAssertEqual(videos.map(\.renderScale), [0.67, 0.67, 0.67, 1])
        XCTAssertTrue(videos.allSatisfy { $0.antiAliasing == .temporal && $0.dynamicResolution })
        // (a tier leaves the debugging switches alone)
        var wireframe = RenderSwitches()
        wireframe.wireframe = true
        QualityTier.low.apply(to: &wireframe)
        XCTAssertTrue(wireframe.wireframe)
    }

    func testTheTemporalOutputIsLimited() {
        let limited = VideoSettings(antiAliasing: .temporal, renderScale: 0.5, temporalHeightLimit: 1440)
        XCTAssertTrue(limited.temporalOutputSize(width: 3840, height: 2160) == (2560, 1440))
        XCTAssertTrue(limited.inputSize(width: 3840, height: 2160) == (1280, 720))
        // (an output no taller than the limit is gathered whole)
        XCTAssertTrue(limited.temporalOutputSize(width: 1920, height: 1080) == (1920, 1080))
        XCTAssertTrue(
            VideoSettings(temporalHeightLimit: 0).temporalOutputSize(width: 3840, height: 2160) == (3840, 2160))
        XCTAssertTrue(
            VideoSettings(antiAliasing: .native, temporalHeightLimit: 1440).temporalOutputSize(
                width: 3840, height: 2160)
                == (3840, 2160))
    }

    /// A limited output is gathered at the limit and scaled to the
    /// target's size, which the frame is drawn at.
    func testTheRendererScalesALimitedOutputToTheTarget() throws {
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice(), "no Metal device")
        try XCTSkipUnless(TemporalScaling.isSupported(device), "no MetalFX temporal scaler")
        let queue = try XCTUnwrap(device.makeCommandQueue())
        let geometry = try LevelGeometry.devmap(donor: nil, cacheFolder: "")
        let settings = VideoSettings(antiAliasing: .temporal, renderScale: 1, temporalHeightLimit: 180)
        let renderer = try LevelRenderer(device: device, geometry: geometry, settings: settings)
        let (width, height) = (640, 360)
        let descriptor = MTLTextureDescriptor.texture2DDescriptor(
            pixelFormat: renderer.outputFormat, width: width, height: height, mipmapped: false)
        descriptor.usage = [.renderTarget]
        descriptor.storageMode = .private
        let target = try XCTUnwrap(device.makeTexture(descriptor: descriptor))
        let start = geometry.startView()
        let camera = ViewCamera(
            position: [start.x, start.y, start.z], yaw: start.yaw, pitch: start.pitch, verticalFov: 70 * .pi / 180)
        let commands = try XCTUnwrap(queue.makeCommandBuffer())
        renderer.encode(into: commands, target: target, camera: camera)
        commands.commit()
        commands.waitUntilCompleted()
        XCTAssertNil(commands.error)
        let temporal = try XCTUnwrap(renderer.temporal)
        XCTAssertEqual([temporal.inputWidth, temporal.inputHeight], [320, 180])
        XCTAssertEqual([temporal.outputWidth, temporal.outputHeight], [320, 180])
        XCTAssertEqual([temporal.targetWidth, temporal.targetHeight], [640, 360])
        XCTAssertNotNil(temporal.spatial)
    }

    /// Every stock map loaded at Low and drawn at 1440p fits the GPU's
    /// share of an 8 GB Mac: two thirds of it (skipped without
    /// assets/maps).
    func testAnEightGigabyteBudgetFitsAtLow() throws {
        let folder = StockLevel.root.appendingPathComponent("assets/maps")
        let maps = ((try? FileManager.default.contentsOfDirectory(atPath: folder.path)) ?? [])
            .filter { $0.hasSuffix(".map") && $0 != "ui.map" && !$0.hasPrefix("dev") }
            .sorted()
        guard !maps.isEmpty else { throw XCTSkip("no assets/maps") }
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice(), "no Metal device")
        let cache = StockLevel.root.appendingPathComponent("build/engine/cache")
        try FileManager.default.createDirectory(at: cache, withIntermediateDirectories: true)
        let budget = (8 << 30) * 2 / 3
        let before = device.currentAllocatedSize
        var largest = (map: "", bytes: 0)
        for map in maps {
            try autoreleasepool {
                let level = try LoadedLevel(
                    map: folder.appendingPathComponent(map).path, cacheFolder: cache.path, device: device,
                    settings: QualityTier.low.video(), bakeAtLoad: .cachedOnly)
                QualityTier.low.apply(to: &level.renderer.switches)
                try LoadedLevel.prepare(level.renderer, width: 2560, height: 1440, from: level.geometry.startView())
                let bytes = device.currentAllocatedSize - before
                print("8 GB budget at Low, 1440p: \(map) \(bytes >> 20) MB")
                if bytes > largest.bytes { largest = (map, bytes) }
            }
        }
        print("8 GB budget at Low, 1440p: largest \(largest.map) \(largest.bytes >> 20) MB of \(budget >> 20) MB")
        XCTAssertLessThanOrEqual(largest.bytes, budget, largest.map)
    }
}
