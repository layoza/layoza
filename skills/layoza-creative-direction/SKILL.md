---
name: layoza-creative-direction
description: >-
  Reading Layoza's design engine output (industry profiles, 3 structural
  directions, 0-10 dials for variance, motion and density). The art direction
  itself lives in layoza-art-direction; load that for any new site or redesign.
---

# Layoza: Creative Direction (engine input)

```bash
layoza design-system "<brief>" --mode new --json
```

- **Directions** are structural skeletons (layout composition, content
  treatment, dials), not finished looks. Their palettes and fonts come from a
  generic industry profile, so the visual world comes from
  [`layoza-art-direction`](../layoza-art-direction/SKILL.md) (seven worlds,
  `layoza deal`, world from far and subject up close).
- **Dials** (0–10): Design Variance, Motion Intensity, Visual Density. Use them
  as a starting register; the user's prompt and the committed world override them.
- **match_confidence**: `strong` means the profile's UX rules apply (booking
  flows, trust signals); `weak` or `none` means only its structure is useful.
- **Present** (attended runs): one committed direction, up to two alternates,
  and a "category standard, played straight" exit. **Unattended:** build the
  committed one and list your assumptions.

## Product UI register
For apps, dashboards, admin and tools ([`layoza-product-ui`](../layoza-product-ui/SKILL.md)), override the engine's dials:
- **Design Variance ≤ ~3**: standard navigation, layout model and controls; no asymmetric heroes or broken grids inside the shell.
- **Visual Density 6–7**: default 40px rows, 8/4px spacing grid, compact mode available.
- **Motion Intensity ≤ 2**: the "Tool / product UI" personality, 120/200/280ms, no decorative motion.

Present one extra direction type for these briefs: **"Category standard, polished, with one signature move"**: the layout users already know, executed better than the category's usual product (tinted shell, tuned type, complete states) plus one named distinctive move (a signature data treatment, a precise accent, brand-world empty states). It is usually the committed direction.
