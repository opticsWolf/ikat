//! Data → pgfplots `tikzpicture`.
//!
//! Strategy note (see `docs/RESEARCH.md`): this module emits pgfplots
//! code directly instead of depending on the `pgfplots` crate or on
//! matplotlib. Rationale:
//!
//! - the paper range needed here is tiny (bar+whiskers, log-y, line
//!   plots with error bands, axis labels, source footnotes);
//! - direct emission gives byte-level control of IEEE-friendly output
//!   (no `~` in tick labels, `$\sim$` for approximations, exact
//!   `footnotesize` footnote style the paper already uses);
//! - figures compile in the document's own TeX run, so fonts always
//!   match — the whole point of dropping matplotlib.
//!
//! Numeric formatting mirrors the paper's measured-data style:
//! plain decimal coordinates, no scientific notation (pgfplots would
//! reformat anyway on a log axis).

fn fmt_num(x: f64) -> String {
    if x == x.trunc() && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        // Trim float noise: 4 significant decimals, no trailing zeros.
        let s = format!("{x:.4}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn esc_tick(s: &str) -> String {
    s.replace('&', "\\&")
        .replace('%', "\\%")
        .replace('#', "\\#")
        .replace('_', "\\_")
}

/// Legend placement keyword. `auto` (the default) tries the four
/// inside corners in `AUTO_ORDER`, testing each against the drawn
/// data, and falls back to `below` when every corner is occupied. The corners can
/// never touch axis labels (labels live outside the axis box); the
/// `below` row clears tick labels by construction (`at 0.5,-0.18`).
/// Anything outside the seven words is an error — a misspelled
/// position must fail here, not as a silent default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegendPos {
    Auto,
    Below,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    OutsideRight,
}

/// Corner trial order for `Auto`: top row first (the classic spot
/// above the data), left before right, then the bottom row.
const AUTO_ORDER: [LegendPos; 4] = [
    LegendPos::TopLeft,
    LegendPos::TopRight,
    LegendPos::BottomRight,
    LegendPos::BottomLeft,
];

/// Something drawn that a legend must not cover: a line segment or
/// a filled rect, in axis data coordinates (log-mapped for log-y
/// bar charts, so the test matches what the eye sees).
#[derive(Debug, Clone, Copy)]
enum Obstacle {
    Seg(f64, f64, f64, f64),
    Bar((f64, f64, f64, f64)),
}

// Segment geometry lives in `layout.rs` now (moved, not rewritten).
use crate::layout::{rects_overlap, seg_hits_rect};

/// Candidate legend box for a corner, on the *visual* axis ranges
/// (see callers). Deliberately oversized — 30% of the x-range by
/// 25% of the y-range — so a "free" verdict really means empty.
/// Degenerate ranges (single point) yield `None`: no inside corner
/// is testable, fall back below.
fn corner_box(
    corner: LegendPos,
    vx0: f64,
    vx1: f64,
    vy0: f64,
    vy1: f64,
) -> Option<(f64, f64, f64, f64)> {
    let (dx, dy) = (vx1 - vx0, vy1 - vy0);
    if !(dx > 0.0) || !(dy > 0.0) {
        return None;
    }
    let (w, h) = (0.30 * dx, 0.25 * dy);
    Some(match corner {
        LegendPos::TopLeft => (vx0, vy1 - h, vx0 + w, vy1),
        LegendPos::TopRight => (vx1 - w, vy1 - h, vx1, vy1),
        LegendPos::BottomLeft => (vx0, vy0, vx0 + w, vy0 + h),
        LegendPos::BottomRight => (vx1 - w, vy0, vx1, vy0 + h),
        _ => return None,
    })
}

/// First free corner in `AUTO_ORDER`, else `Below`. `vx`/`vy` are
/// the visual axis ranges (data ranges plus the padding pgfplots
/// adds — overestimated on purpose, so verdicts stay conservative).
fn pick_auto(
    vx0: f64,
    vx1: f64,
    vy0: f64,
    vy1: f64,
    obstacles: &[Obstacle],
) -> LegendPos {
    for corner in AUTO_ORDER {
        let Some(b) = corner_box(corner, vx0, vx1, vy0, vy1) else {
            return LegendPos::Below;
        };
        let hit = obstacles.iter().any(|o| match *o {
            Obstacle::Seg(x1, y1, x2, y2) => seg_hits_rect(x1, y1, x2, y2, b),
            Obstacle::Bar(r) => rects_overlap(r, b),
        });
        if !hit {
            return corner;
        }
    }
    LegendPos::Below
}

