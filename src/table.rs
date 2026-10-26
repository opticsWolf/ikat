//! Pipe tables → `tabularx`, and `.bib` utilities.
//! Byte-parity with the paper build's table stage is the contract.

use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

use crate::config::{FloatNeeds, Pos, Span};
use crate::esc::inline;

fn sep_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^:?-{2,}:?$").unwrap())
}

fn field_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^\s*([A-Za-z]+)\s*=").unwrap())
}

fn key_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"@\w+\{([^,\s]+)\s*,").unwrap())
}

pub fn split_row(line: &str) -> Vec<String> {
    line.trim().trim_matches('|').split('|').map(|c| c.trim().to_string()).collect()
}

pub fn is_sep(line: &str) -> bool {
    let cells = split_row(line);
    !cells.is_empty() && cells.iter().all(|c| sep_re().is_match(c))
}

/// Portrait-fit ladder (M7.5): the widest *unbreakable token* in the
/// table decides the level — prose labels wrap in X columns, but
/// numeric cells (`-0.234**(0.096)`) cannot break and print on top
/// of each other when wider than their column share (econ-tax's
/// 13-column tables overlapped at 27pt). Levels, decided by the
/// single point `table_compact` (autolayout shares it for notes):
/// - `Fit`: current output, byte-identical (`\small`, all-`X`).
/// - `Compact`: `\footnotesize` + 3pt `\tabcolsep`.
///   (`Rotate` additionally packs to 2pt padding: at 13 columns
///   the padding itself is the overflow.)
/// - `Rotate`: `sidewaystable(*)` — portrait cannot hold the table
///   at any readable size, so the text height becomes the width.
/// A `width=` fraction scales against `\textheight` under rotation
/// (absolute widths pass through); `pos` becomes `[p]` there — a
/// sideways float takes its own page.
/// The ladder only runs when `compact` (the `[tables]
/// compact_tables` knob) is set — otherwise every table takes the
/// `Fit` path, byte-identical to the pre-ladder output.
/// `\small` digit/paren pitch (Computer Modern digits are 0.5em;
/// 9pt `\small` -> ~4.5pt = 0.16cm). Conservative for prose
/// (lowercase averages less) — the ladder must not miss real
/// numeric overflow, and prose overestimates only compact early.
const TABLE_TOKEN_CM: f64 = 0.16;
const FOOT_FACTOR: f64 = 0.89;
const TABCOLSEP_CM: f64 = 0.21;
const TABCOLSEP_SMALL_CM: f64 = 0.11;

/// Compaction level for a table: column `share` vs the widest
/// unbreakable token. Shared with autolayout's reason notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableLevel {
    Fit,
    Compact,
    Rotate,
}

pub(crate) fn table_compact(span_cm: f64, ncols: usize, worst_cm: f64) -> TableLevel {
    let share = span_cm / ncols.max(1) as f64;
    if worst_cm <= share - 2.0 * TABCOLSEP_CM {
        TableLevel::Fit
    } else if worst_cm * FOOT_FACTOR <= share - 2.0 * TABCOLSEP_SMALL_CM {
        TableLevel::Compact
    } else {
        TableLevel::Rotate
    }
}

/// Widest unbreakable token (whitespace-delimited run) across raw
/// cells, in cm. `label_len` folds `$..$` math and `\X` escapes to
/// one glyph each — the same ruler as the crowding floor.
pub(crate) fn worst_token_cm(rows: &[Vec<String>]) -> f64 {
    rows.iter()
        .flat_map(|r| r.iter())
        .flat_map(|c| c.split_whitespace())
        .map(crate::autolayout::label_len)
        .max()
        .unwrap_or(0) as f64
        * TABLE_TOKEN_CM
}

