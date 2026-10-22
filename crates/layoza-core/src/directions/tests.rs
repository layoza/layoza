use super::*;

#[test]
fn portfolio_project_vocabulary_does_not_invent_the_clients_inventory() {
    let query = "Industry: independent web agency\nHighly animated kinetic website. Portfolio: Aster cultural programme, festival identity and event booking concept. No actual events or products for sale.";
    let signals = crate::intelligence::brief_signals(query);
    for variant in 0..32 {
        let directions = deal(&signals, query, variant, None);
        assert!(directions.iter().all(|d| !matches!(d.arch.key, "night-programme" | "flash-sheet")), "variant {variant}");
    }
}

#[test]
fn client_task_objects_and_available_capabilities_survive_dealing() {
    let query = "Industry: public library\nPrimary action: search the catalogue\nSubject objects: books, reading rooms, community events\nNo animation.";
    let signals = brief_signals(query);
    let subject = read_subject(query, &signals);
    assert_eq!(subject.action, "search the catalogue");
    assert_eq!(subject.objects, vec!["books", "reading rooms", "community events"]);
    for variant in 0..8 {
        let dealt = deal(&signals, query, variant, None);
        assert_eq!(dealt.len(), 3);
        for d in dealt {
            assert_ne!(d.arch.hero_material, "video");
            assert!(!["quote-led", "booking-board", "week-grid", "service-area"].contains(&d.arch.key));
        }
    }
}
use crate::intelligence::{brief_signals, recommend_design, recommend_design_variant, wcag_contrast, OperatingMode};
use std::collections::HashSet;

fn oklab(hex: &str) -> (f64, f64, f64) {
    let (r, g, b) = rgb(hex).unwrap();
    let lin = |c: u8| {
        let c = c as f64 / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    };
    let (r, g, b) = (lin(r), lin(g), lin(b));
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
    (
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    )
}

fn delta_e(a: &str, b: &str) -> f64 {
    let (x, y) = (oklab(a), oklab(b));
    ((x.0 - y.0).powi(2) + (x.1 - y.1).powi(2) + (x.2 - y.2).powi(2)).sqrt()
}

/// (hue degrees, chroma) in OKLCh.
fn lch(hex: &str) -> (f64, f64, f64) {
    let (l, a, b) = oklab(hex);
    ((b.atan2(a).to_degrees() + 360.0) % 360.0, (a * a + b * b).sqrt(), l)
}

#[test]
fn pools_are_wide_valid_and_aa() {
    let t = type_pool();
    let p = palette_pool();
    let a = ARCHETYPES();
    assert!(p.len() >= 200 && t.len() >= 150 && a.len() >= 60, "{} palettes, {} type, {} archetypes", p.len(), t.len(), a.len());
    let mut ids = HashSet::new();
    for e in t {
        assert!(ids.insert(e.pairing.id.clone()), "duplicate {}", e.pairing.id);
        let url = &e.pairing.google_fonts_url;
        assert!(url.starts_with("https://fonts.googleapis.com/css2?family=") || url.starts_with("https://api.fontshare.com/v2/css?"), "{}", e.pairing.id);
        assert!(matches!(e.source, "google" | "fontshare") && !e.licence.is_empty(), "{}", e.pairing.id);
        assert!(e.scripts.contains(&"latin") && !e.fallbacks.is_empty(), "{}", e.pairing.id);
    }
    for e in p {
        assert!(ids.insert(e.palette.id.clone()), "duplicate {}", e.palette.id);
        let c = |x: &str, y: &str| wcag_contrast(x, y).unwrap();
        let pl = &e.palette;
        for (name, v) in [("body/bg", c(&pl.foreground, &pl.background)), ("body/surface", c(&pl.foreground, &pl.surface)), ("muted/bg", c(&pl.muted, &pl.background)), ("muted/surface", c(&pl.muted, &pl.surface))] {
            assert!(v >= 4.5, "{} fails AA {name} {v:.2}", pl.id);
        }
        assert!(!e.look_family.is_empty() && !e.warmth.is_empty() && !e.accent_hue.is_empty(), "{}", pl.id);
    }
    let mut keys = HashSet::new();
    for x in a {
        assert!(keys.insert(x.key), "duplicate archetype {}", x.key);
        assert!(["centred", "full-bleed", "layered", "object-wrap", "poster", "split", "stacked", "sticky-stage"].contains(&x.composition), "{}", x.key);
        assert!(!x.structure.is_empty() && !x.motion_hint.is_empty() && !x.mobile.is_empty() && !x.radius.is_empty(), "{}", x.key);
    }
}

