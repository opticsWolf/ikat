//! Document configuration: columns, margins, per-element spanning.
//!
//! An `ikat.toml` sits next to the manuscript:
//!
//! ```toml
//! [document]
//! class = "IEEEtran"
//! class_options = ["conference"]
//! columns = 2
//! margins = "0.75in"
//!
//! [spans]
//! diagram = "wide"    # figure* — TikZ flowcharts breathe at text width
//! plot    = "column"  # figure  — data graphics stay in-column
//! table   = "column"
//! default = "column"
//! ```
//!
//! A single element overrides its kind default from Markdown with a
//! trailing attribute comment on the fence info line, e.g.
//! ```` ```mermaid {span=column} ````.

use serde::{Deserialize, Serialize};

/// One/two-column placement of a floated element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Span {
    /// Single column: `figure` / one-column table.
    Column,
    /// Full text width: `figure*`.
    Wide,
}

impl Span {
    pub fn latex_env(&self) -> &'static str {
        match self {
            Span::Column => "figure",
            Span::Wide => "figure*",
        }
    }

    pub fn latex_width(&self) -> &'static str {
        match self {
            Span::Column => "\\columnwidth",
            Span::Wide => "\\textwidth",
        }
    }
}

/// `[document]` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    #[serde(default = "default_class")]
    pub class: String,
    #[serde(default)]
    pub class_options: Vec<String>,
    #[serde(default = "default_columns")]
    pub columns: u8,
    #[serde(default = "default_margins")]
    pub margins: String,
}

fn default_class() -> String {
    "IEEEtran".to_string()
}
fn default_columns() -> u8 {
    2
}
fn default_margins() -> String {
    "0.75in".to_string()
}

impl Default for Document {
    fn default() -> Self {
        Self {
            class: default_class(),
            class_options: vec!["conference".to_string()],
            columns: default_columns(),
            margins: default_margins(),
        }
    }
}

/// `[spans]` table: default float span per element kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spans {
    #[serde(default = "span_wide")]
    pub diagram: Span,
    #[serde(default = "span_column")]
    pub plot: Span,
    #[serde(default = "span_column")]
    pub table: Span,
    #[serde(default = "span_column")]
    pub default: Span,
}

fn span_wide() -> Span {
    Span::Wide
}
fn span_column() -> Span {
    Span::Column
}

impl Default for Spans {
    fn default() -> Self {
        Self {
            diagram: Span::Wide,
            plot: Span::Column,
            table: Span::Column,
            default: Span::Column,
        }
    }
}

impl Spans {
    pub fn for_kind(&self, kind: &str) -> Span {
        match kind {
            "diagram" => self.diagram,
            "plot" => self.plot,
            "table" => self.table,
            _ => self.default,
        }
    }
}

/// Whole-file configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub document: Document,
    #[serde(default)]
    pub spans: Spans,
}

impl Config {
    pub fn from_toml(src: &str) -> Result<Self, String> {
        toml::from_str(src).map_err(|e| format!("ikat.toml: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_favour_readable_diagrams() {
        let cfg = Config::from_toml("").unwrap();
        assert_eq!(cfg.document.class, "IEEEtran");
        assert_eq!(cfg.document.columns, 2);
        assert_eq!(cfg.spans.diagram, Span::Wide);
        assert_eq!(cfg.spans.plot, Span::Column);
    }

    #[test]
    fn spans_parse_and_envs_map() {
        let cfg = Config::from_toml(
            "[spans]\ndiagram = \"column\"\nplot = \"wide\"\n",
        )
        .unwrap();
        assert_eq!(cfg.spans.for_kind("diagram").latex_env(), "figure");
        assert_eq!(cfg.spans.for_kind("plot").latex_env(), "figure*");
        assert_eq!(cfg.spans.for_kind("table").latex_env(), "figure");
        assert_eq!(cfg.spans.for_kind("diagram").latex_width(), "\\columnwidth");
    }

    #[test]
    fn bad_toml_is_an_error_string() {
        assert!(Config::from_toml("[spans\ndiagram = 42\n").is_err());
    }
}
