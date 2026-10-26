# full/ — full-content md export of the 10 bench papers

Each paper directory holds the **complete paper as Markdown plus pictures**,
exported from the bench fetch cache (`../bench-layout/<id>.xml` JATS for
structure/captions/tables/refs + `../bench-layout/<id>.pdf` for raster
figures):

- `<id>.md` — full text (sections, paragraphs), real JATS tables as md
  tables, and one ` ```mermaid ` fence per figure (precompiled: the fence
  body is the PLOS graphic id, the PNG rides the weaver registry).
- `<id>.bib` — references from the JATS ref-list (`@misc` notes carry the
  full mixed-citation text; no fields invented).
- `figs/*.png` — extracted PDF rasters in reading order, matched to the
  JATS figures in body order (regenerate with the script; git-ignored).
- `<id>-auto.md`, `<id>.tex` — engine-applied source + woven output
  (regenerable proof intermediates; git-ignored).
- `out/<id>.pdf` — compiled weave proof, 10 for 10 (binaries; ignored).

Regenerate: `python examples/auto-proof/export_full.py [paper-id...]`
(needs the bench-layout fetch cache + TinyTeX on PATH).

## Fidelity notes (honest limits)

- Pictures are measured, not guessed: the exporter pins each fence with
  engine-mirroring thresholds (short `force`, wide `top`, tall `page`).
  The engine keeps them as-is; `applied` counts in REPORT.md are tables
  (and the two showcase-style width floors) only.
- Dense numeric tables overflow narrow X columns (econ 441, psych 200,
  math 219 overfulls): real JATS content, tabularx limits, kept full.
- `med-ioannidis-2005` tables live in JATS as graphics, not text:
  exported as honest prose placeholders, not invented cells.
- Front matter is complete: every contrib (given names + surnames)
  and every affiliation with its contrib→aff xref marks rides the
  spec (`authors`/`affiliations`/`author_affils`); the woven head
  wraps them in a parbox with numeric superscript marks (M7.10).
- Inline/displayed math flattens to linear `$..$` text (sub/sup kept);
  `dropped.formula` + `folded`/`leftovers` in REPORT.md audit every loss.
- Non-ASCII folds to ASCII/TeX math deterministically (`FOLD` table in
  the script); the escaper owns `— – … × µ` and friends (PASS set).
- The `\\[` trap (table row starting with `[`), JATS `[12]` brackets
  around cites, and `Fig 2.` caption echoes are all normalized.
