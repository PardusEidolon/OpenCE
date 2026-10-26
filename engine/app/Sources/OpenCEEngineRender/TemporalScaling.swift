// Temporal anti-aliasing and upscaling with MetalFX: the video settings,
// the jitter sequence, and the scaler with its inputs (the level drawn
// jittered at the render scale, its depth, and its motion, which for the
// still level is the camera's alone, rebuilt from depth, so the sprites,
// which move on their own, are marked in its reactive mask and taken from
// the frame they are drawn in).

import Metal
import MetalFX
import simd

/// How the frame is smoothed.
public enum AntiAliasing: String, CaseIterable, Sendable {
    /// Drawn at the output's size, not smoothed.
    case native
    /// Drawn jittered at the render scale and gathered over frames by
    /// MetalFX's temporal scaler into the output's size.
    case temporal
}

/// The renderer's video settings.
public struct VideoSettings: Equatable, Sendable {
    public var antiAliasing: AntiAliasing
    /// The share of the output's width and height drawn before MetalFX
    /// scales it up (temporal only; 1 smooths without upscaling).
    public var renderScale: Float
    /// Whether the render scale steps down from `renderScale` when the
    /// GPU's frames run long (temporal only).
    public var dynamicResolution: Bool
    /// The frame time dynamic resolution holds to, in milliseconds.
    public var frameTargetMilliseconds: Double
    /// The tallest output the temporal scaler gathers into (0: the
    /// output's own): a taller output is gathered at this height and
    /// scaled the rest of the way by MetalFX's spatial scaler, since the
    /// temporal scaler's cost goes with its output (about 5.5 ms a frame
    /// into 4K on an M1 Max, 2 into 1440p).
    public var temporalHeightLimit: Int

    public init(
        antiAliasing: AntiAliasing = .temporal, renderScale: Float = 1, dynamicResolution: Bool = false,
        frameTargetMilliseconds: Double = 1000.0 / 120, temporalHeightLimit: Int = 0
    ) {
        self.antiAliasing = antiAliasing
        self.renderScale = renderScale
        self.dynamicResolution = dynamicResolution
        self.frameTargetMilliseconds = frameTargetMilliseconds
        self.temporalHeightLimit = temporalHeightLimit
    }

    /// The size the temporal scaler gathers into for an output `width` by
    /// `height`: the output's, or no taller than `temporalHeightLimit`.
    public func temporalOutputSize(width: Int, height: Int) -> (width: Int, height: Int) {
        guard antiAliasing == .temporal, temporalHeightLimit > 0, height > temporalHeightLimit else {
            return (width, height)
        }
        let scaled = (Double(width) * Double(temporalHeightLimit) / Double(height)).rounded()
        return (max(Int(scaled), 1), temporalHeightLimit)
    }

    /// The smallest and largest render scales.
    public static let renderScales: ClosedRange<Float> = 0.5...1

    /// The size drawn for an output `width` by `height`.
    public func inputSize(width: Int, height: Int) -> (width: Int, height: Int) {
        inputSize(width: width, height: height, scale: renderScale)
    }

    /// The size drawn for an output `width` by `height` at `renderScale`
    /// (a share of the temporal scaler's output).
    public func inputSize(width: Int, height: Int, scale renderScale: Float) -> (width: Int, height: Int) {
        guard antiAliasing == .temporal else { return (width, height) }
        let (width, height) = temporalOutputSize(width: width, height: height)
        let scale = renderScale.isFinite ? min(max(renderScale, Self.renderScales.lowerBound), 1) : 1
        return (max(Int((Float(width) * scale).rounded()), 1), max(Int((Float(height) * scale).rounded()), 1))
    }
}

/// The sub-pixel offsets the frames are drawn at: Halton's bases 2 and 3,
/// each in -0.5 to 0.5 of a pixel, repeating every `length` frames.
public enum Jitter {
    public static let length = 32

