//! Weave-time Unicode gate (probe issues #1/#2).
//!
//! The template heads are `pdflatex` + `inputenc`/`lmodern`/`textcomp`,
//! so any non-ASCII character outside the explicit transliteration
//! tables in `esc.rs` / `emit.rs` dies inside the TeX log (`! LaTeX
//! Error: Unicode character … not set up for use with LaTeX`). The
//! fail-fast doctrine says that must fail here instead, with a
//! `file:line:` diagnostic naming the character and its replacement.
//!
//! Two classes pass: (a) the transliteration table below — the set
//! the emitters replace (`UNI` + `SYM_TOKENS` keys in `esc.rs` plus
//! the subscript / superscript characters handled there); (b) raw
//! pass-through Latin accent ranges the `inputenc` UTF-8 heads
//! already render (author names like Bohlen, Mate, Safarik must
//! survive verbatim — the golden paper carries them raw). Everything
//! else non-ASCII (Greek, Cyrillic, CJK, unmapped symbols) is
//! rejected. ASCII always passes.
//!
//! Third class (needle-math follow-up): combining diacritical marks
//! (U+0300–U+036F) fold to text-mode accents after an ASCII
//! alphanumeric base (`n` + U+0302 becomes `\^{n}`), and the
//! precomposed caron letters Ẑ/ẑ map to `\v{Z}`/`\v{z}`. The fold
//! runs in every text emitter, so gate-allowed input always renders.
//! Inside `$…$` math (and display-math spans, which are verbatim
//! math) combining marks still fail — with a math-accent hint like
//! `$\hat{n}$` — because text accents break math mode.

/// Every non-ASCII character the pipeline transliterates.
/// Must stay in sync with `esc.rs::UNI` + `SYM_TOKENS` and the
/// subscript/superscript handling there, plus `emit.rs::esc_label`.
const ALLOWED: &[char] = &[
    '§', '—', '–', '…', '≈', '∅', '≤', '×', '→', '≥', 'µ', '₂', '₁', '₃', '✓', '○', '◐', '†',
    '₀', 'ₙ', 'ᵢ', '³',
];

/// Raw pass-through: `inputenc` UTF-8 heads render these without any
/// emitter mapping (Latin-1 Supplement + Latin Extended-A/B — the
/// accented author-name vocabulary). Narrower than full inputenc
/// coverage on purpose: anything outside Latin still fails here.
fn is_passthrough(c: char) -> bool {
    matches!(c, '\u{a0}'..='\u{24f}')
}

use std::borrow::Cow;

/// Combining diacritical marks ikat folds (U+0300–U+036F working
/// subset): (mark, text-mode accent command). The fold applies only
/// after an ASCII alphanumeric base; anything else fails with a
/// hint, same as any unmapped character.
const COMBINING: &[(char, &str)] = &[
    ('\u{300}', "\\`"),  // grave
    ('\u{301}', "\\'"),  // acute
    ('\u{302}', "\\^"),  // circumflex
    ('\u{303}', "\\~"),  // tilde
    ('\u{304}', "\\="),  // macron
    ('\u{306}', "\\u"),  // breve
    ('\u{307}', "\\."),  // dot above
    ('\u{308}', "\\\""), // diaeresis
    ('\u{30a}', "\\r"),  // ring above
    ('\u{30b}', "\\H"),  // double acute
    ('\u{30c}', "\\v"),  // caron
    ('\u{327}', "\\c"),  // cedilla
    ('\u{328}', "\\k"),  // ogonek
];

/// Text-mode accent command for one combining mark, if mapped.
fn combining_cmd(mark: char) -> Option<&'static str> {
    COMBINING.iter().find(|(m, _)| *m == mark).map(|(_, c)| *c)
}

/// True for U+0300–U+036F (foldable or not — the gate decides).
fn is_combining_mark(c: char) -> bool {
    matches!(c, '\u{300}'..='\u{36f}')
}

