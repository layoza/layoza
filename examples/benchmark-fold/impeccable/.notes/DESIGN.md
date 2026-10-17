---
name: Fold Architects
description: The studio's model table. Card and greyboard study models on a mountboard ground, one green for action.
colors:
  ground: "#F3F3F0"
  card: "#FFFFFF"
  card-2: "#E6E4DF"
  roof: "#EEEDE9"
  grey: "#B9B4AB"
  grey-dk: "#5E5A53"
  board-1: "#D3CFC7"
  board-2: "#A9A39A"
  board-3: "#8C877E"
  balsa-1: "#E6D3AC"
  balsa-2: "#D6BF94"
  balsa-3: "#BFA474"
  mat-soft: "#C9D6CF"
  ink: "#1D1C1A"
  ink-2: "#56524B"
  rule: "#CFCBC3"
  act: "#1F5A4A"
  act-hover: "#143E33"
  error: "#9B2C1F"
typography:
  display:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "clamp(2.25rem, 4.1vw, 3.75rem)"
    fontWeight: 330
    lineHeight: 1.08
    letterSpacing: "-0.022em"
  heading:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "clamp(1.875rem, 3.2vw, 2.875rem)"
    fontWeight: 360
    lineHeight: 1.08
    letterSpacing: "-0.018em"
  title:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "1.25rem"
    fontWeight: 500
    lineHeight: 1.25
  body:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "1.0625rem"
    fontWeight: 400
    lineHeight: 1.6
  meta:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.5
rounded:
  none: "0px"
spacing:
  sp-1: "0.5rem"
  sp-2: "1rem"
  sp-3: "1.5rem"
  sp-4: "2.5rem"
  sp-5: "4rem"
  sp-6: "clamp(5rem, 9vw, 8.5rem)"
components:
  button-primary:
    backgroundColor: "{colors.act}"
    textColor: "{colors.card}"
    rounded: "{rounded.none}"
    padding: "0.85rem 1.5rem"
    height: "48px"
  button-primary-hover:
    backgroundColor: "{colors.act-hover}"
  chip:
    backgroundColor: "transparent"
    textColor: "{colors.ink}"
    rounded: "{rounded.none}"
    padding: "0.5rem 0.95rem"
    height: "44px"
  chip-active:
    backgroundColor: "{colors.act}"
    textColor: "{colors.card}"
  input:
    backgroundColor: "{colors.card}"
    textColor: "{colors.ink}"
    rounded: "{rounded.none}"
    padding: "0.75rem 0.85rem"
    height: "48px"
---

# Design System: Fold Architects

## Overview
The page is the studio's model table. The only imagery is axonometric study models drawn in white card, greyboard and balsa, sitting on greyboard plinths on a cool mountboard ground. The client asked for simple, quiet and minimal, so the only motion is a colour change on hover and focus. The page feels authored because of the drawings and their strict material rules, not because of decoration.

## Colors
### Primary
- **Cutting-mat green `#1F5A4A`** is used only for things you can act on: links, the primary button, the active filter, focus rings and text selection. Hover changes it to `#143E33`.
### Neutral
- **Mountboard `#F3F3F0`** is the page ground. **Card `#FFFFFF`** is the Approach band and the form fields. **Greyboard `#D3CFC7`** is the footer and the plinths. **Ink `#1D1C1A`** is text and line work. **Ink-2 `#56524B`** is secondary text. **Rule `#CFCBC3`** is hairlines.
### Materials (drawings only)
- **Card faces** are `#FFFFFF`, `#E6E4DF` and `#B9B4AB`; card shows what already existed. **Balsa faces** are `#E6D3AC`, `#D6BF94` and `#BFA474`; balsa shows what was added. Window openings are `#5E5A53`.
### Named Rules
- **One green for action.** Green is only for things you can press.
- **Card is existing, balsa is new.**

## Typography
Archivo is the only typeface. Its width axis carries the hierarchy.
### Hierarchy
- **Display** is used for the hero only: wdth 118, weight 330, up to 3.75rem.
- **Section heading**: wdth 116, weight 360, up to 2.875rem, set under a full-width 1px ink rule.
- **Title**: wdth 112, weight 500, 1.25rem.
- **Body**: 1.0625rem with 1.6 line height, held to a measure of about 40ch.
- **Meta**: 0.875rem in ink-2, with tabular lining numerals.
### Named Rules
- **No eyebrows.** A heading opens its section on a rule. Where order matters, a numeral hangs before the heading on its baseline.

## Layout
- The grid has 12 columns, a maximum width of 82rem and a gutter of `clamp(1.25rem, 4.5vw, 4rem)`. Sections are spaced by `sp-6`.
- In the section head, the heading takes columns 1 to 6 and a short ink-2 note takes columns 7 to 12. They stack below 64rem.
- On desktop the hero statement takes columns 1 to 6 and the model takes 6 to 12. Below 64rem they stack.
- Work runs 3 columns, then 2, then 1. Steps run 4, then 2, then 1. Studio puts the plan and paragraph in columns 1 to 7 and the people in 9 to 12.

## Elevation & Depth
The page is flat, with no shadows. Depth exists only inside the drawings, through face tone: top lightest, left mid, right darkest.

## Shapes
Corners are square everywhere and all lines are 1px hairlines. In drawings, a solid line is a cut and a dashed line (4 3) is a fold: ridges, eaves and wall corners.

## Components
### Buttons
- **Primary:** solid green, white text, 48px tall.
- **Secondary:** ink outline, 44px tall, turns green on hover.
### Chips
Filter chips have a 1px rule border and show a count. The active chip is filled green and set with aria-pressed.
### Inputs / Fields
- **Default:** white, with a 1px board-3 border that turns ink on hover.
- **Focus:** a 2px green outline.
- **Invalid:** an error-red border and an icon message that names the problem and how to recover.
### Navigation
Ink text links turn green and underline on hover. On phones a Menu button opens a full-height panel of large links separated by rules.
### Study model (signature)
An axonometric card model sits on its greyboard plinth with no frame. Under it, a caption strip carries a 1px ink rule, the title, meta (place, year, type) and a one-line description.

## Do's and Don'ts
### Do:
- Draw new imagery as card and balsa models or sections, in the same line grammar.
- Keep green for action only.
### Don't:
- Add animation, gradients, shadows or rounded corners.
- Use photos or stock imagery instead of drawings.
- Put kicker or eyebrow labels above headings.
