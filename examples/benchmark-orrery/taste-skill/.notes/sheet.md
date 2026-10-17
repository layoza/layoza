# DIRECTION SHEET — The split stage
seed 199277873 · drawn 2026-10-15 · redraws: 4 · sector culture
Brief: Orrery, a 60-seat independent planetarium dome and night-sky shop on Bristol harbourside; live presenter-led shows, Friday rooftop telescope nights

| | |
| --- | --- |
| Direction | The split stage |
| Hero | Menu column, sticky photograph |
| Type | Rokkitt / Literata |
| Colour | Stone and blueprint |
| Sections | list (The printed menu sheet) → contact (Clip-out coupon form) → how-it-works (Itinerary on a slip) → showcase (Object on a disc) → pricing (Prices like a menu) → testimonial (Quotes on paper slips) → faq (Questions beside a tall photograph) → footer (A single quiet line) |
| Motion | Slow crossfade |
| Photographs | the "Sky and space" set, as atmosphere |

This is the direction for this build. Adopt it. Do not blend it with the layout you would have made anyway,
and do not redraw because it is unfamiliar: unfamiliar is the point. Redraw a slot only when it conflicts
with a stated brand constraint or needs an asset that does not exist, and say why in your report.

The parts of this sheet were written separately and will disagree in small ways. Settle every disagreement in this order:
**house rules** (including type no smaller than 13px and body text of 16px or more) → **the direction** (the idea, the composition habits, where the
one accent goes) → **the type system and the palette** (which face does which job, the five size steps, which colour is the ground and where the
accent goes) → **the hero blueprint** → **the section blueprints**. Under each blueprint below, "Adjusted for this page" lists where it disagrees.
In practice: take geometry from blueprints (columns, proportions, what sits where) and take everything else from above them. Map every size a
blueprint names onto one of your five type steps instead of adding a sixth. Use one button shape, one corner radius and one pair of page margins on
the whole page, whatever individual blueprints say. If a blueprint names a kind of typeface (a light serif, a heavy grotesk), use the dealt face.

## 1. The idea
Half the screen never moves. One side is a fixed photograph (or object, or product view) that stays in place for the whole visit; the other side is a calm scrolling column with everything you need to read. The fixed half keeps the feeling present while the reader gets on with the facts, like sitting in the room while reading the menu. As sections pass, the fixed image may quietly change to the one that matches.

Before anything else, write one sentence in your notes: "On this page, <the subject's own material> becomes <the device>."
Fill the first blank from your inventory of what only this client has (its name and what it means, its street, its numbers, its
constraint), then put the nearest competitor in the sentence: if it still works, the idea is the category's, not the client's.
Then name the one live thing the visitor can do on the page (SKILL.md step 4).

## 2. Composition
Desktop: a two-column page, 50/50 or 45/55. One column is `position: sticky; top: 0; height: 100svh` and holds a single photograph filling it edge to edge, or the object centred on a flat field. The other column scrolls and holds all content in a single narrow measure (max 46ch) with comfortable padding (48–72px): name and one-line description first, then sections separated by generous space, then contact and footer. Headings in the scrolling column are modest (32–44px). The column grounds differ: photograph on one side, flat paper colour on the other, meeting at a clean vertical edge. On a phone the fixed half becomes a 60svh image at the top that stays sticky behind the first section, then releases.

## 3. Type — Rokkitt / Literata
Narrow geometric slab capitals as a tall title block, then a bookish serif for a long essay beneath. Suits manufacturing, trades, manifestos and industrial history.
```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Rokkitt:wght@800&family=Literata:ital,wght@0,400;0,600;1,400;1,600&display=swap" rel="stylesheet">
```
- Display: **Rokkitt** — weight 800 · tracking -0.01em · leading 0.85 · uppercase, sized by cap height (about 14vw)
- Text: **Literata** — 400 for body, 600 for emphasis · leading 1.6 · tracking 0
```css
:root {
  --step-small: clamp(0.8125rem, 0.79rem + 0.1vw, 0.875rem);
  --step-body: clamp(1.125rem, 1.05rem + 0.3vw, 1.3125rem);
  --step-lead: clamp(1.25rem, 1.1rem + 0.7vw, 1.625rem);
  --step-head: clamp(2rem, 1.3rem + 3.2vw, 4rem);
}
```
A text-first setting: a readable serif or a calm sans at 17–19px, with one display size for the name. Left-aligned throughout.
The direction decides sizes and how many of them; the type system decides how this face is set. Where the two give different tracking or leading, use the values of the type system: they are tuned to this face.
The display size itself comes from the hero blueprint below.

