"""Benchmark-JSON preset loader: JSON in, tikzpicture out.

Thin wrapper over the Rust core (`ikat._core.preset_to_tikz`).
Rust never touches the filesystem: this module reads preset files
and names them in errors. A string that is an existing file path
is read as a file; anything else is parsed as raw JSON.
"""

from __future__ import annotations

from pathlib import Path

from ._core import preset_to_tikz as _preset_to_tikz


def load_preset(path_or_src: str) -> str:
    """Render a preset (file path or raw JSON string) to TikZ."""
    p = Path(str(path_or_src))
    if p.is_file():
        src = p.read_text(encoding="utf-8")
        origin = str(p)
    else:
        src = str(path_or_src)
        origin = "<preset>"
    try:
        return _preset_to_tikz(src)
    except ValueError as e:
        raise ValueError(f"preset {origin}: {e}") from e
