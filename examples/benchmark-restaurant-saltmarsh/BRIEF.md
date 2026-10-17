# Brief: Saltmarsh (restaurant website)

## Business
Saltmarsh is a 40-cover seasonal seafood restaurant in a converted boathouse on the Norfolk coast (UK). The menu changes with the season and with what the local day boats land.

## Audience
Visitors to the Norfolk coast, weekend travellers from London and Norwich, locals marking an occasion, groups looking for a private room, and people buying a gift.

## Deliverable
A single-page website as one self-contained `index.html` (inline CSS/JS; Google Fonts and pinned CDN libraries allowed; no external images). Must work at 1440px desktop and on a 390px phone.

## Required sections
1. Hero that conveys the mood of the place. Compose the art yourself (SVG/CSS/canvas); it must not look like a stock photo.
2. About: the chef and the boats/fishers the kitchen buys from.
3. Menu with three tabs (Lunch, Dinner, Drinks). Items carry dietary tags V (vegetarian), VG (vegan), GF (gluten free). A dietary filter that actually filters the items.
4. Booking widget: date, time slot (only open slots can be selected; the restaurant is closed on Mondays), party size 1–8, name, email. Validation with clear errors and a confirmation state on success.
5. Private dining enquiry (form or clear enquiry route).
6. Opening hours and "find us", with a simple drawn map (no map embed).
7. Gift vouchers.
8. Press quotes (fictional publications and writers).
9. Footer.

## Required working interactions
- Menu tabs switch content and follow the ARIA tabs pattern (roles, arrow-key navigation, selected state).
- Dietary filter (V / VG / GF) shows only matching items.
- Booking: Mondays and past dates unavailable; closed/full slots not selectable; party size limited to 1–8; name and email validated; errors announced to assistive technology; success shows a confirmation summary.
- Private dining enquiry validates and confirms (if a form is used).
- Navigation works on desktop and phone.

## Accessibility and quality
- Keyboard reachable throughout, visible focus, form errors announced.
- Respect `prefers-reduced-motion`.
- Readable contrast; no horizontal scroll at 390px.

## Brand constraints
No palette or fonts are given; the designer decides.

## Content rules
Prices in GBP (£). All people, publications, suppliers and boats are fictional. No real reviews or trademarks.
