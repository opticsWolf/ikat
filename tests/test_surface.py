"""Tests for the unified surface: CLI commands and MCP tool registration.

CLI tests run main(argv) against tmp dirs (no TeX: --no-pdf/weave).
MCP tests call the tool functions directly + assert registration;
skipped when the mcp extra is absent.
"""

import io
import json

import pytest

from ikat import __version__
from ikat.cli import main as cli_main

needs_mcp = pytest.mark.skipif(
    __import__("importlib").util.find_spec("mcp.server.fastmcp") is None,
    reason="mcp extra absent",
)

MD = "# Hi\n\n## 1. Sec\n\nBody text.\n"


def test_weave_stdout(tmp_path, capsys):
    md = tmp_path / "d.md"
    md.write_text(MD, encoding="utf-8")
    assert cli_main(["weave", str(md)]) == 0
    out = capsys.readouterr().out
    assert "\\documentclass[conference]{IEEEtran}" in out
    assert "\\section{Sec}" in out


def test_build_no_pdf(tmp_path, capsys):
    md = tmp_path / "d.md"
    md.write_text(MD, encoding="utf-8")
    out = tmp_path / "out"
    assert cli_main(["build", str(md), "--outdir", str(out), "--no-pdf"]) == 0
    tex = (out / "d.tex").read_text(encoding="utf-8")
    assert "\\maketitle" in tex
    assert "tex:" in capsys.readouterr().out


def test_build_with_toml_and_spec(tmp_path):
    (tmp_path / "ikat.toml").write_text('[template]\npreamble_append = ["% hi"]\n', encoding="utf-8")
    md = tmp_path / "d.md"
    md.write_text(MD, encoding="utf-8")
    spec = tmp_path / "s.json"
    spec.write_text(json.dumps({"bib_style": "plain", "bogus_key": 1}), encoding="utf-8")
    out = tmp_path / "out"
    assert cli_main(["build", str(md), "--toml", str(tmp_path / "ikat.toml"),
                     "--outdir", str(out), "--no-pdf"]) == 0
    assert cli_main(["build", str(md), "--outdir", str(out), "--no-pdf",
                     "--spec", str(spec)]) == 0
    assert "\\bibliographystyle{plain}" in (out / "d.tex").read_text(encoding="utf-8")


def test_check_reports_and_fails(tmp_path, capsys, monkeypatch):
    from ikat import texenv
    tex = tmp_path / "d.tex"
    tex.write_text("\\documentclass{article}\\usepackage{pgfplots}\n", encoding="utf-8")
    monkeypatch.setattr(texenv.shutil, "which", lambda _: None)
    assert cli_main(["check", str(tex)]) == 1
    rep = json.loads(capsys.readouterr().out)
    assert rep["unprobed"] == ["pgfplots.sty", "article.cls"]


def test_templates_commands(tmp_path, capsys):
    assert cli_main(["templates"]) == 0
    names = capsys.readouterr().out.split()
    assert len(names) == 9 and "arxiv" in names
    assert cli_main(["template", "arxiv", "--show-preset"]) == 0
    out = capsys.readouterr().out
    assert "\\documentclass[11pt]{article}" in out
    assert "bib_style=IEEEtran" in out
    assert cli_main(["template", "nope"]) == 1


def test_flowchart_stdin(capsys, monkeypatch):
    import sys
    monkeypatch.setattr(sys, "stdin", __import__("io").StringIO("graph TD\na[x]-->b[y]\n"))
    assert cli_main(["flowchart", "-"]) == 0
    assert "\\begin{tikzpicture}" in capsys.readouterr().out


SEQ = ("sequenceDiagram\nparticipant A\nparticipant B\n"
       "A->>B: hi\nB-->>A: yo\nalt x\nA->>B: r\nend\n")


def test_sequence_through_shared_entry(capsys, monkeypatch):
    import sys
    monkeypatch.setattr(sys, "stdin", __import__("io").StringIO(SEQ))
    assert cli_main(["flowchart", "-"]) == 0
    out = capsys.readouterr().out
    assert "\\draw[dashed]" in out and "rectangle" in out


