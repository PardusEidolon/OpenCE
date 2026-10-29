// The app's entry: game.wasm, the original's own code, played in the
// engine's window (GamePlayer.swift), on its main menu or on the map --map
// names, full screen with --full-screen. --control (or --socket PATH) opens
// the control socket beside the window, for the opence tool. --headless
// plays it with no window, drawn offscreen at --size (640x480 by default),
// answering the control socket until the game quits. --benchmark SECONDS
// plays --map (bloodgulch) offscreen at --size (3840x2160 by default) and
// prints its report.
import AppKit
import Metal
import OpenCEEngineCore
import OpenCEEngineKit
import OpenCEEngineRender

if CommandLine.arguments.contains("--version") {
    print("OpenCE Engine \(engineVersion())")
    exit(0)
}

/// The game's arguments: the command line's.
let launchArguments = CommandLine.arguments

/// The value after `name` among the arguments, or nil.
func argument(_ name: String) -> String? {
    let arguments = launchArguments
    guard let index = arguments.firstIndex(of: name), index + 1 < arguments.count else { return nil }
    return arguments[index + 1]
}

func fail(_ message: String) -> Never {
    FileHandle.standardError.write(Data("OpenCE Engine: \(message)\n".utf8))
    exit(1)
}

/// --size's width and height, or `fallback`'s.
func sizeArgument(_ fallback: String) -> (width: Int, height: Int) {
    let size = (argument("--size") ?? fallback).split(separator: "x").compactMap { Int($0) }
    guard size.count == 2, size.allSatisfy({ (1...16384).contains($0) }) else { fail("--size is WIDTHxHEIGHT") }
    return (size[0], size[1])
}

/// The control socket's path: --socket's, else the opence tool's own, or
/// nil when neither --socket nor --control asks for it.
func controlSocket() -> String? {
    if let path = argument("--socket") { return path }
    return launchArguments.contains("--control") || launchArguments.contains("--headless")
        ? defaultControlSocket() : nil
}

/// Opens the control socket on `game`, when asked for.
func listen(_ game: WasmGame) {
    guard let path = controlSocket() else { return }
    do {
        try game.listen(path: path)
        FileHandle.standardError.write(Data("OpenCE Engine: control socket: \(path)\n".utf8))
    } catch {
        fail("the control socket: \(error)")
    }
}

/// The benchmark of game.wasm (--benchmark SECONDS): --map (bloodgulch when
/// none is named) loaded from assets by --wasm (the build's game.wasm when
/// none is named) and played for --load-seconds (20) and then SECONDS, drawn
/// through GameRenderer offscreen at --size (3840x2160), its pipelines from
/// the archive in the wasm cache. Each frame is the slower of the game's CPU
/// frame and its GPU work (BenchmarkResult.game); the report is printed as
/// JSON (--baseline with a report kept before, to fail on a regression).
func benchmark(seconds text: String) -> Never {
    guard let seconds = Double(text), seconds.isFinite, seconds > 0 else { fail("--benchmark is seconds, not \(text)") }
    let size = sizeArgument("3840x2160")
    guard let load = Double(argument("--load-seconds") ?? "20"), load.isFinite, load >= 0 else {
        fail("--load-seconds is seconds")
    }
    let map = argument("--map") ?? "bloodgulch"
    let (game, renderer, saves) = offscreenGame(
        size: size, map: map, seconds: UInt32((load + seconds).rounded(.up)), defaultCache: "build/engine/wasm/cache")
    defer { try? FileManager.default.removeItem(at: saves) }
    let gpu = GPUTimes()
    renderer.presented = { gpu.append($0) }
    guard game.profile() else { fail("game.wasm started before it was profiled") }
    let end = play(game, renderer: renderer)
    // (the frames of the last SECONDS, the load and the first frames left out)
    let frames = Int(seconds * 60)
    let result = BenchmarkResult.game(
        width: size.width, height: size.height, seconds: seconds, cpuTimes: Array(end.frameMs.suffix(frames)),
        gpuTimes: gpu.times)
    let targetMs = 1000.0 / 120
    let name = "wasm_\(map)"
    print(result.json(path: name, targetMs: targetMs))
    if let baseline = argument("--baseline") {
        do {
            let lost = benchmarkRegressions(
                baseline: try benchmarkBaseline(URL(fileURLWithPath: baseline)),
                current: result.report(targetMs: targetMs))
            for line in lost {
                FileHandle.standardError.write(Data("\(name): \(line)\n".utf8))
            }
            if !lost.isEmpty { exit(2) }
        } catch {
            fail("\(baseline): \(error)")
        }
    }
    exit(0)
}