impl LegendPos {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "auto" => Ok(Self::Auto),
            "below" => Ok(Self::Below),
            "top-left" => Ok(Self::TopLeft),
            "top-right" => Ok(Self::TopRight),
            "bottom-left" => Ok(Self::BottomLeft),
            "bottom-right" => Ok(Self::BottomRight),
            "outside-right" => Ok(Self::OutsideRight),
            _ => Err(format!(
                "legend must be auto|below|top-left|top-right|bottom-left|bottom-right|outside-right, got {s:?}"
            )),
        }
    }

    /// The pgfplots legend line for this position. `Auto` has no
    /// line of its own — resolve it against the data first (the
    /// emitters do this via `pick_auto`).
    pub fn latex(&self) -> &'static str {
        match self {
            Self::Auto => unreachable!("LegendPos::Auto must be resolved with data before latex()"),
            Self::Below => "legend style={at={(0.5,-0.18)},anchor=north,legend columns=-1,font=\\footnotesize},",
            Self::TopLeft => "legend style={at={(0.02,0.98)},anchor=north west,font=\\footnotesize},",
            Self::TopRight => "legend style={at={(0.98,0.98)},anchor=north east,font=\\footnotesize},",
            Self::BottomLeft => "legend style={at={(0.02,0.02)},anchor=south west,font=\\footnotesize},",
            Self::BottomRight => "legend style={at={(0.98,0.02)},anchor=south east,font=\\footnotesize},",
            Self::OutsideRight => "legend pos=outer north east,",
        }
    }
}

