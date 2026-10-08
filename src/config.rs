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

/// Float placement for one element: how close to its manuscript
/// position LaTeX may keep it. `figure*` wide floats only support
/// Top/Bottom/Page/Barrier (LaTeX bans `[h]`/`[H]` on `*` floats).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Pos {
    /// `[t]`: top of column/page (default, journals-safe).
    Top,
    /// `[b]`: bottom (wide needs `dblfloatfix`, auto-added).
    Bottom,
    /// `[!tb]`: same column, top preferred, limits overridden.
    Both,
    /// `[p]`: dedicated float page.
    Page,
    /// `[h]`: here if it fits (polite request).
    Here,
    /// `[H]`: exactly here (needs `float` pkg, auto-added).
    Force,
    /// `\FloatBarrier` + `[t]`: never drift past this element.
    Barrier,
}

impl Pos {
    pub fn parse(s: &str) -> Result<Pos, String> {
        match s {
            "top" => Ok(Pos::Top),
            "bottom" => Ok(Pos::Bottom),
            "both" => Ok(Pos::Both),
            "page" => Ok(Pos::Page),
            "here" => Ok(Pos::Here),
            "force" => Ok(Pos::Force),
            "barrier" => Ok(Pos::Barrier),
            _ => Err(format!("pos must be top|bottom|both|page|here|force|barrier, got {s:?}")),
        }
    }

    /// LaTeX placement spec, or empty for Barrier (prefix instead).
    pub fn latex_spec(self) -> &'static str {
        match self {
            Pos::Top => "[t]",
            Pos::Bottom => "[b]",
            Pos::Both => "[!tb]",
            Pos::Page => "[p]",
            Pos::Here => "[h]",
            Pos::Force => "[H]",
            Pos::Barrier => "[t]",
        }
    }
}

/// Caption placement: figures conventionally below, tables above.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptionPos {
    Top,
    Bottom,
}

impl CaptionPos {
    pub fn parse(s: &str) -> Result<CaptionPos, String> {
        match s {
            "top" => Ok(CaptionPos::Top),
            "bottom" => Ok(CaptionPos::Bottom),
            _ => Err(format!("captionpos must be top|bottom, got {s:?}")),
        }
    }
}

/// Package needs one float element can trigger, OR-combined across
/// the document. The preamble (or template validation) consumes this.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FloatNeeds {
    pub float_h: bool,
    pub barrier: bool,
    pub dblfloat: bool,
    pub graphicx: bool,
}

impl FloatNeeds {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn add(&mut self, o: FloatNeeds) {
        self.float_h |= o.float_h;
        self.barrier |= o.barrier;
        self.dblfloat |= o.dblfloat;
        self.graphicx |= o.graphicx;
    }

    /// Preamble lines, in load order.
    pub fn packages(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.float_h {
            out.push("\\usepackage{float}".to_string());
        }
        if self.barrier {
            out.push("\\usepackage{placeins}".to_string());
        }
        if self.dblfloat {
            out.push("\\usepackage{dblfloatfix}".to_string());
        }
        out
    }

    /// LaTeX names template validation requires in the head.
    pub fn required_names(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        if self.float_h {
            out.push("float");
        }
        if self.barrier {
            out.push("placeins");
        }
        if self.dblfloat {
            out.push("dblfloatfix");
        }
        if self.graphicx {
            out.push("graphicx");
        }
        out
    }
}

/// `[spans]` table: default float span per element kind.
/// `picture` (precompiled graphics) falls back to `diagram` when
/// unset, so existing documents keep their wide figures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spans {
    #[serde(default = "span_wide")]
    pub diagram: Span,
    #[serde(default = "span_column")]
    pub plot: Span,
    #[serde(default = "span_column")]
    pub table: Span,
    #[serde(default)]
    pub picture: Option<Span>,
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
            picture: None,
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
            "picture" => self.picture.unwrap_or(self.diagram),
            _ => self.default,
        }
    }
}