/// game.wasm played with no window (--headless): drawn offscreen at --size
/// (640x480), answering the control socket until it quits.
func headless() -> Never {
    let (game, renderer, saves) = offscreenGame(
        size: sizeArgument("640x480"), map: argument("--map"), seconds: nil,
        defaultCache: supportFolder("wasm"))
    defer { try? FileManager.default.removeItem(at: saves) }
    listen(game)
    _ = play(game, renderer: renderer)
    exit(0)
}

/// game.wasm loaded to draw offscreen at `size`, its saves in a folder of
/// its own (removed by the caller), and its pipelines from the archive in
/// the cache (--cache, else `defaultCache`).
func offscreenGame(
    size: (width: Int, height: Int), map: String?, seconds: UInt32?, defaultCache: String
) -> (WasmGame, GameRenderer, URL) {
    // (each folder made absolute: Wasmtime's cache takes no relative path)
    let absolute = { (path: String) in URL(fileURLWithPath: path).standardizedFileURL.path }
    let wasm = absolute(argument("--wasm") ?? "build/engine/wasm/game.wasm")
    let cache = absolute(argument("--cache") ?? defaultCache)
    guard let device = MTLCreateSystemDefaultDevice() else { fail("no Metal device") }
    guard let window = OffscreenWindow(device: device, width: size.width, height: size.height),
        let renderer = GameRenderer(device: device, window: window)
    else { fail("no offscreen window at \(size.width)x\(size.height)") }
    let saves = FileManager.default.temporaryDirectory.appendingPathComponent("opence-\(UUID().uuidString)")
    do {
        try FileManager.default.createDirectory(at: saves, withIntermediateDirectories: true)
        let game = try WasmGame(
            options: WasmGameOptions(
                wasm: wasm, data: absolute(argument("--data") ?? "assets"), saves: saves.path, cache: cache,
                width: UInt32(size.width), height: UInt32(size.height), map: map, seconds: seconds, settings: []))
        renderer.archive = PipelineArchive(
            device: device, url: URL(fileURLWithPath: cache).appendingPathComponent("pipelines.metallib"))
        renderer.playing = { game.ticks() > 0 }
        return (game, renderer, saves)
    } catch {
        fail("\(wasm): \(error)")
    }
}

/// Runs `game` into `renderer` until it quits, and answers how it ended.
func play(_ game: WasmGame, renderer: GameRenderer) -> GameEnd {
    do {
        try game.start(target: renderer)
    } catch {
        fail("game.wasm: \(error)")
    }
    let end = game.wait()
    renderer.finish()
    if let error = end.error { fail("game.wasm: \(error)") }
    return end
}

/// The GPU times GameRenderer hands back as each frame completes, in order.
final class GPUTimes: @unchecked Sendable {
    private let lock = NSLock()
    private var kept: [Double] = []

    func append(_ milliseconds: Double) {
        lock.lock()
        kept.append(milliseconds)
        lock.unlock()
    }

    var times: [Double] {
        lock.lock()
        defer { lock.unlock() }
        return kept
    }
}

/// The report in a benchmark's JSON kept before.
func benchmarkBaseline(_ url: URL) throws -> BenchmarkReport {
    let object = try JSONSerialization.jsonObject(with: Data(contentsOf: url))
    guard let fields = object as? [String: Any] else { fail("\(url.path) is not a benchmark's JSON") }
    func number(_ key: String) -> Double { (fields[key] as? NSNumber)?.doubleValue ?? 0 }
    return BenchmarkReport(
        frames: UInt32(clamping: Int(number("frames"))), meanMs: number("mean_ms"), p50Ms: number("p50_ms"),
        p99Ms: number("p99_ms"), p999Ms: number("p999_ms"), hitches: UInt32(clamping: Int(number("hitches"))),
        pacingMs: number("pacing_ms"))
}

if let seconds = argument("--benchmark") {
    benchmark(seconds: seconds)
}

if CommandLine.arguments.contains("--headless") {
    headless()
}

let app = NSApplication.shared
let delegate = GamePlayer()
app.delegate = delegate
app.setActivationPolicy(.regular)
app.run()
