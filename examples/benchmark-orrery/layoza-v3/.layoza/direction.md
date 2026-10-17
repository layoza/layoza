# Orrery — direction (dev only, never served)

## Inventory
Subject facts: independent planetarium + night-sky shop; Bristol harbourside; 60-seat dome; live presenter-led shows;
weekday school sessions; rooftop telescope evenings on clear Fridays; shop (star maps, telescopes, kits); membership;
fictional business (no awards, no real claims).
Subject objects: the dome with its projected sky (CIRCLED), the star projector, reclined seats and upturned heads,
the presenter's pointer, planisphere / star chart, telescope on the roof.
Scene: someone at home in the evening planning a family outing or a Friday night; also teachers by day.

## Rut
Reflex: navy-black starfield, purple nebula gradient, glow, a space-tech sans (Orbitron), icon cards.
Obvious opposite: sepia/cream antique celestial atlas with engraved Latin serif.
Name pun: brass clockwork orrery gears (kept only as the tiny wordmark device).

## Seven candidates (deal --top 4 -> #4 dealt, key be523d4bd6c6d827)
1 Planisphere star wheel  2 Dome at showtime  3 Cyanotype  4 GLASS-PLATE ASTROPHOTOGRAPHY NEGATIVES (dealt)
5 Bristol harbourside at night  6 Celestial star atlas  7 Red-light observing

## Fusion
World (far): glass-plate negatives — pale silver-green glass ground, the sky inverted (black emulsion stars),
red and blue hand-ink annotations (circled objects, plate numbers), reseau measuring grid, black edge tape.
Signature (near): the dome seen from the audience — hemisphere filled with tonight's real sky over Bristol
(computed from a bright-star catalogue), reclined seats with upturned heads, the star ball, the presenter's red pointer.
Idea sentence: "On this page the Orrery dome becomes an annotated glass plate of tonight's Bristol sky, and the
visitor turns the hour to watch the sky wheel over the seats."
Challengers: 2 (dome at showtime) merged into the signature. 1 (planisphere) donates a raise: the sky is DIALLED by the
visitor (time slider), not just shown. 5 (harbourside) donates the place: dockside cranes/terrace roofline on the roof
section.

## Tokens
glass #D6DDD9 (ground) · glass-deep #C3CCC7 (alt field, plate edges) · emulsion #14171C (ink + dark fields)
red ink #A61E17 (action + annotation only) · blue ink #1C3478 (booking field, plate numbers)
Type: Archivo variable (wdth 62-125) — condensed 72% bold for display, 100% for body; Cedarville Cursive for
ink annotations only (aria-hidden or duplicated in HTML).
Scale: display XL clamp(56px,7vw,104px) · H2 clamp(40px,4.4vw,64px) · H3 26px · body 18px · small 14px.
Alignment: 12-col, 64px side margin desktop / 16px phone; text anchors at col 1 and col 7.

## First viewport
Desktop:  [wordmark  nav ........ Book  Basket(0)]
          [H1 2 lines        |  GLASS PLATE: dome hemisphere, ]
          [lede              |  stars, seats, projector, red   ]
          [Book / Fridays    |  pointer  (~50% of the screen)   ]
          [                  |  slider: turn the sky 18:00-02:00]
Phone:    [wordmark  Book  Basket  Menu] / H1 2 lines / dome plate full width / lede / CTA pair

## Sections (layout words)
hero=split statement+plate / shows=mosaic of 4 large finder plates / book=blue field form + live ticket stub /
telescope=dark positive field, evening timeline + roof skyline + 3 dates list / shop=specimen tray (ruled 3x2 drawings) /
membership=ruled table rows by magnitude / schools=plan (60-seat plan drawing) / voices=statement quote + 2 slips /
faq=accordion list / footer=dark, name at full width + visit facts.

## Motion
Load: the sky "develops" (stars fade in brightest-first) while the dome sky turns 18:00 -> 19:30 (2.4s, once).
Signature: visitor turns the hour slider; real alt/az recomputed, pointer follows Vega.
Sections: show plates' red ink circles draw when entering. Micro: basket count turns like a star wheel; add-to-basket
drops a star into the basket; booking success tears the ticket stub and stamps it. Reduced motion: no turning, no
travel; states change instantly. Tools: vanilla SVG + WAAPI/CSS only (no library needed).

## Reflex checks allowed
Reseau grid lines: real notation of astrographic plates. Hand-script face: plate annotations were handwritten.

FINISH: Unverified is unfinished: art loop done, layoza qa 0 errors.
