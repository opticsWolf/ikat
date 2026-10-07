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

## CLI / MCP / API — one surface

Same operations three ways. Python API is the reference; the CLI
and the MCP server call it directly (`ikat[mcp]` pins `mcp>=1,<2`).

```bash
ikat build doc.md --toml ikat.toml --outdir out --ensure-packages
ikat weave doc.md --spec spec.json > doc.tex   # tex to stdout, no TeX needed
ikat check doc.tex [--install]                 # missing packages (-> exit 1)
ikat templates | ikat template arxiv --show-preset
printf 'graph TD\na[x]-->b[y]\n' | ikat flowchart -
ikat version
```

`--spec` is BuildSpec JSON (diagrams/plots/table_captions/bib_style/…;
unknown keys ignored). MCP server over stdio:

```bash
python -m ikat.mcp_server
```

```jsonc
// Claude Desktop / pi client config
{ "mcpServers": { "ikat": {
  "command": "/path/to/.venv/Scripts/python.exe",
  "args": ["-m", "ikat.mcp_server"],
  "cwd": "/path/to/ikat" } } }
```

| operation | Python API | CLI | MCP tool |
|---|---|---|---|
| weave document | `build_document` / `build_from_paths` | `ikat build` / `weave` | `weave_document` |
| flowchart | `flowchart_to_tikz` | `ikat flowchart` | `flowchart_to_tikz` |
| bar chart | `barchart_to_tikz` | — | `barchart_to_tikz` |
| line plot | `lineplot_to_tikz` | — | `lineplot_to_tikz` |
| check packages | `check_tex_env` | `ikat check` | `check_tex_packages` |
| install packages | `ensure_tex_packages` | `ikat check --install` | `ensure_tex_packages` |
| templates | `list_templates` / `template_path` / `template_preset` | `ikat templates` / `template` | `list_templates` / `get_template` |

`compile_pdf` stays out of MCP (workdir-bound, minutes-long);
drive it from the CLI.

## Templates

The generated preamble fits the common case; journals are not
the common case. `[template]` in `ikat.toml` hands you the head:

```toml
[template]
preamble_file = "journal-head.tex"      # replaces the generated head
preamble_append = ["\\usepackage{natbib}"]  # extra lines before \\begin{document}
```

- `preamble_file` (resolved relative to the toml) replaces
  everything before `\begin{document}` — class, packages, title,
  author. The body tail (`\maketitle` … bibliography …
  `\end{document}`) stays generated.
- Safety over silence: ikat validates the replacement carries a
  `\documentclass` and every package the woven body provably
  needs (`tikz` for diagrams, `pgfplots` for plots, `graphicx`
  for precompiled figures, `tabularx` for tables). A head that
  drops one fails fast naming it, instead of dying in a TeX log.
- Path-based builds resolve it for you:
  `build_from_paths(md, toml, spec)`; `spec.preamble_override`
  set directly beats the toml value.

### Shipped library

Nine starting heads in `ikat/templates/` (each documents its
companion config). Tokens `{{{title}}}`, `{{{author}}}` and
`{{{thanks}}}` fill from the manuscript H1 + spec author/thanks;
an emptied `\\thanks{}` is dropped (it kills IEEEtran titles).

| name | class | bib_style | needs flag |
|---|---|---|---|
| `arxiv` | article 11pt, arXiv-safe | IEEEtran | — |
| `article-1col` / `-2col` | article + geometry | IEEEtran | — |
| `article-3col` | article + multicol (all spans column) | IEEEtran | — |
| `ieee-conference` | IEEEtran conference (the generated default, explicit) | IEEEtran | — |
| `acm-sigconf` | acmart sigconf (no hyperref/lmodern/geometry: class-owned) | ACM-Reference-Format | abstract first |
| `springer-llncs` | llncs runningheads | splncs04 | — |
| `elsevier` | elsarticle preprint | elsarticle-num | abstract first |
| `aps` | revtex4-2 (title+authors ride AfterEndPreamble; no tabularx: REVTeX clashes) | apsrev4-2 | abstract first |

`abstract_before_maketitle = true` (in `[template]`) hoists the
body's abstract env before `\\maketitle` for top-matter classes
(ACM, Elsevier, APS set the abstract in the title block — a late
abstract is silently dropped, proven by probe). `template_preset(name)`
returns each head's bib_style + toml snippet:

```python
from ikat import template_path, template_preset
preset = template_preset("elsevier")
# {'bib_style': 'elsarticle-num', 'toml': '[template]\nabstract_before_maketitle = true\n'}
```

Every head is compile-proven locally (abstract + TikZ + table
through pdflatex, title/abstract text verified in the PDF);
`examples/headsproof.py` re-runs the proof. Publisher classes
were installed from TeX Live archives into the local texmf tree
(same trick as IEEEtran/pgfplots) — on a full TeX Live they just
work. Heads are starting points, not submissions: always verify
against the publisher's proof.

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

Never debug a missing `.sty` by hand again — the texliveonfly-style
helper probes any `.tex` preamble and installs what's absent:

```python
from ikat import check_tex_env, ensure_tex_packages
check_tex_env(open("doc.tex").read())
# {'missing': ['pgfplots.sty'], 'unprobed': [], 'ok': False, ...}
ensure_tex_packages(open("doc.tex").read())  # tlmgr install + re-probe
```

`compile_pdf(workdir, main, ensure_packages=True)` runs the same
check before compiling and raises with a fix-it hint (manual
texmf install or MiKTeX Portable) instead of a cryptic TeX error.

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
