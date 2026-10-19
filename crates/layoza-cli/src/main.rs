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
    baseline::{new_run_id, record_looks, RECENT_LOOK_RUNS},
    intelligence::{catalog_industries, looks_of, recommend_design_avoiding, OperatingMode},
    mechanics::{
        catalogue, deal_mechanics_with, get_mechanic, list_mechanics, DealOptions, ExistingSite, Intensity, Mechanic,
        MechanicKind,
    },
    qa::{compare_look_fingerprints, run_browser_qa},
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

    // Every flag is honoured or rejected: unknown flags, missing values and bad numbers are errors.
    if subcommand != "check-edit" {
        if let Err(e) = check_flags(&args) {
            exit_err(&e, json_flag);
        }
    }

    match subcommand {
        "init" => {
            let scope = match get_flag_value(&args, "--scope").as_deref() {
                Some("global") => InstallScope::Global,
                None | Some("project") => InstallScope::Project,
                Some(other) => exit_err(&format!("--scope must be project or global (got `{}`).", other), json_flag),
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
            let export_skill = args.iter().any(|a| a == "--export-skill");
            if export_skill && !write_flag {
                exit_err("--export-skill writes a file, so it needs --write.", json_flag);
            }

            match inspect_project(&target_path, None, screenshot_only) {
                Ok((dna, inventory)) => {
                    let mut written = Vec::new();
                    if write_flag {
                        match write_dna_artifacts(&target_path, &dna, &inventory) {
                            Ok(w) => written = w,
                            Err(e) => exit_err(&e, json_flag),
                        }
                        if export_skill {
                            // A per-project skill so any later session can load this look again.
                            let slug: String = dna.project_name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
                            let rel = format!(".claude/skills/{}-look/SKILL.md", slug.trim_matches('-'));
                            let full = target_path.join(&rel);
                            if let Some(dir) = full.parent() {
                                fs::create_dir_all(dir).unwrap_or_else(|e| exit_err(&e.to_string(), json_flag));
                            }
                            fs::write(&full, layoza_core::design_md::generate_project_skill_md(&dna).as_bytes()).unwrap_or_else(|e| exit_err(&e.to_string(), json_flag));
                            written.push(rel);
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

            let query = positional_arg(&args, 1).filter(|q| !q.trim().is_empty()).unwrap_or_else(|| {
                exit_err("Missing brief. Usage: layoza design-system \"<brief or industry slug>\" [--mode new|extend|improve|redesign]", json_flag)
            });
            if query.chars().count() > MAX_BRIEF_CHARS {
                exit_err(&format!("The brief is limited to {} characters.", MAX_BRIEF_CHARS), json_flag);
            }
            let mode = match get_flag_value(&args, "--mode").as_deref() {
                None | Some("new") | Some("a") => OperatingMode::NewWebsite,
                Some("extend") | Some("b") => OperatingMode::ExtendExisting,
                Some("improve") | Some("c") => OperatingMode::ImproveExisting,
                Some("redesign") | Some("d") => OperatingMode::FullRedesign,
                Some(other) => exit_err(&format!("--mode must be new, extend, improve or redesign (got `{}`).", other), json_flag),
            };
            let dial = |flag: &str| num_flag::<u8>(&args, flag, 0, 10, json_flag);
            let (v, m, d) = (dial("--variance"), dial("--motion"), dial("--density"));
            let dials = if v.is_some() || m.is_some() || d.is_some() {
                Some(DesignDials::clamped(v.unwrap_or(5), m.unwrap_or(5), d.unwrap_or(5)))
            } else {
                None
            };

            let existing_dna = match mode {
                OperatingMode::ExtendExisting | OperatingMode::ImproveExisting => load_site_dna(&cwd).map(|(d, _)| d),
                _ => None,
            };
            let variant = num_flag::<u64>(&args, "--variant", 0, 1000, json_flag).unwrap_or(0);
            // Look ledger: only for new looks, only inside a project, and not with --no-ledger.
            let new_look = matches!(mode, OperatingMode::NewWebsite | OperatingMode::FullRedesign);
            let ledger_root = (new_look && !args.iter().any(|a| a == "--no-ledger") && looks_like_project(&cwd)).then(|| cwd.clone());
            let recent = ledger_root.as_deref().map(|r| layoza_core::process::recent_looks_or_stamps(r, RECENT_LOOK_RUNS)).unwrap_or_default();
            // Asked for the same look again: keep it and change the knobs instead of rotating away.
            let steer: &[layoza_core::baseline::Look] = if layoza_core::process::same_look_requested(&query) { &[] } else { &recent };
            let mut rec = recommend_design_avoiding(&query, mode, existing_dna.as_ref(), dials, variant, steer);
            layoza_core::process::apply_project_memory(&mut rec, ledger_root.as_deref(), &recent);
            if let Some(root) = &ledger_root {
                if let Err(e) = record_looks(root, &looks_of(&rec, &new_run_id(&query))) {
                    eprintln!("Warning: look ledger not updated: {}", e);
                }
            }
            // New looks only: an existing site follows its own design, so no avoid list.
            let avoid = matches!(mode, OperatingMode::NewWebsite | OperatingMode::FullRedesign)
                .then(layoza_core::history::avoid_recent);
            if json_flag {
                let mut out = serde_json::to_value(&rec).unwrap();
                if let Some(a) = &avoid {
                    out["avoid_recent"] = a.clone();
                }
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                println!("=== Layoza Design System Recommendation ({:?}) ===", rec.mode);
                println!("Matched Industry : {} (`{}`)", rec.matched_industry.name, rec.matched_industry.slug);
                println!("Match Confidence : {} (score {})", rec.match_confidence, rec.match_score);
                if rec.match_confidence == "none" {
                    println!(
                        "  FALLBACK: no catalogue industry matched; `{}` was read from the brief's own words. Check it against what the user meant.",
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
                    "Industry baseline: type {} / {} (Mono: {}); palette {} bg={} fg={} accent={} (WCAG {:.2}:1)",
                    rec.matched_industry.typography_pairing.display_font,
                    rec.matched_industry.typography_pairing.body_font,
                    rec.matched_industry.typography_pairing.mono_font,
                    rec.matched_industry.palette.id,
                    rec.matched_industry.palette.background,
                    rec.matched_industry.palette.foreground,
                    rec.matched_industry.palette.primary_accent,
                    rec.matched_industry.palette.wcag_body_contrast
                );
                let letters = ['A', 'B', 'C'];
                let per: Vec<String> = rec
                    .directions
                    .iter()
                    .enumerate()
                    .map(|(i, d)| format!("{} {} / {}", letters.get(i).copied().unwrap_or('?'), d.typography.display_font, d.typography.body_font))
                    .collect();
                println!("Direction type   : {}", per.join(" | "));
                if !rec.brief_signals.brief_colors.is_empty() {
                    println!(
                        "Brief Colours    : {} (the brief's own palette wins; direction palettes are fallbacks)",
                        rec.brief_signals.brief_colors.join(", ")
                    );
                }
                if !rec.brief_signals.brief_fonts.is_empty() {
                    println!(
                        "Brief Fonts      : {} (the brief's own type wins; direction pairings are fallbacks)",
                        rec.brief_signals.brief_fonts.join(", ")
                    );
                }
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
                    println!(
                        "      Type    : {} / {} / {} (scale {})",
                        dir.typography.display_font, dir.typography.body_font, dir.typography.mono_font, dir.typography.scale_ratio
                    );
                    println!(
                        "      Palette : {} bg={} fg={} accent={} (body WCAG {:.2}:1, computed)",
                        dir.palette.id, dir.palette.background, dir.palette.foreground, dir.palette.primary_accent, dir.palette.wcag_body_contrast
                    );
                    println!("      Content : {}", dir.content_treatment);
                    println!("      Motion  : {}", dir.motion_choreography);
                    if !dir.mechanics.is_empty() {
                        let m: Vec<String> = dir
                            .mechanics
                            .iter()
                            .map(|m| {
                                let comp = if m.composition.is_empty() { String::new() } else { format!(", {}", m.composition) };
                                format!("{} ({}, {}{})", m.id, m.role, m.intensity.as_str(), comp)
                            })
                            .collect();
                        println!("      Mechanics: {}", m.join(" | "));
                    }
                    if !dir.rationale.is_empty() {
                        println!("      Why     : {}", dir.rationale);
                    }
                }
                if let Some(a) = avoid.as_ref().filter(|a| a["entries"].as_u64().unwrap_or(0) > 0) {
                    let list = |k: &str| -> String {
                        a[k].as_array()
                            .map(|v| v.iter().filter_map(|x| x.as_str().map(str::to_string)).collect::<Vec<_>>().join(", "))
                            .unwrap_or_default()
                    };
                    println!("\nAvoid recent ({} picks in your design history):", a["entries"]);
                    for d in a["directions"].as_array().into_iter().flatten() {
                        println!("  - {} ({})", d["direction"].as_str().unwrap_or(""), d["date"].as_str().unwrap_or(""));
                    }
                    println!("  Fonts: {}", list("fonts"));
                    println!("  Motifs: {}", list("motifs"));
                    println!("  Make all three directions materially different unless the user asks for the same look.");
                }
                println!("\nMechanics: `layoza mechanics show <id>` prints a blueprint; `layoza mechanics deal \"<brief>\"` deals a full page.");
            }
        }
        "animate" => {
            let category = positional_arg(&args, 1);
            let intensity = num_flag::<u8>(&args, "--intensity", 0, 10, json_flag).unwrap_or(6);
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
                        println!("WCAG 2.2 AA Automated  : {}", wcag_line(&report.accessibility));
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
            let gate_root = if target_path.is_file() { target_path.parent().unwrap_or(&cwd).to_path_buf() } else { target_path.clone() };
            let gates = layoza_core::process::check_gates(&gate_root);
            match audit_project(&target_path, None, None, None, None) {
                Ok(report) => {
                    if json_flag {
                        let out = serde_json::json!({
                            "verified": report.passed_release_gate && gates.is_empty(),
                            "verification_status": report.verification_status,
                            "process_gates": gates,
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
                        if !gates.is_empty() {
                            println!("Process gates: {} unmet", gates.len());
                            for g in &gates {
                                println!("  [{}] {}: {} Fix: {}", g.rule, g.file, g.message, g.fix);
                            }
                        }
                    }
                    if !gates.is_empty() {
                        process::exit(1);
                    }
                    exit_for_gate(&report);
                }
                Err(e) => exit_err(&e, json_flag),
            }
        }
        "qa" => {
            // `qa --compare <a.json> <b.json>`: re-skin verdict between two saved QA reports; exit 2 on a re-skin.
            if let Some(i) = args.iter().position(|a| a == "--compare") {
                let (Some(a), Some(b)) = (args.get(i + 1), args.get(i + 2)) else {
                    exit_err("`qa --compare` needs two report files: layoza qa --compare <a.json> <b.json>", json_flag)
                };
                let verdict = compare_qa_reports(&cwd.join(a), &cwd.join(b)).unwrap_or_else(|e| exit_err(&e, json_flag));
                let v = verdict["verdict"].as_str().unwrap_or("unmeasured").to_string();
                if json_flag {
                    println!("{}", serde_json::to_string_pretty(&verdict).unwrap());
                } else {
                    println!("=== Layoza Look Compare: {} vs {} ===", a, b);
                    println!("Verdict                : {}", v);
                    println!("Layout similarity      : {}", verdict["layout_similarity"]);
                    println!("Palette distance       : {}", verdict["palette_distance"]);
                    if let Some(r) = verdict["reason"].as_str() {
                        println!("Reason                 : {}", r);
                    }
                    if v == "re-skin" {
                        println!("Same skeleton in new colours: this is a re-skin, not a new direction.");
                    }
                }
                if v == "re-skin" {
                    process::exit(2);
                }
                return;
            }
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
                "structure" => match layoza_core::process::redesign_structure_check(&cwd, &id) {
                    Ok(checks) => {
                        let same = checks.iter().filter(|c| c.same_structure).count();
                        println!("{}", serde_json::to_string_pretty(&serde_json::json!({ "files": checks, "same_structure": same, "max_similarity": layoza_core::process::REDESIGN_MAX_SIMILARITY })).unwrap());
                        if same > 0 {
                            eprintln!("{} file(s) kept the old page skeleton; a redesign must change the structural fingerprint.", same);
                            process::exit(1);
                        }
                    }
                    Err(e) => exit_err(&e, json_flag),
                },
                "freshness" => match is_dna_baseline_fresh(&cwd) {
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
                other => exit_err(&format!("Unknown baseline action `{}`: use create, rollback, structure or freshness.", other), json_flag),
            }
        }
        "benchmark" => {
            let report = run_benchmark_suite();
            if json_flag {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            } else {
                println!(
                    "=== Layoza pipeline regression checks: {}/{} passed ({:.1}%) — not a design-quality score ===",
                    report.passed_briefs, report.total_briefs, report.pass_rate_percent
                );
                for r in &report.results {
                    println!(
                        "  [{}] {:<54} | fw={:<19} | WCAG={:.1}:1 | pass={}",
                        r.id, r.title, r.framework, r.wcag_contrast_ratio, r.passed_automated_gates
                    );
                }
            }
            if report.passed_briefs < report.total_briefs {
                std::process::exit(1);
            }
        }
        "deal" => {
            // Deals which candidate world to build: a stable hash of the candidate list picks one of
            // the top N, so runs stop converging on the model's safest first pick (idea: impeccable's
            // concept roll; Layoza's version is offline, reproducible, and bounded to the top N).
            let text = positional_arg(&args, 1).unwrap_or_else(|| exit_err("Usage: layoza deal \"<candidate list>\" [--top N]", json_flag));
            let top: u64 = num_flag::<u64>(&args, "--top", 1, 1000, json_flag).unwrap_or(4);
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
                    let existing_mode = match get_flag_value(&args, "--mode").as_deref() {
                        None | Some("new" | "a" | "redesign" | "d") => false,
                        Some("extend" | "improve" | "b" | "c") => true,
                        Some(other) => exit_err(&format!("--mode must be new, extend, improve or redesign (got `{}`).", other), json_flag),
                    };
                    if brief.trim().is_empty() || brief.chars().count() > MAX_BRIEF_CHARS {
                        exit_err(&format!("The brief must be 1-{} characters.", MAX_BRIEF_CHARS), json_flag);
                    }
                    let existing = match get_flag_value(&args, "--existing") {
                        Some(p) => Some(existing_profile(&cwd.join(&p), json_flag)),
                        None if existing_mode => Some(existing_profile(&cwd, json_flag)),
                        None => None,
                    };
                    // Look ledger: new looks inside a project steer away from recent heroes (not with --no-ledger).
                    let ledger_root = (!existing_mode && !args.iter().any(|a| a == "--no-ledger") && looks_like_project(&cwd)).then(|| cwd.clone());
                    let recent = ledger_root.as_deref().map(|r| layoza_core::process::recent_looks_or_stamps(r, RECENT_LOOK_RUNS)).unwrap_or_default();
                    let same_look = layoza_core::process::same_look_requested(&brief);
                    let recent_ids: Vec<String> = if same_look { Vec::new() } else { recent.iter().flat_map(|l| l.hero.iter().chain(l.mechanics.iter()).cloned()).collect() };
                    let run_brief = brief.clone();
                    let deal = deal_mechanics_with(&DealOptions {
                        recent_ids,
                        brief,
                        niche: get_flag_value(&args, "--niche"),
                        mood: get_flag_value(&args, "--mood"),
                        intensity,
                        count,
                        seed,
                        existing,
                        exclude_ids: get_flag_value(&args, "--avoid")
                            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
                            .unwrap_or_default(),
                        ..Default::default()
                    });
                    let rotation = ledger_root.as_ref().map(|_| {
                        let picking: Vec<String> = deal.hero.iter().map(|h| h.id.clone()).collect();
                        layoza_core::process::rotation_statement(&recent, &picking, same_look)
                    });
                    if let Some(root) = &ledger_root {
                        let look = layoza_core::baseline::Look {
                            run: new_run_id(&run_brief),
                            source: "deal".into(),
                            hero: deal.hero.as_ref().map(|h| h.id.clone()),
                            mechanics: deal.signature.iter().map(|m| m.id.clone()).collect(),
                            ..Default::default()
                        };
                        if let Err(e) = record_looks(root, &[look]) {
                            eprintln!("Warning: look ledger not updated: {}", e);
                        }
                    }
                    if json_flag {
                        let mut v = serde_json::to_value(&deal).unwrap();
                        if let Some(r) = &rotation {
                            v["rotation"] = serde_json::json!(r);
                        }
                        println!("{}", serde_json::to_string_pretty(&v).unwrap());
                    } else {
                        if let Some(r) = &rotation {
                            println!("Rotation: {}", r);
                        }
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
                        if !deal.fit.is_empty() {
                            println!("\nFit:");
                            for f in &deal.fit {
                                println!("  - {} [{} / {}]: {}", f.id, f.slot, f.fit, f.reason);
                            }
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
                None | Some("project") if args.iter().any(|a| a == "--local") => HookTarget::ProjectLocal,
                None | Some("project") => HookTarget::Project,
                Some(other) => exit_err(&format!("--scope must be project or global (got `{}`).", other), json_flag),
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
/// The WCAG line never says PASS on a static scan: it names what the verdict rests on.
fn wcag_line(a: &layoza_core::verifier::AccessibilityReport) -> String {
    if a.evidence_basis.is_empty() || a.evidence_basis.starts_with("static checks only") {
        "unverified (static checks only)".to_string()
    } else {
        format!("{} ({})", if a.passes_automated_wcag22_aa { "pass" } else { "fail" }, a.evidence_basis)
    }
}

/// Reads two saved QA reports (or bare look fingerprints) and compares their looks.
fn compare_qa_reports(a: &Path, b: &Path) -> Result<serde_json::Value, String> {
    let read = |p: &Path| -> Result<serde_json::Value, String> {
        let text = fs::read_to_string(p).map_err(|e| format!("cannot read {}: {}", p.display(), e))?;
        let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("{} is not JSON: {}", p.display(), e))?;
        Ok(v.get("look_fingerprint").cloned().unwrap_or(v))
    };
    Ok(compare_look_fingerprints(&read(a)?, &read(b)?))
}

const VALUE_FLAGS: &[&str] = &[
    "--scope", "--agents", "--mode", "--variance", "--motion", "--density", "--lint", "--intensity", "--recipe",
    "--framework", "--title", "--subtitle", "--cta", "--output", "--file", "--snapshot", "--path", "--top",
    "--niche", "--kind", "--count", "--seed", "--existing", "--mood", "--avoid", "--variant",
];

/// Longest brief accepted by the CLI and the MCP server (characters).
const MAX_BRIEF_CHARS: usize = 2000;

/// Flags each command (or `mechanics <action>`) honours, besides `--json` and `--help`.
fn allowed_flags(args: &[String]) -> &'static [&'static str] {
    match args[0].as_str() {
        "init" => &["--scope", "--agents"],
        "inspect" => &["--path", "--write", "-w", "--write-artifacts", "--screenshot-only", "--export-skill"],
        "design-system" => &["--lint", "--diff", "--mode", "--variance", "--motion", "--density", "--variant", "--no-ledger"],
        "animate" => &["--recipe", "--intensity"],
        "verify" => &["--repair", "--fix"],
        "generate" => &["--framework", "--title", "--subtitle", "--cta", "--output"],
        "baseline" => &["--file"],
        "deal" => &["--top"],
        "mechanics" => match positional_arg(args, 1).as_deref() {
            Some("deal") => &["--count", "--seed", "--niche", "--mood", "--intensity", "--mode", "--existing", "--avoid", "--no-ledger"],
            Some("show") => &[],
            _ => &["--niche", "--kind", "--intensity"],
        },
        "hooks" => &["--scope", "--local"],
        _ => &[],
    }
}

/// Rejects flags the command does not honour and value flags given without a value, so nothing
/// the user typed is silently dropped.
fn check_flags(args: &[String]) -> Result<(), String> {
    let allowed = allowed_flags(args);
    let mut i = 1;
    while i < args.len() {
        let a = args[i].as_str();
        if a.len() > 1 && a.starts_with('-') && a.parse::<f64>().is_err() {
            let name = a.split('=').next().unwrap_or(a);
            if !(name == "--json" || allowed.contains(&name)) {
                return Err(format!("Unknown option `{}` for `layoza {}`. Run `layoza {} --help`.", name, args[0], args[0]));
            }
            if VALUE_FLAGS.contains(&name) && !a.contains('=') {
                match args.get(i + 1) {
                    Some(v) if !v.starts_with("--") => i += 1,
                    _ => return Err(format!("`{}` needs a value.", name)),
                }
            }
        }
        i += 1;
    }
    Ok(())
}

/// A numeric flag in `[min, max]`; a non-number or out-of-range value is an error, never a default.
fn num_flag<T: std::str::FromStr + PartialOrd + std::fmt::Display + Copy>(args: &[String], flag: &str, min: T, max: T, json_flag: bool) -> Option<T> {
    let raw = get_flag_value(args, flag)?;
    match raw.trim().parse::<T>() {
        Ok(n) if n >= min && n <= max => Some(n),
        _ => exit_err(&format!("{} must be a number from {} to {} (got `{}`).", flag, min, max, raw), json_flag),
    }
}

/// True when `dir` looks like a project (so the look ledger may be written there).
fn looks_like_project(dir: &Path) -> bool {
    [".layoza", "package.json", "index.html", ".git", "Cargo.toml", "composer.json", "theme.json"].iter().any(|m| dir.join(m).exists())
}

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
        "inspect" => "Usage: layoza inspect [<path>|--path <path>] [--write [--export-skill]] [--screenshot-only] [--json]\n  Extract Site DNA from a project directory (default `.`); --write writes DESIGN.md (the nine standard sections, then motion, variants, brand and decisions), BRAND.md, MOTION.md, SITE-DNA.json, COMPONENT-INVENTORY.json; --export-skill also writes .claude/skills/<project>-look/SKILL.md so later sessions load this look.",
        "design-system" => "Usage: layoza design-system \"<brief or industry slug>\" [--mode new|extend|improve|redesign] [--variance N] [--motion N] [--density N] [--variant N] [--no-ledger] [--json]\n       layoza design-system --lint <DESIGN.md>\n       layoza design-system --diff <before.md> <after.md>\n  Recommend an industry profile, dials and 3 creative directions; prints match confidence and alternatives; --variant N re-deals fonts, palettes and mechanics for the same brief. In a project (package.json, index.html, .git, .layoza...), new/redesign runs record their looks in .layoza/looks.jsonl and steer away from the last 5 runs; --no-ledger skips both.",
        "animate" => "Usage: layoza animate [<category|id|engine>] [--recipe <id>] [--intensity 0-10] [--json]\n  Recommend animation recipes. Categories: hero_animation, section_animation, scroll_animation, interactive_effect, page_transition, three_experience. --recipe selects one recipe by id.",
        "audit" => "Usage: layoza audit [<path>] [--json]\n  Deterministic accessibility, layout, motion and Core Web Vitals checks; exits 1 if the release gate fails.",
        "verify" => "Usage: layoza verify [<path>] [--repair] [--json]\n  Verify release gates and the process gate files in .layoza/ (direction-approved.md, brand-spec.md, product-facts.md, self-critique stamp); exits 1 when any fails. --repair applies bounded auto-repairs to a single HTML file.",
        "qa" => "Usage: layoza qa [<url|file|dir>] [--json] | layoza qa --compare <a.json> <b.json>\n  Headless Chromium multi-viewport QA (default index.html). --compare reads two saved browser-qa-report.json files and exits 2 on a re-skin (same skeleton, new colours). Overall timeout: LAYOZA_QA_TIMEOUT_SECS (default 600).",
        "doctor" => "Usage: layoza doctor [--json]\n  Validate agent installations, skill frontmatter, runtimes and smoke tests.",
        "generate" => "Usage: layoza generate [--framework <fw>] [--title <t>] [--subtitle <s>] [--cta <c>] [--output <dir>] [--json]\n  Generate a framework-native component from the project's Site DNA.",
        "baseline" => "Usage: layoza baseline create [<id>] [--file <path>]...\n       layoza baseline rollback [<id>]\n       layoza baseline structure [<id>]\n       layoza baseline [freshness]\n  structure compares each page's skeleton with the baseline and exits 1 when a redesign kept it.\n  create without --file snapshots the project's UI sources (html/css/js/ts/tsx/jsx/vue/svelte/astro/liquid/php, theme JSON, DESIGN.md...), skipping node_modules, .git, target, dist, build, .layoza and hidden dirs (caps: 2000 files, 1 MB/file, 32 MB total); skipped paths are reported. --file is repeatable and errors on paths that cannot be snapshotted. rollback exits 1 if it restores nothing.",
        "benchmark" => "Usage: layoza benchmark [--json]\n  Run 32 pipeline regression checks (no browser; not a design-quality score).",
        "deal" => "Usage: layoza deal \"<candidate list>\" [--top N] [--json]\n  Deterministically pick one of the top N (default 4) candidates from a stable hash of the list.",
        "mechanics" => "Usage: layoza mechanics list [--niche X] [--kind hero|section|transition] [--intensity calm|bold|wild] [--json]\n       layoza mechanics deal \"<brief>\" [--count N] [--seed S] [--niche X] [--mood m] [--intensity calm|bold|wild] [--mode extend|improve] [--existing <dir>] [--avoid id1,id2] [--no-ledger] [--json]\n       layoza mechanics show <id> [--json]\n  Deal 1 hero + N section mechanics (2 signature + suggestions) + 1 transition, deterministic per seed. New sites follow the brief; --existing <dir> or --mode extend|improve caps the deal to the inspected site's motion intensity and stack (SITE-DNA.json, else a fresh inspect). --avoid skips mechanics used recently.",
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
  benchmark       Run 32 pipeline regression checks (not a quality score)
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
    fn wcag_line_never_passes_a_static_scan() {
        let mut acc = layoza_core::verifier::AccessibilityReport {
            target_standard: "WCAG 2.2 Level AA".into(),
            automated_violations: 0,
            passes_automated_wcag22_aa: false,
            manual_review_required: Vec::new(),
            evidence_basis: "static checks only: no fresh browser QA report".into(),
            needs_browser: Vec::new(),
        };
        assert_eq!(wcag_line(&acc), "unverified (static checks only)");
        acc.evidence_basis = "browser report (qa/browser-qa-report.json) + static checks".into();
        acc.passes_automated_wcag22_aa = true;
        assert!(wcag_line(&acc).starts_with("pass (browser report"));
    }

    #[test]
    fn qa_compare_reads_reports_and_names_a_reskin() {
        let dir = std::env::temp_dir().join("layoza_cli_compare_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let fp = |l: f64, c: f64| serde_json::json!({ "look_fingerprint": { "layout": { "text": [1.0, l, 0.0], "media": [0.0, 1.0, 0.0] }, "palette": [{ "oklab": [c, 0.1, 0.0], "share": 1.0 }] } });
        fs::write(dir.join("a.json"), fp(0.5, 0.2).to_string()).unwrap();
        fs::write(dir.join("b.json"), fp(0.5, 0.9).to_string()).unwrap();
        let v = compare_qa_reports(&dir.join("a.json"), &dir.join("b.json")).unwrap();
        assert_eq!(v["verdict"], "re-skin");
        assert!(compare_qa_reports(&dir.join("a.json"), &dir.join("missing.json")).is_err());
        let _ = fs::remove_dir_all(&dir);
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
    fn unknown_flags_and_missing_values_are_rejected() {
        assert!(check_flags(&a(&["design-system", "bakery", "--mode", "new", "--no-ledger", "--json"])).is_ok());
        assert!(check_flags(&a(&["design-system", "bakery", "--varaint", "2"])).unwrap_err().contains("--varaint"));
        assert!(check_flags(&a(&["design-system", "bakery", "--mode"])).unwrap_err().contains("needs a value"));
        assert!(check_flags(&a(&["mechanics", "list", "--seed", "4"])).is_err(), "list ignores --seed, so it must reject it");
        assert!(check_flags(&a(&["mechanics", "deal", "bakery", "--seed=4"])).is_ok());
        assert!(check_flags(&a(&["audit", ".", "--repair"])).is_err());
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
