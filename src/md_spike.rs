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

// (pulldown-cmark now ships in production: see `crate::md`)

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
                out.push(Block::Directive(crate::doc::fence_attr_str(s).to_string()));
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

/// Production-backed candidate: `crate::md::blocks()` mapped onto
/// the harness `Block` stream. Post-switch this is the cross-check —
/// the frozen `hand_parse` oracle vs the shipped mapping. Any drift
/// between the two reddens here (and in the golden test).
fn pdc_parse(md: &str) -> Result<Vec<Block>, String> {
    let mut out = Vec::new();
    for b in crate::md::blocks(md)? {
        match b {
            crate::md::MdBlock::Title(t) => {
                if let Some(Block::Title(_)) = out.last() {
                    out.pop();
                }
                out.push(Block::Title(t));
            }
            crate::md::MdBlock::H2(h) => out.push(Block::H2(h)),
            crate::md::MdBlock::H3(h) => out.push(Block::H3(h)),
            crate::md::MdBlock::Appendix => out.push(Block::Appendix),
            crate::md::MdBlock::Abstract => out.push(Block::Abstract),
            crate::md::MdBlock::Fence { info, body } => {
                // Same backtick-strip the hand scanner applied.
                let mut sp = info.trim_start_matches('`').splitn(2, char::is_whitespace);
                out.push(Block::Fence {
                    lang: sp.next().unwrap_or("").to_string(),
                    attr: sp.next().unwrap_or("").trim().to_string(),
                    body,
                });
            }
            crate::md::MdBlock::Table(rows) => out.push(Block::Table(rows)),
            crate::md::MdBlock::Directive(a) => out.push(Block::Directive(a)),
            crate::md::MdBlock::Para(ls) => out.push(Block::Para(ls.join(" "))),
        }
    }
    Ok(out)
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
