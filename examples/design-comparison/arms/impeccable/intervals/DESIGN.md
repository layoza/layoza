---
name: Intervals
description: Local comparison study, extracted from the built page.
colors:
  ink: "#111111"
  muted: "#626262"
  paper: "#ffffff"
  line: "#cccccc"
typography:
  display:
    fontFamily: "Helvetica, sans-serif"
    fontSize: "clamp(48px, 6.9vw, 96px)"
    fontWeight: 400
  body:
    fontFamily: "Helvetica, sans-serif"
    fontSize: "16px"
    lineHeight: 1.5
rounded:
  control: "0px"
components:
  button-primary:
    backgroundColor: "#111111"
    textColor: "#ffffff"
    rounded: "{rounded.control}"
    padding: "14px 24px"
---

# Design System: Intervals

## Overview

**Creative North Star: "The photographic spread"**

This records the built fictional comparison page. It is a source extraction, not a visual-review verdict.

## Colors

Frontmatter colors are the source values used in this page. Large areas carry the brief's palette. Text and controls use the darkest foreground with visible keyboard focus.

## Typography

The display face is Helvetica, sans-serif; the body uses Helvetica. Display size is recorded in the frontmatter. Headline size changes at the mobile breakpoint while control labels stay readable.

## Layout

Four-percent outer margins. Opening pairs share a 1.1fr / 1fr grid, with an eight-percent gutter. The black exhibition band uses the same grid. At 700px the photographs stay paired, while the title and later sections stack. The later work pair becomes one column.

## Elevation & Depth

Flat monochrome. No shadows or animation. Photograph boundaries stay complete.

## Shapes

Primary controls use 0px corner radii. Borders clarify editable fields and grouped task surfaces.

## Components

Primary buttons use the frontmatter colors and padding, with a darker or lighter hover response. Visible focus rings use an offset outline. Labels remain visible above inputs. Form confirmations occupy role=status regions. Navigation uses standard anchors to the page sections.

## Do's and Don'ts

- Do preserve the common copy and local-only feedback.
- Do keep all meaningful content visible without animation initialization.
- Do keep the native form controls and visible labels.
- Don't invent factual proof or staff.
- Don't treat this documentation as independent design approval.