    static func halton(_ index: Int, base: Int) -> Float {
        var fraction: Float = 1
        var result: Float = 0
        var index = index
        while index > 0 {
            fraction /= Float(base)
            result += fraction * Float(index % base)
            index /= base
        }
        return result
    }

    /// The offset of frame `frame`, in pixels, x right and y down.
    public static func offset(frame: Int) -> SIMD2<Float> {
        let index = frame % length + 1
        return SIMD2(halton(index, base: 2), halton(index, base: 3)) - 0.5
    }

    /// `viewProjection` moved by `offset` pixels of a `width` by `height`
    /// target.
    public static func apply(
        _ offset: SIMD2<Float>, to viewProjection: simd_float4x4, width: Int,
        height: Int
    ) -> simd_float4x4 {
        var shift = matrix_identity_float4x4
        // (clip space's y is up; a pixel's is down)
        shift.columns.3.x = 2 * offset.x / Float(max(width, 1))
        shift.columns.3.y = -2 * offset.y / Float(max(height, 1))
        return shift * viewProjection
    }
}

/// The motion pass's uniforms, as the shader's Motion.
struct MotionUniforms {
    var reprojection: simd_float4x4
    var size: SIMD2<Float>
}

/// MetalFX's temporal scaler for one input and output size, and what it
/// remembers between frames; and, when the output is limited below the
/// target's size, the spatial scaler that takes it the rest of the way.
final class TemporalScaling {
    let scaler: MTLFXTemporalScaler
    let spatial: MTLFXSpatialScaler?
    /// The spatial scaler's output, at the target's size.
    let upscaled: MTLTexture?
    let color: MTLTexture
    let depth: MTLTexture
    let motion: MTLTexture
    /// How much of each pixel is a sprite (the particle pass writes it).
    let reactive: MTLTexture
    let output: MTLTexture
    var frame = 0
    /// The last frame's unjittered view-projection (nil: no history).
    var previous: simd_float4x4?

    static let motionFormat = MTLPixelFormat.rg16Float
    static let reactiveFormat = MTLPixelFormat.r8Unorm

    static func isSupported(_ device: MTLDevice) -> Bool {
        MTLFXTemporalScalerDescriptor.supportsDevice(device)
    }

    init?(
        device: MTLDevice, input: (width: Int, height: Int), output: (width: Int, height: Int),
        target: (width: Int, height: Int)? = nil
    ) {
        let descriptor = MTLFXTemporalScalerDescriptor()
        descriptor.inputWidth = input.width
        descriptor.inputHeight = input.height
        descriptor.outputWidth = output.width
        descriptor.outputHeight = output.height
        descriptor.colorTextureFormat = LevelRenderer.frameFormat
        descriptor.depthTextureFormat = LevelRenderer.depthFormat
        descriptor.motionTextureFormat = Self.motionFormat
        descriptor.outputTextureFormat = LevelRenderer.frameFormat
        descriptor.isAutoExposureEnabled = false
        descriptor.isReactiveMaskTextureEnabled = true
        descriptor.reactiveMaskTextureFormat = Self.reactiveFormat
        guard let scaler = descriptor.makeTemporalScaler(device: device) else { return nil }
        scaler.isDepthReversed = true
        // (the motion pass writes uv units; MetalFX wants pixels)
        scaler.motionVectorScaleX = Float(input.width)
        scaler.motionVectorScaleY = Float(input.height)
        scaler.inputContentWidth = input.width
        scaler.inputContentHeight = input.height

        func texture(
            _ format: MTLPixelFormat, _ size: (width: Int, height: Int),
            _ usage: MTLTextureUsage
        ) -> MTLTexture? {
            let made = MTLTextureDescriptor.texture2DDescriptor(
                pixelFormat: format, width: size.width,
                height: size.height, mipmapped: false)
            made.usage = usage
            made.storageMode = .private
            return device.makeTexture(descriptor: made)
        }
        guard
            let color = texture(
                LevelRenderer.frameFormat, input, scaler.colorTextureUsage.union([.renderTarget, .shaderRead])),
            let depth = texture(
                LevelRenderer.depthFormat, input, scaler.depthTextureUsage.union([.renderTarget, .shaderRead])),
            let motion = texture(Self.motionFormat, input, scaler.motionTextureUsage.union(.renderTarget)),
            let reactive = texture(Self.reactiveFormat, input, scaler.reactiveTextureUsage.union(.renderTarget)),
            let out = texture(LevelRenderer.frameFormat, output, scaler.outputTextureUsage.union(.shaderRead))
        else { return nil }
        if let target, target != output {
            let spatialDescriptor = MTLFXSpatialScalerDescriptor()
            spatialDescriptor.inputWidth = output.width
            spatialDescriptor.inputHeight = output.height
            spatialDescriptor.outputWidth = target.width
            spatialDescriptor.outputHeight = target.height
            spatialDescriptor.colorTextureFormat = LevelRenderer.frameFormat
            spatialDescriptor.outputTextureFormat = LevelRenderer.frameFormat
            // (the frame is linear and not yet exposed)
            spatialDescriptor.colorProcessingMode = .hdr
            guard let spatial = spatialDescriptor.makeSpatialScaler(device: device),
                let upscaled = texture(
                    LevelRenderer.frameFormat, target, spatial.outputTextureUsage.union(.shaderRead))
            else { return nil }
            spatial.inputContentWidth = output.width
            spatial.inputContentHeight = output.height
            self.spatial = spatial
            self.upscaled = upscaled
        } else {
            spatial = nil
            upscaled = nil
        }
        self.scaler = scaler
        self.color = color
        self.depth = depth
        self.motion = motion
        self.reactive = reactive
        self.output = out
    }

