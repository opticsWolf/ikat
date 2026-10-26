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
/// - `Tiny`: `\scriptsize` + 2pt padding but still portrait — the
///   PLOS-original density for dense regression tables.
///   (`Rotate` keeps the same 2pt packing: at 13 columns the
///   padding itself is the overflow.)
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
const SCRIPT_FACTOR: f64 = 0.78;
const TABCOLSEP_CM: f64 = 0.21;
const TABCOLSEP_SMALL_CM: f64 = 0.11;
const TABCOLSEP_TINY_CM: f64 = 0.07; // 2pt, like rotated tables

/// Compaction level for a table: column `share` vs the widest
/// unbreakable token. Shared with autolayout's reason notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableLevel {
    Fit,
    Compact,
    Tiny,
    Rotate,
}

pub(crate) fn table_compact(span_cm: f64, ncols: usize, worst_cm: f64) -> TableLevel {
    let share = span_cm / ncols.max(1) as f64;
    if worst_cm <= share - 2.0 * TABCOLSEP_CM {
        TableLevel::Fit
    } else if worst_cm * FOOT_FACTOR <= share - 2.0 * TABCOLSEP_SMALL_CM {
        TableLevel::Compact
    } else if worst_cm * SCRIPT_FACTOR <= share - 2.0 * TABCOLSEP_TINY_CM {
        // Portrait at `\scriptsize`: the PLOS-original density for
        // dense regression tables (7pt type, relative to the span,
        // matches the originals' 7.5pt on their wider measure).
        TableLevel::Tiny
    } else {
        TableLevel::Rotate
    }
}

/// Narrow glyph class: digits and numeric punctuation run well
/// under the 0.5em average the base ruler assumes (`-`/`.`/`(`/`)`
/// are 0.28-0.33em, `*` 0.39em). Weighted 0.85 — still above the
/// class mean fraction, so the estimate stays conservative.
fn narrow_glyph(c: char) -> bool {
    matches!(
        c,
        '0'..='9' | '.' | ',' | '+' | '-' | '*' | '(' | ')' | '%' | '/' | ':'
    )
}

/// Effective token length for tables: `label_len` folding for math
/// (`$..$` -> one glyph) and escapes (`\X` -> one glyph), with
/// narrow glyphs at 0.85 weight. Digit-heavy regression cells
/// shrink toward their true width; prose keeps full weight.
/// (No hyphenation discount: IEEEtran sets `\hyphenpenalty=750`,
/// so plain words effectively never break in X columns — every
/// whitespace-delimited run binds, exactly as measured.) Diagrams
/// keep `label_len`.
pub(crate) fn table_token_len(tok: &str) -> f64 {
    let mut eff = 0.0;
    let mut cs = tok.chars();
    while let Some(c) = cs.next() {
        if c == '$' {
            eff += 1.0;
            for d in cs.by_ref() {
                if d == '$' {
                    break;
                }
            }
        } else if c == '\\' {
            eff += 1.0;
            cs.next();
        } else if narrow_glyph(c) {
            eff += 0.85;
        } else {
            eff += 1.0;
        }
    }
    eff
}

/// Widest unbreakable token (whitespace-delimited run) across raw
/// cells, in cm. `label_len` folds `$..$` math and `\X` escapes to
/// one glyph each — the same ruler as the crowding floor.
pub(crate) fn worst_token_cm(rows: &[Vec<String>]) -> f64 {
    rows.iter()
        .flat_map(|r| r.iter())
        .flat_map(|c| c.split_whitespace())
        .map(table_token_len)
        .reduce(f64::max)
        .unwrap_or(0.0)
        * TABLE_TOKEN_CM
}

/// Split marker: survives `inline()` (`esc_text` would destroy a
/// literal `\newline`, emphasis/math slicing could split a plain
/// space) and is replaced with the real break after rendering. A
/// leak would fail loudly in TeX rather than print silently.
const STACK_MARK: &str = "\x00STACK\x00";

/// Estimate/SE split (M7.6): a cell ending in a digit-bearing
/// parenthesized group (`0.080***(0.015)`) stacks like the PLOS
/// originals — estimate above, error below — halving the longest
/// token. Returns the stacked cell with `STACK_MARK` at the break
/// (`inline()` runs after; the mark becomes `\newline`). Only
/// the trailing group splits; stars stay with the estimate, empty
/// heads and digit-free notes (`value (high)`) stay single-line.
pub(crate) fn split_est_se(cell: &str) -> Option<String> {
    let core = cell.trim_end_matches('*');
    let stars = &cell[core.len()..];
    let open = core.rfind('(')?;
    let (head, paren) = (&core[..open], &core[open..]);
    if !paren.ends_with(')') || head.trim().is_empty() {
        return None;
    }
    if !paren.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(format!("{head}{stars}{STACK_MARK}{paren}"))
}

