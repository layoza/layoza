// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Design Intelligence Engine: 4 Operating Modes (A, B, C, D), 18+ Industry Profiles,
// Searchable UI Style Database with provenance, Typography Pairings, Color Palettes,
// 3 Adjustable Dials (Design Variance, Motion Intensity, Visual Density), and 3-Direction Concept Dealer.

use crate::dna::{DesignDials, SiteDna};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatingMode {
    /// Mode A: New Website from scratch (offers 3 materially distinct creative directions)
    NewWebsite,
    /// Mode B: Extend Existing Website (inspects Site DNA first; strict preservation mode)
    ExtendExisting,
    /// Mode C: Improve Existing Website (baseline first, targeted improvements, zero regression)
    ImproveExisting,
    /// Mode D: Full Creative Redesign (baseline + rollback, 3 new directions preserving content)
    FullRedesign,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryRule {
    pub slug: String,
    pub name: String,
    pub keywords: Vec<String>,
    pub recommended_styles: Vec<String>,
    pub layout_archetype: String,
    pub first_viewport_priority: String,
    pub typography_pairing: TypographyPairing,
    pub palette: PaletteRecommendation,
    pub conversion_trust_elements: Vec<String>,
    pub motion_language: String,
    pub anti_patterns: Vec<String>,
    pub default_dials: DesignDials,
    pub provenance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyPairing {
    pub id: String,
    pub display_font: String,
    pub body_font: String,
    pub mono_font: String,
    pub mood: String,
    pub scale_ratio: String,
    pub google_fonts_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteRecommendation {
    pub id: String,
    pub background: String,
    pub surface: String,
    pub foreground: String,
    pub muted: String,
    pub primary_accent: String,
    pub secondary_accent: String,
    pub wcag_body_contrast: f64,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleFamily {
    pub slug: String,
    pub name: String,
    pub category: String,
    pub keywords: Vec<String>,
    pub characteristics: Vec<String>,
    pub best_for: Vec<String>,
    pub avoid_when: Vec<String>,
    pub provenance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeDirection {
    pub index: u8,
    pub title: String,
    pub style_family: String,
    pub layout_composition: String,
    pub visual_identity: String,
    pub typography: TypographyPairing,
    pub palette: PaletteRecommendation,
    pub content_treatment: String,
    pub motion_choreography: String,
    pub dials: DesignDials,
    /// Dealt mechanics for this direction: 1 hero + 2 signature sections (`layoza mechanics show <id>`).
    #[serde(default)]
    pub mechanics: Vec<crate::mechanics::MechanicRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignRecommendation {
    pub mode: OperatingMode,
    pub query: String,
    pub matched_industry: IndustryRule,
    /// Keyword score of the matched industry (0 = no keyword matched; the profile is a generic fallback).
    pub match_score: usize,
    /// "strong" (>= 6), "weak" (3-5), or "none" (0, generic fallback — derive the world from the brief).
    pub match_confidence: String,
    /// Next-best industry slugs with their scores, for the agent to consider.
    pub alternative_industries: Vec<(String, usize)>,
    pub distinctiveness: DistinctivenessBrief,
    /// Surface, niche families, moods and constraints read from the brief; these drive Directions B and C.
    pub brief_signals: BriefSignals,
    pub matched_styles: Vec<StyleFamily>,
    pub active_dials: DesignDials,
    pub preservation_enforced: bool,
    pub directions: Vec<CreativeDirection>,
    pub workflow_guidance: Vec<String>,
}

/// Brief-derived guidance that pushes a design away from category defaults toward the brand's own world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistinctivenessBrief {
    /// Places named in the brief (e.g. "Lahore"): mine local architecture, crafts, signage, and colour.
    pub place_cues: Vec<String>,
    /// Materials, products, and processes named in the brief: sample palette and texture from them.
    pub material_cues: Vec<String>,
    /// The look most generators produce for this category; do not ship it by default.
    pub category_cliches: Vec<String>,
    pub instruction: String,
}

/// Lower-cased word tokens (letters, digits, `.`) for whole-word keyword matching. `-` and `_` split
/// words, so slugs like `saas-startup` and hyphenated keywords match as phrases ("saas startup").
fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !(c.is_alphanumeric() || c == '.'))
        .map(|t| t.trim_matches('.').to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

/// Whole-word (or whole-phrase) match, so `ai` no longer matches "detail" and `art` no longer matches "start".
fn keyword_hit(tokens: &[String], keyword: &str) -> bool {
    let parts = tokenize(keyword);
    match parts.len() {
        0 => false,
        1 => tokens.iter().any(|t| *t == parts[0] || (parts[0].len() > 3 && *t == format!("{}s", parts[0]))),
        n => tokens.windows(n).any(|w| w.iter().zip(&parts).all(|(a, b)| a == b)),
    }
}

/// Scores every industry and returns them best-first as (score, rule).
fn rank_industries(query: &str, industries: &[IndustryRule]) -> Vec<(usize, IndustryRule)> {
    let tokens = tokenize(query);
    let mut ranked: Vec<(usize, IndustryRule)> = industries
        .iter()
        .map(|ind| {
            let mut score = 0usize;
            if keyword_hit(&tokens, &ind.slug.replace('-', " ")) {
                score += 5;
            }
            score += ind.keywords.iter().filter(|kw| keyword_hit(&tokens, kw)).count() * 3;
            (score, ind.clone())
        })
        .collect();
    ranked.sort_by(|a, b| b.0.cmp(&a.0));
    ranked
}

/// WCAG 2.x contrast ratio between two `#RRGGBB` colours (None if either is not a 6-digit hex).
pub fn wcag_contrast(fg: &str, bg: &str) -> Option<f64> {
    fn lum(hex: &str) -> Option<f64> {
        let h = hex.trim_start_matches('#');
        if h.len() != 6 {
            return None;
        }
        let ch = |i: usize| -> Option<f64> {
            let v = u8::from_str_radix(&h[i..i + 2], 16).ok()? as f64 / 255.0;
            Some(if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) })
        };
        Some(0.2126 * ch(0)? + 0.7152 * ch(2)? + 0.0722 * ch(4)?)
    }
    let (a, b) = (lum(fg)?, lum(bg)?);
    Some((a.max(b) + 0.05) / (a.min(b) + 0.05))
}

/// Replaces the hand-written contrast figure with the computed one and flags accents that are unsafe for text.
/// Deals each direction its own hero + 2 signature mechanics, at the direction's motion dial and with
/// no mechanism family shared between directions. Existing sites (Mode B/C) are capped by their DNA.
fn attach_mechanics(directions: &mut [CreativeDirection], query: &str, mode: OperatingMode, existing_dna: Option<&SiteDna>) {
    use crate::mechanics::{deal_mechanics_with, DealOptions, ExistingSite, Intensity, MechanicRef};
    let existing = match mode {
        OperatingMode::ExtendExisting | OperatingMode::ImproveExisting => existing_dna.map(|d| ExistingSite::from_dna(d, "SITE-DNA")),
        _ => None,
    };
    let base = fnv1a(&query.to_lowercase());
    let mut used_families: Vec<String> = Vec::new();
    for d in directions.iter_mut() {
        let deal = deal_mechanics_with(&DealOptions {
            brief: query.to_string(),
            intensity: Some(Intensity::from_dial(d.dials.motion_intensity)),
            count: 2,
            seed: Some(base.wrapping_add(d.index as u64)),
            existing: existing.clone(),
            exclude_families: used_families.clone(),
            ..Default::default()
        });
        d.mechanics = deal.hero.iter().chain(&deal.signature).map(MechanicRef::from).collect();
        used_families.extend(d.mechanics.iter().map(|m| m.family.clone()));
    }
}

fn verify_palette(p: &mut PaletteRecommendation) {
    if let Some(ratio) = wcag_contrast(&p.foreground, &p.background) {
        p.wcag_body_contrast = (ratio * 100.0).round() / 100.0;
    }
    if let Some(accent) = wcag_contrast(&p.primary_accent, &p.background) {
        if accent < 4.5 && !p.notes.contains("primary accent is") {
            p.notes.push_str(&format!(
                " | Computed: primary accent is {:.2}:1 on the background, so use it for fills, large text (>= 24px) or icons only, never body-size text.",
                accent
            ));
        }
    }
}

fn fnv1a(text: &str) -> u64 {
    text.bytes().fold(0xcbf29ce484222325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3))
}

fn pairing(id: &str, display: &str, body: &str, mono: &str, mood: &str, ratio: &str, url: &str) -> TypographyPairing {
    TypographyPairing {
        id: id.into(),
        display_font: display.into(),
        body_font: body.into(),
        mono_font: mono.into(),
        mood: mood.into(),
        scale_ratio: ratio.into(),
        google_fonts_url: url.into(),
    }
}

#[allow(clippy::too_many_arguments)]
fn palette(id: &str, bg: &str, surface: &str, fg: &str, muted: &str, primary: &str, secondary: &str, contrast: f64, notes: &str) -> PaletteRecommendation {
    PaletteRecommendation {
        id: id.into(),
        background: bg.into(),
        surface: surface.into(),
        foreground: fg.into(),
        muted: muted.into(),
        primary_accent: primary.into(),
        secondary_accent: secondary.into(),
        wcag_body_contrast: contrast,
        notes: notes.into(),
    }
}

/// What the brief is building; decides which compositions for Directions B and C make sense.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Surface {
    /// Dashboards, admin, internal tools: task speed beats spectacle.
    ProductUi,
    /// Stores and catalogues: the product and the purchase path lead.
    Commerce,
    /// Restaurants, cafes, hotels, venues: the place, the menu and booking lead.
    Venue,
    /// Studios, portfolios, agencies: the work itself is the show.
    Showcase,
    /// Magazines, journals, blogs: reading comes first.
    Editorial,
    General,
}

/// Niche, mood and named constraints read from the brief (deterministic keyword rules).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BriefSignals {
    pub surface: Surface,
    /// Palette/subject families detected in the brief (e.g. `marine`, `warm-grain`, `textile`, `kinetic`).
    pub niche_tags: Vec<String>,
    /// Mood words found in the brief (e.g. `calm`, `bold`, `dark`).
    pub moods: Vec<String>,
    /// Named constraints that override the defaults (e.g. `reduced-motion`, `performance`).
    pub constraints: Vec<String>,
    /// Net motion adjustment applied to every direction's dial.
    pub motion_bias: i8,
    /// Hard ceiling on motion intensity for every direction, when a constraint or surface demands one.
    pub motion_cap: Option<u8>,
}

/// Subject word -> palette/subject family.
const NICHE_LEXICON: &[(&str, &str)] = &[
    ("seafood", "marine"), ("fish", "marine"), ("oyster", "marine"), ("oysters", "marine"), ("harbour", "marine"),
    ("harbor", "marine"), ("coast", "marine"), ("coastal", "marine"), ("sea", "marine"), ("ocean", "marine"),
    ("marina", "marine"), ("catch", "marine"), ("surf", "marine"), ("sailing", "marine"), ("beach", "marine"),
    ("bakery", "warm-grain"), ("bread", "warm-grain"), ("pastry", "warm-grain"), ("patisserie", "warm-grain"),
    ("flour", "warm-grain"), ("dough", "warm-grain"), ("sourdough", "warm-grain"), ("oven", "warm-grain"),
    ("croissant", "warm-grain"), ("loaves", "warm-grain"), ("cafe", "warm-grain"), ("coffee", "warm-grain"),
    ("fashion", "textile"), ("linen", "textile"), ("apparel", "textile"), ("clothing", "textile"),
    ("womenswear", "textile"), ("menswear", "textile"), ("textile", "textile"), ("cotton", "textile"),
    ("wool", "textile"), ("silk", "textile"), ("denim", "textile"), ("boutique", "textile"), ("garment", "textile"),
    ("kinetic", "kinetic"), ("motion", "kinetic"), ("animation", "kinetic"), ("experimental", "kinetic"),
    ("film", "kinetic"), ("showreel", "kinetic"), ("reel", "kinetic"), ("music", "kinetic"), ("festival", "kinetic"),
    ("garden", "botanical"), ("botanical", "botanical"), ("plants", "botanical"), ("florist", "botanical"),
    ("flowers", "botanical"), ("herbs", "botanical"), ("organic", "botanical"), ("farm", "botanical"), ("tea", "botanical"),
    ("dashboard", "data"), ("operations", "data"), ("ops", "data"), ("analytics", "data"), ("admin", "data"),
    ("tracking", "data"), ("inventory", "data"), ("planning", "data"), ("logistics", "data"), ("scheduling", "data"),
    ("workshop", "industrial"), ("repair", "industrial"), ("bicycle", "industrial"), ("machine", "industrial"),
    ("factory", "industrial"), ("manufacturing", "industrial"), ("steel", "industrial"), ("engineering", "industrial"),
    ("bar", "night"), ("club", "night"), ("cocktail", "night"), ("nightlife", "night"), ("cinema", "night"),
    ("wine", "earth"), ("vineyard", "earth"), ("clay", "earth"), ("ceramics", "earth"), ("pottery", "earth"),
    ("terracotta", "earth"), ("leather", "earth"),
];

