// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
//! Direction dealer for `design-system`: decoupled type and palette pools, layout archetypes
//! (layout composition + hero material + look family + corner language + knobs), and a small
//! brief reader that writes each direction's copy and rationale from the brief's own subject.
//!
//! The pools live in `directions/*.json` (Layoza's own data, loaded once). Nothing here fixes a
//! design: every direction is a set of options with the reason it was picked and the
//! alternatives it beat, so the agent can choose differently with the reason written down.

use std::sync::OnceLock;

use serde::Deserialize;

use crate::dna::DesignDials;
use crate::intelligence::{
    fnv1a, keyword_hit, pairing, palette, tokenize, verify_palette, wcag_contrast, BriefSignals, IndustryRule, PaletteRecommendation, Surface,
    TypographyPairing,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tone {
    Light,
    Dark,
    Field,
}

impl Tone {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Tone::Light => "light ground",
            Tone::Dark => "dark ground",
            Tone::Field => "saturated colour field",
        }
    }
    /// Classifies any palette by its background: strongly chromatic grounds are colour fields,
    /// otherwise relative luminance decides light or dark.
    pub(crate) fn of(p: &PaletteRecommendation) -> Tone {
        let l = luminance(&p.background).unwrap_or(0.5);
        let ch = rgb(&p.background).map(|(r, g, b)| [r, g, b]).unwrap_or([0, 0, 0]);
        let chroma = ch.iter().max().zip(ch.iter().min()).map(|(a, b)| a - b).unwrap_or(0);
        if chroma >= 60 {
            Tone::Field
        } else if l > 0.5 {
            Tone::Light
        } else if l < 0.03 {
            Tone::Dark
        } else {
            Tone::Field
        }
    }
    fn parse(s: &str) -> Tone {
        match s {
            "dark" => Tone::Dark,
            "field" => Tone::Field,
            _ => Tone::Light,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Voice {
    Editorial,
    Expressive,
    Utility,
}

impl Voice {
    fn label(self) -> &'static str {
        match self {
            Voice::Editorial => "editorial",
            Voice::Expressive => "expressive",
            Voice::Utility => "utility",
        }
    }
    fn parse(s: &str) -> Voice {
        match s {
            "editorial" => Voice::Editorial,
            "expressive" => Voice::Expressive,
            _ => Voice::Utility,
        }
    }
}

fn rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let h = hex.trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let c = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok();
    Some((c(0)?, c(2)?, c(4)?))
}

/// WCAG relative luminance, 0 (black) to 1 (white).
pub(crate) fn luminance(hex: &str) -> Option<f64> {
    wcag_contrast(hex, "#000000").map(|c| c * 0.05 - 0.05)
}

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn leak_all(v: Vec<String>) -> &'static [&'static str] {
    Box::leak(v.into_iter().map(leak).collect::<Vec<_>>().into_boxed_slice())
}

/// One type pairing with the facts the dealer scores on.
pub(crate) struct TypeEntry {
    pub pairing: TypographyPairing,
    pub voice: Voice,
    pub tags: &'static [&'static str],
    /// Writing systems both display and body faces cover (`latin`, `japanese`, `arabic`, ...).
    pub scripts: &'static [&'static str],
    /// Families seen on most AI-built sites (Inter, Poppins, Fraunces...): allowed, flagged, scored lower.
    pub overused: bool,
    pub look_family: &'static str,
    /// Axis value: serif, sans, condensed, display, slab, mono, rounded or hand.
    pub display_style: &'static str,
    /// `google` or `fontshare`.
    pub source: &'static str,
    pub licence: &'static str,
    /// CSS fallback stacks for display and body, matched to each face's class.
    pub fallbacks: &'static str,
    /// Stylesheet for the mono face when it is not in the main URL (Fontshare pairings load their
    /// mono from Google Fonts); empty when the main URL already carries it.
    pub mono_url: &'static str,
}

/// One palette with the axes the dealer scores on.
pub(crate) struct PaletteEntry {
    pub palette: PaletteRecommendation,
    pub tone: Tone,
    pub tags: &'static [&'static str],
    pub look_family: &'static str,
    /// Axis value: warm, cool or neutral.
    pub warmth: &'static str,
    /// Axis value: the primary accent's hue family.
    pub accent_hue: &'static str,
}

#[derive(Deserialize)]
struct RawType {
    id: String,
    display: String,
    body: String,
    mono: String,
    mood: String,
    ratio: String,
    voice: String,
    look_family: String,
    display_style: String,
    tags: Vec<String>,
    scripts: Vec<String>,
    licence: String,
    source: String,
    overused: bool,
    display_class: String,
    body_class: String,
    url: String,
    #[serde(default)]
    mono_url: String,
}

#[derive(Deserialize)]
struct RawPalette {
    id: String,
    bg: String,
    surface: String,
    fg: String,
    muted: String,
    primary: String,
    secondary: String,
    notes: String,
    tags: Vec<String>,
    look_family: String,
    warmth: String,
    accent_hue: String,
}

#[derive(Deserialize)]
struct RawArchetype {
    key: String,
    name: String,
    style_family: String,
    surfaces: Vec<String>,
    roles: Vec<String>,
    tone: String,
    voice: String,
    composition: String,
    calm_bias: i8,
    look_family: String,
    hero_material: String,
    radius: Vec<String>,
    #[serde(default)]
    registers: Vec<String>,
    tags: Vec<String>,
    avoid: Vec<String>,
    needs: Vec<String>,
    density: String,
    span_irregularity: String,
    accent_placement: String,
    why: String,
    structure: String,
    motion_hint: String,
    mobile: String,
    layout: String,
    identity: String,
    content: String,
    motion_calm: String,
    motion_bold: String,
}

/// CSS fallback stack for a face class from the data files.
fn fallback(class: &str) -> &'static str {
    match class {
        "serif" | "didone" => "Georgia, 'Times New Roman', serif",
        "slab" => "Rockwell, 'Roboto Slab', Georgia, serif",
        "round" => "ui-rounded, 'SF Pro Rounded', system-ui, sans-serif",
        "mono" | "pixel" => "ui-monospace, 'SFMono-Regular', Menlo, Consolas, monospace",
        "hand" => "'Segoe Print', 'Bradley Hand', cursive",
        "display" => "Impact, 'Arial Narrow Bold', system-ui, sans-serif",
        "condensed" => "'Arial Narrow', 'Roboto Condensed', system-ui, sans-serif",
        _ => "system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif",
    }
}

/// Every family is on Google Fonts (SIL OFL 1.1, or Apache 2.0 for the Roboto faces) or on
/// Fontshare (ITF Free Font License); `licence` and `source` say which.
pub(crate) fn type_pool() -> &'static [TypeEntry] {
    static POOL: OnceLock<Vec<TypeEntry>> = OnceLock::new();
    POOL.get_or_init(|| {
        let raw: Vec<RawType> = serde_json::from_str(include_str!("directions/types.json")).expect("directions/types.json");
        raw.into_iter()
            .map(|r| {
                let fallbacks = leak(format!("{}: {}; {}: {}", r.display, fallback(&r.display_class), r.body, fallback(&r.body_class)));
                TypeEntry {
                    pairing: pairing(&r.id, &r.display, &r.body, &r.mono, &r.mood, &r.ratio, &r.url),
                    voice: Voice::parse(&r.voice),
                    tags: leak_all(r.tags),
                    scripts: leak_all(r.scripts),
                    overused: r.overused,
                    look_family: leak(r.look_family),
                    display_style: leak(r.display_style),
                    source: leak(r.source),
                    licence: leak(r.licence),
                    fallbacks,
                    mono_url: leak(r.mono_url),
                }
            })
            .collect()
    })
}

/// Body and muted text clear WCAG AA (>= 4.5:1) on background and surface for every palette
/// (tested); the contrast figure is computed, never typed in.
pub(crate) fn palette_pool() -> &'static [PaletteEntry] {
    static POOL: OnceLock<Vec<PaletteEntry>> = OnceLock::new();
    POOL.get_or_init(|| {
        let raw: Vec<RawPalette> = serde_json::from_str(include_str!("directions/palettes.json")).expect("directions/palettes.json");
        raw.into_iter()
            .map(|r| {
                let mut p = palette(&r.id, &r.bg, &r.surface, &r.fg, &r.muted, &r.primary, &r.secondary, 0.0, &r.notes);
                verify_palette(&mut p);
                let tone = Tone::of(&p);
                PaletteEntry { palette: p, tone, tags: leak_all(r.tags), look_family: leak(r.look_family), warmth: leak(r.warmth), accent_hue: leak(r.accent_hue) }
            })
            .collect()
    })
}

/// A direction structure: layout composition, hero material, look family, corner language,
/// palette tone, type voice and the written templates for each part of the direction.
pub(crate) struct Archetype {
    pub key: &'static str,
    pub name: &'static str,
    pub style_family: &'static str,
    /// Surfaces it fits; empty = every surface.
    pub surfaces: &'static [Surface],
    /// Direction roles it can fill: A (structural), B (expressive), C (the working tool).
    pub roles: &'static [&'static str],
    pub tone: Tone,
    pub voice: Voice,
    /// First-screen composition the hero is built around.
    pub composition: &'static str,
    /// +1 suits calm briefs, -1 suits bold ones.
    pub calm_bias: i8,
    /// Visual family; the three directions never share one.
    pub look_family: &'static str,
    /// What the first screen is made of (giant-type, photo, document, map, live-ui...).
    pub hero_material: &'static str,
    /// Corner languages it suits, best first.
    pub radius: &'static [&'static str],
    /// Registers where this layout reads as world-class studio work (see `crate::register`).
    pub registers: &'static [&'static str],
    /// Subjects it suits (scored), subjects it must not serve, subjects it requires (any one).
    pub tags: &'static [&'static str],
    pub avoid: &'static [&'static str],
    pub needs: &'static [&'static str],
    /// Knobs: how packed sections are, how irregular the spans are, where the accent goes.
    pub density: &'static str,
    pub span_irregularity: &'static str,
    pub accent_placement: &'static str,
    /// When this structure is the right answer.
    pub why: &'static str,
    pub structure: &'static str,
    pub motion_hint: &'static str,
    pub mobile: &'static str,
    layout: &'static str,
    identity: &'static str,
    content: &'static str,
    motion_calm: &'static str,
    motion_bold: &'static str,
}

impl Archetype {
    /// The layout opens on an index or register with no image above the fold, so the page shape
    /// never deals it an image-led hero frame.
    pub(crate) fn no_hero_image(&self) -> bool {
        let text = format!("{} {}", self.layout, self.identity).to_lowercase();
        ["no hero-image", "no hero image", "no imagery above", "no images above"].iter().any(|k| text.contains(k))
    }
}

fn surface_of(s: &str) -> Surface {
    match s {
        "product-ui" => S::ProductUi,
        "commerce" => S::Commerce,
        "venue" => S::Venue,
        "showcase" => S::Showcase,
        "editorial" => S::Editorial,
        "devtool" => S::DevTool,
        _ => S::General,
    }
}

/// The archetype pool (Layoza's own layouts, see `directions/archetypes.json`).
#[allow(non_snake_case)]
pub(crate) fn ARCHETYPES() -> &'static [Archetype] {
    static POOL: OnceLock<Vec<Archetype>> = OnceLock::new();
    POOL.get_or_init(|| {
        let raw: Vec<RawArchetype> = serde_json::from_str(include_str!("directions/archetypes.json")).expect("directions/archetypes.json");
        raw.into_iter()
            .map(|r| Archetype {
                key: leak(r.key),
                name: leak(r.name),
                style_family: leak(r.style_family),
                surfaces: Box::leak(r.surfaces.iter().map(|s| surface_of(s)).collect::<Vec<_>>().into_boxed_slice()),
                roles: leak_all(r.roles),
                tone: Tone::parse(&r.tone),
                voice: Voice::parse(&r.voice),
                composition: leak(r.composition),
                calm_bias: r.calm_bias,
                look_family: leak(r.look_family),
                hero_material: leak(r.hero_material),
                radius: leak_all(r.radius),
                registers: leak_all(r.registers),
                tags: leak_all(r.tags),
                avoid: leak_all(r.avoid),
                needs: leak_all(r.needs),
                density: leak(r.density),
                span_irregularity: leak(r.span_irregularity),
                accent_placement: leak(r.accent_placement),
                why: leak(r.why),
                structure: leak(r.structure),
                motion_hint: leak(r.motion_hint),
                mobile: leak(r.mobile),
                layout: leak(r.layout),
                identity: leak(r.identity),
                content: leak(r.content),
                motion_calm: leak(r.motion_calm),
                motion_bold: leak(r.motion_bold),
            })
            .collect()
    })
}

use Surface as S;

/// Corner-radius language, written out for the identity text.
fn radius_text(key: &str) -> &'static str {
    match key {
        "sharp-0" => "square corners (0) on cards, buttons and images",
        "hairline-2" => "2px on controls and cards, square images",
        "tool-6" => "6px on controls, 8px on panels, square images",
        "soft-12" => "12px on cards and images, 8px on controls",
        "round-play" => "fully rounded buttons and chips, 20px cards",
        "stamp-circle" => "square blocks with circular stamps and badges as the only round shapes",
        "ticket-notch" => "ticket shapes: notched or perforated corners on cards, square controls",
        "cut-corner" => "45-degree clipped corners on panels and buttons",
        "arched" => "arch-topped images and frames, 4px controls",
        _ => "square structure, rounded only on small tags",
    }
}

/// The trade's objects, primary action, notation and subject tags, read from the brief's words.
struct Trade {
    words: &'static [&'static str],
    surface: Option<Surface>,
    objects: [&'static str; 3],
    action: &'static str,
    notation: &'static str,
    /// Subject tags; `generic` marks a broad word (studio, shop, app) that never leads the action.
    tags: &'static [&'static str],
}

const fn tr(
    words: &'static [&'static str],
    surface: Option<Surface>,
    objects: [&'static str; 3],
    action: &'static str,
    notation: &'static str,
    tags: &'static [&'static str],
) -> Trade {
    Trade { words, surface, objects, action, notation, tags }
}

