"""Tests for the texliveonfly-style ensure-packages helper.

Pure scan/map tests run against the Rust core with no TeX needed;
subprocess behavior is faked. One live test probes the real
environment, skipped when no TeX is installed.
"""

import shutil

import pytest

import ikat
from ikat import (
    check_tex_env,
    document_class,
    ensure_tex_packages,
    package_needs,
    tex_package_needs,
    used_packages,
)
from ikat import texenv

PREAMBLE = """% \\usepackage{commented-out}
\\documentclass[10pt,conference]{IEEEtran}
\\usepackage[utf8]{inputenc}
\\usepackage{lmodern,textcomp}
\\usepackage{amsmath,amssymb}
\\usepackage{tabularx}
\\usepackage{graphicx}
\\usepackage{tikz}
\\usetikzlibrary{shapes.geometric,arrows.meta}
\\usepackage{pgfplots}
\\pgfplotsset{compat=1.18}
\\usepackage[hidelinks]{hyperref}
"""


def test_scan_lists_packages_not_class():
    pkgs = used_packages(PREAMBLE)
    assert pkgs == ["inputenc", "lmodern", "textcomp", "amsmath", "amssymb",
                    "tabularx", "graphicx", "tikz", "pgfplots", "hyperref"]
    assert document_class(PREAMBLE) == "IEEEtran"


def test_needs_maps_to_probe_files_and_tlmgr():
    req = tex_package_needs(PREAMBLE)
    by_file = dict(req)
    assert by_file["tikz.sty"] == "pgf"
    assert by_file["pgfplots.sty"] == "pgfplots"
    assert by_file["tabularx.sty"] == "tools"
    assert by_file["IEEEtran.cls"] == "IEEEtran"


def test_package_needs_falls_back_to_name_convention():
    assert package_needs(["obscure"], None) == [("obscure.sty", "obscure")]


def test_check_reports_missing(monkeypatch):
    def fake_run(cmd, **kw):
        assert cmd[0] == "kpsewhich"

        class P:
            stdout = "/tex/latex/pgf/tikz.sty\n"

        return P()

    monkeypatch.setattr(texenv.shutil, "which", lambda _: "/bin/kpsewhich")
    monkeypatch.setattr(texenv.subprocess, "run", fake_run)
    rep = check_tex_env("\\usepackage{tikz,pgfplots}\n")
    assert rep["missing"] == ["pgfplots.sty"]
    assert not rep["ok"]


def test_check_unprobed_without_kpsewhich(monkeypatch):
    monkeypatch.setattr(texenv.shutil, "which", lambda _: None)
    rep = check_tex_env("\\usepackage{tikz}\n")
    assert rep["unprobed"] == ["tikz.sty"]
    assert not rep["ok"]


def test_ensure_installs_then_reprobes(monkeypatch):
    state = {"installed": False}

    def fake_run(cmd, **kw):
        class P:
            returncode = 0
            stdout = ""
            stderr = ""

        if cmd[0] == "kpsewhich":
            P.stdout = "/tex/latex/pgfplots/pgfplots.sty\n" if state["installed"] else ""
            return P()
        assert cmd[:2] == ["tlmgr", "install"]
        assert cmd[2:] == ["pgfplots"]
        state["installed"] = True
        return P()

    monkeypatch.setattr(texenv.shutil, "which", lambda _: "/bin/x")
    monkeypatch.setattr(texenv.subprocess, "run", fake_run)
    rep = ensure_tex_packages("\\usepackage{pgfplots}\n")
    assert rep["ok"]
    assert rep["installed_now"] == ["pgfplots.sty"]


def test_ensure_no_tlmgr_gives_hint(monkeypatch):
    def which(name):
        return "/bin/kpsewhich" if name == "kpsewhich" else None

    monkeypatch.setattr(texenv.shutil, "which", which)
    monkeypatch.setattr(
        texenv.subprocess, "run",
        lambda cmd, **kw: type("P", (), {"stdout": ""})(),
    )
    rep = ensure_tex_packages("\\usepackage{pgfplots}\n")
    assert not rep["ok"]
    assert "tlmgr not on PATH" in rep["hint"]


@pytest.mark.skipif(shutil.which("kpsewhich") is None, reason="no TeX installed")
def test_live_probe_sees_base_files():
    assert texenv.probe(["article.cls", "graphicx.sty"]) == {
        "article.cls": True, "graphicx.sty": True}


def test_compile_pdf_engine_passthrough(monkeypatch, tmp_path):
    # G.2: `engine=` is honored — tectonic routes to the tectonic
    # path, anything else names the options. No binary needed.
    from ikat import compile as compile_mod
    from ikat.compile import CompileError, compile_pdf

    seen = {}
    monkeypatch.setattr(
        compile_mod, "_compile_tectonic",
        lambda workdir, main: seen.update(workdir=workdir, main=main) or tmp_path / "x.pdf",
    )
    compile_pdf(tmp_path, "doc.tex", engine="tectonic")
    assert (seen["workdir"], seen["main"]) == (tmp_path, "doc.tex")
    with pytest.raises(CompileError, match="pdflatex\|tectonic"):
        compile_pdf(tmp_path, "doc.tex", engine="luatex")
