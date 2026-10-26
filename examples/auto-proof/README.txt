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

Hand vs auto showcase differ only in Fig 2 width (0.58 vs floor-bound
0.57) and Table 3 pinning (barrier vs force); Table 3's fence sits at
the section-5 head so table* takes the p7 top (late-wide rule).
