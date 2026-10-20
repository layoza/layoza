---
name: Noor
description: Local comparison study, extracted from the built page.
colors:
  sage: "#e7ede7"
  ink: "#173c37"
  white: "#fbfaf6"
  line: "#b9c8be"
  muted: "#4d655d"
typography:
  display:
    fontFamily: "Georgia, serif"
    fontSize: "clamp(52px, 5vw, 72px)"
    fontWeight: 400
  body:
    fontFamily: "Arial, sans-serif"
    fontSize: "16px"
    lineHeight: 1.5
rounded:
  control: "40px"
components:
  button-primary:
    backgroundColor: "#173c37"
    textColor: "#fbfaf6"
    rounded: "{rounded.control}"
    padding: "13px 20px"
---

# Design System: Noor

## Overview

**Creative North Star: "A clear first step"**

This records the built fictional comparison page. It is a source extraction, not a visual-review verdict.

## Colors

Frontmatter colors are the source values used in this page. Large areas carry the brief's palette. Text and controls use the darkest foreground with visible keyboard focus.

## Typography

The display face is Georgia, serif; the body uses Arial. Display size is recorded in the frontmatter. Headline size changes at the mobile breakpoint while control labels stay readable.

## Layout

Five-percent outer margins. Welcome and request use a 1.25fr / 1fr split with a nine-percent gap. First-visit and treatment regions use open two-column layouts. At 700px all substantive sections stack with six-percent margins.

## Elevation & Depth

Tonal sage and warm white. The request surface has a quiet offset shadow, 0 14px 35px #173c3708. No animation.

## Shapes

Primary controls use 40px corner radii. Borders clarify editable fields and grouped task surfaces.

## Components

Primary buttons use the frontmatter colors and padding, with a darker or lighter hover response. Visible focus rings use an offset outline. Labels remain visible above inputs. Form confirmations occupy role=status regions. Navigation uses standard anchors to the page sections.

## Do's and Don'ts

- Do preserve the common copy and local-only feedback.
- Do keep all meaningful content visible without animation initialization.
- Do keep the native form controls and visible labels.
- Don't invent factual proof or staff.
- Don't treat this documentation as independent design approval.
