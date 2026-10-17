# Tallyhouse (bakery ops dashboard) — round rd1 results

Blind, 1 opus judge, 24 static frames. Weights: Design 0.4, Usability 0.3, Creativity 0.2, Content 0.1. Part B not in total. Judged 2026-10-16.

| Rank | Arm | Letter | Total | Static | Colour | Type | Mobile | Polish |
|---|---|---|---|---|---|---|---|---|
| 1 | impeccable | U | **7.50** | 7.5 | 7.5 | 7 | 7.5 | 7.5 |
| 2 | **layoza** | R | 7.40 | 7.5 | **8** | 7.5 | 7.5 | 7 |
| 3 | frontend-design | T | 7.35 | 7 | 7 | 7.5 | 7 | 6.5 |
| 4 | ui-ux-pro-max | P | 6.90 | 7 | 7 | 6.5 | 6.5 | 7.5 |
| 5 | taste-skill | S | 6.45 | 6 | 8 | 8 | 6 | 5.5 |
| 6 | baseline | Q | 6.00 | 6 | 6 | 5.5 | 5 | 6.5 |

Top three within 0.15 — judge calls it near-tie. Motion: low evidence for all (stills). Pending-orders, loading/error states and drawer not visible in stills.

## Layoza flaws (judge)
- Headline KPI is 7-day sales, brief asks for **today's** sales. Brief miss — cost Usability/Content.
- Items sold and sales change both show "+0.2%" — looks fake.
- Mobile rota headers clipped ("TUE 13 O"). QA heading-break / clip checks did not catch it (table header cells, not headings).

## Layoza strengths
- Strongest brand feel without hurting function (warm palette, serif heads, "Review 15 low-stock items" CTA). Best Colour.
- Role-coloured rota with legend + today column; mobile stock cards keep Reorder visible.

## Lab QA (3× median)
| Arm | LCP ms | CLS | QA errors |
|---|---|---|---|
| layoza | **540** | 0.052 | **0** |
| ui-ux-pro-max | 2252 | 0.080 | text-overlap ×3 vp |
| impeccable | 2124 | 0.082 | text-overlap ×3 vp |
| frontend-design | 1716 | 0.040 | text-overlap ×3 vp |
| baseline | 1696 | **0.712** | focus-obscured, menu-toggle-broken |
| taste-skill | 3144 | 0.054 | text-overlap ×3 vp |

Rival text-overlap errors not manually confirmed — may be chart-label false positives.
`functional.py` (store/brand probes) does not fit a dashboard and was skipped; build agent's Playwright checks passed on every arm.

Key: `{"layoza":"R","ui-ux-pro-max":"P","impeccable":"U","frontend-design":"T","baseline":"Q","taste-skill":"S"}`

## Round rd3 — Layoza rebuilt blind as `layoza-v4` (2026-10-16)

Rivals unchanged. Fresh blind opus judge, 24 frames. Key: layoza-v4 U, frontend-design Q, taste-skill R, impeccable S, ui-ux-pro-max T, baseline P.

| Rank | Arm | Design | Usability | Creativity | Content | Total |
|---|---|---|---|---|---|---|
| 1 | **layoza-v4** | 8.5 | 7.8 | 9.0 | 9.0 | **8.44** |
| 2 | frontend-design | 7.8 | 8.3 | 6.5 | 8.5 | 7.76 |
| 3 | taste-skill | 7.5 | 5.5 | 8.5 | 6.5 | 7.00 |
| 4 | impeccable | 7.2 | 7.8 | 5.5 | 6.5 | 6.97 |
| 5 | ui-ux-pro-max | 6.8 | 6.5 | 6.0 | 7.5 | 6.62 |
| 6 | baseline | 6.5 | 6.0 | 3.5 | 7.5 | 5.85 |

Margin +0.68 (clear win). Layoza won data viz, colour, type; frontend-design won mobile and polish. Judge-verified: every Layoza total adds up. Layoza flaws: sync pills wrap into two ragged rows, change KPI pushed below a tall receipt on mobile, half-empty right column beside Running low, low-contrast small mono receipt text. Lab: 0 QA errors/warnings, LCP 1152 ms, CLS 0.02.

A first rd3 blind set wrongly captured the old `layoza` folder (operator error); it was voided and archived.

## Round rd4 — Layoza rebuilt blind as `layoza-v5` (2026-10-17)

Rivals unchanged. Fresh blind opus judge, 24 frames, read only BRIEF.md and `blind-rd4`. Key: layoza-v5 U, frontend-design R, impeccable T, taste-skill Q, ui-ux-pro-max S, baseline P.

| Rank | Arm | Design | Usability | Creativity | Content | Total |
|---|---|---|---|---|---|---|
| 1 | **layoza-v5** | 8.0 | 8.0 | 8.5 | 9.0 | **8.20** |
| 2 | frontend-design | 7.5 | 7.5 | 6.0 | 8.0 | 7.25 |
| 3 | impeccable | 7.0 | 7.5 | 6.0 | 6.5 | 6.90 |
| 4 | taste-skill | 6.5 | 5.0 | 8.5 | 7.0 | 6.50 |
| 5 | ui-ux-pro-max | 6.5 | 6.0 | 5.5 | 7.0 | 6.20 |
| 6 | baseline | 6.0 | 5.5 | 3.5 | 6.5 | 5.40 |

Margin +0.95 (clear win). Layoza won data viz, layout, colour, type and polish; frontend-design won mobile. Judge-verified: receipt, shops and hourly bars all add to £3,389.44; low-stock count matches its rule; only entry with a visible error state.
Layoza flaws: shop switcher scrolls sideways on mobile with only a fade hint; rota day chips clip "Su 18" at 375; "+1.4%" on Items sold has no comparison label; demo clock 14:02 against an early-morning brief.
v5 total (8.20) is lower than v4's rd3 total (8.44) but with a wider margin; different judges, so the two are not directly comparable. Lab: 0 QA errors/warnings, LCP 1656 ms, CLS 0.004.
