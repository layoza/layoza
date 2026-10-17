// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Design Quality, Browser Verification, WCAG 2.2 AA Accessibility, Core Web Vitals (Lab vs Field),
// Before/After Regression Diffing, and Bounded Auto-Repair Engine.

use crate::dna::SiteDna;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Deterministic, objective technical/accessibility/layout rule
    DeterministicObjective,
    /// Context-aware design critique (never presented as a scientific measurement)
    SubjectiveCritique,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub rule_id: String,
    pub kind: FindingKind,
    pub severity: Severity,
    pub category: String,
    pub file_or_selector: String,
    pub message: String,
    pub remediation: String,
    pub auto_repairable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWebVitalsEvaluation {
    /// Either `"laboratory_synthetic"` (Lighthouse/Playwright lab) or `"field_crux_rum"` (real-user field data)
    pub methodology_source: String,
    pub methodology_disclosure: String,
    /// `None` means not measured; never treated as a pass.
    pub lcp_ms: Option<f64>,
    pub lcp_passes_target: Option<bool>, // <= 2500ms
    pub inp_or_tbt_ms: Option<f64>,
    pub inp_passes_target: Option<bool>, // <= 200ms
    pub cls: Option<f64>,
    pub cls_passes_target: Option<bool>, // <= 0.10
    /// True only when all three metrics were measured.
    pub measured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityReport {
    pub target_standard: String,
    pub automated_violations: usize,
    pub passes_automated_wcag22_aa: bool,
    pub manual_review_required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualRegressionComparison {
    pub baseline_present: bool,
    pub viewports_compared: Vec<String>,
    pub pixel_diff_ratio: f64,
    pub intended_changes: Vec<String>,
    pub unintended_regressions: Vec<String>,
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub target_path: String,
    pub passed_release_gate: bool,
    pub objective_error_count: usize,
    pub objective_warning_count: usize,
    pub subjective_critique_notes: usize,
    pub findings: Vec<AuditFinding>,
    pub accessibility: AccessibilityReport,
    pub core_web_vitals: CoreWebVitalsEvaluation,
    pub visual_regression: Option<VisualRegressionComparison>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoRepairResult {
    pub file_path: String,
    pub applied_repairs: Vec<String>,
    pub diff_summary: String,
    pub modified: bool,
}

/// Audits a project directory or HTML/CSS/Component file using Layoza's deterministic detector rules
/// and context-aware critique rules.
pub fn audit_project(
    project_root: &Path,
    existing_dna: Option<&SiteDna>,
    lab_lcp_ms: Option<f64>,
    lab_inp_ms: Option<f64>,
    lab_cls: Option<f64>,
) -> Result<AuditReport, String> {
    if !project_root.exists() {
        return Err(format!(
            "Audit target `{}` does not exist.",
            project_root.display()
        ));
    }

    let mut findings = Vec::new();
    let mut files_to_scan = Vec::new();
    if project_root.is_file() {
        files_to_scan.push(project_root.to_path_buf());
    } else {
        collect_auditable_files(project_root, &mut files_to_scan, 0);
    }

    for file_path in &files_to_scan {
        let Ok(content) = fs::read_to_string(file_path) else {
            continue;
        };
        let label = file_path
            .strip_prefix(project_root)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/");
        audit_file_content(&label, &content, existing_dna, &mut findings);
    }

    // 7. Ingest the live Chromium QA report written by `scripts/browser-qa.mjs`, if present
    let base_dir = if project_root.is_file() {
        project_root.parent().unwrap_or(project_root)
    } else {
        project_root
    };
    let mut browser_cwv: (Option<f64>, Option<f64>, Option<f64>) = (None, None, None);
    let mut browser_cwv_source = String::new();
    for candidate_rel in ["qa/browser-qa-report.json", "qa-report.json", ".layoza/qa-snapshot.json"] {
        let qa_path = base_dir.join(candidate_rel);
        let Ok(raw_json) = fs::read_to_string(&qa_path) else {
            continue;
        };
        let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw_json) else {
            findings.push(browser_finding(
                "BROWSER-QA-UNREADABLE",
                Severity::Warning,
                "browser-qa",
                candidate_rel,
                "Browser QA report is not valid JSON and was ignored.".into(),
                "Re-run `layoza qa <file-or-url>`.",
            ));
            break;
        };
        let schema_major = val
            .get("schema_version")
            .and_then(|v| v.as_str())
            .and_then(|v| v.split('.').next())
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(1);
        if schema_major < 2 {
            findings.push(browser_finding(
                "BROWSER-QA-LEGACY",
                Severity::Warning,
                "browser-qa",
                candidate_rel,
                "Browser QA report uses the legacy v1 schema, whose performance and contrast values were estimated rather than measured; it was ignored.".into(),
                "Re-run `layoza qa <file-or-url>` to produce a measured v2 report.",
            ));
            break;
        }
        if let Some(stale) = newest_source_after(&files_to_scan, &qa_path) {
            findings.push(browser_finding(
                "BROWSER-QA-STALE",
                Severity::Warning,
                "browser-qa",
                candidate_rel,
                format!("Browser QA report is older than `{}`; it was ignored.", stale),
                "Re-run `layoza qa <file-or-url>` after editing.",
            ));
            break;
        }
        ingest_browser_report(&val, candidate_rel, &mut findings);
        browser_cwv = (
            val.get("lcp_ms").and_then(|n| n.as_f64()),
            val.get("inp_ms").and_then(|n| n.as_f64()),
            val.get("cls").and_then(|n| n.as_f64()),
        );
        browser_cwv_source = candidate_rel.to_string();
        break;
    }

    // Caller-supplied lab numbers take precedence over the browser report. Nothing is invented.
    let lcp = lab_lcp_ms.or(browser_cwv.0);
    let inp = lab_inp_ms.or(browser_cwv.1);
    let cls = lab_cls.or(browser_cwv.2);
    let measured = lcp.is_some() && inp.is_some() && cls.is_some();
    let cwv = CoreWebVitalsEvaluation {
        methodology_source: "laboratory_synthetic".to_string(),
        methodology_disclosure: if browser_cwv_source.is_empty() {
            "Laboratory values supplied by the caller, if any. Not CrUX field telemetry.".to_string()
        } else {
            format!(
                "Measured in one headless Chromium load per viewport (`{}`): LCP/CLS via PerformanceObserver, INP via an Event Timing proxy. Lab diagnostics, not CrUX field telemetry.",
                browser_cwv_source
            )
        },
        lcp_ms: lcp,
        lcp_passes_target: lcp.map(|v| v <= 2500.0),
        inp_or_tbt_ms: inp,
        inp_passes_target: inp.map(|v| v <= 200.0),
        cls,
        cls_passes_target: cls.map(|v| v <= 0.10),
        measured,
    };
    if !measured {
        findings.push(browser_finding(
            "CWV-NOT-MEASURED",
            Severity::Warning,
            "performance",
            ".",
            "Core Web Vitals were not measured, so performance is unverified (not passed).".into(),
            "Run `layoza qa <file-or-url>` or pass lab_lcp_ms / lab_inp_ms / lab_cls.",
        ));
    }
    let cwv_failed = [cwv.lcp_passes_target, cwv.inp_passes_target, cwv.cls_passes_target].contains(&Some(false));

    let objective_error_count = findings
        .iter()
        .filter(|f| f.kind == FindingKind::DeterministicObjective && f.severity == Severity::Error)
        .count();
    let objective_warning_count = findings
        .iter()
        .filter(|f| f.kind == FindingKind::DeterministicObjective && f.severity == Severity::Warning)
        .count();
    let subjective_critique_notes = findings
        .iter()
        .filter(|f| f.kind == FindingKind::SubjectiveCritique)
        .count();

    let a11y_violations = findings
        .iter()
        .filter(|f| f.category == "accessibility" && f.severity == Severity::Error)
        .count();

    let passed_release_gate = objective_error_count == 0 && !cwv_failed;

    Ok(AuditReport {
        target_path: project_root.to_string_lossy().replace('\\', "/"),
        passed_release_gate,
        objective_error_count,
        objective_warning_count,
        subjective_critique_notes,
        findings,
        accessibility: AccessibilityReport {
            target_standard: "WCAG 2.2 Level AA".to_string(),
            automated_violations: a11y_violations,
            passes_automated_wcag22_aa: a11y_violations == 0,
            manual_review_required: vec![
                "Verify keyboard Tab/Shift+Tab focus order matches visual reading sequence across 375px, 768px, and 1280px viewports (WCAG 2.4.3).".to_string(),
                "Verify focus appearance and non-obscuration when sticky headers or dialogs are active (WCAG 2.4.11 / 2.4.13).".to_string(),
                "Verify screen-reader announcement clarity for dynamic state changes and live regions (WCAG 4.1.3).".to_string(),
            ],
        },
        core_web_vitals: cwv,
        visual_regression: None,
    })
}

fn collect_auditable_files(current: &Path, out: &mut Vec<std::path::PathBuf>, depth: usize) {
    if depth > 6 {
        return;
    }
    let Ok(entries) = fs::read_dir(current) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if matches!(
            name,
            ".git" | "node_modules" | "target" | ".next" | "dist" | ".layoza"
        ) {
            continue;
        }
        if p.is_dir() {
            collect_auditable_files(&p, out, depth + 1);
        } else {
            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            if matches!(
                ext,
                "html" | "css" | "tsx" | "jsx" | "astro" | "vue" | "svelte"
            ) {
                out.push(p);
            }
        }
    }
}

fn browser_finding(
    rule_id: &str,
    severity: Severity,
    category: &str,
    location: &str,
    message: String,
    remediation: &str,
) -> AuditFinding {
    AuditFinding {
        rule_id: rule_id.into(),
        kind: FindingKind::DeterministicObjective,
        severity,
        category: category.into(),
        file_or_selector: location.into(),
        message,
        remediation: remediation.into(),
        auto_repairable: false,
    }
}

/// Returns the first scanned source file modified after the QA report, if any.
fn newest_source_after(sources: &[std::path::PathBuf], report: &Path) -> Option<String> {
    let report_time = fs::metadata(report).and_then(|m| m.modified()).ok()?;
    sources.iter().find_map(|src| {
        let t = fs::metadata(src).and_then(|m| m.modified()).ok()?;
        (t > report_time).then(|| src.to_string_lossy().replace('\\', "/"))
    })
}

/// Maps rule ids emitted by `scripts/browser-qa.mjs` to (finding rule id, category, remediation).
fn browser_rule(rule: &str) -> (&'static str, &'static str, &'static str) {
    match rule {
        "document-overflow" | "text-clip-or-bleed" => (
            "BROWSER-HORIZONTAL-OVERFLOW",
            "layout-geometry",
            "Constrain widths (`max-width: 100%`, `min-width: 0` on flex/grid children) and scale display type with `clamp()` so nothing exceeds the viewport.",
        ),
        "interactive-overlap" => (
            "BROWSER-INTERACTIVE-OVERLAP",
            "layout-geometry",
            "Give overlapping controls space (gap, wrapping, or hiding secondary actions at small widths).",
        ),
        "label-wrap" => (
            "BROWSER-LABEL-WRAP",
            "layout-geometry",
            "Keep short UI labels on one line (`white-space: nowrap`) or shorten them at small widths.",
        ),
        "runaway-canvas" => (
            "BROWSER-RUNAWAY-CANVAS",
            "layout-geometry",
            "Size the canvas from its container with a ResizeObserver and cap devicePixelRatio; never derive height from its own rendered height.",
        ),
        "heading-crowding" => (
            "BROWSER-HEADING-CROWDING",
            "layout-spacing",
            "Increase column gap beside the heading or stack the columns at this width.",
        ),
        "cramped-leading" => (
            "BROWSER-CRAMPED-LEADING",
            "typography",
            "Raise multi-line display heading line-height to at least ~1.0.",
        ),
        "nav-unreachable" => (
            "BROWSER-NAV-UNREACHABLE",
            "responsive-navigation",
            "Provide a visible menu toggle (`<button aria-expanded aria-controls>`) whenever header nav links are hidden.",
        ),
        "menu-toggle-broken" => (
            "BROWSER-MENU-TOGGLE-BROKEN",
            "responsive-navigation",
            "Make the menu toggle reveal the navigation links and update `aria-expanded`.",
        ),
        "unlabeled-control" => (
            "A11Y-UNLABELED-CONTROL",
            "accessibility",
            "Give every control an accessible name (visible text, `aria-label`, or an associated `<label>`).",
        ),
        "text-hidden-after-scroll" => (
            "BROWSER-TEXT-HIDDEN",
            "motion",
            "Reveal animations must end visible; never leave text at opacity 0 (use IntersectionObserver with a no-JS/reduced-motion fallback).",
        ),
        "focus-obscured" => (
            "A11Y-FOCUS-OBSCURED",
            "accessibility",
            "Add `html { scroll-padding-top: <header height + 1rem> }` so focused and anchor-targeted elements land below the sticky header (WCAG 2.4.11).",
        ),
        "primary-action-below-fold" => (
            "PERSUADE-ACTION-BELOW-FOLD",
            "conversion",
            "Keep the hero's main action (buy, book, sign up) inside the first viewport at every width: shorten the headline, stack art after the copy on small screens, or move the action up.",
        ),
        "zoom-blocked" => (
            "A11Y-ZOOM-BLOCKED",
            "accessibility",
            "Remove `user-scalable=no` and any `maximum-scale` below 2 from the viewport meta; people with low vision must be able to pinch-zoom (WCAG 1.4.4).",
        ),
        "ios-input-zoom" => (
            "UI-IOS-INPUT-ZOOM",
            "usability",
            "Set form fields to at least 16px on small screens (e.g. `input, select, textarea { font-size: max(16px, 1rem) }`); iOS Safari zooms the page on focus otherwise.",
        ),
        "dark-color-scheme" => (
            "UI-DARK-COLOR-SCHEME",
            "usability",
            "Declare `:root { color-scheme: dark }` (or `<meta name=\"color-scheme\" content=\"dark\">`) so scrollbars, form controls and autofill match the dark page.",
        ),
        "image-no-size" => (
            "PERF-IMAGE-NO-SIZE",
            "performance",
            "Give every image `width` and `height` attributes (or a CSS `aspect-ratio`) so the browser reserves its box before it loads (CLS).",
        ),
        "pointer-not-button" => (
            "A11Y-POINTER-NOT-BUTTON",
            "accessibility",
            "Use a real `<button>` or `<a href>` for anything clickable; a `div` with `cursor: pointer` is unreachable by keyboard (WCAG 2.1.1).",
        ),
        "field-autocomplete" => (
            "UI-FIELD-AUTOCOMPLETE",
            "usability",
            "Add `autocomplete` tokens (`email`, `tel`, `name`, `street-address`, `postal-code`) and the matching `type`/`inputmode` to personal-data fields (WCAG 1.3.5).",
        ),
        "transition-all" => (
            "UI-TRANSITION-ALL",
            "performance",
            "Replace `transition: all` with the properties that should move (`transform`, `opacity`, `color`, `background-color`); `all` also animates layout and every future change.",
        ),
        "field-boundary-contrast" => (
            "UI-FIELD-BOUNDARY-CONTRAST",
            "accessibility",
            "Give form fields a border or fill with at least 3:1 contrast against the surface they sit on (WCAG 1.4.11), especially on coloured section fields.",
        ),
        "render-blocking-fonts" => (
            "UI-RENDER-BLOCKING-FONTS",
            "performance",
            "Load third-party font stylesheets without blocking first paint: preconnect, `media=\"print\" onload=\"this.media='all'\"` with a `<noscript>` copy, and metric-matched fallback `@font-face` rules.",
        ),
        "svg-text-illegible" => (
            "UI-SVG-TEXT-ILLEGIBLE",
            "accessibility",
            "SVG labels scaled down by the viewBox render under 9px: enlarge them at this width, hide them and carry the content in HTML, or swap to a mobile-specific drawing.",
        ),
        "text-offscreen" => (
            "UI-TEXT-OFFSCREEN",
            "layout",
            "Keep every label inside the viewport at every width: anchor edge labels inward (`text-anchor: start`/`end`, `left: 0`), inset the drawing, or re-stage it for phones. `overflow-x: hidden` on body hides the overflow, not the cut text.",
        ),
        "text-mid-reveal" => (
            "UI-TEXT-MID-REVEAL",
            "usability",
            "A visitor who stops scrolling must see finished headings: complete entrance reveals within ~600ms of entering view, or scrub them so every resting scroll position shows fully readable text (layoza-animations §2d).",
        ),
        "text-overlap" => (
            "UI-TEXT-OVERLAP",
            "usability",
            "Two pieces of visible text cover each other at a resting scroll position: give each its own space at every breakpoint and scroll step (reflow, resize, or sequence pinned content so one leaves before the next lands); only aria-hidden decorative type may sit under live text.",
        ),
        "bar-bleed" => (
            "UI-BAR-BLEED",
            "visual",
            "Content text reads through a translucent fixed or sticky bar as it scrolls beneath, so the bar's own labels sit on ghost type: make the bar opaque once content passes under it, or back it with a strong blur.",
        ),
        "sparse-frame" => (
            "UI-SPARSE-FRAME",
            "visual",
            "A resting scroll frame is nearly empty: every frame of a pinned or scroll-linked section must be a full composition, so scale the stage content up, bring the next beat in sooner, or shorten the pin.",
        ),
        "empty-panel" => (
            "UI-EMPTY-PANEL",
            "visual",
            "A large tinted or bordered box is mostly empty: fill it with real content (summary, policy, map, next step) or let it shrink to its content.",
        ),
        "ragged-control-row" => (
            "UI-RAGGED-ROW",
            "visual",
            "A row of pills, chips or filters wraps into a centred or right-aligned second line, or strands one control alone: make the row scroll horizontally, shrink the controls, or group them so every line is full.",
        ),
        "menu-escape-broken" => ("UI-MENU-ESCAPE","accessibility","Escape does not close the open mobile menu when focus is on its toggle: handle Escape on the document while the menu is open, close it and return focus to the toggle."),
        "menu-close-slow" => ("UI-MENU-CLOSE-SLOW","motion","The mobile menu takes longer than about 800ms to leave the screen after Escape: shorten its exit animation to about 200-300ms so closing feels instant."),
        "art-over-text" => ("UI-ART-OVER-TEXT","visual","Canvas, WebGL or video art is painted across a headline, CTA or body text at a resting scroll position: give the text its own zone or a solid plate, and keep the art beside or behind it, never through it."),
        "motion-too-sparse" => ("UI-MOTION-SPARSE","motion","Fewer distinct elements move, transition or follow scroll than the page's declared motion level expects: give each mid-page section its own dealt mechanic (`layoza mechanics deal`), or declare <meta name=\"layoza-motion\" content=\"calm\"> (or `none`) if a quiet page is intended."),
        "pinned-stage-static" => ("UI-PINNED-STATIC","motion","A sticky or pinned stage looks the same at 25%, 50% and 75% of its scroll range: map a visible change to scroll progress across the range (or shorten the pin), so every still is a different, complete composition."),
        "floating-control-covers-text" => ("UI-FLOAT-COVER","visual","A floating button or pill sits on top of body text while scrolling: dock it in a solid header, keep a matching gutter clear in the text column, or move it where text never runs."),
        "hero-empty-band" => (
            "UI-HERO-BAND",
            "visual",
            "A tall stretch of the first phone screen holds no text or art: bring the hero art up, crop it into the band, or tighten the spacing so the first frame is composed edge to edge.",
        ),
        "heading-break" => (
            "UI-HEADING-FIT",
            "visual",
            "A display heading splits a word across lines or has glyphs cut off by the viewport or a clipping parent: size display type with clamp() against the narrowest supported viewport so every word fits whole.",
        ),
        "small-submit" => (
            "UI-SMALL-SUBMIT",
            "usability",
            "Make each form's submit button at least 44px tall and full-size where the eye ends; a small submit reads as an afterthought and is hard to tap.",
        ),
        "dead-strip-below-footer" => (
            "UI-DEAD-STRIP-BELOW-FOOTER",
            "layout",
            "End the document on the last content: remove the leftover min-height, spacer or pinned-stage padding that leaves a blank band below the footer.",
        ),
        "broken-image" => (
            "BROWSER-BROKEN-IMAGE",
            "content",
            "Fix image URLs or remove the broken images.",
        ),
        _ => ("BROWSER-ISSUE", "browser-qa", "See the detail message."),
    }
}

/// Builds the non-blocking finding for one `critique` issue group emitted by the browser runner's
/// ported impeccable rules (`craft-<upstream-id>`). Always `SubjectiveCritique` / `Info`, so it never
/// changes `passed_release_gate` or the objective counts.
fn craft_finding(rule: &str, location: &str, vp_label: &str, detail: &str, more: &str) -> AuditFinding {
    let id = rule.strip_prefix("craft-").unwrap_or(rule);
    let (name, description) = craft_rule(id);
    let brand_note = if matches!(id, "overused-font" | "italic-serif-display" | "cream-palette" | "ai-color-palette") {
        " If the existing brand requires this, declare it with <meta name=\"layoza-brand-allow\" content=\"font:<family>, palette:<cream|purple|cyan>\"> and the rule stands down."
    } else {
        ""
    };
    AuditFinding {
        rule_id: format!("CRAFT-{}", id.to_ascii_uppercase()),
        kind: FindingKind::SubjectiveCritique,
        severity: Severity::Info,
        category: "design-critique".into(),
        file_or_selector: location.into(),
        message: format!("Design critique: {} at {}: {}{}", name, vp_label, detail, more),
        remediation: format!("{}{}", description, brand_note),
        auto_repairable: false,
    }
}

/// Names and descriptions of the design-critique rules ported into `scripts/browser-qa.mjs`.
/// Text copied from pbakaus/impeccable (Apache-2.0), commit 2f98774,
/// crates/foundation/src/registry.rs (`ANTIPATTERNS`).
fn craft_rule(id: &str) -> (&'static str, &'static str) {
    match id {
        "side-tab" => ("Side-tab accent border", "Thick colored border on one side of a card — the most recognizable tell of AI-generated UIs. Use a subtler accent or remove it entirely."),
        "border-accent-on-rounded" => ("Border accent on rounded element", "Thick accent border on a rounded card — the border clashes with the rounded corners. Remove the border or the border-radius."),
        "overused-font" => ("Overused font", "Inter, Roboto, Fraunces, Geist, Plus Jakarta Sans, and Space Grotesk are used on so many sites they no longer feel distinctive. Each new wave of AI-generated UIs converges on the same handful of faces. Choose a face that gives your interface personality."),
        "flat-type-hierarchy" => ("Flat type hierarchy", "Dominant heading and body roles are separated by less than 1.25× at every step, leaving the size hierarchy flat. Add at least one stronger size step."),
        "gradient-text" => ("Gradient text", "Gradient text is decorative rather than meaningful — a common AI tell, especially on headings and metrics. Use solid colors for text."),
        "ai-color-palette" => ("AI color palette", "Purple/violet gradients and cyan-on-dark are the most recognizable tells of AI-generated UIs. A gradient in one of those hues is the tell on its own; flat neon ink on a dark ground is charged once a second tell hue joins it. Choose a distinctive, intentional palette."),
        "cream-palette" => ("Cream / beige palette", "A warm cream or beige page background has become the default \"tasteful\" AI surface, reached for by reflex. Choose a background that comes from a deliberate palette, not the safe warm off-white."),
        "nested-cards" => ("Nested cards", "Cards inside cards create visual noise and excessive depth. Flatten the hierarchy — use spacing, typography, and dividers instead of nesting containers."),
        "monotonous-spacing" => ("Monotonous spacing", "The same spacing value used everywhere — no rhythm, no variation. Use tight groupings for related items and generous separations between sections."),
        "bounce-easing" => ("Bounce or elastic easing", "Bounce and elastic easing feel dated and tacky. Real objects decelerate smoothly — use exponential easing (ease-out-quart/quint/expo) instead."),
        "pulsing-dot" => ("Pulsing status dot", "Small pulsing status dots simulate liveness decoratively. Reserve pulse animation for indicators tied to genuinely live, changing data; a static indicator with clear labeling is honest and calmer."),
        "blinking-cursor" => ("Decorative blinking cursor", "A blinking text cursor animated into a hero or landing section simulates typing where no input exists. It borrows the dev-tool aesthetic as decoration. Real editable fields draw their own caret; anywhere else, let the composition hold attention without a fake prompt."),
        "shape-assembled-illustration" => ("Shape-assembled illustration", "A large inline SVG that builds a pictorial scene from a pile of primitive shapes reads as placeholder clip art, not illustration. Icons, logos, and data graphics are fine at their scale; a hero-sized visual deserves real artwork, a photograph, or a deliberately drawn graphic."),
        "organic-clip-path" => ("Organic contour drawn as clip-path", "A clip-path polygon with many arbitrary vertices, or a curved clip-path path(), is CSS approximating a torn edge, blob, or silhouette. It reads as the cheap version of the effect and is usually a produced or photographic material replaced with code. Derive an alpha matte from the real image, or ship the shape as a cut-out raster; keep clip-path for geometry (cut corners, diagonals, hexagons)."),
        "buried-raster" => ("Raster buried under a wash or opacity", "A background image under a near-opaque gradient wash, or a raster on an element at near-zero opacity, never reaches the screen: the page shows the wash, and the produced texture or photo ships as a compliance token. Let the material show (a tint under 0.9 alpha, a blend mode, an opacity you can see) or remove the file."),
        "dark-glow" => ("Glowing shadow accents", "Colored glow shadows — a zero-offset chromatic halo (box- or text-shadow) on any background, or any colored blurred shadow on a dark background — are the default \"cool\" look of AI-generated UIs. Use neutral elevation shadows and subtle, purposeful lighting instead."),
        "radial-halo" => ("Radial-gradient background halo", "A chromatic radial-gradient wash — saturated at the center, fading to transparent — used as a decorative background glow on a dark page. Same tell as glowing shadows, drawn with a gradient instead of a shadow. Ground the surface with a solid or subtly shifted background instead."),
        "radial-spotlight-glow" => ("Decorative radial spotlight glow", "An accent-colored radial gradient fading to transparent, dropped behind a hero or section as a \"spotlight\" and bright enough against that surface to read as a cloud floating over the copy. It is a reflex AI decoration — the translucent cousin of the saturated radial halo. Let the surface stand on its own, or light the composition with a deliberate material accent rather than a floating colored haze."),
        "marquee" => ("Auto-scrolling marquee", "Continuously auto-scrolling content demands attention it has not earned and hides half its content at any moment. Reserve motion for content that changes; let readers move at their own pace."),
        "icon-tile-stack" => ("Icon tile stacked above heading", "A small rounded-square icon container above a heading is the universal AI feature-card template — every generator outputs this exact shape. Try a side-by-side icon and heading, or let the icon sit in flow without its own container."),
        "italic-serif-display" => ("Italic serif display headline", "Oversized italic serif (Fraunces, Recoleta, Playfair, Newsreader-italic) as the primary hero headline reads as taste in isolation but has become the universal AI-startup landing page hero. Set roman, or move to a non-serif display face. Editorial / magazine register may legitimately want this — judge by context."),
        "hero-eyebrow-chip" => ("Hero eyebrow / pill chip", "A tiny uppercase letter-spaced label sitting immediately above an oversized hero headline — or the same shape rendered as a pill chip — is now the default AI SaaS hero. Drop the eyebrow, integrate the kicker into the headline, or run it as a navigation breadcrumb instead."),
        "kicker-above-heading" => ("Kicker / eyebrow label above heading", "A tiny tracked uppercase or small-caps label sitting as its own block directly above a heading is banned outright, repeated or not. Generated kickers never earn their place: the heading carries its own weight. Delete the label and let the heading speak; if the words matter, work them into the heading or the body."),
        "numbered-section-labels" => ("Tiny numbered section labels", "Small numeric index labels riding next to section headings, repeated section after section, are AI editorial scaffolding — a page numbering its own chapters instead of earning structure. Let hierarchy, content, and rhythm carry the sequence."),
        "em-dash-overuse" => ("Em-dash overuse", "Em-dash saturation in body copy is an AI cadence tell. Advisory only: humans use em-dashes legitimately, so this fires only on saturation — at least 8 em-dashes (— or --) at a density near one per 500 characters of body text — never on a long article that uses a few. Prefer commas, colons, periods, or parentheses."),
        "marketing-buzzword" => ("Marketing buzzword", "Generic SaaS phrases (streamline / empower / supercharge / world-class / enterprise-grade / next-generation / cutting-edge / etc) are instant AI tells. Pick a specific verb and noun that says what the product literally does."),
        "aphoristic-cadence" => ("Aphoristic-cadence copy", "Three or more sections landing on a short rebuttal sentence (\"X. No Y.\" / \"X. Just Y.\") or a manufactured-contrast aphorism (\"Not a feature. A platform.\") reads as AI cadence, not voice. Once is fine; the pattern is the tell."),
        "oversized-h1" => ("Oversized hero headline", "A full-sentence headline set at display size ends up dominating the viewport, leaving no room for anything else above the fold. A punchy one- or two-word headline at that size is fine — the problem is a long headline blown up too large. Set long headlines smaller, or tighten the copy."),
        "extreme-negative-tracking" => ("Crushed letter spacing", "Letter-spacing pulled tighter than the point where characters keep their own shapes costs legibility. Tighten display type optically, not destructively."),
        "edge-flush-cards" => ("Cards flush against the scroller edge", "Cards inside a horizontal scroller or tab panel sit flush against the container edge at rest while keeping a gutter on the other side, so their edges and rounded corners get cut off. Usually the panel is sized wider than its clip box. Keep a consistent inset on both sides."),
        "gray-on-color" => ("Gray text on colored background", "Gray text looks washed out on colored backgrounds. Use a darker shade of the background color instead, or white/near-white for contrast."),
        "line-length" => ("Line length too long", "Text lines wider than ~80 characters are hard to read. The eye loses its place tracking back to the start of the next line, so it is measured on the lines that rendered and charged when more than one of them runs long. Add a max-width (65ch to 75ch) to text containers."),
        "cramped-padding" => ("Cramped padding", "Text is too close to the edge of its container. Two shapes: (1) an element with its own text where the space between the rendered text and the border box is too small for the font size, and (2) a wrapper whose children's text lands flush against a visible boundary (border, outline, or non-transparent background) with nothing to inset it. Add at least 8px (ideally 12–16px) of space inside bordered, outlined, or colored containers."),
        "body-text-viewport-edge" => ("Body text touching viewport edge", "Body paragraphs render flush against the left or right viewport edge with no container providing horizontal padding: closer than 16px, or 12px on viewports 480px wide or narrower. Wrap content in a container with at least 16px (ideally 24-32px) of horizontal padding, or apply max-width with mx-auto. Body text that runs past the viewport edge is reported once per page as overflow, naming the widest element: constrain that element's width instead of adding padding."),
        "tight-leading" => ("Tight line height", "Line height below 1.3x the font size makes multi-line text hard to read. Use 1.5 to 1.7 for body text so lines have room to breathe."),
        "justified-text" => ("Justified text", "In a column this narrow, justifying without hyphenation stretches the word spaces until vertical rivers of white run down the block. Use text-align: left, widen the measure, or enable hyphens: auto if you must justify. Scripts that justify on a character grid or by elongating glyphs, such as CJK, Thai and Arabic, are not affected and are not reported."),
        "tiny-text" => ("Tiny body text", "Body text below 12px is hard to read, especially on high-DPI screens. Use at least 14px for body content, 16px is ideal."),
        "undersized-ui-text" => ("Undersized functional text", "Interactive and content-bearing UI text (links, buttons, nav items, labels, table cells, meta rows, timecodes) below 11px is a legibility failure, not a style choice. WCAG sets no absolute pixel floor, but functional text under 11px is a defensible quality bar: it fails on high-DPI and small viewports and it degrades tap and read targets. The 11px floor holds even inside a footer; only non-interactive legal smallprint gets the softer 10px floor. Being ON the DESIGN.md size ramp does not exempt a value here: adding 8px to the ramp launders the token but not the legibility problem, and that is exactly the escape hatch this rule closes. Exempts sup/sub, visually-hidden (sr-only) text, and code/terminal contexts. Decorative letterspaced micro-labels are still functional and stay in scope."),
        "all-caps-body" => ("All-caps body text", "Long passages in uppercase are hard to read. We recognize words by shape (ascenders and descenders), which all-caps removes. Reserve uppercase for short labels and headings. Only a run that reads as a sentence counts: 80 characters or more of an element's own text. Buttons, nav items, kickers and eyebrows set in caps are a convention and stay silent."),
        "wide-tracking" => ("Wide letter spacing on body text", "Letter spacing above 0.05em on body text disrupts natural character groupings and slows reading. Reserve wide tracking for short uppercase labels only."),
        "repeated-container-text" => ("Same text repeated inside one container", "The same literal text rendered three or more times in structurally different spots inside a single card or panel is redundant messaging — usually a status or label wired into every slot of a template. Say it once, in the slot where it matters most."),
        "gpt-thin-border-wide-shadow" => ("Hairline border with wide shadow", "Every card of this row wears a hairline border paired with a wide, diffuse shadow, which is a recurring generated-UI signature. One floating panel earns both; a whole row wearing them reads as a default nobody chose. Across the row, commit to one: a defined edge or a soft elevation."),
        "repeating-stripes-gradient" => ("Repeating-gradient stripes", "Repeating-gradient stripes used as surface decoration are a recurring generated-UI signature. Reach for a deliberate texture or leave the surface plain."),
        "codex-grid-background" => ("Decorative grid-line background", "A decorative grid or line-field background drawn with hairline linear-gradient layers tiled by a fixed pixel cell is a recurring generated-UI signature. Reserve grid overlays for actual canvas, map, blueprint, or measurement surfaces; elsewhere use product structure or a plain surface."),
        "heading-rhythm" => ("Heading crowded against the previous block", "A heading binds to the content it introduces, so the rendered space above it should exceed the space below it. When headings across a page sit as close or closer to the block above than to their own content, every section reads as if it captions the previous one. Open up the space above each heading."),
        // Layoza-original generic-default tells (ideas: anthropics/skills frontend-design, Apache-2.0)
        "arrow-cta" => ("Arrow appended to action text", "Ending link and button labels with an arrow glyph (→, ↗, ») is template chrome that appears whatever the subject. Let the label say where it goes; keep arrows for real direction (next/previous, external links)."),
        "middot-meta" => ("Middle-dot meta strings", "Meta lines joined with middle dots ('A · B · C') are a recurring generated-page signature. Set the facts in a structure that fits them (a definition list, a table row, a sentence) or keep only the one that matters."),
        "caps-dash-label" => ("'WORD — fragment' labels", "Labels built as a capitalised word, a spaced em dash and a fragment read as generated editorial chrome. Write the label as plain words, or let the heading carry it."),
        "headline-accent-word" => ("Single accented word in a headline", "Setting one word or short phrase of a headline in another colour, italic or face is the commonest generated-headline treatment. Make the whole headline the type moment, or let the words carry the emphasis."),
        "dark-acid-accent" => ("Near-black with one acid accent", "A near-black ground with a single acid-green or vermilion accent is a default 'bold' look generators reach for regardless of subject. Derive ground and accent from the subject's own world."),
        "uniform-radius" => ("One radius on everything", "One border-radius on buttons, fields, cards and images regardless of hierarchy is the SaaS-kit default. Give radius a scale tied to size and role (tight on controls, larger on containers, none where the world is square)."),
        "card-kit" => ("Identical shadowed card rows", "Content chopped into identical rounded cards with the same soft shadow, section after section, is the SaaS card kit. Vary the structure by what each section holds: a list, a table, a feature at scale, prose."),
        "mono-meta-labels" => ("Monospace as label costume", "A monospace face on small data labels outside code borrows a 'technical' costume. Use mono only for code, tabular figures that need it, or a world that is genuinely typewritten."),
        "scattered-reveal" => ("Same entrance on every section", "A fade-and-slide-up entrance repeated on section after section reads as generated. One orchestrated moment lands better; motion that answers a person's action is welcome."),
        "vague-cta" => ("Vague action text", "\"Submit\", \"Click here\" and repeated \"Learn more\" do not say what happens. Name the outcome (\"Book the slot\", \"Send enquiry\") and keep that name through the flow."),
        "centered-prose" => ("Centre-aligned paragraphs", "Paragraphs of four or more lines set centred make every line start in a new place, which slows reading. Left-align running text; centre only short lines."),
        "font-family-sprawl" => ("Too many typefaces", "Four or more typefaces rendering text dilute the voice. One family, or two clearly distinct ones with fixed roles, carries a page."),
        "hero-metric" => ("Big-number stats as the hero", "A first viewport led by big-number stats with small labels is the default hero treatment. Open with the subject's most characteristic thing: the product, the place, the working action."),
        "display-scale" => ("Small display type", "On desktop no text reaches architectural scale (largest under 5vw), so the page reads as a smooth template type scale. Let one headline or the name reach about 8vw, with two or three sizes and nothing between, unless a small-type direction is deliberate (declare it with layoza-brand-allow)."),
        "dead-controls" => ("Dead controls", "Most buttons have no transition and no :active press state, so clicks feel unacknowledged. Give every control hover, press and state feedback (layoza-animations 2c); under a static brief an instant :active colour change is enough."),
        "hero-art-scale" => ("Small hero art", "The first desktop viewport opens as headline + buttons with, at most, a small visual beside them: the split-hero template whatever world paints it. Let an authored artefact of the world (drawn, built in SVG/CSS/canvas, or photographed) carry at least a fifth of the first screen, or make the type itself the artwork at that scale."),
        "theater-slop-phrase" => ("Theater framing copy","Dismissing something as \"theater\" is a recurring generated-copy tic. Say plainly what the thing does or does not do."),
        _ => (
            "Design critique",
            "A design anti-pattern flagged by the ported impeccable rules; review it against the brief and brand.",
        ),
    }
}

/// One off-system value reported by the browser runner's design-system rules (Layoza-original,
/// checked against the project's own DESIGN.md / SITE-DNA.json tokens).
struct DesignDrift {
    rule: String,
    value: String,
    detail: String,
    nearest_token: String,
    nearest_value: String,
    viewports: Vec<String>,
}

/// Design-system drift is objective (the page breaks its own declared system), so it is a
/// `DeterministicObjective` warning: it counts toward `objective_warning_count` but, like every
/// warning, never fails the release gate.
fn design_system_finding(d: &DesignDrift, source: &str) -> AuditFinding {
    let id = d.rule.trim_start_matches("design-system-");
    let what = match id {
        "color" => "colour",
        "font" => "font family",
        "radius" => "border radius",
        "font-size" => "font size",
        _ => "value",
    };
    let token = if d.nearest_token.is_empty() {
        "the nearest declared token".to_string()
    } else {
        format!("the `{}` token ({})", d.nearest_token, d.nearest_value)
    };
    AuditFinding {
        rule_id: format!("DESIGN-SYSTEM-{}", id.to_ascii_uppercase()),
        kind: FindingKind::DeterministicObjective,
        severity: Severity::Warning,
        category: "design-system".into(),
        file_or_selector: format!("{} ({})", source, d.viewports.join(", ")),
        message: format!(
            "Off-system {} `{}` at {}: {}",
            what,
            d.value,
            d.viewports.join(", "),
            d.detail
        ),
        remediation: format!(
            "Replace `{}` with {}, or add it to DESIGN.md / SITE-DNA.json if it is an intentional new token.",
            d.value, token
        ),
        auto_repairable: false,
    }
}

/// Converts a v2 browser QA report into audit findings (used for live URLs that have no local source).
pub fn browser_report_findings(report: &serde_json::Value, source: &str) -> Vec<AuditFinding> {
    let mut findings = Vec::new();
    ingest_browser_report(report, source, &mut findings);
    findings
}

fn ingest_browser_report(val: &serde_json::Value, source: &str, findings: &mut Vec<AuditFinding>) {
    let num = |k: &str| val.get(k).and_then(|n| n.as_f64());
    let flag = |k: &str| val.get(k).and_then(|b| b.as_bool());
    let strings = |k: &str| -> Vec<String> {
        val.get(k)
            .and_then(|s| s.as_array())
            .into_iter()
            .flatten()
            .filter_map(|s| s.as_str().map(String::from))
            .collect()
    };

    // Design-system drift: one finding per (rule, off-system value), merged across viewports
    let mut design_drift: Vec<DesignDrift> = Vec::new();
    for vp in val.get("viewports").and_then(|v| v.as_array()).into_iter().flatten() {
        let vp_label = vp.get("label").and_then(|l| l.as_str()).unwrap_or("viewport");
        let location = format!("{} ({})", source, vp_label);
        // One finding per (rule, severity) per viewport
        let mut grouped: Vec<(String, String, Vec<String>)> = Vec::new();
        for issue in vp.get("issues").and_then(|i| i.as_array()).into_iter().flatten() {
            let field = |k: &str| issue.get(k).and_then(|r| r.as_str()).unwrap_or("").to_string();
            let (rule, severity, detail) = (field("rule"), field("severity"), field("detail"));
            if rule.starts_with("design-system-") {
                let value = field("value");
                match design_drift.iter_mut().find(|d| d.rule == rule && d.value == value) {
                    Some(d) => {
                        if !d.viewports.iter().any(|v| v == vp_label) {
                            d.viewports.push(vp_label.to_string());
                        }
                    }
                    None => design_drift.push(DesignDrift {
                        rule,
                        value,
                        detail,
                        nearest_token: field("nearest_token"),
                        nearest_value: field("nearest_value"),
                        viewports: vec![vp_label.to_string()],
                    }),
                }
                continue;
            }
            match grouped.iter_mut().find(|(r, s, _)| *r == rule && *s == severity) {
                Some((_, _, details)) => details.push(detail),
                None => grouped.push((rule, severity, vec![detail])),
            }
        }
        for (rule, severity, details) in grouped {
            let more = if details.len() > 1 { format!(" (+{} more)", details.len() - 1) } else { String::new() };
            // Ported impeccable design critique: advisory only, never part of the release gate
            if severity == "critique" {
                findings.push(craft_finding(&rule, &location, vp_label, &details[0], &more));
                continue;
            }
            let (rule_id, category, remediation) = browser_rule(&rule);
            let sev = if severity == "error" { Severity::Error } else { Severity::Warning };
            findings.push(browser_finding(
                rule_id,
                sev,
                category,
                &location,
                format!("{} at {}: {}{}", rule, vp_label, details[0], more),
                remediation,
            ));
        }
        let console: Vec<&str> = vp
            .get("console_errors")
            .and_then(|c| c.as_array())
            .into_iter()
            .flatten()
            .filter_map(|c| c.as_str())
            .collect();
        if !console.is_empty() {
            findings.push(browser_finding(
                "BROWSER-CONSOLE-ERROR",
                Severity::Warning,
                "runtime",
                &location,
                format!("{} console/page error(s); first: {}", console.len(), console[0]),
                "Fix the runtime errors reported by the page.",
            ));
        }
    }

    for d in design_drift {
        findings.push(design_system_finding(&d, source));
    }

    if let Some(fails) = num("contrast_failures").filter(|n| *n > 0.0) {
        let samples = strings("contrast_failure_samples");
        findings.push(browser_finding(
            "A11Y-CONTRAST",
            Severity::Error,
            "accessibility",
            source,
            format!(
                "{} text element(s) below WCAG 1.4.3 contrast (min {:.2}:1). e.g. {}",
                fails,
                num("min_contrast_ratio").unwrap_or(0.0),
                samples.iter().take(3).cloned().collect::<Vec<_>>().join("; ")
            ),
            "Darken/lighten the text or its background until normal text reaches 4.5:1 and large text 3:1.",
        ));
    }
    if let Some(n) = num("decorative_contrast_failures").filter(|n| *n > 0.0) {
        findings.push(browser_finding(
            "A11Y-CONTRAST-DECORATIVE",
            Severity::Warning,
            "accessibility",
            source,
            format!(
                "{} visible aria-hidden text element(s) below contrast minimums. e.g. {}",
                n,
                strings("decorative_contrast_samples").iter().take(3).cloned().collect::<Vec<_>>().join("; ")
            ),
            "Raise contrast unless the text is purely decorative (WCAG 1.4.3 incidental exception).",
        ));
    }
    if let Some(n) = num("missing_alt_count").filter(|n| *n > 0.0) {
        findings.push(browser_finding(
            "A11Y-IMG-ALT-LIVE",
            Severity::Error,
            "accessibility",
            source,
            format!("{} rendered <img> element(s) lack an alt attribute (WCAG 1.1.1).", n),
            "Add descriptive `alt` text, or `alt=\"\"` for decorative images.",
        ));
    }
    if let Some(n) = num("undersized_targets").filter(|n| *n > 0.0) {
        findings.push(browser_finding(
            "A11Y-TARGET-SIZE",
            Severity::Error,
            "accessibility",
            source,
            format!(
                "{} interactive target(s) smaller than 24x24px (smallest {:.0}px) (WCAG 2.5.8).",
                n,
                num("min_touch_target_px").unwrap_or(0.0)
            ),
            "Increase the hit area to at least 24x24px (padding or min-height/min-width).",
        ));
    }
    if flag("heading_hierarchy_valid") == Some(false) {
        findings.push(browser_finding(
            "A11Y-HEADING-HIERARCHY",
            Severity::Warning,
            "accessibility",
            source,
            format!("Heading structure problems: {}", strings("heading_problems").join("; ")),
            "Use one <h1> and do not skip heading levels.",
        ));
    }
    if flag("landmarks_present") == Some(false) {
        findings.push(browser_finding(
            "A11Y-MAIN-LANDMARK",
            Severity::Warning,
            "accessibility",
            source,
            "No <main> landmark found.".into(),
            "Wrap primary content in <main>.",
        ));
    }
    if flag("focus_visible_styled") == Some(false) {
        findings.push(browser_finding(
            "A11Y-FOCUS-VISIBLE-LIVE",
            Severity::Warning,
            "accessibility",
            source,
            "No `:focus-visible` rule found in readable stylesheets; relying on browser default focus rings.".into(),
            "Add a high-contrast `:focus-visible` style.",
        ));
    }
}

pub fn audit_file_content(
    file_label: &str,
    content: &str,
    existing_dna: Option<&SiteDna>,
    findings: &mut Vec<AuditFinding>,
) {
    let lower = content.to_ascii_lowercase();
    let is_html_doc = file_label.ends_with(".html") && lower.contains("<html");

    // 1. HTML document language & viewport checks
    if is_html_doc {
        if !lower.contains("lang=") {
            findings.push(AuditFinding {
                rule_id: "A11Y-HTML-LANG".into(),
                kind: FindingKind::DeterministicObjective,
                severity: Severity::Error,
                category: "accessibility".into(),
                file_or_selector: file_label.into(),
                message: "Root `<html>` element is missing a `lang` attribute (WCAG 3.1.1).".into(),
                remediation: "Add `lang=\"en\"` (or the appropriate BCP 47 language tag) to `<html>`.".into(),
                auto_repairable: true,
            });
        }
        if !lower.contains("name=\"viewport\"") && !lower.contains("name='viewport'") {
            findings.push(AuditFinding {
                rule_id: "RESP-VIEWPORT-META".into(),
                kind: FindingKind::DeterministicObjective,
                severity: Severity::Error,
                category: "responsive".into(),
                file_or_selector: file_label.into(),
                message: "Document `<head>` is missing `<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">`.".into(),
                remediation: "Include the responsive viewport meta tag in `<head>`.".into(),
                auto_repairable: true,
            });
        }
    }

    // 2. Image alt attribute check (WCAG 1.1.1)
    let mut search_idx = 0;
    while let Some(pos) = lower[search_idx..].find("<img ") {
        let start = search_idx + pos;
        if let Some(end_rel) = lower[start..].find('>') {
            let tag = &lower[start..=start + end_rel];
            if !tag.contains("alt=") {
                findings.push(AuditFinding {
                    rule_id: "A11Y-IMG-ALT".into(),
                    kind: FindingKind::DeterministicObjective,
                    severity: Severity::Error,
                    category: "accessibility".into(),
                    file_or_selector: file_label.into(),
                    message: "Found `<img>` element without an `alt` attribute (WCAG 1.1.1).".into(),
                    remediation: "Provide descriptive `alt=\"...\"` text or `alt=\"\"` for decorative images.".into(),
                    auto_repairable: true,
                });
                break;
            }
            search_idx = start + end_rel + 1;
        } else {
            break;
        }
    }

    // 3. Focus ring suppression without focus-visible replacement (WCAG 2.4.7)
    if (lower.contains("outline: none") || lower.contains("outline:none") || lower.contains("outline: 0"))
        && !lower.contains("focus-visible")
    {
        findings.push(AuditFinding {
            rule_id: "A11Y-FOCUS-VISIBLE".into(),
            kind: FindingKind::DeterministicObjective,
            severity: Severity::Error,
            category: "accessibility".into(),
            file_or_selector: file_label.into(),
            message: "Focus outline (`outline: none` / `0`) removed without a `:focus-visible` indicator (WCAG 2.4.7).".into(),
            remediation: "Provide a high-contrast `:focus-visible` outline or ring on interactive elements.".into(),
            auto_repairable: true,
        });
    }

    // 4. Motion safety: animations without prefers-reduced-motion guard
    let has_animation = lower.contains("@keyframes")
        || lower.contains("gsap.")
        || lower.contains("scrolltrigger")
        || lower.contains("requestanimationframe");
    let has_reduced_motion_guard = lower.contains("prefers-reduced-motion")
        || lower.contains("usereducedmotion")
        || lower.contains("matchmedia");
    if has_animation && !has_reduced_motion_guard {
        findings.push(AuditFinding {
            rule_id: "MOTION-REDUCED-MOTION".into(),
            kind: FindingKind::DeterministicObjective,
            severity: Severity::Warning,
            category: "motion".into(),
            file_or_selector: file_label.into(),
            message: "Animations detected without a `prefers-reduced-motion` guard in this file.".into(),
            remediation: "Add `@media (prefers-reduced-motion: reduce)` or `window.matchMedia('(prefers-reduced-motion: reduce)')` fallback.".into(),
            auto_repairable: true,
        });
    }

    // 5. Competing scroll engines
    if lower.contains("lenis") && (lower.contains("locomotivescroll") || lower.contains("scrollsmoother")) {
        findings.push(AuditFinding {
            rule_id: "MOTION-COMPETING-SCROLL".into(),
            kind: FindingKind::DeterministicObjective,
            severity: Severity::Error,
            category: "motion".into(),
            file_or_selector: file_label.into(),
            message: "Multiple competing smooth-scroll engines detected (`Lenis` + `LocomotiveScroll`/`ScrollSmoother`).".into(),
            remediation: "Use native scrolling by default, or a single scroll engine when justified.".into(),
            auto_repairable: false,
        });
    }

    // 6. Context-sensitive Subjective Critique (never blocks existing brand DNA!)
    let brand_uses_purple = existing_dna
        .map(|dna| {
            dna.colors
                .iter()
                .any(|c| c.value.eq_ignore_ascii_case("#8b5cf6") || c.name.contains("purple"))
        })
        .unwrap_or(false);
    if !brand_uses_purple && lower.contains("#8b5cf6") && lower.contains("#ec4899") {
        findings.push(AuditFinding {
            rule_id: "CRITIQUE-GENERIC-GRADIENT".into(),
            kind: FindingKind::SubjectiveCritique,
            severity: Severity::Info,
            category: "design-critique".into(),
            file_or_selector: file_label.into(),
            message: "Subjective design critique: Default AI purple-to-pink gradient (`#8b5cf6` -> `#ec4899`) detected without matching brand DNA.".into(),
            remediation: "Consider using an industry-calibrated palette from `DESIGN.md` unless explicitly requested by the brand.".into(),
            auto_repairable: false,
        });
    }
}