@needs_mcp
def test_sequence_mcp_parity():
    from ikat import mcp_server

    assert "rectangle" in mcp_server.flowchart_to_tikz(SEQ)


STATE = ("stateDiagram-v2\n[*] --> A\nA --> B : go\n"
         "state Box {\nA --> B\n}\nB --> [*]\n")


def test_state_through_shared_entry(capsys, monkeypatch):
    import sys
    monkeypatch.setattr(sys, "stdin", __import__("io").StringIO(STATE))
    assert cli_main(["flowchart", "-"]) == 0
    out = capsys.readouterr().out
    assert "fill=black" in out and "rectangle" in out


@needs_mcp
def test_state_mcp_parity():
    from ikat import mcp_server

    assert "circle,draw" in mcp_server.flowchart_to_tikz(STATE)


SUB = ("graph TD\nsubgraph ours[Our box]\na[x]-->b[y]\nend\nb-->c[z]\n")


def test_subgraph_cli(capsys, monkeypatch):
    import sys
    monkeypatch.setattr(sys, "stdin", __import__("io").StringIO(SUB))
    assert cli_main(["flowchart", "-"]) == 0
    out = capsys.readouterr().out
    assert "rectangle" in out and "{Our box}" in out


@needs_mcp
def test_subgraph_mcp_parity():
    from ikat import mcp_server

    assert "{Our box}" in mcp_server.flowchart_to_tikz(SUB)


def test_skeleton_commands(capsys):
    assert cli_main(["skeletons"]) == 0
    assert "plain" in capsys.readouterr().out
    assert cli_main(["skeleton", "skeleton-plain"]) == 0
    assert "{{body}}" in capsys.readouterr().out


def test_plot_commands(tmp_path, capsys):
    bar = tmp_path / "bar.json"
    bar.write_text('{"title": "t", "ylabel": "y", "group_labels": ["a"],'
                     ' "series_names": ["s"], "values": [[2.0]], "mins": [[2.0]],'
                     ' "maxs": [[2.0]]}', encoding="utf-8")
    assert cli_main(["barchart", str(bar), "--legend", "below"]) == 0
    out = capsys.readouterr().out
    assert "anchor=north,legend columns=-1" in out
    line = tmp_path / "line.json"
    line.write_text('{"title": "t", "xlabel": "x", "ylabel": "y", "xs": [1.0],'
                      ' "names": ["s"], "yss": [[2.0]], "errs": [[0.0]]}', encoding="utf-8")
    assert cli_main(["lineplot", str(line)]) == 0
    assert "\\begin{tikzpicture}" in capsys.readouterr().out


def test_preset_flags(tmp_path, capsys, monkeypatch):
    loc = "examples/ikat-paper/fig-loc.json"
    assert cli_main(["barchart", "--preset", loc]) == 0
    assert "\\addlegendentry{shipped code}" in capsys.readouterr().out
    # stdin preset
    src = open(loc, encoding="utf-8").read()
    monkeypatch.setattr("sys.stdin", io.StringIO(src))
    assert cli_main(["lineplot", "--preset", "-", "--legend", "below"]) == 0
    assert "anchor=north,legend columns=-1" in capsys.readouterr().out
    # FILE + --preset conflict names both
    assert cli_main(["barchart", str(tmp_path / "x.json"), "--preset", loc]) == 1
    assert "--preset" in capsys.readouterr().err
    # neither FILE nor --preset
    assert cli_main(["barchart"]) == 1


@needs_mcp
def test_mcp_preset_param():
    from ikat import mcp_server

    loc = open("examples/ikat-paper/fig-tests.json", encoding="utf-8").read()
    r = mcp_server.lineplot_to_tikz("t", "x", "y", [1.0], ["s"], [[1.0]], [[0.0]],
                                     preset_json=loc)
    assert "\\addlegendentry{Rust tests}" in r


# ──── Registry: one source of truth for every surface ────


