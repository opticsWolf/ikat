# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Stack

Static HTML/CSS/JavaScript; GitHub Pages deployment through GitHub Actions (confirmed by the user).

## Users

Inferred from the repository's README and showcase: academic authors and researchers who write technical papers in Markdown and need a LaTeX/PDF output. The primary action for this site is to install and try ikat (confirmed by the user).

## Product Purpose

Ikat converts Markdown manuscripts and figure data into LaTeX and PDFs, with a Rust core and thin Python wrappers. Success is a visitor understanding what it does and finding a concrete path to install and try it.

## Positioning

Figures are carried as data or editable diagram source and emitted as TikZ/pgfplots during document build, rather than pasted screenshots; the generated figures use the document's TeX fonts. Source: README and `examples/ikat-paper/`.

## Operating Context

Users author Markdown, configuration, citation data, and figure inputs; build with the Python CLI/API and a local TeX installation when PDF output is required. `ikat weave` can emit TeX without a TeX installation. The website's primary workflow is install/try, with repository documentation and the showcase as supporting material.

## Capabilities and Constraints

The repository documents Markdown-to-LaTeX/PDF builds, Mermaid flowchart-to-TikZ, direct pgfplots bar/line chart emitters, per-element float configuration, templates and whole-document skeletons, package checks, a CLI, and an optional MCP server. PDF compilation uses an installed TeX toolchain; TinyTeX/pdfTeX is the documented default and Tectonic is optional. Do not imply unsupported Mermaid syntax or fabricate performance results. As of this record, publication to PyPI/crates.io is not confirmed; the release workflow requires repository secrets and a matching version tag.

## Brand Commitments

The product is named `ikat`, after the textile patterning technique. For this website, the user requires a light-colored background and strongly contrasting colors.

## Evidence on Hand

- `README.md` and `docs/QUICKREF.md`: product description, supported interfaces, usage, and documented requirements.
- `examples/ikat-paper/`: a showcase manuscript and build script exercising the pipeline; generated build outputs are local/ignored.
- No customer testimonials or independently measured performance claims are supplied. Do not invent either.

## Product Principles

- Keep Markdown, diagrams, and plot data editable in source.
- Generate figures as vector TeX-native output rather than screenshots.
- Let Rust own parsing/layout/code generation and keep Python wrappers thin.
- Fail fast on unsupported syntax and configuration.
- State TeX and release prerequisites honestly.
