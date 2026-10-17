// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Baseline management, atomic rollback, security path sandboxing, untrusted input sanitization,
// and credential redaction.

use crate::inspector::compute_project_fingerprint;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineSnapshot {
    pub id: String,
    pub reason: String,
    pub fingerprint: String,
    pub files: BTreeMap<String, String>,
    /// Paths deliberately not captured (excluded directory, cap reached, too large, not UTF-8).
    #[serde(default)]
    pub skipped: Vec<SkippedPath>,
    /// Total number of skipped paths (`skipped` lists at most `MAX_SKIPPED_LISTED`).
    #[serde(default)]
    pub skipped_total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkippedPath {
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackReport {
    pub baseline_id: String,
    pub restored_files: Vec<String>,
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Default-discovery limits: enough for a real site's UI sources, small enough to keep the JSON sane.
pub const MAX_BASELINE_FILES: usize = 2000;
pub const MAX_BASELINE_FILE_BYTES: u64 = 1024 * 1024;
pub const MAX_BASELINE_TOTAL_BYTES: u64 = 32 * 1024 * 1024;
const MAX_SKIPPED_LISTED: usize = 200;

/// Directories never walked by default discovery (dependencies, VCS, build output, Layoza state).
const EXCLUDED_DIRS: &[&str] = &[
    "node_modules", ".git", "target", "dist", "build", ".layoza", ".next", ".nuxt", ".svelte-kit", ".astro",
    ".output", ".turbo", ".cache", "vendor", "coverage",
];
const UI_EXTENSIONS: &[&str] = &[
    "html", "htm", "css", "scss", "sass", "less", "js", "mjs", "cjs", "ts", "tsx", "jsx", "vue", "svelte", "astro",
    "liquid", "php", "twig",
];
const DESIGN_DOCS: &[&str] = &["DESIGN.md", "BRAND.md", "MOTION.md", "DESIGN-DECISIONS.md"];
const THEME_JSON_NAMES: &[&str] = &[
    "theme.json", "settings_data.json", "settings_schema.json", "tokens.json", "design-tokens.json", "SITE-DNA.json",
    "COMPONENT-INVENTORY.json",
];
/// JSON inside these directories is treated as theme/template data (Shopify, Gutenberg, token folders).
const THEME_JSON_DIRS: &[&str] = &["templates", "sections", "config", "blocks", "patterns", "styles", "tokens"];

/// Validates that `candidate` resolves inside `approved_root` to prevent path traversal (`..`).
pub fn validate_safe_project_path(approved_root: &Path, candidate: &Path) -> Result<PathBuf, String> {
    let root_canon = approved_root
        .canonicalize()
        .map_err(|e| format!("Invalid project root `{}`: {}", approved_root.display(), e))?;

    let joined = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        approved_root.join(candidate)
    };

    // Check components for `..` even before file creation
    for comp in candidate.components() {
        if matches!(comp, std::path::Component::ParentDir) {
            return Err("Path traversal (`..`) outside approved project root is forbidden.".to_string());
        }
    }

    if joined.exists() {
        let cand_canon = joined
            .canonicalize()
            .map_err(|e| format!("Cannot resolve path `{}`: {}", joined.display(), e))?;
        ensure_inside(&cand_canon, &root_canon)?;
        return Ok(strip_verbatim_prefix(cand_canon));
    }

    // Not created yet: resolve the nearest existing ancestor (following symlinks) so absolute paths
    // and symlinked parents outside the root are rejected before anything is written there.
    let mut ancestor = joined.as_path();
    let mut rest: Vec<std::ffi::OsString> = Vec::new();
    while !ancestor.exists() {
        match (ancestor.parent(), ancestor.file_name()) {
            (Some(parent), Some(name)) => {
                rest.push(name.to_os_string());
                ancestor = parent;
            }
            _ => {
                return Err(format!(
                    "Security violation: path `{}` has no existing ancestor inside approved project root `{}`.",
                    joined.display(),
                    root_canon.display()
                ))
            }
        }
    }
    let anc_canon = ancestor
        .canonicalize()
        .map_err(|e| format!("Cannot resolve path `{}`: {}", ancestor.display(), e))?;
    ensure_inside(&anc_canon, &root_canon)?;
    let mut resolved = anc_canon;
    for part in rest.iter().rev() {
        resolved.push(part);
    }
    Ok(strip_verbatim_prefix(resolved))
}

fn ensure_inside(candidate_canon: &Path, root_canon: &Path) -> Result<(), String> {
    if candidate_canon.starts_with(root_canon) {
        Ok(())
    } else {
        Err(format!(
            "Security violation: path `{}` escapes approved project root `{}`.",
            candidate_canon.display(),
            root_canon.display()
        ))
    }
}

/// Windows `canonicalize()` returns `\\?\C:\...` verbatim paths, which leak into reports and confuse
/// tools that don't expect them; drop the prefix for ordinary drive paths (UNC shares are kept).
fn strip_verbatim_prefix(path: PathBuf) -> PathBuf {
    let s = path.to_string_lossy();
    match s.strip_prefix(r"\\?\") {
        Some(rest) if !rest.starts_with(r"UNC\") => PathBuf::from(rest),
        _ => path,
    }
}

/// Treats inspected website text, DOM strings, or external design files as untrusted data,
/// stripping prompt-injection directives that attempt to impersonate agent instructions.
pub fn sanitize_untrusted_content(input: &str) -> String {
    let mut sanitized = Vec::new();
    for line in input.lines() {
        let lower = line.to_ascii_lowercase();
        if lower.contains("ignore previous instructions")
            || lower.contains("ignore all prior instructions")
            || lower.contains("system prompt:")
            || lower.contains("<system_message>")
            || lower.contains("execute shell command")
        {
            sanitized.push("[REDACTED_UNTRUSTED_PROMPT_INJECTION_ATTEMPT]".to_string());
        } else {
            sanitized.push(redact_credentials(line));
        }
    }
    sanitized.join("\n")
}

/// Redacts common API keys, Bearer tokens, and secret assignments from logs, tool results, and reports.
/// Every occurrence is redacted. A prefix only counts at a token boundary and when followed by a
/// secret-shaped run (>= 16 key characters), so identifiers such as `space-grotesk-public-sans`,
/// `mask-image` or `task-ui` are left intact.
pub fn redact_credentials(input: &str) -> String {
    const PREFIXES: [&str; 7] = ["sk-", "ghp_", "gho_", "github_pat_", "AIza", "xoxb-", "Bearer "];
    const MIN_SECRET_LEN: usize = 16;
    let is_key_char = |c: char| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~' | '+' | '/' | '=');
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        let rest = &input[i..];
        let at_boundary = input[..i].chars().next_back().is_none_or(|c| !c.is_ascii_alphanumeric() && c != '-' && c != '_');
        let hit = if at_boundary { PREFIXES.iter().find(|p| rest.starts_with(**p)) } else { None };
        if let Some(prefix) = hit {
            let body = &rest[prefix.len()..];
            let len = body.find(|c: char| !is_key_char(c)).unwrap_or(body.len());
            if len >= MIN_SECRET_LEN {
                out.push_str(prefix);
                out.push_str("[REDACTED]");
                i += prefix.len() + len;
                continue;
            }
        }
        let ch = rest.chars().next().unwrap_or_default();
        out.push(ch);
        i += ch.len_utf8().max(1);
    }
    out
}

/// True for files default discovery treats as UI source: markup, styles, scripts, component and
/// template files, theme/token JSON, and the Layoza design documents.
fn is_ui_source(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if DESIGN_DOCS.contains(&name) {
        return true;
    }
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    if ext == "json" {
        let parent = path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()).unwrap_or("");
        return THEME_JSON_NAMES.contains(&name) || THEME_JSON_DIRS.contains(&parent);
    }
    UI_EXTENSIONS.contains(&ext.as_str())
}

fn rel_key(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/")
}

/// Walks `dir` without following symlinks and returns UI source files in path order, plus the
/// directories it deliberately left out (dependencies, VCS, build output, hidden directories).
/// `root` is only used to make reported paths relative.
pub fn discover_baseline_files(root: &Path, dir: &Path) -> (Vec<PathBuf>, Vec<SkippedPath>) {
    let mut found = Vec::new();
    let mut skipped = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let mut entries: Vec<_> = match fs::read_dir(&current) {
            Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
            Err(e) => {
                skipped.push(SkippedPath { path: rel_key(root, &current), reason: format!("unreadable directory: {}", e) });
                continue;
            }
        };
        entries.sort_by_key(|e| e.file_name());
        let mut subdirs = Vec::new();
        for entry in entries {
            let path = entry.path();
            let Ok(ft) = entry.file_type() else { continue };
            if ft.is_symlink() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if ft.is_dir() {
                if EXCLUDED_DIRS.contains(&name.as_str()) || name.starts_with('.') {
                    skipped.push(SkippedPath { path: rel_key(root, &path), reason: "excluded directory".into() });
                } else {
                    subdirs.push(path);
                }
            } else if ft.is_file() && is_ui_source(&path) {
                found.push(path);
            }
        }
        // Reversed so the stack pops subdirectories in name order
        stack.extend(subdirs.into_iter().rev());
    }
    (found, skipped)
}

