---
name: Ledgerline
description: Bank feeds matched, VAT returns prepared; printed as a cheque-and-remittance security document.
colors:
  ink: "#0D3A2C"
  ink-deep: "#082A1F"
  ink-2: "#14513E"
  ink-line: "#2E6B57"
  tint: "#E6EFEA"
  paper: "#F3F7F4"
  rule: "#B9CFC5"
  rule-soft: "#D3E2DB"
  muted: "#3F5F53"
  on-ink: "#E6EFEA"
  on-ink-muted: "#A9C7BA"
  plum: "#7B1E3F"
  plum-tint: "#F3E4EA"
  act: "#C2401C"
  act-hover: "#A8361A"
  focus: "#F0A23B"
  perforation: "#A9C3B7"
  stub-ground: "#E9F1ED"
  field-line-on-ink: "#4E8572"
typography:
  display:
    fontFamily: "Bodoni Moda, Didot, Georgia, serif"
    fontSize: "clamp(2.6rem, 5vw, 4.8rem)"
    fontWeight: 500
    lineHeight: 0.98
    letterSpacing: "-0.025em"
    fontVariation: "\"opsz\" 96"
  headline:
    fontFamily: "Bodoni Moda, Didot, Georgia, serif"
    fontSize: "clamp(2.1rem, 3.9vw, 3.5rem)"
    fontWeight: 500
    lineHeight: 1.03
    letterSpacing: "-0.02em"
    fontVariation: "\"opsz\" 72"
  title:
    fontFamily: "Bodoni Moda, Didot, Georgia, serif"
    fontSize: "1.45rem"
    fontWeight: 600
    lineHeight: 1.15
    fontVariation: "\"opsz\" 30"
  body:
    fontFamily: "Schibsted Grotesk, Segoe UI, system-ui, sans-serif"
    fontSize: "1.0625rem"
    fontWeight: 400
    lineHeight: 1.6
    fontFeature: "\"ss01\""
  label:
    fontFamily: "Schibsted Grotesk, Segoe UI, system-ui, sans-serif"
    fontSize: "0.72rem"
    fontWeight: 600
    letterSpacing: "0.08em"
  figure:
    fontFamily: "Spline Sans Mono, ui-monospace, Cascadia Mono, monospace"
    fontSize: "0.95rem"
    fontWeight: 600
    letterSpacing: "-0.01em"
    fontFeature: "\"tnum\""
rounded:
  hairline: "2px"
  sm: "3px"
  full: "50%"
spacing:
  gutter: "clamp(16px, 4vw, 56px)"
  max: "1280px"
  section: "clamp(72px, 9vw, 128px)"
  sec-head-gap: "clamp(36px, 5vw, 64px)"
  row: "22px"
  cell: "9px 8px"
components:
  button-act:
    backgroundColor: "{colors.act}"
    textColor: "#FFFFFF"
    rounded: "{rounded.sm}"
    padding: "0.7em 1.25em"
    height: "48px"
  button-act-hover:
    backgroundColor: "{colors.act-hover}"
    textColor: "#FFFFFF"
  button-line:
    backgroundColor: "transparent"
    rounded: "{rounded.sm}"
    padding: "0.7em 1.25em"
    height: "48px"
  input-on-ink:
    backgroundColor: "{colors.ink-deep}"
    textColor: "{colors.on-ink}"
    rounded: "{rounded.sm}"
    padding: "0.7em 0.9em"
    height: "52px"
  input-on-paper:
    backgroundColor: "#FFFFFF"
    textColor: "{colors.ink}"
    rounded: "{rounded.sm}"
    padding: "0.6em 0.8em"
    height: "50px"
  segmented-option-checked:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.paper}"
    rounded: "{rounded.hairline}"
    padding: "0.55em 1.1em"
    height: "42px"
  nav-link:
    textColor: "{colors.on-ink}"
    typography: "{typography.body}"
    padding: "6px 0"
  plan-recommended:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.on-ink}"
    padding: "30px clamp(18px, 2.4vw, 34px)"
  vat-box-total:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.paper}"
    typography: "{typography.figure}"
    padding: "5px 10px"
---

# Design System: Ledgerline

## Overview

**Creative North Star: "The Security Print"**

