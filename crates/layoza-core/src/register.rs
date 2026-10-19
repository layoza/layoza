// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
//! The register a brief speaks in: the kind of professional look its audience trusts. Every
//! catalogue entry (archetype, mechanic, motif, button) names the registers where it reads as
//! professional; the deal only offers an entry to a brief that shares one of them, so novelty
//! never costs credibility (no newspaper masthead on a flagship tech brand).

/// All registers, in the order they are reported.
pub const REGISTERS: &[&str] = &["premium", "cinematic", "corporate", "technical", "editorial", "craft", "playful", "civic"];

const WORDS: &[(&str, &[&str])] = &[
    ("premium", &["premium", "luxury", "luxe", "high-end", "flagship", "elegant", "sophisticated", "refined", "exclusive", "boutique", "bespoke", "awwwards", "award-winning"]),
    ("cinematic", &["cinematic", "immersive", "filmic", "dramatic", "spectacular", "3d", "video wall", "showreel", "launch", "architectural"]),
    ("corporate", &["b2b", "supplier", "enterprise", "corporate", "quote", "consultancy", "consulting", "firm", "company", "solutions", "services", "industrial", "manufacturer", "installation"]),
    ("technical", &["developer", "devtool", "api", "sdk", "cli", "open source", "open-source", "dashboard", "saas", "platform", "infrastructure", "data", "engineering"]),
    ("editorial", &["magazine", "journal", "editorial", "publication", "newsletter", "essay", "longread", "journalism", "zine", "blog", "studio", "portfolio", "agency", "gallery", "museum"]),
    ("craft", &["bakery", "handmade", "artisan", "craft", "local", "family-run", "farm", "cafe", "coffee", "florist", "pottery", "workshop", "restaurant"]),
    ("playful", &["playful", "fun", "kids", "children", "toy", "game", "gaming", "festival", "party", "quirky", "colourful", "colorful", "y2k", "retro"]),
    ("civic", &["council", "government", "charity", "nonprofit", "non-profit", "public service", "school", "clinic", "care", "library", "community", "volunteer", "funeral", "legal aid"]),
];

/// Registers a brief speaks in, strongest first. Never empty: a brief with no register words
/// reads as `corporate` (the safe professional default).
pub fn brief_registers(brief: &str) -> Vec<&'static str> {
    let l = brief.to_lowercase();
    let hit = |w: &str| {
        l.match_indices(w).any(|(i, _)| {
            let before = l[..i].chars().next_back().is_none_or(|c| !c.is_alphanumeric());
            let after = l[i + w.len()..].chars().next().is_none_or(|c| !c.is_alphanumeric());
            before && after
        })
    };
    let mut scored: Vec<(usize, &'static str)> = WORDS.iter().map(|(r, ws)| (ws.iter().filter(|w| hit(w)).count(), *r)).filter(|(n, _)| *n > 0).collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| REGISTERS.iter().position(|r| *r == a.1).cmp(&REGISTERS.iter().position(|r| *r == b.1))));
    let mut out: Vec<&'static str> = scored.into_iter().map(|(_, r)| r).collect();
    if out.is_empty() {
        out.push("corporate");
    }
    out
}

/// True when an entry tagged `entry` fits a brief speaking `brief`. An untagged entry fits all.
pub fn fits(entry: &[impl AsRef<str>], brief: &[&str]) -> bool {
    entry.is_empty() || entry.iter().any(|e| brief.contains(&e.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn briefs_read_their_register() {
        let led = brief_registers("premium cinematic homepage for an LED video wall supplier, B2B quotes, 3D advertising");
        assert!(led.contains(&"premium") && led.contains(&"cinematic") && led.contains(&"corporate"));
        assert!(!led.contains(&"playful") && !led.contains(&"civic"));
        assert_eq!(brief_registers("a family-run bakery in the old town")[0], "craft");
        assert_eq!(brief_registers("something nice"), vec!["corporate"]);
        assert!(fits(&["premium", "editorial"], &led));
        assert!(!fits(&["playful"], &led));
        assert!(fits(&Vec::<String>::new(), &led));
    }
}
