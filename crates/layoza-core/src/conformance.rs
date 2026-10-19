// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
//! Render-vs-deal conformance: does a built page follow the page shape it was dealt? Compares the
//! render facts measured in Chromium (`scripts/render-facts.mjs`) with the direction's restraint
//! rules, its button shape and its motif. Pure and deterministic; the browser run lives in `qa`.
//!
//! Each restraint string is matched by its opening words. A rule that permits something ("only
//! where the brief names them") allows a little of it, never a page full of it. A fact the page
//! did not yield is `unknown`, never a pass.

use crate::directions::page_shape::PageShape;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Outcome of one check.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pass,
    Fail,
    Unknown,
}

/// One rule checked against the render.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConformCheck {
    /// The restraint text, or `button: <id>` / `motif: <id>`.
    pub rule: String,
    pub status: Status,
    /// What was measured, in a few words.
    pub evidence: String,
}

/// Conformance of one render with one dealt page shape.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conformance {
    /// Share of measured checks that pass, 0-100 (unknown checks are left out; 0 when none measured).
    pub score: u8,
    pub passed: usize,
    pub failed: usize,
    pub unknown: usize,
    pub checks: Vec<ConformCheck>,
}

/// Restraint rules that permit a little of what they otherwise ban.
fn permits(rule: &str) -> bool {
    let r = rule.to_lowercase();
    r.contains("only where the brief")
}

/// Per-width facts: `facts.viewports` (or a bare array).
fn viewports(facts: &Value) -> Vec<&Value> {
    facts.get("viewports").or(Some(facts)).and_then(|v| v.as_array()).map(|a| a.iter().collect()).unwrap_or_default()
}

fn label(vp: &Value) -> String {
    vp.get("label").and_then(|l| l.as_str()).map(String::from).unwrap_or_else(|| format!("{}px", vp["width"].as_u64().unwrap_or(0)))
}

fn desktop<'a>(vps: &[&'a Value]) -> Option<&'a Value> {
    vps.iter().copied().max_by_key(|v| v["width"].as_u64().unwrap_or(0))
}

fn len(v: &Value) -> usize {
    v.as_array().map(|a| a.len()).unwrap_or(0)
}

/// Max over widths of a count read by `f`; `None` when no width measured it.
fn max_over(vps: &[&Value], f: impl Fn(&Value) -> Option<u64>) -> Option<u64> {
    vps.iter().filter_map(|v| f(v)).max()
}

/// First sample texts from a list of `{text}` or `{el}` objects.
fn samples(list: &Value, n: usize) -> String {
    list.as_array()
        .into_iter()
        .flatten()
        .take(n)
        .filter_map(|x| x.get("text").or_else(|| x.get("el")).and_then(|t| t.as_str()))
        .map(|t| format!("\"{}\"", t))
        .collect::<Vec<_>>()
        .join(", ")
}

fn mk(rule: &str, status: Status, evidence: impl Into<String>) -> ConformCheck {
    ConformCheck { rule: rule.to_string(), status, evidence: evidence.into() }
}

fn pass_if(rule: &str, ok: bool, evidence: String) -> ConformCheck {
    mk(rule, if ok { Status::Pass } else { Status::Fail }, evidence)
}

/// Checks `facts` (render facts at one or more widths) against `page_shape`.
pub fn check(page_shape: &PageShape, facts: &Value) -> Conformance {
    let vps = viewports(facts);
    let mut checks: Vec<ConformCheck> = page_shape.restraint.iter().map(|r| check_restraint(r, &vps)).collect();
    if page_shape.ground.build.contains(HERO_DARK) {
        checks.push(check_hero_dark(&vps));
    }
    if !page_shape.button.id.is_empty() {
        checks.push(check_button(&page_shape.button.id, &vps));
    }
    if !page_shape.motif.id.is_empty() {
        checks.push(check_motif(&page_shape.motif.id, &vps));
    }
    let passed = checks.iter().filter(|c| c.status == Status::Pass).count();
    let failed = checks.iter().filter(|c| c.status == Status::Fail).count();
    let unknown = checks.len() - passed - failed;
    let score = if passed + failed == 0 { 0 } else { ((passed as f64 * 100.0) / (passed + failed) as f64).round() as u8 };
    Conformance { score, passed, failed, unknown, checks }
}

