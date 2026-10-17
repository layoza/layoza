# Round 10b — Kinetic Signal (Layoza rebuilt with live Layoza MCP, rivals' R10 pages reused)

Blind judge (1 judge, opus) on real viewport frames captured while scrolling (software WebGL for all). Weights: design 0.4, usability 0.3, creativity 0.2, content 0.1. Part B area scores are not in the total.

| Rank | Tool | Label | Design | Usability | Creativity | Content | Weighted | Motion | Static | Colour | Type | Mobile | Polish |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | ui-ux-pro-max | U | 8.0 | 8.0 | 7.5 | 8.5 | **7.95** | 7.5 | 8.0 | 8.0 | 8.0 | 8.0 | 7.0 |
| 2 | **Layoza (v2)** | P | 8.0 | 7.5 | 7.5 | 8.5 | 7.80 | 8.0 | 7.5 | 8.0 | 7.5 | 7.5 | 6.5 |
| 3 | taste-skill | Q | 8.0 | 7.0 | 8.0 | 7.5 | 7.65 | 8.5 | 7.5 | 7.5 | 7.5 | 7.0 | 6.5 |
| 4 | baseline | T | 7.5 | 6.5 | 7.5 | 8.0 | 7.25 | 7.5 | 7.0 | 7.5 | 7.5 | 7.0 | 5.5 |
| 5 | impeccable | R | 7.0 | 5.5 | 7.0 | 7.5 | 6.60 | 7.0 | 6.5 | 6.5 | 7.0 | 5.5 | 4.5 |
| 6 | frontend-design | S | 5.5 | 5.0 | 5.5 | 6.5 | 5.45 | 5.5 | 5.0 | 5.5 | 6.0 | 4.5 | 3.5 |

Layoza loss (gap 0.15 is inside judge noise, still a loss). Layoza wins no area outright (ties colour with ui-ux-pro-max).

Layoza flaws named by the judge:
- Pinned services stage: tiny card floating in a mostly empty dark frame (desktop scroll frames 4–8).
- Insights frame mostly empty; "Have something in mind?" label clipped under the header at one resting frame.
- Mobile: cobalt ribbon runs behind paragraph text; lime line crosses body copy.

Own QA on the final page: 0 errors, 0 warnings (text-overlap, bar-bleed, heading-break all clean), LCP 484 ms, CLS 0.0067. Remaining critiques: Inter 75% of text, six numbered kickers (brief-mandated), primitive-shape SVG art, four motion libraries.

## Round rk2 — Layoza rebuilt blind as `layoza-v3` (2026-10-16)

Rivals' R10 pages unchanged. Fresh blind opus judge. Key: layoza-v3 U, taste-skill T, impeccable S, ui-ux-pro-max R, baseline P, frontend-design Q.

| Rank | Arm | Design | Usability | Creativity | Content | Total |
|---|---|---|---|---|---|---|
| 1 | **layoza-v3** | 8.0 | 8.5 | 7.0 | 9.0 | **8.05** |
| 2 | taste-skill | 7.8 | 8.0 | 7.5 | 8.5 | 7.87 |
| 3 | impeccable | 7.8 | 7.5 | 7.5 | 7.5 | 7.62 |
| 4 | ui-ux-pro-max | 7.2 | 6.5 | 7.5 | 7.0 | 7.03 |
| 5 | baseline | 7.0 | 6.0 | 7.0 | 7.5 | 6.75 |
| 6 | frontend-design | 6.3 | 6.5 | 6.0 | 7.0 | 6.37 |

Margin +0.18: first, but within judge noise — not a proven win. Layoza won static, colour, type, polish; motion went to ui-ux-pro-max (ribbon), mobile to taste-skill. Layoza flaws: headings look cut by the sticky nav in mid-scroll frames, dead dark band after the showreel, ribbon tiny on mobile, safe torus silhouette (creativity 7.0, lowest of the top three). Note: some rk2 QA runs overlapped QA-script edits; judges used screenshots only.

## Round rk3 — Layoza rebuilt blind as `layoza-v4` (2026-10-17) — Layoza lost

Rivals unchanged. Fresh blind opus judge, read only BRIEF.md and `blind-rk3`. Key: layoza-v4 R, taste-skill P, baseline U, ui-ux-pro-max Q, impeccable S, frontend-design T.

| Rank | Arm | Design | Usability | Creativity | Content | Total |
|---|---|---|---|---|---|---|
| 1 | taste-skill | 8.0 | 7.5 | 8.0 | 8.5 | **7.90** |
| 2 | baseline | 8.0 | 7.5 | 7.5 | 8.0 | 7.75 |
| 3 | **layoza-v4** | 8.0 | 6.5 | 8.0 | 8.0 | 7.55 |
| 4 | ui-ux-pro-max | 7.5 | 6.5 | 8.0 | 8.0 | 7.35 |
| 5 | impeccable | 7.0 | 6.5 | 7.0 | 7.5 | 6.90 |
| 6 | frontend-design | 6.0 | 5.5 | 5.5 | 6.0 | 5.75 |

