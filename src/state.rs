//! Mermaid `stateDiagram-v2` subset → TikZ.
//!
//! Covered syntax (dispatched from `flowchart_to_tikz` on the
//! header line):
//!
//! ```text
//! stateDiagram-v2
//! [*] --> Fences
//! Fences --> Convert : strict subset
//! state "Convert body" as Convert
//! state Wrap {
//!   Inner --> Done
//! }
//! Validate --> [*]
//! ```
//!
//! - `[*] --> Name` start, `Name --> [*]` end
//! - `A --> B [: label]` transitions (arrow only — `-->` is the
//!   transition connector; anything else errors)
//! - `state "Label" as Name` long form
//! - one level of `state Name { … }` composite boxes; nesting a
//!   composite inside a composite is a build error, not a silent
//!   flat render
//!
//! Layout reuses the flowchart layered BFS (`mermaid::layered_xy`):
//! states are nodes, transitions are edges — layout first, boxes
//! second, so a composite box can never distort coordinates.

use std::collections::HashMap;

use crate::emit::{edge_label_node, tikz_node, unquote, PICTURE_HEAD};
use crate::mermaid::{self, EdgeStyle, Shape};

const START: &str = "__start";
const END: &str = "__end";

struct Composite {
    name: String,
    members: Vec<String>,
    /// Absolute 1-based line of the opening `state X {`.
    line: usize,
    /// Opening statement text (echo for missing-`}`).
    stmt: String,
}

