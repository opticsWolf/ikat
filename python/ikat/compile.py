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
        raise CompileError(f"{exe} not on PATH") from e
    if p.returncode != 0:
        tail = (p.stdout + p.stderr)[-2000:]
        raise CompileError(f"{exe} failed in {workdir}:\n{tail}")
    return p.stdout


def compile_pdf(workdir: str | Path, main_tex: str) -> Path:
    """Compile `main_tex` inside `workdir`; return the PDF path."""
    if shutil.which("pdflatex") is None:
        raise CompileError("pdflatex not on PATH")
    workdir = Path(workdir)
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
