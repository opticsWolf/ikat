# ikat — Quick Reference

**v0.2.0 · md → tex/pdf pipeline, Rust core + thin Python**

## Showcase paper

[Read the exported ikat paper (PDF)](../site/assets/ikat-paper.pdf). Its [Markdown source and build instructions](../examples/ikat-paper/) are also in the repository.

## Install & build

```bash
uv venv .venv && uv pip install -p .venv maturin pytest "mcp>=1,<2"
VIRTUAL_ENV=$PWD/.venv maturin develop   # builds ikat._core in place
cargo test                               # 59 Rust tests
.venv/Scripts/pytest tests/              # 45 pytest (windows; pytest tests/ elsewhere)
```

## CLI (`ikat`)

```bash
ikat build DOC.md --toml ikat.toml --outdir out [--spec S.json] [--bib R.bib]
     [--engine pdflatex|tectonic] [--bib-style B] [--ensure-packages] [--no-pdf]
ikat weave DOC.md --toml ikat.toml --spec S.json > doc.tex   # no TeX needed
ikat check DOC.tex [--install] [--verbose]                   # exit 1 if missing
ikat templates | ikat template NAME [--show-preset]
ikat skeletons | ikat skeleton skeleton-plain
printf 'graph TD\na[x]-->b[y]\n' | ikat flowchart -
echo '{...}' | ikat barchart - [--legend WORD]    # JSON payload, stdout tikzpicture
echo '{...}' | ikat lineplot - [--legend WORD]
ikat standalone FIG.md --out fig.pdf             # one mermaid|chart fence → PDF
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
| `graph TD/LR/...` | directions TD/TB/LR/RL/BT; nodes `[]` `{}` `([])` `[[]]`; edges `-->`, `---`, `==>` with `\|label\|`; chains, `;`, `%%`, `<br/>` | other directions/shapes, subgraphs, styling |
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
`[floats]` tuning (fractions, counters, `barrier_sections`) emits
only when non-default.

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
- **CLI:** script `ikat build`, `weave`, `check`, chart and flowchart emitters, `standalone`, templates, and skeletons.
- **MCP:** optional stdio tools for connected agents; install `mcp>=1,<2` and run `python -m ikat.mcp_server`.
- **Skills:** focused [Python API](../skills/ikat-api/SKILL.md), [CLI](../skills/ikat-cli/SKILL.md), and [MCP](../skills/ikat-mcp/SKILL.md) guides, plus a general [ikat guide](../skills/ikat/SKILL.md). Install a chosen skill directory in the agent's configured skill location.

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
diagrams; + `pgfplots` for plots. `tex_requirements(tikz, plots)`
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
