//! Auto-layout (M7.1): measure floats, suggest `span`/`pos`.
//!
//! The doctrine, stated up front so nobody mistakes this for a
//! pagination simulator: ikat knows content sizes but NOT text flow
//! (only LaTeX knows where a paragraph breaks), so this module
//! classifies floats by measured size and recommends the placement
//! the showcase paper proved by hand — it never simulates pages.
//! LaTeX still places everything; `ikat layout` only proposes the
//! constraints, and `--apply` (Python side) rewrites fence attrs.
//!
//! Measurement sources, per kind:
//!
//! - diagrams: the emitted TikZ itself. `flowchart_to_tikz` (which
//!   also dispatches sequence/state) returns coordinates in TikZ cm,
//!   so a coordinate scan of its output is a true content box — no
//!   re-parsing, no emitter churn, golden-safe by construction. A
//!   fixed text margin covers labels outside node centers.
//! - plots: preset JSON (`kind: bar|line`) measures pgfplots default
//!   axis geometry (constants below — `width=240pt`, `height=207pt`,
//!   plus title and axis labels; the emitters set no explicit dims).
//!   A `below` legend adds one row. Estimates, documented as such.
//!   External plot PDFs (the `spec.plots` registry shape) keep hand
//!   attrs — same rule as pictures.
//! - tables: row count times `\small` row height; width from the
//!   widest cell per column times a per-character estimate. X-column
//!   wrapping means wide prose cells overestimate — conservative on
//!   purpose (a column table that fits loosely still fits).
//! - pictures (precompiled PDFs): unmeasurable — kept as-is.
//!
//! Policy (defaults in `[layout]`): short tables and plots pin with
//! `force` ([H] prints exactly at the paragraph), short diagrams sit
//! with `here` (fail-soft: `[h]` promotes to `[ht]` when the column
//! remainder is short), tall diagrams take a dedicated `page`.
//! Non-lead content taller than `page_frac` of the text height is
//! pre-empted to `page` before any other test; the section lead is
//! exempt and gets `lead_bonus_cm` extra budget instead (section
//! starts land near fresh columns). Wide content that cannot fit a
//! column even at `min_width` goes `wide`+`top`. `width=` shrinks
//! for column overflow only, never for height (coordinates scale,
//! text does not — shrinking tall content crowds labels), floored
//! by the crowding floor measured from placed node labels and
//! capped at 15% over the fit (flagged when capped). A `wide`
//! float declared after `late_wide_lines` prose lines earns a
//! move-the-fence note (page-top floats declared late strand
//! trailing pages) — advisory, the engine never reorders. A `wide`
//! float taller than the page-top budget (`top_frac` of the text
//! height) takes a dedicated `page` instead of `top`: `[t]` cannot
//! legally hold it, so it (and the queue behind it) would pile at
//! the document end. The third and later `wide` floats since a
//! section heading earn a spread-the-fences note for the same
//! queue reason.

use crate::config::{LayoutPolicy, Pos, Span};
use crate::doc::{fence_attr_str, parse_attrs, BuildSpec};
use crate::error::Error;
use serde::Serialize;

/// Margin around measured TikZ coordinates (cm): the `layout::node_box`
/// halves (1.7 x, 0.45 y — the project's conservative text model)
/// plus edge-label air above. Coordinates mark node centers; text
/// sticks out past them, and the 2.4pt overfull on the showcase
/// sequence at width=0.60 proved 0.6 too tight.
const TIKZ_MARGIN_X: f64 = 1.7;
const TIKZ_MARGIN_Y: f64 = 0.8;

/// pgfplots default axis geometry in cm (`width=240pt`,
/// `height=207pt`), plus title above and axis labels. The emitters
/// set no explicit dims, so every plot ships at this natural size
/// before `width=` scaling.
const PLOT_W1: f64 = 9.5;
const PLOT_H1: f64 = 8.8;
/// Extra height when the legend sits below the axis (one row).
const LEGEND_BELOW_CM: f64 = 0.8;

/// `\small` table row pitch and header allowance (cm).
const TABLE_ROW_CM: f64 = 0.55;
const TABLE_HEAD_CM: f64 = 0.7;
/// Per-character width estimate for `\small` table cells (cm).
const TABLE_CHAR_CM: f64 = 0.11;
/// Table chrome (rules, padding) allowance (cm).
const TABLE_CHROME_CM: f64 = 0.6;

/// One suggestion, JSON-serializable for the Python side. `index`
/// is the fence ordinal within its kind (mermaid fence #, table #,
/// plot key for plots) — the unit `apply` rewrites. `width` is a
/// fraction, omitted (`None`) at natural size.
#[derive(Debug, Clone, Serialize)]
pub struct Suggestion {
    pub index: usize,
    pub kind: String,
    pub key: String,
    pub line: usize,
    pub est_w_cm: f64,
    pub est_h_cm: f64,
    pub span: String,
    pub pos: String,
    pub width: Option<f64>,
    pub action: String,
    pub reason: String,
}

