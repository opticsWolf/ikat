"""Transcribe the matplotlib data sources into ikat preset fixtures.

Reads `benchmarks/results/tpcbih_style_v2_{1,3}.json` (the same files
`benchmarks/plot_paper_figs.py` loads) plus that script's labels, and
writes the four `fig-*.json` fixtures. Mechanical transcription —
review the git diff, not the numbers: every value must trace to the
results files or to an explicit, recorded decision below.

Honest deltas (also recorded in ACCEPTED.md):
- Fig A (`fork_audit`) has no min/max in the results (medians only),
  so mins == maxs == values: degenerate whiskers, not measured spread.
- Line-plot errors are symmetric in ikat (`+- (0,e)`): `e` is
  max(lo,hi), the conservative cover of the observed spread.
- `\n` in matplotlib tick labels becomes a space (pgfplots has no
  multiline ticks in this emitter).
- Fig D's x axis was log-scale in matplotlib; ikat lineplot is
  linear. The shape story (flat across depth) survives; the spacing
  does not. Recorded, not hidden.
"""

import json
import sys
from pathlib import Path

HERE = Path(__file__).parent
RES = HERE.parent.parent.parent / "benchmarks" / "results"

FOOT = ("PRELIMINARY — non-reference hardware; medians of 5, bars show min/max. "
        "First readings, not reference figures (paper §5).")


def load(scale):
    with open(RES / f"tpcbih_style_v2_{scale}.json") as f:
        return json.load(f)["figures"]


def med(d, k):
    return d[k]["median_ms"]


def mm(d, k):
    """(median, min, max); missing spread degenerates to the median."""
    r = d[k]
    m = r["median_ms"]
    return (m, r.get("min_ms", m), r.get("max_ms", m))


def sym(lo, hi):
    return max(lo, hi)


def main():
    d1, d3 = load(1), load(3)
    figs = {}

    v1, _, _ = mm(d1, "fork_audit")
    v3, _, _ = mm(d3, "fork_audit")
    figs["fig-fork-flat"] = {
        "kind": "bar",
        "title": "Fork is flat across $3\\times$ data (O(1), one register row)",
        "ylabel": "fork latency (ms)",
        "log_y": False,
        "group_labels": ["scale 1 (640c/1520e)", "scale 3 (1920c/4560e)"],
        "series_names": ["fork"],
        "values": [[v1, v3]], "mins": [[v1, v3]], "maxs": [[v1, v3]],
        "footnote": FOOT,
    }

    groups, series, vals, mins, maxs = [], ["scale 1", "scale 3"], [], [], []
    for g, key in [("T6a valid-slice at txn point", "T6a_validslice_txnpoint"),
                   ("T6b txn-versions at valid point", "T6b_txnversions_validpoint")]:
        groups.append(g)
    for d in (d1, d3):
        v, lo, hi = [], [], []
        for key in ("T6a_validslice_txnpoint", "T6b_txnversions_validpoint"):
            m, a, b = mm(d, key)
            v.append(m)
            lo.append(a)
            hi.append(b)
        vals.append(v)
        mins.append(lo)
        maxs.append(hi)
    figs["fig-t6-asymmetry"] = {
        "kind": "bar",
        "title": "T6: transaction-dimension work costs an order more",
        "ylabel": "latency (ms, log)",
        "log_y": True,
        "group_labels": groups,
        "series_names": series,
        "values": vals, "mins": mins, "maxs": maxs,
        "footnote": FOOT,
    }

    pairs = [("R1", "R1_statechange_main", "R1_statechange_audit"),
             ("R3", "R3_agg_main", "R3_agg_audit"),
             ("Q2", "Q2_reconstruct_on_main", "Q2_reconstruct_on_audit"),
             ("Q3", "Q3_traverse_main", "Q3_traverse_audit")]
    tm, bm, te_lo, te_hi, be_lo, be_hi = [], [], [], [], [], []
    for _, mk, bk in pairs:
        m, a, b = mm(d3, mk)
        tm.append(m)
        te_lo.append(a)
        te_hi.append(b)
        m, a, b = mm(d3, bk)
        bm.append(m)
        be_lo.append(a)
        be_hi.append(b)
    z = med(d3, "Q3_traverse_audit0_nowrites")
    figs["fig-trunk-branch"] = {
        "kind": "bar",
        "title": "Branch cost is ancestry, not divergence",
        "ylabel": "latency (ms, log, scale 3)",
        "log_y": True,
        "group_labels": [p[0] for p in pairs],
        "series_names": ["trunk", "branch (audit)"],
        "values": [tm, bm], "mins": [te_lo, be_lo], "maxs": [te_hi, be_hi],
        "refline": [3.8, 4.2, z, "zero-write fork"],
        "footnote": FOOT,
    }

    depths = [1, 2, 5, 10]
    yss, errs = [], []
    for d in (d1, d3):
        ys, es = [], []
        for depth in depths:
            m, a, b = mm(d, f"chain_traverse_d{depth}")
            ys.append(m)
            es.append(sym(m - a, b - m))
        yss.append(ys)
        errs.append(es)
    figs["fig-chain-depth"] = {
        "kind": "line",
        "title": "Branched reads hold flat across chain depth",
        "xlabel": "fork-chain depth",
        "ylabel": "branched traversal (ms)",
        "xs": depths,
        "series_names": ["scale 1", "scale 3"],
        "yss": yss, "errs": errs,
        "footnote": FOOT,
    }

    for name, fig in figs.items():
        (HERE / f"{name}.json").write_text(json.dumps(fig, indent=2) + "\n", encoding="utf-8")
        print("wrote", name)


if __name__ == "__main__":
    sys.exit(main())
