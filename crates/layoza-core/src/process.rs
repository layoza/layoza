// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
//! Design-process memory and gates that live in the project's `.layoza/` folder.
//!
//! - Gate files: a checkpoint counts only once its file exists and says what it must
//!   (`direction-approved.md`, `brand-spec.md`, `product-facts.md`). "continue" never skips a gate.
//! - The look stamp: one comment line at the top of the main CSS naming the direction, structure,
//!   hero and the pre-build self-critique scores. The ledger rebuilds history from stamps when
//!   `.layoza/looks.jsonl` is missing.
//! - Rotation: a plain-words statement of what recent runs used, returned with every deal.
//! - Taste: the user's own words about taste, kept locally in `.layoza/taste.md`.
//! - Structural fingerprint: how similar a redesign's page skeleton is to the baseline's.

use crate::baseline::Look;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

pub const DIRECTION_APPROVED: &str = ".layoza/direction-approved.md";
pub const BRAND_SPEC: &str = ".layoza/brand-spec.md";
pub const PRODUCT_FACTS: &str = ".layoza/product-facts.md";
pub const TASTE: &str = ".layoza/taste.md";
const DIRECTION_LOG: &str = ".layoza/direction.md";

/// Self-critique axes in stamp order (1-5 each; any score below 3 forces a revision).
pub const CRITIQUE_AXES: [&str; 6] = ["philosophy", "hierarchy", "execution", "specificity", "restraint", "variety"];

/// Similarity of page skeletons at or above which a redesign counts as the same structure.
pub const REDESIGN_MAX_SIMILARITY: f64 = 0.8;

// ---------------------------------------------------------------------------------------------
// Brief reading

fn lc_words(q: &str) -> Vec<String> {
    q.to_lowercase().split(|c: char| !c.is_alphanumeric() && c != '-').filter(|w| !w.is_empty()).map(str::to_string).collect()
}

/// True when the brief asks for one component rather than a page or site: page-level steps
/// (hero, three directions) are skipped; tokens, states, accessibility and motion rules stay.
pub fn is_component_brief(q: &str) -> bool {
    let w = lc_words(q);
    let has = |x: &str| w.iter().any(|t| t == x || t.trim_end_matches('s') == x);
    let page = ["page", "site", "website", "landing", "homepage", "home", "portfolio", "app", "store", "shop", "dashboard", "microsite", "blog"];
    if page.iter().any(|p| has(p)) {
        return false;
    }
    let parts = [
        "component", "button", "card", "navbar", "nav", "modal", "dialog", "form", "widget", "toggle", "dropdown", "footer",
        "header", "tooltip", "tabs", "accordion", "carousel", "badge", "table", "input", "slider", "menu", "pagination", "toast",
        "sidebar", "stepper", "datepicker", "avatar", "breadcrumb",
    ];
    parts.iter().any(|p| has(p))
}

/// The user asked for the same look as before: keep it and change the knobs instead of rotating.
pub fn same_look_requested(q: &str) -> bool {
    let l = q.to_lowercase();
    ["same look", "same style", "same design", "like last time", "same as before", "same as last", "keep the look", "again with the same"]
        .iter()
        .any(|p| l.contains(p))
}

/// Why the brief needs a made-to-measure route instead of a library pick, if it does.
pub fn custom_route_reason(q: &str, match_score: usize, brief_objects: usize) -> Option<String> {
    let l = q.to_lowercase();
    let asks = ["bespoke", "made-to-measure", "made to measure", "one-off", "unlike any", "never seen", "not a template", "no template", "our own look", "invent", "from scratch", "custom look", "one of a kind"];
    if let Some(p) = asks.iter().find(|p| l.contains(*p)) {
        return Some(format!("the brief asks for it (\"{}\")", p));
    }
    let words = q.split_whitespace().count();
    if match_score == 0 && brief_objects >= 3 && words >= 25 {
        return Some("no library trade fits and the brief describes its own world in detail".into());
    }
    None
}

/// Made-to-measure protocol for briefs no library pick fits.
pub fn custom_route_protocol(reason: &str) -> String {
    format!(
        "CUSTOM ROUTE ({reason}): treat the library directions as sketches, not picks. Derive the look from the brief's objects: \
         (1) palette from photographs or physical materials of 2-3 named objects (sample the real colours, then check WCAG pairs); \
         (2) type from the trade's own lettering (signage, labels, documents, instruments) and a readable body face; \
         (3) layout from how the objects are arranged in real life (a ledger, a shelf, a route, a timetable); \
         (4) one hero material the subject actually has. Record each derivation in .layoza/direction.md so the reviewer can check it."
    )
}

