# Ledgerline — design plan (frontend-design skill, pass 1 + review)

## Subject
Brief defines the subject clearly (bank-feed reconciliation + MTD VAT prep for UK practices, 3–50 staff;
buyers = practice owners/managers). No client confirmation needed; decisions below are mine.

## Draft 1 (rejected parts)
- Draft: cream paper, serif display, navy accent, broadsheet columns. -> Reads as default trait #1/#3. Rejected.
- Draft: dark dashboard hero screenshot with neon green "matched" chips. -> Default trait #2. Rejected.
- Draft: features as 6 identical rounded cards with icons. -> Default trait #4. Rejected.

## Revised plan
Concept: "The tick and the query". Accountants reconcile by ticking a line in pen when it agrees
to a document, and circling it with a query when it doesn't. The page is built from that ritual:
analysis-pad green paper, biro-blue ticks for matches, red-pen query rings for exceptions,
and the HMRC 9-box VAT return as the endpoint.

Color
- Pad green (bg)      #E9F0EA
- Ruling green        #B5CBBB (column rules, borders)
- Ledger ink (text)   #12352B
- Biro blue (action, ticks) #2340A8
- Query red (flags)   #B3261E
- Sheet white (surfaces) #FBFDFB

Type
- Schibsted Grotesk: everything (headings 700/800 tight, body 400). Tabular figures for money.
- Kalam: ONLY handwritten pen annotations inside the reconciliation sheet / estimator working.

Layout (left aligned throughout)
```
[wordmark] Product How it works Pricing Security Customers   Sign in [Start free trial]
H1 (left, 2/3 width)                         |
support + CTAs                                |
+------------------------------------------------------------------+
| BANK FEED        | tick |  MATCHED TO              |  VAT box    |
| 03 Jun  Stripe   |  ✓   |  INV-0412 ...            |             |
| 04 Jun  ???      |  (?) |  query: no receipt       |             |
+------------------------------------------------------------------+
| Box1 VAT on sales | Box4 VAT reclaimed | Box5 net VAT due  [Ready to submit] |
```
Steps: 4 real sequential steps on one horizontal ruled track (numbered because it IS a sequence).
Features: two-column ruled list (title / line) like analysis-pad rows — not cards.
Estimator: inputs left; right shows the working like a pen calculation + big hours result.
Pricing: three columns, middle one sheet-white with blue rule; toggle monthly/yearly.
Security: three plain statements in prose blocks.
Quotes: three, set as pull quotes with firm + town.
FAQ: accordion buttons with aria-expanded.
Signup: form on sheet-white panel.

Principles
- Boldness spent on one thing: the reconciliation sheet hero with tick/query animation (one load sequence).
- Everything else quiet: no eyebrows, no caps labels, no arrows on buttons, no mono labels.
- Money in tabular numerals, GBP, UK dates.

## Build log / self-critique
- Pass 1 screenshot: hero headline left, empty right half, sheet pushed below fold -> split hero (h1 left, lede+CTAs right), sheet now visible at 1440x900.
- Schibsted Grotesk tabular figures render with wide, odd spacing -> dropped tnum, kept lining nums + nowrap.
- Query ring clipped by global svg max-width -> max-width:none on .ring.
- FAQ: button lives inside h3 so sibling selector failed -> JS toggles .open on the panel; closed panels visibility:hidden.
- Removed middle-dot meta strings (skill's template-chrome tell).
- VAT check: Box1 490+860=1,350.00; Box4 102+13.07=115.07; Box5 1,234.93.
- Playwright: no horizontal scroll at 1440/768/375, no console errors; estimator, yearly toggle, FAQ, form validation + success, mobile menu all exercised.
- Decisions taken in place of client confirmation: plan names/prices (£120/£290/£560, 60/200/600 clients), fictional firms/address, 30-day trial, ".example" emails.
