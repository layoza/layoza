# Tidefest — direction (dev only, never served)

## Inventory
Facts: fictional 3-day electronic/alt festival; beach near Newquay, Cornwall; 14–16 Aug 2027; 8,000 capacity;
stages Shoreline (on the sand, facing the Atlantic) and The Cave; camping; a sunrise set every morning;
18 fictional acts; 3 ticket types in GBP; opening 14 Aug 12:00 Europe/London.
Objects: the Shoreline stage rig (truss, line arrays, sub stacks, light beams) (CIRCLED), the Atlantic horizon + sun,
the cave mouth, the wristband, the tent.
Scene: on the sand from golden hour to sunrise. Dark-leaning, but the page itself is one night: amber -> night -> dawn.

## Rut
Reflex: near-black + neon accent + glow, lineup grid of square artist photos, generic wave graphic.
Obvious opposite: sun-bleached pastel surf-shop minimalism.
Name pun: a giant wave/tide as the hero (avoid: tide is also the previous benchmark's winning thread).

## Deal
7 candidates (sunrise sky, stage rig, riso rave flyers, surf decals, tide tables, wristband, sea cave).
`layoza deal --top 4` -> #2 Stage rig on the sand. Built as dealt.
Raise from #1 Sunrise sky: total palette commitment, the sky owns every section field.
Raise from #3 Riso flyers: billing courage, names set at poster scale by billing order.

## THESIS
On this page, scrolling runs one Tidefest night on the Shoreline stage: the sun sets into the Atlantic, the rig lights
up, burns through the Cave hours and hands over to the sunrise set at the footer. The visitor works the lighting desk
(pointer steers the beams). Refuses: black page + neon, photo-grid lineup.

## OWN-WORLD
Sky fields down the page (golden hour amber/magenta -> Congo-blue night -> cave black -> sea-teal small hours ->
violet 04:00 -> dawn amber). Stage-lighting gel colours. Components: lighting-desk faders, wristbands, LED-wall
readout, running-order billing. Header clock maps scroll to the night (18:00 -> 06:00).

## TOKENS
--night #0B0D2B  --cave #08070F  --gel-magenta #C8124F  --gel-amber #FFB238  --sea #0B4650  --sand #F6E3BC
Type: Archivo variable (wdth 62-125, wght 100-900) only. Display wdth 125/900 (wordmark), condensed 75/850 headings,
body 100/400. Sizes: wordmark ~15vw, H2 clamp(3rem,7vw,7rem), H3 2rem, body 18px, small 14px.
Alignment: one left anchor (gutter) + the 7/12 column line.

## FIRST VIEWPORT
Desktop:
[logo TIDEFEST | Line-up Tickets Camping Info | 18:00 clock | Buy tickets]
[ sky (golden hour) with sun over sea                                 ]
[ TIDEFEST wordmark (huge, spans width)                               ]
[ Shoreline stage rig drawn at scale (~45%) beams fanning over sea    ]
[ 14-16 Aug 2027 . beach near Newquay . 8,000 | [Buy tickets] [Line-up] ]
Phone: header (logo, Buy, menu); wordmark full width; stage art; dates/place; Buy tickets button above fold.

## SECTIONS (acts)
I 18:00 Sundown, hero: pinned scrub sun sets, sky darkens, rig lights ignite. (stage/full-bleed)
II 21:00 Line-up: billing poster, day filter radios, stage per act. (poster/list)
   22:30 Countdown: LED-wall readout at display size on magenta. (statement)
III 00:00 The Cave, tickets: cave mouth opens on scroll; wristband steppers + total. (rack)
IV 02:00 Camping & getting there: route line drawn by scroll, 4 stops. (diagram)
   04:00 FAQ: accordion on violet. (list)
V 05:30 Sunrise set: newsletter + footer, sun rises from sea, wordmark again. (close)

## MOTION
GSAP 3.13 + ScrollTrigger + SplitText (pinned). Thread: page progress -> clock, sky sun/moon. Hero pinned scrub
(desktop), unpinned scrub on phone. Kinetic type: wordmark chars rise from the sea line on load; section headings
widen (wdth axis) as they enter. Reactive: pointer steers beams; scroll velocity drives the line-up marquee;
live countdown; total rolls. Micro: day filter sweeps a beam; ticket add clips a wristband; success stamps a band.
Reduced motion: no pin/scrub; each act shows its key frame statically; clock still updates.

## FINISH
Unverified is unfinished: art loop done, `layoza qa` 0 errors.