/// Compares before and after audit/visual snapshots to distinguish intended changes from unintended regressions.
pub fn verify_before_after(
    before: &AuditReport,
    after: &AuditReport,
    pixel_diff_ratio: f64,
    intended_descriptions: Vec<String>,
) -> VisualRegressionComparison {
    let mut unintended = Vec::new();
    if after.objective_error_count > before.objective_error_count {
        unintended.push(format!(
            "Objective errors increased from {} to {}.",
            before.objective_error_count, after.objective_error_count
        ));
    }
    // Error-level rules that appear only after the change
    let before_rules: std::collections::HashSet<&str> = before
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .map(|f| f.rule_id.as_str())
        .collect();
    let mut new_rules: Vec<&str> = after
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Error && !before_rules.contains(f.rule_id.as_str()))
        .map(|f| f.rule_id.as_str())
        .collect();
    new_rules.sort_unstable();
    new_rules.dedup();
    if !new_rules.is_empty() {
        unintended.push(format!("New error rule(s) after change: {}.", new_rules.join(", ")));
    }
    if !after.accessibility.passes_automated_wcag22_aa && before.accessibility.passes_automated_wcag22_aa {
        unintended.push("Automated WCAG 2.2 AA accessibility regressed.".to_string());
    }
    let (b, a) = (&before.core_web_vitals, &after.core_web_vitals);
    for (name, was, now, value) in [
        ("LCP", b.lcp_passes_target, a.lcp_passes_target, a.lcp_ms),
        ("INP", b.inp_passes_target, a.inp_passes_target, a.inp_or_tbt_ms),
        ("CLS", b.cls_passes_target, a.cls_passes_target, a.cls),
    ] {
        if was == Some(true) && now == Some(false) {
            unintended.push(format!("{} regressed past its target (now {:.3}).", name, value.unwrap_or(0.0)));
        }
    }

    VisualRegressionComparison {
        baseline_present: true,
        viewports_compared: vec![
            "1440x900 (desktop)".to_string(),
            "768x1024 (tablet)".to_string(),
            "375x812 (mobile)".to_string(),
        ],
        pixel_diff_ratio,
        intended_changes: intended_descriptions,
        unintended_regressions: unintended.clone(),
        accepted: unintended.is_empty() && after.passed_release_gate,
    }
}