## 4. Colour — Stone and blueprint
```css
:root {
  --bg: #dfdedb;
  --surface: #d3d2ce;
  --ink: #26251e;
  --muted: #5f5d55;
  --accent: #2456d6;
  --accent-ink: #ffffff;
}
```
Stone paper carries the whole page with near-black and grey text. One image at a time is printed in the accent blue as its only ink, and the same blue fills the single button.
The paper colour of the reading column is chosen against the photograph: a tone taken from its shadows or its lightest area. One ink, optionally one accent ink for links and the action.

## 5. Imagery
**The project has no photographs of its own yet.** First choice: **real pictures of the subject's world from the open collections**: its city and street in old postcards and photographs, an old map of the neighbourhood, its materials up close, its trade in engravings, museum objects of its kind (references/imagery.md §3). Start with `node ~/AppData/Local/Temp/claude/C--Users-user-Desktop-sharethrone/725a4bdd-89bc-4f8f-8d81-ab622d19f68b/scratchpad/skills/taste-skill-src/skills/taste-skill/scripts/assets.mjs "bristol postcard"`, then "bristol old map" and "planetarium dome", then queries from your inventory (its street, its materials, its trade's tools and history). Download what you use into the project with `--get` and credit every one on the page (`--credits` prints the footer line). They show the world around the client, never the client's own room, dish, product, person or team. Second, for what no honest picture exists of: draw the subject's own things in code, large, in one flat graphic language and the palette's real colours (its tiles, its windows, its objects; references/imagery.md §8). Third: the dealt photographs, only where an honestly generic picture fits (the sea, the weather, a material), never one a visitor could take for the thing itself and never an unrelated scene. One treatment for all of them. If this direction is built on photographs, plan the set before building (four to six pictures of one family) and reuse them at different crops and scales instead of finding or drawing a new picture for every slot; keep drawings as `<symbol>` + `<use>` or generate them with a small script so the page stays light. List every found picture (with its source and licence), every dealt photograph and every drawing that stands in for photography in your report.

One excellent portrait-orientation photograph for the fixed half, plus three or four more that crossfade in as their sections arrive. Small inline photographs in the reading column are optional and should be few.

**Floor (the inspector checks it):** at least four real pictures on the page, usually six to twelve; SVG drawings do not count. One moving picture where an honest clip exists: `node <skill>/scripts/assets.mjs --video "<query>"`, used as a muted loop with a poster frame. Whatever this direction says about pictures, it is carried out with real ones.

