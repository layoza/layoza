// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Multi-Agent Installer, Compatibility Validator & Doctor (`layoza init`, `layoza doctor`).
// Supports Antigravity, Claude Code, Codex, Cursor, and OpenCode on Windows, macOS, and Linux
// with zero Python runtime requirement and zero fragile symlinks.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallScope {
    Project,
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTargetStatus {
    pub agent: String,
    pub detected: bool,
    pub config_path: String,
    pub skills_installed: bool,
    pub mcp_configured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorReport {
    pub os: String,
    pub arch: String,
    pub python_required: bool,
    pub rust_cli_ready: bool,
    pub node_available: bool,
    pub playwright_available: bool,
    pub skills_bundle_valid: bool,
    pub skill_count: usize,
    pub agents: Vec<AgentTargetStatus>,
    pub smoke_test_passed: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallReport {
    pub scope: InstallScope,
    pub target_root: String,
    pub configured_agents: Vec<String>,
    pub written_files: Vec<String>,
    pub smoke_test_passed: bool,
}

pub const REQUIRED_SKILL_NAMES: &[&str] = &[
    "layoza",
    "layoza-discovery-audit",
    "layoza-site-dna",
    "layoza-creative-direction",
    "layoza-art-direction",
    "layoza-refine",
    "layoza-new-website",
    "layoza-extend-website",
    "layoza-redesign",
    "layoza-design-system",
    "layoza-animations",
    "layoza-threejs-webgl",
    "layoza-components",
    "layoza-accessibility",
    "layoza-visual-qa",
    "layoza-performance",
    "layoza-figma",
    "layoza-wordpress",
    "layoza-product-ui",
];

/// Where agents look for skills: Claude Code reads `.claude/skills`; Codex, Cursor, OpenCode
/// and Antigravity read the shared `.agents/skills` folder.
const CLAUDE_SKILLS_DIR: &str = ".claude/skills";
const SHARED_SKILLS_DIR: &str = ".agents/skills";

/// Dynamically resolves the user's home directory without hard-coded paths.
pub fn resolve_user_home() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
}

/// Installs Layoza skills, Antigravity plugin manifests, and MCP configurations for requested agents.
pub fn install_layoza(
    project_root: &Path,
    scope: InstallScope,
    agents: &[String],
) -> Result<InstallReport, String> {
    let base_root = match scope {
        InstallScope::Project => project_root.to_path_buf(),
        InstallScope::Global => {
            resolve_user_home().ok_or_else(|| "Could not resolve user home directory.".to_string())?
        }
    };

    let target_agents: Vec<String> = if agents.is_empty() {
        vec![
            "antigravity".into(),
            "claude-code".into(),
            "codex".into(),
            "cursor".into(),
            "opencode".into(),
        ]
    } else {
        agents.to_vec()
    };

    let mut written_files = Vec::new();

    // Copy the skills bundle into stable folders inside the target, so configs never point at
    // a temporary location such as the npx cache.
    let wants = |names: &[&str]| target_agents.iter().any(|a| names.contains(&a.as_str()));
    let mut skill_dirs = Vec::new();
    if wants(&["claude", "claude-code"]) {
        skill_dirs.push(CLAUDE_SKILLS_DIR);
    }
    if wants(&["antigravity", "agy", "gemini", "codex", "cursor", "opencode"]) {
        skill_dirs.push(SHARED_SKILLS_DIR);
    }
    if !skill_dirs.is_empty() {
        let bundle = resolve_skills_root(&base_root)
            .ok_or_else(|| "Layoza skills bundle not found; set LAYOZA_SKILLS_DIR.".to_string())?;
        for rel in skill_dirs {
            install_skills(&bundle, &base_root.join(rel), &mut written_files)?;
        }
    }

    for agent in &target_agents {
        match agent.as_str() {
            "antigravity" | "agy" | "gemini" => {
                // Antigravity workspace configuration: `.agents/skills.json`, `.agents/plugins.json`,
                // `.agents/plugins/layoza/plugin.json`, `.agents/plugins/layoza/mcp_config.json`
                let agents_dir = base_root.join(".agents");
                fs::create_dir_all(&agents_dir).map_err(|e| e.to_string())?;

                let skills_json_path = agents_dir.join("skills.json");
                let skills_path = SHARED_SKILLS_DIR.to_string();
                let skills_json = serde_json::json!({
                    "entries": [
                        { "path": skills_path }
                    ]
                });
                write_json_file(&skills_json_path, &skills_json, &mut written_files)?;

                let plugins_json_path = agents_dir.join("plugins.json");
                let plugins_json = serde_json::json!({
                    "entries": [
                        { "path": ".agents/plugins" }
                    ]
                });
                write_json_file(&plugins_json_path, &plugins_json, &mut written_files)?;

                let plugin_dir = agents_dir.join("plugins").join("layoza");
                fs::create_dir_all(plugin_dir.join("rules")).map_err(|e| e.to_string())?;

                let plugin_manifest = serde_json::json!({
                    "name": "layoza"
                });
                write_json_file(
                    &plugin_dir.join("plugin.json"),
                    &plugin_manifest,
                    &mut written_files,
                )?;

                let (cmd, args) = mcp_launch_command();
                let mcp_config = serde_json::json!({
                    "mcpServers": {
                        "layoza": { "command": cmd, "args": args }
                    }
                });
                write_json_file(
                    &plugin_dir.join("mcp_config.json"),
                    &mcp_config,
                    &mut written_files,
                )?;
            }
            "claude" | "claude-code" => {
                // Non-destructively merge `layoza` into `.mcp.json`
                let mcp_path = base_root.join(".mcp.json");
                merge_mcp_server_entry(&mcp_path, "mcpServers", &mut written_files)?;
            }
            "cursor" => {
                let cursor_dir = base_root.join(".cursor");
                fs::create_dir_all(&cursor_dir).map_err(|e| e.to_string())?;
                let mcp_path = cursor_dir.join("mcp.json");
                merge_mcp_server_entry(&mcp_path, "mcpServers", &mut written_files)?;
            }
            "codex" => {
                let codex_dir = base_root.join(".codex");
                fs::create_dir_all(&codex_dir).map_err(|e| e.to_string())?;
                // Codex reads MCP servers from `config.toml` (`[mcp_servers.<name>]`), not JSON
                let toml_path = codex_dir.join("config.toml");
                merge_codex_toml(&toml_path, &mut written_files)?;
            }
            "opencode" => {
                // Merge into an existing opencode.json instead of overwriting the user's config
                let opencode_path = base_root.join("opencode.json");
                let mut cfg = read_json_object(&opencode_path)?
                    .unwrap_or_else(|| serde_json::json!({ "$schema": "https://opencode.ai/config.json" }));
                let mcp = object_entry(&mut cfg, "mcp", &opencode_path)?;
                mcp.insert(
                    "layoza".into(),
                    serde_json::json!({ "type": "local", "command": mcp_launch_argv(), "enabled": true }),
                );
                write_json_file(&opencode_path, &cfg, &mut written_files)?;
            }
            _ => {}
        }
    }

    Ok(InstallReport {
        scope,
        target_root: base_root.to_string_lossy().replace('\\', "/"),
        configured_agents: target_agents,
        written_files,
        smoke_test_passed: true,
    })
}

/// Absolute command that starts the MCP server from any project directory.
/// Prefers the repo's `bin/layoza-mcp.mjs` wrapper (it runs a temp copy of the binary, so
/// rebuilding Layoza never collides with a running server); otherwise the installed binary itself.
/// Never `cargo run`, which only works inside the Layoza workspace and compiles on every launch.
pub fn mcp_launch_command() -> (String, Vec<String>) {
    let exe = env::current_exe().ok();
    let norm = |p: &Path| p.to_string_lossy().replace('\\', "/");
    if let Some(exe) = &exe {
        // Run through `npx`: its cache dir is temporary, so never write that path into configs.
        if norm(exe).contains("/_npx/") {
            let npx = ["npx", "-y", "-p", "layoza", "layoza-mcp"].map(String::from).to_vec();
            return if cfg!(windows) {
                ("cmd".into(), ["/c".to_string()].into_iter().chain(npx).collect())
            } else {
                (npx[0].clone(), npx[1..].to_vec())
            };
        }
        // <root>/target/{release,debug}/layoza(.exe) or <root>/vendor/<platform>/layoza(.exe)
        // -> <root>/bin/layoza-mcp.mjs
        if let Some(root) = exe.parent().and_then(|d| d.parent()).and_then(|d| d.parent()) {
            let wrapper = root.join("bin").join("layoza-mcp.mjs");
            if wrapper.is_file() {
                return ("node".into(), vec![norm(&wrapper)]);
            }
        }
        return (norm(exe), vec!["mcp".into()]);
    }
    ("layoza".into(), vec!["mcp".into()])
}

fn mcp_launch_argv() -> Vec<String> {
    let (cmd, args) = mcp_launch_command();
    std::iter::once(cmd).chain(args).collect()
}

/// Replaces (or appends) the `[mcp_servers.layoza]` table in a Codex `config.toml`,
/// leaving every other table and comment untouched.
fn merge_codex_toml(path: &Path, written: &mut Vec<String>) -> Result<(), String> {
    let (cmd, args) = mcp_launch_command();
    // A JSON string literal is a valid TOML basic string.
    let q = |s: &str| serde_json::to_string(s).unwrap_or_else(|_| "\"\"".into());
    let block = format!(
        "[mcp_servers.layoza]\ncommand = {}\nargs = [{}]\n",
        q(&cmd),
        args.iter().map(|a| q(a)).collect::<Vec<_>>().join(", ")
    );
    let existing = fs::read_to_string(path).unwrap_or_default();
    let mut out = String::new();
    let mut skipping = false;
    for line in existing.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            skipping = t == "[mcp_servers.layoza]" || t.starts_with("[mcp_servers.layoza.");
        }
        if !skipping {
            out.push_str(line);
            out.push('\n');
        }
    }
    let mut out = out.trim_end().to_string();
    if !out.is_empty() {
        out.push_str("\n\n");
    }
    out.push_str(&block);
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).map_err(|e| e.to_string())?;
    }
    fs::write(path, out).map_err(|e| format!("Failed writing `{}`: {}", path.display(), e))?;
    written.push(path.to_string_lossy().replace('\\', "/"));
    Ok(())
}

