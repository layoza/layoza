# MOTION.md — layoza-mcp

## 1. Motion Identity Summary

- **Detected Libraries**: CSS Transitions / Native Web Animations API
- **Motion Intensity Dial**: `3/10`
- **Scroll Strategy**: `native` (Native scroll preserved unless explicitly opted in)
- **Honors `prefers-reduced-motion` in Source**: `true`

## 2. Timing & Easing Tokens

- **Durations**: 180ms, 320ms, 600ms
- **Easing Curves**: cubic-bezier(0.22, 1, 0.36, 1)

## 3. Mandatory Motion Quality & Safety Rules

1. **Reduced Motion**: Every CSS animation and JS timeline (`gsap.matchMedia()`, `useReducedMotion()`) MUST provide an instant, readable static fallback when `(prefers-reduced-motion: reduce)` is active.
2. **Zero Content Hiding**: Never leave essential text or CTAs at `opacity: 0` or `visibility: hidden` if JavaScript fails or before ScrollTrigger initializes.
3. **Lifecycle Cleanup**: Clean up all `gsap.context()`, `ScrollTrigger`, `IntersectionObserver`, `requestAnimationFrame`, and Three.js `geometry`/`material`/`renderer` instances on unmount or route transition.
4. **No Unsolicited Scroll Hijacking**: Do not introduce Lenis or custom wheel interceptors unless `motion_intensity >= 7` and smooth narrative scrolling is explicitly appropriate.
