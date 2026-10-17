# ui-ux-pro-max arm — Orrery decisions

Searches run (repo root, python .claude/skills/ui-ux-pro-max/scripts/search.py):
1. "planetarium night sky shop tickets" --design-system --variance 7 --motion 5 --density 3
   -> Hero+Features+CTA, Swiss minimalism, green/pink palette (rejected: off-topic for a night-sky venue; skill says re-run with different keywords)
2. "astronomy museum dark cosmic" --design-system (same dials)
   -> Scroll-triggered storytelling pattern, Minimalism & Swiss Style, gallery black palette, Inter + Playfair, GSAP stagger preset (standard tier)
3. "space astronomy dark night" --domain color -> chose "Alarm & World Clock": time amber + night indigo on dark
   (#0F172A bg, #192134 card, #D97706 primary / on-primary #000, #F59E0B secondary, #6366F1 accent, #94A3B8 muted fg)
4. typography: "editorial serif elegant cultural", "scientific vintage editorial" -> Newsreader (News Editorial heading) + Inter (Classic Elegant body); JetBrains Mono for data labels (from design-system font URL)
5. product "museum cultural venue tickets" -> Museum/Gallery: Swiss minimal + motion-driven, storytelling
6. ux: inline validation (validate on blur, aria-describedby errors, focusable error summary), live badge ("3 items in basket", single atomic status)

Signature: animated SVG orrery in hero (CSS rotation, static under prefers-reduced-motion).
Motion: CSS stagger reveal (no GSAP dependency), only applied when JS + IO + no reduced motion.
