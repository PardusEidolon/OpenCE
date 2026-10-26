// The launcher: its four choices, and its settings (the data folders, the
// display and the window mode, and the graphics), kept in range and saved.
import AppKit
import Foundation
import OpenCEEngineCore
import OpenCEEngineKit
@testable import OpenCEEngineMenu
import OpenCEEngineRender
import SwiftUI
import XCTest

@MainActor
final class LauncherTests: XCTestCase {
    var folder: URL!

    override func setUp() async throws {
        folder = FileManager.default.temporaryDirectory.appendingPathComponent("launcher-\(UUID().uuidString)")
    }

    override func tearDown() async throws {
        try? FileManager.default.removeItem(at: folder)
    }

    func launcher(displays: [String] = ["Built-in", "Studio Display"]) -> LauncherModel {
        let settings = SettingsModel(path: folder.appendingPathComponent("settings.toml").path)
        return LauncherModel(settings: settings, displays: displays, root: folder)
    }

    func testThereArePlaySettingsDeveloperAndExitOnly() {
        XCTAssertEqual(LauncherChoice.allCases.map(\.title), ["Play", "Settings", "Developer", "Exit"])
    }

    func testPlayAndExitSaveAndSettingsShowsThem() {
        let model = launcher()
        var done: [String] = []
        model.onPlay = { done.append("play") }
        model.onExit = { done.append("exit") }
        model.choose(.settings)
        XCTAssertTrue(model.showsSettings)
        XCTAssertEqual(done, [])
        model.windowMode = .fullScreen
        model.choose(.play)
        XCTAssertEqual(done, ["play"])
        XCTAssertTrue(loadSettings(path: model.settings.path).video.fullScreen, "saved before the game starts")
        model.choose(.exit)
        XCTAssertEqual(done, ["play", "exit"])
    }

    func testTheFoldersAreAbsoluteOrTheStockOnes() throws {
        let model = launcher()
        XCTAssertEqual(model.folders.map(\.lastPathComponent), ["maps", "custom_maps"])
        XCTAssertEqual(model.folders.first?.deletingLastPathComponent().lastPathComponent, "assets")
        model.mapsFolder = "relative/maps"
        XCTAssertEqual(model.mapsFolder, "", "not absolute: the stock one")
        model.mapsFolder = "/Volumes/Halo/maps/../maps"
        model.customMapsFolder = "/Volumes/Halo/custom"
        XCTAssertEqual(model.folders.map(\.path), ["/Volumes/Halo/maps", "/Volumes/Halo/custom"])
        XCTAssertEqual(model.problem, "There is no ui.map in /Volumes/Halo/maps.")
        let maps = folder.appendingPathComponent("maps")
        try FileManager.default.createDirectory(at: maps, withIntermediateDirectories: true)
        try Data().write(to: maps.appendingPathComponent("ui.map"))
        model.mapsFolder = maps.path
        XCTAssertNil(model.problem)
    }

    func testTheDisplayIsOneThereAndResetIsTheStockOnes() {
        let model = launcher()
        XCTAssertEqual(model.display, 0)
        model.display = 1
        XCTAssertEqual(model.settings.settings.video.display, 1)
        model.display = 5
        XCTAssertEqual(model.display, 0, "a display not there is the main one")
        model.settings.settings.video.display = 1
        XCTAssertEqual(launcherWith(model.settings, displays: ["Built-in"]).display, 0, "unplugged since")
        model.windowMode = .fullScreen
        model.mapsFolder = "/maps"
        model.settings.settings.game.playerName = "Chief"
        model.reset()
        XCTAssertEqual(model.windowMode, .windowed)
        XCTAssertEqual(model.mapsFolder, "")
        XCTAssertEqual(model.display, 0)
        XCTAssertEqual(model.settings.settings.game.playerName, "Chief", "the game's own settings are the menus'")
        XCTAssertEqual(LauncherModel(settings: model.settings, displays: [], root: folder).displays, ["Main Display"])
    }

    func testTheViewDraws() {
        let model = launcher()
        let view = NSHostingView(rootView: LauncherView(model: model))
        view.frame = NSRect(x: 0, y: 0, width: 460, height: 380)
        view.layoutSubtreeIfNeeded()
        XCTAssertGreaterThan(view.fittingSize.height, 0)
        model.choose(.settings)
        view.layoutSubtreeIfNeeded()
        XCTAssertGreaterThan(view.fittingSize.height, 0)
    }

    func testATierPresetsTheGraphicsAndAChangeMakesThemCustom() {
        let model = launcher()
        model.tier = .high
        XCTAssertEqual(model.settings.settings.graphics, QualityTier.high.settings.graphics)
        XCTAssertEqual(model.settings.settings.graphics.matchingTier, .high)
        model.settings.shadowResolution = 2048
        XCTAssertEqual(model.tier, .high, "no change: still the tier")
        model.settings.shadowResolution = 4096
        XCTAssertNil(model.tier, "a change by hand is Custom")
        XCTAssertEqual(model.settings.settings.graphics.tier, "custom")
        XCTAssertEqual(model.settings.shadowResolution, 4096, "Custom keeps the settings")
        model.section = .graphics
        model.reset()
        XCTAssertEqual(model.settings.shadowResolution, 4096, "Custom has no tier to reset to")
        model.tier = .low
        model.settings.reflections = true
        model.tier = .low
        model.settings.contactShadows = true
        model.settings.settings.graphics.tier = "low"
        model.reset()
        XCTAssertEqual(model.settings.settings.graphics, QualityTier.low.settings.graphics, "Reset to tier")
        model.settings.save()
        XCTAssertEqual(loadSettings(path: model.settings.path).graphics.tier, "low")
    }

