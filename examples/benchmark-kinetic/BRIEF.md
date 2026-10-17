# Brief R10: Kinetic Signal agency homepage (client prompt, verbatim below)

## Benchmark harness notes (same for every arm)
- No existing project: build a new single file, `index.html`, with inline CSS and JS.
- Libraries may load from a CDN (cdnjs.cloudflare.com, cdn.jsdelivr.net, unpkg.com) with pinned versions. Fonts come from Google Fonts. No external images or videos; build artwork with CSS, SVG, canvas or WebGL.
- The agency has no real name, clients, projects or testimonials. Use a clearly editable placeholder wordmark and clearly labelled demonstration projects. Never invent real clients, outcomes or endorsements; omit testimonials, as the prompt says.
- The contact CTA must reach a working on-page form or a mailto link.
- Verify in a real browser at 1440, 768 and 375 widths.

---

## Client prompt

MASTER AI PROMPT: WORLD-CLASS ANIMATED AGENCY HOMEPAGE

### 1. Your role and objective
Act as an elite creative director, award-winning UI/UX designer, senior creative developer, 3D motion designer, GSAP animation expert, and frontend performance engineer.

Your mission is to design and build an extraordinary, fully functional, production-quality digital agency homepage that can compete visually with the best creative agency websites featured on Awwwards, CSS Design Awards, and similar design showcases.

The homepage must feel like a digital art experience while remaining a highly effective business website.

Creative direction:
* Premium, experimental, sophisticated, and confident.
* Minimalist layout with bold, oversized typography.
* Cinematic motion, meaningful micro-interactions, and carefully orchestrated scroll animations.
* Beautiful 2D and selective 3D compositions.
* Unique custom-designed visuals instead of generic marketing templates.
* Every section must have its own creative identity while feeling part of one cohesive experience.
* Dynamic but never chaotic.
* Smooth, immersive, responsive, and optimized for real devices.

Important: Do not create a basic agency landing page with ordinary cards, predictable gradients, generic floating circles, or repeated fade-in animations. Create something that feels designed by a top-tier creative studio.

### 2. Design concept: "Kinetic Signal"
Build the entire homepage around a distinctive visual concept called Kinetic Signal. The concept represents how an agency transforms ideas into powerful digital experiences.

Use a custom animated, sculptural ribbon as the core visual identity. It should feel like a living design element that folds, flows, rotates, and reacts to interactions. This visual language should influence typography, layouts, transitions, section dividers, interactive elements, and motion design.

Use deliberate visual contrast between dark immersive sections and bright editorial sections.

**Exact colour system**

Core colours:
* Primary background: `#090B11` (Obsidian black)
* Secondary dark: `#151923` (Deep graphite)
* Main light background: `#F6F5F0` (Warm porcelain)
* Primary light text: `#F6F5F0`
* Primary dark text: `#111318`
* Secondary text: `#9299A8`
* Border colour on dark: `#2B303B`
* Border colour on light: `#D9DAD6`

Signature accents:
* Electric cobalt: `#4865FF`
* Cobalt hover: `#304EF0`
* Signal lime: `#D8FF57`
* Soft lavender: `#A99BFF`

Use cobalt as the principal brand accent. Signal lime should appear sparingly for striking moments, highlights, active states, and special interaction feedback. Avoid excessive gradients and multiple competing accent colours.

Section colour rhythm:
1. Hero: Obsidian
2. Agency introduction: Warm porcelain
3. Services: Obsidian
4. Featured work: Warm porcelain
5. Agency philosophy: Cobalt blue
6. Process: Warm porcelain
7. Experimental showcase: Obsidian
8. Testimonials / insights: Warm porcelain
9. Contact finale: Obsidian with electric cobalt visual highlights

Transitions between these sections must feel intentional and beautifully animated.

### 3. Typography system
Use strong editorial typography inspired by premium design studios.
* Headlines: Space Grotesk or an equally distinctive, high-quality geometric display typeface.
* Body: Inter or an equally polished, highly readable modern sans-serif.

