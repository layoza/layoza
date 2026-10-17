# Brief: Loom & Leaf (fictional online shop)

## Business
Loom & Leaf is a fictional two-person studio in Lancashire, England, that makes handmade linen homewares: tea towels, aprons, table runners and cushion covers. They sell online, direct to customers, from a single-page storefront.

## Audience
UK home cooks and people furnishing their homes who value handmade goods, natural materials and small makers. They browse on phones as often as on desktops, and want to know what the product is like up close, what it costs, and when it will arrive.

## Deliverable
One self-contained HTML file (`index.html`): inline CSS and JS, Google Fonts and pinned CDN libraries allowed, no external images. All imagery is authored in SVG, CSS or canvas (no stock-photo look). Must work at 1440px desktop and on a 390px phone.

## Required sections
1. Header with shop name, a working product search, and a cart button showing the item count.
2. Hero that sells the craft (the making and the cloth) with authored artwork, and a route into the shop.
3. Product grid of at least 9 fictional products. Each card shows name, price (GBP) and colour swatches.
4. Category filter (All, Kitchen, Table, Living) and sort (price low-high, price high-low, newest) that both actually change the grid.
5. Product quick-view dialog: colour choice, quantity, and an "Add to basket" button.
6. Cart drawer: line items, change quantity, remove, running subtotal/total, and progress toward free delivery at £60.
7. Story / craft section about the studio and how the linen is made.
8. Customer reviews.
9. Delivery and returns information.
10. Newsletter signup with validation (empty and invalid email states, success state).
11. Footer.

## Required working interactions
- Search filters products by name (works together with category and sort).
- Category filter and sort update the grid.
- Quick-view opens from each product, lets you choose colour and quantity, and adds to the cart.
- Cart count in header updates; drawer updates quantity, remove, totals and free-delivery progress.
- Newsletter form validates and shows success.

## Accessibility
- Dialog and drawer trap focus, close on Escape, and return focus to the trigger.
- Everything keyboard reachable with visible focus.
- WCAG 2.2 AA contrast; honour `prefers-reduced-motion`.

## Brand constraints
No palette or fonts given: each arm decides.

## Content rules
- Prices in GBP. Products, people, reviews and studio details are fictional; reviews and names must read as illustrative, not as real testimonials from real people.
- No invented awards, press logos or certifications.
