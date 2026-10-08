# ikat architecture

Version 0.2.0 · 2026-10-07 · 59 cargo + 45 pytest green.

Per-module test counts are deliberately not recorded here — they go
stale every release that adds a test, which is every release.
Regenerate them: `cargo test` (Rust) and
`VIRTUAL_ENV=$PWD/.venv maturin develop && .venv/Scripts/pytest tests/`
(Python, against the built module).

```
mini.md + ikat.toml
      │  fences, tables, citations
      ▼
┌───────────── python/ikat/pipeline.py ──────────────┐
│ Element(kind, body, caption, label, span?)         │
│ resolve_span(): toml default ← fence attr ← call   │
│ figure_env(): figure vs figure* + width            │
└──────┬──────────────────────────────┬──────────────┘
       │ mermaid fence                │ data (plots)
       ▼                              ▼
┌─ Rust src/mermaid.rs ──┐  ┌── Rust src/plot.rs ───┐
│ parse → layered layout │  │ tables → pgfplots code│
│ → tikzpicture          │  │ bar+whiskers, log-y,  │
└────────────────────────┘  │ lines+error bars      │
                            └───────────────────────┘
       └──────────────┬───────────────┘
                      ▼
        complete .tex ──► ikat.compile ──► pdflatex/bibtex ──► PDF
```

## 1. The layer contract

- **Rust owns everything computable.** Parsing, layout, numeric
  formatting, and code generation are deterministic byte-level work —
  exactly what Rust is good at, and exactly what must never differ
  between runs. Python never post-processes Rust output, only embeds it.
- **Python owns documents and tools.** Fence extraction, span
  resolution, float assembly, spec registries, and driving `pdflatex`
  are glue: thin, readable, replaceable. Strings cross the PyO3
  boundary; decisions do not — except through `BuildSpec`, the one
  shared model (see §4).
- **No matplotlib, ever.** Data graphics compile inside the document's
  own TeX run (pgfplots), so figure fonts are document fonts by
  construction. The pipeline cannot produce a font-mismatched figure.
- **No Node.js.** Mermaid renders through our own flowchart-subset
  parser, not mermaid-cli + browser + SVG round-trip.

## 2. End-to-end flow

There are two entry paths, and the distinction matters:

**Fragment path** (`pipeline.weave_fragment`, used by `demo.py`):
markdown + toml → `Element` list (kind, body TeX, caption, label,
span/pos/width). No registries, no numbering, no bibliography —
each fence converts standalone. This is the fast loop for single
figures and the demo.

**Document path** (`document.build_document` / `build_from_paths`,
used by `ikat build`, the showcase `build.py`, and the retired
paper builder): markdown + toml + `BuildSpec` → Rust `convert()`,
which returns a 5-tuple — woven body, document title (the H1),
diagram count, table count, and `FloatNeeds` (which of
`float`/`placeins`/`dblfloatfix`/`graphicx` the body provably
requires). Python then wraps the body: generated preamble, or a
level-1 `preamble_file`, or a level-3 `skeleton`; validates that
the wrapper carries every needed package; appends the bibliography
lines; and hands the complete `.tex` to `compile_pdf`.

The hand-off between the layers is narrow on purpose:

| direction | mechanism |
|---|---|
| Python → Rust | `BuildSpec` JSON (`spec_from_dict` is the single constructor the API, CLI `--spec`, and MCP `spec_json` all share), toml text, markdown text |
| Rust → Python | woven body + title + counts + `FloatNeeds`; error strings (line-numbered errors are M4.1, still open) |
| Python → TeX | complete `.tex`, staged `.bib`/`.bst`, figure PDFs |
| TeX → Python | the PDF path, or a `CompileError` with the log tail |

## 3. Rust core (`src/`, via `ikat._core`)

### 3.1 `mermaid.rs` — flowchart subset → TikZ

Deliberately not mermaid: `graph TD`/`graph LR` with `-->` edges,
`---` links, `|label|` edge labels, `[]` box nodes and `{}`
diamond nodes. Anything else — other directions, other node
shapes, subgraphs, styling statements — is a string error naming
the offending statement, because a diagram that almost renders is
worse than one that refuses.

