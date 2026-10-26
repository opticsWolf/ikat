auto-proof/ — ikat auto-layout deliverables (Phase H, v0.10.0).

  hand-weaved-ikat-paper.pdf   showcase woven with hand-tuned attrs
                              (examples/ikat-paper/out/ikat-paper.pdf)
  autolayout-ikat-paper.pdf   same manuscript, attrs from `ikat layout --apply`
                              (rebuilt by make-auto-paper.py -> build-ikat/)
  bench-<id>.pdf (x10)        bench-layout stubs (PLOS image rects + JATS
                              table shapes) through suggest -> apply -> weave
                              -> pdflatex. Probe content: chain diagrams with
                              1-char labels ("Fig i." captions) + uniform
                              tables — layout proof, not papers.
  make-auto-paper.py          rebuilds build-ikat/ + autolayout PDF
  build/                      per-paper weave+compile workdirs (.tex kept)
  build-ikat/                 applied showcase workdir (.tex kept)

All PDFs compile with 0 overfull. The hand and auto showcase papers
differ only in Fig 2 width (0.58 vs floor-bound 0.57) and Table 3
pinning (barrier vs force).
  full/                   full-content md per paper (<id>.md + <id>.bib,
                          figs/*.png) from JATS XML + PDF rasters via
                          export_full.py; out/<id>.pdf are weave proofs
                          (10 for 10). See full/README.md + REPORT.md.
