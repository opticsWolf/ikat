# Changelog

All notable changes, derived from the commit messages. Formats:
everything up to **v0.10.0** is combined per minor version
(v0.1.0 … v0.10.0); the 0.10.1+ series and later keep one entry
per released version. Phase/milestone names (M-x.y, Phases A–H)
are the roadmap's — history lives here now, planning in
`ROADMAP.md`.

## v0.13.5 — 2026-11-09

MCP parity + skill audit (agents see adaptations too).

- `weave_document` returns `notes`: auto-adapt fixes, fallbacks,
  embedded figures reach MCP agents, not just CLI stderr
- All four skill packs audited: warn-and-render doctrine in
  `ikat` + `ikat-api`, notes channel in `ikat-cli` + `ikat-mcp`
  (the general + API doctrine still described fail-fast)
- No weave-behavior change (206 cargo + 139 pytest)

## v0.13.4 — 2026-11-09

Local-figure auto-embed (host verifies, core stays pure).

- Local `![alt](path)` embeds as a real float when host-verified:
  exists, PDF/PNG/JPG, single-page PDF (stdlib page scan) or
  `#page=N` fragment (explicit intent, pdfTeX `page=` option);
  caption from alt (`Figure N` fallback), `fig:localN` labels
  collision-checked, picture span, height cap
- Missing/wrong-format (GIF/SVG hint conversion)/multi-page/
  unverifiable targets degrade with precise notes, never TeX-log
  failures; remote hrefs unchanged (bridge downloads register
  via pre-fillable `local_figures`)
- Rust core still never touches the filesystem (explicit design
  rule, now documented)
- `tests/test_local_figures.py` pins it (206 cargo + 139 pytest)

## v0.13.3 — 2026-11-09

Warn-and-render compatibility (JATS-shaped manuscripts weave).

- Lists render as paragraphs (bullets strip, ordered numbers
  stay), one note per run — no more `lists are not in the subset`
- Unicode fallback table (smart quotes, U+2212 minus, legend
  symbols, subscripts ₄–₉): applications aggregate into one
  `unicode fallbacks:` note; only unrepresentable characters fail
- Reference definitions drop with a note; captionless tables take
  `Table N`; remote figures render as alt-text (alt-less omit);
  `[^l]` footnotes become `\footnote`; `<sub>`/`<sup>` become
  chemistry math — every adaptation in `BuildResult.notes`
- Baselines hold: showcase + frozen paper byte-identical;
  `tests/test_jats_compat.py` pins the new behavior (204 cargo +
  131 pytest)

## v0.13.2 — 2026-11-09

Figure auto-adapt: less warning, more fixing (probe report II).

- Weave-time auto-adaptation for inline mermaid diagrams: measured
  geometry fills un-set span/pos/width (narrow goes column, tall
  takes a dedicated page, column overflow shrinks above the
  label-crowding floor); explicit attrs always win
- Picture-mode height cap (`[width,height=0.85\textheight,
  keepaspectratio]`, `[floats] figure_max_height`): tall standalone
  PDFs scale instead of clipping; kill-switch `[layout]
  auto_adapt = false`
- Cluster boxes expand to fit long labels; routing clearance widens
  past the default for them (short labels route byte-identically)
- Every applied adaptation reports into `BuildResult.notes`
  (stderr on the CLI); plots/tables untouched
- Baselines hold: showcase `.tex` byte-identical before/after (7 pages, 0 overfulls),
  frozen paper's only delta is the cap string (MediaBox-proven
  no-op on all four PDFs); `tests/test_figure_baseline.py` pins it

## v0.13.1 — 2026-11-09

Combining diacritics fold (needle-math follow-up, one minor feature).

- Base+mark pairs (`n` + U+0302, `z` + U+030C, …) weave clean: the
  gate skips foldable pairs and every text emitter folds them to
  text-mode accents; precomposed Ẑ/ẑ map alongside (`322dec7`)
- Inside `$…$` math and display-math spans (verbatim math) combining
  marks fail with the exact math accent (`$\hat{n}$`) instead —
  text accents would break math mode; stacking, lone marks and
  non-alnum bases fail loud with U+ codes
- Plot titles/legends/ticks, the scalar author and preset footnotes
  render the same folds (they bypassed the transliteration tables)