/// Automatically repairs bounded, reproducible HTML/CSS accessibility and motion issues while recording a clear summary.
pub fn auto_repair_html_file(file_path: &Path) -> Result<AutoRepairResult, String> {
    let original = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed reading `{}`: {}", file_path.display(), e))?;
    let mut updated = original.clone();
    let mut applied = Vec::new();

    // 1. Add `lang="en"` to `<html>` if missing
    if updated.contains("<html>") {
        updated = updated.replacen("<html>", "<html lang=\"en\">", 1);
        applied.push("Added `lang=\"en\"` to `<html>` root element (WCAG 3.1.1).".to_string());
    }

    // 2. Add responsive viewport meta tag if missing in `<head>`
    let lower = updated.to_ascii_lowercase();
    if lower.contains("<head>") && !lower.contains("name=\"viewport\"") && !lower.contains("name='viewport'") {
        updated = updated.replacen(
            "<head>",
            "<head>\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />",
            1,
        );
        applied.push("Inserted responsive `<meta name=\"viewport\">` tag in `<head>`.".to_string());
    }

    // 3. Inject reduced-motion and focus-visible safeguard block before `</style>` if missing
    let lower_now = updated.to_ascii_lowercase();
    if lower_now.contains("</style>") && !lower_now.contains("prefers-reduced-motion") {
        let safeguard = "\n    :focus-visible { outline: 2px solid currentColor; outline-offset: 3px; }\n    @media (prefers-reduced-motion: reduce) {\n      *, *::before, *::after { animation-duration: 0.01ms !important; animation-iteration-count: 1 !important; transition-duration: 0.01ms !important; scroll-behavior: auto !important; }\n    }\n  </style>";
        updated = updated.replacen("</style>", safeguard, 1);
        applied.push(
            "Injected `:focus-visible` ring and `@media (prefers-reduced-motion: reduce)` safeguard into `<style>`."
                .to_string(),
        );
    }

    let modified = updated != original;
    if modified {
        fs::write(file_path, &updated)
            .map_err(|e| format!("Failed writing repaired file `{}`: {}", file_path.display(), e))?;
    }

    Ok(AutoRepairResult {
        file_path: file_path.to_string_lossy().replace('\\', "/"),
        diff_summary: if modified {
            format!("Applied {} bounded repair(s).", applied.len())
        } else {
            "No bounded repairs needed.".to_string()
        },
        applied_repairs: applied,
        modified,
    })
}

