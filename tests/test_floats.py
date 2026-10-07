"""Tests for M5.1 float formatting on the Python side."""

import pytest

from ikat import BuildSpec, build_document, figure_env, weave_fragment
from ikat import tex_extra_packages
from ikat import texenv

MD = """# T

## 1. I

Hi.

```mermaid {span=column pos=both width=0.8}
graph TD
a[x]-->b[y]
```
"""


def test_weave_fragment_parses_float_attrs():
    (el,) = weave_fragment(MD)
    assert (el.span, el.pos, el.width) == ("column", "both", "0.8")


def test_weave_fragment_rejects_bad_attrs():
    with pytest.raises(ValueError):
        weave_fragment(MD.replace("pos=both", "pos=center"))
    with pytest.raises(ValueError):
        weave_fragment(MD.replace("width=0.8", "width=huge"))


def test_figure_env_placements():
    tikz = "\\begin{tikzpicture}\\node{a};\\end{tikzpicture}"
    f = figure_env(tikz, "C", "l", "column", "both")
    assert "\\begin{figure}[!tb]" in f
    f = figure_env("p.pdf", "C", "l", "wide", "barrier", "0.8", "top")
    assert f.startswith("\\FloatBarrier\n\\begin{figure*}[t]")
    assert "\\includegraphics[width=0.8\\textwidth]{p.pdf}" in f
    assert f.index("\\caption") < f.index("\\includegraphics")
    with pytest.raises(ValueError):
        figure_env(tikz, "C", "l", "wide", "force")


def test_plot_attrs_through_spec():
    spec = BuildSpec(plots=[("k", "C")], plot_attrs={"k": "pos=page"})
    r = build_document("# T\n\n## 1. I\n\nHi.\n", "", spec)
    assert "\\begin{figure}[p]" in r.tex


def test_extra_packages_and_check():
    assert tex_extra_packages("plain") == []
    tex = "\\begin{figure}[H]x\\end{figure}\n\\FloatBarrier\n"
    assert tex_extra_packages(tex) == ["\\usepackage{float}", "\\usepackage{placeins}"]
    req = dict(texenv.needs(tex + "\\documentclass{article}\n"))
    assert req["float.sty"] == "float"
    assert req["placeins.sty"] == "placeins"
