# Direction log — Layoza site

PURPOSE: Developers who already use a coding agent; make them run `npx layoza init`.
PICKED: The user's master prompt (navy / off-white bands, Space Grotesk + IBM Plex Mono, lime / cyan / violet accents, 12 named sections). No alternative directions were offered because the brief fixes the direction.
HERO: Full-width headline, layered engine below it at the right. The brief asks for this split; the headline leads.
HIERARCHY: Hero headline > section headlines (caps, one accent phrase each) > section-specific object (plates, slider, DNA sheet, motion tiles, orbit, path, QA frames, gallery, blueprint, terminal, mark) > body.

ROUND 1: 1440 review. Workflow curve ran through text, modes row stretched, blueprint text overflowed boxes, hero gap. Fixed.
ROUND 2: 320–1920 full pages. Final-mark scatter crossed the headline on tablet, modes row spacing. Fixed.
ROUND 3: User report — the pinned "AI builds fast" slider showed half plates. Cause: the pinned block was 891px on a 760px screen and the scrub rested between plates. Pinned scene now fits one screen under the nav, centres the active plate and snaps to whole plates; no pin below 620px tall.
ROUND 4: `layoza qa` on the production build: violet small text 3.58:1 on paper, dimmed steps below 4.5:1, final heading leading 0.95, torn ticket stub cut at 375, no Escape on the mobile menu. Fixed. Result: 0 errors, 5 warnings (see qa.json); remaining warnings judged detector limits (pin spacer, orbit list, wireframe drawings).

ROUND 5: Lighthouse mobile home 45 (TBT 2840ms). Causes: font-swap relayouts, GSAP / three.js / ClientRouter on the critical path, eager video poster. Reveals moved to IntersectionObserver, scenes and engine load when idle, fonts preloaded, native view transitions. Home mobile 95–97, every page 96–100 performance and 100 a11y / best practices / SEO. Simulated mobile LCP still 2.5–2.8s.
ROUND 6: QA detector fixes removed two false positives (pin spacer, wire marks); the "ragged row" warning was real (Works-with chips wrapped 5 + 1) and is fixed with an even grid. A heading resting in the bottom 12% of the screen never revealed; reveals now also fire when scrolling settles, and run 0.6s instead of 0.9s. Result: 0 errors, 1 warning (aria-hidden decorative "!" mark, exempt from 1.4.3).

REVIEWER: Not run (no fresh-eyes subagent, kept to budget). Open.
