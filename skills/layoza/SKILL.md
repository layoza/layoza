---
name: layoza
description: >-
  Primary entry point for Layoza, the AI-native UI/UX design, motion, Site DNA
  and frontend verification system. Activate first whenever the user asks to
  create a website, landing page, app or dashboard, add to or improve an
  existing site, redesign a frontend, build animations, build WordPress block
  themes, or run browser/accessibility/performance QA.
---

# Layoza: Design Orchestrator

## 0. Prime directive

1. **The user's prompt always wins.** Follow a named style, library, scope,
   "minimal" or "no animation" exactly. The defaults below apply only where the
   prompt leaves the choice open.
2. **Design first.** Act as the best designer in the world on this job. The page
   must be unmistakably about its subject and better than what a top studio
   would ship. QA is the floor, never the goal; a QA-clean page that is plain
   or off-subject has failed.
3. **Nothing is hard-coded.** Art, type, motion, 3D and the tools behind them
   (CSS, WAAPI, GSAP, Motion, WebGL, SVG) are chosen per brief.
4. **Never cut ambition to clear a warning.** Fix the cause and keep the idea.
   Accessibility and reduced motion are always kept.

## 1. Route by mode (load only what the mode needs)

**Intake before routing.** Know what the site is for (purpose and primary
action), who it is for, and whether it is new or an existing site/repo; ask the
user for whatever the prompt leaves open. An existing project in the folder is
never Mode A. For new sites, the user picks one of three directions before any
build (`layoza-art-direction` step 3b).

| Mode | When | First command | Load |
| :--- | :--- | :--- | :--- |
| **A: New** | New site, page, app or dashboard | `layoza design-system "<brief>" --mode new --json` | [`layoza-art-direction`](../layoza-art-direction/SKILL.md) (+ [`layoza-new-website`](../layoza-new-website/SKILL.md) for stack) |
| **B: Extend** | Add a page, section or component to an existing site | `layoza inspect . --write` | [`layoza-extend-website`](../layoza-extend-website/SKILL.md) + [`layoza-site-dna`](../layoza-site-dna/SKILL.md) |
| **C: Improve** | Audit or fix an existing site | `layoza baseline create pre-edit`, then `layoza audit .` | [`layoza-discovery-audit`](../layoza-discovery-audit/SKILL.md) + [`layoza-refine`](../layoza-refine/SKILL.md) |
| **D: Redesign** | Replace an existing site's look | `layoza baseline create pre-redesign` | [`layoza-redesign`](../layoza-redesign/SKILL.md) + [`layoza-art-direction`](../layoza-art-direction/SKILL.md) |

**Surface before world.** If the brief is an app, dashboard, admin, settings
screen, internal tool or mobile app UI (the visitor finishes a task rather than
being persuaded), load [`layoza-product-ui`](../layoza-product-ui/SKILL.md)
instead of `layoza-art-direction`'s world-building, in every mode. The brand
still lends type, palette, density and one signature move. A product with a
marketing home uses art direction for the home and product UI for the app.

On demand:
- [`layoza-animations`](../layoza-animations/SKILL.md): motion, deciding what and which tool;
- [`layoza-threejs-webgl`](../layoza-threejs-webgl/SKILL.md): 3D;
- [`layoza-product-ui`](../layoza-product-ui/SKILL.md): apps, dashboards, tables, forms, data viz;
- [`layoza-components`](../layoza-components/SKILL.md): framework output;
- [`layoza-wordpress`](../layoza-wordpress/SKILL.md): WordPress;
- [`layoza-design-system`](../layoza-design-system/SKILL.md): `DESIGN.md`;
- [`layoza-figma`](../layoza-figma/SKILL.md): Figma;
- [`layoza-accessibility`](../layoza-accessibility/SKILL.md): accessibility;
- [`layoza-performance`](../layoza-performance/SKILL.md): performance;
- [`layoza-visual-qa`](../layoza-visual-qa/SKILL.md): QA rules.

## 2. Design guardrails

1. **Subject at scale:** the first screen shows the trade's most characteristic
   object as authored art. A stranger can name the business with the words covered.
2. **World from far, subject up close:** the world comes from a dealt candidate
   (`layoza deal`); the subject is its signature.
3. Colour owns sections (a quiet or minimal prompt: one ground, one accent),
   type can be the image, each section's composition comes from its content,
   and the idea reaches the footer.
4. Mechanics (overflow, contrast, focus, leading) are enforced by `layoza qa`.

## 2b. Win rules (every niche)

Read [`references/what-wins.md`](references/what-wins.md) before building and tick it before
shipping: brief fidelity, main action in reach at 375px, no voids, colour restraint,
real-feeling data, phone tables, the trade's artefacts as UI, the `micro-css-polish`
motion floor on every page (even static ones), and 0 errors + 0 warnings on the final file.
Deal hero and section mechanics with `layoza mechanics deal "<brief>"` (MCP `deal_mechanics`); see
[`references/mechanics.md`](references/mechanics.md).

## 3. Done means

```bash
layoza qa <html-file-or-url>
```
Real Chromium at 1440/768/375, scrolled like a visitor: contrast, overflow,
clipping, nav, focus, hidden text and lab Core Web Vitals (`null` = unmeasured,
never a pass). Then open `qa/desktop.png`, `qa/mobile-hero.png` and
`qa/mobile-menu-open.png` and run the art loop
(`layoza-art-direction/reference/finish.md`). A page is done when
`.layoza/direction.md` records `PURPOSE:`, the user's `PICKED:` direction, 3+
`ROUND` lines and a `REVIEWER:` verdict (the audit enforces this), its design
scores ≥ 9 (fresh-eyes reviewer agrees), QA of the **final** file shows 0 errors
and 0 warnings, and every `what-wins.md` item is ticked.

**Apps and dashboards** are done differently: every view renders its state
matrix (empty, loading, partial, error, offline, permission denied, sparse,
dense), `layoza qa` shows 0 errors and 0 warnings, `what-wins.md` items 1, 5, 6, 8 and 9
are ticked, tables work at 375px and the main task
passes a keyboard-only walk-through (`layoza-product-ui` §14). No art-loop
score is required.