def test_registry_round_trip_and_dispatch_errors():
    from ikat.registry import TOOL_REGISTRY, execute_tool

    assert len(TOOL_REGISTRY) == 14
    names = [spec.name for spec in TOOL_REGISTRY]
    assert len(set(names)) == 14
    # dispatch with defaults + explicit args
    tikz = execute_tool("flowchart_to_tikz", {"src": "graph TD\na[x]-->b[y]\n"})
    assert "\\begin{tikzpicture}" in tikz
    tikz2 = execute_tool("lineplot_to_tikz", {
        "title": "t", "xlabel": "x", "ylabel": "y", "xs": [1.0],
        "names": ["s"], "yss": [[1.0]], "errs": [[0.0]], "legend": "below"})
    assert "anchor=north" in tikz2
    # unknown tool names the valid set
    with pytest.raises(ValueError) as e:
        execute_tool("nope")
    assert "weave_document" in str(e.value)
    # every required param of every spec is first (valid signatures)
    for spec in TOOL_REGISTRY:
        req = [p.required for p in spec.params]
        assert req == sorted(req, key=lambda v: not v), f"{spec.name}: required params must precede optional"


def test_registry_llm_definitions_are_wellformed():
    from ikat.registry import TOOL_REGISTRY, llm_tool_definitions

    defs = llm_tool_definitions()
    assert len(defs) == len(TOOL_REGISTRY)
    by_name = {d["function"]["name"]: d for d in defs}
    legend = by_name["barchart_to_tikz"]["function"]["parameters"]["properties"]["legend"]
    assert legend["enum"][0] == "auto" and "outside-right" in legend["enum"]
    assert legend["description"]
    for d in defs:
        fn = d["function"]
        assert d["type"] == "function" and fn["description"]
        params = fn["parameters"]
        assert set(params["required"]) <= set(params["properties"])
        for prop in params["properties"].values():
            assert prop.get("description"), f"{fn['name']} param lacks description"


@needs_mcp
def test_mcp_server_is_registry_generated():
    import asyncio

    from ikat import mcp_server
    from ikat.registry import TOOL_REGISTRY

    tools = {t.name: t for t in asyncio.run(mcp_server.mcp.list_tools())}
    assert set(tools) == {s.name for s in TOOL_REGISTRY}
    for spec in TOOL_REGISTRY:
        t = tools[spec.name]
        ann = t.annotations
        assert ann.readOnlyHint is spec.readonly
        assert ann.destructiveHint is spec.destructive
        assert ann.openWorldHint is spec.openworld
        assert ann.idempotentHint is spec.idempotent
        schema_props = set((t.inputSchema or {}).get("properties", {}))
        assert schema_props == {p.name for p in spec.params}, f"{spec.name}: schema params drifted"


# ──── Skill installer ────


def test_skills_installer_lifecycle(tmp_path, capsys, monkeypatch):
    from ikat import skills as sk

    dest = tmp_path / "agent-skills"
    monkeypatch.setattr(sk, "default_destinations", lambda *, project=False: (dest,))
    assert cli_main(["skills", "install"]) == 0
    out = capsys.readouterr().out
    assert out.count("installed ") == 4
    assert (dest / "ikat" / "SKILL.md").is_file()
    assert cli_main(["skills", "install"]) == 0  # re-run skips, hint printed
    out = capsys.readouterr().out
    assert out.count("skipped ") == 4 and "--force" in out


def test_skills_install_statuses(tmp_path):
    from ikat.skills import install

    dest = tmp_path / "skills"
    first = install(dest)
    assert [r.status for r in first] == ["installed"] * 4
    again = install(dest)
    assert [r.status for r in again] == ["skipped"] * 4
    forced = install(dest, force=True)
    assert [r.status for r in forced] == ["updated"] * 4
    one = install(dest, names=["ikat-cli"], force=True)
    assert [r.status for r in one] == ["updated"] and one[0].path.name == "ikat-cli"
    # content is verbatim
    from ikat.skills import skill_text
    assert (dest / "ikat-cli" / "SKILL.md").read_text(encoding="utf-8") == skill_text("ikat-cli")


