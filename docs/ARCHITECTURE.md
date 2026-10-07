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
| flowchart grammar + layout + TikZ | `src/mermaid.rs` | cargo test |
| legend auto-placement + geometry | `src/plot.rs` (`LegendPos`, collision test) | cargo test |
| tables / bib utils | `src/table.rs` | cargo test |
| block parse + document assembly | `src/doc.rs` | cargo test + golden |
| inline / citations / escaping | `src/esc.rs` | cargo test |
| bar/line + whiskers + log-y + refline | `src/plot.rs` | cargo test |
| `ikat.toml` model + validation | `src/config.rs` | cargo test |
| PyO3 bindings | `src/lib.rs` | pytest |
| preamble scan + file→tlmgr map | `src/texenv.rs` | cargo test |
| tectonic engine cascade | `src/tectonic.rs` | cargo test |
| pulldown-cmark spike (tests only) | `src/md_spike.rs` | cargo test |
| Element / spans / fences / floats | `python/ikat/pipeline.py` | pytest |
| BuildSpec assembly + templates | `python/ikat/document.py` | pytest |
| pdflatex/bibtex driver | `python/ikat/compile.py` | manual + CI tex job |
| kpsewhich/tlmgr probe + install | `python/ikat/texenv.py` | pytest |
| shipped heads + presets + skeletons | `python/ikat/templates/` + `template_preset` | pytest + headsproof |
| unified CLI | `python/ikat/cli.py` (`ikat` script) | pytest test_surface |
| MCP server (FastMCP stdio) | `python/ikat/mcp_server.py` | pytest test_surface (skips w/o extra) |

## Roadmap

The plan of record is `docs/ROADMAP.md` (milestones M1–M6 with
acceptance gates). Done: pipeline parity + golden master (M1, M4.4),
floats (M5.1), skeletons (M5.2), CLI/MCP/surface parity (M4.2, M4.8),
legend auto-placement (M2.4), CI + release workflows (M4.2b).
The open frontier: benchmark-JSON presets and plot regeneration
(M2.1, M2.2), figure subsets + layout (M3), the pulldown-cmark
switch (M5.4), line-numbered errors (M4.1), and shipping itself
(M4.3 — names claimed, tokens pending).
