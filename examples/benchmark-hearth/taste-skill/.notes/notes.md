# Hearth & Rye — taste-skill build notes

## Reading of the brief
Fictional 40-cover wood-fired restaurant + morning bakery, Northern Quarter, Manchester. One oven: bread from 6am, dinner from 17:30. Open kitchen. Dinner Wed–Sun, bakery Tue–Sun mornings. Imagery: none of its own, brief forbids external images -> everything drawn in SVG/canvas (overrides skill's open-collection step; note in report).

## Inventory (things only this client has)
1. Name: hearth (the oven floor) + rye (the grain of its darkest loaf)
2. One oven runs the whole day: bread 6am, dinner same oven at night
3. Oven mouth = an arch (the one shape)
4. 40 covers, open kitchen
5. Chef's table for 10 at the pass, facing the oven
6. Rhythm: bakery Tue–Sun mornings, dinner Wed–Sun, Monday cold oven
7. Sold-out-by times on the counter (the bread runs out)
8. Falling heat: bread bakes in the heat after the fire is raked out; dinner on a rebuilt fire
9. Northern Quarter: red-brick warehouses, glazed tile shopfronts, painted signs
10. Printed things: bakery chalkboard of sold-out times, booking slip, bread bag
11. Temperatures across the day (300°C floor at dawn, 380°C for flatbreads at service)

## Argument
1. Hero: one oven, from dawn to last orders -> giant oven arch + live oven status
2. Tonight's menu, all of it, with diet tags -> menu list with dish drawings, dietary filter
3. Morning bakery, and it runs out -> loaf on the arch, six names with prices and sold-out-by
4. The oven runs the day -> inset panel, moving fire picture, day scrubber (live thing)
5. Book (Wed–Sun) -> clip-out coupon form
6. Chef's table for 10 -> specimen tiles of what's included
7. Guests say -> three paper slips (fictional)
8. Questions -> filled-tile accordion (5)
9. Visit -> footer: wordmark built from arch blocks, address, hours, phone, socials

## Direction
Dealt 3: place-carries-it, one-motif, page-is-the-object. Chose `one-motif` (seed 499807271): the oven mouth's arch is the subject's own simplest shape and the one-oven constraint is what no competitor has; place-carries-it needs photographs we cannot have.
Redraws:
- type: Chillax is Fontshare; brief allows Google Fonts only. Redrew -> Albert Sans (Google). (I then re-ran the redraw twice more by mistake while checking output — got Libre Franklin/Source Serif and Switzer; adopting the first legitimate result, Albert Sans.)
- section:testimonial -> testimonial-paper-slips: brief needs 3 reviews and there are no speaker portraits; the dealt blueprint holds one quote with a portrait.
Palette kept (bottle green #133621 / gold #f4d96c): reads as Victorian Manchester shopfront paint and glazed tile, fits.

## Idea
On this page, **the mouth of Hearth & Rye's one oven** becomes **the single arch that shapes the hero, every picture, every button, the bakery stage and the footer wordmark**.
Competitor test: another wood-fired place has an oven, but not one that does the 6am bread and the dinner — the arch carries that day (the live clock), so it is this client's.

## Live thing
The oven clock: on load, reads Manchester time and tells you what is in the oven right now and at what heat (hero line + oven section). A slider scrubs the day 04:30–23:00; the fire picture burns hotter or lower with it. Monday = cold oven; Tuesday = bread only. Fallback without JS: the written timetable.

## Sample content to confirm
All dish names/prices, bakery items/prices/times, oven timetable temperatures, address (fictional unit on Tib Street), phone 0161 496 0418 (Ofcom drama range), chef's table price, reviews (fictional), FAQ answers, social handles.

## Gate + look rounds
Round 1 inspect: 3 FAIL (few-pictures, empty-screens 1–3, overflow 768). Fixed: hero stage no longer pointer-events:none (meter + honest), menu on its own surface field with 104px dish pictures and larger names, coupon notch overflow clipped.
Round 1 look: oven panel picture stranded at bottom -> made left column sticky with the fire beside the scrubber; slips overlapped text -> wider, less overlap; tablet hero arch covered gold-on-gold text -> stacked hero up to 900px; rye drawing read as a crate -> redrawn.
Round 2 look: bakery loaves offset/small (symbol <use> needed width/height) -> fixed; croissant into fact column -> box narrowed; coupon perforation stopped short (svg replaced-element height) -> fixed; mobile header button wrapped -> icon-only toggle under 420px; mobile bakery order -> heading, arch, fact, list; success state not shown (form display:grid beat [hidden]) -> fixed.
Final: lint 0 FAIL (2 WARN: padStart for clock times, defensible). inspect: 1 FAIL few-pictures + WARN no-footage, both forced by the brief ("No external images: use CSS/SVG/canvas"). With --allow few-pictures,no-footage: 0 FAIL 0 WARN.
QA (Playwright, file://): filters V=8 (incl. vegan), VG=3, GF=4; bakery switch; scrubber; empty-submit shows 5 inline errors and focuses name; bad email message; 22 Wed–Sun dates in next 30 days; 9 times 17:30–21:30; success message; FAQ one-open; mobile drawer opens/closes on link; no horizontal overflow at 375; reduced-motion renders full content; no console errors.

## Critique (final)
Design 7 · Usability 8 · Creativity 7 · Content 8 -> 7.4 weighted.
Short: canvas flames still read a little spiky/clip-art; zero real pictures (brief); coupon left half has a soft hole on desktop; menu surface field is only one tone step from the ground; drawings are competent flat illustration, not photography.
