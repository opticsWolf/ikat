//! Markdown block splitter (E.1 / M5.4): the hand line-scanner's
//! successor. pulldown-cmark owns block BOUNDARIES; every content
//! rule is applied to raw source slices (byte offsets), so inline
//! escaping, fence attrs, tables, and directives behave exactly as
//! before. Inline events are IGNORED — the slice is the truth.
//!
//! Mapping (each justified by the `md_spike` parity harness, which
//! diffs this module against the frozen hand rules on the full
//! corpus plus latent-divergence synthetics):
//! - `>` quote lines: pre-deleted (the hand scanner dropped them
//!   without even breaking paragraphs — deletion replicates that).
//! - `%% table`-before-table: pre-extracted to `(table_seq, attrs)`
//!   (fence-aware, same "next non-blank is `|`" rule); `Directive`
//!   blocks splice positionally before their table. A dangling
//!   `%% table` stays paragraph text, as before.
//! - Link reference definitions `[label]: ...` are a hard error:
//!   the event stream consumes them with no event at all, so the
//!   text would vanish silently. Loud beats silent; they were never
//!   in the subset.
//! - ATX `#`/`##`/`###` (+ `## Appendix`/`## Abstract` prefixes):
//!   headings as before. Setext, H4+, closed-ATX quirks: raw lines
//!   as paragraph text (the hand scanner never knew them; closed
//!   `## X ##` keeps its closers — same as hand's `s[3..]`).
//! - Backtick fences: raw info line + raw body lines (pulldown
//!   preserves both verbatim — probed). `~~~` and indented code:
//!   raw lines as paragraph text (hand's obliviousness, replicated).
//! - Tables: raw slice rows (delimiter included), same as the hand
//!   `|`-runs. `|`-runs pulldown rejects (lone `|`, no delimiter)
//!   are re-scanned out of paragraph slices and tabled — the hand
//!   scanner tabled ANY such run.
//! - Paragraphs break ONLY on blank lines (gap rule by line
//!   arithmetic — spans may swallow newlines, so counting `\n`s in
//!   the gap miscounts). Titles never flush; last `# ` wins.
//! - Anything else (lists, HTML blocks, thematic breaks): raw slice
//!   lines as paragraph text.

use pulldown_cmark::{Event, Options, Parser, Tag};

/// Block stream `convert()` consumes.
#[derive(Debug, PartialEq)]
pub enum MdBlock {
    Title(String),
    H2(String),
    H3(String),
    Appendix,
    Abstract,
    Fence { info: String, body: String },
    Table(Vec<String>),
    Directive(String),
    Para(Vec<String>),
}

/// Fence-aware pre-pass: drop `>` quote lines, extract
/// `%% table`-before-table directives keyed by the table sequence
/// they precede, reject link reference definitions. Returns the
/// stripped lines plus `(table_seq, attr_string)`.
pub fn prepass(md: &str) -> Result<(Vec<String>, Vec<(usize, String)>), String> {
    let lines: Vec<&str> = md.lines().collect();
    let mut kept: Vec<bool> = vec![true; lines.len()];
    let mut directives: Vec<(usize, String)> = Vec::new();
    let mut infence = false;
    for (n, l) in lines.iter().enumerate() {
        let s = l.trim();
        if s.starts_with("```") {
            infence = !infence;
            continue;
        }
        if infence {
            continue;
        }
        if s.starts_with('>') {
            kept[n] = false;
            continue;
        }
        if s.starts_with("%% table") {
            let mut j = n + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j < lines.len() && lines[j].trim().starts_with('|') {
                kept[n] = false;
                // Sequence number = count of `|`-runs in the kept
                // lines strictly before the target table's line.
                let mut runs = 0;
                let mut k = 0;
                while k < j {
                    if kept[k] && lines[k].trim().starts_with('|') {
                        runs += 1;
                        while k < j && kept[k] && lines[k].trim().starts_with('|') {
                            k += 1;
                        }
                    } else {
                        k += 1;
                    }
                }
                directives.push((runs, crate::doc::fence_attr_str(s).to_string()));
            }
            continue;
        }
        // Link reference definition (`[label]: dest`, not `[^..]:`):
        // the event stream swallows these with no event — the text
        // would vanish. Error loudly instead.
        if !s.starts_with("[^") && s.starts_with('[') {
            if let Some(col) = s.find("]:") {
                if !s[1..col].contains([' ', '\n', '[']) {
                    return Err(format!(
                        "line {}: reference definitions `[label]: ...` are not in the subset (write the text inline)",
                        n + 1
                    ));
                }
            }
        }
    }
    let stripped: Vec<String> = lines
        .iter()
        .enumerate()
        .filter(|(n, _)| kept[*n])
        .map(|(_, l)| l.to_string())
        .collect();
    Ok((stripped, directives))
}

