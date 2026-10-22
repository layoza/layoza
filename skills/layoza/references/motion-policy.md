# Layoza motion default

Include purposeful motion by default, at the level the brief needs, until the
user requests none. Tools follow the single policy in `layoza-animations` § 2:
the user's prompt wins, design comes first, and CSS, WAAPI, GSAP, Motion or
WebGL are chosen per brief, never mandated. Three.js renders geometry; it does
not replace the timing system. Never load an animation library for a
no-animation request. Keep focus and semantic state updates immediate. Each
animated property has one owner. An existing site keeps its engine unless the
user asks to change it.

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
