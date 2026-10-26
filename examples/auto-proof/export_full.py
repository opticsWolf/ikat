"""Full-content md export of the 10 bench papers (text + tables + pictures).

Reads the bench fetch cache (../bench-layout/<id>.xml JATS for
structure/captions/tables/refs + ../bench-layout/<id>.pdf for raster
figures via PyMuPDF, AGPL bench-only like fetch.py) and writes, per
paper, full/<id>/<id>.md + <id>.bib + figs/*.png. Then each md runs
the engine (suggest -> apply) and weaves + compiles to full/out/
as a weave proof.

Pictures ride precompiled mermaid fences (path_prefix="figs/"):
the md stays editable source, the PNGs are measured (PDF rects)
and hand-pinned with engine-mirroring thresholds (short -> force,
wide -> top, tall -> page, else here). The engine keeps pictures
as-is (size unknown to it) — the export is the measurer.

Run from the ikat repo root with the ikat .venv:
  .venv/Scripts/python.exe examples/auto-proof/export_full.py [paper-id...]
With no args, exports all 10. Needs TinyTeX on PATH for the proofs.
"""

import json
import os
import re
import shutil
import sys
import unicodedata
import xml.etree.ElementTree as ET
from pathlib import Path

HERE = Path("examples/auto-proof").resolve()
SRC = Path("examples/bench-layout").resolve()
OUT = HERE / "full"

os.environ["PATH"] = (
    r"C:\Users\Main\AppData\Roaming\TinyTeX\bin\windows"
    + os.pathsep
    + os.environ["PATH"]
)

from ikat import BuildSpec, DiagramEntry, apply_layout, build_document, compile_pdf, suggest_layout  # noqa: E402

import pymupdf  # noqa: E402  (AGPL, bench-only like fetch.py)

MIN_AREA_CM2 = 5.0  # bench-proven bar: kills logos, equation shards, rule tiles
PT_CM = 2.54 / 72
COL_CM = 8.9
TEXT_CM = 17.8

# Unicode -> ASCII or protected $..$ math.
# PASS: the escaper (src/esc.rs UNI/SYM_TOKENS) already covers these —
# folding them here would only shadow it (en dashes became "?" once).
PASS = set("\u2014\u2013\u2026\u2248\u2205\u2264\u00d7\u2192\u2265"
            "\u00b5\u2082\u2081\u2083\u2713\u25cb\u25d0\u2020\u00a7")