#[test]
fn palettes_are_distinct_and_free_of_slop() {
    let p = palette_pool();
    for (i, x) in p.iter().enumerate() {
        for y in &p[i + 1..] {
            let d = delta_e(&x.palette.primary_accent, &y.palette.primary_accent);
            assert!(d >= 0.04, "{} and {} primaries too close ({d:.3})", x.palette.id, y.palette.id);
        }
    }
    let tailwind = ["#3B82F6", "#2563EB", "#1D4ED8", "#6366F1", "#4F46E5", "#8B5CF6", "#7C3AED"];
    let mut acid = 0;
    for e in p {
        let pl = &e.palette;
        for hex in [&pl.primary_accent, &pl.secondary_accent, &pl.background] {
            assert!(!tailwind.contains(&hex.to_uppercase().as_str()), "{} uses Tailwind default {hex}", pl.id);
        }
        let (bh, bc, bl) = lch(&pl.background);
        let (ph, pc, pl_) = lch(&pl.primary_accent);
        // Cream ground with a clay/terracotta accent.
        let cream = bl > 0.9 && bc > 0.012 && (60.0..=110.0).contains(&bh);
        let clay = (25.0..=60.0).contains(&ph) && pc > 0.08 && (0.45..=0.7).contains(&pl_);
        assert!(!(cream && clay), "{} is cream + clay", pl.id);
        // Near-black ground with an acid yellow-green accent.
        if bl < 0.2 && (100.0..=135.0).contains(&ph) && pc > 0.15 && pl_ > 0.8 {
            acid += 1;
        }
        if e.tags.iter().any(|t| matches!(*t, "legal" | "finance")) {
            assert!(!navy_gold(pl), "{} is navy + gold", pl.id);
        }
    }
    assert!(acid <= 3, "{acid} near-black + acid palettes");
}

#[test]
fn type_overuse_is_rare_and_scripts_are_covered() {
    let t = type_pool();
    let over = t.iter().filter(|e| e.overused).count();
    assert!(over >= 1 && over * 10 <= t.len(), "{over} overused pairings");
    let non_latin = t.iter().filter(|e| e.scripts.iter().any(|s| matches!(*s, "japanese" | "korean" | "chinese-simplified" | "arabic" | "hebrew" | "devanagari" | "thai"))).count();
    assert!(non_latin >= 15, "{non_latin} non-Latin pairings");
}

#[test]
fn every_surface_has_options_in_every_role() {
    let a = ARCHETYPES();
    for s in [S::ProductUi, S::Commerce, S::Venue, S::Showcase, S::Editorial, S::DevTool, S::General] {
        for role in ["A", "B", "C"] {
            let n = a.iter().filter(|x| x.roles.contains(&role) && fits(x, s) && x.needs.is_empty()).count();
            assert!(n >= 2, "{s:?} role {role}: {n}");
        }
    }
    let splits = a.iter().filter(|x| x.composition == "split").count();
    assert!(splits * 5 <= a.len(), "{splits} split archetypes");
}

