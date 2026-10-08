//! Mermaid flowchart subset → TikZ.
//!
//! Covered syntax (the subset papers actually use):
//!
//! ```text
//! graph TD
//! q{"Which question?"}
//! q -->|"true at v"| av["as_of_valid(v)<br/>L's current state"]
//! av -->|compose| both["both instants"]
//! ```
//!
//! - directions `TD`, `TB`, `LR`, `RL`, `BT` (`graph` or `flowchart`)
//! - node shapes `[rect]`, `{diamond}`, `([stadium])`, `[[subroutine]]`
//! - edges `-->`, `---`, `==>` with optional `|label|`
//! - edge chains (`A-->B-->C`), `;` separators, `%%` comments,
//!   `<br/>` line breaks in labels
//!
//! Layout is deterministic layered: depth from roots via BFS, siblings
//! ordered by first appearance, explicit coordinates in cm. No guessing,
//! no overlap for DAGs.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Td,
    Lr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Shape {
    Rect,
    Diamond,
    Stadium,
    Subroutine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EdgeStyle {
    Arrow,
    Line,
    Thick,
}

#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub id: String,
    pub label: String,
    pub shape: Shape,
}

#[derive(Debug, Clone)]
pub(crate) struct Edge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub style: EdgeStyle,
    /// Absolute (base-resolved) 1-based source line of the edge's
    /// statement, for emission-time errors (router exhaustion).
    pub line: usize,
    /// The statement text (echo for emission-time errors).
    pub stmt: String,
}

use crate::emit::{edge_label_node, tikz_node, unquote};

fn is_id_char(c: char) -> bool {
    // NOTE: `-` is deliberately excluded: `a-->b` must lex the edge,
    // not swallow `--` into the id.
    c.is_alphanumeric() || c == '_'
}



/// Parse `id[shape...]` at the head of `s`.
/// Returns (id, shape, label, rest).
fn parse_node(s: &str) -> Option<(String, Shape, String, &str)> {
    let end = s.find(|c: char| !is_id_char(c))?;
    let (id, mut rest) = (s[..end].to_string(), &s[end..]);
    let (shape, open, close) = if rest.starts_with("([[") || rest.starts_with("[[") {
        let open = if rest.starts_with("([[") { "([[" } else { "[[" };
        (Shape::Subroutine, open, "]]")
    } else if rest.starts_with("([") {
        (Shape::Stadium, "([", "])")
    } else if rest.starts_with('[') {
        (Shape::Rect, "[", "]")
    } else if rest.starts_with('{') {
        (Shape::Diamond, "{", "}")
    } else {
        return None;
    };
    rest = &rest[open.len()..];
    let close_at = rest.find(close)?;
    let label = unquote(&rest[..close_at]);
    rest = rest[close_at + close.len()..].trim_start();
    // Stadium's extra paren: "([label])" leaves ")".
    if shape == Shape::Stadium && rest.starts_with(')') {
        rest = rest[1..].trim_start();
    }
    Some((id, shape, label, rest))
}

/// Parse one edge connector at the head of `s`.
/// Returns (style, edge label, target node spec, rest).
fn parse_edge(s: &str) -> Option<(EdgeStyle, Option<String>, &str)> {
    let (style, rest) = if s.starts_with("-->") {
        (EdgeStyle::Arrow, &s[3..])
    } else if s.starts_with("==>") {
        (EdgeStyle::Thick, &s[3..])
    } else if s.starts_with("---") {
        (EdgeStyle::Line, &s[3..])
    } else if s.starts_with("--") {
        // Dashed/text edges ("-- text -->") degrade to plain arrows.
        let mut r = &s[2..];
        if let Some(i) = r.find("-->") {
            r = &r[i..];
            (EdgeStyle::Arrow, r)
        } else {
            return None;
        }
    } else {
        return None;
    };
    let rest = rest.trim_start();
    let (label, rest) = if rest.starts_with('|') {
        let end = rest[1..].find('|')?;
        (
            Some(unquote(&rest[1..1 + end])),
            rest[1 + end + 1..].trim_start(),
        )
    } else {
        (None, rest)
    };
    Some((style, label, rest))
}

