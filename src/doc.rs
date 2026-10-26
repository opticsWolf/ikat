//! Block parsing and document assembly: headings, fences, tables,
//! paragraphs, abstract, appendix, preamble. Byte-parity with the
//! paper build's `convert()` + `main()` is the contract.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::config::{CaptionPos, Config, FloatNeeds, Pos, Span};
use crate::esc::inline;
use crate::mathwidth::display_math_width_cm;
use crate::md::MdKind;
use crate::mermaid::flowchart_to_tikz;
use crate::table::table_block;

/// One mermaid fence, in source order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramEntry {
    pub key: String,
    pub caption: String,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_prefix")]
    pub path_prefix: String,
}

fn default_mode() -> String {
    "precompiled".to_string()
}
fn default_prefix() -> String {
    "figs/tikz/".to_string()
}

/// Everything document-specific that ikat.toml doesn't cover.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildSpec {
    #[serde(default)]
    pub diagrams: Vec<DiagramEntry>,
    #[serde(default)]
    pub plots: Vec<(String, String)>,
    /// Per-plot `{k=v}` attrs by plot key (serde default: none).
    #[serde(default)]
    pub plot_attrs: HashMap<String, String>,
    #[serde(default = "default_plot_dir")]
    pub plot_dir: String,
    #[serde(default)]
    pub table_captions: Vec<String>,
    #[serde(default)]
    pub table_specs: HashMap<String, String>,
    #[serde(default)]
    pub plot_insert_before: String,
    #[serde(default)]
    pub title_thanks: String,
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(default = "default_bib")]
    pub bib_name: String,
    #[serde(default = "default_paths")]
    pub graphicspaths: Vec<String>,
    #[serde(default)]
    pub bib_keys: HashSet<String>,
    /// Literal `.tex` head replacing the generated preamble (file
    /// *content* of `[template] preamble_file`; empty = none).
    #[serde(default)]
    pub preamble_override: String,
    /// Whole-document skeleton (file *content* of `[template]`
    /// `skeleton`); mutually exclusive with `preamble_override`.
    /// Empty = level 0/1 path.
    #[serde(default)]
    pub skeleton: String,
    /// Extra preamble lines, appended just before `\begin{document}`.
    #[serde(default)]
    pub preamble_append: Vec<String>,
    /// `\bibliographystyle` name (publisher templates override it).
    #[serde(default = "default_bibstyle")]
    pub bib_style: String,
}

fn default_plot_dir() -> String {
    "figs/".to_string()
}
fn default_author() -> String {
    "opticsWolf".to_string()
}
fn default_bibstyle() -> String {
    "IEEEtran".to_string()
}
fn default_bib() -> String {
    "refs-paper".to_string()
}
fn default_paths() -> Vec<String> {
    ["./", "figs/", "figs/tikz/"].iter().map(|s| s.to_string()).collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub title: String,
    pub body: String,
    pub tex: String,
    pub n_diagrams: usize,
    pub n_tables: usize,
}

/// Render a validated width against a span width: fractions scale
/// it (`0.8` of `\columnwidth`), anything else passes through.
fn width_to_tex(width: Option<&str>, span_w: &str) -> String {
    match width {
        None => span_w.to_string(),
        Some(f) if f.parse::<f64>().map(|v| v > 0.0 && v <= 2.0).unwrap_or(false) => {
            format!("{f}{span_w}")
        }
        Some(abs) => abs.to_string(),
    }
}

/// Per-element float options from a `{k=v ...}` attribute string
/// (fence info lines, `%% table {...}` comments, plot registry).
/// Unknown keys and bad values are errors — strict subset doctrine.
#[derive(Debug, Clone, Default)]
pub struct ElemAttrs {
    pub span: Option<Span>,
    pub pos: Option<Pos>,
    pub width: Option<String>,
    pub captionpos: Option<CaptionPos>,
}

/// A width is a fraction of the span width (`0.8`) or a TeX length
/// (`5cm`, `\columnwidth`, ...). Anything else is an error.
fn parse_width(s: &str) -> Result<String, String> {
    if let Ok(v) = s.parse::<f64>() {
        if v > 0.0 && v <= 2.0 {
            return Ok(s.to_string());
        }
        return Err(format!("width fraction must be in (0, 2], got {s:?}"));
    }
    let len_re =
        regex::Regex::new(r"^[0-9.]+(cm|mm|in|pt|pc|bp|dd|cc|sp|em|ex)$|\\").unwrap();
    if len_re.is_match(s) {
        return Ok(s.to_string());
    }
    Err(format!("width must be a fraction or TeX length, got {s:?}"))
}

/// Attribute parser. `line` is the 1-based source line of the attr
/// string (fence info or directive line); the echo is the attr
/// string itself — short, and it always contains the bad token.
pub fn parse_attrs(attr: &str, what: &str, line: usize) -> Result<(ElemAttrs, bool), crate::error::Error> {
    use crate::error::Error;
    let mut out = ElemAttrs::default();
    let mut inline = false;
    for tok in attr.split_whitespace() {
        if tok == "{inline}" || tok == "inline" {
            inline = true;
            continue;
        }
        // Caret column of this token inside the attr string (char
        // columns; leading whitespace inside braces counts — the
        // classic off-by-indent lives here).
        let tok_col = attr.find(tok).map(|b| crate::error::col_of(attr, b));
        let Some((k, v)) = tok.split_once('=') else {
            return Err(Error::new(
                format!("{what}: bad attribute {tok:?} (want k=v)"),
                line,
                attr.to_string()
            ).with_col(tok_col));
        };
        match k {
            "span" => {
                out.span = Some(match v {
                    "column" => Span::Column,
                    "wide" => Span::Wide,
                    _ => {
                        return Err(Error::new(
                            format!("{what}: span must be column|wide, got {v:?}"),
                            line,
                            attr.to_string(),
                        ).with_col(tok_col))
                    }
                })
            }
            "pos" => {
                out.pos = Some(Pos::parse(v).map_err(|e| {
                    Error::new(format!("{what}: {e}"), line, attr.to_string()).with_col(tok_col)
                })?)
            }
            "width" => {
                out.width = Some(parse_width(v).map_err(|e| {
                    Error::new(format!("{what}: {e}"), line, attr.to_string()).with_col(tok_col)
                })?)
            }
            "captionpos" => {
                out.captionpos = Some(
                    CaptionPos::parse(v).map_err(|e| {
                        Error::new(format!("{what}: {e}"), line, attr.to_string()).with_col(tok_col)
                    })?,
                )
            }
            _ => {
                return Err(Error::new(
                    format!("{what}: unknown attribute {k:?} (span|pos|width|captionpos)"),
                    line,
                    attr.to_string(),
                ).with_col(tok_col))
            }
        }
    }
    Ok((out, inline))
}

/// Extract the `{...}` attribute string from a fence info line.
pub(crate) fn fence_attr_str(line: &str) -> &str {
    match (line.find('{'), line.rfind('}')) {
        (Some(a), Some(b)) if b > a => &line[a + 1..b],
        _ => "",
    }
}

fn figure_block(
    body_tex: &str,
    caption: &str,
    label: &str,
    span: Span,
    pos: Pos,
    width: Option<&str>,
    caption_bottom: bool,
) -> Result<(String, FloatNeeds), String> {
    let mut needs = FloatNeeds::none();
    let wide = matches!(span, Span::Wide);
    if wide && matches!(pos, Pos::Here | Pos::Force) {
        return Err("figure*: pos here/force is illegal on full-width floats (use span=column)".to_string());
    }
    if wide && matches!(pos, Pos::Bottom | Pos::Both) {
        needs.dblfloat = true;
    }
    if matches!(pos, Pos::Force) {
        needs.float_h = true;
    }
    let barrier = matches!(pos, Pos::Barrier);
    if barrier {
        needs.barrier = true;
    }
    let (env, span_w) = match span {
        Span::Column => ("figure", "\\columnwidth"),
        Span::Wide => ("figure*", "\\textwidth"),
    };
    let is_tikz = body_tex.trim_start().starts_with("\\begin{tikzpicture}");
    let body = if is_tikz {
        match width {
            None => body_tex.to_string(),
            Some(f) if f.parse::<f64>().is_ok() && !body_tex.contains("[scale=") => {
                if body_tex.contains("\\begin{tikzpicture}[") {
                    body_tex.replacen(
                        "\\begin{tikzpicture}[",
                        &format!("\\begin{{tikzpicture}}[scale={f},"),
                        1,
                    )
                } else {
                    body_tex.replacen(
                        "\\begin{tikzpicture}",
                        &format!("\\begin{{tikzpicture}}[scale={f}]"),
                        1,
                    )
                }
            }
            _ => {
                needs.graphicx = true;
                let w = width_to_tex(width, span_w);
                format!("\\resizebox{{{w}}}{{!}}{{{body_tex}}}")
            }
        }
    } else if body_tex.trim_start().starts_with("\\includegraphics") {
        body_tex.to_string()
    } else {
        format!("\\includegraphics[width={}]{{{body_tex}}}", width_to_tex(width, span_w))
    };
    let caption_lines = format!("\\caption{{{caption}}}\n\\label{{{label}}}");
    let inner = if caption_bottom {
        format!("{body}\n{caption_lines}")
    } else {
        format!("{caption_lines}\n{body}")
    };
    let prefix = if barrier { "\\FloatBarrier\n" } else { "" };
    Ok((
        format!("{prefix}\\begin{{{env}}}{}\n\\centering\n{inner}\n\\end{{{env}}}", pos.latex_spec()),
        needs,
    ))
}

