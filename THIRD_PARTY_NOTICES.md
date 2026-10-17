# Third-Party Notices & License Attributions

**Layoza** is open-source software licensed under the **MIT License** (with Apache-2.0 compatibility).

This file preserves required copyright notices, license texts, and attributions for permissively licensed upstream projects whose specifications, taxonomies, or architectural concepts informed or were adapted into Layoza, as well as explicit legal boundaries for restricted third-party repositories.

---

## 1. Permissively Licensed Upstream Attributions

### A. UI UX Pro Max (`nextlevelbuilder/ui-ux-pro-max-skill`)
- **Upstream URL**: https://github.com/nextlevelbuilder/ui-ux-pro-max-skill
- **Inspected Commit**: `7246805992a42b7bba37d4cac83b01416f0c6c44`
- **Usage in Layoza**: Industry reasoning categories, UI style taxonomy concepts, typography pairing metadata, and pre-delivery UX checklist concepts adapted into Layoza's native Rust Design Intelligence Engine (`crates/layoza-core/src/intelligence.rs`).
- **License**: MIT License

```text
MIT License

Copyright (c) 2024 Next Level Builder

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

### B. Impeccable (`pbakaus/impeccable`) & Platform Design Skills (`ehmo/platform-design-skills`)
- **Upstream URL**: https://github.com/pbakaus/impeccable
- **Inspected Commit**: `2f98774f86bc05563876643db37e6c37c322850f` (engine v0.1.14, skill v4.5.2)
- **License**: Apache License 2.0 (`Copyright 2025 Paul Bakaus`). A full copy of the license text and upstream `NOTICE.md` is kept in every Layoza skill folder that contains adapted impeccable material: `skills/layoza-art-direction/`, `skills/layoza-refine/`, and `skills/layoza-discovery-audit/` (each has `LICENSE` + `NOTICE.md`).
- **Usage in Layoza**:
  1. **No vendored copy** — Layoza does not ship impeccable's skill. A full vendored copy was removed on 2026-10-14; only the ideas Layoza needs are adapted (below), each improved with Layoza's measured verification.
  2. **Adapted files (modified by Layoza)** — `skills/layoza-art-direction/` adapts impeccable's art-direction method; `skills/layoza-refine/` and `skills/layoza-discovery-audit/reference/product-context.md` adapt its refinement commands and product-context step. Each adapted file starts with a header naming its upstream source file(s) and what Layoza changed:
     - `SKILL.md` — adapted from `reference/new-work.md`, `reference/mode-persuade.md`, `reference/craft-floor.md`. Rewritten around Layoza inputs (`layoza design-system --json`: `match_confidence`, `distinctiveness`, 3 directions), Mode B/C brand preservation (`SITE-DNA.json` wins over bans), and `layoza qa` measured verification; impeccable's engine commands (concept-seed, serve-question, build-phase, comp-spec, detect) removed.
     - `reference/world.md` — adapted from `reference/new-work.md` (sections 1-5) and `reference/mode-persuade.md`.
     - `reference/craft-floor.md` — adapted from `reference/craft-floor.md`, with checks mapped to `layoza qa` rules.
     - `reference/system-passes.md` — adapted from `reference/bolder.md`, `reference/typeset.md`, `reference/colorize.md`, `reference/layout.md`, `reference/animate.md`.
     - `reference/finish.md` — adapted from `reference/new-work.md` (section 7), `reference/polish.md`, `reference/critique.md`.
     - `reference/engineering-lessons.md` — original Layoza content (no upstream text); benchmark write-up moved to `docs/benchmarks/kiln-clay-2026-10-14.md`.
     - `reference/surfaces.md` — adapted from `reference/mode-operate.md`, `reference/mode-read.md`, `reference/operate.md` (and the one-line Experience mode in `SKILL.md`). Comp/concept-seed rounds removed; each surface (Operate, Read, Experience, Persuade) mapped to Layoza modes and to the `layoza qa` rules and budgets that matter on it; Experience mode written out in full.
     - `skills/layoza-refine/` — targeted refinement commands for existing pages:
       - `SKILL.md` — command table and shared measured loop; command ideas from `reference/quieter.md`, `distill.md`, `delight.md`, `clarify.md`, `onboard.md`, `harden.md`, `adapt.md`, `optimize.md`, `audit.md` (bolder/typeset/colorize/layout/animate/polish/critique link to `layoza-art-direction` instead of being repeated). Every command runs `layoza baseline create` → `layoza qa` before → change → `layoza qa` after → regression check (`compare_before_after` over MCP) → rollback on a measured regression, and names the `layoza qa` rules that prove it and the ones it must not break; `SITE-DNA.json` / `DESIGN.md` brand tokens win over taste bans.
       - `reference/intensity.md` — adapted from `reference/quieter.md`, `reference/distill.md`, `reference/delight.md`; adds measured guards (contrast after desaturation, flat hierarchy after quieting, nav reachability after cutting), a removal ledger, and a delight cost budget.
       - `reference/copy-and-flow.md` — adapted from `reference/clarify.md`, `reference/onboard.md`; adds a before/after string evidence table, a no-invented-facts rule tied to `PRODUCT.md`, and a measured first-value walk.
       - `reference/resilience.md` — adapted from `reference/harden.md`, `reference/adapt.md`, `reference/optimize.md`; adds a stress-fixture copy run through `layoza qa`, the exact viewports `layoza qa` measures (and the ones it does not), and a two-run lab-metric noise rule with lab vs field separation.
       - `reference/evaluate.md` — adapted from `reference/audit.md` (routing idea of `reference/critique.md`); the 0-4 dimension scores are capped by measured `layoza audit` / `layoza qa` failures.
     - `skills/layoza-discovery-audit/reference/product-context.md` — adapted from `reference/init.md`; `PRODUCT.md` is pre-filled from `layoza inspect --json` / `layoza design-system --json`, every field carries an observed / inferred / confirmed tier, the user is asked only for gaps, and a Brand authority section records which source wins per Layoza mode. impeccable context/live-mode/config plumbing removed.
  3. **Concepts (no text copied)** — design refinement verb taxonomy (`audit`, `critique`, `polish`, `distill`, `harden`, `animate`, `typeset`, `colorize`, `adapt`, `optimize`), separation of durable product context (`PRODUCT.md` / `BRAND.md`) from visual tokens (`DESIGN.md`), and deterministic static/DOM anti-pattern detector concepts, re-implemented in Layoza's own Rust verifier (`crates/layoza-core/src/verifier.rs`).

```text
Copyright 2025 Paul Bakaus

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

