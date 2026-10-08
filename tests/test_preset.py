"""Preset loader: benchmark JSON in, tikzpicture out (Phase A.1)."""

import json

import pytest

from ikat import barchart_to_tikz, lineplot_to_tikz
from ikat.preset import load_preset

LOC = "examples/ikat-paper/fig-loc.json"
TESTS = "examples/ikat-paper/fig-tests.json"


def test_file_and_string_agree():
    from pathlib import Path

    raw = Path(LOC).read_text(encoding="utf-8")
    assert load_preset(LOC) == load_preset(raw)


def test_error_names_origin():
    # raw string: origin is <preset>, kind set is named
    with pytest.raises(ValueError, match=r"preset <preset>.*bar\|line"):
        load_preset('{"kind": "pie", "title": "t", "ylabel": "y"}')
    # unknown kind through a real file path names the file
    import tempfile, pathlib

    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as f:
        json.dump({"kind": "pie", "title": "t", "ylabel": "y"}, f)
        name = f.name
    try:
        with pytest.raises(ValueError, match="pie") as exc:
            load_preset(name)
        assert name in str(exc.value)
    finally:
        pathlib.Path(name).unlink()


def test_loc_parity_with_hand_assembled():
    data = json.loads(open(LOC, encoding="utf-8").read())
    hand = barchart_to_tikz(
        data["title"], data["ylabel"], data["log_y"],
        data["group_labels"], data["series_names"],
        data["values"], data["mins"], data["maxs"], None,
    )
    assert load_preset(LOC) == hand


def test_regen_fixtures_parse_and_match_snaps():
    import pathlib

    for name in ("fig-fork-flat", "fig-t6-asymmetry", "fig-trunk-branch", "fig-chain-depth"):
        tikz = load_preset(f"examples/regen/{name}.json")
        assert "\\begin{tikzpicture}" in tikz
        snap = pathlib.Path(f"examples/regen/snap/{name}.tex")
        assert snap.exists(), f"missing snapshot for {name}"
        assert snap.read_text(encoding="utf-8") == tikz, f"snapshot drift: {name}"


def test_growth_parity_with_hand_assembled():
    data = json.loads(open(TESTS, encoding="utf-8").read())
    hand = lineplot_to_tikz(
        data["title"], data["xlabel"], data["ylabel"],
        [float(x) for x in data["xs"]],
        data["series_names"],
        [[float(v) for v in row] for row in data["yss"]],
        [[float(v) for v in row] for row in data["errs"]],
    )
    assert load_preset(TESTS) == hand
