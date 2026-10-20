# Taste arm — forward test, 21 October 2026

These are independently generated local demonstration pages, made with the same model family used for the comparison. They are not submissions by, or approved examples from, the skill author. No other arm or old design-lab HTML was read. Only this arm was written. Root handled the workspace Mode A routing separately; no Layoza direction was imported into these designs.

## Instructions actually used

Read the current upstream `skills/taste-skill/SKILL.md`, the draw script, its app-draw write/history paths, the full app-screen reference, and the copy, layout and critique references. The source script uses only its local research library and Node standard libraries. The history file was redirected with `TASTE_SKILL_HISTORY` into this arm, avoiding writes to a home-level skill history.

Each actual options draw and full direction draw was run. Exact stdout is retained in `*-options.txt` and `*-direction.txt`; machine-readable direction sheets are retained under each brief's `draw/direction.json`. Fixed seeds make the selection auditable. The drawn date is the tool's own timestamp; the local comparison date is 21 October 2026.

The common protocol takes priority over incompatible skill defaults: no external imagery, font downloads, or libraries; required copy preserved; no motion for Intervals or Noor; complete composition of all four gallery works; real local-only forms rather than the skill's mailto fallback. No photo-search command was run because asset parity is mandatory. Dealt motion and photo lists are retrieval output, not claims that they were implemented.

## Intervals

Inventory: Intervals as the space between moments; Atget; Stieglitz; Paris street; the ocean crossing in The Steerage; water lilies; Versailles fountain; dates 1898 / 1904 / 1907 / 1922–23; four complete prints; monochrome; Helvetica; enquiries.

Argument / proof: ordinary moments deserve attention → three historical prints on the first screen; the photographs make a conversation → supplied exhibition introduction; water offers another way of looking → Nymphéa; two distinct practices → brief factual artist biographies; enquiry → labelled local name/email form.

Three composition ideas from the options draw:

1. Split stage: a print stays fixed while the exhibition is read.
2. Uneven photographs: historical images placed as different-sized prints on a gallery wall.
3. Place carries it: a photograph occupies the first screen with the exhibition name.

Selected **uneven-photographs**, seed **21201**, hero **Three prints, one line**. The name's “intervals” become the spaces between prints; their unequal proportions and complete frames make looking the primary activity. The first photograph sits smaller at left, the vertical street print dominates the centre, and The Steerage answers it at right. The later water image has its own larger moment. Live thing: the exhibition enquiry.

Adaptations: no shutter or drift because motion is forbidden. No image cropping, notwithstanding blueprint ratios. The about blueprint assumes portraits that are absent; use two factual artist columns on a flat grey field, retaining its type-led contrast. The contact blueprint assumes links and no form; the protocol requires a form, placed in the corresponding right-hand enquiry column. The footer retains its large returning wordmark without an empty viewport. Helvetica is explicitly named first, with Arial as the Windows fallback.

## Willow

Inventory: Willow; neighbourhood; curiosity; books; free membership; four actual catalogue fixtures; title / author searches; Saturday storytime; make & mend; Thursday club; weekday / Saturday hours; closed Sunday; library photograph.

Argument / proof: this library offers new discoveries → exact introductory copy; find one → working four-book searchable catalogue; visit at a useful time → opening hours beside the reading-space photograph; join neighbours → three dated example events; join for free → local membership request.

Three composition ideas considered from the draw (the filtered pool returned only two distinct directions):

1. One motif: the open book's fold carries the system.
2. Page is the object: a stack of tabbed catalogue folders.
3. Page is the object, alternate application: a tipped-in library membership card over a reading-room photograph.

Selected **page-is-the-object**, seed **21202**, hero **Tabbed folder stack**. The catalogue folder becomes the page: section tabs, coloured paper panels, book records, a printed event sheet and the perforated joining coupon. The full required headline is retained over the file tabs rather than replaced with the shorter name suggested in the generic hero. Live thing: title/author search and empty result feedback.