const TRADES: &[Trade] = &[
    tr(&["dashboard", "admin", "console", "analytics", "ops", "operations", "backoffice"], Some(S::ProductUi),
        ["today's numbers", "the exception list", "the plan table"], "resolve what is late or short", "units, timestamps, owners and status", &["data"]),
    tr(&["sales", "revenue", "orders", "takings"], Some(S::ProductUi),
        ["the day's takings", "the order book", "the product mix"], "see what sold and reorder", "units sold, margins and order times", &["data"]),
    tr(&["law", "citizenship", "citizens advice", "advice centre", "immigration", "lawyer", "lawyers", "attorney", "attorneys", "solicitor", "solicitors", "legal", "barrister", "barristers", "notary", "litigation"], None,
        ["the case file", "the signed agreement", "the hearing calendar"], "book a first consultation", "matter numbers, clause references and hearing dates", &["legal"]),
    tr(&["kids", "children", "child", "toddlers", "toddler", "toy", "toys", "playground"], None,
        ["hands-on activities mid-use", "the activity map", "the stamp card"], "plan the day by age and time", "age bands, session times and wayfinding colours", &["play"]),
    tr(&["photography gallery", "photographic gallery", "photograph archive", "photography archive"], Some(S::Showcase),
        ["the complete photograph", "the artist and collection caption", "the selected works"], "view the photographs and enquire", "artist names, titles, dates and collection sources", &["civic"]),
    tr(&["museum", "exhibition", "exhibitions", "gallery", "heritage"], Some(S::Venue),
        ["the object on its plinth", "the wall label", "the floor plan"], "plan a visit and book a timed ticket", "accession numbers, wall labels and gallery numbers", &["civic"]),
    tr(&["seafood", "fish", "oyster", "oysters", "fishmonger"], Some(S::Venue),
        ["the day's catch on ice", "the oyster tray", "the chalkboard of landings"], "book a table", "catch weights, landing times and market prices", &["marine", "food"]),
    tr(&["bakery", "bread", "sourdough", "pastry", "patisserie", "baker", "loaves"], Some(S::Venue),
        ["loaves cooling on the rack", "the oven door", "the flour-dusted bench"], "order for collection", "bake times, batch sizes and weights", &["warm-grain", "food"]),
    tr(&["restaurant", "food truck", "street food", "bistro", "brasserie", "diner", "kitchen", "trattoria", "pizzeria", "pizza", "ramen", "izakaya", "taqueria"], Some(S::Venue),
        ["the plate on the pass", "tonight's menu", "the dining room"], "book a table", "menu prices, covers and service times", &["food"]),
    tr(&["cafe", "coffee", "espresso", "roastery", "roaster"], Some(S::Venue),
        ["the cup on the counter", "the roast profile", "the bag label"], "order ahead or find the counter", "roast dates, origins and brew ratios", &["food", "warm-grain"]),
    tr(&["linen", "fabric", "textile", "textiles", "cotton", "wool", "silk", "denim"], Some(S::Commerce),
        ["the weave up close", "the swatch card", "the garment on the body"], "choose a size and add to bag", "fibre content, weight in GSM and care symbols", &["textile"]),
    tr(&["fashion", "apparel", "clothing", "womenswear", "menswear", "garment"], Some(S::Commerce),
        ["the piece on the body", "the fit notes", "the look"], "choose a size and add to bag", "size charts, fit notes and care labels", &["textile"]),
    tr(&["kinetic", "motion", "animation", "showreel"], Some(S::Showcase),
        ["type in motion", "the frame sequence", "the edit timeline"], "open the reel or start a project", "frame numbers, timecodes and easing curves", &["kinetic"]),
    tr(&["smd", "signage", "billboard", "billboards", "led screen", "led screens", "led display", "led displays", "led wall", "led walls", "video wall", "video walls", "jumbotron"], None,
        ["the lit screen on its site", "the module and its pixel pitch", "the cabinet from the side"], "request a site survey or quote", "pixel pitch, nits, viewing distance and IP rating", &["industrial", "infrastructure"]),
    // Web studios name their clients' trades in sample work ("a sailing-school booking app");
    // their own trade must lead so those nouns do not lend the studio a school's notation.
    tr(&["web development", "web developer", "web developers", "web design", "web designer", "web agency", "web studio", "digital agency", "development studio", "development agency"], Some(S::Showcase),
        ["the site running in the browser", "the build beside its code", "the launch checklist"], "book an intro call or start a project", "stacks, sprint dates and launch dates", &["code"]),
    tr(&["agency", "studio", "portfolio", "designer", "photographer"], Some(S::Showcase),
        ["the work itself", "the process sketches", "the client credits"], "open a case study or start a project", "project numbers, years and credits", &["generic"]),
    tr(&["cli", "sdk", "api", "devtool", "devtools", "dev tool", "dev tools", "developer", "developers", "terminal", "mcp", "compiler", "software library", "javascript library", "python library", "component library"], Some(S::DevTool),
        ["the command and its real output", "the diff it produces", "the config file"], "copy the install command", "commands, flags, exit codes and versions", &["code"]),
    tr(&["saas", "software", "platform", "app"], None,
        ["the product screen doing its job", "the before and after of one task", "the plan table"], "start a trial", "plan limits, uptime and changelog dates", &["generic"]),
    // Dentistry owns its objects: a dental practice shown as a generic treatment room loses the
    // teeth, the X-ray and the chair that make it recognisable at a glance.
    tr(&["dental", "dental clinic", "dental practice", "dental surgery", "dental care", "dentist", "dentists", "dentistry", "orthodontist", "orthodontics", "teeth", "tooth"], None,
        ["the teeth on the dental chart", "the X-ray on the light box", "the chair under the lamp"], "book an appointment", "tooth numbers, treatment names, appointment times and fixed prices", &["care"]),
    tr(&["clinic", "physiotherapist", "chiropractor", "chiropractic", "osteopath", "podiatrist", "podiatry", "nutritionist", "dietitian", "pediatrician", "paediatrician", "orthopedic", "orthopaedic", "dermatologist", "dermatology", "health", "healthcare", "doctor", "physio", "physiotherapy", "therapy", "therapist", "medical", "gp", "surgery", "optician", "optometrist"], None,
        ["the treatment room", "the appointment slot", "the clinician"], "book an appointment", "appointment times, treatment names and prices", &["care"]),
    tr(&["gym", "dance studio", "dance classes", "personal trainer", "karate", "dojo", "martial arts", "boxing", "crossfit", "yoga", "pilates", "fitness", "climbing"], None,
        ["the class in progress", "the timetable", "the studio floor"], "book a class", "class times, levels and spaces left", &["sport"]),
    tr(&["hotel", "bed and breakfast", "motel", "hospitality", "inn", "guesthouse", "hostel", "resort"], Some(S::Venue),
        ["the room at first light", "the view from the window", "the key on the desk"], "check dates and book a room", "room types, nightly rates and check-in times", &["luxe"]),
    tr(&["architect", "architects", "architecture", "interiors"], Some(S::Showcase),
        ["the building on its site", "the section drawing", "the material sample"], "view a project or start a brief", "supplied project names, categories and design responses", &["modernist"]),
    tr(&["bookshop", "literary agent", "authors", "bookstore", "bookseller", "books", "publisher"], None,
        ["spines on the shelf", "the opening page", "the staff-pick card"], "find a book or reserve it", "ISBNs, shelf marks and publication dates", &["literary"]),
    tr(&["wine", "vineyard", "winery", "vintner", "cellar"], None,
        ["the bottle and its label", "the vineyard rows", "the tasting sheet"], "order a case or book a tasting", "vintages, grape varieties and parcel names", &["earth"]),
    tr(&["brewery", "microbrewery", "distillery", "distiller", "gin", "whisky", "whiskey", "spirits", "beverage brand", "drinks brand", "cider", "beer", "taproom"], Some(S::Venue),
        ["the can art", "the tap list", "the brew kit"], "see what is on tap", "ABV, IBU and batch dates", &["warm-grain", "food"]),
    tr(&["estate", "real estate developer", "property developer", "property developers", "property", "properties", "realtor", "listings"], None,
        ["the home itself", "the floor plan", "the street outside"], "book a viewing", "floor areas, prices and energy ratings", &["generic"]),
    tr(&["charity", "food bank", "food banks", "blood donation", "nonprofit", "non-profit", "ngo", "foundation", "donate", "donation", "donations"], None,
        ["the people helped", "the project on the ground", "where the money goes"], "donate or volunteer", "amounts, places and reporting dates", &["civic"]),
    tr(&["salon", "sauna", "bathhouse", "hammam", "makeup artist", "barber", "hairdresser", "spa", "beauty", "nails"], Some(S::Venue),
        ["the chair", "the cut itself", "the price list"], "book a slot", "services, durations and prices", &["luxe"]),
    tr(&["vet", "veterinarian", "veterinarians", "animal hospital", "veterinary", "pets", "pet", "dog", "dogs", "cats"], None,
        ["the animal in care", "the waiting room", "the treatment plan"], "book an appointment", "species, treatments and opening hours", &["care"]),
    tr(&["school", "tutor", "tutors", "test prep", "exam prep", "driving instructor", "driving lessons", "education", "educational", "edtech", "university", "college", "course", "courses", "academy", "tutoring"], None,
        ["the class at work", "the timetable", "the work students make"], "apply or book an open day", "term dates, course codes and fees", &["civic"]),
    tr(&["festival", "record label", "dj", "djs", "music", "band", "concert", "gig", "tour"], Some(S::Showcase),
        ["the stage", "the line-up", "the ticket"], "buy a ticket", "set times, stages and dates", &["celebration", "kinetic"]),
    tr(&["florist", "flowers", "garden", "plants"], None,
        ["stems in the bucket", "the bouquet being tied", "the delivery card"], "order a bouquet for a date", "stem names, seasons and delivery slots", &["botanical"]),
    tr(&["bicycle", "bike", "bikes", "cycling", "cyclist"], None,
        ["the part on the stand", "the tool wall", "the job ticket"], "book a repair slot", "part numbers, torque values and job tickets", &["industrial"]),
    tr(&["repair", "repairs", "workshop", "mechanic", "mechanics", "garage"], None,
        ["the part on the bench", "the tool wall", "the job ticket"], "book a repair slot", "part numbers, job tickets and turnaround times", &["industrial", "generic"]),
    tr(&["ceramics", "pottery", "clay", "kiln", "stoneware"], None,
        ["the pot on the wheel", "the glaze test tiles", "the kiln shelf"], "buy a piece or book a class", "glaze codes, firing temperatures and batch numbers", &["earth"]),
    tr(&["bank", "credit union", "building society", "financial advisor", "financial adviser", "financial planner", "wealth management", "mortgage", "mortgage broker", "pension", "pensions", "finance", "fintech", "accounting", "accountant", "accountants", "insurance", "investment"], None,
        ["the statement", "the account balance", "the signed form"], "open an account or book a call", "rates, fees and dates", &["finance"]),
    tr(&["magazine", "journal", "blog", "newsletter", "publication"], Some(S::Editorial),
        ["the cover", "the lead story", "the issue index"], "read the lead story or subscribe", "issue numbers, bylines and dates", &["literary"]),
    tr(&["travel", "hiking guide", "guided hikes", "mountain guide", "scuba", "scuba diving", "dive centre", "dive center", "tours", "trips", "holiday", "holidays"], None,
        ["the place itself", "the route", "the day-by-day itinerary"], "check dates and book", "dates, distances and prices", &["generic"]),
    tr(&["watch", "diamond dealer", "diamonds", "gold dealer", "bullion", "gold bullion", "watchmaker", "horologist", "watches", "jewellery", "jewelry", "jeweller", "jeweler"], Some(S::Commerce),
        ["the piece at macro scale", "the movement or setting", "the hallmark"], "book a viewing or buy", "calibres, carats and hallmarks", &["luxe"]),
    tr(&["phone", "phones", "smartphone", "smartphones", "mobile phones", "electronics", "gadgets", "headphones", "earbuds", "laptop", "laptops"], Some(S::Commerce),
        ["the device in the hand", "the colourways side by side", "the camera and screen up close"], "choose a model and add to bag", "storage sizes, battery hours, prices and delivery dates", &["tech"]),
    tr(&["store", "school uniform", "uniform supplier", "shop", "ecommerce", "e-commerce", "online store", "webshop", "boutique"], Some(S::Commerce),
        ["the product", "price and stock", "the parcel"], "add to bag", "prices, stock and delivery dates", &["generic"]),
    // Trades added with the wider pools.
    tr(&["funeral", "funerals", "undertaker", "undertakers", "mortuary", "cremation", "crematorium", "bereavement"], None,
        ["the chapel set for a service", "the order of service", "the notices register"], "arrange a funeral or read a notice", "service dates, times and places", &["memorial"]),
    tr(&["cemetery", "memorial", "memorials", "headstone", "headstones", "burial"], None,
        ["the grounds in season", "the stone and its lettering", "the plot map"], "find a grave or arrange a memorial", "plot numbers, dates and inscriptions", &["memorial"]),
    tr(&["hospice", "palliative"], None,
        ["the quiet room", "the care team", "the family's day"], "ask for help or refer someone", "visiting hours, contacts and support times", &["care", "memorial"]),
    tr(&["tattoo", "tattoos", "tattooist", "tattooer", "piercing", "irezumi"], None,
        ["the flash sheet", "the artist's healed work", "the booking calendar"], "book a consultation with an artist", "artist names, session lengths and deposit terms", &["ink"]),
    tr(&["utility", "energy supplier", "energy company", "utilities", "water", "wastewater", "sewer", "electricity", "gas supply"], None,
        ["the supply map", "the service notice", "the meter and its bill"], "report a problem or check your supply", "zones, outage times, tariffs and account numbers", &["infrastructure", "civic"]),
    tr(&["council", "embassy", "consulate", "municipal", "municipality", "borough", "city hall", "government", "civic", "parish", "county"], None,
        ["the service a resident needs", "the notice board", "the ward map"], "find a service and do it online", "service names, ward names and dates", &["civic"]),
    tr(&["wedding", "weddings", "bridal", "banquet", "event venue", "events venue"], Some(S::Venue),
        ["the room set for the day", "the ceremony spot", "the date calendar"], "check a date and book a viewing", "capacities, dates and package prices", &["celebration"]),
    tr(&["camp", "camps", "bootcamp", "summer camp", "holiday club"], None,
        ["campers mid-project", "the week-by-week programme", "the kit list"], "pick a week and enrol", "weeks, age bands, places left and fees", &["play"]),
    tr(&["nursery", "childcare", "daycare", "preschool", "kindergarten", "creche"], None,
        ["the room at play", "the daily rhythm", "the key person"], "book a visit and ask about places", "age rooms, session times and fees", &["play", "care"]),
    tr(&["plumber", "plumbers", "plumbing", "heating", "boiler", "hvac", "drainage"], None,
        ["the job on site", "the van and the call-out area", "the fixed-price list"], "get a quote or book a call-out", "call-out fees, response times and service areas", &["trades"]),
    tr(&["electrician", "kitchen fitter", "kitchen fitters", "kitchen installer", "kitchen installation", "bathroom fitter", "bathroom fitters", "construction", "construction company", "flooring", "floor fitter", "electricians", "electrical", "roofer", "roofers", "roofing", "builder", "builders", "contractor", "contractors", "joiner", "carpenter", "decorator", "handyman", "renovation", "renovations"], None,
        ["the finished job", "the crew on site", "the quote sheet"], "get a quote or book a call-out", "day rates, certifications and service areas", &["trades"]),
    tr(&["locksmith", "gunsmith", "gun shop", "shooting range", "glazier", "pest"], None,
        ["the van at the door", "the job done", "the price list"], "call now or book a slot", "response times, prices and areas covered", &["trades"]),
    tr(&["landscaping", "landscaper", "landscapers", "gardener", "gardeners", "tree surgeon", "arborist"], None,
        ["the garden before and after", "the planting plan", "the crew at work"], "get a quote for the garden", "areas, plant names and seasons", &["botanical", "trades"]),
    tr(&["cleaning", "moving company", "self storage", "storage units", "laundromat", "laundrette", "dry cleaner", "cleaner", "cleaners", "removals", "removal", "movers"], None,
        ["the room after the job", "the van and crew", "the fixed-price list"], "get a price and book a date", "room counts, hours and prices", &["trades"]),
    tr(&["solar", "heat pump", "insulation", "retrofit"], None,
        ["the panels on the roof", "the savings sheet", "the install day"], "get a survey and quote", "kW, payback years and grant amounts", &["infrastructure", "trades"]),
    tr(&["logistics", "freight", "shipping", "courier", "couriers", "haulage", "haulier", "trucking", "cargo", "warehouse", "fulfilment", "fulfillment"], None,
        ["the load on its lane", "the route map", "the load board"], "get a rate or track a shipment", "tracking numbers, lanes, weights and ETAs", &["infrastructure", "industrial"]),
    tr(&["invoicing", "invoice", "invoices", "billing", "payments", "payroll", "bookkeeping", "tax", "expenses"], None,
        ["the invoice itself", "the payment status", "the ledger"], "send a first invoice", "invoice numbers, amounts, due dates and currencies", &["finance"]),
    tr(&["bar", "cocktail", "cocktails", "pub", "speakeasy", "nightclub", "club night"], Some(S::Venue),
        ["the drink on the bar", "the back bar", "the room at night"], "book a table or see what is on", "drink names, prices and opening hours", &["night", "food"]),
    tr(&["church", "chapel", "parish church", "cathedral", "temple", "mosque", "synagogue"], None,
        ["the building and its doors", "the service times", "the community at work"], "find a service time", "service times, festivals and notices", &["civic"]),
    tr(&["public library", "library", "community library", "municipal library", "libraries", "reading room"], Some(S::Venue),
        ["the reading room", "the catalogue", "the events board"], "find a book or join", "shelf marks, opening hours and event dates", &["literary", "civic"]),
    tr(&["farm", "farmshop", "orchard", "dairy", "cheesemaker", "creamery"], None,
        ["the field in season", "the produce box", "the farm-gate board"], "order a box or visit the farm", "seasons, weights and collection days", &["earth", "food"]),
    tr(&["butcher", "health food store", "health food shop", "wholefoods", "supermarket", "supermarkets", "hypermarket", "convenience store", "deli", "grocer", "grocery", "greengrocer", "cheesemonger"], Some(S::Venue),
        ["the counter in the morning", "the price board", "the order slip"], "order for collection", "weights, cuts and prices", &["food"]),
    tr(&["pharmacy", "pharmacist", "chemist"], None,
        ["the counter", "the prescription", "the opening hours"], "order a repeat prescription", "opening hours, services and prices", &["care"]),
    tr(&["recruitment", "recruiter", "staffing", "careers", "jobs", "hiring"], None,
        ["the open roles", "the people placed", "the process"], "see open roles or register", "role titles, salaries and locations", &["generic"]),
    tr(&["coworking", "co-working", "workspace", "office space"], Some(S::Venue),
        ["the desk at work", "the floor plan", "the day pass"], "book a tour or a day pass", "desk counts, prices and opening hours", &["modernist"]),
    tr(&["sports club", "tennis court", "tennis courts", "padel court", "squash court", "basketball court", "food court", "court booking", "court hire", "book a court", "golf", "golf club", "equestrian", "horse riding", "riding school", "stables", "sports team", "football", "rugby", "cricket", "tennis", "athletics", "swimming", "league"], None,
        ["the team at play", "the fixtures list", "the clubhouse"], "join or see fixtures", "fixtures, results and membership fees", &["sport"]),
    tr(&["cinema", "comedy club", "stand-up comedy", "circus", "magician", "dance company", "theatre", "theater", "playhouse", "opera"], Some(S::Venue),
        ["the stage or screen", "tonight's programme", "the seat plan"], "pick a showing and book seats", "showtimes, runtimes and seat rows", &["night", "celebration"]),
    tr(&["vinyl", "record shop", "record store"], Some(S::Commerce),
        ["the sleeve in the crate", "the new arrivals", "the listening post"], "find a record or reserve it", "catalogue numbers, labels and pressings", &["literary", "night"]),
    tr(&["car", "cars", "dealership", "dealer", "motors", "vehicles"], Some(S::Commerce),
        ["the car on the forecourt", "the spec sheet", "the test drive"], "book a test drive", "mileage, prices and model years", &["industrial"]),
    tr(&["tea", "teahouse", "matcha"], None,
        ["the leaf and the cup", "the steeping guide", "the tin label"], "order a tin or book a tasting", "origins, harvests and steep times", &["food", "botanical"]),
    tr(&["chocolate", "chocolatier", "confectionery", "sweets", "ice cream", "gelato", "gelateria"], None,
        ["the piece cut open", "the flavour board", "the gift box"], "order a box or find the shop", "origins, percentages and flavours", &["food", "celebration"]),
    // Category coverage: every product category a brief can name gets its own objects, so the page
    // shows that trade's things instead of "the business itself".
    tr(&["luxury", "luxury brand", "maison", "couture", "haute couture"], Some(S::Commerce),
        ["the single piece under one light", "the atelier hands at work", "the house archive"], "book a private appointment", "collection names, edition numbers and maison dates", &["luxe", "generic"]),
    tr(&["b2b", "b2b service", "professional services", "managed service", "managed services"], None,
        ["the client's problem in their own numbers", "the engagement plan by week", "the signed-off result"], "book a scoping call", "service levels, response times and contract terms", &["generic"]),
    tr(&["productivity", "to-do", "todo", "task list", "task manager"], Some(S::ProductUi),
        ["today's list with one item ticked", "the week laid out", "the focus view"], "start a free list", "due dates, priorities and streaks", &["data"]),
    tr(&["remote work", "remote team", "remote teams", "collaboration", "whiteboard", "team chat"], Some(S::ProductUi),
        ["the shared board mid-edit", "the teammates' cursors", "the thread that settled it"], "invite the team", "time zones, mentions and edit history", &["data"]),
    tr(&["gaming", "game developer", "game studio", "game studios", "indie game", "video game", "video games", "game studio", "esports", "e-sports"], Some(S::Showcase),
        ["the playable character in motion", "the world map", "the trailer frame"], "wishlist or play now", "platforms, release dates and player counts", &["play", "night"]),
    tr(&["ev charging", "ev charger", "ev chargers", "charging station", "charging stations", "chargepoint", "electric vehicle", "electric vehicles"], None,
        ["the charger plugged into the car", "the station map", "the charge curve"], "find a charger or get one installed", "kilowatts, minutes to 80% and price per kWh", &["infrastructure", "tech"]),
    tr(&["subscription box", "subscription boxes", "monthly box", "box subscription"], Some(S::Commerce),
        ["this month's box opened", "the items laid out", "the delivery calendar"], "start a subscription", "box contents, delivery dates and price per month", &["celebration"]),
    tr(&["knowledge base", "documentation", "docs site", "help center", "help centre", "user guide"], Some(S::Editorial),
        ["the search box with an answer", "the guide's table of contents", "the code or screenshot in the article"], "search the docs", "article titles, versions and updated dates", &["code", "literary"]),
    tr(&["hyperlocal", "neighbourhood services", "neighborhood services", "local services", "odd jobs", "errands"], None,
        ["the neighbour doing the job", "the street map with helpers", "the job card with a price"], "post a job or book a helper", "distances, arrival times and fixed prices", &["trades"]),
    tr(&["streaming", "tv channel", "broadcaster", "television", "ott", "video on demand", "streaming service", "watch online"], Some(S::Showcase),
        ["the title poster wall", "the paused frame", "the continue-watching row"], "start watching", "seasons, episodes, runtimes and plans", &["night"]),
    tr(&["peer-to-peer", "p2p", "rental marketplace", "resale", "second-hand", "secondhand", "pre-owned"], Some(S::Commerce),
        ["the item and its owner", "the listing photo with a price", "the handover"], "list an item or rent one", "prices, deposits and seller ratings", &["generic"]),
    tr(&["marketplace", "marketplaces"], Some(S::Commerce),
        ["the listing grid", "the seller's stall", "the item in a buyer's hands"], "browse listings or start selling", "prices, seller ratings and delivery times", &["generic"]),
    tr(&["senior care", "elderly", "eldercare", "elder care", "care home", "care homes", "assisted living", "retirement home", "home care"], None,
        ["the carer and the resident", "the day's routine", "the visiting room"], "arrange a visit or call", "care levels, visiting times and weekly fees", &["care"]),
    tr(&["airline", "private jet", "jet charter", "charter flights", "airlines", "flight", "flights", "airport", "aviation"], Some(S::Commerce),
        ["the aircraft at the gate", "the route map", "the boarding pass"], "search flights", "flight numbers, departure times and fares", &["infrastructure"]),
    tr(&["event management", "event planner", "event planners", "event planning", "events agency", "corporate events"], None,
        ["the room set for the event", "the run sheet", "the guest list"], "plan an event or get a quote", "dates, headcounts and timings", &["celebration"]),
    tr(&["membership site", "membership community", "members club", "members' club", "online community", "community platform"], None,
        ["the members together", "the member card", "the programme of meetups"], "join", "member numbers, tiers and annual fees", &["civic"]),
    tr(&["digital products", "digital product", "digital downloads", "digital download", "ebook", "ebooks", "presets", "templates"], Some(S::Commerce),
        ["the file previewed open", "the bundle contents", "the result made with it"], "buy and download", "file formats, page counts and licences", &["generic"]),
    tr(&["biotech", "life sciences", "biotechnology", "laboratory", "pharma", "pharmaceutical", "genomics"], None,
        ["the sample under the microscope", "the pipeline chart", "the molecule"], "read the science or partner", "compound codes, trial phases and assay results", &["tech"]),
    tr(&["space tech", "aerospace", "satellite", "satellites", "rocket", "rockets", "launch vehicle", "orbital"], None,
        ["the vehicle on the pad", "the orbit diagram", "the mission patch"], "follow the mission or partner", "altitudes, payload masses and launch dates", &["tech", "industrial"]),
    tr(&["quantum", "quantum computing", "qubit", "qubits"], Some(S::DevTool),
        ["the circuit diagram", "the cryostat", "the measurement histogram"], "run a first circuit", "qubit counts, gate fidelities and coherence times", &["tech", "code"]),
    tr(&["drone", "drones", "uav", "fleet manager", "fleet management"], Some(S::ProductUi),
        ["the live map with each unit", "the unit's telemetry card", "the mission log"], "act on the unit that needs attention", "unit IDs, battery levels and ETAs", &["data", "tech"]),
    tr(&["climate tech", "clean energy", "renewable", "renewables", "sustainable energy", "wind farm", "wind energy"], None,
        ["the turbine or panel field", "the emissions curve", "the grid map"], "see the impact or partner", "megawatts, tonnes of CO2 and project dates", &["earth", "infrastructure"]),
    tr(&["habit tracker", "habit", "habits"], None,
        ["the streak calendar", "today's check-ins", "the progress ring"], "start a habit", "streak days, completion rates and reminders", &["play"]),
    tr(&["food delivery", "takeaway", "takeout", "meal delivery", "on-demand delivery", "grocery delivery"], Some(S::Commerce),
        ["the dish arriving at the door", "the courier on the map", "the menu card"], "order now", "delivery times, fees and dish prices", &["food"]),
    tr(&["ride hailing", "ride-hailing", "rideshare", "ride share", "taxi", "taxis", "cab", "cabs", "chauffeur"], None,
        ["the car arriving at the kerb", "the route on the map", "the driver card"], "book a ride", "pickup times, fares and car plates", &["infrastructure"]),
    tr(&["meditation", "mindfulness", "breathing", "breathwork"], None,
        ["the breathing guide", "the session list", "the quiet room"], "begin a session", "session lengths, streaks and times of day", &["care", "botanical"]),
    tr(&["crm", "client management", "customer management", "sales pipeline"], Some(S::ProductUi),
        ["the deal pipeline", "the client card with history", "the follow-up list"], "act on today's follow-ups", "deal values, stages and last-contact dates", &["data"]),
    tr(&["inventory", "stock management", "stock control", "stocktake", "stock levels"], Some(S::ProductUi),
        ["the shelf with counts", "the low-stock list", "the reorder sheet"], "reorder what is short", "SKUs, quantities and reorder points", &["data", "industrial"]),
    tr(&["flashcard", "flashcards", "study tool", "spaced repetition"], None,
        ["the card flipped to the answer", "the review queue", "the mastery chart"], "start studying", "cards due, recall rates and exam dates", &["play", "literary"]),
    tr(&["timer", "timers", "pomodoro", "stopwatch", "countdown"], None,
        ["the dial counting down", "the session log", "the break screen"], "start the timer", "minutes, sessions and breaks", &["data"]),
    tr(&["parenting", "au pair", "au pair agency", "nanny agency", "nannies", "baby", "babies", "newborn", "infant", "baby tracker"], None,
        ["the baby's day at a glance", "the feed and sleep log", "the growth chart"], "log the first feed", "feed times, sleep hours and weights", &["care", "play"]),
    tr(&["scanner", "document scanner", "document manager", "pdf scanner"], None,
        ["the page being scanned", "the cropped document", "the folder of scans"], "scan a document", "page counts, file sizes and dates", &["data"]),
    tr(&["password manager", "passwords", "passkey", "passkeys"], None,
        ["the vault locked", "the login entry", "the strength report"], "set up the vault", "entry counts, strength scores and breach alerts", &["tech", "code"]),
    tr(&["bill split", "bill splitting", "expense splitter", "split the bill", "shared expenses", "split expenses"], None,
        ["the receipt split into shares", "the who-owes-whom list", "the settle-up"], "split a bill", "amounts, shares and balances", &["finance"]),
    tr(&["voice recorder", "voice memo", "voice memos", "voice notes", "dictation", "transcription"], None,
        ["the waveform recording", "the transcript beside it", "the memo list"], "start recording", "durations, timestamps and word counts", &["data"]),
    tr(&["bookmark", "bookmarks", "read later", "read-later", "reading list", "save for later"], None,
        ["the saved article card", "the reading queue", "the highlights"], "save the first link", "read times, sources and save dates", &["literary"]),
    tr(&["calculator", "unit converter", "converter"], None,
        ["the keypad with a result", "the unit pair", "the history tape"], "convert or calculate", "units, values and decimal places", &["data"]),
    tr(&["alarm clock", "alarm app", "world clock", "time zones", "timezone"], None,
        ["the clock face", "the cities by hour", "the alarm list"], "set an alarm", "hours, time zones and wake times", &["data"]),
    tr(&["file manager", "file transfer", "file sharing", "send files", "cloud storage"], None,
        ["the folder tree", "the file in transit", "the share link"], "send a file", "file sizes, transfer speeds and expiry dates", &["data", "tech"]),
    tr(&["email client", "email app", "inbox", "mail app"], Some(S::ProductUi),
        ["the inbox at zero", "the thread open", "the compose window"], "connect an inbox", "senders, subjects and received times", &["data"]),
    tr(&["puzzle game", "puzzle", "puzzles", "match-3", "match three", "jigsaw"], Some(S::Showcase),
        ["the board mid-move", "the level map", "the solved moment"], "play the first level", "levels, moves and stars", &["play"]),
    tr(&["trivia", "quiz game", "pub quiz", "quiz night"], Some(S::Showcase),
        ["the question card with four answers", "the scoreboard", "the category wheel"], "play a round", "scores, rounds and categories", &["play"]),
    tr(&["board game", "board games", "card game", "card games", "tabletop", "chess", "poker"], Some(S::Showcase),
        ["the board mid-game", "the hand of cards", "the box and its pieces"], "play or order the game", "player counts, play times and ages", &["play"]),
    tr(&["idle game", "clicker", "clicker game", "incremental game", "tycoon"], Some(S::Showcase),
        ["the number climbing", "the upgrade shop", "the factory growing"], "start playing", "rates, upgrades and totals", &["play"]),
    tr(&["word game", "crossword", "crosswords", "wordle", "anagram", "scrabble"], Some(S::Showcase),
        ["the grid half solved", "the letter tiles", "today's puzzle number"], "play today's puzzle", "puzzle numbers, letters and streaks", &["play", "literary"]),
    tr(&["arcade", "retro game", "retro games", "platformer", "pixel game", "8-bit"], Some(S::Showcase),
        ["the sprite mid-jump", "the high-score table", "the cabinet"], "press start", "scores, levels and lives", &["play", "night"]),
    tr(&["photo editor", "photo editing", "photo filters", "lightroom presets"], None,
        ["the photo before and after", "the slider panel", "the filter strip"], "edit a photo", "exposure values, filter names and export sizes", &["ink"]),
    tr(&["video editor", "video editing", "short video", "short videos", "reels"], None,
        ["the timeline with clips", "the frame being trimmed", "the export preview"], "make a video", "durations, frame rates and aspect ratios", &["kinetic"]),
    tr(&["drawing app", "sketching", "sketchbook app", "painting app", "illustration app", "canvas app"], None,
        ["the stroke being drawn", "the brush palette", "the finished sketch"], "start drawing", "brush sizes, layers and canvas sizes", &["ink"]),
    tr(&["meme", "memes", "sticker", "stickers", "sticker maker", "gif maker"], None,
        ["the meme with its caption", "the sticker sheet", "the template picker"], "make one now", "templates, captions and share counts", &["play", "novelty"]),
    tr(&["ai avatar", "ai avatars", "avatar generator", "ai photo", "ai portrait", "ai portraits", "headshot generator"], None,
        ["the selfie and the generated portraits", "the style picker", "the result grid"], "upload a photo", "styles, image counts and turnaround minutes", &["tech"]),
    tr(&["wardrobe", "outfit", "outfits", "closet", "capsule wardrobe"], None,
        ["the outfit laid flat", "the closet grid", "the week of looks"], "add the first piece", "items, wears and seasons", &["textile"]),
    tr(&["plant care", "houseplant", "houseplants", "watering"], None,
        ["the plant on the sill", "the watering schedule", "the leaf close-up"], "add your first plant", "watering days, light levels and species names", &["botanical"]),
    tr(&["reading tracker", "book tracker", "reading log", "reading challenge", "goodreads"], None,
        ["the book on the shelf", "the reading progress", "the year in books"], "log a book", "pages, ratings and finish dates", &["literary"]),
    tr(&["family calendar", "chores", "chore chart", "family organiser", "family organizer"], None,
        ["the family week on the fridge", "the chore chart", "the shared list"], "set up the family", "names, days and times", &["play", "care"]),
    tr(&["mood tracker", "mood journal", "journaling"], None,
        ["the mood scale today", "the month in colours", "the journal note"], "log today's mood", "mood scores, dates and notes", &["care"]),
    tr(&["gift ideas", "gifting", "wishlist", "wish list", "gift registry"], None,
        ["the wrapped gift", "the wishlist", "the person it is for"], "start a wishlist", "prices, occasions and dates", &["celebration"]),
    tr(&["sleep tracker", "sleep tracking", "sleep app", "insomnia"], None,
        ["the night's sleep chart", "the bedtime routine", "the dark bedroom"], "start tonight", "hours slept, sleep stages and bedtimes", &["night", "care"]),
    tr(&["calorie", "calories", "nutrition", "macros", "meal plan", "meal planning", "diet"], None,
        ["the plate with its numbers", "the day's food log", "the macro ring"], "log the first meal", "calories, grams and portions", &["food", "data"]),
    tr(&["period tracker", "cycle tracker", "menstrual", "menstruation", "fertility", "ovulation"], None,
        ["the cycle calendar", "the symptom log", "the next-period forecast"], "start tracking", "cycle days, dates and symptoms", &["care"]),
    tr(&["medication", "medications", "pill reminder", "pills", "prescription", "prescriptions"], None,
        ["the pill box by day", "the dose reminder", "the refill note"], "add a medication", "doses, times and refill dates", &["care"]),
    tr(&["fasting", "intermittent fasting", "fast tracker"], None,
        ["the fasting clock", "the eating window", "the history of fasts"], "start a fast", "hours fasted, windows and streaks", &["care", "data"]),
    tr(&["anonymous", "confession", "confessions", "secret sharing"], None,
        ["the anonymous post card", "the replies", "the safety rules"], "post anonymously", "post times, reply counts and moderation rules", &["night"]),
    tr(&["local events", "whats on", "what's on", "event discovery", "events near"], None,
        ["the poster for tonight", "the map of nearby events", "the weekend list"], "find something tonight", "dates, venues and ticket prices", &["night", "celebration"]),
    tr(&["coding challenge", "coding challenges", "leetcode", "coding practice", "coding interview", "katas"], Some(S::DevTool),
        ["the problem statement", "the code editor with tests", "the leaderboard"], "solve the first problem", "difficulty, runtimes and pass rates", &["code"]),
    tr(&["parking", "car park", "parking finder", "parking space", "parking spaces"], None,
        ["the free bay on the map", "the street sign", "the timer for the bay"], "find and pay for a space", "spaces free, prices per hour and walk times", &["infrastructure"]),
    tr(&["public transit", "railway", "bus company", "coach operator", "ferry", "ferries", "public transport", "transit", "bus routes", "metro", "subway", "tram", "commute"], None,
        ["the line map", "the next departures board", "the stop"], "plan a journey", "line numbers, departure times and platforms", &["infrastructure", "civic"]),
    tr(&["road trip", "road trips", "campervan", "rv", "route planner"], None,
        ["the open road", "the route with stops", "the packed car"], "plan the route", "miles, drive times and stops", &["earth"]),
    tr(&["vpn", "privacy tool", "ad blocker", "encryption", "encrypted"], None,
        ["the connection switch on", "the server map", "the blocked-tracker count"], "connect securely", "server locations, speeds and protocols", &["tech", "night"]),
    tr(&["emergency sos", "sos", "personal safety", "safety app", "panic button"], None,
        ["the one-tap alert button", "the trusted contacts", "the shared location"], "set up emergency contacts", "contacts, locations and response times", &["civic"]),
    tr(&["white noise", "ambient sound", "ambient sounds", "soundscape", "soundscapes", "rain sounds"], None,
        ["the sound mixer", "the scene playing", "the sleep timer"], "press play", "sound names, volumes and timer minutes", &["night", "botanical"]),
    tr(&["home decor", "home decoration", "interior design", "interior designer", "furniture", "homeware", "homewares"], None,
        ["the styled room", "the swatch board", "the piece in place"], "shop the room or book a consult", "dimensions, materials and prices", &["textile", "modernist"]),
    tr(&["forum", "forums", "discussion board", "message board", "bulletin board"], Some(S::Editorial),
        ["the thread with replies", "the topic list", "the member badges"], "join the discussion", "reply counts, last-post times and member names", &["literary", "civic"]),
    tr(&["directory", "directories", "listing site", "listings site", "business directory"], None,
        ["the listing card", "the filter and map", "the category index"], "search the directory or get listed", "categories, locations and ratings", &["generic"]),
    tr(&["status page", "incident management", "uptime", "outage", "outages"], Some(S::ProductUi),
        ["the system status board", "the incident timeline", "the uptime bars"], "subscribe to updates", "uptime percentages, incident times and component names", &["data", "code"]),
    tr(&["wiki", "wikis", "encyclopedia", "encyclopaedia"], Some(S::Editorial),
        ["the article with its contents box", "the infobox", "the edit history"], "search or start reading", "article titles, citations and revision dates", &["literary"]),
    tr(&["changelog", "release notes", "what's new", "product updates"], Some(S::Editorial),
        ["the version entry", "the before and after of the change", "the release timeline"], "see what changed", "version numbers, dates and change types", &["code"]),
    tr(&["classifieds", "classified ads", "buy and sell", "buy-sell", "for sale"], Some(S::Commerce),
        ["the ad card with a price", "the photos of the item", "the seller contact"], "post an ad or browse", "prices, locations and posted dates", &["generic"]),
    tr(&["conference", "conferences", "symposium", "summit", "convention", "expo"], Some(S::Venue),
        ["the speaker on stage", "the programme by day", "the venue hall"], "get a ticket", "session times, speakers and ticket tiers", &["celebration"]),
    tr(&["e-signature", "esignature", "e-sign", "document signing", "sign documents"], Some(S::ProductUi),
        ["the document with the signature box", "the signing order", "the audit trail"], "send a document to sign", "signers, dates and document status", &["legal"]),
    tr(&["feature flag", "feature flags", "feature toggles", "remote config", "config management"], Some(S::DevTool),
        ["the flag toggle with its rollout", "the targeting rules", "the change log"], "create the first flag", "flag keys, rollout percentages and environments", &["code"]),
    tr(&["grant", "grants", "grant portal", "funding portal", "funding application", "funding applications", "bursary"], None,
        ["the open call with its deadline", "the application steps", "the funded projects"], "check eligibility and apply", "deadlines, award amounts and eligibility rules", &["civic"]),
    tr(&["lms", "learning management", "e-learning", "elearning", "online course", "online courses"], None,
        ["the lesson player", "the course outline with progress", "the certificate"], "start learning", "modules, durations and completion rates", &["literary"]),
    tr(&["open source", "open-source", "github project"], Some(S::DevTool),
        ["the install line", "the repo with its stars", "the contributors"], "star or install", "versions, licences and contributor counts", &["code"]),
    tr(&["patient portal", "health records", "medical records", "telehealth", "telemedicine"], Some(S::ProductUi),
        ["the appointment card", "the test result with its range", "the message to the clinician"], "sign in or book", "appointment times, result values and reference ranges", &["care"]),
    tr(&["patent", "patents", "intellectual property", "trademark", "trademarks"], None,
        ["the patent drawing", "the claims text", "the filing timeline"], "search the register", "filing numbers, dates and classes", &["legal", "literary"]),
    tr(&["testimonial widget", "review widget", "reviews widget", "social proof widget", "social proof"], Some(S::DevTool),
        ["the review card on a client's site", "the star summary", "the embed snippet"], "collect the first reviews", "ratings, review counts and dates", &["generic"]),
    tr(&["ticketing", "box office", "ticket sales"], Some(S::Commerce),
        ["the seat map", "the ticket in hand", "the event poster"], "pick seats and buy", "dates, seat rows and ticket prices", &["night", "celebration"]),
    tr(&["social media", "social network", "social networking", "social app"], None,
        ["the feed with a real post", "the profile", "the conversation under a post"], "join and follow", "handles, likes and post times", &["generic"]),
    tr(&["chatbot", "chatbots", "ai assistant", "ai assistants", "ai chat", "ai agent", "ai agents", "llm", "llms"], Some(S::ProductUi),
        ["the conversation with a real answer", "the sources it cited", "the task it finished"], "ask the first question", "response times, context sizes and usage limits", &["tech"]),
    tr(&["nft", "nfts", "web3", "crypto", "cryptocurrency", "blockchain", "defi"], None,
        ["the wallet with its holdings", "the on-chain record", "the token artwork"], "connect a wallet", "prices, supply figures and block heights", &["finance", "tech"]),
    tr(&["creator economy", "youtuber", "youtubers", "youtube", "streamer", "streamers", "twitch", "creators", "creator platform", "influencer", "influencers"], None,
        ["the creator at work", "the earnings card", "the fans' messages"], "start earning", "subscribers, payouts and post counts", &["play"]),
    tr(&["mental health", "psychologist", "psychologists", "psychotherapist", "psychotherapy", "psychiatrist", "counselling", "counseling", "wellbeing", "well-being"], None,
        ["the counsellor and the chair", "the check-in screen", "the first-session steps"], "book a first session", "session lengths, fees and availability", &["care"]),
    tr(&["podcast", "podcasts", "podcasting"], Some(S::Editorial),
        ["the episode player", "the hosts at the mic", "the episode list"], "listen to the latest episode", "episode numbers, runtimes and release dates", &["night", "literary"]),
    tr(&["dating", "matchmaking", "singles"], None,
        ["the profile card", "the first message", "the date itself"], "create a profile", "ages, distances and prompts", &["night", "celebration"]),
    tr(&["micro-credentials", "credentials", "badges", "certification", "certifications"], None,
        ["the badge earned", "the skills path", "the verification page"], "earn the first badge", "skill names, issue dates and verifier IDs", &["civic"]),
    tr(&["newsroom", "breaking news", "news site", "news platform", "news media", "news app", "newspaper", "media outlet"], Some(S::Editorial),
        ["the lead story with its photo", "the live updates", "the section front"], "read today's news", "headlines, bylines and timestamps", &["literary"]),
    tr(&["freelancer", "freelancers", "freelance", "gig work"], None,
        ["the freelancer's profile and work", "the brief posted", "the delivered job"], "post a job or find work", "day rates, ratings and delivery dates", &["generic"]),
    tr(&["language learning", "language app", "languages", "vocabulary", "language course"], None,
        ["the phrase with its translation", "the lesson streak", "the speaking exercise"], "start the first lesson", "levels, words learned and streak days", &["play", "literary"]),
    tr(&["cybersecurity", "cyber security", "security platform", "threat detection", "pentest", "penetration testing"], Some(S::ProductUi),
        ["the alert with its timeline", "the attack surface map", "the fix applied"], "book a demo or run a scan", "CVE IDs, severity scores and detection times", &["tech", "code"]),
    tr(&["biohacking", "longevity", "wearable", "wearables", "biomarker", "biomarkers"], None,
        ["the biomarker chart", "the wearable on the wrist", "the daily protocol"], "start tracking", "biomarker values, ranges and dates", &["tech", "care"]),
    tr(&["generative art", "creative coding", "ai art", "art generator"], Some(S::Showcase),
        ["the piece rendering live", "the seed and its parameters", "the gallery of outputs"], "make a piece", "seeds, parameters and edition numbers", &["ink", "kinetic"]),
    tr(&["spatial computing", "vr", "virtual reality", "augmented reality", "mixed reality", "xr", "headset", "headsets"], Some(S::Showcase),
        ["the room with floating windows", "the headset", "the hands in the scene"], "try it or download", "devices, frame rates and field of view", &["tech", "kinetic"]),
    tr(&["messaging", "messenger", "chat app", "group chat"], None,
        ["the conversation thread", "the group list", "the voice note"], "start a chat", "read receipts, member counts and timestamps", &["generic"]),
    tr(&["notes app", "note-taking", "note taking", "writing app", "notebook app"], None,
        ["the note mid-sentence", "the linked notes", "the folder list"], "write the first note", "word counts, tags and edit dates", &["literary"]),
    tr(&["recipe", "recipes", "cooking", "cookbook", "cookery"], None,
        ["the dish finished", "the ingredient list", "the step in the pan"], "cook the first recipe", "quantities, cook times and servings", &["food", "warm-grain"]),
    tr(&["weather", "forecast", "forecasts"], None,
        ["the sky right now", "the hourly forecast", "the radar map"], "check your forecast", "temperatures, rain chances and wind speeds", &["earth"]),
    tr(&["booking app", "appointment app", "appointment booking", "online booking", "booking system", "appointment scheduling"], Some(S::ProductUi),
        ["the open slots by day", "the booking confirmed", "the reminder"], "book a slot", "times, durations and staff names", &["data"]),
    tr(&["calendar", "calendars", "calendar app", "scheduling", "scheduling app"], Some(S::ProductUi),
        ["the week with events", "the meeting being placed", "the shared availability"], "connect a calendar", "times, time zones and attendees", &["data"]),
    tr(&["translator", "translators", "translation", "translate", "interpreter"], None,
        ["the sentence in two languages", "the language pair", "the spoken phrase"], "translate now", "language pairs, word counts and turnaround", &["literary"]),
    tr(&["couple", "couples", "relationship", "relationships"], None,
        ["the two of them", "the shared moments", "the question of the day"], "start together", "anniversaries, dates and shared lists", &["celebration", "care"]),
    tr(&["wallpaper", "wallpapers", "icon pack", "icon packs", "home screen"], None,
        ["the phone screen dressed", "the wallpaper grid", "the icon set"], "browse and apply", "resolutions, packs and download counts", &["play"]),
    tr(&["auction", "auctions", "auctioneer", "auctioneers", "bidding"], Some(S::Commerce),
        ["the lot under the hammer", "the current bid", "the catalogue"], "register to bid", "lot numbers, estimates and closing times", &["luxe"]),
    tr(&["citizen science", "biodiversity", "field observations", "wildlife survey"], None,
        ["the observation in the field", "the shared map of sightings", "the dataset"], "log a sighting", "species, locations and counts", &["earth", "civic"]),
    tr(&["crowdfunding", "crowdfund", "kickstarter", "backers", "pledge", "pledges"], None,
        ["the thing being funded", "the funding bar", "the reward tiers"], "back the project", "amounts raised, backers and days left", &["celebration"]),
    tr(&["review platform", "review site", "reviews platform", "consumer reviews"], None,
        ["the review with its stars", "the business page", "the reply from the owner"], "read or write a review", "ratings, review counts and dates", &["civic"]),
    tr(&["running club", "running app", "runners", "run club", "marathon", "trail running", "gps", "cycling gps", "running app"], None,
        ["the route on the map", "the split times", "the runner mid-stride"], "start a run", "distances, paces and elevation", &["sport", "earth"]),
    tr(&["hydration", "water reminder", "drink water"], None,
        ["the glass filling", "the day's intake", "the next reminder"], "set a daily goal", "millilitres, glasses and reminder times", &["care"]),
    tr(&["beat maker", "beatmaker", "beat making", "music production", "daw", "synth", "synths"], Some(S::Showcase),
        ["the beat grid", "the pads lit", "the mixer"], "make a beat", "BPMs, bars and tracks", &["night", "kinetic"]),
    tr(&["instrument", "luthier", "guitar maker", "violin maker", "instruments", "piano", "guitar", "violin", "music lessons", "drum lessons"], None,
        ["the hands on the instrument", "the lesson plan", "the first piece played"], "book a lesson", "grades, lesson lengths and fees", &["literary"]),
    tr(&["research lab", "research group", "research centre", "research center", "research institute"], Some(S::Editorial),
        ["the team at the bench", "the latest paper", "the data figure"], "read the research or join", "paper titles, grants and dates", &["tech", "literary"]),
    tr(&["link in bio", "link-in-bio", "bio link", "bio links"], None,
        ["the phone page with its links", "the creator's photo", "the click counts"], "claim your link", "link counts, clicks and handles", &["play"]),
    tr(&["no-code", "low-code", "no code", "low code", "website builder", "page builder", "app builder"], Some(S::ProductUi),
        ["the canvas being built", "the block library", "the published result"], "start building", "blocks, templates and publish dates", &["generic"]),
    tr(&["resume", "resumes", "cv", "cvs", "resume builder", "cv builder"], None,
        ["the finished CV page", "the section editor", "the job it won"], "build a CV", "roles, dates and skills", &["literary"]),
    tr(&["survey", "surveys", "form builder", "questionnaire", "questionnaires", "polls"], Some(S::ProductUi),
        ["the question being answered", "the results chart", "the form editor"], "build a form", "responses, completion rates and question types", &["data"]),
    tr(&["ai product", "ai products", "ai tool", "ai tools", "ai startup", "artificial intelligence", "machine learning"], None,
        ["the output on a real input", "the prompt and the result side by side", "the workflow it replaces"], "try it on your own input", "accuracy figures, speeds and plan limits", &["tech"]),
    tr(&["culture", "cultural", "arts centre", "arts center", "cultural centre", "cultural center", "arts council"], Some(S::Venue),
        ["the work on the wall or the stage", "the season programme", "the audience in the room"], "see what's on", "dates, venues and ticket prices", &["literary", "civic"]),
    tr(&["event", "events"], None,
        ["the room on the night", "the schedule", "the people who came"], "get a place", "dates, times and capacities", &["celebration", "generic"]),
    tr(&["physical product", "physical products", "hardware", "consumer product", "consumer products", "device", "devices", "product launch"], Some(S::Commerce),
        ["the object turned in the light", "the exploded view", "the spec sheet"], "pre-order or buy", "dimensions, materials, battery life and prices", &["industrial", "tech"]),
    // Everyday trades, civic services and industry that no catalogue category names.
    tr(&["campsite", "campsites", "campground", "glamping", "caravan park", "holiday park"], Some(S::Venue),
        ["the pitch at dusk", "the site map", "the fire and the tent"], "book a pitch", "pitch types, nights and prices", &["earth", "botanical", "warm"]),
    tr(&["marina", "boat rental", "boat hire", "yacht", "yachts", "yacht charter", "boatyard", "sailing"], Some(S::Venue),
        ["the boat on its mooring", "the berth map", "the day on the water"], "book a boat or a berth", "lengths, berths and day rates", &["marine", "light"]),
    tr(&["cruise", "cruises", "cruise line"], Some(S::Commerce),
        ["the ship at the quay", "the route by port", "the cabin"], "find a sailing", "ports, nights and cabin grades", &["marine", "luxe"]),
    tr(&["bowling", "escape room", "escape rooms", "laser tag", "paintball", "go kart", "go-kart", "karting", "trampoline park", "mini golf", "soft play", "ice rink", "skating", "archery", "amusement park", "theme park", "water park"], Some(S::Venue),
        ["the activity mid-play", "the session times", "the party package"], "book a session or a party", "session times, group sizes and prices", &["play", "bold", "kinetic"]),
    tr(&["zoo", "zoos", "aquarium", "aquariums", "planetarium", "wildlife park", "safari park"], Some(S::Venue),
        ["the animals or the sky up close", "the site map", "today's talks and feeding times"], "book tickets", "opening hours, talk times and ticket prices", &["botanical", "earth", "play"]),
    tr(&["casino", "casinos", "sports betting", "betting", "bookmaker", "lottery", "bingo", "fantasy sports"], None,
        ["the table or the board", "the odds", "the safer-play limits"], "join, with limits set", "odds, stakes and age rules", &["night", "luxe", "bold"]),
    tr(&["astrology", "tarot", "horoscope", "horoscopes", "psychic"], None,
        ["the chart drawn", "the reading", "the cards laid out"], "book a reading", "signs, dates and reading lengths", &["night", "ink"]),
    tr(&["life coach", "coaching", "executive coach", "business coach", "mentoring"], None,
        ["the coach and the client talking", "the programme stages", "the client's result in their words"], "book a discovery call", "session counts, lengths and fees", &["warm", "calm"]),
    tr(&["security company", "security guards", "private investigator", "home security", "cctv", "alarm system", "burglar alarm", "smart lock", "smart locks"], None,
        ["the camera or the guard on post", "the protected site plan", "the alert reaching the phone"], "get a security survey", "response times, licences and monthly fees", &["infrastructure", "night"]),
    tr(&["home inspector", "surveyor", "surveyors", "building survey"], None,
        ["the inspector on site", "the defect photo", "the report pages"], "book an inspection", "survey levels, findings and fees", &["trades"]),
    tr(&["it support", "managed it", "tech support", "helpdesk"], None,
        ["the fixed laptop", "the ticket queue", "the engineer at the desk"], "get support", "response times, devices and monthly fees", &["code", "data"]),
    tr(&["web hosting", "domain registrar", "data centre", "data center", "cloud provider", "cloud hosting", "vps"], Some(S::DevTool),
        ["the server rack", "the region map", "the deploy log"], "deploy or pick a plan", "regions, uptime and monthly prices", &["code", "data", "infrastructure"]),
    tr(&["telecom", "telecoms", "internet service provider", "isp", "broadband", "mobile network", "fibre", "fiber"], None,
        ["the coverage map", "the router and the speed test", "the plan card"], "check your address", "speeds, coverage and monthly prices", &["infrastructure", "data", "cool"]),
    tr(&["manufacturing", "manufacturer", "manufacturers", "factory", "factories", "steel", "foundry", "packaging", "chemical", "chemicals", "contract manufacturing"], Some(S::Showcase),
        ["the line running", "the part and its tolerance", "the plant floor"], "request a quote", "tolerances, capacities, certifications and lead times", &["industrial", "modernist"]),
    tr(&["mining", "quarry", "oil and gas", "drilling", "nuclear", "power plant", "refinery"], None,
        ["the site from above", "the safety record", "the output figures"], "read the operations or partner", "tonnages, safety days and output", &["industrial", "earth"]),
    tr(&["robotics", "semiconductor", "semiconductors", "3d printing", "additive manufacturing"], None,
        ["the machine at work", "the part in close-up", "the spec table"], "request a demo", "specs, cycle times and capacities", &["industrial", "data", "cool"]),
    tr(&["recycling", "waste management", "bin collection", "skip hire"], None,
        ["the truck on its round", "the sorting line", "the collection calendar"], "book a collection", "collection days, tonnages and prices", &["earth", "botanical", "civic"]),
    tr(&["snack", "snacks", "food brand", "granola", "condiments"], Some(S::Commerce),
        ["the pack and what is inside", "the ingredients", "the serving moment"], "find it or order", "flavours, weights and nutrition", &["food", "bold", "play"]),
    tr(&["supplement", "supplements", "vitamins", "protein powder"], Some(S::Commerce),
        ["the tub and the dose", "the ingredient label", "the test certificate"], "subscribe or buy", "doses, ingredients and test results", &["care", "bold"]),
    tr(&["stationery", "notebooks", "pens", "paper goods"], Some(S::Commerce),
        ["the page and the pen", "the paper weights", "the desk set"], "shop the range", "paper weights, sizes and prices", &["literary", "ink"]),
    tr(&["cannabis", "dispensary", "cbd", "hemp", "vape", "vapes"], Some(S::Commerce),
        ["the product and its lab sheet", "the strengths", "the age check"], "check age and browse", "strengths, lab results and age rules", &["botanical", "night"]),
    tr(&["cosmetics", "skincare", "makeup", "fragrance", "fragrances", "perfume", "perfumer", "beauty brand"], Some(S::Commerce),
        ["the product on skin", "the ingredient list", "the shade or scent range"], "find your shade or scent", "ingredients, shades and sizes", &["luxe", "light"]),
    tr(&["candle", "candles", "soap", "soaps", "leather goods", "leatherwork", "upholsterer", "upholstery", "tailor", "tailors", "tailoring", "alterations"], None,
        ["the piece in the maker's hands", "the material close-up", "the finished piece"], "order or commission", "materials, sizes and lead times", &["textile", "warm-grain"]),
    tr(&["shopping centre", "shopping center", "mall", "malls", "outlet mall"], Some(S::Venue),
        ["the stores by floor", "the opening hours", "the parking"], "plan a visit", "store names, floors and opening hours", &["bold", "light", "generic"]),
    tr(&["flea market", "farmers market", "craft fair", "crafts fair", "night market", "market stalls"], Some(S::Venue),
        ["the stalls in the morning", "the traders list", "the market dates"], "find the next market", "dates, stall numbers and times", &["food", "warm", "celebration"]),
    tr(&["consultancy", "consultancies", "consulting", "consultant", "consultants"], None,
        ["the client problem and the answer", "the method in steps", "the case result in figures"], "book a first conversation", "sectors, case figures and timeframes", &["finance", "calm", "generic"]),
    tr(&["engineering firm", "civil engineering", "structural engineer", "structural engineers", "engineers", "urban planning", "town planning"], None,
        ["the drawing and the built result", "the project list", "the site under work"], "discuss a project", "project values, spans and dates", &["industrial", "modernist"]),
    tr(&["hr", "human resources", "people ops", "environmental consultancy", "sustainability consultancy"], None,
        ["the team the work helps", "the process in steps", "the measured result"], "book a call", "headcounts, timelines and fees", &["calm", "warm", "generic"]),
    tr(&["video production", "film studio", "animation studio", "videographer", "videographers", "film production", "filmmaker", "filmmakers", "production company"], Some(S::Showcase),
        ["the frame from the latest film", "the crew on set", "the client list"], "watch the reel or book a shoot", "runtimes, formats and shoot days", &["kinetic", "night"]),
    tr(&["orchestra", "orchestras", "choir", "choirs", "symphony", "ensemble"], Some(S::Venue),
        ["the players on stage", "the season concerts", "the conductor"], "book concert seats", "dates, programmes and seat prices", &["literary", "celebration", "night"]),
    tr(&["radio", "radio station", "radio stations"], Some(S::Editorial),
        ["the live studio", "the schedule", "the on-air host"], "listen live", "frequencies, show times and hosts", &["night", "kinetic"]),
    tr(&["political campaign", "candidate", "political party"], None,
        ["the candidate with the people", "the pledges", "the polling-day plan"], "volunteer or donate", "pledges, dates and polling places", &["civic", "bold"]),
    tr(&["trade union", "trades union", "labour union", "labor union", "workers union"], None,
        ["the members together", "the wins in numbers", "the join form"], "join the union", "members, pay rates and dates", &["civic", "bold", "warm"]),
    tr(&["police", "fire department", "fire service", "fire brigade", "ambulance", "ambulance service", "coastguard"], None,
        ["the crew and the vehicle", "the report-it routes", "the safety advice"], "report or get help", "emergency numbers, response times and areas", &["civic", "infrastructure"]),
    tr(&["court", "courts", "court service", "courthouse", "crown court", "magistrates", "tribunal", "prison", "prisons", "probation"], None,
        ["the building and its entrance", "the case list", "the visitor rules"], "find your hearing or visit", "case numbers, dates and rooms", &["legal", "civic"]),
    tr(&["think tank", "policy institute", "policy research"], Some(S::Editorial),
        ["the latest report", "the chart that argues it", "the fellows"], "read the research", "report titles, figures and dates", &["literary", "editorial"]),
    tr(&["community centre", "community center", "youth club", "youth centre", "youth center", "village hall", "scouts"], Some(S::Venue),
        ["the hall in use", "the week's sessions", "the people who run it"], "join a session or hire the hall", "session times, ages and hire rates", &["civic", "warm", "play"]),
    tr(&["animal shelter", "animal rescue", "dog rescue", "cat rescue", "rehoming"], None,
        ["the animal waiting for a home", "the adoption steps", "the volunteers"], "adopt or donate", "ages, breeds and adoption fees", &["warm", "care", "botanical"]),
    tr(&["homeless shelter", "homelessness", "addiction recovery", "rehab", "rehab centre", "drug rehab", "sobriety", "helpline", "crisis line", "suicide prevention"], None,
        ["the person helped and the helper", "the first step to call", "the services by need"], "get help now", "phone numbers, opening hours and services", &["warm", "care", "calm"]),
    tr(&["hospital", "hospitals", "urgent care", "a&e", "emergency department", "nhs trust"], None,
        ["the ward and the people in it", "the wait times", "the departments by floor"], "find a service or wait time", "departments, wait times and visiting hours", &["care", "calm", "cool"]),
    tr(&["home nursing", "nurse", "nurses", "nursing", "midwife", "midwives", "doula", "doulas", "maternity", "pregnancy", "prenatal"], None,
        ["the carer with the family", "the care plan by week", "the first visit"], "arrange a visit", "visit times, weeks and fees", &["care", "warm", "calm"]),
    tr(&["dna test", "dna testing", "genealogy", "family history", "ancestry", "blood test", "lab testing", "diagnostics"], None,
        ["the kit and the sample", "the result chart", "the family tree"], "order a test", "markers, ranges and turnaround days", &["data", "care"]),
    tr(&["history society", "local history", "historical society"], Some(S::Editorial),
        ["the old photograph", "the catalogue entry", "the talk programme"], "search the archive or join", "dates, catalogue numbers and places", &["memorial", "literary"]),
    tr(&["plastic surgeon", "plastic surgery", "cosmetic surgery", "aesthetic clinic", "aesthetics", "botox"], None,
        ["the surgeon in consultation", "the procedure steps", "the recovery timeline"], "book a consultation", "procedures, recovery weeks and fees", &["luxe", "care"]),
    tr(&["gas station", "petrol station", "service station"], None,
        ["the forecourt at night", "the fuel prices", "the shop and coffee"], "find the nearest station", "fuel prices, opening hours and services", &["infrastructure", "bold"]),
    tr(&["catering", "caterer", "caterers"], None,
        ["the spread laid out", "the menus by event", "the team serving"], "get a catering quote", "headcounts, menu prices and dates", &["food", "celebration"]),
    tr(&["scooter", "scooters", "e-scooter", "e-scooters", "bike share", "micromobility", "car sharing", "car share"], None,
        ["the vehicle on the street", "the map of nearby vehicles", "the unlock screen"], "unlock a ride", "minutes, prices per minute and zones", &["kinetic", "bold"]),
    tr(&["toll road", "motorway", "highways", "bridge operator"], None,
        ["the road at dawn", "the toll and payment", "the traffic map"], "pay a toll or check traffic", "prices, routes and closures", &["infrastructure"]),
    tr(&["air purifier", "smart home", "home appliance", "home appliances"], Some(S::Commerce),
        ["the device in the room", "the before and after reading", "the spec card"], "buy or compare", "readings, room sizes and prices", &["cool", "light"]),
];

