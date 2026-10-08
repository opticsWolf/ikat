"""Tests for standalone figures: wrap, CLI, MCP (no TeX except e2e)."""

import shutil

import pytest

from ikat import compile_standalone, fence_to_tikz, plots_needed, tikz_needed, wrap_standalone
from ikat.cli import main as cli_main

needs_tex = pytest.mark.skipif(shutil.which("pdflatex") is None, reason="no pdflatex")
needs_mcp = pytest.mark.skipif(
    __import__("importlib").util.find_spec("mcp.server.fastmcp") is None,
    reason="mcp extra absent",
)

FLOW = "graph TD\na[x]-->b[y]\n"


def test_predicates_scan():
    assert tikz_needed("\\begin{tikzpicture}") and not tikz_needed("plain")
    assert plots_needed("\\begin{axis}") and not plots_needed("\\begin{tikzpicture}")


def test_wrap_minimal_preamble():
    tikz = fence_to_tikz("mermaid", FLOW)
    tex = wrap_standalone(tikz)
    assert "\\documentclass[tikz,border=5pt]{standalone}" in tex
    assert "\\usetikzlibrary{shapes.geometric,arrows.meta,positioning}" in tex
    assert "pgfplots" not in tex  # TikZ-only body: no pgfplots line


def test_wrap_pgfplots_body():
    from ikat import preset_to_tikz

    tikz = preset_to_tikz(open("examples/regen/fig-chain-depth.json", encoding="utf-8").read())
    tex = wrap_standalone(tikz)
    assert "\\usepackage{pgfplots}" in tex
    with pytest.raises(ValueError, match="tikzpicture"):
        wrap_standalone("no picture here")
    with pytest.raises(ValueError, match="mermaid\\|chart"):
        fence_to_tikz("python", "x = 1")


def test_cli_multi_fence_names_count(tmp_path, capsys):
    md = tmp_path / "two.md"
    md.write_text("```mermaid\ngraph TD\na[x]\n```\n\n```mermaid\ngraph TD\nb[y]\n```\n", encoding="utf-8")
    assert cli_main(["standalone", str(md), "--out", str(tmp_path / "f.pdf")]) == 1
    assert "2 mermaid/chart fences" in capsys.readouterr().err


@needs_tex
def test_cli_end_to_end_pdf(tmp_path, capsys):
    md = tmp_path / "one.md"
    md.write_text("```mermaid\n" + FLOW + "```\n", encoding="utf-8")
    out = tmp_path / "fig.pdf"
    assert cli_main(["standalone", str(md), "--out", str(out)]) == 0
    assert out.stat().st_size > 1000
    assert "pdf:" in capsys.readouterr().out


def test_engine_passthrough(monkeypatch, tmp_path):
    import ikat.compile as comp
    import ikat.standalone as st

    seen = {}

    def fake_compile(workdir, main_tex, engine="pdflatex"):
        seen["engine"] = engine
        return workdir / "fig.pdf"

    monkeypatch.setattr(comp, "compile_pdf", fake_compile)
    st.compile_standalone(fence_to_tikz("mermaid", FLOW), tmp_path, engine="tectonic")
    assert seen["engine"] == "tectonic"


@needs_mcp
def test_mcp_returns_source_not_bytes():
    from ikat import mcp_server

    r = mcp_server.standalone_figure("```mermaid\n" + FLOW + "```\n")
    assert r["lang"] == "mermaid"
    assert r["tex"].startswith("\\documentclass[tikz")
    assert isinstance(r["tex"], str)  # source, not PDF bytes
