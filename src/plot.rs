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
    out.push_str("  xtick=data,\n  xticklabels={");
    out.push_str(
        &group_labels
            .iter()
            .map(|l| format!("{{{}}}", esc_tick(l)))
            .collect::<Vec<_>>()
            .join(","),
    );
    out.push_str("},\n  legend style={at={(0.02,0.98)},anchor=north west,font=\\footnotesize},\n");
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
    out.push_str("  legend style={at={(0.02,0.98)},anchor=north west,font=\\footnotesize},\n");
    out.push_str("  error bars/y dir=both, error bars/y explicit,\n]\n");
    for (name, ys, es) in series {
        out.push_str("  \\addplot+[error bars/.cd,y explicit] coordinates {\n");
        for ((x, y), e) in xs.iter().zip(ys.iter()).zip(es.iter()) {
            out.push_str(&format!(
                "    ({},{}) +- (,{})\n",
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
        let tikz = barchart("t", "latency (ms, log, scale 3)", true, &gl, &sn, &v, &lo, &hi, None).unwrap();
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
        )
        .unwrap();
        assert!(tikz.contains("\\draw[dashed]"));
        assert!(barchart("t", "y", false, &[], &sn, &v, &lo, &hi, None).is_err());
        assert!(barchart("t", "y", false, &gl, &sn, &[vec![1.0]], &lo, &hi, None).is_err());
        assert!(lineplot("t", "x", "y", &[], &[]).is_err());
    }

    #[test]
    fn line_emits_series() {
        let tikz = lineplot(
            "t",
            "fork-chain depth",
            "branched traversal (ms)",
            &[1.0, 2.0, 5.0, 10.0],
            &[("scale 1", vec![3.0, 3.1, 5.5, 4.2], vec![0.5, 0.6, 1.0, 0.8])],
        )
        .unwrap();
        assert!(tikz.contains("\\addlegendentry{scale 1}"));
        assert!(tikz.contains("(10,4.2)"));
    }
}