/// What the brief is about, in its own words.
#[derive(Debug, Clone)]
pub(crate) struct Subject {
    /// The subject phrase ("seafood restaurant", "kids' museum").
    pub phrase: String,
    pub objects: Vec<String>,
    pub action: String,
    pub notation: String,
    /// True when objects came from a known trade; false when only the phrase is known.
    pub from_trades: bool,
    /// Subject tags of the matched trades (memorial, ink, trades...), on top of the brief's niche tags.
    pub tags: Vec<&'static str>,
    /// The brief's own content nouns, for the rationale when no trade matched.
    pub nouns: Vec<String>,
}

const FILLER: &[&str] = &[
    "a", "an", "the", "our", "my", "their", "new", "website", "site", "homepage", "home", "landing", "marketing", "web", "online",
    "build", "make", "create", "design", "redesign", "page", "please",
];
const BREAKS: &[&str] = &[
    "for", "on", "with", "in", "that", "which", "to", "and", "near", "from", "where", "who", "selling", "offering", "using", "by", "at",
    "of", "about", "like",
];

/// Words ending in "-ing" that name a trade or thing rather than open a clause.
const ING_NOUNS: &[&str] = &[
    "wedding", "building", "clothing", "catering", "planning", "printing", "roofing", "flooring", "plumbing",
    "coding", "banking", "accounting", "marketing", "consulting", "engineering", "housing", "landscaping",
    "cleaning", "training", "parking", "lighting", "packaging", "shipping", "hosting", "recruiting", "tutoring",
    "dining", "gaming", "ticketing", "invoicing", "publishing", "painting", "piercing", "boxing", "climbing",
    "sailing", "fishing", "farming", "mining", "manufacturing", "nursing", "dancing", "learning", "wellbeing",
    "staffing", "coaching", "branding", "bowling", "skating", "surfing", "diving", "cycling", "running",
    "swimming", "riding", "camping", "glamping", "brewing", "baking", "roasting", "tailoring", "framing",
    "welding", "fencing", "decking", "tiling", "glazing", "heating", "cooling", "computing", "billing",
    "booking", "lending", "trading", "investing", "funding", "advertising", "networking", "streaming",
    "messaging", "monitoring", "testing", "logging", "mapping", "tracking", "scheduling", "routing", "storing",
    "spring", "evening", "morning", "king", "ring", "thing", "something", "nothing", "everything",
];

