# Read-only analysis: where do authors/affiliations drop out of the
# export chain? Original PDF -> JATS cache -> export_full.py -> md -> tex.
import xml.etree.ElementTree as ET
from pathlib import Path

HERE = Path(__file__).parent
cache = HERE / "bench-layout"
full = HERE / "auto-proof" / "full"

def snap(s, n=70):
    return re.sub(r"\s+", " ", s).strip()[:n] if (s := s or "") else ""

import re
print("== JATS cache (bench-layout/<id>.xml) ==")
for x in sorted(cache.glob("*.xml")):
    r = ET.parse(x).getroot()
    contribs = r.findall(".//contrib")
    with_given = sum(1 for c in contribs if c.find("name/given-names") is not None)
    affs = r.findall(".//aff")
    # first contrib's full name + first aff label/institution as a sample
    sample = ""
    if contribs:
        c = contribs[0]
        sample = f"{c.findtext('name/given-names') or ''} {c.findtext('name/surname') or ''}".strip()
    print(f"{x.name:28s} contribs={len(contribs):3d} given={with_given:3d} affs={len(affs):3d}  e.g. {sample!r}")

print()
print("== what the exporter wrote (md line 1-2, frontmatter keys) ==")
for d in sorted(p for p in full.iterdir() if p.is_dir()):
    mds = list(d.glob("*.md"))
    if not mds:
        continue
    md = mds[0]
    head = md.read_text(encoding="utf-8", errors="replace").splitlines()[:4]
    authors_lines = [ln for ln in md.read_text(encoding="utf-8", errors="replace").splitlines()[:200]
                     if "et al" in ln.lower()][:2]
    print(f"{d.name:24s} first lines: {[h[:60] for h in head[:2]]}")
    print(f"{'':24s} 'et al' lines: {authors_lines[:1]}")

print()
print("== what reached the woven TeX (\\\\title / \\\\author) ==")
for t in sorted((full / "out").glob("*.tex")):
    txt = t.read_text(encoding="utf-8", errors="replace")
    tline = next((ln for ln in txt.splitlines() if ln.startswith("\\title")), "(no \\title)")
    aline = next((ln for ln in txt.splitlines() if ln.startswith("\\author")), "(no \\author)")
    print(f"{t.name:28s} {tline[:60]}")
    print(f"{'':28s} {aline[:60]}")
