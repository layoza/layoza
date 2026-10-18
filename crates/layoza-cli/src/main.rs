// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Layoza CLI (`layoza`): fast native command-line interface sharing `layoza-core` with `layoza-mcp`.

use layoza_core::{
    adapters::generate_framework_artifact,
    animation::select_animations,
    baseline::{create_baseline, rollback_baseline},
    benchmark::run_benchmark_suite,
    design_md::{diff_design_md, generate_design_md, lint_design_md},
    dna::DesignDials,
    hooks::{check_edit, hook_command, hook_status, install_hook, settings_path, uninstall_hook, HookTarget},
    inspector::{inspect_project, is_dna_baseline_fresh, write_dna_artifacts},
    installer::{install_layoza, run_doctor, InstallScope},
    intelligence::{catalog_industries, recommend_design, OperatingMode},
    mechanics::{
        catalogue, deal_mechanics_with, get_mechanic, list_mechanics, DealOptions, ExistingSite, Intensity, Mechanic,
        MechanicKind,
    },
    qa::run_browser_qa,
    verifier::{audit_project, auto_repair_html_file, browser_report_findings, Severity},
};
use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    if args.is_empty() || args[0] == "--help" || args[0] == "-h" || args[0] == "help" {
        print_help();
        return;
    }

    if args[0] == "--version" || args[0] == "-V" || args[0] == "version" {
        println!("layoza {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let json_flag = args.iter().any(|a| a == "--json");
    let subcommand = args[0].as_str();

    // `<command> --help` / `-h` prints that command's usage and never runs it
    if args[1..].iter().any(|a| a == "--help" || a == "-h") {
        match subcommand_usage(subcommand) {
            Some(usage) => {
                println!("{}", usage);
                return;
            }
            None => exit_err(&format!("Unknown command `{}`. Run `layoza --help`.", subcommand), json_flag),
        }
    }

    match subcommand {
        "init" => {
            let scope = if get_flag_value(&args, "--scope").as_deref() == Some("global") {
                InstallScope::Global
            } else {
                InstallScope::Project
            };
            let agents: Vec<String> = get_flag_value(&args, "--agents")
                .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
                .unwrap_or_default();

            match install_layoza(&cwd, scope, &agents) {
                Ok(report) => {
                    if json_flag {
                        println!("{}", serde_json::to_string_pretty(&report).unwrap());
                    } else {
                        println!("Layoza initialized ({:?} scope) in `{}`", report.scope, report.target_root);
                        println!("Configured agents: {}", report.configured_agents.join(", "));
                        for f in &report.written_files {
                            println!("  wrote: {}", f);
                        }
                    }
                }
                Err(e) => exit_err(&e, json_flag),
            }
        }
        "inspect" => {
            let target_arg = get_flag_value(&args, "--path")
                .or_else(|| positional_arg(&args, 1))
                .unwrap_or_else(|| ".".to_string());
            let target_path = cwd.join(&target_arg);
            let write_flag = args
                .iter()
                .any(|a| a == "--write" || a == "-w" || a == "--write-artifacts");
            let screenshot_only = args.iter().any(|a| a == "--screenshot-only");

            match inspect_project(&target_path, None, screenshot_only) {
                Ok((dna, inventory)) => {
                    let mut written = Vec::new();
                    if write_flag {
                        match write_dna_artifacts(&target_path, &dna, &inventory) {
                            Ok(w) => written = w,
                            Err(e) => exit_err(&e, json_flag),
                        }
                    }
                    if json_flag {
                        let out = serde_json::json!({
                            "dna": dna,
                            "inventory": inventory,
                            "written_files": written
                        });
                        println!("{}", serde_json::to_string_pretty(&out).unwrap());
                    } else {
                        println!("=== Layoza Site DNA Inspection: {} ===", dna.project_name);
                        println!("Framework        : {}", dna.framework);
                        println!("Preservation Mode: {}", dna.preservation_mode);
                        println!("Confidence Score : {:.0}%", dna.confidence.overall_score * 100.0);
                        println!("Colors Extracted : {}", dna.colors.len());
                        println!("Components Found : {}", inventory.total_components);
                        println!("Baseline Hash    : {}", dna.baseline_fingerprint);
                        if !written.is_empty() {
                            println!("Generated DNA Artifacts:");
                            for w in &written {
                                println!("  - {}", w);
                            }
                        }
                    }
                }
                Err(e) => exit_err(&e, json_flag),
            }
        }
        "design-system" => {
            if let Some(lint_file) = get_flag_value(&args, "--lint") {
                let raw = fs::read_to_string(cwd.join(&lint_file)).unwrap_or_else(|e| {
                    exit_err(&format!("Cannot read `{}`: {}", lint_file, e), json_flag);
                });
                let report = lint_design_md(&raw);
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
                if !report.valid {
                    process::exit(1);
                }
                return;
            }
            if args.iter().any(|a| a == "--diff") {
                let b_file = positional_arg(&args, 1).unwrap_or_else(|| "DESIGN.md".to_string());
                let a_file = positional_arg(&args, 2).unwrap_or_else(|| "DESIGN.md".to_string());
                let b_raw = fs::read_to_string(cwd.join(&b_file))
                    .unwrap_or_else(|e| exit_err(&e.to_string(), json_flag));
                let a_raw = fs::read_to_string(cwd.join(&a_file))
                    .unwrap_or_else(|e| exit_err(&e.to_string(), json_flag));
                let diff = diff_design_md(&b_raw, &a_raw);
                println!("{}", serde_json::to_string_pretty(&diff).unwrap());
                return;
            }

            let query = positional_arg(&args, 1).unwrap_or_else(|| "saas-startup".to_string());
            let mode = match get_flag_value(&args, "--mode").as_deref() {
                Some("extend") | Some("b") => OperatingMode::ExtendExisting,
                Some("improve") | Some("c") => OperatingMode::ImproveExisting,
                Some("redesign") | Some("d") => OperatingMode::FullRedesign,
                _ => OperatingMode::NewWebsite,
            };
            let dials = if get_flag_value(&args, "--variance").is_some()
                || get_flag_value(&args, "--motion").is_some()
                || get_flag_value(&args, "--density").is_some()
            {
                let v = get_flag_value(&args, "--variance")
                    .and_then(|s| s.parse::<u8>().ok())
                    .unwrap_or(5);
                let m = get_flag_value(&args, "--motion")
                    .and_then(|s| s.parse::<u8>().ok())
                    .unwrap_or(5);
                let d = get_flag_value(&args, "--density")
                    .and_then(|s| s.parse::<u8>().ok())
                    .unwrap_or(5);
                Some(DesignDials::clamped(v, m, d))
            } else {
                None
            };

            let existing_dna = match mode {
                OperatingMode::ExtendExisting | OperatingMode::ImproveExisting => load_site_dna(&cwd).map(|(d, _)| d),
                _ => None,
            };
            let rec = recommend_design(&query, mode, existing_dna.as_ref(), dials);
            if json_flag {
                println!("{}", serde_json::to_string_pretty(&rec).unwrap());
            } else {
                println!("=== Layoza Design System Recommendation ({:?}) ===", rec.mode);
                println!("Matched Industry : {} (`{}`)", rec.matched_industry.name, rec.matched_industry.slug);
                println!("Match Confidence : {} (score {})", rec.match_confidence, rec.match_score);
                if rec.match_confidence == "none" {
                    println!(
                        "  FALLBACK: no industry keyword or slug matched the brief; `{}` is a generic default.",
                        rec.matched_industry.slug
                    );
                    let slugs: Vec<String> = catalog_industries().into_iter().map(|i| i.slug).collect();
                    println!("  Pass an industry slug or keyword instead, e.g. one of: {}", slugs.join(", "));
                }
                if !rec.alternative_industries.is_empty() {
                    let alts: Vec<String> =
                        rec.alternative_industries.iter().map(|(s, sc)| format!("{} ({})", s, sc)).collect();
                    println!("Alternatives     : {}", alts.join(", "));
                }
                println!("Layout Archetype : {}", rec.matched_industry.layout_archetype);
                println!(
                    "Typography       : {} / {} (Mono: {})",
                    rec.matched_industry.typography_pairing.display_font,
                    rec.matched_industry.typography_pairing.body_font,
                    rec.matched_industry.typography_pairing.mono_font
                );
                println!(
                    "Palette          : bg={} fg={} accent={} (WCAG {:.1}:1)",
                    rec.matched_industry.palette.background,
                    rec.matched_industry.palette.foreground,
                    rec.matched_industry.palette.primary_accent,
                    rec.matched_industry.palette.wcag_body_contrast
                );
                println!(
                    "Active Dials     : Variance={}/10 | Motion={}/10 | Density={}/10",
                    rec.active_dials.design_variance,
                    rec.active_dials.motion_intensity,
                    rec.active_dials.visual_density
                );
                println!("\nThree Materially Distinct Directions:");
                for dir in &rec.directions {
                    println!("  [{}] {}", dir.index, dir.title);
                    println!("      Layout  : {}", dir.layout_composition);
                    println!("      Identity: {}", dir.visual_identity);
                    println!("      Motion  : {}", dir.motion_choreography);
                    if !dir.mechanics.is_empty() {
                        let m: Vec<String> =
                            dir.mechanics.iter().map(|m| format!("{} ({}, {})", m.id, m.role, m.intensity.as_str())).collect();
                        println!("      Mechanics: {}", m.join(" | "));
                    }
                }
                println!("\nMechanics: `layoza mechanics show <id>` prints a blueprint; `layoza mechanics deal \"<brief>\"` deals a full page.");
            }
        }
        "animate" => {
            let category = positional_arg(&args, 1);
            let intensity = get_flag_value(&args, "--intensity")
                .and_then(|s| s.parse::<u8>().ok())
                .unwrap_or(6);
            let recipe = get_flag_value(&args, "--recipe");
            let recipes = select_animations(category.as_deref(), recipe.as_deref(), intensity)
                .unwrap_or_else(|e| exit_err(&e, json_flag));
            if json_flag {
                println!("{}", serde_json::to_string_pretty(&recipes).unwrap());
            } else {
                println!("=== Layoza Animation & 3D Recipes (Intensity {}/10) ===", intensity);
                for r in &recipes {
                    println!("- [{}] {} (engine: `{}`)", r.id, r.name, r.engine);
                    println!("  Reduced-Motion: {}", r.reduced_motion_strategy);
                    println!("  Cleanup       : {}", r.cleanup_contract);
                }
            }
        }
        "audit" => {
            let target_arg = positional_arg(&args, 1).unwrap_or_else(|| ".".to_string());
            let target_path = cwd.join(&target_arg);
            match audit_project(&target_path, None, None, None, None) {
                Ok(report) => {
                    if json_flag {
                        println!("{}", serde_json::to_string_pretty(&report).unwrap());
                    } else {
                        println!("=== Layoza Design & Quality Audit: {} ===", report.target_path);
                        println!("Release Gate           : {}", gate_label(&report));
                        println!("Objective Errors       : {}", report.objective_error_count);
                        println!("Objective Warnings     : {}", report.objective_warning_count);
                        println!("Subjective Critique    : {} (non-blocking)", report.subjective_critique_notes);
                        println!(
                            "WCAG 2.2 AA Automated  : {}",
                            if report.accessibility.passes_automated_wcag22_aa { "PASS" } else { "FAIL" }
                        );
                        let cwv = &report.core_web_vitals;
                        let fmt = |v: Option<f64>, digits: usize, unit: &str| {
                            v.map(|x| format!("{:.*}{}", digits, x, unit)).unwrap_or_else(|| "not measured".to_string())
                        };
                        println!(
                            "Lab Core Web Vitals    : LCP={}, INP={}, CLS={} ({})",
                            fmt(cwv.lcp_ms, 0, "ms"),
                            fmt(cwv.inp_or_tbt_ms, 0, "ms"),
                            fmt(cwv.cls, 3, ""),
                            cwv.methodology_source
                        );
                        for f in &report.findings {
                            println!("  [{:?}][{}] {}: {}", f.severity, f.rule_id, f.file_or_selector, f.message);
                        }
                    }
                    exit_for_gate(&report);
                }
                Err(e) => exit_err(&e, json_flag),
            }
        }
        "verify" => {
            let target_arg = positional_arg(&args, 1).unwrap_or_else(|| ".".to_string());
            let target_path = cwd.join(&target_arg);
            let repair_flag = args.iter().any(|a| a == "--repair" || a == "--fix");

            let mut repairs = Vec::new();
            if repair_flag && target_path.is_file() {
                match auto_repair_html_file(&target_path) {
                    Ok(r) => repairs.push(r),
                    Err(e) => exit_err(&e, json_flag),
                }
            }
            match audit_project(&target_path, None, None, None, None) {
                Ok(report) => {
                    if json_flag {
                        let out = serde_json::json!({
                            "verified": report.passed_release_gate,
                            "verification_status": report.verification_status,
                            "repairs": repairs,
                            "audit": report
                        });
                        println!("{}", serde_json::to_string_pretty(&out).unwrap());
                    } else {
                        println!("=== Layoza Verification Gate ===");
                        if !repairs.is_empty() {
                            for r in &repairs {
                                println!("Auto-Repair ({}): {}", r.file_path, r.diff_summary);
                            }
                        }
                        println!("Release Gate: {}", gate_label(&report));
                        for f in &report.findings {
                            println!("  [{:?}][{}] {}: {}", f.severity, f.rule_id, f.file_or_selector, f.message);
                        }
                    }
                    exit_for_gate(&report);
                }
                Err(e) => exit_err(&e, json_flag),
            }
        }
        "qa" => {
            let target_arg = positional_arg(&args, 1).unwrap_or_else(|| "index.html".to_string());
            let is_url = target_arg.starts_with("http://") || target_arg.starts_with("https://");
            let target_path = cwd.join(&target_arg);
            let qa_dir = if !is_url && target_path.is_file() {
                target_path.parent().unwrap_or(&cwd).join("qa")
            } else if !is_url && target_path.is_dir() {
                target_path.join("qa")
            } else {
                cwd.join(".layoza/screenshots")
            };
            let out_json = qa_dir.join("browser-qa-report.json");
            let run_target = if is_url {
                target_arg.clone()
            } else if target_path.is_dir() {
                target_path.join("index.html").to_string_lossy().to_string()
            } else {
                target_path.to_string_lossy().to_string()
            };
            let live = match run_browser_qa(&cwd, &run_target, &out_json, &qa_dir) {
                Ok(v) => v,
                Err(e) => exit_err(&e, json_flag),
            };
            if is_url {
                let findings = browser_report_findings(&live, &target_arg);
                let errors = findings.iter().filter(|f| f.severity == Severity::Error).count();
                if json_flag {
                    println!("{}", serde_json::to_string_pretty(&serde_json::json!({ "report": live, "findings": findings })).unwrap());
                } else {
                    println!("=== Layoza Live Chromium QA: {} ===", target_arg);
                    println!("Screenshots Saved      : {}", qa_dir.display());
                    println!(
                        "Lab Core Web Vitals    : LCP={} INP~{} CLS={}",
                        live["lcp_ms"], live["inp_ms"], live["cls"]
                    );
                    println!("Min Contrast Ratio     : {}", live["min_contrast_ratio"]);
                    println!("Objective Errors       : {}", errors);
                    for f in &findings {
                        println!("  [{:?}][{}] {}: {}", f.severity, f.rule_id, f.file_or_selector, f.message);
                    }
                }
                if errors > 0 {
                    process::exit(1);
                }
                return;
            }
            let audit_target = if !is_url && target_path.is_file() {
                target_path.parent().unwrap_or(&cwd).to_path_buf()
            } else {
                target_path.clone()
            };
            match audit_project(&audit_target, None, None, None, None) {
                Ok(report) => {
                    if json_flag {
                        println!("{}", serde_json::to_string_pretty(&report).unwrap());
                    } else {
                        println!("=== Layoza Closed-Loop Chromium Visual & DOM QA ===");
                        println!("Target                 : {}", report.target_path);
                        println!("Screenshots Saved      : {}", qa_dir.display());
                        println!("  - desktop.png          (1440×900)");
                        println!("  - tablet.png           (768×1024)");
                        println!("  - mobile.png           (375×812 full-page)");
                        println!("  - mobile-hero.png      (375×812 1:1 header/hero)");
                        println!("  - <viewport>-menu-open.png (only when a menu toggle exists)");
                        let cwv = &report.core_web_vitals;
                        let fmt = |v: Option<f64>, digits: usize, unit: &str| {
                            v.map(|x| format!("{:.*}{}", digits, x, unit)).unwrap_or_else(|| "not measured".to_string())
                        };
                        println!(
                            "Lab Core Web Vitals    : LCP={} INP~{} CLS={}",
                            fmt(cwv.lcp_ms, 0, "ms"),
                            fmt(cwv.inp_or_tbt_ms, 0, "ms"),
                            fmt(cwv.cls, 3, "")
                        );
                        println!("Release Gate           : {}", gate_label(&report));
                        println!("Objective Errors       : {}", report.objective_error_count);
                        println!("Objective Warnings     : {}", report.objective_warning_count);
                        for f in &report.findings {
                            println!("  [{:?}][{}] {}: {}", f.severity, f.rule_id, f.file_or_selector, f.message);
                        }
                    }
                    exit_for_gate(&report);
                }
                Err(e) => exit_err(&e, json_flag),
            }
        }
        "doctor" => {
            let doc = run_doctor(&cwd);
            if json_flag {
                println!("{}", serde_json::to_string_pretty(&doc).unwrap());
            } else {
                println!("=== Layoza Doctor ===");
                println!("OS / Arch          : {} / {}", doc.os, doc.arch);
                println!("Python Required    : {} (100% Native Rust Core)", doc.python_required);
                println!("Node.js Available  : {}", doc.node_available);
                println!("Skills Validated   : {}/{}", doc.skill_count, layoza_core::installer::REQUIRED_SKILL_NAMES.len());
                println!("Smoke Test Passed  : {}", doc.smoke_test_passed);
                for a in &doc.agents {
                    println!(
                        "  Agent `{:<12}`: detected={}, skills_installed={}, mcp_configured={}",
                        a.agent, a.detected, a.skills_installed, a.mcp_configured
                    );
                }
                if !doc.issues.is_empty() {
                    println!("Issues:");
                    for issue in &doc.issues {
                        println!("  ! {}", issue);
                    }
                }
            }
            if !doc.smoke_test_passed {
                process::exit(1);
            }
        }
        "generate" => {
            let fw = get_flag_value(&args, "--framework").unwrap_or_else(|| "html-css-js".to_string());
            let title = get_flag_value(&args, "--title").unwrap_or_else(|| "Precision Design Engineering".to_string());
            let subtitle = get_flag_value(&args, "--subtitle")
                .unwrap_or_else(|| "Verified design quality, Site DNA preservation, and real-browser QA.".to_string());
            let cta = get_flag_value(&args, "--cta").unwrap_or_else(|| "Get Started".to_string());
            let out_dir = get_flag_value(&args, "--output").map(|s| cwd.join(s));

            let (dna, _) = inspect_project(&cwd, None, false).unwrap_or_else(|e| exit_err(&e, json_flag));
            let artifact = generate_framework_artifact(&fw, &title, &subtitle, &cta, &dna);

            if let Some(dir) = &out_dir {
                for (rel_file, content) in &artifact.files {
                    let full_p = dir.join(rel_file);
                    if let Some(parent) = full_p.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    fs::write(&full_p, content).unwrap_or_else(|e| exit_err(&e.to_string(), json_flag));
                }
                let md = generate_design_md(&dna);
                let _ = fs::write(dir.join("DESIGN.md"), md);
            }

            if json_flag {
                println!("{}", serde_json::to_string_pretty(&artifact).unwrap());
            } else {
                println!("Generated `{}` artifact ({} file(s))", artifact.framework, artifact.files.len());
                for k in artifact.files.keys() {
                    println!("  - {}", k);
                }
            }
        }
        "baseline" => {
            let action = positional_arg(&args, 1).unwrap_or_else(|| "freshness".to_string());
            let id = positional_arg(&args, 2).unwrap_or_else(|| "default".to_string());
            match action.as_str() {
                "create" => {
                    // No --file: snapshot the project's UI sources; --file is repeatable and strict
                    let files = get_flag_values(&args, "--file");
                    match create_baseline(&cwd, &id, "CLI baseline", &files) {
                        Ok(snap) => {
                            println!("{}", serde_json::to_string_pretty(&snap).unwrap());
                            eprintln!(
                                "Baseline `{}`: {} file(s) captured, {} path(s) skipped.",
                                snap.id,
                                snap.files.len(),
                                snap.skipped_total
                            );
                        }
                        Err(e) => exit_err(&e, json_flag),
                    }
                }
                "rollback" => match rollback_baseline(&cwd, &id) {
                    Ok(rb) => {
                        println!("{}", serde_json::to_string_pretty(&rb).unwrap());
                        if !rb.success {
                            process::exit(1);
                        }
                    }
                    Err(e) => exit_err(&e, json_flag),
                },
                _ => match is_dna_baseline_fresh(&cwd) {
                    Ok((fresh, stored, current)) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&serde_json::json!({
                                "fresh": fresh,
                                "stored_fingerprint": stored,
                                "current_fingerprint": current
                            }))
                            .unwrap()
                        );
                    }
                    Err(e) => exit_err(&e, json_flag),
                },
            }
        }
        "benchmark" => {
            let report = run_benchmark_suite();
            if json_flag {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            } else {
                println!(
                    "=== Layoza Benchmark Suite: {}/{} Passed ({:.1}%) ===",
                    report.passed_briefs, report.total_briefs, report.pass_rate_percent
                );
                for r in &report.results {
                    println!(
                        "  [{}] {:<54} | fw={:<19} | WCAG={:.1}:1 | pass={}",
                        r.id, r.title, r.framework, r.wcag_contrast_ratio, r.passed_automated_gates
                    );
                }
            }
        }
        "deal" => {
            // Deals which candidate world to build: a stable hash of the candidate list picks one of
            // the top N, so runs stop converging on the model's safest first pick (idea: impeccable's
            // concept roll; Layoza's version is offline, reproducible, and bounded to the top N).
            let text = positional_arg(&args, 1).unwrap_or_else(|| exit_err("Usage: layoza deal \"<candidate list>\" [--top N]", json_flag));
            let top: u64 = get_flag_value(&args, "--top").and_then(|v| v.parse().ok()).filter(|n| *n >= 1).unwrap_or(4);
            let mut h: u64 = 0xcbf29ce484222325;
            for b in text.trim().to_lowercase().bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
            let pick = (h % top) + 1;
            if json_flag {
                println!("{}", serde_json::json!({ "pick": pick, "of_top": top, "key": format!("{:016x}", h) }));
            } else {
                println!("Build candidate #{} (dealt from your top {}; key {:016x}).", pick, top, h);
            }
        }
        "mechanics" => {
            let action = positional_arg(&args, 1).unwrap_or_else(|| "list".to_string());
            let kind = get_flag_value(&args, "--kind").map(|k| {
                MechanicKind::parse(&k)
                    .unwrap_or_else(|| exit_err(&format!("Unknown --kind `{}` (hero|section|transition).", k), json_flag))
            });
            let intensity = get_flag_value(&args, "--intensity").map(|i| {
                Intensity::parse(&i)
                    .unwrap_or_else(|| exit_err(&format!("Unknown --intensity `{}` (calm|bold|wild or 0-10).", i), json_flag))
            });
            match action.as_str() {
                "list" => {
                    let niche = get_flag_value(&args, "--niche");
                    let list = list_mechanics(niche.as_deref(), kind, intensity);
                    if json_flag {
                        println!("{}", serde_json::to_string_pretty(&list).unwrap());
                    } else {
                        println!("{} of {} mechanics", list.len(), catalogue().len());
                        for m in list {
                            println!(
                                "  {:<40} {:<10} {:<11} {:<5} {:<5} {}",
                                m.id,
                                format!("{:?}", m.kind).to_lowercase(),
                                m.role,
                                m.intensity.as_str(),
                                m.tier.as_str(),
                                m.name
                            );
                        }
                    }
                }
                "show" => {
                    let id = positional_arg(&args, 2)
                        .unwrap_or_else(|| exit_err("Usage: layoza mechanics show <id>", json_flag));
                    let m = get_mechanic(&id).unwrap_or_else(|| {
                        exit_err(&format!("Unknown mechanic `{}`. Run `layoza mechanics list`.", id), json_flag)
                    });
                    if json_flag {
                        println!("{}", serde_json::to_string_pretty(m).unwrap());
                    } else {
                        print_mechanic(m, "");
                    }
                }
                "deal" => {
                    let brief = positional_arg(&args, 2).unwrap_or_else(|| {
                        exit_err("Usage: layoza mechanics deal \"<brief>\" [--count N] [--seed S]", json_flag)
                    });
                    let count = match get_flag_value(&args, "--count") {
                        Some(c) => c
                            .parse::<usize>()
                            .ok()
                            .filter(|n| (2..=10).contains(n))
                            .unwrap_or_else(|| exit_err("--count must be 2-10", json_flag)),
                        None => 5,
                    };
                    let seed = get_flag_value(&args, "--seed").map(|s| {
                        s.parse::<u64>().unwrap_or_else(|_| exit_err("--seed must be a non-negative integer", json_flag))
                    });
                    let existing_mode =
                        matches!(get_flag_value(&args, "--mode").as_deref(), Some("extend" | "improve" | "b" | "c"));
                    let existing = match get_flag_value(&args, "--existing") {
                        Some(p) => Some(existing_profile(&cwd.join(&p), json_flag)),
                        None if existing_mode => Some(existing_profile(&cwd, json_flag)),
                        None => None,
                    };
                    let deal = deal_mechanics_with(&DealOptions {
                        brief,
                        niche: get_flag_value(&args, "--niche"),
                        mood: get_flag_value(&args, "--mood"),
                        intensity,
                        count,
                        seed,
                        existing,
                        ..Default::default()
                    });
                    if json_flag {
                        println!("{}", serde_json::to_string_pretty(&deal).unwrap());
                    } else {
                        let c = &deal.context;
                        let or_dash = |v: &Vec<String>| if v.is_empty() { "-".to_string() } else { v.join(", ") };
                        println!("=== Layoza mechanics deal ({} site) ===", c.mode);
                        println!(
                            "Niches: {} | Moods: {} | Intensity: {} ({}) | Max tier: {} | Seed: {}",
                            or_dash(&c.niches),
                            or_dash(&c.moods),
                            c.intensity.as_str(),
                            c.intensity_source,
                            c.max_tier.as_str(),
                            c.seed
                        );
                        if let Some(h) = &deal.hero {
                            print_mechanic(h, "HERO");
                        }
                        for m in &deal.signature {
                            print_mechanic(m, "SIGNATURE");
                        }
                        for m in &deal.suggestions {
                            println!(
                                "\n[SUGGESTION] {} - {} ({}, {}): {}",
                                m.id,
                                m.name,
                                m.role,
                                m.intensity.as_str(),
                                m.still_check
                            );
                        }
                        if let Some(t) = &deal.transition {
                            println!("\n[TRANSITION] {} - {} ({})", t.id, t.name, t.role);
                        }
                        println!("\nNotes:");
                        for n in deal.notes.iter().chain(&deal.rules) {
                            println!("  - {}", n);
                        }
                    }
                }
                other => exit_err(&format!("Unknown mechanics action `{}` (list|deal|show).", other), json_flag),
            }
        }
        "hooks" => {
            let action = positional_arg(&args, 1).unwrap_or_else(|| "status".to_string());
            let target = match get_flag_value(&args, "--scope").as_deref() {
                Some("global") => HookTarget::Global,
                _ if args.iter().any(|a| a == "--local") => HookTarget::ProjectLocal,
                _ => HookTarget::Project,
            };
            let path = settings_path(&cwd, target).unwrap_or_else(|e| exit_err(&e, json_flag));
            let shown = path.to_string_lossy().replace('\\', "/");
            match action.as_str() {
                "on" => {
                    let command = hook_command();
                    match install_hook(&path, &command) {
                        Ok(change) => {
                            if json_flag {
                                println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                                    "change": change, "scope": target.label(), "settings_path": shown, "command": command
                                })).unwrap());
                            } else {
                                println!("Layoza edit hook {:?} ({} scope) in `{}`", change, target.label(), shown);
                                println!("  PostToolUse [Edit|Write|MultiEdit] -> {}", command);
                                println!("  Restart the agent session (or open /hooks) so the harness reloads settings.");
                            }
                        }
                        Err(e) => exit_err(&e, json_flag),
                    }
                }
                "off" => match uninstall_hook(&path) {
                    Ok(change) => {
                        if json_flag {
                            println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                                "change": change, "scope": target.label(), "settings_path": shown
                            })).unwrap());
                        } else {
                            println!("Layoza edit hook {:?} ({} scope) in `{}`", change, target.label(), shown);
                        }
                    }
                    Err(e) => exit_err(&e, json_flag),
                },
                "status" => {
                    let mut all = Vec::new();
                    for t in [HookTarget::Project, HookTarget::ProjectLocal, HookTarget::Global] {
                        if let Ok(p) = settings_path(&cwd, t) {
                            all.push(hook_status(&p, t));
                        }
                    }
                    if json_flag {
                        println!("{}", serde_json::to_string_pretty(&all).unwrap());
                    } else {
                        for s in &all {
                            let state = if s.installed { "ON " } else { "off" };
                            println!("  [{}] {:<13} {}", state, s.target.label(), s.settings_path);
                            if let Some(c) = &s.command {
                                println!("        command: {}", c);
                                if s.command_resolves == Some(false) {
                                    println!("        WARNING: binary not found; run `layoza hooks on` again to repair.");
                                }
                            }
                        }
                    }
                }
                other => exit_err(&format!("Unknown hooks action `{}`. Use on|off|status.", other), json_flag),
            }
        }
        "check-edit" => {
            // PostToolUse hook entry point: never blocks, never fails the agent's edit.
            let mut input = String::new();
            let _ = std::io::stdin().take(4 * 1024 * 1024).read_to_string(&mut input);
            let project_dir = env::var_os("CLAUDE_PROJECT_DIR").map(PathBuf::from);
            let now_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            let out = std::panic::catch_unwind(|| {
                check_edit(&input, project_dir.as_deref().map(Path::new), &cwd, now_ms)
            })
            .ok()
            .flatten();
            if let Some(o) = out {
                println!("{}", o);
            }
            process::exit(0);
        }
        "mcp" => {
            let mut mcp_root = cwd.clone();
            for ancestor in cwd.ancestors() {
                if ancestor.file_name().and_then(|n| n.to_str()) == Some(".agents") {
                    if let Some(parent) = ancestor.parent() {
                        mcp_root = parent.to_path_buf();
                        break;
                    }
                }
            }
            if let Err(e) = layoza_mcp::run_stdio_server(&mcp_root) {
                eprintln!("Layoza MCP server error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            exit_err(&format!("Unknown command `{}`. Run `layoza --help`.", subcommand), json_flag);
        }
    }
}

