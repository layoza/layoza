//! Evidence-backed creative workspace. The calling designer owns semantic understanding;
//! heuristics are a fallback, never a substitute for explicit client facts and visual review.
use crate::baseline::validate_safe_project_path;
use crate::intelligence::DesignRecommendation;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, path::Path};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct BriefContext {
    pub subject: String,
    pub audience: String,
    pub primary_action: String,
    pub objects: Vec<String>,
    pub facts: Vec<String>,
    pub required_content: Vec<String>,
    pub forbidden: Vec<String>,
    pub display_font: String,
    pub body_font: String,
    pub colors: Vec<String>,
    pub assets: Vec<String>,
}

impl BriefContext {
    pub fn validate(&self) -> Result<(), String> {
        if self.subject.trim().is_empty() { return Err("brief_context.subject is required; name the actual business or product.".into()); }
        let serialized = serde_json::to_string(self).map_err(|e| e.to_string())?;
        if serialized.chars().count() > 20_000 { return Err("brief_context is limited to 20000 characters.".into()); }
        for c in &self.colors {
            if crate::intelligence::wcag_contrast(c, "#FFFFFF").is_none() { return Err(format!("Invalid pinned hex colour `{c}`.")); }
        }
        Ok(())
    }

    pub fn query(&self, raw: &str) -> String {
        let mut s = format!("Industry: {}\nClient brief: {raw}", self.subject.trim());
        for (label, value) in [("Audience", &self.audience), ("Primary action", &self.primary_action), ("Heading font", &self.display_font), ("Body font", &self.body_font)] {
            if !value.trim().is_empty() { s.push_str(&format!("\n{label}: {}", value.trim())); }
        }
        if !self.colors.is_empty() { s.push_str(&format!("\nBrand colours: {}", self.colors.join(", "))); }
        if !self.objects.is_empty() { s.push_str(&format!("\nSubject objects: {}", self.objects.join(", "))); }
        for forbidden in &self.forbidden { s.push_str(&format!("\nNo {forbidden}.")); }
        s
    }

    pub fn apply(&self, rec: &mut DesignRecommendation) {
        if let Some(b) = rec.understood_brief.as_mut() {
            b.subject = self.subject.clone();
            if !self.audience.is_empty() { b.audience = self.audience.clone(); }
            if !self.primary_action.is_empty() { b.primary_action = self.primary_action.clone(); }
            b.must_haves.extend(self.required_content.iter().cloned());
            b.must_haves.extend(self.forbidden.iter().map(|f| format!("Do not use: {f}")));
            b.assumed.retain(|s| !(s.starts_with("audience:") && !self.audience.is_empty()) && !(s.starts_with("primary action:") && !self.primary_action.is_empty()));
        }
    }
}

/// The asset plan is executable: each direction names what the designer needs to obtain,
/// what is already local, and the role/crop that must be checked in the final render.
pub fn creative_workspace(rec: &DesignRecommendation, context: Option<&BriefContext>, inventory: &Value) -> Value {
    let subject = rec.understood_brief.as_ref().map(|b| b.subject.as_str()).unwrap_or(&rec.query);
    let concepts: Vec<Value> = rec.directions.iter().map(|d| json!({
        "direction": d.index, "title": d.title, "composition": d.layout_composition,
        "why_this_subject": d.rationale, "image_medium": d.hero_material,
        "asset_plan": {
            "subject": subject, "medium": d.hero_material,
            "available": inventory["assets"],
            "required_role": "A truthful subject image or working product view, composed at desktop and phone sizes",
            "acquisition": "Use supplied assets first. If missing, run scripts/design-assets.mjs search <subject> or import a verified image; register source, licence and credit with design_assets. Generate or draw only when that medium serves the idea.",
            "crop": "Choose focal point and aspect ratio per composition; preserve the subject on phones",
            "fallback": "If the required material cannot be obtained, change the concept honestly; never substitute a fake photograph or invented product proof"
        },
        "content_proof": context.map(|c| &c.facts),
        "tradeoff": format!("{} requires convincing {} material; judge the actual render before committing.", d.title, d.hero_material)
    })).collect();
    json!({
        "brief_context": context,
        "context_basis": if context.is_some() { "explicit structured client context supplied by the calling designer" } else { "heuristic reading; verify subject and pinned choices before building" },
        "concepts": concepts,
        "selection": "Render the three concepts with identical real content and available assets. Pick the clearest, most expressive, subject-specific composition; record the decision and visible tradeoffs with review_design. Familiar controls may be reused when they serve the task.",
        "success": ["brief fidelity", "clear hierarchy", "expressive typography", "convincing imagery", "coherent colour and spacing", "complete page rhythm", "designed phone composition", "usable interactions"],
        "novelty": "A supporting signal, never a reason to change a working brand or navigation pattern",
        "design_gate": "Browser defects and recorded process only. Exceptional design requires reviewing the actual renders, not passing a numeric checker."
    })
}

