# Changelog

All notable changes to Layoza are listed here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/layoza/layoza/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/layoza/layoza/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/layoza/layoza/releases/tag/v0.1.2
[0.1.1]: https://github.com/layoza/layoza/releases/tag/v0.1.1
[0.1.0]: https://github.com/layoza/layoza/releases/tag/v0.1.0
