---
name: layoza-refine
description: >-
  Targeted refinement commands for an EXISTING page or component (Mode C):
  quieter, distill, delight, clarify, onboard, harden, adapt, optimize, audit,
  plus the type/colour/layout/motion/bolder passes and polish/critique from
  layoza-art-direction. Every command runs as a measured loop: baseline,
  `layoza qa` before, one scoped change, `layoza qa` after, regression check,
  rollback if a measured metric got worse. The site's own brand DNA wins.
---
<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/quieter.md, distill.md, delight.md, clarify.md, onboard.md, harden.md, adapt.md, optimize.md, audit.md (command ideas only; bolder/typeset/colorize/layout/animate/polish/critique are linked from layoza-art-direction, not repeated). Modified by Layoza: every command rewritten as a measured before/after loop on `layoza qa` + `layoza baseline` with automatic rollback on metric regression; each command names the `layoza qa` rules and report fields that prove it worked and the ones it must not break; SITE-DNA.json / DESIGN.md brand authority over taste bans; PRODUCT.md context gate; impeccable slash commands, detector, live mode, and AskUserQuestion plumbing removed. See LICENSE and NOTICE.md in this folder. -->

# Layoza — Refine an existing page

A refinement is **one scoped command on one target** (a page, section, or
component that already exists), proven by measurement. Taste without a
before/after number is a suggestion, not a result.

## 0. Before any command

1. **Brand authority.** Run `layoza inspect . --write` if `SITE-DNA.json` is
   missing or `layoza baseline` (freshness) says it is stale. Tokens recorded
   in `SITE-DNA.json` / `DESIGN.md` (colours, fonts, radii, spacing, motion)
   **win over every taste ban** in this skill and in `layoza-art-direction`.
   A `craft-*` finding on a recorded brand token is not a defect: declare it in
   the page head, e.g. `<meta name="layoza-brand-allow" content="font:Inter, palette:cream">`
   (suppressed findings stay listed in `craft_critique_suppressed`). Only
   declare what `SITE-DNA.json` actually records; using the meta to silence a
   finding the brand does not own is gaming the checker.
2. **Product context.** Commands marked *ctx* below need to know who the page is
   for and what the visitor came to do. Read `PRODUCT.md`; if it is missing,
   build it with [`layoza-discovery-audit` step 0](../layoza-discovery-audit/SKILL.md)
   (pre-filled from inspect output; ask only for the gaps).
3. **Surface.** Decide whether the target is Persuade, Operate, Read, or
   Experience ([surfaces.md](../layoza-art-direction/reference/surfaces.md)).
   The same command means different things on each (quieter on a dashboard =
   less noise; on a portfolio = more air around the work).
4. **Scope is sovereign.** Touch only the named target. A command that needs a
   new identity (new palette, new fonts) has become a Mode D redesign: stop and
   say so.

## 1. The measured loop (every command)

```bash
# 1. Reversible baseline of every file you will edit (one --file per id via CLI)
layoza baseline create refine-<cmd>-css  --file <path/to/styles.css>
layoza baseline create refine-<cmd>-html --file <path/to/page.html>
# 2. Before: measure, then keep a copy (qa overwrites its report)
layoza qa <page.html-or-url>
cp <dir>/qa/browser-qa-report.json <dir>/qa/before-<cmd>.json
# 3. One scoped change (the command's reference file)
# 4. After: measure again and compare
layoza qa <page.html-or-url>
# 5. Regression? restore every baseline you created
layoza baseline rollback refine-<cmd>-css
```

Over MCP the same loop is: `safe_rollback` `{action:"create", baseline_id, files:[...], write_approved:true}`
(also saves the "before" audit) → `capture_screenshots` → change →
`capture_screenshots` → `compare_before_after` `{path, baseline_id}` (separates
intended changes from regressions) → `safe_rollback` `{action:"rollback"}` if needed.

