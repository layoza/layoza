# Retry (narrower)
## Design System: Saltmarsh

### Design Dials
- **Variance:** 6/10 — Balanced / Modern
- **Motion:** 3/10 — Subtle
- **Density:** 3/10 — Spacious

### Pattern
- **Name:** Funnel (3-Step Conversion)
- **Conversion Focus:** Progressive disclosure. Show only essential info per step. Use progress indicators. Multiple CTAs.
- **CTA Placement:** Each step: mini-CTA. Final: main CTA
- **Color Strategy:** Step colors: 1 (Red/Problem), 2 (Orange/Process), 3 (Green/Solution). CTA: Brand color
- **Sections:** Hero > Step 1 (problem) > Step 2 (solution) > Step 3 (action) > CTA progression

### Style
- **Name:** Vibrant & Block-based
- **Mode Support:** Light supported | Dark supported
- **Keywords:** Bold, energetic, playful, block layout, geometric shapes, high color contrast, duotone, modern, energetic
- **Best For:** Startups, creative agencies, gaming, social media, youth-focused, entertainment, consumer
- **Performance:** cost:low|drivers:none | **Accessibility:** risk:conditional|requires:contrast-text-4.5,keyboard,visible-focus,reduced-motion

### Colors
| Role | Hex | CSS Variable |
|------|-----|--------------|
| Primary | `#DC2626` | `--color-primary` |
| On Primary | `#FFFFFF` | `--color-on-primary` |
| Secondary | `#F87171` | `--color-secondary` |
| On Secondary | `#0F172A` | `--color-on-secondary` |
| Accent/CTA | `#A16207` | `--color-accent` |
| On Accent/CTA | `#FFFFFF` | `--color-on-accent` |
| Background | `#FEF2F2` | `--color-background` |
| Foreground | `#450A0A` | `--color-foreground` |
| Card | `#FFFFFF` | `--color-card` |
| Card Foreground | `#450A0A` | `--color-card-foreground` |
| Muted | `#F0EDF1` | `--color-muted` |
| Muted Foreground | `#475569` | `--color-muted-foreground` |
| Border | `#FECACA` | `--color-border` |
| Destructive | `#DC2626` | `--color-destructive` |
| On Destructive | `#FFFFFF` | `--color-on-destructive` |
| Ring | `#DC2626` | `--color-ring` |

