//! Inline text: citations, code spans, math protection, escaping,
//! emphasis. Byte-parity with the paper build's inline stage is the
//! contract — the golden test diffs full documents.

use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

fn cite_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\[([^\[\]]+)\]").unwrap())
}

fn cite_inner_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^(?:(.*?):\s*)?`([^`]+)`$").unwrap())
}

pub(crate) fn image_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("!\\[([^\\]]*)\\]\\(([^)\\s]+)(?:\\s+\"[^\"]*\")?\\)").unwrap())
}

fn sub_html_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("<sub>([^<>]*)</sub>").unwrap())
}

fn sup_html_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("<sup>([^<>]*)</sup>").unwrap())
}

fn code_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"`([^`]+)`").unwrap())
}

fn bold_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\*\*(.+?)\*\*").unwrap())
}

fn italic_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\*(.+?)\*").unwrap())
}

fn sub_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("([A-Za-z0-9\\)\\]])([₀₁₂₃ₙᵢ])").unwrap())
}

fn cub_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new("([0-9])³").unwrap())
}

const UNI: &[(&str, &str)] = &[
    ("§", "\\S{}"),
    ("—", "---"),
    ("–", "--"),
    ("…", "\\dots{}"),
    ("≈", "$\\approx$"),
    ("∅", "$\\emptyset$"),
    ("≤", "$\\leq$"),
    ("×", "$\\times$"),
    ("→", "$\\to$"),
    ("≥", "$\\geq$"),
    ("µ", "\\textmu{}"),
    ("₂", "$_{2}$"),
    ("₁", "$_{1}$"),
    ("₃", "$_{3}$"),
];

const SYM_TOKENS: &[(&str, &str)] = &[
    ("✓", "$\\checkmark$"),
    ("○", "$\\circ$"),
    ("◐", "$\\oslash$"),
    ("†", "\\dag{}"),
];

const ESCAPES: &[(char, &str)] = &[
    ('\\', "\\textbackslash{}"),
    ('&', "\\&"),
    ('%', "\\%"),
    ('#', "\\#"),
    ('_', "\\_"),
    ('{', "\\{"),
    ('}', "\\}"),
    ('~', "$\\sim$"),
    ('^', "\\textasciicircum{}"),
];

fn sub_map(c: char) -> Option<char> {
    match c {
        '₀' => Some('0'),
        '₁' => Some('1'),
        '₂' => Some('2'),
        '₃' => Some('3'),
        'ₙ' => Some('n'),
        'ᵢ' => Some('i'),
        _ => None,
    }
}

pub fn esc_text(s: &str) -> String {
    let mut s = s.to_string();
    for (ch, rep) in ESCAPES {
        s = s.replace(*ch, rep);
    }
    for (ch, rep) in UNI.iter().chain(SYM_TOKENS.iter()) {
        s = s.replace(*ch, rep);
    }
    s = sub_re()
        .replace_all(&s, |cap: &regex::Captures| {
            let c = cap[2].chars().next().unwrap();
            format!("${}_{{{}}}$", &cap[1], sub_map(c).unwrap())
        })
        .into_owned();
    s = cub_re()
        .replace_all(&s, |cap: &regex::Captures| format!("${}^{{3}}$", &cap[1]))
        .into_owned();
    // Combining folds + precomposed extras (probe follow-up): the
    // gate allows them, so the emitter must render them.
    crate::unicode::transliterate_extra(&s).into_owned()
}

pub fn esc_code(s: &str) -> String {
    let mut s = s.to_string();
    for (ch, rep) in ESCAPES {
        s = s.replace(*ch, rep);
    }
    for (ch, rep) in UNI.iter().chain(SYM_TOKENS.iter()) {
        s = s.replace(*ch, rep);
    }
    // Combining folds + precomposed extras (probe follow-up).
    crate::unicode::transliterate_extra(&s).into_owned()
}

/// Split into (is_math, segment) on `$...$` spans.
fn protect_math(text: &str) -> Vec<(bool, String)> {
    let mut parts = Vec::new();
    let mut buf = String::new();
    let mut math = false;
    for c in text.chars() {
        if c == '$' {
            parts.push((math, std::mem::take(&mut buf)));
            math = !math;
        } else {
            buf.push(c);
        }
    }
    parts.push((math, buf));
    parts
}

fn stash(ph: &mut Vec<String>, s: String) -> String {
    ph.push(s);
    format!("\x00{}\x00", ph.len() - 1)
}

