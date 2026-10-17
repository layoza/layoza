# Volta One — direction (dev only, never served)

## Engine + deal
- `layoza design-system --mode new --json` → experimental-artistic (strong). Directions: A swiss-editorial (7:5 type+object), B kinetic pinned stage, C tactile workbench. Palettes/fonts ignored (generic profile).
- Seven candidates: 1 Bristol painted terraces, 2 OS contour map of Bristol's hills, 3 exploded parts catalogue, 4 Brunel engineering drawings, 5 national cycle-route signage, 6 Floating Harbour cranes, 7 frame-builder's workshop.
- `layoza deal ... --top 4` → **#2 Ordnance-Survey-style contour map of Bristol's hills** (key 7718059ec6d0eeb5).
- Skeleton: Direction B (pinned stage, one canvas carries the story) with Direction A's 7:5 split and tables-not-stat-blocks.
- Challengers: #1 terraces (competitive on audience recognition, declined on product clarity) donates raise: **colour owns whole fields** (Explorer-orange booking field, Avon-blue specs/footer fields). #3 exploded catalogue (declined: becomes a costume) donates raise: **callouts with leader lines are real labels, every one names a true part**.

## Subject
Facts: Volta One; small Bristol company; electric city bike; 16 kg; 80 km range; carbon belt drive; battery hidden in down tube; Graphite / Moss / Clay; £2,190; test rides in Bristol, Bath, Cardiff.
Objects: the frame (circled), the belt + sprockets, the battery inside the down tube, the wheels, the hill.
Scene: commuter in daylight on Bristol's hills → light page.

## THESIS
"On this page, Bristol's hills become a laser-cut contour model the Volta One stands on — rotate the bike, scroll, and the hill flattens as the story reaches its weight."
Refuses: dark void + neon HUD 3D product page; Apple-style white product float.

## OWN-WORLD
Survey map: Pennant-stone ground, contour lines in OS brown-orange, index contours heavier with elevation numerals, map-key tables, grid-reference labels, scale bar + north arrow in the 3D stage, Explorer-cover orange as a field, River Avon blue as a field. Controls: square-cornered map-key chips, 2px ink outlines, focus ring in contour orange.

## TOKENS
- Pennant #D9DED6 (ground) · Ink #18201D (text) · Contour #B4531E (lines) / Contour-text #8E3E12 · Explorer #E8611F (field) · Avon #173F55 (field)
- Paints: Graphite #3A3E42 · Moss #5D6B3B · Clay #B3623E
- Archivo (variable width; display 125 wdth 800, body 100 wdth 400) + IBM Plex Mono (map furniture only: grid refs, scale bar, elevation numerals, sheet ref). Spec values use Archivo tabular figures (revised after round 2: mono read as costume).
- Scale: 14 / 17 / 24 / 44 / clamp(64–150)
- Alignment: everything on the left edge of the chapter column; stage is the right 7 columns.

## FIRST VIEWPORT
Desktop
```
[logo VOLTA]  Ride Specs Colours Test ride      [Order]
| Volta One (huge)          |  3D contour plinth + bike (≈58%) |
| promise line              |    callout labels, N arrow,      |
| £2,190  [Order] [Test]    |    scale bar, rotate controls    |
```
Phone
```
[logo]                 [Order][≡]
[ 3D stage sticky 44svh, bike on plinth ]
Volta One
promise / £2,190
[Order] [Book a test ride]
```

## SECTIONS
1. Hero — claim: a hill-flattening city bike; proof: the bike in 3D on Bristol contours.
2. Battery — the battery is inside the down tube; proof: frame goes x-ray, cells glow. (stage)
3. Belt — no chain, no oil; proof: close-up, belt teeth run, cranks turn. (stage)
4. Weight — 16 kg; proof: bike lifts on a hanging scale, hill layers compress. (stage)
5. Colours — three local colours; proof: paint changes on the bike + order summary. (stage)
6. Specs — map key table on Avon field (13 rows).
7. Riders — elevation profile with three pinned quotes (A/B/C = places).
8. Test ride — form on Explorer field, grid-ref styled.
9. FAQ — details accordion, plain list composition.
10. Footer — Ink field (Avon already owns specs; two Avon fields read as repetition), address, socials.

## MOTION
- Three.js (pinned 0.160.0) procedural bike + contour plinth; drag/keys/buttons rotate; scroll chapters drive camera, x-ray, belt run, lift/flatten, paint.
- Damped camera lerp; under reduced motion: snaps, no auto-rotate, belt static.
- Section entrances: none beyond the stage (stage carries the motion). Micro: 160ms press/colour.
- Fallback: authored SVG side-view (paint via CSS var) when no WebGL / no JS.

## Reflex lines
- Mono: only for measurements, grid refs and spec values (real data).
- Contour-line background: a real (generated) contour map, the dealt world.

## FINISH
Unverified is unfinished: art loop done, `layoza qa` 0 errors.

## Art loop log
- R1 7.1: hero bike small, plinth washed white, eyebrows + mono costume, `frame` name clash broke 3D. Fixed.
- R2 7.8: camera framing, hypsometric plinth tints, profile pins, route labels, perf (deferred init, software-GL light path, endless-loop fix).
- R3 (fresh-eyes 7.9, fix): drawer CTA clip, nav active state, colours summary above fold, chips -> key labels, drag hint only with 3D. Applied.
- Open: plinth elevation numerals; native date field shows OS-locale format.
