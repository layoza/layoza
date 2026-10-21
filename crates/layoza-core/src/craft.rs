// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
//! The craft floor: the finish every page must reach before its direction is judged. A direction
//! decides what a page is; the floor decides whether it looks professionally built. Each rule is a
//! number a reviewer (or `layoza conform`) can check on the render, never an adjective. The numbers
//! are tuned per register (a cinematic launch carries larger display type and slower motion than a
//! developer tool), and `tuning` is the single source the render checks read, so the words the
//! builder is dealt and the thresholds the page is measured against never drift apart.

use crate::register::brief_registers;

/// Small execution kit, shared by the brief and JSON MCP responses. These are starting
/// tokens, not a layout template or a substitute for the incumbent site's tokens.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecutionFoundation {
    pub authority: String,
    pub spacing_px: Vec<u16>,
    pub layout_css: String,
    pub type_fit: String,
    pub actions: String,
    pub motion: MotionDelivery,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MotionDelivery {
    pub dial: u8,
    pub engine: String,
    pub default_policy: String,
    pub runtime_asset: String,
    pub section_runtime_asset: String,
    pub section_authoring: String,
    pub tier: String,
    pub deliverable: String,
    pub scene_spec: Vec<String>,
    pub evidence: String,
}

pub fn motion_delivery(dial: u8) -> MotionDelivery {
    let dial = dial.min(10);
    let (tier, deliverable) = match dial {
        0 => ("none", "Static composition and immediate state feedback. No entrance, ambient, scrub or hover travel."),
        1..=3 => ("quiet", "Clear control feedback and restrained transitions. A calm brief does not require section reveals."),
        4..=6 => ("expressive", "A coordinated opening and subject-specific interaction or scroll scene, where the content supports it. Avoid unrelated effects."),
        _ => ("high", "A visibly transformative signature scene, coordinated type/art opening and connected section handoffs. Describe what changes shape, scale, composition or state; a progress line, floating cards and small translate/rotate tweens alone do not deliver a highly animated brief. Restage the scene for touch screens."),
    };
    MotionDelivery {
        dial,
        engine: if dial == 0 { "none" } else { "gsap" }.into(),
        default_policy: "Include purposeful animation by default. Use GSAP for authored DOM, SVG and Three.js state animation unless the user explicitly requests another engine or no animation. Dial 0 creates no decorative timelines or animation loops. Reduced-motion preferences resolve to complete stills; focus and semantic state updates remain immediate.".into(),
        runtime_asset: "skills/layoza-animations/assets/gsap-motion.mjs".into(),
        section_runtime_asset: "skills/layoza-animations/assets/section-motion.mjs".into(),
        section_authoring: "Use recommend_animation.motion_plan for a content-specific thesis, section transformations and phone staging. Rich motion needs connected chapter mechanisms, not repeated reveals. Verify normal playback with scripts/review-motion.mjs; plans alone are unverified.".into(),
        tier: tier.into(),
        deliverable: deliverable.into(),
        scene_spec: if dial == 0 { Vec::new() } else { vec![
            "Subject/object and the idea its transformation communicates".into(),
            "Trigger, start composition, transformation, resolved composition and handoff".into(),
            "Timing/easing, phone staging, pause/interrupt behavior, reduced-motion still and teardown".into(),
        ] },
        evidence: if dial >= 7 {
            "Inspect a recording on desktop and phone, including the actual opening and scroll transitions. Sample signature scenes at start/25/50/75/end. Judge the visible transformation, continuity and legibility; recipe count and a clean still screenshot cannot establish motion quality."
        } else {
            "Inspect the actual state changes; verify readable static/no-JS and reduced-motion states."
        }.into(),
    }
}

