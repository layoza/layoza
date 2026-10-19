---
name: layoza-art-direction
description: >-
  World-class art direction for new websites, apps and redesigns (Mode A/D):
  turn a brief into ONE committed visual world that is unmistakably about its
  subject, build it at studio quality (art, type, colour, layout, motion), and
  iterate on screenshots until it beats what a top studio would ship, with
  measured `layoza qa` as the floor. In Mode B/C use only its preservation and
  finish rules; the existing brand DNA wins.
---
<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774: reference/new-work.md (rut, seven candidates, fusion and raises, calibration and rendition prior, "build the assigned direction", web leverage), reference/visualize.md ("commitment is depth"), reference/mode-persuade.md, and agents/impeccable-finish-reviewer.md (TYPE / MATERIAL / GROUND / ceiling checks). Modified by Layoza: rewritten around `layoza design-system --json`, `layoza deal` (offline, reproducible candidate deal) and measured `layoza qa`; "world from far, subject up close" fusion; Mode B/C brand preservation; impeccable engine plumbing (concept-seed catalog, serve-question, build-phase, comp-spec, plates) removed. Ideas adapted in Layoza's own words: subject inventory, section argument, idea sentence, copy-deck-first, scored screenshot critique with caps (Leonxlnx/taste-skill, MIT); subject grounding and token plan with ASCII wireframes (anthropics/skills frontend-design, Apache-2.0). See LICENSE and NOTICE.md in this folder. -->

# Layoza: Art Direction

You are the best designer in the world on this job. Your page has to be
unmistakably about its subject, impossible to guess from the category name, and
better than what any top studio would ship, while still working perfectly as a
page. Passing `layoza qa` is the floor. The goal is the design.

| Reference | Load when |
| :--- | :--- |
| [reference/world.md](reference/world.md) | Steps 2–4: candidates, deal, fusion |
| [reference/art-production.md](reference/art-production.md) | Step 6: drawing the subject, illustration, assets, type as image |
| [reference/craft-floor.md](reference/craft-floor.md) | Just before the first UI edit |
| [reference/system-passes.md](reference/system-passes.md) | A colour, type, layout, or "make it bolder" pass |
| [reference/finish.md](reference/finish.md) | Step 8: art loop, reviewer, QA |
| [reference/surfaces.md](reference/surfaces.md) | App/dashboard (Operate), docs/article (Read), portfolio (Experience) |
| [reference/engineering-lessons.md](reference/engineering-lessons.md) | CLS, sticky headers, font swap, canvas sizing |
| [../layoza-animations/SKILL.md](../layoza-animations/SKILL.md) | Step 7: motion |

## 0. Authority (decide first)

1. **The user's prompt wins.** An explicit instruction (style, palette, font,
   stack, layout, motion library, "minimal", "no animation") overrides every
   Layoza default, ban, deal and `avoid_recent` entry, and is followed exactly;
   a pinned world pins the world, not its softest rendition. Everything below
   governs only what the prompt leaves open.
2. **Mode A** (new): this whole skill. **Mode D** (redesign): keep product truth,
   content, routes, functions and explicit brand commitments; replace the
   visual world. **Mode B** (extend): inherit `SITE-DNA.json` / `DESIGN.md` /
   `MOTION.md`; use steps 6–8 only. **Mode C** (improve): steps 6–8 only, inside
   the incumbent world. In B/C never offer directions, sketches or a pick.
3. Bans here apply to new work. A brand's own DNA (Inter, cream, numbered
   sections) is kept in B/C, and in D unless the user drops it.
4. **Intake: know what the site is for before any look.** Fill these from the
   prompt; ask the user, in one short message, only for what it leaves open:
   - **Purpose:** what the site must achieve (sell, book, sign up, install,
     explain, recruit) and the one primary action.
   - **Audience:** who arrives, from where, knowing what.
   - **New or existing:** a new site (Mode A), or an existing site, repo or
     brand to extend, improve or redesign (B/C/D). Check the folder too: an
     existing project means `layoza inspect . --write` first.
   - **Must-keep:** real content, proof, brand assets, platform.
   Never invent a purpose the prompt doesn't support. If nobody can be asked
   (unattended run), write the assumption and why.