Design principles:
* Oversized hero headline, approximately 100–150px on large desktop screens.
* Responsive typography using `clamp()`.
* Tight headline tracking.
* Strong contrast between large display typography and small editorial labels.
* Beautiful line breaks rather than awkward automatic wrapping.
* Carefully controlled text alignment, baselines, and visual rhythm.
* Small uppercase section labels with generous letter spacing.
* A consistent spacing and typography system.

Headlines should feel architectural and intentionally composed.

**Typography animations.** Use different motion treatments only where they serve the composition:
* Split-line text reveals.
* Staggered word entrance.
* Character masking.
* Letter tracking expansion.
* Moving typographic outlines.
* Scroll-controlled headline reveals.
* Animated highlighted keywords.
* Subtle perspective-based title entrance.

Avoid applying identical animation presets everywhere.

### 4. Animation technologies
Use the best tool for each animation instead of forcing everything into one library.

**GSAP + ScrollTrigger.** Primary animation engine for:
* Hero entrance timeline.
* Scroll storytelling.
* Pinned sections.
* Parallax layers.
* Text reveal timelines.
* Horizontal visual transitions.
* Mask reveals.
* Section colour changes.
* Coordinated complex animations.

**Framer Motion / Motion.** Use for:
* Component entrance and exit.
* Navigation transitions.
* Mobile menu.
* Interactive buttons.
* Cards and accordions.
* Stateful micro-interactions.
* Layout animations.

**Three.js / React Three Fiber (when appropriate).** Use for:
* Custom 3D hero sculpture.
* Interactive ribbon geometry.
* Camera movement.
* Material and lighting animation.
* Subtle pointer-based 3D interaction.

**WebGL / shaders.** Only where necessary to make the custom visual distinctive and performant.

**Smooth scrolling.** Use Lenis when it integrates cleanly with ScrollTrigger and the existing project. Avoid conflicting scroll systems.

Animation rules:
* Aim for 60 FPS on capable devices.
* Prefer GPU-friendly transform and opacity animations.
* Typical micro-interactions: 180–350ms.
* Typography entrances: 600–1000ms.
* Major transitions: 900–1500ms.
* Use premium easing curves such as `power3.out`, `expo.out`, and appropriately tuned custom easing.
* Avoid uncontrolled bouncing.
* Avoid unnecessarily long animation delays.
* Support reduced-motion preferences.

### 5. Section 1: Cinematic navigation + hero
This is the most important section. The visitor must immediately feel that this is an extraordinary agency website.

**Header design.** Create a minimal, sophisticated navigation.
* Left: agency logo or editable wordmark.
* Centre: Work, Services, About, Insights.
* Right: "Start a Project" CTA with a custom animated arrow.

Header requirements:
* Transparent over the dark hero.
* Beautiful typography and precise alignment.
* Sticky positioning where appropriate.
* Smooth transformation into a compact navigation after scrolling.
* Animated underline and magnetic button response.
* Excellent mobile navigation with a full-screen animated menu.
* No clutter.

**Hero layout.** Full viewport height with generous breathing room.

Left side: oversized editorial typography.
* Example headline: "WE MAKE" / "IDEAS MOVE."
* The word "MOVE" should have a distinctive animated treatment.
* Supporting paragraph: "We build remarkable brands, digital experiences, and campaigns that create real momentum."
* Primary CTA: "Explore Our Work"
* Secondary CTA: "Let's Talk"

Right side: an extraordinary custom Three.js visual.

**Three.js hero sculpture.** Create a sculptural, folded, flowing metallic ribbon. It should:
* Feature a refined silver material with subtle cobalt reflections.
* Have soft cinematic lighting and convincing depth.
* Move continuously and very slowly while idle.
* React subtly to pointer movement.
* Rotate with controlled damped motion.
* Change orientation during the entrance animation.
* Have beautiful silhouettes, not random 3D geometry.
* Avoid looking like a common rotating sphere or generic particle orb.

Use the sculpture as a sophisticated part of the composition, not a distracting decoration.

**Hero entrance sequence**
* 0.0–0.4s: Background and interface establish the visual space.
* 0.2–0.9s: Logo and navigation enter smoothly.
* 0.35–1.25s: Hero headline reveals line by line through masks, with gentle vertical movement.
* 0.7–1.6s: 3D ribbon unfolds from a compressed state into its main composition.
* 1.1–1.7s: Supporting paragraph reveals.
* 1.3–1.9s: CTA buttons appear.
* 1.7s onward: Ambient sculpture animation begins.

