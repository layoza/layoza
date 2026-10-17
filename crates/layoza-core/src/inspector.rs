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
        if let Ok(meta) = fs::metadata(&full) {
            let len = meta.len();
            hash ^= len;
            hash = hash.wrapping_mul(0x100000001b3);
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
    let mut typography = Vec::new();
    let mut keyframes = BTreeSet::new();
    let mut honors_reduced_motion = false;
    let mut dark_mode = "light-default".to_string();
    let mut a11y_patterns = BTreeSet::new();
    let mut components = Vec::new();
    let mut nav_elements = BTreeSet::new();
    let mut footer_elements = BTreeSet::new();
    let mut grid_systems = BTreeSet::new();
    let mut max_widths = BTreeSet::new();

    // 1. Inspect package.json if present
    let pkg_path = project_root.join("package.json");
    if let Ok(pkg_raw) = fs::read_to_string(&pkg_path) {
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
            extract_wp_theme_json_tokens(&raw, &mut colors, &mut seen_color_names, &mut typography);
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
                variants: vec!["default".to_string()],
                props_or_classes: Vec::new(),
                accessible_patterns: Vec::new(),
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
        {
            if let Ok(content) = fs::read_to_string(&full) {
                scan_source_content(
                    rel,
                    &content,
                    &mut colors,
                    &mut seen_color_names,
                    &mut typography,
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

    if typography.is_empty() {
        typography.push(TypographyToken {
            role: "h1".to_string(),
            font_family: "Instrument Serif, Georgia, serif".to_string(),
            font_size: "clamp(2.5rem, 5vw, 4.25rem)".to_string(),
            font_weight: "600".to_string(),
            line_height: "1.08".to_string(),
            letter_spacing: "-0.025em".to_string(),
            tier: KnowledgeTier::InferredRule,
        });
        typography.push(TypographyToken {
            role: "body".to_string(),
            font_family: "Plus Jakarta Sans, system-ui, sans-serif".to_string(),
            font_size: "1rem".to_string(),
            font_weight: "400".to_string(),
            line_height: "1.6".to_string(),
            letter_spacing: "0em".to_string(),
            tier: KnowledgeTier::InferredRule,
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

    let mut spacing_scale = BTreeMap::new();
    spacing_scale.insert("xs".to_string(), "4px".to_string());
    spacing_scale.insert("sm".to_string(), "8px".to_string());
    spacing_scale.insert("md".to_string(), "16px".to_string());
    spacing_scale.insert("lg".to_string(), "32px".to_string());
    spacing_scale.insert("xl".to_string(), "64px".to_string());
    spacing_scale.insert("section".to_string(), "96px".to_string());

    let mut radii_scale = BTreeMap::new();
    radii_scale.insert("sm".to_string(), "6px".to_string());
    radii_scale.insert("md".to_string(), "12px".to_string());
    radii_scale.insert("lg".to_string(), "20px".to_string());

    let mut shadows = BTreeMap::new();
    shadows.insert(
        "sm".to_string(),
        "0 1px 2px rgba(15, 23, 42, 0.06)".to_string(),
    );
    shadows.insert(
        "md".to_string(),
        "0 10px 30px -10px rgba(15, 23, 42, 0.10)".to_string(),
    );

    let mut breakpoints = BTreeMap::new();
    breakpoints.insert("mobile".to_string(), "375px".to_string());
    breakpoints.insert("tablet".to_string(), "768px".to_string());
    breakpoints.insert("desktop".to_string(), "1280px".to_string());

    let mut decisions = Vec::new();
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
        content_max_widths: if max_widths.is_empty() {
            vec!["1280px".to_string()]
        } else {
            max_widths.into_iter().collect()
        },
        grid_systems: if grid_systems.is_empty() {
            vec!["CSS Grid".to_string(), "Flexbox".to_string()]
        } else {
            grid_systems.into_iter().collect()
        },
        dark_mode_strategy: dark_mode,
        accessibility_patterns: a11y_patterns.into_iter().collect(),
        motion: MotionProfile {
            libraries_detected: motion_libs.into_iter().collect(),
            keyframes_detected: keyframes.into_iter().collect(),
            transition_durations: vec!["180ms".to_string(), "320ms".to_string(), "600ms".to_string()],
            easing_curves: vec!["cubic-bezier(0.22, 1, 0.36, 1)".to_string()],
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
    typography: &mut Vec<TypographyToken>,
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
                } else if name.contains("font") && typography.len() < 6 {
                    typography.push(TypographyToken {
                        role: name.clone(),
                        font_family: val.trim_matches('"').trim_matches('\'').to_string(),
                        font_size: "1rem".to_string(),
                        font_weight: "400".to_string(),
                        line_height: "1.5".to_string(),
                        letter_spacing: "0em".to_string(),
                        tier: KnowledgeTier::ObservedFact,
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
    typography: &mut Vec<TypographyToken>,
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
    if let Some(families) = val
        .pointer("/settings/typography/fontFamilies")
        .and_then(|v| v.as_array())
    {
        for item in families {
            let slug = item.get("slug").and_then(|s| s.as_str()).unwrap_or("body");
            let family = item
                .get("fontFamily")
                .and_then(|s| s.as_str())
                .unwrap_or("system-ui, sans-serif");
            typography.push(TypographyToken {
                role: slug.to_string(),
                font_family: family.to_string(),
                font_size: "1rem".to_string(),
                font_weight: "400".to_string(),
                line_height: "1.5".to_string(),
                letter_spacing: "0em".to_string(),
                tier: KnowledgeTier::ObservedFact,
            });
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
