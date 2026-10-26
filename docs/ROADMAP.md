# ikat roadmap

Status 2026-10-26 (M7.2 top-fit, 0.10.2): Phases 1, 2 (plots),
3 (standalone, sequence, state), 4 (core), 5, 6, D (layout +
heading guards), E (parser switch), F (line-numbered errors),
G (tectonic hardening), H (auto-layout) done — 141 cargo + 88
pytest green (84 passed, 4 skipped),
showcase (10 pages, 4 diagrams) + regen + standalone + heads
proofs build under pdflatex AND tectonic, golden paper
byte-identical (`ebe698db`), 0 undefined citations both
engines. Open: tectonic crate embedding (Linux-CI exercise,
by design — not a milestone).

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
- **M4.10 tectonic hardening.** ✅ DONE 2026-10-08 (Phase G,
  0.8.1 → gate 0.9.0) — (1) CI `tex` job installs the pinned
  0.17.0 binary (checksum-verified linux-gnu tarball) and builds
  demo + showcase with `--engine tectonic` (new flag on both
  builders, default pdflatex); (2) showcase (10 pp) AND main
  paper (22 pp) verified under tectonic, 0 undefined citations
  both engines, engine deltas (all font-encoding class: TU/OTF
  vs OT1/Type1 extraction, quote glyphs, hyphenation points)
  ledgered in `ACCEPTED.md` — no content delta, nothing filed;
  (3) Windows-embedding limitation in README (binary cascade is
  the supported path; embedding stays Linux-CI by design).
  Found in passing: tectonic swallows the XeTeX log without
  `--keep-logs` (undefined citations would pass silently) —
  `_compile_tectonic` now passes it and writes the transcript
  beside the kept log; the gate grep covers both. Red-then-green
  proven locally with the exact gate patterns (broken citation
  → 2 matches; clean → 0).

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

## Phase H — Auto-layout (M7.1) → 0.10.0

Advisory auto-layouter: `ikat layout` measures every float and
suggests `span`/`pos`/`width` with reasons; `--apply` rewrites
fence attrs. No pagination simulation (only LaTeX knows text
flow) — classification by measured size, LaTeX still places.
Estimators: TikZ coord scan + `node_box` margins (diagrams),
pgfplots defaults (plots), row-pitch + cell widths (tables);
pictures/external PDFs keep hand attrs. Policy: `page_frac`
pre-empt, section-lead bonus, wide ratio, crowding floor from
placed node labels. Surfaces in the same commits: Rust core +
`layout.py` (`suggest`/`apply`/`report`) + `ikat layout` CLI +
` suggest_layout` MCP tool + QUICKREF/skills/ARCHITECTURE.
Co-agent B1 bench (`examples/bench-layout/`, untracked scratch)
drives `suggest_layout` over 10 PLOS papers: v2 distributions
reproduce exactly under the final engine; showcase engine-vs-hand
diff is 2 hunks (Fig 2 `0.57` vs hand `0.58`, Table 3 `force` vs
`barrier`). E1 closed as expected (PLOS is single-column; the
1-col run puts everything in-flow).

*Acceptance (M7 gate):* `cargo test` + `pytest` green
(135 + 86), demo + showcase + regen + standalone + heads proofs
build, golden byte-identical, auto-applied showcase copy builds
8pp with 0 overfull and the hand float distribution.

## M7.7 — Honest portrait limits (Tiny rung + pin height) → 0.10.7

`Tiny`: a fourth ladder rung — `\scriptsize` + 2pt padding but
still portrait — for dense regression tables between `Compact`
and `Rotate` (relative type size matches the originals' 7.5pt on
their wider measure). A narrow-aware token ruler (digits/punct at
0.85) replaces the flat 0.16/char. A hyphenation discount was tried
and reverted the same session: isolation renders proved TeX never
breaks the words (IEEEtran sets `\hyphenpenalty=750` — measured
750 in the class), and the discount printed 16 overlapping tables;
every whitespace run binds.
Pin-height promotion: stacked rows double a table's height, so a
column table taller than the in-flow budget now floats wide + top
instead of overflowing (`\vbox too high`) — the width ladder
approves the column, the height vetoes the pin. Tail threshold
recalibrated 10 → 6 paragraphs (a column of downstream text; the
paragraph unit had made 10 three times stricter than the JATS-line
calibration) and the exporter's tail counter counts paragraphs,
fixing chem Fig 5's lone float page (10pp → 8pp). Corpus: econ
0 overfulls with 8/10 tables portrait; the two true 13-col tables stay
rotated — `globalization`-class words need 2.08cm against a
1.37cm share, mathematically unportraitable at IEEE widths (the
originals need 19cm + 7.5pt for the same).

