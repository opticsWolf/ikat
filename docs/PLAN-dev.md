# ikat development plan — dev branch

Base: `v0.2.1` (59 cargo + 45 pytest green, CI green, crates.io
shipped, PyPI pending publisher registration).
Source of truth for scope: `docs/ROADMAP.md` open items —
M2.1, M2.2, M3.1–M3.4, M4.1, M4.10, M5.4, tectonic crate embedding.
(M4.3 is closed by the 0.2.x releases; only PyPI publisher setup
remains, which is a settings page, not code.)

Version rule: **+0.0.1 per minor feature, +0.1.0 per completed
phase.** Every bump is its own commit: `cargo test` + `pytest`
green, `maturin develop` warning-free, demo + showcase still
build, versions in `Cargo.toml` / `pyproject.toml` /
`python/ikat/__init__.py` / `Cargo.lock` in sync.
`main` moves only by merging a finished phase.

Test-count convention: counts below are new tests per feature,
on top of the 59 + 45 base. Never restate totals in docs —
regenerate with `cargo test` and `pytest`.

---

## Phase A — Data presets (M2.1 + M2.2) → 0.3.0

Kill the hand-assembled argument lists in
`examples/ikat-paper/build.py`: benchmark JSON goes in,
`tikzpicture` comes out. The JSON shape is dictated by what
`build.py` already assembles by hand today (series names,
values, min/max whiskers, optional refline, footnote text) —
the loader wraps the existing pattern, it does not invent a
new schema and migrate callers to it.

### A.1 preset loader → 0.2.2

New file `src/preset.rs`:

- `#[derive(Deserialize)] struct BenchmarkJson` with fields
  `title: String`, `ylabel: String`, `group_labels: Vec<String>`,
  `series_names: Vec<String>`, `values/mins/maxs:
  Vec<Vec<f64>>`, `log_y: bool` (default false),
  `refline: Option<[f64; 4]>` as `[x0, x1, y, label]`,
  `footnote: Option<String>`, `kind: "bar" | "line"`, and for
  lines `xs: Vec<f64>`, `yss/errs: Vec<Vec<f64>>`.
- `parse_preset(src: &str) -> Result<PresetChart, String>`
  validates: rectangular series (every row matches
  `group_labels.len()`), mins ≤ values ≤ maxs elementwise,
  non-empty names, `xs` strictly increasing for lines.
  Every violation names the file and the offending key —
  `preset results/foo.json: series 'a' row 2 has 3 values,
  expected 4 groups` — never a bare serde error.
- `PresetChart` converts into the existing `BarchartArgs` /
  `LineplotArgs` (no emitter changes in A.1; the emitters are
  already proven).
- PyO3: `preset_to_tikz(json_src: &str) -> Result<String,
  String>` in `lib.rs`, dispatching on `kind`.
- Python `python/ikat/preset.py`: `load_preset(path_or_src) ->
  str` thin wrapper (reads a file path or raw JSON string;
  the Rust side never touches the filesystem — same boundary
  rule as everywhere else).
- Tests: 6 new Rust unit tests (ragged series, mins>maxs,
  missing key, bad kind, empty names, valid bar + valid line
  golden strings); `tests/test_preset.py`: 4 tests (file vs
  string input, error passthrough, parity with today's
  hand-assembled `build.py` output — the same JSON that
  `build.py` assembles must emit the same TikZ through the
  loader; write that JSON down as `examples/ikat-paper/fig-loc.json`
  and `fig-tests.json`).

*Acceptance:* `preset_to_tikz` output for both showcase
figures is byte-identical to the current hand-assembled
emitter output.

### A.2 surface wiring → 0.2.3

- CLI (`python/ikat/cli.py`): `barchart --preset FILE|-` and
  `lineplot --preset FILE|-`; `--preset -` reads stdin (same
  convention as the JSON payload); `--legend` still overrides
  the payload's legend key; `--preset` and inline JSON payload
  together are an error naming both flags.
