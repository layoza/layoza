<!-- Layoza-original synthesis. Numeric starting points adapted (paraphrased, not copied) from:
     lottiefiles/motion-design-skill (MIT), emilkowalski/skills (MIT), vercel-labs/web-interface-guidelines (MIT),
     heygen-com/hyperframes motion docs (Apache-2.0). See THIRD_PARTY_NOTICES.md. -->

# Motion system: personality, physics, choreography

Use this when you set `MOTION.md` tokens or plan a page's motion. The numbers
are starting points; the brief and the world decide the final values.

## 1. Pick one motion personality and derive every token from it

The personality comes from the brand register in `direction.md`, not from
taste. One signature ease carries about 80% of tweens; one or two others
cover the exceptions.

| Personality | Quick / standard / slow | Overshoot | Signature ease | Fits |
| :--- | :--- | :--- | :--- | :--- |
| Calm / premium | 300 / 500 / 800ms | 0% | long ease-out (quint/expo) | luxury, galleries, hospitality, health |
| Assured / corporate | 200 / 300 / 450ms | 0–3% | cubic/quart out | finance, B2B, legal, public services |
| Lively / playful | 150 / 250 / 400ms | 10–20% on play moments | spring, bounce 0.2–0.3 | kids, games, consumer apps, food |
| Charged / kinetic | 100 / 180 / 300ms, plus long scrubbed acts | 0–5% | expo out, hard cuts | agencies, music, sport, launches |
| Tool / product UI | 120 / 200 / 280ms | 0% | cubic out; no decorative motion | dashboards, settings, editors |

Write the result as tokens (`--dur-quick`, `--dur-std`, `--dur-slow`,
`--ease-sig`, `--ease-move`) and use only those.

## 2. Timing physics

- **Distance scales duration.** Base the duration on about 100px of travel.
  Then multiply: ~50px ×0.8, ~200px ×1.3, ~400px ×1.6, full screen ×1.8–2.
- **Size scales too.** Small chips and icons are quick; panels and sheets are
  standard; full-screen scenes are slow.
- **Exits are faster.** An exit takes about 70% of the entrance time and
  accelerates out (ease-in is fine *only* for exits).
- **Overshoot budget by meaning:** error 0%, routine feedback 2–5%, success
  5–10%, celebration 15–25% (and rare). Premium registers stay at 0%.
- **Springs:** describe them by perceived length (Motion `visualDuration`)
  plus bounce. Response about 0.25–0.35s for chips, 0.35–0.5s for entrances,
  0.5–0.7s for hero landings. Overshoot moves transforms, never opacity.
  For JS-free springs, sample the curve into CSS `linear()` (about one stop
  per 20ms).
- **Seekable springs.** For the `?t=` harness and scroll scrubs, a spring
  must be a pure function of progress (a closed-form ease), never a
  simulation that depends on frame history.

## 3. Choreography

- **Order by importance, not by DOM order.** The headline lands first, then
  the proof, then the action. Supporting parts follow the eye path.
- **Crowd limit.** With three or more animated items, at most about a third
  move at once. Keep a whole stagger under ~500ms, and under ~20 animated
  items in one viewport.
- **Coherent grouping.** Parts of one group can share an ease and direction;
  coordinate their stagger. Avoid copying that same entrance across unrelated
  sections. Vary pacing where the reading order calls for it, not to meet a
  quota of different effects.
- **Long travel gets a waypoint.** Movement across more than a third of the
  screen gets an intermediate keyframe or an arc, never a straight slide.
- **Depth by speed.** Parallax layers move at about 1.0×, 0.5× and 0.2×.
  Background counter-motion runs at 20–30% of the hero's speed, in the
  opposite direction.
- **Hold the resolve.** Leave 100–200ms of stillness after a sequence lands
  before the next one starts. Stillness is what makes the motion read.
