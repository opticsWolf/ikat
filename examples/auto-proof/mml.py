"""MathML -> LaTeX for the bench exporter (M7.8 honest math).

The corpus JATS carries real MathML (511 equations in math-hilfer-2024),
but the exporter used to flatten it with itertext: fractions lost their
bar, sub/superscripts juxtaposed, and every symbol became its own $..$
span, so nothing could break across lines. This module reconstructs
single structured $...$ spans instead.

Three codepoints are publisher-misencoded (roles proven from the
original PDF + symbol coexistence, shapes matched to renders):
  U+01BB -> the paper's time variable, rendered here as \\zeta
           (coexists with a real, different tau: S(UBB-tau) with UBB!=tau,
           and the PDF sets the two from different fonts);
  U+049F -> upright beta subscript (alternates with real beta) -> \\beta;
  U+0277 -> summation/Gamma index (Gamma(k+1), sum k=0) -> k.
"""

import unicodedata
import xml.etree.ElementTree as ET


def local(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


# Bare-Latin: what a <mi> holding plain text means in math.
GREEK = {
    "\u03b1": r"\alpha", "\u03b2": r"\beta", "\u03b3": r"\gamma",
    "\u03b4": r"\delta", "\u03b5": r"\varepsilon", "\u03ba": r"\kappa",
    "\u03bb": r"\lambda", "\u03bc": r"\mu", "\u03c0": r"\pi",
    "\u03c3": r"\sigma", "\u03c4": r"\tau", "\u03c7": r"\chi",
    "\u03b7": r"\eta", "\u03b8": r"\theta", "\u03b9": r"\iota",
    "\u03bd": r"\nu", "\u03be": r"\xi", "\u03c1": r"\rho",
    "\u03c2": r"\varsigma", "\u03c6": r"\varphi", "\u03c8": r"\psi",
    "\u03c9": r"\omega", "\u0393": r"\Gamma", "\u0398": r"\Theta",
    "\u039b": r"\Lambda", "\u039e": r"\Xi", "\u03a0": r"\Pi",
    "\u03a3": r"\Sigma", "\u03a6": r"\Phi", "\u03a8": r"\Psi",
    "\u03a9": r"\Omega", "\u0394": r"\Delta",
    "\u03d1": r"\vartheta", "\u03d5": r"\phi",
    "\u03d6": r"\varpi", "\u03f5": r"\epsilon", "\u03c5": r"\upsilon",
    "\u03d0": r"\beta",
    "\u2113": r"\ell", "\u2135": r"\aleph",
    "\u221d": r"\propto", "\u2225": r"\|",
    "\u22a5": r"\perp", "\u2220": r"\angle",
    "\u2243": r"\simeq", "\u2245": r"\cong",
    "\u2118": r"\wp", "\u211b": r"\Re", "\u2131": r"\mathcal{F}",
    "\u2115": r"\mathbb{N}", "\u211d": r"\mathbb{R}",
    "\u2124": r"\mathbb{Z}", "\u211a": r"\mathbb{Q}",
    "\u210f": r"\hbar", "\u2205": r"\emptyset",
}

# Publisher-misencoded glyphs (see module docstring).
SPECIAL = {
    "\u01bb": r"\zeta",
    "\u049f": r"\beta",
    "\u0277": "k",
}

# <mo> text -> bare LaTeX (no $ wrapping; caller owns math mode).
OPS = {
    "\u2212": "-", "\u00b1": r"\pm", "\u00b0": r"^{\circ}",
    "\u00d7": r"\times", "\u00f7": r"\div", "\u22c5": r"\cdot",
    "\u2219": r"\cdot", "\u2218": r"\circ", "\u2217": "*",
    "\u2264": r"\leq", "\u2265": r"\geq", "\u2260": r"\neq",
    "\u2248": r"\approx", "\u2261": r"\equiv", "\u223c": r"\sim",
    "\u2190": r"\leftarrow", "\u2192": r"\to",
    "\u21d2": r"\Rightarrow", "\u27e8": r"\langle",
    "\u27e9": r"\rangle", "\u3008": r"\langle", "\u3009": r"\rangle",
    "\u2329": r"\langle", "\u232a": r"\rangle",
    "\u2032": "'", "\u2033": "''",
    "\u2202": r"\partial", "\u2207": r"\nabla",
    "\u221e": r"\infty", "\u222b": r"\int", "\u2211": r"\sum",
    "\u220f": r"\prod", "\u22c0": r"\bigwedge", "\u22c1": r"\bigvee",
    "\u22c3": r"\bigcup", "\u22c2": r"\bigcap",
    "\u2208": r"\in", "\u2209": r"\notin", "\u220b": r"\ni",
    "\u2282": r"\subset", "\u2286": r"\subseteq",
    "\u2283": r"\supset", "\u2287": r"\supseteq",
    "\u222a": r"\cup", "\u2229": r"\cap", "\u2295": r"\oplus",
    "\u2297": r"\otimes",
    "\u2200": r"\forall", "\u2203": r"\exists", "\u00ac": r"\neg",
    "\u2227": r"\wedge", "\u2228": r"\vee",
    "\u22ef": r"\cdots", "\u22ee": r"\vdots", "\u22f1": r"\ddots",
    "\u2026": r"\cdots",
    "\u2016": r"\Vert", "\u2017": "_",
    "\u2061": "", "\u2062": "", "\u2063": "", "\u2064": "+",
    "\u2009": r"\,", "\u200a": r"\,", "\u2003": r"\quad",
    "\u00a0": " ", "\u00b9": "^{1}", "\u00b2": "^{2}",
    "\u00b3": "^{3}",
    "*": r"\ast", "\u200b": "", "\u200d": "",
    "\u2060": "", "\u02dc": r"\sim",
}

FUNC_NAMES = {
    "log": r"\log", "ln": r"\ln", "sin": r"\sin", "cos": r"\cos",
    "tan": r"\tan", "exp": r"\exp", "max": r"\max", "min": r"\min",
    "lim": r"\lim", "sup": r"\sup", "inf": r"\inf", "det": r"\det",
    "dim": r"\dim", "ker": r"\ker", "arg": r"\arg", "deg": r"\deg",
}

LIMIT_OPS = {
    r"\sum", r"\prod", r"\int", r"\iint", r"\bigcup", r"\bigcap",
    r"\lim", r"\max", r"\min", r"\sup", r"\inf", r"\liminf", r"\limsup",
}

ACCENTS = {
    "^": "hat", "\u00af": "bar", "\u02c9": "bar", "\u00af\u200b": "bar",
    "-": "bar", "~": "tilde", "\u02dc": "tilde",
    "\u2192": "overrightarrow", "\u2190": "overleftarrow",
    ".": "dot", "..": "ddot", "\u00a8": "ddot",
    "\u02dc": "tilde",
}


def mathify(text: str, stats: dict) -> str:
    """Plain JATS text (JATS-level sub/sup, titles) -> bare math LaTeX."""
    out = []
    for ch in text:
        if ch in SPECIAL:
            out.append(SPECIAL[ch])
        elif ch in GREEK:
            out.append(GREEK[ch])
        elif ch in OPS:
            out.append(OPS[ch])
        elif ord(ch) < 128:
            out.append(ch)
        else:
            decomp = unicodedata.normalize("NFKD", ch)
            base = "".join(c for c in decomp if ord(c) < 128)
            if len(decomp) == 1 and decomp in GREEK:
                out.append(GREEK[decomp])
            elif base and all(c.isascii() and (c.isalnum() or c in "+-=<>") for c in base):
                out.append(base[0] if len(base) == 1 else r"\mathrm{%s}" % base)
                stats["mathify_fold"] = stats.get("mathify_fold", 0) + 1
            else:
                stats["mathify_lost"] = sorted(
                    set(stats.get("mathify_lost", []) or []) | {"U+%04X" % ord(ch)})[:12]
                out.append("?")
    return "".join(out)


def _mi_text(text: str, variant: str, stats: dict) -> str:
    t = (text or "").replace("\u00a0", "").strip()
    if not t:
        return ""
    if len(t) == 1:
        ch = t
        if ch in SPECIAL:
            cmd = SPECIAL[ch]
        elif ch in GREEK:
            cmd = GREEK[ch]
        elif ch in OPS:
            return OPS[ch]
        else:
            decomp = unicodedata.normalize("NFKD", ch)
            if len(decomp) == 1 and decomp in GREEK:
                cmd = GREEK[decomp]
            elif len(decomp) == 1 and ord(decomp) < 128 and decomp.isalnum():
                cmd = decomp
            elif ord(ch) < 128 and ch.isalnum():
                cmd = ch
            else:
                stats["mml_lost"] = sorted(
                    set(stats.get("mml_lost", []) or []) | {"U+%04X" % ord(ch)})[:12]
                return "?"
        if variant == "normal":
            # Upright shape requested; roman has no Greek, so Greek
            # keeps its command (shape distinction lost, content kept).
            if cmd.startswith("\\"):
                return cmd
            return r"\mathrm{%s}" % cmd
        if variant == "bold":
            return r"\mathbf{%s}" % cmd if not cmd.startswith("\\") else r"\boldsymbol{%s}" % cmd
        if variant == "script":
            return r"\mathcal{%s}" % cmd if len(cmd) == 1 else cmd
        if variant == "double-struck":
            # Lowercase blackboard (a/b/c/d nodes here) has no msbm
            # glyph and the original prints plain italic: drop it.
            if len(cmd) == 1 and cmd.islower():
                return cmd
            return r"\mathbb{%s}" % cmd if len(cmd) == 1 else cmd
        if variant == "fraktur":
            return r"\mathfrak{%s}" % cmd if len(cmd) == 1 else cmd
        if variant in ("sans-serif", "monospace"):
            return cmd if cmd.startswith("\\") else r"\mathsf{%s}" % cmd
        return cmd
    word = "".join(c for c in t if ord(c) < 128 or c in GREEK)
    if word in FUNC_NAMES:
        return FUNC_NAMES[word]
    if word.isalpha() and all(ord(c) < 128 for c in word):
        if variant == "normal":
            return r"\mathrm{%s}" % word
        return r"\mathit{%s}" % word
    return mathify(t, stats)


def _kids(node: ET.Element):
    return [c for c in node if isinstance(c.tag, str)]


def _tex(node: ET.Element, stats: dict) -> str:
    t = local(node.tag)
    if t == "mi":
        return _mi_text(node.text or "", (node.get("mathvariant") or "").strip(), stats)
    if t == "mn":
        return mathify((node.text or "").strip(), stats)
    if t == "mo":
        txt = (node.text or "").strip()
        if not txt:
            return ""
        # Braces always scale (cases-Brace; JATS never marks
        # stretchy, and set braces size harmlessly); the root fixup
        # closes strays. A lone bar NEVER scales: kets (|gamma>)
        # have no right partner, and cross-group \left..\right is
        # a TeX error. Norms scale via \Vert pairing at the root.
        if txt in "{}":
            return {"{": r"\left\{", "}": r"\right\}"}[txt]
        if txt == "|":
            return "|"
        if txt in "()[]" and node.get("stretchy") == "true":
            return {"(": r"\left(", ")": r"\right)",
                    "[": r"\left[", "]": r"\right]"}[txt]
        if len(txt) == 1 and txt in OPS:
            return OPS[txt]
        return mathify(txt, stats)
    if t == "mtext":
        return r"\text{%s}" % _escape_text("".join(node.itertext()))
    if t == "mspace":
        w = (node.get("width") or "").strip()
        try:
            em = float(w.rstrip("em")) if w.endswith("em") else 0.0
        except ValueError:
            em = 0.0
        return r"\quad" if em >= 1.0 else r"\;" if em >= 0.2 else r"\,"
    if t in ("mrow", "mstyle", "mpadded", "mphantom", "semantics"):
        kids = _kids(node)
        if t == "semantics":
            kids = [c for c in kids if local(c.tag) != "annotation"]
            kids = kids[:1]
        return _join(_tex(c, stats) for c in kids)
    if t == "mfrac":
        kids = _kids(node)
        if len(kids) >= 2:
            return r"\frac{%s}{%s}" % (_tex(kids[0], stats), _tex(kids[1], stats))
        stats["mml_short_frac"] = stats.get("mml_short_frac", 0) + 1
        return _join(_tex(c, stats) for c in kids)
    if t == "msqrt":
        kids = _kids(node)
        return r"\sqrt{%s}" % _join(_tex(c, stats) for c in kids)
    if t == "mroot":
        kids = _kids(node)
        if len(kids) >= 2:
            return r"\sqrt[%s]{%s}" % (_tex(kids[1], stats), _tex(kids[0], stats))
        return r"\sqrt{%s}" % _join(_tex(c, stats) for c in kids)
    if t in ("msub", "msup", "msubsup"):
        kids = _kids(node)
        base = _tex(kids[0], stats) if len(kids) > 0 else ""
        sub = _tex(kids[1], stats) if len(kids) > 1 else ""
        sup = _tex(kids[2], stats) if t == "msubsup" and len(kids) > 2 else ""
        if t == "msup" and len(kids) > 1:
            sup = _tex(kids[1], stats)
        if not base:
            # Empty-base prescript (^H D): the sup/sub floats left.
            if t == "msub":
                return "_{%s}" % sub
            return "^{%s}" % sup
        # A bare multi-letter base would misattach the script.
        # Single atoms (one char, one command, delimited) stay bare.
        import re as _re
        single = len(base) == 1 or _re.fullmatch(r"\\[a-zA-Z]+", base)
        if not single and not base.startswith(("\\left", "(", "[")):
            base = "{%s}" % base
        # Same-level nesting (prescript coded as msup-on-msup):
        # TeX forbids x^a^b, so an empty atom separates the scripts.
        if t in ("msub", "msubsup") and sub and "_" in _trailing_scripts(base):
            base += "{}"
        if t in ("msup", "msubsup") and sup and "^" in _trailing_scripts(base):
            base += "{}"
        out = base
        if t in ("msub", "msubsup"):
            out += "_{%s}" % sub
        if t in ("msup", "msubsup"):
            out += "^{%s}" % sup
        return out
    if t in ("munder", "mover", "munderover"):
        kids = _kids(node)
        base = _tex(kids[0], stats) if len(kids) > 0 else ""
        under = _tex(kids[1], stats) if len(kids) > 1 else ""
        over = _tex(kids[2], stats) if t == "munderover" and len(kids) > 2 else ""
        if t == "mover" and len(kids) > 1:
            over = _tex(kids[1], stats)
        if base in LIMIT_OPS:
            out = base
            if t in ("munder", "munderover"):
                out += "_{%s}" % under
            if t in ("mover", "munderover"):
                out += "^{%s}" % over
            return out
        if t == "mover" and len(over) == 1:
            acc = ACCENTS.get(over, "")
            if acc:
                return r"\%s{%s}" % (acc, base)
        out = base
        if t in ("munder", "munderover"):
            out = r"\underset{%s}{%s}" % (under, out)
        if t in ("mover", "munderover"):
            out = r"\overset{%s}{%s}" % (over, out)
        return out
    if t == "mtable":
        rows = []
        for tr in node:
            if local(tr.tag) not in ("mtr", "mlabeledtr"):
                continue
            cells = [c for td in tr if isinstance(td.tag, str)
                     for c in ([_join(_tex(x, stats) for x in td if isinstance(x.tag, str))]
                              if local(td.tag) == "mtd" else [])]
            rows.append(" & ".join(cells))
        stats["mml_matrix"] = stats.get("mml_matrix", 0) + 1
        return r"\begin{matrix} %s \end{matrix}" % r" \\ ".join(rows)
    if t == "mfenced":
        o, c = node.get("open", "("), node.get("close", ")")
        sep = node.get("separators", ",")
        parts = [_tex(k, stats) for k in _kids(node)]
        return o + (" " + sep + " ").join(parts) + c
    if t == "annotation":
        return ""
    stats["mml_passthrough"] = sorted(
        set(stats.get("mml_passthrough", []) or []) | {t})[:12]
    return _join(_tex(c, stats) for c in _kids(node))


def _known_cmds() -> set:
    import re as _re
    names = set()
    for v in list(GREEK.values()) + list(OPS.values()) + list(FUNC_NAMES.values()):
        m = _re.fullmatch(r"\\([a-zA-Z]+)", v)
        if m:
            names.add(m.group(1))
    return names


_KNOWN = None


def _split_cmds(tex: str) -> str:
    """Split `\\inL` -> `\\in L`, leave `\\infty}` alone."""
    global _KNOWN
    if _KNOWN is None:
        _KNOWN = _known_cmds()
    out = []
    i, n = 0, len(tex)
    while i < n:
        if tex[i] != "\\" or i + 1 >= n or not tex[i + 1].isalpha():
            out.append(tex[i])
            i += 1
            continue
        j = i + 1
        while j < n and tex[j].isalpha():
            j += 1
        run = tex[i + 1:j]
        if run in _KNOWN:
            out.append(tex[i:j])
            i = j
            continue
        k = len(run) - 1
        while k > 0 and run[:k] not in _KNOWN:
            k -= 1
        if k > 0:
            out.append(tex[i:i + 1 + k] + " ")
            i = i + 1 + k
        else:
            out.append(tex[i:j])
            i = j
    return "".join(out)


def _trailing_scripts(base):
    """Script chars attached at top level at the very end.
    Handles braced (x^{ab}) and bare single-char (x^a) forms."""
    found = set()
    i = len(base) - 1
    while i >= 0:
        if base[i] == "}":
            depth, j = 0, i
            while j >= 0:
                if base[j] == "}":
                    depth += 1
                elif base[j] == "{":
                    depth -= 1
                    if depth == 0:
                        break
                j -= 1
            if j < 1 or base[j - 1] not in ("^", "_"):
                break
            found.add(base[j - 1])
            i = j - 2
        elif i >= 1 and base[i - 1] in ("^", "_") and base[i] not in ("{", "}", "^", "_"):
            found.add(base[i - 1])
            i -= 2
        else:
            break
    return found


def _cat(a, b):
    """Sibling join with prescript awareness: an empty-base
    sup/sub (prescript idiom) landing right after the same script
    level would be a TeX double-script error, so an empty atom
    separates them. Mixed levels (x_a^b) pass through."""
    if not a:
        return b
    if not b:
        return a
    lead = "^" if b.startswith("^") else "_" if b.startswith("_") else None
    if lead and lead in _trailing_scripts(a):
        return a + "{}" + b
    return a + b


def _join(parts):
    import functools as _f
    return _f.reduce(_cat, parts, "")


def _escape_text(s: str) -> str:
    return (s.replace("\\", r"\textbackslash{}")
             .replace("{", r"\{").replace("}", r"\}")
             .replace("_", r"\_").replace("%", r"\%")
             .replace("&", r"\&").replace("#", r"\#"))


def mml_to_tex(node: ET.Element, stats: dict) -> str:
    """A <math> (or formula) element -> bare LaTeX (no $ wrapping).

    Closes stray stretchy fences: a lone stretchy `{` (cases brace)
    gets its `\\right.` so the span always compiles.
    """
    tex = _join(_tex(c, stats) if local(node.tag) == "math" else _tex(node, stats)
                  for c in ([node] if local(node.tag) != "math" else _kids(node)))
    # Norms stay plain \Vert: positional pairing (even per-segment)
    # cannot respect TeX grouping — a \left\Vert in one cell and
    # its \right\Vert in the next is a fatal group error, and
    # multline splits would strand halves. Big fences from real
    # stretchy JATS (\left\Vert pairs the JATS itself nests)
    # still size; single bars never scaled (kets).
    # Command/letter separation with TeX semantics: TeX reads the
    # LONGEST alpha run after `\\`, so `\\infty}` is one command
    # (never split) while `\\inL` is undefined — split it after the
    # longest KNOWN prefix (`\\in L`). A regex alternation cannot do
    # this: when the longest fails its lookahead it backtracks and
    # shreds (`\\infty` -> `\\inf ty`).
    tex = _split_cmds(tex)
    lefts = tex.count(r"\left") - tex.count(r"\right")
    if lefts > 0:
        tex += r" \right." * lefts
        stats["mml_unclosed_fence"] = stats.get("mml_unclosed_fence", 0) + lefts
    return tex.strip()


EQ_PRE = "\ue000"
EQ_SUF = "\ue001"


def eq_mark(idx):
    return "%s%d%s" % (EQ_PRE, idx, EQ_SUF)


def merge_math(par):
    import re as _re
    par = _re.sub(r"\*\$([^\$]*\\[^\$]*)\$\*", r"$\1$", par)
    prev = None
    while prev != par:
        prev = par
        par = _re.sub(r"\$([^\$]*)\$\$([_\^]\{[^\$]*\})\$", r"$\1\2$", par)
        par = _re.sub(r"\$([^\$]+)\$\$([^\$]+)\$", r"$\1\2$", par)
    return par


def split_displays(txt, ctx):
    import re as _re
    out, pos = [], 0
    for m in _re.finditer("\ue000(\\d+)\ue001", txt):
        if m.start() > pos:
            out.append(("p", txt[pos:m.start()]))
        out.append(("eq", ctx.displays[int(m.group(1))]))
        pos = m.end()
    if pos < len(txt):
        out.append(("p", txt[pos:]))
    return out
