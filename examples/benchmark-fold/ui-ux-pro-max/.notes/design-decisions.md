# Fold — ui-ux-pro-max arm notes

## Toolkit queries (run from skill repo root)
- `--design-system "architecture studio portfolio minimal quiet" --variance 3 --motion 1 --density 3`
  - Style: Minimalism & Swiss Style (kept).
  - Pattern: Scroll-Triggered Storytelling (rejected: client forbids animation beyond hover/focus).
  - Colors: pure black bg (rejected: "quiet" brief + architecture drawings read better on paper).
  - Typography: Archivo / Space Grotesk (kept).
  - Motion snippet: GSAP scroll reveal (rejected per client brief; no GSAP).
- `--domain color "architecture firm warm paper"` -> Architecture / Interior: #171717 / #404040 / accent #A16207 (kept, bg warmed to paper).
- `--domain landing "portfolio filter case studies"` -> Portfolio Grid: Hero > Project Grid > About/Philosophy > Contact, filter by category (kept).
- `--domain ux "inline validation form error"` -> blur validation, inline errors w/ aria-describedby, focusable error summary (kept).
- `--domain ux "mobile menu hamburger focus"` -> visible 2px focus rings (kept).

## Palette
paper #F6F4EF, ink #171717, ink-2 #404040, muted #57534E, rule #D6D1C7, ochre #A16207 (decorative + large only; small text uses ink), error #B42318, success #166534.

## Signature
A "drawing set": every project is a thin-line SVG plan/section/elevation on a sheet with a sheet number (A-01..A-06) and scale bar. The wordmark carries a folded-corner mark.
