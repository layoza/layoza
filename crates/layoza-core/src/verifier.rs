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
    /// True only when a fresh browser QA report was ingested and it, plus the static scan, found no
    /// accessibility errors. A static scan alone never passes: it cannot see the rendered page.
    pub passes_automated_wcag22_aa: bool,
    pub manual_review_required: Vec<String>,
    /// What the verdict rests on: "static checks only" or "browser report (<path>) + static checks".
    #[serde(default)]
    pub evidence_basis: String,
    /// Checks a static source scan cannot make; empty when a browser report was ingested.
    #[serde(default)]
    pub needs_browser: Vec<String>,
}

/// Checks that need a rendered page. A static audit lists them as not checked instead of passing them.
pub const NEEDS_BROWSER: &[&str] = &[
    "Text contrast against the real backdrop (composited alpha, images, gradients) at computed font sizes",
    "Target size (WCAG 2.5.8), overlap, occlusion and sideways overflow from 320px to 1920px",
    "Focus visibility, focus order and keyboard reach of every control",
    "Motion under prefers-reduced-motion and text left hidden by animations",
    "Legibility floors after the cascade (body text under 14px, labels under 12px)",
    "Look fingerprint and re-skin detection",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualRegressionComparison {
    pub baseline_present: bool,
    /// Viewports whose screenshots were diffed. Empty: Layoza compares audit findings and Core
    /// Web Vitals only and does not diff screenshots.
    pub viewports_compared: Vec<String>,
    /// Pixel difference supplied by the caller, if any. Layoza does not measure this itself.
    pub pixel_diff_ratio: Option<f64>,
    /// Plain statement of what this comparison is based on.
    #[serde(default)]
    pub comparison_basis: String,
    pub intended_changes: Vec<String>,
    pub unintended_regressions: Vec<String>,
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub target_path: String,
    /// True only when there are no objective errors and fresh browser evidence was ingested.
    pub passed_release_gate: bool,
    /// "passed", "failed", or "unverified" (no objective errors, but no fresh browser evidence).
    #[serde(default)]
    pub verification_status: String,
    /// What the gate covers. It measures defects; it never judges whether the design is good.
    #[serde(default)]
    pub gate_scope: String,
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

/// Options for [`audit_project_with`].
#[derive(Debug, Clone, Default)]
pub struct AuditOptions {
    /// Strict release gate: gate-file warnings (a new site with no recorded user approval of its
    /// direction) become errors.
    pub strict: bool,
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
    audit_project_with(project_root, existing_dna, lab_lcp_ms, lab_inp_ms, lab_cls, &AuditOptions::default())
}

/// [`audit_project`] with options (`strict`).
pub fn audit_project_with(
    project_root: &Path,
    existing_dna: Option<&SiteDna>,
    lab_lcp_ms: Option<f64>,
    lab_inp_ms: Option<f64>,
    lab_cls: Option<f64>,
    opts: &AuditOptions,
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
        // Read bytes and decode leniently: one stray non-UTF-8 byte must not hide a whole file
        let Ok(bytes) = fs::read(file_path) else {
            continue;
        };
        let content = String::from_utf8_lossy(&bytes);
        let content = content.trim_start_matches('\u{feff}');
        let label = file_path
            .strip_prefix(project_root)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/");
        audit_file_content(&label, content, existing_dna, &mut findings);
    }

    // 7. Ingest the live Chromium QA report written by `scripts/browser-qa.mjs`, if present
    let base_dir = if project_root.is_file() {
        project_root.parent().unwrap_or(project_root)
    } else {
        project_root
    };
    let mut browser_cwv: (Option<f64>, Option<f64>, Option<f64>) = (None, None, None);
    let mut browser_cwv_source = String::new();
    // Passes the ingested browser report says it skipped (fast mode), and its mode
    let mut browser_skipped: Vec<String> = Vec::new();
    let mut browser_mode = String::new();
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
        ingest_section_reskins(&val, candidate_rel, &mut findings);
        browser_skipped = val
            .get("skipped_passes")
            .and_then(|s| s.as_array())
            .into_iter()
            .flatten()
            .filter_map(|s| s.as_str().map(|s| format!("Skipped by the browser report: {}", s)))
            .collect();
        browser_mode = val.get("mode").and_then(|m| m.as_str()).unwrap_or("").to_string();
        browser_cwv = (
            val.get("lcp_ms").and_then(|n| n.as_f64()),
            val.get("inp_ms").and_then(|n| n.as_f64()),
            val.get("cls").and_then(|n| n.as_f64()),
        );
        browser_cwv_source = candidate_rel.to_string();
        break;
    }

    audit_design_process(base_dir, &mut findings);
    audit_direction_approval(base_dir, opts.strict, &mut findings);

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
    let browser_verified = !browser_cwv_source.is_empty();
    if !browser_verified {
        findings.push(browser_finding(
            "BROWSER-QA-NOT-MEASURED",
            Severity::Warning,
            "browser-qa",
            ".",
            "No fresh visual browser QA report was ingested, so the design gate is unverified.".into(),
            "Run `layoza qa <file-or-url>` for visual checks. Speed measurements are optional.",
        ));
    }

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

    let failed = objective_error_count > 0;
    let passed_release_gate = !failed && browser_verified;
    if passed_release_gate {
        if let Some(ledger) = crate::history::ledger_path() {
            record_verified_direction(project_root, &ledger, &mut findings);
        }
    }
    let verification_status = if failed {
        "failed"
    } else if browser_verified {
        "passed"
    } else {
        "unverified"
    }
    .to_string();

    Ok(AuditReport {
        target_path: project_root.to_string_lossy().replace('\\', "/"),
        passed_release_gate,
        verification_status,
        gate_scope: GATE_SCOPE.to_string(),
        objective_error_count,
        objective_warning_count,
        subjective_critique_notes,
        findings,
        accessibility: AccessibilityReport {
            target_standard: "WCAG 2.2 Level AA".to_string(),
            automated_violations: a11y_violations,
            passes_automated_wcag22_aa: a11y_violations == 0 && !browser_cwv_source.is_empty(),
            evidence_basis: if browser_cwv_source.is_empty() {
                "static checks only: no fresh browser QA report, so the automated WCAG 2.2 AA result is not a pass".to_string()
            } else {
                let mode = if browser_mode.is_empty() { String::new() } else { format!(", {} mode", browser_mode) };
                format!("browser report ({}{}) + static checks", browser_cwv_source, mode)
            },
            needs_browser: if browser_cwv_source.is_empty() {
                NEEDS_BROWSER.iter().map(|s| s.to_string()).collect()
            } else {
                browser_skipped
            },
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
            // .blade.php templates are covered by "php"
            if matches!(
                ext,
                "html" | "htm" | "css" | "tsx" | "jsx" | "astro" | "vue" | "svelte" | "php" | "liquid"
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

/// Plain statement of what a passing gate means, carried in every report.
pub const GATE_SCOPE: &str = "Design and usability defects: accessibility, layout and the recorded design process, backed by fresh browser evidence. Speed is optional and never blocks this gate. Design quality is judged on the screenshots and by the user.";

/// Minimum logged screenshot reviews before a Mode A/D build can pass the gate.
pub const MIN_ART_LOOP_ROUNDS: usize = 1;

/// Checks the design process recorded in `.layoza/direction.md` (written in Mode A/D).
///
/// Defect checks cannot judge taste, so a page could pass the gate while the
/// direction pick and the art loop were silently skipped. When a direction file
/// exists, the gate also requires:
/// - a `PURPOSE:` line: what the site is for, its audience, new or existing;
/// - a `PICKED:` line naming the direction the user chose (or `PICKED: unattended`
///   with a reason, for runs with nobody to ask). An existing-site (Mode B/C) flow
///   that keeps this file records `PICKED: existing — <design followed>`;
///   it never offers new directions;
/// - a `SIGNATURE:` line naming the page's one subject-driven set piece;
/// - at least [`MIN_ART_LOOP_ROUNDS`] `ROUND <n>:` line from the screenshot review.
///
/// Projects without a direction file (usually Mode B/C) are not affected.
///
/// For a new direction it also checks that the record names the trade's objects
/// ([`audit_direction_subject`]) and, with the user-level history ledger, warns when
/// the pick closely repeats a recent project's; a verified pick is recorded
/// ([`audit_direction_history`]).
fn audit_design_process(base_dir: &Path, findings: &mut Vec<AuditFinding>) {
    audit_design_process_with(base_dir, crate::history::ledger_path().as_deref(), findings);
}

/// Gate file for a new-site build: the user's approval of the direction is recorded in
/// `.layoza/direction-approved.md`. A new site (`.layoza/direction.md` exists and neither its
/// PICKED nor its PURPOSE line says `existing`) without that file gets a WARN, or a FAIL under
/// `strict`. Without `.layoza/direction.md` the build kind is unknown and nothing is reported here
/// (the design-process checks cover that file).
fn audit_direction_approval(base_dir: &Path, strict: bool, findings: &mut Vec<AuditFinding>) {
    let Ok(raw) = fs::read_to_string(base_dir.join(".layoza/direction.md")) else {
        return;
    };
    let upper = raw.strip_prefix('\u{feff}').unwrap_or(&raw).to_ascii_uppercase();
    let line_says_existing = |key: &str| {
        upper.lines().any(|l| {
            let l = l.replace('*', "");
            let l = l.trim().trim_start_matches(['-', '#', ' ']);
            l.strip_prefix(key).is_some_and(|v| {
                v.split(|c: char| !c.is_ascii_alphanumeric()).any(|w| w == "EXISTING")
                    && !v.split(|c: char| !c.is_ascii_alphanumeric()).any(|w| w == "NEW")
            })
        })
    };
    if line_says_existing("PICKED:") || line_says_existing("PURPOSE:") {
        return;
    }
    // An automatic pick decided from the niche (`PICKED: auto — <reason>`) or an unattended run
    // records its own reason; it needs no separate approval file.
    let picked_by_rule = upper.lines().any(|l| {
        let l = l.replace('*', "");
        let l = l.trim().trim_start_matches(['-', '#', ' ']);
        l.strip_prefix("PICKED:").is_some_and(|v| {
            let mut words = v.split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty());
            matches!(words.next(), Some("AUTO" | "UNATTENDED")) && words.next().is_some()
        })
    });
    if picked_by_rule {
        return;
    }
    let rel = ".layoza/direction-approved.md";
    let approved = fs::read_to_string(base_dir.join(rel)).is_ok_and(|t| !t.trim_start_matches('\u{feff}').trim().is_empty());
    if approved {
        return;
    }
    findings.push(browser_finding(
        "DESIGN-DIRECTION-NOT-APPROVED",
        if strict { Severity::Error } else { Severity::Warning },
        "design-process",
        rel,
        format!(
            "This is a new-site build and `{}` is missing or empty, so there is no record that the user approved the direction before the build.{}",
            rel,
            if strict { " The strict gate fails on this." } else { " Under --strict this fails the gate." }
        ),
        "After the user picks a direction, write .layoza/direction-approved.md with the direction, who approved it and when.",
    ));
}

/// Section re-skins from the look ledger (`look_ledger.section_matches` in the browser report): a
/// nav, hero, section or footer whose structure matches a same-role section of another target.
fn ingest_section_reskins(val: &serde_json::Value, source: &str, findings: &mut Vec<AuditFinding>) {
    let matches = val.pointer("/look_ledger/section_matches").and_then(|m| m.as_array());
    for m in matches.into_iter().flatten().take(12) {
        let s = |k: &str| m.get(k).map(|v| v.as_str().map(String::from).unwrap_or_else(|| v.to_string())).unwrap_or_default();
        let sim = m.get("similarity").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let palette = m
            .get("palette_distance")
            .and_then(|x| x.as_f64())
            .map(|d| format!(", palette distance {:.3}", d))
            .unwrap_or_default();
        let same_type = m.get("same_type").and_then(|x| x.as_bool()) == Some(true);
        let ty = if same_type { ", same type" } else { ", different type" };
        let copy = same_type && m.get("palette_distance").and_then(|x| x.as_f64()).is_some_and(|d| d < 0.03);
        let verdict = if copy { "a copy of that section" } else { "the same section with new paint" };
        findings.push(browser_finding(
            "QA-SECTION-RESKIN",
            Severity::Warning,
            "design-quality",
            &format!("{} ({} #{})", source, s("role"), s("section_index")),
            format!(
                "The {} \"{}\" has the structure of the {} \"{}\" on {} (similarity {:.3}{}{}): {}.",
                s("role"), s("section_label"), s("role"), s("matched_label"), s("matched_target"), sim, palette, ty, verdict
            ),
            "Recompose this section for this site (column count, media placement, reading order), not only its colours and type.",
        ));
    }
}

fn audit_design_process_with(base_dir: &Path, ledger: Option<&Path>, findings: &mut Vec<AuditFinding>) {
    let rel = ".layoza/direction.md";
    let Ok(raw) = fs::read_to_string(base_dir.join(rel)) else {
        return;
    };
    // Windows editors and PowerShell 5.1 `Set-Content -Encoding utf8` prepend a BOM, which would hide the first line.
    let text = raw.strip_prefix('\u{feff}').unwrap_or(&raw).to_string();
    let lines: Vec<String> = text
        .lines()
        .map(|l| {
            l.replace('*', "")
                .trim()
                .trim_start_matches(['-', '#', ' '])
                .to_ascii_uppercase()
        })
        .collect();
    let purpose = lines
        .iter()
        .find_map(|l| l.strip_prefix("PURPOSE:").map(|v| v.trim().to_string()));
    if purpose.as_deref().unwrap_or("").is_empty() {
        findings.push(browser_finding(
            "DESIGN-NO-PURPOSE",
            Severity::Error,
            "design-process",
            rel,
            "No purpose is recorded, so the design was started without knowing what the site is for. Record the purpose and primary action, the audience, whether the site is new or extends an existing one, and what must be kept; ask the user for anything the prompt leaves open.".into(),
            "Add `PURPOSE: <goal and primary action>; audience: <who>; new|existing; keep: <must-keep>` to .layoza/direction.md.",
        ));
    }
    let pick = lines
        .iter()
        .find_map(|l| l.strip_prefix("PICKED:").map(|v| v.trim().to_string()));
    match pick.as_deref() {
        None | Some("") => findings.push(browser_finding(
            "DESIGN-NO-DIRECTION-PICK",
            Severity::Error,
            "design-process",
            rel,
            "No direction pick is recorded, so the look was chosen without the user. Show the user three distinct directions (a rendered hero sketch screenshot each, not ASCII) and wait for their choice before building.".into(),
            "Add `PICKED: <direction> (chosen by the user)` to .layoza/direction.md, `PICKED: auto — <direction>, <why it fits the niche>`, `PICKED: unattended — <why nobody could be asked>`, or for an existing site `PICKED: existing — <design followed>`.",
        )),
        _ => {}
    }
    let rounds = lines
        .iter()
        .filter(|l| {
            l.strip_prefix("ROUND ")
                .and_then(|r| r.split(':').next())
                .is_some_and(|n| {
                    !n.trim().is_empty() && n.trim().chars().all(|c| c.is_ascii_digit())
                })
        })
        .count();
    let current_visual_review = crate::creative::has_visual_selection(base_dir);
    if rounds < MIN_ART_LOOP_ROUNDS && !current_visual_review {
        findings.push(browser_finding(
            "DESIGN-NOT-REVIEWED",
            Severity::Error,
            "design-process",
            rel,
            "No screenshot review is logged. Passing defect checks does not mean the design is good.".into(),
            "Inspect actual desktop and phone compositions, fix visible problems, and persist candidate paths, strengths, weaknesses and changes with review_design. Legacy ROUND notes remain accepted; the default review budget is three rounds, not a taste certificate.",
        ));
    }
    // An existing site follows its own design: no signature, subject or repeat checks.
    if pick.as_deref().is_some_and(|p| p.starts_with("EXISTING")) {
        return;
    }
    let signature = lines.iter().find_map(|l| l.strip_prefix("SIGNATURE:").map(|v| v.trim().to_string()));
    if signature.as_deref().unwrap_or("").is_empty() && !current_visual_review {
        findings.push(browser_finding(
            "DESIGN-NO-SIGNATURE",
            Severity::Info,
            "design-critique",
            rel,
            "No distinctive composition decision or current visual review is recorded; judge the actual page before suggesting extra effects.".into(),
            "Record the chosen composition and visible tradeoff with review_design. A signature effect is optional; familiar patterns can serve the task.",
        ));
    }
    // One motion scale throughout engine, skills and review: 0 none .. 10 immersive.
    let motion = lines.iter().find_map(|l| l.strip_prefix("MOTION:").map(|v| v.trim().to_string()));
    if motion.as_deref().unwrap_or("").split(|c: char| !c.is_ascii_digit()).next().and_then(|s| s.parse::<u8>().ok()).is_none_or(|n| n > 10) {
        findings.push(browser_finding(
            "DESIGN-NO-MOTION-LEVEL",
            Severity::Warning,
            "design-process",
            rel,
            "No motion level is decided, so the page falls back to a default amount of motion that may not fit the niche (an agency sells its motion; a clinic needs calm).".into(),
            "Add `MOTION: <0-10> — <why this audience and task need it>`. Zero means no animation.",
        ));
    }
    let blocks = crate::history::parse_direction(&text);
    audit_direction_subject(&blocks, rel, findings);
    if let Some(ledger) = ledger {
        audit_direction_history(base_dir, &text, &blocks, ledger, rel, findings);
    }
}

/// Adjectives that describe a style rather than the subject ("modern, clean, bold").
const STYLE_WORDS: &[&str] = &[
    "modern", "clean", "bold", "minimal", "minimalist", "minimalistic", "sleek", "elegant", "premium", "luxury",
    "luxurious", "luxe", "simple", "fresh", "vibrant", "playful", "professional", "trustworthy", "friendly",
    "warm", "cool", "dark", "light", "bright", "calm", "soft", "sharp", "striking", "stunning", "beautiful",
    "creative", "unique", "innovative", "dynamic", "contemporary", "timeless", "classic", "sophisticated",
    "refined", "polished", "crisp", "airy", "spacious", "subtle", "rich", "edgy", "futuristic", "organic",
    "natural", "earthy", "vintage", "retro", "colorful", "colourful", "vivid", "punchy", "confident",
    "approachable", "inviting", "engaging", "immersive", "cinematic", "expressive", "quiet", "loud", "high-end",
    "upscale", "chic", "stylish", "trendy", "techy", "corporate", "whimsical", "nostalgic", "moody", "energetic",
    "serene", "delightful", "amazing", "great", "nice", "good", "best", "impactful", "memorable", "distinctive",
    "standout", "fun", "cozy", "cosy", "flat", "geometric", "abstract", "gradient", "glassy",
];

/// Words that are neither the subject nor a style: design vocabulary, process labels,
/// colour names, common verbs and English stop words.
const NEUTRAL_WORDS: &[&str] = &[
    "design", "style", "look", "looks", "feel", "feels", "aesthetic", "vibe", "palette", "colour", "color",
    "colors", "colours", "font", "fonts", "type", "typography", "serif", "sans", "mono", "accent", "hero",
    "section", "sections", "page", "pages", "website", "site", "landing", "homepage", "cta", "button", "buttons",
    "grid", "card", "cards", "motion", "animation", "animations", "layout", "whitespace", "space", "spacing",
    "visual", "visuals", "image", "images", "photo", "photos", "imagery", "text", "copy", "headline", "headlines",
    "brand", "branding", "identity", "theme", "interface", "experience", "direction", "directions", "option",
    "version", "purpose", "audience", "new", "existing", "keep", "must", "user", "users", "visitor", "visitors",
    "people", "customers", "clients", "chosen", "picked", "pick", "unattended", "primary", "action", "goal",
    "business", "company", "product", "products", "service", "services", "online", "web", "mobile", "desktop",
    "first", "viewport", "screen", "scroll", "thesis", "world", "own", "tokens", "black", "white", "grey", "gray",
    "red", "blue", "green", "yellow", "orange", "purple", "pink", "navy", "cream", "beige", "gold", "silver",
    "teal", "ink", "the", "and", "for", "with", "that", "this", "from", "into", "our", "their", "your", "its",
    "are", "was", "were", "will", "can", "has", "have", "not", "but", "all", "any", "each", "more", "most",
    "very", "just", "only", "also", "than", "then", "when", "where", "who", "what", "how", "why", "which",
    "while", "about", "over", "under", "out", "off", "one", "two", "three", "via", "per", "like", "becomes",
    "become", "made", "make", "makes", "use", "used", "using", "show", "shows", "sell", "sells", "selling", "buy",
    "book", "get", "see", "help", "find", "learn", "sign", "join", "contact", "start", "try", "drive",
    "increase", "grow", "convert", "follow", "follows", "following", "improve", "add", "run", "runs", "build",
    "built", "create", "give", "gives", "let", "lets", "want", "need", "needs", "should", "would", "could",
    "every", "other", "some", "many", "much", "really", "choice", "lots", "big", "large", "small", "full",
    "high", "low", "top", "side", "left", "right", "center", "centre", "main", "plus",
];

/// Subject-bearing words in `text`: alphabetic words of 3+ letters that are neither
/// style nor neutral words. Returns (distinct concrete words, style-word count).
fn subject_words(text: &str) -> (Vec<String>, usize) {
    let mut concrete: Vec<String> = Vec::new();
    let mut style = 0;
    for w in text.split(|c: char| !(c.is_alphabetic() || c == '-')) {
        let w = w.trim_matches('-').to_lowercase();
        if w.chars().count() < 3 {
            continue;
        }
        if STYLE_WORDS.contains(&w.as_str()) {
            style += 1;
        } else if !NEUTRAL_WORDS.contains(&w.as_str()) && !concrete.contains(&w) {
            concrete.push(w);
        }
    }
    (concrete, style)
}

/// Subject-first check: the purpose and the picked direction must name the trade's own
/// objects (catch board, harbour chart, lens), not only style words. A record made of
/// nothing but style words is an error; a thin or style-only direction is a warning.
fn audit_direction_subject(
    blocks: &std::collections::BTreeMap<String, String>,
    rel: &str,
    findings: &mut Vec<AuditFinding>,
) {
    let join = |keys: &[&str]| keys.iter().filter_map(|k| blocks.get(*k)).cloned().collect::<Vec<_>>().join(" ");
    let direction_keys = ["PICKED", "SIGNATURE", "THESIS", "OWN-WORLD", "FIRST VIEWPORT", "SECTIONS", "ART PLAN"];
    let (all_nouns, all_style) = subject_words(&format!("{} {}", join(&["PURPOSE"]), join(&direction_keys)));
    let (dir_nouns, dir_style) = subject_words(&join(&direction_keys));
    let remediation = "Name the trade's concrete objects in PURPOSE and SIGNATURE: the dish, tool, room, garment or document a stranger would recognise, e.g. `THESIS: the daily catch board becomes a tide chart`. Style words (modern, clean, bold) do not count.";
    if all_nouns.is_empty() {
        let generic = all_style >= 2;
        findings.push(browser_finding(
            if generic { "DESIGN-GENERIC-DIRECTION" } else { "DESIGN-NO-SUBJECT" },
            if generic { Severity::Error } else { Severity::Warning },
            "design-process",
            rel,
            if generic {
                "The purpose and direction use only style words and name nothing from the trade, so the design cannot be about its subject.".into()
            } else {
                "The purpose and direction name no concrete object from the trade, so nothing anchors the design to its subject.".into()
            },
            remediation,
        ));
    } else if dir_nouns.is_empty() && dir_style >= 1 {
        findings.push(browser_finding(
            "DESIGN-GENERIC-DIRECTION",
            Severity::Warning,
            "design-process",
            rel,
            "The picked direction is described only in style words; it names none of the trade's objects, so a neighbouring business could use it unchanged.".into(),
            remediation,
        ));
    }
}

/// Anti-repeat memory: warns when this pick closely repeats one of the last
/// [`crate::history::RECENT_WINDOW`] picks from other projects (palette, type families,
/// motifs, mechanics). A pick is recorded by [`record_verified_direction`] once the release gate passes. `REPEAT-OK: <the user's
/// words>` in direction.md marks a repeat the user asked for.
fn audit_direction_history(
    base_dir: &Path,
    text: &str,
    blocks: &std::collections::BTreeMap<String, String>,
    ledger: &Path,
    rel: &str,
    findings: &mut Vec<AuditFinding>,
) {
    use crate::history;
    let Some(entry) = history::entry_from_direction(base_dir, text) else {
        return;
    };
    if !blocks.contains_key("REPEAT-OK") {
        let past = history::recent(ledger, history::RECENT_WINDOW, Some(&entry.project));
        if let Some((p, signals)) = past
            .iter()
            .rev()
            .map(|p| (p, history::repeat_signals(&entry, p)))
            .find(|(_, s)| !s.is_empty())
        {
            findings.push(browser_finding(
                "DESIGN-REPEATS-RECENT",
                Severity::Warning,
                "design-process",
                rel,
                format!(
                    "The picked direction closely repeats `{}` from another project ({}): same {}. Designs drift toward the same look across projects.",
                    p.direction,
                    p.date,
                    signals.join("; ")
                ),
                "Change the palette family, type families and motifs so the direction is materially different (see `avoid_recent` in `get_design_system`). If the user asked for this same look, add `REPEAT-OK: <the user's words>` to .layoza/direction.md.",
            ));
        }
    }
}

/// Adds a new-direction pick to the design history ledger. Called only once the
/// release gate passes, so the ledger holds verified designs, not every draft an
/// audit sees. Existing-site flows are never recorded (see `entry_from_direction`).
fn record_verified_direction(base_dir: &Path, ledger: &Path, findings: &mut Vec<AuditFinding>) {
    use crate::history;
    let rel = ".layoza/direction.md";
    let Ok(raw) = fs::read_to_string(base_dir.join(rel)) else {
        return;
    };
    let text = raw.strip_prefix('\u{feff}').unwrap_or(&raw);
    let Some(entry) = history::entry_from_direction(base_dir, text) else {
        return;
    };
    if let Err(e) = history::record(ledger, &entry) {
        findings.push(browser_finding(
            "DESIGN-HISTORY-UNWRITABLE",
            Severity::Info,
            "design-process",
            rel,
            format!("The direction pick could not be added to the design history ledger: {}.", e),
            "Set LAYOZA_HOME to a writable directory, or LAYOZA_NO_HISTORY=1 to turn the ledger off.",
        ));
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
        "ledger-heavy-page" => ("UI-LEDGER-HEAVY","visual","Most sections are stacked rows, lists or tables with no dominant visual, so the page reads as a spreadsheet: keep data to the one or two sections where the visitor needs it (price, specs, plans) and give every other section one visual moment (authored art, 3D, a scroll scene, full-bleed colour or type as image)."),
        "visual-starved-page" => ("UI-VISUAL-STARVED","visual","Most sections carry no visual moment, so the page reads as a document of text and boxes: give each section one dominant visual drawn from its own content (authored art, photography, 3D, a scroll scene or type set as image) covering at least a quarter of it."),
        "zigzag-stack" => ("UI-ZIGZAG-STACK","visual","Rows of media beside text flip sides one after another, the stock template rhythm: give the items one shared composition (a set seen together, a scene, a comparison or one stage that changes) instead of alternating rows."),
        "wireframe-page" => ("UI-WIREFRAME-PAGE","visual","Most sections are built from 1px-ruled boxes or rows on a flat ground, so the page reads as a wireframe: separate surfaces by tone, scale and space (a tonal field, an inset plate, open ground) and keep rules for real tables."),
        "no-frame-break" => ("UI-NO-FRAME-BREAK","visual","No section breaks its frame, so the page stays flat: set type over a picture, run a picture full-bleed, or let an object cross a section edge in at least two sections."),
        "signature-missing" => ("UI-NO-SIGNATURE","motion","The page has no set piece: build one scroll- or hand-driven scene from the subject (what it does or reveals, staged across one to two screens, final frame shown without JavaScript), or declare <meta name=\"layoza-motion\" content=\"calm\"> for a still page."),
        "repeated-card-look" => ("UI-REPEATED-CARD","visual","One card treatment (same border, fill, radius and shadow) repeats across most sections, so the page reads as a template of boxes: give each section its own material (open ground, full-bleed colour field, inset plate, art) and keep cards only where content is truly a set of peers."),
        "menu-escape-broken" => ("UI-MENU-ESCAPE","accessibility","Escape does not close the open mobile menu when focus is on its toggle: handle Escape on the document while the menu is open, close it and return focus to the toggle."),
        "menu-close-slow" => ("UI-MENU-CLOSE-SLOW","motion","The mobile menu takes longer than about 800ms to leave the screen after Escape: shorten its exit animation to about 200-300ms so closing feels instant."),
        "art-over-text" => ("UI-ART-OVER-TEXT","visual","Canvas, WebGL or video art is painted across a headline, CTA or body text at a resting scroll position: give the text its own zone or a solid plate, and keep the art beside or behind it, never through it."),
        "text-over-graphic-contrast" => ("UI-TEXT-OVER-ART-CONTRAST","accessibility","Text sitting on an image, canvas or gradient falls below 4.5:1 against the pixels behind it: put the text on a solid or scrimmed plate, move it off the art, or darken/lighten the art under it."),
        "text-over-busy-graphic" => ("UI-TEXT-OVER-BUSY-ART","visual","Text runs across a busy, high-detail part of an image or canvas, so letters fight the picture: give the text a quiet zone or a plate, or crop the art so the busy area sits beside the text."),
        "text-occluded" => ("UI-TEXT-OCCLUDED","visual","Another element is painted on top of visible text at a resting scroll position: fix the stacking or layout so the text is never covered."),
        "motion-too-sparse" => ("UI-MOTION-SPARSE","motion","Fewer distinct elements move, transition or follow scroll than the page's declared motion level expects: build one scroll- or hand-driven signature set piece from the subject (then two motion ideas are enough), or declare <meta name=\"layoza-motion\" content=\"calm\"> (or `none`) if a quiet page is intended."),
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
        let mut grouped: Vec<(String, String, Vec<String>, &serde_json::Value)> = Vec::new();
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
            match grouped.iter_mut().find(|(r, s, _, _)| *r == rule && *s == severity) {
                Some((_, _, details, _)) => details.push(detail),
                None => grouped.push((rule, severity, vec![detail], issue)),
            }
        }
        for (rule, severity, details, first) in grouped {
            let more = if details.len() > 1 { format!(" (+{} more)", details.len() - 1) } else { String::new() };
            // Ported impeccable design critique: advisory only, never part of the release gate
            if severity == "critique" {
                findings.push(craft_finding(&rule, &location, vp_label, &details[0], &more));
                continue;
            }
            let (rule_id, category, remediation) = browser_rule(&rule);
            let sev = if severity == "error" { Severity::Error } else { Severity::Warning };
            // Quality-sweep issues carry their own why / fix / measured value: keep them, under QA-<RULE>
            let text = |k: &str| first.get(k).and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
            if rule_id == "BROWSER-ISSUE" && !text("fix").is_empty() {
                let category = if text("category").is_empty() { "browser-qa".to_string() } else { text("category") };
                let selector = text("selector");
                findings.push(AuditFinding {
                    rule_id: format!("QA-{}", rule.to_ascii_uppercase()),
                    kind: FindingKind::DeterministicObjective,
                    severity: sev,
                    category,
                    file_or_selector: if selector.is_empty() { location.clone() } else { format!("{} {}", location, selector) },
                    message: format!("{} at {}: measured {}. {}{}", rule, vp_label, text("measured"), text("why"), more),
                    remediation: text("fix"),
                    auto_repairable: false,
                });
                continue;
            }
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

    let unmeasurable: Vec<String> = val
        .get("contrast_unmeasurable_reasons")
        .and_then(|r| r.as_object())
        .into_iter()
        .flatten()
        .map(|(k, n)| format!("{} x{}", k, n.as_f64().unwrap_or(0.0)))
        .collect();
    if !unmeasurable.is_empty() {
        findings.push(browser_finding(
            "A11Y-CONTRAST-UNMEASURABLE",
            Severity::Info,
            "accessibility",
            source,
            format!("contrast unmeasurable (image/gradient) for some text, so no ratio is claimed: {}", unmeasurable.join(", ")),
            "Check these by eye or give the text a solid backing (a scrim or a solid panel) so contrast can be measured.",
        ));
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
        // Real GSAP calls only: a `<script src=".../gsap.min.js">` tag loads, it does not animate.
        || ["gsap.to(", "gsap.from(", "gsap.fromto(", "gsap.timeline(", "gsap.set("]
            .iter()
            .any(|call| lower.contains(call))
        || lower.contains("scrolltrigger.create")
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

    // 5b. Static subset of the browser QA rules: markup in every template language, plus CSS
    audit_static_subset(file_label, content, findings);

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

// ---------------------------------------------------------------------------------------------
// Static subset: the browser QA rules that source text can decide on its own
// ---------------------------------------------------------------------------------------------

fn is_markup_file(label: &str) -> bool {
    let l = label.to_ascii_lowercase();
    [".html", ".htm", ".vue", ".svelte", ".astro", ".jsx", ".tsx", ".php", ".liquid"]
        .iter()
        .any(|e| l.ends_with(e))
}

/// Opening tags (`<name ...>`) in lowercased markup.
fn markup_tags(lower: &str) -> Vec<&str> {
    let b = lower.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < b.len() {
        if b[i] == b'<' && b[i + 1].is_ascii_alphabetic() {
            match lower[i..].find('>') {
                Some(e) => {
                    out.push(&lower[i..=i + e]);
                    i += e + 1;
                    continue;
                }
                None => break,
            }
        }
        i += 1;
    }
    out
}

fn tag_name(tag: &str) -> &str {
    let t = &tag[1..];
    let end = t.find(|c: char| !(c.is_ascii_alphanumeric() || c == '-')).unwrap_or(t.len());
    &t[..end]
}

/// Value of attribute `name` in a lowercased tag: `Some("")` for a bare boolean attribute. Handles
/// quoted values, JSX `{...}` values and Vue/Svelte `:name` bindings.
fn attr_value<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let mut i = 0;
    while let Some(p) = tag[i..].find(name) {
        let s = i + p;
        i = s + name.len();
        let before_ok = tag[..s].chars().last().is_some_and(|c| c.is_whitespace() || c == ':');
        let rest = &tag[s + name.len()..];
        let after_ok = rest.chars().next().is_none_or(|c| c.is_whitespace() || c == '=' || c == '>' || c == '/');
        if !before_ok || !after_ok {
            continue;
        }
        let Some(v) = rest.trim_start().strip_prefix('=') else { return Some("") };
        let v = v.trim_start();
        let close = match v.chars().next()? {
            '"' => '"',
            '\'' => '\'',
            '{' => '}',
            _ => {
                let end = v.find(|c: char| c.is_whitespace() || c == '>').unwrap_or(v.len());
                return Some(&v[..end]);
            }
        };
        let body = &v[1..];
        return body.find(close).map(|end| body[..end].trim_matches(|c| c == '"' || c == '\''));
    }
    None
}

fn static_finding(rule: &str, severity: Severity, category: &str, file: &str, message: String, fix: &str) -> AuditFinding {
    AuditFinding {
        rule_id: rule.into(),
        kind: FindingKind::DeterministicObjective,
        severity,
        category: category.into(),
        file_or_selector: file.into(),
        message,
        remediation: fix.into(),
        auto_repairable: false,
    }
}

fn audit_static_subset(file_label: &str, content: &str, findings: &mut Vec<AuditFinding>) {
    let lower = content.to_ascii_lowercase();
    let is_css = file_label.to_ascii_lowercase().ends_with(".css");
    if is_markup_file(file_label) {
        audit_static_markup(file_label, &lower, findings);
    }
    let css = if is_css { lower.clone() } else { style_blocks(&lower) };
    if !css.trim().is_empty() {
        audit_static_css(file_label, &css, findings);
    }
}

/// Text inside `<style>` blocks (any template language).
fn style_blocks(lower: &str) -> String {
    let mut out = String::new();
    let mut i = 0;
    while let Some(p) = lower[i..].find("<style") {
        let s = i + p;
        let Some(open_end) = lower[s..].find('>') else { break };
        let body_start = s + open_end + 1;
        let Some(close) = lower[body_start..].find("</style") else { break };
        out.push_str(&lower[body_start..body_start + close]);
        out.push('\n');
        i = body_start + close;
    }
    out
}

fn audit_static_markup(file: &str, lower: &str, findings: &mut Vec<AuditFinding>) {
    let is_plain_html = file.ends_with(".html") || file.ends_with(".htm");
    let tags = markup_tags(lower);

    // Placeholder copy and stock names: say "verify", never "fake"
    let text_only: String = {
        let mut s = String::new();
        let mut depth = 0i32;
        for c in lower.chars() {
            match c {
                '<' => depth += 1,
                '>' => {
                    depth = (depth - 1).max(0);
                    s.push(' ');
                }
                _ if depth == 0 => s.push(c),
                _ => {}
            }
        }
        s
    };
    if let Some(hit) = ["lorem ipsum", "dolor sit amet", "your text here", "placeholder text"].iter().find(|p| text_only.contains(*p)) {
        findings.push(static_finding(
            "QA-STATIC-PLACEHOLDER-COPY",
            Severity::Warning,
            "content",
            file,
            format!("Placeholder copy \"{}\" in visible text. Verify this is real before release.", hit),
            "Replace filler with the real copy, or mark the section as unfinished so it cannot ship.",
        ));
    }
    let stock = ["john doe", "jane doe", "john smith", "acme corp", "acme inc", "@example.com", "123 main st", "(555) 555", "555-01"];
    let hits: Vec<&str> = stock.iter().copied().filter(|p| text_only.contains(p)).collect();
    if !hits.is_empty() {
        findings.push(static_finding(
            "QA-STATIC-PLACEHOLDER-NAMES",
            Severity::Warning,
            "content",
            file,
            format!("Stock placeholder people, companies or contacts ({}). Verify this is real.", hits.join(", ")),
            "Use the client's real names and contact details, or clearly labelled sample data in demos.",
        ));
    }

    let mut positive_tab = 0;
    let mut ids: Vec<&str> = Vec::new();
    let mut dup_ids: Vec<&str> = Vec::new();
    let mut alt_files: Vec<&str> = Vec::new();
    let (mut autoplay_loud, mut no_poster) = (0, 0);
    let mut zoom_blocked = false;
    for tag in &tags {
        let name = tag_name(tag);
        if let Some(v) = attr_value(tag, "tabindex") {
            if v.trim().parse::<i32>().is_ok_and(|n| n > 0) {
                positive_tab += 1;
            }
        }
        if let Some(id) = attr_value(tag, "id") {
            let literal = !id.is_empty() && !id.contains(['{', '$', '<', '%', '`', '+']);
            if literal {
                if ids.contains(&id) {
                    if !dup_ids.contains(&id) {
                        dup_ids.push(id);
                    }
                } else {
                    ids.push(id);
                }
            }
        }
        match name {
            "img" => {
                if let Some(alt) = attr_value(tag, "alt") {
                    let a = alt.trim();
                    let ext = [".jpg", ".jpeg", ".png", ".gif", ".webp", ".avif", ".svg"].iter().any(|e| a.ends_with(e));
                    let camera = ["img_", "img-", "dsc_", "dsc0", "pxl_", "screenshot"].iter().any(|p| a.starts_with(p))
                        && a.chars().any(|c| c.is_ascii_digit());
                    if !a.is_empty() && (ext || camera) {
                        alt_files.push(a);
                    }
                }
            }
            "video" | "audio" => {
                if attr_value(tag, "autoplay").is_some() && attr_value(tag, "muted").is_none() {
                    autoplay_loud += 1;
                }
                if name == "video" && attr_value(tag, "poster").is_none() {
                    no_poster += 1;
                }
            }
            "meta" if attr_value(tag, "name") == Some("viewport") => {
                let c: String = attr_value(tag, "content").unwrap_or("").chars().filter(|c| !c.is_whitespace()).collect();
                let max_scale = c
                    .split(',')
                    .find_map(|p| p.strip_prefix("maximum-scale="))
                    .and_then(|v| v.parse::<f64>().ok());
                zoom_blocked = c.contains("user-scalable=no") || c.contains("user-scalable=0") || max_scale.is_some_and(|m| m < 2.0);
            }
            _ => {}
        }
    }
    if positive_tab > 0 {
        findings.push(static_finding(
            "A11Y-TABINDEX-POSITIVE",
            Severity::Error,
            "accessibility",
            file,
            format!("{} element(s) with a positive tabindex: keyboard order no longer follows the page (WCAG 2.4.3).", positive_tab),
            "Use tabindex=\"0\" (or none) and put elements in the DOM in reading order.",
        ));
    }
    if !dup_ids.is_empty() {
        // In a component the same literal id can sit in exclusive branches, so it is a warning there
        findings.push(static_finding(
            "A11Y-DUPLICATE-ID",
            if is_plain_html { Severity::Error } else { Severity::Warning },
            "accessibility",
            file,
            format!("Duplicate id value(s): {}. Labels, aria references and anchors then point at the wrong element.", dup_ids.iter().take(5).cloned().collect::<Vec<_>>().join(", ")),
            "Give every id a unique value; generate ids per instance in reusable components.",
        ));
    }
    if !alt_files.is_empty() {
        findings.push(static_finding(
            "A11Y-ALT-FILENAME",
            Severity::Error,
            "accessibility",
            file,
            format!("{} image alt text(s) are file names (e.g. \"{}\"), which say nothing to a screen reader (WCAG 1.1.1).", alt_files.len(), alt_files[0]),
            "Describe what the image shows and why it is there, or use alt=\"\" if it is decorative.",
        ));
    }
    if autoplay_loud > 0 {
        findings.push(static_finding(
            "MEDIA-AUTOPLAY-UNMUTED",
            Severity::Warning,
            "media",
            file,
            format!("{} autoplaying media element(s) without `muted`: browsers block them, and sound that starts on its own fails WCAG 1.4.2 after 3s.", autoplay_loud),
            "Add `muted` (and `playsinline`) to autoplaying video, or start playback only on a user action.",
        ));
    }
    if no_poster > 0 {
        findings.push(static_finding(
            "MEDIA-VIDEO-NO-POSTER",
            Severity::Warning,
            "performance",
            file,
            format!("{} <video> element(s) without a poster: the box is blank until the video decodes, and it may be the LCP element.", no_poster),
            "Add a `poster` image sized like the video (a compressed first frame).",
        ));
    }
    if zoom_blocked {
        findings.push(static_finding(
            "A11Y-ZOOM-DISABLED",
            Severity::Error,
            "accessibility",
            file,
            "The viewport meta blocks pinch zoom (user-scalable=no or maximum-scale under 2), so text cannot be enlarged to 200% (WCAG 1.4.4).".into(),
            "Use `width=device-width, initial-scale=1` and drop user-scalable / maximum-scale.",
        ));
    }
}

/// One CSS style rule with the @media context it sits in ("" at top level).
#[derive(Debug, Clone)]
struct CssRule {
    media: String,
    selector: String,
    decls: Vec<(String, String)>,
}

fn strip_css_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(s) = rest.find("/*") {
        out.push_str(&rest[..s]);
        match rest[s + 2..].find("*/") {
            Some(e) => rest = &rest[s + 2 + e + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

fn parse_decls(body: &str) -> Vec<(String, String)> {
    body.split(';')
        .filter_map(|d| {
            let (k, v) = d.split_once(':')?;
            let k = k.trim();
            let v = v.trim().trim_end_matches("!important").trim();
            (!k.is_empty() && !k.contains(['{', '}'])).then(|| (k.to_string(), v.to_string()))
        })
        .collect()
}

/// Splits a stylesheet into style rules (each tagged with its @media context, contexts never merged)
/// and @keyframes declaration lists.
fn parse_css(css: &str, media: &str, rules: &mut Vec<CssRule>, keyframes: &mut Vec<(String, Vec<(String, String)>)>, depth: usize) {
    if depth > 8 {
        return;
    }
    let mut i = 0;
    let b = css.as_bytes();
    while i < b.len() {
        let Some(rel) = css[i..].find(['{', ';', '}']) else { break };
        let at = i + rel;
        if b[at] != b'{' {
            i = at + 1;
            continue;
        }
        let prelude = css[i..at].trim().to_string();
        let mut d = 0usize;
        let mut end = css.len();
        for (k, c) in css[at..].char_indices() {
            match c {
                '{' => d += 1,
                '}' => {
                    d -= 1;
                    if d == 0 {
                        end = at + k;
                        break;
                    }
                }
                _ => {}
            }
        }
        let body = &css[at + 1..end.min(css.len())];
        if let Some(cond) = prelude.strip_prefix("@media") {
            let cond = cond.trim();
            let ctx = if media.is_empty() { cond.to_string() } else { format!("{} and {}", media, cond) };
            parse_css(body, &ctx, rules, keyframes, depth + 1);
        } else if prelude.contains("keyframes") {
            let name = prelude.split_whitespace().nth(1).unwrap_or("").to_string();
            let mut frames = Vec::new();
            let mut inner = Vec::new();
            parse_css(body, media, &mut inner, &mut Vec::new(), depth + 1);
            for r in inner {
                frames.extend(r.decls);
            }
            keyframes.push((name, frames));
        } else if prelude.starts_with("@supports") || prelude.starts_with("@layer") || prelude.starts_with("@container") {
            parse_css(body, media, rules, keyframes, depth + 1);
        } else if !prelude.starts_with('@') {
            let own = body.split('{').next().unwrap_or("");
            // With nesting, the text after the last ';' before the first '{' is a nested selector
            let own = if body.contains('{') { own.rsplit_once(';').map_or("", |(a, _)| a) } else { own };
            let selector = prelude.split_whitespace().collect::<Vec<_>>().join(" ");
            rules.push(CssRule { media: media.to_string(), selector: selector.clone(), decls: parse_decls(own) });
            if body.contains('{') {
                // Native CSS nesting: nested rules are checked as their own rules
                parse_css(&body[own.len()..], media, rules, keyframes, depth + 1);
            }
        }
        i = end + 1;
    }
}

/// Parses #rgb(a), #rrggbb(aa), rgb()/rgba() (comma or space syntax), white, black, transparent.
fn parse_css_color(v: &str) -> Option<[f64; 4]> {
    let v = v.trim();
    match v {
        "white" => return Some([255.0, 255.0, 255.0, 1.0]),
        "black" => return Some([0.0, 0.0, 0.0, 1.0]),
        "transparent" => return Some([0.0, 0.0, 0.0, 0.0]),
        _ => {}
    }
    if let Some(h) = v.strip_prefix('#') {
        let h: Vec<u8> = h.bytes().collect();
        let hex = |s: &[u8]| u8::from_str_radix(std::str::from_utf8(s).ok()?, 16).ok().map(|n| n as f64);
        return match h.len() {
            3 | 4 => {
                let d = |i: usize| hex(&[h[i], h[i]]);
                let a = if h.len() == 4 { d(3)? / 255.0 } else { 1.0 };
                Some([d(0)?, d(1)?, d(2)?, a])
            }
            6 | 8 => {
                let a = if h.len() == 8 { hex(&h[6..8])? / 255.0 } else { 1.0 };
                Some([hex(&h[0..2])?, hex(&h[2..4])?, hex(&h[4..6])?, a])
            }
            _ => None,
        };
    }
    let inner = v.strip_prefix("rgba(").or_else(|| v.strip_prefix("rgb("))?.strip_suffix(')')?;
    let parts: Vec<&str> = inner.split(|c: char| c == ',' || c == '/' || c.is_whitespace()).filter(|p| !p.is_empty()).collect();
    if parts.len() < 3 {
        return None;
    }
    let chan = |p: &str| -> Option<f64> {
        if let Some(pc) = p.strip_suffix('%') { pc.parse::<f64>().ok().map(|n| n * 2.55) } else { p.parse::<f64>().ok() }
    };
    let a = match parts.get(3) {
        Some(p) => match p.strip_suffix('%') {
            Some(pc) => pc.parse::<f64>().ok()? / 100.0,
            None => p.parse::<f64>().ok()?,
        },
        None => 1.0,
    };
    Some([chan(parts[0])?, chan(parts[1])?, chan(parts[2])?, a])
}

fn rel_luminance(c: [f64; 4]) -> f64 {
    let lin = |v: f64| {
        let s = v / 255.0;
        if s <= 0.04045 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * lin(c[0]) + 0.7152 * lin(c[1]) + 0.0722 * lin(c[2])
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum StaticBg {
    Unset,
    Solid([f64; 4]),
    /// Image, gradient, translucent or unknown (var(), currentColor...): no ratio is claimed
    Unmeasurable,
}

/// Applies declarations in order. A `background:` shorthand resets the colour: with only an image the
/// backdrop becomes unmeasurable, never the earlier `background-color`.
fn resolve_pair(decls: &[&(String, String)]) -> (Option<[f64; 4]>, StaticBg) {
    let mut fg = None;
    let mut bg = StaticBg::Unset;
    for (k, v) in decls {
        match k.as_str() {
            "color" => fg = parse_css_color(v),
            "background-color" => {
                bg = match parse_css_color(v) {
                    Some(c) if c[3] >= 0.999 => StaticBg::Solid(c),
                    Some(c) if c[3] <= 0.001 => StaticBg::Unset,
                    _ => StaticBg::Unmeasurable,
                }
            }
            "background-image" if v.trim() != "none" => bg = StaticBg::Unmeasurable,
            "background" => {
                let v = v.trim();
                bg = if v.contains("url(") || v.contains("gradient(") || v.contains("var(") || v.contains("image-set(") {
                    StaticBg::Unmeasurable
                } else {
                    match v.split_whitespace().rev().find_map(parse_css_color).or_else(|| parse_css_color(v)) {
                        Some(c) if c[3] >= 0.999 => StaticBg::Solid(c),
                        Some(c) if c[3] <= 0.001 => StaticBg::Unset,
                        Some(_) => StaticBg::Unmeasurable,
                        None => StaticBg::Unset,
                    }
                }
            }
            _ => {}
        }
    }
    (fg, bg)
}

fn audit_static_css(file: &str, css: &str, findings: &mut Vec<AuditFinding>) {
    let css = strip_css_comments(css);
    let mut rules = Vec::new();
    let mut keyframes = Vec::new();
    parse_css(&css, "", &mut rules, &mut keyframes, 0);

    // transition: all
    let trans_all = rules
        .iter()
        .filter(|r| {
            r.decls.iter().any(|(k, v)| {
                (k == "transition" || k == "transition-property")
                    && v.split(',').any(|part| part.split_whitespace().next() == Some("all"))
            })
        })
        .map(|r| r.selector.as_str())
        .collect::<Vec<_>>();
    if !trans_all.is_empty() {
        findings.push(static_finding(
            "UI-TRANSITION-ALL",
            Severity::Warning,
            "performance",
            file,
            format!("`transition: all` on {} rule(s), e.g. `{}`.", trans_all.len(), trans_all[0]),
            "Name the properties that should move (`transform`, `opacity`, `color`, `background-color`); `all` also animates layout and every future change.",
        ));
    }

    // Keyframes that animate layout properties
    let layout_props = ["width", "height", "top", "left", "right", "bottom", "margin", "padding", "inset"];
    let layout_kf: Vec<String> = keyframes
        .iter()
        .filter_map(|(name, decls)| {
            decls
                .iter()
                .find(|(k, _)| layout_props.iter().any(|p| k == p || k.starts_with(&format!("{}-", p))))
                .map(|(k, _)| format!("{} ({})", name, k))
        })
        .collect();
    if !layout_kf.is_empty() {
        findings.push(static_finding(
            "MOTION-LAYOUT-KEYFRAMES",
            Severity::Warning,
            "performance",
            file,
            format!("@keyframes animate layout properties: {}. Each frame re-runs layout and can jank and shift content.", layout_kf.join(", ")),
            "Animate `transform` and `opacity` instead (scale/translate), or clip-path for reveals.",
        ));
    }

    // Static contrast: only colour pairs both known in one cascade context. A @media block sees the
    // top-level rules plus its own, never another @media block's.
    let mut selectors: Vec<&str> = rules.iter().map(|r| r.selector.as_str()).collect();
    selectors.sort_unstable();
    selectors.dedup();
    let (mut unmeasurable, mut reported) = (0usize, 0usize);
    for sel in selectors {
        let mine: Vec<&CssRule> = rules.iter().filter(|r| r.selector == sel).collect();
        let mut contexts: Vec<&str> = mine.iter().map(|r| r.media.as_str()).collect();
        contexts.sort_unstable();
        contexts.dedup();
        if !contexts.contains(&"") {
            contexts.insert(0, "");
        }
        let mut done = false;
        let mut sel_unmeasurable = false;
        for ctx in contexts {
            let decls: Vec<&(String, String)> = mine
                .iter()
                .filter(|r| r.media.is_empty())
                .chain(mine.iter().filter(|r| !ctx.is_empty() && r.media == ctx))
                .flat_map(|r| r.decls.iter())
                .collect();
            let (fg, bg) = resolve_pair(&decls);
            let Some(fg) = fg else { continue };
            match bg {
                StaticBg::Unmeasurable => sel_unmeasurable = true,
                StaticBg::Solid(b) if !done && fg[3] >= 0.999 => {
                    let (l1, l2) = (rel_luminance(fg), rel_luminance(b));
                    let ratio = (l1.max(l2) + 0.05) / (l1.min(l2) + 0.05);
                    if ratio < 4.5 {
                        done = true;
                        reported += 1;
                        if reported > 6 {
                            continue;
                        }
                        let where_ = if ctx.is_empty() { String::new() } else { format!(" inside @media {}", ctx) };
                        let (sev, note) = if ratio < 3.0 {
                            (Severity::Error, "fails WCAG 1.4.3 at any text size")
                        } else {
                            (Severity::Warning, "fails for normal text, passes only for large text; `layoza qa` measures the real size")
                        };
                        findings.push(static_finding(
                            "A11Y-STATIC-CONTRAST",
                            sev,
                            "accessibility",
                            &format!("{} {}", file, sel),
                            format!("Text on its own declared background measures {:.2}:1{} ({}).", ratio, where_, note),
                            "Darken or lighten the text or background until normal text reaches 4.5:1 and large text 3:1.",
                        ));
                    }
                }
                _ => {}
            }
        }
        if sel_unmeasurable {
            unmeasurable += 1;
        }
    }
    if unmeasurable > 0 {
        findings.push(static_finding(
            "A11Y-CONTRAST-UNMEASURABLE",
            Severity::Info,
            "accessibility",
            file,
            format!("contrast unmeasurable (image/gradient) for {} rule(s) with text on an image, gradient or translucent background: static checks claim no ratio.", unmeasurable),
            "Run `layoza qa` to composite the real backdrop, or give the text a solid backing.",
        ));
    }
}

/// Compares before and after audit/visual snapshots to distinguish intended changes from unintended regressions.
pub fn verify_before_after(
    before: &AuditReport,
    after: &AuditReport,
    pixel_diff_ratio: Option<f64>,
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

    VisualRegressionComparison {
        baseline_present: true,
        viewports_compared: Vec::new(),
        comparison_basis: if pixel_diff_ratio.is_some() {
            "Audit findings and an externally supplied pixel-change ratio; this function does not decode screenshots. Speed is optional and does not control acceptance.".to_string()
        } else {
            "Audit findings only; no screenshot pixel-change measurement supplied. Speed is optional and does not control acceptance.".to_string()
        },
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
mod static_subset_tests {
    use super::*;
    use serde_json::json;

    fn run(label: &str, src: &str) -> Vec<AuditFinding> {
        let mut f = Vec::new();
        audit_static_subset(label, src, &mut f);
        f
    }

    fn ids(f: &[AuditFinding]) -> Vec<&str> {
        f.iter().map(|x| x.rule_id.as_str()).collect()
    }

    #[test]
    fn exclusive_media_blocks_are_never_merged() {
        let f = run("a.css", "@media (max-width:600px){.a{color:#eee}} @media (min-width:601px){.a{background:#fff}}");
        assert!(!ids(&f).contains(&"A11Y-STATIC-CONTRAST"), "{:?}", ids(&f));
        // Top-level rules do apply inside a media block
        let f = run("a.css", ".b{color:#eee} @media (min-width:601px){.b{background:#fff}}");
        let hit = f.iter().find(|x| x.rule_id == "A11Y-STATIC-CONTRAST").expect("pair under media");
        assert!(hit.message.contains("@media (min-width:601px)"));
        assert_eq!(hit.severity, Severity::Error);
    }

    #[test]
    fn background_shorthand_with_an_image_resets_the_colour() {
        let f = run("a.css", ".c{background-color:#fff;color:#eee;background:url(x.png)}");
        assert!(!ids(&f).contains(&"A11Y-STATIC-CONTRAST"));
        assert!(ids(&f).contains(&"A11Y-CONTRAST-UNMEASURABLE"));
        let f = run("a.css", ".d{color:#767676;background:#fff}");
        assert!(!ids(&f).contains(&"A11Y-STATIC-CONTRAST"), "4.54:1 passes");
        let f = run("a.css", ".e{color:#777;background:#fff none}");
        assert_eq!(f.iter().find(|x| x.rule_id == "A11Y-STATIC-CONTRAST").unwrap().severity, Severity::Warning);
    }

    #[test]
    fn same_rules_run_on_every_template_language() {
        let src = r#"<template><img src="a.jpg" alt="IMG_1234.jpg"><button tabindex="3">x</button>
            <video autoplay src="v.mp4"></video><style>.x{transition:all .2s}@keyframes grow{to{width:100%}}</style></template>"#;
        for label in ["c.vue", "c.svelte", "c.astro", "c.jsx", "c.tsx", "p.php", "v.blade.php", "s.liquid", "i.html"] {
            let f = run(label, src);
            for want in ["A11Y-ALT-FILENAME", "A11Y-TABINDEX-POSITIVE", "MEDIA-AUTOPLAY-UNMUTED", "MEDIA-VIDEO-NO-POSTER", "UI-TRANSITION-ALL", "MOTION-LAYOUT-KEYFRAMES"] {
                assert!(ids(&f).contains(&want), "{} missing {}: {:?}", label, want, ids(&f));
            }
        }
        // JSX spelling and muted video
        let f = run("c.tsx", r#"<video autoPlay muted playsInline poster="p.jpg" /><div tabIndex={0} />"#);
        assert!(!ids(&f).contains(&"MEDIA-AUTOPLAY-UNMUTED") && !ids(&f).contains(&"A11Y-TABINDEX-POSITIVE"));
    }

    #[test]
    fn markup_rules_catch_ids_zoom_and_placeholders() {
        let f = run(
            "i.html",
            r#"<html><head><meta name="viewport" content="width=device-width, maximum-scale=1"></head>
            <body><p id="a">Lorem ipsum dolor</p><p id="a">Call Jane Doe</p></body></html>"#,
        );
        for want in ["A11Y-ZOOM-DISABLED", "A11Y-DUPLICATE-ID", "QA-STATIC-PLACEHOLDER-COPY", "QA-STATIC-PLACEHOLDER-NAMES"] {
            assert!(ids(&f).contains(&want), "missing {}", want);
        }
        assert!(f.iter().find(|x| x.rule_id == "QA-STATIC-PLACEHOLDER-COPY").unwrap().message.contains("Verify this is real"));
        let ok = run("i.html", r#"<meta name="viewport" content="width=device-width, initial-scale=1"><p id="a">Real copy</p>"#);
        assert!(ok.is_empty(), "{:?}", ids(&ok));
    }

    #[test]
    fn static_audit_never_passes_wcag_without_a_browser_report() {
        let dir = std::env::temp_dir().join(format!("layoza-honesty-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        fs::write(dir.join("index.html"), "<!doctype html><html lang=\"en\"><head><meta name=\"viewport\" content=\"width=device-width\"></head><body><main><h1>Hi</h1></main></body></html>").unwrap();
        fs::write(dir.join("t.liquid"), "<img src=\"a.png\" alt=\"a.png\">").unwrap();
        let r = audit_project(&dir, None, None, None, None).unwrap();
        assert!(!r.accessibility.passes_automated_wcag22_aa);
        assert!(r.accessibility.evidence_basis.starts_with("static checks only"));
        assert!(!r.accessibility.needs_browser.is_empty());
        assert!(r.findings.iter().any(|f| f.rule_id == "A11Y-ALT-FILENAME" && f.file_or_selector == "t.liquid"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn quality_issues_keep_their_own_why_fix_and_measured_value() {
        let report = json!({
            "schema_version": "2.0.0",
            "contrast_unmeasurable_reasons": { "gradient": 2 },
            "viewports": [{ "label": "mobile", "issues": [
                { "rule": "text-below-floor", "severity": "error", "category": "legibility", "selector": "p.fine",
                  "measured": "12px body text", "why": "Too small to read.", "fix": "Set body text to 16px.", "detail": "p.fine: 12px" }
            ]}]
        });
        let f = browser_report_findings(&report, "qa.json");
        let q = f.iter().find(|x| x.rule_id == "QA-TEXT-BELOW-FLOOR").expect("generic QA finding");
        assert_eq!(q.severity, Severity::Error);
        assert_eq!(q.category, "legibility");
        assert_eq!(q.remediation, "Set body text to 16px.");
        assert!(q.message.contains("12px body text") && q.file_or_selector.ends_with("p.fine"));
        assert!(f.iter().any(|x| x.rule_id == "A11Y-CONTRAST-UNMEASURABLE" && x.message.contains("gradient")));
    }
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

    fn process_rules(direction: Option<&str>) -> Vec<String> {
        let root = std::env::temp_dir().join(format!(
            "layoza-process-{}-{}",
            std::process::id(),
            direction.map(|d| d.len()).unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".layoza")).unwrap();
        if let Some(d) = direction {
            std::fs::write(root.join(".layoza/direction.md"), d).unwrap();
        }
        let mut findings = Vec::new();
        audit_design_process_with(&root, None, &mut findings);
        let _ = std::fs::remove_dir_all(&root);
        findings.into_iter().map(|f| f.rule_id).collect()
    }

    #[test]
    fn design_process_is_required_only_when_a_direction_exists() {
        // The motion level must be a decided number, not prose.
        let rules = process_rules(Some("PURPOSE: x; new
PICKED: auto — x
SIGNATURE: x
MOTION: heavy
ROUND 1: x
"));
        assert!(rules.contains(&"DESIGN-NO-MOTION-LEVEL".to_string()));

        // Mode B/C projects have no direction file and are untouched.
        assert!(process_rules(None).is_empty());

        // A direction with no pick, no signature and no review fails each check.
        let rules = process_rules(Some("THESIS: x\n"));
        assert!(rules.contains(&"DESIGN-NO-PURPOSE".to_string()));
        assert!(rules.contains(&"DESIGN-NO-DIRECTION-PICK".to_string()));
        assert!(rules.contains(&"DESIGN-NOT-REVIEWED".to_string()));
        assert!(rules.contains(&"DESIGN-NO-SIGNATURE".to_string()));

        // No screenshot review logged and no signature.
        let rules = process_rules(Some("PURPOSE: sell\nPICKED: B (user)\n"));
        assert_eq!(rules, vec!["DESIGN-NOT-REVIEWED".to_string(), "DESIGN-NO-SIGNATURE".to_string(), "DESIGN-NO-MOTION-LEVEL".to_string(), "DESIGN-NO-SUBJECT".to_string()]);

        // A UTF-8 BOM (PowerShell 5.1 Set-Content) must not hide the first line.
        let rules = process_rules(Some("\u{feff}PURPOSE: sell\nPICKED: B (user)\nROUND 1: fixed hero crop\n"));
        assert_eq!(rules, vec!["DESIGN-NO-SIGNATURE".to_string(), "DESIGN-NO-MOTION-LEVEL".to_string(), "DESIGN-NO-SUBJECT".to_string()]);

        // An empty pick does not count; "round" prose does not count as a round;
        // markdown bold around a key is accepted.
        let rules = process_rules(Some("**PURPOSE:** install\nPICKED:\nSIGNATURE: x\nMOTION: 2\nROUND one: x\n"));
        assert_eq!(rules, vec!["DESIGN-NO-DIRECTION-PICK".to_string(), "DESIGN-NOT-REVIEWED".to_string()]);
        let rules = process_rules(Some("**PURPOSE:** install\nPICKED:\nSIGNATURE: x\nMOTION: 2\n- Round 1: 7\n"));
        assert_eq!(rules, vec!["DESIGN-NO-DIRECTION-PICK".to_string()]);

        // A complete record passes: purpose, auto pick, signature, one review round.
        assert!(process_rules(Some(
            "PURPOSE: book tables; audience: locals; new\nPICKED: auto — harbour chart\nSIGNATURE: the catch board fills as you scroll the tide\nMOTION: 2 — locals booking on phones\nROUND 1: tightened hero\n"
        ))
        .is_empty());

        // An existing site (Mode B/C) follows its own design: no signature needed, still reviewed.
        let rules = process_rules(Some(
            "PURPOSE: add a pricing section; audience: current visitors; existing\nPICKED: existing — follows SITE-DNA.json\n",
        ));
        assert_eq!(rules, vec!["DESIGN-NOT-REVIEWED".to_string()]);
        assert!(process_rules(Some(
            "PURPOSE: improve contrast; existing\n**PICKED:** existing — incumbent design\nROUND 1: 7\n"
        ))
        .is_empty());
    }

    const LOOP: &str = "SIGNATURE: x\nMOTION: 3\nROUND 1: 7\n";

    #[test]
    fn direction_must_name_the_trades_objects() {
        // Only style words: clearly generic, an error.
        let rules = process_rules(Some(&format!("PURPOSE: a modern clean site; new\nPICKED: A bold minimal\n{}", LOOP)));
        assert_eq!(rules, vec!["DESIGN-GENERIC-DIRECTION".to_string()]);
        // The purpose names the trade but the direction is only style: a warning.
        let mut findings = Vec::new();
        let blocks = crate::history::parse_direction(
            "PURPOSE: book tables at a seafood bar\nPICKED: A (user)\nTHESIS: bold, modern and clean\n",
        );
        audit_direction_subject(&blocks, "d", &mut findings);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "DESIGN-GENERIC-DIRECTION");
        assert_eq!(findings[0].severity, Severity::Warning);
        // A direction that names the trade's objects passes.
        assert!(process_rules(Some(&format!(
            "PURPOSE: book tables at a seafood bar; new\nPICKED: B harbour chart (user)\nTHESIS: the daily catch board becomes a tide chart\n{}",
            LOOP
        )))
        .is_empty());
    }

    #[test]
    fn repeated_direction_warns_and_is_recorded_once() {
        let base = std::env::temp_dir().join(format!("layoza-repeat-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let ledger = base.join("home/history.jsonl");
        let run = |name: &str, direction: &str| -> Vec<String> {
            let root = base.join(name);
            std::fs::create_dir_all(root.join(".layoza")).unwrap();
            std::fs::write(root.join(".layoza/direction.md"), direction).unwrap();
            let mut findings = Vec::new();
            audit_design_process_with(&root, Some(&ledger), &mut findings);
            // Each run stands in for a passed gate, which is when a pick is recorded.
            record_verified_direction(&root, &ledger, &mut findings);
            findings.into_iter().map(|f| f.rule_id).collect()
        };
        let first = format!(
            "PURPOSE: book tables at a harbour bar; new\nPICKED: B harbour chart (user)\nTHESIS: catch board as tide chart\nOWN-WORLD: blueprint paper, ticket stubs\nTOKENS: #0f1b2d #1d6f8a #f4efe6 #e2552b; Fraunces display, Inter body\n{}",
            LOOP
        );
        assert!(run("a", &first).is_empty());
        assert!(run("a", &first).is_empty(), "re-auditing a project must not warn on itself");
        assert_eq!(crate::history::read_all(&ledger).len(), 1);
        let second = first.replace("harbour bar", "river cafe").replace("harbour chart", "river map");
        assert_eq!(run("b", &second), vec!["DESIGN-REPEATS-RECENT".to_string()]);
        // The user asked for the same look: no warning.
        assert!(run("c", &format!("{}REPEAT-OK: client wants the sister-site look\n", second)).is_empty());
        // Existing sites are never recorded.
        run("d", "PURPOSE: add pricing; existing\nPICKED: existing — SITE-DNA.json\n");
        assert_eq!(crate::history::read_all(&ledger).len(), 3);
        let _ = std::fs::remove_dir_all(&base);
    }
}

#[cfg(test)]
mod gate_file_tests {
    use super::*;
    use serde_json::json;

    fn site(tag: &str, direction: Option<&str>, approved: Option<&str>) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("layoza-gate-{}-{}", tag, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join(".layoza")).unwrap();
        if let Some(d) = direction {
            fs::write(dir.join(".layoza/direction.md"), d).unwrap();
        }
        if let Some(a) = approved {
            fs::write(dir.join(".layoza/direction-approved.md"), a).unwrap();
        }
        dir
    }

    fn gate(dir: &Path, strict: bool) -> Vec<AuditFinding> {
        let mut f = Vec::new();
        audit_direction_approval(dir, strict, &mut f);
        f
    }

    #[test]
    fn new_site_without_approval_warns_and_fails_under_strict() {
        let dir = site("new", Some("PURPOSE: book tables; audience: diners; new; keep: menu\nPICKED: B (chosen by the user)\n"), None);
        let f = gate(&dir, false);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].rule_id, "DESIGN-DIRECTION-NOT-APPROVED");
        assert_eq!(f[0].severity, Severity::Warning);
        assert_eq!(gate(&dir, true)[0].severity, Severity::Error);
        let strict = audit_project_with(&dir, None, Some(900.0), Some(40.0), Some(0.01), &AuditOptions { strict: true }).unwrap();
        assert!(!strict.passed_release_gate);
        assert!(strict.findings.iter().any(|x| x.rule_id == "DESIGN-DIRECTION-NOT-APPROVED" && x.severity == Severity::Error));
        fs::write(dir.join(".layoza/direction-approved.md"), "B, approved by the user 2026-10-19").unwrap();
        assert!(gate(&dir, true).is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn existing_site_or_unknown_kind_has_no_gate_file_finding() {
        let dir = site("existing", Some("PURPOSE: sell shoes; audience: runners; existing; keep: cart\nPICKED: existing - follow the shop\n"), None);
        assert!(gate(&dir, true).is_empty());
        let none = site("unknown", None, None);
        assert!(gate(&none, true).is_empty());
        let empty = site("empty", Some("PURPOSE: x; new\nPICKED: A\n"), Some("  \n"));
        assert_eq!(gate(&empty, false).len(), 1);
        let auto = site("auto", Some("PURPOSE: sell phones; audience: shoppers; new\nPICKED: auto — C configurator, fits a phone store\n"), None);
        assert!(gate(&auto, true).is_empty(), "a reasoned auto pick needs no approval file");
        let bare = site("bare-auto", Some("PURPOSE: sell phones; new\nPICKED: auto\n"), None);
        assert_eq!(gate(&bare, false).len(), 1, "auto without a reason is not a pick");
        for d in [dir, none, empty, auto, bare] {
            let _ = fs::remove_dir_all(&d);
        }
    }

    #[test]
    fn section_reskins_are_ingested_with_their_score() {
        let report = json!({ "look_ledger": { "section_matches": [
            { "rule": "QA-SECTION-RESKIN", "role": "hero", "section_index": 1, "section_label": "Book a table",
              "matched_target": "old/index.html", "matched_label": "Plan a trip", "similarity": 0.962, "palette_distance": 0.31, "same_type": false }
        ] } });
        let mut f = Vec::new();
        ingest_section_reskins(&report, "qa/browser-qa-report.json", &mut f);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].rule_id, "QA-SECTION-RESKIN");
        assert_eq!(f[0].severity, Severity::Warning);
        assert!(f[0].message.contains("similarity 0.962") && f[0].message.contains("old/index.html"), "{}", f[0].message);
    }

    #[test]
    fn same_section_twice_issue_maps_to_its_own_rule_id() {
        let report = json!({ "viewports": [{ "label": "desktop", "issues": [{
            "rule": "same-section-twice", "severity": "warning", "category": "design-quality", "selector": "section #2 / #4",
            "detail": "d", "measured": "structure similarity 0.97 (flag at >= 0.93)", "why": "w", "fix": "f" }] }] });
        let mut f = Vec::new();
        ingest_browser_report(&report, "qa/browser-qa-report.json", &mut f);
        assert!(
            f.iter().any(|x| x.rule_id == "QA-SAME-SECTION-TWICE" && x.severity == Severity::Warning),
            "{:?}",
            f.iter().map(|x| &x.rule_id).collect::<Vec<_>>()
        );
    }

    #[test]
    fn skipped_passes_are_listed_as_needing_the_browser() {
        let dir = site("skipped", None, None);
        fs::create_dir_all(dir.join("qa")).unwrap();
        let report = json!({ "schema_version": "2.0", "mode": "fast", "lcp_ms": 900.0, "inp_ms": 40.0, "cls": 0.01,
            "skipped_passes": ["throttled-perf@375 (fast mode; run with --full)"], "viewports": [] });
        fs::write(dir.join("qa/browser-qa-report.json"), report.to_string()).unwrap();
        let r = audit_project(&dir, None, None, None, None).unwrap();
        assert!(r.accessibility.needs_browser.iter().any(|s| s.contains("throttled-perf@375")), "{:?}", r.accessibility.needs_browser);
        assert!(r.accessibility.evidence_basis.contains("fast mode"), "{}", r.accessibility.evidence_basis);
        let _ = fs::remove_dir_all(&dir);
    }
}
