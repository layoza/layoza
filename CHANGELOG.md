# Changelog

All notable changes to Layoza are listed here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed
- Taste rules: score the plan before code, two or three working type sizes, hand-broken headlines (2 lines desktop, 3 phone), one spacing scale, a picture floor with one crop system, section gaps at least twice inner gaps; docs, articles and portfolios let the content be the signature.
- The review step checks every section on its own crop (one idea with proof, type from tokens, spacing on the scale, a composition its neighbours do not share), runs a headline-swap test and cuts one element.
- `get_design_system` now returns a short Markdown design brief by default (`detail: "brief"`):
  one whole-page look, twelve taste rules, an avoid list and a four-step build. The SMDLED brief
  went from about 91 KB of JSON to 7 KB. `detail: "summary"` / `"full"` still return the JSON deal.
- The art-direct prompt is five steps: brief, five-line plan, one-pass build, one screenshot
  review (three worst problems, at most two rounds), QA once. Dealt parts are ideas, not mandates.
- The brief now opens with the signature moment: one subject-driven set piece the visitor drives by
  scroll or hand, with a done-when check (start, middle and end frames, phone, no-JS final frame).
  New taste rules for launch-style pacing and choreographed motion; new bans on zig-zag stacks and
  pages assembled from stock parts. The art-direct plan names `SIGNATURE:` and builds it first.
- The design-process gate matches the simple path: `direction.md` needs `PURPOSE:`, `PICKED:`,
  `SIGNATURE:` and one `ROUND` from the screenshot review. `DESIGN-ART-LOOP-INCOMPLETE` (three
  rounds plus a reviewer verdict) is replaced by `DESIGN-NOT-REVIEWED` and `DESIGN-NO-SIGNATURE`.
  The entry skill (`skills/layoza/SKILL.md`) now teaches the same five steps and asks for 0 QA errors.
- `motion-too-sparse` needs only two motion ideas when the page has a live signature set piece
  (a pinned stage that changes with scroll, or a scroll-timeline animation).

### Added
- QA: `picture-floor`, `headline-too-many-lines`, `blank-stretch`, `heading-level-skip`, `thin-text-on-dark` (warnings) and `colour-sprawl`, `spacing-sprawl`, `selection-unstyled` (critiques).
- `zigzag-stack` (`UI-ZIGZAG-STACK`): three or more media-beside-text rows that flip sides warn; four
  or more fail the gate. It is the commonest template rhythm.
- `signature-missing` (`UI-NO-SIGNATURE`): a page that is not declared calm warns when no pinned
  stage changes with scroll and no animation is linked to scroll.

### Fixed
- Live QA on `file://` pages read no stylesheet rules (Chromium treats each file as its own origin), so every stylesheet-driven check ran blind and reported false "no ::selection" and "no error states" findings. Chromium now launches with file access for local pages.
- `visual-starved-page` treated each item of a list titled with `h2`s as its own section, and judged a sticky photo or an image strip against the full height of a tall section. Sections are now landmark sections, and media is summed and measured against one viewport.
- `ledger-heavy-page` fired at two data sections although its own advice allows one or two; it now fires at three.
- The static reduced-motion check matched the `gsap.min.js` script URL as an animation; it now looks for real GSAP calls.
- Fonts the brief labels by role ("Headings: …", "Body: …") keep that role; a body face is no longer dealt as display or the reverse.
- The brief's own secondary-text colour is kept (darkened only as far as contrast needs) and a brief tint becomes the surface, instead of unrelated base colours.
- The short brief no longer cuts ideas off mid-sentence.
- `QA-SECTION-RESKIN` no longer flags a page as a copy of itself when one QA run used the file
  and another a loopback URL on a different port.
- `testimonial-unsourced` reads the attribution in a quote's `figcaption` instead of counting the
  inner `blockquote` as a separate, unattributed testimonial.
- The brief carries the look's tokens as a paste-ready `:root` CSS block.
- `tools/list` shows the 13 tools the prompts use. Eight lookup tools stay callable and are listed
  with `LAYOZA_TOOLS=all`.

### Added
- Stated purpose (stop look-alike AI output) in the MCP server instructions, prompts, README,
  entry skill and the generated project skill.
- Uniqueness ledger `.layoza/looks.jsonl` covers every slot (palette, type, hero, nav, sections,
  footer, archetype); `design-system` and `mechanics deal` steer away from the last 20 entries
  (`--window N`) and print a `uniqueness` block and a `ledger:` line. Typography is compared by
  font family, so a renamed pairing with the same faces counts as a repeat and is steered away.
