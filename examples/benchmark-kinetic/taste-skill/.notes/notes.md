# Dev notes (taste-skill arm, brief R10 Kinetic Signal)

## Reading
Agency homepage, no real name/clients. Client prompt is prescriptive (sections, copy, colours, fonts, GSAP/Three).
Client prompt wins over skill defaults where they conflict; recorded below.

## Inventory (what belongs to this subject)
1. The concept name "Kinetic Signal" (editable placeholder wordmark).
2. The ribbon: folded, twisted, metallic silver with cobalt reflections.
3. Exact palette: obsidian #090B11, graphite #151923, porcelain #F6F5F0, cobalt #4865FF/#304EF0, lime #D8FF57, lavender #A99BFF.
4. The headline "WE MAKE / IDEAS MOVE." with MOVE as the kinetic word.
5. Four disciplines: brand, web, growth, motion.
6. Four stages: Discover, Define, Create, Launch & Evolve.
7. The section rhythm dark/light/cobalt (nine chapters).
8. Statements: "Good design is seen" -> "Great design is felt", "Let's make it matter".
9. Constraint: no real work, no real testimonials -> demonstration projects, testimonials omitted.

## Argument (claims -> proof)
- Hero: we make ideas move -> the ribbon itself, moving, reacting.
- Intro: four disciplines, one team -> braid graphic of four strands becoming one ribbon.
- Services: what we do, concretely -> each discipline shown as its own live composition.
- Work: what it looks like -> four demonstration projects at scale (clearly labelled).
- Philosophy: what we believe -> seen -> felt transformation.
- Process: how we work -> four stages, scroll-driven.
- Showreel: motion craft -> live coded motion studies you can drag.
- Insights: how we think -> three sample essays.
- Contact: start -> working form (mailto) + email.

## Direction
Dealt: colour-chapters (seed 869301407) out of sentence-as-layout / colour-chapters / dense-tiles.
Why: the client already prescribes a chapter-by-chapter field rhythm; colour-chapters is the composition that carries it.
--keep palette,type (brand fixes them).
Hero blueprint (mosaic masthead with photographs) overridden by the client's explicit hero (type left, Three.js ribbon right).
Section blueprints used: staircase statement (intro), stacking sheets -> sheet transition for services, giant title (work),
accordion of tinted panels (process), object/list stage (showreel), one item per poster line (insights), contact-as-headline (footer).

## Idea
On this page, the signal ribbon (the folded strip in the hero) becomes the transition device and motif of every chapter:
it folds the sections into each other, braids the four disciplines, is twisted by hand in the showreel and crosses the contact type.
Live thing: drag across the showreel stage to twist the ribbon; hero ribbon follows the pointer.

## Overrides of skill house rules (client wins)
- Section labels "01 / WHO WE ARE" style (eyebrow, micro-label, numbering): requested verbatim by client.
- Thin dividers / progress lines: requested by client.
- Custom cursor, Lenis: requested by client (desktop fine pointer only, off under reduced motion).
- Pictures floor: brief harness forbids external images/video -> all artwork coded (canvas/SVG/WebGL).
- Lavender: used minimally (only inside one service composition).
- Secondary grey #9299A8 fails contrast on porcelain -> on light fields secondary text uses #5A606D.

## Critique (final round)
Seen in one second: the silver/cobalt ribbon beside "WE MAKE IDEAS MOVE."
Idea: the ribbon is the transition and motif of every chapter (folds between sections, braid, showreel, contact ribbon).
Live thing: drag to twist the showreel ribbon; hero ribbon follows the pointer.
Layout words: hero=split / intro=staircase / services=sticky index+stage / work=giant title+editorial wall / philosophy=pinned statement / process=accordion strip / reel=full stage / insights=poster lines / contact=massive type+form / footer=contact as headline.
Scores: design 7.5, usability 7.5, creativity 7, content 7 -> ~7.4.
Caps: client-mandated labels and 01-04 numbering (skill cap 6.5 by its own rules); no real pictures (brief forbids external images).
Remaining: services titles wrap to two lines at 1440; Low Orbit art is thin; 25 font sizes from clamp() steps.

## Verification
inspect.mjs: 0 FAIL with --allow eyebrow,numbering,micro-label,few-pictures,no-footage,palette-cream; WARN size-count, pinned-over-picture.
Playwright: 1440/768/375 scroll-through, no console errors, no horizontal overflow; menu, dialog, accordion, form (validation + mailto), reel tabs/pause, reduced-motion pass.
