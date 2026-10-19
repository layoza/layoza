// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
//! Page shape: the dealt page-level form of a direction (hero frame, flow, section heads, ground,
//! material, labels, motion). Fonts and palettes alone let every page fall back to one house
//! layout; the page shape deals the form itself, distinct across the three directions of a deal
//! and steered away from the project's look ledger.
//!
//! Every axis value carries fit tags: `+ctx` scores the value up when the brief or direction has
//! that context (a surface, a register, the archetype's hero material or composition, the motion
//! tier), `-ctx` rules it out there, `house` marks the reflex default an AI falls back to (dealt
//! less often). Explicit asks in the brief pin an axis instead of dealing it.

use serde::{Deserialize, Serialize};

use super::{read_subject, tag_weight, Tone};
use crate::baseline::Look;
use crate::intelligence::{fnv1a, tokenize, BriefSignals, Surface};

/// One dealt axis value: a stable id (ledger key) and the instruction to build it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ShapePick {
    pub id: String,
    pub build: String,
}

/// One motion technique with the tool it is built with and its fallback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MotionTechnique {
    pub id: String,
    pub build: String,
    /// Tool id (`gsap`, `css-scroll`, `three`, `r3f`, `motion`, `view-transitions`, `waapi`, `lenis`,
    /// `canvas-svg`, `lottie`, `css`).
    pub tool: String,
    /// Tool as named to the builder ("GSAP + ScrollTrigger").
    pub tool_name: String,
    /// What reduced-motion, no-JS and no-WebGL visitors (and screenshots) see.
    pub fallback: String,
}

/// The page-level form of one direction. Binding: build every axis as dealt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PageShape {
    pub hero_frame: ShapePick,
    pub flow: ShapePick,
    pub section_head: ShapePick,
    pub ground: ShapePick,
    pub material: ShapePick,
    pub labels: ShapePick,
    /// `id` is the tier (`heavy`, `medium`, `micro`); `build` is the whole motion instruction.
    pub motion: ShapePick,
    /// The signature technique first; heavy tiers carry at least three.
    #[serde(default)]
    pub motion_techniques: Vec<MotionTechnique>,
    /// Signature technique id (ledger key).
    #[serde(default)]
    pub motion_signature: String,
    /// Tool ids used, in technique order, without duplicates (ledger key).
    #[serde(default)]
    pub motion_tools: Vec<String>,
    /// Axes the brief asked for explicitly (`ground`, `motion`, `material`...): set from the prompt,
    /// shared by all three directions and exempt from the uniqueness steering.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pinned_by_prompt: Vec<String>,
    /// The one motif this direction repeats (media frames, the primary button, bullets, one
    /// divider), drawn from one of the trade's own objects. Distinct across the directions.
    #[serde(default)]
    pub motif: ShapePick,
    /// The button shape, decided once for the page and carrying the motif.
    #[serde(default)]
    pub button: ShapePick,
    /// Page-wide restraint rules; each one already yields to an explicit ask in the brief.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restraint: Vec<String>,
}

impl PageShape {
    /// Compact text lines for terminals.
    pub fn lines(&self) -> Vec<String> {
        let pin = |axis: &str| if self.pinned_by_prompt.iter().any(|p| p == axis) { " (pinned by prompt)" } else { "" };
        let mut out = vec![
            format!("Hero frame [{}]{}: {}", self.hero_frame.id, pin("hero_frame"), self.hero_frame.build),
            format!("Flow [{}]{}: {}", self.flow.id, pin("flow"), self.flow.build),
            format!("Section heads [{}]{}: {}", self.section_head.id, pin("section_head"), self.section_head.build),
            format!("Ground [{}]{}: {}", self.ground.id, pin("ground"), self.ground.build),
            format!("Material [{}]{}: {}", self.material.id, pin("material"), self.material.build),
            format!("Labels [{}]{}: {}", self.labels.id, pin("labels"), self.labels.build),
            format!("Motion [{}]{}: signature {}, tools {}", self.motion.id, pin("motion"), self.motion_signature, self.motion_tools.join(", ")),
        ];
        out.extend(self.motion_techniques.iter().map(|t| format!("  - {} [{}]: {} Fallback: {}", t.id, t.tool_name, t.build, t.fallback)));
        out.push(format!("  {REDUCED}"));
        if !self.motif.id.is_empty() {
            out.push(format!("Motif [{}]: {}", self.motif.id, self.motif.build));
            out.push(format!("Buttons [{}]: {}", self.button.id, self.button.build));
        }
        if !self.restraint.is_empty() {
            out.push("Restraint (the brief's own asks override):".into());
            out.extend(self.restraint.iter().map(|r| format!("  - {r}")));
        }
        out
    }
}

pub(crate) const REDUCED: &str =
    "Reduced motion: under prefers-reduced-motion: reduce every effect is replaced by its final state; all content stays visible and readable.";

struct Opt {
    id: &'static str,
    build: &'static str,
    fit: &'static [&'static str],
}

const fn o(id: &'static str, build: &'static str, fit: &'static [&'static str]) -> Opt {
    Opt { id, build, fit }
}

const AXES: [&str; 6] = ["hero_frame", "flow", "section_head", "ground", "material", "labels"];

const HERO_FRAMES: &[Opt] = &[
    o("full-bleed-scene", "Full-bleed scene: one edge-to-edge image or rendered world, the headline set over it.", &["+showcase", "+venue", "+hero:photo", "+hero:video", "+hero:map", "+comp:full-bleed", "+bold", "+commerce"]),
    o("poster-type", "Poster-scale type fills the first screen: words only, no image, no widget.", &["+showcase", "+editorial", "+hero:giant-type", "+comp:poster", "+bold", "-commerce", "-product-ui", "-devtool", "-supplier"]),
    o("split", "Split: headline on one half, the working object on the other, for a stated reason.", &["house", "+comp:split"]),
    o("object-centre", "One object centre-stage at large scale; type tucked around or behind it.", &["+commerce", "+hero:object", "+hero:illustration", "+comp:object-wrap", "+comp:centred", "-devtool", "+venue", "+product-ui"]),
    o("pinned-story", "Pinned scroll story: the first screen stays fixed while the subject assembles on scroll.", &["+showcase", "+product-ui", "+comp:sticky-stage", "+tier:heavy", "-tier:micro", "-solemn", "+commerce", "+devtool"]),
    o("index-first", "Index first: the page opens on a list of real items with counts, no hero image.", &["+editorial", "+devtool", "+hero:document", "+hero:data-as-image", "-commerce", "+product-ui", "-supplier"]),
    o("product-ui-bleed", "Product UI full-bleed: the live interface fills the screen; the headline sits inside it.", &["+product-ui", "+devtool", "+hero:live-ui", "+hero:code-output", "-venue", "-solemn", "-commerce", "-showcase", "-editorial", "-general"]),
    o("collage", "Scattered collage: several real artefacts across the screen at different scales and angles.", &["+showcase", "+playful", "+hero:collage", "+hero:document", "-product-ui", "-devtool", "-solemn", "-commerce", "+venue", "+editorial", "-supplier"]),
    o("work-strip", "Short headline above a full-width strip of real work or products running sideways.", &["+showcase", "+commerce", "+venue", "-devtool"]),
    o("giant-figure", "One giant figure, date or price leads; the headline sits small beneath it, proof first.", &["+product-ui", "+commerce", "+hero:data-as-image", "-solemn", "-venue", "+devtool"]),
    o("close-crop", "Extreme close-up crop of one detail fills the screen; headline tucked in one corner.", &["+venue", "+commerce", "+hero:photo", "+hero:object", "+calm", "-product-ui", "-devtool", "+editorial"]),
    o("quiet-statement", "Centred quiet statement on open ground, one small image or mark below it.", &["+calm", "+solemn", "+trust", "+comp:centred", "-bold", "-commerce", "-supplier"]),
];

const FLOWS: &[Opt] = &[
    o("stacked-bands", "Stacked full-width bands, each section its own band with its own rhythm.", &["house"]),
    o("horizontal-chapters", "Horizontal chapters: one section scrolls sideways through panels, then the page continues down.", &["+showcase", "+commerce", "-tier:micro", "-solemn", "-devtool", "+venue"]),
    o("sticky-index", "Sticky index on one side; content panes scroll past it on the other side.", &["+product-ui", "+devtool", "+editorial", "+commerce", "+supplier"]),
    o("bento", "Bento mosaic: sections packed as unequal tiles in one grid, sized by importance.", &["+product-ui", "+commerce", "+devtool", "-solemn"]),
    o("editorial-column", "Single editorial column (~65ch) with images breaking out full-width between passages.", &["+editorial", "+solemn", "+calm", "-commerce", "+venue", "-supplier"]),
    o("snap-slides", "Full-screen snap slides: one idea per viewport, scroll snaps between them.", &["+showcase", "+bold", "-product-ui", "-devtool", "-solemn", "-commerce", "+venue", "-supplier"]),
    o("staggered", "Asymmetric staggered blocks alternating edges and widths down the page.", &["+showcase", "+venue", "+commerce", "+supplier"]),
    o("layered-deck", "Stacked deck: each section slides over the previous one as a sticky layer.", &["+showcase", "+product-ui", "-solemn", "+commerce", "+venue", "+supplier"]),
    o("spreads", "Magazine spreads: each section a facing pair, a full image page beside a text page.", &["+editorial", "+venue", "+showcase", "-product-ui", "-devtool", "-supplier"]),
    o("spine", "One continuous rail runs down the page; every section hangs off it in order.", &["+devtool", "+product-ui", "+solemn", "-commerce", "-supplier"]),
    o("product-shelves", "Product-first shelves: rows of real items with price and action lead every section.", &["+commerce", "-solemn", "-editorial", "-devtool", "-product-ui", "-showcase", "-venue", "-general"]),
    o("drawer-rows", "Numbered rows that open in place to reveal each section, like a chapter index.", &["+calm", "+solemn", "+devtool", "+editorial", "+trust", "-commerce", "-supplier"]),
];

