"""JATS-compatibility end to end (0.13.3): warn-and-render.

A miniature JATS-shaped manuscript — lists, smart quotes, U+2212
minus, legend symbols, a captionless table, a remote figure, an
equation image, a footnote, chemistry subscripts — must weave with
every adaptation reported, and nothing silently corrupted: no
literal `![...]`, `[^1]`, or `<sub>` in the body.
"""

from ikat import BuildSpec, build_document


def _spec(**kw):
    kw.setdefault("diagrams", [])
    kw.setdefault("plots", [])
    kw.setdefault("table_captions", [])
    return BuildSpec(**kw)


JATS_MD = """# Survey platform

## Introduction

Survey “platforms” use plain-text ranges 5−3 for version control[^1].

- text input type
- numeric input type

## References

1. R Core Team. R manual; 2024.
2. Other work; 2023.

| Feature | A | B |
| --- | --- | --- |
| Open | ∎ | □ |
| Stars | ★ | ✗ |

![Fig 1: core technologies](https://example.com/g001.jpg)

Energy ![](https://example.com/e001.jpg) released from H<sub>2</sub>O.

$$x^{2} + y^{2} \\tag{Eq 1}$$

Fused SO$_{2}$NH here.

[^1]: The note.
"""


def test_jats_shaped_manuscript_weaves_with_notes():
    r = build_document(JATS_MD, "", _spec())
    body, notes = r.body, r.notes
    # Nothing silently corrupted.
    assert "![" not in body
    assert "[^1]" not in body
    assert "<sub>" not in body
    # Adaptations rendered.
    assert "text input type" in body
    assert "1. R Core Team" in body
    assert "``platforms''" in body
    assert "5-3" in body
    assert "Table 1" in body
    assert "[{\\#}]" in body and "[ ]" in body
    assert "Fig 1: core technologies" in body
    assert "H$_{2}$O" in body
    assert "\\footnote{The note.}" in body
    assert "\\tag{Eq 1}" in body
    # Every adaptation reported.
    text = "\n".join(notes)
    assert "rendered as paragraphs" in text
    assert "unicode fallbacks" in text
    assert "automatic caption" in text
    assert "figures:" in text
    assert "footnotes: 1" in text
    assert "html:" in text
    assert len(notes) >= 6


def test_still_fails_on_unrepresentable():
    import pytest

    with pytest.raises(ValueError, match="U\\+1F600"):
        build_document("# T\n\nEmoji 😀 here.\n", "", _spec())
