// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Premium Animation & 3D Engine: reusable recipes and context-sensitive recommendation system
// across GSAP (ScrollTrigger, SplitText, MorphSVG, MotionPath), Motion, Three.js / R3F / Drei,
// CSS / Native WAAPI, and conditional Lenis smooth scrolling.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotionCategory {
    HeroAnimation,
    SectionAnimation,
    ScrollAnimation,
    InteractiveEffect,
    PageTransition,
    ThreeExperience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationRecipe {
    pub id: String,
    pub name: String,
    pub category: MotionCategory,
    pub engine: String,
    pub min_motion_intensity: u8,
    pub max_motion_intensity: u8,
    pub description: String,
    pub performance_cost: String,
    pub reduced_motion_strategy: String,
    pub cleanup_contract: String,
    pub code_snippet: String,
    pub css_fallback: String,
}

/// Returns the catalog of production-tested, accessible animation and 3D recipes.
pub fn catalog_animation_recipes() -> Vec<AnimationRecipe> {
    let mut recipes = vec![
        AnimationRecipe {
            id: "scroll-section-choreography".into(),
            name: "Authored responsive section choreography and actual scroll inspection".into(),
            category: MotionCategory::ScrollAnimation,
            engine: "gsap-scrolltrigger".into(),
            min_motion_intensity: 1,
            max_motion_intensity: 10,
            description: "Executable section host with caller-authored desktop and phone timelines. Choose actual transformations through recommend_animation.motion_plan; this helper owns responsive ScrollTriggers, lifecycle and measured state inspection, not the visual concept. Run scripts/review-motion.mjs for real scroll evidence; authored or captured does not mean beautiful.".into(),
            performance_cost: "Depends on caller-authored scenes; no speed score.".into(),
            reduced_motion_strategy: "Dynamic reduced motion removes all timelines and pins; resolve callbacks preserve complete readable stills.".into(),
            cleanup_contract: "destroy() reverts matchMedia, timeline ownership, scroll triggers, inspection registry and pending media refresh; return it from the GSAP lifecycle composition.".into(),
            code_snippet: include_str!("../../../skills/layoza-animations/assets/section-motion.mjs").into(),
            css_fallback: "/* Author readable resolved CSS before initializing any scene. */".into(),
        },
        AnimationRecipe {
            id: "threejs-scene-staging".into(),
            name: "Fit the actual spatial pose and explain its real progression".into(),
            category: MotionCategory::ThreeExperience,
            engine: "threejs".into(),
            min_motion_intensity: 0,
            max_motion_intensity: 10,
            description: "Executable fitPerspectiveBox and scenePhase helpers. Fit actual posed model bounds, excluding backdrop/floor, to the camera's aspect and view direction; this avoids both a postage-stamp desktop object and cropped phone geometry. Use the brief's actual object and phases; match semantic captions to geometry thresholds, keep them in a reserved HTML strip outside the canvas, and preserve a resolved paused state. Helpers do not author an identity, scene or animation timeline for you.".into(),
            performance_cost: "CPU pose bounds and camera fit; invoke on pose/size changes. No speed score.".into(),
            reduced_motion_strategy: "Fit the resolved static pose and show its accurate phase explanation; no RAF or listeners are created by these helpers.".into(),
            cleanup_contract: "Pure helpers allocate temporary vectors only, no persistent resources or listeners; caller retains ownership of renderer, meshes and lifecycle.".into(),
            code_snippet: include_str!("../../../skills/layoza-threejs-webgl/assets/scene-staging.mjs").into(),
            css_fallback: ".scene-frame{position:relative;isolation:isolate}.scene-caption{position:relative;margin-block-start:1rem}.scene-frame canvas{display:block;width:100%;height:100%}".into(),
        },
        AnimationRecipe {
            id: "threejs-spatial-work".into(),
            name: "Real artwork unfolds into a spatial portfolio and selected work".into(),
            category: MotionCategory::ThreeExperience,
            engine: "threejs".into(),
            min_motion_intensity: 7,
            max_motion_intensity: 10,
            description: "Executable, framework-neutral stage with injected pinned THREE runtime and client-supplied artwork/colours. Curved image surfaces unfold into a triptych, then a selected project comes forward; phone reframes a layered stack. setProgress(0..1) connects to native scroll or GSAP. Keep semantic captions, real links and a complete HTML artwork poster outside the decorative canvas; retain the poster until all textures load. This is a starting mechanism, not an agency house style or proof of artistic quality.".into(),
            performance_cost: "WebGL stage; pause offscreen and when document hidden. Speed scoring is optional.".into(),
            reduced_motion_strategy: "Dynamic media-query changes and setPaused(true) cancel the RAF and show a resolved still; semantic HTML remains usable. Failed texture/context keeps HTML poster.".into(),
            cleanup_contract: "dispose() aborts listeners, cancels RAF, disconnects resize/intersection observers and disposes all geometry, textures, materials and renderer, including late texture arrivals.".into(),
            code_snippet: include_str!("../../../skills/layoza-threejs-webgl/assets/spatial-work-stage.mjs").into(),
            css_fallback: ".spatial-stage{position:relative;min-height:360px}.spatial-stage canvas{position:absolute;inset:0;width:100%;height:100%;pointer-events:none}.spatial-poster{position:relative;z-index:1}.spatial-stage[data-webgl-ready=true] .spatial-poster{visibility:hidden}".into(),
        },
        AnimationRecipe {
            id: "scroll-composition-to-work".into(),
            name: "Identity composition resolves into project work".into(),
            category: MotionCategory::ScrollAnimation,
            engine: "gsap-scrolltrigger".into(),
            min_motion_intensity: 7,
            max_motion_intensity: 10,
            description: "A coordinated composition transformation: identity planes align into a project viewport, the viewport expands and the work caption resolves. Use real supplied work. The art changes its role instead of merely floating; desktop scrubs a reserved stage and phones restage it without pinning. Example mechanism, not a required house style.".into(),
            performance_cost: "One reserved transform stage; no performance score implied".into(),
            reduced_motion_strategy: "Final art and captions remain visible in source CSS. MatchMedia removes pinning and transformations when reduced motion changes live.".into(),
            cleanup_contract: "The returned cleanup reverts only this scene's context and media queries; invoke it on route teardown or pause, then reinitialize on resume.".into(),
            code_snippet: r#"import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
gsap.registerPlugin(ScrollTrigger);
// Mark a source-visible scene with data-work-scene; its reserved stage contains
// data-work-frame (actual project art) and optional data-work-plane ornaments.
// Caption and links remain outside the art layers. CSS below is the final state.
export function initCompositionToWork(scene) {
  if (!scene) return () => {};
  const stage = scene.querySelector('[data-work-stage]');
  const frame = scene.querySelector('[data-work-frame]');
  if (!stage || !frame) return () => {};
  const planes = scene.querySelectorAll('[data-work-plane]');
  const mm = gsap.matchMedia();
  mm.add({ desktop: '(min-width: 900px)', motion: '(prefers-reduced-motion: no-preference)' }, ({conditions}) => {
    if (!conditions.motion) return;
    const desktop = conditions.desktop;
    const tl = gsap.timeline({ scrollTrigger: {
      trigger: scene, start: desktop ? 'top top' : 'top 80%',
      end: desktop ? '+=110%' : 'bottom 30%', pin: desktop ? stage : false,
      scrub: 0.7, invalidateOnRefresh: true
    }});
    // One timeline owns each transform: initial scattered identity -> aligned
    // planes -> expanded project. Keep real copy readable throughout.
    tl.fromTo(planes, {
      xPercent: i => (i % 2 ? 45 : -45), rotation: i => (i % 2 ? 24 : -24),
      scaleY: 0.35
    }, { xPercent: 0, rotation: 0, scaleY: 1, duration: 0.45, stagger: 0.06 }, 0)
    .fromTo(frame, {scale: desktop ? 0.58 : 0.88, rotation: desktop ? -8 : -3},
      {scale: 1, rotation: 0, duration: 0.8, ease: 'power2.inOut'}, 0.15)
    .to(planes, {scaleX: 0, transformOrigin: 'right center', duration: 0.4, stagger: 0.05}, 0.65);
  }, scene);
  return () => mm.revert();
}"#.into(),
            css_fallback: r#"[data-work-scene]{position:relative}
[data-work-stage]{position:relative;min-height:clamp(360px,65vh,820px);display:grid;place-items:center;overflow:clip;isolation:isolate}
[data-work-frame]{position:relative;z-index:1;width:100%;margin:0;transform:none}
[data-work-frame] img{display:block;width:100%;height:auto}
[data-work-plane]{position:absolute;inset:12% 8%;pointer-events:none;z-index:0}
@media(prefers-reduced-motion:reduce){[data-work-plane]{display:none}}
"#.into(),
        },
        AnimationRecipe {
            id: "hero-gsap-masked-split".into(),
            name: "GSAP Masked Headline & Staggered Hero Entrance".into(),
            category: MotionCategory::HeroAnimation,
            engine: "gsap".into(),
            min_motion_intensity: 4,
            max_motion_intensity: 10,
            description: "Reveals hero display lines from overflow-hidden masks with exponential easing, paired with `gsap.matchMedia()` so reduced-motion users see instant static text.".into(),
            performance_cost: "low (compositor transform + opacity only)".into(),
            reduced_motion_strategy: "Uses `gsap.matchMedia()` `(prefers-reduced-motion: no-preference)`; defaults to `opacity: 1; transform: none` in CSS so content is never hidden if JS fails.".into(),
            cleanup_contract: "Wrapped in `gsap.context(() => { ... }, rootRef)` and calls `ctx.revert()` on unmount.".into(),
            code_snippet: r#"// GSAP Masked Hero Reveal with Reduced-Motion & React/Vanilla Cleanup
import gsap from "gsap";

export function initHeroEntrance(containerEl) {
  if (!containerEl) return () => {};
  const mm = gsap.matchMedia();
  const ctx = gsap.context(() => {
    mm.add("(prefers-reduced-motion: no-preference)", () => {
      gsap.fromTo(
        "[data-animate='hero-line']",
        { yPercent: 105, opacity: 0 },
        {
          yPercent: 0,
          opacity: 1,
          duration: 0.78,
          stagger: 0.08,
          ease: "power3.out",
          clearProps: "transform"
        }
      );
      gsap.fromTo(
        "[data-animate='hero-cta']",
        { y: 16, opacity: 0 },
        { y: 0, opacity: 1, duration: 0.55, delay: 0.28, ease: "power2.out" }
      );
    });
  }, containerEl);

  return () => {
    mm.revert();
    ctx.revert();
  };
}"#.into(),
            css_fallback: r#"[data-animate="hero-line"], [data-animate="hero-cta"] {
  opacity: 1;
  transform: none;
  will-change: transform, opacity;
}
@media (prefers-reduced-motion: reduce) {
  [data-animate="hero-line"], [data-animate="hero-cta"] {
    transition: none !important;
    animation: none !important;
    transform: none !important;
    opacity: 1 !important;
  }
}"#.into(),
        },
        AnimationRecipe {
            id: "scroll-gsap-pinned-narrative".into(),
            name: "GSAP ScrollTrigger Pinned Narrative Showcase".into(),
            category: MotionCategory::ScrollAnimation,
            engine: "gsap-scrolltrigger".into(),
            min_motion_intensity: 6,
            max_motion_intensity: 10,
            description: "Synchronizes a sticky visual stage with sequential narrative steps on desktop, gracefully stacking as a clean vertical flow on mobile (`< 768px`) and under `prefers-reduced-motion`.".into(),
            performance_cost: "medium (ScrollTrigger scrub)".into(),
            reduced_motion_strategy: "Pins only when `(min-width: 768px) and (prefers-reduced-motion: no-preference)`; falls back to natural document scroll otherwise.".into(),
            cleanup_contract: "Kills all ScrollTrigger instances and reverts inline pin spacers via `mm.revert()`.".into(),
            code_snippet: r#"import gsap from "gsap";
import { ScrollTrigger } from "gsap/ScrollTrigger";
gsap.registerPlugin(ScrollTrigger);

export function initPinnedNarrative(sectionEl) {
  if (!sectionEl) return () => {};
  const mm = gsap.matchMedia();
  mm.add("(min-width: 768px) and (prefers-reduced-motion: no-preference)", () => {
    const panels = gsap.utils.toArray("[data-narrative-panel]", sectionEl);
    const tl = gsap.timeline({
      scrollTrigger: {
        trigger: sectionEl,
        start: "top top",
        end: () => `+=${panels.length * 420}`,
        pin: true,
        scrub: 0.6,
        invalidateOnRefresh: true
      }
    });
    panels.forEach((panel, i) => {
      if (i > 0) {
        tl.fromTo(panel, { yPercent: 25, opacity: 0 }, { yPercent: 0, opacity: 1, duration: 1 }, i - 0.3);
      }
    });
  });
  return () => mm.revert();
}"#.into(),
            css_fallback: r#"[data-narrative-panel] { opacity: 1; }"#.into(),
        },
        AnimationRecipe {
            id: "section-waapi-stagger-reveal".into(),
            name: "Zero-Dependency IntersectionObserver + Native WAAPI Section Reveal".into(),
            category: MotionCategory::SectionAnimation,
            engine: "css-waapi".into(),
            min_motion_intensity: 2,
            max_motion_intensity: 7,
            description: "Lightweight native Web Animations API section card and metric reveal with zero external JS bundle weight, ideal for Astro, WordPress Gutenberg, and fast marketing sites.".into(),
            performance_cost: "very low (0 KB dependency)".into(),
            reduced_motion_strategy: "Checks `window.matchMedia('(prefers-reduced-motion: reduce)').matches` before scheduling any keyframe animation.".into(),
            cleanup_contract: "Disconnects `IntersectionObserver` on cleanup or after elements have entered.".into(),
            code_snippet: r#"export function initNativeSectionReveals(root = document) {
  if (typeof window === "undefined") return () => {};
  if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) return () => {};

  const items = Array.from(root.querySelectorAll("[data-reveal]"));
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (!entry.isIntersecting) return;
        const el = entry.target;
        const delay = Number(el.getAttribute("data-reveal-delay") || 0);
        el.animate(
          [
            { opacity: 0, transform: "translateY(18px)" },
            { opacity: 1, transform: "translateY(0px)" }
          ],
          {
            duration: 520,
            delay,
            easing: "cubic-bezier(0.22, 1, 0.36, 1)",
            fill: "both"
          }
        );
        observer.unobserve(el);
      });
    },
    { threshold: 0.14 }
  );

  items.forEach((el) => observer.observe(el));
  return () => observer.disconnect();
}"#.into(),
            css_fallback: r#"[data-reveal] { opacity: 1; transform: translateY(0); }"#.into(),
        },
        AnimationRecipe {
            id: "micro-css-polish".into(),
            name: "Baseline Micro-Interaction & Entrance Polish (CSS only)".into(),
            category: MotionCategory::InteractiveEffect,
            engine: "css".into(),
            min_motion_intensity: 1,
            max_motion_intensity: 6,
            description: "The motion floor every page ships, even calm or 'static' sites, dashboards and stores: one hero entrance, hover/press feedback on cards and buttons, an animated link underline, instant focus rings, and a CSS scroll-driven fade for section content where supported. Zero JS, content visible by default, never hides text.".into(),
            performance_cost: "none (CSS only, compositor-friendly transform/opacity)".into(),
            reduced_motion_strategy: "All rules sit inside `@media (prefers-reduced-motion: no-preference)`; with reduce, states change instantly and nothing moves.".into(),
            cleanup_contract: "No listeners, timers or GPU objects to clean up.".into(),
            code_snippet: r##"@media (prefers-reduced-motion: no-preference) {
  /* 1. One calm hero entrance: content is visible at rest; the animation only plays once. */
  .hero-copy > * { animation: rise .7s cubic-bezier(.22,1,.36,1) both; }
  .hero-copy > :nth-child(2) { animation-delay: .08s; }
  .hero-copy > :nth-child(3) { animation-delay: .16s; }
  @keyframes rise { from { opacity: 0; transform: translateY(14px); } }

  /* 2. Hover / press feedback on interactive surfaces (pointer devices only). */
  @media (hover: hover) {
    .card { transition: transform .25s ease, box-shadow .25s ease; }
    .card:hover { transform: translateY(-3px); box-shadow: 0 10px 24px -12px rgb(0 0 0 / .25); }
    .card img, .card svg.art { transition: transform .5s cubic-bezier(.22,1,.36,1); }
    .card:hover img, .card:hover svg.art { transform: scale(1.03); }
  }
  .btn { transition: background-color .2s ease, color .2s ease, transform .12s ease; }
  .btn:active { transform: scale(.97); }

  /* 3. Link underline that draws in. */
  .link { background: linear-gradient(currentColor 0 0) 0 100% / 0 1px no-repeat; transition: background-size .3s ease; }
  .link:hover, .link:focus-visible { background-size: 100% 1px; }

  /* 4. Focus rings are never animated: they must appear instantly. */

  /* 5. Section content fades up as it scrolls in, only where scroll-driven animation exists.
        Without support (or with reduced motion) content is simply visible. */
  @supports (animation-timeline: view()) {
    .reveal { animation: rise linear both; animation-timeline: view(); animation-range: entry 0% entry 40%; }
  }
}"##.into(),
            css_fallback: r#".hero-copy > *, .reveal { opacity: 1; transform: none; }"#.into(),
        },
        AnimationRecipe {
            id: "interactive-magnetic-control".into(),
            name: "Accessible Magnetic CTA & Spotlight Surface".into(),
            category: MotionCategory::InteractiveEffect,
            engine: "motion-or-vanilla".into(),
            min_motion_intensity: 5,
            max_motion_intensity: 10,
            description: "Cursor-reactive magnetic pull on fine pointers (`(pointer: fine)`) while preserving crisp `:focus-visible` keyboard rings and touch-screen stability.".into(),
            performance_cost: "low (requestAnimationFrame throttled)".into(),
            reduced_motion_strategy: "Disabled automatically on touch devices `(pointer: coarse)` or when `prefers-reduced-motion: reduce` is enabled.".into(),
            cleanup_contract: "Removes `pointermove` and `pointerleave` listeners and resets transform on cleanup.".into(),
            code_snippet: r#"export function attachMagneticControl(buttonEl, strength = 0.24) {
  if (!buttonEl || typeof window === "undefined") return () => {};
  const canAnimate =
    window.matchMedia("(pointer: fine)").matches &&
    !window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  if (!canAnimate) return () => {};

  let rafId = 0;
  const onMove = (e) => {
    const rect = buttonEl.getBoundingClientRect();
    const dx = (e.clientX - (rect.left + rect.width / 2)) * strength;
    const dy = (e.clientY - (rect.top + rect.height / 2)) * strength;
    cancelAnimationFrame(rafId);
    rafId = requestAnimationFrame(() => {
      buttonEl.style.transform = `translate3d(${dx.toFixed(1)}px, ${dy.toFixed(1)}px, 0)`;
    });
  };
  const onLeave = () => {
    cancelAnimationFrame(rafId);
    buttonEl.style.transform = "translate3d(0, 0, 0)";
  };

  buttonEl.addEventListener("pointermove", onMove, { passive: true });
  buttonEl.addEventListener("pointerleave", onLeave, { passive: true });
  return () => {
    cancelAnimationFrame(rafId);
    buttonEl.removeEventListener("pointermove", onMove);
    buttonEl.removeEventListener("pointerleave", onLeave);
    buttonEl.style.transform = "";
  };
}"#.into(),
            css_fallback: r#".magnetic-btn {
  transition: transform 220ms cubic-bezier(0.22, 1, 0.36, 1);
}
.magnetic-btn:focus-visible {
  outline: 2px solid var(--color-accent, #2563EB);
  outline-offset: 3px;
}"#.into(),
        },
        AnimationRecipe {
            id: "page-transition-view-continuity".into(),
            name: "Router-Aware View Transition & Image Continuity".into(),
            category: MotionCategory::PageTransition,
            engine: "css-waapi".into(),
            min_motion_intensity: 3,
            max_motion_intensity: 9,
            description: "Coordinates route changes using the native View Transitions API (`document.startViewTransition`) with graceful instant fallback and browser history preservation.".into(),
            performance_cost: "low".into(),
            reduced_motion_strategy: "Skips `startViewTransition` when `prefers-reduced-motion: reduce` is active, executing the route update immediately.".into(),
            cleanup_contract: "Preserves native browser back/forward history and focus restoration.".into(),
            code_snippet: r#"export function navigateWithContinuity(updateDomCallback) {
  if (
    typeof document === "undefined" ||
    !document.startViewTransition ||
    window.matchMedia("(prefers-reduced-motion: reduce)").matches
  ) {
    updateDomCallback();
    return;
  }
  document.startViewTransition(() => {
    updateDomCallback();
  });
}"#.into(),
            css_fallback: r#"@media (prefers-reduced-motion: reduce) {
  ::view-transition-group(*),
  ::view-transition-old(*),
  ::view-transition-new(*) {
    animation: none !important;
  }
}"#.into(),
        },
        AnimationRecipe {
            id: "threejs-interactive-stage".into(),
            name: "Three.js / WebGL Responsive Artifact Stage with Full GPU Disposal".into(),
            category: MotionCategory::ThreeExperience,
            engine: "threejs".into(),
            min_motion_intensity: 7,
            max_motion_intensity: 10,
            description: "Renders an interactive 3D geometric artifact with physically based lighting, pointer parallax, IntersectionObserver offscreen pause, full GPU memory disposal, and static fallback.".into(),
            performance_cost: "high (justified only for flagship 3D/spatial experiences)".into(),
            reduced_motion_strategy: "Renders a single static frame (`renderer.render(scene, camera)`) instead of running a continuous animation loop when `prefers-reduced-motion: reduce` is set.".into(),
            cleanup_contract: "Cancels animation frame, disconnects ResizeObserver and IntersectionObserver, disposes geometry, material, and WebGLRenderer context.".into(),
            code_snippet: r#"import * as THREE from "three";

export function mountThreeArtifactStage(canvasEl) {
  if (!canvasEl || typeof window === "undefined") return () => {};
  const motionQuery = window.matchMedia("(prefers-reduced-motion: reduce)");
  const lifecycle = new AbortController();

  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(42, 1, 0.1, 100);
  camera.position.set(0, 0, 4.2);

  const renderer = new THREE.WebGLRenderer({
    canvas: canvasEl,
    antialias: true,
    alpha: true,
    powerPreference: "high-performance"
  });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));

  const geometry = new THREE.IcosahedronGeometry(1.15, 2);
  const material = new THREE.MeshPhysicalMaterial({
    color: 0x38bdf8,
    roughness: 0.18,
    metalness: 0.65,
    wireframe: false
  });
  const mesh = new THREE.Mesh(geometry, material);
  scene.add(mesh);

  const keyLight = new THREE.DirectionalLight(0xffffff, 2.2);
  keyLight.position.set(3, 4, 5);
  const rimLight = new THREE.PointLight(0xf43f5e, 1.8, 12);
  rimLight.position.set(-3, -2, 3);
  scene.add(keyLight, rimLight, new THREE.AmbientLight(0xffffff, 0.45));

  const resize = () => {
    const { clientWidth, clientHeight } = canvasEl.parentElement || canvasEl;
    const w = Math.max(clientWidth, 240);
    const h = Math.max(clientHeight, 240);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    renderer.setSize(w, h, false);
    renderer.render(scene, camera);
  };
  resize();

  let rafId = 0;
  let isVisible = true;
  const tick = () => {
    if (!motionQuery.matches && isVisible && !document.hidden) {
      mesh.rotation.y += 0.0045;
      mesh.rotation.x += 0.002;
      renderer.render(scene, camera);
    }
    if (!motionQuery.matches && isVisible && !document.hidden) {
      rafId = requestAnimationFrame(tick);
    }
  };
  const sync = () => {
    cancelAnimationFrame(rafId);
    rafId = 0;
    renderer.render(scene, camera);
    if (!motionQuery.matches && isVisible && !document.hidden) rafId = requestAnimationFrame(tick);
  };
  motionQuery.addEventListener("change", sync, { signal: lifecycle.signal });
  document.addEventListener("visibilitychange", sync, { signal: lifecycle.signal });
  sync();

  const ro = new ResizeObserver(resize);
  ro.observe(canvasEl.parentElement || canvasEl);
  const io = new IntersectionObserver(([entry]) => {
    isVisible = Boolean(entry && entry.isIntersecting);
    sync();
  });
  io.observe(canvasEl);

  return () => {
    cancelAnimationFrame(rafId);
    lifecycle.abort();
    ro.disconnect();
    io.disconnect();
    geometry.dispose();
    material.dispose();
    renderer.dispose();
  };
}"#.into(),
            css_fallback: r#".three-stage-container {
  position: relative;
  min-height: 360px;
}
.three-stage-fallback {
  pointer-events: none;
}"#.into(),
        },
        AnimationRecipe {
            id: "scroll-lenis-conditional".into(),
            name: "Conditional Lenis Smooth Scroll Synchronized with GSAP ScrollTrigger".into(),
            category: MotionCategory::ScrollAnimation,
            engine: "lenis".into(),
            min_motion_intensity: 8,
            max_motion_intensity: 10,
            description: "Opt-in smooth scrolling ONLY for high-motion narrative experiences (`motion_intensity >= 8`). Never hijacks native scroll on standard content sites, dashboards, or reduced-motion sessions.".into(),
            performance_cost: "medium".into(),
            reduced_motion_strategy: "Returns a no-op immediately if `(prefers-reduced-motion: reduce)` or touch pointer `(pointer: coarse)` is detected.".into(),
            cleanup_contract: "Calls `lenis.destroy()` and removes the GSAP ticker callback on unmount.".into(),
            code_snippet: r#"import Lenis from "lenis";
import gsap from "gsap";
import { ScrollTrigger } from "gsap/ScrollTrigger";

export function initConditionalLenis({ motionIntensity = 5 } = {}) {
  if (typeof window === "undefined") return () => {};
  if (
    motionIntensity < 8 ||
    window.matchMedia("(prefers-reduced-motion: reduce)").matches ||
    window.matchMedia("(pointer: coarse)").matches
  ) {
    return () => {};
  }
  const lenis = new Lenis({ duration: 1.05, smoothWheel: true });
  lenis.on("scroll", ScrollTrigger.update);
  const onTick = (time) => lenis.raf(time * 1000);
  gsap.ticker.add(onTick);
  gsap.ticker.lagSmoothing(0);

  return () => {
    gsap.ticker.remove(onTick);
    lenis.destroy();
  };
}"#.into(),
            css_fallback: r#"html.lenis, html.lenis body { height: auto; }"#.into(),
        },
    ];
    // Native/CSS alternatives remain addressable for explicit user requests.
    // Default category selection uses the shared GSAP lifecycle for every tier.
    for (id, name, category) in [
        ("section-gsap-authored", "GSAP content-shaped section choreography", MotionCategory::SectionAnimation),
        ("interactive-gsap-feedback", "GSAP interruptible interaction feedback", MotionCategory::InteractiveEffect),
        ("page-gsap-transition", "GSAP navigation transition with immediate semantic update", MotionCategory::PageTransition),
        ("threejs-gsap-stage", "GSAP-driven subject scene with visible static resolution", MotionCategory::ThreeExperience),
    ] {
        recipes.push(AnimationRecipe {
            id: id.into(), name: name.into(), category, engine: "gsap".into(),
            min_motion_intensity: 1, max_motion_intensity: 10,
            description: "Use the injected pinned GSAP runtime for authored timelines, interactions and object-state changes. compose() authors the subject-specific transformation; render() may draw Three.js through the scoped GSAP ticker. Keep essential source content visible. Do not substitute an anonymous rotating primitive or identical fade-up for a subject idea.".into(),
            performance_cost: "Depends on the authored DOM or scene; no speed score is implied.".into(),
            reduced_motion_strategy: "Dynamic preference changes revert the scoped timelines, remove the render ticker and call resolve() for a complete static composition. enabled:false creates no timelines or observers.".into(),
            cleanup_contract: "Call destroy() on teardown; it reverts GSAP context/ScrollTrigger, removes scoped listeners and ticker, and disconnects the observer. Dispose Three.js geometry/materials/textures separately.".into(),
            code_snippet: include_str!("../../../skills/layoza-animations/assets/gsap-motion.mjs").into(),
            css_fallback: "Essential text, artwork, controls and the resolved poster are visible in source CSS; no opacity:0 startup dependency.".into(),
        });
    }
    recipes.push(AnimationRecipe {
        id: "react-motion-presence".into(), name: "Explicitly requested Motion for React presence".into(),
        category: MotionCategory::InteractiveEffect, engine: "motion/react".into(),
        min_motion_intensity: 1, max_motion_intensity: 10,
        description: "Opt-in when the user requests Framer Motion / Motion for React. Let it own component presence/layout; GSAP owns separate choreography and scene state. Never animate the same element/property from both engines.".into(),
        performance_cost: "React component transitions; inspect the actual device behavior.".into(),
        reduced_motion_strategy: "useReducedMotion and the page pause state remove travel and set duration:0.".into(),
        cleanup_contract: "AnimatePresence owns component presence; React unmount removes it. Keep focus/aria state changes immediate.".into(),
        code_snippet: "import { AnimatePresence, motion, useReducedMotion } from 'motion/react';\nexport function PresencePanel({ show, children, paused = false }) {\n  const still = useReducedMotion() || paused;\n  return <AnimatePresence initial={false}>{show && <motion.div key=\"panel\" initial={still ? false : { opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} exit={still ? { opacity: 0 } : { opacity: 0, y: -4 }} transition={{ duration: still ? 0 : .2 }}>{children}</motion.div>}</AnimatePresence>;\n}".into(),
        css_fallback: "Source content stays available; use native/SSR markup and stable controls outside the animated presence region.".into(),
    });
    recipes
}

