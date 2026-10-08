"""Side-by-side pixel scores: ikat regen PDFs vs matplotlib originals.

Renders page 0 of each `out/<name>-ikat.pdf` and the matching
`paper/figs/<name>.pdf` to grayscale thumbnails of equal width and
reports the mean absolute pixel difference (0 = identical). This is
a REVIEW aid, not an identity gate: the two toolchains lay out
differently (fonts, figsize, log-x), so scores are recorded in
ACCEPTED.md once (human-reviewed) and `--check` only guards against
regressions past baseline + tolerance.

Usage: `... diff.py` (report) | `... diff.py --check` (exit 1 on drift).
Needs PyMuPDF (fitz).
"""

import json
import sys
from pathlib import Path

HERE = Path(__file__).parent
OUT = HERE / "out"
ORIG = HERE.parent.parent.parent / "paper" / "figs"
SCORES = HERE / "scores.json"
WIDTH, HEIGHT = 240, 180
TOL = 0.02

NAMES = ["fig-fork-flat", "fig-t6-asymmetry", "fig-trunk-branch", "fig-chain-depth"]


def thumb(pdf: Path) -> list[int]:
    try:
        import pymupdf as fitz
    except ImportError:  # legacy namespace (PyMuPDF < 1.24)
        import fitz

    # Fixed canvas both sides (aspect NOT preserved): this is a
    # regression score, not a beauty metric. Ikat-vs-itself over time
    # is exact; ikat-vs-matplotlib is review-once, then pinned.
    doc = fitz.open(pdf)
    r = doc[0].rect
    pix = doc[0].get_pixmap(matrix=fitz.Matrix(WIDTH / r.width, HEIGHT / r.height))
    assert pix.width == WIDTH and pix.height == HEIGHT and pix.n >= 3, pix
    s = pix.samples
    return [sum(s[(y * WIDTH + x) * pix.n:(y * WIDTH + x) * pix.n + 3]) // 3
            for y in range(HEIGHT) for x in range(WIDTH)]


def score(a: list[int], b: list[int]) -> float:
    n = min(len(a), len(b))
    if n == 0:
        return 1.0
    # length-normalize: compare over the shorter canvas
    return sum(abs(x - y) for x, y in zip(a[:n], b[:n])) / n / 255.0


def main() -> int:
    got = {}
    for name in NAMES:
        ikat_pdf, orig_pdf = OUT / f"{name}-ikat.pdf", ORIG / f"{name}.pdf"
        if not ikat_pdf.exists():
            print(f"missing {ikat_pdf} — run regen.py first")
            return 2
        if not orig_pdf.exists():
            print(f"missing original {orig_pdf} — skipped")
            continue
        s = score(thumb(ikat_pdf), thumb(orig_pdf))
        got[name] = round(s, 4)
        print(f"{name}: {s:.4f}")
    if "--check" in sys.argv:
        base = json.loads(SCORES.read_text(encoding="utf-8"))
        bad = [n for n, s in got.items() if s > base.get(n, 0.0) + TOL]
        if bad:
            print(f"DRIFT past baseline+{TOL}: {bad}")
            return 1
        print("within baseline tolerance")
    elif got:
        SCORES.write_text(json.dumps(got, indent=2) + "\n", encoding="utf-8")
        print(f"recorded -> {SCORES} (review PNGs, then commit as baseline)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
