// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
//! Layoza mechanics catalogue and deal.
//!
//! A mechanic is one specific, buildable hero / section / transition idea (blueprint, scroll-progress
//! motion, still check, mobile restage, accessibility end state, failure modes, CSS and JS paths).
//! `deal_mechanics` picks a coherent, deterministic set per page so mid-page sections stop falling
//! back to stock patterns. Layoza-original catalogue, written from first principles; tool-agnostic.
//!
//! Mode-aware: for a new site (Mode A/D) the brief drives niche, mood and intensity. For an existing
//! site (Mode B/C) the inspected site's motion intensity and stack cap what may be dealt, so a deal
//! never restyles the site.

use crate::dna::SiteDna;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::sync::OnceLock;

const CATALOGUE_JSON: &str = include_str!("mechanics/catalogue.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MechanicKind {
    Hero,
    Section,
    Transition,
}

impl MechanicKind {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "hero" => Some(Self::Hero),
            "section" => Some(Self::Section),
            "transition" | "micro" => Some(Self::Transition),
            _ => None,
        }
    }
}

/// Motion intensity tier. Ordered: calm < bold < wild.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Intensity {
    Calm,
    Bold,
    Wild,
}

impl Intensity {
    /// Accepts `calm|bold|wild` (aliases `simple`, `subtle`, `minimal`, `medium`, `max`) or a 0-10 dial.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim().to_ascii_lowercase();
        if let Ok(n) = s.parse::<u8>() {
            return Some(Self::from_dial(n));
        }
        match s.as_str() {
            "calm" | "simple" | "subtle" | "minimal" | "low" => Some(Self::Calm),
            "bold" | "medium" | "mid" => Some(Self::Bold),
            "wild" | "max" | "high" => Some(Self::Wild),
            _ => None,
        }
    }
    /// 0-3 calm, 4-7 bold, 8-10 wild (the Layoza motion dial).
    pub fn from_dial(n: u8) -> Self {
        match n {
            0..=3 => Self::Calm,
            4..=7 => Self::Bold,
            _ => Self::Wild,
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Calm => "calm",
            Self::Bold => "bold",
            Self::Wild => "wild",
        }
    }
}

/// What a mechanic needs to build. Ordered: css < js < webgl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Css,
    Js,
    Webgl,
}

impl Tier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Css => "css",
            Self::Js => "js",
            Self::Webgl => "webgl",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StackNotes {
    /// CSS-only path (scroll/view timelines, transitions, sticky, :has(), IntersectionObserver fallback).
    pub css: String,
    /// Scripted path (IntersectionObserver, WAAPI, rAF progress, Canvas, WebGL, or the project's library).
    pub js: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mechanic {
    pub id: String,
    pub name: String,
    pub kind: MechanicKind,
    /// Mechanism family; a deal never uses two mechanics of one family.
    pub family: String,
    /// Page role (`hero`, `transition`/`micro`, or a section role such as `menu`, `pricing`, `process`).
    pub role: String,
    pub niches: Vec<String>,
    pub moods: Vec<String>,
    pub intensity: Intensity,
    pub tier: Tier,
    /// Hero only: the headline and an art/3D object physically interlock.
    pub headline_interlock: bool,
    pub blueprint: String,
    /// What moves, mapped to scroll progress (never velocity).
    pub motion: String,
    /// What a screenshot at 50% of the section's scroll range must show.
    pub still_check: String,
    /// How it restages at 375px.
    pub mobile: String,
    /// Reduced-motion end state, keyboard and focus.
    pub a11y: String,
    /// Failure modes.
    pub avoid: String,
    pub stack_notes: StackNotes,
}

/// The niche vocabulary every entry's `niches` is drawn from.
pub const NICHES: &[&str] = &[
    "store", "restaurant", "local-business", "saas", "dashboard", "agency", "kinetic", "3d", "editorial",
    "portfolio", "finance", "health", "education", "events", "real-estate", "travel", "nonprofit", "docs",
    "landing", "professional", "any",
];

/// The parsed catalogue (embedded at compile time; parsed once).
pub fn catalogue() -> &'static [Mechanic] {
    static CAT: OnceLock<Vec<Mechanic>> = OnceLock::new();
    CAT.get_or_init(|| serde_json::from_str(CATALOGUE_JSON).expect("embedded mechanics catalogue is valid JSON"))
}

pub fn get_mechanic(id: &str) -> Option<&'static Mechanic> {
    let id = id.trim();
    catalogue().iter().find(|m| m.id.eq_ignore_ascii_case(id))
}

/// Lists mechanics, optionally filtered by niche (entries tagged `any` match every niche), kind and
/// intensity (exact tier).
pub fn list_mechanics(niche: Option<&str>, kind: Option<MechanicKind>, intensity: Option<Intensity>) -> Vec<&'static Mechanic> {
    let niche = niche.map(normalize_niche);
    catalogue()
        .iter()
        .filter(|m| kind.is_none_or(|k| m.kind == k))
        .filter(|m| intensity.is_none_or(|i| m.intensity == i))
        .filter(|m| niche.as_deref().is_none_or(|n| m.niches.iter().any(|x| x == n || x == "any")))
        .collect()
}

/// Maps common spellings onto the niche vocabulary (`real estate` -> `real-estate`, `shop` -> `store`).
pub fn normalize_niche(raw: &str) -> String {
    let s = raw.trim().to_ascii_lowercase().replace([' ', '_'], "-");
    match s.as_str() {
        "shop" | "ecommerce" | "e-commerce" | "retail" => "store".into(),
        "cafe" | "food" | "bar" => "restaurant".into(),
        "local" | "bakery" | "trades" => "local-business".into(),
        "app" | "product" | "startup" => "saas".into(),
        "admin" | "product-ui" => "dashboard".into(),
        "studio" => "agency".into(),
        "webgl" | "three" | "threejs" => "3d".into(),
        "magazine" | "blog" | "journal" => "editorial".into(),
        "fintech" | "bank" => "finance".into(),
        "clinic" | "wellness" | "medical" => "health".into(),
        "legal" | "law" | "law-firm" | "corporate" | "consulting" | "consultancy" | "accountancy" | "services" => "professional".into(),
        "school" | "course" | "learning" => "education".into(),
        "event" | "festival" | "conference" => "events".into(),
        "property" | "realestate" => "real-estate".into(),
        "hotel" | "tourism" => "travel".into(),
        "charity" | "non-profit" | "ngo" => "nonprofit".into(),
        "documentation" => "docs".into(),
        _ => s,
    }
}

