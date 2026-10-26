"""ikat MCP server: the CLI/API operations as agent tools.

Run:  python -m ikat.mcp_server        (stdio transport)
Requires the `mcp` extra:  pip install ikat[mcp]   (pins mcp>=1,<2)

The server is THIN: every tool is generated from
``ikat.registry.TOOL_REGISTRY`` — name, description, parameter
schemas (descriptions + legend enum), return type, and MCP
annotations — so the MCP surface cannot drift from the registry.
The same registry feeds :func:`ikat.registry.llm_tool_definitions`
(function-calling consumers) and :func:`ikat.registry.execute_tool`
(drive the surface from a ``(name, arguments)`` pair with no MCP
installed).

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
| skills         | ikat.skills install     | ikat skills            | (resources below)   |
| version        | ikat.__version__        | ikat version           | version             |

`compile_pdf` stays out of MCP: it is workdir-bound and slow;
drive it from the CLI (`ikat build`) instead.

Skill packs ship inside the package and are served as resources
`skill://ikat`, `skill://ikat-api`, `skill://ikat-cli`, and
`skill://ikat-mcp` (routing, signatures, doctrine — no repo checkout
needed); `ikat skills install` copies them to disk for harnesses
that discover skills as directories.
"""

from __future__ import annotations

import inspect
from typing import Annotated, Literal

from pydantic import Field

try:
    from mcp.server.fastmcp import FastMCP
    from mcp.types import ToolAnnotations
except ImportError as e:  # pragma: no cover
    raise SystemExit("ikat MCP needs the mcp extra: pip install ikat[mcp]") from e

from ikat import __version__
from ikat.registry import (
    TOOL_REGISTRY,
    ToolSpec,
    execute_tool as _execute_tool,
    llm_tool_definitions,
)
from ikat.registry import (  # re-exported: the composites live in the registry
    barchart_tool as barchart_to_tikz,
    float_packages,
    get_skeleton,
    get_template,
    lineplot_tool as lineplot_to_tikz,
    llm_tool_definitions,  # noqa: F401  (consumers grab it from here too)
    list_skeletons,
    list_templates,
    standalone_figure,
    suggest_layout,
    version,
    weave_document,
)
from ikat.skills import skill_description, skill_names, skill_text

from ikat import flowchart_to_tikz  # noqa: F401  (parity with the API name)

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


def _mcp_tool_for(spec: ToolSpec):
    """Build an MCP tool wrapper for one registry entry.

    Name, docstring, signature (with per-parameter Field
    descriptions and Literal enums), return annotation, and
    annotations are all generated from the spec, so the MCP input
    schema cannot drift from the registry. Every call dispatches
    through the registry's shared executor.
    """
    parameters = []
    annotations: dict = {}
    for p in spec.params:
        if p.enum is not None and p.type is str:
            base = Literal[tuple(p.enum)]  # type: ignore[misc]
        else:
            base = p.type
        if p.required:
            annotated = Annotated[base, Field(description=p.description)]
            parameters.append(inspect.Parameter(
                p.name, inspect.Parameter.POSITIONAL_OR_KEYWORD,
                annotation=annotated))
        else:
            annotated = Annotated[base, Field(description=p.description, default=p.default)]
            parameters.append(inspect.Parameter(
                p.name, inspect.Parameter.POSITIONAL_OR_KEYWORD,
                default=p.default, annotation=annotated))
        annotations[p.name] = annotated
    annotations["return"] = spec.returns

    def wrapper(**kwargs):
        return _execute_tool(spec.name, kwargs)

    wrapper.__name__ = spec.name
    wrapper.__doc__ = spec.description
    wrapper.__signature__ = inspect.Signature(parameters=parameters)
    wrapper.__annotations__ = annotations
    return wrapper


def build_server() -> FastMCP:
    """Build a fully-wired FastMCP instance (no I/O).

    Tools are registered from ``TOOL_REGISTRY`` (the single source of
    truth); the skill packs ride along as resources.
    """
    server = FastMCP("ikat", instructions=INSTRUCTIONS)
    for spec in TOOL_REGISTRY:
        annotations = ToolAnnotations(
            readOnlyHint=spec.readonly,
            destructiveHint=spec.destructive,
            idempotentHint=spec.idempotent,
            openWorldHint=spec.openworld,
        )
        server.tool(annotations=annotations)(_mcp_tool_for(spec))

    for name in skill_names():
        _register_skill_resource(server, name)
    return server


def _register_skill_resource(server: FastMCP, name: str) -> None:
    """Serve one bundled skill pack as a resource."""

    @server.resource(
        f"skill://{name}",
        name=name,
        description=skill_description(name),
        mime_type="text/markdown",
    )
    def _read_skill() -> str:
        """The bundled SKILL.md, verbatim."""
        return skill_text(name)


mcp = build_server()


def main() -> None:
    mcp.run()


if __name__ == "__main__":
    main()