/// Expressive, high-contrast stage options for Direction B, tagged by niche family.
fn expressive_pool() -> Vec<(TypographyPairing, PaletteRecommendation, &'static [&'static str])> {
    vec![
        (
            pairing("syne-plus-jakarta-kinetic", "Syne", "Plus Jakarta Sans", "JetBrains Mono", "Kinetic, bold, flagship launch", "1.333 (Perfect Fourth)",
                "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&family=Plus+Jakarta+Sans:wght@400;500;600&family=Syne:wght@700;800&display=swap"),
            palette("stage-obsidian-coral", "#0E1014", "#181B22", "#F5F4F0", "#94A3B8", "#F97316", "#38BDF8", 16.4, "Deep obsidian stage with coral and sky accents"),
            &["kinetic", "night"],
        ),
        (
            pairing("unbounded-onest", "Unbounded", "Onest", "JetBrains Mono", "Wide, loud, poster-like", "1.414 (Augmented Fourth)",
                "https://fonts.googleapis.com/css2?family=Onest:wght@400;500;600&family=Unbounded:wght@600;800&family=JetBrains+Mono:wght@400&display=swap"),
            palette("cobalt-saturated-field", "#1B2FA8", "#2238C4", "#F7F7F2", "#C9CFF2", "#FFC83D", "#4FE3C1", 9.8, "Saturated cobalt colour field with sun-yellow and mint"),
            &["kinetic", "bold"],
        ),
        (
            pairing("dela-gothic-figtree", "Dela Gothic One", "Figtree", "Space Mono", "Heavy, graphic, street-poster energy", "1.333 (Perfect Fourth)",
                "https://fonts.googleapis.com/css2?family=Dela+Gothic+One&family=Figtree:wght@400;500;600&family=Space+Mono&display=swap"),
            palette("oxblood-bone", "#3A0D12", "#4A141A", "#F4EBDD", "#D9BFA8", "#FF6A3D", "#E9C46A", 13.9, "Oxblood stage with bone type and ember accents"),
            &["night", "earth", "bold"],
        ),
        (
            pairing("instrument-serif-inter-tight", "Instrument Serif", "Inter Tight", "IBM Plex Mono", "Cinematic, editorial contrast", "1.5 (Perfect Fifth)",
                "https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=Inter+Tight:wght@400;500;600&family=IBM+Plex+Mono&display=swap"),
            palette("forest-night-chartreuse", "#0F1F17", "#16291F", "#EEF3EC", "#9FB5A6", "#C7F25C", "#7FD1FF", 15.1, "Night-forest stage with chartreuse signal colour"),
            &["botanical", "night"],
        ),
        (
            pairing("dm-serif-work-sans", "DM Serif Display", "Work Sans", "DM Mono", "Salt-weathered, confident, maritime signage", "1.333 (Perfect Fourth)",
                "https://fonts.googleapis.com/css2?family=DM+Mono&family=DM+Serif+Display&family=Work+Sans:wght@400;500;600&display=swap"),
            palette("deep-harbour-buoy", "#0B2230", "#12303F", "#EEF4F2", "#9DB8C0", "#FF7A59", "#7FD6C2", 14.9, "Deep harbour water with buoy-orange and sea-glass accents"),
            &["marine", "night"],
        ),
        (
            pairing("young-serif-instrument-sans", "Young Serif", "Instrument Sans", "Space Mono", "Warm, hand-made, oven-lit", "1.250 (Major Third)",
                "https://fonts.googleapis.com/css2?family=Instrument+Sans:wght@400;500;600&family=Space+Mono&family=Young+Serif&display=swap"),
            palette("oven-crust-night", "#2A1A10", "#3A2617", "#F6EBDD", "#CDB8A0", "#F2A541", "#E86A4A", 14.6, "Dark oven interior with crust-gold and ember accents"),
            &["warm-grain", "earth"],
        ),
        (
            pairing("bodoni-moda-jost", "Bodoni Moda", "Jost", "DM Mono", "High-fashion, editorial, tailored", "1.5 (Perfect Fifth)",
                "https://fonts.googleapis.com/css2?family=Bodoni+Moda:ital,wght@0,500;0,700;1,500&family=DM+Mono&family=Jost:wght@400;500&display=swap"),
            palette("indigo-loom", "#1C2340", "#252D52", "#F3EFE6", "#B9BCD3", "#E8B04B", "#E4826B", 13.6, "Indigo-dyed cloth with saffron thread and madder accents"),
            &["textile"],
        ),
        (
            pairing("barlow-condensed-plex", "Barlow Condensed", "IBM Plex Sans", "IBM Plex Mono", "Control-room, legible at a glance", "1.200 (Minor Third)",
                "https://fonts.googleapis.com/css2?family=Barlow+Condensed:wght@600;700&family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@400;500;600&display=swap"),
            palette("ops-console-night", "#0D1117", "#161B22", "#E6EDF3", "#9DA7B3", "#3FB950", "#F0883E", 15.8, "Night console where state colour (ok / late / short) is the only decoration"),
            &["data", "industrial", "night"],
        ),
    ]
}

/// Utility-first, light, interaction-led options for Direction C, tagged by niche family.
fn utility_pool() -> Vec<(TypographyPairing, PaletteRecommendation, &'static [&'static str])> {
    vec![
        (
            pairing("bricolage-ibm-plex", "Bricolage Grotesque", "IBM Plex Sans", "IBM Plex Mono", "Engineered, interactive, utility-first", "1.250 (Major Third)",
                "https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:wght@600;700&family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@400;500;600&display=swap"),
            palette("workbench-mineral", "#F4F5F3", "#FFFFFF", "#111615", "#4A5552", "#0D9488", "#2563EB", 16.8, "Mineral neutral with teal and cobalt interactive states"),
            &["data"],
        ),
        (
            pairing("archivo-atkinson", "Archivo", "Atkinson Hyperlegible", "JetBrains Mono", "Plain-spoken, legible, practical", "1.200 (Minor Third)",
                "https://fonts.googleapis.com/css2?family=Archivo:wght@600;800&family=Atkinson+Hyperlegible:wght@400;700&family=JetBrains+Mono&display=swap"),
            palette("paper-signal-red", "#FBFAF7", "#FFFFFF", "#141414", "#555555", "#D7261E", "#1F5FBF", 18.1, "Paper white with signal red and ink blue"),
            &["industrial", "data", "bold"],
        ),
        (
            pairing("manrope-dm-mono", "Manrope", "Manrope", "DM Mono", "Calm, precise, product-like", "1.250 (Major Third)",
                "https://fonts.googleapis.com/css2?family=DM+Mono&family=Manrope:wght@400;500;700;800&display=swap"),
            palette("sage-graphite", "#EEF2EE", "#FFFFFF", "#1A1F1B", "#4D5A50", "#2F6B4F", "#B4542B", 15.4, "Sage daylight with graphite text and rust accents"),
            &["botanical", "calm"],
        ),
        (
            pairing("space-grotesk-public-sans", "Space Grotesk", "Public Sans", "Space Mono", "Technical, friendly, modular", "1.250 (Major Third)",
                "https://fonts.googleapis.com/css2?family=Public+Sans:wght@400;500;600&family=Space+Grotesk:wght@500;700&family=Space+Mono&display=swap"),
            palette("lilac-ink", "#F3F1FA", "#FFFFFF", "#17142B", "#4E4A66", "#5B3FD9", "#E0751F", 16.0, "Cool lilac paper with violet controls and orange highlights"),
            &["kinetic", "bold"],
        ),
        (
            pairing("libre-franklin-source-sans", "Libre Franklin", "Source Sans 3", "Source Code Pro", "Clear, nautical-chart precise", "1.250 (Major Third)",
                "https://fonts.googleapis.com/css2?family=Libre+Franklin:wght@600;800&family=Source+Code+Pro&family=Source+Sans+3:wght@400;500;600&display=swap"),
            palette("tide-chart-paper", "#F2F6F7", "#FFFFFF", "#0E2430", "#44606B", "#0B6E8A", "#D9572B", 15.6, "Tide-chart paper with deep-channel blue and signal-flag orange"),
            &["marine"],
        ),
        (
            pairing("zilla-slab-source-sans", "Zilla Slab", "Source Sans 3", "Source Code Pro", "Sturdy, ledger-like, kitchen-board", "1.200 (Minor Third)",
                "https://fonts.googleapis.com/css2?family=Source+Code+Pro&family=Source+Sans+3:wght@400;500;600&family=Zilla+Slab:wght@600;700&display=swap"),
            palette("rye-ledger", "#EFEFEA", "#FFFFFF", "#1E1A16", "#57504A", "#A0461C", "#3D6B4F", 16.2, "Flour-dusted grey ledger with rye-crust and herb-green accents"),
            &["warm-grain", "data", "earth"],
        ),
        (
            pairing("cormorant-hanken", "Cormorant Garamond", "Hanken Grotesk", "DM Mono", "Quiet, tactile, swatch-book", "1.333 (Perfect Fourth)",
                "https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,600;1,500&family=DM+Mono&family=Hanken+Grotesk:wght@400;500;600&display=swap"),
            palette("raw-linen-ink", "#E9E9E4", "#F7F7F4", "#191919", "#504E4B", "#8C3B2E", "#2F4A6D", 15.1, "Undyed linen grey with madder-red and indigo swatches"),
            &["textile", "calm"],
        ),
    ]
}

/// Picks the pool entry whose tags best match the brief; ties resolve by a stable per-brief hash.
/// Never reuses a display font or palette already chosen for another direction.
fn pick_from_pool(
    pool: Vec<(TypographyPairing, PaletteRecommendation, &'static [&'static str])>,
    signals: &BriefSignals,
    query: &str,
    salt: &str,
    taken_fonts: &[String],
    taken_palettes: &[String],
) -> (TypographyPairing, PaletteRecommendation) {
    let start = (fnv1a(&format!("{}{}", salt, query.to_lowercase())) % pool.len() as u64) as usize;
    let score = |tags: &[&str]| -> usize {
        tags.iter()
            .map(|t| {
                if *t == "data" && signals.surface == Surface::ProductUi {
                    // A task UI needs a data-legible palette before a niche-flavoured one
                    5
                } else if signals.niche_tags.iter().any(|n| n == t) {
                    3
                } else if signals.moods.iter().any(|m| m == t) {
                    1
                } else {
                    0
                }
            })
            .sum()
    };
    let mut best: Option<(usize, usize)> = None; // (score, index)
    for i in 0..pool.len() {
        let idx = (start + i) % pool.len();
        let (t, p, tags) = &pool[idx];
        if taken_fonts.contains(&t.display_font) || taken_palettes.contains(&p.id) {
            continue;
        }
        let s = score(tags);
        if best.map(|(bs, _)| s > bs).unwrap_or(true) {
            best = Some((s, idx));
        }
    }
    let idx = best.map(|(_, i)| i).unwrap_or(start);
    (pool[idx].0.clone(), pool[idx].1.clone())
}

