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
    /// Why this direction was dealt for this brief: structure, type, palette and hero composition,
    /// and the other structures that fit the same slot.
    #[serde(default)]
    pub rationale: String,
    /// Dealt mechanics for this direction: 1 hero + 2 signature sections (`layoza mechanics show <id>`).
    #[serde(default)]
    pub mechanics: Vec<crate::mechanics::MechanicRef>,
    /// What the hero is made of (real photo, data as image, procedural texture, physical object,
    /// giant type). The three directions are never all pure typography.
    #[serde(default)]
    pub hero_material: String,
    /// Corner-radius language of this direction (sharp-0, soft-12, ticket-notch, cut-corner...).
    #[serde(default)]
    pub radius: String,
    /// Direction archetype id (layout composition + hero material + look family + corners).
    #[serde(default)]
    pub archetype: String,
    /// Page furniture for a new site: this direction's navigation mechanic (never shared with
    /// another direction of the same run).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nav: Option<crate::mechanics::MechanicRef>,
    /// Page furniture for a new site: this direction's footer mechanic (never shared).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<crate::mechanics::MechanicRef>,
    /// Type scale tokens built from the pairing's ratio, with the floors enforced: body text never
    /// below 1rem (16px), labels and captions never below 0.75rem (12px).
    #[serde(default)]
    pub type_tokens: std::collections::BTreeMap<String, String>,
    /// Second stylesheet link for this direction's mono / data face (free fonts only), when the
    /// pairing's own `font_url` does not already carry it. Load it next to `typography.font_url`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_font_url: Option<String>,
    /// Binding page-level form for a new site or redesign: hero frame, flow, section heads, ground,
    /// material, labels and motion (techniques, tools, fallbacks). Distinct across the three
    /// directions and steered away from the look ledger; axes in `pinned_by_prompt` follow the brief.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_shape: Option<crate::directions::page_shape::PageShape>,
}

/// One-line reminder printed above deal and design-system text output.
pub const PURPOSE_LINE: &str = "Layoza: build a page that belongs to this brief alone. No copies, no re-skins, no repeats of the ledger.";

/// Why Layoza exists, in one paragraph. Shared by the MCP instructions, prompts, README and skills.
pub const PURPOSE: &str = "Layoza exists so AI-built sites stop looking like slop. Make every design unique: never a copy, a re-skin or a repeat, from the nav through the hero and every section to the footer. Build it modern and subject-first, for this brief and no other. Reusing a look from the ledger, a template layout or another tool's house style counts as a failure, even when the page passes QA.";

/// Five checks to run before shipping; each slot must differ from the ledger and fit the brief.
pub const PURPOSE_CHECKLIST: [&str; 5] = [
    "Nav: a pattern the ledger has not shown recently, sized to this site's real routes.",
    "Hero: shows the trade's own objects or work, not a stock split or a gradient blob.",
    "Sections: each one earns its place with this brief's content; no filler grid repeated from a past run.",
    "Footer: built for what visitors of this trade need last, not the default four columns.",
    "Motion: one choreography chosen for this subject, reduced-motion safe, not the last run's reveal.",
];

/// Purpose paragraph plus checklist as Markdown.
pub fn purpose_markdown() -> String {
    let mut s = String::from(PURPOSE);
    s.push_str("\n\nBefore you ship, check each slot is unique against the ledger and on-brief:\n");
    for c in PURPOSE_CHECKLIST {
        s.push_str("- ");
        s.push_str(c);
        s.push('\n');
    }
    s
}

/// The page shape is binding (guidance line and MCP instructions).
pub const PAGE_SHAPE_RULE: &str = "PAGE SHAPE: each direction's `page_shape` is binding. Build its hero frame, flow, section heads, ground, material, labels and motion exactly as dealt; it wins over `layout_composition` wording. Use the named motion tools and keep every fallback. Axes in `pinned_by_prompt` follow the user's words; every other axis stays as dealt, unique against the ledger.";

/// How to read `understood_brief` (MCP instructions).
pub const UNDERSTOOD_RULE: &str = "Read understood_brief; proceed on its niche-based assumptions without asking; ask only an open_question that blocks building; explicit user asks (a colour, font, style, motion level) override the deal for that axis only — every other axis stays unique.";

/// Smallest body size, in rem (16px at the default root size).
pub const BODY_FLOOR_REM: f64 = 1.0;
/// Smallest label / caption size, in rem (12px).
pub const LABEL_FLOOR_REM: f64 = 0.75;