#[cfg(test)]
mod design_system_tests {
    use super::*;
    use crate::qa::{design_tokens_from_design_md, design_tokens_from_site_dna, discover_design_tokens};
    use serde_json::json;

    fn drift_report() -> serde_json::Value {
        let color = json!({
            "rule": "design-system-color", "severity": "warning",
            "detail": "text colour #e11d48 on 2 elements, e.g. <span.price> \"$12\"; nearest token accent = #c2410c (9.8 ΔE00 away)",
            "value": "#e11d48", "nearest_token": "accent", "nearest_value": "#c2410c", "distance": 9.8, "count": 2
        });
        json!({
            "schema_version": "2.0.0",
            "viewports": [
                { "label": "desktop", "issues": [
                    color.clone(),
                    { "rule": "design-system-font", "severity": "warning", "detail": "font-family comic sans ms on 1 element",
                      "value": "comic sans ms", "nearest_token": "body", "nearest_value": "source serif 4" },
                    { "rule": "craft-heading-rhythm", "severity": "critique", "detail": "h2 \"Menu\" has 8px above vs 40px below" }
                ]},
                { "label": "mobile", "issues": [ color ] }
            ]
        })
    }

    #[test]
    fn art_over_text_maps_to_a_named_visual_rule() {
        let (id, category, remediation) = browser_rule("art-over-text");
        assert_eq!((id, category), ("UI-ART-OVER-TEXT", "visual"));
        assert!(remediation.contains("solid plate") && remediation.contains("never through it"));
    }