5. **Re-read `.layoza/direction.md` first** whenever context was compacted or a
   session resumes; it is the memory of every decision below.

## 1. Inventory the subject (before any look)

```bash
layoza design-system "<brief>" --mode new --json
```

Treat the engine output as raw material, not answers: `distinctiveness` (place
and material cues, category clichés), `match_confidence`, and three structural
directions. Its palettes and fonts are a fallback register only.

**Avoid your own recent work.** MCP `get_design_system` returns `avoid_recent`:
the directions, palettes, type families, motifs and mechanics of the last 8
picks across the user's projects (CLI: the last 8 lines of
`~/.layoza/history.jsonl`, or `$LAYOZA_HOME`). All three directions must differ
from them in palette family, type families and motifs. Reuse a recent look only
when the user asks for it, and record `REPEAT-OK: <the user's words>` in
`direction.md`; otherwise `layoza audit` warns `DESIGN-REPEATS-RECENT`.

Then write:

- **Subject facts:** 8–12 concrete facts from the brief (who, where, what is
  sold or done, prices, people, hours, what makes it different).
- **Subject objects:** the 5 things a visitor sees and handles in this
  business. Circle the most characteristic one.
- **Argument:** one claim and its proof per section. A section with no claim is
  cut; the order follows the argument, not a stock template.
- **Scene:** who uses it, where, under what light. This decides light or dark,
  never the category.

**Software, developer tools, APIs, MCP servers and AI products:** the subject
objects are the product *working*: the terminal session, the agent's tool
calls, the editor or IDE, the before/after output, the config snippet, the
diff. Draw those at scale and let the visitor run one. A metaphor world (a
casino, a kitchen, a cockpit) may dress the page, but the hero must still show
the real tool doing the real job, and the subject test applies with no
exceptions.

## 2. Name the rut (treatments, both ends)

- **Reflex:** the layout, palette and type this category always ships (start
  from `category_cliches`).
- **Obvious opposite:** the predictable reaction against the reflex (cream
  craft → black brutalist craft). Both ends are the rut.
- **Calibration:** generated pages cluster on cream + serif + terracotta;
  near-black + one neon accent + glow; broadsheet hairlines + italic serif +
  tiny mono labels; flat ink + black boxes + hard offset shadows; and the
  Layoza house skeleton (split hero, stacked full-width bands, every section
  head "title left, note right", mono labels, every section a paper ticket or
  receipt). **Treat your
  first palette as already spent:** if OWN-WORLD reads cream, paper, parchment
  or lamplight and the brief didn't pin it, rework it from the world's real,
  saturated materials.

The subject is never the rut. Its objects are your best material; only the
generic *treatments* are banned.

## 3. Seven worlds, then deal

List **seven** candidate worlds the audience knows by heart, across at least
three families: place (architecture, signage, transit, landscape, light);
notation and print (charts, maps, manuals, packaging); screens, light and
signal; material and process; rendered objects and scale; ritual; and the
subject's own objects. Give each two lines: why it
fits and can carry the argument, and how it looks (ground, colours, type voices,
one signature). Order them by resonance, then let the dealer pick from your top four:

```bash
layoza deal "<the seven candidates, one per line>" --top 4
```

**Fit-check the dealt world** before using it: (1) does the audience already
know this world by heart, (2) can it carry the argument section by section,
(3) does it make the product clearer, not just prettier? Two "no" answers mean
re-deal or take the next candidate, and say why. The deal exists because a
model's first pick converges on the category; overrule it only when the dealt
world fails truth, clarity or this fit-check. Detail and a worked example:
[reference/world.md](reference/world.md).

### 3b. Pick by niche fit (ask only when asked)

**Existing site (Mode B/C)? Skip directions and sketches:** use
[layoza-extend-website](../layoza-extend-website/SKILL.md) or
[layoza-refine](../layoza-refine/SKILL.md) and follow the site's own design.

Shape the dealt world and the two strongest challengers into **three directions
that differ in layout, identity, type, content treatment and motion**, each on
its dealt `page_shape`.

