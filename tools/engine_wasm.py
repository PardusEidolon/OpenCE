"""Build the game, source/ unchanged, as build/engine/wasm/game.wasm.

The game runs in Wasmtime inside the macOS app (track native_port_20261028).
The module is compiled from the web build's flags (tools/web_build.py), which
already made source/ correct for wasm32: an ILP32 target, as the game's data
formats need, where a call must match its callee's type exactly (the link's
--fatal-warnings), and where the game files that call variadic functions
without a prototype get one (tools/android_build.py VARIADIC_PROTOTYPE_FILES).

What goes into the module is the game: source/ less what port.json excludes,
the port's own game units (port/linux/game), stb_vorbis and musl's maths, so
every run computes the same results. The C library is wasi-libc. Every
function the game calls that none of these defines is left undefined and
becomes an import from "env": that set is the platform layer, which the Rust
runtime provides (engine/crates/runtime). tools/engine_wasm_imports.txt lists
it; the test (tools/test_engine_wasm.py) fails when the module's imports or
exports differ from the lists, so a change to the platform layer's surface is
always a reviewed one.

Units are not optimised together at the link (no -flto): LLVM's link-time
optimisation turns a call whose parameter types differ only in width into a
trap (tools/web_build.py).

Run inside nix develop, which puts a clang with the wasm32 target, wasm-ld
and wasi-libc (HALO_WASI_LIBC, HALO_WASI_LIBC_DEV) in the environment:

    python3 tools/engine_wasm.py            # build game.wasm
    python3 tools/engine_wasm.py --imports  # print the module's imports
"""

from __future__ import annotations

import argparse
import concurrent.futures
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional, Sequence, Tuple

ROOT = Path(__file__).resolve().parent.parent
if __package__ in (None, ""):
    sys.path.insert(0, str(ROOT))

from tools.android_build import VARIADIC_PROTOTYPE_FILES  # noqa: E402
from tools.linux_build import (  # noqa: E402
    MUSL_MATH_DIR,
    STB_VORBIS_SOURCE,
    XDK_INCLUDE,
    game_sources,
    musl_math_sources,
)
from tools.web_build import GAME_FLAGS, PROTOTYPE_FILES, WEB_CODE_FLAGS  # noqa: E402

LINUX_DIR = Path("port/linux")
ANDROID_DIR = Path("port/android")
PORT_CONFIG = LINUX_DIR / "port.json"
BUILD = Path("build/engine/wasm")
IMPORTS_LIST = Path("tools/engine_wasm_imports.txt")
EXPORTS_LIST = Path("tools/engine_wasm_exports.txt")

# the game's target: wasm32 with atomics and a shared memory, so its threads
# are its own, each the module instantiated again over the same memory
# (wasi-threads; opence_runtime's threads module answers its imports)
TARGET = "wasm32-wasip1-threads"

# The game's address space. The module starts with 2 GB, which wasi-libc's
# malloc takes as its first region (__heap_base to __heap_end, the end of
# the initial memory). The platform then grows the memory past the Xbox's
# window, 0x80000000 to 0xA0000000 (port/linux/src/platform.h, at the
# desktop sizes), before the game runs, so the heap's later growth lands
# above it and the window's pages are the game's alone. The memory may grow
# to wasm32's 4 GB.
MEMORY_BYTES = 0x80000000
MAXIMUM_MEMORY_BYTES = 0x100000000
# (the game recurses deeply in places: scripts, the BSP's walks)
STACK_BYTES = 8 * 1024 * 1024

# The ABI the game was written for, as the web build reproduces it, with
# its threads (-pthread: atomics and bulk memory).
ABI_FLAGS = [
    f"--target={TARGET}",
    "-pthread",
    "-DHALO_WASMTIME=1",
    "-fshort-wchar",
    "-ffp-contract=off",
    "-msimd128",
    "-mbulk-memory",
    "-mnontrapping-fptoint",
    "-O2",
    "-g2",
]

# what the module exports to the runtime: the game's entry point, the C
# heap, which the platform layer allocates the game's buffers from, and what
# its threads need (CreateThread calls the game's pthread_create, and a new
# thread starts at wasi_thread_start)
EXPORTS = ["halo_game_main", "malloc", "free", "pthread_create", "pthread_detach", "wasi_thread_start"]

