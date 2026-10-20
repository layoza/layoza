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
        ("accent-2", Some(s(p, "secondary_accent").to_string())),
        ("font-display", font("display_font")),
        ("font-body", font("body_font")),
        ("size-display", Some(s(tt, "display").to_string())),
        ("size-h2", Some(s(tt, "h2").to_string())),
        ("size-body", Some(s(tt, "body").to_string())),
        ("size-small", Some(s(tt, "small").to_string())),
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
        let _ = writeln!(m, "- Material to obtain: {}. Use design_assets to inventory/register actual images or working product views, with source, licence, credit, role and crop.", s(d,"hero_material"));
        let _ = writeln!(m, "- Tradeoff: {}", clip(s(d,"rationale"), 260));
        let _ = writeln!(m, "- Suggested motion: {}. Explicit no-animation means 0; the user's preference wins.", d["dials"]["motion_intensity"]);
        if i == 0 { tokens_css(&mut m,d); }
    }
    let _ = writeln!(m, "\n## Choose on the render");
    let _ = writeln!(m, "Use the same factual copy and available material across three desktop/phone first screens. Compare hierarchy, typography, image integration, clarity, full-page rhythm and phone composition. Choose the strongest complete idea and say what it gains and sacrifices. Familiar controls can be correct. Page shapes, motif ids and novelty scores are suggestions, not orders.");
    let _ = writeln!(m, "\n## Build, review and show");
    let _ = writeln!(m, "1. Record PURPOSE, pinned choices, candidates, PICKED, reason, SECTIONS, MATERIAL and MOTION (0-10) in .layoza/direction.md. Pick transparently as the designer unless the user asked to choose; never invent user approval.");
    let _ = writeln!(m, "2. Prototype the most uncertain identity-defining composition first, then complete the whole page in the native framework. Every section earns its place; no fixed section/picture quota or forced decorative trick.");
    let _ = writeln!(m, "3. Inspect actual desktop/phone renders and important interaction/motion states. Name visible strengths and the two weakest decisions, fix them, then recapture. Use a fresh independent critic when available; label self-review honestly. Default review budget: three rounds; unresolved issues are unfinished work, not a pass.");
    let _ = writeln!(m, "4. Persist candidates, chosen direction, reasons, changes and actual screenshot paths with review_design. Keep asset provenance with design_assets. Run capture_screenshots / layoza qa for real visual, responsive, interaction and accessibility defects. Speed diagnostics are optional.");
    let _ = writeln!(m, "5. Show the final designs and concrete evidence. Passing a checker or a self-score does not establish superiority over rivals.");
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn neon_on_black_spots_the_stock_dark_look() {
        assert!(neon_on_black("#101510", "#B2F630"));
        assert!(!neon_on_black("#FFF", "#B2F630"));
        assert!(!neon_on_black("#1F2A4A", "#B2F630"));
        assert!(!neon_on_black("#101510", "#8C6A3F"));
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
        assert!(md.len() < 6500, "{}", md.len());
    }
}