/// Type scale tokens from a ratio label such as `1.250 (Major Third)`. Steps are rounded to 3
/// decimals; body and label sizes are clamped to their floors whatever the ratio says.
pub fn type_tokens(scale_ratio: &str) -> std::collections::BTreeMap<String, String> {
    let r: f64 = scale_ratio.split_whitespace().next().and_then(|s| s.parse().ok()).filter(|r: &f64| (1.05..=2.0).contains(r)).unwrap_or(1.25);
    let rem = |v: f64| {
        let s = format!("{:.3}", v);
        format!("{}rem", s.trim_end_matches('0').trim_end_matches('.'))
    };
    let body = BODY_FLOOR_REM;
    let label = (body / r).max(LABEL_FLOOR_REM).min(body);
    let mut t = std::collections::BTreeMap::new();
    t.insert("label".to_string(), rem(label));
    t.insert("caption".to_string(), rem(label));
    t.insert("small".to_string(), rem(((body + label) / 2.0).max(LABEL_FLOOR_REM)));
    t.insert("body".to_string(), rem(body));
    t.insert("lead".to_string(), rem(body * r));
    t.insert("h3".to_string(), rem(body * r.powi(2)));
    t.insert("h2".to_string(), rem(body * r.powi(3)));
    t.insert("h1".to_string(), rem(body * r.powi(4)));
    t.insert("display".to_string(), format!("clamp({}, 6vw, {})", rem(body * r.powi(4)), rem(body * r.powi(6))));
    t.insert("ratio".to_string(), format!("{}", r));
    t
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
    /// `page` or `component`. A component brief skips page-level steps (hero, three directions)
    /// but keeps tokens, states, accessibility and motion rules.
    #[serde(default)]
    pub scope: String,
    /// Set when no library pick fits a brief with strong creative intent: the made-to-measure
    /// protocol to follow instead of forcing a library direction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_route: Option<String>,
    /// Plain-words history of this project's recent looks and what this run picks from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<String>,
    /// The user's recorded taste (from `.layoza/taste.md`), word for word.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taste: Vec<String>,
    /// Per-slot fresh / repeat report against the project's look ledger (set by the CLI and MCP
    /// server, which own the ledger).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uniqueness: Option<crate::uniqueness::Uniqueness>,
    /// What was read from the brief before dealing, with niche-based defaults for what it left out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub understood_brief: Option<crate::directions::page_shape::UnderstoodBrief>,
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
pub(crate) fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !(c.is_alphanumeric() || c == '.'))
        .map(|t| t.trim_matches('.').to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

/// The brief with its negated parts removed, so "avoid dashboard mockups" or "do not use
/// gradient blobs" never count as asking for a dashboard or a blob. A segment (line, sentence or
/// bullet) is dropped when it contains a negation word; bullets under a header that ends in
/// "avoid:" / "never:" / "don't:" are dropped until the next non-bullet segment.
pub fn affirmed_brief(query: &str) -> String {
    const NEG: &[&str] = &["avoid", "never", "no", "not", "don't", "dont", "without", "nor", "instead of", "rather than"];
    let mut kept: Vec<&str> = Vec::new();
    let mut in_negative_list = false;
    for line in query.lines() {
        // Bullets flattened onto one line ("Avoid: * a * b") still split into segments.
        let segs = line
            .split(['.', ';', '!', '?'])
            .flat_map(|s| s.split(" * ").enumerate().flat_map(|(i, b)| b.split(" - ").enumerate().map(move |(j, x)| (i > 0 || j > 0, x))));
        for (split_bullet, seg) in segs {
            let t = seg.trim();
            if t.is_empty() {
                continue;
            }
            let bullet = split_bullet || t.starts_with(['*', '-', '•']) || line.trim_start().starts_with(['*', '-', '•']);
            let lower = t.to_lowercase();
            let toks = tokenize(&lower.replace('\u{2019}', "'"));
            let negated = NEG.iter().any(|n| {
                let parts = tokenize(n);
                if parts.len() == 1 && !n.contains('\'') {
                    toks.iter().any(|w| w == n)
                } else {
                    lower.contains(n)
                }
            });
            let header = lower.trim_end().ends_with(':');
            if header {
                in_negative_list = negated;
                if !negated {
                    kept.push(t);
                }
                continue;
            }
            if !bullet {
                in_negative_list = false;
            }
            if negated || (bullet && in_negative_list) {
                continue;
            }
            kept.push(t);
        }
    }
    kept.join(". ")
}

/// Whole-word (or whole-phrase) match, so `ai` no longer matches "detail" and `art` no longer matches "start".
pub(crate) fn keyword_hit(tokens: &[String], keyword: &str) -> bool {
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
#[allow(clippy::too_many_arguments)]
fn attach_mechanics(directions: &mut [CreativeDirection], query: &str, niches: &[String], mode: OperatingMode, existing_dna: Option<&SiteDna>, variant: u64, compositions: &[&str], recent_ids: &[String]) {
    use crate::mechanics::{deal_mechanics_with, DealOptions, ExistingSite, Intensity, MechanicRef};
    let existing = match mode {
        OperatingMode::ExtendExisting | OperatingMode::ImproveExisting => existing_dna.map(|d| ExistingSite::from_dna(d, "SITE-DNA")),
        _ => None,
    };
    let base = fnv1a(&query.to_lowercase()) ^ variant.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut used_families: Vec<String> = Vec::new();
    let mut used_compositions: Vec<String> = Vec::new();
    let mut used_looks: Vec<String> = Vec::new();
    let mut materials: Vec<String> = Vec::new();
    let mut used_furniture: Vec<String> = Vec::new();
    for (i, d) in directions.iter_mut().enumerate() {
        let deal = deal_mechanics_with(&DealOptions {
            brief: query.to_string(),
            intensity: Some(Intensity::from_dial(d.dials.motion_intensity)),
            count: 2,
            seed: Some(base.wrapping_add(d.index as u64)),
            existing: existing.clone(),
            exclude_families: used_families.clone(),
            // The direction's archetype suggests a composition; heroes already used by another
            // direction are skipped, so the three first screens differ.
            prefer_composition: compositions.get(i).map(|c| c.to_string()),
            exclude_compositions: used_compositions.clone(),
            hero_offset: variant as usize,
            // One niche signal: the deal reads the trade exactly as the design system did.
            niches: Some(niches.to_vec()),
            recent_ids: recent_ids.to_vec(),
            // No two directions share a look family, and the last is never a third pure-type hero.
            exclude_look_families: used_looks.clone(),
            avoid_giant_type: !materials.is_empty() && materials.iter().all(|m| m == "giant-type"),
            // No two directions share a nav or a footer.
            exclude_ids: used_furniture.clone(),
            ..Default::default()
        });
        d.nav = deal.nav.as_ref().map(MechanicRef::from);
        d.footer = deal.footer.as_ref().map(MechanicRef::from);
        used_furniture.extend(deal.nav.iter().chain(&deal.footer).map(|m| m.id.clone()));
        used_looks.extend(deal.hero.iter().map(|h| h.look_family.clone()).filter(|f| !f.is_empty()));
        materials.extend(deal.hero.iter().map(|h| h.hero_material.clone()));
        d.mechanics = deal.hero.iter().chain(&deal.signature).map(MechanicRef::from).collect();
        used_families.extend(d.mechanics.iter().map(|m| m.family.clone()));
        used_compositions.extend(deal.hero.iter().map(|h| h.composition.clone()).filter(|c| !c.is_empty()));
    }
}

pub(crate) fn verify_palette(p: &mut PaletteRecommendation) {
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

pub(crate) fn fnv1a(text: &str) -> u64 {
    text.bytes().fold(0xcbf29ce484222325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3))
}

pub(crate) fn pairing(id: &str, display: &str, body: &str, mono: &str, mood: &str, ratio: &str, url: &str) -> TypographyPairing {
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
pub(crate) fn palette(id: &str, bg: &str, surface: &str, fg: &str, muted: &str, primary: &str, secondary: &str, contrast: f64, notes: &str) -> PaletteRecommendation {
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
    /// Developer tools, CLIs, SDKs, MCP servers, AI-agent infrastructure: the tool working on real
    /// input, and the install command, lead.
    DevTool,
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
    /// Hex colours written in the brief, in order. When present they are the palette; the
    /// directions' palettes are only fallbacks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub brief_colors: Vec<String>,
    /// Font families the brief names, in order. When present they are the type system; the
    /// directions' pairings are only fallbacks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub brief_fonts: Vec<String>,
    /// Catalogue niches the brief names (`mechanics::detect_niches`): the one niche signal shared by
    /// `design-system` and `deal`, so both read the same trade. `niche_tags` are palette families only.
    #[serde(default)]
    pub niches: Vec<String>,
    /// The brief's own nouns (heuristic noun phrases, no trade table). Map every mechanic onto them.
    #[serde(default)]
    pub brief_objects: Vec<String>,
    /// The visitor's action as the brief words it; absent when the brief names none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brief_action: Option<String>,
}

/// Families recognised when a brief names its own type. Longer names first so "Geist Mono" wins
/// over "Geist" and "IBM Plex Mono" over "IBM Plex Sans".
// Recognition only: a brief that names one of these is read as naming its brand type. Commercial
// faces in this list (Neue Montreal, ...) are never put in a CDN link; directions only link free fonts.
const KNOWN_FONTS: &[&str] = &[
    "Atkinson Hyperlegible", "Barlow Condensed", "Bricolage Grotesque", "Cabinet Grotesk", "Clash Display",
    "Cormorant Garamond", "DM Serif Display", "EB Garamond", "General Sans", "Hanken Grotesk", "IBM Plex Mono",
    "IBM Plex Sans", "IBM Plex Serif", "Instrument Sans", "Instrument Serif", "JetBrains Mono", "Libre Franklin",
    "Neue Montreal", "Playfair Display", "Plus Jakarta Sans", "Public Sans", "Roboto Mono", "Source Sans 3",
    "Space Grotesk", "Space Mono", "Work Sans", "Fira Code", "Fira Sans", "Geist Mono", "DM Sans", "DM Mono",
    "Archivo", "Barlow", "Fraunces", "Geist", "Inter", "Karla", "Lexend", "Lora", "Manrope", "Montserrat",
    "Newsreader", "Outfit", "Poppins", "Roboto", "Satoshi", "Sora", "Syne", "Urbanist",
];

/// Font families named in the brief, in first-mention order.
fn brief_font_names(query: &str) -> Vec<String> {
    let lower = query.to_lowercase();
    let bytes = lower.as_bytes();
    let boundary = |i: usize| i >= bytes.len() || !bytes[i].is_ascii_alphanumeric();
    let mut found: Vec<(usize, &str)> = Vec::new();
    let mut taken: Vec<(usize, usize)> = Vec::new();
    for name in KNOWN_FONTS {
        let n = name.to_lowercase();
        let mut from = 0;
        while let Some(off) = lower[from..].find(&n) {
            let i = from + off;
            let end = i + n.len();
            let free = !taken.iter().any(|&(a, b)| i < b && end > a);
            if (i == 0 || boundary(i - 1)) && boundary(end) && free {
                taken.push((i, end));
                if !found.iter().any(|(_, f)| f == name) {
                    found.push((i, name));
                }
            }
            from = end;
        }
    }
    found.sort();
    found.into_iter().map(|(_, f)| f.to_string()).collect()
}

/// `#RRGGBB` / `#RGB` colours in the brief, upper-cased, first occurrence order.
fn brief_hex_colors(query: &str) -> Vec<String> {
    let b = query.as_bytes();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'#' {
            let n = b[i + 1..].iter().take_while(|c| c.is_ascii_hexdigit()).count();
            if n == 6 || n == 3 {
                let hex = format!("#{}", &query[i + 1..i + 1 + n]).to_uppercase();
                if !out.contains(&hex) {
                    out.push(hex);
                }
            }
            i += 1 + n;
        } else {
            i += 1;
        }
    }
    out
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
    ("devtools", "code"), ("devtool", "code"), ("developer", "code"), ("developers", "code"), ("cli", "code"),
    ("sdk", "code"), ("api", "code"), ("mcp", "code"), ("compiler", "code"), ("terminal", "code"), ("github", "code"),
    ("npm", "code"), ("llm", "code"), ("coding", "code"), ("open source", "code"), ("command line", "code"),
    ("dev tool", "code"), ("dev tools", "code"),
    ("law", "legal"), ("lawyer", "legal"), ("lawyers", "legal"), ("attorney", "legal"), ("attorneys", "legal"),
    ("solicitor", "legal"), ("solicitors", "legal"), ("legal", "legal"), ("barrister", "legal"), ("barristers", "legal"),
    ("notary", "legal"), ("litigation", "legal"),
    ("kids", "play"), ("children", "play"), ("child", "play"), ("toddler", "play"), ("toddlers", "play"), ("toy", "play"),
    ("toys", "play"), ("playground", "play"), ("nursery", "play"),
    ("clinic", "care"), ("dental", "care"), ("dentist", "care"), ("doctor", "care"), ("physio", "care"),
    ("physiotherapy", "care"), ("therapy", "care"), ("therapist", "care"), ("health", "care"), ("medical", "care"),
    ("wellness", "care"), ("yoga", "care"), ("pilates", "care"), ("vet", "care"), ("veterinary", "care"), ("pharmacy", "care"),
    ("museum", "civic"), ("library", "civic"), ("archive", "civic"), ("heritage", "civic"), ("council", "civic"),
    ("charity", "civic"), ("nonprofit", "civic"), ("foundation", "civic"), ("school", "civic"), ("university", "civic"),
    ("exhibition", "civic"),
    ("luxury", "luxe"), ("watch", "luxe"), ("watches", "luxe"), ("jewellery", "luxe"), ("jewelry", "luxe"),
    ("perfume", "luxe"), ("couture", "luxe"),
    ("funeral", "memorial"), ("funerals", "memorial"), ("memorial", "memorial"), ("cremation", "memorial"),
    ("crematorium", "memorial"), ("cemetery", "memorial"), ("obituary", "memorial"), ("undertaker", "memorial"),
    ("bereavement", "memorial"), ("remembrance", "memorial"), ("burial", "memorial"),
    ("tattoo", "ink"), ("tattoos", "ink"), ("tattooist", "ink"), ("tattooing", "ink"), ("piercing", "ink"),
    ("flash sheet", "ink"), ("printmaking", "ink"), ("letterpress", "ink"), ("calligraphy", "ink"),
    ("utility", "infrastructure"), ("utilities", "infrastructure"), ("infrastructure", "infrastructure"),
    ("water supply", "infrastructure"), ("grid", "infrastructure"), ("electricity", "infrastructure"),
    ("broadband", "infrastructure"), ("transit", "infrastructure"), ("railway", "infrastructure"),
    ("wastewater", "infrastructure"), ("pipeline", "infrastructure"), ("outage", "infrastructure"),
    ("wedding", "celebration"), ("weddings", "celebration"), ("bride", "celebration"), ("bridal", "celebration"),
    ("celebration", "celebration"), ("party", "celebration"), ("anniversary", "celebration"),
    ("banquet", "celebration"), ("engagement", "celebration"), ("gala", "celebration"),
    ("camp", "play"), ("summer camp", "play"), ("day camp", "play"), ("holiday club", "play"),
    ("scouts", "play"), ("playgroup", "play"),
    ("plumber", "trades"), ("plumbing", "trades"), ("electrician", "trades"), ("roofer", "trades"),
    ("roofing", "trades"), ("joiner", "trades"), ("carpenter", "trades"), ("builder", "trades"),
    ("handyman", "trades"), ("hvac", "trades"), ("boiler", "trades"), ("locksmith", "trades"),
    ("contractor", "trades"), ("tradesman", "trades"),
    ("freight", "data"), ("shipping", "data"), ("warehouse", "data"), ("fleet", "data"),
    ("courier", "data"), ("haulage", "data"), ("supply chain", "data"), ("dispatch", "data"),
    ("bookshop", "literary"), ("bookstore", "literary"), ("bookseller", "literary"), ("books", "literary"),
    ("literary", "literary"), ("poetry", "literary"), ("publisher", "literary"), ("novel", "literary"),
    ("reading", "literary"),
    ("invoicing", "finance"), ("invoice", "finance"), ("invoices", "finance"), ("accounting", "finance"),
    ("bookkeeping", "finance"), ("payroll", "finance"), ("fintech", "finance"), ("bank", "finance"),
    ("banking", "finance"), ("tax", "finance"), ("budget", "finance"), ("payments", "finance"),
];

/// True when the brief names a right-to-left language or direction (Arabic, Hebrew, Persian/Farsi,
/// Urdu; `lang="ar|he|fa|ur"`, `dir="rtl"`, "RTL", "right-to-left").
pub fn is_rtl_brief(query: &str) -> bool {
    let q = query.to_lowercase();
    let toks = tokenize(&q);
    let words = ["arabic", "hebrew", "persian", "farsi", "urdu", "rtl", "right-to-left"];
    let compact: String = q.chars().filter(|c| !c.is_whitespace() && *c != '"' && *c != '\'').collect();
    words.iter().any(|w| keyword_hit(&toks, w))
        || q.contains("right-to-left")
        || compact.contains("dir=rtl")
        || ["ar", "he", "fa", "ur"].iter().any(|l| {
            [format!("lang={l}"), format!("lang:{l}"), format!("locale={l}"), format!("locale:{l}")].iter().any(|p| {
                compact.match_indices(p.as_str()).any(|(i, _)| !compact[i + p.len()..].starts_with(|c: char| c.is_ascii_alphabetic()))
            })
        })
}

/// Reads surface, niche families, moods and named constraints from the brief.
pub fn brief_signals(query: &str) -> BriefSignals {
    let tokens = tokenize(query);
    let has = |w: &str| keyword_hit(&tokens, w);
    let any = |ws: &[&str]| ws.iter().any(|w| has(w));

    // The surface with the most distinct hits wins (ties keep this order), so a long brief that
    // mentions a dashboard once as an example is still read as the dev tool it describes.
    let count = |ws: &[&str]| ws.iter().filter(|w| has(w)).count();
    let surfaces: [(Surface, usize); 6] = [
        (Surface::ProductUi, count(&["dashboard", "admin", "console", "portal", "backoffice", "back-office", "crm", "erp", "internal tool", "ops", "analytics"])),
        (
            Surface::DevTool,
            count(&[
                "devtools", "devtool", "dev tool", "dev tools", "developer tools", "developer tool", "developer platform", "cli", "command line", "sdk", "mcp",
                "open source", "open-source", "compiler", "coding agent", "coding agents", "ai agent", "ai agents", "agent skills", "llm",
                "llms", "npm", "github", "api reference",
            ]),
        ),
        (Surface::Commerce, count(&["store", "shop", "ecommerce", "e-commerce", "boutique", "catalog", "catalogue", "selling", "sells", "webshop"])),
        (Surface::Venue, count(&["restaurant", "cafe", "bistro", "bar", "hotel", "menu", "reservations", "booking", "bakery", "venue", "brasserie", "diner", "museum"])),
        (Surface::Showcase, count(&["portfolio", "studio", "agency", "showreel", "reel", "artist", "photographer", "designer"])),
        (Surface::Editorial, count(&["magazine", "journal", "blog", "publication", "newsletter", "news"])),
    ];
    let best = surfaces.iter().map(|(_, c)| *c).max().unwrap_or(0);
    let surface = if best == 0 {
        Surface::General
    } else {
        surfaces.iter().find(|(_, c)| *c == best).map(|(s, _)| *s).unwrap_or(Surface::General)
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
    // The user's own motion words win over the industry default: "highly animated" must not
    // come back as a mid motion dial.
    let heavy_motion = [
        "highly animated", "heavily animated", "very animated", "fully animated", "lots of animation", "lots of motion",
        "motion-rich", "motion rich", "animation-heavy", "cinematic", "immersive", "scrollytelling", "scroll storytelling",
        "scroll-driven", "scroll driven", "wow factor", "awwwards",
    ];
    let heavy = heavy_motion.iter().any(|p| lower.contains(p));
    if heavy {
        if !moods.iter().any(|m| m == "bold") {
            moods.push("bold".into());
        }
        motion_bias += 4;
    } else if any(&["animated", "animations", "animation", "dynamic", "lively"]) && !lower.contains("no animation") {
        motion_bias += 2;
    }
    let mut constraints: Vec<String> = Vec::new();
    let mut motion_cap: Option<u8> = None;
    let mut cap = |c: u8| motion_cap = Some(motion_cap.map_or(c, |m: u8| m.min(c)));
    if ["no animation", "no motion", "without animation", "static site"].iter().any(|p| lower.contains(p)) {
        constraints.push("reduced-motion".into());
        cap(1);
    } else if ["reduced motion", "reduced-motion", "reduce motion"].iter().any(|p| lower.contains(p)) {
        // "Respect prefers-reduced-motion" next to "highly animated" asks for a fallback, not stillness.
        if heavy {
            constraints.push("reduced-motion fallback".into());
        } else {
            constraints.push("reduced-motion".into());
            cap(1);
        }
    }
    // Accessibility and performance asks are guardrails (reduced-motion fallback, transform-only
    // animation, budgets). When the same brief explicitly asks for heavy motion they must not
    // silently cut it to a mid dial; elderly/senior audiences and low-bandwidth still cap.
    if any(&["accessible", "accessibility", "wcag", "elderly", "seniors"]) {
        constraints.push("accessibility-first".into());
        if !heavy || any(&["elderly", "seniors"]) {
            cap(5);
        }
    }
    if any(&["fast", "lightweight", "performance", "low-bandwidth", "offline"]) {
        constraints.push("performance-budget".into());
        if !heavy || any(&["low-bandwidth", "offline"]) {
            cap(5);
        }
    }
    if surface == Surface::ProductUi {
        constraints.push("task-ui (no scroll choreography)".into());
        cap(4);
    }
    if is_rtl_brief(query) {
        constraints.push("rtl (logical CSS properties, mirrored layout)".into());
    }

    let brief_colors = brief_hex_colors(query);
    let brief_fonts = brief_font_names(query);
    let niches = crate::mechanics::niche_signal(query);
    let brief_objects = crate::mechanics::brief_objects(query);
    let brief_action = crate::mechanics::brief_action(query);
    BriefSignals { surface, niche_tags, moods, constraints, motion_bias, motion_cap, brief_colors, brief_fonts, niches, brief_objects, brief_action }
}

/// Applies the brief's motion bias and ceiling to a direction's motion dial.
fn tune_motion(base: u8, signals: &BriefSignals) -> u8 {
    let biased = (base as i16 + signals.motion_bias as i16).clamp(0, 10) as u8;
    signals.motion_cap.map_or(biased, |c| biased.min(c))
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

/// Category clichés; for a slug the catalogue does not list (incl. `brief-derived`), they are read
/// from the brief's surface and niche words instead of a fixed generic list.
fn category_cliches_for(slug: &str, signals: &BriefSignals) -> Vec<String> {
    let known = ["artisan-craft", "local-business", "saas-startup", "ai-devtools", "luxury-premium", "creative-portfolio", "hospitality-travel", "ecommerce-catalog"];
    if known.contains(&slug) {
        category_cliches(slug)
    } else {
        crate::directions::derived_cliches(signals)
    }
}

fn distinctiveness_for(query: &str, slug: &str, signals: &BriefSignals) -> DistinctivenessBrief {
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
        category_cliches: category_cliches_for(slug, signals),
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
            name: "AI Systems, Developer Tools & Open-Source CLIs".into(),
            // Multi-word agent phrases instead of a bare `agent`, which used to claim "travel agent" / "estate agent" briefs
            keywords: [
                "ai", "devtools", "devtool", "dev tool", "dev tools", "developer tools", "developer tool", "developer platform", "cli", "command line", "sdk",
                "compiler", "api", "infrastructure", "rust", "mcp", "mcp server", "open source", "ai agent", "ai agents", "coding agent",
                "coding agents", "agent skills", "agentic", "llm", "llms", "npm", "github", "coding assistant", "language server",
            ]
            .iter()
            .map(|k| k.to_string())
            .collect(),
            recommended_styles: vec!["precision-terminal-craft".into(), "crisp-product-workbench".into()],
            layout_archetype: "Command/Code Proof Hero + Benchmark Telemetry Ledger".into(),
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
            conversion_trust_elements: vec![
                "One-click CLI install snippet".into(),
                "Reproducible p99 benchmark table".into(),
                "Live interactive API/MCP inspector".into(),
                "Open-source proof: licence, release cadence, changelog and a real before/after of the tool's output".into(),
            ],
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
    raw_query: &str,
    mode: OperatingMode,
    existing_dna: Option<&SiteDna>,
    override_dials: Option<DesignDials>,
) -> DesignRecommendation {
    recommend_design_variant(raw_query, mode, existing_dna, override_dials, 0)
}

/// Same as [`recommend_design`], but `variant` > 0 re-deals the directions' hero compositions,
/// fonts and palettes for the same brief, so asking again gives a different set of looks.
pub fn recommend_design_variant(
    raw_query: &str,
    mode: OperatingMode,
    existing_dna: Option<&SiteDna>,
    override_dials: Option<DesignDials>,
    variant: u64,
) -> DesignRecommendation {
    recommend_design_steered(raw_query, mode, existing_dna, override_dials, variant, &[])
}

/// The looks (palette, type, hero, layout, mechanics) of a recommendation's directions, for the
/// project's look ledger.
pub fn looks_of(rec: &DesignRecommendation, run: &str) -> Vec<crate::baseline::Look> {
    rec.directions
        .iter()
        .map(|d| crate::baseline::Look {
            run: run.to_string(),
            source: "design-system".into(),
            palette: Some(d.palette.id.clone()),
            // Families, not pairing ids: two pairings that share a display face are the same look.
            typography: Some(crate::uniqueness::type_key(&d.typography.display_font, &d.typography.body_font)),
            hero: d.mechanics.iter().find(|m| m.kind == crate::mechanics::MechanicKind::Hero).map(|m| m.id.clone()),
            layout: Some(d.style_family.clone()),
            mechanics: d.mechanics.iter().chain(&d.nav).chain(&d.footer).map(|m| m.id.clone()).collect(),
            nav: d.nav.as_ref().map(|m| m.id.clone()),
            footer: d.footer.as_ref().map(|m| m.id.clone()),
            sections: d.mechanics.iter().filter(|m| m.kind != crate::mechanics::MechanicKind::Hero).map(|m| m.id.clone()).collect(),
            archetype: (!d.archetype.is_empty()).then(|| d.archetype.clone()),
            transition: None,
            at: None,
            hero_frame: d.page_shape.as_ref().map(|p| p.hero_frame.id.clone()),
            flow: d.page_shape.as_ref().map(|p| p.flow.id.clone()),
            section_head: d.page_shape.as_ref().map(|p| p.section_head.id.clone()),
            ground: d.page_shape.as_ref().map(|p| p.ground.id.clone()),
            material: d.page_shape.as_ref().map(|p| p.material.id.clone()),
            labels: d.page_shape.as_ref().map(|p| p.labels.id.clone()),
            motion_signature: d.page_shape.as_ref().map(|p| p.motion_signature.clone()).filter(|s| !s.is_empty()),
            motion_tools: d.page_shape.as_ref().map(|p| p.motion_tools.clone()).unwrap_or_default(),
            pinned: d.page_shape.as_ref().map(|p| p.pinned_by_prompt.clone()).unwrap_or_default(),
        })
        .collect()
}

/// [`recommend_design_variant`] steered away from `recent` looks (this project's last runs, from
/// `.layoza/looks.jsonl`): the dealt mechanics avoid recent heroes and sections, and of variants
/// `variant..variant+6` the one sharing the fewest palettes, type pairings and layouts with the
/// recent runs is returned. A note in `workflow_guidance` says what was avoided.
pub fn recommend_design_avoiding(
    raw_query: &str,
    mode: OperatingMode,
    existing_dna: Option<&SiteDna>,
    override_dials: Option<DesignDials>,
    variant: u64,
    recent: &[crate::baseline::Look],
) -> DesignRecommendation {
    if recent.is_empty() {
        return recommend_design_variant(raw_query, mode, existing_dna, override_dials, variant);
    }
    let ids = crate::uniqueness::recent_mechanic_ids(recent);
    // Every slot counts: palette, type, archetype, nav, hero, sections and footer.
    let overlap = |rec: &DesignRecommendation| -> usize { crate::uniqueness::check(&looks_of(rec, "candidate"), recent, recent.len(), true).repeats };
    // Recent type families (ledger keys are `display/body`) steer the type deal itself.
    let (mut recent_display, mut recent_body): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    for (d, b) in recent.iter().filter_map(|l| l.typography.as_deref()?.split_once('/')) {
        recent_display.push(d.to_string());
        recent_body.push(b.to_string());
    }
    let mut best: Option<(usize, DesignRecommendation)> = None;
    for v in variant..variant + 6 {
        let rec = crate::directions::with_recent_faces(recent_display.clone(), recent_body.clone(), || {
            crate::directions::page_shape::with_recent_shapes(recent.to_vec(), || recommend_design_steered(raw_query, mode, existing_dna, override_dials, v, &ids))
        });
        let o = overlap(&rec);
        if best.as_ref().is_none_or(|(b, _)| o < *b) {
            best = Some((o, rec));
        }
        if o == 0 {
            break;
        }
    }
    let (o, mut rec) = best.expect("at least one variant");
    let runs: std::collections::BTreeSet<&str> = recent.iter().map(|l| l.run.as_str()).collect();
    rec.workflow_guidance.push(format!(
        "LOOK LEDGER: steered away from {} look(s) in this project's last {} run(s) (.layoza/looks.jsonl); {} slot repeat(s) remain (see `uniqueness`). Pass ledger=false (CLI --no-ledger) to ignore it.",
        recent.len(),
        runs.len(),
        o
    ));
    rec
}

/// [`recommend_design_variant`] that also steers the dealt mechanics away from `recent_ids`
/// (heroes and sections used in this project's recent runs, from `.layoza/looks.jsonl`).
pub fn recommend_design_steered(
    raw_query: &str,
    mode: OperatingMode,
    existing_dna: Option<&SiteDna>,
    override_dials: Option<DesignDials>,
    variant: u64,
    recent_ids: &[String],
) -> DesignRecommendation {
    // Match on what the brief asks for, not on what it rules out.
    let affirmed = affirmed_brief(raw_query);
    let query: &str = if affirmed.trim().is_empty() { raw_query } else { &affirmed };
    let q_lower = query.to_ascii_lowercase();
    let industries = catalog_industries();
    let styles = catalog_styles();

    // Whole-word keyword scoring; no silent fallback (confidence is reported to the agent)
    let ranked = rank_industries(query, &industries);
    let match_score = ranked[0].0;
    let signals = brief_signals(query);
    let mut subject = crate::directions::read_subject(query, &signals);
    if !subject.from_trades {
        // Unknown trade: the brief's own nouns and verbs, not a fixed fallback shared by every
        // unknown brief. No verb in the brief means no guessed call to action.
        let mut objects: Vec<String> = signals.brief_objects.iter().take(3).map(|o| format!("the {o}")).collect();
        for fallback in &subject.objects {
            if objects.len() >= 3 {
                break;
            }
            if !objects.contains(fallback) {
                objects.push(fallback.clone());
            }
        }
        subject.objects = objects;
        subject.action = signals
            .brief_action
            .clone()
            .unwrap_or_else(|| "the visitor's next step (the brief names none: use the niche's usual action, do not assume booking)".into());
    }
    let mut best_industry = if match_score > 0 {
        ranked[0].1.clone()
    } else {
        // No catalogue industry matched: build the profile from the brief's own words instead of
        // handing back a canned local-business default. Type and palette are the best pool fit.
        let first = crate::directions::deal(&signals, query, 0, None);
        crate::directions::brief_industry(query, &signals, &subject, first[0].typography.clone(), first[0].palette.clone())
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
    // ("museum" alone no longer triggers it: a kids' museum is not a porcelain gallery.)
    let wants_light_gallery = q_lower.contains("ivory") || q_lower.contains("porcelain") || q_lower.contains("gallery");
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

    // Explicit dial overrides and preservation modes keep the caller's / site's motion level untouched
    let tune_brief_motion = override_dials.is_none() && !preservation_enforced;
    let tune = |m: u8| if tune_brief_motion { tune_motion(m, &signals) } else { m };
    let heavy = tune_brief_motion && signals.motion_bias >= 4 && signals.motion_cap.is_none_or(|c| c >= 7);

    // Deal three directions: each is an archetype (layout composition + palette tone + type voice +
    // hero composition) with type and palette chosen for it from decoupled pools. A variant > 0
    // re-deals every slot away from variant 0's picks.
    // Read the brief before dealing: pins, register, stack and the motion tier it allows.
    let new_site = matches!(mode, OperatingMode::NewWebsite | OperatingMode::FullRedesign);
    let framework = existing_dna.map(|d| d.framework.as_str());
    let shape_brief = crate::directions::page_shape::ShapeBrief::read(&signals, query, raw_query, framework, variant, signals.motion_bias >= 4);
    let understood_brief = crate::directions::page_shape::understand(&shape_brief, &signals, query, &best_industry.name, &match_confidence, tune(active_dials.motion_intensity));
    let industry_look = (match_score > 0).then_some((&best_industry.typography_pairing, &best_industry.palette));
    let dealt = crate::directions::deal(&signals, query, variant, industry_look);
    let (place_cues, material_cues) = extract_cues(query);
    let ctx = crate::directions::Ctx {
        subject: &subject,
        place: place_cues.first().map(String::as_str),
        material: material_cues.first().map(String::as_str),
        surface: signals.surface,
    };
    let mut directions: Vec<CreativeDirection> = Vec::new();
    for (i, d) in dealt.iter().enumerate() {
        let v = active_dials.design_variance;
        let m = active_dials.motion_intensity;
        let dn = active_dials.visual_density;
        let mut dials = match i {
            0 => DesignDials::clamped(v, tune(m.min(5)), dn),
            1 => DesignDials::clamped((v + 3).min(10), tune((m + 3).min(10)), dn.saturating_sub(1)),
            _ => DesignDials::clamped(v, tune(m), (dn + 2).min(10)),
        };
        if heavy {
            dials.motion_intensity = dials.motion_intensity.max(8);
        }
        if tune_brief_motion && new_site {
            // The dial and the page shape's motion tier agree: the niche caps it, the prompt pins it.
            dials.motion_intensity = shape_brief.clamp_dial(dials.motion_intensity);
        }
        let text = crate::directions::write(d, &ctx, dials.motion_intensity);
        let mut motion = text.motion;
        if heavy {
            // A quiet direction's "nothing moves on scroll" would contradict the brief.
            for quiet in ["; nothing moves on scroll", "; zero decorative motion", "; no scroll-triggered choreography"] {
                motion = motion.replace(quiet, "");
            }
            motion.push_str(
                ". The brief asks for heavy motion: keep this direction's character but make the page move with scroll (pinned, scrubbed sequences that show the subject's own objects working, hand-offs between sections); every still frame stays readable and reduced-motion shows the complete content",
            );
        }
        let letter = ['A', 'B', 'C'][i];
        directions.push(CreativeDirection {
            hero_material: String::new(),
            radius: d.radius.to_string(),
            extra_font_url: d.extra_font_url.clone(),
            archetype: d.arch.key.to_string(),
            nav: None,
            footer: None,
            type_tokens: type_tokens(&d.typography.scale_ratio),
            index: i as u8 + 1,
            title: format!("Direction {letter} — {} ({})", best_industry.name, d.arch.name),
            style_family: d.arch.style_family.into(),
            layout_composition: text.layout,
            visual_identity: text.identity,
            typography: d.typography.clone(),
            palette: d.palette.clone(),
            content_treatment: text.content,
            motion_choreography: motion,
            dials,
            rationale: crate::directions::rationale(d, &signals, &subject, &best_industry.name),
            mechanics: Vec::new(),
            page_shape: None,
        });
    }
    if new_site {
        let ins: Vec<crate::directions::page_shape::ShapeIn> = dealt
            .iter()
            .zip(&directions)
            .map(|(d, c)| crate::directions::page_shape::ShapeIn {
                hero_material: d.arch.hero_material,
                composition: d.arch.composition,
                tone: d.tone,
                motion_dial: c.dials.motion_intensity,
            })
            .collect();
        for (c, shape) in directions.iter_mut().zip(crate::directions::page_shape::deal_shapes(&shape_brief, &ins)) {
            c.page_shape = Some(shape);
        }
    }
    let mut active_dials = active_dials;
    active_dials.motion_intensity = tune(active_dials.motion_intensity);
    let compositions: Vec<&str> = dealt.iter().map(|d| d.arch.composition).collect();

    for d in &mut directions {
        verify_palette(&mut d.palette);
    }
    attach_mechanics(&mut directions, query, &signals.niches, mode, existing_dna, variant, &compositions, recent_ids);
    // The direction and its hero must describe the same first screen. When no fitting hero of the
    // suggested composition exists, the direction takes the dealt hero's composition instead of
    // promising one layout and handing over another.
    let mut compositions: Vec<String> = compositions.iter().map(|c| c.to_string()).collect();
    for (d, comp) in directions.iter_mut().zip(compositions.iter_mut()) {
        let Some(h) = d.mechanics.first() else { continue };
        if h.composition.is_empty() || h.composition == *comp {
            continue;
        }
        let said = format!("First screen: {} composition", comp);
        let said_split = format!("{} (a split, chosen only because two views must be read side by side)", said);
        let now = format!(
            "First screen: {} composition (no fitting {} hero for this trade, so the direction follows the dealt hero {})",
            h.composition, comp, h.id
        );
        d.rationale = if d.rationale.contains(&said_split) { d.rationale.replacen(&said_split, &now, 1) } else { d.rationale.replacen(&said, &now, 1) };
        *comp = h.composition.clone();
    }
    verify_palette(&mut best_industry.palette);

    let mut workflow_guidance: Vec<String> = match mode {
        OperatingMode::NewWebsite => vec![
            "Mode A (New Website): the user's prompt wins over everything here. The 3 directions are structural raw material (layout, content treatment, dials), not finished looks; the visual world comes from art direction.".into(),
            "DISTINCTIVENESS: Read `distinctiveness` first. If `match_confidence` is `none` or `weak`, the industry profile is only a fallback; build the visual world from the brief's place and material cues instead, and do not ship any listed category cliché unless the brief asks for it.".into(),
            "ART DIRECTION (load `skills/layoza-art-direction`): commit to ONE specific world. Derive the world from far (place, notation, print traditions), keep the subject up close (the trade's object as the signature interaction, at scale), rule out the category reflex and its obvious opposite, and put the visitor's real action in the first viewport. Let colour own whole sections. Decide motion and tools per design; nothing is mandatory.".into(),
            "ENGINEERING LESSONS: sticky headers need `html { scroll-padding-top: <header height + 1rem> }` (WCAG 2.4.11); web fonts need metric-adjusted fallback `@font-face` (size-adjust / ascent-override) or the font swap re-wraps headings and fails CLS; scroll reveals must end visible.".into(),
            format!("HERO: Layoza does not fix the first-screen composition; you choose it, from the subject and the brief. The directions suggest {} (each with its reason in `rationale`), and the dealt hero mechanic is one fit for that composition, not the only one. Give the three directions three different first screens, do not default to headline-left / visual-right (a split needs a stated reason), and record `HERO:` with the reason in `.layoza/direction.md`.", compositions.join(" / ")),
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

    if signals.constraints.iter().any(|c| c.starts_with("rtl")) {
        workflow_guidance.insert(
            0,
            "RTL: the brief names a right-to-left language. Set `<html dir=\"rtl\" lang=\"..\">`, use logical CSS properties only (margin-inline-start, padding-inline, inset-inline-end, text-align: start, border-inline-start), mirror the layout and directional icons and motion (an entrance from the start edge comes from the right), keep numerals, code and brand marks LTR where they belong, and pick fonts that ship the script.".into(),
        );
    }
    if match_score == 0 && matches!(mode, OperatingMode::NewWebsite | OperatingMode::FullRedesign) {
        workflow_guidance.insert(
            0,
            format!(
                "BRIEF OBJECTS: no catalogue trade matched, so the subject is read from the brief's own words: {}. Map every direction and every dealt mechanic onto these objects (the hero shows one of them, sections are built from their real names, numbers and records), never onto the catalogue's example objects. Call to action: {}.",
                if signals.brief_objects.is_empty() { "none read (ask the user what the page shows)".to_string() } else { signals.brief_objects.join(", ") },
                signals.brief_action.clone().unwrap_or_else(|| "the brief names none; ask the user, do not assume booking or contact".into())
            ),
        );
    }
    if !signals.brief_colors.is_empty() && matches!(mode, OperatingMode::NewWebsite | OperatingMode::FullRedesign) {
        workflow_guidance.insert(
            0,
            format!(
                "BRIEF COLOURS: the brief names its own colours ({}). Use them as the palette in every direction (check WCAG pairs); the direction palettes below are fallbacks only.",
                signals.brief_colors.join(", ")
            ),
        );
    }
    if !signals.brief_fonts.is_empty() && matches!(mode, OperatingMode::NewWebsite | OperatingMode::FullRedesign) {
        workflow_guidance.insert(
            0,
            format!(
                "BRIEF FONTS: the brief names its own type ({}). Build every direction's type system from these (pick the display, body and mono roles among them); the direction pairings below are fallbacks only.",
                signals.brief_fonts.join(", ")
            ),
        );
    }
    let heroes: Vec<(String, String, String)> = directions
        .iter()
        .map(|d| {
            d.mechanics
                .iter()
                .find(|m| m.kind == crate::mechanics::MechanicKind::Hero)
                .map(|m| (m.id.clone(), format!("{} {}", m.hero_material, m.name), m.composition.clone()))
                .unwrap_or_else(|| (d.style_family.clone(), d.layout_composition.clone(), String::new()))
        })
        .collect();
    for (d, m) in directions.iter_mut().zip(crate::process::hero_materials(&heroes)) {
        d.hero_material = m;
    }
    for (d, dealt) in directions.iter_mut().zip(&dealt) {
        if d.hero_material.trim().is_empty() {
            d.hero_material = dealt.arch.hero_material.to_string();
        }
    }
    let new_look = matches!(mode, OperatingMode::NewWebsite | OperatingMode::FullRedesign);
    let component = crate::process::is_component_brief(raw_query);
    let custom_route = if new_look && !component {
        crate::process::custom_route_reason(raw_query, match_score, signals.brief_objects.len()).map(|r| crate::process::custom_route_protocol(&r))
    } else {
        None
    };
    if new_look && !component {
        workflow_guidance.push(
            "DIRECTIONS: sketch more candidates than you need, from three strategies: (a) what the strongest designer for this subject would make with no budget limit; (b) a verified real reference in the same medium, screenshotted and studied (structure, not pixels); (c) a random roll from Layoza's libraries (these three directions, or `variant`). Keep the 3 that are strongest, most different from each other and not in the look ledger. One shared family (e.g. warm paper serif) is fine; three from the same family is the failure. At most one may be overcooked (the whole page as one prop or metaphor). Each names its `hero_material`; gather the images the content needs before designing. Present them and stop for the user's pick; record it in .layoza/direction-approved.md.".into(),
        );
        workflow_guidance.push(PAGE_SHAPE_RULE.into());
        workflow_guidance.push("FORM: for each page or section answer narrative role, viewing distance, visual temperature, content capacity and visual motif before styling it; at delivery add one line per section saying which content the form came from.".into());
        workflow_guidance.push("COPY: specific nouns from the brief, no startup clichés, no invented metrics, reviews or logo walls (use honest placeholders such as `[customer quote — to supply]`), and call-to-action verbs taken from what the visitor actually does.".into());
        workflow_guidance.push(format!(
            "SELF-CRITIQUE before QA: score {} from 1 to 5; any score below 3 means revise first. Put the stamp `{}` as the first line of the main CSS (your direction, structure, hero, palette, type and scores).",
            crate::process::CRITIQUE_AXES.join(", "),
            crate::process::stamp_line("<A|B|C>", "<style_family>", "<hero id>", "<palette id>", "<type id>", &[4, 4, 4, 4, 4, 4])
        ));
    }
    if crate::process::names_checkable_facts(raw_query) {
        workflow_guidance.insert(0, "FACT CHECK: the brief names products, versions, people or events. Verify each (web search) or ask the user before using it, and record it with its source in .layoza/product-facts.md. Never invent specs, dates or prices.".into());
    }
    if let Some(c) = &custom_route {
        workflow_guidance.insert(0, c.clone());
    }
    if component {
        workflow_guidance.insert(
            0,
            "COMPONENT SCOPE: the brief is one component, not a page. Skip page-level steps (hero, three directions, art loop); keep the project's tokens, every state (default, hover, focus-visible, active, disabled, loading, error, empty), keyboard and screen-reader behaviour, reduced-motion rules and RTL with logical properties. Stamp it with `scope: component`.".into(),
        );
    }
    DesignRecommendation {
        scope: if component { "component".into() } else { "page".into() },
        custom_route,
        rotation: None,
        taste: Vec::new(),
        uniqueness: None,
        understood_brief: Some(understood_brief),
        mode,
        query: raw_query.to_string(),
        distinctiveness: distinctiveness_for(query, &best_industry.slug, &signals),
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

        // Niche drives the palette family (B and C are dealt from the brief's niche tags)
        let pool = crate::directions::palette_pool();
        let tagged = |id: &str, tag: &str| pool.iter().any(|e| e.palette.id == id && e.tags.contains(&tag));
        for k in 1..3 {
            assert!(tagged(&sea.directions[k].palette.id, "marine"), "{}", sea.directions[k].palette.id);
            assert!(tagged(&fashion.directions[k].palette.id, "textile"), "{}", fashion.directions[k].palette.id);
            assert!(tagged(&dash.directions[k].palette.id, "data"), "{}", dash.directions[k].palette.id);
        }

        // A task UI never gets scroll choreography or high motion; a kinetic studio keeps it
        for d in &dash.directions {
            assert!(d.dials.motion_intensity <= 4, "{}: {}", d.title, d.dials.motion_intensity);
            assert!(!d.motion_choreography.contains("ScrollTrigger"), "{}", d.title);
        }
        assert!(kinetic.directions[1].dials.motion_intensity >= 9);
    }

    #[test]
    fn no_fixed_hero_and_variant_redeals() {
        let q = "open source mcp server site for coding agents";
        let r0 = rec(q);
        assert!(r0.directions.iter().all(|d| !d.layout_composition.contains("7:5")));
        assert!(r0.workflow_guidance.iter().any(|g| g.starts_with("HERO:")));
        let looks = |r: &DesignRecommendation| r.directions.iter().map(|d| (d.palette.id.clone(), d.typography.display_font.clone())).collect::<Vec<_>>();
        assert_eq!(looks(&rec(q)), looks(&r0));
        let differs = (1..6u64).filter(|v| looks(&recommend_design_variant(q, OperatingMode::NewWebsite, None, None, *v)) != looks(&r0)).count();
        assert!(differs >= 3, "{differs}");
    }

    #[test]
    fn negated_parts_do_not_steer_the_match() {
        let b = "Highly animated marketing site for an open-source MCP server for coding agents on GitHub and npm.\nAbsolutely avoid:\n* Overused dashboard mockups\n* Generic gradient blobs\nDo not use a static generic dashboard image.\nRespect prefers-reduced-motion.";
        let a = affirmed_brief(b);
        assert!(!a.contains("dashboard"), "{a}");
        assert!(a.contains("MCP server"), "{a}");
        let r = rec(b);
        assert_eq!(r.brief_signals.surface, Surface::DevTool);
        assert!(r.directions.iter().all(|d| d.dials.motion_intensity >= 8), "{:?}", r.brief_signals);
        assert!(r.brief_signals.constraints.iter().any(|c| c == "reduced-motion fallback"));
        // a plain "reduced motion" ask without heavy motion still calms the page
        assert!(rec("bakery site, reduced motion please").directions.iter().all(|d| d.dials.motion_intensity <= 1));
        // flattened bullets
        assert!(!affirmed_brief("Avoid: * dashboard mockups * neon").contains("dashboard"));
    }

    #[test]
    fn brief_fonts_are_read_and_negated_fonts_are_not() {
        let r = rec("open-source cli site. Fonts: Space Grotesk, Geist Mono and IBM Plex Mono. Avoid Inter.");
        assert_eq!(r.brief_signals.brief_fonts, vec!["Space Grotesk", "Geist Mono", "IBM Plex Mono"]);
        assert!(r.workflow_guidance.iter().any(|g| g.starts_with("BRIEF FONTS:")));
        assert!(rec("a calm bakery site").brief_signals.brief_fonts.is_empty());
        // Font names inside ordinary words are not fonts
        assert!(rec("international interiors studio").brief_signals.brief_fonts.is_empty());
    }

    #[test]
    fn brief_colors_and_soft_caps() {
        let r = rec("highly animated open source mcp site, accessibility and performance matter; navy #080B13, lime #c8ff61, #080b13 again");
        assert_eq!(r.brief_signals.brief_colors, vec!["#080B13".to_string(), "#C8FF61".to_string()]);
        assert!(r.workflow_guidance[0].contains("#C8FF61"));
        assert!(r.directions.iter().all(|d| d.dials.motion_intensity >= 8));
        let seniors = rec("highly animated site for seniors");
        assert!(seniors.directions.iter().all(|d| d.dials.motion_intensity <= 5));
    }

    #[test]
    fn heavy_motion_words_raise_every_direction() {
        let r = rec("open-source mcp server site, highly animated and cinematic");
        assert!(r.active_dials.motion_intensity >= 8, "{}", r.active_dials.motion_intensity);
        for d in &r.directions {
            assert!(d.dials.motion_intensity >= 8, "{}: {}", d.title, d.dials.motion_intensity);
            assert!(d.motion_choreography.contains("heavy motion"), "{}", d.title);
            assert!(!d.motion_choreography.contains("nothing moves on scroll"), "{}", d.title);
        }
        // "no animation" still wins, and a task UI keeps its cap.
        let calm = rec("open-source mcp server site, cinematic but no animation please");
        assert!(calm.directions.iter().all(|d| d.dials.motion_intensity <= 1));
        let dash = rec("immersive analytics dashboard");
        assert!(dash.directions.iter().all(|d| d.dials.motion_intensity <= 4));
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
    fn devtool_briefs_route_to_ai_devtools_with_devtool_directions() {
        for q in [
            "marketing site for an open-source MCP server and AI agent skills that make coding agents design better",
            "landing page for a Rust CLI that lints Terraform",
            "docs and homepage for a TypeScript SDK for our payments API",
            "developer tools startup homepage for an LLM observability library",
        ] {
            let r = rec(q);
            assert_eq!(r.matched_industry.slug, "ai-devtools", "{q}");
            assert_eq!(r.brief_signals.surface, Surface::DevTool, "{q}");
            assert!(r.brief_signals.niche_tags.iter().any(|t| t == "code"), "{q}: {:?}", r.brief_signals.niche_tags);
            assert!(!r.directions[2].layout_composition.contains("try a frame"), "{q}: generic Direction C copy");
        }
        let r = rec("marketing site for an open-source MCP server and AI agent skills that make coding agents design better");
        let ranked = rank_industries(&r.query, &catalog_industries());
        assert!(ranked[0].0 >= ranked[1].0 + 9, "weak lead: {:?}", ranked.iter().take(3).map(|(s, i)| (*s, i.slug.clone())).collect::<Vec<_>>());
    }

    #[test]
    fn devtools_keywords_do_not_steal_other_industries() {
        for (q, not) in [
            ("independent travel agent booking honeymoon trips", "ai-devtools"),
            ("estate agent listings for family homes in Leeds", "ai-devtools"),
            ("seafood restaurant on the harbour with a daily catch menu", "ai-devtools"),
            ("creative agency portfolio for a branding studio", "ai-devtools"),
        ] {
            let r = rec(q);
            assert_ne!(r.matched_industry.slug, not, "{q}");
            assert_ne!(r.brief_signals.surface, Surface::DevTool, "{q}");
        }
        assert_eq!(rec("bakery operations dashboard for production planning").brief_signals.surface, Surface::ProductUi);
    }

    #[test]
    fn every_variant_redeals_all_three_directions() {
        for q in [
            "seafood restaurant on the harbour with a daily catch menu",
            "bakery sales dashboard",
            "linen store",
            "kinetic creative agency",
            "SaaS dev tool",
            "law firm",
            "kids' museum",
        ] {
            let at = |v: u64| recommend_design_variant(q, OperatingMode::NewWebsite, None, None, v);
            let key = |d: &CreativeDirection| (d.title.clone(), d.typography.display_font.clone(), d.palette.id.clone());
            let r0 = at(0);
            let mut prev = r0.directions.iter().map(key).collect::<Vec<_>>();
            for v in 1..=6 {
                let r = at(v);
                let now = r.directions.iter().map(key).collect::<Vec<_>>();
                for slot in 0..3 {
                    let (a, b) = (&r0.directions[slot], &r.directions[slot]);
                    assert_ne!(a.typography.display_font, b.typography.display_font, "{q} v{v} slot {slot} font");
                    assert_ne!(a.palette.id, b.palette.id, "{q} v{v} slot {slot} palette");
                    assert_ne!(a.layout_composition, b.layout_composition, "{q} v{v} slot {slot} layout");
                }
                assert_ne!(now, prev, "{q} v{v} repeats v{}", v - 1);
                prev = now;
                // Within a set: three different first-screen compositions, fonts and palettes.
                let comps: std::collections::HashSet<_> = r.directions.iter().map(|d| d.mechanics[0].composition.clone()).collect();
                let fonts: std::collections::HashSet<_> = r.directions.iter().map(|d| d.typography.display_font.clone()).collect();
                let pals: std::collections::HashSet<_> = r.directions.iter().map(|d| d.palette.id.clone()).collect();
                assert_eq!((fonts.len(), pals.len()), (3, 3), "{q} v{v}");
                assert!(comps.len() >= 2, "{q} v{v} hero compositions {comps:?}");
            }
        }
    }

    #[test]
    fn each_direction_states_its_dealt_hero_s_composition() {
        for q in ["law firm in Boston", "kids museum with hands-on exhibits", "seafood restaurant by the harbour", "linen store", "open-source cli"] {
            for v in 0..6 {
                let r = recommend_design_variant(q, OperatingMode::NewWebsite, None, None, v);
                for d in &r.directions {
                    let Some(h) = d.mechanics.first() else { continue };
                    let said = format!("First screen: {} composition", h.composition);
                    assert!(d.rationale.contains(&said), "{q} v{v} {}: hero {} is {} but rationale says: {}", d.title, h.id, h.composition, d.rationale);
                }
            }
        }
    }

    #[test]
    fn copy_and_rationale_come_from_the_brief() {
        let law = rec("law firm");
        let museum = rec("kids' museum");
        for (r, word) in [(&law, "law firm"), (&museum, "kids")] {
            for d in &r.directions {
                assert!(d.rationale.contains(word), "{}: {}", d.title, d.rationale);
                assert!(d.layout_composition.to_lowercase().contains(word) || d.content_treatment.to_lowercase().contains(word), "{}: {}", d.title, d.layout_composition);
                assert!(d.rationale.contains(&d.typography.display_font) && d.rationale.contains(&d.palette.id), "{}", d.rationale);
            }
        }
        for k in 0..3 {
            assert_ne!(law.directions[k].layout_composition, museum.directions[k].layout_composition);
            assert_ne!(law.directions[k].rationale, museum.directions[k].rationale);
        }
    }

    #[test]
    fn unmatched_briefs_are_read_from_their_own_words() {
        let r = rec("law firm");
        assert_eq!(r.match_confidence, "none");
        assert_eq!(r.matched_industry.slug, "brief-derived");
        assert!(r.matched_industry.name.starts_with("Law firm"), "{}", r.matched_industry.name);
        assert!(r.matched_industry.layout_archetype.contains("case file"), "{}", r.matched_industry.layout_archetype);
        let cl = r.distinctiveness.category_cliches.join(" | ");
        assert!(cl.contains("gavel"), "{cl}");
        let kids = rec("kids' museum");
        let kcl = kids.distinctiveness.category_cliches.join(" | ");
        assert!(kcl.contains("mascot"), "{kcl}");
        assert_ne!(cl, kcl);
        // A museum brief no longer forces Playfair
        assert!(kids.directions.iter().all(|d| d.typography.display_font != "Playfair Display"));
    }

    #[test]
    fn directions_print_their_own_type_not_the_industry_pairing() {
        let r = rec("seafood restaurant on the harbour with a daily catch menu");
        assert_eq!(r.directions[0].typography.id, r.matched_industry.typography_pairing.id);
        assert_ne!(r.directions[1].typography.id, r.matched_industry.typography_pairing.id);
        assert_ne!(r.directions[2].typography.id, r.matched_industry.typography_pairing.id);
    }

    #[test]
    fn design_system_and_deal_read_one_niche_signal() {
        use crate::mechanics::{deal_mechanics_with, get_mechanic, DealOptions};
        for q in ["vineyard wedding", "kids coding camp", "bakery dashboard", "law firm", "harbour seafood restaurant", "funeral home"] {
            let r = rec(q);
            let d = deal_mechanics_with(&DealOptions { brief: q.into(), count: 2, ..Default::default() });
            assert_eq!(r.brief_signals.niches, d.context.niches, "{q}: design-system and deal disagree");
            // Every direction's hero is dealt for those same niches: never another trade's own object.
            for dir in &r.directions {
                let hero = get_mechanic(&dir.mechanics[0].id).unwrap();
                let primary = hero.niches.first().cloned().unwrap_or_default();
                let narrow = ["store", "restaurant", "real-estate", "events", "travel", "legal", "museum"];
                assert!(
                    !narrow.contains(&primary.as_str()) || r.brief_signals.niches.contains(&primary) || hero.niches.iter().any(|n| r.brief_signals.niches.contains(n)),
                    "{q}: hero {} belongs to {primary}",
                    hero.id
                );
            }
        }
    }

    /// The subject phrase keeps the brief's last four content words, so "funeral home helping
    /// families arrange services" reads "helping families arrange services". The fix belongs in
    /// `directions::subject_phrase` (agent B's file); kept here, ignored, until it lands.
    #[test]
    fn ledger_type_families_are_not_dealt_again() {
        let q = "independent brand and digital design agency";
        let first = recommend_design(q, OperatingMode::NewWebsite, None, None);
        let recent = looks_of(&first, "r1");
        let next = recommend_design_avoiding("online store selling smartphones and phone accessories", OperatingMode::NewWebsite, None, None, 0, &recent);
        let used: Vec<String> = first.directions.iter().map(|d| crate::directions::family(&d.typography.display_font)).collect();
        for d in &next.directions {
            assert!(!used.contains(&crate::directions::family(&d.typography.display_font)), "{} reused", d.typography.display_font);
        }
    }

    #[test]
    fn subject_phrase_keeps_the_trade_noun() {
        let funeral = rec("funeral home helping families arrange services");
        assert!(funeral.matched_industry.layout_archetype.to_lowercase().contains("funeral home"), "{}", funeral.matched_industry.layout_archetype);
    }

    #[test]
    fn unknown_trades_are_written_from_their_own_nouns_and_verbs() {
        let funeral = rec("funeral home helping families arrange services");
        let water = rec("municipal water utility where residents report leaks and pay bills");
        let records = rec("county records archive of land deeds and maps");
        for r in [&funeral, &water, &records] {
            assert_eq!(r.matched_industry.slug, "brief-derived");
            assert!(!r.matched_industry.layout_archetype.contains("make first contact"), "{}", r.matched_industry.layout_archetype);
        }
        assert!(funeral.matched_industry.layout_archetype.contains("chapel") || funeral.matched_industry.layout_archetype.contains("funeral"), "{}", funeral.matched_industry.layout_archetype);
        assert!(funeral.matched_industry.first_viewport_priority.contains("arrange"), "{}", funeral.matched_industry.first_viewport_priority);
        assert!(water.matched_industry.first_viewport_priority.contains("report"), "{}", water.matched_industry.first_viewport_priority);
        // No verb in the brief: no guessed booking, the agent is told to ask.
        // A records archive is now a known trade (its own action), so only the generic fallback is barred.
        assert!(!records.matched_industry.first_viewport_priority.contains("call, book"));
        assert_ne!(funeral.matched_industry.layout_archetype, water.matched_industry.layout_archetype);
        let heroes = |r: &DesignRecommendation| r.directions.iter().map(|d| d.mechanics[0].id.clone()).collect::<Vec<_>>();
        assert_ne!(heroes(&funeral), heroes(&water));
        assert_eq!(funeral.brief_signals.brief_objects[0], "funeral home");
    }

    #[test]
    fn rtl_briefs_get_logical_properties_and_mirrored_layout() {
        for q in ["Arabic bakery site in Riyadh", "law office, lang=\"he\"", "Urdu poetry journal", "bookshop with dir=rtl"] {
            assert!(is_rtl_brief(q), "{q}");
            let r = rec(q);
            assert!(r.workflow_guidance.iter().any(|g| g.starts_with("RTL:") && g.contains("logical CSS")), "{q}");
        }
        for q in ["harbour seafood restaurant", "lang=\"en\" art gallery", "farm shop", "heron watching tours"] {
            assert!(!is_rtl_brief(q), "{q}");
        }
    }

    #[test]
    fn the_look_ledger_steers_the_next_run_away_from_recent_looks() {
        let dir = std::env::temp_dir().join(format!("layoza-ledger-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let q = "harbour seafood restaurant with a daily catch board";
        assert!(crate::baseline::recent_looks(&dir, crate::baseline::RECENT_LOOK_RUNS).is_empty());
        let first = rec(q);
        crate::baseline::record_looks(&dir, &looks_of(&first, "run-1")).unwrap();
        let recent = crate::baseline::recent_looks(&dir, crate::baseline::RECENT_LOOK_RUNS);
        assert_eq!(recent.len(), first.directions.len());
        let second = recommend_design_avoiding(q, OperatingMode::NewWebsite, None, None, 0, &recent);
        let heroes = |r: &DesignRecommendation| r.directions.iter().map(|d| d.mechanics[0].id.clone()).collect::<Vec<_>>();
        let palettes = |r: &DesignRecommendation| r.directions.iter().map(|d| d.palette.id.clone()).collect::<Vec<_>>();
        assert_ne!(heroes(&first), heroes(&second));
        assert_ne!(palettes(&first), palettes(&second));
        assert!(second.workflow_guidance.iter().any(|g| g.starts_with("LOOK LEDGER")));
        // Without a ledger the deal is the plain, reproducible one.
        assert_eq!(heroes(&recommend_design_avoiding(q, OperatingMode::NewWebsite, None, None, 0, &[])), heroes(&first));
        let _ = std::fs::remove_dir_all(&dir);
    }
}

#[cfg(test)]
mod direction_uniqueness_tests {
    use super::*;

    #[test]
    fn three_directions_share_no_nav_hero_footer_archetype_or_palette() {
        for brief in [
            "family funeral home with pre-planning",
            "neighbourhood bakery with a daily bread board",
            "b2b analytics saas dashboard",
            "regional water utility outage and billing portal",
            "tattoo studio booking and flash gallery",
            "harbour seafood restaurant with daily catch menu",
        ] {
            for variant in [0, 3] {
                let rec = recommend_design_variant(brief, OperatingMode::NewWebsite, None, None, variant);
                let looks = looks_of(&rec, "t");
                assert_eq!(looks.len(), 3, "{brief}");
                type Get = fn(&crate::baseline::Look) -> Option<String>;
                let slots: [(&str, Get); 5] = [
                    ("nav", |l| l.nav.clone()),
                    ("hero", |l| l.hero.clone()),
                    ("footer", |l| l.footer.clone()),
                    ("archetype", |l| l.archetype.clone()),
                    ("palette", |l| l.palette.clone()),
                ];
                for (name, get) in slots {
                    let ids: Vec<String> = looks.iter().filter_map(get).collect();
                    let mut uniq = ids.clone();
                    uniq.sort();
                    uniq.dedup();
                    assert_eq!(uniq.len(), ids.len(), "{brief} v{variant}: directions share a {name}: {ids:?}");
                }
            }
        }
    }
}
