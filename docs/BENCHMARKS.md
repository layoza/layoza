# Internal Self-Check Suite

`layoza benchmark` is a **regression test for Layoza's own rules**, used by maintainers before a release. It is not a measure of design quality and it does not compare Layoza with other tools.

For a real, judged comparison against other design skills, see [CROSS-TYPE-BENCHMARK-2026-10.md](CROSS-TYPE-BENCHMARK-2026-10.md).

## Run it

```bash
cargo run --bin layoza -- benchmark --json > benchmark-results.json
```

## What it checks

The suite has 32 briefs, defined in [`crates/layoza-core/src/benchmark.rs`](../crates/layoza-core/src/benchmark.rs). They cover all four operating modes and the framework adapters. For each brief it runs these deterministic checks:

- the generated `DESIGN.md` passes `layoza lint`
- a static audit of the template files the adapter emits finds no errors
- every recommended animation recipe has a reduced-motion fallback
- the entry skill fits within the context budget
- BM-32 only: a prompt-injection payload is redacted during the Site DNA scan

## What it does not check

- It does not render pages in a browser. The contrast ratio is read from the matched industry palette, not measured.
- It does not compile framework output (React, Astro, Liquid and so on).
- It does not edit existing sites, so it cannot measure how well an existing site's design is preserved. `preserves_existing_dna` is always `null`.
- It does not score design quality, accessibility or performance.

## Last run

On 2026-10-17 (the v0.1.2 code), 32 of 32 briefs passed these checks. A pass means Layoza's rules are internally consistent. It says nothing about how good the generated pages look.