/// Recommends context-appropriate animation recipes based on target category and `motion_intensity` (0–10).
pub fn recommend_animations(
    category_filter: Option<&str>,
    motion_intensity: u8,
) -> Vec<AnimationRecipe> {
    let clamped = motion_intensity.min(10);
    if clamped == 0 {
        return Vec::new();
    }
    catalog_animation_recipes()
        .into_iter()
        .filter(|r| {
            let intensity_ok = clamped >= r.min_motion_intensity && clamped <= r.max_motion_intensity;
            intensity_ok && default_engine_matches(r, category_filter) && category_matches(r, category_filter)
        })
        .collect()
}

impl MotionCategory {
    /// Every category, in catalog order.
    pub const ALL: [MotionCategory; 6] = [
        MotionCategory::HeroAnimation,
        MotionCategory::SectionAnimation,
        MotionCategory::ScrollAnimation,
        MotionCategory::InteractiveEffect,
        MotionCategory::PageTransition,
        MotionCategory::ThreeExperience,
    ];

    /// The serialized (snake_case) name, e.g. `hero_animation`.
    pub fn as_str(&self) -> &'static str {
        match self {
            MotionCategory::HeroAnimation => "hero_animation",
            MotionCategory::SectionAnimation => "section_animation",
            MotionCategory::ScrollAnimation => "scroll_animation",
            MotionCategory::InteractiveEffect => "interactive_effect",
            MotionCategory::PageTransition => "page_transition",
            MotionCategory::ThreeExperience => "three_experience",
        }
    }
}