- **Hand-offs match velocity.** The outgoing element accelerates out
  (about 0.3s, ease-in), the incoming one decelerates in (ease-out), and
  their fastest moments meet at the cut. Energy follows mood: calm
  transitions 0.5–0.8s (sine), medium 0.3–0.5s (cubic/quart), high
  0.15–0.3s (quint/expo).

## 4. Interaction physics

- **Interruptible by default.** Anything the user can re-trigger (toggles,
  toasts, menus, tabs) uses transitions or springs that retarget from the
  current value. Keyframe animations restart from zero, so use them only for
  one-shot moments. Any new input cancels or retargets the running motion.
- **Origin follows the trigger.** Popovers and menus scale from their
  trigger (`transform-origin` at the anchor; Radix and Floating UI expose
  `--radix-*-transform-origin` / `--transform-origin`). Modals scale from the
  centre. SVG parts animate on a `<g>` with `transform-box: fill-box`.
- **Tooltips.** Delay the first one (~300–500ms). Once one is open, its
  neighbours open instantly, with no animation, until the pointer leaves the
  group.
- **Loaders don't flicker.** Wait ~150–300ms before showing a spinner or
  skeleton; once shown, keep it for at least ~300–500ms.
- **Drag and swipe.** Capture the pointer while dragging and ignore extra
  touches. A flick dismisses when its velocity is above about 0.1px/ms,
  whatever the distance. Past an edge, add rubber-band resistance (the
  element moves a shrinking fraction of the finger's travel). Every gesture
  has a tap or keyboard alternative.
- **Theme switches.** Turn transitions off while swapping light/dark tokens,
  so the whole page doesn't fade colour by colour.

## 5. Accessibility of motion (beyond reduced motion)

- Anything that moves, blinks or scrolls automatically for more than 5s gets
  a visible pause/stop control (WCAG 2.2.2): tickers, marquees, autoplay
  video, looping canvases. Off-screen loops pause.
- Flashing stays below 3 times a second (WCAG 2.3.1).
- Parallax and large-scale zooms are vestibular triggers: under reduced motion
  they become still frames, not slower versions.

## 6. Text effects: a shared vocabulary

Name the effect in the storyboard so the build is unambiguous. Layoza's own
spec for each:

| Name | What happens | Starting values |
| :--- | :--- | :--- |
| `line-mask-rise` | lines rise from behind a mask | yPercent 105→0, 0.9–1.1s, expo out, stagger 0.08s; pad the mask ~0.2em so descenders aren't cut |
| `word-cascade` | words rise and fade in reading order | y 0.4em→0, 0.5s, stagger 0.03–0.05s |
| `char-scatter-settle` | letters arrive from small random offsets | ±0.3em, rotate ±8°, 0.6s, stagger from centre; only for ≤ 2 words |
| `focus-resolve` | the heading sharpens from blur | blur 8→0px plus opacity, 0.6s; one element, never a full-screen scrub |
| `weight-breathe` | a variable font's axis morphs | e.g. `wght` 300→700 or `opsz` 144→72 through a CSS variable, 0.4–0.6s |
| `scramble-to-word` | random glyphs settle into the word | 0.6–0.9s, monospace or tabular figures only; aria-label holds the real text |
| `counter-roll` | digits roll to the value | 0.8–1.4s, tabular-nums, ease-out; the final value is in the HTML |
| `marquee-track` | a line runs edge to edge | ~60–90px/s, pause on hover and focus, pause control if longer than 5s |
| `scroll-highlight` | words light up as the reader passes them | scrubbed opacity 0.25→1 per word; the rest state at the end is 1 |

Every effect finishes within ~600ms of entering view (or is scrubbed), keeps
the real text in the DOM, and has a still state for reduced motion
(`layoza qa` warns `text-mid-reveal` otherwise).

## 7. Review the motion like an editor

- Watch each sequence at 0.25× speed (`gsap.globalTimeline.timeScale(0.25)`
  or DevTools' animation panel): find overlaps, dead gaps and moments where
  two things fight for attention.
- Write down the timeline as a beat list (time, element, property, ease). If
  you can't, the choreography isn't designed yet.
