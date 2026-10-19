// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Agent edit hooks (`layoza hooks on|off|status`, `layoza check-edit`).
//
// `hooks on` merges a Claude Code `PostToolUse` command hook into `.claude/settings.json`
// (project), `.claude/settings.local.json` (project, machine-local) or the user settings (global).
// After every Edit/Write/MultiEdit the agent's harness pipes the hook payload into
// `layoza check-edit`, which runs Layoza's static detector on just the edited UI file and feeds a
// compact, brand-aware summary back to the model via `hookSpecificOutput.additionalContext`.
//
// Design rules:
// - Never blocks: always exit 0, never emits `decision: "block"`, swallows every internal error.
// - Fast: no browser, no project walk; one file (<= 512 KB) plus at most a few DNA lookups.
// - Brand-aware: colours from `SITE-DNA.json` / `DESIGN.md` are never flagged as off-brand.
// - Quiet: an unchanged file is not re-checked within 10 s (a changed one always is), objective
//   errors/warnings only plus <= 5 critique notes, and one real-browser QA reminder per session.
// - Opt-in rules: besides the verifier's static audit, guidelines that carry a `hook` block
//   (`guidelines.rs`) run as cheap pattern checks. Rules marked `default_on: false` run only when
//   the project lists them in `.layoza/hooks.json`; any rule can be disabled there.
// - Latency budget: the whole check targets <= 300 ms per file (configurable). Guideline rules stop
//   as soon as the budget is spent, and the summary reports the measured time and any skipped work.
//
// `.layoza/hooks.json` (all keys optional):
//   { "budget_ms": 300, "enable": ["I18N-02"], "disable": ["NAV-07"], "verifier": true }
// `LAYOZA_HOOK_BUDGET_MS` overrides `budget_ms`.

use crate::dna::SiteDna;
use crate::guidelines;
use crate::installer::resolve_user_home;
use crate::verifier::{audit_file_content, AuditFinding, FindingKind, Severity};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Marker argument carried by Layoza's hook command; `off`/`status` identify our entry by it,
/// so hooks written by the user or other tools are never touched.
pub const HOOK_MARKER: &str = "--layoza-hook";
/// Tools whose successful completion triggers `check-edit`.
pub const HOOK_MATCHER: &str = "Edit|Write|MultiEdit";
/// Seconds before the harness cancels the hook (the check itself takes milliseconds).
pub const HOOK_TIMEOUT_SECS: u64 = 10;
/// Same file re-checked within this window is skipped.
pub const RATE_LIMIT_MS: u64 = 10_000;
/// Critique notes shown per edit, at most.
pub const MAX_CRITIQUE_NOTES: usize = 5;
/// Files larger than this are skipped (generated bundles, minified CSS, etc).
pub const MAX_FILE_BYTES: u64 = 512 * 1024;
/// Default per-file latency budget for `check-edit`.
pub const DEFAULT_BUDGET_MS: u64 = 300;
/// Guideline hook findings shown per edit, at most.
pub const MAX_GUIDELINE_FINDINGS: usize = 6;

pub const UI_EXTENSIONS: &[&str] = &[
    "html", "htm", "css", "scss", "tsx", "jsx", "vue", "svelte", "astro", "php", "liquid",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HookTarget {
    /// `<project>/.claude/settings.json` (shared with the team)
    Project,
    /// `<project>/.claude/settings.local.json` (machine-local, normally gitignored)
    ProjectLocal,
    /// `~/.claude/settings.json` (every project for this user)
    Global,
}

impl HookTarget {
    pub fn label(&self) -> &'static str {
        match self {
            HookTarget::Project => "project",
            HookTarget::ProjectLocal => "project-local",
            HookTarget::Global => "global",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HookStatus {
    pub target: HookTarget,
    pub settings_path: String,
    pub installed: bool,
    pub command: Option<String>,
    /// Whether the executable referenced by the installed command still exists on disk.
    pub command_resolves: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HookChange {
    Installed,
    Updated,
    AlreadyInstalled,
    Removed,
    NotInstalled,
}

/// Resolves the Claude Code settings file for a hook target.
pub fn settings_path(project_root: &Path, target: HookTarget) -> Result<PathBuf, String> {
    match target {
        HookTarget::Project => Ok(project_root.join(".claude").join("settings.json")),
        HookTarget::ProjectLocal => Ok(project_root.join(".claude").join("settings.local.json")),
        HookTarget::Global => resolve_user_home()
            .map(|h| h.join(".claude").join("settings.json"))
            .ok_or_else(|| "Could not resolve the user home directory (USERPROFILE/HOME).".into()),
    }
}

/// Absolute shell command the hook runs. Points at the running `layoza` binary directly (a hook
/// is short-lived, so it never holds the binary open across rebuilds the way the MCP server does).
pub fn hook_command() -> String {
    let exe = env::current_exe()
        .ok()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|| "layoza".to_string());
    format!("\"{}\" check-edit {}", exe, HOOK_MARKER)
}

fn is_layoza_handler(handler: &Value) -> bool {
    handler
        .get("command")
        .and_then(Value::as_str)
        .map(|c| c.contains("check-edit") && c.contains(HOOK_MARKER))
        .unwrap_or(false)
}

fn read_settings(path: &Path) -> Result<Value, String> {
    if !path.exists() {
        return Ok(json!({}));
    }
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("Failed reading `{}`: {}", path.display(), e))?;
    let raw = raw.trim_start_matches('\u{feff}');
    if raw.trim().is_empty() {
        return Ok(json!({}));
    }
    let val: Value = serde_json::from_str(raw).map_err(|e| {
        format!(
            "`{}` is not valid JSON ({}); refusing to modify it. Fix the file and retry.",
            path.display(),
            e
        )
    })?;
    if !val.is_object() {
        return Err(format!(
            "`{}` is not a JSON object; refusing to modify it.",
            path.display()
        ));
    }
    Ok(val)
}

fn write_settings(path: &Path, val: &Value) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let mut text = serde_json::to_string_pretty(val).map_err(|e| e.to_string())?;
    text.push('\n');
    fs::write(path, text).map_err(|e| format!("Failed writing `{}`: {}", path.display(), e))
}

/// Commands of every Layoza handler currently present in `settings.hooks.PostToolUse`.
fn layoza_commands(settings: &Value) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(groups) = settings.pointer("/hooks/PostToolUse").and_then(Value::as_array) {
        for g in groups {
            if let Some(handlers) = g.get("hooks").and_then(Value::as_array) {
                for h in handlers.iter().filter(|h| is_layoza_handler(h)) {
                    if let Some(c) = h.get("command").and_then(Value::as_str) {
                        out.push(c.to_string());
                    }
                }
            }
        }
    }
    out
}

