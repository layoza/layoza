---
name: Signal Studio
colors:
  obsidian: "#090B11"
  graphite: "#151923"
  porcelain: "#F6F5F0"
  ink: "#111318"
  muted-dark: "#9299A8"
  muted-light: "#555B67"
  cobalt: "#4865FF"
  cobalt-hover: "#304EF0"
  lime: "#D8FF57"
  lavender: "#A99BFF"
typography:
  display: { fontFamily: "Space Grotesk", fontWeight: 600, letterSpacing: "-0.04em", lineHeight: 0.92 }
  body: { fontFamily: "Inter", fontSize: "17px", fontWeight: 400, lineHeight: 1.55 }
  label: { fontFamily: "Inter", fontSize: "12px", fontWeight: 600, letterSpacing: "0.2em" }
rounded:
  pill: "999px"
  card: "28px"
spacing:
  gutter: "clamp(20px,4.4vw,72px)"
  section: "clamp(104px,12vw,200px)"
components:
  button-primary: { backgroundColor: "{colors.porcelain}", textColor: "{colors.ink}", rounded: "{rounded.pill}", height: "52px" }
  button-cobalt: { backgroundColor: "{colors.cobalt-hover}", textColor: "{colors.porcelain}", rounded: "{rounded.pill}", height: "52px" }
---

# Signal Studio (Kinetic Signal) - Design System

## Colors
- Obsidian `#090B11` (primary dark ground), Graphite `#151923` (raised dark surface)
- Porcelain `#F6F5F0` (light ground), Ink `#111318` (text on light)
- Secondary text: `#9299A8` on dark only; `#555B67` on porcelain (6.2:1)
- Borders: `#2B303B` dark, `#D9DAD6` light
- Cobalt `#4865FF` (brand, large text/fills), Cobalt hover `#304EF0` (small accent text on light, buttons with white text)
- Lime `#D8FF57` (signal accent, focus ring on dark, sparing), Lavender `#A99BFF` (small accent on dark)

## Typography
- Display: Space Grotesk 500-700, tracking -0.04em, line-height 0.86-0.98
- Body: Inter 400-600, 17px base, line-height 1.55
- Scale: xl clamp(3.75rem,9.8vw,9.25rem); l clamp(2.9rem,7.2vw,7rem); m clamp(2.1rem,4.4vw,4.25rem); s clamp(1.45rem,2.2vw,2rem)
- Labels: 12px uppercase, 0.2em tracking

## Spacing
- Side gutter clamp(20px,4.4vw,72px); section padding clamp(104px,12vw,200px); 12-column grid, gap 20-48px

## Components
- Pill buttons (52px, 44px small, 76px big): masked text slide, diagonal arrow swap, internal circle fill, magnetic on fine pointers
- Ribbon fold section transitions (SVG bulge scrubbed flat)
- Signal rail: fixed right progress thread with section number
- Staged sticky lists (services, process) on desktop with stacked fallback

## Motion
- Signature ease cubic-bezier(.19,1,.22,1) (expo.out); move ease cubic-bezier(.77,0,.175,1)
- Durations: quick 180ms, standard 320ms, slow 900ms; reveals finish within 0.9s
- Hero entrance is CSS (content visible without JS); scroll work in GSAP ScrollTrigger inside matchMedia
- Lenis on fine pointers only; Three.js ribbon lazy-loaded, paused off-screen, disposed on pagehide
- prefers-reduced-motion: all animation disabled, final states shown

## Accessibility
- WCAG 2.2 AA contrast, lime/cobalt focus rings, 44px targets, skip link, landmarks, modal menu with focus trap and Escape