**Preserved upstream `NOTICE.md` from `pbakaus/impeccable` (verbatim)**:
```text
# Third-Party Notices

This project includes content derived from third-party work, used under the terms of its original license.

## Platform Design Skills

The `skill/reference/ios.md` and `skill/reference/android.md` platform reference files are distilled from ehmo's `platform-design-skills` (Apple Human Interface Guidelines and Material Design 3 rules), rewritten in Impeccable's voice.

**Original work:** https://github.com/ehmo/platform-design-skills
**Original license:** MIT
**Author:** ehmo
```

---

### C. Taste Skill (`Leonxlnx/taste-skill`)
- **Upstream URL**: https://github.com/Leonxlnx/taste-skill
- **Inspected Commit**: `c924fad1a79c7d70a04bcc8c415b593e9928ad8a`
- **Usage in Layoza**: Adjustable 3-axis design property dials (`Design Variance`, `Motion Intensity`, `Visual Density`), 3-direction creative concept generation workflow, and anti-truncation completeness rules adapted in `crates/layoza-core/src/intelligence.rs`. Its `research.md` / `layout.md` model-panel figures (display type ~8.4vw vs 5.0vw, few sizes) are cited, in Layoza's own words, in `skills/layoza-art-direction/reference/ceiling.md` and the `craft-display-scale` QA threshold.
- **License**: MIT License

```text
MIT License

Copyright (c) 2026 Leonxlnx

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```

---