#[test]
fn dark_plus_neon_is_dealt_only_when_asked() {
    let neon = crate::intelligence::palette("x", "#0E0B14", "#18131F", "#F5F0FA", "#ADA2BD", "#FE3181", "#3FE3F0", 0.0, "");
    let teal = crate::intelligence::palette("y", "#123E3E", "#18484A", "#F5F0FA", "#ADA2BD", "#FE3181", "#3FE3F0", 0.0, "");
    assert!(neon_on_black(&neon));
    assert!(!neon_on_black(&teal));
    // Dusky purples, console greys and mint-on-black are the same reflex; gold on black is not.
    for (bg, accent) in [("#2A1B3D", "#FF71CE"), ("#0D1117", "#3FB950"), ("#0F1A17", "#7BDCB5")] {
        assert!(neon_on_black(&crate::intelligence::palette("z", bg, bg, "#F5F0FA", "#ADA2BD", accent, "#3FE3F0", 0.0, "")), "{bg}/{accent}");
    }
    assert!(!neon_on_black(&crate::intelligence::palette("g", "#0B0B0C", "#151517", "#F5F0FA", "#ADA2BD", "#C9A227", "#8FB7C9", 0.0, "")));
    let q = "Independent web design and development agency that builds animated marketing sites. Heavy, bold scroll-driven animation, big kinetic type.";
    let sig = brief_signals(q);
    for v in 0..6 {
        let dealt = deal(&sig, q, v, None);
        assert!(dealt.iter().filter(|d| neon_on_black(&d.palette)).count() <= 1, "{q} v{v} dealt dark + neon more than once");
        // A face written for another trade must not carry that trade's mood into the brief.
        for d in &dealt {
            assert!(!d.typography.mood.to_lowercase().contains("harbour"), "v{v}: {}", d.typography.mood);
        }
    }
}

#[test]
fn law_and_money_briefs_never_get_navy_and_gold() {
    let navy = crate::intelligence::palette("x", "#1B2A4A", "#22335A", "#F4F2EC", "#C8CCDA", "#C9A54C", "#8FB7C9", 0.0, "");
    assert!(navy_gold(&navy));
    for q in ["law firm in Boston", "family law attorney", "private bank for founders"] {
        let sig = brief_signals(q);
        for v in 0..8 {
            for d in deal(&sig, q, v, None) {
                assert!(!navy_gold(&d.palette), "{q} v{v} dealt navy-and-gold {}", d.palette.id);
            }
        }
    }
}

#[test]
fn later_variants_stay_on_the_brief_s_own_subject() {
    for q in ["law firm in Boston", "kids museum with hands-on exhibits", "seafood restaurant by the harbour", "funeral home in Leeds"] {
        let sig = brief_signals(q);
        let subject = read_subject(q, &sig);
        let has = |t: &str| {
            tag_weight(&sig, &subject, t) > 0 || (t == "data" && sig.surface == S::ProductUi) || (t == "code" && sig.surface == S::DevTool)
        };
        for v in 0..7 {
            for d in deal(&sig, q, v, None) {
                let te = d.type_entry.unwrap();
                let pe = palette_pool().iter().find(|e| e.palette.id == d.palette.id).unwrap();
                assert!(!foreign(te.tags, &has), "{q} v{v}: type {} carries {:?}", te.pairing.id, te.tags);
                assert!(!foreign(pe.tags, &has), "{q} v{v}: palette {} carries {:?}", pe.palette.id, pe.tags);
            }
        }
    }
}

