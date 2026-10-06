"""Proof: a Rust-emitted pgfplots chart compiles with local TeX."""

from ikat import barchart_to_tikz, tex_requirements

tikz = barchart_to_tikz(
    "Branch cost is ancestry, not divergence",
    "latency (ms, log, scale 3)",
    True,
    ["R1", "R3", "Q2", "Q3"],
    ["trunk", "branch (audit)"],
    [[7.4, 24.0, 55.0, 0.47], [28.0, 80.0, 52.0, 66.0]],
    [[6.9, 20.0, 45.0, 0.46], [25.0, 70.0, 45.0, 60.0]],
    [[8.1, 28.0, 65.0, 0.48], [31.0, 90.0, 60.0, 72.0]],
    (2.8, 3.2, 66.0, "zero-write fork"),
)
req = tex_requirements(False, True)
lines = [r"\documentclass{article}", r"\usepackage[utf8]{inputenc}"]
lines += req
lines += [r"\begin{document}", tikz, r"\end{document}"]
with open("plotproof.tex", "w", encoding="utf-8") as f:
    f.write("\n".join(lines) + "\n")
print("tex ok,", len(tikz), "chars of tikz")
