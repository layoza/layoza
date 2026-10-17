# Layoza CLI Reference (`layoza`)

All commands support `--json` for deterministic machine-readable output.

## Global Syntax
```bash
layoza <subcommand> [options] [--json]
```

---

## Subcommands

### 1. `layoza init`
Installs Layoza skills, Antigravity plugin manifests, and MCP configurations.
```bash
layoza init [--path <dir>] [--global] [--agent <all|antigravity|claude-code|cursor|codex|opencode>] [--json]
```

### 2. `layoza doctor`
Diagnoses OS/architecture, Rust CLI readiness, Node/Playwright availability, skill bundle validity (`16/16`), and multi-agent configuration status.
```bash
layoza doctor [--path <dir>] [--json]
```

### 3. `layoza inspect`
Inspects a local project directory (and optional browser DOM metrics) to extract `SiteDna` and optionally write all 6 design artifacts (`DESIGN.md`, `BRAND.md`, `MOTION.md`, `SITE-DNA.json`, `COMPONENT-INVENTORY.json`, `DESIGN-DECISIONS.md`).
```bash
layoza inspect --path <dir> [--write-artifacts] [--json]
```

### 4. `layoza design-system`
Recommends a domain-tuned design system, generates 3 distinct creative directions, or lints/diffs `DESIGN.md`.
```bash
# Recommend industry profile + 3 distinct creative directions
layoza design-system recommend --industry fintech [--mode new-website] [--variance 6] [--motion 5] [--density 5] [--json]

# Lint a DESIGN.md file for required sections & WCAG 2.2 AA contrast
layoza design-system lint --file DESIGN.md [--json]

# Diff two DESIGN.md files
layoza design-system diff --old DESIGN.v1.md --new DESIGN.md [--json]
```

### 5. `layoza animate`
Generates production-ready, reduced-motion-safe animation or 3D WebGL code recipes with explicit cleanup.
```bash
layoza animate --recipe <staggered-hero|scroll-pinned-story|waapi-spring-reveal|magnetic-cta|view-transition-route|threejs-hero-scene|lenis-gsap-sync> [--framework react|astro|html|vue] [--intensity 6] [--json]
```

### 6. `layoza generate`
Generates framework-native UI components honoring the project's `SiteDna` tokens.
```bash
layoza generate --component hero --framework <react|nextjs|astro|wordpress-gutenberg|html|vue|nuxt|sveltekit|elementor|bricks|shopify-liquid> [--title "..."] [--subtitle "..."] [--cta "..."] [--path <dir>] [--json]
```

### 7. `layoza audit` & `layoza verify`
Audits a file or folder with static rules and, when present, the browser report written by `layoza qa` (`qa/browser-qa-report.json`). Objective checks decide the release gate; subjective design critique is reported but never blocks it. `verify --repair` first applies bounded HTML fixes to a single file.
```bash
layoza qa index.html
layoza audit [path] [--json]
layoza verify [path] [--repair] [--json]
```
The release gate has three outcomes. **passed** (exit 0): no objective errors, and Core Web Vitals were measured and met. **failed** (exit 1): objective errors, or a Core Web Vital missed its target. **unverified** (exit 2): no objective errors, but Core Web Vitals were not measured, so run `layoza qa` first.

### 8. `layoza baseline`
Creates or rolls back non-destructive design baseline snapshots in `.layoza/baselines/`.
```bash
layoza baseline create --path <dir> --label "pre-redesign" [--json]
layoza baseline list --path <dir> [--json]
layoza baseline rollback --path <dir> --id <baseline-id> [--json]
```

### 9. `layoza benchmark`
Runs the 32-brief internal self-check suite: a regression test of Layoza's own rules, not a design-quality score. See [BENCHMARKS.md](BENCHMARKS.md).
```bash
layoza benchmark [--json]
```

### 10. `layoza mcp`
Starts the stdio JSON-RPC 2.0 Model Context Protocol (MCP) server.
```bash
layoza mcp [--read-only]
```
