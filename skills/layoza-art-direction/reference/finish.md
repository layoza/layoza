<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/new-work.md (section 7), reference/polish.md and reference/critique.md. Modified by Layoza: the finish gate is `layoza qa` (measured Chromium at 1440/768/375) instead of the impeccable detector, comp-diff, and build-phase state machine; critique condensed to a design-specificity verdict, Nielsen scoring with n/a renormalisation, P0-P3 severities, and persona walk-throughs; snapshot persistence and impeccable sub-agent names removed; added anti-gaming rules. -->

# Finish: measured, then looked at

## 1. Valid evidence first

- Settle entrance motion before judging: an element hidden by animation timing
  reads as missing and gets "fixed" into a regression. (`layoza qa` scrolls the
  page like a visitor and reports text that is still hidden after scroll.)
- Run:

  ```bash
  layoza qa <file-or-url>
  ```

  Output: `qa/browser-qa-report.json` plus `qa/desktop.png`, `qa/tablet.png`,
  `qa/mobile.png`, `qa/mobile-hero.png`, and `qa/*-menu-open.png` beside the file.
- Open every screenshot once and confirm it shows what its name claims: no
  blank regions, no half-loaded state, no wrong section. A malformed capture
  binds nothing; recapture.
- Never judge fidelity from one full-page thumbnail. Look at the first viewport
  at each width and at the open mobile menu.

## 2. Fix every error, read every warning

| Rule | Usual cause | Real fix |
| :--- | :--- | :--- |
| `focus-obscured` | sticky header covers focused element | `html { scroll-padding-top: calc(var(--header-h) + 1rem) }` |
| `text-hidden-after-scroll` | reveal animation never fires or starts at `opacity:0` | visible default; animate from visible; reduced-motion path |
| `document-overflow` / `text-clip-or-bleed` | fixed widths, long words, `100vw` inside scrollbars | fluid widths, `min()`/`clamp()`, `overflow-wrap:anywhere` on user text |
| `label-wrap` / `heading-crowding` | nav or button labels too long for the column | shorten, `white-space:nowrap` on short labels, re-grid |
| `nav-unreachable` | desktop nav hidden with no mobile menu | a real toggle with `aria-expanded`, focus management, Esc |
| `target-size` / `interactive-overlap` | icon buttons < 24px with neighbours close | enlarge hit area (padding), separate targets |
| `unlabeled-control` | icon-only button, input without label | visible label or `aria-label` naming the action |
| `cramped-leading` | display line height < 1.0 | `line-height ≥ 1.0` on multi-line display |
| CLS > 0.1 | font swap, images without dimensions | metric-adjusted fallback `@font-face`; `width`/`height` or `aspect-ratio` |
| LCP > 2.5s | render-blocking fonts/CSS, huge hero asset | preload the one hero face, inline critical CSS, size images |
| `zoom-blocked` (error) | `user-scalable=no` / `maximum-scale=1` in the viewport meta | `width=device-width, initial-scale=1` only |
| `ios-input-zoom` | form fields under 16px on phones | `font-size: max(16px, 1rem)` on inputs, selects, textareas |
| `pointer-not-button` | `div` with `cursor:pointer` and a click handler | real `<button>` or `<a href>` |
| `field-autocomplete` | personal-data field without `autocomplete` / right `type` | `autocomplete="email"`, `type="email"`, `inputmode` |
| `dark-color-scheme` / `image-no-size` / `transition-all` | platform defaults left alone | `color-scheme: dark`; `width`/`height`; list transitioned properties |

**Never game a checker.** Do not hide content from the scan, set
`opacity: 0.01`, shrink text below legibility to dodge overflow, remove a real
feature, or special-case the QA user agent. A rule you believe is wrong for this
page: leave it failing, say why in the report, and let the user decide.
`null` metrics are "not measured", never "passed".

## 3. Art loop (design director pass; at least 3 rounds)

