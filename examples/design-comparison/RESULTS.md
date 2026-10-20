# Matched design comparison, 21 October 2026

Layoza did not win every brief. Taste v2 produced the strongest bakery dashboard in this run. Its advantage was useful phone density and operational ordering, not more decoration. Layoza led the library; Impeccable led the gallery; UI UX Pro Max led the clinic.

Open [the visual comparison](index.html). Each brief has side-by-side desktop and phone boards, full source pages and retained screenshots. [Anonymous critique](blind-review.md), [key](blind-key.json), [frozen source hashes](frozen-source-hashes.json), [protocol](protocol.md) and [pinned upstream versions](upstreams.json) are retained. These are skill-conditioned implementations by our agents, not submissions by the authors.

| Brief | Initial blind visual ordering | Layoza's specific weakness |
|---|---|---|
| Intervals gallery | Impeccable > Layoza ≈ Taste > UI UX Pro Max | Heading delayed photographs; repeated mounts lengthened phone viewing |
| Willow library | Layoza > Impeccable > Taste ≈ UI UX Pro Max | Two-row phone header, wrapped search arrow, small secondary labels |
| Noor Dental | UI UX Pro Max > Layoza > Taste > Impeccable | Long phone opening, duplicated reassurance, faint input borders |
| Hearth operations | Taste > Impeccable ≈ UI UX Pro Max > Layoza | Oversized phone summary, operational risks too late, early demo disclosure missing |

One image-only critic reviewed eight anonymous boards and all sixteen full phone captures without reading source, rationale or identities. Letters were shuffled separately for each brief. Its findings are judgments, not measured probabilities. Root's interpretation and later changes are disclosed separately.

## What was actually wrong in our MCP

The structured client task and objects changed the summary, but the industry-focusing stage discarded those lines before composing directions. A library could therefore receive museum-like or booking-led assumptions despite an explicit catalogue task. The dealer also allowed concepts requiring absent capabilities or evidence: video with no motion, a quote without a supplied quote, availability grids without slots, a postcode check without coverage information, and a developer playground for ordinary bakery operations.

The generated craft floor contradicted the skills. It prescribed a signature animation, four layout types, a picture in every phone hero, universal crops and an accent-area quota. These recipes could work against gallery preservation, monochrome briefs and dense applications.

Changes in the core and actual CLI/MCP output:

- Preserve explicit client actions and objects through focusing, subject reading and direction composition, including unknown trades.
- Prefer the specific photography-gallery trade over generic museum ticket/plinth assumptions.
- Reject the incompatible direction candidates above before ranking them.
- Generate contextual craft guidance: zero motion when pinned; whole artworks; real working interfaces; no fixed layout or accent-area quotas; truthful labelled illustrative material.
- Update art-production and product-UI skills with the observed image-selection and phone task-order failures, replacing generic advice rather than adding a larger checklist.

[Updated MCP outputs](engine-after/) and [behavior checks](engine-fix-checks.json) use the same four frozen brief contexts as the originals in `arms/layoza/retrieval/`. Engine improvements alone do not establish better visual taste.

## Visible Layoza refinements

The original sixteen pages and rankings remain frozen. Four separately named [refined Layoza pages](arms/layoza-refined/) and [before/after boards](refinements.html) show the response:

- Gallery: photographs first on desktop and phone; complete framing; larger captions; quieter mounts; introduction retained after the first pair.
- Library: one-row phone header, single-line Search, larger secondary labels and tighter event spacing; early search retained.
- Clinic: bounded human consultation photo first, compact phone header/heading, fewer pauses before first-visit guidance and clearer input borders. Both stock photographs remain labelled.
- Bakery: compact four-way counts, early oven/shortage strip, ovens/stock before collections, visible phone demo badge, quantities/units and working controls retained.

The [second visual review](refinement-review.md) confirms improvement and records tradeoffs. It is explicitly nonblind and does not rerank refined Layoza against unrefined rivals. Secondary navigation links are reduced in compact phone headers while their content remains available.

## Validation

- Initial comparison: 16 pages × desktop 1440, phone 390 and narrow 320 = 48 viewport checks; all 16 required local task flows pass.
- Refined set: another 12 viewport checks and four local task flows pass. Shared [checks](checks.json) cover overflow, image loading, labels, h1, anchors, runtime errors, catalogue/no-results safety, forms/date constraints, filters, queued→baking→ready and batch creation. No speed checks.
- Rust workspace: 248 passed, none failed, one ignored debug-print test. [Full output](validation/rust-tests.txt).
- Actual updated MCP outputs pass client-context/capability checks; stdio [smoke](validation/mcp-smoke.txt) passes, with all 24 default tools.
- Updated Windows binary is local. Other platform prebuilds and npm publication are outside this GitHub upload.

This is not a full accessibility certification, a human usability study or a statistical benchmark.

## Where we still lack and limits

The initial dashboard was less efficient than Taste's; the revised dashboard needs a new matched round before claiming parity or a win. Gallery art direction was less decisive than Impeccable's initial art-first sequence. Clinic pacing was less controlled than UI UX Pro Max. The revisions address these observed weaknesses; they do not prove universal superiority.

Creative range remains less proven: the library and clinic repeatedly converge on cream, green, serif type and arches across multiple systems. These briefs partly encourage that convergence. A broader brand/typography study needs less-related briefs and permitted font sourcing, not additional forbidden-style rules.

The controlled run used common local photographs and system fonts. Taste's inspector failed to launch its Chrome, and Impeccable's native engine/context and seed tools were unavailable; both used their documented fallbacks plus common Chromium checks. UI UX Pro Max's real Python retrievals and Taste's actual seeded deals are retained. Layoza's builder used actual MCP retrievals but did not produce the full three rendered candidate studies before selection. These deviations prevent describing this as each product's unrestricted native workflow. One model family and one judge also limit generalization. The four clients are fictional, realistic briefs; no claim of live client approval is made.

Public upstreams: [Taste](https://github.com/Leonxlnx/taste-skill), [UI UX Pro Max](https://github.com/nextlevelbuilder/ui-ux-pro-max-skill), [Impeccable](https://github.com/pbakaus/impeccable). Exact commits and [licences](licenses/) retained. Photograph credits and provenance: [CREDITS](CREDITS.txt), [asset sources](asset-sources.json).
