// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Live Chromium QA runner: locates `scripts/browser-qa.mjs` (or the copy embedded in this binary),
// runs it against a URL or local HTML file, and returns the measured v2 report.

use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

/// Copy of `scripts/browser-qa.mjs` compiled into the binary so live QA works from any project directory.
const EMBEDDED_QA_SCRIPT: &str = include_str!("../../../scripts/browser-qa.mjs");

/// Finds the QA script: `LAYOZA_QA_SCRIPT`, then `<project>/scripts/browser-qa.mjs` only when the
/// project is the Layoza repository itself (so a target project's own same-named script is never
/// executed), then the embedded copy written to the OS temp directory.
pub fn qa_script_path(project_root: &Path) -> Result<PathBuf, String> {
    if let Ok(custom) = std::env::var("LAYOZA_QA_SCRIPT") {
        let p = PathBuf::from(custom);
        if p.is_file() {
            return Ok(p);
        }
    }
    let local = project_root.join("scripts/browser-qa.mjs");
    if is_layoza_repo(project_root) && local.is_file() {
        return Ok(local);
    }
    let dir = std::env::temp_dir().join("layoza");
    fs::create_dir_all(&dir).map_err(|e| format!("Cannot create `{}`: {}", dir.display(), e))?;
    let path = dir.join(format!("browser-qa-{}.mjs", env!("CARGO_PKG_VERSION")));
    let current = fs::read_to_string(&path).unwrap_or_default();
    if current != EMBEDDED_QA_SCRIPT {
        fs::write(&path, EMBEDDED_QA_SCRIPT)
            .map_err(|e| format!("Cannot write embedded QA script to `{}`: {}", path.display(), e))?;
    }
    Ok(path)
}

/// True when `root` is the Layoza source repository (its own core crate sits under `crates/`).
fn is_layoza_repo(root: &Path) -> bool {
    fs::read_to_string(root.join("crates/layoza-core/Cargo.toml"))
        .map(|t| t.contains("name = \"layoza-core\""))
        .unwrap_or(false)
}

/// Default overall limit for one QA run; override with `LAYOZA_QA_TIMEOUT_SECS`.
pub const DEFAULT_QA_TIMEOUT_SECS: u64 = 180;

fn qa_timeout() -> Duration {
    let secs = std::env::var("LAYOZA_QA_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_QA_TIMEOUT_SECS);
    Duration::from_secs(secs)
}

/// Kills the runner and the browser processes it started.
fn kill_tree(child: &mut Child) {
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .args(["/PID", &child.id().to_string(), "/T", "/F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    #[cfg(unix)]
    {
        // The runner leads its own process group (see `run_browser_qa`), so this reaches Chromium too
        let _ = Command::new("kill")
            .args(["-KILL", &format!("-{}", child.id())])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let _ = child.kill();
    let _ = child.wait();
}

/// Runs `cmd` to completion, or kills it once `timeout` elapses. Output pipes are drained on
/// threads so a chatty runner cannot block on a full pipe.
fn output_with_timeout(cmd: &mut Command, timeout: Duration) -> Result<Output, String> {
    cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Could not start `node` for browser QA (Node.js >= 18 required): {}", e))?;
    let drain = |pipe: Option<Box<dyn Read + Send>>| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(mut p) = pipe {
                let _ = p.read_to_end(&mut buf);
            }
            buf
        })
    };
    let out_t = drain(child.stdout.take().map(|p| Box::new(p) as Box<dyn Read + Send>));
    let err_t = drain(child.stderr.take().map(|p| Box::new(p) as Box<dyn Read + Send>));
    let deadline = Instant::now() + timeout;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() >= deadline => {
                kill_tree(&mut child);
                return Err(format!(
                    "Browser QA runner timed out after {}s and was killed (set LAYOZA_QA_TIMEOUT_SECS to change the limit).",
                    timeout.as_secs()
                ));
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(100)),
            Err(e) => {
                kill_tree(&mut child);
                return Err(format!("Browser QA runner could not be awaited: {}", e));
            }
        }
    };
    Ok(Output { status, stdout: out_t.join().unwrap_or_default(), stderr: err_t.join().unwrap_or_default() })
}

