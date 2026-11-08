"""Probe-issue gates end to end (issues #1-#4).

Unicode outside the transliteration table + Latin pass-through must
fail at weave time with file:line: (never inside a TeX log); bullet
lists must fail instead of silently flattening; emphasis spanning
$...$ math must emit textbf around verbatim math.
"""

import re

import pytest

from ikat import BuildSpec, DiagramEntry, build_document, build_from_paths, flowchart_to_tikz


def _spec(**kw):
    kw.setdefault("diagrams", [])
    kw.setdefault("plots", [])
    kw.setdefault("table_captions", [])
    return BuildSpec(**kw)


def test_unicode_body_error_is_file_line(tmp_path):
    (tmp_path / "doc.md").write_text("# T\n\nR(λ,θ) spectra.\n", encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    with pytest.raises(ValueError, match=r"doc\.md:3:.*U\+03BB") as ei:
        build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", _spec())
    assert "lambda" in str(ei.value)


def test_unicode_mermaid_inline_error_is_file_line(tmp_path):
    md = ("# T\n\n```mermaid {inline}\n"
          "graph TD\n"
          "A[R(λ) spectra]-->B[merit]\n"
          "```\n")
    (tmp_path / "doc.md").write_text(md, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    spec = _spec(diagrams=[DiagramEntry(key="x", caption="C.", mode="inline")])
    with pytest.raises(ValueError, match=r"doc\.md:5:.*U\+03BB") as ei:
        build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", spec)
    assert "lambda" in str(ei.value)


def test_unicode_flowchart_api_keeps_rust_rendering():
    with pytest.raises(ValueError, match=re.escape(" --> line 2")) as ei:
        flowchart_to_tikz("graph TD\nA[R(λ) spectra]-->B[merit]")
    assert "U+03BB" in str(ei.value)


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


def test_unicode_display_math_errors():
    with pytest.raises(ValueError, match="U\\+03BB"):
        build_document("# T\n\n$$R(λ) = x^2$$\n", "", _spec())


def test_lists_fail_fast(tmp_path):
    (tmp_path / "doc.md").write_text("# T\n\n- item\n", encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    with pytest.raises(ValueError, match=r"doc\.md:3:.*lists are not in the subset") as ei:
        build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", _spec())
    assert "- item" in str(ei.value)


def test_bidir_and_fan():
    tikz = flowchart_to_tikz("graph TD\nA[x]<-->B[y]")
    assert "<->" in tikz
    tikz = flowchart_to_tikz("graph TD\nA[x] & B[y] --> C[z]")
    assert tikz.count("\\draw[") == 2
    tikz = flowchart_to_tikz("graph TD\nA[x] --> B[y] & C[z]")
    assert tikz.count("\\draw[") == 2


def test_bold_across_math_supported():
    r = build_document("# T\n\n**$P(z)$ depends only on $n(z)$**\n", "", _spec())
    assert "**" not in r.body
    assert r.body.startswith("\\textbf{$P(z)$")
    assert "$n(z)$" in r.body
