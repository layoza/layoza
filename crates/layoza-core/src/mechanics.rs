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

#[cfg(test)]
#[path = "mechanics/brief_tests.rs"]
mod brief_tests;

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
    /// Hero only: the first-screen composition (`split`, `stacked`, `centred`, `layered`,
    /// `full-bleed`, `sticky-stage`, `object-wrap`, `poster`). Left/right splits are kept a
    /// minority of the catalogue and are never preferred by the deal.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub composition: String,
    /// Hero only: what the first screen is made of (`real-photo`, `generated-image`,
    /// `procedural`, `data-image`, `giant-type`, `physical-object`, `video`), so a deal
    /// can avoid a page that is all typography.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hero_material: String,
    /// The visual look family (`warm-paper-serif`, `dark-single-accent`, `hairline-newspaper`, ...).
    /// Every hero, nav, footer and section head carries one. Families are fine; repeating one
    /// across a deal is not.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub look_family: String,
    /// Conditional mechanics: dealt only when the brief contains at least one of these words or
    /// phrases (a bilingual nav needs a brief that mentions languages). Empty means always eligible.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requires_signal: Vec<String>,
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
    "landing", "professional", "legal", "museum", "devtools", "memorial", "body-art", "civic", "venue", "camp",
    "winery", "home-services", "industrial", "any",
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
        "legal" | "law" | "law-firm" | "lawyer" | "attorney" => "legal".into(),
        "museum" | "science-centre" | "science-center" | "planetarium" | "aquarium" | "zoo" => "museum".into(),
        "corporate" | "consulting" | "consultancy" | "accountancy" | "services" => "professional".into(),
        "school" | "course" | "learning" => "education".into(),
        "event" | "festival" | "conference" => "events".into(),
        "property" | "realestate" => "real-estate".into(),
        "hotel" | "tourism" => "travel".into(),
        "charity" | "non-profit" | "ngo" => "nonprofit".into(),
        "documentation" => "docs".into(),
        "funeral" | "funeral-home" | "funeral-director" | "cremation" | "cemetery" => "memorial".into(),
        "tattoo" | "tattoo-studio" | "piercing" | "bodyart" => "body-art".into(),
        "government" | "council" | "utility" | "municipal" | "public-sector" => "civic".into(),
        "wedding-venue" | "event-space" | "events-venue" => "venue".into(),
        "summer-camp" | "kids-camp" | "day-camp" => "camp".into(),
        "vineyard" | "brewery" | "distillery" | "cidery" => "winery".into(),
        "plumber" | "electrician" | "roofer" | "contractor" | "home-service" | "trades-services" => "home-services".into(),
        "b2b" | "manufacturer" | "manufacturing" | "supplier" | "b2b-supplier" | "industrial-b2b" | "signage" | "led" => "industrial".into(),
        "devtool" | "dev-tools" | "developer-tools" | "ai-devtools" | "cli" | "mcp" | "open-source" | "oss" => "devtools".into(),
        _ => s,
    }
}

// ---------------------------------------------------------------------------------------------
// Brief reading
// ---------------------------------------------------------------------------------------------

