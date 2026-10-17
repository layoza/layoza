# Halden Eyecare — design plan (frontend-design skill)

Subject: independent optician on the Shore, Leith. Audience: local adults/families booking a test or buying frames.
Primary job: get people to book an eye test; secondary: browse/buy frames.

## Tokens
- Chart paper  #E8EEEB  (cool grey-green page, like a backlit chart box surround)
- Chart white  #FFFFFF  (surfaces: test chart, form, frame swatches)
- Harbour ink  #12263A  (text, rules)
- Coating violet #5A4BC2 (AR-lens coating sheen; primary actions, focus)
- Lens green   #1D6A57  (secondary accents: NHS "free", success states)
- Slate        #55687A  (muted text, 5.3:1 on paper)

Type: Archivo (variable width; expanded 118–125, 600–800) for display, prices, UI; Source Serif 4 for body (18px / 1.6).
Scale (Bringhurst): 14 / 16 / 18 / 21 / 24 / 36 / 48 / 72.

## Layout
Left-aligned, asymmetric. Hero: headline + CTAs left (7 cols); right a white illuminated test chart (5 cols).
```
[wordmark]  Eye tests Frames Lenses About Visit        [Bag 0]
| Eye tests, frames and      |  +-------------------+ |
| repairs on the Shore.      |  |   H               | |
| supporting line            |  |  A L              | |
| [Book an eye test] [Frames]|  |  D E N  (blurred) | |
|                            |  | Lens 1 | Lens 2   | |
```
Eye tests: a prescription-form style table (it is tabular data), booking form beside it.
Frames: 3x2 grid of white swatches each with a distinct SVG frame drawing. Lenses: 3 columns.
Workshop: a job-ticket list with turnaround. Testimonials: 3 quotes, plain. FAQ: details/summary. Footer.

## Principles
- Signature moment (only one): the refraction chart. "Which is clearer, 1 or 2?" — each click of the trial lens sharpens the chart. It is the most characteristic thing in an optician's world.
- Everything else quiet: hairline harbour-ink rules only where they separate data rows; no shadows; radius 2px on controls, 0 elsewhere except lens circles.
- No caps labels, no eyebrows, no arrows in buttons, no middot meta strings, no numbered markers (no sequences here).

## Review vs generic defaults
- First instinct was cream + high-contrast serif display "boutique" look (default #1). Changed to cool chart-paper grey-green + expanded grotesk, which echoes optotype charts and shop signage.
- First instinct for eye tests was 4 rounded cards; changed to a table since it is price/duration data.
- Accent: considered tortoiseshell amber (too near terracotta tell); chose AR-coating violet.

## Decisions made without client (would confirm)
- Prices, durations, frame names/makers, turnaround times are placeholder content invented for the fictional brand.
- Address "41 The Shore", phone in Ofcom drama range 0131 496 0xxx.
- "Choose" on a lens package selects it (one at a time) and counts in the bag.
- Form has no backend: success message states the request was received and the shop will email to confirm.
