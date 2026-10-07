# ikat

**Weave Markdown into camera-ready LaTeX.** `ikat` is an md→tex/pdf
pipeline whose compute core is Rust (via PyO3/maturin) with thin Python
wrappers: Markdown documents go in, conference-ready PDFs come out, and
every figure is generated — never screenshotted.

The name is a fabric pattern: in *ikat* weaving, threads are patterned
**before** they reach the loom, so the design is carried by the material
itself. Likewise, an ikat document carries its figures as data and
diagram source: the pipeline weaves them into TikZ/pgfplots at build
time, set in the document's own fonts. No matplotlib, no pasted PNGs,
no font mismatch between text and figures.

## What it does

- **Markdown → LaTeX** (`ikat.pipeline`): headings, tables, citations,
  code, footnotes, and fenced elements → a complete `.tex` document.
- **Mermaid → TikZ** (`ikat._core.flowchart_to_tikz`, Rust): the
  `graph TD/LR` flowchart subset → a `tikzpicture` with deterministic
  layered layout. No Node.js, no browser, no SVG round-trip.
- **Data → pgfplots** (`ikat._core`, Rust): bar charts with
  min/max whiskers, log axes, line plots → `tikzpicture` code compiled
  by your LaTeX installation, so figure fonts always match the paper.
- **Per-element spanning** (`ikat.toml`): one/two-column layout is not
  global — each element kind (diagram, plot, table) declares
  `column` or `wide`, overridable per element from Markdown.
- **Compile** (`ikat.compile`): `pdflatex`/`bibtex` driver that turns
  the woven tree into PDF.

## Layout

```text
ikat/
  src/            Rust core (pyo3): mermaid.rs, plot.rs, config.rs
  python/ikat/    thin wrappers: pipeline.py, compile.py
  tests/          pytest suite (runs against maturin develop build)
  docs/           ARCHITECTURE.md, RESEARCH.md
  examples/       ikat.toml + mini.md starter document
```

## Develop

Requires Rust (cargo) and Python ≥ 3.10.

```bash
uv venv .venv && uv pip install -p .venv maturin pytest
VIRTUAL_ENV=$PWD/.venv maturin develop
.venv/Scripts/pytest tests/      # windows
cargo test                       # Rust unit tests
```

## LaTeX requirements

ikat emits standard LaTeX; it never bundles classes or packages.
You need a working TeX installation with `pdflatex` (and `bibtex`
if you stage a `.bib`). TeX Live `scheme-full` covers everything;
minimal schemes (TinyTeX included) need at least:

| what | packages | needed when |
|---|---|---|
| base document | `amsmath amssymb tabularx graphicx hyperref lmodern textcomp` + your document class (`IEEEtran`, …) | always |
| inline diagrams | `pgf` (`tikz`) + TikZ libraries `shapes.geometric arrows.meta positioning` | any woven `tikzpicture` |
| data plots | `pgfplots` (compat 1.18) | any woven `axis` environment |

Precompiled-PDF figures need nothing beyond the base set — one
reason to prefer them for journal submissions.

```bash
tlmgr install amsmath amssymb tabularx graphicx hyperref lmodern \
  textcomp pgf pgfplots IEEEtran
```

Ask ikat what a given body needs instead of guessing:

```python
from ikat import tex_requirements
tex_requirements(has_tikz=True, has_plots=False)
# ['\\usepackage{tikz}',
#  '\\usetikzlibrary{shapes.geometric,arrows.meta,positioning}']
```

The document builder does this automatically: `build_document`
scans the woven body and injects only the packages it contains.

Notes:

- If `tlmgr` fails (broken mirror/GPG — seen in the wild),
  install from a TeX Live archive instead: unpack
  `systems/texlive/tlnet/archive/<pkg>.tar.xz`, copy its `tex/`
  tree into your texmf-local tree, run `mktexlsr`. That is how the
  dev setup gained `IEEEtran` and `pgfplots`. Windows alternative:
  MiKTeX Portable needs no admin rights and auto-installs missing
  packages at compile time (its own manager/mirrors); ikat's
  `compile_pdf` shells to `pdflatex`, so it works unchanged.
- arXiv's TeX Live ships everything above, so an ikat bundle
  (`.tex` + `.bbl` + figure PDFs, no class/style files) compiles
  there as-is.

## License

MIT OR Apache-2.0.