def test_skills_cli_paths(tmp_path, capsys):
    dest = tmp_path / "d"
    assert cli_main(["skills", "install", "--dest", str(dest)]) == 0
    assert (dest / "ikat-api" / "SKILL.md").is_file()
    capsys.readouterr()  # drain the install output
    assert cli_main(["skills", "show", "ikat-mcp"]) == 0
    assert capsys.readouterr().out.startswith("---")
    assert cli_main(["skills", "show", "nope"]) == 1
    assert "unknown skill" in capsys.readouterr().err
    assert cli_main(["skills", "install", "--dest", str(tmp_path / "x"), "--skill", "nope"]) == 1
    assert "unknown skill" in capsys.readouterr().err
    # --dest + --project conflict names both
    assert cli_main(["skills", "install", "--dest", str(dest), "--project"]) == 1
    assert "mutually exclusive" in capsys.readouterr().err


# ──── `ikat help` — argparse help as a verb ────


def test_help_verb_lists_and_details(capsys):
    assert cli_main(["help"]) == 0
    out = capsys.readouterr().out
    for cmd in ("build", "weave", "layout", "check", "flowchart", "skills",
                "standalone", "version", "help"):
        assert cmd in out, f"`ikat help` never lists {cmd}"
    assert cli_main(["help", "build"]) == 0
    assert "--no-pdf" in capsys.readouterr().out
    assert cli_main(["help", "nope"]) == 1
    assert "unknown command" in capsys.readouterr().err
    # bare `ikat`: help, not an argparse error
    assert cli_main([]) == 0
    assert "build" in capsys.readouterr().out
    # --json dumps the agent-tool registry
    assert cli_main(["help", "--json"]) == 0
    defs = json.loads(capsys.readouterr().out)
    assert len(defs) == 14
    legend = [d for d in defs
              if d["function"]["name"] == "barchart_to_tikz"
              ][0]["function"]["parameters"]["properties"]["legend"]
    assert legend["enum"][0] == "auto"


def test_buildspec_authors_roundtrip_and_weave():
    """M7.10: full author front matter rides the spec JSON."""
    from ikat import BuildSpec, build_document

    spec = BuildSpec(
        authors=["Marta Coll", "Other Author"],
        affiliations=["Inst One", "Inst Two"],
        author_affils=[[0], [0, 1]],
    )
    spec2 = BuildSpec()
    spec2.__dict__.update(json.loads(spec.to_json()))
    assert spec2.authors == ["Marta Coll", "Other Author"]
    assert spec2.author_affils == [[0], [0, 1]]
    r = build_document("# T\n\nPara.\n", "", spec)
    assert "\\parbox{0.9\\textwidth}{\\centering Marta Coll\\textsuperscript{1}" in r.tex
    assert "\\textsuperscript{2}Inst Two" in r.tex
    # legacy scalar author: byte-identical legacy line
    r0 = build_document("# T\n\nPara.\n", "", BuildSpec())
    assert "\\author{\\IEEEauthorblockN{opticsWolf}}" in r0.tex
    # out-of-range mark fails fast with the count named
    with pytest.raises(ValueError, match="only 1 affiliations"):
        build_document("# T\n\nPara.\n", "", BuildSpec(
            authors=["A"], affiliations=["I"], author_affils=[[3]]))


def test_cli_author_flags(capsys):
    """`--author` / `--affiliation` / `--author-affil` accumulate."""
    from ikat import BuildSpec
    from ikat.cli import _apply_authors

    spec = BuildSpec()
    ns = type("NS", (), {})()
    ns.author = ["A", "B"]
    ns.affiliation = ["I1", "I2"]
    ns.author_affil = ["1:1", "2:1,2"]
    _apply_authors(spec, ns)
    assert spec.authors == ["A", "B"]
    assert spec.affiliations == ["I1", "I2"]
    assert spec.author_affils == [[0], [0, 1]]
    ns.author_affil = ["bad"]
    with pytest.raises(ValueError, match="1-based"):
        _apply_authors(BuildSpec(), ns)


def test_version(capsys):
    assert cli_main(["version"]) == 0
    assert capsys.readouterr().out.strip() == __version__


# ──── MCP schema fidelity + annotations (gossamer-audit transfer) ────


