"""RETIRED 2026-10-07 (ikat M4.4 self-hosting). Do not run.

This script built paper-2026-10-05.tex before ikat could. Its
registries now live in paper/ikat-spec.json (hand-owned) and the
build is one CLI call:

    ikat weave paper/paper-2026-10-05.md --spec paper/ikat-spec.json \\
        --bib paper/refs.bib

Kept for archaeology: the inline()/table_block()/bib_safe()
lineage that ikat's Rust core reimplements.
"""


"""paper-2026-10-05.md -> LaTeX IEEEtran two-column -> PDF.

Usage: python build-paper.py   (outputs paper-2026-10-05.tex, then run pdflatex+bibtex)
Figures: paper/figs/tikz/*.pdf (diagrams), paper/figs/*.pdf (plots).
Bibliography: refs.bib (keys already match the [`key`] citations).
"""
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
PLACEHOLDER = "\x00{}\x00"
SRC = os.path.join(HERE, "paper-2026-10-05.md")

MERMAID_FIGS = [
    ("fig-branch-tree", "The branches register: a tree over transaction time. "
     "Each lineage carries a parent pointer and a fork instant, non-decreasing "
     "down every root path; reads resolve by nearest ancestor."),
    ("fig-read-questions", "The three read operations answer different questions "
     "by construction. Composing the valid-time and recorded-time reads answers "
     "what was believed at $r$ about what was true at $v$."),
    ("fig-stratum", "Stratum placement: schema generation, trigger DDL, query "
     "compilation, and temporal logic above the libSQL connection (Doctrine I "
     "boundary); the engine below is never modified."),
    ("fig-ledger-flow", "Doctrines II--VI as storage shape: the append-only ledger, "
     "the trigger-captured log, and the disposable cache. Ad-hoc deletion aborts "
     "at the trigger layer (Doctrine V)."),
]
PLOT_FIGS = [
    ("fig-fork-flat", "Fork latency is flat across $3\\times$ data, confirming "
     "the O(1) construction empirically (one register row)."),
    ("fig-t6-asymmetry", "The T6 dimensional test: valid-slice-at-transaction-point "
     "against transaction-versions-at-valid-point. Transaction-dimension work "
     "costs an order of magnitude more on this store."),
    ("fig-trunk-branch", "Trunk against branched reads at scale 3 "
     "(R1 state change, R3 six-instant aggregation, Q2 replay, Q3 traversal). "
     "The dashed line marks a zero-write fork: branch cost is ancestry, "
     "not divergence."),
    ("fig-chain-depth", "Branched traversal holds flat across fork-chain depths "
     "1--10 at both scales (medians of 5, bars show min/max)."),
]
TABLE_CAPTIONS = [
    "The eight doctrines: the design contract.",
    "Lines of work: what each branches, and whether the two clocks stay apart.",
    "Evaluation matrix: workload classes against lineage scopes. Legend: reported, "
    "outstanding (defined, reported in the accompanying artifact), preliminary "
    "readings, not applicable.",
    "Nearest-ancestor resolution at work: visibility of each assertion to a read "
    "on lineage L.",
    "The three read operations, the questions they answer, and their mechanisms.",
    "Branch-aware physical design: every index and the query that reads it.",
    "Evaluation results: confirmed entries of the two-dimensional matrix.",
    "First adapted-workload readings (preliminary, non-reference hardware): "
    "trunk against branch at scales 1 and 3.",
    "Claim-to-decision map: paper claims and the register entries recording them.",
]

UNI = {
    "§": r"\S{}",
    "—": "---",
    "–": "--",
    "…": r"\dots{}",
    "≈": r"$\approx$",
    "∅": r"$\emptyset$",
    "≤": r"$\leq$",
    "×": r"$\times$",
    "→": r"$\to$",
    "≥": r"$\geq$",
    "µ": r"\textmu{}",
    "₂": r"$_{2}$",
    "₁": r"$_{1}$",
    "₃": r"$_{3}$",
}
SYM_TOKENS = {"✓": r"$\checkmark$", "○": r"$\circ$", "◐": r"$\oslash$",
              "†": r"\dag{}"}


def protect_math(text):
    """Split into (is_math, segment) on $...$ spans."""
    parts, buf, math, i = [], "", False, 0
    while i < len(text):
        if text[i] == "$":
            parts.append((math, buf))
            buf, math = "", not math
            i += 1
        else:
            buf += text[i]
            i += 1
    parts.append((math, buf))
    return parts


SUB_MAP = {"₀": "0", "₁": "1", "₂": "2", "₃": "3", "ₙ": "n", "ᵢ": "i"}

