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
    pos: str | None = None  # top|bottom|both|page|here|force|barrier
    width: str | None = None  # fraction of span width or TeX length
    captionpos: str | None = None  # top|bottom|None (= convention)
    extra: dict = field(default_factory=dict)


_POS_SPEC = {
    "top": "[t]", "bottom": "[b]", "both": "[!tb]", "page": "[p]",
    "here": "[h]", "force": "[H]", "barrier": "[t]",
}


def config_spans(toml_src: str) -> dict:
    """Resolved `span_<kind> -> figure|figure*` mapping from ikat.toml."""
    return _parse_config(toml_src)


def resolve_span(kind: str, local: str | None, spans: dict) -> str:
    """Final span name (`"column"` / `"wide"`) for one element."""
    if local in ("column", "wide"):
        return local
    env = spans.get(f"span_{kind}", spans.get("span_default", "figure"))
    return "wide" if env == "figure*" else "column"


def resolve_pos(kind: str, local: str | None, floats: dict | None = None) -> str:
    """Final pos name for one element (toml default, else per-kind map)."""
    if local in _POS_SPEC:
        return local
    return (floats or {}).get("float_pos_default", "top")


def figure_env(body_tex: str, caption: str, label: str, span: str,
               pos: str = "top", width: str | None = None,
               captionpos: str | None = None) -> str:
    """Wrap TikZ/figure body in a float of the right width and place."""
    env = "figure*" if span == "wide" else "figure"
    if span == "wide" and pos in ("here", "force"):
        raise ValueError(f"figure*: pos {pos} is illegal on full-width floats")
    sw = "\\textwidth" if span == "wide" else "\\columnwidth"
    if width is None:
        w = sw
    elif re.fullmatch(r"[0-9.]*[0-9]", width or ""):
        w = f"{width}{sw}"
    else:
        w = width
    width_opt = f"[width={w}]"
    if body_tex.lstrip().startswith("\\begin{tikzpicture}"):
        body = body_tex
        if width is not None and re.fullmatch(r"[0-9.]*[0-9]", width) and "[scale=" not in body:
            body = body.replace("\\begin{tikzpicture}", "\\begin{tikzpicture}[scale=" + width + "]", 1)
    else:  # external graphic: scale it
        body = f"\\includegraphics{width_opt}{{{body_tex}}}"
    spec = _POS_SPEC.get(pos, "[t]")
    cap = f"\\caption{{{caption}}}\n\\label{{{label}}}"
    inner = f"{body}\n{cap}" if (captionpos or "bottom") == "bottom" else f"{cap}\n{body}"
    prefix = "\\FloatBarrier\n" if pos == "barrier" else ""
    lines = [f"{prefix}\\begin{{{env}}}{spec}", "\\centering", inner, f"\\end{{{env}}}"]
    return "\n".join(lines)


def extract_fences(md: str) -> list[tuple[str, str, str]]:
    """All fenced blocks: (language, `{attr}` string, body)."""
    return [
        (lang, (_ATTR_RE.search(info) or [None, ""])[1], body)
        for lang, info, body in _FENCE_RE.findall(md)
    ]


def _attr(attr: str, key: str, allowed: tuple[str, ...]) -> str | None:
    m = re.search(rf"{key}\s*=\s*([^\s}}]+)", attr)
    if not m:
        return None
    if m.group(1) not in allowed:
        raise ValueError(f"{key} must be {'|'.join(allowed)}, got {m.group(1)!r}")
    return m.group(1)


def _width_attr(attr: str) -> str | None:
    m = re.search(r"width\s*=\s*([^\s}]+)", attr)
    if not m:
        return None
    v = m.group(1)
    if re.fullmatch(r"[0-9.]*[0-9]", v):
        if not 0 < float(v) <= 2:
            raise ValueError(f"width fraction must be in (0, 2], got {v!r}")
        return v
    if re.fullmatch(r"[0-9.]+(cm|mm|in|pt|pc|bp|dd|cc|sp|em|ex)", v) or v.startswith("\\"):
        return v
    raise ValueError(f"width must be a fraction or TeX length, got {v!r}")


def _span_attr(attr: str) -> str | None:
    return _attr(attr, "span", ("column", "wide"))


def weave_fragment(md: str, toml_src: str = "") -> list[Element]:
    """Convert every `mermaid` fence in `md` to a spanned TikZ element.

    Captions/labels come from a `%% caption: ...` / `%% label: ...`
    comment on the fence's first lines; span from `{span=...}` or the
    configured kind default.
    """
    spans = config_spans(toml_src)
    floats = {k: v for k, v in spans.items() if k.startswith("float_")}
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
        pos = resolve_pos("diagram", _attr(attr, "pos", tuple(_POS_SPEC)), floats)
        out.append(Element("diagram", tikz, caption, label, span, pos,
                           _width_attr(attr), _attr(attr, "captionpos", ("top", "bottom"))))
    return out
