---
name: layoza-animations
description: Motion design for the web. Decide what motion a design needs, then pick the right tool for it (CSS, WAAPI, View Transitions, GSAP + ScrollTrigger/SplitText, Motion, Lenis, canvas/WebGL), with reduced-motion support, visible-without-JS content and clean teardown.
---

# Layoza Specialist Skill: Motion Design (`layoza-animations`)

Motion is a design decision, made per brief. Nothing here is mandatory except
the guardrails in section 3. The user's prompt always wins: if they name a
library, a feel, or "no animation", do exactly that. In Mode B/C, extend the
site's existing stack and `MOTION.md`.

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

## 2. Pick the lightest tool that does it well

| Need | Tool |
| :--- | :--- |
| Hover, press, open/close, small state changes | CSS transitions / `@starting-style`, WAAPI |
| Scroll-linked effects that are simple | CSS scroll-driven animations (`animation-timeline: view()`), with a fallback |
| Orchestrated timelines, pinned/scrubbed scroll stories, text splitting, SVG drawing | GSAP + ScrollTrigger / SplitText (all plugins free); pin an exact version |
| React component state, layout, presence, gestures | Motion (Framer Motion) |
| Route morphs | View Transitions API |
| Smooth-scroll showcases | Lenis (section 4) |
| Particles, shaders, 3D | canvas / WebGL / Three.js ([`layoza-threejs-webgl`](../layoza-threejs-webgl/SKILL.md)) |

Load libraries from an allowed CDN with a pinned version, or from npm in a
bundled project. Never add a library the design doesn't use.

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

Never use ease-in on UI. Never animate keyboard-initiated actions. Gate hover
motion behind `@media (hover: hover) and (pointer: fine)`.