#[test]
fn subject_is_read_from_the_brief() {
    let s = |q: &str| read_subject(q, &brief_signals(q));
    assert_eq!(s("law firm").phrase, "law firm");
    assert_eq!(s("law firm").action, "book a first consultation");
    assert_eq!(s("Marketing site for an open-source MCP server").phrase, "open-source mcp server");
    assert_eq!(s("seafood restaurant on the harbour with a daily catch menu").phrase, "seafood restaurant");
    let kids = s("kids' museum");
    assert!(kids.objects.iter().any(|o| o.contains("plinth")), "{:?}", kids.objects);
    let dash = s("bakery sales dashboard");
    assert_eq!(dash.objects[0], "today's numbers");
    assert!(dash.objects.contains(&"today's numbers".to_string()), "{:?}", dash.objects);
    let unknown = s("A detailed start page");
    assert!(!unknown.from_trades);
    assert_eq!(unknown.phrase, "detailed start");
    // The head of the phrase leads the action; the modifier still adds its objects and tags.
    let venue = s("vineyard wedding venue");
    assert_eq!(venue.action, "check a date and book a viewing");
    assert!(venue.tags.contains(&"celebration") && venue.tags.contains(&"earth"), "{:?}", venue.tags);
    assert_eq!(s("funeral home").phrase, "funeral home");
    assert_eq!(s("bakery home page").phrase, "bakery");
    assert_eq!(s("tattoo studio in Osaka").action, "book a consultation with an artist");
}

const EIGHT: [&str; 8] = [
    "funeral home",
    "tattoo studio in Osaka",
    "municipal water utility",
    "vineyard wedding venue",
    "kids coding camp",
    "B2B invoicing API",
    "plumbing contractor",
    "independent bookshop",
];

fn arch_of(title: &str) -> &'static Archetype {
    let name = title.rsplit_once('(').map(|(_, n)| n.trim_end_matches(')')).unwrap_or("");
    ARCHETYPES().iter().find(|a| a.name == name).unwrap_or_else(|| panic!("no archetype for {title}"))
}

#[test]
fn eight_briefs_get_distinct_on_tone_directions() {
    let recs: Vec<_> = EIGHT.iter().map(|q| recommend_design(q, OperatingMode::NewWebsite, None, None)).collect();
    let (mut pals, mut fonts, mut layouts) = (HashSet::new(), HashSet::new(), HashSet::new());
    for (q, r) in EIGHT.iter().zip(&recs) {
        assert_eq!(r.directions.len(), 3, "{q}");
        let a = &r.directions[0];
        assert!(pals.insert(a.palette.id.clone()), "{q}: Direction A palette {} repeats", a.palette.id);
        assert!(fonts.insert(a.typography.display_font.clone()), "{q}: Direction A display {} repeats", a.typography.display_font);
        assert!(layouts.insert(arch_of(&a.title).key), "{q}: Direction A layout {} repeats", a.title);
        let archs: Vec<_> = r.directions.iter().map(|d| arch_of(&d.title)).collect();
        let fams: HashSet<_> = archs.iter().map(|x| x.look_family).collect();
        assert_eq!(fams.len(), 3, "{q}: look families {:?}", archs.iter().map(|x| x.look_family).collect::<Vec<_>>());
        assert!(archs.iter().any(|x| x.hero_material != "giant-type"), "{q}: all heroes are type-only");
    }
    // Solemn: no saturated colour field and no poster for a funeral home.
    for d in &recs[0].directions {
        assert_ne!(Tone::of(&d.palette), Tone::Field, "funeral got colour field {}", d.palette.id);
        assert_ne!(arch_of(&d.title).composition, "poster", "funeral got a poster: {}", d.title);
    }
    // Osaka asks for Japanese coverage on every dealt pairing.
    let jp = recs[1]
        .directions
        .iter()
        .filter(|d| type_pool().iter().any(|e| e.pairing.display_font == d.typography.display_font && e.scripts.contains(&"japanese")))
        .count();
    assert!(jp >= 2, "Osaka: {jp} Japanese-covering pairings");
}