/// Spoken name for gate hints.
fn mark_name(mark: char) -> &'static str {
    match mark {
        '\u{300}' => "combining grave",
        '\u{301}' => "combining acute",
        '\u{302}' => "combining circumflex",
        '\u{303}' => "combining tilde",
        '\u{304}' => "combining macron",
        '\u{306}' => "combining breve",
        '\u{307}' => "dot above",
        '\u{308}' => "combining diaeresis",
        '\u{30a}' => "combining ring above",
        '\u{30b}' => "double acute",
        '\u{30c}' => "combining caron",
        '\u{327}' => "cedilla",
        '\u{328}' => "ogonek",
        _ => "combining mark",
    }
}

/// Math-mode accent for gate hints; marks without a math-mode
/// equivalent get `None` and the hint falls back to text mode.
fn math_accent(mark: char) -> Option<&'static str> {
    match mark {
        '\u{300}' => Some("\\grave"),
        '\u{301}' => Some("\\acute"),
        '\u{302}' => Some("\\hat"),
        '\u{303}' => Some("\\tilde"),
        '\u{304}' => Some("\\bar"),
        '\u{306}' => Some("\\breve"),
        '\u{307}' => Some("\\dot"),
        '\u{308}' => Some("\\ddot"),
        '\u{30c}' => Some("\\check"),
        _ => None,
    }
}

/// Fold ASCII-alnum + combining-mark pairs to text-mode accents
/// (`n` + U+0302 becomes `\^{n}`). Borrowed when nothing folds (the
/// common case: zero allocation). Every text emitter runs this, so
/// whatever the gate allows always renders.
pub fn fold_marks(s: &str) -> Cow<str> {
    if !s.chars().any(is_combining_mark) {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c.is_ascii_alphanumeric() {
            if let Some(&m) = it.peek() {
                if let Some(cmd) = combining_cmd(m) {
                    it.next();
                    out.push_str(cmd);
                    out.push('{');
                    out.push(c);
                    out.push('}');
                    continue;
                }
            }
        }
        out.push(c);
    }
    Cow::Owned(out)
}

/// Transliteration for emitters without their own table (plot ticks,
/// the scalar author, preset prose): combining folds plus the
/// precomposed caron letters. Table-backed emitters (`esc_text`,
/// `esc_label`) run `fold_marks` and carry their own ẑ entries.
pub fn transliterate_extra(s: &str) -> Cow<str> {
    let folded = fold_marks(s);
    if !folded.contains('Ẑ') && !folded.contains('ẑ') {
        return folded;
    }
    let t = folded.replace('Ẑ', "\\v{Z}").replace('ẑ', "\\v{z}");
    Cow::Owned(t)
}

/// Precomposed extras outside the pass-through range (needle math):
/// caron letters, mapped by `transliterate_extra` in emitters without
/// their own table (plot ticks, scalar author, preset prose).
const EXTRA_ALLOWED: &[char] = &['Ẑ', 'ẑ'];

/// ASCII, transliterated, inputenc pass-through Latin, or a
/// precomposed extra mapped by `transliterate_extra`.
pub fn is_allowed(c: char) -> bool {
    c.is_ascii() || ALLOWED.contains(&c) || EXTRA_ALLOWED.contains(&c) || is_passthrough(c)
}

/// First disallowed character in `s`: (char, char-column-1-based).
pub fn find_bad(s: &str) -> Option<(char, usize)> {
    for (i, c) in s.chars().enumerate() {
        if !is_allowed(c) {
            return Some((c, i + 1));
        }
    }
    None
}

