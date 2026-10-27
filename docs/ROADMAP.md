# ikat roadmap

Planning lives here; history lives in `docs/CHANGELOG.md`.

Status 2026-10-27 (0.12.0, on `main`): all planned phases and
milestones are done — Phases 1–6 (pipeline, plots, figures,
hardening, floats/skeletons/parsing, showcase), Phases A–H
(presets, standalone, sequence/state, layout+guards, parser
switch, line-numbered errors, tectonic hardening, auto-layout),
M7.2–M7.10b (float policy, math, author front matter, MCP
surface), the Linux-gated tectonic embed, and texres as the
third engine. Shipped: v0.11.0 on PyPI (wheels 3.10–3.15 + PyPy,
Trusted Publishing) and crates.io. 173 cargo + 104 pytest green;
hand and auto showcases byte-identical; the site link-gates its
deploys. Open: nothing.

Acceptance rules for every change: `cargo test` + `pytest` green,
`maturin develop` warning-free, demo + showcase paper still build.

## Engine ladder (current)

1. `pdflatex` (default) — a local TeX distribution; `bibtex` for
   staged bibliographies; `ensure_packages` can install missing
   LaTeX packages (texliveonfly trick).
2. `tectonic` — the `TECTONIC_EXE`/PATH binary everywhere; on
   Linux also embeddable via the `tectonic` cargo feature
   (Linux-gated, CI-exercised; the C/C++ XeTeX core has no
   pkg-config answer elsewhere).
3. `texres` — the one-binary Rust TeX distribution
   (`TEXRES_EXE`/PATH): LaTeX, packages, fonts, and BibTeX
   bundled, all passes to convergence, nothing to install.

## Watch: TeXres — the next engine step (2026-10-27)

**TeXres** (github.com/leoliu0/texres, v0.7.8 2026-10-23,
MIT/Apache-2.0): a TeX distribution in a single Rust executable —
pdfLaTeX/XeLaTeX/LuaLaTeX + formats + package archive + fonts +
BibTeX/Biber, fully offline, output CI-compared against TeX Live
2026. Empirically verified on the showcase: the woven
`ikat-paper.tex` (IEEEtran + TikZ + pgfplots + staged `.bib`)
compiled on Windows with zero other software installed — 7 pages,
IEEE-style References, line-anchored errors. Also surveyed, all
lesser: `texforge` (workflow wrapper that installs tectonic),
`tex_engine` (GPL, research-grade), `rustatex` (own language,
toy), Typst (different language — an emission target, not an
engine swap).

The strategic prize: texres ships a **C library and a WASM
module** that compile documents in memory — a C embedding WITHOUT
the pkg-config/system-lib chain that makes the `tectonic` cargo
feature Linux-only. The credible endgame for an embedded engine
on Windows; could replace the tectonic embed entirely.

**Embed feasibility (checked 2026-10-27):** upstream publishes NO
crate to crates.io (`libtex` absent); a git-dependency on their
workspace would work, but the data is baked into the binary
(that is why the exe is 810 MB) — an ikat embed would balloon the
extension module to ~800 MB, far past PyPI's 100 MB upload cap.
So: embed = source-build/offline-deployment option only, never a
wheel; the cascade slot is the shippable form (shipped, 0.11.2).

**Why watch, not deepen:** 810 MB per platform, v0.7.x maturity,
single maintainer. Revisit when a Windows/CI pain point or the
WASM/C-library embedding justifies the work.

## Non-goals

Full mermaid coverage (deliberately a strict subset), WYSIWYG,
bibliography management (`.bib` stays the source of truth),
reference-manager integration.

## Ordering

Nothing is queued. New work lands as M7.x-style milestones with
their own gate commits and version bumps (the 0.10.x pattern);
version numbering has no 1.0 planned. Candidate directions, none
committed: an in-memory texres embed (see the watch item), slim
crates.io packaging via `[package] include`, and Macrame-side
integration work tracked in the Macrame_docs repository.

## Tracked elsewhere (not ikat work)

- **Paper track** (`paper/`): pending `[NOT YET MEASURED]`
  benches, `0.19.0` tag + decision-register permalink, fig
  regeneration via ikat plots (M2.2's consumer), `arxiv-submit/`
  refresh and submission.
- **okfgraph upstream**: `--target chunks` segfault, parallel-CLI
  DB lock, phantom `image_count`, INFO-on-stdout breaking
  `--json`, no `okf delete`.
- **Repo hygiene**: Macrame_docs has no remote; local commits
  (incl. `paper/ikat-spec.json`) need a backup remote.
