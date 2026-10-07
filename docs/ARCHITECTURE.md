# ikat architecture

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

## Why this split

- **Rust owns everything computable.** Parsing, layout, numeric
  formatting, and code generation are deterministic byte-level work —
  exactly what Rust is good at, and exactly what must never differ
  between runs. Python never post-processes Rust output, only embeds it.
- **Python owns documents and tools.** Fence extraction, span
  resolution, float assembly, and driving `pdflatex` are glue: thin,
  readable, replaceable.
- **No matplotlib, ever.** Data graphics compile inside the document's
  own TeX run (pgfplots), so figure fonts are document fonts by
  construction. The pipeline cannot produce a font-mismatched figure.
- **No Node.js.** Mermaid renders through our own flowchart-subset
  parser, not mermaid-cli + browser + SVG round-trip.

## Spanning model

Two-column classes (`IEEEtran`, `acmart`, …) distinguish `figure`
(one column) from `figure*` (both). ikat resolves per element:

`ikat.toml [spans]` kind default → ```` ```mermaid {span=…} ````
fence attribute → explicit call-site argument.

Defaults (`diagram = "wide"`, `plot/table = "column"`) encode the
lesson from the Macrame paper: flowcharts need text width, data
graphics hold a column.

## Module map

| piece | lives in | tested by |
|---|---|---|
| flowchart grammar + layout + TikZ | `src/mermaid.rs` | `cargo test` (10) |
| tables / bib utils | `src/table.rs` | `cargo test` (4) |
| block parse + document assembly | `src/doc.rs` | `cargo test` (2) + golden |
| inline / citations / escaping | `src/esc.rs` | `cargo test` (5) |
| bar/line + whiskers + log-y + refline | `src/plot.rs` | `cargo test` |
| `ikat.toml` model | `src/config.rs` | `cargo test` |
| PyO3 bindings | `src/lib.rs` | pytest |
| Element / spans / fences / floats | `python/ikat/pipeline.py` | pytest |
| pdflatex/bibtex driver | `python/ikat/compile.py` | manual (needs TeX) |
| preamble scan + file→tlmgr map | `src/texenv.rs` | `cargo test` (6) |
| kpsewhich/tlmgr probe + install | `python/ikat/texenv.py` | pytest (7+fakes) |
| shipped heads + presets | `python/ikat/templates/` + `template_preset` | pytest + headsproof (needs TeX) |
| unified CLI | `python/ikat/cli.py` (`ikat` script) | pytest test_surface |
| MCP server (FastMCP stdio) | `python/ikat/mcp_server.py` | pytest test_surface (skips w/o extra) |

## Roadmap

1. Port the paper's `build-paper.py` stages (tables, citations,
   unicode, sectioning) onto `Element`s — pipeline.py grows, Rust
   untouched.
2. Plot presets from benchmark JSON (the paper's `results/*.json`
   shape → `barchart_to_tikz` directly).
3. Standalone-TikZ export (`\documentclass[tikz]`) for arXiv-safe
   precompiled figures.
4. Sequence/state mermaid subsets, behind the same `flowchart_to_tikz`
   entry point.