fn get_flag_value(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == flag {
            return args.get(i + 1).cloned();
        }
        if let Some(rest) = args[i].strip_prefix(&format!("{}=", flag)) {
            return Some(rest.to_string());
        }
    }
    None
}

/// Every flag that takes a value. `positional_arg` skips the token after any of these, and a unit
/// test checks that every flag read through `get_flag_value(s)` is listed here.
const VALUE_FLAGS: &[&str] = &[
    "--scope", "--agents", "--mode", "--variance", "--motion", "--density", "--lint", "--intensity", "--recipe",
    "--framework", "--title", "--subtitle", "--cta", "--output", "--file", "--snapshot", "--path", "--top",
    "--niche", "--kind", "--count", "--seed", "--existing", "--mood",
];

/// All values of a repeatable flag (`--file a --file b`, `--file=a`).
fn get_flag_values(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
            }
            i += 2;
            continue;
        }
        if let Some(rest) = args[i].strip_prefix(&format!("{}=", flag)) {
            out.push(rest.to_string());
        }
        i += 1;
    }
    out
}

/// The `idx`-th positional argument (0 = the subcommand), skipping flags and the values of
/// `VALUE_FLAGS` (`--top 2`, `--file x`; `--flag=value` is a single token).
fn positional_arg(args: &[String], idx: usize) -> Option<String> {
    let mut pos = Vec::new();
    let mut skip_next = false;
    for (i, a) in args.iter().enumerate() {
        if i == 0 {
            pos.push(a.clone());
            continue;
        }
        if skip_next {
            skip_next = false;
            continue;
        }
        if VALUE_FLAGS.contains(&a.as_str()) {
            skip_next = true;
            continue;
        }
        if !a.starts_with('-') {
            pos.push(a.clone());
        }
    }
    pos.get(idx).cloned()
}