Layout is deterministic layered (BFS depth from the roots, nodes
ordered within a layer), so the same source always emits the same
coordinates — byte parity is a feature, not an accident. Edges
carry `node[midway,above,fill=white,inner sep=1pt]` labels: the
white knockout keeps diagonal edges from striking through their
own labels. Fence attributes (`span`, `pos`, `width`, `scale`)
are parsed in Rust; the `[scale=]` splice exists in both the Rust
and the Python mirror (a past double-bracket bug is pinned by a
test on each side).

`sequenceDiagram` dispatches on its header line through the same
`flowchart_to_tikz` entry (Phase C.1): participant columns in
declaration order, one row per message, `alt`/`else`/`opt` boxes
spanning all columns with the label outside above the top edge
(inside it would strike the first message's label — proven by
probe, fixed before the showcase). The knockout (`fill=white`)
and node (`\node (id) at (x,y)`) vocabulary live in `emit.rs`,
lifted verbatim from `mermaid.rs` — the golden paper proves the
move byte-neutral.

`stateDiagram-v2` (Phase C.2, `state.rs`) reuses the flowchart
placer itself: `Graph`/`depths`/`layered_xy` are `pub(crate)` in
`mermaid.rs`, states become nodes and transitions become edges.
Layout first, boxes second — the composite rect is computed from
placed member coordinates, so a box can never move them. The
box-label rule (inside-top with half-row headroom, after an
outside-above label struck an edge label in review) is recorded
in the code for the D.1 subgraph boxes to reuse. `[*]` renders
as a filled dot (start) and bullseye (end); nested composites
are a build error.

Phase D.1 (`layout.rs`, `mermaid.rs`): the legend resolver's
segment geometry (`segs_cross`, `seg_hits_rect`) moved here and
is reused, not rewritten. `depths()` is longest-path layering
(multi-parent = max+1; cyclic depths cap at n, still
terminating). `subgraph ID [title]`…`end` clusters draw
post-layout via the shared `cluster_box`/`cluster_rect`
(same numbers as state composites). Edges crossing a
non-endpoint node box (conservative 3.4×0.9cm estimate)
reroute via a pushed perpendicular midpoint (8 pushes;
exhaustion names the edge); clear segments emit the exact
historical line, so clean graphs are provably untouched. The
showcase Figure 1 re-laid under max-depth (reviewed, clean);
the golden main paper never calls the emitter (precompiled
mode), so it stays byte-identical by construction.

### 3.2 `plot.rs` — data → pgfplots

Two emitters, no plotting library: `barchart` (grouped bars with
min/max whiskers, optional log-y, optional dashed refline) and
`lineplot` (series with symmetric error bars, axis pinned to the
data so no meaningless pre-first-point gap appears). Numbers are
formatted plain-decimal (4 significant decimals max, no trailing
zeros, no scientific notation); tick labels are TeX-escaped
(`$\sim$` for approximations, never `~`).

**Legend placement** (`LegendPos`) is the module's second job.
Seven keywords: `auto` (default), `below`, four inside corners,
`outside-right`. `auto` tries the corners in top-left, top-right,
bottom-right, bottom-left order, testing each candidate box
against the drawn data — polyline plus error-bar segments for
lines (orientation-test segment intersection), whole-slot bar
rects up to the whisker top for bars (log-mapped for log-y, which
is what the eye sees), plus the refline — on visual ranges that
deliberately overestimate pgfplots' padding, so verdicts stay
conservative. Boxes are 30%-by-25% of the ranges: oversized on
purpose, so "free" really means empty. All four occupied, or a
degenerate range (single point), falls back to the under-axis
row, which clears tick labels by construction. Inside corners
cannot touch axis labels — labels live outside the axis box.
Unknown words fail naming the full set.

**Presets** (`preset.rs`, Phase A.1): benchmark JSON in,
`tikzpicture` out. The schema wraps what the showcase `build.py`
assembled by hand — `kind` (`bar`|`line`), `title`, `ylabel`
(+ `xlabel`), shared `series_names`, bar fields `group_labels`
`values`/`mins`/`maxs` (+ `log_y`, `refline`), line fields `xs`
`yss`/`errs`, optional `legend` and `footnote` (rendered
`{\footnotesize …\par}` after the picture). Validation is
strict: rectangular series, `min ≤ value ≤ max` elementwise, `xs`
strictly increasing — violations name the key, never a bare serde
dump. Filesystem rule holds: Rust parses strings, Python
(`preset.py::load_preset`) reads files and names them in errors.
The emitters are untouched; the loader converts into their
argument structs.

