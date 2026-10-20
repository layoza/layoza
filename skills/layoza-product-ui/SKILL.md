---
name: layoza-product-ui
description: >-
  Product and application UI: dashboards, SaaS and web-app screens, admin
  panels, settings, data tables, forms, data visualisation, and mobile app
  UI. Covers component-library authority (shadcn on Radix / Base UI / React
  Aria), shell and navigation, density modes, numbers, the per-view state
  matrix, overlays, dark mode and tool motion. Load instead of
  layoza-art-direction's world-building whenever the visitor's job is to
  finish a task rather than be persuaded.
---
<!-- Layoza original, written in its own words. Ideas adapted (no text copied) from vercel-labs/web-interface-guidelines (MIT), pbakaus/impeccable operate/harden references (Apache-2.0, modified), shadcn-ui/ui skill docs (MIT), the W3C WAI-ARIA Authoring Practices Guide, Material Design density and touch-target figures, Apple HIG's 44pt target, and Stephen Few's dashboard principles (concepts only). See THIRD_PARTY_NOTICES.md, "Product UI ideas". -->

# Layoza: Product UI

A product screen succeeds when a practised user finishes the task fast and a
new user never gets lost. Clarity and density come first; the brand lends
type, palette, density and one signature move, never the controls.

| Reference | Load when |
| :--- | :--- |
| [reference/tables-forms.md](reference/tables-forms.md) | Building any table, list, form or settings page |
| [reference/data-viz.md](reference/data-viz.md) | Any chart, KPI tile, sparkline or dashboard |
| [../layoza-art-direction/reference/surfaces.md](../layoza-art-direction/reference/surfaces.md) | The "Operate" surface: what the brand may own |
| [../layoza-animations/reference/motion-system.md](../layoza-animations/reference/motion-system.md) | Motion tokens (§1 "Tool / product UI", §4 loaders) |
| [../layoza-accessibility/SKILL.md](../layoza-accessibility/SKILL.md) | WCAG 2.2 details, live regions, grid patterns |
| [../layoza-performance/SKILL.md](../layoza-performance/SKILL.md) | Optional, only when the user requests speed diagnostics |

## 0. Authority (decide first)

1. **The user's prompt wins**: named library, chart tool, density or style.
2. **The project's component library wins next.** Run `layoza inspect . --write`
   (MCP `inspect_project`, `discover_components`) and read
   `COMPONENT-INVENTORY.json`. Use its Button, Input, Table, Dialog; never add a
   second library for one screen.
3. **Compose primitives, never fork or re-skin them.** Build a `DataTable` from
   the library's `Table` + `Checkbox` + `DropdownMenu`; change looks through
   tokens and variants, not by copying a primitive's source and editing it.
4. **No library at all:** native HTML (`<dialog>`, `<details>`, `<table>`,
   `popover`, form controls) plus tokens. Pick a headless library only when the
   stack and brief call for one; do not default to any single vendor.
5. **Mode B/C:** match the incumbent density, table and form components
   exactly ([`layoza-extend-website`](../layoza-extend-website/SKILL.md)).

### shadcn detection

If `components.json` exists, the project uses shadcn/ui. Read its `style`,
`tailwind.css`, `aliases.ui` and `iconLibrary`, then confirm the primitive
base from `package.json`:

| Dependency present | Base | Consequence |
| :--- | :--- | :--- |
| `radix-ui` or `@radix-ui/react-*` | Radix | `asChild` composition; `data-state` styling |
| `@base-ui-components/react` or `@base-ui/react` | Base UI | `render` prop composition; `data-*` state attributes |
| `react-aria-components` | React Aria | render props; `data-hovered` / `data-focus-visible` |

Add missing pieces with the project's own CLI (`npx shadcn@latest add <name>`)
so they land in `aliases.ui` with the right base. Never paste a component built
for another base.

## 1. Shell and information architecture

- **Side nav** 240–280px wide, collapsing to a 56–64px icon rail (icons get
  `aria-label` and a tooltip); below 768px it becomes a drawer or bottom tabs.
  Persist the collapsed state.
- **Top bar** 48–64px: workspace switcher, search, notifications, account.
  Sticky; content below it gets `scroll-margin-top` equal to its height.
- **Breadcrumbs** once the hierarchy is deeper than 2 levels. The last crumb is
  the current page (`aria-current="page"`), not a link.
- **Command palette** (Cmd/Ctrl+K) once there are 10 or more routes or
  frequent actions; it searches routes, records and actions, and shows each
  action's shortcut.
- **State lives in the URL**: filters, tabs, sort, page, selected record and
  open drawer. Back/forward, reload and share must restore the exact view.
- **One H1 per view**, landmarks `nav`/`main`/`aside`, and a skip link.
- **Page header**: title, useful context and primary action. On phones, judge
  the space these consume against the working rows below them. Compact summary
  counts should support the task rather than push it out of view. For operations
  screens, put time-critical risks and shortages before secondary reports or
  collections, while retaining quantities, units, location and demo status.

## 2. Density modes

