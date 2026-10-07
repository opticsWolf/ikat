# ikat roadmap

Status: Phase 0 done (scaffold, Rust core, thin Python, 10 cargo + 5 pytest
green, `examples/demo.py` proves md → TikZ → PDF).

Acceptance rules for every phase: `cargo test` + `pytest` green,
`maturin develop` warning-free, demo still builds.

## Phase 1 — Dogfood the paper (pipeline parity) ✅ DONE 2026-10-06

All compute in Rust (`src/esc.rs`, `src/table.rs`, `src/doc.rs`);
Python (`python/ikat/document.py`) only carries strings over the
PyO3 boundary. Golden test `tests/test_paper.py` rebuilds
`paper-2026-10-05.tex` **byte-identical** to `build-paper.py` output
(4 diagrams, 9 tables, 50-entry `.bib` derivative identical too).
Strictness upgrade kept: dangling citation keys fail the build.

Goal: ikat rebuilds `paper-2026-10-05.tex` from the draft markdown.
This is the fastest hardener: the paper's converter edge cases become
ikat's test suite.

- **M1.1 tables.** Pipe tables → `tabularx` with per-table colspec
  (`Xcl` appendix style), caption registry, `TABLE` numbering.
  Port from `build-paper.py::table_block`.
- **M1.2 citations.** `[key]`, `[author: key]`, `;`-groups, prose
  fallback — with keyset validation against the staged `.bib` so a
  dangling key fails the build instead of printing raw.
- **M1.3 inline.** Unicode→LaTeX map, `$…$` math protection,
  code/emphasis, IEEE section-number stripping.
- **M1.4 document writer.** Preamble from `ikat.toml` (class, options,
  columns, margins), float assembly in source order, `.bbl` passthrough.

*Acceptance:* ikat output diffed against the checked-in `.tex`
shows only intended improvements; resulting PDF compiles with
0 undefined citations.

## Phase 2 — Data plots live (matplotlib out)

- **M2.1 presets.** Benchmark JSON (`results/*.json` shape) →
  `barchart_to_tikz` / `lineplot_to_tikz` directly: series names,
  whiskers from min/max, refline, footnote text.
- **M2.2 regeneration.** The 4 paper plots rebuilt through ikat;
  side-by-side visual diff against the matplotlib PDFs.
- **M2.3 local pgfplots.** ✅ DONE 2026-10-06 — tlnet `pgfplots.tar.xz`
  extracted into the TinyTeX texmf tree (`tlmgr` GPG is broken on
  this mirror chain); `tex_requirements()` auto-detects TikZ/pgfplots
  needs from the woven body so preambles stay minimal.

*Acceptance:* `paper/figs/` producible with no matplotlib import
anywhere in the loop.

## Phase 3 — Figures advanced

- **M3.1 standalone export.** `\documentclass[tikz]{standalone}`
  wrapper for precompiled, arXiv-safe figure PDFs.
- **M3.2 sequence subset**, **M3.3 state subset** behind the same
  `flowchart_to_tikz` entry point (separate grammars, shared emitter).
- **M3.4 layout.** Edge routing that avoids node interiors, subgraph
  cluster boxes, wider DAG support (today: layered trees/DAGs only).

*Acceptance:* the paper's remaining 3 TikZ diagrams regenerate from
their mermaid fences with no hand-tuning.

## Phase 4 — Harden & release

- **M4.1 errors.** Line-numbered parse errors (statement echo +
  caret) instead of bare strings.
- **M4.2 CLI.** ✅ DONE 2026-10-07 — `ikat build/weave/check/templates/template/flowchart/version`
  (`python/ikat/cli.py`, `ikat` console script): same operations as
  the API, `--spec` BuildSpec JSON, `--ensure-packages` gate.
- **M4.8 MCP server.** ✅ DONE 2026-10-07 — `python -m
  ikat.mcp_server` (FastMCP stdio, `ikat[mcp]` extra): nine tools
  mirroring the CLI/API map; `compile_pdf` deliberately excluded.
- **M4.2b CI compile check.** Docker TeX Live job compiling the
  demo and the paper on every push (backend verification, not a
  user-facing backend).
- **M4.5 ensure-packages helper.** ✅ DONE 2026-10-06 — the
  texliveonfly trick: `src/texenv.rs` scans any preamble into
  `(probe file, tlmgr package)` needs, `python/ikat/texenv.py`
  probes via one `kpsewhich` call and `tlmgr install`s what's
  missing; `compile_pdf(..., ensure_packages=True)` gates the
  build on it.
- **M4.6 user templates.** ✅ DONE 2026-10-06 — `[template]`
  `preamble_file` replaces the generated head, `preamble_append`
  adds lines before `\\begin{document}`; Rust validates the
  replacement carries a class + every package the body needs.
- **M4.7 template library.** ✅ DONE 2026-10-07 — nine shipped
  heads (arXiv, article 1/2/3-col, IEEE, ACM, LNCS, Elsevier, APS)
  with `{{{title}}}`/`{{{author}}}`/`{{{thanks}}}` tokens,
  per-template presets (bib_style + toml), `abstract_before_maketitle`
  hoist for top-matter classes, all nine compile-proven locally.
- **M4.3 claim the names.** Publish 0.1.0 to crates.io + PyPI early —
  both `ikat` names are currently free and publishing reserves them.
- **M4.4 self-hosting.** The next paper revision is built with ikat
  itself; `build-paper.py` retires to `docs/`.

## Non-goals

Full mermaid coverage (deliberately a strict subset), WYSIWYG,
bibliography management (`.bib` stays the source of truth),
reference-manager integration.

## Ordering

Phase 1 before 2 (plots need a pipeline to land in); 3 after 2
(layout work needs real diagrams to test against); 4 last, except
M4.3, which can jump the queue any time — it costs nothing.

## Phase 5 — Floats, skeletons, parsing (spec: docs/FORMAT-DRAFT.md)

- **M5.1 float formatting.** ✅ DONE 2026-10-07 — per-element
  `span/pos/width/captionpos` attrs (`%% table {...}` for tables,
  `plot_attrs` registry for plots) + `[floats]` fractions/counters/
  `barrier_sections` + `table*` + `picture` kind; packages
auto-added or template-validated; compile-proven to PDF.
- **M5.2 level-3 skeleton.** ✅ DONE 2026-10-07 — `[template]`
  `skeleton` with `{{body}}`/`{{bibliography}}`/`{{abstract}}`
  token contract (comment-aware counting/rendering, unknown-token
  errors, exclusion vs `preamble_file`), shipped
  `skeleton-plain.tex`, compile-proven to PDF.
- **M5.3 Markdown AST spike.** pulldown-cmark event-diff vs the
  hand scanner on the paper manuscript; switch only on empty diff
  + byte-identical golden `.tex`. Document the verdict here.
