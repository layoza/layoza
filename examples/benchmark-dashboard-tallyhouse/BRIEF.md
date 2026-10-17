# Brief: Tallyhouse

## Business
Tallyhouse is an internal web app for the owner of a small bakery chain with four shops. It is a single-page operations dashboard, not a marketing site. All data is fictional sample data and must be labelled as such somewhere visible.

Sample shops: Bridge Street, Market Square, Old Mill Lane, Harbour Row (fictional).

## Audience
One owner (and occasionally a shop manager) checking the business early in the morning on a laptop, and on a phone between shops. They want to know quickly: how are sales going, what is about to run out, who is working, and what has been ordered.

## Required screens / regions
1. Top bar with a shop switcher (All shops + the four shops) and a date range control (e.g. Today / 7 days / 30 days, or similar).
2. KPI row: today's sales (GBP), change vs the same day last week, items sold, count of low-stock items.
3. Sales chart for the selected range. Draw it yourself (SVG or canvas); a chart library is allowed but not required. Must update when shop or range changes.
4. Low-stock table: item, shop, on hand, par/minimum level, supplier, and a working "Reorder" action per row.
5. "Pending orders" list: clicking Reorder moves that item out of the low-stock table into Pending orders (with supplier, quantity, expected date). KPI low-stock count updates.
6. Working filter / search on the stock table (text search; optional supplier or shop filter).
7. Weekly staff rota grid (staff x days of the week, shifts shown, current day identifiable).
8. Supplier orders (the pending orders list may be part of this region).
9. Empty, loading and error states shown somewhere sensible (e.g. no search results, a loading state when switching shop/range, a recoverable error for one panel with a retry).
10. Settings drawer (opened from the top bar; can be closed with Escape and a close button; contains a few sensible preferences, e.g. low-stock threshold, default shop, compact rows).

## Required working interactions
- Shop switcher and date range change the KPIs and chart.
- Reorder moves an item to Pending orders and updates the low-stock count.
- Search/filter on the stock table, with an empty state when nothing matches.
- Settings drawer opens/closes (button, close button, Escape), focus handled sensibly.
- Loading and error states are reachable and recoverable.

## Viewports
Must work at 1440px desktop and on a 390px phone (no horizontal page scroll; tables may scroll inside their own container or reflow).

## Brand constraints
No palette or fonts are given. Each design decides its own.

## Content rules
- Real-sounding bakery items: sourdough loaf, croissants, pain au chocolat, cinnamon buns, baguettes, rye flour, butter, eggs, etc.
- Prices and sales in GBP (£). UK date format (e.g. 16/10/2026 or Fri 16 Oct).
- Fictional suppliers and staff names; clearly sample data.

## Accessibility
- Everything keyboard reachable, visible focus indicator.
- Tables use proper header cells (th with scope) and captions or labels.
- Honour prefers-reduced-motion.
- Single self-contained HTML file (inline CSS/JS; Google Fonts and pinned CDN libraries allowed; no external images).
