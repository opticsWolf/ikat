# ikat roadmap

Status 2026-10-08 (Phase F gate, 0.8.0): Phases 1, 2 (plots),
3 (standalone, sequence, state), 4 (core), 5, 6, D (layout +
heading guards), E (parser switch), F (line-numbered errors)
done — 118 cargo + 75 pytest green, showcase (10 pages,
4 diagrams) + regen + standalone + heads proofs build, golden
paper byte-identical (`ebe698db`), 0 undefined citations.
Open: M4.10, tectonic crate embedding.

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

- **M2.1 presets.** ✅ DONE 2026-10-08 (Phase A.1+A.2, 0.2.2+0.2.3)
  — `src/preset.rs` + `load_preset`: benchmark JSON → emitters
  (series names, min/max-validated whiskers, refline, footnote,
  legend key); CLI `--preset` + MCP `preset_json` wired with the
  surfaces synchronized in the same commits; skills updated.
- **M2.2 regeneration.** ✅ DONE 2026-10-08 (Phase A.3, 0.2.4)
  — `examples/regen/`: 4 fixtures transcribed from
  `benchmarks/results`, TikZ snapshots (CI byte-gate), standalone
  PDFs, pixel scores pinned in `ACCEPTED.md` with reviewed deltas
  (degenerate Fig-A whiskers, symmetric max-errors, linear Fig-D
  x, below-legend/45° touch, refline edge clip). Outputs stay in
  `regen/out/` — staging into `paper/figs/` is the paper track's
  call.
- **M2.3 local pgfplots.** ✅ DONE 2026-10-06 — tlnet `pgfplots.tar.xz`
  extracted into the TinyTeX texmf tree (`tlmgr` GPG is broken on
  this mirror chain); `tex_requirements()` auto-detects TikZ/pgfplots
  needs from the woven body so preambles stay minimal.
- **M2.4 legend positioning.** ✅ DONE 2026-10-07 (keyword;
  `auto` resolver follows) — `plot::LegendPos`: `auto` (the
  default) tries the inside corners in top-left, top-right,
  bottom-right, bottom-left order, testing each against the drawn
  data — polyline + error-bar segments for lines, whole-slot bar
  rects to the whisker top (log-mapped for log-y) plus the refline
  — on visual ranges that overestimate pgfplots' padding, and falls
  back to the under-axis row when every corner is occupied.
  Inside corners cannot touch axis labels (labels live outside the
  axis box); the below row clears tick labels by construction.
  Explicit `below`, four corners, and `outside-right` place it by
  hand; unknown words are `ValueError`s naming the set. Threaded
  Rust core → PyO3 (keyword-with-default, backward compatible) →
  MCP tools; showcase figs prove it (LOC → top-right, growth →
  top-left).
- **M2.5 emitter polish.** ✅ DONE 2026-10-07 — mermaid edge
  labels get `fill=white` knockout (diagonal edges no longer
  strike through `column`/`wide`); barchart x ticks rotate 45°;
  lineplot pins `xmin`/`xmax` to the data (no pre-first-point gap).
  All three are structural emitter changes, each with a Rust test.

*Acceptance:* `paper/figs/` producible with no matplotlib import
anywhere in the loop.

## Phase 3 — Figures advanced (all OPEN)

- **M3.1 standalone export.** ✅ DONE 2026-10-08 (Phase B,
  0.3.1+0.4.0) — `python/ikat/standalone.py` (`wrap_standalone`
  + `compile_standalone`), `ikat standalone` (exactly-one-fence
  rule), MCP `standalone_figure` (returns source, workdir rule),
  `tikz_needed`/`plots_needed` predicates, four paper-diagram
  proofs with ledger scores in `examples/regen/ACCEPTED.md`.
- **M3.2 sequence subset**, **M3.3 state subset** ✅ DONE
  2026-10-08 (Phase C, 0.4.1+0.4.2+0.5.0) — `src/sequence.rs`
  (columns in declaration order, alt/else/opt boxes) +
  `src/state.rs` (shared placer, composite boxes) behind the same
  `flowchart_to_tikz` entry; `emit.rs` holds the shared TikZ
  vocabulary; showcase Figures 2+3 are the proofs.
