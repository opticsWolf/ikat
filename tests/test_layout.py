"""Tests for auto-layout (M7.1): suggest/apply/report, CLI, MCP.

No TeX anywhere: measuring runs the Rust emitters only. Fences use
inline mode so diagrams measure TikZ; registry entries mirror the
showcase shape (key/caption/mode).
"""

import json

import pytest

from ikat import apply_layout, layout_report, suggest_layout
from ikat.cli import main as cli_main

needs_mcp = pytest.mark.skipif(
    __import__("importlib").util.find_spec("mcp.server.fastmcp") is None,
    reason="mcp extra absent",
)

TINY_DIAGRAM = "```mermaid {span=column pos=top}\ngraph TD\na[x]-->b[y]\n```\n"
TALL_DIAGRAM = "```mermaid\n" + "graph TD\n" + "".join(f"n{i}[x]-->n{i+1}[y]\n" for i in range(12)) + "```\n"
SMALL_TABLE = "%% table {span=column pos=top}\n| A | B |\n|--|--|\n| 1 | 2 |\n"
WIDE_TABLE = "| Token | Fills from | Required |\n|--|--|--|\n| `{{bibliography}}` | style plus staged `.bib` name and more text here | exactly once unless `bib_name` is empty |\n"


def _spec(*, diagrams=(), captions=(), plots=()):
    return {
        "diagrams": [{"key": k, "caption": c, "mode": "inline"} for k, c in diagrams],
        "table_captions": list(captions),
        "plots": list(plots),
    }


def test_short_diagram_suggests_here():
    md = f"# T\n\n## 1. A\n\n{TINY_DIAGRAM}"
    (s,) = suggest_layout(md, "", _spec(diagrams=[("d0", "Tiny.")]))
    assert s["kind"] == "diagram" and s["action"] == "change"
    assert (s["span"], s["pos"]) == ("column", "here")
    assert "cm" in s["reason"]


def test_tall_diagram_lead_stays_second_goes():
    md = f"# T\n\n## 1. A\n\n{TINY_DIAGRAM}\n{TALL_DIAGRAM}\n"
    spec = _spec(diagrams=[("d0", "Tiny."), ("d1", "Tall.")])
    tiny, tall = suggest_layout(md, "", spec)
    assert (tiny["span"], tiny["pos"]) == ("column", "here"), tiny
    assert tall["pos"] == "page", tall
    assert tall["est_h_cm"] > 13.0, tall
    assert tall["width"] is None  # pages keep natural size


def test_short_table_pins_force_and_wide_floats():
    md = f"# T\n\n## 1. A\n\n{SMALL_TABLE}\n{WIDE_TABLE}\n"
    a, b = suggest_layout(md, "", _spec(captions=["S.", "W."]))
    assert (a["span"], a["pos"]) == ("column", "force"), a
    assert (b["span"], b["pos"]) == ("wide", "top"), b


def test_plot_suggests_force_report_only():
    preset = json.dumps(
        {
            "kind": "bar",
            "title": "t",
            "ylabel": "y",
            "log_y": False,
            "groups": ["g"],
            "series": [{"name": "s", "values": [1.0], "mins": [1.0], "maxs": [1.0]}],
        }
    )
    md = "# T\n\n## 1. A\n\nBody.\n"
    (s,) = suggest_layout(md, "", _spec(plots=[("fig-p", preset)]))
    assert s["kind"] == "plot" and (s["span"], s["pos"]) == ("column", "force"), s
    new_md, n = apply_layout(md, [s])
    assert (new_md, n) == (md, 0)  # plots never rewrite md


def test_h2_resets_section_lead():
    md = (
        "# T\n\n## 1. A\n\n" + TINY_DIAGRAM +
        "\n## 2. B\n\n" + TINY_DIAGRAM
    )
    spec = _spec(diagrams=[("d0", "A."), ("d1", "B.")])
    first, second = suggest_layout(md, "", spec)
    assert (first["span"], first["pos"]) == ("column", "here"), first
    assert (second["span"], second["pos"]) == ("column", "here"), second
    assert "lead" in second["reason"], second["reason"]


def test_late_wide_table_names_the_fix():
    prose = "\n".join(f"Prose line {i} fills the column." for i in range(20))
    md = f"# T\n\n## 1. A\n\n{prose}\n\n{WIDE_TABLE}\n"
    spec = _spec(captions=["W."])
    (s,) = suggest_layout(md, "", spec)
    assert s["span"] == "wide" and s["pos"] == "top", s
    assert "move the fence" in s["reason"], s["reason"]


def test_section_head_wide_table_is_quiet():
    md = f"# T\n\n## 1. A\n\n{WIDE_TABLE}\n"
    (s,) = suggest_layout(md, "", _spec(captions=["W."]))
    assert s["span"] == "wide", s
    assert "move the fence" not in s["reason"], s["reason"]


def test_apply_rewrites_and_converges():
    md = f"# T\n\n## 1. A\n\n{TINY_DIAGRAM}\n{SMALL_TABLE}\n"
    spec = _spec(diagrams=[("d0", "Tiny.")], captions=["S."])
    first = suggest_layout(md, "", spec)
    assert all(s["action"] == "change" for s in first)
    new_md, n = apply_layout(md, first)
    assert n == 2
    assert "{span=column pos=here}" in new_md
    second = suggest_layout(new_md, "", spec)
    assert [s["action"] for s in second] == ["keep", "keep"], second


def test_apply_inserts_missing_table_directive():
    md = "# T\n\n## 1. A\n\n| A |\n|--|--|\n| 1 |\n"
    spec = _spec(captions=["S."])
    (s,) = suggest_layout(md, "", spec)
    assert s["action"] == "change"  # default top -> force
    new_md, n = apply_layout(md, (s,))
    assert n == 1 and "%% table {span=column pos=force}" in new_md


def test_strict_errors_carry_lines():
    bad = "# T\n\n## 1. A\n\n```mermaid\ngraph TD\na-->b\nbad@@stmt\n```\n"
    with pytest.raises(ValueError):
        suggest_layout(bad, "", _spec(diagrams=[("d0", "B.")]))

    fence = "# T\n\n## 1. A\n\n```mermaid\ngraph TD\na-->b\n```\n"
    with pytest.raises(ValueError, match="no registry entry"):
        suggest_layout(fence, "", _spec())


def test_cli_report_and_apply(tmp_path, capsys):
    md = tmp_path / "d.md"
    md.write_text(f"# T\n\n## 1. A\n\n{TINY_DIAGRAM}", encoding="utf-8")
    spec = tmp_path / "s.json"
    spec.write_text(json.dumps(_spec(diagrams=[("d0", "Tiny.")])), encoding="utf-8")
    assert cli_main(["layout", str(md), "--spec", str(spec)]) == 0
    out = capsys.readouterr().out
    assert "diagram #0" in out and "here" in out
    assert cli_main(["layout", str(md), "--spec", str(spec), "--apply"]) == 0
    assert "{span=column pos=here}" in md.read_text(encoding="utf-8")


@needs_mcp
def test_mcp_suggest_layout():
    from ikat import mcp_server

    md = f"# T\n\n## 1. A\n\n{TINY_DIAGRAM}"
    r = mcp_server.suggest_layout(md, "", json.dumps(_spec(diagrams=[("d0", "Tiny.")])))
    assert r["suggestions"][0]["pos"] == "here"
    assert "here" in r["report"]
