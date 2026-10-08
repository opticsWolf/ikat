"""Phase F (M4.1): line-numbered errors end to end.

Rust renders `{msg}\\n --> line {N}\\n{echo}` (no line part when no
manuscript line applies); `build_from_paths` (and the CLI) upgrade
that to `file:line: message` (+ echo) — template/skeleton errors
resolve against the head file, everything else against the md.
Pure-string API calls keep the Rust rendering verbatim.
"""

import re

import pytest

from ikat import BuildSpec, DiagramEntry, build_document, build_from_paths
from ikat import preset_to_tikz


def _spec(**kw):
    kw.setdefault("diagrams", [])
    kw.setdefault("plots", [])
    kw.setdefault("table_captions", [])
    return BuildSpec(**kw)


def test_mermaid_statement_error_is_file_line(tmp_path):
    # Sequence dispatch inside a fence: `loop` is outside the
    # strict subset (body line 4 → md line 7).
    md = ("# T\n\n```mermaid {inline}\n"
          "sequenceDiagram\n"
          "participant A\n"
          "participant B\n"
          "loop x\n"
          "```\n")
    (tmp_path / "doc.md").write_text(md, encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    spec = _spec(diagrams=[DiagramEntry(key="x", caption="C.", mode="inline")])
    with pytest.raises(ValueError, match=r"doc\.md:7:.*unsupported statement") as ei:
        build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", spec)
    assert "loop x" in str(ei.value)  # echo carries the statement


def test_dangling_key_error_is_file_line(tmp_path):
    (tmp_path / "doc.md").write_text("# T\n\nSee [`nope`] here.\n", encoding="utf-8")
    (tmp_path / "ikat.toml").write_text("", encoding="utf-8")
    spec = _spec()
    spec.bib_keys = {"a"}
    with pytest.raises(ValueError, match=r"doc\.md:3:.*dangling citation key") as ei:
        build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", spec)
    assert "nope" in str(ei.value)


def test_titlesec_error_points_at_head_file(tmp_path):
    (tmp_path / "doc.md").write_text("# T\n\nBody.\n", encoding="utf-8")
    (tmp_path / "ikat.toml").write_text(
        '[template]\npreamble_file = "head.tex"\n', encoding="utf-8")
    (tmp_path / "head.tex").write_text(
        "\\documentclass[conference]{IEEEtran}\n"
        "\\usepackage[nobottomtitles]{titlesec}\n"
        "\\begin{document}\n", encoding="utf-8")
    with pytest.raises(ValueError, match=r"head\.tex:2:.*titlesec") as ei:
        build_from_paths(tmp_path / "doc.md", tmp_path / "ikat.toml", _spec())
    assert "typography" in str(ei.value)


def test_data_error_has_no_line_part():
    # Legend words ride JSON, never the manuscript: no line part,
    # the echo carries the word.
    with pytest.raises(ValueError, match="preset legend") as ei:
        preset_to_tikz('{"kind": "bar", "title": "t", "ylabel": "y", '
                       '"legend": "middle", '
                       '"group_labels": ["g"], "series_names": ["s"], '
                       '"values": [[1]], "mins": [[1]], "maxs": [[1]]}')
    text = str(ei.value)
    assert " --> line" not in text
    assert "middle" in text


def test_string_api_keeps_rust_rendering():
    # No file anywhere: the Rust Display crosses verbatim.
    spec = _spec(diagrams=[DiagramEntry(key="x", caption="C.", mode="inline")])
    md = "# T\n\n```mermaid {inline}\ngraph TD\na[x]\nbogus !!\n```\n"
    with pytest.raises(ValueError, match=re.escape(" --> line 6")) as ei:
        build_document(md, "", spec)
    assert "bogus !!" in str(ei.value)