// ---------------------------------------------------------------------------------------------
// Brief reading
// ---------------------------------------------------------------------------------------------

/// Brief word/phrase -> niche.
const NICHE_WORDS: &[(&str, &str)] = &[
    ("shop", "store"), ("store", "store"), ("boutique", "store"), ("ecommerce", "store"), ("e-commerce", "store"),
    ("retail", "store"), ("apparel", "store"), ("fashion", "store"), ("skincare", "store"), ("cosmetics", "store"),
    ("sneakers", "store"), ("furniture", "store"), ("merch", "store"), ("catalogue", "store"), ("catalog", "store"),
    ("womenswear", "store"), ("menswear", "store"), ("jewellery", "store"), ("jewelry", "store"),
    ("restaurant", "restaurant"), ("bistro", "restaurant"), ("cafe", "restaurant"), ("café", "restaurant"),
    ("coffee", "restaurant"), ("bar", "restaurant"), ("dining", "restaurant"), ("chef", "restaurant"),
    ("eatery", "restaurant"), ("pizzeria", "restaurant"), ("ramen", "restaurant"), ("seafood", "restaurant"),
    ("wine", "restaurant"), ("brewery", "restaurant"), ("tasting menu", "restaurant"), ("bakery", "restaurant"),
    ("bakery", "local-business"), ("baker", "local-business"), ("florist", "local-business"), ("salon", "local-business"),
    ("barber", "local-business"), ("plumber", "local-business"), ("electrician", "local-business"),
    ("garage", "local-business"), ("laundry", "local-business"), ("neighbourhood", "local-business"),
    ("neighborhood", "local-business"), ("local", "local-business"), ("tailor", "local-business"),
    ("repair", "local-business"), ("pottery", "local-business"), ("bookshop", "local-business"),
    ("saas", "saas"), ("software", "saas"), ("platform", "saas"), ("startup", "saas"), ("api", "saas"),
    ("b2b", "saas"), ("integration", "saas"), ("devtool", "saas"),
    ("dashboard", "dashboard"), ("admin", "dashboard"), ("analytics", "dashboard"), ("console", "dashboard"),
    ("internal tool", "dashboard"), ("crm", "dashboard"), ("back office", "dashboard"), ("monitoring", "dashboard"),
    ("agency", "agency"), ("studio", "agency"), ("branding", "agency"),
    ("production company", "agency"),
    ("kinetic", "kinetic"), ("motion design", "kinetic"), ("motion", "kinetic"), ("animation studio", "kinetic"),
    ("3d", "3d"), ("webgl", "3d"), ("three.js", "3d"), ("threejs", "3d"), ("immersive", "3d"), ("ribbon", "3d"),
    ("sculpture", "3d"), ("shader", "3d"), ("spatial", "3d"),
    ("magazine", "editorial"), ("journal", "editorial"), ("editorial", "editorial"), ("publication", "editorial"),
    ("blog", "editorial"), ("newsletter", "editorial"), ("essay", "editorial"), ("news", "editorial"),
    ("zine", "editorial"), ("longform", "editorial"),
    ("portfolio", "portfolio"), ("photographer", "portfolio"), ("illustrator", "portfolio"), ("artist", "portfolio"),
    ("freelancer", "portfolio"), ("resume", "portfolio"), ("architect", "portfolio"),
    ("finance", "finance"), ("bank", "finance"), ("fintech", "finance"), ("investing", "finance"),
    ("wealth", "finance"), ("accounting", "finance"), ("insurance", "finance"), ("payments", "finance"),
    ("ledger", "finance"), ("budget", "finance"), ("tax", "finance"), ("invoicing", "finance"),
    ("clinic", "health"), ("health", "health"), ("medical", "health"), ("dental", "health"), ("dentist", "health"),
    ("therapy", "health"), ("wellness", "health"), ("yoga", "health"), ("physio", "health"), ("pharmacy", "health"),
    ("eyecare", "health"), ("optician", "health"), ("fitness", "health"), ("gym", "health"),
    ("school", "education"), ("course", "education"), ("university", "education"), ("academy", "education"),
    ("learning", "education"), ("tutor", "education"), ("education", "education"), ("bootcamp", "education"),
    ("curriculum", "education"), ("workshop", "education"),
    ("festival", "events"), ("conference", "events"), ("event", "events"), ("concert", "events"),
    ("summit", "events"), ("meetup", "events"), ("tickets", "events"), ("gig", "events"), ("wedding", "events"),
    ("exhibition", "events"),
    ("real estate", "real-estate"), ("property", "real-estate"), ("apartment", "real-estate"),
    ("homes", "real-estate"), ("realtor", "real-estate"), ("estate agent", "real-estate"), ("rental", "real-estate"),
    ("lettings", "real-estate"),
    ("travel", "travel"), ("hotel", "travel"), ("resort", "travel"), ("tour", "travel"), ("trip", "travel"),
    ("itinerary", "travel"), ("hostel", "travel"), ("airline", "travel"), ("cruise", "travel"),
    ("destination", "travel"), ("retreat", "travel"),
    ("nonprofit", "nonprofit"), ("non-profit", "nonprofit"), ("charity", "nonprofit"), ("ngo", "nonprofit"),
    ("foundation", "nonprofit"), ("donate", "nonprofit"), ("volunteer", "nonprofit"),
    ("docs", "docs"), ("documentation", "docs"), ("developer docs", "docs"), ("api reference", "docs"),
    ("knowledge base", "docs"), ("changelog", "docs"), ("sdk", "docs"),
    ("law firm", "professional"), ("law", "professional"), ("lawyer", "professional"), ("legal", "professional"),
    ("attorney", "professional"), ("solicitor", "professional"), ("barrister", "professional"),
    ("notary", "professional"), ("accountant", "professional"), ("accountancy", "professional"),
    ("bookkeeping", "professional"), ("consultancy", "professional"), ("consulting", "professional"),
    ("consultant", "professional"), ("advisory", "professional"), ("advisor", "professional"),
    ("corporate", "professional"), ("firm", "professional"), ("recruitment", "professional"),
    ("surveyor", "professional"), ("compliance", "professional"),
    ("landing", "landing"), ("launch", "landing"), ("waitlist", "landing"), ("landing page", "landing"),
];

