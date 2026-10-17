# Volta One — design plan (frontend-design skill, pass 1 + review)

Subject: fictional 16 kg electric city bike from a small Bristol company. Audience: city commuters
in Bristol/Bath/Cardiff who live with stairs, trains and small hallways. Primary job: make the
bike tangible (rotate it, see the battery and belt), then convert to order or test ride.
(Skill says "confirm with client": brief already names subject, so decided myself.)

## Tokens
Color
- Studio grey  #D9DFDF  page backdrop (cool seamless photo-studio paper, not cream)
- Paper hi     #EDF0EF  specs/quotes surfaces, form fields
- Ink          #182024  text (slate, not #111)
- Ink soft     #46525A  secondary text (≈6:1 on studio grey)
- Bristol cobalt #2337A6 single accent: wordmark, primary CTA, test-ride block, battery glow
- Bike colours (product, not page palette): Graphite #3B3F43, Moss #56653F, Clay #A8654A

Type: one family, Archivo variable (wdth 62–125, wght). Width is the expressive axis:
extra-expanded 125/800 for the giant wordmark and headings, normal width for body. Scale
14/18/24/36/48 + fluid display. Tabular figures in specs and price.

Layout: one continuous "stage" — a sticky full-viewport WebGL canvas; five scroll steps overlay it.
```
[hero]   VOLTA ONE (cobalt, giant, behind)      [battery]  panel |      bike close-up
            🚲 3D bike in front                              text |   frame turns glassy
 promise / £2,190      [Order] [Test ride]      [belt] [weight: bike lifted on a strap] [colours+summary]
```
Then quiet sections: specs table, cobalt test-ride block with form, three rider quotes, FAQ, ink footer.
Left-aligned text throughout; only the wordmark is centred.

Principles
1. The bike is the hero; the camera tells the story (scroll blends between framed states).
2. Boldness spent in one place: 3D + cobalt wordmark. Everything below is calm and typographic.
3. Every 3D state shows the trade's object: battery inside tube, belt teeth moving, bike hung on a strap at 16 kg.

## Review against the brief / generic defaults
- First instinct was a dark "tech" 3D stage with glowing accent → that is default #2 (near-black + bright accent). Revised to a daylight studio grey, like a real product shoot.
- Considered blueprint-cobalt grid background (Bristol blue → blueprint) → reads as a theme costume; kept cobalt as accent only.
- Considered 01/02/03 markers on the three stories → they are not a sequence; removed.
- Clay product colour is near the Claude terracotta tell — kept only as a bike option (brief requires it), never as UI accent.
- Giant wordmark behind a product is a known product-page move; kept deliberately because the bike physically overlaps it in 3D (depth is the point of the brief), and it is the h1.
- No all-caps labels, eyebrows, middle-dot meta, mono labels, or arrow-suffixed buttons.

## Decisions taken without the client
- Specs invented for a fictional product (360 Wh, 250 W hub, 25 km/h assist, sizes S/M/L). No awards, no third-party brands.
- "Add to basket" shows a confirmation only (no checkout exists).
- Address is fictional.

## Build log / self-critique (screenshots in .notes/shots, scripts shoot.py + interact.py)
- Pass 1: frame "glass" state didn't render — material.transparent toggle needed needsUpdate. Fixed.
- Belt read as a chain (round tube + bumps) → rebuilt as flat 18 mm ribbon with inward teeth and light print marks that travel.
- Close-ups too tight; colour step cropped the front wheel → pulled cameras back.
- Mobile: turn buttons collided with wordmark, CTA wrapped, colour panel hid the bike → moved wordmark down, nowrap CTAs, compact colour panel.
- Removed "City — days" em-dash labels (template tell) → two-column rows.
- Success copy claimed an email was sent → reworded to what actually happens.
- Wordmark fade moved out of the 3D module so the no-WebGL fallback also hands over.
Verified: 1440/768/375 no horizontal scroll, form errors + success, accordion, mobile menu (Esc closes), colour → 3D + summary, no-WebGL SVG fallback, reduced-motion load, zero page errors.
