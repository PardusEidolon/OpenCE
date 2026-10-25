import Foundation
import Metal
import OpenCEEngineCore
import OpenCEEngineKit
import OpenCEEngineRender
import XCTest

/// The far hills and the cliffs (Blood Gulch's; needs assets/maps): a
/// compressed texture's chain ends where its longer side is a block, as the
/// original's does, so the levels below (zeros in the stock maps) are never
/// drawn and the hills' steep and distant triangles, which sample the
/// smallest levels of the micro detail map, are not drawn black. Under the
/// levels below a block, 0.3% of these views' pixels were all but black;
/// now no more than 0.05% are.
final class FarHillsTests: XCTestCase {
    /// The share of the frame's pixels that are all but black.
    func black(_ frame: CapturedFrame) -> Float {
        var count = 0
        for y in 0..<frame.height {
            for x in 0..<frame.width {
                let c = SIMD3<Float>(frame.rgb(x: x, y: y))
                if 0.2126 * c.x + 0.7152 * c.y + 0.0722 * c.z < 16 { count += 1 }
            }
        }
        return Float(count) / Float(max(frame.width * frame.height, 1))
    }

    func testTheFarHillsTrianglesAreNotDrawnBlack() throws {
        let level = try StockLevel(map: "bloodgulch")
        let start = level.geometry.startView()
        let views: [(String, SIMD3<Float>, Float, Float)] = [
            ("east", SIMD3(start.x, start.y, start.z + 30), 0, -0.25),
            ("west", SIMD3(start.x, start.y, start.z + 30), .pi, -0.25),
            ("north", SIMD3(start.x, start.y, start.z + 1), .pi / 2, -0.05),
            ("south", SIMD3(start.x, start.y, start.z + 1), -.pi / 2, -0.05),
        ]
        var share: Float = 0
        for (name, position, yaw, pitch) in views {
            let camera = ViewCamera(position: position, yaw: yaw, pitch: pitch, verticalFov: 70 * .pi / 180)
            let renderer = try level.renderer(VideoSettings(antiAliasing: .native, renderScale: 1))
            renderer.switches.view = .albedo
            let shot = try renderer.capture(width: 960, height: 540, camera: camera, frames: 8, queue: level.queue)
            print("\(name): \(black(shot)) black")
            share += black(shot) / Float(views.count)
        }
        XCTAssertLessThanOrEqual(share, 0.0005, "the share of the albedo's pixels that are all but black")
    }
}
