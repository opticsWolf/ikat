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
  spanning model. Borrowed: user template overrides (shipped as
  `[template]`, roadmap M4.6); quote-style options declined —
  generated preambles shouldn't need them.
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

## Compile-backend alternatives (checked 2026-10-06)

The job: turn woven `.tex` into PDF without fighting system TeX.
Tectonic (above) is the embeddable candidate; the rest of the field:

- **TinyTeX (incumbent).** Minimal TeX Live + on-demand `tlmgr`.
  What ikat uses today. Weakness demonstrated firsthand: `tlmgr`
  GPG/mirror failures forced manual IEEEtran/pgfplots installs.
  Stays the default (arXiv-closest engine: real pdfTeX).
- **MiKTeX portable + on-the-fly install.** Best Windows story:
  needs no admin rights, and missing packages auto-install at
  compile time (its own manager/mirrors — a different chain from
  the `tlmgr` one that bit us). Still a system install, just a
  portable one. Verdict: document as the Windows fallback path;
  no code changes needed (`compile_pdf` already shells to
  `pdflatex`).
- **SwiftLaTeX (XeTeX/pdfTeX → WebAssembly).** Full engines in
  WASM, near-identical XeTeX (minus full ICU: locale linebreaking
  caveat), deployable server-side via wasmtime/Node. Fascinating
  but wrong weight class: embedding a WASM runtime + texmf
  payload dwarfs the problem. Verdict: out of scope; revisit if
  ikat ever needs browser-side preview.
- **Docker TeX Live images** (`texlive/texlive`, pandoc/latex
  images, latex GitHub Actions). The CI standard: reproducible,
  version-pinned, zero host pollution. Needs a daemon, so never a
  library default — but ideal as ikat's own CI job compiling the
  demo and the paper on every push. Verdict: adopt for CI, not
  as a backend.
- **Hosted compile APIs** (latexonline-style endpoints,
  self-hosted Overleaf CE). Zero-install at the price of network,
  trust, and availability. Verdict: manual fallback only, never
  a dependency.
- **texliveonfly(.py).** Wrapper that auto-installs missing TeX
  Live packages mid-compile. Complementary idea (an ikat
  "ensure packages" helper could steal the trick), not an engine.
- **Typst.** Different language, cannot compile `.tex`; journals
  and arXiv still require LaTeX in 2026. Out of scope by
  definition — noted only because it comes up every time.

Net: no change to the tectonic decision (optional engine behind
the flag). Actionable now: (1) document MiKTeX-portable as the
Windows fallback in README, (2) Docker-based CI compile check as
a roadmap item.

## Publisher requirements survey (checked 2026-10-07)

What each shipped head assumes, verified against installed class
sources (not just docs) and proven by compiling every head with
an abstract + TikZ + table through pdflatex:

- **ACM (acmart, sigconf).** Class pre-loads graphicx + hyperref
  (configure via `\hypersetup`, never re-load with options) and
  the newtxmath font stack (re-loading amssymb clashes on
  `\Bbbk`); lmodern/geometry forbidden. Abstract is set in
  `\maketitle` ⇒ flag required. Bib `ACM-Reference-Format`
  (ships in the class package). Double-blind: add
  `review,anonymous` to the class options.
- **Springer LNCS (llncs, runningheads).** Plain `\maketitle` +
  abstract-after; bib `splncs04`. No flag.
- **Elsevier (elsarticle, preprint).** Preamble declarations work
  with no frontmatter env (proven), but the abstract is top
  matter ⇒ flag required or it vanishes (proven dropped). Final
  formats `[5p]`/`[3p]` take the same source. Bib
  `elsarticle-num`.
- **APS (revtex4-2, aps/prl).** `\title` AND `\author` are
  body-scoped ⇒ both ride `\AfterEndPreamble` (etoolbox);
  abstract before `\maketitle` ⇒ flag required. tabularx
  clashes with REVTeX internals (`Extra \or`) ⇒ omitted; bodies
  with tabularx fail validation naming it. Bib `apsrev4-2`.
- **IEEE (IEEEtran, conference).** The generated default, made
  explicit. Empty `\thanks{}` drops the title ⇒ ikat strips it.
- **arXiv.** article 11pt single-spaced, standard packages only;
  nothing outside TeX Live, no shell-escape.

Local TeX cost of the survey: acmart/llncs/elsarticle/revtex +
their bst files + the acmart dependency tree (xstring, textcase,
aliascnt, microtype, totpages, environ, trimspaces, hyperxmp,
ifmtarg, oberdiek, balance/preprint, manyfoot/ncctools, fonts for
libertine+newtx with updmap-enabled maps, upquote, fontaxes,
binhex/kastrup) installed from tlnet archives into texmf-local —
the same trick as IEEEtran/pgfplots, now routine.

## Strategy decisions

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

## Tectonic backend evaluation (checked 2026-10-06)

[Tectonic](https://tectonic-typesetting.github.io/en-US/) (crate
`tectonic` 0.17.0, maintained, ~196k downloads) is a complete
XeTeX-based TeX engine as an embeddable Rust library: no system TeX,
support files auto-downloaded as content-addressed bundles, an
all-in-one `latex_to_pdf` plus a `driver` module, and BibTeX via
`tectonic_engine_bibtex`. No usable Python bindings exist on PyPI —
but ikat is already Rust+PyO3, so wrapping it ourselves is natural.

Fit for ikat: **yes, as an optional engine, not the default.**

- For: zero-install builds (no TinyTeX wrestling like the
  IEEEtran/pgfplots episodes); pinned bundle ⇒ reproducible PDFs
  across machines; single `compile_pdf(engine="tectonic")` path.
- Against: engine is XeTeX, not pdfTeX — our `[utf8]{inputenc}`
  preamble needs an engine-conditional branch; the bundle is a
  TeX Live *snapshot* that will drift from arXiv's live tree, so
  arXiv preview remains the submission authority; first builds
  need network + cache (offline requires a pre-seeded bundle);
  linking C/C++ engines balloons maturin wheels.
- Design: `tectonic` cargo feature, **off by default** (lean
  default wheels); `compile` module in Rust wrapping the driver
  with bibtex passes; Python only threads `engine=` through.
  IEEEtran/pgfplots ship in the bundle (full TeX Live snapshot).

Not implemented yet: tectonic's C build is slow and its CDN may be
unreachable from some sandboxes — prototype on a networked machine
first, behind the feature flag.
