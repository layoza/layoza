# BRAND.md — arm-a-layoza

## 1. Brand Preservation & Operating Contract

- **Default Mode**: `Preservation Mode (Extend/Improve without breaking existing visual identity)`
- **Detected Framework**: `html-css-js`
- **Dark Mode Strategy**: `light-default`
- **Extraction Confidence**: `85%` (Source inspected: `true`, Browser inspected: `false`)

## 2. Brand Color Architecture

| Token | Value | Role | Provenance Tier | Source |
| :--- | :--- | :--- | :--- | :--- |
| `color-bg-limestone` | `#F7F6F2` | CSS custom property `--color-bg-limestone` | `ObservedFact` | `index.html` |
| `color-surface-warm` | `#EFECE6` | CSS custom property `--color-surface-warm` | `ObservedFact` | `index.html` |
| `color-surface-card` | `#FFFFFF` | CSS custom property `--color-surface-card` | `ObservedFact` | `index.html` |
| `color-ink-slate` | `#14181D` | CSS custom property `--color-ink-slate` | `ObservedFact` | `index.html` |
| `color-muted-slate` | `#4F5864` | CSS custom property `--color-muted-slate` | `ObservedFact` | `index.html` |
| `color-accent-emerald` | `#0F3D3E` | CSS custom property `--color-accent-emerald` | `ObservedFact` | `index.html` |
| `color-accent-terracotta` | `#B84A27` | CSS custom property `--color-accent-terracotta` | `ObservedFact` | `index.html` |
| `color-hairline` | `#D8D3C8` | CSS custom property `--color-hairline` | `ObservedFact` | `index.html` |

## 3. Typography & Voice

| Role | Font Family | Size | Weight | Line Height | Letter Spacing | Provenance |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `font-display` | `Instrument Serif", Georgia, "Times New Roman", serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |
| `font-body` | `Public Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |
| `font-mono` | `JetBrains Mono", Consolas, monospace` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |

## 4. Calibrated Design Dials (0–10)

- **Design Variance**: `4/10`
- **Motion Intensity**: `3/10`
- **Visual Density**: `5/10`

## 5. Recorded Unknowns & Verification Boundaries

- Live browser DOM snapshot not attached during static scan: run `layoza audit` or `scripts/browser-qa.mjs` to merge computed styles across desktop, tablet, and mobile viewports.
