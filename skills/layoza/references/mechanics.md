# Mechanics: dealt ideas for hero, sections and transitions

A mechanic is one specific, buildable idea: a blueprint (grid, clamp() sizes, states), motion mapped
to scroll progress, a `still_check`, a 375px restage, a reduced-motion end state, failure modes and
both a CSS-only and a scripted path. Layoza's catalogue serves every niche (store, restaurant, local
business, SaaS, dashboard, agency, kinetic, 3D, editorial, portfolio, finance, health, education,
events, real estate, travel, nonprofit, docs, landing, professional services such as law or accounting) at three intensities: `calm`, `bold`, `wild`.

## Get a deal

```bash
layoza mechanics deal "<brief>" [--count 5] [--seed N] [--intensity calm|bold|wild] [--json]
layoza mechanics deal "<brief>" --existing .        # Mode B/C: follow the inspected site
layoza mechanics show <id>                          # full entry
layoza mechanics list --niche restaurant --kind section --intensity calm
```

MCP: `deal_mechanics` (brief, niche, mood, intensity, count, seed, mode, existing_motion_intensity)
and `get_mechanic` (id). `layoza design-system` already gives each of the 3 directions its own hero
plus 2 signature mechanics.

A deal returns 1 `hero`, 2 `signature` sections, `count - 2` `suggestions` and 1 `transition`,
never two of the same mechanism family, and the same deal for the same brief and seed.

## Rules

1. **New site (Mode A/D): the brief wins.** Its niche, mood and intensity words drive the deal;
   `--intensity` applies only when the brief names none.
2. **Existing site (Mode B/C): the site wins.** The deal is capped at the site's motion intensity and
   stack (SITE-DNA.json or a fresh inspect). Keep the current hero; build every mechanic with the
   site's own tokens, type, easing and durations. Never use a mechanic to restyle the site.
3. **Build the two signature mechanics verbatim in spirit**: keep the mechanism and the `still_check`,
   adapt colour, type, copy and objects to the brand and the trade's real artefacts.
4. **Suggestions are optional.** A section whose content has its own shape (a real menu, timetable,
   ledger) follows that shape instead.
5. **Calm is first-class.** A calm deal is all quiet, CSS-capable ideas; each section still gets a
   distinct idea. Do not add bold motion to a calm or existing site.
6. **Declare the motion level**: `<meta name="layoza-motion" content="calm|bold|wild|none">`.
   `layoza qa` uses it for `motion-too-sparse` (calm pages get a lower threshold; `none` skips it).
7. **Verify each mechanic's `still_check`** with `layoza qa`: no `pinned-stage-static` (a pinned stage
   must differ at 25/50/75% of its range), `motion-too-sparse`, `text-mid-reveal` or `art-over-text`.
