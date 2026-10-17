# Dev notes (ui-ux-pro-max arm)

## Skill queries run
- `--design-system "creative agency portfolio immersive motion" --variance 8 --motion 9 --density 3` -> pattern "Scroll-Triggered Storytelling" (kept), style Brutalism + pink/cyan palette + Archivo (overridden: client prompt fixes colours and fonts).
- `--domain gsap "pinned scroll scrub section"` -> max 1-2 pins, scrub 0.5-1.5, refresh after fonts.
- `--stack threejs "render loop offscreen dispose"` -> rAF/setAnimationLoop only, dispose geometry/material.
- `--domain ux "reduced motion parallax"` -> final readable state, no scroll-jacking under reduced motion.
- `--domain ux "mobile menu focus trap"` -> visible focus rings on every control incl. modal.
- `--domain gsap "custom cursor magnetic hover"` -> quickTo, clamp pull ~0.3, limit to focal elements (elastic ease rejected: brief forbids bounce).

## Decisions
- Client palette used verbatim. Derived: `#5A606C` muted text on porcelain (brief #9299A8 is 2.7:1 on porcelain), `#304EF0` (cobalt hover) for small cobalt text on light (5.5:1), white for small text on cobalt (4.6:1).
- Two sticky sequences only on desktop (services, philosophy); process uses a sticky artwork column only. Mobile: none.
- Testimonials omitted (none genuine). Projects and insights are labelled demonstrations.
