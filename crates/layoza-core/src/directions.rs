// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
//! Direction dealer for `design-system`: decoupled type and palette pools, direction archetypes
//! (layout composition + palette tone + type voice + hero composition), and a small brief reader
//! that writes each direction's copy and rationale from the brief's own subject.
//!
//! Nothing here fixes a design: every direction is a set of options with the reason it was picked
//! and the alternatives it beat, so the agent can choose differently with the reason written down.

use crate::dna::DesignDials;
use crate::intelligence::{
    fnv1a, keyword_hit, pairing, palette, tokenize, verify_palette, BriefSignals, IndustryRule, PaletteRecommendation, Surface,
    TypographyPairing,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tone {
    Light,
    Dark,
    Field,
}

impl Tone {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Tone::Light => "light ground",
            Tone::Dark => "dark ground",
            Tone::Field => "saturated colour field",
        }
    }
    /// Classifies any palette by its background: strongly chromatic grounds are colour fields,
    /// otherwise relative luminance decides light or dark.
    pub(crate) fn of(p: &PaletteRecommendation) -> Tone {
        let l = crate::intelligence::wcag_contrast(&p.background, "#000000").map(|c| c * 0.05 - 0.05).unwrap_or(0.5);
        let h = p.background.trim_start_matches('#');
        let ch: Vec<u8> = (0..3).filter_map(|i| h.get(i * 2..i * 2 + 2).and_then(|x| u8::from_str_radix(x, 16).ok())).collect();
        let chroma = ch.iter().max().zip(ch.iter().min()).map(|(a, b)| a - b).unwrap_or(0);
        if chroma >= 60 {
            Tone::Field
        } else if l > 0.5 {
            Tone::Light
        } else if l < 0.03 {
            Tone::Dark
        } else {
            Tone::Field
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Voice {
    Editorial,
    Expressive,
    Utility,
}

impl Voice {
    fn label(self) -> &'static str {
        match self {
            Voice::Editorial => "editorial",
            Voice::Expressive => "expressive",
            Voice::Utility => "utility",
        }
    }
}

pub(crate) struct TypeEntry {
    pub pairing: TypographyPairing,
    pub voice: Voice,
    pub tags: &'static [&'static str],
}

pub(crate) struct PaletteEntry {
    pub palette: PaletteRecommendation,
    pub tone: Tone,
    pub tags: &'static [&'static str],
}

/// Google Fonts css2 URL for `Family:axes` specs.
fn gf(specs: &[&str]) -> String {
    let q: Vec<String> = specs.iter().map(|f| format!("family={}", f.replace(' ', "+"))).collect();
    format!("https://fonts.googleapis.com/css2?{}&display=swap", q.join("&"))
}

#[allow(clippy::too_many_arguments)]
fn ty(id: &str, display: &str, body: &str, mono: &str, mood: &str, ratio: &str, voice: Voice, tags: &'static [&'static str], url: &str) -> TypeEntry {
    TypeEntry { pairing: pairing(id, display, body, mono, mood, ratio, url), voice, tags }
}

/// Every family here is on Google Fonts under the SIL Open Font License (or Apache 2.0 for Roboto Mono).
pub(crate) fn type_pool() -> Vec<TypeEntry> {
    use Voice::*;
    vec![
        ty("syne-plus-jakarta-kinetic", "Syne", "Plus Jakarta Sans", "JetBrains Mono", "Kinetic, bold, flagship launch", "1.333 (Perfect Fourth)", Expressive, &["kinetic", "night"],
            "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&family=Plus+Jakarta+Sans:wght@400;500;600&family=Syne:wght@700;800&display=swap"),
        ty("unbounded-onest", "Unbounded", "Onest", "JetBrains Mono", "Wide, loud, poster-like", "1.414 (Augmented Fourth)", Expressive, &["kinetic", "bold"],
            "https://fonts.googleapis.com/css2?family=Onest:wght@400;500;600&family=Unbounded:wght@600;800&family=JetBrains+Mono:wght@400&display=swap"),
        ty("dela-gothic-figtree", "Dela Gothic One", "Figtree", "Space Mono", "Heavy, graphic, street-poster energy", "1.333 (Perfect Fourth)", Expressive, &["night", "earth", "bold"],
            "https://fonts.googleapis.com/css2?family=Dela+Gothic+One&family=Figtree:wght@400;500;600&family=Space+Mono&display=swap"),
        ty("instrument-serif-inter-tight", "Instrument Serif", "Inter Tight", "IBM Plex Mono", "Cinematic, editorial contrast", "1.5 (Perfect Fifth)", Editorial, &["botanical", "night", "kinetic"],
            "https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=Inter+Tight:wght@400;500;600&family=IBM+Plex+Mono&display=swap"),
        ty("dm-serif-work-sans", "DM Serif Display", "Work Sans", "DM Mono", "Salt-weathered, confident, maritime signage", "1.333 (Perfect Fourth)", Expressive, &["marine", "night"],
            "https://fonts.googleapis.com/css2?family=DM+Mono&family=DM+Serif+Display&family=Work+Sans:wght@400;500;600&display=swap"),
        ty("young-serif-instrument-sans", "Young Serif", "Instrument Sans", "Space Mono", "Warm, hand-made, oven-lit", "1.250 (Major Third)", Editorial, &["warm-grain", "earth"],
            "https://fonts.googleapis.com/css2?family=Instrument+Sans:wght@400;500;600&family=Space+Mono&family=Young+Serif&display=swap"),
        ty("bodoni-moda-jost", "Bodoni Moda", "Jost", "DM Mono", "High-fashion, editorial, tailored", "1.5 (Perfect Fifth)", Expressive, &["textile", "luxe"],
            "https://fonts.googleapis.com/css2?family=Bodoni+Moda:ital,wght@0,500;0,700;1,500&family=DM+Mono&family=Jost:wght@400;500&display=swap"),
        ty("barlow-condensed-plex", "Barlow Condensed", "IBM Plex Sans", "IBM Plex Mono", "Control-room, legible at a glance", "1.200 (Minor Third)", Utility, &["data", "industrial", "night"],
            "https://fonts.googleapis.com/css2?family=Barlow+Condensed:wght@600;700&family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@400;500;600&display=swap"),
        ty("geist-terminal", "Geist", "Geist", "Geist Mono", "Terminal-native, exacting, transcript-led", "1.250 (Major Third)", Utility, &["code", "night"],
            "https://fonts.googleapis.com/css2?family=Geist:wght@400;500;600;700&family=Geist+Mono:wght@400;500&display=swap"),
        ty("bricolage-ibm-plex", "Bricolage Grotesque", "IBM Plex Sans", "IBM Plex Mono", "Engineered, interactive, utility-first", "1.250 (Major Third)", Utility, &["data", "kinetic"],
            "https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:wght@600;700&family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@400;500;600&display=swap"),
        ty("archivo-atkinson", "Archivo", "Atkinson Hyperlegible", "JetBrains Mono", "Plain-spoken, legible, practical", "1.200 (Minor Third)", Utility, &["industrial", "data", "care", "bold"],
            "https://fonts.googleapis.com/css2?family=Archivo:wght@600;800&family=Atkinson+Hyperlegible:wght@400;700&family=JetBrains+Mono&display=swap"),
        ty("manrope-dm-mono", "Manrope", "Manrope", "DM Mono", "Calm, precise, product-like", "1.250 (Major Third)", Utility, &["botanical", "care", "calm"],
            "https://fonts.googleapis.com/css2?family=DM+Mono&family=Manrope:wght@400;500;700;800&display=swap"),
        ty("space-grotesk-public-sans", "Space Grotesk", "Public Sans", "Space Mono", "Technical, friendly, modular", "1.250 (Major Third)", Utility, &["kinetic", "code", "bold"],
            "https://fonts.googleapis.com/css2?family=Public+Sans:wght@400;500;600&family=Space+Grotesk:wght@500;700&family=Space+Mono&display=swap"),
        ty("libre-franklin-source-sans", "Libre Franklin", "Source Sans 3", "Source Code Pro", "Clear, nautical-chart precise", "1.250 (Major Third)", Utility, &["marine", "civic"],
            "https://fonts.googleapis.com/css2?family=Libre+Franklin:wght@600;800&family=Source+Code+Pro&family=Source+Sans+3:wght@400;500;600&display=swap"),
        ty("zilla-slab-source-sans", "Zilla Slab", "Source Sans 3", "Source Code Pro", "Sturdy, ledger-like, kitchen-board", "1.200 (Minor Third)", Utility, &["warm-grain", "data", "earth"],
            "https://fonts.googleapis.com/css2?family=Source+Code+Pro&family=Source+Sans+3:wght@400;500;600&family=Zilla+Slab:wght@600;700&display=swap"),
        ty("cormorant-hanken", "Cormorant Garamond", "Hanken Grotesk", "DM Mono", "Quiet, tactile, swatch-book", "1.333 (Perfect Fourth)", Editorial, &["textile", "calm", "luxe"],
            "https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,600;1,500&family=DM+Mono&family=Hanken+Grotesk:wght@400;500;600&display=swap"),
        ty("schibsted-albert-fira", "Schibsted Grotesk", "Albert Sans", "Fira Code", "Docs-grade, plain, copy-paste ready", "1.200 (Minor Third)", Utility, &["code", "data"],
            "https://fonts.googleapis.com/css2?family=Albert+Sans:wght@400;500;600&family=Fira+Code:wght@400;500&family=Schibsted+Grotesk:wght@600;700&display=swap"),
        ty("gloock-figtree", "Gloock", "Figtree", "IBM Plex Mono", "Sharp-serifed, authoritative, letterpress", "1.414 (Augmented Fourth)", Editorial, &["legal", "civic", "earth"],
            &gf(&["Gloock", "Figtree:wght@400;500;600", "IBM Plex Mono:wght@400;500"])),
        ty("libre-baskerville-source-sans", "Libre Baskerville", "Source Sans 3", "Source Code Pro", "Bookish, sober, legible at length", "1.250 (Major Third)", Editorial, &["legal", "civic"],
            &gf(&["Libre Baskerville:ital,wght@0,400;0,700;1,400", "Source Sans 3:wght@400;500;600", "Source Code Pro"])),
        ty("spectral-plex-sans", "Spectral", "IBM Plex Sans", "IBM Plex Mono", "Screen-first serif, documentary", "1.333 (Perfect Fourth)", Editorial, &["legal", "civic", "data"],
            &gf(&["Spectral:ital,wght@0,500;0,700;1,500", "IBM Plex Sans:wght@400;500;600", "IBM Plex Mono"])),
        ty("literata-inter", "Literata", "Inter", "JetBrains Mono", "Long-read, warm, unhurried", "1.250 (Major Third)", Editorial, &["civic", "care", "calm"],
            &gf(&["Literata:ital,opsz,wght@0,7..72,500;0,7..72,700;1,7..72,500", "Inter:wght@400;500;600", "JetBrains Mono"])),
        ty("fredoka-nunito", "Fredoka", "Nunito", "Space Mono", "Round, friendly, chunky", "1.333 (Perfect Fourth)", Expressive, &["play"],
            &gf(&["Fredoka:wght@500;600;700", "Nunito:wght@400;600;700", "Space Mono"])),
        ty("baloo2-atkinson", "Baloo 2", "Atkinson Hyperlegible", "DM Mono", "Soft, legible for young and old readers", "1.250 (Major Third)", Utility, &["play", "care"],
            &gf(&["Baloo 2:wght@500;700", "Atkinson Hyperlegible:wght@400;700", "DM Mono"])),
        ty("bebas-neue-barlow", "Bebas Neue", "Barlow", "Roboto Mono", "Condensed caps, stencil-and-signage", "1.414 (Augmented Fourth)", Expressive, &["industrial", "night", "kinetic"],
            &gf(&["Bebas Neue", "Barlow:wght@400;500;600", "Roboto Mono"])),
        ty("anton-work-sans", "Anton", "Work Sans", "Space Mono", "Poster-loud, tight, shouty", "1.5 (Perfect Fifth)", Expressive, &["kinetic", "night", "bold"],
            &gf(&["Anton", "Work Sans:wght@400;500;600", "Space Mono"])),
        ty("big-shoulders-chivo", "Big Shoulders Display", "Chivo", "Chivo Mono", "Dockside signage, condensed and civic", "1.414 (Augmented Fourth)", Expressive, &["industrial", "marine", "data"],
            &gf(&["Big Shoulders Display:wght@700;900", "Chivo:wght@400;500;700", "Chivo Mono"])),
        ty("archivo-black-narrow", "Archivo Black", "Archivo Narrow", "JetBrains Mono", "Heavy grotesk, newsprint punch", "1.414 (Augmented Fourth)", Expressive, &["kinetic", "bold", "data"],
            &gf(&["Archivo Black", "Archivo Narrow:wght@400;500;600", "JetBrains Mono"])),
        ty("epilogue-mono", "Epilogue", "Epilogue", "JetBrains Mono", "Geometric-grotesk, systematic", "1.250 (Major Third)", Utility, &["data", "code"],
            &gf(&["Epilogue:wght@400;500;700;800", "JetBrains Mono"])),
        ty("familjen-inter", "Familjen Grotesk", "Inter", "DM Mono", "Clean, quietly characterful", "1.250 (Major Third)", Utility, &["textile", "botanical", "care"],
            &gf(&["Familjen Grotesk:wght@500;700", "Inter:wght@400;500;600", "DM Mono"])),
        ty("red-hat-family", "Red Hat Display", "Red Hat Text", "Red Hat Mono", "Open, technical, even-handed", "1.250 (Major Third)", Utility, &["code", "care", "data"],
            &gf(&["Red Hat Display:wght@500;700;900", "Red Hat Text:wght@400;500", "Red Hat Mono"])),
        ty("sora-rubik", "Sora", "Rubik", "Roboto Mono", "Rounded-geometric, approachable", "1.250 (Major Third)", Utility, &["play", "data"],
            &gf(&["Sora:wght@500;700", "Rubik:wght@400;500", "Roboto Mono"])),
        ty("alegreya-sans", "Alegreya", "Alegreya Sans", "Space Mono", "Calligraphic, humane, organic", "1.333 (Perfect Fourth)", Editorial, &["botanical", "earth", "warm-grain"],
            &gf(&["Alegreya:ital,wght@0,500;0,700;1,500", "Alegreya Sans:wght@400;500;700", "Space Mono"])),
        ty("crimson-pro-work-sans", "Crimson Pro", "Work Sans", "DM Mono", "Old-style, quiet, crafted", "1.333 (Perfect Fourth)", Editorial, &["textile", "warm-grain", "earth"],
            &gf(&["Crimson Pro:ital,wght@0,500;0,700;1,500", "Work Sans:wght@400;500;600", "DM Mono"])),
        ty("oswald-source-serif", "Oswald", "Source Serif 4", "Source Code Pro", "Condensed headings over a reading serif", "1.333 (Perfect Fourth)", Expressive, &["marine", "industrial", "civic"],
            &gf(&["Oswald:wght@500;700", "Source Serif 4:ital,wght@0,400;0,600;1,400", "Source Code Pro"])),
        ty("eb-garamond-hanken", "EB Garamond", "Hanken Grotesk", "DM Mono", "Classical, restrained, archival", "1.333 (Perfect Fourth)", Editorial, &["luxe", "civic", "textile"],
            &gf(&["EB Garamond:ital,wght@0,500;0,700;1,500", "Hanken Grotesk:wght@400;500;600", "DM Mono"])),
        ty("italiana-manrope", "Italiana", "Manrope", "DM Mono", "Hairline display, couture", "1.5 (Perfect Fifth)", Expressive, &["luxe", "textile"],
            &gf(&["Italiana", "Manrope:wght@400;500;600", "DM Mono"])),
        ty("chivo-workshop", "Chivo", "Chivo", "Chivo Mono", "Workshop-plain, sturdy", "1.200 (Minor Third)", Utility, &["industrial", "code"],
            &gf(&["Chivo:wght@400;500;700;900", "Chivo Mono:wght@400;500"])),
        ty("plex-serif-sans", "IBM Plex Serif", "IBM Plex Sans", "IBM Plex Mono", "Corporate-precise, documentary", "1.250 (Major Third)", Editorial, &["data", "legal", "code"],
            &gf(&["IBM Plex Serif:ital,wght@0,500;0,600;1,500", "IBM Plex Sans:wght@400;500;600", "IBM Plex Mono"])),
    ]
}

