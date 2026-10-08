//! Shared TikZ emission helpers for the diagram grammars.
//!
//! Lifted verbatim from `mermaid.rs` (the flowchart grammar) so the
//! sequence/state emitters share the exact label knockout and node
//! vocabulary — byte parity of existing output is the proof the
//! move was mechanical.

/// Escape a mermaid label for LaTeX text mode.
/// Order matters: entities, then stash line breaks, then escape
/// specials, then restore breaks (else `\` gets escaped too).
pub fn esc_label(s: &str) -> String {
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

/// `\node (id) at (x,y) [style] {label};` — one box on the canvas.
pub fn tikz_node(id: &str, x: f64, y: f64, style: &str, label: &str) -> String {
    format!(
        "  \\node ({}) at ({:.1},{:.1}) [{style}] {{{}}};\n",
        id,
        x,
        y,
        esc_label(label)
    )
}

/// Cluster/composite box: rect + inside-top label (the box-label
/// rule: half-row headroom above the top member, label tucked just
/// inside — outside-above struck edge labels in review).
pub fn cluster_rect(text: &str, r: (f64, f64, f64, f64), at: (f64, f64)) -> String {
    format!(
        "  \\draw ({:.1},{:.1}) rectangle ({:.1},{:.1});\n  \\node[anchor=north west,font=\\footnotesize\\itshape] at ({:.1},{:.1}) {{{}}};\n",
        r.0, r.1, r.2, r.3, at.0, at.1, esc_label(text)
    )
}

/// Mid-edge label with the white knockout (text never struck).
pub fn edge_label_node(lbl: &str) -> String {
    format!(
        "node[midway,above,fill=white,inner sep=1pt,font=\\footnotesize] {{{}}}",
        esc_label(lbl)
    )
}

/// The shared `tikzpicture` head: arrow tip, base node style, and
/// the box/diamond/stadium/subroutine vocabulary. One literal, all
/// grammars — a style drift would break golden parity loudly.
pub const PICTURE_HEAD: &str =
    "\\begin{tikzpicture}[>=Stealth,\n  every node/.style={align=center,font=\\small},\n  box/.style={draw,rounded corners=2pt,fill=gray!8},\n  dia/.style={draw,diamond,aspect=2,fill=blue!8},\n  stad/.style={draw,rounded corners=10pt,fill=gray!8},\n  sub/.style={draw,double,fill=gray!8}]\n";

/// Knockout label pinned at an absolute point (rerouted edges:
/// the midpoint carries the label instead of `midway`).
pub fn placed_label_node(lbl: &str, x: f64, y: f64) -> String {
    format!(
        "\\node[fill=white,inner sep=1pt,font=\\footnotesize] at ({x:.1},{y:.1}) {{{}}}",
        esc_label(lbl)
    )
}

/// Mermaid lets label text wear one layer of double quotes; take it off.
pub fn unquote(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2 && t.starts_with('"') && t.ends_with('"') {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}
