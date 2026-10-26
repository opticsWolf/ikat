"""ikat MCP server: the CLI/API operations as agent tools.

Run:  python -m ikat.mcp_server        (stdio transport)
Requires the `mcp` extra:  pip install ikat[mcp]   (pins mcp>=1,<2)

Same operations, three surfaces:

| operation      | Python API              | CLI                    | MCP tool            |
|----------------|-------------------------|------------------------|---------------------|
| weave document | build_document /        | ikat build / weave     | weave_document      |
|                | build_from_paths        |                        |                     |
| float layout   | suggest_layout          | ikat layout            | suggest_layout      |
| flowchart      | flowchart_to_tikz       | ikat flowchart         | flowchart_to_tikz   |
| standalone fig | compile_standalone /    | ikat standalone        | standalone_figure   |
|                | wrap_standalone         |                        | (returns .tex only) |
| bar chart      | barchart_to_tikz        | ikat barchart          | barchart_to_tikz    |
| line plot      | lineplot_to_tikz        | ikat lineplot          | lineplot_to_tikz    |
| check packages | check_tex_env           | ikat check             | check_tex_packages  |
| install pkgs   | ensure_tex_packages     | ikat check --install   | ensure_tex_packages |
| float packages | tex_extra_packages      | (in `ikat check`)      | float_packages      |
| templates      | list_templates /        | ikat templates /       | list_templates /    |
|                | template_path/preset    | ikat template          | get_template        |
| skeletons      | list_skeletons          | ikat skeletons /       | list_skeletons /    |
|                |                         | ikat skeleton          | get_skeleton        |
| version        | ikat.__version__        | ikat version           | version             |

`compile_pdf` stays out of MCP: it is workdir-bound and slow;
drive it from the CLI (`ikat build`) instead.

Skill packs ship inside the package and are served as resources
`skill://ikat`, `skill://ikat-api`, `skill://ikat-cli`, and
`skill://ikat-mcp` (routing, signatures, doctrine — no repo checkout
needed). Schemas carry per-parameter descriptions and enum constraints
(`legend` keywords), and every tool declares MCP annotations
(readOnly/destructive/idempotent/openWorld hints) so hosts can guard
the one mutating tool (`ensure_tex_packages`) without hardcoding.
"""

from __future__ import annotations

from typing import Annotated, Literal

from pydantic import Field

try:
    from mcp.server.fastmcp import FastMCP
    from mcp.types import ToolAnnotations
except ImportError as e:  # pragma: no cover
    raise SystemExit("ikat MCP needs the mcp extra: pip install ikat[mcp]") from e

import json as _json

import ikat
from ikat import spec_from_dict
from ikat import (
    barchart_to_tikz as _bar,
    build_document as _build,
    check_tex_env as _check,
    ensure_tex_packages as _ensure,
    flowchart_to_tikz as _flow,
    lineplot_to_tikz as _line,
    list_templates as _templates,
    list_skeletons as _skeletons,
    template_path as _tpath,
    template_preset as _preset,
)
from ikat.skills import skill_description, skill_names, skill_text

#: Legend keywords shared by barchart/lineplot (also the CLI `--legend`
#: choices); an unknown word is a build error naming this set.
Legend = Literal[
    "auto", "below", "top-left", "top-right",
    "bottom-left", "bottom-right", "outside-right",
]

INSTRUCTIONS = (
    "Weave Markdown into camera-ready LaTeX/PDF with a Rust core: whole\n"
    "documents (numbering, captions, bibliography), mermaid\n"
    "flowchart/sequence/state diagrams as TikZ, and pgfplots bar/line\n"
    "figures. Strict subset, fail-fast: errors name the manuscript line\n"
    "and echo the offending text instead of failing later in a TeX log.\n\n"
    "## Doctrine\n\n"
    "- Tools here never compile and never touch the network (only\n"
    "  ensure_tex_packages mutates anything, via tlmgr). Weave here,\n"
    "  then compile with the `ikat build` CLI — compile_pdf is\n"
    "  workdir-bound and slow and deliberately not a tool.\n"
    "- `spec_json` is BuildSpec JSON (same shape as CLI `--spec`):\n"
    "  diagram/plot/table registries, captions, bib_style. Unknown keys\n"
    "  are ignored; missing registry entries fail naming the fence.\n"
    "- The bundled skill packs are readable as resources: skill://ikat\n"
    "(general guide), skill://ikat-api, skill://ikat-cli,\n"
    "skill://ikat-mcp (this surface's own skill)."
)

