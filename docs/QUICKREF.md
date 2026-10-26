# ikat — Quick Reference

**v0.11.0 · md → tex/pdf pipeline, Rust core + thin Python**

## Showcase paper

[Read the exported ikat paper (PDF)](../site/assets/ikat-paper.pdf). Its [Markdown source and build instructions](../examples/ikat-paper/) are also in the repository.

## Install & build

```bash
uv venv .venv && uv pip install -p .venv maturin pytest "mcp>=1,<2"
VIRTUAL_ENV=$PWD/.venv maturin develop   # builds ikat._core in place
cargo test                               # 173 Rust tests
.venv/Scripts/pytest tests/              # 104 pytest, 4 skipped (windows; pytest tests/ elsewhere)
```

## CLI (`ikat`)

```bash
ikat build DOC.md --toml ikat.toml --outdir out [--spec S.json] [--bib R.bib]
     [--author NAME…] [--affiliation INST…] [--author-affil A:N[,N…]]
     [--authors-max N]
     [--engine pdflatex|tectonic] [--bib-style B] [--ensure-packages] [--no-pdf]
ikat weave DOC.md --toml ikat.toml --spec S.json [--bib R.bib] [--author NAME…]
     [--affiliation INST…] [--author-affil A:N[,N…]] [--authors-max N] > doc.tex   # no TeX needed
ikat check DOC.tex [--install] [--verbose]                   # exit 1 if missing
ikat templates | ikat template NAME [--show-preset]
ikat skeletons | ikat skeleton skeleton-plain
printf 'graph TD\na[x]-->b[y]\n' | ikat flowchart -
echo '{...}' | ikat barchart - [--legend WORD]    # JSON payload, stdout tikzpicture
echo '{...}' | ikat lineplot - [--legend WORD]
ikat standalone FIG.md --out fig.pdf             # one mermaid|chart fence → PDF
ikat layout DOC.md [--toml T] [--spec S.json] [--json] [--apply]  # no TeX needed
ikat skills list | show ikat-cli | install [--dest DIR…] [--project] [--skill N…] [--force]
ikat help [--json] [COMMAND]          # command list, or one command's flags
ikat version
```

