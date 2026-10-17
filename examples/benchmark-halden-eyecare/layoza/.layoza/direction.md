# Halden Eyecare — direction contract (dev-only)

Engine: `layoza design-system --mode new --json` → artisan-craft (strong), alt local-business. Place cues: Leith, Scotland. No material cues; derived from the optician's own instruments.

## Rut
- Mechanism: an independent optician where the test, the frame wall and the repair bench are in one shop on the Shore.
- Scene: Leith locals and walkers on the Shore in daylight, mostly on phones; they want to know "is my NHS test free, when can I come, can you fix my glasses".
- Reflex: clinical white + medical blue + stock smiling-patient photo + Snellen chart; craft variant cream + terracotta + italic serif.
- Obvious opposite: black fashion-eyewear boutique, huge sans, one neon accent.
- Literal reading: an eye / Snellen letters / blur-to-focus headline (refused).

## Seven candidates (by resonance)
1. Trial lens case: bottle-green velvet tray, rows of nickel-rimmed lenses, red (minus) and black (plus) rim tabs, engraved powers. Signature: the row of rimmed lens wells.
2. TABO axis scale on trial frames / Rx slips: 0–180° hairline semicircle, ruled SPH/CYL/AXIS table.
3. Leith Shore warehouses: grey-gold sandstone, cast-iron bollards, dark teal Water of Leith.
4. Edinburgh enamel street signs: black sans on white with rounded border.
5. Frame acetate blocks: layered havana, horn, crystal colours.
6. Focimeter target: concentric rings + crosshair.
7. Snellen chart (literal; declined).
Families: instrument, notation/data, architecture/place, signage, material.

## Fusion
World = 1 (trial lens case) with 2's ruled Rx table as its paperwork (same object family: the trial set and its prescription slip).
Skeleton = Direction C (interactive workbench hero, live controls above the fold) — real action in hero.
- Direction A (swiss grid): competitive on clarity, declined on identification. Raise: exposed hairline rules carry the tables (eye tests, workshop) instead of cards.
- Direction B (cobalt kinetic): declined. Raise: total palette commitment — velvet green owns whole sections, not accents.

## THESIS
The page is the optometrist's trial case: you pick a day like picking a lens. Refuses the clinical-white-plus-stock-photo optician page.

## OWN-WORLD
Velvet green fields (hero, lenses, visit) hold nickel-rimmed round controls with red/black tabs; paper sections are Rx slips with hairline-ruled rows and condensed engraved numerals.

## STORY
Understand: free NHS tests, two named optometrists, frames + workshop on site. Believe: concrete prices, durations, turnaround times. Do: pick a day in the hero, finish the booking form; add a frame to the bag.

## TOKENS
- velvet #12362F (field), velvet-deep #0C2722 (footer)
- rx-paper #EEF2EC (ground), paper-white #F8FAF6 (form surface)
- ink #142420 (text), ink-muted #46564F
- nickel #B7C2BC (rims, rules on velvet)
- tab-red #B3302A (primary action, minus tab), on velvet use #F2B8A8 tint for small text
- Type: Archivo (wdth 62–125; condensed for engraved numerals/labels, semi-expanded for display) + Atkinson Hyperlegible (body; designed for low-vision readers — the reason it is the voice of an optician).
- Alignment: left throughout; numerals tabular.

## FIRST VIEWPORT
Desktop 1440x900:
```
[HALDEN wordmark]  Eye tests Frames Lenses About Visit   [Bag (0)]
|-------------------- velvet field ---------------------------|
| H1 (2 lines, paper)                     |  trial frame SVG   |
| support line                            |  TABO 0-180 scale  |
| [Book an eye test] [See the frame wall] |  axis sweeps once  |
|--- "Pick a day" -- (o)(o)(o)(o)(o)(o) lens chips -----------|
|   [type select]  [Continue booking]                         |
```
Phone 375:
```
[HALDEN]          [Bag 0][Menu]
H1 (4 lines)
support
[Book an eye test]
[See the frame wall]
Pick a day: (o)(o)(o)> scroll row
[type select]
[Continue booking]
```
## SIGNATURE
On load the cylinder-axis marker on the trial frame sweeps from 180° to 90° and the lens tab settles (like refining axis), 1.4s ease-out, once. Reduced motion: drawn at 90°, no sweep.

## FINISH
Unverified is unfinished: this build ends with `layoza qa` at 0 errors and reviewed screenshots.

## Sibling test
A Glasgow optician would also have a trial case → added Leith specificity: copy rooted in the Shore, hours/visit section with the Shore schematic line along the Water of Leith, frame names from the Shore (Kittiwake, Signal Tower, Bollard, Swing Bridge, Tidemark, Granton). Changed: product names and visit section, not the world.

## Assumptions (unattended)
Prices, durations, makers, address number-less, phone in Ofcom drama range (0131 496 0xxx), testimonials and opening hours are authored sample content; footer labels them illustrative.
