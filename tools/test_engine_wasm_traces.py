"""The reference traces in the wasm build: the compiler wrapper
(tools/engine_wasm_cc.py) and the driver that runs every trace through it
(tools/engine_wasm_traces.py).

    python -m pytest -q tools/test_engine_wasm_traces.py

The test that runs a trace needs the wasm toolchain nix develop provides
(HALO_WASM_BUILTINS); without it, it skips.
"""

import os
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from tools import engine_wasm, engine_wasm_cc, engine_wasm_traces  # noqa: E402


@pytest.fixture
def toolchain(monkeypatch, tmp_path):
    monkeypatch.setattr(engine_wasm, "wasi_paths", lambda: (tmp_path / "include", tmp_path / "lib"))
    monkeypatch.setattr(engine_wasm, "clang", lambda: "clang-wasm")
    monkeypatch.setenv("HALO_WASM_BUILTINS", str(tmp_path / "builtins.a"))
    return tmp_path


def test_the_harness_keeps_its_own_optimisation_and_takes_the_game_s_abi(toolchain):
    command = engine_wasm_cc.wasm_command(
        ["-std=gnu99", "-O1", "-ffp-contract=off", "-w", "h.c", "-lm"], toolchain / "h.wasm"
    )
    assert command[0] == "clang-wasm"
    assert f"--target={engine_wasm.TARGET}" in command
    assert "-O1" in command and "-O2" not in command and "-g2" not in command
    assert "-lm" not in command
    assert str(toolchain / "lib" / "crt1-command.o") in command
    assert command[-2:] == ["-o", str(toolchain / "h.wasm")]


def test_the_harness_keeps_the_game_s_diagnostics_as_warnings(toolchain):
    command = engine_wasm_cc.wasm_command(["h.c"], toolchain / "h.wasm")
    assert "-Wno-error=incompatible-pointer-types" in command
    assert engine_wasm_cc.DIAGNOSTIC_FLAGS
    assert all(flag in engine_wasm.GAME_FLAGS for flag in engine_wasm_cc.DIAGNOSTIC_FLAGS)


def test_the_executable_runs_the_module_in_the_runner(toolchain, monkeypatch):
    built = []
    monkeypatch.setattr(
        subprocess, "run", lambda command: built.append(command) or subprocess.CompletedProcess(command, 0)
    )
    monkeypatch.setenv("OPENCE_WASM_RUN", "/opt/opence wasm")
    executable = toolchain / "trace"
    assert engine_wasm_cc.main(["h.c", "-o", str(executable), "-lm"]) == 0
    assert built and built[0][-1] == str(toolchain / "trace.wasm")
    script = executable.read_text()
    assert script.startswith("#!/bin/sh\n")
    assert "'/opt/opence wasm'" in script and str(toolchain / "trace.wasm") in script
    assert os.access(executable, os.X_OK)


def test_a_failed_compile_writes_no_executable(toolchain, monkeypatch):
    monkeypatch.setattr(subprocess, "run", lambda command: subprocess.CompletedProcess(command, 1))
    monkeypatch.setenv("OPENCE_WASM_RUN", "runner")
    assert engine_wasm_cc.main(["h.c", "-o", str(toolchain / "trace")]) == 1
    assert not (toolchain / "trace").exists()


def test_no_output_is_an_error(toolchain):
    with pytest.raises(SystemExit):
        engine_wasm_cc.main(["h.c"])
    with pytest.raises(SystemExit):
        engine_wasm_cc.main(["h.c", "-o"])


def test_the_driver_finds_every_reference_script():
    found = engine_wasm_traces.scripts()
    every = sorted(script.name for script in (ROOT / "tools").glob("engine_*_reference.py"))
    assert [script.name for script in found] == every
    assert all(script.name.endswith("_reference.py") for script in found)
    assert [script.name for script in engine_wasm_traces.scripts("facing")] == ["engine_facing_reference.py"]


@pytest.mark.skipif(not os.environ.get("HALO_WASM_BUILTINS"), reason="needs nix develop's wasm toolchain")
def test_a_trace_matches_in_the_wasm_build():
    assert engine_wasm_traces.main(["--only", "facing_reference"]) == 0
