"""The ikat agent-tool registry: one source of truth for every surface.

Fourteen operations, one table. The MCP server generates its FastMCP
tools (schemas, annotations, docstrings) from ``TOOL_REGISTRY``;
LLM function-calling consumers derive their tool definitions from the
same specs via :func:`llm_tool_definitions`; and
:func:`execute_tool` dispatches a ``(name, arguments)`` pair without
MCP installed — the whole surface is drivable from a plain dict.

This is the pattern the gossamer audit validated, at ikat's scale:
the registry replaces hand-written per-tool functions, so schema and
tool list cannot drift from the definitions (and the drift tests pin
the skills to this table).

Each :class:`ToolSpec` carries the MCP annotation flags as plain data
(``readonly``/``destructive``/``idempotent``/``openworld``); the MCP
server maps them onto ``ToolAnnotations``. Every tool is local
computation except ``ensure_tex_packages`` (tlmgr installs into the
TeX environment and downloads from CTAN mirrors).
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Callable

from ikat import __version__
from ikat import (
    barchart_to_tikz,
    check_tex_env,
    ensure_tex_packages,
    flowchart_to_tikz,
    lineplot_to_tikz,
    list_skeletons,
    list_templates as _list_templates,
    tex_extra_packages,
)
from ikat.document import template_path, template_preset

_MISSING = object()


class ToolParam:
    """One parameter of a registry tool."""

    __slots__ = ("name", "type", "default", "description", "enum")

    def __init__(self, name, type, default=_MISSING, description="", enum=None):
        self.name = name
        self.type = type  # Python annotation: str / int / bool / list[...]
        self.default = default  # _MISSING when required
        self.description = description
        self.enum = enum  # optional list of allowed values (str-typed params)

    @property
    def required(self) -> bool:
        return self.default is _MISSING

    @property
    def json_schema(self) -> dict:
        if self.type is str:
            schema = {"type": "string"}
        elif self.type is int:
            schema = {"type": "integer"}
        elif self.type is float:
            schema = {"type": "number"}
        elif self.type is bool:
            schema = {"type": "boolean"}
        else:  # list[str] / list[list[float]] / list[float]
            schema = {"type": "array"}
        if self.enum is not None:
            schema["enum"] = list(self.enum)
        if self.description:
            schema["description"] = self.description
        if not self.required:
            schema["default"] = self.default
        return schema


@dataclass(frozen=True)
class ToolSpec:
    """One registry tool: description, callable, params, MCP hints."""

    name: str
    description: str
    method: Callable
    params: tuple[ToolParam, ...]
    returns: type = str
    readonly: bool = True
    destructive: bool = False
    idempotent: bool = True
    openworld: bool = False

    def kwargs(self, arguments: dict | None = None) -> dict:
        """Registry defaults plus caller-supplied arguments.

        Mutable defaults are copied per call so one invocation can
        never contaminate the next.
        """
        merged = {}
        for p in self.params:
            if p.required:
                continue
            default = p.default
            if isinstance(default, list):
                default = list(default)
            merged[p.name] = default
        merged.update(arguments or {})
        return merged

    def llm_definition(self) -> dict:
        """Function-calling definition for LLM consumers."""
        return {
            "type": "function",
            "function": {
                "name": self.name,
                "description": self.description,
                "parameters": {
                    "type": "object",
                    "properties": {p.name: p.json_schema for p in self.params},
                    "required": [p.name for p in self.params if p.required],
                },
            },
        }


# ────────────────────────────────────────────────────────────────
# Composite implementations (the thin MCP-level wrappers over the
# plain API functions, defined here so every surface shares them).
# ────────────────────────────────────────────────────────────────

def weave_document(md_text: str, toml_text: str, spec_json: str) -> dict:
    """Weave Markdown + ikat.toml + spec JSON into LaTeX (no TeX needed)."""
    import json as _json

    from ikat import build_document, spec_from_dict

    r = build_document(md_text, toml_text, spec_from_dict(_json.loads(spec_json)))
    return {"title": r.title, "body": r.body, "tex": r.tex,
            "n_diagrams": r.n_diagrams, "n_tables": r.n_tables}


def standalone_figure(md_text: str) -> dict:
    """One mermaid/chart fence -> wrapped standalone `fig.tex` source."""
    from ikat.pipeline import extract_fences
    from ikat.standalone import fence_to_tikz, wrap_standalone

    fences = [(l, b) for l, _, b in extract_fences(md_text) if l in ("mermaid", "chart")]
    if not fences:
        raise ValueError("no mermaid/chart fence found")
    if len(fences) > 1:
        raise ValueError(f"{len(fences)} mermaid/chart fences found, need exactly 1")
    lang, body = fences[0]
    tikz = fence_to_tikz(lang, body)
    needs = [n for n, has in (("tikz", True), ("pgfplots", "\\begin{axis}" in tikz)) if has]
    return {"lang": lang, "tex": wrap_standalone(tikz), "needs": needs}


def suggest_layout(md_text: str, toml_text: str = "", spec_json: str = "{}") -> dict:
    """Measure floats, suggest span/pos/width per fence (advisory only)."""
    import json as _json

    from ikat import spec_from_dict
    from ikat.layout import report as _report
    from ikat.layout import suggest as _suggest

    suggs = _suggest(md_text, toml_text, spec_from_dict(_json.loads(spec_json or "{}")))
    return {"suggestions": suggs, "report": _report(suggs)}


def barchart_tool(title: str, ylabel: str, log_y: bool,
                  group_labels: list[str], series_names: list[str],
                  values: list[list[float]], mins: list[list[float]],
                  maxs: list[list[float]], legend: str = "auto",
                  preset_json: str = "") -> str:
    """Grouped bar chart with min/max whiskers -> tikzpicture (pgfplots)."""
    if preset_json:
        from ikat.preset import load_preset
        return load_preset(preset_json)
    return barchart_to_tikz(title, ylabel, log_y, group_labels, series_names,
                            values, mins, maxs, None, legend)


def lineplot_tool(title: str, xlabel: str, ylabel: str, xs: list[float],
                  names: list[str], yss: list[list[float]],
                  errs: list[list[float]], legend: str = "auto",
                  preset_json: str = "") -> str:
    """Line plot with symmetric error bars -> tikzpicture (pgfplots)."""
    if preset_json:
        from ikat.preset import load_preset
        return load_preset(preset_json)
    return lineplot_to_tikz(title, xlabel, ylabel, xs, names, yss, errs, legend)


def float_packages(tex_source: str) -> list[str]:
    """Extra packages for hand-written floats."""
    return tex_extra_packages(tex_source)


def list_templates() -> list[str]:
    """Shipped heads (arxiv, acm-sigconf, ...) plus skeletons."""
    return _list_templates() + _list_skeletons()


def get_template(name: str) -> dict:
    """A shipped head + its bib_style/toml preset."""
    p = template_path(name)
    preset = template_preset(name)
    return {"name": name, "head": p.read_text(encoding="utf-8"),
            "bib_style": preset["bib_style"], "toml": preset["toml"]}


def get_skeleton(name: str) -> dict:
    """A shipped whole-document skeleton's content (level 3)."""
    p = template_path(name)
    return {"name": name, "skeleton": p.read_text(encoding="utf-8")}