fn check_restraint(rule: &str, vps: &[&Value]) -> ConformCheck {
    if vps.is_empty() {
        return mk(rule, Status::Unknown, "no render facts");
    }
    let lower = rule.to_lowercase();
    let starts = |p: &str| lower.starts_with(p);
    let permitted = permits(rule);
    if starts("no eyebrow") || starts("eyebrow labels") {
        let eyebrows = max_over(vps, |v| Some(len(&v["eyebrows"]) as u64)).unwrap_or(0);
        let caps = max_over(vps, |v| v["tiny_caps"]["count"].as_u64()).unwrap_or(0);
        let limit = if permitted { 2 } else { 0 };
        let worst = vps.iter().max_by_key(|v| len(&v["eyebrows"])).copied().unwrap_or(vps[0]);
        let mut ev = format!("{} eyebrow(s) before headings, {} tiny tracked-caps label(s) (limit {})", eyebrows, caps, limit);
        if eyebrows > 0 {
            ev.push_str(&format!("; e.g. {}", samples(&worst["eyebrows"], 3)));
        } else if caps > 0 {
            ev.push_str(&format!("; e.g. {}", samples(&worst["tiny_caps"]["samples"], 3)));
        }
        return pass_if(rule, eyebrows <= limit && caps <= limit, ev);
    }
    if starts("no monospace") {
        let chars = max_over(vps, |v| v["mono"]["chars"].as_u64()).unwrap_or(0);
        let ex = vps.iter().map(|v| samples(&v["mono"]["samples"], 2)).find(|s| !s.is_empty()).unwrap_or_default();
        let ev = if chars == 0 { "no visible text set in a monospace family".to_string() } else { format!("{} characters in a monospace family; e.g. {}", chars, ex) };
        return pass_if(rule, chars == 0, ev);
    }
    if starts("no index numerals") || starts("index numerals") {
        let n = max_over(vps, |v| Some(len(&v["numerals"]) as u64)).unwrap_or(0);
        let limit = if permitted { 6 } else { 0 };
        let worst = vps.iter().max_by_key(|v| len(&v["numerals"])).copied().unwrap_or(vps[0]);
        let mut ev = format!("{} index numeral label(s) (limit {})", n, limit);
        if n > 0 {
            ev.push_str(&format!("; e.g. {}", samples(&worst["numerals"], 3)));
        }
        return pass_if(rule, n <= limit, ev);
    }
    // "One light effect" is the wording in older deal JSON
    if starts("no gradients") || starts("light effects only where") || starts("one light effect") {
        let count = |v: &Value, k: &str| len(&v["effects"][k]) as u64;
        let total = |v: &Value| ["gradients", "glows", "backdrop", "blobs"].iter().map(|k| count(v, k)).sum::<u64>();
        let worst = vps.iter().max_by_key(|v| total(v)).copied().unwrap_or(vps[0]);
        let n = total(worst);
        let limit = if permitted { 2 } else { 0 };
        let mut ev = format!(
            "{} gradient(s), {} glow(s), {} backdrop blur(s), {} blob(s) at {} (limit {} in all)",
            count(worst, "gradients"),
            count(worst, "glows"),
            count(worst, "backdrop"),
            count(worst, "blobs"),
            label(worst),
            limit
        );
        if let Some(s) = worst["effects"]["scrims"].as_u64().filter(|s| *s > 0) {
            ev.push_str(&format!("; {} neutral scrim(s) not counted", s));
        }
        for k in ["gradients", "glows", "backdrop", "blobs"] {
            if count(worst, k) > 0 {
                ev.push_str(&format!("; {} e.g. {}", k, samples(&worst["effects"][k], 2)));
            }
        }
        return pass_if(rule, n <= limit, ev);
    }
    if starts("no icon cards") {
        let icons = max_over(vps, |v| Some(len(&v["icon_card_rows"]) as u64)).unwrap_or(0);
        let same = max_over(vps, |v| Some(len(&v["identical_card_rows"]) as u64)).unwrap_or(0);
        let d = desktop(vps).unwrap_or(vps[0]);
        let mut ev = format!("{} icon-card row(s), {} row(s) of 3+ identical cards", icons, same);
        let rows: Vec<String> = d["icon_card_rows"]
            .as_array()
            .into_iter()
            .flatten()
            .chain(d["identical_card_rows"].as_array().into_iter().flatten())
            .take(3)
            .map(|r| format!("{} x{} at {}x{}", r["el"].as_str().unwrap_or("?"), r["count"], r["w"], r["h"]))
            .collect();
        if !rows.is_empty() {
            ev.push_str(&format!("; {}", rows.join(", ")));
        }
        return pass_if(rule, icons == 0 && same == 0, ev);
    }
    if starts("at most two typefaces") {
        let d = desktop(vps).unwrap_or(vps[0]);
        let fonts: Vec<(String, u64)> = d["fonts"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|f| Some((f["family"].as_str()?.to_string(), f["chars"].as_u64()?)))
            .collect();
        let total: u64 = fonts.iter().map(|f| f.1).sum();
        if total == 0 {
            return mk(rule, Status::Unknown, "no visible text measured");
        }
        // A face carrying under 1% of the text (a stray glyph or a fallback) is not a typeface choice
        let used: Vec<&(String, u64)> = fonts.iter().filter(|f| f.1 * 100 >= total).collect();
        let ev = format!(
            "{} typeface(s): {}",
            used.len(),
            used.iter().map(|f| format!("{} ({} chars)", f.0, f.1)).collect::<Vec<_>>().join(", ")
        );
        return pass_if(rule, used.len() <= 2, ev);
    }
    if starts("one h1") {
        let mut problems = Vec::new();
        let mut parts = Vec::new();
        for v in vps {
            let count = v["h1"]["count"].as_u64().unwrap_or(0);
            let lines = v["h1"]["lines"].as_array().and_then(|l| l.first()).and_then(|l| l.as_u64()).unwrap_or(0);
            parts.push(format!("{}: {} h1, {} line(s)", label(v), count, lines));
            if count != 1 {
                problems.push(format!("{} h1 at {}", count, label(v)));
            }
            if lines > 2 {
                problems.push(format!("h1 runs {} lines at {}", lines, label(v)));
            }
        }
        let d = desktop(vps).unwrap_or(vps[0]);
        let words = d["statement"]["max_words"].as_u64().unwrap_or(0).max(d["statement"]["hero_words"].as_u64().unwrap_or(0));
        // "About 25 words": a statement past 32 words is a paragraph, not a statement
        if words > 32 {
            let sample = d["statement"]["sample"].as_str().filter(|s| !s.is_empty()).or(d["statement"]["hero_sample"].as_str()).unwrap_or("");
            problems.push(format!("a {}-word statement (\"{}\")", words, sample));
        }
        parts.push(format!("longest statement {} words", words));
        let ev = if problems.is_empty() { parts.join("; ") } else { format!("{}; {}", problems.join("; "), parts.join("; ")) };
        return pass_if(rule, problems.is_empty(), ev);
    }
    if starts("sections ") {
        let Some(d) = desktop(vps).filter(|d| d["width"].as_u64().unwrap_or(0) >= 1024) else {
            return mk(rule, Status::Unknown, "no desktop width measured");
        };
        let mut gaps: Vec<i64> = d["section_gaps"].as_array().into_iter().flatten().filter_map(|g| g.as_i64()).collect();
        if gaps.len() < 2 {
            return mk(rule, Status::Unknown, format!("{} section gap(s) measured; need 2+", gaps.len()));
        }
        // Gaps run content-to-content, so they span two section paddings: 2 x (88..240px)
        let cramped = gaps.iter().filter(|g| **g < 88).count();
        gaps.sort_unstable();
        let median = gaps[gaps.len() / 2];
        let ev = format!(
            "median content gap {}px at {} (want about 176-480px, two paddings); {} gap(s) under 88px; range {}-{}px",
            median,
            label(d),
            cramped,
            gaps[0],
            gaps[gaps.len() - 1]
        );
        return pass_if(rule, (176..=480).contains(&median) && cramped == 0, ev);
    }
    if starts("real pictures") {
        let d = desktop(vps).unwrap_or(vps[0]);
        let Some(n) = d["images"]["raster"].as_u64() else {
            return mk(rule, Status::Unknown, "pictures not measured");
        };
        return pass_if(rule, n >= 4, format!("{} distinct raster picture(s) at least 200px wide at {} (want 4+)", n, label(d)));
    }
    if starts("one live interaction") {
        let d = desktop(vps).unwrap_or(vps[0]);
        let ia = &d["interaction"];
        if ia.is_null() {
            return mk(rule, Status::Unknown, "interaction not measured");
        }
        let mut kinds: Vec<&str> = ia["controls"].as_array().into_iter().flatten().filter_map(|c| c["kind"].as_str()).collect();
        kinds.dedup();
        let table = ia["table"].as_bool().unwrap_or(false);
        let noscript = ia["noscript"].as_bool().unwrap_or(false);
        let ev = format!(
            "controls: {}; table: {}; noscript: {}",
            if kinds.is_empty() { "none".to_string() } else { kinds.join(", ") },
            if table { "yes" } else { "no" },
            if noscript { "yes" } else { "no" }
        );
        return pass_if(rule, !kinds.is_empty() && (table || noscript), ev);
    }
    if starts("the footer returns the brand line") {
        let d = desktop(vps).unwrap_or(vps[0]);
        let f = &d["footer"];
        let Some(ratio) = f["width_ratio"].as_f64() else {
            return mk(rule, Status::Unknown, "no footer text measured");
        };
        let ev = format!(
            "largest footer text \"{}\" at {}px spans {:.0}% of the {} width (want 60%+)",
            f["text"].as_str().unwrap_or(""),
            f["font_px"],
            ratio * 100.0,
            label(d)
        );
        return pass_if(rule, ratio >= 0.6, ev);
    }
    mk(rule, Status::Unknown, "no render check for this rule yet")
}

