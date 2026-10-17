# Layoza MCP Server Reference (`layoza mcp`)

Layoza includes a zero-dependency Rust stdio JSON-RPC 2.0 MCP server in [`crates/layoza-mcp/src/lib.rs`](../crates/layoza-mcp/src/lib.rs).

## Launching the Server
```bash
layoza mcp
# Or in strict read-only mode (disables file-writing tools):
layoza mcp --read-only
```

---

## Exposed MCP Tools (14 Tools)

| Tool Name | Mutates Disk? | Description |
| :--- | :--- | :--- |
| `layoza_inspect_project` | No | Scans a project directory for framework, tokens, fonts, routes, components, and baseline hash. |
| `layoza_write_design_artifacts` | **Yes** | Writes `DESIGN.md`, `BRAND.md`, `MOTION.md`, `SITE-DNA.json`, `COMPONENT-INVENTORY.json`, and `DESIGN-DECISIONS.md`. |
| `layoza_lint_design_md` | No | Validates a `DESIGN.md` document for required sections and WCAG 2.2 AA contrast ratios. |
| `layoza_diff_design_md` | No | Diffs two `DESIGN.md` documents and reports added/removed/changed tokens and sections. |
| `layoza_recommend_design` | No | Returns industry profile, anti-patterns, style match, and 3 distinct creative directions. |
| `layoza_search_styles` | No | Searches Layoza's style catalog by keyword, industry, or aesthetic family with provenance. |
| `layoza_generate_animation` | No | Generates GSAP, ScrollTrigger, WAAPI, View Transitions, Three.js, or conditional Lenis code with cleanup & reduced-motion support. |
| `layoza_generate_component` | No | Generates framework-native code (`react`, `nextjs`, `astro`, `wordpress-gutenberg`, `html`, `vue`, `sveltekit`, `elementor`, `bricks`, `shopify-liquid`). |
| `layoza_verify_snapshot` | No | Evaluates a browser QA snapshot (objective checks, subjective rubric, WCAG 2.2 AA, Lab CWV, and bounded auto-repair). |
| `layoza_check_contrast` | No | Computes exact WCAG 2.2 relative luminance contrast ratio between two hex colors (`4.5:1` AA, `7.0:1` AAA). |
| `layoza_create_baseline` | **Yes** | Creates a non-destructive baseline backup in `.layoza/baselines/<id>`. |
| `layoza_rollback_baseline` | **Yes** | Restores files from a stored `.layoza/baselines/<id>` snapshot. |
| `layoza_doctor` | No | Runs full multi-agent installation and environment diagnostics. |
| `layoza_run_benchmarks` | No | Runs the 32-brief Layoza benchmark suite and returns structured metrics. |

---

## Exposed MCP Resources (4 Resources)
- `layoza://catalog/styles` — Full searchable design style taxonomy with upstream provenance
- `layoza://catalog/industries` — 18 industry design intelligence profiles
- `layoza://catalog/benchmarks` — 32 reproducible benchmark briefs
- `layoza://docs/operating-modes` — Operating Modes A, B, C, and D specification
