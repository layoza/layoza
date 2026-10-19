// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Compatible implementation of the Google DESIGN.md specification (Apache-2.0 spec compatibility):
// - Generates normative YAML frontmatter + canonical 8-section Markdown prose
// - Lints DESIGN.md files for section ordering, duplicate sections, broken token references,
//   and WCAG 2.2 AA contrast ratios
// - Diffs two DESIGN.md files for token and lint regressions

use crate::dna::SiteDna;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The nine standard DESIGN.md sections in order (numbered headings such as `## 2. Color Palette
/// & Roles` match too). Layoza's own sections (Motion, Variants, Brand, Decisions) follow them.
pub const CANONICAL_SECTIONS: &[(&str, &[&str])] = &[
    ("Visual Theme & Atmosphere", &["Visual Theme & Atmosphere", "Overview", "Brand & Style"]),
    ("Color Palette & Roles", &["Color Palette & Roles", "Colors", "Colours"]),
    ("Typography Rules", &["Typography Rules", "Typography"]),
    ("Component Stylings", &["Component Stylings", "Components"]),
    ("Layout Principles", &["Layout Principles", "Layout", "Layout & Spacing"]),
    ("Depth & Elevation", &["Depth & Elevation", "Elevation & Depth", "Elevation", "Shapes"]),
    ("Do's and Don'ts", &["Do's and Don'ts", "Dos and Donts"]),
    ("Responsive Behavior", &["Responsive Behavior", "Responsive Behaviour", "Responsive"]),
    ("Agent Prompt Guide", &["Agent Prompt Guide"]),
];

/// The earlier eight-section order, still accepted so existing files do not start failing.
const LEGACY_ORDER: &[&str] = &["Overview", "Colors", "Typography", "Layout", "Elevation & Depth", "Shapes", "Components", "Do's and Don'ts"];