FOLD = {
    "\u2212": "$-$", "\u00b1": "$\\pm$", "\u00b0": "$^{\\circ}$",
    "\u00d7": "$\\times$", "\u00f7": "$\\div$",
    "\u03b1": "$\\alpha$", "\u03b2": "$\\beta$", "\u03b3": "$\\gamma$",
    "\u03b4": "$\\delta$", "\u03b5": "$\\varepsilon$", "\u03ba": "$\\kappa$",
    "\u03bb": "$\\lambda$", "\u03bc": "$\\mu$", "\u03c0": "$\\pi$",
    "\u03c3": "$\\sigma$", "\u03c4": "$\\tau$", "\u03c7": "$\\chi$",
    "\u03a9": "$\\Omega$", "\u0394": "$\\Delta$",
    "\u2018": "'", "\u2019": "'", "\u201c": '"', "\u201d": '"',
    "\u00a0": " ", "\u2009": " ", "\u200b": "",
    "\u00b9": "$^{1}$", "\u00b2": "$^{2}$", "\u00b3": "$^{3}$",
    "\u2070": "$^{0}$", "\u2074": "$^{4}$", "\u2075": "$^{5}$",
    "\u2076": "$^{6}$", "\u2077": "$^{7}$", "\u2078": "$^{8}$",
    "\u2079": "$^{9}$", "\u207a": "$^{+}$", "\u207b": "$^{-}$",
    "\u207f": "$^{n}$",
    "\u2080": "$_{0}$", "\u2083": "$_{3}$", "\u2084": "$_{4}$",
    "\u2085": "$_{5}$", "\u2086": "$_{6}$", "\u2087": "$_{7}$",
    "\u2088": "$_{8}$", "\u2089": "$_{9}$", "\u2092": "$_{o}$",
    "\u208a": "$_{+}$", "\u208b": "$_{-}$",
    "\u22c6": "$\\star$", "\u25b3": "$\\triangle$",
    "\u03b7": "$\\eta$", "\u03b8": "$\\theta$", "\u03b9": "$\\iota$",
    "\u03bd": "$\\nu$", "\u03be": "$\\xi$", "\u03c1": "$\\rho$",
    "\u03c2": "$\\varsigma$", "\u03c6": "$\\varphi$", "\u03c8": "$\\psi$",
    "\u03c9": "$\\omega$", "\u0393": "$\\Gamma$", "\u0398": "$\\Theta$",
    "\u039b": "$\\Lambda$", "\u039e": "$\\Xi$", "\u03a0": "$\\Pi$",
    "\u03a3": "$\\Sigma$", "\u03a6": "$\\Phi$", "\u03a8": "$\\Psi$",
    "\u27e8": "<", "\u27e9": ">", "\u2032": "'", "\u2033": "''",
    "\u2219": "$\\cdot$", "\u2202": "$\\partial$", "\u2207": "$\\nabla$",
    "\u221e": "$\\infty$", "\u222b": "$\\int$", "\u2211": "$\\sum$",
    "\u2295": "$\\oplus$", "\u2297": "$\\otimes$", "\u210f": "$\\hbar$",
    "\u2208": "$\\in$", "\u2209": "$\\notin$", "\u220b": "$\\ni$",
    "\u2282": "$\\subset$", "\u2286": "$\\subseteq$",
    "\u222a": "$\\cup$", "\u2229": "$\\cap$",
    "\u2200": "$\\forall$", "\u2203": "$\\exists$", "\u00ac": "$\\neg$",
    "\u2227": "$\\wedge$", "\u2228": "$\\vee$",
    "\u2218": "$\\circ$", "\u22c5": "$\\cdot$", "\u22c6": "$\\star$",
    "\u22ef": "$\\cdots$", "\u22ee": "$\\vdots$", "\u22f1": "$\\ddots$",
    "\u3008": "<", "\u3009": ">", "\u2329": "<", "\u232a": ">",
    "\u201e": '"', "\u201c": '"', "\u201d": '"',
    "\u0142": "l", "\u0141": "L", "\u00f8": "o", "\u00d8": "O",
    "\u00e6": "ae", "\u00c6": "Ae", "\u0153": "oe", "\u00df": "ss",
    "\u0111": "d", "\u014b": "ng", "\u0131": "i",
    "\u03c5": "$\\upsilon$", "\u03f5": "$\\epsilon$", "\u03d0": "$\\beta$",
    "\u03d1": "$\\theta$", "\u03d5": "$\\phi$", "\u03d6": "$\\varpi$",
    "\u2254": "$:=$", "\u2261": "$\\equiv$", "\u25c7": "$\\diamond$",
    "\u2016": "$\\Vert$", "\u223c": "$\\sim$",
    "\u2115": "$\\mathbb{N}$", "\u211d": "$\\mathbb{R}$",
    "\u2124": "$\\mathbb{Z}$", "\u211a": "$\\mathbb{Q}$",
    "\u2010": "-", "\u00ab": '"', "\u00bb": '"', "\u201a": ",",
    "\u00ae": "(R)", "\u2003": " ", "\u200a": " ", "\u02dc": "~",
    "\u2550": "", "\u200d": "", "\u2118": "$\\wp$",
    "\u211b": "$\\Re$", "\u2131": "$\\mathcal{F}$",
    "\u017b": "z", "\u017c": "Z",
    "\u2190": "$\\leftarrow$", "\u2192": "$\\to$",
    "\u21d2": "$\\Rightarrow$", "\u221e": "$\\infty$",
    "\u2264": "$\\leq$", "\u2265": "$\\geq$", "\u2260": "$\\neq$",
    "\u2248": "$\\approx$", "\u2026": "...", "\u0131": "i",
}