**Decide, don't interview.** Pick the direction that best fits the niche and
the brief by the fit-check, record `PICKED: auto — <direction>, <why it fits
the niche>` in `direction.md`, and build. Ask the user only when they asked to
choose, or when `understood_brief.open_questions` holds something that blocks
building. Anything the user named (colour, font, style, motion level) is kept
exactly; every other axis stays as dealt.

When the user asked to choose: write one small static hero per direction
(`.layoza/sketches/a|b|c/index.html`, real fonts, palette, copy), screenshot
them (MCP `capture_screenshots` with `sketch: true`, or `layoza qa`), show the
images with one line of strength and weakness each plus your recommendation,
wait, and record `PICKED: <direction> (chosen by the user)`. `layoza audit` /
`qa` fail with `DESIGN-NO-DIRECTION-PICK` until a `PICKED:` line exists.

## 4. Fuse: world from far, subject up close

- The dealt world supplies the **ground, palette, type voice and structure**.
- The circled subject object becomes the **signature**: drawn at scale in the
  first screen and, ideally, the thing the visitor operates. Example: a
  harbour-chart world for an optician, with a lens loupe that magnifies it.
- Write the **idea sentence**: "On this page, [subject thing] becomes [device]."
  If a competitor could use that sentence unchanged, it isn't an idea yet.
- Weigh the next two candidates as challengers on **audience recognition** and
  **product clarity**. A declined challenger donates one *discipline* (total
  palette commitment, a grid's density courage), written as a "raise" line;
  never donate clothes.
- **Subject test:** cover every word on the planned first screen. A stranger
  must still name the business from the picture.
- **Sibling test:** would you write this exact plan for a neighbouring
  business in the same category? Every block that matches is a default: revise
  it and note what changed.
- **The page shape is dealt and binding.** Build `page_shape` as dealt: hero
  frame, flow, section heads, ground, material, labels and motion. It exists
  because agents drift to one skeleton (headline-left / visual-right over
  stacked bands). Change an axis only when the user named it, or when it fails
  the subject test; record the change and the reason as `HERO:` / `SHAPE:` in
  `.layoza/direction.md`.
- **One motif, one button, and restraint are binding too.** `page_shape.motif`
  is the single device the page repeats: media frames, the primary button,
  list bullets and one divider between chapters. It is drawn from one of the
  trade's own objects, and nothing else decorates. `page_shape.button` decides
  the button shape once for the whole page. `page_shape.restraint` lists what
  the page leaves out (eyebrows, mono, index numerals, glow, icon cards, extra
  typefaces...). Each rule already yields to what the brief asks for, so build
  it as written. A premium page comes from one idea applied with discipline,
  not from six decorative choices stacked on top of each other.

## 5. Contract and copy (before code)

Write `.layoza/direction.md`, a dev-only file that is never served:

- **PURPOSE:** the intake from step 0: purpose and primary action, audience,
  new or existing, must-keep. `layoza audit` fails with `DESIGN-NO-PURPOSE`
  until this line exists.
- **PICKED:** the auto pick or the user's choice from step 3b (Mode B/C, if this file is kept:
  `PICKED: existing — <the site design being followed>`).
- **THESIS:** the idea sentence and the category arrangement it refuses. Name
  the trade's objects (the catch board, the lens, the ticket), not style words:
  `PURPOSE`/`PICKED`/`THESIS` made only of "modern, clean, bold" fail with
  `DESIGN-GENERIC-DIRECTION`.
- **OWN-WORLD:** palette roles, materials and component language, recognisable
  with all content removed.
- **TOKENS:** 4–6 named hex values, 1–2 families with roles, a 5-step type
  scale, the alignment rule. `layoza audit` records the pick's hexes, families,
  motifs and mechanic ids (never copy or paths) in the history ledger that
  feeds `avoid_recent`; Mode B/C picks (`existing`) are not recorded.
- **FIRST VIEWPORT:** the picked sketch's screenshots (desktop and phone). Note where
  the subject art sits (the dominant mass, about 40% or more) and where the real
  action works (buy, book, try; compact, never a second focal panel).
- **HIERARCHY:** before any layout, what the visitor must take in 1st, 2nd and
  3rd on the first screen and on each section; layout serves this order.
- **SECTIONS:** one line each: claim, proof, and the composition that fits its
  content (a wall of objects, a ticket, a diagram, a map, a single quote at
  scale). No two consecutive sections share a composition.