/// The error carries the offending key separately so `inline`
/// can place the caret on it.
fn cite_group(inner: &str, keys: Option<&HashSet<String>>, ph: &mut Vec<String>) -> Result<String, (String, String)> {
    if !inner.contains('`') {
        return Ok(format!("[{inner}]"));
    }
    let mut outs = Vec::new();
    for p in inner.split(';').map(str::trim) {
        if let Some(mm) = cite_inner_re().captures(p) {
            let author = mm.get(1).map(|m| m.as_str());
            let key = &mm[2];
            if let Some(set) = keys {
                if !set.contains(key) {
                    return Err((format!("dangling citation key `{key}`"), key.to_string()));
                }
            }
            match author {
                Some(a) if !a.is_empty() => {
                    outs.push(format!("{}~\\cite{{{}}}", inline_inner(a, keys, ph)?, key))
                }
                _ => outs.push(format!("\\cite{{{key}}}")),
            }
        } else {
            outs.push(inline_inner(p, keys, ph)?);
        }
    }
    let joined = outs.join(", ");
    Ok(stash(ph, joined))
}

fn inline_inner(text: &str, keys: Option<&HashSet<String>>, ph: &mut Vec<String>) -> Result<String, (String, String)> {
    // Bracket groups (citations).
    let re = cite_re();
    let mut buf = String::new();
    let mut last = 0;
    for m in re.find_iter(text) {
        buf.push_str(&text[last..m.start()]);
        buf.push_str(&cite_group(&m.as_str()[1..m.as_str().len() - 1], keys, ph)?);
        last = m.end();
    }
    buf.push_str(&text[last..]);
    let mut text = buf;

    // Code spans.
    let mut buf = String::new();
    let mut last = 0;
    for m in code_re().captures_iter(&text.clone()) {
        let whole = m.get(0).unwrap();
        buf.push_str(&text[last..whole.start()]);
        buf.push_str(&stash(ph, format!("\\texttt{{{}}}", esc_code(&m[1]))));
        last = whole.end();
    }
    buf.push_str(&text[last..]);
    text = buf;

    // Math-aware emphasis (probe #4): stash dollar-math spans as
    // placeholder slots FIRST, so a bold span crossing math matches
    // as one span and emits textbf around verbatim math (valid
    // LaTeX). Math restores after emphasis; escapes never touch it.
    let mut mixed = String::new();
    for (is_math, seg) in protect_math(&text) {
        if is_math {
            // Greek auto-map (probe follow-up): math restores
            // verbatim, so mapped letters render bare here.
            mixed.push_str(&stash(ph, format!("${}$", crate::unicode::map_greek_math(&seg))));
        } else {
            mixed.push_str(&seg);
        }
    }
    let mut seg = esc_text(&mixed);
        seg = bold_re()
            .replace_all(&seg, |cap: &regex::Captures| format!("\\textbf{{{}}}", &cap[1]))
            .into_owned();
        seg = italic_re()
            .replace_all(&seg, |cap: &regex::Captures| format!("\\textit{{{}}}", &cap[1]))
            .into_owned();
    Ok(seg)
}