Ledgerline is printed, not rendered. Every surface is a piece of security stationery: a mint security-tint ground under fine generated guilloche line-work, deep intaglio green that owns whole bands of the page, and paper documents (remittance advice, statement, cheque, application slip) laid on those bands with perforated stub edges and MICR-style code lines. The product is shown as the document it produces, never as a dashboard screenshot.

Density is that of a financial document: rows on fixed columns, hairline rules, tabular figures, written-out amounts. Colour is ink discipline. Green is the ink, plum is overprinted only where something needs a person, vermilion is overprinted only on things you can press. Display type is engraved Bodoni at large optical sizes; the working text is a plain grotesk; every figure and code sits in a monospace with tabular numerals.

Motion is a single authored moment (the hero clearing pass) plus quiet state transitions; content is always visible by default and reduced motion shows the finished state.

**Key Characteristics:**
- Alternating bands: intaglio-green fields and mint-tint fields, with paper documents lifted onto them.
- Procedurally generated guilloche (wave bands, rose fields, hypotrochoid rosettes) behind every paper and band, each from its own seed.
- Rows on fixed columns; state restyles a row, never the grid.
- Three-ink colour logic: green structure, plum exceptions, vermilion actions.
- Bodoni display, Schibsted Grotesk text, Spline Sans Mono figures.
- Near-square corners (2-3px); perforation (2px dashed) is the divider between a document and its stub.

## Colors

A monochrome intaglio-green system on cool mint paper, with two reserved overprint inks.

### Primary
- **Intaglio Green** (`ink`): the page's ink. Text on light grounds, full-bleed header/hero/estimator/security/signup bands, table header rules (1.5px), the recommended plan, the VAT "to pay" box, checked segmented options, the last step marker.
- **Deep Intaglio** (`ink-deep`): footer band, mobile drawer, and the inset ground of inputs sitting on green bands.
- **Engraver's Green** (`ink-2`): hover tint for interactive headings (FAQ summary).
- **Plate Line** (`ink-line`): rules and dividers on green bands (fact rows, drawer links, footer base).

### Secondary
- **Exception Plum** (`plum`): reserved for exceptions: the flagged bank line, the FLAG capability, form error text and invalid borders, the flag count in the tally.
- **Plum Tint** (`plum-tint`): row ground for a flagged line.

### Tertiary
- **Vermilion Overprint** (`act`): the action ink. Only on pressable primary actions (Start free trial, plan CTAs, submit). White label text sits at 5.2:1.
- **Pressed Vermilion** (`act-hover`): hover state of the action ink; also the text caret colour.
- **Signal Amber** (`focus`): the focus ring (3px solid, 3px offset) on every focusable element.

### Neutral
- **Security Tint** (`tint`): body ground and the default page paper. Also `on-ink`, the text colour on green bands.
- **Document Paper** (`paper`): the lifted paper objects (remittance doc, statement, cheque, plan book, signup slip).
- **Stub Ground** (`stub-ground`): the detachable stub/counterfoil area of a document.
- **Perforation** (`perforation`): the 2px dashed tear line between document and stub, and between plans.
- **Ledger Rule** (`rule`) and **Soft Rule** (`rule-soft`): row dividers and specimen borders on paper; soft rule between table rows.
- **Muted Ink** (`muted`): secondary text, dates, captions and table headers on tint/paper (about 6:1 on tint).
- **Muted On-Ink** (`on-ink-muted`): secondary text, hints and labels on green bands (about 7:1 on ink).
- **Field Line on Ink** (`field-line-on-ink`): 1.5px border of inputs on green bands.

### Named Rules
**The Three Inks Rule.** Green carries structure and text; plum appears only where a person must judge something; vermilion appears only on something you can press. If an element is neither an exception nor an action, it is green.

**The Glyph-And-Word Rule.** Every state carries a glyph and a word (tick + "Invoice INV-0418", flag + "No match"), never colour alone.

## Typography

**Display Font:** Bodoni Moda (with Didot, Georgia)
**Body Font:** Schibsted Grotesk (with Segoe UI, system-ui)
**Label/Mono Font:** Spline Sans Mono (with ui-monospace, Cascadia Mono)

**Character:** Engraved high-contrast Didone headlines, set with the optical-size axis matched to the size, over a plain, sturdy grotesk; figures and codes are always mono with tabular numerals, like a printed statement.

