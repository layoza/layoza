# Tidefest — direction (dev only, never served)

## Engine
design-system: match_confidence none (fell back to local-business). Used only Direction B
(kinetic narrative / pinned stage) as skeleton; palette+fonts discarded.

## Subject inventory
Facts: fictional 3-day electronic + alternative festival; beach near Newquay, Cornwall; 14–16 Aug 2027;
cap 8,000; stages Shoreline + The Cave; camping; sunrise set every morning; Atlantic surf coast; tides.
Objects: the stage rig (truss + line-array speaker stacks) on sand; the sound itself (bass waves);
wristband/ticket; tents; the sea/tide line at dawn. CIRCLED: the stage rig on the sand, throwing sound.
Scene: someone on a phone in autumn/spring planning a summer trip with friends; daylight reading -> light page,
with one deep-water night section (The Cave / night) earned by content.

## Rut
Reflex: near-black + neon gradient, lasers, glitch, blurred crowd photo, "festival poster" sans.
Obvious opposite: pastel sunset gradient + serif "coastal" boho.
Name pun: tide/waves as decoration. Avoided as decoration; tide used only as real notation.

## Seven worlds (by resonance) -> deal
1 Admiralty chart + tide tables  <- DEALT (#1, key 1282d03273dcf8e4)
2 RNLI flags + enamel beach signs
3 Line-up screenprint poster
4 Speaker stacks + waveform
5 Fistral surf heat boards
6 Sunrise ritual
7 Sea cave / slate geology
Challengers: flags (2) donates RAISE: total colour commitment — chart colours own whole bands like flag fields,
never sprinkled. Poster (3) donates RAISE: tiered type sizes for the line-up (headliners biggest).

## THESIS
On this page, the sound from the Shoreline stage becomes the bay's depth contours: the bass pushes chart
contour lines out across the water, and the tide curve becomes the festival clock.
Refuses: neon-on-black rave page and the pastel-sunset coastal page.

## OWN-WORLD
Admiralty chart: buff land, chart-blue shallows, white deep water, black ink hairlines, MAGENTA for lights
(the chart convention for light flares) = our action colour. Italic labels for water features, roman for land
(real chart convention) — reused: stage/venue names roman caps, water/time labels italic. Soundings numerals,
contour lines, scale bars, lat/long ticks on frames. Buttons = chart-label boxes with hairline frame.

## TOKENS
--ink #0D1F33 (chart ink, navy-black)  --chart #F3F7F8 (deep-water white, cool)
--sand #F2C94C (land buff, saturated)  --shallow #8FCBE6 (shallow-water blue)
--light #B5136F (magenta light flare: CTAs, focus)  --deep #0B3A63 (night water field)
Fonts: Anybody (display, variable wdth 50–150, wght; kinetic) + Instrument Sans (body/labels, italic for water).
Scale: 15 / 18 / 28 / 56 / clamp(72–200). Alignment: 12-col, two left edges (col 1, col 5).

## FIRST VIEWPORT
Desktop:
+----------------------------------------------------------------+
| TIDEFEST(logo)   Line-up Tickets Camping Info     [Buy tickets] |
| T I D E F E S T  (kinetic Anybody, full width, swells per letter)|
| 14–16 Aug 2027 · Newquay, Cornwall   [Buy tickets] lat/long     |
|  ~~~~ chart canvas: sea (blue) + buff headland; STAGE RIG drawn  |
|  at scale on the sand (~45% of viewport), sound contours ripple  |
|  out across the bay with soundings; sun on horizon               |
+----------------------------------------------------------------+
Phone: logo + menu; TIDEFEST wordmark stacked 2 lines; dates/place; CTA; rig art bottom half.

## SECTIONS
- Hero: claim = beach festival on the Cornish coast; proof = the rig on the sand + dates; chart canvas.
- Tide clock (pinned scroll story): claim = a day here runs from noon to sunrise; proof = tide curve draws,
  sun sets/rises, events plotted (gates, Shoreline, The Cave, sunrise set). Composition: a chart diagram.
- Line-up: claim = 18 acts, 2 stages, 3 days; proof = tide-table rows with times, tiered type; day filter.
- Countdown: claim = it's coming; proof = days/h/m/s + tide cycles until gates. Composition: type at scale on
  magenta field.
- Tickets: claim = simple, priced; proof = 3 ticket stubs, steppers, running total. Composition: tickets.
- Camping & getting there: chart plan of site + 4 legend blocks. Composition: map + legend.
- FAQ: accordion, sand field.  Newsletter: deep-water field.  Footer: chart frame, scale bar.

## MOTION
Client: motion is the brand. GSAP 3.13 + ScrollTrigger (pinned/scrubbed tide clock, entrances);
canvas for contour ripples (reactive to pointer + 124 BPM pulse, paused offscreen);
variable-font kinetic type (wdth wave across TIDEFEST, reacts to pointer); line-up names stretch on hover;
countdown digits roll; ticket total rolls. Frequent controls <=200ms. Reduced motion: no pin/scrub, static
contours, static type; all content visible without JS.

## Reflex-check notes
- Small italic chart labels near headings are chart notation (place labels), not eyebrows; keep sparse.
- Hairline frames with lat/long ticks = real chart neatline.

## Brief conflict
14 Aug 2027 is a Saturday, but brief requires Fri/Sat/Sun filter. Tabs say Fri/Sat/Sun without dates;
flag in report.

## FINISH
Unverified is unfinished: art loop done, `layoza qa` 0 errors.

## Deliberate CRAFT notes
- craft-repeating-stripes-gradient on .neatline: it is the Admiralty chart border graduation (alternating
  black/white minute bars), the dealt world's own device, used once under the hero. Kept.

## Art loop log
R1 (self, ~6.4): heading reveal stuck (clip-path to none), stage cut by fold, mobile menu trapped inside
  header (backdrop-filter containing block), map label clipped, Instrument Sans flagged overused -> fixed;
  body face -> Schibsted Grotesk; fonts CSS made non-blocking (LCP variance 2.5–7.5s under load -> <1.5s).
R2 (fresh-eyes reviewer, 7.3, FIX): contours too faint; mobile tide chart unreadable; kinetic artefacts;
  split hero copy + orphan sun; 18 uniform rows; small defects -> all addressed.
R3: see report.
