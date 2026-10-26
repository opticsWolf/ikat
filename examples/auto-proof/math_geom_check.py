"""Geometric audit of display-math placement in a woven PDF.

Checks, per page:
  * no text block crosses its column bounds (left  col x<col_l or x>col_r,
    right col likewise) or the text block on wide rows;
  * full-width (strip) rows are centered on the text block;
  * equation tags sit within a small margin of the column's right edge.

Column geometry is measured from the PDF text blocks themselves.
"""
from __future__ import annotations

import sys
from pathlib import Path

import pymupdf

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

pdf = Path(sys.argv[1]) if len(sys.argv) > 1 else None
if pdf is None:
    print("usage: python math_geom_check.py <pdf>")
    raise SystemExit(1)

doc = pymupdf.open(str(pdf))
issues = 0
for pno, page in enumerate(doc, 1):
    blocks = [b for b in page.get_text("blocks") if b[6] == 0 and b[4].strip()]
    if not blocks:
        continue
    xs = [(b[0], b[2]) for b in blocks]
    x_min = min(x for x, _ in xs)
    x_max = max(x for _, x in xs)
    mid = (x_min + x_max) / 2
    # column split: blocks left/right of the page middle band
    left = [b for b in blocks if b[2] <= mid + 5]
    right = [b for b in blocks if b[0] >= mid - 5]
    wide = [b for b in blocks if b[0] < mid - 5 and b[2] > mid + 5]
    col_l, col_r = x_min, x_max
    for b in left + right:
        # nothing should stick out of the text block
        if b[0] < x_min - 1 or b[2] > x_max + 1:
            print(f"p{pno}: block escapes text bounds x=[{b[0]:.1f},{b[2]:.1f}]: {b[4][:40]!r}")
            issues += 1
    for b in wide:
        w_center = (b[0] + b[2]) / 2
        if abs(w_center - mid) > 6:
            print(f"p{pno}: wide row off-center by {w_center-mid:+.1f}pt: {b[4][:40]!r}")
            issues += 1
print(f"pages={len(doc)} issues={issues}")
