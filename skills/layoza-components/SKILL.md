---
name: layoza-components
description: Framework-native UI component generation across React, Next.js, Astro, Vue, Nuxt, SvelteKit, semantic HTML/CSS/JS, and WordPress without violating project conventions or clean-room licensing.
---

# Layoza Specialist Skill: Framework-Native Components (`layoza-components`)

Use this skill when generating reusable UI components (Navbars, Heroes, Bento Grids, Pricing Tables, Data Tables, Modals, Accordions, Command Palettes, Footers) tailored to the host project's exact stack.

## 1. Stack Detection First
Never dump React/Tailwind code into an Astro, Vue, SvelteKit, WordPress, or vanilla CSS project.
1. Inspect the project via `layoza inspect . --write` or MCP `inspect_project` / `discover_components`.
2. Reuse existing primitives (`components/ui/*`) when present instead of creating duplicate Button/Card abstractions.
3. Consume design tokens (`var(--color-*)`, `var(--font-*)`, `var(--radius-*)`) from `SITE-DNA.json` / `DESIGN.md`.

## 2. Clean-Room & Licensing Guardrails
- **MIT Inspiration Allowed**: `magicuidesign/magicui` (MIT) and `ibelick/motion-primitives` (MIT) patterns may be adapted with attribution in `THIRD_PARTY_NOTICES.md`.
- **Commons Clause Restricted**: Never copy source code from `DavidHDev/react-bits` or `imskyleen/animate-ui` (both carry Commons Clause restrictions). Implement all interactive primitives from first principles using Layoza's clean-room adapters.

## 3. CLI & MCP Generation
```bash
layoza generate --component hero --framework nextjs --title "Precision Cloud Infrastructure" --subtitle "Sub-millisecond edge compute with verifiable telemetry."
```
Supported `--framework` targets:
- `react`, `nextjs`, `astro`, `vue`, `nuxt`, `sveltekit`, `html`, `wordpress-gutenberg`, `elementor`, `bricks`, `shopify-liquid`.

## 4. Compose, don't fork (shadcn / Radix / Base UI / React Aria)
- `components.json` present means shadcn/ui. Confirm the primitive base from `package.json`: `radix-ui` or `@radix-ui/react-*` → Radix (`asChild`); `@base-ui-components/react` or `@base-ui/react` → Base UI (`render` prop); `react-aria-components` → React Aria.
- Add missing pieces with the project's CLI (`npx shadcn@latest add <name>`) so they land in `aliases.ui` on the right base.
- Build features by composing primitives (`DataTable` = `Table` + `Checkbox` + `DropdownMenu`). Change looks through tokens and `cva` variants; never copy a primitive's source to restyle it, and never mix two bases.

## 5. States by variant (every interactive component ships all of them)

| State | Visual rule | Semantics |
| :--- | :--- | :--- |
| Default | Token colours; ≥3:1 boundary vs surface | Native element or correct role |
| Hover | One step darker/lighter surface; no layout shift | `@media (hover: hover)` only |
| Focus-visible | 2px ring, 2px offset, ≥3:1 vs control and surface | `:focus-visible`; never removed |
| Active | Pressed surface; ≤120ms | `aria-pressed` for toggles |
| Disabled | Muted, keeps 3:1 label if it explains why | `disabled` (or `aria-disabled` + tooltip when it must stay focusable) |
| Loading | Spinner after ~150–300ms, width locked | `aria-busy="true"`, repeat clicks ignored |
| Error | Danger token + icon + text, never colour alone | `aria-invalid`, `aria-describedby` → message |
| Selected | Selected token + check or indicator | `aria-selected` / `aria-checked` / `aria-current` |

Apply per variant (primary, secondary, ghost, destructive): a ghost button still needs a visible focus ring and hover.

## 6. Keyboard maps (WAI-ARIA APG patterns)

| Component | Keys |
| :--- | :--- |
| Button / toggle | Enter, Space |
| Menu button | Enter/Space/↓ opens, ↑↓ move, Home/End, type-ahead, Esc closes and refocuses trigger |
| Tabs | One tab stop; ←→ move, Home/End; activate on focus (or Enter for manual) |
| Dialog | Focus inside on open, Tab trapped, Esc closes, focus returns to trigger |
| Combobox | ↓ opens list, ↑↓ move, Enter selects, Esc closes then clears, typing filters |
| Listbox / select | ↑↓, Home/End, type-ahead, Space/Enter selects; Shift+↑↓ for multi |
| Accordion | Enter/Space toggles; optional ↑↓ between headers |
| Radio group | One tab stop; arrows move and select |
| Slider | ←→/↑↓ step, Page Up/Down large step, Home/End min/max |
| Grid (data) | One tab stop; arrows move cells, Enter/F2 edits, Esc cancels |
| Tooltip | Opens on focus, Esc dismisses, never holds actions |

Prefer the library's primitive, which already implements the map; test it anyway with Tab, arrows and Esc.