/// Per-command usage for `layoza <command> --help`.
fn subcommand_usage(cmd: &str) -> Option<&'static str> {
    Some(match cmd {
        "init" => "Usage: layoza init [--scope project|global] [--agents a,b,c] [--json]\n  Install Layoza skills, plugin and MCP configs for the selected agents.",
        "inspect" => "Usage: layoza inspect [<path>|--path <path>] [--write] [--screenshot-only] [--json]\n  Extract Site DNA from a project directory (default `.`); --write writes DESIGN.md, BRAND.md, MOTION.md, SITE-DNA.json, COMPONENT-INVENTORY.json.",
        "design-system" => "Usage: layoza design-system \"<brief or industry slug>\" [--mode new|extend|improve|redesign] [--variance N] [--motion N] [--density N] [--json]\n       layoza design-system --lint <DESIGN.md>\n       layoza design-system --diff <before.md> <after.md>\n  Recommend an industry profile, dials and 3 creative directions; prints match confidence and alternatives.",
        "animate" => "Usage: layoza animate [<category|id|engine>] [--recipe <id>] [--intensity 0-10] [--json]\n  Recommend animation recipes. Categories: hero_animation, section_animation, scroll_animation, interactive_effect, page_transition, three_experience. --recipe selects one recipe by id.",
        "audit" => "Usage: layoza audit [<path>] [--json]\n  Deterministic accessibility, layout, motion and Core Web Vitals checks; exits 1 if the release gate fails.",
        "verify" => "Usage: layoza verify [<path>] [--repair] [--json]\n  Verify release gates; --repair applies bounded auto-repairs to a single HTML file.",
        "qa" => "Usage: layoza qa [<url|file|dir>] [--json]\n  Headless Chromium multi-viewport QA (default index.html). Overall timeout: LAYOZA_QA_TIMEOUT_SECS (default 180).",
        "doctor" => "Usage: layoza doctor [--json]\n  Validate agent installations, skill frontmatter, runtimes and smoke tests.",
        "generate" => "Usage: layoza generate [--framework <fw>] [--title <t>] [--subtitle <s>] [--cta <c>] [--output <dir>] [--json]\n  Generate a framework-native component from the project's Site DNA.",
        "baseline" => "Usage: layoza baseline create [<id>] [--file <path>]...\n       layoza baseline rollback [<id>]\n       layoza baseline [freshness]\n  create without --file snapshots the project's UI sources (html/css/js/ts/tsx/jsx/vue/svelte/astro/liquid/php, theme JSON, DESIGN.md...), skipping node_modules, .git, target, dist, build, .layoza and hidden dirs (caps: 2000 files, 1 MB/file, 32 MB total); skipped paths are reported. --file is repeatable and errors on paths that cannot be snapshotted. rollback exits 1 if it restores nothing.",
        "benchmark" => "Usage: layoza benchmark [--json]\n  Run the 32 reproducible evaluation briefs.",
        "deal" => "Usage: layoza deal \"<candidate list>\" [--top N] [--json]\n  Deterministically pick one of the top N (default 4) candidates from a stable hash of the list.",
        "mechanics" => "Usage: layoza mechanics list [--niche X] [--kind hero|section|transition] [--intensity calm|bold|wild] [--json]\n       layoza mechanics deal \"<brief>\" [--count N] [--seed S] [--niche X] [--mood m] [--intensity calm|bold|wild] [--mode extend|improve] [--existing <dir>] [--json]\n       layoza mechanics show <id> [--json]\n  Deal 1 hero + N section mechanics (2 signature + suggestions) + 1 transition, deterministic per seed. New sites follow the brief; --existing <dir> or --mode extend|improve caps the deal to the inspected site's motion intensity and stack (SITE-DNA.json, else a fresh inspect).",
        "hooks" => "Usage: layoza hooks on|off|status [--scope project|global] [--local] [--json]\n  Manage the Claude Code PostToolUse hook that checks UI files after each edit.",
        "check-edit" => "Usage: layoza check-edit < hook.json\n  Hook entry point: reads PostToolUse JSON on stdin and reports findings; never blocks.",
        "mcp" => "Usage: layoza mcp\n  Start the Layoza MCP stdio server.",
        _ => return None,
    })
}