#[test]
fn directions_vary_corners_and_axes() {
    for q in EIGHT {
        let sig = brief_signals(q);
        let ds = deal(&sig, q, 0, None);
        let radii: HashSet<_> = ds.iter().map(|d| d.radius).collect();
        assert_eq!(radii.len(), 3, "{q}: corners {:?}", ds.iter().map(|d| d.radius).collect::<Vec<_>>());
        if !(sig.niche_tags.iter().any(|t| t == "play") || read_subject(q, &sig).tags.contains(&"play")) {
            assert!(!ds.iter().any(|d| d.radius == "round-play") || q.contains("camp"), "{q}: pill corners");
        }
        for w in ds.windows(2) {
            let ax = |d: &Dealt| {
                let pe = palette_pool().iter().find(|e| e.palette.id == d.palette.id).unwrap();
                (d.tone, pe.accent_hue, pe.warmth, d.type_entry.unwrap().display_style)
            };
            assert_ne!(ax(&w[0]), ax(&w[1]), "{q}: consecutive directions share every axis");
        }
    }
}

#[test]
fn brief_colours_and_fonts_lead() {
    let q = "bakery site, brand colours #111111 and #E4572E, fonts Space Grotesk and Literata";
    let sig = brief_signals(q);
    assert!(!sig.brief_colors.is_empty(), "{:?}", sig.brief_colors);
    for d in deal(&sig, q, 0, None) {
        assert_eq!(d.palette.background, "#111111", "{}", d.palette.id);
        assert_eq!(d.palette.primary_accent.to_uppercase(), "#E4572E");
        assert!(wcag_contrast(&d.palette.foreground, &d.palette.background).unwrap() >= 4.5);
        assert!(wcag_contrast(&d.palette.muted, &d.palette.surface).unwrap() >= 4.5);
        if !sig.brief_fonts.is_empty() {
            let named = [d.typography.display_font.as_str(), d.typography.body_font.as_str()];
            assert!(named.iter().any(|f| sig.brief_fonts.iter().any(|b| b == f)), "{named:?} vs {:?}", sig.brief_fonts);
        }
    }
}

#[test]
#[ignore = "prints the eight-brief deal for inspection"]
fn dbg_eight() {
    for q in EIGHT {
        let sig = brief_signals(q);
        let sub = read_subject(q, &sig);
        let r = recommend_design(q, OperatingMode::NewWebsite, None, None);
        eprintln!("== {q} surface {:?} niche {:?} moods {:?} subj {:?} ind {} score {}", sig.surface, sig.niche_tags, sig.moods, sub.tags, r.matched_industry.slug, r.match_score);
        for d in &r.directions {
            let a = arch_of(&d.title);
            eprintln!("  {} | {} / {} | {} | {} {} {}", a.key, d.typography.display_font, d.typography.body_font, d.palette.id, a.look_family, a.hero_material, a.composition);
        }
    }
}

#[test]
fn unmatched_briefs_spread_over_the_pool() {
    let qs = ["quiet page for a zither collective", "page for a glassblowing co-op", "falconry centre", "lighthouse keepers' society", "kite flying meetup"];
    let picks: HashSet<_> = qs
        .iter()
        .map(|q| {
            let sig = brief_signals(q);
            deal(&sig, q, 0, None)[1].palette.id.clone()
        })
        .collect();
    assert!(picks.len() >= 4, "{picks:?}");
}

fn novelty(face: &str) -> bool {
    type_pool().iter().any(|e| e.pairing.display_font == face && e.tags.contains(&"novelty"))
}

#[test]
fn sober_briefs_never_get_display_gimmicks() {
    let sober = ["B2B invoicing API", "accounting firm", "payments platform for banks", "law firm", "payroll SaaS for mid-size companies", "developer API for identity checks"];
    for q in sober {
        for v in 0..4 {
            for d in recommend_design_variant(q, OperatingMode::NewWebsite, None, None, v).directions {
                assert!(!novelty(&d.typography.display_font), "{q} v{v}: novelty face {}", d.typography.display_font);
            }
        }
    }
    // The register still reaches briefs that ask for it.
    let q = "retro arcade bar with neon signs";
    let any = (0..4).flat_map(|v| recommend_design_variant(q, OperatingMode::NewWebsite, None, None, v).directions).any(|d| novelty(&d.typography.display_font));
    assert!(any, "{q}: no novelty face in four variants");
}