const SECTION_HEADS: &[Opt] = &[
    o("title-note", "Title left, short note right on one shared baseline.", &["house", "+supplier"]),
    o("centred-overline", "Centred title with one small overline word above it; symmetric openings.", &["+calm", "+solemn", "+venue", "+commerce", "-supplier"]),
    o("margin-numeral", "Giant section numeral in the left margin, the title small beside it.", &["-trust", "+showcase", "+editorial", "+bold", "+venue", "+product-ui", "+devtool", "-supplier"]),
    o("run-in", "Run-in heads: a bold title opens the first sentence of the section, no line break.", &["+editorial", "+calm", "+devtool", "+venue", "+product-ui", "-supplier"]),
    o("outline-title", "Oversized outline title bleeding off the edge, filling solid as it scrolls in.", &["+showcase", "+bold", "-solemn", "-calm", "-trust", "+commerce", "-supplier"]),
    o("no-heads", "No section heads: the content itself and a change of ground mark each section.", &["+showcase", "+commerce"]),
    o("vertical-label", "Vertical label rotated along each section's edge; content starts at the top.", &["+showcase", "+editorial", "-solemn", "+venue", "-supplier"]),
    o("banner-rule", "Full-width title between hairline rules, set huge like a newspaper banner.", &["+editorial", "+bold", "+venue", "+devtool", "-supplier"]),
    o("question-heads", "Each head is a visitor's question in plain words; the answer starts right below.", &["+product-ui", "+devtool", "+solemn", "+calm", "+trust", "+supplier"]),
    o("tab-heads", "Heads sit as tabs on each section panel's top edge, like folder dividers.", &["+product-ui", "+devtool", "+commerce", "-supplier"]),
    o("inline-count", "Each head is one sentence carrying a live real number (\"212 jobs booked today\").", &["+product-ui", "+commerce", "-solemn", "-supplier"]),
];

const GROUNDS: &[Opt] = &[
    o("dark-throughout", "Dark ground throughout; one accent colour, no light sections.", &["-tone:light", "-tone:field"]),
    o("light-throughout", "Light ground throughout; sections separate by space and rules, not colour.", &["-tone:dark", "-tone:field"]),
    o("colour-fields", "Each section owns a saturated colour field from the palette; contrast checked per field.", &["+bold", "+playful", "-solemn", "-trust"]),
    o("soft-gradient", "Soft gradients drifting between two pale tints of the palette; no hard bands.", &["+calm", "-tone:dark"]),
    o("full-bleed-media", "Full-bleed image or 3D backdrops behind key sections, with a scrim for text contrast.", &["+venue", "+showcase", "+commerce"]),
    o("inverted-alternating", "Alternate ground and ink sections: palette background, then the foreground colour as ground.", &["+editorial", "+devtool"]),
    o("night-glow", "Dark ground lit by soft glow pools of the accent behind key objects.", &["+product-ui", "+devtool", "-tone:light", "-tone:field", "-solemn"]),
    o("tinted-field", "The palette's tinted field throughout; one darker band carries the main action.", &["-tone:light", "-tone:dark"]),
    o("light-interlude", "Dark ground throughout, broken once by a bright paper interlude where the key proof or offer sits.", &["+showcase", "+commerce", "+editorial", "-tone:light", "-tone:field"]),
    o("paper-grain", "Warm paper ground with a faint grain; sections part on torn or deckled edges, not boxes.", &["+editorial", "+calm", "+venue", "+playful", "-tone:dark", "-product-ui", "-devtool"]),
    o("split-ground", "Each section split down the middle: one half on the palette ground, the other on its ink.", &["+editorial", "+showcase", "+commerce", "+product-ui", "-solemn", "-calm"]),
    o("duotone-photo", "Photos washed into a two-colour duotone from the palette become the section grounds.", &["+venue", "+showcase", "+editorial", "+hero:photo", "-product-ui", "-devtool", "-solemn"]),
];

const MATERIALS: &[Opt] = &[
    o("flat-graphic", "Flat graphic shapes and solid colour blocks; no texture, shadow or photos.", &["+product-ui", "+devtool", "+showcase", "+commerce"]),
    o("paper-print", "Paper and print artefacts: tickets, receipts, labels, stamps, folded sheets.", &["house", "+editorial", "+hero:document", "-devtool", "+venue", "-supplier"]),
    o("glass-light", "Glass and light: translucent panels, blur, glow and refraction over a deep ground.", &["+product-ui", "+devtool", "+bold", "-solemn", "-trust", "-supplier"]),
    o("rendered-3d", "Rendered 3D objects of the trade, lit and shadowed, as the main visuals.", &["+commerce", "+showcase", "+bold", "+hero:object", "-solemn", "-devtool"]),
    o("illustrated", "One illustration style (one hand, one line weight) across every section.", &["+playful", "+venue", "+general", "+hero:illustration", "+editorial", "-supplier"]),
    o("typographic", "Typographic only: type, rules and numerals carry every section; no imagery.", &["+editorial", "+showcase", "+calm", "+hero:giant-type", "-commerce", "+venue", "+devtool", "-supplier"]),
    o("product-screens", "Real product screens and data states, cropped and annotated, as the visuals.", &["+product-ui", "+devtool", "+hero:live-ui", "+hero:code-output", "-venue", "-solemn"]),
    o("photographic", "Photographic art direction: real-looking scenes of people, places and the work.", &["+venue", "+showcase", "+solemn", "+commerce", "+hero:photo", "+hero:video", "-devtool", "+editorial", "+supplier"]),
    o("diagrammatic", "Diagrams and technical drawings: plans, exploded views, flow lines and callouts.", &["+devtool", "+product-ui", "+commerce", "+hero:data-as-image", "+editorial", "+supplier"]),
    o("tactile", "Tactile surfaces sampled from the trade itself: fabric, stone, wood, metal grain.", &["+venue", "+solemn", "+calm", "-product-ui", "-devtool", "-supplier"]),
    o("shoppable", "Shoppable cut-outs on plain ground with price, variant and add-to-cart on each.", &["+commerce", "-solemn", "-editorial", "-devtool", "-product-ui", "-showcase", "-general"]),
];

const LABELS: &[Opt] = &[
    o("mono-caps", "Monospace caps for labels, tags and metadata.", &["house", "+devtool", "-supplier"]),
    o("small-caps", "True small caps from the body face for labels and metadata.", &["+editorial", "+solemn", "+calm", "+venue", "+showcase", "-supplier"]),
    o("sentence-case", "Sentence case only, body face; labels read as plain words, no caps anywhere.", &["+calm", "+solemn", "+commerce", "+product-ui", "+trust", "+venue", "+devtool", "+supplier"]),
    o("numbered", "Numbered labels (01, 02, §3) set in the display face's figures.", &["+showcase", "+editorial", "+devtool", "+commerce", "+product-ui", "-supplier"]),
    o("pill-tags", "Labels as filled pill tags in the accent colour, sentence case.", &["+commerce", "+product-ui", "+playful", "-solemn", "-supplier"]),
    o("italic-captions", "Italic labels in the display face, set like captions in a book.", &["+editorial", "+venue", "+solemn", "+showcase", "-supplier"]),
    o("stamped", "Labels as rubber-stamp marks in the accent ink, slightly rotated, like a tag or a receipt stamp.", &["+playful", "+venue", "+commerce", "-solemn", "-trust", "-devtool", "+showcase", "-supplier"]),
    o("outline-tags", "Labels in hairline-outlined tags, no fill, in the body face.", &["+product-ui", "+devtool", "+commerce", "+supplier"]),
    o("accent-dash", "A short accent dash before each label, body face semibold, sentence case.", &["+trust", "+calm", "+product-ui", "+solemn", "+devtool", "+supplier"]),
    o("wide-caps", "Wide-tracked caps in the display face for labels; one size, one weight.", &["+showcase", "+bold", "+venue", "-solemn", "-trust", "-devtool", "-supplier"]),
];

fn pool(axis: usize) -> &'static [Opt] {
    [HERO_FRAMES, FLOWS, SECTION_HEADS, GROUNDS, MATERIALS, LABELS][axis]
}

/// Motion tier, from the direction's motion dial, capped by the niche and pinned by the prompt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Tier {
    Micro,
    Medium,
    Heavy,
}

impl Tier {
    fn id(self) -> &'static str {
        match self {
            Tier::Micro => "micro",
            Tier::Medium => "medium",
            Tier::Heavy => "heavy",
        }
    }
    fn of_dial(m: u8) -> Tier {
        match m {
            8.. => Tier::Heavy,
            4..=7 => Tier::Medium,
            _ => Tier::Micro,
        }
    }
}

/// Front-end stack, for the motion tools a direction may use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stack {
    React,
    Vue,
    Plain,
    Cms,
}

impl Stack {
    fn key(self) -> &'static str {
        match self {
            Stack::React => "react",
            Stack::Vue => "vue",
            Stack::Plain => "plain",
            Stack::Cms => "cms",
        }
    }
}

/// Tools: id, name, stacks it fits.
const TOOLS: &[(&str, &str, &[&str])] = &[
    ("gsap", "GSAP + ScrollTrigger", &["react", "vue", "plain", "cms"]),
    ("css-scroll", "CSS scroll-driven animations (animation-timeline: view()/scroll())", &["react", "vue", "plain", "cms"]),
    ("waapi", "Web Animations API", &["react", "vue", "plain", "cms"]),
    ("css", "CSS transitions", &["react", "vue", "plain", "cms"]),
    ("view-transitions", "View Transitions API", &["react", "vue", "plain"]),
    ("three", "Three.js", &["vue", "plain"]),
    ("r3f", "React Three Fiber", &["react"]),
    ("motion", "Motion (formerly Framer Motion)", &["react", "vue"]),
    ("lenis", "Lenis smooth scroll", &["react", "vue", "plain"]),
    ("canvas-svg", "Canvas 2D / SVG path morphs", &["react", "vue", "plain"]),
    ("lottie", "Lottie-style vector sequence (dotLottie / lottie-web)", &["react", "vue", "plain"]),
];

struct Tech {
    id: &'static str,
    tier: Tier,
    build: &'static str,
    fallback: &'static str,
    fit: &'static [&'static str],
    tools: &'static [&'static str],
}

