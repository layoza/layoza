// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
//! Uniqueness: every slot of a new look (nav, hero, each section, footer, transition, palette, type
//! pairing, archetype) is checked against the project's look ledger (`.layoza/looks.jsonl`). The
//! deal down-ranks recently used picks; this module reports, per slot, whether the pick is fresh or
//! a repeat, so a repeat is never silent.

use crate::baseline::Look;
use crate::mechanics::MechanicDeal;
use serde::{Deserialize, Serialize};

/// One slot's verdict.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SlotCheck {
    /// `nav`, `hero`, `section-1`, `footer`, `transition`, `palette`, `typography`, `archetype`;
    /// prefixed with the direction letter in a design system (`A.hero`).
    pub slot: String,
    pub id: String,
    /// `fresh`, `repeat (last used ...)`, or `unchecked (ledger off)`.
    pub status: String,
}

/// Uniqueness report for one deal or one design system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Uniqueness {
    /// `on` (checked against the ledger) or `off` (nothing was checked).
    pub ledger: String,
    /// Ledger entries a slot is compared with.
    pub window: usize,
    /// Entries actually read (fewer than `window` in a young project).
    pub entries_compared: usize,
    pub slots: Vec<SlotCheck>,
    pub fresh: usize,
    pub repeats: usize,
    /// 0-100: share of checked slots that are fresh.
    pub score: u8,
    /// One line for the agent to show the user.
    pub verdict: String,
}

fn age(at: Option<u64>) -> Option<String> {
    let then = at?;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok()?.as_secs();
    let s = now.saturating_sub(then);
    Some(match s {
        0..=59 => "just now".to_string(),
        60..=3599 => format!("{} min ago", s / 60),
        3600..=86_399 => format!("{} h ago", s / 3600),
        _ => format!("{} d ago", s / 86_400),
    })
}

/// The slot picks of one look, in page order.
pub fn slots_of(look: &Look) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut push = |k: &str, v: &Option<String>| {
        if let Some(v) = v.as_ref().filter(|v| !v.is_empty()) {
            out.push((k.to_string(), v.clone()));
        }
    };
    push("nav", &look.nav);
    push("hero", &look.hero);
    for (i, s) in look.sections.iter().enumerate() {
        push(&format!("section-{}", i + 1), &Some(s.clone()));
    }
    push("footer", &look.footer);
    push("transition", &look.transition);
    push("palette", &look.palette);
    push("typography", &look.typography);
    push("archetype", &look.archetype);
    out
}

/// True when `look` used `id` in a slot of the same kind (sections match any section or, for
/// older entries, any recorded mechanic).
fn used_in(look: &Look, kind: &str, id: &str) -> bool {
    let eq = |v: &Option<String>| v.as_deref().is_some_and(|x| x.eq_ignore_ascii_case(id));
    match kind {
        "nav" => eq(&look.nav),
        "hero" => eq(&look.hero),
        "footer" => eq(&look.footer),
        "transition" => eq(&look.transition),
        "palette" => eq(&look.palette),
        "typography" => eq(&look.typography),
        "archetype" => eq(&look.archetype),
        _ => look.sections.iter().chain(&look.mechanics).any(|x| x.eq_ignore_ascii_case(id)),
    }
}

fn kind_of(slot: &str) -> &str {
    let s = slot.rsplit('.').next().unwrap_or(slot);
    if s.starts_with("section-") {
        "section"
    } else {
        s
    }
}

/// Checks `looks` (one for a deal, three for a design system) against `recent` (the last `window`
/// ledger entries, oldest first). Directions in the same run are also checked against each other.
/// `ledger_on == false` reports every slot as unchecked.
pub fn check(looks: &[Look], recent: &[Look], window: usize, ledger_on: bool) -> Uniqueness {
    let letters = ['A', 'B', 'C', 'D', 'E'];
    let mut slots: Vec<SlotCheck> = Vec::new();
    let (mut fresh, mut repeats) = (0usize, 0usize);
    for (i, look) in looks.iter().enumerate() {
        for (slot, id) in slots_of(look) {
            let name = if looks.len() > 1 { format!("{}.{}", letters.get(i).copied().unwrap_or('X'), slot) } else { slot.clone() };
            let kind = kind_of(&slot);
            let status = if !ledger_on {
                "unchecked (ledger off)".to_string()
            } else if let Some(j) = looks[..i].iter().position(|o| used_in(o, kind, &id)) {
                repeats += 1;
                format!("repeat (also in direction {} of this run)", letters.get(j).copied().unwrap_or('X'))
            } else if let Some(back) = recent.iter().rev().position(|l| used_in(l, kind, &id)) {
                repeats += 1;
                let l = &recent[recent.len() - 1 - back];
                let when = age(l.at).map(|a| format!(", {a}")).unwrap_or_default();
                format!("repeat (last used {} entr{} ago{})", back + 1, if back == 0 { "y" } else { "ies" }, when)
            } else {
                fresh += 1;
                "fresh".to_string()
            };
            slots.push(SlotCheck { slot: name, id, status });
        }
    }
    let checked = fresh + repeats;
    let score = if checked == 0 { 0 } else { ((fresh * 100) as f64 / checked as f64).round() as u8 };
    let verdict = if !ledger_on {
        "Ledger off: no slot was checked for repeats. Uniqueness needs the ledger; leave it on unless the user asked.".to_string()
    } else if repeats == 0 {
        format!("All {} slots are fresh against the last {} ledger entries.", fresh, recent.len())
    } else {
        let names: Vec<String> = slots.iter().filter(|s| s.status.starts_with("repeat")).map(|s| format!("{} {}", s.slot, s.id)).collect();
        format!(
            "{} of {} slots repeat ({}): no fresh pick fit those slots as well. Tell the user, then re-deal with another seed/variant, `avoid` the ids, or replace them by hand.",
            repeats,
            checked,
            names.join(", ")
        )
    };
    Uniqueness { ledger: if ledger_on { "on" } else { "off" }.into(), window, entries_compared: recent.len(), slots, fresh, repeats, score, verdict }
}

