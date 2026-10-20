//! The short design brief: signals to decide from, one suggested look and a rubric, as Markdown.
//!
//! This is what `get_design_system` returns by default. The model designs the
//! whole page in one pass from it; the dealt parts are ideas, not mandates.

use serde_json::Value;
use std::fmt::Write as _;

/// What the agent decides before building; Layoza only reads the signals.
pub const DECIDE: &[&str] = &[
    "MOTION: 0-5 from site type, audience, task urgency and asset strength. Guide: agency or studio, or 'award, bold' 4-5; portfolio 3-5; brand launch 4 on the story, 2 in the shop; SaaS 2-3; restaurant or local 1-2 (luxury 3); docs, gov, health 0-2. Weak assets or mostly-phone visitors: keep type motion, drop WebGL. The user's words override.",
    "CONCEPTS: three from this brief's own facts (people, place, materials, how the work is made or used), scored 0-10 for originality and fit; keep the winner and say why. Genericness test: if an average AI would make it for any business in the trade, push it further.",
    "SIGNATURE: one mechanic this subject owns. Then pick techniques for your level from references/decide.md (each says when it fits); none is required.",
];

/// Rubric questions. The agent and a blind critic score each 0-10 from screenshots.
pub const TASTE: &[&str] = &[
    "Subject: does the first screen show the trade's own object, place, work or product at scale, so a stranger names the business with the words covered?",
    "Proof: does each section make one claim with proof (a picture, a sourced number, a real name)?",
    "Type: one family or two clearly different; display 4-6x body, headlines broken by hand; body 16-20px at 45-75ch?",
    "Colour and depth: ground, ink, accent and 2-3 tokens, quiet or immersive by choice; one depth system (photo light, overlapping planes or tonal fields); body contrast 4.5:1+?",
    "Layout: 5-7 sections whose rhythm changes; two or more break their frame; neighbours never place the heading alike; no blank stretch over half a screen?",
    "Motion: does the amount match your level? 0-1 feedback only; 2 reveals and one scroll scene; 3 a living hero plus scroll scenes; 4-5 motion on every screen (split headings, pinned runs, moving work, smooth scroll, cursor-led previews, transitions, WebGL where it serves). One easing family, transform/opacity/clip-path, 60fps on a phone; reduced motion and no JS show the final frame.",
    "Imagery: real photos of the subject or crafted renders, four or more, one full-bleed, work shown big?",
    "Finish: the customer's words, exact button labels, focus ring, every state designed; main action above the fold at 390px, 44px taps, no sideways scroll?",
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
    "A heavy-motion kit with no idea behind it: 0-100 preloader, blob cursor reading 'View', stacked sticky cards, RGB-split hover, every heading scrubbed letter by letter.",
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
    let subject = s(ub, "subject");
    let _ = writeln!(m, "# Design brief: {}\n", if subject.is_empty() { "this site" } else { subject });

    let _ = writeln!(m, "## Who it is for");
    for (label, key) in [("Trade", "trade"), ("Audience", "audience"), ("Main action", "primary_action"), ("Stack", "stack")] {
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

    let _ = writeln!(m, "\n## Decide first (you choose; write each in .layoza/direction.md)");
    let motion = s(ub, "motion_wish");
    for r in DECIDE {
        let _ = writeln!(m, "- {r}");
    }
    if !motion.is_empty() {
        let _ = writeln!(m, "- Layoza's read of the niche, for you to confirm or change: motion {motion}.");
    }

    if let Some(d) = dirs.first() {
        let _ = writeln!(m, "\n## The signature moment (build this first)");
        let _ = writeln!(m, "The set piece the page is remembered for: what this subject does that nothing else does (how it is made, used, moves, scales, or changes with distance, time or light), staged as a scene the visitor drives by scroll or hand, one to two screens, built from the real subject. It must fail for a rival business.");
        let lead = s(d, "motion_choreography");
        if !lead.is_empty() {
            let _ = writeln!(m, "- Starting idea: {}", clip(lead, 300));
        }
        let _ = writeln!(m, "- Done when: start, middle and end frames each read as a poster, it is smooth on a phone, and the final frame works without JavaScript.");

        let _ = writeln!(m, "\n## A suggested look: {}", title(d));
        let _ = writeln!(m, "Options, not orders: take, change or drop any line so it serves your concept. The user's own asks always win.\n");
        let _ = writeln!(m, "- Composition: {}", clip(s(d, "layout_composition"), 260));
        let _ = writeln!(m, "- Content: {}", clip(s(d, "content_treatment"), 200));
        let _ = writeln!(m, "- Shape: radius {}; hero material: {}", s(d, "radius"), s(d, "hero_material"));
        let _ = writeln!(m, "- Fonts: {} (body contrast {}:1)", s(&d["typography"], "google_fonts_url"), d["palette"]["wcag_body_contrast"]);
        tokens_css(&mut m, d);
        if neon_on_black(s(&d["palette"], "background"), s(&d["palette"], "primary_accent")) {
            let _ = writeln!(m, "- Check: this palette is near-black with one neon accent, which the avoid list bans. Give the ground a colour of its own or let the photography carry the colour.");
        }
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

    let _ = writeln!(m, "\n## Rubric (you, then a blind critic, score each 0-10 from screenshots; fix anything under 8)");
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
        // Only the suggested look's slots: the other looks are listed by name, not built.
        let rep: Vec<String> = slots
            .iter()
            .filter(|x| s(x, "status").contains("repeat") && !s(x, "slot").starts_with("B.") && !s(x, "slot").starts_with("C."))
            .map(|x| format!("{} ({})", s(x, "slot").trim_start_matches("A."), s(x, "id")))
            .collect();
        if !rep.is_empty() {
            let _ = writeln!(m, "- Repeats a recent run, change these: {}", rep.join(", "));
        }
    }

    let _ = writeln!(m, "\n## How to build");
    let _ = writeln!(m, "1. Write .layoza/direction.md from \"Decide first\": MOTION, three concepts, PICKED, SIGNATURE, SECTIONS. Change any part of the suggested look that fights your concept; say what you changed.");
    let _ = writeln!(m, "2. Build the signature moment first, then the rest of the page around it in one pass at your motion level, in the project's own framework, with real copy for this business.");
    let _ = writeln!(m, "3. Screenshot the signature at start, middle and end, then the page at 1440px and 390px, and score the rubric. Swap two section headlines: if both still work, rebuild one.");
    let _ = writeln!(m, "4. Blind critic: a fresh subagent that sees only this brief and the screenshots names the business, scores the rubric and lists the three weakest spots. Fix anything under 8; log `ROUND n: <what changed>`, two rounds at most.");
    let _ = writeln!(m, "5. Run `capture_screenshots` (or `layoza qa <url>`) and fix its errors. Done.");
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
        assert!(md.contains("## The signature moment") && md.contains("## A suggested look: Counter") && md.contains("## Decide first") && md.contains("The morning board") && md.contains("Flour dust."));
        assert!(md.contains(":root {\n  --ground: #FFF;") && md.contains("--font-display: \"Fraunces\""));
        assert!(md.contains("- Map:") && md.contains("- Repeats a recent run, change these: nav (nav-x)"));
        assert!(!md.contains("…") && md.contains("- Composition: The counter at dawn. The counter"), "long text keeps whole sentences");
        assert!(md.len() < 6000, "{}", md.len());
        // The agent decides the motion level; no rule fixes it to quiet.
        assert!(md.contains("MOTION: 0-5") && !md.contains("Elsewhere quiet") && !md.contains("keep the interface quiet"));
    }
}
