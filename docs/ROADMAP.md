# ikat roadmap

Status 2026-10-07 (`4c6c9f8`): Phases 1, 4 (minus M4.1/M4.2b/ship),
5, and 6 done — 54 cargo + 43 pytest green, showcase paper builds
(6 pages, 2 diagrams, 2 plots, 4 tables, 0 undefined citations).
Open: M2.1, M2.2, M3.1–M3.4, M4.1, M4.2b, M4.3 (tokens only),
M5.4 (the pulldown-cmark switch), tectonic crate embedding.

Acceptance rules for every phase: `cargo test` + `pytest` green,
`maturin develop` warning-free, demo + showcase paper still build.

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

- **M2.1 presets.** OPEN — benchmark JSON (`results/*.json` shape)
  → `barchart_to_tikz` / `lineplot_to_tikz` directly: series names,
  whiskers from min/max, refline, footnote text. No helper exists
  yet; callers hand-assemble the argument lists (see
  `examples/ikat-paper/build.py` for the current pattern to wrap).
- **M2.2 regeneration.** OPEN — the 4 main-paper plots
  (`paper/figs/fig-*.pdf`, today matplotlib) rebuilt through ikat;
  side-by-side visual diff against the matplotlib PDFs. Blocked
  only on M2.1 (needs the preset loader to feed the emitters).
- **M2.3 local pgfplots.** ✅ DONE 2026-10-06 — tlnet `pgfplots.tar.xz`
  extracted into the TinyTeX texmf tree (`tlmgr` GPG is broken on
  this mirror chain); `tex_requirements()` auto-detects TikZ/pgfplots
  needs from the woven body so preambles stay minimal.
- **M2.4 legend positioning.** ✅ DONE 2026-10-07 (`4c6c9f8`) —
  `plot::LegendPos` keyword on both emitters: `below` (default,
  a horizontal row under the axis that cannot cover data),
  `top-left` / `top-right` / `bottom-left` / `bottom-right`
  in-axis corners, `outside-right` beside the plot; unknown words
  are `ValueError`s naming the set. Threaded Rust core → PyO3
  (keyword-with-default, backward compatible) → MCP tools;
  `outside-right` compile-proven to PDF.
- **M2.5 emitter polish.** ✅ DONE 2026-10-07 — mermaid edge
  labels get `fill=white` knockout (diagonal edges no longer
  strike through `column`/`wide`); barchart x ticks rotate 45°;
  lineplot pins `xmin`/`xmax` to the data (no pre-first-point gap).
  All three are structural emitter changes, each with a Rust test.

*Acceptance:* `paper/figs/` producible with no matplotlib import
anywhere in the loop.

## Phase 3 — Figures advanced (all OPEN)

- **M3.1 standalone export.** `\documentclass[tikz]{standalone}`
  wrapper for precompiled, arXiv-safe figure PDFs. Entry point:
  new `ikat standalone` subcommand or `compile_standalone()`
  taking one fence's TikZ + auto-detected preamble.
- **M3.2 sequence subset**, **M3.3 state subset** behind the same
  `flowchart_to_tikz` entry point (separate grammars, shared emitter).
- **M3.4 layout.** Edge routing that avoids node interiors, subgraph
  cluster boxes, wider DAG support (today: layered trees/DAGs only).

*Acceptance:* the paper's TikZ diagrams (corpus: the four
`paper/figs/tikz/fig-*.tex` sources — branch-tree, ledger-flow,
read-questions, stratum) regenerate from their mermaid fences with
no hand-tuning.

## Phase 4 — Harden & release

- **M4.1 errors.** OPEN — line-numbered parse errors (statement
  echo + caret) instead of bare strings. Concrete scope: thread
  source line numbers from the md scanner through `convert()` so
  failures in mermaid fences (`flowchart_to_tikz`), `%% table`
  attrs, `plot_attrs` names, template tokens, and legend keywords
  report `file:line: message` with the offending line echoed.
  Today all of these return context-free `String`s.
- **M4.2 CLI.** ✅ DONE 2026-10-07 — `ikat build/weave/check/templates/template/flowchart/version`
  (`python/ikat/cli.py`, `ikat` console script): same operations as
  the API, `--spec` BuildSpec JSON, `--ensure-packages` gate.
- **M4.8 MCP server.** ✅ DONE 2026-10-07 — `python -m
  ikat.mcp_server` (FastMCP stdio, `ikat[mcp]` extra): nine tools
  mirroring the CLI/API map; `compile_pdf` deliberately excluded.
