"""Document pipeline: Markdown fences → floated LaTeX elements.

An `Element` is one floatable unit (diagram, plot, table). Its column
span resolves from three levels, weakest first:

1. `ikat.toml [spans]` default for the element kind
   (`diagram = "wide"`, `plot/table = "column"`);
2. per-element override on the fence info line,
   e.g. ```` ```mermaid {span=column} ````;
3. explicit `span=` argument at the call site.

`figure` spans one column, `figure*` spans both — the same vocabulary
the paper build already uses, now configurable instead of hardcoded.
"""

from __future__ import annotations

import re
from dataclasses import dataclass, field

from ._core import flowchart_to_tikz as _flowchart_to_tikz
from ._core import parse_config as _parse_config

_FENCE_RE = re.compile(r"```(\w+)([^\n]*)\n(.*?)```", re.DOTALL)
_ATTR_RE = re.compile(r"\{([^}]*)\}")


@dataclass
class Element:
    kind: str  # "diagram" | "plot" | "table" | ...
    body_tex: str
    caption: str = ""
    label: str = ""
    span: str | None = None  # "column" | "wide" | None (= configured)
    extra: dict = field(default_factory=dict)


def config_spans(toml_src: str) -> dict:
    """Resolved `span_<kind> -> figure|figure*` mapping from ikat.toml."""
    return _parse_config(toml_src)


def resolve_span(kind: str, local: str | None, spans: dict) -> str:
    """Final span name (`"column"` / `"wide"`) for one element."""
    if local in ("column", "wide"):
        return local
    env = spans.get(f"span_{kind}", spans.get("span_default", "figure"))
    return "wide" if env == "figure*" else "column"


def figure_env(body_tex: str, caption: str, label: str, span: str) -> str:
    """Wrap TikZ/figure body in a float of the right width."""
    env = "figure*" if span == "wide" else "figure"
    width = "\\textwidth" if span == "wide" else "\\columnwidth"
    width_opt = f"[width={width}]"
    if body_tex.lstrip().startswith("\\begin{tikzpicture}"):
        body = body_tex
    else:  # external graphic: scale it
        body = f"\\includegraphics{width_opt}{{{body_tex}}}"
    lines = [
        f"\\begin{{{env}}}[t]",
        "\\centering",
        body,
        f"\\caption{{{caption}}}",
        f"\\label{{{label}}}",
        f"\\end{{{env}}}",
    ]
    return "\n".join(lines)


def extract_fences(md: str) -> list[tuple[str, str, str]]:
    """All fenced blocks: (language, `{attr}` string, body)."""
    return [
        (lang, (_ATTR_RE.search(info) or [None, ""])[1], body)
        for lang, info, body in _FENCE_RE.findall(md)
    ]


def _span_attr(attr: str) -> str | None:
    m = re.search(r"span\s*=\s*(column|wide)", attr)
    return m.group(1) if m else None


def weave_fragment(md: str, toml_src: str = "") -> list[Element]:
    """Convert every `mermaid` fence in `md` to a spanned TikZ element.

    Captions/labels come from a `%% caption: ...` / `%% label: ...`
    comment on the fence's first lines; span from `{span=...}` or the
    configured kind default.
    """
    spans = config_spans(toml_src)
    out: list[Element] = []
    for lang, attr, body in extract_fences(md):
        if lang != "mermaid":
            continue
        caption = label = ""
        for line in body.splitlines()[:4]:
            s = line.strip()
            if s.startswith("%% caption:"):
                caption = s.split(":", 1)[1].strip()
            elif s.startswith("%% label:"):
                label = s.split(":", 1)[1].strip()
        tikz = _flowchart_to_tikz(body)
        span = resolve_span("diagram", _span_attr(attr), spans)
        out.append(Element("diagram", tikz, caption, label, span))
    return out
