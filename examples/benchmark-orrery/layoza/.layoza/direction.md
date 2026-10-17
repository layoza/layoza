# Orrery — direction (dev-only, never served)

## Inventory
Subject facts: independent planetarium + night-sky shop; Bristol harbourside; 60-seat dome; live presenter-led shows;
school sessions weekdays; rooftop telescope evenings on clear Fridays; shop sells star maps, telescopes, kits;
sessions 11:00 / 14:00 / 19:30; membership; small, independent, local.
Subject objects: the dome sky (projected stars) [CIRCLED], the star projector, the telescope on the roof, the paper star map /
planisphere, the ticket. Scene: planning an evening out, often on a phone after dark; inside, house lights down -> dark ground,
but a printed instrument's saturated ultramarine, not near-black.
Argument: Hero: this is the sky over this harbour, live -> working planisphere for Bristol. Shows: four live shows -> programme.
Book: picking a session is picking a sky -> form + wheel set to your session. Telescope nights: real sky when clear, plan B when
cloudy -> steps + dated stubs with computed moon phase. Shop: take the sky home -> wall of drawn objects. Membership: come back
all year -> three columns. Voices -> quotes. FAQ -> accordion. Visit -> footer.

## Rut
Reflex: near-black space + purple nebula gradient + glow + Orbitron/Space-type font + rocket icons; product card grid with pills.
Obvious opposite: cream paper "vintage astronomy" engraving with serif + brass. Name pun: brass orrery gears (kept as candidate 7 only).

## Seven worlds -> deal (`layoza deal --top 4`, key d785c3a04acf470c) => #1 PLANISPHERE
1 Planisphere star wheel 51.45N  2 Dome lights-down ritual  3 Nautical almanac tables  4 Hand-coloured celestial atlas
5 Cliftonwood terraces  6 Brunel red-oxide iron  7 Brass orrery (pun)
Challengers: #2 dome ritual (competitive: strong recognition, weaker product clarity) -> RAISE: "house-lights discipline":
light only where there is information; no decorative glow. #3 almanac (declined) -> RAISE: "tabular honesty": every date,
time and price set in tabular figures in ruled columns, computed not invented (moon phases, dates).
Place (#5) appears only as the harbour skyline the wheel rises over (cranes, terraces, the Orrery roof dome).

## THESIS
On this page, the planisphere becomes the booking dial: the hero wheel is a working star wheel for Bristol, and choosing a
session turns a wheel to the sky over the harbour at that hour. Refuses: space-black + nebula + glow hero, card grid of products.

## OWN-WORLD
Printed instrument card: ultramarine card stock, bone ink printing, sun-yellow date ring, cinnabar ecliptic. Hairline ruled rings,
tick scales, condensed capital ring lettering, italic star names. Buttons = printed tabs; focus ring = yellow ring like the date
index; selection = yellow.

## TOKENS
--ultra #1D2E86 (card ground) · --deep #0F1A52 (sky disc / footer) · --bone #F1E9D5 (print + light fields) ·
--sun #F5B83D (date ring, primary action) · --cinnabar #C8402A (ecliptic, alerts) · --ink #121A45 (text on light)
Families: Archivo (variable wdth/wght; display, ring labels, UI) + Newsreader (body, italic star names).
Scale: 0.8125rem labels / 1.125rem body / 1.5rem / 2.75rem / clamp(3rem,6.6vw,6rem) display. Alignment: two left edges (gutter, 7/12 col).

## FIRST VIEWPORT
Desktop 1440x900:
+--------------------------------------------------------------+
| ORRERY   Shows Telescope nights Shop Schools Visit  [Basket 0]|
| H1 two lines (cols 1-6)          (  PLANISPHERE  ~620px   )   |
| lede                             (  disc+window+hour ring )   |
| [Book a dome show] [Telescope nights]  time slider + visible  |
| ~~~~~~~~ harbour skyline: cranes, terraces, Orrery dome ~~~~~ |
+--------------------------------------------------------------+
Phone 375x812: header / H1 / lede (short) / CTAs / wheel 340px peeking above skyline.
Subject art >= 45% of desktop first screen.

## SECTIONS
Shows: ruled programme rows with drawn emblem (timetable). Book: bone instruction card, form + mini wheel. Telescope: steps
diagram + three ticket stubs. Shop: yellow field wall of drawn objects (no card chrome). Membership: bone ruled three columns.
Voices: ultramarine, three quotes set large/small rhythm. Schools: narrow band. FAQ: bone accordion. Footer deep.

## MOTION
Load: wheel sets itself to "now" (1.4s ease-out rotate) - rare moment. Drag/turn + range input to scrub hours (instant).
Booking: mini wheel turns to chosen session (600ms). Programme rules draw in via scroll-driven CSS (supports-gated).
Basket count bump 160ms. Reduced motion: no rotation tweens, no rule drawing. No library needed (vanilla + CSS).

## FINISH
Unverified is unfinished: art loop done, `layoza qa` 0 errors.

## Reflex-check notes
Tabular mono-ish figures used for data only (tabular-nums on Archivo, no mono costume).

## Art loop log
R1 (self 7.8): skyline was below the fold and the hero overflowed 900px. Fixed hero sizing, lowered the hill and moved the caption into the control. Mini wheel band blended into the yellow field, so it now uses a bone band. Fixed cramped leading.
R2 (self ~8.0): star label collisions (removed the Pleiades/Castor labels); tablet gap between slider and skyline.
R3 (fresh-eyes reviewer 8.1, "fix"): the word spacing had collapsed, so tracking is now -0.022em and word-spacing +0.08em. Pills became printed tabs (3px buttons, 6px fields). Shop moved to the sun field and Schools to deep. Shop and Membership use stacked headers to break the repeated header pattern. The mobile menu now has a scrim and closes on an outside click. Declined: "offset oval window". The window is the computed 51.45N horizon, already offset with the pole near the north edge.
Final: about 8.3. QA: 0 errors, 0 warnings, 0 craft notes.