- **M4.2b CI compile check.** OPEN (documented-only: no Docker
  daemon on this machine) — Docker TeX Live job compiling the demo
  AND the showcase paper on every push (backend verification, not
  a user-facing backend). Concrete job: `texlive/texlive` image +
  maturin build, `ikat build examples/mini.md` and
  `examples/ikat-paper/build.py`, fail on any undefined citation.
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
- **M4.3 claim the names.** ✅ READY 2026-10-07 — both `ikat`
  names verified free; `cargo publish --dry-run` green; release
  wheel proven in a clean venv (9 heads + skeleton, weave ok);
  `v0.1.0` tagged. Ships with two commands once the tokens exist:
  `cargo login` + `cargo publish` (crates.io), `maturin publish`
  or `twine upload dist/*` (PyPI). Until then: install from the
  GitHub repo (`pip install git+https://github.com/opticsWolf/ikat`).
- **M4.4 self-hosting.** ✅ DONE 2026-10-07 — the paper builds with
  ikat itself (`ikat weave paper/paper-2026-10-05.md --spec
  paper/ikat-spec.json --bib paper/refs.bib`, byte-identical);
  `build-paper.py` retired (kept in git history, not in the tree); new
  `--bib` flag derives `bib_keys` from the `.bib` (no frozen
  derived data, no drift).
- **M4.9 tectonic engine (prototype).** ✅ DONE 2026-10-07 —
  `compile_pdf(engine="tectonic")` cascades embedded binding
  (`tectonic` cargo feature, off by default) → `TECTONIC_EXE`/PATH
  binary; `tex_for_tectonic` drops the inputenc line; woven doc
  with TikZ+table compiled to a content-verified PDF. Tectonic
  *crate* embedding stays OPEN (blocked on Windows C deps —
  pkg-config / system libs; the `tectonic` cargo feature does not
  link here, so embedding is a Linux-CI exercise verified under
  M4.2b); empty `bib_name` now emits no bibliography lines (tectonic auto-runs
  BibTeX and dies on empty `\bibliography{}`).

## Non-goals

Full mermaid coverage (deliberately a strict subset), WYSIWYG,
bibliography management (`.bib` stays the source of truth),
reference-manager integration.

## Ordering

Phases 1, 4 (core), 5, 6 done. Remaining order: M2.1 → M2.2 (the
preset loader feeds regeneration) → M3 (layout work needs real
diagrams to test against) → M5.4 (the parser switch needs the full
test corpus green) → M4.1 whenever (error paths are additive).
M4.3 can jump the queue any time — it costs nothing but tokens.
M4.2b unblocks tectonic-embedding verification.

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
- **M5.3 Markdown AST spike.** ✅ SPIKE DONE 2026-10-07 —
  pulldown-cmark 0.13.4 event-diff vs the hand scanner: all four
  required properties hold (verbatim info strings, identical
  headings, matching table shapes, aligned paras modulo the
  `%%` directive line, which stays pre-processing either way).
  Switch viable, deferred to its own golden-parity milestone.
- **M5.4 pulldown-cmark switch.** OPEN — rewrite `convert()` on
  the pulldown-cmark event stream. Acceptance: empty diff on the
  spike corpus AND the golden paper rebuilds byte-identical AND
  all 54 cargo + 43 pytest stay green. The hand scanner
  (`%%` directive pre-processing, `esc.rs` map) stays regardless:
  only the block splitter moves.

## Phase 6 — Showcase paper ✅ DONE 2026-10-07

`examples/ikat-paper/` builds a 6-page two-column paper about ikat
itself (`build.py` + `ikat.toml` + `skel.tex` + `refs.bib`): two
inline mermaid diagrams (wide pipeline, column float-resolution),
two pgfplots figures (LOC bars from `wc -l`, test-growth line from
`git grep` history — no invented numbers), four tables, citations
via BibTeX, math, unicode. It is the release demo AND the
constructive proof that every md feature composes in one document.
Side fixes landed here and covered by tests: mermaid `[scale=]`
attribute splice (Rust + Python), lineplot `+- (0,0)` zero-error
emission. The shipped `IEEEtran.bst` is CTAN v1.14 (a 404-page
copy found in the texmf tree was replaced and the file staged
locally with an HTML guard).

*Acceptance:* `build.py` runs green, PDF is 6 pages with 2
diagrams + 2 plots + 4 tables and 0 undefined citations.

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
