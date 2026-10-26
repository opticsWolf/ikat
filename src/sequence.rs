//! Mermaid `sequenceDiagram` subset → TikZ.
//!
//! Covered syntax (dispatched from `flowchart_to_tikz` on the
//! header line — no new fence type, no new entry point):
//!
//! ```text
//! sequenceDiagram
//! participant C as Caller
//! actor T as TeX
//! C->>T: weave(md)
//! T-->>C: PDF
//! alt engine tectonic
//! C->>T: --engine tectonic
//! else default
//! C->>T: pdflatex
//! end
//! ```
//!
//! - `participant NAME [as Label]`, `actor NAME [as Label]`
//!   (actors render as boxes — no stick figures in papers)
//! - `A->>B: text` solid, `A-->>B: text` dashed messages
//! - `alt text` / `else [text]` / `opt text` / `end` boxes
//!   spanning all participant columns
//! - everything else (`loop`, `par`, `rect`, `autonumber`,
//!   `Note`, `activate`, ...) is a string error naming the
//!   statement — the strict-subset doctrine applies unchanged
//!
//! Layout is deterministic: participant columns in declaration
//! order (undeclared names append in first-use order), one row
//! per message, boxes drawn behind the messages they span.

use crate::emit::{edge_label_node, esc_label, tikz_node, unquote, PICTURE_HEAD};

const DX: f64 = 3.5;
const DY: f64 = 1.1;

struct Block {
    /// Box label lines: `alt`/`opt` text, then each `else` text.
    labels: Vec<String>,
    /// Message-row index where the box opens.
    top_row: usize,
    /// Absolute 1-based line of the opening statement.
    top_line: usize,
    /// Opening statement text (echo for missing-`end`).
    stmt: String,
}

fn col_x(i: usize) -> f64 {
    i as f64 * DX
}

fn msg_y(row: usize) -> f64 {
    -1.2 - row as f64 * DY
}

/// `participant NAME [as Label]` → (name, label). Actor identical.
fn parse_decl(words: &[&str]) -> Option<(String, String)> {
    if words.len() < 2 {
        return None;
    }
    let name = words[1].to_string();
    let label = if words.len() >= 4 && words[2] == "as" {
        words[3..].join(" ")
    } else if words.len() == 2 {
        name.clone()
    } else {
        return None;
    };
    Some((name, unquote(&label)))
}

/// `A->>B: text` / `A-->>B: text` → (from, to, dashed, text).
fn parse_msg(stmt: &str) -> Option<(String, String, bool, String)> {
    let (head, text) = stmt.split_once(':')?;
    let head = head.trim();
    let (dashed, parts) = if let Some(i) = head.find("-->>") {
        (true, (head[..i].trim(), head[i + 4..].trim()))
    } else if let Some(i) = head.find("->>") {
        (false, (head[..i].trim(), head[i + 3..].trim()))
    } else {
        return None;
    };
    if parts.0.is_empty() || parts.1.is_empty() {
        return None;
    }
    Some((parts.0.to_string(), parts.1.to_string(), dashed, unquote(text.trim())))
}

