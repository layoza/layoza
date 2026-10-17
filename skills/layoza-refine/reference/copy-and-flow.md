<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/clarify.md and reference/onboard.md. Modified by Layoza: condensed into one copy-and-flow guide in Layoza's own words; mapped to `layoza qa` rules (unlabeled-control, missing_alt_count, label-wrap, craft-marketing-buzzword, craft-theater-slop-phrase, craft-aphoristic-cadence, primary-action-below-fold); added a string-diff evidence table, a no-invented-facts rule tied to PRODUCT.md "Evidence on hand", and a first-viewport task walk; tooltip/tour library advice and impeccable hand-offs removed. -->

# Copy and flow: clarify, onboard

Both start from `PRODUCT.md` (who the visitor is, the job they came for, voice).
Without it you are guessing tone; build it first (discovery-audit step 0).

---

## Clarify (labels, errors, states, and copy are unclear)

**Read the whole path, not isolated strings.** Walk the primary task from the
first viewport to its confirmation and list every string a visitor meets:
buttons, links, nav, form labels and hints, errors, empty/loading/success
states, alt text, accessible names.

**For each state decide, in order:** the one fact the visitor needs now; the
action available next; context that changes that decision; the tone the moment
needs (calm for errors, brief for routine success). Say each idea once.

**Rewrite rules:**
- Actions: a specific verb + object when the outcome is not obvious
  ("Book the Saturday class", not "Submit"). One word per concept across the
  site; keep `PRODUCT.md` terminology.
- Destructive actions say what will be lost; an undo beats a confirm
  dialog when recovery is possible; a confirm button repeats the action ("Delete draft"), never
  `OK` / `Yes`.
- Forms: persistent visible labels (placeholders are examples), format rules
  before submit, errors that say what to fix next to the field and are
  announced (`aria-describedby`, `aria-live` / `role="alert"`).
- Errors answer: what failed, why (if known and useful), how to recover. No raw
  codes as the message; no jokes about money, privacy, or lost work.
- Empty states distinguish first use, no results, filtered out, no permission,
  and failure, each with the next useful action.
- Links make sense out of context; icon-only controls get an accessible name
  that matches the visible outcome; alt text carries the image's information
  (empty `alt=""` for decoration).
- Write whole translatable sentences, not concatenated fragments; leave room
  for 30–40% expansion.

**No invented facts (Layoza rule).** Never add a statistic, testimonial, price,
guarantee, or customer name that is not in `PRODUCT.md` "Evidence on hand" or
the existing copy. Marketing filler is also measured: `craft-marketing-buzzword`,
`craft-theater-slop-phrase`, `craft-aphoristic-cadence`, `craft-em-dash-overuse`.

**Evidence table** (goes in the report; it is the diff a reviewer reads):

| Location (selector) | Before | After | Why |
| :--- | :--- | :--- | :--- |

**Proof:** `unlabeled-control` = 0 and `missing_alt_count` = 0 in the after
report; the craft copy rules above down; **guard** `label-wrap` and
`text-clip-or-bleed` (clearer labels are often longer and wrap at 375px).

---

## Onboard (first use and empty states)

Onboarding's job is to get the visitor to the **first proof of value** fast,
not to teach the product.

1. **Name the first value moment** from `PRODUCT.md` (first booking, first
   project created, first result seen) and the shortest real path to it.
2. **Walk it as a first-timer** (Jordan in finish.md § 3) and count steps,
   fields, and scrolls. Write the count down; it is the before metric.
3. **Cut steps:** collect only what this step needs (more later), smart
   defaults, sample data or a template so the first action works immediately,
   real functionality instead of a separate tutorial mode.
4. **Empty states are the onboarding:** what will appear here, why it matters,
   one clear action (plus a template/import option when useful).
5. **Optional and respectful:** anything instructional is skippable,
   dismissal is remembered (wrap `localStorage` in try/catch), and returning
   visitors never see first-run onboarding again. Contextual hints appear at
   the point of use, at most one at a time, and never trap focus.
6. Tours only when the surface is genuinely complex: ≤ 5 steps, on real UI,
   replayable from help.

**Proof:** the walk's step/field count went down (before → after in the
report); `primary-action-below-fold` is clear for the first-use view;
**guards** `focus-obscured`, `unlabeled-control`, `interactive-overlap` (hint
popovers overlapping controls), and `nav-unreachable` stay at 0.