- MCP (`python/ikat/mcp_server.py`): `preset_json: str = ""`
  params on `barchart_to_tikz` / `lineplot_to_tikz`; tool
  descriptions updated.
- Docs: QUICKREF preset row + payload-shape note, README CLI
  example gains a `--preset` line, ARCHITECTURE §3.2 gets a
  preset paragraph.
- Tests: `tests/test_surface.py` +4 (CLI preset file, CLI
  preset stdin, CLI preset+payload conflict, MCP preset
  param); the surface test's pinned tool list is unchanged
  (no new tools, only new params).

### A.3 regeneration harness → 0.2.4

New dir `examples/regen/`:

- `regen.py`: for each of the 4 main-paper plots, loads a
  checked-in JSON fixture (`fig-*.json`, transcribed once from
  the matplotlib data sources — `benchmarks/plot_paper_figs.py`
  — with the transcription reviewed and committed), runs it
  through `preset_to_tikz`, stages the TikZ next to the
  matplotlib PDF.
- `diff.sh` / `diff.py`: renders both the matplotlib PDF page
  and the ikat-compiled PDF page to PNG (PyMuPDF, already a
  dependency of the diagram tooling) and reports a pixel-diff
  score; scores are recorded in `regen/ACCEPTED.md` with the
  review date — anything above threshold fails the script.
  First run establishes the baseline (human-reviewed once);
  after that the scores are pinned and the script is a
  regression gate.
- `paper/figs/` build path switches to the loader (matplotlib
  scripts stay in git history for provenance but leave the
  loop — nothing imports `matplotlib` from any build path).
- Tests: `tests/test_preset.py` +2 (all four fixtures parse;
  regen script exits 0 on the pinned baselines).

*Acceptance (M2 gate, verbatim):* `paper/figs/` producible with
no matplotlib import anywhere in the loop.

### Phase A gate → 0.3.0

Full suite green, showcase + regen build on CI TeX Live (the
`tex` job gains a regen step: fixtures → TikZ → compile →
pixel-gate), ROADMAP M2.1/M2.2 marked DONE, ARCHITECTURE §3.2
preset paragraph landed, merge to `main`.

---

## Phase B — Standalone figures (M3.1) → 0.4.0

Precompiled, arXiv-safe figure PDFs from single fences — the
missing piece between "ikat emits TikZ" and "journals accept
the PDF". Reuses the preamble auto-detection instead of
inventing a second one.

### B.1 standalone emitter → 0.3.1

- Rust: expose the existing `tex_requirements` scan over PyO3
  (`tikz_needed(tex) -> bool`, `plots_needed(tex) -> bool` —
  today Python cannot ask; B.1 needs the answer to build the
  minimal standalone preamble). No new detection logic.
- Python `python/ikat/standalone.py`:
  `compile_standalone(tikz_src, workdir, engine="pdflatex") ->
  Path` — wraps one fence's TikZ in
  `\documentclass[tikz]{standalone}` + detected
  `usepackage{tikz}` / `usepackage{pgfplots}` lines only,
  writes `fig.tex`, calls the existing `compile_pdf`, returns
  the PDF path. No bibliography, no floats, no caption — a
  standalone figure is a picture, not a float.
- CLI: `ikat standalone FENCE.md --out fig.pdf [--engine …]`
  (extracts the first mermaid/chart fence via `extract_fences`;
  more than one fence is an error naming the count — explicit
  is better than "first wins" silently).
- MCP: `standalone_figure(md_text)` tool returning the PDF
  bytes? No — workdir-bound binaries stay out of MCP (same rule
  as `compile_pdf`): the tool returns the wrapped `fig.tex`
  source and the caller compiles. Document the rule in the
  tool description so agents don't file it as a bug.
- Proofs: all four `paper/figs/tikz/fig-*.tex` sources
  regenerate from their mermaid fences with no hand-tuning and
  compile to PDFs pixel-matching (modulo timestamps) the staged
  ones — recorded in `examples/regen/ACCEPTED.md` alongside A.3.