/// `[template]` table: user-supplied preamble pieces.
///
/// ```toml
/// [template]
/// preamble_file = "journal-head.tex"  # replaces the generated head
/// preamble_append = ["\\usepackage{natbib}"]  # extra lines before \\begin{document}
/// ```
///
/// `preamble_file` is resolved by Python relative to the toml file;
/// Rust only ever sees file *content* (via `BuildSpec`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Template {
    #[serde(default)]
    pub preamble_file: String,
    #[serde(default)]
    pub preamble_append: Vec<String>,
    /// Top-matter classes (ACM, Elsevier, APS) set the abstract in
    /// `\maketitle`: hoist the body's abstract env before it.
    #[serde(default)]
    pub abstract_before_maketitle: bool,
    /// Whole-document skeleton file (level 3 escape hatch);
    /// mutually exclusive with `preamble_file` (checked at build).
    #[serde(default)]
    pub skeleton: String,
}

/// `[typography]` table: keep-with-next guards against stranded
/// headings. A heading at a column/page bottom with its paragraph
/// starting on the next one is a layout bug ikat makes structurally
/// impossible — LaTeX's `\@afterheading` only partly prevents it
/// (floats and two-column balancing defeat it), so the guards are
/// explicit. Default ON: this is a bugfix, not a style choice.
///
/// ```toml
/// [typography]
/// keep_with_next = true  # penalties + needspace before sections
/// min_lines = 2         # body lines required after a heading
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    #[serde(default = "keep_true")]
    pub keep_with_next: bool,
    #[serde(default = "min_two")]
    pub min_lines: i64,
}

fn keep_true() -> bool {
    true
}
fn min_two() -> i64 {
    2
}

impl Default for Typography {
    fn default() -> Self {
        Self { keep_with_next: true, min_lines: 2 }
    }
}

impl Typography {
    /// Guard lines for knobs that differ from LaTeX defaults.
    /// LaTeX ships club/widow penalties at 150 and no heading
    /// guards, so a default `[typography]` EMITS lines (unlike
    /// `[floats]`, whose defaults match LaTeX). Turning the guards
    /// off emits nothing — free breaks are already the default.
    pub fn setup_lines(&self) -> Vec<String> {
        if !self.keep_with_next {
            return Vec::new();
        }
        vec![
            "\\clubpenalty=10000".to_string(),
            "\\widowpenalty=10000".to_string(),
        ]
    }

    /// `\needspace` reservation before a heading: the heading plus
    /// `min_lines` of body must fit, else the break comes first.
    pub fn needspace_line(&self) -> String {
        format!("\\needspace{{{}\\baselineskip}}", self.min_lines + 1)
    }
}

/// `[floats]` table: document-wide float tuning. Fraction/counter
/// lines are emitted into the preamble ONLY when they differ from
/// the LaTeX defaults, so default documents weave byte-identically.
///
/// ```toml
/// [floats]
/// pos_default = "top"
/// topfraction = 0.9
/// bottomfraction = 0.8
/// textfraction = 0.1
/// floatpagefraction = 0.6
/// topnumber = 2
/// bottomnumber = 2
/// barrier_sections = false
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Floats {
    #[serde(default = "pos_top")]
    pub pos_default: Pos,
    #[serde(default = "frac_top")]
    pub topfraction: f64,
    #[serde(default = "frac_bottom")]
    pub bottomfraction: f64,
    #[serde(default = "frac_text")]
    pub textfraction: f64,
    #[serde(default = "frac_floatpage")]
    pub floatpagefraction: f64,
    #[serde(default = "num_top")]
    pub topnumber: i64,
    #[serde(default = "num_bottom")]
    pub bottomnumber: i64,
    #[serde(default)]
    pub barrier_sections: bool,
}

fn pos_top() -> Pos {
    Pos::Top
}
fn frac_top() -> f64 {
    0.7
}
fn frac_bottom() -> f64 {
    0.3
}
fn frac_text() -> f64 {
    0.2
}
fn frac_floatpage() -> f64 {
    0.5
}
fn num_top() -> i64 {
    2
}
fn num_bottom() -> i64 {
    1
}

impl Default for Floats {
    fn default() -> Self {
        Self {
            pos_default: Pos::Top,
            topfraction: 0.7,
            bottomfraction: 0.3,
            textfraction: 0.2,
            floatpagefraction: 0.5,
            topnumber: 2,
            bottomnumber: 1,
            barrier_sections: false,
        }
    }
}

