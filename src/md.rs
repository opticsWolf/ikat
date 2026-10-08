//! Markdown block splitter (E.1 / M5.4; line numbers added F.1):
//! pulldown-cmark owns block BOUNDARIES; every content rule is
//! applied to raw source slices (byte offsets). Inline events are
//! IGNORED — the slice is the truth.
//!
//! Every block carries its 1-based ORIGINAL manuscript line
//! (`prepass` deletions shift stripped lines, so a line map rides
//! along). Fence/table inner lines resolve as `block.line + row`
//! (fence and table bodies are consecutive original lines).
//! Paragraph lines may be non-consecutive (a deleted quote between
//! them); para errors report the first line with the joined text
//! as echo.
//!
//! Mapping (each justified by the `md_spike` parity harness):
//! - `>` quote lines: pre-deleted (the hand scanner dropped them
//!   without even breaking paragraphs — deletion replicates that).
//! - `%% table`-before-table: pre-extracted to
//!   `(table_seq, attrs, orig_line)`; `Directive` blocks splice
//!   positionally before their table. A dangling `%% table` stays
//!   paragraph text, as before.
//! - Link reference definitions `[label]: ...` are a hard error
//!   naming the line (the stream would swallow them silently).
//! - ATX `#`/`##`/`###` (+ `## Appendix`/`## Abstract` prefixes):
//!   headings as before. Setext, H4+, `#NoSpace`: raw lines as
//!   paragraph text (closed `## X ##` keeps its closers).
//! - Backtick fences: raw info line + raw body lines (pulldown
//!   preserves both verbatim — probed). `~~~` and indented code:
//!   raw lines as paragraph text.
//! - Tables: raw slice rows (delimiter included). `|`-runs
//!   pulldown rejects are re-scanned out of paragraph slices.
//! - Paragraphs break ONLY on blank lines (stripped-index
//!   arithmetic). Titles never flush; last wins.

use pulldown_cmark::{Event, Options, Parser, Tag};

use crate::error::Error;

/// A block with its 1-based original manuscript line.
#[derive(Debug, PartialEq)]
pub struct MdBlock {
    pub line: usize,
    pub kind: MdKind,
}

/// Block stream `convert()` consumes.
#[derive(Debug, PartialEq)]
pub enum MdKind {
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
/// stripped lines, their 1-based original line numbers, and
/// `(table_seq, attrs, orig_line)` directives.
pub fn prepass(md: &str) -> Result<(Vec<String>, Vec<usize>, Vec<(usize, String, usize)>), Error> {
    let lines: Vec<&str> = md.lines().collect();
    let mut kept: Vec<bool> = vec![true; lines.len()];
    let mut directives: Vec<(usize, String, usize)> = Vec::new();
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
                directives.push((runs, crate::doc::fence_attr_str(s).to_string(), n + 1));
            }
            continue;
        }
        // Link reference definition (`[label]: dest`, not `[^..]:`):
        // the event stream swallows these with no event — the text
        // would vanish. Error loudly instead, naming the line.
        if !s.starts_with("[^") && s.starts_with('[') {
            if let Some(col) = s.find("]:") {
                if !s[1..col].contains([' ', '\n', '[']) {
                    return Err(Error::new(
                        "reference definitions `[label]: ...` are not in the subset (write the text inline)",
                        n + 1,
                        s.to_string(),
                    ));
                }
            }
        }
    }
    let mut stripped = Vec::new();
    let mut orig = Vec::new();
    for (n, l) in lines.iter().enumerate() {
        if kept[n] {
            stripped.push(l.to_string());
            orig.push(n + 1);
        }
    }
    Ok((stripped, orig, directives))
}