#[allow(clippy::too_many_arguments)]
fn pe(id: &str, bg: &str, surface: &str, fg: &str, muted: &str, primary: &str, secondary: &str, notes: &str, tone: Tone, tags: &'static [&'static str]) -> PaletteEntry {
    let mut p = palette(id, bg, surface, fg, muted, primary, secondary, 0.0, notes);
    verify_palette(&mut p);
    PaletteEntry { palette: p, tone, tags }
}

/// Body and muted text clear WCAG AA (>= 4.5:1) on every background here (tested); the contrast
/// figure is computed, never typed in.
pub(crate) fn palette_pool() -> Vec<PaletteEntry> {
    use Tone::*;
    vec![
        pe("stage-obsidian-coral", "#0E1014", "#181B22", "#F5F4F0", "#94A3B8", "#F97316", "#38BDF8", "Deep obsidian stage with coral and sky accents", Dark, &["kinetic", "night"]),
        pe("cobalt-saturated-field", "#1B2FA8", "#2238C4", "#F7F7F2", "#C9CFF2", "#FFC83D", "#4FE3C1", "Saturated cobalt colour field with sun-yellow and mint", Field, &["kinetic", "bold"]),
        pe("oxblood-bone", "#3A0D12", "#4A141A", "#F4EBDD", "#D9BFA8", "#FF6A3D", "#E9C46A", "Oxblood stage with bone type and ember accents", Dark, &["night", "earth", "bold"]),
        pe("forest-night-chartreuse", "#0F1F17", "#16291F", "#EEF3EC", "#9FB5A6", "#C7F25C", "#7FD1FF", "Night-forest stage with chartreuse signal colour", Dark, &["botanical", "night"]),
        pe("deep-harbour-buoy", "#0B2230", "#12303F", "#EEF4F2", "#9DB8C0", "#FF7A59", "#7FD6C2", "Deep harbour water with buoy-orange and sea-glass accents", Dark, &["marine", "night"]),
        pe("oven-crust-night", "#2A1A10", "#3A2617", "#F6EBDD", "#CDB8A0", "#F2A541", "#E86A4A", "Dark oven interior with crust-gold and ember accents", Dark, &["warm-grain", "earth"]),
        pe("indigo-loom", "#1C2340", "#252D52", "#F3EFE6", "#B9BCD3", "#E8B04B", "#E4826B", "Indigo-dyed cloth with saffron thread and madder accents", Dark, &["textile"]),
        pe("ops-console-night", "#0D1117", "#161B22", "#E6EDF3", "#9DA7B3", "#3FB950", "#F0883E", "Night console where state colour (ok / late / short) is the only decoration", Dark, &["data", "industrial", "night"]),
        pe("phosphor-session", "#0A0F0C", "#111915", "#E8F1EA", "#93A89A", "#4ADE80", "#FBBF24", "Dark session surface where green/amber mean pass/warn in real tool output, never decoration", Dark, &["code", "night"]),
        pe("workbench-mineral", "#F4F5F3", "#FFFFFF", "#111615", "#4A5552", "#0D9488", "#2563EB", "Mineral neutral with teal and cobalt interactive states", Light, &["data"]),
        pe("paper-signal-red", "#FBFAF7", "#FFFFFF", "#141414", "#555555", "#D7261E", "#1F5FBF", "Paper white with signal red and ink blue", Light, &["industrial", "data", "bold"]),
        pe("sage-graphite", "#EEF2EE", "#FFFFFF", "#1A1F1B", "#4D5A50", "#2F6B4F", "#B4542B", "Sage daylight with graphite text and rust accents", Light, &["botanical", "calm", "care"]),
        pe("lilac-ink", "#F3F1FA", "#FFFFFF", "#17142B", "#4E4A66", "#5B3FD9", "#E0751F", "Cool lilac paper with violet controls and orange highlights", Light, &["kinetic", "bold", "play"]),
        pe("tide-chart-paper", "#F2F6F7", "#FFFFFF", "#0E2430", "#44606B", "#0B6E8A", "#D9572B", "Tide-chart paper with deep-channel blue and signal-flag orange", Light, &["marine"]),
        pe("rye-ledger", "#EFEFEA", "#FFFFFF", "#1E1A16", "#57504A", "#A0461C", "#3D6B4F", "Flour-dusted grey ledger with rye-crust and herb-green accents", Light, &["warm-grain", "data", "earth"]),
        pe("raw-linen-ink", "#E9E9E4", "#F7F7F4", "#191919", "#504E4B", "#8C3B2E", "#2F4A6D", "Undyed linen grey with madder-red and indigo swatches", Light, &["textile", "calm"]),
        pe("manpage-paper", "#F6F6F3", "#FFFFFF", "#121417", "#4B5058", "#1D4ED8", "#B45309", "Man-page paper with link-blue actions and amber for warnings in real output", Light, &["code", "data"]),
        pe("kelp-field", "#1F5C5A", "#236663", "#F4F1E8", "#CFE0DA", "#FFB347", "#F2E3B3", "Kelp-green colour field with net-float orange and sand", Field, &["marine", "botanical"]),
        pe("saffron-crust-field", "#E39B3A", "#E8A64B", "#1F1408", "#3D2810", "#6B1E0E", "#FFF4E0", "Baked-crust saffron field with burnt-brown type", Field, &["warm-grain", "earth"]),
        pe("madder-dye-field", "#8E2F25", "#9C382D", "#FBF1E6", "#F0CDBF", "#F5C04A", "#9FC3E0", "Madder-dyed red field with weld-yellow and woad-blue threads", Field, &["textile", "earth"]),
        pe("acid-signal-paper", "#F4F4EF", "#FFFFFF", "#0A0A0A", "#4A4A4A", "#2B2BFF", "#D1145A", "Bright paper with electric blue and magenta signal colour", Light, &["kinetic", "bold"]),
        pe("moss-field", "#4F6B3A", "#587642", "#F6F3E7", "#E2E7D3", "#F2C14E", "#F4A38F", "Moss-green field with pollen-yellow and petal-pink", Field, &["botanical"]),
        pe("machinist-blueprint", "#1D3F72", "#234A84", "#F2F5FA", "#C5D2E8", "#FFC93C", "#FF9A6B", "Blueprint blue with drafting-white lines and hazard-yellow marks", Field, &["industrial", "data", "code"]),
        pe("graphite-hazard", "#1A1B1D", "#232528", "#EDEDE8", "#A3A6A0", "#F5B700", "#4FB0C6", "Graphite workshop floor with hazard-yellow and coolant-blue", Dark, &["industrial", "night"]),
        pe("velvet-neon", "#170F24", "#221735", "#F3EEFA", "#B7A8D1", "#FF4FA3", "#3EE6C5", "Velvet-purple night with neon pink and mint", Dark, &["night", "kinetic"]),
        pe("terracotta-field", "#9A4426", "#A54D2E", "#FFF7EE", "#F5D9C8", "#F6C25B", "#BFE0D5", "Fired-terracotta field with slip-white type and glaze accents", Field, &["earth"]),
        pe("kiln-ash-paper", "#E8E6E3", "#F5F4F2", "#221E1B", "#5A534D", "#9C4A2F", "#2E5E6B", "Kiln-ash grey with iron-oxide and celadon-teal accents", Light, &["earth", "warm-grain"]),
        pe("patch-yellow-field", "#F2D64B", "#F5DE6A", "#121212", "#3A3420", "#1E3FA0", "#9E1F14", "Highlighter-yellow field with ink-blue links and diff-red", Field, &["code", "kinetic", "play"]),
        pe("chancery-green", "#12261E", "#183229", "#F1EEE4", "#B4BFB4", "#E58A6B", "#8FB7C9", "Deep chancery green with sealing-wax red and slate-blue (no brass)", Dark, &["legal", "luxe"]),
        pe("court-paper-oxide", "#F3F3F0", "#FFFFFF", "#15171A", "#4B4F55", "#7A2E1F", "#1F3D5C", "Bond-paper grey with seal-oxide red and record-blue", Light, &["legal", "civic"]),
        pe("ink-blue-field", "#1E2F5C", "#26386A", "#F4F2EC", "#C8CCDA", "#F09A7E", "#8FD3BF", "Fountain-ink blue field with coral and verdigris (no gold)", Field, &["legal", "civic", "marine"]),
        pe("crayon-paper", "#F7F9FC", "#FFFFFF", "#1B1B3A", "#4A4A6A", "#C8401F", "#1F6F94", "Cool drawing-paper white with crayon red and pool blue", Light, &["play"]),
        pe("sunny-field", "#FFD23F", "#FFDA5E", "#1B1B3A", "#463A12", "#1F4FD8", "#B01E33", "Sun-yellow field with navy type, primary blue and red", Field, &["play", "warm-grain"]),
        pe("night-planetarium", "#14163A", "#1C1F4D", "#F5F3FF", "#B9B7E0", "#FFD23F", "#FF8A97", "Planetarium dome navy with star-yellow and coral", Dark, &["play", "civic", "night"]),
        pe("clinic-daylight", "#F2F7F6", "#FFFFFF", "#102A2A", "#456262", "#0F766E", "#B23A0B", "Clean daylight with deep-teal actions and a warm alert colour", Light, &["care"]),
        pe("dusk-calm", "#1E2A33", "#26343F", "#EEF3F5", "#A9BAC4", "#8FD3C7", "#F2B880", "Blue-hour slate with mint and apricot", Dark, &["care", "night", "calm"]),
        pe("archive-stone", "#EBEBE7", "#F7F7F4", "#18181A", "#505056", "#A3271F", "#2453A6", "Archive-box stone with catalogue red and record blue", Light, &["civic", "legal"]),
        pe("lacquer-cinnabar", "#0C0B0A", "#171513", "#F2EEE6", "#A8A196", "#D1503F", "#7FA08B", "Black lacquer with cinnabar and jade (no gold)", Dark, &["luxe", "night"]),
        pe("signal-orange-field", "#FF5B1F", "#FF6A33", "#111111", "#3B1606", "#1B1BB0", "#FFFFFF", "Signal-orange field with black type and ultramarine", Field, &["kinetic", "bold"]),
    ]
}

