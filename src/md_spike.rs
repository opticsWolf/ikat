//! M5.3 spike (FORMAT-DRAFT C.2): parse one manuscript with BOTH the
//! hand scanner's block rules and pulldown-cmark, diff the structure.
//! Test-only module (`#[cfg(test)]`); pulldown-cmark is a dev-dependency
//! and ships in no wheel. Switching needs an EMPTY diff on the
//! required properties plus a byte-identical golden paper — this
//! spike settles whether that is even reachable.

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

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

/// Block descriptors from the HAND scanner's rules (`doc.rs convert`):
/// headings, fences (lang + verbatim attr string), table runs with
/// shape, `%%` directives, paragraphs (breaks only, text elided).
fn hand_blocks(md: &str) -> Vec<String> {
    let lines: Vec<&str> = md.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    let mut para_open = false;
    let flush = |out: &mut Vec<String>, para_open: &mut bool| {
        if *para_open {
            out.push("Para".to_string());
            *para_open = false;
        }
    };
    while i < lines.len() {
        let s = lines[i].trim();
        if s.starts_with("```") {
            flush(&mut out, &mut para_open);
            let info = s.trim_start_matches('`');
            let mut sp = info.splitn(2, char::is_whitespace);
            let lang = sp.next().unwrap_or("");
            let attr = sp.next().unwrap_or("").trim();
            out.push(format!("Fence({lang}|{attr})"));
            i += 1;
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                i += 1;
            }
            i += 1;
            continue;
        }
        if s.starts_with("%% table") {
            flush(&mut out, &mut para_open);
            out.push(format!("Directive({})", s.trim_start_matches("%% table").trim()));
            i += 1;
            continue;
        }
        if s.starts_with('|') {
            flush(&mut out, &mut para_open);
            let mut rows = 0;
            let mut cols = 0;
            while i < lines.len() && lines[i].trim().starts_with('|') {
                let cells: Vec<&str> = lines[i]
                    .trim()
                    .trim_matches('|')
                    .split('|')
                    .map(str::trim)
                    .collect();
                cols = cols.max(cells.len());
                rows += 1;
                i += 1;
            }
            out.push(format!("Table(rows={rows},cols={cols})"));
            continue;
        }
        if let Some(h) = s.strip_prefix("### ") {
            flush(&mut out, &mut para_open);
            out.push(format!("H3({h})"));
        } else if let Some(h) = s.strip_prefix("## ") {
            flush(&mut out, &mut para_open);
            out.push(format!("H2({h})"));
        } else if let Some(h) = s.strip_prefix("# ") {
            flush(&mut out, &mut para_open);
            out.push(format!("H1({h})"));
        } else if s.is_empty() {
            flush(&mut out, &mut para_open);
        } else {
            para_open = true;
        }
        i += 1;
    }
    flush(&mut out, &mut para_open);
    out
}

/// Block descriptors from pulldown-cmark events: headings with text,
/// fenced info strings VERBATIM, table head/row cell counts,
/// paragraph breaks.
fn pdc_blocks(md: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut heading: Option<(String, String)> = None;
    let mut row_cells = 0;
    let mut in_row = false;
    let mut table_rows: Vec<usize> = Vec::new();
    for ev in Parser::new_ext(md, Options::ENABLE_TABLES) {
        match ev {
            Event::Start(Tag::Heading { level, .. }) => {
                heading = Some((format!("{level:?}").to_uppercase(), String::new()));
            }
            Event::Text(t) => {
                if let Some((_, ref mut text)) = heading {
                    text.push_str(&t);
                } else if in_row {
                    row_cells += 1;
                }
            }
            Event::End(tag) => match tag {
                pulldown_cmark::TagEnd::Heading(_) => {
                    let (lvl, text) = heading.take().unwrap();
                    let n: &str = match lvl.as_str() {
                        "H1" => "1",
                        "H2" => "2",
                        _ => "3",
                    };
                    out.push(format!("H{n}({text})"));
                }
                pulldown_cmark::TagEnd::TableHead => {
                    table_rows.push(row_cells);
                    row_cells = 0;
                    in_row = false;
                }
                pulldown_cmark::TagEnd::TableRow => {
                    table_rows.push(row_cells);
                    row_cells = 0;
                    in_row = false;
                }
                pulldown_cmark::TagEnd::Table => {
                    out.push(format!("Table(head+rows={table_rows:?})"));
                    table_rows.clear();
                }
                pulldown_cmark::TagEnd::Paragraph => out.push("Para".to_string()),
                _ => {}
            },
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                let info = info.trim();
                let mut sp = info.splitn(2, char::is_whitespace);
                out.push(format!(
                    "Fence({}|{})",
                    sp.next().unwrap_or(""),
                    sp.next().unwrap_or("").trim()
                ));
            }
            Event::Start(Tag::Table(_)) => {}
            Event::Start(Tag::TableHead | Tag::TableRow) => {
                in_row = true;
                row_cells = 0;
            }
            _ => {}
        }
    }
    out
}

#[test]
fn spike_event_diff() {
    let hand = hand_blocks(SAMPLE);
    let pdc = pdc_blocks(SAMPLE);
    eprintln!("HAND: {hand:#?}\nPDC:  {pdc:#?}");
    // Required property 1: fenced info strings arrive verbatim.
    let pinfos: Vec<String> = pdc
        .iter()
        .filter_map(|b| b.strip_prefix("Fence("))
        .map(|s| s.strip_suffix(')').unwrap().to_string())
        .collect();
    assert_eq!(pinfos, ["mermaid|{span=column pos=both}", "mermaid|{inline}"]);
    // Required property 2: headings identical (levels + text).
    let hheads: Vec<&str> = hand
        .iter()
        .filter(|b| b.starts_with('H'))
        .map(String::as_str)
        .collect();
    let pheads: Vec<&str> = pdc
        .iter()
        .filter(|b| b.starts_with('H'))
        .map(String::as_str)
        .collect();
    assert_eq!(hheads, pheads);
    // Required property 3: table shape parity (cols; data rows =
    // total minus delimiter on the hand side, minus head on pdc).
    assert!(hand.contains(&"Table(rows=4,cols=2)".to_string()));
    assert!(pdc.contains(&"Table(head+rows=[2, 2, 2])".to_string()));
    // Required property 4: paragraph breaks. pulldown-cmark sees the
    // `%% table` directive line as paragraph TEXT (it cannot know our
    // directives) — exactly one extra Para, identified, everything
    // else aligned.
    let hparas = hand.iter().filter(|b| *b == "Para").count();
    let pparas = pdc.iter().filter(|b| *b == "Para").count();
    assert_eq!((hparas, pparas), (3, 4));
    assert!(hand.contains(&"Directive({pos=barrier})".to_string()));
}
