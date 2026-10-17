# Fold Architects — direction (dev only, never served)

Engine: `design-system --mode new --json` -> match_confidence weak (marketing-agency fallback), place cue Glasgow. Engine palettes/fonts discarded (generic); Direction A (swiss editorial grid, 7:5, type + object, content-shaped sections) used as skeleton. Directions B/C motion budgets ignored: client pins "no animation beyond hover/focus".

## Subject inventory
Facts: Fold; 6 people; Glasgow; housing retrofits; small civic buildings; timber extensions; client wants simple, quiet, minimal; no awards/claims; enquiries by project type + budget; work filterable Housing/Civic/Extensions.
Objects: the drawing set (sections, plans, title blocks); the tenement wall build-up (stone + insulation); timber frame/CLT panels; the site/ground line; the model. CIRCLED: the long section drawing through a Glasgow tenement with its new rear timber extension.
Scene: homeowners, housing associations, community groups reading on a laptop at a desk or phone in the evening -> light ground, like a print on a table.

## Rut
Reflex: black/white minimal architecture portfolio, full-bleed photo hero, huge thin sans, grey everything.
Opposite: brutalist concrete-grey heavy grotesk with raw grid lines.
Name pun: origami folded paper (candidate 7, not built).

## Deal
Seven candidates (drawing set, sandstone tenements, timber/CLT, OS street-grid map, retrofit build-up, Subway signage, folded paper). `layoza deal --top 4` -> #1 Architect's working drawing set. Built as dealt.
Challengers: #2 sandstone tenements (audience recognition: high; declined as world — would push to warm red/cream rut) donates RAISE: "place specificity — the drawn building is a Glasgow tenement, not a generic house". #3 timber (product clarity: competitive) donates RAISE: "structural honesty — the extension is drawn as real timber construction (joists, rafters, CLT folded plate), not a silhouette".

## THESIS
On this page, the studio's drawing set becomes the site: the hero is Section A–A through a Glasgow tenement being retrofitted, with a timber extension whose folded roof is the studio's name; every section is a sheet with a title block. Refuses: the photo-hero + grey-grid architecture portfolio.

## OWN-WORLD
A dyeline/whiteprint drawing: cool pale drafting-film ground, violet-blue ink line, one revision red for what Fold adds (insulation, new timber, the fold). Hatching for cut stone, hairline dimension strings, level datums, sheet numbers. Components: buttons are ink-outlined title-block cells; focus ring = revision red; filter = drawing-register tabs.

## TOKENS
--film #F1F2EE (ground) · --ink #262B66 (text/line) · --ink-2 #4C5187 (secondary text) · --tint #DDE0EC (hatch fill, hover rows) · --rev #C2362B (revision red: new work, focus) · --rev-light #FF9C8F (red on ink field)
Fonts: Archivo 400/500/600 (display + body; swapped from Instrument Sans after QA flagged it as overused); IBM Plex Mono 400/500 (drawing numbers, dimensions, register data only — measurement use).
Scale: 14 mono / 17 body / 22 lead / 40 h2 / clamp(44,6vw,84) display. Tracking display -0.03em, leading 1.0.
Alignment: 12-col grid, all text on 2 edges: col 1 and col 6 (7:5 split).

## FIRST VIEWPORT
Desktop 1440x900:
```
[FOLD (fold glyph)]                         Work Studio Approach Contact
------------------------------------------------------------------------
| Statement (5 cols)       |  SECTION A–A DRAWING (7 cols, ~55% mass)  |
| "Careful retrofits,      |  tenement 4 storeys, hatched stone,       |
|  small civic buildings,  |  red insulation lining, rear timber       |
|  timber extensions.      |  extension w/ folded roof, datums,        |
|  Glasgow."               |  dimension string, ground line            |
| [See the work]           |  title block: A-101 Section A–A 1:100     |
```
Phone 375: header + menu button; statement (3 lines) + link; drawing full width below (crop: extension + lower tenement visible in first screen).

## SECTIONS
- Work (A-200 Drawing register): claim = we do three kinds of work, here are six; proof = register rows w/ small authored drawing per project; composition = drawing register table w/ tabs filter.
- Approach (A-300 Wall build-up): claim = how we work in four principles; composition = one large wall build-up detail, 4 layers with leader lines to principles (sequence carries information: outside -> in).
- Studio (A-400 Studio plan): claim = six people, small on purpose; composition = paragraph + floor plan of the studio with six desks keyed to a name schedule.
- Contact (A-500 Enquiry sheet): ink field owns the section; form set as a title-block sheet.
- Footer: title block strip with address/email/phone/social.

## MOTION
None beyond hover/focus (client pin). Hover: 150ms colour/background. Mobile menu opens instantly. Filter instant. prefers-reduced-motion: removes the hover transitions too.

## FINISH
Unverified is unfinished: art loop done, `layoza qa` 0 errors.

## Reflex lines (why allowed)
- Mono: drawing numbers/dimensions/register data — real measurement notation.
- Hairline rules: the drawing set's real line system, not a broadsheet costume; used with hatching and title blocks.
- Layer numbers 1–4: order through a wall carries information.

## Copy deck
H1: "Careful retrofits, small civic buildings and timber extensions in Glasgow."
Lead: "Fold is six architects and designers working on the buildings people already live in and the small public rooms a neighbourhood needs."
CTA: "See the work" -> #work. Form submit: "Send enquiry" -> "Enquiry sent".
All project content illustrative (fictional studio); drawing labelled illustrative.
