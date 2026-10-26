"""Probe TeX for exact math-glyph widths and emit a calibrated Rust table.

Measures at the three math styles (display/text = 10pt, script = 7pt,
scriptscript = 5pt) every glyph, Greek letter, relation, binary, delimiter,
big operator, and operator name that appears in the corpus, plus construct
overheads (radical, fraction delimiters, matrix column separation, script
placement, thin spaces). Emits `glyphs_widths.json` and a Rust table.
"""
from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
WORK = HERE / "probe-w"

LOWERS = "abcdefghijklmnopqrstuvwxyz"
UPPERS = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
DIGITS = "0123456789"

GREEK = ["alpha", "beta", "gamma", "delta", "epsilon", "varepsilon", "zeta",
         "eta", "theta", "vartheta", "mu", "nu", "xi", "pi", "phi", "varphi",
         "rho", "sigma", "tau", "Gamma", "Delta", "Pi", "wp", "hbar", "infty"]
RELS = ["sim", "to", "leq", "geq", "in", "notin", "approx", "equiv", "neq"]
BINS = ["otimes", "times", "ast", "cdot"]
DELIMS = ["langle", "rangle"]
BIGOPS = ["sum", "int", "oint", "prod"]
OPNAMES = ["max", "min", "sup", "inf", "lim", "liminf", "limsup", "Pr",
           "cos", "sin", "tan", "log", "exp", "ker"]

STYLES = {"D": "displaystyle", "S": "scriptstyle", "SS": "scriptscriptstyle"}

lines: list[str] = [
    r"\documentclass{article}",
    r"\usepackage{amsmath,amssymb}",
    r"\begin{document}",
]


def probe(name: str, body: str) -> None:
    lines.append(rf"\sbox0{{$\displaystyle {body}$}}")
    lines.append(rf"\typeout{{{name} \the\wd0}}")


def probe_style(key: str, token: str, copies: int) -> None:
    """Width of `copies` consecutive tokens, per style."""
    body = token * copies
    for sk, sv in STYLES.items():
        probe(f"G|{sk}|{key}|{copies}", "\\" + sv + " " + body)


# --- single glyphs (8 copies for precision) ---
for c in LOWERS + UPPERS + DIGITS:
    probe_style(c, c, 8)
for g in GREEK:
    probe_style(g, "\\" + g, 8)
for g in RELS:
    probe_style(g, "\\" + g, 8)
probe_style("=", "=", 8)
probe_style("<", "<", 8)
probe_style(">", ">", 8)
probe_style("+", "+", 8)
probe_style("-", "-", 8)
probe_style("(", "(", 8)
probe_style(")", ")", 8)
probe_style("[", "[", 8)
probe_style("]", "]", 8)
for g in BINS:
    probe_style(g, "\\" + g, 8)
for g in DELIMS:
    probe_style(g, "\\" + g, 8)
probe_style("Vert", "\\Vert", 8)
probe_style("pipe", "|", 8)
probe_style("slash", "/", 8)
for g in ["cdots", "vdots"]:
    probe_style(g, "\\" + g, 8)

# --- big operators (single, scripts matter separately) ---
for g in BIGOPS:
    probe_style(g, "\\" + g, 1)

# --- operator names (single) ---
for g in OPNAMES:
    probe_style(g, "\\" + g, 1)

# --- classed glyphs single-copy (no mu-glue contamination) ---
for g in ["sim", "to", "leq", "geq", "in", "notin", "approx", "equiv",
          "neq", "otimes", "times", "ast", "cdot", "langle", "rangle",
          "cdots", "vdots"]:
    probe_style(g, "\\" + g, 1)
for c, name in [("=", "eq"), ("<", "lt"), (">", "gt"), ("+", "plus"),
                ("-", "minus"), ("|", "pipe"), ("/", "slash"),
                ("\\{", "lbrace"), ("\\}", "rbrace")]:
    for sk, sv in STYLES.items():
        probe(f"G|{sk}|{name}|1", "\\" + sv + " " + c)