impl Floats {
    /// Preamble lines for knobs that differ from LaTeX defaults.
    /// Empty for a default `[floats]` (golden parity).
    pub fn setup_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        let mut frac = |name: &str, val: f64, dflt: f64| {
            if (val - dflt).abs() > 1e-9 {
                lines.push(format!("\\renewcommand{{\\{name}}}{{{val}}}"));
            }
        };
        frac("topfraction", self.topfraction, 0.7);
        frac("bottomfraction", self.bottomfraction, 0.3);
        frac("textfraction", self.textfraction, 0.2);
        frac("floatpagefraction", self.floatpagefraction, 0.5);
        if self.topnumber != 2 {
            lines.push(format!("\\setcounter{{topnumber}}{{{}}}", self.topnumber));
        }
        if self.bottomnumber != 1 {
            lines.push(format!("\\setcounter{{bottomnumber}}{{{}}}", self.bottomnumber));
        }
        lines
    }
}

/// Whole-file configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub document: Document,
    #[serde(default)]
    pub spans: Spans,
    #[serde(default)]
    pub floats: Floats,
    #[serde(default)]
    pub typography: Typography,
    #[serde(default)]
    pub template: Template,
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
    fn typography_defaults_and_guards() {
        let t = Config::from_toml("").unwrap().typography;
        assert!(t.keep_with_next);
        assert_eq!(t.min_lines, 2);
        let lines = t.setup_lines();
        assert!(lines.contains(&"\\clubpenalty=10000".to_string()));
        assert!(lines.contains(&"\\widowpenalty=10000".to_string()));
        // Off emits nothing: free breaks are already the default.
        let off = Config::from_toml("[typography]\nkeep_with_next = false\n").unwrap().typography;
        assert!(off.setup_lines().is_empty());
    }

    #[test]
    fn needspace_scales_with_min_lines() {
        let t = Config::from_toml("").unwrap().typography;
        assert_eq!(t.needspace_line(), "\\needspace{3\\baselineskip}");
        let t4 = Config::from_toml("[typography]\nmin_lines = 4\n").unwrap().typography;
        assert_eq!(t4.needspace_line(), "\\needspace{5\\baselineskip}");
    }

    #[test]
    fn template_table_parses() {
        let cfg = Config::from_toml(
            "[template]\npreamble_file = \"head.tex\"\npreamble_append = [\"\\\\usepackage{natbib}\"]\n",
        )
        .unwrap();
        assert_eq!(cfg.template.preamble_file, "head.tex");
        assert_eq!(cfg.template.preamble_append, vec!["\\usepackage{natbib}"]);
        assert!(Config::from_toml("").unwrap().template.preamble_append.is_empty());
    }

    #[test]
    fn template_flag_parses() {
        let cfg = Config::from_toml("[template]\nabstract_before_maketitle = true\n").unwrap();
        assert!(cfg.template.abstract_before_maketitle);
        assert!(!Config::from_toml("").unwrap().template.abstract_before_maketitle);
    }

    #[test]
    fn picture_falls_back_to_diagram() {
        let cfg = Config::from_toml("").unwrap();
        assert_eq!(cfg.spans.for_kind("picture"), Span::Wide);
        let cfg = Config::from_toml("[spans]\npicture = \"column\"\n").unwrap();
        assert_eq!(cfg.spans.for_kind("picture"), Span::Column);
        assert_eq!(cfg.spans.for_kind("diagram"), Span::Wide);
    }

    #[test]
    fn pos_parses_strictly() {
        assert_eq!(Pos::parse("both"), Ok(Pos::Both));
        assert_eq!(Pos::Both.latex_spec(), "[!tb]");
        assert_eq!(Pos::Barrier.latex_spec(), "[t]");
        assert!(Pos::parse("center").is_err());
    }

    #[test]
    fn float_setup_lines_only_nondefaults() {
        assert!(Config::from_toml("").unwrap().floats.setup_lines().is_empty());
        let cfg = Config::from_toml("[floats]\ntopfraction = 0.9\nbottomnumber = 2\n").unwrap();
        let lines = cfg.floats.setup_lines();
        assert!(lines.contains(&"\\renewcommand{\\topfraction}{0.9}".to_string()));
        assert!(lines.contains(&"\\setcounter{bottomnumber}{2}".to_string()));
        assert_eq!(lines.len(), 2);
        assert!(cfg.floats.barrier_sections == false);
    }

    #[test]
    fn bad_toml_is_an_error_string() {
        assert!(Config::from_toml("[spans\ndiagram = 42\n").is_err());
    }
}