/// A direction structure: layout composition, palette tone, type voice and hero composition.
pub(crate) struct Archetype {
    pub key: &'static str,
    /// 0 = A (structural), 1 = B (expressive), 2 = C (working tool).
    pub slot: u8,
    pub name: &'static str,
    pub style_family: &'static str,
    /// Surfaces it fits; empty = every surface.
    pub surfaces: &'static [Surface],
    pub tone: Tone,
    pub voice: Voice,
    /// First-screen composition the hero is built around.
    pub composition: &'static str,
    /// When this structure is the right answer.
    pub why: &'static str,
    /// +1 suits calm briefs, -1 suits bold ones.
    pub calm_bias: i8,
}

use Surface as S;
const NOT_UI: &[Surface] = &[S::Commerce, S::Venue, S::Showcase, S::Editorial, S::DevTool, S::General];
const STORY: &[Surface] = &[S::Commerce, S::Venue, S::Showcase, S::Editorial, S::General];

pub(crate) const ARCHETYPES: &[Archetype] = &[
    // A: structural
    Archetype { key: "type-architecture", slot: 0, name: "Type Architecture", style_family: "swiss-editorial-grid", surfaces: NOT_UI, tone: Tone::Light, voice: Voice::Editorial, composition: "stacked",
        why: "the brief's own words and the trade's notation can carry the first screen without stock imagery", calm_bias: 0 },
    Archetype { key: "index-archive", slot: 0, name: "Catalogue Index", style_family: "swiss-editorial-grid", surfaces: &[S::ProductUi, S::Commerce, S::Showcase, S::Editorial, S::General], tone: Tone::Light, voice: Voice::Editorial, composition: "stacked",
        why: "the subject has many comparable items, and browsing them is the main job", calm_bias: 1 },
    Archetype { key: "editorial-longread", slot: 0, name: "Long Read", style_family: "block-first-editorial", surfaces: STORY, tone: Tone::Light, voice: Voice::Editorial, composition: "centred",
        why: "trust is earned by explaining (who, how, why) before asking for the action", calm_bias: 1 },
    Archetype { key: "specimen-object", slot: 0, name: "Specimen Plate", style_family: "atelier-minimalism", surfaces: STORY, tone: Tone::Light, voice: Voice::Editorial, composition: "object-wrap",
        why: "one physical object can stand for the whole business when it is shown at scale and labelled", calm_bias: 1 },
    Archetype { key: "map-wayfinding", slot: 0, name: "Wayfinding Map", style_family: "swiss-editorial-grid", surfaces: &[S::Venue, S::General], tone: Tone::Light, voice: Voice::Utility, composition: "layered",
        why: "a visit is the conversion, so where things are and how to get there leads", calm_bias: 0 },
    Archetype { key: "summary-ledger", slot: 0, name: "Daily Ledger", style_family: "swiss-editorial-grid", surfaces: &[S::ProductUi], tone: Tone::Light, voice: Voice::Utility, composition: "stacked",
        why: "a task UI is read once a day as a summary before anyone acts on it", calm_bias: 1 },
    Archetype { key: "reference-manual", slot: 0, name: "Reference Manual", style_family: "swiss-editorial-grid", surfaces: &[S::DevTool], tone: Tone::Light, voice: Voice::Utility, composition: "stacked",
        why: "developers judge a tool by how fast they can find the exact command and its output", calm_bias: 1 },
    // B: expressive
    Archetype { key: "full-bleed-stage", slot: 1, name: "Full-Bleed Stage", style_family: "kinetic-editorial", surfaces: STORY, tone: Tone::Dark, voice: Voice::Expressive, composition: "full-bleed",
        why: "atmosphere sells it: the place, the product or the work has to be felt before it is read", calm_bias: -1 },
    Archetype { key: "scroll-sequence", slot: 1, name: "Scroll Sequence", style_family: "kinetic-editorial", surfaces: NOT_UI, tone: Tone::Dark, voice: Voice::Expressive, composition: "sticky-stage",
        why: "the subject has a process worth watching step by step", calm_bias: -1 },
    Archetype { key: "colour-field-poster", slot: 1, name: "Colour-Field Poster", style_family: "block-first-editorial", surfaces: NOT_UI, tone: Tone::Field, voice: Voice::Expressive, composition: "poster",
        why: "the brand needs one loud, memorable message per screen more than detail", calm_bias: -1 },
    Archetype { key: "collage-board", slot: 1, name: "Pin-Board Collage", style_family: "kinetic-editorial", surfaces: STORY, tone: Tone::Field, voice: Voice::Expressive, composition: "layered",
        why: "personality and the trade's many small artefacts matter more than a single hero object", calm_bias: 0 },
    Archetype { key: "live-wall", slot: 1, name: "Live Operations Wall", style_family: "high-density-workbench", surfaces: &[S::ProductUi], tone: Tone::Dark, voice: Voice::Utility, composition: "full-bleed",
        why: "the team watches the numbers all day on a shared screen and acts on exceptions", calm_bias: 0 },
    Archetype { key: "timeline-axis", slot: 1, name: "Timeline Axis", style_family: "high-density-workbench", surfaces: &[S::ProductUi], tone: Tone::Field, voice: Voice::Utility, composition: "sticky-stage",
        why: "the work is scheduled, so time is the axis everything else hangs on", calm_bias: 0 },
    Archetype { key: "transcript-replay", slot: 1, name: "Session Replay", style_family: "precision-terminal-craft", surfaces: &[S::DevTool], tone: Tone::Dark, voice: Voice::Utility, composition: "sticky-stage",
        why: "a real session on real input is the only proof developers trust", calm_bias: -1 },
    // C: the working tool
    Archetype { key: "working-tool-first", slot: 2, name: "Working Tool First", style_family: "crisp-product-workbench", surfaces: STORY, tone: Tone::Light, voice: Voice::Utility, composition: "centred",
        why: "the visitor's question (price, time, fit, availability) can be answered by a small tool on the first screen", calm_bias: 0 },
    Archetype { key: "workbench-split", slot: 2, name: "Task Workbench", style_family: "crisp-product-workbench", surfaces: &[S::ProductUi], tone: Tone::Light, voice: Voice::Utility, composition: "split",
        why: "editing and seeing the consequence must happen side by side (the one case a split is earned)", calm_bias: 1 },
    Archetype { key: "command-console", slot: 2, name: "Command-First Console", style_family: "crisp-product-workbench", surfaces: &[S::ProductUi, S::DevTool], tone: Tone::Light, voice: Voice::Utility, composition: "centred",
        why: "expert users repeat a few jobs many times a day and type faster than they click", calm_bias: 1 },
    Archetype { key: "configurator", slot: 2, name: "Configurator", style_family: "crisp-product-workbench", surfaces: &[S::Commerce], tone: Tone::Light, voice: Voice::Utility, composition: "split",
        why: "the product has options, and price, stock and delivery must update beside the product", calm_bias: 0 },
    Archetype { key: "booking-board", slot: 2, name: "Booking-First Board", style_family: "crisp-product-workbench", surfaces: &[S::Venue, S::General], tone: Tone::Light, voice: Voice::Utility, composition: "stacked",
        why: "most visitors arrive to book or check availability, often on a phone", calm_bias: 0 },
    Archetype { key: "playground", slot: 2, name: "Install-and-Try Playground", style_family: "crisp-product-workbench", surfaces: &[S::DevTool], tone: Tone::Light, voice: Voice::Utility, composition: "centred",
        why: "time to first success decides adoption", calm_bias: 0 },
    Archetype { key: "case-index", slot: 2, name: "Sortable Case Index", style_family: "crisp-product-workbench", surfaces: &[S::Showcase], tone: Tone::Light, voice: Voice::Utility, composition: "stacked",
        why: "clients want to find work like theirs fast, not scroll a reel", calm_bias: 1 },
    Archetype { key: "guided-finder", slot: 2, name: "Guided Finder", style_family: "crisp-product-workbench", surfaces: &[S::General, S::Commerce, S::Venue, S::Editorial], tone: Tone::Light, voice: Voice::Utility, composition: "centred",
        why: "visitors arrive unsure which service or product fits them", calm_bias: 0 },
];

