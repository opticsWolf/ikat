"""Auto-layout (M7.1): measure floats, suggest and apply placements.

Thin wrappers over `ikat._core.suggest_layout` (all measuring and
policy lives in Rust, `src/autolayout.rs`). `suggest()` is advisory
— it returns per-fence suggestions, nothing is rewritten.
`apply()` rewrites mermaid-fence and `%% table` attrs in the
Markdown for suggestions with `action == "change"`. Plots ride the
caller's hook (`plot_insert_before`), not md fences, so plot
suggestions are report-only: put the suggested attrs into
`plot_attrs` by hand.
"""

from __future__ import annotations

import json

import ikat
from ikat.document import BuildSpec, spec_from_dict


def suggest(
    md_text: str,
    toml_src: str = "",
    spec: BuildSpec | dict | None = None,
) -> list[dict]:
    """Measure every floatable fence and suggest span/pos/width.

    Each suggestion: `index` (fence ordinal within its kind),
    `kind` (`diagram`/`picture`/`table`/`plot`), `key`, `line`,
    `est_w_cm`/`est_h_cm` (content box, caption excluded),
    `span`, `pos`, `width` (fraction or None), `action`
    (`change`/`keep`), `reason` (one line with numbers).
    Raises `ValueError` on strict-subset or registry errors.
    """
    if spec is None:
        spec = BuildSpec()
    elif isinstance(spec, dict):
        spec = spec_from_dict(spec)
    return json.loads(ikat._core.suggest_layout(md_text, toml_src, spec.to_json()))


def report(suggestions: list[dict]) -> str:
    """One human-readable line per suggestion."""
    lines = []
    for s in suggestions:
        w = f" width={s['width']:.2f}" if s.get("width") is not None else ""
        lines.append(
            f"{s['kind']} #{s['index']} ({s['key']}, line {s['line']}): "
            f"est {s['est_w_cm']:.1f}x{s['est_h_cm']:.1f}cm -> "
            f"{s['span']}+{s['pos']}{w} [{s['action']}]: {s['reason']}"
        )
    return "\n".join(lines)


def _fence_attr_line(s: dict) -> str:
    w = f" width={s['width']:.2f}" if s.get("width") is not None else ""
    return f"span={s['span']} pos={s['pos']}{w}"


def apply(md_text: str, suggestions: list[dict]) -> tuple[str, int]:
    """Rewrite fence attrs for `change` suggestions.

    Mermaid fences are matched by ordinal; tables by ordinal via
    their pending `%% table` directive (inserted when missing).
    Returns `(new_md, n_changed)`. Plots are skipped (report-only).
    """
    changes = [s for s in suggestions if s.get("action") == "change" and s["kind"] in ("diagram", "table")]
    if not changes:
        return md_text, 0
    by_kind: dict[str, dict[int, dict]] = {}
    for s in changes:
        by_kind.setdefault(s["kind"], {})[s["index"]] = s

    lines = md_text.split("\n")
    out: list[str] = []
    n_changed = 0
    mermaid_seen = -1
    pending_directive: int | None = None
    table_seen = -1

    def flush_table(i: int) -> None:
        # A table block starts at out-line i: attach or insert its
        # directive. (Called before appending the row line.)
        nonlocal n_changed, pending_directive, table_seen
        table_seen += 1
        s = by_kind.get("table", {}).get(table_seen)
        if s is None:
            pending_directive = None
            return
        new_attrs = _table_attrs(s, out[pending_directive] if pending_directive is not None else "")
        if pending_directive is not None:
            out[pending_directive] = _rewrite_directive(out[pending_directive], new_attrs)
        else:
            out.append(f"%% table {{{new_attrs}}}")
        n_changed += 1
        pending_directive = None

    i = 0
    while i < len(lines):
        ln = lines[i]
        stripped = ln.strip()
        if stripped.startswith("```mermaid"):
            mermaid_seen += 1
            s = by_kind.get("diagram", {}).get(mermaid_seen)
            if s is not None:
                ln = _rewrite_fence(ln, s)
                n_changed += 1
            out.append(ln)
            i += 1
            continue
        if stripped.startswith("%% table"):
            pending_directive = len(out)
            out.append(ln)
            i += 1
            continue
        if stripped.startswith("|"):
            # Table block start (previous out-line is not a row).
            prev = out[-1].strip() if out else ""
            if not prev.startswith("|"):
                flush_table(len(out))
            out.append(ln)
            i += 1
            continue
        out.append(ln)
        i += 1
    return "\n".join(out), n_changed


def _keep_captionpos(old_attrs: str) -> str:
    for tok in old_attrs.split():
        if tok.startswith("captionpos="):
            return " " + tok
        if tok.startswith("captionpos"):
            return " " + tok
    return ""


def _rewrite_fence(fence_line: str, s: dict) -> str:
    """Replace the `{...}` attr block of a ```mermaid fence line."""
    a, b = fence_line.find("{"), fence_line.rfind("}")
    old = fence_line[a + 1 : b] if 0 <= a < b else ""
    new_attrs = _fence_attr_line(s) + _keep_captionpos(old)
    head = fence_line[:a] if a >= 0 else fence_line
    return f"{head}{{{new_attrs}}}"


def _table_attrs(s: dict, directive_line: str) -> str:
    old = ""
    if directive_line:
        a, b = directive_line.find("{"), directive_line.rfind("}")
        old = directive_line[a + 1 : b] if 0 <= a < b else ""
    return _fence_attr_line(s) + _keep_captionpos(old)


def _rewrite_directive(directive_line: str, new_attrs: str) -> str:
    a = directive_line.find("{")
    head = directive_line[:a] if a >= 0 else directive_line
    return f"{head}{{{new_attrs}}}"
