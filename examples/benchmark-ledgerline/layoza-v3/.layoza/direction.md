# Ledgerline — direction (dev only, never served)

## Inventory
Subject facts: UK accountancy practices, 3–50 staff; buyers are practice owners/managers; reads clients' bank feeds; matches transactions to invoices and receipts; flags what it can't match; prepares VAT returns; Making Tax Digital submission; quarterly VAT rhythm; human approves before filing; trial signup.
Subject objects: bank statement lines, invoices/receipts, the tick that pairs them, the flag/query to the client, **the nine-box VAT return** (circled), the submission receipt.
Scene: practice manager at a desk on a weekday, daylight, two monitors, quarter-end pressure. -> light page.

## Rut
Reflex: blue/purple SaaS gradient, bento icon cards, laptop dashboard mockup, logo cloud.
Obvious opposite: black + neon fintech terminal.
Calibration avoided: cream+serif+terracotta, broadsheet hairlines+italic serif+mono labels.

## Seven candidates (deal --top 4 -> #3)
1 Columnar analysis pad · 2 Ticked bank statement · **3 VAT100 nine-box return (dealt)** · 4 date/PAID rubber stamps · 5 lever-arch files + index tabs · 6 quarter-end wall planner · 7 cheque security printing.
Challengers: #1 analysis pad -> raise: "column courage: every figure on the page sits right-aligned in tabular columns". #2 ticked statement -> raise: "every claim is a line item": the match tick is the one micro-gesture used everywhere a thing is confirmed.

## THESIS
On this page, the nine-box VAT return becomes the hero itself: a printed return sheet whose boxes fill from a live bank feed as each line is ticked. Refuses the laptop-dashboard-mockup + icon bento arrangement.

## OWN-WORLD
Printed government-style return form (own design, no crown/HMRC branding): drop-out red ink for rules, box numbers and pre-printed text; entries in blue-black ink; pale drop-out tint inside boxes; boxes 6–9 with pre-printed ".00". Square corners, ruled 1px borders, no shadows. Recognisable with content removed: red-ruled numbered boxes.

## TOKENS
--paper #FBF8F7 (ground) · --red #B8293A (drop-out ink: rules, box numbers, primary button, red field) · --tint #F7DCDA (box interiors, tint field) · --ink #16233F (entries, body, navy field) · --ink-2 #4A5470 (secondary) · --match #1D6B4F (ticks only).
Family: Archivo variable (wdth 62–125). Display: wdth 72, wght 800. Body: wdth 100, 400/600. Figures: tabular-nums.
Scale: d1 clamp(2.75rem,7.2vw,7rem) · d2 clamp(2.25rem,4.6vw,4.25rem) · h3 1.375rem · body 1.125rem · label .8125rem.
Alignment: two anchors — container left edge and the 7/12 column line.

## FIRST VIEWPORT
Desktop:
```
[mark Ledgerline | Product How Pricing Security Customers | Sign in [Start free trial]]
+== sheet (red rules) =========================================================+
| BANK FEED IN.                              | Return period / example client  |
| VAT RETURN OUT.          (d1, ink)         | lede · [Start free trial] [How] |
+---- bank feed: 5 lines, ticks -------------+---- boxes 1..9 rows, figures ---+
| 03 Mar GRANGE CAFE    +1,440.00  ✓ INV     | [1] VAT due on sales  8,412.60  |
| ... 07 Mar flagged  ? [Request receipt]    | ... [5] Net VAT to pay 5,457.25 |
+=============================================================================+
```
Sheet = ~75% of first screen. Action: header button + sheet CTA.
Phone: header (mark, Start free trial, menu) -> headline 2 lines -> lede -> CTAs -> sheet feed rows start above the fold.

## SECTIONS
- hero: claim "feed in, return out", proof = the live sheet. Layout: sheet.
- how (Part 1–4): sequence diagram with drawn artefacts, one red route line. Layout: sequence.
- product (6 capabilities): tint field, ruled form list a–f, two columns. Layout: list.
- estimator: red field, two input boxes + result at d1 scale. Layout: statement.
- pricing: paper, three ruled form columns + segmented radio. Layout: table.
- security: navy field, statement left + three ruled items. Layout: split.
- customers: one quote at scale + two smaller. Layout: quote.
- faq: tint field, "Notes on completing" accordion. Layout: list.
- trial: paper, signup as a numbered return form with declaration. Layout: form.
- footer: navy close, wordmark full width, address, links one ruled row.

## MOTION
Hero load: rows tick in sequence (pen tick stroke draw), boxes count up, box 5 last. Once; Replay button. Visitor operates the flagged row (Request receipt -> matched, boxes 4/5/7 roll). Estimator figures roll 300ms. Pricing figures roll. How-it-works route line draws when in view (drawn by default without JS). Form: errors slide under field; success = dated "Received" stamp arrival. Accordion grid-rows 0fr->1fr. Buttons hover/press. Tool: CSS + WAAPI + rAF; no library needed. Reduced motion: final states instantly, no travel.
Subject micro-moments: the match tick, the received stamp.

## Reflex-check lines
- Box numbers 1–9 / Part 1–4 carry information (form and sequence), not decoration.
- Tabular figures are data, not costume.

FINISH: Unverified is unfinished: art loop done, `layoza qa` 0 errors.
