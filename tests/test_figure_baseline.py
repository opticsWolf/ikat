"""Figure auto-adapt baseline: the ikat paper must not get worse.

The showcase paper is hand-placed (explicit span/pos on its floats)
and renders clean (7 pages, 0 overfulls), so weave-time
auto-adaptation must be a strict no-op there: zero notes, no height
guards, identical floats. Any note firing on this manuscript fails
the test and forces deliberate review — that is the "not worse"
gate. Synthetic problem figures (the report's measurements) pin the
fixes: tall picture caps, narrow-to-column, explicit-attrs silence,
kill switch, cluster expansion.
"""

import json
from pathlib import Path

from ikat import BuildSpec, DiagramEntry, build_from_paths
from ikat.cli import main as cli_main

ROOT = Path(__file__).resolve().parent.parent
PAPER_MD = ROOT / "examples" / "ikat-paper" / "ikat-paper.md"
PAPER_TOML = ROOT / "examples" / "ikat-paper" / "ikat.toml"


def _paper_spec():
    return BuildSpec(
        diagrams=[
            DiagramEntry(key="fig-pipeline", caption="Pipeline.", mode="inline"),
            DiagramEntry(key="fig-sequence", caption="Sequence.", mode="inline"),
            DiagramEntry(key="fig-states", caption="States.", mode="inline"),
            DiagramEntry(key="fig-floats", caption="Floats.", mode="inline"),
        ],
        plots=[("fig-loc", "Loc."), ("fig-tests", "Tests.")],
        table_captions=["Per-element float attributes.",
                        "Test suites and what they guard.",
                        "Skeleton token contract.",
                        "Compile engines compared."],
        table_specs={"2": "XXl"},
    )


def test_showcase_adapts_nothing():
    r = build_from_paths(PAPER_MD, PAPER_TOML, _paper_spec())
    assert r.n_diagrams == 4
    assert r.notes == []
    assert "height=" not in r.body


def _inline_spec(key="x"):
    return BuildSpec(diagrams=[DiagramEntry(key=key, caption="C.", mode="inline")])


def _write(tmp_path, md, toml=""):
    (tmp_path / "doc.md").write_text(md, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text(toml, encoding="utf-8")
    return tmp_path / "doc.md", tmp_path / "ikat.toml"


def test_narrow_diagram_goes_column(tmp_path):
    md, toml = _write(tmp_path, "# T\n\n```mermaid {inline}\ngraph TD\nA[x]-->B[y]\n```\n")
    r = build_from_paths(md, toml, _inline_spec())
    assert "\\begin{figure}" in r.body
    assert "figure*" not in r.body
    assert len(r.notes) == 1
    assert r.notes[0].startswith("mermaid fence #0")


def test_tall_picture_gets_height_cap(tmp_path):
    md, toml = _write(tmp_path, "# T\n\n```mermaid\ngraph TD\nA[x]-->B[y]\n```\n")
    spec = BuildSpec(diagrams=[DiagramEntry(key="ext", caption="C.")])
    r = build_from_paths(md, toml, spec)
    assert "height=0.85\\textheight,keepaspectratio" in r.body
    assert len(r.notes) == 1
    assert "fig:ext" in r.notes[0]


def test_explicit_attrs_stay_silent(tmp_path):
    md, toml = _write(tmp_path, "# T\n\n```mermaid {inline span=wide pos=top}\ngraph TD\nA[x]-->B[y]\n```\n")
    r = build_from_paths(md, toml, _inline_spec())
    assert "figure*" in r.body
    assert r.notes == []


def test_kill_switch_disables_auto(tmp_path):
    md, toml = _write(tmp_path, "# T\n\n```mermaid\ngraph TD\nA[x]-->B[y]\n```\n",
                      "[layout]\nauto_adapt = false\n")
    spec = BuildSpec(diagrams=[DiagramEntry(key="ext", caption="C.")])
    r = build_from_paths(md, toml, spec)
    assert "height=" not in r.body
    assert r.notes == []


def test_cluster_expansion_reports(tmp_path):
    md, toml = _write(
        tmp_path,
        "# T\n\n```mermaid {inline}\ngraph TD\nsubgraph s\nA[pop: silica-rich stacks for scale]\nend\nA-->B[ok]\n```\n")
    r = build_from_paths(md, toml, _inline_spec())
    assert any("subgraph `s` widened" in n for n in r.notes)


def test_cli_reports_adaptations_on_stderr(tmp_path, capsys):
    md, _ = _write(tmp_path, "# T\n\n```mermaid {inline}\ngraph TD\nA[x]-->B[y]\n```\n")
    spec = {"diagrams": [{"key": "x", "caption": "C.", "mode": "inline"}]}
    (tmp_path / "spec.json").write_text(json.dumps(spec), encoding="utf-8")
    assert cli_main(["weave", str(md), "--spec", str(tmp_path / "spec.json")]) == 0
    assert "mermaid fence #0" in capsys.readouterr().err
