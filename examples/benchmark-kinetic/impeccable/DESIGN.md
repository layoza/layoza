---
name: Name Studio — Kinetic Signal
description: Agency homepage where one sculpted ribbon carries the identity across alternating obsidian and porcelain fields.
colors:
  obsidian: "#090B11"
  graphite: "#151923"
  porcelain: "#F6F5F0"
  ink: "#111318"
  mute-dark: "#9299A8"
  mute-light: "#5A5F6B"
  line-dark: "#2B303B"
  line-light: "#D9DAD6"
  cobalt: "#4865FF"
  cobalt-hover: "#304EF0"
  signal-lime: "#D8FF57"
  lavender: "#A99BFF"
typography:
  hero:
    fontFamily: "Space Grotesk, Helvetica Neue, Arial, sans-serif"
    fontSize: "min(10.4vw, 9.6rem)"
    fontWeight: 600
    lineHeight: 0.84
    letterSpacing: "-0.045em"
  display:
    fontFamily: "Space Grotesk, Helvetica Neue, Arial, sans-serif"
    fontSize: "clamp(3rem, 8.4vw, 8.75rem)"
    fontWeight: 600
    lineHeight: 0.88
    letterSpacing: "-0.04em"
  statement:
    fontFamily: "Space Grotesk, Helvetica Neue, Arial, sans-serif"
    fontSize: "clamp(2.5rem, 6.3vw, 6.6rem)"
    fontWeight: 500
    lineHeight: 0.96
    letterSpacing: "-0.04em"
  title:
    fontFamily: "Space Grotesk, Helvetica Neue, Arial, sans-serif"
    fontSize: "clamp(1.6rem, 2.6vw, 2.6rem)"
    fontWeight: 600
    lineHeight: 1
    letterSpacing: "-0.035em"
  body:
    fontFamily: "Inter, system-ui, sans-serif"
    fontSize: "clamp(1rem, .96rem + .2vw, 1.125rem)"
    fontWeight: 400
    lineHeight: 1.6
    letterSpacing: "normal"
  label:
    fontFamily: "Inter, system-ui, sans-serif"
    fontSize: "0.75rem"
    fontWeight: 500
    lineHeight: 1.3
    letterSpacing: "0.2em"
rounded:
  sm: "10px"
  md: "14px"
  lg: "22px"
  xl: "28px"
  pill: "999px"
spacing:
  gutter: "clamp(20px, 4.4vw, 72px)"
  grid-gap: "24px"
  section: "clamp(96px, 13vw, 200px)"
components:
  button-primary:
    backgroundColor: "{colors.porcelain}"
    textColor: "{colors.obsidian}"
    rounded: "{rounded.pill}"
    height: "54px"
    padding: "0 10px 0 26px"
  button-primary-hover:
    backgroundColor: "{colors.cobalt}"
    textColor: "#FFFFFF"
  button-cobalt:
    backgroundColor: "{colors.cobalt}"
    textColor: "#FFFFFF"
    rounded: "{rounded.pill}"
    height: "54px"
  button-cobalt-hover:
    backgroundColor: "{colors.signal-lime}"
    textColor: "{colors.obsidian}"
  button-ghost:
    backgroundColor: "transparent"
    textColor: "{colors.porcelain}"
    rounded: "{rounded.pill}"
    height: "54px"
    padding: "0 26px"
  input:
    backgroundColor: "{colors.obsidian}"
    textColor: "{colors.porcelain}"
    rounded: "{rounded.md}"
    height: "52px"
    padding: "14px 16px"
  chip-selected:
    backgroundColor: "{colors.signal-lime}"
    textColor: "{colors.obsidian}"
    rounded: "{rounded.pill}"
    height: "42px"
  stage:
    backgroundColor: "{colors.graphite}"
    rounded: "{rounded.xl}"
---

## Overview

