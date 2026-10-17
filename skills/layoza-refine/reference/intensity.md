<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/quieter.md, reference/distill.md and reference/delight.md. Modified by Layoza: three commands condensed into one intensity guide in Layoza's own words; each mapped to the `layoza qa` craft rules it should clear and the metrics it must not break (contrast after desaturation, flat hierarchy after quieting, nav reachability after cutting); per-surface meaning taken from Layoza's surfaces.md; SITE-DNA.json brand authority; a removal ledger and delight cost budget added; impeccable hand-offs and question tooling removed. -->

# Intensity: quieter, distill, delight

All three change *how much* the page says, not *what brand* it is. Run the
measured loop in [SKILL.md](../SKILL.md) around each. `bolder` (the opposite of
quieter) lives in [system-passes.md](../../layoza-art-direction/reference/system-passes.md);
never run bolder and quieter on the same target in one session.

---

## Quieter (too loud, too busy, tiring)

**Meaning by surface.** Persuade/Experience: more air, a narrower palette,
less drama, the point of view intact. Operate/Read: less noise so the tool or
text disappears into the task (flatter surfaces, fewer accents, less motion).

**Find the sources first** (name them with selectors before editing):
saturation, too many high-contrast juxtapositions, too many heavy weights
competing, decorative glows/halos/gradients, motion that never rests, and
"everything is big" (no scale steps). `layoza qa` usually already names them:
`craft-dark-glow`, `craft-radial-halo`, `craft-radial-spotlight-glow`,
`craft-gradient-text`, `craft-pulsing-dot`, `craft-marquee`,
`craft-gpt-thin-border-wide-shadow`, `craft-ai-color-palette`.

**Moves, cheapest first:**
1. Remove decoration that carries no information (glows, stacked shadows,
   texture overlays). Removal beats toning down.
2. Reduce the number of things competing: keep one or two anchors at full
   strength, step everything else down a weight (700 → 500) or a size step.
3. Lower chroma on large areas; keep the brand's action colour at full
   strength *on the action only*. Tint neutrals toward the brand hue instead
   of pure grey.
4. Shorten motion travel and duration; delete loops that convey nothing.
5. Even out arbitrary spacing jumps onto the existing scale in `SITE-DNA.json`.

**Guards Layoza measures (upstream only warned):**
- Desaturating and lightening text is the classic way to *create* contrast
  failures. `contrast_failures` must not rise; `min_contrast_ratio` stays ≥ 4.5.
- Quieting the headline is the classic way to create a flat page.
  `craft-flat-type-hierarchy` must not appear and `h1_to_body_ratio` must not
  fall below the brand's own baseline value.
- Recorded brand colours are not "too loud" by your taste. If the brand's
  primary is saturated, quiet its *dosage*, not its value.

**Verify:** the page is calmer in the screenshots at all three widths, still
recognisably this brand, the primary action still the most obvious thing.

---

## Distill (cluttered, too many choices)

**Find the one job.** From `PRODUCT.md`: what is the visitor's single primary
action on this target? Everything else is supporting, secondary, or removable.

**Inventory before cutting** (write it down; this is the removal ledger):

| Element | Job it does | Said elsewhere? | Decision |
| :--- | :--- | :--- | :--- |
| e.g. second CTA in hero | duplicates nav "Book" | yes | remove |

Decisions: keep, merge, move behind disclosure (`<details>`, a tab, a
second step), or remove. **Never remove** a route, a form field the backend
requires, legal text, or a nav item without the user's yes. Put the ledger in
the report so every removal can be reversed by a human.

**Typical cuts:** cards inside cards (`craft-nested-cards`); icon-tile grids
that restate the heading (`craft-icon-tile-stack`); eyebrow chips and kickers
above every heading (`craft-hero-eyebrow-chip`, `craft-kicker-above-heading`);
repeated copy (`craft-repeated-container-text`); more than one equal-weight CTA
per decision; extra font families (`font_family_count`); intro paragraphs that
restate their heading.

**Guards:** `nav-unreachable` and `menu-toggle-broken` must not appear (cutting
nav is the usual way to break mobile); `primary-action-below-fold` should clear,
not appear; no accessibility labels removed with the clutter.

**Verify:** the primary action is reachable in fewer steps or less scroll; the
ledger has no unexplained removals; nothing a visitor needs to decide is gone.

---

## Delight (correct, but forgettable)

Delight is the product's character showing up **at a moment that earns it**,
not whimsy sprinkled on every click.

**Pick the moment** (one per session): a meaningful success (booking made,
order placed), a wait that can be informative, a first-use or empty state, an
error the visitor needs to recover from, or a discovery that reveals real
utility. Routine actions get certainty, not celebration.

**Write a one-line delight thesis** before code: the emotion you are aiming for
and why it is native to *this* product (from `PRODUCT.md` voice and the
world in `SITE-DNA.json` / `BRAND.md`). If a neighbouring product could use the
same flourish unchanged, it is not delight, it is decoration.

**Cost budget (Layoza addition):** the moment may add no new dependency, at
most a few KB of CSS/JS, nothing to the LCP path, and no new font. Measured
guards: `lcp_ms`, `inp_ms` ≤ 200, `cls`, `text-hidden-after-scroll` = 0,
`reduced_motion_respected` stays true. Sound only on explicit user opt-in.

**Never:** delay or block the task, fake progress, joke about money, privacy,
deletion, or lost work, add claims the product cannot back, or make the moment
unskippable. It must still feel good on the hundredth repetition.

**Verify:** the task is just as fast with the flourish; keyboard, touch, muted,
and reduced-motion paths work; the moment is visible in a screenshot or a
described interaction you actually ran.