fn squash(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_lowercase()
}

/// A filter matches a recipe's category (`hero_animation`, `hero`, `HeroAnimation`), id, or engine.
fn category_matches(r: &AnimationRecipe, filter: Option<&str>) -> bool {
    match filter.map(str::trim) {
        None | Some("") | Some("all") => true,
        Some(c) => {
            let c_low = c.to_ascii_lowercase();
            let c_sq = squash(c);
            if ["framermotion", "motionreact"].contains(&c_sq.as_str()) { return r.engine == "motion/react"; }
            r.id.contains(&c_low) || r.engine.contains(&c_low) || (!c_sq.is_empty() && squash(r.category.as_str()).contains(&c_sq))
        }
    }
}

fn default_engine_matches(r: &AnimationRecipe, filter: Option<&str>) -> bool {
    // A named engine/id is an explicit alternative lookup. Broad categories
    // never silently recommend another animation runtime.
    let explicit = filter.is_some_and(|f| r.id.eq_ignore_ascii_case(f) || r.engine.eq_ignore_ascii_case(f)
        || (["framermotion", "motionreact"].contains(&squash(f).as_str()) && r.engine == "motion/react"));
    explicit || r.engine.contains("gsap")
}

/// Agents often pass the *site type* ("ecommerce", "dashboard", "restaurant") as the category.
/// Map those to the motion categories that suit them instead of failing.
fn site_type_categories(filter: &str) -> Option<&'static [MotionCategory]> {
    use MotionCategory::*;
    const PRODUCT_UI: &[MotionCategory] = &[SectionAnimation, InteractiveEffect];
    const COMMERCE: &[MotionCategory] = &[HeroAnimation, SectionAnimation, InteractiveEffect];
    const BRAND: &[MotionCategory] = &[HeroAnimation, SectionAnimation, ScrollAnimation, InteractiveEffect];
    let f = squash(filter);
    let has = |words: &[&str]| words.iter().any(|w| f.contains(w));
    // Commerce first so "apparel shop" is not caught by "app".
    if has(&["ecommerce", "commerce", "store", "shop", "retail", "cart", "dtc", "apparel"]) {
        Some(COMMERCE)
    } else if has(&["dashboard", "admin", "saas", "app", "tool", "portal", "analytics", "crm"]) {
        Some(PRODUCT_UI)
    } else if has(&[
        "restaurant", "cafe", "bar", "hotel", "hospitality", "local", "food", "bakery", "brand", "marketing",
        "landing", "agency", "studio", "portfolio", "editorial", "blog", "corporate", "nonprofit", "event",
    ]) {
        Some(BRAND)
    } else {
        None
    }
}

