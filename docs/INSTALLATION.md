# Layoza Installation Guide

## Prerequisites
- **Rust Toolchain**: `rustc >= 1.80` (`cargo` included)
- **Node.js** (`>= 18`, optional but recommended for live browser QA via `scripts/browser-qa.mjs`)
- **Playwright / Chromium** (either Node `playwright` or Python `playwright` with Chromium/Chrome/Edge installed)

> **Zero Python Requirement for Core Operations**: Unlike `ui-ux-pro-max-skill`, Layoza's CLI (`layoza`) and MCP server (`layoza mcp`) are compiled native Rust binaries with zero Python runtime requirement.

---

## 1. Build from Source
```bash
git clone https://github.com/layoza/layoza.git
cd layoza
cargo build --release
```
The compiled binary is located at:
- Windows: `target\release\layoza.exe`
- macOS / Linux: `target/release/layoza`

---

## 2. Project-Scope Installation (Recommended)
Run `layoza init` inside any repository to install project-local agent configurations:
```bash
cargo run --bin layoza -- init
```
To install for a single agent only:
```bash
cargo run --bin layoza -- init --agent antigravity
```

---

## 3. Global User-Scope Installation
To install Layoza's skills and MCP configuration into your user profile (`~/.gemini/config` or `~/.agents`):
```bash
cargo run --bin layoza -- init --global
```

---

## 4. Verify Installation with `layoza doctor`
```bash
cargo run --bin layoza -- doctor
```
`layoza doctor` checks:
1. Rust CLI readiness (`rust_cli_ready: true`)
2. All 16 progressive-disclosure skills in `skills/*/SKILL.md` (`skills_bundle_valid: true`)
3. Agent configuration files for Antigravity, Claude Code, Cursor, Codex, and OpenCode
4. Node.js and Playwright availability for live browser verification