fn subject_phrase(query: &str) -> String {
    let first_line = query.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    // Prompt wrappers are metadata, not the client's identity. Preserve ordinary
    // colon-bearing brand names; only unwrap explicit brief/prompt labels.
    let first_line = first_line.split_once(':').and_then(|(label, value)| {
        let label = label.trim().trim_start_matches('#').trim().to_ascii_lowercase();
        ["master ai prompt", "ai prompt", "prompt", "design brief", "brief"].contains(&label.as_str()).then_some(value)
    }).unwrap_or(first_line);
    let sentence = first_line.split(['.', ';', ':', '!', '?', ',', '(', '\n']).next().unwrap_or("");
    let words: Vec<String> = sentence
        .split_whitespace()
        .map(|w| w.chars().filter(|c| c.is_alphanumeric() || *c == '\'' || *c == '-' || *c == '\u{2019}').collect::<String>())
        .filter(|w| !w.is_empty())
        .collect();
    let mut segments: Vec<Vec<String>> = vec![Vec::new()];
    for (wi, w) in words.iter().enumerate() {
        let lw = w.to_lowercase();
        // A participle after the subject opens a clause ("funeral home helping families ..."):
        // the subject ends there, so its head noun is kept.
        let participle = lw.len() > 4
            && lw.ends_with("ing")
            && !ING_NOUNS.contains(&lw.as_str())
            && segments.last().is_some_and(|s| !s.is_empty());
        let colour_pair = lw == "and" && wi > 0 && words[wi - 1].eq_ignore_ascii_case("black") && words.get(wi + 1).is_some_and(|s| s.eq_ignore_ascii_case("white"));
        // "web design and development agency": a coordinating "and" joins modifiers of one head noun,
        // so the subject keeps running instead of stopping at the first modifier.
        if lw == "and" && !colour_pair && segments.last().is_some_and(|s| !s.is_empty()) {
            continue;
        }
        if (BREAKS.contains(&lw.as_str()) && !colour_pair) || participle {
            segments.push(Vec::new());
        } else if let Some(last) = segments.last_mut() {
            last.push(w.clone());
        }
    }
    let content = |seg: &Vec<String>| -> Vec<String> {
        // "home" is filler ("home page") except as the head noun of a trade ("funeral home", "care home").
        let last = seg.len().saturating_sub(1);
        // "web", "design" and "marketing" are the trade itself in front of a firm ("web design agency").
        let firm = seg.last().is_some_and(|w| ["agency", "studio", "studios", "consultancy", "firm", "collective"].contains(&w.to_lowercase().as_str()));
        seg.iter()
            .enumerate()
            .filter(|(i, w)| {
                let l = w.to_lowercase();
                !FILLER.contains(&l.as_str())
                    || (l == "home" && *i == last && *i > 0)
                    || (firm && ["web", "design", "marketing", "online"].contains(&l.as_str()))
            })
            .map(|(_, w)| w.to_lowercase())
            .collect()
    };
    let chosen = segments.iter().map(content).find(|c| !c.is_empty()).unwrap_or_default();
    let n = chosen.len();
    let phrase = chosen.into_iter().skip(n.saturating_sub(4)).collect::<Vec<_>>().join(" ");
    if phrase.is_empty() { "this business".into() } else { phrase }
}