fn valid_choices() -> String {
    let ids: Vec<String> = catalog_animation_recipes().into_iter().map(|r| r.id).collect();
    let cats: Vec<&str> = MotionCategory::ALL.iter().map(|c| c.as_str()).collect();
    format!("valid ids: {}; valid categories: {}", ids.join(", "), cats.join(", "))
}

/// Like [`recommend_animations`], plus selection of one recipe by exact `recipe_id`, and an error
/// (listing the valid ids and categories) instead of an empty list when nothing matches.
/// A recipe chosen by id is returned whatever the intensity; intensity 0 (motion off) returns `Ok([])`.
/// A valid category with no recipe in range returns its nearest-range recipes.
pub fn select_animations(
    category_filter: Option<&str>,
    recipe_id: Option<&str>,
    motion_intensity: u8,
) -> Result<Vec<AnimationRecipe>, String> {
    if motion_intensity == 0 {
        return Ok(Vec::new());
    }
    if let Some(id) = recipe_id.map(str::trim).filter(|s| !s.is_empty()) {
        return catalog_animation_recipes()
            .into_iter()
            .find(|r| r.id.eq_ignore_ascii_case(id))
            .map(|r| vec![r])
            .ok_or_else(|| format!("no recipe matched `{}`; {}", id, valid_choices()));
    }
    let clamped = motion_intensity.min(10);
    if clamped == 0 {
        return Ok(Vec::new());
    }
    let mut in_category: Vec<AnimationRecipe> = catalog_animation_recipes()
        .into_iter()
        .filter(|r| category_matches(r, category_filter) && default_engine_matches(r, category_filter))
        .collect();
    if in_category.is_empty() {
        if let Some(cats) = category_filter.and_then(site_type_categories) {
            in_category = catalog_animation_recipes().into_iter().filter(|r| cats.contains(&r.category) && r.engine.contains("gsap")).collect();
        }
    }
    if in_category.is_empty() {
        return Err(format!("no recipe matched `{}`; {}", category_filter.unwrap_or(""), valid_choices()));
    }
    let fitting: Vec<AnimationRecipe> = in_category
        .iter()
        .filter(|r| clamped >= r.min_motion_intensity && clamped <= r.max_motion_intensity)
        .cloned()
        .collect();
    if fitting.is_empty() {
        // A valid category whose recipes sit outside the requested intensity: return the
        // closest-range recipes rather than failing, so the caller can tune them up or down.
        let distance = |r: &AnimationRecipe| {
            if clamped < r.min_motion_intensity {
                r.min_motion_intensity - clamped
            } else {
                clamped.saturating_sub(r.max_motion_intensity)
            }
        };
        let nearest = in_category.iter().map(distance).min().unwrap_or(0);
        return Ok(in_category.into_iter().filter(|r| distance(r) == nearest).collect());
    }
    Ok(fitting)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_are_gsap_and_explicit_other_engines_and_zero_still_work() {
        for category in MotionCategory::ALL {
            let recipes = select_animations(Some(category.as_str()), None, 5).unwrap();
            assert!(!recipes.is_empty());
            assert!(recipes.iter().all(|r| r.engine.contains("gsap")), "{}", category.as_str());
        }
        assert!(select_animations(Some("corporate"), None, 3).unwrap().iter().all(|r| r.engine.contains("gsap")));
        assert_eq!(select_animations(Some("framer-motion"), None, 5).unwrap()[0].engine, "motion/react");
        assert_eq!(select_animations(None, Some("micro-css-polish"), 5).unwrap()[0].engine, "css");
        assert!(select_animations(Some("framer-motion"), None, 0).unwrap().is_empty());
        assert_eq!(crate::craft::motion_delivery(0).engine, "none");
        assert_eq!(crate::craft::motion_delivery(3).engine, "gsap");
    }

    #[test]
    fn select_by_recipe_id_category_and_reports_misses() {
        let one = select_animations(None, Some("threejs-interactive-stage"), 3).unwrap();
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].id, "threejs-interactive-stage");
        assert!(select_animations(Some("hero_animation"), None, 6).unwrap().iter().all(|r| r.category == MotionCategory::HeroAnimation));
        assert!(!select_animations(Some("hero"), None, 6).unwrap().is_empty());
        let miss = select_animations(Some("nonsense"), None, 6).unwrap_err();
        assert!(miss.starts_with("no recipe matched") && miss.contains("hero_animation") && miss.contains("scroll-lenis-conditional"), "{}", miss);
        assert!(select_animations(None, Some("nope"), 6).unwrap_err().contains("valid ids"));
        assert!(select_animations(None, None, 0).unwrap().is_empty());
        assert!(select_animations(None, Some("scroll-composition-to-work"), 0).unwrap().is_empty());
        assert!(select_animations(Some("scroll"), None, 8).unwrap().iter().any(|r| r.id == "scroll-composition-to-work"));
        assert!(!select_animations(None, None, 3).unwrap().iter().any(|r| r.id == "scroll-composition-to-work"));
        // Site types map to fitting motion categories rather than erroring.
        let dash = select_animations(Some("dashboard"), None, 3).unwrap();
        assert!(!dash.is_empty() && dash.iter().all(|r| r.category != MotionCategory::ThreeExperience && r.category != MotionCategory::ScrollAnimation));
        assert!(!select_animations(Some("ecommerce"), None, 4).unwrap().is_empty());
        assert!(!select_animations(Some("seafood restaurant"), None, 5).unwrap().is_empty());
        // Valid category, intensity outside every range: nearest-range recipes, not an error.
        let near = select_animations(Some("section_animation"), None, 10).unwrap();
        assert!(!near.is_empty() && near.iter().all(|r| r.category == MotionCategory::SectionAnimation));
    }
}