- Tests: pytest +5 (wrap minimal-preamble content for TikZ-only
  vs pgfplots bodies, multi-fence error, CLI end-to-end to
  real PDF via TinyTeX, MCP returns source not bytes, engine
  passthrough to tectonic).

### Phase B gate → 0.4.0

Standalone proofs added to the CI `tex` job (all four paper
figures compile standalone on runner TeX Live), QUICKREF +
README standalone rows, ROADMAP M3.1 DONE, merge to `main`.

---

## Phase C — New grammars (M3.2 + M3.3) → 0.5.0

Sequence and state diagrams behind the same `flowchart_to_tikz`
entry point: separate strict grammars, shared TikZ emitter.
Dispatch on the diagram header line (`sequenceDiagram` /
`stateDiagram-v2`, mermaid's own keywords — no new syntax to
learn, no new fence type).

### C.1 sequence subset → 0.4.1

New file `src/sequence.rs`:

- Grammar: `participant NAME [as Label]` / `actor NAME`
  declarations, `A->>B: text` solid and `A-->>B: text` dashed
  messages, `alt text` / `else` / `opt text` / `end` boxes.
  Participant order is declaration order (deterministic —
  byte parity holds). Message labels get the same
  `fill=white` knockout as flowchart edge labels.
- Everything else (loops, par, rect, autonumber, styling) is a
  string error naming the statement — the strict-subset
  doctrine applies unchanged.
- Refactor note: lift the TikZ label-knockout and node-box
  helpers out of `mermaid.rs` into a shared `src/emit.rs`
  first (mechanical move, golden paper proves the move is
  byte-neutral), then write the sequence emitter against it.
- Proof figure: the ikat weave pipeline as a sequence diagram
  (caller → Python → Rust → TeX → PDF), added to the showcase
  paper as a third diagram (counts in the showcase README and
  the paper abstract move 2→3 together — no silent drift).
- Tests: Rust +7 (column layout math, alt-box spanning, error
  on `loop`, error on missing `end`, knockout on labels,
  declaration-order determinism, dashed vs solid); pytest +2
  (CLI/MCP parity through the shared entry point).

### C.2 state subset → 0.4.2

New file `src/state.rs`:

