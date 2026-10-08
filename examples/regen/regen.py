"""Regenerate the four main-paper plots through ikat (Phase A.3).

For each `fig-*.json` fixture: `load_preset` → TikZ, byte-compared
against the committed snapshot in `snap/` (CI-safe deterministic
gate — emitter drift fails here, not in a pixel diff), then wrapped
standalone and compiled to `out/fig-*.pdf` (needs pdflatex).

Usage: `.venv/Scripts/python.exe examples/regen/regen.py [--no-pdf]`
`--update-snaps` rewrites the snapshots (review the diff after).
"""

import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).parent
ROOT = HERE.parent.parent
sys.path.insert(0, str(ROOT / "python"))

from ikat import load_preset, tex_requirements  # noqa: E402

SNAP = HERE / "snap"
OUT = HERE / "out"
FIGS = ["fig-fork-flat", "fig-t6-asymmetry", "fig-trunk-branch", "fig-chain-depth"]


def standalone(tikz: str) -> str:
    # Same tight-page wrapper as the showcase build (preview AFTER
    # pgfplots — load-order quirk, proven by probe).
    req = tex_requirements("tikzpicture" in tikz, True)
    lines = ["\\documentclass{article}", "\\usepackage[utf8]{inputenc}"]
    lines += req
    lines += [
        "\\usepackage[active,tightpage]{preview}",
        "\\PreviewEnvironment{tikzpicture}",
        "\\setlength\\PreviewBorder{4pt}",
    ]
    return "\n".join(lines + ["\\begin{document}", tikz, "\\end{document}"])


def main() -> int:
    update = "--update-snaps" in sys.argv
    no_pdf = "--no-pdf" in sys.argv
    OUT.mkdir(exist_ok=True)
    SNAP.mkdir(exist_ok=True)
    failed = 0
    for name in FIGS:
        tikz = load_preset(HERE / f"{name}.json")
        snap = SNAP / f"{name}.tex"
        if update or not snap.exists():
            snap.write_text(tikz, encoding="utf-8")
            print(f"snap {name} (written)")
        elif snap.read_text(encoding="utf-8") != tikz:
            print(f"SNAPSHOT DRIFT: {name} (emitter output changed — review, then --update-snaps)")
            failed += 1
        else:
            print(f"snap {name} ok")
        if not no_pdf:
            src = OUT / f"{name}.tex"
            src.write_text(standalone(tikz), encoding="utf-8")
            p = subprocess.run(
                ["pdflatex", "-halt-on-error", "-interaction=nonstopmode", src.name],
                cwd=OUT, capture_output=True, text=True,
            )
            if p.returncode != 0:
                print(f"pdflatex failed on {name}:\n{(p.stdout + p.stderr)[-1500:]}")
                return 2
            (OUT / f"{name}-ikat.pdf").unlink(missing_ok=True)
            (OUT / f"{name}.pdf").rename(OUT / f"{name}-ikat.pdf")
            print(f"pdf  {name} ok")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