def version() -> str:
    """ikat version."""
    return __version__


#: The legend keywords (also the CLI `--legend` choices); an unknown
#: word is a build error naming this set.
LEGEND_KEYWORDS = ("auto", "below", "top-left", "top-right",
                   "bottom-left", "bottom-right", "outside-right")

TOOL_REGISTRY = (
    ToolSpec(
        "weave_document",
        "Weave Markdown + ikat.toml + spec JSON into LaTeX (no TeX needed).\n\n"
        "Returns title/body/tex/diagram+table counts.",
        weave_document,
        (
            ToolParam("md_text", str, description="Markdown manuscript (whole document; display math as $$…$$ paragraphs)"),
            ToolParam("toml_text", str, description="ikat.toml source (class, columns, spans, floats); empty = defaults"),
            ToolParam("spec_json", str, description="BuildSpec JSON (same shape as CLI --spec): diagrams/plots/table captions/bib_style, plus authors/affiliations/author_affils (full author front matter; author_affils[i] = 0-based affiliation indices per author) and authors_max (opt-in: first N names + ', et al.', 0 = never); unknown keys ignored"),
        ),
        returns=dict,
    ),
    ToolSpec(
        "flowchart_to_tikz",
        "Mermaid flowchart block -> standalone tikzpicture (strict subset).",
        flowchart_to_tikz,
        (
            ToolParam("src", str, description="Mermaid block: graph TD/LR, sequenceDiagram, or stateDiagram-v2 (header dispatch); strict subset, errors name the statement"),
        ),
    ),
    ToolSpec(
        "barchart_to_tikz",
        "Grouped bar chart with min/max whiskers -> tikzpicture (pgfplots).",
        barchart_tool,
        (
            ToolParam("title", str, description="Chart title"),
            ToolParam("ylabel", str, description="Y-axis label"),
            ToolParam("log_y", bool, description="Logarithmic y axis (whisker tops log-mapped for legend collision checks)"),
            ToolParam("group_labels", list, description="X-axis group labels"),
            ToolParam("series_names", list, description="One entry per series (legend entries)"),
            ToolParam("values", list, description="values[series][group] — measured data, never invented"),
            ToolParam("mins", list, description="Lower whiskers, same shape as values"),
            ToolParam("maxs", list, description="Upper whiskers, same shape as values"),
            ToolParam("legend", str, "auto", "Position keyword: auto (default: first collision-free inside corner TL,TR,BR,BL, else below) | below | four corners | outside-right", enum=LEGEND_KEYWORDS),
            ToolParam("preset_json", str, "", "Preset document (same schema as load_preset); when non-empty it is rendered and the explicit data args above are ignored"),
        ),
    ),
    ToolSpec(
        "lineplot_to_tikz",
        "Line plot with symmetric error bars -> tikzpicture (pgfplots).",
        lineplot_tool,
        (
            ToolParam("title", str, description="Chart title"),
            ToolParam("xlabel", str, description="X-axis label"),
            ToolParam("ylabel", str, description="Y-axis label"),
            ToolParam("xs", list, description="X coordinates, strictly increasing"),
            ToolParam("names", list, description="One entry per series (legend entries)"),
            ToolParam("yss", list, description="yss[series][point] — measured data, never invented"),
            ToolParam("errs", list, description="Symmetric error bars, same shape as yss"),
            ToolParam("legend", str, "auto", "Same keywords as barchart_to_tikz: auto | below | four corners | outside-right", enum=LEGEND_KEYWORDS),
            ToolParam("preset_json", str, "", "Preset document (same schema as load_preset); when non-empty it is rendered and the explicit data args above are ignored"),
        ),
    ),
    ToolSpec(
        "standalone_figure",
        "One mermaid/chart fence -> wrapped standalone `fig.tex` source.\n\n"
        "Returns {lang, tex, needs}; the CALLER compiles (workdir-bound\n"
        "binaries stay out of MCP — same rule as `compile_pdf`; drive it\n"
        "from `ikat standalone` / `ikat build`). Exactly one fence is\n"
        "accepted; more is an error naming the count.",
        standalone_figure,
        (
            ToolParam("md_text", str, description="Markdown containing exactly one mermaid or chart fence"),
        ),
        returns=dict,
    ),
    ToolSpec(
        "suggest_layout",
        "Measure floats, suggest span/pos/width per fence (auto-layout).\n\n"
        "Advisory only: returns {suggestions, report}; nothing is\n"
        "rewritten. Plots are report-only (the caller owns `plot_attrs`).\n"
        "Errors name manuscript lines, like the weaver.",
        suggest_layout,
        (
            ToolParam("md_text", str, description="Markdown manuscript whose fences to measure"),
            ToolParam("toml_text", str, "", "ikat.toml source (column geometry + span defaults); empty = defaults"),
            ToolParam("spec_json", str, "{}", "BuildSpec JSON; only plots/table metadata matter here"),
        ),
        returns=dict,
    ),
    ToolSpec(
        "check_tex_packages",
        "Which TeX packages a .tex source needs / misses (kpsewhich probe).",
        check_tex_env,
        (
            ToolParam("tex_source", str, description="LaTeX source to scan for \\usepackage/\\documentclass needs"),
        ),
        returns=dict,
    ),
    ToolSpec(
        "ensure_tex_packages",
        "Probe and tlmgr-install missing TeX packages; report + fix-it hint.\n\n"
        "The only tool here that mutates anything (the TeX environment;\n"
        "tlmgr downloads from CTAN mirrors).",
        ensure_tex_packages,
        (
            ToolParam("tex_source", str, description="LaTeX source whose missing packages should be installed"),
            ToolParam("install", bool, True, "true (default) runs tlmgr install for every missing package, then re-probes; false only reports"),
        ),
        readonly=False, openworld=True,
        returns=dict,
    ),
    ToolSpec(
        "float_packages",
        "Extra packages for hand-written floats: `[H]` -> float, `\\FloatBarrier` -> placeins.",
        float_packages,
        (
            ToolParam("tex_source", str, description="LaTeX source to scan for [H]/\\FloatBarrier usage"),
        ),
        returns=list,
    ),
    ToolSpec(
        "list_templates",
        "Shipped heads (arxiv, acm-sigconf, ...) plus skeletons.",
        list_templates,
        (),
        returns=list,
    ),
    ToolSpec(
        "get_template",
        "A shipped head + its bib_style/toml preset.",
        get_template,
        (
            ToolParam("name", str, description="Head name as listed by list_templates, e.g. 'arxiv', 'ieee-conference'"),
        ),
        returns=dict,
    ),
    ToolSpec(
        "list_skeletons",
        "Shipped whole-document skeletons (level 3, `{{body}}` contract).",
        list_skeletons,
        (),
        returns=list,
    ),
    ToolSpec(
        "get_skeleton",
        "A shipped whole-document skeleton's content (level 3).",
        get_skeleton,
        (
            ToolParam("name", str, description="Skeleton name as listed by list_skeletons, e.g. 'skeleton-plain'"),
        ),
        returns=dict,
    ),
    ToolSpec(
        "version",
        "ikat version.",
        version,
        (),
    ),
)

