"""Emit the calibrated Rust glyph table and validate the width model.

Reads `glyphs_raw.json` (from math_probe_widths.py), derives per-style glyph
widths in cm, validates the structural model against TeX truth on composite
probes, then rewrites the table between the BEGIN/END CALIBRATED TABLE
markers in `src/mathwidth.rs`.
"""
from __future__ import annotations

import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
PT2CM = 2.54 / 72.27

raw = json.loads((HERE / "glyphs_raw.json").read_text(encoding="utf-8"))

GREEK = ["alpha", "beta", "gamma", "delta", "epsilon", "varepsilon", "zeta",
         "eta", "theta", "vartheta", "mu", "nu", "xi", "pi", "phi", "varphi",
         "rho", "sigma", "tau", "Gamma", "Delta", "Pi", "wp", "hbar", "infty"]
RELS = ["sim", "to", "leq", "geq", "in", "notin", "approx", "equiv", "neq"]
BINS = ["otimes", "times", "ast", "cdot"]
BIGOPS = ["sum", "int", "oint", "prod"]
OPNAMES = ["max", "min", "sup", "inf", "lim", "liminf", "limsup", "Pr",
           "cos", "sin", "tan", "log", "exp", "ker"]
STYLES = ["D", "S", "SS"]


def val(key: str) -> float:
    return raw[key] * PT2CM


def glyph_row(rust: str, probe: str, copies: int) -> str:
    ws = [val(f"G|{st}|{probe}|{copies}") / copies for st in STYLES]
    return f'    ("{rust}", [{ws[0]:.4f}, {ws[1]:.4f}, {ws[2]:.4f}]),'


rows: list[str] = []
for c in "abcdefghijklmnopqrstuvwxyz":
    rows.append(glyph_row(c, c, 8))
for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ":
    rows.append(glyph_row(c, c, 8))
for c in "0123456789":
    rows.append(glyph_row(c, c, 8))
for g in GREEK:
    rows.append(glyph_row(g, g, 8))
# roman (cmr) glyphs for \text/\mathrm: probed under the R prefix
for c in "abcdefghijklmnopqrstuvwxyz0123456789":
    rows.append(glyph_row("R" + c, "R" + c, 8))
# classed glyphs: single-copy probes (multi-copy would smear mu-glue in)
for g in RELS + BINS + ["langle", "rangle", "cdots", "vdots"]:
    rows.append(glyph_row(g, g, 1))
# Vert is probed x8 (Ord-class delimiter, no glue)
rows.append(glyph_row("Vert", "Vert", 8))
for rust in ["eq", "lt", "gt", "plus", "minus", "pipe", "slash",
             "lbrace", "rbrace"]:
    rows.append(glyph_row(rust, rust, 1))
# plain delimiters are Ord (no glue): multi-copy is clean
for rust, tok in [("lpar", "("), ("rpar", ")"), ("lbrk", "["), ("rbrk", "]")]:
    rows.append(glyph_row(rust, tok, 8))
for g in BIGOPS + OPNAMES:
    rows.append(glyph_row(g, g, 1))

# constants (validated below)
SCRIPTSPACE = val("SP|XSUP") - val("SP|BASEX") - val("G|S|s|8") / 8
NULLDELIM2 = val("SP|FRACst") - val("SP|TEXTs")  # s/frac gap; refine below
NULLDELIM2 = val("SP|FRACst") - 4.6875 * PT2CM   # max(s@10)=4.6875pt
THICK = (val("SP|VXEQQ") - 2 * val("SP|BASEX") - val("G|D|eq|1")) / 2
MED = (val("SP|VXPLUS") - 2 * val("SP|BASEX") - val("G|D|plus|1")) / 2
THIN = (val("SP|SPthin") - 2 * val("SP|BASEX")) / 2
QUAD = val("SP|SPquad") - 2 * val("SP|BASEX")
PRIME = val("SP|XPRIME") - val("SP|BASEX")
g_w = [val(f"G|{st}|g|8") / 8 for st in STYLES]
RAD = [val(f"SP|{st}RAD6") - 6 * gw for st, gw in zip(STYLES, g_w)]
MAT_GAP = val("SP|AMAT2") - (val("G|D|a|8") + val("G|D|d|8")) / 8
MAT_BRACK = {
    "bmatrix": val("SP|MAT2") - val("SP|AMAT2"),
    "pmatrix": val("SP|PMAT2") - val("SP|AMAT2"),
    "matrix": 0.0,
}
BOLD = (val("SP|BOLDzeta") / (val("G|D|zeta|8") / 8)
        + val("SP|BOLDab") / ((val("G|D|a|8") + val("G|D|b|8")) / 8)) / 2


CHARKEY = {"lpar": "(", "rpar": ")", "lbrk": "[", "rbrk": "]"}


def gw(name: str, style: int) -> float:
    """Glyph width (cm) from the probe for style D/S/SS = 0/1/2."""
    name = CHARKEY.get(name, name)
    for copies in (1, 8):
        key = f"G|{STYLES[style]}|{name}|{copies}"
        if key in raw:
            return val(key) / copies
    raise KeyError(name)


# ---- model validation moved to Rust tests (src/mathwidth.rs) ----
# The composite probes in glyphs_raw.json are the TeX truth; the Rust
# unit tests assert the walker reproduces them within 0.01 cm.

print(f"\nconstants: SCRIPTSPACE={SCRIPTSPACE:.4f} NULLDELIM2={NULLDELIM2:.4f} "
      f"THICK={THICK:.4f} MED={MED:.4f} THIN={THIN:.4f} QUAD={QUAD:.4f} "
      f"PRIME={PRIME:.4f} RAD={[round(r, 4) for r in RAD]} MAT_GAP={MAT_GAP:.4f} "
      f"brackets={ {k: round(v, 4) for k, v in MAT_BRACK.items()} } BOLD={BOLD:.3f}")

# ---- emit Rust table ----
src = (ROOT / "src" / "mathwidth.rs").read_text(encoding="utf-8")
table = "\n".join(rows)
new_src = re.sub(
    r"(// BEGIN CALIBRATED TABLE.*?= &\[\n).*?(\n?\];?\n?// END CALIBRATED TABLE)",
    lambda m: m.group(1) + table + "\n];// END CALIBRATED TABLE",
    src, flags=re.S)
(ROOT / "src" / "mathwidth.rs").write_text(new_src, encoding="utf-8")
print(f"\nwrote {len(rows)} glyph rows to src/mathwidth.rs")
