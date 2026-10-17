# Render tiers, stack choice and look (three.js r186 baseline)

Written against three.js 0.186.1 (Oct 2026). three.js renames APIs between
releases, so pin the version in the importmap and check the release notes
before you copy an API name from older code.

## 1. Choose the technique before the library

| Need | Pick |
|---|---|
| A product or object model, ribbons, scenes | three.js `WebGLRenderer` (default) |
| Abstract hero art with no model (fields, blobs, gradients) | a raw WebGL2 fragment shader (SDF or noise, about 2 KB), no library |
| Compute or particle counts in the 100k range | three.js `WebGPURenderer` + TSL, **with** a WebGL2 fallback and `await renderer.init()` |
| A scanned real place or object | a Gaussian-splat renderer on three.js (low splat count for captures) |
| Logo or UI vector motion | Rive, Lottie or SVG |
| Card stacks, type, parallax, reveals | CSS (`view-timeline`, transforms), no GPU budget |
| A cinematic product spin that must be frame-exact | a baked image sequence or video, scrubbed by scroll progress |
| An existing project on Babylon, PlayCanvas, R3F or Spline | stay on that engine |

Never ship WebGPU as the only path. Headless capture browsers and many devices
have no `navigator.gpu`.

## 2. Two tiers, one composition

Detect a software renderer once and pick a tier. The composition, camera
framing and colours stay identical between tiers; only the cost changes, so
screenshots taken on the lite tier show the same design visitors see.

```js
const gl = renderer.getContext();
const info = gl.getExtension('WEBGL_debug_renderer_info');
const name = info ? gl.getParameter(info.UNMASKED_RENDERER_WEBGL) : '';
const lite = /SwiftShader|llvmpipe|Basic Render|software/i.test(name)
  || navigator.hardwareConcurrency <= 4;
```

| Feature | Full tier | Lite tier |
|---|---|---|
| DPR | `min(devicePixelRatio, 2)` | 1 |
| Lighting | PBR with env map (`RoomEnvironment` through `PMREMGenerator`) | matcap material (rich look at almost no cost) or the same env map with no extra lights |
| Shadows | soft shadow map, or a baked contact shadow | baked contact-shadow texture only |
| Transmission, iridescence, clearcoat | allowed (transmission adds a render pass) | faked with a fresnel rim plus env reflection |
| Bloom, chromatic aberration, grain | at most one full-screen pass | none: bake glow into textures or sprites, grain as a CSS overlay |
| Geometry | full segments | segments roughly halved |

## 3. Look rules that read in a still

- Tone mapping: `AgXToneMapping` or `ACESFilmicToneMapping`, output
  `SRGBColorSpace`, set the exposure deliberately.
- Give every silhouette a fresnel rim, so it separates from the ground on
  both light and dark sections.
- Use a matcap when there is no time for a light rig. It beats flat PBR with
  no env map.
- Damp every animated value (lerp factor about 0.06–0.1 per frame) so stills
  never catch a jerk.
- Drive deformation, twist, dissolve and camera by **section progress**, not
  scroll velocity. A screenshot has zero velocity, so velocity effects never
  show; progress effects make the 25/50/75% stills differ.
- Mid-change shader states (a dissolve with a glowing edge, a twist halfway
  through) show both states in one still, which reads as motion.
- Kinetic lines: `TubeGeometry` along a `CatmullRomCurve3`, or a flat ribbon
  strip. Many repeated pieces: `InstancedMesh`.
- Type in 3D space: SDF text (e.g. troika-three-text) or a canvas texture,
  always with the real text in the DOM for screen readers and search.

## 4. Robustness

- Paint a poster image of the first frame before three.js loads. It is the
  LCP element and the reduced-motion end state.
- Handle `webglcontextlost` (prevent default, stop the loop, show the poster)
  and `webglcontextrestored` (rebuild). Chromium no longer falls back to
  SwiftShader automatically, so context creation can fail: catch it and keep
  the poster.
- Capture harnesses that need WebGL launch Chromium with
  `--use-angle=swiftshader --enable-unsafe-swiftshader`.
- Load from a pinned importmap, e.g.
  `"three": "https://cdn.jsdelivr.net/npm/three@0.186.1/build/three.module.js"`
  and `"three/addons/": "https://cdn.jsdelivr.net/npm/three@0.186.1/examples/jsm/"`.
- All the rules in SKILL.md §1 still apply: DPR cap, pause off-screen,
  dispose everything, reduced-motion fallback.