**Where the evidence is.** Rule hits live in `viewports[].issues[]` of
`qa/browser-qa-report.json` (`rule`, `severity`: `error` / `warning` /
`critique`); design rules are `craft-*` (summary in `craft_critique`). The CLI
summary prints the same hits upper-cased (`A11Y-FOCUS-OBSCURED`,
`CRAFT-KICKER-ABOVE-HEADING`). Side-by-side summary of two reports:

```bash
node -e "const fs=require('fs');for(const f of process.argv.slice(1)){const d=JSON.parse(fs.readFileSync(f,'utf8')),c={};for(const v of d.viewports)for(const i of v.issues)if(i.severity!=='critique')c[v.label+':'+i.rule]=(c[v.label+':'+i.rule]||0)+1;const k=['contrast_failures','unlabeled_controls','undersized_targets','missing_alt_count','heading_hierarchy_valid','focus_visible_styled','reduced_motion_respected','lcp_ms','cls','inp_ms','h1_to_body_ratio','max_line_length_ch','font_family_count','unmeasured'];console.log(f,JSON.stringify({issues:c,craft:d.craft_critique,...Object.fromEntries(k.map(x=>[x,d[x]]))}))}" qa/before-<cmd>.json qa/browser-qa-report.json
```

**Keep, fix, or roll back.** Compare `before-<cmd>.json` with the new report:

| Outcome | Rule |
| :--- | :--- |
| Regression (any of the list below) | Fix it in the same round, re-measure; still worse → roll back and report. Never ship a refinement that made a measured metric worse. |
| Target metric moved, nothing regressed | Keep. Open the screenshots and confirm the change is visible and on-brand. |
| Target metric did not move | The change did not do what the command claims. Re-read the finding, or report "no measurable effect" honestly. |
| Command has no metric (judgment only) | Say so; evidence is the before/after screenshots at all three widths plus the reference file's verify list. |

**A regression is:** a new `error` rule in `issues`; `contrast_failures`,
`undersized_targets`, `unlabeled_controls`, or `missing_alt_count` going up;
`heading_hierarchy_valid`, `focus_visible_styled`, `reduced_motion_respected`,
or `landmarks_present` going `true` → `false`; a lab metric crossing its
budget (`lcp_ms` > 2500, `cls` > 0.1, `inp_ms` > 200) or worsening by more than
noise (re-run `layoza qa` once; treat a change as real only if both runs agree);
a new `craft-*` finding in the area you touched. A value going from a number to
`null` is **unmeasured**, not passed: re-run, then report it as unverified.

Two fix rounds per command maximum (same ceiling as [finish.md](../layoza-art-direction/reference/finish.md)).

## 2. Commands