/// Split `md` into blocks per the mapping above.
pub fn blocks(md: &str) -> Result<Vec<MdBlock>, String> {
    let (stripped, directives) = prepass(md)?;
    let text = stripped.join("\n");
    let mut line_off: Vec<usize> = Vec::with_capacity(stripped.len());
    let mut o = 0;
    for l in &stripped {
        line_off.push(o);
        o += l.len() + 1;
    }
    let line_of = |off: usize| -> usize {
        match line_off.binary_search(&off) {
            Ok(n) => n,
            Err(n) => n.saturating_sub(1),
        }
    };

    let mut out: Vec<MdBlock> = Vec::new();
    let mut para: Vec<String> = Vec::new();
    let mut para_tail: Option<usize> = None;

    let flush = |out: &mut Vec<MdBlock>, para: &mut Vec<String>, tail: &mut Option<usize>| {
        if !para.is_empty() {
            out.push(MdBlock::Para(std::mem::take(para)));
            *tail = None;
        }
    };
    // Feed raw text lines into the pending para, flushing first when
    // a truly blank stripped line lies between (titles and other
    // non-flushing blocks between text lines do NOT split them).
    let feed = |out: &mut Vec<MdBlock>,
                para: &mut Vec<String>,
                tail: &mut Option<usize>,
                slice_start: usize,
                slice_end: usize,
                raw_lines: &[String]| {
        if !para.is_empty() {
            let prev = line_of(tail.unwrap_or(0).saturating_sub(1));
            let first = line_of(slice_start);
            let blank = (prev + 1..first)
                .any(|k| stripped.get(k).map(|l| l.trim().is_empty()).unwrap_or(false));
            if blank {
                out.push(MdBlock::Para(std::mem::take(para)));
            }
        }
        for l in raw_lines {
            let t = l.trim();
            if !t.is_empty() {
                para.push(t.to_string());
            }
        }
        *tail = Some(slice_end);
    };

    enum Kind {
        Para,
        Heading,
        Code,
        Table,
        Other,
    }
    let mut stack: Vec<(Kind, usize)> = Vec::new();
    let mut spans: Vec<(Kind, usize, usize)> = Vec::new();
    for (ev, range) in Parser::new_ext(&text, Options::ENABLE_TABLES).into_offset_iter() {
        match ev {
            Event::Start(tag) => {
                let k = match tag {
                    Tag::Paragraph => Kind::Para,
                    Tag::Heading { .. } => Kind::Heading,
                    Tag::CodeBlock(_) => Kind::Code,
                    Tag::Table(_) => Kind::Table,
                    _ => Kind::Other,
                };
                stack.push((k, range.start));
            }
            Event::End(_) => {
                if let Some((k, s)) = stack.pop() {
                    if stack.is_empty() {
                        spans.push((k, s, range.end));
                    }
                }
            }
            _ => {}
        }
    }

    for (kind, s, e) in spans {
        let slice_lines: Vec<String> = text[s..e].split('\n').map(|l| l.to_string()).collect();
        match kind {
            Kind::Other => {
                // BlockQuote is pre-deleted; the rest (lists, HTML
                // blocks, thematic breaks) feeds raw lines as para
                // text — the hand scanner's obliviousness, kept.
                feed(&mut out, &mut para, &mut para_tail, s, e, &slice_lines);
            }
            Kind::Para => {
                // Re-scan for `|`-runs the hand scanner would table
                // (including shapes pulldown rejects).
                let mut seg: Vec<String> = Vec::new();
                let mut run: Vec<String> = Vec::new();
                let mut runs: Vec<(Vec<String>, Vec<String>)> = Vec::new();
                for l in slice_lines {
                    if l.trim().starts_with('|') {
                        if !seg.is_empty() {
                            runs.push((std::mem::take(&mut seg), Vec::new()));
                        }
                        run.push(l.trim().to_string());
                    } else {
                        if !run.is_empty() {
                            runs.push((Vec::new(), std::mem::take(&mut run)));
                        }
                        seg.push(l);
                    }
                }
                if !run.is_empty() {
                    runs.push((Vec::new(), run));
                }
                if !seg.is_empty() {
                    runs.push((seg, Vec::new()));
                }
                for (textseg, tablerun) in runs {
                    if !tablerun.is_empty() {
                        flush(&mut out, &mut para, &mut para_tail);
                        out.push(MdBlock::Table(tablerun));
                    } else {
                        feed(&mut out, &mut para, &mut para_tail, s, e, &textseg);
                    }
                }
            }
            Kind::Heading => {
                let first = slice_lines[0].trim().to_string();
                if let Some(rest) = first.strip_prefix("### ") {
                    flush(&mut out, &mut para, &mut para_tail);
                    out.push(MdBlock::H3(rest.to_string()));
                } else if let Some(rest) = first.strip_prefix("## ") {
                    flush(&mut out, &mut para, &mut para_tail);
                    if rest.strip_prefix("Appendix").is_some() {
                        out.push(MdBlock::Appendix);
                    } else if rest.starts_with("Abstract") {
                        out.push(MdBlock::Abstract);
                    } else {
                        out.push(MdBlock::H2(rest.to_string()));
                    }
                } else if let Some(rest) = first.strip_prefix("# ") {
                    // No flush (hand never flushed); last wins.
                    if let Some(MdBlock::Title(_)) = out.last() {
                        out.pop();
                    }
                    out.push(MdBlock::Title(rest.to_string()));
                } else {
                    // Setext, H4+, `#NoSpace`: raw lines as text.
                    feed(&mut out, &mut para, &mut para_tail, s, e, &slice_lines);
                }
            }
            Kind::Code => {
                let first = slice_lines[0].trim().to_string();
                if first.starts_with("```") {
                    flush(&mut out, &mut para, &mut para_tail);
                    let mut body = Vec::new();
                    for l in &slice_lines[1..] {
                        let t = l.trim();
                        if !t.is_empty() && t.chars().all(|c| c == '`') {
                            break;
                        }
                        body.push(l.to_string());
                    }
                    out.push(MdBlock::Fence { info: first, body: body.join("\n") });
                } else {
                    // `~~~` or indented code: raw lines as text.
                    feed(&mut out, &mut para, &mut para_tail, s, e, &slice_lines);
                }
            }
            Kind::Table => {
                flush(&mut out, &mut para, &mut para_tail);
                let mut rows: Vec<String> =
                    slice_lines.iter().map(|l| l.trim().to_string()).collect();
                while rows.last().map(|l| l.is_empty()).unwrap_or(false) {
                    rows.pop();
                }
                out.push(MdBlock::Table(rows));
            }
        }
    }
    flush(&mut out, &mut para, &mut para_tail);

    // Splice directives before their tables by sequence number.
    let mut sorted = directives;
    sorted.sort_by_key(|(seq, _)| *seq);
    let mut diri = 0usize;
    let mut table_idx = 0usize;
    let mut merged: Vec<MdBlock> = Vec::with_capacity(out.len() + sorted.len());
    for b in out {
        if matches!(b, MdBlock::Table(_)) {
            while diri < sorted.len() && sorted[diri].0 == table_idx {
                merged.push(MdBlock::Directive(sorted[diri].1.clone()));
                diri += 1;
            }
            table_idx += 1;
        }
        merged.push(b);
    }
    Ok(merged)
}
