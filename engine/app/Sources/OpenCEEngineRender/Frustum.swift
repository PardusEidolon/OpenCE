import simd

/// The six planes of a view-projection's clip volume (Metal's: x and y in
/// -w to w, z in 0 to w), each facing in, for culling boxes the GPU would
/// clip away whole.
struct Frustum {
    /// Each plane's normal and distance: a point p is on its inside where
    /// dot(normal, p) + distance >= 0.
    let planes: [SIMD4<Float>]

    init(_ viewProjection: simd_float4x4) {
        let m = viewProjection
        func row(_ index: Int) -> SIMD4<Float> {
            SIMD4(m.columns.0[index], m.columns.1[index], m.columns.2[index], m.columns.3[index])
        }
        let (x, y, z, w) = (row(0), row(1), row(2), row(3))
        planes = [w + x, w - x, w + y, w - y, z, w - z]
    }

    /// Whether the box from `low` to `high` may be in view: false only
    /// when it is wholly outside one of the planes.
    func contains(low: SIMD3<Float>, high: SIMD3<Float>) -> Bool {
        for plane in planes {
            let normal = SIMD3(plane.x, plane.y, plane.z)
            // (the corner furthest along the plane's normal)
            let corner = low.replacing(with: high, where: normal .>= 0)
            if simd_dot(normal, corner) + plane.w < 0 {
                return false
            }
        }
        return true
    }

    /// Whether the sphere at `center` of `radius` may be in view: false
    /// only when it is wholly outside one of the planes.
    func contains(center: SIMD3<Float>, radius: Float) -> Bool {
        for plane in planes {
            let normal = SIMD3(plane.x, plane.y, plane.z)
            if simd_dot(normal, center) + plane.w < -radius * simd_length(normal) {
                return false
            }
        }
        return true
    }

    /// Whether the point is inside every plane.
    func contains(_ point: SIMD3<Float>) -> Bool {
        planes.allSatisfy { simd_dot(SIMD3($0.x, $0.y, $0.z), point) + $0.w >= 0 }
    }
}
