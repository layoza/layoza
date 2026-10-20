// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Anti-repeat design memory: a small user-level ledger of picked directions.
//
// Designs drift toward the same motifs across projects. Each time a project's
// `.layoza/direction.md` records a pick, the verifier appends one line to
// `~/.layoza/history.jsonl` (or `$LAYOZA_HOME/history.jsonl`). The ledger holds
// only a hash of the project path, the date, the picked direction name, palette
// hexes, type families, motif keywords and mechanic ids: never page content,
// copy, paths or secrets. `get_design_system` reads it back as `avoid_recent`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Entries `avoid_recent` and the repeat check look back over.
pub const RECENT_WINDOW: usize = 8;
const LEDGER_FILE: &str = "history.jsonl";
/// Above this size the ledger is compacted to its newest [`KEEP_ON_COMPACT`] lines.
const MAX_LEDGER_BYTES: u64 = 256 * 1024;
const KEEP_ON_COMPACT: usize = 200;
const MAX_DIRECTION_CHARS: usize = 80;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// FNV-1a hash of the canonical project directory; the path itself is never stored.
    pub project: String,
    /// UTC date, `YYYY-MM-DD`.
    pub date: String,
    pub direction: String,
    #[serde(default)]
    pub palette: Vec<String>,
    #[serde(default)]
    pub fonts: Vec<String>,
    #[serde(default)]
    pub motifs: Vec<String>,
    #[serde(default)]
    pub mechanics: Vec<String>,
}

/// Directory holding the ledger: `$LAYOZA_HOME`, else `<home>/.layoza`.
/// `LAYOZA_NO_HISTORY=1` disables the ledger. Unit tests never touch the real home.
pub fn layoza_home() -> Option<PathBuf> {
    if std::env::var_os("LAYOZA_NO_HISTORY").is_some_and(|v| !v.is_empty() && v != "0") {
        return None;
    }
    if let Some(dir) = std::env::var_os("LAYOZA_HOME").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(dir));
    }
    if cfg!(test) {
        return Some(std::env::temp_dir().join(format!("layoza-test-home-{}", std::process::id())));
    }
    crate::installer::resolve_user_home().map(|h| h.join(".layoza"))
}

pub fn ledger_path() -> Option<PathBuf> {
    layoza_home().map(|h| h.join(LEDGER_FILE))
}

/// Stable 64-bit FNV-1a hex digest (std's hasher is not stable across releases).
pub fn fnv1a_hex(s: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{:016x}", h)
}

pub fn project_hash(dir: &Path) -> String {
    let canon = fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    fnv1a_hex(&canon.to_string_lossy().replace('\\', "/").to_lowercase())
}

/// UTC `YYYY-MM-DD` without a date crate (Howard Hinnant's civil-from-days).
pub fn today_utc() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let z = secs.div_euclid(86_400) + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

/// Splits `direction.md` into `KEY -> text` blocks. Keys are the uppercase labels the
/// art-direction skill writes (`PURPOSE:`, `- **TOKENS:** ...`); lines that are not a
/// key continue the previous block. Values keep their original case.
pub fn parse_direction(text: &str) -> BTreeMap<String, String> {
    const KEYS: &[&str] = &[
        "PURPOSE", "PICKED", "THESIS", "OWN-WORLD", "TOKENS", "FONTS", "FIRST VIEWPORT", "SECTIONS",
        "ART PLAN", "SIGNATURE", "QUIET", "MOTION", "FINISH", "REVIEWER", "REPEAT-OK", "ROUND",
    ];
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    let mut current: Option<String> = None;
    for raw in text.lines() {
        let line = raw.replace('*', "");
        let line = line.trim().trim_start_matches(['-', '#', ' ']);
        let upper = line.to_ascii_uppercase();
        let hit = KEYS.iter().find_map(|k| {
            let rest = upper.strip_prefix(k)?;
            // `ROUND 2:` keeps its number in the key; other keys need the colon right away.
            let colon = rest.find(':')?;
            let between = rest[..colon].trim();
            let ok = if *k == "ROUND" { between.chars().all(|c| c.is_ascii_digit()) && !between.is_empty() } else { between.is_empty() };
            ok.then(|| (format!("{}{}", k, if *k == "ROUND" { format!(" {}", between) } else { String::new() }), line[k.len() + colon + 1..].trim().to_string()))
        });
        match hit {
            Some((key, value)) => {
                let slot = out.entry(key.clone()).or_default();
                if !slot.is_empty() {
                    slot.push('\n');
                }
                slot.push_str(&value);
                current = Some(key);
            }
            None => {
                if let (Some(k), false) = (&current, line.is_empty()) {
                    let slot = out.entry(k.clone()).or_default();
                    slot.push('\n');
                    slot.push_str(line);
                }
            }
        }
    }
    out
}