### 3.3 `table.rs`, `esc.rs`, `doc.rs` — the paper path

- `table.rs`: pipe tables → `tabularx` with per-table column
  specs (the appendix `Xcl` style), caption registry, TABLE
  numbering.
- `esc.rs`: the Unicode→LaTeX map, `$…$` math protection (math
  passes through untouched), code/emphasis spans, IEEE section-
  number stripping. Custom and kept even if the block parser ever
  moves to pulldown-cmark — escaping is orthogonal to splitting.
- `doc.rs`: `convert()`, the document assembler. Walks the line
  stream, dispatches fences/tables/citations/paragraphs, threads
  `FloatNeeds` upward, enforces the citation keyset (a dangling
  key fails the build instead of printing raw). The golden test
  rebuilds the frozen paper `.tex` byte-identical — the paper is
  the test suite.

### 3.4 `config.rs` — `ikat.toml`, validated

`Document` (class, options, columns, margins), `Spans`
(per-kind diagram/plot/table/picture/default), `Template`
(`preamble_file`, `preamble_append`, `abstract_before_maketitle`,
`skeleton`), `Floats` (fractions, counters, `barrier_sections`),
`Pos` (the seven positions with their exact LaTeX specs).
Defaults never move: `[floats]` tuning emits only when
non-default, so a default document weaves byte-identically before
and after the feature existed. Non-default values are validated
at parse time, not at TeX time.

### 3.5 `texenv.rs`, `tectonic.rs`, `md.rs`, `md_spike.rs`

- `texenv.rs`: scans any preamble into `(probe file, tlmgr
  package)` needs — the static half of the texliveonfly trick.
  The subprocess half lives in Python (`texenv.py`: one
  `kpsewhich` call to probe, one `tlmgr install` to fix).
- `tectonic.rs`: the optional engine path (cargo feature, off by
  default so wheels stay lean; hardened Phase G — 0.9.0).
  `tex_for_tectonic` drops the `inputenc` line; empty `bib_name`
  emits no bibliography lines because tectonic auto-runs BibTeX
  and dies on an empty `\bibliography{}`. CI (`tex` job)
  installs the pinned 0.17.0 binary (checksum-verified) and
  builds demo + showcase with `--engine tectonic` under the
  same undefined-citation gate: `_compile_tectonic` passes
  `--keep-logs` (tectonic otherwise swallows the XeTeX log and
  undefined citations would pass silently — found during G.1
  verification) and the gate greps both engines' `.log` files.
  Both papers build under both engines with 0 undefined
  citations (22 + 10 pages, same geometry; engine deltas —
  all font-encoding class — ledgered in `ACCEPTED.md`). Crate
  embedding does not link on Windows (C deps) — the supported
  path is the `TECTONIC_EXE`/PATH binary cascade, and embedding
  stays a Linux-CI exercise by design (recorded so it stops
  being re-proposed).
- `md.rs` (E.1/M5.4): the block splitter. pulldown-cmark owns
  boundaries (`Parser::into_offset_iter`, `TABLES` only — no
  other extensions); content comes from raw source slices, so
  `esc.rs`, fence attrs, tables, and directives behave exactly as
  the retired hand scanner did. Inline events are ignored. The
  fence-aware pre-pass drops `>` quotes (the old scanner dropped
  them mid-paragraph too), extracts `%% table` directives by
  table sequence number, and errors loudly on `[label]:`
  reference definitions (the stream would swallow them silently).
  Paragraphs break only on blank lines (line arithmetic, never
  newline counting — spans swallow newlines); consecutive `Para`
  blocks are blank-separated by construction, so `convert()`
  flushes between them (the one consumption bug the golden test
  caught mid-switch).
- `md_spike.rs`: the frozen hand-scanner oracle beside a thin
  production-backed mapping. Post-switch it runs as the permanent
  cross-check: oracle vs `md::blocks()` on the full corpus
  (141+35+3 blocks, empty diff) plus latent-divergence synthetics.
  If the mapping ever drifts from the hand rules, this reddens
  before the golden does.

