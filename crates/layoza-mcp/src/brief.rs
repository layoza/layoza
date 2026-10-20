//! The short design brief: one whole-page look plus taste rules, as Markdown.
//!
//! This is what `get_design_system` returns by default. The model designs the
//! whole page in one pass from it; the dealt parts are ideas, not mandates.

use serde_json::Value;
use std::fmt::Write as _;

/// Taste rules every page meets. Short on purpose: one idea per line.
pub const TASTE: &[&str] = &[
    "Before code, plan: a two-word tone pushed to its extreme (e.g. sunlit-brutal), the signature, the material (surface, light, texture), 4-6 colour tokens, the type pair, one layout line per section. Score it 0-10 against this brief's 10; fix every line below 8.",
    "The hero opens on the subject itself: the trade's own object, place, work or product at scale, not decoration.",
    "Every section is a claim plus its proof (a picture, a sourced number, a real name). No proof, cut it.",
    "Type: one family or two clearly different; display 4-6x body, leading 0.9-1.05, slight negative tracking; body 16-20px, 45-75ch. Two or three sizes do the work. Headlines break by hand: 2 lines desktop, 3 phone.",
    "Colour: ground, ink, accent, 2-3 supporting tokens; quiet or immersive (a full-bleed colour field), chosen first. Secondary text tints the ground's hue; body contrast 4.5:1+.",
    "Depth: one system - photographic light, three overlapping planes with contact shadow, or tonal fields stepping in lightness. Flat only if asked. Separate surfaces by tone, scale and space; ruled lines only for real tables.",
    "Pacing: one idea per screen near the top, 5-7 sections, rhythm changing between full-bleed, contained and dense; one screen where type or subject alone fills the view.",
    "Layout: a real grid with asymmetry; two or more sections break their frame (type over a picture, an object off the edge or across a seam); neighbours never place the heading alike; section gaps 120-220px; no blank stretch over half a screen.",
    "Every label, number and divider informs; one radius, one rule weight, one spacing scale.",
    "Copy in the customer's words: concrete nouns, numbers, places, names. Buttons say the exact action.",
    "Motion: the signature is driven by scroll or hand in planned order (40-80ms staggers, one easing). The hero holds one living element and one scroll move shows depth between planes. Elsewhere quiet: feedback under 300ms, hover only on (hover:hover), transform/opacity/clip-path only. No JS or reduced motion shows the final frame.",
    "Imagery: real photos or footage of the subject, else a crafted render of the real object; no stock or SVG people, no emoji. Four or more pictures, one full-bleed, one crop ratio and treatment; work shown big, not as thumbnails.",
    "Finish: themed selection, focus ring and scrollbar; tabular figures; every state designed. Mobile: main action above the fold at 390px, 44px taps, portrait re-crops, no sideways scroll.",
];

/// Patterns that read as AI-made. Ban unless the brief asks for them.
pub const AVOID: &[&str] = &[
    "Small tracked-caps eyebrows above headings; one coloured or italic word inside a headline.",
    "Grids of identical icon-topped cards with one radius and soft shadow; cards in cards.",
    "A hero of big stat numbers, or a gradient blob behind a centred headline and two pill buttons.",
    "Decorative gradient text, washes, glows, glass (light falloff, photo scrims, faint grain on print/film worlds are fine); zero-offset shadows, coloured side borders over 1px.",
    "Stock AI palettes: purple-to-pink, violet on dark, near-black with one neon accent, cream with serif and terracotta.",
    "Monospace as a 'tech' costume; middle-dot meta strings; arrows on every link.",
    "A page assembled from stock parts (hero, logo strip, feature cards, steps, testimonials, CTA) with no set piece, moving only by fade-up and hover lift; every section the same centred heading + subhead + grid.",
    "Zig-zag stacks: picture left and text right, then swapped, down the page.",
    "A wireframe page: 1px-outlined boxes and ruled lists section after section, flat, nothing overlapping.",
    "Invented proof: reviews, logos, statistics or awards the brief did not give.",
];

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

