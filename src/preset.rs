//! Benchmark-JSON preset loader: JSON in, `tikzpicture` out.
//!
//! The schema wraps what `examples/ikat-paper/build.py` already
//! assembles by hand (series names, values, min/max whiskers,
//! optional refline, footnote) — no new schema invented, so the
//! showcase figures must round-trip byte-identical through here.
//! Rust never touches the filesystem: the Python wrapper
//! (`python/ikat/preset.py`) reads files and names them in errors.

use serde::Deserialize;

use crate::plot::{self, LegendPos};

/// One preset document. Kind-specific fields are all optional at
/// the type level so a missing one yields a serde error naming the
/// key; semantic checks (rectangular, ordered, non-empty) run in
/// `validate` with file-agnostic messages (the caller prepends the
/// origin — Rust never sees paths).
#[derive(Deserialize)]
struct Preset {
    kind: String,
    title: String,
    ylabel: String,
    #[serde(default)]
    legend: String,
    #[serde(default)]
    footnote: Option<String>,
    // bar
    #[serde(default)]
    log_y: bool,
    #[serde(default)]
    group_labels: Vec<String>,
    #[serde(default)]
    series_names: Vec<String>,
    #[serde(default)]
    values: Vec<Vec<f64>>,
    #[serde(default)]
    mins: Vec<Vec<f64>>,
    #[serde(default)]
    maxs: Vec<Vec<f64>>,
    #[serde(default)]
    refline: Option<(f64, f64, f64, String)>,
    // line
    #[serde(default)]
    xlabel: String,
    #[serde(default)]
    xs: Vec<f64>,
    #[serde(default)]
    yss: Vec<Vec<f64>>,
    #[serde(default)]
    errs: Vec<Vec<f64>>,
}

fn esc_prose(s: &str) -> String {
    s.replace('&', "\\&")
        .replace('%', "\\%")
        .replace('#', "\\#")
        .replace('_', "\\_")
}

fn with_footnote(mut tikz: String, footnote: &Option<String>) -> String {
    if let Some(f) = footnote {
        tikz.push_str(&format!("{{\\footnotesize {}\\par}}\n", esc_prose(f)));
    }
    tikz
}

