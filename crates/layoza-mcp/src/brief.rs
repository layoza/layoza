//! Three concrete compositions, material requirements and an evidence-backed visual workflow.
//!
//! This is what `get_design_system` returns by default. The model designs the
//! whole page in one pass from it; the dealt parts are ideas, not mandates.

use serde_json::Value;
use std::fmt::Write as _;

fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v.get(k).and_then(|x| x.as_str()).unwrap_or("")
}

/// Whole sentences of `t` up to about `max` characters. The first sentence is always kept whole
/// (up to twice `max`), so an idea is never cut off mid-thought.
fn clip(t: &str, max: usize) -> String {
    let t = t.trim();
    if t.chars().count() <= max {
        return t.to_string();
    }
    let mut out = String::new();
    for sentence in t.split_inclusive(". ") {
        let next = out.chars().count() + sentence.chars().count();
        if !out.is_empty() && next > max {
            break;
        }
        out.push_str(sentence);
        if next > max {
            break;
        }
    }
    if out.chars().count() > max * 2 {
        // One run-on sentence: end at the last clause break instead.
        let cut: String = out.chars().take(max * 2).collect();
        let end = cut.rfind(['.', ';', ':', ',']).unwrap_or(cut.len());
        out = format!("{}.", cut[..end].trim_end());
    }
    out.trim_end().to_string()
}

fn list(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default()
}

fn title(d: &Value) -> String {
    let t = s(d, "title");
    let t = t.split_once(" — ").map(|(_, r)| r).unwrap_or(t);
    // "Industry (Object Study)" names the idea in brackets; the industry is already said above.
    match (t.rfind(" ("), t.ends_with(')')) {
        (Some(i), true) => t[i + 2..t.len() - 1].to_string(),
        _ => t.to_string(),
    }
}

/// A warm pale ground that `layoza qa`'s cream-palette critique flags (same test as the QA script:
/// every channel at least 209, red >= green >= blue, and red minus blue between 6 and 48).
fn is_cream(hex: &str) -> bool {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 {
        return false;
    }
    let c = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok();
    let (Some(r), Some(g), Some(b)) = (c(0), c(2), c(4)) else { return false };
    r.min(g).min(b) >= 209 && r >= g && g >= b && (6..=48).contains(&(r - b))
}

/// The look's tokens as a `:root` block the agent can paste straight into its CSS.
fn tokens_css(m: &mut String, d: &Value) {
    let (p, t, tt) = (&d["palette"], &d["typography"], &d["type_tokens"]);
    let font = |k: &str| {
        let f = s(t, k);
        (!f.is_empty()).then(|| format!("\"{f}\", system-ui, sans-serif"))
    };
    let vars = [
        ("ground", Some(s(p, "background").to_string())),
        ("surface", Some(s(p, "surface").to_string())),
        ("ink", Some(s(p, "foreground").to_string())),
        ("muted", Some(s(p, "muted").to_string())),
        ("accent", Some(s(p, "primary_accent").to_string())),
        ("font-display", font("display_font")),
        ("font-body", font("body_font")),
        ("size-display", Some(s(tt, "display").to_string())),
        ("size-h2", Some(s(tt, "h2").to_string())),
        ("size-body", Some(s(tt, "body").to_string())),
        ("size-small", Some(s(tt, "small").to_string())),
        ("action-fill", Some(s(&d["visual_system"], "action_fill").to_string())),
        ("on-action", Some(s(&d["visual_system"], "on_action").to_string())),
        ("action-hover", Some(s(&d["visual_system"], "hover_fill").to_string())),
        ("action-ink", Some(s(&d["visual_system"], "ink").to_string())),
        ("action-focus", Some(s(&d["visual_system"], "focus").to_string())),
        ("control-radius", Some(s(&d["visual_system"], "control_radius").to_string())),
    ];
    let lines: Vec<String> = vars.into_iter().filter_map(|(k, v)| v.filter(|v| !v.is_empty()).map(|v| format!("  --{k}: {v};"))).collect();
    if !lines.is_empty() {
        let _ = writeln!(m, "\n```css\n:root {{\n{}\n}}\n```\n", lines.join("\n"));
    }
}

