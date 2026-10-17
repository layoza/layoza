# BRAND.md — wordpress-block-theme

## 1. Brand Preservation & Operating Contract

- **Default Mode**: `Preservation Mode (Extend/Improve without breaking existing visual identity)`
- **Detected Framework**: `wordpress-gutenberg`
- **Dark Mode Strategy**: `light-default`
- **Extraction Confidence**: `85%` (Source inspected: `true`, Browser inspected: `false`)

## 2. Brand Color Architecture

| Token | Value | Role | Provenance Tier | Source |
| :--- | :--- | :--- | :--- | :--- |
| `canvas` | `#F8F7F4` | WordPress theme.json palette token (Canvas Background) | `ObservedFact` | `theme.json` |
| `ink` | `#141619` | WordPress theme.json palette token (Primary Ink) | `ObservedFact` | `theme.json` |
| `accent` | `#1849A9` | WordPress theme.json palette token (Brand Accent) | `ObservedFact` | `theme.json` |
| `surface` | `#FFFFFF` | WordPress theme.json palette token (Card Surface) | `ObservedFact` | `theme.json` |

## 3. Typography & Voice

| Role | Font Family | Size | Weight | Line Height | Letter Spacing | Provenance |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `display` | `"Instrument Serif", Georgia, serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |
| `body` | `"Plus Jakarta Sans", system-ui, sans-serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |

## 4. Calibrated Design Dials (0–10)

- **Design Variance**: `4/10`
- **Motion Intensity**: `3/10`
- **Visual Density**: `5/10`

## 5. Recorded Unknowns & Verification Boundaries

- Live browser DOM snapshot not attached during static scan: run `layoza audit` or `scripts/browser-qa.mjs` to merge computed styles across desktop, tablet, and mobile viewports.
