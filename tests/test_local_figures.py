"""Local-figure policy end to end (0.13.4).

`![alt](target)` with a host-verified local target embeds as a real
float (caption from alt, `fig:localN` label, height cap); missing /
wrong-format / multi-page / unverifiable targets degrade with precise
notes. PNG + PDF fixtures are constructed byte-exact (stdlib only).
"""

import struct
import zlib

from ikat import BuildSpec
from ikat.document import LocalFigure, build_from_paths


def _spec(**kw):
    kw.setdefault("diagrams", [])
    kw.setdefault("plots", [])
    kw.setdefault("table_captions", [])
    return BuildSpec(**kw)


def _png():
    def chunk(tag, data):
        c = tag + data
        return struct.pack(">I", len(data)) + c + struct.pack(">I", zlib.crc32(c))

    raw = b"\x00\xff\x00\x00"  # one red pixel
    return (b"\x89PNG\r\n\x1a\n"
            + chunk(b"IHDR", struct.pack(">IIBBBBB", 1, 1, 8, 2, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(raw))
            + chunk(b"IEND", b""))


def _pdf(pages):
    bodies = [b"<< /Type /Catalog /Pages 2 0 R >>"]
    kids = " ".join(f"{3 + i} 0 R" for i in range(pages))
    bodies.append(f"<< /Type /Pages /Kids [{kids}] /Count {pages} >>".encode("ascii"))
    for i in range(pages):
        bodies.append(
            f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 100 100] "
            f"/Contents {3 + pages + i} 0 R >>".encode("ascii"))
    for _ in range(pages):
        bodies.append(b"<< /Length 0 >>\nstream\n\nendstream")
    parts = [b"%PDF-1.4"]
    offsets = []
    for i, body in enumerate(bodies, start=1):
        offsets.append(len(b"\n".join(parts)) + 1)
        parts.append(f"{i} 0 obj".encode("ascii"))
        parts.append(body)
        parts.append(b"endobj")
    xref_pos = len(b"\n".join(parts)) + 1
    xref = ["xref", f"0 {len(bodies) + 1}", "0000000000 65535 f "]
    xref += [f"{o:010d} 00000 n " for o in offsets]
    parts.append("\n".join(xref).encode("ascii"))
    parts.append(
        f"trailer\n<< /Size {len(bodies) + 1} /Root 1 0 R >>\n"
        f"startxref\n{xref_pos}\n%%EOF".encode("ascii"))
    return b"\n".join(parts)


def _paths(tmp_path, md):
    (tmp_path / "doc.md").write_text(md, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    return tmp_path / "doc.md", tmp_path / "ikat.toml"


def test_local_png_embeds_as_float(tmp_path):
    (tmp_path / "a.png").write_bytes(_png())
    md, toml = _paths(tmp_path, "# T\n\n![Cap here](a.png)\n")
    r = build_from_paths(md, toml, _spec())
    assert "\\includegraphics[width=\\textwidth" in r.body
    assert "a.png" in r.body
    assert "\\label{fig:local1}" in r.body
    assert "Cap here" in r.body
    assert any("embedded from" in n and "a.png" in n for n in r.notes)


def test_missing_file_renders_alt_text(tmp_path):
    md, toml = _paths(tmp_path, "# T\n\n![Cap here](nope.png)\n")
    r = build_from_paths(md, toml, _spec())
    assert "includegraphics" not in r.body
    assert "Cap here" in r.body
    assert any("not found" in n for n in r.notes)


def test_gif_refuses_with_hint(tmp_path):
    (tmp_path / "x.gif").write_bytes(b"GIF89a")
    md, toml = _paths(tmp_path, "# T\n\n![Cap](x.gif)\n")
    r = build_from_paths(md, toml, _spec())
    assert "includegraphics" not in r.body
    assert any("convert GIF to PNG" in n for n in r.notes)


def test_multipage_pdf_refuses(tmp_path):
    (tmp_path / "s.pdf").write_bytes(_pdf(2))
    md, toml = _paths(tmp_path, "# T\n\n![Supp](s.pdf)\n")
    r = build_from_paths(md, toml, _spec())
    assert "includegraphics" not in r.body
    assert any("has 2 pages" in n for n in r.notes)


def test_fragment_embeds_with_page(tmp_path):
    (tmp_path / "s.pdf").write_bytes(_pdf(2))
    md, toml = _paths(tmp_path, "# T\n\n![Supp](s.pdf#page=2)\n")
    r = build_from_paths(md, toml, _spec())
    assert "page=2" in r.body
    assert "s.pdf" in r.body


def test_single_page_pdf_embeds(tmp_path):
    (tmp_path / "one.pdf").write_bytes(_pdf(1))
    md, toml = _paths(tmp_path, "# T\n\n![One](one.pdf)\n")
    r = build_from_paths(md, toml, _spec())
    assert "one.pdf" in r.body
    assert "includegraphics" in r.body


def test_garbage_pdf_unverifiable(tmp_path):
    (tmp_path / "g.pdf").write_bytes(b"not a pdf at all")
    md, toml = _paths(tmp_path, "# T\n\n![G](g.pdf)\n")
    r = build_from_paths(md, toml, _spec())
    assert "includegraphics" not in r.body
    assert any("unverifiable" in n for n in r.notes)


def test_prefilled_bridge_entry_wins(tmp_path):
    # No file on disk: host-verified entries are trusted (bridge hook).
    md, toml = _paths(tmp_path, "# T\n\n![B](b.png)\n")
    spec = _spec()
    spec.local_figures = [LocalFigure(target="b.png", path="/srv/figs/b.png",
                                      page=0, embed=True, note="")]
    r = build_from_paths(md, toml, spec)
    assert "/srv/figs/b.png" in r.body
    assert "includegraphics" in r.body