### Hierarchy
- **Display** (500, clamp 2.6-4.8rem, 0.98, opsz 96): the hero headline only, balanced wrap.
- **Headline** (500, clamp 2.1-3.5rem, 1.03, opsz 72): section headings.
- **Title** (600, about 1.2-1.6rem, 1.15-1.2, opsz 18-40): document titles, step and capability names, plan names, fact terms, FAQ questions. Optical size steps down with size (opsz 18 at 1.35rem document title, 30 for most titles, 40 for plan names and the wordmark).
- **Body** (400, 1.0625rem, 1.6, ss01): running text; section intros at 1.12rem, capped at 60ch; FAQ answers at 62ch.
- **Label** (600, 0.72rem, 0.08em tracking, uppercase): table column heads, VAT box captions, cheque field keys (0.74rem, 0.1em). Footer column heads use the same treatment at 0.82rem.
- **Figure** (Spline Sans Mono 500-600, tabular-nums, -0.01em): every amount, date, reference, code and MICR line.

### Named Rules
**The Tabular Figures Rule.** Any number that can be compared (amounts, counts, box values, inputs) is set in the figure face with tabular numerals, right-aligned in its column.

**The Optical Size Rule.** Bodoni is always set with `opsz` matched to its size; never use the display cut small or the text cut large.

## Layout

A 1280px centred container with a fluid gutter (16-56px). Sections breathe at 72-128px vertical padding; a section head (max 760px) sits 36-64px above its content. Major split compositions use a 5/7 column grid (hero, estimator, security, signup), FAQ uses 4/8, and the gap scales 28-80px.

Inside documents, content runs on fixed table columns with 9px/8px cells and full-width row dividers; statement rows use a 120px code column plus two text columns with 22px vertical padding. The four-step process sits on a 4-column grid joined by a dashed rule through the step markers.

Breakpoints: at 1080px the nav collapses to a drawer and every split grid stacks; at 820px the plan book and cheque stack (perforations turn horizontal); at 640px the remittance document drops its stub and date column, labels compress into a box key, and forms go single-column.

## Elevation & Depth

A hybrid: bands are flat, and paper is lifted. Depth comes from the paper-on-band contrast first and a soft, long, offset-down shadow second, as if a printed sheet lay on the green field. Nothing floats in the UI sense; there are no hover lifts.

### Shadow Vocabulary
- **Sheet on Ink** (`0 2px 3px rgba(0,0,0,.18), 0 30px 60px -24px rgba(0,0,0,.55)`): paper documents on green bands (remittance doc, cheque, signup slip, slightly stronger at .2/.6).
- **Sheet on Tint** (`0 1px 2px rgba(13,58,44,.12), 0 24px 48px -30px rgba(13,58,44,.45)`): paper on the mint ground (statement, plan book), shadow tinted with ink.
- **Overprint Press** (`0 1px 0 rgba(0,0,0,.18), 0 6px 16px -8px rgba(120,30,10,.55)`): the vermilion action button only.
- **Drawer Drop** (`0 18px 30px -12px rgba(0,0,0,.45)`): mobile navigation drawer.

### Named Rules
**The Paper Lift Rule.** Only paper objects cast shadows, and only downward; bands and rows stay flat.

## Shapes

Near-square everywhere: 2px on documents and tags, 3px on buttons, inputs and the cheque. Circles are reserved for stamped devices: step markers, the FAQ plus/close disc, slider thumbs, and the punched notches (16px ink circles) where a perforation meets a document edge. Perforation is a 2px dashed line in the perforation colour; ledger rules are 1px solid; header rules above rows and totals are 1.5px solid ink. Bordered VAT boxes butt together with collapsed 1px borders.

## Components

### Buttons
Overprinted and decisive.
- **Shape:** near-square (3px), 48px minimum height (42px in the header).
- **Primary (action):** vermilion overprint with white 650-weight label, 0.7em/1.25em padding, overprint-press shadow.
- **Hover / Focus:** hover deepens to pressed vermilion; active nudges down 1px; 200ms ease-out; focus is the amber 3px ring.
- **Line (secondary):** transparent with an inset 1.5px currentColor stroke that thickens to 2.5px on hover. Used for plan CTAs on paper and Sign in in the drawer.
- **Arrow link:** inline 600-weight link with a 1.5px underline border that thickens to 3px on hover, plus a 16px stroked arrow glyph.