impl Uniqueness {
    /// Compact text for terminals: one header line, the verdict, then only the repeated slots.
    pub fn lines(&self) -> Vec<String> {
        let mut out = vec![format!(
            "Uniqueness: {}/100 ({} fresh, {} repeat(s); ledger {}, {} of the last {} entries compared)",
            self.score, self.fresh, self.repeats, self.ledger, self.entries_compared, self.window
        )];
        out.push(format!("  {}", self.verdict));
        for s in self.slots.iter().filter(|s| !s.status.starts_with("fresh")) {
            out.push(format!("  - {} {}: {}", s.slot, s.id, s.status));
        }
        out
    }
}

/// The ledger window to use: `requested` clamped to 1..=MAX_UNIQUENESS_WINDOW, default
/// UNIQUENESS_WINDOW.
pub fn window(requested: Option<u64>) -> usize {
    requested.map(|n| (n as usize).clamp(1, crate::baseline::MAX_UNIQUENESS_WINDOW)).unwrap_or(crate::baseline::UNIQUENESS_WINDOW)
}

/// One explicit line about the ledger write: `written <path>`, `not written (<error>)` or
/// `off (<why>)`.
pub fn ledger_status<T>(root: Option<&std::path::Path>, written: Option<&Result<T, String>>, off_reason: &str) -> String {
    match (root, written) {
        (Some(r), Some(Ok(_))) => format!("written {}", r.join(crate::baseline::LOOK_LEDGER).display()),
        (Some(_), Some(Err(e))) => format!("not written ({e})"),
        _ => format!("off ({off_reason})"),
    }
}

/// The ledger entry for a bare mechanics deal: every slot it filled.
pub fn deal_look(deal: &MechanicDeal, run: &str) -> Look {
    let id = |m: &Option<crate::mechanics::Mechanic>| m.as_ref().map(|m| m.id.clone());
    Look {
        run: run.to_string(),
        source: "deal".into(),
        hero: id(&deal.hero),
        nav: id(&deal.nav),
        footer: id(&deal.footer),
        transition: id(&deal.transition),
        sections: deal.signature.iter().map(|m| m.id.clone()).collect(),
        mechanics: deal
            .hero
            .iter()
            .chain(&deal.signature)
            .chain(&deal.nav)
            .chain(&deal.footer)
            .chain(&deal.transition)
            .map(|m| m.id.clone())
            .collect(),
        ..Default::default()
    }
}

/// Every mechanic id in `recent` (oldest first), most recently used first and without duplicates,
/// for the deal's graded recent-use penalty.
pub fn recent_mechanic_ids(recent: &[Look]) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for l in recent.iter().rev() {
        for id in l.hero.iter().chain(&l.nav).chain(&l.footer).chain(&l.transition).chain(&l.sections).chain(&l.mechanics) {
            if !ids.iter().any(|x| x.eq_ignore_ascii_case(id)) {
                ids.push(id.clone());
            }
        }
    }
    ids
}

