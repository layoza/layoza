<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/craft-floor.md. Modified by Layoza: each verify item mapped to the `layoza qa` rule that measures it; impeccable design-hook/detector references removed; added Mode B/C brand-DNA scope for refusals, sticky-header focus, font-swap CLS, and reduced-motion/cleanup requirements from Layoza's AGENTS.md contract. Layoza additions (own wording, ideas only): forms/platform row (vercel-labs/web-interface-guidelines, ibelick/ui-skills, MIT), motion frequency budget (emilkowalski/skills, MIT), template-chrome refusals (anthropics/skills frontend-design, Apache-2.0). -->

# Craft floor

Load after the direction is settled, before the first UI edit. Build without
announcing the checklist. A pinned brief, the committed world, or (in Mode B/C)
the brand's `SITE-DNA.json` overrides anything here; your own habit does not.

## Verify (measured where Layoza can measure it)

Each item is a check on the built result, not an intention. Run them together
in one `layoza qa` pass plus one screenshot review, not as separate trips.

| Check | Floor | Measured by `layoza qa` |
| :--- | :--- | :--- |
| Contrast | Body and placeholder ≥ 4.5:1, large text ≥ 3:1, controls/focus ≥ 3:1. On coloured surfaces tint secondary text from that hue; never washed-out gray. | per-element contrast |
| Overflow | Nothing wider than 375px; no horizontal scroll. | `document-overflow`, `runaway-canvas` |
| Type | Body measure 45–75ch; display max ~6rem; tracking floor −0.04em (−0.02 to −0.03em reads better); balanced headings; multi-line display `line-height ≥ 1.0`. Run real copy at every breakpoint. | `text-clip-or-bleed`, `cramped-leading`, `label-wrap`, `heading-crowding` |
| Spacing | Tight groups, generous separation, more space above a heading than below it. One rhythm. | screenshots |
| Depth | Shadows carry an offset and soft blur; declare elevation once (border **or** shadow). Radius is a scale by role and size (controls tighter than containers), never one value on everything; nested corners are concentric (inner = outer − padding). | `craft-uniform-radius`, `craft-card-kit`, screenshots |
| Motion | Designed per brief (see `layoza-animations` for what and which tool), exponential ease-out from an already-visible default; reach past transform/opacity (clip-path, mask, filter) on small areas or one-off tweens, never a full-viewport scrub. **Budget by frequency:** what a person does many times a visit (menus, tabs, filters, add-to-cart) answers in ≤ 200ms or not at all (dialogs and drawers 200–300ms); only rare moments (first load, a completed booking) may perform. Pop-ups grow from their trigger, start at ~0.95 scale with opacity (never `scale(0)`), and use ease-out. Hover motion sits behind `@media (hover: hover)`. **Micro-motion floor on every page** (`layoza-animations` 2c): hover, press, open/close, add/select, form and live-number feedback, with at least two moments that speak the subject. Every animation honours `prefers-reduced-motion: reduce` (keep fades, drop movement) and cleans up listeners/timelines/GPU objects. | `text-hidden*`, `motion-ignores-reduce`, `craft-scattered-reveal`, `craft-dead-controls`, `transition-all` |
| States | Hover, focus-visible, active, disabled, loading, error, empty, success. Real content, working controls. | `unlabeled-control`, `interactive-overlap` |
| Keyboard | Visible `:focus-visible` ring from the palette; focus never hidden under a sticky header (`scroll-padding-top`). | `focus-obscured` |
| Touch | Targets ≥ 24×24 CSS px (44 preferred), separated. | `target-size` |
| Navigation | Mobile menu reachable, opens, closes, traps nothing. Anything clickable is a real `<a href>` or `<button>`. | `nav-unreachable`, `pointer-not-button` + `*-menu-open.png` |
| Forms & platform | Fields ≥ 16px on phones (iOS zooms otherwise); never lock pinch zoom; `autocomplete` tokens plus the right `type`/`inputmode` on personal data; `color-scheme: dark` on dark pages; `width`/`height` or `aspect-ratio` on every image; list the properties you transition (no `transition: all`); `text-wrap: balance` on headings; `dvh` for full-height sections. | `ios-input-zoom`, `zoom-blocked`, `field-autocomplete`, `dark-color-scheme`, `image-no-size`, `transition-all` |
| Performance (optional) | Metric-adjusted fallback `@font-face` for every web font. LCP/INP/CLS only on request; never blocks design. | `--performance` (null = unmeasured) |
| Browser surfaces | Theme what you did not draw: `::selection`, `caret-color`, scrollbars, focus rings, `text-underline-offset`, `font-variant-numeric: tabular-nums` in data. Cheapest signal a page was built, not assembled. | screenshots |
| Copy | The product's own language. Controls name their outcome and keep that name through the flow ("Book the slot" → "Slot booked"); errors name the problem and the recovery without apologising; empty states invite one next action. | `craft-vague-cta`, functional test |
| Coverage | Every brief requirement present and findable within seconds. | functional test |

