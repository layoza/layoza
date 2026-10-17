<!-- Layoza-original wording. Correctness rules adapted (ideas only, rewritten) from greensock/gsap-skills (MIT, (c) 2026 GreenSock, commit aed9cfd: gsap-core, gsap-timeline, gsap-scrolltrigger, gsap-plugins, gsap-performance, gsap-react, gsap-frameworks), Ga14ctic/awwwards-skill (MIT, (c) 2026 Sam Luker, commit 5a15177: reference/gotchas.md), aditraanggra/awwwards-skills (MIT, commit 99efa62: animation-orchestration hero cascade) and C-Jeril/framer-motion-skills (MIT, (c) 2024 kuakua-app, commit 837d726). Layoza stays library-neutral: these rules apply only once the design has chosen that tool. -->

# Library rules: once a tool is chosen, use it correctly

Pick the tool in SKILL.md §2 first. The rules below cover the mistakes agents
make most often with each tool. Each one costs blind-jury points as a frozen,
blank or cropped frame.

## GSAP: core and timelines

- **Stacked `from()`.** `from()` and `fromTo()` apply their start values at
  once. If two of them animate the same property on the same element, give
  the later one `immediateRender: false`, or that element is never seen.
  This is the most common cause of text that stays hidden after scroll.
- **Prefer `autoAlpha` to `opacity`.** At 0 it also sets
  `visibility: hidden`, so invisible layers stop catching clicks. CSS must
  never pre-hide the element; animate *from* the hidden state.
- **Choreograph in one timeline.**
  - Give the timeline a `defaults` object (ease and duration) and name the
    beats with labels.
  - Overlap beats with the position parameter (`"<"`, `"<0.15"`, `"-=0.3"`),
    never with `delay` on each tween.
  - A hero cascade: kicker → words (stagger 0.06–0.08) → lede at `-=0.4` →
    actions at `-=0.4`.
- **Staggers carry the meaning.** Use `{ each: 0.08, from: "center" | "edges" |
  "random" | index }` and choose the origin from the content (a dial spreads
  from the centre, a list from the top).
- **Layout changes** (filters, reorders, expanding cards): record
  `Flip.getState(items)`, change the DOM, then
  `Flip.from(state, { duration: 0.5, ease: "power2.inOut", absolute: true })`.
  In React, use layout animations instead.
- **Pointer-driven motion** (cursor followers, magnetic buttons, tilt): build
  one `gsap.quickTo(el, "x", { duration: 0.4, ease: "power3" })` per axis and
  feed it the pointer position. Never start a new tween on each `mousemove`.
  Gate it behind `(hover: hover) and (pointer: fine)`.
- **`will-change`** goes only on elements that are animating, added just
  before and removed just after. Never apply it to everything.

## GSAP: ScrollTrigger

1. **One driver per trigger.** Choose `scrub` (tied to the scrollbar) *or*
   `toggleActions` (plays once on enter). If both are set, scrub wins and the
   toggle actions are silently ignored.
2. **Attach `scrollTrigger` only to a timeline or a top-level tween.** Never
   put it on a tween inside a timeline, and never nest scroll-triggered
   timelines inside a parent timeline.
3. **Creation order is refresh order.** Create triggers in page order, top
   to bottom. If they are created out of order (async data, lazy sections),
   give each a `refreshPriority` so the first section on the page refreshes
   first. Otherwise one pin's spacer shifts every later start position.
4. **Refresh after content settles.** Call `ScrollTrigger.refresh()` after
   `document.fonts.ready`, after images load and after injected content.
   Resizes already refresh, debounced at about 200ms.
5. **Grids and lists:** use
   `ScrollTrigger.batch(".card", { interval: 0.1, batchMax: 4, onEnter: (b) => gsap.to(b, { autoAlpha: 1, y: 0, stagger: 0.08, overwrite: true }) })`.
   Cards that enter together cascade as one group instead of firing one by
   one.
