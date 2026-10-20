# Decide: signals, motion level and techniques

Layoza does not pick the design. You do, from this brief's facts. This page
lists signals to read and techniques to choose from, each with when it fits.
Nothing here is required; the user's own words override all of it.

## 1. Read the signals

- **What is sold.** Craft itself (agency, studio, portfolio, launch) means the
  page is the proof, so motion and art carry weight. A product or service
  sold to a busy visitor means the task comes first.
- **Audience.** Peers and juries, buyers comparing options, shoppers, locals on
  phones, people under stress (health, money, government).
- **Task urgency.** Exploring, or booking/buying/finding hours right now.
- **Asset strength.** Real photography, footage, 3D or product? Weak assets
  push toward type-led motion, not WebGL.
- **Budget and device.** Mostly phone visitors, slow networks, older devices.

## 2. Choose a motion level (0–5) and write why

| Level | Feels like | Typical fit |
| :-- | :-- | :-- |
| 0 | Still, instant feedback only | Docs, forms, gov, emergency info |
| 1 | Gentle fades and hover states | Clinics, finance tools, local services |
| 2 | Reveals plus one scroll scene | Restaurants, shops' buying flow, B2B |
| 3 | A living hero plus several scroll scenes | SaaS stories, hotels, brands |
| 4 | Motion on every screen, smooth scroll, pinned chapters | Launches, portfolios, studios |
| 5 | An immersive world (WebGL/3D set piece, transitions, cursor play) | Agencies, award entries, campaigns |

Agencies and studios sell their motion: 4–5 is the norm, not a risk. A
restaurant at 5 hides the menu. Drop a level or two for accessibility-first
audiences; keep type motion and drop WebGL when assets or devices are weak.

## 3. Techniques (pick what serves the concept)

| Technique | Level | When it fits | Keep it safe |
| :-- | :-- | :-- | :-- |
| Split headings (line/word mask rise) | 2+ | Type-led pages, editorial voice | Text is in the DOM and visible before JS |
| Scrubbed image scale, clip or mask reveal | 2+ | Strong photography or work | Transform/clip only; no layout shift |
| Pinned scroll chapter | 2+ | A process, a product turning, a story in steps | Few per page; never pin a form |
| Horizontal or stacked run | 3+ | A body of work, a timeline | Also reachable by keyboard and list |
| Smooth scroll (e.g. Lenis) | 3+ | Long, cinematic pages | Off under reduced motion; anchors still work |
| Cursor-led previews, magnetic buttons | 3+ | Work indexes, studios | `(hover:hover) and (pointer:fine)` only; keep focus rings |
| Kinetic oversized type (variable axes, scroll scale) | 3+ | Studios, launches, music | Transform/opacity; no text hidden at rest |
| Page or shared-element transitions | 4+ | Multi-page work sites | Move focus to the new `h1`; crossfade under reduced motion |
| WebGL or 3D set piece (shader, model, particles) | 4+ | The subject is an object, material or world | Lazy start after first paint, DPR cap ~1.5–2, pause off-screen, static poster fallback |
| Preloader | 5 | Only when it really loads heavy assets | Under ~2s, skippable, never hides the LCP element |

## 4. Clichés to avoid at any level

The generic heavy-motion kit with no idea behind it: a 0→100 counter
preloader, a blob cursor that says "View", stacked sticky cards, RGB-split
hover, every heading scrubbed letter by letter, a 3D bottle spinning on scroll.
The sites people remember own one mechanic that only their subject could have.

## 5. Always

Honour `prefers-reduced-motion` (show the final frame), clean up timelines,
listeners and GPU objects, and keep INP under 200ms while things move.
