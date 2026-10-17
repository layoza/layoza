
## 2026-10-16T11:34:58.949Z recommend_layout
args: {"query":"Saltmarsh, a 40-cover seasonal seafood restaurant in a converted boathouse on the Norfolk coast. Single page: hero mood, chef and day boats, tabbed menu with dietary filter, booking widget, private dining, hours and drawn map, gift vouchers, press quotes.","mode":"new"}

{
  "mode": "new_website",
  "query": "Saltmarsh, a 40-cover seasonal seafood restaurant in a converted boathouse on the Norfolk coast. Single page: hero mood, chef and day boats, tabbed menu with dietary filter, booking widget, private dining, hours and drawn map, gift vouchers, press quotes.",
  "matched_industry": {
    "slug": "hospitality-travel",
    "name": "Hospitality, Hotels, Restaurants & Travel",
    "keywords": [
      "hotel",
      "resort",
      "travel",
      "restaurant",
      "hospitality",
      "booking",
      "dining"
    ],
    "recommended_styles": [
      "atelier-minimalism",
      "kinetic-editorial"
    ],
    "layout_archetype": "Atmospheric Destination Hero + Tactile Booking Bar & Curated Experiences",
    "first_viewport_priority": "Immersive sense of place, frictionless date/guest availability bar, signature experience",
    "typography_pairing": {
      "id": "playfair-karla",
      "display_font": "Playfair Display",
      "body_font": "Karla",
      "mono_font": "Space Mono",
      "mood": "Warm, atmospheric, inviting",
      "scale_ratio": "1.333 (Perfect Fourth)",
      "google_fonts_url": "https://fonts.google.com/share?selection.family=Karla:wght@400;500;600|Playfair+Display:ital,wght@0,600;1,400"
    },
    "palette": {
      "id": "terracotta-olive",
      "background": "#FBF9F5",
      "surface": "#F2ECE1",
      "foreground": "#231F20",
      "muted": "#5F5850",
      "primary_accent": "#9A3412",
      "secondary_accent": "#3F621

## 2026-10-16T11:35:04.980Z get_design_system
args: {"action":"generate","path":"examples/benchmark-restaurant-saltmarsh/layoza"}

{
  "design_md": "---\nversion: \"alpha\"\nname: \"layoza\"\ndescription: \"Extracted and calibrated design system for layoza (html-css-js)\"\ncolors:\n  primary: \"#141619\"\n  background: \"#F8F7F4\"\n  accent: \"#C84B31\"\n  muted: \"#5C6470\"\ntypography:\n  h1:\n    fontFamily: \"Instrument Serif, Georgia, serif\"\n    fontSize: \"clamp(2.5rem, 5vw, 4.25rem)\"\n    fontWeight: \"600\"\n    lineHeight: \"1.08\"\n    letterSpacing: \"-0.025em\"\n  body:\n    fontFamily: \"Plus Jakarta Sans, system-ui, sans-serif\"\n    fontSize: \"1rem\"\n    fontWeight: \"400\"\n    lineHeight: \"1.6\"\n    letterSpacing: \"0em\"\nrounded:\n  lg: \"20px\"\n  md: \"12px\"\n  sm: \"6px\"\nspacing:\n  lg: \"32px\"\n  md: \"16px\"\n  section: \"96px\"\n  sm: \"8px\"\n  xl: \"64px\"\n  xs: \"4px\"\ncomponents:\n  button-primary:\n    backgroundColor: \"{colors.primary}\"\n    textColor: \"{colors.background}\"\n    rounded: \"{rounded.lg}\"\n    padding: \"12px\"\n---\n\n## Overview\n\n`layoza` uses the `html-css-js` framework with Design Variance `4/10`, Motion Intensity `3/10`, and Visual Density `5/10`. Preservation mode is `true`.\n\n## Colors\n\n- **primary (`#141619`)**: Primary brand ink and headline foreground _(Source: `inferred-baseline`, Tier: `InferredRule`)_\n- **background (`#F8F7F4`)**: Primary canvas surface _(Source: `inferred-baseline`, Tier: `InferredRule`)_\n- **accent (`#C84B31`)**: Primary interactive CTA and focus accent _(Source: `inferred-baseline`, Tier: `InferredRule

## 2026-10-16T11:35:45.204Z recommend_animation
args: {"category":"hospitality restaurant single page: tabbed menu, booking confirmation, hero art of a fish crate with landing docket","motion_intensity":5}

no recipe matched `hospitality restaurant single page: tabbed menu, booking confirmation, hero art of a fish crate with landing docket`; valid ids: hero-gsap-masked-split, scroll-gsap-pinned-narrative, section-waapi-stagger-reveal, interactive-magnetic-control, page-transition-view-continuity, threejs-interactive-stage, scroll-lenis-conditional; valid categories: hero_animation, section_animation, scroll_animation, interactive_effect, page_transition, three_experience
[isError]

## 2026-10-16T11:35:50.361Z recommend_animation
args: {"category":"section_animation","motion_intensity":4}

{
  "motion_intensity": 4,
  "count": 1,
  "recipes": [
    {
      "id": "section-waapi-stagger-reveal",
      "name": "Zero-Dependency IntersectionObserver + Native WAAPI Section Reveal",
      "category": "section_animation",
      "engine": "css-waapi",
      "min_motion_intensity": 2,
      "max_motion_intensity": 7,
      "description": "Lightweight native Web Animations API section card and metric reveal with zero external JS bundle weight, ideal for Astro, WordPress Gutenberg, and fast marketing sites.",
      "performance_cost": "very low (0 KB dependency)",
      "reduced_motion_strategy": "Checks `window.matchMedia('(prefers-reduced-motion: reduce)').matches` before scheduling any keyframe animation.",
      "cleanup_contract": "Disconnects `IntersectionObserver` on cleanup or after elements have entered.",
      "code_snippet": "export function initNativeSectionReveals(root = document) {\n  if (typeof window === \"undefined\") return () => {};\n  if (window.matchMedia(\"(prefers-reduced-motion: reduce)\").matches) return () => {};\n\n  const items = Array.from(root.querySelectorAll(\"[data-reveal]\"));\n  const observer = new IntersectionObserver(\n    (entries) => {\n      entries.forEach((entry) => {\n        if (!entry.isIntersecting) return;\n        const el = entry.target;\n        const delay = Number(el.getAttribute(\"data-reveal-delay\") || 0);\n        el.animate(\n          [\n            { opacity: 0, transform: \"translateY(18px)\" },\n            { o

## 2026-10-16T11:45:07.112Z verify_functional
args: {"path":"examples/benchmark-restaurant-saltmarsh/layoza"}

{
  "audit": {
    "target_path": "examples/benchmark-restaurant-saltmarsh/layoza",
    "passed_release_gate": true,
    "objective_error_count": 0,
    "objective_warning_count": 1,
    "subjective_critique_notes": 0,
    "findings": [
      {
        "rule_id": "CWV-NOT-MEASURED",
        "kind": "deterministic_objective",
        "severity": "warning",
        "category": "performance",
        "file_or_selector": ".",
        "message": "Core Web Vitals were not measured, so performance is unverified (not passed).",
        "remediation": "Run `layoza qa <file-or-url>` or pass lab_lcp_ms / lab_inp_ms / lab_cls.",
        "auto_repairable": false
      }
    ],
    "accessibility": {
      "target_standard": "WCAG 2.2 Level AA",
      "automated_violations": 0,
      "passes_automated_wcag22_aa": true,
      "manual_review_required": [
        "Verify keyboard Tab/Shift+Tab focus order matches visual reading sequence across 375px, 768px, and 1280px viewports (WCAG 2.4.3).",
        "Verify focus appearance and non-obscuration when sticky headers or dialogs are active (WCAG 2.4.11 / 2.4.13).",
        "Verify screen-reader announcement clarity for dynamic state changes and live regions (WCAG 4.1.3)."
      ]
    },
    "core_web_vitals": {
      "methodology_source": "laboratory_synthetic",
      "methodology_disclosure": "Laboratory values supplied by the caller, if any. Not CrUX field telemetry.",
      "lcp_ms": null,
      "lcp_passes_