/// Bar chart with min/max whiskers, matching the paper's trunk-vs-branch
/// figure: series drawn side by side, dashed reference line optional.
///
/// `refline` is `(x0, x1, y, label)` in axis coordinates.
#[allow(clippy::too_many_arguments)]
pub fn barchart(
    title: &str,
    ylabel: &str,
    log_y: bool,
    group_labels: &[String],
    series_names: &[String],
    values: &[Vec<f64>],
    mins: &[Vec<f64>],
    maxs: &[Vec<f64>],
    refline: Option<(f64, f64, f64, String)>,
    legend: LegendPos,
) -> Result<String, String> {
    if group_labels.is_empty() || series_names.is_empty() {
        return Err("barchart: no groups or no series".to_string());
    }
    let (n_groups, n_series) = (group_labels.len(), series_names.len());
    for (tag, t) in [("values", values), ("mins", mins), ("maxs", maxs)] {
        if t.len() != n_series || t.iter().any(|col| col.len() != n_groups) {
            return Err(format!("barchart: ragged {tag}"));
        }
    }

    let mut out = String::new();
    out.push_str("\\begin{tikzpicture}\n\\begin{axis}[\n");
    out.push_str(&format!("  title={{{title}}},\n"));
    out.push_str(&format!("  ylabel={{{ylabel}}},\n"));
    out.push_str("  ybar, bar width=0.32,\n  enlarge x limits=0.25,\n");
    if log_y {
        out.push_str("  ymode=log, log origin=infty,\n");
    }
    // Collision model for `Auto`: bars fill the whole slot from the
    // axis floor to the whisker top, in log-mapped space for log-y
    // (that is what the eye sees). Visual ranges overestimate
    // pgfplots' padding on purpose, so verdicts stay conservative.
    let ty = |y: f64| {
        if log_y {
            if y > 0.0 {
                y.log10()
            } else {
                f64::NEG_INFINITY
            }
        } else {
            y
        }
    };
    let (vx0, vx1) = (
        1.0 - 0.25 * (n_groups as f64 - 1.0),
        n_groups as f64 + 0.25 * (n_groups as f64 - 1.0),
    );
    let (mut lymin, mut lymax) = (f64::INFINITY, f64::NEG_INFINITY);
    for col in maxs.iter().chain(mins.iter()) {
        for v in col {
            let t = ty(*v);
            if t.is_finite() {
                lymin = lymin.min(t);
                lymax = lymax.max(t);
            }
        }
    }
    if let Some((_, _, y, _)) = &refline {
        let t = ty(*y);
        if t.is_finite() {
            lymin = lymin.min(t);
            lymax = lymax.max(t);
        }
    }
    if !lymin.is_finite() {
        // No positive data on a log axis (or empty): nothing testable.
        lymin = 0.0;
        lymax = 1.0;
    }
    if !log_y {
        lymin = lymin.min(0.0); // bars start at the axis floor
    }
    let ldy = (lymax - lymin).max(1e-9);
    let (vy0, vy1) = (lymin - 0.15 * ldy, lymax + 0.15 * ldy);
    let mut obstacles: Vec<Obstacle> = Vec::with_capacity(n_groups * n_series + 1);
    for s in 0..n_series {
        for i in 0..n_groups {
            let top = ty(maxs[s][i]);
            if top.is_finite() {
                obstacles.push(Obstacle::Bar((i as f64 + 0.5, vy0, i as f64 + 1.5, top)));
            }
        }
    }
    if let Some((x0, x1, y, _)) = &refline {
        let t = ty(*y);
        if t.is_finite() {
            obstacles.push(Obstacle::Seg(*x0, t, *x1, t));
        }
    }
    let legend = match legend {
        LegendPos::Auto => pick_auto(vx0, vx1, vy0, vy1, &obstacles),
        fixed => fixed,
    };
    out.push_str("  xtick=data,\n  x tick label style={rotate=45,anchor=east},\n  xticklabels={");
    out.push_str(
        &group_labels
            .iter()
            .map(|l| format!("{{{}}}", esc_tick(l)))
            .collect::<Vec<_>>()
            .join(","),
    );
    out.push_str("},\n  ");
    out.push_str(legend.latex());
    out.push_str("\n");
    out.push_str("  error bars/y dir=both, error bars/y explicit,\n]\n");

    for (s, name) in series_names.iter().enumerate() {
        out.push_str("  \\addplot+[error bars/.cd,y explicit] coordinates {\n");
        for i in 0..n_groups {
            let v = values[s][i];
            let lo = v - mins[s][i];
            let hi = maxs[s][i] - v;
            out.push_str(&format!(
                "    ({},{} ) +- ({},{} )\n",
                i + 1,
                fmt_num(v),
                fmt_num(lo.max(0.0)),
                fmt_num(hi.max(0.0))
            ));
        }
        out.push_str(&format!("  }};\n  \\addlegendentry{{{name}}}\n"));
    }
    // Legend entries are supplied by the caller layer (series names live
    // in Python, where the benchmark harness already names them).
    if let Some((x0, x1, y, label)) = refline {
        out.push_str(&format!(
            "  \\draw[dashed] (axis cs:{},{}) -- (axis cs:{},{}) node[above,font=\\footnotesize] {{{}}};\n",
            fmt_num(x0),
            fmt_num(y),
            fmt_num(x1),
            fmt_num(y),
            esc_tick(&label)
        ));
    }
    out.push_str("\\end{axis}\n\\end{tikzpicture}\n");
    Ok(out)
}

