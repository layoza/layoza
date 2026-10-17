# Multi-Agent Compatibility Guide

Layoza supports five major AI coding environments out of the box via `layoza init` and `layoza doctor`.

| Agent | Skill Discovery Path | Plugin / Manifest Path | MCP Configuration Path | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Google Antigravity** | `.agents/skills.json` + `.agents/skills/*/SKILL.md` | `.agents/plugins/layoza/plugin.json` + `.agents/plugins.json` | `.agents/plugins/layoza/mcp_config.json` | Verified |
| **Claude Code** | `.claude/skills/*/SKILL.md` | `.mcp.json` | `.mcp.json` (`mcpServers.layoza`) | Verified |
| **Cursor** | `.agents/skills/*/SKILL.md` | `.cursor/mcp.json` | `.cursor/mcp.json` (`mcpServers.layoza`) | Verified |
| **OpenAI Codex** | `.agents/skills/*/SKILL.md` | `.codex/config.toml` (`[mcp_servers.layoza]`) | `.codex/config.toml` | Verified |
| **OpenCode** | `.agents/skills/*/SKILL.md` | `opencode.json` | `opencode.json` (`mcp.layoza`) | Verified |

## Progressive-Disclosure Skill Routing
`layoza init` copies the same `skills/` bundle into `.claude/skills/` (Claude Code) and `.agents/skills/` (the other four), so every agent gets identical instructions:
1. `skills/layoza/SKILL.md` acts as the lightweight universal entry router (~90 lines).
2. Specialized workflows (`layoza-site-dna`, `layoza-extend-website`, `layoza-animations`, `layoza-threejs-webgl`, `layoza-wordpress`, etc.) are loaded on demand only when relevant to the user's request.
