"""Phase 1: document assembly. Thin wrapper — all compute lives in Rust.

`BuildSpec` carries everything document-specific that `ikat.toml`
doesn't cover (diagram/plot/table registries, thanks, author,
bibliography name, the validated `.bib` keyset). It serializes to
JSON over the PyO3 boundary; `src/doc.rs` does the rest and returns
`BuildResult`.
"""

from __future__ import annotations

import json
from dataclasses import asdict, dataclass, field

from ._core import bib_keys as _bib_keys
from ._core import bib_safe as _bib_safe
from ._core import build_document as _build_document


@dataclass
class DiagramEntry:
    key: str
    caption: str
    mode: str = "precompiled"
    path_prefix: str = "figs/tikz/"


@dataclass
class BuildSpec:
    diagrams: list[DiagramEntry] = field(default_factory=list)
    plots: list[tuple[str, str]] = field(default_factory=list)
    plot_dir: str = "figs/"
    table_captions: list[str] = field(default_factory=list)
    table_specs: dict[int, str] = field(default_factory=dict)
    plot_insert_before: str = ""
    title_thanks: str = ""
    author: str = "opticsWolf"
    bib_name: str = "refs-paper"
    graphicspaths: tuple[str, ...] = ("./", "figs/", "figs/tikz/")
    bib_keys: set = field(default_factory=set)

    def to_json(self) -> str:
        d = asdict(self)
        d["graphicspaths"] = list(self.graphicspaths)
        d["bib_keys"] = sorted(self.bib_keys)
        d["table_specs"] = {str(k): v for k, v in self.table_specs.items()}
        return json.dumps(d)


@dataclass
class BuildResult:
    title: str
    body: str
    tex: str
    n_diagrams: int
    n_tables: int


def build_document(md_text: str, toml_src: str, spec: BuildSpec) -> BuildResult:
    r = json.loads(_build_document(md_text, toml_src, spec.to_json()))
    return BuildResult(r["title"], r["body"], r["tex"], r["n_diagrams"], r["n_tables"])


def bib_keys(bib_src: str) -> set:
    return set(_bib_keys(bib_src))


def bib_safe(bib_src: str) -> str:
    return _bib_safe(bib_src)
