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
from collections.abc import Iterator
from importlib.resources import files

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
    for line in match.group(1).splitlines():
        if not line or line[0].isspace():
            continue
        m = _FIELD.match(line)
        if not m:
            continue
        value = m.group(2).strip()
        if len(value) >= 2 and value[0] == value[-1] and value[0] in "'\"":
            value = value[1:-1]
        fields[m.group(1)] = value
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


__all__ = [
    "SKILL_NAMES",
    "SkillNotFound",
    "iter_skills",
    "skill_description",
    "skill_names",
    "skill_text",
]
