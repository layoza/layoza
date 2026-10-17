---
name: Orrery
description: Two-ink planetarium season programme for a 60-seat harbourside dome
colors:
  ultramarine-ink: "#1F2BB8"
  ultramarine-deep: "#141B7C"
  sodium-amber: "#FFB000"
  cool-stock: "#E9ECEF"
  stock-shade: "#DDE2E7"
  night-text: "#10143D"
  ink-soft: "#3A4176"
  on-ink: "#F1F3F6"
  on-ink-soft: "#C3C9F4"
  taken-seat: "#4A55C9"
  error: "#A3201B"
  error-on-ink: "#FFC9C4"
typography:
  display:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "clamp(2.6rem, 1rem + 4.7vw, 5.25rem)"
    fontWeight: 850
    lineHeight: 0.92
    letterSpacing: "-0.02em"
    fontVariation: "'wdth' 125"
  headline:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "clamp(2.1rem, 1rem + 3.4vw, 4rem)"
    fontWeight: 820
    lineHeight: 0.94
    letterSpacing: "-0.015em"
    fontVariation: "'wdth' 125"
  billing:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "clamp(1.9rem, 1.3rem + 1.8vw, 3rem)"
    fontWeight: 850
    lineHeight: 0.95
    fontVariation: "'wdth' 70"
  title:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "clamp(1.35rem, 1.1rem + 0.9vw, 1.9rem)"
    fontWeight: 780
    lineHeight: 1.02
    fontVariation: "'wdth' 118"
  body:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "1.0625rem"
    fontWeight: 420
    lineHeight: 1.55
    fontFeature: "'tnum', 'lnum'"
  label:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "0.8125rem"
    fontWeight: 650
    lineHeight: 1.2
    letterSpacing: "0.07em"
    fontVariation: "'wdth' 82"
rounded:
  print: "2px"
  seat: "50%"
spacing:
  s2: "8px"
  s3: "12px"
  s4: "16px"
  s5: "24px"
  s6: "32px"
  s7: "48px"
  section: "clamp(72px, 9vw, 136px)"
  gutter: "clamp(16px, 4vw, 48px)"
components:
  button-amber:
    backgroundColor: "{colors.sodium-amber}"
    textColor: "{colors.night-text}"
    rounded: "{rounded.print}"
    padding: "12px 22px"
    height: "48px"
  button-line:
    backgroundColor: "transparent"
    textColor: "{colors.ultramarine-ink}"
    rounded: "{rounded.print}"
    padding: "12px 22px"
    height: "48px"
  button-line-hover:
    backgroundColor: "{colors.ultramarine-ink}"
    textColor: "{colors.on-ink}"
  input-on-ink:
    backgroundColor: "{colors.ultramarine-deep}"
    textColor: "{colors.on-ink}"
    rounded: "{rounded.print}"
    padding: "12px 14px"
    height: "52px"
  session-chip-selected:
    backgroundColor: "{colors.sodium-amber}"
    textColor: "{colors.night-text}"
    rounded: "{rounded.print}"
    height: "64px"
---

# Design System: Orrery

## Overview

**Creative North Star: "The Season Programme"**

Orrery's pages read like a mid-century planetarium programme printed in two inks on cool uncoated stock. Ultramarine owns whole fields: booking, membership, the footer and the dome itself. Sodium amber is the second ink. It never decorates. It marks the one thing that is live, chosen or held. Where amber lands on ultramarine (held seats on the dome plan), the two inks overprint (multiply), the way a press would print them.

Everything is set in one variable grotesk, Archivo. Hierarchy comes only from width, weight and size: wide black for headings, condensed caps for billed show titles, normal width for reading. Information is ruled, not boxed. Timetables, show listings, shop modules and FAQ all sit on hairlines under a heavy 2px ink rule, like the pages of a programme. The signature object is the dome plan, a circular diagram of the 60-seat dome. It appears in the hero and again in booking, where your seats light up in amber.

**Key Characteristics:**
- Two inks on cool stock; no third hue
- One family (Archivo) at many widths
- Ruled rows instead of cards
- Square print corners (2px)
- Halftone dot screens as the only texture
- Perforated ticket edges on confirmations and dates

## Colors

The palette is a press palette: two spot inks and their stock.

### Primary
- **Ultramarine Ink** (`ultramarine-ink`): Section fields, headings on stock, rules, outlined buttons, and the dome disc.
- **Ultramarine Deep** (`ultramarine-deep`): Ink-on-ink depth. Used for the projected-sky disc, form inputs on blue fields and the ticket body.

### Secondary
- **Sodium Amber** (`sodium-amber`): The active ink. Used for the primary action, the selected session, held seats, the live presenter mark, the basket count once it holds items, the "Go" weather chip.

### Neutral
- **Cool Stock** (`cool-stock`): The page ground, the paper itself.
- **Stock Shade** (`stock-shade`): The second paper tone, for shop art beds, the schools panel and the scrollbar track.
- **Night Text** (`night-text`): Body text on stock, and text on amber.
- **Ink Soft** (`ink-soft`): Secondary text on stock, tinted from the ink and never grey.
- **On Ink / On Ink Soft** (`on-ink`, `on-ink-soft`): Primary and secondary text on ultramarine fields.
- **Taken Seat** (`taken-seat`): Occupied seats on the dome plan.

### Named Rules
**The Active Ink Rule.** Amber marks only what is live, selected or held, plus the one primary action per view. If nothing is active, there is no amber.