*Notes: Appetizing red + warm gold [Accent adjusted from #CA8A04]*

### Typography
- **Heading:** Playfair Display SC
- **Body:** Karla
- **Mood:** restaurant, menu, culinary, elegant, foodie, hospitality
- **Best For:** Restaurants, cafes, food blogs, culinary, hospitality
- **Google Fonts:** https://fonts.googleapis.com/css2?family=Karla:wght@300;400;500;600;700&family=Playfair+Display+SC:wght@400;700&display=swap
- **CSS Import:**
```css
@import url('https://fonts.googleapis.com/css2?family=Karla:wght@300;400;500;600;700&family=Playfair+Display+SC:wght@400;700&display=swap');
```

### Key Effects
Large sections (48px+ gaps), animated patterns, bold hover (color shift), scroll-snap, large type (32px+), 200-300ms

### Motion
**Scroll Reveal** (Subtle) — Trigger: scroll (viewport enter) | Duration: 300-400ms | Easing: `power1.out`
```js
gsap.from(el, { opacity: 0, y: 12, duration: 0.35, ease: 'power1.out', scrollTrigger: { trigger: el, start: 'top 90%', toggleActions: 'play none none reverse' } });
```
*Framework notes: Requires the ScrollTrigger plugin registered once via gsap.registerPlugin(ScrollTrigger); Use matchMedia('(prefers-reduced-motion: reduce)') to skip non-essential motion and render the final state immediately*
- ✅ Keep the y offset small (8-16px) so it reads as a fade, not a slide
- ❌ Don't reveal below-the-fold content needed for SEO/crawlers as invisible-by-default without a no-JS fallback

### Avoid (Anti-patterns)
- Low-quality imagery
- Outdated hours

### Pre-Delivery Checklist
- [ ] No emojis as icons (use SVG: Heroicons/Lucide)
- [ ] cursor-pointer on all clickable elements
- [ ] Hover states with smooth transitions (150-300ms)
- [ ] Light mode: text contrast 4.5:1 minimum
- [ ] Focus states visible for keyboard nav
- [ ] prefers-reduced-motion respected
- [ ] Responsive: 375px, 768px, 1024px, 1440px


# style domain
## UI Pro Max Search Results
**Domain:** style | **Query:** editorial organic warm
**Source:** styles.csv | **Found:** 3 results

### Result 1
- **Style ID:** nature-distilled
- **Style Category:** Nature Distilled
- **Aliases:** 
- **Status:** active
- **Parent Style ID:** 
- **Preferred Mode:** auto
- **Type:** General
- **Keywords:** Muted earthy, skin tones, wood, soil, sand, terracotta, warmth, organic materials, handmade warmth
- **Primary Colors:** Terracotta #C67B5C, Sand Beige #D4C4A8, Warm Clay #B5651D, Soft Cream #F5F0E1
- **Effects & Animation:** Subtle parallax, natural easing (ease-out), texture overlays, grain effects, soft shadows
- **Best For:** Wellness brands, sustainable products, artisan goods, organic food, spa/beauty, home decor
- **Light Mode ✓:** supported
- **Dark Mode ✓:** conditional
- **Performance:** cost:low|drivers:none
- **Accessibility:** risk:low|requires:contrast-text-4.5,keyboard,visible-focus,reduced-motion
- **Framework Compatibility:** tailwind|css
- **Complexity:** Low
- **AI Prompt Keywords:** Design with nature distilled aesthetic. Use: muted earthy colors (terracotta, sand, olive), organic materials feel, warm tones, handmade warmth, natural textures, artisan quality, sustainable vibe, soft gradients.
- **CSS/Technical Keywords:** background: warm earth tones, color: #C67B5C #D4C4A8 #6B7B3C, border-radius: organic (varied), box-shadow: soft natural, texture overlays (grain), font: humanist sans-serif
- **Implementation Checklist:** ☐ Earth tones dominant, ☐ Warm feel achieved, ☐ Textures subtle, ☐ Handmade quality, ☐ Sustainable messaging, ☐ Calming aesthetic
- **Design System Variables:** --terracotta: #C67B5C, --sand-beige: #D4C4A8, --warm-clay: #B5651D, --soft-cream: #F5F0E1, --olive-green: #6B7B3C, --grain-opacity: 0.1

### Result 2
- **Style ID:** organic-biophilic
- **Style Category:** Organic Biophilic
- **Aliases:** 
- **Status:** active
- **Parent Style ID:** 
- **Preferred Mode:** auto
- **Type:** General
- **Keywords:** Nature, organic shapes, green, sustainable, rounded, flowing, wellness, earthy, natural textures
- **Primary Colors:** #228B22 (Forest Green), #8B4513 (Earth Brown), #87CEEB (Sky Blue), #F5F5DC (Beige)
- **Effects & Animation:** Rounded corners (16-24px), organic curves (border-radius variations), natural shadows, flowing SVG shapes
- **Best For:** Wellness apps, sustainability brands, eco products, health apps, meditation, organic food brands
- **Light Mode ✓:** supported
- **Dark Mode ✓:** supported
- **Performance:** cost:low|drivers:none
- **Accessibility:** risk:low|requires:contrast-text-4.5,keyboard,visible-focus,reduced-motion
- **Framework Compatibility:** tailwind|css
- **Complexity:** Low
- **AI Prompt Keywords:** Design a biophilic organic interface. Use: nature-inspired colors (greens, browns), organic curved shapes, rounded corners (16-24px), natural textures (wood, stone), flowing SVG elements, wellness aesthetic, earthy palette.
- **CSS/Technical Keywords:** border-radius: 16-24px (varied), background: earth tones, SVG organic shapes (blob), box-shadow: natural soft, color: #228B22 #8B4513 #87CEEB, texture overlays (subtle)
- **Implementation Checklist:** ☐ Earth tones dominant, ☐ Organic curves present, ☐ Natural textures subtle, ☐ Green accents, ☐ Rounded everywhere, ☐ Calming feel
- **Design System Variables:** --forest-green: #228B22, --earth-brown: #8B4513, --sky-blue: #87CEEB, --cream-bg: #F5F5DC, --organic-radius: 24px, --shadow-soft: 0 8px 32px rgba(0,0,0,0.08)

### Result 3
- **Style ID:** biomimetic-organic-2-0
- **Style Category:** Biomimetic / Organic 2.0
- **Aliases:** Biomimetic/Organic 2.0
- **Status:** active
- **Parent Style ID:** 
- **Preferred Mode:** auto
- **Type:** General
- **Keywords:** Nature-inspired, cellular, fluid, breathing, generative, algorithms, life-like
- **Primary Colors:** Cellular Pink #FF9999, Chlorophyll Green #00FF41, Bioluminescent Blue
- **Effects & Animation:** Breathing animations, fluid morphing, generative growth, physics-based movement
- **Best For:** Sustainability tech, biotech, advanced health, meditation, generative art platforms
- **Light Mode ✓:** supported
- **Dark Mode ✓:** supported
- **Performance:** cost:moderate|drivers:animation,blur
- **Accessibility:** risk:low|requires:contrast-text-4.5,keyboard,visible-focus,reduced-motion
- **Framework Compatibility:** canvas|webgl
- **Complexity:** High
- **AI Prompt Keywords:** Design a biomimetic organic interface. Use: cellular/fluid shapes, breathing animations, generative patterns, bioluminescent colors, physics-based movement, nature algorithms, life-like elements, flowing gradients.
- **CSS/Technical Keywords:** SVG morphing (SMIL or GSAP), canvas for generative, animation: breathing (scale pulse), filter: blur for organic, clip-path for cellular, WebGL for advanced, physics libraries
- **Implementation Checklist:** ☐ Organic shapes present, ☐ Animations feel alive, ☐ Generative elements, ☐ Performance monitored, ☐ Mobile fallback, ☐ Accessibility alt content
- **Design System Variables:** --cellular-pink: #FF9999, --chlorophyll: #00FF41, --bioluminescent: #00FFFF, --breathing-duration: 4s, --morph-ease: cubic-bezier(0.4, 0, 0.2, 1), --organic-blur: 20px


# ux: error summary
## UI Pro Max Search Results
**Domain:** ux | **Query:** error summary validation
**Source:** ux-guidelines.csv | **Found:** 3 results

### Result 1
- **Category:** Forms / Accessibility
- **Issue:** Focusable Error Summary
- **Platform:** Web
- **Description:** An error summary for failed validation complements inline field errors and must be easy to find by keyboard and screen reader users
- **Do:** Place it at the top of the form; move focus to its heading or container after failed submit; link each item to its invalid field; retain inline errors
- **Don't:** Replace inline errors with a visual-only summary or move focus on every blur
- **Code Example Good:** <div role="alert" tabindex="-1" aria-labelledby="error-title"><h2 id="error-title">There is a problem</h2><a href="#email">Enter an email address</a></div>
- **Code Example Bad:** Toast only with no field links or focus target
- **Severity:** High

### Result 2
- **Category:** Accessibility
- **Issue:** Error Messages
- **Platform:** All
- **Description:** Error messages must be announced
- **Do:** Use aria-live or role=alert for errors
- **Don't:** Visual-only error indication
- **Code Example Good:** role='alert'
- **Code Example Bad:** Red border only
- **Severity:** High

### Result 3
- **Category:** Forms
- **Issue:** Error Placement
- **Platform:** All
- **Description:** Each invalid field needs an inline error connected to that field
- **Do:** Show a specific error below the input and reference it with aria-describedby
- **Don't:** Show only a top-level error without identifying each invalid field
- **Code Example Good:** <input aria-describedby="email-error"><p id="email-error">Enter an email address</p>
- **Code Example Bad:** Red border or summary only
- **Severity:** High


# ux: tabs keyboard
## UI Pro Max Search Results
**Domain:** ux | **Query:** tabs keyboard arrow
**Source:** ux-guidelines.csv | **Found:** 2 results

### Result 1
- **Category:** Accessibility
- **Issue:** Keyboard Navigation
- **Platform:** Web
- **Description:** Web users need complete keyboard navigation with visible focus on every operable control
- **Do:** Keep tab order aligned with visual order and test every action without a pointer
- **Don't:** Keyboard traps or illogical tab order
- **Code Example Good:** tabIndex for custom order
- **Code Example Bad:** Unreachable elements
- **Severity:** High

### Result 2
- **Category:** Accessibility
- **Issue:** Skip Links
- **Platform:** Web
- **Description:** Allow keyboard users to skip navigation
- **Do:** Provide skip to main content link
- **Don't:** No skip link on nav-heavy pages
- **Code Example Good:** Skip to main content link
- **Code Example Bad:** 100 tabs to reach content
- **Severity:** Medium


# landing
## UI Pro Max Search Results
**Domain:** landing | **Query:** restaurant reservation hero
**Source:** landing.csv | **Found:** 0 results

No matches. This is not a match with an empty value -- the query did not hit the database. Retry with broader/different keywords before falling back to general defaults, and say explicitly that no database match was found if you do fall back.
**Closest known terms:** Hero + Features + CTA, Hero + Testimonials + CTA, Hero-Centric + Conversion, Hero-Centric + Trust, Hero-Centric Design, Portfolio + Hero-Centric
