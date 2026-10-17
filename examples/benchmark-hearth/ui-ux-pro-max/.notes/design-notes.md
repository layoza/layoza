# Hearth & Rye — ui-ux-pro-max dev notes
Queries run (repo root, python .claude/skills/ui-ux-pro-max/scripts/search.py):
- "restaurant bakery wood-fired fine dining" --design-system --variance 7 --motion 5 --density 3
  -> style Vibrant & Block-based, palette Restaurant/Food (#DC2626 red/#FEF2F2 pink), type Playfair Display SC / Karla.
  Fit check: red/pink block style is off-brief for a wood-fired bakery; kept typography + gold accent idea.
- "editorial luxury organic warm" --domain style -> Nature Distilled (terracotta/sand/clay/cream/olive, grain, soft shadows) -> adopted.
- "restaurant elegant editorial serif" --domain typography -> Restaurant Menu (Playfair Display SC + Karla) -> adopted (+ Playfair Display italic for display).
- landing "restaurant food" / "hero booking reservation": 0 results; retry "Hero-Centric Design" -> hero dominant + sticky nav CTA, static under reduced motion.
- ux: error summary (focusable, role=alert), inline on-blur validation, aria-describedby errors, aria labels, visible focus.
Tokens: cream #F5F0E1, sand #EBE1CC/#D4C4A8, ink #2A1D15, muted #6B5544, soot #1C1410, clay #9A4A1C, ember #E8925A, olive #55622E, error #A3341F.
Contrast checked with contrast.py (all >= 4.8:1 for text pairs).