@needs_mcp
def test_mcp_schemas_carry_descriptions_and_enums():
    import asyncio

    from ikat import mcp_server

    assert mcp_server.mcp.instructions, "server instructions unset"
    tools = {t.name: t for t in asyncio.run(mcp_server.mcp.list_tools())}
    assert len(tools) == 14
    for name, t in tools.items():
        for pname, prop in (t.inputSchema or {}).get("properties", {}).items():
            assert prop.get("description"), f"{name}.{pname} lacks a schema description"
    legend = tools["barchart_to_tikz"].inputSchema["properties"]["legend"]
    assert legend["enum"] == ["auto", "below", "top-left", "top-right",
                              "bottom-left", "bottom-right", "outside-right"]
    assert tools["lineplot_to_tikz"].inputSchema["properties"]["legend"]["enum"] == legend["enum"]


@needs_mcp
def test_mcp_annotations_pure_vs_install():
    import asyncio

    from ikat import mcp_server

    tools = {t.name: t for t in asyncio.run(mcp_server.mcp.list_tools())}
    assert tools["ensure_tex_packages"].annotations.readOnlyHint is False
    assert tools["ensure_tex_packages"].annotations.openWorldHint is True
    for name, t in tools.items():
        if name == "ensure_tex_packages":
            continue
        ann = t.annotations
        assert ann.readOnlyHint is True, f"{name} should be read-only"
        assert ann.destructiveHint is False, f"{name} should be non-destructive"
        assert ann.openWorldHint is False, f"{name} should be network-free"


@needs_mcp
def test_mcp_skill_resources():
    import asyncio

    from ikat import mcp_server

    resources = {str(r.uri): r for r in asyncio.run(mcp_server.mcp.list_resources())}
    assert set(resources) == {"skill://ikat", "skill://ikat-api",
                              "skill://ikat-cli", "skill://ikat-mcp"}
    result = asyncio.run(mcp_server.mcp.read_resource("skill://ikat-cli"))
    from ikat.skills import skill_text

    assert result[0].content == skill_text("ikat-cli")  # verbatim bundle


# ──── Skill drift: the bundled SKILL.md files cannot rot ────


def _commands_taught(skill_text: str) -> set[str]:
    """`ikat CMD` subcommands taught inside fenced code blocks."""
    import re

    fences = re.findall(r"```(?:bash|sh)?\n(.*?)```", skill_text, re.DOTALL)
    taught = set()
    for fence in fences:
        for line in fence.splitlines():
            for piece in re.split(r"\||&&|;", line):
                piece = piece.strip()
                if piece.startswith("ikat "):
                    taught.add(piece.split()[1])
    return taught


def _flags_taught(skill_text: str) -> dict[str, set[str]]:
    """Per-command `--flags` taught inside fenced code blocks.

    A piece starting with `ikat CMD` rebinds the current command;
    pipe-segments without `ikat` (`show NAME`, `install [--dest …]`)
    and indented continuation lines belong to the current command.
    """
    import re

    fences = re.findall(r"```(?:bash|sh)?\n(.*?)```", skill_text, re.DOTALL)
    taught: dict[str, set[str]] = {}
    cur = None
    for fence in fences:
        for line in fence.splitlines():
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue
            for piece in re.split(r"\||&&|;", stripped):
                piece = piece.strip()
                m = re.match(r"ikat ([a-z][a-z-]*)", piece)
                if m:
                    cur = m.group(1)
                elif not stripped.startswith("ikat") and line[:1] not in (" ", "\t"):
                    continue  # a non-command segment of a non-command line
                taught.setdefault(cur, set())
                taught[cur] |= set(re.findall(r"--([a-z][a-z-]*)", piece))
    return taught


def _real_flags(parser) -> dict[str, set[str]]:
    """Per-command optional flags from the argparse parser itself."""
    subs = parser._subparsers._group_actions[0].choices
    real: dict[str, set[str]] = {}
    for cmd, subp in subs.items():
        flags = set()
        actions = list(subp._actions)
        # nested sub-subparsers (skills list/show/install): union them
        for ga in getattr(subp._subparsers, "_group_actions", []) or []:
            for nested in ga.choices.values():
                actions.extend(nested._actions)
        for a in actions:
            for o in a.option_strings:
                if o not in ("-h", "--help"):
                    flags.add(o.lstrip("-"))
        real[cmd] = flags
    return real