pub fn execution_foundation(dial: u8) -> ExecutionFoundation {
    ExecutionFoundation {
        authority: "Starter values for new work only. Explicit brief sizes and existing project tokens win. Optical composition still requires render review.".into(),
        spacing_px: vec![4, 8, 12, 16, 24, 32, 48, 64, 96, 128],
        layout_css: ":root{--gutter:clamp(20px,4vw,64px);--content:1320px;--section-space:clamp(64px,8vw,128px);--copy-gap:24px;--control-gap:12px;--control-height:52px} .container{width:min(calc(100% - 2 * var(--gutter)),var(--content));margin-inline:auto} .actions{display:flex;flex-wrap:wrap;align-items:center;gap:var(--control-gap)} .actions>a,.actions>button{min-height:var(--control-height);display:inline-flex;align-items:center;justify-content:center;gap:12px;padding:12px 24px;font:600 1rem/1.25 var(--font-body);text-align:center}".into(),
        type_fit: "Use each concept's font pair and size tokens, then fit the actual copy at 320/390/768/1440px. Set role-specific leading and measure; start display tracking near -0.03em, body at 0, and inspect counters/word gaps before tightening. Keep heading, supporting copy and action as one aligned group. Remove unearned italic, symbol and label voices rather than adding fonts. A large headline alone is not a hierarchy; examine its relationship to navigation, copy, art and actions at physical size. Recompose phone line breaks instead of shrinking the whole desktop.".into(),
        actions: "One clear primary action per decision group. Secondary actions share baseline and hit height, with visibly quieter emphasis. Keep label and icon together without arbitrary space-between gaps; allow the group to stack on narrow screens. Check real labels, focus/hover/pressed/disabled states, text zoom and open menus. Place art in a reserved grid/stage; its motion must not collide with copy or actions. Use smaller gaps within a content group than between groups; do not compensate for a weak composition with empty height.".into(),
        motion: motion_delivery(dial),
    }
}

/// Measurable thresholds for one register. `conformance` checks renders against these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CraftTuning {
    /// The register the numbers come from.
    pub register: &'static str,
    /// Desktop header height range in px (one row).
    pub header_px: (u32, u32),
    /// Most distinct font sizes (px, rounded) carrying text on a desktop render.
    pub max_type_sizes: u32,
    /// Largest h1 size on desktop, px.
    pub h1_max_px: u32,
    /// Body copy size range, px.
    pub body_px: (u32, u32),
    /// Longest prose line, in characters.
    pub measure_chars: u32,
    /// Primary button height range, px.
    pub button_px: (u32, u32),
    /// Desktop section padding range, px (each side of a section).
    pub section_pad_px: (u32, u32),
    /// Interface motion duration range, ms (the one signature moment may run longer).
    pub motion_ms: (u32, u32),
    /// Most text blocks inside the hero's first viewport.
    pub hero_blocks: u32,
}

const DEFAULT_REGISTER: &str = "corporate";

const TUNINGS: &[CraftTuning] = &[
    CraftTuning { register: "premium", header_px: (64, 88), max_type_sizes: 9, h1_max_px: 128, body_px: (16, 19), measure_chars: 80, button_px: (44, 60), section_pad_px: (112, 200), motion_ms: (240, 700), hero_blocks: 12 },
    CraftTuning { register: "cinematic", header_px: (60, 88), max_type_sizes: 9, h1_max_px: 152, body_px: (16, 19), measure_chars: 80, button_px: (44, 60), section_pad_px: (112, 220), motion_ms: (300, 900), hero_blocks: 12 },
    CraftTuning { register: "corporate", header_px: (60, 84), max_type_sizes: 10, h1_max_px: 104, body_px: (15, 18), measure_chars: 82, button_px: (44, 56), section_pad_px: (96, 168), motion_ms: (150, 320), hero_blocks: 16 },
    CraftTuning { register: "technical", header_px: (52, 72), max_type_sizes: 10, h1_max_px: 88, body_px: (14, 17), measure_chars: 82, button_px: (36, 52), section_pad_px: (80, 144), motion_ms: (120, 250), hero_blocks: 16 },
    CraftTuning { register: "editorial", header_px: (52, 88), max_type_sizes: 10, h1_max_px: 168, body_px: (17, 22), measure_chars: 78, button_px: (40, 56), section_pad_px: (96, 192), motion_ms: (180, 500), hero_blocks: 14 },
    CraftTuning { register: "craft", header_px: (60, 96), max_type_sizes: 9, h1_max_px: 120, body_px: (17, 20), measure_chars: 76, button_px: (44, 60), section_pad_px: (88, 168), motion_ms: (200, 450), hero_blocks: 14 },
    CraftTuning { register: "playful", header_px: (60, 96), max_type_sizes: 10, h1_max_px: 128, body_px: (16, 20), measure_chars: 76, button_px: (44, 64), section_pad_px: (80, 160), motion_ms: (180, 600), hero_blocks: 16 },
    CraftTuning { register: "civic", header_px: (60, 88), max_type_sizes: 8, h1_max_px: 80, body_px: (18, 21), measure_chars: 76, button_px: (48, 60), section_pad_px: (72, 136), motion_ms: (120, 250), hero_blocks: 14 },
];

