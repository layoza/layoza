---
name: layoza-figma
description: Authorized Figma token, variable, frame, and component context ingestion into Layoza DESIGN.md and SITE-DNA.json without fabricating missing measurements.
---

# Layoza Specialist Skill: Authorized Figma Context (`layoza-figma`)

Use this skill when the user provides a Figma file export, W3C Design Tokens JSON, or authorized Figma MCP context and wants to translate it into `DESIGN.md`, `SITE-DNA.json`, or production frontend components.

## 1. Ingestion Hierarchy & Provenance
1. **Authorized Figma Variables / Token Exports (`ObservedSource`, confidence `0.95–1.0`)**:
   - Map color styles/variables directly into `--color-*` semantic tokens.
   - Map text styles (`fontFamily`, `fontWeight`, `fontSize`, `lineHeight`, `letterSpacing`) into `--font-*` and typography scale tokens.
   - Map Auto Layout padding, gap, and corner radius into spacing and radius tokens.
2. **Exported Frame Screenshots (`ObservedBrowser`, confidence `0.65–0.80`)**:
   - When only raster frame exports are available, extract visual hierarchy and layout proportions while explicitly recording unknown hover/focus/motion states in `SITE-DNA.json -> unknowns`.
3. **Motion Reconstruction**:
   - Treat multi-frame Figma storyboards as state keyframes (Initial -> Active -> Exit) rather than static slides, and synthesize matching GSAP / Motion transitions via `layoza-animations`.

## 2. Workflow
1. Import tokens or inspect project with `layoza inspect . --write`.
2. Validate contrast and structural completeness with `layoza design-system --lint DESIGN.md`.
3. Generate framework-native components bound to the imported tokens via `layoza generate`.

## 3. Modes, Variants and Missing States
1. **Variable modes → token modes:** a `Light` / `Dark` collection mode maps to `:root` and `:root[data-theme="dark"]` values of the same token; a `Comfortable` / `Default` / `Compact` mode maps to `[data-density]` values (`--row-h`, `--control-h`, `--cell-px`). Keep Figma's variable names as token names (`color/bg/danger` → `--color-bg-danger`).
2. **Component variants → state tokens:** `State=Hover|Focus|Pressed|Disabled|Loading|Error|Selected` properties map onto the states table in [`layoza-components`](../layoza-components/SKILL.md) §5 and to semantic tokens (`danger`, `selected`, `focus`); `Size` and `Density` properties map to density tokens, not new components.
3. **Missing states are unknowns, not guesses:** when a component lacks a focus, disabled, loading, error, empty or dark-mode variant, list each one in `SITE-DNA.json -> unknowns` (component, state, confidence 0) and in the report; derive it from the state tokens only after saying so.
