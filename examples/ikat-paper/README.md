# ikat showcase paper

A short paper about ikat that is its own demo: every construct in
`ikat-paper.md` exercises a real pipeline feature. Rebuild with:

```text
.venv/Scripts/python.exe examples/ikat-paper/build.py
```

needs `pdflatex` + `bibtex` on PATH. Outputs land in `out/`
(git-ignored): two plot PDFs, the woven `.tex`, the staged `.bib`
(+ local `IEEEtran.bst`, never bundled for arXiv), and the final PDF.

[Read the committed ikat showcase PDF](../../site/assets/ikat-paper.pdf).

## Features exercised

Blocks: `#` title, `## N.` numbered sections (numbers stripped),
`###` subsections, `## Abstract` env, multi-line paragraphs,
inline mermaid (`mode="inline"`) with `{span,pos,width}` attrs,
`%% table {pos,captionpos,span}` directives (incl. `table*` wide),
pipe tables with a `table_specs` override.
Inline: `**bold**`, `*italic*`, `` `code` ``, `$math$`, unicode
(— – … → ≈ µ ₂ § ✓), TeX escapes, both citation forms
(`[text: \`key\`]` and `` [`key`] ``), `†` note paragraphs.
Config: `[spans]`, `[floats]` fractions + counters, `[template]`
skeleton + `abstract_before_maketitle`, `plot_attrs` registry,
`plot_insert_before`, `bib_keys` derived from `refs.bib`.

## Data provenance (no invented numbers)

- `fig-loc`: `wc -l` per Rust module at the build commit.
- `fig-tests`: `#[test]` / `def test` counts per commit via
  `git grep <rev>`, plotted in build order, no smoothing.
- Suite counts in Table 2: `cargo test` / `pytest` as run.
- Bibliography: title + URL + access date only; no authors guessed.
