---
version: "alpha"
name: "existing-site-preservation"
description: "Extracted and calibrated design system for existing-site-preservation (html-css-js)"
colors:
  color-bg-parchment: "#F6F1EA"
  color-surface-cream: "#FFFDF9"
  color-ink-espresso: "#1E1815"
  color-muted-umber: "#5C4E46"
  color-accent-terracotta: "#B84A27"
  color-border-sand: "#DFD5C7"
typography:
  font-display:
    fontFamily: "Cormorant Garamond", Georgia, serif"
    fontSize: "1rem"
    fontWeight: "400"
    lineHeight: "1.5"
    letterSpacing: "0em"
  font-body:
    fontFamily: "DM Sans", -apple-system, BlinkMacSystemFont, sans-serif"
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
    backgroundColor: "{colors.color-ink-espresso}"
    textColor: "{colors.color-bg-parchment}"
    rounded: "{rounded.lg}"
    padding: "12px"
---

## Overview

`existing-site-preservation` uses the `html-css-js` framework with Design Variance `4/10`, Motion Intensity `3/10`, and Visual Density `5/10`. Preservation mode is `true`.

## Colors

- **color-bg-parchment (`#F6F1EA`)**: CSS custom property `--color-bg-parchment` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-surface-cream (`#FFFDF9`)**: CSS custom property `--color-surface-cream` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-ink-espresso (`#1E1815`)**: CSS custom property `--color-ink-espresso` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-muted-umber (`#5C4E46`)**: CSS custom property `--color-muted-umber` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-accent-terracotta (`#B84A27`)**: CSS custom property `--color-accent-terracotta` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-border-sand (`#DFD5C7`)**: CSS custom property `--color-border-sand` _(Source: `index.html`, Tier: `ObservedFact`)_

## Typography

- **font-display**: `Cormorant Garamond", Georgia, serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)
- **font-body**: `DM Sans", -apple-system, BlinkMacSystemFont, sans-serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)

## Layout

- **Max Content Widths**: 1180px, 60ch
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