/// Reads `<dir>/SITE-DNA.json` (written by `layoza inspect --write`), or the file itself, if it parses.
fn load_site_dna(dir: &Path) -> Option<(layoza_core::dna::SiteDna, String)> {
    let p = if dir.is_file() { dir.to_path_buf() } else { dir.join("SITE-DNA.json") };
    let raw = fs::read_to_string(&p).ok()?;
    serde_json::from_str(&raw).ok().map(|d| (d, "SITE-DNA.json".to_string()))
}

/// Motion profile of an existing site: SITE-DNA.json when present, else a fresh inspection.
fn existing_profile(dir: &Path, json_flag: bool) -> ExistingSite {
    if let Some((dna, src)) = load_site_dna(dir) {
        return ExistingSite::from_dna(&dna, &src);
    }
    match inspect_project(dir, None, false) {
        Ok((dna, _)) => ExistingSite::from_dna(&dna, "inspect"),
        Err(e) => exit_err(&format!("Cannot inspect existing site `{}`: {}", dir.display(), e), json_flag),
    }
}

fn print_mechanic(m: &Mechanic, label: &str) {
    let tag = if label.is_empty() { String::new() } else { format!("[{}] ", label) };
    println!(
        "\n{}{} - {} ({:?} / {} / {} / {})",
        tag,
        m.id,
        m.name,
        m.kind,
        m.role,
        m.intensity.as_str(),
        m.tier.as_str()
    );
    println!("  Niches    : {} | Moods: {} | Family: {}", m.niches.join(", "), m.moods.join(", "), m.family);
    println!("  Blueprint : {}", m.blueprint);
    println!("  Motion    : {}", m.motion);
    println!("  Still     : {}", m.still_check);
    println!("  Mobile    : {}", m.mobile);
    println!("  A11y      : {}", m.a11y);
    println!("  Avoid     : {}", m.avoid);
    println!("  CSS path  : {}", m.stack_notes.css);
    println!("  JS path   : {}", m.stack_notes.js);
}

