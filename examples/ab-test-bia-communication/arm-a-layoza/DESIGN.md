---
version: "alpha"
name: "arm-a-layoza"
description: "Extracted and calibrated design system for arm-a-layoza (html-css-js)"
colors:
  color-bg-limestone: "#F7F6F2"
  color-surface-warm: "#EFECE6"
  color-surface-card: "#FFFFFF"
  color-ink-slate: "#14181D"
  color-muted-slate: "#4F5864"
  color-accent-emerald: "#0F3D3E"
  color-accent-terracotta: "#B84A27"
  color-hairline: "#D8D3C8"
typography:
  font-display:
    fontFamily: "Instrument Serif", Georgia, "Times New Roman", serif"
    fontSize: "1rem"
    fontWeight: "400"
    lineHeight: "1.5"
    letterSpacing: "0em"
  font-body:
    fontFamily: "Public Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif"
    fontSize: "1rem"
    fontWeight: "400"
    lineHeight: "1.5"
    letterSpacing: "0em"
  font-mono:
    fontFamily: "JetBrains Mono", Consolas, monospace"
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
    backgroundColor: "{colors.color-ink-slate}"
    textColor: "{colors.color-bg-limestone}"
    rounded: "{rounded.lg}"
    padding: "12px"
---

## Overview

`arm-a-layoza` uses the `html-css-js` framework with Design Variance `4/10`, Motion Intensity `3/10`, and Visual Density `5/10`. Preservation mode is `true`.

## Colors

- **color-bg-limestone (`#F7F6F2`)**: CSS custom property `--color-bg-limestone` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-surface-warm (`#EFECE6`)**: CSS custom property `--color-surface-warm` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-surface-card (`#FFFFFF`)**: CSS custom property `--color-surface-card` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-ink-slate (`#14181D`)**: CSS custom property `--color-ink-slate` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-muted-slate (`#4F5864`)**: CSS custom property `--color-muted-slate` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-accent-emerald (`#0F3D3E`)**: CSS custom property `--color-accent-emerald` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-accent-terracotta (`#B84A27`)**: CSS custom property `--color-accent-terracotta` _(Source: `index.html`, Tier: `ObservedFact`)_
- **color-hairline (`#D8D3C8`)**: CSS custom property `--color-hairline` _(Source: `index.html`, Tier: `ObservedFact`)_

## Typography

- **font-display**: `Instrument Serif", Georgia, "Times New Roman", serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)
- **font-body**: `Public Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)
- **font-mono**: `JetBrains Mono", Consolas, monospace` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)

## Layout

- **Max Content Widths**: 58ch, 60ch, 900px) {, var(--max-w)
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