/// Line plot with symmetric vertical error bars (chain-depth figure).
pub fn lineplot(
    title: &str,
    xlabel: &str,
    ylabel: &str,
    xs: &[f64],
    series: &[(&str, Vec<f64>, Vec<f64>)],
    legend: LegendPos,
) -> Result<String, String> {
    if xs.is_empty() || series.is_empty() {
        return Err("lineplot: empty data".to_string());
    }
    for (_, ys, es) in series {
        if ys.len() != xs.len() || es.len() != xs.len() {
            return Err("lineplot: ragged series".to_string());
        }
    }
    let mut out = String::new();
    out.push_str("\\begin{tikzpicture}\n\\begin{axis}[\n");
    out.push_str(&format!("  title={{{title}}},\n  xlabel={{{xlabel}}},\n  ylabel={{{ylabel}}},\n"));
    let (mut xmin, mut xmax) = (xs[0], xs[0]);
    for x in xs {
        xmin = xmin.min(*x);
        xmax = xmax.max(*x);
    }
    // Pin the axis to the data: pgfplots otherwise starts at zero,
    // leaving a meaningless gap before the first commit (or sample).
    out.push_str(&format!("  xmin={}, xmax={},\n", xmin.floor(), xmax.ceil()));
    // Collision model for `Auto`: polyline segments plus the
    // vertical error-bar whiskers. x limits are explicit (exact);
    // y padding overestimates pgfplots' default, conservatively.
    let (vx0, vx1) = (xmin.floor(), xmax.ceil());
    let (mut dymin, mut dymax) = (f64::INFINITY, f64::NEG_INFINITY);
    for (_, ys, es) in series {
        for (y, e) in ys.iter().zip(es.iter()) {
            dymin = dymin.min(y - e);
            dymax = dymax.max(y + e);
        }
    }
    let ddy = (dymax - dymin).max(1e-9);
    let (vy0, vy1) = (dymin - 0.15 * ddy, dymax + 0.15 * ddy);
    let mut obstacles: Vec<Obstacle> = Vec::new();
    for (_, ys, es) in series {
        for w in 0..xs.len() {
            obstacles.push(Obstacle::Seg(xs[w], ys[w] - es[w], xs[w], ys[w] + es[w]));
            if w + 1 < xs.len() {
                obstacles.push(Obstacle::Seg(xs[w], ys[w], xs[w + 1], ys[w + 1]));
            }
        }
    }
    let legend = match legend {
        LegendPos::Auto => pick_auto(vx0, vx1, vy0, vy1, &obstacles),
        fixed => fixed,
    };
    out.push_str("  ");
    out.push_str(legend.latex());
    out.push_str("\n");
    out.push_str("  error bars/y dir=both, error bars/y explicit,\n]\n");
    for (name, ys, es) in series {
        out.push_str("  \\addplot+[error bars/.cd,y explicit] coordinates {\n");
        for ((x, y), e) in xs.iter().zip(ys.iter()).zip(es.iter()) {
            out.push_str(&format!(
                "    ({},{}) +- (0,{})\n",
                fmt_num(*x),
                fmt_num(*y),
                fmt_num(e.max(0.0))
            ));
        }
        out.push_str(&format!("  }};\n  \\addlegendentry{{{name}}}\n"));
    }
    out.push_str("\\end{axis}\n\\end{tikzpicture}\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legend_auto_picks_free_corner() {
        // Rising line: top-right is crossed by the last segment,
        // top-left is empty -> first in AUTO_ORDER that is free.
        let got = lineplot(
            "t", "x", "y",
            &[1.0, 2.0, 3.0, 4.0],
            &[("s", vec![1.0, 2.0, 3.0, 4.0], vec![0.0, 0.0, 0.0, 0.0])],
            LegendPos::Auto,
        )
        .unwrap();
        assert!(got.contains("anchor=north west"), "expected top-left");
    }

    #[test]
    fn legend_auto_falls_below_when_full() {
        // Rising + falling series occupy all four corners.
        let got = lineplot(
            "t", "x", "y",
            &[1.0, 2.0, 3.0, 4.0],
            &[
                ("up", vec![1.0, 2.0, 3.0, 4.0], vec![0.0, 0.0, 0.0, 0.0]),
                ("dn", vec![4.0, 3.0, 2.0, 1.0], vec![0.0, 0.0, 0.0, 0.0]),
            ],
            LegendPos::Auto,
        )
        .unwrap();
        assert!(got.contains("anchor=north,legend columns=-1"), "expected below");
    }

    #[test]
    fn legend_auto_bar_skips_tall_group() {
        // Tall first bar kills top-left; short rest leave top-right.
        let gl = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
        let sn = vec!["s".to_string()];
        let v = vec![vec![100.0, 10.0, 10.0, 10.0]];
        let got = barchart("t", "y", false, &gl, &sn, &v, &v, &v, None, LegendPos::Auto).unwrap();
        assert!(got.contains("anchor=north east"), "expected top-right");
    }

    #[test]
    fn legend_auto_degenerate_point_falls_below() {
        let got = lineplot("t", "x", "y", &[1.0], &[("s", vec![2.0], vec![0.0])], LegendPos::Auto).unwrap();
        assert!(got.contains("anchor=north,legend columns=-1"), "expected below");
    }

    #[test]
    fn legend_auto_parses_and_errors_name_set() {
        assert_eq!(LegendPos::parse("auto").unwrap(), LegendPos::Auto);
        let err = LegendPos::parse("center").unwrap_err();
        assert!(err.contains("auto|below|top-left"), "{err}");
    }

    #[test]
    fn legend_keywords_all_positions() {
        for (word, want) in [
            ("below", "anchor=north,legend columns=-1"),
            ("top-left", "anchor=north west"),
            ("top-right", "anchor=north east"),
            ("bottom-left", "anchor=south west"),
            ("bottom-right", "anchor=south east"),
            ("outside-right", "legend pos=outer north east"),
        ] {
            let pos = LegendPos::parse(word).unwrap();
            let got = lineplot("t", "x", "y", &[1.0], &[("s", vec![2.0], vec![0.0])], pos).unwrap();
            assert!(got.contains(want), "{word}");
        }
        assert!(LegendPos::parse("center").is_err());
        assert!(LegendPos::parse("").is_err());
    }

    #[test]
    fn legend_sits_below_data() {
        // Legends live below the axis (never over data): anchored
        // north at y<0 with one horizontal row.
        let got = lineplot("t", "x", "y", &[1.0], &[("s", vec![2.0], vec![0.0])], LegendPos::Below).unwrap();
        assert!(got.contains("at={(0.5,-0.18)},anchor=north,legend columns=-1"));
        assert!(!got.contains("anchor=north west"));
    }

    #[test]
    fn lineplot_error_bars_parse() {
        let got = lineplot("t", "x", "y", &[1.0], &[("s", vec![2.0], vec![0.0])], LegendPos::Below).unwrap();
        assert!(got.contains("(1,2) +- (0,0)"));
        assert!(!got.contains("+- (,"));
    }

    fn groups() -> (Vec<String>, Vec<String>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>) {
        (
            vec!["R1".to_string(), "Q3".to_string()],
            vec!["trunk".to_string(), "branch (audit)".to_string()],
            vec![vec![7.4, 0.47], vec![28.0, 66.0]],
            vec![vec![6.9, 0.46], vec![25.0, 60.0]],
            vec![vec![8.1, 0.48], vec![31.0, 72.0]],
        )
    }

    #[test]
    fn bar_emits_axis_and_whiskers() {
        let (gl, sn, v, lo, hi) = groups();
        let tikz = barchart("t", "latency (ms, log, scale 3)", true, &gl, &sn, &v, &lo, &hi, None, LegendPos::Below).unwrap();
        assert!(tikz.contains("\\begin{axis}"));
        assert!(tikz.contains("ymode=log"));
        assert!(tikz.contains("(1,7.4"));
        assert!(tikz.contains("xticklabels={{R1},{Q3}}"));
        assert!(tikz.contains("\\addlegendentry{branch (audit)}"));
    }

    #[test]
    fn bar_refline_and_errors() {
        let (gl, sn, v, lo, hi) = groups();
        let tikz = barchart(
            "t",
            "y",
            false,
            &gl,
            &sn,
            &v,
            &lo,
            &hi,
            Some((2.8, 3.2, 66.0, "zero-write fork".to_string())),
            LegendPos::TopRight,
        )
        .unwrap();
        assert!(tikz.contains("\\draw[dashed]"));
        assert!(tikz.contains("anchor=north east"));
        assert!(barchart("t", "y", false, &[], &sn, &v, &lo, &hi, None, LegendPos::Below).is_err());
        assert!(barchart("t", "y", false, &gl, &sn, &[vec![1.0]], &lo, &hi, None, LegendPos::Below).is_err());
        assert!(lineplot("t", "x", "y", &[], &[], LegendPos::Below).is_err());
    }

    #[test]
    fn line_emits_series() {
        let tikz = lineplot(
            "t",
            "fork-chain depth",
            "branched traversal (ms)",
            &[1.0, 2.0, 5.0, 10.0],
            &[("scale 1", vec![3.0, 3.1, 5.5, 4.2], vec![0.5, 0.6, 1.0, 0.8])],
            LegendPos::Below,
        )
        .unwrap();
        assert!(tikz.contains("\\addlegendentry{scale 1}"));
        assert!(tikz.contains("(10,4.2)"));
    }
}
