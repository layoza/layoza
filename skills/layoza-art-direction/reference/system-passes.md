<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/bolder.md, reference/typeset.md, reference/colorize.md, reference/layout.md and reference/animate.md. Modified by Layoza: condensed five commands into one pass guide; impeccable detector scans and live-mode parameters replaced by `layoza qa` evidence; added Mode B/C SITE-DNA preservation, metric-adjusted font fallbacks, and Layoza's motion cleanup contract. -->

# System passes: bolder, colour, type, layout, motion

Each pass works **inside** the committed world (or, in Mode B/C, inside the
brand recorded in `SITE-DNA.json` / `DESIGN.md`). If a pass would need a new
identity, stop: that is step 4 of SKILL.md (Mode A/D) or a question for the user.

Every pass ends the same way: answer its verify list with rendered or source
evidence (a selector, a computed value, a screenshot), then rerun `layoza qa`.
A bare "yes" is not verification.

---

## Bolder (a section reads flat)

"Bolder" is an amplification request scoped to something that already exists.
The reflex answer, more effects, is the opposite of bold; reject it first.

- **Scope is sovereign.** Touch only the named target. No new colours, fonts,
  radii, shadows, or primitives the system does not own. If the system truly
  cannot express it, name the exact addition and ask.
- **Why it reads flat:** it quietly opts out of the system's strongest moves
  (display type at full strength, the signature motif, density and pacing).
  Bring it up to what its neighbours already reach, in their vocabulary.
- **Commit, then clarify.** Make one decisive move completely and quiet
  everything around it. If every element got louder, the section got flatter.
- **Give it its own rhythm:** a peak in the scroll, a shift in density or pace.
- **Skeleton test:** strip the copy. Does the bare structure still say what the
  section is and why it matters? If it only works once words return, the
  boldness is in the text size, not the design.
- Verify: nothing outside the target changed; no new primitives; actions work
  the same; the section is the same brand, only more sure of itself.

## Colour

- Audit first: confirmed brand colours, current roles (surface, text, action,
  semantic), places where grayscale hides hierarchy or state, contrast
  failures, colour-only meaning.
- Name the temperature, dominant relationship, contrast range, and dosage
  before editing. Build **roles**, not a bag of swatches: canvas and raised
  surfaces; primary/secondary text; action, focus, selection; borders;
  success/warning/error/info; data scales when needed.
- New palettes in OKLCH. Hue comes from product meaning and the world, never a
  category association. Reduce chroma near white and black.
- **Let the strongest colour own a deliberate region or role.** Keep the primary
  action's colour for the action. On coloured surfaces derive secondary text
  from the surface hue. Neutral gray is valid only when it serves the world.
- Data needs lightness, shape, label, or pattern, not hue alone.
- Dark mode is composed, not inverted. Prefer explicit colours over stacked
  translucent overlays (alpha makes contrast context-dependent).
- Verify: every colour has a role; attention lands on the action; states and
  both themes pass contrast (body 4.5:1, large 3:1, controls/focus 3:1); the
  result is recognisably this product, not "colourful".

## Type

- Assess with evidence: which faces/weights/roles exist and whether they fit
  the world; whether heading, body, label, metadata, and data roles are
  distinguishable at a glance; whether the scale is deliberate; measure;
  stress (long headings, localisation, zoom, narrow containers, fallback).
- State roles, intended contrast, measure, authoritative faces, and constraints
  before editing. Fewest roles and families that make hierarchy unmistakable;
  combine size, weight, space, and tone.
- Body ≥ 16px, 45–75ch, line height tuned inversely to measure. Light text on
  dark: slightly more leading, a touch more tracking, one step more weight.
- Tabular numerals for prices and data; paragraph spacing **or** indent, not both.
- **Delivery:** load only used weights; `font-display: swap` plus a
  metric-adjusted fallback `@font-face` (`size-adjust`, `ascent-override`,
  `descent-override`, `line-gap-override`) so the swap does not shift layout.
  See [engineering-lessons.md](engineering-lessons.md) for the snippet.
- **Never render-block on the font stylesheet.** Add
  `<link rel="preconnect">` to fonts.googleapis.com and fonts.gstatic.com,
  then load the stylesheet with `media="print" onload="this.media='all'"`
  and a `<noscript>` copy. The headline is usually the LCP element. In the
  2026-10 benchmark, Layoza pages with non-blocking fonts measured 0.3–0.6s
  LCP and one with a blocking stylesheet measured 4.0s. Heavy libraries
  (three.js, GSAP) load with `defer` or after the first paint, never ahead of
  the headline.
- Preserve zoom, user font settings, and text scaling.

## Layout

- Assess: squint test (primary, secondary, groups still read when blurred);
  grouping by proximity before containers; rhythm of tight vs generous
  intervals; topology matches content (are equal cards really equal?);
  density per region; adaptation at narrow, intermediate, wide, zoomed;
  DOM/focus order agrees with visual order; sticky elements and safe areas.
- Name the primary path, what groups and what separates, what leads, the
  density, and how structure changes across widths, before editing.
- Group by meaning; use `gap` for sibling rhythm; one documented spacing scale
  (a 4-unit base gives useful middle steps); container-aware components;
  responsive changes are structural (reorder, collapse, reveal), not shrinking.
- Variation is not a goal; break repetition only when content or priority changes.

## Motion

- Find the job: acknowledge an action, make a state change legible, preserve
  continuity, direct attention once, embody the world. Do not animate a static
  area because it exists.
- **Motion thesis** before code: the one focal moment (from this product and
  world; a generic fade-and-rise, hover lift, parallax, or scroll reveal is not
  a thesis), continuity, feedback, budget.
- Choose material by meaning: FLIP/view transitions for continuity; bounded
  blur/filter/shadow for focus; clip-path/mask for reveal; colour or shader for
  material energy when the world and runtime support it. One strong idea.
- Timing: 100–150ms feedback, 150–300ms state change, 200–300ms overlay (full-height sheets ≤ 400ms),
  300–500ms layout change,
  500–800ms one authored entrance. Exit faster than entrance.
  `cubic-bezier(0.16, 1, 0.3, 1)` for confident arrivals; no reflexive bounce.
- Stagger only real lists, with a capped total delay.
- **Content visible by default** so a failed script hides nothing (`layoza qa`
  flags `text-hidden-after-scroll`). Do not animate layout properties; transform,
  FLIP, or grid tricks instead. `will-change` only during the animation.
- Every animation: a `prefers-reduced-motion: reduce` path that removes spatial
  movement but keeps meaningful feedback; loops stop when offscreen; listeners,
  timelines, observers, and GPU objects cleaned up on unmount.
- Pick the tool per design (CSS, WAAPI, GSAP, Motion, View Transitions, canvas/
  WebGL) with `layoza-animations`; in an existing project, prefer its stack.
- Verify: removing any animation would lose meaning or authored character, not
  mere decoration; INP stays ≤ 200ms.