### Cards / Containers
There are no cards. Containers are paper documents.
- **Corner Style:** 2-3px.
- **Background:** document paper, with a stub-ground counterfoil where present.
- **Shadow Strategy:** Sheet on Ink or Sheet on Tint (see Elevation).
- **Border:** none on the sheet; internal 1.5px ink header rules and 1px soft row rules; perforated 2px dashed stubs with punched notches.
- **Internal Padding:** about 22-30px, fluid (clamp 16-40px horizontally).

### Inputs / Fields
- **On ink:** deep intaglio inset with a 1.5px field-line border, mint text, 52px tall (48px for numeric), 3px corners; border lightens on hover; numeric inputs use the figure face, right-aligned.
- **On paper:** white field, 1.5px grey-green border that turns ink on hover, 50px tall; custom select with a stroked chevron.
- **Focus:** amber 3px ring, 3px offset.
- **Error:** plum border on a faint plum ground, plum message line with an alert glyph; on green bands errors use a light coral text.
- **Range:** 4px track filled in mint to the value, ink-ringed tint thumb.

### Chips (segmented control)
- **Style:** 1.5px ink outline on paper, 3px padding between options.
- **State:** checked option fills ink with paper text; focus ring drawn on the label.

### Navigation
Mint text on the intaglio header band, 500 weight at 0.97rem with a 1.5px transparent underline that shows muted on hover. Wordmark is Bodoni 600 at 1.6rem beside a generated rosette mark. Below 1080px a 48px menu button opens a deep-intaglio drawer: Bodoni link list on plate-line rules, then line + action buttons; Escape closes and returns focus.

### Bank-Feed Row (signature)
A table row on fixed columns: date, payee, right-aligned amount, and a state cell with glyph + word + mono reference. Matched rows tick in ink; a flagged row takes the plum-tint ground and plum flag + "No match"; pending rows show a dashed circle and "Matching". State restyles the row and never moves a column.

### Remittance Document (signature)
Paper sheet on the green hero with a vertical perforated stub (mono uppercase vertical caption), a generated rosette seal with ring text, the bank-feed rows, a tally line, and collapsed-border VAT boxes where the final box is ink-filled.

### Cheque Estimator (signature)
A paper cheque on a green band: Bodoni italic payee and amount-in-words lines on 1px ink baselines, a 2px-bordered white amount box in the figure face, a MICR line of mono groups with transit/on-us glyphs encoding the inputs, and a perforated side stub of mono totals.

### Statement and Plan Book
The capabilities list is a statement sheet (header rule, mono code column, Bodoni titles; the exception capability's title is plum). Pricing is a three-leaf book split by vertical perforations; the recommended leaf is ink with a tint tag straddling its top edge.

### Guilloche Layer (signature)
An absolutely positioned, pointer-free layer of seeded SVG paths behind every band and sheet: wave bands (10-22 lines, 0.7-0.8 non-scaling stroke) on bands and statements, rose fields in plan corners, hypotrochoid rosettes for seal, mark and watermark. Strokes sit just off the ground colour (a darker green on ink, a pale mint on paper) so the pattern reads as texture, never as content. Each instance uses its own seed.

## Do's and Don'ts

### Do:
- **Do** generate each guilloche from its own seed so no two surfaces share a pattern.
- **Do** put every comparable number in Spline Sans Mono with tabular numerals, right-aligned.
- **Do** pair every state with a glyph and a word (tick + reference, flag + "No match").
- **Do** lay paper documents on flat bands and lift only the paper (Sheet on Ink / Sheet on Tint).
- **Do** use perforation (2px dashed, punched notches) to separate a document from its stub, counterfoil or sibling leaf.
- **Do** keep the hero clearing pass as the single authored motion moment, with the cleared state shown by default under reduced motion.

### Don't:
- **Don't** use vermilion on anything that cannot be pressed.
- **Don't** use plum for anything that is not an exception, flag or error.
- **Don't** put content in rounded soft cards; corners stay at 2-3px and containers are documents.
- **Don't** let a state change move a column or reflow a row grid.
- **Don't** show the product as a floating dashboard screenshot; show the paper document it produces.
