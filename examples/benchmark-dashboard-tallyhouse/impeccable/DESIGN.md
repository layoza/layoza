# Tallyhouse design (impeccable arm)

World: Victorian glazed-tile bakery shopfront. Operate mode: the world lends type, palette, density and one signature move. Everything else is standard controls.

## Tokens
- Fascia #163A2C with gilt wordmark #D8B65E (decorative only). Tile band under the fascia.
- Ground #EEF2F1 (cool white glaze), panels #FFFFFF, grout rules #C8D1CE / #DDE4E1.
- Ink #16201C / #44514B / #5C6963.
- State glazes: green #1E6B4E (primary, up, shop lead), oxblood #8C2B2E (critical, down, bakehouse), cobalt #25488A (series, focus, counter), amber #8A5A0B (low).
- Type: Figtree 400–800, tabular figures everywhere. Radius 3px.

## Signature move
Rota as a tile wall: each shift is a glazed tile whose position and width are its real hours on a 05:00–19:00 lane. Early bake cover row flags days with no baker before 06:00. On phones, one day is shown at a time and the times are written out under the tile.

## Rules
- State by shape as well as hue: pending orders have dashed borders, sent orders solid.
- Destructive action (Cancel order) is a quiet underlined text button, kept apart from primaries.
- The decision strip shows only the next decision (items below threshold, cut-off time).
- Motion is 150–200ms and state-only (tooltip, drawer, toast). It is removed entirely under prefers-reduced-motion.
- Settings drawer is a native dialog (Esc, close button, backdrop, focus returns to opener).

## Provenance
No raster images. All art is inline SVG/CSS. Sample data is fictional and labelled.

## Finish review (in-thread, no subagent available)
Checked against craft-floor: no eyebrows, no coloured side stripes, no gradient text or glass, no unicode icons, and no monospace. The KPI row is required by the brief.

Verdict: ship. Remaining nits:
- the pending panel is tall and empty on first load (desktop);
- 30-day chart labels are thinned to about 10.
