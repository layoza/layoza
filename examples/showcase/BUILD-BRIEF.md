# Showcase build brief (shared by all three demos)

Goal: one single-file page per niche that a world-class studio would sign. Each page must look
like it came from a different studio: different colour, type, page shape and motion. QA-clean is
the floor, not the goal. If it reads as a template, it failed.

## Inputs
- Your dealt direction is in `<niche>.txt` in this folder (the block named in your task). The
  `Page shape (binding)` lines are binding: build that hero frame, flow, section heads, ground,
  material, labels and motion techniques as written.
- Read before building: `../../skills/layoza/references/what-wins.md` and the section
  "The bar: best in the world, never a template" in `../../skills/layoza-art-direction/SKILL.md`.

## Hard rules
1. Output: `<niche>/index.html` (one file: inline CSS and JS), plus `<niche>/direction.md` with
   `PURPOSE:`, `PICKED: auto — <direction>, <why it fits the niche>`, `SHAPE:` (the dealt axes),
   3 `ROUND` lines (what you changed after looking at screenshots), and `REVIEWER:` (an honest
   fresh-eyes score out of 10 with the weakest point).
2. External scripts only from `cdn.jsdelivr.net/npm/`, `cdnjs.cloudflare.com` or `unpkg.com`,
   with exact pinned versions (e.g. `three@0.160.0`, `gsap@3.12.5`). Fonts only from Google Fonts
   (`fonts.googleapis.com`). No other network requests. Photos: only the approved free Unsplash
   files already in `<niche>/img/` (reference them as `img/<file>.jpg`, `loading="lazy"` except
   the hero, real `alt`, `width`/`height` set). Do not download anything. No real brands on
   devices: phones, earbuds and chargers are fictional and drawn (Three.js, SVG, canvas, CSS).
3. Every section unique: no two sections share a layout; no reflex fillers (logo wall, three
   icon cards, testimonial carousel, stat row, generic CTA band). Every section leads with ONE
   dominant visual moment drawn from its own subject (photo, 3D object, scene, illustration,
   diagram drawn large, or type as image) covering at least a quarter of the section. Real
   data (prices, times, specs) lives in only the 1-2 sections where the visitor needs it; never
   build the page from stacked rows, lists and tables, and never repeat one card style across
   sections. It must look like a world-class studio's site, not a spreadsheet. Nav and footer
   are authored for this page too.
4. Motion is heavy and real: build the dealt techniques with the named tools. Every effect has a
   static final frame that shows without JS and under `prefers-reduced-motion: reduce`; no text
   is hidden until a script runs (start states are applied by JS only after it loads). Pause
   WebGL/canvas off-screen; clean up listeners.
5. WCAG 2.2 AA: body contrast >= 4.5:1, visible `:focus-visible`, landmarks, skip link, labelled
   controls, 16px body minimum, 12px label minimum. Works at 375px with no horizontal scroll and
   the main action reachable on the first screen; mobile menu works.
6. Fictional brand names only. No real brands, logos or people.

## Done means
- `../../target/release/layoza.exe qa <niche>/index.html` shows 0 errors and 0 warnings on the
  final file, including UI-LEDGER-HEAVY, UI-VISUAL-STARVED and UI-REPEATED-CARD (run it from this folder; it writes `<niche>/qa/`).
- You opened `qa/desktop.png`, `qa/mobile-hero.png` and `qa/mobile-menu-open.png` and they look
  finished: no voids, no overlap, text over 3D readable.
- Report: the QA summary line, which techniques/tools shipped, any deviation from the dealt shape
  and why, and the 3 weakest things about the page (be brutal).