/// The clip-path when it cuts a shape: a polygon, path or round inset; a plain `inset()` is a
/// rectangle (often a reveal or sweep caught mid-state) and leaves the box as it is.
fn shape_clip(b: &Value) -> Option<&str> {
    b["clip"].as_str().filter(|c| !c.trim_start().starts_with("inset(") || c.contains("round"))
}

/// The ground build sentence a dark-hero deal carries.
const HERO_DARK: &str = "The hero opens on the dark ground";

/// A dealt dark hero: the first opaque ground behind the h1 is dark, or the h1 is set in light text
/// (a photo or video hero with a transparent box) at every width.
fn check_hero_dark(vps: &[&Value]) -> ConformCheck {
    let rule = "ground: the hero opens on the dark ground";
    let mut parts = Vec::new();
    let mut verdicts = Vec::new();
    for v in vps {
        let g = &v["hero_ground"];
        let bg = g["bg_luminance"].as_f64();
        let text = g["h1_text_luminance"].as_f64();
        if bg.is_none() && text.is_none() {
            continue;
        }
        let dark = bg.is_some_and(|l| l < 0.2) || text.is_some_and(|l| l > 0.6);
        verdicts.push(dark);
        let f = |x: Option<f64>| x.map(|l| format!("{:.2}", l)).unwrap_or_else(|| "none".into());
        parts.push(format!("{}: ground luminance {} ({}), h1 text luminance {}", label(v), f(bg), g["el"].as_str().unwrap_or("no opaque box"), f(text)));
    }
    if verdicts.is_empty() {
        return mk(rule, Status::Unknown, "no h1 to find the hero by");
    }
    pass_if(rule, verdicts.iter().all(|d| *d), format!("{} (dark under 0.20, or light h1 text over 0.60)", parts.join("; ")))
}

