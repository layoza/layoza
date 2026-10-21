<!-- Layoza-original synthesis (2026-10-15). Sources: Awwwards evaluation criteria (awwwards.com/about-evaluation: Design 40 / Usability 30 / Creativity 20 / Content 10); a juror's write-up (hontran.dev, "how to win an Awwwards Site of the Day"); Butterick, Practical Typography, "Mixing fonts"; Leonxlnx/taste-skill research.md and layout.md (MIT, model-panel study of 248 sites, figures quoted as that study's, not human-jury data); blind-jury notes from Layoza's own 2026-10 benchmark rounds. Items marked [judgment] are Layoza's own inference. -->

# The ceiling: what lifts an 8 to a 9+

Pages at 8 are good. Pages at 9+ pass every test below. Run this list in the
art loop once the page scores 8; each failed line is a named fix.

## 1. Twelve tests

| # | Test (check it in the screenshots) | Fix when it fails |
| :--- | :--- | :--- |
| 1 | **The idea lasts to the last pixel.** Cover the logo: could a competitor use this page? Does every section, including the form and footer, show the motif? | Carry the world into the section that went generic; never end on stock blocks. |
| 2 | **One dominant element per screen.** In each viewport the largest element is at least 2× the next (area or type size). | Cut annotations or secondary cards, or scale the lead up. |
| 3 | **Type at architectural scale.** The largest text is about 8vw or more on desktop and the hero headline 6vw or more (in the study, strong pages had a median of 8.4vw, weak pages 5.0vw). Exception: a quiet brief built on small type plus a live demonstration. | Shorten the headline so it can grow. |
| 4 | **Two or three sizes, nothing between.** Count the font sizes outside body copy: no more than 3. A smooth five-step scale reads as a template. | Merge the in-between sizes. |
| 5 | **Usability polish.** All of these hold: <ul><li>body text 16px or more and nothing under 13px;</li><li>field edges 3:1;</li><li>submit button 44px or more;</li><li>radios for one-of-many choices;</li><li>mobile labels two lines or fewer;</li><li>the primary action in the header at every width.</li></ul> Per the juror source, this is the gap between an Honourable Mention and Site of the Day. | The usability pass in `finish.md`. |
| 6 | **Planned emptiness only.** The gaps between sections are larger than any gap inside one. Nothing is left blank under the footer or under a pinned stage, and every void has a matching mass on the other side. | End the document on the last element; move or cut the void. |
| 7 | **A coherent motion identity.** Name the thesis and the content transformations that express it. Rich-motion briefs can have several substantial chapter mechanisms, with quiet reading intervals. | Cut unrelated effects; strengthen section-specific transformations rather than repeating entrances or stopping after the hero. |
| 8 | **No layout twice in a row.** Label each section with one layout word (list, split, statement, full-bleed, mosaic, table). No word appears twice in a row or three times on the page. | Re-compose from the content. |
| 9 | **Ornament is a budget.** One accent device (a rule, a stamp, a colour) appears on 3 or fewer element types. [judgment] | Strip it from everything except where it means something. |
| 10 | **The subject's objects are drawn in detail.** A stranger can name the object and see how it works. | Apply the fidelity bar and the detail density test (§5) in `art-production.md` to every section, not only the hero. |
| 11 | **Archives get scale.** Project, product and show lists are a few large items (desktop thumbnails 25vw or wider) or one item per view. Never a table with tiny thumbnails. [judgment] | Enlarge the items, or show one at a time with an index. |
| 12 | **The ending is designed.** The page closes on the name at full width, an inverted hero, or one line. No link-column footer template. | Redesign the close in the world's vocabulary. |

## 2. Type pairing without the clash

- **One family first.** Use weight, size, width and italic before adding a
  second face. Two faces at most.
- **Low contrast usually beats high contrast.** A heavy grotesque display over a
  bookish old-style body is the clash pattern juries name.
- **Pair from the same designer or superfamily**, for example IBM Plex
  Sans/Serif/Mono or Source Sans 3/Source Serif 4. Match the era and skeleton:
  - grotesque with a transitional serif;
  - humanist sans with an old-style serif;
  - geometric sans with a didone.

  [judgment]
- **Keep x-heights within about 5%** at equal size, and keep the paragraph
  colour (weight density) similar.
- **Give each face one job.** Never mix faces inside a paragraph.
- **Strong contrast only at the extremes and only as the concept,** repeated
  at every scale, never by accident between H2 and body.
- **Blur test:** blurred to about 4px, the headings and the body should still
  read as one voice.

## 3. Quiet briefs that score 9+

- Restraint must be complete: one ground plus one accent, at most 2 section
  grounds, square or near-square corners, no resting shadows, no gradients.
- Restraint still needs one live thing: a working micro-demo, a slow ambient
  drawing, or a detailed drawn object.
- An empty-feeling section gets a bigger main element or is cut; never add
  decoration. A quiet page that reads as empty fails.

## 4. Motion and 3D at the top level

- Motion and WebGL carry the story or a feeling. As decoration they read as
  noise.
- On animated briefs, one scroll-driven thread runs the whole page
  (`layoza-animations` §2e). Motion confined to one pinned section loses to a
  page that tells one story end to end.
- Choreography (timing, easing, order) matters more than effect count. Several
  earned transformations can serve one thesis; unrelated novelty weakens it.
- **Every still is finished art:**
  - The rest state, the reduced-motion state and every frame of a pinned story
    are complete compositions.
  - Test with frames at 0, 25, 50, 75 and 100% of each pinned segment; none
    may be empty or faint.
- Strong sites mostly use pinned stages, masked line or blur reveals, media
  swapped behind a fixed element, and scroll-linked scale. Parallax and spring
  bounce are rare among them.
- 3D needs lighting, a material story and a camera that tells a beat; a plain
  model is not direction. Inspect actual playback and interaction. Speed measurements are optional.
- No gate, no long preloader, and no blank first paint.

## 5. Things that look impressive and lose

- Information hidden behind the concept.
- Text at 9–13px.
- Text over art without a scrim.
- A mobile layout bolted on after desktop.
- Template devices:
  - icon-topped cards;
  - pill badges;
  - three or more tiny tracked uppercase labels;
  - gradient text;
  - a centred headline with the media below it.
- Recolouring the page mid-section.
- Several effects competing, or trends with no reason.