    #[test]
    fn motion_density_rules_map_to_named_motion_rules() {
        let (id, category, remediation) = browser_rule("motion-too-sparse");
        assert_eq!((id, category), ("UI-MOTION-SPARSE", "motion"));
        assert!(remediation.contains("layoza-motion") && remediation.contains("calm"));
        let (id, category, remediation) = browser_rule("pinned-stage-static");
        assert_eq!((id, category), ("UI-PINNED-STATIC", "motion"));
        assert!(remediation.contains("25%"));
    }

    #[test]
    fn design_system_warnings_are_objective_and_name_the_nearest_token() {
        let findings = browser_report_findings(&drift_report(), "qa/browser-qa-report.json");
        let ds: Vec<&AuditFinding> = findings.iter().filter(|f| f.category == "design-system").collect();
        assert_eq!(ds.len(), 2, "one finding per off-system value, merged across viewports: {:#?}", ds);
        let color = ds.iter().find(|f| f.rule_id == "DESIGN-SYSTEM-COLOR").expect("colour drift");
        assert_eq!(color.kind, FindingKind::DeterministicObjective);
        assert_eq!(color.severity, Severity::Warning);
        assert!(color.file_or_selector.contains("desktop, mobile"), "{}", color.file_or_selector);
        assert!(color.remediation.contains("`accent` token (#c2410c)"), "{}", color.remediation);
        let font = ds.iter().find(|f| f.rule_id == "DESIGN-SYSTEM-FONT").expect("font drift");
        assert!(font.remediation.contains("`body` token"), "{}", font.remediation);
        // Never errors: design-system drift cannot fail the release gate on its own
        assert!(findings.iter().all(|f| f.severity != Severity::Error));
        // heading-rhythm stays a non-blocking craft critique
        let hr = findings.iter().find(|f| f.rule_id == "CRAFT-HEADING-RHYTHM").expect("heading rhythm");
        assert_eq!(hr.kind, FindingKind::SubjectiveCritique);
        assert_eq!(hr.severity, Severity::Info);
        assert!(hr.message.contains("Heading crowded against the previous block"));
    }