## 4. Python layer (`python/ikat/`)

### 4.1 `pipeline.py` — fences, spans, floats

`extract_fences` splits the manuscript; `weave_fragment` maps
each fence to an `Element`. Span resolution is a three-level
cascade — toml kind default, fence attribute, explicit call-site
argument — with `picture` falling back to `diagram`. Position
resolution (`resolve_pos`) mirrors it against `[floats]`.
`figure_env` chooses `figure` vs `figure*` plus width, and
refuses the illegal combination (wide + `here`/`force`) as a
build error with the fix attached. Attr parsing (`_attr`,
`_width_attr`, `_span_attr`) is strict: unknown keys and doubled
tokens fail here, never in a TeX log. A Rust mirror of the
span/pos logic exists for the document path; the two are pinned
to each other by shared test vectors.

### 4.2 `document.py` — BuildSpec assembly

`BuildSpec` is the whole build in one dataclass: `diagrams`
(keyed caption registry), `plots` (keyed captions) + `plot_dir` +
`plot_insert_before` + `plot_attrs` (per-plot span/pos tuning),
`table_captions` + `table_specs` (per-table colspecs),
`title_thanks`/`author`, `bib_name`/`bib_style`/`bib_keys`
(derived from the `.bib` via `--bib`, never frozen),
`graphicspaths`, `preamble_override`/`preamble_append`/`skeleton`.
`spec_from_dict` is the one constructor; `to_json` round-trips
it for `--spec` files. Registries are hand-owned — the spec, not
the markdown, names captions — so renaming a caption cannot
desynchronize the document. `build_document` runs `convert()`,
wraps, validates, and returns `BuildResult` (title, body, full
tex, counts). Template helpers (`list_templates`,
`template_path`, `template_preset`, `list_skeletons`) serve the
CLI, MCP, and headsproof alike.

### 4.3 `compile.py`, `texenv.py`, `cli.py`, `mcp_server.py`

- `compile.py`: `compile_pdf(workdir, main, ensure_packages,
  engine)` drives `pdflatex`+`bibtex` passes (or the tectonic
  cascade), raising `CompileError` with the log tail instead of
  a return code. `ensure_packages=True` gates the build on the
  texenv probe. Deliberately excluded from MCP (workdir-bound,
  minutes-long) — drive it from the CLI.
- `texenv.py`: `check` (report) / `ensure` (install + re-probe)
  around the Rust scan; failure carries the manual texmf-install
  recipe and the MiKTeX Portable fallback.
- `standalone.py`: `fence_to_tikz` (`mermaid`|`chart`), `wrap_standalone`
  (minimal `[tikz,border=5pt]{standalone}` + detected libraries),
  `compile_standalone` (writes `fig.tex`, drives `compile_pdf`).
  MCP gets the source only (`standalone_figure`), never the binary.
- `cli.py`: thirteen subcommands (`build weave check templates
  template skeletons skeleton flowchart barchart lineplot
  standalone version`), each a thin call into the API. Plot commands take a
  JSON payload (file or stdin) with an optional `--legend`
  override.
- `mcp_server.py`: FastMCP stdio, twelve tools mirroring the
  CLI/API map one-to-one (minus `compile_pdf`). The surface
  test pins the full tool list, so a new tool without MCP
  registration fails loudly.

## 5. Spanning model

Two-column classes (`IEEEtran`, `acmart`, …) distinguish `figure`
(one column) from `figure*` (both). ikat resolves per element:

