"""Tests for the unified surface: CLI commands and MCP tool registration.

CLI tests run main(argv) against tmp dirs (no TeX: --no-pdf/weave).
MCP tests call the tool functions directly + assert registration;
skipped when the mcp extra is absent.
"""

import io
import json

import pytest

from ikat import __version__
from ikat.cli import main as cli_main

needs_mcp = pytest.mark.skipif(
    __import__("importlib").util.find_spec("mcp.server.fastmcp") is None,
    reason="mcp extra absent",
)

MD = "# Hi\n\n## 1. Sec\n\nBody text.\n"


def test_weave_stdout(tmp_path, capsys):
    md = tmp_path / "d.md"
    md.write_text(MD, encoding="utf-8")
    assert cli_main(["weave", str(md)]) == 0
    out = capsys.readouterr().out
    assert "\\documentclass[conference]{IEEEtran}" in out
    assert "\\section{Sec}" in out


def test_build_no_pdf(tmp_path, capsys):
    md = tmp_path / "d.md"
    md.write_text(MD, encoding="utf-8")
    out = tmp_path / "out"
    assert cli_main(["build", str(md), "--outdir", str(out), "--no-pdf"]) == 0
    tex = (out / "d.tex").read_text(encoding="utf-8")
    assert "\\maketitle" in tex
    assert "tex:" in capsys.readouterr().out


def test_build_with_toml_and_spec(tmp_path):
    (tmp_path / "ikat.toml").write_text('[template]\npreamble_append = ["% hi"]\n', encoding="utf-8")
    md = tmp_path / "d.md"
    md.write_text(MD, encoding="utf-8")
    spec = tmp_path / "s.json"
    spec.write_text(json.dumps({"bib_style": "plain", "bogus_key": 1}), encoding="utf-8")
    out = tmp_path / "out"
    assert cli_main(["build", str(md), "--toml", str(tmp_path / "ikat.toml"),
                     "--outdir", str(out), "--no-pdf"]) == 0
    assert cli_main(["build", str(md), "--outdir", str(out), "--no-pdf",
                     "--spec", str(spec)]) == 0
    assert "\\bibliographystyle{plain}" in (out / "d.tex").read_text(encoding="utf-8")


def test_check_reports_and_fails(tmp_path, capsys, monkeypatch):
    from ikat import texenv
    tex = tmp_path / "d.tex"
    tex.write_text("\\documentclass{article}\\usepackage{pgfplots}\n", encoding="utf-8")
    monkeypatch.setattr(texenv.shutil, "which", lambda _: None)
    assert cli_main(["check", str(tex)]) == 1
    rep = json.loads(capsys.readouterr().out)
    assert rep["unprobed"] == ["pgfplots.sty", "article.cls"]


def test_templates_commands(tmp_path, capsys):
    assert cli_main(["templates"]) == 0
    names = capsys.readouterr().out.split()
    assert len(names) == 9 and "arxiv" in names
    assert cli_main(["template", "arxiv", "--show-preset"]) == 0
    out = capsys.readouterr().out
    assert "\\documentclass[11pt]{article}" in out
    assert "bib_style=IEEEtran" in out
    assert cli_main(["template", "nope"]) == 1


def test_flowchart_stdin(capsys, monkeypatch):
    import sys
    monkeypatch.setattr(sys, "stdin", __import__("io").StringIO("graph TD\na[x]-->b[y]\n"))
    assert cli_main(["flowchart", "-"]) == 0
    assert "\\begin{tikzpicture}" in capsys.readouterr().out


def test_skeleton_commands(capsys):
    assert cli_main(["skeletons"]) == 0
    assert "plain" in capsys.readouterr().out
    assert cli_main(["skeleton", "skeleton-plain"]) == 0
    assert "{{body}}" in capsys.readouterr().out


def test_plot_commands(tmp_path, capsys):
    bar = tmp_path / "bar.json"
    bar.write_text('{"title": "t", "ylabel": "y", "group_labels": ["a"],'
                     ' "series_names": ["s"], "values": [[2.0]], "mins": [[2.0]],'
                     ' "maxs": [[2.0]]}', encoding="utf-8")
    assert cli_main(["barchart", str(bar), "--legend", "below"]) == 0
    out = capsys.readouterr().out
    assert "anchor=north,legend columns=-1" in out
    line = tmp_path / "line.json"
    line.write_text('{"title": "t", "xlabel": "x", "ylabel": "y", "xs": [1.0],'
                      ' "names": ["s"], "yss": [[2.0]], "errs": [[0.0]]}', encoding="utf-8")
    assert cli_main(["lineplot", str(line)]) == 0
    assert "\\begin{tikzpicture}" in capsys.readouterr().out


def test_preset_flags(tmp_path, capsys, monkeypatch):
    loc = "examples/ikat-paper/fig-loc.json"
    assert cli_main(["barchart", "--preset", loc]) == 0
    assert "\\addlegendentry{shipped code}" in capsys.readouterr().out
    # stdin preset
    src = open(loc, encoding="utf-8").read()
    monkeypatch.setattr("sys.stdin", io.StringIO(src))
    assert cli_main(["lineplot", "--preset", "-", "--legend", "below"]) == 0
    assert "anchor=north,legend columns=-1" in capsys.readouterr().out
    # FILE + --preset conflict names both
    assert cli_main(["barchart", str(tmp_path / "x.json"), "--preset", loc]) == 1
    assert "--preset" in capsys.readouterr().err
    # neither FILE nor --preset
    assert cli_main(["barchart"]) == 1


@needs_mcp
def test_mcp_preset_param():
    from ikat import mcp_server

    loc = open("examples/ikat-paper/fig-tests.json", encoding="utf-8").read()
    r = mcp_server.lineplot_to_tikz("t", "x", "y", [1.0], ["s"], [[1.0]], [[0.0]],
                                     preset_json=loc)
    assert "\\addlegendentry{Rust tests}" in r


def test_version(capsys):
    assert cli_main(["version"]) == 0
    assert capsys.readouterr().out.strip() == __version__


@needs_mcp
def test_mcp_tools_registered():
    from ikat import mcp_server
    import asyncio

    names = sorted(t.name for t in asyncio.run(mcp_server.mcp.list_tools()))
    assert names == ["barchart_to_tikz", "check_tex_packages", "ensure_tex_packages",
                     "float_packages", "flowchart_to_tikz", "get_skeleton", "get_template", "lineplot_to_tikz",
                     "list_skeletons", "list_templates", "standalone_figure", "version", "weave_document"]


@needs_mcp
def test_mcp_weave_and_template():
    from ikat import mcp_server

    r = mcp_server.weave_document(MD, "", '{"bib_style": "plain"}')
    assert r["tex"].count("\\documentclass") == 1
    assert "\\bibliographystyle{plain}" in r["tex"]
    t = mcp_server.get_template("aps")
    assert t["bib_style"] == "apsrev4-2"
    assert "\\documentclass" in t["head"]
    assert mcp_server.version() == __version__