- **M3.4 layout.** ✅ DONE 2026-10-08 (Phase D.1, 0.5.1) —
  `src/layout.rs` (legend geometry reused: `segs_cross`,
  `seg_hits_rect`, `cluster_box`), longest-path layering
  (multi-parent = max+1), `subgraph ID [title]`…`end` clusters
  (nesting errors), edge router (clear segments byte-identical,
  crossings reroute via pushed midpoint, exhaustion names the
  edge); showcase Figure 1 re-laid and reviewed.
- **D.2 heading guards.** ✅ DONE 2026-10-08 (0.5.2) —
  `[typography]` (`keep_with_next = true` default, `min_lines =
  2`): `\clubpenalty`/`\widowpenalty` 10000 +
  `\needspace{min_lines+1\baselineskip}` before sections;
  needspace auto-added (generated/override/skeleton paths);
  `titlesec` under IEEE rejected by `check_titlesec` +
  `ikat check`; main paper `.tex` regenerated (61 added guard
  lines, 0 removed) and recompiled clean.

*Acceptance:* the paper's TikZ diagrams (corpus: the four
`paper/figs/tikz/fig-*.tex` sources — branch-tree, ledger-flow,
read-questions, stratum) regenerate from their mermaid fences with
no hand-tuning.

## Phase 4 — Harden & release

- **M4.1 errors.** ✅ DONE 2026-10-08 (Phase F, 0.7.1 → gate
  0.8.0) — `src/error.rs::Error { msg, line, col, echo }` at
every fail-fast site (61 `Err`s audited: mermaid/sequence/state
statements + edge targets + empty diagrams, router exhaustion,
subgraph/composite errors, ragged plot/preset series, dangling
keys, template/skeleton tokens, titlesec, float attrs, legend
words, refdefs). Manuscript lines via `md.rs` original-number
blocks (fence bodies `line + row`), head-file lines for
templates, line 0 (no line part) for JSON/TOML data errors;
carets where the grammar knows the token (char columns,
indent-exact). Python `document.with_file` upgrades to
`file:line:` (head-path routing for template errors); CLI
prints verbatim; MCP returns the text. 118 cargo + 75 pytest
(16 + 6 new error tests); golden untouched (`ebe698db`).
- **M4.2 CLI.** ✅ DONE 2026-10-07 — `ikat build/weave/check/templates/template/flowchart/version`
  (`python/ikat/cli.py`, `ikat` console script): same operations as
  the API, `--spec` BuildSpec JSON, `--ensure-packages` gate.
- **M4.8 MCP server.** ✅ DONE 2026-10-07 — `python -m
  ikat.mcp_server` (FastMCP stdio, `ikat[mcp]` extra): nine tools
  mirroring the CLI/API map; `compile_pdf` deliberately excluded.
- **M4.2b CI compile check.** ✅ DONE — `.github/workflows/ci.yml`:
  `test` runs cargo + pytest on ubuntu/windows/macos-latest;
  `tex` (ubuntu) installs real TeX Live via apt, builds the demo
  and the showcase paper, and fails on undefined citations or
  control sequences. (Docker image dropped: runner TeX Live is
  the same backend verification with less machinery; the tectonic
  crate-embedding check rides along once M4.2b exists.)
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
  What M4.9 did NOT cover is itemized in M4.10.
- **M4.10 tectonic hardening.** OPEN — three items: (1) a
  tectonic step in the CI `tex` job (install the 0.17.0 binary,
  build demo + showcase with `--engine tectonic`, same
  undefined-citation gate); (2) showcase paper and main paper
  verified under tectonic (only a probe doc is proven; XeTeX
  font handling differs from pdfTeX); (3) the Windows-embedding
  limitation documented in README's LaTeX section.

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
M4.10 rides along with any CI work (it is a `tex`-job step plus
verification runs, no new machinery).

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
- **M5.4 pulldown-cmark switch.** ✅ DONE 2026-10-08 (Phase E,
  0.6.1 → gate 0.7.0) — `src/md.rs` production mapping +
  `convert()` on `MdBlock`s. Acceptance met: empty diff on the
  harness (141+35+3 corpus blocks + latent synthetics), golden
  paper byte-identical (`ebe698db`, 0 edits to the golden file),
  98 cargo + 69 pytest green, all proofs build. One consumption
  bug found mid-switch (blank-separated Paras merged — `convert`
  now flushes between consecutive `Para` blocks); one orphaned
  `#[cfg(test)]` on `mod mermaid` found via a failed
  `maturin develop` (probe-cleanup scar, fixed). `esc.rs`
  untouched, as the spike predicted.

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
