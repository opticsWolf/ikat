"""Phase 1: document assembly. Thin wrapper — all compute lives in Rust.

`BuildSpec` carries everything document-specific that `ikat.toml`
doesn't cover (diagram/plot/table registries, thanks, author,
bibliography name, the validated `.bib` keyset). It serializes to
JSON over the PyO3 boundary; `src/doc.rs` does the rest and returns
`BuildResult`.
"""

from __future__ import annotations

import json
from dataclasses import asdict, dataclass, field, replace
from pathlib import Path

from ._core import bib_keys as _bib_keys
from ._core import bib_safe as _bib_safe
from ._core import build_document as _build_document
from ._core import parse_config as _parse_config

import re as _re
import zlib as _zlib

_LINE_RE = _re.compile(r"^([^\n]+)\n --> line (\d+)\n", _re.MULTILINE)

# Error kinds whose lines are head-file-relative (the template or
# skeleton text), not manuscript-relative. For these the head path
# is the honest attribution; everything else gets the md path.
_HEAD_MSGS = ("template ", "[template]", "skeleton ", "titlesec", "documentclass")


def with_file(path: str | Path | None, head_path: str | Path | None, fn, *args):
    """Call `fn(*args)`, upgrading a Rust `Error` Display into
    `file:line: message` (+ echo lines).

    Rust renders `{msg}\\n --> line {N}\\n{echo}` (no line part when
    no manuscript line applies). Python — the only layer that sees
    paths (Rust never touches the filesystem) — prepends the file:
    `md_path:N: msg` + echo. Template/skeleton errors resolve
    against the head file, so they get `head_path:N:` instead.
    Line-less errors become `path: msg` (+ echo). `path=None`
    (pure-string API) leaves the Rust rendering untouched.
    """
    try:
        return fn(*args)
    except ValueError as e:
        if path is None:
            raise
        text = str(e)
        m = _LINE_RE.match(text)
        if not m:
            raise ValueError(f"{path}: {text}") from e
        msg, n = m.group(1), m.group(2)
        echo = text[m.end():]
        owner = head_path if head_path and msg.startswith(_HEAD_MSGS) else path
        tail = f"\n{echo}" if echo else ""
        raise ValueError(f"{owner}:{n}: {msg}{tail}") from e


@dataclass
@dataclass
class LocalFigure:
    """Host-verified local figure (0.13.4): one `![alt](target)`.
    The Python layer resolves `target` against the manuscript dir,
    checks existence + format, and records the verdict; the Rust
    core trusts these entries (it never touches the filesystem).
    Pre-fill entries to register bridge downloads explicitly."""
    target: str  # raw `(...)` target as written (match key)
    path: str = ""  # resolved path to embed (absolute posix)
    page: int = 0  # `#page=N` fragment (0 = none)
    embed: bool = False  # host verdict: float, or alt-text + note
    note: str = ""  # human reason when embed is false


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
    # M7.10 full author front matter: non-empty `authors` replaces the
    # scalar above in the woven head; `author_affils[i]` are 0-based
    # indices into `affiliations` (shorter = trailing authors unmarked).
    authors: list[str] = field(default_factory=list)
    affiliations: list[str] = field(default_factory=list)
    author_affils: list[list[int]] = field(default_factory=list)
    # opt-in collapse: > 0 composes the first N names + ", et al."
    authors_max: int = 0
    bib_name: str = "refs-paper"
    graphicspaths: tuple[str, ...] = ("./", "figs/", "figs/tikz/")
    bib_keys: set = field(default_factory=set)
    preamble_override: str = ""
    preamble_append: list[str] = field(default_factory=list)
    skeleton: str = ""  # level-3 whole-document content; exclusive with preamble_override
    bib_style: str = "IEEEtran"
    plot_attrs: dict[str, str] = field(default_factory=dict)
    # Local-figure policy (0.13.4): explicit base dir for `![alt](path)`
    # resolution (default: the md file's parent, else CWD).
    figure_dir: str = ""
    # Host-verified local figures (bridge downloads go here).
    local_figures: list[LocalFigure] = field(default_factory=list)

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
    # Weave-time figure auto-adaptations, in fence order (empty
    # when nothing adapted). The CLI prints these to stderr.
    notes: list[str] = field(default_factory=list)


