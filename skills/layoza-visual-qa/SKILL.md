---
name: layoza-visual-qa
description: Real-browser verification with `layoza qa` (headless Chromium at 1440/768/375, scrolled like a visitor), what its measured rules mean, how objective errors differ from design critique, and before/after comparison.
---

For a full-page composition review, run `node scripts/review-composition.mjs <url-or-html> <output-directory>`. It captures full pages at 1440/768/390 and measures type scale, first-screen fill, section rhythm and media reach. Read the saved images and `composition-review.json`. Its observations are prompts, not verdicts: judge them against the brief and chosen identity.

# Layoza: Visual QA (`layoza qa`)

QA is the floor under the design, not the goal. Design critique lives in
`layoza-art-direction/reference/finish.md` (the art loop).

```bash
layoza qa <file-or-url>        # or: node scripts/browser-qa.mjs --file index.html --out <report.json> --screenshots <dir>
layoza verify .                # ingests the report into the release gate
```

`layoza qa` writes `browser-qa-report.json` and `desktop`, `tablet`, `mobile`,
`mobile-hero`, `*-menu-open` PNGs to `qa/` next to a target file (inside a target
folder; `.layoza/screenshots/` for a URL); the script uses `--out`/`--screenshots`.
Look at the screenshots; a report never proves a design. `reduced_motion_respected`
is measured under emulated reduce (`null` = no motion, fine).

- **Judge after motion settles:** a frame caught mid-animation or before the
  scene draws proves nothing; re-capture.
- **Labels over 3D, canvas, video or images:** check by eye at every width
  that text stays readable over the actual pixels; add a scrim, plate or
  position change when it is not.

## What is measured

| Severity | Rules (examples) | Meaning |
| :--- | :--- | :--- |
| `error` (blocks release) | `document-overflow`, `text-clip-or-bleed`, contrast < 4.5:1, `target-size` (WCAG 2.5.8), `unlabeled-control`, `focus-obscured` (2.4.11), `text-hidden` / `-without-js`, `text-faded-or-masked`, `nav-unreachable`, `menu-toggle-broken`, `zoom-blocked`, broken images, console errors | Objective failure. Fix the cause. |
| `warning` | `label-wrap`, `heading-crowding`, `ios-input-zoom`, `field-autocomplete`, `dark-color-scheme`, `image-no-size`, `pointer-not-button`, `transition-all`, `focus-ring-missing`, `motion-ignores-reduce`, `reduced-motion-hides-text`, `design-system-*` drift | Objective, but needs judgment. Fix or justify. |
| `critique` (`CRAFT-*`) | template chrome, card kits, `visual-starved`, overused fonts, monotonous spacing, and more | Generated-page tells. Review them against the committed direction. |

Speed diagnostics are off by default (also with `--full`); run `--performance` or MCP `audit_performance` only when asked. Unmeasured speed stays `null` and never blocks.

## Rules for fixing

- Never game a checker: no hiding content from the scan, no `opacity: 0.01`, no
  empty `prefers-reduced-motion` rule, no cutting a feature or idea to clear a rule.
- A rule you believe is wrong for this page: leave it, say why, and let the
  user decide. Brand DNA can be declared with
  `<meta name="layoza-brand-allow" content="font:…, palette:…, rule:…">`.
- Before/after: `layoza baseline create <id>`, make the changes, then
  `compare_before_after` (MCP) or `layoza verify .`.
