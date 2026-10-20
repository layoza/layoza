---
name: layoza-performance
description: Core Web Vitals optimization (LCP, CLS, INP), font loading strategy, image optimization, animation frame-budget discipline, and honest Lab vs Field telemetry reporting.
---

# Layoza Specialist Skill: Frontend Performance (`layoza-performance`)

Use this skill only when the user explicitly requests speed diagnostics or optimization. It is optional and never a prerequisite for designing, visually reviewing, or completing a design. Run `layoza qa <target> --performance` or MCP `audit_performance` separately from the design verdict.

## 1. Core Web Vitals Targets
- **LCP (Largest Contentful Paint)**: `<= 2,500ms`
  - Never hide the hero headline or primary hero image at `opacity: 0` waiting for a slow JS bundle to hydrate before painting.
  - Use `fetchpriority="high"` and `loading="eager"` on the primary above-the-fold hero visual; use `loading="lazy"` and `decoding="async"` on below-the-fold media.
- **CLS (Cumulative Layout Shift)**: `<= 0.10` (target `0.00`)
  - Always set explicit `width` and `height` or CSS `aspect-ratio` on every `<img>`, `<video>`, and `<canvas>`.
  - Pair web fonts with `font-display: swap` and metric-compatible fallback fonts (`size-adjust`) to prevent layout reflow on font swap.
- **INP (Interaction to Next Paint)**: `<= 200ms`
  - Keep main-thread tasks under `50ms`.
  - Avoid synchronous layout thrashing (`getBoundingClientRect` inside `scroll` or `mousemove` handlers; use `ResizeObserver`, `IntersectionObserver`, or batched `requestAnimationFrame`).

## 2. Honest Lab vs Field Distinction
Never claim that a synthetic local Playwright or Lighthouse run represents real-user CrUX field data.
- **Laboratory Metrics (`LaboratorySynthetic`)**: Measured deterministically in headless Chromium on local hardware; ideal for pre-commit regression gating.
- **Field Metrics (`FieldRealUser`)**: Real User Monitoring (RUM / CrUX 75th percentile across real devices and networks). Always label performance reports with their exact `PerformanceMeasurementSource`.

## 3. INP in apps (interaction-heavy screens)
- **Virtualise** lists and tables above ~200 rows; fixed row heights from the density token keep it cheap.
- **Debounce** search and filter inputs 200–300ms; update the input immediately, defer the results (`startTransition` / `useDeferredValue` in React, or an idle callback).
- **Move heavy work off the main thread:** parsing, sorting or aggregating >10k rows, CSV export and diffing go to a Web Worker; yield in long loops (`scheduler.yield()` where supported, else `setTimeout(0)` chunks).
- **Optimistic UI** for toggles, renames and reorders; roll back with a message on failure. Visual feedback within 100ms, work done within 500ms or show progress.

## 4. Budgets and loading states
- **JS per route** (compressed, initial): marketing ≤100KB, app shell ≤170KB, each lazy route ≤100KB more. Split charts, editors and maps into their own chunks loaded on use.
- **Skeletons:** delay ~150–300ms, show at least ~300–500ms, and reserve the final size (CLS 0). Never swap a 40px skeleton for a 64px row.
- **Images:** explicit `width`/`height` or `aspect-ratio`, `srcset` + `sizes`, AVIF/WebP; avatars and logos sized to their rendered box (×2 for HiDPI); lazy below the fold only.
- **Fonts:** at most 2 families and 4 files on first load, WOFF2, subset, preload only the one used above the fold, metric-matched fallback (`size-adjust`, `ascent-override`). Tabular data can use the system UI stack.
