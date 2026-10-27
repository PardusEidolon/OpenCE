// A developer launch: the arguments --map and its companions give, and
// the drawing switches it can set, each of which the renderer reads.
import OpenCEEngineKit
import OpenCEEngineRender
import XCTest

final class DeveloperTests: XCTestCase {
    /// Each switch offered changes what the renderer reads when it alone
    /// is turned off, so none a launch can set does nothing.
    func testEverySwitchOfferedReachesTheRenderer() {
        let all = RenderSwitches(flag: { _ in true }, value: { _ in "" })
        for name in RenderSwitches.flags {
            XCTAssertNotEqual(RenderSwitches(flag: { $0 != name }, value: { _ in "" }), all, name)
        }
    }

    func testThePlainLaunchIsDevgulchAtItsStart() {
        let launch = DeveloperLaunch()
        XCTAssertEqual(launch.arguments, ["--map", "devgulch"])
        XCTAssertTrue(launch.settings.isEmpty)
        XCTAssertTrue(launch.commands.isEmpty)
        XCTAssertNil(DeveloperLaunch(arguments: ["OpenCE Engine", "--fly"]), "no --map: not a launch")
        XCTAssertEqual(DeveloperLaunch(arguments: ["--map", "bloodgulch", "--bsp", "0"])?.level, "bloodgulch")
    }

    func testALaunchsArgumentsAreReadBackByTheGame() throws {
        let launch = DeveloperLaunch(
            level: "devgulch", station: "vehicles", view: .normals, variables: ["render.gtao": "false"],
            capture: "/tmp/devgulch-vehicles.png")
        XCTAssertEqual(
            launch.arguments,
            [
                "--map", "devgulch", "--station", "vehicles", "--view", "normals",
                "--set", "render.gtao=false", "--capture", "/tmp/devgulch-vehicles.png",
            ])
        let read = try XCTUnwrap(DeveloperLaunch(arguments: ["OpenCE Engine", "--frame-times"] + launch.arguments))
        XCTAssertEqual(read, launch)
        XCTAssertEqual(read.settings.map(\.name), ["render.gtao", "render.view"])
        XCTAssertEqual(read.settings.map(\.value), ["false", "normals"])
        XCTAssertEqual(read.commands, ["station vehicles", "frame \"/tmp/devgulch-vehicles.png\""])
    }
}