/// Stacked worst token: splittable cells measure per line (the
/// break halves them); other cells measure whole as before.
pub(crate) fn worst_stacked_cm(rows: &[Vec<String>]) -> f64 {
    rows.iter()
        .flat_map(|r| r.iter())
        .map(|c| match split_est_se(c) {
            Some(stacked) => stacked
                .split(STACK_MARK)
                .flat_map(|line| line.split_whitespace())
                .map(table_token_len)
                .reduce(f64::max)
                .unwrap_or(0.0),
            None => c
                .split_whitespace()
                .map(table_token_len)
                .reduce(f64::max)
                .unwrap_or(0.0),
        })
        .reduce(f64::max)
        .unwrap_or(0.0)
        * TABLE_TOKEN_CM
}

/// Single decision point for the ladder, shared by the weaver and
/// the auto-layout notes (same inputs, same gate — suggestions and
/// output cannot disagree). Returns the effective span (a column
/// table the column cannot hold promotes to `wide`), the level,
/// and the rows to emit (stacked when stacking changed the worst
/// token). With the knob off every table stays `Fit` in its span.
/// Pinned-table height: stacked rows count double (two text
/// lines), plus header/rules, plus the caption. Pitches are
/// leading-order line heights per level (small 14.4pt, footnotesize
/// 12pt, scriptsize 10.5pt). Conservative — a pin that fits under
/// the in-flow budget always compiles; one over it strands a
/// half-empty column.
pub(crate) fn table_pin_height(rows: &[Vec<String>], level: TableLevel) -> f64 {
    let pitch = match level {
        TableLevel::Fit => 0.5,
        TableLevel::Compact => 0.45,
        TableLevel::Tiny => 0.4,
        TableLevel::Rotate => 0.37,
    };
    let lines: usize = rows
        .iter()
        .map(|r| {
            if r.iter().any(|c| c.contains(STACK_MARK)) {
                2
            } else {
                1
            }
        })
        .sum();
    lines as f64 * pitch + 1.0
}

