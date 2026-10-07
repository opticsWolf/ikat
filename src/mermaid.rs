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

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Td,
    Lr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rect,
    Diamond,
    Stadium,
    Subroutine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeStyle {
    Arrow,
    Line,
    Thick,
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    label: String,
    shape: Shape,
}

#[derive(Debug, Clone)]
struct Edge {
    from: String,
    to: String,
    label: Option<String>,
    style: EdgeStyle,
}

/// Escape a mermaid label for LaTeX text mode.
/// Order matters: entities, then stash line breaks, then escape
/// specials, then restore breaks (else `\` gets escaped too).
fn esc_label(s: &str) -> String {
    const BR: char = '\u{E000}';
    let mut s = s
        .replace("<br/>", &BR.to_string())
        .replace("<br>", &BR.to_string())
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&");
    for (ch, rep) in [
        ('\\', "\\textbackslash{}"),
        ('&', "\\&"),
        ('%', "\\%"),
        ('$', "\\$"),
        ('#', "\\#"),
        ('_', "\\_"),
        ('{', "\\{"),
        ('}', "\\}"),
        ('~', "\\textasciitilde{}"),
        ('^', "\\textasciicircum{}"),
    ] {
        s = s.replace(ch, rep);
    }
    s = s.replace(BR, "\\\\");
    // Common prose glyphs mermaid authors actually type.
    for (ch, rep) in [
        ('×', "$\\times$"),
        ('→', "$\\to$"),
        ('–', "--"),
        ('—', "---"),
        ('≤', "$\\leq$"),
        ('≥', "$\\geq$"),
        ('✓', "\\checkmark{}"),
    ] {
        s = s.replace(ch, rep);
    }
    s
}

fn is_id_char(c: char) -> bool {
    // NOTE: `-` is deliberately excluded: `a-->b` must lex the edge,
    // not swallow `--` into the id.
    c.is_alphanumeric() || c == '_'
}

