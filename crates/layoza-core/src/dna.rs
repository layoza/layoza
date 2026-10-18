// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Site DNA Data Structures & Artifact Generators (DESIGN.md, BRAND.md, MOTION.md,
// SITE-DNA.json, COMPONENT-INVENTORY.json, DESIGN-DECISIONS.md).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Distinguishes between hard observed facts from source/DOM, inferred design rules,
/// and explicitly user-approved design decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeTier {
    ObservedFact,
    InferredRule,
    UserApprovedDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionEntry {
    pub id: String,
    pub category: String,
    pub tier: KnowledgeTier,
    pub statement: String,
    pub evidence_source: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorToken {
    pub name: String,
    pub value: String,
    pub role: String,
    pub tier: KnowledgeTier,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyToken {
    pub role: String,
    pub font_family: String,
    pub font_size: String,
    pub font_weight: String,
    pub line_height: String,
    pub letter_spacing: String,
    pub tier: KnowledgeTier,
    /// Where the font was found (file + evidence), or why it was inferred.
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentEntry {
    pub name: String,
    pub category: String,
    pub file_path: String,
    pub variants: Vec<String>,
    pub props_or_classes: Vec<String>,
    pub accessible_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInventory {
    pub project_name: String,
    pub framework: String,
    pub total_components: usize,
    pub components: Vec<ComponentEntry>,
    pub layout_primitives: Vec<String>,
    pub navigation_elements: Vec<String>,
    pub footer_elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionProfile {
    pub libraries_detected: Vec<String>,
    pub keyframes_detected: Vec<String>,
    pub transition_durations: Vec<String>,
    pub easing_curves: Vec<String>,
    pub scroll_behavior: String,
    pub honors_reduced_motion: bool,
    pub recommended_motion_intensity: u8,
    pub tier: KnowledgeTier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscrepancyRecord {
    pub property: String,
    pub source_value: String,
    pub rendered_value: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionConfidence {
    pub overall_score: f64,
    pub source_inspected: bool,
    pub browser_inspected: bool,
    pub screenshot_only: bool,
    pub unknowns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteDna {
    pub schema_version: String,
    pub project_name: String,
    pub preservation_mode: bool,
    pub framework: String,
    pub routes: Vec<String>,
    pub colors: Vec<ColorToken>,
    pub typography: Vec<TypographyToken>,
    pub spacing_scale: BTreeMap<String, String>,
    pub radii_scale: BTreeMap<String, String>,
    pub shadows: BTreeMap<String, String>,
    pub breakpoints: BTreeMap<String, String>,
    pub content_max_widths: Vec<String>,
    pub grid_systems: Vec<String>,
    pub dark_mode_strategy: String,
    pub accessibility_patterns: Vec<String>,
    pub motion: MotionProfile,
    pub dials: DesignDials,
    pub discrepancies: Vec<DiscrepancyRecord>,
    pub confidence: ExtractionConfidence,
    pub decisions: Vec<DecisionEntry>,
    pub baseline_fingerprint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesignDials {
    /// 0 to 10: Layout experimentation (0 = strict grid/symmetric, 10 = highly asymmetric/avant-garde)
    pub design_variance: u8,
    /// 0 to 10: Motion depth (0 = static/no motion, 5 = purposeful transitions/reveals, 10 = choreographed WebGL/GSAP timelines)
    pub motion_intensity: u8,
    /// 0 to 10: Information density per viewport (0 = gallery whitespace, 5 = balanced marketing, 10 = high-density pro terminal/dashboard)
    pub visual_density: u8,
}

impl DesignDials {
    pub fn clamped(design_variance: u8, motion_intensity: u8, visual_density: u8) -> Self {
        Self {
            design_variance: design_variance.min(10),
            motion_intensity: motion_intensity.min(10),
            visual_density: visual_density.min(10),
        }
    }
}

impl Default for DesignDials {
    fn default() -> Self {
        Self {
            design_variance: 4,
            motion_intensity: 5,
            visual_density: 5,
        }
    }
}

impl SiteDna {
    /// Generates `BRAND.md` summarizing brand identity, voice, audience, and visual anchors.
    pub fn to_brand_md(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# BRAND.md — {}\n\n", self.project_name));
        out.push_str("## 1. Brand Preservation & Operating Contract\n\n");
        out.push_str(&format!(
            "- **Default Mode**: `{}`\n",
            if self.preservation_mode {
                "Preservation Mode (Extend/Improve without breaking existing visual identity)"
            } else {
                "Greenfield / Authorized Redesign"
            }
        ));
        out.push_str(&format!("- **Detected Framework**: `{}`\n", self.framework));
        out.push_str(&format!(
            "- **Dark Mode Strategy**: `{}`\n",
            self.dark_mode_strategy
        ));
        out.push_str(&format!(
            "- **Extraction Confidence**: `{:.0}%` (Source inspected: `{}`, Browser inspected: `{}`)\n\n",
            self.confidence.overall_score * 100.0,
            self.confidence.source_inspected,
            self.confidence.browser_inspected
        ));

        out.push_str("## 2. Brand Color Architecture\n\n");
        out.push_str("| Token | Value | Role | Provenance Tier | Source |\n");
        out.push_str("| :--- | :--- | :--- | :--- | :--- |\n");
        for c in &self.colors {
            out.push_str(&format!(
                "| `{}` | `{}` | {} | `{:?}` | `{}` |\n",
                c.name, c.value, c.role, c.tier, c.source
            ));
        }

        out.push_str("\n## 3. Typography & Voice\n\n");
        out.push_str("| Role | Font Family | Size | Weight | Line Height | Letter Spacing | Provenance | Source |\n");
        out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");
        for t in &self.typography {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{:?}` | {} |\n",
                t.role,
                t.font_family,
                t.font_size,
                t.font_weight,
                t.line_height,
                t.letter_spacing,
                t.tier,
                t.source.replace('|', "/")
            ));
        }

        out.push_str("\n## 4. Calibrated Design Dials (0–10)\n\n");
        out.push_str(&format!(
            "- **Design Variance**: `{}/10`\n- **Motion Intensity**: `{}/10`\n- **Visual Density**: `{}/10`\n",
            self.dials.design_variance, self.dials.motion_intensity, self.dials.visual_density
        ));

        if !self.confidence.unknowns.is_empty() {
            out.push_str("\n## 5. Recorded Unknowns & Verification Boundaries\n\n");
            for u in &self.confidence.unknowns {
                out.push_str(&format!("- {}\n", u));
            }
        }
        out
    }

    /// Generates `MOTION.md` documenting observed and recommended choreography, easing, and cleanup rules.
    pub fn to_motion_md(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# MOTION.md — {}\n\n", self.project_name));
        out.push_str("## 1. Motion Identity Summary\n\n");
        out.push_str(&format!(
            "- **Detected Libraries**: {}\n",
            if self.motion.libraries_detected.is_empty() {
                "CSS Transitions / Native Web Animations API".to_string()
            } else {
                self.motion.libraries_detected.join(", ")
            }
        ));
        out.push_str(&format!(
            "- **Motion Intensity Dial**: `{}/10`\n",
            self.dials.motion_intensity
        ));
        out.push_str(&format!(
            "- **Scroll Strategy**: `{}` (Native scroll preserved unless explicitly opted in)\n",
            self.motion.scroll_behavior
        ));
        out.push_str(&format!(
            "- **Honors `prefers-reduced-motion` in Source**: `{}`\n\n",
            self.motion.honors_reduced_motion
        ));

        out.push_str("## 2. Timing & Easing Tokens\n\n");
        out.push_str("- **Durations**: ");
        if self.motion.transition_durations.is_empty() {
            out.push_str("`180ms` (micro), `320ms` (surface), `600ms` (entrance)\n");
        } else {
            out.push_str(&format!("{}\n", self.motion.transition_durations.join(", ")));
        }
        out.push_str("- **Easing Curves**: ");
        if self.motion.easing_curves.is_empty() {
            out.push_str("`cubic-bezier(0.22, 1, 0.36, 1)` (expo-out), `cubic-bezier(0.16, 1, 0.3, 1)`\n");
        } else {
            out.push_str(&format!("{}\n", self.motion.easing_curves.join(", ")));
        }

        if !self.motion.keyframes_detected.is_empty() {
            out.push_str(&format!(
                "- **Detected CSS Keyframes**: `{}`\n",
                self.motion.keyframes_detected.join("`, `")
            ));
        }

        out.push_str("\n## 3. Mandatory Motion Quality & Safety Rules\n\n");
        out.push_str("1. **Reduced Motion**: Every CSS animation and JS timeline (`gsap.matchMedia()`, `useReducedMotion()`) MUST provide an instant, readable static fallback when `(prefers-reduced-motion: reduce)` is active.\n");
        out.push_str("2. **Zero Content Hiding**: Never leave essential text or CTAs at `opacity: 0` or `visibility: hidden` if JavaScript fails or before ScrollTrigger initializes.\n");
        out.push_str("3. **Lifecycle Cleanup**: Clean up all `gsap.context()`, `ScrollTrigger`, `IntersectionObserver`, `requestAnimationFrame`, and Three.js `geometry`/`material`/`renderer` instances on unmount or route transition.\n");
        out.push_str("4. **No Unsolicited Scroll Hijacking**: Do not introduce Lenis or custom wheel interceptors unless `motion_intensity >= 7` and smooth narrative scrolling is explicitly appropriate.\n");
        out
    }

    /// Generates `DESIGN-DECISIONS.md` separating Observed Facts, Inferred Rules, and User-Approved Decisions.
    pub fn to_design_decisions_md(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# DESIGN-DECISIONS.md — {}\n\n", self.project_name));
        out.push_str("This ledger maintains a strict epistemic separation between **Observed Facts** (measured directly from source files or browser DOM), **Inferred Rules** (derived by Layoza's Design Intelligence Engine), and **User-Approved Decisions** (explicitly confirmed by the user).\n\n");

        out.push_str("## 1. Observed Facts (`ObservedFact`)\n\n");
        out.push_str("| ID | Category | Statement | Evidence Source | Confidence |\n");
        out.push_str("| :--- | :--- | :--- | :--- | :--- |\n");
        for d in self
            .decisions
            .iter()
            .filter(|d| d.tier == KnowledgeTier::ObservedFact)
        {
            out.push_str(&format!(
                "| `{}` | {} | {} | `{}` | `{:.0}%` |\n",
                d.id,
                d.category,
                d.statement,
                d.evidence_source,
                d.confidence * 100.0
            ));
        }

        out.push_str("\n## 2. Inferred Design Rules (`InferredRule`)\n\n");
        out.push_str("| ID | Category | Statement | Basis | Confidence |\n");
        out.push_str("| :--- | :--- | :--- | :--- | :--- |\n");
        for d in self
            .decisions
            .iter()
            .filter(|d| d.tier == KnowledgeTier::InferredRule)
        {
            out.push_str(&format!(
                "| `{}` | {} | {} | `{}` | `{:.0}%` |\n",
                d.id,
                d.category,
                d.statement,
                d.evidence_source,
                d.confidence * 100.0
            ));
        }

        out.push_str("\n## 3. User-Approved Decisions (`UserApprovedDecision`)\n\n");
        let approved: Vec<_> = self
            .decisions
            .iter()
            .filter(|d| d.tier == KnowledgeTier::UserApprovedDecision)
            .collect();
        if approved.is_empty() {
            out.push_str("_No user overrides recorded yet. Existing site preservation rules apply by default._\n");
        } else {
            out.push_str("| ID | Category | Statement | Approval Source |\n");
            out.push_str("| :--- | :--- | :--- | :--- |\n");
            for d in approved {
                out.push_str(&format!(
                    "| `{}` | {} | {} | `{}` |\n",
                    d.id, d.category, d.statement, d.evidence_source
                ));
            }
        }

        if !self.discrepancies.is_empty() {
            out.push_str("\n## 4. Source vs. Rendered Browser Discrepancies\n\n");
            out.push_str("| Property | Source Value | Rendered DOM Value | Explanation |\n");
            out.push_str("| :--- | :--- | :--- | :--- |\n");
            for disc in &self.discrepancies {
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` | {} |\n",
                    disc.property, disc.source_value, disc.rendered_value, disc.explanation
                ));
            }
        }

        out
    }
}
