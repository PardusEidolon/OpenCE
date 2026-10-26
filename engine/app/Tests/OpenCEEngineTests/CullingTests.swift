import Metal
import XCTest
import simd

@testable import OpenCEEngineCore
@testable import OpenCEEngineRender

/// The frustum's culling: a box or sphere is culled only when it is
/// wholly outside one plane, and on Blood Gulch the draws kept are the
/// draws a brute-force test of every vertex finds in view.
final class CullingTests: XCTestCase {
    /// The identity's clip volume is the box from (-1, -1, 0) to (1, 1, 1).
    func testTheFrustumCullsOnlyWhatIsWhollyOutsideAPlane() {
        let view = Frustum(matrix_identity_float4x4)
        XCTAssertTrue(view.contains([0, 0, 0.5]))
        XCTAssertFalse(view.contains([0, 0, -0.5]))
        XCTAssertFalse(view.contains([1.5, 0, 0.5]))
        // (a box across a plane is kept; one beyond it is not)
        XCTAssertTrue(view.contains(low: [0.5, -0.1, 0.2], high: [3, 0.1, 0.3]))
        XCTAssertFalse(view.contains(low: [1.1, -0.1, 0.2], high: [3, 0.1, 0.3]))
        XCTAssertFalse(view.contains(low: [-0.1, -0.1, -2], high: [0.1, 0.1, -0.01]))
        // (a box everywhere is never culled)
        XCTAssertTrue(
            view.contains(low: SIMD3(repeating: -.infinity), high: SIMD3(repeating: .infinity)))
        XCTAssertTrue(view.contains(center: [1.4, 0, 0.5], radius: 0.5))
        XCTAssertFalse(view.contains(center: [1.6, 0, 0.5], radius: 0.5))
    }

    func testBloodGulchDrawsWhatBruteForceSees() throws {
        let level = try StockLevel(map: "bloodgulch")
        let renderer = try level.renderer(VideoSettings(antiAliasing: .native, renderScale: 1))
        let geometry = level.geometry
        let stride = Int(geometry.vertexStride())
        let vertices = geometry.vertices(), indices = geometry.indices()
        func position(_ at: Int) -> SIMD3<Float> {
            let vertex = indices.withUnsafeBytes { Int($0.loadUnaligned(fromByteOffset: at * 4, as: UInt32.self)) }
            return vertices.withUnsafeBytes { raw in
                SIMD3(
                    raw.loadUnaligned(fromByteOffset: vertex * stride, as: Float.self),
                    raw.loadUnaligned(fromByteOffset: vertex * stride + 4, as: Float.self),
                    raw.loadUnaligned(fromByteOffset: vertex * stride + 8, as: Float.self))
            }
        }
        let draws = renderer.opaqueDraws
        XCTAssertGreaterThan(draws.count, 79, "the level is not split into cells")
        let start = geometry.startView()
        let at = SIMD3(start.x, start.y, start.z)
        let fov: Float = 70 * .pi / 180
        // (the start view turned four ways, and looking down)
        let views =
            [Float(0), .pi / 2, .pi, 3 * .pi / 2].map {
                ViewCamera(position: at, yaw: start.yaw + $0, pitch: 0, verticalFov: fov)
            } + [ViewCamera(position: at, yaw: start.yaw, pitch: -1.2, verticalFov: fov)]
        var culled = 0
        for camera in views {
            let matrix = camera.viewProjection(aspect: 16 / 9)
            let frustum = Frustum(matrix)
            let kept = Set(renderer.visibleOpaqueDraws(matrix))
            for (number, draw) in draws.enumerated() {
                let points = (draw.firstIndex..<(draw.firstIndex + draw.indexCount)).map(position)
                if points.contains(where: frustum.contains) {
                    XCTAssertTrue(kept.contains(number), "draw \(number) is in view but culled")
                }
                if !kept.contains(number) {
                    culled += 1
                    // (all its vertices are outside one plane)
                    let outside = frustum.planes.contains { plane in
                        points.allSatisfy { simd_dot(SIMD3(plane.x, plane.y, plane.z), $0) + plane.w < 0 }
                    }
                    XCTAssertTrue(outside, "draw \(number) is culled but not wholly outside a plane")
                }
            }
        }
        let share = Double(culled) / Double(draws.count * views.count)
        print("culling: \(draws.count) opaque draws, \(Int(share * 100))% culled over \(views.count) views")
        XCTAssertGreaterThan(share, 0.3, "too little is culled")
    }
}