/// Parse preset JSON and emit the `tikzpicture`. Errors name the
/// offending key/value, never a bare serde dump.
pub fn preset_to_tikz(src: &str) -> Result<String, String> {
    let p: Preset = serde_json::from_str(src).map_err(|e| format!("preset: {e}"))?;
    let legend_src = if p.legend.is_empty() { "auto" } else { &p.legend };
    let legend = LegendPos::parse(legend_src).map_err(|e| format!("preset legend: {e}"))?;
    match p.kind.as_str() {
        "bar" => {
            if p.group_labels.is_empty() || p.series_names.is_empty() {
                return Err("preset bar: no group_labels or no series_names".to_string());
            }
            let (ng, ns) = (p.group_labels.len(), p.series_names.len());
            for (tag, t) in [("values", &p.values), ("mins", &p.mins), ("maxs", &p.maxs)] {
                if t.len() != ns || t.iter().any(|col| col.len() != ng) {
                    return Err(format!("preset bar: ragged {tag}"));
                }
            }
            for s in 0..ns {
                for i in 0..ng {
                    if !(p.mins[s][i] <= p.values[s][i] && p.values[s][i] <= p.maxs[s][i]) {
                        return Err(format!(
                            "preset bar: series {} group {i}: min > value or value > max",
                            p.series_names[s]
                        ));
                    }
                }
            }
            let tikz = plot::barchart(
                &p.title,
                &p.ylabel,
                p.log_y,
                &p.group_labels,
                &p.series_names,
                &p.values,
                &p.mins,
                &p.maxs,
                p.refline,
                legend,
            )?;
            Ok(with_footnote(tikz, &p.footnote))
        }
        "line" => {
            if p.xs.is_empty() || p.series_names.is_empty() {
                return Err("preset line: no xs or no series_names".to_string());
            }
            if p.series_names.len() != p.yss.len() || p.series_names.len() != p.errs.len() {
                return Err("preset line: series_names/yss/errs length mismatch".to_string());
            }
            for w in 1..p.xs.len() {
                if !(p.xs[w] > p.xs[w - 1]) {
                    return Err("preset line: xs not strictly increasing".to_string());
                }
            }
            let series: Vec<(&str, Vec<f64>, Vec<f64>)> = p
                .series_names
                .iter()
                .zip(p.yss.iter())
                .zip(p.errs.iter())
                .map(|((n, y), e)| (n.as_str(), y.clone(), e.clone()))
                .collect();
            for (name, ys, es) in &series {
                if ys.len() != p.xs.len() || es.len() != p.xs.len() {
                    return Err(format!("preset line: ragged series {name}"));
                }
            }
            let tikz = plot::lineplot(&p.title, &p.xlabel, &p.ylabel, &p.xs, &series, legend)?;
            Ok(with_footnote(tikz, &p.footnote))
        }
        other => Err(format!("preset: kind must be bar|line, got {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bar_src() -> String {
        serde_json::json!({
            "kind": "bar", "title": "t", "ylabel": "y",
            "group_labels": ["a", "b"], "series_names": ["s"],
            "values": [[1.0, 2.0]], "mins": [[0.5, 1.5]], "maxs": [[1.5, 2.5]],
        })
        .to_string()
    }

    #[test]
    fn bar_valid_golden() {
        let got = preset_to_tikz(&bar_src()).unwrap();
        assert!(got.contains("\\begin{axis}"));
        assert!(got.contains("(1,1 )"));
        assert!(got.contains("\\addlegendentry{s}"));
    }

    #[test]
    fn line_valid_golden() {
        let src = serde_json::json!({
            "kind": "line", "title": "t", "xlabel": "x", "ylabel": "y",
            "xs": [1.0, 2.0], "series_names": ["s"],
            "yss": [[3.0, 4.0]], "errs": [[0.1, 0.2]],
        })
        .to_string();
        let got = preset_to_tikz(&src).unwrap();
        assert!(got.contains("(2,4) +- (0,0.2)"));
    }

    #[test]
    fn bar_ragged_fails() {
        let mut v: serde_json::Value = serde_json::from_str(&bar_src()).unwrap();
        v["values"] = serde_json::json!([[1.0]]);
        let err = preset_to_tikz(&v.to_string()).unwrap_err();
        assert!(err.contains("ragged values"), "{err}");
    }

    #[test]
    fn bar_min_above_value_fails() {
        let mut v: serde_json::Value = serde_json::from_str(&bar_src()).unwrap();
        v["mins"] = serde_json::json!([[5.0, 1.5]]);
        let err = preset_to_tikz(&v.to_string()).unwrap_err();
        assert!(err.contains("min > value"), "{err}");
    }

    #[test]
    fn missing_key_names_it() {
        let mut v: serde_json::Value = serde_json::from_str(&bar_src()).unwrap();
        v.as_object_mut().unwrap().remove("title");
        let err = preset_to_tikz(&v.to_string()).unwrap_err();
        assert!(err.contains("title"), "{err}");
    }

    #[test]
    fn bad_kind_names_set() {
        let mut v: serde_json::Value = serde_json::from_str(&bar_src()).unwrap();
        v["kind"] = serde_json::json!("pie");
        let err = preset_to_tikz(&v.to_string()).unwrap_err();
        assert!(err.contains("bar|line"), "{err}");
    }

    #[test]
    fn empty_names_fail() {
        let mut v: serde_json::Value = serde_json::from_str(&bar_src()).unwrap();
        v["series_names"] = serde_json::json!([]);
        let err = preset_to_tikz(&v.to_string()).unwrap_err();
        assert!(err.contains("no group_labels or no series_names"), "{err}");
    }

    #[test]
    fn footnote_appends_group() {
        let mut v: serde_json::Value = serde_json::from_str(&bar_src()).unwrap();
        v["footnote"] = serde_json::json!("source: build.py & grep");
        let got = preset_to_tikz(&v.to_string()).unwrap();
        assert!(got.contains("{\\footnotesize source: build.py \\& grep\\par}"));
    }
}