fn strip_section_number(head: &str) -> String {
    // Strip a leading `N. ` or `N.M ` marker (IEEE classes number
    // sections themselves). Anything else passes through untouched.
    let b = head.as_bytes();
    let mut i = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i > 0 && i < b.len() && b[i] == b'.' {
        let mut j = i + 1;
        let mut k = j;
        while k < b.len() && b[k].is_ascii_digit() {
            k += 1;
        }
        if k > j {
            j = k;
        }
        if j < b.len() && b[j] == b' ' {
            return head[j + 1..].to_string();
        }
    }
    head.to_string()
}

/// Split a woven body's abstract env out for top-matter classes
/// (ACM, Elsevier, APS): returns `(abstract_block, body_without)`;
/// empty abstract when the body has none.
pub fn split_abstract(body: &str) -> (String, String) {
    let (Some(start), Some(end)) = (body.find("\\begin{abstract}"), body.find("\\end{abstract}")) else {
        return (String::new(), body.to_string());
    };
    let end = end + "\\end{abstract}".len();
    if end <= start {
        return (String::new(), body.to_string());
    }
    let (head, tail) = body.split_at(start);
    let (abs, rest) = tail.split_at(end - start);
    (abs.to_string(), format!("{head}{}", rest.trim_start_matches('\n')))
}

/// Fill the shipped-template tokens from the build: `{{title}}`
/// from the manuscript H1, `{{author}}` / `{{thanks}}` from the
/// spec. Only applied to `preamble_override` content.
pub fn render_override(override_tex: &str, title: &str, author: &str, thanks: &str) -> String {
    let rendered = override_tex.replace("{{title}}", title).replace("{{author}}", author).replace("{{thanks}}", thanks);
    // An emptied \thanks{} drops the whole title in IEEEtran;
    // an empty thanks must vanish, not render.
    rendered.replace("\\thanks{}", "")
}

/// Split a line into (code, comment) at the first unescaped `%`.
/// The comment keeps its `%`; a line without one yields `""`.
fn split_comment(line: &str) -> (&str, &str) {
    let mut search = 0;
    while let Some(at) = line[search..].find('%') {
        let at = search + at;
        if at > 0 && line.as_bytes()[at - 1] == b'\\' {
            search = at + 1;
            continue;
        }
        return (&line[..at], &line[at..]);
    }
    (line, "")
}

/// Strip TeX `%` comments (unescaped `%` to end of line) for
/// token COUNTING: a `{{body}}` in a comment is documentation,
/// not a second body.
fn strip_tex_comments(skel: &str) -> String {
    skel.lines()
        .map(|line| split_comment(line).0)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Fill a LEVEL-3 skeleton: complete `.tex` with `{{tokens}}`.
/// `{{title}}/{{author}}/{{thanks}}` fill as level 1;
/// 1-based line number of the byte index inside a head/skeleton
/// file (template errors are head-relative; Python attributes the
/// head file — see `document._with_file`).
fn head_line(tex: &str, byte_idx: usize) -> usize {
    tex[..byte_idx.min(tex.len())].matches('\n').count() + 1
}

/// Line + echo + caret for a `{{token}}` inside a skeleton: the
/// token's own line/column when present, else line 1 with no
/// position and an empty echo.
fn tok_line(skel: &str, tok: &str) -> (usize, String, Option<usize>) {
    let pat = format!("{{{{{tok}}}}}");
    match skel.find(&pat) {
        Some(_) => {
            let line = skel.lines().find(|l| l.contains(&pat)).unwrap_or("");
            let ln = head_line(skel, skel.find(&pat).unwrap_or(0));
            let col = line.find(&pat).map(|b| crate::error::col_of(line, b));
            (ln, line.trim().to_string(), col)
        }
        None => (1, String::new(), None),
    }
}

/// `{{body}}` is required exactly once; `{{bibliography}}` is
/// required exactly once unless `bib` is empty (biblatex-style
/// skeletons print refs themselves); `{{abstract}}` is required
/// exactly once when `abstract_tex` is non-empty, else renders
/// empty. Unknown `{{word}}` tokens and duplicates are errors —
/// a typo'd token must fail here, not as a silent blank page.
pub fn render_skeleton(
    skel: &str,
    title: &str,
    author: &str,
    thanks: &str,
    abstract_tex: &str,
    body: &str,
    bib: &str,
) -> Result<String, crate::error::Error> {
    let tok_re = regex::Regex::new(r"\{\{([A-Za-z_]+)\}\}").unwrap();
    let countable = strip_tex_comments(skel);
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for cap in tok_re.captures_iter(&countable) {
        *counts.entry(cap.get(1).unwrap().as_str()).or_default() += 1;
    }
    for tok in counts.keys() {
        match *tok {
            "title" | "author" | "thanks" | "body" | "abstract" | "bibliography" => {}
            _ => {
                let (ln, echo, col) = tok_line(skel, tok);
                return Err(crate::error::Error::new(
                    format!("skeleton has unknown token {{{{{tok}}}}}"),
                    ln,
                    echo,
                ).with_col(col));
            }
        }
    }
    let count = |tok: &str| counts.get(tok).copied().unwrap_or(0);
    // `{{body}}` is the point of the skeleton: exactly once.
    // `{{bibliography}}` / `{{abstract}}` are required exactly
    // once when they carry content; when empty they may appear
    // at most once and render as nothing (forward-compatible
    // skeletons for manuscripts gaining refs/abstracts later).
    if count("body") != 1 {
        let (ln, echo, _) = tok_line(skel, "body");
        return Err(crate::error::Error::new("skeleton needs {{body}} exactly once", ln, echo));
    }
    for (tok, content) in [("bibliography", bib), ("abstract", abstract_tex)] {
        let n = count(tok);
        if !content.is_empty() && n != 1 {
            let (ln, echo, _) = tok_line(skel, tok);
            return Err(crate::error::Error::new(
                format!("skeleton needs {{{{{tok}}}}} exactly once"),
                ln,
                echo,
            ));
        }
        if content.is_empty() && n > 1 {
            let (ln, echo, _) = tok_line(skel, tok);
            return Err(crate::error::Error::new(
                format!("skeleton has {{{{{tok}}}}} {n} times (max once)"),
                ln,
                echo,
            ));
        }
    }
    // Absent-but-empty tokens render as nothing, so skeletons
    // stay forward-compatible with manuscripts gaining refs later.
    let rendered = skel
        .lines()
        .map(|line| {
            // Tokens substitute in code only; comments documenting
            // the token contract keep their literal `{{names}}`.
            let (code, comment) = split_comment(line);
            let filled = tok_re
                .replace_all(code, |cap: &regex::Captures| match cap.get(1).unwrap().as_str() {
                    "title" => title.to_string(),
                    "author" => author.to_string(),
                    "thanks" => thanks.to_string(),
                    "abstract" => abstract_tex.to_string(),
                    "body" => body.to_string(),
                    // `{{bibliography}}` is the only token whose
                    // replacement is built here, not passed in.
                    _ => bib.to_string(),
                })
                .to_string();
            format!("{filled}{comment}")
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\\thanks{}", "");
    Ok(rendered)
}

/// A user-supplied preamble head must carry a document class and
/// every package the woven body provably needs; otherwise the
/// failure would surface as a cryptic TeX log, far from its cause.
/// `titlesec` breaks `IEEEtran` sectioning (notably
/// `[nobottomtitles]`): reject the combination naming both, and
/// point at the built-in `[typography]` guards as the replacement.
/// Do NOT re-propose titlesec here — this error is the record.
pub fn check_titlesec(preamble_tex: &str, class: Option<&str>) -> Result<(), crate::error::Error> {
    let uses_titlesec = crate::texenv::used_packages(preamble_tex).iter().any(|p| p == "titlesec");
    if uses_titlesec && class.map(|c| c.starts_with("IEEE")).unwrap_or(false) {
        let at = preamble_tex.find("titlesec").unwrap_or(0);
        let ln = head_line(preamble_tex, at);
        let line = preamble_tex.lines().nth(ln.saturating_sub(1)).unwrap_or("");
        return Err(crate::error::Error::new(
            "template loads titlesec under an IEEE class (incompatible sectioning): remove titlesec and use the built-in [typography] keep_with_next guards instead",
            ln,
            line.trim().to_string(),
        ).with_col(line.find("titlesec").map(|b| crate::error::col_of(line, b))));
    }
    Ok(())
}

pub fn validate_template(
    override_tex: &str,
    body: &str,
    needs: &FloatNeeds,
) -> Result<(), crate::error::Error> {
    if crate::texenv::document_class(override_tex).is_none() {
        return Err(crate::error::Error::new(
            "template has no \\documentclass",
            1,
            override_tex.lines().next().unwrap_or("").trim().to_string(),
        ));
    }
    check_titlesec(override_tex, crate::texenv::document_class(override_tex).as_deref())?;
    let mut required: Vec<&str> = Vec::new();
    if body.contains("\\includegraphics") {
        required.push("graphicx");
    }
    if body.contains("\\begin{tikzpicture}") {
        required.push("tikz");
    }
    if body.contains("\\begin{axis}") {
        required.push("pgfplots");
    }
    if body.contains("\\begin{tabularx}") {
        required.push("tabularx");
    }
    let have = crate::texenv::used_packages(override_tex);
    let mut required: Vec<&str> = required;
    required.extend(needs.required_names());
    let missing: Vec<&&str> = required.iter().filter(|r| !have.iter().any(|h| h == **r)).collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(crate::error::Error::new(
            format!(
                "template drops packages the body needs: {}",
                missing.into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")
            ),
            1,
            override_tex.lines().next().unwrap_or("").trim().to_string(),
        ))
    }
}

/// Heuristic package scan for ARBITRARY `.tex` (user heads, pasted
/// preambles): `\FloatBarrier` needs placeins, `[H]` needs float,
/// `\needspace` needs needspace. The builder itself threads
/// precise `FloatNeeds` instead.
pub fn tex_extra_packages(tex: &str) -> Vec<String> {
    let mut out = Vec::new();
    if tex.contains("[H]") {
        out.push("\\usepackage{float}".to_string());
    }
    if tex.contains("\\FloatBarrier") {
        out.push("\\usepackage{placeins}".to_string());
    }
    if tex.contains("\\needspace") {
        out.push("\\usepackage{needspace}".to_string());
    }
    if tex.contains("sidewaystable") {
        out.push("\\usepackage{rotating}".to_string());
    }
    if tex.contains("\\begin{strip}") {
        out.push("\\usepackage{cuted}".to_string());
    }
    out
}

/// brace-depth 0 and outside `\\left..\\right` pairs (which may not
/// span `\\\\`). Prefer `=` (operator closes the row); else binary
/// `+`/`-` (operator opens the continuation). Never invent breaks
/// inside fractions, scripts, or matrices â€” unbreakable rows stay
/// whole and keep their residual overfull honestly.
/// Split a trailing tag off a display span (the emitter puts it
/// there); returns (body, tag-or-empty). Only a tag that closes at
/// the very end counts.
fn split_trailing_tag(tex: &str) -> (&str, &str) {
    let t = tex.trim_end();
    let Some(i) = t.rfind("\\tag{") else { return (t, ""); };
    let mut depth = 1usize;
    let mut j = i + 5;
    let bytes = t.as_bytes();
    while j < bytes.len() {
        match bytes[j] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    j += 1;
                    break;
                }
            }
            _ => {}
        }
        j += 1;
    }
    if depth == 0 && j >= t.len() {
        (&t[..i], &t[i..])
    } else {
        (t, "")
    }
}

