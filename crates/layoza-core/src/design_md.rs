// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Compatible implementation of the Google DESIGN.md specification (Apache-2.0 spec compatibility):
// - Generates normative YAML frontmatter + canonical 8-section Markdown prose
// - Lints DESIGN.md files for section ordering, duplicate sections, broken token references,
//   and WCAG 2.2 AA contrast ratios
// - Diffs two DESIGN.md files for token and lint regressions

use crate::dna::SiteDna;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const CANONICAL_SECTIONS: &[(&str, &[&str])] = &[
    ("Overview", &["Overview", "Brand & Style"]),
    ("Colors", &["Colors"]),
    ("Typography", &["Typography"]),
    ("Layout", &["Layout", "Layout & Spacing"]),
    ("Elevation & Depth", &["Elevation & Depth", "Elevation"]),
    ("Shapes", &["Shapes"]),
    ("Components", &["Components"]),
    ("Do's and Don'ts", &["Do's and Don'ts", "Dos and Donts"]),
];

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

    // Canonical 8 sections in strict order
    out.push_str("## Overview\n\n");
    out.push_str(&format!(
        "`{}` uses the `{}` framework with Design Variance `{}/10`, Motion Intensity `{}/10`, and Visual Density `{}/10`. Preservation mode is `{}`.\n\n",
        dna.project_name,
        dna.framework,
        dna.dials.design_variance,
        dna.dials.motion_intensity,
        dna.dials.visual_density,
        dna.preservation_mode
    ));

    out.push_str("## Colors\n\n");
    for c in &dna.colors {
        out.push_str(&format!(
            "- **{} (`{}`)**: {} _(Source: `{}`, Tier: `{:?}`)_\n",
            c.name, c.value, c.role, c.source, c.tier
        ));
    }
    out.push('\n');

    out.push_str("## Typography\n\n");
    for t in &dna.typography {
        out.push_str(&format!(
            "- **{}**: `{}` at `{}` (weight `{}`, line-height `{}`, tracking `{}`)\n",
            t.role, t.font_family, t.font_size, t.font_weight, t.line_height, t.letter_spacing
        ));
    }
    out.push('\n');

    out.push_str("## Layout\n\n");
    out.push_str(&format!(
        "- **Max Content Widths**: {}\n",
        if dna.content_max_widths.is_empty() {
            "`1280px`".to_string()
        } else {
            dna.content_max_widths.join(", ")
        }
    ));
    out.push_str(&format!(
        "- **Grid Systems**: {}\n\n",
        if dna.grid_systems.is_empty() {
            "CSS Grid + Flexbox".to_string()
        } else {
            dna.grid_systems.join(", ")
        }
    ));

    out.push_str("## Elevation & Depth\n\n");
    for (k, v) in &dna.shadows {
        out.push_str(&format!("- **{}**: `{}`\n", k, v));
    }
    out.push('\n');

    out.push_str("## Shapes\n\n");
    for (k, v) in &dna.radii_scale {
        out.push_str(&format!("- **{}**: `{}`\n", k, v));
    }
    out.push('\n');

    out.push_str("## Components\n\n");
    out.push_str("- **button-primary**: Primary interactive call-to-action using brand accent and accessible foreground contrast.\n\n");

    out.push_str("## Do's and Don'ts\n\n");
    out.push_str("- **Do**: Preserve existing design tokens, spacing rhythm, and component hierarchy when adding sections or pages.\n");
    out.push_str("- **Do**: Honor `prefers-reduced-motion: reduce` and maintain WCAG 2.2 AA contrast ratios.\n");
    out.push_str("- **Don't**: Introduce conflicting CSS frameworks, unrequested SaaS purple gradients, or nested card clutter.\n");

    out
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
    for h in &parsed.headings {
        if seen_headings.iter().any(|s: &String| s.eq_ignore_ascii_case(h)) {
            findings.push(DesignMdFinding {
                severity: "error".to_string(),
                path: format!("sections.{}", h),
                message: format!("Duplicate section heading `## {}` is not permitted by DESIGN.md spec.", h),
            });
        }
        seen_headings.push(h.clone());

        if let Some((canon_idx, (canon_name, _))) = CANONICAL_SECTIONS
            .iter()
            .enumerate()
            .find(|(_, (_, aliases))| aliases.iter().any(|a| a.eq_ignore_ascii_case(h)))
        {
            if let Some(prev) = last_canonical_idx {
                if canon_idx < prev {
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
