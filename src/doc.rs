//! Block parsing and document assembly: headings, fences, tables,
//! paragraphs, abstract, appendix, preamble. Byte-parity with the
//! paper build's `convert()` + `main()` is the contract.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::config::{Config, Span};
use crate::esc::inline;
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
    /// Extra preamble lines, appended just before `\begin{document}`.
    #[serde(default)]
    pub preamble_append: Vec<String>,
}

fn default_plot_dir() -> String {
    "figs/".to_string()
}
fn default_author() -> String {
    "opticsWolf".to_string()
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

fn figure_block(body_tex: &str, caption: &str, label: &str, span: Span) -> String {
    let (env, width) = match span {
        Span::Column => ("figure", "\\columnwidth"),
        Span::Wide => ("figure*", "\\textwidth"),
    };
    let body = if body_tex.trim_start().starts_with("\\begin{tikzpicture}") {
        body_tex.to_string()
    } else {
        format!("\\includegraphics[width={width}]{{{body_tex}}}")
    };
    format!("\\begin{{{env}}}[t]\n\\centering\n{body}\n\\caption{{{caption}}}\n\\label{{{label}}}\n\\end{{{env}}}")
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

/// A user-supplied preamble head must carry a document class and
/// every package the woven body provably needs; otherwise the
/// failure would surface as a cryptic TeX log, far from its cause.
pub fn validate_template(override_tex: &str, body: &str) -> Result<(), String> {
    if crate::texenv::document_class(override_tex).is_none() {
        return Err("template preamble_file has no \\documentclass".to_string());
    }
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
    let missing: Vec<&&str> = required.iter().filter(|r| !have.iter().any(|h| h == **r)).collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "template preamble_file drops packages the body needs: {}",
            missing.into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")
        ))
    }
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

pub fn convert(lines: &[String], spec: &BuildSpec, cfg: &Config) -> Result<(String, String, usize, usize), String> {
    let keys = if spec.bib_keys.is_empty() { None } else { Some(&spec.bib_keys) };
    let mut out: Vec<String> = Vec::new();
    let mut para: Vec<String> = Vec::new();
    let mut title = String::new();
    let mut in_abstract = false;
    let mut diagram_idx = 0usize;
    let mut table_idx = 0usize;

    let flush = |out: &mut Vec<String>, para: &mut Vec<String>| -> Result<(), String> {
        if !para.is_empty() {
            let mut text = inline(&para.join(" "), keys)?;
            if text.starts_with("\\dag{}") {
                text = format!("\\textit{{Note: }}{}", &text["\\dag{}".len()..]);
            }
            out.push(text + "\n");
            para.clear();
        }
        Ok(())
    };

    let mut i = 0;
    while i < lines.len() {
        let s = lines[i].trim();
        if s.is_empty() {
            flush(&mut out, &mut para)?;
            i += 1;
            continue;
        }
        if s.starts_with('>') {
            i += 1;
            continue;
        }
        if s.starts_with("```") {
            flush(&mut out, &mut para)?;
            if s.contains("mermaid") {
                if diagram_idx >= spec.diagrams.len() {
                    return Err(format!("mermaid fence #{diagram_idx} has no registry entry"));
                }
                let mut fence = Vec::new();
                let mut j = i + 1;
                while j < lines.len() && !lines[j].trim().starts_with("```") {
                    fence.push(lines[j].trim_end_matches('\n').to_string());
                    j += 1;
                }
                let entry = &spec.diagrams[diagram_idx];
                let mode = if s.contains("{inline}") { "inline" } else { entry.mode.as_str() };
                let span = cfg.spans.for_kind("diagram");
                let body = if mode == "inline" {
                    flowchart_to_tikz(&fence.join("\n"))?
                } else {
                    format!("{}{}", entry.path_prefix, entry.key)
                };
                out.push(
                    figure_block(
                        &body,
                        &inline(&entry.caption, keys)?,
                        &format!("fig:{}", entry.key),
                        span,
                    ) + "\n",
                );
                diagram_idx += 1;
                i = j + 1;
                continue;
            }
            i += 1;
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                i += 1;
            }
            i += 1;
            continue;
        }
        if s.starts_with('|') {
            flush(&mut out, &mut para)?;
            let mut rows = Vec::new();
            while i < lines.len() && lines[i].trim().starts_with('|') {
                rows.push(lines[i].trim().to_string());
                i += 1;
            }
            if table_idx >= spec.table_captions.len() {
                return Err(format!("table #{table_idx} has no caption"));
            }
            out.push(
                table_block(
                    &rows,
                    &spec.table_captions[table_idx],
                    keys,
                    spec.table_specs.get(&table_idx.to_string()).map(String::as_str),
                )? + "\n",
            );
            table_idx += 1;
            continue;
        }
        if s.starts_with("## Appendix") {
            flush(&mut out, &mut para)?;
            out.push("\\appendix\n\\section{Claim-to-decision map}\n".to_string());
            i += 1;
            continue;
        }
        if s.starts_with("## Abstract") {
            flush(&mut out, &mut para)?;
            out.push("\\begin{abstract}\n".to_string());
            in_abstract = true;
            i += 1;
            continue;
        }
        if s.starts_with("## ") {
            flush(&mut out, &mut para)?;
            if in_abstract {
                out.push("\\end{abstract}\n".to_string());
                in_abstract = false;
            }
            let head = strip_section_number(&s[3..]);
            out.push(format!("\\section{{{}}}\n", inline(&head, keys)?));
            i += 1;
            continue;
        }
        if s.starts_with("### ") {
            flush(&mut out, &mut para)?;
            let head = strip_section_number(&s[4..]);
            out.push(format!("\\subsection{{{}}}\n", inline(&head, keys)?));
            i += 1;
            continue;
        }
        if s.starts_with("# ") {
            title = inline(&s[2..], keys)?;
            i += 1;
            continue;
        }
        para.push(s.to_string());
        i += 1;
    }
    flush(&mut out, &mut para)?;
    if in_abstract {
        out.push("\\end{abstract}\n".to_string());
    }
    Ok((title, out.join("\n"), diagram_idx, table_idx))
}

