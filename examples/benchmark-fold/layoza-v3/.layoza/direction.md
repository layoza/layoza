# Fold Architects: direction (dev only, never served)

## Inventory
Engine: matched "marketing-agency" at weak confidence; directions used for structure only (A: swiss editorial grid, asymmetric 7:5, type as image). Palettes/fonts from the engine discarded.

Subject facts: Fold; 6 people; Glasgow; housing retrofits; small civic buildings; timber extensions; client wants simple, quiet, minimal; no motion beyond hover/focus; enquiries by form; work filtered by type; 4 principles.
Subject objects: the card massing model on its base; drawings (plan, section); the tenement; timber framing; the model-maker's cutting mat and scalpel. Circled: **the white card model on a greyboard base**.
Scene: a client (homeowner, community trust) at a desk on a grey Glasgow afternoon, reading calmly. Light page.

## Rut
Reflex: white page, black Helvetica, full-bleed photo grid, project list with tiny thumbnails.
Obvious opposite: black brutalist architecture site with huge mono type.
Calibration: avoid cream + serif; avoid broadsheet hairlines + tiny mono labels.

## Seven worlds -> deal
1 drawing sheet + title block; 2 Glasgow red sandstone tenements; 3 wally close tiles; 4 folded greyboard card model (name pun, also a true studio object); 5 larch/CLT timber; 6 OS street plan; 7 planning site notice.
`layoza deal --top 4` -> **#4 folded card model** (key a43343ce9e4387a3). Kept: architects really make card models; it passes truth and clarity.
Challengers: #1 drawing sheet donates its discipline -> **raise: UK alteration-drawing convention, existing in white/black, new work in red, used as the only colour logic on the page.** #2 sandstone donates the accent hue (red sandstone red), not a motif.

## THESIS
On this page, every project is shown as the studio's own card model on a greyboard base, existing fabric in white card and Fold's new work in red, so the colour itself says what the studio did.
Refuses: photo-grid portfolio with thumbnails.

## OWN-WORLD
Greyboard ground (the model base board) covering the page; white card for models and the enquiry sheet; ink for line; one red for new work and for actions. Square corners, 1px ink hairlines, no resting UI shadows (cast shadows only inside the drawn models).

## TOKENS
--board #D4D0C7 (ground), --board-deep #BDB8AD (model bases, rules), --card #F7F6F2 (form sheet, fields), --ink #1C1B19, --ink-2 #4A4741 (secondary), --red #9B3A24 (new work + actions).
Type: Schibsted Grotesk only (400/500/700). Sizes: display clamp(48px, 7.4vw, 112px) [hero + one-word section heads], title 24px, body 18px, meta 15px. Left edges: page margin and 7-col start.

## FIRST VIEWPORT
Desktop:
```
Fold                     Work Studio Approach Contact  [Start an enquiry]
Old buildings,                      ┌──────────────────────────────┐
new rooms.                          │  card model: tenement (white) │
                                    │  + red folded timber extension│
Fold is a six-person studio ...     │  on greyboard base   (~48%)   │
See the work                        └──────────────────────────────┘
                                     legend: white existing / red new
```
Phone:
```
Fold                 [Enquire] [Menu]
Old buildings,
new rooms.
[ model, full width, ~300px ]
lede / See the work
```

## SECTIONS
- Hero = split: claim (what we do) / proof (the model of a retrofit + extension).
- Work = mosaic of 6 large models (>=25vw desktop), filter radios-as-pressed-buttons; red marks what Fold did on each.
- Approach = list: 4 principles as large titled rows on hairlines.
- Studio = table: paragraph + 6-cell register (role / name), like a model's label plate.
- Contact = split: big word + direct lines / white card enquiry sheet.
- Footer = close: full-width wordmark, small hero model, address/email/phone/social.

## MOTION
Client forbids animation beyond hover/focus. Hover: colour/underline only, 150ms colour transition on fine pointers. Focus: 2px red ring offset 3px. Filter and menu are instant (no transitions). Reduced motion: removes the colour transition too.

## FINISH
Unverified is unfinished: art loop done, `layoza qa` 0 errors.

## Art loop log
ROUND 1: Seen in one second: white card tenement with red timber sheds. Subject named from picture: yes. Layout per section: hero=split / work=mosaic / studio=table / approach=list / contact=split / footer=close.
Scores: design 8 · usability 8.5 · creativity 7.5 · content 8.5 -> 8.1. Fixes: section order matched to nav; studio/approach recomposed on the col-6 anchor; mobile footer model collision; comma spacing from tabular-nums; menu close icon; form blur shifted the submit button mid-click (bug fixed).
Reviewer R1: fix, 8.3 (red used as decoration on 6+ element types; link-column footer; in-between type sizes).
ROUND 2: red restricted to new work + logo fold + the one enquiry button; footer = wordmark + model + one line; hero model 88% so the base reads whole; card-block legend swatches; visible filter count; mobile register one column; sizes merged to 15/18/24/display/wordmark; filter pressed-state no longer fades.
CRAFT note left on purpose: craft-shape-assembled-illustration counts polygons in the generated isometric models; they are projected 3D solids with one light and one line system (models.py), not primitive piles.
QA warning left: render-blocking-fonts flags the recommended media=print/onload pattern after the swap has run (checker reads media after load).