/// One `subgraph ID [title]` … `end` cluster. Members are node ids
/// touched while the block is open; the box is drawn post-layout
/// (same layout-first rule as state composites). Nested subgraphs
/// are a build error.
pub(crate) struct Subgraph {
    pub id: String,
    pub title: String,
    pub members: Vec<String>,
    /// Absolute 1-based line of the opening `subgraph` statement
    /// (missing-`end` and empty-box errors point here).
    pub line: usize,
    /// Opening statement text (echo for those errors).
    pub stmt: String,
}

/// Shared with the state emitter: states are nodes, transitions
/// are edges, layout is this module's layered placer.
pub(crate) struct Graph {
    pub direction: Direction,
    pub nodes: Vec<Node>,
    pub node_idx: HashMap<String, usize>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub(crate) fn ensure_node(&mut self, id: &str) {
        if !self.node_idx.contains_key(id) {
            self.node_idx.insert(id.to_string(), self.nodes.len());
            self.nodes.push(Node {
                id: id.to_string(),
                label: id.to_string(),
                shape: Shape::Rect,
            });
        }
    }

    pub(crate) fn set_node(&mut self, id: String, shape: Shape, label: String) {
        self.ensure_node(&id);
        let i = self.node_idx[&id];
        self.nodes[i] = Node { id, label, shape };
    }
}

/// `subgraph ID` or `subgraph ID[title words]` → (id, title).
/// A bare `subgraph` (or `subgraphs`) is NOT a block — it falls
/// through to node parsing exactly as before.
fn parse_subgraph(stmt: &str) -> Option<(String, String)> {
    let rest = stmt.strip_prefix("subgraph")?;
    let rest = rest.strip_prefix(|c: char| c == ' ' || c == '\t')?;
    let rest = rest.trim_start();
    if rest.is_empty() {
        return None;
    }
    let (id, tail) = match rest.find(|c: char| !(c.is_alphanumeric() || c == '_')) {
        Some(0) => return None,
        Some(i) => (rest[..i].to_string(), rest[i..].trim_start()),
        None => (rest.to_string(), ""),
    };
    if tail.is_empty() {
        return Some((id.clone(), id));
    }
    let title = tail
        .strip_prefix('[')
        .and_then(|t| t.strip_suffix(']'))?;
    Some((id, unquote(title)))
}

