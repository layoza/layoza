# Cross-type benchmark — October 2026

Question: is Layoza best on *any* kind of website, not just brand/marketing pages?
Method: same brief per type, 6 arms built in sequence by one agent per type, graded by the same QA script, blind-judged by one opus judge per type from static screenshots. Weights: Design 0.4, Usability 0.3, Creativity 0.2, Content 0.1.

## Latest rounds — 2026-10-16 (rebuilt Layoza arms, rivals frozen)

Layoza pages were rebuilt fresh in new folders (`layoza-v3`, `layoza-v4`) by
agents that never saw old pages, results or judge notes; only the skill rules
changed. Rival pages are the same files as below. One fresh blind opus judge per
round, scroll-frame stills at 1440 and 375.

| Arm | Kinetic (rk2, v3) | Store (rs2, v3) | Dashboard (rd3, v4) | Restaurant (rr3, v4) | Mean |
|---|---|---|---|---|---|
| **layoza** | **8.05 (1st)** | **8.05 (1st)** | **8.44 (1st)** | **8.55 (1st)** | **8.27** |
| impeccable | 7.62 | 7.65 (2nd) | 6.97 | 7.45 (2nd) | 7.42 |
| ui-ux-pro-max | 7.03 | 7.30 | 6.62 | 7.20 | 7.04 |
| frontend-design | 6.37 | 7.15 | 7.76 (2nd) | 6.65 | 6.98 |
| taste-skill | 7.87 (2nd) | 6.15 | 7.00 | 6.25 | 6.82 |
| baseline | 6.75 | 6.75 | 5.85 | 7.30 | 6.66 |

Margins over the runner-up: kinetic +0.18, store +0.40, dashboard +0.68,
restaurant +1.10. Honest reading:
- Store, dashboard and restaurant are clear wins (> 0.3).
- Kinetic is first but within judge noise. The same rival page moved up to
  0.4 between judges on the same frames (frontend-design dashboard: 7.35 in a
  voided run, 7.76 in rd3).
- Nothing scored 9+. Every judge said no entry ships unchanged at a top studio.
- Area wins for Layoza: kinetic static/colour/type/polish (motion went to
  ui-ux-pro-max, mobile to taste-skill); dashboard data viz/colour/type
  (mobile and polish to frontend-design); restaurant imagery/colour/type/
  mobile/layout, polish tied.
- Voided run: the first rd3 blind set captured the old `layoza` folder instead
  of `layoza-v4` (operator error: `ARMS` not passed to the blind script). It
  was archived as `blind-rd3-v1mistake` and re-run; the old v1 page placed 3rd
  (6.95) there.

Remaining Layoza flaws named by judges: headings looking cut by the sticky
header in mid-scroll stills; small signature art on mobile (kinetic ribbon);
ragged wrapped pill rows and a half-empty column (dashboard); an empty band on
the mobile hero and an over-used stencil face (restaurant). Folded into
`skills/layoza/references/what-wins.md` rule 10.

## Totals (first round, superseded above)

| Arm | Kinetic agency (r10b) | Store (rs1) | Dashboard (rd1) | Restaurant (rr1) | Mean, 3 new types | Mean, all 4 |
|---|---|---|---|---|---|---|
| **layoza** | 7.80 (2nd) | 7.75 (2nd) | 7.40 (2nd) | **8.20 (1st)** | 7.78 | **7.79** |
| impeccable | 6.60 (5th) | **8.10 (1st)** | **7.50 (1st)** | 8.15 (2nd) | **7.92** | 7.59 |
| ui-ux-pro-max | **7.95 (1st)** | 6.95 | 6.90 | 7.20 | 7.02 | 7.25 |
| frontend-design | 5.45 | 7.10 | 7.35 | 6.65 | 7.03 | 6.64 |
| baseline | 7.25 | 7.25 | 6.00 | 6.75 | 6.67 | 6.81 |
| taste-skill | 7.65 | 6.50 | 6.45 | 6.20 | 6.38 | 6.70 |

## Verdict (honest)
- **Layoza is not "best of best".** It wins 1 of 4 types, by 0.05 (a tie). On the 3 new types impeccable leads by 0.13.
- Layoza is the **most consistent**: top-2 in all 4 types; every other tool drops to 5th or below somewhere.
- Highest 4-type mean (7.79), but the lead over impeccable (0.20) is under the 0.3 bar we set for claiming "best".
- No arm reached 9 in any type.

## Area winners (judge Part B)
| Area | Store | Dashboard | Restaurant | Layoza wins |
|---|---|---|---|---|
| Static | impeccable | impeccable = layoza | impeccable | 0.5 / 3 |
| Colour | impeccable = layoza | **layoza** | impeccable | 1.5 / 3 |
| Typography | **layoza** | taste-skill | **layoza** | 2 / 3 |
| Mobile | baseline | impeccable = layoza | **layoza** | 1.5 / 3 |
| Polish | baseline = impeccable | impeccable = ui-ux-pro-max | ui-ux-pro-max | 0 / 3 |
| Motion | not judgeable from stills in any type | | | — |

## Technical (lab, not seen by judge)
Layoza had the fastest LCP in every new type (540 / 408 / 1876 ms) and 0 QA errors on dashboard and restaurant. Store had 2 `focus-obscured` errors that the build agent did not report.

## Layoza flaws that cost points (generalisable)
1. **Brief fidelity**: dashboard headlined 7-day sales when the brief said today's sales.
2. **Primary action visibility**: store product cards had no visible add/quick-view; mobile CTA below the fold (QA warned, agent shipped).
3. **Mobile hero order**: hero art / CTA pushed below the fold on store and restaurant.
4. **Colour restraint**: restaurant section colours too loud.
5. **Clipped table headers on mobile** (dashboard rota) — QA does not check non-heading text clipping in tables.
6. **Fake-looking data** (two KPIs both "+0.2%").
7. **Polish never won** — empty asides / newsletter voids recur.

## Limits
- One judge per type; ±0.2 is judge noise. Static frames only — motion untested in this round.
- Same agent built all 6 arms per type in sequence (later arms may benefit from learning the brief).
- `functional.py` probes are store/brand-specific; dashboard skipped them, store results for cart/faq unreliable for all arms.
