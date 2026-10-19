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

/// Every mechanic id in `recent`, for the deal's recent-use penalty.
pub fn recent_mechanic_ids(recent: &[Look]) -> Vec<String> {
    let mut ids: Vec<String> = recent
        .iter()
        .flat_map(|l| l.hero.iter().chain(&l.nav).chain(&l.footer).chain(&l.transition).chain(&l.sections).chain(&l.mechanics).cloned())
        .collect();
    ids.sort();
    ids.dedup();
    ids
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

    /// Ten deals in a row for one brief, each recorded in the ledger: no two have the same slot set,
    /// and no slot repeats in more than 30% of the deals.
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
            for slot in ["nav", "hero", "section-1", "section-2", "footer", "transition"] {
                let repeats = reports.iter().filter(|u| u.slots.iter().any(|s| s.slot == slot && s.status.starts_with("repeat"))).count();
                assert!(repeats * 100 < 30 * reports.len(),"{brief}: slot {slot} repeated in {repeats}/10 deals");
            }
        }
    }
}
