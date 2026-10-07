---
name: ikat project site
description: Cool-white field-service manual for a Markdown-to-LaTeX pipeline.
colors:
  primary: "#1649e8"
  primary-deep: "#0e36b6"
  surface-cool: "#f3f7fc"
  surface-white: "#ffffff"
  surface-blue: "#e8effa"
  text-ink: "#111d2c"
  text-body: "#24344a"
  text-muted: "#46576b"
  rule: "#c3cfde"
  rule-dark: "#7f91a7"
  signal-rust: "#b43b22"
  signal-soft: "#fae5df"
typography:
  display:
    fontFamily: "Barlow Condensed, sans-serif"
    fontSize: "clamp(4rem, 6.7vw, 6rem)"
    fontWeight: 700
    lineHeight: 0.93
    letterSpacing: "-0.025em"
  body:
    fontFamily: "Source Sans 3, Segoe UI, sans-serif"
    fontSize: "1.0625rem"
    fontWeight: 400
    lineHeight: 1.58
  label:
    fontFamily: "IBM Plex Mono, Cascadia Mono, Consolas, monospace"
    fontSize: "0.64rem"
    fontWeight: 400
    lineHeight: 1.4
    letterSpacing: "0.055em"
components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.surface-white}"
    rounded: "0px"
    padding: "0.8rem 1.15rem"
    height: "52px"
  button-primary-hover:
    backgroundColor: "{colors.primary-deep}"
    textColor: "{colors.surface-white}"
  button-outline:
    backgroundColor: "{colors.surface-white}"
    textColor: "{colors.primary-deep}"
    rounded: "0px"
    padding: "0.45rem 0.7rem"
    height: "40px"
  button-copy:
    backgroundColor: "{colors.surface-white}"
    textColor: "{colors.primary-deep}"
    rounded: "0px"
    padding: "0.35rem 0.65rem"
    height: "36px"
---

# Design System: ikat project site

## Overview

**Creative North Star: "The Exploded Service Manual"**

The site treats ikat's manuscript pipeline as a useful assembly drawing, not a decorative weaving metaphor. Labeled white plates show real input, figure source, Rust processing, and TeX output; thin leaders, clear part labels, and restrained shadows give each stage a place. The opening composition makes the product and the try-it action legible before the visitor scrolls.

The atmosphere is cool, bright, and high-contrast, as requested by the user. Cobalt carries the main action and structure; signal rust marks annotations. Source Sans 3 carries reading copy, Barlow Condensed gives headings and part names a compact technical voice, and IBM Plex Mono is reserved for commands, code, and measurements.

**Key Characteristics:**
- Cool-white page field and pale blue drafting surface.
- Cobalt structure and action, with signal-rust annotations.
- Separated, labeled assembly plates instead of generic feature-card scaffolding.
- Locally served display, reading, and code typefaces.

## Colors

The palette pairs a light, cool field with dark ink, a confident cobalt action color, and a warm rust annotation color.

### Primary
- **Instrument Cobalt**: The primary action, emphasized headings, diagram structure, and active output.
- **Deep Cobalt**: Link text and hover states where the stronger value improves contrast.

### Secondary
- **Signal Rust**: Annotation marks, part indices, and small section labels. It is a signal, not a second action color.
- **Soft Rust Wash**: A quiet field behind annotation indices.

### Neutral
- **Cool Paper**: The page ground; keep it light rather than shifting to cream or a dark developer-tool surface.
- **White Plate**: Assembly plates, code sheets, and install surfaces.
- **Drafting Blue**: The pale technical grid and quiet section field.
- **Ink**: Primary text and strong structural lines.
- **Body Ink**: Paragraphs and descriptive copy.
- **Muted Ink**: Secondary labels, status text, and notes; do not substitute low-contrast gray.
- **Fine Rule / Dark Rule**: Table, sheet, and section boundaries.

### Named Rules
**The Light Field Rule.** Keep the page light; make contrast with ink, cobalt, and rust rather than a dark overall surface.

**The Product-First Plate Rule.** Every labeled plate must identify a real part of ikat's source-to-output pipeline. Never use the assembly drawing as unexplained decoration.

## Typography

**Display Font:** Barlow Condensed (self-hosted; sans-serif fallback)

**Body Font:** Source Sans 3 (self-hosted; Segoe UI, sans-serif fallback)

**Label/Mono Font:** IBM Plex Mono (self-hosted; Cascadia Mono, Consolas, monospace fallback)

**Character:** Compressed, weighty display lettering gives the field-manual headings a strong silhouette. Source Sans 3 keeps technical explanations comfortable to read; the mono face appears only where text is code, data, or a measured label.