# Annotation sets: every tool is local computation (no network) except
# ensure_tex_packages, whose tlmgr step downloads from CTAN mirrors and
# installs into the TeX environment — the only mutating tool here.
_PURE = ToolAnnotations(
    readOnlyHint=True, destructiveHint=False, idempotentHint=True,
    openWorldHint=False,
)
_INSTALL = ToolAnnotations(
    readOnlyHint=False, destructiveHint=False, idempotentHint=True,
    openWorldHint=True,
)

mcp = FastMCP("ikat", instructions=INSTRUCTIONS)


@mcp.tool(annotations=_PURE)
def weave_document(
    md_text: Annotated[str, Field(description="Markdown manuscript (whole document; display math as $$…$$ paragraphs)")],
    toml_text: Annotated[str, Field(description="ikat.toml source (class, columns, spans, floats); empty = defaults")],
    spec_json: Annotated[str, Field(description="BuildSpec JSON (same shape as CLI --spec): diagrams/plots/table captions/bib_style; unknown keys ignored")],
) -> dict:
    """Weave Markdown + ikat.toml + spec JSON into LaTeX (no TeX needed).

    Returns title/body/tex/diagram+table counts."""
    r = _build(md_text, toml_text, spec_from_dict(_json.loads(spec_json)))
    return {"title": r.title, "body": r.body, "tex": r.tex,
            "n_diagrams": r.n_diagrams, "n_tables": r.n_tables}


@mcp.tool(annotations=_PURE)
def flowchart_to_tikz(
    src: Annotated[str, Field(description="Mermaid block: graph TD/LR, sequenceDiagram, or stateDiagram-v2 (header dispatch); strict subset, errors name the statement")],
) -> str:
    """Mermaid flowchart block -> standalone tikzpicture (strict subset)."""
    return _flow(src)


@mcp.tool(annotations=_PURE)
def barchart_to_tikz(
    title: Annotated[str, Field(description="Chart title")],
    ylabel: Annotated[str, Field(description="Y-axis label")],
    log_y: Annotated[bool, Field(description="Logarithmic y axis (whisker tops log-mapped for legend collision checks)")],
    group_labels: Annotated[list[str], Field(description="X-axis group labels")],
    series_names: Annotated[list[str], Field(description="One entry per series (legend entries)")],
    values: Annotated[list[list[float]], Field(description="values[series][group] — measured data, never invented")],
    mins: Annotated[list[list[float]], Field(description="Lower whiskers, same shape as values")],
    maxs: Annotated[list[list[float]], Field(description="Upper whiskers, same shape as values")],
    legend: Annotated[Legend, Field(description="Position keyword: auto (default: first collision-free inside corner TL,TR,BR,BL, else below) | below | four corners | outside-right")] = "auto",
    preset_json: Annotated[str, Field(description="Preset document (same schema as load_preset); when non-empty it is rendered and the explicit data args above are ignored")] = "",
) -> str:
    """Grouped bar chart with min/max whiskers -> tikzpicture (pgfplots)."""
    if preset_json:
        from ikat.preset import load_preset
        return load_preset(preset_json)
    return _bar(title, ylabel, log_y, group_labels, series_names, values, mins, maxs, None, legend)


@mcp.tool(annotations=_PURE)
def lineplot_to_tikz(
    title: Annotated[str, Field(description="Chart title")],
    xlabel: Annotated[str, Field(description="X-axis label")],
    ylabel: Annotated[str, Field(description="Y-axis label")],
    xs: Annotated[list[float], Field(description="X coordinates, strictly increasing")],
    names: Annotated[list[str], Field(description="One entry per series (legend entries)")],
    yss: Annotated[list[list[float]], Field(description="yss[series][point] — measured data, never invented")],
    errs: Annotated[list[list[float]], Field(description="Symmetric error bars, same shape as yss")],
    legend: Annotated[Legend, Field(description="Same keywords as barchart_to_tikz: auto | below | four corners | outside-right")] = "auto",
    preset_json: Annotated[str, Field(description="Preset document (same schema as load_preset); when non-empty it is rendered and the explicit data args above are ignored")] = "",
) -> str:
    """Line plot with symmetric error bars -> tikzpicture (pgfplots)."""
    if preset_json:
        from ikat.preset import load_preset
        return load_preset(preset_json)
    return _line(title, xlabel, ylabel, xs, names, yss, errs, legend)