/// Thresholds for the strongest register named (the first one with a tuning); `corporate` when
/// none is named.
pub fn tuning(registers: &[&str]) -> CraftTuning {
    let find = |r: &str| TUNINGS.iter().find(|t| t.register == r).copied();
    registers.iter().find_map(|r| find(r)).or_else(|| find(DEFAULT_REGISTER)).expect("default register has a tuning")
}

/// The craft floor for a brief's registers (strongest first, as `register::brief_registers`
/// returns them): concrete rules with numbers that every page meets whatever its direction. The
/// first register sets the numbers; each named register adds its own line.
pub fn craft_floor(registers: &[&str]) -> Vec<String> {
    let t = tuning(registers);
    let mut out = vec![
        format!(
            "Header: one row, {}-{}px tall on desktop and 56-76px on mobile; logo 24-40px high on the left, 4-6 nav links of 15-16px at weight 500 with 24-40px between them, the primary action on the right at the header's button size. Phone or hours sit in that row or the footer; no utility bar stacked above the nav. Mark the current link with a 2px underline 6px below the text; a sticky header turns opaque after the first scroll, never taller.",
            t.header_px.0, t.header_px.1
        ),
        format!(
            "Primary button: {}-{}px tall, horizontal padding 20-32px (about twice the vertical), label 15-17px at weight 600 in sentence or title case with tracking 0-0.02em, at most one drawn SVG icon of 16-20px set 8px from the label. The secondary action matches its height, radius and padding as an outline (1-1.5px) or ghost; one button shape and radius on the whole page, 44px minimum hit area everywhere.",
            t.button_px.0, t.button_px.1
        ),
        "Button states, all four authored: :hover shifts the fill by 6-10% lightness or nudges the icon 2-4px; :active presses with translateY(1px) or scale(0.98) over 80-120ms; :focus-visible draws a 2px ring at 2-3px offset with at least 3:1 contrast; disabled sits at 45-55% opacity with cursor not-allowed. No state changes width, height or padding.".to_string(),
        format!(
            "Type scale: at most {} distinct font sizes each carry 2% or more of the text on a desktop render, each step at least 1.2x the one below; body {}-{}px at line-height 1.5-1.65; headings at line-height 1.0-1.15. The h1 tops out at {}px on desktop (set with clamp()), and display sizes of 48px and up track -0.02 to -0.04em, never tighter than -0.05em; body text keeps tracking at 0.",
            t.max_type_sizes, t.body_px.0, t.body_px.1, t.h1_max_px
        ),
        format!(
            "Measure: prose never runs past {} characters a line (cap paragraphs at 60-68ch with max-width); h1 lines stay under 20ch and h2 under 24ch; no centred paragraph runs past three lines.",
            t.measure_chars
        ),
        format!(
            "Spacing rhythm: every gap comes from one 4/8px scale. Sections pad {}-{}px top and bottom on desktop and 64-96px on mobile; a heading sits 16-24px above its text and has at least twice that space above it; content width caps at 1200-1320px with 24-40px gutters on desktop and 16-20px on mobile.",
            t.section_pad_px.0, t.section_pad_px.1
        ),
        "Section heads: one heading per section (one h2, or the h1 in the hero) and at most one lead paragraph of 40 words or fewer. No stacked pile of label, heading, accent-coloured second heading and paragraph; a second voice goes in the body, not the head.".to_string(),
        "Composition: no two neighbouring sections share a layout; use 4 or more layout kinds across the page (split, full-bleed media, list or table, asymmetric grid, quote, form). Card rows vary span or media; never a grid of equal cards that differ only by text.".to_string(),
        format!(
            "Hero: the h1, one supporting paragraph of 30 words or fewer, at most two actions and one real picture or film; at most {} text blocks in the first viewport. Spec tables, chip rows and stat grids go below the fold (one slim proof strip may sit on the fold line). On a 390x844 phone a real picture is in view before the first scroll.",
            t.hero_blocks
        ),
        "Imagery: real photographs or renders of the trade's own objects, one aspect ratio per row (16:9, 3:2, 4:3 or 1:1) with object-fit: cover, corner radius from the button's family. Nothing decorative over a product picture: no hairlines, crosshairs, hotspot squares or graph-paper grids. Text on a photo sits on a scrim that holds 4.5:1; captions 13-14px in the muted ink.".to_string(),
        "Colour: one accent on no more than about 10% of any screen (primary action, links, one highlight); neutrals tinted toward the accent hue, never pure grey; at most two ground colours plus one accent ground across the page; body and secondary text at 4.5:1 or better, large text at 3:1.".to_string(),
        "Icons: one drawn SVG set at one stroke weight (1.5 or 2px) and one size per context (16, 20 or 24px), coloured with currentColor. Never a Unicode arrow, check mark, star or emoji standing in for an icon.".to_string(),
        format!(
            "Motion: one signature moment; every other transition runs {}-{}ms ease-out on transform and opacity only. Reveals travel 24px or less, start from content that is already readable, and differ by section role rather than one fade-up everywhere. No infinite loops besides hero media; prefers-reduced-motion turns all of it off.",
            t.motion_ms.0, t.motion_ms.1
        ),
        "Footer: brand and one-line description, two or three link groups and the contact block, four columns at most on desktop; group titles 13-14px at weight 600 in sentence case; links 15-16px with 10-12px between rows; a bottom bar for legal and copyright at 13-14px; 64px or more of top padding.".to_string(),
        "Browser surfaces: ::selection and focus rings use the palette; link underlines sit 3-4px below the text at 1-2px thickness; tables and figures use tabular numerals; no unthemed default blue links anywhere.".to_string(),
    ];
    let mut seen: Vec<&str> = Vec::new();
    for r in registers {
        if seen.contains(r) {
            continue;
        }
        seen.push(r);
        let line = match *r {
            "premium" => "Premium: fewer, larger moves; one display face used at 2-3 sizes only, generous negative space (a section may hold a single line and a picture), photography at 50% of the width or more.",
            "cinematic" => "Cinematic: the hero picture or film fills 60% or more of the first desktop viewport; dark grounds keep text at 4.5:1 through a scrim, and the one signature motion is tied to the subject (the screen lighting up, the product turning), never ambient drift.",
            "corporate" => "Corporate: the quote or contact action is reachable from every screen (header button plus one per long section), proof (client names, numbers with their source) sits within the first 2 scrolls (about 1800px).",
            "technical" => "Technical: figures in tabular numerals, real UI or diagrams instead of illustration, monospace only for actual code, radii 4-8px, nothing animates longer than 250ms.",
            "editorial" => "Editorial: a real reading column (60-68ch) with a serif or a strong grotesk at 18-21px; pictures run full-bleed or in the column, never as thumbnails in cards.",
            "craft" => "Craft: warm photography of hands, materials and the place in natural light, 3 or more real pictures above the footer, no stock; type with some character in the headings and a plain body; prices and opening hours are text, not images.",
            "playful" => "Playful: colour carries play, layout keeps order: bold hues on large grounds, one spring ease of 300-500ms for interactions, never more than one rotated or bouncing element per screen.",
            "civic" => "Civic: plain language at 18px or more, every action a labelled button of 48px or taller, contrast 7:1 for body text where possible, no motion needed to understand anything.",
            _ => continue,
        };
        out.push(line.to_string());
    }
    out
}