/// True when the brief names things that can be wrong if assumed: versions, years, launches, events.
pub fn names_checkable_facts(q: &str) -> bool {
    let w = lc_words(q);
    let versioned = w.iter().any(|t| {
        let digits = t.chars().filter(|c| c.is_ascii_digit()).count();
        (t.starts_with('v') && digits > 0 && digits + 1 == t.chars().count())
            || (digits == 4 && t.len() == 4 && (t.starts_with("19") || t.starts_with("20")))
            || (digits > 0
                && t.chars().next().is_some_and(|c| c.is_alphabetic())
                && !t.chars().all(|c| c.is_ascii_hexdigit())
                && t.len() <= 12)
    });
    versioned || w.iter().any(|t| matches!(t.as_str(), "launch" | "release" | "conference" | "festival" | "summit" | "tour" | "edition" | "keynote" | "ceo" | "founder"))
}

// ---------------------------------------------------------------------------------------------
// Hero material

/// What a direction's hero is made of, read from its hero mechanic.
pub fn hero_material(id: &str, name: &str, composition: &str) -> &'static str {
    let t = format!("{} {} {}", id, name, composition).to_lowercase();
    // The catalogue's own `hero_material` code (passed at the start of `name`) wins over keywords.
    for (code, label) in [
        ("real-photo", "real photo or footage of the subject"),
        ("video", "real photo or footage of the subject"),
        ("generated-image", "generated image of the subject (label it as illustration)"),
        ("data-image", "data as image (the trade's real numbers drawn large)"),
        ("procedural", "procedural texture (SVG or canvas drawn from the subject)"),
        ("physical-object", "physical object, photographed or modelled"),
        ("giant-type", "giant type"),
    ] {
        if name.to_lowercase().starts_with(code) {
            return label;
        }
    }
    let any = |ks: &[&str]| ks.iter().any(|k| t.contains(k));
    if any(&["photo", "image", "portrait", "gallery", "film", "video", "reel", "lens", "camera"]) {
        "real photo or footage of the subject"
    } else if any(&["chart", "data", "ledger", "board", "table", "map", "timetable", "gauge", "meter", "graph", "count", "tally", "score"]) {
        "data as image (the trade's real numbers drawn large)"
    } else if any(&["canvas", "texture", "grain", "noise", "shader", "particle", "wave", "field", "generative", "pattern", "svg"]) {
        "procedural texture (SVG or canvas drawn from the subject)"
    } else if any(&["object", "product", "specimen", "3d", "artifact", "artefact", "tool", "package", "label", "bottle", "jar"]) {
        "physical object, photographed or modelled"
    } else if any(&["type", "word", "letter", "headline", "title", "glyph", "marquee", "poster", "manifesto"]) {
        "giant type"
    } else {
        "real photo or footage of the subject"
    }
}

