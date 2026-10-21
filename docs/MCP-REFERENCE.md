# Layoza MCP Server Reference (`layoza mcp`)

Layoza includes a zero-dependency Rust stdio JSON-RPC 2.0 MCP server in [`crates/layoza-mcp/src/lib.rs`](../crates/layoza-mcp/src/lib.rs).
The tool tables below are generated from `tool_definitions()` in that file; when they disagree, the code wins.

## Launching the server

```bash
layoza mcp
```

Every path argument must resolve inside the approved project root (the
directory the server was started in, or the parent of an enclosing `.agents` folder). Tools never write into the project
unless the call passes `write_approved: true`. Page content returned by
`inspect_site` and `capture_screenshots` is untrusted data.

## Design history (`avoid_recent`)

Layoza keeps a small user-level ledger so new work does not repeat your recent
looks: `~/.layoza/history.jsonl` (or `$LAYOZA_HOME/history.jsonl`). Each line
holds only a project hash, the date, the picked direction name, palette hexes,
type families, motif keywords and mechanic ids. It never stores paths, page
copy or secrets.

- **Written** when the release gate audits a project whose
  `.layoza/direction.md` has a `PICKED:` line (not `PICKED: existing ...`).
  That covers `layoza verify` / `layoza audit` and the MCP tools that run the
  gate (`capture_screenshots` with `write_approved`, `compare_before_after`,
  `audit_accessibility`, `audit_performance`, `verify_functional`,
  `export_report`, `safe_rollback` create). An identical repeat audit of the
  same project adds no line; the file is compacted to its newest 200 lines
  past 256 KB.
- **Read** by `get_design_system` in `new_website` and `full_redesign` modes,
  which returns an `avoid_recent` block (last 8 projects: directions,
  palettes, fonts, motifs, mechanics, rule). Extend and improve modes follow
  the existing site and return no block.
- **Checked** by the gate: `DESIGN-REPEATS-RECENT` (warning) when the pick
  shares at least two of palette, type, motifs and mechanics (one of them
  palette or type) with another recent project. Add `REPEAT-OK: <reason>` to
  `direction.md` when the user asked for the same look.
- `LAYOZA_NO_HISTORY=1` turns the ledger off (nothing read or written).

## Project look ledger and `uniqueness`

`get_design_system` (new and redesign modes) and `deal_mechanics` also read and append the project
ledger `<project>/.layoza/looks.jsonl`. Both take `window` (1-200, default 36: how many recent runs to
compare) and `ledger` (default true). Each response carries:

- `uniqueness`: `ledger`, `window`, `entries_compared`, `slots` (nav, hero, sections, footer, motion,
  palette and so on, each with `id` and `status` fresh or repeat), `fresh`, `repeats`, `score` (0-100)
  and `verdict`.
- `ledger`: always present, one of `written <path>`, `not written (<error>)` or `off (<reason>)`.

When a niche's pool is exhausted inside the window (for example, a funeral home has five calm navs),
the dealer falls back to the least recently used option and reports it as a repeat.

## QA scope arguments

`capture_screenshots`, `audit_accessibility`, `verify_functional` and `audit_performance` take
`full` (run every browser pass) and `allow` (list of pass names to run); responses include `qa_mode`
with the mode, the passes run and `skipped_passes`. `audit_accessibility` and `verify_functional`
also take `strict` (gate warnings, such as an unapproved direction, become errors). The server reads `LAYOZA_QA_TIMEOUT_SECS`,
`LAYOZA_PLAYWRIGHT_DIR` and `LAYOZA_QA_ENGINE=python`. Live URLs must resolve to a public address;
private, link-local and metadata ranges are refused, while `localhost` stays allowed.

## Tools (24 listed) and prompts (7)

`tools/list` returns 24 tools. `recommend_layout` is deprecated and no longer listed, but still answers
`tools/call` for older clients; use `get_design_system` instead. Every call is checked against the tool's
schema first: unknown arguments, wrong types, out-of-range numbers, empty briefs and over-long strings
are rejected with a clear error. `prompts/list` returns seven workflow prompts: `art-direct`, `critique`,
`anti-slop-review`, `extend-site`, `improve-site`, `redesign`, `study`.

`deal_mechanics` returns, for a new site, `nav` and `footer` slots (page furniture, never a signature) and
hero refs carry `hero_material` and `look_family`. A given `seed` rotates among the in-niche heroes that fit;
directions never share a hero look family.

