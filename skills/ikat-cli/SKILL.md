---
name: ikat-cli
description: >
  ikat Markdown→LaTeX/PDF pipeline via its `ikat` console script
  (build, weave, check, templates/template, skeletons/skeleton,
  flowchart, barchart, lineplot, version). Use when the task needs to
  weave or compile documents, check TeX packages, or emit TikZ/pgfplots
  from the shell with no Python written. For in-process work use the
  ikat-api skill; when an MCP server is wired use the ikat-mcp skill.
---

# ikat skill (CLI)

Twelve subcommands, each a thin call into the Python API. Needs the
built module on PATH (`maturin develop` installs the `ikat` script).

## Commands

```bash
ikat build DOC.md --toml T.toml --outdir OUT [--spec S.json] [--bib R.bib]
     [--engine pdflatex|tectonic] [--bib-style B] [--ensure-packages] [--no-pdf]
ikat weave DOC.md --toml T.toml --spec S.json > doc.tex   # stdout, no TeX needed
ikat check DOC.tex [--install] [--verbose]                # exit 1 if anything missing
ikat templates | ikat template NAME [--show-preset]
ikat skeletons | ikat skeleton skeleton-plain
printf 'graph TD\na[x]-->b[y]\n' | ikat flowchart -
echo '{...}' | ikat barchart - [--legend WORD]            # JSON payload → tikzpicture
echo '{...}' | ikat lineplot - [--legend WORD]
ikat version
```

- `--spec` is BuildSpec JSON (`diagrams/plots/table_captions/bib_style/...`).
- `--bib refs.bib` derives the allowed citation keys (dangling keys fail).
- `--engine tectonic` uses the `TECTONIC_EXE`/PATH binary cascade.
- Plot payloads: barchart needs `title ylabel group_labels series_names
  values mins maxs` (+ optional `log_y refline:[x0,x1,y,label] legend`);
  lineplot needs `title xlabel ylabel xs names yss errs` (+ optional
  `legend`). `--legend` overrides the payload.
- `check` covers float packages too (`placeins/float/dblfloatfix`).

## Errors

Non-zero exit with `ikat <cmd>: <message>` on stderr: bad JSON
payload, unknown legend word (names the set), missing registry
entry (`mermaid fence #N has no registry entry` → add `--spec`
with a `diagrams` entry), template dropping a needed package
(names it), missing binary (names the install). Nothing fails
inside a TeX log that could have failed here. Line numbers on
errors are still open work (M4.1) — messages are context-free
strings today.