### D. Google `DESIGN.md` Specification (`google-labs-code/design.md`) & Stitch Skills (`google-labs-code/stitch-skills`)
- **Upstream URLs**:
  - https://github.com/google-labs-code/design.md (`9bf8eae67128b6cc55ad9bf86665767deb4c11cd`)
  - https://github.com/google-labs-code/stitch-skills (`0337446dadde6f8c94210444e2aa9d546126480f`)
- **Usage in Layoza**: Compatibility with the Google `DESIGN.md` YAML frontmatter token schema (`colors`, `typography`, `rounded`, `spacing`, `components`, `{path.to.token}` references) and canonical 8-section Markdown prose structure (`Overview`, `Colors`, `Typography`, `Layout`, `Elevation & Depth`, `Shapes`, `Components`, `Do's and Don'ts`), implemented in Rust in `crates/layoza-core/src/design_md.rs`.
- **License**: Apache License 2.0 (`Copyright Google LLC`)

---

### E. Awesome Design Skills (`bergside/awesome-design-skills`)
- **Upstream URL**: https://github.com/bergside/awesome-design-skills (`f631a09b4fcc0166f2e2c1a8c81906ef680c57e8`)
- **Usage in Layoza**: Style family taxonomy and paired `SKILL.md` + `DESIGN.md` reference patterns.
- **License**: MIT License

---

### F. gstack (`garrytan/gstack`)
- **Upstream URL**: https://github.com/garrytan/gstack (`953a62a72412ba2ec430972226b7b3a999839df5`)
- **Usage in Layoza**: Multi-direction "design shotgun" exploration and slop-scan heuristic categories adapted into Layoza's Mode A & Mode C workflows.
- **License**: MIT License (`Copyright (c) 2026 Garry Tan`)

---

### G. Magic UI (`magicuidesign/magicui`) & Motion Primitives (`ibelick/motion-primitives`)
- **Upstream URLs**:
  - https://github.com/magicuidesign/magicui (`85f97fd27b053339af74ba9dd4ca428ee25989fb`)
  - https://github.com/ibelick/motion-primitives (`120f64f6ca60348e251f929e9c81f11ccbe45eda`)
- **Usage in Layoza**: Component pattern taxonomy and spring animation metadata.
- **License**: MIT License (`Copyright (c) Magic UI`, `Copyright (c) Julien Thibeaut / ibelick`)

---

### I. Design-guidance ideas adapted on 2026-10-15 (no code or text copied)
Layoza re-implemented these ideas in its own words and its own code. No upstream file is vendored.
- **Anthropic Skills (`anthropics/skills`: frontend-design, web-artifacts-builder, theme-factory, canvas-design)**. Apache License 2.0 (Copyright Anthropic, PBC). Ideas used: the generic-default "calibration" traits, which became Layoza-original detectors in `scripts/browser-qa.mjs` (`layozaGenericTells`: `craft-arrow-cta`, `craft-middot-meta`, `craft-caps-dash-label`, `craft-headline-accent-word`, `craft-dark-acid-accent`, `craft-uniform-radius`, `craft-card-kit`, `craft-mono-meta-labels`, `craft-scattered-reveal`, `craft-vague-cta`, `craft-centered-prose`, `craft-font-family-sprawl`, `craft-hero-metric`); also the TOKENS block, ASCII wireframes, sibling test and "remove one thing" pass in `skills/layoza-art-direction/`. Anthropic brand colours and fonts are used only as detector references, never as output.
- **Vercel Web Interface Guidelines (`vercel-labs/web-interface-guidelines`)** and **`ibelick/ui-skills`**. MIT. Ideas behind `layozaInterfaceChecks` (`zoom-blocked`, `ios-input-zoom`, `dark-color-scheme`, `image-no-size`, `pointer-not-button`, `field-autocomplete`, `transition-all`) and the craft-floor "Forms & platform" row.
- **pbakaus/impeccable, second adaptation (2026-10-15)**. Apache-2.0. `skills/layoza-art-direction/SKILL.md` and `reference/finish.md` now adapt more of `reference/new-work.md`: calibration and "rendition prior", "build the assigned direction", "build the form's web leverage", and the challenger "raise". They also adapt `reference/visualize.md` ("commitment is depth") and the finish-reviewer's TYPE / MATERIAL / GROUND / ceiling checks. Each file's header comment names the sources and Layoza's changes. Layoza's `layoza deal` command is an original offline replacement for impeccable's concept-seed roll.
- **`Leonxlnx/taste-skill`**. MIT. Ideas, in Layoza's own words: subject inventory, a section argument, the idea sentence, writing the copy deck first, type and space value ranges, and a scored screenshot critique with caps. Second pass (commit f624f37, 2026-10-15): substance over removal (thin pages lose), headlines two lines or fewer at every width, the idea reaching pricing/FAQ/footer with a designed footer close, colour dosage ranges (ground 70–90%, accent under 5%, fields differing in lightness), display about 4× body with authored line breaks and optical capital alignment, counted anchor lines, extra critique caps and a fixed critique write-up shape. Not adopted: mandatory motion quotas, "drawings do not count" picture caps, blanket bans and stock-photo filler.
- **`greensock/gsap-skills`** and GSAP docs. MIT. Motion value ranges and SplitText/ScrollTrigger usage notes in `skills/layoza-animations`.
- **`emilkowalski/skills`**. MIT. Idea: the motion budget scales with how often an action happens (craft-floor Motion row).
- **`Dammyjay93/interface-design`**. MIT. Idea: keep a persisted log of directions already tried (`.layoza/directions-tried.md`).
- **`raunofreiberg/interfaces`**. No license (all rights reserved). Read for reference only; nothing adapted.