pub fn to_tikz(src: &str, base: usize) -> Result<String, crate::error::Error> {
    use crate::error::Error;
    // (parse text, raw echo, body-relative 0-based line).
    let mut raw_lines: Vec<(String, String, usize)> = Vec::new();
    for (idx, l) in src.lines().enumerate() {
        let cut = l.find("%%").map(|i| &l[..i]).unwrap_or(l);
        let t = cut.trim();
        if !t.is_empty() {
            raw_lines.push((t.to_string(), l.to_string(), idx));
        }
    }
    let mut lines = raw_lines.iter();
    let (first, first_raw, first_idx) = lines.next().ok_or_else(|| {
        Error::new("empty mermaid block", base, String::new())
    })?.clone();
    if first != "sequenceDiagram" {
        return Err(Error::new(
            format!("not a sequenceDiagram (got `{first}`)"),
            base + first_idx,
            first_raw.clone(),
        ));
    }

    let mut names: Vec<String> = Vec::new();
    let mut labels: Vec<String> = Vec::new();
    let ensure = |name: &str, names: &mut Vec<String>, labels: &mut Vec<String>| {
        if !names.contains(&name.to_string()) {
            names.push(name.to_string());
            labels.push(name.to_string());
        }
    };
    let mut draws: Vec<String> = Vec::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut row: usize = 0;

    for (stmt, raw, idx) in lines {
        let line: usize = base + *idx;
        let words: Vec<&str> = stmt.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        match words[0] {
            "participant" | "actor" => {
                let (name, label) = parse_decl(&words).ok_or_else(|| {
                    Error::new(format!("bad declaration `{stmt}`"), line, raw.clone())
                })?;
                if !names.contains(&name) {
                    names.push(name);
                    labels.push(label);
                }
            }
            "alt" | "opt" => {
                let text = stmt[words[0].len()..].trim();
                if text.is_empty() {
                    return Err(Error::new(
                        format!("`{}` needs a label in `{stmt}`", words[0]),
                        line,
                        raw.clone(),
                    ));
                }
                blocks.push(Block {
                    labels: vec![unquote(text)],
                    top_row: row,
                    top_line: line,
                    stmt: raw.clone(),
                });
            }
            "else" => {
                let b = blocks.last_mut().ok_or_else(|| {
                    Error::new(format!("`else` without `alt` in `{stmt}`"), line, raw.clone())
                })?;
                b.labels.push(unquote(stmt["else".len()..].trim()));
                let (x0, x1) = (col_x(0) - 1.4, col_x(names.len().saturating_sub(1)) + 1.4);
                let y = msg_y(row) + DY / 2.0;
                draws.push(format!("  \\draw[dashed] ({x0:.1},{y:.1}) -- ({x1:.1},{y:.1});\n"));
            }
            "end" => {
                let b = blocks.pop().ok_or_else(|| {
                    Error::new(format!("`end` without opener in `{stmt}`"), line, raw.clone())
                })?;
                if names.len() < 2 {
                    return Err(Error::new(
                        format!("box needs two participants in `{stmt}`"),
                        line,
                        raw.clone(),
                    ));
                }
                let (x0, x1) = (col_x(0) - 1.4, col_x(names.len() - 1) + 1.4);
                let last = msg_y(row.saturating_sub(1));
                let (yt, yb) = (msg_y(b.top_row) + 0.5, last - 0.8);
                draws.push(format!("  \\draw ({x0:.1},{yt:.1}) rectangle ({x1:.1},{yb:.1});\n"));
                // Box label sits INSIDE below the bottom edge's air:
                // the band above the top edge is shared with the
                // previous message's `fill=white` label box (wider
                // than its glyphs), and `width` fractions scale
                // coordinates but not text, so no outside slot is
                // safe at every scale. Below the last message lives
                // nothing but lifelines: `fill=white` covers those
                // behind the text, as message labels already do.
                // `inner sep=0pt` makes the anchor the true text top.
                draws.push(format!(
                    "  \\node[anchor=north west,font=\\footnotesize\\itshape,fill=white,inner sep=0pt] at ({:.1},{:.1}) {{{}}};\n",
                    x0 + 0.1,
                    last - 0.1,
                    esc_label(&b.labels.join(" / "))
                ));
            }
            _ => {
                // Message, or a strict-subset error naming the statement.
                let (from, to, dashed, text) = parse_msg(stmt).ok_or_else(|| {
                    Error::new(format!("unsupported statement `{stmt}`"), line, raw.clone())
                })?;
                if from == to {
                    return Err(Error::new(
                        format!("self-messages are not supported in `{stmt}`"),
                        line,
                        raw.clone(),
                    ));
                }
                ensure(&from.clone(), &mut names, &mut labels);
                ensure(&to.clone(), &mut names, &mut labels);
                let (xa, xb, y) = (col_x(names.iter().position(|n| n == &from).unwrap()),
                                   col_x(names.iter().position(|n| n == &to).unwrap()), msg_y(row));
                let style = if dashed { "dashed,->" } else { "->" };
                draws.push(format!(
                    "  \\draw[{style}] ({xa:.1},{y:.1}) -- ({xb:.1},{y:.1}) {};\n",
                    edge_label_node(&text)
                ));
                row += 1;
            }
        }
    }
    if let Some(b) = blocks.pop() {
        return Err(Error::new(
            format!("missing `end` for `{}`", b.labels.join(" / ")),
            b.top_line,
            b.stmt,
        ));
    }
    if names.is_empty() {
        return Err(Error::new("no participants", base, String::new()));
    }

    let mut out = String::from(PICTURE_HEAD);
    for (i, (name, label)) in names.iter().zip(labels.iter()).enumerate() {
        out.push_str(&tikz_node(name, col_x(i), 0.0, "box", label));
    }
    let y_end = msg_y(row.saturating_sub(1)) - 0.6;
    for i in 0..names.len() {
        out.push_str(&format!(
            "  \\draw[dashed] ({:.1},-0.4) -- ({:.1},{:.1});\n",
            col_x(i),
            col_x(i),
            y_end
        ));
    }
    for d in draws {
        out.push_str(&d);
    }
    out.push_str("\\end{tikzpicture}\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PIPE: &str = "sequenceDiagram\nparticipant C as Caller\nparticipant P as Python\nC->>P: weave(md)\nP-->>C: tex";

    #[test]
    fn error_unsupported_statement_carries_line() {
        // `loop` is body line 4 (0-based 3), base 7 → line 10.
        let src = "sequenceDiagram\nparticipant A\nparticipant B\nloop every day\nA->>B: x\nend";
        let e = to_tikz(src, 7).unwrap_err();
        assert!(e.msg.contains("unsupported statement"), "got: {e}");
        assert_eq!(e.line, 10, "got: {e}");
        assert!(e.echo.contains("loop"), "got: {e}");
    }

    #[test]
    fn column_layout_math() {
        let tikz = to_tikz("sequenceDiagram\nparticipant A\nparticipant B\nparticipant C\nA->>C: hi", 1).unwrap();
        assert!(tikz.contains("\\node (A) at (0.0,0.0)"), "col 0, got:\n{tikz}");
        assert!(tikz.contains("\\node (B) at (3.5,0.0)"), "col 1, got:\n{tikz}");
        assert!(tikz.contains("\\node (C) at (7.0,0.0)"), "col 2, got:\n{tikz}");
    }

    #[test]
    fn alt_box_spanning() {
        let tikz = to_tikz("sequenceDiagram\nparticipant A\nparticipant B\nA->>B: x\nalt fail\nA->>B: retry\nelse ok\nA->>B: done\nend", 1).unwrap();
        assert!(tikz.contains("rectangle"), "box, got:\n{tikz}");
        assert!(tikz.contains("dashed"), "else divider, got:\n{tikz}");
        assert!(tikz.contains("fail / ok"), "labels joined, got:\n{tikz}");
    }

    #[test]
    fn loop_is_an_error() {
        assert!(to_tikz("sequenceDiagram\nparticipant A\nparticipant B\nloop every day\nA->>B: x\nend", 1).is_err());
    }

    #[test]
    fn missing_end_is_an_error() {
        assert!(to_tikz("sequenceDiagram\nparticipant A\nparticipant B\nalt x\nA->>B: y", 1).is_err());
        assert!(to_tikz("sequenceDiagram\nparticipant A\nparticipant B\nend", 1).is_err());
    }

    #[test]
    fn knockout_on_labels() {
        let tikz = to_tikz(PIPE, 1).unwrap();
        assert!(tikz.contains("node[midway,above,fill=white"), "knockout, got:\n{tikz}");
    }

    #[test]
    fn declaration_order_not_use_order() {
        let tikz = to_tikz("sequenceDiagram\nparticipant A\nparticipant B\nB->>A: first use is B", 1).unwrap();
        assert!(tikz.contains("\\node (A) at (0.0,0.0)"), "declaration wins, got:\n{tikz}");
    }

    #[test]
    fn dashed_vs_solid() {
        let tikz = to_tikz(PIPE, 1).unwrap();
        let dashed = tikz.matches("\\draw[dashed,->]").count();
        let solid = tikz.matches("\\draw[->]").count();
        assert_eq!((solid, dashed), (1, 1), "one each, got:\n{tikz}");
    }
}
