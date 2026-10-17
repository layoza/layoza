# Layoza Progressive-Disclosure Skills Guide

Layoza ships **16 skills** (`skills/*/SKILL.md`) designed around progressive disclosure so AI coding agents never suffer from context-window bloat.

---

## 1. Universal Entry Router
- **[`layoza`](../skills/layoza/SKILL.md)**: Always loaded first. Classifies the request into Operating Mode A, B, C, or D, checks behavioral dials (`DESIGN_VARIANCE`, `MOTION_INTENSITY`, `VISUAL_DENSITY`), and routes to only the needed specialist skills.

---

## 2. Mode & Workflow Specialist Skills
- **[`layoza-discovery-audit`](../skills/layoza-discovery-audit/SKILL.md)**: Evaluates project readiness, stakeholder intent, and baseline UX gaps.
- **[`layoza-site-dna`](../skills/layoza-site-dna/SKILL.md)**: Extracts `DESIGN.md`, `BRAND.md`, `MOTION.md`, `SITE-DNA.json`, `COMPONENT-INVENTORY.json`, and `DESIGN-DECISIONS.md` with provenance tags.
- **[`layoza-creative-direction`](../skills/layoza-creative-direction/SKILL.md)**: Synthesizes 3 genuinely distinct creative directions (layout archetype, typography pairing, palette, motion language, and explicit trade-offs).
- **[`layoza-new-website`](../skills/layoza-new-website/SKILL.md)**: Mode A end-to-end build workflow with anti-AI-slop enforcement.
- **[`layoza-extend-website`](../skills/layoza-extend-website/SKILL.md)**: Mode B strict preservation workflow for adding pages, heroes, pricing tables, or footers to an existing site.
- **[`layoza-redesign`](../skills/layoza-redesign/SKILL.md)**: Mode C (Targeted Improvement) and Mode D (Full Creative Redesign) with mandatory `.layoza/baselines/<id>` snapshot and rollback.
- **[`layoza-design-system`](../skills/layoza-design-system/SKILL.md)**: Token architecture, Google `DESIGN.md` spec generation, linting, and diffing.

---

## 3. Engineering, Motion & Verification Specialist Skills
- **[`layoza-animations`](../skills/layoza-animations/SKILL.md)**: GSAP, ScrollTrigger, Motion, WAAPI, View Transitions, magnetic interactions, and conditional Lenis guardrails.
- **[`layoza-threejs-webgl`](../skills/layoza-threejs-webgl/SKILL.md)**: Three.js / React Three Fiber scenes with DPR capped at `<= 2.0`, `IntersectionObserver` pausing, GPU buffer/material/texture disposal, and reduced-motion fallbacks.
- **[`layoza-components`](../skills/layoza-components/SKILL.md)**: Clean-room, framework-native UI component generation across 11 framework targets.
- **[`layoza-accessibility`](../skills/layoza-accessibility/SKILL.md)**: WCAG 2.2 AA contrast math, `:focus-visible`, keyboard operability, target size (`>= 24x24px`), and automated vs manual audit separation.
- **[`layoza-visual-qa`](../skills/layoza-visual-qa/SKILL.md)**: Live Chromium multi-viewport DOM geometry checks, screenshot comparison, objective vs subjective separation, and bounded auto-repair.
- **[`layoza-performance`](../skills/layoza-performance/SKILL.md)**: Core Web Vitals (`LCP <= 2500ms`, `CLS <= 0.10`, `INP <= 200ms`) and honest `LaboratorySynthetic` vs `FieldRealUser` telemetry labeling.
- **[`layoza-figma`](../skills/layoza-figma/SKILL.md)**: Authorized Figma variable, token, and frame ingestion into `SITE-DNA.json` and `DESIGN.md`.
- **[`layoza-wordpress`](../skills/layoza-wordpress/SKILL.md)**: Native WordPress Gutenberg Block Themes (`theme.json` v3, `patterns/*.php`, `templates/*.html`) plus Elementor and Bricks JSON adapters.
