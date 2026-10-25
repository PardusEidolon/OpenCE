"""tools/engine_reference_map.py: the map of the functions the engine ports, each with its trace or a reason, and the
harness finding a function whose return type is on the line before its name."""

import importlib.util
import sys
from pathlib import Path

TOOLS = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(TOOLS))
from harness import function  # noqa: E402

spec = importlib.util.spec_from_file_location("engine_reference_map", TOOLS / "engine_reference_map.py")
reference_map = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reference_map)

SOURCE = """static real_vector3d *
direction3d_from_table(
	real_vector3d *result,
	short index);

static real_vector3d *
direction3d_from_table(
	real_vector3d *result,
	short index)
{
	*result= table[index];
	return result;
}

short seed_random_range(unsigned long *seed, short lower, short upper)
{
	return lower;
}
"""


def test_a_return_type_on_the_line_before_is_found():
    found = function(SOURCE, "direction3d_from_table")
    assert found.startswith("static real_vector3d *\ndirection3d_from_table(")
    assert found.endswith("return result;\n}")


def test_the_definitions_are_found_and_not_the_prototypes():
    assert reference_map.DEFINITION.findall(SOURCE) == ["direction3d_from_table", "seed_random_range"]


def test_a_ported_function_needs_a_trace_or_a_reason(monkeypatch):
    monkeypatch.setattr(reference_map, "REASONS", {})
    files = {"traced_one": "source/math/a.c", "untraced_one": "source/math/a.c"}
    places = {"traced_one": "sim/src/a.rs:1", "untraced_one": "sim/src/a.rs:2"}
    traces = {"traced_one": "engine_a_reference"}
    assert reference_map.problems(files, places, traces) == ["untraced_one (sim/src/a.rs:2) has no trace and no reason"]


def test_a_reason_goes_stale(monkeypatch):
    monkeypatch.setattr(reference_map, "REASONS", {"now_traced": "why", "not_ported": "why"})
    files = {"now_traced": "source/math/a.c"}
    places = {"now_traced": "sim/src/a.rs:1"}
    traces = {"now_traced": "engine_a_reference"}
    assert reference_map.problems(files, places, traces) == [
        "not_ported is no longer ported: its reason is stale",
        "now_traced is traced by engine_a_reference: its reason is stale",
    ]


def test_the_table_names_each_trace_or_reason(monkeypatch):
    monkeypatch.setattr(reference_map, "REASONS", {"untraced_one": "it reads the world"})
    files = {"traced_one": "source/math/a.c", "untraced_one": "source/math/a.c"}
    places = {"traced_one": "sim/src/a.rs:1", "untraced_one": "sim/src/a.rs:2"}
    rows = reference_map.table(files, places, {"traced_one": "engine_a_reference"}).splitlines()
    assert rows[0] == reference_map.BEGIN and rows[-1] == reference_map.END
    assert any("`traced_one`" in row and "`tools/engine_a_reference.py`" in row for row in rows)
    assert any("`untraced_one`" in row and "it reads the world" in row for row in rows)


def test_the_map_is_written_between_its_markers():
    docs = f"before\n{reference_map.BEGIN}\nold\n{reference_map.END}\nafter\n"
    new = f"{reference_map.BEGIN}\nnew\n{reference_map.END}"
    assert reference_map.written(docs, new) == f"before\n{new}\nafter\n"


def script(path, stale):
    path.write_text(
        "import sys\nif '--check' in sys.argv and " + repr(stale) + ":\n    raise SystemExit('the trace is stale')\n"
    )
    return path


def test_a_stale_trace_fails_the_check(tmp_path, monkeypatch):
    # the negative control: a script whose --check fails is reported, one whose check passes is not
    current = script(tmp_path / "engine_current_reference.py", stale=False)
    stale = script(tmp_path / "engine_stale_reference.py", stale=True)
    monkeypatch.setattr(reference_map, "scripts", lambda: {current.stem: current, stale.stem: stale})
    assert reference_map.stale_traces() == ["engine_stale_reference: the trace is stale"]
