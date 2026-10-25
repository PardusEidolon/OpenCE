import Foundation
import Metal
import OpenCEEngineCore
import OpenCEEngineKit
import OpenCEEngineRender
import XCTest

/// The insides of Blood Gulch's two bases (devgulch's; needs assets/maps),
/// lit by their lamps and the light that comes in at the doors: each texel
/// is traced by enough paths that the walls' light is smooth, not blotched
/// by the tracer's noise, so neighbouring texels under the roof step hard no
/// more often than on the stock lightmaps; and no light comes through the
/// walls, so the texels that only the building's own surfaces light (out
/// of the sky's and the sun's sight, and of the lamps' reach) are lit well
/// past the stock texels over them hardly anywhere. A view of each base's inside is
/// written to build/engine/golden (beside the stock view's) and compared
/// with its approved frame, if there is one, in assets/golden/engine.
final class BunkerTests: XCTestCase {
    /// The bases: a box about each, from the floor to under the roof's top
    /// (which is outdoors), and a point inside to look from.
    let bases: [(name: String, low: [Float], high: [Float], inside: SIMD3<Float>)] = [
        ("red", [85, -168, -1], [106, -151, 4], SIMD3(95.5, -159.5, 0.4)),
        ("blue", [30, -87, -1], [50, -71, 4], SIMD3(40.1, -79.1, 0.4)),
    ]

    func testTheBasesInsidesAreSmoothAndLitOnlyFromInside() throws {
        let level = try StockLevel(map: "bloodgulch")
        let plan = try level.geometry.bakePlan(quick: false)
        let light = try LightBaker.bake(plan, tracer: PathTracer(device: level.device))
        let out = StockLevel.root.appendingPathComponent("build/engine/golden")
        try FileManager.default.createDirectory(at: out, withIntermediateDirectories: true)
        for base in bases {
            let inside = plan.sheltered(light: light, low: base.low, high: base.high)
            let steps = inside.edges
            XCTAssertGreaterThan(steps.pairs, 1000, "the \(base.name) base has texels under its roof")
            XCTAssertGreaterThan(steps.stockPairs, 100, "and so do its stock pages")
            let baked = Float(steps.hard) / Float(steps.pairs)
            let stock = Float(steps.stockHard) / Float(steps.stockPairs)
            let leaks = Float(inside.leaks) / Float(max(inside.texels, 1))
            print(
                "\(base.name): baked \(steps.hard)/\(steps.pairs) hard (\(baked)), mean \(steps.mean); "
                    + "stock \(steps.stockHard)/\(steps.stockPairs) hard (\(stock)), mean \(steps.stockMean); "
                    + "\(inside.leaks)/\(inside.texels) lit only from inside past stock (\(leaks))")
            XCTAssertLessThanOrEqual(baked, stock, "the \(base.name) base's share of neighbours that step hard")
            XCTAssertLessThanOrEqual(steps.mean, steps.stockMean, "the \(base.name) base's mean step")
            XCTAssertGreaterThan(inside.texels, 100, "the \(base.name) base has texels lit only from inside")
            XCTAssertLessThanOrEqual(leaks, 0.01, "the \(base.name) base's texels lit only from inside past stock")

            // the inside, looking along the base's length
            let camera = ViewCamera(position: base.inside, yaw: 0, pitch: 0, verticalFov: 70 * .pi / 180)
            var shots: [Bool: CapturedFrame] = [:]
            for stock in [false, true] {
                let renderer = try level.renderer(VideoSettings(antiAliasing: .native, renderScale: 1))
                try renderer.useBakedLighting(light)
                renderer.switches.view = stock ? .stock : .frame
                shots[stock] = try renderer.capture(
                    width: 480, height: 270, camera: camera, frames: 8,
                    objects: level.objects, queue: level.queue)
            }
            let file = "devgulch_\(base.name)_base_480x270.png"
            try shots[false]!.writePNG(to: out.appendingPathComponent(file))
            try shots[true]!.writePNG(to: out.appendingPathComponent("devgulch_\(base.name)_base_stock_480x270.png"))
            let approved = StockLevel.root.appendingPathComponent("assets/golden/engine/\(file)")
            if let approvedFrame = CapturedFrame.readPNG(at: approved) {
                let difference = try XCTUnwrap(shots[false]!.difference(from: approvedFrame), file)
                XCTAssertLessThan(difference.share, 0.005, "\(file) differs from the approved frame")
            } else {
                print("golden frame \(file): none approved yet (assets/golden/engine)")
            }
        }
    }
}