/// Removes Layoza handlers in place; drops groups left empty and the `PostToolUse` / `hooks`
/// keys only if Layoza's removal emptied them. Returns how many handlers were removed.
fn strip_layoza_handlers(settings: &mut Value) -> usize {
    let mut removed = 0;
    let Some(hooks) = settings.get_mut("hooks").and_then(Value::as_object_mut) else {
        return 0;
    };
    if let Some(groups) = hooks.get_mut("PostToolUse").and_then(Value::as_array_mut) {
        groups.retain_mut(|g| {
            let Some(handlers) = g.get_mut("hooks").and_then(Value::as_array_mut) else {
                return true;
            };
            let before = handlers.len();
            handlers.retain(|h| !is_layoza_handler(h));
            let n = before - handlers.len();
            removed += n;
            !(n > 0 && handlers.is_empty())
        });
        if removed > 0 && groups.is_empty() {
            hooks.remove("PostToolUse");
        }
    }
    if removed > 0 && hooks.is_empty() {
        if let Some(obj) = settings.as_object_mut() {
            obj.remove("hooks");
        }
    }
    removed
}

/// The exact matcher group Layoza adds under `hooks.PostToolUse`.
pub fn hook_entry(command: &str) -> Value {
    json!({
        "matcher": HOOK_MATCHER,
        "hooks": [
            { "type": "command", "command": command, "timeout": HOOK_TIMEOUT_SECS }
        ]
    })
}

/// Merges Layoza's PostToolUse hook into `settings_file`, preserving every other key and hook.
/// Idempotent: an identical entry is left alone; a stale one (old binary path) is replaced.
pub fn install_hook(settings_file: &Path, command: &str) -> Result<HookChange, String> {
    let mut settings = read_settings(settings_file)?;
    let existing = layoza_commands(&settings);
    if existing.len() == 1 && existing[0] == command {
        return Ok(HookChange::AlreadyInstalled);
    }
    let had_any = !existing.is_empty();
    strip_layoza_handlers(&mut settings);

    let obj = settings.as_object_mut().expect("settings is an object");
    let hooks = obj.entry("hooks").or_insert_with(|| json!({}));
    if !hooks.is_object() {
        return Err(format!(
            "`hooks` in `{}` is not an object; refusing to modify it.",
            settings_file.display()
        ));
    }
    let post = hooks
        .as_object_mut()
        .unwrap()
        .entry("PostToolUse")
        .or_insert_with(|| json!([]));
    let Some(groups) = post.as_array_mut() else {
        return Err(format!(
            "`hooks.PostToolUse` in `{}` is not an array; refusing to modify it.",
            settings_file.display()
        ));
    };
    groups.push(hook_entry(command));
    write_settings(settings_file, &settings)?;
    Ok(if had_any { HookChange::Updated } else { HookChange::Installed })
}

/// Removes only Layoza's handler(s); everything else in the file stays byte-for-byte equivalent.
pub fn uninstall_hook(settings_file: &Path) -> Result<HookChange, String> {
    if !settings_file.exists() {
        return Ok(HookChange::NotInstalled);
    }
    let mut settings = read_settings(settings_file)?;
    if strip_layoza_handlers(&mut settings) == 0 {
        return Ok(HookChange::NotInstalled);
    }
    write_settings(settings_file, &settings)?;
    Ok(HookChange::Removed)
}

fn command_executable(command: &str) -> Option<PathBuf> {
    let c = command.trim();
    let exe = if let Some(rest) = c.strip_prefix('"') {
        rest.split('"').next()?
    } else {
        c.split_whitespace().next()?
    };
    Some(PathBuf::from(exe))
}

pub fn hook_status(settings_file: &Path, target: HookTarget) -> HookStatus {
    let cmds = read_settings(settings_file)
        .map(|s| layoza_commands(&s))
        .unwrap_or_default();
    let command = cmds.into_iter().next();
    let command_resolves = command.as_deref().and_then(command_executable).map(|p| {
        // Bare names (e.g. `layoza` on PATH) cannot be checked cheaply; report them as resolvable.
        !p.is_absolute() || p.is_file()
    });
    HookStatus {
        target,
        settings_path: settings_file.to_string_lossy().replace('\\', "/"),
        installed: command.is_some(),
        command,
        command_resolves,
    }
}

// ---------------------------------------------------------------------------------------------
// check-edit
// ---------------------------------------------------------------------------------------------

pub fn is_ui_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| UI_EXTENSIONS.iter().any(|u| u.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}

