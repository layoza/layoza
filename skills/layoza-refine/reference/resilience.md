<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/harden.md, reference/adapt.md and reference/optimize.md. Modified by Layoza: condensed into one resilience guide in Layoza's own words; harden gains a measurable stress fixture run through `layoza qa` instead of a manual checklist; adapt is mapped to the 1440/768/375 viewports and mobile-menu test that `layoza qa` actually runs, with unmeasured widths named; optimize is driven by the lab LCP/CLS/INP that `layoza qa` / `audit_performance` measure, with a two-run noise rule and lab vs field separation; native-platform, print/email detail, library lists and impeccable hand-offs removed. -->

# Resilience: harden, adapt, optimize

These three have the strongest metrics in Layoza. Run the measured loop in
[SKILL.md](../SKILL.md); copy each `qa/browser-qa-report.json` to a named file
right after its run, because every run in the same folder overwrites it.

---

## Harden (works only with perfect data)

Upstream hardening is a manual checklist. Layoza makes the stress case
**measurable**: build a throwaway stress copy and run the same gate on it.

**1. Stress fixture.** Copy the page next to the original (so relative assets
resolve) as `<name>.layoza-stress.html`, and add before `</body>`:

```html
<script>/* Layoza stress fixture: never ship, delete after the run */
(() => {
  const LONG = " Donaudampfschifffahrtsgesellschaftskapitänsmütze";
  const q = new URLSearchParams(location.search);
  document.querySelectorAll("h1,h2,h3,button,a,label,th,td,li,figcaption,.price")
    .forEach((el) => { if (!el.children.length && el.textContent.trim()) el.textContent += LONG; });
  if (q.has("rtl")) document.documentElement.dir = "rtl";
})();
</script>
```

Run `layoza qa <name>.layoza-stress.html`, save the report as
`qa/stress-before.json`; after the fix run it again (`qa/stress-after.json`).
For RTL, serve the folder over http and run `layoza qa "http://localhost:<port>/<name>.layoza-stress.html?rtl"`.
Delete the stress copy when done.

**2. Gate on the stress copy:** `error` issues (`text-clip-or-bleed`,
`document-overflow`, `interactive-overlap`, `focus-obscured`) go to 0, or each
one left is named with its reason (e.g. a brand-mandated single-line logo).
Expect many on the first run: on a clean example page the fixture turned 5
errors into 29, mostly `text-clip-or-bleed` at 375px. `label-wrap` / `heading-crowding` warnings are expected under
absurd strings: read them and fix the ones a real translation (≈ +35%) would hit.

**3. Fixes that hold up:**
- Text: `min-width: 0` on flex/grid children; `overflow-wrap: anywhere` on
  user-supplied text; `text-overflow: ellipsis` or `line-clamp` only where the
  full value is available elsewhere (title attribute is not enough for touch).
- No fixed widths on text containers; buttons sized by padding.
- Logical properties (`margin-inline-start`, `padding-inline`, `inset-inline`)
  so RTL mirrors; mirror directional icons only.
- `Intl.NumberFormat` / `Intl.DateTimeFormat` for money and dates; a plural
  library or `Intl.PluralRules`, never `count + "s"`.
- Focus: `scroll-padding-top` for sticky headers (`focus-obscured`); visible
  `:focus-visible` (`focus_visible_styled`); 24×24px minimum targets
  (`target-size`); names on every control (`unlabeled-control`).
- States: every async control has loading (disable to stop double submit),
  error with retry that keeps the visitor's input, and empty. One failing widget
  never blanks the page.
- Content visible without JS where the framework allows; reveal animations
  start from visible (`text-hidden-after-scroll`).
- Custom drag/slider controls: pointer events with `touch-action`, and clear
  drag state on `pointercancel`, `lostpointercapture`, and `blur`.

**4. What `layoza qa` does not prove.** Offline/slow network, API error codes,
1000+ item lists, concurrent submits, and touch gestures are not measured.
Test what you can (devtools offline, a forced error), and list the rest as
untested in the report, naming what produced any evidence (emulated viewport,
synthesized input, Chromium only).

