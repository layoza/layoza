<!-- Adapted from pbakaus/impeccable (Apache-2.0), commit 2f98774, reference/mode-operate.md, reference/mode-read.md, reference/operate.md (and the one-line Experience mode in SKILL.md). Modified by Layoza: comp/concept-seed/image-generation rounds removed; the four surfaces are mapped to Layoza modes A-D and to what `layoza qa` measures on each (with surface-specific budgets and the craft rules that are expected vs defects); Experience mode written out in full (upstream has one line); added mixed-surface pages and a brand-authority rule for Mode B/C. -->

# Surfaces: Persuade, Operate, Read, Experience

The surface names what the visitor's success looks like. Decide it per page
(a marketing site can contain a docs section and an app). It changes what the
world may touch and what `layoza qa` evidence matters most.

| Surface | Visitor succeeds when | The world owns | The world never owns |
| :--- | :--- | :--- | :--- |
| **Persuade** | they decide and act | the whole page (SKILL.md step 6) | the working form of the action |
| **Operate** | they finish a task | type, palette, density, one signature move | layout model, navigation, controls |
| **Read** | they understand something | the frame: masthead, rail, ground, openers, how code/tables are set | the reading column and reading order |
| **Experience** | they are inside the work | the stage around the work | the work itself (it leads) |

In Mode B/C the incumbent brand (`SITE-DNA.json`) still wins; the surface only
tells you where its expression is allowed to concentrate.

---

## Operate (apps, dashboards, admin, settings, tools)

A category-fluent user must trust it on sight. The failure is not flatness, it
is **strangeness without purpose**: invented affordances, display fonts on
labels, decorated buttons, choreography between them and the task.

- **The world lends four things only:** type, palette, density, one signature
  move named concretely (service health drawn as a transit line, a schedule in
  timetable rhythm). Navigation, layout model, and controls are the platform's
  standard ones: buttons, tabs, inputs, selects, sortable tables, `<dialog>`.
  A tool or instrument the audience uses (terminal, cockpit, ledger) returns
  only as those four, never as a costume (fake function-key bars, ruled paper
  as a table).
- **Colour does jobs:** a committed shell (tinted rail or header, second
  neutral layer for panels), a standard state set (interaction: hover,
  focus, pressed, selected, disabled, busy; outcome: info, success, warning, error), data series, selection.
  Accent marks actions and state, not decoration. A grey screen with one
  accent is the category default without its polish, not restraint.
- **Type:** one well-tuned family is often right; a fixed rem scale with a
  tight 1.125–1.2 ratio (no fluid display sizes); tabular numerals in data;
  prose still 65–75ch, tables may run wider.
- **Components:** no control ships with half its states: rest, hover, focus,
  pressed, disabled, busy, and failed are all designed. Skeletons over spinners inside content.
  Empty states that teach the next action. Overlays escape `overflow` clipping
  (`<dialog>`, popover API, `position: fixed`, portal). Inline before modal.
- **Motion:** 120–280ms (quick 120, standard 200, slow 280; see `layoza-product-ui` §12), state only; no page-load choreography.
- **Shell first:** first viewport = navigation, the work areas and their
  purpose, the data a user checks first shown in a realistic state (an alert
  firing, a service running degraded, a filter that matches nothing), then where the signature move lives. Real labels,
  units, plausible values; never placeholder tiles.

**Measure in `layoza qa`:** `inp_ms` ≤ 200 (interaction-heavy); `target-size`
and `interactive-overlap` = 0 (dense controls); `unlabeled-control` = 0 (icon
toolbars); `focus-obscured` = 0 (sticky toolbars); `document-overflow` /
`text-clip-or-bleed` at 768 and 375 (tables and side rails). Expected, not
defects: `craft-overused-font` on a system/sans UI face the brand records,
`primary-action-below-fold` on screens with no single primary action (say so).
**Self-check:** hide the product name. Is it still recognisably a working product of its
category, cleaner than the category standard and more distinctive than it?

---

## Read (docs, articles, guides, help, changelogs)

The reader's question leads; comprehension and wayfinding stay intact. Docs
read as docs, an article as an article.

- **The world owns the frame, the column stays calm.** Masthead, navigation
  rail, ground, title and section openers, and the setting of code, tables,
  and callouts commit fully to the world's palette, type, and signature move.
  The reading column inside them: a reading face, real contrast, ~60–75
  characters at ~16px+ body, nothing performing behind the text.
- Monospace for code, parameters, small labels; never running text or
  headings by default.
- Wayfinding: where am I (current item in nav, breadcrumbs or running head),
  contents for long pages, previous/next, headings in a strict hierarchy with
  anchor links.
- Worlds borrowed from print publishing transfer best: dictionaries and
  atlases, field guides, timetables, manuals, footnoted scholarship,
  newspaper section wayfinding.
- First viewport: where the reader stands, the title, an opening that answers
  or frames their question, the column, the way forward.

**Measure in `layoza qa`:** `max_line_length_ch` ≤ 75 and no `craft-line-length`;
no `craft-tight-leading`, `craft-justified-text`, `craft-tiny-text`,
`craft-all-caps-body`; `heading_hierarchy_valid` true; `landmarks_present`
true (nav + main); `contrast_failures` 0 including code blocks; `cls` ≤ 0.1
(web fonts on long pages). **Self-check:** can a reader tell where they are and
read comfortably? Cover the name: could this be any product's docs? Then the
frame has not taken the world. Does it read as an artifact before a document?
Then the world has leaked into the column; move it back to the frame.

---

## Experience (portfolios, galleries, showcases, exhibitions)

The visitor is inside the work. The interface recedes; the work leads from the
first viewport.

- **The work is the hero**, at real size and quality, not a thumbnail behind a
  headline. Navigation is quiet and always findable; captions and credits are
  set like a catalogue, not marketing copy.
- **Pacing over sections:** a sequence with rhythm (full-bleed, pairs, a pause
  of text, detail crops) instead of a uniform grid of equal cards.
- **Motion serves looking:** transitions between works preserve continuity
  (shared element, crossfade); no motion on top of the work itself unless the
  work is motion. Full reduced-motion path.
- **Respect the asset:** correct aspect ratios, no forced crops of artwork,
  zoom/full view for detail, alt text that describes the work for people who
  cannot see it.
- Persuasion still exists (commission, buy, contact) but sits after the work
  earns it, and stays one tap away.

**Measure in `layoza qa`:** `lcp_ms` ≤ 2500 with a large hero asset (the usual
failure: serve right-sized `srcset`, `fetchpriority="high"` on the first work);
`cls` ≤ 0.1 (every media element has dimensions or `aspect-ratio`);
`missing_alt_count` 0; `broken-image` 0; `text-hidden-after-scroll` 0 (captions
revealed by animation); `nav-unreachable` 0 with a minimal header.
`primary-action-below-fold` is acceptable here when the first viewport is the
work; say so in the report.

---

## Persuade (landing pages, marketing, pricing, campaigns)

Covered by SKILL.md step 6 and [world.md](world.md). Measure:
`primary-action-below-fold` cleared at 375 and 1440, `lcp_ms`, contrast of copy
on imagery, and the memory test.

## Mixed pages

A product site often has a Persuade home, Read docs, and an Operate app. Keep
one world across them; let each surface take only what its row in the table
allows. Run `layoza qa` on one representative page per surface.
