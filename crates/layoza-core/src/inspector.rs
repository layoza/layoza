// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Existing Site DNA Engine: inspects local project directories, localhost/public URLs (via browser-qa),
// design token files, and screenshots to produce DESIGN.md, BRAND.md, MOTION.md, SITE-DNA.json,
// COMPONENT-INVENTORY.json, and DESIGN-DECISIONS.md.

use crate::design_md::generate_design_md;
use crate::dna::{
    ColorToken, ComponentEntry, ComponentInventory, DecisionEntry, DesignDials, DiscrepancyRecord,
    ExtractionConfidence, KnowledgeTier, MotionProfile, SiteDna, TypographyToken,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionResult {
    pub dna: SiteDna,
    pub inventory: ComponentInventory,
    pub generated_files: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BrowserDomSnapshot {
    pub url: String,
    pub computed_body_bg: Option<String>,
    pub computed_body_color: Option<String>,
    pub computed_heading_font: Option<String>,
    pub computed_body_font: Option<String>,
    pub detected_overflow_elements: usize,
    pub console_errors: Vec<String>,
    pub viewports_tested: Vec<String>,
}

/// Computes a deterministic FNV-1a 64-bit hex fingerprint over tracked source files.
pub fn compute_project_fingerprint(root: &Path) -> String {
    let mut files = Vec::new();
    collect_project_files(root, root, &mut files, 0);
    files.sort();

    let mut hash: u64 = 0xcbf29ce484222325;
    for rel in files {
        for b in rel.as_bytes() {
            hash ^= *b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        let full = root.join(&rel);
        if let Ok(bytes) = fs::read(&full) {
            for b in bytes { hash = (hash ^ b as u64).wrapping_mul(0x100000001b3); }
        }
    }
    format!("{:016x}", hash)
}

fn collect_project_files(root: &Path, current: &Path, out: &mut Vec<String>, depth: usize) {
    if depth > 6 {
        return;
    }
    let Ok(entries) = fs::read_dir(current) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if matches!(
            name,
            ".git"
                | ".internal"
                | "qa"
                | "screenshots"
                | "vendor"
                | "node_modules"
                | "target"
                | ".next"
                | "dist"
                | "build"
                | ".layoza"
                | ".agents"
                | ".claude"
                | ".cursor"
                | ".codex"
                | ".opencode"
        ) {
            continue;
        }
        if path.is_dir() {
            collect_project_files(root, &path, out, depth + 1);
        } else if let Ok(rel) = path.strip_prefix(root) {
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if ! matches!(
                name,
                "DESIGN.md"
                    | "BRAND.md"
                    | "MOTION.md"
                    | "SITE-DNA.json"
                    | "COMPONENT-INVENTORY.json"
                    | "DESIGN-DECISIONS.md"
            ) {
                out.push(rel_str);
            }
        }
    }
}

/// Inspects a project directory (and optional browser DOM snapshot or screenshot reference)
/// and extracts the complete Site DNA and Component Inventory.
pub fn inspect_project(
    project_root: &Path,
    browser_snapshot: Option<&BrowserDomSnapshot>,
    screenshot_only: bool,
) -> Result<(SiteDna, ComponentInventory), String> {
    if !project_root.exists() {
        return Err(format!(
            "Project root `{}` does not exist.",
            project_root.display()
        ));
    }

    let project_name = project_root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project")
        .to_string();

    let mut files = Vec::new();
    collect_project_files(project_root, project_root, &mut files, 0);
    files.sort();

    let mut framework = "html-css-js".to_string();
    let mut motion_libs = BTreeSet::new();
    let mut routes = Vec::new();
    let mut colors = Vec::new();
    let mut seen_color_names = BTreeSet::new();
    let mut font_scan = FontScan::default();
    // Sketches, drafts, fixtures and tests are not the shipped site: their fonts count only when
    // nothing else declares any
    let mut draft_font_scan = FontScan::default();
    let mut keyframes = BTreeSet::new();
    let mut honors_reduced_motion = false;
    let mut dark_mode = "light-default".to_string();
    let mut a11y_patterns = BTreeSet::new();
    let mut components = Vec::new();
    let mut nav_elements = BTreeSet::new();
    let mut footer_elements = BTreeSet::new();
    let mut grid_systems = BTreeSet::new();
    let mut max_widths = BTreeSet::new();
    let mut observed_tokens = crate::source_tokens::SourceTokens::default();

    // 1. Inspect package.json if present
    let pkg_path = project_root.join("package.json");
    if let Ok(pkg_raw) = fs::read_to_string(&pkg_path) {
        scan_fonts("package.json", &pkg_raw, &mut font_scan);
        if pkg_raw.contains("\"next\"") {
            framework = "nextjs".to_string();
        } else if pkg_raw.contains("\"astro\"") {
            framework = "astro".to_string();
        } else if pkg_raw.contains("\"nuxt\"") {
            framework = "nuxt".to_string();
        } else if pkg_raw.contains("\"@sveltejs/kit\"") || pkg_raw.contains("\"svelte\"") {
            framework = "sveltekit".to_string();
        } else if pkg_raw.contains("\"vue\"") {
            framework = "vue".to_string();
        } else if pkg_raw.contains("\"react\"") {
            framework = "react".to_string();
        }

        for (dep, label) in [
            ("\"gsap\"", "GSAP"),
            ("\"motion\"", "Motion"),
            ("\"framer-motion\"", "Motion (Framer Motion)"),
            ("\"three\"", "Three.js"),
            ("\"@react-three/fiber\"", "React Three Fiber"),
            ("\"@react-three/drei\"", "Drei"),
            ("\"lenis\"", "Lenis"),
            ("\"@studio-freight/lenis\"", "Lenis"),
        ] {
            if pkg_raw.contains(dep) {
                motion_libs.insert(label.to_string());
            }
        }
    }

    // 2. Check WordPress Gutenberg / block theme (`theme.json` or `style.css` with `Theme Name:`)
    let wp_theme_json = project_root.join("theme.json");
    if wp_theme_json.exists() {
        framework = "wordpress-gutenberg".to_string();
        if let Ok(raw) = fs::read_to_string(&wp_theme_json) {
            extract_wp_theme_json_tokens(&raw, &mut colors, &mut seen_color_names);
            scan_theme_json_fonts(&raw, &mut font_scan);
        }
    }

    // Check Shopify Liquid / Elementor / Bricks markers
    if files.iter().any(|f| f.ends_with(".liquid")) {
        framework = "shopify-liquid".to_string();
    } else if files.iter().any(|f| f.contains("elementor")) {
        framework = "elementor".to_string();
    } else if files.iter().any(|f| f.contains("bricks")) {
        framework = "bricks".to_string();
    }

    // 3. Scan project files for routes, CSS variables, components, motion, and accessibility
    for rel in &files {
        let full: PathBuf = project_root.join(rel);
        let lower = rel.to_ascii_lowercase();

        // Route discovery
        if lower.ends_with("page.tsx")
            || lower.ends_with("page.jsx")
            || lower.ends_with("page.js")
            || lower.starts_with("pages/")
            || lower.starts_with("src/pages/")
            || lower.starts_with("templates/")
            || (lower.ends_with(".html") && !lower.contains("node_modules"))
        {
            routes.push(rel.clone());
        }

        // Component discovery
        if lower.contains("components/")
            || lower.contains("patterns/")
            || lower.contains("sections/")
            || lower.ends_with(".tsx")
            || lower.ends_with(".astro")
            || lower.ends_with(".vue")
            || lower.ends_with(".svelte")
        {
            let comp_name = Path::new(rel)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Component")
                .to_string();
            let category = if lower.contains("hero") {
                "hero"
            } else if lower.contains("nav") || lower.contains("header") {
                "navigation"
            } else if lower.contains("footer") {
                "footer"
            } else if lower.contains("pricing") {
                "pricing"
            } else {
                "ui-component"
            };
            components.push(ComponentEntry {
                name: comp_name,
                category: category.to_string(),
                file_path: rel.clone(),
                variants: fs::read_to_string(&full).map(|s| crate::source_tokens::component_details(&s).0).unwrap_or_default(),
                props_or_classes: fs::read_to_string(&full).map(|s| crate::source_tokens::component_details(&s).1).unwrap_or_default(),
                accessible_patterns: fs::read_to_string(&full).map(|s| crate::source_tokens::component_details(&s).2).unwrap_or_default(),
            });
        }

        // Read text content of relevant source files
        if lower.ends_with(".css")
            || lower.ends_with(".scss")
            || lower.ends_with(".html")
            || lower.ends_with(".tsx")
            || lower.ends_with(".jsx")
            || lower.ends_with(".ts")
            || lower.ends_with(".js")
            || lower.ends_with(".astro")
            || lower.ends_with(".vue")
            || lower.ends_with(".svelte")
            || lower.ends_with(".php")
            || lower.ends_with(".less")
            || lower.ends_with(".sass")
            || lower.ends_with(".mjs")
            || lower.ends_with(".cjs")
            || lower.ends_with(".liquid")
        {
            if let Ok(content) = fs::read_to_string(&full) {
                if !is_draft_path(rel) { observed_tokens.scan(rel, &content); }
                let target = if is_draft_path(rel) { &mut draft_font_scan } else { &mut font_scan };
                scan_fonts(rel, &content, target);
                scan_source_content(
                    rel,
                    &content,
                    &mut colors,
                    &mut seen_color_names,
                    &mut keyframes,
                    &mut honors_reduced_motion,
                    &mut dark_mode,
                    &mut a11y_patterns,
                    &mut nav_elements,
                    &mut footer_elements,
                    &mut grid_systems,
                    &mut max_widths,
                    &mut motion_libs,
                );
            }
        }
    }

    // Provide sensible observed/inferred fallbacks when inspecting an empty or minimal project
    let source_inspected = !files.is_empty() && !screenshot_only;
    if colors.is_empty() {
        // Fallback colours are never observed, whether or not sources were scanned
        let tier = KnowledgeTier::InferredRule;
        colors.push(ColorToken {
            name: "primary".to_string(),
            value: "#141619".to_string(),
            role: "Primary brand ink and headline foreground".to_string(),
            tier: tier.clone(),
            source: "inferred-baseline".to_string(),
        });
        colors.push(ColorToken {
            name: "background".to_string(),
            value: "#F8F7F4".to_string(),
            role: "Primary canvas surface".to_string(),
            tier: tier.clone(),
            source: "inferred-baseline".to_string(),
        });
        colors.push(ColorToken {
            name: "accent".to_string(),
            value: "#C84B31".to_string(),
            role: "Primary interactive CTA and focus accent".to_string(),
            tier: tier.clone(),
            source: "inferred-baseline".to_string(),
        });
        colors.push(ColorToken {
            name: "muted".to_string(),
            value: "#5C6470".to_string(),
            role: "Secondary body copy and metadata".to_string(),
            tier,
            source: "inferred-baseline".to_string(),
        });
    }

    let resolved_fonts = if screenshot_only {
        None
    } else {
        resolve_typography(&font_scan).or_else(|| {
            resolve_typography(&draft_font_scan).map(|(t, s)| {
                (t, format!("{} (found only in sketch/draft/test files, not in the shipped source)", s))
            })
        })
    };
    let typography_observed = resolved_fonts.is_some();
    let (mut typography, typography_summary) = resolved_fonts.unwrap_or_default();
    if typography.is_empty() {
        // Nothing in the sources declares a font: this baseline is a guess, and says so.
        typography.push(TypographyToken {
            role: "display".to_string(),
            font_family: "Instrument Serif, Georgia, serif".to_string(),
            font_size: "clamp(2.5rem, 5vw, 4.25rem)".to_string(),
            font_weight: "600".to_string(),
            line_height: "1.08".to_string(),
            letter_spacing: "-0.025em".to_string(),
            tier: KnowledgeTier::InferredRule,
            source: "inferred-baseline: no font declarations found in the project sources".to_string(),
        });
        typography.push(TypographyToken {
            role: "body".to_string(),
            font_family: "Plus Jakarta Sans, system-ui, sans-serif".to_string(),
            font_size: "1rem".to_string(),
            font_weight: "400".to_string(),
            line_height: "1.6".to_string(),
            letter_spacing: "0em".to_string(),
            tier: KnowledgeTier::InferredRule,
            source: "inferred-baseline: no font declarations found in the project sources".to_string(),
        });
    }

    let mut discrepancies = Vec::new();
    if let Some(snap) = browser_snapshot {
        if let Some(rendered_bg) = &snap.computed_body_bg {
            if let Some(src_bg) = colors
                .iter()
                .find(|c| c.name.contains("bg") || c.name.contains("background"))
            {
                if !src_bg.value.eq_ignore_ascii_case(rendered_bg) {
                    discrepancies.push(DiscrepancyRecord {
                        property: "body.backgroundColor".to_string(),
                        source_value: src_bg.value.clone(),
                        rendered_value: rendered_bg.clone(),
                        explanation: "Source token and browser computed style differ (check dark-mode media query or runtime theme class override).".to_string(),
                    });
                }
            }
        }
    }

    let mut unknowns = Vec::new();
    if screenshot_only {
        unknowns.push("Screenshot-only input: exact CSS custom property names (`--*`), hover/focus state transitions, and JavaScript scroll timelines cannot be deterministically verified from pixels alone.".to_string());
    }
    if browser_snapshot.is_none() {
        unknowns.push("Live browser DOM snapshot not attached during static scan: run `layoza audit` or `scripts/browser-qa.mjs` to merge computed styles across desktop, tablet, and mobile viewports.".to_string());
    }

    if !typography_observed {
        unknowns.push("Typography: no font declarations found (no @font-face, font-family, font stack custom property, Google Fonts link, @fontsource package, next/font import, Tailwind fontFamily or theme.json fontFamilies). The display/body fonts listed are an inferred baseline, not the site's fonts.".to_string());
    }

    let overall_score = match (source_inspected, browser_snapshot.is_some(), screenshot_only) {
        (_, _, true) => 0.45,
        (true, true, false) => 0.96,
        (true, false, false) => 0.85,
        (false, true, false) => 0.78,
        (false, false, false) => 0.55,
    };

    let motion_intensity = if motion_libs.iter().any(|l| l.contains("Three") || l.contains("GSAP")) {
        7
    } else if !motion_libs.is_empty() || !keyframes.is_empty() {
        5
    } else {
        3
    };

    let spacing_scale = observed_tokens.spacing;
    let radii_scale = observed_tokens.radii;
    let shadows = observed_tokens.shadows;
    let breakpoints = observed_tokens.breakpoints;
    for (label, missing) in [("spacing", spacing_scale.is_empty()), ("radii", radii_scale.is_empty()), ("shadows", shadows.is_empty()), ("breakpoints", breakpoints.is_empty())] {
        if missing { unknowns.push(format!("No {label} declarations extracted; values remain unknown. Do not replace the incumbent design with a default scale.")); }
    }

    let mut decisions = Vec::new();
    for (token, source) in &observed_tokens.evidence {
        decisions.push(DecisionEntry { id: format!("TOKEN-{}", decisions.len() + 1), category: "source-token".into(), tier: KnowledgeTier::ObservedFact, statement: format!("Source declaration `{token}`; verify the computed cascade on a sibling render."), evidence_source: source.clone(), confidence: 0.8 });
    }
    decisions.push(DecisionEntry {
        id: "DEC-001".to_string(),
        category: "framework".to_string(),
        tier: if source_inspected {
            KnowledgeTier::ObservedFact
        } else {
            KnowledgeTier::InferredRule
        },
        statement: format!("Project uses the `{}` stack.", framework),
        evidence_source: "project-scanner".to_string(),
        confidence: overall_score,
    });
    decisions.push(DecisionEntry {
        id: "DEC-002".to_string(),
        category: "preservation".to_string(),
        tier: KnowledgeTier::InferredRule,
        statement: "Existing websites default to Preservation Mode: new sections or pages must reuse existing typography, color tokens, spacing scale, and motion intensity.".to_string(),
        evidence_source: "layoza-mode-b-policy".to_string(),
        confidence: 1.0,
    });
    decisions.push(DecisionEntry {
        id: "DEC-003".to_string(),
        category: "typography".to_string(),
        tier: if typography_observed {
            KnowledgeTier::ObservedFact
        } else {
            KnowledgeTier::InferredRule
        },
        statement: if typography_observed {
            format!("Site fonts observed in source: {}. Reuse them; do not introduce new families.", typography_summary)
        } else {
            "No font declarations found; display/body fonts are an inferred baseline to confirm with the user.".to_string()
        },
        evidence_source: if typography_observed {
            "font-scanner (see typography[].source)".to_string()
        } else {
            "inferred-baseline".to_string()
        },
        confidence: if typography_observed { overall_score } else { 0.3 },
    });

    let fingerprint = compute_project_fingerprint(project_root);

    let dna = SiteDna {
        schema_version: "1.0.0".to_string(),
        project_name: project_name.clone(),
        preservation_mode: source_inspected,
        framework: framework.clone(),
        routes,
        colors,
        typography,
        spacing_scale,
        radii_scale,
        shadows,
        breakpoints,
        content_max_widths: max_widths.into_iter().collect(),
        grid_systems: grid_systems.into_iter().collect(),
        dark_mode_strategy: dark_mode,
        accessibility_patterns: a11y_patterns.into_iter().collect(),
        motion: MotionProfile {
            libraries_detected: motion_libs.into_iter().collect(),
            keyframes_detected: keyframes.into_iter().collect(),
            transition_durations: observed_tokens.durations.into_iter().collect(),
            easing_curves: observed_tokens.easings.into_iter().collect(),
            scroll_behavior: "native".to_string(),
            honors_reduced_motion,
            recommended_motion_intensity: motion_intensity,
            tier: if source_inspected {
                KnowledgeTier::ObservedFact
            } else {
                KnowledgeTier::InferredRule
            },
        },
        dials: DesignDials::clamped(4, motion_intensity, 5),
        discrepancies,
        confidence: ExtractionConfidence {
            overall_score,
            source_inspected,
            browser_inspected: browser_snapshot.is_some(),
            screenshot_only,
            unknowns,
        },
        decisions,
        baseline_fingerprint: fingerprint,
    };

    let inventory = ComponentInventory {
        project_name,
        framework,
        total_components: components.len(),
        components,
        layout_primitives: vec![
            "container".to_string(),
            "grid".to_string(),
            "section-shell".to_string(),
        ],
        navigation_elements: nav_elements.into_iter().collect(),
        footer_elements: footer_elements.into_iter().collect(),
    };

    Ok((dna, inventory))
}

#[allow(clippy::too_many_arguments)]
fn scan_source_content(
    rel_path: &str,
    content: &str,
    colors: &mut Vec<ColorToken>,
    seen_color_names: &mut BTreeSet<String>,
    keyframes: &mut BTreeSet<String>,
    honors_reduced_motion: &mut bool,
    dark_mode: &mut String,
    a11y_patterns: &mut BTreeSet<String>,
    nav_elements: &mut BTreeSet<String>,
    footer_elements: &mut BTreeSet<String>,
    grid_systems: &mut BTreeSet<String>,
    max_widths: &mut BTreeSet<String>,
    motion_libs: &mut BTreeSet<String>,
) {
    if content.contains("prefers-reduced-motion") || content.contains("useReducedMotion") || content.contains("gsap.matchMedia") {
        *honors_reduced_motion = true;
        a11y_patterns.insert("prefers-reduced-motion".to_string());
    }
    if content.contains("prefers-color-scheme: dark") || content.contains("dark:") || content.contains("data-theme=\"dark\"") {
        *dark_mode = "supported (media/class)".to_string();
    }
    if content.contains("aria-label") || content.contains("aria-expanded") || content.contains("role=") {
        a11y_patterns.insert("semantic-aria-attributes".to_string());
    }
    if content.contains(":focus-visible") || content.contains("focus-visible:") {
        a11y_patterns.insert("focus-visible-ring".to_string());
    }
    if content.contains("<nav") {
        nav_elements.insert(format!("<nav> in {}", rel_path));
    }
    if content.contains("<footer") {
        footer_elements.insert(format!("<footer> in {}", rel_path));
    }
    if content.contains("display: grid") || content.contains("grid-template-columns") {
        grid_systems.insert("CSS Grid".to_string());
    }
    if content.contains("display: flex") {
        grid_systems.insert("Flexbox".to_string());
    }
    if content.contains("gsap.") || content.contains("ScrollTrigger") {
        motion_libs.insert("GSAP".to_string());
    }
    if content.contains("THREE.") || content.contains("WebGLRenderer") {
        motion_libs.insert("Three.js".to_string());
    }

    for line in content.lines() {
        let trimmed = line.trim();
        // Extract CSS custom properties (`--color-*: #...;` or `--*: #...;`), including several per
        // line as in inline or minified `:root{--ink:#111;--paper:#fff}` and `style="--x:..."`.
        for segment in trimmed.split([';', '{', '}']) {
            // `--name` at the segment start, or right after a quote/space (`style="--x:#fff"`)
            let seg = segment.trim();
            let Some(pos) = seg.match_indices("--").map(|(i, _)| i).find(|&i| {
                i == 0 || matches!(seg.as_bytes()[i - 1], b'"' | b'\'' | b' ' | b'\t')
            }) else {
                continue;
            };
            let rest = &seg[pos + 2..];
            if let Some((var_name, raw_val)) = rest.split_once(':') {
                if var_name.trim().is_empty() || var_name.contains(char::is_whitespace) {
                    continue;
                }
                let raw_val = if pos > 0 {
                    // inline style: the value ends at the closing attribute quote
                    raw_val.split(['"', '\'']).next().unwrap_or(raw_val)
                } else {
                    raw_val
                };
                let name = var_name.trim().to_string();
                let val = raw_val.trim().trim_end_matches(';').trim().to_string();
                if (val.starts_with('#') || val.starts_with("rgb") || val.starts_with("oklch") || val.starts_with("hsl"))
                    && !seen_color_names.contains(&name)
                {
                    seen_color_names.insert(name.clone());
                    colors.push(ColorToken {
                        name: name.clone(),
                        value: val,
                        role: format!("CSS custom property `--{}`", name),
                        tier: KnowledgeTier::ObservedFact,
                        source: rel_path.to_string(),
                    });
                }
            }
        }

        if let Some(kf) = trimmed.strip_prefix("@keyframes ") {
            let name = kf
                .split(|c: char| c.is_whitespace() || c == '{')
                .next()
                .unwrap_or("")
                .trim();
            if !name.is_empty() {
                keyframes.insert(name.to_string());
            }
        }

        if let Some(mw_idx) = trimmed.find("max-width:") {
            let after = &trimmed[mw_idx + "max-width:".len()..];
            let val = after
                .split([';', '}', '"'])
                .next()
                .unwrap_or("")
                .trim();
            if !val.is_empty() && val.len() <= 16 {
                max_widths.insert(val.to_string());
            }
        }
    }
}

fn extract_wp_theme_json_tokens(
    raw: &str,
    colors: &mut Vec<ColorToken>,
    seen_color_names: &mut BTreeSet<String>,
) {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(raw) else {
        return;
    };
    if let Some(palette) = val
        .pointer("/settings/color/palette")
        .and_then(|v| v.as_array())
    {
        for item in palette {
            let slug = item.get("slug").and_then(|s| s.as_str()).unwrap_or("wp-color");
            let color = item.get("color").and_then(|s| s.as_str()).unwrap_or("#000000");
            let name = item.get("name").and_then(|s| s.as_str()).unwrap_or(slug);
            if !seen_color_names.contains(slug) {
                seen_color_names.insert(slug.to_string());
                colors.push(ColorToken {
                    name: slug.to_string(),
                    value: color.to_string(),
                    role: format!("WordPress theme.json palette token ({})", name),
                    tier: KnowledgeTier::ObservedFact,
                    source: "theme.json".to_string(),
                });
            }
        }
    }
}

/// Writes all 6 Site DNA artifacts (`DESIGN.md`, `BRAND.md`, `MOTION.md`,
/// `SITE-DNA.json`, `COMPONENT-INVENTORY.json`, `DESIGN-DECISIONS.md`) into `output_dir`.
pub fn write_dna_artifacts(
    output_dir: &Path,
    dna: &SiteDna,
    inventory: &ComponentInventory,
) -> Result<Vec<String>, String> {
    fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output dir: {}", e))?;

    let files: [(&str, String); 6] = [
        ("DESIGN.md", generate_design_md(dna)),
        ("BRAND.md", dna.to_brand_md()),
        ("MOTION.md", dna.to_motion_md()),
        (
            "SITE-DNA.json",
            serde_json::to_string_pretty(dna).map_err(|e| e.to_string())?,
        ),
        (
            "COMPONENT-INVENTORY.json",
            serde_json::to_string_pretty(inventory).map_err(|e| e.to_string())?,
        ),
        ("DESIGN-DECISIONS.md", dna.to_design_decisions_md()),
    ];

    let mut written = Vec::new();
    for (filename, content) in files {
        let p = output_dir.join(filename);
        fs::write(&p, content).map_err(|e| format!("Failed writing `{}`: {}", p.display(), e))?;
        written.push(p.to_string_lossy().replace('\\', "/"));
    }

    Ok(written)
}

/// Checks whether a stored `SITE-DNA.json` baseline in `project_root` is still fresh.
pub fn is_dna_baseline_fresh(project_root: &Path) -> Result<(bool, String, String), String> {
    let dna_path = project_root.join("SITE-DNA.json");
    if !dna_path.exists() {
        return Ok((false, "none".to_string(), compute_project_fingerprint(project_root)));
    }
    let raw = fs::read_to_string(&dna_path).map_err(|e| e.to_string())?;
    let stored: SiteDna = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    let current = compute_project_fingerprint(project_root);
    Ok((
        stored.baseline_fingerprint == current,
        stored.baseline_fingerprint,
        current,
    ))
}

// ---------------------------------------------------------------------------
// Real-font detection
// ---------------------------------------------------------------------------
//
// Collects font evidence from every place a project can declare type: `@font-face`, CSS
// `font-family` / `font:` declarations (stylesheets, `<style>`, inline `style=""`, JSX
// `fontFamily:`), CSS custom properties holding font stacks, Google/Bunny Fonts URLs, `@fontsource`
// packages (package.json and imports), `next/font` imports, Tailwind `fontFamily` and theme.json
// `fontFamilies`. Fonts are then mapped to display / body / mono roles by where they are used.

const GENERIC_FAMILIES: [&str; 11] = [
    "serif", "sans-serif", "monospace", "cursive", "fantasy", "system-ui", "ui-serif", "ui-sans-serif",
    "ui-monospace", "ui-rounded", "math",
];

#[derive(Debug, Default)]
struct FontUse {
    selector: String,
    value: String,
    size: Option<String>,
    weight: Option<String>,
    line_height: Option<String>,
    letter_spacing: Option<String>,
    source: String,
}

#[derive(Debug, Default)]
struct FontScan {
    /// custom property name (without `--`) -> (stack, file)
    vars: BTreeMap<String, (String, String)>,
    uses: Vec<FontUse>,
    /// (family, evidence) for fonts a project loads (`@font-face`, Google Fonts, `@fontsource`, next/font)
    loaded: Vec<(String, String)>,
    /// (role-hint key, stack, evidence) from Tailwind `fontFamily` / theme.json `fontFamilies`
    configured: Vec<(String, String, String)>,
}

fn is_quote(c: char) -> bool {
    c == '"' || c == '\'' || c == '`'
}

fn normalize_stack(raw: &str) -> String {
    let raw = raw.replace("!important", "");
    raw.split(',')
        .map(|f| f.trim().trim_matches(is_quote).trim())
        .filter(|f| !f.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

fn primary_family(stack: &str) -> String {
    stack.split(',').next().unwrap_or("").trim().to_string()
}

/// True when a custom-property value reads as a font stack rather than a size, colour or weight.
fn looks_like_font_stack(name: &str, val: &str) -> bool {
    let v = val.trim();
    if v.is_empty()
        || v.len() > 300
        || v.starts_with('#')
        || v.contains("rgb")
        || v.contains("hsl")
        || v.contains("oklch")
    {
        return false;
    }
    let lname = name.to_ascii_lowercase();
    if ["size", "weight", "leading", "tracking", "line", "spacing", "feature", "variation", "optical"]
        .iter()
        .any(|k| lname.contains(k))
    {
        return false;
    }
    let names_font = lname.contains("font") || lname.contains("family");
    if v.starts_with("var(") {
        return names_font;
    }
    let lower = v.to_ascii_lowercase();
    let has_generic = lower
        .split(',')
        .any(|f| GENERIC_FAMILIES.contains(&f.trim().trim_matches(is_quote)));
    let quoted = v.starts_with(is_quote);
    let alpha_stack = v.chars().all(|c| c.is_alphanumeric() || " ,-_".contains(c) || is_quote(c))
        && v.chars().next().is_some_and(|c| c.is_alphabetic() || is_quote(c));
    alpha_stack && (has_generic || quoted || names_font)
}

/// (family-or-var, size, weight, line-height) read from a `font:` shorthand.
type FontShorthand = (String, Option<String>, Option<String>, Option<String>);

/// Splits a `font:` shorthand into its family-or-var, size, weight and line-height.
fn parse_font_shorthand(v: &str) -> Option<FontShorthand> {
    let v = v.replace("!important", "");
    let v = v.trim();
    if v.is_empty() || matches!(v, "inherit" | "initial" | "unset" | "revert") {
        return None;
    }
    let tokens: Vec<&str> = v.split_whitespace().collect();
    let size_idx = tokens.iter().position(|t| {
        (t.starts_with(|c: char| c.is_ascii_digit() || c == '.') && !t.chars().all(|c| c.is_ascii_digit()))
            || t.starts_with("clamp(")
            || t.starts_with("calc(")
    })?;
    let mut weight = None;
    for t in &tokens[..size_idx] {
        if t.chars().all(|c| c.is_ascii_digit()) || matches!(*t, "bold" | "normal" | "lighter" | "bolder") {
            weight = Some(t.to_string());
        }
    }
    let size_tok = tokens[size_idx];
    let (size, mut lh) = match size_tok.split_once('/') {
        Some((s, l)) => (s.to_string(), Some(l.to_string()).filter(|l| !l.is_empty())),
        None => (size_tok.to_string(), None),
    };
    let mut rest_start = size_idx + 1;
    if lh.is_none() && tokens.get(rest_start) == Some(&"/") {
        lh = tokens.get(rest_start + 1).map(|s| s.to_string());
        rest_start += 2;
    }
    let family = tokens.get(rest_start..).map(|r| r.join(" ")).unwrap_or_default();
    if family.trim().is_empty() {
        return None;
    }
    Some((family, Some(size), weight, lh))
}

fn add_loaded(scan: &mut FontScan, family: &str, evidence: String) {
    let family = family.trim().trim_matches(is_quote).trim();
    if family.is_empty() || family.len() > 80 {
        return;
    }
    if !scan
        .loaded
        .iter()
        .any(|(f, e)| f.eq_ignore_ascii_case(family) && *e == evidence)
    {
        scan.loaded.push((family.to_string(), evidence));
    }
}

fn title_case_slug(slug: &str) -> String {
    slug.split(['-', '_'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            if matches!(w, "ibm" | "pt" | "eb" | "dm") {
                return w.to_ascii_uppercase();
            }
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Text between the first `open` at/after `start` and its matching `close`.
fn balanced_block(s: &str, start: usize, open: char, close: char) -> Option<&str> {
    let rel = s.get(start..)?.find(open)?;
    let begin = start + rel + open.len_utf8();
    let mut depth = 1usize;
    for (i, ch) in s[begin..].char_indices() {
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth -= 1;
            if depth == 0 {
                return Some(&s[begin..begin + i]);
            }
        }
    }
    None
}

fn quoted_strings(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if is_quote(c) {
            let mut buf = String::new();
            for d in chars.by_ref() {
                if d == c {
                    break;
                }
                buf.push(d);
            }
            out.push(buf);
        }
    }
    out
}

/// Paths that are not the shipped site (sketches, drafts, fixtures, tests, stories).
fn is_draft_path(rel: &str) -> bool {
    const DRAFT_DIRS: &[&str] = &[
        "sketch", "sketches", "draft", "drafts", "old-draft", "archive", "archived", "mockup", "mockups",
        "prototype", "prototypes", "fixture", "fixtures", "__fixtures__", "test", "tests", "__tests__", "e2e",
        "stories", "playground", "scratch", "backup", "backups",
    ];
    let lower = rel.to_ascii_lowercase();
    let mut parts: Vec<&str> = lower.split('/').collect();
    parts.pop();
    parts.iter().any(|p| DRAFT_DIRS.contains(p))
}

fn scan_fonts(rel: &str, content: &str, scan: &mut FontScan) {
    let lower_rel = rel.to_ascii_lowercase();
    let file_name = lower_rel.rsplit('/').next().unwrap_or(&lower_rel).to_string();

    // @fontsource / @fontsource-variable (package.json deps and CSS/JS imports)
    for prefix in ["@fontsource-variable/", "@fontsource/"] {
        let mut from = 0;
        while let Some(i) = content[from..].find(prefix) {
            let start = from + i;
            from = start + prefix.len();
            let slug: String = content[from..]
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
                .collect();
            if slug.is_empty() {
                continue;
            }
            add_loaded(scan, &title_case_slug(&slug), format!("{} ({}{})", rel, prefix, slug));
        }
    }
    if file_name == "package.json" {
        return;
    }

    // Google Fonts / Bunny Fonts stylesheet URLs (`family=A:wght@400&family=B+C`, v1 `A|B`)
    for host in ["fonts.googleapis.com/css", "fonts.bunny.net/css"] {
        let mut from = 0;
        while let Some(i) = content[from..].find(host) {
            let start = from + i;
            from = start + host.len();
            let url: String = content[start..]
                .chars()
                .take_while(|c| !matches!(c, '"' | '\'' | ')' | ' ' | '\n' | '>' | '`'))
                .collect();
            let query = url.split_once('?').map(|(_, q)| q).unwrap_or("");
            for param in query.replace("&amp;", "&").split('&') {
                if let Some(val) = param.strip_prefix("family=") {
                    for fam in val.split('|') {
                        let name = fam.split(':').next().unwrap_or("").replace('+', " ").replace("%20", " ");
                        add_loaded(scan, &name, format!("{} (Google Fonts link)", rel));
                    }
                }
            }
        }
    }

    // next/font/google: `import { Inter, Roboto_Mono } from 'next/font/google'` and the
    // `Inter({ variable: '--font-sans' })` call binding the family to a CSS variable.
    if let Some(idx) = content.find("next/font/google") {
        if let Some(imp) = content[..idx].rfind("import") {
            if let Some(names) = balanced_block(&content[..idx], imp, '{', '}') {
                for raw in names.split(',') {
                    let ident = raw.split(" as ").next().unwrap_or("").trim();
                    if ident.is_empty() || !ident.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                        continue;
                    }
                    let family = ident.replace('_', " ");
                    add_loaded(scan, &family, format!("{} (next/font/google)", rel));
                    let call = format!("{}(", ident);
                    let Some(ci) = content[idx..].find(&call) else { continue };
                    let Some(args) = balanced_block(content, idx + ci, '(', ')') else { continue };
                    let Some(vi) = args.find("variable") else { continue };
                    if let Some(var) = quoted_strings(&args[vi..]).into_iter().next() {
                        if let Some(name) = var.strip_prefix("--") {
                            scan.vars.insert(name.to_string(), (family.clone(), format!("{} (next/font variable)", rel)));
                        }
                    }
                }
            }
        }
    }

    // Tailwind config `fontFamily: { display: ['Fraunces', 'serif'], sans: 'Inter' }`
    if file_name.starts_with("tailwind.config") {
        let Some(fi) = content.find("fontFamily") else { return };
        let Some(block) = balanced_block(content, fi, '{', '}') else { return };
        let mut rest = block;
        while let Some(colon) = rest.find(':') {
            let key = rest[..colon]
                .rsplit([',', '\n', '{'])
                .next()
                .unwrap_or("")
                .trim()
                .trim_matches(is_quote)
                .to_string();
            let after = &rest[colon + 1..];
            let (value, consumed) = if after.trim_start().starts_with('[') {
                let off = after.find('[').unwrap_or(0);
                let inner = balanced_block(after, off, '[', ']').unwrap_or("");
                (inner, off + inner.len() + 2)
            } else {
                let end = after.find([',', '\n', '}']).unwrap_or(after.len());
                (&after[..end], end)
            };
            let fams = quoted_strings(value);
            if !key.is_empty() && !fams.is_empty() {
                scan.configured
                    .push((key, normalize_stack(&fams.join(",")), format!("{} (tailwind fontFamily)", rel)));
            }
            rest = &after[consumed.min(after.len())..];
        }
        return;
    }

    // CSS custom properties holding font stacks (several per line / minified)
    for segment in content.split([';', '{', '}', '\n']) {
        let seg = segment.trim();
        let Some(pos) = seg
            .match_indices("--")
            .map(|(i, _)| i)
            .find(|&i| i == 0 || matches!(seg.as_bytes()[i - 1], b'"' | b'\'' | b' ' | b'\t'))
        else {
            continue;
        };
        let Some((name, val)) = seg[pos + 2..].split_once(':') else { continue };
        let name = name.trim();
        if name.is_empty() || name.contains(char::is_whitespace) {
            continue;
        }
        let val = if pos > 0 { val.split('"').next().unwrap_or(val) } else { val };
        if looks_like_font_stack(name, val) {
            let v = val.trim();
            let stack = if v.starts_with("var(") { v.to_string() } else { normalize_stack(v) };
            scan.vars.entry(name.to_string()).or_insert((stack, rel.to_string()));
        }
    }

    // CSS rules: `selector { font-family: ...; font-size: ... }` (stylesheets, <style>, CSS-in-JS)
    for chunk in content.split('}') {
        let Some((sel_raw, body)) = chunk.rsplit_once('{') else { continue };
        if !body.contains("font") {
            continue;
        }
        let mut sel = sel_raw;
        if let Some(si) = sel.rfind("<style") {
            sel = sel[si..].split_once('>').map(|(_, r)| r).unwrap_or("");
        }
        for sep in ["{", ";", "*/", "`"] {
            if let Some(i) = sel.rfind(sep) {
                sel = &sel[i + sep.len()..];
            }
        }
        let sel = sel.trim();
        let mut u = FontUse { selector: sel.to_string(), source: rel.to_string(), ..Default::default() };
        for decl in body.split(';') {
            let Some((prop, val)) = decl.split_once(':') else { continue };
            let prop = prop.trim().to_ascii_lowercase();
            let val = val.replace("!important", "").trim().to_string();
            match prop.as_str() {
                "font-family" => u.value = val,
                "font" => {
                    if let Some((fam, size, weight, lh)) = parse_font_shorthand(&val) {
                        u.value = fam;
                        u.size = u.size.or(size);
                        u.weight = u.weight.or(weight);
                        u.line_height = u.line_height.or(lh);
                    } else if val.starts_with("var(") {
                        u.value = val;
                    }
                }
                "font-size" => u.size = Some(val),
                "font-weight" => u.weight = Some(val),
                "line-height" => u.line_height = Some(val),
                "letter-spacing" => u.letter_spacing = Some(val),
                _ => {}
            }
        }
        if u.value.is_empty() || matches!(u.value.as_str(), "inherit" | "initial" | "unset" | "revert") {
            continue;
        }
        if sel.eq_ignore_ascii_case("@font-face") {
            add_loaded(scan, &primary_family(&normalize_stack(&u.value)), format!("{} (@font-face)", rel));
            continue;
        }
        if sel.is_empty() || sel.len() > 300 || sel.contains(['<', '=', '(']) && !sel.contains(':') {
            continue;
        }
        scan.uses.push(u);
    }

    // Inline `style="font-family:..."` attributes; the tag name stands in for the selector
    let mut from = 0;
    while let Some(i) = content[from..].find("style=\"") {
        let start = from + i + 7;
        from = start;
        let Some(end) = content[start..].find('"') else { break };
        let attr = &content[start..start + end];
        if let Some(fi) = attr.find("font-family:") {
            let val = attr[fi + 12..].split(';').next().unwrap_or("").trim().to_string();
            let tag: String = content[..start]
                .rfind('<')
                .map(|lt| content[lt + 1..start].chars().take_while(|c| c.is_ascii_alphanumeric()).collect())
                .unwrap_or_default();
            if !val.is_empty() && !val.contains("&quot;") {
                scan.uses.push(FontUse {
                    selector: tag,
                    value: val,
                    source: format!("{} (inline style)", rel),
                    ..Default::default()
                });
            }
        }
    }

    // JSX inline styles: `fontFamily: 'Inter, sans-serif'` / `fontFamily: "var(--font-x)"`
    let mut from = 0;
    while let Some(i) = content[from..].find("fontFamily:") {
        let start = from + i + 11;
        from = start;
        let tail = content[start..].trim_start();
        if tail.starts_with(is_quote) {
            if let Some(val) = quoted_strings(tail).into_iter().next() {
                scan.uses.push(FontUse {
                    selector: "inline-style".into(),
                    value: val,
                    source: format!("{} (fontFamily)", rel),
                    ..Default::default()
                });
            }
        }
    }
}

/// theme.json `settings.typography.fontFamilies` plus the `styles` that apply them.
fn scan_theme_json_fonts(raw: &str, scan: &mut FontScan) {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(raw) else { return };
    for item in val
        .pointer("/settings/typography/fontFamilies")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
    {
        let slug = item.get("slug").and_then(|s| s.as_str()).unwrap_or("");
        let Some(family) = item.get("fontFamily").and_then(|s| s.as_str()) else { continue };
        let stack = normalize_stack(family);
        scan.vars
            .insert(format!("wp--preset--font-family--{}", slug), (stack.clone(), "theme.json".into()));
        scan.configured
            .push((slug.to_string(), stack, "theme.json (settings.typography.fontFamilies)".into()));
    }
    // `var:preset|font-family|slug` is theme.json shorthand for the preset custom property
    let wp_var = |s: &str| match s.strip_prefix("var:preset|font-family|") {
        Some(slug) => format!("var(--wp--preset--font-family--{})", slug),
        None => s.to_string(),
    };
    for (ptr, selector) in [
        ("/styles/typography", "body"),
        ("/styles/elements/heading/typography", "h2"),
        ("/styles/elements/h1/typography", "h1"),
    ] {
        let Some(t) = val.pointer(ptr) else { continue };
        let Some(f) = t.get("fontFamily").and_then(|s| s.as_str()) else { continue };
        let g = |k: &str| t.get(k).and_then(|s| s.as_str()).map(str::to_string);
        scan.uses.push(FontUse {
            selector: selector.into(),
            value: wp_var(f),
            size: g("fontSize"),
            weight: g("fontWeight"),
            line_height: g("lineHeight"),
            letter_spacing: g("letterSpacing"),
            source: format!("theme.json ({})", ptr.trim_start_matches('/').replace('/', ".")),
        });
    }
}

/// Resolves a font-family value (literal stack or `var(--x, fallback)`) to (stack, var name, var file).
fn resolve_font_value(
    value: &str,
    vars: &BTreeMap<String, (String, String)>,
    depth: u8,
) -> Option<(String, Option<String>, Option<String>)> {
    let v = value.trim();
    if depth > 4 || v.is_empty() {
        return None;
    }
    if let Some(inner) = v.strip_prefix("var(") {
        let inner = inner.strip_suffix(')').unwrap_or(inner);
        let (name, fallback) = match inner.split_once(',') {
            Some((n, f)) => (n.trim(), Some(f.trim())),
            None => (inner.trim(), None),
        };
        let name = name.trim_start_matches("--");
        if let Some((stack, file)) = vars.get(name) {
            if let Some((s, inner_var, inner_file)) = resolve_font_value(stack, vars, depth + 1) {
                return Some((s, Some(inner_var.unwrap_or_else(|| name.to_string())), Some(inner_file.unwrap_or_else(|| file.clone()))));
            }
        }
        return fallback
            .and_then(|f| resolve_font_value(f, vars, depth + 1))
            .map(|(s, _, _)| (s, Some(name.to_string()), None));
    }
    if v.contains("var(") {
        return None;
    }
    let s = normalize_stack(v);
    if s.is_empty() || matches!(s.as_str(), "inherit" | "initial" | "unset" | "revert") {
        None
    } else {
        Some((s, None, None))
    }
}

/// (display, body, mono) weights for a selector, judged by its subject (last compound).
fn selector_role_scores(selector: &str) -> [i32; 3] {
    let mut s = [0i32; 3];
    for part in selector.to_ascii_lowercase().split(',') {
        let subject = part.split([' ', '>', '+', '~']).rfind(|p| !p.is_empty()).unwrap_or("");
        for word in subject
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
            .filter(|w| !w.is_empty())
        {
            match word {
                "h1" => s[0] = s[0].max(4),
                "h2" => s[0] = s[0].max(3),
                "h3" | "h4" | "h5" | "h6" => s[0] = s[0].max(2),
                "body" => s[1] = s[1].max(4),
                "html" | "root" => s[1] = s[1].max(3),
                "p" => s[1] = s[1].max(1),
                "code" | "pre" | "kbd" | "samp" => s[2] = s[2].max(3),
                w if w.contains("mono") => s[2] = s[2].max(2),
                w if ["heading", "title", "display", "hero", "headline"].iter().any(|k| w.contains(k)) => {
                    s[0] = s[0].max(2)
                }
                _ => {}
            }
        }
    }
    s
}

/// Role hint (0 display, 1 body, 2 mono) from a var / config key / family name.
fn name_role_hint(name: &str) -> Option<usize> {
    let n = name.to_ascii_lowercase();
    if n.contains("mono") || n.contains("code") {
        Some(2)
    } else if n.contains("sans") || ["body", "text", "base", "copy", "ui"].iter().any(|k| n.contains(k)) {
        Some(1)
    } else if ["display", "heading", "head", "title", "serif", "hero"].iter().any(|k| n.contains(k)) {
        Some(0)
    } else {
        None
    }
}

#[derive(Default)]
struct FontCandidate {
    stack: String,
    score: [i32; 3],
    /// best (score, use index) per role, for metrics
    best: [Option<(i32, usize)>; 3],
    evidence: Vec<String>,
    first_seen: usize,
}

impl FontCandidate {
    fn note(&mut self, ev: String) {
        if !self.evidence.contains(&ev) {
            self.evidence.push(ev);
        }
    }
}

fn candidate<'a>(cands: &'a mut Vec<FontCandidate>, stack: &str) -> &'a mut FontCandidate {
    let k = primary_family(stack);
    let pos = cands.iter().position(|c| primary_family(&c.stack).eq_ignore_ascii_case(&k));
    let idx = pos.unwrap_or_else(|| {
        let n = cands.len();
        cands.push(FontCandidate { stack: stack.to_string(), first_seen: n, ..Default::default() });
        n
    });
    &mut cands[idx]
}

/// Builds observed display/body/mono typography tokens from the scan, with a one-line summary.
/// Returns `None` when the project declares no fonts at all.
fn resolve_typography(scan: &FontScan) -> Option<(Vec<TypographyToken>, String)> {
    let mut cands: Vec<FontCandidate> = Vec::new();

    for (i, u) in scan.uses.iter().enumerate() {
        let Some((stack, var, var_file)) = resolve_font_value(&u.value, &scan.vars, 0) else { continue };
        let mut sc = selector_role_scores(&u.selector);
        if let Some(r) = var.as_deref().and_then(name_role_hint) {
            if sc.iter().any(|s| *s > 0) {
                sc[r] += 1;
            }
        }
        let sel = if u.selector.is_empty() { "inline" } else { u.selector.as_str() };
        let mut ev = match (&var, &var_file) {
            (Some(v), Some(f)) if f != &u.source => format!("{} (--{}); used by `{}` in {}", f, v, sel, u.source),
            (Some(v), _) => format!("{} (--{}, `{}`)", u.source, v, sel),
            (None, _) => format!("{} (`{}`)", u.source, sel),
        };
        if ev.chars().count() > 160 {
            ev = ev.chars().take(160).collect::<String>() + "...";
        }
        let c = candidate(&mut cands, &stack);
        if sc.iter().any(|s| *s > 0) || c.evidence.len() < 2 {
            c.note(ev);
        }
        for (r, &w) in sc.iter().enumerate() {
            c.score[r] += w;
            if w > 0 && c.best[r].is_none_or(|(b, _)| w > b) {
                c.best[r] = Some((w, i));
            }
        }
    }

    // Declared but not visibly used: font vars, Tailwind / theme.json families. A role named in a
    // token or config key counts, but less than real selector usage.
    let mut declared: Vec<(String, String, String)> = scan
        .vars
        .iter()
        .filter(|(_, (s, _))| !s.starts_with("var("))
        .map(|(n, (s, f))| (n.clone(), s.clone(), format!("{} (--{})", f, n)))
        .collect();
    declared.extend(scan.configured.iter().cloned());
    for (name, stack, ev) in declared {
        if GENERIC_FAMILIES.contains(&primary_family(&stack).to_ascii_lowercase().as_str()) {
            continue;
        }
        let hint = name_role_hint(&name).or_else(|| name_role_hint(&stack).filter(|r| *r == 2));
        let c = candidate(&mut cands, &stack);
        if !c.evidence.iter().any(|e| e.contains(&format!("(--{}", name))) {
            c.note(ev);
        }
        if let Some(r) = hint {
            c.score[r] += 1;
        }
    }

    // Loaded fonts attach to the stack that names them, or stand alone
    for (family, ev) in &scan.loaded {
        let fam = family.to_ascii_lowercase();
        let names_it = |c: &FontCandidate| {
            c.stack.split(',').any(|f| {
                let f = f.trim().to_ascii_lowercase();
                f == fam || f.starts_with(&format!("{} ", fam)) || fam.starts_with(&format!("{} ", f))
            })
        };
        match cands.iter_mut().find(|c| names_it(c)) {
            Some(c) => c.note(ev.clone()),
            None => {
                let c = candidate(&mut cands, family);
                c.note(ev.clone());
                if name_role_hint(family) == Some(2) {
                    c.score[2] += 1;
                }
            }
        }
    }

    if cands.is_empty() {
        return None;
    }

    let pick = |role: usize, taken: &[Option<usize>]| -> Option<usize> {
        cands
            .iter()
            .enumerate()
            .filter(|(i, c)| c.score[role] > 0 && !taken.contains(&Some(*i)))
            .max_by(|(_, a), (_, b)| a.score[role].cmp(&b.score[role]).then(b.first_seen.cmp(&a.first_seen)))
            .map(|(i, _)| i)
    };
    let mono = pick(2, &[]);
    let body = pick(1, &[mono]).or_else(|| {
        // nothing styles body/html: first non-mono candidate in discovery order
        cands
            .iter()
            .enumerate()
            .filter(|(i, c)| Some(*i) != mono && c.score[0] == 0)
            .min_by_key(|(_, c)| c.first_seen)
            .map(|(i, _)| i)
    });
    let display = pick(0, &[mono]);

    let metric = |c: &FontCandidate, role: usize, f: fn(&FontUse) -> Option<String>| -> String {
        c.best[role]
            .and_then(|(_, i)| f(&scan.uses[i]))
            .unwrap_or_else(|| "inherit".to_string())
    };
    let token = |c: &FontCandidate, role: usize, name: &str| TypographyToken {
        role: name.to_string(),
        font_family: c.stack.clone(),
        font_size: metric(c, role, |u| u.size.clone()),
        font_weight: metric(c, role, |u| u.weight.clone()),
        line_height: metric(c, role, |u| u.line_height.clone()),
        letter_spacing: metric(c, role, |u| u.letter_spacing.clone()),
        tier: KnowledgeTier::ObservedFact,
        source: c.evidence.iter().take(4).cloned().collect::<Vec<_>>().join("; "),
    };

    let mut out = Vec::new();
    let mut used: Vec<usize> = Vec::new();
    let mut notes: Vec<&str> = Vec::new();
    match (display, body) {
        (Some(d), _) => {
            out.push(token(&cands[d], 0, "display"));
            used.push(d);
        }
        (None, Some(b)) => {
            let mut t = token(&cands[b], 1, "display");
            t.source = format!("{}; no heading font declared, so headings inherit the body font", t.source);
            out.push(t);
            notes.push("headings inherit the body font");
        }
        (None, None) => {}
    }
    match body {
        Some(b) => {
            out.push(token(&cands[b], 1, "body"));
            used.push(b);
        }
        None => {
            out.push(TypographyToken {
                role: "body".into(),
                font_family: "serif".into(),
                font_size: "inherit".into(),
                font_weight: "inherit".into(),
                line_height: "inherit".into(),
                letter_spacing: "inherit".into(),
                tier: KnowledgeTier::InferredRule,
                source: "inferred: no body font-family declared, so the browser default (serif) applies".into(),
            });
            notes.push("no body font declared");
        }
    }
    if let Some(m) = mono.filter(|m| !used.contains(m)) {
        out.push(token(&cands[m], 2, "mono"));
        used.push(m);
    }
    let mut extras = 0;
    for (i, c) in cands.iter().enumerate() {
        if out.len() >= 6 {
            break;
        }
        if used.contains(&i) || GENERIC_FAMILIES.contains(&primary_family(&c.stack).to_ascii_lowercase().as_str()) {
            continue;
        }
        let role = (0..3).max_by_key(|r| (c.score[*r], -(*r as i32))).unwrap_or(1);
        let mut t = token(c, role, "extra");
        extras += 1;
        t.role = format!("extra-{}", extras);
        out.push(t);
    }
    let mut summary = out
        .iter()
        .filter(|t| t.tier == KnowledgeTier::ObservedFact)
        .map(|t| format!("{} = {}", t.role, primary_family(&t.font_family)))
        .collect::<Vec<_>>()
        .join(", ");
    if !notes.is_empty() {
        summary = format!("{} ({})", summary, notes.join("; "));
    }
    Some((out, summary))
}

#[cfg(test)]
mod font_tests {
    use super::*;

    fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("layoza_fonts_{}", name));
        let _ = fs::remove_dir_all(&dir);
        for (rel, body) in files {
            let p = dir.join(rel);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, body).unwrap();
        }
        dir
    }

    fn role<'a>(dna: &'a SiteDna, r: &str) -> &'a TypographyToken {
        dna.typography
            .iter()
            .find(|t| t.role == r)
            .unwrap_or_else(|| panic!("no `{r}` role in {:?}", dna.typography))
    }

    #[test]
    fn google_fonts_and_minified_vars_map_by_usage() {
        let html = r#"<link href="https://fonts.googleapis.com/css2?family=Archivo:wdth,wght@75..100,400..800&family=Gloock&family=Stardos+Stencil:wght@700&display=swap" rel="stylesheet"><style>:root{--display:"Gloock", Georgia, serif;--sans:"Archivo", "Helvetica Neue", Arial, sans-serif;--stencil:"Stardos Stencil", "Archivo", sans-serif}body{margin:0;font-family:var(--sans);font-size:17px;line-height:1.55}h1{font-family:var(--display);font-weight:400;font-size:clamp(76px, 8.6vw, 128px);line-height:.9;letter-spacing:-.035em}.tag{font-family:var(--stencil)}</style>"#;
        let dir = project("google", &[("index.html", html)]);
        let (dna, _) = inspect_project(&dir, None, false).unwrap();
        assert_eq!(dna.typography[0].role, "display");
        assert_eq!(dna.typography[1].role, "body");
        let d = role(&dna, "display");
        assert!(d.font_family.starts_with("Gloock"), "{d:?}");
        assert_eq!(d.tier, KnowledgeTier::ObservedFact);
        assert_eq!(d.font_size, "clamp(76px, 8.6vw, 128px)");
        assert!(d.source.contains("index.html") && d.source.contains("Google Fonts"), "{}", d.source);
        let b = role(&dna, "body");
        assert!(b.font_family.starts_with("Archivo"), "{b:?}");
        assert_eq!(b.font_size, "17px");
        assert!(dna.typography.iter().any(|t| t.font_family.starts_with("Stardos Stencil")));
        assert!(!dna.typography.iter().any(|t| t.font_family.contains("Instrument Serif")));
        assert!(dna.decisions.iter().any(|d| d.id == "DEC-003" && d.tier == KnowledgeTier::ObservedFact));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn fontsource_and_font_shorthand() {
        let css = "@import '@fontsource-variable/space-grotesk';\n@import '@fontsource/ibm-plex-mono/400.css';\n:root {\n  --sans: 'Space Grotesk Variable', 'Space Grotesk', system-ui, sans-serif;\n  --mono: 'IBM Plex Mono', ui-monospace, monospace;\n}\nbody { font: 400 17px/1.55 var(--sans); }\n.mono { font-family: var(--mono); }\n";
        let pkg = r#"{"dependencies":{"@fontsource-variable/space-grotesk":"^5","@fontsource/ibm-plex-mono":"^5","astro":"^5"}}"#;
        let dir = project("fontsource", &[("src/styles/global.css", css), ("package.json", pkg)]);
        let (dna, _) = inspect_project(&dir, None, false).unwrap();
        let b = role(&dna, "body");
        assert!(b.font_family.starts_with("Space Grotesk Variable"), "{b:?}");
        assert_eq!((b.font_size.as_str(), b.line_height.as_str(), b.font_weight.as_str()), ("17px", "1.55", "400"));
        assert!(b.source.contains("@fontsource-variable/space-grotesk"), "{}", b.source);
        // no heading font: display reports the inherited body font rather than a made-up serif
        let d = role(&dna, "display");
        assert!(d.font_family.starts_with("Space Grotesk") && d.source.contains("inherit"), "{d:?}");
        let m = role(&dna, "mono");
        assert!(m.font_family.starts_with("IBM Plex Mono"), "{m:?}");
        assert!(m.source.contains("@fontsource/ibm-plex-mono"), "{}", m.source);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn tailwind_next_font_and_theme_json() {
        let tw = "module.exports = { theme: { extend: { fontFamily: { display: ['Fraunces', 'serif'], sans: ['var(--font-inter)', 'system-ui'], mono: ['\"JetBrains Mono\"', 'monospace'] } } } }";
        let layout = "import { Inter } from 'next/font/google';\nconst inter = Inter({ subsets: ['latin'], variable: '--font-inter' });\n";
        let dir = project("tailwind", &[("tailwind.config.js", tw), ("app/layout.tsx", layout)]);
        let (dna, _) = inspect_project(&dir, None, false).unwrap();
        assert!(role(&dna, "display").font_family.starts_with("Fraunces"));
        assert!(role(&dna, "mono").font_family.starts_with("JetBrains Mono"));
        assert!(dna.typography.iter().any(|t| t.font_family.starts_with("Inter") && t.source.contains("next/font")), "{:?}", dna.typography);
        let _ = fs::remove_dir_all(&dir);

        let theme = r#"{"version":3,"settings":{"typography":{"fontFamilies":[{"slug":"heading","fontFamily":"\"Playfair Display\", serif"},{"slug":"base","fontFamily":"Lato, sans-serif"}]}},"styles":{"typography":{"fontFamily":"var:preset|font-family|base","fontSize":"1.0625rem"},"elements":{"heading":{"typography":{"fontFamily":"var(--wp--preset--font-family--heading)"}}}}}"#;
        let dir = project("themejson", &[("theme.json", theme)]);
        let (dna, _) = inspect_project(&dir, None, false).unwrap();
        assert!(role(&dna, "display").font_family.starts_with("Playfair Display"));
        let b = role(&dna, "body");
        assert!(b.font_family.starts_with("Lato") && b.font_size == "1.0625rem", "{b:?}");
        assert!(b.source.contains("theme.json"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn no_fonts_falls_back_and_says_so() {
        let dir = project("none", &[("index.html", "<main><h1>Hi</h1></main>")]);
        let (dna, _) = inspect_project(&dir, None, false).unwrap();
        assert!(dna.typography.iter().all(|t| t.tier == KnowledgeTier::InferredRule));
        assert!(dna.typography[0].source.starts_with("inferred-baseline"));
        assert!(dna.confidence.unknowns.iter().any(|u| u.starts_with("Typography: no font declarations")));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn font_shorthand_parsing() {
        assert_eq!(
            parse_font_shorthand("italic 600 1.25rem / 1.2 \"Inter\", sans-serif"),
            Some(("\"Inter\", sans-serif".into(), Some("1.25rem".into()), Some("600".into()), Some("1.2".into())))
        );
        assert_eq!(parse_font_shorthand("inherit"), None);
        assert!(!looks_like_font_stack("font-size-lg", "1.25rem"));
        assert!(!looks_like_font_stack("ink", "#111"));
        assert!(looks_like_font_stack("display", "\"Gloock\", Georgia, serif"));
        assert!(!looks_like_font_stack("ease", "cubic-bezier(.2,1,.3,1)"));
    }

    #[test]
    fn sketch_and_draft_fonts_do_not_override_the_shipped_source() {
        let css = ":root { --sans: 'Space Grotesk', system-ui, sans-serif; --mono: 'IBM Plex Mono', monospace; }\nbody { font: 400 17px/1.55 var(--sans); }\ncode { font-family: var(--mono); }\n";
        let sketch = r#"<style>:root{--display:"Bricolage Grotesque",sans-serif;--mono:"JetBrains Mono",monospace}h1{font-family:var(--display)}code{font-family:var(--mono)}</style>"#;
        let dir = project("drafts", &[("src/styles/global.css", css), ("sketches/old-draft/index.html", sketch)]);
        let (dna, _) = inspect_project(&dir, None, false).unwrap();
        assert!(role(&dna, "body").font_family.starts_with("Space Grotesk"));
        assert!(role(&dna, "display").font_family.starts_with("Space Grotesk"), "{:?}", dna.typography);
        assert!(role(&dna, "mono").font_family.starts_with("IBM Plex Mono"), "{:?}", dna.typography);
        assert!(!dna.typography.iter().any(|t| t.source.contains("sketches/")), "{:?}", dna.typography);
        // Fonts found only in drafts still count, and the decision says where they came from
        let only = project("drafts_only", &[("sketches/a/index.html", sketch)]);
        let (dna, _) = inspect_project(&only, None, false).unwrap();
        assert!(role(&dna, "display").font_family.starts_with("Bricolage Grotesque"));
        let dec = dna.decisions.iter().find(|d| d.id == "DEC-003").unwrap();
        assert!(dec.statement.contains("sketch/draft/test"), "{}", dec.statement);
        assert!(is_draft_path("tests/fixtures/a.css") && !is_draft_path("src/tests.css") && !is_draft_path("index.html"));
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::remove_dir_all(&only);
    }
}