/// Scan `(x, y)` numeric coordinate pairs out of TikZ source and
/// return the content box. Non-numeric parens (node ids like `(A)`,
/// `(axis cs:…)` prefixes) are skipped — only `(` immediately
/// followed by a number parses.
pub(crate) fn tikz_bbox(tikz: &str) -> Option<(f64, f64, f64, f64)> {
    let b = tikz.as_bytes();
    let mut i = 0usize;
    let mut pts: Vec<(f64, f64)> = Vec::new();
    while i < b.len() {
        if b[i] == b'(' {
            if let Some((x, j)) = scan_num(b, i + 1) {
                let k = skip_ws(b, j);
                if k < b.len() && b[k] == b',' {
                    if let Some((y, l)) = scan_num(b, k + 1) {
                        let m = skip_ws(b, l);
                        if m < b.len() && b[m] == b')' {
                            pts.push((x, y));
                            i = m + 1;
                            continue;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if pts.is_empty() {
        return None;
    }
    let (mut x0, mut y0, mut x1, mut y1) =
        (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for (x, y) in pts {
        x0 = x0.min(x);
        y0 = y0.min(y);
        x1 = x1.max(x);
        y1 = y1.max(y);
    }
    Some((x0, y0, x1, y1))
}

fn skip_ws(b: &[u8], mut i: usize) -> usize {
    while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
        i += 1;
    }
    i
}

/// `[-]d[.d]` at `b[i..]` -> (value, first index past it).
fn scan_num(b: &[u8], i: usize) -> Option<(f64, usize)> {
    let mut j = skip_ws(b, i);
    let start = j;
    if j < b.len() && (b[j] == b'-' || b[j] == b'+') {
        j += 1;
    }
    let mut digits = 0usize;
    while j < b.len() && b[j].is_ascii_digit() {
        j += 1;
        digits += 1;
    }
    if j < b.len() && b[j] == b'.' {
        j += 1;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return None;
    }
    std::str::from_utf8(&b[start..j]).ok()?.parse::<f64>().ok().map(|v| (v, j))
}

/// Placed node labels from TikZ source: `(center-x, center-y,
/// longest label line in chars)`. Parses ONLY `\node (ID) at (x,y)`
/// lines (emitted by `emit::tikz_node`) — edge labels (`\node[midway`)
/// and composite box labels (`\node[anchor=`) are full-width-safe by
/// construction and must not constrain the shrink floor.
/// Label length counts `\X` escapes as one char and splits `\\`
/// line breaks (approximation, ±20% — a floor, not a fit).
pub(crate) fn node_labels(tikz: &str) -> Vec<(f64, f64, usize)> {
    let mut nodes = Vec::new();
    for line in tikz.lines() {
        let t = line.trim_start();
        let rest = match t.strip_prefix("\\node (") {
            Some(r) => r,
            None => continue,
        };
        let (id, rest) = match rest.split_once(')') {
            Some(p) => p,
            None => continue,
        };
        if id.contains(' ') || id.is_empty() {
            continue;
        }
        let rest = match rest.strip_prefix(" at (") {
            Some(r) => r,
            None => continue,
        };
        let b = rest.as_bytes();
        let (x, j) = match scan_num(b, 0) {
            Some(p) => p,
            None => continue,
        };
        let k = skip_ws(b, j);
        if k >= b.len() || b[k] != b',' {
            continue;
        }
        let (y, l) = match scan_num(b, k + 1) {
            Some(p) => p,
            None => continue,
        };
        let m = skip_ws(b, l);
        if m >= b.len() || b[m] != b')' {
            continue;
        }
        let tail = &rest[m + 1..];
        let braced = match tail.find('{') {
            Some(i) => &tail[i..],
            None => continue,
        };
        let label = match balanced_braces(braced) {
            Some(s) => s,
            None => continue,
        };
        nodes.push((x, y, label_len(&label)));
    }
    nodes
}

/// First balanced `{...}` group (without the outer braces).
fn balanced_braces(s: &str) -> Option<String> {
    let mut depth = 0usize;
    let mut start = None;
    for (i, c) in s.char_indices() {
        if c == '{' {
            if depth == 0 {
                start = Some(i + 1);
            }
            depth += 1;
        } else if c == '}' {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return start.map(|a| s[a..i].to_string());
            }
        }
    }
    None
}

/// Longest `\\`-separated line, `\X` escapes collapsed to one char.
pub(crate) fn label_len(label: &str) -> usize {
    let plain = label
        .replace("\\textbackslash{}", "x")
        .replace("\\textasciitilde{}", "x")
        .replace("\\textasciicircum{}", "x")
        .replace("\\checkmark{}", "x");
    plain
        .split("\\\\")
        .map(|seg| {
            let mut n = 0usize;
            let mut chars = seg.chars();
            while let Some(c) = chars.next() {
                if c == '\\' {
                    // `\X` escape: one glyph-ish.
                    chars.next();
                    n += 1;
                } else if c == '$' {
                    for d in chars.by_ref() {
                        if d == '$' {
                            break;
                        }
                    }
                    n += 1;
                } else {
                    n += 1;
                }
            }
            n
        })
        .max()
        .unwrap_or(0)
}

/// Per-character width for node labels (cm). Calibrated on the
/// showcase sequence: 0.52-width boxes touch, 0.57 clears.
const NODE_CHAR_CM: f64 = 0.21;

/// Crowding floor: the narrowest `width=` that keeps same-rank node
/// labels apart. Pairwise over placed nodes: two labels need
/// `(lenA+lenB)/2` chars side by side, so `w >= need/dx`. Only
/// same-rank pairs (`|dy| < 1.0`, rows are 1.1cm+ apart) count;
/// vertical stacks never collide.
pub(crate) fn crowd_floor(nodes: &[(f64, f64, usize)]) -> f64 {
    let mut f = 0.0;
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let dx = (nodes[i].0 - nodes[j].0).abs();
            let dy = (nodes[i].1 - nodes[j].1).abs();
            if dx < 0.5 || dx > 6.0 || dy >= 1.0 {
                continue;
            }
            let need = (nodes[i].2 + nodes[j].2) as f64 / 2.0 * NODE_CHAR_CM / dx;
            if need > f {
                f = need;
            }
        }
    }
    (f * 100.0).ceil() / 100.0
}

/// Natural (unscaled) content size of one inline diagram fence, plus
/// the crowding floor (0.0 when nothing constrains it).
pub(crate) fn measure_diagram(src: &str, base: usize) -> Result<(f64, f64, f64), Error> {
    let tikz = crate::mermaid::flowchart_to_tikz(src, base)?;
    let (x0, y0, x1, y1) = tikz_bbox(&tikz)
        .ok_or_else(|| Error::new("diagram has no measurable coordinates", base, String::new()))?;
    Ok((
        (x1 - x0 + 2.0 * TIKZ_MARGIN_X).max(1.0),
        (y1 - y0 + 2.0 * TIKZ_MARGIN_Y).max(1.0),
        crowd_floor(&node_labels(&tikz)),
    ))
}

/// Natural content size of one plot preset: pgfplots defaults, plus
/// a legend row when the preset puts the legend below the axis.
pub(crate) fn measure_plot(preset_src: &str) -> (f64, f64) {
    let below = serde_json::from_str::<serde_json::Value>(preset_src)
        .ok()
        .and_then(|v| v.get("legend").and_then(|l| l.as_str()).map(|s| s.to_string()))
        .map(|s| s == "below")
        .unwrap_or(false);
    (PLOT_W1, PLOT_H1 + if below { LEGEND_BELOW_CM } else { 0.0 })
}

/// Natural content size of one pipe table: rows times row pitch,
/// width from the widest cell per column.
pub(crate) fn measure_table(rows: &[String]) -> (f64, f64) {
    let data: Vec<Vec<String>> = rows
        .iter()
        .filter(|r| !crate::table::is_sep(r))
        .map(|r| crate::table::split_row(r))
        .collect();
    // Header row counts even when it is the only row.
    let n = data.len().max(1) as f64;
    let ncols = data.iter().map(|r| r.len()).max().unwrap_or(1);
    let mut w = TABLE_CHROME_CM;
    for c in 0..ncols {
        let widest = data.iter().filter_map(|r| r.get(c)).map(|s| s.chars().count()).max().unwrap_or(0);
        w += widest as f64 * TABLE_CHAR_CM;
    }
    (w, TABLE_HEAD_CM + n * TABLE_ROW_CM)
}

/// Fit width for content `w1` in a `span_w` column: exact ratio with
/// 2% estimator headroom, rounded to 2dp. Deliberately NOT floored
/// onto a grid — flooring cost the 0.03 that crowds sequence node
/// labels (proven by probe: 0.50 touches, 0.53 clears).
fn fit_width(span_w: f64, w1: f64) -> f64 {
    ((0.98 * span_w / w1 * 100.0).round()) / 100.0
}

fn fmt2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

/// Policy core: natural content size plus current attrs in,
/// recommended `(span, pos, width)` plus reason out.
pub(crate) fn suggest_for(
    p: &LayoutPolicy,
    kind: &str,
    w1: f64,
    h1: f64,
    cur_span: &str,
    cur_pos: &str,
    lead: bool,
    floor: f64,
) -> (String, String, Option<f64>, String) {
    let col = p.column_cm;
    let cap = p.caption_cm;
    // Page-top budget: content + caption taller than this cannot
    // take a `[t]` top (topfraction/dbltopfraction) — LaTeX refuses
    // the float and defers it (and its queue) toward the end.
    let top_max = p.top_frac * p.textheight_cm;
    if kind == "table" {
        let span = if w1 > p.wide_ratio * col { "wide" } else { "column" };
        let tall = h1 + cap;
        if span == "wide" && tall > top_max {
            return (
                "wide".to_string(),
                "page".to_string(),
                None,
                format!(
                    "table est {w1:.1}cm wide, {tall:.1}cm tall: overflows the {col}cm column and exceeds the {top_max:.1}cm page-top budget -> dedicated page ([t] cannot hold it, the queue would strand at the document end)"
                ),
            );
        }
        if span == "column" && tall > p.textheight_cm {
            return (
                "column".to_string(),
                "page".to_string(),
                None,
                format!(
                    "table est {tall:.1}cm tall: taller than the {:.1}cm text height, [H] would blast through the page bottom -> dedicated page",
                    p.textheight_cm
                ),
            );
        }
        let pos = if span == "wide" { "top" } else { "force" };
        let reason = format!(
            "table est {w1:.1}cm wide: {} -> {span}+{pos}",
            if span == "wide" {
                format!("overflows the {col}cm column, floats top")
            } else {
                "[H] pins it to its paragraph".to_string()
            }
        );
        return (span.to_string(), pos.to_string(), None, reason);
    }
    if kind == "picture" {
        return (
            cur_span.to_string(),
            cur_pos.to_string(),
            None,
            "external graphic: size unknown, hand attrs stand".to_string(),
        );
    }
    // Diagrams and plots: fit width for the column first.
    if kind == "plot" {
        // Data graphics pin to their paragraph ([H] prints exactly
        // where declared); size is bounded by the axis defaults, so
        // only the width shrinks, never the placement.
        let mut w = fit_width(col, w1).min(1.0);
        if w < p.min_width {
            w = p.min_width;
        }
        let width = if w < 0.995 { Some(w) } else { None };
        let h = h1 * w + cap;
        let reason = format!(
            "plot est {w1:.1}x{h1:.1}cm natural: width={} pins [H] at {h:.1}cm with caption",
            match width {
                Some(v) => format!("{v:.2}"),
                None => "natural".to_string(),
            }
        );
        return ("column".to_string(), "force".to_string(), width, reason);
    }
    // Diagrams: `width=` shrinks for COLUMN OVERFLOW only, never for
    // height — coordinates scale but text does not, so shrinking a
    // tall diagram crowds its labels (proven by probe: a 4-node
    // pipeline at width=0.45 overlaps adjacent node text). Height is
    // decided by `pos`: the section lead sits `here` at natural size
    // (fresh-column odds, fail-soft `[h]` promotion); a non-lead
    // taller than budget takes a dedicated `page`.
    let budget = p.here_max_cm + if lead { p.lead_bonus_cm } else { 0.0 };
    // Tall pre-empt: content taller than `page_frac` of the text height
    // takes a dedicated page even before shrinking is tried — shrinking
    // buys column fit, never in-flow fit. Section leads are exempt
    // (fresh-column odds, decided by `budget` below).
    if !lead && h1 + cap > p.page_frac * p.textheight_cm {
        let lim = p.page_frac * p.textheight_cm;
        return (
            "column".to_string(),
            "page".to_string(),
            None,
            format!(
                "tall diagram: est {:.1}cm exceeds the page pre-empt ({:.1}cm = {:.0}% of the text height) -> dedicated page",
                h1 + cap,
                lim,
                p.page_frac * 100.0,
            ),
        );
    }
    if w1 <= col {
        // Fits the column: never shrink. In-flow when lead or short.
        if lead || h1 + cap <= budget {
            let why = if lead { "section lead sits at natural size" } else { "short enough to sit" };
            return (
                "column".to_string(),
                "here".to_string(),
                None,
                format!("est {w1:.1}x{h1:.1}cm fits the column: {why} -> here"),
            );
        }
        return (
            "column".to_string(),
            "page".to_string(),
            None,
            format!(
                "tall diagram: est {h1:.1}cm content exceeds the {budget:.1}cm in-flow budget -> dedicated page"
            ),
        );
    }
    // Width overflow: the crowding floor binds first (labels apart),
    // the est-fit caps it — the margin model overestimates true
    // width ~10%, so 15% over the fit stays truly fitting. When the
    // floor exceeds the cap, fit wins and the reason says so.
    let fit = fit_width(col, w1).min(1.0);
    let mut w = fit.max(floor).min(1.0);
    let mut flag = String::new();
    if w > 1.15 * fit {
        w = ((1.15 * fit).max(p.min_width) * 100.0).round() / 100.0;
        flag = format!("label floor {floor:.2} capped at fit headroom: crowding risk remains");
    }
    w = w.max(p.min_width);
    let width = if w < 0.995 { Some(w) } else { None };
    let h = h1 * w + cap;
    // Escape hatch: content that cannot fit a column at all goes
    // wide instead of overflowing a page float — `page` when too
    // tall for a top (same budget as tables), `top` otherwise.
    if w1 * p.min_width > col {
        let ww = fit_width(p.textwidth_cm, w1).min(1.0);
        let width = if ww < 0.995 { Some(ww) } else { None };
        let tall = h1 * ww + cap;
        if tall > top_max {
            return (
                "wide".to_string(),
                "page".to_string(),
                width,
                format!("content est {w1:.1}cm overflows a {col}cm column at any readable width and {tall:.1}cm exceeds the {top_max:.1}cm page-top budget -> dedicated page"),
            );
        }
        return (
            "wide".to_string(),
            "top".to_string(),
            width,
            format!("content est {w1:.1}cm overflows a {col}cm column at any readable width -> wide+top"),
        );
    }
    if lead || h <= budget {
        let mut notes = Vec::new();
        if lead {
            notes.push("section lead".to_string());
        }
        if floor > fit {
            notes.push(format!("floor {floor:.2} keeps node labels apart"));
        }
        if !flag.is_empty() {
            notes.push(flag);
        }
        let tail = if notes.is_empty() { String::new() } else { format!("; {}", notes.join("; ")) };
        return (
            "column".to_string(),
            "here".to_string(),
            width,
            format!("shrunk to fit the column (est {h:.1}cm with caption{tail}) -> here"),
        );
    }
    (
        "column".to_string(),
        "page".to_string(),
        width,
        format!("tall diagram: est {h:.1}cm shrunk still exceeds the {budget:.1}cm budget -> dedicated page"),
    )
}

fn span_str(s: &Span) -> &'static str {
    match s {
        Span::Column => "column",
        Span::Wide => "wide",
    }
}

fn pos_str(p: &Pos) -> &'static str {
    match p {
        Pos::Top => "top",
        Pos::Bottom => "bottom",
        Pos::Both => "both",
        Pos::Page => "page",
        Pos::Here => "here",
        Pos::Force => "force",
        Pos::Barrier => "barrier",
    }
}

/// Walk the manuscript exactly the way `convert()` pairs fences
/// with registry entries (mermaid fence #N ↔ `diagrams[N]`, table #N
/// ↔ `table_captions[N]`, plots from `spec.plots`), measure each
/// float, and suggest `span`/`pos`/`width`. Strict-subset and
/// registry errors propagate with lines — a manuscript that cannot
/// weave cannot be measured either.
pub fn suggest_layout(
    md_text: &str,
    toml_src: &str,
    spec: &BuildSpec,
) -> Result<Vec<Suggestion>, Error> {
    use crate::md::{blocks, MdKind};
    let cfg = crate::config::Config::from_toml(toml_src)
        .map_err(|e| Error::new(e, 1, String::new()))?;
    let p = &cfg.layout;
    let mut out: Vec<Suggestion> = Vec::new();
    let mut pending_table: Option<(crate::doc::ElemAttrs, usize)> = None;
    let mut diagram_idx = 0usize;
    let mut table_idx = 0usize;
    // Diagrams since the last `##` section: the first one is the
    // section lead (fresh-column odds, inflow bonus in policy).
    let mut section_diagrams = 0usize;
    // Total body prose (tail rule input) and the remainder ahead of
    // each float: a `wide`+`top` with almost no text left has no
    // downstream tops — dedicated page instead of end pile / eject.
    let total_prose: usize = blocks(md_text)?
        .iter()
        .filter_map(|b| match &b.kind {
            MdKind::Para(para) => Some(para.iter().filter(|l| !l.trim().is_empty()).count()),
            _ => None,
        })
        .sum();
    let mut seen_prose = 0usize;
    // Non-empty prose lines since the last float or section start:
    // a `wide` float declared after a wall of prose strands pages.
    let mut prose_lines = 0usize;
    // `wide` floats declared since the last `##` heading: page tops
    // queue behind each other, so the third and later in one stretch
    // earn a spread-the-fences note (same strand mechanism as late
    // declaration, cumulative instead of positional).
    let mut wides = 0usize;
    for b in blocks(md_text)? {
        let line = b.line;
        match b.kind {
            MdKind::H2(_) => {
                section_diagrams = 0;
                prose_lines = 0;
                wides = 0;
            }
            MdKind::Para(para) => {
                let n = para.iter().filter(|l| !l.trim().is_empty()).count();
                prose_lines += n;
                seen_prose += n;
            }
            MdKind::Fence { info: s, body: fence_body } => {
                if !s.contains("mermaid") {
                    continue;
                }
                if diagram_idx >= spec.diagrams.len() {
                    return Err(Error::new(
                        format!("mermaid fence #{diagram_idx} has no registry entry"),
                        line,
                        s.clone(),
                    ));
                }
                let entry = &spec.diagrams[diagram_idx];
                let (attrs, inline_flag) = crate::doc::parse_attrs(
                    fence_attr_str(&s),
                    &format!("mermaid fence #{diagram_idx}"),
                    line,
                )?;
                let mode = if inline_flag || s.contains("{inline}") { "inline" } else { entry.mode.as_str() };
                let kind = if mode == "inline" { "diagram" } else { "picture" };
                let cur_span = attrs.span.map(|x| span_str(&x).to_string()).unwrap_or_else(|| "default".to_string());
                let cur_pos = attrs.pos.map(|x| pos_str(&x).to_string()).unwrap_or_else(|| "default".to_string());
                let natural: Option<(f64, f64, f64)> = if kind == "diagram" {
                    Some(measure_diagram(&fence_body, line + 1).map_err(|e| Error {
                        msg: format!("mermaid fence #{diagram_idx}: {}", e.msg),
                        ..e
                    })?)
                } else {
                    None
                };
                let (w1, h1, floor) = natural.unwrap_or((0.0, 0.0, 0.0));
                let lead = section_diagrams == 0;
                section_diagrams += 1;
                let (mut span, mut pos, width, mut reason) = if kind == "picture" {
                    suggest_for(p, "picture", w1, h1, &cur_span, &cur_pos, false, 0.0)
                } else {
                    suggest_for(p, "diagram", w1, h1, &cur_span, &cur_pos, lead, floor)
                };
                tail_page(p, &mut span, &mut pos, &mut reason, total_prose, seen_prose);
                reason.push_str(&late_note(p, &span, prose_lines));
                reason.push_str(&queue_note(&span, &mut wides));
                prose_lines = 0;
                let same_width = match (attrs.width.as_deref(), width) {
                    (None, None) => true,
                    (Some(a), Some(w)) => a.parse::<f64>().map(|v| (v - w).abs() < 0.005).unwrap_or(false),
                    _ => false,
                };
                let action = if cur_span == span && cur_pos == pos && same_width { "keep" } else { "change" };
                out.push(Suggestion {
                    index: diagram_idx,
                    kind: kind.to_string(),
                    key: entry.key.clone(),
                    line,
                    est_w_cm: fmt2(w1),
                    est_h_cm: fmt2(h1),
                    span,
                    pos,
                    width,
                    action: action.to_string(),
                    reason,
                });
                diagram_idx += 1;
            }
            MdKind::Directive(a) => {
                let (attrs, _) = parse_attrs(&a, &format!("table #{table_idx}"), line)?;
                if attrs.span.is_some() || attrs.pos.is_some() || attrs.width.is_some() || attrs.captionpos.is_some() {
                    pending_table = Some((attrs, line));
                }
            }
            MdKind::Table(rows) => {
                if table_idx >= spec.table_captions.len() {
                    return Err(Error::new(
                        format!("table #{table_idx} has no caption"),
                        line,
                        rows.first().cloned().unwrap_or_default(),
                    ));
                }
                let (tattrs, dline) = pending_table.take().map(|(a, l)| (a, l)).unwrap_or((crate::doc::ElemAttrs::default(), line));
                let (w1, h1) = measure_table(&rows);
                let cur_span = tattrs.span.map(|s| span_str(&s).to_string()).unwrap_or_else(|| "default".to_string());
                let cur_pos = tattrs.pos.map(|s| pos_str(&s).to_string()).unwrap_or_else(|| "default".to_string());
                let (mut span, mut pos, width, mut reason) = suggest_for(p, "table", w1, h1, &cur_span, &cur_pos, false, 0.0);
                // Portrait-fit ladder via the shared decision: same
                // inputs, same gate as the weaver, so the suggestion
                // and the output agree before and after `--apply`.
                // `stacked` reports whether estimate/SE stacking
                // changed the worst token (the PLOS-original trick).
                {
                    use crate::table::{decide_table, is_sep, split_row, worst_stacked_cm, worst_token_cm, TableLevel};
                    let data: Vec<Vec<String>> = rows
                        .iter()
                        .filter(|r| !is_sep(r))
                        .map(|r| split_row(r))
                        .collect();
                    let ncols = data.iter().map(|r| r.len()).max().unwrap_or(1);
                    let raw_worst = worst_token_cm(&data);
                    let cur_wide = span == "wide";
                    let on = cfg.tables.compact_tables;
                    let (eff_wide, level, out) = decide_table(
                        p.column_cm,
                        p.textwidth_cm,
                        cur_wide,
                        ncols,
                        data.clone(),
                        on,
                    );
                    // Advisory twin with the knob forced on: names the
                    // opt-in when the manuscript leaves it off.
                    let (adv_wide, adv_level, adv_out) = decide_table(
                        p.column_cm,
                        p.textwidth_cm,
                        cur_wide,
                        ncols,
                        data,
                        true,
                    );
                    let stacked = worst_stacked_cm(&out) < raw_worst;
                    let share =
                        (if eff_wide { p.textwidth_cm } else { p.column_cm }) / ncols as f64;
                    let worst = worst_stacked_cm(&out);
                    if eff_wide && !cur_wide {
                        span = "wide".to_string();
                        reason.push_str(&format!(
                            "; column cannot hold it (worst token {raw_worst:.2}cm vs {:.2}cm column share): promoted to wide",
                            p.column_cm / ncols as f64
                        ));
                        // `force`/`here` are illegal on `table*`: the
                        // in-flow pin becomes a page top (or a tail
                        // page below).
                        if pos == "force" || pos == "here" {
                            pos = "top".to_string();
                            reason.push_str("; in-flow pin illegal on wide floats: floats top");
                        }
                    }
                    let stack_note = if stacked { ", stacked estimate/SE cells" } else { "" };
                    match level {
                        TableLevel::Fit => {
                            if stacked {
                                reason.push_str(&format!(
                                    "; stacked estimate/SE cells (worst {raw_worst:.2}cm -> {worst:.2}cm): fits portrait at full size"
                                ));
                            }
                        }
                        TableLevel::Compact => reason.push_str(&format!(
                            "; portrait is tight (worst token {worst:.2}cm vs {share:.2}cm share{stack_note}): weaver compacts to footnotesize + 3pt padding"
                        )),
                        TableLevel::Rotate => {
                            pos = "page".to_string();
                            reason.push_str(&format!(
                                "; portrait cannot hold it compacted (worst token {worst:.2}cm vs {share:.2}cm share{stack_note}): weaver rotates (sidewaystable)"
                            ));
                        }
                    }
                    // Knob off but the ladder would fire: name the opt-in.
                    if !on && adv_level != TableLevel::Fit {
                        let adv_share = (if adv_wide { p.textwidth_cm } else { p.column_cm })
                            / ncols as f64;
                        let adv_worst = worst_stacked_cm(&adv_out);
                        let adv_stacked = adv_worst < raw_worst;
                        let adv_kind = match adv_level {
                            TableLevel::Compact => "compact to footnotesize",
                            _ => "rotate (sidewaystable)",
                        };
                        reason.push_str(&format!(
                            "; portrait cannot hold it (worst token {adv_worst:.2}cm vs {adv_share:.2}cm share{}): set [tables] compact_tables = true to {adv_kind} automatically",
                            if adv_stacked { ", stacked estimate/SE cells" } else { "" }
                        ));
                    }
                }
                tail_page(p, &mut span, &mut pos, &mut reason, total_prose, seen_prose);
                reason.push_str(&late_note(p, &span, prose_lines));
                reason.push_str(&queue_note(&span, &mut wides));
                prose_lines = 0;
                let action = if cur_span == span && cur_pos == pos { "keep" } else { "change" };
                out.push(Suggestion {
                    index: table_idx,
                    kind: "table".to_string(),
                    key: format!("table-{table_idx}"),
                    line: dline,
                    est_w_cm: fmt2(w1),
                    est_h_cm: fmt2(h1),
                    span,
                    pos,
                    width,
                    action: action.to_string(),
                    reason,
                });
                table_idx += 1;
            }
            _ => {}
        }
    }
    // Plots ride the caller's hook (`plot_insert_before`), not md
    // fences: measured here, applied by the caller via `plot_attrs`.
    // The registry second element decides the kind: preset JSON
    // (`kind: bar|line`) measures like a diagram; anything else is
    // an external PDF (the showcase shape) and keeps hand attrs.
    for (key, preset_src) in &spec.plots {
        let is_preset = serde_json::from_str::<serde_json::Value>(preset_src)
            .ok()
            .and_then(|v| v.get("kind").and_then(|k| k.as_str()).map(|s| s.to_string()))
            .map(|k| k == "bar" || k == "line")
            .unwrap_or(false);
        let kind = if is_preset { "plot" } else { "picture" };
        let (w1, h1) = if is_preset { measure_plot(preset_src) } else { (0.0, 0.0) };
        let cur = spec.plot_attrs.get(key).map(|s| s.as_str()).unwrap_or("");
        let (cur_span, cur_pos, cur_width) = parse_loose_attrs(cur);
        let (span, pos, width, reason) = if is_preset {
            suggest_for(p, "plot", w1, h1, &cur_span, &cur_pos, false, 0.0)
        } else {
            // External plot PDF: same keep-rule as pictures, but the
            // caller applies it through `plot_attrs`, not fences.
            (
                cur_span.clone(),
                cur_pos.clone(),
                cur_width,
                "external plot PDF: size unknown, caller-owned plot_attrs stand".to_string(),
            )
        };
        let same_width = match (cur_width, width) {
            (None, None) => true,
            (Some(a), Some(w)) => (a - w).abs() < 0.005,
            _ => false,
        };
        let action = if cur_span == span && cur_pos == pos && same_width { "keep" } else { "change" };
        out.push(Suggestion {
            index: out.len(),
            kind: kind.to_string(),
            key: key.clone(),
            line: 0,
            est_w_cm: fmt2(w1),
            est_h_cm: fmt2(h1),
            span,
            pos,
            width,
            action: action.to_string(),
            reason,
        });
    }
    Ok(out)
}

/// Parse a loose `{k=v}` attr fragment (plot attrs from the caller):
/// unknown keys ignored, missing keys become `"default"`/`None`.
fn parse_loose_attrs(cur: &str) -> (String, String, Option<f64>) {
    let mut span = "default".to_string();
    let mut pos = "default".to_string();
    let mut width: Option<f64> = None;
    for tok in cur.split_whitespace() {
        let t = tok.trim_matches(|c| c == ',' || c == '{' || c == '}');
        if let Some((k, v)) = t.split_once('=') {
            match k {
                "span" => span = v.to_string(),
                "pos" => pos = v.to_string(),
                "width" => width = v.parse::<f64>().ok(),
                _ => {}
            }
        }
    }
    (span, pos, width)
}

/// Spread-the-fences note: the third and later `wide` floats since
/// the section heading queue behind each other on scarce page tops
/// and pile toward the document end. Counts through `wides` (the
/// caller resets it per section); advisory like `late_note`.
fn queue_note(span: &str, wides: &mut usize) -> String {
    if span == "wide" {
        *wides += 1;
        if *wides >= 3 {
            return format!(
                "; wide float #{wides} since the section heading — page tops queue behind each other: spread the fences across sections (move above section prose)"
            );
        }
    }
    String::new()
}

/// Tail-page override: a `wide`+`top` float with fewer than
/// `tail_lines` body prose lines remaining takes a dedicated page.
/// No downstream tops are left (it would pile at the document end),
/// and a still-pending float meeting `\FloatBarrier` ejects the
/// column — the placeins eject strands a half-empty column behind
/// it, the exact void this rule prevents. Float pages never strand
/// columns: text flows past them. Skipped in prose-free manuscripts
/// (B1 stubs), where there is no discussing text to stay near.
fn tail_page(p: &LayoutPolicy, span: &mut String, pos: &mut String, reason: &mut String, total_prose: usize, seen_prose: usize) {
    if span == "wide" && pos == "top" && total_prose > 0 {
        let remaining = total_prose.saturating_sub(seen_prose);
        if remaining < p.tail_lines {
            *pos = "page".to_string();
            reason.push_str(&format!(
                "; tail float ({remaining} prose lines remain in the body): no downstream tops left -> dedicated page stays adjacent instead of piling at the end"
            ));
        }
    }
}

/// Move-the-fence note: a `wide` float declared after a wall of
/// prose strands a trailing page (page-top floats need a top at or
/// after their declaration). Advisory — the engine never reorders,
/// it only names the fix.
fn late_note(p: &LayoutPolicy, span: &str, prose_lines: usize) -> String {
    if span == "wide" && prose_lines > p.late_wide_lines {
        format!(
            "; declared {prose_lines} prose lines into the section — likely strands a trailing page: move the fence above the prose (right after the section heading)"
        )
    } else {
        String::new()
    }
}

/// JSON envelope for the Python side.
pub fn suggest_layout_json(md_text: &str, toml_src: &str, spec: &BuildSpec) -> Result<String, Error> {
    let suggs = suggest_layout(md_text, toml_src, spec)?;
    serde_json::to_string(&suggs).map_err(|e| Error::new(format!("layout json: {e}"), 1, String::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pol() -> LayoutPolicy {
        LayoutPolicy {
            textheight_cm: 23.0,
            column_cm: 8.9,
            textwidth_cm: 17.8,
            caption_cm: 1.0,
            here_max_cm: 7.0,
            page_frac: 0.6,
            min_width: 0.4,
            wide_ratio: 1.25,
            lead_bonus_cm: 1.0,
            late_wide_lines: 15,
            top_frac: 0.7,
            tail_lines: 10,
        }
    }

    #[test]
    fn scanner_reads_node_coords_not_ids() {
        let tikz = "\\node (A) at (0.0,0.0) {x};\n\\draw[->] (0.0,0.0) -- (4.2,-2.4) {};\n";
        let b = tikz_bbox(tikz).unwrap();
        assert_eq!(b, (0.0, -2.4, 4.2, 0.0), "got {b:?}");
    }

    #[test]
    fn scanner_skips_axis_cs_and_empty() {
        let tikz = "\\begin{axis}\n\\addplot coordinates {(axis cs:1,2)};\n\\end{axis}\n";
        assert!(tikz_bbox(tikz).is_none(), "data coords are not TikZ cm");
        assert!(tikz_bbox("no coords here").is_none());
    }

    #[test]
    fn diagram_measure_matches_known_grid() {
        // Fork on the 4.2/2.4 layered grid: siblings share a rank
        // (side by side), the chain drops one rank, plus margins.
        // One rank spans x=0..4.2 (coords), two ranks y=0..-2.4.
        let src = "graph TD\na[x]-->b[y]\na-->c[z]";
        let (w, h, _) = measure_diagram(src, 1).unwrap();
        assert!((w - (4.2 + 2.0 * TIKZ_MARGIN_X)).abs() < 0.05, "w={w}");
        assert!((h - (2.4 + 2.0 * TIKZ_MARGIN_Y)).abs() < 0.05, "h={h}");
    }

    #[test]
    fn table_measure_pins_numbers() {
        let rows = vec![
            "| A | B |".to_string(),
            "|--|--|".to_string(),
            "| `x_y` | 3 |".to_string(),
        ];
        let (w, h) = measure_table(&rows);
        // Raw cells keep backticks: col0 widest is "`x_y`" (5 chars),
        // col1 widest is 1 char -> (5+1)*0.11+0.6.
        assert!((w - (6.0 * TABLE_CHAR_CM + TABLE_CHROME_CM)).abs() < 1e-9, "w={w}");
        // header + one data row.
        assert!((h - (TABLE_HEAD_CM + 2.0 * TABLE_ROW_CM)).abs() < 1e-9, "h={h}");
    }

    #[test]
    fn short_table_pins_force() {
        let (span, pos, width, _) = suggest_for(&pol(), "table", 6.0, 4.0, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "force"), "short table pins");
        assert!(width.is_none());
    }

    #[test]
    fn late_wide_table_names_the_fix() {
        // Pure policy knows nothing of prose: the note is walk state.
        assert_eq!(late_note(&pol(), "wide", 20), "; declared 20 prose lines into the section — likely strands a trailing page: move the fence above the prose (right after the section heading)");
        assert!(late_note(&pol(), "wide", 3).is_empty());
        assert!(late_note(&pol(), "column", 20).is_empty());
    }

    #[test]
    fn wide_table_floats_top() {
        let (span, pos, _, reason) = suggest_for(&pol(), "table", 13.8, 5.0, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("wide", "top"));
        assert!(reason.contains("13.8"), "numbers in reason: {reason}");
    }

    #[test]
    fn tall_wide_table_takes_page_not_top() {
        // 24 rows ≈ 13.9cm + caption: fits the 16.1cm top budget.
        let (span, pos, _, _) = suggest_for(&pol(), "table", 14.0, 13.9, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("wide", "top"));
        // 41 rows ≈ 23.2cm + caption: [t] cannot hold it — [t]
        // would strand it (and its queue) at the document end.
        let (span, pos, _, reason) = suggest_for(&pol(), "table", 14.0, 23.2, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("wide", "page"));
        assert!(reason.contains("page-top budget"), "names the trap: {reason}");
    }

    #[test]
    fn overtall_column_table_takes_page_not_force() {
        // Taller than the text height: [H] would blast through.
        let (span, pos, _, reason) = suggest_for(&pol(), "table", 6.0, 24.0, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "page"));
        assert!(reason.contains("[H]"), "names the breakage: {reason}");
        // A normal tall-but-fitting column table still pins.
        let (span, pos, _, _) = suggest_for(&pol(), "table", 6.0, 12.0, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "force"));
    }

    #[test]
    fn lead_wide_tall_diagram_escapes_to_page() {
        // Lead 30cm-wide, 28cm-tall diagram: pre-empt skips leads,
        // the width escape shrinks to 0.58 but 17.2cm still exceeds
        // the top budget — dedicated page, not an unholdable top.
        let (span, pos, _, reason) =
            suggest_for(&pol(), "diagram", 30.0, 28.0, "default", "default", true, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("wide", "page"));
        assert!(reason.contains("page-top budget"), "named: {reason}");
    }

    #[test]
    fn tail_page_overrides_top_only_near_body_end() {
        let mut span = "wide".to_string();
        let mut pos = "top".to_string();
        let mut reason = String::from("overflows, floats top");
        // 9 of 100 prose lines seen: plenty of tops ahead, untouched.
        tail_page(&pol(), &mut span, &mut pos, &mut reason, 100, 9);
        assert_eq!(pos.as_str(), "top");
        // 95 of 100 seen: no downstream tops left -> page.
        tail_page(&pol(), &mut span, &mut pos, &mut reason, 100, 95);
        assert_eq!(pos.as_str(), "page");
        assert!(reason.contains("tail float"), "named: {reason}");
        // Column floats never eject: untouched at the tail.
        let mut span = "column".to_string();
        let mut pos = "force".to_string();
        let mut reason = String::new();
        tail_page(&pol(), &mut span, &mut pos, &mut reason, 100, 99);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "force"));
        // Prose-free manuscripts (B1 stubs): no adjacency to
        // protect, pins stay exact.
        let mut span = "wide".to_string();
        let mut pos = "top".to_string();
        let mut reason = String::new();
        tail_page(&pol(), &mut span, &mut pos, &mut reason, 0, 0);
        assert_eq!(pos.as_str(), "top");
    }

    #[test]
    fn queue_note_fires_on_third_wide() {
        let mut wides = 0usize;
        assert!(queue_note("column", &mut wides).is_empty());
        assert_eq!(wides, 0, "non-wide does not count");
        assert!(queue_note("wide", &mut wides).is_empty());
        assert!(queue_note("wide", &mut wides).is_empty());
        let third = queue_note("wide", &mut wides);
        assert!(third.contains("#3"), "counts: {third}");
        assert!(third.contains("spread the fences"), "names the fix: {third}");
    }

    #[test]
    fn tall_diagram_takes_page() {
        let (span, pos, width, _) = suggest_for(&pol(), "diagram", 5.0, 16.0, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "page"));
        assert!(width.is_none(), "natural width fits the column");
    }

    #[test]
    fn short_diagram_shrinks_to_here() {
        // 17cm-wide pipeline: exact fit with headroom, height fits.
        let (span, pos, width, _) = suggest_for(&pol(), "diagram", 17.0, 5.0, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "here"));
        assert_eq!(width, Some(0.51), "0.98*8.9/17 to 2dp");
    }

    #[test]
    fn plot_pins_force_at_fit_width() {
        let (span, pos, width, _) = suggest_for(&pol(), "plot", PLOT_W1, PLOT_H1, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "force"));
        assert!(width.is_some(), "natural 9.5cm overflows the column");
    }

    #[test]
    fn picture_keeps_hand_attrs() {
        let (span, pos, width, reason) = suggest_for(&pol(), "picture", 0.0, 0.0, "wide", "top", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("wide", "top"));
        assert!(width.is_none());
        assert!(reason.contains("unknown"), "got: {reason}");
    }

    #[test]
    fn node_labels_parse_placed_only() {
        let tikz = "  \\node (A) at (0.0,0.0) [box] {Caller};\n  \\node (B) at (3.5,0.0) [box] {TeX engine};\n  \\draw[->] (0.0,0.0) -- (3.5,0.0) \\node[midway,above] {go};\n";
        let nodes = node_labels(tikz);
        assert_eq!(nodes.len(), 2, "edge label excluded, got {nodes:?}");
        assert_eq!(nodes[0], (0.0, 0.0, 6));
        assert_eq!(nodes[1], (3.5, 0.0, 10));
    }

    #[test]
    fn crowd_floor_math() {
        // (9+10)/2 chars * 0.21 / 3.5 = 0.57: the showcase pair.
        let f = crowd_floor(&[(0.0, 0.0, 9), (3.5, 0.0, 10)]);
        assert!((f - 0.57).abs() < 0.015, "f={f}");
        // Vertical stacks never collide.
        assert_eq!(crowd_floor(&[(0.0, 0.0, 20), (0.0, -2.4, 20)]), 0.0);
        // Far ranks excluded.
        assert_eq!(crowd_floor(&[(0.0, 0.0, 30), (8.4, 0.0, 30)]), 0.0);
    }

    #[test]
    fn floor_binds_over_fit() {
        // 17cm content, fit 0.51, floor 0.57: floor wins under the cap.
        let (span, pos, width, reason) =
            suggest_for(&pol(), "diagram", 17.0, 5.0, "default", "default", false, 0.57);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "here"));
        assert_eq!(width, Some(0.57), "floor binds: {reason}");
        assert!(reason.contains("floor 0.57"), "named: {reason}");
    }

    #[test]
    fn floor_capped_with_warning() {
        // Floor 0.90 exceeds 1.15x fit (0.59): fit wins, flagged.
        let (_, _, width, reason) =
            suggest_for(&pol(), "diagram", 17.0, 5.0, "default", "default", false, 0.90);
        assert_eq!(width, Some(0.59), "capped: {reason}");
        assert!(reason.contains("crowding risk remains"), "flagged: {reason}");
    }

    #[test]
    fn oversize_diagram_escapes_wide() {
        let (span, pos, _, _) = suggest_for(&pol(), "diagram", 30.0, 4.0, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("wide", "top"));
    }

    #[test]
    fn page_tall_preempts_before_shrink() {
        // 30cm wide AND 20cm tall: the wide escape would float it top,
        // but page-tall content takes a dedicated page before shrinking.
        let (span, pos, width, reason) =
            suggest_for(&pol(), "diagram", 30.0, 20.0, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "page"));
        assert!(width.is_none(), "pre-empt keeps natural size: {width:?}");
        assert!(reason.contains("pre-empt"), "named: {reason}");
        // Section leads stay exempt (fresh-column odds decide below).
        let (_, pos_lead, _, _) =
            suggest_for(&pol(), "diagram", 5.4, 13.0, "default", "default", true, 0.0);
        assert_eq!(pos_lead.as_str(), "here");
    }

    #[test]
    fn section_lead_gets_inflow_bonus() {
        // 13cm content: non-lead trips the tall pre-empt (14 > 13.8)
        // and shrinks past the floor; the lead fits on the bonus.
        let (_, pos_lead, w_lead, _) = suggest_for(&pol(), "diagram", 5.4, 13.0, "default", "default", true, 0.0);
        let (_, pos_plain, w_plain, _) = suggest_for(&pol(), "diagram", 5.4, 13.0, "default", "default", false, 0.0);
        assert_eq!(pos_lead.as_str(), "here", "lead stays in-flow");
        assert!(w_lead.is_none(), "lead never shrinks for height: {w_lead:?}");
        assert_eq!(pos_plain.as_str(), "page", "non-lead exiled");
        assert!(w_plain.is_none(), "page floats keep natural size: {w_plain:?}");

        // Narrow + short non-lead: in-flow, natural.
        let (span, pos, width, _) = suggest_for(&pol(), "diagram", 5.0, 3.0, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "here"));
        assert!(width.is_none());
    }

    #[test]
    fn wide_ratio_forgives_slight_overflow() {
        // 9.5cm est in an 8.9cm column: wraps fine, stays column.
        let (span, pos, _, _) = suggest_for(&pol(), "table", 9.5, 3.5, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("column", "force"));
        // 13.1cm est is clearly wider: earns the full width.
        let (span, pos, _, _) = suggest_for(&pol(), "table", 13.1, 3.5, "default", "default", false, 0.0);
        assert_eq!((span.as_str(), pos.as_str()), ("wide", "top"));
    }
}