Photographs dealt for this build, from the "Sky and space" set (skies, clouds, stars and night skies)
Order of preference for imagery: (1) the project's own photographs and product shots; (2) images you generate, if an image tool is
available; (3) real pictures of the subject's world found in the open collections (assets.mjs, downloaded and credited on the page);
(4) drawings of the subject in code; (5) these. They are real photographs (Unsplash licence, served by picsum.photos) of places, light and
texture. They are atmosphere, never a stand-in for a specific product, dish, person or interface. List the ones you use in your report so
they can be replaced.
- `https://picsum.photos/id/901/1600/1000` — Green aurora sweeping across the night sky above snowy ground · dark tone · emerald green, dark teal, slate · native square · photo: Marcelo Quinan
- `https://picsum.photos/id/765/1600/1000` — Night sky fading to an orange glow along a dark horizon · dark tone · midnight blue, burnt orange, black · calm area: top · native landscape · photo: Grant McIver
- `https://picsum.photos/id/869/1600/1000` — Starry night sky above a mountain glowing warm at its peak · dark tone · navy, deep blue, amber · calm area: top · native landscape · photo: Caleb Ralston
- `https://picsum.photos/id/505/1600/1000` — Tiny silhouetted figure on a dark ridge under a blue and orange twilight sky · dark tone · deep blue, orange, black · calm area: top · native landscape · photo: lee Scott
- `https://picsum.photos/id/114/1600/1000` — Dark storm clouds hanging over a faint mountain ridge · dark tone · dark teal, muted beige · calm area: all · native landscape · photo: Brian Gonzalez
- `https://picsum.photos/id/607/1600/1000` — Soft streaked clouds in a blue sky above a calm sea · mid tone · sky blue, white, teal · calm area: all · native portrait · photo: Grégoire Hervé-Bazin
- `https://picsum.photos/id/721/1600/1000` — Clear twilight sky fading from deep blue to an orange horizon · dark tone · navy, orange, dusty pink · calm area: all · native landscape · photo: Jeremy Ricketts
- `https://picsum.photos/id/974/1600/1000` — Person standing beneath the Milky Way with a warm glow on the horizon · dark tone · deep navy, violet, amber · calm area: left · native landscape · photo: Greg Rakozy
The last two path numbers are width and height: request the exact crop you render (`/1200/1500` for a 4:5 portrait, `/2400/1350` for a hero) and
add `?grayscale` to desaturate. Use three to six of them, not all eight. Always set `width`, `height`, `alt` and `loading="lazy"` (not on the hero image).
Other sets that fit this brief: city-night, architecture, fog-and-quiet, coast — `node draw.mjs --photos <set>` prints one in full; `--photos list` shows all sets. Taking photographs from another set is your call and is not a redraw; keep them to one light and one treatment.

## 6. Hero — Menu column, sticky photograph
The viewport is two panels of equal width with a 12px gap between them and 12px outer margins, both with a 20px radius. Right panel: one photograph of the place filling it edge to edge (object-fit: cover), set to position: sticky; top: 12px; height: calc(100svh - 24px), so it stays put while the left side scrolls. A solid bar in the paper colour, 56px tall, lies across the photograph's bottom edge inside the panel and carries opening hours on the left and the address on the right at 15px. Left panel: a filled paper-colour sheet set like a menu card, everything centred. Its top row holds the wordmark, about 300px wide, in the centre with one text link either side. Below, at 34% of the height, the headline in a light serif at clamp(40px, 3.6vw, 56px) on two lines in the accent ink, max 18ch per line. Under it one sentence at 17px, max 40ch, then the single action as a solid rectangle in the accent ink. The lower quarter of the sheet is empty before the first section begins.

On a phone: The photograph comes first as a rounded panel 56svh tall on 12px margins, sticky at the top so the paper sheet slides up over its lower part. The hours bar stays on the photograph's bottom edge. The sheet holds the wordmark centred at 46% of the width, the headline at 9vw on two lines, the sentence and a full-width action.
Headline: A warm 3-6 word line in a light serif, at most 34 characters, centred on two lines.

Adjusted for this page:
- It sets text at 12px: use your small step (13px or more) instead.
- It names its own corners: use the one radius of this page (square unless the direction says otherwise) and one button shape.
- It names a kind of typeface: use the dealt faces (display Rokkitt, text Literata) and their settings; capitals only where the type system sets them.

## 7. Sections, in order
Each is a composition to rebuild with this project's content. Keep the geometry; change the subject. Re-order or drop a
section when the argument of the page needs it (see references/copy.md), never to fall back to a card grid.

