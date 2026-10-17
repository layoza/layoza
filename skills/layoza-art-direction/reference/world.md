<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/new-work.md (sections 1-5) and reference/mode-persuade.md. Modified by Layoza: concept-seed dice, catalog challengers, serve-question decision page, and comp/image-generation rounds replaced by fusing candidates with the 3 directions from `layoza design-system --json`; added match_confidence/distinctiveness handling, Layoza mode mapping (A/B/C/D), a worked Bergen smokehouse worked example, and a dev-only contract location. -->

# Deriving and committing the world

## 1. Decide what is already true

Read `DESIGN.md`, `BRAND.md`, `SITE-DNA.json` (if `layoza inspect` has run),
representative code, tokens, components, and assets.

- **Redesign (Mode D):** preserve product truth, content, function,
  constraints, and explicit brand commitments; replace the old visual world
  rather than polishing it.
- **Established world (Mode B/C):** inherit it. A missing `DESIGN.md` does not
  erase a coherent identity already in code; run `layoza inspect . --write`
  and document that identity instead of inventing a replacement.
- **Incomplete brand:** keep confirmed assets and recognisable traits (logo,
  name, a signature colour), then expand the system for this surface.
- **No visual authority (Mode A):** create a new world.

## 2. Ask only what changes the work

Attended runs: ask two or three related questions, never CSS values or canned
aesthetic lanes.

- **Persuade** (landing, marketing, store, pricing): who must act, what they
  should believe, which real proof, content, or assets earn that belief.
- **Operate** (app, dashboard, admin): the task, information, important states,
  frequency, constraints.
- **Read** (docs, articles): the reader's question, source material, wayfinding.
- **Experience** (portfolio, gallery): what leads, how exploration unfolds.

Always: what success looks like, what must stay untouched, and what would make
a polished result feel wrong. Unattended: infer, and list assumptions.

## 3. Name the rut

Write the mechanism, the audience's real scene, its cultural home, and what
this first surface must prove. Then name:

- the **reflex**: the page this category always ships. Start from
  `distinctiveness.category_cliches`; add what you would have built by habit.
- the **obvious opposite**: what a designer reacting against the reflex ships
  (cream craft → black brutalist craft; corporate blue SaaS → neon-on-black SaaS).
- the **name pun**: a picture of the brand *name* or a titled metaphor (a
  bakery called Hearth → a fireplace). At most one candidate.

None of these three may be the build unless the user picks the standard exit.
All three are *treatments*. The subject's own objects (what a visitor sees and
handles in this business) are never part of the rut: the committed world must
show the most characteristic one as art at scale in the first screen.

## 4. Seven candidates

List seven concrete visual systems, artifacts, places, or rituals the audience
knows by heart, ordered by resonance. Each gets two lines:

- **why** it resonates and can carry the mechanism;
- **look**: ground, main colours, type voices, the one signature that makes it recognisable.

Rules:

- **At least two candidates come from the subject's own objects and rituals**
  (the counter, the tool, the product in use). Place and material candidates
  then supply palette, pattern and type *around* the subject; they never
  replace it.
- Start from `place_cues` and `material_cues`. A place cue is a whole visual
  culture: architecture, tilework, textiles, signage, transit graphics,
  packaging, printed ephemera, the colour of its light and soil.
- The audience's world includes its **graphic and screen traditions**:
  notation, publications, identity programmes, data graphics, interfaces it
  reads daily. A nameable abstract system (a school of poster, a documentation
  standard) is as concrete as any artifact.
- Ask: what would this look like as a physical object? What did its world
  look like before the web?
- On Persuade/Experience surfaces, *software* chrome the audience operates (a
  terminal, a control panel) counts toward the rut: at most one candidate.
  Physical tools of the trade (an optician's trial frame, a baker's peel) are
  subject objects, not chrome. On
  Operate/Read surfaces, never take candidates from those tools at all; they
  come back as a costume.
- Near-duplicates count once. If more than three of seven share one material
  family, the derivation stopped at the obvious artifact: dig until the list
  spans at least three families.
- Every candidate must already be viable: true in every claim it visualises, a
  real palette and component family, one product-specific experience,
  buildable at full-page scale within the assets and performance budget. A
  candidate that fails on truth is replaced, never rescued.

### Worked example (Bryggehus, a smokehouse and fish counter in Bergen)

Reflex: navy + white nautical stripes, rope icons, a serif with anchors.
Obvious opposite: sterile white Scandinavian minimalism with one grey sans.
Literal reading: smoke (grey haze, blurred photos). Candidates, roughly by resonance:

1. **Bryggen wharf houses**: ochre, oxblood and white clapboard gables; signature: the gable row.
2. **Rosemaling** folk painting: scrolling acanthus in red, blue-green and mustard on dark wood.
3. Harbour **fish-auction boards**: chalk prices, ruled columns, stencilled crate numbers.
4. **Alder smoke and tarred timber**: near-black browns, resin amber (material, quiet).
5. Coastal **weather and tide charts**: hairline isobars, Beaufort numerals (data tradition).
6. **Stockfish drying racks** (hjell): repeating timber lattice, silver fish against sky.
7. The Saturday **fish-market ritual**: paper wrap, string, the counter itself.