Kinetic Signal: a single sculpted ribbon is the studio's signature, and it travels. It is real WebGL geometry in the hero (silver with cobalt reflections), a drawn line through philosophy, process, contact and footer, and a fold in the section dividers. Whole sections alternate between obsidian and porcelain fields. One cobalt field (philosophy) is the dramatic turn. Type is architectural: Space Grotesk at display scale with tight tracking, and Inter for reading. Both faces were pinned by the client brief.

## Colors

### Primary
Cobalt `#4865FF` is the principal accent. It fills the philosophy field, ribbon bands, progress fills and the contact CTA. Hover deepens to `#304EF0`, or flips to lime on cobalt buttons. Small text never sits in cobalt on obsidian (4.3:1); use lavender for small accent text on dark.

### Secondary
Signal lime `#D8FF57` is reserved for live states: the MOVE signal, active index, focus rings on dark, selected chips, the cursor dot, and the one word "FELT." Never use it as a fill for large regions, and never as text on porcelain.

### Tertiary
Lavender `#A99BFF` carries reflections, secondary strands and outline glyphs on dark.

### Neutral
Obsidian `#090B11` and porcelain `#F6F5F0` are the two fields. Graphite `#151923` is for stages and the form. Ink `#111318` is text on porcelain. For secondary text use `#9299A8` on dark and `#5A5F6B` on light; never use `#9299A8` on porcelain. Hairlines are `#2B303B` on dark and `#D9DAD6` on light.

### Named Rules
- **Field, not accent.** Colour commits at section scale; a section is one field.
- **Lime is a signal.** If lime appears, something is live, selected or focused.

## Typography

Space Grotesk 500–650 for display, Inter 400–600 for text.
- **Hero:** about 150px on 1440, line-height 0.84, tracking -0.045em.
- **Section display:** up to 8.75rem.
- **Statements:** up to 6.6rem.
- **Titles:** 1.6–2.6rem.
- **Body:** 16–18px with a 66ch measure.

Line breaks are composed in markup (one `.ln` per line); never leave display lines to automatic wrapping. Outlined display text uses a 1.5px stroke in the field's text colour.

## Layout

Layout uses a 12-column grid, 24px gaps, a fluid gutter (`clamp(20px, 4.4vw, 72px)`), a 1680px max width and generous section padding.

Three scroll sequences are sticky, and only at 900px and up, with motion allowed: services (340vh), philosophy (230vh) and process (300vh). Below 900px, and under reduced motion, every sequence becomes a native vertical stack.

The selected work section runs on an editorial rhythm: one cinematic 16:8 piece, two staggered 4:5 pieces, then one full-bleed 21:9 piece.

## Elevation & Depth

Depth comes from the fields and the WebGL ribbon, not from shadows. Only three shadows exist:
- **Compact header pill:** `0 10px 30px -12px` black at 45%.
- **Contact form:** a cobalt glow, `0 40px 80px -40px` at 35%.
- **Insight preview:** `0 30px 60px -20px`.

## Shapes

Pills (999px) are for every control. Stages and the form use 22–28px radii, project media 14–24px, and inputs 14px. Ribbon paths always use round caps.

## Components

- **Buttons:** pill-shaped. The label slides inside a mask on hover. A circular arrow badge exits diagonally while a twin enters, and a fill rises from below.
- **Nav links:** directional underline reveal.
- **Header:** transparent over the hero, then a dark pill after 60px of scroll. Below 900px, a burger opens a full-screen clip-reveal menu.
- **Inputs:** obsidian wells on graphite, with a cobalt focus ring and errors inline in `#FFB3A8`.
- **Cursor:** fine pointers only. It has View and Drag states, where Drag appears only on the draggable reel canvas.

## Do's and Don'ts

- **Do** end every scroll sequence in a readable static state, and render content visible when JS fails.
- **Do** label demonstration content as demonstration.
- **Don't** add same-size icon cards, gradient text, or glass panels.
- **Don't** let a ribbon stroke cross running body text.
- **Don't** repeat one fade-up entrance across sections. Each section owns one reveal type: mask, tracking, perspective, clip or stroke draw.