`capture_screenshots` takes `compare_with` (an earlier `browser-qa-report.json` in the project) and returns
`look_compare` with the verdict `re-skin`, `near-duplicate` or `distinct`. The CLI equivalent is
`layoza qa --compare <a.json> <b.json>`, which exits 2 on a re-skin.

| Tool | Writes to disk | Summary |
| :--- | :--- | :--- |
| [`inspect_project`](#inspect_project) | No | Read-only inspection of a local project directory to extract Site DNA, routes, CSS tokens, framework, and component inventory. |
| [`inspect_site`](#inspect_site) | No | Loads a live http(s) URL in headless Chromium (desktop/tablet/mobile) and returns its rendered Site DNA (palette, fonts, type scale, radii, CSS custom properties, framework and motion hints) plus measured accessibility, layout, and lab Core Web Vitals. |
| [`extract_site_dna`](#extract_site_dna) | Only with `write_approved: true` | Extracts Site DNA and writes DESIGN.md, BRAND.md, MOTION.md, SITE-DNA.json, COMPONENT-INVENTORY.json, and DESIGN-DECISIONS.md (requires write_approved=true). |
| [`get_design_system`](#get_design_system) | No | Design system for a brief or a project. |
| [`recommend_layout`](#recommend_layout) (deprecated, unlisted) | No | Recommends layout archetypes, typography, color palettes, and 3 distinct creative directions for Modes A/B/C/D. |
| [`recommend_animation`](#recommend_animation) | No | Recommends accessible GSAP, Motion, WAAPI, Three.js/R3F, or conditional Lenis animation recipes with reduced-motion and cleanup guarantees. |
| [`discover_components`](#discover_components) | No | Discovers existing project UI components and generates framework-native component patterns. |
| [`capture_screenshots`](#capture_screenshots) | Only with `write_approved: true` | Loads a local HTML file or http(s) URL in headless Chromium at 1440x900, 768x1024 and 375x812, waits for motion to settle, saves full-page, 1:1 mobile-hero and menu-open screenshots, and returns a compact summary: verdict and counts, lab LCP/CLS/INP proxy and minimum contrast first, then findings deduplicated across viewports (one line per rule with the viewports and a short example), then screenshot paths. |
| [`compare_before_after`](#compare_before_after) | No | Compares the current audit of `path` against the audit report saved when `safe_rollback` created `baseline_id`, separating intended changes from regressions (new error rules or accessibility regressions). |
| [`audit_accessibility`](#audit_accessibility) | No | Audits WCAG 2.2 AA accessibility compliance, reporting automated violations separately from manual review requirements. |
| [`audit_performance`](#audit_performance) | No | Optional, only when explicitly requested. Speed diagnostics never block the design gate. Evaluates Core Web Vitals targets (LCP <= 2.5s, INP <= 200ms, CLS <= 0.1). |
| [`verify_functional`](#verify_functional) | Only with `write_approved: true` | Runs full release-gate verification and optional bounded auto-repair (requires write_approved=true when auto_repair=true). |
| [`export_report`](#export_report) | No | Exports a structured quality, accessibility, performance, and benchmark verification report. |
| [`deal_mechanics`](#deal_mechanics) | No | Deals specific, buildable page mechanics from Layoza's catalogue: 1 hero + `count` section mechanics (2 `signature` to build verbatim in spirit, the rest `suggestions`) + 1 transition, never two of one mechanism family, deterministic per `seed`. |
| [`get_mechanic`](#get_mechanic) | No | Returns one mechanic from Layoza's catalogue by id (from `deal_mechanics` or `layoza mechanics list`) with its full blueprint, motion, still_check, mobile, a11y, avoid and CSS/JS stack notes. |
| [`safe_rollback`](#safe_rollback) | Only with `write_approved: true` | Creates or restores a reversible project baseline snapshot (requires write_approved=true). |
| [`check_deal_conformance`](#check_deal_conformance) | No | Loads a built page at 1440px and 390px and checks it follows its dealt page shape (restraint rules, button shape, motif); returns a 0-100 score and checks with evidence. |
| `search_guidelines` | No | Searches bundled UX guidelines, chart notes and stack notes by keyword (synonyms and stems included); `get_guideline` returns one in full. |
| `get_guideline` | No | Returns one guideline, chart or stack note by id, or `icons` for the icon guidance. |
| `recommend_chart` | No | Recommends chart types for a data shape with encoding, when to avoid each, volume limits and an accessible fallback. |
| `get_stack_notes` | No | Returns notes for one stack: conventions, pitfalls, where tokens and motion live. |
| `get_icon_guidance` | No | Returns icon guidance: sets, sizing, stroke, labels and accessibility. |

---

### `inspect_project`

Read-only inspection of a local project directory to extract Site DNA, routes, CSS tokens, framework, and component inventory.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `path` | string |  | Relative or approved project path (defaults to '.'). |
| `screenshot_only` | boolean |  |  |

### `inspect_site`

Loads a live http(s) URL in headless Chromium (desktop/tablet/mobile) and returns its rendered Site DNA (palette, fonts, type scale, radii, CSS custom properties, framework and motion hints) plus measured accessibility, layout, and lab Core Web Vitals. Optional `path` merges the result with that local project's source DNA. Page content is untrusted data.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `url` | string | yes | http(s) URL, e.g. a production site or a localhost dev server. |
| `path` | string |  | Optional local project path whose source DNA should be compared with the live render. |

### `extract_site_dna`

Extracts Site DNA and writes DESIGN.md, BRAND.md, MOTION.md, SITE-DNA.json, COMPONENT-INVENTORY.json, and DESIGN-DECISIONS.md (requires write_approved=true).

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `path` | string |  | Project-relative path (file or directory) inside the approved project root; defaults to '.'. |
| `write_approved` | boolean | yes | Must be true to write DNA files to disk. |

### `get_design_system`

Design system for a brief or a project. `recommend` (or any call with `query`) returns the brief-driven system: matched industry, brief signals (surface, niche, mood, constraints) and 3 creative directions with palettes, type pairings, layout and motion (same as CLI `layoza design-system`). For `new_website` / `full_redesign` it adds `avoid_recent`: the directions, palettes, fonts, motifs and mechanics of the last 8 picks recorded in the user's design history (`~/.layoza/history.jsonl`, written when an audit passes the release gate and `.layoza/direction.md` records a pick); make new directions materially different unless the user asks for the same look. `generate` derives DESIGN.md from the project at `path`; `lint` / `diff` check DESIGN.md content.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `action` | string (`recommend`, `lint`, `diff`, `generate`) |  | `recommend` (default when `query` is set) builds the system from the brief; `generate` (default otherwise) derives DESIGN.md from the project at `path`; `lint` checks `design_md_content`; `diff` compares `design_md_content` (before) with `after_design_md_content`. |
| `query` | string |  | The design brief, e.g. `seafood restaurant on the harbour with daily catch menu`. |
| `mode` | string (`new_website`, `extend_existing`, `improve_existing`, `full_redesign`) |  |  |
| `variant` | integer 0–1000 |  | 0 (default) is the stable answer for this brief. Another number re-deals the directions' fonts, palettes and mechanics for the same brief; use it when the user wants different options. |
| `path` | string |  | Project-relative path (file or directory) inside the approved project root; defaults to '.'. |
| `design_md_content` | string |  |  |
| `after_design_md_content` | string |  |  |

### `recommend_layout`

Recommends layout archetypes, typography, color palettes, and 3 distinct creative directions for Modes A/B/C/D.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `query` | string | yes |  |
| `mode` | string (`new_website`, `extend_existing`, `improve_existing`, `full_redesign`) |  |  |
| `design_variance` | integer 0–10 |  |  |
| `motion_intensity` | integer 0–10 |  |  |
| `visual_density` | integer 0–10 |  |  |

### `recommend_animation`

Recommends accessible GSAP, Motion, WAAPI, Three.js/R3F, or conditional Lenis animation recipes with reduced-motion and cleanup guarantees.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `category` | string (`all`, `<motion categories>`) |  | Restrict recipes to one motion category; omit or `all` for every category. |
| `recipe` | string |  | Return exactly one recipe by id (e.g. `hero-gsap-masked-split`), regardless of intensity. |
| `motion_intensity` | integer 0–10 |  | Motion dial 0-10 (default 6); 0 means motion off and returns no recipes. |

### `discover_components`

Discovers existing project UI components and generates framework-native component patterns.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `path` | string |  | Project-relative path (file or directory) inside the approved project root; defaults to '.'. |
| `framework` | string |  |  |
| `title` | string |  |  |

### `capture_screenshots`

Loads a local HTML file or http(s) URL in headless Chromium at 1440x900, 768x1024 and 375x812, waits for motion to settle, saves full-page, 1:1 mobile-hero and menu-open screenshots, and returns a compact summary: verdict and counts, lab LCP/CLS/INP proxy and minimum contrast first, then findings deduplicated across viewports (one line per rule with the viewports and a short example), then screenshot paths. The full detail is written to `capture-full.json` next to the screenshots (`full_detail`); pass `detail: "full"` to get it inline. Screenshots and the report go to a per-target scratch directory outside the project by default. For a local file with `write_approved: true` they are written to `<file's directory>/qa/` instead, and the release-gate audit of that directory (source plus this live report) is returned.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `target_url_or_file` | string | yes | http(s) URL, or an HTML file / directory (uses its index.html) inside the project. |
| `write_approved` | boolean |  | Local files only: true writes screenshots and the report into `<dir>/qa/` inside the project and returns the merged release-gate audit. Default false writes to a scratch directory. |
| `sketch` | boolean |  | true returns only the screenshot paths per viewport plus error/warning counts (no findings, metrics, Site DNA or release-gate audit); the full report is still saved as `browser-qa-report.json` in `screenshot_dir`. Use it to show direction sketches cheaply. Default false. |
| `detail` | string (`summary`, `full`) |  | "summary" (default) returns the compact result and writes the full detail to `capture-full.json`; "full" returns the full verbose report inline (quality, every finding, the live Chromium report and, with `write_approved`, the release-gate audit). |

### `compare_before_after`

Compares the current audit of `path` against the audit report saved when `safe_rollback` created `baseline_id`, separating intended changes from regressions (new error rules or accessibility regressions). Run `capture_screenshots` first so the current audit includes live browser results.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `path` | string |  | Project-relative path (file or directory) inside the approved project root; defaults to '.'. |
| `baseline_id` | string | yes |  |
| `pixel_diff_ratio` | number |  | Optional pixel diff ratio you measured yourself. Echoed back as supplied; Layoza does not diff screenshots. |
| `intended_changes` | array of string |  |  |

### `audit_accessibility`

Audits WCAG 2.2 AA accessibility compliance, reporting automated violations separately from manual review requirements.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `path` | string |  | Project-relative path (file or directory) inside the approved project root; defaults to '.'. |

### `audit_performance`

Optional, only when explicitly requested. Speed diagnostics never block the design gate. Evaluates Core Web Vitals targets (LCP <= 2.5s, INP <= 200ms, CLS <= 0.1). Pass `target_url_or_file` to measure them in headless Chromium, or pass lab values you measured elsewhere. Unmeasured metrics are reported as null, never as passing.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `path` | string |  | Project-relative path (file or directory) inside the approved project root; defaults to '.'. |
| `target_url_or_file` | string |  | http(s) URL or project-relative HTML file to measure live. |
| `lab_lcp_ms` | number |  |  |
| `lab_inp_ms` | number |  |  |
| `lab_cls` | number |  |  |

### `verify_functional`

Runs full release-gate verification and optional bounded auto-repair (requires write_approved=true when auto_repair=true).

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `path` | string |  | Project-relative path (file or directory) inside the approved project root; defaults to '.'. |
| `auto_repair` | boolean |  |  |
| `write_approved` | boolean |  |  |

### `export_report`

Exports a structured quality, accessibility, performance, and benchmark verification report.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `include_benchmarks` | boolean |  |  |

### `deal_mechanics`

Read-only. Deals specific, buildable page mechanics from Layoza's catalogue: 1 hero + `count` section mechanics (2 `signature` to build verbatim in spirit, the rest `suggestions`) + 1 transition, never two of one mechanism family, deterministic per `seed`. Each entry has a blueprint (clamp() sizes, states), scroll-progress motion, still_check, mobile, a11y, avoid and CSS/JS paths. New sites (`new_website`, `full_redesign`): the brief drives niche, mood and intensity. Existing sites (`extend_existing`, `improve_existing`): the site's motion language caps intensity and stack (from `existing_motion_intensity`, or by inspecting `path`), so nothing dealt restyles the site. Mechanics built around another trade's object are excluded; `fit` gives each dealt mechanic's fit class and reason.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `brief` | string | yes | The page brief, e.g. `neighbourhood bakery with a daily bread board`. |
| `niche` | string |  | Optional niche hint (store, restaurant, local-business, saas, dashboard, agency, kinetic, 3d, editorial, portfolio, finance, health, education, events, real-estate, travel, nonprofit, docs, landing, professional, legal, museum, devtools, memorial, body-art, civic, venue, camp, winery, home-services, any); added after niches the brief names. |
| `mood` | string |  | Optional comma-separated moods (calm, warm, playful, bold, luxe, technical, editorial, organic, brutal, dark, bright, trustworthy, experimental, minimal). |
| `intensity` | string (`calm`, `bold`, `wild`) |  | Motion tier. New site: used only when the brief names none. Existing site: may only lower the site's own level. |
| `count` | integer 2–10 |  | Section mechanics to deal (default 5). |
| `seed` | integer 0– |  | Deterministic seed; defaults to a hash of the brief. |
| `mode` | string (`new_website`, `extend_existing`, `improve_existing`, `full_redesign`) |  |  |
| `existing_motion_intensity` | integer 0–10 |  | Existing site's motion dial (0-3 calm, 4-7 bold, 8-10 wild). Without it, an existing-site deal inspects `path`. |
| `avoid` | array of string |  | Mechanic ids used recently (e.g. on the last projects); never dealt again. |
| `path` | string |  | Project-relative path (file or directory) inside the approved project root; defaults to '.'. |

### `get_mechanic`

Read-only. Returns one mechanic from Layoza's catalogue by id (from `deal_mechanics` or `layoza mechanics list`) with its full blueprint, motion, still_check, mobile, a11y, avoid and CSS/JS stack notes.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `id` | string | yes | Mechanic id, e.g. `menu-chalkboard-daily-board`. |

### `safe_rollback`

Creates or restores a reversible project baseline snapshot (requires write_approved=true). `create` without `files` snapshots the project's UI sources (html/css/js/ts/tsx/jsx/vue/svelte/astro/liquid/php, theme JSON, DESIGN.md/BRAND.md/MOTION.md), skipping node_modules, .git, target, dist, build, .layoza and hidden directories, capped at 2000 files / 1 MB per file / 32 MB total; skipped paths are listed in the result. Explicit `files` that cannot be snapshotted are an error, as is a baseline that would capture nothing. `rollback` is an error if it restores no files.

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `action` | string (`create`, `rollback`) | yes |  |
| `baseline_id` | string | yes |  |
| `files` | array of string |  | Optional project-relative files or directories to snapshot instead of the default UI-source discovery. |
| `path` | string |  | Project path whose audit is saved with the baseline for `compare_before_after` (defaults to '.'). |
| `write_approved` | boolean | yes |  |

---

## Resources (4)

- `layoza://design-system/industries`: industry design profiles (typography, palettes, layouts, dials).
- `layoza://design-system/styles`: UI style families with characteristics and provenance.
- `layoza://motion/recipes`: GSAP, WAAPI, Motion, Three.js and conditional Lenis recipes.
- `layoza://benchmarks/briefs`: the reproducible evaluation briefs.

### `check_deal_conformance`

Read-only. Loads a built page in headless Chromium at 1440px and 390px (scrolled, never clicked) and checks it follows its dealt page shape: each restraint rule, the dealt button shape and the motif. Returns `score` (0-100 over measured checks), `passed` / `failed` / `unknown` and `checks` with evidence. Render facts are written to a scratch directory outside the project (`render_facts`).

| Input | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `target` | string | yes | http(s) URL, or an HTML file / directory (uses its index.html) inside the project. |
| `deal` | string or object | yes | Path to saved `get_design_system` / `design-system --json` output inside the project, or an inline direction or `page_shape` object. |
| `direction` | integer | no | Direction to check, 1-based (default 1). |
| `allow_private_network` | boolean | no | Allow a private-network host. localhost is always allowed. |

## Evidence-backed design workflow

The default MCP tool list exposes all 24 supported tools. `get_design_system` accepts `brief_context` to preserve the actual client subject, facts and pinned choices and returns three compositions even in its default brief. `design_assets` inventories/registers real images with provenance; `import_design_tokens` imports local DTCG/Figma Variables JSON with aliases and modes; `review_design` persists actual screenshot evidence and flags stale source/renders; `compare_before_after` can decode and compare supplied PNG screenshots. Performance diagnostics remain opt-in.
