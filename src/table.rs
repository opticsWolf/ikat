//! Pipe tables → `tabularx`, and `.bib` utilities.
//! Byte-parity with the paper build's table stage is the contract.

use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

use crate::esc::inline;

fn sep_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^:?-{2,}:?$").unwrap())
}

fn field_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"(?m)^\s*([A-Za-z]+)\s*=").unwrap())
}

fn key_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"@\w+\{([^,\s]+)\s*,").unwrap())
}

pub fn split_row(line: &str) -> Vec<String> {
    line.trim().trim_matches('|').split('|').map(|c| c.trim().to_string()).collect()
}

pub fn is_sep(line: &str) -> bool {
    let cells = split_row(line);
    !cells.is_empty() && cells.iter().all(|c| sep_re().is_match(c))
}

/// Full `table` float. First row is bolded; `spec` overrides the
/// default all-`X` column string (e.g. the appendix `Xcl`).
pub fn table_block(
    rows: &[String],
    cap: &str,
    keys: Option<&HashSet<String>>,
    spec: Option<&str>,
) -> Result<String, String> {
    let n = rows.iter().map(|r| split_row(r).len()).max().unwrap_or(0);
    let mut lines = vec![
        "\\begin{table}[t]".to_string(),
        format!("\\caption{{{cap}}}"),
        "\\small".to_string(),
        format!("\\begin{{tabularx}}{{\\columnwidth}}{{{}}}", spec.unwrap_or(&"X".repeat(n))),
        "\\hline".to_string(),
    ];
    let mut first = true;
    for r in rows {
        if is_sep(r) {
            continue;
        }
        let mut cells = split_row(r);
        cells.resize(n, String::new());
        let mut rendered = Vec::new();
        for c in &cells {
            let mut cell = inline(c, keys)?;
            if first {
                cell = format!("\\textbf{{{cell}}}");
            }
            rendered.push(cell);
        }
        lines.push(rendered.join(" & ") + " \\\\");
        if first {
            lines.push("\\hline".to_string());
            first = false;
        }
    }
    lines.push("\\hline".to_string());
    lines.push("\\end{tabularx}".to_string());
    lines.push("\\end{table}".to_string());
    Ok(lines.join("\n"))
}

/// Every `@type{key,` entry key in a .bib source.
pub fn bib_keys_of(bib_src: &str) -> HashSet<String> {
    key_re().captures_iter(bib_src).map(|c| c[1].to_string()).collect()
}

/// Escape `_#%&` when not backslash-escaped (regex has no look-behind,
/// so this is a char scan).
fn escape_field_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_backslash = false;
    for c in s.chars() {
        if (c == '_' || c == '#' || c == '%' || c == '&') && !prev_backslash {
            out.push('\\');
        }
        prev_backslash = c == '\\' && !prev_backslash;
        out.push(c);
    }
    out
}

/// BibTeX-safe derivative: brace bare authors, escape specials outside
/// author/url/doi fields. Pure function over source text.
pub fn bib_safe(bib_src: &str) -> String {
    let author_re = Regex::new(r"^(\s*author\s*=\s*\{)(.*)(\},\s*)$").unwrap();
    let mut out = Vec::new();
    for raw in bib_src.lines() {
        let mut line = raw.to_string();
        if let Some(cap) = author_re.captures(raw) {
            if !cap[2].starts_with('{') {
                line = format!("{}{{{}}}{}", &cap[1], &cap[2], &cap[3]);
            }
        }
        if let Some(f) = field_re().captures(raw) {
            match f[1].to_lowercase().as_str() {
                "author" | "url" | "doi" => {}
                _ => line = escape_field_text(&line),
            }
        }
        out.push(line);
    }
    let mut s = out.join("\n");
    s.push('\n');
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sep_detection() {
        assert!(is_sep("| --- | :---: | --: |"));
        assert!(!is_sep("| a | b |"));
    }

    #[test]
    fn block_shape() {
        let rows = ["| A | B |".to_string(), "|--|--|".to_string(), "| `x_y` | 3 |".to_string()];
        let got = table_block(&rows, "Cap.", None, None).unwrap();
        assert!(got.contains("\\begin{tabularx}{\\columnwidth}{XX}"));
        assert!(got.contains("\\textbf{A} & \\textbf{B} \\\\"));
        assert!(got.contains("\\texttt{x\\_y} & 3 \\\\"));
    }

    #[test]
    fn appendix_spec() {
        let rows = ["| A | B | C |".to_string(), "| 1 | 2 | 3 |".to_string()];
        let got = table_block(&rows, "C.", None, Some("Xcl")).unwrap();
        assert!(got.contains("{Xcl}"));
    }

    #[test]
    fn bib_roundtrip() {
        let src = "@article{k,\n  author = {A. B and C. D},\n  title = {x_y and R&D},\n  url = {http://e/x_y},\n}\n";
        let safe = bib_safe(src);
        assert!(safe.contains("author = {{A. B and C. D}},"));
        assert!(safe.contains("title = {x\\_y and R\\&D},"));
        assert!(safe.contains("url = {http://e/x_y},"));
        let keys = bib_keys_of(src);
        assert!(keys.contains("k"));
    }
}