const MOOD_WORDS: &[(&str, &str)] = &[
    ("calm", "calm"), ("quiet", "calm"), ("serene", "calm"), ("gentle", "calm"), ("soft", "calm"),
    ("understated", "calm"), ("simple", "minimal"), ("minimal", "minimal"), ("minimalist", "minimal"), ("clean", "minimal"),
    ("warm", "warm"), ("cosy", "warm"), ("cozy", "warm"), ("friendly", "warm"), ("neighbourhood", "warm"),
    ("neighborhood", "warm"), ("homely", "warm"), ("rustic", "warm"), ("bakery", "warm"),
    ("playful", "playful"), ("fun", "playful"), ("quirky", "playful"), ("whimsical", "playful"),
    ("colourful", "playful"), ("colorful", "playful"),
    ("bold", "bold"), ("loud", "bold"), ("striking", "bold"), ("punchy", "bold"), ("confident", "bold"),
    ("luxury", "luxe"), ("luxe", "luxe"), ("premium", "luxe"), ("high-end", "luxe"), ("elegant", "luxe"),
    ("refined", "luxe"), ("atelier", "luxe"),
    ("technical", "technical"), ("developer", "technical"), ("precise", "technical"), ("engineering", "technical"),
    ("data", "technical"),
    ("editorial", "editorial"), ("magazine", "editorial"), ("typographic", "editorial"), ("literary", "editorial"),
    ("organic", "organic"), ("natural", "organic"), ("handmade", "organic"), ("artisan", "organic"),
    ("craft", "organic"), ("botanical", "organic"),
    ("brutalist", "brutal"), ("brutal", "brutal"), ("raw", "brutal"), ("stark", "brutal"),
    ("dark", "dark"), ("night", "dark"), ("noir", "dark"), ("moody", "dark"),
    ("bright", "bright"), ("sunny", "bright"), ("vivid", "bright"), ("airy", "bright"),
    ("trusted", "trustworthy"), ("trustworthy", "trustworthy"), ("secure", "trustworthy"),
    ("reliable", "trustworthy"), ("clinical", "trustworthy"),
    ("experimental", "experimental"), ("avant-garde", "experimental"), ("unconventional", "experimental"),
    ("kinetic", "experimental"), ("immersive", "experimental"),
];

/// Explicit restraint in the brief. Checked first: restraint beats any ambition word.
const CALM_WORDS: &[&str] = &[
    "calm", "quiet", "minimal", "minimalist", "simple", "subtle", "gentle", "understated", "static", "serene",
    "no animation", "no motion", "reduced motion", "low motion", "little motion", "without animation",
];
const WILD_WORDS: &[&str] = &[
    "wild", "immersive", "experimental", "kinetic", "webgl", "3d", "three.js", "threejs", "showpiece", "maximal",
    "avant-garde", "motion design",
];
const BOLD_WORDS: &[&str] = &["bold", "dynamic", "animated", "energetic", "lively", "scroll-driven", "playful"];

fn tokens(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !(c.is_alphanumeric() || c == '.' || c == '-'))
        .map(|t| t.trim_matches(|c| c == '.' || c == '-').to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

/// Whole-word or whole-phrase hit (a phrase matches consecutive tokens; `-` inside a word is kept,
/// so `e-commerce` and `avant-garde` match as written).
fn hit(toks: &[String], phrase: &str) -> bool {
    first_hit(toks, phrase).is_some()
}

/// Token index where a word/phrase first appears in the brief.
fn first_hit(toks: &[String], phrase: &str) -> Option<usize> {
    let parts = tokens(phrase);
    match parts.len() {
        0 => None,
        1 => toks.iter().position(|t| *t == parts[0] || (parts[0].len() > 3 && *t == format!("{}s", parts[0]))),
        n => toks.windows(n).position(|w| w.iter().zip(&parts).all(|(a, b)| a == b)),
    }
}

/// Niches named in the brief, in the order the brief names them (the first named niche leads).
pub fn detect_niches(brief: &str) -> Vec<String> {
    let toks = tokens(brief);
    let mut found: Vec<(usize, usize, &str)> = NICHE_WORDS
        .iter()
        .enumerate()
        .filter_map(|(rank, (w, n))| first_hit(&toks, w).map(|pos| (pos, rank, *n)))
        .collect();
    found.sort();
    let mut out: Vec<String> = Vec::new();
    for (_, _, n) in found {
        if !out.iter().any(|x| x == n) {
            out.push(n.to_string());
        }
    }
    out
}

pub fn detect_moods(brief: &str) -> Vec<String> {
    let toks = tokens(brief);
    let mut out: Vec<String> = Vec::new();
    for (w, m) in MOOD_WORDS {
        if hit(&toks, w) && !out.iter().any(|x| x == m) {
            out.push((*m).to_string());
        }
    }
    out
}

/// Intensity the brief asks for in so many words, if any. Restraint wins over ambition.
pub fn brief_intensity(brief: &str) -> Option<Intensity> {
    let toks = tokens(brief);
    if CALM_WORDS.iter().any(|w| hit(&toks, w)) {
        Some(Intensity::Calm)
    } else if WILD_WORDS.iter().any(|w| hit(&toks, w)) {
        Some(Intensity::Wild)
    } else if BOLD_WORDS.iter().any(|w| hit(&toks, w)) {
        Some(Intensity::Bold)
    } else {
        None
    }
}

/// Default intensity for a niche when the brief names none: task surfaces calm, showcases wild.
fn niche_default_intensity(niches: &[String]) -> Intensity {
    match niches.first().map(String::as_str) {
        Some("dashboard" | "docs" | "health" | "finance" | "professional") => Intensity::Calm,
        Some("kinetic" | "3d") => Intensity::Wild,
        _ => Intensity::Bold,
    }
}

/// Section roles that read as the trade's own artefact for a niche (they lead the signature pick).
fn trade_roles(niche: &str) -> &'static [&'static str] {
    match niche {
        "store" => &["catalogue", "comparison", "gallery"],
        "restaurant" => &["menu", "location", "schedule"],
        "local-business" => &["menu", "services", "location", "about"],
        "saas" => &["features", "pricing", "process", "proof"],
        "dashboard" => &["data-story", "features", "docs-nav"],
        "agency" => &["work", "services", "process"],
        "kinetic" | "3d" => &["work", "process", "services", "gallery"],
        "editorial" => &["data-story", "gallery", "about"],
        "portfolio" => &["work", "gallery", "about"],
        "finance" => &["data-story", "pricing", "comparison", "proof"],
        "health" => &["services", "schedule", "team", "process"],
        "education" => &["curriculum", "schedule", "team"],
        "events" => &["schedule", "location", "listing"],
        "real-estate" => &["listing", "location", "comparison"],
        "travel" => &["itinerary", "location", "gallery"],
        "nonprofit" => &["impact", "data-story", "timeline"],
        "docs" => &["docs-nav", "features", "faq"],
        "landing" => &["features", "proof", "pricing", "cta"],
        "professional" => &["services", "team", "proof", "process", "timeline"],
        _ => &[],
    }
}