- Design knowledge: 208 palettes with computed contrast, 200 free type pairings, 75 layout
  archetypes, 318 mechanics with nav, footer, section-head and proof roles, 163 UX guidelines,
  40 chart situations, 16 stack notes and icon guidance (`guidelines`, `chart` commands and
  5 MCP tools).
- Browser QA rules for section re-skins (`QA-SECTION-RESKIN`, `QA-SAME-SECTION-TWICE`) and the
  `DESIGN-DIRECTION-NOT-APPROVED` gate; `qa --full` / `--allow`, `audit` / `verify --strict`;
  opt-in edit hook with a 300 ms budget; 16px / 12px type-floor lint.
- Page shape: every new-site direction is dealt a binding `page_shape` (hero frame, section flow,
  section head, ground, material, label style and a motion tier with named techniques, tools and
  static fallbacks). The three directions of a deal differ on every axis and the deal steers away
  from the ledger; prompt words (dark, minimal, animated, bento, glass, 3D, a named stack) pin
  only their own axis.
- `understood_brief` on `design-system`: trade, audience, goal, tone, motion wish, stack and
  must-haves read from the prompt, with niche-based assumptions instead of questions (at most
  one or two blocking open questions).
- Agencies, SaaS and other showcase niches default to heavy motion (GSAP, Three.js / R3F,
  Motion, Lenis, view transitions, CSS scroll timelines, chosen per direction); solemn and task
  UIs stay calmer.
- User-level look ledger (`~/.layoza/user/.layoza/looks.jsonl`, under `$LAYOZA_HOME` when set):
  every project mirrors its looks there, and a new site started outside a project uses it, so two
  projects are not dealt the same palette, type or page shape. Previously the ledger was off
  outside a project and separate sites could repeat a look.
- The direction gate accepts `PICKED: auto — <direction>, <why it fits the niche>`.

### Changed
- `layoza qa` is fast by default (about 14 s instead of about 70 s); `--full` runs the throttled
  pass and every width. The Node runner now calls page functions it was handed as strings.
- Long briefs keep the trade's head noun (e.g. "funeral home", "wedding venue").
- MCP private-host guard resolves DNS before allowing a URL.
- Skills: designer persona, decide-from-niche intake, binding page shape, and a "best in the
  world, never a template" bar where every section earns its own layout.

## [0.1.4] - 2026-10-19

### Added
- MCP `capture_screenshots` takes an optional `sketch: true` that returns only the
  screenshot paths per viewport plus error and warning counts, skipping findings, metrics,
  Site DNA and the release-gate audit. The full report is still written to disk. Default
  output is unchanged.
- Design history: a user-level ledger at `~/.layoza/history.jsonl` (`$LAYOZA_HOME` overrides
  the folder, `LAYOZA_NO_HISTORY=1` turns it off). When an audit passes the release gate on a
  project whose `.layoza/direction.md` has a pick, it appends one line holding only a project hash, the date,
  the direction name, palette hexes, type families, motif keywords and mechanic ids (no paths,
  page copy or secrets). Repeat audits of an unchanged design add nothing.
- MCP `capture_screenshots` returns a compact summary by default (top findings, counts, paths;
  about 7 KB instead of about 75 KB on a real site); `detail: "full"` returns everything, and the
  full report is always written to `capture-full.json`. A mobile hero crop is saved and returned.
- Browser QA waits for motion to settle before screenshots and checks, and adds three checks for
  text over graphics: `UI-TEXT-OVER-ART-CONTRAST`, `UI-TEXT-OVER-BUSY-ART`, `UI-TEXT-OCCLUDED`.
  Headless Chromium often draws nothing for WebGL, so text over 3D still needs a check by eye.
- Site DNA reads real fonts (@fontsource, Google/Bunny links, `next/font`, Tailwind `fontFamily`,
  CSS variables, `font-family` / `font:` rules, @font-face, theme.json) and gives each typography
  token a `source`; it falls back to an inferred baseline only when nothing is declared, and says
  so. Fonts in sketch, draft, fixture, test and stories folders count only as a last resort.
- `ai-devtools` industry covers MCP servers, CLIs, coding agents and open-source tools, with two
  developer-tool directions (session replay, install-and-try playground).
- `deal_mechanics` filters by fit to the brief's trade (mechanics built for another trade are left
  out), returns a `fit` reason per dealt mechanic, and takes an `avoid` list of mechanic ids. New
  `devtools` niche; catalogue at 76 mechanics. CLI: `layoza mechanics deal --avoid a,b` prints
  fit reasons; `layoza design-system` includes `avoid_recent` in new and redesign modes.
