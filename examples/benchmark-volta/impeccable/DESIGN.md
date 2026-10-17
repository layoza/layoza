---
name: Volta One
description: The corrugated carton the bike arrives in, printed in one flexo ink and opened around a bike you can turn.
colors:
  kraft: "#C69C6D"
  kraft-shade: "#B88D5F"
  kraft-deep: "#9E7347"
  liner: "#DDC39B"
  label-stock: "#F1E9DA"
  flexo-ink: "#18266B"
  xray-blue: "#2B45C4"
  error-red: "#8E1F12"
  graphite: "#3A3D41"
  moss: "#5D6B44"
  clay: "#B0603F"
typography:
  display:
    fontFamily: "Big Shoulders Stencil Display, Archivo, sans-serif"
    fontSize: "clamp(3.5rem, 6.2vw, 6rem)"
    fontWeight: 900
    lineHeight: 0.86
    letterSpacing: "0"
  headline:
    fontFamily: "Big Shoulders Stencil Display, Archivo, sans-serif"
    fontSize: "clamp(3rem, 7vw, 6rem)"
    fontWeight: 800
    lineHeight: 0.92
    letterSpacing: "0"
  title:
    fontFamily: "Big Shoulders Stencil Display, Archivo, sans-serif"
    fontSize: "1.75rem"
    fontWeight: 800
    lineHeight: 1.1
    letterSpacing: "0.02em"
  body:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "1.0625rem"
    fontWeight: 400
    lineHeight: 1.55
    fontVariation: "\"wdth\" 100"
  button:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "1rem"
    fontWeight: 700
    lineHeight: 1
    letterSpacing: "0.06em"
    fontVariation: "\"wdth\" 88"
  label:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "0.75rem"
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: "0.14em"
rounded:
  print: "2px"
  label: "3px"
  slip: "4px"
spacing:
  s1: "8px"
  s2: "16px"
  s3: "24px"
  s4: "32px"
  s6: "48px"
  s8: "64px"
  s12: "96px"
  s16: "128px"
components:
  button-ink:
    backgroundColor: "{colors.flexo-ink}"
    textColor: "{colors.label-stock}"
    typography: "{typography.button}"
    rounded: "{rounded.print}"
    padding: "0 24px"
    height: "48px"
  button-line:
    backgroundColor: "transparent"
    textColor: "{colors.flexo-ink}"
    typography: "{typography.button}"
    rounded: "{rounded.print}"
    padding: "0 24px"
    height: "48px"
  shipping-label:
    backgroundColor: "{colors.label-stock}"
    textColor: "{colors.flexo-ink}"
    rounded: "{rounded.label}"
    padding: "32px"
  story-flap:
    backgroundColor: "{colors.kraft}"
    textColor: "{colors.flexo-ink}"
    padding: "32px"
  form-field:
    backgroundColor: "transparent"
    textColor: "{colors.flexo-ink}"
    typography: "{typography.body}"
    rounded: "0"
    height: "44px"
  tick-box:
    backgroundColor: "{colors.label-stock}"
    textColor: "{colors.flexo-ink}"
    rounded: "{rounded.print}"
    padding: "16px 16px 14px"
  turn-control:
    backgroundColor: "{colors.label-stock}"
    textColor: "{colors.flexo-ink}"
    rounded: "{rounded.print}"
    size: "44px"
---

# Design System: Volta One

## Overview

**Creative North Star: "The Opened Carton"**

Every surface is part of the corrugated box the bike ships in. The ground is kraft board with a fine paper grain, the inside is a lighter liner, and everything printed on it comes from one deep flexo blue ink: solid where the ink lays down hard, as line work where it is ruled, and screened back to a faint tint where a large mark sits behind the product. The page reads as packaging that has been opened, not as a studio stage: die-lines, fold lines, registration marks and handling symbols are the decoration, and each one is a real carton convention.