/// The trade's objects, primary action and notation, read from the brief's words.
struct Trade {
    words: &'static [&'static str],
    surface: Option<Surface>,
    objects: [&'static str; 3],
    action: &'static str,
    notation: &'static str,
}

const TRADES: &[Trade] = &[
    Trade { words: &["dashboard", "admin", "console", "analytics", "ops", "operations", "backoffice"], surface: Some(S::ProductUi),
        objects: ["today's numbers", "the exception list", "the plan table"], action: "resolve what is late or short", notation: "units, timestamps, owners and status" },
    Trade { words: &["sales", "revenue", "orders", "takings"], surface: Some(S::ProductUi),
        objects: ["the day's takings", "the order book", "the product mix"], action: "see what sold and reorder", notation: "units sold, margins and order times" },
    Trade { words: &["law", "lawyer", "lawyers", "attorney", "attorneys", "solicitor", "solicitors", "legal", "barrister", "barristers", "notary", "litigation"], surface: None,
        objects: ["the case file", "the signed agreement", "the hearing calendar"], action: "book a first consultation", notation: "matter numbers, clause references and hearing dates" },
    Trade { words: &["kids", "children", "child", "toddlers", "toddler", "toy", "toys", "playground", "nursery"], surface: None,
        objects: ["hands-on exhibits mid-use", "the activity map", "the stamp card"], action: "plan the day by age and time", notation: "age bands, session times and wayfinding colours" },
    Trade { words: &["museum", "exhibition", "exhibitions", "gallery", "heritage"], surface: Some(S::Venue),
        objects: ["the object on its plinth", "the wall label", "the floor plan"], action: "plan a visit and book a timed ticket", notation: "accession numbers, wall labels and gallery numbers" },
    Trade { words: &["seafood", "fish", "oyster", "oysters", "fishmonger"], surface: Some(S::Venue),
        objects: ["the day's catch on ice", "the oyster tray", "the chalkboard of landings"], action: "book a table", notation: "catch weights, landing times and market prices" },
    Trade { words: &["bakery", "bread", "sourdough", "pastry", "patisserie", "baker", "loaves"], surface: Some(S::Venue),
        objects: ["loaves cooling on the rack", "the oven door", "the flour-dusted bench"], action: "order for collection", notation: "bake times, batch sizes and weights" },
    Trade { words: &["restaurant", "bistro", "brasserie", "diner", "kitchen", "trattoria"], surface: Some(S::Venue),
        objects: ["the plate on the pass", "tonight's menu", "the dining room"], action: "book a table", notation: "menu prices, covers and service times" },
    Trade { words: &["cafe", "coffee", "espresso", "roastery", "roaster"], surface: Some(S::Venue),
        objects: ["the cup on the counter", "the roast profile", "the bag label"], action: "order ahead or find the counter", notation: "roast dates, origins and brew ratios" },
    Trade { words: &["linen", "fabric", "textile", "textiles", "cotton", "wool", "silk", "denim"], surface: Some(S::Commerce),
        objects: ["the weave up close", "the swatch card", "the garment on the body"], action: "choose a size and add to bag", notation: "fibre content, weight in GSM and care symbols" },
    Trade { words: &["fashion", "apparel", "clothing", "womenswear", "menswear", "garment"], surface: Some(S::Commerce),
        objects: ["the piece on the body", "the fit notes", "the look"], action: "choose a size and add to bag", notation: "size charts, fit notes and care labels" },
    Trade { words: &["kinetic", "motion", "animation", "showreel"], surface: Some(S::Showcase),
        objects: ["type in motion", "the frame sequence", "the edit timeline"], action: "open the reel or start a project", notation: "frame numbers, timecodes and easing curves" },
    Trade { words: &["agency", "studio", "portfolio", "designer", "photographer"], surface: Some(S::Showcase),
        objects: ["the work itself", "the process sketches", "the client credits"], action: "open a case study or start a project", notation: "project numbers, years and credits" },
    Trade { words: &["cli", "sdk", "api", "devtool", "devtools", "dev tool", "dev tools", "developer", "developers", "terminal", "mcp", "compiler", "library"], surface: Some(S::DevTool),
        objects: ["the command and its real output", "the diff it produces", "the config file"], action: "copy the install command", notation: "commands, flags, exit codes and versions" },
    Trade { words: &["saas", "software", "platform", "app"], surface: None,
        objects: ["the product screen doing its job", "the before and after of one task", "the plan table"], action: "start a trial", notation: "plan limits, uptime and changelog dates" },
    Trade { words: &["clinic", "dental", "dentist", "doctor", "physio", "physiotherapy", "therapy", "therapist", "medical"], surface: None,
        objects: ["the treatment room", "the appointment slot", "the clinician"], action: "book an appointment", notation: "appointment times, treatment names and prices" },
    Trade { words: &["gym", "yoga", "pilates", "fitness", "climbing"], surface: None,
        objects: ["the class in progress", "the timetable", "the studio floor"], action: "book a class", notation: "class times, levels and spaces left" },
    Trade { words: &["hotel", "inn", "guesthouse", "hostel", "resort"], surface: Some(S::Venue),
        objects: ["the room at first light", "the view from the window", "the key on the desk"], action: "check dates and book a room", notation: "room types, nightly rates and check-in times" },
    Trade { words: &["architect", "architects", "architecture", "interiors"], surface: Some(S::Showcase),
        objects: ["the building on its site", "the section drawing", "the material sample"], action: "view a project or start a brief", notation: "drawing numbers, scales and project years" },
    Trade { words: &["bookshop", "bookstore", "books", "library", "publisher"], surface: None,
        objects: ["spines on the shelf", "the opening page", "the staff-pick card"], action: "find a book or reserve it", notation: "ISBNs, shelf marks and publication dates" },
    Trade { words: &["wine", "vineyard", "winery"], surface: None,
        objects: ["the bottle and its label", "the vineyard rows", "the tasting sheet"], action: "order a case or book a tasting", notation: "vintages, grape varieties and parcel names" },
    Trade { words: &["brewery", "beer", "taproom"], surface: Some(S::Venue),
        objects: ["the can art", "the tap list", "the brew kit"], action: "see what is on tap", notation: "ABV, IBU and batch dates" },
    Trade { words: &["estate", "property", "properties", "homes", "realtor", "listings"], surface: None,
        objects: ["the home itself", "the floor plan", "the street outside"], action: "book a viewing", notation: "floor areas, prices and energy ratings" },
    Trade { words: &["charity", "nonprofit", "foundation", "donate", "donation", "donations"], surface: None,
        objects: ["the people helped", "the project on the ground", "where the money goes"], action: "donate or volunteer", notation: "amounts, places and reporting dates" },
    Trade { words: &["salon", "barber", "hairdresser", "spa", "beauty"], surface: Some(S::Venue),
        objects: ["the chair", "the cut itself", "the price list"], action: "book a slot", notation: "services, durations and prices" },
    Trade { words: &["vet", "veterinary", "pets", "pet", "dog", "dogs", "cats"], surface: None,
        objects: ["the animal in care", "the waiting room", "the treatment plan"], action: "book an appointment", notation: "species, treatments and opening hours" },
    Trade { words: &["school", "university", "course", "courses", "academy", "tutoring"], surface: None,
        objects: ["the class at work", "the timetable", "the work students make"], action: "apply or book an open day", notation: "term dates, course codes and fees" },
    Trade { words: &["festival", "music", "band", "concert", "gig", "tour"], surface: Some(S::Showcase),
        objects: ["the stage", "the line-up", "the ticket"], action: "buy a ticket", notation: "set times, stages and dates" },
    Trade { words: &["florist", "flowers", "garden", "plants"], surface: None,
        objects: ["stems in the bucket", "the bouquet being tied", "the delivery card"], action: "order a bouquet for a date", notation: "stem names, seasons and delivery slots" },
    Trade { words: &["bicycle", "bike", "bikes", "repair", "workshop", "mechanic", "garage"], surface: None,
        objects: ["the part on the stand", "the tool wall", "the job ticket"], action: "book a repair slot", notation: "part numbers, torque values and job tickets" },
    Trade { words: &["ceramics", "pottery", "clay", "kiln", "stoneware"], surface: None,
        objects: ["the pot on the wheel", "the glaze test tiles", "the kiln shelf"], action: "buy a piece or book a class", notation: "glaze codes, firing temperatures and batch numbers" },
    Trade { words: &["bank", "finance", "fintech", "accounting", "accountant", "accountants", "insurance", "investment"], surface: None,
        objects: ["the statement", "the account balance", "the signed form"], action: "open an account or book a call", notation: "rates, fees and dates" },
    Trade { words: &["magazine", "journal", "blog", "newsletter", "publication"], surface: Some(S::Editorial),
        objects: ["the cover", "the lead story", "the issue index"], action: "read the lead story or subscribe", notation: "issue numbers, bylines and dates" },
    Trade { words: &["travel", "tours", "trips", "holiday", "holidays"], surface: None,
        objects: ["the place itself", "the route", "the day-by-day itinerary"], action: "check dates and book", notation: "dates, distances and prices" },
    Trade { words: &["watch", "watches", "jewellery", "jewelry", "jeweller", "jeweler"], surface: Some(S::Commerce),
        objects: ["the piece at macro scale", "the movement or setting", "the hallmark"], action: "book a viewing or buy", notation: "calibres, carats and hallmarks" },
    Trade { words: &["store", "shop", "ecommerce", "webshop", "boutique"], surface: Some(S::Commerce),
        objects: ["the product", "price and stock", "the parcel"], action: "add to bag", notation: "prices, stock and delivery dates" },
];