### H. Playwright MCP (`microsoft/playwright-mcp`), axe-core (`dequelabs/axe-core`), and Lighthouse (`GoogleChrome/lighthouse`)
- **Upstream URLs**:
  - https://github.com/microsoft/playwright-mcp (Apache-2.0, `Copyright Microsoft Corporation`)
  - https://github.com/dequelabs/axe-core (MPL-2.0, `Copyright Deque Systems, Inc.`)
  - https://github.com/GoogleChrome/lighthouse (Apache-2.0, `Copyright Google Inc.`)
- **Usage in Layoza**: Used via external integration in Layoza's browser QA runner (`scripts/browser-qa.mjs`) to capture multi-viewport screenshots, extract computed DOM styles, execute WCAG 2.2 AA accessibility rules, and measure laboratory Core Web Vitals.

---

## 2. Restricted / Non-Bundled Upstream Projects (Legal Boundaries Enforced)

1. **React Bits (`DavidHDev/react-bits`)** — Licensed under **MIT + Commons Clause License Condition v1.0** (`Copyright (c) 2026 David Haz`).
   - **Enforcement**: Layoza does **not** copy, bundle, or redistribute any code or components from `react-bits`.
2. **Animate UI (`imskyleen/animate-ui`)** — Licensed under **MIT + Commons Clause License Condition** (`Copyright (c) 2025 Elliot Sutton`).
   - **Enforcement**: Layoza does **not** copy, bundle, or redistribute any code or components from `animate-ui`.