/// Full inline render with placeholder restore (descending indices —
/// nested stashes expand inside-out). `line` is the 1-based source
/// line for errors (the caller knows it; the echo is the text
/// itself, which always contains the offending key).
pub fn inline(text: &str, keys: Option<&HashSet<String>>, line: usize) -> Result<String, crate::error::Error> {
    // Unicode gate (probe #2): fail here with file:line:, never
    // inside the TeX log. Mapped characters pass; everything else
    // non-ASCII errors (math segments included — raw unicode is
    // fatal in pdflatex math too).
    crate::unicode::check(text, line)?;
    let mut text = text.to_string();
    // Compat (0.13.3): figures render as their alt-text (remote
    // graphics are not embedded) or drop when alt-less (equation
    // images) — literal `![](...)` in the PDF was silent
    // corruption. Runs before citations (which would match `[..]`).
    text = image_re()
        .replace_all(&text, |cap: &regex::Captures| {
            let alt = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            crate::notes::record_image(alt.is_empty());
            alt.to_string()
        })
        .into_owned();
    // Compat (0.13.3): `<sub>`/`<sup>` become chemistry math
    // (`H$_{2}$O` renders); other inline HTML stays verbatim.
    text = sub_html_re()
        .replace_all(&text, |cap: &regex::Captures| {
            crate::notes::record_tag(true);
            format!("$_{{{}}}$", &cap[1])
        })
        .into_owned();
    text = sup_html_re()
        .replace_all(&text, |cap: &regex::Captures| {
            crate::notes::record_tag(false);
            format!("$^{{{}}}$", &cap[1])
        })
        .into_owned();
    let mut ph = Vec::new();
    let mut text = inline_inner(&text, keys, &mut ph)
        .map_err(|(m, key)| {
            // Caret on the key's backticked occurrence (char
            // columns — the echo may hold multibyte text).
            let mut e = crate::error::Error::new(m, line, text.to_string());
            let pat = format!("`{key}`");
            if let Some(b) = text.find(&pat) {
                e.col = Some(text[..b].chars().count() + 1);
            }
            e
        })?;
    for i in (0..ph.len()).rev() {
        text = text.replace(&format!("\x00{i}\x00"), &ph[i].clone());
    }
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_keys() -> Option<HashSet<String>> {
        None
    }

    #[test]
    fn single_cite() {
        assert_eq!(inline("[`snodgrass90`]", no_keys().as_ref(), 1).unwrap(), "\\cite{snodgrass90}");
    }

    #[test]
    fn author_cite_group() {
        let keys: HashSet<String> =
            ["branching_temporal_dbs_LNCS639".into(), "sarda_reddy_1999".into()].into();
        let got = inline(
            "[Snodgrass survey chapter: `branching_temporal_dbs_LNCS639`; Sarda and Reddy 1999: `sarda_reddy_1999`; branching-time algebra, no open copy]",
            Some(&keys),
            7,
        )
        .unwrap();
        assert_eq!(
            got,
            "Snodgrass survey chapter~\\cite{branching_temporal_dbs_LNCS639}, Sarda and Reddy 1999~\\cite{sarda_reddy_1999}, branching-time algebra, no open copy"
        );
    }

    #[test]
    fn dangling_key_fails() {
        let keys: HashSet<String> = ["a".into()].into();
        assert!(inline("[`nope`]", Some(&keys), 3).is_err());
        // Without a keyset there is nothing to validate against.
        assert!(inline("[`nope`]", None, 3).is_ok());
    }

    #[test]
    #[test]
    #[test]
    fn greek_wraps_in_text_maps_bare_in_math() {
        let got = inline("R(λ) spectra", no_keys().as_ref(), 1).unwrap();
        assert!(got.contains("$\\lambda$"), "got: {got}");
        let got = inline("val $σ + π$", no_keys().as_ref(), 1).unwrap();
        assert!(got.contains("\\sigma + \\pi"), "got: {got}");
        assert!(!got.contains("$$"), "no nested math: {got}");
    }

    #[test]
    fn combining_folds_to_text_accents() {
        let got = inline("n\u{302} over \u{1e91}", no_keys().as_ref(), 1).unwrap();
        assert!(got.contains("\\^{n}"), "got: {got}");
        assert!(got.contains("\\v{z}"), "got: {got}");
        // ... but not inside math.
        let e = inline("val $n\u{302}$", no_keys().as_ref(), 1).unwrap_err();
        assert!(e.msg.contains("\\hat{n}"), "got: {e}");
    }

    #[test]
    fn math_protected_from_escapes() {
        let got = inline("fork at $r$ about $v$ and 1.1--1.3$x$", no_keys().as_ref(), 1).unwrap();
        assert!(got.contains("$r$"));
        assert!(got.contains("1.1--1.3$x$"));
    }

    #[test]
    fn code_unicode_emphasis() {
        let got = inline("`as_of_valid(v)` §3.2 and *true at v* and **bold**", no_keys().as_ref(), 1).unwrap();
        assert!(got.contains("\\texttt{as\\_of\\_valid(v)}"));
        assert!(got.contains("\\S{}3.2"));
        assert!(got.contains("\\textit{true at v}"));
        assert!(got.contains("\\textbf{bold}"));
    }

    #[test]
    fn bold_across_math_supported() {
        // Probe #4: emphasis spanning math emits textbf around
        // verbatim math instead of literal stars.
        let got = inline(
            "**$P(z)$ depends only on the current $n(z)$**",
            no_keys().as_ref(),
            1,
        )
        .unwrap();
        assert!(!got.contains("**"), "literal stars survive: {got}");
        assert!(got.contains("$P(z)$"), "math kept: {got}");
        assert!(got.contains("$n(z)$"), "math kept: {got}");
        assert!(got.starts_with("\\textbf{"), "wrapped: {got}");
    }

    #[test]
    fn italic_across_math_supported() {
        let got = inline("*rate $r$ rises*", no_keys().as_ref(), 1).unwrap();
        assert!(!got.contains('*'), "literal star survives: {got}");
        assert!(got.contains("$r$"), "math kept: {got}");
        assert!(got.starts_with("\\textit{"), "wrapped: {got}");
    }
}
