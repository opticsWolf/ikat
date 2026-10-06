"""M1.4 acceptance: ikat rebuilds the paper byte-identical.

Builds `paper-2026-10-05.tex` from the draft markdown through
`ikat.document` (Rust core) and diffs against the checked-in file
produced by `build-paper.py`. Registries (captions, thanks) are read
from `build-paper.py` itself so the two can never drift apart.

Needs the paper tree next to ikat/ (skips otherwise).
"""

import importlib.util
from pathlib import Path

import pytest

from ikat import BuildSpec, DiagramEntry, bib_keys, bib_safe, build_document

IKAT = Path(__file__).parent.parent
PAPER = IKAT.parent / "paper"


def _load_legacy():
    spec = importlib.util.spec_from_file_location(
        "build_paper", PAPER / "build-paper.py"
    )
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


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


def _spec(legacy, ref_bib):
    thanks = (
        "Revision of 2026-10-05: system references updated to 0.19.0; "
        "new material on attributes, bulk bytes, gates, and vector "
        "semantics. Decision codes (D-nnn) refer to the register "
        "appendix. No new measured figures."
    )
    return BuildSpec(
        diagrams=[DiagramEntry(k, c) for k, c in legacy.MERMAID_FIGS],
        plots=list(legacy.PLOT_FIGS),
        table_captions=list(legacy.TABLE_CAPTIONS),
        table_specs={8: "Xcl"},
        plot_insert_before="\\section{Discussion: branching vs alternatives}",
        title_thanks=thanks,
        author="opticsWolf",
        bib_name="refs-paper",
        bib_keys=bib_keys(ref_bib),
    )


def test_counts(paper_texts):
    md, ref_bib, *_ = paper_texts
    r = build_document(md, "", _spec(_load_legacy(), ref_bib))
    assert (r.n_diagrams, r.n_tables) == (4, 9)


def test_byte_identical_tex(paper_texts):
    md, ref_bib, expected_tex, _, toml_src = paper_texts
    r = build_document(md, toml_src, _spec(_load_legacy(), ref_bib))
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
