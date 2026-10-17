# Round 5 — Ledgerline B2B accounting software (open style)

Blind judge, labels hidden, key revealed after scoring. Weights: design 0.4, usability 0.3, creativity 0.2, content 0.1.

| Rank | Tool | Label | Design | Usability | Creativity | Content | Weighted | QA errors | Functional |
|---|---|---|---|---|---|---|---|---|---|
| 1 | **Layoza** | T | 8.5 | 7.5 | 9.0 | 9.0 | **8.35** | 0 | all pass |
| 2 | impeccable | Q | 8.0 | 7.5 | 8.5 | 8.5 | 8.00 | 0 (1 warning) | all pass |
| 3 | frontend-design | S | 7.5 | 8.0 | 8.0 | 9.0 | 7.90 | 3 (focus-obscured) | all pass |
| 4 | baseline | P | 7.5 | 8.0 | 6.5 | 8.5 | 7.55 | 3 (focus-obscured) | all pass |
| 5 | ui-ux-pro-max | R | 7.0 | 8.0 | 5.0 | 8.0 | 7.00 | 3 (focus-obscured) | all pass |

Judge on the winner: the hero is the nine-box VAT return fed by bank lines tagged with the boxes they fill, and the form language runs through every section.

Layoza weaknesses named by the judge — note usability is its lowest score (7.5):
- Billing toggle built as two checkboxes ("Pay monthly / Pay yearly"): wrong control, should be a radio or segmented control.
- Hero overlapping cards cramped; the bank card clips the return.
- On mobile the nine-box labels wrap to three lines.
- Red rules on almost every element, close to too much.

Lab LCP: Layoza 588 ms (non-blocking font load), others 1.3–1.4 s. Basket check does not apply to this brief.
