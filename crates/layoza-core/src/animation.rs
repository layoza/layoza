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
    vec![
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
  const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

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
    if (!reducedMotion && isVisible) {
      mesh.rotation.y += 0.0045;
      mesh.rotation.x += 0.002;
      renderer.render(scene, camera);
    }
    if (!reducedMotion) {
      rafId = requestAnimationFrame(tick);
    }
  };
  if (!reducedMotion) {
    rafId = requestAnimationFrame(tick);
  }

  const ro = new ResizeObserver(resize);
  ro.observe(canvasEl.parentElement || canvasEl);
  const io = new IntersectionObserver(([entry]) => {
    isVisible = Boolean(entry && entry.isIntersecting);
  });
  io.observe(canvasEl);

  return () => {
    cancelAnimationFrame(rafId);
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
    ]
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
            intensity_ok && category_matches(r, category_filter)
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
            r.id.contains(&c_low) || r.engine.contains(&c_low) || (!c_sq.is_empty() && squash(r.category.as_str()).contains(&c_sq))
        }
    }
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
        .filter(|r| category_matches(r, category_filter))
        .collect();
    if in_category.is_empty() {
        if let Some(cats) = category_filter.and_then(site_type_categories) {
            in_category = catalog_animation_recipes().into_iter().filter(|r| cats.contains(&r.category)).collect();
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
