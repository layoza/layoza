# Round 4 — Orrery planetarium (fresh brief, Layoza not tuned on it)

Blind judge (labels hidden, key revealed after scoring). Weights: design 0.4, usability 0.3, creativity 0.2, content fit 0.1.

| Rank | Tool | Blind label | Design | Usability | Creativity | Fit | Weighted | QA errors | Functional |
|---|---|---|---|---|---|---|---|---|---|
| 1 | **Layoza** | R | 8.5 | 8 | 9 | 9 | **8.50** | 0 | all pass |
| 2 | impeccable | Q | 7.5 | 8 | 8 | 8 | 7.80 | 0 | all pass (1 JS error in edge path) |
| 3 | baseline (no skill) | P | 7.5 | 7.5 | 5.5 | 7.5 | 7.10 | 3 (focus-obscured) | all pass |
| 4 | frontend-design | T | 6.5 | 7 | 7 | 8 | 6.90 | 3 (focus-obscured) | all pass |
| 5 | ui-ux-pro-max | S | 7 | 7.5 | 5 | 7 | 6.75 | 0 | all pass |

Judge on the winner: "its artwork is the product: a working planisphere tuned to Bristol's latitude, over a harbour skyline, carried through to the shop items."

Layoza weaknesses named by the judge (to fix in the skill, not in this page):
- Display grotesque vs bookish serif body pairing slightly awkward.
- Saffron booking band: faint input borders, small submit lost bottom-left.
- Empty strip below the footer.

Grader notes: functional.py fixed twice this round (form picker chose today's date whose early sessions had passed; loose "add" button regex). All arms re-run with the same fixed grader.
