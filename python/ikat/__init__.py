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
    plots_needed,
    preset_to_tikz,
    tex_extra_packages,
    tex_requirements,
    tikz_needed,
    used_packages,
)
from .compile import compile_pdf
from .document import BuildResult, BuildSpec, DiagramEntry, build_document
from .document import build_from_paths, list_skeletons, list_templates, read_skeleton, read_template, template_path
from .document import spec_from_dict, template_preset
from .document import template_preset
from .preset import load_preset
from .standalone import compile_standalone, fence_to_tikz, wrap_standalone
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

__version__ = "0.5.1"

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
    "compile_standalone",
    "fence_to_tikz",
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
    "load_preset",
    "package_needs",
    "preset_to_tikz",
    "parse_config",
    "plots_needed",
    "read_skeleton",
    "read_template",
    "resolve_span",
    "spec_from_dict",
    "template_path",
    "template_preset",
    "tex_extra_packages",
    "tikz_needed",
    "tex_package_needs",
    "tex_requirements",
    "used_packages",
    "weave_fragment",
    "wrap_standalone",
]
