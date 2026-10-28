#!/usr/bin/env python3
"""A C compiler for the reference traces that builds them as wasm32 and runs
them in the game's runtime (opence_runtime), so that a trace recorded from
the wasm build can be compared with the native one.

    CC="$PWD/tools/engine_wasm_cc.py" python3 tools/engine_foo_reference.py --check

The reference scripts (tools/engine_*_reference.py) compile their harness
with $CC, as `$CC <flags> source.c -o executable -lm`, and run the
executable. This compiles the same source for wasm32-wasip1 with the game's
ABI flags (tools/engine_wasm.py) into executable.wasm, and writes
executable as a script that runs it with the runtime's `opence-wasm`
($OPENCE_WASM_RUN), whose output is the harness's own. A script whose
--check passes this way has the same trace in the wasm build as natively.
"""

import os
import shlex
import subprocess
import sys
from pathlib import Path
from typing import List, Sequence

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from tools import engine_wasm  # noqa: E402

# the host compiler's flags a wasm build cannot take
HOST_ONLY = {"-lm", "-pthread", "-lpthread"}

# the game's ABI less its optimisation and debug levels, which the
# reference keeps as it asked
ABI_FLAGS = [flag for flag in engine_wasm.ABI_FLAGS if not flag.startswith(("-O", "-g"))]

# the diagnostics the game's build keeps as warnings, which clang 22 makes
# errors; they change no code, and the harnesses build natively with them as
# warnings (under -w)
DIAGNOSTIC_FLAGS = [flag for flag in engine_wasm.GAME_FLAGS if flag.startswith("-Wno-error=")]


def wasm_command(arguments: Sequence[str], output: Path) -> List[str]:
    """the clang command that builds the harness, with its own flags and the
    game's ABI, as a WASI command module at output"""
    include, lib = engine_wasm.wasi_paths()
    builtins = os.environ.get("HALO_WASM_BUILTINS")
    if not builtins:
        raise SystemExit("engine_wasm_cc: run inside nix develop (HALO_WASM_BUILTINS is unset)")
    kept = [argument for argument in arguments if argument not in HOST_ONLY]
    return [
        engine_wasm.clang(),
        *ABI_FLAGS,
        "-isystem",
        str(include),
        *kept,
        *DIAGNOSTIC_FLAGS,
        # (as engine_wasm.link: the unwrapped clang finds neither by itself)
        "-nostdlib",
        str(lib / "crt1-command.o"),
        f"-L{lib}",
        "-Wl,-z,stack-size=8388608",
        "-lc",
        builtins,
        "-o",
        str(output),
    ]


def runner() -> str:
    run = os.environ.get("OPENCE_WASM_RUN")
    if not run:
        raise SystemExit("engine_wasm_cc: OPENCE_WASM_RUN names no runner (tools/engine_wasm_traces.py sets it)")
    return run


def main(arguments: Sequence[str]) -> int:
    arguments = list(arguments)
    if "-o" not in arguments or arguments.index("-o") + 1 >= len(arguments):
        raise SystemExit("engine_wasm_cc: no -o executable")
    at = arguments.index("-o")
    executable = Path(arguments[at + 1])
    rest = arguments[:at] + arguments[at + 2 :]
    module = executable.with_suffix(".wasm")
    result = subprocess.run(wasm_command(rest, module))
    if result.returncode:
        return result.returncode
    executable.write_text(f"#!/bin/sh\nexec {shlex.quote(runner())} {shlex.quote(str(module))} \"$@\"\n")
    executable.chmod(0o755)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