The bike is the only thing in colour. Its frame finishes (Graphite, Moss, Clay) and the brighter x-ray blue used to reveal its battery and belt live on the product and the product's own swatches, never on the box. Density is generous and print-like: large stencil display type, wide flute-pitch spacing, and set text kept to readable measures. Motion is one eased camera dolly along a single axis plus part reveals; under reduced motion everything snaps, and without WebGL a printed side elevation of the bike takes its place.

**Key Characteristics:**
- Kraft board ground with paper grain; liner for the 3D stage; label stock for anything you read closely or fill in.
- One ink: flexo blue at full strength, as rules, or screened to 13-14% opacity. No second brand hue.
- Stencil display for everything printed large; Archivo (variable width) for set text.
- Carton conventions as ornament: dashed cut lines, dotted fold lines, registration crosses, this-way-up arrows, a 16 KG weight mark, a sine-flute corrugated seam.
- Bike colours appear only on the product.

## Colors

A two-material palette (kraft board and paper label) printed in one dark flexo blue, with the product carrying the only other colour.

### Primary
- **Flexo Ink** (flexo-ink): the single printing ink. Solid fills on primary buttons, spec-table captions, shipping-slip headers and the footer ("the bottom of the box"); all text; all borders, rules, die-lines and icons. Screened versions (13-14% alpha for background prints and soft hover fills, 30-55% alpha for dashed and dotted rules) are the same ink, not new colours.

### Neutral
- **Kraft Board** (kraft): the page ground, the sticky header ("top flap") and the story flaps, always with the grain texture.
- **Kraft Shade** (kraft-shade): the corrugated seams between sections and the test-ride band; the scrollbar track.
- **Kraft Deep** (kraft-deep): line colour of the flute and liners in the corrugated seam; never a fill.
- **Inner Liner** (liner): the sticky 3D stage behind the bike, the inside of the box.
- **Label Stock** (label-stock): shipping labels, order note, spec table, tick-box fields, test-ride slip, rider-note slips and turn controls; also the text colour on solid ink.

### Product (on the bike only)
- **Graphite** (graphite), **Moss** (moss), **Clay** (clay): frame finishes. Used on the 3D frame, the SVG fallback frame and the colour tick-box swatches. Nowhere else.
- **X-ray Blue** (xray-blue): the battery and belt highlight inside the bike during the reveal chapters. Product-only, like the frame finishes.

### Feedback
- **Error Red** (error-red): validation messages and the solid underline of an invalid field. Not decorative.

### Named Rules
**The One Ink Rule.** The box is printed in flexo blue only. Tints are screens of that ink by opacity; a second printed hue on the carton is off-world.

**The Product Holds the Colour Rule.** Graphite, Moss, Clay and the x-ray blue belong to the bike and its swatches. They never colour a button, rule, background or heading.

## Typography

**Display Font:** Big Shoulders Stencil Display (with Archivo, sans-serif)
**Body Font:** Archivo variable, width axis 62-125 (with system-ui, sans-serif)

**Character:** The stencil face is what a carton printer sprays on the side of a box: condensed, uppercase, bridged. Archivo is the plain shipping-document voice beside it, narrowed slightly (width 82-92) for buttons, navigation and quotes.

### Hierarchy
- **Print** (900, clamp(9rem, 27vw, 26rem), line-height 0.78): the screened VOLTA brand and 16 KG mark printed behind the bike on the stage, ink at 13% alpha, decorative and hidden from assistive tech.
- **Display** (900, clamp(3.5rem, 6.2vw, 6rem), 0.86, uppercase): the product name on the hero shipping label only.
- **Headline** (800, clamp(3rem, 7vw, 6rem), 0.92, uppercase): section headings; story-flap headings step down to clamp(2.75rem, 4.4vw, 4.25rem). Balanced wrapping.
- **Title** (800, 1.375-1.75rem, uppercase): colour names in tick boxes, slip headers, spec key rows, FAQ questions, rider names, price (2.75rem, tabular figures).
- **Body** (400-500, 1.0625rem, 1.55): running text, capped at 36-44ch in panels and 60-68ch in long-form answers. Promise lines run larger (1.3125rem, width 92).
- **Button** (700, 1rem, 0.06em tracking, uppercase, width 88): all buttons.
- **Label** (700, 0.75-0.8125rem, 0.1-0.14em tracking, uppercase): form field labels, legends, handling-mark captions, spec-row headers, part callouts, footer column headings, the label-footer sender line.