`barchart` payload: `title ylabel group_labels series_names values mins maxs`
+ optional `log_y refline:[x0,x1,y,label] legend`.
`lineplot` payload: `title xlabel ylabel xs names yss errs` + optional `legend`.
`--preset FILE|-`: benchmark JSON instead of a payload (`kind bar|line`,
`series_names` shared; bar adds `group_labels values mins maxs`, line adds
`xs yss errs`; optional `legend footnote`). Python: `load_preset(path_or_json).

## Legend keyword

`auto` (default) → `below` → `top-left` `top-right` `bottom-left`
`bottom-right` → `outside-right`. Anything else is a build error.

`auto` tries inside corners in TL, TR, BR, BL order, testing each
against the drawn data (line + error-bar segments; bar slots to the
whisker top, log-mapped; refline) on conservatively padded ranges,
and falls back below when all four are occupied. Inside corners
cannot touch axis labels; the below row clears tick labels.

## Diagram subsets (all through `flowchart_to_tikz`, header dispatch)

| grammar | covered | errors |
|---|---|---|
| `graph TD/LR/...` | directions TD/TB/LR/RL/BT; nodes `[]` `{}` `([])` `[[]]`; edges `-->`, `---`, `==>` with `\|label\|`; chains, `;`, `%%`, `<br/>`; `subgraph ID [title]`…`end` (nesting errors); multi-parent takes max(parent)+1; crossing edges reroute (exhaustion errors) | other directions/shapes, styling, `{{}}`, dotted edges |
| `sequenceDiagram` | `participant`/`actor` (+ `as` labels); `->>` solid, `-->>` dashed; `alt`/`else`/`opt`/`end` boxes | `loop`/`par`/`Note`/self-messages, anything else |
| `stateDiagram-v2` | `[*]` endpoints; `A --> B [: label]`; `state "Label" as Name`; one composite level | nested composites, `direction`, notes |

Layout: flowcharts layered BFS; sequences one row per message;
states reuse the flowchart placer (layout first, boxes second).

## Float attributes (per element)

Diagrams/plots take `{...}` on the fence; tables via `%% table {...}`
on the line above.

| attr | values | default |
|---|---|---|
| `span` | `column` \| `wide` | `[spans]` per kind (`diagram` wide; plot/table column) |
| `pos` | `top` `bottom` `both`(`[!tb]`) `page` `here` `force`(`[H]`) `barrier` | `[floats] pos_default` (`top`) |
| `width` | fraction (`0.8`) or TeX length (`5cm`) | span width |
| `captionpos` | `top` \| `bottom` | figures bottom, tables top |

Hard errors: `figure*` + `here`/`force`; unknown attrs/keys.
Wide+bottom loads `dblfloatfix`; `force`/`barrier` load
`float`/`placeins` (or the template must carry them).

Display math fills its own paragraph as one `$$...$$` span (M7.8):
the weaver emits `\[...\]` (`\tag{N}` rides inside, from the
exporter's `$$tex \tag{N}$$`). Width is decided by a TeX-calibrated
estimator (`src/mathwidth.rs`: glyph widths probed at 10/7/5 pt,
the class-glue table with previous-atom tracking, zero glue in
script styles, stacked script max, column-aligned matrices). A span
wider than the column
takes a full-width `strip` (cuted); wider than the full measure
breaks BEFORE top-level `=`/`+`/`-` into left-aligned `aligned`
rows (the relation opens the continuation, original style; matrix
spans split into one strip per piece instead — `&` is illegal in
`aligned`). Whole-paragraph only: stray `$$`, empty spans, alien
environments, and unbalanced braces are hard errors.
Fraction `width` scales TikZ coordinates, not text (no `transform
shape`): labels stay full-size, so leave air around them.
`[floats]` tuning (fractions, counters, `barrier_sections`,
`barrier_before_refs`) emits only when non-default.
`[tables] compact_tables` (default off) runs the portrait-fit
ladder: the widest whitespace-delimited token per table decides.
`Fit` keeps `\small`; `Compact` steps to `\footnotesize` + 3pt
`\tabcolsep`; `Tiny` steps to `\scriptsize` + 2pt padding but stays
portrait (PLOS-original density for dense regression tables);
`Rotate` emits `sidewaystable(*)` (own page) in `\scriptsize` +
2pt, sized against `\textheight`. A column table the portrait
column cannot hold promotes to `wide` first; a stacked table too
tall to pin in-flow promotes wide + top instead of overflowing
the column. Tokens measure narrow-aware (digits/punctuation at
0.85 weight). No hyphenation discount: IEEEtran sets
`\hyphenpenalty=750`, so plain words effectively never break —
every run binds, exactly as measured.
Before deciding, the ladder stacks estimate/SE cells
(`0.080***(0.015)` → estimate over `(0.015)` via `\newline`,
PLOS-original style): the break halves the longest token, so 7-col
regression tables fit portrait at full size and rotated 13-col
tables gain headroom. Only trailing digit-bearing paren groups
split; stars stay with the estimate, prose notes stay single-line.
Off keeps every table byte-identical (the frozen paper track
carries accepted mild overfulls); the auto-layout notes always
measure the ladder and name the opt-in when it is off.
`barrier_before_refs` (default off) flushes pending floats ahead
of `\bibliography`: without it, end-queued floats spill into/past
`References` (bio-medsea's Figs 7-14 landed after it). Opt-in so
default weaves stay byte-identical; needs `placeins` (auto-added
to generated heads, required in skeletons/overrides).

## Auto-layout (`ikat layout`, M7.1)

Advisory, not automatic: measures every float and suggests
`span`/`pos`/`width` with one-line reasons carrying numbers.
`--apply` rewrites mermaid-fence and `%% table` attrs in place;
plots are report-only (they ride `plot_attrs`, owned by the caller).
Strict-subset and registry errors propagate with manuscript lines —
a file that cannot weave cannot be measured.

Policy (`[layout]` knobs, IEEE-ish defaults): short tables and
plots pin with `force` ([H] prints at the paragraph); short
diagrams sit with `here` (fail-soft `[h]`→`[ht]`); tall diagrams
take a dedicated `page` — non-lead content taller than
`page_frac` (0.6) of the text height is pre-empted there before
any other test. A section's lead diagram is exempt and gets
`lead_bonus_cm` (1.0) extra in-flow budget instead — section
starts land near fresh columns. Tables wider than `wide_ratio`
(1.25) × the column go `wide`+`top` — unless taller than the
page-top budget (`top_frac` (0.7) × text height): `[t]` cannot
legally hold those, so they take a dedicated `wide`+`page`
instead of queueing toward the document end (a 41-row table
piled three pages at the end until this rule). Column tables
taller than the text height go `page` too (`[H]` would blast
through the page bottom). The third and later `wide` floats
since a section heading earn a spread-the-fences note — page
tops queue behind each other. `width=` shrinks for column
overflow only, never for height (coordinates scale, text does
not: shrinking tall content crowds labels), floored by a crowding
floor measured from placed node labels (`NODE_CHAR_CM` 0.21,
same-rank pairs) and capped at 15% over the fit (flagged when
capped). A `wide` float declared after `late_wide_lines` (15)
prose lines earns a move-the-fence note — page-top floats declared
late strand trailing pages; the engine advises, never reorders.
A `wide`+`top` float with fewer than `tail_lines` (6) body prose
lines remaining takes a dedicated `page`: no downstream tops are
left (it would pile at the end), and a still-pending float meeting
`\FloatBarrier` ejects the column — the placeins eject strands a
half-empty column behind it. Float pages never strand columns.
Skipped in prose-free manuscripts (B1 stubs carry no discussing
text, so their pins stay exact).
External graphics (precompiled diagrams, plot PDFs) keep
hand attrs: size unknown. Full table: `textheight_cm` 23.0,
`column_cm` 8.9, `textwidth_cm` 17.8, `caption_cm` 1.0,
`here_max_cm` 7.0 (showcase-calibrated), `page_frac` 0.6,
`min_width` 0.4, `late_wide_lines` 15, `top_frac` 0.7 (the LaTeX
`topfraction`/`dbltopfraction` default), `tail_lines` 6.

Markdown subset notes (the parser is CommonMark-shaped, the subset
is strict): line-starting `[label]: ...` reference definitions are
a build error (the text would vanish silently — write it inline);
`~~~` fences, indented code, lists, HTML blocks, setext headings,
and H4+ feed their raw lines as paragraph text (as the old scanner
did); `>` quote lines are dropped without breaking paragraphs.

`[typography]` keep-with-next (default ON): `\clubpenalty` +
`\widowpenalty` at 10000, `\needspace{3\baselineskip}` before
every section (`min_lines+1`; `min_lines = 2` default). Set
`keep_with_next = false` to restore free breaks. Never load
`titlesec` under an IEEE class — it breaks sectioning and
`ikat check` / `validate_template` reject it; the built-in
guards are the replacement.

Error format: `path:line: message` + echo (+ `^` caret where
the grammar knows the token). Manuscript lines for content
errors, head-file lines for template/skeleton errors, no line
part for data errors (ragged JSON series etc. — the echo
carries shapes/values). String-API errors keep the Rust
rendering (` --> line N`) verbatim.

## Templates

```toml
[template]
preamble_file = "head.tex"        # level 1: replaces the head (validated)
preamble_append = ["\\usepackage{x}"]  # extra lines before \begin{document}
skeleton = "skel.tex"             # level 3: whole document (exclusive w/ file)
abstract_before_maketitle = true  # ACM/Elsevier/APS top-matter classes
```

Head tokens: `{{{title}}}` `{{{author}}}` `{{{thanks}}}` (empty
`\thanks{}` drops). Skeleton tokens, each exactly once:
`{{body}}`, `{{bibliography}}` (unless `bib_name` empty),
`{{abstract}}` (when the manuscript has one). Unknown/duplicated
tokens are errors; `%` comments are ignored by the counter.
Shipped: 9 heads + `skeleton-plain` (`template_preset(NAME)` →
bib_style + toml snippet).

## LLM agent surface

The same Rust-backed operations are available through four entry points:

- **Python API:** build documents and emit TikZ/pgfplots in process.
- **CLI:** script `ikat build`, `weave`, `check`, chart and flowchart emitters, `standalone`, templates, skeletons, and `ikat skills install` (copies the bundled skill packs into agent skill directories).
- **MCP:** optional stdio tools for connected agents; install `mcp>=1,<2` and run `python -m ikat.mcp_server`.
- **Skills:** focused [Python API](../python/ikat/skills/ikat-api/SKILL.md), [CLI](../python/ikat/skills/ikat-cli/SKILL.md), and [MCP](../python/ikat/skills/ikat-mcp/SKILL.md) guides, plus a general [ikat guide](../python/ikat/skills/ikat/SKILL.md). Install with `ikat skills install` (Claude Code, OpenCode, pi, and Agent Skills directories by default; `--dest DIR` for anything else), or read them from a wired MCP server as `skill://…` resources.