const fn t(id: &'static str, tier: Tier, build: &'static str, fallback: &'static str, fit: &'static [&'static str], tools: &'static [&'static str]) -> Tech {
    Tech { id, tier, build, fallback, fit, tools }
}

const STATIC: &str = "no JS shows the same final frame, visible to screenshots.";

const TECHS: &[Tech] = &[
    // Heavy techniques.
    t("pinned-scene", Tier::Heavy, "Pinned scroll scene: one section pins while the subject assembles over ~200vh of scroll.", "the assembled final frame is the static layout;", &["+showcase", "+product-ui", "+commerce"], &["gsap", "css-scroll", "motion"]),
    t("kinetic-type", Tier::Heavy, "Kinetic split type: headline words and letters stagger in and re-set with scroll.", "the headline renders set and complete before any script runs;", &["+showcase", "+editorial", "+bold", "-trust", "-solemn"], &["gsap", "motion", "waapi"]),
    t("webgl-scene", Tier::Heavy, "WebGL scene: a lit 3D object or shader field of the trade behind the hero, paused off-screen.", "a pre-rendered still of the scene is the poster; no WebGL shows that still;", &["+showcase", "+commerce", "+bold", "-trust", "-solemn"], &["r3f", "three"]),
    t("scrubbed-sequence", Tier::Heavy, "Scroll-scrubbed sequence: a frame or image sequence of the product turning, tied to scroll.", "the key frame shows as a still image;", &["+commerce", "+product-ui"], &["gsap", "css-scroll", "canvas-svg"]),
    t("cursor-field", Tier::Heavy, "Cursor-reactive field: a grid, particle or type field leaning toward the pointer.", "the field renders at rest; touch and keyboard users lose nothing;", &["+showcase", "+playful", "-trust", "-solemn"], &["canvas-svg", "r3f", "three", "waapi"]),
    t("page-transitions", Tier::Heavy, "Page and view transitions: shared elements morph between list and detail views.", "plain navigation; every page is complete on load;", &["+commerce", "+product-ui", "+editorial", "+showcase"], &["view-transitions", "motion"]),
    t("marquee-physics", Tier::Heavy, "Marquee with physics: rows drift, speed up with scroll velocity and drag with inertia.", "rows sit still and wrap; every item stays reachable;", &["+showcase", "+commerce", "-trust", "-solemn", "-calm"], &["gsap", "motion", "waapi"]),
    t("smooth-depth", Tier::Heavy, "Smooth-scroll depth: layered parallax planes over inertial scroll.", "native scroll with flat planes;", &["+showcase", "+venue", "-trust", "-solemn"], &["lenis", "css-scroll"]),
    t("vector-sequence", Tier::Heavy, "Vector sequence: an animated line drawing of the trade's process plays as its section enters.", "the last frame shows as a static SVG;", &["+product-ui", "+devtool", "+editorial", "+venue"], &["lottie", "canvas-svg"]),
    // Medium signatures (on top of scroll reveals and hover physics).
    t("split-entrance", Tier::Medium, "Headline split entrance: words rise in once on load.", "the headline is visible without JS;", &["+showcase", "+editorial"], &["gsap", "motion", "waapi", "css-scroll"]),
    t("progress-rail", Tier::Medium, "Scroll-linked progress rail tracking the reader through the sections.", "the rail shows the section list, static;", &["+editorial", "+devtool", "+product-ui", "+solemn"], &["css-scroll", "gsap"]),
    t("drag-carousel", Tier::Medium, "Draggable carousel of real items with inertia and snap.", "a plain scroll-snap row;", &["+commerce", "+showcase", "-solemn"], &["motion", "gsap", "waapi"]),
    t("shared-element", Tier::Medium, "Shared-element transition from a list item into its detail view.", "plain navigation;", &["+commerce", "+product-ui"], &["view-transitions", "motion"]),
    t("count-up", Tier::Medium, "Real figures count up once to their values as they enter.", "the final numbers are in the HTML;", &["+product-ui", "+commerce", "-solemn"], &["waapi", "gsap"]),
    t("mask-wipe", Tier::Medium, "Images reveal through a mask wipe as they enter.", "images are fully shown;", &["+venue", "+showcase", "+editorial"], &["css-scroll", "gsap", "waapi"]),
    t("path-draw", Tier::Medium, "An SVG line draws itself along the section's route or process.", "the line is drawn complete;", &["+devtool", "+product-ui", "+venue", "+solemn", "+trust"], &["canvas-svg", "css-scroll", "gsap"]),
    t("pinned-steps", Tier::Medium, "One pinned section steps through three states of the subject.", "the three states stack as plain sections;", &["+product-ui", "+showcase"], &["gsap", "css-scroll", "motion"]),
    t("spring-tilt", Tier::Medium, "Cards tilt and lift toward the pointer with spring physics.", "cards sit flat;", &["+commerce", "+playful", "-solemn", "-trust"], &["motion", "waapi"]),
    // Micro signatures.
    t("underline-sweep", Tier::Micro, "Link underlines sweep in on hover and focus, 180ms.", "underlines show statically;", &["+editorial"], &["css"]),
    t("press-depth", Tier::Micro, "Buttons press down 1-2px with a shadow change on active.", "buttons keep their resting state;", &["+commerce", "+product-ui"], &["css"]),
    t("focus-draw", Tier::Micro, "Focus rings draw around controls in 150ms.", "focus rings show instantly;", &["+devtool", "+trust"], &["css"]),
    t("tint-shift", Tier::Micro, "Cards shift one tint on hover, 200ms.", "no tint shift;", &["+venue", "+solemn", "+calm"], &["css"]),
    t("image-settle", Tier::Micro, "Images settle from 1.02 scale to 1 once on load.", "images show at rest;", &["+showcase", "+venue"], &["waapi", "css"]),
    t("arrow-nudge", Tier::Micro, "Link and button arrows nudge 3px forward on hover and focus.", "arrows sit still;", &["+commerce", "+product-ui", "+devtool"], &["css"]),
    t("card-lift", Tier::Micro, "Cards lift 2px with a softer, wider shadow on hover, 180ms.", "cards sit flat;", &["+commerce", "+venue", "+showcase"], &["css"]),
    t("fade-in-once", Tier::Micro, "Sections fade from 0.6 to full opacity once as they enter, no movement.", "sections show at full opacity;", &["+editorial", "+solemn", "+calm", "+trust"], &["css-scroll", "waapi"]),
    t("caption-reveal", Tier::Micro, "Image captions slide up 4px and darken on hover and focus.", "captions show at rest;", &["+venue", "+editorial", "+showcase"], &["css"]),
    t("check-tick", Tier::Micro, "Form checks and toggles tick with a 160ms stroke draw.", "controls switch instantly;", &["+product-ui", "+trust", "+calm"], &["css", "waapi"]),
];

/// What the brief pins and its register, read once per deal.
pub(crate) struct ShapeBrief {
    ctx: Vec<&'static str>,
    pins: [Option<&'static str>; 6],
    tier_pin: Option<Tier>,
    /// Highest tier the niche allows unless the prompt asks for more.
    tier_cap: Tier,
    pub(crate) stack: Stack,
    /// True when the brief or project names the stack.
    pub(crate) stack_named: bool,
    seed: u64,
    pub(crate) tone_words: Vec<&'static str>,
    /// Palette and type axes the brief named ("palette" for a named colour or hex, "typography"
    /// for a named font): pinned there, every other axis still dealt and steered.
    brand_pins: Vec<&'static str>,
    /// Page-wide restraint rules, with the brief's explicit asks already carved out.
    restraint: Vec<String>,
}

const HIGH_MOTION_WORDS: &[&str] = &[
    "agency", "studio", "creative", "portfolio", "entertainment", "gaming", "game", "games", "esports", "fashion", "launch", "launches",
    "festival", "music", "film", "brand", "smartphone", "smartphones", "phone", "phones", "gadget", "gadgets", "electronics", "headphones",
    "sneakers", "streetwear", "nightclub", "club", "concert", "consumer",
    // Product marketing pages: a task UI (dashboard, admin) is held back by the audience/task cap instead.
    "saas", "startup", "software", "platform", "app",
];

const HIGH_TAGS: &[&str] = &["kinetic", "play", "celebration", "sport", "night", "y2k"];
const TRUST_TAGS: &[&str] = &["finance", "legal", "care", "health", "medical", "memorial", "civic"];

/// Phrase or whole-word hit in the lower-cased brief.
fn hit(q: &str, words: &[String], k: &str) -> bool {
    if k.contains(' ') { q.contains(k) } else { words.iter().any(|w| w == k) }
}

/// Words after "dark"/"light" that make it part of a thing, not a ground ask.
const NOT_GROUND: &[&str] = &["chocolate", "roast", "roasts", "sky", "skies", "academia", "matter", "horse", "web", "souls", "beer", "rum", "meat", "spirits", "bulb", "bulbs", "fixtures", "lunch", "lunches", "meals"];

/// Named colours the brief asks for, by palette accent-hue family.
const HUE_WORDS: &[(&str, &str)] = &[
    ("green", "green"), ("emerald", "green"), ("olive", "green"), ("sage", "green"), ("forest", "green"), ("lime", "green"),
    ("blue", "blue"), ("navy", "blue"), ("cobalt", "blue"), ("azure", "blue"),
    ("red", "red"), ("crimson", "red"), ("scarlet", "red"), ("burgundy", "red"), ("maroon", "red"),
    ("orange", "orange"), ("terracotta", "orange"), ("rust", "orange"), ("amber", "orange"),
    ("pink", "pink"), ("magenta", "pink"), ("fuchsia", "pink"), ("rose", "pink"),
    ("purple", "violet"), ("violet", "violet"), ("lilac", "violet"), ("lavender", "violet"),
    ("yellow", "yellow"), ("mustard", "yellow"), ("gold", "yellow"), ("golden", "yellow"),
    ("teal", "teal"), ("turquoise", "teal"), ("aqua", "teal"), ("cyan", "teal"),
];

/// Ground tone and accent hue the brief asks for in words ("dark", "light mode", "in green").
/// `query` is the affirmed brief. Colour words only count next to a colour cue ("in green", "green
/// and gold", "green palette") or after dark/light/deep/bright, so "green energy" stays a subject.
pub(crate) fn palette_pins(query: &str) -> (Option<Tone>, Option<&'static str>) {
    let q = query.to_lowercase();
    let words = tokenize(&q);
    let next_ok = |i: usize| words.get(i + 1).is_none_or(|n| !NOT_GROUND.contains(&n.as_str()));
    let dark = words.iter().enumerate().any(|(i, w)| matches!(w.as_str(), "dark" | "moody" | "dark-mode") && next_ok(i))
        || ["dark mode", "dark theme", "night mode"].iter().any(|k| q.contains(k));
    let light = ["light mode", "light theme", "light background", "light and airy", "bright and airy"].iter().any(|k| q.contains(k))
        || words.iter().any(|w| w == "airy");
    let tone = match (dark, light) {
        (true, false) => Some(Tone::Dark),
        (false, true) => Some(Tone::Light),
        _ => None,
    };
    const CUES_BEFORE: &[&str] = &["in", "dark", "deep", "light", "bright", "pale", "and", "&", "with", "using", "accent", "accents", "of"];
    const CUES_AFTER: &[&str] = &["palette", "colour", "color", "colours", "colors", "accent", "accents", "and", "theme", "tones", "brand"];
    let hue = words.iter().enumerate().find_map(|(i, w)| {
        let (_, fam) = HUE_WORDS.iter().find(|(k, _)| k == w)?;
        let before = i.checked_sub(1).and_then(|j| words.get(j)).is_some_and(|p| CUES_BEFORE.contains(&p.as_str()));
        let after = words.get(i + 1).is_some_and(|n| CUES_AFTER.contains(&n.as_str()));
        (before || after || i + 1 == words.len()).then_some(*fam)
    });
    (tone, hue)
}

impl ShapeBrief {
    /// `query` is the affirmed brief (negated parts removed); `raw` the user's own words, read only
    /// for negated motion asks ("no animation"). `framework` is the inspected project's, if any.
    pub(crate) fn read(signals: &BriefSignals, query: &str, raw: &str, framework: Option<&str>, variant: u64, heavy_brief: bool) -> ShapeBrief {
        let subject = read_subject(query, signals);
        let has = |tag: &str| tag_weight(signals, &subject, tag) > 0;
        let mood = |m: &str| signals.moods.iter().any(|x| x == m);
        let q = query.to_lowercase();
        let words = tokenize(&q);
        let raw_l = raw.to_lowercase();
        let solemn = has("memorial") || (has("legal") && !has("kinetic")) || (has("care") && !has("play"));
        // Software briefs the surface reader leaves general ("B2B SaaS for ...") read as product UI here.
        // "B2B" alone is a supplier as often as software, so it does not make a product UI by itself.
        let software = ["saas", "software", "app", "platform", "startup"].iter().any(|w| hit(&q, &words, w));
        // A quote-led B2B supplier sells on trust: plain labels, no stamps, motion capped at medium.
        let trust = solemn || TRUST_TAGS.iter().any(|t| has(t)) || (!software && crate::intelligence::b2b_brief(&words));
        let playful = has("play") || has("celebration") || has("sport");
        let mut ctx: Vec<&'static str> = vec![match signals.surface {
            Surface::General if software => "product-ui",
            Surface::ProductUi => "product-ui",
            Surface::Commerce => "commerce",
            Surface::Venue => "venue",
            Surface::Showcase => "showcase",
            Surface::Editorial => "editorial",
            Surface::DevTool => "devtool",
            Surface::General => "general",
        }];
        let mut tone_words: Vec<&'static str> = Vec::new();
        for (on, w) in [(solemn, "solemn"), (trust && !solemn, "trust"), (playful, "playful"), (mood("calm"), "calm"), (mood("bold"), "bold"), (mood("dark"), "dark"), (mood("light"), "light")] {
            if on {
                ctx.push(w);
                tone_words.push(w);
            }
        }
        if solemn {
            ctx.push("trust");
        }
        // A supplier selling to businesses on quotes: plain structure, no page-as-gimmick flows.
        if !software && crate::intelligence::b2b_brief(&words) {
            ctx.push("supplier");
        }
        // The trade's subject tags steer the motif deal.
        ctx.extend(MOTIF_TAGS.iter().filter(|t| has(t)));
        // Prompt pins: the user's explicit asks win over the deal.
        let mut pins: [Option<&'static str>; 6] = [None; 6];
        let pin_rules: &[(usize, &[&str], &str)] = &[
            (0, &["split hero", "split-screen", "split screen"], "split"),
            (0, &["video hero", "full-bleed", "fullscreen hero", "full-screen hero"], "full-bleed-scene"),
            // "Editorial type hierarchy" is a quality, not a layout; only a layout ask pins the column.
            (1, &["editorial layout", "editorial column", "magazine layout", "magazine-style", "magazine style", "longform", "long-form"], "editorial-column"),
            (1, &["bento"], "bento"),
            (1, &["slides", "slideshow", "fullpage", "full-page scroll"], "snap-slides"),
            (1, &["horizontal scroll", "sideways scroll"], "horizontal-chapters"),
            // A process timeline is a section, not the page's flow; only a page-level ask pins the spine.
            (1, &["timeline layout", "timeline page", "page as a timeline", "one long timeline", "vertical timeline site"], "spine"),
            (2, &["brutalist", "brutalism"], "banner-rule"),
            // "3D" is often the trade itself (3D LED, 3D printing); only a visuals ask pins rendered 3D.
            (4, &["3d renders", "3d visuals", "rendered 3d", "3d-rendered", "3d illustration", "3d illustrations", "3d objects"], "rendered-3d"),
            (4, &["glassmorphism", "frosted glass", "glass ui", "glass panels", "glass cards"], "glass-light"),
            (4, &["illustrated style", "illustration style", "illustration-led", "illustrated site", "illustrations throughout", "hand-drawn"], "illustrated"),
            (4, &["photography-led", "photographic", "photo-led"], "photographic"),
            (4, &["brutalist", "brutalism", "typographic", "type-led", "type-only"], "typographic"),
            (5, &["small caps", "small-caps"], "small-caps"),
            (5, &["monospace"], "mono-caps"),
        ];
        for (axis, keys, id) in pin_rules {
            if pins[*axis].is_none() && keys.iter().any(|k| hit(&q, &words, k)) {
                pins[*axis] = Some(id);
            }
        }
        let (tone_pin, hue_pin) = palette_pins(query);
        // A brief that asks for light and dark chapters gets the alternating ground, whatever its tone.
        let alternating = ["alternate light and dark", "alternating light and dark", "light and dark sections", "alternate dark and light", "dark and light sections", "alternate between light and dark"]
            .iter()
            .any(|k| q.contains(k));
        pins[3] = match tone_pin {
            _ if alternating => Some("inverted-alternating"),
            Some(Tone::Dark) => Some("dark-throughout"),
            Some(Tone::Light) => Some("light-throughout"),
            _ => None,
        };
        // "Static" and "still" name fallbacks as often as a mood ("a static image on slow devices");
        // a whole-page ask ("static site") is read as no motion above.
        let calm_words = ["minimal", "minimalist", "calm", "quiet", "understated", "subtle", "serene", "gentle"];
        let heavy_words = ["animated", "animation-heavy", "immersive", "kinetic", "cinematic", "motion-rich", "3d", "webgl", "bold", "scrollytelling", "experimental", "interactive"];
        let no_motion = ["no animation", "no animations", "no motion", "without animation", "static site", "static page"].iter().any(|p| raw_l.contains(p))
            || signals.constraints.iter().any(|c| c == "reduced-motion");
        let calm_ask = no_motion || calm_words.iter().any(|w| hit(&q, &words, w));
        let heavy_ask = heavy_brief || heavy_words.iter().any(|w| hit(&q, &words, w));
        let tier_pin = match (calm_ask, heavy_ask) {
            (true, false) => Some(Tier::Micro),
            (false, true) => Some(Tier::Heavy),
            (true, true) if no_motion => Some(Tier::Micro),
            _ => None,
        };
        let high_ok = signals.surface == Surface::Showcase || HIGH_TAGS.iter().any(|t| has(t)) || HIGH_MOTION_WORDS.iter().any(|w| hit(&q, &words, w));
        let tier_cap = if trust { Tier::Medium } else if high_ok { Tier::Heavy } else { Tier::Medium };
        // The brief's audience or task caps (seniors, a task UI) outrank a motion ask.
        let hard_cap = signals.motion_cap.map(Tier::of_dial);
        let tier_pin = tier_pin.map(|t| hard_cap.map_or(t, |c| t.min(c)));
        let tier_cap = hard_cap.map_or(tier_cap, |c| tier_cap.min(c));
        let fw = framework.unwrap_or("").to_lowercase();
        let named = |ks: &[&str]| ks.iter().any(|k| hit(&q, &words, k) || fw.contains(k));
        let (stack, stack_named) = if named(&["wordpress", "elementor", "gutenberg", "shopify", "liquid", "woocommerce", "bricks", "squarespace", "webflow", "wix"]) {
            (Stack::Cms, true)
        } else if named(&["react", "next", "nextjs", "next.js", "remix", "gatsby"]) {
            (Stack::React, true)
        } else if named(&["vue", "nuxt"]) {
            (Stack::Vue, true)
        } else {
            (Stack::Plain, named(&["html", "astro", "svelte", "sveltekit", "vanilla", "static html"]))
        };
        let mut brand_pins = Vec::new();
        if hue_pin.is_some() || !signals.brief_colors.is_empty() {
            brand_pins.push("palette");
        }
        if !signals.brief_fonts.is_empty() {
            brand_pins.push("typography");
        }
        let restraint = restraint_rules(&raw_l, signals.surface == Surface::DevTool || has("code"));
        ShapeBrief { ctx, pins, tier_pin, tier_cap, stack, stack_named, seed: fnv1a(&format!("{q}|{variant}")), tone_words, brand_pins, restraint }
    }

    /// The motion tier for a direction whose motion dial is `dial`.
    pub(crate) fn tier(&self, dial: u8) -> Tier {
        // A niche that allows heavy motion gets it on every direction by default; the dial only
        // places directions inside a calmer niche's range. The prompt's own ask always wins.
        self.tier_pin.unwrap_or_else(|| if self.tier_cap == Tier::Heavy { Tier::Heavy } else { Tier::of_dial(dial).min(self.tier_cap) })
    }

    /// The motion dial clamped into its tier (so dials and page shape agree).
    pub(crate) fn clamp_dial(&self, dial: u8) -> u8 {
        match self.tier(dial) {
            Tier::Heavy => dial.max(8),
            Tier::Medium => dial.clamp(4, 7),
            Tier::Micro => dial.min(3),
        }
    }

    /// Axes the prompt pinned, by name.
    pub(crate) fn pinned(&self) -> Vec<String> {
        let mut v: Vec<String> = AXES.iter().zip(self.pins.iter()).filter(|(_, p)| p.is_some()).map(|(a, _)| a.to_string()).collect();
        if self.tier_pin.is_some() {
            v.push("motion".into());
        }
        v.extend(self.brand_pins.iter().map(|p| p.to_string()));
        v
    }

    /// The prompt's motion wish, or the niche default for `dial`.
    pub(crate) fn motion_wish(&self, dial: u8) -> String {
        match self.tier_pin {
            Some(t) => format!("{} (asked in the brief)", t.id()),
            None => format!("{} (niche default, up to {})", self.tier(dial).id(), self.tier_cap.id()),
        }
    }

    pub(crate) fn stack_key(&self) -> &'static str {
        self.stack.key()
    }
}

/// One direction's inputs to the shape deal.
pub(crate) struct ShapeIn<'a> {
    pub hero_material: &'a str,
    pub composition: &'a str,
    pub tone: Tone,
    pub motion_dial: u8,
    /// The trade object this direction's motif is drawn from ("the module and its pixel pitch").
    pub object: &'a str,
}

