"""Tests for M5.2 level-3 skeletons on the Python side."""

import pytest

from ikat import BuildSpec, build_document, build_from_paths, list_skeletons, template_path

MD = """# T

## Abstract

Abs here.

## 1. I

Hi.
"""

SKEL = template_path("skeleton-plain").read_text(encoding="utf-8")


def test_shipped_skeleton_lists():
    assert "skeleton-plain" in list_skeletons()
    for tok in ("{{title}}", "{{body}}", "{{bibliography}}", "{{abstract}}"):
        assert tok in SKEL


def test_skeleton_round_trip():
    toml = "[template]\nabstract_before_maketitle = true\n"
    spec = BuildSpec(skeleton=SKEL)
    r = build_document(MD, toml, spec)
    assert "\\title{T}" in r.tex
    assert "\\begin{abstract}" in r.tex
    assert r.tex.index("Abs here.") < r.tex.index("Hi.")
    assert "\\bibliography{refs-paper}" in r.tex
    # Tokens survive only inside % comments (the documented contract).
    assert all(l.lstrip().startswith("%") for l in r.tex.splitlines() if "{{" in l)


def test_skeleton_preamble_append(tmp_path):
    toml = tmp_path / "ikat.toml"
    toml.write_text('[template]\nskeleton = "s.tex"\n', encoding="utf-8")
    (tmp_path / "s.tex").write_text(SKEL, encoding="utf-8")
    (tmp_path / "d.md").write_text(MD, encoding="utf-8")
    spec = BuildSpec(preamble_append=["\\usepackage{booktabs}"])
    r = build_from_paths(tmp_path / "d.md", toml, spec)
    assert r.tex.index("\\usepackage{booktabs}") < r.tex.index("\\begin{document}")


def test_skeleton_excludes_preamble_file(tmp_path):
    toml = tmp_path / "ikat.toml"
    toml.write_text('[template]\npreamble_file = "h.tex"\n', encoding="utf-8")
    (tmp_path / "h.tex").write_text("\\documentclass{article}\n", encoding="utf-8")
    (tmp_path / "d.md").write_text(MD, encoding="utf-8")
    with pytest.raises(ValueError, match="mutually exclusive"):
        build_from_paths(tmp_path / "d.md", toml, BuildSpec(skeleton=SKEL))
    # skeleton alone resolves from the toml key
    (tmp_path / "s.tex").write_text(SKEL, encoding="utf-8")
    toml.write_text('[template]\nskeleton = "s.tex"\n', encoding="utf-8")
    r = build_from_paths(tmp_path / "d.md", toml, BuildSpec())
    assert "\\title{T}" in r.tex
