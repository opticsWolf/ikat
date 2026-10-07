"""Throwaway proof: every shipped head compiles with abstract + TikZ + table."""

import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).parent
sys.path.insert(0, str(HERE.parent / "python"))
sys.path.insert(0, r"D:\User\Documents\Python\Macrame_docs\.venv\Lib\site-packages")

from ikat import BuildSpec, DiagramEntry, build_document, list_templates, template_path, template_preset

MD = """# Probe Title

## Abstract

Probe abstract text here.

## 1. Intro

Body text with a diagram and a table.

```mermaid
graph TD
a[x]-->b[y]
```

| A |
|---|
| 1 |
"""

# REVTeX clashes with tabularx: the APS probe drops the table.
MD_NOTABLE = """# Probe Title

## Abstract

Probe abstract text here.

## 1. Intro

Body text with a diagram.

```mermaid
graph TD
a[x]-->b[y]
```
"""

fails = []
for name in list_templates():
    preset = template_preset(name)
    head = template_path(name).read_text(encoding="utf-8")
    toml = preset["toml"]
    md = MD_NOTABLE if name == "aps" else MD
    captions = [] if name == "aps" else ["T."]
    spec = BuildSpec(
        diagrams=[DiagramEntry(key="fig-x", caption="C.", mode="inline")],
        plots=[], table_captions=captions, bib_style=preset["bib_style"],
        preamble_override=head,
    )
    try:
        r = build_document(md, toml, spec)
    except ValueError as e:
        fails.append((name, f"weave: {e}"))
        continue
    d = HERE / "demo-out" / f"head-{name}"
    d.mkdir(parents=True, exist_ok=True)
    (d / "probe.tex").write_text(r.tex, encoding="utf-8")
    p = subprocess.run(
        ["pdflatex", "-halt-on-error", "-interaction=nonstopmode", "probe.tex"],
        cwd=d, capture_output=True, text=True, timeout=300,
    )
    pdf = d / "probe.pdf"
    if p.returncode != 0 or not pdf.exists():
        tail = (p.stdout + p.stderr)[-600:]
        fails.append((name, f"tex: {tail}"))
        continue
    import pymupdf
    text = " ".join(pymupdf.open(pdf)[0].get_text().split())
    for needle in ["Probe abstract", "Probe Title"]:
        if needle not in text:
            fails.append((name, f"dropped: {needle!r}"))
            break
    else:
        print(f"OK   {name} ({pdf.stat().st_size} bytes)")

print("FAILURES:", fails if fails else "none")
