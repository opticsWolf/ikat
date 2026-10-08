"""Standalone proofs (Phase B.1): the four paper diagrams, no hand-tuning.

For each `fig-*.md`: single mermaid fence -> emitter TikZ ->
`compile_standalone` -> `out/<name>.pdf`. Pure emitter output;
the staged `paper/figs/tikz/*.pdf` are hand-tuned (custom colors,
positioning anchors), so pixel scores vs those originals are
recorded in `../regen/ACCEPTED.md` with the style deltas — the
proof is the unbroken path, not a pixel match.

Usage: `.venv/Scripts/python.exe examples/standalone/proofs.py`
"""

import sys
from pathlib import Path

HERE = Path(__file__).parent
ROOT = HERE.parent.parent
sys.path.insert(0, str(ROOT / "python"))

from ikat import compile_standalone, fence_to_tikz  # noqa: E402
from ikat.pipeline import extract_fences  # noqa: E402

OUT = HERE / "out"
NAMES = ["fig-branch-tree", "fig-ledger-flow", "fig-read-questions", "fig-stratum"]


def main() -> int:
    OUT.mkdir(exist_ok=True)
    for name in NAMES:
        md = (HERE / f"{name}.md").read_text(encoding="utf-8")
        fences = [(l, b) for l, _, b in extract_fences(md) if l in ("mermaid", "chart")]
        assert len(fences) == 1, f"{name}: need exactly 1 fence, got {len(fences)}"
        tikz = fence_to_tikz(*fences[0])
        (OUT / f"{name}.tex").write_text(tikz, encoding="utf-8")
        pdf = compile_standalone(tikz, OUT, engine="pdflatex")
        (OUT / f"{name}.pdf").unlink(missing_ok=True)  # Windows rename won't overwrite
        pdf.rename(OUT / f"{name}.pdf")
        print(f"pdf  {name} ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