Typeface draw: **Libre Franklin / Source Serif 4**. Asset parity and offline operation take priority: local Arial / Georgia preserve the gothic-headline / reading-serif roles without fetching fonts. This is a documented fidelity limitation. No mock book covers or invented book metadata. The single supplied library photograph is credited near the picture. Four real book records fill the slips; no extra images were added to satisfy the skill's generic image-count floor. Only a small, reduced-motion-aware record entrance and control press feedback are implemented; the eight-element motion minimum is not met, and is not claimed.

## Noor

Inventory: Noor; Lahore; Urdu welcome; anxiety about visits; listening; explaining before decisions; first-visit stages; check-ups; hygiene; restorative care; two illustrative stock photos; preferred future date; request rather than booking.

Argument / proof: care can begin with a calmer conversation → exact headline and introduction; the first visit is explained → three complete stages; care is understandable → three treatment descriptions; take the next step → local request with explicit not-booked confirmation.

Three options drawn:

1. Split stage: one calm room stays present beside a reading column.
2. Uneven photographs: stock photographs at different scales.
3. Place carries it: a room photograph as a full-screen first impression.

Selected **split-stage**, seed **21203**, hero **Menu column, sticky photograph**. Listening and explanation are the scrolling column; a dental-room photograph remains quietly beside it. The second supplied photograph closes the reading column. The appointment field breaks the split into a full-width sage-ink section so the long page is not monotonous. Live thing: local appointment request, constrained to today or later.

Typeface draw: **Boska**. Offline local Georgia preserves a serif-led reading column; the actual Boska face is unavailable in common assets and is not claimed as loaded. The date minimum uses local calendar components. Urdu carries `lang=ur` and `dir=rtl`. No crossfade or entrance motion is implemented because the brief forbids animation. Both photographs have nearby illustrative labels. No room image is represented as the fictional clinic's own premises, and no staff, address, ratings or results are invented. Generic itinerary durations and contact email assumed by the section blueprints are deliberately omitted because the brief provides neither.

## Hearth

Inventory: five production batches; pieces 48 / 96 / 48 / 24 / 32; 248 total; baking / queued / ready states; oven 01 / 02 temperatures; 12 minutes remaining; butter / rye flour; three collection customers and times; Canal Street; Alex Morgan; Wednesday 21 October 09:12; demo-only changes.

Screen question: **Which batches need the next production action?** Finding at initial load: Country sourdough is in the oven. Data and jobs: review five batches, start a queued batch, mark a baking batch ready, add a product/quantity batch. Secondary questions: what are the ovens doing; which stock is low; who is collecting next?

Three composition ideas considered (the filtered app draw returned only two styles):

1. Accent ladder with open plane and production list as the main work area.
2. Framed sheet with one tinted L-shaped app frame.
3. Accent ladder with production lanes; rejected because five rows and two-step changes are easier to compare in a single list.

Selected **accent-ladder**, seed **21204**, shell **Open plane**; overview and list patterns. The warmer paper and small brick-red oven/progress/action marks give it a bakery face, while the pale tint remains a subordinate state cue. A simple drawn oven arch names the product beside the Hearth wordmark. The production list takes the main column and the oven / stock rail starts alongside it; collections follow beneath and remain visible on the desktop first screen. All numeric summaries and production-strip widths are calculated from the batch array. Grid arithmetic: fluid canvas after 230px navigation and two 32px gutters; main column + 24px gutter + 316px rail equals the whole available canvas. At phone width the navigation folds, rail stacks, and each row explicitly keeps its quantity and action visible.

The sheet's sell-out-strip signature needs sell-out times and a comparison day that are absent. Instead the production-state strip encodes the actual known quantities (144 ready / 48 baking / 56 queued initially); no sales series is invented. Queued → Baking → Ready controls and add-batch dialog are real. Filtered zero results are explained; no loading or network errors are simulated because data is synchronous and local. The figure face is local Arial with tabular numerals, matching the kept app type role.

## Gates and equal repair