/// Render the deal as a compact Markdown brief.
pub fn markdown(out: &Value) -> String {
    let ub = &out["understood_brief"];
    let dirs = out["directions"].as_array().cloned().unwrap_or_default();
    let mut m = String::new();
    let subject = s(ub, "subject");
    let _ = writeln!(m, "# Design brief: {}\n", if subject.is_empty() { "this site" } else { subject });

    let _ = writeln!(m, "## Who it is for");
    for (label, key) in [("Trade", "trade"), ("Audience", "audience"), ("Main action", "primary_action"), ("Motion", "motion_wish"), ("Stack", "stack")] {
        let v = s(ub, key);
        if !v.is_empty() {
            let _ = writeln!(m, "- {label}: {v}");
        }
    }
    let tone = list(&ub["tone"]);
    if !tone.is_empty() {
        let _ = writeln!(m, "- Tone: {}", tone.join("; "));
    }
    let must = list(&ub["must_haves"]);
    if !must.is_empty() {
        let _ = writeln!(m, "- Must keep (user asked): {}", clip(&must.join(", "), 400));
    }
    let open = list(&ub["open_questions"]);
    if !open.is_empty() {
        let _ = writeln!(m, "- Ask only if it blocks building: {}", open.join(" / "));
    }

    if let Some(d) = dirs.first() {
        let _ = writeln!(m, "\n## The signature moment (build this first)");
        let _ = writeln!(m, "The set piece the page is remembered for: what this subject does that nothing else does (how it is made, used, moves, scales, or changes with distance, time or light), staged as a scene the visitor drives by scroll or hand, one to two screens, built from the real subject. It must fail for a rival business. On a docs page, article or portfolio the content or the work itself is the signature: keep the interface quiet around it.");
        let lead = s(d, "motion_choreography");
        if !lead.is_empty() {
            let _ = writeln!(m, "- Starting idea: {}", clip(lead, 300));
        }
        let _ = writeln!(m, "- Done when: start, middle and end frames each read as a poster, it is smooth on a phone, and the final frame works without JavaScript.");

        let _ = writeln!(m, "\n## The look: {}", title(d));
        let _ = writeln!(m, "One look that frames the signature. Tokens, type and shape are the system; the rest are raw ideas, kept only if they serve the signature. Where two ideas clash, the signature wins; the user's own asks always win.\n");
        let _ = writeln!(m, "- Composition: {}", clip(s(d, "layout_composition"), 260));
        let _ = writeln!(m, "- Content: {}", clip(s(d, "content_treatment"), 200));
        let _ = writeln!(m, "- Shape: radius {}; hero material: {}", s(d, "radius"), s(d, "hero_material"));
        let _ = writeln!(m, "- Fonts: {} (body contrast {}:1)", s(&d["typography"], "google_fonts_url"), d["palette"]["wcag_body_contrast"]);
        tokens_css(&mut m, d);
        let ps = &d["page_shape"];
        for (label, key) in [("Hero idea", "hero_frame"), ("Grounds", "ground"), ("Motif", "motif")] {
            let b = s(&ps[key], "build");
            if !b.is_empty() {
                let _ = writeln!(m, "- {label}: {}", clip(b, 220));
            }
        }
        let nav = s(&d["nav"], "name");
        let foot = s(&d["footer"], "name");
        if !nav.is_empty() || !foot.is_empty() {
            let _ = writeln!(m, "- Header idea: {nav}. Footer idea: {foot}.");
        }
        let mech: Vec<&str> = d["mechanics"].as_array().map(|a| a.iter().map(|x| s(x, "name")).filter(|n| !n.is_empty()).collect()).unwrap_or_default();
        if !mech.is_empty() {
            let _ = writeln!(m, "- Section ideas: {}.", mech.join("; "));
        }
    }

    if dirs.len() > 1 {
        let _ = writeln!(m, "\n## Other looks (if the user asks for options)");
        for d in &dirs[1..] {
            let _ = writeln!(m, "- {}: {} / {}, accent {}", title(d), s(&d["typography"], "display_font"), s(&d["typography"], "body_font"), s(&d["palette"], "primary_accent"));
        }
    }

    let _ = writeln!(m, "\n## Taste rules");
    for r in TASTE {
        let _ = writeln!(m, "- {r}");
    }
    let _ = writeln!(m, "\n## Avoid (reads as AI-made)");
    for r in AVOID {
        let _ = writeln!(m, "- {r}");
    }
    let ar = &out["avoid_recent"];
    let recent: Vec<String> = ["palettes", "fonts", "motifs"].iter().flat_map(|k| list(&ar[*k])).collect();
    if !recent.is_empty() {
        let _ = writeln!(m, "- Recently used, do not repeat: {}", clip(&recent.join(", "), 300));
    }
    if let Some(slots) = out["uniqueness"]["slots"].as_array() {
        let rep: Vec<String> = slots.iter().filter(|x| s(x, "status").contains("repeat")).map(|x| format!("{} ({})", s(x, "slot"), s(x, "id"))).collect();
        if !rep.is_empty() {
            let _ = writeln!(m, "- Repeats a recent run, change these: {}", rep.join(", "));
        }
    }

    let _ = writeln!(m, "\n## How to build");
    let _ = writeln!(m, "1. Write the plan (first taste rule), signature first. Change any part of the look above that fights it; say what you changed.");
    let _ = writeln!(m, "2. Build the signature moment first, then the rest of the page around it in one pass, in the project's own framework, with real copy for this business.");
    let _ = writeln!(m, "3. Screenshot the signature at three scroll positions, then the whole page at 1440px and 390px. Ask: could a template have made this? Swap two section headlines: if both sections still work, they are interchangeable, so rebuild one. Cut one element that does not earn its place. Fix the three worst problems (generic, crowded, broken), at most two rounds.");
    let _ = writeln!(m, "4. Run `capture_screenshots` (or `layoza qa <url>`) once and fix its errors. Done.");
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
        assert!(md.contains("## The signature moment") && md.contains("## The look: Counter") && md.contains("The morning board") && md.contains("Flour dust."));
        assert!(md.contains(":root {\n  --ground: #FFF;") && md.contains("--font-display: \"Fraunces\""));
        assert!(md.contains("- Map:") && md.contains("A.nav (nav-x)"));
        assert!(!md.contains("…") && md.contains("- Composition: The counter at dawn. The counter"), "long text keeps whole sentences");
        assert!(md.len() < 6000, "{}", md.len());
    }
}