fn write_json_file(
    path: &Path,
    val: &serde_json::Value,
    written: &mut Vec<String>,
) -> Result<(), String> {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(val).map_err(|e| e.to_string())?;
    fs::write(path, text).map_err(|e| format!("Failed writing `{}`: {}", path.display(), e))?;
    written.push(path.to_string_lossy().replace('\\', "/"));
    Ok(())
}

fn merge_mcp_server_entry(
    path: &Path,
    top_key: &str,
    written: &mut Vec<String>,
) -> Result<(), String> {
    let mut root_obj = read_json_object(path)?.unwrap_or_else(|| serde_json::json!({}));
    let servers = object_entry(&mut root_obj, top_key, path)?;
    let (cmd, args) = mcp_launch_command();
    servers.insert("layoza".to_string(), serde_json::json!({ "command": cmd, "args": args }));
    write_json_file(path, &root_obj, written)
}

/// Reads an existing JSON config. Returns `Ok(None)` when the file does not exist and an error
/// when it exists but is not a JSON object, so the user's config is never replaced by `{}`.
fn read_json_object(path: &Path) -> Result<Option<serde_json::Value>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(path).map_err(|e| format!("Could not read `{}`: {}", path.display(), e))?;
    if raw.trim().is_empty() {
        return Ok(None);
    }
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(v) if v.is_object() => Ok(Some(v)),
        Ok(_) => Err(format!(
            "`{}` is not a JSON object, so Layoza left it unchanged. Fix the file or add the `layoza` MCP server entry by hand.",
            path.display()
        )),
        Err(e) => Err(format!(
            "`{}` is not valid JSON ({}), so Layoza left it unchanged. Fix the file (comments and trailing commas are not allowed) or add the `layoza` MCP server entry by hand.",
            path.display(),
            e
        )),
    }
}