/// Brief word/phrase -> niche.
const NICHE_WORDS: &[(&str, &str)] = &[
    ("funeral", "memorial"), ("funeral home", "memorial"), ("funeral director", "memorial"),
    ("cremation", "memorial"), ("crematorium", "memorial"), ("memorial", "memorial"), ("cemetery", "memorial"),
    ("obituary", "memorial"), ("undertaker", "memorial"),
    ("tattoo", "body-art"), ("tattooist", "body-art"), ("tattoo parlour", "body-art"), ("piercing", "body-art"),
    ("council", "civic"), ("municipal", "civic"), ("government", "civic"), ("public transport", "civic"),
    ("transit", "civic"), ("utility", "civic"), ("utilities", "civic"), ("town hall", "civic"),
    ("local authority", "civic"), ("public service", "civic"),
    ("venue", "venue"), ("wedding venue", "venue"), ("event space", "venue"), ("banquet hall", "venue"),
    ("function room", "venue"),
    ("camp", "camp"), ("summer camp", "camp"), ("day camp", "camp"), ("kids camp", "camp"),
    ("coding camp", "camp"), ("summer school", "camp"), ("holiday club", "camp"),
    ("vineyard", "winery"), ("winery", "winery"), ("brewery", "winery"), ("distillery", "winery"),
    ("cellar door", "winery"), ("taproom", "winery"), ("cidery", "winery"),
    ("plumber", "home-services"), ("plumbing", "home-services"), ("electrician", "home-services"),
    ("roofer", "home-services"), ("roofing", "home-services"), ("contractor", "home-services"),
    ("handyman", "home-services"), ("hvac", "home-services"), ("locksmith", "home-services"),
    ("boiler", "home-services"),
    ("industrial", "industrial"), ("manufacturer", "industrial"), ("manufacturing", "industrial"), ("supplier", "industrial"),
    ("b2b", "industrial"), ("fabrication", "industrial"), ("machinery", "industrial"), ("wholesale", "industrial"),
    ("distributor", "industrial"), ("signage", "industrial"), ("smd", "industrial"), ("led screen", "industrial"),
    ("led display", "industrial"), ("led wall", "industrial"), ("video wall", "industrial"), ("billboard", "industrial"),
    ("shop", "store"), ("store", "store"), ("boutique", "store"), ("ecommerce", "store"), ("e-commerce", "store"),
    ("retail", "store"), ("apparel", "store"), ("fashion", "store"), ("skincare", "store"), ("cosmetics", "store"),
    ("sneakers", "store"), ("furniture", "store"), ("merch", "store"), ("catalogue", "store"), ("catalog", "store"),
    ("womenswear", "store"), ("menswear", "store"), ("jewellery", "store"), ("jewelry", "store"),
    ("restaurant", "restaurant"), ("bistro", "restaurant"), ("cafe", "restaurant"), ("café", "restaurant"),
    ("coffee", "restaurant"), ("bar", "restaurant"), ("dining", "restaurant"), ("chef", "restaurant"),
    ("eatery", "restaurant"), ("pizzeria", "restaurant"), ("ramen", "restaurant"), ("seafood", "restaurant"),
    ("wine", "restaurant"), ("tasting menu", "restaurant"), ("bakery", "restaurant"),
    ("bakery", "local-business"), ("baker", "local-business"), ("florist", "local-business"), ("salon", "local-business"),
    ("barber", "local-business"),
    ("garage", "local-business"), ("laundry", "local-business"), ("neighbourhood", "local-business"),
    ("neighborhood", "local-business"), ("local", "local-business"), ("tailor", "local-business"),
    ("repair", "local-business"), ("pottery", "local-business"), ("bookshop", "local-business"),
    ("saas", "saas"), ("software", "saas"), ("platform", "saas"), ("startup", "saas"), ("api", "saas"),
    ("b2b", "saas"), ("integration", "saas"),
    ("devtool", "devtools"), ("devtools", "devtools"), ("dev tools", "devtools"), ("developer tools", "devtools"),
    ("developer tool", "devtools"), ("developer platform", "devtools"), ("cli", "devtools"), ("command line", "devtools"),
    ("command-line", "devtools"), ("mcp", "devtools"), ("mcp server", "devtools"), ("open source", "devtools"),
    ("open-source", "devtools"), ("coding agent", "devtools"), ("ai agent", "devtools"), ("agent skills", "devtools"),
    ("llm", "devtools"), ("compiler", "devtools"), ("npm", "devtools"), ("github", "devtools"),
    ("sdk", "devtools"), ("language server", "devtools"),
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
    ("museum", "museum"), ("science centre", "museum"), ("science center", "museum"), ("planetarium", "museum"),
    ("aquarium", "museum"), ("zoo", "museum"), ("exhibit", "museum"), ("hands-on", "museum"),
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
    ("law firm", "legal"), ("law", "legal"), ("lawyer", "legal"), ("legal", "legal"),
    ("attorney", "legal"), ("solicitor", "legal"), ("barrister", "legal"),
    ("notary", "legal"), ("accountant", "professional"), ("accountancy", "professional"),
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
    "avant-garde", "motion design", "highly animated", "heavily animated", "very animated", "fully animated",
    "cinematic", "motion-rich", "scrollytelling", "lots of animation", "lots of motion",
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

/// Niches that name a technique or a page type, not a trade: "Three.js", "kinetic", "landing page"
/// say how the site is built, not what it is for, so they never lead when a trade is named.
const TECHNIQUE_NICHES: &[&str] = &["3d", "kinetic", "landing"];

/// Niches that name what is being built (a tool) rather than whose site it is.
const SURFACE_NICHES: &[&str] = &["dashboard", "devtools", "docs"];

/// A brief this long is a spec, not a one-liner: it mentions trades in passing (showcase examples,
/// "not a dashboard"), so niches are ranked by how often the brief returns to them.
const LONG_BRIEF_TOKENS: usize = 120;

/// Niches named in the brief. A short brief keeps the order the brief names them (the first named
/// niche leads); a long brief ranks them by how many times they come up and drops passing mentions.
/// Either way a trade niche leads a technique niche.
pub fn detect_niches(brief: &str) -> Vec<String> {
    let toks = tokens(brief);
    // niche -> (mentions, first position)
    let mut seen: Vec<(&str, usize, usize)> = Vec::new();
    for (w, n) in NICHE_WORDS {
        let parts = tokens(w);
        if parts.is_empty() {
            continue;
        }
        let positions: Vec<usize> = toks
            .windows(parts.len())
            .enumerate()
            .filter(|(_, win)| {
                win.iter().zip(&parts).all(|(a, b)| a == b || (parts.len() == 1 && b.len() > 3 && *a == format!("{}s", b)))
            })
            .map(|(i, _)| i)
            // A simile ("design like a studio") names a quality, not the site's trade.
            .filter(|i| !is_simile(&toks, *i))
            .collect();
        let Some(&first) = positions.first() else { continue };
        match seen.iter_mut().find(|(x, _, _)| x == n) {
            Some(e) => {
                e.1 += positions.len();
                e.2 = e.2.min(first);
            }
            None => seen.push((n, positions.len(), first)),
        }
    }
    let long = toks.len() > LONG_BRIEF_TOKENS;
    if long {
        let top = seen.iter().map(|e| e.1).max().unwrap_or(0);
        let floor = 2.max(top / 4);
        seen.retain(|e| e.1 >= floor);
    }
    let has_trade = seen.iter().any(|e| !TECHNIQUE_NICHES.contains(&e.0));
    seen.sort_by_key(|&(n, count, first)| {
        let technique = has_trade && TECHNIQUE_NICHES.contains(&n);
        // In a short brief the product surface leads the trade: a "bakery dashboard" is a
        // dashboard about a bakery, so its first screen is the tool, not the shop window.
        let trade_first = long || !SURFACE_NICHES.contains(&n);
        (technique, trade_first, if long { usize::MAX - count } else { 0 }, first)
    });
    let mut out: Vec<String> = seen.into_iter().map(|e| e.0.to_string()).collect();
    if long {
        out.truncate(4);
    }
    out
}

/// The one niche signal for a brief, shared by `design-system` and `deal`: the niches it names, with
/// a tool brief's storefront trades dropped ("bakery dashboard" is a dashboard, not a shop window).
pub fn niche_signal(brief: &str) -> Vec<String> {
    let mut niches = detect_niches(brief);
    if niches.first().is_some_and(|n| SURFACE_NICHES.contains(&n.as_str())) {
        niches.retain(|n| SURFACE_NICHES.contains(&n.as_str()) || !NARROW_TRADES.contains(&n.as_str()));
    }
    // A supplier names the buyers it serves (schools, weddings, venues); those are its market, not
    // its trade, and a bare "B2B" is not software unless the brief also names a software product.
    if niches.iter().any(|n| n == "industrial") {
        let toks = tokens(brief);
        let software = ["saas", "software", "app", "platform", "api", "dashboard"].iter().any(|w| toks.iter().any(|t| t == w));
        // "3D" names the product ("3D LED displays") far more often than a 3D web build here.
        let low = brief.to_lowercase();
        let web_3d = ["webgl", "three.js", "threejs", "3d model", "3d viewer", "3d product viewer", "3d render", "3d scene", "3d configurator"].iter().any(|k| low.contains(k));
        niches.retain(|n| match n.as_str() {
            "saas" => software,
            "3d" => web_3d,
            "events" | "education" | "venue" | "restaurant" | "hospitality" | "civic" | "memorial" | "health" | "retail" => false,
            _ => true,
        });
    }
    niches
}

/// A conditional mechanic is eligible only when the brief carries one of its signal words.
fn signal_met(m: &Mechanic, brief_tokens: &[String]) -> bool {
    m.requires_signal.is_empty() || m.requires_signal.iter().any(|w| hit(brief_tokens, w))
}

/// True when the word at `pos` follows `like` / `as` (optionally with an article): a comparison.
fn is_simile(toks: &[String], pos: usize) -> bool {
    let prev = |back: usize| pos.checked_sub(back).and_then(|i| toks.get(i)).map(String::as_str);
    let cmp = |t: Option<&str>| matches!(t, Some("like" | "as"));
    cmp(prev(1)) || (matches!(prev(1), Some("a" | "an" | "the")) && cmp(prev(2)))
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

/// Function words, page-type filler and generic quality adjectives. They break a noun phrase and
/// never become a brief object. Grammar only: no trade's own words live here.
const OBJECT_STOPWORDS: &[&str] = &[
    "a", "an", "the", "this", "that", "these", "those", "its", "it", "our", "my", "your", "their", "his", "her", "we", "they",
    "you", "i", "us", "them", "who", "whom", "which", "what", "where", "when", "while", "how", "why", "whose", "there", "here",
    "and", "or", "but", "nor", "so", "yet", "also", "plus", "then", "than", "as", "like", "if", "because", "both", "either",
    "for", "with", "without", "in", "on", "at", "by", "of", "to", "from", "into", "onto", "over", "under", "about", "around",
    "near", "across", "through", "between", "after", "before", "during", "per", "via", "within", "behind", "above", "below",
    "is", "are", "was", "were", "be", "been", "being", "am", "has", "have", "had", "do", "does", "did", "can", "could",
    "will", "would", "should", "may", "might", "must", "shall", "let", "lets", "need", "needs", "want", "wants", "get", "gets",
    "not", "no", "only", "just", "very", "really", "more", "most", "less", "much", "many", "some", "any", "all", "each",
    "every", "other", "own", "same", "such", "new", "old", "one", "two", "three",
    "website", "websites", "site", "sites", "page", "pages", "homepage", "landing", "web", "online", "app", "design",
    "redesign", "build", "make", "create", "please", "section", "sections", "ui", "ux", "layout", "theme", "template",
    "modern", "clean", "minimal", "minimalist", "simple", "beautiful", "elegant", "sleek", "stylish", "premium", "nice",
    "good", "great", "best", "bold", "calm", "quiet", "warm", "cool", "fresh", "friendly", "professional", "trustworthy",
    "dark", "light", "bright", "playful", "luxury", "luxurious", "high-end", "unique", "creative", "stunning", "animated",
    "dynamic", "cinematic", "subtle", "responsive", "mobile-first", "accessible", "fast", "local", "small", "big", "large",
];

/// Verbs a visitor can do on a page, with forms that read as the same act. Generic web acts only.
const ACTION_VERBS: &[&str] = &[
    "book", "order", "buy", "shop", "donate", "give", "call", "visit", "apply", "register", "enrol", "enroll", "join",
    "subscribe", "download", "install", "report", "pay", "request", "schedule", "plan", "arrange", "reserve", "rent",
    "hire", "contact", "quote", "compare", "track", "find", "browse", "search", "learn", "try", "start", "sign", "volunteer",
    "adopt", "sponsor", "vote", "submit", "upload", "check", "renew", "claim", "file", "send", "message", "chat", "read",
    "listen", "watch", "explore", "locate", "pre-order", "preorder", "enquire", "inquire", "ask", "attend", "bid", "list",
    "sell", "trade", "swap", "borrow", "return", "repair", "service", "collect", "deliver", "ship", "pick", "consult",
    "commission", "tour", "taste", "sample", "train", "practise", "practice", "meet", "remember", "honour", "honor",
    "share", "pledge", "petition", "lodge", "notify", "update", "manage", "monitor", "test", "print", "publish", "post",
];

fn verb_base(tok: &str) -> Option<&'static str> {
    ACTION_VERBS.iter().copied().find(|v| {
        tok == *v
            || tok == format!("{v}s")
            || tok == format!("{v}es")
            || tok == format!("{v}ing")
            || (v.ends_with('e') && tok == format!("{}ing", &v[..v.len() - 1]))
            || (v.len() <= 4 && tok == format!("{v}{}ing", &v[v.len() - 1..]))
    })
}

fn object_words(brief: &str) -> Vec<Vec<String>> {
    // Clauses split on punctuation; words keep inner hyphens and apostrophes.
    brief
        .to_lowercase()
        .split(['.', ',', ';', ':', '!', '?', '(', ')', '\n', '/', '"', '|'])
        .map(|clause| {
            clause
                .split_whitespace()
                .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).replace('\u{2019}', "'"))
                .map(|w| w.trim_end_matches("'s").to_string())
                .filter(|w| !w.is_empty())
                .collect()
        })
        .collect()
}

fn is_adverb(w: &str) -> bool {
    ["ally", "ately", "ently", "fully", "ously", "ively", "lessly", "ingly"].iter().any(|e| w.len() > e.len() + 2 && w.ends_with(e))
}

/// The action verb at `i` when it reads as something someone does, not a noun modifier:
/// "residents report leaks", "to arrange", "co-op selling", "Book a table" -> verb;
/// "repair workshop", "book shop" -> noun.
fn action_at(clause: &[String], i: usize) -> Option<&'static str> {
    let w = clause[i].as_str();
    let base = verb_base(w)?;
    let inflected = w != base;
    let prev = i.checked_sub(1).map(|j| clause[j].as_str());
    let next = clause.get(i + 1).map(String::as_str);
    let ok = match prev {
        None => matches!(next, Some("a" | "an" | "the" | "your" | "our" | "up" | "in" | "online" | "now" | "today")),
        Some("to" | "can" | "and" | "or" | "then" | "please" | "who" | "that" | "will" | "lets" | "let" | "help" | "helps" | "helping") => true,
        Some(p) if OBJECT_STOPWORDS.contains(&p) => inflected,
        // After a content word: a plural subject ("residents report") or an inflected verb ("co-op selling").
        Some(p) if verb_base(p).is_some() => false,
        Some(p) => inflected || (p.len() > 3 && p.ends_with('s') && !p.ends_with("ss")),
    };
    ok.then_some(base)
}

fn breaks_phrase(clause: &[String], i: usize, run_len: usize) -> bool {
    let w = clause[i].as_str();
    OBJECT_STOPWORDS.contains(&w)
        || is_adverb(w)
        || w.chars().all(|c| c.is_ascii_digit())
        || action_at(clause, i).is_some()
        // A participle after a noun starts a new clause ("library lending drills").
        || (run_len > 0 && w.len() > 5 && w.ends_with("ing"))
}