/// Brand context discovered next to the edited file.
#[derive(Debug, Default)]
struct BrandContext {
    source: Option<String>,
    dna: Option<SiteDna>,
    palette: BTreeSet<String>,
}

/// Lower-case `#rrggbb` (alpha dropped, shorthand expanded) for every hex colour in `text`.
/// Skips fragment links such as `href="#add"`.
pub fn extract_hex_colors(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'#' {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < bytes.len() && bytes[j].is_ascii_hexdigit() {
            j += 1;
        }
        let len = j - (i + 1);
        let boundary_ok = j >= bytes.len() || !(bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_' || bytes[j] == b'-');
        let is_fragment = i >= 2
            && (bytes[i - 1] == b'"' || bytes[i - 1] == b'\'')
            && bytes[i - 2] == b'=';
        if boundary_ok && !is_fragment && matches!(len, 3 | 4 | 6 | 8) {
            let hex = text[i + 1..j].to_ascii_lowercase();
            let six = match len {
                3 | 4 => hex.chars().take(3).flat_map(|c| [c, c]).collect::<String>(),
                _ => hex[..6].to_string(),
            };
            out.push(format!("#{}", six));
        }
        i = j.max(i + 1);
    }
    out
}

fn load_brand_context(file: &Path, project_root: &Path) -> BrandContext {
    let mut ctx = BrandContext::default();
    let start = file.parent().unwrap_or(project_root);
    let mut dirs: Vec<PathBuf> = Vec::new();
    for anc in start.ancestors() {
        dirs.push(anc.to_path_buf());
        dirs.push(anc.join(".layoza"));
        if anc == project_root || dirs.len() > 24 {
            break;
        }
    }
    if !start.starts_with(project_root) {
        dirs.push(project_root.to_path_buf());
        dirs.push(project_root.join(".layoza"));
    }

    for d in &dirs {
        let dna_path = d.join("SITE-DNA.json");
        if ctx.source.is_none() && dna_path.is_file() {
            if let Ok(raw) = fs::read_to_string(&dna_path) {
                let raw = raw.trim_start_matches('\u{feff}');
                if let Ok(val) = serde_json::from_str::<Value>(raw) {
                    if let Some(colors) = val.get("colors").and_then(Value::as_array) {
                        for c in colors {
                            if let Some(v) = c.get("value").and_then(Value::as_str) {
                                ctx.palette.extend(extract_hex_colors(v));
                            }
                        }
                    }
                    ctx.dna = serde_json::from_value::<SiteDna>(val).ok();
                    ctx.source = Some(dna_path.to_string_lossy().replace('\\', "/"));
                }
            }
        }
        let design_md = d.join("DESIGN.md");
        if design_md.is_file() {
            if let Ok(raw) = fs::read_to_string(&design_md) {
                ctx.palette.extend(extract_hex_colors(&raw));
                if ctx.source.is_none() {
                    ctx.source = Some(design_md.to_string_lossy().replace('\\', "/"));
                }
            }
        }
        if ctx.source.is_some() {
            break;
        }
    }
    ctx
}

/// Per-project hook settings from `.layoza/hooks.json`. Unknown keys are ignored and a malformed
/// file falls back to defaults, so a typo never breaks the hook.
#[derive(Debug, Clone, serde::Deserialize, Serialize)]
pub struct HookConfig {
    #[serde(default = "default_budget")]
    pub budget_ms: u64,
    /// Guideline ids to run even though they ship `default_on: false`.
    #[serde(default)]
    pub enable: Vec<String>,
    /// Guideline ids (or verifier rule ids) never reported for this project.
    #[serde(default)]
    pub disable: Vec<String>,
    /// Run the verifier's static audit (on by default).
    #[serde(default = "default_true")]
    pub verifier: bool,
}

fn default_budget() -> u64 {
    DEFAULT_BUDGET_MS
}
fn default_true() -> bool {
    true
}

impl Default for HookConfig {
    fn default() -> Self {
        HookConfig { budget_ms: DEFAULT_BUDGET_MS, enable: Vec::new(), disable: Vec::new(), verifier: true }
    }
}

impl HookConfig {
    pub fn load(project_root: &Path) -> HookConfig {
        let mut cfg: HookConfig = fs::read_to_string(project_root.join(".layoza").join("hooks.json"))
            .ok()
            .and_then(|s| serde_json::from_str(s.trim_start_matches('\u{feff}')).ok())
            .unwrap_or_default();
        if let Some(ms) = env::var("LAYOZA_HOOK_BUDGET_MS").ok().and_then(|v| v.trim().parse::<u64>().ok()) {
            cfg.budget_ms = ms;
        }
        cfg.budget_ms = cfg.budget_ms.clamp(20, 5_000);
        cfg
    }

    fn is_disabled(&self, id: &str) -> bool {
        self.disable.iter().any(|d| d.eq_ignore_ascii_case(id))
    }

    /// A guideline rule runs when it opted in (`default_on`) or the project enabled it, and the
    /// project has not disabled it.
    pub fn rule_active(&self, id: &str, default_on: bool) -> bool {
        !self.is_disabled(id) && (default_on || self.enable.iter().any(|e| e.eq_ignore_ascii_case(id)))
    }
}

/// Exemption tokens that are about the whole document (a reduced-motion media query, a
/// color-scheme declaration) and so count anywhere in the file.
const FILE_WIDE_EXEMPTIONS: &[&str] = &["prefers-reduced-motion", "color-scheme"];
/// Lines either side of a hit that its exemption may sit on (a `<label>` just above an input).
const EXEMPTION_LINES: usize = 2;
/// Hits looked at per pattern before the rule is reported anyway.
const MAX_HITS_PER_PATTERN: usize = 200;

/// Whitespace-free, lower-case lines plus each line's start offset in `joined` (lines joined by '\n').
struct Squashed {
    joined: String,
    starts: Vec<usize>,
    whole: String,
}

fn squash(content: &str) -> Squashed {
    let lines: Vec<String> = content.lines().map(|l| l.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_lowercase()).collect();
    let mut starts = Vec::with_capacity(lines.len());
    let mut joined = String::new();
    for l in &lines {
        starts.push(joined.len());
        joined.push_str(l);
        joined.push('\n');
    }
    let whole = lines.concat();
    Squashed { joined, starts, whole }
}

/// True when every hit of `pattern` has one of `unless` in its own element or rule block
/// (`<...>` or `{...}`) or within EXEMPTION_LINES lines of it. Document-level tokens count
/// anywhere. A pattern that only appears across a line break is judged on the whole file.
fn every_hit_exempt(sq: &Squashed, pattern: &str, unless: &[String]) -> bool {
    if unless.is_empty() {
        return false;
    }
    if unless.iter().any(|u| FILE_WIDE_EXEMPTIONS.contains(&u.as_str()) && sq.whole.contains(u.as_str())) {
        return true;
    }
    let j = &sq.joined;
    let mut seen = 0;
    for (p, _) in j.match_indices(pattern) {
        seen += 1;
        if seen > MAX_HITS_PER_PATTERN {
            return false;
        }
        let line = sq.starts.partition_point(|s| *s <= p).saturating_sub(1);
        let lo = sq.starts[line.saturating_sub(EXEMPTION_LINES)];
        let hi = sq.starts.get(line + EXEMPTION_LINES + 1).copied().unwrap_or(j.len());
        let open = j[..p].rfind(['{', '<']).unwrap_or(0);
        let close = j[p + pattern.len()..].find(['}', '>']).map(|c| p + pattern.len() + c + 1).unwrap_or(j.len());
        let near = |a: usize, b: usize| unless.iter().any(|u| j[a..b].contains(u.as_str()));
        if !(near(open, close) || near(lo, hi)) {
            return false;
        }
    }
    if seen == 0 {
        // Only matched with line breaks removed: fall back to the file-level check.
        return unless.iter().any(|u| sq.whole.contains(u.as_str()));
    }
    true
}

/// Runs the opted-in guideline pattern rules on one file until `deadline_ms` after `start`.
/// Returns the findings and how many active rules were skipped because the budget ran out.
pub fn guideline_findings(
    label: &str,
    content: &str,
    cfg: &HookConfig,
    start: Instant,
    deadline_ms: u64,
) -> (Vec<AuditFinding>, usize) {
    let sq = squash(content);
    let squashed = &sq.whole;
    let mut out = Vec::new();
    let mut skipped = 0;
    for (g, rule) in guidelines::hook_rules() {
        if !cfg.rule_active(&g.id, rule.default_on) {
            continue;
        }
        if start.elapsed().as_millis() as u64 >= deadline_ms {
            skipped += 1;
            continue;
        }
        // A rule fires when some hit has no exemption in its own element, block or line window.
        let fires = rule.patterns.iter().any(|p| squashed.contains(p.as_str()) && !every_hit_exempt(&sq, p, &rule.unless));
        if !fires {
            continue;
        }
        let severity = match g.severity.as_str() {
            "critical" => Severity::Error,
            "low" => Severity::Info,
            _ => Severity::Warning,
        };
        out.push(AuditFinding {
            rule_id: g.id.clone(),
            kind: FindingKind::DeterministicObjective,
            severity,
            category: g.category.clone(),
            file_or_selector: label.into(),
            message: format!(
                "{} ({}).",
                g.rule.trim_end_matches('.'),
                rule.message.as_deref().unwrap_or("pattern found")
            ),
            remediation: format!("{} Check: {}.", g.do_.trim_end_matches('.'), g.check),
            auto_repairable: false,
        });
    }
    (out, skipped)
}

fn content_hash(content: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    content.hash(&mut h);
    h.finish()
}

#[derive(Debug, Default, serde::Deserialize, Serialize)]
struct HookState {
    #[serde(default)]
    checked: std::collections::BTreeMap<String, u64>,
    /// Content hash at the last check; a changed file is re-checked even inside the window.
    #[serde(default)]
    hashes: std::collections::BTreeMap<String, u64>,
    #[serde(default)]
    reminded_sessions: Vec<String>,
}

fn state_path(project_root: &Path) -> PathBuf {
    project_root.join(".layoza").join("hook-state.json")
}

fn load_state(project_root: &Path) -> HookState {
    fs::read_to_string(state_path(project_root))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_state(project_root: &Path, state: &HookState) {
    let p = state_path(project_root);
    if let Some(d) = p.parent() {
        let _ = fs::create_dir_all(d);
    }
    if let Ok(s) = serde_json::to_string(state) {
        let _ = fs::write(p, s);
    }
}

fn rate_key(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");
    if cfg!(windows) {
        s.to_ascii_lowercase()
    } else {
        s
    }
}

fn severity_tag(f: &AuditFinding) -> &'static str {
    match (&f.kind, &f.severity) {
        (FindingKind::SubjectiveCritique, _) => "NOTE",
        (_, Severity::Error) => "ERROR",
        (_, Severity::Warning) => "WARN",
        (_, Severity::Info) => "INFO",
    }
}

fn off_palette_note(label: &str, content: &str, palette: &BTreeSet<String>) -> Option<AuditFinding> {
    if palette.len() < 3 {
        return None; // not enough brand evidence to call anything off-brand
    }
    let mut off: Vec<String> = Vec::new();
    for c in extract_hex_colors(content) {
        if c == "#ffffff" || c == "#000000" || palette.contains(&c) || off.contains(&c) {
            continue;
        }
        off.push(c);
    }
    if off.is_empty() {
        return None;
    }
    let shown: Vec<&str> = off.iter().take(4).map(String::as_str).collect();
    Some(AuditFinding {
        rule_id: "BRAND-OFF-PALETTE".into(),
        kind: FindingKind::SubjectiveCritique,
        severity: Severity::Info,
        category: "brand".into(),
        file_or_selector: label.into(),
        message: format!(
            "{} hard-coded colour(s) not in the brand palette: {}{}.",
            off.len(),
            shown.join(", "),
            if off.len() > shown.len() { ", ..." } else { "" }
        ),
        remediation: "Use the existing brand tokens / CSS variables from SITE-DNA.json or DESIGN.md unless this is an intentional new token.".into(),
        auto_repairable: false,
    })
}

/// Runs the per-edit check. `input` is the hook's stdin JSON. Returns the exact stdout to print
/// (a Claude Code hook JSON object) or `None` when the hook should stay silent.
/// Never returns an error: anything unexpected degrades to silence.
pub fn check_edit(input: &str, project_dir_env: Option<&Path>, cwd: &Path, now_ms: u64) -> Option<String> {
    let start = Instant::now();
    if env::var("LAYOZA_HOOK").map(|v| v == "off" || v == "0").unwrap_or(false) {
        return None;
    }
    let payload: Value = serde_json::from_str(input.trim_start_matches('\u{feff}')).ok()?;
    let raw_path = payload
        .pointer("/tool_input/file_path")
        .or_else(|| payload.pointer("/tool_input/path"))
        .or_else(|| payload.pointer("/tool_response/filePath"))
        .and_then(Value::as_str)?;
    let payload_cwd = payload.get("cwd").and_then(Value::as_str).map(PathBuf::from);
    let project_root: PathBuf = project_dir_env
        .map(Path::to_path_buf)
        .or(payload_cwd.clone())
        .unwrap_or_else(|| cwd.to_path_buf());

    let mut file = PathBuf::from(raw_path);
    if file.is_relative() {
        file = payload_cwd.unwrap_or_else(|| cwd.to_path_buf()).join(file);
    }
    if !is_ui_file(&file) {
        return None;
    }
    let meta = fs::metadata(&file).ok()?;
    if !meta.is_file() || meta.len() > MAX_FILE_BYTES {
        return None;
    }

    let content = fs::read_to_string(&file).ok()?;
    let hash = content_hash(&content);

    // Rate limit (unchanged content only) + once-per-session reminder bookkeeping.
    let mut state = load_state(&project_root);
    let key = rate_key(&file);
    if let Some(&last) = state.checked.get(&key) {
        let unchanged = state.hashes.get(&key).is_none_or(|h| *h == hash);
        if unchanged && now_ms.saturating_sub(last) < RATE_LIMIT_MS {
            return None;
        }
    }
    state.checked.retain(|_, t| now_ms.saturating_sub(*t) < 3_600_000);
    let live: BTreeSet<String> = state.checked.keys().cloned().collect();
    state.hashes.retain(|k, _| live.contains(k));
    state.checked.insert(key.clone(), now_ms);
    state.hashes.insert(key, hash);
    let session = payload
        .get("session_id")
        .and_then(Value::as_str)
        .unwrap_or("unknown-session")
        .to_string();
    let remind = !state.reminded_sessions.contains(&session);
    if remind {
        state.reminded_sessions.push(session);
        let excess = state.reminded_sessions.len().saturating_sub(32);
        state.reminded_sessions.drain(..excess);
    }
    save_state(&project_root, &state);

    let cfg = HookConfig::load(&project_root);
    let label = file
        .strip_prefix(&project_root)
        .unwrap_or(&file)
        .to_string_lossy()
        .replace('\\', "/");
    let brand = load_brand_context(&file, &project_root);

    let mut findings = Vec::new();
    if cfg.verifier {
        audit_file_content(&label, &content, brand.dna.as_ref(), &mut findings);
    }
    if let Some(n) = off_palette_note(&label, &content, &brand.palette) {
        findings.push(n);
    }
    // Static subset of the guideline rules, only while the latency budget lasts.
    let (mut guide, skipped) = guideline_findings(&label, &content, &cfg, start, cfg.budget_ms);
    guide.truncate(MAX_GUIDELINE_FINDINGS);
    findings.extend(guide);
    findings.retain(|f| !cfg.is_disabled(&f.rule_id));

    // Brand-aware filter: never critique colours the brand itself owns.
    findings.retain(|f| {
        if f.kind != FindingKind::SubjectiveCritique || f.rule_id == "BRAND-OFF-PALETTE" {
            return true;
        }
        !extract_hex_colors(&f.message).iter().any(|c| brand.palette.contains(c))
    });

    let objective: Vec<&AuditFinding> = findings
        .iter()
        .filter(|f| {
            f.kind == FindingKind::DeterministicObjective
                && matches!(f.severity, Severity::Error | Severity::Warning)
        })
        .collect();
    let critiques: Vec<&AuditFinding> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::SubjectiveCritique)
        .take(MAX_CRITIQUE_NOTES)
        .collect();

    if objective.is_empty() && critiques.is_empty() && !remind {
        return None;
    }

    let elapsed_ms = start.elapsed().as_millis() as u64;
    let timing = if elapsed_ms > cfg.budget_ms || skipped > 0 {
        format!(
            " [{} ms, over the {} ms budget; {} guideline rule(s) skipped]",
            elapsed_ms, cfg.budget_ms, skipped
        )
    } else {
        format!(" [{} ms of {} ms budget]", elapsed_ms, cfg.budget_ms)
    };

    let mut lines = Vec::new();
    let errors = objective.iter().filter(|f| f.severity == Severity::Error).count();
    let warnings = objective.len() - errors;
    if objective.is_empty() && critiques.is_empty() {
        lines.push(format!("Layoza check-edit: `{}` passed the static checks.{}", label, timing));
    } else {
        lines.push(format!(
            "Layoza check-edit on `{}`: {} error(s), {} warning(s), {} design note(s){}.{}",
            label,
            errors,
            warnings,
            critiques.len(),
            brand
                .source
                .as_deref()
                .map(|s| format!(" (brand context: {})", s.rsplit('/').next().unwrap_or(s)))
                .unwrap_or_default(),
            timing
        ));
        for f in objective.iter().chain(critiques.iter()) {
            lines.push(format!(
                "- {} {}: {} Fix: {}",
                severity_tag(f),
                f.rule_id,
                f.message,
                f.remediation
            ));
        }
        if !objective.is_empty() {
            lines.push("These are objective static findings for this edit; fix them before moving on. NOTE items are advisory.".into());
        }
    }
    if remind {
        lines.push(format!(
            "Reminder (shown once per session): before declaring this UI work done, run `layoza qa {}` for the real-browser check (viewports, mobile menu, console, contrast).",
            label
        ));
    }

    let mut out = json!({
        "hookSpecificOutput": {
            "hookEventName": "PostToolUse",
            "additionalContext": lines.join("\n")
        }
    });
    if !objective.is_empty() {
        out["systemMessage"] = json!(format!(
            "Layoza: {} error(s), {} warning(s) in {}",
            errors, warnings, label
        ));
    }
    serde_json::to_string(&out).ok()
}

