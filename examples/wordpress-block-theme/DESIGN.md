---
version: "alpha"
name: "wordpress-block-theme"
description: "Extracted and calibrated design system for wordpress-block-theme (wordpress-gutenberg)"
colors:
  canvas: "#F8F7F4"
  ink: "#141619"
  accent: "#1849A9"
  surface: "#FFFFFF"
typography:
  display:
    fontFamily: ""Instrument Serif", Georgia, serif"
    fontSize: "1rem"
    fontWeight: "400"
    lineHeight: "1.5"
    letterSpacing: "0em"
  body:
    fontFamily: ""Plus Jakarta Sans", system-ui, sans-serif"
    fontSize: "1rem"
    fontWeight: "400"
    lineHeight: "1.5"
    letterSpacing: "0em"
rounded:
  lg: "20px"
  md: "12px"
  sm: "6px"
spacing:
  lg: "32px"
  md: "16px"
  section: "96px"
  sm: "8px"
  xl: "64px"
  xs: "4px"
components:
  button-primary:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.canvas}"
    rounded: "{rounded.lg}"
    padding: "12px"
---

## Overview

`wordpress-block-theme` uses the `wordpress-gutenberg` framework with Design Variance `4/10`, Motion Intensity `3/10`, and Visual Density `5/10`. Preservation mode is `true`.

## Colors

- **canvas (`#F8F7F4`)**: WordPress theme.json palette token (Canvas Background) _(Source: `theme.json`, Tier: `ObservedFact`)_
- **ink (`#141619`)**: WordPress theme.json palette token (Primary Ink) _(Source: `theme.json`, Tier: `ObservedFact`)_
- **accent (`#1849A9`)**: WordPress theme.json palette token (Brand Accent) _(Source: `theme.json`, Tier: `ObservedFact`)_
- **surface (`#FFFFFF`)**: WordPress theme.json palette token (Card Surface) _(Source: `theme.json`, Tier: `ObservedFact`)_

## Typography

- **display**: `"Instrument Serif", Georgia, serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)
- **body**: `"Plus Jakarta Sans", system-ui, sans-serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)

## Layout

- **Max Content Widths**: 1280px
- **Grid Systems**: CSS Grid, Flexbox

## Elevation & Depth

- **md**: `0 10px 30px -10px rgba(15, 23, 42, 0.10)`
- **sm**: `0 1px 2px rgba(15, 23, 42, 0.06)`

## Shapes

- **lg**: `20px`
- **md**: `12px`
- **sm**: `6px`

## Components

- **button-primary**: Primary interactive call-to-action using brand accent and accessible foreground contrast.

## Do's and Don'ts

- **Do**: Preserve existing design tokens, spacing rhythm, and component hierarchy when adding sections or pages.
- **Do**: Honor `prefers-reduced-motion: reduce` and maintain WCAG 2.2 AA contrast ratios.
- **Don't**: Introduce conflicting CSS frameworks, unrequested SaaS purple gradients, or nested card clutter.
