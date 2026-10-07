"""End-to-end tests for the thin layer (run against maturin develop)."""

import ikat
from ikat import (
    barchart_to_tikz,
    config_spans,
    figure_env,
    flowchart_to_tikz,
    resolve_span,
    weave_fragment,
)

READ_Q = """graph TD
%% caption: The three read operations answer different questions by construction.
%% label: fig:read-questions
q{"Which question?"}
q -->|"true at v"| av["as_of_valid(v)<br/>L's current state"]
av -->|compose| both["both instants"]
"""


def test_mermaid_end_to_end():
    tikz = flowchart_to_tikz(READ_Q)
    assert "\\begin{tikzpicture}" in tikz
    assert tikz.count("\\node") == 3
    assert "as\\_of\\_valid(v)" in tikz
    assert "[dia]" in tikz


def test_barchart_end_to_end():
    tikz = barchart_to_tikz(
        "t",
        "latency (ms)",
        True,
        ["R1", "Q3"],
        ["trunk", "branch"],
        [[7.4, 0.47], [28.0, 66.0]],
        [[6.9, 0.46], [25.0, 60.0]],
        [[8.1, 0.48], [31.0, 72.0]],
        (2.8, 3.2, 66.0, "zero-write fork"),
    )
    assert "ymode=log" in tikz
    assert "\\addlegendentry{branch}" in tikz
    assert "\\draw[dashed]" in tikz


def test_config_defaults_and_override():
    spans = config_spans("")
    assert spans["span_diagram"] == "figure*"
    assert spans["span_plot"] == "figure"
    assert resolve_span("diagram", None, spans) == "wide"
    assert resolve_span("plot", "wide", spans) == "wide"
    assert resolve_span("plot", None, spans) == "column"


def test_figure_env_widths():
    wide = figure_env("figs/a.pdf", "C", "fig:x", "wide")
    assert "\\begin{figure*}" in wide and "\\textwidth" in wide
    col = figure_env("figs/a.pdf", "C", "fig:a", "column")
    assert "\\begin{figure}" in col and "\\includegraphics[width=\\columnwidth]{figs/a.pdf}" in col
    tikz = figure_env(flowchart_to_tikz("graph TD\na[x]-->b[y]"), "C", "fig:t", "wide")
    assert "tikzpicture" in tikz and "includegraphics" not in tikz


def test_weave_fragment():
    md = "text\n\n```mermaid {span=column}\n" + READ_Q + "```\n"
    (el,) = weave_fragment(md)
    assert el.kind == "diagram"
    assert el.span == "column"  # local override beats wide default
    assert el.label == "fig:read-questions"
    assert el.body_tex.startswith("\\begin{tikzpicture}")
    (el2,) = weave_fragment("```mermaid\n" + READ_Q + "```\n")
    assert el2.span == "wide"  # configured kind default


def test_empty_bib_name_omits_bibliography():
    from ikat import BuildSpec, build_document

    r = build_document("# T\n\n## 1. I\n\nHi.\n", "", BuildSpec(bib_name=""))
    assert "bibliography" not in r.tex
    r = build_document("# T\n\n## 1. I\n\nHi.\n", "", BuildSpec())
    assert r"\bibliography{refs-paper}" in r.tex


def test_tex_for_tectonic_drops_inputenc():
    from ikat.compile import tex_for_tectonic

    tex = ('\\documentclass{article}' + '\n' + '\\usepackage[utf8]{inputenc}' + '\n' + '\\usepackage{tikz}' + '\n')
    out = tex_for_tectonic(tex)
    assert "inputenc" not in out
    assert r"\usepackage{tikz}" in out


def test_tectonic_engine_errors_helpfully_without_backend(monkeypatch, tmp_path):
    from ikat.compile import CompileError, compile_pdf

    monkeypatch.delenv("TECTONIC_EXE", raising=False)
    monkeypatch.setattr("shutil.which", lambda *a, **k: None)
    (tmp_path / "t.tex").write_text("\\documentclass{article}\n", encoding="utf-8")
    try:
        compile_pdf(tmp_path, "t.tex", engine="tectonic")
    except CompileError as e:
        assert "tectonic" in str(e).lower()
    else:  # pragma: no cover - embedded binding present (feature build)
        pass


def test_legend_keyword_positions():
    from ikat import lineplot_to_tikz

    base = dict(title="t", xlabel="x", ylabel="y", xs=[1.0, 2.0],
                names=["s"], yss=[[2.0, 3.0]], errs=[[0.1, 0.1]])
    # Default is auto: rising stub leaves top-left free.
    assert "anchor=north west" in lineplot_to_tikz(**base)
    assert "anchor=north,legend columns=-1" in lineplot_to_tikz(**base, legend="below")
    assert "anchor=north east" in lineplot_to_tikz(**base, legend="top-right")
    assert "outer north east" in lineplot_to_tikz(**base, legend="outside-right")
    try:
        lineplot_to_tikz(**base, legend="center")
    except ValueError as e:
        assert "below|" in str(e)
    else:  # pragma: no cover
        raise AssertionError("bad legend keyword accepted")