- Proven end to end: folded accents compile under pdflatex
- Greek auto-map (external-transform follow-up): full alphabet +
  ς/ϵ/ϑ/ϕ/ϱ map to `$…$`-wrapped commands in text, bare in math
  (omicron → plain `o`; ε/ς follow the hint vocabulary); AMS-only
  ϰ/ϖ and other symbols still fail with exact hints

## v0.13.0 — 2026-11-08

External probe-report hardening batch: fail-fast instead of TeX-log
mysteries, plus a flowchart subset expansion.

- Weave-time unicode gate (`492380f`) — any non-ASCII outside the
  transliteration table + Latin pass-through fails with `file:line:`,
  caret, U+ code and replacement hint (Greek hints like the lambda
  one); wired into body, headings, captions, cells, display math,
  author names, plot labels/footnotes and diagram labels; the label
  emitter expanded to the full table; precompiled fences exempt
  (their labels never reach TeX)
- Bullet/ordered list markers rejected with the line instead of
  silently flattening (`492380f`)
- Math-aware emphasis (`492380f`) — emphasis spanning dollar-math
  emits textbf/textit around verbatim math instead of literal stars
- Flowchart `<-->` bidirectional edges + `&` fan-in/fan-out with
  all-pairs semantics (`b1642f0`); quoted edge labels verified and
  locked; short linear chains locked with a regression test
- Placeholder-author stderr warning on `build`/`weave` (`c414852`)
- Docs: manuscript-subset section (README), unicode gate + tag
  numbering + matrix-only rule (QUICKREF), strictness catalog
  (ARCHITECTURE), skill boundaries (ikat, ikat-api, ikat-cli)

## v0.12.0 — 2026-10-27

Docs + site engine-story sync (no code change over 0.11.2).

- Docs sync: the engine story across README/ARCHITECTURE/QUICKREF
  (`2f80d83`); LaTeX requirements scoped to the pdflatex engine
- site: the PDF story reflects the engine options (`daf66d5`);
  the TeX-boundary note covers all three engines (`591d622`)
- Pages: gate deploys on absolute-link check —
  `site/check_links.py` GETs every absolute href/src before the
  artifact upload (the skills-404 drift class) (`67f4006`);
  site skill links follow the M7.9 move (`4901a6c`)
- Installation hints: PyPI wheel as the primary path +
  TeX-tier guidance; the "not on PATH" CompileErrors carry the
  remedy (TinyTeX, `--no-pdf`, tectonic/texres) (`8c5e6c5`)
- README Layout section: complete module map (all 19 Rust
  modules, skills/ nested, site/ + workflows mapped) (`0e1ad02`)
- ROADMAP: 0.11.0 shipped; TeXres watch item added (`7a21066`);
  phase headings reflect completion (`95e0700`); document split
  into ROADMAP (planning) + CHANGELOG (this file)

## v0.11.2 — 2026-10-27

- texres engine: cascade slot + `--engine texres`
  (`3db4e6f`) — the texres one-binary Rust TeX distribution
  (LaTeX, packages, fonts, BibTeX bundled; nothing to install):
  `compile_pdf(engine="texres")`, stdout transcript kept beside
  the PDF for the CI gate, `ensure_packages` a no-op; example
  builders' choices updated; proven end-to-end on the demo, the
  showcase (7 pages, References), and the CLI

## v0.11.1 — 2026-10-27

- tectonic feature is Linux-gated; CI exercises the embed
  (`95a808d`) — the optional dep moved under
  `cfg(target_os = "linux")`, so `--features tectonic` resolves
  on every target (no-op off Linux) instead of failing the
  build; ubuntu CI compiles the embedded engine
  (`cargo test --features tectonic`); Windows keeps the binary
  cascade

## v0.11.0 — 2026-10-26

First release from merged `main`: tag `v0.11.0` pushed, PyPI
wheels (3.10–3.15 + PyPy, Trusted Publishing) and the crates.io
crate published.

- load_preset: POSIX long-string fix (`a070802`) —
  `Path.is_file()` on a >NAME_MAX string raises ENAMETOOLONG on
  POSIX (Windows answers False); regression test added
- handwoven showcase ports the engine's winning attrs →
  hand == auto byte-identical (`6e469e4`); site asset = the
  auto-layout paper (`e9bb1cd`); auto-proof provenance honest
  (`332bb50`); examples cleanup — bench cache downloadable,
  never committed (`0cb7a12`, `bench_fetch.py` + manifest)
- Docs + README sync to 0.11.0 (`baa79ba`)

