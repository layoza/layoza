---
name: layoza-visual-qa
description: Real-browser verification with `layoza qa` (headless Chromium at 1440/768/375, scrolled like a visitor), what its measured rules mean, how objective errors differ from design critique, and before/after comparison.
---

# Layoza: Visual QA (`layoza qa`)

QA is the floor under the design, not the goal. Design critique lives in
`layoza-art-direction/reference/finish.md` (the art loop).

```bash
layoza qa <file-or-url>        # or: node scripts/browser-qa.mjs --file index.html --out qa/report.json --screenshots qa
layoza verify .                # ingests the report into the release gate
```

Output: `qa/browser-qa-report.json`, `qa/desktop.png`, `qa/tablet.png`,
`qa/mobile.png`, `qa/mobile-hero.png` and `qa/*-menu-open.png`. Open the
screenshots and look at them; the report alone never proves a design.

- **Judge after motion settles.** Entrances, reveals, canvas and WebGL must
  have finished before a frame counts; a frame caught mid-animation or before
  the scene draws proves nothing. Re-capture if it looks unfinished.
- **Labels over 3D, canvas, video or images** are checked by eye at every
  width: the text must stay readable over the actual pixels behind it. Do not
  rely on the automated contrast checks alone there; add a scrim, plate or
  position change when it is not.

## What is measured

| Severity | Rules (examples) | Meaning |
| :--- | :--- | :--- |
| `error` (blocks release) | `document-overflow`, `text-clip-or-bleed`, contrast < 4.5:1, `target-size` (WCAG 2.5.8), `unlabeled-control`, `focus-obscured` (2.4.11), `text-hidden-after-scroll`, `nav-unreachable`, `menu-toggle-broken`, `zoom-blocked`, broken images, console errors | Objective failure. Fix the cause. |
| `warning` | `label-wrap`, `heading-crowding`, `ios-input-zoom`, `field-autocomplete`, `dark-color-scheme`, `image-no-size`, `pointer-not-button`, `transition-all`, `design-system-*` drift | Objective, but needs judgment. Fix or justify. |
| `critique` (`CRAFT-*`) | template chrome, card kits, `hero-art-scale`, overused fonts, monotonous spacing, and more | Generated-page tells. Review them against the committed direction. |

Lab LCP, CLS and INP are measured; an unmeasured value is `null`, never a pass.

## Rules for fixing

- Never game a checker: no hiding content from the scan, no `opacity: 0.01`, no
  cutting a feature or a design idea to clear a rule.
- A rule you believe is wrong for this page: leave it, say why, and let the
  user decide. Brand DNA can be declared with
  `<meta name="layoza-brand-allow" content="font:…, palette:…, rule:…">`.
- Before/after: `layoza baseline create <id>`, make the changes, then
  `compare_before_after` (MCP) or `layoza verify .`.
