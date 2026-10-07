"""ikat command line: the same operations as the Python API and MCP tools.

    ikat build MD [--toml T] [--outdir D] [--spec S.json] [--bib-style B]
                 [--ensure-packages] [--no-pdf]
    ikat weave MD [--toml T] [--spec S.json]     # .tex to stdout
    ikat check FILE.tex [--install]             # missing TeX packages
    ikat templates | ikat template NAME [--show-preset]
    ikat flowchart [FILE|-]                     # mermaid -> tikzpicture
    ikat version
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

import ikat
from ikat import BuildSpec, DiagramEntry, spec_from_dict


def _spec_from_json(path: str | None) -> BuildSpec:
    if not path:
        return BuildSpec()
    return spec_from_dict(json.loads(Path(path).read_text(encoding="utf-8")))


def _read_md_toml(md_path: str, toml_path: str | None) -> tuple[str, str, Path | None]:
    md = Path(md_path).read_text(encoding="utf-8")
    if toml_path:
        tp = Path(toml_path)
        return md, tp.read_text(encoding="utf-8"), tp.parent
    return md, "", None


def _apply_bib(spec, bib_path: str | None) -> None:
    """`--bib refs.bib`: derive the cited-key set from the bibliography
    instead of freezing it into the spec (no drift by construction)."""
    if bib_path:
        from ikat import bib_keys

        spec.bib_keys = bib_keys(Path(bib_path).read_text(encoding="utf-8"))


def cmd_build(a: argparse.Namespace) -> int:
    from ikat import build_document, build_from_paths, compile_pdf

    spec = _spec_from_json(a.spec)
    if a.bib_style:
        spec.bib_style = a.bib_style
    _apply_bib(spec, a.bib)
    outdir = Path(a.outdir or ".")
    outdir.mkdir(parents=True, exist_ok=True)
    stem = Path(a.md).stem
    if a.toml:
        r = build_from_paths(a.md, a.toml, spec)
    else:
        md, toml_src, _ = _read_md_toml(a.md, None)
        r = build_document(md, toml_src, spec)
    tex_path = outdir / f"{stem}.tex"
    tex_path.write_text(r.tex, encoding="utf-8")
    print(f"tex: {tex_path} (diagrams={r.n_diagrams} tables={r.n_tables})")
    if a.no_pdf:
        return 0
    try:
        pdf = compile_pdf(outdir, tex_path.name, ensure_packages=a.ensure_packages)
    except Exception as e:
        print(f"compile failed: {e}", file=sys.stderr)
        return 1
    print(f"pdf: {pdf} ({pdf.stat().st_size} bytes)")
    return 0


def cmd_weave(a: argparse.Namespace) -> int:
    from ikat import build_document, build_from_paths

    spec = _spec_from_json(a.spec)
    _apply_bib(spec, a.bib)
    if a.toml:
        r = build_from_paths(a.md, a.toml, spec)
    else:
        md, toml_src, _ = _read_md_toml(a.md, None)
        r = build_document(md, toml_src, spec)
    sys.stdout.write(r.tex + ("\n" if not r.tex.endswith("\n") else ""))
    return 0


def cmd_check(a: argparse.Namespace) -> int:
    from ikat import check_tex_env, ensure_tex_packages

    tex = Path(a.tex).read_text(encoding="utf-8")
    rep = ensure_tex_packages(tex) if a.install else check_tex_env(tex)
    print(json.dumps({k: v for k, v in rep.items() if k != "needs"}, indent=1))
    if rep.get("needs") is not None and a.verbose:
        print("needs:", rep["needs"])
    return 0 if rep["ok"] else 1


def cmd_templates(a: argparse.Namespace) -> int:
    from ikat import list_templates

    for name in list_templates():
        print(name)
    return 0


def cmd_template(a: argparse.Namespace) -> int:
    from ikat import template_path, template_preset

    try:
        p = template_path(a.name)
    except FileNotFoundError as e:
        print(e, file=sys.stderr)
        return 1
    sys.stdout.write(p.read_text(encoding="utf-8"))
    if a.show_preset:
        preset = template_preset(a.name)
        print(f"\n% --- preset: bib_style={preset['bib_style']} ---")
        if preset["toml"]:
            print(f"% --- toml ---\n{preset['toml']}", end="")
    return 0


def cmd_flowchart(a: argparse.Namespace) -> int:
    from ikat import flowchart_to_tikz

    src = sys.stdin.read() if a.file == "-" else Path(a.file).read_text(encoding="utf-8")
    try:
        sys.stdout.write(flowchart_to_tikz(src) + "\n")
    except ValueError as e:
        print(f"ikat flowchart: {e}", file=sys.stderr)
        return 1
    return 0


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="ikat", description="Weave Markdown into camera-ready LaTeX.")
    sub = p.add_subparsers(dest="cmd", required=True)

    b = sub.add_parser("build", help="md -> .tex -> .pdf in one go")
    b.add_argument("md")
    b.add_argument("--toml", default=None)
    b.add_argument("--outdir", default=None)
    b.add_argument("--spec", default=None, help="BuildSpec JSON (diagrams/plots/captions/...)")
    b.add_argument("--bib", default=None, help=".bib file: cite only its keys (else all keys pass)")
    b.add_argument("--bib-style", default=None)
    b.add_argument("--ensure-packages", action="store_true")
    b.add_argument("--no-pdf", action="store_true", help="stop after .tex (no TeX needed)")
    b.set_defaults(func=cmd_build)

    w = sub.add_parser("weave", help="md -> .tex on stdout (no TeX needed)")
    w.add_argument("md")
    w.add_argument("--toml", default=None)
    w.add_argument("--spec", default=None)
    w.add_argument("--bib", default=None, help=".bib file: cite only its keys (else all keys pass)")
    w.set_defaults(func=cmd_weave)

    c = sub.add_parser("check", help="report missing TeX packages for a .tex file")
    c.add_argument("tex")
    c.add_argument("--install", action="store_true", help="tlmgr install what's missing")
    c.add_argument("--verbose", action="store_true")
    c.set_defaults(func=cmd_check)

    t = sub.add_parser("templates", help="list shipped preamble heads")
    t.set_defaults(func=cmd_templates)

    g = sub.add_parser("template", help="print a shipped preamble head")
    g.add_argument("name")
    g.add_argument("--show-preset", action="store_true")
    g.set_defaults(func=cmd_template)

    f = sub.add_parser("flowchart", help="mermaid flowchart -> tikzpicture on stdout")
    f.add_argument("file", help="file or - for stdin")
    f.set_defaults(func=cmd_flowchart)

    v = sub.add_parser("version", help="print ikat version")
    v.set_defaults(func=lambda a: (print(ikat.__version__), 0)[1])
    return p


def main(argv: list[str] | None = None) -> int:
    a = build_parser().parse_args(argv)
    try:
        return a.func(a)
    except (ValueError, FileNotFoundError, TypeError) as e:
        print(f"ikat {a.cmd}: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
