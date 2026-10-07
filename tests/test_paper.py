"""M1.4 acceptance (+M4.4 self-hosting): ikat rebuilds the paper byte-identical.

Builds `paper-2026-10-05.tex` from the draft markdown through
`ikat.document` (Rust core) and diffs against the checked-in file.
Registries live in `paper/ikat-spec.json` (hand-owned, extracted
once from the retired `build-paper.py`); `bib_keys` derive from
`refs.bib` at build time so the two can never drift apart.

Needs the paper tree next to ikat/ (skips otherwise).
"""

import json
from pathlib import Path

import pytest

from ikat import BuildSpec, bib_keys, bib_safe, build_document

IKAT = Path(__file__).parent.parent
PAPER = IKAT.parent / "paper"


def _spec(ref_bib):
    raw = json.loads((PAPER / "ikat-spec.json").read_text(encoding="utf-8"))
    return BuildSpec(
        diagrams=raw["diagrams"],
        plots=raw["plots"],
        table_captions=raw["table_captions"],
        table_specs=raw["table_specs"],
        plot_insert_before=raw["plot_insert_before"],
        title_thanks=raw["title_thanks"],
        author=raw["author"],
        bib_name=raw["bib_name"],
        bib_keys=bib_keys(ref_bib),
    )


def test_counts(paper_texts):
    md, ref_bib, *_ = paper_texts
    r = build_document(md, "", _spec(ref_bib))
    assert (r.n_diagrams, r.n_tables) == (4, 9)


@pytest.fixture(scope="module")
def paper_texts():
    if not (PAPER / "paper-2026-10-05.md").exists():
        pytest.skip("paper tree not next to ikat/")
    md = (PAPER / "paper-2026-10-05.md").read_text(encoding="utf-8")
    ref_bib = (PAPER / "refs.bib").read_text(encoding="utf-8")
    expected_tex = (PAPER / "paper-2026-10-05.tex").read_text(encoding="utf-8")
    expected_bib = (PAPER / "refs-paper.bib").read_text(encoding="utf-8")
    toml_src = ""  # paper build uses IEEEtran conference defaults
    return md, ref_bib, expected_tex, expected_bib, toml_src


def test_counts(paper_texts):
    md, ref_bib, *_ = paper_texts
    r = build_document(md, "", _spec(ref_bib))
    assert (r.n_diagrams, r.n_tables) == (4, 9)


def test_byte_identical_tex(paper_texts):
    md, ref_bib, expected_tex, _, toml_src = paper_texts
    r = build_document(md, toml_src, _spec(ref_bib))
    got, want = r.tex, expected_tex
    if got != want:
        import difflib

        diff = list(
            difflib.unified_diff(
                want.splitlines(), got.splitlines(), "build-paper.py", "ikat", n=2
            )
        )
        pytest.fail("tex differs:\n" + "\n".join(diff[:40]))


def test_byte_identical_bib(paper_texts):
    _, ref_bib, _, expected_bib, _ = paper_texts
    assert bib_safe(ref_bib) == expected_bib
