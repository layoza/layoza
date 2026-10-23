---
name: layoza-art-direction
description: >-
  Art direction for new websites, apps and redesigns (Mode A/D):
  turn a brief into ONE committed visual world that is unmistakably about its
  subject, build it at studio quality (art, type, colour, layout, motion), and
  iterate on actual screenshots and motion sequences, with
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

Read the offer in the paused opening, not only its slogan. For an agency,
"digital experiences" needs a concrete service noun such as web design or
development in the supporting sentence. Compare the time and space needed to
reach the actual enquiry fields on phone; a second large decorative CTA can
delay the task even when both buttons are clear. In comparisons, repair common
implementation defects in every arm before attributing differences to a skill.

Write .layoza/direction.md with PURPOSE, audience, pinned choices, candidates,
PICKED, reason, SECTIONS, MATERIAL and MOTION (0-10, the same scale as engine dials).
Use PICKED: auto with a reason when making the design decision yourself. Ask for
a pick only when the user requested selection. Never invent user approval words.

## 4. Build the complete idea

Prototype the most uncertain, identity-defining composition first: it may be an
image/type relationship, an interaction, a working product view or a scene.
For a high-motion brief this prototype must demonstrate the signature transformation
on desktop and phone, not merely a static hero or small floating elements.
Capture its first screen at 1280x800 and on a phone before building the rest:
headline, supporting line and first action must show without scrolling.
Then finish every section with the same care. Section order follows the argument
and task; section form follows its content. Familiar structures are allowed when
they are the clearest answer. No prescribed number of sections, pictures or tricks.

Use the native framework and existing primitives. Bind spacing, radii, colours,
type and motion to observed project tokens; unknown extraction is not permission
to restyle the site. The generate command is a scaffold, not the finished design.

Use execution_foundation from the MCP for starter spacing/control primitives and
motion delivery. Use the selected concept's tokens, not automatically Concept 1.
Fit actual heading and button labels at 320/390/768/1440px. Keep heading, support
and action aligned; related items sit closer than separate groups. Reserve a
stage for moving art so it cannot cross copy or actions. Inspect display tracking,
line breaks and navigation/control scale relative to the headline at physical
screen size. Remove competing type voices or ornament that weaken reading order.
Valid tokens alone do not make an oversized headline well composed.

Use the selected direction's `composition_system` to prototype an actual static
copy/type lockup: it contains the brief's quoted copy, editable role CSS and a
candidate identity anchor. Render it with the real artwork and visual_system
controls before motion can disguise a weak reading order. It is a starting
fragment, not a required hero layout. Keep title, useful support and the actual
action in a clear relationship; a stage caption belongs to the stage, with its
own reserved space. Compare full-size paused frames at desktop and phone.

Carry the chosen typographic and material relationship into an ordinary section
and the enquiry state. Inspect those together with the hero: if the only shared
identity is an accent colour and a button radius, develop the composition further.
Expression may come from a deliberate type contrast, notation, a material edge,
proportion or a spatial relationship grounded in the subject. Choose what the
real content supports; do not accumulate all of them or make familiar actions
harder to recognize. Record what changes after inspecting actual static frames,
separately from motion quality.

Use `composition_system.typography_study` to render the selected families with
the real headline, ordinary content and enquiry action before committing. Its
role CSS is editable; do not paste its specimen as the site. Contrast may come
from width, weight, case, italic, spacing or one justified second face. Compare
these as compositions with the actual artwork, rather than choosing a novel font
name. Keep display tracking and transformations off body copy and controls.
Record the repeatable relationship the type creates in the opening, a content
section and enquiry; apply it in those surfaces, then verify the loaded font and
real label fit. Use the paused comparison to decide whether the identity improved.

Treat navigation as one composed component, including utility states. Check its
normal, reduced-motion and open-menu renders: repairing a hidden motion control
with a prominent pill can create a new hierarchy problem. Collapse according to
actual content fit; do not progressively shrink the brand to squeeze in utilities.
The optional [responsive-header controller](assets/responsive-header.mjs) supports
moving one shared utility into the phone disclosure, Escape/focus return, keyboard
containment and cleanup. Adapt its markup/breakpoint and style to the project;
it does not prescribe a palette, wordmark, menu appearance or a universal header.

Check `matched_industry` against the brief's actual trade before using its proof
language. Its type and palette are the category default every build of that trade
starts from; build from a direction's deal instead. Words such as studio, creative and agency do not
identify the business on their own. If routing is wrong, correct the subject and
retrieve again; do not turn architectural work, research or another practice into
an advertising reel. Treat category styling as a candidate and judge it with the
actual artwork, not as an identity already selected for the client.

For palette and CTA decisions, use
[reference/colour-actions.md](reference/colour-actions.md). Read each concept's
visual_system roles and tested control styles. Compare the colours with actual
artwork and action groups; a swatch strip or contrast pass cannot establish
cohesion. Do not add every accent simply because the palette lists it.

The dealt palette and type pair are the identity floor, not a mood board: pick one
concept's set and keep its hue family and faces (lightness may shift for contrast).
Swapping them for a personal favourite is how independent builds for one trade
collapse into the same page. Three blind dental builds all drifted to a warm serif
on cream with deep green and an amber lamp although the deal offered coral, yellow
and sea-glass. If the deal itself is cream, declare it with the
`layoza-brand-allow` meta the brief prints; a cream ground you chose yourself stays
a finding to fix, not to accept.

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

## Motion authority
Motion tools: see [`layoza-animations` § 2](../layoza-animations/SKILL.md). The user's prompt wins; tools are chosen per brief, never mandated.