v4 is worse than v3 (8.05, 1st in rk2). The judge rated v4's ribbon the most sculptural, best-lit one in the field, but usability sank it:
- the ribbon crosses MOVE, the hero CTA and "IT MATTER.";
- the header CTA is clipped, and the header vanishes on desktop after the hero;
- a floating round menu button covers body text on mobile.

QA said 0/0 at the time. A new rule, `floating-control-covers-text`, now catches the button (6 hits); nothing yet catches WebGL drawn over text. Functional probe: Escape did not close the mobile menu from the page's focus state (the QA probe from the toggle disagrees, unconfirmed).

Root-cause fixes went into `what-wins.md` rule 10: a solid desktop header, no lone floating button, art never through text, and pinned text visible from frame 1. A fresh blind v5 follows.

## Round rk4 (2026-10-17) — layoza-v5 fresh blind rebuild

Blind key: layoza-v5=U, ui-ux-pro-max=Q, impeccable=R, frontend-design=S, baseline=P, taste-skill=T. One blind opus judge, 43 stills (1440 + 375 scroll frames).

| Rank | Arm | Design | Usability | Creativity | Content | Total |
|---|---|---|---|---|---|---|
| 1 | **layoza-v5** | 8.0 | 8.5 | 7.0 | 8.5 | **8.00** |
| 2 | taste-skill | 7.8 | 7.5 | 8.0 | 8.0 | 7.77 |
| 3 | impeccable | 7.5 | 7.0 | 7.5 | 8.0 | 7.40 |
| 4 | ui-ux-pro-max | 6.5 | 6.5 | 6.5 | 8.5 | 6.70 |
| 5 | baseline | 6.5 | 5.5 | 7.0 | 5.5 | 6.20 |
| 6 | frontend-design | 6.0 | 6.5 | 5.0 | 7.0 | 6.05 |

Measured: layoza-v5 is the only arm with 0 QA errors, 0 warnings, 0 contrast failures (LCP 0.8s); every rival trips `art-over-text`.

Margin +0.23: Layoza wins kinetic for the second round running (rk2 +0.18) but not by the >0.3 needed to call it a clear 1st. Judge's Layoza flaws: creativity still 7.0 (headline "MOVE" only recoloured, process a standard 4-card stepper, reel "FLOW" outline is safe); "IDEAS" letters collide; "WHAT WE DO." clipped mid-pin; bare cobalt band below Orbit; "NTRAL" cut on a mobile card. taste-skill won Motion (accordion process, ribbon reel switcher).

## Round rk5 (2026-10-17) — layoza-v6 fresh blind rebuild (with what-wins rule 13)

Blind key: layoza-v6=R, ui-ux-pro-max=T, impeccable=S, frontend-design=Q, baseline=P, taste-skill=U. One blind opus judge, 42 stills.

| Rank | Arm | Design | Usability | Creativity | Content | Total |
|---|---|---|---|---|---|---|
| 1 | taste-skill | 8.0 | 8.5 | 7.5 | 8.5 | 8.10 |
| 2 | impeccable | 8.0 | 7.0 | 8.0 | 8.0 | 7.70 |
| 3 | **layoza-v6** | 7.5 | 6.5 | 8.0 | 7.5 | **7.30** |
| 4 | ui-ux-pro-max | 7.0 | 7.0 | 7.0 | 8.0 | 7.10 |
| 5 | baseline | 6.5 | 6.0 | 7.0 | 5.5 | 6.35 |
| 6 | frontend-design | 6.0 | 6.5 | 5.5 | 7.5 | 6.20 |

Layoza lost (3rd). Rule 13 lifted creativity to 8.0 (signal-monitor process cards, chaptered reel) but usability fell to 6.5: header hid on scroll down so no desktop frame had a nav (the rebuild prompt still offered "hide-on-scroll-down header", contradicting what-wins rule 10 — prompt bug, fixed for future rebuilds), lime used on too many words, "FEEL" overlay clipped, CTA cut by frame edge, name "Foldline" drifted from the brief.

Judge noise: the frozen taste-skill page scored 7.87 (rk2), 7.90 (rk3), 7.77 (rk4), 8.10 (rk5) — a 0.33 spread on identical pixels. Single-judge gaps under ~0.35 are not significant. Best Layoza kinetic result remains v5 (rk4, 1st at 8.00, +0.23).
