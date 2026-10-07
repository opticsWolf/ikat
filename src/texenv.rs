//! Preamble scan → TeX Live package needs (the texliveonfly trick).
//!
//! `used_packages` parses `\usepackage` / `\documentclass` out of any
//! `.tex` source (comments stripped, TikZ/pgfplots use inferred), and
//! `package_needs` maps LaTeX names to `(probe file, tlmgr package)`
//! pairs. Touching the disk (kpsewhich) and installing (tlmgr) stays
//! in Python (`texenv.py`), which owns subprocesses; everything here
//! is pure and unit-tested.

/// Cut a line at the first `%` not preceded by a backslash.
fn uncomment(line: &str) -> &str {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && (i == 0 || bytes[i - 1] != b'\\') {
            return &line[..i];
        }
        i += 1;
    }
    line
}

/// Read `[opt]{arg}` at `s`: returns `(options, argument, rest)`.
/// Brackets/braces are assumed un-nested (true for preambles).
fn read_cmd(s: &str) -> Option<(String, &str)> {
    let mut rest = s.trim_start();
    if rest.starts_with('[') {
        let end = rest.find(']')?;
        rest = rest[end + 1..].trim_start();
    }
    let body = rest.strip_prefix('{')?;
    let end = body.find('}')?;
    Some((body[..end].to_string(), &body[end + 1..]))
}

/// All `\usepackage` names in a `.tex` source, in order, no duplicates.
/// `\usetikzlibrary` / pgfplots use imply their packages when the
/// corresponding `\usepackage` line is absent (generated preambles
/// sometimes carry use without declaration — be liberal).
pub fn used_packages(tex: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut push = |name: &str| {
        let name = name.trim();
        if !name.is_empty() && !out.iter().any(|n| n == name) {
            out.push(name.to_string());
        }
    };
    let mut saw_tikzlib = false;
    let mut saw_plots = false;
    for line in tex.lines() {
        let mut rest = uncomment(line);
        loop {
            let pu = rest.find("\\usepackage");
            let dc = rest.find("\\documentclass");
            let (i, tok, is_pkg) = match (pu, dc) {
                (Some(a), Some(b)) => {
                    if a <= b {
                        (a, "\\usepackage", true)
                    } else {
                        (b, "\\documentclass", false)
                    }
                }
                (Some(a), None) => (a, "\\usepackage", true),
                (None, Some(b)) => (b, "\\documentclass", false),
                (None, None) => break,
            };
            // Both commands take `[opt]{arg}`, so one parser serves;
            // the class name is not a package, so only push for
            // `\usepackage` (the class is handled separately).
            let after = &rest[i + tok.len()..];
            match read_cmd(after) {
                Some((arg, tail)) => {
                    if is_pkg {
                        for name in arg.split(',') {
                            push(name);
                        }
                    }
                    rest = tail;
                }
                None => break,
            }
        }
        if rest.contains("\\usetikzlibrary") {
            saw_tikzlib = true;
        }
        if rest.contains("\\pgfplotsset") || rest.contains("\\begin{axis}") {
            saw_plots = true;
        }
    }
    if saw_tikzlib && !out.iter().any(|n| n == "tikz") {
        out.push("tikz".to_string());
    }
    if saw_plots && !out.iter().any(|n| n == "pgfplots") {
        out.push("pgfplots".to_string());
    }
    out
}

/// The `\documentclass` name, if any.
pub fn document_class(tex: &str) -> Option<String> {
    for line in tex.lines() {
        let mut rest = uncomment(line);
        loop {
            let Some(i) = rest.find("\\documentclass") else {
                break;
            };
            let after = &rest[i + "\\documentclass".len()..];
            match read_cmd(after) {
                Some((arg, tail)) => {
                    let cls = arg.split(',').next().unwrap_or("").trim();
                    if !cls.is_empty() {
                        return Some(cls.to_string());
                    }
                    rest = tail;
                }
                None => break,
            }
        }
    }
    None
}