# --- glue model composites (context spacing) ---
probe("SP|VXPLUS", "x+x")
probe("SP|VXEQQ", "x=x")
probe("SP|VXLEQ", r"x\leq x")
probe("SP|VXOT", r"x\otimes y")
# style-scaled glue: same composites in script/scriptscript
probe("SP|VEQS", r"\scriptstyle x=x")
probe("SP|VPLUSS", r"\scriptstyle x+x")
probe("SP|VEQSS", r"\scriptscriptstyle x=x")
# class-table probes: Op->Inner and Ord->Inner thin spaces
probe("SP|SUMFRAC", r"\sum\frac{a}{b}")
probe("SP|XFRAC", r"x\frac{a}{b}")
probe("SP|FRACX", r"\frac{a}{b}x")
probe("SP|FRACFRAC", r"\frac{a}{b}\frac{c}{d}")
probe("SP|PLFRAC", r"x+\frac{a}{b}")
probe("SP|RELFRAC", r"x=\frac{a}{b}")
# full class-pair table (measured, not from the book)
probe("SP|FRACab", r"\frac{a}{b}")
probe("SP|ORDOP", r"x\sum")
probe("SP|OPORD", r"\sum x")
probe("SP|OPOP", r"\sum\sum")
probe("SP|OPREL", r"\sum=x")
probe("SP|INNEROP", r"\frac{a}{b}\sum")
probe("SP|INNERREL", r"\frac{a}{b}=x")
probe("SP|INNEROPEN", r"\frac{a}{b}\left(x\right)")
probe("SP|CLOSEORD", r"\left(x\right)x")
probe("SP|CLOSEINNER", r"\left(x\right)\frac{a}{b}")
probe("SP|ORDOPEN", r"x\left(x\right)")
probe("SP|ORDCLOSE", r"x\left(y\right)")
# script-style decompositions
probe("SP|LIMSCRIPT", r"\scriptstyle n\to\infty")
probe("SP|SUMSCRIPT", r"\scriptstyle k=0")
probe("SP|SCRIPTSIM", r"\scriptstyle\sim")
# boundary disambiguation: glue at list edges
probe("SP|VEQEND", "x=")
probe("SP|VEQSTART", "=x")
probe("SP|PLUSORD", "+x")
probe("SP|ORDPUNCT", "x,")
probe("SP|PUNCTORD", ",x")
probe("SP|RELINNEREND", r"x=\frac{a}{b}")

# --- radicals: overhead = width - content ---
for sk, sv in STYLES.items():
    probe(f"SP|{sk}RAD6", "\\" + sv + r" \sqrt{gggggg}")
    probe(f"SP|{sk}RAD2", "\\" + sv + r" \sqrt{gg}")

# --- fractions ---
probe("SP|FRACst", r"\frac{s}{t}")
probe("SP|FRACsst", r"\frac{ss}{t}")
probe("SP|FRACssss", r"\frac{ssss}{tttt}")
probe("SP|FRACnest", r"\frac{\frac{s}{t}}{u}")

# --- script placement ---
probe("SP|BASEX", "x")
probe("SP|XSUP", r"x^{s}")
probe("SP|XSUB", r"x_{s}")
probe("SP|XSUBSUP", r"x_{s}^{s}")
probe("SP|YSUP", r"y^{s}")
probe("SP|XPRIME", "x'")
probe("SP|XPRIME2", "x''")

# --- spaces ---
probe("SP|SPthin", r"x\,x")
probe("SP|SPmed", r"x\:x")
probe("SP|SPthick", r"x\;x")
probe("SP|SPneg", r"x\!x")
probe("SP|SPquad", r"x\quad x")
probe("SP|SPqquad", r"x\qquad x")

# --- matrices: column separation ---
probe("SP|MAT2", r"\begin{bmatrix} a & b \\ c & d \end{bmatrix}")
probe("SP|MAT3", r"\begin{bmatrix} a & b & c \\ d & e & f \\ g & h & i \end{bmatrix}")
probe("SP|MATW", r"\begin{bmatrix} a & mmmm \\ c & d \end{bmatrix}")
probe("SP|PMAT2", r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}")
probe("SP|AMAT2", r"\begin{matrix} a & b \\ c & d \end{matrix}")

# --- left/right delimiters ---
probe("SP|LRP", r"\left(x\right)")
probe("SP|LRB", r"\left[x\right]")
probe("SP|LRC", r"\left\{x\right\}")
probe("SP|LRANG", r"\left\langle x\right\rangle")
probe("SP|LRV", r"\left\Vert x\right\Vert")

# --- script/wrapper fonts ---
probe("SP|CALA", r"\mathcal{A}")
probe("SP|CALR", r"\mathcal{R}")
probe("SP|CALL", r"\mathcal{L}")
probe("SP|CALD", r"\mathcal{D}")
probe("SP|CALF", r"\mathcal{F}")
probe("SP|CALS", r"\mathcal{S}")
probe("SP|CALT", r"\mathcal{T}")
probe("SP|BBR", r"\mathbb{R}")
probe("SP|BBN", r"\mathbb{N}")
probe("SP|BBE", r"\mathbb{E}")
probe("SP|BBP", r"\mathbb{P}")
probe("SP|TEXTabc", r"\text{abc}")
probe("SP|TEXTs", r"\text{s}")
probe("SP|MROMANab", r"\mathrm{ab}")
probe("SP|BOLDzeta", r"\boldsymbol{\zeta}")
probe("SP|BOLDab", r"\mathbf{ab}")
probe("SP|OVERSETwedge", r"\overset{\wedge}{=}")
probe("SP|OVERSETab", r"\overset{a}{b}")
probe("SP|UNDERSETab", r"\underset{a}{b}")
probe("SP|HATx", r"\hat{x}")
probe("SP|WEDGE", r"\wedge")

