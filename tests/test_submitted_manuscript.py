"""Submitted-manuscript regressions (0.13.6).

Every test here comes from a real manuscript that failed to weave or
to compile in LaTeX, not from a synthetic idea:

* ``Navette_GA/literature/PAPER_ga_thinfilm_strategies_v3.md`` — a
  26-row ledger table in an IEEEtran conference column (``Float too
  large for page by 520pt``), superscript prose (``10⁻⁵``,
  ``X⁽¹⁾``, ``[0,1)ⁿ``), ``∂``, and combining accents inside display
  math (``n̂``, ``ẑ``, ``ñ``).
"""

from __future__ import annotations

from ikat import build_document, spec_from_dict


def _spec(**kw):
    base = {"diagrams": [], "table_captions": []}
    base.update(kw)
    return spec_from_dict(base)


LEDGER_ROWS = "\n".join(
    f"| V{i} | TMM is fast and accurate in loop, which is why growth and selection both run on the exact model and every candidate is evaluated | Confirmed | D16 chunk {i} |"
    for i in range(16, 39)
)


def test_tall_ledger_table_is_placed_so_it_fits_the_page():
    """A 23-row prose table must not become a 520pt-too-tall float."""
    md = f"""# T

## 17. Verification ledger

| # | Claim | Verdict | Evidence |
|---|---|---|---|
{LEDGER_ROWS}
"""
    r = build_document(md, "", _spec(table_captions=["Ledger."]))
    assert any("table" in n for n in r.notes), r.notes
    # Either it is promoted to a full-width float, or set smaller —
    # never left at full size in the column.
    assert "\\begin{table*}" in r.tex or "\\footnotesize" in r.tex or "\\scriptsize" in r.tex


def test_small_table_stays_byte_identical():
    """Auto-adapt must not touch a table that already fits."""
    md = """# T

## 1. A

| Refiner | When | Witness |
|---|---|---|
| Nelder--Mead | Polish | **E00** |
| LM | Least squares | **C00** |
"""
    r = build_document(md, "", _spec(table_captions=["C."]))
    assert "\\begin{table}[t]" in r.tex
    assert "\\small" in r.tex
    assert "\\scriptsize" not in r.tex and "footnotesize" not in r.tex
    assert not [n for n in r.notes if n.startswith("table")], r.notes


def test_superscript_prose_and_partial_symbols_weave():
    """`10⁻⁵`, `X⁽¹⁾`, `[0,1)ⁿ`, `∂F/∂A` are prose, not math."""
    md = """# T

## 1. A

Stop at 3x10\u207b\u2075 relative decrease, 100+ starts, the best refined
to 10\u207b\u2074. The cycle writes X\u207d\u00b9\u207e from the operator, encoder
J:[0,1)\u207f to S, and the adjoint solves for \u2202F/\u2202A.
"""
    r = build_document(md, "", _spec())
    assert "$^{-5}$" in r.tex
    assert "$^{n}$" in r.tex
    assert "$\\partial$" in r.tex
    assert any("unicode fallbacks" in n for n in r.notes), r.notes


def test_combining_accents_fold_inside_display_math():
    """`n̂` inside `$$…$$` is `\\hat{n}`, not a LaTeX Unicode error."""
    md = """# T

## 10.1 Needle variation

$$n\u0303(z) = n(z) \\text{ off } [\u1e91, \u1e91+\\Delta z], \\quad n\u0302 \\text{ on it}\\tag{1}$$
"""
    r = build_document(md, "", _spec())
    assert "\\hat{z}" in r.tex, r.tex
    assert "\\tilde{n}" in r.tex, r.tex
    assert "\\hat{n}" in r.tex, r.tex


def test_combining_accent_without_a_base_still_fails_with_a_hint():
    """Ambiguity still fails: no atom to accent, no silent transform."""
    md = "# T\n\n## 1. A\n\n$$ \u0302 + x $$\n"
    try:
        build_document(md, "", _spec())
    except Exception as e:  # noqa: BLE001 - the message is the contract
        assert "\\hat" in str(e) or "accent" in str(e)
    else:  # pragma: no cover - folded means the gate is permissive
        raise AssertionError("expected a math-accent diagnostic")
