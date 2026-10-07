# Dependency research (via Gossamer, 2026-10-06)

## Name

`loom` is taken on both registries (crates.io 0.7.2, 69M downloads —
a concurrency library; PyPI active). So are `weft`, `selvage`,
`shuttle`, `heddle`, `selvedge`, `jacquard`, `twill`, `dobby`,
`tangle`. Free on **both**: `ikat`, `seersucker`, `herringbone`.
**ikat** wins: a fabric pattern (dye-the-threads-before-weaving ≈
declare figures as data, weave at build time), short, memorable.

## Rust crates evaluated

| crate | ver | verdict |
|---|---|---|
| `mermaid` | 0.2.0 | **reject** — experimental linear algebra, unrelated to diagrams |
| `rust_tikz` | 0.1.0 | **reject** — immature, tiny API, no layout |
| `svg-tikz` | 0.2.1 | **fallback only** — SVG→TikZ path conversion; needs mermaid-cli + Node for the SVG, and yields raw paths, not semantic nodes |
| `pgfplots` (DJDuque) | 0.5.1 | **noted, not used** — real PGFPlots code generator (`Plot2D`, `Axis`, `AxisKey`/`PlotKey`, pdflatex *or* tectonic engines). Revisit if our emitter outgrows bar/line/log; today it would own our output formatting |
| `mdbook-tikz`, `depict-tikz`, `ast-to-mermaid` | — | adjacent, none parse mermaid flowcharts |
| `pyo3` / `maturin` / `serde` / `toml` | 0.29 / 1.15 / 1.0 / 1.1 | **used** — the whole stack |

Online mermaid→TikZ converters (Underleaf, useoctree, assorted
GitHub scripts) confirm demand but are services or unmaintained
snippets — no canonical offline library exists, which justifies our
own flowchart-subset parser.

## Prior art: the two md2tex projects (checked 2026-10-06)

- **paulhectork/md2tex** (Python): generic regex-based md→TeX CLI
  with templates and fine-tuning (quote styles, numbered headers,
  document classes). Hard-requires `minted`, which needs
  `-shell-escape` + Pygments — arXiv-hostile and heavier than our
  whole pipeline. No citations/bib, no figures-from-data, no
  spanning model. Borrow-worthy: user template overrides (a future
  `[template]` key) and quote-style options.
- **lbeckman314/md2tex** (Rust 0.1.3, crates.io): small md→tex/pdf
  via tectonic, forked from md2pdf for mdbook chapters, used by
  mdbook-latex. Unmaintained (Travis era). Scope is book prose:
  no citations, no tables-as-floats, no generated figures. No name
  conflict (`md2tex` vs `ikat`). Validates Rust-for-md→tex; its
  tectonic backend is a candidate future compile engine for
  self-contained builds without system TeX.

ikat's gap vs both: a *paper* pipeline (bib-validated citations,
float tables, IEEE sectioning) where figures are *generated* from
source (mermaid/TikZ, data/pgfplots) in the same build, with
per-element column spanning and arXiv-safe output.

1. **Emit pgfplots, don't plot.** The paper range (bars+whiskers,
   log-y, lines+error bands, reflines, footnotes) is small; a ~150-line
   emitter gives byte-level control (IEEE-safe tick labels, `$\sim$`
   approximations) with zero dependency risk.
2. **Subset grammar, not full mermaid.** Papers use `graph TD/LR` +
   four node shapes + labeled arrows. A strict parser with string
   errors beats a lenient one that silently mislays nodes.
3. **Compile figures in-document.** Standalone precompilation stays an
   option (roadmap §3), but default keeps one TeX run → one font set.
4. **arXiv fit.** Output is plain `tikzpicture`/pgfplots + standard
   packages — no shell-escape, no externalized PDFs, no JS.
