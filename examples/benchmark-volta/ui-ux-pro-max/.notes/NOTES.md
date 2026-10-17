# Dev notes — ui-ux-pro-max arm
- design-system.md: output of `search.py --design-system -p "Volta One" --variance 7 --motion 7 --density 3`.
- searches.md: supplementary domain/stack searches (style, typography x2, color, landing, threejs stack, ux).
- Adopted: pattern Scroll-Triggered Storytelling + Interactive 3D Configurator (landing), style 3D Product Preview,
  warm-stone neutral + amber accent (design-system colours), threejs stack rules (live reduced-motion listener,
  custom spherical drag orbit, button + keyboard alternatives, pause offscreen/hidden).
- Rejected: Cinzel/Josefin (real-estate mood, off-fit) -> Space Grotesk from typography retry.
- Not adopted: GSAP snippet from motion dial (no GSAP needed; motion is in the render loop).
- QA: qa/ screenshots from Playwright Chromium (SwiftShader WebGL) at 1440x900, 768x1024, 375x812 + no-WebGL/reduced-motion.