def spec_from_dict(raw: dict) -> BuildSpec:
    """BuildSpec from a plain dict (CLI --spec JSON, MCP spec_json).
    Unknown keys are ignored; diagrams accept entry dicts."""
    raw = dict(raw)
    diagrams = [
        d if isinstance(d, DiagramEntry) else DiagramEntry(**d)
        for d in raw.pop("diagrams", [])
    ]
    local_figures = [
        f if isinstance(f, LocalFigure) else LocalFigure(**f)
        for f in raw.pop("local_figures", [])
    ]
    raw["local_figures"] = local_figures
    known = set(BuildSpec.__dataclass_fields__)
    raw = {k: v for k, v in raw.items() if k in known}
    if isinstance(raw.get("bib_keys"), list):
        raw["bib_keys"] = set(raw["bib_keys"])
    return BuildSpec(diagrams=diagrams, **raw)


_IMG_RE = _re.compile(r"!\[([^\]]*)\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
_FRAG_RE = _re.compile(r"#page=(\d+)$", _re.IGNORECASE)
_REMOTE_RE = _re.compile(r"(?i)^(https?://|mailto:|data:|#)")
_EMBED_EXTS = {".pdf", ".png", ".jpg", ".jpeg"}
_CONVERT_HINT = {
    ".gif": "convert GIF to PNG and re-run",
    ".svg": "convert SVG to PDF and re-run",
    ".tif": "convert to PNG and re-run",
    ".tiff": "convert to PNG and re-run",
    ".bmp": "convert to PNG and re-run",
}
_PAGE_RE = _re.compile(rb"/Type\s*/Page([^sA-Za-z]|$)")
_PAGES_RE = _re.compile(rb"/Type\s*/Pages([^A-Za-z]|$)")


def _pdf_page_count(path: Path) -> int | None:
    """Conservative PDF page count (stdlib only): counts page dicts
    in raw objects plus zlib-inflated streams (modern PDFs compress
    them), with exactly-one catalog root as a sanity gate. Returns
    None when not evidently a single- or multi-page PDF — callers
    treat unknown as unembeddable (never guess at content)."""
    try:
        data = path.read_bytes()
    except OSError:
        return None
    if not data.startswith(b"%PDF"):
        return None
    n = len(_PAGE_RE.findall(data))
    if n == 1 and len(_PAGES_RE.findall(data)) == 1:
        return 1
    for m in _re.finditer(rb"stream\r?\n(.*?)endstream", data, _re.S):
        for blob in (m.group(1), m.group(1).rstrip(b"\r\n")):
            try:
                dec = _zlib.decompress(blob)
            except Exception:
                continue
            n += len(_PAGE_RE.findall(dec))
            break
    if n >= 1 and len(_PAGES_RE.findall(data)) == 1:
        return n
    return None


def _resolve_local_figures(md_text: str, base: Path) -> list[LocalFigure]:
    """Local-figure policy (0.13.4): per `![alt](target)` target —
    remote/data/mailto untouched (alt-text path); local candidates
    resolved against `base`, verified (existence + PDF/PNG/JPG),
    PDF page-counted (multi-page or unverifiable never embeds:
    bare includegraphics would take page 1 silently); `#page=N`
    fragments are explicit intent and embed with the page option."""
    out: list[LocalFigure] = []
    seen: set[str] = set()
    for m in _IMG_RE.finditer(md_text):
        target = m.group(2)
        if target in seen or _REMOTE_RE.match(target):
            continue
        seen.add(target)
        frag = _FRAG_RE.search(target)
        page = int(frag.group(1)) if frag else 0
        name = target[:frag.start()] if frag else target
        p = base / name
        if not p.is_file():
            out.append(LocalFigure(target, p.as_posix(), 0, False,
                f"'{target}' not found (searched {base.as_posix()}); rendered as alt-text"))
            continue
        ext = p.suffix.lower()
        if ext not in _EMBED_EXTS:
            hint = _CONVERT_HINT.get(ext, "unsupported format for pdflatex")
            out.append(LocalFigure(target, p.as_posix(), 0, False,
                f"'{target}' not embedded: pdflatex needs PDF/PNG/JPG ({hint})"))
            continue
        if ext == ".pdf" and not page:
            n = _pdf_page_count(p)
            if n is None:
                out.append(LocalFigure(target, p.as_posix(), 0, False,
                    f"'{target}' page count unverifiable; not embedded (register it explicitly to include)"))
                continue
            if n > 1:
                out.append(LocalFigure(target, p.as_posix(), 0, False,
                    f"'{target}' has {n} pages; not embedded (add #page=N to select, or register a single-page graphic)"))
                continue
        out.append(LocalFigure(target, p.as_posix(), page, True, ""))
    return out


def _with_local_figures(spec: BuildSpec, md_text: str, base: Path) -> BuildSpec:
    """Inject host-verified local figures (pre-filled bridge entries
    win; auto-resolution covers the rest). Pure-string callers get
    CWD-relative resolution — still verified, still noted."""
    existing = {f.target for f in spec.local_figures}
    fresh = [f for f in _resolve_local_figures(md_text, base) if f.target not in existing]
    if not fresh:
        return spec
    return replace(spec, local_figures=[*spec.local_figures, *fresh])


def build_document(md_text: str, toml_src: str, spec: BuildSpec) -> BuildResult:
    # Absolute base: embedded paths must compile from any CWD.
    base = Path(spec.figure_dir) if spec.figure_dir else Path(".")
    spec = _with_local_figures(spec, md_text, base.resolve())
    r = json.loads(_build_document(md_text, toml_src, spec.to_json()))
    return BuildResult(r["title"], r["body"], r["tex"], r["n_diagrams"], r["n_tables"],
                       r.get("notes", []))


def read_template(toml_dir: str | Path, toml_src: str) -> tuple[str, list[str]]:
    """Resolve `[template]` keys: read `preamble_file` relative to the
    toml directory, split `preamble_append` lines. Returns
    `(override_content, append_lines)`; empty when unconfigured."""
    cfg = _parse_config(toml_src)
    rel = cfg.get("template_preamble_file", "")
    override = (Path(toml_dir) / rel).read_text(encoding="utf-8") if rel else ""
    raw = cfg.get("template_preamble_append", "")
    return override, raw.splitlines() if raw else []


def read_skeleton(toml_dir: str | Path, toml_src: str) -> str:
    """Resolve `[template] skeleton` relative to the toml directory."""
    rel = _parse_config(toml_src).get("template_skeleton", "")
    return (Path(toml_dir) / rel).read_text(encoding="utf-8") if rel else ""


def build_from_paths(md_path: str | Path, toml_path: str | Path, spec: BuildSpec) -> BuildResult:
    """Path-based build: reads md + toml, resolves `[template]` files
    relative to the toml, and weaves. `spec.preamble_*` already set
    take precedence over toml values."""
    toml_path = Path(toml_path)
    toml_src = toml_path.read_text(encoding="utf-8")
    override, append = read_template(toml_path.parent, toml_src)
    skel = read_skeleton(toml_path.parent, toml_src)
    if skel and (override or spec.preamble_override):
        raise ValueError("[template] skeleton is mutually exclusive with preamble_file")
    # Head attribution for template/skeleton errors: the actual
    # file the head text was read from. Inline spec strings stay
    # md-attributed (no file to point at).
    inline_head = bool(spec.preamble_override or spec.skeleton)
    spec = replace(
        spec,
        preamble_override=spec.preamble_override or override,
        preamble_append=[*spec.preamble_append, *append],
        skeleton=spec.skeleton or skel,
    )
    md_text = Path(md_path).read_text(encoding="utf-8")
    base = Path(spec.figure_dir) if spec.figure_dir else Path(md_path).parent
    spec = _with_local_figures(spec, md_text, base.resolve())
    head_path = None
    if not inline_head:
        cfg = _parse_config(toml_src)
        head_rel = cfg.get("template_preamble_file", "") or cfg.get("template_skeleton", "")
        head_path = toml_path.parent / head_rel if head_rel else None
    return with_file(md_path, head_path, build_document, md_text, toml_src, spec)


def _templates_dir() -> Path:
    """Shipped `*-head.tex` library: installed package data first,
    source tree fallback (covers `maturin develop`)."""
    try:
        from importlib.resources import files
        d = files("ikat") / "templates"
        if d.is_dir():
            return Path(str(d))
    except (ImportError, ModuleNotFoundError, TypeError):
        pass
    return Path(__file__).parent / "templates"


def list_templates() -> list[str]:
    """Names of shipped preamble heads (`arxiv`, `acm-sigconf`, …)."""
    d = _templates_dir()
    return sorted(p.stem.removesuffix("-head") for p in d.glob("*-head.tex")) if d.is_dir() else []


def list_skeletons() -> list[str]:
    """Names of shipped whole-document skeletons (`skeleton-plain`)."""
    d = _templates_dir()
    return sorted(p.stem for p in d.glob("skeleton-*.tex")) if d.is_dir() else []


def template_path(name: str) -> Path:
    """Path to a shipped head, e.g. `template_path("arxiv")`, or a
    shipped skeleton (`template_path("skeleton-plain")`). Raises
    `FileNotFoundError` naming what is available."""
    d = _templates_dir()
    for cand in (d / f"{name}-head.tex", d / f"{name}.tex"):
        if cand.is_file() and (cand.stem.endswith("-head") or cand.stem.startswith("skeleton-")):
            return cand
    raise FileNotFoundError(f"no ikat template {name!r}; have: {list_templates() + list_skeletons()}")


#: Per-template companion settings: bibliography style plus the
#: `ikat.toml` snippet the head's comments also document. `spans`
#: notes where a head constrains float placement (3-column).
TEMPLATE_PRESETS: dict[str, dict[str, str]] = {
    "arxiv": {"bib_style": "IEEEtran", "toml": ""},
    "article-1col": {"bib_style": "IEEEtran", "toml": ""},
    "article-2col": {"bib_style": "IEEEtran", "toml": ""},
    "article-3col": {
        "bib_style": "IEEEtran",
        "toml": '[spans]\ndiagram = "column"\nplot = "column"\ntable = "column"\ndefault = "column"\n',
    },
    "ieee-conference": {"bib_style": "IEEEtran", "toml": ""},
    "acm-sigconf": {
        "bib_style": "ACM-Reference-Format",
        "toml": "[template]\nabstract_before_maketitle = true\n",
    },
    "springer-llncs": {"bib_style": "splncs04", "toml": ""},
    "elsevier": {
        "bib_style": "elsarticle-num",
        "toml": "[template]\nabstract_before_maketitle = true\n",
    },
    "aps": {
        "bib_style": "apsrev4-2",
        "toml": "[template]\nabstract_before_maketitle = true\n",
    },
}


def template_preset(name: str) -> dict[str, str]:
    """Companion settings for a shipped template (KeyError if unknown)."""
    return TEMPLATE_PRESETS[name]


def bib_keys(bib_src: str) -> set:
    return set(_bib_keys(bib_src))


def bib_safe(bib_src: str) -> str:
    return _bib_safe(bib_src)