3. **GSAP (`greensock/GSAP`)** — Licensed under the **Standard "No Charge" GreenSock License** (https://gsap.com/standard-license).
   - **Enforcement**: Layoza does **not** bundle GSAP library files. Layoza generates standard consumer code that imports `gsap` in the user's own application when appropriate and documents GSAP's license terms clearly.
4. **Codrops (`github.com/codrops`)** — Repositories have heterogeneous licensing (some MIT such as `codrops/interlude`, many under custom Codrops tutorial terms without SPDX licenses).
   - **Enforcement**: Layoza treats unlicensed Codrops repositories as `reference-only` and implements all motion and WebGL/Three.js demonstrations from clean-room first-principles code.

---

## 3. Product UI ideas (adapted 2026-10-16, no code or text copied)

Used in `skills/layoza-product-ui/` (SKILL.md, `reference/tables-forms.md`, `reference/data-viz.md`) and the product-UI additions to `layoza`, `layoza-new-website`, `layoza-components`, `layoza-design-system`, `layoza-accessibility`, `layoza-performance`, `layoza-extend-website`, `layoza-redesign`, `layoza-wordpress`, `layoza-figma` and `layoza-creative-direction`. All text is Layoza's own; numbers and standards are cited as facts.

- **Vercel Web Interface Guidelines** (`vercel-labs/web-interface-guidelines`, https://github.com/vercel-labs/web-interface-guidelines). MIT License, Copyright (c) Vercel, Inc. Ideas: state in the URL, never blocking paste, `autocomplete`/`inputmode` on fields, 16px mobile inputs, no-break space before units, tabular numerals, loader delay and minimum duration, `color-scheme` and `theme-color` in dark mode, native `<select>` styling, submit enabled until the request starts.
- **Impeccable** (`pbakaus/impeccable`, https://github.com/pbakaus/impeccable), by Paul Bakaus. Apache License 2.0. Ideas from its operate and harden references: the per-view state set (empty, loading, error, permission, sparse and dense data), overlays escaping `overflow`, inline before modal, and the brand lending only type, palette, density and one signature move. **Modified by Layoza:** restated as a required state matrix with timing numbers, density row heights, Layoza QA done criteria and mode routing; no upstream text reused.
- **shadcn/ui skill docs** (`shadcn-ui/ui`, https://github.com/shadcn-ui/ui), by shadcn. MIT License. Ideas: detect the project through `components.json`, distinguish the Radix and Base UI primitive bases, add components with the project's CLI, and compose primitives instead of editing their source.
- **WAI-ARIA Authoring Practices Guide** (W3C, https://www.w3.org/WAI/ARIA/apg/). W3C Software and Document License. Keyboard interaction models for button, menu button, tabs, dialog, combobox, listbox, accordion, radio group, slider, grid and tooltip, summarised in Layoza's own tables.
- **Touch-target and density figures, cited as facts:** WCAG 2.2 SC 2.5.8 Target Size (Minimum) 24×24 CSS px (https://www.w3.org/TR/WCAG22/#target-size-minimum); Apple Human Interface Guidelines 44×44pt minimum hit target (https://developer.apple.com/design/human-interface-guidelines/accessibility); Material Design / Android 48×48dp touch target (https://support.google.com/accessibility/android/answer/7101858) and Material's 4dp density steps (https://m2.material.io/design/layout/applying-density.html).
- **Stephen Few, dashboard design principles** (*Information Dashboard Design*, Analytics Press). Cited as concepts only (single-screen primary set, priority placement, context for every measure, colour for exceptions, no gauges or 3D); no text quoted.

## 4. Motion-system ideas (adapted 2026-10-16, no code or text copied)

`skills/layoza-animations/reference/motion-system.md` restates these ideas in Layoza's own words. Numbers are cited as starting points:

- **lottiefiles/motion-design-skill** (LottieFiles, MIT): motion personalities with duration sets, travel-distance duration scaling, overshoot budgets by meaning, crowd and stagger limits, counter-motion and parallax speed ratios.
- **emilkowalski/skills** (Emil Kowalski, MIT): interruptible transitions over restarting keyframes, trigger-origin popovers, the tooltip group delay, flick-velocity dismissal, rubber-banding, slow-motion review.
- **vercel-labs/web-interface-guidelines** (Vercel, MIT): no `transition: all`, compositor-only properties, the loader show-delay and minimum duration, transitions disabled during theme switches.
- **heygen-com/hyperframes** motion docs (HeyGen, Apache-2.0): closed-form, seekable spring eases; velocity-matched hand-offs; transition energy by mood; the no-sameness rule within a scene; line-mask descender padding. Layoza's text is rewritten, not copied, and these ideas are modified for scroll-driven web pages.
- **pixel-point/animate-text** (no licence): used only as inspiration for naming text effects. Every effect spec in Layoza is its own.
- **raunofreiberg/interfaces** (no licence): ideas only, no text.
