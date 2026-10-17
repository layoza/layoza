# Making Antigravity Better for UI/UX Design, Motion & Frontend Engineering

## Executive Summary

Based on our inspection of Google Antigravity's customization engine (`agy-customizations`, `antigravity-guide`) and your machine's current global configuration (`~/.gemini/config/`), Antigravity has a powerful multi-layered extension architecture—**Skills**, **Plugins**, **MCP Servers**, **Hierarchical Rules**, and **Lifecycle Hooks**—but currently lacks a cohesive **Design Intelligence, Site DNA Extraction, Premium Motion, and Real-Browser QA** system.

 Currently, your global `~/.gemini/config/` includes:
- **MCP Servers**: `wppilot-localhost` and `rea` (Reverse Engineer Anything).
- **Skills**: `hyperframes` (video composition), `reverse-engineer-anything`, and WordPress/PHP security skills (`wordpress-core-2026-expert`, `wp-frontend-expert`, `php-modern-security-expert`, `securty-plugin-suite`).
- **Plugins**: `chrome-devtools-plugin` and `modern-web-guidance-plugin`.

To make Antigravity dramatically better at frontend engineering, UI/UX design, existing-site preservation, and animation, **both Progressive Skills and a fast local Rust CLI + MCP Server are needed**—and they must share a single source of truth (**Layoza**).

---

## 1. How Antigravity's Customization System Works (and How to Optimize It)

| Mechanism | Antigravity Discovery Path | Context & Runtime Characteristics | How Layoza Optimizes It |
| :--- | :--- | :--- | :--- |
| **Skills** (`SKILL.md`) | `.agents/skills/<name>/SKILL.md` (workspace), `.agents/skills.json`, `~/.gemini/config/skills/<name>/SKILL.md` (global) | **Progressive Disclosure**: Only `name` and `description` YAML frontmatter are loaded into context initially. Full instructions and `references/` are read on-demand. | Layoza provides 1 primary orchestrator skill (`layoza`) + 15 specialized workflow skills with concise `SKILL.md` files (< 150 lines each) and on-demand reference files so context budgets are never bloated. |
| **Plugins** (`plugin.json`) | `.agents/plugins/<name>/plugin.json`, `.agents/plugins.json`, `~/.gemini/config/plugins/<name>/plugin.json` | Bundles `plugin.json`, `mcp_config.json`, `hooks.json`, `rules/AGENTS.md`, and `skills/` into one toggleable unit. | Layoza packages a native Antigravity plugin (`.agents/plugins/layoza/`) that automatically mounts all Layoza skills, rules, and the `layoza mcp` server in one unit. |
| **MCP Servers** (`mcp_config.json`) | `~/.gemini/config/mcp_config.json` or inside a plugin's `mcp_config.json` | Supports **Stdio** (`command`, `args`, `env`) and **Streamable HTTP** (`url`). Antigravity supports lazy tool schema loading (`~/.gemini/antigravity/mcp/<server>/`). | `layoza mcp` runs as a zero-overhead native Rust stdio MCP server exposing 14 structured tools (`inspect_project`, `extract_site_dna`, `get_design_system`, `recommend_layout`, `recommend_animation`, `discover_components`, `capture_screenshots`, `compare_visual_diff`, `audit_accessibility`, `audit_performance`, `verify_functional`, `export_report`, `create_baseline`, `rollback_baseline`). |
| **Rules** (`AGENTS.md` / `GEMINI.md`) | Root & hierarchical directory walk | **Capped at 24 KB per file** and **20,000 tokens aggregate budget**. Always active in scope. | Layoza keeps `AGENTS.md` concise (< 8 KB) focused strictly on non-negotiable guardrails: inspect existing site DNA before coding, honor `prefers-reduced-motion`, preserve framework identity, and verify in a real browser. |
| **Hooks** (`hooks.json`) | `.agents/hooks.json` or inside a plugin | Synchronous JSON stdin/stdout hooks on `PreToolUse`, `PostToolUse`, `PreInvocation`, `PostInvocation`, `Stop`. | Optional Layoza safety hook prevents unapproved destructive overwrites of existing design tokens and reminds the agent to run `layoza verify` before completing frontend tasks. |

---

## 2. Why Both CLI + Skills AND an MCP Server Are Needed

As noted in Microsoft's `playwright-mcp` engineering findings and verified across `rea` and `impeccable`:
1. **Why Skills + Rust CLI (`layoza`) Matter**:
   - Progressive skills keep token usage near zero until a design/frontend task is triggered.
   - A compiled Rust CLI (`layoza`) starts in `< 15ms` with **no Python runtime required**, deterministic JSON output (`--json`), and instant BM25 + rule-based search across 195+ industry rules, 85+ UI styles, 75+ typography pairings, and 60+ deterministic code/HTML/CSS anti-slop & accessibility detectors.
2. **Why an MCP Server (`layoza mcp`) Matters**:
   - In Antigravity, MCP tools provide schema-validated, structured tool calls (`call_mcp_tool` / native tool invocation) and `list_resources` / `read_resource` access (`layoza://dna/current`, `layoza://design-system`, `layoza://motion-recipes`, `layoza://audit/latest`) without requiring shell escaping or manual file parsing.
   - Both the CLI and MCP server call the exact same Rust core library (`layoza-core`), guaranteeing zero drift between CLI commands and MCP tool results.
