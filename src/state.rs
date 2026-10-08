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

use crate::emit::{edge_label_node, esc_label, tikz_node, unquote, PICTURE_HEAD};
use crate::mermaid::{self, EdgeStyle, Shape};

const START: &str = "__start";
const END: &str = "__end";

struct Composite {
    name: String,
    members: Vec<String>,
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

pub fn to_tikz(src: &str) -> Result<String, String> {
    let mut lines = src
        .lines()
        .map(|l| {
            let cut = l.find("%%").map(|i| &l[..i]).unwrap_or(l);
            cut.trim()
        })
        .filter(|l| !l.is_empty())
        .peekable();
    let first = lines.next().ok_or("empty mermaid block")?;
    if first != "stateDiagram-v2" {
        return Err(format!("not a stateDiagram-v2 (got `{first}`)"));
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

    for stmt in lines {
        if let Some(rest) = stmt.strip_prefix("state ") {
            let rest = rest.trim();
            if let Some(name) = rest.strip_suffix('{') {
                let name = name.trim().to_string();
                if open.is_some() {
                    return Err("nested composites are not supported".to_string());
                }
                if !is_id(&name) {
                    return Err(format!("bad composite name in `{stmt}`"));
                }
                open = Some(Composite { name, members: Vec::new() });
                continue;
            }
            // Long form: `state "Label" as Name`.
            if let Some(i) = rest.find(" as ") {
                let (label, name) = (rest[..i].trim(), rest[i + 4..].trim());
                if !is_id(name) {
                    return Err(format!("bad state name in `{stmt}`"));
                }
                g.set_node(name.to_string(), Shape::Rect, unquote(label));
                if let Some(c) = open.as_mut() {
                    c.members.push(name.to_string());
                }
                continue;
            }
            return Err(format!("unsupported statement `{stmt}`"));
        }
        if stmt == "}" {
            let c = open.take().ok_or_else(|| format!("`}}` without `state` in `{stmt}`"))?;
            composites.push(c);
            continue;
        }
        let (a, b, label) =
            parse_trans(stmt).ok_or_else(|| format!("unsupported statement `{stmt}`"))?;
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
        });
    }
    if open.is_some() {
        return Err("missing `}` for composite".to_string());
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
        return Err("no states".to_string());
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
    // are facts. Label outside above (same lesson as sequence).
    for c in &composites {
        let pts: Vec<(f64, f64)> = c
            .members
            .iter()
            .filter_map(|m| g.node_idx.get(m).map(|&i| xy[i]))
            .collect();
        if pts.is_empty() {
            return Err(format!("composite `{}` has no placed members", c.name));
        }
        let (x0, x1) = (
            pts.iter().map(|p| p.0).fold(f64::INFINITY, f64::min) - 1.2,
            pts.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max) + 1.2,
        );
        // Box-label rule (shared with future subgraph boxes): label
        // INSIDE at the top with a half-row of headroom (yt = top +
        // 1.1 on a 2.4 row grid). Outside-above collides with edge
        // labels crossing the top edge; inside-top only needs the
        // first member's box to start 0.8 below the label — proven
        // by the showcase Weave box, which struck `packages` before.
        // Screen coords: y decreases downward, so the TOP edge is
        // the max y plus headroom, the BOTTOM the min y minus room.
        let (yt, yb) = (
            pts.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max) + 1.1,
            pts.iter().map(|p| p.1).fold(f64::INFINITY, f64::min) - 0.7,
        );
        out.push_str(&format!("  \\draw ({x0:.1},{yt:.1}) rectangle ({x1:.1},{yb:.1});\n"));
        out.push_str(&format!(
            "  \\node[anchor=north west,font=\\footnotesize\\itshape] at ({:.1},{:.1}) {{{}}};\n",
            x0 + 0.1,
            yt - 0.1,
            esc_label(&c.name)
        ));
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
    fn start_end_rendering() {
        let tikz = to_tikz(DOC).unwrap();
        assert!(tikz.contains("fill=black"), "start dot, got:\n{tikz}");
        assert!(tikz.contains("circle,draw"), "end bullseye, got:\n{tikz}");
        assert!(tikz.contains("\\draw[->] (__start) -- (Fences)"), "start edge, got:\n{tikz}");
        assert!(tikz.contains("-- (__end)"), "end edge, got:\n{tikz}");
    }

    #[test]
    fn composite_bounding_math() {
        let tikz = to_tikz("stateDiagram-v2\n[*] --> A\nstate Box {\nA --> B\n}\nB --> [*]").unwrap();
        assert!(tikz.contains("rectangle"), "box, got:\n{tikz}");
        assert!(tikz.contains("{Box}"), "box label, got:\n{tikz}");
        // Members keep their laid-out coordinates: the box wraps
        // placed points, it never moves them.
        let ax = tikz.lines().find(|l| l.contains("\\node (A)")).unwrap().to_string();
        assert!(ax.contains("at (0.0,"), "A placed free, got: {ax}");
    }

    #[test]
    fn nesting_is_an_error() {
        assert!(to_tikz("stateDiagram-v2\nstate A {\nstate B {\nX --> Y\n}\n}").is_err());
    }

    #[test]
    fn long_form_labels() {
        let tikz = to_tikz("stateDiagram-v2\nstate \"Convert body\" as Convert\n[*] --> Convert").unwrap();
        assert!(tikz.contains("{Convert body}"), "label kept, got:\n{tikz}");
    }

    #[test]
    fn transition_labels() {
        let tikz = to_tikz(DOC).unwrap();
        assert!(tikz.contains("node[midway,above,fill=white"), "knockout, got:\n{tikz}");
        assert!(tikz.contains("{strict subset}"), "label text, got:\n{tikz}");
    }

    #[test]
    fn layout_determinism() {
        assert_eq!(to_tikz(DOC).unwrap(), to_tikz(DOC).unwrap());
    }

    #[test]
    fn unknown_statement_is_an_error() {
        assert!(to_tikz("stateDiagram-v2\nA --> B\nnote right of A: hi").is_err());
        assert!(to_tikz("stateDiagram-v2\nA --> B\ndirection LR").is_err());
    }
}