fn block(blocks: &BTreeMap<String, String>, keys: &[&str]) -> String {
    keys.iter().filter_map(|k| blocks.get(*k)).cloned().collect::<Vec<_>>().join("\n")
}

/// Every `#rgb` / `#rrggbb` hex in `text`, lowercased and expanded, de-duplicated in order.
pub fn hexes(text: &str) -> Vec<String> {
    let b = text.as_bytes();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'#' && (i == 0 || !b[i - 1].is_ascii_alphanumeric()) {
            let mut j = i + 1;
            while j < b.len() && b[j].is_ascii_hexdigit() {
                j += 1;
            }
            let len = j - i - 1;
            let boundary = j == b.len() || !b[j].is_ascii_alphanumeric();
            if boundary && (len == 3 || len == 6) {
                let h = text[i + 1..j].to_ascii_lowercase();
                let full = if len == 3 { h.chars().flat_map(|c| [c, c]).collect() } else { h };
                let v = format!("#{}", full);
                if !out.contains(&v) {
                    out.push(v);
                }
            }
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

/// True when `needle` occurs in `hay` (both lowercase) on word boundaries.
fn has_term(hay: &str, needle: &str) -> bool {
    let hb = hay.as_bytes();
    let mut start = 0;
    while let Some(pos) = hay[start..].find(needle) {
        let s = start + pos;
        let e = s + needle.len();
        let before_ok = s == 0 || !hb[s - 1].is_ascii_alphanumeric();
        let after_ok = e == hb.len() || !hb[e].is_ascii_alphanumeric();
        if before_ok && after_ok {
            return true;
        }
        start = e;
    }
    false
}

/// Recurring visual motifs worth tracking across projects. Matching is on word boundaries.
pub const MOTIF_VOCAB: &[&str] = &[
    "bento", "marquee", "ticker", "editorial", "magazine", "brutalist", "swiss", "poster", "collage",
    "scrapbook", "sticker", "zine", "terminal", "blueprint", "schematic", "diagram", "map", "chart",
    "ticket", "receipt", "label", "stamp", "polaroid", "film", "grain", "noise", "halftone", "risograph",
    "gradient", "mesh gradient", "glass", "glassmorphism", "aurora", "glow", "neon", "chrome",
    "holographic", "paper", "cream", "kraft", "monochrome", "duotone", "split screen", "asymmetric",
    "oversized type", "giant type", "kinetic type", "serif display", "handwritten", "outline type",
    "card stack", "carousel", "parallax", "sticky", "horizontal scroll", "scrollytelling", "3d",
    "webgl", "particles", "isometric", "line art", "full-bleed", "dark mode", "numbered sections",
    "index", "catalogue", "archive", "timeline", "ledger", "chalkboard", "neon sign", "blob",
    "noise texture", "pixel", "retro", "y2k", "bauhaus", "art deco", "grid", "masonry",
];

/// Well-known web font families not covered by the industry catalogue.
const EXTRA_FONTS: &[&str] = &[
    "Inter", "Roboto", "Open Sans", "Lato", "Montserrat", "Poppins", "Raleway", "Nunito", "Work Sans",
    "DM Sans", "DM Serif Display", "DM Mono", "Space Grotesk", "Space Mono", "IBM Plex Sans", "IBM Plex Serif",
    "IBM Plex Mono", "JetBrains Mono", "Fraunces", "Playfair Display", "Cormorant Garamond", "Cormorant",
    "EB Garamond", "Libre Baskerville", "Lora", "Merriweather", "Source Serif", "Instrument Serif",
    "Instrument Sans", "Syne", "Manrope", "Outfit", "Sora", "Plus Jakarta Sans", "Archivo", "Bricolage Grotesque",
    "Unbounded", "Bebas Neue", "Anton", "Oswald", "Familjen Grotesk", "Hanken Grotesk", "Schibsted Grotesk",
    "Newsreader", "Spectral", "Bodoni Moda", "Jost", "Recoleta", "Clash Display", "Satoshi", "General Sans",
    "Geist", "Geist Mono", "Figtree", "Onest", "Epilogue", "Rubik", "Karla", "Barlow", "Chivo", "Caveat",
];

fn font_vocab() -> Vec<String> {
    let mut v: Vec<String> = EXTRA_FONTS.iter().map(|s| s.to_string()).collect();
    for r in crate::intelligence::catalog_industries() {
        let t = r.typography_pairing;
        for f in [t.display_font, t.body_font, t.mono_font] {
            let f = f.trim().to_string();
            if !f.is_empty() && !v.iter().any(|x| x.eq_ignore_ascii_case(&f)) {
                v.push(f);
            }
        }
    }
    // Longest first so "IBM Plex Sans" wins over a shorter overlapping name.
    v.sort_by_key(|f| std::cmp::Reverse(f.len()));
    v
}

/// Font families named in `text`: an explicit `FONTS:` list plus known family names.
pub fn fonts_in(text: &str, explicit: Option<&str>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Some(list) = explicit {
        for f in list.split([',', '/', ';', '\n', '+']) {
            let f = f.split('(').next().unwrap_or("").trim().trim_matches(['`', '"', '\'']);
            if f.len() >= 2 && !out.iter().any(|x: &String| x.eq_ignore_ascii_case(f)) {
                out.push(f.to_string());
            }
        }
    }
    let lower = text.to_lowercase();
    for f in font_vocab() {
        let fl = f.to_lowercase();
        if has_term(&lower, &fl) && !out.iter().any(|x| x.eq_ignore_ascii_case(&f) || x.to_lowercase().contains(&fl)) {
            out.push(f);
        }
    }
    out
}

pub fn motifs_in(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    MOTIF_VOCAB.iter().filter(|m| has_term(&lower, m)).map(|m| m.to_string()).collect()
}

pub fn mechanics_in(text: &str) -> Vec<String> {
    crate::mechanics::catalogue()
        .iter()
        .filter(|m| has_term(text, &m.id))
        .map(|m| m.id.clone())
        .collect()
}

/// Keeps the direction label short and drops anything that looks like a URL or address.
fn clean_direction(picked: &str) -> String {
    let words: Vec<&str> = picked.split_whitespace().filter(|w| !w.contains("://") && !w.contains('@')).collect();
    words.join(" ").chars().take(MAX_DIRECTION_CHARS).collect()
}

/// Builds the ledger entry for a project's `direction.md`. `None` when there is no real
/// pick to remember: an empty `PICKED:` or an existing site (`PICKED: existing — ...`),
/// which follows its own design rather than a new direction.
pub fn entry_from_direction(project_dir: &Path, text: &str) -> Option<HistoryEntry> {
    let blocks = parse_direction(text);
    let picked = blocks.get("PICKED")?.lines().next().unwrap_or("").trim().to_string();
    if picked.is_empty() || picked.to_ascii_lowercase().starts_with("existing") {
        return None;
    }
    let style = block(&blocks, &["PICKED", "THESIS", "OWN-WORLD", "TOKENS", "FONTS"]);
    let mut palette = hexes(&block(&blocks, &["TOKENS", "OWN-WORLD", "PICKED"]));
    if palette.is_empty() {
        palette = hexes(text);
    }
    palette.truncate(8);
    let mut fonts = fonts_in(&style, blocks.get("FONTS").map(|s| s.as_str()));
    fonts.truncate(4);
    let motifs = motifs_in(&block(
        &blocks,
        &["PICKED", "THESIS", "OWN-WORLD", "FIRST VIEWPORT", "SECTIONS", "ART PLAN", "MOTION"],
    ));
    Some(HistoryEntry {
        project: project_hash(project_dir),
        date: today_utc(),
        direction: clean_direction(&picked),
        palette,
        fonts,
        motifs,
        mechanics: mechanics_in(text),
    })
}

/// Reads the ledger, newest last. Unreadable lines are skipped; a missing ledger is empty.
pub fn read_all(path: &Path) -> Vec<HistoryEntry> {
    let Ok(raw) = fs::read_to_string(path) else {
        return Vec::new();
    };
    raw.lines().filter_map(|l| serde_json::from_str::<HistoryEntry>(l.trim()).ok()).collect()
}

/// The newest entry per project, newest last, at most `n`, optionally excluding one project.
pub fn recent(path: &Path, n: usize, exclude_project: Option<&str>) -> Vec<HistoryEntry> {
    let mut seen: Vec<String> = Vec::new();
    let mut out: Vec<HistoryEntry> = Vec::new();
    for e in read_all(path).into_iter().rev() {
        if Some(e.project.as_str()) == exclude_project || seen.contains(&e.project) {
            continue;
        }
        seen.push(e.project.clone());
        out.push(e);
        if out.len() == n {
            break;
        }
    }
    out.reverse();
    out
}

fn same_design(a: &HistoryEntry, b: &HistoryEntry) -> bool {
    a.direction == b.direction && a.palette == b.palette && a.fonts == b.fonts && a.motifs == b.motifs && a.mechanics == b.mechanics
}

/// Appends `entry` unless that project's newest entry already records the same design,
/// so re-running `audit` / `verify` does not flood the ledger. Returns whether a line was written.
pub fn record(path: &Path, entry: &HistoryEntry) -> std::io::Result<bool> {
    let all = read_all(path);
    if all.iter().rev().find(|e| e.project == entry.project).is_some_and(|e| same_design(e, entry)) {
        return Ok(false);
    }
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    if fs::metadata(path).map(|m| m.len() > MAX_LEDGER_BYTES).unwrap_or(false) {
        let keep = &all[all.len().saturating_sub(KEEP_ON_COMPACT)..];
        let body: String = keep.iter().filter_map(|e| serde_json::to_string(e).ok()).map(|l| l + "\n").collect();
        fs::write(path, body)?;
    }
    let mut f = fs::OpenOptions::new().create(true).append(true).open(path)?;
    let line = serde_json::to_string(entry).map_err(std::io::Error::other)?;
    f.write_all(format!("{}\n", line).as_bytes())?;
    Ok(true)
}

fn rgb(hex: &str) -> Option<(i32, i32, i32)> {
    let h = hex.strip_prefix('#')?;
    if h.len() != 6 {
        return None;
    }
    let p = |i: usize| i32::from_str_radix(&h[i..i + 2], 16).ok();
    Some((p(0)?, p(2)?, p(4)?))
}

/// Share of `a`'s colours that have a near match (RGB distance < 28) in `b`.
pub fn palette_overlap(a: &[String], b: &[String]) -> f64 {
    let a: Vec<_> = a.iter().filter_map(|h| rgb(h)).collect();
    let b: Vec<_> = b.iter().filter_map(|h| rgb(h)).collect();
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let near = a
        .iter()
        .filter(|(r, g, bl)| b.iter().any(|(r2, g2, b2)| ((r - r2).pow(2) + (g - g2).pow(2) + (bl - b2).pow(2)) < 28 * 28))
        .count();
    near as f64 / a.len() as f64
}

fn shared(a: &[String], b: &[String]) -> Vec<String> {
    a.iter().filter(|x| b.iter().any(|y| y.eq_ignore_ascii_case(x))).cloned().collect()
}

/// How closely `current` repeats `past`: the matching signals, empty when it does not.
/// A repeat needs two of: palette (half the colours near-identical), type families
/// (at least half shared), motifs (2+ shared, Jaccard >= 0.4), mechanics (2+ shared),
/// and one of the two must be palette or type.
pub fn repeat_signals(current: &HistoryEntry, past: &HistoryEntry) -> Vec<String> {
    let mut signals = Vec::new();
    let pal = palette_overlap(&current.palette, &past.palette);
    if current.palette.len() >= 2 && pal >= 0.5 {
        signals.push(format!("palette ({:.0}% of colours near-identical)", pal * 100.0));
    }
    let f = shared(&current.fonts, &past.fonts);
    if !f.is_empty() && f.len() * 2 >= current.fonts.len().max(past.fonts.len()) {
        signals.push(format!("type families ({})", f.join(", ")));
    }
    let m = shared(&current.motifs, &past.motifs);
    let union = current.motifs.len() + past.motifs.len() - m.len();
    if m.len() >= 2 && union > 0 && m.len() as f64 / union as f64 >= 0.4 {
        signals.push(format!("motifs ({})", m.join(", ")));
    }
    let mech = shared(&current.mechanics, &past.mechanics);
    if mech.len() >= 2 {
        signals.push(format!("mechanics ({})", mech.join(", ")));
    }
    let visual = signals.iter().any(|s| s.starts_with("palette") || s.starts_with("type"));
    if signals.len() >= 2 && visual {
        signals
    } else {
        Vec::new()
    }
}

fn by_frequency<'a>(items: impl Iterator<Item = &'a String>) -> Vec<String> {
    let mut counts: Vec<(String, usize)> = Vec::new();
    for it in items {
        match counts.iter_mut().find(|(k, _)| k.eq_ignore_ascii_case(it)) {
            Some((_, c)) => *c += 1,
            None => counts.push((it.clone(), 1)),
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    counts.into_iter().map(|(k, _)| k).collect()
}

/// The `avoid_recent` block for a new design: the last [`RECENT_WINDOW`] picked
/// directions (one per project) and the palettes, fonts, motifs and mechanics they used.
pub fn avoid_recent_at(path: &Path) -> serde_json::Value {
    let recent = recent(path, RECENT_WINDOW, None);
    serde_json::json!({
        "entries": recent.len(),
        "directions": recent.iter().rev().map(|e| serde_json::json!({ "date": e.date, "direction": e.direction })).collect::<Vec<_>>(),
        "palettes": recent.iter().rev().filter(|e| !e.palette.is_empty()).map(|e| e.palette.clone()).collect::<Vec<_>>(),
        "fonts": by_frequency(recent.iter().flat_map(|e| e.fonts.iter())),
        "motifs": by_frequency(recent.iter().flat_map(|e| e.motifs.iter())),
        "mechanics": by_frequency(recent.iter().flat_map(|e| e.mechanics.iter())),
        "rule": "Make all three directions materially different from these recent picks: a different palette family, different type families, different motifs and mechanics. Reuse them only when the user asks for the same look, and then record `REPEAT-OK: <the user's words>` in direction.md.",
    })
}

/// [`avoid_recent_at`] for the user's ledger; an empty block when the ledger is disabled.
pub fn avoid_recent() -> serde_json::Value {
    match ledger_path() {
        Some(p) => avoid_recent_at(&p),
        None => serde_json::json!({ "entries": 0, "note": "History is disabled (LAYOZA_NO_HISTORY) or no home directory was found." }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("layoza-history-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    const DIRECTION: &str = "PURPOSE: book tables at a harbour seafood bar; audience: locals; new\n\
        - **PICKED:** B — Harbour chart (chosen by the user)\n\
        THESIS: the daily catch board becomes a tide chart\n\
        OWN-WORLD: blueprint paper, ticket stubs, grain\n\
        TOKENS: ink #0f1b2d, sea #1d6f8a, paper #f4efe6, buoy #e2552b; Fraunces display, Inter body\n\
        SECTIONS:\n- Menu: chalkboard of the catch\n- Map: harbour map with the bar pinned\n";

    #[test]
    fn parses_blocks_and_extracts_only_design_signals() {
        let blocks = parse_direction(DIRECTION);
        assert_eq!(blocks["PICKED"], "B — Harbour chart (chosen by the user)");
        assert!(blocks["SECTIONS"].contains("harbour map"));
        let dir = tmp("extract");
        let e = entry_from_direction(&dir, DIRECTION).unwrap();
        assert_eq!(e.palette, vec!["#0f1b2d", "#1d6f8a", "#f4efe6", "#e2552b"]);
        assert!(e.fonts.contains(&"Fraunces".to_string()) && e.fonts.contains(&"Inter".to_string()), "{:?}", e.fonts);
        for m in ["blueprint", "ticket", "grain", "chalkboard", "map", "chart"] {
            assert!(e.motifs.contains(&m.to_string()), "{} missing from {:?}", m, e.motifs);
        }
        assert_eq!(e.project.len(), 16);
        assert_eq!(e.date.len(), 10);
        // "Inter" must not match inside "interface".
        assert!(fonts_in("an interface for internal use", None).is_empty());
        // Existing sites and empty picks are not remembered.
        assert!(entry_from_direction(&dir, "PICKED: existing — follows SITE-DNA.json\n").is_none());
        assert!(entry_from_direction(&dir, "PICKED:\n").is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn records_once_per_design_and_flags_repeats_across_projects() {
        let home = tmp("ledger");
        let ledger = home.join("history.jsonl");
        let a = tmp("proj-a");
        let b = tmp("proj-b");
        let ea = entry_from_direction(&a, DIRECTION).unwrap();
        assert!(record(&ledger, &ea).unwrap());
        assert!(!record(&ledger, &ea).unwrap(), "identical re-record must be skipped");
        assert_eq!(read_all(&ledger).len(), 1);

        let similar = DIRECTION.replace("#1d6f8a", "#1e708b").replace("Harbour chart", "Tide table");
        let eb = entry_from_direction(&b, &similar).unwrap();
        let past = recent(&ledger, RECENT_WINDOW, Some(&eb.project));
        assert_eq!(past.len(), 1);
        let signals = repeat_signals(&eb, &past[0]);
        assert!(signals.iter().any(|s| s.starts_with("palette")), "{:?}", signals);
        assert!(signals.iter().any(|s| s.starts_with("type")), "{:?}", signals);

        let distinct = "PICKED: A — Night market\nTOKENS: #120a24, #ff3fa4, #ffe95c; Syne display, Space Mono body\nOWN-WORLD: neon sign, halftone\n";
        let ec = entry_from_direction(&b, distinct).unwrap();
        assert!(repeat_signals(&ec, &past[0]).is_empty());

        assert!(record(&ledger, &eb).unwrap());
        let block = avoid_recent_at(&ledger);
        assert_eq!(block["entries"], 2);
        assert_eq!(block["directions"][0]["direction"], "B — Tide table (chosen by the user)");
        assert_eq!(block["fonts"][0], "Fraunces");
        // The ledger never stores the project path or page copy.
        let raw = fs::read_to_string(&ledger).unwrap();
        assert!(!raw.contains("layoza-history-proj") && !raw.contains("book tables"));
        for d in [home, a, b] {
            let _ = fs::remove_dir_all(d);
        }
    }

    #[test]
    fn civil_date_is_well_formed() {
        let d = today_utc();
        assert!(d.starts_with("20") && d.as_bytes()[4] == b'-' && d.as_bytes()[7] == b'-', "{}", d);
    }
}