- **ART PLAN:** one row per section: the object drawn, its named parts, light,
  people or props, and what moves (`reference/art-production.md` §5). Build
  the weakest row first.
- **MOTION:** what moves and why, chosen per design (see step 7).
- **FINISH:** "Unverified is unfinished: art loop done, `layoza qa` 0 errors."

Then write the **copy deck**: headlines of two lines or fewer; CTAs that name
the outcome and keep that name through the flow; no invented proof (prices,
awards, customers); illustrative content labelled as such.

## 6. Build the committed world at full strength

Load [reference/craft-floor.md](reference/craft-floor.md) and
[reference/art-production.md](reference/art-production.md).

- **Build the dealt world, not a safer interpretation of it.** In unattended
  work, the safe rendition is the known risk. Land the first build fully
  committed; later passes clarify and never dilute.
- **Commit every atom:** nav, buttons, inputs, links, focus rings, selection,
  scrollbars and the bag/cart are rebuilt in the world's vocabulary. A stock
  component in a committed world is a lapse.
- **Author the assets** at production fidelity (the art-production fidelity
  bar). Gradients, glass and icon tiles where an asset belongs are the gap
  wearing chrome.
- **Build the form's web leverage:** when the world implies a technique
  (canvas, WebGL, SVG drawing, scroll-driven change, view transitions), build
  the technique itself, not a static imitation of it.
- **Type:**
  - Display tracking −0.02 to −0.05em, leading 0.85–1.05, about 4× body size
    (3× is the floor).
  - Headlines run two lines or fewer at every width: cut words first, then
    widen the measure, and resize last. Large intro sentences are paragraphs
    (about 25 words, statement size), not headlines.
  - Break display lines by hand: no orphan word, and no line ending on "a",
    "the" or "of". Pull round or pointed capitals (O, C, A, V) slightly left
    so the edge reads straight.
  - Body 16–20px, leading 1.45–1.6, 45–75ch.
  - One family first, two at most, paired by era and skeleton
    (`reference/ceiling.md` §2). Use two or three display sizes and nothing
    in between.
  - Type may *be* the image.
- **Space and colour:**
  - Section gaps of 120–220px on desktop, clearly larger than gaps inside a section.
  - Count the left edges in the screenshot. Text blocks share 2–3 anchor lines,
    and headings, prices and footer links start on them too.
  - Colour owns whole sections (fields, not sprinkled accents):
    - the ground covers 70–90% of the page;
    - the accent stays under 5% of any screen and marks the action;
    - each field has its own ink at full contrast;
    - neighbouring fields differ in lightness, not only in hue.
  - One depth strategy.
- **Quiet, minimal, simple or calm in the prompt** overrides the colour fields
  and house moves:
  - one ground and one accent;
  - no coloured section blocks unless the user asks for them;
  - ornament only where the world needs it.

  Restraint still needs substance, so fill freed space with scale (bigger
  type), real content and one carefully drawn artefact, never with another
  ornament. Short pages are fine. A page built only by removing things reads
  as thin, not quiet.
- **The idea reaches the end:** pricing, FAQ, form and footer speak the world
  too. The footer is a designed close (the name again, the place, the
  signature object small), not a link-column template. A strong hero followed
  by stock blocks is the most common way a good page loses.
- **Controls mean what they do:** one-of-many choices (billing period, sizes,
  days) are radio groups or segmented controls. Never use checkboxes that
  could both be ticked.
- **Commitment is depth:** one dominant move per screen, supported by material,
  type and spacing; regions that compete with the lead at the same scale make
  the page louder, not bolder.

## 7. Motion: designed, not defaulted

Build `page_shape.motion` as dealt: its tier follows the niche (and the
user's words win), its tools are named per technique (GSAP + ScrollTrigger,
Three.js / R3F, Motion, CSS scroll-driven animation, View Transitions, WAAPI,
canvas or SVG). A heavy tier means the heavy techniques ship for real: a pinned
scene that changes, type that moves with meaning, a live canvas or WebGL
object. Each has a static frame visible without JS, without WebGL and under
reduced motion, so screenshots never show an empty slab. Then decide what each
move makes *this* subject understood or felt. Values, tools and guardrails:
[layoza-animations](../layoza-animations/SKILL.md). Never one identical fade-up
on every section; content is visible without JS; reduced motion is honoured.