/// Creates a reversible baseline snapshot in `<project_root>/.layoza/baselines/<id>.json`
/// before Mode C / Mode D modifications or bounded auto-repairs.
///
/// With no `tracked_relative_paths`, the project's UI source files are discovered (see
/// [`discover_baseline_files`]) within the count/size caps, and everything left out is listed in
/// `skipped`. Explicit paths are strict: a missing, unreadable, non-UTF-8 or oversized file is an
/// error; an explicit directory is expanded with the discovery rules. A baseline that would capture
/// no files is an error, because rolling it back could restore nothing.
pub fn create_baseline(
    project_root: &Path,
    baseline_id: &str,
    reason: &str,
    tracked_relative_paths: &[String],
) -> Result<BaselineSnapshot, String> {
    let root = validate_safe_project_path(project_root, Path::new("."))?;
    let explicit = !tracked_relative_paths.is_empty();
    let mut candidates: Vec<(PathBuf, bool)> = Vec::new(); // (path, explicitly named file)
    let mut skipped = Vec::new();
    if explicit {
        for rel in tracked_relative_paths {
            let safe_path = validate_safe_project_path(project_root, Path::new(rel))?;
            if safe_path.is_file() {
                candidates.push((safe_path, true));
            } else if safe_path.is_dir() {
                let (found, skip) = discover_baseline_files(&root, &safe_path);
                if found.is_empty() {
                    return Err(format!("Cannot snapshot `{}`: directory contains no UI source files.", rel));
                }
                candidates.extend(found.into_iter().map(|p| (p, false)));
                skipped.extend(skip);
            } else {
                return Err(format!("Cannot snapshot `{}`: path does not exist.", rel));
            }
        }
    } else {
        let (found, skip) = discover_baseline_files(&root, &root);
        candidates.extend(found.into_iter().map(|p| (p, false)));
        skipped.extend(skip);
    }

    let mut files = BTreeMap::new();
    let mut total: u64 = 0;
    for (path, named) in candidates {
        let key = rel_key(&root, &path);
        if files.contains_key(&key) {
            continue;
        }
        let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let refusal = if size > MAX_BASELINE_FILE_BYTES {
            Some(format!("larger than {} bytes ({} bytes)", MAX_BASELINE_FILE_BYTES, size))
        } else if files.len() >= MAX_BASELINE_FILES {
            Some(format!("file cap of {} reached", MAX_BASELINE_FILES))
        } else if total + size > MAX_BASELINE_TOTAL_BYTES {
            Some(format!("total size cap of {} bytes reached", MAX_BASELINE_TOTAL_BYTES))
        } else {
            None
        };
        let outcome = match refusal {
            Some(r) => Err(r),
            None => match fs::read(&path) {
                Ok(bytes) => String::from_utf8(bytes).map_err(|_| "not valid UTF-8".to_string()),
                Err(e) => Err(format!("unreadable: {}", e)),
            },
        };
        match outcome {
            Ok(content) => {
                total += content.len() as u64;
                files.insert(key, content);
            }
            Err(why) if named => return Err(format!("Cannot snapshot `{}`: {}.", key, why)),
            Err(why) => skipped.push(SkippedPath { path: key, reason: why }),
        }
    }

    if files.is_empty() {
        return Err(format!(
            "Baseline would capture no files: no UI source files ({}, theme JSON, DESIGN.md) found under `{}`. Pass explicit paths to snapshot.",
            UI_EXTENSIONS.join("/"),
            root.display()
        ));
    }

    let skipped_total = skipped.len();
    skipped.truncate(MAX_SKIPPED_LISTED);
    let snapshot = BaselineSnapshot {
        id: baseline_id.to_string(),
        reason: reason.to_string(),
        fingerprint: compute_project_fingerprint(project_root),
        files,
        skipped,
        skipped_total,
    };

    let dir = project_root.join(".layoza").join("baselines");
    fs::create_dir_all(&dir).map_err(|e| format!("Failed creating baseline directory: {}", e))?;
    let file_path = dir.join(format!("{}.json", sanitize_id(baseline_id)));
    let json = serde_json::to_string_pretty(&snapshot).map_err(|e| e.to_string())?;
    fs::write(&file_path, json).map_err(|e| format!("Failed writing baseline file: {}", e))?;

    Ok(snapshot)
}