/// What the brief is about, in its own words.
#[derive(Debug, Clone)]
pub(crate) struct Subject {
    /// The subject phrase ("seafood restaurant", "kids' museum").
    pub phrase: String,
    pub objects: Vec<String>,
    pub action: String,
    pub notation: String,
    /// True when objects came from a known trade; false when only the phrase is known.
    pub from_trades: bool,
}

const FILLER: &[&str] = &[
    "a", "an", "the", "our", "my", "their", "new", "website", "site", "homepage", "home", "landing", "marketing", "web", "online",
    "build", "make", "create", "design", "redesign", "page", "please",
];
const BREAKS: &[&str] = &[
    "for", "on", "with", "in", "that", "which", "to", "and", "near", "from", "where", "who", "selling", "offering", "using", "by", "at",
    "of", "about", "like",
];

fn subject_phrase(query: &str) -> String {
    let first_line = query.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    let sentence = first_line.split(['.', ';', ':', '!', '?', ',', '\n']).next().unwrap_or("");
    let words: Vec<String> = sentence
        .split_whitespace()
        .map(|w| w.chars().filter(|c| c.is_alphanumeric() || *c == '\'' || *c == '-' || *c == '\u{2019}').collect::<String>())
        .filter(|w| !w.is_empty())
        .collect();
    let mut segments: Vec<Vec<String>> = vec![Vec::new()];
    for w in words {
        if BREAKS.contains(&w.to_lowercase().as_str()) {
            segments.push(Vec::new());
        } else if let Some(last) = segments.last_mut() {
            last.push(w);
        }
    }
    let content = |seg: &Vec<String>| -> Vec<String> {
        seg.iter().filter(|w| !FILLER.contains(&w.to_lowercase().as_str())).map(|w| w.to_lowercase()).collect()
    };
    let chosen = segments.iter().map(content).find(|c| !c.is_empty()).unwrap_or_default();
    let n = chosen.len();
    let phrase = chosen.into_iter().skip(n.saturating_sub(4)).collect::<Vec<_>>().join(" ");
    if phrase.is_empty() { "this business".into() } else { phrase }
}

pub(crate) fn read_subject(query: &str, signals: &BriefSignals) -> Subject {
    let tokens = tokenize(query);
    let pos = |w: &str| -> Option<usize> {
        let parts = tokenize(w);
        if parts.len() == 1 {
            tokens.iter().position(|t| *t == parts[0] || (parts[0].len() > 3 && *t == format!("{}s", parts[0])))
        } else if keyword_hit(&tokens, w) {
            Some(tokens.len())
        } else {
            None
        }
    };
    let mut hits: Vec<(bool, usize, &Trade)> = TRADES
        .iter()
        .filter_map(|t| {
            let off = matches!(signals.surface, S::ProductUi | S::DevTool) && t.surface != Some(signals.surface);
            t.words.iter().filter_map(|w| pos(w)).min().map(|p| (off, p, t))
        })
        .collect();
    // On a task surface the matching trade leads ("bakery sales dashboard" is a dashboard first);
    // otherwise the brief's word order does ("kids' museum" is about kids before museums).
    hits.sort_by_key(|(off_surface, p, _)| (*off_surface, *p));
    let phrase = subject_phrase(query);
    if hits.is_empty() {
        let head = phrase.split_whitespace().last().unwrap_or("business").to_string();
        let action = match signals.surface {
            S::Commerce => "add to bag",
            S::Venue => "book or plan a visit",
            S::ProductUi => "act on today's exceptions",
            S::DevTool => "copy the install command",
            S::Showcase => "open the work",
            S::Editorial => "start reading",
            S::General => "make first contact (call, book or ask)",
        };
        return Subject {
            objects: vec![format!("the {head} itself"), "the people it serves".into(), "the result it delivers".into()],
            action: action.into(),
            notation: "the trade's own names, numbers and dates".into(),
            phrase,
            from_trades: false,
        };
    }
    let mut objects: Vec<String> = Vec::new();
    for round in 0..3 {
        for (_, _, t) in &hits {
            if objects.len() < 3 && !objects.iter().any(|o| o == t.objects[round]) {
                objects.push(t.objects[round].to_string());
            }
        }
    }
    let notation = match hits.get(1) {
        Some((_, _, t2)) if t2.notation != hits[0].2.notation => format!("{}; {}", hits[0].2.notation, t2.notation),
        _ => hits[0].2.notation.to_string(),
    };
    Subject { phrase, objects, action: hits[0].2.action.to_string(), notation, from_trades: true }
}

/// One direction as dealt: its structure, type and palette, and the options it beat.
pub(crate) struct Dealt {
    pub arch: &'static Archetype,
    pub typography: TypographyPairing,
    pub palette: PaletteRecommendation,
    pub tone: Tone,
    pub type_tags: Vec<&'static str>,
    pub palette_tags: Vec<&'static str>,
    /// Other archetypes that fit this slot for this brief.
    pub alternatives: Vec<&'static str>,
    /// True when type and palette are the matched industry profile's own.
    pub from_industry: bool,
}

fn fits(a: &Archetype, s: Surface) -> bool {
    a.surfaces.is_empty() || a.surfaces.contains(&s)
}

/// Re-deal index: variant 0 takes the best fit; variant n > 0 walks the other near-best fits.
/// Tags that name a trade's subject matter (not a mood). A pick carrying only other trades'
/// subject tags reads as borrowed: a marine palette or a rounded kids' face on a law firm.
const SUBJECT_TAGS: &[&str] = &["marine", "play", "legal", "botanical", "textile", "care", "luxe", "warm-grain", "earth", "civic", "industrial"];

/// Dark blue ground with a gold/ochre/brass accent: the "trust" palette law and finance default to.
pub(crate) fn navy_gold(p: &PaletteRecommendation) -> bool {
    fn hsl(hex: &str) -> Option<(f64, f64, f64)> {
        let h = hex.trim_start_matches('#');
        if h.len() != 6 {
            return None;
        }
        let c = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok().map(|v| v as f64 / 255.0);
        let (r, g, b) = (c(0)?, c(2)?, c(4)?);
        let (max, min) = (r.max(g).max(b), r.min(g).min(b));
        let l = (max + min) / 2.0;
        let d = max - min;
        if d == 0.0 {
            return Some((0.0, 0.0, l));
        }
        let s = d / (1.0 - (2.0 * l - 1.0).abs());
        let hue = if max == r { 60.0 * (((g - b) / d) % 6.0) } else if max == g { 60.0 * ((b - r) / d + 2.0) } else { 60.0 * ((r - g) / d + 4.0) };
        Some(((hue + 360.0) % 360.0, s, l))
    }
    let navy = hsl(&p.background).is_some_and(|(h, s, l)| (200.0..=250.0).contains(&h) && s > 0.2 && l < 0.35);
    let gold = |hex: &str| hsl(hex).is_some_and(|(h, s, l)| (35.0..=55.0).contains(&h) && s > 0.45 && (0.35..=0.8).contains(&l));
    navy && (gold(&p.primary_accent) || gold(&p.secondary_accent))
}

/// `has` should also accept the tags the surface itself wants (data, code), so a dashboard
/// palette that happens to be tagged industrial is still on-brief.
/// The first subject tag names what a pick was made for, so a pick led by another trade's subject
/// (a court-paper palette that also suits civic buildings) still reads as borrowed.
fn foreign(tags: &[&str], has: &dyn Fn(&str) -> bool) -> bool {
    let lead = tags.iter().find(|t| SUBJECT_TAGS.contains(t));
    // A surface tag the brief wants (data for a dashboard, code for a dev tool) outranks the lead.
    let surface = tags.iter().any(|t| matches!(*t, "data" | "code") && has(t));
    lead.is_some_and(|t| !has(t)) && !surface
}

fn rotate(variant: u64, n: usize) -> usize {
    if variant == 0 || n == 0 {
        0
    } else {
        ((variant - 1) % n as u64) as usize
    }
}

/// Near-best window: every candidate within `margin` of the best, at least `min` when available.
fn window<T: Copy>(ranked: &[(i32, T)], margin: i32, min: usize) -> Vec<T> {
    let top = ranked.first().map(|(s, _)| *s).unwrap_or(0);
    ranked.iter().enumerate().filter(|(i, (s, _))| *s + margin >= top || *i < min).map(|(_, (_, t))| *t).collect()
}