    func testEachGraphicsSettingReachesTheRenderer() {
        let model = launcher()
        model.tier = .medium
        func drawn() -> ([String], VideoSettings) {
            (model.settings.settings.graphics.variables.map(\.value), VideoSettings(model.settings.settings))
        }
        let changes: [(String, () -> Void)] = [
            ("ambient occlusion", { model.settings.ambientOcclusion = "full" }),
            ("reflections", { model.settings.reflections = true }),
            ("contact shadows", { model.settings.contactShadows = false }),
            ("shadow resolution", { model.settings.shadowResolution = 4096 }),
            ("upscale height", { model.settings.upscaleHeight = 0 }),
            ("dynamic resolution", { model.settings.dynamicResolution = false }),
        ]
        for (name, change) in changes {
            model.tier = .medium
            let (variables, video) = drawn()
            change()
            let (changedVariables, changedVideo) = drawn()
            XCTAssertTrue(
                variables != changedVariables || video.temporalHeightLimit != changedVideo.temporalHeightLimit
                    || video.dynamicResolution != changedVideo.dynamicResolution, name)
            XCTAssertNil(model.tier, name)
        }
        var switches = RenderSwitches()
        model.tier = .ultra
        model.settings.settings.graphics.apply(to: &switches)
        XCTAssertFalse(switches.halfOcclusion)
        XCTAssertTrue(switches.ssr)
    }

    func testTheMeterIsTheEstimateAtTheDisplaysSize() {
        let mac = MacGraphics(
            detected: .ultra, name: "Apple M1 Max, 32 GPU cores, 32 GB", memoryBudget: 21 << 30, width: 3840,
            height: 2160, frameMilliseconds: 1000.0 / 120)
        let model = LauncherModel(
            settings: SettingsModel(path: folder.appendingPathComponent("settings.toml").path), displays: [],
            root: folder, graphics: mac)
        model.tier = .low
        let low = model.memoryEstimate
        model.tier = .ultra
        XCTAssertGreaterThan(model.memoryEstimate, low)
        XCTAssertEqual(
            model.memoryEstimate, GraphicsMemory.estimate(QualityTier.ultra.settings, width: 3840, height: 2160))
        XCTAssertEqual(
            model.benchmarkArguments,
            [
                "--benchmark", "10", "--path", "bloodgulch", "--quality", "saved", "--size", "3840x2160", "--maps",
                folder.appendingPathComponent("assets/maps").path,
            ])
    }

    func testTheBenchmarkSavesRunsAndSaysItsResult() async {
        let model = launcher()
        XCTAssertEqual(model.benchmark.text, "Not run yet")
        model.runBenchmark()
        XCTAssertEqual(model.benchmark, .none, "nothing to run it with")
        var ran: [[String]] = []
        model.benchmarker = { arguments in
            ran.append(arguments)
            return Data("loading\n{\"mean_ms\": 5.418, \"p99_ms\": 6.384, \"hitches\": 0}\n".utf8)
        }
        model.tier = .high
        model.runBenchmark()
        XCTAssertEqual(model.benchmark, .running)
        XCTAssertEqual(loadSettings(path: model.settings.path).graphics.tier, "high", "saved for the run")
        for _ in 0..<100 where model.benchmark == .running {
            await Task.yield()
        }
        XCTAssertEqual(ran.count, 1)
        XCTAssertEqual(model.benchmark, .done(meanMs: 5.418, p99Ms: 6.384, hitches: 0))
        XCTAssertEqual(model.benchmark.text, "Last run: mean 5.42 ms · p99 6.38 ms · 0 hitches")
        XCTAssertEqual(BenchmarkRun.parse(Data("error: no map\n".utf8)), .failed("error: no map"))
        XCTAssertEqual(BenchmarkRun.parse(Data()), .failed("no report"))
    }

    func testEachSectionDraws() {
        let model = launcher()
        let view = NSHostingView(rootView: LauncherView(model: model))
        view.frame = NSRect(x: 0, y: 0, width: 620, height: 640)
        model.choose(.settings)
        for section in LauncherSection.allCases {
            model.section = section
            view.layoutSubtreeIfNeeded()
            XCTAssertGreaterThan(view.fittingSize.height, 0, section.title)
        }
        XCTAssertEqual(LauncherSection.allCases.map(\.title), ["Folders", "Display", "Graphics"])
    }

    func launcherWith(_ settings: SettingsModel, displays: [String]) -> LauncherModel {
        LauncherModel(settings: settings, displays: displays, root: folder)
    }
}
