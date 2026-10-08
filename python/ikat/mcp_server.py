"""ikat MCP server: the CLI/API operations as agent tools.

Run:  python -m ikat.mcp_server        (stdio transport)
Requires the `mcp` extra:  pip install ikat[mcp]   (pins mcp>=1,<2)

Same operations, three surfaces:

| operation      | Python API              | CLI                    | MCP tool            |
|----------------|-------------------------|------------------------|---------------------|
| weave document | build_document /        | ikat build / weave     | weave_document      |
|                | build_from_paths        |                        |                     |
| flowchart      | flowchart_to_tikz       | ikat flowchart         | flowchart_to_tikz   |
| standalone fig | compile_standalone /    | ikat standalone        | standalone_figure   |
|                | wrap_standalone         |                        | (returns .tex only) |
| bar chart      | barchart_to_tikz        | —                      | barchart_to_tikz    |
| line plot      | lineplot_to_tikz        | —                      | lineplot_to_tikz    |
| check packages | check_tex_env           | ikat check             | check_tex_packages  |
| install pkgs   | ensure_tex_packages     | ikat check --install   | ensure_tex_packages |
| templates      | list_templates /        | ikat templates /       | list_templates /    |
|                | template_path/preset    | ikat template          | get_template        |

`compile_pdf` stays out of MCP: it is workdir-bound and slow;
drive it from the CLI (`ikat build`) instead.
"""

from __future__ import annotations

try:
    from mcp.server.fastmcp import FastMCP
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

mcp = FastMCP("ikat")


@mcp.tool()
def weave_document(md_text: str, toml_text: str, spec_json: str) -> dict:
    """Weave Markdown + ikat.toml + spec JSON into LaTeX (no TeX needed).

    spec_json carries diagrams/plots/table captions/bib_style (see
    `ikat build --help`); returns title/body/tex/diagram+table counts.
    """
    r = _build(md_text, toml_text, spec_from_dict(_json.loads(spec_json)))
    return {"title": r.title, "body": r.body, "tex": r.tex,
            "n_diagrams": r.n_diagrams, "n_tables": r.n_tables}


@mcp.tool()
def flowchart_to_tikz(src: str) -> str:
    """Mermaid flowchart block -> standalone tikzpicture (strict subset)."""
    return _flow(src)


@mcp.tool()
def barchart_to_tikz(title: str, ylabel: str, log_y: bool, group_labels: list[str],
                     series_names: list[str], values: list[list[float]],
                     mins: list[list[float]], maxs: list[list[float]],
                     legend: str = "auto", preset_json: str = "") -> str:
    """Grouped bar chart with min/max whiskers -> tikzpicture (pgfplots).

    `legend`: auto (default: first collision-free inside corner,
    else below) | below | top-left | top-right | bottom-left
    | bottom-right | outside-right.
    `preset_json`: when non-empty, a preset document (same schema as
    `load_preset`) rendered instead — the explicit data args above
    are ignored."""
    if preset_json:
        from ikat.preset import load_preset
        return load_preset(preset_json)
    return _bar(title, ylabel, log_y, group_labels, series_names, values, mins, maxs, None, legend)


@mcp.tool()
def lineplot_to_tikz(title: str, xlabel: str, ylabel: str, xs: list[float],
                     names: list[str], yss: list[list[float]],
                     errs: list[list[float]], legend: str = "auto", preset_json: str = "") -> str:
    """Line plot with symmetric error bars -> tikzpicture (pgfplots).

    `legend`: same keywords as `barchart_to_tikz`.
    `preset_json`: when non-empty, a preset document rendered
    instead — the explicit data args above are ignored."""
    if preset_json:
        from ikat.preset import load_preset
        return load_preset(preset_json)
    return _line(title, xlabel, ylabel, xs, names, yss, errs, legend)


@mcp.tool()
def standalone_figure(md_text: str) -> dict:
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


@mcp.tool()
def check_tex_packages(tex_source: str) -> dict:
    """Which TeX packages a .tex source needs / misses (kpsewhich probe)."""
    return _check(tex_source)


@mcp.tool()
def ensure_tex_packages(tex_source: str, install: bool = True) -> dict:
    """Probe and tlmgr-install missing TeX packages; report + fix-it hint."""
    return _ensure(tex_source, install=install)


@mcp.tool()
def float_packages(tex_source: str) -> list[str]:
    """Extra packages for hand-written floats: `[H]` -> float, `\\FloatBarrier` -> placeins."""
    from ikat import tex_extra_packages as _xp

    return _xp(tex_source)


@mcp.tool()
def list_templates() -> list[str]:
    """Shipped heads (arxiv, acm-sigconf, ...) plus skeletons."""
    return _templates() + _skeletons()


@mcp.tool()
def get_template(name: str) -> dict:
    """A shipped head + its bib_style/toml preset."""
    p = _tpath(name)
    preset = _preset(name)
    return {"name": name, "head": p.read_text(encoding="utf-8"),
            "bib_style": preset["bib_style"], "toml": preset["toml"]}


@mcp.tool()
def list_skeletons() -> list[str]:
    """Shipped whole-document skeletons (level 3, `{{body}}` contract)."""
    return _skeletons()


@mcp.tool()
def get_skeleton(name: str) -> dict:
    """A shipped whole-document skeleton's content (level 3)."""
    p = _tpath(name)
    return {"name": name, "skeleton": p.read_text(encoding="utf-8")}


@mcp.tool()
def version() -> str:
    """ikat version."""
    return ikat.__version__


def main() -> None:
    mcp.run()


if __name__ == "__main__":
    main()