def esc_text(s):
    for ch, rep in [("\\", r"\textbackslash{}"), ("&", r"\&"), ("%", r"\%"),
                    ("#", r"\#"), ("_", r"\_"), ("{", r"\{"), ("}", r"\}"),
                    ("~", r"$\sim$"), ("^", r"\textasciicircum{}")]:
        s = s.replace(ch, rep)
    for ch, rep in list(UNI.items()) + list(SYM_TOKENS.items()):
        s = s.replace(ch, rep)
    s = re.sub("([A-Za-z0-9\)\]])([₀₁₂₃ₙᵢ])",
               lambda m: "$" + m.group(1) + "_{" + SUB_MAP[m.group(2)] + "}$", s)
    s = re.sub("([0-9])³", lambda m: "$" + m.group(1) + "^{3}$", s)
    return s


def esc_code(s):
    for ch, rep in [("\\", r"\textbackslash{}"), ("&", r"\&"), ("%", r"\%"),
                    ("#", r"\#"), ("_", r"\_"), ("{", r"\{"), ("}", r"\}"),
                    ("~", r"$\sim$"), ("^", r"\textasciicircum{}")]:
        s = s.replace(ch, rep)
    for ch, rep in list(UNI.items()) + list(SYM_TOKENS.items()):
        s = s.replace(ch, rep)
    return s


def inline(text, _ph=None):
    top = _ph is None
    ph = [] if top else _ph

    def stash(s):
        ph.append(s)
        return PLACEHOLDER.format(len(ph) - 1)

    def cite_group(m):
        if "`" not in m.group(1):
            return m.group(0)
        parts = [p.strip() for p in m.group(1).split(";")]
        outs = []
        for p in parts:
            mm = re.fullmatch(r"(?:(.*?):\s*)?`([^`]+)`", p)
            if mm:
                author, key = mm.group(1), mm.group(2)
                if author:
                    outs.append(inline(author, ph) + "~\\cite{" + key + "}")
                else:
                    outs.append("\\cite{" + key + "}")
            else:
                outs.append(inline(p, ph))
        return stash(", ".join(outs))

    text = re.sub(r"\[([^\[\]]+)\]", cite_group, text)
    pre = re.compile(r"`([^`]+)`")
    text = pre.sub(lambda m: stash("\\texttt{" + esc_code(m.group(1)) + "}"), text)
    out = []
    for is_math, seg in protect_math(text):
        if is_math:
            out.append("$" + seg + "$")
            continue
        seg = esc_text(seg)
        seg = re.sub(r"\*\*(.+?)\*\*", r"\\textbf{\1}", seg)
        seg = re.sub(r"\*(.+?)\*", r"\\textit{\1}", seg)
        out.append(seg)
    text = "".join(out)
    if top:
        for i in range(len(ph) - 1, -1, -1):
            text = text.replace(PLACEHOLDER.format(i), ph[i])
    return text


def split_row(line):
    return [c.strip() for c in line.strip().strip("|").split("|")]


def is_sep(line):
    cells = split_row(line)
    return bool(cells) and all(re.fullmatch(r":?-{2,}:?", c) for c in cells)


def table_block(rows, cap, spec=None):
    n = max(len(split_row(r)) for r in rows)
    lines = [r"\begin{table}[t]", r"\caption{" + cap + "}", r"\small",
             r"\begin{tabularx}{\columnwidth}{" + (spec or "X" * n) + r"}", r"\hline"]
    first = True
    for r in rows:
        if is_sep(r):
            continue
        cells = split_row(r)
        cells += [""] * (n - len(cells))
        cells = [inline(c) for c in cells]
        if first:
            cells = [r"\textbf{" + c + "}" for c in cells]
        lines.append(" & ".join(cells) + r" \\")
        if first:
            lines.append(r"\hline")
            first = False
    lines += [r"\hline", r"\end{tabularx}", r"\end{table}"]
    return "\n".join(lines)


def figure(path, caption, label, wide=False):
    env = "figure*" if wide else "figure"
    width = "\\textwidth" if wide else "\\columnwidth"
    return ("\n".join([
        r"\begin{" + env + "}[t]", r"\centering",
        r"\includegraphics[width=" + width + "]{" + path + "}",
        r"\caption{" + caption + "}", r"\label{" + label + "}",
        r"\end{" + env + "}"]))


