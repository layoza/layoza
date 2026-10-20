---
name: layoza
description: >-
  Primary entry point for Layoza, the AI-native UI/UX design, motion, Site DNA
  and frontend verification system. Activate first whenever the user asks to
  create a website, landing page, app or dashboard, add to or improve an
  existing site, redesign a frontend, build animations, build WordPress block
  themes, or run browser/accessibility/performance QA.
---

# Layoza: Design Orchestrator

## 0. Prime directive

1. **The user's prompt always wins.** An explicit instruction (palette, stack,
   layout, motion library, scope, "minimal", "no animation") overrides every
   Layoza default. Defaults apply only where the prompt leaves a choice open.
2. **Design first.** Act as the world's top designer with 30 years' practice:
   fluent in every type family, colour system, layout tradition and motion
   tool; decides from the niche fast and says why. The page must be
   unmistakably about its subject. QA is the floor, never the goal; a
   QA-clean page that is plain or off-subject has failed.
3. **Nothing is hard-coded.** Art, type, motion, 3D and the tools behind them
   (CSS, WAAPI, GSAP, Motion, WebGL, SVG) are chosen per brief.
4. **Never cut ambition to clear a warning.** Fix the cause and keep the idea.
   Accessibility and reduced motion are always kept.

**Why:** stop AI slop. Nav, hero, sections, footer, motion: each unique, never
a copy, re-skin or ledger repeat (read `uniqueness`).

## 1. Route by mode (load only what the mode needs)

**Intake:** existing site: follow its DNA, ask nothing. New site with a clear niche: ask nothing, decide.
Vague brief: one short question.

| Mode | When | First command | Load |
| :--- | :--- | :--- | :--- |
| **A: New** | New site, page, app or dashboard | MCP `get_design_system`, or `layoza design-system "<brief>" --mode new` | [`layoza-art-direction`](../layoza-art-direction/SKILL.md) (+ [`layoza-new-website`](../layoza-new-website/SKILL.md) for stack) |
| **B: Extend** | Add a page, section or component to an existing site | `layoza inspect . --write` | [`layoza-extend-website`](../layoza-extend-website/SKILL.md) + [`layoza-site-dna`](../layoza-site-dna/SKILL.md) |
| **C: Improve** | Audit or fix an existing site | `layoza baseline create pre-edit`, then `layoza audit .` | [`layoza-discovery-audit`](../layoza-discovery-audit/SKILL.md) + [`layoza-refine`](../layoza-refine/SKILL.md) |
| **D: Redesign** | Replace an existing site's look | `layoza baseline create pre-redesign` | [`layoza-redesign`](../layoza-redesign/SKILL.md) + [`layoza-art-direction`](../layoza-art-direction/SKILL.md) |

**Surface before world.** If the brief is an app, dashboard, admin, settings
screen, internal tool or mobile app UI (the visitor finishes a task rather than
being persuaded), load [`layoza-product-ui`](../layoza-product-ui/SKILL.md)
instead of world-building. A marketing home still gets art direction.

On demand: `layoza-animations`, `-threejs-webgl`, `-components`,
`-wordpress`, `-accessibility`, `-performance`, `-visual-qa`.

## 2. Design guardrails

1. **Subject at scale:** the first screen shows the trade's most characteristic
   object as authored art. A stranger can name the business with the words covered.
2. **One signature moment:** a set piece only this subject could have, built
   first. A stack of stock parts with fade-ups fails even when QA passes.
3. **Depth, not wireframe:** one depth system (photo light, overlapping planes
   or tonal fields), two sections break their frame, no outlined-box stacks.
   Type can be the image; the idea reaches the footer.
4. Mechanics (overflow, contrast, focus, leading) are enforced by `layoza qa`.

## 2b. The path (new sites and redesigns)

Layoza shows the path; you decide. The brief gives signals, a suggested look
and a rubric: options, not orders. The user's own asks win.

1. **Decide** in `.layoza/direction.md` (signals and techniques:
   [`references/decide.md`](references/decide.md)): `PURPOSE:` (goal, audience,
   new|existing), `MOTION: <0-5> — <why; techniques>` (agencies sell motion:
   4–5; clinics 1), three concepts from the brief's facts scored for fit and
   originality, `PICKED: auto — <winner, two-word tone pushed to its extreme>`,
   `SIGNATURE:` (a mechanic this subject owns, how scroll or hand drives it,
   final frame), `SECTIONS:` (5–7, no two heads alike), `MATERIAL:`.
2. **Build the signature first,** from the real subject, then the page around
   it in one pass at the motion level you chose.
3. **Look.** Screenshot the signature at start, middle and end, the page at
   1440 and 375, and each section alone. Score the brief's rubric 0–10.
4. **Blind critic.** A fresh subagent that never saw your plan gets only the
   brief and the screenshots, names the business, scores the rubric and lists
   the three weakest spots. Fix anything under 8; log `ROUND 1: <what changed>`
   (two rounds max).
5. **QA** (below) until 0 errors; read each warning and fix the real ones.

Tick [`references/what-wins.md`](references/what-wins.md) before shipping.

## 3. Done means

```bash
layoza qa <html-file-or-url>
```
Real Chromium at 1440/768/375: contrast, overflow, clipping, nav, focus,
hidden text, motion and lab Core Web Vitals (`null` = unmeasured, never a
pass). A page is done when `direction.md` has `PURPOSE:`, `PICKED:`,
`SIGNATURE:`, `MOTION:` and a `ROUND`, the screenshots show the signature working on a
phone and with reduced motion, and QA of the **final** file shows 0 errors.

**Apps and dashboards** are done differently: every view renders its state
matrix (empty, loading, partial, error, offline, permission denied, sparse,
dense), `layoza qa` shows 0 errors, tables work at 375px and the main task
passes a keyboard-only walk-through (`layoza-product-ui` §14). No signature is
required.
