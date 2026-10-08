---
name: ikat-mcp
description: >
  ikat Markdown→LaTeX/PDF pipeline via its FastMCP stdio server
  (weave_document, flowchart_to_tikz, barchart_to_tikz,
  lineplot_to_tikz, standalone_figure, check/ensure/float packages,
  list/get templates and skeletons, version). Use when the task needs to weave documents
  or emit TikZ/pgfplots and okf-style MCP tools are wired. Prefers
  these tools over shelling out. For shell-only environments use the
  ikat-cli skill; for in-process work use the ikat-api skill.
---

# ikat skill (MCP)

```jsonc
// client config
{ "mcpServers": { "ikat": {
  "command": "/path/to/.venv/Scripts/python.exe",
  "args": ["-m", "ikat.mcp_server"],
  "cwd": "/path/to/ikat" } } }
```

## Which tool when

| Need | Tool |
|---|---|
| Weave markdown → `.tex` | `weave_document(md_text, toml_text, spec_json)` — `spec_json` is BuildSpec JSON, same shape as CLI `--spec` |
| Mermaid fence → tikzpicture | `flowchart_to_tikz(src)` — flowchart, `sequenceDiagram`, or `stateDiagram-v2` (header dispatch), strict subset, errors name the statement |
| One fence → standalone source | `standalone_figure(md_text)` → `{lang, tex, needs}` — the WRAPPED `fig.tex`, not PDF bytes; the caller compiles (same workdir rule as `compile_pdf`) |
| Bar / line data → tikzpicture | `barchart_to_tikz(...)` / `lineplot_to_tikz(...)`, `legend=` keyword (`auto` default; unknown words error naming the set), `preset_json=` (preset document string; when set, explicit data args ignored) |
| What does this `.tex` need | `check_tex_packages(tex_source)` → `{missing, ok}` |
| Install what's missing | `ensure_tex_packages(tex_source, install=true)` |
| Float-only needs | `float_packages(tex_source)` — `placeins/float/dblfloatfix` for `force`/`barrier`/wide+bottom |
| Discover heads / skeletons | `list_templates()` (heads+skeletons) or `list_skeletons()` |
| Fetch one | `get_template(name)` → `{head, bib_style, toml}`; `get_skeleton(name)` → `{skeleton}` |
| Pin the version | `version()` |

## Results and errors

- Tools return values directly (no okf-style envelope): strings,
  dicts, lists. Failures raise with the multi-line rendering a
  CLI user would see (`path:line: message` + echo + `^` caret
  where known; no file prefix on string inputs — the Rust
  ` --> line N` form crosses verbatim): dangling citation keys,
  unknown tokens/attrs/legend words, registry misses.
- There is deliberately NO `compile_pdf` tool (workdir-bound,
  minutes-long): weave here, compile via the `ikat build` CLI.
- Unknown `spec_json` keys are ignored; missing registries fail
  (`mermaid fence #N has no registry entry` → pass `diagrams`
  entries). Templates that drop a needed package fail naming it.
- Full surface map: `docs/QUICKREF.md` (API × CLI × MCP table).
