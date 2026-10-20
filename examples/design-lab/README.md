# Layoza design lab

Four realistic fictional-client briefs, three rendered compositions each, retained source and browser evidence. Open [the local study index](index.html) or [the overview image](design-studies.png).

| Study | Selected design | Desktop | Phone |
|---|---|---|---|
| Photography gallery | [Intervals](intervals/index.html) | [Render](intervals/qa/desktop-a.png) | [Render](intervals/qa/mobile-a.png) |
| Public library | [Willow](willow/index.html) | [Render](willow/qa/desktop-a.png) | [Render](willow/qa/mobile-a.png) |
| Dental clinic | [Noor](noor/index.html) | [Render](noor/qa/desktop-a.png) | [Render](noor/qa/mobile-a.png) |
| Bakery dashboard | [Hearth](hearth/index.html) | [Render](hearth/qa/desktop-a.png) | [Render](hearth/qa/mobile-a.png) |

Each folder also retains `concept-b.html`, `concept-c.html`, the original brief, CLI and MCP recommendations, provenance records, direction choice, screenshot evidence and a persisted review. The first-round renders remain in `qa/round-1/`.

The MCP supplies brief understanding, starting compositions, searchable design knowledge, asset and token provenance, native scaffolds and verification. An assistant designer developed the actual HTML/CSS and interactions. The engine does not autonomously generate these complete designs from its JSON. The same factual content and available assets were used across alternatives. Clinical imagery is labelled as illustrative; historical photographs retain original titles and dates; operational data is explicitly fictional.

An independent assistant critic received only the briefs and rendered images, not source or rationale. Its first round selected A for Intervals/Hearth and C for Willow/Noor. The final selected pages incorporate those findings: phone gallery works/captions/enquiry, search and hours before library promotion, a human consultation with first-visit explanation, and phone production quantities plus early operational alerts. The second review found no blocking visible composition issue. This is an image-only assistant review, not a human panel or a competitor ranking. Remaining small metadata type was disclosed; the final functional checks additionally exercised queued → baking → ready behavior.

## Reproduce locally

From the repository root:

```powershell
cargo build --locked --release --bin layoza
node examples/design-lab/evaluate.mjs
node examples/design-lab/verify-engine.mjs
node examples/design-lab/build.mjs
# Use a directory containing Playwright and its installed Chromium:
$env:LAYOZA_PLAYWRIGHT_DIR='C:\path\to\node-runtime'
$env:PLAYWRIGHT_BROWSERS_PATH='C:\path\to\playwright-browsers'
node examples/design-lab/capture.mjs
node examples/design-lab/persist-reviews.mjs
```

On another platform pass the native binary path as the first argument to the evaluation/contract scripts. The capture script owns a localhost-only server and closes both Chromium and that server. It makes no remote application changes. Form submissions stay on the page, membership/enquiry messages are unsent, and production state resets on reload. No performance checks run.

`engine-results.json` records four real MCP brief/asset contracts. `contract-results.json` records token aliases/modes, truthful source extraction, same-length edit freshness and six separately frozen held-out briefs; held-out briefs test engine contracts only, not visual quality. `browser-results.json` records 32 viewport/variant checks at 1440, 768, 390 and 375 pixels plus four real interaction flows. Runtime and package evidence is summarized in [RESULTS.md](RESULTS.md).

See [asset sources](asset-sources.json) and [credits](CREDITS.txt). No publication or universal “best design” claim is implied.