    #[test]
    fn interface_checks_and_generic_tells_map_to_named_findings() {
        let report = json!({
            "schema_version": "2.0.0",
            "viewports": [{ "label": "mobile", "issues": [
                { "rule": "zoom-blocked", "severity": "error", "detail": "viewport meta blocks pinch zoom" },
                { "rule": "ios-input-zoom", "severity": "warning", "detail": "1 form field under 16px" },
                { "rule": "craft-arrow-cta", "severity": "critique", "detail": "2 links end in an arrow glyph" }
            ]}]
        });
        let findings = browser_report_findings(&report, "qa/browser-qa-report.json");
        let zoom = findings.iter().find(|f| f.rule_id == "A11Y-ZOOM-BLOCKED").expect("zoom-blocked mapped");
        assert_eq!(zoom.severity, Severity::Error);
        assert!(zoom.remediation.contains("WCAG 1.4.4"));
        let ios = findings.iter().find(|f| f.rule_id == "UI-IOS-INPUT-ZOOM").expect("ios-input-zoom mapped");
        assert_eq!(ios.severity, Severity::Warning);
        let arrow = findings.iter().find(|f| f.rule_id == "CRAFT-ARROW-CTA").expect("arrow tell mapped");
        assert_eq!(arrow.kind, FindingKind::SubjectiveCritique);
        assert!(arrow.message.contains("Arrow appended to action text"), "{}", arrow.message);
    }

