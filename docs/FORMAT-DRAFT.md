# Formatting draft: floats, skeleton level 3, Markdown parsing

Status: M5.1 BUILT 2026-10-07 (sections A/A.1–A.4); B (skeleton)
and C (AST spike) still draft. Nothing here changes defaults.

Conventions today (baseline, all in `figure[t]` / `table[t]`):

- diagrams/plots: float `figure`/`figure*` by span, `\centering`,
  caption BELOW the graphic, `\label` after caption.
- tables: float `table`, `\caption` ABOVE the tabularx (IEEE style),
  `\small`, first row bolded, colspec all-`X` or per-table override.
- pictures (precompiled PDFs/PNGs): `\includegraphics[width=span]`,
  same float/caption rules as diagrams.

---

## A. Float formatting options

### A.1 Per-element attributes (fence info line, all kinds)

````markdown
```mermaid {span=column pos=both width=0.8 captionpos=top}
```table {pos=barrier}
````

| attr | values | default | notes |
|---|---|---|---|
| `span` | `column` \| `wide` | per-kind `[spans]` | `figure` vs `figure*`; tables: `table` vs `table*` (new: wide tables) |
| `pos` | `top` \| `bottom` \| `both` \| `page` \| `here` \| `force` \| `barrier` | `[floats] pos_default` (`top`) | see mapping |
| `width` | fraction (`0.8`) or TeX length (`5cm`) | span width | pictures/diagrams only; scales `\includegraphics` or TikZ `scale=` |
| `captionpos` | `top` \| `bottom` | figures `bottom`, tables `top` | explicit override of the convention |
| `label`, `caption` | (existing `%%` comments) | — | unchanged |

`pos` → LaTeX mapping:

| `pos` | in-column | wide (`*`) |
|---|---|---|
| `top` | `[t]` | `[t]` |
| `bottom` | `[b]` | `[b]` + auto `dblfloatfix` (stock LaTeX bans bottom wide floats) |
| `both` | `[!tb]` | `[!tb]` (+ dblfloatfix) |
| `page` | `[p]` | `[p]` |
| `here` | `[h]` | **error** (`figure*` has no here; use `span=column` or `force`→column) |
| `force` | `[H]` + auto `float` pkg | **error** (`[H]` is illegal on `*` floats) |
| `barrier` | `\FloatBarrier` + `[t]` | `\FloatBarrier` + `[t]` (+ dblfloatfix only if combined with bottom) |

`barrier` needs `placeins`; `force`/`here`... `here` needs nothing;
packages auto-added to the preamble via the `tex_requirements`
mechanism (detected from the woven body: `\FloatBarrier` →
placeins, `[H]` → float, `dblfloatfix` marker comment → dblfloatfix).

### A.2 Document level: `[spans]` + new `[floats]`

```toml
[spans]
diagram = "wide"      # existing
plot = "column"       # existing
table = "column"      # existing
picture = "column"    # NEW kind (precompiled graphics, was `default`)
default = "column"    # existing fallback

[floats]
pos_default = "top"   # default pos for all elements
topfraction = 0.9     # was LaTeX 0.7: tall TikZ fits top placement
bottomfraction = 0.8  # was 0.3: bottom becomes usable
textfraction = 0.1    # was 0.2: pages may hold 90% floats
floatpagefraction = 0.6
topnumber = 2
bottomnumber = 2
barrier_sections = false  # true: \FloatBarrier before every \section
```

Emitted as `\renewcommand`/`\setcounter` lines in the generated
preamble (after packages, before `\begin{document}`; appended
verbatim into template heads too — they are preamble-legal).
`barrier_sections` inserts `\FloatBarrier` ahead of each section
head during assembly (not a body edit: deterministic, testable).

### A.3 Validation (fail fast,all in Rust)

- wide + `force`/`here` → error naming the element + fix
  (`span=column` or drop to `both`).
- `force` outside… no other restriction; but WARN (cargo test
  asserts the warning string?) — no: keep errors only. Journals
  forbid `[H]`; the shipped heads never emit it; user choice.
- unknown attr value → error listing allowed values (strict
  subset grammar, same doctrine as mermaid).
- `width=` parses as fraction 0–1 or TeX length regex;
  else error.
- template heads: validation unchanged (required ⊆ provided);
  `[floats]` lines are preamble-legal anywhere.

### A.4 Wide tables

`table*` support falls out of `span=wide` for tables (colspec
width `\textwidth`). Validation: none special.

---

## B. Level 3: document skeleton