Actual Taste source lint commands ran on the three public pages and on the app with `--app`. Both reports (`page-lint.txt`, `app-lint.txt`) returned **0 FAIL, 0 WARN**. This is source lint, not a visual-readiness verdict.

Actual Taste browser inspection was attempted for all four pages, with explicit assigned-arm output folders. All four attempts failed before the browser could load a page: installed Chrome exited with **4294930433**, Crashpad registration `TransactNamedPipe: The pipe has been ended` / `crash server failed to launch`. The original errors are retained in each brief's `inspection.txt`. No successful Taste inspector or zero-fail rendered report is claimed.

Root supplies identical Playwright captures and structural / interaction tests for all arms. Shared initial feedback reported no gallery/library overflow, broken images or missing labels at 1440 / 390 / 320. Equal repair feedback required explicit `type=submit` hooks and gallery grayscale. Before repair all four initial pages were copied to `initial-before-repair.html`. The final version adds explicit submit types and grayscale to all gallery pictures. No second discretionary design round was taken.

## Visual review of shared captures

Opened the root's actual `renders/taste/{intervals,willow,noor,hearth}/{desktop,mobile}.png` images after the repair. This is a self-critique, not an anonymous comparative judgment. Tablet and 320px screenshots were not visually reviewed by this builder. The external critics and root's final shared tests remain the comparative evidence.

Intervals: first glance is the large street print; the complete frames and unequal sizes create the intended wall. The title, image captions, artist material and enquiry are all legible. Phone order is composed rather than just three compressed columns. Its weakness is the long vertical experience and small right-hand Steerage print; the image-led choice is quieter than some visitors may prefer. Author self-score: design 8, usability 8, creativity 7, content 8; weighted 7.8. Motion score caps are inapplicable to this explicit no-animation brief.

Willow: first glance is the yellow headline, followed immediately by meaningful catalogue tabs and search. The paper-object concept carries through catalogue, events and joining coupon. The phone coupon and events adapt clearly. Visible limitation: the long Field Guide title takes three lines on phone and the search placeholder is clipped; this is a specific departure from the skill's two-line heading rule. The functional label remains complete. Author self-score: design 7.5, usability 8, creativity 7.5, content 8; weighted 7.7. No further round was taken after equal repair.

Noor: the calm serif title and room photograph are clear and restrained; Urdu and illustrative captions are visible. The phone's photograph and process explanation read in an understandable sequence. The full-page screenshot leaves blank space in the right half below the initial sticky picture; that capture does not show the sticky image's presence during an actual scroll, but a long repeated split remains a compositional risk. Author self-score: design 7.5, usability 8, creativity 7, content 8; weighted 7.6.

Hearth: the biggest number answers the production quantity question and the batch rows remain the main work surface. Quantities and next actions are visible on phone. Ovens, stock and upcoming collections appear as genuine operational information, with no invented trend chart. Weakness: the production table is generously spaced, pushing collections near the bottom of the desktop fold, and phone supporting information requires scrolling. Author self-score: design 7.5, usability 8, creativity 7, content 8; weighted 7.6. App screens have no page imagery/motion floor.

## Assets and limits

Intervals: four supplied Cleveland Museum of Art images, CC0, creators/titles/dates in visible captions, collection and CC0 in the footer. Complete compositions are kept; grayscale normalises their tint to the explicit monochrome brief.

Willow: supplied `library-zetong-li.jpg`, with visible “Illustrative library photograph · Zetong Li”. Noor: supplied `dental-room-katarzyna.jpg` and `dental-consultation-harold.jpg`, with nearby illustrative labels and supplied creator identifiers. No additional licences are guessed; common acquisition records remain the source of licence provenance. Hearth uses only a small original inline oven mark and rendered real fixture data.

All clients, hours, events, customer orders and account data are explicitly demonstration content. Forms never send or store personal details. This is an offline comparison build, not a backend, production integration or publication. Generic skill requirements for extra imagery, actual downloaded dealt fonts, extensive motion and portrait photography are constrained by the matched brief protocol; these limitations must not be silently presented as full upstream compliance.