`CRAFT-*` notes in the report are non-blocking critique: each one names a
pattern that generated pages share. Fix the ones your direction didn't choose on
purpose. For a deliberate one, say why (or declare it with `layoza-brand-allow`
when it is brand DNA).

Judge the screenshots before re-reading code. Each round, run these passes:

1. **Subject test:** cover the words on the first screen. Can a stranger name
   the business from the picture alone? If not, nothing else matters yet.
2. **One-second test:** after one second on the first screen, what do you
   remember? It should be the signature, not a mood.
3. **Squint test:** blur your eyes. Is there one dominant mass and a clear
   reading order, or several things shouting at the same scale?
4. **Cover-the-logo / sibling test:** could a competitor in the same category
   ship this page unchanged? Could someone guess it from the category, or from
   category-plus-avoidance?
5. **TYPE:** does the display lettering have the character the world needs
   (width, weight, contrast, terminals), at architectural scale?
6. **MATERIAL:** is everything that should look like a material (glass, paper,
   enamel, metal) rendered honestly, with no CSS bevels or fake effects?
   Imitation material is the most reliable mark of machine-made design.
7. **GROUND:** is the page field the colour OWN-WORLD names? Hunt for drift
   toward the default (warm cream on light pages, blue-black slate on dark).
8. **Ceiling:** which of the world's native devices are still unused (frame,
   depth, lettering treatment, ornament, motion)? Name one to add.
9. **Sameness:** do consecutive sections share a composition? Each section's
   form should come from its content.
10. **Phone composition:** is the 375px first screen designed (signature visible,
    action reachable) or just stacked?
11. **Motion (mid-scroll capture):** does the signature motion read? Is
    anything blank, late, or repeated identically? Does every frame caught
    mid-animation still look designed?
12. **Prompt fidelity:** list every style word in the prompt (quiet, bold,
    animated, 3D, minimal, playful). Does the page do each one at full
    strength, and nothing the prompt ruled out?
13. **Substance:** are any desktop screens mostly empty ground? Does every
    section carry its full content at full density?
14. **Whole-page idea:** do pricing, FAQ, form and footer still speak the
    world, or does the page turn into a template after the hero?
15. **Anchor lines:** count the left edges in each screenshot. More than 3
    reads as unaligned.

**Score** each round from 0 to 10: 0.4 × design + 0.3 × usability + 0.2 ×
creativity + 0.1 × content. Caps (they override the arithmetic):
- a QA error caps the round at 6;
- a style word from the prompt not delivered, or a house move the prompt ruled
  out (for example colour blocks on a "quiet" brief), caps the round at 7;
- a generic idea (the sibling test fails) caps creativity at 6;
- a page that turns into a template after the hero caps creativity at 7;
- two or more desktop screens of mostly empty ground cap design at 6.5;
- a headline of three or more lines at any width caps design at 7;
- no real button for the primary action on the first screen caps usability at 7;
- the wrong control for the job (checkboxes for a one-of-many choice) caps
  usability at 7;
- anything covering a headline at any scroll position caps usability at 6;
- a blank band where a reveal didn't fire, or content clipped off screen, caps
  usability at 5;
- a placeholder (grey box, lorem, "image here") caps content at 5;
- a brief section missing or cut to a few words caps content at 6;
- any self-score of 9 or more is re-scored as a neutral jury would.

**Write the critique down** in the same shape every round, so rounds compare:

```
ROUND n
Seen in one second: …
Subject named from the picture alone: yes / no
Idea (one sentence): …
Prompt style words → delivered? …
Layout word per section: hero=… / shows=… / … / footer=…
Scores: design _ · usability _ · creativity _ · content _ → _._  (caps applied: …)
Top 3 problems, each with the screenshot it appears in and the fix:
1. …
```

Name problems the way a designer would tell a colleague. For example, "the
lede starts 40px left of the headline's edge", or "the quotes are the third
split layout in a row". "Could be more polished" is not a finding.