### 1. list — The printed menu sheet (band)
The list is one tall sheet of paper. A filled paper-colour panel 640px wide sits centred, or offset to columns 7–12, on a contrasting flat ground or a full-bleed photograph, with square corners and its top and bottom edges cut into shallow scallops by `clip-path` or an SVG `mask`. Inside, 64px padding and a centred, symmetrical setting like a printed card: the venue or section name at 40–48px, a one-line note at 15px, then three or four groups 56px apart. Each group opens with its name at 22px, centred. Items follow at 17px with 16px between them: the dish or service at the left of a 420px measure, the price at the right with tabular figures, and a one-line description at 14px in the soft ink under the name. Alignment alone links name and price; there are no leaders. The sheet ends with the opening hours as two centred lines. Holds 12–24 items.
On a phone: The sheet spans the screen with 10px of ground showing at each side and 28px padding; the scalloped edges stay at top and bottom. The measure is the full inner width and prices stay right-aligned. Group names at 20px, items at 16px.

### 2. contact — Clip-out coupon form (split)
The form is a coupon to cut out. One filled card in a strong flat colour, about 2.2:1, spans columns 2–11 with a 14px radius, with 120px of ground above and below. Semicircular notches 28px across are cut into its top and bottom edges at the 46% mark with `clip-path` or an SVG `mask`, and a vertical row of round dots (a dotted `border-left` 5px thick in the ground colour) joins the two notches as the perforation. Left of the perforation, with 48px padding: a two-line heading of three to five words at clamp(28px, 3vw, 44px) in heavy capitals at the top, and one sentence at 16px at the bottom. Right of it: four inputs stacked with 10px gaps, each a filled block 52px tall in a lighter tone of the card with its placeholder at 16px, then a full-width solid submit bar 56px tall in the darkest ink. Inputs keep visually hidden `label` elements.
On a phone: The card turns upright and spans the width with 12px margins. The perforation becomes horizontal: notches on the left and right edges, dots across. Heading and sentence sit above it; the four inputs and the submit bar sit below, each 54px tall.

Adjusted for this page:
- It names its own corners: use the one radius of this page (square unless the direction says otherwise) and one button shape.
- It names a kind of typeface: use the dealt faces (display Rokkitt, text Literata) and their settings; capitals only where the type system sets them.
- It draws lines or borders: keep them only inside a table, a form or a `data-ui` fragment; elsewhere separate with space or a change of ground.

### 3. how-it-works — Itinerary on a slip (table)
The process is printed as an itinerary or receipt. One paper-coloured slip 420–480px wide stands on a contrasting flat ground at columns 7–11; its bottom edge is a zigzag cut with clip-path and its top edge is straight. The heading, two to four words at clamp(40px, 4.4vw, 68px), sits in columns 1–5 with a 30ch paragraph, top-aligned with the slip. On the slip, in the text face at 16px with tabular figures: stages grouped under time headings that carry the order ('Monday', 'Midweek', 'Friday'), each heading in bold with 28px above it. Under a heading, two to four rows; each row has what happens on the left, a duration or time on the right, and a leader of round 4px dots between. The final row is inverted, paper-coloured text on a full-width ink bar 44px tall, and states the result: what the reader holds at the end and the total time.
On a phone: The heading and paragraph come first. The slip follows at full width minus 16px margins, keeping its zigzag bottom edge. Rows keep left text, dots and right value on one line; long descriptions wrap under the left text. The inverted final row stays full width.

### 4. showcase — Object on a disc (index)
One section about a viewport tall on a plain ground. A flat filled disc 64vh across, in one palette colour, sits with its centre at 58% of the width; swap the disc for the subject's own shape if it has one. A cut-out object photograph stands in front of it, centred on it, about 78% of the disc's height and breaking the disc's outline at the top. Bottom-left, starting on the margin and ending level with the disc's lower edge: a vertical list of four to six item names at clamp(28px, 2.9vw, 44px), leading 1.15. The selected name is in full ink and the rest at 25%; choosing a name swaps the object (300ms crossfade) and the disc's colour. The heading, two to four words at the same size as the names but in the soft ink, is the first line of that list. One fact about the selected item sits at the disc's right edge at 17px.
On a phone: The disc comes first, 86vw across and centred, with the object in front. The names follow as a single sideways-scrolling line at 26px with 24px gaps, the selected one in full ink. The fact sits under the names at 16px.

Adjusted for this page:
- It fades text: anything a visitor reads, inactive items of a list included, is `--ink` or `--muted` at full strength (contrast 4.5:1 or more). The active item is told apart by `--ink` against `--muted`, weight or the accent, not by fading the rest.