/// The craft floor read straight from a brief.
pub fn craft_floor_for_brief(brief: &str) -> Vec<String> {
    let signals = crate::intelligence::brief_signals(brief);
    let mut floor = craft_floor(&brief_registers(brief));
    let product = matches!(signals.surface, crate::intelligence::Surface::ProductUi | crate::intelligence::Surface::DevTool);
    let lower = brief.to_lowercase();
    for rule in &mut floor {
        if rule.starts_with("Composition:") {
            *rule = "Composition: choose section forms from the content and reading order. Repeated rows, equal cards and consistent app panels are useful when people compare equivalent items. Vary composition only when it improves hierarchy; no minimum number of layout kinds.".into();
        } else if rule.starts_with("Hero:") {
            *rule = if product { "Hero: this is a working interface. Put the primary task, essential quantities and actionable exceptions in view, including on phones. Use the product itself; no required marketing hero, stock photograph or forced display headline." } else { "Hero: establish the subject, dominant visual relationship and useful next action. Let the actual content determine the number of pictures, amount of copy and first-screen balance; no mandatory picture or fixed text-block quota." }.into();
        } else if rule.starts_with("Imagery:") {
            *rule = if lower.contains("gallery") || lower.contains("archive") { "Imagery: preserve the complete composition of collection works with intrinsic sizing or object-fit: contain. Use whitespace deliberately around differing aspect ratios. Keep artist, title, date and source readable; cropping requires an explicitly labelled detail view." } else { "Imagery: use relevant, licensed material and truthful labels. Decide crop and aspect ratio for each image's role; no universal ratio or radius. Label illustrative stock near the image and never imply it shows the actual client. Text on images must remain readable at 4.5:1." }.into();
        } else if rule.starts_with("Colour:") {
            *rule = "Colour: respect pinned brand colours and monochrome constraints. Assign clear roles to grounds, text, actions and status; choose saturation and colour area from the concept, without an accent-area quota. Body and secondary text need 4.5:1 contrast, large text 3:1.".into();
        } else if rule.starts_with("Motion:") {
            let dial = crate::intelligence::motion_dial_pin(brief)
                .unwrap_or(if signals.motion_bias >= 4 { 8 } else { 4 });
            let dial = signals.motion_cap.map_or(dial, |cap| dial.min(cap));
            *rule = format!("Motion: {} Keep content readable before initialization, honor prefers-reduced-motion and clean up timelines/listeners. {}", motion_delivery(dial).deliverable, motion_delivery(dial).evidence);
        } else if rule.starts_with("Spacing rhythm:") {
            *rule = "Spacing rhythm: use a coherent scale suited to density and content. Group related controls tightly and separate different tasks clearly. Preserve incumbent tokens; do not impose fixed section padding or an arbitrary 4/8px scale on every project.".into();
        } else if rule.starts_with("Craft:") {
            *rule = "Craft: use convincing material and characterful typography where the subject supports it. State prices and hours as readable text. Licensed illustrative stock is acceptable when honestly labelled; no compulsory picture count.".into();
        } else if signals.motion_cap == Some(0) && rule.starts_with("Button states") {
            *rule = "Button states: author hover, :active, disabled and :focus-visible feedback without movement. Use a visible 2px focus ring with 3:1 contrast; do not change the control's size or hide its label.".into();
        }
    }
    floor
}

