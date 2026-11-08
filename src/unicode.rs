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

/// ASCII, transliterated, or inputenc pass-through Latin.
pub fn is_allowed(c: char) -> bool {
    c.is_ascii() || ALLOWED.contains(&c) || is_passthrough(c)
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

/// Check a single text value (paragraph, heading, caption, cell,
/// display-math span, author name, plot label). `line` is the
/// 1-based manuscript line; the echo is the text itself (which
/// always contains the offending character).
pub fn check(text: &str, line: usize) -> Result<(), crate::error::Error> {
    if let Some((c, col)) = find_bad(text) {
        return Err(crate::error::Error::new(msg_for(c), line, text.to_string()).with_col(Some(col)));
    }
    Ok(())
}

/// Check spec/CLI data with no manuscript line (author names,
/// affiliations, thanks, plot labels): line-less `Error::data` so
/// Python renders `path: message` + echo.
pub fn check_data(text: &str) -> Result<(), crate::error::Error> {
    if let Some((c, _)) = find_bad(text) {
        return Err(crate::error::Error::data(msg_for(c), text.to_string()));
    }
    Ok(())
}

/// Check a multi-line source block (mermaid fence bodies): the error
/// points at the offending body line (`base` = 1-based line of the
/// first body line; the echo is that raw line with its caret).
pub fn check_block(src: &str, base: usize) -> Result<(), crate::error::Error> {
    for (idx, line_text) in src.lines().enumerate() {
        if let Some((c, col)) = find_bad(line_text) {
            return Err(crate::error::Error::new(
                msg_for(c),
                base + idx,
                line_text.to_string(),
            )
            .with_col(Some(col)));
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
    fn prime_still_fails() {
        // U+2032 is outside Latin ranges and the table.
        assert!(check("t₂′", 1).is_err());
    }
}