- Grammar: `[*] --> Name` start, `Name --> [*]` end, `A -->
  B : label` transitions, `state "Label" as Name` long form,
  one level of `state Name { … }` composite box. Nesting a
  composite inside a composite is a build error ("nested
  composites are not supported", not a silent flat render).
- Layout reuses the layered BFS from `mermaid.rs` (states are
  nodes, transitions are edges) with the composite box drawn
  around its members post-layout — layout first, boxes second,
  so boxes can never distort coordinates.
- Proof figure: the document-path state machine
  (fences → convert → wrap → validate → compile), added to the
  showcase paper (diagram count 3→4, same coupled-count rule).
- Tests: Rust +7 (start/end rendering, composite bounding math,
  nesting error, long-form labels, transition labels, layout
  determinism, unknown statement error); pytest +2.

### Phase C gate → 0.5.0

Both grammars in CLI/MCP/QUICKREF/README (subset tables for
each, mirroring the flowchart table), ARCHITECTURE §3.1 gains
the dispatch + `emit.rs` paragraph, ROADMAP M3.2/M3.3 DONE,
merge to `main`.

---

## Phase D — Layout engine (M3.4) → 0.6.0

Edge routing that avoids node interiors, subgraph cluster
boxes, wider DAG support. Builds on the legend resolver's
geometry muscle (segment intersection is already implemented
and tested — reuse it, don't rewrite it).

### D.1 routing + clusters → 0.5.1

- New file `src/layout.rs`: `segs_cross` and `seg_hits_rect`
  move here from `plot.rs` (same mechanical-move-then-prove
  pattern as `emit.rs`: move, run the legend tests, confirm
  green, then build on it).
- Edge router: after layered placement, any edge whose segment
  intersects a non-endpoint node box reroutes via a midpoint
  offset perpendicular to the segment, pushed until the
  two-segment path clears all boxes (bounded iterations —
  exhaustion is a build error naming the edge, not an infinite
  loop and not a line through a node).
- Subgraph support in `mermaid.rs`: accept `subgraph ID [title]`
  … `end` blocks (today a parse error); the box is drawn around
  laid-out members post-layout (same layout-first rule as state
  composites). Nested subgraphs: error, same rationale.
- DAG widening: the layered placer already handles DAGs; D.1
  extends the layer assignment past trees (multi-parent nodes
  take max(parent depth)+1 — document the rule in the subset
  table) and proves it with a diamond-heavy corpus diagram.
- *Acceptance:* the four paper TikZ diagrams regenerate with no
  manual coordinate tweaks; golden paper still byte-identical
  (routing changes nothing when no edge crosses a node — the
  common case must be provably untouched, asserted by the
  golden test itself).
- Tests: Rust +9 (segment helpers moved green, midpoint push
  direction, iteration bound, subgraph box math, nesting
  error, diamond layering, no-op routing on clean graphs,
  determinism across runs, wide-graph layer bound); pytest +2.

### Phase D gate → 0.6.0

ARCHITECTURE §3.1 layout paragraph, subset table updated
(subgraph rule, multi-parent rule), ROADMAP M3.4 DONE —
**Phase 3 closes**, merge to `main`.

---

## Phase E — Parser switch (M5.4) → 0.7.0

Rewrite the block splitter on the pulldown-cmark event stream
per the M5.3 spike verdict. The spike proved all four required
properties; this phase spends that proof.

### E.1 pulldown-cmark `convert()` → 0.6.1

- Cargo: `pulldown-cmark` moves from dev-spike to
  `[dependencies]` (pin the 0.13.x series; version bumps of the
  parser are their own commits with a golden re-run).
- `src/doc.rs`: `convert()` consumes the event stream;
  `%%`-directive pre-processing and the `esc.rs` map stay
  exactly where they are (the spike's core finding: only the
  splitter moves). Fence info strings, headings, tables, and
  paragraphs map per the spike's alignment table in
  `src/md_spike.rs` — that file becomes the regression corpus,
  not dead code: keep it running as a cross-check test.
- Procedure (order matters): (1) run golden + spike on the
  current tree, record hashes; (2) rewrite behind a
  `#[cfg(feature = "pdc")]`-style gate? No — no feature flag,
  a flag would double the test matrix; rewrite in place on
  `dev`, golden test runs on every commit; (3) any red golden
  is fixed in the rewrite, never by editing the golden file.
- *Acceptance (all three, no exceptions):* empty diff on the
  spike corpus AND golden paper byte-identical AND full suite
  green. **If the golden moves by one byte, the switch is
  reverted and this phase re-plans from the diff** — the
  acceptance is written as a revert trigger, not an aspiration.
- Tests: spike corpus kept green throughout (+0 new tests
  needed — the corpus IS the test; add +3 for `%%`-edge cases
  the rewrite newly exposes, if any).

### Phase E gate → 0.7.0

`md_spike.rs` documented as the permanent regression corpus,
ARCHITECTURE §3.5 updated (spike → switch, how the mapping
works), ROADMAP M5.4 DONE, merge to `main`.

---

## Phase F — Line-numbered errors (M4.1) → 0.8.0

Additive by design (error paths only — no golden output can
change, since errors never appeared in output). Rides last
among code phases but can jump the queue any time without
conflicts.

### F.1 threaded line numbers → 0.7.1

- New file `src/error.rs`: `Error { line: usize, col:
  Option<usize>, echo: String, msg: String }` with a `Display`
  rendering `msg` + ` --> line N` + echo. Replace every
  `Result<_, String>` in the fail-fast sites (ARCHITECTURE §7
  is the checklist: mermaid statements, edge targets, empty
  diagrams, ragged series, dangling keys, template/skeleton
  tokens, wide+here floats, unknown attrs/keys/legend words)
  with `Result<_, Error>`, threading the scanner's line numbers
  through `convert()`, `flowchart_to_tikz`, the plot emitters,
  and the attr parsers.
- PyO3 boundary: `Error`'s Display string crosses (Python
  raises it as today — exception type unchanged, message
  format upgraded); Python prepends the filename (`file:line:
  message`) since Rust never sees paths (filesystem rule).
- CLI prints the multi-line rendering verbatim; MCP returns it
  as the error text.
- Tests: one per catalog site (+12 Rust, +4 pytest) asserting
  the `file:line:` format with `file:line:` matched by regex
  and the echo line containing the offending source text.

### F.2 caret rendering → 0.7.2

- Where the grammar knows the token span (mermaid statement
  keyword, attr keys, legend word, template token braces),
  `col` is `Some` and the renderer adds a `^` caret line
  (spaces to the column, one caret per offending token —
  no multi-caret spans, keep it simple).
- Where it doesn't (ragged series counts, dangling keys),
  `col` stays `None` and the echo line alone carries the
  context — absence of a caret is never a failure, only a
  missing precision.
- Tests: +6 asserting caret column arithmetic on indented
  sources (the caret must account for leading whitespace —
  off-by-indent is the classic bug here; test it directly).

### Phase F gate → 0.8.0

ARCHITECTURE §7 updated (M4.1 gap closed — the section that
names this as "the known gap" gets rewritten, not appended
to), QUICKREF error-format row, ROADMAP M4.1 DONE, merge to
`main`.

---

## Phase G — Engine hardening (M4.10) → 0.9.0

No new machinery — CI steps plus verification runs. Smallest
phase; scheduled last because it verifies everything above.

### G.1 CI tectonic step → 0.8.1

- `.github/workflows/ci.yml` `tex` job: install the pinned
  0.17.0 binary (GitHub release download, not `cargo install`
  — CI minutes matter; checksum-pinned), build demo +
  showcase with `--engine tectonic`, same undefined-citation
  gate. This requires `examples/ikat-paper/build.py` to accept
  an `--engine` passthrough (add it: `--engine` flag defaulting
  to `pdflatex`, forwarded to `compile_pdf`).
- Tests: none new in-repo (CI is the test); the job itself
  must go red-then-green once to prove the gate works
  (temporarily break a citation in a scratch run, or trust
  the pdflatex gate's history — record which in the commit
  message).

### G.2 paper verification → 0.8.2

- Showcase paper AND main paper compile under tectonic;
  XeTeX-vs-pdfTeX deltas recorded in `examples/regen/ACCEPTED.md`
  (same ledger as the plot diffs — one acceptance ledger, not
  scattered notes). Any delta that changes content (not just
  metrics) is a bug filed against the emitter, not accepted.
- README LaTeX-requirements section documents the
  Windows-embedding limitation (crate doesn't link: C deps;
  binary cascade is the supported path). Crate embedding
  itself stays a Linux-CI exercise — explicitly out of scope,
  recorded in ROADMAP so it stops being re-proposed.
- Tests: pytest +1 (tectonic engine passthrough in
  `compile_pdf` — the cascade order binding → env → PATH is
  already tested; this pins the showcase flag plumbing).

### Phase G gate → 0.9.0

ROADMAP M4.10 DONE, ARCHITECTURE §3.5 updated (prototype →
  hardened, what the CI step covers), merge to `main`.

---

## Standing rules (all phases)

1. Golden paper never moves without intent; byte parity is the
   tripwire for every phase.
2. Strict-subset doctrine: new grammars fail loudly, never
   almost-render.
3. No invented numbers: every figure in proofs/showcase traces
   to measurement or is marked `[NOT YET MEASURED]`.
4. Surface parity: Rust feature → Python API → CLI → MCP →
   QUICKREF, each step tested (surface test pins the tool list).
5. `dev` is integration; `main` only takes finished phases
   (squash or fast-forward at phase gates — maintainer's call
   at 0.3.0).
6. One acceptance ledger: `examples/regen/ACCEPTED.md` records
   every pinned diff/score/delta with review dates. No
   drive-by numbers anywhere else.
