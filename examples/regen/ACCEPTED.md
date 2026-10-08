# Regen acceptance ledger (Phase A.3)

One ledger for all pinned diffs/scores/deltas (standing rule 6).
Reviewed 2026-10-08 against `paper/figs/fig-*.pdf` (matplotlib).

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
