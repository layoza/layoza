# Tidefest — ui-ux-pro-max design notes

Queries run (repo root, python .claude/skills/ui-ux-pro-max/scripts/search.py):
- "music festival event bold animated nightlife" --design-system -p Tidefest --variance 9 --motion 9 --density 4
  -> Hero-Centric pattern, Brutalism style, purple #7C3AED / orange #EA580C on light #FAF5FF, Inter/Playfair, GSAP magnetic hover (Complex tier). Avoid: "Confusing registration + No countdown".
- --domain color "music festival nightlife vibrant dark" -> Music Creation dark palette (#0F172A bg, #7C3AED primary).
- --domain typography "bold poster display kinetic" -> "Kinetic Motion" Syncopate + Space Mono (best for: music festivals); "Kinetic Brutalism" Space Grotesk.
- --domain gsap "scroll pin kinetic text" -> SplitText char stagger; pinned scrub timeline (max 1-2 pins); subtle scroll reveal.
- --domain ux "reduced motion" -> honor prefers-reduced-motion, final readable state, no scroll-jacking.
- --domain landing "event conference landing" -> Hero(date/location/countdown) > speakers > agenda > register; sticky CTA; exact deadline as text.
- --domain style "motion-driven kinetic" -> Motion-Driven style, transform-only, IO, reduced-motion.

Synthesis (deviations noted):
- Style: Motion-Driven + Brutalist block structure (sharp 0px corners, visible rules, big type). Brutalism's "instant transitions" effect dropped because the brief makes motion the brand.
- Palette: kept the design-system hues (violet primary + orange CTA) but moved to the dark mode variant from the colour search because a night-beach festival with sunrise sets reads dark:
  bg #0C0A1D, surface #16132D, fg #F5F3FF, muted #A5A3C2, violet #A78BFA (text-safe on dark), violet deep #7C3AED, orange CTA #F97316 (black text), sunrise #FDBA74, error #F87171, success #4ADE80.
- Type: Syncopate 700 (display, from "Kinetic Motion"), Space Grotesk (body; Space Mono is tiring for paragraphs), Space Mono (numbers, labels).
- Motion: GSAP 3.13.0 + ScrollTrigger from jsdelivr, one pinned horizontal "tide" sequence (desktop + motion only), manual char split for hero, magnetic CTAs, scroll-velocity marquee, canvas tide that reacts to pointer and to scroll (sun rises).
- Reduced motion: canvas draws one static frame, no pin (panels stack), marquee static, no char animation; all content visible without JS.

## QA (qa.py, Python Playwright Chromium; results in qa-report.json, shots/)
- 1440x900, 768x1024, 375x812, plus reduced-motion at 375 and 1440: no console errors, no horizontal overflow.
- Day filter (Sat -> 6 acts), countdown ticks, stepper total (2 Weekend + 1 Camping - 1 Weekend = £440), checkout message, FAQ expand, email invalid -> error, valid -> success, mobile menu open + Escape close: all pass at every viewport.
- Pinned horizontal days sequence active only at >=901px with motion allowed; stacks otherwise.
- Fixes after first pass: footer word overflow, pin layout leaving empty viewport, small mobile title.
- Open: 14 Aug 2027 is actually a Saturday; brief requires Fri/Sat/Sun filter, so weekday labels follow the brief and the countdown copy avoids naming the weekday.
