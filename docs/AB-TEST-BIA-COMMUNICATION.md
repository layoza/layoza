# Head-to-Head A/B Benchmark: "Make a Website for BIA Communication"

> Historical experiment. Some original page sources/renders are no longer retained. Scores here are development notes, not a current reproducible competitor ranking. See [the retained design lab](../examples/design-lab/README.md) for current local workflow evidence.

**Prompt Under Test**:
> *"Make a website for BIA Communication"*

**Experimental Setup**:
- **Arm A (`examples/ab-test-bia-communication/arm-a-layoza/`)**:
  Built with **Layoza's 16 progressive-disclosure skills + Project-Scoped Layoza MCP Server (`layoza_layoza`)** + `layoza inspect` (`DESIGN.md` & `SITE-DNA.json`) + `scripts/browser-qa.mjs` & `layoza verify`.
- **Arm B (`examples/ab-test-bia-communication/arm-b-copied-skills/`)**:
  Built with the **"Copy All 138 GitHub Skills" approach** (raw static markdown skills from `awesome-design-skills`, `ui-ux-pro-max`, `taste-skill`, `impeccable`, `frontend-design` without Layoza's MCP engine or browser verification loop).

---

## 1. Live Multi-Viewport Chromium QA Results (`1440×900`, `768×1024`, `375×812`)

Both implementations were rendered in headless Chromium via [`scripts/browser-qa.mjs`](../scripts/browser-qa.mjs) and audited via `layoza verify`:

| Metric / Verification Gate | Arm A: **Layoza Skills + Project MCP** | Arm B: **Copied All 138 GitHub Skills** | Winner |
| :--- | :--- | :--- | :--- |
| **Skill Context Footprint** | **16 focused skills** (0 context truncation) | **152 skills** (triggered IDE context-budget exclusion warning!) | **Arm A (Layoza)** |
| **Mobile `375×812` Horizontal Overflow** | **`0.0 px`** (Zero scroll bleed) | **`94.0 px` overflow** (Nav bar broke mobile viewport) | **Arm A (Layoza)** |
| **Desktop `1440×900` & Tablet `768×1024` Overflow** | **`0.0 px`** | **`0.0 px`** | Tie |
| **WCAG 2.2 AA `:focus-visible` Styling** | **`true`** (`2px solid #0F3D3E` + skip link) | **`false`** (No keyboard focus ring) | **Arm A (Layoza)** |
| **`prefers-reduced-motion: reduce` Guard** | **`true`** (CSS `@media` + JS `matchMedia` guard) | **`false`** (Missing reduced-motion support) | **Arm A (Layoza)** |
| **WCAG 2.2 SC 2.5.8 Touch Target Size** | **`44.0 px`** (All controls `>= 38–44px`) | **`20.0 px`** (Undersized `< 24px` nav links) | **Arm A (Layoza)** |
| **Typography Scale & Pairing (`H1 / Body`)** | **`3.91x`** (`Instrument Serif` + `Public Sans` + `JetBrains Mono`) | **`2.67x`** (Generic `system-ui` sans-serif) | **Arm A (Layoza)** |
| **Color Palette & Anti-Slop Originality** | Warm Limestone (`#F7F6F2`) + Deep Slate (`#14181D`, `16.48:1`) + Mineral Emerald (`#0F3D3E`) | Generic white/slate + AI-slop blue-purple gradient (`linear-gradient(135deg, #2563EB, #4F46E5)`) | **Arm A (Layoza)** |
| **Interactive Domain Proof Above the Fold** | Live **Mandate Protocol Workbench** (M&A / Crisis / IPO tabs) + **Filterable Case Ledger** | Static 3-column generic service cards | **Arm A (Layoza)** |
| **Generated `DESIGN.md` & `SITE-DNA.json`** | **Yes** (6 artifacts with `Baseline Hash: e50a29a59a962377`) | **No** | **Arm A (Layoza)** |
| **Layoza Release Gate (`layoza verify`)** | **PASS (`100%`)** | **FAIL** (Mobile overflow `94px`, touch targets `20px`, missing `:focus-visible` & `prefers-reduced-motion`) | **Arm A (Layoza)** |

---

## 2. Why "Focused Layoza Skills + Project MCP" Beats "Copying All 138 GitHub Skills"

1. **Zero Context Window Overflow**:
   - When all 138 upstream GitHub skills were copied into `skills/`, Antigravity's skill loader hit its context budget ceiling and emitted:
     `The following items were excluded due to context budget limits...`
   - With Layoza's **16 progressive-disclosure skills** in `skills/` (while keeping the 138 upstream skills archived in `data/upstream/`), 100% of Layoza's skills load cleanly with zero truncation.
2. **Deterministic MCP Design Reasoning vs Conflicting Markdown Prompts**:
   - Calling `layoza_layoza::recommend_layout` returned a single coherent domain profile (`Corporate & Enterprise` -> `Asymmetric Editorial Authority Grid with Proof Metrics Rail`, `Instrument Serif` + `Public Sans`, `#F7F6F2` limestone + `#0F3D3E` emerald, `15.2:1` contrast) plus 3 distinct creative directions.
   - By contrast, raw copied skills contain conflicting advice (`corporate` vs `colorful` vs `gradient`), leading to the classic blue-purple gradient button cliché.
3. **Real Browser Geometry Verification Catches Mobile Bugs**:
   - Static markdown skills cannot see that a flex navbar overflows a `375px` mobile screen by `94px`. Layoza's `scripts/browser-qa.mjs` + `layoza verify` caught the `94px` mobile overflow, `20px` touch target violation, missing `:focus-visible`, and missing `prefers-reduced-motion` immediately.

---

## 3. Artifacts & Screenshots

- **Arm A (Layoza + Project MCP)**:
  - Code: ``examples/ab-test-bia-communication/arm-a-layoza/index.html`` (archived artifact unavailable)
  - Extracted Design System: ``examples/ab-test-bia-communication/arm-a-layoza/DESIGN.md`` (archived artifact unavailable)
  - Extracted Site DNA: ``examples/ab-test-bia-communication/arm-a-layoza/SITE-DNA.json`` (archived artifact unavailable)
  - QA Snapshot: [`.layoza/qa-bia-arm-a.json`](../.layoza/qa-bia-arm-a.json)
  - Screenshots: `.layoza/screenshots/bia-arm-a/{desktop,tablet,mobile}.png`
- **Arm B (Copied All Skills)**:
  - Code: ``examples/ab-test-bia-communication/arm-b-copied-skills/index.html`` (archived artifact unavailable)
  - QA Snapshot: [`.layoza/qa-bia-arm-b.json`](../.layoza/qa-bia-arm-b.json)
  - Screenshots: `.layoza/screenshots/bia-arm-b/{desktop,tablet,mobile}.png`