- MCP `get_design_system` returns an `avoid_recent` block (last 8 projects: directions,
  palettes, fonts, motifs, mechanics) in `new_website` and `full_redesign` modes.
- `DESIGN-REPEATS-RECENT` (warning): the pick shares at least two of palette, type, motifs and
  mechanics with another recent project (one of them palette or type). A `REPEAT-OK: <reason>`
  line in `direction.md` accepts a look the user asked to keep. `DESIGN-HISTORY-UNWRITABLE`
  (info) when the ledger cannot be written.
- Subject check on `direction.md`: `DESIGN-NO-SUBJECT` (warning) when the purpose and direction
  name none of the trade's objects; `DESIGN-GENERIC-DIRECTION` (warning when only the direction
  lines are style words, error when the whole file is style words with no subject at all).
- `get_design_system` reads hex colours written in the brief (`brief_signals.brief_colors`) and
  puts a BRIEF COLOURS line first in the workflow guidance: the brief's palette wins and the
  direction palettes are fallbacks. The CLI prints them as `Brief Colours`.
- Brief fonts: font families named in the brief (Space Grotesk, Geist Mono, IBM Plex Mono...)
  are read into `brief_signals.brief_fonts` and a `BRIEF FONTS` guidance line; the direction
  pairings are fallbacks. The CLI prints them as `Brief Fonts`.
- Three wild devtools mechanics: `hero-exploded-pipeline-stack`, `process-request-trace-scrub`,
  `data-story-benchmark-run-replay`. A heavy-motion dev-tool brief no longer borrows globes,
  hallways, turntables and split-flap boards from other trades.
- Art-direction critique gains reading order (against a new `HIERARCHY` line in
  `direction.md`), an interaction-state inventory and a cliché swap (passes 16-18).
- `get_design_system` `variant` param (CLI `--variant N`): 0 stays the stable answer; another
  number samples near-best fonts, palettes and mechanics for the same brief.
- Workflow guidance gains a HERO line: Layoza picks no first-screen composition; the agent
  chooses it from the subject, gives the three directions different first screens, needs a
  reason for a headline-left / visual-right split and records `HERO:` in `direction.md`. The
  art-direction skill explains how (examples, not a menu).

### Fixed
- Museums, science centres, aquariums and zoos get their own `museum` niche and mechanics
  (floor-plan wayfinding, specimen drawer, question of the day, age paths, today's
  programme, wristband tickets, wall-label gallery) instead of borrowing law-firm or
  restaurant pieces.
- Tool briefs ("bakery dashboard") now lead with the tool: dashboard, devtools and docs
  outrank the trade in short briefs, the trade's storefront mechanics (menus, shelves) are
  left out, and three dashboard heroes (ops wall board, day scrub, command bar) were added.
  The command-bar hero is now CSS-only so calm directions can use it.
- A mechanic built around another narrow trade's object is foreign even when a secondary tag
  matches the brief, unless it is also tagged for one of the brief's own narrow trades.
  Across 6 test briefs x 4 variants, picks borrowed from another trade fell from 24 of 212
  (0.1.3) to 0 of 216.
- Palettes and type pairings led by another trade's subject tag score lower (-7, was -4),
  except when the brief wants the pairing's surface tag (data, code).
- Law-firm briefs read as their own `legal` niche and get six legal mechanics (case-caption hero,
  exhibit-binder hero, practice-area clause index, matter docket timeline, engagement-letter terms,
  bar-roll register) instead of the tape-measure timeline, measured before/after and odometer hero,
  which no longer claim the `professional` niche.
- Law and finance briefs are never dealt a navy-and-gold palette, their listed cliché;
  `chancery-green` and `ink-blue-field` dropped their brass and gold accents.
- Each direction now states the same first-screen composition as its dealt hero. A fitting hero in
  the direction's composition wins; when none fits, the direction follows the hero and says why.
- `design-system --variant N`: later variants no longer drift into another trade's look (a
  marine palette or a rounded kids' face for a law firm). Fonts and palettes tagged only for other
  subjects drop out of the near-best window.
- `design-system --mode new`: the three directions are now dealt from shared pools (38 Google
  Fonts pairings, 39 palettes, each checked for 4.5:1 body contrast; 22 layout archetypes) instead
  of being templated off the industry profile. Each direction prints its own type and palette (the
  summary used to show the industry pairing for all three). `--variant N` re-deals all three slots
  (variant 2 used to repeat variant 0). Direction copy and the new "Why" line are composed from the
  brief's own subject words and the options picked. Briefs that match no industry are read from
  their own words instead of falling back to local-business, and unlisted industries get cliches
  derived from the brief. Hero mechanics now declare a composition; 2 of 20 are splits, splits get
  no default preference, and a direction says so when its hero differs from the suggested layout.