# wasi-libc for each target, as nix develop names it
WASI_LIBC = {
    "wasm32-wasip1-threads": "HALO_WASI_LIBC_THREADS",
    "wasm32-wasip1": "HALO_WASI_LIBC",
}


def wasi_paths(target: str = TARGET) -> Tuple[Path, Path]:
    """wasi-libc's include and lib directories for target (nix develop's
    HALO_WASI_LIBC and HALO_WASI_LIBC_DEV, or their _THREADS forms)"""
    name = WASI_LIBC[target]
    dev = os.environ.get(f"{name}_DEV")
    lib = os.environ.get(name)
    if not dev or not lib:
        raise SystemExit(f"engine_wasm: run inside nix develop ({name} and {name}_DEV are unset)")
    return Path(dev) / "include" / target, Path(lib) / "lib" / target


def clang() -> str:
    return os.environ.get("HALO_WASM_CC") or "clang"


def semantics_header(build: Path) -> Path:
    """MSVC's inline and tag semantics for the game
    (tools/linux_msvc_semantics.py, as the Linux build makes it)"""
    header = build / "halo_msvc_semantics.h"
    scan = ["--all-inlines", "--tags", "source", "--inlines", "source", "--inlines", str(XDK_INCLUDE)]
    subprocess.run(
        [sys.executable, "tools/linux_msvc_semantics.py", "--output", str(header), *scan],
        check=True,
        cwd=ROOT,
    )
    return header


def units(config: Dict[str, object], build: Path, include: Path) -> List[Tuple[Path, List[str]]]:
    """every unit of the module, with its flags"""
    prefix = LINUX_DIR / "include" / "halo_linux_prefix.h"
    common = [*ABI_FLAGS, f"-isystem{include}"]
    game = [
        *common,
        # clang 22 crashes emitting a common symbol for wasm32-wasip1 (in
        # AsmPrinter::emitGlobalVariable), so the game's tentative
        # definitions are compiled as definitions, and each must be defined
        # in one unit only
        *("-fno-common" if flag == "-fcommon" else flag for flag in WEB_CODE_FLAGS),
        *GAME_FLAGS,
        "-include",
        str(prefix),
        "-include",
        str(build / "halo_msvc_semantics.h"),
        f"-I{LINUX_DIR}/include",
        "-iquote",
        str(config["game_sources"]),
        # (port.json's defines and include directories, as
        # game_defines_and_includes has them for a shell)
        *(f"-D{define}" for define in config["game"].get("defines", [])),  # type: ignore[index]
        *(f"-I{directory}" for directory in config["game"].get("include_dirs", [])),  # type: ignore[index]
        "-idirafter",
        str(XDK_INCLUDE),
    ]
    result: List[Tuple[Path, List[str]]] = []
    for source in game_sources(config):  # type: ignore[arg-type]
        name = source.as_posix()
        flags = list(game)
        if name in VARIADIC_PROTOTYPE_FILES:
            flags += ["-include", f"{ANDROID_DIR}/include/halo_android_variadic_prototypes.h"]
        if name in PROTOTYPE_FILES:
            flags += ["-include", PROTOTYPE_FILES[name]]
        if name == "source/shell/shell_xbox.c":
            # the runtime calls the game's main once the platform is up
            flags += ["-Dmain=halo_game_main"]
        result.append((source, flags))
    for source in sorted(Path(str(config["game_sources"])).glob("*.c")):
        result.append((source, list(game)))
    result.append((STB_VORBIS_SOURCE, list(game)))
    musl = [*common, "-std=gnu11", "-w", f"-I{MUSL_MATH_DIR}/include", "-include", f"{MUSL_MATH_DIR}/include/libm.h"]
    for source in musl_math_sources():
        result.append((source, list(musl)))
    return result


