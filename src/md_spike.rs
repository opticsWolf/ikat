//! Phase E parity harness (M5.4/E.1): the hand scanner's block rules
//! beside the candidate pulldown-cmark mapping, diffed on the REAL
//! corpus plus synthetic latent-divergence cases. Test-only module;
//! pulldown-cmark is still a dev-dependency here. When this harness
//! is green, E.1 moves the `pdc_*` logic into `convert()` and this
//! file stays as the permanent regression cross-check.
//!
//! Mapping principle (the whole bet): pulldown-cmark owns block
//! BOUNDARIES; every content rule stays hand code on raw source
//! slices (offsets), so inline escaping, fence attrs, tables, and
//! directives behave byte-identically. Inline events are IGNORED —
//! the slice is the truth. Constructs the hand scanner never knew
//! (lists, HTML blocks, setext, `~~~`, indented code, H4+) feed
//! their raw slice lines back as paragraph text — exactly what the
//! hand scanner did with them. Two deliberate exceptions, both
//! loud: `>` quote lines are pre-deleted (the hand scanner dropped
//! them without even breaking paragraphs — deletion replicates
//! that exactly), and link reference definitions are a hard error
//! (pulldown consumes them silently; silent text loss is worse
//! than a loud rejection, and they are outside the subset).

use pulldown_cmark::{Event, Options, Parser, Tag};

/// Block stream both parsers must agree on.
#[derive(Debug, PartialEq)]
enum Block {
    Title(String), // `# ` — last one wins, never flushes
    H2(String),    // `## ...` after `## ` stripped (pre-strip_section_number)
    H3(String),    // `### ...` after `### ` stripped
    Appendix,      // `## Appendix...` prefix
    Abstract,      // `## Abstract...` prefix
    Fence { lang: String, attr: String, body: String },
    Table(Vec<String>), // trimmed source rows, delimiter included
    Directive(String),   // `%% table ...` attr string (before a table)
    Para(String),        // trimmed lines joined with one space
}

/// Mirror of `convert()`'s line walk: blocks only, no spec/cfg.
/// Any drift between this and `convert()` invalidates the harness.
fn hand_parse(md: &str) -> Vec<Block> {
    let lines: Vec<&str> = md.lines().collect();
    let mut out = Vec::new();
    let mut para: Vec<String> = Vec::new();
    let mut i = 0;
    let flush = |out: &mut Vec<Block>, para: &mut Vec<String>| {
        if !para.is_empty() {
            out.push(Block::Para(para.join(" ")));
            para.clear();
        }
    };
    while i < lines.len() {
        let s = lines[i].trim();
        if s.is_empty() {
            flush(&mut out, &mut para);
            i += 1;
            continue;
        }
        if s.starts_with('>') {
            i += 1;
            continue;
        }
        if s.starts_with("```") {
            flush(&mut out, &mut para);
            let info = s.trim_start_matches('`');
            let mut sp = info.splitn(2, char::is_whitespace);
            let (lang, attr) = (
                sp.next().unwrap_or("").to_string(),
                sp.next().unwrap_or("").trim().to_string(),
            );
            let mut body = Vec::new();
            let mut j = i + 1;
            while j < lines.len() && !lines[j].trim().starts_with("```") {
                body.push(lines[j].to_string());
                j += 1;
            }
            out.push(Block::Fence { lang, attr, body: body.join("\n") });
            i = j + 1;
            continue;
        }
        if s.starts_with("%% table") {
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j < lines.len() && lines[j].trim().starts_with('|') {
                out.push(Block::Directive(attr_str(s)));
                i = j;
                continue;
            }
        }
        if s.starts_with('|') {
            flush(&mut out, &mut para);
            let mut rows = Vec::new();
            while i < lines.len() && lines[i].trim().starts_with('|') {
                rows.push(lines[i].trim().to_string());
                i += 1;
            }
            out.push(Block::Table(rows));
            continue;
        }
        if s.starts_with("## Appendix") {
            flush(&mut out, &mut para);
            out.push(Block::Appendix);
            i += 1;
            continue;
        }
        if s.starts_with("## Abstract") {
            flush(&mut out, &mut para);
            out.push(Block::Abstract);
            i += 1;
            continue;
        }
        if s.starts_with("## ") {
            flush(&mut out, &mut para);
            out.push(Block::H2(s[3..].to_string()));
            i += 1;
            continue;
        }
        if s.starts_with("### ") {
            flush(&mut out, &mut para);
            out.push(Block::H3(s[4..].to_string()));
            i += 1;
            continue;
        }
        if s.starts_with("# ") {
            if let Some(Block::Title(_)) = out.last() {
                out.pop();
            }
            // Hand keeps every `# ` but only the last survives as
            // `title`; earlier ones vanish. Record just the last by
            // replacing — the harness models the observable stream.
            out.push(Block::Title(s[2..].to_string()));
            i += 1;
            continue;
        }
        para.push(s.to_string());
        i += 1;
    }
    flush(&mut out, &mut para);
    out
}

