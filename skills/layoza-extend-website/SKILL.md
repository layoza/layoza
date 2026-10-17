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

1. **INSPECT FIRST — DO NOT CODE BLIND**:
   ```bash
   cargo run --quiet --bin layoza -- inspect <project-dir> --write
   ```
   Read `SITE-DNA.json`, `DESIGN.md`, `BRAND.md`, `MOTION.md`, and `COMPONENT-INVENTORY.json`.
2. **Match Original Design Team Craft**:
   - Reuse the exact existing CSS variables (`--*`), Tailwind tokens, or `theme.json` presets.
   - Match the existing heading/body font families, container max-widths, border radii, and vertical section padding.
   - Match the existing motion library and easing curves.
3. **Zero Unrequested Dependencies or Framework Swaps**:
   - Never introduce a different CSS framework or animation library for a single new section.
4. **Verify Visual & Token Consistency**:
   ```bash
   cargo run --quiet --bin layoza -- verify <project-dir>
   ```

## Extending an App Screen (dashboards, admin, settings)
1. **Reuse, don't rebuild:** find the existing table, form field, empty state, skeleton, toast and dialog components in `COMPONENT-INVENTORY.json` (MCP `discover_components`) and use them. A second `DataTable` or `FormField` is a defect.
2. **Match the incumbent density:** measure row height, control height, cell padding and type size on a sibling screen (devtools or `layoza qa` screenshots) and use the same tokens. A comfortable 48px table next to compact 32px ones reads as a different product.
3. **Inherit the state set:** the new view renders the same empty, loading, error and permission-denied patterns as its siblings, with the same copy tone; missing ones follow [`layoza-product-ui`](../layoza-product-ui/SKILL.md) §6 in the incumbent style.
4. **Same URL and shortcut conventions:** filters/sort/page in the query string the way siblings do it; register new actions in the existing command palette and shortcut map.

## Tested Example
```bash
cargo run --quiet --bin layoza -- inspect examples/existing-site-preservation --write
cargo run --quiet --bin layoza -- verify examples/existing-site-preservation
```
