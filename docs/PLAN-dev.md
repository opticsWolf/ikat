# ikat development plan — dev branch

Base: `v0.2.1` (59 cargo + 45 pytest green, CI green, crates.io
shipped, PyPI pending publisher registration).
Source of truth for scope: `docs/ROADMAP.md` open items —
M2.1, M2.2, M3.1–M3.4, M4.1, M4.10, M5.4, tectonic crate embedding.
(M4.3 is closed by the 0.2.x releases; only PyPI publisher setup
remains, which is a settings page, not code.)

Version rule: **+0.0.1 per minor feature, +0.1.0 per completed
phase.** Every bump is its own commit (`cargo test` + `pytest`
green, demo + showcase still build, versions in
`Cargo.toml`/`pyproject.toml`/`__init__.py`/`Cargo.lock` in sync).
`main` moves only by merging a finished phase.

## Phase A — Data presets (M2.1 + M2.2) → 0.3.0

Kill the hand-assembled argument lists in `ikat-paper/build.py`:
benchmark JSON goes in, `tikzpicture` comes out.

- **A.1 preset loader → 0.2.2.** Rust `src/preset.rs`:
  `results/*.json` shape (`series_names`, per-series values +
  min/max, optional `refline`, `footnote`) → validated
  `BarchartArgs`/`LineplotArgs` structs; malformed JSON and ragged
  series are string errors naming the file. PyO3 `preset_to_chart`
  binding + `python/ikat/preset.py` thin wrapper.
  *Acceptance:* the ikat-paper LOC/growth JSON round-trips through
  the loader with zero hand assembly; Rust unit tests for ragged/
  missing-key inputs.
- **A.2 surface wiring → 0.2.3.** CLI `ikat barchart --preset F`
  / `lineplot --preset F` (file or stdin, `--legend` still
  overrides), MCP `preset` params on both chart tools, QUICKREF +
  README entries. Surface test extended (CLI×MCP parity holds).
- **A.3 regeneration harness → 0.2.4.** `examples/regen/`: the 4
  main-paper plots rebuilt through ikat from checked-in JSON +
  a side-by-side diff script (visual + `diff` on the emitted
  TikZ, recorded deltas reviewed once, then pinned).
  *Acceptance:* `paper/figs/` producible with no matplotlib
  import anywhere in the loop (M2 acceptance gate, verbatim).
- **Phase gate → 0.3.0.** Full suite green, showcase + regen
  build on CI TeX Live, ROADMAP M2.1/M2.2 marked DONE, merge to
  `main`.

## Phase B — Standalone figures (M3.1) → 0.4.0

Precompiled, arXiv-safe figure PDFs from single fences.

- **B.1 standalone emitter → 0.3.1.** `compile_standalone(tikz,
  needs)` in Python + Rust preamble detection reuse
  (`tex_requirements`): wraps one fence's TikZ in
  `\documentclass[tikz]{standalone}` with the minimal detected
  preamble, compiles, returns the PDF path. CLI `ikat standalone
  FENCE.md --out fig.pdf`; MCP tool to match.
  *Acceptance:* all four `paper/figs/tikz/fig-*.tex` sources
  regenerate from their mermaid fences with no hand-tuning, and
  compile to PDFs identical (modulo timestamps) to the staged
  ones.
- **Phase gate → 0.4.0.** Standalone proofs added to the CI `tex`
  job, ROADMAP M3.1 DONE, merge to `main`.

## Phase C — New grammars (M3.2 + M3.3) → 0.5.0

Sequence and state diagrams behind the same `flowchart_to_tikz`
entry point: separate strict grammars, shared TikZ emitter.

- **C.1 sequence subset → 0.4.1.** `participant`/actor columns,
  `->>`/`-->>` messages with labels, `alt`/`opt` boxes. Anything
  else fails naming the statement (strict-subset doctrine holds).
  Showcase-style proof figure + Rust tests.
- **C.2 state subset → 0.4.2.** `[*]` start/end, named states,
  `transition : label` edges, composite box (no nesting — nesting
  is a build error, not a silent flat render). Proof figure +
  tests.