### Hierarchy
- **Display** (700, `clamp(4rem, 6.7vw, 6rem)`, line-height 0.93, tracking −0.025em): Hero heading, balanced and constrained to a short measure.
- **Headline** (700, `clamp(2.8rem, 5vw, 4.6rem)`, line-height 0.98): Major section titles.
- **Title** (700, typically 1.65–2rem, line-height near 1): Plate names, install title, and capability labels.
- **Body** (400, 1.0625rem, line-height 1.58): Paragraphs and explanatory copy; keep reading measures around 65–70ch.
- **Label** (400, 0.64–0.72rem, tracking 0.055em, uppercase): Part identifiers, data labels, and small resource types.

### Named Rules
**The Mono-For-Data Rule.** Use monospace for actual commands, code, and measurement labels, never as a generic “technical” costume for paragraphs.

## Layout

The main container tops out at 1240px. It uses 32px side insets below that cap until 800px, 20px below 800px, and 16px below 620px. The hero is a two-column promise-and-assembly composition until 800px, then stacks with the product explanation before the diagram. At mobile widths, the exploded drawing becomes a diagonal stack and its assembled state becomes a vertical sequence; the proof sheets, install layout, capability rows, and resource list collapse to one column.

Sections are separated by fine horizontal rules and changes in light surface tone. The flow uses open rows, ruled lists, and code sheets; repeated same-size icon cards are not the page's content structure. Keep headings and paragraphs separated by more space above the heading than below it.

## Elevation & Depth

The system is mostly flat. A small, soft, offset shadow separates the assembly drawing and install sheet from the page; a quieter shadow gives individual plates a little depth. The active output gets a restrained blue lift. Rules and tonal shifts do most of the structural work.

### Shadow Vocabulary
- **Sheet lift** (`0 12px 26px rgb(17 29 44 / 9%)`): Outer assembly sheet.
- **Plate separation** (`0 7px 15px rgb(17 29 44 / 12%)`): Individual exploded parts.
- **Install lift** (`0 10px 22px rgb(17 29 44 / 8%)`): Quick-start sheet.
- **Active output** (`0 12px 22px rgb(22 73 232 / 18%)`): Output plate after assembly.

**The Soft-Depth Rule.** Depth uses an offset plus blur. Never add a hard, zero-blur block shadow or a decorative glow.

## Shapes

Sheets and controls use square corners and one-pixel rules. The assembly parts are rectangular paper plates, not rounded cards. The small circular join mark in the code proof is a single diagram annotation, not a general radius system. The drafting grid belongs only inside the assembly illustration.

## Components

### Buttons
- **Primary action:** Square cobalt field, white bold text, generous tap height. Hover deepens the cobalt; keyboard focus uses the rust outline.
- **Outline control:** White field with a one-pixel cobalt border, used for the assembly action.
- **Copy control:** Compact white button with cobalt border and text; announces copied or blocked status through a live region.
- **Do not** replace action labels with icon-only controls.

### Assembly Plates
- **Shape:** Square, one-pixel ink or cobalt boundary, with cobalt or rust on the top rule of the relevant part.
- **Content:** Each plate labels its input or output and uses real pipeline vocabulary. The assemble control aligns the parts; the static exploded layout remains understandable without JavaScript.
- **Responsive behavior:** The drawing becomes a vertical stack on narrow screens; no part may be clipped or become unreadable.

### Code Sheets
- **Shape:** Square bordered sheets with a caption row and a light blue output field.
- **Content:** Source and generated TikZ stay selectable text. Keep long code horizontally scrollable rather than shrinking it to illegibility.

### Disclosure Controls
- Native `<details>` groups present platform-specific commands. Preserve their keyboard and open/closed semantics; do not replace them with a custom wizard.

### Navigation
- Top navigation is a simple text row with one square outlined repository link. It wraps at narrow widths and remains visible without a menu interaction.
- Resource links form ruled rows. Hover may shift a row slightly; labels stay clear and the whole link remains keyboard reachable.

## Do's and Don'ts

### Do:
- **Do** keep the main field cool and light, with ink-dark body text and cobalt actions.
- **Do** use rust as a compact annotation color, not as a large text field.
- **Do** make the assembly interaction explain an actual input-to-output transformation.
- **Do** keep source examples selectable and use the locally served font files.
- **Do** preserve the single authored motion moment and honor `prefers-reduced-motion`.

### Don't:
- **Don't** turn the page into a dark terminal or a warm cream editorial page.
- **Don't** replace the assembly illustration with generic feature cards or decorative grid backgrounds across the whole site.
- **Don't** invent performance numbers, release availability, customer claims, or unsupported Mermaid syntax.
- **Don't** paste figures as screenshots when the product can emit editable TikZ/pgfplots.
- **Don't** use monospace for ordinary prose or use rust annotations as small low-contrast text on white.
