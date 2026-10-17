# Contributing to Layoza

Thanks for helping. Bug reports, benchmark briefs and fixes are all welcome.

## Set up

```bash
git clone https://github.com/layoza/layoza
cd layoza
cargo test
node bin/layoza.mjs --version
```

You need Rust (stable) and Node 18 or newer. Visual QA also needs Playwright's
Chromium: `npx playwright install chromium`.

## Making a change

1. Open an issue first for anything larger than a small fix, so we can agree on the approach.
2. Keep the change focused. One fix or feature per pull request.
3. Add or update tests in the crate you touched, and run `cargo test` before pushing.
4. If you change a skill in `skills/`, keep its `SKILL.md` front matter valid and its
   instructions tool-neutral: skills must follow the user's prompt and never force a
   specific library.
5. If you change UI output or QA rules, include before/after screenshots in the pull request.

## Benchmarks

Benchmark results must stay fair:

- Every arm gets the same brief, the same model and the same time.
- Judges are blind: arms are relabelled before judging.
- Never edit a rival arm after it is graded.
- Report losses as plainly as wins.

## Third-party ideas

Layoza may adapt ideas from other open-source projects, but never copies them wholesale.
Credit the source and its licence in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

## Licence

By contributing you agree that your work is released under the [MIT licence](LICENSE).
