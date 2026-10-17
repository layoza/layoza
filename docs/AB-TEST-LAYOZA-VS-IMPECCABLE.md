# Head-to-Head Master Benchmark: Layoza MCP vs `pbakaus/impeccable`

**Master Prompt Under Test**:
> **MASTER PROMPT: Build a World-Class, Highly Animated Creative Agency Website** (20 sections — Floating & Full-Screen Overlay Nav, Signature Cinematic 3D Hero `"WE MAKE DIGITAL FEEL ALIVE."`, 6-Discipline Interactive Services Stage, 5 Believable Agency Projects with Full-Screen Pinned Feature, Unconventional Brand Manifesto, Interactive Creative Playground, 01–05 Scroll-Driven Process, Demo-Disclosed Impact Metrics, Demo Brand Wall & Testimonials, Experimental Spatial Transition, Spectacular Final Contact CTA `"LET'S MAKE SOMETHING IMPOSSIBLE TO IGNORE."`, Intentional Footer, Global Motion System, and WCAG/Performance Engineering).

---

## 1. Installed Systems & Deliverables

1. **Installed `pbakaus/impeccable`**:
   - Cloned from `https://github.com/pbakaus/impeccable` and installed into `skills/impeccable/` (vendored copy, removed 2026-10-14) (including `reference/new-work.md`, `reference/overdrive.md`, `reference/craft-floor.md`, and its 18 sub-command references).
2. **Design 1 — Built with Our Layoza MCP + Layoza Skills (`KINETIX // ATELIER`)**:
   - **Live Website**: [`examples/master-agency-benchmark/layoza-mcp/index.html`](../examples/master-agency-benchmark/layoza-mcp/index.html)
   - **Extracted Google-Spec Design System**: [`examples/master-agency-benchmark/layoza-mcp/DESIGN.md`](../examples/master-agency-benchmark/layoza-mcp/DESIGN.md)
   - **Machine-Readable Site DNA**: [`examples/master-agency-benchmark/layoza-mcp/SITE-DNA.json`](../examples/master-agency-benchmark/layoza-mcp/SITE-DNA.json)
   - **Motion Specification**: [`examples/master-agency-benchmark/layoza-mcp/MOTION.md`](../examples/master-agency-benchmark/layoza-mcp/MOTION.md)
   - **Multi-Viewport Chromium QA Report**: [`examples/master-agency-benchmark/layoza-mcp/qa-report.json`](../examples/master-agency-benchmark/layoza-mcp/qa-report.json)
   - **Rendered Chromium Screenshots**:
     - Desktop (`1440×900`): [`examples/master-agency-benchmark/layoza-mcp/screenshots/desktop.png`](../examples/master-agency-benchmark/layoza-mcp/screenshots/desktop.png)
     - Tablet (`768×1024`): [`examples/master-agency-benchmark/layoza-mcp/screenshots/tablet.png`](../examples/master-agency-benchmark/layoza-mcp/screenshots/tablet.png)
     - Mobile (`375×812`): [`examples/master-agency-benchmark/layoza-mcp/screenshots/mobile.png`](../examples/master-agency-benchmark/layoza-mcp/screenshots/mobile.png)
3. **Design 2 — Built with `pbakaus/impeccable` (`MONOLITH // BUREAU`)**:
   - **Live Website**: [`examples/master-agency-benchmark/impeccable/index.html`](../examples/master-agency-benchmark/impeccable/index.html)
   - **Multi-Viewport Chromium QA Report**: [`examples/master-agency-benchmark/impeccable/qa-report.json`](../examples/master-agency-benchmark/impeccable/qa-report.json)
   - **Rendered Chromium Screenshots**:
     - Desktop (`1440×900`): [`examples/master-agency-benchmark/impeccable/screenshots/desktop.png`](../examples/master-agency-benchmark/impeccable/screenshots/desktop.png)
     - Tablet (`768×1024`): [`examples/master-agency-benchmark/impeccable/screenshots/tablet.png`](../examples/master-agency-benchmark/impeccable/screenshots/tablet.png)
     - Mobile (`375×812`): [`examples/master-agency-benchmark/impeccable/screenshots/mobile.png`](../examples/master-agency-benchmark/impeccable/screenshots/mobile.png)

---

## 2. How Inspecting the Real Screenshots Improved Our Layoza MCP Engine

When we inspected the initial rendered Chromium screenshots (`desktop.png`, `tablet.png`, `mobile.png`), we discovered **4 subtle real-world DOM geometry traps** that ordinary `document.documentElement.scrollWidth <= window.innerWidth` checks miss when `html, body { overflow-x: hidden }` is set:

