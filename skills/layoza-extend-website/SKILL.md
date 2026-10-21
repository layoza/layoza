---
name: layoza-extend-website
description: >-
  Mode B skill for adding a new page, section, hero, navbar, footer, pricing block,
  testimonial block, FAQ, or service page to an existing website while strictly
  preserving its brand identity, tokens, framework, and motion language.
---

# Layoza — Extend Existing Website (Mode B: Preservation Mode)

## Trigger Conditions
- The user asks to add a page, section, footer, navbar, hero, pricing table, testimonial block, or FAQ to an existing project.

## Non-Negotiable Preservation Rules

0. **The user wins, then the site.** An explicit user instruction (palette,
   stack, layout, motion library, scope) overrides every Layoza default; where
   the user is silent, the existing design decides.
   **No new directions.** The existing site *is* the direction: never offer
   three directions, hero sketches or a pick, and never run `layoza deal` or
   `design-system --mode new` for the look. Build exactly the section or page
   asked for so it looks authored by the original team. A request for a new
   look is Mode D ([`layoza-redesign`](../layoza-redesign/SKILL.md)): say so and stop.
1. **INSPECT FIRST — DO NOT CODE BLIND**:
   ```bash
   cargo run --quiet --bin layoza -- inspect <project-dir> --write
   ```
   (MCP: `inspect_project`, then `extract_site_dna` with `write_approved: true`.)
   Read `SITE-DNA.json`, `DESIGN.md`, `BRAND.md`, `MOTION.md`, and `COMPONENT-INVENTORY.json`,
   and open one or two sibling pages or sections to copy their structure.
2. **Match Original Design Team Craft**:
   - Reuse the exact existing CSS variables (`--*`), Tailwind tokens, or `theme.json` presets.
   - Match the existing heading/body font families, container max-widths, border radii, and vertical section padding.
   - Match the existing motion library and easing curves.
   - Reuse existing components and copy tone; build new markup only where none exists.
3. **Zero Unrequested Dependencies or Framework Swaps**:
   - Never introduce a different CSS framework or animation library for a single new section.
4. **Verify Visual & Token Consistency**:
   ```bash
   cargo run --quiet --bin layoza -- verify <project-dir>
   ```
   Then screenshot the new part beside a sibling (`layoza qa`) and confirm it
   reads as the same site. No `.layoza/direction.md` is needed; if you log an
   art loop there, record `PICKED: existing — <the design being followed>`.

## Extending an App Screen (dashboards, admin, settings)
1. **Reuse, don't rebuild:** find the existing table, form field, empty state, skeleton, toast and dialog components in `COMPONENT-INVENTORY.json` (MCP `discover_components`) and use them. A second `DataTable` or `FormField` is a defect.
2. **Match the incumbent density:** measure row height, control height, cell padding and type size on a sibling screen (devtools or `layoza qa` screenshots) and use the same tokens. A comfortable 48px table next to compact 32px ones reads as a different product.
3. **Inherit the state set:** the new view renders the same empty, loading, error and permission-denied patterns as its siblings, with the same copy tone; missing ones follow [`layoza-product-ui`](../layoza-product-ui/SKILL.md) §6 in the incumbent style.
4. **Same URL and shortcut conventions:** filters/sort/page in the query string the way siblings do it; register new actions in the existing command palette and shortcut map.

## Tested Example
```bash
cargo run --quiet --bin layoza -- inspect path/to/site --write
cargo run --quiet --bin layoza -- verify path/to/site
```