# --- real overset/underset patterns from the corpus ---
probe("SP|OVST1", r"\overset{\sim}{\vartheta}")
probe("SP|UNST1", r"\underset{\rho}{\text{max}}")

# --- limits placement per style (stacked vs side scripts) ---
probe("SP|SUMlim", r"\sum_{k=0}^{\beta}")
probe("SP|INTlim", r"\int_{0}^{\zeta}")
probe("SP|LIMlim", r"\lim_{x\in A}")
probe("SP|MAXlim", r"\max_{x\in A}")
probe("SP|MAXbare", r"\max")
probe("SP|LIMbare", r"\lim")

# --- roman per char (\text and \mathrm use cmr) ---
for c in LOWERS + DIGITS:
    probe_style("R" + c, r"\mathrm{" + c + "}", 8)

# --- validation composites ---
probe("SP|VXSUPg", r"x^{g}")
probe("SP|VXSUBg", r"x_{g}")
probe("SP|VXSUBSUPg", r"x_{g}^{g}")
probe("SP|VFRACfrac", r"\frac{\vartheta(\zeta)}{\Gamma}")
probe("SP|VSUMlim2", r"\sum_{k=0}^{\beta}\frac{\zeta}{k}")
probe("SP|VLIMfrac", r"\lim_{n\to\infty}\frac{\sigma}{n}")
probe("SP|VMATmix", r"\begin{bmatrix} \vartheta & \zeta \\ \rho & \sigma \end{bmatrix}")
probe("SP|VLRAINBOW", r"\frac{\alpha}{\beta}+\sqrt{\gamma}\cdot\mathcal{S}\leq\Vert\varphi\Vert")

# --- delta chain for the rainbow composite ---
probe("SP|P1", r"\frac{\alpha}{\beta}")
probe("SP|P2", r"\frac{\alpha}{\beta}+")
probe("SP|P3", r"\frac{\alpha}{\beta}+\sqrt{\gamma}")
probe("SP|P4", r"\frac{\alpha}{\beta}+\sqrt{\gamma}\cdot")
probe("SP|P5", r"\frac{\alpha}{\beta}+\sqrt{\gamma}\cdot\mathcal{S}")
probe("SP|P6", r"\frac{\alpha}{\beta}+\sqrt{\gamma}\cdot\mathcal{S}\leq")
probe("SP|P7", r"\frac{\alpha}{\beta}+\sqrt{\gamma}\cdot\mathcal{S}\leq\Vert")
probe("SP|P8", r"\frac{\alpha}{\beta}+\sqrt{\gamma}\cdot\mathcal{S}\leq\Vert\varphi\Vert")
probe("SP|SQRTG", r"\sqrt{\gamma}")
probe("SP|CALSW", r"\mathcal{S}")

probe("SP|SQRTFRAC", r"\sqrt{\frac{a}{b}}")
probe("SP|SQRTFRACFRAC", r"\sqrt{\frac{\frac{a}{b}}{c}}")

lines.append(r"\end{document}")

WORK.mkdir(parents=True, exist_ok=True)
tex = WORK / "probe.tex"
tex.write_text("\n".join(lines), encoding="utf-8")
subprocess.run(
    ["pdflatex", "-interaction=nonstopmode", f"-output-directory={WORK}", str(tex)],
    capture_output=True, text=True, cwd=str(WORK),
)
log = (WORK / "probe.log").read_text(encoding="utf-8", errors="replace")

vals: dict[str, float] = {}
for m in re.finditer(r"^(?:G\|[A-Z]+\|.+?\|\d+|SP\|.+?) ([0-9.]+)pt\s*$", log, re.M):
    key = m.group(0).rsplit(None, 1)[0]
    pt = float(m.group(1))
    vals[key] = pt

out: dict[str, float] = {}
# per-glyph per-style, averaged over copies
for key, pt in vals.items():
    out[key] = pt

(HERE / "glyphs_raw.json").write_text(json.dumps(out, indent=1), encoding="utf-8")
print(f"measured {len(out)} keys")