/// Reads surface, niche families, moods and named constraints from the brief.
pub fn brief_signals(query: &str) -> BriefSignals {
    let tokens = tokenize(query);
    let has = |w: &str| keyword_hit(&tokens, w);
    let any = |ws: &[&str]| ws.iter().any(|w| has(w));

    let surface = if any(&["dashboard", "admin", "console", "portal", "backoffice", "back-office", "crm", "erp", "internal tool", "ops", "analytics"]) {
        Surface::ProductUi
    } else if any(&["store", "shop", "ecommerce", "e-commerce", "boutique", "catalog", "catalogue", "selling", "sells", "webshop"]) {
        Surface::Commerce
    } else if any(&["restaurant", "cafe", "bistro", "bar", "hotel", "menu", "reservations", "booking", "bakery", "venue", "brasserie", "diner"]) {
        Surface::Venue
    } else if any(&["portfolio", "studio", "agency", "showreel", "reel", "artist", "photographer", "designer"]) {
        Surface::Showcase
    } else if any(&["magazine", "journal", "blog", "publication", "newsletter", "news"]) {
        Surface::Editorial
    } else {
        Surface::General
    };

    let mut niche_tags: Vec<String> = Vec::new();
    for (word, tag) in NICHE_LEXICON {
        if has(word) && !niche_tags.iter().any(|t| t == tag) {
            niche_tags.push((*tag).to_string());
        }
    }

    let mut moods: Vec<String> = Vec::new();
    let mut motion_bias: i8 = 0;
    for (words, mood, bias) in [
        (&["calm", "quiet", "serene", "gentle", "slow", "minimal", "understated"][..], "calm", -2i8),
        (&["bold", "loud", "playful", "energetic", "experimental", "kinetic", "vibrant"][..], "bold", 2),
        (&["dark", "moody", "night"][..], "dark", 0),
        (&["light", "bright", "airy", "sunny"][..], "light", 0),
    ] {
        if any(words) {
            moods.push(mood.to_string());
            motion_bias += bias;
        }
    }

    let lower = query.to_lowercase();
    let mut constraints: Vec<String> = Vec::new();
    let mut motion_cap: Option<u8> = None;
    let mut cap = |c: u8| motion_cap = Some(motion_cap.map_or(c, |m: u8| m.min(c)));
    if ["no animation", "no motion", "without animation", "reduced motion", "reduced-motion", "static site"].iter().any(|p| lower.contains(p)) {
        constraints.push("reduced-motion".into());
        cap(1);
    }
    if any(&["accessible", "accessibility", "wcag", "elderly", "seniors"]) {
        constraints.push("accessibility-first".into());
        cap(5);
    }
    if any(&["fast", "lightweight", "performance", "low-bandwidth", "offline"]) {
        constraints.push("performance-budget".into());
        cap(5);
    }
    if surface == Surface::ProductUi {
        constraints.push("task-ui (no scroll choreography)".into());
        cap(4);
    }

    BriefSignals { surface, niche_tags, moods, constraints, motion_bias, motion_cap }
}

/// Applies the brief's motion bias and ceiling to a direction's motion dial.
fn tune_motion(base: u8, signals: &BriefSignals) -> u8 {
    let biased = (base as i16 + signals.motion_bias as i16).clamp(0, 10) as u8;
    signals.motion_cap.map_or(biased, |c| biased.min(c))
}

struct DirectionCopy {
    name: &'static str,
    style_family: &'static str,
    layout: &'static str,
    identity: &'static str,
    content: &'static str,
    motion: &'static str,
}

/// Direction B per surface: the high-contrast, narrative option.
fn direction_b_copy(surface: Surface) -> DirectionCopy {
    match surface {
        Surface::ProductUi => DirectionCopy {
            name: "Live Operations Wall",
            style_family: "high-density-workbench",
            layout: "Full-width live status wall: today's timeline (batches, shifts, orders) as the hero on a horizontal time axis, an exceptions rail pinned right, drill-down drawers instead of page changes",
            identity: "Dark control-room surface where state colour (on track / late / short / wasted) is the only decoration; condensed numerals, tabular figures, no illustration",
            content: "Exceptions first: what is late, short or wasted right now and who owns it, then the plan, then history",
            motion: "Data-driven only: 150-200ms value tweens on update and FLIP row insert/remove; no scroll-triggered choreography; reduced-motion swaps tweens for instant updates",
        },
        Surface::Commerce => DirectionCopy {
            name: "Lookbook Stage",
            style_family: "kinetic-editorial",
            layout: "Full-bleed lookbook: one hero product at scale with scroll-scrubbed detail crops (material, stitching, label), then a shoppable story sequence with a sticky size-and-add-to-bag rail",
            identity: "Product photography and the material itself carry the page; display type set like a printed lookbook caption, colour taken from the collection",
            content: "Story-led: the piece, how it is made, how it wears, then the buy decision with fit, fabric and care facts beside the button",
            motion: "Scroll-scrubbed image crops (ScrollTrigger or CSS scroll-driven animations) and a sticky purchase rail; reduced-motion shows static crops",
        },
        Surface::Venue => DirectionCopy {
            name: "Arrival Sequence",
            style_family: "kinetic-editorial",
            layout: "Cinematic arrival: the signature dish or room at full bleed, then today's menu revealed as a scrolling board/ticket strip, with the booking bar docked and usable at every scroll position",
            identity: "Atmosphere drawn from the place itself (water, light, the counter, the board), hand-set menu typography, prices in the menu not hidden in PDFs",
            content: "Place first, then what is on today (with prices and sold-out states), then book; opening hours and location as a ruled table",
            motion: "Slow 600-900ms reveals keyed to the menu sequence; the booking bar never animates away; reduced-motion keeps fades only",
        },
        Surface::Showcase | Surface::Editorial | Surface::General => DirectionCopy {
            name: "Kinetic Narrative & Choreographed Stage",
            style_family: "kinetic-editorial",
            layout: "Full-bleed choreographed stage with pinned ScrollTrigger narrative sequence, staggered asymmetric showcase, and magnetic CTA rail",
            identity: "High-drama contrast stage with expressive display typography, tactile depth planes, and bespoke SVG/3D focal points",
            content: "Story-driven sequential progression: Problem tension -> Interactive live artifact -> Quantified proof",
            motion: "GSAP timeline entrance + ScrollTrigger pinned sequence + pointer parallax with `gsap.matchMedia()` reduced-motion guard",
        },
    }
}

/// Direction C per surface: the interaction-led, utility option.
fn direction_c_copy(surface: Surface) -> DirectionCopy {
    match surface {
        Surface::ProductUi => DirectionCopy {
            name: "Task Workbench",
            style_family: "crisp-product-workbench",
            layout: "Split workbench: the editable planning table (keyboard navigable cells, bulk actions) on the left, a live consequence panel (yield, waste, cost, capacity) on the right, filters as a command bar",
            identity: "Light, quiet surface with one accent for actions and semantic colours for state; every control sized for repeated daily use",
            content: "Interactive proof: change a quantity or date and watch the plan and its consequences recalculate in place",
            motion: "120-180ms state transitions only; zero decorative motion",
        },
        Surface::Commerce => DirectionCopy {
            name: "Product Configurator",
            style_family: "crisp-product-workbench",
            layout: "Configurator hero: pick size, colour or material and see price, stock and delivery date update in place; then a filterable catalogue whose cards carry real product facts (material, fit, care)",
            identity: "Swatch-book vocabulary: real material swatches as controls, honest product photography, no generic pill buttons",
            content: "Decision support first: comparison, sizing help and returns policy beside the add-to-bag action",
            motion: "FLIP re-layout when filters change and 180ms swatch transitions; nothing moves on scroll",
        },
        Surface::Venue => DirectionCopy {
            name: "Booking-First Board",
            style_family: "crisp-product-workbench",
            layout: "Booking-first hero: date, party size and time slots working above the fold beside today's board (dishes, catch or bakes with prices), then location, hours and access as a ruled table",
            identity: "The venue's own board, ticket or label as the visual system; large legible type for phones at the door",
            content: "Utility first: can I get a table or order, what is on today, how do I get there",
            motion: "Slot-selection micro-interactions only (160ms); reduced-motion removes them",
        },
        Surface::Showcase => DirectionCopy {
            name: "Interactive Case Index",
            style_family: "crisp-product-workbench",
            layout: "Case-study index as an interactive table: filter by discipline, client and year; hover or focus previews a loop of the work; each case opens with process artefacts before the hero shot",
            identity: "Archive vocabulary: catalogue numbers, ruled rows, the work as the only colour",
            content: "Proof by browsing: let visitors sort the work themselves, then show process and credits",
            motion: "Preview loops on hover/focus (paused under reduced motion) and FLIP filter re-layout",
        },
        Surface::Editorial | Surface::General => DirectionCopy {
            name: "Tactile Product Workbench & Interactive Proof",
            style_family: "crisp-product-workbench",
            layout: "Interactive hero where the visitor operates the subject itself (try a frame, fit a part, set a date) immediately above the fold, then sections built from real product structure",
            identity: "Tactile, rebuilt controls in the world's own material (dials, tickets, swatches), one depth strategy, every atom (buttons, inputs, focus) redrawn in that vocabulary",
            content: "Interactive proof-first: let the user manipulate live tabs, filters, or calculators right inside the hero",
            motion: "State-driven spring transitions (180ms–280ms) and FLIP layout morphs on user interaction",
        },
    }
}

const MATERIAL_LEXICON: &[&str] = &[
    "clay", "ceramic", "ceramics", "stoneware", "porcelain", "terracotta", "glaze", "kiln", "tile", "tiles", "wood", "oak",
    "walnut", "teak", "linen", "cotton", "wool", "silk", "indigo", "leather", "brass", "copper", "bronze", "steel", "iron",
    "concrete", "stone", "marble", "terrazzo", "glass", "paper", "ink", "coffee", "tea", "chai", "spice", "spices", "saffron",
    "honey", "bread", "flour", "wine", "chocolate", "flowers", "herbs", "soap", "candle", "candles", "rug", "rugs", "textile",
    "fish", "oyster", "oysters", "seafood", "dough", "pastry", "sourdough", "loaves", "oven", "denim", "fabric", "harbour", "harbor",
];

const PLACE_STOPWORDS: &[&str] = &["the", "a", "an", "our", "your", "their", "this", "that", "one", "every", "each"];

/// Extracts place names (capitalised words after "in", "from", "based in", "near") and material words from the brief.
fn extract_cues(query: &str) -> (Vec<String>, Vec<String>) {
    let words: Vec<&str> = query.split_whitespace().collect();
    let mut places = Vec::new();
    for (i, w) in words.iter().enumerate() {
        let lw = w.to_lowercase();
        if !matches!(lw.as_str(), "in" | "from" | "near" | "across") {
            continue;
        }
        let mut name = Vec::new();
        for next in words.iter().skip(i + 1).take(3) {
            let clean: String = next.chars().filter(|c| c.is_alphanumeric() || *c == '-' || *c == '\'').collect();
            let starts_upper = clean.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);
            if !starts_upper || PLACE_STOPWORDS.contains(&clean.to_lowercase().as_str()) {
                break;
            }
            name.push(clean);
            if next.ends_with(',') || next.ends_with('.') {
                break;
            }
        }
        if !name.is_empty() {
            let place = name.join(" ");
            if !places.contains(&place) {
                places.push(place);
            }
        }
    }
    let tokens = tokenize(query);
    let mut materials: Vec<String> = Vec::new();
    for m in MATERIAL_LEXICON {
        if tokens.iter().any(|t| t == m) && !materials.iter().any(|x| x == m) {
            materials.push((*m).to_string());
        }
    }
    (places, materials)
}

/// True when a direction's fonts/palette reproduce a cliché listed for its own category.
pub fn violates_own_cliches(slug: &str, typography: &TypographyPairing, palette: &PaletteRecommendation) -> Option<String> {
    let cliches = category_cliches(slug).join(" ").to_lowercase();
    for font in ["fraunces", "playfair"] {
        if cliches.contains(font) && typography.display_font.to_lowercase().contains(font) {
            return Some(format!("display font {} is listed as a cliché for {}", typography.display_font, slug));
        }
    }
    if cliches.contains("cream") {
        let h = palette.background.trim_start_matches('#');
        if let (Ok(r), Ok(b)) = (u8::from_str_radix(&h[0..2], 16), u8::from_str_radix(&h[4..6], 16)) {
            // Warm near-white: red channel clearly above blue
            if r as i32 > 235 && r as i32 - b as i32 >= 6 {
                return Some(format!("background {} is a cream cliché for {}", palette.background, slug));
            }
        }
    }
    None
}

/// The default look generators fall into for each category.
fn category_cliches(slug: &str) -> Vec<String> {
    let list: &[&str] = match slug {
        "artisan-craft" | "local-business" => &[
            "Cream/oatmeal background with a terracotta accent",
            "Italic high-contrast serif headline (Fraunces / Playfair italic) with one coloured italic word",
            "Arched image frame, rotating circular text badge, process-word marquee strip",
            "Giant faint wordmark in the footer",
        ],
        "saas-startup" | "ai-devtools" => &[
            "Purple-to-blue gradient hero with glowing orbs",
            "Bento grid of feature cards with icons",
            "Logo cloud + three-tier pricing table straight from a template",
            "Dark mode with neon accent and terminal mockup by default",
        ],
        "luxury-premium" | "creative-portfolio" => &[
            "Black-and-white with a single gold accent",
            "Huge thin serif + letter-spaced caps everywhere",
            "Full-screen hero video with centred logo",
        ],
        "hospitality-travel" => &[
            "Full-bleed sunset hero photo with centred serif headline",
            "Beige palette with gold accent and script font",
        ],
        "ecommerce-catalog" => &[
            "White grid of identical product cards with a pill 'Add to cart'",
            "Announcement bar + generic hero carousel",
        ],
        _ => &[
            "Template hero (headline left, illustration right, two buttons)",
            "Uniform card grids with icon + heading + paragraph",
            "Gradient blobs and glassmorphism with no brand reason",
        ],
    };
    list.iter().map(|s| s.to_string()).collect()
}

