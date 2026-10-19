// Tests for reading unknown-trade briefs: their own nouns, verbs and a brief-seeded deal.
use super::*;

const UNUSUAL: &[&str] = &[
    "funeral home helping families arrange services",
    "municipal water utility where residents report leaks and pay bills",
    "beekeeping supply co-op selling hive frames and smokers",
    "county records archive of land deeds and maps",
    "community tool library lending drills and ladders",
    "chimney sweep with soot-blackened brushes",
    "lighthouse keepers' heritage society",
    "scrap metal yard buying copper wire",
];

fn deal(b: &str) -> MechanicDeal {
    deal_mechanics_with(&DealOptions { brief: b.to_string(), count: 2, ..Default::default() })
}

#[test]
fn brief_objects_are_the_brief_s_own_nouns() {
    assert_eq!(brief_objects("funeral home helping families arrange services"), vec!["funeral home", "families", "services"]);
    assert_eq!(brief_objects("municipal water utility where residents report leaks and pay bills")[0], "municipal water utility");
    assert_eq!(brief_objects("community tool library lending drills and ladders"), vec!["community tool library", "drills", "ladders"]);
    // Page-type filler and quality adjectives never become objects.
    assert_eq!(brief_objects("a modern, clean website for a chimney sweep"), vec!["chimney sweep"]);
}

#[test]
fn the_call_to_action_comes_from_the_brief_s_verbs_or_is_absent() {
    assert_eq!(brief_action("municipal water utility where residents report leaks and pay bills").as_deref(), Some("report leaks"));
    assert_eq!(brief_action("funeral home helping families arrange services").as_deref(), Some("arrange services"));
    assert_eq!(brief_action("Book a table at our harbour bistro").as_deref(), Some("book table"));
    // A noun modifier is not an action, and no verb means no guessed booking.
    assert_eq!(brief_action("violin repair workshop"), None);
    assert_eq!(brief_action("county records archive of land deeds and maps"), None);
}

#[test]
fn unknown_trades_get_their_own_deal_not_one_fixed_fallback() {
    let deals: Vec<MechanicDeal> = UNUSUAL.iter().map(|b| deal(b)).collect();
    for (b, d) in UNUSUAL.iter().zip(&deals) {
        assert!(d.context.niches.is_empty(), "{b} should name no catalogue niche: {:?}", d.context.niches);
        assert!(!d.brief_objects.is_empty(), "{b}: no objects read");
        assert!(d.notes.iter().any(|n| n.contains(&d.brief_objects[0])), "{b}: guidance must name the brief's objects");
        assert!(d.rules[0].contains("brief_objects"));
    }
    // Pairwise different: the whole first screen (hero + signature) and the objects it is built around.
    for i in 0..deals.len() {
        for j in i + 1..deals.len() {
            let key = |d: &MechanicDeal| (d.hero.as_ref().map(|h| h.id.clone()), d.signature.iter().map(|m| m.id.clone()).collect::<Vec<_>>());
            assert_ne!(key(&deals[i]), key(&deals[j]), "{} vs {}", UNUSUAL[i], UNUSUAL[j]);
            assert_ne!(deals[i].brief_objects, deals[j].brief_objects);
        }
    }
    // The hero is drawn from every broad hero by a hash of the whole brief, not from a 4-wide
    // score window. 8 pairwise-different heroes need at least 8 broad heroes; the catalogue has 7
    // at bold, so this asserts a spread (raise it to 8 once the catalogue grows).
    let heroes: BTreeSet<String> = deals.iter().filter_map(|d| d.hero.as_ref().map(|h| h.id.clone())).collect();
    assert!(heroes.len() >= 4, "heroes collapsed onto {:?}", heroes);
}

#[test]
fn a_shared_niche_signal_is_used_verbatim() {
    let d = deal_mechanics_with(&DealOptions {
        brief: "vineyard wedding".into(),
        niches: Some(vec!["events".into()]),
        count: 2,
        ..Default::default()
    });
    assert_eq!(d.context.niches, vec!["events".to_string()]);
}

#[test]
fn recent_heroes_are_steered_away_from() {
    let first = deal("funeral home helping families arrange services");
    let hero = first.hero.unwrap().id;
    let again = deal_mechanics_with(&DealOptions {
        brief: "funeral home helping families arrange services".into(),
        count: 2,
        recent_ids: vec![hero.clone()],
        ..Default::default()
    });
    assert_ne!(again.hero.unwrap().id, hero);
}

#[test]
fn seeds_serialise_as_strings_and_read_either_form() {
    let d = deal_mechanics_with(&DealOptions { brief: "chimney sweep".into(), count: 2, seed: Some(u64::MAX), ..Default::default() });
    let v = serde_json::to_value(&d.context).unwrap();
    assert_eq!(v["seed"], serde_json::json!(u64::MAX.to_string()));
    let back: DealContext = serde_json::from_value(v).unwrap();
    assert_eq!(back.seed, u64::MAX);
    assert_eq!(seed_serde::from_json(&serde_json::json!("42")), Some(42));
    assert_eq!(seed_serde::from_json(&serde_json::json!(42)), Some(42));
    assert_eq!(seed_serde::from_json(&serde_json::json!(-1)), None);
}
