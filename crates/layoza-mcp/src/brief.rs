//! The short design brief: one whole-page look plus taste rules, as Markdown.
//!
//! This is what `get_design_system` returns by default. The model designs the
//! whole page in one pass from it; the dealt parts are ideas, not mandates.

use serde_json::Value;
use std::fmt::Write as _;

/// Taste rules every page meets. Short on purpose: one idea per line.
pub const TASTE: &[&str] = &[
    "Before code, write a 5-line plan: the one idea (it must fail if a rival business used it), the 4-6 colour tokens, the type pair, a one-line layout sentence per section, the one motion moment. Check it against the brief and rewrite anything generic.",
    "The hero opens on the subject itself: the trade's own object, place, work or product at scale. Decoration never stands in for the subject.",
    "Every section is a claim plus its proof (a picture, a number with a source, a real name). A section with no proof gets cut, not padded.",
    "Type: one family or two clearly different ones, display 4-6x the body size, tight leading (0.9-1.05) and slight negative tracking on big type, body 16-20px at 45-75 characters a line, at most 6 sizes. No habitual default face for display.",
    "Colour: one ground, one ink, one accent, plus at most 2-3 supporting tokens. Secondary text is a tint of the ground's hue, not neutral grey; body contrast 4.5:1 or more. Pick light or dark from where the page is read, not from the industry cliché.",
    "Layout: a real grid with deliberate asymmetry, sections that change rhythm, more space above a heading than below, generous gaps between sections (96-200px desktop). Headlines at most 2-3 lines.",
    "Every border, label, number and divider must carry information. Numbers only for real sequences; one radius and one rule weight across the page.",
    "Copy in the customer's words: concrete nouns, real numbers, places, names. Buttons say the exact action ('Get a quote for my wall'). Sentence case.",
    "Motion: one authored moment tied to the subject, plus quick feedback when the user acts. Strong ease-out, UI motion under 300ms, exits faster than entrances, only transform and opacity, hover only on (hover:hover). Content is visible before any script runs and under reduced motion.",
    "Imagery: real or brand photos and footage of the subject; if none exist, a crafted render or diagram of the real object, never stock people, SVG people or emoji icons.",
    "Finish: themed selection, focus ring, caret and scrollbar; tabular figures for data; hover, focus, active, disabled, empty and error states all designed.",
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
    "The same fade-and-rise on every section; hover lifts on every card.",
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
        let _ = writeln!(m, "\n## The look: {}", title(d));
        let _ = writeln!(m, "One whole-page look. Build every section in it. The ideas below are a starting point: keep what fits this brief, change what does not, and the user's own asks always win.\n");
        let _ = writeln!(m, "- Composition: {}", clip(s(d, "layout_composition"), 360));
        let _ = writeln!(m, "- Content: {}", clip(s(d, "content_treatment"), 200));
        let p = &d["palette"];
        let _ = writeln!(
            m,
            "- Palette: ground {} · surface {} · ink {} · muted {} · accent {} · accent 2 {} (body contrast {}:1)",
            s(p, "background"),
            s(p, "surface"),
            s(p, "foreground"),
            s(p, "muted"),
            s(p, "primary_accent"),
            s(p, "secondary_accent"),
            p["wcag_body_contrast"]
        );
        let t = &d["typography"];
        let _ = writeln!(m, "- Type: {} for display, {} for body; scale {}. Fonts: {}", s(t, "display_font"), s(t, "body_font"), s(t, "scale_ratio"), s(t, "google_fonts_url"));
        let tt = &d["type_tokens"];
        if tt.is_object() {
            let _ = writeln!(m, "- Sizes: display {} · h2 {} · body {} · small {}", s(tt, "display"), s(tt, "h2"), s(tt, "body"), s(tt, "small"));
        }
        let _ = writeln!(m, "- Shape: radius {}; hero material: {}", s(d, "radius"), s(d, "hero_material"));
        let ps = &d["page_shape"];
        for (label, key) in [("Hero idea", "hero_frame"), ("Page flow", "flow"), ("Grounds", "ground"), ("Motif", "motif"), ("Labels", "labels")] {
            let b = s(&ps[key], "build");
            if !b.is_empty() {
                let _ = writeln!(m, "- {label}: {}", clip(b, 220));
            }
        }
        let _ = writeln!(m, "- Motion: {}", clip(s(d, "motion_choreography"), 300));
        let nav = s(&d["nav"], "name");
        let foot = s(&d["footer"], "name");
        if !nav.is_empty() || !foot.is_empty() {
            let _ = writeln!(m, "- Header idea: {nav}. Footer idea: {foot}.");
        }
        let mech: Vec<&str> = d["mechanics"].as_array().map(|a| a.iter().map(|x| s(x, "name")).filter(|n| !n.is_empty()).collect()).unwrap_or_default();
        if !mech.is_empty() {
            let _ = writeln!(m, "- Section ideas: {}.", mech.join("; "));
        }
        let pinned = list(&ub["pinned_by_prompt"]);
        if !pinned.is_empty() {
            let _ = writeln!(m, "- Pinned by the user's words: {}.", pinned.join(", "));
        }
    }

    if dirs.len() > 1 {
        let _ = writeln!(m, "\n## Other looks (only if the user wants options)");
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
    let _ = writeln!(m, "1. Write the 5-line plan (first taste rule). Change any part of the look above that fights it; say what you changed.");
    let _ = writeln!(m, "2. Build the page in one pass in the project's own framework, with real copy for this business.");
    let _ = writeln!(m, "3. Screenshot at 1440px and 390px. Fix the three worst problems (generic, crowded, broken), at most two rounds.");
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
        assert!(md.contains("## The look: Counter") && md.contains("The morning board") && md.contains("Flour dust."));
        assert!(md.contains("- Map:") && md.contains("A.nav (nav-x)"));
        assert!(md.contains("…"), "long composition is clipped");
        assert!(md.len() < 6000, "{}", md.len());
    }
}
