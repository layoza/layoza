# Kinetic composition: bold 3D art + readable type

For motion-led studio, agency and showcase sites with WebGL or canvas hero art
(a ribbon, a crystal, a fluid mass). Two blind-review failures to avoid:
**safe and small** (a plain torus, thumbnail-sized on a phone: "generic") and
**bold but on top of the words** (art through the headline, CTA or body;
vanished header; floating buttons: usability penalty). Every rule here can be
checked in a 1440 or 375 still. Pick any tool the brief allows (CSS
scroll-driven animation, WAAPI, GSAP, Motion, plain rAF); none is required.

## 1. Zones first, then art

Before modelling, draw the hero as two zones on a 12-column (1440) and
4-column (375) grid and write them down:

| Viewport | Text zone (headline, line, CTA) | Art zone (silhouette bounding box) |
|---|---|---|
| 1440x900 | cols 1–6, lower-left or centre-left, inside 64px gutters | cols 5–12, 70–95% of viewport height; may bleed off the right and top edges |
| 375x812 | full width, bottom 45% of the first screen (CTA fully visible above the fold) | top 55–60% of the first screen, silhouette 90–120% of the viewport width, cropping off one or two edges |

Tests (run on the stills):
- **K1 Overlap budget.** Art may overlap only display type (≥ 48px), only
  behind it, and only at a contrast the type still passes. Body copy, labels,
  nav and every button sit on a clean ground or a plate (§2). No exceptions
  mid-scroll.
- **K2 Silhouette size.** The art's projected bounding box covers ≥ 35% of the
  1440 hero area and ≥ 40% of the 375 first screen. Under that it reads as
  a thumbnail; make it bigger or crop it, never shrink it to fit.
- **K3 Bold shape.** The silhouette is the subject's own form or a
  non-primitive (a ribbon with twist and varying width, a folded sheet, a
  swept extrusion along a curve). A bare torus, sphere, cube or torus-knot
  fails, however good the shader.
- **K4 Crop with intent.** On phones the art bleeds off an edge rather than
  floating whole and small in the middle. A cropped big shape reads as
  confident; a centred small one reads as a default.

## 2. Three legal layerings

1. **Type on the art's negative space** (best). Shape the art around the text
   zone: the ribbon's loop opens where the headline sits, the camera sits so
   the text zone is empty sky. Verify by rendering the art alone and checking
   the text zone's bounding box is ≥ 90% background.
2. **Art behind display type, with a plate.** The headline sits on a solid or
   near-solid plate (`background: var(--ground)` or a 92%+ opaque panel), or
   the art is masked out under the text box. Never rely on a text-shadow.
3. **Art frames type.** The art wraps around or passes behind the type block
   (a ribbon looping behind a headline): the art is z-index below the text,
   and a knock-out mask clears it inside the text box.

Knock-out mask recipe (the canvas stays one layer; the text box punches a soft
hole through it):

```css
.hero { position: relative; isolation: isolate; }
.hero canvas { position: absolute; inset: 0; z-index: -1;
  /* --tx/--ty/--tw/--th: text box in px, set from getBoundingClientRect */
  mask: radial-gradient(calc(var(--tw) * .65) calc(var(--th) * .75)
        at var(--tx) var(--ty), #0000 60%, #000 100%); }
```

In a shader, do the same with a uniform rectangle (`uTextRect`) and fade
the art's alpha or saturation inside it with `smoothstep`. Update the
rectangle on resize and after fonts load (`document.fonts.ready`).

**The HTML is the type.** Headlines, CTAs and body stay real DOM text above
the canvas (selectable, indexable, readable before WebGL loads). If a WebGL
copy of the headline is used for an effect, the DOM text is still there,
sized the same, and the effect only adds to it.

## 3. Camera and scale so the silhouette stays big on 375

The default perspective camera keeps a fixed *vertical* field of view, so a
portrait screen shows far less width and wide art shrinks or clips. Fit the
camera to the art's bounding box per viewport instead of using one distance:

```js
function fitDistance(box, camera, fill /* 0.9 desktop, 1.1 phone */) {
  const size = box.getSize(new THREE.Vector3());
  const vFov = THREE.MathUtils.degToRad(camera.fov);
  const hFov = 2 * Math.atan(Math.tan(vFov / 2) * camera.aspect);
  const dH = (size.y / fill) / 2 / Math.tan(vFov / 2);
  const dW = (size.x / fill) / 2 / Math.tan(hFov / 2);
  return Math.max(dH, dW) + size.z / 2;
}
```

