// The renderer's shaders linked ahead of time: tools/engine_build.py
// compiles each shader to .air and links all but the bake's into
// default.metallib, with default.metallib.sources beside it naming each
// source linked by its SHA-256. A pass hands its source here: if it is one
// of those linked, the pass gets the linked library and nothing is
// compiled at run time; an edited source (a hot reload) is compiled as
// before.
import CryptoKit
import Foundation
import Metal

public enum ShaderLibrary {
    private static let state = LinkedShaders()

    /// How many sources have been compiled at run time, not found linked.
    public static var compiled: Int {
        state.lock.lock()
        defer { state.lock.unlock() }
        return state.compiled
    }

    /// The metallib: `OPENCE_METALLIB` (the tests', set by the build), or
    /// the bundle's Resources/default.metallib.
    public static func url(bundle: Bundle = .main) -> URL? {
        if let path = ProcessInfo.processInfo.environment["OPENCE_METALLIB"] {
            return URL(fileURLWithPath: path)
        }
        let bundled = bundle.resourceURL?.appendingPathComponent("default.metallib")
        return bundled.flatMap { FileManager.default.fileExists(atPath: $0.path) ? $0 : nil }
    }

    /// The linked library for `device`, loaded once; nil if there is no
    /// metallib (a build that did not link one).
    public static func linked(device: MTLDevice, bundle: Bundle = .main) throws -> MTLLibrary? {
        try entry(device: device, bundle: bundle)?.0
    }

    /// The library for `source`: the linked one when `source` is one of
    /// those linked, else `source` compiled.
    public static func make(device: MTLDevice, source: String) throws -> MTLLibrary {
        if let found = try entry(device: device, bundle: .main), found.1.contains(hash(source)) {
            return found.0
        }
        let library = try device.makeLibrary(source: source, options: nil)
        state.lock.lock()
        state.compiled += 1
        state.lock.unlock()
        return library
    }

    private static func entry(device: MTLDevice, bundle: Bundle) throws -> (MTLLibrary, Set<String>)? {
        state.lock.lock()
        defer { state.lock.unlock() }
        let key = ObjectIdentifier(device)
        if let found = state.loaded[key] {
            return found
        }
        guard let url = url(bundle: bundle) else {
            return nil
        }
        let library: MTLLibrary
        do {
            library = try device.makeLibrary(URL: url)
        } catch {
            throw LevelRendererError.shaders("\(url.lastPathComponent): \(error.localizedDescription)")
        }
        let listed = (try? String(contentsOf: url.appendingPathExtension("sources"), encoding: .utf8)) ?? ""
        let hashes = Set(listed.split(separator: "\n").compactMap { $0.split(separator: " ").first.map(String.init) })
        state.loaded[key] = (library, hashes)
        return (library, hashes)
    }

    private static func hash(_ source: String) -> String {
        SHA256.hash(data: Data(source.utf8)).map { String(format: "%02x", $0) }.joined()
    }
}

/// The libraries loaded and the count compiled, kept under the lock.
private final class LinkedShaders: @unchecked Sendable {
    let lock = NSLock()
    /// the linked library and the hashes of its sources, by device
    var loaded: [ObjectIdentifier: (MTLLibrary, Set<String>)] = [:]
    var compiled = 0
}