def convert(lines):
    out, para = [], []
    title, in_abstract = "", False
    mermaid_idx, table_idx = 0, 0
    i = 0

    def flush():
        if para:
            text = inline(" ".join(para))
            if text.startswith("\\dag{}"):
                text = "\\textit{Note: }" + text[len("\\dag{}"):]
            out.append(text + "\n")
            para.clear()

    while i < len(lines):
        line = lines[i].rstrip("\n")
        s = line.strip()
        if not s:
            flush()
            i += 1
            continue
        if s.startswith(">"):
            i += 1
            continue
        if s.startswith("```"):
            flush()
            if "mermaid" in s:
                key, cap = MERMAID_FIGS[mermaid_idx]
                mermaid_idx += 1
                out.append(figure("figs/tikz/" + key, inline(cap),
                                  "fig:" + key, wide=True) + "\n")
            i += 1
            while i < len(lines) and not lines[i].strip().startswith("```"):
                i += 1
            i += 1
            continue
        if s.startswith("|"):
            flush()
            rows = []
            while i < len(lines) and lines[i].strip().startswith("|"):
                rows.append(lines[i].strip())
                i += 1
            out.append(table_block(rows, TABLE_CAPTIONS[table_idx],
                                   "Xcl" if table_idx == 8 else None) + "\n")
            table_idx += 1
            continue
        if s.startswith("## Appendix"):
            flush()
            out.append(r"\appendix" + "\n" + r"\section{Claim-to-decision map}" + "\n")
            i += 1
            continue
        if s.startswith("## Abstract"):
            flush()
            out.append(r"\begin{abstract}" + "\n")
            in_abstract = True
            i += 1
            continue
        if s.startswith("## "):
            flush()
            if in_abstract:
                out.append(r"\end{abstract}" + "\n")
                in_abstract = False
            head = re.sub(r"^\d+\.\s*", "", s[3:])
            out.append(r"\section{" + inline(head) + "}" + "\n")
            if head.strip() == "6. Discussion: branching vs alternatives":
                pass
            i += 1
            continue
        if s.startswith("### "):
            flush()
            head = re.sub(r"^\d+\.\d+\s*", "", s[4:])
            out.append(r"\subsection{" + inline(head) + "}" + "\n")
            i += 1
            continue
        if s.startswith("# "):
            title = inline(s[2:])
            i += 1
            continue
        para.append(s)
        i += 1
    flush()
    if in_abstract:
        out.append(r"\end{abstract}" + "\n")
    return title, "\n".join(out), mermaid_idx, table_idx


def bib_safe(src, dst):
    out = []
    for line in open(src, encoding="utf-8"):
        raw = line.rstrip("\n")
        m = re.match(r"(\s*author\s*=\s*\{)(.*)(\},\s*)$", raw)
        if m and not m.group(2).startswith("{"):
            raw = m.group(1) + "{" + m.group(2) + "}" + m.group(3)
        key = re.match(r"\s*([A-Za-z]+)\s*=", raw)
        if key and key.group(1).lower() not in ("author", "url", "doi"):
            raw = re.sub(r"(?<!\\)([_#%])", r"\\\1", raw)
            raw = re.sub(r"(?<!\\)&", r"\\&", raw)
        out.append(raw + "\n")
    open(dst, "w", encoding="utf-8").write("".join(out))


def main():
    with open(SRC, encoding="utf-8") as f:
        lines = f.readlines()
    title, body, nm, nt = convert(lines)
    assert nm == len(MERMAID_FIGS), nm
    assert nt == len(TABLE_CAPTIONS), nt
    plot_figs = "\n".join(
        figure("figs/" + key, cap, "fig:" + key)
        for key, cap in PLOT_FIGS)
    body = body.replace(
        r"\section{Discussion: branching vs alternatives}",
        plot_figs + "\n" + r"\section{Discussion: branching vs alternatives}")
    thanks = ("Revision of 2026-10-05: system references updated to 0.19.0; "
              "new material on attributes, bulk bytes, gates, and vector "
              "semantics. Decision codes (D-nnn) refer to the register "
              "appendix. No new measured figures.")
    doc = "\n".join([
        r"\documentclass[conference]{IEEEtran}",
        r"\usepackage[utf8]{inputenc}",
        r"\usepackage{lmodern}",
        r"\usepackage{textcomp}",
        r"\usepackage{amsmath,amssymb}",
        r"\usepackage{tabularx}",
        r"\usepackage{graphicx}",
        r"\usepackage[hidelinks]{hyperref}",
        r"\graphicspath{{./}{figs/}{figs/tikz/}}",
        r"\title{" + title + r"\thanks{" + thanks + "}}",
        r"\author{\IEEEauthorblockN{opticsWolf}}",
        r"\begin{document}", r"\maketitle", body,
        r"\bibliographystyle{IEEEtran}", r"\bibliography{refs}",
        r"\end{document}", ""])
    tex = os.path.join(HERE, "paper-2026-10-05.tex")
    with open(tex, "w", encoding="utf-8") as f:
        f.write(doc)
    bib_safe(os.path.join(HERE, "refs.bib"),
             os.path.join(HERE, "refs-paper.bib"))
    doc = doc.replace(r"\bibliography{refs}", r"\bibliography{refs-paper}")
    with open(tex, "w", encoding="utf-8") as f:
        f.write(doc)
    print("wrote", tex)


if __name__ == "__main__":
    main()