### Named Rules
**The Printed vs Set Rule.** Anything that would be printed on the carton (brand, headings, numbers that matter, names) is stencil uppercase; anything you read as a document (sentences, form input, answers) is Archivo sentence case.

**The Label Is Functional Rule.** Small tracked-caps type names a field, a column, a part or a sender. It never sits above a heading as an introducer.

## Layout

Spacing runs on an 8px flute pitch (8, 16, 24, 32, 48, 64, 96, 128); every gap is a multiple. Content sits in a 1312px max container with a fluid gutter of clamp(16px, 4vw, 64px). Long sections pad 128px vertically (96px on narrow screens) and are separated by a 24px corrugated seam rather than whitespace alone.

The opening is a scroll story: a full-height sticky liner stage holds the 3D bike centre-right while panels scroll over its left side (width min(440px, 36vw)), one per chapter: shipping label, battery, belt, weight, colour and order. Section heads pair a large heading with an intro paragraph in two equal columns; the specs, test-ride and FAQ sections use asymmetric 2:1, 5:7 and 4:8 splits. Rider notes sit on a 12-column grid at staggered offsets like slips dropped in a box.

At 860px and below the header collapses to a menu button with a drop-down flap, panels go full width and anchor to the bottom of the stage, all splits stack to one column, and the turn controls move to the top right of the stage.

## Elevation & Depth

Depth is physical paper, not interface chrome. Sheets that sit on the board (shipping label, test-ride slip, story flaps, rider slips) cast a soft, warm-brown diffuse shadow from below with a 1-2px tight lip under the edge, as card stock does on cardboard. Everything printed directly on the board (rules, marks, prints, tables bordered in ink) is flat. Solid-ink buttons carry the same lip-plus-diffuse pair and lift 1px on hover, press 1px on active.

### Shadow Vocabulary
- **Label lift** (`0 1px 0 rgba(60,36,12,.18), 0 18px 34px -16px rgba(60,36,12,.45)`): hero shipping label.
- **Slip lift** (`0 2px 0 rgba(60,36,12,.18), 0 26px 44px -22px rgba(60,36,12,.6)`): the test-ride shipping slip, the largest sheet.
- **Flap lift** (`0 18px 34px -18px rgba(60,36,12,.55)`): story flaps; rider slips use a lighter `0 14px 26px -16px rgba(60,36,12,.5)`.
- **Ink press** (`0 2px 0 rgba(24,38,107,.25), 0 6px 14px -6px rgba(24,38,107,.45)`): solid-ink buttons at rest.

### Named Rules
**The Warm Shadow Rule.** Shadows are tinted kraft brown (rgba(60,36,12)) or ink blue on ink buttons, never neutral grey or black, and always diffuse below the sheet.

## Shapes

Corners are nearly square, like cut board and printed labels: 2px on buttons, controls and bordered panels, 3px on labels and notes, 4px on the shipping slip. Circles appear only where the carton world has them: part-callout rings, the FAQ plus/close disc, registration marks. Borders are ink at 2-3px solid for structure, 1.5px dashed for cut lines and row dividers, and 2px dotted for fold lines (the header's bottom edge, the stage's fold line, the footer base). The corrugated seam is a repeated sine flute between two liner lines, 24px tall.

