# Halden Eyecare - ui-ux-pro-max arm notes

Searches run (skill scripts/search.py):
1. "independent optician eyecare retail boutique craft" --design-system (variance 6, motion 3, density 4)
   -> Minimalism, burgundy #7C2D12 / craft gold #A16207 on #FEF2F2, Rubik/Nunito Sans. Rejected palette (reads as wine/retail, not eye health); kept gold accent idea.
2. "optician eyewear clinic healthcare" --design-system -> Neumorphism (rejected: a11y risk high per DB), Healthcare cyan palette.
3. "healthcare clinic calm trustworthy" --domain color -> Medical Clinic: #0891B2 / #134E4A / #F0FDFA / #CCFBF1. ADOPTED (primary darkened to #0E7490 so white CTA text passes 4.5:1).
4. "editorial elegant serif boutique" --domain typography -> Classic Elegant: Playfair Display + Inter. ADOPTED.
5. ux: "inline validation error near field" (aria-describedby inline errors + focusable error summary), "live badge count screen reader" (one atomic status message, e.g. "3 items in your bag").

Signature moment: hero Snellen-style chart blurred, seen sharp only through a pair of SVG spectacles; lenses "come into focus" on load and follow the pointer on fine-pointer devices. Static under prefers-reduced-motion.

## Verification (2026-10-15)
- Built-in browser pane at 1440x900 / 768x1024 / 375x812: no horizontal overflow (docW <= viewport), mobile menu opens/closes, Escape closes and returns focus, focus moves to first link.
- Scripted interactions: 2 frames + 1 lens -> bag count 3, live status "Varifocal lenses chosen. 3 items in your bag."; empty submit -> 5 inline errors + focused error summary; valid submit -> success message; FAQ toggles aria-expanded/hidden; day select has 14 options (Sundays disabled).
- Fixed during QA: mobile menu clipped by header backdrop-filter (moved nav out of header); menu CTA styling overridden by `.mobile-nav a`; hidden toast peeking at bottom; lens prices inheriting small muted text; spectacle bridge drawn inside lens rings.
- Headless Chrome screenshots in this folder (shot-1440, shot-768, top-*). Headless can't render below ~500px wide, so 375 was checked in the pane only.