def fold(s: str, stats: dict) -> str:
    out = []
    for ch in s:
        if ord(ch) < 128:
            out.append("\\$" if ch == "$" else ch)
        elif ch in PASS:
            out.append(ch)  # src/esc.rs covers it at weave time
        elif ch in FOLD:
            out.append(FOLD[ch])
        else:
            stats["folded"] = stats.get("folded", 0) + 1
            stats["fold_chars"] = sorted(set(stats.get("fold_chars", [])) | {f"U+{ord(ch):04X}"})[:12]
            decomp = unicodedata.normalize("NFKD", ch)
            if decomp in FOLD:
                out.append(FOLD[decomp])
            else:
                out.append(decomp.encode("ascii", "ignore").decode() or "?")
    return "".join(out)


def mathsafe(s: str, stats: dict, what: str) -> str:
    """Weave guard: an odd $ count kills pdflatex (Missing }). Strip and count."""
    if s.count("$") % 2:
        stats["unbalanced_math"] = stats.get("unbalanced_math", 0) + 1
        stats["unbalanced_where"] = sorted(set(stats.get("unbalanced_where", []) | {what}))[:6]
        return s.replace("$", "")
    return s


def clean_prose(s: str, ctx: Ctx, what: str) -> str:
    """Paragraph-level weave guards: pipes become tables under the
    re-scan (bra-ket kills the caption registry), so they go first."""
    if "|" in s:
        ctx.stats["pipes"] = ctx.stats.get("pipes", 0) + s.count("|")
        s = s.replace("|", "/")
    if "[ [`ref" in s:
        ctx.stats["brackets"] = ctx.stats.get("brackets", 0) + s.count("[ [`ref")
        s = s.replace("[ [`ref", "[[`ref")
    return mathsafe(s, ctx.stats, what)


