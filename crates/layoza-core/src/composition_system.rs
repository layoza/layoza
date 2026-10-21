//! Executable static lockups and concept-specific identity roles. These are
//! editable starting compositions, not an aesthetic gate or a house template.
use crate::intelligence::CreativeDirection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionSystem {
    pub identity_anchor: String,
    pub layout_family: String,
    pub identity_study: IdentityStudy,
    pub typography_relationship: String,
    #[serde(default)]
    pub typography_study: TypographyStudy,
    pub signature_application: Vec<String>,
    pub static_lockup_html: String,
    pub static_lockup_css: String,
    pub visual_review: String,
}

/// Editable, real-copy type specimens across three surfaces. This is evidence
/// to render and critique, not a claim that a font pairing creates an identity.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypographyStudy {
    pub status: String,
    pub display_family: String,
    pub body_family: String,
    pub specimen_html: String,
    pub role_css: String,
    pub inspection: Vec<String>,
}

fn explicit_headline(query: &str) -> Option<String> {
    quoted_copy(query, &["exact hero headline:", "exact headline:", "headline:"]).or_else(|| {
        let mut lines = query.lines();
        while let Some(line) = lines.next() {
            let label = line.trim().trim_start_matches('#').trim().trim_end_matches(':').to_ascii_lowercase();
            if ["headline concept", "hero headline", "exact headline", "main headline"].contains(&label.as_str()) {
                let next = lines.by_ref().find(|l| !l.trim().is_empty())?.trim();
                for (open, close) in [('“', '”'), ('"', '"')] {
                    if let Some(value) = next.strip_prefix(open).and_then(|v| v.strip_suffix(close)) {
                        return Some(value.into());
                    }
                }
            }
        }
        None
    })
}

fn typography_study(d: &CreativeDirection, headline: &str, subject: &str, action: &str, material: &[String]) -> TypographyStudy {
    let family = |s: &str| serde_json::to_string(&s.replace(['\r', '\n'], " ")).unwrap();
    let specimen = material.first().map(String::as_str).unwrap_or(subject);
    TypographyStudy {
        status: "Proposed specimens; actual fonts, copy fit and cross-surface identity require rendered review.".into(),
        display_family: d.typography.display_font.clone(), body_family: d.typography.body_font.clone(),
        specimen_html: format!("<div class=\"type-study\"><header data-type-surface=\"opening\"><h1 class=\"type-study-display\">{}</h1><p class=\"type-study-body\">{}</p><a class=\"type-study-control\" href=\"#contact\">{}</a></header><section data-type-surface=\"ordinary\"><h2 class=\"type-study-heading\">{}</h2><p class=\"type-study-meta\">{}</p></section><section data-type-surface=\"enquiry\"><h2 class=\"type-study-heading\">{}</h2><a class=\"type-study-control\" href=\"#contact\">{}</a></section></div>", html(headline), html(subject), html(action), html(specimen), html(subject), html(action), html(action)),
        role_css: format!(".type-study{{--type-display:{};--type-body:{}}}.type-study-display,.type-study-heading{{font-family:var(--type-display);font-weight:500;text-wrap:balance;margin:0}}.type-study-display{{font-size:clamp(2.5rem,6vw,6rem);line-height:1.03;letter-spacing:-.025em}}.type-study-heading{{font-size:clamp(2rem,4vw,4rem);line-height:1.08;letter-spacing:-.02em}}.type-study-body{{font:400 1.125rem/1.6 var(--type-body);max-width:48ch}}.type-study-control{{font:500 1rem/1.25 var(--type-body);min-height:2.75rem;display:inline-flex;align-items:center;padding:.75rem 1.25rem}}.type-study-meta{{font:400 .875rem/1.5 var(--type-body)}}.type-study>[data-type-surface]{{display:grid;gap:1.25rem;padding-block:2rem;min-width:0}}", family(&d.typography.display_font), family(&d.typography.body_font)),
        inspection: vec![
            "Load the actual font files and verify their supported weights. Pinned families win; these CSS sizes are editable starts, not a finished composition.".into(),
            "Render opening, an actual ordinary section and enquiry together. Choose a recognizable relationship of type, artwork and space; the font name alone is not the identity.".into(),
            "At 320/390/768/1440 inspect exact line breaks, clipping, paragraph measure, captions and control labels. Keep controls readable without inheriting display compression or tracking.".into(),
            "With motion paused, compare against the reference using the same brief and assets. Retain actual improvements and tradeoffs; do not infer quality from role counts or this specimen.".into(),
        ],
    }
}

