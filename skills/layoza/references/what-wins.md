# What loses a blind comparison (every niche, every stack)

Blind judges docked Layoza for these across stores, dashboards, restaurants and
agencies. Treat each as a hard rule unless the prompt overrides it.

1. **Brief fidelity first.** Before designing, copy every concrete requirement
   in the brief (headline metric, named sections, states, hours, closed days,
   prices, cut-offs) into a checklist. The brief's named primary metric or
   action is the most prominent thing on its screen, worded the brief's way
   ("today's sales", not "7-day sales"). Tick the list against the final
   screenshots before calling it done.
2. **The main action is always in reach.** At 375px the first screen shows
   name → promise → main action; art may crop or follow, never push the action
   below the fold (`primary-action-below-fold` is an error). Anything for sale
   shows its price and a visible add / quick-view / book control on the card.
3. **No voids.** Every tinted or bordered box is filled with real content or
   shrinks to fit: asides carry a summary, policy, map or next step; a
   newsletter is a compact band, not a half-empty screen (`empty-panel`). The
   hero is composed edge to edge (no half of it empty sky), and the last
   sections before the footer hold their grid instead of stranding one block.
4. **Colour restraint.** Colour owns sections, but from one family: at most
   one loud ground per scroll screen and at most three saturated grounds per
   page. Pink-then-yellow-then-blue reads as noise.
5. **Data that feels real.** Sample numbers are plausible, varied and
   internally consistent: no two KPIs share a value, units pluralise
   ("1 pack", "3 packs"), dates and weekdays agree, totals add up, and every
   status agrees with its own rule (if "low" means under 50% of par, no listed
   item sits at 80%).
6. **Tables on phones** reflow into cards, or keep a sticky first column with
   a visible scroll cue (edge fade or "scroll →"). Header text is never cut.
7. **Subject, in the dealt material.** The trade must be unmistakable, but
   the dealt `page_shape.material` decides how it is drawn: product screens,
   rendered 3D, glass and light, illustration, pure type, or (only when dealt)
   paper and print artefacts. Never default to tickets, receipts and dockets on
   every section; one object-as-UI move per page is a signature, ten is a
   costume. Keep standard affordances (buttons look like buttons, tables scan).
8. **Motion floor.** Every page, including calm, minimal, "static" sites,
   dashboards and stores, ships the `micro-css-polish` recipe
   (`layoza animate --recipe micro-css-polish`): one hero entrance, hover and
   press feedback, a drawn link underline, eased focus, and scroll-driven
   section fades where supported. Content stays visible without it and
   reduced motion disables it. Skip only when the prompt says no animation.
9. **Sticky panels stay in their lane.** A sticky sidebar or summary column
   lives inside the grid row it belongs to, so the full-width section after
   it never scrolls beneath it (`text-overlap` "sticky panel covers content").
   Review the scroll screenshots, not only the first screen.
10. **Judges read scroll stills.** Section headings must never look cut by
   the header in a mid-scroll frame. On desktop keep a compact, solid header
   in view (a vanished header reads as broken) with `scroll-margin-top` on
   sections; on phones a solid bar, or one that hides on scroll down. Never
   leave a lone floating button over the text column
   (`floating-control-covers-text`); Escape closes any open menu from
   anywhere (`menu-escape-broken`). Signature art (3D, canvas, illustration)
   never crosses headlines, CTAs or body copy in any frame: give text its
   own zone or a solid plate, and keep the art behind and beside it, not
   through it (`art-over-text`). A pinned stage shows its text from its first frame (animate
   the art, not the words), so no frame is an empty coloured slab.
   The signature art stays big at 375px, never
   a thumbnail; its silhouette is bold, not a safe default shape. No dead
   bands between sections, no half-empty grid columns, and control rows
   (pills, chips, filters) never wrap into ragged second lines: scroll them
   or let them shrink (`ragged-control-row`). The first 375px screen has no
   empty band between copy and art (`hero-empty-band`); one display face is
   for display only, with a calmer face for section heads and UI labels.
11. **Polish is a score.** Ship with 0 QA errors **and** 0 warnings. Re-run
   `layoza qa` after the last edit and report that run's numbers, never an
   earlier one.
12. **Kinetic / WebGL hero art:** follow
   [`kinetic-composition.md`](kinetic-composition.md) (zones, plates, camera fit, pinned first frame).
13. **Clean is not creative.** Judges score a correct, tidy page about 7 for
   creativity; rivals with one bold mechanic score 8. Every page needs at
   least two signature moves unique to its concept, beyond the hero art:
   the headline's key word gets a treatment that enacts its meaning (a word
   about motion moves or bends, not just recolours), and one mid-page section
   uses a mechanic the trade or concept suggests instead of a stock pattern
   (no plain 4-card stepper, card grid or outline-text reel). Keep it legible
   in every still, and check display type for collisions such as touching
   letters.
   Get the two signature mechanics from `layoza mechanics deal` ([`mechanics.md`](mechanics.md)).
14. **The page shape is binding.** Build the direction's dealt `page_shape`
   (hero frame, flow, section heads, ground, material, labels, motion) as
   dealt. Two sites from Layoza must not share a skeleton: if your page is a
   split hero over stacked bands with "title left, note right" heads and mono
   labels, and that was not dealt, it is the house default leaking: rebuild.
   High `motion_intensity` means the dealt heavy techniques actually ship
   (pinned scenes, kinetic type, canvas/WebGL), each with a static fallback
   that QA and headless screenshots can see (never an empty pinned slab).
