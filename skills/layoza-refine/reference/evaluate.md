<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/audit.md (and the routing idea of reference/critique.md, whose rubric Layoza already adapts in layoza-art-direction/reference/finish.md). Modified by Layoza: the five-dimension 0-4 health score is anchored to measured `layoza audit --json` / `layoza qa` fields with score caps, so a dimension cannot be rated "good" while a measured failure stands; theming is checked against SITE-DNA.json tokens and discrepancies; recommendations route to layoza-refine commands with the metric each should move; impeccable detector and slash-command lists removed. -->

# Evaluate: audit (report only)

`audit` documents; it does not fix. Its output is the work order for the other
commands. For a design-quality verdict use `critique`
([finish.md § 3](../../layoza-art-direction/reference/finish.md)); run both when
the user asks "review this".

## 1. Collect evidence

```bash
layoza inspect . --json            # SITE-DNA tokens, discrepancies, confidence.unknowns
layoza audit . --json              # static findings: A11Y-*, RESP-*, MOTION-*, CRAFT-*
layoza qa <page.html-or-url>       # live Chromium report + screenshots
```

Open the screenshots. Verify each `craft-*` / `CRAFT-*` finding in context
before reporting it (a finding on a brand token recorded in `SITE-DNA.json` is
not a defect; list it as brand-allowed).

## 2. Score five dimensions (0–4), with measured caps

Score from the evidence, then apply the cap: a dimension **cannot score above
its cap** while the measured failure stands. This is what stops an audit from
calling a page "good" over a failing number.

| Dimension | Measured inputs | Cap |
| :--- | :--- | :--- |
| Accessibility | `contrast_failures`, `unlabeled_controls`, `missing_alt_count`, `undersized_targets`, `focus-ring-missing`, `heading_hierarchy_valid`, `landmarks_present`, `focus-obscured` | any contrast failure, unlabeled control, or `focus-ring-missing` → max 2 |
| Visual composition | before/after screenshots: hierarchy, balance, typography, imagery and brief fidelity | judgment from the actual render; name the strongest and weakest design decisions |
| Theming / tokens | `SITE-DNA.json` `colors` vs values hard-coded in source; `discrepancies`; dark-mode strategy | > 5 untokenised colours in the target → max 2 |
| Responsive | `document-overflow`, `text-clip-or-bleed`, `nav-unreachable`, `menu-toggle-broken`, `label-wrap`, `primary-action-below-fold` | any error rule → max 1 |
| Implementation integrity | `craft_critique` summary (verified), repeated one-off styles, component drift vs `COMPONENT-INVENTORY.json` | — (judgment, cite evidence) |

Total /20: 18–20 excellent, 14–17 good, 10–13 acceptable, 6–9 poor, 0–5 critical.

## 3. Report

1. **Integrity verdict first:** does the implementation express one coherent,
   product-specific system, or a pile of defaults? Cite evidence.
2. **Score table** (dimension, score, cap applied?, key finding).
3. **Findings by severity** — P0 blocks a task; P1 significant difficulty or a
   WCAG AA failure; P2 annoyance with a workaround; P3 polish. Each: location
   (file:line or selector), what the visitor experiences, the rule or standard,
   the fix, and **the command + metric** that will prove it:

   | Sev | Finding | Location | Command | Metric that must move |
   | :--- | :--- | :--- | :--- | :--- |
   | P1 | icon buttons unnamed | `.cart-btn` | `clarify` | `unlabeled_controls` 3 → 0 |

4. **Systemic patterns** (same failure in many places → fix at the token or
   component level, once).
5. **What works** — practices to keep, so later commands do not "fix" them.
6. **Unmeasured** — every `null` field and everything `layoza qa` does not test.

Keep P3 noise short. Recommend commands in order: P0/P1 fixes first, at most
one intensity command, `polish` last. Re-running `audit` after the fixes is the
before/after the user sees.