Provide immediate readable content even while optional animation modules initialise.

**Hero scroll transition.** As the visitor scrolls:
* Headline subtly shifts position.
* The 3D sculpture rotates and moves toward the centre.
* The CTA and metadata recede naturally.
* The composition transitions into the next section through a carefully controlled mask or shape reveal.

No abrupt cuts or artificial scroll hijacking. Add a small animated scroll indicator with an elegant vertical line.

### 6. Section 2: Agency introduction
Background: warm porcelain. Create a sophisticated editorial split layout.
* Small label: "01 / WHO WE ARE"
* Large statement: "Not just another agency. A creative force for what's next."

Use an asymmetrical grid. One side features oversized typography. The other side contains a concise agency description with a supporting animated abstract graphic.

**Scroll animation.** As the section enters:
* Background transitions smoothly from dark to light.
* Text reveals in two or three carefully choreographed stages.
* Selected important words become visually emphasised.
* Thin dividers extend horizontally.
* Supporting elements enter with subtle offsets.

Do not animate every individual character unnecessarily. Add a memorable typographic interaction where a key phrase transforms from an outlined state to a solid state as the user scrolls.

### 7. Section 3: Interactive services experience
Background: obsidian.
* Heading: "WHAT WE DO."
* Subheading: "Different disciplines. One obsessive standard."

Create a large interactive services experience, not ordinary service cards. Suggested services, to be replaced by the agency's actual offerings:
1. Brand Strategy & Identity
2. Website Design & Development
3. Digital Marketing & Growth
4. Motion Design & Experiences

**Desktop layout**
* Left side: large interactive service titles stacked vertically.
* Right side: a generous visual preview stage containing animated compositions representing each service.

When a service becomes active:
* Its title expands or receives a refined typographic transformation.
* A short service description appears.
* A relevant visual composition animates into view.
* The previous composition exits elegantly.
* A section index updates.
* A thin progress line changes.

**Individual service visuals**
* Brand Identity: animated layered typography, shapes, and brand compositions.
* Website Development: a premium website composition smoothly transitioning between layouts.
* Digital Marketing: an abstract signal visualisation with animated paths, data-inspired compositions, and kinetic shapes.
* Motion Design: a sophisticated looping motion artwork.

Use GSAP timelines and Motion layout transitions.

**Scroll interaction.** The services section may use a controlled sticky visual stage on desktop. Each service becomes active at a defined scroll interval. On mobile, turn the experience into accessible vertical interactive panels rather than forcing horizontal scrolling.

### 8. Section 4: Featured projects
Background: warm porcelain.
* Large heading: "SELECTED WORK."
* Subheading: "Proof looks better than promises."

Show 3–4 premium project showcases. Do not use generic small cards. Use large edge-to-edge or editorial project layouts with significant imagery. Alternate layouts to establish rhythm:
* One large cinematic project.
* Two staggered editorial projects.
* One full-width immersive feature.

**Project presentation.** Each project should show:
* Project name
* Category
* Year, if available
* Short project description
* Relevant imagery or motion preview
* "View Project" interaction

**Animation behaviour.** On scroll:
* Project visuals reveal using masks.
* Images animate from subtle scale to their final position.
* Text enters with precise timing.
* Staggered layouts create controlled depth.

On hover:
* Image transitions smoothly.
* A circular "View Case" cursor or label appears where appropriate.
* Typography changes subtly.
* Background media responds with small parallax movement.

Use real agency projects when available. Never fabricate actual clients, testimonials, or measurable project outcomes. If project content is missing, use clearly editable demonstration content.

### 9. Section 5: Brand philosophy
Background: electric cobalt. This section should provide a dramatic visual change.

Create a large cinematic typographic statement, "GOOD DESIGN IS SEEN.", then transition into "GREAT DESIGN IS FELT." Use very large white or warm-porcelain typography.

