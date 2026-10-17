# Ledgerline - ui-ux-pro-max arm notes

Queries run (from skill repo root):
- "B2B SaaS accounting fintech trustworthy" --design-system -p Ledgerline --variance 5 --motion 3 --density 5
  -> Pattern Hero+Features+CTA; Style Glassmorphism (light); Primary #2563EB, Accent/CTA #EA580C (on-accent black),
     bg #F8FAFC, fg #1E293B, muted-fg #475569, border #E2E8F0; Plus Jakarta Sans; subtle scroll reveal; avoid dark-by-default.
- "fintech accounting professional" --domain typography -> IBM Plex Sans "Financial Trust" (used Plex Mono for figures)
- "SaaS pricing calculator trust" --domain landing -> trust-authority-conversion, pricing-focused (show annual savings transparently)
- "accounting bookkeeping software" --domain color -> 0 results; retried "fintech finance B2B" -> B2B Service navy #0F172A + #0369A1
- "inline validation error message" --domain ux -> error summary w/ focus, blur validation, aria-describedby
- "accordion keyboard expand" --domain ux -> keyboard nav + skip link

Synthesis: light page, navy (#0F172A) hero band so glass cards read, trust blue primary, orange CTA only.
Glass restricted to header + hero product panel (not body text) for contrast.
Signature: hero "reconciliation board" - bank lines matched to invoices/receipts, one flagged, VAT box summary.
Motion: vanilla IntersectionObserver fade (12px), disabled under reduced motion, content visible without JS. No GSAP needed.

## Verification (Playwright Chromium, qa.py + sections.py)
- No horizontal overflow at 1440/768/375; no page JS errors.
- Estimator 100 clients x 90 tx -> 100 h. Yearly toggle -> 65.83 / 157.50 / 332.50 per month, "billed £790 yearly (save £158)".
- FAQ toggles aria-expanded; empty submit -> 4-item focused error summary + inline errors; valid submit -> success panel.
- Mobile menu opens (aria-expanded=true), Escape closes.
Fixes after first pass: mobile board truncation, header translucency, popular-plan badge wording (no usage claim), 2dp yearly prices, error summary hidden under sticky header.
