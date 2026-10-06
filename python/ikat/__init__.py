"""ikat: weave Markdown into camera-ready LaTeX.

Thin Python wrappers over the Rust core (`ikat._core`, built with
maturin). All parsing, layout, and code generation lives in Rust;
this package only models documents and drives tools.
"""

from ._core import (
    barchart_to_tikz,
    bib_keys,
    bib_safe,
    flowchart_to_tikz,
    lineplot_to_tikz,
    parse_config,
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
    "extract_fences",
    "figure_env",
    "flowchart_to_tikz",
    "lineplot_to_tikz",
    "parse_config",
    "resolve_span",
    "weave_fragment",
]
