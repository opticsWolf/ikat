"""Build the ikat showcase paper: plots, weave, compile.

Usage (from ikat/ with the dev venv):
    .venv/Scripts/python.exe examples/ikat-paper/build.py
Needs pdflatex + bibtex on PATH (TinyTeX is fine).

Every construct in ikat-paper.md exercises a real pipeline feature:
inline mermaid with span/pos/width attrs, %% table directives,
citations, math, code spans, unicode, a dagger note, skeleton
tokens, plot registry + plot_attrs, and [floats] tuning.
"""

import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).parent
ROOT = HERE.parent.parent
sys.path.insert(0, str(ROOT / "python"))

from ikat import (  # noqa: E402
    BuildSpec,
    DiagramEntry,
    barchart_to_tikz,
    bib_keys,
    build_from_paths,
    compile_pdf,
    lineplot_to_tikz,
    tex_requirements,
)

OUT = HERE / "out"
FIGS = OUT / "figs"

# Real data, grep-counted from the ikat tree (see README below).
RUST_LOC = {
    "config": 473, "doc": 1065, "esc": 281, "lib": 214,
    "mermaid": 450, "plot": 224, "table": 213,
    "tectonic": 43, "texenv": 253,
}
GROWTH_RUST = [21, 22, 22, 22, 22, 22, 28, 32, 36, 36, 36, 46, 49, 50, 50, 50, 51]
GROWTH_PY = [8, 8, 8, 8, 8, 8, 16, 20, 22, 31, 31, 36, 40, 40, 41, 41, 44]

THANKS = (
    "Showcase paper: every construct below — floats, tables, "
    "citations, plots — is rendered by ikat itself."
)


def standalone(tikz: str) -> str:
    # Tight page: the plot PDF must contain no page margins, or
    # \includegraphics scales the whitespace instead of the plot.
    # preview loads AFTER pgfplots (before it, axis envs ship zero
    # pages — load-order quirk, proven by probe).
    req = tex_requirements("tikzpicture" in tikz, True)
    lines = ["\\documentclass{article}", "\\usepackage[utf8]{inputenc}"]
    lines += req
    lines += [
        "\\usepackage[active,tightpage]{preview}",
        "\\PreviewEnvironment{tikzpicture}",
        "\\setlength\\PreviewBorder{4pt}",
    ]
    return "\n".join(lines + ["\\begin{document}", tikz, "\\end{document}"])


def compile_standalone(tex: str, pdf: Path) -> None:
    work = pdf.parent
    src = work / (pdf.stem + ".tex")
    src.write_text(tex, encoding="utf-8")
    for exe, args in (
        ("pdflatex", ["-halt-on-error", "-interaction=nonstopmode", src.name]),
    ):
        p = subprocess.run([exe, *args], cwd=work, capture_output=True, text=True)
        if p.returncode != 0:
            raise SystemExit(f"{exe} failed:\n{(p.stdout + p.stderr)[-1500:]}")
    got = work / (pdf.stem + ".pdf")
    got.rename(pdf)


def make_plots() -> None:
    groups = list(RUST_LOC)
    vals = [list(RUST_LOC.values())]
    loc = barchart_to_tikz(
        "ikat Rust: lines of code by module", "lines", False,
        groups, ["shipped code"], vals, vals, vals, None,
    )
    compile_standalone(standalone(loc), FIGS / "fig-loc.pdf")
    xs = [float(i + 1) for i in range(len(GROWTH_RUST))]
    zero = [0.0] * len(xs)
    growth = lineplot_to_tikz(
        "test functions per commit (grep)", "commit (build order)", "count",
        xs, ["Rust tests", "Python tests"],
        [list(map(float, GROWTH_RUST)), list(map(float, GROWTH_PY))],
        [zero, zero],
    )
    compile_standalone(standalone(growth), FIGS / "fig-tests.pdf")
    print(f"plots: {FIGS / 'fig-loc.pdf'}, {FIGS / 'fig-tests.pdf'}")


def stage_bib_style(out: Path, style: str = "IEEEtran") -> None:
    """bibtex finds `.bst` via kpsewhich or not at all: stage a local
    copy next to the document (same trick as the main paper build).
    Never bundled for arXiv; purely a local-compile concern."""
    import subprocess

    try:
        hit = subprocess.run(
            ["kpsewhich", f"{style}.bst"], capture_output=True, text=True
        ).stdout.strip()
    except FileNotFoundError:
        hit = ""
    if hit:
        return
    candidates = [
        Path("/c/Users/Main/texmf/bibtex/bst/IEEEtran/IEEEtran.bst"),
        Path.home() / "texmf" / "bibtex" / "bst" / "IEEEtran" / "IEEEtran.bst",
    ]
    for cand in candidates:
        if cand.is_file():
            head = cand.read_text(encoding="utf-8", errors="replace")[:200]
            if "<html" in head.lower():  # a failed download, not a style
                continue
            shutil.copy(cand, out / f"{style}.bst")
            print(f"staged {style}.bst (local compile only)")
            return
    raise SystemExit(f"no {style}.bst found (kpsewhich + texmf); install it")


def make_paper(engine: str = "pdflatex") -> None:
    ref_bib = (HERE / "refs.bib").read_text(encoding="utf-8")
    spec = BuildSpec(
        diagrams=[
            DiagramEntry(key="fig-pipeline", caption="The ikat build: sources in, assembly, one engine decision.", mode="inline"),
            DiagramEntry(key="fig-sequence", caption="The same build as a sequence: calls across the PyO3 boundary, engine choice as an alt box.", mode="inline"),
            DiagramEntry(key="fig-states", caption="The document path as states: fences convert, the body wraps and validates, the engine compiles.", mode="inline"),
            DiagramEntry(key="fig-floats", caption="Float resolution: attrs against policy.", mode="inline"),
        ],
        plots=[
            ("fig-loc", "Rust lines of code by shipped module (wc, HEAD; test-only spike excluded)."),
            ("fig-tests", "Test functions per commit, both suites (grep, no smoothing)."),
        ],
        plot_attrs={"fig-loc": "pos=force width=0.75", "fig-tests": "pos=force width=0.75"},
        table_captions=[
            "Per-element float attributes.",
            "Test suites and what they guard.",
            "Compile engines compared.",
            "Skeleton token contract.",
        ],
        table_specs={"3": "XXl"},
        plot_insert_before="\\subsection{Where",
        title_thanks=THANKS,
        author="opticsWolf",
        bib_name="ikat-refs",
        bib_keys=set(bib_keys(ref_bib)),
    )
    r = build_from_paths(HERE / "ikat-paper.md", HERE / "ikat.toml", spec)
    (OUT / "ikat-paper.tex").write_text(r.tex, encoding="utf-8")
    shutil.copy(HERE / "refs.bib", OUT / "ikat-refs.bib")
    stage_bib_style(OUT, spec.bib_style)
    pdf = compile_pdf(OUT, "ikat-paper.tex", engine=engine)
    print(f"paper: {pdf} ({pdf.stat().st_size} bytes, "
          f"diagrams={r.n_diagrams} tables={r.n_tables})")


if __name__ == "__main__":
    import argparse

    ap = argparse.ArgumentParser()
    ap.add_argument("--engine", default="pdflatex", choices=["pdflatex", "tectonic"])
    engine = ap.parse_args().engine
    if shutil.which(engine) is None:
        raise SystemExit(f"{engine} not on PATH")
    OUT.mkdir(exist_ok=True)
    FIGS.mkdir(exist_ok=True)
    make_plots()
    make_paper(engine)
