// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Layoza Core Library: single source of truth shared across the CLI, MCP server, and AI Skills.

pub mod adapters;
pub mod animation;
pub mod baseline;
pub mod benchmark;
pub mod design_md;
pub mod dna;
pub mod hooks;
pub mod installer;
pub mod inspector;
pub mod intelligence;
pub mod mechanics;
pub mod qa;
pub mod verifier;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_wcag_contrast_and_design_md_lint() {
        let ratio = design_md::wcag_contrast_ratio((20, 22, 25), (248, 247, 244));
        assert!(ratio > 14.0, "Expected high contrast ratio, got {}", ratio);

        let temp_dir = std::env::temp_dir().join("layoza_test_design_md");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let (dna, _) = inspector::inspect_project(&temp_dir, None, false).unwrap();
        let md = design_md::generate_design_md(&dna);
        let lint = design_md::lint_design_md(&md);
        assert!(lint.valid, "Generated DESIGN.md must pass spec lint: {:?}", lint.findings);
        assert_eq!(lint.summary.errors, 0);

        let diff = design_md::diff_design_md(&md, &md);
        assert!(!diff.regression);
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_inline_root_custom_properties_are_extracted() {
        let temp_dir = std::env::temp_dir().join("layoza_test_inline_root");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        fs::write(
            temp_dir.join("index.html"),
            "<style>:root{--ink:#14161a;--paper:#f6f5f0;--font-display:\"Space Grotesk\", sans-serif}</style>\n<div style=\"--accent:#4865ff\">x</div>\n<script>for(let i=9;i--;){}</script>",
        )
        .unwrap();
        let (dna, _) = inspector::inspect_project(&temp_dir, None, false).unwrap();
        let names: Vec<&str> = dna.colors.iter().map(|c| c.name.as_str()).collect();
        for want in ["ink", "paper", "accent"] {
            assert!(names.contains(&want), "missing --{want}, got {names:?}");
        }
        let accent = dna.colors.iter().find(|c| c.name == "accent").unwrap();
        assert_eq!(accent.value, "#4865ff");
        assert!(dna
            .typography
            .iter()
            .any(|t| t.role == "font-display" && t.font_family.starts_with("Space Grotesk")));
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_three_directions_and_modes() {
        let rec = intelligence::recommend_design(
            "Build a luxury watch atelier website",
            intelligence::OperatingMode::NewWebsite,
            None,
            None,
        );
        assert_eq!(rec.matched_industry.slug, "luxury-premium");
        assert_eq!(rec.directions.len(), 3);
        assert_ne!(
            rec.directions[0].layout_composition,
            rec.directions[1].layout_composition
        );
        assert_ne!(
            rec.directions[0].typography.display_font,
            rec.directions[1].typography.display_font
        );
    }

    #[test]
    fn test_framework_adapters_preserve_native_formats() {
        let temp_dir = std::env::temp_dir().join("layoza_test_adapters");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        let (dna, _) = inspector::inspect_project(&temp_dir, None, false).unwrap();

        let wp = adapters::generate_framework_artifact(
            "wordpress-gutenberg",
            "Editorial Block Hero",
            "Native block theme with theme.json v3",
            "Read More",
            &dna,
        );
        assert!(wp.files.contains_key("theme.json"));
        assert!(wp.files.contains_key("patterns/hero-section.php"));
        assert!(wp.files["patterns/hero-section.php"].contains("<!-- wp:group"));

        let astro = adapters::generate_framework_artifact(
            "astro",
            "Astro Fast Hero",
            "Zero JS by default",
            "Launch",
            &dna,
        );
        assert!(astro.files.contains_key("src/components/LayozaSection.astro"));

        let elementor = adapters::generate_framework_artifact(
            "elementor",
            "Elementor Container",
            "Native JSON widget tree",
            "Explore",
            &dna,
        );
        assert!(elementor.files["elementor/layoza-section-template.json"].contains("\"elType\": \"container\""));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_baseline_and_rollback_and_security() {
        let temp_dir = std::env::temp_dir().join("layoza_test_baseline");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let index_path = temp_dir.join("index.html");
        fs::write(&index_path, "<h1>Original Baseline</h1>").unwrap();

        let snap = baseline::create_baseline(
            &temp_dir,
            "pre-redesign",
            "Test Mode D baseline",
            &["index.html".to_string()],
        )
        .unwrap();
        assert_eq!(snap.files.len(), 1);

        // Modify file destructively
        fs::write(&index_path, "<h1>Broken Replacement</h1>").unwrap();
        assert_eq!(
            fs::read_to_string(&index_path).unwrap(),
            "<h1>Broken Replacement</h1>"
        );

        // Rollback
        let rb = baseline::rollback_baseline(&temp_dir, "pre-redesign").unwrap();
        assert!(rb.success);
        assert_eq!(
            fs::read_to_string(&index_path).unwrap(),
            "<h1>Original Baseline</h1>"
        );

        // Path traversal block
        let traversal = baseline::validate_safe_project_path(&temp_dir, std::path::Path::new("../outside.txt"));
        assert!(traversal.is_err());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_baseline_default_discovery_skips_and_errors() {
        let root = std::env::temp_dir().join("layoza_test_baseline_discovery");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("src/components")).unwrap();
        fs::create_dir_all(root.join("node_modules/pkg")).unwrap();
        fs::create_dir_all(root.join("dist")).unwrap();
        fs::create_dir_all(root.join("templates")).unwrap();
        fs::write(root.join("index.html"), "<h1>Home</h1>").unwrap();
        fs::write(root.join("DESIGN.md"), "# Design").unwrap();
        fs::write(root.join("src/components/Card.tsx"), "export const Card = 1;").unwrap();
        fs::write(root.join("templates/index.json"), "{}").unwrap();
        fs::write(root.join("package-lock.json"), "{}").unwrap();
        fs::write(root.join("node_modules/pkg/index.js"), "x").unwrap();
        fs::write(root.join("dist/app.js"), "x").unwrap();
        fs::write(root.join("bad.css"), [0xffu8, 0xfe, 0x00]).unwrap();
        fs::write(root.join("huge.css"), vec![b'a'; (baseline::MAX_BASELINE_FILE_BYTES + 1) as usize]).unwrap();

        // Default (no paths): UI sources only, dependencies/build output excluded and reported
        let snap = baseline::create_baseline(&root, "auto", "test", &[]).unwrap();
        let keys: Vec<&str> = snap.files.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, vec!["DESIGN.md", "index.html", "src/components/Card.tsx", "templates/index.json"]);
        let skipped: Vec<(&str, &str)> = snap.skipped.iter().map(|s| (s.path.as_str(), s.reason.as_str())).collect();
        assert!(skipped.contains(&("node_modules", "excluded directory")), "{:?}", skipped);
        assert!(skipped.contains(&("dist", "excluded directory")), "{:?}", skipped);
        assert!(skipped.contains(&("bad.css", "not valid UTF-8")), "{:?}", skipped);
        assert!(skipped.iter().any(|(p, r)| *p == "huge.css" && r.starts_with("larger than")), "{:?}", skipped);
        assert_eq!(snap.skipped_total, snap.skipped.len());

        // Rollback restores every captured file
        fs::write(root.join("src/components/Card.tsx"), "broken").unwrap();
        let rb = baseline::rollback_baseline(&root, "auto").unwrap();
        assert!(rb.success);
        assert_eq!(rb.restored_files.len(), 4);
        assert_eq!(fs::read_to_string(root.join("src/components/Card.tsx")).unwrap(), "export const Card = 1;");

        // Repeatable explicit paths: a directory expands, missing / non-UTF-8 / oversized files are errors
        let snap = baseline::create_baseline(&root, "ex", "test", &["src".into(), "index.html".into()]).unwrap();
        assert_eq!(snap.files.len(), 2);
        assert!(baseline::create_baseline(&root, "ex", "test", &["missing.html".into()]).is_err());
        assert!(baseline::create_baseline(&root, "ex", "test", &["bad.css".into()]).is_err());
        assert!(baseline::create_baseline(&root, "ex", "test", &["huge.css".into()]).is_err());

        // A legacy empty baseline must not report a successful rollback
        fs::write(
            root.join(".layoza/baselines/empty.json"),
            r#"{"id":"empty","reason":"legacy","fingerprint":"0","files":{}}"#,
        )
        .unwrap();
        let rb = baseline::rollback_baseline(&root, "empty").unwrap();
        assert!(!rb.success);
        assert!(rb.restored_files.is_empty());

        // A project with no UI sources cannot produce an empty baseline
        let empty = root.join("no-ui");
        fs::create_dir_all(&empty).unwrap();
        fs::write(empty.join("notes.txt"), "plain text").unwrap();
        assert!(baseline::create_baseline(&empty, "none", "test", &[]).is_err());

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_safe_path_rejects_nonexistent_paths_outside_root() {
        let base = std::env::temp_dir().join("layoza_test_safe_path");
        let _ = fs::remove_dir_all(&base);
        let root = base.join("project");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(base.join("outside")).unwrap();

        // Non-existent absolute path outside the root must be rejected
        let outside = base.join("outside").join("new").join("file.html");
        assert!(baseline::validate_safe_project_path(&root, &outside).is_err());
        let sibling = base.join("project-evil").join("x.html");
        assert!(baseline::validate_safe_project_path(&root, &sibling).is_err());

        // Non-existent paths inside the root resolve under the canonical root
        let ok = baseline::validate_safe_project_path(&root, std::path::Path::new("qa/new/report.json")).unwrap();
        assert!(ok.ends_with("qa/new/report.json"));
        let canon_root = root.canonicalize().unwrap();
        let canon_root = canon_root.to_string_lossy();
        let canon_root = canon_root.trim_start_matches(r"\\?\");
        assert!(ok.to_string_lossy().starts_with(canon_root), "{}", ok.display());
        let abs_inside = root.join("deep").join("a.css");
        assert!(baseline::validate_safe_project_path(&root, &abs_inside).is_ok());

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn test_industry_slug_queries_match_directly() {
        for (query, slug) in [
            ("saas-startup", "saas-startup"),
            ("saas_startup", "saas-startup"),
            ("finance-fintech", "finance-fintech"),
            ("ai-devtools", "ai-devtools"),
            ("a supply-chain platform for manufacturing", "industrial-b2b"),
        ] {
            let rec = intelligence::recommend_design(query, intelligence::OperatingMode::NewWebsite, None, None);
            assert_eq!(rec.matched_industry.slug, slug, "query `{}`", query);
            assert_ne!(rec.match_confidence, "none", "query `{}`", query);
        }
    }

    #[test]
    fn test_auto_repair_and_benchmark_suite() {
        let temp_dir = std::env::temp_dir().join("layoza_test_repair");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let broken_html = temp_dir.join("broken.html");
        fs::write(
            &broken_html,
            "<!DOCTYPE html><html><head><style>@keyframes spin { to { transform: rotate(360deg); } }</style></head><body><h1>Test</h1></body></html>",
        )
        .unwrap();

        let before = verifier::audit_project(&temp_dir, None, None, None, None).unwrap();
        assert!(before.objective_error_count >= 2);

        let repair = verifier::auto_repair_html_file(&broken_html).unwrap();
        assert!(repair.modified);
        assert_eq!(repair.applied_repairs.len(), 3);

        let after = verifier::audit_project(&temp_dir, None, None, None, None).unwrap();
        assert_eq!(after.objective_error_count, 0);

        let suite = benchmark::run_benchmark_suite();
        assert_eq!(suite.total_briefs, 32);
        assert_eq!(suite.passed_briefs, 32);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_cwv_unmeasured_is_not_a_pass() {
        let dir = std::env::temp_dir().join("layoza_cwv_unmeasured");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("index.html"), "<!doctype html><html lang=\"en\"><head><meta name=\"viewport\" content=\"width=device-width\"></head><body><main><h1>Hi</h1></main></body></html>").unwrap();
        let report = verifier::audit_project(&dir, None, None, None, None).unwrap();
        assert!(!report.core_web_vitals.measured);
        assert_eq!(report.core_web_vitals.lcp_ms, None);
        assert_eq!(report.core_web_vitals.lcp_passes_target, None);
        assert!(report.findings.iter().any(|f| f.rule_id == "CWV-NOT-MEASURED"));

        let slow = verifier::audit_project(&dir, None, Some(4000.0), Some(50.0), Some(0.01)).unwrap();
        assert_eq!(slow.core_web_vitals.lcp_passes_target, Some(false));
        assert!(!slow.passed_release_gate);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_browser_report_ingest_legacy_and_v2() {
        let dir = std::env::temp_dir().join("layoza_qa_ingest");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("qa")).unwrap();
        fs::write(dir.join("index.html"), "<!doctype html><html lang=\"en\"><head><meta name=\"viewport\" content=\"width=device-width\"></head><body><main><h1>Hi</h1></main></body></html>").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));

        // Legacy v1 report (estimated values) must be ignored, not trusted
        fs::write(dir.join("qa/browser-qa-report.json"), r#"{"lcp_ms":180.0,"cls":0.0,"inp_ms":48.0,"viewports":[]}"#).unwrap();
        let legacy = verifier::audit_project(&dir, None, None, None, None).unwrap();
        assert!(legacy.findings.iter().any(|f| f.rule_id == "BROWSER-QA-LEGACY"));
        assert!(!legacy.core_web_vitals.measured);

        // v2 report: measured metrics and issues become findings
        let v2 = r#"{"schema_version":"2.0.0","lcp_ms":1200,"cls":0.02,"inp_ms":40,
            "contrast_failures":2,"min_contrast_ratio":3.1,"contrast_failure_samples":["<span> 3.10:1"],
            "missing_alt_count":0,"undersized_targets":0,"heading_hierarchy_valid":true,"landmarks_present":true,"focus_visible_styled":true,
            "viewports":[{"label":"mobile","issues":[{"rule":"nav-unreachable","severity":"error","detail":"hidden nav"},
                                                   {"rule":"label-wrap","severity":"warning","detail":"a"},{"rule":"label-wrap","severity":"warning","detail":"b"}],
                          "console_errors":[]}]}"#;
        fs::write(dir.join("qa/browser-qa-report.json"), v2).unwrap();
        let r = verifier::audit_project(&dir, None, None, None, None).unwrap();
        assert!(r.core_web_vitals.measured);
        assert_eq!(r.core_web_vitals.lcp_ms, Some(1200.0));
        assert!(r.findings.iter().any(|f| f.rule_id == "A11Y-CONTRAST" && f.severity == verifier::Severity::Error));
        assert!(r.findings.iter().any(|f| f.rule_id == "BROWSER-NAV-UNREACHABLE"));
        // Repeated issues of one rule in one viewport collapse into a single finding
        assert_eq!(r.findings.iter().filter(|f| f.rule_id == "BROWSER-LABEL-WRAP").count(), 1);
        assert!(!r.accessibility.passes_automated_wcag22_aa);
        assert!(!r.passed_release_gate);

        // A source edit after the report makes it stale
        std::thread::sleep(std::time::Duration::from_millis(1100));
        fs::write(dir.join("index.html"), "<!doctype html><html lang=\"en\"><head><meta name=\"viewport\" content=\"width=device-width\"></head><body><main><h1>Edited</h1></main></body></html>").unwrap();
        let stale = verifier::audit_project(&dir, None, None, None, None).unwrap();
        assert!(stale.findings.iter().any(|f| f.rule_id == "BROWSER-QA-STALE"));
        assert!(!stale.findings.iter().any(|f| f.rule_id == "A11Y-CONTRAST"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_craft_critique_issues_are_non_blocking_subjective_findings() {
        // Ported impeccable rules arrive as `critique` issues; they become Info-level
        // SubjectiveCritique findings, grouped per (rule, viewport), with the upstream description.
        let report: serde_json::Value = serde_json::from_str(
            r#"{"schema_version":"2.0.0","viewports":[
                {"label":"desktop","issues":[
                    {"rule":"craft-cream-palette","severity":"critique","detail":"cream/beige page background rgb(243, 237, 227)"},
                    {"rule":"craft-side-tab","severity":"critique","detail":"border-left: 4px + border-radius: 12px"},
                    {"rule":"craft-side-tab","severity":"critique","detail":"border-right: 4px"},
                    {"rule":"label-wrap","severity":"warning","detail":"a"}],"console_errors":[]},
                {"label":"mobile","issues":[
                    {"rule":"craft-side-tab","severity":"critique","detail":"border-left: 4px"},
                    {"rule":"craft-not-in-registry","severity":"critique","detail":"x"}],"console_errors":[]}]}"#,
        )
        .unwrap();
        let findings = verifier::browser_report_findings(&report, "qa/browser-qa-report.json");
        let craft: Vec<_> = findings.iter().filter(|f| f.category == "design-critique").collect();
        assert_eq!(craft.len(), 4, "{:#?}", craft);
        assert!(craft
            .iter()
            .all(|f| f.kind == verifier::FindingKind::SubjectiveCritique && f.severity == verifier::Severity::Info));
        assert_eq!(craft.iter().filter(|f| f.rule_id == "CRAFT-SIDE-TAB").count(), 2);
        let side_tab_desktop = craft
            .iter()
            .find(|f| f.rule_id == "CRAFT-SIDE-TAB" && f.file_or_selector.contains("desktop"))
            .unwrap();
        assert!(side_tab_desktop.message.contains("(+1 more)"));
        assert!(side_tab_desktop.remediation.starts_with("Thick colored border on one side of a card"));
        let cream = craft.iter().find(|f| f.rule_id == "CRAFT-CREAM-PALETTE").unwrap();
        assert!(cream.remediation.contains("warm cream or beige page background"));
        assert!(cream.remediation.contains("layoza-brand-allow"));
        assert!(craft.iter().any(|f| f.rule_id == "CRAFT-NOT-IN-REGISTRY"));
        // The objective warning in the same viewport keeps its own path
        assert!(findings.iter().any(|f| f.rule_id == "BROWSER-LABEL-WRAP"
            && f.kind == verifier::FindingKind::DeterministicObjective
            && f.severity == verifier::Severity::Warning));
    }

    #[test]
    fn test_craft_critique_does_not_change_release_gate() {
        let dir = std::env::temp_dir().join("layoza_craft_gate");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("qa")).unwrap();
        fs::write(dir.join("index.html"), "<!doctype html><html lang=\"en\"><head><meta name=\"viewport\" content=\"width=device-width\"></head><body><main><h1>Hi</h1></main></body></html>").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));

        let base = r#""schema_version":"2.0.0","lcp_ms":900,"cls":0.01,"inp_ms":40,
            "contrast_failures":0,"missing_alt_count":0,"undersized_targets":0,
            "heading_hierarchy_valid":true,"landmarks_present":true,"focus_visible_styled":true"#;
        let clean = format!(r#"{{{base},"viewports":[{{"label":"desktop","issues":[],"console_errors":[]}}]}}"#);
        fs::write(dir.join("qa/browser-qa-report.json"), &clean).unwrap();
        let without = verifier::audit_project(&dir, None, None, None, None).unwrap();

        let critiqued = format!(
            r#"{{{base},"craft_critique":{{"craft-overused-font":1,"craft-nested-cards":2}},"viewports":[
                {{"label":"desktop","issues":[
                    {{"rule":"craft-overused-font","severity":"critique","detail":"Primary font: inter (90% of text)"}},
                    {{"rule":"craft-nested-cards","severity":"critique","detail":"Card inside card"}},
                    {{"rule":"craft-nested-cards","severity":"critique","detail":"Card inside card"}}],"console_errors":[]}},
                {{"label":"mobile","issues":[
                    {{"rule":"craft-nested-cards","severity":"critique","detail":"Card inside card"}}],"console_errors":[]}}]}}"#
        );
        fs::write(dir.join("qa/browser-qa-report.json"), &critiqued).unwrap();
        let with = verifier::audit_project(&dir, None, None, None, None).unwrap();

        assert!(without.passed_release_gate, "{:#?}", without.findings);
        assert_eq!(with.passed_release_gate, without.passed_release_gate);
        assert_eq!(with.objective_error_count, without.objective_error_count);
        assert_eq!(with.objective_warning_count, without.objective_warning_count);
        assert_eq!(with.accessibility.automated_violations, without.accessibility.automated_violations);
        assert_eq!(with.subjective_critique_notes, without.subjective_critique_notes + 3);
        // Critique never shows up as a before/after regression either
        let cmp = verifier::verify_before_after(&without, &with, 0.0, vec![]);
        assert!(cmp.unintended_regressions.is_empty(), "{:?}", cmp.unintended_regressions);
        assert!(cmp.accepted);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_before_after_flags_new_error_rules() {
        let dir = std::env::temp_dir().join("layoza_before_after");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("index.html"), "<!doctype html><html lang=\"en\"><head><meta name=\"viewport\" content=\"width=device-width\"></head><body></body></html>").unwrap();
        let before = verifier::audit_project(&dir, None, Some(1000.0), Some(50.0), Some(0.01)).unwrap();
        fs::write(dir.join("index.html"), "<!doctype html><html><head><meta name=\"viewport\" content=\"width=device-width\"></head><body><img src=\"a.png\"></body></html>").unwrap();
        let after = verifier::audit_project(&dir, None, Some(1000.0), Some(50.0), Some(0.3)).unwrap();
        let cmp = verifier::verify_before_after(&before, &after, 0.0, vec![]);
        assert!(!cmp.accepted);
        assert!(cmp.unintended_regressions.iter().any(|m| m.contains("A11Y-HTML-LANG")));
        assert!(cmp.unintended_regressions.iter().any(|m| m.starts_with("CLS")));
        let same = verifier::verify_before_after(&before, &before, 0.0, vec![]);
        assert!(same.unintended_regressions.is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_industry_matching_is_whole_word_and_reports_confidence() {
        let ceramics = intelligence::recommend_design(
            "Small-batch ceramics studio in Lahore selling handmade tableware and kiln workshops",
            intelligence::OperatingMode::NewWebsite,
            None,
            None,
        );
        assert_eq!(ceramics.matched_industry.slug, "artisan-craft");
        assert_eq!(ceramics.match_confidence, "strong");
        assert_eq!(ceramics.distinctiveness.place_cues, vec!["Lahore".to_string()]);
        assert!(ceramics.distinctiveness.material_cues.contains(&"kiln".to_string()));
        assert!(!ceramics.distinctiveness.category_cliches.is_empty());

        // "detail"/"start" must not hit the `ai` / `art` keywords
        let none = intelligence::recommend_design("A detailed start page", intelligence::OperatingMode::NewWebsite, None, None);
        assert_eq!(none.match_confidence, "none");
        assert_ne!(none.matched_industry.slug, "ai-devtools");

        // Directions B and C are chosen per brief and never repeat direction A's fonts
        for rec in [&ceramics, &none] {
            let fonts: Vec<&str> = rec.directions.iter().map(|d| d.typography.display_font.as_str()).collect();
            assert_eq!(rec.directions.len(), 3);
            assert!(fonts[0] != fonts[1] && fonts[1] != fonts[2] && fonts[0] != fonts[2], "{:?}", fonts);
        }
    }

    #[test]
    fn test_palette_contrast_is_computed_not_claimed() {
        assert!((intelligence::wcag_contrast("#000000", "#FFFFFF").unwrap() - 21.0).abs() < 0.01);
        for ind in intelligence::catalog_industries() {
            let p = &ind.palette;
            let body = intelligence::wcag_contrast(&p.foreground, &p.background).unwrap();
            let muted = intelligence::wcag_contrast(&p.muted, &p.background).unwrap();
            assert!((p.wcag_body_contrast - body).abs() < 0.01, "{} reports {} but is {}", p.id, p.wcag_body_contrast, body);
            assert!(body >= 4.5 && muted >= 4.5, "{} fails AA: body {:.2}, muted {:.2}", p.id, body, muted);
        }
        let rec = intelligence::recommend_design("ceramics studio", intelligence::OperatingMode::NewWebsite, None, None);
        for d in &rec.directions {
            let body = intelligence::wcag_contrast(&d.palette.foreground, &d.palette.background).unwrap();
            assert!((d.palette.wcag_body_contrast - body).abs() < 0.01);
        }
    }

    #[test]
    fn test_every_recommended_style_exists_in_catalog() {
        let styles: Vec<String> = intelligence::catalog_styles().into_iter().map(|s| s.slug).collect();
        for ind in intelligence::catalog_industries() {
            assert!(!ind.recommended_styles.is_empty(), "{} has no styles", ind.slug);
            for st in &ind.recommended_styles {
                assert!(styles.contains(st), "{} recommends unknown style {}", ind.slug, st);
            }
        }
        let bike = intelligence::recommend_design(
            "Cooperative bicycle repair workshop with a small coffee bar in Rotterdam",
            intelligence::OperatingMode::NewWebsite,
            None,
            None,
        );
        assert_eq!(bike.matched_industry.slug, "local-business");
        assert!(bike.matched_styles.iter().all(|s| s.slug != "precision-terminal-craft"), "`ai` must not match `repair`");
    }

    #[test]
    fn test_installer_never_writes_cargo_run_and_merges_opencode() {
        let dir = std::env::temp_dir().join("layoza_install_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("opencode.json"),
            r#"{"model":"keep-me","mcp":{"other":{"type":"local","command":["x"]}}}"#,
        )
        .unwrap();
        fs::create_dir_all(dir.join(".codex")).unwrap();
        fs::write(
            dir.join(".codex/config.toml"),
            "model = \"keep-me\"\n\n[mcp_servers.layoza]\ncommand = \"old\"\n\n[mcp_servers.other]\ncommand = \"x\"\n",
        )
        .unwrap();
        fs::create_dir_all(dir.join(".claude/skills/my-own-skill")).unwrap();
        fs::write(dir.join(".claude/skills/my-own-skill/SKILL.md"), "mine").unwrap();
        let agents: Vec<String> = ["claude", "cursor", "opencode", "codex"].iter().map(|s| s.to_string()).collect();
        installer::install_layoza(&dir, installer::InstallScope::Project, &agents).unwrap();
        installer::install_layoza(&dir, installer::InstallScope::Project, &agents).unwrap();

        for rel in [".claude/skills", ".agents/skills"] {
            for name in installer::REQUIRED_SKILL_NAMES {
                assert!(dir.join(rel).join(name).join("SKILL.md").is_file(), "{}/{} not installed", rel, name);
            }
        }
        assert_eq!(fs::read_to_string(dir.join(".claude/skills/my-own-skill/SKILL.md")).unwrap(), "mine");
        assert_eq!(installer::REQUIRED_SKILL_NAMES.len(), fs::read_dir("../../skills").unwrap().count(), "every bundled skill must be required");

        let codex = fs::read_to_string(dir.join(".codex/config.toml")).unwrap();
        assert!(codex.contains("model = \"keep-me\"") && codex.contains("[mcp_servers.other]"), "codex config lost: {}", codex);
        assert_eq!(codex.matches("[mcp_servers.layoza]").count(), 1, "layoza table duplicated: {}", codex);
        assert!(!codex.contains("\"old\""), "stale layoza entry kept: {}", codex);

        for f in [".mcp.json", ".cursor/mcp.json", "opencode.json", ".codex/config.toml"] {
            let raw = fs::read_to_string(dir.join(f)).unwrap();
            assert!(!raw.contains("\"cargo\""), "{} still launches via cargo run: {}", f, raw);
        }
        let oc: serde_json::Value = serde_json::from_str(&fs::read_to_string(dir.join("opencode.json")).unwrap()).unwrap();
        assert_eq!(oc["model"], "keep-me", "existing opencode settings must survive");
        assert!(oc["mcp"]["other"].is_object(), "existing MCP servers must survive");
        assert!(oc["mcp"]["layoza"]["command"].is_array());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_no_profile_recommends_its_own_cliche() {
        for ind in intelligence::catalog_industries() {
            if let Some(why) = intelligence::violates_own_cliches(&ind.slug, &ind.typography_pairing, &ind.palette) {
                panic!("{}", why);
            }
        }
    }
}
