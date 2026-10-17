# Tidefest — design plan (frontend-design skill, two passes)

## Subject / audience / job (decided, not confirmed with client — unattended run)
- Subject: 3-day electronic + alternative festival on a Cornish beach, 8,000 cap, Shoreline + The Cave stages, daily sunrise set.
- Audience: 18–35 UK festivalgoers deciding whether to go and which ticket to buy.
- Primary job: get people to tickets; secondary: line-up by day.

## Pass 1 tokens
- Color: Atlantic #0B2E3F (base dark), Surf #3FD0C9, Sunrise #FFB23E, Flare #FF4F79, Foam #EAF7F4, Granite #5A6B73.
- Type: Anybody (variable wdth 50–150, wght 100–900) for everything. The width axis is the kinetic device.
- Layout: full-bleed hero wordmark edge to edge, left-aligned meta + CTA underneath; pinned scroll "sunrise" sequence; line-up as timetable rows (not cards); tickets as 3 rows with steppers + sticky total bar.
- Principles: motion = the sea. Everything that moves moves like a swell (sine, slow, continuous) or answers the user.

## Review against the brief / defaults
- First instinct was near-black + neon (festival default, AI trait #2). Changed: dark base is a real Atlantic blue, with two warm sunrise colours, not one acid accent.
- First instinct for line-up was a grid of artist cards with gradients (SaaS kit). Changed: timetable rows grouped by time, stage marked by icon + name. Rows encode real info (set time, stage).
- No all-caps eyebrows, no mono data labels, no middle-dot meta strings, no arrows in button text.
- Signature (the one bold thing): the TIDEFEST wordmark swells — each letter's width/weight driven by a travelling sine wave, pointer/touch pushes a local swell. Supporting scroll sequence: sun rises over the horizon as you scroll the pinned "sunrise" panel.

## Decisions made without the client
- 14 Aug 2027 is a Saturday, not a Friday. The brief asks for a Fri/Sat/Sun filter but also fixes 14–16 Aug next year. Kept the client's dates (factual anchor, countdown target) and labelled the filter with the true weekdays (Sat/Sun/Mon) — flagged in report.
- Countdown target: 2027-08-14 12:00 BST = 11:00 UTC.
- Prices invented: Day £79, Weekend £185, Weekend + camping £235. No checkout exists, so the pay button states that payment isn't connected on this page.
- Contact uses .example domain; social links are placeholders.

## Reduced motion
- Wordmark renders with a frozen swell shape; no rAF. Sunrise panel un-pins, sun shown risen, all lines visible. Wave drift, digit rolls and stagger disabled.

## Self-critique passes (Playwright, shots/ folder)
1. Hero wave SVGs clipped by global svg{max-width:100%} -> seams. Fixed (max-width:none, animate svg not path).
2. Sunrise lines too big, overlapped dark sea with dark text. Reduced scale, anchored to top, shortened line 2.
3. Tablet 2px overflow from footer socials. Fixed with minmax(0,..) + wrap.
4. Mobile menu clipped: backdrop-filter on header made it the containing block for fixed nav. Removed backdrop-filter.
5. Wordmark too small on tablet/mobile with empty top. Two-line TIDE / FEST under 860px.
6. Reduced-motion sunrise panel: last line hit the sea. Added bottom padding.
Checks: no console errors; no horizontal scroll at 1440/768/375; filter shows 6 sets; 2 Weekend + 1 camping = £605; email error + success; menu opens, Escape closes; both motion modes.