thread_local! {
    /// Ledger entries (oldest first) the shape deal steers away from, set by [`with_recent_shapes`].
    static RECENT_SHAPES: std::cell::RefCell<Vec<Look>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Runs `f` with the ledger's recent looks steering the page-shape deal.
pub(crate) fn with_recent_shapes<T>(recent: Vec<Look>, f: impl FnOnce() -> T) -> T {
    RECENT_SHAPES.with(|r| *r.borrow_mut() = recent);
    let out = f();
    RECENT_SHAPES.with(|r| r.borrow_mut().clear());
    out
}

pub(crate) fn axis_of(l: &Look, axis: usize) -> Option<&str> {
    [&l.hero_frame, &l.flow, &l.section_head, &l.ground, &l.material, &l.labels][axis].as_deref()
}

fn tone_key(t: Tone) -> &'static str {
    match t {
        Tone::Light => "tone:light",
        Tone::Dark => "tone:dark",
        Tone::Field => "tone:field",
    }
}

/// Fit score of `fit` tags in `ctx`; `None` when a `-tag` rules the value out.
fn fit_score(fit: &[&str], ctx: &[String]) -> Option<i32> {
    let mut s = 0;
    for f in fit {
        if let Some(t) = f.strip_prefix('-') {
            if ctx.iter().any(|c| c == t) {
                return None;
            }
        } else if let Some(t) = f.strip_prefix('+') {
            if ctx.iter().any(|c| c == t) {
                s += 3;
            }
        } else if *f == "house" {
            s -= 2;
        }
    }
    Some(s)
}