fn exit_err(msg: &str, json_flag: bool) -> ! {
    if json_flag {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({ "error": msg })).unwrap()
        );
    } else {
        eprintln!("Error: {}", msg);
    }
    process::exit(1);
}

fn print_help() {
    println!(
        r#"Layoza CLI v{} — Open-Source AI-Native UI/UX Layout Expert, Site DNA & Visual QA System

USAGE:
  layoza <COMMAND> [OPTIONS]

COMMANDS:
  init            Initialize Layoza skills, Antigravity plugin, and MCP configs (--scope project|global)
  inspect         Inspect a project directory or site and extract Site DNA (--write, --json)
  design-system   Recommend design systems, 3 directions, or lint/diff DESIGN.md (--mode, --lint, --diff)
  animate         Recommend GSAP, Motion, WAAPI, Three.js/R3F, or Lenis recipes (--intensity 0-10, --recipe <id>)
  audit           Run deterministic accessibility, layout, motion, and Core Web Vitals checks
  verify          Verify release gates and optionally apply bounded auto-repairs (--repair)
  qa              Run closed-loop headless Chromium multi-viewport screenshots, mobile menu test & verify
  doctor          Validate agent installations, skill frontmatter, runtimes, and smoke tests
  generate        Generate framework-native components/themes (--framework, --output)
  baseline        Manage reversible baselines (create, rollback, freshness)
  benchmark       Run the 32 reproducible evaluation briefs
  deal            Pick one candidate from your top N by a stable hash: deal "<list>" [--top N]
  mechanics       Deal hero/section/transition mechanics for a page: mechanics list|deal|show
                  (--niche, --kind, --intensity calm|bold|wild, --count, --seed, --existing <dir>)
  hooks           Auto-check UI files after every agent edit: hooks on|off|status
                  [--scope project|global] [--local] (Claude Code PostToolUse hook)
  check-edit      Hook entry point: reads the hook JSON on stdin, statically audits the edited
                  UI file (brand-aware, rate-limited, never blocks) and reports to the agent
  mcp             Start the Layoza Model Context Protocol (MCP) stdio server

Run `layoza <COMMAND> --help` for that command's options.
"#,
        env!("CARGO_PKG_VERSION")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn positional_arg_skips_value_flags() {
        let args = a(&["deal", "--top", "2", "a, b, c"]);
        assert_eq!(positional_arg(&args, 1).as_deref(), Some("a, b, c"));
        let args = a(&["baseline", "--file", "x.html", "create", "--file=y.css", "b1"]);
        assert_eq!(positional_arg(&args, 1).as_deref(), Some("create"));
        assert_eq!(positional_arg(&args, 2).as_deref(), Some("b1"));
        assert_eq!(get_flag_values(&args, "--file"), vec!["x.html".to_string(), "y.css".to_string()]);
        let args = a(&["animate", "--recipe", "hero-gsap-masked-split", "--intensity", "7"]);
        assert_eq!(positional_arg(&args, 1), None);
        // Boolean flags do not swallow the next positional
        let args = a(&["inspect", "--write", "site"]);
        assert_eq!(positional_arg(&args, 1).as_deref(), Some("site"));
    }

    #[test]
    fn every_flag_read_with_a_value_is_a_value_flag() {
        let src = include_str!("main.rs");
        for marker in ["get_flag_value(&args, \"", "get_flag_values(&args, \""] {
            for chunk in src.split(marker).skip(1) {
                let flag = &chunk[..chunk.find('"').unwrap()];
                assert!(VALUE_FLAGS.contains(&flag), "`{}` takes a value but is missing from VALUE_FLAGS", flag);
            }
        }
    }

    #[test]
    fn every_command_has_usage() {
        for cmd in [
            "init", "inspect", "design-system", "animate", "audit", "verify", "qa", "doctor", "generate", "baseline",
            "benchmark", "deal", "mechanics", "hooks", "check-edit", "mcp",
        ] {
            assert!(subcommand_usage(cmd).is_some(), "{}", cmd);
        }
    }
}

/// Human-readable release gate status.
fn gate_label(report: &layoza_core::verifier::AuditReport) -> String {
    match report.verification_status.as_str() {
        "passed" => "PASSED (defects only; this gate cannot judge whether the design is good)".to_string(),
        "unverified" => "UNVERIFIED (no objective errors, but Core Web Vitals were not measured; run `layoza qa` first)".to_string(),
        _ => "FAILED".to_string(),
    }
}

/// Exit code for the release gate: 0 passed, 1 failed, 2 unverified.
fn exit_for_gate(report: &layoza_core::verifier::AuditReport) {
    match report.verification_status.as_str() {
        "passed" => {}
        "unverified" => process::exit(2),
        _ => process::exit(1),
    }
}