**Animation.** Build one coordinated GSAP timeline.
* First statement enters.
* Typography shifts position.
* Selected words transform.
* Final statement becomes the dominant composition.
* A subtle moving line or sculptural shape supports the statement.

The visual should be bold, minimal, and emotional. Avoid unnecessary illustrations or decorative cards.

### 10. Section 6: Our process
Background: warm porcelain. Heading: "FROM POSSIBILITY TO IMPACT."

Create an interactive process experience with four stages:
1. Discover
2. Define
3. Create
4. Launch & Evolve

**Design.** A refined timeline with large stage numbers, animated connectors, and changing contextual artwork.

**Desktop animation.** Use scroll-driven progression. As each stage becomes active:
* The stage number grows or changes position.
* The description reveals.
* The illustration transforms.
* Timeline progress advances.
* Previous stages remain understandable.

The process should feel like a coherent sequence rather than four disconnected cards. On mobile, use a beautiful vertical timeline with native scrolling.

### 11. Section 7: Creative experiment / showreel
Background: obsidian. This section should be a visual highlight of the entire homepage.

Heading: "BUILT TO MAKE YOU FEEL SOMETHING."

Create a large experimental media stage that can showcase motion design, creative campaigns, 3D work, or a showreel. Use a sophisticated composition with:
* Large motion media.
* Oversized typography overlay.
* Responsive video or canvas presentation.
* Custom play/pause controls.
* Refined transitions.
* Layered subtle motion.

If no video exists, use a custom coded animation as a polished placeholder rather than a broken video player.

**Immersive transition.** On entering:
* The media stage expands.
* The surrounding layout recedes.
* Typography animates into its final position.
* The custom creative sequence begins.

Respect autoplay restrictions, mute autoplaying media, and pause offscreen rendering.

### 12. Section 8: Testimonials and insights
Background: warm porcelain. Keep this section more restrained after the immersive showcase.

Create a premium editorial testimonial feature if genuine testimonials are available. Use large quotes, understated pagination, and a tasteful animated transition.

For insights, display 2–3 articles or creative thinking pieces with distinctive typography and imagery. Avoid generic blog cards and carousel overload. Use subtle hover image reveals and soft typography transitions.

If genuine testimonials are unavailable, omit the testimonial component rather than inventing endorsements.

### 13. Section 9: Massive contact finale
Background: obsidian. This should be one of the most memorable sections.
* Small heading: "HAVE SOMETHING IN MIND?"
* Massive typography: "LET'S MAKE" / "IT MATTER."

Make the statement occupy much of the visual composition. Below it:
* "Start a Project" CTA.
* Email address.
* Social links.
* Brief contact message.

**Contact animation.** When the user scrolls into this section:
* The background transitions smoothly to dark.
* Large text reveals with a cinematic stagger.
* An elegant moving cobalt shape or ribbon crosses behind the typography.
* The primary CTA enters with a subtle magnetic effect.
* The cursor interaction becomes slightly more expressive.

**CTA interaction.** Create a premium rounded button. On hover:
* Background changes from porcelain to cobalt or signal lime.
* Arrow shifts diagonally.
* Text responds with a smooth sliding transition.
* A subtle internal shape expands.

The CTA must link to a working contact route or form.

### 14. Footer experience
Use an oversized but uncluttered editorial footer. Include:
* Agency name / wordmark.
* Navigation.
* Contact details.
* Social profiles.
* Location, if known.
* Copyright year.
* Back-to-top interaction.

Add one tasteful, large typographic or abstract visual motif. Animate footer appearance naturally. Do not overcomplicate the footer.

### 15. Global micro-interactions
Implement a cohesive interaction language throughout the homepage.

**Custom cursor.** On capable desktop pointer devices:
* Small precise cursor follower.
* Smooth magnetic interactions.
* Contextual state changes.
* "View" state over project media.
* "Drag" state only where dragging actually works.

Hide the custom cursor on touch devices and preserve a normal functional cursor as fallback.

**Buttons**
* Masked text-slide transitions.
* Subtle directional arrow motion.
* Magnetic hover with small displacement.
* Refined fill transitions.

**Links**
* Directional underline reveals.
* Typography colour shifts.
* No distracting repeated effects.