/// LaTeX replacement hint for common manuscript characters outside
/// the transliteration table (the reporter's physics-paper vocabulary).
/// `None` = no specific hint; the caller falls back to the generic
/// ASCII-table message.
pub fn hint(c: char) -> Option<&'static str> {
    match c {
        'λ' => Some("$\\lambda$"),
        'θ' => Some("$\\theta$"),
        'ρ' => Some("$\\rho$"),
        'δ' => Some("$\\delta$"),
        'Ψ' => Some("$\\Psi$"),
        'ε' => Some("$\\varepsilon$"),
        'Δ' => Some("$\\Delta$"),
        'α' => Some("$\\alpha$"),
        'β' => Some("$\\beta$"),
        'γ' => Some("$\\gamma$"),
        'Ω' => Some("$\\Omega$"),
        'μ' => Some("$\\mu$ (or $\\micro$ in text)"),
        '±' => Some("$\\pm$"),
        '↑' => Some("$\\uparrow$"),
        '↓' => Some("$\\downarrow$"),
        '₀' => Some("$_{0}$"),
        '₅' => Some("$_{5}$"),
        'ⁿ' => Some("$^{n}$"),
        '·' => Some("$\\cdot$"),
        'ń' => Some("\\'n"),
        'ä' => Some("\\\"a"),
        'ö' => Some("\\\"o"),
        'ü' => Some("\\\"u"),
        'é' => Some("\\'e"),
        'è' => Some("\\`e"),
        // Lone combining marks (no foldable base): name the shape
        // the author must put right after an ASCII letter.
        '\u{300}' => Some("the mark right after an ASCII base letter (e.g. n + U+0300)"),
        '\u{301}' => Some("the mark right after an ASCII base letter (e.g. n + U+0301)"),
        '\u{302}' => Some("the mark right after an ASCII base letter (e.g. n + U+0302)"),
        '\u{303}' => Some("the mark right after an ASCII base letter (e.g. n + U+0303)"),
        '\u{304}' => Some("the mark right after an ASCII base letter (e.g. n + U+0304)"),
        '\u{306}' => Some("the mark right after an ASCII base letter (e.g. n + U+0306)"),
        '\u{307}' => Some("the mark right after an ASCII base letter (e.g. n + U+0307)"),
        '\u{308}' => Some("the mark right after an ASCII base letter (e.g. n + U+0308)"),
        '\u{30a}' => Some("the mark right after an ASCII base letter (e.g. n + U+030A)"),
        '\u{30b}' => Some("the mark right after an ASCII base letter (e.g. n + U+030B)"),
        '\u{30c}' => Some("the mark right after an ASCII base letter (e.g. z + U+030C)"),
        '\u{327}' => Some("the mark right after an ASCII base letter (e.g. c + U+0327)"),
        '\u{328}' => Some("the mark right after an ASCII base letter (e.g. n + U+0328)"),
        _ => None,
    }
}

/// Error message for one offending character.
pub fn msg_for(c: char) -> String {
    let code = format!("U+{:04X}", c as u32);
    match hint(c) {
        Some(rep) => format!(
            "unicode character '{c}' ({code}) is not in the pdflatex subset; write {rep} instead"
        ),
        None => format!(
            "unicode character '{c}' ({code}) is not in the pdflatex subset; write a LaTeX command instead (raw Latin accents pass, Greek and symbols need commands)"
        ),
    }
}

/// One gate finding: the character, its 1-based char column,
/// whether it sits in `$…$` math, and the preceding base letter for
/// math-accent hints (`Some('n')` names `$\hat{n}$` exactly).
struct Bad {
    c: char,
    col: usize,
    math: bool,
    base: Option<char>,
}

/// Split into (is_math, segment): only CLOSED `$…$` spans count as
/// math. An unmatched `$` and everything after it stays text — an
/// unmatched `$` fails elsewhere, and the gate must not mis-hint a
/// combining mark in what the author meant as text.
fn split_math(s: &str) -> Vec<(bool, &str)> {
    let mut segs = Vec::new();
    let mut rest = s;
    while let Some(open) = rest.find('$') {
        let (text, after) = rest.split_at(open);
        match after[1..].find('$') {
            Some(close) => {
                if !text.is_empty() {
                    segs.push((false, text));
                }
                segs.push((true, &after[1..1 + close]));
                rest = &after[1 + close + 1..];
            }
            None => {
                segs.push((false, rest));
                rest = "";
            }
        }
    }
    if !rest.is_empty() {
        segs.push((false, rest));
    }
    segs
}

