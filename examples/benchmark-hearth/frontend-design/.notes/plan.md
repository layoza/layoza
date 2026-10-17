# Hearth & Rye plan (frontend-design skill)

Subject: 40-cover wood-fired restaurant + morning bakery, Northern Quarter, Manchester. One oven, bread at 6, dinner at night.
Audience: locals booking dinner, morning bakery regulars, people planning a group meal.
Primary job: get a table booked; secondary: get people to the bakery before things sell out.
Decision (no client to confirm with): "best restaurant website in the world" is aspiration, not a style. Interpreted as: the most specific page possible about this one oven.

## Tokens (v1)
- cream #F4F1EA + terracotta + big serif  -> REJECTED in review (skill's trait #1, and the obvious "bakery" default)
## Tokens (revised)
- Tile green #1E3B34 (Victorian glazed faience on Northern Quarter facades) - primary dark
- Glaze #2D5348 - secondary dark
- Flour #EEF0EA - cool, flour-on-tile pale (not warm cream)
- Brick #A8432F - actions on light
- Ember #F08A3C - heat, only on dark green
- Ink #1A1F1C
Type: Young Serif (display: soft, doughy, heavy serif) + Schibsted Grotesk (body/UI). Sentence case labels, no caps eyebrows, no mono.

## Layout
```
[wordmark]   Menu Bakery Book Private dining Visit   [Book a table]
| headline (left, big)          |  brick oven arch SVG, live temp |
| line + 2 CTAs + live status   |  in the mouth                   |
-- Tonight: filter chips / small | large | desserts (columns) --
-- Bakery: 6 items as bars on a 08:00-14:00 morning line (sold-out-by) --
== Oven's day (green): 24h temperature curve, scrub slider, phases ==  <- signature
-- Book (form) beside visit facts --
-- Private dining arch panel --
-- Reviews, FAQ, footer --
```
Left aligned throughout. Arch motif (oven mouth) used for hero art + private dining panel only.

## Review against generic default
- v1 hero was "big headline + photo-ish illustration + 3 stat numbers" -> cut stats; hero art is the oven itself with a live reading derived from Manchester time.
- v1 bakery = card grid -> changed to a time-bar list (sold-out-by is temporal data, so a timeline is information, not decoration).
- Oven story as prose paragraphs -> made it the one bold element: the temperature curve of a day; the phases are a real sequence so a timeline is justified.

## Signature
The oven's day: one curve, 24h, scrubbable; hero reads the same model live.

## Motion
Only: hero ember flicker (off under reduced motion). User-triggered: accordion, menu filter, nav, form states.

## Log
- see critique.md after screenshots