/// Shape bucket of one button: `pill`, `square`, `soft`, `clip`, `mask` or `rounded`.
fn button_shape(b: &Value) -> &'static str {
    let radius = b["radius"].as_f64().unwrap_or(0.0);
    let height = b["height"].as_f64().unwrap_or(0.0);
    if shape_clip(b).is_some() {
        "clip"
    } else if b["mask"].as_str().is_some() {
        "mask"
    } else if height > 0.0 && radius >= height / 2.0 - 1.0 {
        "pill"
    } else if radius <= 3.0 {
        "square"
    } else if (6.0..=14.0).contains(&radius) {
        "soft"
    } else {
        "rounded"
    }
}

fn check_button(id: &str, vps: &[&Value]) -> ConformCheck {
    let rule = format!("button: {}", id);
    let Some(d) = desktop(vps) else { return mk(&rule, Status::Unknown, "no render facts") };
    let all: Vec<&Value> = d["buttons"].as_array().into_iter().flatten().collect();
    let primary: Vec<&Value> = all.iter().copied().filter(|b| b["primary"].as_bool().unwrap_or(false)).collect();
    let set = if primary.is_empty() { all.clone() } else { primary.clone() };
    if set.is_empty() {
        return mk(&rule, Status::Unknown, "no buttons found");
    }
    let mut shapes: Vec<&str> = set.iter().map(|b| button_shape(b)).collect();
    shapes.sort_unstable();
    shapes.dedup();
    let want_ok = |b: &Value| -> bool {
        let shape = button_shape(b);
        match id {
            "pill" => shape == "pill",
            "chamfer" => shape == "clip",
            "notched" => shape == "clip" || shape == "mask",
            "soft-square" => shape == "soft",
            // square-stepped carries its stepped cut as a clip on a square box
            "square-stepped" => shape == "square" || shape == "clip",
            _ if id.starts_with("square") => shape == "square",
            _ => true,
        }
    };
    let off: Vec<&Value> = set.iter().copied().filter(|b| !want_ok(b)).collect();
    let which = if primary.is_empty() { "button(s)" } else { "primary button(s)" };
    let mut ev = format!(
        "{} {}, shapes: {}",
        set.len(),
        which,
        shapes.join(", ")
    );
    if let Some(b) = off.first() {
        ev.push_str(&format!(
            "; \"{}\" is {} (radius {}px, height {}px{})",
            b["text"].as_str().unwrap_or(""),
            button_shape(b),
            b["radius"],
            b["height"],
            if shape_clip(b).is_some() { ", clip-path" } else { "" }
        ));
    }
    // One shape for the page; square-stepped tolerates square secondaries beside the stepped primary
    let one_shape = shapes.len() == 1 || (id == "square-stepped" && shapes == ["clip", "square"]);
    pass_if(&rule, one_shape && off.is_empty(), ev)
}

