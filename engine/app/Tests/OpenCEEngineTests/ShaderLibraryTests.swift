import Foundation
import Metal
import OpenCEEngineRender
import XCTest

/// The shaders linked ahead of time: `default.metallib`, linked from each
/// shader's `.air` by tools/engine_build.py, is loaded with
/// `makeLibrary(URL:)`, and a pass whose source is the one linked gets the
/// linked library without compiling; an edited source (a hot reload) is
/// compiled as before.
final class ShaderLibraryTests: XCTestCase {
    func device() throws -> MTLDevice {
        try XCTUnwrap(MTLCreateSystemDefaultDevice(), "no Metal device")
    }

    /// Every shader but the bake's (compiled per bake, with safe maths).
    let linked = Shader.allCases.filter { $0 != .bake }

    func testTheMetallibHoldsEveryLinkedShadersFunctions() throws {
        let device = try device()
        let library = try XCTUnwrap(
            try ShaderLibrary.linked(device: device), "no default.metallib (OPENCE_METALLIB or the bundle's)")
        let names = Set(library.functionNames)
        for shader in linked {
            let compiled = try device.makeLibrary(source: try ShaderSources.source(shader), options: nil)
            for function in compiled.functionNames {
                XCTAssertTrue(names.contains(function), "\(shader.file): \(function)")
            }
        }
        XCTAssertFalse(names.contains("bake_irradiance"))
    }

    func testALinkedSourceIsNotCompiled() throws {
        let device = try device()
        let library = try XCTUnwrap(try ShaderLibrary.linked(device: device))
        let before = ShaderLibrary.compiled
        for shader in linked {
            XCTAssertTrue(
                try ShaderLibrary.make(device: device, source: try ShaderSources.source(shader)) === library,
                shader.file)
        }
        XCTAssertEqual(ShaderLibrary.compiled, before)
    }

    func testAnEditedSourceIsCompiled() throws {
        let device = try device()
        let library = try XCTUnwrap(try ShaderLibrary.linked(device: device))
        let before = ShaderLibrary.compiled
        let edited = try ShaderLibrary.make(
            device: device, source: try ShaderSources.source(.hud) + "\n// edited\n")
        XCTAssertFalse(edited === library)
        XCTAssertTrue(edited.functionNames.contains("hud_vertex"))
        XCTAssertEqual(ShaderLibrary.compiled, before + 1)
    }

    /// No pass compiles its source itself: each goes through ShaderLibrary
    /// (the bake's PathTracer apart).
    func testThePassesGoThroughTheLinkedLibrary() throws {
        var sources = URL(fileURLWithPath: #filePath)
        for _ in 0..<3 {
            sources.deleteLastPathComponent()
        }
        sources.appendPathComponent("Sources/OpenCEEngineRender")
        let files = try XCTUnwrap(FileManager.default.enumerator(at: sources, includingPropertiesForKeys: nil))
        var found = 0
        for case let file as URL in files where file.pathExtension == "swift" {
            found += 1
            let name = file.lastPathComponent
            if name == "ShaderLibrary.swift" || name == "PathTracer.swift" {
                continue
            }
            let text = try String(contentsOf: file, encoding: .utf8)
            XCTAssertFalse(text.contains("makeLibrary(source:"), name)
        }
        XCTAssertGreaterThan(found, 10)
    }
}
