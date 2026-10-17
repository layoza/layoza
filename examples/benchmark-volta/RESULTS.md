# Round 8 — Volta One e-bike (style: real-time 3D)

Blind judge on real viewport frames captured while scrolling (software WebGL for all). Weights: design 0.4, usability 0.3, creativity 0.2, content 0.1.

| Rank | Tool | Label | Design | Usability | Creativity | Content | Weighted | QA errors | Lab LCP (median) |
|---|---|---|---|---|---|---|---|---|---|
| 1 | **Layoza** | P | 8.0 | 7.5 | 8.5 | 9.0 | **8.05** | 0 | **4048 ms (fails ≤2.5 s)** |
| 2 | impeccable | T | 7.5 | 7.0 | 8.5 | 9.0 | 7.70 | focus-obscured | 2364 ms |
| 3 | baseline | Q | 7.5 | 7.5 | 6.0 | 8.5 | 7.30 | focus-obscured | 1884 ms |
| 4 | frontend-design | R | 7.5 | 6.5 | 6.5 | 8.5 | 7.10 | focus-obscured | 2208 ms |
| 5 | ui-ux-pro-max | S | 6.5 | 7.0 | 5.5 | 8.5 | 6.65 | 0 | 1896 ms |

Judge on the winner: the OS-map concept makes the 3D bike, scroll progress ("Sheet n of 5") and the Bristol story one idea; every claim is shown in 3D.

Layoza flaws (brutal):
- Bike model plain: thin uniform tubes, down tube doesn't read as holding a battery, belt renders like a chain. frontend-design's bike (R) was judged the best model.
- Colour section: the pinned stage empties to a bare map while the order summary is on screen.
- Dead space below the hero title block.
- Slowest page in the round: render-blocking Google Fonts stylesheet → 4.0 s LCP. Fixed in the skill (system-passes.md) and now measured (`render-blocking-fonts`).
- 6 SVG labels under 9 px (new `svg-text-illegible` rule).