/// The brief's content words (no filler, no joining words), in order, at most five.
fn brief_nouns(query: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for t in tokenize(query) {
        if t.len() > 2 && !FILLER.contains(&t.as_str()) && !BREAKS.contains(&t.as_str()) && !out.contains(&t) && t.chars().all(|c| c.is_alphabetic() || c == '-') {
            out.push(t);
        }
        if out.len() == 5 {
            break;
        }
    }
    out
}

pub(crate) fn read_subject(query: &str, signals: &BriefSignals) -> Subject {
    // A labelled trade line ("Industry: ...") names the subject better than a long spec's first line.
    let mut subject = crate::intelligence::focus_line(query)
        .map(|focus| read_subject_in(&focus, signals))
        .filter(|s| s.from_trades)
        .unwrap_or_else(|| read_subject_in(query, signals));
    // Structured client context must shape the compositions, not only the summary.
    // Keep three real objects for the templates without inventing extra client material.
    for line in query.lines() {
        if let Some(action) = line.strip_prefix("Primary action:").map(str::trim).filter(|s| !s.is_empty()) {
            subject.action = action.to_string();
        }
        if let Some(objects) = line.strip_prefix("Subject objects:") {
            let mut objects: Vec<String> = objects.split(',').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect();
            if !objects.is_empty() {
                while objects.len() < 3 { objects.push(objects.last().unwrap().clone()); }
                subject.objects = objects;
            }
        }
    }
    subject
}

fn read_subject_in(query: &str, signals: &BriefSignals) -> Subject {
    let tokens = tokenize(query);
    let pos = |w: &str| -> Option<usize> {
        let parts = tokenize(w);
        if parts.len() == 1 {
            tokens.iter().position(|t| *t == parts[0] || (parts[0].len() > 3 && *t == format!("{}s", parts[0])))
        } else if !parts.is_empty() {
            tokens.windows(parts.len()).position(|slice| slice == parts)
        } else {
            None
        }
    };
    let mut hits: Vec<(bool, usize, &Trade)> = TRADES
        .iter()
        .filter_map(|t| {
            let off = matches!(signals.surface, S::ProductUi | S::DevTool) && t.surface != Some(signals.surface);
            t.words.iter().filter_map(|w| pos(w)).min().map(|p| (off, p, t))
        })
        .collect();
    // On a task surface the matching trade leads ("bakery sales dashboard" is a dashboard first);
    // otherwise the brief's word order does ("kids' museum" is about kids before museums).
    hits.sort_by_key(|(off_surface, p, _)| (*off_surface, *p));
    let phrase = subject_phrase(query);
    let nouns = brief_nouns(query);
    if hits.is_empty() {
        let head = phrase.split_whitespace().last().unwrap_or("business").to_string();
        let action = match signals.surface {
            S::Commerce => "add to bag",
            S::Venue => "book or plan a visit",
            S::ProductUi => "act on today's exceptions",
            S::DevTool => "copy the install command",
            S::Showcase => "open the work",
            S::Editorial => "start reading",
            S::General => "make first contact (call, book or ask)",
        };
        return Subject {
            objects: vec![format!("the {head} itself"), "the people it serves".into(), "the result it delivers".into()],
            action: action.into(),
            notation: "the trade's own names, numbers and dates".into(),
            phrase,
            from_trades: false,
            tags: Vec::new(),
            nouns,
        };
    }
    // The action comes from the head of the subject phrase: the last specific trade word inside it
    // ("vineyard wedding venue" books a viewing; "tattoo studio" books an artist, not a project).
    let phrase_end = phrase.split_whitespace().last().and_then(|w| tokenize(w).into_iter().last()).and_then(|w| tokens.iter().position(|t| *t == w));
    let lead_off = hits[0].0;
    let head = hits
        .iter()
        .filter(|(off, p, t)| *off == lead_off && !t.tags.contains(&"generic") && phrase_end.is_some_and(|e| *p <= e))
        .max_by_key(|(_, p, _)| *p)
        .map(|(_, _, t)| *t)
        .or_else(|| hits.iter().find(|(off, _, t)| *off == lead_off && !t.tags.contains(&"generic")).map(|(_, _, t)| *t))
        .unwrap_or(hits[0].2);
    // A recognised multiword trade is more specific than its generic head noun:
    // photography gallery must not inherit a museum's plinths and ticket booking.
    // Compared as tokens, so case and hyphens do not hide it ("Link-in-Bio Page Builder").
    let phrase_tokens = tokenize(&phrase);
    let head = hits.iter().filter(|(off, _, _)| *off == lead_off)
        .flat_map(|(_, _, t)| t.words.iter().filter_map(|w| {
            let parts = tokenize(w);
            let n = parts.len();
            (n > 1 && phrase_tokens.windows(n).any(|s| s == parts.as_slice())).then_some((n, *t))
        }))
        .max_by_key(|(n, _)| *n).map(|(_, t)| t).unwrap_or(head);
    // One subject owns its objects and notation. Build techniques and incidental nouns must not
    // splice a second trade's world into the client's content.
    let objects = head.objects.iter().map(|s| s.to_string()).collect();
    let notation = head.notation.to_string();
    // A repair modifier keeps the trade's objects but takes the repair action
    // ("watch repair atelier" books a repair, not a viewing).
    let repairs = phrase.split_whitespace().any(|w| matches!(w.to_lowercase().as_str(), "repair" | "repairs"));
    let action = match hits.iter().map(|(_, _, t)| *t).find(|t| t.words.contains(&"repair")) {
        Some(r) if repairs && !std::ptr::eq(r, head) => r.action,
        _ => head.action,
    };
    // The head trade's tags lead (a "municipal water utility" is infrastructure first).
    let mut tags: Vec<&'static str> = head.tags.iter().copied().filter(|t| *t != "generic").collect();
    for (_, _, t) in hits.iter().take(3) {
        // A generic modifier ("repair" in "watch repair atelier") lends no tags to another head.
        if t.tags.contains(&"generic") && !std::ptr::eq(*t, head) {
            continue;
        }
        for tag in t.tags {
            if *tag != "generic" && !tags.contains(tag) {
                tags.push(tag);
            }
        }
    }
    Subject { phrase, objects, action: action.to_string(), notation, from_trades: true, tags, nouns }
}

/// One direction as dealt: its structure, type and palette, and the options it beat.
pub(crate) struct Dealt {
    pub arch: &'static Archetype,
    /// 0 = A, 1 = B, 2 = C.
    pub slot: u8,
    pub typography: TypographyPairing,
    pub palette: PaletteRecommendation,
    pub tone: Tone,
    pub type_tags: Vec<&'static str>,
    pub palette_tags: Vec<&'static str>,
    /// Other archetypes that fit this slot for this brief.
    pub alternatives: Vec<&'static str>,
    /// True when type and palette are the matched industry profile's own.
    pub from_industry: bool,
    /// Corner-radius language key (sharp-0, soft-12, ticket-notch...).
    pub radius: &'static str,
    /// Writing system the brief's place or language asked for, when the pairing covers it.
    pub script: Option<&'static str>,
    /// Pool facts for the rationale: overused flag, licence, fallbacks.
    pub type_entry: Option<&'static TypeEntry>,
    /// Brand notes: the brief's own colours or fonts this direction is built around.
    pub brand: Vec<String>,
    /// Second stylesheet for faces the main URL does not carry (a Google mono beside Fontshare faces).
    pub extra_font_url: Option<String>,
}

fn fits(a: &Archetype, s: Surface) -> bool {
    a.surfaces.is_empty() || a.surfaces.contains(&s)
}

/// Tags that name a trade's subject matter (not a mood). A pick carrying only other trades'
/// subject tags reads as borrowed: a marine palette or a rounded kids' face on a law firm.
const SUBJECT_TAGS: &[&str] = &[
    "marine", "play", "legal", "botanical", "textile", "care", "luxe", "warm-grain", "earth", "civic", "industrial", "memorial", "ink", "food",
    "celebration", "infrastructure", "trades", "literary", "finance", "sport",
];

fn hsl(hex: &str) -> Option<(f64, f64, f64)> {
    let (r, g, b) = rgb(hex)?;
    let (r, g, b) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let l = (max + min) / 2.0;
    let d = max - min;
    if d == 0.0 {
        return Some((0.0, 0.0, l));
    }
    let s = d / (1.0 - (2.0 * l - 1.0).abs());
    let hue = if max == r { 60.0 * (((g - b) / d) % 6.0) } else if max == g { 60.0 * ((b - r) / d + 2.0) } else { 60.0 * ((r - g) / d + 4.0) };
    Some(((hue + 360.0) % 360.0, s, l))
}

/// Dark blue ground with a gold/ochre/brass accent: the "trust" palette law and finance default to.
pub(crate) fn navy_gold(p: &PaletteRecommendation) -> bool {
    let navy = hsl(&p.background).is_some_and(|(h, s, l)| (200.0..=250.0).contains(&h) && s > 0.2 && l < 0.35);
    let gold = |hex: &str| hsl(hex).is_some_and(|(h, s, l)| (35.0..=55.0).contains(&h) && s > 0.45 && (0.35..=0.8).contains(&l));
    navy && (gold(&p.primary_accent) || gold(&p.secondary_accent))
}

/// A near-black ground with little hue of its own and one hot, saturated accent: the stock
/// "dark + neon" look the avoid list bans. A dark ground with its own colour (deep teal, oxblood) does not count.
pub(crate) fn neon_on_black(p: &PaletteRecommendation) -> bool {
    // Near-black or a dusky, barely tinted dark (a vaporwave purple, a console grey-blue); a
    // saturated deep teal or navy is a coloured ground, not black.
    let black = hsl(&p.background).is_some_and(|(_, s, l)| l < 0.2 && (s < 0.45 || l < 0.06));
    // A luminous saturated accent outside the warm reds, oranges and golds (gold on black is the
    // luxury classic, not neon); terminal greens and mints count.
    let hot = hsl(&p.primary_accent).is_some_and(|(h, s, l)| s > 0.45 && (0.4..=0.8).contains(&l) && (55.0..350.0).contains(&h));
    black && hot
}

/// The first subject tag names what a pick was made for, so a pick led by another trade's subject
/// (a court-paper palette that also suits civic buildings) still reads as borrowed.
fn foreign(tags: &[&str], has: &dyn Fn(&str) -> bool) -> bool {
    let lead = tags.iter().find(|t| SUBJECT_TAGS.contains(t));
    // A surface tag the brief wants (data for a dashboard, code for a dev tool) outranks the lead.
    let surface = tags.iter().any(|t| matches!(*t, "data" | "code") && has(t));
    lead.is_some_and(|t| !has(t)) && !surface
}

fn rotate(variant: u64, n: usize) -> usize {
    if variant == 0 || n == 0 {
        0
    } else {
        ((variant - 1) % n as u64) as usize
    }
}

/// Near-best window: every candidate within `margin` of the best, at least `min` when available.
fn window<T: Copy>(ranked: &[(i32, T)], margin: i32, min: usize) -> Vec<T> {
    let top = ranked.first().map(|(s, _)| *s).unwrap_or(0);
    ranked.iter().enumerate().filter(|(i, (s, _))| *s + margin >= top || *i < min).map(|(_, (_, t))| *t).collect()
}

/// Writing system implied by a place or language named in the brief.
fn brief_script(query: &str) -> Option<&'static str> {
    const MAP: &[(&str, &[&str])] = &[
        ("japanese", &["japan", "japanese", "tokyo", "osaka", "kyoto", "sapporo", "fukuoka", "nagoya", "yokohama", "kobe", "nara"]),
        ("korean", &["korea", "korean", "seoul", "busan", "incheon"]),
        ("chinese-simplified", &["china", "chinese", "mandarin", "beijing", "shanghai", "shenzhen", "guangzhou", "chengdu"]),
        ("arabic", &["arabic", "dubai", "abu dhabi", "cairo", "riyadh", "jeddah", "doha", "beirut", "amman", "marrakech", "casablanca", "muscat"]),
        ("hebrew", &["hebrew", "israel", "tel aviv", "jerusalem", "haifa"]),
        ("devanagari", &["hindi", "marathi", "nepali", "delhi", "mumbai", "jaipur", "pune", "varanasi", "kathmandu"]),
        ("thai", &["thai", "thailand", "bangkok", "chiang mai", "phuket"]),
        ("greek", &["greek", "greece", "athens", "thessaloniki", "crete"]),
        ("cyrillic", &["russian", "ukrainian", "bulgarian", "serbian", "kazakh", "moscow", "kyiv", "sofia", "belgrade", "almaty", "odesa"]),
    ];
    let tokens = tokenize(query);
    MAP.iter().find(|(_, ws)| ws.iter().any(|w| keyword_hit(&tokens, w))).map(|(s, _)| *s)
}

const NON_LATIN: &[&str] = &["japanese", "korean", "chinese-simplified", "arabic", "hebrew", "devanagari", "thai"];

/// Tags whose loud register clashes with a solemn subject (funerals, care, law).
const LOUD: &[&str] = &["kinetic", "play", "y2k", "bold", "celebration", "70s"];

fn mix(a: (u8, u8, u8), b: (u8, u8, u8), t: f64) -> String {
    let m = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t).round().clamp(0.0, 255.0) as u8;
    format!("#{:02X}{:02X}{:02X}", m(a.0, b.0), m(a.1, b.1), m(a.2, b.2))
}

fn contrast(a: &str, b: &str) -> f64 {
    wcag_contrast(a, b).unwrap_or(0.0)
}

/// The brief's colours that can be a page ground (near-black or near-white), first one wins.
fn brand_ground(colors: &[String]) -> Option<&String> {
    colors.iter().find(|c| luminance(c).is_some_and(|l| !(0.03..=0.7).contains(&l)))
}