/// Restores all files captured in `<project_root>/.layoza/baselines/<id>.json`.
pub fn rollback_baseline(project_root: &Path, baseline_id: &str) -> Result<RollbackReport, String> {
    let file_path = project_root
        .join(".layoza")
        .join("baselines")
        .join(format!("{}.json", sanitize_id(baseline_id)));
    if !file_path.exists() {
        return Err(format!(
            "Baseline `{}` not found at `{}`.",
            baseline_id,
            file_path.display()
        ));
    }
    let raw = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let snapshot: BaselineSnapshot = serde_json::from_str(&raw).map_err(|e| e.to_string())?;

    let mut restored_files = Vec::new();
    for (rel, content) in &snapshot.files {
        let target = validate_safe_project_path(project_root, Path::new(rel))?;
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&target, content).map_err(|e| format!("Failed restoring `{}`: {}", rel, e))?;
        restored_files.push(rel.clone());
    }

    let success = !restored_files.is_empty();
    Ok(RollbackReport {
        baseline_id: snapshot.id,
        message: (!success).then(|| "Baseline contains no files; nothing was restored.".to_string()),
        restored_files,
        success,
    })
}

/// Reduces an id to `[A-Za-z0-9_-]` for safe use as a file name.
pub fn sanitize_id(id: &str) -> String {
    id.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::redact_credentials;

    #[test]
    fn redacts_every_secret_but_not_identifiers() {
        let s = "k1=sk-abcdefghijklmnop1234 k2=\"sk-ZYXWVUTSRQPONMLK9876\" Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.payload";
        let r = redact_credentials(s);
        assert!(!r.contains("abcdefghijklmnop1234") && !r.contains("ZYXWVUTSRQPONMLK9876") && !r.contains("eyJhbGci"), "{}", r);
        for keep in ["space-grotesk-public-sans", "mask-image: url(a.svg)", "task-ui (no scroll)", "desk-lamp", "sk-short", "ghp_x"] {
            assert_eq!(redact_credentials(keep), keep);
        }
        assert_eq!(redact_credentials("naïve café sk-"), "naïve café sk-");
    }
}