pub(crate) fn break_display_rows(tex: &str) -> Vec<String> {
    let mut rows = vec![tex.to_string()];
    for _ in 0..6 {
        let idx = rows
            .iter()
            .enumerate()
            .filter(|(_, r)| display_math_width_cm(r) > 17.8)
            .max_by(|(_, a), (_, b)| {
                display_math_width_cm(a).partial_cmp(&display_math_width_cm(b)).unwrap()
            })
            .map(|(i, _)| i);
        let i = match idx {
            Some(i) => i,
            None => break,
        };
        match split_row_once(&rows[i]) {
            Some((a, b)) => {
                rows[i] = b;
                rows.insert(i, a);
            }
            None => break,
        }
    }
    rows
}

/// One split of a display row at a top-level break point.
fn split_row_once(row: &str) -> Option<(String, String)> {
    // Candidate split byte offsets with their operator class.
    let mut eq_at: Option<usize> = None;
    let mut bin_at: Option<usize> = None;
    let mut depth = 0i32;
    let mut left_depth = 0i32;
    let mut matrix_depth = 0i32;
    let mut prev_closable = false;
    let bytes = row.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '\\' {
            let mut j = i + 1;
            while j < bytes.len() && (bytes[j] as char).is_ascii_alphabetic() {
                j += 1;
            }
            let name = &row[i + 1..j];
            if name == "left" {
                left_depth += 1;
                prev_closable = false;
            } else if name == "right" {
                // The unclosed-fence fixup appends a matchless
                // `\right.`: never let it drive the depth negative.
                left_depth = (left_depth - 1).max(0);
                prev_closable = true;
            } else if name == "begin" {
                matrix_depth += 1;
            } else if name == "end" {
                matrix_depth = (matrix_depth - 1).max(0);
            } else if !name.is_empty() {
                // A typeset command is an atom (`\\alpha - x`
                // breaks after the alpha, not before the minus).
                prev_closable = true;
            }
            i = if name.is_empty() { j + 1 } else { j };
            continue;
        }
        match c {
            '{' => {
                depth += 1;
                prev_closable = false;
            }
            '}' => {
                depth -= 1;
                prev_closable = true;
            }
            '=' if depth == 0 && left_depth == 0 && matrix_depth == 0 => {
                eq_at = Some(i);
                prev_closable = false;
            }
            '+' | '-' if depth == 0 && left_depth == 0 && matrix_depth == 0 && prev_closable => {
                bin_at = Some(i);
                prev_closable = false;
            }
            ')' | ']' => prev_closable = true,
            ',' | ';' => prev_closable = false,
            ' ' => {}
            _ if c.is_alphanumeric() => prev_closable = true,
            _ => prev_closable = false,
        }
        i += 1;
    }
    // Latest `=` keeps rows longest-first; else latest binary op.
    // Both split BEFORE the operator so it opens the continuation
    // (original style: `A` / `= B` / `+ C`). A leading `=` is a
    // continuation head from an earlier split, not a split point.
    if let Some(p) = eq_at {
        if p > 0 {
            return Some((row[..p].trim_end().to_string(), row[p..].trim_start().to_string()));
        }
    }
    if let Some(p) = bin_at {
        // Operator opens the continuation: `A` / `+ B`.
        return Some((row[..p].trim_end().to_string(), row[p..].trim_start().to_string()));
    }
    None
}

/// Does this `.tex` fragment need TikZ? (standalone preamble builder)
/// Python could ask with `in`, but the scan lives with the detector.
pub fn tikz_needed(tex: &str) -> bool {
    tex.contains("\\begin{tikzpicture}")
}

/// Does this `.tex` fragment need pgfplots?
pub fn plots_needed(tex: &str) -> bool {
    tex.contains("\\begin{axis}")
}

/// LaTeX packages a woven body needs, detected from what it contains.
/// Precompiled PDF figures need nothing; inline `tikzpicture`s need
/// TikZ + shape/arrow libraries; pgfplots `axis` environments need
/// pgfplots. Everything named here ships in TeX Live (and arXiv's).
pub fn tex_requirements(has_tikz: bool, has_plots: bool) -> Vec<String> {
    let mut req = Vec::new();
    if has_tikz {
        req.push("\\usepackage{tikz}".to_string());
        req.push("\\usetikzlibrary{shapes.geometric,arrows.meta,positioning}".to_string());
    }
    if has_plots {
        req.push("\\usepackage{pgfplots}".to_string());
        req.push("\\pgfplotsset{compat=1.18}".to_string());
    }
    req
}