pub(crate) fn decide_table(
    col_cm: f64,
    text_cm: f64,
    wide: bool,
    ncols: usize,
    data: Vec<Vec<String>>,
    compact: bool,
    here_max_cm: f64,
) -> (bool, TableLevel, Vec<Vec<String>>, bool) {
    if !compact {
        return (wide, TableLevel::Fit, data, false);
    }
    let span_cm = if wide { text_cm } else { col_cm };
    if table_compact(span_cm, ncols, worst_token_cm(&data)) == TableLevel::Fit {
        return (wide, TableLevel::Fit, data, false);
    }
    // Stacking attempt (M7.6): estimate/SE cells break over two
    // lines like the PLOS originals, halving the longest token.
    let stacked: Vec<Vec<String>> = data
        .iter()
        .map(|r| r.iter().map(|c| split_est_se(c).unwrap_or_else(|| c.clone())).collect())
        .collect();
    let data = if stacked != data { stacked } else { data };
    let worst = worst_stacked_cm(&data);
    let mut eff_wide = wide;
    let mut tall = false;
    if !wide && table_compact(col_cm, ncols, worst) == TableLevel::Rotate {
        eff_wide = true;
    } else if !wide {
        // Pin-height promotion: a stacked table too tall to pin
        // in-flow floats wide instead of overflowing the column
        // (`\vbox too high` — econ's 4-col interaction table
        // pinned 10 stacked rows `[H]`). Width alone approves the
        // column; height vetoes the pin.
        let col_level = table_compact(col_cm, ncols, worst);
        if table_pin_height(&data, col_level) > here_max_cm {
            eff_wide = true;
            tall = true;
        }
    }
    let span_cm = if eff_wide { text_cm } else { col_cm };
    (
        eff_wide,
        table_compact(span_cm, ncols, worst),
        data,
        tall,
    )
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
    here_max_cm: f64,
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
    // Row order survives (headers stay first): parallel line numbers
    // for per-row error carets.
    let raws: Vec<(usize, Vec<String>)> = rows
        .iter()
        .enumerate()
        .filter(|(_, r)| !is_sep(r))
        .map(|(i, r)| (line + i, split_row(r)))
        .collect();
    let data: Vec<Vec<String>> = raws.iter().map(|(_, c)| c.clone()).collect();
    // Single decision point (M7.5/M7.6): the engine mirrors this
    // exact call on the same inputs, so suggestions and output
    // agree before and after `--apply`.
    let (wide, level, out, tall) =
        decide_table(col_cm, text_cm, wide, n, data, compact, here_max_cm);
    // A height-promoted table cannot keep an in-flow pin: `here`
    // and `force` are column pins, illegal and unphysical on the
    // wide float — it floats top (same fail-soft as `here` -> `ht`).
    let pos = if tall && matches!(pos, Pos::Here | Pos::Force) {
        Pos::Top
    } else {
        pos
    };
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
    // `Compact` steps one size down; `Tiny` steps two but stays
    // portrait (PLOS-original density); `Rotate` steps two and turns
    // sideways — a table that fails portrait even tiny needs the
    // 23cm rotated width to avoid print-on-print.
    lines.extend([
        (match level {
            TableLevel::Fit => "\\small",
            TableLevel::Compact => "\\footnotesize",
            TableLevel::Tiny | TableLevel::Rotate => "\\scriptsize",
        })
        .to_string(),
        format!("\\begin{{tabularx}}{{{w}}}{{{}}}", spec.unwrap_or(&"X".repeat(n))),
        "\\hline".to_string(),
    ]);
    if !matches!(level, TableLevel::Fit) {
        // Ahead of the `tabularx` line: the preamble parses the
        // padding when the environment opens, so a later setting
        // would miss this table. `Tiny` and rotated tables pack
        // tighter (2pt): at 13 columns the padding itself overflows.
        let pad = if matches!(level, TableLevel::Tiny | TableLevel::Rotate) {
            "2pt"
        } else {
            "3pt"
        };
        lines.insert(lines.len() - 2, format!("\\setlength{{\\tabcolsep}}{{{pad}}}"));
    }
    let mut first = true;
    for ((ln, _), cells) in raws.iter().zip(out.iter()) {
        let mut cells = cells.clone();
        cells.resize(n, String::new());
        let mut rendered = Vec::new();
        for c in &cells {
            // Stack cells carry the split mark through `inline()`
            // (a literal `\\newline` would not survive escaping)
            // and it becomes the real break afterwards.
            let mut cell = inline(c, keys, *ln)?;
            cell = cell.replace(STACK_MARK, "\\newline ");
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
        let (got, needs) = table_block(&rows, "Cap.", None, None, Span::Column, Pos::Top, true, None, 8.9, 17.8, false, 7.0, 4).unwrap();
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
            table_block(&rows, "C.", None, None, Span::Wide, Pos::Barrier, true, Some("0.8"), 8.9, 17.8, false, 7.0, 4).unwrap();
        assert!(got.contains("\\FloatBarrier\n\\begin{table*}[t]"));
        assert!(got.contains("\\begin{tabularx}{0.8\\textwidth}{X}"));
        assert!(needs.barrier);
    }

    #[test]
    fn wide_force_table_is_an_error() {
        let rows = ["| A |".to_string()];
        assert!(table_block(&rows, "C.", None, None, Span::Wide, Pos::Force, true, None, 8.9, 17.8, false, 7.0, 4).is_err());
    }

    #[test]
    fn appendix_spec() {
        let rows = ["| A | B | C |".to_string(), "| 1 | 2 | 3 |".to_string()];
        let (got, _) = table_block(&rows, "C.", None, Some("Xcl"), Span::Column, Pos::Top, true, None, 8.9, 17.8, false, 7.0, 4).unwrap();
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
    fn est_se_split_rules() {
        // Stars stay with the estimate, break before the paren group.
        let s = split_est_se("0.080***(0.015)").unwrap();
        assert!(s.starts_with("0.080***"), "stars ride up: {s}");
        assert!(s.contains(STACK_MARK), "mark: {s}");
        assert!(s.ends_with("(0.015)"), "paren intact: {s}");
        // No parens, empty head, digit-free note: no split.
        assert!(split_est_se("Yes").is_none());
        assert!(split_est_se("(0.015)").is_none());
        assert!(split_est_se("value (high)").is_none());
        // Digit-free even with stars: still no split.
        assert!(split_est_se("est (se)**").is_none());
    }

    #[test]
    fn worst_token_ignores_wrappable_prose() {
        let rows = vec![vec!["government expenditure".to_string()], vec!["-0.234**(0.096)".to_string()]];
        // token binds: 15 narrow glyphs at 0.85 -> 12.75 eff chars,
        // 12.75*0.16 = 2.04. (IEEEtran sets `\hyphenpenalty=750`,
        // so `government` (10) binds whole at 1.6 — no hyphenation
        // discount; the prose row binds, the number row binds
        // harder.)
        assert!((worst_token_cm(&rows) - 2.04).abs() < 1e-9, "got {}", worst_token_cm(&rows));
        // Long single words bind: `globalization` (13) overflows a
        // 13-col share whole — measured, not discounted.
        let rows = vec![vec!["globalization".to_string()]];
        assert!((worst_token_cm(&rows) - 13.0 * 0.16).abs() < 1e-9, "got {}", worst_token_cm(&rows));
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
            table_block(&rows, "C.", None, None, Span::Wide, Pos::Top, true, None, 8.9, 17.8, true, 7.0, 4).unwrap();
        assert!(got.contains("\\begin{sidewaystable*}[p]"), "rotates: {got}");
        assert!(got.contains("\\scriptsize"), "shrinks two steps");
        assert!(got.contains("\\setlength{\\tabcolsep}{2pt}"), "packs padding");
        assert!(!got.contains("\\small") && !got.contains("footnotesize"), "no larger size");
        assert!(needs.rotating);
        // 7 cols of 16-char paren-free tokens (2.56cm): portrait fits
        // compacted (2.56*0.89 = 2.28 <= 2.54-0.22). No parens, so no
        // stacking involved — pure compaction.
        let rows7 = vec![
            "| L | -0.1234567890123 | -0.1234567890123 | -0.1234567890123 | -0.1234567890123 | -0.1234567890123 | -0.1234567890123 |".to_string(),
        ];
        let (got, needs) =
            table_block(&rows7, "C.", None, None, Span::Wide, Pos::Top, true, None, 8.9, 17.8, true, 7.0, 4).unwrap();
        assert!(got.contains("\\begin{table*}[t]"), "stays portrait: {got}");
        assert!(got.contains("\\footnotesize"), "compacts");
        assert!(!got.contains(STACK_MARK), "no split without parens");
        assert!(!needs.rotating);
    }

    #[test]
    fn stacked_cells_fit_portrait_at_full_size() {
        // The PLOS-original trick: `0.080***(0.015)` (15 raw chars,
        // 2.4cm) stacks estimate over error, longest line 9 chars
        // (1.44cm) — fits a 7-col share (2.12cm) at full size.
        let rows = vec![
            "| L | -0.123456(0.1234) | -0.123456(0.1234) | -0.123456(0.1234) | -0.123456(0.1234) | -0.123456(0.1234) | -0.123456(0.1234) |".to_string(),
        ];
        let (got, _) =
            table_block(&rows, "C.", None, None, Span::Wide, Pos::Top, true, None, 8.9, 17.8, true, 7.0, 4).unwrap();
        assert!(got.contains("\\begin{table*}[t]"), "stays portrait");
        assert!(got.contains("\\small"), "full size kept");
        assert!(got.contains("\\newline "), "break emitted");
    }

    #[test]
    fn tiny_sets_scriptsize_portrait() {
        // 13 cols with 10-char stacked heads, all narrow -> 8.5 eff
        // = 1.36cm: too wide for Compact (1.36*0.89 = 1.21 > 1.37
        // -0.22) but fits Tiny (1.36*0.78 = 1.06 <= 1.37-0.14):
        // portrait scriptsize, no sideways turn.
        let mut rows = vec!["| L".to_string() + &" | -0.1234567(0.12)".repeat(12) + " |"];
        rows.insert(0, "| H".to_string() + &" | (1)".repeat(12) + " |");
        rows.insert(1, "|---".to_string() + &"|---".repeat(12) + "|");
        let (got, needs) =
            table_block(&rows, "C.", None, None, Span::Wide, Pos::Top, true, None, 8.9, 17.8, true, 7.0, 4).unwrap();
        assert!(got.contains("\\begin{table*}[t]"), "stays portrait: {got}");
        assert!(got.contains("\\scriptsize"), "two steps down");
        assert!(got.contains("\\setlength{\\tabcolsep}{2pt}"), "packs like rotated");
        assert!(!got.contains("sideways"), "never turns: {got}");
        assert!(!needs.rotating);
    }

    #[test]
    fn tall_stacked_column_promotes_to_wide_top() {
        // A 4-col table that fits the column portrait (Compact)
        // but stacks 10 rows tall (~10cm over the 7cm in-flow
        // budget): pinning `[H]` would overflow the column, so it
        // promotes wide and the pin degrades to top.
        let mut rows = vec!["| L | (1) | (2) | (3) |".to_string()];
        rows.extend((0..10).map(|_| "| Laboratory | 0.086***(0.016) |  |  |".to_string()));
        let (got, needs) =
            table_block(&rows, "C.", None, None, Span::Column, Pos::Force, true, None, 8.9, 17.8, true, 7.0, 4).unwrap();
        assert!(got.contains("\\begin{table*}[t]"), "floats wide top: {got}");
        assert!(!needs.float_h, "no in-flow pin kept");
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
            table_block(&rows, "C.", None, None, Span::Wide, Pos::Top, true, None, 8.9, 17.8, false, 7.0, 4).unwrap();
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