Level 1 replaces the head; level 3 replaces the WHOLE document.
`[template] skeleton = "paper-skel.tex"` (mutually exclusive with
`preamble_file`; error if both set).

### B.1 Token contract

The skeleton is complete `.tex` with tokens; ikat renders values,
validates, done — no generated head or tail at all:

| token | fills from |
|---|---|
| `{{title}}` `{{author}}` `{{thanks}}` | as level 1 |
| `{{body}}` | woven body (floats + text + tables) |
| `{{bibliography}}` | `\bibliographystyle{<spec bib_style>}` + `\bibliography{<spec bib_name>}` |

Rules: `{{body}}` REQUIRED (error if absent); `{{bibliography}}`
REQUIRED unless spec `bib_name` is empty (working papers without
refs); tokens may appear anywhere (frontmatter, multicol wraps,
appendices after the bibliography — all expressible); unknown
`{{word}}` tokens → error (typo guard); `{{body}}` twice → error.
`abstract_before_maketitle` still applies (split before render).
`preamble_append` inserts before `\begin{document}` as usual.
Package validation scans the WHOLE skeleton (it is the preamble).

### B.2 What this unlocks (currently hacky or impossible)

- Native ACM/Elsevier frontmatter + abstract order, no hooks.
- multicol 3-column WITHOUT the `\apptocmd{\maketitle}` trick
  (`\begin{multicols}{3}{{body}}\end{multicols}`... precisely:
  body wrapped literally in the file).
- biblatex (`\printbibliography` instead of the token content —
  skeleton authors just write it and leave `bib_name` empty).
- Per-journal tails: acknowledgments env, appendices, data
  availability statements, `\supplementary` blocks.
- `{{body}}`-less skeletons are errors, so no silent empty PDFs.

### B.3 Shipped example

`templates/skeleton-plain.tex`: article + `{{body}}` + tail,
parallel to the generated default, as the customization starting
point (level 1 heads stay the recommended path; skeleton is the
escape hatch).

### B.4 Implementation order

1. `BuildSpec.skeleton: String` (content; serde default "") +
   `render_skeleton` + validation (Rust, unit-tested).
2. `build_from_paths` reads `[template] skeleton` (mutual
   exclusion check in Python, clearer error than Rust).
3. `skeleton-plain.tex` + pytest round-trip + headsproof-style
   compile check with a wrapped body.
4. Docs: README skeleton section; head comments cross-link.

---

## C. Markdown parsing: stay hand-rolled or take an AST package?

Today: a hand-rolled line scanner in `doc.rs` (headings, fences
with `{attr}` info strings, pipe tables, paragraphs) + custom
inline pass in `esc.rs` (`\cite[`k`]`, math shielding, escapes).

### C.1 Candidates

- **pulldown-cmark** (Rust): de-facto standard (mdBook renders
  with it), CommonMark + GFM tables + fenced info strings,
  event-stream (no tree allocation), fast, maintained.
- **comrak** (Rust): full AST + GFM extensions; heavier, tree
  you must walk; wins only if we need AST surgery (we don't —
  our transform is streaming: events in, LaTeX out).
- **markdown-it-py / mistune / marko** (Python): wrong side of
  the boundary (Rust owns parsing); would split the pipeline
  across languages for zero gain.

Non-standard syntax (`[`k`]` citations, `{span=}` attrs, `%%`
fence comments) stays custom in ALL options — AST libraries
deliver it as plain text/code events for us to post-process.
No package removes custom code; the question is only who owns
block structure.

### C.2 Decision: spike pulldown-cmark, switch only on parity

1. Spike (no behavior change): parse the paper manuscript with
   BOTH the hand scanner and pulldown-cmark; diff the event
   stream (headings/fences/tables/paragraph breaks) in a cargo
   test. Unknowns to settle: info-string fidelity (`{span=}`
   must survive verbatim), table cell splitting parity,
   blank-line/paragraph rules vs our golden `.tex`.
2. Switch only if the diff is empty AND the golden paper test
   stays byte-identical. Keep `esc.rs` inline pass untouched
   (it operates on text events either way).
3. If the diff never closes: stay hand-rolled and say so here.
   A 400-line scanner we fully understand beats a dependency
   that parses 95% of our needs.

Rationale: our Markdown is a strict subset (paper manuscripts),
not arbitrary user input — the usual reason to take an AST
package (robustness against the wild) barely applies, while
byte-parity with the published paper is a hard constraint the
spike must satisfy first.
