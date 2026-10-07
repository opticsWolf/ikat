//! ikat core: Rust compute for the md→tex/pdf pipeline, exposed to
//! Python through PyO3. Thin wrappers live in `python/ikat/`; all
//! parsing, layout, and code generation lives here.

use pyo3::prelude::*;

mod config;
mod doc;
mod esc;
#[cfg(test)]
mod md_spike;
mod mermaid;
mod plot;
mod table;
#[cfg(feature = "tectonic")]
mod tectonic;
mod texenv;

/// Convert a mermaid flowchart block to a standalone `tikzpicture`.
///
/// Raises `ValueError` on empty input, unknown directions, or
/// unparsable edges.
#[pyfunction]
fn flowchart_to_tikz(src: &str) -> PyResult<String> {
    mermaid::flowchart_to_tikz(src)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
}

/// Grouped bar chart with min/max whiskers → `tikzpicture` (pgfplots).
///
/// `values/mins/maxs` are per-series lists over groups:
/// `values[s][i]` is series `s` at group `i`. `legend` is a position
/// keyword (`auto` default: first collision-free inside corner, else
/// below; `below`, `top-left`, `top-right`, `bottom-left`,
/// `bottom-right`, `outside-right`).
#[pyfunction]
#[allow(clippy::too_many_arguments)]
#[pyo3(signature = (title, ylabel, log_y, group_labels, series_names, values, mins, maxs, refline, legend="auto"))]
fn barchart_to_tikz(
    title: &str,
    ylabel: &str,
    log_y: bool,
    group_labels: Vec<String>,
    series_names: Vec<String>,
    values: Vec<Vec<f64>>,
    mins: Vec<Vec<f64>>,
    maxs: Vec<Vec<f64>>,
    refline: Option<(f64, f64, f64, String)>,
    legend: &str,
) -> PyResult<String> {
    let legend = plot::LegendPos::parse(legend)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
    plot::barchart(
        title,
        ylabel,
        log_y,
        &group_labels,
        &series_names,
        &values,
        &mins,
        &maxs,
        refline,
        legend,
    )
    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
}

/// Line plot with symmetric error bars → `tikzpicture` (pgfplots).
/// `legend` is a position keyword (see `barchart_to_tikz`).
#[pyfunction]
#[pyo3(signature = (title, xlabel, ylabel, xs, names, yss, errs, legend="auto"))]
fn lineplot_to_tikz(
    title: &str,
    xlabel: &str,
    ylabel: &str,
    xs: Vec<f64>,
    names: Vec<String>,
    yss: Vec<Vec<f64>>,
    errs: Vec<Vec<f64>>,
    legend: &str,
) -> PyResult<String> {
    let legend = plot::LegendPos::parse(legend)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
    if names.len() != yss.len() || names.len() != errs.len() {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "lineplot: names/yss/errs length mismatch",
        ));
    }
    let series: Vec<(&str, Vec<f64>, Vec<f64>)> = names
        .iter()
        .zip(yss.into_iter())
        .zip(errs.into_iter())
        .map(|((n, y), e)| (n.as_str(), y, e))
        .collect();
    plot::lineplot(title, xlabel, ylabel, &xs, &series, legend)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
}

/// Parse an `ikat.toml` document config. Returns the resolved
/// per-kind spans plus document settings as a dict.
#[pyfunction]
fn parse_config(src: &str) -> PyResult<std::collections::HashMap<String, String>> {
    let cfg = config::Config::from_toml(src)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
    let mut m = std::collections::HashMap::new();
    m.insert("class".to_string(), cfg.document.class.clone());
    m.insert(
        "class_options".to_string(),
        cfg.document.class_options.join(","),
    );
    m.insert("columns".to_string(), cfg.document.columns.to_string());
    m.insert("margins".to_string(), cfg.document.margins.clone());
    m.insert("template_preamble_file".to_string(), cfg.template.preamble_file.clone());
    m.insert("template_preamble_append".to_string(), cfg.template.preamble_append.join("\n"));
    m.insert(
        "template_abstract_before_maketitle".to_string(),
        cfg.template.abstract_before_maketitle.to_string(),
    );
    m.insert("float_pos_default".to_string(), format!("{:?}", cfg.floats.pos_default).to_lowercase());
    m.insert("float_topfraction".to_string(), cfg.floats.topfraction.to_string());
    m.insert("float_bottomfraction".to_string(), cfg.floats.bottomfraction.to_string());
    m.insert("float_textfraction".to_string(), cfg.floats.textfraction.to_string());
    m.insert("float_floatpagefraction".to_string(), cfg.floats.floatpagefraction.to_string());
    m.insert("float_topnumber".to_string(), cfg.floats.topnumber.to_string());
    m.insert("float_bottomnumber".to_string(), cfg.floats.bottomnumber.to_string());
    m.insert("float_barrier_sections".to_string(), cfg.floats.barrier_sections.to_string());
    m.insert("template_skeleton".to_string(), cfg.template.skeleton.clone());
    for kind in ["diagram", "plot", "table", "picture", "default"] {
        let span = cfg.spans.for_kind(kind);
        m.insert(format!("span_{kind}"), span.latex_env().to_string());
        m.insert(format!("width_{kind}"), span.latex_width().to_string());
    }
    Ok(m)
}

