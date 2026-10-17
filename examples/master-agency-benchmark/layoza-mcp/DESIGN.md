---
version: "alpha"
name: "layoza-mcp"
description: "Extracted and calibrated design system for layoza-mcp (html-css-js)"
colors:
  bg-primary: "#0D0E11"
  bg-surface: "#17191E"
  bg-elevated: "#21242C"
  bg-glass: "rgba(23, 25, 30, 0.82)"
  text-primary: "#F5F3EF"
  text-secondary: "#B8BCC6"
  text-muted: "#9CA3AF"
  accent-primary: "#FF4D2D"
  accent-secondary: "#E2F163"
  accent-tertiary: "#38BDF8"
  border-subtle: "rgba(245, 243, 239, 0.12)"
  border-strong: "rgba(245, 243, 239, 0.28)"
typography:
  font-display:
    fontFamily: "Syne', system-ui, -apple-system, sans-serif"
    fontSize: "1rem"
    fontWeight: "400"
    lineHeight: "1.5"
    letterSpacing: "0em"
  font-sans:
    fontFamily: "Plus Jakarta Sans', system-ui, -apple-system, sans-serif"
    fontSize: "1rem"
    fontWeight: "400"
    lineHeight: "1.5"
    letterSpacing: "0em"
  font-mono:
    fontFamily: "Space Mono', monospace"
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
    backgroundColor: "{colors.text-primary}"
    textColor: "{colors.bg-primary}"
    rounded: "{rounded.lg}"
    padding: "12px"
---

## Overview

`layoza-mcp` uses the `html-css-js` framework with Design Variance `4/10`, Motion Intensity `7/10`, and Visual Density `5/10`. Preservation mode is `true`.

## Colors

- **bg-primary (`#0D0E11`)**: CSS custom property `--bg-primary` _(Source: `index.html`, Tier: `ObservedFact`)_
- **bg-surface (`#17191E`)**: CSS custom property `--bg-surface` _(Source: `index.html`, Tier: `ObservedFact`)_
- **bg-elevated (`#21242C`)**: CSS custom property `--bg-elevated` _(Source: `index.html`, Tier: `ObservedFact`)_
- **bg-glass (`rgba(23, 25, 30, 0.82)`)**: CSS custom property `--bg-glass` _(Source: `index.html`, Tier: `ObservedFact`)_
- **text-primary (`#F5F3EF`)**: CSS custom property `--text-primary` _(Source: `index.html`, Tier: `ObservedFact`)_
- **text-secondary (`#B8BCC6`)**: CSS custom property `--text-secondary` _(Source: `index.html`, Tier: `ObservedFact`)_
- **text-muted (`#9CA3AF`)**: CSS custom property `--text-muted` _(Source: `index.html`, Tier: `ObservedFact`)_
- **accent-primary (`#FF4D2D`)**: CSS custom property `--accent-primary` _(Source: `index.html`, Tier: `ObservedFact`)_
- **accent-secondary (`#E2F163`)**: CSS custom property `--accent-secondary` _(Source: `index.html`, Tier: `ObservedFact`)_
- **accent-tertiary (`#38BDF8`)**: CSS custom property `--accent-tertiary` _(Source: `index.html`, Tier: `ObservedFact`)_
- **border-subtle (`rgba(245, 243, 239, 0.12)`)**: CSS custom property `--border-subtle` _(Source: `index.html`, Tier: `ObservedFact`)_
- **border-strong (`rgba(245, 243, 239, 0.28)`)**: CSS custom property `--border-strong` _(Source: `index.html`, Tier: `ObservedFact`)_

## Typography

- **font-display**: `Syne', system-ui, -apple-system, sans-serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)
- **font-sans**: `Plus Jakarta Sans', system-ui, -apple-system, sans-serif` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)
- **font-mono**: `Space Mono', monospace` at `1rem` (weight `400`, line-height `1.5`, tracking `0em`)

## Layout

- **Max Content Widths**: 1024px) {, 1120px) {, 34ch, 56ch, 58ch, 768px) {, 900px) {, 920px
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
