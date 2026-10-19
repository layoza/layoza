// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Layoza knowledge layer: UX guidelines, chart selection, per-stack notes and icon guidance.
//
// The data lives in `guidelines/*.json` and is compiled into the binary with `include_str!`, so the
// CLI, MCP server and edit hook all read the same tables with no file I/O. Every guideline carries
// a measurable check where one exists and says who can measure it (`browser` QA, `static` scan or
// `manual` review). Guidelines with a `hook` block opt in to the per-edit hook (`hooks.rs`).

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::OnceLock;

const UX_JSON: &str = include_str!("guidelines/ux.json");
const CHARTS_JSON: &str = include_str!("guidelines/charts.json");
const STACKS_JSON: &str = include_str!("guidelines/stacks.json");
const ICONS_JSON: &str = include_str!("guidelines/icons.json");

/// A static pattern check a guideline contributes to the edit hook.
/// Patterns and exemptions are matched against the file lower-cased with all whitespace removed.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HookRule {
    pub patterns: Vec<String>,
    #[serde(default)]
    pub unless: Vec<String>,
    /// Runs without project opt-in. Noisy rules ship `false` and are enabled in `.layoza/hooks.json`.
    pub default_on: bool,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guideline {
    pub id: String,
    pub category: String,
    pub rule: String,
    /// Measurable pass condition (thresholds in CSS px, ratios, ms).
    pub check: String,
    /// `browser` (agent QA can measure it on a rendered page), `static` (source scan) or `manual`.
    pub measured_by: String,
    #[serde(rename = "do")]
    pub do_: String,
    pub dont: String,
    pub good: String,
    pub bad: String,
    /// `critical` | `high` | `medium` | `low`.
    pub severity: String,
    pub platforms: Vec<String>,
    pub keywords: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hook: Option<HookRule>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChartFallback {
    pub table: String,
    pub aria_summary: String,
    pub keyboard: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chart {
    pub id: String,
    pub situation: String,
    pub data_shape: String,
    pub task: String,
    pub keywords: Vec<String>,
    pub best: String,
    pub alternatives: Vec<String>,
    pub avoid_when: String,
    pub volume: String,
    pub encoding: String,
    pub fallback: ChartFallback,
    /// `svg`, `canvas` or a threshold note.
    pub render: String,
    /// Library-agnostic options; none is required.
    pub libraries: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DoDont {
    #[serde(rename = "do")]
    pub do_: String,
    pub dont: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StackNotes {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    /// `web` or `mobile`.
    pub platform: String,
    pub tokens: DoDont,
    pub motion: DoDont,
    pub media: DoDont,
    pub a11y: DoDont,
    pub performance: DoDont,
}

impl StackNotes {
    pub fn sections(&self) -> [(&'static str, &DoDont); 5] {
        [
            ("tokens", &self.tokens),
            ("motion", &self.motion),
            ("media", &self.media),
            ("a11y", &self.a11y),
            ("performance", &self.performance),
        ]
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IconRole {
    pub role: String,
    pub examples: Vec<String>,
    pub guidance: String,
    pub label: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IconLibrary {
    pub name: String,
    pub style: String,
    pub fits: String,
    pub stacks: Vec<String>,
    pub license: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IconSize {
    pub text_px: u32,
    pub icon_px: u32,
    pub stroke: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IconGuide {
    pub rules: Vec<String>,
    pub roles: Vec<IconRole>,
    pub libraries: Vec<IconLibrary>,
    pub sizing: Vec<IconSize>,
}

struct Store {
    guidelines: Vec<Guideline>,
    charts: Vec<Chart>,
    stacks: Vec<StackNotes>,
    icons: IconGuide,
}

fn store() -> &'static Store {
    static STORE: OnceLock<Store> = OnceLock::new();
    STORE.get_or_init(|| Store {
        guidelines: serde_json::from_str(UX_JSON).expect("guidelines/ux.json is valid"),
        charts: serde_json::from_str(CHARTS_JSON).expect("guidelines/charts.json is valid"),
        stacks: serde_json::from_str(STACKS_JSON).expect("guidelines/stacks.json is valid"),
        icons: serde_json::from_str(ICONS_JSON).expect("guidelines/icons.json is valid"),
    })
}

pub fn guidelines() -> &'static [Guideline] {
    &store().guidelines
}
pub fn charts() -> &'static [Chart] {
    &store().charts
}
pub fn stacks() -> &'static [StackNotes] {
    &store().stacks
}
pub fn icons() -> &'static IconGuide {
    &store().icons
}

/// Distinct guideline categories in table order.
pub fn categories() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for g in guidelines() {
        if !out.contains(&g.category.as_str()) {
            out.push(&g.category);
        }
    }
    out
}

fn norm(s: &str) -> String {
    s.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// Resolves a stack by id, name or alias (`next`, `Next.js`, `nextjs` all work).
pub fn find_stack(name: &str) -> Option<&'static StackNotes> {
    let n = norm(name);
    let n2 = n.replace(['.', '-'], "");
    stacks().iter().find(|s| {
        norm(&s.id) == n
            || norm(&s.name) == n
            || norm(&s.name).replace(['.', '-'], "") == n2
            || s.aliases.iter().any(|a| norm(a) == n || norm(a).replace(['.', '-'], "") == n2)
    })
}

/// Guidelines whose `hook` block opts them in to the per-edit hook.
pub fn hook_rules() -> impl Iterator<Item = (&'static Guideline, &'static HookRule)> {
    guidelines().iter().filter_map(|g| g.hook.as_ref().map(|h| (g, h)))
}

/// Platforms a stack's guidance applies to (`mobile` stacks cover iOS and Android).
fn stack_platforms(stack: &StackNotes) -> &'static [&'static str] {
    match stack.id.as_str() {
        "swiftui" => &["ios"],
        "compose" => &["android"],
        _ if stack.platform == "mobile" => &["ios", "android"],
        _ => &["web"],
    }
}

fn platform_matches(g: &Guideline, wanted: &[&str]) -> bool {
    wanted.is_empty() || g.platforms.iter().any(|p| wanted.iter().any(|w| p.eq_ignore_ascii_case(w)))
}

/// One search result. `data` is the full record so callers can render any field.
#[derive(Debug, Clone, Serialize)]
pub struct SearchHit {
    /// `guideline` | `chart` | `stack`.
    pub kind: &'static str,
    pub id: String,
    pub title: String,
    pub score: u32,
    pub data: Value,
}

const STOPWORDS: &[&str] = &[
    "a", "an", "the", "and", "or", "of", "for", "to", "in", "on", "with", "by", "is", "are", "how", "what", "when",
    "should", "do", "i", "my", "over", "vs", "best", "use", "using",
];

/// Words a searcher uses for the same thing. A query word in a group also matches the others,
/// at one point less than a direct hit.
const SYNONYMS: &[&[&str]] = &[
    &["modal", "dialog", "popup", "lightbox", "overlay"],
    &["nav", "navigation", "menu", "navbar"],
    &["button", "cta", "call-to-action"],
    &["color", "colour"],
    &["font", "typography", "typeface", "text-size"],
    &["motion", "animation", "animate", "transition"],
    &["mobile", "phone", "small-screen"],
    &["touch", "tap"],
    &["form", "input", "field"],
    &["image", "img", "photo", "picture"],
    &["loading", "spinner", "skeleton", "progress"],
    &["error", "validation", "invalid"],
    &["a11y", "accessibility", "accessible", "screen-reader", "wcag"],
    &["toast", "snackbar", "notification"],
    &["dark-mode", "dark", "theme"],
    &["icon", "glyph", "pictogram"],
    &["performance", "perf", "speed", "vitals"],
    &["spacing", "whitespace", "padding", "margin"],
    &["rtl", "right-to-left", "bidi"],
    &["keyboard", "focus"],
    &["chart", "graph", "visualization", "dataviz"],
];

/// Strips one common English suffix: "targets" -> "target", "animated" -> "animat",
/// "loading" -> "load". Roots shorter than 4 letters are left alone.
fn stem(t: &str) -> String {
    for suf in ["ations", "ation", "ings", "ing", "ies", "ers", "ed", "es", "er", "ly", "s"] {
        if let Some(root) = t.strip_suffix(suf) {
            // "tapping" -> "tapp" -> "tap": undo a doubled final consonant.
            let b = root.as_bytes();
            let undoubled = matches!(suf, "ing" | "ings" | "ed" | "er" | "ers")
                && b.len() >= 4
                && b[b.len() - 1] == b[b.len() - 2]
                && !matches!(b[b.len() - 1], b's' | b'l' | b'e' | b'o');
            if undoubled {
                return root[..root.len() - 1].to_string();
            }
            if root.len() >= 4 && !(suf == "s" && root.ends_with('s')) {
                return if suf == "ies" { format!("{root}y") } else { root.to_string() };
            }
        }
    }
    t.to_string()
}

/// Multi-word names searched as one word.
const PHRASES: &[(&str, &str)] = &[
    ("screen reader", "screen-reader"),
    ("right to left", "right-to-left"),
    ("call to action", "call-to-action"),
    ("dark mode", "dark-mode"),
    ("small screen", "small-screen"),
    ("text size", "text-size"),
];

/// One query word with its alternatives: (text, weight penalty). The word and its stem cost 0,
/// synonyms cost 1.
type Term = Vec<(String, u32)>;

fn terms(query: &str) -> Vec<Term> {
    let mut q = query.to_ascii_lowercase();
    for (from, to) in PHRASES {
        q = q.replace(from, to);
    }
    q.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '.'))
        .map(|t| t.trim_matches(|c| c == '-' || c == '.'))
        .filter(|t| !t.is_empty() && !STOPWORDS.contains(t))
        .map(|t| {
            let root = stem(t);
            let mut alts: Term = vec![(t.to_string(), 0)];
            if root != t {
                alts.push((root.clone(), 0));
            }
            for group in SYNONYMS {
                if group.iter().any(|w| *w == t || *w == root || stem(w) == root) {
                    for w in group.iter() {
                        if !alts.iter().any(|(a, _)| a == w) {
                            alts.push((w.to_string(), 1));
                        }
                    }
                }
            }
            alts
        })
        .collect()
}

fn field_has(f: &str, t: &str) -> bool {
    if t.len() <= 2 {
        f.split(|c: char| !c.is_ascii_alphanumeric()).any(|w| w == t)
    } else {
        f.contains(t)
    }
}

/// Scores weighted fields: each query word earns the best weight among fields that contain it
/// or one of its alternatives (minus the alternative's penalty). Short alternatives (<= 2 chars)
/// must match a whole word to avoid noise.
fn score(fields: &[(u32, String)], terms: &[Term]) -> u32 {
    let mut total = 0;
    let mut matched = 0;
    for alts in terms {
        let best = alts
            .iter()
            .flat_map(|(t, pen)| fields.iter().filter(move |(_, f)| field_has(f, t)).map(move |(w, _)| w.saturating_sub(*pen).max(1)))
            .max()
            .unwrap_or(0);
        if best > 0 {
            matched += 1;
        }
        total += best;
    }
    if terms.len() > 1 && matched == terms.len() {
        total += 4; // every word found
    }
    total
}

fn lc(s: &str) -> String {
    s.to_ascii_lowercase()
}

/// Full-text search across guidelines, charts and stack notes.
/// - `stack`: limit stack notes to that stack and guidelines to its platforms.
/// - `platform`: `web`, `ios`, `android`, `desktop` or `mobile` (= ios + android).
pub fn search(query: &str, stack: Option<&str>, platform: Option<&str>, limit: usize) -> Vec<SearchHit> {
    let ts = terms(query);
    let limit = if limit == 0 { 10 } else { limit };
    let stack_rec = stack.and_then(find_stack);
    let mut wanted: Vec<&str> = match platform.map(lc).as_deref() {
        Some("mobile") => vec!["ios", "android"],
        Some(p) if !p.is_empty() => vec![match p {
            "web" => "web",
            "ios" => "ios",
            "android" => "android",
            "desktop" => "desktop",
            _ => "web",
        }],
        _ => vec![],
    };
    if wanted.is_empty() {
        if let Some(s) = stack_rec {
            wanted = stack_platforms(s).to_vec();
        }
    }

    let mut hits: Vec<SearchHit> = Vec::new();
    if ts.is_empty() {
        return hits;
    }

    for g in guidelines().iter().filter(|g| platform_matches(g, &wanted)) {
        let fields = vec![
            (6, lc(&g.id)),
            (5, g.keywords.iter().map(|k| lc(k)).collect::<Vec<_>>().join(" | ")),
            (4, lc(&g.category)),
            (3, lc(&g.rule)),
            (2, lc(&g.check)),
            (1, lc(&format!("{} {} {} {}", g.do_, g.dont, g.good, g.bad))),
        ];
        let s = score(&fields, &ts);
        if s > 0 {
            let sev_bonus = match g.severity.as_str() {
                "critical" => 2,
                "high" => 1,
                _ => 0,
            };
            hits.push(SearchHit {
                kind: "guideline",
                id: g.id.clone(),
                title: g.rule.clone(),
                score: s * 2 + sev_bonus,
                data: serde_json::to_value(g).unwrap_or(Value::Null),
            });
        }
    }

    // Chart advice is platform-neutral, so charts are always candidates.
    {
        for c in charts() {
            let fields = vec![
                (6, lc(&c.id)),
                (5, c.keywords.iter().map(|k| lc(k)).collect::<Vec<_>>().join(" | ")),
                (4, format!("{} {}", lc(&c.data_shape), lc(&c.task))),
                (3, lc(&format!("{} {}", c.situation, c.best))),
                (1, lc(&format!("{} {} {}", c.alternatives.join(" "), c.avoid_when, c.encoding))),
            ];
            let s = score(&fields, &ts);
            if s > 0 {
                hits.push(SearchHit {
                    kind: "chart",
                    id: c.id.clone(),
                    title: c.situation.clone(),
                    score: s * 2,
                    data: serde_json::to_value(c).unwrap_or(Value::Null),
                });
            }
        }
    }

    let stack_pool: Vec<&StackNotes> = match stack_rec {
        Some(s) => vec![s],
        None => stacks().iter().collect(),
    };
    for s in stack_pool {
        for (section, dd) in s.sections() {
            let fields = vec![
                (6, format!("{} {} {}", lc(&s.id), lc(&s.name), s.aliases.iter().map(|a| lc(a)).collect::<Vec<_>>().join(" "))),
                (4, section.to_string()),
                (2, lc(&format!("{} {}", dd.do_, dd.dont))),
            ];
            let sc = score(&fields, &ts);
            if sc > 0 {
                hits.push(SearchHit {
                    kind: "stack",
                    id: format!("{}#{}", s.id, section),
                    title: format!("{} — {}", s.name, section),
                    score: sc * 2,
                    data: serde_json::json!({ "stack": s.id, "name": s.name, "section": section, "do": dd.do_, "dont": dd.dont }),
                });
            }
        }
    }

    // When the caller named a stack, its matching notes come first; then by score.
    let pinned = |h: &SearchHit| stack_rec.is_some() && h.kind == "stack";
    hits.sort_by(|a, b| {
        pinned(b).cmp(&pinned(a)).then(b.score.cmp(&a.score)).then_with(|| a.id.cmp(&b.id))
    });
    hits.truncate(limit);
    hits
}

/// Looks up any record: a guideline id (`TOUCH-01`), chart id (`line-trend`), stack id or alias
/// (`nextjs`), a stack section (`nextjs#motion`) or `icons`.
pub fn get(id: &str) -> Option<Value> {
    let key = id.trim();
    if key.eq_ignore_ascii_case("icons") {
        return serde_json::to_value(icons()).ok();
    }
    if let Some(g) = guidelines().iter().find(|g| g.id.eq_ignore_ascii_case(key)) {
        return serde_json::to_value(g).ok();
    }
    if let Some(c) = charts().iter().find(|c| c.id.eq_ignore_ascii_case(key)) {
        return serde_json::to_value(c).ok();
    }
    let (stack_key, section) = match key.split_once('#') {
        Some((a, b)) => (a, Some(b)),
        None => (key, None),
    };
    let s = find_stack(stack_key)?;
    match section {
        None => serde_json::to_value(s).ok(),
        Some(sec) => s
            .sections()
            .into_iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(sec))
            .and_then(|(_, dd)| serde_json::to_value(dd).ok()),
    }
}

