---
name: Hearth
description: Local comparison study, extracted from the built page.
colors:
  paper: "#f5f3ee"
  white: "#fffefa"
  ink: "#292b28"
  muted: "#676963"
  brick: "#963d28"
  rail: "#713321"
  line: "#d9d8cf"
  green: "#32634e"
  amber: "#815921"
typography:
  display:
    fontFamily: "Arial, sans-serif"
    fontSize: "32px"
    fontWeight: 500
  body:
    fontFamily: "Arial, sans-serif"
    fontSize: "14px"
    lineHeight: 1.5
rounded:
  control: "4px"
components:
  button-primary:
    backgroundColor: "#963d28"
    textColor: "#ffffff"
    rounded: "{rounded.control}"
    padding: "10px 16px"
---

# Design System: Hearth

## Overview

**Creative North Star: "The working shift"**

This records the built fictional comparison page. It is a source extraction, not a visual-review verdict.

## Colors

Frontmatter colors are the source values used in this page. Large areas carry the brief's palette. Text and controls use the darkest foreground with visible keyboard focus.

## Typography

The display face is Arial, sans-serif; the body uses Arial. Display size is recorded in the frontmatter. A fixed task-oriented scale keeps quantities, states and actions easy to scan.

## Layout

A 200px brick rail and a fluid workspace. The workspace has 36px desktop padding. Production and support use a fluid column plus 280px support column with a 28px gutter. At 1150px support falls below production; at 700px the rail becomes a top navigation and batch rows regroup to keep quantities and next actions visible.

## Elevation & Depth

Warm paper surfaces and fine rules. Only the native add-batch dialog floats, with an offset 0 20px 80px #292b2833 shadow.

## Shapes

Primary controls use 4px corner radii. Borders clarify editable fields and grouped task surfaces.

## Components

Primary buttons use the frontmatter colors and padding, with a darker or lighter hover response. Visible focus rings use an offset outline. Labels remain visible above inputs. Form confirmations occupy role=status regions. Filtering uses pressed-state buttons; the native dialog preserves expected keyboard behavior.

## Do's and Don'ts

- Do preserve the common copy and local-only feedback.
- Do keep all meaningful content visible without animation initialization.
- Do keep the native form controls and visible labels.
- Don't invent factual proof or staff.
- Don't treat this documentation as independent design approval.