fn check_motif(id: &str, vps: &[&Value]) -> ConformCheck {
    let rule = format!("motif: {}", id);
    if vps.is_empty() {
        return mk(&rule, Status::Unknown, "no render facts");
    }
    let n = |k: &str| max_over(vps, |v| v["motif"][k].as_u64()).unwrap_or(0);
    let (hint, found, need) = match id {
        "stepped-corner" => ("stepped clip-path polygons", n("stepped_polygons"), 1),
        "seam-grid" => ("hairline grid lines or 1-2px grid seams", n("grid_lines") + n("hairline_grids"), 1),
        "lit-edge" => ("single-side accent edges on media frames", n("single_side_borders") + n("edge_lines"), 1),
        "cut-corner" => ("chamfer clip-path polygons", n("chamfer_polygons"), 1),
        "ruled-frame" => ("full-width hairline rules", n("rules"), 2),
        "registration-marks" => ("L-shaped corner marks", n("corner_marks"), 2),
        "notched-edge" => ("side notches (radial masks or round side cut-outs)", n("radial_masks") + n("side_dots"), 1),
        "arch-top" => ("arch-topped frames", n("arch_tops"), 1),
        "torn-edge" => ("torn edges (many-point polygons or edge masks on wide bands)", n("torn_edges"), 1),
        "round-mask" => ("circular media masks", n("round_masks"), 1),
        _ => return mk(&rule, Status::Unknown, "no render hint for this motif"),
    };
    let mut ev = format!("{} {} found (want {}+)", found, hint, need);
    if found < need {
        let seen: Vec<String> = ["stepped_polygons", "chamfer_polygons", "grid_lines", "single_side_borders", "corner_marks", "arch_tops", "round_masks", "torn_edges", "rules"]
            .iter()
            .filter(|k| n(k) > 0)
            .map(|k| format!("{} {}", n(k), k.replace('_', " ")))
            .collect();
        if !seen.is_empty() {
            ev.push_str(&format!("; the page shows {} instead", seen.join(", ")));
        }
    }
    pass_if(&rule, found >= need, ev)
}

/// The page shape for `direction` (1-based) from what `--deal` points at: a `design-system --json`
/// result, a direction object, a bare `page_shape`, or any JSON that holds `directions`.
pub fn page_shape_from_deal(deal: &Value, direction: usize) -> Result<PageShape, String> {
    let parse = |v: &Value| serde_json::from_value::<PageShape>(v.clone()).map_err(|e| format!("page_shape does not parse: {}", e));
    if deal.get("restraint").is_some() || deal.get("hero_frame").is_some() {
        return parse(deal);
    }
    if let Some(ps) = deal.get("page_shape") {
        return parse(ps);
    }
    let list = find_directions(deal).ok_or("No `directions` with a `page_shape` in the deal JSON.")?;
    let n = list.len();
    let dir = list
        .get(direction.saturating_sub(1))
        .ok_or_else(|| format!("--direction {} is out of range: the deal has {} direction(s).", direction, n))?;
    parse(dir.get("page_shape").ok_or("That direction has no `page_shape`.")?)
}