### 5. pricing — Prices like a menu (list)
Prices set the way a good menu sets them. A heading of two to three words at 32–40px sits in columns 1–3. Columns 5–12 hold the list in two text columns with a 64px gap. Items are grouped under plain group names at 24px in medium weight with 48px above each group. Each item is a row: the name at 19px on the left, the price at the same size on the right on the same baseline, with tabular figures and the currency sign shown only once per group; beneath the name, one line of description at 16px in the softer ink, no wider than 36ch. Rows are 22px apart. Name and price are joined only by empty space (flex with `justify-content: space-between`), with no dot leaders and no lines. One small photograph (4:5, three columns wide) may sit under the heading in the left zone.
On a phone: One column. The heading at 28px, then the photograph at 60% width if used. Each item keeps name left and price right on one line at 17px, with its description beneath in the softer ink. Group names at 20px with 40px above.

### 6. testimonial — Quotes on paper slips (strip)
A heading of two to four words at about 3vw sits top-left in columns 1–6. Beneath it three paper slips fan across columns 2–11 on a contrasting flat ground: each is a filled paper-coloured panel about 300px wide, of three different lengths (440, 520 and 600px), rotated −4, 2 and 5 degrees and overlapping its neighbour by roughly 40px. The top and bottom edges of each slip are zigzag tears cut with `clip-path: polygon()`. On a slip, from the top: the speaker's name centred at 18px bold, the role on one line, a square greyscale photograph of the speaker 120px wide, then the quotation at 16px in the text face, left-aligned with 24px padding. The longest slip runs past the bottom of the section, which clips it with `overflow: hidden`, so the slips look freshly printed rather than laid out as cards.
On a phone: One slip per row, each 78% of the screen wide and kept rotated by two or three degrees in alternating directions, the first hugging the left margin, the second the right, each overlapping the one above by 32px. The heading sits above at 8vw on two lines.

### 7. faq — Questions beside a tall photograph (split)
One field in a deep or pale flat tone, split 5 to 7. Left, columns 1–5: a heading of two to four words at 36–44px; under it the three or four practical facts people ask first (address, times, how to arrive) as plain lines at 18px; then five or six questions as an accordion with no lines between rows, only 28px of space. Each question is a `<summary>` at 20px with a small chevron drawn from two 2px strokes at its right; the open answer sits beneath at 17px in the softer ink, on the same left edge. One filled button and one text link close the column. Right, columns 7–12: a tall photograph (3:4) that bleeds off the top edge of the section and stops 80px short of the bottom, with a second, square photograph a third of its width overlapping its lower-left corner by 40px.
On a phone: The tall photograph leads at full width, 4:5, with the square one overlapping its bottom-right corner at 38% width. Heading at 30px, facts at 17px, then the accordion at full width with 24px between questions and 44px touch targets. Button full width at the end.

### 8. footer — A single quiet line (strip)
No footer block at all. After at least 30svh of empty page ground, one baseline 30px above the bottom edge carries three items in the page's ordinary small text size (14–16px), the same size and weight as the navigation: at the left margin, what this is in three or four words; centred, two to five links separated by 24px of space; at the right margin, the year and a privacy link. Use a three-column grid (1fr auto 1fr) so the centre group is truly centred whatever the side lengths. No logo, no background change, no line above it, no columns. Links are in the full ink; the rest in the softer ink. If the page has a narrow text column, the three items may instead sit on that column's left edge as one wrapped line. The restraint only works if the page above has already made its invitation.
On a phone: Three short lines at 14px on the left margin with 8px between them: the description, the links on one wrapped line, then year and privacy. Links stay in the full ink. Leave 20svh of empty ground above and 24px below.

## 8. Motion — Slow crossfade
The principal photograph quietly changes: three or four images of the same framing crossfade over about 1.6 seconds, holding five to six seconds each, or the fixed image changes when its matching section arrives. It should feel like light shifting in a room. Pause when the tab is hidden; under reduced motion show only the first image.
This is the signature move, the one loud moment. Around it the page still needs the rest of the motion layer (SKILL.md step 9): an entrance for the parts of every section, feedback on every control, and one moving picture. The inspector fails a page that barely moves. Recipes: references/motion.md.