fn distinctiveness_for(query: &str, slug: &str) -> DistinctivenessBrief {
    let (place_cues, material_cues) = extract_cues(query);
    let mut instruction = String::from(
        "Subject first: list the objects, tools and moments a visitor actually sees in this business (a bakery: loaves, the oven, flour; an optician: frames, lenses, the test chart). The first screen must show the most characteristic of them as authored art at scale, so a stranger can name the business with the text covered. ",
    );
    instruction.push_str(
        "Then write down 3 concrete references from the brand's own world (local architecture, craft traditions, signage, landscape, the material itself) and derive palette, pattern, and type voice from them. ",
    );
    if !place_cues.is_empty() {
        instruction.push_str(&format!("Place: {} — what does it look like there, specifically? ", place_cues.join(", ")));
    }
    if !material_cues.is_empty() {
        instruction.push_str(&format!("Materials: {} — sample real colours and textures from them. ", material_cues.join(", ")));
    }
    instruction.push_str("The clichés below are generic *treatments* (layouts, kits, palettes), not the subject's own objects: avoid the treatments, never the subject.");
    DistinctivenessBrief {
        place_cues,
        material_cues,
        category_cliches: category_cliches(slug),
        instruction,
    }
}

/// Returns all 19 core industry rules with rich taxonomy and provenance.
pub fn catalog_industries() -> Vec<IndustryRule> {
    let mut rules = raw_industries();
    for r in &mut rules {
        verify_palette(&mut r.palette);
    }
    rules
}