- A brief asking for heavy motion ("highly animated", "cinematic", "scrollytelling", ...) now
  raises every direction's motion dial to at least 8 and adds a scroll-choreography note; before,
  the dial stayed at the industry default and a direction could say nothing moves.
- Asking for accessibility or performance in the same brief no longer silently caps that heavy
  motion at 5; they stay as guardrails. Elderly/senior audiences, low-bandwidth and offline still cap.
- Negated parts of a brief no longer steer the match: lines, sentences and bullets with
  avoid / never / no / not / don't / without (and bullets under an "Avoid:" header) are dropped
  before industry, surface, niche and mechanic matching. "Avoid dashboard mockups" used to turn
  a dev-tool brief into a dashboard workbench.
- Long briefs (over 120 words) rank niches by how often they come up and drop passing mentions
  (showcase examples, "not a dashboard"); technique niches (`3d`, `kinetic`, `landing`) never
  lead a trade. "Three.js" in a dev-tool spec used to make `3d` the lead niche.
- Surface is chosen by the most keyword hits, not by the first list that matches.
- "Respect prefers-reduced-motion" next to a heavy-motion ask is a fallback constraint, not a
  motion cap of 1. An explicit heavy-motion phrase also beats a stray calm word ("a minimal
  footer") in `deal_mechanics`.
- Direction A drops its fixed 7:5 hero ratio, fixed section clamp and fixed 160-240ms timing;
  it also re-deals fonts and palette on a `variant`. The devtools archetype no longer says
  "Split ... Hero", and two Direction C layouts no longer force side-by-side heroes. Direction B's
  showcase motion copy no longer names GSAP. A quiet direction's "nothing moves on scroll" is
  removed when the brief asks for heavy motion.
- A simile no longer sets the niche: "design like a studio" stops dealing agency mechanics.
  Heavy-motion words now deal at wild intensity.

### Changed
- Skills: an explicit user instruction (palette, stack, layout, motion library, scope) now
  overrides every Layoza default in the router, art-direction, new-website, extend-website,
  redesign and refine skills. New-look directions must differ from `avoid_recent` unless the
  user asks for the same look; extend and improve modes follow the existing design.
- Browser QA overall timeout default raised from 180s to 600s (`LAYOZA_QA_TIMEOUT_SECS`); real
  multi-viewport runs took 2–4 minutes.
- Visual QA guidance: judge screenshots only after motion settles, and check text over 3D,
  canvas, video or images by eye instead of trusting the automated contrast checks there.
- `docs/MCP-REFERENCE.md` is regenerated from the 16 actual tools and 4 resources; the stale
  `layoza_*` names and the nonexistent `--read-only` flag are gone.
- Skills: the direction pick for new sites (Mode A/D) now shows three rendered hero
  sketches (one static HTML file per direction, screenshotted at 1440 and 375) with a
  strength, weakness and recommendation, never ASCII or text wireframes.
- Skills: existing sites (Mode B/C) never get new directions, sketches or a pick; the
  extend, refine and Site DNA skills inspect first and follow the incumbent design. A
  direction file kept in Mode B/C records `PICKED: existing — <design followed>`, which
  satisfies `DESIGN-NO-DIRECTION-PICK` (now named in its remediation, with a test).

## [0.1.3] - 2026-10-18

### Added
- The audit (and so `layoza qa`, `verify` and every MCP tool that reports the gate) now
  checks the recorded design process whenever `.layoza/direction.md` exists:
  `DESIGN-NO-PURPOSE` (no `PURPOSE:` line: what the site is for, audience, new or existing),
  `DESIGN-NO-DIRECTION-PICK` (no `PICKED:` line naming the direction the user chose, or
  `PICKED: unattended — <reason>`) and `DESIGN-ART-LOOP-INCOMPLETE` (fewer than 3
  `ROUND <n>:` lines or no `REVIEWER:` verdict). All three are errors. Projects without a
  direction file (Mode B/C) are unaffected.
- `gate_scope` in the audit report states that the gate covers defects only.
- Skills: an intake step (purpose, audience, new or existing, must-keep; ask for what the
  prompt leaves open), a fit-check on the dealt world, a hard stop where the user picks one
  of three directions, round and reviewer logging in `direction.md`, a subject rule for
  software, developer-tool and MCP products (show the product working), and "never cut
  motion to pass QA".

### Changed
- A passing gate now prints `PASSED (defects only; this gate cannot judge whether the design
  is good)`.
- The benchmark pages under `examples/` were built before these checks; re-auditing them now
  reports the process errors, since their direction files carry no purpose, pick or round log.

## [0.1.2] - 2026-10-18

### Added
- MCP Registry metadata. `package.json` carries `"mcpName": "io.github.layoza/layoza"`
  and the new root `server.json` describes the local stdio server, launched with
  `npx layoza@0.1.2 mcp`. Nothing is published to the Registry by this release on its own;
  see [docs/MCP-REGISTRY-RELEASE.md](docs/MCP-REGISTRY-RELEASE.md).
- `scripts/check-mcp-registry.mjs` checks `server.json` against the 2025-12-11 schema
  rules it uses, that every version matches, the exact `mcpName`, the launch command and
  (with `--tarball`) the packed npm archive.
- `scripts/mcp-smoke.mjs`, a dependency-free MCP stdio client that runs `initialize`,
  `tools/list`, a read-only tool call, bad-input and unknown-tool calls and `ping`, checks
  that stdout carries only JSON-RPC and that the server exits when stdin closes. CI runs
  both scripts, and the release workflow runs them against the packed tarball.
- New brand assets: app icon (`docs/assets/icon-256.png`, also the Registry icon), light and
  dark horizontal logos in the README header, org avatar and repository social preview.

### Fixed
- `layoza init` no longer replaces an existing `.mcp.json`, `.cursor/mcp.json` or
  `opencode.json` with `{}` when the file is not valid JSON. It now leaves the file
  unchanged and tells you to fix it or add the entry by hand.
- `layoza qa` now finds Playwright installed in your project. The QA script runs from a
  temp folder, where a bare import could not see the project's `node_modules`, so QA fell
  back to Python or failed while `layoza doctor` reported Playwright as found.

### Changed
- The release gate no longer passes when Core Web Vitals were not measured. Reports
  carry `verification_status` (`passed`, `failed` or `unverified`), and `audit`, `verify`
  and `qa` exit with 2 when the result is unverified.
- `compare_before_after` states that it compares audit findings and Core Web Vitals only.
  It no longer lists screenshot viewports it never diffed, and `pixel_diff_ratio` is
  `null` unless you supply your own number.
- `layoza benchmark` is described as what it is: an internal regression check of
  Layoza's own rules. Removed the invented quality, accessibility and performance scores
  from `docs/BENCHMARKS.md` and the hard-coded `preserves_existing_dna: true` flag.

## [0.1.1] - 2026-10-17

First npm release.

- Fixed the npm `bin` entries so `npx layoza` and `layoza-mcp` work after install
  (npm stripped the `./`-prefixed paths when publishing 0.1.0, which never reached npm).

## [0.1.0] - 2026-10-17

First public release.

### Added
- **CLI and MCP server** in one Rust binary (`layoza`, `layoza mcp`) with 16 MCP tools:
  design systems, layout and animation advice, site DNA extraction, live Chromium
  screenshots, accessibility and performance audits, before/after comparison,
  baselines and rollback.
- **Prebuilt binaries** for Windows x64, Linux x64/arm64 and macOS x64/arm64, bundled
  in the npm package, so `npx layoza` runs without a Rust toolchain.
- **19 agent skills** (`skills/`) with a router skill and specialists for discovery,
  site DNA, creative directions, motion, three.js, accessibility, visual QA, Core Web
  Vitals and WordPress.
- **Mechanics catalogue**: 71 hero, section and transition mechanics dealt per niche
  (`layoza mechanics list|show|deal`).
- **Visual QA** (`layoza qa`): multi-viewport Chromium checks for overflow, clipped
  text, art over text, menu behaviour, sparse motion and contrast.
- **Installer and doctor** (`layoza init`, `layoza doctor`): copies the skills into
  `.claude/skills` and `.agents/skills` and adds the MCP server for Claude Code, Cursor,
  Codex (`.codex/config.toml`), OpenCode and Antigravity.
- **Blind benchmarks** across four kinds of site with screenshots, in `examples/` and
  `docs/CROSS-TYPE-BENCHMARK-2026-10.md`.

[Unreleased]: https://github.com/layoza/layoza/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/layoza/layoza/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/layoza/layoza/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/layoza/layoza/releases/tag/v0.1.2
[0.1.1]: https://github.com/layoza/layoza/releases/tag/v0.1.1
[0.1.0]: https://github.com/layoza/layoza/releases/tag/v0.1.0
