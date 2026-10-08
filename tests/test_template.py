"""Tests for user-supplied preamble templates (levels 1+2).

`preamble_file` replaces the generated head; `preamble_append`
adds lines before `\\begin{document}`; validation rejects heads
that drop packages the body needs.
"""

import pytest

from ikat import BuildSpec, DiagramEntry, build_from_paths
from ikat import list_templates, template_path, template_preset

MD = "# T\n\n## 1. I\n\nHi.\n"
MD_TIKZ = MD + "\n```mermaid\ngraph TD\na[x]-->b[y]\n```\n"

HEAD = """\\documentclass{article}
\\usepackage[utf8]{inputenc}
\\usepackage{tikz}
\\usetikzlibrary{shapes.geometric,arrows.meta,positioning}
\\usepackage{graphicx}
\\title{T}
\\author{A}
"""

INLINE = [DiagramEntry(key="fig-x", caption="C.", mode="inline")]


def _spec(**kw):
    kw.setdefault("diagrams", [])
    kw.setdefault("plots", [])
    kw.setdefault("table_captions", [])
    return BuildSpec(**kw)


def test_override_and_append(tmp_path):
    (tmp_path / "head.tex").write_text(HEAD, encoding="utf-8")
    (tmp_path / "doc.md").write_text(MD_TIKZ, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text(
        '[template]\npreamble_file = "head.tex"\n'
        'preamble_append = ["\\\\usepackage{natbib}"]\n',
        encoding="utf-8",
    )
    r = build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml",
                         _spec(diagrams=INLINE))
    assert "\\documentclass{article}" in r.tex
    assert "documentclass[conference]{IEEEtran}" not in r.tex
    assert "\\bibliographystyle{IEEEtran}" in r.tex  # tail stays generated
    head = r.tex.split("\\begin{document}")[0]
    assert "\\usepackage{natbib}" in head


def test_override_dropping_tikz_fails(tmp_path):
    (tmp_path / "head.tex").write_text(
        "\\documentclass{article}\n\\title{T}\n", encoding="utf-8")
    (tmp_path / "doc.md").write_text(MD_TIKZ, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text(
        '[template]\npreamble_file = "head.tex"\n', encoding="utf-8")
    with pytest.raises(ValueError, match="tikz"):
        build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml",
                         _spec(diagrams=INLINE))


def test_no_template_keeps_generated_head(tmp_path):
    (tmp_path / "doc.md").write_text(MD, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    r = build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", _spec())
    assert "\\documentclass[conference]{IEEEtran}" in r.tex


def test_library_heads_are_valid_templates():
    names = list_templates()
    assert len(names) == 9
    for name in names:
        head = template_path(name).read_text(encoding="utf-8")
        assert "\\documentclass" in head, name
        preset = template_preset(name)
        assert preset["bib_style"]


def test_presets_cover_library():
    from ikat.document import TEMPLATE_PRESETS
    assert sorted(TEMPLATE_PRESETS) == sorted(list_templates())
    flagged = {n for n, p in TEMPLATE_PRESETS.items() if "abstract_before_maketitle" in p["toml"]}
    assert flagged == {"acm-sigconf", "elsevier", "aps"}


def test_all_heads_weave_with_guards():
    # D.2: every shipped head weaves a section-bearing doc with
    # guards on (needspace auto-added, no titlesec trip).
    from ikat import build_document

    md = "# T\n\n## 1. Intro\n\nBody.\n"
    for name in list_templates():
        if name.startswith("skeleton-"):
            continue
        head = template_path(name).read_text(encoding="utf-8")
        r = build_document(md, "", _spec(preamble_override=head))
        assert "\\needspace{3\\baselineskip}" in r.tex, name
        # rsplit: some heads MENTION \begin{document} in comments.
        assert "\\usepackage{needspace}" in r.tex.rsplit("\\begin{document}", 1)[0], name


def test_cli_check_surfaces_titlesec(tmp_path, capsys):
    from ikat.cli import main as cli_main

    bad = tmp_path / "bad.tex"
    bad.write_text("\\documentclass[conference]{IEEEtran}\n"
                    "\\usepackage[nobottomtitles]{titlesec}\n"
                    "\\begin{document}\nHi.\n\\end{document}\n", encoding="utf-8")
    assert cli_main(["check", str(bad)]) == 1
    assert "titlesec" in capsys.readouterr().err
    good = tmp_path / "good.tex"
    good.write_text("\\documentclass{article}\n\\usepackage{titlesec}\n"
                    "\\begin{document}\nHi.\n\\end{document}\n", encoding="utf-8")
    # article+titlesec passes the titlesec gate (TeX probe may
    # still fail without a TeX install — only the gate matters).
    import shutil
    if shutil.which("kpsewhich") is None:
        pytest.skip("no TeX probe available")
    assert cli_main(["check", str(good)]) == 0


needs_tex = pytest.mark.skipif(
    __import__("shutil").which("pdflatex") is None, reason="no pdflatex")


@needs_tex
def test_showcase_builds_unchanged(capsys):
    # Guards on: the full showcase (weave + compile) still builds.
    import runpy
    from pathlib import Path

    here = Path(__file__).parent.parent / "examples" / "ikat-paper"
    runpy.run_path(str(here / "build.py"), run_name="__main__")
    out = capsys.readouterr().out
    assert "diagrams=4 tables=4" in out


def test_spec_override_beats_toml(tmp_path):
    (tmp_path / "head.tex").write_text(HEAD, encoding="utf-8")
    (tmp_path / "doc.md").write_text(MD, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text(
        '[template]\npreamble_file = "head.tex"\n', encoding="utf-8")
    spec = _spec(preamble_override="\\documentclass{report}\n\\title{T}\n")
    r = build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", spec)
    assert "\\documentclass{report}" in r.tex