| Mode | Row height | Control height | Cell padding (x) | Use |
| :--- | :--- | :--- | :--- | :--- |
| Comfortable | 48px | 40px | 16px | Occasional users, touch, consumer apps |
| Default | 40px | 36px | 12px | Most SaaS screens |
| Compact | 32px | 28–32px | 8px | Power users, ops, finance, logs |

- 8px grid with a 4px sub-grid for icon gaps and tight stacks.
- Ship density as tokens (`--row-h`, `--control-h`, `--cell-px`) switched by
  one attribute (`[data-density="compact"]`), so every table and form follows.
- **Hit areas never shrink with density:** at least 24×24 CSS px on web
  (WCAG 2.5.8), 44×44pt on iOS, 48×48dp on Android. A 28px compact control
  extends its hit area with padding or a pseudo-element; neighbouring targets
  never overlap.
- Default to the comfortable mode on touch (`pointer: coarse`).

## 3. Type and numbers

- Fixed rem scale, ratio 1.125–1.2, base 14–16px (16px for consumer apps). No
  fluid `clamp()` display sizes inside the app shell.
- `font-variant-numeric: tabular-nums` on every column, KPI and timer.
- Right-align numbers and their headers; left-align text; centre only icons
  and status dots.
- A no-break space between value and unit (`12&nbsp;GB`, `4&nbsp;min`).
- Format with `Intl.NumberFormat`, `Intl.DateTimeFormat` and
  `Intl.RelativeTimeFormat` in the user's locale; never hand-build
  separators or currency symbols.
- Relative time ("3 min ago") shows the absolute time on hover and focus
  (`<time datetime>` + `title` or tooltip); switch to a date after 7 days.
- Decimals are consistent within a column (2 for currency, 0–1 for
  percentages); compact notation (`1.2M`) only in KPIs and axes, full value in
  the tooltip.
- Negative values carry a sign and colour, never colour alone.

## 4. Tables (detail: [reference/tables-forms.md](reference/tables-forms.md))

- Sticky header; sticky first column when the table scrolls horizontally.
- Sortable headers are `<button>`s inside `<th aria-sort>`; one sort at a time
  unless the brief needs multi-sort.
- Truncate long text with an ellipsis and keep the full value reachable
  (tooltip, `title`, expanding row or detail drawer).
- Virtualise above ~200 rows, or paginate at 25/50/100; keep the header and
  row count stable.
- Bulk-select bar appears in place of the toolbar with "N selected", the
  bulk actions and "Clear".
- Row actions reachable by keyboard: focusable row, Enter opens, a menu button
  per row; shortcuts listed in the palette.
- Below 768px: a card list or priority columns (keep 2–3, the rest in the
  detail view). No table may force the page to scroll sideways at 375px.

## 5. Forms (detail: [reference/tables-forms.md](reference/tables-forms.md))

- Visible label above every field; placeholder shows an example, never the
  label.
- Validate on blur and on submit, never on each keystroke; clear an error as
  soon as the field becomes valid.
- On a failed submit, focus the first invalid field; each error sits directly
  under its field, linked with `aria-describedby`, plus a summary for long forms.
- Submit stays enabled until the request starts, then shows busy
  (`aria-busy`, label "Saving…") and ignores repeat clicks.
- Inputs at least 16px on mobile (prevents iOS zoom); never block paste.
- Set `autocomplete`, `inputmode`, `type` and `name` on every field.
- **Settings pages state their save model**: autosave (show "Saved" with a
  timestamp, toggles apply immediately) or explicit save (sticky Save/Cancel
  bar, warn on leaving with unsaved changes). Never mix both on one page.

## 6. State matrix (required for every view)

Design and build each row before calling a view done:

| State | What it shows |
| :--- | :--- |
| Empty: first use | What this view is for and the one action that fills it |
| Empty: no results | The filter that excluded everything and "Clear filters" |
| Empty: cleared | Confirmation that the user's own action emptied it ("All done") |
| Loading | Skeleton shaped like the final layout |
| Partial | What loaded, plus which part failed and a retry for that part |
| Error | Plain cause, what still works, retry, and a support reference |
| Offline | Banner, cached data marked stale, queued actions listed |
| Permission denied | Who can grant access and a request button; no blank page |
| Sparse | 1–3 items still look intentional |
| Dense | 10,000 rows, 80-character names, 7-digit numbers do not break layout |

- Loaders: wait ~150–300ms before showing anything, then keep it at least
  ~300–500ms so it never flickers (same numbers as the motion system, §4).
- Skeletons reserve the final dimensions: CLS stays 0.
- Announce async results in an `aria-live="polite"` region ("24 results",
  "Saved"); use `assertive` only for errors that block the task.
- Destructive actions get undo (toast for 5–10s) when reversible, or a confirm
  dialog naming the object ("Delete 3 invoices?") when not.
- Every action answers within 100ms visually and finishes within 500ms, or
  goes optimistic with rollback on failure.

## 7. Dashboards

Principles in the spirit of Stephen Few's dashboard work:

- The primary set fits one screen at 1440×900 without scrolling; secondary
  detail sits behind a click.
- The most important number sits top-left; reading order follows priority.
- Every KPI has context: a comparison (vs last period), a target, or a trend
  sparkline. A bare number is not a KPI.
- Colour only marks exceptions (over budget, failing, late); healthy values
  stay neutral.
- No gauges, dials, 3D, or pies with more than 5 slices.
- Bars start at zero; label lines and bars directly instead of using a legend
  where space allows.
- Show the data's freshness ("Updated 2 min ago") and its time range.

## 8. Data visualisation (detail: [reference/data-viz.md](reference/data-viz.md))

- Pick the chart from the question, not the library's gallery: trend → line,
  compare → bar, part-to-whole ≤5 → stacked bar, distribution → histogram,
  correlation → scatter (full table in the reference).
- At most 7 categorical hues, checked for colour-vision deficiency; beyond 7,
  group as "Other" or use small multiples.
- Every chart has a table fallback (toggle or `<details>`) and a text summary.
- Tooltips open on keyboard focus as well as hover; points are reachable with
  arrow keys or via the table.
- Axes: units in the title, formatted ticks via `Intl`, 4–6 ticks, no
  rotated labels (abbreviate or turn the chart horizontal).
- Use the charting library the project already has; when none exists, pick
  per brief (SVG/CSS for a few static charts, a library for interaction).

## 9. Overlays

- Use `<dialog>` / `showModal()`, the `popover` attribute, or the library's
  portal so overlays escape `overflow: hidden` and stacking contexts.
- Modals trap focus, close on Esc and a visible close button, and return focus
  to the trigger. Popovers close on Esc and outside click.
- **Drawers for in-context edits** (the list stays visible); **modals only to
  block** (a decision that must happen before anything else).
- Never stack more than one modal; a second step replaces the first's content.
- Tooltips hold labels, never actions or essential information.

## 10. Mobile app UI

- Respect safe areas: `viewport-fit=cover` plus
  `env(safe-area-inset-*)` padding on bars and sheets.
- 3–5 bottom tabs with icon and label; more destinations go to a "More" tab.
- The primary action sits in the bottom third, within thumb reach (bottom
  bar, FAB or sticky footer button).
- Every gesture (swipe to delete, pull to refresh, drag to reorder) has a tap
  alternative: a menu item, a button, or move up/down actions.
- Sheets over full-screen modals for short tasks; full screen for long forms.

## 11. Dark mode

- Set `color-scheme: light dark` (or the active one) on `:root` so native
  controls, scrollbars and `<select>` menus follow; give `<select>` and
  `<option>` explicit background and text tokens on Windows.
- Match `<meta name="theme-color">` to the shell surface for each scheme.
- Raise elevation with lighter surfaces, not shadows; avoid pure #000 under
  long-read text.
- Hover, selected and focus need stronger contrast in dark than in light
  (focus ring ≥3:1 against both the control and its surface).
- Recheck every state token and chart hue in both schemes.

## 12. Motion for tools

Personality "Tool / product UI" from
[motion-system.md §1](../layoza-animations/reference/motion-system.md):
120 / 200 / 280ms, 0% overshoot, cubic ease-out, no decorative motion.

- 120ms: hover, press, toggle. 200ms: menus, popovers, tabs. 280ms: drawers,
  sheets, dialogs.
- No page-load choreography, parallax or scroll reveals inside the app.
- Motion states change (row inserted, item removed, panel opened), and is
  interruptible. `prefers-reduced-motion: reduce` swaps movement for an
  opacity change or none.

## 13. A point of view, not the default kit

A screen that looks like an untouched component-library demo is the category
default, not restraint. Keep density and clarity first, then commit to **one
distinctive move** that fits the brand:

- a type choice (a characterful grotesk or a well-set serif for headings with
  a neutral UI face for controls);
- a signature data treatment (status drawn as a timeline, a custom sparkline
  style, a heat strip in the brand hue);
- a precise accent colour used only for action and selection, with a tinted
  neutral shell instead of pure grey;
- empty-state illustrations drawn from the brand's world.

Self-check: hide the logo. Is it still a credible product of its category,
cleaner than the usual one, and recognisably this brand?

## 14. Done means

- Every view renders every row of its state matrix (§6), checked in the
  browser, not only in code.
- `layoza qa <html-file-or-url>` (or
  `node scripts/browser-qa.mjs --file <path>` / `--url <url>`) reports 0
  errors, with the Operate metrics from
  [surfaces.md](../layoza-art-direction/reference/surfaces.md) checked.
  Add `<meta name="layoza-surface" content="app">` so landing-page hero rules
  (hero-metric, hero-art-scale, display-scale) stand down; QA also detects an
  app shell (data table plus nav rail or tab list in the first viewport).
- Tables and forms are usable at 375px with no page-level horizontal scroll.
- A keyboard-only walk-through of the main task (Tab, Enter, Esc, arrows)
  succeeds with a visible focus ring at every step.
- Screenshots of the dense and empty states were reviewed, not just the happy
  path.