fn shape_alias(s: &str) -> String {
    let n = norm(s);
    match n.as_str() {
        "time" | "timeseries" | "temporal" | "over-time" | "trend" => "time-series".into(),
        "category" | "categories" | "nominal" | "bar" => "categorical".into(),
        "share" | "parts" | "proportion" | "composition" | "part-of-whole" => "part-to-whole".into(),
        "relationship" | "correlation" | "two-numeric" | "xy" => "correlation".into(),
        "geo" | "map" | "geographic" | "location" => "geospatial".into(),
        "tree" | "nested" => "hierarchy".into(),
        "graph" | "nodes" => "network".into(),
        "kpi" | "metric" | "single" | "number" => "single-value".into(),
        "table" => "tabular".into(),
        "range" | "interval" | "forecast" => "uncertainty".into(),
        "schedule" | "events" => "time-span".into(),
        "rank" => "ranking".into(),
        _ => n,
    }
}

/// The data shape of a free-text description ("monthly revenue over two years" -> time-series):
/// the whole phrase first, then each word, with period words read as time.
fn infer_shape(s: &str) -> String {
    let known = |sh: &str| charts().iter().any(|c| c.data_shape == sh);
    let full = shape_alias(s);
    if known(&full) {
        return full;
    }
    for w in s.to_ascii_lowercase().split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty()) {
        let root = stem(w);
        let sh = match root.as_str() {
            "hour" | "hourly" | "day" | "daily" | "week" | "weekly" | "month" | "monthly" | "quarter" | "quarterly"
            | "year" | "yearly" | "annual" | "timeline" | "history" | "historical" => "time-series".to_string(),
            "percent" | "percentage" | "breakdown" | "split" | "mix" => "part-to-whole".to_string(),
            "country" | "countries" | "region" | "postcode" | "zip" => "geospatial".to_string(),
            _ => shape_alias(&root),
        };
        if known(&sh) {
            return sh;
        }
    }
    full
}