**Never cut motion to pass QA.** When a QA rule fires on an animation (CLS,
overlap, hidden text, LCP), fix the cause: sequence it after first paint,
animate only `transform`/`opacity`, reserve its box, or mark decorative layers
`aria-hidden`. A timid page that passes is a failed design, not a safe one.

## 8. Art loop, then the floor

Load [reference/finish.md](reference/finish.md).

1. **Screenshots:** `layoza qa <file>` captures 1440/768/375. Also capture
   the first screen and a mid-scroll frame of the signature. Judge the design
   by the screenshots, taken after motion settles (entrances finished, canvas
   drawn), not by the report. Check by eye that every label or headline over
   3D, canvas, video or images is readable at each width; never rely on the
   automated contrast checks alone there.
2. **Art loop (at least 3 rounds; art has no two-round cap):**
   1. Critique like the harshest art director: subject test, the one-second
      test, squint test, cover-the-logo, sameness, the phone composition,
      reading order, states, the cliché swap and the TYPE / MATERIAL / GROUND
      checks.
   2. Score it as 0.4 design + 0.3 usability + 0.2 creativity + 0.1 content.
   3. Fix the top 3 problems in order: idea → hierarchy → composition → type →
      art → colour → detail → motion.
   4. Run the usability pass (finish.md) every round, and the ceiling pass
      (`reference/ceiling.md`) once the score reaches 8.
   5. Stop when the score is ≥ 9 and the fresh-eyes reviewer agrees, or after
      6 rounds; report the reviewer's score.
   6. **Log every round** in `direction.md` as
      `ROUND <n>: <score> — <what changed>`, and the reviewer's verdict as
      `REVIEWER: <ship|fix|rebuild> <score>`. `layoza audit` / `qa` fail with
      `DESIGN-ART-LOOP-INCOMPLETE` until 3 rounds and a verdict are logged.
3. **Fresh-eyes reviewer:** when subagents are available, spawn one with no
   build context. Give it the screenshots, the brief and `direction.md`. It
   returns ship / fix / rebuild using the finish.md checklist.
4. **QA floor:** fix every `layoza qa` error at the cause; never game a
   checker. The gate measures defects only: **PASSED never means the design is
   good.** Only the art loop, the reviewer and the user decide that. `CRAFT-*` notes are reviewed against the direction, not obeyed blindly.
5. **Report honestly:** the score, QA errors (0 or a list), metrics with nulls
   named, what is still open, and anything the user should confirm.

## The bar: best in the world, never a template

A page ships only when a designer would put it in their portfolio:
- one idea a competitor could not reuse unchanged, visible in the first screen;
- one unforgettable signature move, built at full strength, not hinted;
- a page shape no other recent Layoza site used (the ledger proves it);
- type, colour and space with intent at every width, with no defaults left in;
- motion that explains or delights, heavy only where the niche allows;
- usable first: the main action, readable text and keyboard access never lose
  to the art.

**Every section earns its place.** Each one answers a question this niche's
visitor actually asks, shows this subject (not a stock block that would fit any
site), and has its own composition from its content: no two sections on the
page share a layout, and none is a reflex filler (logo wall, icon-card
features, testimonial carousel, stat row, generic CTA band) unless the content
truly is that, then built in the world's language. Cover the headings: a
stranger should still name the business from each section alone. The nav and
footer obey the same rule.

Balance: ambition never excuses broken usability, and passing QA never
excuses a plain page. When the two conflict, keep the idea and fix the cause.

## Absolute refusals for new work (the worst tells only)

Each is allowed only when the prompt or brand asks for it:
- gradient text;
- a side-stripe accent border on cards;
- kicker/eyebrow labels above headings;
- the hero-metric block;
- identical icon-card grids as the page structure;
- nested cards;
- glass and glow as decoration;
- fake material (CSS bevels, embossing, chalk and metal effects imitating
  something the page doesn't render).

Every other habit on the craft floor is a **reflex check**: allowed when you can
state why this world needs it.