/// The brief's own objects: noun phrases read heuristically (runs of content words between function
/// words, verbs and quality adjectives; at most 3 words each), in the order the brief names them.
/// No trade table: a funeral home and a water utility each get their own nouns back.
pub fn brief_objects(brief: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for clause in object_words(brief) {
        let mut run: Vec<&str> = Vec::new();
        let flush = |run: &mut Vec<&str>, out: &mut Vec<String>| {
            if !run.is_empty() {
                let keep = &run[run.len().saturating_sub(3)..];
                let phrase = keep.join(" ");
                if phrase.len() > 2 && !out.contains(&phrase) {
                    out.push(phrase);
                }
                run.clear();
            }
        };
        for (i, w) in clause.iter().enumerate() {
            let participle = !run.is_empty() && w.len() > 5 && w.ends_with("ing");
            if breaks_phrase(&clause, i, run.len()) {
                flush(&mut run, &mut out);
                if participle {
                    continue;
                }
            } else {
                run.push(w);
            }
        }
        flush(&mut run, &mut out);
    }
    out.truncate(8);
    out
}

/// The visitor's action as the brief states it: the first action verb with the noun phrase it takes
/// ("lets residents report leaks" -> "report leaks"). `None` when the brief names no action; the
/// caller must then ask, never guess a booking.
/// The page's main action when the brief labels it ("Primary CTA: Get a Free Quote"). Section-level
/// "CTA:" lines are not the page's action, so only primary/main labels count.
fn labelled_cta(brief: &str) -> Option<String> {
    const LABELS: &[&str] = &["primary cta", "main cta", "primary call to action", "main call to action", "primary action", "main action", "primary button"];
    brief.split(['\n', '.', ';']).find_map(|seg| {
        let t = seg.trim().trim_start_matches(['*', '-', '\u{2022}', ' ']);
        let (label, value) = t.split_once(':')?;
        if !LABELS.contains(&label.trim().to_lowercase().as_str()) {
            return None;
        }
        let v = value.trim().trim_matches(['"', '\'', '`', '\u{201C}', '\u{201D}']).trim().to_lowercase();
        (!v.is_empty() && v.split_whitespace().count() <= 6).then_some(v)
    })
}

pub fn brief_action(brief: &str) -> Option<String> {
    if let Some(cta) = labelled_cta(brief) {
        return Some(cta);
    }
    for clause in object_words(brief) {
        for i in 0..clause.len() {
            let Some(base) = action_at(&clause, i) else { continue };
            // "sign up", "check in", "pick up": keep the particle.
            let mut verb = base.to_string();
            let mut j = i + 1;
            if let Some(p) = clause.get(j).filter(|p| matches!(p.as_str(), "up" | "in" | "out" | "off")) {
                verb = format!("{verb} {p}");
                j += 1;
            }
            let rest: Vec<String> = clause[j..]
                .iter()
                .skip_while(|t| matches!(t.as_str(), "a" | "an" | "the" | "your" | "their" | "our" | "my" | "for" | "on" | "at"))
                .cloned()
                .collect();
            let object: Vec<&str> = rest
                .iter()
                .enumerate()
                .take_while(|(k, t)| {
                    !(OBJECT_STOPWORDS.contains(&t.as_str()) || is_adverb(t) || action_at(&rest, *k).is_some() || (*k > 0 && t.len() > 5 && t.ends_with("ing")))
                })
                .take(3)
                .map(|(_, t)| t.as_str())
                .collect();
            return Some(if object.is_empty() { verb } else { format!("{verb} {}", object.join(" ")) });
        }
    }
    None
}

/// Intensity the brief asks for in so many words, if any. Restraint wins over ambition.
pub fn brief_intensity(brief: &str) -> Option<Intensity> {
    // Fallback and guardrail phrases ("degrade to a static image", "respect prefers-reduced-motion")
    // say what happens when motion is off; they do not ask for a calm page.
    let mut plain = brief.to_lowercase();
    for p in [
        "prefers-reduced-motion", "prefers reduced motion", "reduced-motion support", "reduced motion support",
        "reduced-motion fallback", "reduced motion fallback", "static image", "static fallback", "static poster",
        "static frame", "static version", "static still",
    ] {
        plain = plain.replace(p, " ");
    }
    let toks = tokens(&plain);
    // An explicit heavy-motion ask outranks a stray calm word elsewhere ("a minimal footer").
    const HEAVY: &[&str] = &[
        "highly animated", "heavily animated", "very animated", "fully animated", "cinematic", "motion-rich",
        "scrollytelling", "lots of animation", "lots of motion",
    ];
    let no_motion = ["no animation", "no motion", "without animation", "static site"].iter().any(|p| brief.to_lowercase().contains(p));
    if !no_motion && HEAVY.iter().any(|w| hit(&toks, w)) {
        return Some(Intensity::Wild);
    }
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
        Some("dashboard" | "docs" | "health" | "finance" | "professional" | "legal" | "memorial" | "civic") => Intensity::Calm,
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
        "legal" => &["services", "team", "proof", "process"],
        "museum" => &["schedule", "pricing", "gallery", "location"],
        "devtools" => &["features", "proof", "process", "docs-nav", "data-story", "comparison"],
        "memorial" => &["services", "schedule", "pricing", "about", "process"],
        "civic" => &["services", "schedule", "data-story", "location", "timeline"],
        "venue" => &["gallery", "location", "pricing", "schedule"],
        "camp" => &["schedule", "pricing", "team", "curriculum"],
        "winery" => &["menu", "location", "schedule", "gallery", "about"],
        "home-services" => &["services", "pricing", "proof", "process"],
        "industrial" => &["comparison", "catalogue", "process", "proof", "cta"],
        "body-art" => &["gallery", "team", "process", "pricing"],
        _ => &[],
    }
}

/// Niches whose mechanics depict that trade's own object; foreign to any other brief.
const NARROW_TRADES: &[&str] = &[
    "store", "restaurant", "real-estate", "dashboard", "docs", "events", "travel", "devtools", "legal", "museum",
    "memorial", "body-art", "civic", "venue", "camp", "winery", "home-services", "industrial",
];

/// Niches close enough that one's mechanics read naturally on the other's site. Used only when a
/// niche has too few in-niche entries at the asked intensity, before anything unrelated is dealt.
fn related_niches(niche: &str) -> &'static [&'static str] {
    match niche {
        "devtools" => &["saas", "docs", "dashboard", "landing"],
        "saas" => &["landing", "dashboard", "docs", "devtools"],
        "docs" => &["devtools", "saas", "education"],
        "dashboard" => &["saas", "finance", "devtools"],
        "landing" => &["saas", "store", "events"],
        "store" => &["local-business", "landing"],
        "restaurant" => &["local-business", "events"],
        "local-business" => &["restaurant", "store", "professional"],
        "agency" => &["portfolio", "kinetic", "editorial"],
        "kinetic" => &["agency", "3d", "portfolio"],
        "3d" => &["kinetic", "agency"],
        "editorial" => &["portfolio", "nonprofit"],
        "portfolio" => &["agency", "editorial"],
        "finance" => &["professional", "dashboard", "saas"],
        "health" => &["professional", "local-business"],
        "education" => &["nonprofit", "docs"],
        "events" => &["travel", "restaurant"],
        "real-estate" => &["professional", "local-business"],
        "travel" => &["events", "editorial"],
        "nonprofit" => &["education", "editorial"],
        "professional" => &["finance", "health"],
        "legal" => &["professional", "finance", "editorial"],
        "museum" => &["events", "education", "nonprofit", "travel"],
        "memorial" => &["professional", "nonprofit", "local-business"],
        "civic" => &["nonprofit", "docs", "professional"],
        "venue" => &["events", "travel", "restaurant"],
        "camp" => &["education", "events", "nonprofit"],
        "winery" => &["restaurant", "travel", "events"],
        "home-services" => &["local-business", "professional"],
        "industrial" => &["professional", "3d", "landing"],
        "body-art" => &["local-business", "portfolio", "agency"],
        _ => &[],
    }
}

/// How a dealt mechanic fits the brief, strongest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FitClass {
    /// Tagged with a niche the brief names.
    InNiche,
    /// Tagged `any`.
    Universal,
    /// Tagged with a niche next to the brief's (`related_niches`).
    Related,
    /// No niche named, or tagged only with broad niches: neither the brief's trade nor another's.
    Neutral,
    /// Built around another narrow trade's own object (a menu, a floor plan, a shop shelf).
    Foreign,
}

fn fit_class(m: &Mechanic, niches: &[String]) -> (FitClass, Option<String>) {
    // Built around another narrow trade's object: foreign even when a secondary tag sits next to
    // the brief (a menu card tagged `events` is still a restaurant's menu on a museum site).
    if let Some(primary) = m.niches.first().filter(|p| NARROW_TRADES.contains(&p.as_str()) && !niches.contains(p)) {
        // Tagged for one of the brief's own narrow trades too: shared on purpose, not borrowed.
        let shared = m.niches.iter().any(|n| NARROW_TRADES.contains(&n.as_str()) && niches.contains(n));
        if !niches.is_empty() && !shared {
            return (FitClass::Foreign, Some(primary.clone()));
        }
    }
    if let Some(n) = m.niches.iter().find(|n| niches.contains(n)) {
        return (FitClass::InNiche, Some(n.clone()));
    }
    if m.niches.iter().any(|n| n == "any") {
        return (FitClass::Universal, None);
    }
    for brief_niche in niches {
        if let Some(n) = m.niches.iter().find(|n| related_niches(brief_niche).contains(&n.as_str())) {
            return (FitClass::Related, Some(format!("{} (next to {})", n, brief_niche)));
        }
    }
    match m.niches.first() {
        Some(primary) if NARROW_TRADES.contains(&primary.as_str()) => (FitClass::Foreign, Some(primary.clone())),
        _ => (FitClass::Neutral, None),
    }
}