@mcp.tool(annotations=_PURE)
def standalone_figure(
    md_text: Annotated[str, Field(description="Markdown containing exactly one mermaid or chart fence")],
) -> dict:
    """One mermaid/chart fence -> wrapped standalone `fig.tex` source.

    Returns {lang, tex, needs}; the CALLER compiles (workdir-bound
    binaries stay out of MCP — same rule as `compile_pdf`; drive it
    from `ikat standalone` / `ikat build`). Exactly one fence is
    accepted; more is an error naming the count."""
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


@mcp.tool(annotations=_PURE)
def suggest_layout(
    md_text: Annotated[str, Field(description="Markdown manuscript whose fences to measure")],
    toml_text: Annotated[str, Field(description="ikat.toml source (column geometry + span defaults); empty = defaults")] = "",
    spec_json: Annotated[str, Field(description="BuildSpec JSON; only plots/table metadata matter here")] = "{}",
) -> dict:
    """Measure floats, suggest span/pos/width per fence (auto-layout).

    Advisory only: returns {suggestions, report}; nothing is
    rewritten. Plots are report-only (the caller owns `plot_attrs`).
    Errors name manuscript lines, like the weaver."""
    from ikat.layout import report as _report
    from ikat.layout import suggest as _suggest

    suggs = _suggest(md_text, toml_text, spec_from_dict(_json.loads(spec_json or "{}")))
    return {"suggestions": suggs, "report": _report(suggs)}


@mcp.tool(annotations=_PURE)
def check_tex_packages(
    tex_source: Annotated[str, Field(description="LaTeX source to scan for \\usepackage/\\documentclass needs")],
) -> dict:
    """Which TeX packages a .tex source needs / misses (kpsewhich probe)."""
    return _check(tex_source)


@mcp.tool(annotations=_INSTALL)
def ensure_tex_packages(
    tex_source: Annotated[str, Field(description="LaTeX source whose missing packages should be installed")],
    install: Annotated[bool, Field(description="true (default) runs tlmgr install for every missing package, then re-probes; false only reports")] = True,
) -> dict:
    """Probe and tlmgr-install missing TeX packages; report + fix-it hint.

    The only tool here that mutates anything (the TeX environment;
    tlmgr downloads from CTAN mirrors)."""
    return _ensure(tex_source, install=install)


@mcp.tool(annotations=_PURE)
def float_packages(
    tex_source: Annotated[str, Field(description="LaTeX source to scan for [H]/\\FloatBarrier usage")],
) -> list[str]:
    """Extra packages for hand-written floats: `[H]` -> float, `\\FloatBarrier` -> placeins."""
    from ikat import tex_extra_packages as _xp

    return _xp(tex_source)


@mcp.tool(annotations=_PURE)
def list_templates() -> list[str]:
    """Shipped heads (arxiv, acm-sigconf, ...) plus skeletons."""
    return _templates() + _skeletons()


@mcp.tool(annotations=_PURE)
def get_template(
    name: Annotated[str, Field(description="Head name as listed by list_templates, e.g. 'arxiv', 'ieee-conference'")],
) -> dict:
    """A shipped head + its bib_style/toml preset."""
    p = _tpath(name)
    preset = _preset(name)
    return {"name": name, "head": p.read_text(encoding="utf-8"),
            "bib_style": preset["bib_style"], "toml": preset["toml"]}


@mcp.tool(annotations=_PURE)
def list_skeletons() -> list[str]:
    """Shipped whole-document skeletons (level 3, `{{body}}` contract)."""
    return _skeletons()


@mcp.tool(annotations=_PURE)
def get_skeleton(
    name: Annotated[str, Field(description="Skeleton name as listed by list_skeletons, e.g. 'skeleton-plain'")],
) -> dict:
    """A shipped whole-document skeleton's content (level 3)."""
    p = _tpath(name)
    return {"name": name, "skeleton": p.read_text(encoding="utf-8")}


@mcp.tool(annotations=_PURE)
def version() -> str:
    """ikat version."""
    return ikat.__version__


# Skill packs as resources: an MCP-only host reads the same SKILL.md
# files a harness would install on disk (bundled in the wheel, pinned
# by the drift tests in tests/test_surface.py).
for _name in skill_names():
    _uri = f"skill://{_name}"

    def _make_skill_reader(_n: str):
        @mcp.resource(
            _uri,
            name=_n,
            description=skill_description(_n),
            mime_type="text/markdown",
        )
        def _read_skill() -> str:
            """The bundled SKILL.md, verbatim."""
            return skill_text(_n)

    _make_skill_reader(_name)


def main() -> None:
    mcp.run()


if __name__ == "__main__":
    main()
