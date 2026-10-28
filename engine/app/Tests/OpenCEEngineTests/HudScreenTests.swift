// The HUD at 4K, measured against the HUD at the Xbox's 640×480: Blood
// Gulch's frame 1500 drawn through game.wasm at both sizes, each once with
// the HUD and once without (show_hud), the game's time stepped a 60th of a
// second a frame (debug.frame_clock) so the four runs draw the same frame.
// The HUD's pixels are where a frame with it differs from the frame without;
// the 4K frame's are taken back to the screen's units (852 by 480, each unit
// 4.5 pixels), and each part of the HUD is matched where the game puts it on
// a wider screen. The title-safe frame the HUD keeps to is inset 48 units at
// 640 and widened in proportion (rasterizer_initialize, 48 × 852 / 640 = 63
// at 852), so the elements at the left move 15 units, those at the right 197,
// and those at the middle 106, the screen's own middle. The first-person
// weapon is hidden (display.viewmodel_visible), so its pose does not count as
// HUD; and the grenade count is read from the game at the frame.
// The first-person rifle and hand are measured the same way, a frame with
// them against one without: the view keeps its vertical angle on a wider
// screen (view_fov.c), so they are where the 640 frame has them, 106 units
// further right, the screen's middle.
// Needs assets/maps and a built game.wasm; the frames stay on this machine.

import Foundation
import Metal
import OpenCEEngineCore
import OpenCEEngineRender
import XCTest

/// a value one thread sets and another reads
private final class Locked<Value>: @unchecked Sendable {
    private let lock = NSLock()
    private var held: Value
    init(_ value: Value) { held = value }
    var value: Value {
        get { lock.withLock { held } }
        set { lock.withLock { held = newValue } }
    }
}

final class HudScreenTests: XCTestCase {
    /// the frames drawn, by their size and what was shown, for the tests that share them
    private static let drawn = Locked<[String: (CapturedFrame, String?)]>([:])

    /// the title-safe frame's inset in units on a screen `units` wide (rasterizer_xbox.c)
    private func inset(_ units: Int) -> Int { 48 * units / 640 }

    /// frame 1500 of Blood Gulch at `width` by `height`, with or without the HUD and the
    /// first-person weapon, and the player's grenades as the game counts them then
    private func frame(width: Int, height: Int, hud: Bool, weapon: Bool = false) throws -> (CapturedFrame, String?) {
        let name =
            weapon
            ? "wasm_bloodgulch_rifle_\(width)x\(height)"
            : "wasm_bloodgulch_hud_\(hud ? "on" : "off")_\(width)x\(height)"
        if let frame = Self.drawn.value[name] { return frame }
        let root = StockLevel.root
        let wasm = root.appendingPathComponent("build/engine/wasm/game.wasm")
        let files = FileManager.default
        let saves = files.temporaryDirectory.appendingPathComponent("opence-hud-\(UUID().uuidString)")
        try files.createDirectory(at: saves, withIntermediateDirectories: true)
        defer { try? files.removeItem(at: saves) }
        let device = try XCTUnwrap(MTLCreateSystemDefaultDevice())
        let window = try XCTUnwrap(OffscreenWindow(device: device, width: width, height: height, capture: 1500))
        let renderer = try XCTUnwrap(GameRenderer(device: device, window: window))
        let game = try WasmGame(
            options: WasmGameOptions(
                wasm: wasm.path, data: root.appendingPathComponent("assets").path, saves: saves.path,
                cache: root.appendingPathComponent("build/engine/wasm/cache").path, width: UInt32(width),
                height: UInt32(height), map: "bloodgulch", seconds: 300,
                settings: [
                    "debug.frame_clock=true", "debug.telnet_console=true", "display.viewmodel_visible=\(weapon)",
                ]))
        try game.start(target: renderer)
        // (the HUD hidden early, and only then, so the console's echo has faded by frame
        // 1500; the grenades asked once the frame is taken)
        let grenades = Locked<String?>(nil)
        let console = Thread {
            while !window.hasCaptured {
                if !hud && window.presented < 600 { _ = game.ask(line: "show_hud false", milliseconds: 200) }
                Thread.sleep(forTimeInterval: 0.5)
            }
            grenades.value = game.ask(
                line: "inspect (unit_get_total_grenade_count (unit (list_get (players) 0)))", milliseconds: 2000)
        }
        console.start()
        let end = game.wait()
        renderer.finish()
        XCTAssertNil(end.error)
        XCTAssertEqual(end.refused, 0, end.refusals.joined(separator: "\n"))
        let drawn = try XCTUnwrap(window.capturedFrame(), "only \(window.presented) frames were presented")
        let golden = root.appendingPathComponent("build/engine/golden")
        try files.createDirectory(at: golden, withIntermediateDirectories: true)
        try drawn.writePNG(to: golden.appendingPathComponent("\(name).png"))
        Self.drawn.value[name] = (drawn, grenades.value)
        return (drawn, grenades.value)
    }