## M7.6 — Cell stacking (PLOS-original style) → 0.10.6

Stacks estimate/SE cells before deciding: `0.080***(0.015)`
renders as estimate over `(0.015)` (`\newline` in the `X` cell,
via a split mark that survives `inline()` escaping). Only
trailing digit-bearing paren groups split — stars stay with the
estimate, empty heads and prose notes stay single-line. The
shared `decide_table` (weaver + engine notes) stacks first, then
re-decides: econ's 7-col tables drop from `Compact` to `Fit` at
full size, the 13-col tables rotate roomy. Result: econ 25 → **0**
overfulls (was 441), corpus total 441 → 47 with all remainders
in prose (flattened formulas) or ≤3pt. Prose labels never trigger
(they wrap); the knob stays default-off so the frozen paper and
all default weaves hold byte-identical.

## M7.5 — Table compaction ladder (opt-in) → 0.10.5

`[tables] compact_tables` (default off, so the frozen paper and
all default weaves stay byte-identical): the widest unbreakable
token per table drives `Fit` (`\small`) → `Compact`
(`\footnotesize` + 3pt padding) → `Rotate` (`sidewaystable(*)` in
`\scriptsize` + 2pt padding against `\textheight`). Column
tables that fail promote to `wide` before rotating. Prose labels
never trigger it (they wrap); only unbreakable numeric cells do.
Corpus proof: econ-tax 441 → 25 overfulls (max 2.9pt, invisible),
the 13-column tables rotate readably; psych 200 → 19, bio 38 → 7,
earth 17 → 1. The engine notes always measure the shared ladder
and name the opt-in when it is off; `rotating` joins the strict
package contract (auto-added to generated heads, named for
skeletons/overrides).

## M7.4 — Tail page + no empty columns → 0.10.4

`[layout] tail_lines` (default 10): a `wide`+`top` float with
fewer body prose lines remaining takes a dedicated `page`. Two
birds: no downstream tops are left (it would pile at the document
end), and a still-pending float meeting `\FloatBarrier` ejects
the column — placeins issues `\clearpage` in the first column,
stranding a half-empty column behind it. That eject was the
corpus's void source: section barriers (since reverted for the
corpus) met pending wides and blanked ~17 right columns
(chem p2, bio p17/p22/p33). Float pages never strand columns:
text flows past them, and the refs barrier meets empty defer
lists. Skipped in prose-free manuscripts so B1 stub pins stay
exact. Corpus result: 0 true mid-body voids across all 10 papers
(the 3 remaining flags are final-text-column end geometry + one
normal column break); nothing crosses `References`.

## M7.3 — Refs barrier (opt-in) → 0.10.3

`[floats] barrier_before_refs` (default off, so default weaves
stay byte-identical): a `\FloatBarrier` ahead of
`\bibliography` in all three preamble paths. End-queued floats
otherwise spill into/past `References` — bio-medsea's Figs 7-14
landed on pp37-42 after the reference list started (p27) because
10 wide figure tops saturated and no barrier contained them. With
`barrier_sections` + refs barrier (corpus weave policy), bio's
tables sit inside Discussion beside their citing prose (TABLE IV
under the `(Table 4)` paragraph, p26) and nothing crosses
`References`. Proven over the full 10-paper re-export, 0
unmatched, 0 stranded headings; golden byte-parity holds
(default off).

## M7.2 — Top-fit + queue pressure → 0.10.2

Wide floats taller than the page-top budget (`top_frac` 0.7 ×
text height, the LaTeX `topfraction`/`dbltopfraction` default)
take `wide`+`page` instead of `wide`+`top`: `[t]` cannot legally
hold them, so they (and the queue behind them) pile at the
document end. Column tables taller than the text height go
`page` (`[H]` would blast through). The third and later `wide`
floats since a section heading earn a spread-the-fences note.
Proven on the full-export corpus: psych-maia's 41-row table left
the p24–26 end pile for an in-section float page (9 tables now
p9–p21 beside their prose, 26pp→25pp), 0 stranded headings
across all 10 papers (D.2 needspace guards hold). Exporter
`pic_attrs` mirrors the budget for precompiled pictures.

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
