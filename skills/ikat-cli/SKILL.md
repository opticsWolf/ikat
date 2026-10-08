---
name: ikat-cli
description: >
  ikat Markdown→LaTeX/PDF pipeline via its `ikat` console script
  (build, weave, check, templates/template, skeletons/skeleton,
  flowchart, barchart, lineplot, standalone, version). Use when the task needs to
  weave or compile documents, check TeX packages, or emit TikZ/pgfplots
  from the shell with no Python written. For in-process work use the
  ikat-api skill; when an MCP server is wired use the ikat-mcp skill.
---

# ikat skill (CLI)

Thirteen subcommands, each a thin call into the Python API. Needs the
built module on PATH (`maturin develop` installs the `ikat` script).

## Commands

```bash
ikat build DOC.md --toml T.toml --outdir OUT [--spec S.json] [--bib R.bib]
     [--engine pdflatex|tectonic] [--bib-style B] [--ensure-packages] [--no-pdf]
ikat weave DOC.md --toml T.toml --spec S.json > doc.tex   # stdout, no TeX needed
ikat check DOC.tex [--install] [--verbose]                # exit 1 if anything missing
ikat templates | ikat template NAME [--show-preset]
ikat skeletons | ikat skeleton skeleton-plain
printf 'graph TD\na[x]-->b[y]\n' | ikat flowchart -   # flowcharts, sequenceDiagram, stateDiagram-v2 (header dispatch)
echo '{...}' | ikat barchart - [--legend WORD]            # JSON payload → tikzpicture
echo '{...}' | ikat lineplot - [--legend WORD]
ikat barchart --preset fig.json [--legend WORD]         # preset file (or - for stdin)
ikat lineplot --preset fig.json [--legend WORD]
ikat standalone FIG.md --out fig.pdf [--engine ...]        # one mermaid|chart fence → PDF
ikat version
```

- `--spec` is BuildSpec JSON (`diagrams/plots/table_captions/bib_style/...`).
- `--bib refs.bib` derives the allowed citation keys (dangling keys fail).
- `--engine tectonic` uses the `TECTONIC_EXE`/PATH binary cascade.
- Plot payloads: barchart needs `title ylabel group_labels series_names
  values mins maxs` (+ optional `log_y refline:[x0,x1,y,label] legend`);
  lineplot needs `title xlabel ylabel xs names yss errs` (+ optional
  `legend`). `--legend` overrides the payload.
- `--preset FILE|-`: benchmark-JSON preset instead of a payload
  (`kind bar|line`, same schema as `load_preset` in ikat-api).
  `FILE` + `--preset` together is an error naming both;
  `--legend` overrides the preset's `legend` key.
- `check` covers float packages too (`placeins/float/dblfloatfix`).

## Errors

Non-zero exit, multi-line rendering printed verbatim on stderr:
`path:line: message` + echo (+ `^` caret where the grammar
knows the token). Manuscript lines for content errors (diagram
statements, dangling keys, table attrs), head-file lines for
template/skeleton errors, no line part for data errors (ragged
JSON series — the echo carries shapes). Same catalog as ever:
bad JSON payload, unknown legend word (names the set), missing
registry entry (`mermaid fence #N …` → add `--spec` with a
`diagrams` entry), template dropping a needed package (names
it), missing binary (names the install). Nothing fails inside
a TeX log that could have failed here.
