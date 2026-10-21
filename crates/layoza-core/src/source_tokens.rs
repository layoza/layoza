//! Source-declaration evidence, not computed cascade truth. Unknown values stay absent.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub(crate) struct SourceTokens {
    pub spacing: BTreeMap<String, String>,
    pub radii: BTreeMap<String, String>,
    pub shadows: BTreeMap<String, String>,
    pub breakpoints: BTreeMap<String, String>,
    pub durations: BTreeSet<String>,
    pub easings: BTreeSet<String>,
    pub evidence: BTreeMap<String, String>,
}

impl SourceTokens {
    pub fn scan(&mut self, source: &str, text: &str) {
        let mut css = String::new();
        if ["css", "scss", "sass", "less"].iter().any(|e| source.ends_with(&format!(".{e}"))) { css.push_str(text); }
        else {
            let mut tail = text;
            while let Some(i) = tail.find("<style") {
                let Some(start) = tail[i..].find('>') else { break; };
                tail = &tail[i + start + 1..];
                let Some(end) = tail.find("</style>") else { break; };
                css.push_str(&tail[..end]); css.push('\n'); tail = &tail[end + 8..];
            }
        }
        // Ignore CSS comments: examples and old declarations are not observed active values.
        while let Some(i) = css.find("/*") {
            let end = css[i + 2..].find("*/").map(|n| i + n + 4).unwrap_or(css.len());
            css.replace_range(i..end, " ");
        }
        for (offset, _) in css.match_indices('@') {
            let rest = &css[offset..];
            if !rest.starts_with("@media") { continue; }
            let head = rest.split('{').next().unwrap_or(rest);
            for prop in ["min-width", "max-width"] {
                if let Some((_, value)) = head.split_once(&format!("{prop}:")) {
                    let value = value.split(')').next().unwrap_or(value).trim();
                    if !value.is_empty() { self.breakpoints.insert(format!("{prop}-{value}"), value.into()); self.evidence.insert(format!("breakpoint:{prop}-{value}"), source.into()); }
                }
            }
        }
        let css = css.replace('{', "{;").replace('}', ";};");
        let mut scopes: Vec<String> = Vec::new();
        for segment in css.split(';') {
            let segment = segment.trim();
            if let Some(selector) = segment.strip_suffix('{') { scopes.push(selector.trim().into()); continue; }
            if segment == "}" { scopes.pop(); continue; }
            let scope = scopes.join(" > ");
            let declaration = segment;
            let Some((name, value)) = declaration.trim().trim_start_matches('}').split_once(':') else { continue; };
            let name = name.trim(); let value = value.split('}').next().unwrap_or(value).trim();
            if name.is_empty() || name.contains(char::is_whitespace) || value.is_empty() { continue; }
            let lower = name.to_lowercase();
            let mode = if scope.contains("dark") { "@dark" } else if scope.contains("compact") { "@compact" } else { "" };
            let key = format!("{}{mode}", name.trim_start_matches("--"));
            let field = if lower.starts_with("--") && ["space", "spacing", "gap", "padding"].iter().any(|s| lower.contains(s)) {
                Some((&mut self.spacing, "spacing"))
            } else if lower == "border-radius" || (lower.starts_with("--") && lower.contains("radius")) {
                Some((&mut self.radii, "radius"))
            } else if lower == "box-shadow" || (lower.starts_with("--") && lower.contains("shadow")) {
                Some((&mut self.shadows, "shadow"))
            } else if ["gap", "row-gap", "column-gap", "padding", "padding-block", "padding-inline", "margin-block"].contains(&lower.as_str()) {
                Some((&mut self.spacing, "spacing"))
            } else { None };
            if let Some((map, kind)) = field {
                // Repeated selector-specific declarations keep their source scope instead of
                // pretending one of them is the project's global token.
                let k = if map.get(&key).is_some_and(|old| old != value) { format!("{key} [{}]", scope.trim()) } else { key.clone() };
                map.insert(k.clone(), value.into()); self.evidence.insert(format!("{kind}:{k}"), format!("{source} ({scope})"));
            }
            if lower.contains("transition") || lower.contains("animation") || lower.contains("duration") || lower.contains("ease") {
                for token in value.split(|c: char| c.is_whitespace() || c == ',') {
                    let numeric = token.strip_suffix("ms").or_else(|| token.strip_suffix('s'));
                    if numeric.is_some_and(|n| n.parse::<f64>().is_ok()) { self.durations.insert(token.into()); }
                    if ["linear", "ease", "ease-in", "ease-out", "ease-in-out", "step-start", "step-end"].contains(&token) { self.easings.insert(token.into()); }
                }
                for prefix in ["cubic-bezier(", "steps(", "linear("] {
                    if let Some(i) = value.find(prefix) { if let Some(end) = value[i..].find(')') { self.easings.insert(value[i..i + end + 1].into()); } }
                }
            }
        }
    }
}

pub(crate) fn component_details(text: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut variants = BTreeSet::new(); let mut props = BTreeSet::new(); let mut patterns = BTreeSet::new();
    for part in text.split([',', ';', '\n', '{', '}']) {
        let Some((key, value)) = part.trim().split_once(':') else { continue; };
        let key = key.trim().trim_end_matches('?').trim_matches(['"', '\'']);
        if !key.is_empty() && key.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            if ["string", "boolean", "number", "ReactNode"].iter().any(|t| value.contains(t)) { props.insert(key.to_string()); }
            if ["variant", "size", "intent", "density"].contains(&key) {
                for q in value.split(['"', '\'']).skip(1).step_by(2) { if q.len() < 40 { variants.insert(format!("{key}:{q}")); } }
            }
        }
    }
    for attr in ["className=\"", "class=\""] { for (_, rest) in text.match_indices(attr).map(|(i, _)| (i, &text[i + attr.len()..])) {
        if let Some(end) = rest.find('"') { for c in rest[..end].split_whitespace().take(20) { props.insert(format!("class:{c}")); } }
    }}
    for (needle, label) in [("<button", "native-button"), ("<dialog", "native-dialog"), ("aria-expanded", "expanded-state"), ("aria-describedby", "described-field"), ("focus-visible", "focus-visible"), ("role=\"tab", "tabs"), ("<table", "semantic-table")] {
        if text.contains(needle) { patterns.insert(label.to_string()); }
    }
    (variants.into_iter().collect(), props.into_iter().collect(), patterns.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_actual_declared_geometry_and_motion_without_defaults() {
        let mut t = SourceTokens::default();
        t.scan("site.css", ":root{--spacing-small:7px;--spacing-large:23px;--radius:0px;--shadow:none}button{border-radius:0;transition:transform 713ms linear}@media(min-width:913px){main{padding:23px}}[data-theme=dark]{--radius:2px}");
        assert_eq!(t.spacing["spacing-small"], "7px");
        assert_eq!(t.radii["radius"], "0px");
        assert_eq!(t.radii["radius@dark"], "2px");
        assert_eq!(t.shadows["shadow"], "none");
        assert_eq!(t.breakpoints["min-width-913px"], "913px");
        assert!(t.durations.contains("713ms")); assert!(t.easings.contains("linear"));
        assert!(SourceTokens::default().spacing.is_empty());
    }
}