/// One material per direction, never three pure-typography heroes.
pub fn hero_materials(heroes: &[(String, String, String)]) -> Vec<String> {
    let mut out: Vec<String> = heroes.iter().map(|(i, n, c)| hero_material(i, n, c).to_string()).collect();
    if out.len() >= 2 && out.iter().all(|m| m == "giant type") {
        let n = out.len();
        out[n - 1] = "real photo or footage of the subject".into();
        if n >= 3 {
            out[n - 2] = "procedural texture (SVG or canvas drawn from the subject)".into();
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Look stamp

/// One parsed look stamp (`/* layoza · direction: .. · structure: .. · hero: .. · critique: .. */`).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Stamp {
    pub file: String,
    pub direction: Option<String>,
    pub structure: Option<String>,
    pub hero: Option<String>,
    pub palette: Option<String>,
    pub typography: Option<String>,
    pub scope: Option<String>,
    /// Self-critique scores in [`CRITIQUE_AXES`] order, when the stamp carries them.
    pub critique: Vec<(String, u8)>,
}

/// The stamp line an agent writes at the top of the main CSS file.
pub fn stamp_line(direction: &str, structure: &str, hero: &str, palette: &str, typography: &str, scores: &[u8; 6]) -> String {
    let c: Vec<String> = CRITIQUE_AXES.iter().zip(scores).map(|(a, s)| format!("{}{}", &a[..1].to_uppercase(), s)).collect();
    format!(
        "/* layoza · direction: {direction} · structure: {structure} · hero: {hero} · palette: {palette} · type: {typography} · critique: {} */",
        c.join(" ")
    )
}

/// Parses a stamp line, or `None` when the line is not a Layoza stamp.
pub fn parse_stamp(line: &str, file: &str) -> Option<Stamp> {
    let body = line.trim().strip_prefix("/*")?.trim_end_matches("*/").trim();
    let rest = body.strip_prefix("layoza")?;
    let mut s = Stamp { file: file.to_string(), ..Default::default() };
    for part in rest.split('·').map(str::trim).filter(|p| !p.is_empty()) {
        let Some((k, v)) = part.split_once(':') else { continue };
        let v = v.trim().to_string();
        match k.trim() {
            "direction" => s.direction = Some(v),
            "structure" => s.structure = Some(v),
            "hero" => s.hero = Some(v),
            "palette" => s.palette = Some(v),
            "type" => s.typography = Some(v),
            "scope" => s.scope = Some(v),
            "critique" => {
                for tok in v.split_whitespace() {
                    let (letter, num) = tok.split_at(tok.char_indices().nth(1).map(|(i, _)| i).unwrap_or(tok.len()));
                    if let (Some(axis), Ok(n)) = (CRITIQUE_AXES.iter().find(|a| a[..1].eq_ignore_ascii_case(letter)), num.parse::<u8>()) {
                        s.critique.push((axis.to_string(), n));
                    }
                }
            }
            _ => {}
        }
    }
    Some(s)
}

/// Stamps found as the first non-empty line of CSS files (and inline `<style>` openers) under `root`.
pub fn find_stamps(root: &Path) -> Vec<Stamp> {
    let mut out = Vec::new();
    walk(root, root, 0, &mut out);
    out.sort_by(|a, b| a.file.cmp(&b.file));
    out
}

fn walk(root: &Path, dir: &Path, depth: usize, out: &mut Vec<Stamp>) {
    if depth > 4 || out.len() > 50 {
        return;
    }
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if p.is_dir() {
            if !name.starts_with('.') && !matches!(name.as_str(), "node_modules" | "target" | "dist" | "build" | "vendor" | "qa") {
                walk(root, &p, depth + 1, out);
            }
            continue;
        }
        let css = name.ends_with(".css") || name.ends_with(".scss");
        if !(css || name.ends_with(".html") || name.ends_with(".astro") || name.ends_with(".vue") || name.ends_with(".svelte")) {
            continue;
        }
        let Ok(bytes) = fs::read(&p) else { continue };
        let text = String::from_utf8_lossy(&bytes);
        let rel = p.strip_prefix(root).unwrap_or(&p).to_string_lossy().replace('\\', "/");
        let line = if css {
            text.lines().find(|l| !l.trim().is_empty()).map(str::to_string)
        } else {
            text.find("<style").and_then(|i| text[i..].find('>').map(|j| i + j + 1)).and_then(|k| text[k..].lines().find(|l| !l.trim().is_empty()).map(str::to_string))
        };
        if let Some(s) = line.and_then(|l| parse_stamp(&l, &rel)) {
            out.push(s);
        }
    }
}

/// Recent looks from `.layoza/looks.jsonl`, or rebuilt from CSS stamps when the ledger is missing.
pub fn recent_looks_or_stamps(root: &Path, runs: usize) -> Vec<Look> {
    let looks = crate::baseline::recent_looks(root, runs);
    if !looks.is_empty() || root.join(crate::baseline::LOOK_LEDGER).exists() {
        return looks;
    }
    find_stamps(root)
        .into_iter()
        .filter(|s| s.scope.as_deref() != Some("component"))
        .take(runs)
        .map(|s| Look {
            run: format!("stamp:{}", s.file),
            source: "stamp".into(),
            palette: s.palette,
            typography: s.typography,
            hero: s.hero.clone(),
            layout: s.structure,
            mechanics: s.hero.into_iter().collect(),
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Rotation statement

/// Plain-words history of the recent runs and what this run picks from.
pub fn rotation_statement(recent: &[Look], picking: &[String], same_look: bool) -> String {
    if same_look {
        return "The user asked for the same look again: the structure stays, and only the knobs change (palette shade, type weight, density, motion level).".into();
    }
    if recent.is_empty() {
        return format!("First recorded run for this project; picking from {{{}}}.", picking.join(", "));
    }
    let runs: HashSet<&str> = recent.iter().map(|l| l.run.as_str()).collect();
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for l in recent {
        for v in [l.hero.as_ref(), l.layout.as_ref(), l.palette.as_ref()].into_iter().flatten() {
            *counts.entry(v.clone()).or_default() += 1;
        }
    }
    let mut top: Vec<(String, usize)> = counts.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let used: Vec<String> = top
        .iter()
        .take(4)
        .map(|(k, n)| match n {
            1 => format!("{k} once"),
            2 => format!("{k} twice"),
            n => format!("{k} {n} times"),
        })
        .collect();
    format!("The last {} runs used {}; this run picks from {{{}}}.", runs.len(), used.join(", "), picking.join(", "))
}

/// Fills `rotation` and `taste` on a recommendation from the project's memory, and adds guidance
/// so the agent surfaces both to the user.
pub fn apply_project_memory(rec: &mut crate::intelligence::DesignRecommendation, root: Option<&Path>, recent: &[Look]) {
    let Some(root) = root else { return };
    let picking: Vec<String> = rec
        .directions
        .iter()
        .map(|d| d.mechanics.iter().find(|m| m.kind == crate::mechanics::MechanicKind::Hero).map(|m| m.id.clone()).unwrap_or_else(|| d.style_family.clone()))
        .collect();
    let rot = rotation_statement(recent, &picking, same_look_requested(&rec.query));
    rec.workflow_guidance.insert(0, format!("ROTATION: tell the user in one line before dealing: {rot}"));
    rec.rotation = Some(rot);
    rec.taste = read_taste(root);
    if !rec.taste.is_empty() {
        rec.workflow_guidance.insert(0, format!("TASTE (the user's own words, from .layoza/taste.md; they outrank library defaults): {}", rec.taste.join(" | ")));
    }
}

/// The user's recorded taste lines from `.layoza/taste.md` (word for word, with scope).
pub fn read_taste(root: &Path) -> Vec<String> {
    let Ok(raw) = fs::read(root.join(TASTE)) else { return Vec::new() };
    String::from_utf8_lossy(&raw)
        .lines()
        .filter_map(|l| l.trim().strip_prefix("- ").map(|s| crate::baseline::sanitize_untrusted_content(s)))
        .filter(|s| !s.is_empty())
        .take(20)
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Gate files

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GateFinding {
    pub rule: String,
    pub file: String,
    pub message: String,
    pub fix: String,
}

fn gate(rule: &str, file: &str, message: &str, fix: &str) -> GateFinding {
    GateFinding { rule: rule.into(), file: file.into(), message: message.into(), fix: fix.into() }
}

fn read_text(p: &Path) -> Option<String> {
    fs::read(p).ok().map(|b| String::from_utf8_lossy(&b).into_owned())
}

fn field<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    text.lines().find_map(|l| l.trim().strip_prefix(key)).map(str::trim).filter(|v| !v.is_empty())
}

/// Words that move to the next step but never pick a direction or skip a gate.
fn is_continue_only(said: &str) -> bool {
    let s: String = said.to_lowercase().chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect();
    let s = s.trim();
    s.is_empty() || ["continue", "go on", "go ahead", "ok", "okay", "next", "proceed", "yes", "sure", "keep going", "carry on", "go"].contains(&s)
}

fn said_skip(said: &str) -> bool {
    let s = said.to_lowercase();
    ["skip", "no need", "don't show", "dont show", "just build", "you choose", "your call", "pick for me", "you pick", "no options"].iter().any(|k| s.contains(k))
}

/// Checks the process gate files. Gates apply only once the process has started: a direction log
/// (`.layoza/direction.md`) or a page stamp in the CSS. Returns one finding per unmet gate.
pub fn check_gates(root: &Path) -> Vec<GateFinding> {
    let mut out = Vec::new();
    let direction_log = read_text(&root.join(DIRECTION_LOG));
    let stamps = find_stamps(root);
    let page_stamp = stamps.iter().any(|s| s.scope.as_deref() != Some("component"));

    // Self-critique: any axis below 3 forces a revision before QA.
    for s in &stamps {
        let low: Vec<String> = s.critique.iter().filter(|(_, n)| *n < 3).map(|(a, n)| format!("{a} {n}")).collect();
        if !low.is_empty() {
            out.push(gate(
                "SELF-CRITIQUE-BELOW-3",
                &s.file,
                &format!("The pre-build self-critique scored {} (below 3). Revise before QA.", low.join(", ")),
                "Revise the weak axes, re-score honestly and update the stamp's critique.",
            ));
        }
    }

    if direction_log.is_some() || page_stamp {
        match read_text(&root.join(DIRECTION_APPROVED)) {
            None => out.push(gate(
                "GATE-DIRECTION-NOT-APPROVED",
                DIRECTION_APPROVED,
                "No approval file: the build started without a recorded direction choice.",
                "Show the directions, then write SHOWN:, SCREENSHOTS: and USER SAID: \"<their words>\" to .layoza/direction-approved.md.",
            )),
            Some(t) => {
                let said = field(&t, "USER SAID:").unwrap_or("").trim_matches('"').trim();
                let skipped = field(&t, "SKIPPED:").is_some();
                if said.is_empty() {
                    out.push(gate("GATE-DIRECTION-NO-USER-WORDS", DIRECTION_APPROVED, "The approval has no USER SAID: line with the user's own words.", "Quote the user's reply word for word on a USER SAID: line."));
                } else if is_continue_only(said) {
                    out.push(gate(
                        "GATE-CONTINUE-IS-NOT-A-PICK",
                        DIRECTION_APPROVED,
                        &format!("\"{}\" moves to the next step; it neither picks a direction nor skips this gate.", said),
                        "Ask the user which direction they want (or whether to skip showing options) and record their reply.",
                    ));
                } else if skipped && !said_skip(said) {
                    out.push(gate("GATE-SKIP-WITHOUT-USER", DIRECTION_APPROVED, "SKIPPED: is written, but the quoted words do not ask to skip.", "Only the user can skip this gate; remove SKIPPED: and show the directions."));
                } else if !skipped {
                    let shown = field(&t, "SHOWN:").map(|v| v.split(';').filter(|x| !x.trim().is_empty()).count()).unwrap_or(0);
                    if shown < 3 || field(&t, "SCREENSHOTS:").is_none() {
                        out.push(gate("GATE-DIRECTIONS-NOT-SHOWN", DIRECTION_APPROVED, "The approval must list the three directions shown (SHOWN: a; b; c) and their SCREENSHOTS:.", "Record the three directions and the screenshot paths the user saw."));
                    }
                }
            }
        }
    }

    let log = direction_log.unwrap_or_default();
    let brand_named = field(&log, "BRAND:").is_some_and(|b| !b.eq_ignore_ascii_case("none"))
        || field(&log, "PURPOSE:").is_some_and(|p| p.to_lowercase().contains("existing"))
        || root.join(PRODUCT_FACTS).exists();
    if brand_named {
        match read_text(&root.join(BRAND_SPEC)) {
            None => out.push(gate("GATE-NO-BRAND-SPEC", BRAND_SPEC, "A real brand or product is involved but there is no brand spec.", "Inventory the assets first (logo, product images, UI screenshots), then the spec, in .layoza/brand-spec.md.")),
            Some(t) => {
                let assets_at = t.lines().position(|l| l.trim_start().starts_with("ASSETS:"));
                let none_ok = field(&t, "ASSETS:").is_some_and(|v| v.to_lowercase().contains("none") && v.to_lowercase().contains("user"));
                let items = assets_at.map(|i| t.lines().skip(i + 1).take_while(|l| l.trim_start().starts_with("- ")).count()).unwrap_or(0);
                let first_hex = t.lines().position(|l| l.contains('#') && l.split('#').nth(1).is_some_and(|h| h.chars().take(6).filter(|c| c.is_ascii_hexdigit()).count() >= 3));
                if assets_at.is_none() || (items == 0 && !none_ok) {
                    out.push(gate("GATE-BRAND-NO-ASSETS", BRAND_SPEC, "The brand spec lists no assets. Logo, product images and UI screenshots matter more than hex codes.", "Add an ASSETS: list (- logo: path, - product image: path, - ui screenshot: path), or `ASSETS: none (user confirmed)`."));
                } else if let (Some(a), Some(h)) = (assets_at, first_hex) {
                    if h < a {
                        out.push(gate("GATE-BRAND-SPEC-BEFORE-ASSETS", BRAND_SPEC, "Colour codes come before the asset inventory.", "Move the ASSETS: list above the colours and type."));
                    }
                }
            }
        }
    }

    if let Some(t) = read_text(&root.join(PRODUCT_FACTS)) {
        let unverified: Vec<&str> = t
            .lines()
            .filter_map(|l| l.trim().strip_prefix("- "))
            .filter(|l| {
                let x = l.to_lowercase();
                !(x.contains("source:") || x.contains("user confirmed") || x.contains("http"))
            })
            .collect();
        if !unverified.is_empty() {
            out.push(gate("GATE-FACT-UNVERIFIED", PRODUCT_FACTS, &format!("{} fact(s) have no source or user confirmation.", unverified.len()), "Add `source: <url>` or `user confirmed` to each fact, or remove it from the page."));
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Structural fingerprint

const STRUCT_TAGS: &[&str] = &["header", "nav", "main", "section", "article", "aside", "footer", "form", "table", "ul", "ol", "figure", "img", "video", "canvas", "svg", "h1", "h2", "h3", "dl", "blockquote", "details"];

/// Page skeleton: the sequence of structural tags in document order (text and classes ignored).
pub fn structural_fingerprint(html: &str) -> Vec<String> {
    let lower = html.to_lowercase();
    let mut out = Vec::new();
    let mut rest = lower.as_str();
    while let Some(i) = rest.find('<') {
        rest = &rest[i + 1..];
        let name: String = rest.chars().take_while(|c| c.is_ascii_alphanumeric()).collect();
        if STRUCT_TAGS.contains(&name.as_str()) {
            out.push(name);
        }
    }
    out
}

/// Jaccard similarity of 3-tag shingles (1.0 = same skeleton).
pub fn skeleton_similarity(a: &[String], b: &[String]) -> f64 {
    let sh = |v: &[String]| -> HashSet<String> {
        if v.len() < 3 {
            return v.iter().cloned().collect();
        }
        v.windows(3).map(|w| w.join(">")).collect()
    };
    let (x, y) = (sh(a), sh(b));
    if x.is_empty() && y.is_empty() {
        return 1.0;
    }
    x.intersection(&y).count() as f64 / x.union(&y).count() as f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureCheck {
    pub file: String,
    pub similarity: f64,
    pub same_structure: bool,
}

/// Compares each HTML-like file in a baseline with its current version. A redesign whose skeleton
/// is at least [`REDESIGN_MAX_SIMILARITY`] similar kept the old structural fingerprint.
pub fn redesign_structure_check(root: &Path, baseline_id: &str) -> Result<Vec<StructureCheck>, String> {
    let p = root.join(".layoza").join("baselines").join(format!("{}.json", crate::baseline::sanitize_id(baseline_id)));
    let raw = fs::read(&p).map_err(|e| format!("Cannot read baseline `{}`: {}", baseline_id, e))?;
    let snap: crate::baseline::BaselineSnapshot = serde_json::from_slice(&raw).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for (rel, old) in &snap.files {
        if !(rel.ends_with(".html") || rel.ends_with(".astro") || rel.ends_with(".vue") || rel.ends_with(".svelte") || rel.ends_with(".jsx") || rel.ends_with(".tsx") || rel.ends_with(".liquid") || rel.ends_with(".php")) {
            continue;
        }
        let Some(new) = read_text(&root.join(rel)) else { continue };
        let s = skeleton_similarity(&structural_fingerprint(old), &structural_fingerprint(&new));
        out.push(StructureCheck { file: rel.clone(), similarity: (s * 100.0).round() / 100.0, same_structure: s >= REDESIGN_MAX_SIMILARITY });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("layoza_process_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(d.join(".layoza")).unwrap();
        d
    }

    #[test]
    fn briefs_route_to_component_custom_and_fact_checks() {
        assert!(is_component_brief("a pricing table component with monthly and yearly toggle"));
        assert!(!is_component_brief("landing page for a bakery with a menu card"));
        assert!(custom_route_reason("a bespoke one-off site for our letterpress", 4, 1).is_some());
        assert!(custom_route_reason("bakery website", 9, 2).is_none());
        assert!(names_checkable_facts("launch page for the iPhone 17 event"));
        assert!(!names_checkable_facts("quiet funeral home in a small town"));
        assert!(same_look_requested("same look as last time but for the spring menu"));
    }

    #[test]
    fn three_directions_never_all_pure_type() {
        let h = |i: &str| (i.to_string(), "Giant headline".to_string(), "type".to_string());
        let m = hero_materials(&[h("a"), h("b"), h("c")]);
        assert_eq!(m.iter().filter(|x| *x == "giant type").count(), 1);
    }

    #[test]
    fn stamps_round_trip_rebuild_history_and_low_scores_gate() {
        let d = tmp("stamp");
        let line = stamp_line("B", "ledger-columns", "catch-board", "harbour-ink", "serif-mono", &[4, 4, 3, 4, 2, 4]);
        fs::write(d.join("styles.css"), format!("{line}\nbody{{}}\n")).unwrap();
        let s = &find_stamps(&d)[0];
        assert_eq!(s.structure.as_deref(), Some("ledger-columns"));
        assert_eq!(s.critique.len(), 6);
        let looks = recent_looks_or_stamps(&d, 5);
        assert_eq!(looks[0].hero.as_deref(), Some("catch-board"));
        let rules: Vec<String> = check_gates(&d).into_iter().map(|g| g.rule).collect();
        assert!(rules.contains(&"SELF-CRITIQUE-BELOW-3".to_string()));
        assert!(rules.contains(&"GATE-DIRECTION-NOT-APPROVED".to_string()), "a page stamp starts the direction gate");
        let st = rotation_statement(&looks, &["a".into(), "b".into()], false);
        assert!(st.contains("catch-board once") && st.contains("{a, b}"), "{st}");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn continue_never_passes_the_direction_gate_and_skip_needs_the_user() {
        let d = tmp("gate");
        fs::write(d.join(".layoza/direction.md"), "PURPOSE: sell bread; audience: locals; new\n").unwrap();
        let rules = |d: &Path| check_gates(d).into_iter().map(|g| g.rule).collect::<Vec<_>>();
        fs::write(d.join(DIRECTION_APPROVED), "SHOWN: A; B; C\nSCREENSHOTS: qa/a.png\nUSER SAID: \"continue\"\n").unwrap();
        assert_eq!(rules(&d), vec!["GATE-CONTINUE-IS-NOT-A-PICK"]);
        fs::write(d.join(DIRECTION_APPROVED), "SKIPPED: agent was in a hurry\nUSER SAID: \"looks fine\"\n").unwrap();
        assert_eq!(rules(&d), vec!["GATE-SKIP-WITHOUT-USER"]);
        fs::write(d.join(DIRECTION_APPROVED), "SKIPPED: user asked\nUSER SAID: \"just build it, you choose\"\n").unwrap();
        assert!(rules(&d).is_empty());
        fs::write(d.join(DIRECTION_APPROVED), "SHOWN: A; B; C\nSCREENSHOTS: qa/a.png, qa/b.png, qa/c.png\nUSER SAID: \"B, but with A's type\"\n").unwrap();
        assert!(rules(&d).is_empty());
        // A real brand needs assets before the spec.
        fs::write(d.join(".layoza/direction.md"), "PURPOSE: refresh; existing\nBRAND: Hearth Bakery\n").unwrap();
        assert_eq!(rules(&d), vec!["GATE-NO-BRAND-SPEC"]);
        fs::write(d.join(BRAND_SPEC), "COLOURS: #aa5500\nASSETS:\n- logo: assets/logo.svg\n").unwrap();
        assert_eq!(rules(&d), vec!["GATE-BRAND-SPEC-BEFORE-ASSETS"]);
        fs::write(d.join(BRAND_SPEC), "ASSETS:\n- logo: assets/logo.svg\nCOLOURS: #aa5500\n").unwrap();
        assert!(rules(&d).is_empty());
        fs::write(d.join(PRODUCT_FACTS), "- Oven model X2 ships in May\n").unwrap();
        assert_eq!(rules(&d), vec!["GATE-FACT-UNVERIFIED"]);
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn redesign_skeletons_are_compared() {
        let a = structural_fingerprint("<header><nav></nav></header><main><section><h2></h2><ul></ul></section><section><h2></h2></section></main><footer></footer>");
        let b = structural_fingerprint("<HEADER><nav></nav></HEADER><main><section><h2>x</h2><ul></ul></section><section><h2></h2></section></main><footer></footer>");
        let c = structural_fingerprint("<main><figure><img></figure><article><h1></h1><dl></dl><table></table></article><aside><form></form></aside></main>");
        assert!(skeleton_similarity(&a, &b) >= REDESIGN_MAX_SIMILARITY);
        assert!(skeleton_similarity(&a, &c) < 0.3);
    }
}
