# Loom & Leaf (online store) — round rs1 results

Blind, 1 opus judge, 26 static frames (desktop + mobile). Weights: Design 0.4, Usability 0.3, Creativity 0.2, Content 0.1. Part B areas not in total. Judged 2026-10-16.

| Rank | Arm | Letter | Total | Static | Colour | Type | Mobile | Polish |
|---|---|---|---|---|---|---|---|---|
| 1 | impeccable | Q | **8.10** | 8 | 8 | 8 | 7.5 | 7.5 |
| 2 | **layoza** | R | 7.75 | 8 | 8 | **8.5** | 7 | 7.5 |
| 3 | baseline | U | 7.25 | 7.5 | 7 | 7.5 | **8** | 7.5 |
| 4 | frontend-design | T | 7.10 | 7 | 7 | 8 | 7.5 | 7 |
| 5 | ui-ux-pro-max | S | 6.95 | 6.5 | 7 | 6 | 7 | 7 |
| 6 | taste-skill | P | 6.50 | 6.5 | 7.5 | 7 | 5.5 | 5.5 |

Motion: low evidence for every arm (static captures only).

## Layoza flaws (judge + QA)
- Product cards show no visible add-to-basket / quick-view control — shopping path unclear (Usability 7 vs winner 8). Biggest loss.
- Mobile: "Shop the cloth" CTA below the fold. QA warned `primary-action-below-fold`; the build agent shipped it anyway.
- Newsletter section mostly empty space.
- QA (my rerun): `focus-obscured` errors at mobile + tablet; warnings label-wrap, svg-text-illegible. Build agent reported 0 errors — mismatch.
- Layoza won Typography only; tied Colour/Static with winner.

## Layoza strengths
- Mill hero with border colour toggle; labelled loom diagram + warp/weave/wash/hem table judged best craft storytelling.

## Perf (lab, 3× QA median)
All arms LCP 1.78–1.88s. ui-ux-pro-max CLS 0.21 (fail); others ≤ 0.07.

## Notes
- `functional.py` cart/faq probes failed on all arms — selectors are generic, not a real defect; build agent's own ftest passed 40/40 on every arm.
- Key: `{"layoza":"R","ui-ux-pro-max":"S","impeccable":"Q","frontend-design":"T","baseline":"U","taste-skill":"P"}`

## Round rs2 — Layoza rebuilt blind as `layoza-v3` (2026-10-16)

Rivals unchanged. Fresh blind opus judge. Key: layoza-v3 S, impeccable P, ui-ux-pro-max Q, frontend-design R, baseline T, taste-skill U.

| Rank | Arm | Total |
|---|---|---|
| 1 | **layoza-v3** | **8.05** |
| 2 | impeccable | 7.65 |
| 3 | ui-ux-pro-max | 7.30 |
| 4 | frontend-design | 7.15 |
| 5 | baseline | 6.75 |
| 6 | taste-skill | 6.15 |

Margin +0.40 (clear win). Layoza flaws: Herringbone tag's button below the fold, one mostly empty dark band, mobile category chips clip "Living", no scroll cue on the hero carousel. Lab: 0 QA errors/warnings.
