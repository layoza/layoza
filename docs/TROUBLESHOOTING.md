# Troubleshooting Guide

## 1. `layoza doctor` Reports Missing Agent Configurations
**Symptom**: `layoza doctor` shows `skills_installed: false` or `mcp_configured: false`.
**Fix**:
Run `layoza init` from the repository root:
```bash
cargo run --bin layoza -- init
```

## 2. Live Browser QA (`scripts/browser-qa.mjs`) Cannot Launch Chromium
**Symptom**: `Could not launch headless Chromium, Chrome, or Edge`.
**Fix**:
`scripts/browser-qa.mjs` supports:
1. Local Node Playwright (`npm i -D playwright && npx playwright install chromium`)
2. Python Playwright (`pip install playwright && playwright install chromium`)
3. System-installed Google Chrome or Microsoft Edge via Playwright's `executable_path` fallback.

## 3. `DESIGN.md` Linter Fails on Color Contrast
**Symptom**: `layoza design-system lint --file DESIGN.md` reports a WCAG 2.2 AA contrast failure (`< 4.5:1`).
**Fix**:
Check the primary text color (`#141619` or similar dark ink) and canvas background (`#F8F7F4` or similar light surface) in `DESIGN.md` or run `layoza verify --snapshot .layoza/qa.json --auto-repair` to receive an exact WCAG-compliant hex replacement.

## 4. Existing Website Looks Changed After Adding a Section
**Symptom**: New section introduced unfamiliar fonts or colors.
**Fix**:
1. Roll back immediately using your baseline snapshot:
   ```bash
   cargo run --bin layoza -- baseline rollback --path . --id <baseline-id>
   ```
2. Re-run `layoza inspect --path . --write-artifacts` and ensure Mode B (`layoza-extend-website`) is active before generating the component.
