//! The short design brief: one whole-page look plus taste rules, as Markdown.
//!
//! This is what `get_design_system` returns by default. The model designs the
//! whole page in one pass from it; the dealt parts are ideas, not mandates.

use serde_json::Value;
use std::fmt::Write as _;

/// Taste rules every page meets. Short on purpose: one idea per line.
pub const TASTE: &[&str] = &[
    "Before code, write a 5-line plan: the signature moment, the 4-6 colour tokens, the type pair, a one-line layout sentence per section, what the rest of the page does to set up the signature. Check it against the brief and rewrite anything generic.",
    "The hero opens on the subject itself: the trade's own object, place, work or product at scale. Decoration never stands in for the subject.",
    "Every section is a claim plus its proof (a picture, a number with a source, a real name). A section with no proof gets cut, not padded.",
    "Type: one family or two clearly different ones, display 4-6x the body size, tight leading (0.9-1.05) and slight negative tracking on big type, body 16-20px at 45-75 characters a line, at most 6 sizes. No habitual default face for display.",
    "Colour: one ground, one ink, one accent, plus at most 2-3 supporting tokens. Secondary text is a tint of the ground's hue, not grey; body contrast 4.5:1 or more. Light or dark follows where the page is read, not the industry cliché.",
    "Pacing like a product launch: one idea per screen near the top, 5-7 sections in all, rhythm that changes between full-bleed, contained and dense; at least one screen where type or the subject alone fills the view. Dark and light grounds follow the story, not a pattern.",
    "Layout: a real grid with deliberate asymmetry, more space above a heading than below, generous gaps between sections (96-200px desktop). Headlines at most 2-3 lines.",
    "Every border, label, number and divider carries information; one radius and one rule weight across the page.",
    "Copy in the customer's words: concrete nouns, real numbers, places, names. Buttons say the exact action ('Get a quote for my wall'). Sentence case.",
    "Motion craft: the signature is driven by scroll position or the user's hand, not a timer, and is choreographed: parts arrive in a planned order (40-80ms staggers), one easing family, every move explains something. Outside it, motion stays quiet: UI feedback under 300ms with strong ease-out, exits faster than entrances, hover only on (hover:hover). Animate transform, opacity, clip-path or a canvas only. Without JS or under reduced motion show the final frame.",
    "Imagery: real or brand photos and footage of the subject; if none exist, a crafted render or diagram of the real object, never stock people, SVG people or emoji icons.",
    "Finish: themed selection, focus ring and scrollbar; tabular figures; every state (hover, focus, active, disabled, empty, error) designed.",
    "Mobile is designed, not squeezed: headline readable in 3 lines, main action above the fold at 390px, 44px tap targets, no sideways scroll.",
];

/// Patterns that read as AI-made. Ban unless the brief asks for them.
pub const AVOID: &[&str] = &[
    "Small tracked-caps eyebrow labels above headings; one accented (coloured or italic) word inside a headline.",
    "Grids of identical icon-topped cards with the same radius and soft shadow; cards nested in cards.",
    "Hero of big stat numbers, or a gradient blob/mesh behind a centred headline and two pill buttons.",
    "Gradient text, decorative gradient washes, glows, glass panels, zero-offset shadows.",
    "Stock AI palettes: purple-to-pink, violet/indigo on dark, near-black with one neon accent, cream with serif and terracotta.",
    "Monospace as a 'tech' costume; middle-dot meta strings; arrows tacked onto every link and button.",
    "Coloured side borders thicker than 1px on cards, buttons or callouts.",
    "A page assembled from stock parts (hero, logo strip, feature cards, steps, testimonials, CTA) with no set piece, where the only motion is a fade-up on scroll and a hover lift.",
    "Zig-zag stacks: picture left and text right, then swapped, repeated down the page.",
    "Invented proof: reviews, client logos, statistics or awards the brief did not give.",
    "Every section built from the same centred heading + subhead + grid.",
];

fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v.get(k).and_then(|x| x.as_str()).unwrap_or("")
}

/// Cut text at a sentence or word boundary near `max` characters.
fn clip(t: &str, max: usize) -> String {
    if t.chars().count() <= max {
        return t.to_string();
    }
    let cut: String = t.chars().take(max).collect();
    let end = cut.rfind(". ").map(|i| i + 1).filter(|&i| i > max / 2).or_else(|| cut.rfind(' ')).unwrap_or(cut.len());
    format!("{}…", cut[..end].trim_end())
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
        let _ = writeln!(m, "The one set piece the page is remembered for. Ask what this subject does or reveals that nothing else does (how it is made, used, moves, scales, or changes with distance, time or light) and stage it as a scene the visitor drives by scroll or hand, one to two screens long, built from the real subject. It must fail for a rival business.");
        let lead = s(d, "motion_choreography");
        if !lead.is_empty() {
            let _ = writeln!(m, "- Starting idea: {}", clip(lead, 300));
        }
        let _ = writeln!(m, "- Done when: start, middle and end frames each read as a poster, it is smooth on a phone, and the final frame works without JavaScript.");

        let _ = writeln!(m, "\n## The look: {}", title(d));
        let _ = writeln!(m, "One look that frames the signature. Tokens, type and shape are the system; the rest are raw ideas, kept only if they serve the signature. The user's own asks always win.\n");
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
    let _ = writeln!(m, "1. Write the 5-line plan (first taste rule), signature first. Change any part of the look above that fights it; say what you changed.");
    let _ = writeln!(m, "2. Build the signature moment first, then the rest of the page around it in one pass, in the project's own framework, with real copy for this business.");
    let _ = writeln!(m, "3. Screenshot the signature at three scroll positions, then the whole page at 1440px and 390px. Ask: could a template have made this? Fix the three worst problems (generic, crowded, broken), at most two rounds.");
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
        assert!(md.contains("…"), "long composition is clipped");
        assert!(md.len() < 6000, "{}", md.len());
    }
}
