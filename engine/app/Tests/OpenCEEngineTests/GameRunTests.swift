// game.wasm run headless for the app (needs assets/maps and a built
// game.wasm): Blood Gulch loaded and drawn for some seconds into a window
// off the screen, through WasmGame and GameRenderer.

import Foundation
import Metal
import OpenCEEngineCore
import OpenCEEngineRender
import XCTest

final class GameRunTests: XCTestCase {
    func testBloodGulchIsDrawnHeadless() throws {
        let root = StockLevel.root
        let wasm = root.appendingPathComponent("build/engine/wasm/game.wasm")
        let map = root.appendingPathComponent("assets/maps/bloodgulch.map")
        let files = FileManager.default
        guard files.fileExists(atPath: wasm.path) else { throw XCTSkip("no build/engine/wasm/game.wasm") }
        guard files.fileExists(atPath: map.path) else { throw XCTSkip("no assets/maps/bloodgulch.map") }
        let saves = files.temporaryDirectory.appendingPathComponent("opence-game-run-\(UUID().uuidString)")
        try files.createDirectory(at: saves, withIntermediateDirectories: true)
        defer { try? files.removeItem(at: saves) }

        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice())
        let window = try XCTUnwrap(OffscreenWindow(device: device, width: 640, height: 480))
        let renderer = try XCTUnwrap(GameRenderer(device: device, window: window))
        let game = try WasmGame(
            options: WasmGameOptions(
                wasm: wasm.path, data: root.appendingPathComponent("assets").path, saves: saves.path,
                cache: root.appendingPathComponent("build/engine/wasm/cache").path, width: 640, height: 480,
                map: "bloodgulch", seconds: 30))
        try game.start(target: renderer)
        let end = game.wait()
        renderer.finish()

        print(
            "game run: \(end.batches) batches, \(end.commands) commands, \(end.refused) refused by the game's "
                + "check, \(renderer.statistics.refused) by Metal, \(window.presented) frames presented")
        XCTAssertNil(end.error)
        XCTAssertTrue(end.timedOut)
        XCTAssertEqual(end.refused, 0, end.refusals.joined(separator: "\n"))
        XCTAssertGreaterThan(window.presented, 0)
        // (the level drawn: more than one colour on the screen)
        let pixels = window.pixels()
        XCTAssertGreaterThan(Set(pixels.map { Array($0.prefix(3)) }).count, 16)
        // (to look at it: the last frame as a PPM where OPENCE_GAME_RUN_FRAME names, kept on this machine)
        if let path = ProcessInfo.processInfo.environment["OPENCE_GAME_RUN_FRAME"] {
            let header = Data("P6\n\(window.texture.width) \(window.texture.height)\n255\n".utf8)
            try (header + Data(pixels.flatMap { $0.prefix(3) })).write(to: URL(fileURLWithPath: path))
        }
    }
}