/// Scan one segment; `col0` is the 1-based char column of its first
/// character. Text segments skip foldable base+mark pairs (the
/// emitters render those); math segments fail every combining mark
/// (math mode is verbatim — text accents would break it).
fn scan_seg(seg: &str, math: bool, col0: usize) -> Option<Bad> {
    let mut prev: Option<char> = None;
    let mut it = seg.chars().enumerate().peekable();
    while let Some((i, c)) = it.next() {
        let col = col0 + i;
        if !math && c.is_ascii_alphanumeric() {
            if let Some(&(_, m)) = it.peek() {
                if combining_cmd(m).is_some() {
                    it.next();
                    prev = None;
                    continue;
                }
            }
        }
        if math && is_combining_mark(c) {
            let base = prev.filter(|b| b.is_ascii_alphanumeric());
            return Some(Bad { c, col, math, base });
        }
        if !is_allowed(c) {
            return Some(Bad { c, col, math, base: None });
        }
        prev = Some(c);
    }
    None
}

/// First finding across `$…$`-aware segments (columns count the
/// whole string, delimiters included, matching the old indexing).
fn first_bad(text: &str) -> Option<Bad> {
    let mut col0 = 1usize;
    for (math, seg) in split_math(text) {
        if math {
            col0 += 1; // opening `$`
        }
        if let Some(b) = scan_seg(seg, math, col0) {
            return Some(b);
        }
        col0 += seg.chars().count();
        if math {
            col0 += 1; // closing `$`
        }
    }
    None
}

/// Message for one finding (math findings name the math accent).
fn msg_for_bad(b: &Bad) -> String {
    if b.math {
        msg_for_math(b.c, b.base)
    } else {
        msg_for(b.c)
    }
}

/// Error message for math-mode findings: combining marks name the
/// exact accent (`$\hat{n}$`); anything else reuses the text hint
/// vocabulary flagged as math context.
pub fn msg_for_math(c: char, base: Option<char>) -> String {
    let code = format!("U+{:04X}", c as u32);
    if is_combining_mark(c) {
        let name = mark_name(c);
        match base.and_then(|b| math_accent(c).map(|a| (b, a))) {
            Some((b, acc)) => format!(
                "unicode {name} ({code}) after '{b}' is not in the pdflatex subset in math mode; write ${acc}{{{b}}}$ instead"
            ),
            None => format!(
                "unicode {name} ({code}) is not in the pdflatex subset in math mode; put it after an ASCII letter or write a LaTeX accent command instead"
            ),
        }
    } else {
        match hint(c) {
            Some(rep) => format!(
                "unicode character '{c}' ({code}) is not in the pdflatex subset in math mode; write {rep} instead"
            ),
            None => format!(
                "unicode character '{c}' ({code}) is not in the pdflatex subset in math mode; write a LaTeX command instead"
            ),
        }
    }
}

/// Check a single text value (paragraph, heading, caption, cell,
/// author name, plot label). `$…$` spans scan under math rules;
/// display-math spans use `check_math` (they are verbatim math).
/// `line` is the 1-based manuscript line; the echo is the text
/// itself (which always contains the offending character).
pub fn check(text: &str, line: usize) -> Result<(), crate::error::Error> {
    if let Some(b) = first_bad(text) {
        return Err(crate::error::Error::new(msg_for_bad(&b), line, text.to_string()).with_col(Some(b.col)));
    }
    Ok(())
}

/// Check a display-math span: the emitter passes math through
/// verbatim, so combining marks fail here with math-accent hints
/// (text accents would break math mode).
pub fn check_math(text: &str, line: usize) -> Result<(), crate::error::Error> {
    if let Some(b) = scan_seg(text, true, 1) {
        return Err(crate::error::Error::new(msg_for_math(b.c, b.base), line, text.to_string()).with_col(Some(b.col)));
    }
    Ok(())
}

/// Check spec/CLI data with no manuscript line (author names,
/// affiliations, thanks, plot labels): line-less `Error::data` so
/// Python renders `path: message` + echo.
pub fn check_data(text: &str) -> Result<(), crate::error::Error> {
    if let Some(b) = first_bad(text) {
        return Err(crate::error::Error::data(msg_for_bad(&b), text.to_string()));
    }
    Ok(())
}

