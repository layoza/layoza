<!-- Layoza-original. Replaces the earlier blanket ban on drawn SVG pictures with a fidelity bar. -->

# Art production: drawing the subject at studio fidelity

The signature art is what people remember. Make it at the level of a working
illustrator or information designer, never as clip art.

## 1. Pick the medium the world implies

| World / need | Medium |
| :--- | :--- |
| Notation, maps, charts, diagrams, signage, tickets | Inline SVG drawn as the real notation (correct conventions, scale, labels) |
| Objects (frames, tools, products) | Layered SVG with construction lines and light, or a real photograph / generated image when available |
| Atmosphere, depth, particles, optics | Canvas or WebGL with a static SVG/PNG fallback |
| Typographic worlds (posters, eye charts, timetables) | Type *as* the image: real faces at display scale, set with the world's rules |
| Photography-led brands | Real photos (or generated images when a tool is available), consistent crop and grade, at least 4 pictures |

If the user or project supplies images, use them first. Never fake a photo with
CSS.

## 2. The fidelity bar (check at 2x zoom)

1. **Correct construction.** Draw from the real object's geometry: proportions,
   joins, hinges, thicknesses. A spectacle frame has a bridge, pads, hinges and
   temples at the right angles. Look up how the thing is built before drawing it.
2. **Layered depth.** Use at least three planes (back, mid, front) and overlap
   instead of floating parts. Light comes from one consistent direction:
   highlights, a soft shadow or core shade, and edge accents.
3. **A line system.** Pick one stroke family and 2–3 weights (outline,
   detail, hairline) and use them everywhere. Make sure cap and join styles
   match the world (a technical drawing uses round joins and hairlines; a
   linocut uses heavy shapes).
4. **The world's material, rendered honestly.** Use flat ink for print worlds,
   transparent overlaps for glass and lenses, and real tile and pattern logic
   for textiles. Never use CSS bevels, embossing, chalk or metal effects as a
   stand-in for a material the page doesn't render.
5. **Specific, not generic.** Include details only this subject has: the shop's
   street and its real landmarks, the trade's own notation (a prescription
   slip, a lens-power scale), real labels in the world's typography.
6. **Purposeful scale.** The signature art is the dominant mass of the first
   screen (about 40% or more on desktop). Supporting art in sections stays
   smaller and quieter.
7. **No primitive piles.** A scene built from loose circles and rectangles reads
   as placeholder (`CRAFT-SHAPE-ASSEMBLED-ILLUSTRATION`). Draw paths.

## 3. Make it alive (when the design calls for it)

The signature art should be something the visitor can operate or watch change:
a loupe that magnifies, a lens that sharpens, a part that snaps into place, a
map that pans. Build the technique itself (SVG masks and filters, canvas,
scroll-driven transforms), with a still fallback for reduced motion and no JS.

## 4. Type as image

When type is the art, set it with a face whose character fits the world, at a
scale where it becomes architecture: tight leading and optical tracking, and
cropped or masked only where the world crops it. Keep real text in the DOM
(selectable, readable by screen readers). Use SVG only for lettering that isn't
text.

## 5. Detail all the way down (the 2026-10 juries' most repeated note)

In every blind round, the drawings got thinner after the hero, and every
judge marked it. Plan the art for the whole page before building any of it.

**Art plan.** Write one row per section into `direction.md` before coding:

| Section | Object drawn | Named parts (≥ 5) | Light / material | People or props for scale | What moves or answers |
| :--- | :--- | :--- | :--- | :--- | :--- |
| e.g. Camping | night map of the site | station, road, dune paths, tent rows, beach | moonlight from top left, sodium lamps | tents, a van, a figure with a torch | route draws with scroll |

**Build order:**
1. Draw the weakest row's art first and the hero last.
2. Spend at least 40% of the art time after the hero.
3. A section with no row is either type-led on purpose (say so in the plan)
   or a template that needs art.

**Detail density test.** Crop every section drawing at 2× zoom next to the
hero crop. Each one must show:
- the same line system;
- at least five named parts;
- one light direction;
- one texture.

A drawing that reads as "a shape with a label" fails the test. Redraw it; never
shrink it.

**Techniques that add depth cheaply (SVG and canvas):**
- **Texture:** `feTurbulence` plus `feColorMatrix` grain (one shared filter);
  hatching or stipple `<pattern>`s for shadow; paper or ink noise at 3–6%
  opacity.
- **Form:** two or three stacked gradients (base, core shade, rim light)
  instead of one flat fill. Add a soft contact shadow under every object
  that stands on something.
- **Atmosphere:** depth haze (far planes lighter and lower in contrast),
  light shafts as clipped gradients, and one bright accent in each scene.
- **Life:** small figures, props, signage and lit windows. A world without
  people or traces of use reads as a diagram.
- **Reuse without sameness:** `<symbol>` parts (a tent, a tree, a figure)
  placed with varied scale, rotation and colour, never identical rows.

**Budget.** Share filters and symbols. Keep each section drawing under ~40KB
inline, or draw it in canvas from code, so detail never costs LCP.

## 6. Asset checklist before QA

- [ ] The signature art passes all seven fidelity checks at 2x zoom.
- [ ] Every row of the art plan is built, and every section drawing passes the
      detail density test (§5) next to the hero.
- [ ] Every section has the asset its content needs (an object wall, a diagram,
      a map), not just an empty card.
- [ ] Icons come from one family or are authored SVG with one stroke and one
      weight. No emoji or Unicode glyphs stand in for icons.
- [ ] Decorative SVG has `aria-hidden="true"`; meaningful art has a short
      accessible name.
- [ ] Inline SVG stays lean: reuse `<symbol>`/`<use>`, no editor cruft, and
      under ~60KB per illustration unless it is the hero.