#[test]
fn solemn_briefs_get_human_presence_not_cold_kit() {
    for q in ["funeral home", "hospice care", "cemetery and memorial gardens", "law firm", "bereavement counselling"] {
        for v in 0..3 {
            for d in recommend_design_variant(q, OperatingMode::NewWebsite, None, None, v).directions {
                let a = arch_of(&d.title);
                assert!(!COLD_HEROES.contains(&a.hero_material), "{q} v{v}: cold hero {} in {}", a.hero_material, d.title);
            }
        }
    }
}

#[test]
fn unrelated_briefs_do_not_share_type_families() {
    assert_eq!(family("Libre Caslon Display"), family("Libre Caslon Text"));
    assert_eq!(family("Sofia Sans Condensed"), "sofia sans");
    assert_eq!(family("Archivo Narrow"), "archivo");
    let dirs: Vec<_> = EIGHT.iter().flat_map(|q| recommend_design(q, OperatingMode::NewWebsite, None, None).directions).collect();
    let display: Vec<String> = dirs.iter().map(|d| family(&d.typography.display_font)).collect();
    let distinct: HashSet<_> = display.iter().collect();
    assert!(display.len() == distinct.len(), "display families repeat: {display:?}");
    let mut body: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for d in &dirs {
        *body.entry(family(&d.typography.body_font)).or_default() += 1;
    }
    let worst = body.iter().max_by_key(|(_, n)| **n).unwrap();
    assert!(*worst.1 <= 2, "body family {} used {} times over the eight briefs: {body:?}", worst.0, worst.1);
}

#[test]
fn text_faces_never_set_display_type() {
    for e in type_pool() {
        assert!(display_capable(&e.pairing.display_font, &e.pairing.body_font), "{}: text face {} as display", e.pairing.id, e.pairing.display_font);
    }
    // Profiles can carry a text face (education uses Lora); the deal replaces it.
    for q in EIGHT.iter().copied().chain(["online course platform for a university", "law firm", "independent bookshop"]) {
        for v in 0..3 {
            for d in recommend_design_variant(q, OperatingMode::NewWebsite, None, None, v).directions {
                assert!(display_capable(&d.typography.display_font, &d.typography.body_font), "{q} v{v}: text face {} as display", d.typography.display_font);
            }
        }
    }
}

#[test]
fn task_first_briefs_skip_posters_and_placeless_maps() {
    let is_poster = |a: &Archetype| a.composition == "poster" || a.key == "manifesto";
    for q in ["B2B invoicing API", "accounting firm", "municipal water utility", "payroll SaaS for mid-size companies", "plumbing contractor", "admin dashboard for a logistics company"] {
        for v in 0..4 {
            for d in recommend_design_variant(q, OperatingMode::NewWebsite, None, None, v).directions {
                assert!(!is_poster(arch_of(&d.title)), "{q} v{v}: poster layout {}", d.title);
            }
        }
    }
    for q in ["funeral home", "B2B invoicing API", "accounting firm", "independent bookshop", "payroll SaaS for mid-size companies"] {
        for v in 0..4 {
            for d in recommend_design_variant(q, OperatingMode::NewWebsite, None, None, v).directions {
                assert_ne!(arch_of(&d.title).hero_material, "map", "{q} v{v}: map hero with no place: {}", d.title);
            }
        }
    }
    // The rules give way when the brief asks: a bold campaign can get a poster, a venue its map.
    let poster = (0..4).flat_map(|v| recommend_design_variant("bold editorial campaign for a fintech launch", OperatingMode::NewWebsite, None, None, v).directions).any(|d| is_poster(arch_of(&d.title)));
    assert!(poster, "bold campaign never got a poster");
    let map = (0..4).flat_map(|v| recommend_design_variant("vineyard wedding venue", OperatingMode::NewWebsite, None, None, v).directions).any(|d| arch_of(&d.title).hero_material == "map");
    assert!(map, "venue never got a map");
}