/// Runs headless Chromium QA on `target` (an http(s) URL or a local HTML file path) and returns
/// the parsed report. Any runner failure is returned as an error, never as an empty pass.
pub fn run_browser_qa(project_root: &Path, target: &str, out_json: &Path, screenshot_dir: &Path) -> Result<Value, String> {
    let script = qa_script_path(project_root)?;
    let is_url = target.starts_with("http://") || target.starts_with("https://");
    if !is_url && !Path::new(target).is_file() {
        return Err(format!("QA target `{}` is neither an http(s) URL nor an existing HTML file.", target));
    }
    let mut cmd = Command::new("node");
    cmd.arg(&script)
        .arg(if is_url { "--url" } else { "--file" })
        .arg(target)
        .arg("--out")
        .arg(out_json)
        .arg("--screenshots")
        .arg(screenshot_dir)
        .current_dir(project_root);
    // The project's own declared design system, when it has one, enables the design-system-* rules
    let tokens_file = discover_design_tokens(project_root, if is_url { None } else { Some(Path::new(target)) })
        .and_then(|tokens| write_tokens_temp(&tokens).ok());
    if let Some(path) = &tokens_file {
        cmd.arg("--design-tokens").arg(path);
    }
    let output = output_with_timeout(&mut cmd, qa_timeout());
    if let Some(path) = &tokens_file {
        let _ = fs::remove_file(path);
    }
    let output = output?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let tail: Vec<&str> = stderr.lines().rev().take(8).collect();
        return Err(format!(
            "Browser QA runner failed ({}): {}",
            output.status,
            tail.into_iter().rev().collect::<Vec<_>>().join(" | ")
        ));
    }
    let raw = fs::read_to_string(out_json)
        .map_err(|e| format!("Browser QA finished but `{}` is unreadable: {}", out_json.display(), e))?;
    serde_json::from_str(&raw).map_err(|e| format!("Browser QA report is not valid JSON: {}", e))
}

// ---------------------------------------------------------------------------------------------
// Design-token discovery for the design-system-* browser rules
// ---------------------------------------------------------------------------------------------

/// Files that declare a project's design system, in priority order within one directory.
const TOKEN_SOURCES: &[&str] = &["DESIGN.md", ".layoza/DESIGN.md", ".layoza/SITE-DNA.json", "SITE-DNA.json"];

/// Finds the design system nearest to `target`: each directory from the target's own directory up to
/// `project_root` (inclusive) is checked for `DESIGN.md`, then `SITE-DNA.json`; for URL targets only
/// `project_root` is checked. Returns the compact token JSON
/// `{source, colors:[{name,value}], fonts:[{name,value}], radii:[{name,value}], font_sizes:[{name,value}|{name,min,max}]}`
/// (lengths in px, colours as `#rrggbb`), or `None` when no source declares a usable token.
pub fn discover_design_tokens(project_root: &Path, target: Option<&Path>) -> Option<Value> {
    let root = fs::canonicalize(project_root).unwrap_or_else(|_| project_root.to_path_buf());
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Some(t) = target {
        let joined = if t.is_absolute() { t.to_path_buf() } else { project_root.join(t) };
        let abs = fs::canonicalize(&joined)
            .or_else(|_| fs::canonicalize(t))
            .unwrap_or(joined);
        let mut cur = if abs.is_dir() { Some(abs.as_path()) } else { abs.parent() };
        while let Some(d) = cur {
            dirs.push(d.to_path_buf());
            // Stop at the project root; a target outside the root only contributes its own directory
            if d == root || !d.starts_with(&root) {
                break;
            }
            cur = d.parent();
        }
    }
    if !dirs.contains(&root) {
        dirs.push(root.clone());
    }
    for dir in dirs {
        for name in TOKEN_SOURCES {
            let path = dir.join(name);
            let Ok(content) = fs::read_to_string(&path) else { continue };
            let tokens = if name.ends_with(".md") {
                design_tokens_from_design_md(&content)
            } else {
                design_tokens_from_site_dna(&content)
            };
            if let Some(mut t) = tokens {
                let shown = path.to_string_lossy().replace('\\', "/");
                t["source"] = Value::String(shown.trim_start_matches("//?/").to_string());
                return Some(t);
            }
        }
    }
    None
}

