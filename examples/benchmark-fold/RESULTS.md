# Round 6 — Fold architecture studio (style: SIMPLE, no animation)

Blind judge, labels hidden, key revealed after scoring. Weights: design 0.4, usability 0.3, creativity (within restraint) 0.2, content 0.1.

| Rank | Tool | Label | Design | Usability | Creativity | Content | Weighted | QA errors | Functional |
|---|---|---|---|---|---|---|---|---|---|
| 1 | impeccable | R | 8.3 | 7.8 | 8.5 | 9.0 | **8.26** | 0 | all pass |
| 2 | **Layoza** | P | 8.0 | 8.0 | 8.5 | 9.0 | 8.20 | 0 | all pass |
| 3 | ui-ux-pro-max | T | 8.2 | 8.3 | 7.5 | 9.0 | 8.17 | 0 (4 contrast fails, min 4.48) | all pass |
| 4 | baseline | S | 8.3 | 7.8 | 8.0 | 8.5 | 8.11 | 0 | all pass |
| 5 | frontend-design | Q | 7.5 | 8.0 | 7.0 | 8.5 | 7.65 | 0 | all pass |

Top four within 0.15 = judging noise. Still a Layoza loss.

Why Layoza lost (judge): lilac Approach band + solid navy enquiry block "push against quiet"; hero over-annotated with ~200px dead space under it; project register reads as a spreadsheet with tiny thumbnails.
Root cause in the skill: guardrail "colour owns sections" overrode the client's "simple, quiet" prompt. Fix: style words in the prompt (quiet, minimal, simple, calm) switch colour-blocking off — one ground, one accent.

Grader note: functional.py now fills textareas (all arms re-run). Basket/FAQ checks don't apply to this brief.