| Command | Use when | Proof it worked (`layoza qa`) | Must not break | Guide |
| :--- | :--- | :--- | :--- | :--- |
| `typeset` | hierarchy muddy, text hard to read | fewer `craft-tight-leading`, `cramped-leading`, `craft-flat-type-hierarchy`, `craft-line-length`, `craft-tiny-text`, `craft-undersized-ui-text`; `h1_to_body_ratio`, `max_line_length_ch` ≤ 75 | `cls` (font swap), `label-wrap`, `text-clip-or-bleed` | [system-passes § Type](../layoza-art-direction/reference/system-passes.md) |
| `colorize` | grey, colour without roles | `contrast_failures` = 0, `min_contrast_ratio` ≥ 4.5; fewer `craft-gray-on-color`, `craft-gradient-text` | brand colours in `SITE-DNA.json` | [system-passes § Colour](../layoza-art-direction/reference/system-passes.md) |
| `layout` | crowded, monotonous, misaligned | fewer `craft-monotonous-spacing`, `craft-cramped-padding`, `craft-nested-cards`, `heading-crowding` | `document-overflow`, `primary-action-below-fold` | [system-passes § Layout](../layoza-art-direction/reference/system-passes.md) |
| `animate` | motion missing or wrong | `text-hidden-after-scroll` = 0, `reduced_motion_respected` true; fewer `craft-bounce-easing` | `inp_ms` ≤ 200, `cls` | [system-passes § Motion](../layoza-art-direction/reference/system-passes.md) + [`layoza-animations`](../layoza-animations/SKILL.md) |
| `bolder` | a section reads flat | `craft-flat-type-hierarchy` cleared; `h1_to_body_ratio` up | contrast, no new `craft-*` | [system-passes § Bolder](../layoza-art-direction/reference/system-passes.md) |
| `quieter` | too loud, overstimulating | fewer `craft-dark-glow`, `craft-radial-*`, `craft-gradient-text`, `craft-pulsing-dot`, `craft-gpt-thin-border-wide-shadow` | `contrast_failures`, `craft-flat-type-hierarchy` must **not** appear | [intensity.md](reference/intensity.md) *ctx* |
| `distill` | cluttered, too many choices | fewer `craft-nested-cards`, `craft-icon-tile-stack`, `craft-repeated-container-text`, `craft-hero-eyebrow-chip`; `primary-action-below-fold` cleared; `font_family_count` ≤ before | `nav-unreachable`, every route/form still works | [intensity.md](reference/intensity.md) *ctx* |
| `delight` | correct but forgettable | judgment + screenshots; no new findings | `inp_ms`, `lcp_ms`, `text-hidden-after-scroll`, `reduced_motion_respected` | [intensity.md](reference/intensity.md) *ctx* |
| `clarify` | labels, errors, copy unclear | `unlabeled-control` = 0, `missing_alt_count` = 0; fewer `craft-marketing-buzzword`, `craft-theater-slop-phrase`, `craft-aphoristic-cadence` | `label-wrap` (longer labels wrap) | [copy-and-flow.md](reference/copy-and-flow.md) *ctx* |
| `onboard` | first use / empty states weak | primary action in first viewport (`primary-action-below-fold` cleared); walkthrough evidence | `focus-obscured`, `unlabeled-control` | [copy-and-flow.md](reference/copy-and-flow.md) *ctx* |
| `harden` | only works with perfect data | 0 `focus-obscured`, `target-size`, `unlabeled-control`, `text-hidden-after-scroll`, `interactive-overlap`, `text-clip-or-bleed` **on the stress copy** | everything above on the real page | [resilience.md](reference/resilience.md) |
| `adapt` | breaks or feels desktop-only on small screens | 0 `document-overflow`, `nav-unreachable`, `menu-toggle-broken`, `label-wrap`; `primary-action-below-fold` cleared at 375 | desktop screenshot unchanged in intent | [resilience.md](reference/resilience.md) |
| `optimize` | slow load, jank, layout shift | `lcp_ms` ≤ 2500, `cls` ≤ 0.1, `inp_ms` ≤ 200 (lab; two runs agree) | visual parity, `broken-image` | [resilience.md](reference/resilience.md) + [`layoza-performance`](../layoza-performance/SKILL.md) |
| `audit` | "what's wrong with it?" (no fixes) | scored report from `layoza audit . --json` + `layoza qa` | n/a (read-only) | [evaluate.md](reference/evaluate.md) |
| `critique` | design-quality verdict | P0–P3 list, specificity verdict | n/a (read-only) | [finish.md § 3](../layoza-art-direction/reference/finish.md) |
| `polish` | final pass after any of the above | 0 `error` issues; warnings explained | everything | [finish.md § 2–4](../layoza-art-direction/reference/finish.md) |

Typical order when the user says "improve it": `audit` → the P0/P1 commands it
names (usually `harden`, `adapt`, `clarify`, `optimize`) → at most one
intensity command (`quieter` **or** `bolder`, never both) → `polish`.

## 3. Report

Per command: target, the before → after values of its proof metrics (copied
from the two reports, `null` named as unmeasured), regressions found and how
they were resolved or rolled back, screenshots opened, and what remains open.
Lab Core Web Vitals are LaboratorySynthetic; never present them as field data.