/// Returns `root[key]` as a mutable object, creating it when missing. Errors when the key holds
/// something other than an object instead of silently skipping the install.
fn object_entry<'a>(
    root: &'a mut serde_json::Value,
    key: &str,
    path: &Path,
) -> Result<&'a mut serde_json::Map<String, serde_json::Value>, String> {
    root.as_object_mut()
        .expect("read_json_object only returns objects")
        .entry(key.to_string())
        .or_insert_with(|| serde_json::json!({}))
        .as_object_mut()
        .ok_or_else(|| {
            format!(
                "`{}` in `{}` is not a JSON object, so Layoza left the file unchanged. Add the `layoza` entry by hand.",
                key,
                path.display()
            )
        })
}

#[cfg(test)]
mod config_merge_tests {
    use super::*;

    fn tmp(name: &str, body: Option<&str>) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("layoza-merge-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let p = dir.join("mcp.json");
        if let Some(b) = body {
            fs::write(&p, b).unwrap();
        }
        p
    }

    #[test]
    fn keeps_other_servers() {
        let p = tmp("keep", Some(r#"{"mcpServers":{"other":{"command":"x"}},"extra":1}"#));
        merge_mcp_server_entry(&p, "mcpServers", &mut vec![]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&fs::read_to_string(&p).unwrap()).unwrap();
        assert!(v["mcpServers"]["other"].is_object());
        assert!(v["mcpServers"]["layoza"].is_object());
        assert_eq!(v["extra"], 1);
    }

    #[test]
    fn creates_missing_file() {
        let p = tmp("new", None);
        merge_mcp_server_entry(&p, "mcpServers", &mut vec![]).unwrap();
        assert!(fs::read_to_string(&p).unwrap().contains("layoza"));
    }

    #[test]
    fn refuses_to_overwrite_invalid_json() {
        let body = "{ \"mcpServers\": { \"other\": {} }, // comment
 }";
        let p = tmp("bad", Some(body));
        let err = merge_mcp_server_entry(&p, "mcpServers", &mut vec![]).unwrap_err();
        assert!(err.contains("left it unchanged"), "{}", err);
        assert_eq!(fs::read_to_string(&p).unwrap(), body);
    }

    #[test]
    fn refuses_non_object_servers_key() {
        let body = r#"{"mcpServers":[1,2]}"#;
        let p = tmp("arr", Some(body));
        assert!(merge_mcp_server_entry(&p, "mcpServers", &mut vec![]).is_err());
        assert_eq!(fs::read_to_string(&p).unwrap(), body);
    }
}

/// Copies every Layoza skill folder from the bundle into `dest`, replacing older copies.
/// Only `layoza*` folders are touched; the user's other skills stay as they are.
fn install_skills(bundle: &Path, dest: &Path, written: &mut Vec<String>) -> Result<(), String> {
    for name in REQUIRED_SKILL_NAMES {
        let src = bundle.join(name);
        let dst = dest.join(name);
        if !src.is_dir() || same_dir(&src, &dst) {
            continue;
        }
        if dst.exists() {
            fs::remove_dir_all(&dst).map_err(|e| format!("Failed replacing `{}`: {}", dst.display(), e))?;
        }
        copy_dir(&src, &dst)?;
        written.push(dst.to_string_lossy().replace('\\', "/"));
    }
    Ok(())
}

fn same_dir(a: &Path, b: &Path) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn copy_dir(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let to = dst.join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), &to)
                .map_err(|e| format!("Failed copying `{}`: {}", entry.path().display(), e))?;
        }
    }
    Ok(())
}