/// Penalty for a value last used `age` runs back (0 = the newest run) by `slot`: any recent use
/// loses to an unused value; among used values the least recent run wins, and within one run the
/// value another direction used beats the one this direction used, so each direction rotates.
fn recency(last: Option<(usize, usize)>, slot: usize) -> i32 {
    last.map_or(0, |(age, s)| 200 - 10 * age.min(15) as i32 + if s == slot { 5 } else { 0 })
}

/// Deals one page shape per direction: every axis differs across the directions (a pinned axis is
/// shared), values that fit the brief and the direction score up, and values in the ledger window
/// lose to unused ones.
pub(crate) fn deal_shapes(b: &ShapeBrief, ins: &[ShapeIn]) -> Vec<PageShape> {
    let recent = RECENT_SHAPES.with(|r| r.borrow().clone());
    let ages = crate::baseline::run_slots(&recent);
    let newest_first: Vec<(&Look, (usize, usize))> = recent.iter().zip(ages).rev().collect();
    let last_run = recent.last().map(|l| l.run.clone());
    let paper_last_run = recent.iter().any(|l| Some(&l.run) == last_run.as_ref() && l.material.as_deref() == Some("paper-print"));
    let pinned = b.pinned();
    let jitter = |parts: &str| (fnv1a(&format!("{}|{parts}", b.seed)) % 4) as i32;
    let ctxs: Vec<Vec<String>> = ins
        .iter()
        .map(|d| {
            let mut c: Vec<String> = b.ctx.iter().map(|s| s.to_string()).collect();
            c.push(format!("hero:{}", d.hero_material));
            c.push(format!("comp:{}", d.composition));
            c.push(tone_key(d.tone).to_string());
            c.push(format!("tier:{}", b.tier(d.motion_dial).id()));
            c
        })
        .collect();
    let mut shapes: Vec<PageShape> = ins.iter().map(|_| PageShape { pinned_by_prompt: pinned.clone(), ..Default::default() }).collect();
    for axis in 0..AXES.len() {
        let mut used: Vec<&'static str> = Vec::new();
        for (i, ctx) in ctxs.iter().enumerate() {
            let opt = match b.pins[axis].and_then(|id| pool(axis).iter().find(|o| o.id == id)) {
                Some(p) => p,
                None => {
                    // Niche fit first: only values that fit the niche (or, when too few do, the
                    // best-fitting ones) enter the deal; the ledger and the jitter then
                    // choose among those, so uniqueness never reaches for a value the niche does not fit.
                    let fits: Vec<(i32, &Opt)> = pool(axis)
                        .iter()
                        .filter(|o| !(axis == 4 && o.id == "paper-print" && paper_last_run))
                        .filter_map(|o| Some((fit_score(o.fit, ctx)?, o)))
                        .collect();
                    let mut ranked: Vec<i32> = fits.iter().map(|(f, _)| *f).collect();
                    ranked.sort_unstable_by(|a, b| b.cmp(a));
                    // Enough values actively fit for every direction: neutral ones stay out. The tables
                    // give each surface five or more fitting values, so a repeated brief rotates.
                    let active = ranked.iter().filter(|f| **f > 0).count();
                    let floor = if active >= ins.len() { 1 } else { ranked.get(ins.len()).or(ranked.last()).copied().unwrap_or(0) };
                    let scored: Vec<(i32, &Opt)> = fits
                        .into_iter()
                        .filter(|(f, _)| *f >= floor)
                        .map(|(fit, o)| {
                            let last = newest_first.iter().find(|(l, _)| axis_of(l, axis) == Some(o.id)).map(|(_, a)| *a);
                            (fit - recency(last, i) + jitter(&format!("{axis}|{i}|{}", o.id)), o)
                        })
                        .collect();
                    let fresh: Vec<&(i32, &Opt)> = scored.iter().filter(|(_, o)| !used.contains(&o.id)).collect();
                    let best = if fresh.is_empty() { scored.iter().max_by_key(|(s, o)| (*s, o.id)) } else { fresh.into_iter().max_by_key(|(s, o)| (*s, o.id)) };
                    match best {
                        Some((_, o)) => o,
                        // Every value ruled out (cannot happen with the shipped tables): first value.
                        None => &pool(axis)[0],
                    }
                }
            };
            used.push(opt.id);
            let pick = ShapePick { id: opt.id.into(), build: opt.build.into() };
            let s = &mut shapes[i];
            match axis {
                0 => s.hero_frame = pick,
                1 => s.flow = pick,
                2 => s.section_head = pick,
                3 => s.ground = pick,
                4 => s.material = pick,
                _ => s.labels = pick,
            }
        }
    }
    // Motion: a signature per direction (distinct within the deal, steered from the ledger), more
    // techniques for heavy tiers, and a tool per technique that fits the stack.
    let tool_ok = |tool: &str| TOOLS.iter().any(|(id, _, st)| *id == tool && st.contains(&b.stack.key()));
    let mut used_sigs: Vec<&'static str> = Vec::new();
    let mut used_techs: Vec<&'static str> = Vec::new();
    let mut used_tools: Vec<&'static str> = Vec::new();
    for (i, d) in ins.iter().enumerate() {
        let tier = b.tier(d.motion_dial);
        let ctx = &ctxs[i];
        let usable = |x: &&Tech| x.tier == tier && x.tools.iter().any(|t| tool_ok(t));
        let scored: Vec<(i32, &Tech)> = TECHS
            .iter()
            .filter(usable)
            .filter_map(|x| {
                let fit = fit_score(x.fit, ctx)?;
                let last = newest_first.iter().find(|(l, _)| l.motion_signature.as_deref() == Some(x.id)).map(|(_, a)| *a);
                Some((fit - recency(last, i) + jitter(&format!("sig|{i}|{}", x.id)), x))
            })
            .collect();
        let fresh: Vec<&(i32, &Tech)> = scored.iter().filter(|(_, x)| !used_sigs.contains(&x.id)).collect();
        let sig = if fresh.is_empty() { scored.iter().max_by_key(|(s, x)| (*s, x.id)) } else { fresh.into_iter().max_by_key(|(s, x)| (*s, x.id)) };
        let mut techs: Vec<&Tech> = sig.map(|(_, x)| *x).into_iter().collect();
        if tier == Tier::Heavy {
            let mut more: Vec<(i32, &Tech)> = TECHS
                .iter()
                .filter(usable)
                .filter(|x| !techs.iter().any(|y| y.id == x.id))
                .filter_map(|x| {
                    let fit = fit_score(x.fit, ctx)?;
                    let again = if used_techs.contains(&x.id) { 4 } else { 0 };
                    Some((fit - again + jitter(&format!("tech|{i}|{}", x.id)), x))
                })
                .collect();
            more.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.id.cmp(b.1.id)));
            techs.extend(more.into_iter().take(2).map(|(_, x)| x));
        }
        let mut mine: Vec<&'static str> = Vec::new();
        let mut out: Vec<MotionTechnique> = Vec::new();
        for x in &techs {
            let tool = x
                .tools
                .iter()
                .copied()
                .filter(|t| tool_ok(t))
                .enumerate()
                .max_by_key(|(k, t)| {
                    let d = newest_first.iter().take(6).position(|(l, _)| l.motion_tools.iter().any(|u| u == t));
                    let mut s = -(*k as i32);
                    if mine.contains(t) {
                        s -= 6;
                    }
                    if used_tools.contains(t) {
                        s -= 2;
                    }
                    if d.is_some() {
                        s -= 3;
                    }
                    s + (fnv1a(&format!("{}|tool|{i}|{}|{t}", b.seed, x.id)) % 2) as i32
                })
                .map(|(_, t)| t)
                .unwrap_or("css");
            mine.push(tool);
            used_tools.push(tool);
            used_techs.push(x.id);
            let name = TOOLS.iter().find(|(id, _, _)| *id == tool).map_or(tool, |(_, n, _)| n);
            let fallback = if tier == Tier::Heavy { format!("{} {STATIC}", x.fallback) } else { x.fallback.trim_end_matches(';').to_string() + "." };
            out.push(MotionTechnique { id: x.id.into(), build: x.build.into(), tool: tool.into(), tool_name: name.to_string(), fallback });
        }
        if let Some(s) = techs.first() {
            used_sigs.push(s.id);
        }
        let shape = &mut shapes[i];
        let names: Vec<String> = out.iter().map(|m| format!("{} ({})", m.id, m.tool_name)).collect();
        let build = match tier {
            Tier::Heavy => format!("Heavy motion: {}. Each technique keeps its fallback. {REDUCED}", names.join("; ")),
            Tier::Medium => format!("Medium motion: scroll reveals (opacity plus at most 24px translate, ending visible) and hover physics on cards and links, plus one signature: {}. {REDUCED}", names.join("; ")),
            Tier::Micro => format!("Micro motion only: hover, focus and press states (150-200ms), no scroll choreography; signature: {}. {REDUCED}", names.join("; ")),
        };
        shape.motion = ShapePick { id: tier.id().into(), build };
        shape.motion_signature = out.first().map(|m| m.id.clone()).unwrap_or_default();
        let mut tools: Vec<String> = Vec::new();
        for m in &out {
            if !tools.contains(&m.tool) {
                tools.push(m.tool.clone());
            }
        }
        shape.motion_tools = tools;
        shape.motion_techniques = out;
    }
    // Motif: one per direction, distinct within the deal, fitted to the trade and tone and drawn
    // from that direction's own trade object. Buttons follow the motif, decided once per page.
    let mut used_motifs: Vec<&'static str> = Vec::new();
    for (i, d) in ins.iter().enumerate() {
        let best = MOTIFS
            .iter()
            .filter(|m| !used_motifs.contains(&m.id))
            .filter_map(|m| {
                let last = newest_first.iter().find(|(l, _)| l.motif.as_deref() == Some(m.id)).map(|(_, a)| *a);
                Some((fit_score(m.fit, &ctxs[i])? - recency(last, i) / 50 + jitter(&format!("motif|{i}|{}", m.id)) % 3, m))
            })
            .max_by_key(|(s, m)| (*s, m.id))
            .map_or(&MOTIFS[0], |(_, m)| m);
        used_motifs.push(best.id);
        let object = if d.object.is_empty() { "the trade's own object" } else { d.object };
        let s = &mut shapes[i];
        s.motif = ShapePick { id: best.id.into(), build: best.build.replace("{object}", object) };
        s.button = ShapePick { id: best.button_id.into(), build: best.button.into() };
        s.restraint = b.restraint.clone();
    }
    shapes
}

