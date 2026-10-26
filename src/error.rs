//! Line-numbered build errors (Phase F / M4.1). Every fail-fast
//! site returns `Result<_, Error>` instead of `Result<_, String>`;
//! the message format is upgraded but the Python exception type is
//! unchanged (`ValueError` carrying the Display rendering).
//!
//! `line` is 1-based in the relevant source (manuscript for
//! scanner errors, head file for template errors, snippet for
//! direct API calls, fence-body-relative resolved by the caller
//! for diagram statements). `line == 0` means "no manuscript
//! line": data errors (ragged JSON series, TOML value errors)
//! where the echo — the offending values — is the context. The
//! ` --> line N` part is omitted then, never rendered as `0`.
//! `col` is `None` until F.2 assigns carets; absence of a caret is
//! never a failure, only missing precision.

use std::fmt;

/// A build error with source context.
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    /// Human message (unchanged vocabulary from the String era).
    pub msg: String,
    /// 1-based source line, or 0 when no manuscript line applies.
    pub line: usize,
    /// 1-based column of the offending token (F.2 fills this in).
    pub col: Option<usize>,
    /// Offending source text (statement / line / values).
    pub echo: String,
}

impl Error {
    pub fn new(msg: impl Into<String>, line: usize, echo: impl Into<String>) -> Self {
        Self { msg: msg.into(), line, col: None, echo: echo.into() }
    }

    /// Attach a 1-based caret column (F.2 fills these where the
    /// grammar knows the token span).
    pub fn with_col(mut self, col: Option<usize>) -> Self {
        self.col = col;
        self
    }

    /// Data error: no manuscript line (JSON/TOML values).
    pub fn data(msg: impl Into<String>, echo: impl Into<String>) -> Self {
        Self::new(msg, 0, echo)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Long echoes (data dumps) truncate — the head of the
        // offending text is what locates it.
        let echo: String = self.echo.chars().take(240).collect();
        let cut = self.echo.chars().count() > 240;
        if self.line == 0 {
            if echo.is_empty() {
                write!(f, "{}", self.msg)
            } else if cut {
                write!(f, "{}\n{}…", self.msg, echo)
            } else {
                write!(f, "{}\n{}", self.msg, echo)
            }
        } else {
            writeln!(f, "{}", self.msg)?;
            writeln!(f, " --> line {}", self.line)?;
            if let Some(c) = self.col {
                writeln!(f, "{}^", " ".repeat(c.saturating_sub(1)))?;
            }
            if cut {
                write!(f, "{}…", echo)
            } else {
                write!(f, "{}", echo)
            }
        }
    }
}

impl std::error::Error for Error {}

/// 1-based character column of the byte index inside `line`
/// (caret arithmetic is in chars — echoes may hold multibyte
/// text, and byte columns would misplace the caret).
pub fn col_of(line: &str, byte_idx: usize) -> usize {
    line[..byte_idx.min(line.len())].chars().count() + 1
}

/// `format!` a message into an `Error` at one call site.
#[macro_export]
macro_rules! err {
    ($line:expr, $echo:expr, $($t:tt)*) => {
        $crate::error::Error::new(format!($($t)*), $line, $echo)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_shows_msg_line_echo() {
        let e = Error::new("bad statement", 12, "loop x");
        let s = e.to_string();
        assert!(s.contains("bad statement"), "{s}");
        assert!(s.contains(" --> line 12"), "{s}");
        assert!(s.contains("loop x"), "{s}");
    }

    #[test]
    fn data_error_omits_line() {
        let e = Error::data("ragged values", "[1, 2]");
        let s = e.to_string();
        assert!(!s.contains("line 0"), "{s}");
        assert!(s.contains("ragged values") && s.contains("[1, 2]"), "{s}");
    }

    #[test]
    fn long_echo_truncates() {
        let e = Error::new("m", 1, "x".repeat(300));
        assert!(e.to_string().ends_with('…'));
    }
}