/// Why one dealt mechanic was picked, so the agent can judge the deal instead of trusting it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanicFit {
    pub id: String,
    /// `hero`, `signature`, `suggestion` or `transition`.
    pub slot: String,
    /// `in-niche`, `universal`, `related`, `neutral` or `outside-trade`.
    pub fit: String,
    pub reason: String,
}

/// Roles that sit at the page edges or are page furniture; they are suggested (or dealt into the
/// `nav` / `footer` slots) but never dealt as a signature.
const EDGE_ROLES: &[&str] = &["footer", "faq", "cta", "contact", "nav", "section-head", "proof"];
/// Furniture with its own deal slot, so it never takes a section suggestion.
const SLOT_ROLES: &[&str] = &["nav", "footer"];

/// Look families that read as a document (newspaper, civic form, paper page, drawing sheet).
const DOCUMENT_LOOKS: &[&str] = &["hairline-newspaper", "quiet-civic", "warm-paper-serif", "technical-blueprint"];

/// Look families that read as a staged scene: dark, sparse, photographic.
const STAGED_LOOKS: &[&str] = &["cinematic-dark", "luxe-dark", "dark-single-accent", "luxury-sparse", "documentary-photo"];

/// The header family a brief describes in words: a header laid over the hero that turns solid,
/// glass or transparent, on scroll.
fn brief_nav_family(brief_l: &str) -> Option<&'static str> {
    let header = ["header", "navigation", "navbar", "nav bar", "menu bar"].iter().any(|w| brief_l.contains(w));
    let overlay = ["overlays the hero", "overlay the hero", "over the hero", "transparent header", "transparent nav", "glass", "transparent or"]
        .iter()
        .any(|w| brief_l.contains(w));
    let settles = ["after scrolling", "on scroll", "after scroll", "when scrolling", "sticky"].iter().any(|w| brief_l.contains(w));
    (header && overlay && settles).then_some("nav-overlay-settle")
}

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
    /// Ids never dealt: recently used mechanics (MCP `avoid`), or picks held by another direction.
    /// Matched case-insensitively.
    pub exclude_ids: Vec<String>,
    /// Hero compositions already used by another direction; skipped while any other hero fits.
    pub exclude_compositions: Vec<String>,
    /// Hero composition the caller's direction is built around; a matching hero scores higher.
    pub prefer_composition: Option<String>,
    /// Re-deal offset for the hero: 0 takes the best fit, n > 0 takes a different near-best fit,
    /// so asking for another variant changes the first screen too.
    pub hero_offset: usize,
    /// The shared niche signal, already read from the brief (`intelligence::brief_signals`): used
    /// verbatim so `design-system` and `deal` never disagree about the trade. `None` reads the brief.
    pub niches: Option<Vec<String>>,
    /// Mechanic ids dealt in this project's recent runs (`.layoza/looks.jsonl`), every slot: still
    /// allowed, but scored down by more than any fit bonus, so a fresh fitting pick always wins and a
    /// repeat is dealt only when nothing fresh fits that slot. Ordered most recent first: the more
    /// recent the use, the bigger the penalty, so an exhausted slot cycles to its oldest pick.
    pub recent_ids: Vec<String>,
    /// Optional: the same ids grouped per ledger entry, most recent entry first. When set, the
    /// penalty is graded by entry distance (sharper than the flat list's position).
    pub recent_by_entry: Vec<Vec<String>>,
    /// Hero look families already used by another direction; skipped while any other hero fits.
    pub exclude_look_families: Vec<String>,
    /// Skip `giant-type` heroes while any other hero fits (set when every earlier direction is pure type).
    pub avoid_giant_type: bool,
}

/// Seeds are u64; JSON numbers above 2^53 lose precision in JavaScript clients, so they travel as
/// strings and are read back from either a string or a number.
pub mod seed_serde {
    use serde::{de, Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(v: &u64, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&v.to_string())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Either {
            S(String),
            N(u64),
        }
        match Either::deserialize(d)? {
            Either::N(n) => Ok(n),
            Either::S(t) => t.trim().parse().map_err(de::Error::custom),
        }
    }
    /// Reads a seed argument from JSON: a non-negative integer or a decimal string.
    pub fn from_json(v: &serde_json::Value) -> Option<u64> {
        v.as_u64().or_else(|| v.as_str().and_then(|t| t.trim().parse().ok()))
    }
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
    #[serde(with = "seed_serde")]
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
    /// Page furniture: a navigation mechanic for a new site (absent for existing sites, which keep theirs).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nav: Option<Mechanic>,
    /// Page furniture: a footer mechanic for a new site (absent for existing sites).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<Mechanic>,
    /// The brief's own nouns (heuristic noun phrases). Every dealt mechanic is rebuilt around these.
    #[serde(default)]
    pub brief_objects: Vec<String>,
    /// The visitor's action as the brief words it; absent when the brief names none (ask, never guess).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brief_action: Option<String>,
    /// One entry per dealt mechanic: how it fits the brief's niche, role, mood and intensity.
    #[serde(default)]
    pub fit: Vec<MechanicFit>,
    pub notes: Vec<String>,
    pub rules: Vec<String>,
}

