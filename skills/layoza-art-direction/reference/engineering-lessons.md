<!-- Original Layoza content (MIT). Brief-independent engineering lessons; benchmark write-ups live in docs/benchmarks/ and must not be loaded while building a benchmark brief. -->

# Engineering lessons

## 1. CLS from font swap: metric-adjusted fallback

The no-skill control failed CLS (0.136); the impeccable build had the lowest
(0.010). Web fonts swapping in over a default fallback move every line. Declare
a local fallback whose metrics match the web font and put it second in the stack:

```css
@font-face {
  font-family: "Display Fallback";
  src: local("Arial Bold"), local("Arial-BoldMT"), local("Arial");
  font-weight: 600 800;
  size-adjust: 91%;          /* tune per face: match x-height/advance width */
  ascent-override: 100%;
  descent-override: 26%;
  line-gap-override: 0%;
}
@font-face {
  font-family: "Body Fallback";
  src: local("Arial");
  size-adjust: 98%;
  ascent-override: 96%;
  descent-override: 26%;
  line-gap-override: 0%;
}
:root {
  --font-display: "Your Display Face", "Display Fallback", sans-serif;
  --font-body: "Your Body Face", "Body Fallback", sans-serif;
}
```

Take the override values from the font's own metrics (or a tool such as
Fontaine/Capsize) rather than guessing (optional CLS check:
`layoza qa --performance`). Preload only the face the LCP element uses.

## 2. Sticky headers need `scroll-padding-top`

Keyboard focus landing under a sticky header fails WCAG 2.2 SC 2.4.11
(`focus-obscured` in `layoza qa`). One line fixes it:

```css
:root { --header-h: 4.5rem; }
html { scroll-padding-top: calc(var(--header-h) + 1rem); }
```

If the header height changes at breakpoints, update `--header-h` in the same
media query. Re-run `layoza qa`; the check tabs through focusable elements.

## 3. Read both checkers, game neither

Design-taste rules (the `CRAFT-*` notes) encode one school of taste; `layoza qa`
errors encode measurable defects. Use the detector's findings as
taste prompts, the QA errors as hard gates, and never tune a page to a
checker's blind spot.