#[cfg(test)]
mod hook_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temp_dir(tag: &str) -> PathBuf {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let d = env::temp_dir().join(format!("layoza-hooks-{}-{}-{}-{}", tag, std::process::id(), n, nanos));
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn payload(file: &Path, session: &str) -> String {
        json!({
            "session_id": session,
            "hook_event_name": "PostToolUse",
            "tool_name": "Edit",
            "cwd": file.parent().unwrap().to_string_lossy(),
            "tool_input": { "file_path": file.to_string_lossy() }
        })
        .to_string()
    }

    fn context_of(out: &str) -> String {
        let v: Value = serde_json::from_str(out).unwrap();
        assert_eq!(v["hookSpecificOutput"]["hookEventName"], "PostToolUse");
        assert!(v.get("decision").is_none(), "hook must never block");
        v["hookSpecificOutput"]["additionalContext"].as_str().unwrap().to_string()
    }

    const CMD: &str = "\"C:/tools/layoza.exe\" check-edit --layoza-hook";

    #[test]
    fn install_merges_and_keeps_existing_settings_and_hooks() {
        let d = temp_dir("merge");
        let p = d.join(".claude").join("settings.json");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(
            &p,
            r#"{"permissions":{"allow":["Bash(npm test)"]},"model":"opus",
               "hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"guard.sh"}]}],
                        "PostToolUse":[{"matcher":"Edit|Write","hooks":[{"type":"command","command":"prettier --write"}]}]}}"#,
        )
        .unwrap();

        assert_eq!(install_hook(&p, CMD).unwrap(), HookChange::Installed);
        let v: Value = serde_json::from_str(&fs::read_to_string(&p).unwrap()).unwrap();
        assert_eq!(v["permissions"]["allow"][0], "Bash(npm test)");
        assert_eq!(v["model"], "opus");
        assert_eq!(v["hooks"]["PreToolUse"][0]["hooks"][0]["command"], "guard.sh");
        let post = v["hooks"]["PostToolUse"].as_array().unwrap();
        assert_eq!(post.len(), 2);
        assert_eq!(post[0]["hooks"][0]["command"], "prettier --write");
        assert_eq!(post[1], hook_entry(CMD));

        // Idempotent.
        assert_eq!(install_hook(&p, CMD).unwrap(), HookChange::AlreadyInstalled);
        // Stale path is repaired in place, never duplicated.
        let new_cmd = "\"D:/new/layoza.exe\" check-edit --layoza-hook";
        assert_eq!(install_hook(&p, new_cmd).unwrap(), HookChange::Updated);
        let v: Value = serde_json::from_str(&fs::read_to_string(&p).unwrap()).unwrap();
        assert_eq!(layoza_commands(&v), vec![new_cmd.to_string()]);
        assert_eq!(v["hooks"]["PostToolUse"].as_array().unwrap().len(), 2);
        assert!(hook_status(&p, HookTarget::Project).installed);
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn off_removes_only_layoza_entry() {
        let d = temp_dir("off");
        let p = d.join("settings.json");
        fs::write(
            &p,
            r#"{"hooks":{"PostToolUse":[{"matcher":"Edit|Write|MultiEdit","hooks":[
                {"type":"command","command":"eslint --fix"},
                {"type":"command","command":"layoza check-edit --layoza-hook"}]}]},"env":{"A":"1"}}"#,
        )
        .unwrap();
        assert_eq!(uninstall_hook(&p).unwrap(), HookChange::Removed);
        let v: Value = serde_json::from_str(&fs::read_to_string(&p).unwrap()).unwrap();
        assert_eq!(v["env"]["A"], "1");
        let handlers = v["hooks"]["PostToolUse"][0]["hooks"].as_array().unwrap();
        assert_eq!(handlers.len(), 1);
        assert_eq!(handlers[0]["command"], "eslint --fix");
        assert_eq!(uninstall_hook(&p).unwrap(), HookChange::NotInstalled);
        assert!(!hook_status(&p, HookTarget::Project).installed);

        // on -> off on a fresh file leaves no empty `hooks` scaffolding behind.
        let q = d.join("fresh.json");
        fs::write(&q, r#"{"theme":"dark"}"#).unwrap();
        install_hook(&q, CMD).unwrap();
        uninstall_hook(&q).unwrap();
        let v: Value = serde_json::from_str(&fs::read_to_string(&q).unwrap()).unwrap();
        assert_eq!(v, json!({"theme":"dark"}));
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn malformed_settings_are_never_overwritten() {
        let d = temp_dir("bad");
        let p = d.join("settings.json");
        fs::write(&p, "{ not json").unwrap();
        assert!(install_hook(&p, CMD).is_err());
        assert_eq!(fs::read_to_string(&p).unwrap(), "{ not json");
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn check_edit_is_silent_for_non_ui_files() {
        let d = temp_dir("nonui");
        let f = d.join("main.rs");
        fs::write(&f, "fn main() {}").unwrap();
        assert_eq!(check_edit(&payload(&f, "s1"), Some(&d), &d, 1_000_000), None);
        // Garbage stdin is silent too.
        assert_eq!(check_edit("not json", Some(&d), &d, 1_000_000), None);
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn check_edit_reports_html_findings_and_reminds_once() {
        let d = temp_dir("html");
        let f = d.join("index.html");
        fs::write(
            &f,
            "<!doctype html><html><head><meta name=\"viewport\" content=\"width=device-width\"></head><body><img src=\"a.png\"></body></html>",
        )
        .unwrap();
        let out = check_edit(&payload(&f, "s1"), Some(&d), &d, 1_000_000).expect("findings");
        let ctx = context_of(&out);
        assert!(ctx.contains("A11Y-HTML-LANG"), "{}", ctx);
        assert!(ctx.contains("A11Y-IMG-ALT"), "{}", ctx);
        assert!(ctx.contains("layoza qa index.html"), "{}", ctx);
        assert!(out.contains("systemMessage"));

        // Same session, after the rate-limit window: findings again, reminder not repeated.
        let out2 = check_edit(&payload(&f, "s1"), Some(&d), &d, 1_000_000 + RATE_LIMIT_MS + 1).unwrap();
        let ctx2 = context_of(&out2);
        assert!(ctx2.contains("A11Y-HTML-LANG"));
        assert!(!ctx2.contains("layoza qa"));

        // Clean file in the same session: silent.
        let clean = d.join("clean.css");
        fs::write(&clean, "a:focus-visible{outline:2px solid #123456}").unwrap();
        assert_eq!(check_edit(&payload(&clean, "s1"), Some(&d), &d, 2_000_000), None);
        // Clean file in a new session: just the one-time reminder.
        let ctx3 = context_of(&check_edit(&payload(&clean, "s2"), Some(&d), &d, 3_000_000).unwrap());
        assert!(ctx3.contains("passed the static checks") && ctx3.contains("layoza qa"));
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn check_edit_rate_limits_same_file() {
        let d = temp_dir("rate");
        let f = d.join("page.html");
        fs::write(&f, "<html><body><img src=x></body></html>").unwrap();
        let t0 = 5_000_000;
        assert!(check_edit(&payload(&f, "r"), Some(&d), &d, t0).is_some());
        assert_eq!(check_edit(&payload(&f, "r"), Some(&d), &d, t0 + 3_000), None);
        assert_eq!(check_edit(&payload(&f, "r"), Some(&d), &d, t0 + RATE_LIMIT_MS - 1), None);
        assert!(check_edit(&payload(&f, "r"), Some(&d), &d, t0 + RATE_LIMIT_MS + 5).is_some());
        assert!(state_path(&d).is_file());
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn check_edit_is_brand_aware() {
        let css = ".hero{background:linear-gradient(#8b5cf6,#ec4899)}";
        // Without brand context the generic gradient is critiqued.
        let d = temp_dir("nobrand");
        let f = d.join("hero.css");
        fs::write(&f, css).unwrap();
        let ctx = context_of(&check_edit(&payload(&f, "b"), Some(&d), &d, 1).unwrap());
        assert!(ctx.contains("CRITIQUE-GENERIC-GRADIENT"), "{}", ctx);

        // With SITE-DNA.json owning those colours it is not, but an off-brand colour is noted.
        let d2 = temp_dir("brand");
        fs::write(
            d2.join("SITE-DNA.json"),
            r##"{"colors":[{"name":"violet","value":"#8B5CF6"},{"name":"pink","value":"#ec4899"},{"name":"ink","value":"#111827"}]}"##,
        )
        .unwrap();
        let f2 = d2.join("hero.css");
        fs::write(&f2, format!("{} .cta{{color:#ff6600}}", css)).unwrap();
        let ctx2 = context_of(&check_edit(&payload(&f2, "b"), Some(&d2), &d2, 1).unwrap());
        assert!(!ctx2.contains("CRITIQUE-GENERIC-GRADIENT"), "{}", ctx2);
        assert!(ctx2.contains("BRAND-OFF-PALETTE") && ctx2.contains("#ff6600"), "{}", ctx2);
        assert!(!ctx2.contains("#8b5cf6,"), "{}", ctx2);
        let _ = fs::remove_dir_all(d);
        let _ = fs::remove_dir_all(d2);
    }

    #[test]
    fn guideline_rules_run_opt_in_and_report_timing() {
        let d = temp_dir("guide");
        let f = d.join("page.html");
        let html = "<html lang=\"en\"><head><meta name=\"viewport\" content=\"width=device-width, maximum-scale=1, user-scalable=no\"></head>\
                    <body><main><p style=\"margin-left: 8px\">Hi</p><a href=\"/x\" tabindex=\"3\">x</a></main></body></html>";
        fs::write(&f, html).unwrap();
        // A wide budget: under a loaded parallel test run the 300 ms default can run out before the
        // guideline rules start, which would make this test about the machine, not the rules.
        fs::create_dir_all(d.join(".layoza")).unwrap();
        fs::write(d.join(".layoza").join("hooks.json"), r#"{"budget_ms":5000}"#).unwrap();
        let out = check_edit(&payload(&f, "g"), Some(&d), &d, 1).expect("findings");
        let ctx = context_of(&out);
        assert!(ctx.contains("TOUCH-03"), "zoom rule (default on): {}", ctx);
        assert!(ctx.contains("NAV-11"), "positive tabindex rule: {}", ctx);
        assert!(!ctx.contains("I18N-02"), "opt-in-only rule must stay off by default: {}", ctx);
        assert!(ctx.contains("ms of 5000 ms budget"), "timing reported: {}", ctx);

        // Project opts in to I18N-02 and disables NAV-11; a changed file is re-checked at once.
        fs::write(d.join(".layoza").join("hooks.json"), r#"{"enable":["I18N-02"],"disable":["nav-11"],"budget_ms":4000}"#).unwrap();
        fs::write(&f, format!("{}\n", html)).unwrap();
        let ctx2 = context_of(&check_edit(&payload(&f, "g"), Some(&d), &d, 2).expect("changed file re-checked"));
        assert!(ctx2.contains("I18N-02") && !ctx2.contains("NAV-11"), "{}", ctx2);
        assert!(ctx2.contains("4000 ms budget"), "{}", ctx2);
        // Unchanged within the window: still rate-limited.
        assert_eq!(check_edit(&payload(&f, "g"), Some(&d), &d, 3), None);
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn guideline_rules_respect_the_budget() {
        let cfg = HookConfig::default();
        let start = Instant::now();
        let (found, skipped) = guideline_findings("a.css", ".a{transition:all .2s}", &cfg, start, 0);
        assert!(found.is_empty() && skipped > 0);
        let (found, skipped) = guideline_findings("a.css", ".a{transition: all .2s}", &cfg, start, 10_000);
        assert_eq!(skipped, 0);
        assert!(found.iter().any(|f| f.rule_id == "PERF-07"), "{:?}", found);
        // Exemptions apply: dvh present means 100vh fallback is fine.
        let (found, _) = guideline_findings("a.css", ".h{min-height:100vh;min-height:100dvh}", &cfg, start, 10_000);
        assert!(found.iter().all(|f| f.rule_id != "TOUCH-05"), "{:?}", found);
    }

    #[test]
    fn exemptions_only_cover_their_own_element_or_nearby_lines() {
        let cfg = HookConfig::default();
        let start = Instant::now();
        let fired = |css: &str, id: &str| guideline_findings("a.css", css, &cfg, start, 10_000).0.iter().any(|f| f.rule_id == id);
        // Same block, across lines: exempt.
        assert!(!fired(".hero {\n  min-height: 100vh;\n  min-height: 100dvh;\n}", "TOUCH-05"));
        // A dvh far away in another block does not excuse this 100vh.
        let far = format!(".hero {{ min-height: 100vh; }}\n{}.panel {{ height: 50dvh; }}", ".x { color: red; }\n".repeat(8));
        assert!(fired(&far, "TOUCH-05"), "distant dvh must not exempt");
        // Two hits, one exempt and one not: still reported.
        let mixed = format!(".a {{ height: 100vh; height: 100dvh; }}\n{}.b {{ height: 100vh; }}", ".x { color: red; }\n".repeat(8));
        assert!(fired(&mixed, "TOUCH-05"));
        // HTML: aria-label on the same input exempts; one on a distant element does not.
        let ok = r#"<input placeholder="Email" aria-label="Email">"#;
        assert!(guideline_findings("a.html", ok, &cfg, start, 10_000).0.iter().all(|f| f.rule_id != "FORM-01"));
        let label_above = "<label for=\"e\">Email</label>\n<input id=\"e\" placeholder=\"you@example.com\">";
        assert!(guideline_findings("a.html", label_above, &cfg, start, 10_000).0.iter().all(|f| f.rule_id != "FORM-01"), "label on the line above counts");
        let distant = format!("<input placeholder=\"Email\">\n{}<button aria-label=\"Close\">x</button>", "<p>text</p>\n".repeat(8));
        assert!(guideline_findings("a.html", &distant, &cfg, start, 10_000).0.iter().any(|f| f.rule_id == "FORM-01"));
        // Document-level exemptions still count anywhere in the file.
        let rm = format!("html {{ scroll-behavior: smooth; }}\n{}@media (prefers-reduced-motion: reduce) {{ html {{ scroll-behavior: auto; }} }}", ".x { color: red; }\n".repeat(20));
        assert!(!fired(&rm, "NAV-09"));
    }

    #[test]
    fn hex_extraction_skips_fragment_links() {
        assert_eq!(extract_hex_colors("a{color:#FFF} <a href=\"#add\">"), vec!["#ffffff"]);
        assert_eq!(extract_hex_colors("#12345678 #abcd"), vec!["#123456", "#aabbcc"]);
        assert!(extract_hex_colors("#section-1 #fffff").is_empty());
    }
}