## Reflex checks (new work)

The absolute refusals are in SKILL.md. Everything below is a **reflex check**:
these are the habits generated pages fall into. Each one is allowed when you can
state in one line why *this* world needs it (write the line in
`direction.md`); otherwise, rewrite the element instead of softening it. The
user's prompt and brand DNA override all of them.

Page scaffolds:

- Same-size cards of icon + heading + text as the page structure. Cards are the
  lazy container; nested cards are always wrong.
- The hero-metric template: big number, small label, supporting stats, accent.
- A kicker or eyebrow above a heading. Delete it and let the heading speak.
  (Layoza scope: banned in new work; keep it if `SITE-DNA.json` shows it is the brand's pattern.)
- Section numbers (01 / 02 / 03) unless the sequence carries information.
- A modal for a task that needs neither interruption nor protected focus.

Surface habits:

- Gradient text. Emphasis comes from weight or size.
- Glass and blur as decoration rather than a specific effect.
- A coloured `border-left`/`border-right` above 1px on cards, list items,
  callouts, or alerts.
- Hard offset shadows (`box-shadow: 4px 4px 0`) outside a world that is
  actually neobrutalist.
- Sparklines, progress rings, and soft-shadowed rounded rectangles standing in
  for content.
- Monospace as a costume for "technical" rather than for code, data, or measurement.
- A system display face (Impact, Arial Black, the platform sans, Inter) as the
  display voice of an own-world page. Self-host a face with the right character.
- Unicode glyphs or emoji standing in for icons. Icons come from one real
  library or authored SVG, one stroke and weight.
- Geometric masks (circle, polygon, radial cutout) approximating an organic edge.
- Illustration below the fidelity bar in
  [art-production.md](art-production.md): primitive piles, `doodle` sketches,
  `feTurbulence` grain standing in for real texture. Drawn SVG art that meets the
  bar (correct construction, layered and lit, one line system) is first-class.
- Backgrounds textured only from the subject's world. `repeating-linear-gradient`
  stripes and grid overlays need a real map, blueprint, or tool under them.
- Light or dark picked by category. Pick it from the use scene.
- Unexamined `#000` / `#fff` / neutral gray. Tint from the world unless the
  world (or the brand DNA) really is pure black and white.
- Claims and configuration come from supplied truth; label illustrative values honestly.

Template chrome (appears whatever the subject, so it signals nobody decided):

- An arrow glyph appended to button and link labels (`Get started →`). Keep arrows
  for real direction: next/previous, and `↗` for links that leave the site.
- Meta strings joined with middle dots (`Ann · 5 min · Design`); `WORD — fragment` labels.
- One word of a headline set in another colour, italic, or face. Make the whole
  headline the type moment, or let the words carry the stress.
- Identical shadowed rounded cards as the structure of section after section.
- The same fade-and-slide entrance on every section.
- Centred paragraphs of four or more lines; four or more typefaces.
- Near-black (`#0b0b0b`, `#111`) with one acid-green or vermilion accent as the
  default "bold" look.

`layoza qa` reports each as a `CRAFT-*` note (non-blocking).

The floor holds the mechanics; it never picks the direction. With every check
green, spend the page on the committed world, and when torn between refined and
committed, commit.
