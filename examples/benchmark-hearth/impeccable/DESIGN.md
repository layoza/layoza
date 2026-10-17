---
name: Hearth & Rye
description: Market enamel signs and day-glo trader tickets for a one-oven restaurant and bakery
colors:
  enamel-cobalt: "#1B3A8C"
  cobalt-deep: "#142C6B"
  enamel-white: "#F3F5F2"
  worn-white: "#C9D3EE"
  book-red: "#C8281E"
  book-red-press: "#A51F17"
  steel-black: "#121418"
  steel-2: "#1D2026"
  dayglo-ticket: "#E9FF4D"
  brass: "#B8964A"
  ink-on-white: "#2C4373"
typography:
  display:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "clamp(2.5rem, 6vw, 5.5rem)"
    fontWeight: 900
    lineHeight: 0.92
    letterSpacing: "-0.01em"
  hero:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "clamp(2.75rem, 6.4vw, 6rem)"
    fontWeight: 900
    lineHeight: 0.92
  body:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "1.0625rem"
    fontWeight: 400
    lineHeight: 1.55
  label:
    fontFamily: "Archivo, system-ui, sans-serif"
    fontSize: "0.8rem"
    fontWeight: 700
    letterSpacing: "0.06em"
  ticket-price:
    fontFamily: "Permanent Marker, Archivo, cursive"
    fontSize: "2.3rem"
    fontWeight: 400
    lineHeight: 1
rounded:
  plate: "22px"
  head: "16px"
  field: "12px"
  ticket: "10px"
  pill: "999px"
spacing:
  unit: "8px"
  gutter: "clamp(16px, 4vw, 56px)"
  plate-pad: "clamp(32px, 5vw, 72px)"
  section: "clamp(72px, 9vw, 128px)"
components:
  button-book:
    backgroundColor: "{colors.book-red}"
    textColor: "#FFFFFF"
    rounded: "{rounded.pill}"
    height: "52px"
    padding: "0 26px"
  button-line:
    backgroundColor: "transparent"
    textColor: "{colors.enamel-white}"
    rounded: "{rounded.pill}"
    height: "52px"
  plate-cobalt:
    backgroundColor: "{colors.enamel-cobalt}"
    textColor: "{colors.enamel-white}"
    rounded: "{rounded.plate}"
    padding: "{spacing.plate-pad}"
  plate-white:
    backgroundColor: "{colors.enamel-white}"
    textColor: "{colors.enamel-cobalt}"
    rounded: "{rounded.plate}"
    padding: "{spacing.plate-pad}"
  ticket:
    backgroundColor: "{colors.dayglo-ticket}"
    textColor: "{colors.steel-black}"
    rounded: "{rounded.ticket}"
  input:
    backgroundColor: "#FFFFFF"
    textColor: "{colors.steel-black}"
    rounded: "{rounded.field}"
    height: "54px"
---

# Design System: Hearth & Rye

## Overview

**Creative North Star: "The market enamel sign"**

The page is built from vitreous enamel signs of the kind that hang in Manchester's old market halls. Each section is a fired plate held up by brass screws, and day-glo trader price tickets hang off the plates on strings. The world refuses the two category defaults: the dark ember-glow restaurant and the cream-and-serif artisan bakery. Monumental lettering is the image, and the booking action is the only thing coloured red.

**Key Characteristics:**
- Committed colour: enamel cobalt carries most of the page, and white enamel plates hold the dense content (menu, form, FAQ).
- Plates have a 3px keyline inset 10px from the edge, brass screws at the corners, and an occasional chipped corner showing steel black.
- Day-glo is ticket card only: prices, sell-out times, the live "in the oven" ticket and the booking success ticket.
- One oven tells the story: a canvas fire inside an enamel arch is driven by a day scrubber.

## Colors

The strategy is committed. Four fired enamels plus one ticket card, with no gradients and no tints beyond the named tokens.