/// Builds the palette around the brief's own colours on top of a dealt base palette: a near-black
/// or near-white brief colour becomes the ground, the rest become accents; text and muted text
/// are re-picked so both still clear AA on background and surface.
fn brand_palette(colors: &[String], roles: &std::collections::BTreeMap<String, String>, base: &PaletteRecommendation) -> PaletteRecommendation {
    let ground = brand_ground(colors);
    let free: Vec<&String> = colors.iter().filter(|c| Some(*c) != ground && !roles.iter().any(|(k, r)| k != "primary" && r == *c)).collect();
    // Near-white and near-black brief colours are surfaces and deep tones, not accents, while a
    // mid-tone colour exists to carry the accent.
    let mid = |c: &&String| luminance(c).is_some_and(|l| (0.03..=0.7).contains(&l));
    let (mut accents, deep): (Vec<&String>, Vec<&String>) = if free.iter().any(mid) { free.iter().partition(|c| mid(c)) } else { (free.clone(), Vec::new()) };
    if let Some(i) = roles.get("primary").and_then(|p| accents.iter().position(|c| *c == p)) {
        let p = accents.remove(i);
        accents.insert(0, p);
    }
    let tints: Vec<&String> = accents.iter().skip(2).copied().collect();
    let (bg, surface) = match ground.and_then(|g| rgb(g).map(|c| (g, c))) {
        Some((g, c)) => {
            let toward = if luminance(g).unwrap_or(0.0) < 0.5 { (255, 255, 255) } else { (0, 0, 0) };
            // A tinted brief colour on the same side of mid-grey as the ground is the surface.
            let light = luminance(g).unwrap_or(0.0) >= 0.5;
            let neutral = |h: &str| rgb(h).is_some_and(|(r, gg, b)| r == gg && gg == b);
            let tint = colors.iter().filter(|h| *h != g && !neutral(h) && luminance(h).is_some_and(|l| if light { l > 0.75 } else { l < 0.02 })).min_by(|a, b| {
                let d = |h: &String| (luminance(h).unwrap_or(0.0) - luminance(g).unwrap_or(0.0)).abs();
                d(a).total_cmp(&d(b))
            });
            (g.clone(), tint.cloned().unwrap_or_else(|| mix(c, toward, 0.06)))
        }
        None => (base.background.clone(), base.surface.clone()),
    };
    let ok = |c: &str| contrast(c, &bg) >= 4.5 && contrast(c, &surface) >= 4.5;
    let fg = if let Some(t) = roles.get("text").filter(|t| ok(t)) {
        t.clone()
    } else if ok(&base.foreground) {
        base.foreground.clone()
    } else if contrast("#FFFFFF", &bg) >= contrast("#111111", &bg) {
        "#FFFFFF".to_string()
    } else {
        "#111111".to_string()
    };
    let nudge = |c: &str| rgb(c).and_then(|c| [0.0, 0.1, 0.2, 0.3, 0.4, 0.5].iter().map(|t| mix(c, rgb(&fg).unwrap_or((0, 0, 0)), *t)).find(|m| ok(m)));
    let muted = if let Some(m) = roles.get("muted").and_then(|m| nudge(m)) {
        // The brief's own secondary text, darkened toward the ink only as far as contrast needs.
        m
    } else if ok(&base.muted) {
        base.muted.clone()
    } else {
        let (f, b) = (rgb(&fg).unwrap_or((0, 0, 0)), rgb(&bg).unwrap_or((255, 255, 255)));
        [0.35, 0.3, 0.25, 0.2, 0.15, 0.1, 0.05, 0.0].iter().map(|t| mix(f, b, *t)).find(|c| ok(c)).unwrap_or_else(|| fg.clone())
    };
    let monochrome = colors.len() >= 2 && colors.iter().all(|c| rgb(c).is_some_and(|(r,g,b)| r == g && g == b));
    let primary = roles.get("primary").filter(|c| Some(*c) != ground).cloned().or_else(|| accents.first().map(|c| (*c).clone())).unwrap_or_else(|| if monochrome { fg.clone() } else { base.primary_accent.clone() });
    let secondary = accents.get(1).map(|c| (*c).clone()).unwrap_or_else(|| if monochrome { primary.clone() } else { base.secondary_accent.clone() });
    let hexes: Vec<String> = colors.iter().map(|c| c.trim_start_matches('#').to_lowercase()).collect();
    let notes = format!(
        "Brief colours {} lead: {}; {} | base {}",
        colors.join(", "),
        match ground {
            Some(g) => format!("{g} is the ground"),
            None => format!("ground kept from {}", base.id),
        },
        {
            let mut a = if accents.is_empty() {
                "accents kept from the base".to_string()
            } else {
                format!("{} as accents", accents.iter().take(2).map(|c| c.as_str()).collect::<Vec<_>>().join(" and "))
            };
            if !tints.is_empty() {
                a += &format!("; {} only as sparing light tints", tints.iter().map(|c| c.as_str()).collect::<Vec<_>>().join(" and "));
            }
            if let Some(b) = roles.get("band") {
                a += &format!("; dark chapters on {b} with white text, alternating with the light ground");
            }
            if !deep.is_empty() {
                a += &format!("; {} as surfaces and deep tones, never accents", deep.iter().map(|c| c.as_str()).collect::<Vec<_>>().join(" and "));
            }
            a
        },
        base.notes.split(" | ").next().unwrap_or("")
    );
    let mut p = palette(&format!("brand-{}-on-{}", hexes.join("-"), base.id), &bg, &surface, &fg, &muted, &primary, &secondary, 0.0, &notes);
    verify_palette(&mut p);
    p
}

fn is_mono_name(f: &str) -> bool {
    let l = f.to_lowercase();
    l.contains("mono") || l.contains("code") || l.ends_with(" console")
}

/// Words that give a brief a place to map.
const PLACE_WORDS: &[&str] = &[
    "in", "near", "local", "map", "maps", "location", "locations", "city", "town", "village", "neighbourhood", "neighborhood", "district",
    "region", "route", "routes", "trail", "trails", "venue", "visit", "tour", "tours", "campus", "park", "garden", "gardens", "estate", "branches", "camp", "camps", "school", "schools",
];

/// Faces drawn for running text. They never set display type, unless the pairing is one family
/// with a display optical size (see `OPSZ_FACES`).
const TEXT_ONLY_FACES: &[&str] = &[
    "libre caslon text", "markazi text", "crimson pro", "crimson text", "pt serif", "pt sans", "gelasio", "lora", "atkinson hyperlegible",
    "karla", "public sans", "source sans 3", "work sans", "ibm plex sans", "inter", "open sans", "roboto", "noto sans", "noto serif",
    "libre baskerville", "nunito sans", "figtree", "dm sans",
];

/// Families with an optical-size axis that reaches display sizes.
const OPSZ_FACES: &[&str] = &["literata", "source serif 4", "newsreader", "fraunces", "roboto serif", "piazzolla", "merriweather"];

pub(crate) fn display_capable(display: &str, body: &str) -> bool {
    let d = display.to_lowercase();
    !TEXT_ONLY_FACES.contains(&d.as_str()) || (family(display) == family(body) && OPSZ_FACES.contains(&d.as_str()))
}

thread_local! {
    /// Display and body families used in this project's recent runs (the look ledger), set by
    /// [`with_recent_faces`] for the duration of one deal.
    static RECENT_FACES: std::cell::RefCell<(Vec<String>, Vec<String>)> = const { std::cell::RefCell::new((Vec::new(), Vec::new())) };
}

/// Runs `f` with the ledger's recent display and body families steering the type deal: recent
/// display families are never dealt again (unless the brief names them), recent bodies are
/// down-ranked like a reuse within the brief.
pub(crate) fn with_recent_faces<T>(display: Vec<String>, body: Vec<String>, f: impl FnOnce() -> T) -> T {
    RECENT_FACES.with(|r| *r.borrow_mut() = (display, body));
    let out = f();
    RECENT_FACES.with(|r| *r.borrow_mut() = (Vec::new(), Vec::new()));
    out
}

thread_local! {
    /// Archetype and palette picks from the look ledger, newest first, set by [`with_recent_picks`].
    static RECENT_PICKS: std::cell::RefCell<(Vec<RecentPick>, Vec<RecentPick>)> = const { std::cell::RefCell::new((Vec::new(), Vec::new())) };
}

/// Runs `f` with the ledger's recent archetypes and palettes (newest first) down-ranking those
/// picks, so the same brief asked again is dealt a different structure and colour. Niche fit
/// still gates the pool: a recent pick is only out-scored by the next-best fitting one.
pub(crate) fn with_recent_picks<T>(archetypes: Vec<RecentPick>, palettes: Vec<RecentPick>, f: impl FnOnce() -> T) -> T {
    RECENT_PICKS.with(|r| *r.borrow_mut() = (archetypes, palettes));
    let out = f();
    RECENT_PICKS.with(|r| *r.borrow_mut() = (Vec::new(), Vec::new()));
    out
}

/// Fit points below the best that still compete on freshness when the ledger steers a deal.
const FIT_BAND: i32 = 9;
/// How far below the best palette a niche-seeded pick may fall, and how many it chooses among.
const NICHE_BAND: i32 = 5;
const NICHE_SPREAD: usize = 12;

/// The register a brief speaks in, read for gating looks: its three strongest registers, and
/// whether it is aspirational (premium or cinematic among them). A long brief names words from
/// nearly every register, so only the strongest decide; an aspirational brief is offered looks
/// made for premium or cinematic work first, never a document metaphor that merely fits a
/// weaker register it also mentions.
pub(crate) struct RegisterRead {
    pub(crate) top: Vec<&'static str>,
    pub(crate) aspire: bool,
    /// The neighbour register added to a one-register read (widens archetypes only).
    pub(crate) near: Option<&'static str>,
}

impl RegisterRead {
    pub(crate) fn of(query: &str) -> Self {
        use crate::register::{named_registers, REGISTERS};
        let read = named_registers(query);
        // A brief that names no register (an unusual trade) gates nothing: every register stays
        // open, as before registers existed.
        if read.is_empty() {
            return Self { top: REGISTERS.to_vec(), aspire: false, near: None };
        }
        let mut top: Vec<&'static str> = read.into_iter().take(3).collect();
        // A brief read as one register alone also takes its nearest neighbour, so a bakery can be
        // an editorial index as well as a menu card; premium and cinematic only pair with each other.
        let mut near = None;
        if let [only] = top[..] {
            let n = match only {
                "craft" => "editorial",
                "technical" => "corporate",
                "premium" => "cinematic",
                "cinematic" => "premium",
                "playful" => "craft",
                "civic" => "corporate",
                "editorial" => "premium",
                _ => "technical",
            };
            top.push(n);
            near = Some(n);
        }
        // Aspirational only on a strong word (premium, luxury, cinematic...); a passing "launch"
        // or "3D" leaves the brief's other registers in charge.
        let l = query.to_lowercase();
        let strong = ["premium", "luxury", "luxe", "high-end", "flagship", "cinematic", "immersive", "filmic", "award-winning", "awwwards", "sophisticated", "bespoke"];
        let aspire = top.iter().any(|r| matches!(*r, "premium" | "cinematic")) && strong.iter().any(|w| l.contains(w));
        Self { top, aspire, near }
    }

    /// The read without its neighbour register: motifs and buttons answer only to what the
    /// brief itself names.
    pub(crate) fn strict(&self) -> Self {
        Self { top: self.top.iter().copied().filter(|r| Some(*r) != self.near).collect(), aspire: self.aspire, near: None }
    }

    /// Adds a register the trade implies (a solemn trade reads civic) to a named read.
    pub(crate) fn with(mut self, extra: Option<&'static str>) -> Self {
        if let Some(r) = extra.filter(|r| !self.top.contains(r)) {
            self.top.push(r);
        }
        self
    }

    /// 2: made for the brief's aspirational register; 1: fits one of its top registers (or is
    /// untagged); 0: foreign to the brief's register.
    pub(crate) fn tier(&self, regs: &[&str]) -> u8 {
        if regs.is_empty() {
            return 1;
        }
        // An aspirational brief never takes a look also made for a playful or civic page it did
        // not ask for.
        if self.aspire && regs.iter().any(|r| matches!(*r, "playful" | "civic") && !self.top.contains(r)) {
            return 0;
        }
        if self.aspire && regs.iter().any(|r| matches!(*r, "premium" | "cinematic") && self.top.contains(r)) {
            2
        } else if crate::register::fits(regs, &self.top) {
            1
        } else {
            0
        }
    }

    /// Gate a pool: keep the aspirational fits when at least `need` exist, else the register fits
    /// when at least `need` exist; returns the kept items and a score bonus/penalty per item
    /// (heavy penalty only when too few fit and the foreign ones had to stay).
    pub(crate) fn gate<T: Copy>(&self, pool: &[T], need: usize, regs: impl Fn(T) -> &'static [&'static str], penalty: i32) -> Vec<(T, i32)> {
        let tiers: Vec<u8> = pool.iter().map(|x| self.tier(regs(*x))).collect();
        let n2 = tiers.iter().filter(|t| **t == 2).count();
        let n1 = tiers.iter().filter(|t| **t >= 1).count();
        let floor = if n2 >= need { 2 } else if n1 >= need { 1 } else { 0 };
        pool.iter()
            .zip(tiers)
            .filter(|(_, t)| *t >= floor)
            .map(|(x, t)| {
                let adj = match (floor, t) {
                    (1, 2) => 6,
                    (0, 0) => penalty,
                    (0, 2) => 6,
                    _ => 0,
                };
                (*x, adj)
            })
            .collect()
    }
}

/// A ledger pick: id, runs back (0 = the newest run) and slot in that run (0 = direction A).
pub(crate) type RecentPick = (String, usize, usize);

/// Score cost of reusing `id` in `slot`: last run's pick pays more than the whole fit band, so any
/// other fitting pick beats it; the cost falls one point per run back, so among fitting picks the
/// least recently used wins. The slot that used it last pays one more, so a direction never keeps
/// re-taking its own last pick while the others rotate.
fn recency_cost(newest_first: &[RecentPick], id: &str, slot: usize) -> i32 {
    newest_first.iter().filter(|p| p.0 == id).map(|(_, age, s)| (FIT_BAND + 1 - *age as i32).max(1) + i32::from(*s == slot)).max().unwrap_or(0)
}