## v0.10.15 — 2026-10-26

- M7.10b opt-in author collapse (`--authors-max`)
  (`7da192d`) — first N marked names + ", et al." (IEEE style),
  full affiliation block stays, 0 never collapses, negative
  fails fast; MCP/API via spec; ikat-mcp skill row updated
  (`8acdfba`)

## v0.10.14 — 2026-10-26

- M7.10 full author front matter (`aa14f35`) —
  `BuildSpec.authors/affiliations/author_affils` composed into
  one wrapped `\parbox` with numeric superscript marks
  (IEEEtran-verified); `&` escaped via `esc_text`; out-of-range
  marks fail fast; legacy specs byte-identical; exporter reads
  every contributor + affiliation with xref marks

## v0.10.13 — 2026-10-26

- M7.9c `ikat help [COMMAND]` (`bbe1092`, `56c4037`) —
  exit-0 help verb, bare-invocation help, `--json` registry
  dump (all 14 MCP tools); flag-complete skill drift tests
  (`_flags_taught`/`_real_flags`) pin SKILL.md to the parser

## v0.10.12 — 2026-10-26

- M7.9b tool registry + skill installer (gossamer transfer,
  tranche 2) (`d14549b`) — `registry.py` one source of truth
  for CLI/MCP/API tools; `ikat skills install` bundles the
  skill packs (wheel + MCP resources)

## v0.10.11 — 2026-10-26

- M7.9 MCP schema fidelity + bundled skills (gossamer-audit
  transfer) (`a9bd718`) — per-param descriptions and enums in
  MCP schemas, instructions payload, `skills/__init__` +
  drift tests pinning skill files to the code

## v0.10.10 — 2026-10-25

- M7.8b calibrated display-math width: probe TeX, don't guess
  (`7b7a094`) — `mathwidth.rs` ships TeX-calibrated estimator
  constants; display equations sized by measurement, not
  character counts

## v0.10.9 — 2026-10-25

- M7.8 aligned breaks: relations open continuations, left edge
  holds (`50389fb`); corpus REPORT refreshed — math 0 hbox

## v0.10.8 — 2026-10-25

- M7.8 honest math: MathML rebuild + auto-stretch displays
  (`306500c`) — inline math from MathML semantics, display
  equations stretch to measured widths

## v0.10.7 — 2026-10-25

- M7.7 honest portrait limits: Tiny rung, pin height,
  hyphenpenalty truth (`1928e8e`)

## v0.10.6 — 2026-10-25

- M7.6 cell stacking (PLOS-original style): econ corpus to
  zero overfulls (`ca43524`)

## v0.10.5 — 2026-10-25

- M7.5 table compaction ladder (opt-in): dense tables stop
  overlapping (`c688bf1`)

## v0.10.4 — 2026-10-25

- M7.4 tail page: no empty columns in multi-column layouts
  (`73d16cc`)

## v0.10.3 — 2026-10-25

- M7.3 refs barrier (opt-in): floats stay out of References
  (`8f3069b`)

## v0.10.2 — 2026-10-25

- M7.2 top-fit + queue pressure: wide floats land near their
  text (`70def0a`); full-content md export of the 10 bench
  papers + pictures (`2872a1d`); auto-proof cleaned (`f687dac`)

## v0.10.1 — 2026-10-25

- Late wide-table rule + Table 4 takes the p7 top (`bbe252d`)

## v0.10.0 — 2026-10-25

- Phase H gate (`1fee150`): M7.1 auto-layout done — 136 cargo +
  86 pytest green

## v0.9.0 — 2026-10-08

Phase G gate (`aaa34cb`): M4.10 done, all phases complete.

- M7.1 auto-layout (`ikat layout`): measure floats, suggest
  span/pos (`3822c5d`, bumped 0.9.2); showcase layout pass —
  shared float page, force-pinned tables and plots (`e82532f`)
- Showcase refresh (0.9.1): new title, anchored figures,
  sequence box-label fix (`f40c408`); runpy argv isolation fix
  (`4e06bcb`)

## v0.8.0 — 2026-10-08

Phase F gate (`1d6ffcf`): M4.1 done, 118 cargo + 75 pytest.

- G.1 tectonic CI (0.8.1) (`931d2b3`): pinned 0.17.0 binary by
  checksum, engine passthrough, `--keep-logs` undefined-citation
  gate
