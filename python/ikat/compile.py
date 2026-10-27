"""PDF driver: run pdflatex/bibtex over a woven tree, like arXiv does.

arXiv recompiles TeX source with its own TeX Live; this driver does
the same locally so what-you-see is what-they-build: `pdflatex`,
`bibtex` (only when a `.bib` is staged next to the main file),
`pdflatex`, `pdflatex`.
"""

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path


class CompileError(RuntimeError):
    pass


def _run(exe: str, args: list[str], workdir: Path) -> str:
    try:
        p = subprocess.run(
            [exe, *args],
            cwd=workdir,
            capture_output=True,
            text=True,
            timeout=600,
        )
    except FileNotFoundError as e:
        raise CompileError(
            f"{exe} not on PATH - install a TeX distribution"
            " (TinyTeX: https://yihui.org/tinytex) and run"
            " `ikat check --install` for the packages"
        ) from e
    if p.returncode != 0:
        tail = (p.stdout + p.stderr)[-2000:]
        raise CompileError(f"{exe} failed in {workdir}:\n{tail}")
    return p.stdout


def tex_for_tectonic(tex_source: str) -> str:
    """Adapt woven `.tex` for the Tectonic/XeTeX engine: drop
    `\\usepackage[utf8]{inputenc}` (XeTeX is UTF-8 native and
    errors on it); everything else passes through."""
    return "\n".join(
        line for line in tex_source.splitlines()
        if "\\usepackage[utf8]{inputenc}" not in line
    ) + "\n"


def _compile_tectonic(workdir: str | Path, main_tex: str) -> Path:
    """Embedded binding first (`tectonic` cargo feature), else the
    `tectonic` binary (`TECTONIC_EXE` or PATH). Either way the
    inputenc line is dropped (XeTeX is UTF-8 native)."""
    workdir = Path(workdir)
    try:
        from ._core import compile_tectonic_pdf
    except ImportError:
        compile_tectonic_pdf = None
    import os

    exe = os.environ.get("TECTONIC_EXE") or shutil.which("tectonic")
    if compile_tectonic_pdf is None and exe is None:
        raise CompileError(
            "tectonic engine needs the `tectonic` binary (TECTONIC_EXE or PATH) "
            "or an ikat built with the `tectonic` cargo feature"
        )
    tex = tex_for_tectonic((workdir / main_tex).read_text(encoding="utf-8"))
    if compile_tectonic_pdf is not None:
        pdf = workdir / f"{Path(main_tex).stem}-tectonic.pdf"
        pdf.write_bytes(bytes(compile_tectonic_pdf(tex)))
        return pdf
    src = workdir / f"{Path(main_tex).stem}-tectonic.tex"
    src.write_text(tex, encoding="utf-8")  # adapted copy; source untouched
    # `--keep-logs`: tectonic otherwise swallows the XeTeX log and
    # undefined citations would pass silently. The kept `.log` lets
    # the CI gate grep the same patterns as for pdflatex; the
    # stdout transcript lands beside it as a run record.
    out = _run(exe, ["--keep-logs", src.name], workdir)
    log = workdir / f"{Path(main_tex).stem}-tectonic-stdout.log"
    log.write_text(out, encoding="utf-8")
    pdf = workdir / f"{Path(main_tex).stem}-tectonic.pdf"
    if not pdf.exists():
        raise CompileError(f"no PDF produced for {main_tex}")
    return pdf


def _compile_texres(workdir: str | Path, main_tex: str) -> Path:
    """texres binary (`TEXRES_EXE` or PATH): a TeX distribution in
    one Rust executable (bundled formats, packages, fonts, BibTeX).
    Runs LaTeX + BibTeX as needed to convergence and writes the PDF
    next to the source; aux state lives in texres's own cache dir.
    The engine (pdfLaTeX/XeLaTeX/LuaLaTeX) is picked from the
    preamble, so no inputenc adaptation is needed; `ensure_packages`
    is a no-op here — everything ships bundled."""
    import os

    exe = os.environ.get("TEXRES_EXE") or shutil.which("texres")
    if exe is None:
        raise CompileError(
            "texres engine needs the texres binary (TEXRES_EXE or PATH) - "
            "releases: https://github.com/leoliu0/texres"
        )
    workdir = Path(workdir)
    out = _run(exe, [main_tex], workdir)
    stem = Path(main_tex).stem
    # stdout transcript kept beside the PDF: the CI gate greps the
    # same undefined-citation patterns as for pdflatex.
    (workdir / f"{stem}-texres-stdout.log").write_text(out, encoding="utf-8")
    pdf = workdir / f"{stem}.pdf"
    if not pdf.exists():
        raise CompileError(f"no PDF produced for {main_tex}")
    return pdf


def compile_pdf(workdir: str | Path, main_tex: str, ensure_packages: bool = False,
                engine: str = "pdflatex") -> Path:
    """Compile `main_tex` inside `workdir`; return the PDF path.

    With `ensure_packages=True`, probe the preamble first and `tlmgr
    install` what's missing (the texliveonfly trick); raises
    `CompileError` with a fix-it hint if packages are still missing.

    `engine="tectonic"` routes through the embedded XeTeX engine
    (prototype: needs an ikat built with the `tectonic` cargo
    feature; inputenc line dropped automatically, no `.bib` run).
    """
    if engine == "tectonic":
        return _compile_tectonic(workdir, main_tex)
    if engine == "texres":
        return _compile_texres(workdir, main_tex)
    if engine != "pdflatex":
        raise CompileError(f"unknown engine {engine!r} (pdflatex|tectonic|texres)")
    if shutil.which("pdflatex") is None:
        raise CompileError(
            "pdflatex not on PATH - install a TeX distribution"
            " (TinyTeX: https://yihui.org/tinytex), weave to .tex with"
            " --no-pdf, or use engine='tectonic' or engine='texres'"
            " with the respective binary on PATH"
        )
    workdir = Path(workdir)
    if ensure_packages:
        from .texenv import ensure as _ensure

        rep = _ensure((workdir / main_tex).read_text(encoding="utf-8"))
        if not rep["ok"]:
            raise CompileError(
                f"missing TeX packages for {main_tex}: "
                f"{rep['missing']} {rep['unprobed']} {rep['hint']}"
            )
    stem = Path(main_tex).stem
    has_bib = any(workdir.glob("*.bib"))
    _run("pdflatex", ["-halt-on-error", "-interaction=nonstopmode", main_tex], workdir)
    if has_bib and shutil.which("bibtex") is not None:
        _run("bibtex", [stem], workdir)
        _run("pdflatex", ["-halt-on-error", "-interaction=nonstopmode", main_tex], workdir)
    _run("pdflatex", ["-halt-on-error", "-interaction=nonstopmode", main_tex], workdir)
    pdf = workdir / f"{stem}.pdf"
    if not pdf.exists():
        raise CompileError(f"no PDF produced for {main_tex}")
    return pdf