_REGISTRY_BY_NAME = {spec.name: spec for spec in TOOL_REGISTRY}


def execute_tool(name: str, arguments: dict | None = None) -> Any:
    """Dispatch a tool call by name — the shared executor.

    Single dispatcher for every surface: LLM function-calling
    consumers and scripted automation drive the whole surface with a
    ``(name, arguments)`` pair and get the tool's natural result
    (dict / str / list), no MCP installed.

    Raises ValueError naming the valid tools for an unknown name.
    """
    spec = _REGISTRY_BY_NAME.get(name)
    if spec is None:
        raise ValueError(
            f"Unknown tool: {name!r}. Valid tools: "
            + ", ".join(spec.name for spec in TOOL_REGISTRY)
        )
    return spec.method(**spec.kwargs(arguments))


def llm_tool_definitions() -> list[dict]:
    """Function-calling definitions derived from the registry.

    Same data the MCP schemas render — parameter descriptions, enums
    (legend keywords), defaults, required lists — so the LLM surface
    cannot drift from the MCP surface.
    """
    return [spec.llm_definition() for spec in TOOL_REGISTRY]


__all__ = [
    "LEGEND_KEYWORDS",
    "TOOL_REGISTRY",
    "ToolParam",
    "ToolSpec",
    "execute_tool",
    "llm_tool_definitions",
]
