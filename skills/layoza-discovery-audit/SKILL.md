---
name: layoza-discovery-audit
description: >-
  Project discovery and comprehensive UI/UX, layout, accessibility, and code
  quality audit skill. Activate when inspecting an unfamiliar codebase, diagnosing
  design/usability issues, or establishing a pre-edit quality baseline.
---

# Layoza — Project Discovery & Design Audit

## Trigger Conditions
- The user asks to audit, critique, review, or inspect a frontend project or webpage.
- Before making any Mode B (Extend) or Mode C (Improve) edits.

## Step-by-Step Procedure

0. **Product context (`PRODUCT.md`)**: if the project has no `PRODUCT.md`, build it with [reference/product-context.md](reference/product-context.md): pre-fill from `layoza inspect` / `layoza design-system --json`, then ask the user only for what cannot be inferred (usually the success action). Each field is tagged observed / inferred / confirmed.
1. **Run Static & Structural Discovery**:
   ```bash
   cargo run --quiet --bin layoza -- inspect <path> --json
   cargo run --quiet --bin layoza -- audit <path> --json
   ```
2. **Run Real-Browser Multi-Viewport Audit** (when an HTML entry or dev server URL is available):
   ```bash
   layoza qa <url-or-file>
   ```
3. **Separate Objective Findings from Subjective Critique**:
   - Report **Deterministic Objective Findings** (`A11Y-HTML-LANG`, `RESP-VIEWPORT-META`, `A11Y-IMG-ALT`, `A11Y-FOCUS-VISIBLE`, `MOTION-REDUCED-MOTION`, horizontal overflow, console errors, WCAG contrast `< 4.5:1`) as actionable engineering defects.
   - Report **Subjective Design Critique** (visual hierarchy, typography pairing, generic gradient clichés) clearly labeled as design recommendations—never as scientific metrics.
4. **Scored report and fixes**: for a 0–20 health score with measured caps and a command per finding, use `audit` in [`layoza-refine`](../layoza-refine/SKILL.md); its commands then fix each finding inside a before/after `layoza qa` loop.

## Tested Example
```bash
cargo run --quiet --bin layoza -- audit examples/minimal-corporate/index.html
```
