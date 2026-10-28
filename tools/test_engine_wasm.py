"""game.wasm (tools/engine_wasm.py) and the reader of its imports and exports
(tools/wasm_module.py): modules made here, malformed ones among them, and the
built module's imports and exports against the reviewed lists.

    python -m pytest -q tools/test_engine_wasm.py

The built module's test needs the wasm toolchain nix develop provides
(HALO_WASM_CC); without it, it skips.
"""

import os
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from tools import engine_wasm  # noqa: E402
from tools.wasm_module import MalformedModule, read_module  # noqa: E402

I32, I64, F32, F64 = 0x7F, 0x7E, 0x7D, 0x7C


def leb(value):
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        if value:
            out.append(byte | 0x80)
        else:
            out.append(byte)
            return bytes(out)


def vec(items):
    return leb(len(items)) + b"".join(items)


def name(text):
    raw = text.encode()
    return leb(len(raw)) + raw


def section(ident, body):
    return bytes([ident]) + leb(len(body)) + body


def functype(params, results):
    return b"\x60" + vec([bytes([p]) for p in params]) + vec([bytes([r]) for r in results])


def module(*sections):
    return b"\x00asm\x01\x00\x00\x00" + b"".join(sections)


def sample():
    """two types; imports of a function, a memory and a global; one function
    of the module's own; exports of it, of the imported function and of the
    memory"""
    types = section(1, vec([functype([I32, I32], [I32]), functype([], [F64])]))
    imports = section(
        2,
        vec(
            [
                name("env") + name("halo_platform_tick") + b"\x00" + leb(0),
                name("env") + name("memory") + b"\x02\x00" + leb(1),
                name("env") + name("__stack_pointer") + b"\x03" + bytes([I32]) + b"\x01",
            ]
        ),
    )
    functions = section(3, vec([leb(1)]))
    exports = section(
        7,
        vec(
            [
                name("halo_game_main") + b"\x00" + leb(1),
                name("tick") + b"\x00" + leb(0),
                name("memory") + b"\x02" + leb(0),
            ]
        ),
    )
    return module(types, imports, functions, exports)


def test_a_module_s_imports_and_exports_are_read_with_their_types():
    parsed = read_module(sample())
    assert parsed.imports == [
        "env.halo_platform_tick: func (i32, i32) -> (i32)",
        "env.memory: memory",
        "env.__stack_pointer: global mut i32",
    ]
    assert parsed.exports == [
        "halo_game_main: func () -> (f64)",
        "tick: func (i32, i32) -> (i32)",
        "memory: memory",
    ]


def test_sections_the_reader_does_not_need_are_skipped():
    custom = section(0, name("producers") + b"anything")
    parsed = read_module(module(custom) + sample()[8:])
    assert parsed.exports[0] == "halo_game_main: func () -> (f64)"


@pytest.mark.parametrize(
    "data",
    [
        b"",
        b"\x00asm",
        b"\x7fELF\x01\x00\x00\x00",
        b"\x00asm\x02\x00\x00\x00",
        # a section longer than the module
        module(b"\x01\x10\x00"),
        # an import that ends inside its name
        module(section(2, leb(1) + leb(200) + b"env")),
        # a function import of a type the module does not have
        module(section(2, vec([name("env") + name("f") + b"\x00" + leb(5)]))),
        # an export of a function the module does not have
        module(section(7, vec([name("f") + b"\x00" + leb(9)]))),
        # an LEB128 longer than 32 bits
        module(section(1, b"\xff\xff\xff\xff\xff\x01")),
        # a count far beyond the bytes there are
        module(section(1, leb(0xFFFFFFF))),
    ],
)
def test_malformed_modules_are_refused(data):
    with pytest.raises(MalformedModule):
        read_module(data)


def test_every_truncation_of_a_module_is_refused_or_read():
    """no prefix of a valid module makes the reader fail other than by
    refusing it"""
    data = sample()
    for end in range(len(data)):
        try:
            read_module(data[:end])
        except MalformedModule:
            pass


@pytest.mark.skipif(not os.environ.get("HALO_WASM_CC"), reason="needs the wasm toolchain (nix develop)")
def test_game_wasm_imports_and_exports_are_the_reviewed_lists():
    output = engine_wasm.build(os.cpu_count() or 8)
    imports = (ROOT / engine_wasm.IMPORTS_LIST).read_text().splitlines()
    exports = (ROOT / engine_wasm.EXPORTS_LIST).read_text().splitlines()
    assert engine_wasm.module_imports(output) == imports
    assert engine_wasm.module_exports(output) == exports
    # the game's entry point and the heap the platform allocates from
    for export in engine_wasm.EXPORTS:
        assert any(line.startswith(f"{export}: func") for line in exports)
    # the game reaches the platform through "env" and wasi only
    assert {line.split(".", 1)[0] for line in imports} <= {"env", "wasi_snapshot_preview1"}