/// Subject tags that steer the motif deal.
const MOTIF_TAGS: &[&str] = &[
    "industrial", "infrastructure", "data", "code", "kinetic", "night", "luxe", "ink", "food", "earth", "care", "play",
    "celebration", "civic", "legal", "finance", "memorial", "trades", "sport",
];

struct Motif {
    id: &'static str,
    build: &'static str,
    fit: &'static [&'static str],
    button_id: &'static str,
    button: &'static str,
}

/// One motif per direction, repeated with discipline on media frames, the primary button, list
/// bullets and one divider between chapters. Nothing else decorates the page.
const MOTIFS: &[Motif] = &[
    Motif {
        id: "stepped-corner",
        build: "Stepped corner: a three-step pixel cut in one corner of every media frame and of the primary button, taken from {object}; square bullets echo it, and one band of the same steps divides two chapters. Nothing else decorates.",
        fit: &["+industrial", "+code", "+data", "+kinetic", "+supplier", "+devtool", "-solemn", "-care"],
        button_id: "square-stepped",
        button: "Square primary button with the stepped corner cut top-right; the secondary is a plain square outline. One height, radius 0 everywhere.",
    },
    Motif {
        id: "seam-grid",
        build: "Module seams: a hairline grid at the trade's real unit runs behind media and along section edges, taken from {object}; frames and figures snap to it, and the active item fills one cell in the accent.",
        fit: &["+industrial", "+infrastructure", "+data", "+devtool", "+product-ui", "+supplier", "+finance"],
        button_id: "square-cell",
        button: "Square buttons one grid cell high, radius 2px at most; primary filled, secondary hairline outline.",
    },
    Motif {
        id: "lit-edge",
        build: "Lit edge: one 3px accent line on a single edge of every media frame, the primary button and the active tab, like the edge of {object} switched on; it brightens on hover and focus and never glows.",
        fit: &["+industrial", "+night", "+kinetic", "+supplier", "+dark", "+bold", "+code", "-solemn"],
        button_id: "square-lit",
        button: "Square primary button with the accent line on its leading edge; on hover the line runs the full outline. Secondary is text with the same line under it.",
    },
    Motif {
        id: "cut-corner",
        build: "Cut corner: one 45-degree chamfer on the same corner of frames, panels and the primary button, taken from the edge of {object}; the chamfer size is one token, used everywhere.",
        fit: &["+industrial", "+trades", "+sport", "+bold", "+supplier", "+infrastructure", "-solemn", "-care"],
        button_id: "chamfer",
        button: "Chamfered buttons with one corner cut at the motif's size; the secondary is an outline with the same cut.",
    },
    Motif {
        id: "ruled-frame",
        build: "Ruled frame: every chapter opens on a hairline rule with a short thick start segment, as on a specification sheet for {object}; figures and captions hang from the rule.",
        fit: &["+legal", "+finance", "+civic", "+editorial", "+solemn", "+trust", "+supplier", "+infrastructure"],
        button_id: "square-ruled",
        button: "Square primary button, radius 0-2px; the secondary is an underlined text link with an arrow.",
    },
    Motif {
        id: "registration-marks",
        build: "Registration marks: small corner crop marks frame each image and key figure, as on a press proof of {object}.",
        fit: &["+ink", "+editorial", "+showcase", "+kinetic", "-solemn", "-supplier"],
        button_id: "square-marked",
        button: "Square buttons that gain the crop marks on hover; the secondary is a plain outline.",
    },
    Motif {
        id: "notched-edge",
        build: "Notched edge: semicircle notches on the sides of media frames and the primary button, like a ticket or tag that comes with {object}.",
        fit: &["+venue", "+celebration", "+night", "+food", "+playful", "-trust", "-solemn", "-supplier"],
        button_id: "notched",
        button: "Buttons with the notch on both short sides; the secondary is an outline with the same notches.",
    },
    Motif {
        id: "arch-top",
        build: "Arch top: media frames and the main feature panel rise into one round arch drawn from {object}; one arch, one radius, repeated.",
        fit: &["+solemn", "+care", "+earth", "+luxe", "+calm", "+venue", "+memorial", "-supplier"],
        button_id: "pill",
        button: "Pill buttons echoing the arch; the secondary is a text link.",
    },
    Motif {
        id: "torn-edge",
        build: "Torn edge: chapters part on one torn paper edge, and media frames carry the same edge on one side, as on the wrapping of {object}.",
        fit: &["+editorial", "+food", "+earth", "+play", "+playful", "-trust", "-supplier"],
        button_id: "soft-square",
        button: "Soft square buttons, radius 6px, solid; the secondary is underlined text.",
    },
    Motif {
        id: "round-mask",
        build: "Round mask: one feature image per chapter sits in a full circle and bullets are dots, taken from {object}; everything else stays square.",
        fit: &["+care", "+play", "+product-ui", "+commerce", "+calm", "-supplier"],
        button_id: "pill",
        button: "Pill buttons; the secondary is a pill outline.",
    },
];

/// Page-wide restraint, the discipline that separates an authored page from a kit. Each rule
/// narrows when the brief asks for the thing it bans: the brief always wins.
fn restraint_rules(raw: &str, code: bool) -> Vec<String> {
    let asks = |ks: &[&str]| ks.iter().any(|k| raw.contains(k));
    let mut out = vec![if asks(&["eyebrow", "kicker", "overline"]) {
        "Eyebrow labels only where the brief names them, short and sentence case; no others, no tiny tracked caps.".to_string()
    } else {
        "No eyebrow or kicker labels above headings, and no tiny tracked caps anywhere.".to_string()
    }];
    if !code && !asks(&["monospace", "mono font", "code font"]) {
        out.push("No monospace face: figures use the body face's tabular numerals.".into());
    }
    out.push(if asks(&["01/", "01 /", "01 —", "numbered step", "step number", "numbered process"]) {
        "Index numerals only where the brief numbers things; no section numbering elsewhere.".to_string()
    } else {
        "No index numerals (01, 02) on sections or steps; an ordered list carries order.".to_string()
    });
    out.push(if asks(&["glow", "gradient", "luminous", "lighting effect", "light effect", "glass"]) {
        "One light effect, only where the brief puts it; no gradient washes, blobs or glass elsewhere.".to_string()
    } else {
        "No gradients, glows, glass panels or blobs: flat colour chapters carry the page.".to_string()
    });
    out.extend(
        [
            "No icon cards and no row of three or more identical cards: vary size, span and media per item.",
            "At most two typefaces; hierarchy comes from size and weight.",
            "One h1, authored in at most two lines at every width; statements stay under about 25 words.",
            "Sections 120-240px apart on desktop (about clamp(88px, 11vw, 168px)); never cramped bands.",
            "Real pictures of the trade's objects wherever the brief supplies them, four or more; no stock filler or placeholder art.",
            "One live interaction born from the product (its deciding spec made interactive) with a no-JS table of the same facts.",
            "The footer returns the brand line large across the full width.",
        ]
        .map(String::from),
    );
    out
}

/// What Layoza read from the brief before dealing, with niche-based defaults for everything the
/// brief left out. Build on the assumptions without asking; `open_questions` lists only what
/// blocks building at all.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UnderstoodBrief {
    /// Matched trade or niche, with match confidence.
    pub trade: String,
    /// The subject in the brief's own words.
    pub subject: String,
    pub audience: String,
    pub primary_action: String,
    pub tone: Vec<String>,
    /// Motion tier the prompt asked for, else the niche default.
    pub motion_wish: String,
    /// Front-end stack the brief or project names (`react`, `vue`, `cms`, `plain`); none when unnamed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
    /// Sections, pages, colours and fonts the brief explicitly asks for.
    #[serde(default)]
    pub must_haves: Vec<String>,
    /// Axes the brief named; they override the deal for that axis only.
    #[serde(default)]
    pub pinned_by_prompt: Vec<String>,
    /// Niche-based defaults for what the brief did not say. Proceed on them.
    #[serde(default)]
    pub assumed: Vec<String>,
    /// At most two questions, only for what blocks building (an unknown trade).
    #[serde(default)]
    pub open_questions: Vec<String>,
}

impl UnderstoodBrief {
    pub fn lines(&self) -> Vec<String> {
        let mut out = vec![
            format!("Trade: {}", self.trade),
            format!("Subject: {}", self.subject),
            format!("Audience: {}", self.audience),
            format!("Primary action: {}", self.primary_action),
            format!("Tone: {}", self.tone.join(", ")),
            format!("Motion: {}", self.motion_wish),
            format!("Stack: {}", self.stack.as_deref().unwrap_or("not named")),
        ];
        if !self.must_haves.is_empty() {
            out.push(format!("Must-haves: {}", self.must_haves.join(", ")));
        }
        if !self.pinned_by_prompt.is_empty() {
            out.push(format!("Pinned by prompt: {}", self.pinned_by_prompt.join(", ")));
        }
        out.extend(self.assumed.iter().map(|a| format!("Assumed: {a}")));
        out.extend(self.open_questions.iter().map(|q| format!("Open question: {q}")));
        out
    }
}

const AUDIENCE_WORDS: &[&str] = &[
    "developers", "engineers", "designers", "teams", "families", "couples", "parents", "kids", "children", "students", "teachers",
    "startups", "founders", "enterprises", "creators", "artists", "musicians", "gamers", "patients", "clinicians", "doctors", "nurses",
    "homeowners", "landlords", "tenants", "travellers", "travelers", "tourists", "locals", "investors", "recruiters", "dispatchers",
    "contractors", "retailers", "agencies", "freelancers", "seniors", "athletes", "runners", "collectors", "shoppers",
];
const AUDIENCE_PHRASES: &[&str] = &["small businesses", "small business owners", "pet owners", "b2b", "b2c", "smbs", "first-time buyers", "home cooks"];