fn layout_family(archetype: &str) -> &'static str {
    #[derive(Deserialize)]
    struct Row { key: String, composition: String }
    static ROWS: std::sync::OnceLock<Vec<Row>> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| serde_json::from_str(include_str!("directions/archetypes.json")).expect("valid archetype structures"))
        .iter().find(|r| r.key == archetype).map(|r| r.composition.as_str()).unwrap_or("custom")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityStudy {
    pub status: String,
    pub supplied_material: Vec<String>,
    pub compositional_operation: String,
    pub transfer: Vec<String>,
    pub falsification: String,
}

fn identity_study(index: usize, query: &str) -> IdentityStudy {
    let supplied_material: Vec<String> = query.lines()
        .filter_map(|l| l.trim().strip_prefix("Subject objects:"))
        .flat_map(|l| l.split(',')).map(str::trim).filter(|s| !s.is_empty())
        .map(str::to_owned).collect();
    let operation = match index {
        2 => "Relationship: use the actual relationship between two supplied subjects to organize adjacent masses, scale and the reading sequence. Do not merely place two generic cards beside each other.",
        3 => "Transformation: make an actual process legible through three distinct spatial states. Its completed static state owns the composition; motion explains the change rather than supplying the identity alone.",
        _ => "Material: isolate a recognizable proportion, edge, aperture or working detail in supplied material; use it to organize the image/type relationship rather than add a decorative logo to a generic page.",
    };
    IdentityStudy {
        status: if supplied_material.is_empty() { "Unresolved: no explicit subject material supplied; obtain evidence before asserting a brand identity." } else { "Proposed study using caller-supplied material; not a verified unique identity." }.into(),
        supplied_material,
        compositional_operation: operation.into(),
        transfer: vec![
            "Opening: demonstrate the chosen relationship at page scale with the actual headline, artwork and next action.".into(),
            "Ordinary section: translate that relationship into content grouping or image cropping; preserve legible body text and familiar controls.".into(),
            "Enquiry: retain the recognizable proportion or material relationship while simplifying the task. A recoloured default form is not evidence of transfer.".into(),
        ],
        falsification: "Compare actual opening, body and enquiry renders with brand names masked. If only colour, font or a new symbol distinguishes them from another client, the identity study remains unresolved. Never infer a pass from generated tokens or this prose.".into(),
    }
}

fn html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}

/// Read an explicitly labelled quoted copy string, without rewriting its words.
fn quoted_copy(query: &str, labels: &[&str]) -> Option<String> {
    let lower = query.to_ascii_lowercase();
    for label in labels {
        let Some(start) = lower.find(label) else { continue };
        // Labels are ASCII; ASCII lowercasing does not move this offset.
        let tail = query.get(start + label.len()..)?.trim_start();
        for (open, close) in [('“', '”'), ('"', '"'), ('‘', '’')] {
            if let Some(rest) = tail.strip_prefix(open) {
                if let Some(end) = rest.find(close) { return Some(rest[..end].to_string()); }
            }
        }
    }
    None
}

