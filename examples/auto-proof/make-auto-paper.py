"""Rebuild the auto-layout showcase paper (deliverable provenance).

Takes examples/ikat-paper/ikat-paper.md + ikat.toml, applies the
current engine's suggestions, weaves with the REAL captions from
build.py, and compiles to build-ikat/ikat-paper.pdf. Run from the
ikat repo root with the ikat .venv (TinyTeX on PATH).
"""

import os
import shutil
from pathlib import Path

HERE = Path("examples/auto-proof").resolve()
SRC = Path("examples/ikat-paper").resolve()
WORK = HERE / "build-ikat"

os.environ["PATH"] = (
    r"C:\Users\Main\AppData\Roaming\TinyTeX\bin\windows"
    + os.pathsep
    + os.environ["PATH"]
)

from ikat import (  # noqa: E402
    BuildSpec,
    DiagramEntry,
    apply_layout,
    bib_keys,
    build_from_paths,
    compile_pdf,
    suggest_layout,
)

DIAGRAMS = [
    ("fig-pipeline", "The ikat build: sources in, assembly, one engine decision."),
    ("fig-sequence", "The same build as a sequence: calls across the PyO3 boundary, engine choice as an alt box."),
    ("fig-states", "The document path as states: fences convert, the body wraps and validates, the engine compiles."),
    ("fig-floats", "Float resolution: attrs against policy."),
]
# Block order follows the md: the skeleton table moved ahead of the
# engines table so table* takes the p7 top (mirrors build.py).
TABLES = [
    "Per-element float attributes.",
    "Test suites and what they guard.",
    "Skeleton token contract.",
    "Compile engines compared.",
]
PLOT_TITLES = {
    "fig-loc": "Rust lines of code by shipped module (wc, HEAD; test-only spike excluded).",
    "fig-tests": "Test functions per commit, both suites (grep, no smoothing).",
}


def main() -> None:
    if WORK.exists():
        shutil.rmtree(WORK)
    shutil.copytree(SRC, WORK)
    md = (WORK / "ikat-paper.md").read_text(encoding="utf-8")
    toml = (WORK / "ikat.toml").read_text(encoding="utf-8")
    spec = BuildSpec(
        diagrams=[DiagramEntry(key=k, caption=c, mode="inline") for k, c in DIAGRAMS],
        plots=[(k, t) for k, t in PLOT_TITLES.items()],
        plot_attrs={"fig-loc": "pos=force width=0.75", "fig-tests": "pos=force width=0.75"},
        table_captions=list(TABLES),
        table_specs={"2": "XXl"},
        plot_insert_before="\\subsection{Where",
        author="opticsWolf",
        bib_name="ikat-refs",
        bib_keys=set(bib_keys((SRC / "refs.bib").read_text(encoding="utf-8"))),
    )
    suggs = suggest_layout(md, toml, spec)
    new_md, n = apply_layout(md, suggs)
    (WORK / "ikat-paper.md").write_text(new_md, encoding="utf-8")
    print(f"applied {n}:")
    for s in suggs:
        if s["action"] == "change":
            print("  ", s["key"], s["span"], s["pos"], s.get("width"))
    # Path-based weave: resolves `skeleton = "skel.tex"` relative to
    # the toml directory (string-based build_document cannot, and
    # silently falls back to the generated head — that fallback put
    # the abstract on its own page under IEEEtran).
    r = build_from_paths(WORK / "ikat-paper.md", WORK / "ikat.toml", spec)
    (WORK / "ikat-paper.tex").write_text(r.tex, encoding="utf-8")
    figs = WORK / "figs"
    figs.mkdir(exist_ok=True)
    for name in ("fig-loc.pdf", "fig-tests.pdf"):
        shutil.copy(SRC / "out" / "figs" / name, figs / name)
    shutil.copy(SRC / "refs.bib", WORK / "ikat-refs.bib")
    shutil.copy(SRC / "out" / "IEEEtran.bst", WORK / "IEEEtran.bst")
    pdf = compile_pdf(WORK, "ikat-paper.tex", engine="pdflatex")
    print(f"auto paper: {pdf} ({pdf.stat().st_size} bytes, diagrams={r.n_diagrams} tables={r.n_tables})")
    # The SITE showcase asset carries the auto-layout rendering — the
    # engine outpacks the hand build (7 vs 8 pages, 0 overfulls), so
    # README/QUICKREF link this one. No-op in bare checkouts.
    site_assets = HERE.parent.parent / "site" / "assets"
    if site_assets.is_dir():
        shutil.copy(pdf, site_assets / "ikat-paper.pdf")
        print(f"site: {site_assets / 'ikat-paper.pdf'}")


main()
