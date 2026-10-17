# BRAND.md — layoza-mcp

## 1. Brand Preservation & Operating Contract

- **Default Mode**: `Preservation Mode (Extend/Improve without breaking existing visual identity)`
- **Detected Framework**: `html-css-js`
- **Dark Mode Strategy**: `light-default`
- **Extraction Confidence**: `85%` (Source inspected: `true`, Browser inspected: `false`)

## 2. Brand Color Architecture

| Token | Value | Role | Provenance Tier | Source |
| :--- | :--- | :--- | :--- | :--- |
| `bg-ivory` | `#FAF7F2` | CSS custom property `--bg-ivory` | `ObservedFact` | `index.html` |
| `bg-porcelain` | `#F2ECE1` | CSS custom property `--bg-porcelain` | `ObservedFact` | `index.html` |
| `bg-alabaster` | `#EAE2D3` | CSS custom property `--bg-alabaster` | `ObservedFact` | `index.html` |
| `bg-cobalt-stage` | `#081229` | CSS custom property `--bg-cobalt-stage` | `ObservedFact` | `index.html` |
| `ink-midnight` | `#0B1021` | CSS custom property `--ink-midnight` | `ObservedFact` | `index.html` |
| `ink-body` | `#252D42` | CSS custom property `--ink-body` | `ObservedFact` | `index.html` |
| `ink-muted` | `#525B73` | CSS custom property `--ink-muted` | `ObservedFact` | `index.html` |
| `accent-cobalt` | `#0047FF` | CSS custom property `--accent-cobalt` | `ObservedFact` | `index.html` |
| `accent-cobalt-deep` | `#0033BF` | CSS custom property `--accent-cobalt-deep` | `ObservedFact` | `index.html` |
| `accent-vermilion` | `#E63B20` | CSS custom property `--accent-vermilion` | `ObservedFact` | `index.html` |
| `border-hairline` | `rgba(11, 16, 33, 0.13)` | CSS custom property `--border-hairline` | `ObservedFact` | `index.html` |
| `border-strong` | `rgba(11, 16, 33, 0.28)` | CSS custom property `--border-strong` | `ObservedFact` | `index.html` |

## 3. Typography & Voice

| Role | Font Family | Size | Weight | Line Height | Letter Spacing | Provenance |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `font-display` | `Playfair Display', Georgia, serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |
| `font-sans` | `Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, sans-serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |
| `font-mono` | `JetBrains Mono', monospace` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |

## 4. Calibrated Design Dials (0–10)

- **Design Variance**: `4/10`
- **Motion Intensity**: `3/10`
- **Visual Density**: `5/10`

## 5. Recorded Unknowns & Verification Boundaries

- Live browser DOM snapshot not attached during static scan: run `layoza audit` or `scripts/browser-qa.mjs` to merge computed styles across desktop, tablet, and mobile viewports.
