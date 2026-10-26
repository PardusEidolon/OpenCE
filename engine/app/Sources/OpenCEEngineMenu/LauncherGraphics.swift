// The launcher's Graphics section: the quality tier picked for this Mac,
// each setting with what it does and what it costs, the GPU memory the
// settings take, and a benchmark of them run by the app itself.
import Foundation
import OpenCEEngineCore
import OpenCEEngineRender
import SwiftUI

/// The launcher's sections of settings.
public enum LauncherSection: String, CaseIterable, Sendable {
    case folders, display, graphics

    public var title: String { rawValue.capitalized }
}

/// What the Graphics section knows of this Mac.
public struct MacGraphics: Sendable, Equatable {
    /// The tier picked for it.
    public var detected: QualityTier
    /// Its name as the section shows it ("Apple M1 Max, 32 GPU cores, 32 GB").
    public var name: String
    /// The GPU memory it recommends a game keep within, in bytes.
    public var memoryBudget: Int
    /// The display's size in pixels, and its frame time.
    public var width: Int
    public var height: Int
    public var frameMilliseconds: Double

    public init(
        detected: QualityTier, name: String, memoryBudget: Int, width: Int, height: Int,
        frameMilliseconds: Double
    ) {
        self.detected = detected
        self.name = name
        self.memoryBudget = memoryBudget
        self.width = width
        self.height = height
        self.frameMilliseconds = frameMilliseconds
    }

    /// A Mac not looked at: Medium's, 4 GB, a 1440p display at 60.
    public static let unknown = MacGraphics(
        detected: .medium, name: "this Mac", memoryBudget: 4 << 30, width: 2560, height: 1440,
        frameMilliseconds: 1000.0 / 60)
}

/// The benchmark's last run.
public enum BenchmarkRun: Equatable, Sendable {
    case none
    case running
    case done(meanMs: Double, p99Ms: Double, hitches: Int)
    case failed(String)

    /// The run as the section says it.
    public var text: String {
        switch self {
        case .none: "Not run yet"
        case .running: "Running…"
        case let .done(mean, p99, hitches):
            String(format: "Last run: mean %.2f ms · p99 %.2f ms · %d hitches", mean, p99, hitches)
        case let .failed(why): "Failed: \(why)"
        }
    }

    /// The run the benchmark's JSON report (its last line) gives.
    public static func parse(_ output: Data) -> BenchmarkRun {
        let lines = String(decoding: output, as: UTF8.self).split(separator: "\n")
        guard let line = lines.last(where: { $0.hasPrefix("{") }),
            let object = try? JSONSerialization.jsonObject(with: Data(line.utf8)) as? [String: Any],
            let mean = object["mean_ms"] as? Double, let p99 = object["p99_ms"] as? Double
        else {
            return .failed(lines.last.map(String.init) ?? "no report")
        }
        return .done(meanMs: mean, p99Ms: p99, hitches: object["hitches"] as? Int ?? 0)
    }
}

/// What each setting does and costs: GPU time added per frame on Blood
/// Gulch at 4K, measured on an M1 Max (render scale 0.67 unless said).
enum GraphicsCosts {
    static let ambientOcclusion = "Full: +1.5 ms"
    static let reflections = "+0.17 ms"
    static let contactShadows = "< 0.05 ms"
    static let shadowResolution = "4096: +1.56 ms"
    static let upscaleHeight = "Native: +2.4 ms"
}

extension LauncherModel {
    /// The tier picker's choice: a tier, or nil for Custom.
    public var tier: QualityTier? {
        get { settings.tier }
        set { settings.tier = newValue }
    }

    /// The GPU memory the settings take for the largest stock map at the
    /// display's size, in bytes.
    public var memoryEstimate: Int {
        GraphicsMemory.estimate(settings.settings, width: graphics.width, height: graphics.height)
    }

    /// The arguments the benchmark runs the app with: Blood Gulch's path
    /// for 10 seconds at the saved settings and the display's size.
    public var benchmarkArguments: [String] {
        var arguments = [
            "--benchmark", "10", "--path", "bloodgulch", "--quality", "saved",
            "--size", "\(graphics.width)x\(graphics.height)",
        ]
        if let maps = folders.first {
            arguments += ["--maps", maps.path]
        }
        return arguments
    }

    /// Saves the settings and runs the benchmark with them, its result in
    /// `benchmark` when it ends.
    public func runBenchmark() {
        guard benchmark != .running, let benchmarker else { return }
        guard settings.save() else {
            benchmark = .failed(settings.problem ?? "the settings could not be saved")
            return
        }
        benchmark = .running
        let arguments = benchmarkArguments
        Task {
            let output = await benchmarker(arguments)
            benchmark = BenchmarkRun.parse(output)
        }
    }
}