**Cards and media**
* Subtle scale transformations.
* Progressive image reveal.
* Gentle pointer-responsive parallax.
* Smooth hover exits.

**Page navigation**
* Optional elegant route transitions when the existing application architecture supports them.
* Ensure route changes remain fast, accessible, and reliable.

### 16. Responsive design
Design intentionally for:
* 1920px large desktops.
* 1440px desktops.
* 1024px tablets.
* 768px small tablets.
* 390px and 375px mobile screens.

Do not simply shrink the desktop design. On mobile:
* Recompose the hero typography.
* Simplify 3D complexity.
* Preserve an elegant focal visual.
* Replace pinned sequences that harm usability.
* Disable custom cursor interactions.
* Maintain readable spacing.
* Keep CTA buttons easy to tap.
* Ensure mobile menus and forms work correctly.
* Avoid horizontal overflow.
* Prevent scroll-trapping.

The mobile website must feel as deliberately designed as the desktop website.

### 17. Performance and accessibility
The website must be visually exceptional without being unnecessarily heavy. Requirements:
* Smooth native-feeling scrolling.
* Optimised GSAP timelines.
* Clean ScrollTrigger initialisation and cleanup.
* Avoid duplicate animation instances.
* Lazy-load nonessential Three.js scenes.
* Use responsive image sizes and modern media formats.
* Respect `prefers-reduced-motion`.
* Support keyboard navigation.
* Maintain accessible colour contrast.
* Provide focus states.
* Use semantic HTML.
* Prevent layout shifts.
* Avoid rendering heavy offscreen scenes continuously.
* Pause unnecessary animation in background tabs where practical.
* Optimise for Core Web Vitals and real-device performance.

Provide a static or reduced-motion fallback whenever WebGL is unavailable. Do not sacrifice usability for animation.

### 18. Implementation rules
Before creating anything, inspect the existing project if one is available. Determine:
* Existing framework and project structure.
* Existing brand assets.
* Agency name and actual services.
* Existing components.
* Current design conventions.
* Existing content and routes.

Preserve working functionality and use the current framework wherever possible. If the project uses React, use Motion and Three.js integrations appropriate to that architecture. If it uses another framework, choose equivalent compatible implementations.

Do not introduce unnecessary dependencies. Create reusable components and organised animation logic. Build actual responsive interfaces with functioning interactions. Do not deliver only static mockups, screenshots, wireframes, or disconnected animation demonstrations.

Avoid:
* Generic SaaS layouts.
* Repetitive three-column service cards.
* Random neon gradients.
* Excessive glassmorphism.
* Generic floating circles.
* Poorly designed particle backgrounds.
* Overused spinning 3D spheres.
* Excessive scroll pinning.
* Identical fade-in effects in every section.
* Unreadable typography.
* Fake business credibility.
* Unnecessary animation delays.

### 19. Quality control
After implementation, verify:
1. Hero renders immediately and animations run correctly.
2. All sections have a consistent visual identity.
3. Every interactive control works.
4. Three.js scene works and gracefully degrades.
5. Animations remain smooth during scrolling.
6. ScrollTrigger timelines do not conflict.
7. Navigation works across desktop and mobile.
8. There is no unintended horizontal overflow.
9. Reduced-motion preferences are respected.
10. Layouts work across requested breakpoints.
11. Content remains accessible and readable.
12. There are no major browser console errors.
13. The homepage feels visually premium without animation overload.
14. Contact actions work correctly.
15. All sections look complete and production-ready.

Test in a real browser when available and fix issues discovered during testing.

### 20. Final creative standard
I do not want an ordinary agency template. I want a memorable, expertly art-directed agency homepage with strong typography, intentional colour contrast, custom visuals, and a distinctive motion identity.

Every section should offer something special, but the experience must remain unified. Think like an elite creative studio with dedicated art directors, motion designers, interaction designers, and senior frontend engineers.

The final result should feel like a custom-made, award-caliber digital experience that could genuinely represent a premium international creative agency.

Make the design visually powerful. Make the animation cinematic. Make every interaction intentional. Make the engineering clean, fast, and dependable.

Now design and implement the complete homepage. Do not stop at planning or concept generation.