Families span architecture, folk art, signage/data, material, ritual. The
committed world fuses far and near: if the deal lands on the weather and tide
charts (5), the page is set as a coastal chart in its palette and notation, and
the subject (a whole smoked fish on the counter, drawn at scale) is the hero
signature the visitor can turn and slice; Bryggen's gables (1) donate the
section rhythm as a raise. A page that never leaves the reflex has
not been art-directed, and a page whose first screen does not show fish has
lost its subject.

## 5. Fuse with the engine's three directions

Layoza's directions differ in layout composition, visual identity, type
pairing, content treatment, and motion, with calibrated dials. Use them as
**structure**, the candidate as **world**:

1. The **dealt** candidate (`layoza deal "<seven candidates>" --top 4`) = world
   (palette, material, type voice). The subject's circled object stays the
   **signature**, drawn at scale and ideally operated by the visitor
   ("world from far, subject up close").
2. The direction whose composition and dials best carry that world =
   skeleton. Translate its palette and fonts into the world; keep its
   layout, content treatment, density, and motion budget.
3. Fuse each challenger (the other two directions with the world applied, and
   candidates 2–3) before judging it. Judge on **audience identification** and
   **product clarity** only. Verdicts: wins / competitive / declined.
4. A declined challenger donates one discipline the winner lacks; write it as a
   named "raise" line. Discipline (total palette commitment, density courage,
   structural honesty) transfers; clothes (a button style, a display face, a
   lifted motif) do not.
5. The directions' palettes and fonts always come from a generic profile:
   keep only their structure, and take colour and type from the world.

Pinned constraints win field by field. Where only materials conflict with a
pinned brand colour or font, translate the material expression and say so;
a look mismatch is not grounds to change direction. Change direction on your
own only on named factual grounds (the world cannot carry the product's truth
or task); taste is never grounds.

## 6. Presenting (attended runs)

Present one direction, fully committed, raises visible: world, first viewport,
visitor path, signature interaction, reach into the rest of the site, honest
risk. Then up to two full alternates (winning/competitive challengers), declined
challengers as one compact line each with what the winner kept from them, and
one quiet **standard exit**: the category standard played straight.

- Never recommend the standard exit and never soften the main direction toward
  it. If the user takes it, convention becomes the commitment: ask for two or
  three products it should sit alongside and match their craft level, without
  irony or smuggled quirk.
- If the user says "bolder" or "safer" while choosing, re-derive: bolder =
  candidates from further afield at full commitment; safer = your remaining
  conventional grounded candidates. After two re-rolls, ask what quality is missing.
- Unattended: build the main direction and report assumptions.

## 7. Mode rules (Persuade and Experience)

- **Persuade:** the opening makes the offer intelligible and desirable, exposes
  the real action in working form, and demonstrates something only this
  product can prove. Conversion lives inside the world's vocabulary: a hook in
  one line, a visible primary action, a legible reading order. A committed
  world that hides the offer or the action has not finished translating.
- **Experience:** the work itself leads from the first viewport.
- **Coverage, not performance:** the world's material is the ground and the
  major surfaces; its type sets the headlines. A world reduced to an accent on
  a neutral template is the category default with a sticker on it. But only one
  region acts out the concept; the rest stays in the material without performing.
- **Object worlds:** when the world is an object, the page is that one object
  seen once, at one scale, keeping its real form: a book opens to two pages on
  one gutter, never a third panel; a tile wall has one grid, not two. More
  content goes on the object's own surfaces or the next section, never on
  extra copies or impossible folds.
- **Alternates go further, never cleaner:** a bolder use of the same world, a
  larger or stranger focal move, a composition the category would not attempt.

## 8. Colour and type commitments

- Strategy before swatches: Restrained, Committed (30–60% one saturated
  colour), Full palette (3–4 named roles), Drenched. Colour commits at page
  scale: fields own whole regions.
- One sentence of physical scene decides light or dark.
- Faces are objects from the subject's world. The training-default faces (the list behind
  `layoza qa` rule `craft-overused-font`) need a reason no other face satisfies. A bookish, warm, or
  child-facing subject does not soften this: cream paper is the smallest
  corner of a book's world; cloth, thread, jackets, and endpapers span the
  whole saturated spectrum.
- Energy is not the enemy of trust. "No hype" rules out hype devices, not
  exuberance; "calm coaching" describes the product's behaviour, not the
  surface's energy.

## 9. Contract

Write the six blocks from SKILL.md step 5 to `.layoza/direction.md` (or the
project's dev-only notes folder), never into served files. Re-read it before
building and before the finish review; the finish audits the render against
it, especially FIRST VIEWPORT and SIGNATURE.