    #[test]
    fn design_md_frontmatter_becomes_compact_tokens() {
        let md = r##"---
version: "alpha"
name: "Spoke & Kettle"
colors:
  canvas: "#F7F3EC"
  ink: "#1f1a17"
  accent: "rgb(194, 65, 12)"
  accent-alias: "{colors.accent}"
  overlay: "var(--overlay)"
typography:
  display:
    fontFamily: "Fraunces, Georgia, serif"
    fontSize: "clamp(2.5rem, 6vw, 4.5rem)"
    fontWeight: "600"
  body:
    fontFamily: "Source Serif 4, serif"
    fontSize: "1.125rem"
  caption:
    fontFamily: "Source Serif 4, serif"
    fontSize: "14px"
rounded:
  sm: "4px"
  md: "0.5rem"
  full: "9999px"
  circle: "50%"
spacing:
  md: "16px"
components:
  button-primary:
    backgroundColor: "{colors.ink}"
    rounded: "{rounded.md}"
---

## Overview
"##;
        let t = design_tokens_from_design_md(md).expect("tokens");
        let values = |k: &str| -> Vec<serde_json::Value> {
            t[k].as_array().unwrap().iter().map(|x| x["value"].clone()).collect()
        };
        // Hex normalised, rgb() converted, {colors.*} alias resolved and de-duplicated, var() skipped
        assert_eq!(values("colors"), vec![json!("#f7f3ec"), json!("#1f1a17"), json!("#c2410c")]);
        assert_eq!(t["colors"][2]["name"], "accent");
        assert_eq!(values("fonts"), vec![json!("Fraunces, Georgia, serif"), json!("Source Serif 4, serif")]);
        // rem -> px at a 16px root; percentages skipped
        assert_eq!(values("radii"), vec![json!(4.0), json!(8.0), json!(9999.0)]);
        // clamp() keeps its range; fixed sizes become px
        assert_eq!(t["font_sizes"][0], json!({"name": "display", "min": 40.0, "max": 72.0}));
        assert_eq!(t["font_sizes"][1], json!({"name": "body", "value": 18.0}));
        assert_eq!(t["font_sizes"][2], json!({"name": "caption", "value": 14.0}));
        // No frontmatter, or frontmatter without tokens -> None (report carries design_tokens: null)
        assert!(design_tokens_from_design_md("# Just prose\n").is_none());
        assert!(design_tokens_from_design_md("---\nversion: \"alpha\"\n---\n").is_none());
    }