pub fn convert(
    lines: &[String],
    spec: &BuildSpec,
    cfg: &Config,
) -> Result<(String, String, usize, usize, FloatNeeds), crate::error::Error> {
    use crate::error::Error;
    let keys = if spec.bib_keys.is_empty() { None } else { Some(&spec.bib_keys) };
    let mut out: Vec<String> = Vec::new();
    let mut needs = FloatNeeds::none();
    let mut pending_table: Option<ElemAttrs> = None;
    let mut para: Vec<String> = Vec::new();
    let mut title = String::new();
    let mut in_abstract = false;
    let mut diagram_idx = 0usize;
    let mut table_idx = 0usize;
    // Original line of the pending para's first line (for flush errors).
    let mut para_line = 0usize;

    let flush = |out: &mut Vec<String>, para: &mut Vec<String>, pline: usize| -> Result<(), Error> {
        if !para.is_empty() {
            let mut text = inline(&para.join(" "), keys, pline)?;
            if text.starts_with("\\dag{}") {
                text = format!("\\textit{{Note: }}{}", &text["\\dag{}".len()..]);
            }
            out.push(text + "\n");
            para.clear();
        }
        Ok(())
    };

    // E.1 (M5.4): the line walk is gone — `crate::md::blocks()`
    // owns boundaries (pulldown-cmark) with raw-slice content, and
    // every arm below is the old branch body verbatim on the same
    // inputs. `lines` is rejoined because offsets need one source.
    let md_text = lines.join("\n");
    for b in crate::md::blocks(&md_text)? {
        let line = b.line;
        match b.kind {
            MdKind::Fence { info: s, body: fence_body } => {
                flush(&mut out, &mut para, para_line)?;
                if s.contains("mermaid") {
                    if diagram_idx >= spec.diagrams.len() {
                        return Err(Error::new(format!("mermaid fence #{diagram_idx} has no registry entry"), line, s.clone()));
                    }
                    let entry = &spec.diagrams[diagram_idx];
                    let (attrs, inline_flag) = parse_attrs(
                        fence_attr_str(&s),
                        &format!("mermaid fence #{diagram_idx}"),
                        line,
                    )?;
                    let mode = if inline_flag || s.contains("{inline}") {
                        "inline"
                    } else {
                        entry.mode.as_str()
                    };
                    let kind = if mode == "inline" { "diagram" } else { "picture" };
                    let span = attrs.span.unwrap_or_else(|| cfg.spans.for_kind(kind));
                    let pos = attrs.pos.unwrap_or(cfg.floats.pos_default);
                    let body = if mode == "inline" {
                        flowchart_to_tikz(&fence_body, line + 1).map_err(|e| Error {
                            msg: format!("mermaid fence #{diagram_idx}: {}", e.msg),
                            ..e
                        })?
                    } else {
                        format!("{}{}", entry.path_prefix, entry.key)
                    };
                    let caption_bottom =
                        attrs.captionpos.map(|c| c == CaptionPos::Bottom).unwrap_or(true);
                    let (fig, n) = figure_block(
                        &body,
                        &inline(&entry.caption, keys, line)?,
                        &format!("fig:{}", entry.key),
                        span,
                        pos,
                        attrs.width.as_deref(),
                        caption_bottom,
                    )
                    .map_err(|e| Error::new(format!("mermaid fence #{diagram_idx}: {e}"), line, s.clone()))?;
                    needs.add(n);
                    out.push(fig + "\n");
                    diagram_idx += 1;
                }
                // Non-mermaid fences: skipped silently, exactly as
                // the hand scanner skipped them.
            }
            MdKind::Directive(a) => {
                // Same stash rule, parsed with the same `table #N`
                // context; the Table arm below takes it. (A
                // dangling `%% table` never becomes a Directive —
                // the pre-pass leaves it as paragraph text.)
                let (attrs, _) = parse_attrs(&a, &format!("table #{table_idx}"), line)?;
                if attrs.span.is_some()
                    || attrs.pos.is_some()
                    || attrs.width.is_some()
                    || attrs.captionpos.is_some()
                {
                    pending_table = Some(attrs);
                }
            }
            MdKind::Table(rows) => {
                flush(&mut out, &mut para, para_line)?;
                if table_idx >= spec.table_captions.len() {
                    return Err(Error::new(format!("table #{table_idx} has no caption"), line, rows.first().cloned().unwrap_or_default()));
                }
                let tattrs = pending_table.take().unwrap_or_default();
                let tspan = tattrs.span.unwrap_or_else(|| cfg.spans.for_kind("table"));
                let tpos = tattrs.pos.unwrap_or(cfg.floats.pos_default);
                let caption_top = tattrs.captionpos.map(|c| c == CaptionPos::Top).unwrap_or(true);
                // Portrait spans in cm for the compaction ladder: the
                // layout policy carries the class geometry, so custom
                // classes compact against their own widths.
                let (tab, n) = table_block(
                    &rows,
                    &spec.table_captions[table_idx],
                    keys,
                    spec.table_specs.get(&table_idx.to_string()).map(String::as_str),
                    tspan,
                    tpos,
                    caption_top,
                    tattrs.width.as_deref(),
                    cfg.layout.column_cm,
                    cfg.layout.textwidth_cm,
                    cfg.tables.compact_tables,
                    cfg.layout.here_max_cm,
                    line,
                )
                .map_err(|e| Error {
                    msg: format!("table #{table_idx}: {}", e.msg),
                    ..e
                })?;
                needs.add(n);
                out.push(tab + "\n");
                table_idx += 1;
            }
            MdKind::Appendix => {
                flush(&mut out, &mut para, para_line)?;
                out.push("\\appendix\n\\section{Claim-to-decision map}\n".to_string());
            }
            MdKind::Abstract => {
                flush(&mut out, &mut para, para_line)?;
                out.push("\\begin{abstract}\n".to_string());
                in_abstract = true;
            }
            MdKind::H2(h) => {
                flush(&mut out, &mut para, para_line)?;
                if in_abstract {
                    out.push("\\end{abstract}\n".to_string());
                    in_abstract = false;
                }
                let head = strip_section_number(&h);
                if cfg.floats.barrier_sections {
                    out.push("\\FloatBarrier\n".to_string());
                    needs.barrier = true;
                }
                if cfg.typography.keep_with_next {
                    out.push(cfg.typography.needspace_line() + "\n");
                }
                out.push(format!("\\section{{{}}}\n", inline(&head, keys, line)?));
            }
            MdKind::H3(h) => {
                flush(&mut out, &mut para, para_line)?;
                let head = strip_section_number(&h);
                if cfg.floats.barrier_sections {
                    out.push("\\FloatBarrier\n".to_string());
                    needs.barrier = true;
                }
                if cfg.typography.keep_with_next {
                    out.push(cfg.typography.needspace_line() + "\n");
                }
                out.push(format!("\\subsection{{{}}}\n", inline(&head, keys, line)?));
            }
            MdKind::Title(t) => {
                title = inline(&t, keys, line)?;
            }
            MdKind::Para(ls) => {
                // Consecutive Para blocks are always blank-separated
                // (`blocks()` splits there), so a pending para must
                // flush first — the old scanner flushed on the blank
                // line itself. Lines WITHIN one block join safely.
                if !para.is_empty() {
                    flush(&mut out, &mut para, para_line)?;
                }
                // The block's line is its first fed line's original.
                para_line = line;
                para.extend(ls);
            }
            MdKind::DisplayMath(t) => {
                // M7.8 honest math: structured spans break at
                // relations inside the column; a span wider than the
                // column takes a full-width `strip` (cuted) instead
                // of overflowing. Weaver-side and deterministic — no
                // advisory needed, the width is intrinsic.
                flush(&mut out, &mut para, para_line)?;
                if t.contains("$$") {
                    return Err(Error::new(
                        format!("line {line}: display math holds a second `$$`; one span per paragraph"),
                        line,
                        t.clone(),
                    ));
                }
                let no_matrix = t.replace("\\begin{matrix}", "").replace("\\end{matrix}", "");
                if no_matrix.contains("\\begin") || no_matrix.contains("\\end") {
                    return Err(Error::new(
                        format!("line {line}: display math allows no environments except matrix"),
                        line,
                        t.clone(),
                    ));
                }
                // `\\left\\{` delimiters and `\\{` literals are not
                // groups: skip them so a cases-Brace (`\\left\\{` +
                // `\\right.`) does not read as unbalanced.
                let mut balance = 0i32;
                let mut bs = t.chars().peekable();
                while let Some(c) = bs.next() {
                    if c != '\\' {
                        if c == '{' {
                            balance += 1;
                        } else if c == '}' {
                            balance -= 1;
                        }
                        continue;
                    }
                    let mut name = String::new();
                    while let Some(&d) = bs.peek() {
                        if d.is_ascii_alphabetic() {
                            name.push(d);
                            bs.next();
                        } else {
                            break;
                        }
                    }
                    if name == "left" || name == "right" {
                        // Delimiter token: `{`, `}`, `|`, `.`, `\\{`,
                        // or a command (`\\langle`). A lone backslash
                        // followed by a non-letter is ONE delimiter
                        // (`\\{`); consuming only the backslash would
                        // strand the brace and fake an imbalance.
                        if bs.peek() == Some(&'\\') {
                            bs.next();
                            if bs.peek().map(|d| d.is_ascii_alphabetic()).unwrap_or(false) {
                                while let Some(&d) = bs.peek() {
                                    if d.is_ascii_alphabetic() {
                                        bs.next();
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                bs.next();
                            }
                        } else {
                            bs.next();
                        }
                    } else if name.is_empty() {
                        bs.next(); // `\\{`, `\\%` ... literal, not a group.
                    }
                }
                if balance != 0 {
                    return Err(Error::new(
                        format!("line {line}: display math has unbalanced braces"),
                        line,
                        t.clone(),
                    ));
                }
                let w = display_math_width_cm(&t);
                if w <= cfg.layout.column_cm {
                    out.push(format!("\\[{t}\\]\n"));
                } else {
                    needs.strip = true;
                    if w > cfg.layout.textwidth_cm {
                        // Wider than the full measure: break at
                        // top-level relations. Plain spans use one
                        // multline* inside the strip; matrix spans
                        // keep `&` tabs, which multline forbids, so
                        // each piece takes its own strip instead.
                        let (dbody, dtag) = split_trailing_tag(&t);
                        let rows = break_display_rows(dbody);
                        if t.contains("\\begin{matrix}") {
                            let last = rows.len().saturating_sub(1);
                            for (i, r) in rows.iter().enumerate() {
                                let tag_here = if i == last { dtag } else { "" };
                                out.push(format!("\\begin{{strip}}\n\\[{r}{t}\\]\n\\end{{strip}}\n", r = r, t = tag_here));
                            }
                        } else {
                            let aligned = rows
                                .iter()
                                .map(|r| format!("& {}", r))
                                .collect::<Vec<_>>()
                                .join(" \\\\\n");
                            out.push(format!(
                                "\\begin{{strip}}\n\\[\\begin{{aligned}}\n{}\n\\end{{aligned}}{}\\]\n\\end{{strip}}\n",
                                aligned, dtag
                            ));
                        }
                    } else {
                        out.push(format!("\\begin{{strip}}\n\\[{t}\\]\n\\end{{strip}}\n"));
                    }
                }
            }
        }
    }
    flush(&mut out, &mut para, para_line)?;
    if in_abstract {
        out.push("\\end{abstract}\n".to_string());
    }
    Ok((title, out.join("\n"), diagram_idx, table_idx, needs))
}

pub fn build_document(md_text: &str, toml_src: &str, spec: &BuildSpec) -> Result<BuildResult, crate::error::Error> {
    let cfg = Config::from_toml(toml_src).map_err(|e| crate::error::Error::data(e, String::new()))?;
    let lines: Vec<String> = md_text.lines().map(|l| l.to_string()).collect();
    let (title, mut body, n_diagrams, n_tables, mut needs) = convert(&lines, spec, &cfg)?;
    if n_diagrams != spec.diagrams.len() {
        return Err(crate::error::Error::data(
            format!("{n_diagrams} fences vs {} registry entries", spec.diagrams.len()),
            String::new(),
        ));
    }
    if n_tables != spec.table_captions.len() {
        return Err(crate::error::Error::data(
            format!("{n_tables} tables vs {} captions", spec.table_captions.len()),
            String::new(),
        ));
    }
    if !spec.plots.is_empty() {
        let mut figs = Vec::new();
        for (key, cap) in &spec.plots {
            // Plot attrs ride the spec dict (JSON/Python), not the
            // manuscript: line 0 (data class — no manuscript line).
            let pattrs = match spec.plot_attrs.get(key) {
                Some(a) => parse_attrs(a, &format!("plot {key}"), 0)?.0,
                None => ElemAttrs::default(),
            };
            let pspan = pattrs.span.unwrap_or_else(|| cfg.spans.for_kind("plot"));
            let ppos = pattrs.pos.unwrap_or(cfg.floats.pos_default);
            let caption_bottom =
                pattrs.captionpos.map(|c| c == CaptionPos::Bottom).unwrap_or(true);
            let (fig, n) = figure_block(
                &format!("{}{}", spec.plot_dir, key),
                cap,
                &format!("fig:{key}"),
                pspan,
                ppos,
                pattrs.width.as_deref(),
                caption_bottom,
            )
            .map_err(|e| crate::error::Error::new(format!("plot {key}: {e}"), 0, key.clone()))?;
            needs.add(n);
            figs.push(fig);
        }
        let plot_figs = figs.join("\n");
        if !spec.plot_insert_before.is_empty() && body.contains(&spec.plot_insert_before) {
            body = body.replacen(&spec.plot_insert_before, &format!("{plot_figs}\n{}", spec.plot_insert_before), 1);
        } else {
            body = format!("{body}\n{plot_figs}\n");
        }
    }
    // Flush pending floats before the reference list: without a
    // barrier, end-queued floats spill into/past `References`
    // (bio-medsea's Figs 7-14 landed after it). Opt-in so default
    // weaves stay byte-identical.
    if cfg.floats.barrier_before_refs && !spec.bib_name.is_empty() {
        needs.barrier = true;
    }
    let paths: String = spec.graphicspaths.iter().map(|p| format!("{{{p}}}")).collect();
    let opts = cfg.document.class_options.join(",");
    let has_tikz = body.contains("\\begin{tikzpicture}");
    let has_plots = body.contains("\\begin{axis}");
    let mut head: Vec<String> = if !spec.skeleton.is_empty() {
        // Level 3: the skeleton IS the document. No generated
        // head/tail at all — render tokens, splice appends, done.
        if !spec.preamble_override.is_empty() || !cfg.template.preamble_file.is_empty() {
            return Err(crate::error::Error::data(
                "[template] skeleton is mutually exclusive with preamble_file",
                String::new(),
            ));
        }
        validate_template(&spec.skeleton, &body, &needs)?;
        let (abs, rest) = if cfg.template.abstract_before_maketitle {
            split_abstract(&body)
        } else {
            (String::new(), body.clone())
        };
        let bib = if spec.bib_name.is_empty() {
            String::new()
        } else if cfg.floats.barrier_before_refs {
            format!(
                "\\FloatBarrier\n\\bibliographystyle{{{}}}\n\\bibliography{{{}}}",
                spec.bib_style, spec.bib_name
            )
        } else {
            format!(
                "\\bibliographystyle{{{}}}\n\\bibliography{{{}}}",
                spec.bib_style, spec.bib_name
            )
        };
        let mut rendered = render_skeleton(
            &spec.skeleton,
            &title,
            &spec.author,
            &spec.title_thanks,
            &abs,
            &rest,
            &bib,
        )?;
        // needspace auto-add (same rule as the override path): the
        // body carries \needspace commands the builder emitted, so
        // the package must be present whether or not the skeleton
        // ships it. Additive only, right before \begin{document}.
        let need_needspace = body.contains("\\needspace")
            && !crate::texenv::used_packages(&spec.skeleton).iter().any(|p| p == "needspace");
        if !spec.preamble_append.is_empty() || need_needspace {
            let Some(at) = rendered.find("\\begin{document}") else {
                return Err(crate::error::Error::data(
                    "skeleton has no \\begin{document} for preamble_append",
                    String::new(),
                ));
            };
            let mut extra = spec.preamble_append.join("\n");
            if need_needspace {
                if !extra.is_empty() {
                    extra.push('\n');
                }
                extra.push_str("\\usepackage{needspace}");
            }
            rendered.insert_str(at, &(extra + "\n"));
        }
        return Ok(BuildResult {
            title,
            body: body.clone(),
            tex: rendered + "\n",
            n_diagrams,
            n_tables,
        });
    } else if spec.preamble_override.is_empty() {
        let mut generated = vec![
            format!("\\documentclass[{opts}]{{{}}}", cfg.document.class),
            "\\usepackage[utf8]{inputenc}".to_string(),
            "\\usepackage{lmodern}".to_string(),
            "\\usepackage{textcomp}".to_string(),
            "\\usepackage{amsmath,amssymb}".to_string(),
            "\\usepackage{tabularx}".to_string(),
            "\\usepackage{graphicx}".to_string(),
        ];
        generated.extend(tex_requirements(has_tikz, has_plots));
        generated.extend(needs.packages());
        generated.extend(cfg.floats.setup_lines());
        generated.extend(cfg.typography.setup_lines());
        if body.contains("\\needspace") {
            generated.push("\\usepackage{needspace}".to_string());
        }
        generated.extend([
            "\\usepackage[hidelinks]{hyperref}".to_string(),
            format!("\\graphicspath{{{paths}}}"),
            if spec.title_thanks.is_empty() {
                format!("\\title{{{title}}}")
            } else {
                format!("\\title{{{title}\\thanks{{{}}}}}", spec.title_thanks)
            },
            format!("\\author{{\\IEEEauthorblockN{{{}}}}}", spec.author),
        ]);
        generated
    } else {
        validate_template(&spec.preamble_override, &body, &needs)?;
        let mut over = vec![render_override(&spec.preamble_override, &title, &spec.author, &spec.title_thanks)];
        over.extend(cfg.floats.setup_lines());
        over.extend(cfg.typography.setup_lines());
        if body.contains("\\needspace")
            && !crate::texenv::used_packages(&spec.preamble_override).iter().any(|p| p == "needspace")
        {
            over.push("\\usepackage{needspace}".to_string());
        }
        over
    };
    head.extend(spec.preamble_append.clone());
    head.extend(["\\begin{document}".to_string()]);
    let (topmatter, rest) = if cfg.template.abstract_before_maketitle {
        split_abstract(&body)
    } else {
        (String::new(), body.clone())
    };
    if !topmatter.is_empty() {
        head.push(topmatter);
    }
    // No bib_name (working paper without refs): no bibliography lines
    // at all. An empty `\bibliography{}` warns under pdfTeX but kills
    // engines that auto-run BibTeX (tectonic builds an item-less
    // thebibliography and dies at \end{thebibliography}).
    head.extend([
        "\\maketitle".to_string(),
        rest,
    ]);
    if !spec.bib_name.is_empty() {
        if cfg.floats.barrier_before_refs {
            head.push("\\FloatBarrier".to_string());
        }
        head.extend([
            format!("\\bibliographystyle{{{}}}", spec.bib_style),
            format!("\\bibliography{{{}}}", spec.bib_name),
        ]);
    }
    head.extend(["\\end{document}".to_string(), String::new()]);
    let doc = head.join("\n");
    Ok(BuildResult {
        title,
        body: body.clone(),
        tex: doc,
        n_diagrams,
        n_tables,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> BuildSpec {
        BuildSpec {
            diagrams: vec![DiagramEntry {
                key: "fig-x".into(),
                caption: "Cap $r$.".into(),
                mode: "inline".into(),
                path_prefix: "figs/tikz/".into(),
            }],
            plots: vec![],
            plot_attrs: HashMap::new(),
            plot_dir: "figs/".into(),
            table_captions: vec!["T.".into()],
            table_specs: HashMap::new(),
            plot_insert_before: String::new(),
            title_thanks: "th".into(),
            author: "opticsWolf".into(),
            bib_name: "refs-paper".into(),
            graphicspaths: vec!["./".into()],
            bib_keys: HashSet::new(),
            preamble_override: String::new(),
            preamble_append: Vec::new(),
            skeleton: String::new(),
            bib_style: "IEEEtran".to_string(),
        }
    }

    #[test]
    fn mini_doc() {
        let md = "# Title here\n\n## Abstract\n\nAbs text.\n\n## 1. Intro\n\nHello [`k`] world.\n\n```mermaid\ngraph TD\na[x]-->b[y]\n```\n\n| A |\n|---|\n| 1 |\n";
        let r = build_document(md, "", &spec()).unwrap();
        assert!(r.tex.contains("\\title{Title here\\thanks{th}}"));
        assert!(r.tex.contains("\\begin{abstract}"));
        assert!(r.tex.contains("\\section{Intro}"));
        assert!(r.tex.contains("\\cite{k}"));
        assert!(r.tex.contains("\\begin{tikzpicture}"));
        assert!(r.tex.contains("\\usepackage{tikz}"));
        assert!(r.tex.contains("\\usetikzlibrary{shapes.geometric,arrows.meta,positioning}"));
        assert!(!r.tex.contains("pgfplots"));
        assert!(r.tex.contains("\\begin{tabularx}"));
        assert_eq!((r.n_diagrams, r.n_tables), (1, 1));
    }

    #[test]
    fn requirements_only_what_is_used() {
        assert!(tex_requirements(false, false).is_empty());
        let tikz = tex_requirements(true, false);
        assert_eq!(tikz.len(), 2);
        assert!(tikz[0].contains("usepackage{tikz}"));
        let plots = tex_requirements(false, true);
        assert!(plots.iter().any(|l| l.contains("pgfplots")));
    }

    #[test]
    fn template_override_replaces_head() {
        let md = "# T\n\n## 1. I\n\nHi.\n";
        let mut s = spec();
        s.diagrams.clear();
        s.table_captions.clear();
        s.preamble_override =
            "\\documentclass{article}\n\\usepackage{lmodern}\n\\title{T}\n\\author{A}".to_string();
        s.preamble_append = vec!["\\usepackage{natbib}".to_string()];
        let r = build_document(md, "", &s).unwrap();
        assert!(r.tex.contains("\\documentclass{article}"));
        assert!(!r.tex.contains("documentclass[conference]{IEEEtran}"));
        assert!(!r.tex.contains("\\usepackage{tabularx}"));
        let head = r.tex.split("\\begin{document}").next().unwrap();
        assert!(head.contains("\\usepackage{natbib}"));
        assert!(head.contains("\\usepackage{lmodern}"));
    }

    #[test]
    fn template_missing_package_is_an_error() {
        let md = "# T\n\n## 1. I\n\nHi.\n\n```mermaid\ngraph TD\na[x]-->b[y]\n```\n";
        let mut s = spec();
        s.table_captions.clear();
        s.preamble_override = "\\documentclass{article}\n\\title{T}".to_string();
        let err = build_document(md, "", &s).unwrap_err();
        assert!(err.msg.contains("tikz"), "unexpected: {err}");
        assert_eq!(err.line, 1, "head-relative line, got: {err}");
    }

    #[test]
    fn override_tokens_render_from_build() {
        assert_eq!(
            render_override("\\title{{title}}\\author{{author}}\\thanks{{thanks}}", "T", "A", "th"),
            "\\titleT\\authorA\\thanksth"
        );
    }

    #[test]
    fn bib_style_follows_spec() {
        let md = "# T\n\n## 1. I\n\nHi.\n";
        let mut s = spec();
        s.diagrams.clear();
        s.table_captions.clear();
        s.bib_style = "splncs04".to_string();
        let r = build_document(md, "", &s).unwrap();
        assert!(r.tex.contains("\\bibliographystyle{splncs04}"));
    }

    #[test]
    fn abstract_hoist_for_topmatter_classes() {
        let body = "Sec.\\begin{abstract}Abs.\\end{abstract}\\section{X}";
        let (abs, rest) = split_abstract(body);
        assert_eq!(abs, "\\begin{abstract}Abs.\\end{abstract}");
        assert_eq!(rest, "Sec.\\section{X}");
        assert_eq!(split_abstract("no abstract here"), (String::new(), "no abstract here".to_string()));
    }

    #[test]
    fn template_without_class_is_an_error() {
        let err = validate_template("\\usepackage{tikz}\n", "plain", &FloatNeeds::none()).unwrap_err();
        assert!(err.msg.contains("documentclass"), "got: {err}");
        assert_eq!(err.line, 1, "got: {err}");
    }

    #[test]
    fn titlesec_under_ieee_is_an_error() {
        let head = "\\documentclass[conference]{IEEEtran}\n\\usepackage[nobottomtitles]{titlesec}\n";
        let err = check_titlesec(head, Some("IEEEtran")).unwrap_err();
        assert!(err.msg.contains("titlesec") && err.msg.contains("typography"), "got: {err}");
        assert_eq!(err.line, 2, "titlesec is on head line 2, got: {err}");
        assert!(err.echo.contains("titlesec"), "got: {err}");
        // Same preamble under article: fine.
        assert!(check_titlesec(head, Some("article")).is_ok());
        // And the full validator surfaces it too.
        assert!(validate_template(head, "plain", &FloatNeeds::none()).is_err());
    }

    #[test]
    fn needspace_auto_package_on_override() {
        // Body needs needspace (guards on), head lacks it: the
        // builder adds the package instead of failing.
        let mut s = spec();
        s.diagrams = vec![];
        s.table_captions = vec![];
        s.preamble_override = "\\documentclass{article}\n\\begin{document}\n".to_string();
        let r = build_document("# T\n\n## 1. Sec\n\nBody.\n", "", &s).unwrap();
        assert!(r.tex.contains("\\usepackage{needspace}"), "auto-added");
        assert!(r.tex.contains("\\needspace{3\\baselineskip}"), "guard emitted");
        assert!(r.tex.contains("\\clubpenalty=10000"), "penalties emitted");
    }

    #[test]
    fn error_dangling_key_carries_line() {
        use std::collections::HashSet;
        let mut s = spec();
        s.diagrams = vec![];
        s.table_captions = vec![];
        s.bib_keys = HashSet::from(["a".to_string()]);
        let err = build_document("# T\n\nSee [`nope`] here.\n", "", &s).unwrap_err();
        assert!(err.msg.contains("dangling citation key"), "got: {err}");
        assert_eq!(err.line, 3, "got: {err}");
        assert!(err.echo.contains("nope"), "got: {err}");
    }

    #[test]
    fn error_skeleton_token_carries_line() {
        let skel = "\\documentclass{article}\n\\begin{document}\n{{bogus}}\n\\end{document}\n";
        let err = render_skeleton(skel, "T", "A", "", "", "B.", "").unwrap_err();
        assert!(err.msg.contains("unknown token"), "got: {err}");
        assert_eq!(err.line, 3, "got: {err}");
        assert!(err.echo.contains("bogus"), "got: {err}");
    }

    #[test]
    fn error_wide_here_table_carries_line() {
        let mut s = spec();
        s.diagrams = vec![];
        s.table_captions = vec!["C.".to_string()];
        let md = "# T\n\n%% table {span=wide pos=here}\n\n| A |\n|---|\n| 1 |\n";
        let err = build_document(md, "", &s).unwrap_err();
        assert!(err.msg.contains("here/force"), "got: {err}");
        assert_eq!(err.line, 5, "table starts line 5, got: {err}");
    }

    #[test]
    fn error_unknown_attr_carries_line() {
        let mut s = spec();
        s.diagrams = vec![DiagramEntry { key: "x".into(), caption: "C.".into(), mode: "precompiled".into(), path_prefix: "p/".into() }];
        s.table_captions = vec![];
        let md = "# T\n\n```mermaid {pos=middle}\ngraph TD\na[x]\n```\n";
        let err = build_document(md, "", &s).unwrap_err();
        assert!(err.msg.contains("pos must be"), "got: {err}");
        assert_eq!(err.line, 3, "fence info line, got: {err}");
        assert!(err.echo.contains("pos=middle"), "got: {err}");
    }

    #[test]
    fn error_attr_caret_counts_leading_whitespace() {
        // The attr string keeps its inner leading space (`{ ...}`)
        // — the caret must count it, not the trimmed token.
        let err = parse_attrs("  span=wide bogus=1", "t", 9).unwrap_err();
        assert_eq!(err.line, 9, "got: {err}");
        assert_eq!(err.col, Some(13), "bogus starts char 13, got: {err}");
        let s = err.to_string();
        let caret = s.lines().nth(2).unwrap_or("");
        assert_eq!(caret, format!("{}^", " ".repeat(12)), "got: {s}");
    }

    #[test]
    fn error_skeleton_token_caret() {
        let skel = "\\documentclass{article}\n\\begin{document}\n  {{bogus}} here\n\\end{document}\n";
        let err = render_skeleton(skel, "T", "A", "", "", "B.", "").unwrap_err();
        assert_eq!(err.line, 3, "got: {err}");
        assert_eq!(err.col, Some(3), "token starts char 3, got: {err}");
    }

    #[test]
    fn attrs_parse_strictly() {
        let (a, inl) = parse_attrs("span=column pos=both width=0.8 captionpos=top", "t", 5).unwrap();
        assert!(!inl);
        assert_eq!(a.span, Some(Span::Column));
        assert_eq!(a.pos, Some(Pos::Both));
        assert_eq!(a.width.as_deref(), Some("0.8"));
        assert_eq!(a.captionpos, Some(CaptionPos::Top));
        let (_, inl) = parse_attrs("{inline}", "t", 5).unwrap();
        assert!(inl);
        assert!(parse_attrs("pos=center", "t", 5).is_err());
        assert!(parse_attrs("bogus=1", "t", 5).is_err());
        assert!(parse_attrs("width=huge", "t", 5).is_err());
        assert!(parse_attrs("width=0", "t", 5).is_err());
        assert!(parse_attrs("width=5cm", "t", 5).unwrap().0.width.is_some());
    }

    #[test]
    fn figure_width_with_existing_options() {
        let tikz = "\\begin{tikzpicture}[>=Stealth]\n\\node{a};\\end{tikzpicture}";
        let (f, _) = figure_block(tikz, "C", "l", Span::Column, Pos::Top, Some("0.9"), true).unwrap();
        assert!(f.contains("\\begin{tikzpicture}[scale=0.9,>=Stealth]"));
        assert!(!f.contains("]["));
    }

    #[test]
    fn figure_placements_and_needs() {
        let tikz = "\\begin{tikzpicture}\\node{a};\\end{tikzpicture}";
        let (f, n) = figure_block(tikz, "C", "l", Span::Column, Pos::Both, None, true).unwrap();
        assert!(f.contains("\\begin{figure}[!tb]"));
        assert_eq!(n, FloatNeeds::none());
        let (f, n) = figure_block(tikz, "C", "l", Span::Column, Pos::Force, None, true).unwrap();
        assert!(f.contains("\\begin{figure}[H]"));
        assert!(n.float_h);
        let (f, n) = figure_block("pic.pdf", "C", "l", Span::Wide, Pos::Barrier, Some("0.8"), false).unwrap();
        assert!(f.contains("\\FloatBarrier\n\\begin{figure*}[t]"));
        assert!(f.contains("\\includegraphics[width=0.8\\textwidth]{pic.pdf}"));
        assert!(f.contains("\\caption{C}\n\\label{l}\n\\includegraphics"));
        assert!(n.barrier);
        let (_, n) = figure_block("p.pdf", "C", "l", Span::Wide, Pos::Bottom, None, true).unwrap();
        assert!(n.dblfloat);
        assert!(figure_block(tikz, "C", "l", Span::Wide, Pos::Force, None, true).is_err());
        assert!(figure_block(tikz, "C", "l", Span::Wide, Pos::Here, None, true).is_err());
    }

    #[test]
    fn fence_attrs_flow_end_to_end() {
        let md = "# T\n\n## 1. I\n\nHi.\n\n```mermaid {span=column pos=both}\ngraph TD\na[x]-->b[y]\n```\n\n%% table {pos=barrier}\n\n| A |\n|---|\n| 1 |\n";
        let mut s = spec();
        s.preamble_override = String::new();
        let r = build_document(md, "", &s).unwrap();
        assert!(r.tex.contains("\\begin{figure}[!tb]"));
        assert!(r.tex.contains("\\FloatBarrier\n\\begin{table}[t]"));
        assert!(r.tex.contains("\\usepackage{placeins}"));
        assert!(!r.tex.contains("\\usepackage{float}"));
    }

    #[test]
    fn refs_barrier_is_opt_in_and_placed() {
        let md = "# T\n\n## 1. A\n\nx\n";
        let mut s = spec();
        s.diagrams.clear();
        s.table_captions.clear();
        // Default off: byte-identical weaves, no barrier line.
        let r = build_document(md, "", &s).unwrap();
        assert!(!r.tex.contains("\\FloatBarrier"));
        // Opt-in: barrier ahead of the bibliography + placeins.
        let r = build_document(md, "[floats]\nbarrier_before_refs = true\n", &s).unwrap();
        let pos_b = r.tex.find("\\FloatBarrier").unwrap();
        let pos_r = r.tex.find("\\bibliography{refs-paper}").unwrap();
        assert!(pos_b < pos_r, "barrier flushes before the refs");
        assert!(r.tex.contains("\\usepackage{placeins}"));
        // No bib: knob on but nothing to guard — no stray barrier.
        let mut s = spec();
        s.bib_name.clear();
        s.diagrams.clear();
        s.table_captions.clear();
        let r = build_document(md, "[floats]\nbarrier_before_refs = true\n", &s).unwrap();
        assert!(!r.tex.contains("\\FloatBarrier"));
    }

    #[test]
    fn rotated_table_pulls_rotating_package() {
        // 13 unbreakable columns: portrait cannot hold them.
        let head = "| L |".to_string() + &"| -0.319(0.224) |".repeat(12);
        let sep = "|---|".to_string() + &"|---|".repeat(12);
        let md = format!("# T\n\n## 1. A\n\n%% table\n\n{head}\n{sep}\n| d | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |\n");
        let mut s = spec();
        s.table_captions = vec!["W.".into()];
        s.diagrams.clear();
        let r = build_document(&md, "[tables]\ncompact_tables = true\n", &s).unwrap();
        // Column span promotes to wide before rotating: the 13
        // unbreakable columns cannot live in 8.9cm at any size.
        assert!(r.tex.contains("\\begin{sidewaystable*}[p]"), "rotates wide");
        assert!(r.tex.contains("\\usepackage{rotating}"), "package follows needs");
    }

    #[test]
    fn display_math_narrow_stays_column() {
        // Short spans center in-flow: no strip, no cuted.
        let md = "# T\n\n$$\\vartheta_{\\beta}(\\zeta)$$\n";
        let mut s = spec();
        s.diagrams.clear();
        s.table_captions.clear();
        let r = build_document(md, "", &s).unwrap();
        assert!(r.tex.contains("\\[\\vartheta_{\\beta}(\\zeta)\\]"), "plain display");
        assert!(!r.tex.contains("strip"), "no strip for narrow math");
        assert!(!r.tex.contains("cuted"), "no cuted for narrow math");
    }

    #[test]
    fn display_math_wide_takes_strip() {
        // A 13-column-style monster cannot fit 8.9cm: full-width
        // strip + cuted package, tag preserved.
        let inner = "\\vartheta_{\\beta}=0.086^{*}+0.080^{**}+0.088^{**}+0.085^{**}+0.093^{**}".to_string()
            + &"+0.080^{**}+0.068^{*}+0.072^{*}+0.070^{*}+0.065^{*}+0.056^{*}+0.060^{*}";
        let md = format!("# T\n\n$${}$$\n", format!("{inner} \\tag{{9}}"));
        let mut s = spec();
        s.diagrams.clear();
        s.table_captions.clear();
        let r = build_document(&md, "", &s).unwrap();
        assert!(r.tex.contains("\\begin{strip}"), "wide math strips");
        assert!(r.tex.contains("\\usepackage{cuted}"), "package follows needs");
        assert!(r.tex.contains("\\tag{9}"), "tag rides inside");
    }

    #[test]
    fn display_math_width_frac_takes_max() {
        // A fraction is as wide as its widest part (plus the nulldelimiter
        // pair), not the sum: TeX-measured 0.4209 / 0.4887 cm.
        let half = display_math_width_cm("\\frac{ab}{c}");
        let whole = display_math_width_cm("abc");
        assert!((half - 0.4209).abs() < 0.01, "got {half}");
        assert!((whole - 0.4887).abs() < 0.01, "got {whole}");
        assert!(display_math_width_cm("x \\tag{12}") < 0.5, "tag excluded");
    }

    #[test]
    fn display_math_brace_delimiters_balance() {
        // `\\left\\{` + `\\right.` is balanced delimiters, not groups.
        let md = "# T\n\n$$\\left\\{x\\right. \\tag{2}$$\n";
        let mut s = spec();
        s.diagrams.clear();
        s.table_captions.clear();
        let r = build_document(md, "", &s).unwrap();
        assert!(r.tex.contains("\\[\\left\\{x\\right. \\tag{2}\\]"), "kept: {}", r.tex);
    }

    #[test]
    fn display_math_breaks_monster_rows() {
        // A 610-char Galerkin monster: strip + multline, split at `=`.
        let a = "\\vartheta(\\zeta)";
        let chunk = format!("(\\frac{{x}}{{y}}+\\int_{{0}}^{{\\zeta}}{a}d{a})^{{\\frac{{1}}{{p}}}}+{a})", a = a);
        let big = format!("G{a}={c}+{c}+{c}+{c}+{c}={a}", a = a, c = chunk);
        let md = format!("# T\n\n$${big}$$\n");
        let mut s = spec();
        s.diagrams.clear();
        s.table_captions.clear();
        let r = build_document(&md, "", &s).unwrap();
        // Split BEFORE `=`: the lead stands alone, continuations
        // left-align under it (no dangling `A =` over a centered body).
        assert!(r.tex.contains("\\begin{aligned}"), "breaks left");
        assert!(r.tex.contains("& G\\vartheta"), "lead row kept");
        assert!(r.tex.contains("& ="), "relation opens continuation");
        assert!(!r.tex.contains("multline"), "no centering");
    }

    #[test]
    #[ignore]
    fn dump_estimates() {
        let Ok(path) = std::env::var("IKAT_ESTIMATE_FILE") else {
            eprintln!("set IKAT_ESTIMATE_FILE");
            return;
        };
        // Two modes: a markdown file of `$$...$$` spans, or a plain
        // file with one raw body per line (`IKAT_ESTIMATE_RAW=1`).
        let raw = std::env::var("IKAT_ESTIMATE_RAW").is_ok();
        let md = std::fs::read_to_string(&path).expect("read md");
        let mut i = 0usize;
        for line in md.lines() {
            let s = line.trim();
            let body = if raw {
                if s.is_empty() {
                    continue;
                }
                s
            } else {
                if !(s.starts_with("$$") && s.ends_with("$$") && s.len() > 4) {
                    continue;
                }
                &s[2..s.len() - 2]
            };
            let w = display_math_width_cm(body);
            println!("EST\t{}\t{:.3}", i, w);
            i += 1;
        }
        eprintln!("spans: {}", i);
    }

    #[test]
    fn display_math_no_split_inside_frac_or_matrix() {
        // No top-level operator: one row, however wide.
        let md = "# T\n\n$$\\frac{aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa}{b}$$\n";
        let rows = break_display_rows(&md[5..md.len() - 3]);
        assert_eq!(rows.len(), 1, "frac never splits");
    }

    #[test]
    fn display_math_rejects_alien_environments() {
        let mut s = spec();
        s.diagrams.clear();
        s.table_captions.clear();
        let md = "# T\n\n$$\\begin{equation}x\\end{equation}$$\n";
        let err = build_document(md, "", &s).unwrap_err();
        assert!(err.msg.contains("except matrix"), "got {}", err.msg);
    }

    #[test]
    fn strip_extra_package_detected() {
        assert!(tex_extra_packages("\\begin{strip}x").contains(&"\\usepackage{cuted}".to_string()));
    }

    #[test]
    fn float_tuning_and_section_barriers() {
        let md = "# T\n\n## 1. A\n\nx\n\n## 2. B\n\ny\n";
        let mut s = spec();
        s.diagrams.clear();
        s.table_captions.clear();
        let toml = "[floats]\ntopfraction = 0.9\nbarrier_sections = true\npos_default = \"page\"\n";
        let r = build_document(md, toml, &s).unwrap();
        assert!(r.tex.contains("\\renewcommand{\\topfraction}{0.9}"));
        assert_eq!(r.tex.matches("\\FloatBarrier").count(), 2);
        assert!(r.tex.contains("\\usepackage{placeins}"));
    }

    #[test]
    fn extra_packages_heuristic() {
        assert!(tex_extra_packages("plain").is_empty());
        let got = tex_extra_packages("\\begin{figure}[H]x\\FloatBarrier");
        assert!(got.contains(&"\\usepackage{float}".to_string()));
        assert!(got.contains(&"\\usepackage{placeins}".to_string()));
    }

    #[test]
    fn skeleton_renders_and_guards() {
        let skel = "\\documentclass{article}\n\\title{{title}}\n\\begin{document}\n{{body}}\n{{bibliography}}\n\\end{document}\n";
        let r = render_skeleton(skel, "T", "A", "", "", "B", "\\bibliography{refs}").unwrap();
        assert!(r.contains("\\titleT"));
        assert!(r.contains("B\n\\bibliography{refs}"));
        // Missing body, doubled body, unknown token, missing bib.
        assert!(render_skeleton("no tokens", "T", "A", "", "", "B", "bib").is_err());
        assert!(render_skeleton("{{body}}{{body}}", "T", "A", "", "", "B", "").is_err());
        assert!(render_skeleton("{{body}}{{typo}}", "T", "A", "", "", "B", "").is_err());
        assert!(render_skeleton("{{body}}", "T", "A", "", "", "B", "nonempty-bib").is_err());
        // Empty bib/abstract tokens render as nothing.
        let r = render_skeleton("{{body}}\n{{bibliography}}\n{{abstract}}", "T", "A", "", "", "B", "").unwrap();
        assert_eq!(r, "B\n\n");
        // Non-empty abstract requires its token.
        assert!(render_skeleton("{{body}}", "T", "A", "", "ABS", "B", "").is_err());
        let r = render_skeleton("{{abstract}}\n{{body}}", "T", "A", "", "ABS", "B", "").unwrap();
        assert!(r.starts_with("ABS\nB"));
        // thanks strip mirrors level 1 (`\thanks{{{thanks}}}` convention).
        let r = render_skeleton("\\title{T\\thanks{{{thanks}}}}\n{{body}}", "T", "A", "", "", "B", "").unwrap();
        assert!(!r.contains("thanks"));
        let r = render_skeleton("\\title{T\\thanks{{{thanks}}}}\n{{body}}", "T", "A", "nth", "", "B", "").unwrap();
        assert!(r.contains("\\title{T\\thanks{nth}}"));
    }

    #[test]
    fn skeleton_ignores_comment_tokens() {
        let skel = "% {{body}} documented here\nreal {{body}} here\n";
        let r = render_skeleton(skel, "T", "A", "", "", "B", "").unwrap();
        assert!(r.contains("% {{body}} documented here\nreal B here"));
    }

    #[test]
    fn skeleton_end_to_end_and_exclusion() {
        let md = "# T\n\n## 1. I\n\nHi.\n";
        let skel = "\\documentclass{article}\n\\usepackage{tabularx}\n\\begin{document}\n\\title{{title}}\n\\author{{author}}\n\\maketitle\n{{body}}\n{{bibliography}}\n\\end{document}\n";
        let mut s = spec();
        s.skeleton = skel.to_string();
        s.diagrams.clear();
        s.table_captions.clear();
        let r = build_document(md, "", &s).unwrap();
        assert!(r.tex.contains("\\titleT\n\\authoropticsWolf\n\\maketitle"));
        assert!(r.tex.contains("\\bibliographystyle{IEEEtran}\n\\bibliography{refs-paper}"));
        assert!(!r.tex.contains("{{"));
        // Mutual exclusion with the override path.
        s.preamble_override = "\\documentclass{article}".to_string();
        assert!(build_document(md, "", &s).is_err());
    }

    #[test]
    fn fence_count_mismatch() {
        let mut s = spec();
        s.diagrams.clear();
        assert!(build_document("```mermaid\ngraph TD\na[x]\n```\n", "", &s).is_err());
    }

}
