# Saltmarsh (seafood restaurant) — round rr1 results

Blind, 1 opus judge, 35 static frames. Weights: Design 0.4, Usability 0.3, Creativity 0.2, Content 0.1. Part B not in total. Judged 2026-10-16.

| Rank | Arm | Letter | Total | Static | Colour | Type | Mobile | Polish |
|---|---|---|---|---|---|---|---|---|
| 1 | **layoza** | P | **8.20** | 8 | 7.5 | **8.5** | **7.5** | 7.5 |
| 2 | impeccable | T | 8.15 | **8.5** | **8** | 8 | 7 | 7 |
| 3 | ui-ux-pro-max | S | 7.20 | 7 | 7.5 | 7 | 7.5 | **8** |
| 4 | baseline | Q | 6.75 | 6.5 | 7 | 7 | 6.5 | 6.5 |
| 5 | frontend-design | R | 6.65 | 6 | 5.5 | 7 | 6.5 | 6 |
| 6 | taste-skill | U | 6.20 | 6 | 6 | 6.5 | 5 | 5 |

Margin over 2nd: 0.05 — a tie within judge noise, not a clear win. Motion: no evidence (stills).

## Layoza flaws (judge first)
- Section colours (pink / yellow / bright blue) swing too hard — noisy (desktop-scroll-1, -2). Lost Colour to impeccable.
- Mobile: hero art drops below the fold; header wordmark repeats right above the giant wordmark.
- Booking time slots hidden until a date is picked.
- Build agent: empty navy booking aside on desktop, left-heavy press section; get_design_system output generic/off-brief.

## Layoza strengths
- Hero shows the trade (fish crate, crab, mackerel, landing docket); stencil type; boat registrations through copy.
- Mondays struck out, Net Loft floor plan, crate-tag voucher, nautical-chart map. Content 9.

## Lab QA (3× median)
| Arm | LCP ms | CLS | QA errors |
|---|---|---|---|
| layoza | **408** | 0.009 | **0** (0 warnings) |
| ui-ux-pro-max | 1828 | 0.043 | 0 |
| impeccable | 1872 | 0.003 | focus-obscured mobile+tablet |
| frontend-design | 1920 | 0.041 | focus-obscured mobile |
| baseline | 1876 | 0.018 | focus-obscured mobile |
| taste-skill | 3112 | 0.003 | 0 |

Key: `{"layoza":"P","ui-ux-pro-max":"S","impeccable":"T","frontend-design":"R","baseline":"Q","taste-skill":"U"}`

## Round rr3 — Layoza rebuilt blind as `layoza-v4` (2026-10-16)

Rivals unchanged. Fresh blind opus judge, 35 frames. Key: layoza-v4 U, impeccable T, baseline R, ui-ux-pro-max P, frontend-design Q, taste-skill S.

| Rank | Arm | Design | Usability | Creativity | Content | Total |
|---|---|---|---|---|---|---|
| 1 | **layoza-v4** | 8.5 | 8.5 | 8.5 | 9.0 | **8.55** |
| 2 | impeccable | 7.0 | 7.0 | 8.5 | 8.5 | 7.45 |
| 3 | baseline | 7.5 | 7.5 | 6.0 | 8.5 | 7.30 |
| 4 | ui-ux-pro-max | 7.0 | 8.0 | 5.5 | 9.0 | 7.20 |
| 5 | frontend-design | 6.0 | 7.0 | 6.5 | 8.5 | 6.65 |
| 6 | taste-skill | 6.0 | 5.5 | 7.0 | 8.0 | 6.25 |

Margin +1.10 (clear win). Layoza won imagery, colour, type, mobile and layout; polish tied with ui-ux-pro-max. Layoza flaws: ~200px empty dark band on the mobile hero with the art starting below the fold, stencil display face over-used, cropped "FRESH FISH" watermark, footer wordmark flush to the right edge. Lab: 0 QA errors/warnings, LCP 1768 ms, CLS 0.007. (`cart_works`/`faq_opens` functional checks fail for every arm; they are store checks, not applicable.)

## Round rr4 — Layoza rebuilt blind as `layoza-v5` (2026-10-17)

Rivals unchanged. Fresh blind opus judge, 35 frames, read only BRIEF.md and `blind-rr4` (judge note: could not zoom image files; judged at display size). Key: layoza-v5 S, impeccable P, ui-ux-pro-max R, baseline U, frontend-design Q, taste-skill T.

| Rank | Arm | Design | Usability | Creativity | Content | Total |
|---|---|---|---|---|---|---|
| 1 | **layoza-v5** | 8.5 | 8.5 | 8.5 | 9.0 | **8.55** |
| 2 | impeccable | 7.0 | 8.5 | 9.0 | 9.0 | 7.75 |
| 3 | ui-ux-pro-max | 7.5 | 8.0 | 6.0 | 9.0 | 7.50 |
| 4 | baseline | 7.0 | 8.0 | 5.5 | 8.5 | 7.15 |
| 5 | frontend-design | 6.0 | 7.5 | 6.5 | 8.5 | 6.80 |
| 6 | taste-skill | 5.5 | 6.5 | 6.0 | 8.0 | 6.15 |

Margin +0.80 (clear win). Layoza won imagery, colour, type, mobile and polish (layout tied with impeccable). Same total as v4 in rr3 (8.55), margin smaller (+1.10 then).
Layoza flaws: dead dark band under the menu card on desktop; past calendar dates look nearly the same as open ones; on the first 375 screen the art sits fully below the copy, so it reads as text on black. Impeccable won creativity (9.0 vs 8.5).
Lab: 0 QA errors/warnings, min contrast 5.17, LCP 1660 ms, CLS 0.0006. Functional probe `valid_submit_success` failed (generic probe; the build agent's own booking check passed — unconfirmed which is right).