fn parse(src: &str, base: usize) -> Result<(Graph, Vec<Subgraph>), crate::error::Error> {
    use crate::error::Error;
    // (parse text, raw echo, body-relative 0-based line). `%%`
    // comments strip for parsing but the echo keeps the raw line.
    let mut lines: Vec<(String, String, usize)> = Vec::new();
    for (idx, l) in src.lines().enumerate() {
        let cut = l.find("%%").map(|i| &l[..i]).unwrap_or(l);
        let t = cut.trim();
        if !t.is_empty() {
            lines.push((t.to_string(), l.trim().to_string(), idx));
        }
    }
    let (first, first_raw, first_idx) = lines.first().ok_or_else(|| {
        Error::new("empty mermaid block", base, String::new())
    })?.clone();
    let dir_word = first
        .strip_prefix("graph")
        .or_else(|| first.strip_prefix("flowchart"))
        .ok_or_else(|| {
            Error::new(
                "first line must be `graph TD/LR/...`",
                base + first_idx,
                first_raw.clone(),
            )
        })?
        .trim();
    let direction = match dir_word {
        "TD" | "TB" => Direction::Td,
        "LR" | "RL" => Direction::Lr,
        "BT" => Direction::Td, // laid top-down; BT flip is cosmetic
        _ => {
            return Err(Error::new(
                format!("unsupported direction `{dir_word}`"),
                base + first_idx,
                first_raw.clone(),
            ))
        }
    };

    let mut g = Graph {
        direction,
        nodes: Vec::new(),
        node_idx: HashMap::new(),
        edges: Vec::new(),
    };

    // Statements split on ';' (newlines already separate chains);
    // each keeps its body-relative line and raw echo.
    let mut stmts: Vec<(String, String, usize)> = Vec::new();
    for (t, raw, idx) in lines.iter().skip(1) {
        for part in t.split(';') {
            let s = part.trim();
            if !s.is_empty() {
                stmts.push((s.to_string(), raw.clone(), *idx));
            }
        }
    }
    if stmts.is_empty() {
        return Err(Error::new("no statements", base + first_idx, first_raw.clone()));
    }

    let mut boxes: Vec<Subgraph> = Vec::new();
    let mut open: Option<Subgraph> = None;
    // Raw open-statement text for the missing-`end` echo (at most
    // one box is ever open — nesting is an error).
    let mut open_raw: Option<String> = None;

    for (stmt, raw, idx) in &stmts {
        let line = base + idx;
        if let Some((id, title)) = parse_subgraph(stmt) {
            if open.is_some() {
                return Err(Error::new(
                    "nested subgraphs are not supported",
                    line,
                    raw.clone(),
                ));
            }
            open = Some(Subgraph {
                id,
                title,
                members: Vec::new(),
                line: line,
                stmt: raw.clone(),
            });
            open_raw = Some(raw.clone());
            continue;
        }
        if *stmt == "end" {
            let b = open.take().ok_or_else(|| {
                Error::new("end without subgraph", line, raw.clone())
            })?;
            open_raw = None;
            boxes.push(b);
            continue;
        }
        let mut rest = stmt.as_str();
        let mut touched: Vec<String> = Vec::new();
        // Optional leading node definition or bare id.
        let mut current: Option<String> = None;
        if let Some((id, shape, label, r)) = parse_node(rest) {
            g.set_node(id.clone(), shape, label);
            current = Some(id.clone());
            touched.push(id);
            rest = r;
        } else if let Some(end) = rest.find(|c: char| !is_id_char(c)) {
            if end > 0 {
                current = Some(rest[..end].to_string());
                g.ensure_node(&rest[..end]);
                touched.push(rest[..end].to_string());
                rest = rest[end..].trim_start();
            }
        } else if !rest.is_empty() {
            g.ensure_node(rest);
            current = Some(rest.to_string());
            touched.push(rest.to_string());
            rest = "";
        }
        // Edge chain: (connector target)*.
        while !rest.trim_start().is_empty() {
            rest = rest.trim_start();
            let (style, label, r) = parse_edge(rest).ok_or_else(|| {
                Error::new(
                    format!("cannot parse edge in statement `{stmt}` near `{rest}`"),
                    line,
                    raw.clone(),
                )
            })?;
            rest = r;
            let target = if let Some((id, shape, label, r2)) = parse_node(rest) {
                g.set_node(id.clone(), shape, label);
                rest = r2;
                id
            } else {
                let end = rest.find(|c: char| !is_id_char(c)).unwrap_or(rest.len());
                if end == 0 {
                    return Err(Error::new(
                        format!("missing edge target in `{stmt}`"),
                        line,
                        raw.clone(),
                    ));
                }
                let id = rest[..end].to_string();
                g.ensure_node(&id);
                rest = rest[end..].trim_start();
                id
            };
            let from = current.clone().ok_or_else(|| {
                Error::new(format!("edge without source in `{stmt}`"), line, raw.clone())
            })?;
            g.edges.push(Edge {
                from: from.clone(),
                to: target.clone(),
                label,
                style,
                line: line,
                stmt: raw.clone(),
            });
            touched.push(from);
            touched.push(target.clone());
            current = Some(target);
        }
        // A lone node definition with no edges is already recorded.
        if let Some(b) = open.as_mut() {
            for t in touched {
                if !b.members.contains(&t) {
                    b.members.push(t);
                }
            }
        }
    }
    if let Some(b) = open {
        return Err(Error::new(
            "missing `end` for subgraph".to_string(),
            b.line,
            open_raw.unwrap_or_default(),
        ));
    }
    if g.nodes.is_empty() {
        return Err(Error::new("no nodes", base + first_idx, first_raw.clone()));
    }
    Ok((g, boxes))
}


