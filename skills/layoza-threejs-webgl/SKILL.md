---
name: layoza-threejs-webgl
description: Interactive 3D web scenes, Three.js, React Three Fiber (R3F), Drei, custom GLSL shaders, post-processing budgets, GPU disposal, and mobile/reduced-motion fallbacks.
---

# Layoza Specialist Skill: Three.js & WebGL (`layoza-threejs-webgl`)

Use this skill when building 3D hero stages, interactive product configurators, shader backgrounds, or spatial WebGL storytelling.

## 1. Mandatory WebGL Engineering Rules
1. **Explicit DPR Cap**: Always cap device pixel ratio at `Math.min(window.devicePixelRatio, 2)` (or `[1, 2]` in `<Canvas dpr={[1, 2]}>`) so 3x/4x mobile retina screens do not melt GPU fill-rate.
2. **IntersectionObserver Pause Loop**: Pause `requestAnimationFrame` (or set R3F `frameloop="demand"` / pause render loop) whenever the `<canvas>` scrolls outside the viewport (`!entry.isIntersecting`) or `document.hidden` is true.
3. **Complete GPU Memory Disposal**: On component unmount, explicitly traverse and dispose:
   - Every `BufferGeometry` (`geometry.dispose()`)
   - Every `Material` and array of materials (`material.dispose()`)
   - Every `Texture` / `RenderTarget` (`texture.dispose()`)
   - Call `renderer.dispose()`
4. **Accessible & Low-Power Fallback**:
   - Provide `aria-label` or `role="img"` on the canvas container, or mark `aria-hidden="true"` when decorative alongside semantic HTML text.
   - When `prefers-reduced-motion: reduce` is enabled or WebGL context creation fails, render a crisp static fallback or single still frame rather than a broken blank box.

## 1b. Design: the model is the product

From the 2026-10 blind 3D benchmark: the winning page tied the 3D, the scroll
progress and the place into one idea, but it lost points for a plain model. The
entry with the best-built bike scored highest on desirability.

1. **Fidelity before effects.** Build from real proportions: write down the key
   dimensions first (wheelbase, tube diameters, wheel size), then model them.
   - Vary profiles: the tube that holds a battery is visibly fat, and joints
     have fillets.
   - Name the parts the copy talks about and give each its true form. A belt is
     a flat toothed ribbon, not a chain.
   - Add the details that sell the object: mudguards, cable runs, light lenses,
     tyre tread, a saddle with a seam.
   - Check the model at 2× zoom before writing any story.
2. **Light it like a product shoot:** a key, a fill and a rim light or an
   environment map; a contact shadow under the object; and colours that hold
   on both light and dark grounds. Never put a black object on a near-black
   stage. Physical materials (`MeshPhysicalMaterial` with clearcoat,
   iridescence or transmission) look dead without an environment map: bake a
   small studio env through `PMREMGenerator` and set `scene.environment`.
   Transparent text or label rings: `side: FrontSide`, `alphaTest: 0.1` and an
   explicit `renderOrder`, so they never flicker behind the model.
3. **Every claim is shown in 3D:** each story chapter changes the object (see
   inside, lift, close in). A camera that only moves is decoration.
4. **The stage is never empty mid-story.** Configurators keep the object in view
   while options are chosen (pin the stage beside the picker on desktop; on
   phones, show the object above the options). Frame it with margin at every
   width so wheels and edges are never cropped.
5. **Load order:** headline and CTA paint first (non-blocking fonts), and
   three.js loads after that, with `defer` or after first paint. Show a static
   poster of the object until the scene is ready. Adapt quality on software
   renderers or slow frames (fewer lights, no shadows, 1× DPR) instead of
   dropping the idea.

6. **Stills are finished art.** Capture the scene at 0, 25, 50, 75 and 100%
   of each pinned chapter and as the reduced-motion poster (use the `?t=`
   seek harness in `layoza-animations/reference/library-rules.md` so captures
   are exact, not scroll-timed); every frame must
   be a complete, lit composition. Inspect actual playback for stutter; speed
   measurements are optional and never a design-workflow gate.

7. **Hero art with type:** zone grid, knock-out mask, camera fit for 375 and
   pinned-stage rules live in `layoza/references/kinetic-composition.md`.

8. **Stack, tiers and look:** which technique to pick (three.js, raw shader,
   WebGPU with fallback, CSS, image sequence), the software-renderer lite tier,
   matcap, tone mapping, progress-driven effects and context loss live in
   `references/render-tiers.md`.

## 2. CLI & MCP Integration
Generate a complete Three.js or React Three Fiber scene with built-in disposal and IntersectionObserver pausing:
```bash
layoza animate --recipe threejs-interactive-stage --framework react --intensity 8
```
Or call the MCP tool `recommend_animation` with `category: "three_experience"`, plus `framework` and `intensity`.

For a portfolio or studio brief with real supplied artwork, retrieve
`recipe: "threejs-spatial-work"` with `motion_intensity: 9`. Its executable module is also at
[assets/spatial-work-stage.mjs](assets/spatial-work-stage.mjs). Inject the project's
pinned THREE module, artwork URLs/aspect ratios and selected design-system colours.
Connect `setProgress()` to an authored scroll transition; connect `setPaused()` to
the page's motion control and call `dispose()` on teardown. It includes curved
surface-to-gallery-to-selected-work states, a phone stack and observable lifecycle
diagnostics. This is one viable mechanism, not a mandatory aesthetic.

For a web development agency, show its actual work or a truthful interface
construction rather than treating a rotating primitive as the studio's identity.
Keep complete semantic captions and controls beside the canvas. Review the
handoff between the spatial work and the following portfolio: a disconnected
3D showcase followed by unrelated cards is still a weak idea. During section
motion keep words inside their reserved area; avoid offscreen text becoming an
empty slab on phones. Keep artwork and captions in separate flow boxes so
transforming an image cannot collide with the project name.

Inspect which works remain recognizable on touch screens: a depth stack with
only its front image visible may technically contain three projects while
communicating only one. The spatial-work starter uses a complete feature above
two smaller work surfaces on phones. Adapt that arrangement to the actual art,
and bring it closer to the proposition without squeezing the action labels or
shrinking useful copy. Canvas contrast does not substitute for readable HTML
scene captions and project metadata.

For actual geometry, retrieve `recipe: "threejs-scene-staging"` or use
[assets/scene-staging.mjs](assets/scene-staging.mjs). Its `fitPerspectiveBox`
fits the current pose at the actual aspect/view direction, excluding infinite
floors and scenery. Compare the assembled AND separated bounds: a model can be
uncropped yet much too small for its desktop stage. Use `scenePhase` with the
scene's real transition thresholds and brief-specific explanations. Keep the
caption strip in normal HTML flow outside the canvas; the highlighted phase
must describe what the geometry is actually doing. These helpers supply camera
and narrative primitives, not a universal building or a mandatory drawing style.
