<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/new-work.md (section 7), reference/polish.md and reference/critique.md. Modified by Layoza: the finish gate is `layoza qa` (measured Chromium at 1440/768/375) instead of the impeccable detector, comp-diff, and build-phase state machine; critique condensed to a design-specificity verdict, Nielsen scoring with n/a renormalisation, P0-P3 severities, and persona walk-throughs; snapshot persistence and impeccable sub-agent names removed; added anti-gaming rules. -->

# Finish: inspect the actual design

Follow [the canonical process](../../layoza/references/creative-process.md).
Three review rounds is a default budget, not a quality certificate.

## Evidence

Capture desktop and phone first screens, the complete page, important section
crops, open navigation and relevant UI/motion states. Let assets and deliberate
entrances settle. A blank or wrong capture is not evidence. Inspect images at a
size where type, crops and relationships can be judged.

## Design review

For each candidate and the final page, write concrete observations about:
1. Brief fidelity: subject, task, audience, facts and pinned aesthetic choices.
2. Hierarchy: dominant mass, reading order, action and secondary detail.
3. Typography: character, proportions, scale relationships, line breaks and measure.
4. Imagery: real relevance, quality, focal point, crop, integration and credit.
5. Colour and material: coherent roles, contrast, intentional depth or flatness.
6. Rhythm: sections that progress the argument and a carefully finished footer.
7. Phone composition: redesigned relationships, readable content and reachable action.
8. Interaction: meaningful states, motion continuity, keyboard behavior and feedback.

Describe the two weakest decisions with screenshot/section references and a
specific change. A numeric rubric can summarize judgment; it cannot prove taste.
Style tells such as eyebrows, borders, gradients or monospace are contextual, not
automatic defects. Ask whether the device earns its place.

Use a fresh critic when available. Give them only the brief and images, retain
their observations, and record whether they were independent. Fix the highest
impact weakness, recapture the changed compositions and check the whole page.
Do not keep changing an already coherent page merely to increase a self-score.

## Retain the decision

Call review_design with the original brief, chosen_direction, alternatives,
reason, strengths, weaknesses, changes, rubric and actual screenshot paths.
Assets stay in the design_assets ledger. The record fingerprints the images and
source; it does not claim that a human or independent judge approved them.
For an automatic pick, log PICKED: auto and the reason; do not fake user quotes.

## Usability floor

Run layoza qa / capture_screenshots for actual overflow, clipping, readability,
missing images, hidden text, keyboard/focus, menu and responsive problems.
Check task flows and the full app state matrix where applicable. Keep no-JS and
reduced-motion content meaningful. Investigate warnings in context.

Speed diagnostics are optional and never a design gate or a rollback trigger.
If a real visual or interaction defect remains, name it. Show the final renders
and the remaining limitations; do not claim the page beats every rival.
