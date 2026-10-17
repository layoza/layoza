# Hearth & Rye — direction (dev-only, never served)

## Inventory
Facts: 40 covers; wood-fired; morning bakery + evening restaurant; Northern Quarter, Manchester; bread from 06:00 in the same oven as dinner; open kitchen; dinner Wed–Sun; bakery counter Tue–Sun mornings; chef's table up to 10; dinner slots 17:30–21:30.
Objects: the oven (brick arch mouth, dome, fire) [CIRCLED], rye loaves on the hearth, the peel, bannetons/proving baskets, the pass with its tickets.
Scene: phone at breakfast or in the evening deciding where to eat; warm fire light against Manchester grey. Light paper ground with soot fields for the fire.
Argument (follows the oven's day): hero=one oven runs the day (proof: scrub the hour) / oven story=how (proof: the curve) / bakery=morning (proof: sold-out-by chart) / menu=tonight (proof: dishes with their oven heat) / book / chef's table=closest seat to the fire / guests / FAQ / close.

## Rut
Reflex: full-bleed sunset food photo, beige + gold, script, centred serif. Opposite: black brutalist with neon. Name pun: a fireplace. None used.

## Deal
Seven candidates: oven temperature log*, NQ glazed faience, cotton shippers' tickets, baker's production sheet, kitchen order tickets, brick/iron mill warehouses, embers & ash. `layoza deal --top 4` -> #1 (key 7db9234d830e17e8): THE OVEN'S DAY AS A TEMPERATURE LOG.
Raise (from faience, declined: weaker product clarity): total palette commitment — colour owns whole fields (paper / firebrick / soot), never sprinkled.
Raise (from production sheet, declined: reads as admin): density courage in the menu and bakery tables.

## THESIS
"On this page, the oven's single day becomes the chart the visitor reads and scrubs: one plotted pen line from the 04:30 fire to the 22:30 bank-down, and every section is a point on it." Refuses: atmospheric photo hero + booking bar + beige.

## OWN-WORLD
Recorder chart paper (cool, grey-green ruled hairlines — justified: the page IS a chart), red recorder pen ink for the plotted line and the action, firebrick and soot fields for the oven itself. Components: ruled rows, axis ticks, pen dots, stamped readouts in mono. Square corners (chart paper), 2px radius on controls only.

## TOKENS
--paper #E9ECE3 (ground) · --grid #B9C7BC (hairline) · --soot #171B19 (ink + dark field) · --pen #C42B17 (accent/action; white text 5.3:1) · --brick #6E2316 (field) · --crust #8A5528 (bread art) · --ember #F08A3C (fire art only, on soot)
Fonts: Archivo (variable wdth 62–125, wght) — display in wdth 112–125 / 800; body wdth 100 / 400. Martian Mono for measurements only (times, °C, prices).
Scale: display clamp(3.4rem, 8.4vw, 8.6rem) / h2 clamp(2.4rem, 5vw, 4.6rem) / h3 1.35rem / body 1.125rem / data 0.875rem.
Alignment: a 12-col grid, 2 anchor lines (page gutter and col 7).

## FIRST VIEWPORT
Desktop:
[H&R wordmark | Menu Bakery Book Private dining Visit | Book a table]
| Bread at six.            |   ______ OVEN (≈50% w)  ___ |
| Supper at nine.          |  /  brick arch, fire, loaves |
| lede (2 lines)           | |   ember canvas in mouth    |
| [Book a table][Tonight's menu] |__ readout 06:00 · 290°C_|
|==== hour axis 05..23 + pen line + range scrubber =========|
Phone: header(wordmark, Book, menu) / H1 2 lines / oven art ~300px / scrubber / CTAs within 812.

## SECTIONS
oven story: claim one oven runs the day; proof annotated 24h curve drawn by scroll; full-bleed diagram on soot.
bakery: claim bread every morning; proof 6 drawn breads + sold-out-by bars on a 07–14 axis; chart on paper.
menu: tonight's 11 dishes with price, tags, oven heat; filter radios; three ruled columns (table) on paper.
book: form as a reservation chit; time + party as segmented radios; split on brick field.
chef's table: plan drawing of 10 seats facing the oven; plan + list on soot.
guests: one quote at scale + two small; statement on paper.
FAQ: ruled accordion list on paper.
footer: name full width, the curve small, address/hours/phone/social on soot.

## MOTION
Hero: line-mask headline reveal (CSS), fire flicker + ember canvas (paused off-screen, static under reduced motion). Scrubber: readout rolls, oven contents swap, fire glow scales with temp (≤200ms). Oven story: pen line draws with scroll, pen dot follows (visible fully drawn without JS / reduced motion). Bakery bars grow once on entry. Micro: filter, accordion, booking success plots your table as a pen dot on the dinner curve.

## FINISH
Unverified is unfinished: art loop done, `layoza qa` 0 errors.