### Named Rules
**The Die-Line Rule.** Dashed means cut, dotted means fold, solid means print. Choose the line style by what the line is on a real carton.

## Components

### Buttons
Printed and pressed, never glossy.
- **Shape:** near-square (2px), 2px ink border, min height 48px (40px in the header).
- **Primary (Ink):** solid flexo ink with label-stock text, uppercase Archivo at width 88. Used for Order and Send.
- **Secondary (Line):** transparent with ink border and ink text; hover fills with a 14% ink screen.
- **Hover / Focus / Active:** ink button lifts 1px and deepens its shadow; presses 1px on active. Focus is a 3px ink outline offset 3px, with a label-stock halo on solid ink.

### Shipping Label (signature)
The hero panel: label-stock sheet, 2px ink border, 32px padding, label lift shadow. Holds the product name, promise, diamond-bulleted facts, a dashed-rule price row, the two CTAs, and a solid-ruled footer with the sender line and this-way-up arrows.

### Story Flaps
Kraft panels with grain and a 3px ink top edge, flap lift shadow, one headline, one paragraph and a part callout chip (ring plus tracked caps) that names the part highlighted on the 3D bike.

### Tick-box Fields (colour picker)
A three-column bordered strip on label stock. Each cell has a square tick box, a swatch and the colour name in stencil. Unchosen swatches are dashed outlines in their own colour; the chosen one prints solid with an ink border and a drawn tick.

### Inputs / Fields (test-ride slip)
- **Style:** fields are cells of a shipping slip ruled in 2px ink; the input itself is borderless with a 2px dashed ink underline, square corners, 44px min height.
- **Focus:** the underline goes solid and the cell warms to a lighter stock.
- **Valid / Error:** a drawn tick follows a valid label; an invalid field gets a solid error-red underline and a message below.
- **Success:** a rotated stencil RECEIVED stamp in a 3px ink box.

### Navigation
Sticky top flap: kraft with grain, 64px tall, a 2px dotted ink fold line along its bottom. Links are Archivo 600 at width 92 with an ink underline that draws in from the left on hover. Below 860px a square menu button opens a kraft drop-down whose items are stencil uppercase at 2rem, separated by dashed cut lines.

### Turn Controls and Callouts
Three 44px label-stock squares with ink borders and line icons (turn left, turn right, reset), beside a "Drag to turn" handling-mark hint. Part callouts on the stage are an ink ring, an ink leader line and a solid-ink tag in tracked caps.

### Spec Panel and Handling Marks
The spec table is the box's side panel: label stock, 3px ink border, a solid-ink stencil caption, tracked-caps row headers and tabular figures. Beside it, a three-cell strip of handling marks (this way up, 16 kg net, a bike symbol) and a numbered contents list that works as an index.

## Do's and Don'ts

### Do:
- **Do** print every carton element in flexo ink (#18266B) only, at full strength, as line work, or screened by opacity.
- **Do** keep every gap on the 8px flute pitch (8, 16, 24, 32, 48, 64, 96, 128).
- **Do** separate major sections with the 24px corrugated sine-flute seam.
- **Do** pick line style by carton meaning: dashed for cut, dotted for fold, solid for print.
- **Do** set printed matter in Big Shoulders Stencil uppercase and documents in Archivo sentence case.
- **Do** honour reduced motion by snapping the dolly, turns and colour changes, and keep the SVG side elevation as the no-WebGL state.

### Don't:
- **Don't** use Graphite, Moss, Clay or the x-ray blue anywhere but the bike and its swatches.
- **Don't** add a second printed hue, a gradient fill or a glow to the carton.
- **Don't** turn the carton into a dark studio stage with glowing edges.
- **Don't** use neutral grey or black shadows; shadows are warm kraft brown or ink blue.
- **Don't** place tracked-caps labels above headings as introducers; small caps label fields, parts, columns and senders only.
- **Don't** round corners beyond 4px except for true circles (rings, discs, registration marks).
