# BRAND.md — layoza-mcp

## 1. Brand Preservation & Operating Contract

- **Default Mode**: `Preservation Mode (Extend/Improve without breaking existing visual identity)`
- **Detected Framework**: `html-css-js`
- **Dark Mode Strategy**: `light-default`
- **Extraction Confidence**: `85%` (Source inspected: `true`, Browser inspected: `false`)

## 2. Brand Color Architecture

| Token | Value | Role | Provenance Tier | Source |
| :--- | :--- | :--- | :--- | :--- |
| `bg-primary` | `#0D0E11` | CSS custom property `--bg-primary` | `ObservedFact` | `index.html` |
| `bg-surface` | `#17191E` | CSS custom property `--bg-surface` | `ObservedFact` | `index.html` |
| `bg-elevated` | `#21242C` | CSS custom property `--bg-elevated` | `ObservedFact` | `index.html` |
| `bg-glass` | `rgba(23, 25, 30, 0.82)` | CSS custom property `--bg-glass` | `ObservedFact` | `index.html` |
| `text-primary` | `#F5F3EF` | CSS custom property `--text-primary` | `ObservedFact` | `index.html` |
| `text-secondary` | `#B8BCC6` | CSS custom property `--text-secondary` | `ObservedFact` | `index.html` |
| `text-muted` | `#9CA3AF` | CSS custom property `--text-muted` | `ObservedFact` | `index.html` |
| `accent-primary` | `#FF4D2D` | CSS custom property `--accent-primary` | `ObservedFact` | `index.html` |
| `accent-secondary` | `#E2F163` | CSS custom property `--accent-secondary` | `ObservedFact` | `index.html` |
| `accent-tertiary` | `#38BDF8` | CSS custom property `--accent-tertiary` | `ObservedFact` | `index.html` |
| `border-subtle` | `rgba(245, 243, 239, 0.12)` | CSS custom property `--border-subtle` | `ObservedFact` | `index.html` |
| `border-strong` | `rgba(245, 243, 239, 0.28)` | CSS custom property `--border-strong` | `ObservedFact` | `index.html` |

## 3. Typography & Voice

| Role | Font Family | Size | Weight | Line Height | Letter Spacing | Provenance |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `font-display` | `Syne', system-ui, -apple-system, sans-serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |
| `font-sans` | `Plus Jakarta Sans', system-ui, -apple-system, sans-serif` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |
| `font-mono` | `Space Mono', monospace` | `1rem` | `400` | `1.5` | `0em` | `ObservedFact` |

## 4. Calibrated Design Dials (0–10)

- **Design Variance**: `4/10`
- **Motion Intensity**: `7/10`
- **Visual Density**: `5/10`

## 5. Recorded Unknowns & Verification Boundaries

- Live browser DOM snapshot not attached during static scan: run `layoza audit` or `scripts/browser-qa.mjs` to merge computed styles across desktop, tablet, and mobile viewports.