    #[test]
    fn site_dna_and_discovery_prefer_the_nearest_declared_system() {
        let dna = r##"{"colors":[{"name":"ink","value":"#111"}],"typography":[{"role":"body","font_family":"Inter","font_size":"16px"}],"radii_scale":{"md":"6px"}}"##;
        let t = design_tokens_from_site_dna(dna).expect("dna tokens");
        assert_eq!(t["colors"][0]["value"], "#111111");
        assert_eq!(t["radii"][0]["value"], 6.0);

        let root = std::env::temp_dir().join(format!("layoza-ds-discovery-{}", std::process::id()));
        let site = root.join("site");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(site.join(".layoza")).unwrap();
        let page = site.join("index.html");
        std::fs::write(&page, "<!doctype html><title>x</title>").unwrap();
        // Nothing declared anywhere -> no tokens
        assert!(discover_design_tokens(&root, Some(&page)).is_none());
        // Root-level DNA is found from a nested target
        std::fs::write(root.join("SITE-DNA.json"), dna).unwrap();
        let found = discover_design_tokens(&root, Some(&page)).expect("root dna");
        assert!(found["source"].as_str().unwrap().ends_with("SITE-DNA.json"));
        // A DESIGN.md beside the target wins over the root
        std::fs::write(site.join("DESIGN.md"), "---\ncolors:\n  paper: \"#fafafa\"\n---\n").unwrap();
        let near = discover_design_tokens(&root, Some(&page)).expect("nearest design md");
        assert!(near["source"].as_str().unwrap().ends_with("site/DESIGN.md"), "{}", near["source"]);
        assert_eq!(near["colors"][0]["value"], "#fafafa");
        let _ = std::fs::remove_dir_all(&root);
    }
}