/// Check a multi-line source block (mermaid fence bodies): the error
/// points at the offending body line (`base` = 1-based line of the
/// first body line; the echo is that raw line with its caret).
/// Labels are text mode (mermaid `$` escapes to `\$`), so whole
/// lines scan under text rules.
pub fn check_block(src: &str, base: usize) -> Result<(), crate::error::Error> {
    for (idx, line_text) in src.lines().enumerate() {
        if let Some(b) = scan_seg(line_text, false, 1) {
            return Err(crate::error::Error::new(
                msg_for_bad(&b),
                base + idx,
                line_text.to_string(),
            )
            .with_col(Some(b.col)));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_and_mapped_pass() {
        assert!(check("plain ASCII $x^2$ -- here", 1).is_ok());
        // Every transliterated character passes.
        assert!(check("§ — – … ≈ ∅ ≤ × → ≥ µ ₂₁₃ ✓ ○ ◐ † ₀ₙᵢ ³", 1).is_ok());
    }

    #[test]
    fn greek_fails_with_hint_and_caret() {
        let e = check("R(λ,θ) spectra", 7).unwrap_err();
        assert!(e.msg.contains('λ'), "got: {e}");
        assert!(e.msg.contains("U+03BB"), "got: {e}");
        assert!(e.msg.contains("\\lambda"), "got: {e}");
        assert_eq!((e.line, e.col), (7, Some(3)), "got: {e}");
        assert!(e.echo.contains('λ'), "got: {e}");
    }

    #[test]
    fn block_check_names_body_line() {
        let e = check_block("graph TD\nA[R(λ) spectra]-->B[merit]", 11).unwrap_err();
        assert_eq!(e.line, 12, "got: {e}");
        assert!(e.msg.contains("U+03BB"), "got: {e}");
        assert!(e.echo.contains('λ'), "got: {e}");
    }

    #[test]
    fn unhinted_char_gets_generic_message() {
        let e = check("snowman ☃", 1).unwrap_err();
        assert!(e.msg.contains("U+2603"), "got: {e}");
        assert!(e.msg.contains("LaTeX command"), "got: {e}");
    }

    #[test]
    fn latin_accents_pass_through() {
        // Golden-paper author names ride raw via inputenc.
        assert!(check("Böhlen, Máté, Šafařík, Stantić, Bliujutė", 1).is_ok());
    }

    #[test]
    fn combining_folds_after_ascii_base() {
        // n + U+0302 and z + U+030C pass (the emitters fold them);
        // precomposed z-caron and passthrough A-tilde ride along.
        assert!(check("n\u{302} spectra and z\u{30c}", 1).is_ok());
        assert!(check("Ã ẑ ride along", 1).is_ok());
        assert_eq!(fold_marks("n\u{302}"), "\\^{n}");
        assert_eq!(fold_marks("z\u{30c}"), "\\v{z}");
        assert_eq!(fold_marks("plain"), "plain");
    }

    #[test]
    fn combining_without_base_fails_loud() {
        // Stacked marks: the second has no base.
        let e = check("n\u{302}\u{303}", 1).unwrap_err();
        assert!(e.msg.contains("U+0303"), "got: {e}");
        // Lone mark names itself.
        let e = check("\u{302}", 1).unwrap_err();
        assert!(e.msg.contains("U+0302"), "got: {e}");
        // Non-alnum base does not fold.
        assert!(check("&\u{302}", 1).is_err());
    }

    #[test]
    fn combining_in_math_hints_math_accent() {
        let e = check("energy $n\u{302}$ here", 1).unwrap_err();
        assert!(e.msg.contains("\\hat{n}"), "got: {e}");
        assert!(e.msg.contains("math mode"), "got: {e}");
        // Columns count the whole string, delimiters included.
        assert_eq!(e.col, Some(10), "got: {e}");
    }

    #[test]
    fn unmatched_dollar_stays_text() {
        // No closing `$`: text rules, the pair folds.
        assert!(check("price $5 and n\u{302}", 1).is_ok());
    }

    #[test]
    fn display_math_check_rejects_combining() {
        let e = check_math("x + n\u{303}", 4).unwrap_err();
        assert!(e.msg.contains("\\tilde{n}"), "got: {e}");
        assert_eq!((e.line, e.col), (4, Some(6)), "got: {e}");
    }

    fn prime_still_fails() {
        // U+2032 is outside Latin ranges and the table.
        assert!(check("t₂′", 1).is_err());
    }
}