/// Deals the three directions. `industry` is the matched profile's own type and palette; it keeps
/// Direction A at variant 0. A variant > 0 never repeats variant 0's pick in any slot (archetype,
/// display font, palette) while another fit exists, and consecutive variants differ.
pub(crate) fn deal(signals: &BriefSignals, query: &str, variant: u64, industry: Option<(&TypographyPairing, &PaletteRecommendation)>) -> Vec<Dealt> {
    let base = if variant > 0 { Some(deal(signals, query, 0, industry)) } else { None };
    let q = query.to_lowercase();
    let h = |s: &str| fnv1a(&format!("{s}|{q}"));
    let has = |tag: &str| signals.niche_tags.iter().any(|t| t == tag) || signals.moods.iter().any(|m| m == tag);
    let mood = |m: &str| signals.moods.iter().any(|x| x == m);
    let wants = |t: &str| has(t) || (t == "data" && signals.surface == S::ProductUi) || (t == "code" && signals.surface == S::DevTool);
    let types = type_pool();
    let pals = palette_pool();

    let mut out: Vec<Dealt> = Vec::new();
    let mut used_comp: Vec<&str> = Vec::new();
    let mut used_fonts: Vec<String> = Vec::new();
    let mut used_pals: Vec<String> = Vec::new();
    let mut used_tones: Vec<Tone> = Vec::new();
    if let Some((t, p)) = industry {
        // B and C never reuse the industry's own display face or palette, even when A re-deals.
        used_fonts.push(t.display_font.clone());
        used_pals.push(p.id.clone());
    }
    for slot in 0..3u8 {
        let prev = base.as_ref().map(|b| &b[slot as usize]);
        // Archetype
        let mut ranked: Vec<(i32, &'static Archetype)> = ARCHETYPES
            .iter()
            .filter(|a| a.slot == slot && fits(a, signals.surface))
            .map(|a| {
                // A structure made for this specific surface beats a general one; "general" briefs
                // get no such bonus, so venue-only ideas (maps, booking boards) do not lead them.
                let mut s = if a.surfaces.len() <= 2 && signals.surface != S::General { 4 } else { 2 };
                if mood("calm") {
                    s += 2 * a.calm_bias as i32;
                }
                if mood("bold") {
                    s -= 2 * a.calm_bias as i32;
                }
                if signals.motion_bias >= 4 && a.composition == "sticky-stage" {
                    s += 2;
                }
                if used_comp.contains(&a.composition) {
                    s -= 10;
                }
                (s, a)
            })
            .collect();
        ranked.sort_by(|x, y| y.0.cmp(&x.0).then_with(|| h(x.1.key).cmp(&h(y.1.key))));
        let mut cands = window(&ranked, 2, 2);
        if let Some(p) = prev {
            if cands.len() > 1 {
                cands.retain(|a| a.key != p.arch.key);
            }
        }
        let arch = cands[rotate(variant, cands.len())];
        let alternatives: Vec<&'static str> = ranked.iter().filter(|(s, a)| a.key != arch.key && *s > -5).map(|(_, a)| a.name).collect();
        used_comp.push(arch.composition);

        // Type and palette: A at variant 0 keeps the matched industry's own.
        if slot == 0 && variant == 0 {
            if let Some((t, p)) = industry {
                let tone = Tone::of(p);
                used_tones.push(tone);
                out.push(Dealt { arch, typography: t.clone(), palette: p.clone(), tone, type_tags: Vec::new(), palette_tags: Vec::new(), alternatives, from_industry: true });
                continue;
            }
        }
        let mut tr: Vec<(i32, usize)> = types
            .iter()
            .enumerate()
            .filter(|(_, e)| !used_fonts.contains(&e.pairing.display_font))
            .map(|(i, e)| {
                let mut s = if e.voice == arch.voice { 4 } else { 0 };
                s += e.tags.iter().map(|t| if *t == "data" && signals.surface == S::ProductUi { 4 } else if has(t) { 3 } else { 0 }).sum::<i32>();
                if signals.surface == S::DevTool && e.tags.contains(&"code") {
                    s += 2;
                }
                if foreign(e.tags, &wants) {
                    s -= 7;
                }
                (s, i)
            })
            .collect();
        tr.sort_by(|x, y| y.0.cmp(&x.0).then_with(|| h(&types[x.1].pairing.id).cmp(&h(&types[y.1].pairing.id))));
        let mut tc = window(&tr, 2, 3);
        if let Some(p) = prev {
            if tc.len() > 1 {
                tc.retain(|i| types[*i].pairing.display_font != p.typography.display_font);
            }
        }
        let te = &types[tc[rotate(variant, tc.len())]];

        let mut pr: Vec<(i32, usize)> = pals
            .iter()
            .enumerate()
            .filter(|(_, e)| !used_pals.contains(&e.palette.id))
            .map(|(i, e)| {
                let mut s = if e.tone == arch.tone { 4 } else { 0 };
                if used_tones.contains(&e.tone) {
                    s -= 3;
                }
                s += e.tags.iter().map(|t| if *t == "data" && signals.surface == S::ProductUi { 4 } else if has(t) { 3 } else { 0 }).sum::<i32>();
                if signals.surface == S::DevTool && e.tags.contains(&"code") {
                    s += 2;
                }
                if foreign(e.tags, &wants) {
                    s -= 7;
                }
                // Law and money briefs list navy-and-gold as their reflex palette: never deal it.
                if (has("legal") || has("finance")) && navy_gold(&e.palette) {
                    s -= 8;
                }
                match e.tone {
                    Tone::Dark if mood("dark") => s += 2,
                    Tone::Light if mood("light") => s += 2,
                    Tone::Field if mood("calm") => s -= 1,
                    _ => {}
                }
                (s, i)
            })
            .collect();
        pr.sort_by(|x, y| y.0.cmp(&x.0).then_with(|| h(&pals[x.1].palette.id).cmp(&h(&pals[y.1].palette.id))));
        let mut pc = window(&pr, 2, 3);
        if let Some(p) = prev {
            if pc.len() > 1 {
                pc.retain(|i| pals[*i].palette.id != p.palette.id);
            }
        }
        let pe = &pals[pc[rotate(variant, pc.len())]];
        used_fonts.push(te.pairing.display_font.clone());
        used_pals.push(pe.palette.id.clone());
        used_tones.push(pe.tone);
        out.push(Dealt {
            arch,
            typography: te.pairing.clone(),
            palette: pe.palette.clone(),
            tone: pe.tone,
            type_tags: te.tags.iter().copied().filter(|t| has(t)).collect(),
            palette_tags: pe.tags.iter().copied().filter(|t| has(t)).collect(),
            alternatives,
            from_industry: false,
        });
    }
    out
}

/// Inputs for writing one direction's copy.
pub(crate) struct Ctx<'a> {
    pub subject: &'a Subject,
    pub place: Option<&'a str>,
    pub material: Option<&'a str>,
    pub surface: Surface,
}

pub(crate) struct DirectionText {
    pub layout: String,
    pub identity: String,
    pub content: String,
    pub motion: String,
}

