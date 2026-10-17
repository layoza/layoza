# Halden Eyecare — direction contract (dev only, not served)

## Subject first
1. Objects: the trial frame (TABO protractor rings, slotted cells, red/black-rimmed trial lenses), the lit test chart, the frame wall, the workshop bench (pliers, screws, hinges), the prescription card. CIRCLED: trial frame over the lit chart.
2. Mechanism: two optometrists refract you lens by lens, then the workshop on the same premises fits and repairs the frame.
3. Scene: Leith Shore, harbour light outside; inside, the test room is dimmed and only the chart glows. Must prove: free NHS test, bookable now, and a real workshop.
4. Reflex: clinical white + teal, split hero with stock illustration, icon-card grids.
5. Obvious opposite: black luxury-fashion eyewear page, giant model photo, thin serif.

## Candidates (by resonance)
1. Refraction ritual: trial frame + lit Snellen chart ("better one, or two?") — subject.
2. The frame wall: frames on rods — subject.
3. Leith Shore painted shopfronts: deep painted fascia with gilt signwriting — place/graphic.
4. Prescription card notation SPH / CYL / AXIS — data tradition.
5. Acetate and tortoiseshell, titanium wire — material.
6. Harbour sandstone warehouses and the Water of Leith — place/architecture.
7. The workshop bench: exploded hinge, screws, pliers — subject/tool.

## THESIS
The page is the test room on the Shore: one lit chart in a painted-green room, a trial frame you look through. Refuses the clinical split hero.

## OWN-WORLD
Fascia green fields (painted shopfront), lit-chart panels, gilt signwriting for the wordmark and primary action, trial-lens red only on lens rims and the bag count. Hairline TABO scale ticks as the structural ornament (a real tool under them).

## STORY
Understand: independent optician, NHS test free. Believe: they refract carefully and fix frames on site. Do: book a slot (prefilled from the hero), add a frame, choose lenses.

## TOKENS
- fascia #12302A (ground of hero, scroll story, booking, footer)
- chart #EEF1EA (lit chart / daylight sections)
- ink #0F1C18 (text on chart)
- gilt #D7B45E (wordmark, primary action on green, scale numerals)
- rim #B8302A (trial-lens rims, bag badge; never body text)
- stone #CFC2A8 (Leith sandstone: workshop field)
- Type: Archivo (wide 112–125 wdth, 700–800) display = optotype letters; Atkinson Hyperlegible body (designed for low-vision readers). Left-aligned throughout; the chart alone is centred.

## FIRST VIEWPORT
Desktop:
```
[Halden]  Eye tests Frames Lenses About Visit        [Bag 0][Book]
 H1 (3 lines, gilt-free chart-white)     |  lit chart 6/60..6/6     |
 sub                                     |==( lens )==( lens )==    |<- trial frame breaks out left over green
 [Book a free NHS test] [See the frame wall]                         |
 [Next openings: 3 slot buttons]          |  chart continues          |
```
Phone: header, art (chart + frame, ~300px), H1, sub, CTAs, slot buttons.

## SIGNATURE + MOTION (revised after owner update: motion chosen per need, not mandated)
- Hero load (GSAP timeline, ~1.4s): chart rows light top-down, frame strokes draw, trial lenses drop into the cells. H1 is NOT split/animated (it is the LCP and needs no theatre). Pointer moves the trial frame and its sharp region (plain JS lerp, hover devices only). RM: static.
- Signature scroll story "One, or two?" (GSAP ScrollTrigger, pin + scrub >= 900px; unpinned scrub below): 3 trial lenses slide into one big cell, blur 10 -> 0, SPH and acuity readout step, step list highlights. RM: sharp, all steps visible.
- Workshop: exploded hinge assembles on scrub (GSAP, reuses the loaded lib). RM: assembled.
- Frame wall: frames swing gently onto the rod when first seen (WAAPI, once). RM: none.
- Everything else static. Micro: CSS press/hover states, frame tilt on hover (CSS), add-to-bag flies a glyph to the bag (WAAPI) + count bump, accordion via CSS grid-rows transition.
- Dropped: SplitText, price tick-ups, ledger reveals, magnetic CTA (none served the brief).

## FINISH
Unverified is unfinished: this build ends with layoza qa at 0 errors and reviewed screenshots.