fn is_id(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// `A --> B [: label]` with `[*]` endpoints allowed.
fn parse_trans(stmt: &str) -> Option<(String, String, Option<String>)> {
    let i = stmt.find("-->")?;
    let (a, rest) = (stmt[..i].trim(), stmt[i + 3..].trim());
    let (b, label) = match rest.split_once(':') {
        Some((b, l)) => (b.trim(), Some(unquote(l.trim()))),
        None => (rest, None),
    };
    if b.is_empty() {
        return None;
    }
    let end_b = b.split_whitespace().next().unwrap_or("");
    if end_b.is_empty() || (label.is_none() && end_b != b) {
        return None;
    }
    let b = end_b.to_string();
    if !(is_id(&a) || a == "[*]") || !(is_id(&b) || b == "[*]") {
        return None;
    }
    Some((a.to_string(), b, label))
}

pub fn to_tikz(src: &str, base: usize) -> Result<String, crate::error::Error> {
    use crate::error::Error;
    // (parse text, raw echo, body-relative 0-based line).
    let mut raw_lines: Vec<(String, String, usize)> = Vec::new();
    for (idx, l) in src.lines().enumerate() {
        let cut = l.find("%%").map(|i| &l[..i]).unwrap_or(l);
        let t = cut.trim();
        if !t.is_empty() {
            raw_lines.push((t.to_string(), l.trim().to_string(), idx));
        }
    }
    let mut lines = raw_lines.iter();
    let (first, first_raw, first_idx) = lines.next().ok_or_else(|| {
        Error::new("empty mermaid block", base, String::new())
    })?.clone();
    if first != "stateDiagram-v2" {
        return Err(Error::new(
            format!("not a stateDiagram-v2 (got `{first}`)"),
            base + first_idx,
            first_raw.clone(),
        ));
    }

    let mut g = mermaid::Graph {
        direction: mermaid::Direction::Td,
        nodes: Vec::new(),
        node_idx: HashMap::new(),
        edges: Vec::new(),
    };
    let mut composites: Vec<Composite> = Vec::new();
    // `state Name {` opens a composite; transitions inside it
    // register their endpoints as members (boxes wrap placed
    // points — membership is the only thing the block adds).
    let mut open: Option<Composite> = None;

    for (stmt, raw, idx) in lines {
        let line: usize = base + *idx;
        if let Some(rest) = stmt.strip_prefix("state ") {
            let rest = rest.trim();
            if let Some(name) = rest.strip_suffix('{') {
                let name = name.trim().to_string();
                if open.is_some() {
                    return Err(Error::new(
                        "nested composites are not supported",
                        line,
                        raw.clone(),
                    ));
                }
                if !is_id(&name) {
                    return Err(Error::new(
                        format!("bad composite name in `{stmt}`"),
                        line,
                        raw.clone(),
                    ));
                }
                open = Some(Composite {
                    name,
                    members: Vec::new(),
                    line,
                    stmt: raw.clone(),
                });
                continue;
            }
            // Long form: `state "Label" as Name`.
            if let Some(i) = rest.find(" as ") {
                let (label, name) = (rest[..i].trim(), rest[i + 4..].trim());
                if !is_id(name) {
                    return Err(Error::new(
                        format!("bad state name in `{stmt}`"),
                        line,
                        raw.clone(),
                    ));
                }
                g.set_node(name.to_string(), Shape::Rect, unquote(label));
                if let Some(c) = open.as_mut() {
                    c.members.push(name.to_string());
                }
                continue;
            }
            return Err(Error::new(
                format!("unsupported statement `{stmt}`"),
                line,
                raw.clone(),
            ));
        }
        if stmt == "}" {
            let c = open.take().ok_or_else(|| {
                Error::new(format!("`}}` without `state` in `{stmt}`"), line, raw.clone())
            })?;
            composites.push(c);
            continue;
        }
        let (a, b, label) = parse_trans(stmt).ok_or_else(|| {
            Error::new(format!("unsupported statement `{stmt}`"), line, raw.clone())
        })?;
        let (a, b) = (
            if a == "[*]" { START.to_string() } else { a },
            if b == "[*]" { END.to_string() } else { b },
        );
        for n in [&a, &b] {
            if !g.node_idx.contains_key(n) {
                g.set_node(n.clone(), Shape::Rect, n.clone());
            }
            if let Some(c) = open.as_mut() {
                if n != START && n != END && !c.members.contains(n) {
                    c.members.push(n.clone());
                }
            }
        }
        g.edges.push(mermaid::Edge {
            from: a,
            to: b,
            label,
            style: EdgeStyle::Arrow,
            line,
            stmt: raw.clone(),
        });
    }
    if let Some(c) = open {
        return Err(Error::new("missing `}` for composite".to_string(), c.line, c.stmt));
    }
    // Long-form members that never appear in a transition still
    // exist: declare them so the box has something to wrap. (The
    // composite NAME is a box label, never a node — it must not
    // enter layout.)
    for c in &composites {
        for m in &c.members {
            if !g.node_idx.contains_key(m) {
                g.set_node(m.clone(), Shape::Rect, m.clone());
            }
        }
    }
    if g.nodes.is_empty() {
        return Err(Error::new("no states", base, String::new()));
    }

    let depth = mermaid::depths(&g);
    let xy = mermaid::layered_xy(&g, &depth);

    let mut out = String::from(PICTURE_HEAD);
    for (i, n) in g.nodes.iter().enumerate() {
        let (x, y) = xy[i];
        if n.id == START {
            out.push_str(&format!(
                "  \\node[circle,fill=black,inner sep=0pt,minimum size=8pt] ({START}) at ({x:.1},{y:.1}) {{}};\n"
            ));
        } else if n.id == END {
            out.push_str(&format!(
                "  \\node[circle,draw,inner sep=0pt,minimum size=12pt] ({END}) at ({x:.1},{y:.1}) {{}};\n  \\node[circle,fill=black,inner sep=0pt,minimum size=6pt] at ({x:.1},{y:.1}) {{}};\n"
            ));
        } else {
            out.push_str(&tikz_node(&n.id, x, y, "box", &n.label));
        }
    }
    // Boxes second: layout is already frozen, members' coordinates
    // are facts. Shared `cluster_box` geometry + `cluster_rect`
    // rendering — the same numbers the flowchart subgraph boxes
    // use (the box-label rule lives on `cluster_box`).
    for c in &composites {
        let pts: Vec<(f64, f64)> = c
            .members
            .iter()
            .filter_map(|m| g.node_idx.get(m).map(|&i| xy[i]))
            .collect();
        let Some((r, at)) = crate::layout::cluster_box(&pts) else {
            return Err(Error::new(
                format!("composite `{}` has no placed members", c.name),
                c.line,
                c.stmt.clone(),
            ));
        };
        out.push_str(&crate::emit::cluster_rect(&c.name, r, at));
    }
    for e in &g.edges {
        if let Some(lbl) = &e.label {
            out.push_str(&format!(
                "  \\draw[->] ({}) -- ({}) {};\n",
                e.from,
                e.to,
                edge_label_node(lbl)
            ));
        } else {
            out.push_str(&format!("  \\draw[->] ({}) -- ({});\n", e.from, e.to));
        }
    }
    out.push_str("\\end{tikzpicture}\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = "stateDiagram-v2\n[*] --> Fences\nFences --> Convert : strict subset\nConvert --> Wrap\nWrap --> Validate\nValidate --> Compile\nCompile --> [*]";

    #[test]
    fn error_bad_transition_carries_line() {
        // `A ==> B` is body line 2 (0-based 1), base 4 → line 5.
        let e = to_tikz("stateDiagram-v2\nA ==> B\n", 4).unwrap_err();
        assert!(e.msg.contains("unsupported statement"), "got: {e}");
        assert_eq!(e.line, 5, "got: {e}");
        assert!(e.echo.contains("==>"), "got: {e}");
    }

    #[test]
    fn start_end_rendering() {
        let tikz = to_tikz(DOC, 1).unwrap();
        assert!(tikz.contains("fill=black"), "start dot, got:\n{tikz}");
        assert!(tikz.contains("circle,draw"), "end bullseye, got:\n{tikz}");
        assert!(tikz.contains("\\draw[->] (__start) -- (Fences)"), "start edge, got:\n{tikz}");
        assert!(tikz.contains("-- (__end)"), "end edge, got:\n{tikz}");
    }

    #[test]
    fn composite_bounding_math() {
        let tikz = to_tikz("stateDiagram-v2\n[*] --> A\nstate Box {\nA --> B\n}\nB --> [*]", 1).unwrap();
        assert!(tikz.contains("rectangle"), "box, got:\n{tikz}");
        assert!(tikz.contains("{Box}"), "box label, got:\n{tikz}");
        // Members keep their laid-out coordinates: the box wraps
        // placed points, it never moves them.
        let ax = tikz.lines().find(|l| l.contains("\\node (A)")).unwrap().to_string();
        assert!(ax.contains("at (0.0,"), "A placed free, got: {ax}");
    }

    #[test]
    fn nesting_is_an_error() {
        assert!(to_tikz("stateDiagram-v2\nstate A {\nstate B {\nX --> Y\n}\n}", 1).is_err());
    }

    #[test]
    fn long_form_labels() {
        let tikz = to_tikz("stateDiagram-v2\nstate \"Convert body\" as Convert\n[*] --> Convert", 1).unwrap();
        assert!(tikz.contains("{Convert body}"), "label kept, got:\n{tikz}");
    }

    #[test]
    fn transition_labels() {
        let tikz = to_tikz(DOC, 1).unwrap();
        assert!(tikz.contains("node[midway,above,fill=white"), "knockout, got:\n{tikz}");
        assert!(tikz.contains("{strict subset}"), "label text, got:\n{tikz}");
    }

    #[test]
    fn layout_determinism() {
        assert_eq!(to_tikz(DOC, 1).unwrap(), to_tikz(DOC, 1).unwrap());
    }

    #[test]
    fn unknown_statement_is_an_error() {
        assert!(to_tikz("stateDiagram-v2\nA --> B\nnote right of A: hi", 1).is_err());
        assert!(to_tikz("stateDiagram-v2\nA --> B\ndirection LR", 1).is_err());
    }
}