1. **Header & Pill Badge Multi-Line Wrapping**:
   - Without `white-space: nowrap; flex-shrink: 0;` on `.brand-logo`, `.nav-link`, `.menu-toggle-btn`, `.btn-magnetic`, and `.step-num`, multi-word navigation items (`Selected Work`, `3D Lab`, `Start Project`, `Index [+]`, `01. Discover`) wrapped onto 2 lines inside the header bar and process badges.
2. **Wide Geometric Display Font (`Syne` 800 Uppercase) Container Clipping**:
   - Because `Syne` 800 has ultra-wide uppercase glyphs (`M`, `W`, `D`, `S`), 10–12 character uppercase words (`DISCIPLINES.`, `ENGINEERED`, `ARCHITECTS.`, `IMPOSSIBLE`) at `clamp(2.1rem, ...)` exceeded the `335px` inner width of a `375px` mobile screen, and `59.8fps` clipped (`scrollWidth > clientWidth`) inside its 4-column metric card on desktop.
3. **CSS Grid `1fr` Track Intrinsic Expansion on `375px` Mobile**:
   - Without `*, *::before, *::after { min-width: 0; }`, CSS Grid `1fr` columns default to `min-width: auto` and expand past `375px` when holding `<canvas>` or `nowrap` HUD badges.
4. **Mobile Header Crowding (`375px`)**:
   - Keeping both `Index [+]` and `Start Project` visible beside the studio wordmark on `375px` mobile pushed the right button `15px` past the viewport edge.

### Permanent Upgrades Made to Layoza MCP & Browser QA:
- **[`scripts/browser-qa.mjs`](../scripts/browser-qa.mjs)**:
  - Added element-level horizontal clipping & bleed detection (`el.scrollWidth > el.clientWidth + 3` and `rect.right > window.innerWidth + 2`) across all headings, metric values, buttons, and header items — even when `body { overflow-x: hidden }` masks the document scrollbar.
  - Added DOM `Range.getClientRects()` multi-line wrapping detection across all `<header>` links, brand wordmarks, action buttons, and `.step-num` / `.srv-tag` badges.
  - Added pairwise 2D AABB collision detection (`overlapping_interactive_elements`) across all visible interactive controls.
- **[`crates/layoza-core/src/verifier.rs`](../crates/layoza-core/src/verifier.rs)**:
  - Automatically ingests `qa-report.json` / `.layoza/qa-snapshot.json` during `layoza audit` and `layoza verify`, failing the release gate (`BROWSER-TEXT-CLIP-OR-HEADER-WRAP`, `BROWSER-HORIZONTAL-OVERFLOW`, `BROWSER-INTERACTIVE-OVERLAP`) whenever any viewport has `clipped_text_nodes > 0` or `horizontal_overflow_px > 0.5`.
- **[`crates/layoza-core/src/intelligence.rs`](../crates/layoza-core/src/intelligence.rs) & [`skills/layoza-visual-qa/SKILL.md`](../skills/layoza-visual-qa/SKILL.md)**:
  - Embedded mandatory **Header & Nav Spacing Guardrails** (`white-space: nowrap; flex-shrink: 0;`, mobile secondary CTA collapse) and **Wide Display Font & Metric Fit Guardrails** (`min-width: 0` on grid items, calibrated `clamp()` bounds for wide display fonts like `Syne`).

---

## 3. Final Verified Multi-Viewport Chromium QA Results (`1440×900`, `768×1024`, `375×812`)

| Verification Gate | Design 1: **Layoza MCP (`layoza-mcp`)** | Design 2: **Impeccable (`impeccable`)** |
| :--- | :--- | :--- |
| **Desktop (`1440×900`) Overflow & Clipped Text Nodes** | **`0.0 px` overflow · `0` clipped/wrapped nodes** | **`0.0 px` overflow · `0` clipped/wrapped nodes** |
| **Tablet (`768×1024`) Overflow & Clipped Text Nodes** | **`0.0 px` overflow · `0` clipped/wrapped nodes** | **`0.0 px` overflow · `0` clipped/wrapped nodes** |
| **Mobile (`375×812`) Overflow & Clipped Text Nodes** | **`0.0 px` overflow · `0` clipped/wrapped nodes** | **`0.0 px` overflow · `0` clipped/wrapped nodes** |
| **Overlapping Interactive Controls (`1440` / `768` / `375`)** | **`0` / `0` / `0`** | **`0` / `0` / `0`** |
| **Primary Text Contrast Ratio** | **`17.42 : 1` (WCAG AAA)** | **`17.37 : 1` (WCAG AAA)** |
| **Minimum Touch Target Size (`WCAG 2.2 SC 2.5.8`)** | **`44.0 px`** | **`44.0 px`** |
| **Unlabeled Interactive Controls** | **`0`** | **`0`** |
| **`:focus-visible` & `prefers-reduced-motion`** | **`true` / `true`** | **`true` / `true`** |
| **`layoza verify` Release Gate** | **PASS (`true`)** | **PASS (`true`)** |