thread_local! {
    /// True while dealing for a niche whose page shape moves at the heavy tier, set by [`with_heavy_tier`].
    static HEAVY_TIER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Runs `f` with the page shape's motion tier known to the archetype deal: a heavy-motion niche is
/// never dealt a layout whose own motion is static (price lists, status boards, focus-only pages).
pub(crate) fn with_heavy_tier<T>(on: bool, f: impl FnOnce() -> T) -> T {
    let was = HEAVY_TIER.with(|h| h.replace(on));
    let out = f();
    HEAVY_TIER.with(|h| h.set(was));
    out
}

/// Whether an archetype's bold motion is still static (nothing moves with scroll or on entry).
fn static_motion(a: &Archetype) -> bool {
    let m = a.motion_bold.to_lowercase();
    ["no motion", "nothing moves", "nothing else moves", "no scroll", "zero decorative", "only;", "only (", "only."].iter().any(|p| m.contains(p)) || m.trim_end_matches('.').ends_with(" only")
}

/// Family of a face: optical and width cuts ("Libre Caslon Display", "Libre Caslon Text",
/// "Sofia Sans Condensed") belong to one family.
pub(crate) fn family(face: &str) -> String {
    const CUTS: &[&str] = &["display", "text", "condensed", "semi", "extra", "narrow", "sc", "caption", "headline", "deck", "micro", "subhead", "poster", "pro"];
    let mut w: Vec<&str> = face.split_whitespace().collect();
    while w.len() > 1 && CUTS.contains(&w[w.len() - 1].to_lowercase().as_str()) {
        w.pop();
    }
    w.join(" ").to_lowercase()
}

/// Hero materials with no human presence; solemn briefs (funerals, care, law) avoid them.
const COLD_HEROES: &[&str] = &["object", "illustration", "code-output", "data-as-image", "live-ui", "procedural-texture", "collage"];

/// Commercial families a brief may name as its brand type. They are never linked to a free CDN:
/// the direction says the brand must self-host them under its own licence.
const PAID_FACES: &[&str] = &[
    "neue montreal", "pp neue montreal", "helvetica neue", "helvetica", "graphik", "gt america", "gt walsheim", "gt sectra", "circular",
    "founders grotesk", "sohne", "s\u{f6}hne", "aeonik", "suisse int'l", "akzidenz-grotesk", "futura pt", "proxima nova", "avenir",
    "gotham", "brandon grotesque", "freight text", "tiempos", "canela",
];

pub(crate) fn is_paid_face(name: &str) -> bool {
    PAID_FACES.contains(&name.to_lowercase().as_str())
}

/// The brief's own type: names rotate through display and body per direction; a mono name fills
/// the mono role. Returns the pairing (main stylesheet), a second stylesheet when faces come from
/// two sources (Fontshare faces with a Google mono), and notes for paid faces.
fn brand_type(fonts: &[String], roles: &std::collections::BTreeMap<String, String>, slot: u8, base: &TypographyPairing) -> (TypographyPairing, Option<String>, Vec<String>) {
    let names: Vec<&String> = fonts.iter().filter(|f| !is_mono_name(f)).collect();
    // Fonts the brief labels by role ("Headings: ...", "Body: ...") keep that role; each direction
    // takes its own pick among the alternatives.
    let by_role = |r: &str| names.iter().copied().filter(|f| roles.get(*f).is_some_and(|x| x == r)).collect::<Vec<_>>();
    let (heads, bodies) = (by_role("display"), by_role("body"));
    // A brief that names two or more faces has its system; a third, unasked mono face only adds noise.
    let mono = fonts.iter().find(|f| is_mono_name(f)).cloned().unwrap_or_else(|| if !names.is_empty() { names[(slot as usize + 1) % names.len()].clone() } else { base.mono_font.clone() });
    let (display, body) = match names.len() {
        _ if !heads.is_empty() && !bodies.is_empty() => (heads[slot as usize % heads.len()].clone(), bodies[slot as usize % bodies.len()].clone()),
        _ if !heads.is_empty() => (heads[slot as usize % heads.len()].clone(), base.body_font.clone()),
        _ if !bodies.is_empty() => (base.display_font.clone(), bodies[slot as usize % bodies.len()].clone()),
        0 => (base.display_font.clone(), base.body_font.clone()),
        // One named face leads display; body comes from the base so long text stays legible.
        1 => (names[0].clone(), names[0].clone()),
        n => (names[slot as usize % n].clone(), names[(slot as usize + 1) % n].clone()),
    };
    let fontshare = |f: &str| type_pool().iter().any(|e| e.source == "fontshare" && (e.pairing.display_font == f || e.pairing.body_font == f));
    let mut fams: Vec<&String> = vec![&display, &body, &mono];
    fams.sort();
    fams.dedup();
    let paid: Vec<String> = fams
        .iter()
        .filter(|f| is_paid_face(f))
        .map(|f| format!("{f} is a commercial face: self-host it under the brand's own licence; it is not linked to a free CDN"))
        .collect();
    let known_free = |f: &str| type_pool().iter().any(|e| e.pairing.display_font == f || e.pairing.body_font == f || e.pairing.mono_font == f);
    let free: Vec<&String> = fams.iter().copied().filter(|f| !is_paid_face(f) && known_free(f)).collect();
    let google: Vec<String> = free.iter().filter(|f| !fontshare(f)).map(|f| {
            // Bold headings need the 700 cut; Google rejects the whole sheet for a weight a face lacks,
            // so single-weight display faces are asked for as they are.
            const ONE_WEIGHT: &[&str] = &["Anton", "Bebas Neue", "Abril Fatface", "Alfa Slab One", "Archivo Black", "Bungee", "Righteous", "Lobster", "Pacifico", "Ultra", "Monoton", "Shrikhand", "Rubik Mono One", "Dela Gothic One", "Titan One", "DM Serif Display", "Instrument Serif", "Black Han Sans"];
            if ONE_WEIGHT.contains(&f.as_str()) { format!("family={}", f.replace(' ', "+")) } else { format!("family={}:wght@400;700", f.replace(' ', "+")) }
        }).collect();
    let shared: Vec<String> = free.iter().filter(|f| fontshare(f)).map(|f| format!("f[]={}@400,700", f.to_lowercase().replace(' ', "-"))).collect();
    let g_url = (!google.is_empty()).then(|| format!("https://fonts.googleapis.com/css2?{}&display=swap", google.join("&")));
    let f_url = (!shared.is_empty()).then(|| format!("https://api.fontshare.com/v2/css?{}&display=swap", shared.join("&")));
    let (url, extra) = match (f_url, g_url) {
        (Some(f), g) => (f, g),
        (None, Some(g)) => (g, None),
        (None, None) => (String::new(), None),
    };
    let slug = |s: &str| s.to_lowercase().replace(' ', "-");
    let p = pairing(
        &format!("brand-{}-{}", slug(&display), slug(&body)),
        &display,
        &body,
        &mono,
        &format!("The brief's own type ({}); scale and roles from {}", fonts.join(", "), base.id),
        &base.scale_ratio,
        &url,
    );
    (p, extra, paid)
}

/// The brief's tag weights: niche tags and moods the brief states (+4) and subject tags read from
/// the lead trade (+3; later subject tags +1).
fn tag_weight(signals: &BriefSignals, subject: &Subject, tag: &str) -> i32 {
    if signals.niche_tags.iter().any(|t| t == tag) || signals.moods.iter().any(|m| m == tag) {
        4
    } else if subject.tags.first() == Some(&tag) {
        3
    } else if subject.tags.contains(&tag) {
        1
    } else {
        0
    }
}

/// Deals the three directions. `industry` is the matched profile's own type and palette; it keeps
/// Direction A at variant 0. A variant > 0 never repeats variant 0's pick in any slot (archetype,
/// display font, palette) while another fit exists, and consecutive variants differ.
///
/// Rules: each direction comes from a different look family; the three hero materials are never
/// all type-only; corner languages differ; each pick differs from the previous direction on at
/// least one axis (ground, display style, accent hue, warmth); a place or language in the brief
/// asks for a pairing that covers its script; the brief's own colours and fonts lead.
pub(crate) fn deal(signals: &BriefSignals, query: &str, variant: u64, industry: Option<(&TypographyPairing, &PaletteRecommendation)>) -> Vec<Dealt> {
    let base = if variant > 0 { Some(deal(signals, query, 0, industry)) } else { None };
    let subject = read_subject(query, signals);
    let q = query.to_lowercase();
    let subject_tokens = tokenize(&crate::intelligence::focus_line(query).unwrap_or_else(|| subject.phrase.clone()));
    let subject_has = |words: &[&str]| words.iter().any(|word| keyword_hit(&subject_tokens, word));
    // A visually different candidate still has to be deliverable with the brief's
    // capabilities and evidence. Novelty never buys invented quotes or unavailable slots.
    let compatible = |a: &&Archetype| {
        (signals.motion_cap.is_none_or(|cap| cap > 1) || a.hero_material != "video")
            // These layouts imply sellable inventory. A kinetic mood or a portfolio
            // project's event vocabulary does not make the client a venue/tattoo shop.
            && (a.key != "night-programme"
                || subject_has(&["festival", "concert", "music venue", "nightclub", "theatre", "theater", "cinema", "events venue", "club night"]))
            && (a.key != "flash-sheet"
                || subject_has(&["tattoo", "tattooist", "tattoos", "piercing", "print shop", "prints"]))
            && (a.key != "quote-led" || q.contains("verified quote:"))
            && (!matches!(a.key, "booking-board" | "week-grid")
                || ["time slots", "availability", "classes", "sessions", "timetable"].iter().any(|s| q.contains(s)))
            && (a.key != "component-playground" || signals.surface != S::ProductUi
                || ["component", "design system", "developer", "code playground"].iter().any(|s| q.contains(s)))
            && (a.key != "service-area"
                || ["postcode", "service area", "coverage check", "call-out", "callout"].iter().any(|s| q.contains(s)))
    };
    let h = |s: &str| fnv1a(&format!("{s}|{q}"));
    let w = |tag: &str| tag_weight(signals, &subject, tag);
    let has = |tag: &str| w(tag) > 0;
    let mood = |m: &str| signals.moods.iter().any(|x| x == m);
    let wants = |t: &str| has(t) || (t == "data" && signals.surface == S::ProductUi) || (t == "code" && signals.surface == S::DevTool);
    let solemn = has("memorial") || (has("legal") && !has("kinetic"));
    let playful = has("play") || has("celebration") || has("sport");
    let script = brief_script(query);
    // Display gimmicks (sci-fi, pixel, arcade, novelty scripts) only when the brief asks for that register.
    let gimmick_ok = ["kinetic", "night", "y2k", "play", "celebration", "70s", "sport", "ink"].iter().any(|t| has(t))
        || ["retro", "arcade", "game", "gaming", "sci-fi", "scifi", "futuristic", "neon", "pixel", "space", "cyber"].iter().any(|k| q.contains(k));
    let no_mono = !(signals.surface == S::DevTool || has("code") || ["monospace", "mono font", "code font"].iter().any(|k| q.contains(k)));
    let neon_ok = has("night") || ["neon", "cyber", "arcade", "nightclub", "nightlife", "rave", "synthwave"].iter().any(|k| q.contains(k));
    // Task-first briefs (money, law, code, utilities, care, dashboards) read; posters only when asked.
    let asks_bold = mood("bold") || ["editorial", "manifesto", "campaign", "poster", "bold"].iter().any(|k| q.contains(k));
    let task_first = matches!(signals.surface, S::ProductUi | S::DevTool) || ["finance", "legal", "code", "infrastructure", "civic", "trades", "care"].iter().any(|t| has(t));
    // A map hero needs a place to map: a venue, a named place, a service area or a location word.
    let words: Vec<&str> = q.split(|c: char| !c.is_alphanumeric()).collect();
    let heavy_tier = HEAVY_TIER.with(|h| h.get());
    let asks_photo = ["photography", "photographs", "photo ", "photos", "footage", "cinematic", "film still", "video wall", "render"]
        .iter()
        .any(|k| q.contains(k));
    let asks_cinematic = ["cinematic", "immersive", "film-like", "filmic"].iter().any(|k| q.contains(k));
    // Solemn trades (law, care, memorial) read in the civic register too: plain, human, trusted.
    let reg = RegisterRead::of(query).with(solemn.then_some("civic"));
    let place = signals.surface == S::Venue
        || script.is_some()
        || ["civic", "infrastructure", "trades"].iter().any(|t| has(t))
        || words.iter().any(|w| PLACE_WORDS.contains(w));
    // Nothing matched at all: seed the pick from the brief itself so unrelated briefs spread over
    // the whole pool instead of all landing on the same top-scored entries.
    let no_match = industry.is_none() && !subject.from_trades;
    let seed = fnv1a(&q);
    let pick = |n: usize| -> usize {
        if no_match && variant == 0 && n > 0 { (seed % n as u64) as usize } else { rotate(variant, n) }
    };
    let (margin, min) = if no_match { (4, 5) } else { (2, 2) };
    let types = type_pool();
    let pals = palette_pool();
    // How many pairings share each body family: the commonest (Karla, Source Sans) yield to rarer ones.
    let mut body_freq: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    for e in types {
        *body_freq.entry(family(&e.pairing.body_font)).or_default() += 1;
    }
    let ground_tone = brand_ground(&signals.brief_colors).map(|g| if luminance(g).unwrap_or(1.0) < 0.5 { Tone::Dark } else { Tone::Light });
    // Ground and hue the brief names in words pin the palette; every other axis is still dealt.
    let (tone_pin, hue_pin) = page_shape::palette_pins(query);

    let mut out: Vec<Dealt> = Vec::new();
    let mut used_comp: Vec<&str> = Vec::new();
    let mut used_fonts: Vec<String> = Vec::new();
    let mut used_bodies: Vec<String> = Vec::new();
    let mut used_pals: Vec<String> = Vec::new();
    let mut used_tones: Vec<Tone> = Vec::new();
    let mut used_families: Vec<&str> = Vec::new();
    let mut used_heroes: Vec<&str> = Vec::new();
    let mut used_radius: Vec<&str> = Vec::new();
    // Axes of the previous direction: (tone, accent hue, warmth, display style).
    let mut prev_axes: Option<(Tone, &str, &str, &str)> = None;
    let (recent_display, recent_body) = RECENT_FACES.with(|r| r.borrow().clone());
    used_fonts.extend(recent_display.iter().cloned());
    used_bodies.extend(recent_body.iter().cloned());
    let (recent_archs, recent_pals) = RECENT_PICKS.with(|r| r.borrow().clone());
    // The ledger's last looks were mostly pale grounds: lean this deal toward dark and colour fields.
    let pale_run = {
        let last: Vec<Tone> = recent_pals.iter().take(6).filter_map(|(id, _, _)| pals.iter().find(|e| &e.palette.id == id)).map(|e| e.tone).collect();
        last.len() >= 3 && last.iter().filter(|t| **t == Tone::Light).count() * 3 >= last.len() * 2
    };
    if let Some((t, p)) = industry {
        // B and C never reuse the industry's own display face or palette, even when A re-deals.
        used_fonts.push(family(&t.display_font));
        used_pals.push(p.id.clone());
    }
    for slot in 0..3u8 {
        let role = ["A", "B", "C"][slot as usize];
        let prev = base.as_ref().map(|b| &b[slot as usize]);
        // Archetype
        let pool: Vec<&'static Archetype> = ARCHETYPES()
            .iter()
            .filter(compatible)
            .filter(|a| a.roles.contains(&role) && fits(a, signals.surface) && (a.needs.is_empty() || a.needs.iter().any(|n| has(n))))
            .collect();
        // Register before novelty: only layouts that read as professional in the brief's register
        // compete (while at least two fit); a foreign one stays only under a heavy penalty.
        // A role with fewer than two layouts in the brief's register borrows fitting ones written
        // for the other roles, rather than dealing a foreign look under a penalty.
        let pool = if pool.iter().filter(|a| reg.tier(a.registers) >= 1).count() < 2 {
            let borrowed: Vec<&'static Archetype> = ARCHETYPES()
                .iter()
                .filter(compatible)
                .filter(|a| fits(a, signals.surface) && (a.needs.is_empty() || a.needs.iter().any(|n| has(n))))
                .filter(|a| reg.tier(a.registers) >= 1 && !used_families.contains(&a.look_family))
                .collect();
            if borrowed.len() >= 2 {
                borrowed
            } else {
                pool
            }
        } else {
            pool
        };
        let gated = reg.gate(&pool, 2, |a: &'static Archetype| a.registers, -40);
        let reg_adj = |a: &Archetype| gated.iter().find(|(g, _)| g.key == a.key).map_or(0, |(_, j)| *j);
        let pool: Vec<&'static Archetype> = gated.iter().map(|(a, _)| *a).collect();
        let fresh: Vec<&'static Archetype> = pool.iter().copied().filter(|a| !used_families.contains(&a.look_family)).collect();
        let pool = if fresh.len() >= 2 { fresh } else { pool };
        let mut ranked: Vec<(i32, &'static Archetype)> = pool
            .into_iter()
            .map(|a| {
                let mut s = reg_adj(a);
                // A structure made for this specific surface beats a general one; "general" briefs
                // get no such bonus, so venue-only ideas (maps, booking boards) do not lead them.
                s += if a.surfaces.len() <= 2 && signals.surface != S::General { 4 } else { 2 };
                s += 2 * a.calm_bias as i32 * (mood("calm") as i32 - mood("bold") as i32);
                if signals.motion_bias >= 4 && a.composition == "sticky-stage" {
                    s += 2;
                }
                if used_comp.contains(&a.composition) {
                    s -= 10;
                }
                if used_families.contains(&a.look_family) {
                    s -= 12;
                }
                s += a.tags.iter().map(|t| w(t)).sum::<i32>().min(8);
                // A layout led by a ceremonial or personal trade (a memorial room, a nursery) reads as
                // borrowed on any other trade, even when it shares the brief's moods (luxe, night).
                let personal = a.tags.iter().find(|t| SUBJECT_TAGS.contains(t)).is_some_and(|t| matches!(*t, "memorial" | "care" | "play" | "celebration" | "ink"));
                if !solemn && !subject.tags.is_empty() && personal && foreign(a.tags, &wants) {
                    s -= 8;
                }
                // A layout written for other trades (a drawing sheet for engineers, a price board for
                // trades) must not land on a showpiece, shop or product brief that shares none of its
                // subjects: fit before novelty. Quiet trades keep their warm cross-trade layouts.
                if heavy_tier && !a.tags.is_empty() && !a.tags.iter().any(|t| wants(t)) {
                    s -= 6;
                }
                // A niche dealt heavy motion gets layouts that move; and a showpiece or shop leads with
                // a visual (object, photo, illustration, scene), not a document or a wall of figures.
                if heavy_tier && signals.surface != S::ProductUi {
                    if static_motion(a) {
                        s -= 15;
                    }
                    if matches!(signals.surface, S::Showcase | S::Commerce) && (matches!(a.hero_material, "document" | "data-as-image") || (signals.surface == S::Commerce && a.hero_material == "giant-type")) {
                        s -= 6;
                    }
                }
                if !a.needs.is_empty() {
                    s += 2;
                }
                if a.avoid.iter().any(|t| has(t)) {
                    s -= 20;
                }
                if used_heroes.contains(&a.hero_material) {
                    s -= 4;
                }
                if slot == 2 && a.hero_material == "giant-type" && used_heroes.iter().filter(|h| **h == "giant-type").count() == 2 {
                    s -= 30;
                }
                if solemn && (a.tone == Tone::Field || a.composition == "poster") {
                    s -= 8;
                }
                if solemn && a.calm_bias < 0 {
                    s -= 4;
                }
                // Solemn briefs want human presence (people, places, letters), not cold kit or UI.
                if solemn {
                    if COLD_HEROES.contains(&a.hero_material) {
                        s -= 8;
                    } else if matches!(a.hero_material, "photo" | "document" | "video") {
                        s += 3;
                    }
                }
                if task_first && !asks_bold && (a.composition == "poster" || a.key == "manifesto") {
                    s -= 10;
                }
                if a.hero_material == "map" && !place {
                    s -= 10;
                }
                // A brief that asks for photography, footage or a cinematic scene wants the real
                // subject first, not a drawing sheet or a figure wall.
                // A cinematic or immersive ask reads as a staged scene, never as a newspaper, a
                // drawing sheet or a paper page.
                if asks_cinematic {
                    if matches!(a.look_family, "cinematic-dark" | "luxury-sparse") {
                        s += 6;
                    } else if matches!(a.look_family, "hairline-newspaper" | "technical-blueprint" | "warm-paper-serif") {
                        s -= 8;
                    }
                }
                if asks_photo {
                    if matches!(a.hero_material, "illustration" | "document" | "data-as-image" | "giant-type") {
                        s -= 8;
                    } else if matches!(a.hero_material, "photo" | "video" | "object") {
                        s += 4;
                    }
                }
                if has("care") && matches!(a.hero_material, "photo" | "video") { s += 7; }
                if (has("care") || q.contains("architecture")) && a.key.contains("before-after") && !q.contains("before and after") && !q.contains("before-after") { s -= 30; }
                (s, a)
            })
            .collect();
        // Niche fit gates first: with a ledger, only archetypes near the best fit compete on
        // freshness, so a recent pick yields to another fitting one, never to a foreign trade's.
        if !recent_archs.is_empty() {
            let top = ranked.iter().map(|x| x.0).max().unwrap_or(0);
            ranked.retain(|x| x.0 >= top - FIT_BAND);
            // A tagged layout written for other trades only (an almanac for a phone shop) stays out
            // while any layout that fits remains.
            let own = |a: &Archetype| a.tags.is_empty() || a.tags.iter().any(|t| wants(t));
            if ranked.iter().any(|x| own(x.1)) {
                ranked.retain(|x| own(x.1));
            }
            for x in ranked.iter_mut() {
                x.0 -= recency_cost(&recent_archs, x.1.key, slot as usize);
            }
        }
        ranked.sort_by(|x, y| y.0.cmp(&x.0).then_with(|| h(x.1.key).cmp(&h(y.1.key))));
        let mut cands = window(&ranked, margin, min);
        if let Some(p) = prev {
            if cands.len() > 1 {
                cands.retain(|a| a.key != p.arch.key);
            }
        }
        let arch = cands[pick(cands.len())];
        let alternatives: Vec<&'static str> = ranked.iter().filter(|(s, a)| a.key != arch.key && *s > -5).take(6).map(|(_, a)| a.name).collect();
        used_comp.push(arch.composition);
        used_families.push(arch.look_family);
        used_heroes.push(arch.hero_material);
        let radius = arch
            .radius
            .iter()
            .copied()
            .filter(|r| *r != "round-play" || playful)
            .find(|r| !used_radius.contains(r))
            .or_else(|| ["hairline-2", "tool-6", "soft-12", "sharp-0", "cut-corner", "arched"].into_iter().find(|r| !used_radius.contains(r)))
            .unwrap_or("hairline-2");
        used_radius.push(radius);

        // All candidates use the same semantic fit process; candidate A has no industry-default privilege.
        let mut tr: Vec<(i32, usize)> = types
            .iter()
            .enumerate()
            .filter(|(_, e)| !used_fonts.contains(&family(&e.pairing.display_font)) && display_capable(&e.pairing.display_font, &e.pairing.body_font))
            // Faces drawn for a non-Latin script (Hahmlet, M PLUS, Suez One) never set a brief written
            // for Latin text, even once the ledger has used up the Latin faces that fit.
            .filter(|(_, e)| script.is_some() || !e.scripts.iter().any(|x| NON_LATIN.contains(x)))
            .map(|(i, e)| {
                let mut s = if e.voice == arch.voice { 4 } else { 0 };
                s += e.tags.iter().map(|t| if *t == "data" && signals.surface == S::ProductUi { 4 } else { w(t) }).sum::<i32>();
                if signals.surface == S::DevTool && e.tags.contains(&"code") {
                    s += 2;
                }
                if foreign(e.tags, &wants) {
                    s -= 7;
                }
                if e.look_family == arch.look_family {
                    s += 2;
                }
                match script {
                    Some(sc) if e.scripts.contains(&sc) => s += 12,
                    _ => {}
                }
                if e.overused {
                    s -= 4;
                }
                if solemn && e.tags.iter().any(|t| LOUD.contains(t)) && !e.tags.iter().any(|t| has(t) && SUBJECT_TAGS.contains(t)) {
                    s -= 6;
                }
                if !playful && matches!(e.display_style, "rounded" | "hand") && solemn {
                    s -= 4;
                }
                if !gimmick_ok && e.tags.contains(&"novelty") {
                    s -= 12;
                }
                // The page's restraint bans a monospace face unless the brief is about code.
                if no_mono && (e.display_style == "mono" || is_mono_name(&e.pairing.display_font) || is_mono_name(&e.pairing.body_font)) {
                    s -= 20;
                }
                // Variety: a face nothing in the brief asks for loses to one it does, and a small
                // brief-seeded jitter stops the same generalist face winning unrelated briefs.
                if !e.tags.iter().any(|t| w(t) > 0) {
                    s -= 2;
                }
                s += (h(&e.pairing.display_font) % 4) as i32;
                // A face picked only for a surface tag (code, data) yields to one on the brief's subject.
                if !subject.tags.is_empty() && !e.tags.iter().any(|t| subject.tags.contains(t)) {
                    s -= 2;
                }
                let bf = family(&e.pairing.body_font);
                s -= body_freq.get(&bf).copied().unwrap_or(0) / 3;
                s += (h(&bf) % 3) as i32;
                if used_bodies.contains(&bf) {
                    s -= 4;
                }
                (s, i)
            })
            .collect();
        tr.sort_by(|x, y| y.0.cmp(&x.0).then_with(|| h(&types[x.1].pairing.id).cmp(&h(&types[y.1].pairing.id))));
        let mut tc = window(&tr, margin, min.max(3));
        if let Some(p) = prev {
            if tc.len() > 1 {
                tc.retain(|i| types[*i].pairing.display_font != p.typography.display_font);
            }
        }
        let te = &types[tc[pick(tc.len())]];

        let mut pr: Vec<(i32, usize)> = pals
            .iter()
            .enumerate()
            .filter(|(_, e)| !used_pals.contains(&e.palette.id))
            .map(|(i, e)| {
                let mut s = if e.tone == arch.tone { 4 } else { 0 };
                if used_tones.contains(&e.tone) {
                    // Most archetypes are written light, so a second pale ground costs more.
                    s -= if e.tone == Tone::Light { 5 } else { 3 };
                }
                if e.tone == Tone::Light && pale_run {
                    s -= 4;
                }
                s += e.tags.iter().map(|t| if *t == "data" && signals.surface == S::ProductUi { 4 } else { w(t) }).sum::<i32>();
                if signals.surface == S::DevTool && e.tags.contains(&"code") {
                    s += 2;
                }
                if foreign(e.tags, &wants) {
                    s -= 7;
                }
                if e.look_family == arch.look_family {
                    s += 2;
                }
                // Law and money briefs list navy-and-gold as their reflex palette: never deal it.
                if (has("legal") || has("finance")) && navy_gold(&e.palette) {
                    s -= 8;
                }
                // Dark + neon is the generic default look; deal it only when the brief asks for night or neon.
                if !neon_ok && neon_on_black(&e.palette) {
                    s -= 6;
                }
                if solemn && (e.tone == Tone::Field || e.tags.iter().any(|t| LOUD.contains(t))) {
                    s -= 6;
                }
                // Patients read a clinic by daylight: a dark care ground stays for briefs that ask for night.
                if has("care") && e.tone == Tone::Dark && !mood("dark") && !has("night") {
                    s -= 10;
                }
                if prev_axes.is_some_and(|(t, hue, warm, style)| t == e.tone && hue == e.accent_hue && warm == e.warmth && style == te.display_style) {
                    s -= 6;
                }
                if ground_tone.is_some_and(|g| g == e.tone) {
                    s += 6;
                }
                if let Some(t) = tone_pin {
                    s += if e.tone == t { 20 } else { -20 };
                }
                if let Some(hue) = hue_pin {
                    s += if e.accent_hue == hue { 12 } else { -12 };
                }
                match e.tone {
                    Tone::Dark if mood("dark") => s += 2,
                    Tone::Light if mood("light") => s += 2,
                    Tone::Field if mood("calm") => s -= 1,
                    _ => {}
                }
                (s, i)
            })
            .collect();
        if !recent_pals.is_empty() {
            let top = pr.iter().map(|x| x.0).max().unwrap_or(0);
            pr.retain(|x| x.0 >= top - FIT_BAND);
            // Palettes a remembered look already wore with this archetype: dealing one again would
            // rebuild that whole look, so it drops below every other fitting palette.
            let worn: Vec<&str> = recent_archs
                .iter()
                .filter(|a| a.0 == arch.key)
                .filter_map(|a| recent_pals.iter().find(|p| p.1 == a.1 && p.2 == a.2).map(|p| p.0.as_str()))
                .collect();
            for x in pr.iter_mut() {
                x.0 -= recency_cost(&recent_pals, &pals[x.1].palette.id, slot as usize);
                if worn.contains(&pals[x.1].palette.id.as_str()) {
                    x.0 -= 4 * FIT_BAND;
                }
            }
        }
        pr.sort_by(|x, y| y.0.cmp(&x.0).then_with(|| h(&pals[x.1].palette.id).cmp(&h(&pals[y.1].palette.id))));
        let mut pc = window(&pr, margin, min.max(3));
        if let Some(p) = prev {
            if pc.len() > 1 {
                pc.retain(|i| pals[*i].palette.id != p.palette.id && !p.palette.id.ends_with(&format!("-on-{}", pals[*i].palette.id)));
            }
        }
        // Niches of one trade score alike, so taking the single top palette would give every
        // bowling alley and soft-play centre the same colours. Each direction instead picks within
        // the near-top fit band, seeded by the niche itself: still a best fit, but its own.
        let near: Vec<usize> = if variant == 0 && !no_match && tone_pin.is_none() && hue_pin.is_none() && ground_tone.is_none() {
            let top = pr.first().map_or(0, |x| x.0);
            // Only palettes that speak one of the niche's own tags compete, when it has any,
            // and none may repeat every axis of the direction before it.
            let on_tag = |i: usize| {
                let e = &pals[i];
                (subject.tags.is_empty() || e.tags.iter().any(|t| has(t)))
                    && !prev_axes.is_some_and(|(t, hue, warm, style)| t == e.tone && hue == e.accent_hue && warm == e.warmth && style == te.display_style)
            };
            let band = |b: i32| -> Vec<usize> { pr.iter().take_while(|x| x.0 >= top - b).filter(|x| on_tag(x.1)).take(NICHE_SPREAD).map(|x| x.1).collect() };
            // A narrow trade with one close fit widens to the full fit band so it still has a choice.
            let near = band(NICHE_BAND);
            if near.len() < 5 { band(FIT_BAND) } else { near }
        } else {
            Vec::new()
        };
        let pe = if near.len() > 1 {
            &pals[near[(fnv1a(q.trim()) % near.len() as u64) as usize]]
        } else {
            &pals[pc[pick(pc.len())]]
        };
        used_fonts.push(family(&te.pairing.display_font));
        used_bodies.push(family(&te.pairing.body_font));
        used_pals.push(pe.palette.id.clone());
        used_tones.push(pe.tone);
        prev_axes = Some((pe.tone, pe.accent_hue, pe.warmth, te.display_style));
        out.push(Dealt {
            arch,
            slot,
            typography: {
                // A mood written for another trade ("harbour-signage grotesk" on an agency) would be
                // read as the reason for the face: describe its style instead.
                let mut p = te.pairing.clone();
                if te.tags.iter().find(|t| SUBJECT_TAGS.contains(t)).is_some_and(|t| !has(t)) {
                    p.mood = format!("{} display, {} family", te.display_style, te.look_family);
                }
                p
            },
            palette: pe.palette.clone(),
            tone: pe.tone,
            type_tags: te.tags.iter().copied().filter(|t| has(t)).collect(),
            palette_tags: pe.tags.iter().copied().filter(|t| has(t)).collect(),
            alternatives,
            from_industry: false,
            radius,
            script: script.filter(|s| te.scripts.contains(s)),
            type_entry: Some(te),
            brand: Vec::new(),
            extra_font_url: (!te.mono_url.is_empty()).then(|| te.mono_url.to_string()),
        });
    }
    // Brand first: the brief's own colours and fonts replace the dealt ones in every direction;
    // the dealt picks stay as the base they are fitted onto.
    for d in &mut out {
        if !signals.brief_colors.is_empty() {
            d.palette = brand_palette(&signals.brief_colors, &signals.brief_color_roles, &d.palette);
            d.tone = Tone::of(&d.palette);
            d.brand.push(format!("palette built around the brief's colours ({})", signals.brief_colors.join(", ")));
        }
        if !signals.brief_fonts.is_empty() {
            let (t, extra, paid) = brand_type(&signals.brief_fonts, &signals.brief_font_roles, d.slot, &d.typography);
            d.typography = t;
            d.extra_font_url = extra;
            d.brand.extend(paid);
            d.brand.push(format!("type built from the brief's fonts ({})", signals.brief_fonts.join(", ")));
        }
    }
    out
}

/// Inputs for writing one direction's copy.
pub(crate) struct Ctx<'a> {
    pub subject: &'a Subject,
    pub place: Option<&'a str>,
    pub material: Option<&'a str>,
    pub surface: Surface,
    /// False when the page's restraint rules out a monospace face: archetype text then names the
    /// body face's tabular figures instead.
    pub mono: bool,
}

pub(crate) struct DirectionText {
    pub layout: String,
    pub identity: String,
    pub content: String,
    pub motion: String,
}

fn cap(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

/// Writes layout, identity, content and motion for one dealt direction at its motion dial.
pub(crate) fn write(d: &Dealt, ctx: &Ctx, motion: u8) -> DirectionText {
    let s = ctx.subject;
    let subj = s.phrase.as_str();
    let (o1, o2, o3) = (s.objects[0].as_str(), s.objects[1].as_str(), s.objects[2].as_str());
    let at = ctx.place.map(|p| format!(" in {p}")).unwrap_or_default();
    let mat = ctx.material.map(str::to_string).unwrap_or_else(|| o1.trim_start_matches("the ").to_string());
    let notes = d.palette.notes.split(" | ").next().unwrap_or("");
    let pal = format!("palette {} ({}): {}", d.palette.id, d.tone.label(), notes.to_lowercase());
    let (t_in, t_ui) = match motion {
        0..=1 => (0, 0),
        2..=4 => (240, 150),
        5..=7 => (520, 180),
        _ => (800, 220),
    };
    let calm = motion <= 4 || ctx.surface == S::ProductUi;
    let mono = if ctx.mono { d.typography.mono_font.clone() } else { format!("{} tabular figures", d.typography.body_font) };
    let fill = |t: &str| -> String {
        t.replace("{big}", &cap(subj))
            .replace("{at}", &at)
            .replace("{subj}", subj)
            .replace("{o1}", o1)
            .replace("{o2}", o2)
            .replace("{o3}", o3)
            .replace("{act}", &s.action)
            .replace("{note}", &s.notation)
            .replace("{mat}", &mat)
            .replace("{display}", &d.typography.display_font)
            .replace("{body}", &d.typography.body_font)
            .replace("{mono}", &mono)
            .replace("{ratio}", &d.typography.scale_ratio)
            .replace("{pal}", &pal)
            .replace("{t_in}", &t_in.to_string())
            .replace("{t_ui}", &t_ui.to_string())
    };
    let a = d.arch;
    let layout = format!(
        "{}. Structure: {}. Knobs: density {}; span irregularity {}; accent on {}. Mobile: {}",
        fill(a.layout).trim_end_matches('.'),
        a.structure.trim_end_matches('.'),
        a.density,
        a.span_irregularity,
        a.accent_placement,
        a.mobile
    );
    let fallbacks = d.type_entry.filter(|e| e.pairing.display_font == d.typography.display_font).map(|e| e.fallbacks);
    let identity = format!(
        "{}; corners: {} ({}); body 1rem (16px) minimum, labels and captions 0.75rem (12px) minimum{}{}",
        fill(a.identity),
        radius_text(d.radius),
        d.radius,
        fallbacks.map(|f| format!("; fallbacks {f}")).unwrap_or_default(),
        d.extra_font_url
            .as_ref()
            .filter(|_| ctx.mono)
            .map(|u| format!("; {} loads from a second stylesheet: {u}", d.typography.mono_font))
            .unwrap_or_default()
    );
    let content = fill(a.content);
    let motion_text = fill(if calm { a.motion_calm } else { a.motion_bold });
    let motion_text = if motion <= 1 {
        format!("No motion: {o1} and every section render in their final state on load; hover and focus change colour only; nothing moves on scroll")
    } else if d.slot == 2 && motion >= 6 && ctx.surface != S::ProductUi {
        motion_text.replace("; nothing moves on scroll", &format!("; sections enter once on scroll ({t_in}ms, transform and opacity only)"))
    } else {
        motion_text
    };
    DirectionText { layout, identity, content, motion: motion_text }
}

/// Why this direction was dealt, from the brief's signals and the options chosen.
pub(crate) fn rationale(d: &Dealt, signals: &BriefSignals, subject: &Subject, industry_name: &str) -> String {
    let surface = format!("{:?}", signals.surface).to_lowercase();
    let mut out = format!("{} for \"{}\" ({} surface): {}.", d.arch.name, subject.phrase, surface, d.arch.why);
    if d.from_industry {
        out.push_str(&format!(" Type {} / {} and palette {} are the {} profile's own.", d.typography.display_font, d.typography.body_font, d.palette.id, industry_name));
    } else {
        let tags = |t: &[&str]| if t.is_empty() { String::new() } else { format!(", tagged {} like the brief", t.join("/")) };
        out.push_str(&format!(
            " Type {} / {} ({}) for {} {} voice{}. Palette {} on a {}{}.",
            d.typography.display_font,
            d.typography.body_font,
            d.typography.mood.to_lowercase(),
            if d.arch.voice == Voice::Utility { "a" } else { "an" },
            d.arch.voice.label(),
            tags(&d.type_tags),
            d.palette.id,
            d.tone.label(),
            tags(&d.palette_tags)
        ));
    }
    out.push_str(&format!(" First screen: {} composition", d.arch.composition));
    if d.arch.composition == "split" {
        out.push_str(" (a split, chosen only because two views must be read side by side)");
    }
    out.push_str(&format!(
        ", hero material {}, look family {}, corners {}. Motion flavour (tier and tools follow page_shape.motion): {}.",
        d.arch.hero_material,
        d.arch.look_family,
        d.radius,
        d.arch.motion_hint.trim_end_matches('.')
    ));
    if let Some(sc) = d.script {
        out.push_str(&format!(" The brief names a {sc}-script place or language and this pairing covers {sc}."));
    }
    if let Some(e) = d.type_entry.filter(|e| e.pairing.display_font == d.typography.display_font) {
        if e.overused {
            out.push_str(" Flag: this pairing uses a face seen on most generated sites; swap it if the brand allows.");
        }
        out.push_str(&format!(" Licence: {}.", e.licence));
    }
    for b in &d.brand {
        out.push_str(&format!(" Brand first: {b}."));
    }
    if !d.alternatives.is_empty() {
        out.push_str(&format!(" Other structures that fit this slot: {}.", d.alternatives.join(", ")));
    }
    if !subject.from_trades {
        out.push_str(" The trade was not recognised: replace the placeholder objects with the real things a visitor sees.");
        if !subject.nouns.is_empty() {
            out.push_str(&format!(" Picks were seeded from the brief's own words ({}).", subject.nouns.join(", ")));
        }
    }
    out
}

/// Clichés derived from the brief's surface and niche when no catalogue industry matched.
pub(crate) fn derived_cliches(signals: &BriefSignals) -> Vec<String> {
    cliches_for(signals, &[])
}

fn cliches_for(signals: &BriefSignals, subject_tags: &[&str]) -> Vec<String> {
    let mut v: Vec<&str> = match signals.surface {
        S::Venue => vec!["Full-bleed hero photo with a centred script headline, and the hours, prices or menu hidden in a PDF"],
        S::Commerce => vec!["White grid of identical product cards with a pill 'Add to cart'"],
        S::Showcase => vec!["Giant name plus a 'Selected works' thumbnail grid"],
        S::Editorial => vec!["Three-column blog-template card feed"],
        S::ProductUi => vec!["A row of four KPI cards over one line chart"],
        S::DevTool => vec!["Purple gradient with a glowing terminal mockup"],
        S::General => vec![],
    };
    let mut tags: Vec<&str> = signals.niche_tags.iter().map(String::as_str).collect();
    tags.extend(subject_tags.iter().copied().filter(|t| !signals.niche_tags.iter().any(|n| n == t)));
    for t in tags {
        v.extend(match t {
            "legal" => &["Scales of justice, gavels or columns as stock imagery", "Navy-and-gold 'trust' palette under a serif headline"][..],
            "play" => &["Primary-colour confetti, wobbly blobs and a cartoon mascot unrelated to the real place", "Rounded comic type for everything, including body text"][..],
            "care" => &["Stock smiling-professional photo on a teal gradient", "Icon trio (heart, shield, clock) with three vague promises"][..],
            "civic" => &["Grey page with one gold serif and a slideshow of exhibitions but no visit information"][..],
            "marine" => &["Anchors, rope and navy-stripe clip art"][..],
            "warm-grain" | "earth" => &["Cream background with a terracotta accent and an italic serif"][..],
            "textile" => &["Beige minimal store with a thin sans and no fabric close-ups"][..],
            "kinetic" => &["Random marquee strips and a cursor blob with no link to the work"][..],
            "botanical" => &["Leaf-pattern overlay on sage green"][..],
            "industrial" => &["Hazard stripes and gear icons as decoration"][..],
            "night" => &["Neon on black for every venue"][..],
            "code" => &["Terminal mockup by default with fake output"][..],
            "luxe" => &["Black and white with a single gold accent"][..],
            "data" => &["Sparkline-in-every-card dashboard template"][..],
            "memorial" => &["Doves, lilies and sunset stock photos under a script font", "Notices and service times buried below a slideshow"][..],
            "ink" => &["Skull-and-rose clip art and grunge texture over everything", "Instagram embed as the portfolio with no artist pages"][..],
            "food" => &["Overhead stock flat-lay of someone else's food"][..],
            "celebration" => &["Blush pink and champagne-gold script for every event", "Enquiry form with no dates, capacities or prices"][..],
            "infrastructure" => &["Stock pylons or pipes on a corporate blue gradient", "Outage and service status hidden behind a news feed"][..],
            "trades" => &["Hard-hat handshake stock photo and a quote form with no prices or service area"][..],
            "literary" => &["Stack-of-old-books stock photo with a script logo"][..],
            "finance" => &["Floating 3D coins and a blue-to-purple gradient"][..],
            "sport" => &["Motion-blurred stock athlete under italic caps"][..],
            _ => &[][..],
        });
    }
    v.extend([
        "Template hero (headline left, illustration right, two buttons)",
        "Uniform card grids with icon + heading + paragraph",
        "Gradient blobs and glassmorphism with no brand reason",
    ]);
    let mut out: Vec<String> = Vec::new();
    for c in v {
        if !out.iter().any(|x| x == c) {
            out.push(c.to_string());
        }
    }
    out
}

/// A profile built from the brief's own words when no catalogue industry matched (confidence `none`).
pub(crate) fn brief_industry(query: &str, signals: &BriefSignals, subject: &Subject, typography: TypographyPairing, palette: PaletteRecommendation) -> IndustryRule {
    let o1 = &subject.objects[0];
    let act = &subject.action;
    let note = &subject.notation;
    let styles: &[&str] = match signals.surface {
        S::ProductUi => &["high-density-workbench", "crisp-product-workbench"],
        S::DevTool => &["precision-terminal-craft", "crisp-product-workbench"],
        S::Commerce => &["atelier-minimalism", "crisp-product-workbench"],
        S::Venue => &["kinetic-editorial", "atelier-minimalism"],
        S::Showcase => &["kinetic-editorial", "spatial-webgl-stage"],
        S::Editorial => &["block-first-editorial", "swiss-editorial-grid"],
        S::General => &["swiss-editorial-grid", "atelier-minimalism"],
    };
    let (v, m, dn) = match signals.surface {
        S::ProductUi => (3, 2, 7),
        S::DevTool => (5, 4, 6),
        S::Showcase => (7, 7, 4),
        S::Editorial => (5, 3, 6),
        _ if subject.tags.contains(&"memorial") => (3, 2, 4),
        _ => (5, 4, 5),
    };
    let motion_language = match signals.surface {
        S::ProductUi => "State changes only: value tweens and row insert/remove; nothing scroll-driven",
        S::DevTool => "Show the tool working (transcript or playground); copy and run states; little else",
        S::Showcase => "The work moves; the interface stays still around it",
        _ if subject.tags.contains(&"memorial") => "Still and quiet: focus and hover states only; nothing enters, loops or moves on scroll",
        _ => "Decide from the subject: motion that shows the trade's objects working, never decoration",
    };
    IndustryRule {
        slug: "brief-derived".into(),
        name: format!("{} (read from the brief)", cap(&subject.phrase)),
        keywords: tokenize(&subject.phrase),
        recommended_styles: styles.iter().map(|s| s.to_string()).collect(),
        layout_archetype: format!("{}: {} and '{}' lead the first screen; {} organise the rest", cap(&subject.phrase), o1, act, note),
        first_viewport_priority: format!("'{}' working next to {}", act, o1),
        typography_pairing: typography,
        palette,
        conversion_trust_elements: vec![
            "Real names, places and opening hours or response times".into(),
            format!("Specifics in the trade's own terms ({note})"),
            format!("One clear primary action: {act}"),
        ],
        motion_language: motion_language.into(),
        anti_patterns: cliches_for(signals, &subject.tags),
        default_dials: DesignDials::clamped(v, m, dn),
        provenance: format!(
            "Synthesised from the brief's own words (\"{}\"); no catalogue industry matched, so check it against what the user meant.",
            query.lines().next().unwrap_or("").chars().take(120).collect::<String>()
        ),
    }
}

pub mod page_shape;

#[cfg(test)]
mod tests;
