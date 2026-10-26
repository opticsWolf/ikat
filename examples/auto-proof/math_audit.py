"""Cross-check the engine's display-math decision against TeX truth.

For every display span in an exported bench markdown:
  * true width, measured by TeX (`\sbox` of `$\displaystyle ...$`)
  * the engine's decision (column / strip / strip+aligned), taken from the
    woven `.tex`
  * the width of each row the engine actually emitted (measured by TeX)

Blocks are matched to source spans by content, not by index, so a matrix
span that emits several strips still lines up.
"""
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

FULL = Path(__file__).resolve().parent / "full"
COLUMN_PT = 8.9 * 72.27 / 2.54
TEXT_PT = 17.8 * 72.27 / 2.54
TAG_RE = re.compile(r"\\tag\{[^}]*\}")


def _letters(i: int) -> str:
    s = ""
    i += 1
    while i:
        i, r = divmod(i - 1, 26)
        s = chr(ord("a") + r) + s
    return s


def measure(items: list[str], workdir: Path) -> list[float | None]:
    workdir.mkdir(parents=True, exist_ok=True)
    tex = workdir / "probe.tex"
    lines = [
        r"\documentclass{article}",
        r"\usepackage{amsmath,amssymb}",
        r"\usepackage[utf8]{inputenc}",
        r"\begin{document}",
    ]
    for i, s in enumerate(items):
        name = "\\wbox" + _letters(i)
        lines.append(r"\newsavebox{%s}" % name)
        lines.append(r"\sbox{%s}{$\displaystyle %s$}" % (name, s))
        lines.append(r"\typeout{PROBEWIDTH %d \the\wd%s}" % (i, name))
    lines.append(r"\end{document}")
    tex.write_text("\n".join(lines), encoding="utf-8")
    subprocess.run(
        ["pdflatex", "-interaction=nonstopmode", f"-output-directory={workdir}", str(tex)],
        capture_output=True,
        text=True,
    )
    log = (workdir / "probe.log").read_text(encoding="utf-8", errors="replace")
    out: list[float | None] = [None] * len(items)
    for m in re.finditer(r"PROBEWIDTH (\d+) ([0-9.]+)pt", log):
        out[int(m.group(1))] = float(m.group(2))
    return out


def spans_of(md: str) -> list[str]:
    out = []
    for line in md.splitlines():
        s = line.strip()
        if s.startswith("$$") and s.endswith("$$") and len(s) > 4:
            out.append(s[2:-2].strip())
    return out


def norm(s: str) -> str:
    s = TAG_RE.sub("", s)
    s = s.replace("\\begin{aligned}", "").replace("\\end{aligned}", "")
    s = s.replace("\\begin{strip}", "").replace("\\end{strip}", "")
    s = re.sub(r"\\[|\\]", "", s)
    s = re.sub(r"[&{}\s]", "", s)
    return s


def blocks_of(tex: str) -> list[dict]:
    """Every `\\[ ... \\]` display block in the woven tex, in order.

    Records whether the block sits inside a cuted `strip` and whether it
    uses `aligned`.
    """
    out: list[dict] = []
    i = 0
    while True:
        j = tex.find("\\[", i)
        if j < 0:
            break
        k = tex.find("\\]", j)
        if k < 0:
            break
        body = tex[j + 2 : k]
        before = tex[:j]
        in_strip = before.rstrip().endswith("\\begin{strip}")
        out.append(
            {
                "body": body.strip(),
                "strip": in_strip,
                "aligned": "\\begin{aligned}" in body,
                "norm": norm(body),
            }
        )
        i = k + 2
    return out


def main() -> int:
    paper = sys.argv[1] if len(sys.argv) > 1 else "math-hilfer-2024"
    d = FULL / paper
    spans = spans_of((d / f"{paper}-auto.md").read_text(encoding="utf-8"))
    tex = (d / f"{paper}.tex").read_text(encoding="utf-8")
    blocks = blocks_of(tex)
    print(f"spans={len(spans)} tex-blocks={len(blocks)}")

    widths = measure([TAG_RE.sub("", s) for s in spans], d / "probe")

    # rows the engine emitted
    rows: list[str] = []
    for b in blocks:
        if b["aligned"]:
            inner = b["body"][len("\\begin{aligned}") :]
            inner = inner[: inner.rfind("\\end{aligned}")]
            for rl in inner.split("\\\\"):
                rl = rl.strip()
                if rl.startswith("&"):
                    rows.append(rl[1:].strip())
    row_w = measure(rows, d / "probe-rows") if rows else []
    over_row = 0
    for w, r in zip(row_w, rows):
        if w is not None and w > TEXT_PT:
            over_row += 1
            print(f"  ROW-OVERFULL {w:.1f}pt: {r[:70]!r}")

    # match blocks to spans by content, greedily
    pool = list(blocks)
    disagree = 0
    unmatched = 0
    for i, (s, w) in enumerate(zip(spans, widths)):
        if w is None:
            print(f"{i:3d} MEASURE-FAIL {s[:50]!r}")
            continue
        ns = norm(s)
        # exact match first, then containment
        hit = None
        for pred in (
            lambda b: b["norm"] == ns,
            lambda b: ns and ns in b["norm"],
            lambda b: b["norm"] and b["norm"] in ns,
        ):
            for bi, b in enumerate(pool):
                if pred(b):
                    hit = bi
                    break
            if hit is not None:
                break
        if hit is None:
            unmatched += 1
            print(f"{i:3d} {w:7.1f}pt NO-MATCH {s[:50]!r}")
            continue
        b = pool.pop(hit)
        truth = "column" if w <= COLUMN_PT else ("strip" if w <= TEXT_PT else "overfull")
        engine = (
            "column"
            if not b["strip"]
            else ("strip+aligned" if b["aligned"] else "strip")
        )
        ok = (
            (engine == "column" and truth == "column")
            or (engine == "strip" and truth == "strip")
            or (engine == "strip+aligned" and truth in ("strip", "overfull"))
        )
        if not ok:
            disagree += 1
            print(f"{i:3d} {w:7.1f}pt truth={truth:9s} engine={engine:14s} {s[:52]!r}")
    print(f"\ndisagreements: {disagree}  unmatched: {unmatched}  row-overflows: {over_row}")
    print(f"leftover tex blocks: {len(pool)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
