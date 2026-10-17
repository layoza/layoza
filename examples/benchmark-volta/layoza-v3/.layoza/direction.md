# Volta One — direction (dev-only, never served)

## Subject
Facts: Volta One; electric city bike; small Bristol company; 16 kg; 80 km range; belt drive;
battery hidden in the frame; Graphite / Moss / Clay; £2,190; test rides in Bristol, Bath, Cardiff.
Objects: the bike (CIRCLED), the down tube with cells inside, the belt + pulleys, the paint
colour chip, the front door/step you carry it up.
Scene: city commuter, daylight on a hillside street of painted terraces; light page.

## Rut
Reflex: black void stage, neon accent, floating 3D bike in studio glow (engine's own palette!).
Opposite: white Apple-style product page, grey sans, bike on white.
Calibration: avoid cream + serif; avoid near-black + one neon.

## Seven worlds → deal
1 frame-builder geometry drawing · 2 Cliftonwood painted terraces · 3 Brunel bridge elevation ·
4 harbourside cranes/stencil · 5 cycle-route contour map · 6 exploded parts catalogue ·
7 wet dusk commute. `layoza deal --top 4` → **#2 Cliftonwood painted terraces** (key f1022d68bb37107d).
Challengers: #1 geometry drawing (declined, donates raise: dimension lines are TRUE — the
weight chapter uses a real dimension readout, not decoration); #3 Brunel (declined, clothes only).

## THESIS
"On this page, the Volta One is parked outside Bristol's painted terraces: each chapter is the
next house along the street, and the paint you pick for the bike is a house colour."
Refuses: the black-void floating product stage.

## OWN-WORLD
Saturated painted render fields (blue, sherbet pink, mustard, deep teal), flat-ink facade drawing
(sash windows with glazing bars, panelled doors with fanlights, house-number plates), kerb line.
Buttons = painted door panels (square-ish, 4px radius); house-number plates for chapter markers
(sequence = house numbers along the street, carries information); swatches = paint chips.

## TOKENS
--render #C4DCE8 (ground, Cliftonwood blue) · --sherbet #F2AEBB · --mustard #E8B847 ·
--teal #1D4C52 (dark field) · --ink #172126 · --door #B0231B (action accent only) · --bone #F6F2EA (ink on teal)
Bike paints: Graphite #34383D · Moss #56653A · Clay #B5603C.
Type: Archivo variable only (wdth 62–125, wght 400–900). Display = wdth 62 wght 800 (fascia
condensed); body = wdth 100 wght 400. Sizes: body 18 · small 15 · h3 28 · h2 clamp(48–96) · display clamp(96–220).
Alignment: two anchors — page gutter and the stage column edge.

## FIRST VIEWPORT
Desktop:
```
[VOLTA logo] Ride Specs Colours Test ride        [Order]
|VOLTA          |  painted wall ##### window ######|
|ONE            |      ___ 3D bike (55% w) ___      |
|promise        |     O=====/\=====O   door [3]     |
|£2,190         |  ---------- kerb -------------    |
|[Book][Colour] |  drag to turn ◀ ▶                 |
```
Phone: header; stage 46svh (bike whole, wall, kerb); h1 + promise + price + two buttons below
visible without scrolling at 812.

## SECTIONS
- hero: claim "a light e-bike for a hilly city" / proof: price + live bike / split stage.
- ride (3 chapters, pinned stage): battery (no.3 pink: tube goes see-through, cells shown),
  belt (no.5 mustard: camera to drive side, belt runs), weight (no.7 teal: bike lifts, 16 kg readout).
- colours: paint-chip radio group + order summary; stage recolours; configurator.
- specs: teal field, ruled table 12 rows, house-plate numerals.
- quotes: three house fronts stepping down the hill (terrace), pink/mustard/blue.
- test ride: form on ground, ticket-like booking panel.
- FAQ: mustard field, accordion list.
- footer: dark teal, terrace roofline drawn, VOLTA ONE at full width, address, socials.

## Reflex-check lines
- House numbers 3/5/7 as section markers: the world's own notation, the sequence = walking the street.
- Colour fields: the dealt world is literally painted house fronts.

## MOTION
- three.js r160 real-time bike, drag/keyboard rotate, idle slow sway (not spin) when not reduced.
- Scroll chapters drive object state (x-ray tube, drive close-up, lift) with eased tweens; wall colour
  crossfades to the next house. Reduced motion: no sway, instant state changes, no tween.
- Micro: paint chip select → bike repaints with a 300ms colour roll; order count bump; accordion
  icon = wheel turning; booking success rings a bike bell.
- No WebGL: authored SVG side elevation poster, recolourable.

## FINISH
Unverified is unfinished: art loop done, `layoza qa` 0 errors.
