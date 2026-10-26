// The saved graphics settings (settings.toml's [graphics]) as the renderer
// draws with them: the tiers' presets, the shell's render.* variables they
// set, and the video settings they give.
import Foundation
import OpenCEEngineCore

extension QualityTier {
    /// Sets every graphics setting, and the render scale, to this tier's:
    /// the one table the tiers are.
    public func preset(into settings: inout Settings) {
        settings.graphics = GraphicsSettings(
            tier: rawValue,
            ambientOcclusion: self == .low ? "off" : self == .ultra ? "full" : "half",
            reflections: self == .high || self == .ultra,
            contactShadows: self != .low,
            shadowResolution: UInt32(self == .low ? 1024 : ShadowPass.defaultResolution),
            dynamicResolution: true,
            upscaleHeight: self == .low ? 1080 : 1440)
        settings.video.antiAliasing = AntiAliasing.temporal.rawValue
        settings.video.renderScale = self == .ultra ? 1 : 0.67
    }

    /// The stock settings at this tier.
    public var settings: Settings {
        var settings = stockSettings()
        preset(into: &settings)
        return settings
    }

    /// The switches this tier sets (the rest left as `switches` has them).
    public func apply(to switches: inout RenderSwitches) {
        settings.graphics.apply(to: &switches)
    }

    /// The video settings this tier draws with: temporal throughout, the
    /// scaler gathering into no more than 1080p at Low and 1440p above
    /// (MetalFX's spatial scaler takes it the rest of the way), drawn at two
    /// thirds of that below Ultra, and dynamic resolution holding
    /// `frameTargetMilliseconds`.
    public func video(frameTargetMilliseconds: Double = 1000.0 / 120) -> VideoSettings {
        VideoSettings(settings, frameTargetMilliseconds: frameTargetMilliseconds)
    }
}

extension GraphicsSettings {
    /// The shell's variables these set, by name, as the shell takes them.
    public var variables: [(name: String, value: String)] {
        [
            ("render.gtao", ambientOcclusion == "off" ? "false" : "true"),
            ("render.ao_quality", ambientOcclusion == "full" ? "full" : "half"),
            ("render.ssr", reflections ? "true" : "false"),
            ("render.contact_shadows", contactShadows ? "true" : "false"),
            ("render.shadow_resolution", "\(shadowResolution)"),
        ]
    }

    /// The switches these set (the rest left as `switches` has them).
    public func apply(to switches: inout RenderSwitches) {
        switches.gtao = ambientOcclusion != "off"
        switches.halfOcclusion = ambientOcclusion != "full"
        switches.ssr = reflections
        switches.contactShadows = contactShadows
        switches.shadowResolution =
            ShadowPass.resolutions.contains(Int(shadowResolution))
            ? Int(shadowResolution) : ShadowPass.defaultResolution
    }

    /// The tier these are all of, or nil when they are Custom.
    public var matchingTier: QualityTier? {
        QualityTier.allCases.first { $0.settings.graphics.sameSettings(as: self) }
    }

    /// Whether every setting but the tier's name is the same.
    func sameSettings(as other: GraphicsSettings) -> Bool {
        var named = other
        named.tier = tier
        return named == self
    }
}

extension VideoSettings {
    /// The video settings `settings` give: the anti-aliasing and render
    /// scale of [video], dynamic resolution and the upscale height of
    /// [graphics], dynamic resolution holding `frameTargetMilliseconds`.
    public init(_ settings: Settings, frameTargetMilliseconds: Double = 1000.0 / 120) {
        self.init(
            antiAliasing: AntiAliasing(rawValue: settings.video.antiAliasing) ?? .temporal,
            renderScale: settings.video.renderScale,
            dynamicResolution: settings.graphics.dynamicResolution,
            frameTargetMilliseconds: frameTargetMilliseconds,
            temporalHeightLimit: Int(settings.graphics.upscaleHeight))
    }
}

/// The GPU memory the largest stock map, Blood Gulch, takes drawn with
/// some settings at some size: a fit to its measured allocation at each
/// tier and with each setting changed alone, at 1080p, 1440p and 4K
/// (within 6% of each). Ambient occlusion and the reflections keep their
/// textures whether drawn or not, so only the sizes count: the shadow
/// maps', the drawn picture's, the temporal scaler's and the window's,
/// whose spatial scaler takes more when the temporal one is limited.
public enum GraphicsMemory {
    /// The estimate, in bytes, for `settings` drawn at `width` by `height`.
    public static func estimate(_ settings: Settings, width: Int, height: Int) -> Int {
        let video = VideoSettings(settings)
        let shadow = Double(settings.graphics.shadowResolution)
        let target = Double(width * height)
        var bytes = 43_000_000 + 32.0 * shadow * shadow
        if video.antiAliasing == .temporal {
            let gathered = video.temporalOutputSize(width: width, height: height)
            let drawn = video.inputSize(width: width, height: height)
            bytes += 251.5 * Double(drawn.width * drawn.height) + 55.6 * Double(gathered.width * gathered.height)
            bytes += (gathered.height < height ? 31.2 : -27.1) * target
        } else {
            bytes += 87.9 * target
        }
        return Int(bytes)
    }
}
