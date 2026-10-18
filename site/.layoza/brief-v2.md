# Layoza site — brief v2 (redesign)

Why v2: v1 ("drawing set") read as a tidy spec sheet, not a show. It was static between sections,
the 3D hero was small and slow to settle, and labels collided with the scene. User verdict: not good.

## Brief (one line)
Marketing site for Layoza, an open-source MCP server + 19 skills that make AI coding agents design
like a studio instead of shipping the average of the web. Highly animated, cinematic, never gimmicky.

## Subject (the trade's own objects — show these, not abstract blobs)
- A generic AI page being *transformed* into a designed one: the before/after bench shots are real.
- Site DNA: real colour swatches, type specimens, spacing grid pulled out of a page.
- Motion mechanics: 76 named mechanics (odometer headline, seam-split, receipt printer, ticket tear…).
- Browser QA: Chromium at 1440 / 768 / 375, findings list, "fix before done".
- Agents: Claude Code, Cursor, Codex, OpenCode, Windsurf, Antigravity.

## Motion intent (high, purposeful)
- Hero: the generic page physically *gets rebuilt* on scroll — scroll-scrubbed, pinned, frame by frame
  (deconstruct → extract DNA → apply motion → verify stamp). Must be readable at every frame.
- Section transitions that hand objects across sections (swatches fly from DNA into the built page).
- Kinetic type used for 1–2 headlines only, not everywhere.
- Every motion: reduced-motion fallback with all content visible; cleanup on navigation; content
  never hidden behind uninitialised JS; LCP ≤ 2.5s lab (LCP element must be plain HTML text).

## Hard rules
- Only verified repo facts. No fake metrics, testimonials, logos or stars.
- Palette and fonts: open — let the directions decide (v1's navy/lime is not mandatory).
- Stack: Astro + React islands where interactive; motion library chosen per direction.
- Pages: Home, Features, Showcase, Getting started, Docs, Open source.
- WCAG 2.2 AA, CTA in first screen at 1920–320.

## Process (local MCP, fixed build)
1. get_design_system (new mode) → 3 materially different directions, avoid_recent respected.
2. deal_mechanics with fit filter → pick per direction.
3. Rendered sketch of each direction → pick one → build.
4. capture_screenshots (compact) every round; judge by screenshots after motion settles.
5. ≥3 ROUND lines + fresh-eyes REVIEWER in direction.md; verify gate passes.