/// The `{...}` slice of a fence info / directive line (same as
/// `fence_attr_str` in `doc.rs`).
fn attr_str(line: &str) -> String {
    match (line.find('{'), line.rfind('}')) {
        (Some(a), Some(b)) if b > a => line[a + 1..b].to_string(),
        _ => String::new(),
    }
}

/// Pre-pass over source lines (fence-aware). Drops `>` quote lines,
/// extracts `%% table`-before-table directives keyed by the table
/// sequence number they precede, and rejects link reference
/// definitions (which the event stream would swallow silently).
/// Returns the stripped line vector plus `(table_seq, attrs)`.
fn prepass(md: &str) -> Result<(Vec<String>, Vec<(usize, String)>), String> {
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
                // lines before the table this directive precedes.
                // Runs are counted below on the stripped output, so
                // record the directive's target run by scanning the
                // kept prefix: count runs strictly before line j.
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
                directives.push((runs, attr_str(s)));
            }
            continue;
        }
        // Link reference definition: `[label]: destination`. The
        // event stream consumes these with no event at all, so the
        // text would vanish. Loud error beats silent loss.
        // `[^...]:` footnote-looking lines are NOT link defs (and
        // footnotes stay disabled, so both sides keep them as text).
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

/// Candidate driver mapping: pre-pass, then pulldown boundaries with
/// raw-slice content. Must equal `hand_parse` on every harness input.
fn pdc_parse(md: &str) -> Result<Vec<Block>, String> {
    let (stripped, directives) = prepass(md)?;
    let text = stripped.join("\n");
    // Byte offset of each stripped line's start (for the blank-line
    // gap rule: spans may or may not swallow their trailing
    // newline, so newline-counting the gap miscounts — line
    // arithmetic does not).
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
    let mut out: Vec<Block> = Vec::new();
    let mut para: Vec<String> = Vec::new();
    // End offset of the last slice that fed the pending para (for
    // the blank-line gap rule).
    let mut para_tail: Option<usize> = None;

    let mut flush = |out: &mut Vec<Block>, para: &mut Vec<String>, tail: &mut Option<usize>| {
        if !para.is_empty() {
            out.push(Block::Para(para.join(" ")));
            para.clear();
        }
        *tail = None;
    };
    // Feed raw text lines into the pending para, flushing first when
    // a blank line separates them from what came before.
    let mut feed = |out: &mut Vec<Block>,
                    para: &mut Vec<String>,
                    tail: &mut Option<usize>,
                    slice_start: usize,
                    slice_end: usize,
                    raw_lines: &[String]| {
        if !para.is_empty() {
            // The hand scanner breaks paragraphs ONLY on blank
            // lines: a title (or any non-flushing block) between
            // two text lines does not split them. Flush iff a truly
            // blank stripped line lies between the last para line
            // and this slice's first line.
            let prev = line_of(tail.unwrap_or(0).saturating_sub(1));
            let first = line_of(slice_start);
            let blank = (prev + 1..first).any(|k| stripped.get(k).map(|l| l.trim().is_empty()).unwrap_or(false));
            if blank {
                out.push(Block::Para(para.join(" ")));
                para.clear();
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

    // Depth-0 block spans: (kind, start, end). Nested events are
    // inside their outer slice and never surface individually.
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
                // Nested blocks (table head/rows, list items) fold
                // into the outer slice — but an `Other` INSIDE a
                // text block (can't happen at depth 0 for our
                // option set) is still tracked uniformly.
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
                // BlockQuote is pre-deleted, so any `Other` here is
                // a list, an HTML block, or a thematic break: raw
                // lines as para text, exactly like the hand scanner
                // did. (Setext/H4+ arrive as Heading events — the
                // non-`#` arm below feeds them the same way.)
                feed(&mut out, &mut para, &mut para_tail, s, e, &slice_lines);
            }
            Kind::Para => {
                // Re-scan for `|`-runs: the hand scanner tables ANY
                // such run, including shapes pulldown rejects (lone
                // `|`, missing delimiter).
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
                        // Directives splice positionally at the end
                        // (by table sequence number); nothing to do
                        // here but count the table. tables_seen is
                        // folded into the final splice pass.
                        out.push(Block::Table(tablerun));
                    } else {
                        feed(&mut out, &mut para, &mut para_tail, s, e, &textseg);
                    }
                }
            }
            Kind::Heading => {
                let first = slice_lines[0].trim().to_string();
                if let Some(rest) = first.strip_prefix("### ") {
                    flush(&mut out, &mut para, &mut para_tail);
                    out.push(Block::H3(rest.to_string()));
                } else if let Some(rest) = first.strip_prefix("## ") {
                    flush(&mut out, &mut para, &mut para_tail);
                    if let Some(t) = rest.strip_prefix("Appendix") {
                        let _ = t;
                        out.push(Block::Appendix);
                    } else if rest.starts_with("Abstract") {
                        out.push(Block::Abstract);
                    } else {
                        out.push(Block::H2(rest.to_string()));
                    }
                } else if let Some(rest) = first.strip_prefix("# ") {
                    // Title: no flush (hand never flushed here). The
                    // hand scanner overwrites `title`, so only the
                    // last survives — model that by replacing an
                    // immediately preceding Title.
                    if let Some(Block::Title(_)) = out.last() {
                        out.pop();
                    }
                    out.push(Block::Title(rest.to_string()));
                } else {
                    // Setext, closed-ATX-without-space, H4+: raw
                    // lines as para text (hand never knew them).
                    // Closed ATX (`## X ##`) DOES start with `## `
                    // and is handled above — kept verbatim there.
                    feed(&mut out, &mut para, &mut para_tail, s, e, &slice_lines);
                }
            }
            Kind::Code => {
                // Same trimmed info line the hand scanner split.
                let first = slice_lines[0].trim().to_string();
                if first.starts_with("```") {
                    flush(&mut out, &mut para, &mut para_tail);
                    let info = first.trim_start_matches('`');
                    let mut sp = info.splitn(2, char::is_whitespace);
                    let (lang, attr) = (
                        sp.next().unwrap_or("").to_string(),
                        sp.next().unwrap_or("").trim().to_string(),
                    );
                    // Content lines are RAW: pulldown preserves them
                    // verbatim (probed: indents, tabs intact) exactly
                    // like the hand scanner's fence loop. The span
                    // covers the closing fence when present — stop
                    // at the all-backtick line, same as hand.
                    let mut body = Vec::new();
                    for l in &slice_lines[1..] {
                        let t = l.trim();
                        if !t.is_empty() && t.chars().all(|c| c == '`') {
                            break;
                        }
                        body.push(l.to_string());
                    }
                    out.push(Block::Fence { lang, attr, body: body.join("\n") });
                } else {
                    // `~~~` or indented code: raw lines as para text.
                    feed(&mut out, &mut para, &mut para_tail, s, e, &slice_lines);
                }
            }
            Kind::Table => {
                flush(&mut out, &mut para, &mut para_tail);
                let mut rows: Vec<String> =
                    slice_lines.iter().map(|l| l.trim().to_string()).collect();
                // The span's end range covers the trailing newline;
                // hand runs stop at the last `|` line, so drop the
                // phantom empties (trailing only — an interior blank
                // would split the hand run too, and GFM ends tables
                // at blanks, so it cannot occur here).
                while rows.last().map(|l| l.is_empty()).unwrap_or(false) {
                    rows.pop();
                }
                out.push(Block::Table(rows));
            }
        }
    }
    flush(&mut out, &mut para, &mut para_tail);

    // Splice directives before their tables by sequence number.
    let mut table_idx = 0usize;
    let mut diri = 0usize;
    let mut sorted = directives.clone();
    sorted.sort_by_key(|(seq, _)| *seq);
    let mut merged: Vec<Block> = Vec::with_capacity(out.len() + sorted.len());
    for b in out {
        if matches!(b, Block::Table(_)) {
            while diri < sorted.len() && sorted[diri].0 == table_idx {
                merged.push(Block::Directive(sorted[diri].1.clone()));
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

    /// Representative manuscript: every block shape `convert()` handles.
    const SAMPLE: &str = "# Title Here\n\
        \n\
        ## Abstract\n\
        \n\
        Abs line one.\n\
        Abs line two.\n\
        \n\
        \n\
        ## 1. First\n\
        \n\
        Para with math $x$ and cite [`k`] inline.\n\
        \n\
        ```mermaid {span=column pos=both}\n\
        graph TD\n\
        %% caption: Cap one.\n\
        a[x]-->b[y]\n\
        ```\n\
        \n\
        %% table {pos=barrier}\n\
        \n\
        | A | B |\n\
        |---|---|\n\
        | 1 | 2 |\n\
        | 3 | 4 |\n\
        \n\
        ### 1.1 Sub\n\
        \n\
        ```mermaid {inline}\n\
        graph TD\n\
        c[x]-->d[y]\n\
        ```\n\
        \n\
        ## Appendix\n\
        \n\
        Tail.\n";

    #[test]
    fn sample_parity() {
        let hand = hand_parse(SAMPLE);
        let pdc = pdc_parse(SAMPLE).unwrap();
        assert_eq!(hand, pdc);
    }

    fn corpus_path(name: &str) -> Option<String> {
        let base = env!("CARGO_MANIFEST_DIR");
        let p = std::path::Path::new(base).join(name);
        std::fs::read_to_string(p).ok()
    }

    #[test]
    fn corpus_parity() {
        for rel in [
            "../paper/paper-2026-10-05.md",
            "examples/ikat-paper/ikat-paper.md",
            "examples/mini.md",
        ] {
            let Some(md) = corpus_path(rel) else {
                continue;
            };
            let hand = hand_parse(&md);
            let pdc = pdc_parse(&md).unwrap();
            assert_eq!(hand, pdc, "block stream differs for {rel}");
        }
    }

    /// Latent divergences inside the accepted subset: each must be an
    /// empty diff, with the rule that makes it so named above.
    #[test]
    fn latent_cases() {
        // Directive directly followed by a table (no blank line).
        let md = "# T\n\n%% table {pos=bottom}\n| A |\n|---|\n| 1 |\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // Dangling directive (no table after): para text both sides.
        let md = "# T\n\n%% table {pos=bottom}\n\nJust text.\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // Quote splitting a paragraph: dropped without breaking it.
        let md = "# T\n\nfoo\n> dropped\nbar\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // Closed ATX stays verbatim (hand never stripped closings).
        let md = "# T\n\n## X ##\n\nBody.\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // `#NoSpace` is paragraph text on both sides.
        let md = "# T\n\n#NoSpace here\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // H4+ is paragraph text (hand never knew it).
        let md = "# T\n\n#### deep\n\nBody.\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // Tilde fence: raw lines as para text.
        let md = "# T\n\n~~~mermaid\ngraph TD\na-->b\n~~~\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // Title never flushes; last `# ` wins.
        let md = "# T\n\nfoo\n# U\nbar\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // Lone `|` line: hand tables it; driver re-scans for it.
        let md = "# T\n\n| just a line\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // Table interrupting a paragraph without a blank line.
        let md = "# T\n\nfoo\n| A |\n|---|\n| 1 |\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // `## Appendix extra` prefix rule; `## Abstract` flow.
        let md = "# T\n\n## Appendix B\n\nTail.\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // Emphasis/code stay raw inside paras on both sides.
        let md = "# T\n\nA *b* and `c` line.\n";
        assert_eq!(hand_parse(md), pdc_parse(md).unwrap());
        // Reference definitions are a loud error, never silent loss.
        let md = "# T\n\n[lab]: /url\n";
        assert!(pdc_parse(md).is_err());
    }
}
