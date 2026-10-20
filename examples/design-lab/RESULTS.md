# Local evaluation results — 21 October 2026

The four selected designs pass the rebuilt MCP browser audit with zero objective errors. This is a local engineering and design-workflow result, not proof of universal design superiority or a full WCAG certification.

| Design | Objective errors | Minimum measured text contrast | Performance diagnostics |
|---|---:|---:|---|
| intervals | 0 | 5.5:1 | Off |
| willow | 0 | 6.27:1 | Off |
| noor | 0 | 5.19:1 | Off |
| hearth | 0 | 4.61:1 | Off |

- Rust workspace: **247 passed, 0 failed, 1 ignored** (the ignored test prints a deal for manual inspection).
- **24 tools** discoverable by default; all seven prompts, argument validation, ping and clean shutdown pass in the exact extracted npm package.
- Compilation from the exact package contents passes. The previously missing render-facts helper and all new helpers are included.
- Windows release, vendor binary and extracted package binary have identical SHA-256: `b3ddc2f3c1e82963ea5d5f3275acbd177aa417c3f4913b7ba64db14e5e2307f1`.
- Four realistic briefs produced 12 rendered compositions; six additional fixed briefs passed engine-contract checks. The held-out cases were not visually evaluated.
- **32** responsive composition checks at 1440, 768, 390 and 375 pixels, plus real enquiry, membership/search, appointment and production/dialog flows. Queued batches start baking before becoming ready; quantities and percentages update together.
- MCP Chromium audits also exercised the standard viewports and extra-width checks. Fast mode runs the full rule sweep on desktop and responsive/layout checks on tablet/phone; no speed diagnostics were requested.
- A fresh assistant critic reviewed only briefs and images in two rounds. Selected layouts changed in response; the final designer/browser pass also fixed contrast, reading sizes and action visibility.
- Review records preserve actual screenshot paths and source fingerprints. Source or screenshot changes make evidence stale. Decoded PNG comparisons retain actual first-round/final changes; an MCP same-render check measures zero change. Pixel changes are not design quality scores.

## What changed in the product

Explicit brief context now controls subject/audience/task/facts and pinned choices; negative instructions no longer become the subject. Public-library, clinical, monochrome/font and unknown-subject probes are covered. Composition material is no longer overwritten by a supporting mechanic; product interfaces stay working interfaces.

The skill router and canonical creative process now agree on assets, rendered comparisons, designer selection and visual review. Novelty, a signature effect and numeric self-scores do not certify taste or force an unnecessary approval pause. Motion uses 0–10 consistently, with zero meaning no animation. The browser critic recognizes task interfaces instead of demanding imagery and frame breaks in every dashboard panel.

Real asset import/search/registration, DTCG/Figma Variables export import with modes and aliases, source-token extraction, component metadata, persisted screenshot review, freshness checks and actual decoded screenshot comparison now have executable paths. Existing geometry is not replaced by invented observed defaults; same-length source edits invalidate evidence. The Vue quoted-heading regression and observed radius reuse are checked.

## Boundaries

The examples are complete local HTML/CSS studies implemented by an assistant designer guided by the MCP. The engine supplies context and tools; its JSON is not an autonomous page-rendering model. Client businesses, programmes, operating figures and forms are explicitly illustrative; no external submissions occur.

Native framework outputs remain starters requiring composition and native runtime review. Vue string round-tripping is checked, but a Vue compiler/full framework integration matrix was unavailable. Figma import covers local token/variable exports, not remote frame conversion. Linux/macOS bundled binaries were not rebuilt here; a multiplatform release still requires CI builds. Nothing was published or deployed. A fair current competitor comparison still requires fresh equal-condition builds and more than one judge.

See [README](README.md), [runtime evidence](runtime-results.json), [contract evidence](contract-results.json), [browser evidence](browser-results.json), [review evidence](review-results.json) and each study's source/render files.