/// Full `table` float. First row is bolded; `spec` overrides the
/// default all-`X` column string (e.g. the appendix `Xcl`).
/// `line` is the 1-based original line of the first row; rows are
/// consecutive original lines, so cell errors resolve per row.
pub fn table_block(
    rows: &[String],
    cap: &str,
    keys: Option<&HashSet<String>>,
    spec: Option<&str>,
    span: Span,
    pos: Pos,
    caption_top: bool,
    width: Option<&str>,
    col_cm: f64,
    text_cm: f64,
    compact: bool,
    line: usize,
) -> Result<(String, FloatNeeds), crate::error::Error> {
    use crate::error::Error;
    let mut needs = FloatNeeds::none();
    let wide = matches!(span, Span::Wide);
    if wide && matches!(pos, Pos::Here | Pos::Force) {
        return Err(Error::new(
            "table*: pos here/force is illegal on full-width floats (use span=column)",
            line,
            rows.first().cloned().unwrap_or_default(),
        ));
    }
    let n = rows.iter().map(|r| split_row(r).len()).max().unwrap_or(0);
    // Compaction ladder (M7.5): the widest unbreakable token
    // decides; wrappable prose never triggers it.
    let data: Vec<Vec<String>> = rows
        .iter()
        .filter(|r| !is_sep(r))
        .map(|r| split_row(r))
        .collect();
    // Span promotion (M7.5): a column table the portrait column
    // cannot hold is re-decided against the full text width — the
    // engine mirrors this exactly (same inputs), so suggestions and
    // output never disagree. The `[tables]` gate covers promotion
    // too: with the knob off every table stays `Fit` in its span.
    let span_cm = if wide { text_cm } else { col_cm };
    let worst = worst_token_cm(&data);
    let (ewide, level) = if !wide && compact && table_compact(span_cm, n, worst) == TableLevel::Rotate {
        (true, table_compact(text_cm, n, worst))
    } else if compact {
        (wide, table_compact(span_cm, n, worst))
    } else {
        (wide, TableLevel::Fit)
    };
    let wide = ewide;
    let rotated = matches!(level, TableLevel::Rotate);
    if rotated {
        needs.rotating = true;
    }
    if wide && matches!(pos, Pos::Bottom | Pos::Both) && !rotated {
        needs.dblfloat = true;
    }
    // `[H]` is only emitted on the in-flow path: a sideways float
    // takes its own page, so force degrades to `[p]` there.
    if matches!(pos, Pos::Force) && !rotated {
        needs.float_h = true;
    }
    if matches!(pos, Pos::Barrier) {
        needs.barrier = true;
    }
    let env = if rotated {
        if wide {
            "sidewaystable*"
        } else {
            "sidewaystable"
        }
    } else if wide {
        "table*"
    } else {
        "table"
    };
    let placement = if rotated {
        "[p]".to_string()
    } else {
        pos.latex_spec().to_string()
    };
    // A rotated *wide* table lives on the (rotated) page height:
    // size it against `\textheight`, the dimension it fills.
    // A rotated *column* table keeps `\columnwidth` (the engine
    // promotes rotators to `wide`; this path only runs when the
    // promotion was not applied, and stretching narrow cells
    // across the page height would be worse than honest overflow).
    let span_w = if rotated && wide {
        "\\textheight"
    } else if wide {
        "\\textwidth"
    } else {
        "\\columnwidth"
    };
    let w = match width {
        None => span_w.to_string(),
        Some(f) if f.parse::<f64>().map(|v| v > 0.0 && v <= 2.0).unwrap_or(false) => {
            format!("{f}{span_w}")
        }
        Some(abs) => abs.to_string(),
    };
    let caption = format!("\\caption{{{cap}}}");
    let mut lines = vec![
        (if matches!(pos, Pos::Barrier) { "\\FloatBarrier\n" } else { "" }).to_string()
            + &format!("\\begin{{{env}}}{placement}"),
    ];
    if caption_top {
        lines.push(caption.clone());
    }
    // `Compact` steps one size down; `Rotate` steps two — a table
    // that fails portrait even compacted needs the smaller glyphs
    // to use the 23cm rotated width without print-on-print.
    lines.extend([
        (match level {
            TableLevel::Fit => "\\small",
            TableLevel::Compact => "\\footnotesize",
            TableLevel::Rotate => "\\scriptsize",
        })
        .to_string(),
        format!("\\begin{{tabularx}}{{{w}}}{{{}}}", spec.unwrap_or(&"X".repeat(n))),
        "\\hline".to_string(),
    ]);
    if !matches!(level, TableLevel::Fit) {
        // Ahead of the `tabularx` line: the preamble parses the
        // padding when the environment opens, so a later setting
        // would miss this table. Rotated tables pack tighter (2pt):
        // at 13 columns the padding itself is the overflow.
        let pad = if matches!(level, TableLevel::Rotate) { "2pt" } else { "3pt" };
        lines.insert(lines.len() - 2, format!("\\setlength{{\\tabcolsep}}{{{pad}}}"));
    }
    let mut first = true;
    for (ri, r) in rows.iter().enumerate() {
        if is_sep(r) {
            continue;
        }
        let mut cells = split_row(r);
        cells.resize(n, String::new());
        let mut rendered = Vec::new();
        for c in &cells {
            let mut cell = inline(c, keys, line + ri)?;
            if first {
                cell = format!("\\textbf{{{cell}}}");
            }
            rendered.push(cell);
        }
        lines.push(rendered.join(" & ") + " \\\\");
        if first {
            lines.push("\\hline".to_string());
            first = false;
        }
    }
    lines.push("\\hline".to_string());
    lines.push("\\end{tabularx}".to_string());
    if !caption_top {
        lines.push(caption);
    }
    lines.push(format!("\\end{{{env}}}"));
    Ok((lines.join("\n"), needs))
}