fn cap(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

/// Writes layout, identity, content and motion for one dealt direction at its motion dial.
pub(crate) fn write(d: &Dealt, ctx: &Ctx, motion: u8) -> DirectionText {
    let s = ctx.subject;
    let subj = s.phrase.as_str();
    let big = cap(subj);
    let (o1, o2, o3) = (s.objects[0].as_str(), s.objects[1].as_str(), s.objects[2].as_str());
    let act = s.action.as_str();
    let note = s.notation.as_str();
    let at = ctx.place.map(|p| format!(" in {p}")).unwrap_or_default();
    let mat = ctx.material.map(str::to_string).unwrap_or_else(|| o1.trim_start_matches("the ").to_string());
    let (display, body, mono, ratio) = (&d.typography.display_font, &d.typography.body_font, &d.typography.mono_font, &d.typography.scale_ratio);
    let notes = d.palette.notes.split(" | ").next().unwrap_or("");
    let pal = format!("palette {} ({}): {}", d.palette.id, d.tone.label(), notes.to_lowercase());
    let (t_in, t_ui) = match motion {
        0..=1 => (0, 0),
        2..=4 => (240, 150),
        5..=7 => (520, 180),
        _ => (800, 220),
    };
    let calm = motion <= 4 || ctx.surface == S::ProductUi;
    let pick = |c: String, m: String| if calm { c } else { m };
    let (layout, identity, content, motion_text) = match d.arch.key {
        "type-architecture" => (
            format!("{big}{at} set as architecture: the headline at poster scale spans the full measure and the grid is drawn from the trade's notation ({note}); {o1} sits inside the type (between lines or in a counter), not beside it; sections step down a strict {ratio} scale with ruled dividers instead of cards"),
            format!("{display} does the image work at display sizes over {body} for reading; {pal}; rules, numbers and labels from the trade are the only ornament"),
            format!("One sentence that says what this {subj} does, then '{act}' working in the first viewport; specifics as ruled tables of {note}; no big-number stat blocks"),
            pick(format!("Type-led load: headline lines settle in turn ({t_in}ms ease-out), rules draw once on entry, {t_ui}ms hover and focus states; nothing moves on scroll"),
                 format!("Headline lines enter on load ({t_in}ms stagger), then scroll progress shifts the type's weight or tracking through the first section while {o1} moves within the lettering; every frame stays readable")),
        ),
        "index-archive" => (
            format!("{big} as a catalogue: the first screen is a ruled index of {o1}, {o2} and {o3} with the trade's own numbers ({note}) in the first column; filters across the top, rows open in place, no hero-image slot"),
            format!("Archive vocabulary: {display} for headings, {body} for rows, {mono} for numbers; {pal}; colour only where an entry needs attention"),
            format!("Browse first: visitors find their own way into the {subj} through the index, then '{act}' from any row"),
            pick(format!("Rows expand in place ({t_ui}ms) and filters re-order with FLIP ({t_ui}ms); nothing moves on scroll"),
                 format!("Rows reveal in scroll order and the active row's preview of {o1} grows into a pinned panel as the list scrolls ({t_in}ms hand-offs)")),
        ),
        "editorial-longread" => (
            format!("The {subj}{at} told as a long read: a centred 60-70ch column opens on {o1} as a full-width plate, then chapters on {o2} and {o3} with pull quotes and margin notes; '{act}' is an inline link at each chapter end and a docked bar"),
            format!("{display} headlines at book-jacket scale with {body} set for long reading and captions in {mono}; {pal}; photographs of the real subject only"),
            format!("Narrative first: why this {subj} exists, how it works, who it is for; the facts ({note}) sit in margin notes beside the story"),
            pick(format!("Chapter openers fade up once ({t_in}ms) and a thin rule shows reading progress; nothing moves on scroll beyond that"),
                 format!("Each chapter plate grows from inset to full width as it scrolls in; margin notes slide in beside the paragraph they annotate ({t_in}ms)")),
        ),
        "specimen-object" => (
            format!("{big} as a specimen plate: {o1} drawn or photographed at scale on a plain ground, the headline wrapping its silhouette, labelled callouts to its details ({note}); {o2} and {o3} follow as further plates in the same callout system"),
            format!("Museum-plate restraint: {display} for plate titles, {body} for callouts, hairline leaders; {pal}; the object supplies the only rich colour"),
            format!("Show the thing before saying anything: plates first, then '{act}', then the facts table"),
            pick(format!("Callout leaders draw on entry ({t_in}ms); hover or focus on a callout highlights its detail; nothing moves on scroll"),
                 format!("Scroll turns or zooms {o1} through its details while each callout lights in turn; the plate comes to rest with every label visible")),
        ),
        "map-wayfinding" => (
            format!("{big}{at} as a map: the first screen is an illustrated plan (rooms, route or neighbourhood) with {o1}, {o2} and {o3} as numbered points; choosing a point opens its panel; '{act}' is pinned to the map key"),
            format!("Wayfinding system: {display} as signage, {body} for panels, numbered markers and colour-coded zones; {pal}"),
            format!("Orientation first: where things are, when they open and how to get there, then '{act}'"),
            pick(format!("Markers pulse once on load and panels slide in {t_ui}ms; nothing moves on scroll"),
                 format!("Scroll walks the route across the map zone by zone, opening each stop's panel ({t_in}ms); the whole map is visible at the end")),
        ),
        "summary-ledger" => (
            format!("{big} as one ruled daily ledger: a one-sentence summary of today at the top, then the full table of {o1} grouped by owner with totals in a sticky footer; {o2} as flagged rows; detail opens in a side drawer"),
            format!("Bookkeeping clarity: {display} for the summary sentence, {body} with tabular figures for rows, {mono} for codes; {pal}; state colour only on cells that need action"),
            format!("Read the day in one sentence, then scan the table; every number links to the rows behind it ({note})"),
            format!("{t_ui}ms row updates and drawer slides only; no scroll-triggered choreography"),
        ),
        "reference-manual" => (
            format!("The {subj} as a reference manual: install line and a one-paragraph synopsis at the top, then NAME, SYNOPSIS, COMMANDS, EXAMPLES and EXIT CODES with a sticky table of contents; every example shows {o1}"),
            format!("Manual typography: {display} for section heads, {body} for prose, {mono} for every command; {pal}; no illustrations, the examples are the pictures"),
            format!("Answer 'what does it do and how do I call it' in the first screen, then the full reference ({note})"),
            format!("{t_ui}ms copy confirmations and a table-of-contents highlight that follows the reading position; nothing moves on scroll"),
        ),
        "full-bleed-stage" => (
            format!("Full-bleed stage for the {subj}{at}: {o1} fills the first screen edge to edge with the headline set into its quiet area; later sections cut between {o2} and {o3} at full width; '{act}' stays docked and usable at every scroll position"),
            format!("Cinematic contrast: {display} at stage scale over {body}; {pal}; real photography or drawn art of the subject, never stock"),
            format!("Atmosphere first, then what is available now ({note}), then '{act}'"),
            pick(format!("Slow cross-fades between plates on view ({t_in}ms); the docked action never animates away; nothing moves on scroll"),
                 format!("Plates pin and hand off as you scroll ({t_in}ms cross-fades keyed to scroll progress) while {o1} drifts slowly; the action bar stays fixed")),
        ),
        "scroll-sequence" => (
            format!("A pinned sequence about the {subj}: the first screen holds {o1} while 3-4 steps of copy pass over it, each step changing the scene ({o2}, then {o3}); normal sections follow"),
            format!("{display} for step headlines, {body} for captions; {pal}; the subject's own process is the story, not abstract shapes"),
            format!("Process as story: each step is one true fact about how this {subj} works, ending on '{act}'"),
            pick(format!("Steps cross-fade in place on click or arrow keys ({t_in}ms); nothing moves on scroll"),
                 format!("The scene is scrubbed by scroll progress (CSS scroll-driven animations or a scroll library): frames at 25/50/75% are each complete; the last frame hands off to '{act}'")),
        ),
        "colour-field-poster" => (
            format!("The {subj} as a run of posters: each section is one full colour field with one large statement and one image ({o1}, then {o2}, then {o3}); the first poster carries '{act}' as a large button"),
            format!("Poster print: {display} set huge, {body} small and tight; {pal}; colour owns whole sections, no gradients"),
            format!("One message per screen in the visitor's words, ending on '{act}'; details ({note}) in a compact closing section"),
            pick(format!("Posters change on view with a {t_in}ms colour wipe; nothing moves on scroll"),
                 "Each colour field wipes into the next with scroll progress while the headline and image move at different speeds".to_string()),
        ),
        "collage-board" => (
            format!("A pin-board collage for the {subj}{at}: {o1}, {o2} and {o3} as overlapping cut-outs, notes and tickets on one layered first screen with the headline stuck on like a label; below, the same pieces open into sections"),
            format!("Hand-assembled: {display} on labels, {body} on notes, tape and shadow as the only depth; {pal}; textures from the trade's own material ({mat})"),
            format!("Personality first: the pieces show what this {subj} is like, then '{act}'"),
            pick(format!("Pieces settle onto the board once on load ({t_in}ms, slight rotation) and lift on hover; nothing moves on scroll"),
                 "Pieces drift apart at different scroll speeds to open each section, then pin again at rest; the headline stays readable in every frame".to_string()),
        ),
        "live-wall" => (
            format!("Full-width live wall for the {subj}: {o1} on a horizontal time axis as the hero, {o2} as an exceptions rail pinned to the side, drill-down drawers instead of page changes"),
            format!("Control-room surface: {display} condensed numerals over {body} with tabular figures; {pal}; state colour (on track, late, short) is the only decoration"),
            format!("Exceptions first: what is late, short or wrong right now and who owns it, then the plan, then history ({note})"),
            format!("Data-driven only: {t_ui}ms value tweens on update and FLIP row insert or remove; no scroll-triggered choreography; reduced-motion swaps tweens for instant updates"),
        ),
        "timeline-axis" => (
            format!("The {subj} on a vertical day timeline: hours run down the page, {o1} and {o3} sit as blocks on the axis, a now-line marks the current time and conflicts are flagged; a side panel edits the selected block"),
            format!("Schedule clarity: {display} for times, {body} for block labels, {mono} for codes; {pal}"),
            format!("Time first: what happens next, what overlaps, then the detail ({note})"),
            format!("The now-line moves with the clock and edited blocks snap in {t_ui}ms; no scroll-triggered choreography"),
        ),
        "transcript-replay" => (
            format!("A real session as the hero for the {subj}: the install command, then the tool running on real input ({o1}), ending on {o2}; then a labelled architecture diagram, then reproducible benchmarks with the method beside the numbers"),
            format!("Session surface: {mono} only for commands and output, {display} for claims, {body} for prose; {pal}; colour reserved for pass, warn, fail and diff states"),
            "Show it working on real input first, then how it works, then proof (method, licence, release cadence, changelog); no invented testimonials or counters".to_string(),
            pick(format!("Transcript steps appear as they come into view ({t_ui}ms per line) with a skip control; nothing moves on scroll beyond that"),
                 "Transcript replay paced to reading speed with pause and skip, plus scroll-linked step progress; reduced-motion renders the complete transcript statically".to_string()),
        ),
        "working-tool-first" => (
            format!("The {subj} opens on the tool itself, centred: a small working control for '{act}' (a picker, calculator or short form built around {o1}) above the fold with the headline as its label; then sections built from {o2} and {o3}"),
            format!("Controls redrawn in the trade's own material ({mat}): {display} for labels, {body} for help text; {pal}; one accent for actions"),
            "Interactive proof: the visitor gets an answer (price, time, fit, availability) before reading any marketing copy".to_string(),
            format!("{t_ui}ms state changes on input, results update in place; nothing moves on scroll"),
        ),
        "workbench-split" => (
            format!("Split workbench for the {subj}, because editing and its consequence must be seen at once: the editable table of {o1} (keyboard-navigable cells, bulk actions) on one side, a live consequence panel ({o3}, totals, capacity) on the other; filters as a command bar"),
            format!("Quiet light surface: {display} for panel heads, {body} with tabular figures; {pal}; semantic colours for state only"),
            format!("Change a quantity or date and watch the plan and its consequences recalculate in place ({note})"),
            format!("{t_ui}ms state transitions only; zero decorative motion"),
        ),
        "command-console" => (
            format!("The {subj} as a command-first console: one centred command bar with suggestions drawn from {o1} and {o2}; results open as panels beneath; shortcuts are shown on every action"),
            format!("{mono} for commands, {body} for results, {display} sparingly for section heads; {pal}"),
            format!("Do the most common job ('{act}') in one typed line, then show the result ({note}) and the next likely action"),
            format!("{t_ui}ms palette open, filter and result transitions; nothing moves on scroll"),
        ),
        "configurator" => (
            format!("Configurator for the {subj}, split because the product and its options must stay in view together: {o1} large on one side, size, colour or material choices on the other with price, stock and delivery updating in place; then a filterable catalogue with real facts ({note})"),
            format!("Swatch-book controls: real material swatches ({mat}) as inputs, {display} for product names, {body} for facts; {pal}"),
            format!("Decision support first: sizing help, comparison and returns policy beside '{act}'"),
            format!("FLIP re-layout when filters change and {t_ui}ms swatch transitions; nothing moves on scroll"),
        ),
        "booking-board" => (
            format!("Booking-first board for the {subj}{at}: date and time slots (with party size where it applies) working above the fold, stacked over today's board ({o1}, {o2}, with prices or availability); location, hours and access follow as a ruled table"),
            format!("The venue's own board, ticket or label as the visual system: {display} for the board, {body} at large sizes for phones; {pal}"),
            format!("Utility first: can I {act}, what is on today, how do I get there"),
            format!("Slot-selection micro-interactions only ({t_ui}ms); nothing moves on scroll; reduced-motion removes them"),
        ),
        "playground" => (
            format!("Install-and-try for the {subj}: a copyable install command with package-manager tabs, centred above a working playground (edit the input, run, see {o1}); then quickstart steps, a searchable command reference and {o3} examples"),
            format!("Docs-grade surface: {display} for headings, {body} for prose, {mono} for code with AA-contrast syntax colours; {pal}"),
            format!("Time to first success: install, first command, expected output, troubleshooting; then the full reference ({note})"),
            format!("{t_ui}ms copy confirmations, tab switches and run states only; nothing moves on scroll"),
        ),
        "case-index" => (
            format!("The {subj}'s work as a sortable index: filter by discipline, client and year; focus or hover previews a loop of {o1}; each case opens on {o2} before the final shot"),
            format!("Archive vocabulary: catalogue numbers in {mono}, titles in {display}, ruled rows in {body}; {pal}; the work is the only colour"),
            format!("Proof by browsing: visitors sort the work themselves, then see process and credits ({note})"),
            format!("Preview loops on hover or focus (paused under reduced motion) and {t_ui}ms FLIP filter re-layout; nothing moves on scroll"),
        ),
        _ => (
            format!("A guided finder for the {subj}, centred: three short questions (about {o1}, about {o2}, and when) lead to one specific answer and '{act}'; the answer page then explains {o3} with real details"),
            format!("Friendly forms: {display} for the questions, {body} for options as large tappable cards; {pal}"),
            "Answer the visitor's actual question first; everything else is one tap away".to_string(),
            format!("Question-to-question slides ({t_ui}ms); nothing moves on scroll"),
        ),
    };
    let motion_text = if motion <= 1 {
        format!("No motion: {o1} and every section render in their final state on load; hover and focus change colour only; nothing moves on scroll")
    } else if d.arch.slot == 2 && motion >= 6 && ctx.surface != S::ProductUi {
        motion_text.replace("; nothing moves on scroll", &format!("; sections enter once on scroll ({t_in}ms, transform and opacity only)"))
    } else {
        motion_text
    };
    DirectionText { layout, identity, content, motion: motion_text }
}

/// Why this direction was dealt, from the brief's signals and the options chosen.
pub(crate) fn rationale(d: &Dealt, signals: &BriefSignals, subject: &Subject, industry_name: &str) -> String {
    let surface = format!("{:?}", signals.surface).to_lowercase();
    let mut out = format!("{} for \"{}\" ({} surface): {}.", d.arch.name, subject.phrase, surface, d.arch.why);
    if d.from_industry {
        out.push_str(&format!(" Type {} / {} and palette {} are the {} profile's own.", d.typography.display_font, d.typography.body_font, d.palette.id, industry_name));
    } else {
        let tags = |t: &[&str]| if t.is_empty() { String::new() } else { format!(", tagged {} like the brief", t.join("/")) };
        out.push_str(&format!(
            " Type {} / {} ({}) for {} {} voice{}. Palette {} on a {}{}.",
            d.typography.display_font,
            d.typography.body_font,
            d.typography.mood.to_lowercase(),
            if d.arch.voice == Voice::Utility { "a" } else { "an" },
            d.arch.voice.label(),
            tags(&d.type_tags),
            d.palette.id,
            d.tone.label(),
            tags(&d.palette_tags)
        ));
    }
    out.push_str(&format!(" First screen: {} composition", d.arch.composition));
    if d.arch.composition == "split" {
        out.push_str(" (a split, chosen only because two views must be read side by side)");
    }
    out.push('.');
    if !d.alternatives.is_empty() {
        out.push_str(&format!(" Other structures that fit this slot: {}.", d.alternatives.join(", ")));
    }
    if !subject.from_trades {
        out.push_str(" The trade was not recognised: replace the placeholder objects with the real things a visitor sees.");
    }
    out
}

/// Clichés derived from the brief's surface and niche when no catalogue industry matched.
pub(crate) fn derived_cliches(signals: &BriefSignals) -> Vec<String> {
    let mut v: Vec<&str> = match signals.surface {
        S::Venue => vec!["Full-bleed hero photo with a centred script headline, and the hours, prices or menu hidden in a PDF"],
        S::Commerce => vec!["White grid of identical product cards with a pill 'Add to cart'"],
        S::Showcase => vec!["Giant name plus a 'Selected works' thumbnail grid"],
        S::Editorial => vec!["Three-column blog-template card feed"],
        S::ProductUi => vec!["A row of four KPI cards over one line chart"],
        S::DevTool => vec!["Purple gradient with a glowing terminal mockup"],
        S::General => vec![],
    };
    for t in &signals.niche_tags {
        v.extend(match t.as_str() {
            "legal" => &["Scales of justice, gavels or columns as stock imagery", "Navy-and-gold 'trust' palette under a serif headline"][..],
            "play" => &["Primary-colour confetti, wobbly blobs and a cartoon mascot unrelated to the real place", "Rounded comic type for everything, including body text"][..],
            "care" => &["Stock smiling-professional photo on a teal gradient", "Icon trio (heart, shield, clock) with three vague promises"][..],
            "civic" => &["Grey page with one gold serif and a slideshow of exhibitions but no visit information"][..],
            "marine" => &["Anchors, rope and navy-stripe clip art"][..],
            "warm-grain" | "earth" => &["Cream background with a terracotta accent and an italic serif"][..],
            "textile" => &["Beige minimal store with a thin sans and no fabric close-ups"][..],
            "kinetic" => &["Random marquee strips and a cursor blob with no link to the work"][..],
            "botanical" => &["Leaf-pattern overlay on sage green"][..],
            "industrial" => &["Hazard stripes and gear icons as decoration"][..],
            "night" => &["Neon on black for every venue"][..],
            "code" => &["Terminal mockup by default with fake output"][..],
            "luxe" => &["Black and white with a single gold accent"][..],
            "data" => &["Sparkline-in-every-card dashboard template"][..],
            _ => &[][..],
        });
    }
    v.extend([
        "Template hero (headline left, illustration right, two buttons)",
        "Uniform card grids with icon + heading + paragraph",
        "Gradient blobs and glassmorphism with no brand reason",
    ]);
    let mut out: Vec<String> = Vec::new();
    for c in v {
        if !out.iter().any(|x| x == c) {
            out.push(c.to_string());
        }
    }
    out
}

/// A profile built from the brief's own words when no catalogue industry matched (confidence `none`).
pub(crate) fn brief_industry(query: &str, signals: &BriefSignals, subject: &Subject, typography: TypographyPairing, palette: PaletteRecommendation) -> IndustryRule {
    let o1 = &subject.objects[0];
    let act = &subject.action;
    let note = &subject.notation;
    let styles: &[&str] = match signals.surface {
        S::ProductUi => &["high-density-workbench", "crisp-product-workbench"],
        S::DevTool => &["precision-terminal-craft", "crisp-product-workbench"],
        S::Commerce => &["atelier-minimalism", "crisp-product-workbench"],
        S::Venue => &["kinetic-editorial", "atelier-minimalism"],
        S::Showcase => &["kinetic-editorial", "spatial-webgl-stage"],
        S::Editorial => &["block-first-editorial", "swiss-editorial-grid"],
        S::General => &["swiss-editorial-grid", "atelier-minimalism"],
    };
    let (v, m, dn) = match signals.surface {
        S::ProductUi => (3, 2, 7),
        S::DevTool => (5, 4, 6),
        S::Showcase => (7, 7, 4),
        S::Editorial => (5, 3, 6),
        _ => (5, 4, 5),
    };
    let motion_language = match signals.surface {
        S::ProductUi => "State changes only: value tweens and row insert/remove; nothing scroll-driven",
        S::DevTool => "Show the tool working (transcript or playground); copy and run states; little else",
        S::Showcase => "The work moves; the interface stays still around it",
        _ => "Decide from the subject: motion that shows the trade's objects working, never decoration",
    };
    IndustryRule {
        slug: "brief-derived".into(),
        name: format!("{} (read from the brief)", cap(&subject.phrase)),
        keywords: tokenize(&subject.phrase),
        recommended_styles: styles.iter().map(|s| s.to_string()).collect(),
        layout_archetype: format!("{}: {} and '{}' lead the first screen; {} organise the rest", cap(&subject.phrase), o1, act, note),
        first_viewport_priority: format!("'{}' working next to {}", act, o1),
        typography_pairing: typography,
        palette,
        conversion_trust_elements: vec![
            "Real names, places and opening hours or response times".into(),
            format!("Specifics in the trade's own terms ({note})"),
            format!("One clear primary action: {act}"),
        ],
        motion_language: motion_language.into(),
        anti_patterns: derived_cliches(signals),
        default_dials: DesignDials::clamped(v, m, dn),
        provenance: format!(
            "Synthesised from the brief's own words (\"{}\"); no catalogue industry matched, so check it against what the user meant.",
            query.lines().next().unwrap_or("").chars().take(120).collect::<String>()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intelligence::{brief_signals, wcag_contrast};

    #[test]
    fn law_and_money_briefs_never_get_navy_and_gold() {
        let navy = crate::intelligence::palette("x", "#1B2A4A", "#22335A", "#F4F2EC", "#C8CCDA", "#C9A54C", "#8FB7C9", 0.0, "");
        assert!(navy_gold(&navy));
        for q in ["law firm in Boston", "family law attorney", "private bank for founders"] {
            let sig = brief_signals(q);
            for v in 0..8 {
                for d in deal(&sig, q, v, None) {
                    assert!(!navy_gold(&d.palette), "{q} v{v} dealt navy-and-gold {}", d.palette.id);
                }
            }
        }
    }

    #[test]
    fn later_variants_stay_on_the_brief_s_own_subject() {
        let types = type_pool();
        let pals = palette_pool();
        for q in ["law firm in Boston", "kids museum with hands-on exhibits", "seafood restaurant by the harbour"] {
            let sig = crate::intelligence::brief_signals(q);
            let has = |t: &str| sig.niche_tags.iter().any(|x| x == t) || sig.moods.iter().any(|m| m == t);
            for v in 0..7 {
                for d in deal(&sig, q, v, None) {
                    let te = types.iter().find(|e| e.pairing.id == d.typography.id).unwrap();
                    let pe = pals.iter().find(|e| e.palette.id == d.palette.id).unwrap();
                    assert!(!foreign(te.tags, &has), "{q} v{v}: type {} carries {:?}", te.pairing.id, te.tags);
                    assert!(!foreign(pe.tags, &has), "{q} v{v}: palette {} carries {:?}", pe.palette.id, pe.tags);
                }
            }
        }
    }

    #[test]
    fn pools_are_wide_valid_and_aa() {
        let t = type_pool();
        let p = palette_pool();
        assert!(t.len() >= 35 && p.len() >= 35, "{} type, {} palettes", t.len(), p.len());
        let mut ids = std::collections::HashSet::new();
        for e in &t {
            assert!(ids.insert(e.pairing.id.clone()), "duplicate {}", e.pairing.id);
            assert!(e.pairing.google_fonts_url.starts_with("https://fonts.googleapis.com/css2?family="), "{}", e.pairing.id);
            assert!(e.pairing.google_fonts_url.contains(&e.pairing.display_font.replace(' ', "+")), "{}", e.pairing.id);
        }
        for e in &p {
            assert!(ids.insert(e.palette.id.clone()), "duplicate {}", e.palette.id);
            let body = wcag_contrast(&e.palette.foreground, &e.palette.background).unwrap();
            let muted = wcag_contrast(&e.palette.muted, &e.palette.background).unwrap();
            let on_surface = wcag_contrast(&e.palette.foreground, &e.palette.surface).unwrap();
            assert!(body >= 4.5 && muted >= 4.5 && on_surface >= 4.5, "{} fails AA: body {body:.2}, muted {muted:.2}, surface {on_surface:.2}", e.palette.id);
            assert_eq!(Tone::of(&e.palette), e.tone, "{} tone", e.palette.id);
        }
        // Every tone exists for most niches, so a re-deal never has to leave the brief's world.
        for niche in ["marine", "warm-grain", "textile", "kinetic", "botanical", "industrial", "earth", "code", "legal", "play"] {
            let tones: std::collections::HashSet<_> = p.iter().filter(|e| e.tags.contains(&niche)).map(|e| e.tone as u8).collect();
            assert!(tones.len() >= 2, "{niche}: {tones:?}");
        }
    }

    #[test]
    fn every_surface_has_options_in_every_slot() {
        for s in [S::ProductUi, S::Commerce, S::Venue, S::Showcase, S::Editorial, S::DevTool, S::General] {
            for slot in 0..3 {
                let n = ARCHETYPES.iter().filter(|a| a.slot == slot && fits(a, s)).count();
                assert!(n >= 2, "{s:?} slot {slot}: {n}");
            }
        }
        let splits = ARCHETYPES.iter().filter(|a| a.composition == "split").count();
        assert!(splits * 5 <= ARCHETYPES.len(), "{splits} split archetypes");
    }

    #[test]
    fn subject_is_read_from_the_brief() {
        let s = |q: &str| read_subject(q, &brief_signals(q));
        assert_eq!(s("law firm").phrase, "law firm");
        assert_eq!(s("law firm").action, "book a first consultation");
        assert_eq!(s("Marketing site for an open-source MCP server").phrase, "open-source mcp server");
        assert_eq!(s("seafood restaurant on the harbour with a daily catch menu").phrase, "seafood restaurant");
        let kids = s("kids' museum");
        assert!(kids.objects[0].contains("hands-on"), "{:?}", kids.objects);
        assert!(kids.objects.iter().any(|o| o.contains("plinth")), "{:?}", kids.objects);
        // Product-UI trades lead a bakery sales dashboard, not the bakery
        let dash = s("bakery sales dashboard");
        assert_eq!(dash.objects[0], "the day's takings");
        assert!(dash.objects.contains(&"today's numbers".to_string()), "{:?}", dash.objects);
        let unknown = s("A detailed start page");
        assert!(!unknown.from_trades);
        assert_eq!(unknown.phrase, "detailed start");
    }
}
