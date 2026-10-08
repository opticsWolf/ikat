"""Standalone figures: one fence's TikZ -> arXiv-safe figure PDF.

A standalone figure is a picture, not a float: no caption, no label,
no bibliography — `\\documentclass[tikz,border=5pt]{standalone}`
(the house style the staged paper figures already use) plus the
minimal detected packages (TikZ comes with the class option;
pgfplots only when the body has an `axis`).
"""

from __future__ import annotations

from pathlib import Path

from ._core import plots_needed, tikz_needed


def fence_to_tikz(lang: str, body: str) -> str:
    """Render one fence body by language: `mermaid` or `chart` (preset JSON)."""
    if lang == "mermaid":
        from ._core import flowchart_to_tikz

        try:
            return flowchart_to_tikz(body)
        except ValueError as e:
            raise ValueError(f"mermaid fence: {e}") from e
    if lang == "chart":
        from .preset import load_preset

        try:
            return load_preset(body)
        except ValueError as e:
            raise ValueError(f"chart fence: {e}") from e
    raise ValueError(f"standalone needs a mermaid|chart fence, got {lang!r}")


def wrap_standalone(tikz_src: str) -> str:
    """Wrap one `tikzpicture` in the minimal standalone document."""
    if not tikz_needed(tikz_src):
        raise ValueError("standalone needs a tikzpicture body")
    lines = ["\\documentclass[tikz,border=5pt]{standalone}"]
    # The class loads TikZ itself, but not the arrow/shape
    # libraries the emitter's `>=Stealth` needs (same set as
    # `tex_requirements`; proven by the Stealth probe above).
    lines += ["\\usetikzlibrary{shapes.geometric,arrows.meta,positioning}"]
    if plots_needed(tikz_src):
        lines += ["\\usepackage{pgfplots}", "\\pgfplotsset{compat=1.18}"]
    return "\n".join(lines + ["\\begin{document}", tikz_src.rstrip(), "\\end{document}"])


def compile_standalone(tikz_src: str, workdir: str | Path,
                       engine: str = "pdflatex") -> Path:
    """Write `fig.tex` in `workdir`, compile it, return the PDF path."""
    from .compile import compile_pdf

    workdir = Path(workdir)
    workdir.mkdir(parents=True, exist_ok=True)
    (workdir / "fig.tex").write_text(wrap_standalone(tikz_src), encoding="utf-8")
    return compile_pdf(workdir, "fig.tex", engine=engine)
