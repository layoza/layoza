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

# Layoza: Art direction through actual composition

Create a page with a deliberate point of view that belongs to this client and
helps its audience. Exceptional execution matters more than adding effects,
obeying a template or removing familiar stylistic devices.

## Authority

The user's explicit choices win. Mode B/C preserves the actual incumbent system;
Mode D replaces the visual world while preserving required content and behavior.
Follow [the canonical creative process](../layoza/references/creative-process.md)
for build order, selection and iteration. This overrides older example workflows.

## 1. Establish the brief's truth

Read the project, real copy, routes, components and supplied images. Record:
subject; audience and usage scene; primary task; factual claims; required content;
explicit visual commitments; forbidden treatments; available material; unknowns.
Never infer a client's real premises, staff, testimonials or product screenshots
from stock material. Ask only about an uncertainty that blocks truthful work.

Call get_design_system with the original request and a brief_context containing
the actual subject, audience, primary_action, objects, facts, required_content,
forbidden, display_font, body_font, colors and assets where known. Check the
understood subject and pinned choices before accepting any suggestion.

## 2. Secure the visual material

Call design_assets inventory. Supplied brand and product material comes first.
Search or import a verified asset with scripts/design-assets.mjs, generate an
image when a suitable tool is available, or author a deliberate illustration.
Register each chosen asset with design_assets: local path, source, licence,
creator/credit, role, alt, and intended crop. A filename is not a rights check.

Choose a concept that the available medium can actually deliver. Photographic
brands need convincing photographs. Product tooling is strongest when it shows
the product working. Typography may lead when the content and concept justify it.
See [art production](reference/art-production.md) for fidelity techniques, not
mandatory object drawings or a quota of pictures.

## 3. Compare compositions before committing

The engine supplies three starting compositions, not finished creative answers.
Develop them into three small, genuinely different rendered first screens:
- different reading order and massing;
- a distinct typographic idea, respecting pinned families;
- a different use of the real images/content;
- a plausible whole-page rhythm, not just a recolour.

Use identical client facts and assets across candidates. Do not improve only your
preferred arm. Render desktop and phone versions. Pick on audience/task fit,
expressive strength, clarity and coherent execution. Novelty is a supporting
signal; it never forces a worse nav or a random metaphor.

Write .layoza/direction.md with PURPOSE, audience, pinned choices, candidates,
PICKED, reason, SECTIONS, MATERIAL and MOTION (0-10, the same scale as engine dials).
Use PICKED: auto with a reason when making the design decision yourself. Ask for
a pick only when the user requested selection. Never invent user approval words.

## 4. Build the complete idea

Prototype the most uncertain, identity-defining composition first: it may be an
image/type relationship, an interaction, a working product view or a scene.
Then finish every section with the same care. Section order follows the argument
and task; section form follows its content. Familiar structures are allowed when
they are the clearest answer. No prescribed number of sections, pictures or tricks.

Use the native framework and existing primitives. Bind spacing, radii, colours,
type and motion to observed project tokens; unknown extraction is not permission
to restyle the site. The generate command is a scaffold, not the finished design.

## 5. Judge what is actually visible

Use [reference/finish.md](reference/finish.md). Look at first screens and section
crops at desktop and phone scale, dense/empty app states, open menus and any
important motion states. Describe visible strengths and weaknesses, not just a score.
When available, use a fresh subagent critic with only the brief and rendered images,
not your rationale; retain their findings and your response. Never present self-review
as independent evidence.

Fix the weakest important decision, recapture and compare. Default budget: three
visual review rounds; continue when a material issue remains and scope allows.
A round cap means report an unfinished limitation, not declare excellence.
Persist candidates, final screenshots, reasoning and changes with review_design.

## 6. Usability floor, then handoff

Run capture_screenshots or layoza qa; fix real overflow, hidden content, unreadable
text, broken controls, focus and responsive defects. Keep reduced-motion fallbacks
and cleanup. Speed measurements are optional. Do not lower artistic ambition just
to make a checker quiet; contextual critique warnings require visual judgment.

Show the actual final designs, what changed during review, and the concrete
limits of the evaluation. A clean checker does not establish that we beat rivals.