fn manifest(root: &Path) -> std::path::PathBuf { root.join(".layoza/assets.json") }
pub fn asset_inventory(root: &Path) -> Value {
    let mut assets: Vec<Value> = fs::read_to_string(manifest(root)).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
    for a in &mut assets {
        let p = a["path"].as_str().unwrap_or("");
        let bytes = validate_safe_project_path(root, Path::new(p)).ok().and_then(|p| fs::read(p).ok());
        a["exists"] = json!(bytes.is_some());
        a["content_current"] = json!(bytes.as_ref().zip(a["content_fingerprint"].as_str()).map(|(b, h)| hash_bytes(b) == h));
    }
    let mut candidates = Vec::new();
    fn walk(root: &Path, dir: &Path, depth: usize, out: &mut Vec<String>) {
        if depth > 4 || out.len() >= 100 { return; }
        let Ok(entries) = fs::read_dir(dir) else { return; };
        for entry in entries.flatten() {
            let Ok(kind) = entry.file_type() else { continue; };
            if kind.is_symlink() { continue; }
            let name = entry.file_name().to_string_lossy().to_string();
            let p = entry.path();
            if kind.is_dir() {
                if !name.starts_with('.') && !["node_modules", "target", "dist", "vendor"].contains(&name.as_str()) { walk(root, &p, depth + 1, out); }
            } else if p.extension().and_then(|e| e.to_str()).is_some_and(|e| ["png", "jpg", "jpeg", "webp", "avif", "svg", "mp4"].contains(&e.to_lowercase().as_str())) {
                out.push(p.strip_prefix(root).unwrap_or(&p).to_string_lossy().replace('\\', "/"));
            }
            if out.len() >= 100 { break; }
        }
    }
    walk(root, root, 0, &mut candidates); candidates.sort();
    json!({"assets": assets, "unregistered_candidates": candidates, "notice": "A candidate path is not a licence or proof that it belongs to this client. Inspect before using it."})
}

pub fn register_asset(root: &Path, record: &Value) -> Result<Value, String> {
    validate_safe_project_path(root, Path::new(".layoza/assets.json"))?;
    for k in ["path", "source", "license", "credit", "role", "alt"] {
        if record[k].as_str().is_none_or(|s| s.trim().is_empty() || s.len() > 2000) { return Err(format!("Asset `{k}` must be a nonempty string, at most 2000 characters.")); }
    }
    let p = validate_safe_project_path(root, Path::new(record["path"].as_str().unwrap()))?;
    if !p.is_file() { return Err("Asset path must name an existing file inside the project.".into()); }
    let mut records: Vec<Value> = fs::read_to_string(manifest(root)).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
    let mut record = record.clone();
    record["path"] = json!(p.strip_prefix(root).unwrap_or(&p).to_string_lossy().replace('\\', "/"));
    record["bytes"] = json!(fs::metadata(&p).map_err(|e| e.to_string())?.len());
    record["content_fingerprint"] = json!(hash_bytes(&fs::read(&p).map_err(|e| e.to_string())?));
    records.retain(|r| r["path"] != record["path"]); records.push(record.clone());
    fs::create_dir_all(root.join(".layoza")).map_err(|e| e.to_string())?;
    fs::write(manifest(root), serde_json::to_vec_pretty(&records).unwrap()).map_err(|e| e.to_string())?;
    Ok(record)
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut h = 0xcbf29ce484222325u64;
    for b in bytes { h = (h ^ *b as u64).wrapping_mul(0x100000001b3); }
    format!("{h:016x}")
}