fn raw_industries() -> Vec<IndustryRule> {
    vec![
        IndustryRule {
            slug: "corporate-enterprise".into(),
            name: "Corporate & Enterprise".into(),
            keywords: vec!["corporate".into(), "enterprise".into(), "consulting".into(), "global".into(), "governance".into(), "compliance".into()],
            recommended_styles: vec!["swiss-editorial-grid".into(), "block-first-editorial".into()],
            layout_archetype: "Asymmetric Editorial Authority Grid with Proof Metrics Rail".into(),
            first_viewport_priority: "Clear institutional value proposition, enterprise proof metrics, and direct briefing/contact CTA".into(),
            typography_pairing: TypographyPairing {
                id: "public-sans-instrument".into(),
                display_font: "Instrument Serif".into(),
                body_font: "Public Sans".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "Authoritative, architectural, trustworthy".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Instrument+Serif|Public+Sans:wght@400;500;600;700".into(),
            },
            palette: PaletteRecommendation {
                id: "limestone-slate".into(),
                background: "#F7F6F2".into(),
                surface: "#EFECE6".into(),
                foreground: "#14181D".into(),
                muted: "#4F5864".into(),
                primary_accent: "#0F3D3E".into(),
                secondary_accent: "#B84A27".into(),
                wcag_body_contrast: 15.2,
                notes: "Warm limestone canvas with deep mineral slate ink and high-trust emerald/terracotta accents".into(),
            },
            conversion_trust_elements: vec!["SOC2/ISO compliance ledger".into(), "Named customer case metrics".into(), "Architecture topology diagram".into()],
            motion_language: "Restrained architectural reveals (240ms cubic-bezier(0.22, 1, 0.36, 1)), tabular counter ticks".into(),
            anti_patterns: vec!["Novelty neon gradients".into(), "Playful bounce easings".into(), "Unlabeled mystery-meat navigation".into()],
            default_dials: DesignDials::clamped(3, 3, 6),
            provenance: "adapted:ui-ux-pro-max+awesome-design-skills+layoza".into(),
        },
        IndustryRule {
            slug: "marketing-agency".into(),
            name: "Marketing & Creative Advertising Agencies".into(),
            keywords: vec!["agency".into(), "studio".into(), "marketing".into(), "advertising".into(), "branding".into(), "creative".into()],
            recommended_styles: vec!["kinetic-editorial".into(), "swiss-editorial-grid".into()],
            layout_archetype: "Full-Bleed Kinetic Reel + Staggered Case Study Index".into(),
            first_viewport_priority: "Immediate signature craft statement, flagship client work reel, and distinctive typographic voice".into(),
            typography_pairing: TypographyPairing {
                id: "syne-plus-jakarta".into(),
                display_font: "Syne".into(),
                body_font: "Plus Jakarta Sans".into(),
                mono_font: "Space Mono".into(),
                mood: "Expressive, kinetic, gallery-grade".into(),
                scale_ratio: "1.333 (Perfect Fourth)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Plus+Jakarta+Sans:wght@400;500;600|Syne:wght@600;700;800".into(),
            },
            palette: PaletteRecommendation {
                id: "obsidian-vermilion".into(),
                background: "#0D0E11".into(),
                surface: "#17191E".into(),
                foreground: "#F5F3EF".into(),
                muted: "#9CA3AF".into(),
                primary_accent: "#FF4D2D".into(),
                secondary_accent: "#E2F163".into(),
                wcag_body_contrast: 16.4,
                notes: "Cinema-dark obsidian stage with vermilion kinetic focal points".into(),
            },
            conversion_trust_elements: vec!["Interactive case study outcomes".into(), "Client roster with shipped artifacts".into(), "Direct scope estimator".into()],
            motion_language: "GSAP SplitText headline reveals, pinned ScrollTrigger case study scrubbers, magnetic CTAs".into(),
            anti_patterns: vec!["Generic 3-column SaaS feature cards".into(), "Stock corporate handshake photos".into()],
            default_dials: DesignDials::clamped(8, 8, 4),
            provenance: "adapted:taste-skill+ui-ux-pro-max+layoza".into(),
        },
        IndustryRule {
            slug: "luxury-premium".into(),
            name: "Luxury & Premium Brands".into(),
            keywords: vec!["luxury".into(), "fashion".into(), "watch".into(), "jewelry".into(), "atelier".into(), "bespoke".into(), "fragrance".into()],
            recommended_styles: vec!["atelier-minimalism".into(), "kinetic-editorial".into()],
            layout_archetype: "High-Whitespace Lookbook Canvas with Asymmetric Monograph Captions".into(),
            first_viewport_priority: "Tactile hero photography/3D object, restrained serif wordmark, unhurried spatial pacing".into(),
            typography_pairing: TypographyPairing {
                id: "playfair-plus-jakarta".into(),
                display_font: "Playfair Display".into(),
                body_font: "Plus Jakarta Sans".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "High-contrast editorial authority, optically balanced italic & roman x-height".into(),
                scale_ratio: "1.333 (Perfect Fourth)".into(),
                google_fonts_url: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&family=Playfair+Display:ital,wght@0,600;0,700;0,800;1,600&family=Plus+Jakarta+Sans:wght@400;500;600;700&display=swap".into(),
            },
            palette: PaletteRecommendation {
                id: "alabaster-espresso".into(),
                background: "#FAF8F5".into(),
                surface: "#F1ECE4".into(),
                foreground: "#1C1917".into(),
                muted: "#6B635B".into(),
                primary_accent: "#8C6239".into(),
                secondary_accent: "#2C3531".into(),
                wcag_body_contrast: 15.8,
                notes: "Warm alabaster paper with deep espresso ink and burnished bronze".into(),
            },
            conversion_trust_elements: vec!["Provenance & craftsmanship narrative".into(), "Material specification sheet".into(), "Private concierge appointment".into()],
            motion_language: "Long-duration curtain image reveals (750ms), subtle parallax depth, zero jarring popups".into(),
            anti_patterns: vec!["Discount countdown timers".into(), "Crowded badge grids".into(), "Harsh saturated blue buttons".into()],
            default_dials: DesignDials::clamped(6, 6, 2),
            provenance: "adapted:ui-ux-pro-max+impeccable+layoza".into(),
        },
        IndustryRule {
            slug: "saas-startup".into(),
            name: "SaaS & Modern Startups".into(),
            keywords: vec!["saas".into(), "startup".into(), "workflow".into(), "productivity".into(), "platform".into(), "b2b".into(), "collaboration".into()],
            recommended_styles: vec!["crisp-product-workbench".into(), "swiss-editorial-grid".into()],
            layout_archetype: "Interactive Product Stage Hero + Workflow Proof Sequence".into(),
            first_viewport_priority: "Concrete product outcome headline, live interactive UI preview, and friction-free primary CTA".into(),
            typography_pairing: TypographyPairing {
                id: "bricolage-inter-tight".into(),
                display_font: "Bricolage Grotesque".into(),
                body_font: "Plus Jakarta Sans".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "Crisp, engineered, approachable".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Bricolage+Grotesque:wght@600;700|Plus+Jakarta+Sans:wght@400;500;600".into(),
            },
            palette: PaletteRecommendation {
                id: "paper-cobalt".into(),
                background: "#FCFCFA".into(),
                surface: "#F2F3EE".into(),
                foreground: "#11151C".into(),
                muted: "#525B6C".into(),
                primary_accent: "#1D4ED8".into(),
                secondary_accent: "#0D9488".into(),
                wcag_body_contrast: 16.9,
                notes: "Engineered warm paper surface with high-contrast ink and cobalt interactive states".into(),
            },
            conversion_trust_elements: vec!["Interactive product state switcher".into(), "Transparent pricing matrix".into(), "Quantified customer time-saved proof".into()],
            motion_language: "State-driven UI micro-interactions, spring layout transitions, choreographed feature tabs".into(),
            anti_patterns: vec!["Predictable AI purple-to-pink glowing blobs".into(), "Fake empty decorative cards".into()],
            default_dials: DesignDials::clamped(5, 6, 5),
            provenance: "adapted:ui-ux-pro-max+taste-skill+layoza".into(),
        },
        IndustryRule {
            slug: "ai-devtools".into(),
            name: "AI Systems & Developer Tools".into(),
            keywords: vec!["ai".into(), "devtools".into(), "cli".into(), "sdk".into(), "compiler".into(), "api".into(), "infrastructure".into(), "rust".into(), "agent".into()],
            recommended_styles: vec!["precision-terminal-craft".into(), "crisp-product-workbench".into()],
            layout_archetype: "Split Command/Code Proof Hero + Benchmark Telemetry Ledger".into(),
            first_viewport_priority: "Copyable install command (`cargo install` / `npx`), live architecture/code telemetry, benchmark latency proof".into(),
            typography_pairing: TypographyPairing {
                id: "space-grotesk-jetbrains".into(),
                display_font: "Space Grotesk".into(),
                body_font: "IBM Plex Sans".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "Technical, deterministic, high-precision".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=IBM+Plex+Sans:wght@400;500;600|JetBrains+Mono:wght@400;500|Space+Grotesk:wght@600;700".into(),
            },
            palette: PaletteRecommendation {
                id: "carbon-amber-phosphor".into(),
                background: "#0B0D10".into(),
                surface: "#13171D".into(),
                foreground: "#ECEFF4".into(),
                muted: "#8B95A5".into(),
                primary_accent: "#F59E0B".into(),
                secondary_accent: "#10B981".into(),
                wcag_body_contrast: 15.9,
                notes: "Low-glare carbon slate with amber hardware telemetry and emerald pass indicators".into(),
            },
            conversion_trust_elements: vec!["One-click CLI install snippet".into(), "Reproducible p99 benchmark table".into(), "Live interactive API/MCP inspector".into()],
            motion_language: "Deterministic step-by-step terminal trace playback, SVG data-path pulses, zero decorative bloat".into(),
            anti_patterns: vec!["Cliché purple sci-fi brain illustrations".into(), "Unreadable low-contrast dark-gray code comments".into()],
            default_dials: DesignDials::clamped(5, 5, 7),
            provenance: "adapted:ui-ux-pro-max+impeccable+layoza".into(),
        },
        IndustryRule {
            slug: "ecommerce-catalog".into(),
            name: "E-Commerce & Product Catalogs".into(),
            keywords: vec!["ecommerce".into(), "shop".into(), "store".into(), "retail".into(), "catalog".into(), "product".into(), "checkout".into()],
            recommended_styles: vec!["crisp-product-workbench".into(), "atelier-minimalism".into()],
            layout_archetype: "Merchandising Hero + High-Scannability Filterable Product Grid".into(),
            first_viewport_priority: "Hero product imagery, clear price/availability cues, fast category navigation".into(),
            typography_pairing: TypographyPairing {
                id: "fraunces-work-sans".into(),
                display_font: "Fraunces".into(),
                body_font: "Work Sans".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "Tactile, conversion-focused, welcoming".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Fraunces:opsz,wght@9..144,600|Work+Sans:wght@400;500;600".into(),
            },
            palette: PaletteRecommendation {
                id: "warm-canvas-terracotta".into(),
                background: "#FAF8F5".into(),
                surface: "#F0ECE3".into(),
                foreground: "#18181B".into(),
                muted: "#52525B".into(),
                primary_accent: "#C2410C".into(),
                secondary_accent: "#15803D".into(),
                wcag_body_contrast: 16.1,
                notes: "Neutral daylight backdrop that never distorts product photography colors".into(),
            },
            conversion_trust_elements: vec!["Shipping & return guarantee rail".into(), "Verified buyer review breakdown".into(), "Accessible variant swatches".into()],
            motion_language: "Quick 180ms image hover zoom, tactile cart drawer slide-over, zero layout shift (CLS = 0)".into(),
            anti_patterns: vec!["Blocking newsletter popups on initial load".into(), "Truncated product titles".into()],
            default_dials: DesignDials::clamped(4, 4, 6),
            provenance: "adapted:ui-ux-pro-max+layoza".into(),
        },
        IndustryRule {
            slug: "finance-fintech".into(),
            name: "Finance, Banking & Fintech".into(),
            keywords: vec!["finance".into(), "fintech".into(), "banking".into(), "payments".into(), "treasury".into(), "accounting".into(), "wealth".into()],
            recommended_styles: vec!["swiss-editorial-grid".into(), "high-density-workbench".into()],
            layout_archetype: "High-Trust Fiduciary Hero + Live Ledger & Security Assurance Grid".into(),
            first_viewport_priority: "Regulatory trust posture, exact financial utility, and tabular numeric clarity".into(),
            typography_pairing: TypographyPairing {
                id: "newsreader-inter".into(),
                display_font: "Newsreader".into(),
                body_font: "Public Sans".into(),
                mono_font: "IBM Plex Mono".into(),
                mood: "Fiduciary, exact, institutional".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=IBM+Plex+Mono:wght@500|Newsreader:opsz,wght@6..72,600|Public+Sans:wght@400;500;600".into(),
            },
            palette: PaletteRecommendation {
                id: "vault-emerald".into(),
                background: "#F6F8F7".into(),
                surface: "#E9EFECE0".into(),
                foreground: "#0C1713".into(),
                muted: "#43544D".into(),
                primary_accent: "#065F46".into(),
                secondary_accent: "#B45309".into(),
                wcag_body_contrast: 16.5,
                notes: "Institutional British racing green and crisp ledger paper with `tabular-nums` alignment".into(),
            },
            conversion_trust_elements: vec!["FDIC/PCI-DSS security disclosures".into(), "Interactive fee/yield calculator".into(), "Audit trail preview".into()],
            motion_language: "Subtle `tabular-nums` balance transitions, crisp chart crosshairs".into(),
            anti_patterns: vec!["Neon crypto casino aesthetics".into(), "Proportional numerals in financial tables".into()],
            default_dials: DesignDials::clamped(3, 4, 7),
            provenance: "adapted:ui-ux-pro-max+impeccable+layoza".into(),
        },
        IndustryRule {
            slug: "healthcare".into(),
            name: "Healthcare, Clinical & Wellness".into(),
            keywords: vec!["healthcare".into(), "medical".into(), "clinic".into(), "hospital".into(), "patient".into(), "patients".into(), "telehealth".into(), "dental".into(), "dentist".into(), "optician".into(), "opticians".into(), "optometrist".into(), "optometrists".into(), "eyecare".into(), "nhs".into(), "pharmacy".into(), "physiotherapy".into(), "physio".into()],
            recommended_styles: vec!["crisp-product-workbench".into(), "swiss-editorial-grid".into()],
            layout_archetype: "Patient-First Action Hero + Care Pathway & Provider Directory".into(),
            first_viewport_priority: "Immediate appointment/triage action, insurance/care clarity, WCAG AAA-leaning readability".into(),
            typography_pairing: TypographyPairing {
                id: "atkinson-lexend".into(),
                display_font: "Lexend".into(),
                body_font: "Atkinson Hyperlegible".into(),
                mono_font: "IBM Plex Mono".into(),
                mood: "Calm, hyper-legible, empathetic".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Atkinson+Hyperlegible:wght@400;700|Lexend:wght@500;600".into(),
            },
            palette: PaletteRecommendation {
                id: "sanctuary-teal".into(),
                background: "#F8FAFC".into(),
                surface: "#EFF6FF".into(),
                foreground: "#0F172A".into(),
                muted: "#334155".into(),
                primary_accent: "#0F766E".into(),
                secondary_accent: "#1D4ED8".into(),
                wcag_body_contrast: 17.1,
                notes: "Calming clinical teal and slate with high-legibility contrast".into(),
            },
            conversion_trust_elements: vec!["Clinician registration shown the local way (e.g. GOC / NHS in the UK, state licence in the US)".into(), "Plain-language privacy note for health data".into(), "Accessible booking flow".into()],
            motion_language: "Gentle opacity/height transitions (200ms); zero vestibular-triggering parallax".into(),
            anti_patterns: vec!["Low-contrast pastel text".into(), "Complex scroll-jacking".into()],
            default_dials: DesignDials::clamped(2, 2, 5),
            provenance: "adapted:ui-ux-pro-max+layoza".into(),
        },
        IndustryRule {
            slug: "education".into(),
            name: "Education, EdTech & Academic Platforms".into(),
            keywords: vec!["education".into(), "course".into(), "university".into(), "learning".into(), "academy".into(), "curriculum".into()],
            recommended_styles: vec!["crisp-product-workbench".into(), "block-first-editorial".into()],
            layout_archetype: "Curriculum Explorer Hero + Interactive Lesson Preview & Outcomes".into(),
            first_viewport_priority: "Learning outcomes, interactive syllabus/module preview, clear enrollment path".into(),
            typography_pairing: TypographyPairing {
                id: "lora-source-sans".into(),
                display_font: "Lora".into(),
                body_font: "Source Sans 3".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "Scholarly, clear, encouraging".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Lora:wght@600;700|Source+Sans+3:wght@400;600".into(),
            },
            palette: PaletteRecommendation {
                id: "oxford-ochre".into(),
                background: "#FCFBF7".into(),
                surface: "#F3EFE6".into(),
                foreground: "#172554".into(),
                muted: "#475569".into(),
                primary_accent: "#1E40AF".into(),
                secondary_accent: "#D97706".into(),
                wcag_body_contrast: 14.9,
                notes: "Warm book-paper background with Oxford navy and ochre highlights".into(),
            },
            conversion_trust_elements: vec!["Detailed module syllabus".into(), "Accreditation & instructor bios".into(), "Student project showcase".into()],
            motion_language: "Progressive disclosure accordions, interactive progress indicators".into(),
            anti_patterns: vec!["Cluttered gamification popups obscuring reading flow".into()],
            default_dials: DesignDials::clamped(4, 4, 6),
            provenance: "adapted:ui-ux-pro-max+layoza".into(),
        },
        IndustryRule {
            slug: "hospitality-travel".into(),
            name: "Hospitality, Hotels, Restaurants & Travel".into(),
            keywords: vec!["hotel".into(), "resort".into(), "travel".into(), "restaurant".into(), "hospitality".into(), "booking".into(), "dining".into()],
            recommended_styles: vec!["atelier-minimalism".into(), "kinetic-editorial".into()],
            layout_archetype: "Atmospheric Destination Hero + Tactile Booking Bar & Curated Experiences".into(),
            first_viewport_priority: "Immersive sense of place, frictionless date/guest availability bar, signature experience".into(),
            typography_pairing: TypographyPairing {
                id: "playfair-karla".into(),
                display_font: "Playfair Display".into(),
                body_font: "Karla".into(),
                mono_font: "Space Mono".into(),
                mood: "Warm, atmospheric, inviting".into(),
                scale_ratio: "1.333 (Perfect Fourth)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Karla:wght@400;500;600|Playfair+Display:ital,wght@0,600;1,400".into(),
            },
            palette: PaletteRecommendation {
                id: "terracotta-olive".into(),
                background: "#FBF9F5".into(),
                surface: "#F2ECE1".into(),
                foreground: "#231F20".into(),
                muted: "#5F5850".into(),
                primary_accent: "#9A3412".into(),
                secondary_accent: "#3F6212".into(),
                wcag_body_contrast: 15.5,
                notes: "Sunlit linen, terracotta clay, and Mediterranean olive tones".into(),
            },
            conversion_trust_elements: vec!["Direct booking rate guarantee".into(), "Location & transit map".into(), "Seasonal menu / room gallery".into()],
            motion_language: "Smooth gallery crossfades, gentle scroll-linked image framing".into(),
            anti_patterns: vec!["Cold sterile SaaS blue palettes".into(), "Hidden address/hours".into()],
            default_dials: DesignDials::clamped(6, 6, 3),
            provenance: "adapted:ui-ux-pro-max+layoza".into(),
        },
        IndustryRule {
            slug: "industrial-b2b".into(),
            name: "Industrial, Manufacturing, Logistics & Energy".into(),
            keywords: vec!["industrial".into(), "manufacturing".into(), "logistics".into(), "robotics".into(), "energy".into(), "hardware".into(), "supply-chain".into()],
            recommended_styles: vec!["precision-terminal-craft".into(), "swiss-editorial-grid".into()],
            layout_archetype: "Technical Specification Grid + Telemetry & Capability Matrix".into(),
            first_viewport_priority: "Operational scale, technical tolerances/specs, RFQ/engineering contact path".into(),
            typography_pairing: TypographyPairing {
                id: "barlow-ibm-plex".into(),
                display_font: "Barlow Condensed".into(),
                body_font: "IBM Plex Sans".into(),
                mono_font: "IBM Plex Mono".into(),
                mood: "Engineered, rugged, exact".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Barlow+Condensed:wght@600;700|IBM+Plex+Mono:wght@400;500|IBM+Plex+Sans:wght@400;500;600".into(),
            },
            palette: PaletteRecommendation {
                id: "steel-safety-orange".into(),
                background: "#F4F5F6".into(),
                surface: "#E5E7EB".into(),
                foreground: "#111827".into(),
                muted: "#4B5563".into(),
                primary_accent: "#EA580C".into(),
                secondary_accent: "#0284C7".into(),
                wcag_body_contrast: 16.2,
                notes: "Machined steel neutral grid with high-visibility safety orange callouts".into(),
            },
            conversion_trust_elements: vec!["ISO/ASTM certification table".into(), "Downloadable CAD/spec sheets".into(), "Facility capacity metrics".into()],
            motion_language: "Mechanical blueprint line draws, exploded 3D/SVG assembly views".into(),
            anti_patterns: vec!["Soft pastel claymorphism".into(), "Vague marketing fluff without specs".into()],
            default_dials: DesignDials::clamped(4, 5, 7),
            provenance: "adapted:ui-ux-pro-max+taste-skill+layoza".into(),
        },
        IndustryRule {
            slug: "creative-portfolio".into(),
            name: "Creative Portfolios & Independent Makers".into(),
            keywords: vec!["portfolio".into(), "designer".into(), "architect".into(), "director".into(), "photographer".into(), "personal".into()],
            recommended_styles: vec!["kinetic-editorial".into(), "spatial-webgl-stage".into()],
            layout_archetype: "Curated Work Index + Deep Case Study Monograph".into(),
            first_viewport_priority: "Clear practitioner identity, selected shipped works, and availability status".into(),
            typography_pairing: TypographyPairing {
                id: "instrument-serif-geist".into(),
                display_font: "Instrument Serif".into(),
                body_font: "Inter".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "Personal, curated, craft-led".into(),
                scale_ratio: "1.333 (Perfect Fourth)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Instrument+Serif:ital@0;1|Inter:wght@400;500;600".into(),
            },
            palette: PaletteRecommendation {
                id: "gallery-stone".into(),
                background: "#F6F5F0".into(),
                surface: "#EBE9E0".into(),
                foreground: "#161615".into(),
                muted: "#575653".into(),
                primary_accent: "#2563EB".into(),
                secondary_accent: "#DC2626".into(),
                wcag_body_contrast: 16.0,
                notes: "Archival gallery stone that lets project artifacts command attention".into(),
            },
            conversion_trust_elements: vec!["Role, timeline, and shipped impact per project".into(), "Direct email copy interaction".into()],
            motion_language: "Hover-linked project preview planes, FLIP route continuity".into(),
            anti_patterns: vec!["Unverified skill percentage bars (e.g. 'HTML 95%')".into()],
            default_dials: DesignDials::clamped(7, 7, 3),
            provenance: "adapted:taste-skill+impeccable+layoza".into(),
        },
        IndustryRule {
            slug: "editorial-publishing".into(),
            name: "Editorial, Journalism & Longform Publishing".into(),
            keywords: vec!["editorial".into(), "magazine".into(), "news".into(), "blog".into(), "publishing".into(), "journal".into(), "essay".into()],
            recommended_styles: vec!["block-first-editorial".into(), "swiss-editorial-grid".into()],
            layout_archetype: "Multi-Column Broadsheet Grid + Longform Reading Measure (65ch)".into(),
            first_viewport_priority: "Lead story hierarchy, author/date metadata, effortless reading comfort".into(),
            typography_pairing: TypographyPairing {
                id: "newsreader-franklin".into(),
                display_font: "Newsreader".into(),
                body_font: "Libre Franklin".into(),
                mono_font: "IBM Plex Mono".into(),
                mood: "Journalistic, literary, authoritative".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Libre+Franklin:wght@400;500;600|Newsreader:ital,opsz,wght@0,6..72,500;0,6..72,700;1,6..72,400".into(),
            },
            palette: PaletteRecommendation {
                id: "broadsheet-ink".into(),
                background: "#F9F7F1".into(),
                surface: "#EFECE4".into(),
                foreground: "#121212".into(),
                muted: "#525252".into(),
                primary_accent: "#991B1B".into(),
                secondary_accent: "#1E3A8A".into(),
                wcag_body_contrast: 17.4,
                notes: "Warm newsprint background with deep press ink and editorial crimson rules".into(),
            },
            conversion_trust_elements: vec!["Author bylines & citations".into(), "Reading time & table of contents".into(), "Clean subscriber box".into()],
            motion_language: "Reading progress hairline, smooth footnote popovers, minimal distraction".into(),
            anti_patterns: vec!["Line lengths exceeding 75ch".into(), "Auto-playing video overlays".into()],
            default_dials: DesignDials::clamped(5, 2, 6),
            provenance: "adapted:google-design-md+ui-ux-pro-max+layoza".into(),
        },
        IndustryRule {
            slug: "dashboard-admin".into(),
            name: "Dashboards, Analytics & Admin Software".into(),
            keywords: vec!["dashboard".into(), "admin".into(), "analytics".into(), "crm".into(), "erp".into(), "monitoring".into(), "observability".into(), "table".into()],
            recommended_styles: vec!["high-density-workbench".into()],
            layout_archetype: "Collapsible Command Sidebar + KPI Strip + Filterable Data Table & Chart Canvas".into(),
            first_viewport_priority: "Actionable status summary, anomaly highlights, keyboard-navigable filter/command bar".into(),
            typography_pairing: TypographyPairing {
                id: "inter-jetbrains-tabular".into(),
                display_font: "Plus Jakarta Sans".into(),
                body_font: "Inter".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "Functional, high-density, scannable".into(),
                scale_ratio: "1.125 (Major Second)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Inter:wght@400;500;600|JetBrains+Mono:wght@400;500|Plus+Jakarta+Sans:wght@600".into(),
            },
            palette: PaletteRecommendation {
                id: "workbench-slate".into(),
                background: "#F8FAFC".into(),
                surface: "#FFFFFF".into(),
                foreground: "#0F172A".into(),
                muted: "#475569".into(),
                primary_accent: "#2563EB".into(),
                secondary_accent: "#059669".into(),
                wcag_body_contrast: 17.5,
                notes: "High-scannability semantic tokens with `font-variant-numeric: tabular-nums`".into(),
            },
            conversion_trust_elements: vec!["Live filter & sort controls".into(), "Export CSV/JSON actions".into(), "Clear empty, loading, and error states".into()],
            motion_language: "Fast 120ms state transitions, skeleton shimmer loaders, zero decorative scroll animations".into(),
            anti_patterns: vec!["Oversized marketing hero inside an authenticated dashboard".into(), "Giant padding that forces horizontal scrolling".into()],
            default_dials: DesignDials::clamped(2, 2, 9),
            provenance: "adapted:impeccable+ui-ux-pro-max+layoza".into(),
        },
        IndustryRule {
            slug: "artisan-craft".into(),
            name: "Artisan, Craft & Maker Commerce".into(),
            keywords: vec![
                "ceramics".into(), "ceramic".into(), "pottery".into(), "potter".into(), "handmade".into(), "craft".into(),
                "maker".into(), "artisan".into(), "kiln".into(), "glaze".into(), "stoneware".into(), "tableware".into(),
                "woodwork".into(), "weaving".into(), "textile".into(), "candles".into(), "leatherwork".into(), "small-batch".into(),
                "workshop".into(), "workshops".into(),
            ],
            recommended_styles: vec!["atelier-minimalism".into(), "swiss-editorial-grid".into()],
            layout_archetype: "Object-led storefront: the made thing and the making process share the first viewport, then collection, classes/commissions, and maker story".into(),
            first_viewport_priority: "The actual object (drawn or photographed), who makes it and where, and one buy action plus one visit/book action".into(),
            typography_pairing: TypographyPairing {
                id: "bricolage-hanken".into(),
                display_font: "Bricolage Grotesque".into(),
                body_font: "Hanken Grotesk".into(),
                mono_font: "IBM Plex Mono".into(),
                mood: "Hand-made but confident; avoids the default italic-serif craft voice".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:opsz,wght@12..96,600;12..96,800&family=Hanken+Grotesk:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap".into(),
            },
            palette: PaletteRecommendation {
                id: "material-derived".into(),
                background: "#EDF1F0".into(),
                surface: "#FFFFFF".into(),
                foreground: "#16181D".into(),
                muted: "#4B5059".into(),
                primary_accent: "#1F4E9A".into(),
                secondary_accent: "#C8891E".into(),
                wcag_body_contrast: 16.2,
                notes: "Starting point only: replace accents with colours sampled from the maker's own materials, glazes, dyes, or place (see `distinctiveness`)".into(),
            },
            conversion_trust_elements: vec![
                "Maker name, place, and process shown, not claimed".into(),
                "Honest batch size, lead time, and care/food-safety notes".into(),
                "Real seat/stock counts for classes and limited runs".into(),
            ],
            motion_language: "Slow, material motion (a turning wheel, a glaze sheen) used once as a signature; everything else quiet and fast".into(),
            anti_patterns: vec![
                "Cream background + terracotta accent + italic serif headline (the default 'craft' look every generator produces)".into(),
                "Stock-photo hands-on-clay hero with no actual product".into(),
                "Rotating circular text badge and process-word marquee as filler".into(),
            ],
            default_dials: DesignDials::clamped(6, 4, 4),
            provenance: "layoza".into(),
        },
        IndustryRule {
            slug: "local-business".into(),
            name: "Local Businesses, Trades & Community Services".into(),
            keywords: vec!["local".into(), "repair".into(), "repairs".into(), "bicycle".into(), "bike".into(), "cycling".into(), "mechanic".into(), "cooperative".into(), "co-op".into(), "coffee".into(), "barber".into(), "florist".into(), "laundry".into(), "plumber".into(), "plumbing".into(), "electrician".into(), "roofing".into(), "cleaning".into(), "bakery".into(), "cafe".into(), "salon".into(), "contractor".into(), "gym".into(), "neighborhood".into()],
            recommended_styles: vec!["swiss-editorial-grid".into(), "crisp-product-workbench".into()],
            layout_archetype: "Immediate Service Area & Phone/Booking Hero + Reviews & Pricing Guide".into(),
            first_viewport_priority: "What you do, where you operate, hours/phone/booking CTA, and real local proof".into(),
            typography_pairing: TypographyPairing {
                id: "barlow-signage".into(),
                display_font: "Barlow Condensed".into(),
                body_font: "Barlow".into(),
                mono_font: "IBM Plex Mono".into(),
                mood: "Street-level, legible, signage-like: reads like the shopfront and the price board".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.googleapis.com/css2?family=Barlow+Condensed:wght@600;800&family=Barlow:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap".into(),
            },
            palette: PaletteRecommendation {
                id: "shopfront-signal".into(),
                background: "#EEF0EE".into(),
                surface: "#FFFFFF".into(),
                foreground: "#15181C".into(),
                muted: "#454B52".into(),
                primary_accent: "#0F5C3A".into(),
                secondary_accent: "#B8321F".into(),
                wcag_body_contrast: 0.0,
                notes: "Cool kerb-grey ground, shopfront green and signal red; replace with the street's own signage colours (see `distinctiveness`)".into(),
            },
            conversion_trust_elements: vec!["Clickable `tel:` phone number".into(), "Service area map & hours".into(), "Transparent service menu".into()],
            motion_language: "Snappy mobile-first drawer and booking interactions".into(),
            anti_patterns: vec!["Burying phone number and address in the footer".into(), "Heavy desktop-only WebGL".into()],
            default_dials: DesignDials::clamped(3, 3, 5),
            provenance: "adapted:ui-ux-pro-max+layoza".into(),
        },
        IndustryRule {
            slug: "mobile-app".into(),
            name: "Mobile-First Consumer Apps & PWA Experiences".into(),
            keywords: vec!["mobile".into(), "ios".into(), "android".into(), "pwa".into(), "app".into(), "consumer".into()],
            recommended_styles: vec!["crisp-product-workbench".into()],
            layout_archetype: "Thumb-Zone Ergonomic Shell + Interactive Device Flow Showcase".into(),
            first_viewport_priority: "44x44px touch targets, bottom-sheet/tab ergonomics, instant visual comprehension".into(),
            typography_pairing: TypographyPairing {
                id: "plus-jakarta-system".into(),
                display_font: "Plus Jakarta Sans".into(),
                body_font: "Plus Jakarta Sans".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "Tactile, responsive, native-feeling".into(),
                scale_ratio: "1.200 (Minor Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Plus+Jakarta+Sans:wght@400;500;600;700".into(),
            },
            palette: PaletteRecommendation {
                id: "adaptive-daylight".into(),
                background: "#FFFFFF".into(),
                surface: "#F4F4F5".into(),
                foreground: "#09090B".into(),
                muted: "#52525B".into(),
                primary_accent: "#0284C7".into(),
                secondary_accent: "#7C3AED".into(),
                wcag_body_contrast: 18.2,
                notes: "High-contrast adaptive daylight surface optimized for outdoor mobile screens".into(),
            },
            conversion_trust_elements: vec!["Touch-friendly interactive prototype".into(), "Offline/fast-load resilience".into()],
            motion_language: "Interruptible spring physics (`stiffness: 380, damping: 30`), swipe-safe gestures".into(),
            anti_patterns: vec!["Hover-only tooltips inaccessible on touch screens".into(), "Touch targets smaller than 44x44px".into()],
            default_dials: DesignDials::clamped(4, 6, 5),
            provenance: "adapted:impeccable+ui-ux-pro-max+layoza".into(),
        },
        IndustryRule {
            slug: "experimental-artistic".into(),
            name: "Experimental, 3D WebGL & Artistic Experiences".into(),
            keywords: vec!["experimental".into(), "3d".into(), "webgl".into(), "threejs".into(), "shader".into(), "interactive".into(), "exhibition".into(), "art".into()],
            recommended_styles: vec!["spatial-webgl-stage".into(), "kinetic-editorial".into()],
            layout_archetype: "Interactive 3D/Shader Viewport + Synchronized DOM HUD Overlay".into(),
            first_viewport_priority: "60fps spatial canvas with accessible DOM overlay and static reduced-motion fallback".into(),
            typography_pairing: TypographyPairing {
                id: "syne-space-mono".into(),
                display_font: "Syne".into(),
                body_font: "Space Grotesk".into(),
                mono_font: "Space Mono".into(),
                mood: "Avant-garde, spatial, immersive".into(),
                scale_ratio: "1.414 (Augmented Fourth)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Space+Grotesk:wght@400;500;700|Space+Mono:wght@400;700|Syne:wght@700;800".into(),
            },
            palette: PaletteRecommendation {
                id: "void-iridescent".into(),
                background: "#07080A".into(),
                surface: "#111318".into(),
                foreground: "#F3F4F6".into(),
                muted: "#9CA3AF".into(),
                primary_accent: "#38BDF8".into(),
                secondary_accent: "#F43F5E".into(),
                wcag_body_contrast: 17.8,
                notes: "Deep spatial void with high-contrast HUD telemetry overlays".into(),
            },
            conversion_trust_elements: vec!["Accessible HUD navigation".into(), "Reduced-motion toggle".into(), "Graceful WebGL fallback".into()],
            motion_language: "Three.js / R3F interactive camera & shader uniforms synced with GSAP ScrollTrigger".into(),
            anti_patterns: vec!["Unbounded GPU loops draining battery when offscreen".into(), "Inaccessible canvas-only text".into()],
            default_dials: DesignDials::clamped(9, 9, 3),
            provenance: "adapted:layoza-original+threejs".into(),
        },
        IndustryRule {
            slug: "wordpress-publishing".into(),
            name: "Native WordPress Gutenberg & Block Themes".into(),
            keywords: vec!["wordpress".into(), "gutenberg".into(), "block-theme".into(), "theme.json".into(), "wp".into(), "cms".into()],
            recommended_styles: vec!["block-first-editorial".into()],
            layout_archetype: "Native Block Pattern Composition (`theme.json` v3 + Semantic Block Markup)".into(),
            first_viewport_priority: "Zero-lock-in core block patterns (`core/group`, `core/columns`, `core/heading`) styled via `theme.json` presets".into(),
            typography_pairing: TypographyPairing {
                id: "instrument-plus-jakarta".into(),
                display_font: "Instrument Serif".into(),
                body_font: "Plus Jakarta Sans".into(),
                mono_font: "JetBrains Mono".into(),
                mood: "Editorial, block-native, maintainable".into(),
                scale_ratio: "1.250 (Major Third)".into(),
                google_fonts_url: "https://fonts.google.com/share?selection.family=Instrument+Serif|Plus+Jakarta+Sans:wght@400;500;600;700".into(),
            },
            palette: PaletteRecommendation {
                id: "wp-editorial-stone".into(),
                background: "#F8F7F4".into(),
                surface: "#EFECE6".into(),
                foreground: "#15171A".into(),
                muted: "#5A626E".into(),
                primary_accent: "#1D4ED8".into(),
                secondary_accent: "#B45309".into(),
                wcag_body_contrast: 16.3,
                notes: "Mapped directly to `--wp--preset--color--*` CSS variables in `theme.json`".into(),
            },
            conversion_trust_elements: vec!["Full Site Editor (FSE) compatibility".into(), "Fluid typography clamp() presets".into(), "Accessible block navigation".into()],
            motion_language: "Lightweight CSS block entrance transitions and IntersectionObserver reveals".into(),
            anti_patterns: vec!["Outputting a Next.js app when the user asked for WordPress".into(), "Hardcoding hex values instead of `var(--wp--preset--color--*)`".into()],
            default_dials: DesignDials::clamped(5, 4, 5),
            provenance: "layoza-original".into(),
        },
    ]
}