/// Niches whose mechanics depict that trade's own object; foreign to any other brief.
const NARROW_TRADES: &[&str] = &["store", "restaurant", "real-estate", "dashboard", "docs", "events", "travel"];

/// Roles that sit at the page edges; they are suggested but never dealt as a signature.
const EDGE_ROLES: &[&str] = &["footer", "faq", "cta", "contact"];

// ---------------------------------------------------------------------------------------------
// Deal
// ---------------------------------------------------------------------------------------------

/// Motion language of an existing site (Mode B/C). Caps what a deal may pick.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistingSite {
    pub intensity: Intensity,
    pub max_tier: Tier,
    pub framework: String,
    /// Niches read from the site itself (project name, routes); used when the brief names none.
    #[serde(default)]
    pub niches: Vec<String>,
    /// Where the profile came from (`SITE-DNA.json`, `inspect`, `--intensity flag`...).
    pub source: String,
}

impl ExistingSite {
    /// Reads intensity from the inspected motion dial and the stack from detected libraries: WebGL only
    /// when the site already ships Three.js/WebGL; scripted motion only when it already animates.
    pub fn from_dna(dna: &SiteDna, source: &str) -> Self {
        let libs = dna.motion.libraries_detected.join(" ").to_ascii_lowercase();
        let intensity = Intensity::from_dial(dna.motion.recommended_motion_intensity);
        let max_tier = if libs.contains("three") || libs.contains("webgl") || libs.contains("r3f") {
            Tier::Webgl
        } else if intensity >= Intensity::Bold || !dna.motion.libraries_detected.is_empty() {
            Tier::Js
        } else {
            Tier::Css
        };
        let words = format!("{} {}", dna.project_name, dna.routes.join(" ")).replace(['-', '_', '/', '.'], " ");
        let niches = detect_niches(&words);
        Self { intensity, max_tier, framework: dna.framework.clone(), niches, source: source.to_string() }
    }