- **Phase gate → 0.5.0.** Both grammars in CLI/MCP/QUICKREF,
  ROADMAP M3.2/M3.3 DONE, merge to `main`.

## Phase D — Layout engine (M3.4) → 0.6.0

- **D.1 routing + clusters → 0.5.1.** Edge routing that avoids
  node interiors (segment-vs-box test, the same geometry muscle
  as the legend resolver), subgraph cluster boxes, wider DAG
  support past today's layered trees. Deterministic output
  (byte parity preserved — layout is seeded, never hashed).
  *Acceptance:* the four paper TikZ diagrams regenerate with no
  manual coordinate tweaks; golden paper still byte-identical.
- **Phase gate → 0.6.0.** ROADMAP M3.4 DONE (Phase 3 closes),
  merge to `main`.

## Phase E — Parser switch (M5.4) → 0.7.0

- **E.1 pulldown-cmark `convert()` → 0.6.1.** Rewrite the block
  splitter on the pulldown-cmark event stream per the M5.3 spike
  verdict (`%%` directive pre-processing and the `esc.rs` map
  stay untouched — only the splitter moves). Cargo: move
  `pulldown-cmark` from dev-spike to real dependency.
  *Acceptance (all three, no exceptions):* empty diff on the
  spike corpus AND golden paper byte-identical AND full suite
  green. If the golden moves by one byte, the switch is reverted
  and this phase re-plans.
- **Phase gate → 0.7.0.** Spike harness retired or kept as the
  regression corpus, ROADMAP M5.4 DONE, merge to `main`.

## Phase F — Line-numbered errors (M4.1) → 0.8.0

Additive by design (error paths only), so it rides last among
code phases — but can jump the queue any time without conflicts.

- **F.1 threaded line numbers → 0.7.1.** Source line numbers from
  the md scanner through `convert()`: mermaid fences
  (`flowchart_to_tikz`), `%% table` attrs, `plot_attrs` names,
  template/skeleton tokens, legend keywords report
  `file:line: message` with the offending line echoed. Rust
  `Error { line, echo, msg }` type replaces bare `String`s at
  every fail-fast site in the strictness catalog (ARCHITECTURE
  §7 lists them); Python formats, CLI prints, MCP returns.
- **F.2 caret rendering → 0.7.2.** Column caret under the
  offending token where the grammar knows the span (mermaid
  statements, attr keys); line echo alone where it doesn't.
  Every catalog site gets a test asserting `file:line:` format.
- **Phase gate → 0.8.0.** ARCHITECTURE §7 updated (M4.1 gap
  closed), ROADMAP M4.1 DONE, merge to `main`.

## Phase G — Engine hardening (M4.10) → 0.9.0

No new machinery — CI steps plus verification runs.

- **G.1 CI tectonic step → 0.8.1.** `tex` job installs the
  0.17.0 binary, builds demo + showcase with
  `--engine tectonic`, same undefined-citation gate.
- **G.2 paper verification → 0.8.2.** Showcase paper AND main
  paper compile under tectonic (XeTeX font handling differs —
  record every delta); Windows-embedding limitation documented in
  README's LaTeX section. Crate embedding itself stays a
  Linux-CI exercise (blocked on Windows C deps — not this phase).
- **Phase gate → 0.9.0.** ROADMAP M4.10 DONE, merge to `main`.

## Standing rules (all phases)

1. Golden paper never moves without intent; byte parity is the
   tripwire for every phase.
2. Strict-subset doctrine: new grammars fail loudly, never
   almost-render.
3. No invented numbers: every figure in proofs/showcase traces to
   measurement or is marked `[NOT YET MEASURED]`.
4. Surface parity: Rust feature → Python API → CLI → MCP →
   QUICKREF, each step tested (surface test pins the tool list).
5. `dev` is integration; `main` only takes finished phases
   (squash or fast-forward at phase gates — maintainer's call
   at 0.3.0).
