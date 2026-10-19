# Spanwise — direction

PURPOSE: One-page site for Spanwise, a fictional B2B SaaS that schedules plumbers, electricians and HVAC crews. Its job is to make an operations manager believe one claim: the board replans the day when a job runs long. Then it should get them to start a 14-day trial.

PICKED: auto — Direction A, System Diagram. Field-service scheduling is a system: calls and bookings feed a board, the board drives phones and customer texts, and the phones end in a paid invoice. Direction A's diagram-first content, with each part named and its limits stated, explains that loop directly. Its diff-view palette (added green, removed red) is how a dispatcher already reads a reshuffle. B and C leaned editorial or atmospheric, and the old page failed by not showing the product.

SHAPE: These are the axes as dealt.
- Hero: product-ui-bleed. The live dispatch board fills the hero, and the headline sits in the board's own side panel.
- Flow: sticky-index. A sticky "On this page" rail with node dots and a progress fill sits on the left, and the panes scroll on the right.
- Section heads: tab-heads. Each h2 sits in a slanted folder tab on the pane's top edge.
- Ground: colour-fields, in this order: deep green, mint, ink, red-deep, white, paper. One loud field per screen. Text on every field was checked at AA or better.
- Material: product-screens. Each one is cropped and annotated: board, system diagram, phone, route map, line drawing, annotated field photo, and the pricing ledger.
- Labels: sentence case throughout, in the body face.
- Motion: heavy.
  - vector-sequence: lottie-web 5.12.2 plays a before/after line drawing. Its JSON is built at runtime from the static SVG, and that SVG is the final frame when there is no JS.
  - pinned-scene: GSAP 3.12.5 + ScrollTrigger pin the 10:42 reshuffle map. A camera pull-back, the old route struck out, the new route drawn and the customer SMS are all scrubbed by scroll progress.
  - page-transitions: the View Transitions API morphs the chosen diagram node into the detail heading.
- Mechanics:
  - hero-frame-zoom-out: the board starts zoomed on the moved job and its site photo, then pulls back. The h1 stays still.
  - features-focus-dim-cropped-screen: the phone screen crops and zooms to each feature, with a dim ring around it.
  - pricing-ledger-column-lock: a real table with a monthly/yearly radio, a plan radio that locks its column with a double rule, an aria-live total and a crew-size slider.
- Furniture:
  - nav-masthead-dateline: a live date and time strip above a solid sticky masthead.
  - footer-search-first: a help search that live-filters on-page articles.

Deviations:
- The sticky index is hidden below 1180px, and the panes go full width.
- The reshuffle plays once on enter on mobile instead of pinning.
- meter.jpg is not used.
- I generated two resized copies inside saas/img: panel-640.jpg and rack-640.jpg. They were made from the approved photos to clear the oversized-raster warning.

ROUND 1: 5 errors, 6 warnings. The causes:
- The underline gradient read as 1:1 contrast.
- The nav CTA inherited ink text.
- The grid track blew out to 770px on mobile.
- The pane transform broke the GSAP pin.
- Fonts blocked render.
- Rasters were oversized.
- The help links wrapped raggedly.
- The index fill animated its height.

I fixed all of these. The underline is now a scaleX pseudo, the grid track is minmax(0,1fr), the pinned pane is excluded from the rise animation, fonts load async, the rasters are resized, the links sit in a grid and the fill uses scaleY.

ROUND 2: 1 error, 2 warnings. The causes:
- The fixed 316px phone overflowed at 320px.
- The pinned stills barely changed.
- The focus ring animated top.

I made the phone fluid, added a camera pull-back mapped to pin progress, and moved the ring to a transform.

ROUND 3: 0 errors, 1 warning. The camera zoom pushed map labels off the edge. I reduced it to 1.3x about the centre. After that the result was 0 errors and 0 warnings.

ROUND 4: polish, still 0 errors and 0 warnings. I made the h1 larger, added a swipe cue and a sticky first column to the mobile ledger, and shortened the pin to 760px. LCP is 1.8s and CLS is 0.004.

REVIEWER: 7/10. Weakest point: the hero board is honest but small-type and dense at 1440px. The headline panel leaves a quiet band above the diff log, and the first screen still reads more as a "tool screenshot" than as one unforgettable image.

ROUND 5 (coordinator review): rebuilt the hero and fixed the voids. QA: 0 errors, 0 warnings. LCP 0.7s, CLS 0.002.
- Hero: the h1 is about 109px at 1440. "One board that" is set condensed (wdth 68). "replans itself." is a boxed, expanded keyword whose width axis tweens from 62 to 125 on load (the kinetic word). The board is a 4-lane stage tilted 9deg, with 16px job text. On load it zooms out from the moved job, the overrun hatch grows, and J-4480 arcs from Marcus's lane into Dee's and turns green, with an elbow connector and badges. A right rail of board changes and the site photo fills the old empty band. The no-JS state is the final frame.
- Voids: the reshuffle pin spacer is gone. The map is CSS-sticky beside the steps, and the replay is additive (highlight trace, van dot, ring), so any still shows the finished route, ETA and text. The phone section lost its 36vh rows and gained a full-height route map behind the phone.
- Before/after: the octagon van is gone. The left half is meter.jpg in red duotone, with the Lottie drawing four van trips and round stop markers. The right half is a crafted J-4480 timeline card (5 timed steps, avatars, Paid same day).
- Mobile: the 10:42 marker now sits under the hour labels, and the moved-job badge rides the block's top edge.
- Type scale is 7 steps: 12, 14, 16, 18, 24, 48, and the display size.
REVIEWER: 8/10. Weakest point: the tilted board crops the left of the first job column, and the stops in the Lottie "before" drawing still read as diagram, not illustration.
