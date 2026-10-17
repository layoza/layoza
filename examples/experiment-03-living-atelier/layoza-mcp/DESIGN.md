---
version: "alpha"
name: "layoza-mcp"
description: "Extracted and calibrated design system for layoza-mcp (html-css-js)"
colors:
  bg-ivory: "#FAF7F2"
  bg-porcelain: "#F2ECE1"
  bg-alabaster: "#EAE2D3"
  bg-cobalt-stage: "#081229"
  ink-midnight: "#0B1021"
  ink-body: "#252D42"
  ink-muted: "#525B73"
  accent-cobalt: "#0047FF"
  accent-cobalt-deep: "#0033BF"
  accent-vermilion: "#E63B20"
  border-hairline: "rgba(11, 16, 33, 0.13)"
  border-strong: "rgba(11, 16, 33, 0.28)"
typography:
  font-display:
    fontFamily: "Playfair Display', Georgia, serif"
    fontSize: "1rem"
    fontWeight: "400"
    lineHeight: "1.5"
    letterSpacing: "0em"
  font-sans:
    fontFamily: "Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, sans-serif"
    fontSize: "1rem"
    fontWeight: "400"
    lineHeight: "1.5"
    letterSpacing: "0em"
  font-mono:
    fontFamily: "JetBrains Mono', monospace"
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
    backgroundColor: "{colors.ink-midnight}"
    textColor: "{colors.bg-ivory}"
    rounded: "{rounded.lg}"
    padding: "12px"
---

## Overview

`layoza-mcp` uses the `html-css-js` framework with Design Variance `4/10`, Motion Intensity `3/10`, and Visual Density `5/10`. Preservation mode is `true`.

## Colors

- **bg-ivory (`#FAF7F2`)**: CSS custom property `--bg-ivory` _(Source: `index.html`, Tier: `ObservedFact`)_
- **bg-porcelain (`#F2ECE1`)**: CSS custom property `--bg-porcelain` _(Source: `index.html`, Tier: `ObservedFact`)_
- **bg-alabaster (`#EAE2D3`)**: CSS custom property `--bg-alabaster` _(Source: `index.html`, Tier: `ObservedFact`)_
- **bg-cobalt-stage (`#081229`)**: CSS custom property `--bg-cobalt-stage` _(Source: `index.html`, Tier: `ObservedFact`)_
- **ink-midnight (`#0B1021`)**: CSS custom property `--ink-midnight` _(Source: `index.html`, Tier: `ObservedFact`)_
- **ink-body (`#252D42`)**: CSS custom property `--ink-body` _(Source: `index.html`, Tier: `ObservedFact`)_
- **ink-muted (`#525B73`)**: CSS custom property `--ink-muted` _(Source: `index.html`, Tier: `ObservedFact`)_
- **accent-cobalt (`#0047FF`)**: CSS custom property `--accent-cobalt` _(Source: `index.html`, Tier: `ObservedFact`)_
- **accent-cobalt-deep (`#0033BF`)**: CSS custom property `--accent-cobalt-deep` _(Source: `index.html`, Tier: `ObservedFact`)_
- **accent-vermilion (`#E63B20`)**: CSS custom property `--accent-vermilion` _(Source: `index.html`, Tier: `ObservedFact`)_
- **border-hairline (`rgba(11, 16, 33, 0.13)`)**: CSS custom property `--border-hairline` _(Source: `index.html`, Tier: `ObservedFact`)_
- **border-strong (`rgba(11, 16, 33, 0.28)`)**: CSS custom property `--border-strong` _(Source: `index.html`, Tier: `ObservedFact`)_

## Typography

- **font-display**: `Playfair Display', Georgia, serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)
- **font-sans**: `Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, sans-serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)
- **font-mono**: `JetBrains Mono', monospace` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)

## Layout

- **Max Content Widths**: 100%, 1080px) {, 1120px, 1380px, 17.5ch, 54ch, 58ch, 64ch, 65ch, 760px, 767px) {
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
