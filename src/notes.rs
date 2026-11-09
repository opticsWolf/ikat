//! Compatibility notes channel (0.13.3): warn-and-render instead of
//! fail-fast. Call sites record adaptations here — list rewrites,
//! dropped reference definitions, unicode fallbacks, figures rendered
//! as text or omitted, footnote renders, automatic table captions;
//! `convert()` drains them into `BuildResult.notes` (stderr on the
//! CLI, in fence order after the layout notes).
//!
//! Thread-local because the emitters are pure functions: the CLI, the
//! Python binding, and the test suite each weave one document per
//! thread. Every public entry (`convert`, `suggest_layout`) clears
//! first, so nothing leaks across documents.

use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    /// Ordered adaptation events (pushed in document order).
    static EVENTS: RefCell<Vec<String>> = RefCell::new(Vec::new());
    /// Graceful unicode substitutions: char -> occurrences.
    static SUBS: RefCell<HashMap<char, usize>> = RefCell::new(HashMap::new());
    /// Collected footnote definitions: label -> raw definition text.
    static FNS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    /// Images: (rendered as alt-text, omitted for empty alt).
    static IMGS: RefCell<(usize, usize)> = RefCell::new((0, 0));
    /// `<sub>` / `<sup>` rewrites: (sub, sup).
    static TAGS: RefCell<(usize, usize)> = RefCell::new((0, 0));
    /// `\footnote` renders.
    static FNOTES: RefCell<usize> = RefCell::new(0);
}

/// Record one adaptation event (already-worded note fragment).
pub fn push(s: String) {
    EVENTS.with(|e| e.borrow_mut().push(s));
}

/// Record one graceful unicode substitution (aggregated at drain).
pub fn record_sub(c: char) {
    SUBS.with(|m| *m.borrow_mut().entry(c).or_insert(0) += 1);
}

/// Tally an image: alt-text render vs empty-alt omission.
pub fn record_image(empty_alt: bool) {
    IMGS.with(|i| {
        let mut b = i.borrow_mut();
        if empty_alt {
            b.1 += 1;
        } else {
            b.0 += 1;
        }
    });
}

/// Tally a `<sub>` (`true`) or `<sup>` (`false`) rewrite.
pub fn record_tag(is_sub: bool) {
    TAGS.with(|t| {
        let mut b = t.borrow_mut();
        if is_sub {
            b.0 += 1;
        } else {
            b.1 += 1;
        }
    });
}

/// Tally a footnote rendered as `\footnote`.
pub fn record_footnote() {
    FNOTES.with(|f| *f.borrow_mut() += 1);
}

/// Store a footnote definition (prepass) for later renders.
pub fn set_fn(label: String, def: String) {
    FNS.with(|m| m.borrow_mut().insert(label, def));
}

/// Fetch a footnote definition (clone: labels may repeat).
pub fn read_fn(label: &str) -> Option<String> {
    FNS.with(|m| m.borrow().get(label).cloned())
}

/// Reset all tallies (every public entry starts here).
pub fn clear() {
    EVENTS.with(|e| e.borrow_mut().clear());
    SUBS.with(|m| m.borrow_mut().clear());
    FNS.with(|m| m.borrow_mut().clear());
    IMGS.with(|i| *i.borrow_mut() = (0, 0));
    TAGS.with(|t| *t.borrow_mut() = (0, 0));
    FNOTES.with(|f| *f.borrow_mut() = 0);
}

/// Aggregate tallies into human notes, then append events in order.
/// Unused footnote definitions report (likely a skipped section).
pub fn drain_into(out: &mut Vec<String>) {
    SUBS.with(|m| {
        let m = m.borrow();
        if !m.is_empty() {
            let mut v: Vec<(&char, &usize)> = m.iter().collect();
            v.sort_by_key(|(c, _)| **c);
            let parts: Vec<String> = v
                .iter()
                .map(|(c, n)| {
                    let rep = crate::unicode::graceful_rep(**c).unwrap_or("?");
                    format!("{c}->{rep} ({n}x)")
                })
                .collect();
            out.push(format!("unicode fallbacks: {}", parts.join(", ")));
        }
    });
    IMGS.with(|i| {
        let (alt, omit) = *i.borrow();
        if alt + omit > 0 {
            out.push(format!(
                "figures: {alt} rendered as alt-text, {omit} omitted (remote graphics are not embedded; register site graphics to include them)"
            ));
        }
    });
    TAGS.with(|t| {
        let (sub, sup) = *t.borrow();
        if sub + sup > 0 {
            out.push(format!(
                "html: {sub} <sub> rendered as $_{{}}$ chemistry, {sup} <sup> rendered as $^{{}}$"
            ));
        }
    });
    FNOTES.with(|f| {
        let n = *f.borrow();
        if n > 0 {
            out.push(format!("footnotes: {n} rendered as \\footnote"));
        }
    });
    FNS.with(|m| {
        let m = m.borrow();
        if !m.is_empty() {
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort();
            let list: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
            out.push(format!(
                "footnote definitions never referenced: {}",
                list.join(", ")
            ));
        }
    });
    EVENTS.with(|e| out.extend(e.borrow().iter().cloned()));
}