fn task_alias(t: &str) -> String {
    let n = norm(t);
    match n.as_str() {
        "comparison" | "compare-values" => "compare".into(),
        "trends" | "over-time" => "trend".into(),
        "share" | "part-to-whole" => "composition".into(),
        "correlate" | "correlation" => "relationship".into(),
        "spread" => "distribution".into(),
        "ranking" | "order" => "rank".into(),
        "map" | "where" => "locate".into(),
        "kpi" | "track" | "live" => "monitor".into(),
        "find" | "exact" => "lookup".into(),
        _ => n,
    }
}

/// Best charts for a data shape and (optional) task, best first. Exact shape + task matches win;
/// keyword and situation matches fill in so free-text inputs ("money over months") still work.
pub fn chart_for(data_shape: &str, task: Option<&str>) -> Vec<&'static Chart> {
    let shape = infer_shape(data_shape);
    let task = task.map(task_alias).filter(|t| !t.is_empty());
    let mut scored: Vec<(u32, &'static Chart)> = charts()
        .iter()
        .filter_map(|c| {
            let text = lc(&format!("{} {} {}", c.situation, c.keywords.join(" "), c.best));
            let mut s: u32 = 0;
            if c.data_shape == shape {
                s += 10;
            } else if !shape.is_empty() && text.contains(&shape) {
                s += 3;
            }
            if let Some(t) = &task {
                if &c.task == t {
                    s += 6;
                } else if text.contains(t.as_str()) {
                    s += 2;
                }
            }
            if c.id.ends_with("-avoid") {
                s = s.saturating_sub(4); // advice entries rank below positive picks
            }
            (s > 0).then_some((s, c))
        })
        .collect();
    // Stable sort: on equal scores the table order (general-purpose charts first) decides.
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, c)| c).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn synonyms_and_stems_find_what_people_mean() {
        let top = |q: &str| search(q, None, None, 3).into_iter().map(|h| h.id).collect::<Vec<_>>();
        for (q, any_of) in [
            ("popup close button", &["OVL-05"][..]),
            ("colour contrast", &["A11Y-01"]),
            ("loading spinner", &["LEE-01", "LEE-02", "FDBK-10"]),
            ("right-to-left layout", &["I18N-02"]),
            ("typeface size", &["COPY-04"]),
            ("graph for trends", &["line-trend"]),
            ("tapping small buttons", &["TOUCH-01", "TOUCH-02"]),
            ("screen reader labels", &["A11Y-06", "OVL-02"]),
            ("navbar on phones", &["NAV-04"]),
        ] {
            let got = top(q);
            assert!(any_of.iter().any(|id| got.iter().any(|g| g == id)), "`{q}` top 3 was {got:?}, wanted one of {any_of:?}");
        }
        assert_eq!(stem("tapping"), "tap");
        assert_eq!(stem("targets"), "target");
        assert_eq!(stem("class"), "class");
        assert_eq!(stem("dialogs"), "dialog");
        // Free-text data shapes resolve through their words.
        let first = |q: &str| chart_for(q, None).first().map(|c| c.data_shape.clone());
        assert_eq!(first("monthly revenue over two years").as_deref(), Some("time-series"));
        assert_eq!(first("percentage of visitors by browser").as_deref(), Some("part-to-whole"));
        assert_eq!(first("sales per country").as_deref(), Some("geospatial"));
    }

    pub const EXPECTED_STACKS: &[&str] = &[
        "html", "react", "nextjs", "astro", "vue", "nuxt", "sveltekit", "gutenberg", "elementor", "bricks",
        "shopify", "tailwind", "react-native", "flutter", "swiftui", "compose",
    ];
    const CATEGORIES: &[&str] = &[
        "forms", "input states", "navigation", "feedback", "loading, empty and error", "modals and overlays",
        "tables", "search", "onboarding", "i18n and RTL", "touch and mobile", "performance", "accessibility",
        "content", "dark mode",
    ];

    #[test]
    fn counts_meet_targets() {
        assert!(guidelines().len() >= 150, "{} guidelines", guidelines().len());
        assert!(charts().len() >= 35, "{} charts", charts().len());
        assert_eq!(stacks().len(), EXPECTED_STACKS.len());
        assert!(icons().roles.len() >= 6 && icons().libraries.len() >= 5);
        let cats = categories();
        for c in CATEGORIES {
            let n = guidelines().iter().filter(|g| g.category == *c).count();
            assert!(n >= 8, "category {c} has only {n}");
        }
        assert_eq!(cats.len(), CATEGORIES.len(), "{:?}", cats);
        assert!(guidelines().iter().filter(|g| g.measured_by == "browser").count() >= 50);
    }

    #[test]
    fn ids_are_unique_across_tables() {
        let mut seen = HashSet::new();
        for id in guidelines().iter().map(|g| g.id.as_str()).chain(charts().iter().map(|c| c.id.as_str())).chain(stacks().iter().map(|s| s.id.as_str())) {
            assert!(seen.insert(id.to_ascii_lowercase()), "duplicate id {id}");
        }
        // No duplicated rule text either (the rivals' weak spot).
        let mut rules = HashSet::new();
        for g in guidelines() {
            assert!(rules.insert(g.rule.to_ascii_lowercase()), "duplicate rule {}", g.id);
        }
    }

    #[test]
    fn required_fields_are_filled() {
        let sev = ["critical", "high", "medium", "low"];
        let by = ["browser", "static", "manual"];
        for g in guidelines() {
            for (name, v) in [("rule", &g.rule), ("check", &g.check), ("do", &g.do_), ("dont", &g.dont), ("good", &g.good), ("bad", &g.bad)] {
                assert!(!v.trim().is_empty(), "{} missing {name}", g.id);
            }
            assert!(g.good != g.bad, "{} good == bad", g.id);
            assert!(sev.contains(&g.severity.as_str()), "{} severity {}", g.id, g.severity);
            assert!(by.contains(&g.measured_by.as_str()), "{} measured_by", g.id);
            assert!(!g.platforms.is_empty() && !g.keywords.is_empty(), "{}", g.id);
            if let Some(h) = &g.hook {
                assert!(!h.patterns.is_empty(), "{} hook without patterns", g.id);
                assert!(h.patterns.iter().chain(&h.unless).all(|p| !p.chars().any(char::is_whitespace) && p.to_ascii_lowercase() == *p), "{} hook patterns must be lower-case with no whitespace", g.id);
            }
        }
        for c in charts() {
            for v in [&c.situation, &c.best, &c.avoid_when, &c.volume, &c.encoding, &c.fallback.table, &c.fallback.aria_summary, &c.fallback.keyboard, &c.libraries] {
                assert!(!v.trim().is_empty(), "chart {} has an empty field", c.id);
            }
            assert!(!c.alternatives.is_empty(), "chart {}", c.id);
        }
        for s in stacks() {
            for (sec, dd) in s.sections() {
                assert!(!dd.do_.is_empty() && !dd.dont.is_empty(), "{}#{sec}", s.id);
            }
        }
    }

    #[test]
    fn every_supported_stack_is_present_and_resolvable() {
        for id in EXPECTED_STACKS {
            assert!(find_stack(id).is_some(), "missing stack {id}");
        }
        assert_eq!(find_stack("Next.js").unwrap().id, "nextjs");
        assert_eq!(find_stack("theme.json").unwrap().id, "gutenberg");
        assert_eq!(find_stack("Jetpack Compose").unwrap().id, "compose");
        assert_eq!(find_stack("svelte").unwrap().id, "sveltekit");
        assert!(get("nextjs#motion").is_some() && get("icons").is_some() && get("TOUCH-01").is_some());
    }

    fn top_ids(q: &str, n: usize) -> Vec<String> {
        search(q, None, None, n).into_iter().map(|h| h.id).collect()
    }

    #[test]
    fn search_returns_relevant_results() {
        let cases: &[(&str, &str)] = &[
            ("touch target size", "TOUCH-01"),
            ("modal focus trap", "OVL-01"),
            ("rtl logical properties", "I18N-02"),
            ("skeleton loading", "LEE-01"),
            ("lcp hero image", "PERF-01"),
            ("line chart trend over time", "line-trend"),
            ("autocomplete combobox", "SRCH-05"),
            ("dark mode color-scheme native controls", "DARK-03"),
        ];
        for (q, want) in cases {
            let ids = top_ids(q, 3);
            assert!(ids.iter().any(|i| i == want), "query '{q}' -> {:?}, wanted {want}", ids);
        }
        // Stack filter surfaces that stack's notes first and keeps platform-appropriate rows.
        let hits = search("reduced motion", Some("swiftui"), None, 5);
        assert_eq!(hits[0].id, "swiftui#motion", "{:?}", hits.iter().map(|h| &h.id).collect::<Vec<_>>());
        assert!(hits.iter().filter(|h| h.kind == "guideline").all(|h| h.data["platforms"].as_array().unwrap().iter().any(|p| p == "ios")));
        // Platform filter excludes web-only rows.
        assert!(search("skip link", None, Some("android"), 10).iter().all(|h| h.id != "NAV-03"));
        assert!(search("", None, None, 5).is_empty());
    }

    #[test]
    fn chart_for_picks_sensible_charts() {
        assert_eq!(chart_for("time-series", Some("trend"))[0].id, "line-trend");
        assert_eq!(chart_for("time", Some("trends"))[0].id, "line-trend");
        let share = chart_for("part-to-whole", Some("composition"));
        assert!(share.iter().take(3).any(|c| c.id == "donut-few-parts"));
        assert_eq!(chart_for("distribution", Some("distribution"))[0].id, "histogram-distribution");
        assert_eq!(chart_for("geo", Some("locate"))[0].data_shape, "geospatial");
        assert!(chart_for("zzz-unknown", None).is_empty());
    }

    #[test]
    fn hook_rules_are_opted_in() {
        let rules: Vec<_> = hook_rules().collect();
        assert!(rules.len() >= 10);
        assert!(rules.iter().any(|(_, h)| !h.default_on), "at least one opt-in-only rule");
    }
}
