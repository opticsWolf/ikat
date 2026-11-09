"""Probe-issue gates end to end (issues #1-#4).

Unrepresentable unicode must fail at weave time with file:line:
(never inside a TeX log); bullet lists render as paragraphs with a
note instead of failing; emphasis spanning $...$ math must emit
textbf around verbatim math.
"""

import re

import pytest

from ikat import BuildSpec, DiagramEntry, barchart_to_tikz, build_document, build_from_paths, flowchart_to_tikz


def _spec(**kw):
    kw.setdefault("diagrams", [])
    kw.setdefault("plots", [])
    kw.setdefault("table_captions", [])
    return BuildSpec(**kw)


def test_greek_automaps_in_body(tmp_path):
    (tmp_path / "doc.md").write_text("# T\n\nR(λ,θ) spectra.\n", encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    r = build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", _spec())
    assert "$\\lambda$" in r.body
    assert "$\\theta$" in r.body


def test_greek_automaps_in_mermaid_label(tmp_path):
    md = ("# T\n\n```mermaid {inline}\n"
          "graph TD\n"
          "A[R(λ) spectra]-->B[merit]\n"
          "```\n")
    (tmp_path / "doc.md").write_text(md, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    spec = _spec(diagrams=[DiagramEntry(key="x", caption="C.", mode="inline")])
    r = build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", spec)
    assert "$\\lambda$" in r.body


def test_greek_flowchart_api_wraps():
    tikz = flowchart_to_tikz("graph TD\nA[R(λ) spectra]-->B[merit]")
    assert "$\\lambda$" in tikz


def test_unicode_precompiled_labels_do_not_reach_tex():
    # Precompiled fences embed an external PDF; labels never reach
    # TikZ, so the gate does not apply to the doc path.
    md = "# T\n\n```mermaid\ngraph TD\nA[R(λ) spectra]-->B[merit]\n```\n"
    spec = _spec(diagrams=[DiagramEntry(key="x", caption="C.")])
    r = build_document(md, "", spec)
    assert "fig:x" in r.body


def test_latin_accents_pass_through():
    # Golden-paper author names ride raw via inputenc.
    r = build_document("# T\n\nBöhlen and Šafařík report results.\n", "", _spec())
    assert "Böhlen" in r.body
    assert "Šafařík" in r.body


def test_greek_automaps_in_display_math():
    r = build_document("# T\n\n$$R(λ) = x^2$$\n", "", _spec())
    assert "\\lambda" in r.body
    assert "$$" not in r.body


def test_lists_render_as_paragraphs(tmp_path):
    (tmp_path / "doc.md").write_text("# T\n\n- item\n\n1. kept\n", encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    r = build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", _spec())
    assert "item" in r.body
    assert "1. kept" in r.body
    assert any("rendered as paragraph" in n for n in r.notes)


def test_bidir_and_fan():
    tikz = flowchart_to_tikz("graph TD\nA[x]<-->B[y]")
    assert "<->" in tikz
    tikz = flowchart_to_tikz("graph TD\nA[x] & B[y] --> C[z]")
    assert tikz.count("\\draw[") == 2
    tikz = flowchart_to_tikz("graph TD\nA[x] --> B[y] & C[z]")
    assert tikz.count("\\draw[") == 2


def test_placeholder_author_warns_on_stderr(tmp_path, capsys):
    from ikat.cli import main as cli_main

    md = tmp_path / "doc.md"
    md.write_text("# T\n\nBody.\n", encoding="utf-8")
    assert cli_main(["weave", str(md)]) == 0
    err = capsys.readouterr().err
    assert "placeholder author" in err
    assert "--author" in err


def test_explicit_author_stays_silent(tmp_path, capsys):
    from ikat.cli import main as cli_main

    md = tmp_path / "doc.md"
    md.write_text("# T\n\nBody.\n", encoding="utf-8")
    assert cli_main(["weave", str(md), "--author", "Ada Lovelace"]) == 0
    assert capsys.readouterr().err == ""


def test_combining_diacritics_weave_clean(tmp_path):
    md = ("# T\n\nThe n\u0302 profile over \u1e91.\n\n```mermaid {inline}\n"
          "graph TD\n"
          "A[n\u0302 spectra]-->B[merit]\n"
          "```\n")
    (tmp_path / "doc.md").write_text(md, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    spec = _spec(diagrams=[DiagramEntry(key="x", caption="C.", mode="inline")])
    r = build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", spec)
    assert "\\^{n}" in r.body
    assert "\\v{z}" in r.body


def test_combining_in_math_hints_hat(tmp_path):
    (tmp_path / "doc.md").write_text("# T\n\nEnergy $n\u0302$ here.\n", encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    with pytest.raises(ValueError, match=r"\\hat\{n\}") as ei:
        build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", _spec())
    assert "math mode" in str(ei.value)


def test_combining_plot_label_weaves():
    tikz = barchart_to_tikz(
        "\u1e91 rate",
        "latency (ms)",
        True,
        ["R1"],
        ["trunk"],
        [[7.4]],
        [[6.9]],
        [[8.1]],
        (2.8, 3.2, 66.0, "zero-write fork"),
    )
    assert "\\v{z} rate" in tikz


def test_bold_across_math_supported():
    r = build_document("# T\n\n**$P(z)$ depends only on $n(z)$**\n", "", _spec())
    assert "**" not in r.body
    assert r.body.startswith("\\textbf{$P(z)$")
    assert "$n(z)$" in r.body
