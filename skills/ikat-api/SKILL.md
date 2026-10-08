---
name: ikat-api
description: >
  ikat Markdown→LaTeX/PDF pipeline via its Python API (build_document,
  build_from_paths, BuildSpec/spec_from_dict, barchart_to_tikz,
  lineplot_to_tikz, compile_pdf, texenv, templates). Use when the task
  needs to weave documents, generate pgfplots figures, or drive TeX
  builds from Python. For shell-only work use the ikat-cli skill; when
  an MCP server is wired use the ikat-mcp skill instead.
---

# ikat skill (Python API)

ikat weaves Markdown (fences, tables, citations) into camera-ready
LaTeX/PDF. Compute lives in a Rust core (`ikat._core`, maturin/PyO3);
Python carries strings and assembles documents. Needs a built module
first: `VIRTUAL_ENV=$PWD/.venv maturin develop`.

## Entry points

| Need | Call |
|---|---|
| Full document (numbering, captions, bib) | `build_document(md_text, toml_src, spec)` or `build_from_paths(md, toml, spec)` → `BuildResult(tex, ...)` |
| Single figure, no registries | `weave_fragment(md, toml_src)` → `[Element]` |
| Flowchart → tikzpicture | `flowchart_to_tikz(src)` — `graph TD/LR` subset + `subgraph` clusters; longest-path layers; crossing edges auto-reroute |
| Build the spec | `spec_from_dict(raw)` — the ONE constructor (CLI `--spec` and MCP `spec_json` share it); unknown keys ignored |
| Bar chart / line plot → tikzpicture | `barchart_to_tikz(...)` / `lineplot_to_tikz(...)`, `legend=` keyword, or `load_preset(path_or_json)` for benchmark JSON |
| Sequence diagram → tikzpicture | `flowchart_to_tikz("sequenceDiagram\n...")` — same entry point, header dispatch; `participant`/`actor`, `->>` solid, `-->>` dashed, `alt`/`else`/`opt`/`end` (strict subset: `loop`/`par`/`Note`/self-msgs error) |
| State diagram → tikzpicture | `flowchart_to_tikz("stateDiagram-v2\n...")` — `[*]` markers, `A --> B [: label]`, `state "Label" as Name`, one composite level (nesting errors) |
| Standalone figure | `compile_standalone(tikz_src, workdir, engine=)` → PDF `Path`; `wrap_standalone(tikz)` → `fig.tex` source; `fence_to_tikz(lang, body)` for `mermaid`\|`chart` fences |
| Compile | `compile_pdf(workdir, main, ensure_packages=False, engine="pdflatex"\|"tectonic")`; raises `CompileError` with the log tail |
| TeX env | `check_tex_env(tex)` report / `ensure_tex_packages(tex)` install+re-probe |
| Templates | `list_templates()`, `template_path(name)`, `template_preset(name)`, `list_skeletons()` |
| Bib keys | `bib_keys(bib_src)` — derive the cited-key set, never freeze it |

## BuildSpec registries (hand-owned, not inferred)

`diagrams=[DiagramEntry(key, caption, mode)]`,
`plots=[(key, caption)]` + `plot_dir` + `plot_insert_before` +
`plot_attrs={key: "pos=bottom"}`,
`table_captions=[...]` + `table_specs={index: "XXl"}`,
`title_thanks/author/bib_name/bib_style/bib_keys`,
`preamble_override/preamble_append/skeleton` (level-1 vs level-3,
mutually exclusive), `graphicspaths`. Renaming a caption cannot
desynchronize the document — the spec names it, not the markdown.

## Legend keyword

`legend="auto"` (default: first collision-free inside corner in
TL,TR,BR,BL order, else the below-axis row) | `"below"` | four
corners | `"outside-right"`. Anything else is `ValueError` naming
the set.

## Presets

`load_preset(path_or_src)`: benchmark JSON → tikzpicture. File
path or raw JSON string; errors name the file. Schema: `kind`
`bar`|`line`, `title`, `ylabel` (+ `xlabel` for lines); bar
fields `group_labels series_names values mins maxs` (+ optional
`log_y refline:[x0,x1,y,label]`), line fields `xs series_names
yss errs`. Optional `legend` (same keywords), optional
`footnote` (rendered `{\footnotesize …\par}` after the picture).
Validation: rectangular series, `min ≤ value ≤ max` elementwise,
`xs` strictly increasing — violations name the file and key.

## Doctrine (do not work around)

- Strict subset, fail-fast: dangling citation keys, unknown
  template/skeleton tokens, wide+here floats, unknown attrs/keys,
  line-starting `[label]: ...` reference definitions
  all raise — never catch-and-continue into a TeX log mystery.
- No invented numbers in figures: plot data comes from measurement
  (grep/wc/benches), marked `[NOT YET MEASURED]` otherwise.
- `compile_pdf` is workdir-bound and slow: batch builds, don't
  call it per figure.
- Full surface map: `docs/QUICKREF.md`. Milestones: `docs/ROADMAP.md`.