const SECTION_WORDS: &[&str] = &[
    "pricing", "faq", "faqs", "testimonials", "reviews", "gallery", "blog", "contact", "booking", "menu", "team", "about", "careers",
    "newsletter", "events", "docs", "documentation", "demo", "signup", "shop", "cart", "checkout", "services", "features", "integrations",
    "changelog", "roadmap", "case", "process", "packages", "map", "locations", "schedule", "timetable", "calendar", "donate", "donations",
    "rsvp", "press", "awards", "clients", "partners", "login", "comparison", "specs", "configurator",
];
const ASK_CUES: &[&str] = &["need", "needs", "with", "include", "includes", "including", "must", "plus", "add", "want", "wants", "has", "have"];
const AFTER_CUES: &[&str] = &["section", "sections", "page", "pages", "form", "table", "tables", "studies", "grid", "list", "widget"];

/// Niche defaults by surface: audience, primary action, tone.
fn surface_defaults(surface: &str) -> (&'static str, &'static str, &'static str) {
    match surface {
        "product-ui" => ("the teams that buy and use the product day to day", "start a trial or book a demo", "clear, confident, task-first"),
        "devtool" => ("developers evaluating the tool", "install it or read the docs", "precise, technical, no hype"),
        "commerce" => ("shoppers comparing products before buying", "add a product to the cart", "direct, product-proud"),
        "venue" => ("locals and visitors deciding whether to come", "book or plan a visit", "warm, specific to the place"),
        "showcase" => ("prospective clients judging the work", "start a project / get in touch", "confident, work-first"),
        "editorial" => ("readers", "read and subscribe", "considered, readable"),
        _ => ("people looking for this service", "get in touch", "plain and specific"),
    }
}