pub fn review_is_current(root: &Path, review: &Value) -> bool {
    review["project_fingerprint"].as_str() == Some(crate::inspector::compute_project_fingerprint(root).as_str()) &&
    review["screenshot_evidence"].as_array().is_some_and(|shots| !shots.is_empty() && shots.iter().all(|s| {
        s["path"].as_str().and_then(|p| validate_safe_project_path(root, Path::new(p)).ok()).and_then(|p| fs::read(p).ok())
            .is_some_and(|bytes| s["content_fingerprint"].as_str() == Some(hash_bytes(&bytes).as_str()))
    }))
}

/// Designer selection is valid evidence without inventing a human approval.
pub fn has_visual_selection(root: &Path) -> bool {
    let Ok(entries) = fs::read_dir(root.join(".layoza/reviews")) else { return false; };
    entries.flatten().filter(|e| e.path().extension().is_some_and(|s| s == "json")).any(|e| {
        let r: Value = fs::read_to_string(e.path()).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(Value::Null);
        r["chosen_direction"].as_str().is_some_and(|s| !s.is_empty()) &&
        r["alternatives"].as_array().is_some_and(|a| a.len() >= 2) &&
        r["screenshot_evidence"].as_array().is_some_and(|a| a.len() >= 3) && review_is_current(root, &r)
    })
}

pub fn compare_screenshots(root: &Path, before: &str, after: &str) -> Result<Value, String> {
    let before = validate_safe_project_path(root, Path::new(before))?;
    let after = validate_safe_project_path(root, Path::new(after))?;
    let run = crate::qa::RunDir::new()?;
    let script = run.path().join("compare-renders.mjs");
    fs::write(&script, include_str!("../../../scripts/compare-renders.mjs")).map_err(|e| e.to_string())?;
    let out = std::process::Command::new("node").arg(script).arg(before).arg(after).output().map_err(|e| format!("Screenshot comparison requires Node: {e}"))?;
    if !out.status.success() { return Err(String::from_utf8_lossy(&out.stderr).trim().into()); }
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}

pub fn import_tokens(root: &Path, source: &str) -> Result<Value, String> {
    let source = validate_safe_project_path(root, Path::new(source))?;
    validate_safe_project_path(root, Path::new("design-tokens.css"))?;
    validate_safe_project_path(root, Path::new(".layoza/token-import.json"))?;
    let run = crate::qa::RunDir::new()?;
    let script = run.path().join("import-tokens.mjs");
    fs::write(&script, include_str!("../../../scripts/import-tokens.mjs")).map_err(|e| e.to_string())?;
    let out = std::process::Command::new("node").arg(script).arg(root).arg(source).output().map_err(|e| format!("Token import requires Node: {e}"))?;
    if !out.status.success() { return Err(String::from_utf8_lossy(&out.stderr).trim().into()); }
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}