def local(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


class Ctx:
    def __init__(self):
        self.refkey = {}   # rid -> refNN
        self.stats = {}
        self.dropped = {}


def rich_text(el: ET.Element, ctx: Ctx) -> str:
    """JATS inline -> md: cites, sub/sup math, emphasis; rest flattened."""
    parts = []

    def rec(node: ET.Element):
        t = local(node.tag)
        if t == "xref" and node.get("ref-type") == "bibr":
            for rid in (node.get("rid") or "").split():
                if rid in ctx.refkey:
                    parts.append(f" [`{ctx.refkey[rid]}`]")
            return
        if t in ("sup",):
            inner = fold("".join(node.itertext()), ctx.stats)
            # MathML inside (mi gamma etc.) already folds to $..$:
            # wrapping again nests math spans and kills pdflatex.
            parts.append(mathsafe(inner, ctx.stats, t) if "$" in inner else f"$^{{{inner}}}$")
            return
        if t in ("sub",):
            inner = fold("".join(node.itertext()), ctx.stats)
            parts.append(mathsafe(inner, ctx.stats, t) if "$" in inner else f"$_{{{inner}}}$")
            return
        if t in ("italic", "i"):
            parts.append("*" + fold("".join(node.itertext()), ctx.stats) + "*")
            return
        if t in ("bold", "b"):
            parts.append("**" + fold("".join(node.itertext()), ctx.stats) + "**")
            return
        if t in ("ext-link", "uri"):
            parts.append(fold("".join(node.itertext()), ctx.stats))
            return
        if t in ("inline-formula", "disp-formula"):
            ctx.dropped["formula"] = ctx.dropped.get("formula", 0) + 1
            parts.append(fold("".join(node.itertext()), ctx.stats))
            return
        if node.text:
            parts.append(fold(node.text, ctx.stats))
        for ch in node:
            rec(ch)
            if ch.tail:
                parts.append(fold(ch.tail, ctx.stats))

    rec(el)
    return re.sub(r"[ \t\xa0]+", " ", "".join(parts)).strip()


def para_text(p: ET.Element, ctx: Ctx) -> str:
    return rich_text(p, ctx)


def table_md(tw: ET.Element, ctx: Ctx) -> tuple[str, str]:
    """Real JATS table -> (md table, caption). Full rows, no cap."""
    cap_el = tw.find("caption")
    label = "".join(tw.findtext("label") or "").strip()
    cap = re.sub(r"\s+", " ", "".join(cap_el.itertext())).strip() if cap_el is not None else ""
    cap = fold(cap, ctx.stats)
    cap = re.sub(r"^(Fig(ure)?|Table)\.?\s*\w+\.?\s*", "", cap)
    # Bare caption: the class prepends its own "TABLE N." (IEEEtran, article).
    caption = mathsafe(cap or "Table.", ctx.stats, "caption")
    rows = []
    for tr in tw.findall(".//tr"):
        cells = []
        for c in tr:
            if local(c.tag) not in ("td", "th"):
                continue
            cell = re.sub(r"\s+", " ", rich_text(c, ctx)).replace("|", " / ")
            cell = mathsafe(cell, ctx.stats, "cell")
            if cell.lstrip().startswith("["):
                cell = "{} " + cell  # \\[ trap: row-start [ reads as \\ dimen
            cells.append(cell)
        if cells:
            rows.append(cells)
    if not rows:
        ctx.dropped["empty-table"] = ctx.dropped.get("empty-table", 0) + 1
        return "", caption
    width = max(len(r) for r in rows)
    rows = [r + [""] * (width - len(r)) for r in rows]
    head, *body = rows
    lines = ["| " + " | ".join(head) + " |", "|" + "|".join("---" for _ in head) + "|"]
    lines += ["| " + " | ".join(r) + " |" for r in body]
    return "\n".join(lines) + "\n", caption


def walk(sec: ET.Element, depth: int, ctx: Ctx, blocks: list, floats: list):
    for child in sec:
        t = local(child.tag)
        if t == "title":
            continue
        if t == "sec":
            title = fold(re.sub(r"\s+", " ", "".join(child.findtext("title") or "")).strip(), ctx.stats)
            blocks.append(("h", depth + 1, title))
            walk(child, depth + 1, ctx, blocks, floats)
        elif t == "p":
            txt = para_text(child, ctx)
            if txt:
                blocks.append(("p", 0, txt))
        elif t == "fig":
            floats.append(("fig", child, len(blocks)))
            blocks.append(("FIG", 0, child.get("id", "")))
        elif t == "table-wrap":
            floats.append(("tab", child, len(blocks)))
            blocks.append(("TAB", 0, child.get("id", "")))
        elif t in ("list",):
            for item in child.findall(".//list-item"):
                txt = para_text(item, ctx)
                if txt:
                    blocks.append(("p", 0, txt))
        elif t in ("disp-formula", "supplementary-material", "media", "boxed-text"):
            if t == "boxed-text":
                for p in child.findall(".//p"):
                    txt = para_text(p, ctx)
                    if txt:
                        blocks.append(("p", 0, txt))
            else:
                ctx.dropped[t] = ctx.dropped.get(t, 0) + 1
        elif t in ("caption", "label", "object-id"):
            continue
        else:
            ctx.dropped[t] = ctx.dropped.get(t, 0) + 1


def extract_images(pdf_path: Path, figdir: Path) -> list:
    """Embedded rasters -> PNGs, reading order, tiny-filtered."""
    doc = pymupdf.open(pdf_path)
    got = []
    seen = set()
    for i, page in enumerate(doc):
        for img in page.get_images(full=True):
            xref = img[0]
            if xref in seen:
                continue
            seen.add(xref)
            try:
                rects = page.get_image_rects(xref)
            except Exception:
                continue
            if not rects:
                continue
            r = max(rects, key=lambda q: q.width * q.height)
            w, h = r.width * PT_CM, r.height * PT_CM
            if w * h < MIN_AREA_CM2:
                continue
            try:
                pix = pymupdf.Pixmap(doc, xref)
                if pix.n - pix.alpha > 3:
                    pix = pymupdf.Pixmap(pymupdf.csRGB, pix)
                png = pix.tobytes("png")
            except Exception:
                continue
            got.append({"page": i + 1, "y": round(r.y0, 1), "w": round(w, 2),
                        "h": round(h, 2), "xref": xref, "png": bytes(png)})
    got.sort(key=lambda g: (g["page"], g["y"]))
    figdir.mkdir(parents=True, exist_ok=True)
    for n, g in enumerate(got):
        (figdir / f"img{n:02d}.png").write_bytes(g["png"])
        g["file"] = f"img{n:02d}.png"
    return got


TEXT_H_CM = 23.0
TOP_MAX_CM = 0.7 * TEXT_H_CM  # mirrors engine [layout] top_frac
TAIL_LINES = 6  # mirrors engine [layout] tail_lines


def pic_attrs(w: float, h: float, tail: bool = False) -> str:
    """Measured-size pinning, mirroring the engine thresholds.
    Wide floats taller than the page-top budget take a dedicated
    page: [t] cannot hold them and the queue strands at the end."""
    if w >= COL_CM * 1.25:
        if h + 1.0 > TOP_MAX_CM or tail:
            return f"{{span=wide pos=page width={min(1.0, round(w / TEXT_CM, 2))}}}"
        return f"{{span=wide pos=top width={min(1.0, round(w / TEXT_CM, 2))}}}"
    if h > 14.0:
        return "{span=column pos=page}"
    if h <= 7.0:
        return f"{{span=column pos=force width={min(1.0, round(w / COL_CM, 2))}}}"
    return f"{{span=column pos=here width={min(1.0, round(w / COL_CM, 2))}}}"


def bib_entry(num: int, ref: ET.Element, ctx: Ctx) -> str:
    raw = re.sub(r"\s+", " ", "".join(ref.itertext())).strip()
    raw = raw.replace("\\", "\\textbackslash{}").replace("&", "\\&").replace("%", "\\%")
    raw = raw.replace("#", "\\#").replace("_", "\\_").replace("{", "\\{").replace("}", "\\}")
    return f"@misc{{ref{num:02d},\n  note={{{fold(raw, ctx.stats)}}}\n}}\n"


def export_one(pid: str) -> dict:
    xml_path, pdf_path = SRC / f"{pid}.xml", SRC / f"{pid}.pdf"
    work = OUT / pid
    figdir = work / "figs"
    if work.exists():
        try:
            shutil.rmtree(work)
        except OSError:
            # Windows: a lingering handle (shell, indexer) can lock
            # the dir itself while its contents stay writable — empty
            # it file by file instead of giving up the whole batch.
            for child in work.iterdir():
                try:
                    if child.is_dir() and not child.is_symlink():
                        shutil.rmtree(child)
                    else:
                        child.unlink()
                except OSError:
                    pass
    work.mkdir(parents=True, exist_ok=True)
    ctx = Ctx()

    root = ET.parse(xml_path).getroot()
    ref_list = root.find("back/ref-list")
    refs = ref_list.findall("ref") if ref_list is not None else []
    for n, ref in enumerate(refs, 1):
        rid = ref.get("id", "")
        if rid:
            ctx.refkey[rid] = f"ref{n:02d}"

    front = root.find("front/article-meta")
    title_el = front.find("title-group/article-title") if front is not None else None
    if title_el is None:
        title_el = ET.Element("nil")
    title = mathsafe(re.sub(r"\s+", " ", rich_text(title_el, ctx)).strip(), ctx.stats, "title") or pid
    surnames = [c.findtext("name/surname") or "" for c in front.findall(".//contrib")]
    surnames = [s for s in surnames if s]
    author = f"{surnames[0]} et al." if surnames else "PLOS ONE"
    n_authors = len(surnames)

    md = [f"# {title}\n"]
    diagrams, captions = [], []
    n_fig_floats = n_tab_floats = n_unmatched = 0

    abstract = root.find("front/article-meta/abstract")
    if abstract is not None:
        md.append("## Abstract\n")
        for p in abstract.findall("p"):
            txt = para_text(p, ctx)
            if txt:
                md.append(txt + "\n")

    body = root.find("body")
    blocks: list = []
    floats: list = []
    if body is not None:
        for child in body:
            t = local(child.tag)
            if t == "sec":
                stitle = fold(re.sub(r"\s+", " ", "".join(child.findtext("title") or "")).strip(), ctx.stats)
                blocks.append(("h", 1, stitle or "Section"))
                walk(child, 1, ctx, blocks, floats)
            elif t == "p":
                txt = para_text(child, ctx)
                if txt:
                    blocks.append(("p", 0, txt))
            else:
                ctx.dropped[f"body:{t}"] = ctx.dropped.get(f"body:{t}", 0) + 1

    ack = root.find("back/ack")
    if ack is not None:
        txts = [para_text(p, ctx) for p in ack.findall(".//p")]
        txts = [t for t in txts if t]
        if txts:
            blocks.append(("h", 1, "Acknowledgements"))
            blocks += [("p", 0, t) for t in txts]

    # Pictures: match body-order figs to reading-order PNGs.
    images = extract_images(pdf_path, figdir) if pdf_path.exists() else []
    fig_hits = [f for f in floats if f[0] == "fig"]
    pool = list(images)
    fig_files: dict = {}
    for _kind, fig_el, _pos in fig_hits:
        if pool:
            g = pool.pop(0)
            fig_files[fig_el.get("id", "")] = g
        else:
            n_unmatched += 1

    # Paragraph-based: a JATS paragraph is one prose unit however its
    # source newlines fall (line-counting undercounts folded text and
    # fires the tail rule pages early — chem Fig 5 stood alone).
    total_prose = sum(1 for k, _, pay in blocks if k == "p" and pay.strip())
    seen_prose = 0
    for bkind, depth, payload in blocks:
        if bkind == "h":
            payload = clean_prose(payload, ctx, "heading")
            hashes = "##" if depth == 1 else "###" if depth == 2 else ""
            if hashes:
                md.append(f"{hashes} {payload}\n")
            else:
                md.append(f"**{payload}.**\n")
        elif bkind == "p":
            seen_prose += 1 if payload.strip() else 0
            payload = clean_prose(payload, ctx, "para")
            if payload:
                md.append(payload + "\n")
        elif bkind == "FIG":
            n_fig_floats += 1
            fig_el = next(f for k, f, _ in fig_hits if f.get("id", "") == payload)
            cap_el = fig_el.find("caption")
            label = "".join(fig_el.findtext("label") or "").strip()
            cap = re.sub(r"\s+", " ", "".join(cap_el.itertext())).strip() if cap_el is not None else ""
            cap = re.sub(r"^(Fig(ure)?|Table)\.?\s*\w+\.?\s*", "", cap)
            # Bare caption: the class prepends its own "Fig. N." — the
            # showcase registry convention ("The ikat build: ...").
            caption = mathsafe(fold(cap or "Figure.", ctx.stats), ctx.stats, "figcap")
            key = f"{pid}-fig{n_fig_floats}"
            g = fig_files.get(payload)
            if g is None:
                # No fence: imageless floats stay prose (honest, weave-safe).
                md.append(f"**{fold(label or 'Figure', ctx.stats)}.** {caption} (figure image not in PDF extract).\n")
            else:
                (figdir / f"{key}.png").write_bytes(g["png"])
                (figdir / g["file"]).unlink(missing_ok=True)
                md.append(f"```mermaid {pic_attrs(g['w'], g['h'], tail=total_prose > 0 and total_prose - seen_prose < TAIL_LINES)}\n{fold(fig_el.find('graphic').get('{http://www.w3.org/1999/xlink}href', key) if fig_el.find('graphic') is not None else key, ctx.stats)}\n```\n")
                diagrams.append(DiagramEntry(key=key, caption=caption, mode="precompiled", path_prefix="figs/"))
        elif bkind == "TAB":
            n_tab_floats += 1
            tw = next(f for k, f, _ in floats if k == "tab" and f.get("id", "") == payload)
            tmd, caption = table_md(tw, ctx)
            if not tmd:
                md.append(f"**Table.** {caption} (empty in JATS, omitted).\n")
                continue
            captions.append(caption)
            md.append("%% table\n\n" + tmd)

    bib = "".join(bib_entry(n, r, ctx) for n, r in enumerate(refs, 1))
    (work / f"{pid}.bib").write_text(bib, encoding="utf-8")
    bst = HERE.parent / "ikat-paper" / "out" / "IEEEtran.bst"
    if bst.exists():
        shutil.copy(bst, work / "IEEEtran.bst")
    md_text = "\n".join(md)
    (work / f"{pid}.md").write_text(md_text, encoding="utf-8")

    spec = BuildSpec(
        diagrams=diagrams,
        table_captions=captions,
        author=author,
        bib_name=pid,
        bib_keys={f"ref{n:02d}" for n in range(1, len(refs) + 1)},
    )
    # Corpus weave policy: pending floats flush before the reference
    # list (otherwise end-queued figures spill past `References`)
    # Section barriers stay OFF: a barrier meeting a still-pending
    # wide float ejects the column (placeins clearpage), leaving a
    # half-empty column behind. Tail floats go to pages instead (no
    # downstream tops left), so the refs barrier meets empty defer
    # lists and never ejects. Dense regression tables compact and
    # rotate automatically ([tables] opt-in: portrait cannot hold
    # unbreakable numeric cells wider than their X share).
    toml_src = "[floats]\nbarrier_before_refs = true\n[tables]\ncompact_tables = true\n"
    suggs = suggest_layout(md_text, toml_src, spec)
    new_md, n_applied = apply_layout(md_text, suggs)
    (work / f"{pid}-auto.md").write_text(new_md, encoding="utf-8")
    r = build_document(new_md, toml_src, spec)
    (work / f"{pid}.tex").write_text(r.tex, encoding="utf-8")
    for png in figdir.glob("*.png"):
        pass  # staged beside the tex via figs/ graphicspath
    (work / f"{pid}.bib").replace(work / f"{pid}.bib")  # staged already
    pdf = compile_pdf(work, f"{pid}.tex", engine="pdflatex")
    proof = OUT / "out"
    proof.mkdir(exist_ok=True)
    shutil.copy(pdf, proof / f"{pid}.pdf")
    log = (work / f"{pid}.log").read_text(encoding="utf-8", errors="replace")
    overfull = len(re.findall(r"^Overfull", log, re.M))
    dist: dict = {}
    for s in suggs:
        dist[f"{s['kind']}+{s['span']}+{s['pos']}"] = dist.get(f"{s['kind']}+{s['span']}+{s['pos']}", 0) + 1
    return {"id": pid, "title": title[:60], "authors": n_authors, "refs": len(refs), "fold_chars": ctx.stats.get("fold_chars", []),
            "figs": n_fig_floats, "tabs": n_tab_floats, "imgs": len(images),
            "unmatched": n_unmatched, "applied": n_applied, "dist": dist,
            "pdf_bytes": pdf.stat().st_size, "overfull": overfull,
            "tex_bytes": len(r.tex.encode()), "dropped": dict(ctx.dropped),
            "folded": ctx.stats.get("folded", 0)}


def main(argv: list) -> None:
    ids = argv or [p["id"] for p in json.loads((SRC / "manifest.json").read_text(encoding="utf-8"))["papers"]]
    OUT.mkdir(exist_ok=True)
    rows = []
    for pid in ids:
        try:
            row = export_one(pid)
        except Exception as e:  # noqa: BLE001 — per-paper report, never abort the batch
            print(f"{pid}: EXPORT-FAIL {type(e).__name__}: {e}")
            continue
        rows.append(row)
        print(f"{row['id']}: figs={row['figs']} tabs={row['tabs']} imgs={row['imgs']} "
              f"unmatched={row['unmatched']} refs={row['refs']} applied={row['applied']} "
              f"overfull={row['overfull']} pdf={row['pdf_bytes']}B dist={row['dist']}")
    rep = ["# full export (JATS text + tables, PDF-extracted PNGs)", ""]
    for r in rows:
        rep.append(f"- {r['id']}: {r['figs']} figs ({r['imgs']} PNGs, {r['unmatched']} imageless), "
                   f"{r['tabs']} tables, {r['refs']} refs, {r['authors']} authors; "
                   f"engine applied {r['applied']}, {r['overfull']} overfull, pdf {r['pdf_bytes']}B; "
                   f"dropped={r['dropped']} folded={r['folded']} leftovers={r.get('fold_chars', [])}")
    (OUT / "REPORT.md").write_text("\n".join(rep) + "\n", encoding="utf-8")
    print(f"wrote {OUT / 'REPORT.md'}")


if __name__ == "__main__":
    main(sys.argv[1:])