/// Things a professional page never does, whatever the register. A brief that asks for one by
/// name may have it; habit never earns it.
pub fn anti_slop() -> Vec<String> {
    [
        "A coloured left or right border thicker than 1px as decoration on buttons, cards, list items, callouts or pictures.",
        "Unicode arrows, check marks, stars or emoji standing in for icons; a text link with a trailing arrow glyph pretending to be a button.",
        "A utility bar stacked above the navigation, or a header taller than one row.",
        "A sticky 'on this page' index or table of contents on a marketing page.",
        "Two headings in one section head, or a second heading in the accent colour restating the first.",
        "Eyebrow or kicker labels over headings, tiny tracked capitals, and section numbers (01 / 02) where the order means nothing.",
        "Spec tables, chip rows, stat grids or badges crowded into the hero beside the h1.",
        "Hairlines, crosshairs, hotspot squares or graph-paper grids drawn over product photography.",
        "Grids of identical icon cards, nested cards, and soft-shadowed rounded rectangles standing in for content.",
        "Gradient text, decorative glows, glass panels and blurred blobs; hard offset block shadows outside a brutalist world.",
        "Monospace type as a costume for 'technical' rather than for code or data.",
        "The same fade-up entrance on every section, scroll-jacking, parallax on body text, and hover effects that change an element's size.",
        "Misleading placeholder, stock or AI-filler pictures presented as client evidence; invented logos, invented testimonials, lorem ipsum. Licensed illustrative material is allowed when labelled truthfully.",
        "More than two typefaces, or more than one accent colour competing for the eye.",
        "Centred paragraphs longer than three lines, and prose that runs the full width of a wide screen.",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delivery_tracks_intent_without_forcing_animation_or_exceeding_scale() {
        assert!(motion_delivery(0).scene_spec.is_empty());
        assert_eq!(motion_delivery(3).tier, "quiet");
        assert_eq!(motion_delivery(6).tier, "expressive");
        assert_eq!(motion_delivery(8).tier, "high");
        assert_eq!(motion_delivery(255).dial, 10);
        let f = execution_foundation(8);
        assert_eq!(f.motion.dial, 8);
        assert!(f.spacing_px.windows(2).all(|s| s[0] < s[1]));
        assert!(f.spacing_px.iter().all(|n| n % 4 == 0));
        for (brief, high) in [
            ("highly animated agency, motion intensity: 8/10", true),
            ("highly animated agency, motion intensity: 2/10", false),
            ("highly animated agency, no animation", false),
        ] {
            let floor = craft_floor_for_brief(brief);
            let motion = floor.iter().find(|r| r.starts_with("Motion:")).unwrap();
            assert_eq!(motion.contains("visibly transformative"), high, "{motion}");
        }
    }

    fn has_number(s: &str) -> bool {
        s.chars().any(|c| c.is_ascii_digit())
    }

    #[test]
    fn every_register_gets_a_numeric_floor_of_the_right_size() {
        for r in crate::register::REGISTERS {
            let floor = craft_floor(&[r]);
            assert!((10..=16).contains(&floor.len()), "{r}: {} rules", floor.len());
            assert!(floor.iter().all(|l| has_number(l)), "{r}: a rule without a number: {:#?}", floor.iter().find(|l| !has_number(l)));
            assert_eq!(tuning(&[r]).register, *r);
        }
        assert!((10..=16).contains(&craft_floor(&[]).len()));
    }

    #[test]
    fn registers_tune_the_numbers() {
        let cinematic = tuning(&["cinematic", "premium"]);
        let technical = tuning(&["technical"]);
        assert!(cinematic.h1_max_px > technical.h1_max_px);
        assert!(cinematic.motion_ms.1 > technical.motion_ms.1);
        assert!(tuning(&["civic"]).body_px.0 >= 18);
        assert_eq!(tuning(&[]).register, "corporate");
        assert_eq!(tuning(&["unknown", "craft"]).register, "craft");
        let led = craft_floor(&["premium", "cinematic", "corporate"]);
        assert!(led.iter().any(|l| l.starts_with("Cinematic:")) && led.iter().any(|l| l.starts_with("Premium:")));
        assert!(led[0].contains("64-88px"), "{}", led[0]);
        assert!(craft_floor(&["technical"])[0].contains("52-72px"));
        assert!(craft_floor(&["craft", "craft"]).iter().filter(|l| l.starts_with("Craft:")).count() == 1);
    }

    #[test]
    fn the_floor_reads_a_brief_and_covers_the_craft_axes() {
        let floor = craft_floor_for_brief("a family-run bakery in the old town").join("\n");
        for axis in ["Header:", "Primary button:", "Button states", "Type scale:", "Measure:", "Spacing rhythm:", "Section heads:", "Composition:", "Hero:", "Imagery:", "Colour:", "Icons:", "Motion:", "Footer:", "Craft:"] {
            assert!(floor.contains(axis), "missing {axis}");
        }
        assert!(floor.contains(":active") && floor.contains(":focus-visible") && floor.contains("prefers-reduced-motion"));
    }

    #[test]
    fn anti_slop_names_the_amateur_tells() {
        let list = anti_slop();
        assert!((10..=16).contains(&list.len()));
        let all = list.join("\n").to_lowercase();
        for tell in ["left or right border", "unicode arrows", "utility bar", "on this page", "two headings", "eyebrow", "hero", "over product photography", "identical icon cards", "gradient text", "monospace", "fade-up"] {
            assert!(all.contains(tell), "missing {tell}");
        }
    }
}
