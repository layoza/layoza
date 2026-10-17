# Ledgerline — direction (dev-only, never served)

## Inventory
Subject facts: UK accountancy practices, 3–50 staff; buyers = practice owners/managers; product reads clients' bank feeds;
matches transactions to invoices + receipts; flags what it can't match; prepares VAT returns; submits via Making Tax Digital;
quarterly VAT rhythm; work today = ticking statement lines against paperwork; many clients per firm; pricing per firm by client limit.
Subject objects: bank statement line, invoice, receipt, the reconciliation tick, **the nine-box VAT return** (circled with the statement line feeding it).
Scene: practice manager at a desk in a bright open-plan office, mid-morning, quarter-end pile → light page.

## Rut
Reflex: blue/purple SaaS gradient, dashboard screenshot in a browser frame, bento icon cards, logo cloud, 3 pricing cards.
Obvious opposite: dark terminal fintech with neon accent.
Name pun (rejected): a literal ruled ledger line / railway line.

## Seven worlds -> deal
1 Green analysis pad · 2 Adding-machine tally roll · 3 Bound ledgers · 4 Nine-box VAT return as notation · 5 Bank security print ·
6 Lever-arch box files · 7 Quarter-end date-stamp ritual.  `layoza deal --top 4` -> **#4 (key 7e399f9bbce9e3b3)**. Built as dealt.

## Fusion
World (far): the official form as notation — printed in **drop-out red** (the ink scanners ignore so only the entered figures are read),
numbered boxes, comb fields for figures, guidance notes, declaration, filing receipt. No crown, no HMRC branding, no imitation of a real form's header.
Subject (near): the bank statement line, ticked against its invoice/receipt, dropping its VAT into the boxes.
IDEA: "On this page the nine-box VAT return is drawn in drop-out red, and the client's bank lines fill it in, in ink, as they're matched."
Skeleton: Direction A (7:5 asymmetric, exposed rules, type as architecture) + Direction C's operable proof (estimator in comb fields).
Challengers: #1 analysis pad (competitive; declined, near-category cliché of green ledgers) -> raise: density courage, figures in tabular columns everywhere.
#2 tally roll (declined, narrow object fights page width) -> raise: total palette commitment to two inks.

## OWN-WORLD
Drop-out red owns form furniture (rules, box frames, box numbers, the How-it-works field). Ballpoint ink-blue owns everything "entered"
(figures, headlines, the Security field). Paper is a cool white tinted faintly red, not cream. Highlighter amber only for the unmatched flag.
Components: buttons = solid ink with square-ish 3px corners; inputs = comb/box fields with red frames; toggle = a form tickbox; focus = 3px ink ring + red offset.

## TOKENS
--paper #FBF8F7 · --paper-2 #F6EDEA · --dropout #D9482B (rules/frames, large type) · --dropout-ink #B0301A (red text ≥4.5) ·
--ink #18264A (text/figures) · --ink-2 #4A5470 (secondary) · --flag #F5B83D (highlighter) · --tick #1F7A52 (matched tick, small use)
Families: Archivo (variable wdth 62–125: condensed 800 display, normal body) + IBM Plex Mono (entered figures, comb fields only).
Scale: 15 / 18 / 24 / 44 / clamp(56–112). Alignment: two left edges (gutter, column 6).

## FIRST VIEWPORT
Desktop:
```
[Ledgerline]  Product How Pricing Security Customers   Sign in [Start free trial]
+------------------------+----------------------------------------------+
| BANK FEED IN.          |  ┌ statement strip ┐  ┌ VAT return (red form)┐|
| VAT RETURN OUT.        |  | line ✓ INV-0412 |→ | 1 ▢▢▢  2 ▢  3 ▢▢▢    ||
| supporting line        |  | line ✓ RCT-88   |  | 4 ▢▢   5 ▢▢  6 ▢▢▢   ||
| [Start free trial][How]|  | line ⚑ no match |  | 7 ..  8  9           ||
|                        |  └─────────────────┘  └ ready to submit ─────┘|
+------------------------+----------------------------------------------+   art ≈ 55%
```
Phone: header + menu button; H1 2-3 lines; CTAs; then art (statement strip stacked over a compact 9-box form), first screen shows top of art.

## SECTIONS
- How it works: claim "four steps, one quarter" / proof the pipeline / composition: red field, horizontal ruled route with 4 stations.
- Capabilities: claim covers the whole job / 6 entries / composition: guidance-notes ruled two-column list with box-codes.
- Estimator: claim time back / live calc 40s per transaction / composition: comb-field form fragment + one big entered figure.
- Pricing: claim simple per-firm / 3 plans / composition: schedule table with tickbox billing toggle.
- Security: claim UK + controlled access / composition: ink-blue field, declaration with signature rule.
- Customers: claim practices like yours / 3 fictional quotes / composition: one quote at scale + two margin notes.
- FAQ: guidance notes accordion, narrow column.
- Trial: the form itself, comb fields, success = filing receipt.
- Footer: red rules, address, links, social.

## MOTION
Hero load: statement lines tick in sequence (stroke draw), box figures count up in Plex Mono, one line stays flagged in amber;
visitor can press "Attach receipt" to resolve it and watch boxes update (signature interaction). How-it-works route line draws once on view.
Micro: tickbox toggle, accordion (≤200ms), button press. CSS + WAAPI + IntersectionObserver; no library needed.
Reduced motion: final states shown, no counting/drawing. Content visible without JS (final state is the HTML default).

## Reflex-check reasons
- Monospace: used only for entered figures/comb fields (data), never as costume.
- Numbered boxes: the box numbers carry information (they are the return's structure); sections are NOT numbered.

FINISH: Unverified is unfinished: art loop done, `layoza qa` 0 errors.

## Copy deck
H1: Bank feed in. VAT return out.
Lede: Ledgerline matches every client transaction to its invoice or receipt, flags the few it can't, and prepares the return for Making Tax Digital. Your team reviews; it stops ticking.
CTAs: Start free trial (-> trial form, "Start my trial" -> "Trial started") / See how it works.
Illustrative content (hero client, figures) labelled "Illustrative client".

## Art loop log
- R1 (7.4): body font-variation-settings killed font-stretch; slip covered form labels; H1 4 lines; mono costume labels; tile numerals in route; box-number contrast 4.28. Fixed all.
- R2 (8.0, fresh-eyes reviewer "fix"): H1 orphan "VAT" (nowrap + size), phone first screen lacked the form (form now first on phone), comb needed decimal point (added), pricing figures now entered in red-framed fields, Security ends in a signed declaration with a date comb + full-width heading.
- R3 (~8.2): LCP 2.7–5s from render-blocking Google Fonts CSS -> non-blocking preload + metric fallbacks; LCP now 0.3–0.65s, CLS 0.
- Deliberate CRAFT notes kept: craft-mono-meta-labels (statement dates are data); craft-hero-art-scale reports 0% because the art is HTML/SVG notation, not an <svg>/<canvas> block; visually ~50% of the desktop first screen.