/// Every `@type{key,` entry key in a .bib source.
pub fn bib_keys_of(bib_src: &str) -> HashSet<String> {
    key_re().captures_iter(bib_src).map(|c| c[1].to_string()).collect()
}

/// Escape `_#%&` when not backslash-escaped (regex has no look-behind,
/// so this is a char scan).
fn escape_field_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_backslash = false;
    for c in s.chars() {
        if (c == '_' || c == '#' || c == '%' || c == '&') && !prev_backslash {
            out.push('\\');
        }
        prev_backslash = c == '\\' && !prev_backslash;
        out.push(c);
    }
    out
}

/// BibTeX-safe derivative: brace bare authors, escape specials outside
/// author/url/doi fields. Pure function over source text.
pub fn bib_safe(bib_src: &str) -> String {
    let author_re = Regex::new(r"^(\s*author\s*=\s*\{)(.*)(\},\s*)$").unwrap();
    let mut out = Vec::new();
    for raw in bib_src.lines() {
        let mut line = raw.to_string();
        if let Some(cap) = author_re.captures(raw) {
            if !cap[2].starts_with('{') {
                line = format!("{}{{{}}}{}", &cap[1], &cap[2], &cap[3]);
            }
        }
        if let Some(f) = field_re().captures(raw) {
            match f[1].to_lowercase().as_str() {
                "author" | "url" | "doi" => {}
                _ => line = escape_field_text(&line),
            }
        }
        out.push(line);
    }
    let mut s = out.join("\n");
    s.push('\n');
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sep_detection() {
        assert!(is_sep("| --- | :---: | --: |"));
        assert!(!is_sep("| a | b |"));
    }

    #[test]
    fn block_shape() {
        let rows = ["| A | B |".to_string(), "|--|--|".to_string(), "| `x_y` | 3 |".to_string()];
        let (got, needs) = table_block(&rows, "Cap.", None, None, Span::Column, Pos::Top, true, None, 8.9, 17.8, false, 4).unwrap();
        assert!(got.contains("\\begin{table}[t]"));
        assert!(got.contains("\\begin{tabularx}{\\columnwidth}{XX}"));
        assert!(got.contains("\\textbf{A} & \\textbf{B} \\\\"));
        assert!(got.contains("\\texttt{x\\_y} & 3 \\\\"));
        assert_eq!(needs, FloatNeeds::none());
    }

    #[test]
    fn wide_barrier_table() {
        let rows = ["| A |".to_string()];
        let (got, needs) =
            table_block(&rows, "C.", None, None, Span::Wide, Pos::Barrier, true, Some("0.8"), 8.9, 17.8, false, 4).unwrap();
        assert!(got.contains("\\FloatBarrier\n\\begin{table*}[t]"));
        assert!(got.contains("\\begin{tabularx}{0.8\\textwidth}{X}"));
        assert!(needs.barrier);
    }

    #[test]
    fn wide_force_table_is_an_error() {
        let rows = ["| A |".to_string()];
        assert!(table_block(&rows, "C.", None, None, Span::Wide, Pos::Force, true, None, 8.9, 17.8, false, 4).is_err());
    }

    #[test]
    fn appendix_spec() {
        let rows = ["| A | B | C |".to_string(), "| 1 | 2 | 3 |".to_string()];
        let (got, _) = table_block(&rows, "C.", None, Some("Xcl"), Span::Column, Pos::Top, true, None, 8.9, 17.8, false, 4).unwrap();
        assert!(got.contains("{Xcl}"));
    }

    #[test]
    fn ladder_thresholds() {
        // 7 cols in 17.8cm: share 2.54, usable 2.12 at 6pt padding.
        assert_eq!(table_compact(17.8, 7, 2.0), TableLevel::Fit);
        // Long prose token still fits: wrappable text never triggers.
        assert_eq!(table_compact(17.8, 7, 2.42), TableLevel::Compact);
        // Compact budget: 2.42*0.89=2.15 <= 2.54-0.22=2.32.
        // 13 cols in 17.8cm: share 1.37, compact budget 1.15.
        assert_eq!(table_compact(17.8, 13, 1.0), TableLevel::Compact);
        assert_eq!(table_compact(17.8, 13, 1.9), TableLevel::Rotate);
    }

    #[test]
    fn worst_token_ignores_wrappable_prose() {
        let rows = vec![vec!["government expenditure".to_string()], vec!["-0.234**(0.096)".to_string()]];
        // "government" is the longest prose token (10); the number
        // token (15 incl. stars) binds: 15*0.16 = 2.4.
        assert!((worst_token_cm(&rows) - 2.4).abs() < 1e-9, "got {}", worst_token_cm(&rows));
        // Math folds: `$\\rho$` counts one glyph, not seven chars.
        let rows = vec![vec!["$\\rho$".to_string()]];
        assert!((worst_token_cm(&rows) - 0.16).abs() < 1e-9, "got {}", worst_token_cm(&rows));
    }

    #[test]
    fn compact_table_shrinks_font_and_padding() {
        // 13 data cols of unbreakable numbers: rotates.
        let mut rows = vec!["| L |".to_string() + &"| -0.319(0.224) |".repeat(12)];
        rows.insert(0, "| H |".to_string() + &"| (1) |".repeat(12));
        rows.insert(1, "|---|".to_string() + &"|---|".repeat(12));
        let (got, needs) =
            table_block(&rows, "C.", None, None, Span::Wide, Pos::Top, true, None, 8.9, 17.8, true, 4).unwrap();
        assert!(got.contains("\\begin{sidewaystable*}[p]"), "rotates: {got}");
        assert!(got.contains("\\scriptsize"), "shrinks two steps");
        assert!(got.contains("\\setlength{\\tabcolsep}{2pt}"), "packs padding");
        assert!(!got.contains("\\small") && !got.contains("footnotesize"), "no larger size");
        assert!(needs.rotating);
        // 7 cols of 16-char tokens (2.56cm): portrait fits compacted
        // (2.56*0.89 = 2.28 <= 2.54-0.22) but not full-size.
        let rows7 = vec![
            "| L | -0.123456(0.12) | -0.123456(0.12) | -0.123456(0.12) | -0.123456(0.12) | -0.123456(0.12) | -0.123456(0.12) |".to_string(),
        ];
        let (got, needs) =
            table_block(&rows7, "C.", None, None, Span::Wide, Pos::Top, true, None, 8.9, 17.8, true, 4).unwrap();
        assert!(got.contains("\\begin{table*}[t]"), "stays portrait: {got}");
        assert!(got.contains("\\footnotesize"), "compacts");
        assert!(!needs.rotating);
    }

    #[test]
    fn ladder_off_by_default_stays_byte_identical() {
        // 13 unbreakable columns with the knob off: plain table*,
        // full-size type — the pre-ladder output, byte for byte.
        let mut rows = vec!["| L |".to_string() + &"| -0.319(0.224) |".repeat(12)];
        rows.insert(0, "| H |".to_string() + &"| (1) |".repeat(12));
        rows.insert(1, "|---|".to_string() + &"|---|".repeat(12));
        let (got, needs) =
            table_block(&rows, "C.", None, None, Span::Wide, Pos::Top, true, None, 8.9, 17.8, false, 4).unwrap();
        assert!(got.contains("\\begin{table*}[t]"), "untouched: {got}");
        assert!(got.contains("\\small"), "full size");
        assert!(!got.contains("sideways"), "no rotation");
        assert!(!needs.rotating);
    }

    #[test]
    fn bib_roundtrip() {
        let src = "@article{k,\n  author = {A. B and C. D},\n  title = {x_y and R&D},\n  url = {http://e/x_y},\n}\n";
        let safe = bib_safe(src);
        assert!(safe.contains("author = {{A. B and C. D}},"));
        assert!(safe.contains("title = {x\\_y and R\\&D},"));
        assert!(safe.contains("url = {http://e/x_y},"));
        let keys = bib_keys_of(src);
        assert!(keys.contains("k"));
    }
}
