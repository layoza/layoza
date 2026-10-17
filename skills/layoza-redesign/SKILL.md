---
name: layoza-redesign
description: >-
  Mode C (targeted existing website improvement) and Mode D (full creative redesign)
  skill with mandatory pre-edit baseline snapshots, regression verification, and
  instant rollback.
---

# Layoza — Existing Website Improvement (Mode C) & Full Redesign (Mode D)

## Trigger Conditions
- **Mode C**: The user asks to improve, polish, modernize, or fix usability/accessibility problems on an existing website without discarding its identity.
- **Mode D**: The user explicitly requests a completely new visual design or full creative overhaul.

## Step-by-Step Procedure

1. **Create a Reversible Baseline Snapshot First**:
   ```bash
   cargo run --quiet --bin layoza -- baseline create pre-redesign --file index.html
   ```
2. **Execute Mode C or Mode D**:
   - **In Mode C**: Run `layoza audit <path>`, fix objective defects (contrast, overflow, focus visibility, typography hierarchy, mobile responsiveness) while keeping working brand tokens intact.
   - **In Mode D**: Run `layoza design-system "<brief>" --mode redesign --json` to generate 3 creative directions, preserve all required user content, routes, and interactive workflows, and implement the chosen direction. Commit to one new world with [`layoza-art-direction`](../layoza-art-direction/SKILL.md) (its step 0 lists what the redesign must keep; the old look is evidence, not authority).
   - **In Mode C, run each fix as a [`layoza-refine`](../layoza-refine/SKILL.md) command** (`audit` → `harden` / `adapt` / `clarify` / `optimize` / `typeset` … → `polish`): baseline, `layoza qa` before and after, roll back any command that regresses a measured metric. Its art-direction use is limited to steps 8–9 (craft floor + measured finish); the existing brand DNA in `SITE-DNA.json` wins over new-work bans.
3. **Verify & Rollback if Regressions Occur**:
   ```bash
   cargo run --quiet --bin layoza -- verify <path>
   # If rollback is ever needed:
   cargo run --quiet --bin layoza -- baseline rollback pre-redesign
   ```

## Parity Checklist (Mode D, and Mode C on apps)
Write it before the first edit; every row must be ticked before release:
- [ ] **URLs:** every route, query parameter (filters, sort, tab) and anchor still resolves, or has a 301 / client redirect.
- [ ] **Keyboard shortcuts:** the old map still works (list it); new ones don't collide with browser or screen-reader keys.
- [ ] **Data density:** rows visible per screen at 1440×900 equal or exceed the old screen for power-user views; offer a compact mode if the new default is roomier.
- [ ] **All states:** empty, loading, partial, error, offline, permission denied, sparse and dense exist for each view ([`layoza-product-ui`](../layoza-product-ui/SKILL.md) §6).
- [ ] **Analytics hooks:** event names, `data-*` tracking attributes, element IDs used by tests, tag managers and support docs are unchanged.
- [ ] **Functions:** every form, export, bulk action and integration still works (`layoza verify <path>`).

**Token-rename migration:** never rename tokens in place. Add the new names, alias the old ones to them (`--brand-500: var(--color-accent);`) with a `/* deprecated: remove after <release> */` note, migrate usages, then delete the aliases in a later release. Run `layoza design-system DESIGN.md DESIGN.v2.md --diff` to list every removed or renamed token.

## Tested Example
```bash
cargo run --quiet --bin layoza -- baseline create v1-backup --file DESIGN.md
```
