# Tidefest — direction (dev-only, never served)

Engine: design-system match_confidence = none (matched "local-business"); used only Direction B's
skeleton (kinetic narrative, choreographed stage). Palette/fonts from the world, not the engine.

## Subject
Facts: Tidefest (fictional); 3 days, 14–16 Aug 2027 (NB: Sat–Mon in 2027 — brief says Fri/Sat/Sun; calendar wins, flagged);
beach near Newquay, Cornwall; capacity 8,000; electronic + alternative; two stages Shoreline (on the sand) and The Cave
(enclosed); camping; a sunrise set every morning; tickets Day / Weekend / Weekend+camping in GBP; opens 12:00 BST 14 Aug.
Objects: the stage on the sand (truss, speaker stacks, light beams), the crowd with hands up, the Atlantic surf,
the sun coming up over the sea at the sunrise set, the wristband. **Circled: the Shoreline stage at the waterline at sunrise.**
Argument: hero (this is a festival on a beach where the sun comes up mid-set) → line-up (who, which day, which stage)
→ countdown (when) → tickets (buy) → camping/getting there (practical) → FAQ → newsletter → footer close.
Scene: phone, evenings, planning with friends; the brand moment is dawn on a beach → dark-to-dawn colour.

## Rut
Reflex: near-black + neon gradient, glitchy grotesque, photo of crowd with confetti, synthwave striped sun.
Obvious opposite: white editorial festival grid with tiny black sans. Both refused.

## Deal
7 candidates (tide charts, lifeguard flags, stage at waterline, sunrise over Atlantic, surf graphics, wristband,
engine houses). `layoza deal --top 4` → **#4 Sunrise over the Atlantic**.
Challengers: tide charts (declined on audience recognition — reads as surf forecast, not party) donates the raise:
**structural honesty — every number on the page is real data, set as tabular figures (times, prices, countdown)**.
Stage at waterline is the subject, used as the signature not the world.

## THESIS
"On this page, the sunrise set becomes the scroll: scrolling lifts the sun out of the Atlantic behind the
Shoreline stage, and the page turns from night to morning." Refuses the black+neon festival template.

## OWN-WORLD
Night sea indigo ground, dawn violet, coral horizon, sun yellow, Atlantic teal. Sky bands are real light,
not stripes. Components are sun discs, horizon rules, wristband-like ticket strips.

## TOKENS
--night #17134A (ground) · --dusk #3A2477 · --coral #FF6A4D · --sun #FFC83D · --sea #0F6E78 · --foam #FFF3E4 (light ink)
Family: Anybody (variable wdth 50–150, wght 100–900) only. Display wdth 50–150 wght 800–900; body wdth 100 wght 400.
Scale: 16/18 body, 24 (lead), 48 (h3/numbers), clamp 64–128 h2, hero wordmark ~16vw.
Alignment: one left anchor at the container edge + one inner column at 7/12.

## FIRST VIEWPORT
Desktop:
```
[TIDEFEST logo]   Line-up Tickets Camping Info      [Buy tickets]
 sky bands (indigo→violet→coral) ............................
        TIDEFEST (giant, letters riding the swell)
 ~~~~~~ sea horizon, sun half-risen, glitter path ~~~~~~~~~~
   [stage truss + speaker stacks + beams] crowd silhouettes  <- subject art ~60%
 14–16 Aug 2027 · Beach near Newquay, Cornwall   [Buy tickets] [See the line-up]
```
Phone: header (logo, Buy tickets, menu) / wordmark 2 lines? no: 1 line at 19vw / sun+stage art / dates+CTA.

## SECTIONS
- Hero: full-bleed scene, pinned scrub on desktop (sun rises, sky to dawn, beams fade).
- Line-up: list of large names (type is the image), day segmented control with sun riding an arc, stage tags.
- Countdown: coral field, giant tabular numerals, a horizon line the numbers sit on.
- Tickets: three wristband strips (horizontal rows), stepper, running total ribbon.
- Camping & getting there: sun-yellow field, 4 blocks along a drawn coastline map line (Newquay, beach, campsite, station).
- FAQ: indigo, accordion with horizon-rule rows.
- Newsletter: coral/dusk split, success = a sun rising inside the form.
- Footer: sun set at full width with the name again.

## MOTION
Signature: hero scroll story (sun rise, sky gradient, wordmark width). Ambient: SVG swell + glitter (rAF, paused offscreen).
Reactive: pointer steers stage beams; wordmark letters bob on swell. Kinetic type: line-up names widen (wdth axis) on
entrance via scroll. Micro: day filter sun travels the arc; stepper numbers roll; total counts; accordion grid-rows;
newsletter sun rise. GSAP 3.13 + ScrollTrigger pinned from jsDelivr. Reduced motion: static dawn frame, no pin,
no bob, fades only.

## FINISH
Unverified is unfinished: art loop done, `layoza qa` 0 errors.