### Primary
- **Enamel Cobalt** (#1B3A8C): the page ground, the hero, bakery and footer plates, and text on white plates. Cobalt Deep (#142C6B) is used for secondary cobalt plates (chef's table, footer) and the scrollbar track.

### Secondary
- **Book Red** (#C8281E): booking actions only. Pressed state is #A51F17.
- **Day-glo Ticket** (#E9FF4D): ticket cards only, plus the error flag behind inline form errors and the focus ring on cobalt.

### Neutral
- **Enamel White** (#F3F5F2): white plates, and lettering on cobalt or steel.
- **Worn White** (#C9D3EE): secondary text on cobalt and steel.
- **Ink on White** (#2C4373): secondary text on white plates.
- **Steel Black** (#121418): the oven plate, chips, and ticket text. Steel 2 (#1D2026) is the oven interior and slider track.
- **Brass** (#B8964A): screws and the bakery ticket rail.

### Named Rules
**The Red Is Booked Rule.** Red appears only on Book a table, Book, and Request this table. Nothing decorative is red, apart from the tiny flame in the logo mark.
**The Ticket Rule.** Day-glo exists only as ticket card stock (and the error flag). It never becomes a general accent.

## Typography

**Display Font:** Archivo variable, at wdth 112–125 and weight 900, uppercase
**Body Font:** Archivo at wdth 100
**Ticket Font:** Permanent Marker, used for ticket prices and ticket headlines only

### Hierarchy
- **Hero** (900, clamp(2.75rem, 6.4vw, 6rem), 0.92): three stacked statements.
- **Display / h2** (900, clamp(2.5rem, 6vw, 5.5rem), 0.92): section names set like sign lettering.
- **Heading / h3** (800–900, 1.05–1.5rem, wdth 104–122): course names and dish names.
- **Body** (400, 1.0625rem, 1.55): measure 46–62ch.
- **Label** (700, 0.8rem, 0.06em tracking, uppercase): form field labels only.

## Layout

Plates sit inside a 1360px max wrap with a fluid gutter (16–56px). Sections are separated by 72–128px of cobalt ground, so each plate reads as its own sign on the wall. Inside a plate, content uses 2- or 3-column grids that collapse to one column below 980px or 720px. Bakery tickets hang from a brass rail at staggered string lengths and tilts, in 6, 3, then 2 columns.

## Elevation & Depth

Plates sit forward of the wall: a 2px solid underside plus a soft drop shadow (0 22px 48px -22px, cool near-black). Tickets have a 2px underside in a darker card tone plus a soft shadow. There are no glows and no hard offset shadows.

## Shapes

- Plates have a 22px radius and a keyline at 12px radius.
- The header plate is 16px. Fields and phase buttons are 12px. Tickets are 10px.
- Buttons and filter chips are pills.
- Each ticket has a punched hole (12px circle) where its string attaches.

## Components

### Buttons
- **Book** (red pill, white uppercase Archivo 800 at wdth 112): lifts 2px on hover and presses 1px on active.
- **Line** (transparent, 3px inset ring in currentColor): fills with white, or cobalt on white plates, on hover.

### Chips
- Dietary filter chips have a 3px cobalt ring and a roundel showing V, VG or GF. The pressed state fills cobalt, and the roundel turns day-glo.

### Cards / Containers
- **Plates** (cobalt, white, steel) are the only containers. Tickets hang on plates. No other card shells are used.

### Inputs / Fields
- 54px fields with a 3px cobalt border on a white field.
- Invalid fields get a steel border, a 4px day-glo ring, and an inline day-glo error flag with an icon and a recovery message.

### Navigation
- A sticky white header plate holds the wordmark, pill links and the red Book button.
- Below 980px: the wordmark, a compact red Book button, and a menu button that opens a full-screen cobalt drawer with large uppercase links.

### Hanging Ticket (signature)
- A day-glo card hangs from a string, with the price or headline in Permanent Marker.
- A damped-spring swing is driven by scroll velocity and pointer enter. It is disabled under reduced motion.

### Oven Scrubber (signature)
- An enamel arch frames a canvas fire. A range input (04:30–23:30, step 5) and six phase buttons drive the clock, phase story, temperature bar and flame intensity.
- Flames are white and worn white with steel keylines.

## Do's and Don'ts

### Do:
- **Do** set section names as monumental expanded caps on plates.
- **Do** put prices and times on tickets, in Permanent Marker.
- **Do** keep red for booking only.
- **Do** honour prefers-reduced-motion by stopping the swing, the fire loop and smooth scroll.

### Don't:
- **Don't** add eyebrows or kicker labels above headings.
- **Don't** use day-glo as a highlight colour outside tickets.
- **Don't** use gradients, glass, or glows.
- **Don't** drift toward cream, serif or ember-orange restaurant defaults.