**Proof:** stress-after has 0 errors; real-page report has no regression;
`focus_visible_styled` and `heading_hierarchy_valid` true; `landmarks_present` true.

---

## Adapt (desktop-only feel, breaks on phones or tablets)

Adapting is rethinking for the context, not shrinking.

**What `layoza qa` measures:** 1440×900, 768×1024, 375×812, scrolling like a
visitor, plus the mobile menu (`qa/mobile-menu-open.png`, `menu-toggle-broken`,
`nav-unreachable`) and the first mobile viewport (`qa/mobile-hero.png`,
`primary-action-below-fold`). **Not measured:** 320px, landscape phones,
200% zoom, 4K. Check those by resizing a real browser and name them as
manual evidence or as untested.

**Moves:**
- Mobile: one column, primary action in the first viewport and within thumb
  reach, nav in a real drawer (`aria-expanded`, focus moved in, Esc closes,
  focus returns), wide header CTAs moved into the drawer, secondary content
  behind `<details>` or tabs, tables to labelled rows.
- Tablet: two columns or list + detail; touch-sized targets at desktop density.
- Desktop: use width with a max line length; do not stretch to 4K.
- Detect input, not width: `@media (pointer: coarse)` for larger targets,
  `@media (hover: hover)` around hover-only effects; nothing essential behind hover.
- Content-driven breakpoints (where *this* layout breaks), `clamp()` for fluid
  steps, container queries for components placed in different columns.
- `viewport-fit=cover` + `env(safe-area-inset-*)` for fixed bars.
- Responsive images (`srcset` + `sizes`; `<picture>` only for a different crop).
- Never hide core functionality on mobile; same information architecture
  everywhere.

**Brand:** keep the breakpoints recorded in `SITE-DNA.json` unless they are the
defect; add a breakpoint rather than moving the brand's.

**Proof:** 0 `document-overflow`, `text-clip-or-bleed`, `nav-unreachable`,
`menu-toggle-broken`; `label-wrap` / `heading-crowding` cleared at 375;
`primary-action-below-fold` cleared; `undersized_targets` not up. Open
`mobile-hero.png`, `mobile-menu-open.png`, `tablet.png`, and `desktop.png`:
the desktop must still look as intended.

---

## Optimize (slow, janky, shifting)

Fix the measured bottleneck, nothing else.

**Measure first.** `layoza qa` reports lab `lcp_ms`, `cls`, and an `inp_ms`
proxy (LaboratorySynthetic). Over MCP, `audit_performance`
`{target_url_or_file}` measures the same. Run twice; a change is real only when
both runs agree. A local `file` target has no network latency, so its LCP is
optimistic: for load work, measure an http URL (local server or the deployed
page). Field data (CrUX/RUM) is reported separately and never replaced by lab.

**By metric:**
- **LCP > 2.5s:** find the LCP element. Hero image: right-sized, modern format,
  `fetchpriority="high"`, never `loading="lazy"`. Hero text: preload only the
  one display face, inline critical CSS, defer the rest; remove render-blocking
  third-party scripts.
- **CLS > 0.1:** `width`/`height` or `aspect-ratio` on media and embeds; a
  metric-adjusted fallback `@font-face` for font swap
  ([engineering-lessons.md](../../layoza-art-direction/reference/engineering-lessons.md));
  never inject content above existing content after load.
- **INP > 200ms:** split long main-thread tasks, load non-essential JS later, debounce input
  and throttle scroll handlers, batch DOM reads before writes, move heavy work
  off the main thread.
- **Jank:** animate `transform`/`opacity`; bound blur/filter areas;
  `will-change` only during the animation; `content-visibility: auto` for long
  below-fold sections; stop offscreen loops.
- **Weight:** remove unused CSS/JS and dependencies, subset fonts, lazy-load
  below-fold media and heavy components.

**Never:** lazy-load above-the-fold content, trade accessibility or a visible
feature for a number, or add `will-change` everywhere.

**Proof:** the targeted metric moved in the right direction in both runs and
is within budget; no other metric worsened; screenshots show visual parity;
no `broken-image`.
