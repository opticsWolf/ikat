"""Fetch the 10-paper layout-bench corpus into examples/bench-layout/.

Third-party publisher content is NEVER committed: this script (re)builds
the local cache from examples/bench_manifest.json, which records only
ids, DOIs, source URLs, and license evidence (CC-BY 4.0 only — the PLOS
family applies CC-BY 4.0 to every article; no NC/ND).

Per paper it downloads
- <id>.pdf — the publisher PDF (`&type=printable` link in the manifest)
- <id>.xml — the JATS manuscript XML (`&type=manuscript`), the input
  export_full.py weaves from
and, when the optional extractors are installed, writes
- <id>.md (pdf-oxide, MIT: to_markdown_all) + <id>.txt + corpus.json
  (pymupdf, AGPL — bench-only: page/char/image-rect inventory).

Usage (from the repo root, ikat .venv):
    .venv/Scripts/python.exe examples/bench_fetch.py [paper-id ...]

Existing files are kept (size > 0); pass paper ids to (re)fetch a
subset. Cache is ~18 MB, git-ignored.
"""

import json
import sys
from pathlib import Path

HERE = Path(__file__).parent
OUT = HERE / "bench-layout"
MANIFEST = json.loads((HERE / "bench_manifest.json").read_text(encoding="utf-8"))


def download(url: str, dest: Path) -> bool:
    if dest.exists() and dest.stat().st_size > 0:
        return False
    import urllib.request

    print(f"GET {url} -> {dest.relative_to(HERE)}")
    dest.parent.mkdir(parents=True, exist_ok=True)
    urllib.request.urlretrieve(url, dest)
    return True


def plos_xml_url(article_url: str) -> str:
    """The JATS manuscript XML next to a PLOS article page."""
    assert "article?id=" in article_url, article_url
    return article_url.replace("article?id=", "article/file?id=") + "&type=manuscript"


def extract(pdf: Path, md: Path, txt: Path) -> dict | None:
    """Hybrid extraction (bench-only, never shipped); None when the
    optional extractors are not installed."""
    try:
        import pymupdf
        from pdf_oxide import PdfDocument
    except ImportError:
        return None
    doc = pymupdf.open(pdf)
    full = [p.get_text() for p in doc]
    txt.write_text("\n\n\x0c\n\n".join(full), encoding="utf-8")
    text = "\n".join(full)
    import re

    figs = len(re.findall(r"Figure\s+\d|Fig\.?\s*\d|FIG\.?\s*\d", text)) + len(re.findall(r"\bFigure\b", text))
    tabs = len(re.findall(r"\bTable\s+\d", text)) + len(re.findall(r"\bTable\b", text))
    pages_detail = []
    for i, page in enumerate(doc):
        rects = []
        for img in page.get_images(full=True):
            try:
                for r in page.get_image_rects(img):
                    rects.append([round(r.width, 1), round(r.height, 1)])
            except Exception:
                pass
        pages_detail.append({"page": i + 1, "chars": len(full[i]),
                             "images": len(rects), "image_rects_pt": rects})
    ox = PdfDocument(str(pdf))
    assert ox.page_count() == len(doc), f"{pdf.name}: oxide vs mupdf page count"
    md.write_text(ox.to_markdown_all(), encoding="utf-8")
    n_tables = 0
    for i in range(ox.page_count()):
        try:
            n_tables += len(ox.extract_tables(i))
        except Exception:
            pass
    return {"pages": len(doc), "fig_mentions": figs,
            "table_mentions": tabs, "tables_detected": n_tables,
            "pages_detail": pages_detail}


def main(argv: list[str]) -> None:
    OUT.mkdir(exist_ok=True)
    corpus = []
    for p in MANIFEST["papers"]:
        if argv and p["id"] not in argv:
            continue
        pdf = OUT / (p["id"] + ".pdf")
        xml = OUT / (p["id"] + ".xml")
        download(p["pdf"], pdf)
        download(plos_xml_url(p["article"]), xml)
        info = extract(pdf, OUT / (p["id"] + ".md"), OUT / (p["id"] + ".txt"))
        entry = {**p, "pdf_bytes": pdf.stat().st_size,
                 "xml_bytes": xml.stat().st_size}
        if info:
            entry |= info
            print(f"{p['id']}: {info['pages']}p figs~{info['fig_mentions']} tabs~{info['table_mentions']}/{info['tables_detected']}")
        else:
            print(f"{p['id']}: fetched (extractors not installed: pip install pdf-oxide pymupdf)")
        corpus.append(entry)
    if corpus and not argv:
        (OUT / "corpus.json").write_text(json.dumps(corpus, indent=1), encoding="utf-8")
        print(f"wrote corpus.json ({len(corpus)} papers)")


if __name__ == "__main__":
    main(sys.argv[1:])