/// Returns the searchable catalog of UI Style Families with provenance.
pub fn catalog_styles() -> Vec<StyleFamily> {
    vec![
        StyleFamily {
            slug: "swiss-editorial-grid".into(),
            name: "Swiss Editorial Grid".into(),
            category: "Architectural & Editorial".into(),
            keywords: vec!["swiss".into(), "editorial".into(), "grid".into(), "corporate".into(), "hairline".into(), "typography".into()],
            characteristics: vec![
                "Exposed structural grid borders (`1px solid`) with generous internal cell padding".into(),
                "High-contrast serif or neo-grotesque display headlines paired with crisp body copy".into(),
                "Asymmetric column ratios (e.g., 4:8 or 5:7) with clear typographic hierarchy".into(),
            ],
            best_for: vec!["Corporate & Enterprise".into(), "Editorial Publishing".into(), "Finance & Fintech".into(), "Architecture".into()],
            avoid_when: vec!["Playful children's apps".into()],
            provenance: "adapted:awesome-design-skills+taste-skill+layoza".into(),
        },
        StyleFamily {
            slug: "kinetic-editorial".into(),
            name: "Kinetic Editorial Showcase".into(),
            category: "Motion-Led & Creative".into(),
            keywords: vec!["kinetic".into(), "agency".into(), "studio".into(), "portfolio".into(), "gsap".into(), "scroll".into()],
            characteristics: vec![
                "Choreographed display typography with SplitText mask reveals".into(),
                "Scroll-synchronized case study pinning and smooth image scale transitions".into(),
                "Tactile magnetic triggers with instant reduced-motion static fallback".into(),
            ],
            best_for: vec!["Marketing Agencies".into(), "Creative Studios".into(), "Product Launches".into()],
            avoid_when: vec!["High-density backoffice data entry forms".into()],
            provenance: "adapted:taste-skill+ui-ux-pro-max+layoza".into(),
        },
        StyleFamily {
            slug: "atelier-minimalism".into(),
            name: "Atelier Tactile Minimalism".into(),
            category: "Luxury & Craft".into(),
            keywords: vec!["luxury".into(), "minimal".into(), "atelier".into(), "warm".into(), "serif".into(), "lookbook".into()],
            characteristics: vec![
                "Warm paper and limestone neutrals (`#FAF8F5`) rather than sterile `#FFFFFF`".into(),
                "Unhurried vertical rhythm (`96px–140px` section spacing) and monograph captions".into(),
                "Zero decorative card wrappers; imagery and typography sit directly on the canvas".into(),
            ],
            best_for: vec!["Luxury Brands".into(), "Hospitality & Resorts".into(), "Design Portfolios".into()],
            avoid_when: vec!["Dense telemetry dashboards".into()],
            provenance: "adapted:impeccable+taste-skill+layoza".into(),
        },
        StyleFamily {
            slug: "crisp-product-workbench".into(),
            name: "Crisp Product Workbench".into(),
            category: "Product & SaaS".into(),
            keywords: vec!["saas".into(), "workbench".into(), "product".into(), "bento".into(), "clean".into(), "startup".into()],
            characteristics: vec![
                "Interactive live product state demonstrations in the hero instead of static screenshots".into(),
                "Subtle surface layering (`#FCFCFA` -> `#F2F3EE`) with crisp 1px borders".into(),
                "Clear scannable feature proof and transparent pricing architecture".into(),
            ],
            best_for: vec!["SaaS Startups".into(), "Productivity Tools".into(), "B2B Platforms".into()],
            avoid_when: vec!["Experimental art installations".into()],
            provenance: "adapted:ui-ux-pro-max+impeccable+layoza".into(),
        },
        StyleFamily {
            slug: "precision-terminal-craft".into(),
            name: "Precision Terminal & Hardware Craft".into(),
            category: "Developer & Industrial".into(),
            keywords: vec!["devtools".into(), "cli".into(), "ai".into(), "terminal".into(), "dark".into(), "industrial".into(), "rust".into()],
            characteristics: vec![
                "Monospace telemetry accents (`JetBrains Mono` / `IBM Plex Mono`) with `tabular-nums`".into(),
                "Low-glare carbon surfaces (`#0B0D10`) with high-contrast amber/emerald semantic signals".into(),
                "Copyable CLI snippets, architecture diagrams, and reproducible benchmark tables".into(),
            ],
            best_for: vec!["AI & Developer Tools".into(), "Infrastructure".into(), "Cybersecurity".into(), "Industrial B2B".into()],
            avoid_when: vec!["Local healthcare clinics".into()],
            provenance: "adapted:awesome-design-skills+ui-ux-pro-max+layoza".into(),
        },
        StyleFamily {
            slug: "high-density-workbench".into(),
            name: "High-Density Operational Cockpit".into(),
            category: "Dashboards & Data".into(),
            keywords: vec!["dashboard".into(), "admin".into(), "analytics".into(), "table".into(), "dense".into(), "finance".into()],
            characteristics: vec![
                "Compact 4px/8px/12px spacing grid maximizing above-the-fold data visibility".into(),
                "Right-aligned `tabular-nums` numeric columns, keyboard-first filtering, and clear status badges".into(),
                "Complete empty, loading, error, and overflow-resilient states".into(),
            ],
            best_for: vec!["SaaS Dashboards".into(), "Fintech Terminals".into(), "Admin Consoles".into()],
            avoid_when: vec!["Brand storytelling landing pages".into()],
            provenance: "adapted:impeccable+ui-ux-pro-max+layoza".into(),
        },
        StyleFamily {
            slug: "spatial-webgl-stage".into(),
            name: "Spatial WebGL & Interactive 3D Stage".into(),
            category: "3D & Experimental".into(),
            keywords: vec!["3d".into(), "threejs".into(), "webgl".into(), "r3f".into(), "shader".into(), "spatial".into(), "interactive".into()],
            characteristics: vec![
                "Demand-rendered or viewport-throttled Three.js / R3F canvas with PBR lighting or custom shaders".into(),
                "Accessible semantic HTML overlay that remains 100% usable when WebGL is disabled or reduced-motion is active".into(),
                "Synchronized camera keyframes driven by scroll progress or pointer parallax".into(),
            ],
            best_for: vec!["Experimental Sites".into(), "3D Product Configurators".into(), "Flagship Hardware Reveals".into()],
            avoid_when: vec!["Content-heavy documentation or low-power utility forms".into()],
            provenance: "layoza-original".into(),
        },
        StyleFamily {
            slug: "block-first-editorial".into(),
            name: "Native WordPress Block-First Editorial".into(),
            category: "CMS & Publishing".into(),
            keywords: vec!["wordpress".into(), "gutenberg".into(), "block".into(), "theme.json".into(), "publishing".into()],
            characteristics: vec![
                "100% native Gutenberg block markup (`<!-- wp:group -->`, `<!-- wp:columns -->`) driven by `theme.json` v3 tokens".into(),
                "Fluid `clamp()` typography scales and semantic `--wp--preset--*` custom properties".into(),
                "Zero page-builder bloat and sub-second LCP performance".into(),
            ],
            best_for: vec!["WordPress Block Themes".into(), "Editorial Publications".into(), "Content-Driven Corporate Sites".into()],
            avoid_when: vec!["Single-page WebGL games".into()],
            provenance: "layoza-original".into(),
        },
    ]
}

