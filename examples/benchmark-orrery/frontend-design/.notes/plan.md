# Orrery: design plan (frontend-design skill, two passes)

## Subject (decided, not confirmed with client: unattended run)
Subject: a 60-seat presenter-led planetarium + small night-sky shop, Bristol harbourside.
Audience: Bristol families, teachers booking school sessions, adult hobbyists.
Primary job: get people to book a dome show (secondary: telescope Friday, shop, membership).

## Pass 1 (first instinct)
- Near-black space background, glowing starfield, one bright accent, big centered headline. Card grid for shows/shop.
- Review: this is generic default #2 (near-black + single bright accent) and the "space site" cliche. Rejected.

## Pass 2 (revised)
Source of identity: the object the venue is named after (a brass orrery) and historical star atlases
(engraved plates, Prussian-blue printed charts, brass instruments). Not "outer space", but "instrument + chart".

Color (5):
- Prussian ink  #13294B  night sections, primary text on light
- Chart paper   #E6EAF0  cool blue-grey paper (deliberately NOT warm cream)
- Brass         #C79A3E  orrery rings, primary buttons on ink (dark text on brass)
- Dome blue     #2F5597  links / focus on paper, secondary surfaces
- Lamp white    #F7F8FB  text on ink
(Error: #A3271F only for form errors.)

Type:
- IM Fell English (+ italic): engraved-atlas display face, headings and prices only.
- Atkinson Hyperlegible: body + UI. Made for legibility; fits schools + low-light reading. Clearly distinct from Fell.
- Scale (classic 6-8-9-10-11-12-14-16-18-21-24-36-48-60-72): body 18px, small 15px, h3 24, h2 48, h1 72 (clamped).

Layout:
```
[wordmark ◯ Orrery]   Shows  Telescope nights  Shop  Schools  Visit   [Basket 0]
---------------------------------------------------------------- (ink)
 Headline (left, 6 cols)              |   LIVE ORRERY (SVG, 6 cols)
 support line                          |   planets at today's approx. positions
 [Book a dome show] [Telescope Fridays]|   date dial readout under it
---------------------------------------------------------------- (paper)
 Shows: programme listing (rows, like a printed programme, not cards)
   title (Fell) | about | duration · age · price columns
 Booking form sits in a panel to the right on desktop (sticky), below on mobile
---------------------------------------------------------------- (ink)
 Telescope nights: 3 numbered steps (a real sequence) + cloudy policy + next 3 Fridays (computed)
---------------------------------------------------------------- (paper)
 Shop: 6 items, 3x2, each with a small engraved SVG glyph, left-aligned text
 Membership: 3 tiers in a row, middle visually marked as "most chosen"? -> no, avoid invented claim; mark by price only.
 Testimonials: 3 quotes, large Fell italic
 FAQ: details/summary accordion
 Footer (ink): address, hours, phone, socials
```
Left-aligned throughout; only hero orrery is centered in its column.

Principles:
- Spend boldness on ONE thing: the hero orrery showing where the planets are today. Everything else quiet.
- Structure = information: numbering only for telescope-night steps (real sequence). Hairlines only where tabular (show rows).
- No all-caps labels, no eyebrows, no middle-dot meta strings, no arrow glyphs on buttons, no mono labels.
- Motion: only the orrery turning slowly (stops on reduced motion) and responses to actions (basket bump, accordion, form success).

## Decisions made without client confirmation
- Fictional address "7 Ropewalk Quay, Bristol BS1" and phone in Ofcom drama range 0117 496 0xxx.
- Prices invented (GBP), show titles invented. No awards or third-party claims.
- Planet positions are approximate mean longitudes (labelled "approximate").

## Critique log
- Pass 1 screenshots: primary hero CTA was brass-on-brass (`.on-ink a` beat `.btn`), toast edge peeked at viewport bottom, anchors hid under sticky header. Fixed.
- Pass 2: nav links turned brass from the fix; tier perk lists stretched by grid 1fr. Fixed.
- Kept: Atkinson Hyperlegible's slashed zero (a legibility feature of the face); noted as a taste call.
- Verified in Playwright at 1440/768/375: no horizontal scroll, no console errors, basket count, form errors + success, accordion, mobile menu.
