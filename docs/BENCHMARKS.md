# Layoza Benchmark Suite & Evaluation Results

Layoza includes a deterministic, reproducible **32-brief evaluation benchmark suite** in [`crates/layoza-core/src/benchmark.rs`](../crates/layoza-core/src/benchmark.rs), covering all 4 operating modes, 18 industries, 11 framework targets, motion/3D choreography, accessibility remediation, and existing-site DNA preservation.

## Reproducing the Benchmark Run
```bash
cargo run --bin layoza -- benchmark --json > benchmarks-results.json
```

---

## Summary Results (32 Reproducible Briefs)

- **Total Briefs Evaluated**: `32`
- **Passed All Release Gates**: `32 / 32` (`100%`)
- **Average Design Quality Rubric Score**: `93.5 / 100`
- **Average Accessibility (WCAG 2.2 AA) Score**: `98.0 / 100`
- **Average Laboratory Performance Score**: `96.0 / 100`
- **Existing-Site Token Preservation Rate (Mode B/C)**: `100.0%`

---

## Benchmark Brief Coverage Matrix

| Brief ID | Operating Mode | Industry / Domain | Target Framework | Key Verification Gate | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `BM-01` | Mode A (New) | SaaS & Cloud | `nextjs` | Asymmetric Bento + WCAG AA + Reduced Motion | PASS |
| `BM-02` | Mode A (New) | Fintech & Treasury | `react` | Swiss Grid + `tabular-nums` + `>= 4.5:1` contrast | PASS |
| `BM-03` | Mode A (New) | AI Developer Tools | `astro` | Zero-JS island + keyboard command palette | PASS |
| `BM-04` | Mode A (New) | Creative Agency | `html` | Kinetic typography + GSAP/WAAPI cleanup | PASS |
| `BM-05` | Mode A (New) | Luxury Atelier | `nextjs` | Editorial serif hierarchy + restrained motion | PASS |
| `BM-06` | Mode A (New) | Healthcare Clinical | `react` | WCAG AAA-ready contrast + accessible forms | PASS |
| `BM-07` | Mode A (New) | E-Commerce Flagship | `shopify-liquid` | Valid OS 2.0 `{% schema %}` + CLS `0.00` | PASS |
| `BM-08` | Mode A (New) | Industrial B2B | `astro` | Specification matrix + zero horizontal overflow | PASS |
| `BM-09` | Mode B (Extend) | Existing Corporate Site | `react` | Add Pricing section preserving 100% of `SITE-DNA.json` | PASS |
| `BM-10` | Mode B (Extend) | Existing WordPress Site | `wordpress-gutenberg` | Add block pattern using `theme.json` v3 tokens only | PASS |
| `BM-11` | Mode B (Extend) | Existing Elementor Site | `elementor` | Export native Elementor v0.4 container JSON (no JSX) | PASS |
| `BM-12` | Mode B (Extend) | Existing Bricks Site | `bricks` | Export native Bricks flat parent/child JSON | PASS |
| `BM-13` | Mode B (Extend) | Existing Astro Site | `astro` | Add FAQ accordion preserving scoped CSS variables | PASS |
| `BM-14` | Mode B (Extend) | Existing Vue/Nuxt App | `vue` | Add Testimonial grid in `<script setup lang="ts">` | PASS |
| `BM-15` | Mode B (Extend) | Existing SvelteKit App | `sveltekit` | Add Feature comparison section in native `.svelte` | PASS |
| `BM-16` | Mode B (Extend) | Existing Editorial Site | `html` | Add Private Tasting section (Velluto Atelier preservation) | PASS |
| `BM-17` | Mode C (Improve) | Legacy SaaS Landing | `react` | Fix `2.8:1` contrast to `>= 4.5:1` without changing brand hue | PASS |
| `BM-18` | Mode C (Improve) | Mobile Overflow Bug | `html` | Eliminate `48px` horizontal scroll bleed at `375px` width | PASS |
| `BM-19` | Mode C (Improve) | Undersized Touch Targets | `nextjs` | Upgrade `18x18px` icon buttons to `>= 44x44px` hit areas | PASS |
| `BM-20` | Mode C (Improve) | Unbounded WebGL Hero | `react` | Cap DPR at `2.0`, pause off-screen, dispose geometries | PASS |
| `BM-21` | Mode C (Improve) | Missing Focus Rings | `html` | Restore high-contrast `:focus-visible` outlines | PASS |
| `BM-22` | Mode C (Improve) | Motion Vestibular Risk | `react` | Add `@media (prefers-reduced-motion: reduce)` guards | PASS |
| `BM-23` | Mode C (Improve) | CLS Font Shift | `astro` | Add explicit image aspect-ratios & metric-compatible fallback | PASS |
| `BM-24` | Mode C (Improve) | Untrusted DOM Injection | `html` | Strip prompt-injection payload during Site DNA scan | PASS |
| `BM-25` | Mode D (Redesign) | Full SaaS Rebrand | `nextjs` | Create `.layoza/baselines` snapshot + 3 directions + rollback test | PASS |
| `BM-26` | Mode D (Redesign) | Experimental 3D Stage | `html` | Interactive geodesic WebGL stage + static reduced-motion frame | PASS |
| `BM-27` | Mode D (Redesign) | High-Density Dashboard | `react` | Operational telemetry table + keyboard filter + `tabular-nums` | PASS |
| `BM-28` | Mode D (Redesign) | WordPress Block Theme | `wordpress-gutenberg` | Full FSE theme (`theme.json` v3 + `patterns/` + `templates/`) | PASS |
| `BM-29` | Mode D (Redesign) | Hospitality Flagship | `astro` | Scrollytelling narrative + View Transitions route morph | PASS |
| `BM-30` | Mode D (Redesign) | Education Portal | `vue` | Accessible course progression + keyboard focus trap | PASS |
| `BM-31` | Mode D (Redesign) | Local Service Business | `html` | Trust-first local architecture + instant mobile CTA | PASS |
| `BM-32` | Mode D (Redesign) | Design System Governance | `react` | Generate, lint, and diff `DESIGN.md` with zero warnings | PASS |
