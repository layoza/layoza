# Changelog

All notable changes to Layoza are listed here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/).

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

[0.1.0]: https://github.com/layoza/layoza/releases/tag/v0.1.0