/// Full document build: Markdown + ikat.toml + spec JSON → result JSON.
///
/// `spec_json` carries diagrams/plots/table captions, thanks, author,
/// bibliography name, graphicspaths, and the validated `.bib` keyset.
/// Returns `{"title","body","tex","n_diagrams","n_tables"}`.
#[pyfunction]
fn build_document(md_text: &str, toml_src: &str, spec_json: &str) -> PyResult<String> {
    let spec: doc::BuildSpec = serde_json::from_str(spec_json)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("spec: {e}")))?;
    let r = doc::build_document(md_text, toml_src, &spec)
        .map_err(pyo3::exceptions::PyValueError::new_err)?;
    serde_json::to_string(&r)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

/// BibTeX-safe derivative of a `.bib` source (brace bare authors,
/// escape specials outside author/url/doi fields).
#[pyfunction]
fn bib_safe(bib_src: &str) -> String {
    table::bib_safe(bib_src)
}

/// Every `@type{key,` entry key in a `.bib` source.
#[pyfunction]
fn bib_keys(bib_src: &str) -> Vec<String> {
    let mut v: Vec<String> = table::bib_keys_of(bib_src).into_iter().collect();
    v.sort();
    v
}

/// LaTeX packages a woven body needs: TikZ for inline diagrams,
/// pgfplots for data plots. Precompiled PDFs need neither.
#[pyfunction]
fn tex_requirements(has_tikz: bool, has_plots: bool) -> Vec<String> {
    doc::tex_requirements(has_tikz, has_plots)
}

/// Scan any `.tex` preamble for `\usepackage` names (comments
/// stripped, TikZ/pgfplots use inferred). Pure scan; probing the
/// disk stays in `texenv.py`.
#[pyfunction]
fn used_packages(tex: &str) -> Vec<String> {
    texenv::used_packages(tex)
}

/// The `\documentclass` name in a `.tex` source, if any.
#[pyfunction]
fn document_class(tex: &str) -> Option<String> {
    texenv::document_class(tex)
}

/// Map LaTeX names + optional class to `(probe file, tlmgr package)`
/// pairs, deduplicated in first-use order.
#[pyfunction]
fn package_needs(names: Vec<String>, class: Option<String>) -> Vec<(String, String)> {
    texenv::package_needs(&names, class.as_deref())
}

/// Heuristic package scan for arbitrary `.tex`: `[H]` and
/// `\FloatBarrier` map to their packages.
#[pyfunction]
fn tex_extra_packages(tex: &str) -> Vec<String> {
    doc::tex_extra_packages(tex)
}

/// Tectonic engine (feature `tectonic` only): full `.tex` in,
/// PDF bytes out. Absent without the feature — see `compile.py`.
#[cfg(feature = "tectonic")]
#[pyfunction]
fn compile_tectonic_pdf(tex: &str) -> PyResult<Vec<u8>> {
    crate::tectonic::compile_tectonic(tex).map_err(pyo3::exceptions::PyRuntimeError::new_err)
}

/// ikat core (Rust): mermaid→TikZ, data→pgfplots, document config.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(flowchart_to_tikz, m)?)?;
    m.add_function(wrap_pyfunction!(barchart_to_tikz, m)?)?;
    m.add_function(wrap_pyfunction!(lineplot_to_tikz, m)?)?;
    m.add_function(wrap_pyfunction!(parse_config, m)?)?;
    m.add_function(wrap_pyfunction!(build_document, m)?)?;
    m.add_function(wrap_pyfunction!(bib_safe, m)?)?;
    m.add_function(wrap_pyfunction!(bib_keys, m)?)?;
    m.add_function(wrap_pyfunction!(tex_requirements, m)?)?;
    m.add_function(wrap_pyfunction!(tex_extra_packages, m)?)?;
    m.add_function(wrap_pyfunction!(used_packages, m)?)?;
    m.add_function(wrap_pyfunction!(document_class, m)?)?;
    m.add_function(wrap_pyfunction!(package_needs, m)?)?;
    #[cfg(feature = "tectonic")]
    m.add_function(wrap_pyfunction!(compile_tectonic_pdf, m)?)?;
    Ok(())
}