**The Overprint Rule.** Overprint is spent only on amber that is already earned: a held seat carries a slightly misregistered amber ring printed `mix-blend-mode: multiply` into the ultramarine, so it reads as two inks meeting. Never overprint decoration or headlines.

**The Tinted Secondary Rule.** Secondary text takes its hue from the ink of its field (`ink-soft` on stock, `on-ink-soft` on blue). Never neutral grey.

## Typography

**Display Font:** Archivo (variable; width 62–125, weight 100–900), with a system-ui fallback.
**Body Font:** Archivo, normal width.

**Character:** One family carries every voice. Wide black is the programme's headline voice. Condensed caps work like the billing on a festival sheet. Normal width carries the reading copy.

### Hierarchy
- **Display** (850, wide 125%): Hero headline only.
- **Headline** (820, wide 125%): Section heads.
- **Billing** (850, condensed 70%, uppercase): Show titles in the programme listing.
- **Title** (780, 118%): Subsections, tiers and panels.
- **Body** (420, 1.0625rem, 1.55): Reading copy with a measure of about 68ch. Numerals are tabular and lining everywhere.
- **Label** (650, 0.8125rem, 82% width, uppercase, 0.07em tracking): Data labels such as Runs, Ages and Seat. Never used as a kicker above a heading.
- **Numerals** (850, wide 125%): Times, prices and dates are set big, like a timetable.

### Named Rules
**The Width Is Hierarchy Rule.** Change width before you change family. A second display face is never added.

## Layout

The layout is a 12-column grid inside a 1320px container with fluid gutters (`gutter`). Section heads split 7/5: the headline on the left, a short standfirst on the right. Sections breathe at `section` padding. Inside sections, content sits on ruled lists: a 2px ink rule on top, then 1px hairlines at 28% ink between rows. The shop is a dense hairline-ruled module grid: 3 columns, then 2 below 980px, then 1 below 560px. Breakpoints:
- 980px: the nav collapses into a full-screen ink drawer.
- 900px: section heads, booking and telescope stack, and the timetable regrids to time / title + meta / action.
- 700px: the hero dome stacks under the copy, capped at 340px wide.
- 560px: the form becomes a single column.

## Elevation & Depth

The system is flat. Depth comes from ink fields and tonal steps (stock to stock-shade, ink to ink-deep). There are two soft shadows, both used only as responses:
- **Lift** (`0 6px 14px -6px rgba(16,20,61,.45)`): Button hover, with a 1px rise.
- **Float** (`0 18px 40px -18px rgba(16,20,61,.5)`): The basket panel and the toast, the only floating layers.

**The Flat Press Rule.** Nothing printed casts a shadow at rest.

## Shapes

Corners are square at a 2px print radius. Circles are reserved for the dome, seats, legend swatches and the FAQ toggle. Borders are 2px ink for controls and panels, and 1px hairlines for rows. The ticket edge is perforated with punched circles drawn by a repeating radial-gradient. It is used on the booking confirmation stub and the telescope-night date blocks.

## Components

### Buttons
- **Shape:** Square print corners (2px), a 48px minimum height, 2px border.
- **Primary (amber):** Amber fill with night text. Use it for the main action of a view.
- **Line:** Transparent with an ink border and ink text. It fills with ink on hover. On ink fields it inverts to an on-ink border.
- **Hover / Focus:** A 1px lift with the Lift shadow. The focus ring is a 3px outline offset by 3px: ink on stock, amber on ink fields.

### Chips
- **Weather states:** Watching (outline), Go (amber fill), Called off (dashed outline, struck). Condensed uppercase 0.78rem.

### Inputs / Fields
- **Style:** Ink-deep fill on ultramarine fields, a 2px on-ink-soft border, 52px tall.
- **Focus:** Amber outline and amber border.
- **Error:** The border turns error-on-ink, with an inline message and an authored alert icon under the field. A summary line sits by the submit button, and focus moves to the first invalid field.

### Session selector
A row of three square tiles with wide numerals. The selected tile fills amber. A session that has already started is struck through and disabled.

### Navigation
A sticky stock header with a 2px ink rule under it. The wordmark is wide black, and nav links are normal-width 620 weight with a stock-shade hover. The basket is an outlined button with an ink count tile, which turns amber when the basket holds items. On mobile, a full-screen ultramarine drawer lists the nav in wide black at 1.9–2.6rem on hairlines.

### Dome Plan (signature)
An SVG plan of the dome. It has 60 seats in three rings (16, 20 and 24) around an amber presenter mark, a slowly turning projected sky (the Plough) clipped to the inner disc, and a halftone screen. In booking, taken seats render as `taken-seat` and held seats as larger amber dots, kept together where possible. The turning stops under reduced motion.

### Ticket confirmation
An ink-deep body with an amber border and an amber stub (date, time, seats) on a perforated edge.

## Do's and Don'ts

### Do:
- **Do** let ultramarine own whole sections; alternate stock and ink fields down the page.
- **Do** set times, prices and dates as wide black numerals.
- **Do** rule lists with a 2px ink top rule and 1px hairlines.
- **Do** use halftone dots (radial-gradient, about 7px pitch) as the only texture.
- **Do** honor `prefers-reduced-motion` for the sky rotation, presenter pulse, drawer and accordion.

### Don't:
- **Don't** add a third hue or gradients; amber and ultramarine are the only inks.
- **Don't** use amber as decoration on inactive elements.
- **Don't** put content in rounded shadowed cards; rule it instead.
- **Don't** add a second typeface; change Archivo's width instead.
- **Don't** put kicker labels above headings.
