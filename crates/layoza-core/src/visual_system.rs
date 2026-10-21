//! Palette roles and usable action styles. Contrast is measured; colour harmony
//! with real artwork remains a rendered decision, not an invented score.
use crate::intelligence::{wcag_contrast, PaletteRecommendation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSystem {
    pub canvas: String,
    pub surface: String,
    pub ink: String,
    pub action_fill: String,
    pub on_action: String,
    pub hover_fill: String,
    pub focus: String,
    pub action_text_contrast: f64,
    pub shape: String,
    pub control_radius: String,
    pub control_css: String,
    pub control_spec: String,
    pub composition_review: String,
}

fn legible_ink(background: &str, preferred: &str) -> String {
    if wcag_contrast(preferred, background).is_some_and(|c| c >= 4.5) {
        return preferred.into();
    }
    ["#000000", "#FFFFFF"].into_iter()
        .max_by(|a, b| wcag_contrast(a, background).unwrap_or(0.0)
            .total_cmp(&wcag_contrast(b, background).unwrap_or(0.0)))
        .unwrap().into()
}

fn hover_fill(fill: &str, ink: &str) -> String {
    let h = fill.trim_start_matches('#');
    if h.len() != 6 || !h.is_ascii() { return fill.into(); }
    let target = if wcag_contrast(ink, "#FFFFFF").unwrap_or(0.0) >= wcag_contrast(ink, "#000000").unwrap_or(0.0) { 255.0 } else { 0.0 };
    let mut out = String::from("#");
    for i in [0, 2, 4] {
        let Ok(channel) = u8::from_str_radix(&h[i..i+2], 16) else { return fill.into(); };
        out.push_str(&format!("{:02X}", (f64::from(channel) * 0.92 + target * 0.08).round() as u8));
    }
    if wcag_contrast(ink, &out).is_some_and(|c| c >= 4.5) { out } else { fill.into() }
}

pub fn visual_system(p: &PaletteRecommendation, shape: &str) -> VisualSystem {
    let fill = p.primary_accent.clone();
    let on_action = legible_ink(&fill, &p.foreground);
    let hover_fill = hover_fill(&fill, &on_action);
    let focus = if wcag_contrast(&fill, &p.background).is_some_and(|c| c >= 3.0) {
        fill.clone()
    } else { legible_ink(&p.background, &p.foreground) };
    let radius = match shape {
        "pill" | "stamp-circle" => "999px",
        "soft-square" | "soft-12" => "12px",
        _ => "0px",
    };
    let control_css = format!(r#":root{{--action-fill:{fill};--on-action:{on_action};--action-ink:{ink};--action-focus:{focus};--control-radius:{radius}}}
.cta{{box-sizing:border-box;display:inline-flex;align-items:center;justify-content:center;gap:.75rem;min-height:52px;padding:12px 24px;border:1px solid transparent;border-radius:var(--control-radius);font:600 1rem/1.25 var(--font-body,system-ui);text-decoration:none;cursor:pointer;transition:background-color .18s,color .18s}}
.cta--primary{{background:var(--action-fill);color:var(--on-action);border-color:var(--on-action)}}
.cta--secondary{{background:transparent;color:var(--action-ink);text-decoration:underline;text-underline-offset:.3em}}
.cta:focus-visible{{outline:3px solid var(--action-focus);outline-offset:4px;box-shadow:0 0 0 4px var(--ground,{canvas})}}
.cta[disabled],.cta[aria-disabled=true]{{cursor:not-allowed;text-decoration:none;border-style:dashed}}
@media(hover:hover) and (pointer:fine){{.cta--primary:not(:disabled):not([aria-disabled=true]):hover{{background:var(--action-hover,{hover_fill})}}.cta--secondary:not(:disabled):not([aria-disabled=true]):hover{{text-decoration-thickness:2px}}}}
.cta:not(:disabled):not([aria-disabled=true]):active{{text-decoration:underline}}
@media(prefers-reduced-motion:reduce){{.cta{{transition:none}}}}"#,
        ink=p.foreground,canvas=p.background);
    VisualSystem {
        canvas: p.background.clone(), surface: p.surface.clone(), ink: p.foreground.clone(),
        action_text_contrast: wcag_contrast(&on_action, &fill).unwrap_or(0.0),
        action_fill: fill, on_action, hover_fill, focus, shape: shape.into(), control_radius: radius.into(), control_css,
        control_spec: format!("Editable control starting point: primary filled with action_fill/on_action; secondary transparent with an underlined label in action_ink. Both use control_radius ({radius}), minimum height 52px and 12px 24px padding. Hover, focus and reduced-motion behavior are defined by control_css. Use this one specification for prose and CSS; if the selected concept or explicit brief changes a treatment, update its tokens, CSS and description together. The motif does not prescribe button geometry. Navigation actions are anchors; state changes are buttons."),
        composition_review: "Use an intentional canvas/surface family and an action accent as a starting relationship, not a universal colour quota. Secondary palette accents are optional, not automatic extra UI colours. Inspect actual supplied art beside the page ground and controls: independent portfolio colours may stay inside neutral or related frames without becoming interface accents. Bold saturated grounds are valid when they support the concept; do not default every site to off-white/charcoal/blue. Compare colour-area balance and CTA visibility in the actual composition and hover/focus states. Contrast establishes readability only, not harmony. Preserve explicit brand tokens and incumbent controls. Shape is a suggestion, not a novelty requirement; custom cutouts need an unclipped focus/hit wrapper. aria-disabled requires corresponding native behavior.".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn action_pairs_stay_readable_when_accent_changes() {
        let mut p = crate::intelligence::recommend_design("design agency", crate::intelligence::OperatingMode::NewWebsite, None, None).directions[0].palette.clone();
        for accent in ["#DBF7B6", "#2834CF", "#FFFF00", "#777777", "#FFFFFF", "#111111"] {
            p.primary_accent = accent.into();
            let roles = visual_system(&p, "soft-square");
            assert_eq!(roles.action_fill, accent);
            assert!(roles.action_text_contrast >= 4.5, "{accent}: {}", roles.action_text_contrast);
            assert!(wcag_contrast(&roles.on_action, &roles.hover_fill).unwrap() >= 4.5);
            assert!(wcag_contrast(&roles.focus, &roles.canvas).unwrap() >= 3.0);
            assert!(roles.control_css.contains(&roles.on_action));
        }
    }
}
