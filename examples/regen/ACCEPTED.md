# Regen acceptance ledger (Phase A.3; engine deltas G.2)

One ledger for all pinned diffs/scores/deltas (standing rule 6).
Reviewed 2026-10-08 against `paper/figs/fig-*.pdf` (matplotlib).
Engine comparison reviewed 2026-10-08 (pdfTeX/TinyTeX vs
tectonic 0.17.0, method + verdict below).

## Baselines (`diff.py --check`, tolerance +0.02)

| figure | score | notes |
|---|---|---|
| fig-fork-flat | 0.1422 | below-legend touches the 45° tick labels — known limitation, see Deltas |
| fig-t6-asymmetry | 0.1125 | clean |
| fig-trunk-branch | 0.1492 | refline end-label clips at right axis edge — known limitation, see Deltas |
| fig-chain-depth | 0.0486 | linear x (matplotlib was log); d=1 sits on the y-axis edge (xmin pin) |

Scores are regression pins (ikat-vs-itself), not identity claims:
the toolchains lay out differently (fonts, figsize, log-x), so the
numbers only say "same as the reviewed run".

## Recorded deltas (reviewed, not hidden)

- **Fig A whiskers degenerate.** `fork_audit` has medians only in
  the results files — mins == maxs == values. No spread was
  measured; none is shown.
- **Symmetric errors.** ikat error bars are `+- (0,e)` with
  `e = max(lo,hi)`: the conservative cover of the observed spread,
  never the exact asymmetric pair.
- **Tick labels flattened.** matplotlib `\n` in group labels is a
  space in presets (the emitter has no multiline ticks).
- **Fig D x axis linear.** matplotlib used log-x; ikat lineplot has
  no log-x. The flat-across-depth story survives; the spacing does
  not. A log-x emitter option is future work, not this phase.
- **Below-legend + 45° labels can touch.** The below row clears
  horizontal tick labels by construction; long rotated labels reach
  into it (fig-fork-flat). Workaround today: shorter labels or an
  explicit corner. A rotation-aware below offset is future work.
- **Refline labels sit at the right end.** A refline ending near
  the right axis edge clips its label (fig-trunk-branch). Same
  class of fix as above; recorded, not silently reworded.

## Staging decision

Regen outputs live in `examples/regen/out/` (+ snapshots in
`snap/`). They are NOT staged into `paper/figs/`: replacing paper
artifacts is the paper track's call (paper track is tracked
elsewhere in ROADMAP). The M2 gate — producible with no
matplotlib in the loop — is met by `regen.py`.

## Standalone proofs (Phase B.1, reviewed 2026-10-08)

`examples/standalone/fig-*.md` (one mermaid fence each, no
hand-tuning) → emitter TikZ → `compile_standalone` → PDF.
Pixel scores vs the staged hand-tuned `paper/figs/tikz/*.pdf`:

| figure | score | delta |
|---|---|---|
| fig-branch-tree | 0.0386 | same tree, gray emitter boxes vs okfill colors |
| fig-ledger-flow | 0.0592 | same pipeline, no dashed group/delta annotations |
| fig-read-questions | 0.0620 | same 3-question fan-in, simpler labels/anchors |
| fig-stratum | 0.1716 | same 4-box stack; hand-tuned dashed group boxes, black boundary bar, and exact captions absent |

The proof is the unbroken path (fence → PDF, zero manual
coordinates), not a pixel match: the staged figures carry
custom colors and `positioning` anchors the emitter does not
produce. Scores are pinned here as the review record.

## Engine deltas (G.2, 2026-10-08): pdfTeX vs tectonic 0.17.0

Both documents build under both engines with 0 undefined
citations (`paper-2026-10-05-tectonic.pdf` 682153 bytes, 22 pp;
`ikat-paper-tectonic.pdf` 131730 bytes, 10 pp — same page counts
and page geometry as pdfTeX). Method: per-page text extraction
compared as whitespace/underscore/hyphen-blind character
sequences (order-sensitive, so any content change shows), plus
the CI log gate on both engines' `.log` files.

- Showcase: ZERO character diffs on all 10 pages. Identical content.
- Main paper: 10/22 pages with micro-diffs, all ratio ≥ 0.9988,
  all in one class — font-encoding extraction, not content:
  (a) underscores (`recorded_at` splits into 2–3 words under
  pdfTeX/Type1 extraction, stays whole under XeTeX);
  (b) OT1-vs-TU quote glyphs (`` `links` `` curls under pdfTeX,
  stays straight under XeTeX — the engine's font stack, same
  source bytes);
  (c) diacritic composition (`Šafárik` extracts decomposed one
  side, composed the other);
  (d) line-break hyphenation points (fonts break lines
  differently; the `-` vanishes in the comparison).
- Verdict: no content delta. Nothing filed against the emitter.
  The `.tex` both engines compile is byte-identical; the
  differences come from the engines' font stacks (Type1/OT1 vs
  OTF/TU), which is precisely what "same source, two engines"
  is allowed to do.