/// Longest-path layering from the roots: a multi-parent node takes
/// max(parent depth)+1, so diamond joins sit below ALL their
/// parents (documented in the subset table). Relaxation converges
/// in under n passes on DAGs; cyclic depths stop at the n cap —
/// output still terminates, deterministically.
pub(crate) fn depths(g: &Graph) -> Vec<usize> {
    let n = g.nodes.len();
    let mut depth = vec![0usize; n];
    for _ in 0..n {
        let mut changed = false;
        for e in &g.edges {
            if let (Some(&a), Some(&b)) = (g.node_idx.get(&e.from), g.node_idx.get(&e.to)) {
                if a != b && depth[b] < depth[a] + 1 && depth[a] + 1 <= n {
                    depth[b] = depth[a] + 1;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    depth
}

/// First content line (comments stripped): the grammar dispatch.
fn first_stmt(src: &str) -> &str {
    src.lines()
        .map(|l| {
            let cut = l.find("%%").map(|i| &l[..i]).unwrap_or(l);
            cut.trim()
        })
        .find(|l| !l.is_empty())
        .unwrap_or("")
}

pub(crate) const DX: f64 = 4.2;
pub(crate) const DY: f64 = 2.4;

/// Deterministic layered coordinates: BFS depth from the roots,
/// siblings ordered by first appearance, explicit cm positions.
/// Shared with the state emitter (same placer, same parity).
pub(crate) fn layered_xy(g: &Graph, depth: &[usize]) -> Vec<(f64, f64)> {
    let max_d = depth.iter().copied().max().unwrap_or(0);
    let mut order: HashMap<usize, usize> = HashMap::new();
    let mut counters = vec![0usize; max_d + 1];
    let mut widths = vec![0usize; max_d + 1];
    for (i, _) in g.nodes.iter().enumerate() {
        widths[depth[i]] += 1;
    }
    for (i, _) in g.nodes.iter().enumerate() {
        let d = depth[i];
        order.insert(i, counters[d]);
        counters[d] += 1;
    }
    g.nodes
        .iter()
        .enumerate()
        .map(|(i, _)| match g.direction {
            Direction::Td => {
                let w = widths[depth[i]] as f64;
                let o = order[&i] as f64;
                ((o - (w - 1.0) / 2.0) * DX, -(depth[i] as f64) * DY)
            }
            Direction::Lr => {
                let w = widths[depth[i]] as f64;
                let o = order[&i] as f64;
                ((depth[i] as f64) * DX, -((o - (w - 1.0) / 2.0) * DY))
            }
        })
        .collect()
}

/// Full `tikzpicture` for a mermaid block: flowchart here,
/// `sequenceDiagram` / `stateDiagram-v2` dispatch to their
/// grammars on the header line (same entry point, same errors).
/// Rendered `tikzpicture`. `base_line` is the 1-based original line
/// of the body's first line (fence line + 1 from `convert()`, 1
/// for bare snippets) — every error below resolves against it.
pub fn flowchart_to_tikz(src: &str, base_line: usize) -> Result<String, crate::error::Error> {
    use crate::error::Error;
    if first_stmt(src) == "sequenceDiagram" {
        return crate::sequence::to_tikz(src, base_line);
    }
    if first_stmt(src) == "stateDiagram-v2" {
        return crate::state::to_tikz(src, base_line);
    }
    let (g, boxes) = parse(src, base_line)?;
    let depth = depths(&g);
    let xy = layered_xy(&g, &depth);

    let mut out = String::from(crate::emit::PICTURE_HEAD);
    for (i, n) in g.nodes.iter().enumerate() {
        let (x, y) = xy[i];
        let style = match n.shape {
            Shape::Rect => "box",
            Shape::Diamond => "dia",
            Shape::Stadium => "stad",
            Shape::Subroutine => "sub",
        };
        out.push_str(&tikz_node(&n.id, x, y, style, &n.label));
    }
    // Subgraph boxes second (layout frozen), edges last so lines
    // overlay box borders instead of hiding under them.
    for sg in &boxes {
        let pts: Vec<(f64, f64)> = sg
            .members
            .iter()
            .filter_map(|m| g.node_idx.get(m).map(|&i| xy[i]))
            .collect();
        let Some((r, at)) = crate::layout::cluster_box(&pts) else {
            return Err(Error::new(
                format!("subgraph `{}` has no placed members", sg.id),
                sg.line,
                sg.stmt.clone(),
            ));
        };
        out.push_str(&crate::emit::cluster_rect(&sg.title, r, at));
    }
    let node_boxes: Vec<(String, (f64, f64, f64, f64))> = g
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.id.clone(), crate::layout::node_box(xy[i].0, xy[i].1)))
        .collect();
    for e in &g.edges {
        let style = match e.style {
            EdgeStyle::Arrow => "->",
            EdgeStyle::Line => "-",
            EdgeStyle::Thick => "->,thick",
        };
        let (ai, bi) = match (g.node_idx.get(&e.from), g.node_idx.get(&e.to)) {
            (Some(&a), Some(&b)) => (a, b),
            _ => {
                return Err(Error::new(
                    format!("edge endpoints missing for `{}->{}`", e.from, e.to),
                    e.line,
                    e.stmt.clone(),
                ))
            }
        };
        // The router returns None for clear segments: the emitted
        // line is then the exact historical one (golden parity).
        match crate::layout::route_edge(&e.from, &e.to, xy[ai], xy[bi], &node_boxes, e.line, &e.stmt)? {
            None => {
                if let Some(lbl) = &e.label {
                    out.push_str(&format!(
                        "  \\draw[{style}] ({}) -- ({}) {};\n",
                        e.from,
                        e.to,
                        edge_label_node(lbl)
                    ));
                } else {
                    out.push_str(&format!(
                        "  \\draw[{style}] ({}) -- ({});\n",
                        e.from, e.to
                    ));
                }
            }
            Some((mx, my)) => {
                out.push_str(&format!(
                    "  \\draw[{style}] ({}) -- ({mx:.1},{my:.1}) -- ({});\n",
                    e.from, e.to
                ));
                if let Some(lbl) = &e.label {
                    out.push_str(&format!(
                        "  {}\n",
                        crate::emit::placed_label_node(lbl, mx, my)
                    ));
                }
            }
        }
    }
    out.push_str("\\end{tikzpicture}\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ_Q: &str = "graph TD\nq{\"Which question?\"}\nq -->|\"true at v\"| av[\"as_of_valid(v)<br/>L's current state\"]\nav -->|compose| both[\"both instants\"]\nar -->|compose| both";

    #[test]
    fn read_questions_shape() {
        let tikz = flowchart_to_tikz(READ_Q, 1).unwrap();
        assert!(tikz.contains("\\begin{tikzpicture}"));
        assert!(tikz.contains("\\node (q)"));
        assert!(tikz.contains("[dia]"), "diamond, got:\n{tikz}");
        assert!(tikz.contains("as\\_of\\_valid(v)\\\\L's current state"), "break kept, got:\n{tikz}");
        assert!(tikz.contains("node[midway,above"), "edge label, got:\n{tikz}");
        // The ar→both edge used to strike straight through av's box;
        // the router reroutes it (one polyline \draw + one placed
        // label node), so \node counts 5, not 4.
        assert_eq!(tikz.matches("\\node").count(), 5);
        assert_eq!(tikz.matches("\\draw").count(), 3);
        assert!(
            tikz.lines().filter(|l| l.contains("\\draw")).any(|l| l.matches("--").count() == 2),
            "one rerouted polyline, got:\n{tikz}"
        );
    }

    #[test]
    fn lr_and_shapes() {
        let tikz = flowchart_to_tikz(
            "flowchart LR\n  a([start]) --> b{decide} --> c[[run]]\n  b --- c", 1)
        .unwrap();
        assert!(tikz.contains("[stad]"));
        assert!(tikz.contains("[sub]"));
        assert!(tikz.contains("\\draw[-]"), "plain line, got:\n{tikz}");
    }

    #[test]
    fn errors_are_strings() {
        assert!(flowchart_to_tikz("", 1).is_err());
        assert!(flowchart_to_tikz("digraph G { a -> b }", 1).is_err());
        assert!(flowchart_to_tikz("graph TD\nq -->", 1).is_err());
    }

    #[test]
    fn error_unknown_statement_carries_line() {
        // Body lines 0-2, base 10: the bad statement is line 12.
        let e = flowchart_to_tikz("graph TD\na[x]\nbogus !!\n", 10).unwrap_err();
        assert_eq!(e.line, 12, "got: {e}");
        assert!(e.echo.contains("bogus"), "got: {e}");
        assert!(e.to_string().contains(" --> line 12"), "got: {e}");
    }

    #[test]
    fn error_missing_target_carries_line() {
        let e = flowchart_to_tikz("graph TD\na[x]-->\n", 5).unwrap_err();
        assert!(e.msg.contains("missing edge target"), "got: {e}");
        assert_eq!(e.line, 6, "got: {e}");
    }

    #[test]
    fn error_empty_block_carries_base() {
        let e = flowchart_to_tikz("", 3).unwrap_err();
        assert_eq!(e.line, 3, "got: {e}");
    }

    #[test]
    fn comments_and_semicolons() {
        let tikz = flowchart_to_tikz("graph TD\n%% hi\na[x]; b[y]; a-->b", 1).unwrap();
        assert_eq!(tikz.matches("\\node").count(), 2);
    }

    #[test]
    fn subgraph_box_math() {
        let tikz = flowchart_to_tikz(
            "graph TD\nsubgraph ours[Our box]\na[x]-->b[y]\nend\nb-->c[z]", 1)
        .unwrap();
        assert!(tikz.contains("rectangle"), "box, got:\n{tikz}");
        assert!(tikz.contains("{Our box}"), "title, got:\n{tikz}");
        // c is outside: the box wraps placed a/b only.
        assert_eq!(tikz.matches("rectangle").count(), 1);
    }

    #[test]
    fn subgraph_nesting_is_an_error() {
        assert!(flowchart_to_tikz("graph TD\nsubgraph a\nsubgraph b\nx[y]\nend\nend", 1).is_err());
        assert!(flowchart_to_tikz("graph TD\na[x]\nend", 1).is_err());
        assert!(flowchart_to_tikz("graph TD\nsubgraph a\na[x]", 1).is_err());
    }

    #[test]
    fn diamond_layering_max_plus_one() {
        let tikz = flowchart_to_tikz(
            "graph TD\na[x]-->b[y]\na-->c[z]\nb-->d[w]\nc-->d", 1)
        .unwrap();
        // d sits below BOTH parents (depth 2), not at first-visit 1.
        assert!(tikz.contains("\\node (d) at (0.0,-4.8)"), "join at max+1, got:\n{tikz}");
    }

    #[test]
    fn router_triggers_on_crossing() {
        let tikz = flowchart_to_tikz("graph TD\na[x]-->b[y]\na-->c[z]\nb-->c", 1).unwrap();
        assert!(
            tikz.lines().filter(|l| l.contains("\\draw")).any(|l| l.matches("--").count() == 2),
            "a→c reroutes around b, got:\n{tikz}"
        );
    }

    #[test]
    fn clean_graphs_have_no_polylines() {
        // No dashes in labels (an em-dash would read as `--`).
        let tikz = flowchart_to_tikz("graph TD\na[alpha]-->b[beta]\nb-->c[gamma]", 1).unwrap();
        for l in tikz.lines().filter(|l| l.contains("\\draw")) {
            assert_eq!(l.matches("--").count(), 1, "straight edge untouched: {l}");
        }
    }

    #[test]
    fn layout_determinism_across_runs() {
        let src = "graph TD\na[x]-->b[y]\nb-->c[z]\na-->c\nsubgraph s\na\nend";
        assert_eq!(flowchart_to_tikz(src, 1).unwrap(), flowchart_to_tikz(src, 1).unwrap());
    }

    #[test]
    fn wide_graph_layer_bound() {
        // 12-chain: depths stay in range, output terminates.
        let mut src = String::from("graph TD\n");
        for i in 0..12 {
            src.push_str(&format!("n{i}[N{i}]\n"));
        }
        for i in 0..11 {
            src.push_str(&format!("n{i}-->n{}\n", i + 1));
        }
        let tikz = flowchart_to_tikz(&src, 1).unwrap();
        assert!(tikz.contains("\\node (n11) at (0.0,-26.4)"), "depth 11 placed, got tail:\n{}", &tikz[tikz.len().saturating_sub(300)..]);
    }
}