/// Mermaid lets label text wear one layer of double quotes; take it off.
fn unquote(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2 && t.starts_with('"') && t.ends_with('"') {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
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

struct Graph {
    direction: Direction,
    nodes: Vec<Node>,
    node_idx: HashMap<String, usize>,
    edges: Vec<Edge>,
}

impl Graph {
    fn ensure_node(&mut self, id: &str) {
        if !self.node_idx.contains_key(id) {
            self.node_idx.insert(id.to_string(), self.nodes.len());
            self.nodes.push(Node {
                id: id.to_string(),
                label: id.to_string(),
                shape: Shape::Rect,
            });
        }
    }

    fn set_node(&mut self, id: String, shape: Shape, label: String) {
        self.ensure_node(&id);
        let i = self.node_idx[&id];
        self.nodes[i] = Node { id, label, shape };
    }
}

fn parse(src: &str) -> Result<Graph, String> {
    let mut lines = src
        .lines()
        .map(|l| {
            // Strip %% comments.
            let cut = l.find("%%").map(|i| &l[..i]).unwrap_or(l);
            cut.trim()
        })
        .filter(|l| !l.is_empty())
        .peekable();

    let first = lines.next().ok_or("empty mermaid block")?;
    let dir_word = first
        .strip_prefix("graph")
        .or_else(|| first.strip_prefix("flowchart"))
        .ok_or("first line must be `graph TD/LR/...`")?
        .trim();
    let direction = match dir_word {
        "TD" | "TB" => Direction::Td,
        "LR" | "RL" => Direction::Lr,
        "BT" => Direction::Td, // laid top-down; BT flip is cosmetic
        _ => return Err(format!("unsupported direction `{dir_word}`")),
    };

    let mut g = Graph {
        direction,
        nodes: Vec::new(),
        node_idx: HashMap::new(),
        edges: Vec::new(),
    };

    // Statements split on ';' (newlines already separate chains).
    let stmts: Vec<String> = lines.flat_map(|l| l.split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string)).collect();
    if stmts.is_empty() {
        return Err("no statements".to_string());
    }

    for stmt in &stmts {
        let mut rest = stmt.as_str();
        // Optional leading node definition or bare id.
        let mut current: Option<String> = None;
        if let Some((id, shape, label, r)) = parse_node(rest) {
            g.set_node(id.clone(), shape, label);
            current = Some(id);
            rest = r;
        } else if let Some(end) = rest.find(|c: char| !is_id_char(c)) {
            if end > 0 {
                current = Some(rest[..end].to_string());
                g.ensure_node(&rest[..end]);
                rest = rest[end..].trim_start();
            }
        } else if !rest.is_empty() {
            g.ensure_node(rest);
            current = Some(rest.to_string());
            rest = "";
        }
        // Edge chain: (connector target)*.
        while !rest.trim_start().is_empty() {
            rest = rest.trim_start();
            let (style, label, r) = parse_edge(rest).ok_or_else(|| {
                format!("cannot parse edge in statement `{stmt}` near `{rest}`")
            })?;
            rest = r;
            let target = if let Some((id, shape, label, r2)) = parse_node(rest) {
                g.set_node(id.clone(), shape, label);
                rest = r2;
                id
            } else {
                let end = rest.find(|c: char| !is_id_char(c)).unwrap_or(rest.len());
                if end == 0 {
                    return Err(format!("missing edge target in `{stmt}`"));
                }
                let id = rest[..end].to_string();
                g.ensure_node(&id);
                rest = rest[end..].trim_start();
                id
            };
            let from = current.clone().ok_or_else(|| format!("edge without source in `{stmt}`"))?;
            g.edges.push(Edge { from: from.clone(), to: target.clone(), label, style });
            current = Some(target);
        }
        // A lone node definition with no edges is already recorded.
    }
    if g.nodes.is_empty() {
        return Err("no nodes".to_string());
    }
    Ok(g)
}

/// BFS depth from roots (nodes with no incoming edge).
fn depths(g: &Graph) -> Vec<usize> {
    let mut incoming = vec![0usize; g.nodes.len()];
    for e in &g.edges {
        if let (Some(&a), Some(&b)) = (g.node_idx.get(&e.from), g.node_idx.get(&e.to)) {
            if a != b {
                incoming[b] += 1;
            }
        }
    }
    let mut depth = vec![usize::MAX; g.nodes.len()];
    let mut q = VecDeque::new();
    for (i, &inc) in incoming.iter().enumerate() {
        if inc == 0 {
            depth[i] = 0;
            q.push_back(i);
        }
    }
    if q.is_empty() {
        // Cycle: seed from the first node so output still terminates.
        depth[0] = 0;
        q.push_back(0);
    }
    while let Some(i) = q.pop_front() {
        for e in &g.edges {
            if g.node_idx.get(&e.from) == Some(&i) {
                if let Some(&j) = g.node_idx.get(&e.to) {
                    if depth[j] == usize::MAX {
                        depth[j] = depth[i] + 1;
                        q.push_back(j);
                    }
                }
            }
        }
    }
    for d in depth.iter_mut() {
        if *d == usize::MAX {
            *d = 0;
        }
    }
    depth
}

/// Full `tikzpicture` for a mermaid flowchart block.
pub fn flowchart_to_tikz(src: &str) -> Result<String, String> {
    let g = parse(src)?;
    let depth = depths(&g);

    // Order siblings by first appearance.
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

    const DX: f64 = 4.2;
    const DY: f64 = 2.4;
    let mut out = String::from(
        "\\begin{tikzpicture}[>=Stealth,\n  every node/.style={align=center,font=\\small},\n  box/.style={draw,rounded corners=2pt,fill=gray!8},\n  dia/.style={draw,diamond,aspect=2,fill=blue!8},\n  stad/.style={draw,rounded corners=10pt,fill=gray!8},\n  sub/.style={draw,double,fill=gray!8}]\n",
    );
    for (i, n) in g.nodes.iter().enumerate() {
        let (x, y) = match g.direction {
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
        };
        let style = match n.shape {
            Shape::Rect => "box",
            Shape::Diamond => "dia",
            Shape::Stadium => "stad",
            Shape::Subroutine => "sub",
        };
        out.push_str(&format!(
            "  \\node ({}) at ({:.1},{:.1}) [{style}] {{{}}};\n",
            n.id,
            x,
            y,
            esc_label(&n.label)
        ));
    }
    for e in &g.edges {
        let style = match e.style {
            EdgeStyle::Arrow => "->",
            EdgeStyle::Line => "-",
            EdgeStyle::Thick => "->,thick",
        };
        if let Some(lbl) = &e.label {
            out.push_str(&format!(
                "  \\draw[{style}] ({}) -- ({}) node[midway,above,fill=white,inner sep=1pt,font=\\footnotesize] {{{}}};\n",
                e.from,
                e.to,
                esc_label(lbl)
            ));
        } else {
            out.push_str(&format!("  \\draw[{style}] ({}) -- ({});\n", e.from, e.to));
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
        let tikz = flowchart_to_tikz(READ_Q).unwrap();
        assert!(tikz.contains("\\begin{tikzpicture}"));
        assert!(tikz.contains("\\node (q)"));
        assert!(tikz.contains("[dia]"), "diamond, got:\n{tikz}");
        assert!(tikz.contains("as\\_of\\_valid(v)\\\\L's current state"), "break kept, got:\n{tikz}");
        assert!(tikz.contains("node[midway,above"), "edge label, got:\n{tikz}");
        assert_eq!(tikz.matches("\\node").count(), 4);
        assert_eq!(tikz.matches("\\draw").count(), 3);
    }

    #[test]
    fn lr_and_shapes() {
        let tikz = flowchart_to_tikz(
            "flowchart LR\n  a([start]) --> b{decide} --> c[[run]]\n  b --- c",
        )
        .unwrap();
        assert!(tikz.contains("[stad]"));
        assert!(tikz.contains("[sub]"));
        assert!(tikz.contains("\\draw[-]"), "plain line, got:\n{tikz}");
    }

    #[test]
    fn errors_are_strings() {
        assert!(flowchart_to_tikz("").is_err());
        assert!(flowchart_to_tikz("digraph G { a -> b }").is_err());
        assert!(flowchart_to_tikz("graph TD\nq -->").is_err());
    }

    #[test]
    fn comments_and_semicolons() {
        let tikz = flowchart_to_tikz("graph TD\n%% hi\na[x]; b[y]; a-->b").unwrap();
        assert_eq!(tikz.matches("\\node").count(), 2);
    }
}
