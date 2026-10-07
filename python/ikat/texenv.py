"""TeX environment: probe installed packages, install what's missing.

The texliveonfly trick, split cleanly: Rust (`_core`) scans any
`.tex` preamble into `(probe file, tlmgr package)` needs; this module
owns the subprocesses — one `kpsewhich` call to probe, one `tlmgr
install` to fix — and returns reports instead of raising, so callers
decide how fatal a missing package is.
"""

from __future__ import annotations

import shutil
import subprocess

from ._core import document_class, package_needs, used_packages
from ._core import tex_extra_packages as _extra_pkgs

MANUAL_HINT = (
    "tlmgr is unavailable or failed. Options: install the package from "
    "a TeX Live archive (unpack systems/texlive/tlnet/archive/<pkg>.tar.xz, "
    "copy its tex/ tree into the texmf-local tree, run mktexlsr); "
    "or on Windows use MiKTeX Portable, which auto-installs missing "
    "packages at compile time."
)


def needs(tex_source: str) -> list[tuple[str, str]]:
    """`(probe file, tlmgr package)` pairs a `.tex` source requires."""
    cls = document_class(tex_source)
    req = package_needs(used_packages(tex_source), cls)
    for line in _extra_pkgs(tex_source):
        # "\usepackage{float}" -> ("float.sty", "float")
        name = line.split("{", 1)[1].split("}", 1)[0]
        pair = (f"{name}.sty", name)
        if pair not in req:
            req.append(pair)
    return req


def probe(files: list[str]) -> dict[str, bool | None]:
    """Map each file to True/False via `kpsewhich`, None if unprobable.

    One subprocess for the whole list: kpsewhich prints a path per
    file it finds, so anything not echoed back is missing.
    """
    if shutil.which("kpsewhich") is None:
        return {f: None for f in files}
    p = subprocess.run(
        ["kpsewhich", *files], capture_output=True, text=True, timeout=120
    )
    found = {line.rsplit("/", 1)[-1].rsplit("\\", 1)[-1] for line in p.stdout.splitlines()}
    return {f: (f in found) for f in files}


def check(tex_source: str) -> dict:
    """Report what's needed, missing, or unprobable (no installing)."""
    req = needs(tex_source)
    probed = probe([f for f, _ in req])
    missing = [f for f, _ in req if probed[f] is False]
    unknown = [f for f, _ in req if probed[f] is None]
    return {"needs": req, "missing": missing, "unprobed": unknown,
            "ok": not missing and not unknown}


def ensure(tex_source: str, install: bool = True) -> dict:
    """Probe, and `tlmgr install` what's missing. Returns a report.

    Never raises for missing packages (returns `ok: False` + `hint`);
    raises only when asked to install but no `tlmgr` exists.
    """
    rep = check(tex_source)
    rep["installed_now"] = []
    if not rep["missing"] or not install:
        rep["hint"] = "" if rep["ok"] else MANUAL_HINT
        return rep
    if shutil.which("tlmgr") is None:
        rep["hint"] = "tlmgr not on PATH. " + MANUAL_HINT
        return rep
    tlmgr_pkgs = sorted({pkg for f, pkg in rep["needs"] if f in rep["missing"]})
    p = subprocess.run(
        ["tlmgr", "install", *tlmgr_pkgs],
        capture_output=True, text=True, timeout=900,
    )
    if p.returncode == 0:
        was_missing = set(rep["missing"])
        rep = check(tex_source)
        rep["installed_now"] = sorted(f for f in was_missing if f not in rep["missing"])
        rep["hint"] = "" if rep["ok"] else MANUAL_HINT
    else:
        tail = (p.stdout + p.stderr)[-800:]
        rep["hint"] = f"tlmgr failed: {tail} " + MANUAL_HINT
    return rep