#[test]
fn every_face_is_free_to_use_and_mono_links_stand_alone() {
    let allowed = ["SIL Open Font License 1.1", "ITF Free Font License (Fontshare)", "Apache-2.0 and SIL OFL 1.1"];
    for e in type_pool() {
        let p = &e.pairing;
        assert!(allowed.contains(&e.licence), "{}: licence {}", p.id, e.licence);
        for f in [&p.display_font, &p.body_font, &p.mono_font] {
            assert!(!is_paid_face(f), "{}: commercial face {f}", p.id);
        }
        assert!(!p.google_fonts_url.contains(' '), "{}: joined url {}", p.id, p.google_fonts_url);
        if e.source == "fontshare" {
            assert!(p.google_fonts_url.starts_with("https://api.fontshare.com/v2/css?"), "{}", p.id);
            let mono = p.mono_font.replace(' ', "+");
            assert!(!p.google_fonts_url.contains(&mono), "{}: mono inside fontshare url", p.id);
            assert!(e.mono_url.starts_with("https://fonts.googleapis.com/css2?family=") && e.mono_url.contains(&mono), "{}: mono_url {}", p.id, e.mono_url);
        } else {
            assert!(e.mono_url.is_empty(), "{}: google pairing carries mono_url", p.id);
        }
    }
}

#[test]
fn paid_brief_fonts_are_never_linked_to_a_cdn() {
    let q = "design studio portfolio, fonts Neue Montreal and Inter";
    let sig = brief_signals(q);
    assert!(sig.brief_fonts.iter().any(|f| f == "Neue Montreal"), "{:?}", sig.brief_fonts);
    let mut noted = false;
    for d in deal(&sig, q, 0, None) {
        assert!(!d.typography.google_fonts_url.to_lowercase().contains("neue"), "{}", d.typography.google_fonts_url);
        assert!(!d.extra_font_url.as_deref().unwrap_or("").to_lowercase().contains("neue"));
        noted |= d.brand.iter().any(|b| b.contains("Neue Montreal") && b.contains("self-host"));
    }
    assert!(noted, "no self-host note for Neue Montreal");
}

#[test]
fn coordinated_modifiers_keep_the_head_noun() {
    let p = subject_phrase("Independent web design and development agency (small studio, 8 people) that builds sites");
    assert!(p.ends_with("agency"), "{p}");
    assert!(p.contains("web design"), "{p}");
    assert!(!p.contains("small"), "{p}");
    assert_eq!(subject_phrase("design a website for my bakery"), "bakery");
    assert!(subject_phrase("black and white photography studio").ends_with("white photography studio"));
}

#[test]
fn trades_read_the_subject_not_the_modifier_or_audience() {
    let subj = |q: &str| read_subject(q, &brief_signals(q));
    // A repair modifier keeps the watch objects and takes the repair action.
    let w = subj("watch repair atelier");
    assert!(w.objects.iter().any(|o| o.contains("movement")), "{:?}", w.objects);
    assert!(w.action.contains("repair"), "{}", w.action);
    assert!(!w.tags.contains(&"industrial"), "{:?}", w.tags);
    // SaaS serving a trade shows that trade's objects.
    let f = subj("B2B SaaS for freight brokers");
    assert!(f.objects.iter().any(|o| o.contains("load")), "{:?}", f.objects);
    // A charity is civic, not clinical care.
    assert!(!subj("food bank charity").tags.contains(&"care"));
    // "for startups" names the audience, not the trade.
    let r = crate::intelligence::recommend_design_variant("law firm for startups", crate::intelligence::OperatingMode::NewWebsite, None, None, 0);
    assert!(!r.matched_industry.slug.contains("saas") && !r.matched_industry.name.to_lowercase().contains("saas"), "{}", r.matched_industry.name);
}
