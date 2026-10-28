// The app's game: game.wasm, the original's own code, played in the
// engine's window. It starts on its main menu, or on the map --map names
// (--full-screen for full screen), with the user's maps from the data
// root's assets/maps. GameRenderer draws its commands into the view's own
// layer, whose drawable follows the window's size; GameInput hands it the
// keys, the mouse and the wheel, and SoundOutput plays its mix. The app
// quits when the game does. Escape is the game's; Command-Escape lets the
// mouse go.
import AppKit
import Metal
import OpenCEEngineCore
import OpenCEEngineKit
import OpenCEEngineRender
import OpenCEEngineWindow

/// game.wasm: in the bundle's resources, else (run from the repository)
/// tools/engine_wasm.py's build.
func gameWasm() -> String {
    if let given = argument("--wasm") { return URL(fileURLWithPath: given).standardizedFileURL.path }
    if let bundled = Bundle.main.url(forResource: "game", withExtension: "wasm") { return bundled.path }
    return MapLibrary.dataRoot.appendingPathComponent("build/engine/wasm/game.wasm").path
}

/// The app's folder in Application Support (the game's saves and its
/// compiled modules and pipelines under it).
func supportFolder(_ name: String) -> String {
    let support =
        FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first
        ?? URL(fileURLWithPath: NSTemporaryDirectory())
    let folder = support.appendingPathComponent("OpenCE Engine", isDirectory: true).appendingPathComponent(name)
    try? FileManager.default.createDirectory(at: folder, withIntermediateDirectories: true)
    return folder.path
}

/// The player's menu: quit, and full screen.
@MainActor
func playerMenu() -> NSMenu {
    let menu = NSMenu()
    let appItem = NSMenuItem()
    appItem.submenu = NSMenu()
    appItem.submenu?.addItem(
        withTitle: "Quit OpenCE Engine", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q")
    menu.addItem(appItem)
    let viewItem = NSMenuItem()
    let view = NSMenu(title: "View")
    let fullScreen = view.addItem(
        withTitle: "Enter Full Screen", action: #selector(NSWindow.toggleFullScreen(_:)), keyEquivalent: "f")
    fullScreen.keyEquivalentModifierMask = [.command, .control]
    viewItem.submenu = view
    menu.addItem(viewItem)
    return menu
}

@MainActor
final class GamePlayer: NSObject, NSApplicationDelegate {
    private var controller: EngineWindowController?
    private var game: WasmGame?
    private var input: GameInput?
    private var sound: SoundOutput?
    private var poll: Timer?

    func applicationDidFinishLaunching(_ notification: Notification) {
        NSApp.mainMenu = playerMenu()
        let controller = EngineWindowController()
        self.controller = controller
        controller.show(fullScreen: CommandLine.arguments.contains("--full-screen"))
        let view = controller.window.engineView
        guard let device = view.device else { fail("no Metal device") }
        // (the game draws at the size it started at, which has no resize:
        // its own layer, scaled to the window's bounds)
        // (full screen: the screen's pixels, which the window grows to)
        let screen = NSScreen.main
        let size =
            CommandLine.arguments.contains("--full-screen") && screen != nil
            ? Viewport.pixelSize(points: screen?.frame.size ?? .zero, scale: screen?.backingScaleFactor ?? 1)
            : view.sizeInPixels
        let layer = CAMetalLayer()
        layer.device = device
        layer.pixelFormat = .bgra8Unorm
        layer.framebufferOnly = true
        layer.drawableSize = CGSize(width: max(size.width, 1), height: max(size.height, 1))
        layer.contentsGravity = .resizeAspect
        layer.frame = view.bounds
        view.metalLayer.addSublayer(layer)
        view.onResize = { [weak view] _, _ in
            guard let view else { return }
            CATransaction.begin()
            CATransaction.setDisableActions(true)
            layer.frame = view.bounds
            CATransaction.commit()
        }
        let window = LayerWindow(layer: layer)
        guard let renderer = GameRenderer(device: device, window: window) else { fail("no renderer") }
        let wasm = gameWasm()
        let cache = supportFolder("wasm")
        let game: WasmGame
        do {
            game = try WasmGame(
                options: WasmGameOptions(
                    wasm: wasm, data: MapLibrary.dataRoot.appendingPathComponent("assets").path,
                    saves: supportFolder("saves"), cache: cache, width: UInt32(max(size.width, 1)),
                    height: UInt32(max(size.height, 1)), map: argument("--map"), seconds: nil, settings: []))
        } catch {
            fail("\(wasm): \(error)")
        }
        renderer.archive = PipelineArchive(
            device: device, url: URL(fileURLWithPath: cache).appendingPathComponent("pipelines.metallib"))
        renderer.playing = { game.ticks() > 0 }
        self.game = game
        input = GameInput(game: game)
        view.keyFilter = { [weak self] event in
            // (Command-Escape is the window's: it lets the mouse go)
            let flags = event.modifierFlags
            if event.keyCode == KeyCode.escape, flags.contains(.command) { return false }
            self?.input?.key(
                code: event.keyCode, characters: event.characters ?? "", control: flags.contains(.control),
                shift: flags.contains(.shift), alt: flags.contains(.option), down: event.type == .keyDown)
            return true
        }
        view.scrollFilter = { [weak self] delta, precise in
            self?.input?.scroll(delta: delta, precise: precise)
            return true
        }
        controller.capturesMouse = true
        // (the held buttons and modifiers, and the motion, at the display's rate)
        let interval = 1 / Double(max(view.maximumFramesPerSecond, 60))
        poll = Timer.scheduledTimer(withTimeInterval: interval, repeats: true) { [weak self] _ in
            MainActor.assumeIsolated { self?.handInput() }
        }
        sound = SoundOutput(speakers: game.speakers())
        do {
            try game.start(target: renderer)
        } catch {
            fail("\(wasm): \(error)")
        }
        // (the game ends when it quits; the app with it)
        Thread.detachNewThread {
            let end = game.wait()
            renderer.finish()
            DispatchQueue.main.async {
                if let error = end.error { FileHandle.standardError.write(Data("OpenCE Engine: \(error)\n".utf8)) }
                NSApp.terminate(nil)
            }
        }
    }

    private func handInput() {
        guard let controller else { return }
        input?.held(controller.keys.codes, motion: controller.takeMouseMotion())
    }

    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool { true }
}