    /// An existing site described only by its intensity (`--existing --intensity calm`, MCP param).
    pub fn from_intensity(intensity: Intensity, source: &str) -> Self {
        let max_tier = match intensity {
            Intensity::Calm => Tier::Css,
            _ => Tier::Js,
        };
        Self { intensity, max_tier, framework: "unknown".into(), niches: Vec::new(), source: source.to_string() }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DealOptions {
    pub brief: String,
    pub niche: Option<String>,
    pub mood: Option<String>,
    /// Explicit intensity (flag / MCP param). For a new site the brief's own intensity words still win.
    pub intensity: Option<Intensity>,
    /// Section mechanics to deal: 2 signature + (count - 2) suggestions. Clamped to 2..=10.
    pub count: usize,
    /// Deterministic seed; `None` derives it from the brief so one brief always gets one deal.
    pub seed: Option<u64>,
    /// Present for Mode B/C: the deal follows the existing site's motion language.
    pub existing: Option<ExistingSite>,
    /// Families / ids to skip (used to keep the 3 design directions apart).
    pub exclude_families: Vec<String>,
    pub exclude_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealContext {
    /// `new` (brief drives the deal) or `existing` (the site's motion language caps it).
    pub mode: String,
    pub niches: Vec<String>,
    pub moods: Vec<String>,
    pub intensity: Intensity,
    /// Why this intensity: `brief`, `flag`, `niche default`, `existing site`...
    pub intensity_source: String,
    pub max_tier: Tier,
    pub framework: Option<String>,
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanicDeal {
    pub brief: String,
    pub context: DealContext,
    pub hero: Option<Mechanic>,
    /// Build these two verbatim in spirit: they are the page's signature moves.
    pub signature: Vec<Mechanic>,
    /// Ideas for the remaining sections; use, adapt or replace with the content's own shape.
    pub suggestions: Vec<Mechanic>,
    pub transition: Option<Mechanic>,
    pub notes: Vec<String>,
    pub rules: Vec<String>,
}

fn fnv1a(text: &str) -> u64 {
    text.bytes().fold(0xcbf29ce484222325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3))
}

/// Simple wrapper: deal for a new site from the brief (`niche`/`mood` optional hints).
pub fn deal_mechanics(brief: &str, niche: Option<&str>, mood: Option<&str>, count: usize, seed: u64) -> MechanicDeal {
    deal_mechanics_with(&DealOptions {
        brief: brief.to_string(),
        niche: niche.map(String::from),
        mood: mood.map(String::from),
        count,
        seed: Some(seed),
        ..Default::default()
    })
}

pub fn deal_mechanics_with(opts: &DealOptions) -> MechanicDeal {
    let brief = opts.brief.trim();
    let mut notes = Vec::new();

    // Niches: the brief's own words first, then the hint; the hint never displaces the brief.
    let mut niches = detect_niches(brief);
    if let Some(n) = opts.niche.as_deref().filter(|n| !n.trim().is_empty()) {
        let n = normalize_niche(n);
        if NICHES.contains(&n.as_str()) {
            if !niches.contains(&n) {
                niches.push(n);
            }
        } else {
            notes.push(format!("Unknown niche `{}` ignored; valid niches: {}.", n, NICHES.join(", ")));
        }
    }
    if niches.is_empty() {
        if let Some(site) = opts.existing.as_ref().filter(|s| !s.niches.is_empty()) {
            niches = site.niches.clone();
            notes.push(format!("Niche read from the existing site: {}.", niches.join(", ")));
        }
    }
    let mut moods = detect_moods(brief);
    for m in opts.mood.as_deref().unwrap_or("").split([',', ' ']).map(str::trim).filter(|m| !m.is_empty()) {
        let m = m.to_ascii_lowercase();
        if !moods.contains(&m) {
            moods.push(m);
        }
    }

    // Intensity
    let from_brief = brief_intensity(brief);
    let (intensity, intensity_source, max_tier, framework, mode) = match &opts.existing {
        None => {
            let (i, src) = match (from_brief, opts.intensity) {
                (Some(b), _) => (b, "brief"),
                (None, Some(f)) => (f, "flag"),
                (None, None) => (niche_default_intensity(&niches), "niche default"),
            };
            let tier = match i {
                Intensity::Calm => Tier::Css,
                Intensity::Bold => Tier::Js,
                Intensity::Wild => Tier::Webgl,
            };
            (i, src.to_string(), tier, None, "new")
        }
        Some(site) => {
            // The site's language is the ceiling; a brief word or flag may only lower it
            let mut i = site.intensity;
            let mut src = format!("existing site ({})", site.source);
            for (lower, why) in [(from_brief, "brief"), (opts.intensity, "flag")] {
                if let Some(l) = lower {
                    if l < i {
                        i = l;
                        src = format!("{} (below the existing site's {})", why, site.intensity.as_str());
                    } else if l > i {
                        notes.push(format!(
                            "Requested `{}` motion ({}) is above the existing site's `{}` language; capped to keep the site's identity.",
                            l.as_str(),
                            why,
                            site.intensity.as_str()
                        ));
                    }
                }
            }
            let tier = site.max_tier.min(match i {
                Intensity::Calm => Tier::Css,
                Intensity::Bold => Tier::Js,
                Intensity::Wild => Tier::Webgl,
            });
            notes.push(format!(
                "Existing site: dealt only `{}`-or-calmer mechanics buildable with `{}` or less on `{}`. Build them with the site's own tokens, type, easing and durations; none of these may restyle the site.",
                i.as_str(),
                tier.as_str(),
                site.framework
            ));
            notes.push("Existing site: keep the current hero; the dealt hero applies only to a new page or an explicitly requested hero rework.".into());
            (i, src, tier, Some(site.framework.clone()), "existing")
        }
    };
    if niches.is_empty() {
        notes.push("No niche named in the brief: dealt from universal (`any`) and broadly-tagged mechanics; pass --niche for a sharper deal.".into());
    }
    if intensity == Intensity::Calm {
        notes.push("Calm deal: every mechanic is a quiet, CSS-capable idea. Keep each section's distinct idea; do not add bold motion.".into());
    }
    let toks = tokens(brief);
    if ["no animation", "no motion", "static", "without animation"].iter().any(|w| hit(&toks, w)) {
        notes.push("The brief asks for no motion: build each mechanic as its reduced-motion end state (the composition stays, nothing moves) and declare <meta name=\"layoza-motion\" content=\"none\">.".into());
    }

    let seed = opts.seed.unwrap_or_else(|| fnv1a(&brief.to_lowercase()));
    let count = opts.count.clamp(2, 10);

    let allowed = |m: &Mechanic| -> bool {
        let fits_intensity = if intensity == Intensity::Calm { m.intensity == Intensity::Calm } else { m.intensity <= intensity };
        fits_intensity
            && m.tier <= max_tier
            && !opts.exclude_ids.iter().any(|x| x == &m.id)
            && !opts.exclude_families.iter().any(|x| x == &m.family)
    };
    let niche_match = |m: &Mechanic| niches.is_empty() || m.niches.iter().any(|n| n == "any" || niches.contains(n));
    let score = |m: &Mechanic| -> i64 {
        let mut s: i64 = 0;
        for (rank, n) in niches.iter().enumerate() {
            if m.niches.contains(n) {
                s += if rank == 0 { 8 } else { 4 };
            }
        }
        if m.niches.iter().any(|n| n == "any") {
            s += 2;
        }
        // A mechanic built around another trade's object (a shop shelf, a floor plan, an app screen)
        // reads as a template from the wrong business: push it down unless the brief names that trade
        if let Some(primary) = m.niches.first() {
            if NARROW_TRADES.contains(&primary.as_str()) && !niches.contains(primary) {
                s -= 6;
            }
        }
        s += 2 * m.moods.iter().filter(|x| moods.contains(x)).count() as i64;
        s += match (intensity as i64) - (m.intensity as i64) {
            0 => 4,
            1 => 1,
            _ => 0,
        };
        if m.kind == MechanicKind::Section {
            if let Some(first) = niches.first() {
                if trade_roles(first).contains(&m.role.as_str()) {
                    s += 5;
                }
            }
            if niches.iter().skip(1).any(|n| trade_roles(n).contains(&m.role.as_str())) {
                s += 2;
            }
        }
        // Seeded jitter (0-3) so one seed is reproducible and another seed explores close candidates
        s + (fnv1a(&format!("{}:{}", seed, m.id)) % 4) as i64
    };
    let ranked = |kind: MechanicKind| -> Vec<&'static Mechanic> {
        let pool: Vec<&Mechanic> = catalogue().iter().filter(|m| m.kind == kind && allowed(m)).collect();
        let matched: Vec<&Mechanic> = pool.iter().copied().filter(|m| niche_match(m)).collect();
        // Too few in-niche entries at this intensity: fall back to the whole allowed pool (scored, so
        // in-niche entries still lead)
        let enough = if kind == MechanicKind::Section { 2 } else { 1 };
        let mut v = if matched.len() >= enough { matched } else { pool };
        v.sort_by(|a, b| score(b).cmp(&score(a)).then_with(|| a.id.cmp(&b.id)));
        v
    };

    let mut used_families: BTreeSet<String> = BTreeSet::new();
    let mut used_roles: BTreeSet<String> = BTreeSet::new();
    let take = |list: &[&'static Mechanic], fam: &mut BTreeSet<String>, extra: &dyn Fn(&Mechanic) -> bool| -> Option<Mechanic> {
        let m = list.iter().find(|m| !fam.contains(&m.family) && extra(m))?;
        fam.insert(m.family.clone());
        Some((*m).clone())
    };

    let heroes = ranked(MechanicKind::Hero);
    let hero = take(&heroes, &mut used_families, &|_| true);
    match &hero {
        None => notes.push("No hero mechanic fits these constraints.".into()),
        Some(h) if !niches.is_empty() && !h.niches.iter().any(|n| n == "any" || niches.contains(n)) => notes.push(
            "No in-niche hero at this intensity: dealt the closest fit; rebuild it around the trade's own object.".into(),
        ),
        _ => {}
    }

    let sections = ranked(MechanicKind::Section);
    let mut signature = Vec::new();
    for _ in 0..2 {
        let pick = {
            let roles = used_roles.clone();
            take(&sections, &mut used_families, &|m| !EDGE_ROLES.contains(&m.role.as_str()) && !roles.contains(&m.role))
        };
        if let Some(m) = pick {
            used_roles.insert(m.role.clone());
            signature.push(m);
        }
    }
    let mut suggestions = Vec::new();
    while signature.len() + suggestions.len() < count {
        let roles = used_roles.clone();
        let pick = take(&sections, &mut used_families, &|m| !roles.contains(&m.role))
            .or_else(|| take(&sections, &mut used_families, &|_| true));
        match pick {
            Some(m) => {
                used_roles.insert(m.role.clone());
                suggestions.push(m);
            }
            None => break,
        }
    }
    if signature.len() + suggestions.len() < count {
        notes.push(format!(
            "Only {} distinct-family section mechanics fit these constraints (asked for {}).",
            signature.len() + suggestions.len(),
            count
        ));
    }
    let transitions = ranked(MechanicKind::Transition);
    let transition = take(&transitions, &mut used_families, &|_| true);

    let rules = vec![
        "Build the two `signature` mechanics verbatim in spirit: keep the mechanism and its still_check; adapt colour, type, copy and objects to the brand and the trade.".into(),
        "Scroll motion follows scroll progress, never velocity: screenshots at 25/50/75% of a pinned range must differ and each must be a complete, readable composition.".into(),
        "Every mechanic keeps real DOM text, honours prefers-reduced-motion with its listed end state, and restages at 375px as its `mobile` field says.".into(),
        "Suggestions are optional; a section whose content has its own shape (a real menu, timetable, ledger) should follow that shape instead of a stock grid or stepper.".into(),
        "Declare the page's motion level with <meta name=\"layoza-motion\" content=\"calm|bold|wild|none\"> so `layoza qa` checks motion density at the right threshold.".into(),
        "Verify with `layoza qa`: no `pinned-stage-static`, `motion-too-sparse`, `text-mid-reveal` or `art-over-text` warnings.".into(),
    ];

    MechanicDeal {
        brief: brief.to_string(),
        context: DealContext {
            mode: mode.into(),
            niches,
            moods,
            intensity,
            intensity_source,
            max_tier,
            framework,
            seed,
        },
        hero,
        signature,
        suggestions,
        transition,
        notes,
        rules,
    }
}

/// Compact reference used inside design-system directions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanicRef {
    pub id: String,
    pub name: String,
    pub kind: MechanicKind,
    pub role: String,
    pub family: String,
    pub intensity: Intensity,
    pub still_check: String,
}

impl From<&Mechanic> for MechanicRef {
    fn from(m: &Mechanic) -> Self {
        Self {
            id: m.id.clone(),
            name: m.name.clone(),
            kind: m.kind,
            role: m.role.clone(),
            family: m.family.clone(),
            intensity: m.intensity,
            still_check: m.still_check.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, HashSet};

    fn ids(d: &MechanicDeal) -> Vec<String> {
        d.hero.iter().chain(&d.signature).chain(&d.suggestions).chain(&d.transition).map(|m| m.id.clone()).collect()
    }
    fn all(d: &MechanicDeal) -> Vec<&Mechanic> {
        d.hero.iter().chain(&d.signature).chain(&d.suggestions).chain(&d.transition).collect()
    }

    #[test]
    fn catalogue_size_kinds_and_fields() {
        let cat = catalogue();
        assert!(cat.len() >= 60, "{} entries", cat.len());
        let count = |k| cat.iter().filter(|m| m.kind == k).count();
        assert!(count(MechanicKind::Hero) >= 15);
        assert!(count(MechanicKind::Section) >= 35);
        assert!(count(MechanicKind::Transition) >= 10);
        assert!(cat.iter().filter(|m| m.kind == MechanicKind::Hero && m.headline_interlock).count() >= 6);
        // The calm tier is first-class for every kind
        for k in [MechanicKind::Hero, MechanicKind::Section, MechanicKind::Transition] {
            assert!(cat.iter().filter(|m| m.kind == k && m.intensity == Intensity::Calm).count() >= 4, "{:?}", k);
        }
        let mut seen = HashSet::new();
        for m in cat {
            assert!(seen.insert(m.id.as_str()), "duplicate id {}", m.id);
            for (f, v) in [
                ("id", &m.id), ("name", &m.name), ("family", &m.family), ("role", &m.role), ("blueprint", &m.blueprint),
                ("motion", &m.motion), ("still_check", &m.still_check), ("mobile", &m.mobile), ("a11y", &m.a11y),
                ("avoid", &m.avoid), ("css", &m.stack_notes.css), ("js", &m.stack_notes.js),
            ] {
                assert!(!v.trim().is_empty(), "{}: empty {}", m.id, f);
            }
            assert!(!m.niches.is_empty() && !m.moods.is_empty(), "{}", m.id);
            for n in &m.niches {
                assert!(NICHES.contains(&n.as_str()), "{}: unknown niche {}", m.id, n);
            }
            assert!(
                m.kind == MechanicKind::Transition || ["clamp(", "min(", "rem", "px"].iter().any(|u| m.blueprint.contains(u)),
                "{}: blueprint needs concrete sizes",
                m.id
            );
            assert!(!m.headline_interlock || m.kind == MechanicKind::Hero, "{}", m.id);
            assert!(!m.blueprint.contains('\\') && !m.motion.contains('\\'), "{}", m.id);
        }
        // Every niche is served by at least one hero and two sections
        for n in NICHES.iter().filter(|n| **n != "any") {
            assert!(!list_mechanics(Some(n), Some(MechanicKind::Hero), None).is_empty(), "no hero for {}", n);
            assert!(list_mechanics(Some(n), Some(MechanicKind::Section), None).len() >= 2, "sections for {}", n);
        }
    }

    #[test]
    fn no_near_duplicates() {
        let cat = catalogue();
        let words = |s: &str| -> HashSet<String> { tokens(s).into_iter().filter(|w| w.len() > 3).collect() };
        let mut names = HashSet::new();
        for m in cat {
            assert!(names.insert(m.name.to_lowercase()), "duplicate name {}", m.name);
        }
        for (i, a) in cat.iter().enumerate() {
            let wa = words(&a.blueprint);
            for b in cat.iter().skip(i + 1) {
                let wb = words(&b.blueprint);
                let inter = wa.intersection(&wb).count() as f64;
                let union = wa.union(&wb).count().max(1) as f64;
                assert!(inter / union < 0.5, "{} and {} blueprints are near-duplicates ({:.2})", a.id, b.id, inter / union);
                assert!(
                    !(a.kind == b.kind && a.family == b.family && a.role == b.role && a.niches == b.niches),
                    "{} and {} share kind, family, role and niches",
                    a.id,
                    b.id
                );
            }
        }
    }

    #[test]
    fn deal_is_distinct_coherent_and_deterministic() {
        let a = deal_mechanics("kinetic motion design studio with 3D ribbon", None, None, 5, 7);
        let b = deal_mechanics("kinetic motion design studio with 3D ribbon", None, None, 5, 7);
        assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
        assert!(a.hero.is_some() && a.transition.is_some());
        assert_eq!(a.signature.len(), 2);
        assert_eq!(a.signature.len() + a.suggestions.len(), 5);
        let fams: Vec<&String> = all(&a).iter().map(|m| &m.family).collect();
        let uniq: HashSet<&&String> = fams.iter().collect();
        assert_eq!(fams.len(), uniq.len(), "two mechanics of one family: {:?}", fams);
        let roles: HashSet<&String> = a.signature.iter().map(|m| &m.role).collect();
        assert_eq!(roles.len(), 2);
        assert!(a.signature.iter().all(|m| !EDGE_ROLES.contains(&m.role.as_str())));
        assert_eq!(a.context.intensity, Intensity::Wild);
        // Another seed explores (at least one of several seeds changes the set)
        assert!((0..8u64).any(|s| ids(&deal_mechanics("kinetic motion design studio with 3D ribbon", None, None, 5, s)) != ids(&a)));
        // Default seed comes from the brief
        let c = deal_mechanics_with(&DealOptions { brief: "neighbourhood bakery".into(), count: 4, ..Default::default() });
        let d = deal_mechanics_with(&DealOptions { brief: "neighbourhood bakery".into(), count: 4, ..Default::default() });
        assert_eq!(ids(&c), ids(&d));
    }

    #[test]
    fn niche_filtering() {
        for (brief, niche) in [
            ("seafood restaurant with a daily menu", "restaurant"),
            ("independent store selling linen", "store"),
            ("property lettings agency in Leeds", "real-estate"),
            ("developer documentation for an SDK", "docs"),
            ("charity raising funds for clean water", "nonprofit"),
        ] {
            let d = deal_mechanics(brief, None, None, 4, 1);
            assert_eq!(d.context.niches.first().map(String::as_str), Some(niche), "{}", brief);
            for m in all(&d) {
                assert!(m.niches.iter().any(|n| n == "any" || d.context.niches.contains(n)), "{}: {} off-niche", brief, m.id);
            }
        }
        let menu = deal_mechanics("seafood restaurant with a daily menu", None, None, 4, 1);
        assert!(menu.signature.iter().chain(&menu.suggestions).any(|m| m.role == "menu"), "restaurant deal lacks a menu mechanic");
        let restaurants = list_mechanics(Some("restaurant"), Some(MechanicKind::Section), None);
        assert!(restaurants.iter().all(|m| m.niches.iter().any(|n| n == "restaurant" || n == "any")));
    }

    #[test]
    fn calm_deal_returns_only_calm_entries() {
        let d = deal_mechanics_with(&DealOptions {
            brief: "kinetic agency site".into(),
            intensity: Some(Intensity::Calm),
            count: 6,
            seed: Some(3),
            ..Default::default()
        });
        // A new site's brief wins over the flag: "kinetic" asks for wild
        assert_eq!(d.context.intensity, Intensity::Wild);
        let calm = deal_mechanics_with(&DealOptions {
            brief: "agency site".into(),
            intensity: Some(Intensity::Calm),
            count: 6,
            seed: Some(3),
            ..Default::default()
        });
        assert_eq!(calm.context.intensity, Intensity::Calm);
        assert!(!all(&calm).is_empty());
        for m in all(&calm) {
            assert_eq!(m.intensity, Intensity::Calm, "{}", m.id);
            assert_eq!(m.tier, Tier::Css, "{}", m.id);
        }
        assert_eq!(calm.signature.len(), 2, "calm still deals two distinct signature ideas");
    }

    #[test]
    fn brief_keywords_override_defaults() {
        // Kinetic niche defaults to wild; the brief's "calm" wins over the default and over a flag
        let d = deal_mechanics_with(&DealOptions {
            brief: "calm kinetic type studio".into(),
            intensity: Some(Intensity::Wild),
            seed: Some(1),
            count: 4,
            ..Default::default()
        });
        assert_eq!((d.context.intensity, d.context.intensity_source.as_str()), (Intensity::Calm, "brief"));
        // Dashboards default to calm; an explicit "bold animated" brief raises it
        assert_eq!(deal_mechanics("analytics dashboard", None, None, 4, 1).context.intensity, Intensity::Calm);
        assert_eq!(deal_mechanics("bold animated analytics dashboard", None, None, 4, 1).context.intensity, Intensity::Bold);
        // The brief's niche leads; a niche hint is added after it, never in front
        let n = deal_mechanics("neighbourhood bakery", Some("events"), None, 4, 1);
        assert_eq!(n.context.niches.first().map(String::as_str), Some("local-business"));
        assert!(n.context.niches.contains(&"restaurant".to_string()));
        assert_eq!(n.context.niches.last().map(String::as_str), Some("events"));
        assert!(n.context.moods.contains(&"warm".to_string()));
    }

    #[test]
    fn existing_site_deal_respects_intensity() {
        for site_i in [Intensity::Calm, Intensity::Bold] {
            let d = deal_mechanics_with(&DealOptions {
                brief: "immersive 3d kinetic showcase".into(),
                count: 5,
                seed: Some(2),
                existing: Some(ExistingSite::from_intensity(site_i, "test")),
                ..Default::default()
            });
            assert_eq!(d.context.mode, "existing");
            assert_eq!(d.context.intensity, site_i, "brief cannot raise an existing site's motion");
            for m in all(&d) {
                assert!(m.intensity <= site_i, "{} is {:?} on a {:?} site", m.id, m.intensity, site_i);
                assert!(m.tier < Tier::Webgl, "{} needs WebGL the site does not ship", m.id);
                if site_i == Intensity::Calm {
                    assert_eq!(m.intensity, Intensity::Calm);
                }
            }
            assert!(d.notes.iter().any(|n| n.contains("restyle")), "output must say it follows the site");
            assert!(d.notes.iter().any(|n| n.contains("capped")));
        }
        // A brief word may lower an existing site's motion
        let low = deal_mechanics_with(&DealOptions {
            brief: "add a simple pricing section".into(),
            existing: Some(ExistingSite::from_intensity(Intensity::Bold, "test")),
            count: 3,
            ..Default::default()
        });
        assert_eq!(low.context.intensity, Intensity::Calm);
        // From inspected DNA: no motion libraries + low dial -> calm, CSS only
        let root = std::env::temp_dir().join("layoza_test_mech_existing");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("index.html"), "<main><h1>Plain site</h1></main>").unwrap();
        let (dna, _) = crate::inspector::inspect_project(&root, None, false).unwrap();
        let site = ExistingSite::from_dna(&dna, "inspect");
        assert_eq!((site.intensity, site.max_tier), (Intensity::Calm, Tier::Css));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn exclusions_keep_directions_apart() {
        let a = deal_mechanics("seafood restaurant", None, None, 2, 1);
        let fams: Vec<String> = all(&a).iter().map(|m| m.family.clone()).collect();
        let b = deal_mechanics_with(&DealOptions {
            brief: "seafood restaurant".into(),
            count: 2,
            seed: Some(1),
            exclude_families: fams.clone(),
            ..Default::default()
        });
        for m in all(&b) {
            assert!(!fams.contains(&m.family), "{}", m.id);
        }
        let mut per_role: BTreeMap<&str, usize> = BTreeMap::new();
        for m in catalogue() {
            *per_role.entry(m.role.as_str()).or_default() += 1;
        }
        assert!(per_role.len() >= 20, "{:?}", per_role);
    }

    fn picks(d: &MechanicDeal) -> Vec<Mechanic> {
        d.hero.iter().chain(d.signature.iter()).cloned().collect()
    }

    #[test]
    fn law_firm_gets_no_other_trades_objects() {
        for seed in 0..12 {
            let d = deal_mechanics("corporate law firm", None, None, 4, seed);
            assert_eq!(d.context.niches.first().map(String::as_str), Some("professional"));
            for m in picks(&d) {
                let primary = m.niches.first().map(String::as_str).unwrap_or("");
                assert!(!NARROW_TRADES.contains(&primary), "seed {seed}: law firm dealt {} ({primary})", m.id);
            }
        }
    }

    #[test]
    fn dental_clinic_gets_no_app_hero() {
        for seed in 0..12 {
            let d = deal_mechanics("calm dental clinic", None, None, 4, seed);
            let hero = d.hero.expect("hero");
            assert!(!hero.niches.first().is_some_and(|n| n == "dashboard" || n == "saas"), "seed {seed}: {}", hero.id);
        }
    }

    #[test]
    fn unknown_niche_avoids_narrow_trade_picks() {
        for seed in 0..12 {
            let d = deal_mechanics("a website for my project", None, None, 4, seed);
            assert!(d.context.niches.is_empty());
            for m in picks(&d) {
                let primary = m.niches.first().map(String::as_str).unwrap_or("");
                assert!(!NARROW_TRADES.contains(&primary), "seed {seed}: generic brief dealt {} ({primary})", m.id);
            }
        }
    }

    #[test]
    fn existing_site_niche_comes_from_the_site() {
        let site = ExistingSite {
            intensity: Intensity::Calm,
            max_tier: Tier::Css,
            framework: "html".into(),
            niches: detect_niches("minimal corporate"),
            source: "test".into(),
        };
        let d = deal_mechanics_with(&DealOptions { brief: "add a team section".into(), count: 3, existing: Some(site), ..Default::default() });
        assert_eq!(d.context.niches, vec!["professional".to_string()]);
        for m in picks(&d) {
            assert_eq!(m.intensity, Intensity::Calm, "{}", m.id);
            assert_ne!(m.niches.first().map(String::as_str), Some("store"), "{}", m.id);
        }
    }
}
