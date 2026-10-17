<!-- Layoza original, in its own words. Dashboard principles follow Stephen Few's published concepts (no text quoted). See THIRD_PARTY_NOTICES.md, "Product UI ideas". -->

# Data visualisation and dashboards

## Chart from the question

| The question | Chart | Avoid |
| :--- | :--- | :--- |
| How did it change over time? | Line (area only for one series) | Bars for 30+ time points |
| Which is bigger? | Bar, sorted; horizontal when labels are long | Pie, radar |
| What is it made of? (≤5 parts) | Stacked bar or 100% bar | Pie over 5 slices, donut with KPI inside |
| What is it made of? (>5 parts) | Sorted bar + "Other" | Treemap for precise reading |
| How are values spread? | Histogram, box plot, strip plot | Average alone |
| Do two measures move together? | Scatter (+ trend line if meaningful) | Dual-axis line |
| How far from target? | Bullet chart, bar with target mark | Gauge, dial |
| Where? | Choropleth (rates, not counts) or dot map | 3D globe |
| Flow between stages? | Funnel as bars, or Sankey for branching | Funnel shapes with no axis |
| Single trend in a tile | Sparkline, no axes, last value labelled | Mini bar chart with ticks |
| Exact values matter | Table with in-cell bars | Any chart |

## Colour

- Categorical: at most 7 hues; same lightness band so none shouts; test with a
  CVD simulator (protanopia, deuteranopia, tritanopia). Pair colour with a
  second cue (direct label, marker shape, dash).
- Sequential: one hue, light to dark. Diverging: two hues around a neutral
  midpoint that means something (zero, target).
- Highlight one series in the accent and grey the rest when a chart makes one
  point.
- Status colours (success, warning, danger) never double as series colours.
- Every hue meets 3:1 against the chart surface in both themes.

## Axes and numbers

- Bars and areas start at zero. Lines may zoom to the data range; say so with a
  visible axis.
- 4–6 ticks at round values; compact notation (`1.2k`, `3.4M`) via
  `Intl.NumberFormat({ notation: "compact" })`; units in the axis title, not
  on every tick.
- Dates: ticks follow the range (hours, days, months); the first tick of a new
  year or month shows the larger unit.
- Light gridlines only on the value axis; no chart borders, no 3D, no shadows.
- Direct labels at the end of lines and on bars; a legend only when labels
  collide, placed above the chart in series order.

## Interaction and accessibility

- Tooltips open on hover and on keyboard focus; arrow keys move between points;
  Esc closes. Tooltip shows the full-precision value, unit and date.
- Each chart has an accessible name, a one-sentence text summary of the
  takeaway, and a table fallback (toggle or `<details>`).
- SVG charts: `role="img"` + `aria-label` for static ones; focusable marks
  with labels for interactive ones.
- Respect `prefers-reduced-motion`: no animated draw-in; data updates swap
  in place.
- Large series (10k+ points): canvas or WebGL rendering, downsampling (e.g.
  LTTB) before render, and the table fallback paginated.

## Dashboard layout

- One screen at 1440×900 holds the primary set; a 12-column grid with KPI tiles
  of equal height in the first row.
- Priority runs top-left to bottom-right. Group related charts and share
  their time range and filters (one control bar for the whole board).
- KPI tile: label, value (tabular, ≥24px), delta with sign and arrow and the
  comparison period ("+4.2% vs last week"), target or sparkline.
- Exceptions only in colour: a tile turns warning/danger when it crosses a
  threshold the user can see.
- Show freshness and range at the top ("Last 30 days · updated 2 min ago").
- Every tile has a loading skeleton, an empty state ("No orders yet") and an
  error state of the same size.