**Ceiling pass (once the score reaches 8):** run the twelve tests in
[ceiling.md](ceiling.md). Each failed test is a named fix, and a page that
fails any of them is not a 9.

**Usability pass (every round).** In blind juries usability is where good
pages lose points, so check each item in the screenshots:
- The 375px first screen shows the signature *and* the primary button, with no
  scrolling.
- The primary action is visible in the header at every width, not only in the
  menu.
- No card, label or control overlaps another at any width, and the hero leaves
  every element its own space.
- Form fields have visible edges on their surface, and labels sit above the
  fields. The submit button is full-size and sits where the eye ends.
- Every text label, including SVG and chart labels, is at least 12px on phones;
  hide what can't be read and say it in HTML instead.
- Facts are computed, not guessed. Weekdays come from dates in code (for
  example `new Date(2027, 7, 14).toLocaleDateString("en-GB", { weekday: "short" })`),
  and so do counts, totals and opening hours. A wrong weekday is the kind of
  bug juries spot.
- The header controls (menu button, primary action) fit inside 375px with
  nothing clipped.
- No label hangs off the screen edge at 375px (`text-offscreen`).
  `overflow-x: hidden` hides the overflow, not the cut text.
- Controls match their meaning: radios or segmented controls for one-of-many
  choices, real buttons for actions, links for navigation.
- No dead strips (an empty band under the footer, the hero or a pinned stage).
- In configurators and stories, the object stays in view while it is being
  changed.

**Fix the top 3 problems** per round, in this order: idea → hierarchy →
composition → usability → type → art → colour → detail → motion.

**Stop:**
- Stop when the score is **at least 9** and the fresh-eyes reviewer
  independently agrees, or after 6 rounds.
- Under 9, keep going while each round still finds a fix worth making. A
  reviewer verdict of `ship` below 9 does **not** end the loop: apply the
  changes it names for a 9, then re-run it.
- Report the final score honestly, never rounded up.
- A page that only scores well because a check was skipped has not reached 9.

**Fresh-eyes reviewer:** when subagents are available, spawn one with no build
context. Give it the brief, `direction.md` and the screenshots, and have it run
passes 1–15 and return `ship`, `fix` (ordered fixes) or `rebuild` (what to
re-derive). A `rebuild` on MATERIAL or the subject test outranks every polish fix.

Brief the reviewer as a **blind juror at a top studio**:
- It scores with the same weights and caps.
- It is told that 9+ is rare and must be earned.
- It names the single change that would most raise the score.

Re-run it after each fix batch until it scores 9 or more, or the round budget
runs out. Its score, not the builder's, is the one you report.

**For motion and 3D pages**, capture real viewport frames while scrolling (not
one full-page screenshot, which breaks on sticky and pinned sections). Judge
each frame, including frames caught mid-animation.

For apps, dashboards and Mode C work, also walk the primary task as a
first-timer, a one-handed phone user, and a keyboard plus screen-reader user.
Full heuristics: `layoza-refine/reference/evaluate.md`.

## 4. QA fix rounds (bounded)

- Triage: broken tasks and inaccessible paths → missing states → responsive
  drift → visual inconsistencies → cleanup.
- Batch the QA fixes, rerun `layoza qa`, and re-open the screenshots. **Two QA
  rounds is the ceiling** for unattended runs. This cap is for defect fixing
  only; the art loop above has its own budget.
- Never fix QA by cutting a design idea; fix the cause and keep the idea.
- Finish with a source diff: remove debug output, dead code, orphaned styles
  and temporary files.

## 5. Report honestly

Report the verdict at its real scope: QA errors (0 or the list), warnings you
left and why, the metrics with nulls named as unmeasured, the critique's top
issues with P-levels, and what remains open. "All three fixes resolved" is a
claim a re-run supports; "no issues remain" usually is not. If the user answers
with evidence against your verdict (their screenshot, a named mismatch), it
outranks every capture you made.