## 9. Where the richness comes from
A full-height photograph always in view, against a quiet, well-set column of text.

## 10. How this direction goes wrong
The reading column must be genuinely well typeset, since it is half the page. Do not put cards or grids in it. Check that the fixed image crops well at short viewport heights. In blind tests the commonest complaint was monotony: several screens of the same split with only the text changing. Change what the reading half holds from section to section (a list, a large figure, a table, a full-colour panel, a drawing) and let at least one section break the split and run full width.

## 11. Real references
Studied sites that work this way. Take the principle, never the specifics: no names, copy, images or signature assets of theirs.
- **Le Petit Bleu** (One Page Love, studied score 7.8) `opl-le-petit-bleu`
  Idea: The cafe's teal-and-white stripes become the whole system: one teal ink on cream, everything boxed in thin double outlines like printed menu cards, beside a permanently visible photo of the room.
  Hero: Viewport split 50/50 with 12px gaps. Left: a cream panel with a thin pale-teal outline and rounded corners. Inside, a second outlined box is the nav bar: two-line hamburger left, teal brush-script logo about 300px wide centred, a caps order link right. Below, a two-line light serif headline at about 3.5vw, centred, in teal, then two centred stacks, each a bold caps label, a serif line and a solid teal button wrapped in a second thin outline. Right: a full-height rounded photo of a striped booth, with an outlined hours bar across its top, three slide dots bottom-left and a white address card bottom-right.
  Worth taking: Print everything in one brand ink on cream, including body text, rules and buttons, and let photographs supply the other colours.
- **Flowjam** (One Page Love, studied score 6.8) `opl-flowjam-2`
  Idea: A persistent frosted sidebar card holds the pitch and buttons while a soft-serif content column scrolls past it; beyond that there is no idea stronger than proof by logos and videos.
  Hero: Grainy off-white page. A tall rounded card about 26% of the viewport wide is pinned at the left and runs off the bottom; frosted over blurred pink and yellow flowers, it holds the logo, a two-line serif tagline, a short blurb and two pill buttons in hot pink and charcoal. The main column starts 32% from the left: a hot-pink pill badge, a three-line serif headline at 4.2vw, then a paragraph whose first sentence is grey and the rest black. A lone charcoal pill button floats far right at mid-height. Below: a heart icon with a proof line, then the top of a four-column logo grid with dotted borders.
  Worth taking: Pin a narrow card with logo, one-line pitch and both actions beside the scrolling content, so the call to action never leaves the screen and no top nav is needed.
- **Mosby’s Files** (Awwwards Site of the Day, studied score 8) `aww-mosbys-files`
  Idea: An archive of architects presented as a physical stack of coloured index-tab file folders that you flip through.
  Hero: Charcoal background. A slim top bar over a hairline holds the condensed wordmark dead centre and one mono link far right. The headline is one centred line of heavy condensed uppercase at 10vw, about 71% of the width, in the upper quarter. Beneath it, a three-line centred serif paragraph in mid-grey, about 42% wide. From 51% down sits a stack of file folders 86% of the viewport wide: each is a flat colour band with one to three trapezoid tabs rising from its top-left edge, each tab a different colour holding a serif name. Mono category labels sit right-aligned. The front red folder is open, running off the bottom.
  Worth taking: Build navigation as stacked full-width colour bands with trapezoid tabs, each tab coloured to clash with the folder behind it.
Full notes on any of them: `node draw.mjs --site <id>`.

## Always
Headlines two lines or fewer at every width. No eyebrow labels, no monospace, no 01/02/03, no thin rules, no gradients, no shadows at rest,
no cream-and-terracotta, no violet. `inspect.mjs` measures all of it.

Saved to examples/benchmark-orrery/taste-skill/.taste-skill/direction.json · reprint with `node draw.mjs --show`