/// Strips a leading `N.` / `N)` number from a heading.
fn heading_name(h: &str) -> &str {
    let t = h.trim();
    let rest = t.trim_start_matches(|c: char| c.is_ascii_digit());
    if rest.len() < t.len() {
        rest.trim_start_matches(['.', ')']).trim()
    } else {
        t
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignMdFinding {
    pub severity: String, // "error" | "warning" | "info"
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignMdSummary {
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignMdLintReport {
    pub valid: bool,
    pub findings: Vec<DesignMdFinding>,
    pub summary: DesignMdSummary,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenDelta {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignMdDiffReport {
    pub tokens: BTreeMap<String, TokenDelta>,
    pub before_summary: DesignMdSummary,
    pub after_summary: DesignMdSummary,
    pub regression: bool,
}

/// Generates a Google `DESIGN.md` compliant specification file from `SiteDna`.
pub fn generate_design_md(dna: &SiteDna) -> String {
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str("version: \"alpha\"\n");
    out.push_str(&format!("name: \"{}\"\n", dna.project_name.replace('"', "'")));
    out.push_str(&format!(
        "description: \"Extracted and calibrated design system for {} ({})\"\n",
        dna.project_name.replace('"', "'"),
        dna.framework
    ));

    out.push_str("colors:\n");
    for c in &dna.colors {
        out.push_str(&format!("  {}: \"{}\"\n", sanitize_key(&c.name), c.value));
    }

    out.push_str("typography:\n");
    for t in &dna.typography {
        out.push_str(&format!("  {}:\n", sanitize_key(&t.role)));
        out.push_str(&format!("    fontFamily: \"{}\"\n", t.font_family));
        out.push_str(&format!("    fontSize: \"{}\"\n", t.font_size));
        out.push_str(&format!("    fontWeight: \"{}\"\n", t.font_weight));
        out.push_str(&format!("    lineHeight: \"{}\"\n", t.line_height));
        out.push_str(&format!("    letterSpacing: \"{}\"\n", t.letter_spacing));
    }

    out.push_str("rounded:\n");
    for (k, v) in &dna.radii_scale {
        out.push_str(&format!("  {}: \"{}\"\n", sanitize_key(k), v));
    }

    out.push_str("spacing:\n");
    for (k, v) in &dna.spacing_scale {
        out.push_str(&format!("  {}: \"{}\"\n", sanitize_key(k), v));
    }

    let bg_key = dna
        .colors
        .iter()
        .find(|c| {
            let n = c.name.to_ascii_lowercase();
            n.contains("bg") || n.contains("background") || n.contains("canvas") || n.contains("surface")
        })
        .or_else(|| dna.colors.first())
        .map(|c| sanitize_key(&c.name))
        .unwrap_or_else(|| "background".to_string());
    let primary_key = dna
        .colors
        .iter()
        .find(|c| {
            let k = sanitize_key(&c.name);
            let n = c.name.to_ascii_lowercase();
            k != bg_key
                && (n.contains("ink")
                    || n.contains("fg")
                    || n.contains("text")
                    || n.contains("primary")
                    || n.contains("espresso"))
        })
        .or_else(|| dna.colors.iter().find(|c| sanitize_key(&c.name) != bg_key))
        .map(|c| sanitize_key(&c.name))
        .unwrap_or_else(|| "primary".to_string());
    let radius_key = dna
        .radii_scale
        .keys()
        .next()
        .map(|k| sanitize_key(k))
        .unwrap_or_else(|| "md".to_string());

    out.push_str("components:\n");
    out.push_str("  button-primary:\n");
    out.push_str(&format!("    backgroundColor: \"{{colors.{}}}\"\n", primary_key));
    out.push_str(&format!("    textColor: \"{{colors.{}}}\"\n", bg_key));
    out.push_str(&format!("    rounded: \"{{rounded.{}}}\"\n", radius_key));
    out.push_str("    padding: \"12px\"\n");
    out.push_str("---\n\n");

    out.push_str(&design_md_body(dna, &bg_key, &primary_key));
    out
}

fn color_role_var(name: &str, role: &str) -> String {
    let n = format!("{} {}", name, role).to_ascii_lowercase();
    let r = if n.contains("bg") || n.contains("background") || n.contains("canvas") {
        "background"
    } else if n.contains("surface") || n.contains("card") || n.contains("panel") {
        "surface"
    } else if n.contains("muted") || n.contains("secondary text") || n.contains("mute") {
        "text-muted"
    } else if n.contains("accent") || n.contains("cta") || n.contains("brand") {
        "accent"
    } else if n.contains("border") || n.contains("line") || n.contains("rule") {
        "border"
    } else if n.contains("ink") || n.contains("text") || n.contains("fg") || n.contains("primary") {
        "text"
    } else {
        "extra"
    };
    format!("--color-{}", if r == "extra" { sanitize_key(name) } else { r.to_string() })
}

/// The Markdown body: the nine standard sections in order, then Layoza's own sections.
fn design_md_body(dna: &SiteDna, bg_key: &str, primary_key: &str) -> String {
    let mut out = String::new();
    let dl = &dna.dials;
    let name = &dna.project_name;
    let first_font = |role: &str| dna.typography.iter().find(|t| t.role.to_ascii_lowercase().contains(role)).map(|t| t.font_family.clone());
    let display = first_font("h1").or_else(|| first_font("display")).or_else(|| dna.typography.first().map(|t| t.font_family.clone())).unwrap_or_else(|| "system-ui".into());
    let body = first_font("body").unwrap_or_else(|| display.clone());

    out.push_str("## 1. Visual Theme & Atmosphere\n\n");
    let density = match dl.visual_density { 0..=3 => "airy, gallery-like spacing", 4..=6 => "balanced spacing", _ => "dense, information-first spacing" };
    let variance = match dl.design_variance { 0..=3 => "a strict, symmetric grid", 4..=6 => "a measured grid with occasional breaks", _ => "asymmetric, editorial composition" };
    let motion = match dl.motion_intensity { 0..=2 => "nearly still", 3..=6 => "purposeful transitions", _ => "choreographed, scroll-led motion" };
    out.push_str(&format!(
        "`{}` ({} framework) reads as {}, {} and {}. Display type is {}; body type is {}. Dials: variance {}/10, motion {}/10, density {}/10. Preservation mode: `{}`.\n\n",
        name, dna.framework, variance, density, motion, display, body, dl.design_variance, dl.motion_intensity, dl.visual_density, dna.preservation_mode
    ));

    out.push_str("## 2. Color Palette & Roles\n\n");
    out.push_str("| Token | CSS variable | Hex | Role | Source |\n|---|---|---|---|---|\n");
    for c in &dna.colors {
        out.push_str(&format!("| {} | `{}` | `{}` | {} | `{}` ({:?}) |\n", c.name, color_role_var(&c.name, &c.role), c.value, c.role, c.source, c.tier));
    }
    out.push_str(&format!("\nText `{{colors.{}}}` on `{{colors.{}}}` is the base pair; every text pair keeps at least 4.5:1 contrast.\n\n", primary_key, bg_key));

    out.push_str("## 3. Typography Rules\n\n");
    for t in &dna.typography {
        out.push_str(&format!(
            "- **{}**: `{}` at `{}` (weight `{}`, line-height `{}`, tracking `{}`)\n",
            t.role, t.font_family, t.font_size, t.font_weight, t.line_height, t.letter_spacing
        ));
    }
    out.push_str("- Body measure 45-75 characters; multi-line display headings keep line-height at 1.04 or more; functional text never below 11px.\n\n");

    out.push_str("## 4. Component Stylings\n\n");
    let radius = dna.radii_scale.values().next().cloned().unwrap_or_else(|| "8px".into());
    out.push_str(&format!(
        "- **button-primary**: `{{colors.{}}}` fill, `{{colors.{}}}` label, radius `{}`. States: hover (darken or lift 1 step), focus-visible (2px outline offset 2px, never removed), active (pressed, no layout shift), disabled (reduced contrast plus `aria-disabled`), loading (label kept, spinner beside it).\n",
        primary_key, bg_key, radius
    ));
    out.push_str("- **links**: underlined in body copy; hover and focus-visible change more than colour alone.\n");
    out.push_str("- **inputs**: visible label, 44px target, error text tied with `aria-describedby`, error state not shown by colour alone.\n");
    out.push_str("- **cards / panels**: one radius and one border or shadow style per surface level; no nested cards.\n\n");

    out.push_str("## 5. Layout Principles\n\n");
    out.push_str(&format!("- **Max content widths**: {}\n", if dna.content_max_widths.is_empty() { "`1280px`".to_string() } else { dna.content_max_widths.join(", ") }));
    out.push_str(&format!("- **Grid systems**: {}\n", if dna.grid_systems.is_empty() { "CSS Grid + Flexbox".to_string() } else { dna.grid_systems.join(", ") }));
    out.push_str(&format!("- **Spacing scale**: {}\n", dna.spacing_scale.iter().map(|(k, v)| format!("{k} `{v}`")).collect::<Vec<_>>().join(", ")));
    out.push_str("- Every gap and padding comes from the spacing scale; sections differ in rhythm on purpose, not by accident.\n\n");

    out.push_str("## 6. Depth & Elevation\n\n");
    if dna.shadows.is_empty() {
        out.push_str("- Flat: depth comes from surface colour steps and borders, not shadows.\n");
    }
    for (k, v) in &dna.shadows {
        out.push_str(&format!("- **{}**: `{}`\n", k, v));
    }
    out.push_str(&format!("- **Radii**: {}\n\n", dna.radii_scale.iter().map(|(k, v)| format!("{k} `{v}`")).collect::<Vec<_>>().join(", ")));

    out.push_str("## 7. Do's and Don'ts\n\n");
    out.push_str("- **Do**: reuse these tokens, spacing rhythm and component hierarchy for every new section or page.\n");
    out.push_str("- **Do**: honour `prefers-reduced-motion: reduce` and WCAG 2.2 AA contrast; keep essential text visible without JavaScript.\n");
    out.push_str("- **Don't**: add a second CSS framework, unrequested purple gradients, glass panels or nested cards.\n");
    out.push_str("- **Don't**: invent statistics, reviews or client logos; use honest placeholders until real content exists.\n\n");

    out.push_str("## 8. Responsive Behavior\n\n");
    if dna.breakpoints.is_empty() {
        out.push_str("- Breakpoints: 375px (phone), 768px (tablet), 1440px (desktop); fluid type with `clamp()` between them.\n");
    } else {
        out.push_str(&format!("- Breakpoints: {}\n", dna.breakpoints.iter().map(|(k, v)| format!("{k} `{v}`")).collect::<Vec<_>>().join(", ")));
    }
    out.push_str("- No horizontal scroll at any width; touch targets at least 24px (44px for primary actions); navigation collapses into an accessible menu with focus trapping.\n");
    out.push_str("- Right-to-left pages use logical properties (`margin-inline`, `inset-inline`, `text-align: start`) and mirror the layout.\n\n");

    out.push_str("## 9. Agent Prompt Guide\n\n");
    out.push_str(&format!(
        "When asked to build or change UI for `{}`: read this file first, use only the tokens above (`{}` text on `{}` ground, {} / {}), reuse existing components before adding new ones, keep the motion level at {}/10, and verify with `layoza verify .` before handing back. Quick prompt: \"Build <section> for {} using DESIGN.md tokens, the {} type pairing and its component states; no new colours or fonts.\"\n\n",
        name, primary_key, bg_key, display, body, dl.motion_intensity, name, display
    ));

    // Layoza's own sections follow the nine standard ones.
    out.push_str("## Motion\n\n");
    out.push_str(&format!(
        "- Intensity {}/10. Durations and easings live in MOTION.md when it exists. Every animation honours reduced motion and cleans up on unmount.\n\n",
        dl.motion_intensity
    ));
    out.push_str("## Variants\n\n");
    out.push_str("Token deltas from the base above; anything not listed stays the same.\n\n");
    out.push_str(&format!(
        "- **dark**: `{{colors.{bg}}}` becomes the text colour and `{{colors.{fg}}}` the ground; surfaces step up in lightness instead of using shadows; re-check every text pair at 4.5:1. Strategy found: {}.\n",
        if dna.dark_mode_strategy.is_empty() { "none" } else { dna.dark_mode_strategy.as_str() },
        bg = bg_key,
        fg = primary_key
    ));
    out.push_str("- **light**: the base tokens.\n");
    out.push_str("- **compact**: spacing scale x0.75, body line-height -0.1, control height 36px (targets stay at least 24px).\n");
    out.push_str("- **comfortable**: spacing scale x1.25, body size +1px, control height 48px.\n\n");
    out.push_str("## Brand\n\n");
    out.push_str("- Logo, product images and UI screenshots are the brand's first assets; see BRAND.md and `.layoza/brand-spec.md` when they exist.\n\n");
    out.push_str("## Decisions\n\n");
    if dna.decisions.is_empty() {
        out.push_str("- None recorded yet.\n");
    }
    for d in dna.decisions.iter().take(20) {
        out.push_str(&format!("- {}\n", serde_json::to_string(d).unwrap_or_default().replace('\n', " ")));
    }
    out
}

/// A per-project skill that loads this project's look in any future agent session.
pub fn generate_project_skill_md(dna: &SiteDna) -> String {
    let name = sanitize_key(&dna.project_name);
    let name = if name.is_empty() { "project".to_string() } else { name };
    let colors: Vec<String> = dna.colors.iter().take(8).map(|c| format!("`{}` {} ({})", c.name, c.value, c.role)).collect();
    let fonts: Vec<String> = dna.typography.iter().take(4).map(|t| format!("{}: {}", t.role, t.font_family)).collect();
    format!(
        "---\nname: \"{name}-look\"\ndescription: \"Loads the {proj} visual system (tokens, type, components and motion level) before any UI work on this project.\"\n---\n\n\
         # {proj} look\n\n\
         Use this whenever you build or change UI for {proj}.\n\n\
         1. Read DESIGN.md in the project root; it is the source of truth. This file is only a summary.\n\
         2. Colours: {colors}.\n\
         3. Type: {fonts}.\n\
         4. Motion intensity {motion}/10; honour reduced motion.\n\
         5. Reuse existing components; new values need a DESIGN.md entry first.\n\
         6. Before handing back, run `layoza verify .` and fix or report every failure.\n",
        name = name,
        proj = dna.project_name.replace('"', "'"),
        colors = if colors.is_empty() { "see DESIGN.md".into() } else { colors.join(", ") },
        fonts = if fonts.is_empty() { "see DESIGN.md".into() } else { fonts.join(", ") },
        motion = dna.dials.motion_intensity
    )
}

fn sanitize_key(input: &str) -> String {
    let mut s = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            s.push(ch.to_ascii_lowercase());
        } else if ch.is_whitespace() {
            s.push('-');
        }
    }
    let trimmed = s.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "token".to_string()
    } else {
        trimmed
    }
}

/// Parses hex colors (#RGB or #RRGGBB) into (r, g, b) in 0..=255.
pub fn parse_hex_color(input: &str) -> Option<(u8, u8, u8)> {
    let clean = input.trim().trim_matches('"').trim_matches('\'');
    let hex = clean.strip_prefix('#')?;
    if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
        Some((r, g, b))
    } else if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

fn srgb_channel_to_linear(c: u8) -> f64 {
    let v = (c as f64) / 255.0;
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

pub fn relative_luminance(rgb: (u8, u8, u8)) -> f64 {
    0.2126 * srgb_channel_to_linear(rgb.0)
        + 0.7152 * srgb_channel_to_linear(rgb.1)
        + 0.0722 * srgb_channel_to_linear(rgb.2)
}

/// Computes WCAG 2.x contrast ratio between two RGB colors (1.0 to 21.0).
pub fn wcag_contrast_ratio(c1: (u8, u8, u8), c2: (u8, u8, u8)) -> f64 {
    let l1 = relative_luminance(c1);
    let l2 = relative_luminance(c2);
    let (bright, dark) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (bright + 0.05) / (dark + 0.05)
}

#[derive(Debug, Default)]
struct ParsedDesignMd {
    colors: BTreeMap<String, String>,
    typography: BTreeMap<String, String>,
    rounded: BTreeMap<String, String>,
    spacing: BTreeMap<String, String>,
    components: BTreeMap<String, BTreeMap<String, String>>,
    headings: Vec<String>,
    has_frontmatter: bool,
}

fn parse_design_md_simple(content: &str) -> ParsedDesignMd {
    let mut parsed = ParsedDesignMd::default();
    let lines: Vec<&str> = content.lines().collect();
    let mut idx = 0;
    if lines.first().map(|l| l.trim()) == Some("---") {
        parsed.has_frontmatter = true;
        idx = 1;
        let mut current_group = "";
        let mut current_sub = String::new();
        while idx < lines.len() {
            let line = lines[idx];
            if line.trim() == "---" {
                idx += 1;
                break;
            }
            let indent = line.chars().take_while(|c| *c == ' ').count();
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                if indent == 0 && trimmed.ends_with(':') {
                    current_group = trimmed.trim_end_matches(':');
                    current_sub.clear();
                } else if indent == 2 {
                    if trimmed.ends_with(':') {
                        current_sub = trimmed.trim_end_matches(':').to_string();
                    } else if let Some((k, v)) = trimmed.split_once(':') {
                        let key = k.trim().to_string();
                        let val = v.trim().trim_matches('"').trim_matches('\'').to_string();
                        match current_group {
                            "colors" => {
                                parsed.colors.insert(key, val);
                            }
                            "rounded" => {
                                parsed.rounded.insert(key, val);
                            }
                            "spacing" => {
                                parsed.spacing.insert(key, val);
                            }
                            _ => {}
                        }
                    }
                } else if indent == 4 && !current_sub.is_empty() {
                    if let Some((k, v)) = trimmed.split_once(':') {
                        let key = k.trim().to_string();
                        let val = v.trim().trim_matches('"').trim_matches('\'').to_string();
                        if current_group == "typography" {
                            parsed
                                .typography
                                .entry(current_sub.clone())
                                .and_modify(|e| {
                                    e.push_str(&format!(";{}={}", key, val));
                                })
                                .or_insert_with(|| format!("{}={}", key, val));
                        } else if current_group == "components" {
                            parsed
                                .components
                                .entry(current_sub.clone())
                                .or_default()
                                .insert(key, val);
                        }
                    }
                }
            }
            idx += 1;
        }
    }

    while idx < lines.len() {
        let line = lines[idx].trim();
        if let Some(h) = line.strip_prefix("## ") {
            parsed.headings.push(h.trim().to_string());
        }
        idx += 1;
    }

    parsed
}

fn resolve_token_ref(raw: &str, parsed: &ParsedDesignMd) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        let inner = &trimmed[1..trimmed.len() - 1];
        let (group, key) = inner.split_once('.')?;
        match group {
            "colors" => parsed.colors.get(key).cloned(),
            "rounded" => parsed.rounded.get(key).cloned(),
            "spacing" => parsed.spacing.get(key).cloned(),
            "typography" => parsed.typography.get(key).cloned(),
            _ => None,
        }
    } else {
        Some(trimmed.to_string())
    }
}

/// Validates a `DESIGN.md` file against the Google `DESIGN.md` specification.
pub fn lint_design_md(content: &str) -> DesignMdLintReport {
    let parsed = parse_design_md_simple(content);
    let mut findings = Vec::new();

    if !parsed.has_frontmatter {
        findings.push(DesignMdFinding {
            severity: "error".to_string(),
            path: "frontmatter".to_string(),
            message: "Missing YAML frontmatter delimited by `---` fences.".to_string(),
        });
    }

    if parsed.colors.is_empty() {
        findings.push(DesignMdFinding {
            severity: "warning".to_string(),
            path: "colors".to_string(),
            message: "No color tokens defined in YAML frontmatter.".to_string(),
        });
    }

    // Check duplicate headings & canonical section order
    let mut seen_headings = Vec::new();
    let mut last_canonical_idx: Option<usize> = None;
    // Files in the earlier eight-section order stay valid.
    let legacy_idx: Vec<usize> = parsed.headings.iter().filter_map(|h| LEGACY_ORDER.iter().position(|l| l.eq_ignore_ascii_case(heading_name(h)))).collect();
    let legacy_order_ok = legacy_idx.len() >= 2 && legacy_idx.windows(2).all(|w| w[0] < w[1]);
    for h in &parsed.headings {
        if seen_headings.iter().any(|s: &String| s.eq_ignore_ascii_case(h)) {
            findings.push(DesignMdFinding {
                severity: "error".to_string(),
                path: format!("sections.{}", h),
                message: format!("Duplicate section heading `## {}` is not permitted by DESIGN.md spec.", h),
            });
        }
        seen_headings.push(h.clone());

        let h = heading_name(h);
        if let Some((canon_idx, (canon_name, _))) = CANONICAL_SECTIONS
            .iter()
            .enumerate()
            .find(|(_, (_, aliases))| aliases.iter().any(|a| a.eq_ignore_ascii_case(h)))
        {
            if let Some(prev) = last_canonical_idx {
                if canon_idx < prev && !legacy_order_ok {
                    findings.push(DesignMdFinding {
                        severity: "error".to_string(),
                        path: format!("sections.{}", h),
                        message: format!(
                            "Section `## {}` (canonical `{}`) appears out of canonical DESIGN.md section order.",
                            h, canon_name
                        ),
                    });
                }
            }
            last_canonical_idx = Some(canon_idx);
        }
    }

    // Validate component token references and WCAG contrast
    for (comp_name, props) in &parsed.components {
        for (prop_k, prop_v) in props {
            if prop_v.starts_with('{') && prop_v.ends_with('}') && resolve_token_ref(prop_v, &parsed).is_none() {
                findings.push(DesignMdFinding {
                    severity: "error".to_string(),
                    path: format!("components.{}.{}", comp_name, prop_k),
                    message: format!("Broken token reference `{}`.", prop_v),
                });
            }
        }

        if let (Some(bg_raw), Some(fg_raw)) = (props.get("backgroundColor"), props.get("textColor")) {
            if let (Some(bg_val), Some(fg_val)) = (
                resolve_token_ref(bg_raw, &parsed),
                resolve_token_ref(fg_raw, &parsed),
            ) {
                if let (Some(bg_rgb), Some(fg_rgb)) = (parse_hex_color(&bg_val), parse_hex_color(&fg_val)) {
                    let ratio = wcag_contrast_ratio(bg_rgb, fg_rgb);
                    if ratio < 4.5 {
                        findings.push(DesignMdFinding {
                            severity: "warning".to_string(),
                            path: format!("components.{}", comp_name),
                            message: format!(
                                "textColor ({}) on backgroundColor ({}) has contrast ratio {:.2}:1 — below WCAG 2.2 AA normal text threshold (4.5:1).",
                                fg_val, bg_val, ratio
                            ),
                        });
                    } else {
                        findings.push(DesignMdFinding {
                            severity: "info".to_string(),
                            path: format!("components.{}", comp_name),
                            message: format!(
                                "textColor ({}) on backgroundColor ({}) has contrast ratio {:.2}:1 — passes WCAG 2.2 AA.",
                                fg_val, bg_val, ratio
                            ),
                        });
                    }
                }
            }
        }
    }

    let errors = findings.iter().filter(|f| f.severity == "error").count();
    let warnings = findings.iter().filter(|f| f.severity == "warning").count();
    let infos = findings.iter().filter(|f| f.severity == "info").count();

    DesignMdLintReport {
        valid: errors == 0,
        findings,
        summary: DesignMdSummary {
            errors,
            warnings,
            infos,
        },
    }
}

fn diff_maps(before: &BTreeMap<String, String>, after: &BTreeMap<String, String>) -> TokenDelta {
    let mut delta = TokenDelta::default();
    for (k, v_after) in after {
        match before.get(k) {
            None => delta.added.push(k.clone()),
            Some(v_before) if v_before != v_after => delta.modified.push(k.clone()),
            _ => {}
        }
    }
    for k in before.keys() {
        if !after.contains_key(k) {
            delta.removed.push(k.clone());
        }
    }
    delta
}

/// Diffs two `DESIGN.md` files to detect token additions, removals, modifications, and lint regressions.
pub fn diff_design_md(before_content: &str, after_content: &str) -> DesignMdDiffReport {
    let b = parse_design_md_simple(before_content);
    let a = parse_design_md_simple(after_content);
    let b_lint = lint_design_md(before_content);
    let a_lint = lint_design_md(after_content);

    let mut tokens = BTreeMap::new();
    tokens.insert("colors".to_string(), diff_maps(&b.colors, &a.colors));
    tokens.insert("typography".to_string(), diff_maps(&b.typography, &a.typography));
    tokens.insert("rounded".to_string(), diff_maps(&b.rounded, &a.rounded));
    tokens.insert("spacing".to_string(), diff_maps(&b.spacing, &a.spacing));

    let regression = a_lint.summary.errors > b_lint.summary.errors
        || a_lint.summary.warnings > b_lint.summary.warnings
        || !tokens["colors"].removed.is_empty();

    DesignMdDiffReport {
        tokens,
        before_summary: b_lint.summary,
        after_summary: a_lint.summary,
        regression,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_md_has_the_nine_standard_sections_in_order_then_layoza_extras() {
        let dir = std::env::temp_dir().join(format!("layoza_dmd9_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let (dna, _) = crate::inspector::inspect_project(&dir, None, false).unwrap();
        let md = generate_design_md(&dna);
        let mut last = 0;
        for (name, _) in CANONICAL_SECTIONS {
            let at = md.find(&format!(". {}\n", name)).unwrap_or_else(|| panic!("missing {name}"));
            assert!(at > last, "{name} out of order");
            last = at;
        }
        for extra in ["## Motion", "## Variants", "## Brand", "## Decisions"] {
            assert!(md.find(extra).unwrap() > last, "{extra} must follow the standard sections");
        }
        assert!(md.contains("**dark**") && md.contains("**compact**") && md.contains("--color-"));
        assert!(lint_design_md(&md).valid);
        // Files in the earlier eight-section order stay valid.
        let legacy = "---\ncolors:\n  ink: \"#111111\"\n---\n## Overview\n## Colors\n## Typography\n## Layout\n## Elevation & Depth\n## Shapes\n## Components\n## Do's and Don'ts\n";
        assert!(lint_design_md(legacy).valid, "{:?}", lint_design_md(legacy).findings);
        let skill = generate_project_skill_md(&dna);
        assert!(skill.starts_with("---\nname: \"") && skill.contains("DESIGN.md"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