/// Score taken off a mechanic used in the project's recent ledger entries: larger than any fit bonus
/// (niche 8 + trade role 5 + intensity 4 + moods), so a fresh fitting pick always outranks it.
const RECENT_PENALTY: i64 = 24;
/// Extra penalty for the most recent use, falling to 0 for the oldest, so repeats cycle oldest first.
const RECENCY_GRADE: i64 = 16;
/// Per-entry step of the graded penalty when ids come grouped by ledger entry.
const ENTRY_GRADE: i64 = 8;
/// Fit points below the best in-niche hero that still join the seed rotation.
const HERO_ROTATION_WINDOW: i64 = 10;
/// Fit points below the best furniture pick that still join the seed rotation.
const FURNITURE_WINDOW: i64 = 10;

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
    // What the brief rules out ("avoid dashboard mockups") must not pick the niche.
    let affirmed = crate::intelligence::affirmed_brief(opts.brief.trim());
    let brief = if affirmed.trim().is_empty() { opts.brief.trim() } else { affirmed.as_str() };
    let mut notes = Vec::new();

    // Niches: the brief's own words first, then the hint; the hint never displaces the brief.
    let mut niches = opts.niches.clone().unwrap_or_else(|| niche_signal(brief));
    // A tool brief ("bakery dashboard"): the trade is the tool's content, not its storefront.
    // (Idempotent, so a shared signal from `niche_signal` passes through unchanged.)
    if niches.first().is_some_and(|n| SURFACE_NICHES.contains(&n.as_str())) {
        let before = niches.len();
        niches.retain(|n| SURFACE_NICHES.contains(&n.as_str()) || !NARROW_TRADES.contains(&n.as_str()));
        if niches.len() < before {
            notes.push("Tool brief: the trade is the product's content, so mechanics built on its storefront (menus, shelves) are left out; show the trade's real data inside the tool.".into());
        }
    }
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
    let brief_objs = brief_objects(brief);
    let brief_act = brief_action(brief);
    if niches.is_empty() {
        notes.push(format!(
            "No catalogue niche named in the brief: dealt broad mechanics, chosen by a hash of the whole brief. Map every mechanic onto the brief's own objects ({}) and rebuild its visual around them; pass --niche for a sharper deal.",
            if brief_objs.is_empty() { "none read; ask the user what the page shows".to_string() } else { brief_objs.join(", ") }
        ));
        if brief_act.is_none() {
            notes.push("The brief names no visitor action: ask the user what the primary call to action is; do not assume booking or contact.".into());
        }
    }
    if intensity == Intensity::Calm {
        notes.push("Calm deal: every mechanic is a quiet, CSS-capable idea. Keep each section's distinct idea; do not add bold motion.".into());
    }
    let toks = tokens(brief);
    // "A static image fallback" is a fallback, not a ban: only a whole-page ask stops motion.
    if ["no animation", "no animations", "no motion", "static site", "static page", "without animation"].iter().any(|w| hit(&toks, w)) {
        notes.push("The brief asks for no motion: build each mechanic as its reduced-motion end state (the composition stays, nothing moves) and declare <meta name=\"layoza-motion\" content=\"none\">.".into());
    }

    let seed = opts.seed.unwrap_or_else(|| fnv1a(&brief.to_lowercase()));
    let count = opts.count.clamp(2, 10);

    let allowed = |m: &Mechanic| -> bool {
        let fits_intensity = if intensity == Intensity::Calm { m.intensity == Intensity::Calm } else { m.intensity <= intensity };
        fits_intensity
            && m.tier <= max_tier
            && !opts.exclude_ids.iter().any(|x| x.trim().eq_ignore_ascii_case(&m.id))
            && !opts.exclude_families.iter().any(|x| x == &m.family)
            && signal_met(m, &toks)
    };
    let avoided: Vec<&str> = opts
        .exclude_ids
        .iter()
        .map(|x| x.trim())
        .filter(|x| get_mechanic(x).is_some())
        .collect();
    if !avoided.is_empty() {
        notes.push(format!("Skipped {} excluded / recently used mechanic(s): {}.", avoided.len(), avoided.join(", ")));
    }
    // Fit score without the seeded jitter: seed-independent, so rotation windows hold the same
    // candidates for every seed and the seed only picks among them.
    let fit_score = |m: &Mechanic| -> i64 {
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
        if m.kind == MechanicKind::Hero {
            // A tool brief ("bakery dashboard") opens on the tool: a hero built around the trade's
            // shop window or menu belongs to that trade's own site, not to its software.
            if let (Some(lead), Some(primary)) = (niches.first(), m.niches.first()) {
                if SURFACE_NICHES.contains(&lead.as_str()) && primary != lead && NARROW_TRADES.contains(&primary.as_str()) && !SURFACE_NICHES.contains(&primary.as_str()) {
                    s -= 10;
                }
            }
            if opts.prefer_composition.as_deref().is_some_and(|c| !c.is_empty() && c == m.composition) {
                s += 5;
            }
            // Headline-left / visual-right is the generic default; it must win on fit, not by habit
            if m.composition == "split" {
                s -= 2;
            }
        }
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
        if let Some(a) = opts.recent_by_entry.iter().position(|e| e.iter().any(|x| x.eq_ignore_ascii_case(&m.id))) {
            s -= RECENT_PENALTY + ENTRY_GRADE * (opts.recent_by_entry.len() - a) as i64;
        } else if let Some(i) = opts.recent_ids.iter().position(|x| x.eq_ignore_ascii_case(&m.id)) {
            let n = opts.recent_ids.len() as i64;
            s -= RECENT_PENALTY + RECENCY_GRADE * (n - i as i64) / n;
        }
        s
    };
    // Seeded jitter (0-3) so one seed is reproducible and another seed explores close candidates
    // With no niche the fit scores barely differ, so a wider jitter lets the whole brief decide.
    let score = |m: &Mechanic| -> i64 {
        let spread = if niches.is_empty() { 12 } else { 4 };
        fit_score(m) + (fnv1a(&format!("{}:{}", seed, m.id)) % spread) as i64
    };
    // Pool by fit: in-niche and universal entries first; widen to related niches, then to broad
    // entries, only when too few fit at this intensity. Mechanics built around another trade's own
    // object are dealt only when nothing else is left. Returns (pool, wider tail for suggestions).
    let ranked = |kind: MechanicKind| -> (Vec<&'static Mechanic>, Vec<&'static Mechanic>) {
        let pool: Vec<&'static Mechanic> = catalogue().iter().filter(|m| m.kind == kind && allowed(m)).collect();
        let class = |m: &Mechanic| fit_class(m, &niches).0;
        let limits: &[FitClass] = if niches.is_empty() {
            &[FitClass::Neutral]
        } else {
            &[FitClass::Universal, FitClass::Related, FitClass::Neutral]
        };
        let enough = if kind == MechanicKind::Section { 2 } else { 1 };
        let sort = |v: &mut Vec<&'static Mechanic>| v.sort_by(|a, b| score(b).cmp(&score(a)).then_with(|| a.id.cmp(&b.id)));
        let mut main = pool.clone();
        for lim in limits {
            let tier: Vec<&'static Mechanic> = pool.iter().copied().filter(|m| class(m) <= *lim).collect();
            if tier.len() >= enough {
                main = tier;
                break;
            }
        }
        sort(&mut main);
        let mut tail: Vec<&'static Mechanic> = pool
            .iter()
            .copied()
            .filter(|m| class(m) < FitClass::Foreign && !main.iter().any(|x| x.id == m.id))
            .collect();
        tail.sort_by(|a, b| class(a).cmp(&class(b)).then_with(|| score(b).cmp(&score(a))).then_with(|| a.id.cmp(&b.id)));
        (main, tail)
    };

    let mut used_families: BTreeSet<String> = BTreeSet::new();
    let mut used_roles: BTreeSet<String> = BTreeSet::new();
    let take = |list: &[&'static Mechanic], fam: &mut BTreeSet<String>, extra: &dyn Fn(&Mechanic) -> bool| -> Option<Mechanic> {
        let m = list.iter().find(|m| !fam.contains(&m.family) && extra(m))?;
        fam.insert(m.family.clone());
        Some((*m).clone())
    };
    // Seeded rotation for furniture (nav, footer, section head): the candidates within
    // FURNITURE_WINDOW fit points of the best (seed-independent, at most 6, look families other
    // directions hold skipped while others fit) are ordered by fit then id, and the seed walks
    // them, so consecutive seeds give different picks.
    let take_rotated = |list: &[&'static Mechanic], fam: &mut BTreeSet<String>, extra: &dyn Fn(&Mechanic) -> bool, salt: &str| -> Option<Mechanic> {
        let all: Vec<&'static Mechanic> = list.iter().copied().filter(|m| !fam.contains(&m.family) && extra(m)).collect();
        let other_look: Vec<&'static Mechanic> =
            all.iter().copied().filter(|m| m.look_family.is_empty() || !opts.exclude_look_families.iter().any(|f| f == &m.look_family)).collect();
        let mut cands = if other_look.is_empty() { all } else { other_look };
        let top = cands.iter().map(|m| fit_score(m)).max()?;
        let is_recent = |m: &Mechanic| opts.recent_ids.iter().chain(opts.recent_by_entry.iter().flatten()).any(|x| x.eq_ignore_ascii_case(&m.id));
        if cands.iter().any(|m| fit_score(m) == top && is_recent(m)) {
            // Pool used up: of the near-best fits, the least recently used one is dealt, so a
            // strong fit cannot win every run just because its lead beats the recency grade.
            let age = |m: &Mechanic| opts.recent_ids.iter().position(|x| x.eq_ignore_ascii_case(&m.id)).unwrap_or(usize::MAX);
            let m = cands
                .iter()
                .copied()
                .filter(|m| fit_score(m) + RECENT_PENALTY + RECENCY_GRADE + FURNITURE_WINDOW >= top)
                .max_by(|a, b| age(a).cmp(&age(b)).then_with(|| fit_score(a).cmp(&fit_score(b))))?;
            fam.insert(m.family.clone());
            return Some(m.clone());
        }
        // Only fresh picks rotate.
        cands.retain(|m| fit_score(m) + FURNITURE_WINDOW >= top && !is_recent(m));
        cands.sort_by(|a, b| fit_score(b).cmp(&fit_score(a)).then_with(|| a.id.cmp(&b.id)));
        cands.truncate(6);
        let i = (seed.wrapping_add(fnv1a(salt)) % cands.len() as u64) as usize;
        let m = cands[i];
        fam.insert(m.family.clone());
        Some(m.clone())
    };

    let (heroes, _) = ranked(MechanicKind::Hero);
    let free_comp = |m: &Mechanic| !opts.exclude_compositions.iter().any(|c| c == &m.composition);
    let hero = {
        // Near-best candidates (within 6 points of the best fit, at most 4) in a fresh composition;
        // the offset walks them so a re-deal shows a different first screen.
        let fresh: Vec<&'static Mechanic> = heroes.iter().copied().filter(|m| free_comp(m)).collect();
        let list: Vec<&'static Mechanic> = if fresh.is_empty() { heroes.clone() } else { fresh };
        // Look diversity: never a second hero from a look family another direction already holds,
        // and no pure-type hero when the earlier directions are all type, while anything else fits.
        let other_look = |m: &&'static Mechanic| m.look_family.is_empty() || !opts.exclude_look_families.iter().any(|f| f == &m.look_family);
        let not_type = |m: &&'static Mechanic| !opts.avoid_giant_type || m.hero_material != "giant-type";
        let diverse: Vec<&'static Mechanic> = list.iter().copied().filter(other_look).filter(not_type).collect();
        let list: Vec<&'static Mechanic> = if !diverse.is_empty() {
            diverse
        } else {
            let some: Vec<&'static Mechanic> = list.iter().copied().filter(other_look).collect();
            if some.is_empty() { list } else { some }
        };
        let best = list.first().map(|m| score(m)).unwrap_or(0);
        // The caller's composition is a promise made to the reader (the direction says "centred"),
        // so a fitting hero of that composition wins outright when it is within 10 points of the
        // best; only when none is that close does the window fall back to any composition.
        let wanted = |m: &&'static Mechanic| opts.prefer_composition.as_deref().is_some_and(|c| c == m.composition);
        // Never keep the promise with a borrowed hero: another trade's own object, or a trade's shop
        // window on a tool brief. Then the direction follows the fitting hero instead.
        let borrowed = |m: &Mechanic| {
            let lead = niches.first().map(String::as_str).unwrap_or("");
            let primary = m.niches.first().map(String::as_str).unwrap_or("");
            fit_class(m, &niches).0 == FitClass::Foreign
                || (SURFACE_NICHES.contains(&lead) && primary != lead && NARROW_TRADES.contains(&primary) && !SURFACE_NICHES.contains(&primary))
        };
        let matching: Vec<&'static Mechanic> = list.iter().copied().filter(wanted).filter(|m| !borrowed(m)).collect();
        let window = |v: &[&'static Mechanic], slack: i64| -> Vec<&'static Mechanic> {
            let top = v.first().map(|m| score(m)).unwrap_or(0);
            v.iter().copied().filter(|m| score(m) + slack >= top).take(4).collect()
        };
        // A composition match from a broader class (a universal hero) keeps the promise only when it
        // nearly ties the best; an in-trade hero is not given up for a generic one's layout.
        let best_class = list.first().map(|m| fit_class(m, &niches).0);
        let near: Vec<&'static Mechanic> = match matching.first() {
            Some(m) if score(m) + if Some(fit_class(m, &niches).0) == best_class { 10 } else { 3 } >= best => window(&matching, 6),
            // No niche: every broad hero is about equally (un)fitting, so a tiny score window must
            // not decide. The whole brief picks among all of them, so different briefs diverge.
            _ if niches.is_empty() => {
                let recent = |m: &&'static Mechanic| opts.recent_ids.iter().any(|x| x.eq_ignore_ascii_case(&m.id));
                let fresh: Vec<&'static Mechanic> = list.iter().copied().filter(|m| !recent(m)).collect();
                if fresh.is_empty() { list.clone() } else { fresh }
            }
            _ => window(&list, 6),
        };
        // Seed rotation: with an explicit seed, rotate among the qualifying in-niche heroes (within
        // 10 points of the best, at most 5) so different seeds open on different first screens; the
        // in-niche bonus alone would otherwise win on every seed.
        // The window is built from the seed-independent fit score and ordered by fit then id, and the
        // seed walks it (seed mod len), so consecutive seeds always open on different heroes.
        let rotation: Vec<&'static Mechanic> = if opts.seed.is_some() && !niches.is_empty() && opts.hero_offset == 0 {
            let base: &[&'static Mechanic] = if matching.first().is_some_and(|m| score(m) + 10 >= best) { &matching } else { &list };
            let mut r: Vec<&'static Mechanic> = base.iter().copied().filter(|m| fit_class(m, &niches).0 == FitClass::InNiche).collect();
            let all_in_niche: Vec<&'static Mechanic> = r.clone();
            let top = r.iter().map(|m| fit_score(m)).max().unwrap_or(0);
            let is_recent = |m: &Mechanic| opts.recent_ids.iter().chain(opts.recent_by_entry.iter().flatten()).any(|x| x.eq_ignore_ascii_case(&m.id));
            // Only fresh heroes rotate; once the best one is itself a repeat, no rotation (the
            // graded penalty then deals the least recently used hero).
            let top_fresh = r.iter().any(|m| fit_score(m) == top && !is_recent(m));
            r.retain(|m| fit_score(m) + HERO_ROTATION_WINDOW >= top && top_fresh && !is_recent(m));
            r.sort_by(|a, b| fit_score(b).cmp(&fit_score(a)).then_with(|| a.id.cmp(&b.id)));
            // A thin window (composition filter, one dominant fit) still rotates over at least three
            // fresh in-niche heroes when the niche has them: the nearest-fitting ones fill it.
            if top_fresh && r.len() < 3 {
                let mut more: Vec<&'static Mechanic> = all_in_niche.iter().copied().filter(|m| !is_recent(m) && !r.iter().any(|x| x.id == m.id)).collect();
                more.sort_by(|a, b| fit_score(b).cmp(&fit_score(a)).then_with(|| a.id.cmp(&b.id)));
                r.extend(more.into_iter().take(3 - r.len()));
            }
            r.truncate(6);
            r
        } else {
            Vec::new()
        };
        let rotating = rotation.len() > 1;
        let near = if rotating { rotation } else { near };
        let pick = if niches.is_empty() && !near.is_empty() {
            ((fnv1a(&format!("{}:hero", brief.to_lowercase())) ^ seed) as usize).wrapping_add(opts.hero_offset) % near.len()
        } else if rotating {
            (seed % near.len() as u64) as usize
        } else if near.len() > 1 && opts.hero_offset > 0 {
            1 + (opts.hero_offset - 1) % (near.len() - 1)
        } else {
            0
        };
        let chosen = near.get(pick).copied();
        let ordered: Vec<&'static Mechanic> = chosen.into_iter().chain(list.iter().copied()).collect();
        take(&ordered, &mut used_families, &|_| true)
    };
    match &hero {
        None => notes.push("No hero mechanic fits these constraints.".into()),
        Some(h) if !niches.is_empty() && !h.niches.iter().any(|n| n == "any" || niches.contains(n)) => notes.push(
            "No in-niche hero at this intensity: dealt the closest fit; rebuild it around the trade's own object.".into(),
        ),
        _ => {}
    }

    let (sections, wider) = ranked(MechanicKind::Section);
    let mut signature = Vec::new();
    for _ in 0..2 {
        let pick = {
            let roles = used_roles.clone();
            let ok = |m: &Mechanic| !EDGE_ROLES.contains(&m.role.as_str()) && !roles.contains(&m.role);
            take(&sections, &mut used_families, &ok).or_else(|| take(&wider, &mut used_families, &ok))
        };
        if let Some(m) = pick {
            used_roles.insert(m.role.clone());
            signature.push(m);
        }
    }
    let mut suggestions = Vec::new();
    while signature.len() + suggestions.len() < count {
        let roles = used_roles.clone();
        let new_role = |m: &Mechanic| !roles.contains(&m.role) && !SLOT_ROLES.contains(&m.role.as_str());
        let not_slot = |m: &Mechanic| !SLOT_ROLES.contains(&m.role.as_str());
        let mut pick = take(&sections, &mut used_families, &new_role)
            .or_else(|| take(&sections, &mut used_families, &not_slot))
            .or_else(|| take(&wider, &mut used_families, &new_role))
            .or_else(|| take(&wider, &mut used_families, &not_slot));
        // A section head is furniture too: rotate it by seed instead of always the top fit.
        if let Some(m) = pick.clone().filter(|m| m.role == "section-head") {
            used_families.remove(&m.family);
            let is_head = |x: &Mechanic| x.role == "section-head";
            pick = take_rotated(&sections, &mut used_families, &is_head, "section-head")
                .or_else(|| take_rotated(&wider, &mut used_families, &is_head, "section-head"))
                .or(Some(m.clone()));
            if let Some(p) = &pick {
                used_families.insert(p.family.clone());
            }
        }
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
    let (transitions, _) = ranked(MechanicKind::Transition);
    let transition = take(&transitions, &mut used_families, &|_| true);
    // Page furniture slots, new sites only: an existing site keeps its own navigation and footer.
    let (nav, footer) = if opts.existing.is_none() {
        let brief_l = opts.brief.to_lowercase();
        // A cinematic or immersive brief never gets a newspaper, civic, paper or drawing-sheet header or footer.
        let cinematic = ["cinematic", "immersive", "filmic"].iter().any(|k| brief_l.contains(k));
        let staged = |m: &Mechanic| !cinematic || !DOCUMENT_LOOKS.contains(&m.look_family.as_str());
        let asked_nav = brief_nav_family(&brief_l);
        let mut slot = |role: &str| {
            // The header the brief describes in words is dealt before any rotation.
            if role == "nav" {
                if let Some(fam) = asked_nav {
                    let asked = |m: &Mechanic| m.role == "nav" && m.family == fam;
                    if let Some(m) = take_rotated(&sections, &mut used_families, &asked, role).or_else(|| take_rotated(&wider, &mut used_families, &asked, role)) {
                        return Some(m);
                    }
                }
            }
            let fitting = |m: &Mechanic| m.role == role && staged(m);
            let is_role = |m: &Mechanic| m.role == role;
            // A cinematic brief's furniture comes from the staged looks first.
            let scene = |m: &Mechanic| cinematic && m.role == role && STAGED_LOOKS.contains(&m.look_family.as_str());
            take_rotated(&sections, &mut used_families, &scene, role)
                .or_else(|| take_rotated(&wider, &mut used_families, &scene, role))
                .or_else(|| take_rotated(&sections, &mut used_families, &fitting, role))
                .or_else(|| take_rotated(&wider, &mut used_families, &fitting, role))
                .or_else(|| take_rotated(&sections, &mut used_families, &is_role, role))
                .or_else(|| take_rotated(&wider, &mut used_families, &is_role, role))
        };
        (slot("nav"), slot("footer"))
    } else {
        (None, None)
    };

    // Fit reasons, one per dealt mechanic
    let fit_reason = |m: &Mechanic, slot: &str| -> MechanicFit {
        let (class, detail) = fit_class(m, &niches);
        let detail = detail.unwrap_or_default();
        let (label, mut parts) = match class {
            FitClass::InNiche => ("in-niche", vec![format!("tagged {}, a niche the brief names", detail)]),
            FitClass::Universal => ("universal", vec!["tagged `any`: fits every trade".to_string()]),
            FitClass::Related => ("related", vec![format!("related niche {}; too few in-niche entries fit", detail)]),
            FitClass::Neutral if niches.is_empty() => {
                ("neutral", vec!["no niche named; broad mechanic, not another trade's own object".to_string()])
            }
            FitClass::Neutral => (
                "neutral",
                vec![format!("not tagged {}; broad mechanic dealt because closer entries ran out", niches.join("/"))],
            ),
            FitClass::Foreign => (
                "outside-trade",
                vec![format!("built for {}; dealt only because nothing closer fits; rebuild it around this trade's object", detail)],
            ),
        };
        if m.kind == MechanicKind::Section {
            match niches.iter().find(|n| trade_roles(n).contains(&m.role.as_str())) {
                Some(n) => parts.push(format!("role `{}` is a {} trade role", m.role, n)),
                None => parts.push(format!("role `{}`", m.role)),
            }
        }
        let shared: Vec<&str> = m.moods.iter().filter(|x| moods.contains(x)).map(String::as_str).collect();
        if !shared.is_empty() {
            parts.push(format!("mood {}", shared.join(", ")));
        }
        parts.push(format!("{} / {} within the deal's {} / {}", m.intensity.as_str(), m.tier.as_str(), intensity.as_str(), max_tier.as_str()));
        MechanicFit { id: m.id.clone(), slot: slot.into(), fit: label.into(), reason: parts.join("; ") }
    };
    let mut fit: Vec<MechanicFit> = Vec::new();
    fit.extend(hero.iter().map(|m| fit_reason(m, "hero")));
    fit.extend(signature.iter().map(|m| fit_reason(m, "signature")));
    fit.extend(suggestions.iter().map(|m| fit_reason(m, "suggestion")));
    fit.extend(transition.iter().map(|m| fit_reason(m, "transition")));
    fit.extend(nav.iter().map(|m| fit_reason(m, "nav")));
    fit.extend(footer.iter().map(|m| fit_reason(m, "footer")));
    let outside: Vec<&str> = fit.iter().filter(|f| f.fit == "outside-trade").map(|f| f.id.as_str()).collect();
    if !outside.is_empty() {
        notes.push(format!(
            "Dealt from outside the trade because nothing closer fits these constraints: {}. Rebuild each around this trade's own object or replace it.",
            outside.join(", ")
        ));
    }

    let rules = vec![
        "Map every dealt mechanic onto `brief_objects` (the brief's own nouns): the moving or featured thing is one of them, never the catalogue's example object.".into(),
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
        nav,
        footer,
        brief_objects: brief_objs,
        brief_action: brief_act,
        fit,
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
    /// Hero only: its first-screen composition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub composition: String,
    /// Hero only: what the first screen is made of (`real-photo`, `giant-type`, ...).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hero_material: String,
    /// Visual look family (heroes and furniture: nav, footer, section head); no two directions
    /// share a hero look family.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub look_family: String,
    /// Conditional mechanics: the brief words that made it eligible (any one). Empty = always.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requires_signal: Vec<String>,
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
            composition: m.composition.clone(),
            hero_material: m.hero_material.clone(),
            look_family: m.look_family.clone(),
            requires_signal: m.requires_signal.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn similes_are_not_niches_and_heavy_motion_is_wild() {
        let b = "open-source MCP server that makes coding agents design like a studio; highly animated";
        let n = detect_niches(b);
        assert!(n.contains(&"devtools".to_string()), "{:?}", n);
        assert!(!n.contains(&"agency".to_string()), "{:?}", n);
        assert_eq!(detect_niches("brand studio portfolio"), vec!["agency".to_string(), "portfolio".to_string()]);
        assert_eq!(brief_intensity(b), Some(Intensity::Wild));
        assert_eq!(brief_intensity("cinematic, but no animation"), Some(Intensity::Calm));
        assert_eq!(brief_intensity("highly animated site with a minimal, subtle footer"), Some(Intensity::Wild));
    }

    #[test]
    fn long_brief_ranks_niches_by_weight_and_trade_leads_technique() {
        // A technique word named first does not lead a trade
        assert_eq!(detect_niches("three.js hero for an open-source cli").first().map(String::as_str), Some("devtools"));
        // A long spec that keeps returning to its trade, with passing mentions of others
        let mut b = String::from("Three.js hero, kinetic scroll, showcase a restaurant demo and a store demo. ");
        for _ in 0..6 {
            b.push_str("The MCP server gives coding agents design skills; install the CLI from npm and read the docs on GitHub. ");
        }
        let n = detect_niches(&b);
        assert_eq!(n.first().map(String::as_str), Some("devtools"), "{n:?}");
        assert!(!n.contains(&"restaurant".to_string()) && !n.contains(&"store".to_string()), "{n:?}");
        let d = deal_mechanics(&format!("{b} highly animated"), None, None, 5, 3);
        assert!(d.hero.as_ref().is_some_and(|h| h.niches.contains(&"devtools".to_string())), "{:?}", d.hero.map(|h| h.id));
        for m in all(&d) {
            assert!(!["travel", "store", "real-estate", "events"].contains(&m.niches[0].as_str()), "{}", m.id);
        }
    }

    use super::*;
    use std::collections::{BTreeMap, HashSet};

    #[test]
    fn heroes_declare_compositions_and_splits_are_a_minority() {
        let heroes: Vec<&Mechanic> = catalogue().iter().filter(|m| m.kind == MechanicKind::Hero).collect();
        let known = ["layered", "centred", "object-wrap", "poster", "stacked", "split", "full-bleed", "sticky-stage"];
        for h in &heroes {
            assert!(known.contains(&h.composition.as_str()), "{} composition {:?}", h.id, h.composition);
        }
        let splits = heroes.iter().filter(|h| h.composition == "split").count();
        assert!(splits * 5 <= heroes.len(), "{splits} of {} heroes are splits", heroes.len());
        let kinds: HashSet<&str> = heroes.iter().map(|h| h.composition.as_str()).collect();
        assert!(kinds.len() >= 6, "{kinds:?}");
    }

    #[test]
    fn composition_preference_and_exclusion_steer_the_hero() {
        let base = DealOptions { brief: "seafood restaurant on the harbour".into(), count: 2, seed: Some(7), ..Default::default() };
        let h0 = deal_mechanics_with(&base).hero.expect("hero");
        let excluded = deal_mechanics_with(&DealOptions { exclude_compositions: vec![h0.composition.clone()], ..base.clone() }).hero.expect("hero");
        assert_ne!(excluded.composition, h0.composition);
        // A preference is soft: it keeps a fitting hero of that composition, never forces an off-niche one.
        let kept = deal_mechanics_with(&DealOptions { prefer_composition: Some(h0.composition.clone()), ..base.clone() }).hero.expect("hero");
        assert_eq!(kept.composition, h0.composition);
        // Across many briefs, a split hero is never the majority pick
        let briefs = ["seafood restaurant", "linen store", "bakery dashboard", "open-source cli", "architecture studio", "hotel in Porto", "kids museum", "law firm"];
        let splits = briefs.iter().filter(|b| deal_mechanics_with(&DealOptions { brief: b.to_string(), count: 2, ..Default::default() }).hero.is_some_and(|h| h.composition == "split")).count();
        assert!(splits * 2 < briefs.len(), "{splits} split heroes of {}", briefs.len());
    }

    fn ids(d: &MechanicDeal) -> Vec<String> {
        d.hero.iter().chain(&d.signature).chain(&d.suggestions).chain(&d.transition).chain(&d.nav).chain(&d.footer).map(|m| m.id.clone()).collect()
    }
    fn all(d: &MechanicDeal) -> Vec<&Mechanic> {
        d.hero.iter().chain(&d.signature).chain(&d.suggestions).chain(&d.transition).chain(&d.nav).chain(&d.footer).collect()
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
    fn catalogue_is_broad_with_a_universal_hero_pool() {
        let cat = catalogue();
        assert!(cat.len() >= 260, "{} entries", cat.len());
        let universal: Vec<&Mechanic> =
            cat.iter().filter(|m| m.kind == MechanicKind::Hero && m.niches.iter().any(|n| n == "any")).collect();
        assert!(universal.len() >= 60, "{} universal heroes", universal.len());
        for c in ["layered", "centred", "object-wrap", "poster", "stacked", "split", "full-bleed", "sticky-stage"] {
            assert!(universal.iter().any(|m| m.composition == c), "no universal {c} hero");
        }
        assert!(universal.iter().filter(|m| m.intensity == Intensity::Calm).all(|m| m.tier == Tier::Css));
        assert!(cat.iter().filter(|m| m.kind == MechanicKind::Transition).count() >= 15);
        let role = |r: &str| cat.iter().filter(|m| m.role == r).count();
        assert!(role("nav") >= 12 && role("footer") >= 10 && role("section-head") >= 8 && role("proof") >= 8, "role coverage");
    }

    #[test]
    fn every_hero_names_its_material_and_look() {
        const MATERIALS: &[&str] =
            &["real-photo", "generated-image", "procedural", "data-image", "giant-type", "physical-object", "video"];
        for m in catalogue().iter().filter(|m| m.kind == MechanicKind::Hero) {
            assert!(MATERIALS.contains(&m.hero_material.as_str()), "{}: hero_material {:?}", m.id, m.hero_material);
            assert!(!m.look_family.trim().is_empty(), "{}: no look_family", m.id);
        }
        // The universal pool is not all typography.
        let giant = catalogue().iter().filter(|m| m.kind == MechanicKind::Hero && m.hero_material == "giant-type").count();
        let heroes = catalogue().iter().filter(|m| m.kind == MechanicKind::Hero).count();
        assert!(giant * 4 < heroes, "{giant} of {heroes} heroes are giant type");
    }

    #[test]
    fn new_trades_are_detected() {
        for (brief, niche) in [
            ("funeral home", "memorial"),
            ("municipal water utility", "civic"),
            ("tattoo studio", "body-art"),
            ("vineyard wedding venue", "winery"),
            ("kids coding camp", "camp"),
            ("emergency plumber", "home-services"),
            ("craft brewery taproom", "winery"),
            ("barn wedding venue", "venue"),
        ] {
            let n = detect_niches(brief);
            assert!(n.iter().any(|x| x == niche), "{brief}: {n:?}");
        }
        assert_eq!(detect_niches("funeral home").first().map(String::as_str), Some("memorial"));
        assert_eq!(normalize_niche("funeral home"), "memorial");
        assert_eq!(normalize_niche("electrician"), "home-services");
    }

    #[test]
    fn every_niche_has_hero_depth_and_look_spread() {
        for n in NICHES.iter().filter(|n| **n != "any") {
            let heroes: Vec<&Mechanic> =
                catalogue().iter().filter(|m| m.kind == MechanicKind::Hero && m.niches.iter().any(|x| x == n)).collect();
            let looks: HashSet<&str> = heroes.iter().map(|m| m.look_family.as_str()).collect();
            assert!(heroes.len() >= 4 && looks.len() >= 2, "{n}: {} heroes, {} looks", heroes.len(), looks.len());
        }
        for n in ["civic", "memorial"] {
            let calm: HashSet<&str> = catalogue()
                .iter()
                .filter(|m| m.kind == MechanicKind::Hero && m.intensity == Intensity::Calm && m.niches.iter().any(|x| x == n))
                .map(|m| m.look_family.as_str())
                .collect();
            assert!(calm.len() >= 4, "{n}: calm heroes span only {} looks", calm.len());
        }
    }

    /// Seeds 1-5 rotate the hero for every niche deep enough to rotate (4+ heroes), and the calm
    /// furniture (nav, footer) for the same briefs.
    #[test]
    fn consecutive_seeds_rotate_hero_and_furniture() {
        for n in NICHES.iter().filter(|n| **n != "any") {
            // Eligible depth: the distinct in-niche heroes this brief can be dealt at all (intensity
            // and signals filter some catalogue entries out), measured over 40 seeds.
            let depth = (1..=40u64)
                .filter_map(|s| deal_mechanics_with(&DealOptions { brief: format!("{n} website"), niches: Some(vec![n.to_string()]), seed: Some(s), count: 2, ..Default::default() }).hero)
                .filter(|m| m.niches.iter().any(|x| x == n))
                .map(|m| m.id)
                .collect::<HashSet<String>>()
                .len();
            let deals: Vec<MechanicDeal> = (1..=5u64)
                .map(|s| deal_mechanics_with(&DealOptions { brief: format!("{n} website"), niches: Some(vec![n.to_string()]), seed: Some(s), count: 2, ..Default::default() }))
                .collect();
            let distinct = |f: &dyn Fn(&MechanicDeal) -> Option<String>| deals.iter().filter_map(f).collect::<HashSet<String>>().len();
            if depth >= 4 {
                let h = distinct(&|d| d.hero.as_ref().map(|m| m.id.clone()));
                let ids: Vec<String> = deals.iter().filter_map(|d| d.hero.as_ref().map(|m| m.id.clone())).collect();
                assert!(h >= 3, "{n}: seeds 1-5 gave {h} distinct heroes: {ids:?}");
            }
            let nav = distinct(&|d| d.nav.as_ref().map(|m| m.id.clone()));
            let footer = distinct(&|d| d.footer.as_ref().map(|m| m.id.clone()));
            assert!(nav >= 3 && footer >= 3, "{n}: seeds 1-5 gave {nav} navs, {footer} footers");
        }
        for brief in ["family funeral home offering cremation", "municipal water utility where residents report leaks"] {
            let heroes: HashSet<String> =
                (1..=2u64).filter_map(|s| deal_mechanics_with(&DealOptions { brief: brief.into(), seed: Some(s), count: 2, ..Default::default() }).hero.map(|m| m.id)).collect();
            assert_eq!(heroes.len(), 2, "{brief}: seeds 1 and 2 dealt the same hero");
        }
    }

    #[test]
    fn every_niche_has_varied_furniture() {
        // Unconditional furniture each niche can draw from: the universal pool plus its own entries.
        for (role, need) in [("nav", 6), ("footer", 6), ("section-head", 4)] {
            for m in catalogue().iter().filter(|m| m.role == role) {
                assert!(!m.look_family.trim().is_empty(), "{}: no look_family", m.id);
            }
            for n in NICHES.iter().filter(|n| **n != "any") {
                let pool: Vec<&Mechanic> = catalogue()
                    .iter()
                    .filter(|m| m.role == role && m.requires_signal.is_empty())
                    .filter(|m| m.niches.iter().any(|x| x == "any" || x == n))
                    .collect();
                let looks: HashSet<&str> = pool.iter().map(|m| m.look_family.as_str()).collect();
                assert!(pool.len() >= need && looks.len() >= 3, "{n} {role}: {} options, {} looks", pool.len(), looks.len());
                let calm = pool.iter().filter(|m| m.intensity == Intensity::Calm).count();
                assert!(calm >= 3, "{n} {role}: only {calm} calm options");
            }
        }
    }

    #[test]
    fn conditional_furniture_needs_its_signal() {
        let bilingual = "nav-bilingual-mirrored";
        assert!(!get_mechanic(bilingual).expect("entry").requires_signal.is_empty());
        for m in catalogue() {
            assert!(m.requires_signal.iter().all(|w| !w.trim().is_empty()), "{}: blank signal", m.id);
        }
        // Without the signal it is never dealt, whatever the seed or niche.
        for brief in ["neighbourhood bakery", "municipal water utility", "seafood restaurant", "a website for my project"] {
            for seed in 0..12 {
                let d = deal_mechanics(brief, None, None, 6, seed);
                let dealt = all(&d).into_iter().chain(&d.nav).chain(&d.footer).any(|m| !m.requires_signal.is_empty());
                assert!(!dealt, "{brief} seed {seed}: conditional mechanic dealt without its signal");
            }
        }
        // With the signal it is eligible, and some seed deals it as the nav.
        let brief = "bilingual Arabic and English community centre";
        assert!((0..24).any(|seed| deal_mechanics(brief, None, None, 4, seed).nav.is_some_and(|m| m.id == bilingual)));
    }

    #[test]
    fn funeral_home_and_water_utility_get_different_heroes() {
        for seed in 0..5 {
            let a = deal_mechanics("funeral home", None, None, 4, seed).hero.expect("hero");
            let b = deal_mechanics("municipal water utility", None, None, 4, seed).hero.expect("hero");
            assert_ne!(a.id, b.id, "seed {seed}");
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
    fn kids_museum_and_bakery_dashboard_get_their_own_objects() {
        assert_eq!(detect_niches("kids museum with hands-on exhibits").first().map(String::as_str), Some("museum"));
        assert_eq!(detect_niches("bakery dashboard").first().map(String::as_str), Some("dashboard"));
        for seed in 0..12 {
            let d = deal_mechanics("kids museum with hands-on exhibits", None, None, 2, seed);
            for m in picks(&d) {
                let primary = m.niches.first().map(String::as_str).unwrap_or("");
                assert!(primary == "museum" || !NARROW_TRADES.contains(&primary), "seed {seed}: museum dealt {} ({primary})", m.id);
            }
            let hero = deal_mechanics("bakery dashboard", None, None, 2, seed).hero.expect("hero");
            let primary = hero.niches.first().map(String::as_str).unwrap_or("");
            assert!(!["restaurant", "store", "local-business"].contains(&primary), "seed {seed}: bakery dashboard hero {}", hero.id);
        }
    }

    #[test]
    fn law_firm_gets_no_other_trades_objects() {
        for seed in 0..12 {
            let d = deal_mechanics("corporate law firm", None, None, 4, seed);
            assert!(d.context.niches.iter().any(|n| n == "legal"), "{:?}", d.context.niches);
            for m in picks(&d) {
                let primary = m.niches.first().map(String::as_str).unwrap_or("");
                assert!(primary == "legal" || !NARROW_TRADES.contains(&primary), "seed {seed}: law firm dealt {} ({primary})", m.id);
                for trade_object in ["timeline-tape-measure-track", "proof-before-after-measured", "hero-odometer-in-headline"] {
                    assert_ne!(m.id, trade_object, "seed {seed}");
                }
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

    const DEV_BRIEF: &str = "marketing site for an open-source MCP server and AI agent skills that make coding agents design better";

    #[test]
    fn devtools_brief_gets_devtools_fits_and_no_foreign_trades() {
        assert_eq!(detect_niches(DEV_BRIEF).first().map(String::as_str), Some("devtools"));
        for seed in 0..16 {
            let d = deal_mechanics(DEV_BRIEF, None, None, 6, seed);
            assert!(d.hero.is_some(), "seed {seed}: no hero");
            for m in all(&d) {
                let primary = m.niches.first().map(String::as_str).unwrap_or("");
                assert!(
                    !["restaurant", "store", "real-estate", "travel", "events", "local-business"].contains(&primary),
                    "seed {seed}: dev brief dealt {} ({primary})",
                    m.id
                );
                assert!(!["menu", "location", "curriculum", "itinerary", "listing", "catalogue"].contains(&m.role.as_str()), "seed {seed}: {}", m.id);
            }
            // Every dealt mechanic carries a fit reason, in deal order
            let dealt = ids(&d);
            assert_eq!(d.fit.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), dealt, "seed {seed}");
            for f in &d.fit {
                assert!(!f.reason.trim().is_empty() && f.fit != "outside-trade", "seed {seed}: {:?}", f);
            }
            let h = d.fit.first().unwrap();
            assert_eq!((h.slot.as_str(), h.fit.as_str()), ("hero", "in-niche"), "seed {seed}");
        }
        // Every niche can still be named: devtools has heroes and sections of its own
        assert!(list_mechanics(Some("cli"), Some(MechanicKind::Hero), None).iter().any(|m| m.id == "hero-terminal-session-replay"));
    }

    #[test]
    fn devtools_words_do_not_steal_other_niches() {
        for (brief, lead) in [
            ("seafood restaurant with a daily menu", "restaurant"),
            ("developer documentation for an SDK", "docs"),
            ("analytics dashboard", "dashboard"),
            ("estate agent in Leeds", "real-estate"),
            ("travel agent for walking holidays", "travel"),
        ] {
            let n = detect_niches(brief);
            assert_eq!(n.first().map(String::as_str), Some(lead), "{brief}: {n:?}");
        }
        assert!(!detect_niches("travel agent for walking holidays").contains(&"devtools".to_string()));
        // A restaurant deal never gets the terminal replay or the diff
        for seed in 0..8 {
            let d = deal_mechanics("seafood restaurant with a daily menu", None, None, 6, seed);
            assert!(ids(&d).iter().all(|i| i != "hero-terminal-session-replay" && i != "proof-before-after-diff"), "seed {seed}");
        }
    }

    #[test]
    fn avoid_list_is_never_dealt_and_stays_deterministic() {
        let first = deal_mechanics(DEV_BRIEF, None, None, 5, 3);
        let avoid = ids(&first);
        let opts = DealOptions {
            brief: DEV_BRIEF.into(),
            count: 5,
            seed: Some(3),
            exclude_ids: avoid.iter().map(|i| i.to_uppercase()).collect(),
            ..Default::default()
        };
        let a = deal_mechanics_with(&opts);
        let b = deal_mechanics_with(&opts);
        assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
        for i in ids(&a) {
            assert!(!avoid.contains(&i), "avoided {i} was dealt again");
        }
        assert!(a.hero.is_some(), "avoid list must not empty the hero pool");
        assert!(a.notes.iter().any(|n| n.contains("recently used")));
        assert_eq!(a.fit.len(), ids(&a).len());
    }

    #[test]
    fn generic_brief_excludes_narrow_trade_objects() {
        for seed in 0..12 {
            let d = deal_mechanics("a website for my project", None, None, 6, seed);
            for f in &d.fit {
                assert_ne!(f.fit, "outside-trade", "seed {seed}: {:?}", f);
            }
        }
    }
}