## Python API

```python
from ikat import build_from_paths, BuildSpec, DiagramEntry, bib_keys
from ikat import barchart_to_tikz, lineplot_to_tikz  # legend="auto"
from ikat import compile_pdf, check_tex_env, ensure_tex_packages
```

MCP (`python -m ikat.mcp_server`, stdio): `weave_document`,
`flowchart_to_tikz`, `barchart_to_tikz`, `lineplot_to_tikz`,
`check_tex_packages`, `ensure_tex_packages`, `float_packages`,
`list_templates`, `get_template`, `list_skeletons`,
`get_skeleton`, `version`. `compile_pdf` is CLI-only.

## TeX needed

`pdflatex` (+ `bibtex` with a `.bib`); minimal set: `amsmath
amssymb tabularx graphicx hyperref lmodern textcomp` + class;
+ `tikz` (+`shapes.geometric arrows.meta positioning`) for
diagrams; + `pgfplots` for plots; + `cuted` (sttools bundle) for
over-wide display math. `tex_requirements(tikz, plots)`
reports a body's needs; `ensure_tex_packages` installs them.
arXiv: ship `.tex` + `.bbl` + figure PDFs, no class/style files.

## CI / release

Push/PR: `test` (cargo + pytest, ubuntu/windows/macos) and `tex`
(ubuntu TeX Live builds demo + showcase paper, zero undefined
citations). Tag `v*` (must equal both package versions):
PyPI wheels + crates.io publish in parallel
(`PYPI_API_TOKEN`, `CARGO_REGISTRY_TOKEN` secrets).

*Full docs: `ARCHITECTURE.md` (split + module map),
`ROADMAP.md` (milestones M1–M6), `RESEARCH.md`, `FORMAT-DRAFT.md`
(float/skeleton spec).*
