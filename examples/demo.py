"""End-to-end demo: mini.md -> TikZ -> figure* -> PDF.

Usage (from ikat/ with the dev venv):  .venv/Scripts/python.exe examples/demo.py [--engine tectonic]
Requires the engine on PATH (pdflatex via TinyTeX, or tectonic).
"""

import argparse
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).parent
ROOT = HERE.parent
sys.path.insert(0, str(ROOT / "python"))

# maturin develop installs ikat into .venv; fall back to source tree.
try:
    import ikat
except ImportError:  # pragma: no cover
    raise SystemExit("run `maturin develop` first")

from ikat import compile_pdf, figure_env, weave_fragment  # noqa: E402

PREAMBLE = r"""\documentclass[conference]{IEEEtran}
\usepackage[utf8]{inputenc}
\usepackage{tikz}
\usetikzlibrary{shapes.geometric,arrows.meta,positioning}
% pgfplots joins once plot.rs output is woven in (needs texlive pgfplots).
\title{ikat demo}
\author{\IEEEauthorblockN{opticsWolf}}
\begin{document}
\maketitle
"""

md = (HERE / "mini.md").read_text(encoding="utf-8")
toml_src = (HERE / "ikat.toml").read_text(encoding="utf-8")
outdir = HERE / "demo-out"
outdir.mkdir(exist_ok=True)

floats = [
    figure_env(el.body_tex, el.caption, el.label, el.span, el.pos or "top", el.width, el.captionpos)
    for el in weave_fragment(md, toml_src)
]
tex = PREAMBLE + "\n\n".join(floats) + "\n\\end{document}\n"
(outdir / "mini.tex").write_text(tex, encoding="utf-8")
ap = argparse.ArgumentParser()
ap.add_argument("--engine", default="pdflatex", choices=["pdflatex", "tectonic"])
pdf = compile_pdf(outdir, "mini.tex", engine=ap.parse_args().engine)
print("demo PDF ->", pdf, f"({pdf.stat().st_size} bytes)")