fn write_tokens_temp(tokens: &Value) -> Result<PathBuf, String> {
    let dir = std::env::temp_dir().join("layoza");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = dir.join(format!("design-tokens-{}-{}.json", std::process::id(), nanos));
    fs::write(&path, tokens.to_string()).map_err(|e| e.to_string())?;
    Ok(path)
}

/// A YAML scalar without its quotes or trailing inline comment.
fn yaml_scalar(v: &str) -> String {
    let v = v.trim();
    if let Some(q) = v.chars().next().filter(|c| *c == '"' || *c == '\'') {
        let rest = &v[1..];
        return match rest.find(q) {
            Some(end) => rest[..end].trim().to_string(),
            None => rest.trim().to_string(),
        };
    }
    v.split(" #").next().unwrap_or(v).trim().to_string()
}

/// Converts a CSS length (`px`, `rem`, `em`; unitless numbers are px) to px at a 16px root.
/// Percentages, `var()` and keywords return `None`.
pub fn css_length_px(raw: &str) -> Option<f64> {
    let v = raw.trim().to_ascii_lowercase();
    let (num, mult) = if let Some(n) = v.strip_suffix("rem") {
        (n, 16.0)
    } else if let Some(n) = v.strip_suffix("em") {
        (n, 16.0)
    } else if let Some(n) = v.strip_suffix("px") {
        (n, 1.0)
    } else {
        (v.as_str(), 1.0)
    };
    num.trim().parse::<f64>().ok().filter(|n| n.is_finite() && *n >= 0.0).map(|n| n * mult)
}

/// A font-size token: a fixed length, or a fluid `clamp(min, preferred, max)` kept as its range.
fn font_size_token(name: &str, raw: &str) -> Option<Value> {
    let v = raw.trim();
    if let Some(inner) = v.strip_prefix("clamp(").and_then(|r| r.strip_suffix(')')) {
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 3 {
            let (min, max) = (css_length_px(parts[0])?, css_length_px(parts[2])?);
            return Some(json!({"name": name, "min": min, "max": max}));
        }
        return None;
    }
    css_length_px(v).map(|px| json!({"name": name, "value": px}))
}