/// The mechanic ids of each entry in `recent` (oldest first), most recent entry first, for the
/// deal's per-entry graded penalty (`DealOptions::recent_by_entry`).
pub fn recent_by_entry(recent: &[Look]) -> Vec<Vec<String>> {
    recent
        .iter()
        .rev()
        .map(|l| l.hero.iter().chain(&l.nav).chain(&l.footer).chain(&l.transition).chain(&l.sections).chain(&l.mechanics).cloned().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mechanics::{deal_mechanics_with, DealOptions};

    fn look(hero: &str, nav: &str) -> Look {
        Look { run: hero.into(), hero: Some(hero.into()), nav: Some(nav.into()), ..Default::default() }
    }

    #[test]
    fn repeats_are_named_with_their_distance() {
        let recent = vec![look("H1", "N1"), look("H2", "N2")];
        let u = check(&[look("H1", "N9")], &recent, 20, true);
        assert_eq!(u.slots[0].status, "fresh");
        assert!(u.slots[1].status.starts_with("repeat (last used 2 entries ago"), "{:?}", u.slots);
        assert_eq!((u.fresh, u.repeats, u.score), (1, 1, 50));
        assert!(u.verdict.contains("hero H1"));
        let off = check(&[look("H1", "N9")], &recent, 20, false);
        assert!(off.slots.iter().all(|s| s.status.starts_with("unchecked")) && off.ledger == "off");
        // Directions of one run are checked against each other too.
        let u = check(&[look("H1", "N1"), look("H2", "N1")], &[], 20, true);
        assert!(u.slots.iter().any(|s| s.slot == "B.nav" && s.status.contains("direction A")));
    }

    /// Ten deals in a row for one brief, each recorded in the ledger: no two have the same slot set;
    /// per slot, fewer than 30% of the deals repeat, unless that slot's on-brief pool is smaller than
    /// the run, in which case every pick stays fresh until the pool is used up, a repeat never comes
    /// back within 3 deals (oldest first), and each repeat is named in the verdict.
    fn ten_in_a_row(brief: &str) -> (Vec<Vec<(String, String)>>, Vec<Uniqueness>) {
        let mut ledger: Vec<Look> = Vec::new();
        let mut sets = Vec::new();
        let mut reports = Vec::new();
        for seed in 0..10u64 {
            let recent: Vec<Look> = ledger.iter().rev().take(crate::baseline::UNIQUENESS_WINDOW).rev().cloned().collect();
            let deal = deal_mechanics_with(&DealOptions {
                brief: brief.into(),
                count: 2,
                seed: Some(seed * 7919 + 1),
                recent_ids: recent_mechanic_ids(&recent),
                recent_by_entry: recent_by_entry(&recent),
                ..Default::default()
            });
            let l = deal_look(&deal, &format!("run-{seed}"));
            reports.push(check(std::slice::from_ref(&l), &recent, crate::baseline::UNIQUENESS_WINDOW, true));
            let s = slots_of(&l);
            sets.push(s);
            ledger.push(l);
        }
        (sets, reports)
    }

    #[test]
    fn ten_deals_in_a_row_stay_unique_per_slot() {
        for brief in ["family funeral home offering cremation and memorial services", "neighbourhood sourdough bakery with a morning queue", "open source CLI for database migrations"] {
            let (sets, reports) = ten_in_a_row(brief);
            for i in 0..sets.len() {
                for j in 0..i {
                    assert_ne!(sets[i], sets[j], "{brief}: deals {j} and {i} have the same slot set");
                }
            }
            for slot in ["nav", "hero", "footer", "transition"] {
                let picks: Vec<&str> = sets.iter().map(|s| s.iter().find(|(k, _)| k == slot).map(|(_, v)| v.as_str()).unwrap_or("")).collect();
                let flags: Vec<bool> = reports.iter().map(|u| u.slots.iter().any(|s| s.slot == slot && s.status.starts_with("repeat"))).collect();
                let repeats = flags.iter().filter(|r| **r).count();
                for (u, r) in reports.iter().zip(&flags) {
                    let id = u.slots.iter().find(|s| s.slot == slot).map(|s| s.id.clone()).unwrap_or_default();
                    assert!(!r || u.verdict.contains(&id), "{brief}: repeat {slot} {id} not named: {}", u.verdict);
                }
                if repeats * 100 < 30 * reports.len() {
                    continue;
                }
                let first = flags.iter().position(|r| *r).unwrap_or(picks.len());
                let distinct: std::collections::BTreeSet<&str> = picks.iter().copied().collect();
                assert!(first >= distinct.len() && distinct.len() >= 4, "{brief}: {slot} repeated before its pool ran out: {picks:?}");
                for i in 0..picks.len() {
                    for j in i.saturating_sub(3)..i {
                        assert_ne!(picks[i], picks[j], "{brief}: {slot} came back within 3 deals: {picks:?}");
                    }
                }
            }
            // Sections draw from a wider pool: the page as a whole keeps changing.
            let sections: std::collections::BTreeSet<String> = sets.iter().flat_map(|s| s.iter().filter(|(k, _)| k.starts_with("section-")).map(|(_, v)| v.clone())).collect();
            assert!(sections.len() >= 10, "{brief}: only {} distinct sections in 10 deals", sections.len());
        }
    }
}
