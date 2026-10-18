---
name: layoza-new-website
description: >-
  Mode A: a brand-new website, landing page, web app or dashboard. Routes to
  layoza-art-direction (the design method) for sites and to
  layoza-product-ui for apps, dashboards and tools; covers framework choice
  and delivery.
---

# Layoza: New Website (Mode A)

1. **The user's prompt wins.** An explicit instruction (framework or stack,
   style, palette, fonts, layout, motion library, scope) overrides every Layoza
   default and is followed exactly. Where it leaves the look open, the three
   directions must differ from `avoid_recent` (MCP `get_design_system`) unless
   the user asks for the same look.
   **Intake first:** know the site's purpose and primary action, audience,
   whether it is new or extends an existing site, and what must be kept. Ask
   for what the prompt leaves open (art-direction step 0). If the folder holds
   an existing site, this is Mode B/C/D, not A.
2. **Design method:** follow [`layoza-art-direction`](../layoza-art-direction/SKILL.md)
   end to end: subject inventory, rut, seven worlds plus `layoza deal`, fusion,
   contract and copy, build, motion, art loop, then the QA floor.
3. **Apps, dashboards and tools go elsewhere.** When the brief is a web app,
   SaaS screen, dashboard, admin, settings or mobile app UI, follow
   [`layoza-product-ui`](../layoza-product-ui/SKILL.md) instead of the
   world-building steps:
   - still run `layoza design-system "<brief>" --mode new --json`, but read
     its dials as a product register (Variance ≤3, Density 6–7);
   - the brand lends type, palette, density and one signature move
     (`layoza-art-direction/reference/surfaces.md`, Operate); navigation,
     layout model and controls stay category-standard;
   - build the shell, then each view with its full state matrix, using the
     project's component library if one exists.
   A product with a marketing home splits: art direction for the home,
   product UI for the app, one shared token set.
4. **Framework:** use the project's own stack. When there's none, pick what fits
   the deliverable: one HTML file for a single page, Astro for content sites,
   the existing stack or Next.js / React / Vue / SvelteKit for apps, and WordPress block themes, Shopify Liquid or
   Elementor when the platform is named (see `layoza-components` and
   `layoza-wordpress`).
5. **Motion and 3D** are decided per design: [`layoza-animations`](../layoza-animations/SKILL.md)
   and [`layoza-threejs-webgl`](../layoza-threejs-webgl/SKILL.md).
6. **Done means:** `.layoza/direction.md` records `PURPOSE:`, the user's
   `PICKED:` direction, at least 3 `ROUND` lines and a `REVIEWER:` verdict; the
   art loop scores ≥ 9 (reviewer agrees); and `layoza qa <file-or-url>`
   reports 0 errors, with screenshots reviewed. A passing gate alone is not done.
   **For apps and dashboards:** every view renders its state matrix, `layoza qa`
   reports 0 errors, tables work at 375px, and the main task passes a
   keyboard-only walk-through. No art-loop score; motion uses the tool
   personality (120/200/280ms).
