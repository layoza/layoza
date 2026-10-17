---
name: layoza-design-system
description: >-
  Design system extraction, Google-spec DESIGN.md authoring, token reference
  validation, WCAG contrast linting, and version diffing skill. Activate when
  creating, linting, or diffing DESIGN.md files or design tokens.
---

# Layoza — Design System Extraction & `DESIGN.md` Management

## Trigger Conditions
- Generating, validating (`--lint`), or comparing (`--diff`) `DESIGN.md` files.
- Managing design tokens across colors, typography, spacing, radii, elevation, and component variants.

## Canonical `DESIGN.md` Contract (Google Spec Compatible)

A valid `DESIGN.md` file has:
1. **YAML Frontmatter** (`version`, `name`, `description`, `colors`, `typography`, `rounded`, `spacing`, `components` with `{colors.*}` / `{rounded.*}` token references).
2. **Markdown Body** with sections in strict canonical order:
   `Overview` -> `Colors` -> `Typography` -> `Layout` -> `Elevation & Depth` -> `Shapes` -> `Components` -> `Do's and Don'ts`.

## Commands

```bash
# Lint a DESIGN.md file for canonical section order, broken references, and WCAG 2.2 AA contrast:
cargo run --quiet --bin layoza -- design-system --lint DESIGN.md

# Diff two DESIGN.md files to detect token additions, removals, modifications, and regressions:
cargo run --quiet --bin layoza -- design-system DESIGN.md DESIGN.v2.md --diff
```

## Product tokens (apps, dashboards, admin)

1. **Semantic state tokens**, each a `bg` / `fg` / `border` triple, never a raw hue in components:
   `success`, `warning`, `danger`, `info`, `selected`, `focus` (ring), plus interaction steps `hover` and `pressed` derived from the surface.
2. **Density tokens** switched by one attribute (`[data-density]`): `--row-h` 48/40/32px, `--control-h` 40/36/28px, `--cell-px` 16/12/8px on an 8px grid with a 4px sub-grid. Hit areas stay ≥24px CSS at every density.
3. **Dark-mode pairs**: every colour token has a light and dark value, authored in OKLCH so lightness steps stay even, e.g.
   ```css
   :root { --danger-fg: oklch(0.50 0.19 27); --surface: oklch(0.99 0 0); }
   :root[data-theme="dark"] { --danger-fg: oklch(0.75 0.15 27); --surface: oklch(0.20 0.01 260); }
   ```
   Dark surfaces step lighter for elevation (+0.03–0.05 L per level); hover and focus get a larger step than in light.
4. **Contrast per state token, per surface, per theme**: text tokens ≥4.5:1, icons/borders/focus ring ≥3:1 against every surface they sit on (base, raised, selected row). Record the measured ratios in `DESIGN.md` under `Colors` and re-run `--lint` after any change; a state token that only passes on one surface is a defect.
