#!/usr/bin/env python3
"""Checks every reference trace against the wasm build.

    nix develop -c python3 tools/engine_wasm_traces.py [--only NAME] [--json PATH]

Each tools/engine_*_reference.py compiles the functions it traces from
source/ with $CC and checks that their trace is the one recorded natively.
This runs each script's --check with CC set to tools/engine_wasm_cc.py, so
the same functions are built for wasm32 with the game's ABI and run in the
game's runtime (opence-wasm), and reports which traces match. A script that
passes here traces the same in game.wasm as natively.
"""

import argparse
import json
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Dict, List, Optional

ROOT = Path(__file__).resolve().parent.parent
CARGO_MANIFEST = ROOT / "engine" / "Cargo.toml"
RUNNER = ROOT / "engine" / "target" / "release" / "opence-wasm"
CACHE = ROOT / "build" / "engine" / "wasm" / "cache"
# (the slowest native trace takes a few seconds; a wasm one tens)
TIMEOUT = 600


def scripts(only: Optional[str] = None) -> List[Path]:
    """the reference scripts, by name, or the one whose name contains only"""
    found = sorted((ROOT / "tools").glob("engine_*_reference.py"))
    if only:
        found = [script for script in found if only in script.name]
    return found


def build_runner() -> Path:
    subprocess.run(
        [
            "cargo",
            "build",
            "-q",
            "--release",
            "--manifest-path",
            str(CARGO_MANIFEST),
            "-p",
            "opence_runtime",
            "--bin",
            "opence-wasm",
        ],
        check=True,
    )
    return RUNNER


def environment(runner: Path) -> Dict[str, str]:
    CACHE.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ)
    env["CC"] = str(ROOT / "tools" / "engine_wasm_cc.py")
    env["OPENCE_WASM_RUN"] = str(runner)
    env["OPENCE_WASM_CACHE"] = str(CACHE)
    return env


def check(script: Path, env: Dict[str, str]) -> Dict[str, object]:
    start = time.monotonic()
    try:
        result = subprocess.run(
            [sys.executable, str(script), "--check"],
            env=env,
            capture_output=True,
            text=True,
            timeout=TIMEOUT,
        )
        passed, detail = result.returncode == 0, (result.stdout + result.stderr).strip()
    except subprocess.TimeoutExpired:
        passed, detail = False, f"timed out after {TIMEOUT} s"
    return {
        "script": script.name,
        "passed": passed,
        "seconds": round(time.monotonic() - start, 2),
        # (the end of the output says why it failed)
        "detail": detail[-2000:],
    }


def main(arguments: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--only", help="run only the scripts whose name contains this")
    parser.add_argument("--json", type=Path, help="write the results here")
    parser.add_argument("--jobs", type=int, default=os.cpu_count() or 4)
    options = parser.parse_args(arguments)
    if not os.environ.get("HALO_WASM_BUILTINS"):
        raise SystemExit("engine_wasm_traces: run inside nix develop")
    env = environment(build_runner())
    chosen = scripts(options.only)
    with ThreadPoolExecutor(max_workers=options.jobs) as pool:
        results = list(pool.map(lambda script: check(script, env), chosen))
    for result in results:
        mark = "match" if result["passed"] else "DIFFERS"
        print(f"{mark:8} {result['seconds']:7.2f} s  {result['script']}")
        if not result["passed"]:
            print("         " + str(result["detail"]).splitlines()[-1] if result["detail"] else "")
    passed = sum(1 for result in results if result["passed"])
    print(f"{passed} of {len(results)} traces match in the wasm build")
    if options.json:
        options.json.parent.mkdir(parents=True, exist_ok=True)
        options.json.write_text(json.dumps(results, indent=2) + "\n")
    return 0 if passed == len(results) else 1


if __name__ == "__main__":
    sys.exit(main())