def test_skill_frontmatter_is_valid():
    from ikat.skills import iter_skills

    pairs = dict(iter_skills())  # raises when a frontmatter rots
    assert set(pairs) == {"ikat", "ikat-api", "ikat-cli", "ikat-mcp"}
    assert all(pairs.values())


def test_cli_skill_teaches_real_commands():
    from ikat.cli import build_parser
    from ikat.skills import skill_text

    taught = _commands_taught(skill_text("ikat-cli"))
    real = set(build_parser()._subparsers._group_actions[0].choices)
    phantom = taught - real
    assert not phantom, f"ikat-cli SKILL.md teaches phantom commands: {sorted(phantom)}"
    missing = real - taught
    assert not missing, f"ikat-cli SKILL.md never mentions: {sorted(missing)}"


def test_cli_skill_teaches_real_flags():
    """Every command's flags: none invented, none omitted.

    Gossamer's drift-test model, extended past command names: the
    skill's fenced syntax must match the argparse parser flag by
    flag, so a new CLI flag without a skill update fails here.
    """
    from ikat.cli import build_parser
    from ikat.skills import skill_text

    taught = _flags_taught(skill_text("ikat-cli"))
    real = _real_flags(build_parser())
    for cmd in real:
        got = taught.get(cmd, set())
        phantom = got - real[cmd]
        assert not phantom, f"{cmd}: SKILL.md invents flags {sorted(phantom)}"
        missing = real[cmd] - got
        assert not missing, f"{cmd}: SKILL.md omits flags {sorted(missing)}"


@needs_mcp
def test_mcp_skill_teaches_real_tools():
    import asyncio
    import re

    from ikat import mcp_server
    from ikat.skills import skill_text

    text = skill_text("ikat-mcp")
    named = set(re.findall(r"\b([a-z][a-z_]{3,})\(", text))
    real = {t.name for t in asyncio.run(mcp_server.mcp.list_tools())}
    phantom = {n for n in named
               if n not in real and not hasattr(mcp_server, n) and n not in ikat_exports()}
    assert not phantom, f"ikat-mcp SKILL.md names phantom tools: {sorted(phantom)}"
    for tool in real:
        assert tool in text, f"ikat-mcp SKILL.md never mentions {tool}"


def ikat_exports():
    import ikat

    return set(ikat.__all__)


def test_api_skill_names_real_exports():
    import builtins
    import re

    import ikat
    from ikat.registry import __all__ as registry_exports
    from ikat.skills import __all__ as skills_exports, skill_text

    text = skill_text("ikat-api")
    named = set(re.findall(r"\b([a-z][a-z_]{3,})\(", text))
    valid = set(ikat.__all__) | set(registry_exports) | set(skills_exports)
    phantom = {n for n in named
               if n not in valid and not hasattr(builtins, n)}
    assert not phantom, f"ikat-api SKILL.md names phantom calls: {sorted(phantom)}"


@needs_mcp
def test_mcp_tools_registered():
    from ikat import mcp_server
    import asyncio

    names = sorted(t.name for t in asyncio.run(mcp_server.mcp.list_tools()))
    assert names == ["barchart_to_tikz", "check_tex_packages", "ensure_tex_packages",
                     "float_packages", "flowchart_to_tikz", "get_skeleton", "get_template", "lineplot_to_tikz",
                     "list_skeletons", "list_templates", "standalone_figure", "suggest_layout",
                     "version", "weave_document"]


@needs_mcp
def test_mcp_weave_and_template():
    from ikat import mcp_server

    r = mcp_server.weave_document(MD, "", '{"bib_style": "plain"}')
    assert r["tex"].count("\\documentclass") == 1
    assert "\\bibliographystyle{plain}" in r["tex"]
    t = mcp_server.get_template("aps")
    assert t["bib_style"] == "apsrev4-2"
    assert "\\documentclass" in t["head"]
    assert mcp_server.version() == __version__