/// First `directions` array (depth-first) whose entries carry a `page_shape`.
fn find_directions(v: &Value) -> Option<&Vec<Value>> {
    match v {
        Value::Object(o) => {
            if let Some(a) = o.get("directions").and_then(|d| d.as_array()) {
                if a.iter().any(|d| d.get("page_shape").is_some()) {
                    return Some(a);
                }
            }
            o.values().find_map(find_directions)
        }
        Value::Array(a) => a.iter().find_map(find_directions),
        _ => None,
    }
}

/// Plain-text report lines for terminals.
pub fn report_lines(c: &Conformance) -> Vec<String> {
    let mut out = vec![format!(
        "Conformance score      : {}/100 ({} pass, {} fail, {} unknown)",
        c.score, c.passed, c.failed, c.unknown
    )];
    for k in &c.checks {
        let tag = match k.status {
            Status::Pass => "PASS",
            Status::Fail => "FAIL",
            Status::Unknown => "????",
        };
        let rule: String = if k.rule.chars().count() > 70 { k.rule.chars().take(69).collect::<String>() + "…" } else { k.rule.clone() };
        out.push(format!("  [{}] {}", tag, rule));
        out.push(format!("         {}", k.evidence));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::directions::page_shape::ShapePick;
    use serde_json::json;

    fn shape(restraint: &[&str], motif: &str, button: &str) -> PageShape {
        PageShape {
            motif: ShapePick { id: motif.into(), build: String::new() },
            button: ShapePick { id: button.into(), build: String::new() },
            restraint: restraint.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        }
    }

    const RULES: &[&str] = &[
        "No eyebrow or kicker labels above headings, and no tiny tracked caps anywhere.",
        "No monospace face: figures use the body face's tabular numerals.",
        "No index numerals (01, 02) on sections or steps; an ordered list carries order.",
        "No gradients, glows, glass panels or blobs: flat colour chapters carry the page.",
        "No icon cards and no row of three or more identical cards: vary size, span and media per item.",
        "At most two typefaces; hierarchy comes from size and weight.",
        "One h1, authored in at most two lines at every width; statements stay under about 25 words.",
        "Sections 120-240px apart on desktop (about clamp(88px, 11vw, 168px)); never cramped bands.",
        "Real pictures of the trade's objects wherever the brief supplies them, four or more; no stock filler or placeholder art.",
        "One live interaction born from the product (its deciding spec made interactive) with a no-JS table of the same facts.",
        "The footer returns the brand line large across the full width.",
    ];

    fn clean_vp(width: u64) -> Value {
        json!({
            "label": if width >= 1024 { "desktop" } else { "mobile" },
            "width": width,
            "body_px": 17,
            "fonts": [{ "family": "Archivo", "chars": 9000 }, { "family": "Fraunces", "chars": 1200 }, { "family": "Arial", "chars": 4 }],
            "mono": { "chars": 0, "samples": [] },
            "eyebrows": [],
            "tiny_caps": { "count": 0, "samples": [] },
            "numerals": [],
            "effects": { "gradients": [], "scrims": 1, "glows": [], "backdrop": [], "blobs": [] },
            "icon_card_rows": [],
            "identical_card_rows": [],
            "h1": { "count": 1, "lines": [2] },
            "statement": { "hero_words": 18, "hero_sample": "", "max_words": 22, "sample": "" },
            "section_gaps": [300, 316, 290, 330],
            "images": { "raster": 6 },
            "interaction": { "controls": [{ "kind": "range", "el": "input" }], "table": true, "noscript": false },
            "footer": { "el": "p.brand", "text": "Lumen Works", "font_px": 180, "width_px": 1300, "width_ratio": 0.9 },
            "buttons": [
                { "el": "a.btn", "text": "Get a quote", "radius": 999, "height": 48, "width": 160, "clip": null, "mask": null, "filled": true, "primary": true },
                { "el": "a.btn", "text": "See panels", "radius": 24, "height": 48, "width": 150, "clip": null, "mask": null, "filled": true, "primary": true }
            ],
            "motif": { "arch_tops": 3, "stepped_polygons": 0 }
        })
    }

    #[test]
    fn a_clean_render_passes_every_rule() {
        let facts = json!({ "viewports": [clean_vp(1440), clean_vp(390)] });
        let c = check(&shape(RULES, "arch-top", "pill"), &facts);
        let fails: Vec<_> = c.checks.iter().filter(|k| k.status != Status::Pass).collect();
        assert!(fails.is_empty(), "{:#?}", fails);
        assert_eq!((c.score, c.passed, c.failed), (100, 13, 0));
    }

    #[test]
    fn a_slop_render_fails_with_evidence() {
        let mut vp = clean_vp(1440);
        vp["eyebrows"] = json!([{ "text": "OUR PROCESS", "before": "h2", "font_px": 12, "upper": true, "tracking_em": 0.2 }]);
        vp["mono"] = json!({ "chars": 120, "samples": [{ "el": "span", "family": "JetBrains Mono", "text": "P2.5" }] });
        vp["numerals"] = json!([{ "el": "span", "text": "01 /" }, { "el": "span", "text": "02 /" }]);
        vp["effects"]["gradients"] = json!([{ "el": "div.hero", "w": 1440, "h": 800 }]);
        vp["icon_card_rows"] = json!([{ "el": "div.features", "count": 3, "w": 400, "h": 240 }]);
        vp["fonts"] = json!([{ "family": "Inter", "chars": 5000 }, { "family": "Space Grotesk", "chars": 2000 }, { "family": "JetBrains Mono", "chars": 900 }]);
        vp["h1"] = json!({ "count": 2, "lines": [3, 1] });
        vp["section_gaps"] = json!([40, 64, 48]);
        vp["images"] = json!({ "raster": 1 });
        vp["interaction"] = json!({ "controls": [], "table": false, "noscript": false });
        vp["footer"] = json!({ "el": "p", "text": "© 2026", "font_px": 14, "width_px": 80, "width_ratio": 0.05 });
        vp["buttons"] = json!([
            { "el": "a.btn", "text": "Quote", "radius": 8, "height": 44, "width": 120, "clip": null, "mask": null, "filled": true, "primary": true },
            { "el": "a.btn", "text": "Call", "radius": 999, "height": 44, "width": 120, "clip": null, "mask": null, "filled": true, "primary": true }
        ]);
        vp["motif"] = json!({ "stepped_polygons": 4, "arch_tops": 0 });
        let c = check(&shape(RULES, "arch-top", "pill"), &json!({ "viewports": [vp] }));
        assert_eq!(c.passed, 0, "{:#?}", c.checks);
        assert_eq!(c.score, 0);
        let ev = |p: &str| c.checks.iter().find(|k| k.rule.starts_with(p)).unwrap().evidence.clone();
        assert!(ev("No eyebrow").contains("OUR PROCESS"));
        assert!(ev("No monospace").contains("120 characters"));
        assert!(ev("No index").contains("01 /"));
        assert!(ev("At most two").contains("3 typeface"));
        assert!(ev("One h1").contains("2 h1"));
        assert!(ev("button: pill").contains("soft"));
        assert!(ev("motif: arch-top").contains("stepped polygons"));
    }

    #[test]
    fn permissive_rules_allow_a_little_but_not_a_lot() {
        let eyebrow = "Eyebrow labels only where the brief names them, short and sentence case; no others, no tiny tracked caps.";
        let light = "Light effects only where the brief puts them, each one deliberate; no gradient washes, blobs or glass elsewhere.";
        let mut vp = clean_vp(1440);
        vp["eyebrows"] = json!([{ "text": "Indoor" }, { "text": "Outdoor" }]);
        vp["effects"]["glows"] = json!([{ "el": "div.panel", "blur_px": 30 }]);
        let few = check(&shape(&[eyebrow, light], "", ""), &json!({ "viewports": [vp.clone()] }));
        assert!(few.checks.iter().all(|k| k.status == Status::Pass), "{:#?}", few.checks);
        vp["eyebrows"] = json!([{ "text": "A" }, { "text": "B" }, { "text": "C" }]);
        vp["effects"]["gradients"] = json!([{ "el": "a" }, { "el": "b" }]);
        let many = check(&shape(&[eyebrow, light], "", ""), &json!({ "viewports": [vp] }));
        assert!(many.checks.iter().all(|k| k.status == Status::Fail), "{:#?}", many.checks);
    }

    #[test]
    fn the_old_light_wording_is_still_checked() {
        let old = "One light effect, only where the brief puts it; no gradient washes, blobs or glass elsewhere.";
        let mut vp = clean_vp(1440);
        vp["effects"]["gradients"] = json!([{ "el": "a" }, { "el": "b" }, { "el": "c" }]);
        let c = check(&shape(&[old], "", ""), &json!({ "viewports": [vp] }));
        assert_eq!(c.checks[0].status, Status::Fail, "{:#?}", c.checks);
    }

    #[test]
    fn a_dealt_dark_hero_is_checked_only_when_dealt() {
        let mut dark = shape(&[], "", "");
        dark.ground.build = "Flat chapters. The hero opens on the dark ground the brief describes; light sections follow.".into();
        let run = |ps: &PageShape, bg: Option<f64>, text: f64| {
            let mut vp = clean_vp(1440);
            vp["hero_ground"] = json!({ "el": "section.hero", "bg_luminance": bg, "h1_text_luminance": text });
            check(ps, &json!({ "viewports": [vp] }))
        };
        assert_eq!(run(&dark, Some(0.02), 0.95).checks[0].status, Status::Pass);
        assert_eq!(run(&dark, None, 0.9).checks[0].status, Status::Pass);
        assert_eq!(run(&dark, Some(0.9), 0.02).checks[0].status, Status::Fail);
        assert!(run(&shape(&[], "", ""), Some(0.9), 0.02).checks.is_empty());
        assert_eq!(check(&dark, &json!({ "viewports": [clean_vp(1440)] })).checks[0].status, Status::Unknown);
    }

    #[test]
    fn button_shapes_match_the_dealt_id() {
        let b = |radius: f64, clip: Option<&str>| json!({ "text": "Go", "radius": radius, "height": 48, "clip": clip, "mask": null, "primary": true });
        let run = |id: &str, buttons: Value| {
            let mut vp = clean_vp(1440);
            vp["buttons"] = buttons;
            check_button(id, &[&vp]).status
        };
        assert_eq!(run("pill", json!([b(24.0, None), b(999.0, None)])), Status::Pass);
        assert_eq!(run("square-ruled", json!([b(0.0, None), b(2.0, None)])), Status::Pass);
        assert_eq!(run("square-ruled", json!([b(0.0, None), b(8.0, None)])), Status::Fail);
        assert_eq!(run("square-lit", json!([b(0.0, Some("inset(0px 100% 0px 0px)")), b(0.0, None)])), Status::Pass);
        assert_eq!(run("square-lit", json!([b(0.0, Some("polygon(0 0, 100% 0, 100% 80%, 90% 100%, 0 100%)"))])), Status::Fail);
        assert_eq!(run("chamfer", json!([b(0.0, Some("polygon(0 0, 90% 0, 100% 20%, 100% 100%, 0 100%)"))])), Status::Pass);
        assert_eq!(run("chamfer", json!([b(0.0, None)])), Status::Fail);
        assert_eq!(run("soft-square", json!([b(6.0, None), b(6.0, None)])), Status::Pass);
        assert_eq!(run("soft-square", json!([b(24.0, None)])), Status::Fail);
        assert_eq!(run("pill", json!([])), Status::Unknown);
    }

    #[test]
    fn every_motif_has_a_hint_and_unknown_facts_stay_unknown() {
        for id in ["stepped-corner", "seam-grid", "lit-edge", "cut-corner", "ruled-frame", "registration-marks", "notched-edge", "arch-top", "torn-edge", "round-mask"] {
            let c = check_motif(id, &[&json!({ "width": 1440, "motif": {} })]);
            assert_eq!(c.status, Status::Fail, "{id}");
        }
        let c = check(&shape(&["A rule nobody wrote a check for."], "", ""), &json!({ "viewports": [clean_vp(1440)] }));
        assert_eq!(c.checks[0].status, Status::Unknown);
        assert_eq!(c.score, 0);
        let none = check(&shape(RULES, "arch-top", "pill"), &json!({}));
        assert!(none.checks.iter().all(|k| k.status == Status::Unknown));
    }

    #[test]
    fn page_shape_is_found_in_any_deal_form() {
        let ps = serde_json::to_value(shape(&["No monospace face."], "lit-edge", "square-lit")).unwrap();
        let full = json!({ "profile": {}, "directions": [{ "page_shape": ps.clone() }, { "page_shape": serde_json::to_value(shape(&[], "arch-top", "pill")).unwrap() }] });
        assert_eq!(page_shape_from_deal(&full, 1).unwrap().motif.id, "lit-edge");
        assert_eq!(page_shape_from_deal(&full, 2).unwrap().motif.id, "arch-top");
        assert!(page_shape_from_deal(&full, 3).unwrap_err().contains("out of range"));
        assert_eq!(page_shape_from_deal(&json!({ "page_shape": ps.clone() }), 1).unwrap().button.id, "square-lit");
        assert_eq!(page_shape_from_deal(&ps, 9).unwrap().button.id, "square-lit");
        assert!(page_shape_from_deal(&json!({ "x": 1 }), 1).is_err());
    }
}
