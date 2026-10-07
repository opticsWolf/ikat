//! Tectonic engine prototype (M-tectonic): `cargo feature tectonic`,
//! OFF by default so wheels stay lean and TinyTeX/pdfTeX stays the
//! default (arXiv-closest). Tectonic embeds XeTeX + a cached bundle:
//! no TeX install needed, at the cost of snapshot drift from arXiv's
//! TeX Live and first-run bundle downloads.
//!
//! XeTeX is UTF-8 native: feeds MUST drop `\usepackage[utf8]{inputenc}`
//! (Python `tex_for_tectonic` does this). Everything else is standard
//! LaTeX; bundle gaps (e.g. publisher classes) surface as engine
//! errors naming the missing file — prototype-grade diagnostics.

#[cfg(feature = "tectonic")]
pub fn compile_tectonic(tex: &str) -> Result<Vec<u8>, String> {
    tectonic::latex_to_pdf(tex).map_err(|e| format!("tectonic failed: {e}"))
}

#[cfg(all(test, feature = "tectonic"))]
mod tests {
    use super::*;

    /// Minimal article with a table + TikZ: proves the embedded
    /// engine runs and the bundle covers our core packages.
    /// NOTE: first run downloads the bundle (network + cache dir).
    #[test]
    fn prototype_compiles_mini() {
        let tex = r"\documentclass{article}
\usepackage{lmodern}
\usepackage{amsmath,amssymb}
\usepackage{tabularx}
\usepackage{graphicx}
\usepackage{tikz}
\usetikzlibrary{shapes.geometric,arrows.meta,positioning}
\begin{document}
\title{T}\author{A}\maketitle
Hello.
\begin{tikzpicture}\node[draw](a){x};\end{tikzpicture}
\begin{tabularx}{\columnwidth}{Xl}A&B\\1&2\\\end{tabularx}
\end{document}
";
        let pdf = compile_tectonic(tex).expect("tectonic mini compile");
        assert!(pdf.starts_with(b"%PDF-"));
    }
}
