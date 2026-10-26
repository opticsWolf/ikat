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

Hand and auto showcases now AGREE (M7.10b era): the hand source
ports the engine's winning attrs (tables span=column pos=force
captionpos=bottom, skeleton-contract table span=wide pos=top,
Fig 2 at the 0.57 floor), so both weave BYTE-IDENTICAL tex — 7
pages, 0 overfull, final columns full into the References. Before
the port the hand build predated M7.2/M7.4/M7.5 and left a sparse
page 8; the hand PDF is the same document now, attrs written by
hand instead of suggested.