/// Reads the brief before dealing. `confidence` is the industry match confidence ("strong",
/// "weak", "none"); `dial` the recommendation's motion dial.
pub(crate) fn understand(b: &ShapeBrief, signals: &BriefSignals, query: &str, trade: &str, confidence: &str, dial: u8) -> UnderstoodBrief {
    let subject = read_subject(query, signals);
    let q = query.to_lowercase();
    let words = tokenize(&q);
    let (surface_audience, surface_action, surface_tone) = surface_defaults(b.ctx[0]);
    let mut assumed = Vec::new();
    let named: Vec<String> = AUDIENCE_PHRASES
        .iter()
        .filter(|p| q.contains(*p))
        .map(|p| if *p == "b2b" { "business buyers (B2B)".to_string() } else { p.to_string() })
        .chain(AUDIENCE_WORDS.iter().filter(|w| words.iter().any(|x| x == *w)).map(|w| w.to_string()))
        .collect();
    let b2b = crate::intelligence::b2b_brief(&words);
    let audience = if named.is_empty() && b2b {
        assumed.push("audience: business buyers specifying for their own site (B2B, quote-led)".into());
        "business buyers specifying for their own site or premises".to_string()
    } else if named.is_empty() {
        assumed.push(format!("audience: {surface_audience} (niche default)"));
        surface_audience.to_string()
    } else {
        named.join(", ")
    };
    // A bare verb ("buy") says less than the trade's own action, so the trade's wins over it.
    let trade_action = subject.from_trades.then(|| subject.action.clone());
    let brief_action = signals.brief_action.clone().filter(|a| a.contains(' ') || trade_action.is_none());
    let primary_action = match brief_action.or(trade_action) {
        // The business sells; the visitor buys.
        Some(a) if a.starts_with("sell ") => format!("buy {}", &a[5..]),
        Some(a) if !a.is_empty() => a,
        _ => {
            assumed.push(format!("primary action: {surface_action} (niche default)"));
            surface_action.to_string()
        }
    };
    let mut tone: Vec<String> = b.tone_words.iter().map(|t| t.to_string()).collect();
    if tone.is_empty() {
        assumed.push(format!("tone: {surface_tone} (niche default)"));
        tone.push(surface_tone.to_string());
    }
    let motion_wish = b.motion_wish(dial);
    if b.tier_pin.is_none() {
        assumed.push(format!("motion: {motion_wish}"));
    }
    let stack = b.stack_named.then(|| b.stack_key().to_string());
    if stack.is_none() {
        assumed.push("stack: the project's own framework if one exists, else plain HTML/CSS/JS".into());
    }
    let mut must_haves: Vec<String> = Vec::new();
    for (i, w) in words.iter().enumerate() {
        if !SECTION_WORDS.contains(&w.as_str()) {
            continue;
        }
        let asked_before = words[i.saturating_sub(3)..i].iter().any(|p| ASK_CUES.contains(&p.as_str()));
        let asked_after = words.get(i + 1).is_some_and(|n| AFTER_CUES.contains(&n.as_str()));
        let item = if w == "case" { "case studies".to_string() } else { w.clone() };
        if (asked_after || (asked_before && w != "case")) && !must_haves.contains(&item) {
            must_haves.push(item);
        }
    }
    must_haves.extend(signals.brief_colors.iter().map(|c| format!("colour {c}")));
    if let (_, Some(h)) = palette_pins(query) {
        must_haves.push(format!("{h} palette"));
    }
    must_haves.extend(signals.brief_fonts.iter().map(|f| format!("font {f}")));
    let mut open_questions = Vec::new();
    if confidence == "none" && !subject.from_trades && signals.brief_objects.is_empty() && subject.nouns.is_empty() {
        open_questions.push("What does this business do or sell? The brief names no trade, so the page has no subject to show.".into());
    }
    UnderstoodBrief {
        trade: format!("{trade} ({confidence} match)"),
        subject: if subject.phrase.is_empty() { trade.to_string() } else { subject.phrase.clone() },
        audience,
        primary_action,
        tone,
        motion_wish,
        stack,
        must_haves,
        pinned_by_prompt: b.pinned(),
        assumed,
        open_questions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intelligence::{looks_of, recommend_design_avoiding, recommend_design_variant, DesignRecommendation, OperatingMode};

    fn rec(q: &str) -> DesignRecommendation {
        recommend_design_variant(q, OperatingMode::NewWebsite, None, None, 0)
    }

    fn shapes(r: &DesignRecommendation) -> Vec<&PageShape> {
        r.directions.iter().map(|d| d.page_shape.as_ref().expect("new sites carry a page shape")).collect()
    }

    fn axis(p: &PageShape, a: &str) -> String {
        match a {
            "hero_frame" => p.hero_frame.id.clone(),
            "flow" => p.flow.id.clone(),
            "section_head" => p.section_head.id.clone(),
            "ground" => p.ground.id.clone(),
            "material" => p.material.id.clone(),
            "labels" => p.labels.id.clone(),
            _ => p.motion_signature.clone(),
        }
    }

    #[test]
    fn niche_fit_gates_the_deal_before_uniqueness() {
        // A phone store is dealt shoppable, visual shapes: never a document material, a words-only
        // hero, a chapter-index flow or a foreign trade's archetype.
        let r = rec("mobile phone store ecommerce selling phones, cases, chargers and earbuds");
        for (d, p) in r.directions.iter().zip(shapes(&r)) {
            assert!(!["blueprint-sheet", "price-board", "status-board"].contains(&d.archetype.as_str()), "{}", d.archetype);
            assert!(!["paper-print", "typographic", "tactile"].contains(&p.material.id.as_str()), "{}", p.material.id);
            assert!(!["poster-type", "index-first", "quiet-statement"].contains(&p.hero_frame.id.as_str()), "{}", p.hero_frame.id);
            assert!(!["drawer-rows", "editorial-column", "spine"].contains(&p.flow.id.as_str()), "{}", p.flow.id);
        }
        // Heavy-motion niches never get an archetype whose motion is "nothing moves".
        for q in ["independent brand and digital design agency", "B2B SaaS for scheduling field-service technicians"] {
            let r = rec(q);
            for d in &r.directions {
                assert_ne!(d.archetype, "ticket-stub", "{q}");
            }
        }
    }

    #[test]
    fn the_same_brief_asked_again_is_a_new_design() {
        // The ledger steers archetype, palette, type and furniture, so one brief asked run after
        // run is a new structure and colour each time; a Latin brief never gets a CJK or Hebrew face.
        let q = "neighbourhood sourdough bakery";
        let non_latin = |font: &str| {
            super::super::type_pool().iter().any(|e| e.pairing.display_font == font && e.scripts.iter().any(|s| super::super::NON_LATIN.contains(s)))
        };
        let mut recent: Vec<crate::baseline::Look> = Vec::new();
        let (mut archs, mut pals) = (Vec::new(), Vec::new());
        let mut prev_nav: Option<String> = None;
        let (mut pale, mut total) = (0, 0);
        for i in 0..6 {
            let r = recommend_design_avoiding(q, OperatingMode::NewWebsite, None, None, 0, &recent);
            let a = &r.directions[0];
            archs.push(a.archetype.clone());
            pals.push(a.palette.id.clone());
            let nav = a.nav.as_ref().map(|m| m.id.clone());
            assert!(prev_nav.is_none() || nav != prev_nav, "run {i}: nav {nav:?} repeats the last run");
            prev_nav = nav;
            for d in &r.directions {
                assert!(!non_latin(&d.typography.display_font), "run {i}: {}", d.typography.display_font);
                pale += (super::super::Tone::of(&d.palette) == super::super::Tone::Light) as usize;
                total += 1;
            }
            // The real reader keeps the last RECENT_LOOK_RUNS runs, oldest first.
            recent.extend(looks_of(&r, &format!("run-{i}")));
            let keep = crate::baseline::RECENT_LOOK_RUNS * r.directions.len();
            if recent.len() > keep {
                recent.drain(..recent.len() - keep);
            }
        }
        archs.sort();
        archs.dedup();
        pals.sort();
        pals.dedup();
        assert!(archs.len() >= 5 && pals.len() >= 5, "archetypes {archs:?}, palettes {pals:?}");
        assert!(pale * 3 <= total * 2, "{pale} of {total} grounds pale");
        // Freshness never outranks niche fit: a phone store asked again and again stays a shop.
        let q = "online store selling smartphones and phone accessories";
        let mut recent: Vec<crate::baseline::Look> = Vec::new();
        for i in 0..6 {
            let r = recommend_design_avoiding(q, OperatingMode::NewWebsite, None, None, 0, &recent);
            for d in &r.directions {
                assert!(!["blueprint-sheet", "price-board", "status-board", "almanac"].contains(&d.archetype.as_str()), "run {i}: {}", d.archetype);
            }
            recent.extend(looks_of(&r, &format!("store-{i}")));
            let keep = crate::baseline::RECENT_LOOK_RUNS * r.directions.len();
            if recent.len() > keep {
                recent.drain(..recent.len() - keep);
            }
        }
    }

    const WILD: &[&str] = &["kinetic-type", "webgl-scene", "cursor-field", "marquee-physics", "smooth-depth"];

    #[test]
    fn pools_are_wide_enough() {
        let sizes: Vec<usize> = (0..AXES.len()).map(|a| pool(a).len()).collect();
        assert!(sizes[0] >= 7 && sizes[1] >= 7 && sizes[2] >= 7 && sizes[3] >= 5 && sizes[4] >= 7 && sizes[5] >= 4, "{sizes:?}");
        // Every palette tone keeps at least three grounds, even for a solemn brief.
        for tone in ["tone:light", "tone:dark", "tone:field"] {
            let ctx = vec![tone.to_string(), "solemn".to_string(), "trust".to_string()];
            let n = GROUNDS.iter().filter(|g| fit_score(g.fit, &ctx).is_some()).count();
            assert!(n >= 3, "{tone}: {n}");
        }
    }

    #[test]
    fn directions_of_one_deal_differ_on_every_axis() {
        for q in [
            "independent brand and digital design agency",
            "B2B SaaS for scheduling field-service technicians",
            "online store selling smartphones and phone accessories",
            "family funeral home",
            "neighbourhood sourdough bakery",
            "employment law firm",
            "open source CLI for database migrations",
        ] {
            let r = rec(q);
            let s = shapes(&r);
            for a in ["hero_frame", "flow", "section_head", "ground", "material", "labels", "motion"] {
                let ids: Vec<String> = s.iter().map(|p| axis(p, a)).collect();
                assert!(ids.iter().all(|i| !i.is_empty()), "{q} {a} empty");
                let mut u = ids.clone();
                u.sort();
                u.dedup();
                assert_eq!(u.len(), 3, "{q}: {a} repeats within the deal: {ids:?}");
            }
            assert!(s.iter().filter(|p| p.material.id == "paper-print").count() <= 1, "{q}");
            // The ground agrees with the direction's palette tone.
            for (d, p) in r.directions.iter().zip(&s) {
                let tone = Tone::of(&d.palette);
                let g = GROUNDS.iter().find(|g| g.id == p.ground.id).unwrap();
                assert!(fit_score(g.fit, &[tone_key(tone).to_string()]).is_some(), "{q}: ground {} on {:?}", p.ground.id, tone);
                assert!(p.motion.build.contains("prefers-reduced-motion"), "{q}");
            }
        }
    }

    #[test]
    fn three_ledger_runs_share_no_hero_flow_head_material_or_signature() {
        let briefs = ["independent brand and digital design agency", "B2B SaaS for scheduling field-service technicians", "online store selling smartphones and phone accessories"];
        let mut recent = Vec::new();
        let mut seen: std::collections::BTreeMap<&str, Vec<String>> = Default::default();
        for (i, q) in briefs.iter().enumerate() {
            let r = recommend_design_avoiding(q, OperatingMode::NewWebsite, None, None, 0, &recent);
            // Direction A of each run: the one the AI builds by default.
            let p = shapes(&r)[0];
            for a in ["hero_frame", "flow", "section_head", "material", "motion"] {
                let v = axis(p, a);
                assert!(!seen.get(a).is_some_and(|s| s.contains(&v)), "{q}: {a} {v} repeats {:?}", seen.get(a));
                seen.entry(a).or_default().push(v);
            }
            // Oldest first, as the ledger reader returns it.
            recent.extend(looks_of(&r, &format!("run-{i}")));
        }
    }

    #[test]
    fn same_brief_three_runs_never_repeat_the_motion_signature_or_paper_back_to_back() {
        for q in ["neighbourhood sourdough bakery", "independent brand and digital design agency"] {
            let mut recent: Vec<Look> = Vec::new();
            let mut sigs: Vec<String> = Vec::new();
            let mut paper_prev = false;
            for i in 0..3 {
                let r = recommend_design_avoiding(q, OperatingMode::NewWebsite, None, None, 0, &recent);
                let s = shapes(&r);
                for p in &s {
                    assert!(!sigs.contains(&p.motion_signature), "{q} run {i}: {} again ({sigs:?})", p.motion_signature);
                    sigs.push(p.motion_signature.clone());
                }
                let paper = s.iter().any(|p| p.material.id == "paper-print");
                assert!(!(paper && paper_prev), "{q}: paper-print in consecutive runs");
                paper_prev = paper;
                recent.extend(looks_of(&r, &format!("run-{i}")));
            }
        }
    }

    #[test]
    fn dark_minimal_pins_ground_and_calm_motion() {
        let r = rec("dark minimal portfolio for a wedding photographer in React, needs a pricing section");
        for p in shapes(&r) {
            assert_eq!(p.ground.id, "dark-throughout");
            assert_eq!(p.motion.id, "micro");
            assert!(p.pinned_by_prompt.contains(&"ground".to_string()) && p.pinned_by_prompt.contains(&"motion".to_string()), "{:?}", p.pinned_by_prompt);
        }
        for d in &r.directions {
            assert!(d.dials.motion_intensity <= 3, "{}", d.dials.motion_intensity);
            assert_eq!(Tone::of(&d.palette), Tone::Dark, "{}", d.palette.id);
        }
        let u = r.understood_brief.as_ref().unwrap();
        assert!(u.pinned_by_prompt.contains(&"ground".to_string()) && u.pinned_by_prompt.contains(&"motion".to_string()), "{u:?}");
        assert_eq!(u.stack.as_deref(), Some("react"));
        assert!(u.must_haves.iter().any(|m| m == "pricing"), "{:?}", u.must_haves);
        assert!(u.open_questions.is_empty(), "{:?}", u.open_questions);
    }

    #[test]
    fn highly_animated_agency_gets_heavy_motion_with_several_tools() {
        let r = rec("highly animated agency website");
        let s = shapes(&r);
        for p in &s {
            assert_eq!(p.motion.id, "heavy");
            assert!(p.motion_techniques.len() >= 3, "{:?}", p.motion_techniques);
            assert!(p.motion_tools.len() >= 2, "{:?}", p.motion_tools);
            assert!(p.motion_techniques.iter().all(|t| t.fallback.contains("no JS")), "heavy techniques carry a static-frame fallback");
        }
        let mut sigs: Vec<&str> = s.iter().map(|p| p.motion_signature.as_str()).collect();
        sigs.sort();
        sigs.dedup();
        assert_eq!(sigs.len(), 3);
    }

    #[test]
    fn niches_that_allow_it_default_to_heavy_motion_without_being_asked() {
        for q in ["independent brand and digital design agency", "B2B SaaS for scheduling field-service technicians"] {
            for p in shapes(&rec(q)) {
                assert_eq!(p.motion.id, "heavy", "{q}");
            }
        }
        for q in ["family funeral home", "admin dashboard for field-service dispatchers"] {
            assert!(shapes(&rec(q)).iter().all(|p| p.motion.id != "heavy"), "{q} stays calmer");
        }
    }

    #[test]
    fn solemn_briefs_never_get_wild_motion() {
        for q in ["family funeral home", "estate planning law firm", "hospice care for families", "highly animated funeral home"] {
            for p in shapes(&rec(q)) {
                // An explicit motion ask may raise the tier; the wild techniques stay out regardless.
                if !q.starts_with("highly") {
                    assert_ne!(p.motion.id, "heavy", "{q}");
                }
                assert!(p.motion_techniques.iter().all(|t| !WILD.contains(&t.id.as_str())), "{q}: {:?}", p.motion_techniques);
            }
        }
    }

    #[test]
    fn tools_follow_the_stack() {
        for (q, banned) in [
            ("highly animated agency website in React", &["three"][..]),
            ("highly animated agency website on WordPress", &["three", "r3f", "motion", "lenis", "view-transitions", "canvas-svg", "lottie"][..]),
            ("highly animated agency website", &["r3f", "motion"][..]),
        ] {
            for p in shapes(&rec(q)) {
                assert!(p.motion_tools.iter().all(|t| !banned.contains(&t.as_str())), "{q}: {:?}", p.motion_tools);
            }
        }
    }

    #[test]
    fn pinning_one_axis_keeps_the_others_unique() {
        let first = rec("independent design agency");
        let recent = looks_of(&first, "run-0");
        let r = recommend_design_avoiding("agency site in dark green", OperatingMode::NewWebsite, None, None, 0, &recent);
        let u = r.understood_brief.as_ref().unwrap();
        assert!(u.pinned_by_prompt.contains(&"ground".to_string()) && u.pinned_by_prompt.contains(&"palette".to_string()), "{u:?}");
        assert!(!u.pinned_by_prompt.contains(&"motion".to_string()) && !u.pinned_by_prompt.contains(&"typography".to_string()), "{u:?}");
        let old_fonts: Vec<String> = first.directions.iter().map(|d| d.typography.display_font.clone()).collect();
        let old_sigs: Vec<String> = shapes(&first).iter().map(|p| p.motion_signature.clone()).collect();
        for (d, p) in r.directions.iter().zip(shapes(&r)) {
            assert_eq!(p.ground.id, "dark-throughout");
            assert_eq!(Tone::of(&d.palette), Tone::Dark, "{}", d.palette.id);
            assert!(!old_fonts.contains(&d.typography.display_font), "{} repeats the ledger", d.typography.display_font);
            assert!(!old_sigs.contains(&p.motion_signature), "{} repeats the ledger", p.motion_signature);
        }
        let check = crate::uniqueness::check(&looks_of(&r, "x"), &recent, 20, true);
        let grounds: Vec<_> = check.slots.iter().filter(|s| s.slot.ends_with(".ground")).collect();
        assert!(!grounds.is_empty() && grounds.iter().all(|s| s.status == "pinned by prompt"), "{grounds:?}");
    }

    #[test]
    fn understood_brief_assumes_instead_of_asking() {
        let u = rec("B2B SaaS for scheduling field-service technicians").understood_brief.unwrap();
        assert!(u.open_questions.is_empty(), "{:?}", u.open_questions);
        assert!(!u.assumed.is_empty(), "{:?}", u.assumed);
        assert!(u.stack.is_none());
        let unknown = rec("website").understood_brief.unwrap();
        assert!((1..=2).contains(&unknown.open_questions.len()), "{:?}", unknown.open_questions);
    }
}