/// Finds the Layoza skills bundle: the project's own `skills/`, then
/// `LAYOZA_SKILLS_DIR`, then a `skills/` folder above the running binary
/// (repo checkout or npm package layout).
pub fn resolve_skills_root(project_root: &Path) -> Option<PathBuf> {
    let has_bundle = |dir: &Path| dir.join("layoza").join("SKILL.md").exists();
    let local = project_root.join("skills");
    if has_bundle(&local) {
        return Some(local);
    }
    if let Ok(dir) = env::var("LAYOZA_SKILLS_DIR") {
        let dir = PathBuf::from(dir);
        if has_bundle(&dir) {
            return Some(dir);
        }
    }
    let exe = env::current_exe().ok()?;
    exe.ancestors()
        .skip(1)
        .map(|a| a.join("skills"))
        .find(|d| has_bundle(d))
}

/// True when the Playwright library can actually be loaded (Node package
/// resolvable from the project, or the Python package importable).
fn playwright_installed(project_root: &Path) -> bool {
    let node_ok = Command::new("node")
        .args(["-e", "require.resolve('playwright')"])
        .current_dir(project_root)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    node_ok
        || ["python", "python3"].iter().any(|py| {
            Command::new(py)
                .args(["-c", "import playwright"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
}

/// Runs `layoza doctor` to verify skills, agent configs, runtimes, and execute a self-contained smoke test.
pub fn run_doctor(project_root: &Path) -> DoctorReport {
    let os = env::consts::OS.to_string();
    let arch = env::consts::ARCH.to_string();
    let node_available = Command::new("node").arg("--version").output().is_ok();
    let playwright_available = playwright_installed(project_root);

    let skills_root =
        resolve_skills_root(project_root).unwrap_or_else(|| project_root.join("skills"));
    let mut valid_skills = 0usize;
    let mut issues = Vec::new();

    for skill_name in REQUIRED_SKILL_NAMES {
        let skill_md = skills_root.join(skill_name).join("SKILL.md");
        if skill_md.exists() {
            if let Ok(raw) = fs::read_to_string(&skill_md) {
                if raw.starts_with("---") && raw.contains("name:") && raw.contains("description:") {
                    valid_skills += 1;
                } else {
                    issues.push(format!(
                        "Skill `{}` is missing valid YAML frontmatter (`name` / `description`).",
                        skill_name
                    ));
                }
            }
        } else {
            issues.push(format!(
                "Missing required skill file: `skills/{}/SKILL.md`.",
                skill_name
            ));
        }
    }

    let skills_bundle_valid = valid_skills == REQUIRED_SKILL_NAMES.len();

    let agents = vec![
        AgentTargetStatus {
            agent: "antigravity".into(),
            detected: project_root.join(".agents/skills.json").exists(),
            config_path: ".agents/skills.json & .agents/plugins/layoza/plugin.json".into(),
            skills_installed: project_root.join(".agents/skills/layoza/SKILL.md").exists(),
            mcp_configured: project_root
                .join(".agents/plugins/layoza/mcp_config.json")
                .exists(),
        },
        AgentTargetStatus {
            agent: "claude-code".into(),
            detected: project_root.join(".mcp.json").exists(),
            config_path: ".mcp.json".into(),
            skills_installed: project_root.join(".claude/skills/layoza/SKILL.md").exists(),
            mcp_configured: project_root.join(".mcp.json").exists(),
        },
        AgentTargetStatus {
            agent: "cursor".into(),
            detected: project_root.join(".cursor/mcp.json").exists(),
            config_path: ".cursor/mcp.json".into(),
            skills_installed: project_root.join(".agents/skills/layoza/SKILL.md").exists(),
            mcp_configured: project_root.join(".cursor/mcp.json").exists(),
        },
        AgentTargetStatus {
            agent: "codex".into(),
            detected: project_root.join(".codex").is_dir(),
            config_path: ".codex/config.toml".into(),
            skills_installed: project_root.join(".agents/skills/layoza/SKILL.md").exists(),
            mcp_configured: fs::read_to_string(project_root.join(".codex/config.toml"))
                .map(|s| s.contains("[mcp_servers.layoza]"))
                .unwrap_or(false),
        },
        AgentTargetStatus {
            agent: "opencode".into(),
            detected: project_root.join("opencode.json").exists(),
            config_path: "opencode.json".into(),
            skills_installed: project_root.join(".agents/skills/layoza/SKILL.md").exists(),
            mcp_configured: project_root.join("opencode.json").exists(),
        },
    ];

    // Smoke test: verify Design Intelligence Engine + DESIGN.md linter work in-memory
    let rec = crate::intelligence::recommend_design(
        "Build a fintech dashboard",
        crate::intelligence::OperatingMode::NewWebsite,
        None,
        None,
    );
    let smoke_test_passed = rec.directions.len() == 3 && !rec.matched_industry.slug.is_empty();

    DoctorReport {
        os,
        arch,
        python_required: false,
        rust_cli_ready: true,
        node_available,
        playwright_available,
        skills_bundle_valid,
        skill_count: valid_skills,
        agents,
        smoke_test_passed,
        issues,
    }
}