6. **Horizontal scroll sections:**
   - Pin the wrapper and move the inner track with `ease: "none"`; any other
     ease desyncs the track from the scroll.
   - Triggers inside the track use `containerAnimation: thatTween` with
     starts like `"left center"`.
   - Inner triggers cannot pin or snap.
7. **`matchMedia` owns breakpoints and reduced motion.** Use one
   `gsap.matchMedia()`, with `conditions` such as
   `{ isDesktop: "(min-width: 900px)", reduce: "(prefers-reduced-motion: reduce)" }`,
   and branch on them inside one handler.
   - Never nest `gsap.context()` inside it.
   - Revert everything with `mm.revert()`.
   - After an in-page "reduce motion" toggle, call `gsap.matchMediaRefresh()`.
8. **Smooth scroll (Lenis) sync:**
   - Drive Lenis from the GSAP ticker:
     `gsap.ticker.add((t) => lenis.raf(t * 1000))` and
     `gsap.ticker.lagSmoothing(0)`.
   - Also feed ScrollTrigger: `lenis.on("scroll", ScrollTrigger.update)`.
     Missing this last line is the usual cause of pins drifting.
9. **Route changes in an SPA or Barba:** kill or revert the old page's
   triggers *before* initialising the next page, or pins and spacers pile up.

## GSAP: SplitText (3.13+, free)

- Leave `aria: "auto"` on (the default), so screen readers read the heading,
  not the letters.
- Build the animation inside `onSplit` and return it, so `autoSplit: true`
  re-splits and resyncs after a font swap or resize.
- When splitting into characters:
  - Set `font-kerning: none` on the text, or the letter spacing jumps after
    the split.
  - Never apply `text-wrap: balance` to split text.
  - Gradient text (`background-clip: text`) disappears when split into
    characters. Animate the word as one unit, or put the gradient on each
    line.
- Split only the units you animate: lines for reveals, words for emphasis,
  and characters only for short display words.

## GSAP: React, Next, Vue, Svelte

- **React:**
  - Call `gsap.registerPlugin(useGSAP)` once, then
    `useGSAP(() => { … }, { scope: ref, dependencies: [...], revertOnUpdate: true })`.
  - Wrap animations started from event handlers in `contextSafe(...)`, or
    they leak past unmount.
  - Without the hook, use `const ctx = gsap.context(fn, ref)` and return
    `() => ctx.revert()`.
- **Next and other SSR frameworks:** never run GSAP during server render;
  initialise in client components or effects only. StrictMode runs effects
  twice, and only a revert-based cleanup survives that.
- **Vue / Nuxt:** create animations in `onMounted` and revert them in
  `onUnmounted`.
- **Svelte:** create them in `onMount` and revert them in its cleanup.

## Motion (formerly Framer Motion) for React

- Import from `motion/react`, not the old `framer-motion` package.
- Wrap the app in `<MotionConfig reducedMotion="user">`, and read
  `useReducedMotion()` before any travel, parallax or loop.
- Keep bundles small with `LazyMotion` plus the `m` component.
- For lists, use `AnimatePresence mode="popLayout"` with stable `key`s, so
  exits don't shove their neighbours.
- Use `layout` / `layoutId` for shared-element moves instead of measuring by
  hand.

## CSS and WAAPI (lightest tools)

- Scroll-driven CSS (`animation-timeline: view()`) only inside
  `@supports (animation-timeline: view())`. Without support the content
  must already be visible.
- WAAPI: call `el.animate(keyframes, { duration, easing, fill: "both" })`, and
  `cancel()` on teardown. For reduced motion, check
  `matchMedia("(prefers-reduced-motion: reduce)")` first.

## A deterministic still harness (any library)

Scroll-timed screenshots are flaky. Give the page a dev-only seek hook
behind a query flag: with `?t=0.5`, seek each signature timeline to that
progress (`tl.progress(0.5)` in GSAP, `currentTime` in WAAPI, the scene clock
in three.js) and pause. Capture 0, 0.25, 0.5, 0.75 and 1 for SKILL.md §2d. In
production the hook does nothing unless the flag is set.
