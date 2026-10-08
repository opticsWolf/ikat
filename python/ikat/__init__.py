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
    tex_extra_packages,
    tex_requirements,
    used_packages,
)
from .compile import compile_pdf
from .document import BuildResult, BuildSpec, DiagramEntry, build_document
from .document import build_from_paths, list_skeletons, list_templates, read_skeleton, read_template, template_path
from .document import spec_from_dict, template_preset
from .document import template_preset
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

__version__ = "0.2.1"

__all__ = [
    "BuildResult",
    "BuildSpec",
    "DiagramEntry",
    "Element",
    "barchart_to_tikz",
    "bib_keys",
    "bib_safe",
    "build_document",
    "build_from_paths",
    "list_skeletons",
    "list_templates",
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
    "read_skeleton",
    "read_template",
    "resolve_span",
    "spec_from_dict",
    "template_path",
    "template_preset",
    "tex_extra_packages",
    "tex_package_needs",
    "tex_requirements",
    "used_packages",
    "weave_fragment",
]