    var inputWidth: Int { color.width }
    var inputHeight: Int { color.height }
    var outputWidth: Int { output.width }
    var outputHeight: Int { output.height }
    /// The size this scaling ends at (the spatial scaler's, if any).
    var targetWidth: Int { upscaled?.width ?? output.width }
    var targetHeight: Int { upscaled?.height ?? output.height }
    /// The frame at the target's size.
    var shown: MTLTexture { upscaled ?? output }

    /// The motion pass's uniforms for this frame's unjittered
    /// view-projection (and remembers it for the next frame).
    func motionUniforms(current: simd_float4x4) -> MotionUniforms {
        // (in doubles: the infinite projection's inverse is large)
        func double(_ matrix: simd_float4x4) -> simd_double4x4 {
            simd_double4x4(
                columns: (
                    SIMD4<Double>(matrix.columns.0), SIMD4<Double>(matrix.columns.1),
                    SIMD4<Double>(matrix.columns.2), SIMD4<Double>(matrix.columns.3)
                ))
        }
        let previous = self.previous ?? current
        let reprojection = double(previous) * double(current).inverse
        self.previous = current
        let float = simd_float4x4(
            columns: (
                SIMD4<Float>(reprojection.columns.0),
                SIMD4<Float>(reprojection.columns.1),
                SIMD4<Float>(reprojection.columns.2),
                SIMD4<Float>(reprojection.columns.3)
            ))
        return MotionUniforms(reprojection: float, size: SIMD2(Float(inputWidth), Float(inputHeight)))
    }

    /// Gathers this frame (drawn at `jitter`) into `output`, taking the
    /// sprites from it where `reactive` was written this frame.
    func encode(into commands: MTLCommandBuffer, jitter: SIMD2<Float>, reacting: Bool) {
        scaler.colorTexture = color
        scaler.depthTexture = depth
        scaler.motionTexture = motion
        scaler.reactiveMaskTexture = reacting ? reactive : nil
        scaler.outputTexture = output
        scaler.jitterOffsetX = jitter.x
        scaler.jitterOffsetY = jitter.y
        scaler.reset = frame == 0
        scaler.encode(commandBuffer: commands)
        frame += 1
        if let spatial, let upscaled {
            spatial.colorTexture = output
            spatial.outputTexture = upscaled
            spatial.encode(commandBuffer: commands)
        }
    }
}