/// The Graphics section, as the design draws it.
struct GraphicsSection: View {
    @Bindable var model: LauncherModel

    var body: some View {
        @Bindable var settings = model.settings
        Form {
            Section {
                Picker("Quality", selection: $model.tier) {
                    ForEach(QualityTier.allCases, id: \.self) { tier in
                        Text(tier.title).tag(QualityTier?.some(tier))
                    }
                    Text("Custom").tag(QualityTier?.none)
                }
                .pickerStyle(.segmented)
                Text("Picked for this Mac: \(model.graphics.detected.title) (\(model.graphics.name))")
                    .foregroundStyle(.secondary)
                Text("A tier sets every setting below. Changing one by hand makes the set Custom.")
                    .foregroundStyle(.secondary)
            }
            Section {
                GraphicsRow(
                    title: "Ambient occlusion",
                    detail: "Darkens creases, corners and the ground under objects. Half draws it at half resolution.",
                    cost: GraphicsCosts.ambientOcclusion
                ) {
                    Picker("", selection: $settings.ambientOcclusion) {
                        ForEach(SettingsModel.ambientOcclusions, id: \.self) { Text($0.capitalized).tag($0) }
                    }
                }
                GraphicsRow(
                    title: "Reflections", detail: "Screen-space reflections on water and polished floors.",
                    cost: GraphicsCosts.reflections
                ) {
                    Toggle("", isOn: $settings.reflections)
                }
                GraphicsRow(
                    title: "Contact shadows",
                    detail: "Short shadows where objects meet the ground, finer than the sun's shadow map.",
                    cost: GraphicsCosts.contactShadows
                ) {
                    Toggle("", isOn: $settings.contactShadows)
                }
                GraphicsRow(
                    title: "Shadow resolution",
                    detail: "The size of the sun's shadow maps. Larger is sharper at the edges.",
                    cost: GraphicsCosts.shadowResolution
                ) {
                    Picker("", selection: $settings.shadowResolution) {
                        ForEach(SettingsModel.shadowResolutions, id: \.self) { Text(verbatim: "\($0)").tag($0) }
                    }
                }
                GraphicsRow(
                    title: "Upscale height",
                    detail:
                        "The temporal upscaler gathers the frame at this height; a spatial pass takes it to the display. Native gathers at full size.",
                    cost: GraphicsCosts.upscaleHeight
                ) {
                    Picker("", selection: $settings.upscaleHeight) {
                        ForEach(SettingsModel.upscaleHeights, id: \.self) {
                            Text(verbatim: $0 == 0 ? "Native" : "\($0)").tag($0)
                        }
                    }
                }
                GraphicsRow(
                    title: "Dynamic resolution",
                    detail: "Steps the render scale between 1, 0.67 and 0.5 to hold the display's frame rate.",
                    cost: String(format: "holds %.1f ms", model.graphics.frameMilliseconds)
                ) {
                    Toggle("", isOn: $settings.dynamicResolution)
                }
            } footer: {
                Text("Costs are GPU time added per frame on Blood Gulch at 4K, measured on an M1 Max.")
                    .foregroundStyle(.secondary)
            }
            Section("GPU memory") {
                let used = model.memoryEstimate
                let budget = max(model.graphics.memoryBudget, 1)
                ProgressView(value: min(Double(used) / Double(budget), 1))
                Text("\(used / 1_000_000) MB of \(budget / 1_000_000) MB").monospacedDigit()
                Text("The largest stock map, Blood Gulch, at these settings and the display's size.")
                    .foregroundStyle(.secondary)
            }
            Section("Benchmark") {
                Text("Flies Blood Gulch's path for 10 seconds at these settings.").foregroundStyle(.secondary)
                Text(model.benchmark.text).monospacedDigit()
                Button("Run benchmark") { model.runBenchmark() }
                    .disabled(model.benchmark == .running || model.benchmarker == nil)
            }
        }
        .formStyle(.grouped)
    }
}

/// A setting: its name and what it does, its control, and its cost.
struct GraphicsRow<Control: View>: View {
    let title: String
    let detail: String
    let cost: String
    @ViewBuilder let control: () -> Control

    var body: some View {
        HStack(alignment: .center, spacing: 16) {
            VStack(alignment: .leading, spacing: 4) {
                Text(title).font(MenuFonts.heading(14))
                Text(detail).foregroundStyle(.secondary).fixedSize(horizontal: false, vertical: true)
            }
            Spacer()
            control().labelsHidden().frame(width: 120)
            Text(cost).monospacedDigit().foregroundStyle(.secondary).frame(width: 110, alignment: .trailing)
        }
    }
}
