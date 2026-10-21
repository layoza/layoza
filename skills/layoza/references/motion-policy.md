# Layoza motion default

Include purposeful animation by default until the user requests none. GSAP owns
authored DOM/SVG choreography, scroll sequences and Three.js state animation.
Three.js renders geometry; it does not replace the timing system. Load a pinned
runtime and use the original lifecycle controller at
`layoza-animations/assets/gsap-motion.mjs` where it fits. Never load an animation
library for a no-animation request. Keep focus and semantic state updates immediate.

The user may explicitly request another engine. When Motion for React / Framer
Motion is requested alongside GSAP, use it for React component presence/layout
and use GSAP for separate scene/choreography properties. Each property has one
owner; use wrapper elements when both systems contribute to one composition.
Do not reinterpret a normal internal API request as permission to change engine.

A calm page still receives restrained, useful feedback; this default does not
force 3D, pinning, magnetic cursors or an effect on every heading. An expressive
brief receives an authored sequence and section handoffs, not one repeated fade.
Reduced motion resolves to complete stills, long-running motion has a pause
control, and scoped timelines/listeners/tickers are removed on teardown. Preserve
native scrolling unless the brief requests a smooth-scroll showcase.

For rich-motion briefs, load `layoza-animations/reference/section-choreography.md`.
Author a section map through MCP `recommend_animation.motion_plan`, then build
actual content transformations with `section-motion.mjs`. One coherent thesis
can have several expressive chapter mechanisms; it must not collapse into one
hero plus the same reveal everywhere. Author phone staging separately. Capture
normal scroll states with `node scripts/review-motion.mjs <URL> --out <directory>`
and inspect continuous playback. Motion evidence and static design quality are
separate: neither a written plan nor a passing button test proves good animation.