def compile_unit(source: Path, flags: Sequence[str], obj: Path) -> Optional[str]:
    """compile one unit; its errors, or None"""
    obj.parent.mkdir(parents=True, exist_ok=True)
    if obj.exists() and obj.stat().st_mtime >= source.stat().st_mtime and obj.with_suffix(".flags").exists():
        if obj.with_suffix(".flags").read_text() == "\n".join(flags) and not stale_dependencies(obj):
            return None
    done = subprocess.run(
        [clang(), *flags, "-MMD", "-MF", str(obj.with_suffix(".d")), "-c", str(source), "-o", str(obj)],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if done.returncode != 0:
        return f"{source}:\n{done.stderr}"
    obj.with_suffix(".flags").write_text("\n".join(flags))
    return None


def stale_dependencies(obj: Path) -> bool:
    """a header the unit included is newer than its object"""
    depfile = obj.with_suffix(".d")
    if not depfile.exists():
        return True
    built = obj.stat().st_mtime
    text = depfile.read_text().replace("\\\n", " ")
    for name in text.split(":", 1)[-1].split():
        path = ROOT / name if not os.path.isabs(name) else Path(name)
        if path.exists() and path.stat().st_mtime > built:
            return True
    return False


def link(objects: Sequence[Path], lib: Path, output: Path) -> None:
    builtins = os.environ.get("HALO_WASM_BUILTINS")
    if not builtins:
        raise SystemExit("engine_wasm: run inside nix develop (HALO_WASM_BUILTINS is unset)")
    flags = [
        clang(),
        f"--target={TARGET}",
        "-mexec-model=reactor",
        # wasi-libc's start file and compiler-rt's builtins, named here: the
        # unwrapped clang has neither where it would look
        "-nostdlib",
        str(lib / "crt1-reactor.o"),
        f"-L{lib}",
        "-Wl,--fatal-warnings",
        # the platform layer: every function the game calls that the module
        # does not define is imported from "env"
        "-Wl,--allow-undefined",
        f"-Wl,--initial-memory={MEMORY_BYTES}",
        f"-Wl,--max-memory={MAXIMUM_MEMORY_BYTES}",
        f"-Wl,-z,stack-size={STACK_BYTES}",
        # the memory, shared between the game's threads, is the runtime's,
        # and the function table is exported for the completion routines
        "-pthread",
        "-Wl,--shared-memory",
        "-Wl,--import-memory",
        "-Wl,--export-table",
        *(f"-Wl,--export={name}" for name in EXPORTS),
        "-lsetjmp",
        "-lwasi-emulated-mman",
        "-lc",
        builtins,
        "-o",
        str(output),
    ]
    rsp = output.with_suffix(".rsp")
    # (quoted: "saved games" and "saved films" have spaces)
    rsp.write_text("\n".join(f'"{obj}"' for obj in objects))
    subprocess.run([*flags, f"@{rsp}"], cwd=ROOT, check=True)


def build(jobs: int) -> Path:
    config = json.loads((ROOT / PORT_CONFIG).read_text(encoding="utf-8"))
    out = ROOT / BUILD
    out.mkdir(parents=True, exist_ok=True)
    include, lib = wasi_paths()
    semantics_header(out)
    work = units(config, out, include)
    errors: List[str] = []
    objects: List[Path] = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=jobs) as pool:
        futures = []
        for source, flags in work:
            obj = out / "obj" / source.with_suffix(".o")
            objects.append(obj)
            futures.append(pool.submit(compile_unit, source, flags, obj))
        for future in futures:
            error = future.result()
            if error:
                errors.append(error)
    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        raise SystemExit(f"engine_wasm: {len(errors)} of {len(work)} units failed")
    output = out / "game.wasm"
    link(objects, lib, output)
    return output


def module_imports(module: Path) -> List[str]:
    """the module's imports, "module.name: type", sorted"""
    return sorted(_section(module, "import"))


def module_exports(module: Path) -> List[str]:
    return sorted(_section(module, "export"))


def _section(module: Path, kind: str) -> List[str]:
    """a module's imports or exports, read from its binary (the wasm
    format's import and export sections, read with bounds checks)"""
    from tools.wasm_module import read_module

    parsed = read_module(module.read_bytes())
    return parsed.imports if kind == "import" else parsed.exports


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--jobs", type=int, default=os.cpu_count() or 8)
    parser.add_argument("--imports", action="store_true", help="print the built module's imports")
    parser.add_argument("--exports", action="store_true", help="print the built module's exports")
    parser.add_argument("--write-lists", action="store_true", help="write the import and export lists")
    args = parser.parse_args()
    if shutil.which(clang()) is None:
        raise SystemExit("engine_wasm: no clang (run inside nix develop)")
    output = build(args.jobs)
    if args.imports:
        print("\n".join(module_imports(output)))
    if args.exports:
        print("\n".join(module_exports(output)))
    if args.write_lists:
        (ROOT / IMPORTS_LIST).write_text("\n".join(module_imports(output)) + "\n")
        (ROOT / EXPORTS_LIST).write_text("\n".join(module_exports(output)) + "\n")
    print(output)


if __name__ == "__main__":
    main()