pub fn composition_system(d: &CreativeDirection, subject: &str, action: &str, query: &str) -> CompositionSystem {
    let headline = explicit_headline(query)
        .unwrap_or_else(|| subject.into());
    let support = quoted_copy(query, &["exact support:", "supporting copy:", "support:"]);
    let font = |name: &str| serde_json::to_string(&name.replace(['\r', '\n'], " ")).unwrap();
    let family = layout_family(&d.archetype);
    let relationship = if d.typography.display_font == d.typography.body_font {
        "One type voice; identity must come from authored proportions, material and behavior rather than an unrequested second face"
    } else {
        "The selected display face carries the visual statement; the selected body face carries readable explanation and controls"
    };
    let (size, leading, tracking, measure) = match family {
        "poster" | "full-bleed" => ("clamp(3rem,8vw,8rem)", ".98", "-.035em", "16ch"),
        "split" | "object-wrap" => ("clamp(2.75rem,6.6vw,6.5rem)", "1.03", "-.025em", "17ch"),
        _ => ("clamp(3rem,6vw,6rem)", "1.02", "-.025em", "18ch"),
    };
    let motif = d.page_shape.as_ref().map(|p| p.motif.build.as_str()).filter(|s| !s.is_empty()).unwrap_or(&d.visual_identity);
    let support_html = support.map(|s| format!("<p class=\"identity-support\">{}</p>", html(&s))).unwrap_or_default();
    let mut css = format!(
        ":root{{--identity-display:{};--identity-body:{}}}\n.identity-lockup{{display:grid;justify-items:start;gap:1.25rem;max-width:72rem}}\n.identity-label{{font:500 .875rem/1.4 var(--identity-body);letter-spacing:.04em;margin:0}}\n.identity-title{{font-family:var(--identity-display);font-size:{size};font-weight:500;line-height:{leading};letter-spacing:{tracking};max-width:{measure};text-wrap:balance;margin:0}}\n.identity-support{{font:400 1.125rem/1.55 var(--identity-body);max-width:48ch;margin:0}}\n.identity-actions{{display:flex;flex-wrap:wrap;align-items:center;gap:1rem}}\n.identity-actions>a{{min-height:3.25rem}}\n.identity-art{{position:relative;min-width:0;isolation:isolate}}\n.identity-caption{{font:400 .875rem/1.5 var(--identity-body);margin-block-start:1rem}}\n@media(max-width:40rem){{.identity-lockup{{gap:1rem}}.identity-title{{font-size:clamp(2.5rem,10vw,3.5rem);max-width:100%}}.identity-support{{font-size:1rem}}.identity-actions{{gap:.75rem}}}}",
        font(&d.typography.display_font), font(&d.typography.body_font)
    );
    // The chosen composition owns its prototype. Renumbering a direction must
    // not silently change its layout or impose a different typography concept.
    css.push_str(match family {
        "split" => "\n@media(min-width:60rem){.identity-lockup{max-width:none;grid-template-columns:minmax(0,1.5fr) minmax(16rem,1fr);column-gap:5vw}.identity-title{grid-column:1;grid-row:1/4}.identity-label,.identity-support,.identity-actions{grid-column:2}}",
        "object-wrap" | "layered" => "\n@media(min-width:60rem){.identity-lockup{max-width:none;grid-template-columns:minmax(12rem,.65fr) minmax(0,2fr);column-gap:5vw}.identity-label{grid-column:1;grid-row:1/3}.identity-title,.identity-support,.identity-actions{grid-column:2}.identity-title{max-width:22ch}}",
        "centred" => "\n.identity-lockup{margin-inline:auto;justify-items:center;text-align:center}.identity-actions{justify-content:center}",
        "poster" | "full-bleed" => "\n.identity-lockup{max-width:none;width:100%}.identity-title{max-width:15ch}.identity-support{max-width:40ch}",
        "sticky-stage" => "\n@media(min-width:60rem){.identity-lockup{max-width:42rem}.identity-art{position:sticky;top:6rem;align-self:start}}",
        _ => "\n@media(min-width:60rem){.identity-lockup{max-width:60rem}.identity-title{max-width:15ch}.identity-support{max-width:40ch}.identity-actions{margin-block-start:.5rem}}",
    });
    let study = identity_study(d.index as usize, query);
    let type_study = typography_study(d, &headline, subject, action, &study.supplied_material);
    CompositionSystem {
        layout_family: family.into(),
        identity_anchor: format!("Proposed direction for {}: {}. Verify against supplied material; this catalogue label is not an established brand identity.", subject, d.visual_identity),
        identity_study: study,
        typography_study: type_study,
        typography_relationship: format!("{relationship}. Suggested roles: {} display / {} body. Actual local font availability and explicit brand choices win.", d.typography.display_font, d.typography.body_font),
        signature_application: vec![
            format!("Candidate signature from this direction: {motif}. Edit or replace it when actual artwork makes it implausible; do not add every dealt accessory."),
            "Demonstrate the same chosen proportion/framing/type relationship in the hero, an ordinary work/content section and the enquiry state. Evolve its role with the content; do not paste the same section three times.".into(),
        ],
        static_lockup_html: format!("<header class=\"identity-lockup\"><p class=\"identity-label\">{}</p><h1 class=\"identity-title\">{}</h1>{support_html}<div class=\"identity-actions\"><a class=\"cta cta--primary\" href=\"#contact\">{}</a></div></header>", html(subject), html(&headline), html(action)),
        static_lockup_css: css,
        visual_review: "The fragment is an editable copy/typography starting point, not a finished hero. Compose it with the real artwork and selected visual_system controls. Compare three paused desktop/phone frames with the actual labels: subject, proposition, useful next action and project context must read without motion. Judge identity again in an ordinary section with the brand name hidden. A different font, accent or button radius alone does not establish a distinct identity; a familiar control can be correct. Check animated frames separately for collisions and preserve each control's reserved area.".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn renumbering_does_not_restyle_the_selected_composition() {
        let r = crate::intelligence::recommend_design("Architecture concept studio", crate::intelligence::OperatingMode::NewWebsite, None, None);
        let mut d = r.directions[0].clone();
        d.archetype = "workbench-split".into();
        let a = composition_system(&d, "studio", "Discuss a project", "");
        d.index = 3;
        let b = composition_system(&d, "studio", "Discuss a project", "");
        assert_eq!(a.layout_family, "split");
        assert_eq!(a.static_lockup_css, b.static_lockup_css);
        assert_eq!(a.typography_relationship, b.typography_relationship);
        d.archetype = "full-bleed-stage".into();
        let c = composition_system(&d, "studio", "Discuss a project", "");
        assert_ne!(b.static_lockup_css, c.static_lockup_css);
    }
    #[test]
    fn identity_evidence_does_not_promote_category_guesses_to_facts() {
        let empty = identity_study(1, "A clean corporate website");
        assert!(empty.supplied_material.is_empty());
        assert!(empty.status.starts_with("Unresolved"));
        let q = "Industry: architecture\nSubject objects: Grove Court courtyard, Tide Room listening space";
        let studies: Vec<_> = (1..=3).map(|i| identity_study(i, q)).collect();
        assert_eq!(studies[0].supplied_material, vec!["Grove Court courtyard", "Tide Room listening space"]);
        assert_ne!(studies[0].compositional_operation, studies[1].compositional_operation);
        assert_ne!(studies[1].compositional_operation, studies[2].compositional_operation);
        assert!(studies.iter().all(|s| s.status.contains("not a verified unique identity")));
    }
    #[test]
    fn exact_copy_survives_and_untrusted_markup_is_not_emitted() {
        let q = "Exact hero headline: “Light & <space>.” Exact support: “Rooms for everyday life.”";
        assert_eq!(quoted_copy(q, &["exact hero headline:"]).as_deref(), Some("Light & <space>."));
        assert_eq!(html("Light & <space>."), "Light &amp; &lt;space&gt;.");
        assert_eq!(quoted_copy("A gallery without a specified headline", &["headline:"]), None);
        assert_eq!(quoted_copy("İΧ Exact headline: “Light.”", &["exact headline:"]).as_deref(), Some("Light."));
    }
    #[test]
    fn full_markdown_brief_keeps_hero_copy_instead_of_contact_headline() {
        let q = include_str!("test_fixtures/agency-full-brief.txt");
        assert_eq!(explicit_headline(q).as_deref(), Some("WE DON'T JUST BUILD WEBSITES."));
    }
    #[test]
    fn type_specimen_preserves_pinned_families_and_escapes_real_copy() {
        let r = crate::intelligence::recommend_design("A gallery website", crate::intelligence::OperatingMode::NewWebsite, None, None);
        let mut d = r.directions[0].clone();
        d.typography.display_font = "Client Display".into(); d.typography.body_font = "Client Text".into();
        let q = "Exact headline: \"Light & <space>\"\nSubject objects: Tide Room & <wall>";
        let s = composition_system(&d, "Gallery", "Discuss <project>", q).typography_study;
        assert!(s.role_css.contains("Client Display") && s.role_css.contains("Client Text"));
        for surface in ["opening", "ordinary", "enquiry"] { assert!(s.specimen_html.contains(&format!("data-type-surface=\"{surface}\""))); }
        assert!(s.specimen_html.contains("Light &amp; &lt;space&gt;"));
        assert!(s.specimen_html.contains("Tide Room &amp; &lt;wall&gt;"));
        assert!(!s.specimen_html.contains("<wall>"));
    }
    #[test]
    fn earlier_saved_composition_without_type_specimen_still_loads() {
        let r = crate::intelligence::recommend_design("Gallery", crate::intelligence::OperatingMode::NewWebsite, None, None);
        let study = composition_system(&r.directions[0], "Gallery", "Enquire", "");
        let mut saved = serde_json::to_value(study).unwrap();
        saved.as_object_mut().unwrap().remove("typography_study");
        let restored: CompositionSystem = serde_json::from_value(saved).unwrap();
        assert!(restored.typography_study.specimen_html.is_empty());
        assert!(!restored.static_lockup_html.is_empty());
    }
}
