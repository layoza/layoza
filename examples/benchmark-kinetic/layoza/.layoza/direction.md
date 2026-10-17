# Direction: Kinetic Signal (dev-only, never served)

Dealt world (layoza deal, key 77d9d46934003dc0): #1 Signal chain / oscilloscope notation.
Challengers: #2 rhythmic-gymnastics ribbon (donates: one continuous gesture across the page),
#3 folded sheet metal (donates: honest metal material, fold lines as structure).

## THESIS
On this page, an idea becomes a signal: a flat line in the hero is unfolded, modulated, amplified
and transmitted as the visitor scrolls, and the metallic ribbon is that signal made solid.
Refuses: the agency template (hero + 3 service cards + logo wall + testimonial carousel).

## OWN-WORLD
Oscilloscope at night vs. printed spec sheet by day. Obsidian = the scope screen; porcelain = the
printed notation; cobalt = the trace; lime = the cursor / trigger point (sparingly: active states,
the live dot, CTA hover). Lavender only as a faint second trace. Components: hairline rules,
waveform edges between sections, timecode numerals, magnetic round CTA.

## TOKENS (pinned by client)
obsidian #090B11, graphite #151923, porcelain #F6F5F0, ink #111318, mute-dark #9299A8,
mute-light #5A6070 (tinted down so it passes 4.5:1 on porcelain), line-dark #2B303B,
line-light #D9DAD6, cobalt #4865FF, cobalt-deep #304EF0, lime #D8FF57, lavender #A99BFF.
Type: Space Grotesk (display, labels, numerals), Inter (body). Scale: display XL clamp(56,9.6vw,150),
display L clamp(44,6.4vw,104), display M clamp(30,3.4vw,52), body 17-18, label 12-13 caps +0.16em.
Alignment: one 12-col grid, 3 anchor lines (gutter, col 5, col 7).

## THREAD (page-long motion)
signal = document scroll progress 0..1, in four acts:
I INPUT (hero, intro) / II MODULATE (services, work) / III AMPLIFY (philosophy, process) /
IV TRANSMIT (showreel, insights, contact, footer).
Shown by: header trace line (amplitude + frequency follow signal; speed follows scroll velocity),
waveform section edges (amplitude grows by act), process waveform morph, contact ribbon, footer trace.

## FIRST VIEWPORT
Desktop: [logo | nav | Start a project]  /  left 6 cols: WE MAKE / IDEAS MOVE. + lede + 2 CTAs;
right ~55%: silver ribbon (WebGL), scroll cue bottom-left.
Phone: header (logo, Start, menu) / headline 2 lines / ribbon band behind-under headline / lede / CTA.

## SECTIONS
hero=split statement+3D / intro=asymmetric statement / services=sticky stage list /
work=editorial mosaic of 4 / philosophy=pinned full-bleed statement / process=pinned timeline rail /
showreel=full-bleed media stage / insights=typographic index rows / contact=massive statement+form /
footer=wordmark close.

## MOTION
GSAP+ScrollTrigger+SplitText: hero timeline, edges, scroll stories, pins. Motion (vanilla): menu,
disclosure presence, service stage swaps, form states. Three.js: hero ribbon. Lenis: smooth scroll
synced to ScrollTrigger, off for reduced motion. Subject-speaking feedback: CTA hover sends a pulse
along the header trace; form success draws a transmitted waveform; MOVE letters ride the wave.

## FINISH
Unverified is unfinished: art loop done, layoza qa 0 errors.