/// Normalises a colour token to `#rrggbb`; accepts hex (3/6/8 digits) and `rgb()` / `rgba()`.
fn color_token_hex(raw: &str) -> Option<String> {
    let v = raw.trim();
    let hex = |(r, g, b): (u8, u8, u8)| format!("#{:02x}{:02x}{:02x}", r, g, b);
    // parse_hex_color reads #rgb and the first six digits of #rrggbb / #rrggbbaa
    if let Some(rgb) = crate::design_md::parse_hex_color(v) {
        return Some(hex(rgb));
    }
    let lower = v.to_ascii_lowercase();
    let inner = lower
        .strip_prefix("rgba(")
        .or_else(|| lower.strip_prefix("rgb("))?
        .strip_suffix(')')?;
    let nums: Vec<f64> = inner
        .split(|c: char| c == ',' || c == '/' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .take(3)
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
    if nums.len() != 3 {
        return None;
    }
    let ch = |n: f64| n.clamp(0.0, 255.0).round() as u8;
    Some(hex((ch(nums[0]), ch(nums[1]), ch(nums[2]))))
}

fn push_unique(list: &mut Vec<Value>, item: Value) {
    let same = |x: &Value| ["value", "min", "max"].iter().all(|k| x.get(*k) == item.get(*k));
    if !list.iter().any(same) {
        list.push(item);
    }
}

fn tokens_value(colors: Vec<Value>, fonts: Vec<Value>, radii: Vec<Value>, font_sizes: Vec<Value>) -> Option<Value> {
    if colors.is_empty() && fonts.is_empty() && radii.is_empty() && font_sizes.is_empty() {
        return None;
    }
    Some(json!({"colors": colors, "fonts": fonts, "radii": radii, "font_sizes": font_sizes}))
}

/// Extracts design tokens from the YAML frontmatter of a Layoza / Google-format `DESIGN.md`
/// (`colors:` map, `typography:` roles with `fontFamily` / `fontSize`, `rounded:` map), as written
/// by `design_md::generate_design_md`. `{colors.x}` references are resolved; values that are not
/// concrete colours or lengths (percentages, `var(...)`, keywords) are skipped, never guessed.
pub fn design_tokens_from_design_md(content: &str) -> Option<Value> {
    let mut lines = content.trim_start_matches('\u{feff}').lines();
    if lines.next().map(|l| l.trim()) != Some("---") {
        return None;
    }
    let mut colors_raw: BTreeMap<String, String> = BTreeMap::new();
    let mut color_order: Vec<String> = Vec::new();
    let mut rounded: Vec<(String, String)> = Vec::new();
    let mut typo: Vec<(String, String, String)> = Vec::new(); // (role, key, value)
    let (mut group, mut sub) = (String::new(), String::new());
    for line in lines {
        if line.trim() == "---" {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        let Some((k, v)) = trimmed.split_once(':') else { continue };
        let key = yaml_scalar(k);
        let val = yaml_scalar(v);
        if indent == 0 {
            group = key;
            sub.clear();
            continue;
        }
        if val.is_empty() {
            sub = key;
            continue;
        }
        match group.as_str() {
            "colors" => {
                if !colors_raw.contains_key(&key) {
                    color_order.push(key.clone());
                }
                colors_raw.insert(key, val);
            }
            "rounded" | "radii" => rounded.push((key, val)),
            "typography" if !sub.is_empty() => typo.push((sub.clone(), key, val)),
            _ => {}
        }
    }
    let resolve = |raw: &str| -> String {
        let mut cur = raw.to_string();
        for _ in 0..4 {
            let next = cur
                .trim()
                .strip_prefix("{colors.")
                .and_then(|r| r.strip_suffix('}'))
                .and_then(|k| colors_raw.get(k))
                .cloned();
            match next {
                Some(v) => cur = v,
                None => break,
            }
        }
        cur
    };
    let mut colors = Vec::new();
    for name in &color_order {
        if let Some(hex) = color_token_hex(&resolve(&colors_raw[name])) {
            push_unique(&mut colors, json!({"name": name, "value": hex}));
        }
    }
    let mut fonts = Vec::new();
    let mut font_sizes = Vec::new();
    for (role, key, val) in &typo {
        match key.as_str() {
            "fontFamily" | "font-family" => push_unique(&mut fonts, json!({"name": role, "value": val})),
            "fontSize" | "font-size" => {
                if let Some(t) = font_size_token(role, val) {
                    push_unique(&mut font_sizes, t);
                }
            }
            _ => {}
        }
    }
    let mut radii = Vec::new();
    for (name, val) in &rounded {
        if let Some(px) = css_length_px(val) {
            push_unique(&mut radii, json!({"name": name, "value": px}));
        }
    }
    tokens_value(colors, fonts, radii, font_sizes)
}

/// Extracts design tokens from a `SITE-DNA.json` (`colors[].{name,value}`,
/// `typography[].{role,font_family,font_size}`, `radii_scale{}`). Read loosely so partial DNA files
/// still contribute whatever tokens they declare.
pub fn design_tokens_from_site_dna(content: &str) -> Option<Value> {
    let dna: Value = serde_json::from_str(content.trim_start_matches('\u{feff}')).ok()?;
    let s = |v: &Value, k: &str| v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let mut colors = Vec::new();
    for c in dna.get("colors").and_then(|v| v.as_array()).into_iter().flatten() {
        if let Some(hex) = color_token_hex(&s(c, "value")) {
            push_unique(&mut colors, json!({"name": s(c, "name"), "value": hex}));
        }
    }
    let mut fonts = Vec::new();
    let mut font_sizes = Vec::new();
    for t in dna.get("typography").and_then(|v| v.as_array()).into_iter().flatten() {
        let role = s(t, "role");
        let fam = s(t, "font_family");
        if !fam.trim().is_empty() {
            push_unique(&mut fonts, json!({"name": role, "value": fam}));
        }
        if let Some(tok) = font_size_token(&role, &s(t, "font_size")) {
            push_unique(&mut font_sizes, tok);
        }
    }
    let mut radii = Vec::new();
    let empty = Map::new();
    for (name, v) in dna.get("radii_scale").and_then(|v| v.as_object()).unwrap_or(&empty) {
        if let Some(px) = v.as_str().and_then(css_length_px).or_else(|| v.as_f64()) {
            push_unique(&mut radii, json!({"name": name, "value": px}));
        }
    }
    tokens_value(colors, fonts, radii, font_sizes)
}