```js
// Hero mask reveal (GSAP 3.13+ SplitText), run once fonts are ready
document.fonts.ready.then(() => {
  const tl = gsap.timeline({ defaults: { ease: "expo.out" } });
  SplitText.create(".hero h1", { type: "lines", mask: "lines", autoSplit: true,
    // Return the line tween itself (not tl) so autoSplit reverts only this tween on re-split
    onSplit: (s) => { const t = gsap.from(s.lines, { yPercent: 105, duration: 1, stagger: 0.09 }); tl.add(t, 0); return t; } });
  tl.from(".hero .lede, .hero .actions", { y: 24, autoAlpha: 0, duration: 0.8, stagger: 0.08 }, 0.35);
});
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

## 2c. Micro-motion floor (every page)

Big moments are optional; small feedback is not. Every control answers the
person who touched it. A calm, minimal, content-first or "not heavily
animated" brief still gets this whole floor plus one entrance and gentle
section fades: light motion is what makes a static page feel finished
(baseline CSS: `layoza animate --recipe micro-css-polish`). Only an explicit
"no animation" / "no motion" prompt cuts it back to instant colour/underline
changes and focus rings.

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
| Section arrival | content fades up via CSS `animation-timeline: view()` inside `@supports`; no JS gate, no hidden text without support | entry 0–40% |
| Card / media hover (fine pointer) | lift 2–4px with shadow, or art scales ≤ 1.03 | 250–500ms |

**Speak the subject.** At least two feedback moments use the business's own
objects or gestures, so a neighbouring site couldn't reuse them unchanged:
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

- The rest state, the reduced-motion state and every frame of a pinned or
  scroll-linked story are complete compositions on their own.
- Capture each pinned segment at 0, 25, 50, 75 and 100% progress. No frame may
  be empty, faint (content under ~60% opacity covering most of the view) or
  cropped.
- Entrance reveals finish within ~600ms of entering view, or are scrubbed so
  every resting scroll position shows readable text. Browser QA warns with
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

## 2e. When motion is the brand (animated / kinetic / scroll-driven prompts)

From the 2026-10 animated benchmark: the winner made the **whole page one
scroll story** (a tide that rose from low to high from hero to footer). Entries
whose motion lived only inside one pinned section lost, however polished.

1. **One page-long motion thread.** Pick a quantity from the subject that can
   change: tide height, time of day, temperature, progress of a build, a sky.
   Map scroll progress for the *whole document* onto it, and let every
   section show its current value: a colour, a line height, a clock, a level
   in the art. Section by section, the visitor watches the subject change.
2. **A signature scene per act.** Write the page as 3–5 acts. Each act gets
   one scroll-driven scene drawn from the subject (arrive → set up → peak →
   after), not a generic reveal. Every frame of each scene passes §2d.
3. **Kinetic type is structural.** The hero wordmark and every section
   heading move with intent: masked line reveals, width or weight morphs,
   scroll-scrubbed tracking, or words placed by the thread. The motion must
   come from the subject (letters rising like a tide, glowing like an ember),
   never a stock split-and-fade on everything.
4. **Reactive elements answer the visitor:**
   - the pointer (the art swells or tilts under it);
   - scroll speed (marquees speed up and reverse);
   - live data (countdowns, totals, availability).

   At least two of these, each tied to the subject.
5. **The big numbers are moments.** A countdown, a total or a capacity is a
   display-size event, not inline text.
6. **Pace like film.** Alternate fast and slow acts. Give the slow act room,
   and fill any band left in the scroll budget; an empty band is a failed
   frame, not a pause.
7. **Phones get their own choreography.** No horizontal pin is shrunk into a
   thumbnail. Re-stage each scene vertically; charts and timelines stay
   readable (labels ≥ 12px) or swap to a simpler mobile drawing. The
   wordmark and the primary button sit on the 375px first screen.
8. **Reduced motion keeps the story.** Show each act's key frame as a still
   composition with the thread at that act's value. The narrative stays
   readable without the travel.
9. **Budget:** 60fps on mid hardware; LCP ≤ 2.5s with the headline painted
   before any motion script; only transform and opacity in scroll loops (filter and clip-path repaint; use them on small elements or one-off tweens, never a full-screen scrub).

10. **Detail parity across acts.** Builders spend their best work on the hero
    and run out of budget later, and reviewers mark it every time. Before
    building, list each act's drawn object and give it the hero's level of
    detail: parts named, light, texture, people or props. Build the weakest
    act's art *before* polishing the hero.
11. **The last act is the ending.** The final scene (sunrise, last service,
    launch) *is* the footer: the wordmark at full width, plus one line of
    contact and links inside that scene. Never end with link columns under
    the story.

12. **Art reacts, text holds.** For WebGL/canvas hero art, composition and
    pinned-stage rules: `layoza/references/kinetic-composition.md`.

**Self-check before finishing.** Capture frames every ~760px (desktop) and
~700px (mobile) with the `?t=` harness or real scrolling. Read them as a
storyboard:
- Can you name the thread from the frames alone?
- Does every frame show it?
- Does any frame look like a template section?

Any "no" is the next fix.

## 3. Guardrails (always)

1. **Content visible without JS.** Never hide content in CSS that waits for a
   script; start animations from the visible state (e.g. `gsap.from`, WAAPI
   `fill: "backwards"` only after JS runs). `layoza qa` fails
   `text-hidden-after-scroll` otherwise.
2. **Reduced motion:** honour `prefers-reduced-motion: reduce` (in GSAP:
   `gsap.matchMedia()`). Keep fades and colour changes; drop movement, pinning
   and scrubbing.
3. **Compositor first:** `transform` and `opacity` are the only properties
   that stay off the main thread. `clip-path`, `filter` (blur above all,
   in Safari) and SVG stroke repaint: keep them to small areas or one-off
   tweens, never a full-viewport scrub. Never animate layout properties, and
   never write `transition: all`.
4. **No layout shift.** Reserve space; refresh scroll measurements after fonts
   load.
5. **Frequency budget:** frequent actions answer in ≤ 200ms; only rare moments
   perform. Choreograph rather than stack: at most 2 independent motion systems
   in one viewport.
6. **Teardown:** revert contexts, kill triggers, and cancel animations on
   unmount. Pause idle loops when off-screen.
7. **Performance:** INP ≤ 200ms and CLS ≤ 0.1 hold. Defer scripts, and let the
   LCP element paint before animating it.
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
Recipes: `staggered-hero`, `scroll-pinned-story`, `waapi-spring-reveal`,
`magnetic-cta`, `view-transition-route`, `lenis-gsap-sync`.