#[cfg(test)]
fn rgb(hex: &str) -> Option<[f64; 3]> {
    let h = hex.trim().trim_start_matches('#');
    let h: String = if h.len() == 3 { h.chars().flat_map(|c| [c, c]).collect() } else { h.to_string() };
    if h.len() != 6 || !h.is_ascii() {
        return None;
    }
    let ch = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok().map(|v| f64::from(v) / 255.0);
    Some([ch(0)?, ch(2)?, ch(4)?])
}

/// A near-black ground with one bright, saturated accent: the stock "dark + neon" look.
#[cfg(test)]
fn neon_on_black(ground: &str, accent: &str) -> bool {
    let (Some(g), Some(a)) = (rgb(ground), rgb(accent)) else { return false };
    let lum = |c: [f64; 3]| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
    let spread = |c: [f64; 3]| c.iter().copied().fold(0.0, f64::max) - c.iter().copied().fold(1.0, f64::min);
    lum(g) < 0.1 && spread(g) < 0.12 && lum(a) > 0.45 && spread(a) > 0.6
}

/// Render the deal as a compact Markdown brief.
pub fn markdown(out: &Value) -> String {
    let ub = &out["understood_brief"];
    let dirs = out["directions"].as_array().cloned().unwrap_or_default();
    let mut m = String::new();
    let _ = writeln!(m, "# Design brief: {}\n", s(ub, "subject"));
    if let Some(request) = out.get("client_request").and_then(Value::as_str) {
        let _ = writeln!(m, "## Original client request\n{request}\n\nThe original request overrides catalogue suggestions, including available local fonts, assets and dependency limits. A font inventory offers choices; it does not require using every family.");
    }
    if let Some(context) = out.pointer("/creative_workspace/brief_context").filter(|c| !c.is_null()) {
        let _ = writeln!(m, "\nExplicit client context: {}", context);
    }
    let _ = writeln!(m, "## Establish the truth");
    for (label,key) in [("Subject", "subject"), ("Audience", "audience"), ("Primary task", "primary_action"), ("Stack", "stack")] {
        if !s(ub,key).is_empty() { let _ = writeln!(m, "- {label}: {}", s(ub,key)); }
    }
    for item in list(&ub["must_haves"]) { let _ = writeln!(m, "- Pinned: {item}"); }
    for item in list(&ub["assumed"]) { let _ = writeln!(m, "- Assumption to check: {item}"); }
    for item in list(&ub["open_questions"]) { let _ = writeln!(m, "- Missing fact: {item}"); }
    let _ = writeln!(m, "\nVerify that this names the actual client. Negative constraints and implementation techniques are not the subject. Supply brief_context to correct a heuristic reading before building.");
    let _ = writeln!(m, "\n## Three compositions to render and compare");
    for (i,d) in dirs.iter().enumerate() {
        let _ = writeln!(m, "\n### Concept {}: {}", i+1, title(d));
        let _ = writeln!(m, "- Composition: {}", clip(s(d,"layout_composition"), 520));
        let _ = writeln!(m, "- Content argument: {}", clip(s(d,"content_treatment"), 240));
        let _ = writeln!(m, "- Typography: {} / {}; ground {}; ink {}; accent {}.", s(&d["typography"],"display_font"), s(&d["typography"],"body_font"), s(&d["palette"],"background"), s(&d["palette"],"foreground"), s(&d["palette"],"primary_accent"));
        if is_cream(s(&d["palette"],"background")) {
            let _ = writeln!(m, "- Dealt cream ground: if built, declare `<meta name=\"layoza-brand-allow\" content=\"palette:cream\">`.");
        }
        let _ = writeln!(m, "- Material to obtain: {}. Use design_assets to inventory/register actual images or working product views, with source, licence, credit, role and crop.", s(d,"hero_material"));
        let _ = writeln!(m, "- Tradeoff: {}", clip(s(d,"rationale"), 260));
        let _ = writeln!(m, "- Motion dial: {}. Choreography: {}", d["dials"]["motion_intensity"], s(d,"motion_choreography"));
        if let Some(c) = d.get("composition_system").filter(|v| !v.is_null()) {
            let _ = writeln!(m, "- Identity anchor: {}\n- Type relationship: {}", clip(s(c,"identity_anchor"),200), clip(s(c,"typography_relationship"),220));
            if let Some(study) = c.get("identity_study") {
                let _ = writeln!(m, "- Identity evidence status: {}\n- Composition experiment: {}", s(study,"status"), s(study,"compositional_operation"));
            }
        }
        tokens_css(&mut m,d);
        if let Some(v) = d.get("visual_system").filter(|v| !v.is_null()) {
            let _ = writeln!(m, "- CTA colour pair: {} fill / {} label ({}:1 normal text). Shape suggestion: {}. Distinctiveness comes from the concept, typography and placement; do not force a novel control.", s(v,"action_fill"), s(v,"on_action"), v["action_text_contrast"], s(v,"shape"));
        }
    }
    if let Some(v) = dirs.first().and_then(|d| d.get("visual_system")).filter(|v| !v.is_null()) {
        let css = s(v,"control_css").lines().skip(1).collect::<Vec<_>>().join("\n");
        let _ = writeln!(m, "\n## Palette and CTA execution\n{}\nUse the chosen concept's role variables above with these shared starter classes; explicit brand components win. Secondary palette accents are not automatically exported as UI tokens.\n~~~css\n{css}\n~~~", s(v,"composition_review"));
    }
    if let Some(foundation) = out.get("execution_foundation") {
        // The short brief states motion policy and review below; avoid repeating
        // the same prose in its execution JSON. Full tool JSON retains all fields.
        let mut compact = foundation.clone();
        if let Some(motion) = compact.get_mut("motion").and_then(Value::as_object_mut) {
            for key in ["default_policy", "section_authoring", "evidence"] { motion.remove(key); }
        }
        let _ = writeln!(m, "\n## Execution foundation\n{}", serde_json::to_string_pretty(&compact).unwrap_or_default());
    }
    let _ = writeln!(m, "\n## Choose on the render");
    let _ = writeln!(m, "Each concept's composition_system in detail: full includes editable static HTML/CSS using the exact quoted copy, identity applications and visual review criteria. Compare its paused reading order with real artwork before animating; carry the chosen identity into ordinary sections, not only the hero.");
    let _ = writeln!(m, "Use the same factual copy and available material across three desktop/phone first screens. Compare hierarchy, typography, image integration, clarity, full-page rhythm and phone composition. Choose the strongest complete idea and say what it gains and sacrifices. Familiar controls can be correct. Page shapes, motif ids and novelty scores are suggestions, not orders.");
    let _ = writeln!(m, "Colour and type are dealt: keep one concept's hue family and type pair (user-named axes win; shift lightness only for contrast). Refuse the house reflex unless dealt: warm serif on cream, deep green, amber glow.");
    let _ = writeln!(m, "\n## Build, review and show");
    let _ = writeln!(m, "Motion is on by default until the user explicitly asks for none: GSAP owns authored animation unless another engine is requested. An explicit Framer Motion request can own separate React presence/layout properties. Never let two engines animate the same transform. For rich motion, author recommend_animation.motion_plan, implement section-motion.mjs, and review desktop/phone playback with node scripts/review-motion.mjs <URL> --out .layoza/motion-review. Several earned chapter transformations serve one thesis; repeated reveals and functional tests do not prove motion quality. For identity weaknesses run node scripts/review-identity.mjs <URL> --reference <comparison-URL> --out .layoza/identity-review; inspect actual opening/body/enquiry captures and retain the report with review_design.identity_report.");
    let _ = writeln!(m, "1. Record PURPOSE, pinned choices, candidates, PICKED, reason, SECTIONS, MATERIAL and MOTION (0-10) in .layoza/direction.md. Pick transparently as the designer unless the user asked to choose; never invent user approval.");
    let _ = writeln!(m, "2. Prototype the most uncertain identity-defining composition first, then complete the whole page in the native framework. Every section earns its place; no fixed section/picture quota or forced decorative trick.");
    let _ = writeln!(m, "3. Inspect actual desktop/phone renders and important interaction/motion states. Name visible strengths and the two weakest decisions, fix them, then recapture. Use a fresh independent critic when available; label self-review honestly. Budget: three rounds; unresolved issues are unfinished work.");
    let _ = writeln!(m, "4. Persist candidates, chosen direction, reasons, changes and actual screenshot paths with review_design. Keep asset provenance with design_assets. Run capture_screenshots / layoza qa for real visual, responsive, interaction and accessibility defects.");
    let _ = writeln!(m, "5. Show final designs and evidence; a checker pass or self-score proves no superiority over rivals.");
    let _ = writeln!(m, "Recurring faults, fix before handoff: >7 text sizes; visual-less lower sections; no frame bleed; one card/radius on all; default nav; intro >2.5s; phone-hero empty band.");
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn default_brief_preserves_each_concepts_execution_data() {
        let directions: Vec<Value> = (1..=3).map(|i| json!({
            "title": format!("Concept {i}"),
            "typography": {"display_font": format!("Display {i}"), "body_font": format!("Body {i}")},
            "type_tokens": {"display": format!("{i}rem")},
            "motion_choreography": format!("Subject scene {i} assembles then resolves."),
            "dials": {"motion_intensity": 8}
        })).collect();
        let request = "Local fonts only: Space Grotesk or DM Sans. No remote dependencies. Exact headline: Good businesses deserve extraordinary websites.";
        let out = json!({"client_request": request, "directions": directions, "execution_foundation": layoza_core::craft::execution_foundation(8)});
        let md = markdown(&out);
        assert!(md.contains(request));
        for d in out["directions"].as_array().unwrap() {
            assert!(md.contains(d["motion_choreography"].as_str().unwrap()));
            assert!(md.contains(&format!("--font-display: \"{}\"", d["typography"]["display_font"].as_str().unwrap())));
            assert!(md.contains(&format!("--size-display: {}", d["type_tokens"]["display"].as_str().unwrap())));
        }
        assert!(md.contains(out["execution_foundation"]["layout_css"].as_str().unwrap()));
    }

    #[test]
    fn neon_on_black_spots_the_stock_dark_look() {
        assert!(neon_on_black("#101510", "#B2F630"));
        assert!(!neon_on_black("#FFF", "#B2F630"));
        assert!(!neon_on_black("#1F2A4A", "#B2F630"));
        assert!(!neon_on_black("#101510", "#8C6A3F"));
    }

    #[test]
    fn cream_matches_the_qa_rule_and_only_a_dealt_cream_is_declared() {
        assert!(is_cream("#F4F1EC") && is_cream("#F6F1E9"));
        assert!(!is_cream("#FFFFFF") && !is_cream("#EEF0F5") && !is_cream("#101510") && !is_cream("#FFF"));
        let deal = |bg: &str| markdown(&json!({ "directions": [{ "title": "A", "palette": { "background": bg }, "typography": {} }] }));
        assert!(deal("#F4F1EC").contains("palette:cream"));
        assert!(!deal("#EEF0F5").contains("palette:cream"));
        assert!(deal("#EEF0F5").contains("Colour and type are dealt"));
    }

    #[test]
    fn brief_is_short_and_names_the_look() {
        let out = json!({
            "understood_brief": { "subject": "harbour bakery", "audience": "locals", "primary_action": "order bread", "tone": ["warm"], "must_haves": [], "open_questions": [] },
            "directions": [
                { "title": "Direction A — Bakery (Counter)", "layout_composition": "The counter at dawn. ".repeat(40), "palette": { "background": "#FFF", "primary_accent": "#A33" }, "typography": { "display_font": "Fraunces", "body_font": "Work Sans" }, "page_shape": { "motif": { "build": "Flour dust." } }, "mechanics": [{ "name": "The morning board" }] },
                { "title": "Direction B — Bakery (Map)", "palette": {}, "typography": {} }
            ],
            "uniqueness": { "slots": [{ "slot": "A.nav", "id": "nav-x", "status": "repeat of run 3" }] }
        });
        let md = markdown(&out);
        assert!(md.starts_with("# Design brief: harbour bakery"));
        assert!(md.contains("### Concept 1: Counter") && md.contains("### Concept 2: Map"));
        assert!(md.contains("--font-display: \"Fraunces\""));
        assert!(md.contains("design_assets") && md.contains("review_design"));
        assert!(md.contains("same factual copy") && md.contains("MOTION (0-10)"));
        assert!(!md.contains("binding") && !md.contains("Recently used, do not repeat"));
        assert!(md.len() < 10000, "{}", md.len());
    }
}
