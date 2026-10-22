---
name: layoza-animations
description: Web motion designed per brief: authored choreography, scroll, interaction and Three.js state with the tool that fits (CSS, WAAPI, GSAP, Motion, WebGL), reduced-motion stills and scoped cleanup. The user's prompt wins.
---

# Layoza Specialist Skill: Motion Design (`layoza-animations`)

Follow [the motion policy](../layoza/references/motion-policy.md) and the tool policy in § 2. Where GSAP fits, the reusable [GSAP lifecycle controller](assets/gsap-motion.mjs) handles scoped timelines, dynamic reduced motion, pause/resume, offscreen/hidden rendering and cleanup.

For rich-motion briefs, use [section choreography](reference/section-choreography.md): author content-specific transformations through MCP `recommend_animation.motion_plan`, implement them with the executable [responsive section host](assets/section-motion.mjs) (an ES module: for a page opened over file://, inline it as a classic script that assigns `window.createSectionMotion`), then capture actual scroll states with `scripts/review-motion.mjs`. Several earned mechanisms may serve one thesis; "one signature" never excuses identical reveals across a high-motion brief. Authored plans and passing interaction tests are not motion-quality evidence.

Motion is a design decision, made per brief. Section 3 is the shared guardrail;
section 2e applies when the brief requests rich motion. Other techniques and
numeric values are starting points. The user's prompt always wins (§ 2). In
Mode B/C, extend the site's existing stack and `MOTION.md`.

## 1. Decide what the design needs

Ask what motion would make *this* subject and *this* audience understand or
feel more, then choose only that. Possible layers:

| Layer | Use when |
| :--- | :--- |
| Load choreography | The first screen has art or type that gains from arriving in order |
| Scroll story (pinned or scrubbed) | Something in the subject's world is best *shown* changing (a lens sharpening, a loaf rising, a product assembling) |
| Content-shaped section entrances | Sections have a natural motion (a grid cascades, numbers count, a line draws) |
| Micro-interactions | Anything people press, drag, add or open often |
| Page/route transitions | Multi-page or app-like flows |
| Ambient / 3D | The world itself is spatial or atmospheric and the budget allows |

A quiet, typographic brief may need only micro-interactions. A showcase may
need all of them. One identical fade-up on every section is never the answer
(`CRAFT-SCATTERED-REVEAL`).

## 2. Tool policy: choose per brief, one owner per property

Layoza's one motion-tool policy (other skills point here): the user's prompt wins on any tool, feel or "no animation"; otherwise design comes first and the tool follows it. Choose CSS (incl. scroll-driven), WAAPI, GSAP, Motion for React or WebGL per brief; none is mandated. GSAP + ScrollTrigger is the documented default helper for complex scroll choreography and a live Three.js ticker, never a requirement. CSS holds the readable static state and immediate focus/selection feedback.

One owner per animated property: two engines never write the same transform (use wrappers). Lenis is optional for a requested smooth-scroll showcase (see § 4). WebGL renders a real subject, not a floating primitive.

## 2b. Values that feel professional

First pick a motion personality and derive the tokens from it, then apply the
timing physics, choreography and interaction rules in
[reference/motion-system.md](reference/motion-system.md).

These are starting points; tune them to the world. Sources: emilkowalski/skills,
Leonxlnx/taste-skill and greensock/gsap-skills (all MIT), plus GSAP docs.

| Token | Value |
| :--- | :--- |
| Ease out (UI arrivals) | `cubic-bezier(0.23, 1, 0.32, 1)`; GSAP `power4.out` (quint) or `expo.out` for longer travel |
| Ease in-out (moves on screen) | `cubic-bezier(0.77, 0, 0.175, 1)`; GSAP `power3.inOut` (quart). GSAP power1–4 = quad, cubic, quart, quint |
| Drawer / sheet | `cubic-bezier(0.32, 0.72, 0, 1)` |
| Press | 100–160ms, `scale(0.97)` |
| Hover / colour | 150–200ms, `ease` |
| Dropdown / tooltip | 150–200ms, from the trigger's origin, start at scale 0.95 (never 0) |
| Modal / drawer | 200–300ms (full-height sheets up to 400ms); exits about 70% of the entrance |
| Hero line reveal | mask lines, `yPercent: 105 → 0`, 0.9–1.1s, `expo.out`, stagger 0.08–0.11s |
| Image shutter | `clip-path: inset(0 0 100% 0) → inset(0)` 1.1s plus scale 1.12 → 1 |
| Section entrance | 16–32px rise, stagger 0.06–0.09s, plays once |
| Scrub catch-up | `scrub: 0.6–1`; parallax travel ≤ 8% |
| Springs | Motion `{ type: "spring", visualDuration: 0.4, bounce: 0.15 }`; bounce 0 for UI, 0.1–0.3 only for drag or play; overshoot on transform, never on opacity |

Use ease-out for UI arrivals; an accelerating exit may use ease-in. Keyboard
actions update state and focus immediately; decorative movement must not delay
input or focus. Gate hover
motion behind `@media (hover: hover) and (pointer: fine)`.

```js
// Hero mask reveal (GSAP 3.13+ SplitText) once fonts are ready; reduce = static
document.fonts.ready.then(() => gsap.matchMedia().add("(prefers-reduced-motion: no-preference)", () => {
  const tl = gsap.timeline({ defaults: { ease: "expo.out" } });
  SplitText.create(".hero h1", { type: "lines", mask: "lines", autoSplit: true,
    // Return the line tween itself (not tl) so autoSplit reverts only this tween on re-split
    onSplit: (s) => { const t = gsap.from(s.lines, { yPercent: 105, duration: 1, stagger: 0.09 }); tl.add(t, 0); return t; } });
  tl.from(".hero .lede, .hero .actions", { y: 24, autoAlpha: 0, duration: 0.8, stagger: 0.08 }, 0.35);
}));
```

```js
// Pinned scrub story: the subject changes as you scroll (desktop only)
// Steps are visible in CSS; JS animates FROM hidden, so no-JS and reduced motion still read.
gsap.matchMedia().add("(min-width: 900px) and (prefers-reduced-motion: no-preference)", () => {
  gsap.timeline({ defaults: { ease: "none" }, scrollTrigger: { trigger: ".story", start: "top top", end: "+=250%", pin: true, scrub: 0.8, anticipatePin: 1 } })
    .fromTo(".story .lens", { autoAlpha: 0.4, scale: 1.08 }, { autoAlpha: 1, scale: 1 })
    .fromTo(".story .step", { autoAlpha: 0, y: 24 }, { autoAlpha: 1, y: 0, stagger: 0.5 }, 0);
});
```

## 2c. Control feedback and optional micro-motion

Small feedback confirms actions. A calm or minimal brief can use immediate
colour, underline and focus changes; choose a restrained response instead of a decorative reveal on every section.
For richer motion, choose feedback that fits the concept (baseline CSS:
`layoza animate --recipe interactive-gsap-feedback`). Explicit no-motion requests keep
feedback immediate with colour/underline changes and focus rings.

| Moment | Floor | Value |
| :--- | :--- | :--- |
| Button / link hover (fine pointer only) | colour, underline or fill shift | 150–200ms `ease` |
| Press | `:active` `scale(0.97)` or 1–2px sink | 100–160ms |
| Focus | ring appears, never animated | instant |
| Add / select / toggle | the count or state visibly changes (bump, flip, tick) | 160–240ms |
| Open / close (menu, accordion, basket, dialog) | height via `grid-template-rows: 0fr→1fr` or `interpolate-size`, plus fade; icon rotates | menus/accordions 150–200ms, dialogs/drawers 200–300ms; exit ~70% |
| Form | error shakes ≤ 4px once or slides in under the field; success replaces the form with an arrival | 200–400ms |
| Loading / async | the button shows progress; never a dead click | instant start |
| Live numbers (totals, counters, prices) | roll or count to the new value | 250–400ms |
| Page entrance | hero copy rises 12–16px and fades in, staggered 60–80ms; content is visible if CSS animation never runs | 600–800ms, once |
| Section arrival | content-shaped arrival with visible source content; no shared fade-up imposed on every section | entry 0–40% |
| Card / media hover (fine pointer) | lift 2–4px with shadow, or art scales ≤ 1.03 | 250–500ms |

**Speak the subject.** Where useful, feedback can use the business's own objects or gestures:
- a planetarium basket count turns like a star wheel;
- a bakery's add-to-basket drops a loaf icon into the bag;
- an accountancy match draws a pen tick;
- a booking success tears a ticket stub.

Derive them from the subject inventory, never from a stock list.

**Repeat-proof:** it must still feel right on the 100th click. No celebration on
routine actions, no delay before the result, and nothing blocks the next
input.

**Reduced motion:** keep the colour, opacity and state changes that confirm the
action; drop travel, scale and shake.

`layoza qa` reports `craft-dead-controls` when buttons and links have no
transition and no visible press state.

## 2d. Every still is finished art

From the 2026-10 blind rounds: animated entries lost points on frames caught
mid-scroll (faint, half-revealed, an empty pinned stage), not on the motion itself.

- Resting scroll positions and reduced-motion states are complete, readable
  compositions. Intentional artwork masks may crop during a transition; do not
  leave essential copy half-hidden when scrolling stops.
- Capture each pinned segment at 0, 25, 50, 75 and 100% progress. No frame may
  be empty, faint (content under ~60% opacity covering most of the view) or
  cropped.
- Aim to expose essential text within ~600ms; a coordinated art sequence can
  continue longer while copy/actions stay available. Scrubbed scenes keep every
  resting scroll position readable. Browser QA warns with
  `text-mid-reveal` when a heading is still faded ~0.9s after scrolling stops.
- Stage art fills its stage. In a pinned split (list one side, visual the
  other), the visual spans most of its column and roughly half the viewport
  height. A small card floating in a dark void reads as unfinished, even
  when every other frame is clean (R10b judge, 2026-10-16).
- Decorative lines and ribbons never cross body copy, labels or form fields.
  Only display type may share space with them, and the ribbon sits behind it.
  On mobile, move the ribbon above or below the text block instead of
  thinning it to near-invisible opacity.
- One signature moment carries the page. Cut effects that compete with it;
  choreography (order, timing, easing) is what reviewers score.
- No gate and no long preloader: the first paint already shows the headline
  and the primary action. More rules: `layoza-art-direction/reference/ceiling.md` §4.

## 2e. Deliver the requested motion level

Read execution_foundation.motion from get_design_system and delivery from
recommend_animation. Recipes are ingredients, not choreography. Respect the
explicit dial: 0 static; 1–3 quiet; 4–6 expressive; 7–10 high.

For a highly animated brief, prototype the signature transformation before
finishing the page. Specify its actual subject/object, trigger, starting layout,
transformation, resolved layout and handoff. Coordinate type and art within it.
Connect later sections through the same motion language without imposing a fixed
act count, animated headings everywhere, reactive cursor or particular footer.

Small translate/rotate tweens, floating thumbnails, spinning symbols and a page
progress line alone underdeliver a high-motion brief, however many tweens exist.
Agency example: project identity elements assemble into a readable website, then
expand into the work presentation. This illustrates transformation; it is not a
required agency metaphor.

Stage the phone scene independently for touch and vertical reading. Inspect an
actual recording of opening and scroll transitions on desktop and phone; sample
the signature at start/25/50/75/end. Check legibility, composition, continuity and
resolved actions, plus complete static/reduced-motion versions. If the prototype
looks like minor decoration, change the mechanism before adding more effects.
Stills and QA passes do not prove motion quality. Speed scoring is optional.

## 3. Guardrails (always)

1. **Content visible without JS.** Never hide content in CSS that waits for a
   script; start animations from the visible state (e.g. `gsap.from`, WAAPI
   `fill: "backwards"` only after JS runs). `layoza qa` fails the
   `text-hidden*` rules otherwise.
2. **Reduced motion:** honour `prefers-reduced-motion: reduce` (in GSAP:
   `gsap.matchMedia()`). Keep fades and colour changes; drop movement, pinning
   and scrubbing.
3. **Compositor first:** `transform` and `opacity` are the only properties
   that stay off the main thread. `clip-path`, `filter` (blur above all,
   in Safari) and SVG stroke repaint: keep them to small areas or one-off
   tweens, never a full-viewport scrub by default. Use transforms for scene
   motion; local accordion height/grid transitions are a deliberate exception
   and must not move unrelated content unexpectedly. Never write `transition: all`.
4. **No layout shift.** Reserve space; refresh scroll measurements after fonts
   load.
5. **Frequency budget:** frequent actions answer in ≤ 200ms; only rare moments
   perform. Choreograph rather than stack: at most 2 independent motion systems
   in one viewport.
6. **Teardown:** revert contexts, kill triggers, and cancel animations on
   unmount. Pause idle loops when off-screen.
7. **Design continuity:** content stays visible before and after animation;
   controls remain usable. Speed checks are optional, only on explicit request.
8. **Use the chosen library correctly.** Once you pick GSAP, Motion, Lenis,
   CSS scroll timelines or WAAPI, apply its rules in
   [reference/library-rules.md](reference/library-rules.md). It covers
   scrub vs toggleActions, trigger order and refresh, stacked `from()`,
   SplitText, framework cleanup, Lenis sync, and a `?t=` seek harness for the
   §2d stills.
9. **Interruptible and pausable.** Re-triggerable UI retargets from its
   current value (transitions or springs, not restarting keyframes). Anything
   that moves on its own for more than 5s gets a pause control (WCAG 2.2.2).
   Details: [reference/motion-system.md](reference/motion-system.md) §4–5.

## 4. Lenis

Use it only for scrollytelling showcases. Disable it under reduced motion, keep
native keys working, add `data-lenis-prevent` to modals and drawers, and sync
it with ScrollTrigger if both are used (exact wiring: reference/library-rules.md §ScrollTrigger 8).

## 5. CLI & MCP recipes

```bash
layoza animate --recipe hero-gsap-masked-split --framework react --intensity 6
```
Optional GSAP recipes: `hero-gsap-masked-split`, `section-gsap-authored`, `interactive-gsap-feedback`, `page-gsap-transition`, `scroll-gsap-pinned-narrative`, `threejs-gsap-stage`. Explicit Motion for React: `react-motion-presence`. Legacy alternative-engine recipes remain available by explicit lookup.
