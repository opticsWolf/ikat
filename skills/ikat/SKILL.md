---
name: ikat
description: Use ikat to weave Markdown papers into LaTeX or PDF, generate Mermaid flowcharts as TikZ, create pgfplots charts, and select the Python API, CLI, or MCP interface.
---

# ikat

Use ikat when a task involves building a Markdown manuscript with LaTeX-native diagrams or plots. Ikat's Rust core handles parsing, layout, and code generation; Python provides the document API and command wrappers.

## Choose an interface

- **Python API:** use `build_document` or `build_from_paths` for programmatic builds; use `flowchart_to_tikz`, `barchart_to_tikz`, and `lineplot_to_tikz` for individual figures.
- **CLI:** use `ikat build` for a manuscript, `ikat weave` when only `.tex` is needed, `ikat check` for TeX package checks, and the figure/template/skeleton commands for focused tasks.
- **MCP:** when the client already has ikat configured, use its stdio tools for weaving, TikZ/pgfplots generation, package checks, templates, and skeletons. Start the optional server with `python -m ikat.mcp_server`; it requires the `mcp>=1,<2` extra.
- **This skill:** use these instructions to choose among the interfaces and respect ikat's supported syntax and build boundaries. Install the `skills/ikat/` directory in the agent's configured skill location; the file itself does not install or run ikat.

## Build workflow

1. Inspect the Markdown source, `ikat.toml`, citation file, and any `BuildSpec` data before changing a manuscript.
2. Prefer the Python API for an in-process build, MCP for an already-connected agent workflow, and the CLI for shell automation or PDF compilation.
3. Use `ikat weave` or `ikat build --no-pdf` to emit LaTeX without compiling. A PDF build needs a local `pdflatex`; staged bibliographies also need `bibtex`.
4. Read the complete build result and compiler diagnostics. Do not claim a PDF was produced unless compilation succeeded.

## Supported inputs and boundaries

- Mermaid support is a strict flowchart subset (`graph TD` / `graph LR`); do not invent support for other diagram types or syntax. Let ikat reject unsupported syntax.
- Bar and line plot helpers emit editable pgfplots/TikZ source; preserve their input data and generated source rather than substituting screenshots.
- Use the shipped templates and skeletons when their documented requirements fit. Check TeX dependencies before compiling when needed.
- `compile_pdf` is a Python/CLI operation, not an MCP tool; use the CLI for workdir-bound PDF compilation.
- Do not invent benchmark results, paper measurements, citations, or distribution availability.
