"""ikat: weave Markdown into camera-ready LaTeX.

Thin Python wrappers over the Rust core (`ikat._core`, built with
maturin). All parsing, layout, and code generation lives in Rust;
this package only models documents and drives tools.
"""

from ._core import (
    barchart_to_tikz,
    bib_keys,
    bib_safe,
    document_class,
    flowchart_to_tikz,
    lineplot_to_tikz,
    package_needs,
    parse_config,
    tex_requirements,
    used_packages,
)
from .compile import compile_pdf
from .document import BuildResult, BuildSpec, DiagramEntry, build_document
from .pipeline import (
    Element,
    config_spans,
    extract_fences,
    figure_env,
    resolve_span,
    weave_fragment,
)
from .texenv import check as check_tex_env
from .texenv import ensure as ensure_tex_packages
from .texenv import needs as tex_package_needs

__version__ = "0.1.0"

__all__ = [
    "BuildResult",
    "BuildSpec",
    "DiagramEntry",
    "Element",
    "barchart_to_tikz",
    "bib_keys",
    "bib_safe",
    "build_document",
    "compile_pdf",
    "config_spans",
    "check_tex_env",
    "document_class",
    "ensure_tex_packages",
    "extract_fences",
    "figure_env",
    "flowchart_to_tikz",
    "lineplot_to_tikz",
    "package_needs",
    "parse_config",
    "resolve_span",
    "tex_package_needs",
    "tex_requirements",
    "used_packages",
    "weave_fragment",
]