`ikat.toml [spans]` kind default → ```` ```mermaid {span=…} ````
fence attribute → explicit call-site argument.

Defaults (`diagram = "wide"`, `plot/table = "column"`) encode the
lesson from the Macrame paper: flowcharts need text width, data
graphics hold a column. The showcase paper proves the mix: a wide
pipeline diagram, a column float-resolution diagram, column plots,
and a `table*` token contract — all in one document.

## 6. Templates and skeletons

Level 1 (`preamble_file`) replaces everything before
`\begin{document}`; ikat validates the replacement carries a
`\documentclass` and every package the body provably needs, and
`preamble_append` splices extra lines before
`\begin{document}`. Nine shipped heads cover arXiv, 1/2/3-column
articles, IEEE, ACM, LNCS, Elsevier, APS — each with its
`bib_style` + toml preset, each compile-proven with abstract +
TikZ + table and PDF text verification.

Level 3 (`skeleton`) replaces the whole document. Token contract:
`{{body}}` and `{{bibliography}}` exactly once each
(`{{bibliography}}` waived when `bib_name` is empty, for biblatex
skeletons), `{{abstract}}` exactly once when the manuscript has
one (the whole woven env, honoring
`abstract_before_maketitle`). Unknown and duplicate tokens are
errors; `%` comments are invisible to both counting and
rendering, so skeletons can document themselves.
`skeleton-plain.tex` (article + tokens, `placeins`/`float`
preloaded) is the starting point. Validation scans the whole
skeleton for package needs — the generated tail's assumptions do
not leak into custom documents.

## 7. Strictness catalog (M4.1 closed, Phase F — 0.8.0)

Fail-fast sites, each with a test: unknown mermaid statements,
missing edge targets, empty diagrams; ragged plot series;
dangling citation keys; unknown template/skeleton tokens and
duplicates; template heads missing class or packages; wide floats
with `here`/`force`; unknown float attrs or keys; unknown legend
words; `--bib` keysets that exclude a cited key; reference
definitions; router exhaustion; subgraph/composite nesting. The
doctrine: nothing fails in a TeX log that could have failed in
Rust or in the wrapper.

Errors are `src/error.rs::Error { msg, line, col, echo }`
(`--help` never needed: the message IS the documentation):

- `line` is 1-based in the relevant source — manuscript lines
  for scanner/diagram/table errors (fence bodies resolve as
  `fence line + row`; `md.rs` blocks carry original numbers
  across pre-pass deletions), head-file lines for template
  errors, snippet-relative for direct API calls. `line == 0`
  means no manuscript line applies (JSON/TOML data errors:
  ragged series, preset shapes, config values) — the `--> line`
  part is then omitted, never rendered as `0`, and the echo
  (offending shapes/values) carries the context.
- `col` is `Some` where the grammar knows the token span
  (attr keys, direction words, edge stop positions, template
  token braces, citation keys, the `titlesec` word) and the
  renderer adds a `^` line in character columns — including
  through leading whitespace (the classic off-by-indent is
  pinned by test). Elsewhere `col` stays `None`: absence of a
  caret is never a failure, only missing precision.
- `echo` is the offending source text — raw lines (indent
  kept) so carets align under it.
- The PyO3 boundary carries the Display string unchanged
  (exception type still `ValueError`); Python — the only layer
  that sees paths — upgrades to `file:line: message` in
  `document.with_file` (template/skeleton lines resolve against
  the head file, everything else against the md; line-less
  errors become `path: message`). The CLI prints the rendering
  verbatim; MCP returns it as the error text.

## 8. Testing and CI

Four layers: `cargo test` (unit + golden-paper byte parity +
spike diffs), pytest (end-to-end weaves, surface pinning, float/
skeleton/template suites), compile proofs (`plotproof`,
`floatsproof`, `headsproof`, `skelproof`, `tectproof` — each
compiles real TeX and verifies PDF content, not just exit codes),
and the showcase paper (`examples/ikat-paper/`, a 6-page paper
about ikat built by ikat: the release demo and the composition
proof). `.github/workflows/ci.yml` runs cargo + pytest on
ubuntu/windows/macos and rebuilds the demo + showcase on real
TeX Live, failing on any undefined citation. `release.yml` gates
tags on version equality and publishes PyPI wheels + crates.io
in parallel.

## 9. Roadmap

The plan of record is `docs/ROADMAP.md` (milestones M1–M6 with
acceptance gates). Done: pipeline parity + golden master (M1,
M4.4), floats (M5.1), skeletons (M5.2), CLI/MCP/surface parity
(M4.2, M4.8), legend auto-placement (M2.4), CI + release
workflows (M4.2b), preset loader + regen harness (M2.1, M2.2 —
Phase A, 0.3.0), layout + heading guards (M3.4, Phase D —
0.6.0), parser switch (M5.4, Phase E — 0.7.0), line-numbered
errors (M4.1, Phase F — 0.8.0), tectonic hardening (M4.10,
Phase G — 0.9.0). No open milestones; tectonic crate embedding
remains a Linux-CI exercise by design.