/// `(file kpsewhich can probe, tlmgr package to install)` for a
/// LaTeX package/class name. Unknown names fall back to the tlmgr
/// convention that the package matches the main file stem.
pub fn probe_and_tlmgr(name: &str) -> (String, String) {
    let pair = match name {
        "tikz" => ("tikz.sty", "pgf"),
        "pgfplots" => ("pgfplots.sty", "pgfplots"),
        "amsmath" => ("amsmath.sty", "amsmath"),
        "amssymb" => ("amssymb.sty", "amsmath"),
        "amsthm" => ("amsthm.sty", "amsmath"),
        "tabularx" => ("tabularx.sty", "tools"),
        "array" => ("array.sty", "tools"),
        "graphicx" => ("graphicx.sty", "graphics"),
        "xcolor" => ("xcolor.sty", "graphics"),
        "hyperref" => ("hyperref.sty", "hyperref"),
        "url" => ("url.sty", "url"),
        "lmodern" => ("lmodern.sty", "lmodern"),
        "textcomp" | "inputenc" | "fontenc" => {
            return (format!("{name}.sty"), "latex".to_string());
        }
        "geometry" => ("geometry.sty", "geometry"),
        "booktabs" => ("booktabs.sty", "booktabs"),
        "caption" => ("caption.sty", "caption"),
        "subcaption" => ("subcaption.sty", "caption"),
        "natbib" => ("natbib.sty", "natbib"),
        "article" | "report" | "book" | "letter" => {
            return (format!("{name}.cls"), "latex".to_string());
        }
        "IEEEtran" => ("IEEEtran.cls", "IEEEtran"),
        _ => return (format!("{name}.sty"), name.to_string()),
    };
    (pair.0.to_string(), pair.1.to_string())
}

/// Full need list: mapped packages in first-use order, then the
/// document class (standard classes probe the always-present base).
pub fn package_needs(names: &[String], class: Option<&str>) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for name in names {
        let p = probe_and_tlmgr(name);
        if !out.contains(&p) {
            out.push(p);
        }
    }
    if let Some(cls) = class {
        let p = probe_and_tlmgr(cls);
        if !out.contains(&p) {
            out.push(p);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEX: &str = "% \\usepackage{commented}\n\\documentclass[10pt,conference]{IEEEtran}\n\\usepackage[utf8]{inputenc}\n\\usepackage{amsmath,amssymb}\n\\usepackage{tikz}\n\\usetikzlibrary{arrows.meta}\n\\begin{document}\nHello.\n\\end{document}\n";

    #[test]
    fn scan_finds_packages_and_class() {
        let pkgs = used_packages(TEX);
        assert_eq!(pkgs, vec!["inputenc", "amsmath", "amssymb", "tikz"]);
        assert_eq!(document_class(TEX), Some("IEEEtran".to_string()));
    }

    #[test]
    fn comments_are_ignored() {
        assert!(!used_packages("% \\usepackage{tikz}\n\\begin{document}").contains(&"tikz".to_string()));
    }

    #[test]
    fn inference_adds_missing_parents() {
        let t = "\\usepackage{lmodern}\n\\usetikzlibrary{shapes}\n\\pgfplotsset{compat=1.18}\n";
        let pkgs = used_packages(t);
        assert!(pkgs.contains(&"tikz".to_string()));
        assert!(pkgs.contains(&"pgfplots".to_string()));
    }

    #[test]
    fn no_double_inference_when_declared() {
        let t = "\\usepackage{tikz,pgfplots}\n\\usetikzlibrary{shapes}\n";
        assert_eq!(used_packages(t), vec!["tikz", "pgfplots"]);
    }

    #[test]
    fn mapping_table_spot_checks() {
        assert_eq!(
            probe_and_tlmgr("tikz"),
            ("tikz.sty".to_string(), "pgf".to_string())
        );
        assert_eq!(
            probe_and_tlmgr("tabularx"),
            ("tabularx.sty".to_string(), "tools".to_string())
        );
        assert_eq!(
            probe_and_tlmgr("IEEEtran"),
            ("IEEEtran.cls".to_string(), "IEEEtran".to_string())
        );
        assert_eq!(
            probe_and_tlmgr("article"),
            ("article.cls".to_string(), "latex".to_string())
        );
        assert_eq!(
            probe_and_tlmgr("obscurepkg"),
            ("obscurepkg.sty".to_string(), "obscurepkg".to_string())
        );
    }

    #[test]
    fn needs_dedupes_and_appends_class() {
        let names = vec!["tikz".to_string(), "tikz".to_string()];
        let needs = package_needs(&names, Some("IEEEtran"));
        assert_eq!(needs.len(), 2);
        assert_eq!(needs[1].0, "IEEEtran.cls");
    }
}