/// Generates a complete `DesignRecommendation` (including 3 materially distinct creative directions
/// for Mode A and Mode D, or strict DNA-preserving guidance for Mode B and Mode C).
pub fn recommend_design(
    query: &str,
    mode: OperatingMode,
    existing_dna: Option<&SiteDna>,
    override_dials: Option<DesignDials>,
) -> DesignRecommendation {
    let q_lower = query.to_ascii_lowercase();
    let industries = catalog_industries();
    let styles = catalog_styles();

    // Whole-word keyword scoring; no silent fallback (confidence is reported to the agent)
    let ranked = rank_industries(query, &industries);
    let match_score = ranked[0].0;
    let mut best_industry = if match_score > 0 {
        ranked[0].1.clone()
    } else {
        industries
            .iter()
            .find(|i| i.slug == "local-business")
            .cloned()
            .unwrap_or_else(|| industries[0].clone())
    };
    let match_confidence = match match_score {
        0 => "none",
        1..=5 => "weak",
        _ => "strong",
    }
    .to_string();
    let alternative_industries: Vec<(String, usize)> = ranked
        .iter()
        .skip(1)
        .filter(|(sc, _)| *sc > 0)
        .take(3)
        .map(|(sc, ind)| (ind.slug.clone(), *sc))
        .collect();

    // Prompt-Aware Polarity & Typography Synthesis:
    // When a brief explicitly requests warm ivory / porcelain / digital art gallery / living atelier
    // or specific signature accents (electric cobalt, coral vermilion), adapt the matched industry's
    // palette and typography pairing so the recommendation matches the brief's exact visual polarity.
    let wants_light_gallery = q_lower.contains("ivory")
        || q_lower.contains("porcelain")
        || q_lower.contains("gallery")
        || q_lower.contains("museum");
    if wants_light_gallery {
        best_industry.palette = PaletteRecommendation {
            id: "porcelain-cobalt-vermilion".into(),
            background: "#FAF7F2".into(),
            surface: "#F2ECE1".into(),
            foreground: "#0B1021".into(),
            muted: "#4A556E".into(),
            primary_accent: if q_lower.contains("cobalt") {
                "#0047FF".into()
            } else {
                best_industry.palette.primary_accent.clone()
            },
            secondary_accent: if q_lower.contains("vermilion") || q_lower.contains("coral") {
                "#E63B20".into()
            } else {
                best_industry.palette.secondary_accent.clone()
            },
            wcag_body_contrast: 17.7,
            notes: "Warm gallery porcelain & sculptural alabaster with deep midnight ink, electric cobalt, and coral vermilion".into(),
        };
        best_industry.typography_pairing = TypographyPairing {
            id: "playfair-plus-jakarta-mono".into(),
            display_font: "Playfair Display".into(),
            body_font: "Plus Jakarta Sans".into(),
            mono_font: "JetBrains Mono".into(),
            mood: "Swiss-editorial museum monograph with high-x-height display serif and optically balanced italic accents".into(),
            scale_ratio: "1.333 (Perfect Fourth)".into(),
            google_fonts_url: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&family=Playfair+Display:ital,wght@0,600;0,700;0,800;1,600&family=Plus+Jakarta+Sans:wght@400;500;600;700&display=swap".into(),
        };
    }

    let query_tokens = tokenize(query);
    // Match top styles (whole words, so "ai" does not match "repair")
    let mut matched_styles: Vec<StyleFamily> = styles
        .iter()
        .filter(|s| {
            best_industry.recommended_styles.contains(&s.slug)
                || s.keywords.iter().any(|k| keyword_hit(&query_tokens, k))
        })
        .cloned()
        .collect();
    if matched_styles.is_empty() {
        matched_styles.push(styles[0].clone());
        matched_styles.push(styles[3].clone());
    }

    let preservation_enforced = matches!(
        mode,
        OperatingMode::ExtendExisting | OperatingMode::ImproveExisting
    );

    let active_dials = if let Some(d) = override_dials {
        d
    } else if let Some(dna) = existing_dna {
        if preservation_enforced {
            dna.dials
        } else {
            best_industry.default_dials
        }
    } else {
        best_industry.default_dials
    };

    let signals = brief_signals(query);
    // Explicit dial overrides and preservation modes keep the caller's / site's motion level untouched
    let tune_brief_motion = override_dials.is_none() && !preservation_enforced;
    let tune = |m: u8| if tune_brief_motion { tune_motion(m, &signals) } else { m };
    let a_type = best_industry.typography_pairing.clone();
    let a_pal = best_industry.palette.clone();
    let (b_type, b_pal) =
        pick_from_pool(
            expressive_pool(),
            &signals,
            query,
            "B",
            std::slice::from_ref(&a_type.display_font),
            std::slice::from_ref(&a_pal.id),
        );
    let (c_type, c_pal) = pick_from_pool(
        utility_pool(),
        &signals,
        query,
        "C",
        &[a_type.display_font.clone(), b_type.display_font.clone()],
        &[a_pal.id.clone(), b_pal.id.clone()],
    );
    let b_copy = direction_b_copy(signals.surface);
    let c_copy = direction_c_copy(signals.surface);

    // Build 3 materially different directions (differing in layout composition, visual identity,
    // typography, content treatment, and motion choreography—never just color swaps!).
    // B and C follow the brief's surface (dashboard, store, venue, showcase) and niche palette family.
    let directions = vec![
        CreativeDirection {
            index: 1,
            title: format!("Direction A — {} (Editorial & Structural Clarity)", best_industry.name),
            style_family: "swiss-editorial-grid".into(),
            layout_composition: format!(
                "{} on an exposed structural grid where type and the subject's object share one asymmetric 7:5 composition, generous section rhythm (clamp(5.5rem, 9vw, 9rem)), and each section shaped by its content",
                best_industry.layout_archetype
            ),
            visual_identity: "Type as image: display set large enough to become architecture, ruled structure drawn from the subject's own notation (charts, labels, tickets), no floating card clutter".into(),
            typography: best_industry.typography_pairing.clone(),
            palette: best_industry.palette.clone(),
            content_treatment: "Lead with the subject's most characteristic object at scale and the real action (buy, book) working in the first viewport; specifics as scannable tables, never big-number stat blocks".into(),
            motion_choreography: "Decide per content: a typographic load sequence, content-shaped section entrances, crisp 160-240ms ease-out interactions; reduced-motion keeps fades only".into(),
            dials: DesignDials::clamped(
                active_dials.design_variance,
                tune(active_dials.motion_intensity.min(5)),
                active_dials.visual_density,
            ),
            mechanics: Vec::new(),
        },
        CreativeDirection {
            index: 2,
            title: format!("Direction B — {} ({})", best_industry.name, b_copy.name),
            style_family: b_copy.style_family.into(),
            layout_composition: b_copy.layout.into(),
            visual_identity: b_copy.identity.into(),
            typography: b_type,
            palette: b_pal,
            content_treatment: b_copy.content.into(),
            motion_choreography: b_copy.motion.into(),
            dials: DesignDials::clamped(
                (active_dials.design_variance + 3).min(10),
                tune((active_dials.motion_intensity + 3).min(10)),
                active_dials.visual_density.saturating_sub(1),
            ),
            mechanics: Vec::new(),
        },
        CreativeDirection {
            index: 3,
            title: format!("Direction C — {} ({})", best_industry.name, c_copy.name),
            style_family: c_copy.style_family.into(),
            layout_composition: c_copy.layout.into(),
            visual_identity: c_copy.identity.into(),
            typography: c_type,
            palette: c_pal,
            content_treatment: c_copy.content.into(),
            motion_choreography: c_copy.motion.into(),
            dials: DesignDials::clamped(
                active_dials.design_variance,
                tune(active_dials.motion_intensity),
                (active_dials.visual_density + 2).min(10),
            ),
            mechanics: Vec::new(),
        },
    ];
    let mut active_dials = active_dials;
    active_dials.motion_intensity = tune(active_dials.motion_intensity);

    let mut directions = directions;
    for d in &mut directions {
        verify_palette(&mut d.palette);
    }
    attach_mechanics(&mut directions, query, mode, existing_dna);
    verify_palette(&mut best_industry.palette);

    let workflow_guidance = match mode {
        OperatingMode::NewWebsite => vec![
            "Mode A (New Website): the user's prompt wins over everything here. The 3 directions are structural raw material (layout, content treatment, dials), not finished looks; the visual world comes from art direction.".into(),
            "DISTINCTIVENESS: Read `distinctiveness` first. If `match_confidence` is `none` or `weak`, the industry profile is only a fallback; build the visual world from the brief's place and material cues instead, and do not ship any listed category cliché unless the brief asks for it.".into(),
            "ART DIRECTION (load `skills/layoza-art-direction`): commit to ONE specific world. Derive the world from far (place, notation, print traditions), keep the subject up close (the trade's object as the signature interaction, at scale), rule out the category reflex and its obvious opposite, and put the visitor's real action in the first viewport. Let colour own whole sections. Decide motion and tools per design; nothing is mandatory.".into(),
            "ENGINEERING LESSONS: sticky headers need `html { scroll-padding-top: <header height + 1rem> }` (WCAG 2.4.11); web fonts need metric-adjusted fallback `@font-face` (size-adjust / ascent-override) or the font swap re-wraps headings and fails CLS; scroll reveals must end visible.".into(),
            "TYPE & LEADING: Keep multi-line display headings at `line-height >= 1.0` so ascenders and descenders never collide.".into(),
            "LAYOUT SAFETY: Keep short header links/buttons on one line (`white-space: nowrap`), give side-by-side heading columns >= 16px of clear space, collapse multi-column grids before gaps exceed the viewport, and size any <canvas> from a fixed-size container with devicePixelRatio capped at 2.".into(),
            "Verify in a real browser with `layoza qa <file-or-url>` (1440/768/375): it measures contrast, overflow, clipping, nav, scroll reveals, and lab Core Web Vitals; fix every error.".into(),
        ],
        OperatingMode::ExtendExisting => vec![
            "Mode B (Extend Existing Website — PRESERVATION MODE): Inspect `SITE-DNA.json`, `DESIGN.md`, and `COMPONENT-INVENTORY.json` BEFORE writing any code.".into(),
            "Reuse existing CSS custom properties, typography tokens, spacing rhythm, breakpoints, and motion intensity.".into(),
            "Ensure new header/nav items and badges use `white-space: nowrap; flex-shrink: 0;`, avoid narrow side-by-side label crowding (`gap >= 24px`), and verify `clipped_text_nodes == 0` across viewports.".into(),
            "Do NOT introduce a competing CSS framework, new font families, or unrequested dependencies.".into(),
        ],
        OperatingMode::ImproveExisting => vec![
            "Mode C (Improve Existing Website): Create a reversible baseline (`layoza baseline create`) and run `layoza audit` first.".into(),
            "Fix genuine hierarchy, font optical balance, column crowding (`< 24px` horizontal gap), header wrapping, text clipping (`scrollWidth > clientWidth`), overflow, accessibility, and responsive issues while preserving working brand identity.".into(),
            "Run `layoza verify` after edits to confirm zero visual or functional regressions.".into(),
        ],
        OperatingMode::FullRedesign => vec![
            "Mode D (Full Creative Redesign): Create a reversible baseline (`layoza baseline create`) before replacing existing files.".into(),
            "Select from the 3 distinct creative directions while preserving all required domain content, routes, and user workflows.".into(),
            "Enforce optically balanced display/italic font pairings, `line-height >= 1.04` on multi-line display headings, >= 24px column text separation, and `white-space: nowrap` on header/nav/badge elements; verify in Chromium across 1440px, 768px, and 375px.".into(),
        ],
    };

    DesignRecommendation {
        mode,
        query: query.to_string(),
        distinctiveness: distinctiveness_for(query, &best_industry.slug),
        brief_signals: signals,
        matched_industry: best_industry,
        match_score,
        match_confidence,
        alternative_industries,
        matched_styles,
        active_dials,
        preservation_enforced,
        directions,
        workflow_guidance,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn each_direction_gets_its_own_hero_and_two_signature_mechanics() {
        let rec = recommend_design("seafood restaurant on the harbour with a daily catch menu", OperatingMode::NewWebsite, None, None);
        let mut families = std::collections::HashSet::new();
        for d in &rec.directions {
            assert_eq!(d.mechanics.len(), 3, "direction {}: {:?}", d.index, d.mechanics);
            assert_eq!(d.mechanics[0].kind, crate::mechanics::MechanicKind::Hero);
            for m in &d.mechanics {
                assert!(families.insert(m.family.clone()), "family {} reused across directions", m.family);
                assert!(m.intensity <= crate::mechanics::Intensity::from_dial(d.dials.motion_intensity).max(crate::mechanics::Intensity::Calm));
            }
        }
        let again = recommend_design("seafood restaurant on the harbour with a daily catch menu", OperatingMode::NewWebsite, None, None);
        let ids = |r: &DesignRecommendation| r.directions.iter().flat_map(|d| d.mechanics.iter().map(|m| m.id.clone())).collect::<Vec<_>>();
        assert_eq!(ids(&rec), ids(&again));
    }

    use super::*;

    fn rec(q: &str) -> DesignRecommendation {
        recommend_design(q, OperatingMode::NewWebsite, None, None)
    }

    #[test]
    fn different_niches_give_materially_different_directions() {
        let dash = rec("bakery operations dashboard for production planning and waste tracking");
        let sea = rec("seafood restaurant on the harbour with daily catch menu and reservations");
        let fashion = rec("independent fashion store selling linen womenswear");
        let kinetic = rec("kinetic motion design studio portfolio with bold experimental type");

        assert_eq!(dash.brief_signals.surface, Surface::ProductUi);
        assert_eq!(sea.brief_signals.surface, Surface::Venue);
        assert_eq!(fashion.brief_signals.surface, Surface::Commerce);
        assert_eq!(kinetic.brief_signals.surface, Surface::Showcase);

        let all = [&dash, &sea, &fashion, &kinetic];
        for k in 1..3 {
            for (i, a) in all.iter().enumerate() {
                for b in all.iter().skip(i + 1) {
                    let (da, db) = (&a.directions[k], &b.directions[k]);
                    assert_ne!(da.layout_composition, db.layout_composition, "{} vs {}", a.query, b.query);
                    assert_ne!(da.palette.id, db.palette.id, "{} vs {}", a.query, b.query);
                    assert_ne!(da.typography.display_font, db.typography.display_font, "{} vs {}", a.query, b.query);
                }
            }
        }

        // Niche drives the palette family
        assert_eq!(sea.directions[1].palette.id, "deep-harbour-buoy");
        assert_eq!(sea.directions[2].palette.id, "tide-chart-paper");
        assert_eq!(fashion.directions[1].palette.id, "indigo-loom");
        assert_eq!(dash.directions[1].palette.id, "ops-console-night");
        assert_eq!(dash.directions[2].palette.id, "rye-ledger");

        // A task UI never gets scroll choreography or high motion; a kinetic studio keeps it
        for d in &dash.directions {
            assert!(d.dials.motion_intensity <= 4, "{}: {}", d.title, d.dials.motion_intensity);
            assert!(!d.motion_choreography.contains("ScrollTrigger"), "{}", d.title);
        }
        assert!(kinetic.directions[1].dials.motion_intensity >= 9);
    }

    #[test]
    fn named_constraints_and_overrides() {
        let calm = rec("calm accessible yoga studio portfolio, no animation please");
        assert!(calm.brief_signals.constraints.contains(&"reduced-motion".to_string()));
        assert!(calm.directions.iter().all(|d| d.dials.motion_intensity <= 1));

        // Explicit dials from the caller are not re-tuned
        let forced = recommend_design("bakery operations dashboard", OperatingMode::NewWebsite, None, Some(DesignDials::clamped(5, 8, 5)));
        assert_eq!(forced.active_dials.motion_intensity, 8);

        // Deterministic
        let a = rec("seafood restaurant on the harbour");
        let b = rec("seafood restaurant on the harbour");
        assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
    }

    #[test]
    fn pool_palettes_meet_aa() {
        for (_, p, _) in expressive_pool().into_iter().chain(utility_pool()) {
            let body = wcag_contrast(&p.foreground, &p.background).unwrap();
            let muted = wcag_contrast(&p.muted, &p.background).unwrap();
            assert!(body >= 4.5 && muted >= 4.5, "{} fails AA: body {:.2}, muted {:.2}", p.id, body, muted);
        }
    }
}
