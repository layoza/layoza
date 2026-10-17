# Design plan (frontend-design skill, pass 1 + review)

Subject: an independent creative studio (brand, web, growth, motion). Audience: marketing/product leads choosing a studio. Primary job: prove craft in the first screen, then route to "Start a project".
Decision (client not asked, per harness): agency is unnamed -> wordmark "Studio Name"; email hello@studio-name.example; projects are 4 labelled demonstration concepts for fictional organisations; testimonials omitted; essays labelled placeholder.

Color (brief pins it exactly): obsidian #090B11, graphite #151923, porcelain #F6F5F0, ink #111318, cobalt #4865FF (principal), lime #D8FF57 (rare: the hero period, active nodes, CTA hover), lavender #A99BFF (small accent text on dark — cobalt on obsidian is only 4.3:1).
Contrast fixes: #9299A8 fails on porcelain -> #5B616E used for secondary text on light. Small text on cobalt uses #FFFFFF (4.58:1); porcelain only for large type there. Small cobalt text on light uses #304EF0 (5.5:1).

Type: Space Grotesk (display, client-named) + Inter (body, client-named). Skill prefers non-default faces, but the client prompt names these, so brief wins. Uppercase tracked section labels are also client-mandated ("01 / WHO WE ARE"), so the skill's no-caps-label rule yields.

Layout: left-aligned editorial 12-col grid; asymmetry everywhere; headlines with hand-set line breaks.
  HERO:   [WE MAKE        ][  3D ribbon   ]
          [IDEAS MOVE.    ][  (WebGL)     ]
          [para + 2 CTAs  ]      scroll |
  SERVICES (sticky, native scroll): [titles stacked | stage w/ 4 coded visuals]
  PROCESS (sticky): [big number + morphing dot artwork | stage list + timeline]

Principles / signature: ONE thing is loud — the folded metallic ribbon. It is a real Three.js sculpture in the hero, and every other section reuses its geometry as 2D: ribbon-curve divider into the intro, twisting bar loop in Motion service, ribbon line behind philosophy, light-painted ribbons in the showreel canvas, cobalt ribbon crossing the contact type, ribbon glyph in wordmark/footer.
Section transitions are each different (curve mask, rounded-inset expand, diagonal fold wipe, stage expand, colour tween) — no shared fade-up preset.

Review against brief / generic defaults:
- First instinct was a gradient-orb hero + three service cards: rejected; brief forbids it and it is the default.
- Considered ScrollTrigger pins for services/process/philosophy: changed to CSS sticky + scroll progress (no pin-spacer hijack, works with Lenis and native touch) — brief warns about excessive pinning.
- Avoided "→" glyph CTAs: arrows are drawn SVG with diagonal motion as the brief asks.
- Avoided middle-dot meta strings and mono labels (skill tells).
- Motion (Framer Motion) skipped: no React here; GSAP + CSS cover component transitions; avoids an extra dependency (brief: no unnecessary deps).
Libraries: GSAP 3.12.5 + ScrollTrigger (cdnjs), Lenis 1.1.13 (unpkg, desktop only, off for reduced motion), three 0.160.0 (jsdelivr ESM, dynamic import after first paint).

## Build log / self-critique
- Pass 1 issues fixed from screenshots: hero CTAs collided with ribbon (stacked under para, ribbon pushed right); philosophy swap invisible (from-tweens + invalidateOnRefresh recorded hidden end state; GSAP folded CSS translate into y) -> fromTo + width driven from timeline time; demo-note specificity; 5px mobile overflow from accordion chevron; dialog focus stolen by GSAP transform parse -> CSS keyframe; Escape close handler.
- Tablet hero recomposed (ribbon on top, type below) instead of shrunk desktop.
- Verified: 375/390/768/1024/1440/1920 no horizontal overflow; reduced-motion static fallbacks; no console errors; menu, accordion, dialog, form validation + mailto, reel pause, keyboard focus ring.
- Note: port 8765 was already serving another arm's page; a DOM snippet from it was printed once by accident before I switched to my own port. Not used.