pub fn build_document(md_text: &str, toml_src: &str, spec: &BuildSpec) -> Result<BuildResult, String> {
    let cfg = Config::from_toml(toml_src)?;
    let lines: Vec<String> = md_text.lines().map(|l| l.to_string()).collect();
    let (title, mut body, n_diagrams, n_tables) = convert(&lines, spec, &cfg)?;
    if n_diagrams != spec.diagrams.len() {
        return Err(format!(
            "{n_diagrams} fences vs {} registry entries",
            spec.diagrams.len()
        ));
    }
    if n_tables != spec.table_captions.len() {
        return Err(format!(
            "{n_tables} tables vs {} captions",
            spec.table_captions.len()
        ));
    }
    if !spec.plots.is_empty() {
        let mut figs = Vec::new();
        for (key, cap) in &spec.plots {
            figs.push(figure_block(
                &format!("{}{}", spec.plot_dir, key),
                cap,
                &format!("fig:{key}"),
                cfg.spans.for_kind("plot"),
            ));
        }
        let plot_figs = figs.join("\n");
        if !spec.plot_insert_before.is_empty() && body.contains(&spec.plot_insert_before) {
            body = body.replacen(&spec.plot_insert_before, &format!("{plot_figs}\n{}", spec.plot_insert_before), 1);
        } else {
            body = format!("{body}\n{plot_figs}\n");
        }
    }
    let paths: String = spec.graphicspaths.iter().map(|p| format!("{{{p}}}")).collect();
    let opts = cfg.document.class_options.join(",");
    let has_tikz = body.contains("\\begin{tikzpicture}");
    let has_plots = body.contains("\\begin{axis}");
    let mut head: Vec<String> = if spec.preamble_override.is_empty() {
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
        generated.extend([
            "\\usepackage[hidelinks]{hyperref}".to_string(),
            format!("\\graphicspath{{{paths}}}"),
            format!("\\title{{{title}\\thanks{{{}}}}}", spec.title_thanks),
            format!("\\author{{\\IEEEauthorblockN{{{}}}}}", spec.author),
        ]);
        generated
    } else {
        validate_template(&spec.preamble_override, &body)?;
        vec![spec.preamble_override.clone()]
    };
    head.extend(spec.preamble_append.clone());
    head.extend([
        "\\begin{document}".to_string(),
        "\\maketitle".to_string(),
        body.clone(),
        "\\bibliographystyle{IEEEtran}".to_string(),
        format!("\\bibliography{{{}}}", spec.bib_name),
        "\\end{document}".to_string(),
        String::new(),
    ]);
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
        assert!(err.contains("tikz"), "unexpected: {err}");
    }

    #[test]
    fn template_without_class_is_an_error() {
        let err = validate_template("\\usepackage{tikz}\n", "plain").unwrap_err();
        assert!(err.contains("documentclass"));
    }

    #[test]
    fn fence_count_mismatch() {
        let mut s = spec();
        s.diagrams.clear();
        assert!(build_document("```mermaid\ngraph TD\na[x]\n```\n", "", &s).is_err());
    }
}
