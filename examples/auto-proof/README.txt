auto-proof/ — ikat auto-layout deliverables (Phase H, v0.10.0+).

  hand-weaved-ikat-paper.pdf   showcase woven with hand-tuned attrs
                               (copy of examples/ikat-paper/out/ikat-paper.pdf)
  autolayout-ikat-paper.pdf    same manuscript, attrs from `ikat layout --apply`
                               (copy of build-ikat/ikat-paper.pdf)
  make-auto-paper.py           rebuilds build-ikat/ + autolayout PDF
  build-ikat/                  applied showcase workdir (.tex kept)
  export_full.py               full-content md export of the 10 bench papers
                               (JATS XML text/tables + PDF-extracted PNGs)
  full/                        per-paper <id>.md + <id>.bib + figs/*.png;
                               out/<id>.pdf are weave proofs (10 for 10).
                               See full/README.md + full/REPORT.md.

Stale stub-era artifacts (bench-*.pdf probes, build/ workdirs, render
scratch) were removed 2026-10-26; full/ supersedes them.

Auto layout now OUTPACKS the hand build (M7.2 top-fit, M7.4 tail
page, M7.5 compaction ladder landed after the hand pass): 7 pages
vs 8, both 0 overfull — the auto run pins Table 3 wide-top + the
tables column-force with captionpos=bottom, so page 7 runs both
columns full into the References while the hand build leaves a
sparse page 8 (orphan paragraph + refs). Fig 2 keeps the floor
width (0.57 vs hand 0.58) at zero cost. The hand PDF stays as the
historical hand-tuned reference.
