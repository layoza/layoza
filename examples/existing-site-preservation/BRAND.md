# BRAND.md — existing-site-preservation

## 1. Brand Preservation & Operating Contract

- **Default Mode**: `Preservation Mode (Extend/Improve without breaking existing visual identity)`
- **Detected Framework**: `html-css-js`
- **Dark Mode Strategy**: `light-default`
- **Extraction Confidence**: `85%` (Source inspected: `true`, Browser inspected: `false`)

## 2. Brand Color Architecture

| Token | Value | Role | Provenance Tier | Source |
| :--- | :--- | :--- | :--- | :--- |
| `color-bg-parchment` | `#F6F1EA` | CSS custom property `--color-bg-parchment` | `ObservedFact` | `index.html` |
| `color-surface-cream` | `#FFFDF9` | CSS custom property `--color-surface-cream` | `ObservedFact` | `index.html` |
| `color-ink-espresso` | `#1E1815` | CSS custom property `--color-ink-espresso` | `ObservedFact` | `index.html` |
| `color-muted-umber` | `#5C4E46` | CSS custom property `--color-muted-umber` | `ObservedFact` | `index.html` |
| `color-accent-terracotta` | `#B84A27` | CSS custom property `--color-accent-terracotta` | `ObservedFact` | `index.html` |
| `color-border-sand` | `#DFD5C7` | CSS custom property `--color-border-sand` | `ObservedFact` | `index.html` |

## 3. Typography & Voice

| Role | Font Family | Size | Weight | Line Height | Letter Spacing | Provenance |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `font-display` | `Cormorant Garamond", Georgia, serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |
| `font-body` | `DM Sans", -apple-system, BlinkMacSystemFont, sans-serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |

## 4. Calibrated Design Dials (0–10)

- **Design Variance**: `4/10`
- **Motion Intensity**: `3/10`
- **Visual Density**: `5/10`

## 5. Recorded Unknowns & Verification Boundaries

- Live browser DOM snapshot not attached during static scan: run `layoza audit` or `scripts/browser-qa.mjs` to merge computed styles across desktop, tablet, and mobile viewports.