- Re-run on every resize (`ResizeObserver` on the canvas parent), update
  `camera.aspect` and call `updateProjectionMatrix()`.
- On phones, also **re-pose** the art, not only re-scale: rotate a horizontal
  ribbon to run diagonally or vertically so its long axis matches the screen,
  and offset the art upward (`art.position.y`) into the top art zone.
- Use `fill > 1` on phones so the art bleeds (K4). Cap DPR at 2.
- Size the canvas in CSS to the art zone, not always to the full viewport: on
  375 the canvas can be the top 60% of the hero and the text block a normal
  DOM block below, so no overlap is possible.

## 4. Content first, art reacts

- **K5 Text is never the animated thing in a pinned stage.** Headline, body
  and CTA are at full opacity at progress 0, 0.25, 0.5, 0.75 and 1. Scroll
  drives the art (rotation, twist, camera, colour), a counter, or a mask on
  an image; text may slide a short distance, never fade below 1.
- **Pinned stage at progress 0 is finished.** Author the stage's static CSS
  as its *first frame*, then animate away from it (`to`, not `from`). If a
  library tween sets initial states from JS, the page without JS, before JS
  and at progress 0 must still look identical to the authored first frame.
- **Pin only what has content.** The pinned element contains the heading and
  the art together; pin length ≤ 2–3 viewport heights per chapter. Each
  chapter shows a different key frame, so no two stills look alike.
- CSS-only scroll reveals: use `animation-timeline: view()` with
  `animation-range: entry 0% entry 60%` (ends before the element is fully in),
  wrapped in `@supports (animation-timeline: view())` and
  `@media (prefers-reduced-motion: no-preference)`; outside both, the element
  is simply visible.

```css
@supports (animation-timeline: view()) {
  @media (prefers-reduced-motion: no-preference) {
    .reveal { animation: rise linear both;
      animation-timeline: view(); animation-range: entry 0% entry 60%; }
  }
}
@keyframes rise { from { translate: 0 24px; } to { translate: 0 0; } }
```
(Animate position, not opacity from 0: a still caught mid-range is still
readable.)

## 5. Header, controls, showreel, finale

- **Header:** a compact, solid (or ≥ 92% opaque) bar that stays visible on
  desktop; on phones solid or hide-on-scroll-down/show-on-up. Never a
  transparent header over moving art past the hero.
- **No floating buttons over the text column.** Sound, "scroll", cursor
  badges and "play reel" chips sit in the gutter or inside the art zone.
- **Showreel:** the reel section is the reel at full bleed (16:9 at 1440,
  4:5 or 9:16 crop on 375) with a poster frame set (`poster=`), title and
  play control on a plate in a corner. The section after it starts within
  one viewport; no empty band while a scaled-up video shrinks back.
- **Finale = footer.** The art returns in its final state (the ribbon ties
  off, the shape resolves) behind a full-width wordmark, one contact line and
  links on a plate. Footer height ≤ 100vh with no empty top half.

## 6. Reduced motion, fallback, cleanup

- `prefers-reduced-motion: reduce`: render one hand-picked frame of the art per
  chapter (same composition, same zones), no scrub, no pointer tilt. Story
  text unchanged.
- No WebGL / context lost: a poster image of that same frame at the art zone
  size; the hero layout never collapses.
- Pause the render loop off-screen (`IntersectionObserver`) and when
  `document.hidden`. On teardown: kill scroll triggers/timelines, disconnect
  observers, remove listeners, dispose geometry, materials, textures and
  render targets, then `renderer.dispose()`. Library helpers that revert a
  context (e.g. GSAP `matchMedia`/`context`) count, but must be called.

## 7. Still check (do it before calling it done)

Capture 1440 and 375 at hero, at 0/25/50/75/100% of each pinned chapter,
after the showreel and at the footer. Fail any still that shows: art crossing
body copy or a button; text below full opacity; a pinned slab with no
heading; an art silhouette under K2; a missing header; a band ≥ 40% of the
viewport with no content.
