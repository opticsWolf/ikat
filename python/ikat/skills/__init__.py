"""Agent skill packs bundled with ikat (ikat, ikat-api, ikat-cli, ikat-mcp).

The SKILL.md files live inside the package so the wheel ships them;
`mcp_server` serves each as a `skill://ikat*` resource, so an MCP-only
host can read the guides without a repo checkout. The drift tests in
tests/test_surface.py read them through importlib.resources — the same
path an installed release uses — and pin every taught tool/command to
the live surfaces.
"""

from __future__ import annotations

import re
from collections.abc import Iterable, Iterator
from dataclasses import dataclass
from importlib.resources import files
from pathlib import Path

#: Every skill shipped in the wheel, sorted for stable CLI/resource order.
SKILL_NAMES = ("ikat", "ikat-api", "ikat-cli", "ikat-mcp")

_FRONTMATTER = re.compile(r"\A---\r?\n(.*?)\r?\n---[ \t]*(?:\r?\n|\Z)", re.DOTALL)
_FIELD = re.compile(r"^([A-Za-z_][\w-]*):[ \t]*(.*)$")


class SkillNotFound(KeyError):
    """Raised when a name does not identify a bundled skill."""


def skill_names() -> tuple[str, ...]:
    """Names of the skill packs this distribution ships."""
    return SKILL_NAMES


def skill_text(name: str) -> str:
    """Return one bundled SKILL.md verbatim."""
    if name not in SKILL_NAMES:
        raise SkillNotFound(
            f"unknown skill {name!r}; available: {', '.join(SKILL_NAMES)}"
        )
    return (files(__name__) / name / "SKILL.md").read_text(encoding="utf-8")


def skill_description(name: str) -> str:
    """Frontmatter trigger description for one skill.

    The description is what a harness reads before loading the full
    file, so a missing one is a packaging bug rather than an empty
    string. A frontmatter `name` that disagrees with the directory is
    likewise a packaging bug.
    """
    text = skill_text(name)
    match = _FRONTMATTER.match(text)
    if not match:
        raise ValueError(f"{name}/SKILL.md has no frontmatter")
    fields: dict[str, str] = {}
    lines = match.group(1).splitlines()
    i = 0
    while i < len(lines):
        line = lines[i]
        i += 1
        if not line or line[0].isspace():
            continue  # stray indented line outside a block scalar
        m = _FIELD.match(line)
        if not m:
            continue
        key, value = m.group(1), m.group(2).strip()
        if len(value) >= 2 and value[0] == value[-1] and value[0] in "'\"":
            value = value[1:-1]
        if value in (">", "|", ">-", "|-"):
            # YAML block scalar: gather the indented continuation
            # lines. `>` folds newlines into spaces, `|` keeps them.
            parts: list[str] = []
            while i < len(lines) and lines[i] and lines[i][0] in " \t":
                parts.append(lines[i].strip())
                i += 1
            value = " ".join(parts) if value.startswith(">") else "\n".join(parts)
        fields[key] = value
    description = fields.get("description", "").strip()
    if not description:
        raise ValueError(f"{name}/SKILL.md has no frontmatter description")
    declared = fields.get("name")
    if declared is not None and declared != name:
        raise ValueError(f"{name}/SKILL.md frontmatter name does not match its directory")
    return description


def iter_skills() -> Iterator[tuple[str, str]]:
    """Yield ``(name, description)`` for every bundled skill."""
    for name in SKILL_NAMES:
        yield name, skill_description(name)


def _selected(names: Iterable[str] | None) -> tuple[str, ...]:
    """Validated, deduplicated skill-name selection (all when None)."""
    if not names:
        return SKILL_NAMES
    chosen: list[str] = []
    for name in names:
        if name not in SKILL_NAMES:
            raise SkillNotFound(
                f"unknown skill {name!r}; available: {', '.join(SKILL_NAMES)}"
            )
        if name not in chosen:
            chosen.append(name)
    return tuple(chosen)


@dataclass(frozen=True)
class InstallResult:
    """One skill copy attempt."""

    path: Path
    status: str  # "installed", "updated" or "skipped"


def default_destinations(*, project: bool = False) -> tuple[Path, ...]:
    """Harness skill directories used when the caller names none.

    Personal installs go to Claude Code, OpenCode, pi's user skills
    directory, and the portable Agent Skills location; ``project=True``
    targets the project-local equivalents. Anything else (a custom
    harness) takes an explicit ``--dest``.
    """
    home = Path.home()
    if project:
        root = Path.cwd()
        return (root / ".claude" / "skills", root / ".opencode" / "skills",
                root / ".agents" / "skills")
    return (home / ".claude" / "skills", home / ".config" / "opencode" / "skills",
            home / ".pi" / "agent" / "skills", home / ".agents" / "skills")


def _copy_tree(source, destination: Path) -> None:
    destination.mkdir(parents=True, exist_ok=True)
    for entry in source.iterdir():
        target = destination / entry.name
        if entry.is_dir():
            _copy_tree(entry, target)
        else:
            target.write_bytes(entry.read_bytes())


def install(
    destination: str | Path,
    *,
    names: Iterable[str] | None = None,
    force: bool = False,
) -> list[InstallResult]:
    """Copy bundled skills into ``destination``.

    Existing skill directories are left alone unless ``force`` is set,
    so a re-run never clobbers local edits by accident.
    """
    dest = Path(destination).expanduser()
    results: list[InstallResult] = []
    for name in _selected(names):
        target = dest / name
        marker = target / "SKILL.md"
        existed = marker.is_file()
        if existed and not force:
            results.append(InstallResult(target, "skipped"))
            continue
        _copy_tree(files(__name__) / name, target)
        results.append(InstallResult(target, "updated" if existed else "installed"))
    return results


__all__ = [
    "SKILL_NAMES",
    "InstallResult",
    "SkillNotFound",
    "default_destinations",
    "install",
    "iter_skills",
    "skill_description",
    "skill_names",
    "skill_text",
]