pub fn record_review(root: &Path, record: &Value) -> Result<Value, String> {
    validate_safe_project_path(root, Path::new(".layoza/reviews"))?;
    for k in ["brief", "chosen_direction", "reason", "strengths", "weaknesses", "changes"] {
        if record.get(k).is_none_or(|v| v.is_null() || v.as_str().is_some_and(|s| s.trim().is_empty())) { return Err(format!("Visual review needs `{k}` with concrete evidence.")); }
    }
    for k in ["strengths", "weaknesses", "changes"] {
        if record[k].as_array().is_none_or(|a| a.is_empty() || a.iter().any(|s| s.as_str().is_none_or(|s| s.trim().is_empty()))) {
            return Err(format!("Visual review `{k}` must be a nonempty array of concrete observations."));
        }
    }
    let shots = record["screenshots"].as_array().filter(|a| !a.is_empty()).ok_or("A visual review needs actual screenshot paths.")?;
    let mut verified = Vec::new();
    for shot in shots {
        let p = validate_safe_project_path(root, Path::new(shot.as_str().ok_or("Screenshot paths must be strings.")?))?;
        if !p.is_file() { return Err(format!("Screenshot `{}` does not exist.", p.display())); }
        let bytes = fs::read(&p).map_err(|e| e.to_string())?;
        let png = bytes.len() >= 67 && bytes.starts_with(b"\x89PNG\r\n\x1a\n") && bytes.get(12..16) == Some(b"IHDR") && bytes.get(bytes.len()-8..bytes.len()-4) == Some(b"IEND");
        let jpeg = bytes.len() > 20 && bytes.starts_with(&[0xff, 0xd8, 0xff]) && bytes.ends_with(&[0xff, 0xd9]);
        let webp = bytes.len() > 20 && bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP");
        if !(png || jpeg || webp) { return Err("Review evidence needs a complete PNG, JPEG or WebP image header/trailer; a file extension or signature alone is insufficient.".into()); }
        verified.push(json!({"path": shot, "content_fingerprint": hash_bytes(&bytes)}));
    }
    let mut saved = record.clone();
    saved["screenshot_evidence"] = json!(verified);
    saved["project_fingerprint"] = json!(crate::inspector::compute_project_fingerprint(root));
    saved["review_kind"] = json!(record["review_kind"].as_str().unwrap_or("designer review; not an independent blind evaluation"));
    saved["recorded_at_unix"] = json!(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
    let id = crate::baseline::sanitize_id(record["id"].as_str().unwrap_or("latest"));
    let dir = root.join(".layoza/reviews"); fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let output = validate_safe_project_path(root, Path::new(&format!(".layoza/reviews/{id}.json")))?;
    fs::write(output, serde_json::to_vec_pretty(&saved).unwrap()).map_err(|e| e.to_string())?;
    Ok(json!({"saved": format!(".layoza/reviews/{id}.json"), "review": saved, "notice": "Persistence verifies that evidence files exist, not that a human or independent critic reviewed them. The reviewer must describe visible strengths and weaknesses honestly."}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn structured_context_is_explicit_and_review_cannot_claim_missing_evidence() {
        assert!(BriefContext::default().validate().is_err());
        let c = BriefContext { subject: "public library".into(), primary_action: "reserve a book".into(), ..Default::default() };
        assert!(c.validate().is_ok());
        assert!(c.query("Quiet community space").starts_with("Industry: public library"));
        let root = std::env::temp_dir().join(format!("layoza-review-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let r = json!({"brief":"library", "chosen_direction":"A", "reason":"Reading hierarchy", "strengths":["Readable"], "weaknesses":["Image small"], "changes":["Enlarge image"], "screenshots":["missing.png"]});
        assert!(record_review(&root, &r).is_err());
        fs::write(root.join("fake.png"), b"not an image").unwrap();
        let mut fake = r; fake["screenshots"] = json!(["fake.png"]);
        assert!(record_review(&root, &fake).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn visual_selection_requires_three_real_current_renders_and_detects_same_length_edits() {
        let root = std::env::temp_dir().join(format!("layoza-current-review-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("index.html"), "<h1>Before</h1>").unwrap();
        // Tiny valid PNG fixtures; persistence verifies bytes, not human review.
        let image = [137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,1,0,0,0,1,8,6,0,0,0,31,21,196,137,0,0,0,13,73,68,65,84,8,29,99,248,207,192,240,31,0,5,128,2,63,73,194,219,180,0,0,0,0,73,69,78,68,174,66,96,130];
        for f in ["a.png", "b.png", "c.png"] { fs::write(root.join(f), image).unwrap(); }
        let review = json!({"brief":"gallery", "chosen_direction":"A", "reason":"Larger photographs", "strengths":["Clear reading order"], "weaknesses":["Caption too small"], "changes":["Enlarge caption"], "alternatives":["B", "C"], "screenshots":["a.png","b.png","c.png"]});
        record_review(&root, &review).unwrap();
        assert!(has_visual_selection(&root));
        fs::write(root.join("index.html"), "<h1>After!</h1>").unwrap();
        assert!(!has_visual_selection(&root), "A same-length source edit invalidates evidence");
        record_review(&root, &review).unwrap();
        fs::write(root.join("a.png"), b"changed render").unwrap();
        assert!(!has_visual_selection(&root));
        fs::remove_dir_all(root).unwrap();
    }
}