    /// The HUD's pixels in units: where `with` differs from `without` by more than a quarter, each unit
    /// the box of pixels it covers (`columns` by 480 units over the frame).
    private func mask(with: CapturedFrame, without: CapturedFrame, columns: Int) -> [[Bool]] {
        let (sx, sy) = (Double(with.width) / Double(columns), Double(with.height) / 480)
        return (0..<480).map { row in
            (0..<columns).map { column in
                let xs = Int(Double(column) * sx)..<max(Int(Double(column + 1) * sx), Int(Double(column) * sx) + 1)
                let ys = Int(Double(row) * sy)..<max(Int(Double(row + 1) * sy), Int(Double(row) * sy) + 1)
                var (sum, count) = (0, 0)
                for y in ys where y < with.height {
                    for x in xs where x < with.width {
                        let (a, b) = (with.rgb(x: x, y: y), without.rgb(x: x, y: y))
                        sum += (0..<3).map { abs(Int(a[$0]) - Int(b[$0])) }.max() ?? 0
                        count += 1
                    }
                }
                return count > 0 && sum / count > 64
            }
        }
    }

    func testTheHudAt4KIsWhereTheOriginalPutsItOnAWideScreen() throws {
        let root = StockLevel.root
        let files = FileManager.default
        guard files.fileExists(atPath: root.appendingPathComponent("build/engine/wasm/game.wasm").path) else {
            throw XCTSkip("no build/engine/wasm/game.wasm")
        }
        guard files.fileExists(atPath: root.appendingPathComponent("assets/maps/bloodgulch.map").path) else {
            throw XCTSkip("no assets/maps/bloodgulch.map")
        }
        let (smallOn, smallGrenades) = try frame(width: 640, height: 480, hud: true)
        let small = mask(with: smallOn, without: try frame(width: 640, height: 480, hud: false).0, columns: 640)
        let (wideOn, wideGrenades) = try frame(width: 3840, height: 2160, hud: true)
        let wide = mask(with: wideOn, without: try frame(width: 3840, height: 2160, hud: false).0, columns: 852)
        print("hud grenades: \(smallGrenades ?? "no answer") at 640, \(wideGrenades ?? "no answer") at 4K")
        XCTAssertNotNil(smallGrenades)
        XCTAssertEqual(smallGrenades, wideGrenades)
        // each third of the 640 screen, and how far it moves on the 852 one
        let edge = inset(852) - inset(640)
        compare(
            small, wide, [("left", 0..<213, edge), ("middle", 213..<427, 106), ("right", 427..<640, 212 - edge)], "hud")
    }

    func testTheRifleAndHandAt4KAreWhereTheOriginalPutsThem() throws {
        let root = StockLevel.root
        let files = FileManager.default
        guard files.fileExists(atPath: root.appendingPathComponent("build/engine/wasm/game.wasm").path) else {
            throw XCTSkip("no build/engine/wasm/game.wasm")
        }
        guard files.fileExists(atPath: root.appendingPathComponent("assets/maps/bloodgulch.map").path) else {
            throw XCTSkip("no assets/maps/bloodgulch.map")
        }
        let small = mask(
            with: try frame(width: 640, height: 480, hud: true, weapon: true).0,
            without: try frame(width: 640, height: 480, hud: true).0, columns: 640)
        let wide = mask(
            with: try frame(width: 3840, height: 2160, hud: true, weapon: true).0,
            without: try frame(width: 3840, height: 2160, hud: true).0, columns: 852)
        compare(small, wide, [("rifle", 0..<640, 106)], "first person")
    }

    /// each part of the 640 screen's mask against the 852 one's, `shift` units further right
    private func compare(_ small: [[Bool]], _ wide: [[Bool]], _ parts: [(String, Range<Int>, Int)], _ what: String) {
        for (name, columns, shift) in parts {
            var (both, either, original, drawn) = (0, 0, 0, 0)
            var box = (Int.max, Int.max, Int.min, Int.min)
            for row in 0..<480 {
                for column in columns {
                    let (a, b) = (small[row][column], column + shift < 852 && wide[row][column + shift])
                    both += a && b ? 1 : 0
                    either += a || b ? 1 : 0
                    original += a ? 1 : 0
                    drawn += b ? 1 : 0
                    if a { box = (min(box.0, column), min(box.1, row), max(box.2, column), max(box.3, row)) }
                }
            }
            let overlap = Double(both) / Double(max(either, 1))
            print("\(what) \(name): \(original) units at 640, \(drawn) at 4K, overlap \(overlap), the 640's at \(box)")
            if original > 0 { XCTAssertGreaterThan(overlap, 0.5, name) }
            XCTAssertGreaterThan(original, 0, "no \(what) at 640: \(name)")
        }
    }
}