/// Split `md` into blocks per the mapping above.
pub fn blocks(md: &str) -> Result<Vec<MdBlock>, Error> {
    let (stripped, orig, directives) = prepass(md)?;
    let text = stripped.join("\n");
    // Byte offset of each stripped line's start.
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
    let is_blank = |k: usize| -> bool {
        stripped.get(k).map(|l| l.trim().is_empty()).unwrap_or(false)
    };

    let mut out: Vec<MdBlock> = Vec::new();
    let mut para: Vec<String> = Vec::new();
    // Stripped index of the last line that fed the pending para.
    let mut para_tail: usize = 0;
    // Original line of the pending para's first fed line.
    let mut para_line: usize = 0;

    let flush = |out: &mut Vec<MdBlock>, para: &mut Vec<String>, pline: &mut usize| {
        if !para.is_empty() {
            out.push(MdBlock { line: *pline, kind: MdKind::Para(std::mem::take(para)) });
        }
    };
    // Feed `(stripped index, text)` lines into the pending para,
    // flushing first when a truly blank stripped line lies between
    // the last fed line and this run's first line.
    let feed = |out: &mut Vec<MdBlock>,
                para: &mut Vec<String>,
                tail: &mut usize,
                pline: &mut usize,
                run: &[(usize, String)]| {
        if run.is_empty() {
            return;
        }
        if !para.is_empty() && (tail.saturating_add(1)..run[0].0).any(|k| is_blank(k)) {
            out.push(MdBlock { line: *pline, kind: MdKind::Para(std::mem::take(para)) });
        }
        for (k, l) in run {
            let t = l.trim();
            if !t.is_empty() {
                if para.is_empty() {
                    *pline = orig.get(*k).copied().unwrap_or(1);
                }
                para.push(t.to_string());
                *tail = *k;
            }
        }
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
        // The span covers contiguous stripped lines.
        let base = line_of(s);
        let cover: Vec<(usize, String)> = text[s..e]
            .split('\n')
            .enumerate()
            .map(|(k, l)| (base + k, l.to_string()))
            .collect();
        // 1-based original line of the span's first line.
        let ln = orig.get(base).copied().unwrap_or(1);
        match kind {
            Kind::Other => {
                feed(&mut out, &mut para, &mut para_tail, &mut para_line, &cover);
            }
            Kind::Para => {
                // Re-scan for `|`-runs the hand scanner would table.
                // Segments carry their stripped indices so para
                // lines keep exact originals.
                let mut seg: Vec<(usize, String)> = Vec::new();
                let mut run: Vec<(usize, String)> = Vec::new();
                let mut runs: Vec<(Vec<(usize, String)>, Vec<(usize, String)>)> = Vec::new();
                for (k, l) in cover {
                    if l.trim().starts_with('|') {
                        if !seg.is_empty() {
                            runs.push((std::mem::take(&mut seg), Vec::new()));
                        }
                        run.push((k, l.trim().to_string()));
                    } else {
                        if !run.is_empty() {
                            runs.push((Vec::new(), std::mem::take(&mut run)));
                        }
                        seg.push((k, l));
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
                        flush(&mut out, &mut para, &mut para_line);
                        // Table line: original of its first row.
                        let tline = orig.get(tablerun[0].0).copied().unwrap_or(ln);
                        let rows = tablerun.into_iter().map(|(_, r)| r).collect();
                        out.push(MdBlock { line: tline, kind: MdKind::Table(rows) });
                    } else {
                        feed(&mut out, &mut para, &mut para_tail, &mut para_line, &textseg);
                    }
                }
            }
            Kind::Heading => {
                let first = cover[0].1.trim().to_string();
                if let Some(rest) = first.strip_prefix("### ") {
                    flush(&mut out, &mut para, &mut para_line);
                    out.push(MdBlock { line: ln, kind: MdKind::H3(rest.to_string()) });
                } else if let Some(rest) = first.strip_prefix("## ") {
                    flush(&mut out, &mut para, &mut para_line);
                    if rest.strip_prefix("Appendix").is_some() {
                        out.push(MdBlock { line: ln, kind: MdKind::Appendix });
                    } else if rest.starts_with("Abstract") {
                        out.push(MdBlock { line: ln, kind: MdKind::Abstract });
                    } else {
                        out.push(MdBlock { line: ln, kind: MdKind::H2(rest.to_string()) });
                    }
                } else if let Some(rest) = first.strip_prefix("# ") {
                    if let Some(MdBlock { kind: MdKind::Title(_), .. }) = out.last() {
                        out.pop();
                    }
                    out.push(MdBlock { line: ln, kind: MdKind::Title(rest.to_string()) });
                } else {
                    feed(&mut out, &mut para, &mut para_tail, &mut para_line, &cover);
                }
            }
            Kind::Code => {
                let first = cover[0].1.trim().to_string();
                if first.starts_with("```") {
                    flush(&mut out, &mut para, &mut para_line);
                    let mut body = Vec::new();
                    for (_, l) in cover.iter().skip(1) {
                        let t = l.trim();
                        if !t.is_empty() && t.chars().all(|c| c == '`') {
                            break;
                        }
                        body.push(l.to_string());
                    }
                    out.push(MdBlock {
                        line: ln,
                        kind: MdKind::Fence { info: first, body: body.join("\n") },
                    });
                } else {
                    feed(&mut out, &mut para, &mut para_tail, &mut para_line, &cover);
                }
            }
            Kind::Table => {
                flush(&mut out, &mut para, &mut para_line);
                let mut rows: Vec<String> =
                    cover.iter().map(|(_, l)| l.trim().to_string()).collect();
                while rows.last().map(|l| l.is_empty()).unwrap_or(false) {
                    rows.pop();
                }
                out.push(MdBlock { line: ln, kind: MdKind::Table(rows) });
            }
        }
    }
    flush(&mut out, &mut para, &mut para_line);

    let mut sorted = directives;
    sorted.sort_by_key(|(seq, _, _)| *seq);
    let mut diri = 0usize;
    let mut table_idx = 0usize;
    let mut merged: Vec<MdBlock> = Vec::with_capacity(out.len() + sorted.len());
    for b in out {
        if matches!(b.kind, MdKind::Table(_)) {
            while diri < sorted.len() && sorted[diri].0 == table_idx {
                merged.push(MdBlock {
                    line: sorted[diri].2,
                    kind: MdKind::Directive(sorted[diri].1.clone()),
                });
                diri += 1;
            }
            table_idx += 1;
        }
        merged.push(b);
    }
    Ok(merged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_refdef_names_line() {
        let e = blocks("# T\n\n[lab]: /url\n").unwrap_err();
        assert!(e.msg.contains("reference definitions"), "got: {e}");
        assert_eq!(e.line, 3, "got: {e}");
        assert!(e.echo.contains("[lab]: /url"), "got: {e}");
    }

    #[test]
    fn blocks_carry_original_lines() {
        // Quote + directive lines vanish; the survivors keep
        // original numbers (title 1, section 5, table 7).
        let md = "# T\n\n> dropped\n\n## S\n\n%% table {pos=bottom}\n\n| A |\n|---|\n";
        let bs = blocks(md).unwrap();
        let lines: Vec<usize> = bs.iter().map(|b| b.line).collect();
        assert!(lines.contains(&1), "title: {lines:?}");
        assert!(lines.contains(&5), "section: {lines:?}");
        assert!(lines.contains(&7), "directive orig line: {lines:?}");
        assert!(lines.contains(&9), "table orig line: {lines:?}");
    }
}