- G.2 paper verification (0.8.2) (`4846128`): engine deltas
  ledgered, README note

## v0.7.0 — 2026-10-08

Phase E gate (`a917832`): parser switch done, 98 cargo + 69
pytest.

- F.1 threaded lines (0.7.1) (`3a47f2d`): `Error { msg, line,
  col, echo }` at every fail-fast site; file:line: wrappers
- F.2 carets (0.7.2) (`b7e7277`): column rendering,
  indent-exact columns

## v0.6.0 — 2026-10-08

Phase D gate (`6f43ad3`): layout + guards done, 96 cargo + 69
pytest.

- E.1 parser switch (0.6.1) (`0ed56c4`): `md.rs` production
  mapping on pulldown-cmark, golden identical
- E harness (`869f0cf`): hand vs pulldown block parity, corpus
  empty-diff

## v0.5.0 — 2026-10-08

Phase C gate (`8ff9e99`): M3.2/M3.3 done, 81 cargo + 62 pytest.

- C.1 sequence subset (0.4.1) (`d0c6331`): `emit.rs` shared
  vocabulary, grammar, dispatch, showcase fig 2
- C.2 state subset (0.4.2) (`a650ba3`): shared placer,
  composites, showcase fig 3, subset tables

## v0.4.0 — 2026-10-08

Phase B gate (`cc8dbc8`): M3.1 done, CI proofs step, docs.

- B.1 standalone emitter (0.3.1) (`112cae7`): `tikz_needed`/
  `plots_needed` predicates, wrap + compile, CLI/MCP surface,
  proofs + ledger

## v0.3.0 — 2026-10-08

Phase A gate (`3703a10`): presets + regen done, 67 cargo + 52
pytest.

- A.1 preset loader (0.2.2) (`9c3e9a0`): benchmark JSON in,
  tikzpicture out
- A.2 preset surface wiring (0.2.3) (`ec2287a`): CLI `--preset`,
  MCP `preset_json`, skills + docs
- A.3 regen harness (0.2.4) (`6d4755b`): 4 paper plots via
  presets, snapshots, ledger, CI step

## v0.2.0 — 2026-10-08

From the first tagged release through 0.2.1 (PyPI via trusted
publishing, `e91c8ed`).

- Showcase paper: ikat about ikat, every feature exercised
  (`dbe0ad7`) + review polish (`59bc8bb`, `048b7d3`)
- Legends below the axis, never over data (`c6141b8`); legend
  position keyword (`4c6c9f8`); auto legend placement —
  collision-tested corners first (`cff99e8`); plot error-bar
  fix (`9e15ad6`)
- Surface gaps closed: CLI plots + skeletons, MCP
  `list_skeletons` (`d7d88cd`); CI + release workflows
  (`8a49db0`)
- GitHub Pages product site (`4a2ae19`) + design-system notes
  (`7870453`); badges (`ab5f260`, `8103968`)
- Agent skills: ikat-api, ikat-cli, ikat-mcp (`8299a19`) +
  agent-surface docs (`5030a30`)
- QUICKREF (`5497deb`); ARCHITECTURE full expansion
  (`1fa48a3`); docs refresh (`c3890ae`)
- M4.9 tectonic prototype: engine cascade proven end-to-end
  (`1d96d3b`)

## v0.1.0 — 2026-10-07

First tagged release: the pipeline works and self-hosts.

- Phase 0+1: md-to-tex/pdf pipeline with Rust core (`e2b3d09`);
  crate renamed to ikat, auto-detected TeX requirements
  (`cf3936a`)
- M4.7 template library: 9 compile-proven publisher/layout
  heads (`d2e4ed8`); M4.6 user templates (`62f4aa4`); M4.3
  ensure-packages helper — the texliveonfly trick (`4723746`)
- Unified CLI/MCP/API surface (M4.2 + M4.8) (`0c90c49`)
- M5.1 float formatting: pos/width/captionpos attrs (`b9f2050`);
  M5.2 level-3 skeleton (`f535ffe`); M5.3 pulldown-cmark spike
  (`5249675`); FORMAT-DRAFT + Phase 5 roadmap (`7d9f8d4`)
- M4.4 self-hosting: paper builds with ikat itself (`4bc174e`)
- M4.9 tectonic prototype: engine cascade proven end-to-end
  (`1d96d3b`)
- Publish prep: license files, cargo description/repository
  (`8ba8f00`)
