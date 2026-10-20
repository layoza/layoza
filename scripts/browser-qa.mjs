#!/usr/bin/env node
/**
 * Layoza Real Browser QA & Site DNA Extraction Engine (Playwright + Chromium)
 *
 * Loads a live URL or local HTML file in headless Chromium at Desktop (1440x900), Tablet (768x1024),
 * and Mobile (375x812) and measures — never assumes — the following:
 *  - Multi-viewport screenshots (+ 1:1 mobile hero and, when a menu toggle exists, the opened menu)
 *  - Layout geometry: document overflow, element bleed, clipped text, interactive-control overlap,
 *    header/badge wrapping, side-by-side heading crowding, cramped display leading, runaway canvases,
 *    and a hidden desktop nav without a working mobile toggle
 *  - WCAG 2.2 AA: per-text-element contrast against the composited ancestor background,
 *    missing alt text, unlabeled controls, 24px target size (2.5.8), heading hierarchy, landmarks,
 *    `:focus-visible` and `prefers-reduced-motion` rules
 *  - Text over canvas/video/large images/background images: real-pixel contrast and busyness of what
 *    is behind the text (captured with the text hidden), and text that something paints over
 *  - Motion settle: each viewport waits (capped at 5s) for finite animations to finish and the first
 *    screen to be pixel-stable before it is measured; `motion_unsettled_viewports` lists cap hits
 *  - Laboratory Core Web Vitals: LCP and CLS from PerformanceObserver, and an INP proxy from Event Timing
 *    entries recorded during scripted keyboard interactions. All are labeled LaboratorySynthetic.
 *
 *  - Non-blocking design critique ("craft") findings: design anti-pattern rules ported from
 *    pbakaus/impeccable (Apache-2.0), commit 2f98774, crates/core/src/...; modified by Layoza to run
 *    in-page on computed styles and to honour a brand opt-out
 *    `<meta name="layoza-brand-allow" content="font:Inter, palette:cream">`. Emitted as issues with
 *    rule `craft-<upstream-id>` and severity "critique", plus a top-level `craft_critique` summary.
 *    Full attribution, modifications and the opt-out syntax are documented above `layozaCraftCritique`.
 *    `craft-heading-rhythm` (heading crowded against the block above) is adapted from
 *    pbakaus/impeccable (Apache-2.0) commit 2f98774, crates/core/src/browser/page_checks.rs
 *    (`check_heading_rhythm_dom` and its `rhythm_*` helpers); modified by Layoza: re-implemented as
 *    in-page JavaScript on live geometry, with the simplifications listed at the rule ("Layoza:").
 *
 *  - Design-system conformance (Layoza-original; upstream impeccable's design-system-* rule ids only
 *    inspired the idea, no upstream code is used): with `--design-tokens <json>` (written by
 *    `layoza qa` from the project's DESIGN.md or SITE-DNA.json), rendered text/background colours
 *    (CIEDE2000), font families, border radii and font sizes are compared with the project's own
 *    declared tokens at every viewport. Off-system values are emitted as `design-system-color|font|
 *    radius|font-size` issues with severity "warning", naming the nearest token. Without tokens no
 *    rule runs and the report carries `"design_tokens": null`.
 *
 *  - Motion density and pinned stages (Layoza-original, desktop only, fresh load after all other
 *    checks): `motion-too-sparse` warns when fewer than 6 distinct motion ideas (3 on calm pages:
 *    `<meta name="layoza-motion" content="calm">` or an undeclared table/grid app surface) change
 *    across scroll positions or run as CSS/WAAPI/scroll-timeline animations; `none` skips it.
 *    A page with a live signature set piece (a pinned stage that changes, or a scroll-timeline
 *    animation) needs only 2: the signature plus quiet feedback is the intended shape.
 *    `pinned-stage-static` warns when a sticky/pinned stage taller than half the viewport looks the
 *    same at 25/50/75% of its pinned range (< 2% of pixels change). Summary in `motion_summary`.
 *
 * Any metric that could not be measured is reported as `null` and listed in `unmeasured`;
 * consumers must treat `null` as "not verified", never as a pass.
 *
 * Uses the Node `playwright` package when installed, otherwise Python `playwright.sync_api`.
 * Both paths run the same in-page scripts and share one report assembler.
 */

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";

const VIEWPORTS = [
  { label: "desktop", width: 1440, height: 900 },
  { label: "tablet", width: 768, height: 1024 },
  { label: "mobile", width: 375, height: 812 },
];

// The in-page DOM eval tags the chosen menu toggle so both runners click the same element
const MENU_TOGGLE_SELECTOR = "[data-layoza-menu-toggle]";

// Installed before any page script runs; buffers LCP, layout shifts, and event timings.
const PERF_INIT_SCRIPT = `(() => {
  const s = (window.__layozaPerf = { lcp: null, cls: 0, maxEventMs: null, eventCount: 0, supported: {} });
  function observe(type, cb, extra) {
    try {
      new PerformanceObserver((list) => list.getEntries().forEach(cb)).observe(Object.assign({ type, buffered: true }, extra || {}));
      s.supported[type] = true;
    } catch (e) {
      s.supported[type] = false;
    }
  }
  observe("largest-contentful-paint", (e) => { s.lcp = e.renderTime || e.startTime; });
  observe("layout-shift", (e) => { if (!e.hadRecentInput) s.cls += e.value; });
  const onEvent = (e) => {
    s.eventCount++;
    s.maxEventMs = Math.max(s.maxEventMs || 0, e.duration);
  };
  observe("first-input", onEvent);
  observe("event", onEvent, { durationThreshold: 16 });
})();`;

const PERF_READ_SCRIPT = `(() => {
  const s = window.__layozaPerf;
  if (!s) return null;
  return {
    lcp_ms: s.supported["largest-contentful-paint"] && s.lcp != null ? Math.round(s.lcp) : null,
    cls: s.supported["layout-shift"] ? Number(s.cls.toFixed(4)) : null,
    inp_proxy_ms: s.maxEventMs != null ? Math.round(s.maxEventMs) : null,
    event_entries: s.eventCount,
  };
})()`;

// Scrolls through the page like a visitor so scroll-triggered reveals run before measuring,
// then reports how much text is still invisible (opacity 0 / hidden) after the full pass.
const SCROLL_PASS_SCRIPT = `(async () => {
  const step = Math.max(200, Math.floor(window.innerHeight * 0.7));
  // behavior "instant" bypasses CSS scroll-behavior: smooth, which would otherwise swallow quick jumps
  const go = (top) => window.scrollTo({ top, left: 0, behavior: "instant" });
  // Mid-reveal sampling: a visitor who stops scrolling sees the page ~1s later. Headings in view that are
  // still faded at that moment read as broken frames (2026-10 blind juries). Layoza-original.
  const mid = [];
  const slow = [];
  const eff = (el) => { let o = 1; for (let c = el; c && c.nodeType === 1; c = c.parentElement) { const st = getComputedStyle(c); if (st.display === "none" || st.visibility === "hidden") return -1; o *= parseFloat(st.opacity); } return o; };
  // Text-on-text overlap at each resting scroll position: line boxes of two different visible text
  // runs that cover each other. Compared only within one scroll layer (same nearest fixed/sticky
  // ancestor), so a sticky header passing over content is not a hit but text inside one pinned
  // section is. Decorative layers are skipped: aria-hidden, effective opacity < 0.35, or a stacked
  // copy of the same words (outline/fill effects). Layoza-original.
  const overlaps = [];
  const layerOf = (el) => { for (let c = el; c && c !== document.body; c = c.parentElement) { const p = getComputedStyle(c).position; if (p === "fixed" || p === "sticky") return c; } return null; };
  const visibleBox = (el, r) => {
    let box = { left: r.left, right: r.right, top: r.top, bottom: r.bottom };
    let depth = 0;
    for (let c = el; c && c !== document.body && depth < 10; c = c.parentElement, depth++) {
      const st = getComputedStyle(c);
      // Screen-reader-only text (clip: rect(0 0 0 0)) has no visible box
      if (st.position === "absolute" && st.clip && st.clip.startsWith("rect(0")) return { left: 0, right: 0, top: 0, bottom: 0 };
      if (st.overflow === "visible" && st.overflowX === "visible" && st.overflowY === "visible" && st.clipPath === "none") continue;
      const cr = c.getBoundingClientRect();
      box = { left: Math.max(box.left, cr.left), right: Math.min(box.right, cr.right), top: Math.max(box.top, cr.top), bottom: Math.min(box.bottom, cr.bottom) };
    }
    return box;
  };
  // Bars (fixed/sticky headers) are scored by how much they hide what passes beneath: the most
  // opaque background between the text and the bar, raised by a strong backdrop blur.
  const bleeds = [];
  const floats = [];
  const alphaOf = (col) => {
    if (!col || col === "transparent") return 0;
    if (!col.startsWith("rgba")) return 1;
    const parts = col.slice(col.indexOf("(") + 1, col.indexOf(")")).split(",");
    return parseFloat(parts[3]) || 0;
  };
  const barOpacity = (el, layer) => {
    let best = 0;
    for (let c = el; c; c = c.parentElement) {
      const st = getComputedStyle(c);
      let a = alphaOf(st.backgroundColor) * (parseFloat(st.opacity) || 1);
      const bf = st.backdropFilter || st.webkitBackdropFilter || "";
      if (bf.includes("blur(")) { const px = parseFloat(bf.slice(bf.indexOf("blur(") + 5)) || 0; if (px >= 12) a = Math.max(a, 0.97); }
      best = Math.max(best, a);
      if (c === layer) break;
    }
    return best;
  };
  // Display headings must fit: no word split across two lines, no glyphs cut by the viewport or a
  // clipping ancestor. Checked per word with Range rects at each resting position.
  const breaks = [];
  const headingFit = (h, y) => {
    if (breaks.length >= 8) return;
    // A heading whose own box is clipped sideways sits on a carousel card scrolled part-way out of
    // view; its words are cut by the scroll position, not by the type size, so skip it.
    const hr = h.getBoundingClientRect(), hv = visibleBox(h, hr);
    if (Math.min(hv.right, window.innerWidth) < hr.right - 2 || Math.max(hv.left, 0) > hr.left + 2) return;
    const tw = document.createTreeWalker(h, NodeFilter.SHOW_TEXT);
    for (let n = tw.nextNode(); n; n = tw.nextNode()) {
      const el = n.parentElement;
      if (!el || el.closest("[aria-hidden='true'], svg") || eff(el) < 0.35) continue;
      const er = el.getBoundingClientRect();
      if (er.width <= 2 || er.height <= 2) continue; // screen-reader-only copy
      const s = n.nodeValue || "";
      let i = 0;
      for (const word of s.split(" ")) {
        const start = i; i += word.length + 1;
        if (word.trim().length < 3 || word.includes("-") || word.includes(String.fromCharCode(173))) continue;
        const rg = document.createRange();
        rg.setStart(n, start); rg.setEnd(n, start + word.length);
        const rects = [...rg.getClientRects()].filter((r) => r.width > 1 && r.height > 1);
        if (!rects.length) continue;
        const tops = new Set(rects.map((r) => Math.round(r.top / 4)));
        const head = (h.textContent || "").trim().slice(0, 30);
        let why = "";
        if (tops.size > 1) why = "word '" + word.trim() + "' splits across lines";
        else {
          const r = rects[0], vb = visibleBox(el, r);
          const cut = Math.max(0, r.right - Math.min(vb.right, window.innerWidth), Math.max(vb.left, 0) - r.left);
          if (r.top >= 0 && r.bottom <= window.innerHeight && cut > Math.max(4, r.width * 0.08)) why = "word '" + word.trim() + "' is cut off by " + Math.round(cut) + "px";
        }
        if (why && !breaks.some((m) => m.startsWith(head + ": "))) breaks.push(head + ": " + why + " @" + Math.round(y) + "px");
      }
    }
  };
  // Sparse resting frames: share of a 24x16 viewport grid touched by visible text or media, outside
  // fixed/sticky bars. A pinned stage holding one small card in a dark void scores low here.
  const sparse = [];
  const coverageAt = (y) => {
    const W = window.innerWidth, H = window.innerHeight, cols = 24, rows = 16;
    const cells = new Uint8Array(cols * rows);
    const inBar = (el) => { const L = layerOf(el); return !!L && L.getBoundingClientRect().height < H * 0.25; };
    const mark = (b) => {
      const x0 = Math.max(0, Math.floor(b.left / W * cols)), x1 = Math.min(cols - 1, Math.floor((b.right - 1) / W * cols));
      const y0 = Math.max(0, Math.floor(b.top / H * rows)), y1 = Math.min(rows - 1, Math.floor((b.bottom - 1) / H * rows));
      for (let r = y0; r <= y1; r++) for (let c = x0; c <= x1; c++) cells[r * cols + c] = 1;
    };
    const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    let count = 0;
    for (let n = walker.nextNode(); n && count < 2000; n = walker.nextNode()) {
      if (!(n.nodeValue || "").trim()) continue;
      const el = n.parentElement;
      if (!el || el.closest("script, style, noscript, template, [hidden], [inert], dialog, [popover], details:not([open]) > :not(summary)") || inBar(el) || eff(el) < 0.35) continue;
      const range = document.createRange();
      range.selectNodeContents(n);
      for (const r of range.getClientRects()) { if (r.width > 2 && r.bottom > 0 && r.top < H) { mark(visibleBox(el, r)); count++; } }
    }
    for (const m of document.querySelectorAll("img, svg, canvas, video, picture, iframe")) {
      if (m.parentElement && m.parentElement.closest("svg")) continue; // nested svg counted via its outer svg
      const r = m.getBoundingClientRect();
      if (r.width < 24 || r.height < 24 || r.bottom <= 0 || r.top >= H || inBar(m) || eff(m) < 0.35) continue;
      if (r.width >= W * 0.95 && r.height >= H * 0.95) continue; // full-bleed backdrop says nothing about content
      mark(visibleBox(m, r));
    }
    const share = cells.reduce((s, v) => s + v, 0) / cells.length;
    const atEnd = y + H >= document.documentElement.scrollHeight - 4;
    if (share < 0.1 && !atEnd && sparse.length < 8) sparse.push(Math.round(share * 100) + "% @" + Math.round(y) + "px");
  };
  const textOverlapAt = (y) => {
    if (overlaps.length >= 12) return;
    const runs = [];
    const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    for (let n = walker.nextNode(); n && runs.length < 1500; n = walker.nextNode()) {
      const t = (n.nodeValue || "").trim();
      if (t.length < 2) continue;
      const el = n.parentElement;
      if (!el || el.closest("script, style, noscript, template, svg, [aria-hidden='true'], [hidden], [inert], dialog, [popover], details:not([open]) > :not(summary)")) continue;
      const o = eff(el);
      if (o < 0.35) continue;
      const range = document.createRange();
      range.selectNodeContents(n);
      for (const r of range.getClientRects()) {
        if (r.width < 4 || r.height < 4 || r.bottom < 0 || r.top > window.innerHeight) continue;
        const b = visibleBox(el, r);
        if (b.right - b.left < 4 || b.bottom - b.top < 4) continue;
        runs.push({ n, el, t, b, layer: layerOf(el) });
      }
    }
    runs.sort((a, b) => a.b.top - b.b.top);
    // Floating controls (a round menu button, a chat bubble, a back-to-top pill) that sit on body text
    if (floats.length < 6) {
      const ctrls = [...document.querySelectorAll("button, a[href], [role='button']")].filter((c) => {
        const L = layerOf(c);
        if (!L || c.closest("[aria-hidden='true'], [hidden], [inert], dialog, [popover], details:not([open]) > :not(summary)")) return false;
        const lr = L.getBoundingClientRect(), cr = c.getBoundingClientRect();
        // floating = its own small layer, or a wide bar whose background is see-through (a hidden or transparent header)
        const floating = L === c || (lr.width < window.innerWidth * 0.4 && lr.height < window.innerHeight * 0.4) || barOpacity(c.parentElement, L) < 0.5;
        return floating && cr.width >= 16 && cr.height >= 16 && cr.width < window.innerWidth * 0.5 && cr.height < 120 && cr.bottom > 0 && cr.top < window.innerHeight && eff(c) >= 0.5;
      });
      for (const c of ctrls) {
        const cr = c.getBoundingClientRect();
        for (const r of runs) {
          if (r.layer || c.contains(r.el)) continue;
          const w = Math.min(cr.right, r.b.right) - Math.max(cr.left, r.b.left);
          const h = Math.min(cr.bottom, r.b.bottom) - Math.max(cr.top, r.b.top);
          if (w <= 6 || h < (r.b.bottom - r.b.top) * 0.5) continue;
          const key = (c.getAttribute("aria-label") || c.textContent || c.tagName).trim().slice(0, 20) + " over " + r.t.slice(0, 24);
          if (!floats.some((m) => m.startsWith(key)) && floats.length < 6) floats.push(key + " @" + Math.round(y) + "px");
          break;
        }
      }
    }
    for (let i = 0; i < runs.length && overlaps.length < 12; i++) {
      const a = runs[i];
      for (let j = i + 1; j < runs.length && runs[j].b.top < a.b.bottom; j++) {
        const c = runs[j];
        if (c.n === a.n || c.el === a.el) continue;
        if (c.layer !== a.layer) {
          // Content scrolling under a fixed/sticky bar: a hit only when the bar lets the text read through.
          const top = a.layer ? a : c.layer ? c : null, under = top === a ? c : a;
          if (!top || under.layer) continue;
          // A tall sticky/fixed panel (sidebar, summary column) is not a bar: content must never pass beneath it.
          const tr = top.layer.getBoundingClientRect();
          if (tr.height >= window.innerHeight * 0.25 && tr.width < window.innerWidth * 0.9) {
            const pw = Math.min(a.b.right, c.b.right) - Math.max(a.b.left, c.b.left);
            const ph = Math.min(a.b.bottom, c.b.bottom) - Math.max(a.b.top, c.b.top);
            if (pw > 4 && ph > 2 && overlaps.length < 12) {
              const pk = top.t.slice(0, 24) + " / " + under.t.slice(0, 24);
              if (!overlaps.some((m) => m.startsWith(pk))) overlaps.push(pk + " (sticky panel covers content) @" + Math.round(y) + "px");
            }
            continue;
          }
          if (bleeds.length >= 8) continue;
          const bw = Math.min(a.b.right, c.b.right) - Math.max(a.b.left, c.b.left);
          const bh = Math.min(a.b.bottom, c.b.bottom) - Math.max(a.b.top, c.b.top);
          if (bw <= 4 || bh <= 2 || barOpacity(top.el, top.layer) >= 0.97) continue;
          const key = under.t.slice(0, 24) + " under " + top.t.slice(0, 24);
          if (!bleeds.some((m) => m.startsWith(key))) bleeds.push(key + " @" + Math.round(y) + "px");
          continue;
        }
        const w = Math.min(a.b.right, c.b.right) - Math.max(a.b.left, c.b.left);
        const h = Math.min(a.b.bottom, c.b.bottom) - Math.max(a.b.top, c.b.top);
        if (w <= 4 || h <= 0) continue;
        const minArea = Math.min((a.b.right - a.b.left) * (a.b.bottom - a.b.top), (c.b.right - c.b.left) * (c.b.bottom - c.b.top));
        const minH = Math.min(a.b.bottom - a.b.top, c.b.bottom - c.b.top);
        if (h < minH * 0.4 || w * h < minArea * 0.4) continue;
        const ta = a.el.textContent.trim().toLowerCase(), tc = c.el.textContent.trim().toLowerCase();
        if (ta === tc || ta.includes(tc) || tc.includes(ta)) continue;
        const key = a.t.slice(0, 24) + " / " + c.t.slice(0, 24);
        if (overlaps.some((m) => m.startsWith(key))) continue;
        overlaps.push(key + " @" + Math.round(y) + "px");
      }
    }
  };
  let steps = 0;
  for (let y = 0; y < document.documentElement.scrollHeight; y += step) {
    go(y);
    await new Promise((r) => setTimeout(r, 200));
    if (steps++ >= 40) continue;
    await new Promise((r) => setTimeout(r, 700));
    for (const h of document.querySelectorAll("h1, h2, h3")) {
      const txt = (h.textContent || "").trim();
      if (!txt || h.closest("[aria-hidden='true'], [hidden], [inert], dialog:not([open]), details:not([open]) > :not(summary)")) continue;
      const r = h.getBoundingClientRect();
      if (r.width < 4 || r.height < 4 || r.top < 0 || r.bottom > window.innerHeight) continue;
      headingFit(h, y);
      const o = eff(h);
      if (o < 0) continue;
      // Split-text reveals fade or clip per word/char span, so sample the leaves too.
      const leaves = [...h.querySelectorAll("*")].filter((c) => !c.children.length && (c.textContent || "").trim());
      const faded = leaves.filter((c) => { const co = eff(c); const cr = c.getBoundingClientRect(); return co >= 0 && (co < 0.55 || cr.height < 1); }).length;
      let clipped = false;
      for (let c = h; c && c !== document.body; c = c.parentElement) { const cp = getComputedStyle(c).clipPath; if (cp && cp !== "none" && cp.startsWith("inset") && (cp.match(/[0-9.]+%/g) || []).some((v) => parseFloat(v) >= 10)) { clipped = true; break; } }
      const leafShare = leaves.length >= 2 ? faded / leaves.length : 0;
      if ((o < 0.55 || leafShare >= 0.3 || clipped) && mid.length + slow.length < 20 && !mid.some((m) => m.startsWith(txt.slice(0, 30) + " @")) && !slow.some((m) => m.label.startsWith(txt.slice(0, 30) + " @"))) slow.push({ h, label: txt.slice(0, 30) + " @" + Math.round(y) + "px (" + (o < 0.55 ? "opacity " + o.toFixed(2) : clipped ? "clipped" : Math.round(leafShare * 100) + "% of words faded") + " at 0.9s" });
    }
    // Still faded ~0.9s after scrolling stopped is already too slow (text-mid-reveal); keep watching up
    // to ~3s so the report says whether the reveal lands late or never lands at this position
    const readable = (h) => { const o = eff(h); if (o < 0) return true; if (o < 0.9) return false; const lv = [...h.querySelectorAll("*")].filter((c) => !c.children.length && (c.textContent || "").trim()); return !lv.some((c) => { const co = eff(c); return co >= 0 && co < 0.9; }); };
    for (let waited = 900; slow.length && waited < 3000; waited += 300) {
      await new Promise((r) => setTimeout(r, 300));
      for (let i = slow.length - 1; i >= 0; i--) {
        if (readable(slow[i].h)) { mid.push(slow[i].label + "; readable after ~" + ((waited + 300) / 1000).toFixed(1) + "s)"); slow.splice(i, 1); }
      }
    }
    for (const m of slow.splice(0)) mid.push(m.label + "; still faded at 3.0s)");
    textOverlapAt(y);
    coverageAt(y);
  }
  window.__layozaMidReveal = mid;
  window.__layozaTextOverlap = overlaps;
  window.__layozaBarBleed = bleeds;
  window.__layozaFloatCover = floats;
  window.__layozaHeadingBreak = breaks;
  window.__layozaSparse = sparse;
  go(document.documentElement.scrollHeight);
  await new Promise((r) => setTimeout(r, 500));
  go(0);
  await new Promise((r) => setTimeout(r, 600));
})()`;

// Counts header nav links that are visible right now (used before/after clicking the menu toggle).
const VISIBLE_NAV_LINKS_SCRIPT = `(() => {
  // A link counts only if it is actually on top at its centre point (not clipped by a collapsed drawer or covered)
  const links = Array.from(document.querySelectorAll("nav a[href], [role='navigation'] a[href], [role='dialog'] a[href]"));
  return links.filter((a) => {
    const r = a.getBoundingClientRect();
    if (r.width <= 2 || r.height <= 2 || r.bottom <= 0 || r.top >= window.innerHeight) return false;
    const x = Math.min(window.innerWidth - 1, Math.max(0, r.left + r.width / 2));
    const y = Math.min(window.innerHeight - 1, Math.max(0, r.top + r.height / 2));
    const hit = document.elementFromPoint(x, y);
    return Boolean(hit && (hit === a || a.contains(hit)));
  }).length;
})()`;

// art-over-text (Layoza-original). DOM checks cannot see canvas pixels, so the collector screenshots
// each resting frame twice (art shown, art hidden) and diffs the pixels inside text line boxes.
// Step 1: tag visible canvas/video art layers (>= 15% of the viewport area) and return the visible
// text line boxes that intersect them, each with its intersection rect (the only pixels that may change).
const ART_LINES_SCRIPT = `(() => {
  const W = window.innerWidth, H = window.innerHeight;
  for (const m of document.querySelectorAll("[data-layoza-art]")) m.removeAttribute("data-layoza-art");
  const eff = (el) => { let o = 1; for (let c = el; c && c.nodeType === 1; c = c.parentElement) { const st = getComputedStyle(c); if (st.display === "none" || st.visibility === "hidden") return 0; o *= parseFloat(st.opacity); } return o; };
  const arts = [];
  // Decorative (aria-hidden) outer SVG art counts too: a drawn ribbon crosses text just like a canvas one
  for (const m of document.querySelectorAll("canvas, video, svg[aria-hidden='true']")) {
    if (m.tagName.toLowerCase() === "svg" && m.parentElement && m.parentElement.closest("svg")) continue;
    const r = m.getBoundingClientRect();
    if (r.width * r.height < W * H * 0.15 || eff(m) < 0.05) continue;
    const a = { l: Math.max(0, r.left), t: Math.max(0, r.top), r: Math.min(W, r.right), b: Math.min(H, r.bottom) };
    if (a.r - a.l < 8 || a.b - a.t < 8) continue;
    m.setAttribute("data-layoza-art", "1");
    arts.push(a);
  }
  if (!arts.length) return { arts: 0, lines: [] };
  const clipBox = (el, r) => {
    let box = { l: r.left, r: r.right, t: r.top, b: r.bottom };
    let depth = 0;
    for (let c = el; c && c !== document.body && depth < 10; c = c.parentElement, depth++) {
      const st = getComputedStyle(c);
      if (st.position === "absolute" && st.clip && st.clip.startsWith("rect(0")) return null;
      if (st.overflow === "visible" && st.overflowX === "visible" && st.overflowY === "visible" && st.clipPath === "none") continue;
      const cr = c.getBoundingClientRect();
      box = { l: Math.max(box.l, cr.left), r: Math.min(box.r, cr.right), t: Math.max(box.t, cr.top), b: Math.min(box.b, cr.bottom) };
    }
    box = { l: Math.max(0, box.l), r: Math.min(W, box.r), t: Math.max(0, box.t), b: Math.min(H, box.b) };
    return box.r - box.l >= 4 && box.b - box.t >= 4 ? box : null;
  };
  // Line grouping key: the text element (heading, paragraph, link...) or else the nearest block box.
  // Flex children are blockified, so split-text letter spans would otherwise each be their own block.
  // Text on its own solid plate (alpha >= 0.8, or a strong backdrop blur) between it and the art reads fine
  const alphaOf = (col) => {
    if (!col || col === "transparent") return 0;
    if (!col.startsWith("rgba")) return 1;
    const parts = col.slice(col.indexOf("(") + 1, col.indexOf(")")).split(",");
    return parseFloat(parts[3]) || 0;
  };
  const plates = new Map();
  const onPlate = (el) => {
    if (plates.has(el)) return plates.get(el);
    let res = false;
    for (let c = el; c && c !== document.body; c = c.parentElement) {
      if (c.querySelector("[data-layoza-art]")) break;
      const st = getComputedStyle(c);
      const bf = st.backdropFilter || st.webkitBackdropFilter || "";
      const blur = bf.includes("blur(") ? parseFloat(bf.slice(bf.indexOf("blur(") + 5)) || 0 : 0;
      if (alphaOf(st.backgroundColor) * (parseFloat(st.opacity) || 1) >= 0.8 || st.backgroundImage.includes("gradient(") || blur >= 12) { res = true; break; }
    }
    plates.set(el, res);
    return res;
  };
  const blockOf = (el) => {
    const t = el.closest("h1, h2, h3, h4, h5, h6, p, li, a, button, label, td, th, dt, dd, figcaption, blockquote, summary, legend");
    if (t) return t;
    for (let c = el; c && c !== document.body; c = c.parentElement) { if (!getComputedStyle(c).display.startsWith("inline")) return c; }
    return document.body;
  };
  // Word-level rects so each line box carries its own words
  const words = [];
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
  for (let n = walker.nextNode(); n && words.length < 4000; n = walker.nextNode()) {
    const s = n.nodeValue || "";
    if (s.trim().length < 1) continue;
    const el = n.parentElement;
    if (!el || el.closest("script, style, noscript, template, svg, canvas, video, [hidden], dialog:not([open])")) continue;
    if (el.closest("details:not([open])") && !el.closest("summary")) continue;
    // aria-hidden text is decorative, except split-text letters whose parent carries the aria-label
    const ah = el.closest("[aria-hidden='true']");
    if (ah && !(ah.parentElement && ah.parentElement.hasAttribute("aria-label"))) continue;
    const er = el.getBoundingClientRect();
    if (er.bottom < 0 || er.top > H || er.width < 2 || er.height < 2) continue;
    if (eff(el) < 0.35 || onPlate(el)) continue;
    const block = blockOf(el);
    let i = 0;
    for (const w of s.split(" ")) {
      const start = i; i += w.length + 1;
      if (!w.trim()) continue;
      const rg = document.createRange();
      rg.setStart(n, start); rg.setEnd(n, start + w.length);
      for (const r of rg.getClientRects()) {
        if (r.width < 2 || r.height < 4 || r.bottom < 0 || r.top > H) continue;
        const b = clipBox(el, r);
        if (b) words.push({ w: w.trim(), b, block });
      }
    }
  }
  words.sort((x, y) => x.b.t - y.b.t || x.b.l - y.b.l);
  const lines = [];
  for (const wd of words) {
    const h = wd.b.b - wd.b.t;
    const ln = lines.find((L) => L.block === wd.block && Math.min(L.b, wd.b.b) - Math.max(L.t, wd.b.t) >= Math.min(h, L.b - L.t) * 0.6 && wd.b.l - L.r < h * 2 && L.l - wd.b.r < h * 2);
    if (ln) { ln.l = Math.min(ln.l, wd.b.l); ln.r = Math.max(ln.r, wd.b.r); ln.t = Math.min(ln.t, wd.b.t); ln.b = Math.max(ln.b, wd.b.b); ln.words.push(wd); }
    else lines.push({ block: wd.block, l: wd.b.l, r: wd.b.r, t: wd.b.t, b: wd.b.b, words: [wd] });
  }
  const out = [];
  for (const L of lines) {
    const hits = arts.map((a) => ({ l: Math.max(a.l, L.l), t: Math.max(a.t, L.t), r: Math.min(a.r, L.r), b: Math.min(a.b, L.b) })).filter((x) => x.r - x.l >= 2 && x.b - x.t >= 2);
    if (!hits.length) continue;
    // Join in reading order; split-text letters (no gap between their boxes) join without a space
    // and form one visual word (token), so a ribbon through "MOVE" is judged on MOVE, not on the whole line
    const ws = L.words.slice().sort((x, y) => x.b.l - y.b.l);
    const tokens = [];
    ws.forEach((wd, i) => {
      const k = tokens[tokens.length - 1];
      if (i && wd.b.l - ws[i - 1].b.r <= (wd.b.b - wd.b.t) * 0.08) { k.w += wd.w; k.l = Math.min(k.l, wd.b.l); k.r = Math.max(k.r, wd.b.r); k.t = Math.min(k.t, wd.b.t); k.b = Math.max(k.b, wd.b.b); }
      else tokens.push({ w: wd.w, l: wd.b.l, r: wd.b.r, t: wd.b.t, b: wd.b.b });
    });
    const text = tokens.map((k) => k.w).join(" ");
    const near = tokens.filter((k) => k.w.length >= 2 && hits.some((x) => Math.min(x.r, k.r) - Math.max(x.l, k.l) > 0)).slice(0, 30);
    out.push({ text: text.slice(0, 40), l: L.l, t: L.t, r: L.r, b: L.b, inside: hits, tokens: near });
    if (out.length >= 200) break;
  }
  // Screenshot clip: the union of the candidate lines, so both captures stay small
  let clip = null;
  if (out.length) {
    const x0 = Math.max(0, Math.floor(Math.min(...out.map((o) => o.l)))), y0 = Math.max(0, Math.floor(Math.min(...out.map((o) => o.t))));
    const x1 = Math.min(W, Math.ceil(Math.max(...out.map((o) => o.r)))), y1 = Math.min(H, Math.ceil(Math.max(...out.map((o) => o.b))));
    clip = { x: x0, y: y0, width: Math.max(1, x1 - x0), height: Math.max(1, y1 - y0) };
  }
  return { arts: arts.length, lines: out, clip };
})()`;

// Step 2/3: hide the tagged art layers (visibility keeps layout), then restore them.
const ART_HIDE_SCRIPT = `(() => {
  for (const m of document.querySelectorAll("[data-layoza-art]")) {
    m.setAttribute("data-layoza-art-vis", m.style.getPropertyValue("visibility") + "|" + m.style.getPropertyPriority("visibility"));
    m.style.setProperty("visibility", "hidden", "important");
  }
})()`;
const ART_RESTORE_SCRIPT = `(() => {
  for (const m of document.querySelectorAll("[data-layoza-art]")) {
    const prev = (m.getAttribute("data-layoza-art-vis") || "|").split("|");
    if (prev[0]) m.style.setProperty("visibility", prev[0], prev[1]); else m.style.removeProperty("visibility");
    m.removeAttribute("data-layoza-art-vis");
    m.removeAttribute("data-layoza-art");
  }
})()`;
// Step 4: decode both PNG screenshots in-page (no npm image deps) and return, per line, the share of
// its box pixels whose colour changed by more than 40 (sum of abs RGB diff) when the art was hidden.
const ART_DIFF_SCRIPT = `async (p) => {
  const load = (src) => new Promise((res, rej) => { const im = new Image(); im.onload = () => res(im); im.onerror = () => rej(new Error("screenshot decode failed")); im.src = src; });
  const px = (im) => { const c = document.createElement("canvas"); c.width = im.naturalWidth; c.height = im.naturalHeight; const x = c.getContext("2d", { willReadFrequently: true }); x.drawImage(im, 0, 0); return x.getImageData(0, 0, c.width, c.height); };
  // p.shown: one or more art-visible captures (moving art is sampled over time); p.b: the art-hidden capture
  const B = px(await load(p.b));
  let best = null;
  for (const src of p.shown) {
    const A = px(await load(src));
    // Captures cover p.clip (CSS px); line boxes are viewport coordinates
    const s = A.width / p.clip.width, Wd = A.width, Hd = Math.min(A.height, B.height), ox = p.clip.x, oy = p.clip.y;
    // Share of box pixels that changed; only pixels inside the art rects are read (nothing else may change)
    const share = (box, inside) => {
      const area = Math.max(1, Math.round((box.r - box.l) * s) * Math.round((box.b - box.t) * s));
      let changed = 0;
      for (const q of inside) {
        const l = Math.max(q.l, box.l), r = Math.min(q.r, box.r), t = Math.max(q.t, box.t), b = Math.min(q.b, box.b);
        const x0 = Math.max(0, Math.round((l - ox) * s)), x1 = Math.min(Wd, Math.round((r - ox) * s));
        const y0 = Math.max(0, Math.round((t - oy) * s)), y1 = Math.min(Hd, Math.round((b - oy) * s));
        for (let y = y0; y < y1; y++) {
          for (let x = x0; x < x1; x++) {
            const i = (y * Wd + x) * 4;
            if (Math.abs(A.data[i] - B.data[i]) + Math.abs(A.data[i + 1] - B.data[i + 1]) + Math.abs(A.data[i + 2] - B.data[i + 2]) > 40) changed++;
          }
        }
      }
      return Math.min(1, changed / area);
    };
    const cur = p.lines.map((L) => {
      let word = 0, wordText = "";
      for (const k of L.tokens || []) { const c = share(k, L.inside); if (c > word) { word = c; wordText = k.w; } }
      return { line: share(L, L.inside), word, wordText };
    });
    best = best ? best.map((o, i) => ({ line: Math.max(o.line, cur[i].line), word: Math.max(o.word, cur[i].word), wordText: cur[i].word > o.word ? cur[i].wordText : o.wordText })) : cur;
  }
  return best;
}`;
// Cheap line scans at every step (long pages), pixel diffs only where art meets text, capped
const ART_MAX_SCANS = 30;
const ART_MAX_DIFFS = 20;
const ART_COVER_MIN = 0.12; // share of a line box
const ART_WORD_MIN = 0.3; // share of one visual word's box

// Motion settle (Layoza-original). Before a viewport is measured, wait until finite CSS/WAAPI
// animations have finished and the first screen is pixel-stable across two frames ~300ms apart,
// capped at SETTLE_CAP_MS, so half-played intro fades are not scored as resting contrast or layout.
// Endless loops (infinite iterations, scroll-driven timelines, or more than the cap still to run)
// are tagged data-layoza-loop and masked in the stability screenshots together with canvas/video,
// which animate forever by design.
const SETTLE_CAP_MS = 5000;
const SETTLE_STEP_MS = 200;
// Run modes. fast (default): one browser, viewports and side passes in parallel contexts, the full
// rule sweep at 1440 only (768/375 run layout, overflow and touch rules), one cheap reduced-motion
// pass, no throttled perf pass. full (--full): throttled perf pass and the full sweep at every width.
// Waits are event waits (load, fonts.ready, network idle with a cap, a pixel settle with a cap);
// the short fixed waits left are animation-time allowances after a scroll or key press.
const MODE_CAPS = {
  fast: { settle_ms: 2500, netidle_ms: 1500 },
  full: { settle_ms: SETTLE_CAP_MS, netidle_ms: 3000 },
};
const WAIT = { art_step: 250, art_first: 400, key: 150, scroll_anim: 300, pin: 300, reduced: 400 };
// Resolves after two animation frames (layout and paint have caught up with a resize or scroll)
const FRAMES_SCRIPT = "new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(() => r(true))))";
// Node Playwright evaluates a string as a plain expression and never calls it, so a script written
// as "(arg) => ..." returned the function itself (serialised as undefined) and every settle, diff,
// scroll and clip check that took an argument silently read undefined. Python Playwright guesses
// and calls it. ev() calls function-shaped strings with their argument inlined as JSON (a CDP
// expression, so page CSP does not apply), matching the Python runner.
const FN_SOURCE_RE = /^\s*(async\s+)?(function\b|\([^()]*\)\s*=>|[A-Za-z_$][\w$]*\s*=>)/;
function ev(page, expr, arg) {
  if (typeof expr === "string" && FN_SOURCE_RE.test(expr)) {
    return page.evaluate("(" + expr + ")(" + (arg === undefined ? "" : JSON.stringify(arg)) + ")");
  }
  return arg === undefined ? page.evaluate(expr) : page.evaluate(expr, arg);
}
// document.fonts.ready, capped
const FONTS_READY_SCRIPT = "(cap) => Promise.race([document.fonts ? document.fonts.ready.then(() => true) : Promise.resolve(true), new Promise((r) => setTimeout(() => r(false), cap))])";
// Quality-sweep rule groups that depend on the viewport width (fast mode at 768 and 375)
const LAYOUT_ONLY_RULES = ["sidebar-vanishes", "cta-competition", "row-heights-ragged", "scroller-hidden", "text-below-floor", "sticky-under-header"];
const SETTLE_STABLE_MAX = 0.003; // share of changed pixels (at 320px wide) still counted as stable
const SETTLE_MASK_SELECTOR = "canvas, video, iframe, [data-layoza-loop]";
const SETTLE_PENDING_SCRIPT = `((capMs) => {
  for (const m of document.querySelectorAll("[data-layoza-loop]")) m.removeAttribute("data-layoza-loop");
  let list = [];
  try { list = document.getAnimations ? document.getAnimations() : []; } catch (e) {}
  let pending = 0, loops = 0;
  const docTimeline = typeof DocumentTimeline === "undefined" ? null : DocumentTimeline;
  for (const a of list) {
    if (a.playState !== "running") continue;
    const target = a.effect && a.effect.target;
    let t = null;
    try { t = a.effect && a.effect.getComputedTiming ? a.effect.getComputedTiming() : null; } catch (e) {}
    const scrollDriven = docTimeline && a.timeline && !(a.timeline instanceof docTimeline);
    const remaining = t && isFinite(t.endTime) ? t.endTime - (Number(t.localTime) || 0) : Infinity;
    if (scrollDriven || !isFinite(remaining) || remaining > capMs) {
      loops++;
      if (target && target.nodeType === 1) target.setAttribute("data-layoza-loop", "");
      continue;
    }
    pending++;
  }
  return { pending, loops };
})`;

// text-over-graphic (Layoza-original). DOM contrast cannot see pixels behind text set over a canvas,
// video, large image or background image, so at the art pass's resting positions the collector
// screenshots the candidate text boxes twice (text shown, text hidden via visibility) and measures
// the real pixels behind each text: WCAG contrast of the text colour against every sampled pixel
// (10th percentile and median), luminance busyness (CIE L* standard deviation), and whether hiding
// the text changes any pixel at all (if not, something paints over it: occluded).
const TG_MAX_PER_POSITION = 16;
const TG_MAX_TOTAL = 48;
const TG_MAX_POSITIONS = 8;
const TG_BUSY_LSTD = 18; // L* standard deviation behind the text
const TG_OCCLUDED_MAX = 0.004; // share of the text box that changes when the text is hidden
const TG_FIND_SCRIPT = `((max) => {
  const W = window.innerWidth, H = window.innerHeight;
  for (const m of document.querySelectorAll("[data-layoza-tg]")) m.removeAttribute("data-layoza-tg");
  const parse = (str) => { const m = str && str.match(/rgba?\\(([\\d.]+),\\s*([\\d.]+),\\s*([\\d.]+)(?:,\\s*([\\d.]+))?/i); return m ? [Number(m[1]), Number(m[2]), Number(m[3]), m[4] === undefined ? 1 : Number(m[4])] : null; };
  const vis = (el) => { let o = 1; for (let c = el; c && c.nodeType === 1; c = c.parentElement) { const st = getComputedStyle(c); if (st.display === "none" || st.visibility === "hidden") return 0; o *= parseFloat(st.opacity); } return o; };
  // Mid-reveal text (script-driven opacity or a running animation on the chain) belongs to text-mid-reveal
  // (endless ambient loops do not count: they never come to rest)
  const finite = (a) => { try { return isFinite(a.effect.getComputedTiming().endTime); } catch (e) { return false; } };
  const moving = (el) => { for (let c = el; c && c.nodeType === 1; c = c.parentElement) { try { if (c.getAnimations && c.getAnimations().some((a) => a.playState === "running" && finite(a))) return true; } catch (e) {} } return false; };
  const clampR = (r) => ({ l: Math.max(0, r.left), t: Math.max(0, r.top), r: Math.min(W, r.right), b: Math.min(H, r.bottom) });
  const area = (b) => Math.max(0, b.r - b.l) * Math.max(0, b.b - b.t);
  const inter = (a, b) => ({ l: Math.max(a.l, b.l), t: Math.max(a.t, b.t), r: Math.min(a.r, b.r), b: Math.min(a.b, b.b) });
  const name = (e) => e.tagName.toLowerCase() + (e.id ? "#" + e.id : "") + (typeof e.className === "string" && e.className.trim() ? "." + e.className.trim().split(/\\s+/).slice(0, 2).join(".") : "");
  // Graphics that can sit behind text: canvas/video, large images, large outer SVG, large background images
  const graphics = [];
  const addG = (el, kind, minShare) => {
    const r = el.getBoundingClientRect();
    const b = clampR(r);
    if (area(b) < Math.max(120 * 80, W * H * minShare) || vis(el) < 0.05) return;
    graphics.push({ el, kind, b });
  };
  for (const el of document.querySelectorAll("canvas, video")) addG(el, el.tagName.toLowerCase(), 0.02);
  for (const el of document.querySelectorAll("img, picture > img")) addG(el, "img", 0.04);
  for (const el of document.querySelectorAll("svg")) { if (!(el.parentElement && el.parentElement.closest("svg"))) addG(el, "svg", 0.15); }
  const all = document.body ? document.body.getElementsByTagName("*") : [];
  // Fixed/sticky bars (headers, cookie bars) cover content mid-scroll by design: text under them is skipped
  const covers = [];
  for (let i = 0; i < all.length && i < 6000; i++) {
    const el = all[i];
    const st = getComputedStyle(el);
    const bi = st.backgroundImage;
    if (bi && bi !== "none" && (bi.includes("url(") || bi.includes("gradient("))) addG(el, bi.includes("url(") ? "background-image" : "gradient", 0.04);
    if ((st.position === "fixed" || st.position === "sticky") && st.display !== "none" && st.visibility !== "hidden") {
      const b = clampR(el.getBoundingClientRect());
      if (area(b) > 0) covers.push({ el, b });
    }
  }
  if (!graphics.length) return { graphics: 0, candidates: [], clip: null };
  // Text owners: elements with their own visible text in the viewport
  const owners = new Map();
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
  for (let n = walker.nextNode(); n && owners.size < 400; n = walker.nextNode()) {
    if (!(n.nodeValue || "").trim()) continue;
    const el = n.parentElement;
    if (!el || el.closest("script, style, noscript, template, svg, canvas, video, select, option, [hidden], [aria-hidden='true'], dialog:not([open])")) continue;
    if (el.closest("details:not([open])") && !el.closest("summary")) continue;
    const rg = document.createRange();
    rg.selectNodeContents(n);
    for (const r of rg.getClientRects()) {
      if (r.width < 2 || r.height < 4 || r.bottom <= 0 || r.top >= H || r.right <= 0 || r.left >= W) continue;
      const b = clampR(r);
      const o = owners.get(el);
      if (o) { o.l = Math.min(o.l, b.l); o.t = Math.min(o.t, b.t); o.r = Math.max(o.r, b.r); o.b = Math.max(o.b, b.b); o.text += n.nodeValue; }
      else owners.set(el, { ...b, text: n.nodeValue });
    }
  }
  // A solid plate (alpha >= 0.8) between the text and the graphic means the DOM contrast check already applies
  const plateBetween = (el, g) => {
    for (let c = el; c && c !== document.documentElement; c = c.parentElement) {
      if (c === g.el || c.contains(g.el)) return false;
      const bg = parse(getComputedStyle(c).backgroundColor);
      if (bg && bg[3] >= 0.8) return true;
    }
    return false;
  };
  const out = [];
  const seen = new Set();
  for (const [el, box] of owners) {
    if (out.length >= max) break;
    if (area(box) < 40) continue;
    const text = box.text.replace(/\\s+/g, " ").trim();
    if (text.length < 2 || seen.has(text.slice(0, 40))) continue;
    const st = getComputedStyle(el);
    const col = parse(st.color);
    if (!col || col[3] <= 0.05 || vis(el) < 0.9 || moving(el)) continue;
    if (covers.some((c) => !c.el.contains(el) && !el.contains(c.el) && area(inter(box, c.b)) > 0)) continue;
    const hits = graphics.filter((g) => g.el !== el && !el.contains(g.el) && area(inter(box, g.b)) >= area(box) * 0.3);
    if (!hits.length) continue;
    const g = hits.find((x) => !plateBetween(el, x));
    if (!g) continue;
    seen.add(text.slice(0, 40));
    const id = String(out.length);
    el.setAttribute("data-layoza-tg", id);
    const fs = parseFloat(st.fontSize) || 16, fw = parseInt(st.fontWeight, 10) || 400;
    out.push({
      id, text: text.slice(0, 40), el: name(el), graphic: g.kind + " " + name(g.el),
      l: box.l, t: box.t, r: box.r, b: box.b,
      color: col, large: fs >= 24 || (fs >= 18.66 && fw >= 700),
      halo: (st.textShadow && st.textShadow !== "none") || (parseFloat(st.webkitTextStrokeWidth) || 0) > 0,
    });
  }
  let clip = null;
  if (out.length) {
    const x0 = Math.floor(Math.min(...out.map((o) => o.l))), y0 = Math.floor(Math.min(...out.map((o) => o.t)));
    const x1 = Math.ceil(Math.max(...out.map((o) => o.r))), y1 = Math.ceil(Math.max(...out.map((o) => o.b)));
    clip = { x: Math.max(0, x0), y: Math.max(0, y0), width: Math.max(1, Math.min(W, x1) - Math.max(0, x0)), height: Math.max(1, Math.min(H, y1) - Math.max(0, y0)) };
  }
  return { graphics: graphics.length, candidates: out, clip };
})`;
const TG_HIDE_SCRIPT = `(() => {
  for (const m of document.querySelectorAll("[data-layoza-tg]")) {
    m.setAttribute("data-layoza-tg-vis", m.style.getPropertyValue("visibility") + "|" + m.style.getPropertyPriority("visibility") + "|" + m.style.getPropertyValue("transition") + "|" + m.style.getPropertyPriority("transition"));
    m.style.setProperty("transition", "none", "important");
    m.style.setProperty("visibility", "hidden", "important");
  }
})()`;
const TG_RESTORE_SCRIPT = `(() => {
  for (const m of document.querySelectorAll("[data-layoza-tg]")) {
    const p = (m.getAttribute("data-layoza-tg-vis") || "|||").split("|");
    if (p[0]) m.style.setProperty("visibility", p[0], p[1]); else m.style.removeProperty("visibility");
    if (p[2]) m.style.setProperty("transition", p[2], p[3]); else m.style.removeProperty("transition");
    m.removeAttribute("data-layoza-tg-vis");
    m.removeAttribute("data-layoza-tg");
  }
})()`;
// Decodes the text-shown and text-hidden captures and returns per-candidate pixel statistics
const TG_STATS_SCRIPT = `async (p) => {
  const load = (src) => new Promise((res, rej) => { const im = new Image(); im.onload = () => res(im); im.onerror = () => rej(new Error("screenshot decode failed")); im.src = src; });
  const px = (im) => { const c = document.createElement("canvas"); c.width = im.naturalWidth; c.height = im.naturalHeight; const x = c.getContext("2d", { willReadFrequently: true }); x.drawImage(im, 0, 0); return x.getImageData(0, 0, c.width, c.height); };
  const A = px(await load(p.shown)), B = px(await load(p.hidden));
  const ch = (v) => { v /= 255; return v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4); };
  const lum = (r, g, b) => 0.2126 * ch(r) + 0.7152 * ch(g) + 0.0722 * ch(b);
  const lstar = (y) => (y > 216 / 24389 ? 116 * Math.cbrt(y) - 16 : (24389 / 27) * y);
  const s = B.width / p.clip.width, Wd = B.width, Hd = Math.min(A.height, B.height);
  return p.candidates.map((c) => {
    const x0 = Math.max(0, Math.round((c.l - p.clip.x) * s)), x1 = Math.min(Wd, Math.round((c.r - p.clip.x) * s));
    const y0 = Math.max(0, Math.round((c.t - p.clip.y) * s)), y1 = Math.min(Hd, Math.round((c.b - p.clip.y) * s));
    const step = Math.max(1, Math.floor(Math.sqrt(((x1 - x0) * (y1 - y0)) / 6000)));
    const [tr, tg, tb, ta] = c.color;
    const ratios = [];
    let n = 0, sum = 0, sum2 = 0, changed = 0, total = 0;
    for (let y = y0; y < y1; y++) {
      for (let x = x0; x < x1; x++) {
        const i = (y * Wd + x) * 4;
        total++;
        if (Math.abs(A.data[i] - B.data[i]) + Math.abs(A.data[i + 1] - B.data[i + 1]) + Math.abs(A.data[i + 2] - B.data[i + 2]) > 60) changed++;
        if ((y - y0) % step || (x - x0) % step) continue;
        const r = B.data[i], g = B.data[i + 1], b = B.data[i + 2];
        const lb = lum(r, g, b);
        // Semi-transparent text colour composites over the pixel behind it
        const lt = lum(tr * ta + r * (1 - ta), tg * ta + g * (1 - ta), tb * ta + b * (1 - ta));
        ratios.push((Math.max(lt, lb) + 0.05) / (Math.min(lt, lb) + 0.05));
        const L = lstar(lb);
        n++; sum += L; sum2 += L * L;
      }
    }
    if (!n) return null;
    ratios.sort((a, b) => a - b);
    const mean = sum / n;
    return {
      p10: Number(ratios[Math.floor(ratios.length * 0.1)].toFixed(2)),
      median: Number(ratios[Math.floor(ratios.length * 0.5)].toFixed(2)),
      lstd: Number(Math.sqrt(Math.max(0, sum2 / n - mean * mean)).toFixed(1)),
      changed: Number((changed / Math.max(1, total)).toFixed(4)),
    };
  });
}`;

const DOM_EVAL_SCRIPT = `(() => {
  function parseColor(str) {
    const m = str && str.match(/rgba?\\(([\\d.]+),\\s*([\\d.]+),\\s*([\\d.]+)(?:,\\s*([\\d.]+))?/i);
    if (!m) return null;
    return [Number(m[1]), Number(m[2]), Number(m[3]), m[4] === undefined ? 1 : Number(m[4])];
  }
  function channelLum(c) {
    const v = c / 255;
    return v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
  }
  function relLum(rgb) {
    return 0.2126 * channelLum(rgb[0]) + 0.7152 * channelLum(rgb[1]) + 0.0722 * channelLum(rgb[2]);
  }
  function contrast(a, b) {
    const l1 = relLum(a);
    const l2 = relLum(b);
    return (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05);
  }
  // Source-over compositing of two RGBA colours (straight alpha)
  function over(t, b) {
    const a = t[3] + b[3] * (1 - t[3]);
    if (a <= 0) return [0, 0, 0, 0];
    const mix = (i) => (t[i] * t[3] + b[i] * b[3] * (1 - t[3])) / a;
    return [mix(0), mix(1), mix(2), a];
  }
  const CLEAR = [0, 0, 0, 0];
  const WHITE = [255, 255, 255, 1];
  // The page canvas behind every layer: Chromium paints it dark only for a dark-only color-scheme
  const CANVAS = (() => {
    const sch = (getComputedStyle(document.documentElement).colorScheme || "").toLowerCase();
    return sch.includes("dark") && !sch.includes("light") ? [18, 18, 18, 1] : WHITE;
  })();
  // Why a text backdrop could not be measured: { reason: count }. Unmeasured is never a pass or a fail.
  const unmeasurable = {};
  let unmeasuredReason = null;
  const why = (k) => { unmeasurable[k] = (unmeasurable[k] || 0) + 1; };
  // Colour stops of a gradient-only background-image ([] = none, null = an image or a colour space we cannot read)
  function gradientStops(bi) {
    if (!bi || bi === "none") return [];
    const low = bi.toLowerCase();
    if (["url(", "image-set(", "element(", "cross-fade(", "paint(", "oklch(", "oklab(", "lab(", "lch(", "hwb(", "hsl(", "hsla(", "color("].some((t) => low.includes(t))) return null;
    const out = [];
    for (let i = low.indexOf("rgb"); i >= 0 && out.length < 8; i = low.indexOf("rgb", i + 3)) {
      const c = parseColor(low.slice(i, low.indexOf(")", i) + 1));
      if (c) out.push(c);
    }
    return out.length ? out : null;
  }
  function withAlpha(c, k) {
    return [c[0], c[1], c[2], c[3] * k];
  }
  function overlaps(a, b) {
    return a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top;
  }
  // A sibling layered under the text: image/video/canvas/gradient, or an absolutely/fixed positioned
  // box that paints a background (scrims, overlays, colour panels)
  function layerBeneath(cur, textBox) {
    const parent = cur.parentElement;
    if (!parent) return false;
    for (const sib of Array.from(parent.children)) {
      if (sib === cur) continue;
      const scs = getComputedStyle(sib);
      const media = /^(IMG|PICTURE|VIDEO|CANVAS|SVG|IFRAME)$/i.test(sib.tagName) || (scs.backgroundImage && scs.backgroundImage !== "none") || sib.querySelector("img, picture, video, canvas");
      const sc = parseColor(scs.backgroundColor);
      const panel = (scs.position === "absolute" || scs.position === "fixed") && sc && sc[3] > 0;
      if ((media || panel) && overlaps(sib.getBoundingClientRect(), textBox)) return true;
    }
    return false;
  }
  // Positioned ::before/::after boxes on node that paint a background over most of the host
  // (thin underline bars and collapsed hover fills are ignored). Topmost (::after) first.
  function coveringPseudos(node) {
    const out = [];
    const host = node.getBoundingClientRect();
    for (const p of ["::after", "::before"]) {
      const ps = getComputedStyle(node, p);
      if (!ps.content || ps.content === "none" || ps.content === "normal") continue;
      if (ps.position !== "absolute" && ps.position !== "fixed") continue;
      if (ps.display === "none" || ps.visibility === "hidden" || parseFloat(ps.opacity) <= 0.01) continue;
      if (ps.transform && ps.transform.startsWith("matrix(0,")) continue;
      const w = parseFloat(ps.width), h = parseFloat(ps.height);
      if (!(w >= host.width * 0.5 && h >= host.height * 0.5)) continue;
      const c = parseColor(ps.backgroundColor);
      const image = ps.backgroundImage && ps.backgroundImage !== "none";
      if (image || (c && c[3] > 0)) out.push({ image, color: c ? withAlpha(c, parseFloat(ps.opacity)) : CLEAR });
    }
    return out;
  }
  // Opacity below 1 that is mid-animation (script-driven inline opacity or a running animation or
  // transition) is a reveal state, reported by text-mid-reveal, not the resting contrast
  function transientOpacity(node, cs) {
    if (!(parseFloat(cs.opacity) < 1)) return false;
    if (node.style && node.style.opacity !== "") return true;
    return Boolean(node.getAnimations && node.getAnimations().length);
  }
  // Product of opacity on the element and every ancestor; null when any of it is transient
  function opacityChain(el) {
    let o = 1;
    for (let c = el; c && c.nodeType === 1; c = c.parentElement) {
      const cs = getComputedStyle(c);
      if (transientOpacity(c, cs)) return null;
      o *= parseFloat(cs.opacity);
    }
    return o;
  }
  // Composites the text colour and its backdrop through the ancestor chain, honouring each
  // ancestor's background and opacity group. Returns { fg, bg } as opaque pixels, "uncertain" when
  // a pseudo-element, gradient/image, or positioned overlay may sit under the text (sample instead),
  // or null when the backdrop cannot be measured at all.
  function chainColors(el, fg) {
    const textBox = el.getBoundingClientRect();
    let txt = fg;
    let bgc = CLEAR;
    for (let cur = el; cur && cur.nodeType === 1; cur = cur.parentElement) {
      const cs = getComputedStyle(cur);
      if (cs.mixBlendMode !== "normal" || transientOpacity(cur, cs)) return null;
      if (layerBeneath(cur, textBox) || coveringPseudos(cur).length || (cs.backgroundImage && cs.backgroundImage !== "none")) return "uncertain";
      const b = parseColor(cs.backgroundColor) || CLEAR;
      txt = over(txt, b);
      bgc = over(bgc, b);
      const o = parseFloat(cs.opacity);
      txt = withAlpha(txt, o);
      bgc = withAlpha(bgc, o);
    }
    return { fg: over(txt, CANVAS), bg: over(bgc, CANVAS) };
  }
  // Reads the paint stack under the text's centre with elementsFromPoint (the caller makes every
  // element hit-testable and scrolls the point into view) and composites the backgrounds beneath
  // the text down to the first opaque layer. Returns null when an image, video, canvas, gradient
  // or blend mode sits in that stack before an opaque layer (unmeasured, not failed).
  function sampledColors(el, fg, x, y) {
    const op = opacityChain(el);
    if (op === null) return null;
    const stack = document.elementsFromPoint(x, y);
    // The point must hit the text's own element (clip-path, masks or 3D transforms can move the hit)
    const start = stack.indexOf(el);
    if (start < 0) { unmeasuredReason = "text box covered or moved (clip/mask/transform)"; return null; }
    // One running backdrop per gradient stop combination: gradients are read stop by stop, never guessed
    let accs = [CLEAR];
    let viaGradient = false;
    const opaque = () => accs.every((a) => a[3] >= 0.999);
    for (const n of stack.slice(start)) {
      if (n !== el && el.contains(n)) continue;
      const cs = getComputedStyle(n);
      if (cs.mixBlendMode !== "normal") { unmeasuredReason = "blend mode"; return null; }
      for (const ps of coveringPseudos(n)) {
        if (ps.image) { unmeasuredReason = "image"; return null; }
        accs = accs.map((a) => over(a, ps.color));
      }
      if (opaque()) break;
      if (/^(IMG|PICTURE|VIDEO|CANVAS|SVG|IFRAME|EMBED|OBJECT)$/i.test(n.tagName)) { unmeasuredReason = "image/media"; return null; }
      const k = n === el ? 1 : parseFloat(cs.opacity);
      const stops = gradientStops(cs.backgroundImage);
      if (stops === null) { unmeasuredReason = "image"; return null; }
      const c = parseColor(cs.backgroundColor);
      const base = c && c[3] > 0 ? withAlpha(c, k) : CLEAR;
      if (stops.length) {
        viaGradient = true;
        const next = [];
        for (const a of accs) for (const s of stops) next.push(over(a, over(withAlpha(s, k), base)));
        accs = next.slice(0, 64);
      } else if (base[3] > 0) accs = accs.map((a) => over(a, base));
      if (opaque()) break;
    }
    const pairs = accs.map((a) => {
      const bg = over(a, CANVAS);
      const f = over(withAlpha(fg, op), bg);
      return { fg: f, bg, r: contrast(f, bg) };
    }).sort((p, q) => p.r - q.r);
    const worst = pairs[0], best = pairs[pairs.length - 1];
    return { fg: worst.fg, bg: worst.bg, gradient: viaGradient, best: viaGradient ? best : null };
  }
  // Nearest ancestor below <html>/<body> that clips horizontal overflow (carousels, scrollers, masks)
  function clipAncestor(el) {
    for (let cur = el.parentElement; cur && cur !== document.body && cur !== document.documentElement; cur = cur.parentElement) {
      const ox = getComputedStyle(cur).overflowX;
      if (ox !== "visible") return cur;
    }
    return null;
  }
  // Rendered and perceivable: not display/visibility/opacity hidden, not a screen-reader-only 1px box,
  // and not scrolled/clipped fully out of a clipping ancestor.
  function isVisible(el) {
    const r = el.getBoundingClientRect();
    if (r.width <= 2 || r.height <= 2) return false;
    // Content of a closed <details> keeps its layout box but is not rendered; only checkVisibility sees that.
    if (typeof el.checkVisibility === "function" && !el.checkVisibility()) return false;
    let cur = el;
    while (cur && cur.nodeType === 1) {
      const cs = getComputedStyle(cur);
      if (cs.display === "none" || cs.visibility === "hidden" || parseFloat(cs.opacity) === 0) return false;
      if (cur !== el && cur !== document.body && cur !== document.documentElement && (cs.overflowX !== "visible" || cs.overflowY !== "visible")) {
        const a = cur.getBoundingClientRect();
        if (r.right <= a.left || r.left >= a.right || r.bottom <= a.top || r.top >= a.bottom) return false;
      }
      cur = cur.parentElement;
    }
    return true;
  }
  function textRect(node) {
    const r = document.createRange();
    r.selectNodeContents(node);
    const b = r.getBoundingClientRect();
    return b.width > 0 && b.height > 0 ? b : node.getBoundingClientRect();
  }
  function lineCount(el) {
    const tops = [];
    const walker = document.createTreeWalker(el, NodeFilter.SHOW_TEXT, null);
    const range = document.createRange();
    let n;
    while ((n = walker.nextNode())) {
      if (!n.nodeValue || !n.nodeValue.trim()) continue;
      const host = n.parentElement && n.parentElement.getBoundingClientRect();
      if (host && (host.width <= 2 || host.height <= 2)) continue; // visually-hidden helper text
      range.selectNodeContents(n);
      for (const r of Array.from(range.getClientRects())) {
        if (r.width > 1 && r.height > 1 && !tops.some((t) => Math.abs(t - r.top) < 6)) tops.push(r.top);
      }
    }
    return tops.length;
  }
  function describe(el) {
    const label = (el.textContent || "").trim().replace(/\\s+/g, " ").slice(0, 40);
    const id = el.id ? "#" + el.id : "";
    return "<" + el.tagName.toLowerCase() + id + "> \\"" + label + "\\"";
  }

  const vw = window.innerWidth;
  const issues = [];
  const issue = (rule, severity, detail) => issues.push({ rule, severity, detail });

  // --- Geometry ---------------------------------------------------------------------------
  const docEl = document.documentElement;
  let horizontalOverflowPx = Math.max(0, docEl.scrollWidth - vw);
  if (horizontalOverflowPx > 0.5) {
    issue("document-overflow", "error", "Document scrollWidth " + docEl.scrollWidth + "px exceeds viewport " + vw + "px");
  }

  let clippedTextNodes = 0;
  for (const el of Array.from(document.querySelectorAll("h1, h2, h3, h4, h5, h6, p, li, a, button, label, dt, dd, figcaption, blockquote, th, td, summary, legend, [role='columnheader'], [role='gridcell'], [role='tab']"))) {
    if (!isVisible(el)) continue;
    const cs = getComputedStyle(el);
    const rect = el.getBoundingClientRect();
    if (!(el.textContent || "").trim()) continue;
    if (el.closest("[aria-hidden='true']")) continue;
    // Text cut by a clipping parent (e.g. a narrow rota/table header cell with overflow:hidden showing "TUE 13 O")
    const par = el.parentElement;
    if (par && par !== document.body) {
      const pcs = getComputedStyle(par);
      if ((pcs.overflowX === "hidden" || pcs.overflowX === "clip") && pcs.textOverflow !== "ellipsis" && cs.textOverflow !== "ellipsis" && !clipAncestor(el)) {
        const pr = par.getBoundingClientRect();
        const cut = Math.max(rect.right - pr.right, pr.left - rect.left);
        if (cut > 3 && rect.width > 8) {
          clippedTextNodes++;
          issue("text-clip-or-bleed", "error", describe(el) + " is cut off by its parent by " + Math.round(cut) + "px at " + vw + "px wide");
          continue;
        }
      }
    }
    // Entirely off-screen (e.g. a skip link parked at left:-9999px until focused) is hidden, not bleeding
    const offR = el.getBoundingClientRect();
    if (offR.right <= 0 || offR.left >= vw) continue;
    // Ellipsis truncation is an intentional design choice, not a defect
    const clipsX = cs.overflowX !== "visible" && cs.textOverflow !== "ellipsis";
    const internalOverflow = el.scrollWidth - el.clientWidth;
    // Content past the viewport edge inside a clipping carousel/scroller is intentional;
    // content that only <html>/<body> overflow-x:hidden is masking is a real cut-off.
    const bleedAllowed = Boolean(clipAncestor(el));
    // Measure bleed on the text itself: a padded row with a negative margin (hover band) may poke past the edge while its words do not
    let tr = rect;
    try { const rg = document.createRange(); rg.selectNodeContents(el); const b = rg.getBoundingClientRect(); if (b.width > 0) tr = b; } catch (e) {}
    // Both the box and its text must cross the edge (screen-reader-only text widens the text range)
    const rightBleed = bleedAllowed ? 0 : Math.min(rect.right, tr.right) - vw;
    const leftBleed = bleedAllowed ? 0 : -Math.max(rect.left, tr.left);
    if ((clipsX && internalOverflow > 3) || rightBleed > 2 || leftBleed > 2) {
      clippedTextNodes++;
      horizontalOverflowPx = Math.max(horizontalOverflowPx, Number(Math.max(rightBleed, leftBleed, 0).toFixed(1)));
      issue("text-clip-or-bleed", "error", describe(el) + " scrollW=" + el.scrollWidth + " clientW=" + el.clientWidth + " left=" + Math.round(rect.left) + " right=" + Math.round(rect.right) + "/" + vw);
    }
  }

  // Short single-line UI labels (header links/buttons, badges) wrapping onto 2+ lines
  const labelToken = /(^|[-_])(badge|pill|tag|chip)s?([-_]|$)/i;
  const labelLike = Array.from(document.querySelectorAll("header a, header button, nav a, button, [class]")).filter(
    (el) =>
      el.matches("header a, header button, nav a, button") ||
      // Class-named badges/tags must be leaf labels, not containers holding prices + buttons
      (Array.from(el.classList).some((c) => labelToken.test(c)) && !el.querySelector("a, button, input, select, textarea"))
  );
  for (const el of labelLike) {
    if (!isVisible(el) || /^H[1-6]$/.test(el.tagName)) continue;
    // Brand wordmarks/logo lockups may stack deliberately (e.g. a bilingual name)
    const brandish = el.matches("header a") && (el === el.closest("header").querySelector("a[href]") || /logo|brand|wordmark/i.test(el.className + " " + (el.getAttribute("aria-label") || "")));
    if (brandish) continue;
    // Composite controls (e.g. a calendar tile: date / weekday / slot) stack their parts on purpose
    const composite = el.querySelector("br") || Array.from(el.children).some((c) => /^(block|flex|grid|list-item)$/.test(getComputedStyle(c).display));
    if (composite) continue;
    const text = (el.textContent || "").trim();
    // Short UI labels only: full sentences (FAQ questions, accordion triggers) may wrap legitimately
    if (!text || text.length > 30 || el.hasAttribute("aria-expanded") || el.querySelector("p, div, ul, h1, h2, h3")) continue;
    const lines = lineCount(el);
    if (lines > 1) issue("label-wrap", "warning", describe(el) + " wraps onto " + lines + " lines");
  }

  // Runaway <canvas> sizing (e.g. height:100% inside an auto-height track scaled by DPR)
  for (const canvas of Array.from(document.querySelectorAll("canvas"))) {
    if (!isVisible(canvas)) continue;
    const h = canvas.getBoundingClientRect().height;
    if (h > Math.max(1800, window.innerHeight * 2.2)) {
      issue("runaway-canvas", "error", "<canvas" + (canvas.id ? " #" + canvas.id : "") + "> is " + Math.round(h) + "px tall (> 2.2x viewport)");
    }
  }

  // Side-by-side heading crowding: a heading and a sibling column on the same row with < 16px text gap
  for (const hd of Array.from(document.querySelectorAll("h1, h2, h3"))) {
    if (!isVisible(hd)) continue;
    let col = hd;
    while (col.parentElement && col.parentElement !== document.body) {
      const pd = getComputedStyle(col.parentElement).display;
      if (pd.includes("grid") || pd.includes("flex")) break;
      col = col.parentElement;
    }
    const row = col.parentElement;
    if (!row || row === document.body) continue;
    const pd = getComputedStyle(row).display;
    if (!(pd.includes("grid") || pd.includes("flex"))) continue;
    const a = textRect(hd);
    for (const sib of Array.from(row.children)) {
      if (sib === col || sib.contains(hd) || !isVisible(sib) || !(sib.textContent || "").trim()) continue;
      const b = textRect(sib);
      const sameRow = Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top) > 6;
      if (!sameRow) continue;
      const gap = b.left >= a.left ? b.left - a.right : a.left - b.right;
      if (gap >= -4 && gap < 16) {
        issue("heading-crowding", "warning", describe(hd) + " sits " + Math.round(gap) + "px from sibling column " + describe(sib) + " (min 16px)");
      }
    }
  }

  // Cramped leading on multi-line display headings
  for (const hd of Array.from(document.querySelectorAll("h1, h2"))) {
    if (!isVisible(hd)) continue;
    const cs = getComputedStyle(hd);
    const fsPx = parseFloat(cs.fontSize) || 32;
    const lh = parseFloat(cs.lineHeight) || fsPx * 1.2;
    const lines = lineCount(hd);
    if (lines >= 3 && lh / fsPx < 0.98) {
      issue("cramped-leading", "warning", describe(hd) + " has " + lines + " lines at line-height " + (lh / fsPx).toFixed(2));
    }
  }

  // Hidden desktop nav without a visible menu toggle
  const headerEl = document.querySelector("header, [role='banner']");
  let menuToggleVisible = false;
  if (headerEl && isVisible(headerEl)) {
    const navLinks = Array.from(headerEl.querySelectorAll("nav a[href]"));
    const anyNavVisible = navLinks.some(isVisible);
    const toggles = Array.from(headerEl.querySelectorAll(
      "button[aria-expanded], button[aria-controls], button[aria-haspopup], button[aria-label*='menu' i], button[aria-label*='navigation' i], .mobile-menu-toggle, [data-mobile-menu-toggle], details:has(nav a[href]) > summary"
    ));
    // Prefer a control named like a menu (text or aria-label) over other popup buttons such as a cart
    const named = (b) => /menu|navigation|\bnav\b/i.test((b.getAttribute("aria-label") || "") + " " + (b.textContent || ""));
    const visibleToggles = toggles.filter(isVisible);
    // A menu-named control wins; an unnamed popup button only counts when it is the sole way to reach hidden nav
    const chosen = visibleToggles.find(named) || (!anyNavVisible ? visibleToggles.find((b) => b.hasAttribute("aria-expanded") || b.tagName === "SUMMARY") : null);
    document.querySelectorAll("[data-layoza-menu-toggle]").forEach((b) => b.removeAttribute("data-layoza-menu-toggle"));
    if (chosen) chosen.setAttribute("data-layoza-menu-toggle", "");
    menuToggleVisible = Boolean(chosen);
    if (navLinks.length > 0 && !anyNavVisible && !menuToggleVisible) {
      issue("nav-unreachable", "error", "Header <nav> links are hidden at " + vw + "px with no visible menu toggle");
    }
  }

  // Text still invisible after a full scroll pass (reveal animations that never ran)
  let hiddenTextAfterScroll = 0;
  for (const el of Array.from(document.querySelectorAll("h1, h2, h3, p, li, a, button"))) {
    if (!(el.textContent || "").trim() || el.closest("[aria-hidden='true'], [hidden], dialog:not([open]), details:not([open])")) continue;
    const r = el.getBoundingClientRect();
    if (r.width <= 2 || r.height <= 2) continue;
    let cur = el, faded = false;
    while (cur && cur.nodeType === 1) {
      const cs = getComputedStyle(cur);
      if (cs.display === "none" || cs.visibility === "hidden") { faded = false; break; }
      if (parseFloat(cs.opacity) === 0) faded = true;
      cur = cur.parentElement;
    }
    if (faded) hiddenTextAfterScroll++;
  }
  if (hiddenTextAfterScroll > 0) {
    issue("text-hidden-after-scroll", "error", hiddenTextAfterScroll + " text element(s) remain at opacity 0 after scrolling the whole page");
  }
  const midReveal = window.__layozaMidReveal || [];
  if (midReveal.length > 0) {
    issue("text-mid-reveal", "warning", midReveal.length + " heading(s) still faded about 0.9s after scrolling stopped: " + midReveal.slice(0, 3).join(", ") + "; finish reveals within ~600ms of entering view, or scrub them so every resting position is fully readable");
  }
  const textOverlap = window.__layozaTextOverlap || [];
  if (textOverlap.length > 0) {
    issue("text-overlap", "error", textOverlap.length + " pair(s) of visible text overlap at a resting scroll position: " + textOverlap.slice(0, 3).join("; ") + "; give each text its own space at every breakpoint and scroll position (move, resize or sequence it), or mark a purely decorative layer aria-hidden");
  }
  const barBleed = window.__layozaBarBleed || [];
  if (barBleed.length > 0) {
    issue("bar-bleed", "warning", barBleed.length + " text run(s) read through a fixed/sticky bar while scrolling: " + barBleed.slice(0, 3).join("; ") + "; make the bar background opaque (alpha >= 0.97) or add a strong backdrop blur once content passes beneath it");
  }
  const floatCover = window.__layozaFloatCover || [];
  if (floatCover.length > 0) {
    issue("floating-control-covers-text", "warning", floatCover.length + " floating control(s) sit on top of body text while scrolling: " + floatCover.slice(0, 3).join("; ") + "; dock the control in a solid header or bar, keep a matching gutter clear in the text column, or move it to a corner the text never reaches");
  }
  const sparseFrames = window.__layozaSparse || [];
  if (sparseFrames.length > 0) {
    issue("sparse-frame", "warning", sparseFrames.length + " resting scroll frame(s) are nearly empty (share of the view holding text or media): " + sparseFrames.slice(0, 4).join("; ") + "; every resting frame of a pinned or scroll-linked section must be a full composition: scale the stage content up, add the next beat sooner, or shorten the pin");
  }
  const headingBreak = window.__layozaHeadingBreak || [];
  if (headingBreak.length > 0) {
    issue("heading-break", "error", headingBreak.length + " display heading(s) do not fit: " + headingBreak.slice(0, 3).join("; ") + "; size display type with clamp() against the narrowest viewport so every word fits on one line, uncut");
  }

  // --- Interactive controls ---------------------------------------------------------------
  let undersizedTargets = 0;
  const smallTargets = [];
  let minTargetPx = null;
  let unlabeledControls = 0;
  const rects = [];
  for (const el of Array.from(document.querySelectorAll("a[href], button, input:not([type=hidden]), select, textarea, [role='button']"))) {
    if (!isVisible(el)) continue;
    const r = el.getBoundingClientRect();
    // WCAG 2.5.8 exempts links inline within a sentence
    const inlineInText = el.tagName === "A" && getComputedStyle(el).display === "inline" && el.closest("p, li, td, dd, blockquote");
    if (!inlineInText) {
      const side = Math.min(r.width, r.height);
      minTargetPx = minTargetPx === null ? side : Math.min(minTargetPx, side);
      if (side < 24) smallTargets.push({ el, r });
    }
    const text = (el.textContent || "").trim();
    const hasLabel = (el.labels && el.labels.length > 0) || el.getAttribute("aria-label") || el.getAttribute("aria-labelledby") || el.getAttribute("title") || el.querySelector("img[alt]:not([alt='']), svg[aria-label], [role='img'][aria-label], svg > title") || (el.tagName === "INPUT" && (el.type === "submit" || el.type === "button") && el.value);
    if (!text && !hasLabel) {
      unlabeledControls++;
      issue("unlabeled-control", "error", "<" + el.tagName.toLowerCase() + "> has no accessible name");
    }
    // Inert subtrees take no input; an open dialog or popover is an overlay above the page by design
    if (!el.closest("[inert]")) rects.push({ el, r, overlay: el.closest("dialog, [popover]") });
  }
  // WCAG 2.5.8 spacing exception: an undersized target passes only when a 24px-diameter circle
  // centred on its bounding box intersects neither another target's box nor another undersized
  // target's circle (centres at least 24px apart).
  const centre = (r) => [r.left + r.width / 2, r.top + r.height / 2];
  const smallSet = new Set(smallTargets.map((t) => t.el));
  for (const t of smallTargets) {
    const [cx, cy] = centre(t.r);
    const crowded = rects.some((o) => {
      if (o.el === t.el || o.el.contains(t.el) || t.el.contains(o.el)) return false;
      const dx = Math.max(o.r.left - cx, 0, cx - o.r.right);
      const dy = Math.max(o.r.top - cy, 0, cy - o.r.bottom);
      if (Math.hypot(dx, dy) < 12) return true;
      if (!smallSet.has(o.el)) return false;
      const [ox, oy] = centre(o.r);
      return Math.hypot(cx - ox, cy - oy) < 24;
    });
    if (crowded) {
      undersizedTargets++;
      if (undersizedTargets <= 10) {
        issue("target-size", "error", describe(t.el) + " is " + Math.round(t.r.width) + "x" + Math.round(t.r.height) + "px and its 24px spacing circle reaches a neighbouring target");
      }
    }
  }
  let overlappingInteractiveElements = 0;
  for (let i = 0; i < rects.length; i++) {
    for (let j = i + 1; j < rects.length; j++) {
      const a = rects[i], b = rects[j];
      if (a.el.contains(b.el) || b.el.contains(a.el) || a.overlay !== b.overlay) continue;
      const inside = (x, y) => x.left >= y.left - 1 && x.right <= y.right + 1 && x.top >= y.top - 1 && x.bottom <= y.bottom + 1;
      if (inside(a.r, b.r) || inside(b.r, a.r)) continue;
      // Compare per-line fragments so multi-line inline links in running text don't count as overlapping
      const hit = (p, q) =>
        Math.min(p.right, q.right) - Math.max(p.left, q.left) > 3 && Math.min(p.bottom, q.bottom) - Math.max(p.top, q.top) > 3;
      if (!hit(a.r, b.r)) continue;
      const fa = Array.from(a.el.getClientRects());
      const fb = Array.from(b.el.getClientRects());
      if (fa.some((p) => fb.some((q) => hit(p, q)))) {
        overlappingInteractiveElements++;
        issue("interactive-overlap", "error", describe(a.el) + " overlaps " + describe(b.el));
      }
    }
  }

  // --- Images -----------------------------------------------------------------------------
  const imgs = Array.from(document.querySelectorAll("img"));
  const brokenImages = imgs.filter((img) => img.complete && img.naturalWidth === 0).map((img) => img.getAttribute("src") || "<anonymous-img>");
  const missingAltCount = imgs.filter((img) => !img.hasAttribute("alt")).length;

  // --- Contrast (per visible text-bearing element) ----------------------------------------
  let minContrast = null;
  let contrastFailures = 0;
  let decorativeContrastFailures = 0;
  const decorativeSamples = [];
  let contrastChecked = 0;
  let contrastIndeterminate = 0;
  const contrastSamples = [];
  const requiredRatio = (cs) => {
    const sizePx = parseFloat(cs.fontSize) || 16;
    const bold = parseInt(cs.fontWeight, 10) >= 700;
    return sizePx >= 24 || (bold && sizePx >= 18.66) ? 3 : 4.5;
  };
  const scoreContrast = (el, cs, px, decorative) => {
    const ratio = contrast(px.fg, px.bg);
    const required = requiredRatio(cs);
    contrastChecked++;
    minContrast = minContrast === null ? ratio : Math.min(minContrast, ratio);
    if (ratio < required && decorative) {
      decorativeContrastFailures++;
      if (decorativeSamples.length < 5) decorativeSamples.push(describe(el) + " " + ratio.toFixed(2) + ":1 (aria-hidden)");
    } else if (ratio < required) {
      contrastFailures++;
      if (contrastSamples.length < 8) contrastSamples.push(describe(el) + " " + ratio.toFixed(2) + ":1 (needs " + required + ":1)");
    }
  };
  // Text whose backdrop the style chain cannot settle; sampled with elementsFromPoint below
  const contrastDeferred = [];
  for (const el of Array.from(document.body.querySelectorAll("*"))) {
    const hasOwnText = Array.from(el.childNodes).some((n) => n.nodeType === 3 && n.nodeValue.trim().length > 0);
    if (!hasOwnText || ["SCRIPT", "STYLE", "NOSCRIPT", "TITLE"].includes(el.tagName) || !isVisible(el)) continue;
    const cs = getComputedStyle(el);
    const fg = parseColor(cs.color);
    // SVG text paints with fill over arbitrary shapes, so CSS-color contrast is not meaningful there
    // aria-hidden text is still seen: report it separately as decorative (WCAG 1.4.3 incidental-text exception may apply)
    const decorative = Boolean(el.closest("[aria-hidden='true']"));
    const px = !fg || el.closest("svg") ? null : chainColors(el, fg);
    if (px === "uncertain" && contrastDeferred.length < 300) {
      contrastDeferred.push({ el, cs, fg, decorative });
      continue;
    }
    if (!px || px === "uncertain") {
      contrastIndeterminate++;
      why(!fg ? "unreadable colour" : el.closest("svg") ? "svg text" : "blend mode or running animation");
      continue;
    }
    scoreContrast(el, cs, px, decorative);
  }
  if (contrastDeferred.length) {
    // Make pointer-events:none overlays hit-testable while sampling, then restore scroll and styles
    const probe = document.createElement("style");
    probe.textContent = "*, *::before, *::after { pointer-events: auto !important; }";
    document.head.appendChild(probe);
    const sx = window.scrollX, sy = window.scrollY;
    try {
      for (const d of contrastDeferred) {
        const node = Array.from(d.el.childNodes).find((n) => n.nodeType === 3 && n.nodeValue.trim().length > 0);
        // First line box of the element's own text run (the element box when the run has no layout box)
        const textBox = () => {
          const range = document.createRange();
          range.selectNodeContents(node);
          const b = Array.from(range.getClientRects()).find((q) => q.width > 0 && q.height > 0);
          return b || d.el.getBoundingClientRect();
        };
        let r = textBox();
        let y = r.top + r.height / 2;
        if (y < 0 || y >= window.innerHeight) {
          window.scrollTo({ left: sx, top: window.scrollY + y - window.innerHeight / 2, behavior: "instant" });
          r = textBox();
          y = r.top + r.height / 2;
        }
        const x = r.left + r.width / 2;
        unmeasuredReason = "off screen";
        const px = x >= 0 && x < window.innerWidth && y >= 0 && y < window.innerHeight ? sampledColors(d.el, d.fg, x, y) : null;
        const need = requiredRatio(d.cs);
        // A gradient that passes under some stops and fails under others depends on where the glyphs sit:
        // say so instead of reporting the worst stop as a measured failure
        if (px && px.best && px.best.r >= need && contrast(px.fg, px.bg) < need) {
          contrastIndeterminate++;
          why("gradient (passes on some stops, fails on others)");
        } else if (px) scoreContrast(d.el, d.cs, px, d.decorative);
        else {
          contrastIndeterminate++;
          why(unmeasuredReason || "unknown");
        }
      }
    } finally {
      window.scrollTo({ left: sx, top: sy, behavior: "instant" });
      probe.remove();
    }
  }

  // --- Headings, landmarks, typography ----------------------------------------------------
  const headingLevels = Array.from(document.querySelectorAll("h1, h2, h3, h4, h5, h6")).filter(isVisible).map((h) => Number(h.tagName[1]));
  const headingProblems = [];
  if (!headingLevels.includes(1)) headingProblems.push("no visible <h1>");
  if (headingLevels.filter((l) => l === 1).length > 1) headingProblems.push("multiple <h1>");
  for (let i = 1; i < headingLevels.length; i++) {
    if (headingLevels[i] > headingLevels[i - 1] + 1) headingProblems.push("h" + headingLevels[i - 1] + " → h" + headingLevels[i] + " skips a level");
  }

  const h1 = Array.from(document.querySelectorAll("h1")).find(isVisible);
  const p = Array.from(document.querySelectorAll("p")).find(isVisible);
  const h1ToBodyRatio = h1 && p ? Number((parseFloat(getComputedStyle(h1).fontSize) / Math.max(1, parseFloat(getComputedStyle(p).fontSize))).toFixed(2)) : null;

  let maxLineLengthCh = null;
  for (const para of Array.from(document.querySelectorAll("p"))) {
    if (!isVisible(para) || (para.textContent || "").trim().length < 120) continue;
    const probe = document.createElement("span");
    probe.textContent = "0";
    probe.style.cssText = "position:absolute;visibility:hidden;white-space:nowrap";
    para.appendChild(probe);
    const chPx = probe.getBoundingClientRect().width || 8;
    probe.remove();
    const cs = getComputedStyle(para);
    const contentW = para.clientWidth - parseFloat(cs.paddingLeft) - parseFloat(cs.paddingRight);
    const ch = Math.round(contentW / chPx);
    maxLineLengthCh = maxLineLengthCh === null ? ch : Math.max(maxLineLengthCh, ch);
  }

  const fontSet = new Set();
  for (const el of Array.from(document.body.querySelectorAll("h1, h2, h3, p, li, a, button, code, pre, span, label"))) {
    if (!isVisible(el)) continue;
    const f = (getComputedStyle(el).fontFamily || "").split(",")[0].replace(/['"]/g, "").trim().toLowerCase();
    if (f) fontSet.add(f);
  }

  let hasReducedMotionRule = false;
  let hasFocusVisibleRule = false;
  let unreadableStylesheets = 0;
  for (const sheet of Array.from(document.styleSheets)) {
    try {
      for (const rule of Array.from(sheet.cssRules || [])) {
        const t = rule.cssText || "";
        if (t.includes("prefers-reduced-motion")) hasReducedMotionRule = true;
        if (t.includes(":focus-visible")) hasFocusVisibleRule = true;
      }
    } catch (e) {
      unreadableStylesheets++;
    }
  }

  // --- Primary action in the first viewport: a landing page's hero should show its main action
  // (buy, book, sign up) without scrolling, at every width. Measured at scroll position 0.
  const heroH1 = Array.from(document.querySelectorAll("h1")).find(isVisible);
  if (heroH1) {
    let hero = heroH1.closest("section, header, [class*='hero']") || heroH1.parentElement;
    if (hero && hero.matches("header") && hero.querySelector("nav") && !hero.contains(heroH1.parentElement)) hero = heroH1.parentElement;
    const actions = Array.from((hero || document.body).querySelectorAll("a[href], button")).filter((el) => {
      if (!isVisible(el) || el.closest("nav, [role='navigation']")) return false;
      const r = el.getBoundingClientRect();
      const top = r.top + window.scrollY;
      return top >= 0 && top + r.height <= window.innerHeight;
    });
    // A hero that has an action, but only below the fold, is a defect. A hero with no action at all
    // (dashboard overview, editorial page) is only a prompt to consider one.
    const heroHasAction = Array.from((hero || document.body).querySelectorAll("a[href], button")).some((el) => isVisible(el) && !el.closest("nav, [role='navigation']"));
    if (!actions.length) {
      issue("primary-action-below-fold", heroHasAction ? "error" : "warning", "No hero call-to-action is fully visible in the first " + window.innerHeight + "px at " + vw + "px wide (put the main action before hero art on narrow screens)");
    }
  }

  // --- Empty panel: a large tinted/bordered box that carries almost no text or media reads as unfinished
  // (an empty booking aside, a newsletter band that is mostly void). Background images/gradients count as content.
  const vArea = vw * window.innerHeight;
  const panelBodyBg = getComputedStyle(document.body).backgroundColor;
  let emptyPanels = 0;
  for (const el of Array.from(document.querySelectorAll("section, aside, article, div, form, footer"))) {
    if (emptyPanels >= 6 || !isVisible(el)) continue;
    const r = el.getBoundingClientRect();
    if (r.width < 200 || r.height < 160 || r.width * r.height < vArea * 0.06) continue;
    // A panel cut sideways by the viewport or a clipping ancestor is a carousel slide scrolled part-way out:
    // its content is hidden by the scroll position, so only the sliver in view would be measured. Skip it.
    let clipL = 0, clipR = window.innerWidth;
    for (let a = el.parentElement; a && a !== document.body; a = a.parentElement) {
      if (getComputedStyle(a).overflowX === "visible") continue;
      const ar = a.getBoundingClientRect();
      clipL = Math.max(clipL, ar.left);
      clipR = Math.min(clipR, ar.right);
    }
    if (Math.max(clipL, r.left) > r.left + 2 || Math.min(clipR, r.right) < r.right - 2) continue;
    const cs = getComputedStyle(el);
    if (cs.backgroundImage !== "none") continue;
    // Navigation sidebars are short lists in a tall rail by design
    if (el.matches("nav, [role='navigation']") || el.querySelector(":scope > nav, :scope > [role='navigation']")) continue;
    const bgParts = cs.backgroundColor.replace("rgba(", "").replace("rgb(", "").replace(")", "").split(",");
    const bgAlpha = cs.backgroundColor === "transparent" ? 0 : bgParts.length === 4 ? parseFloat(bgParts[3]) : 1;
    const tinted = bgAlpha > 0.5 && cs.backgroundColor !== panelBodyBg;
    const bordered = parseFloat(cs.borderTopWidth) > 0 && parseFloat(cs.borderLeftWidth) > 0;
    if (!tinted && !bordered) continue;
    if (el.matches(".pin-spacer, .scrollmagic-pin-spacer")) continue;
    let filled = 0;
    const box = r;
    const spans = [];
    // Scroll-pinning room is scroll distance, not empty layout: a GSAP/ScrollMagic pin spacer pads itself by
    // the pinned scroll length, and a tall parent of a sticky child exists to give the child room to stick.
    // Those stretches are covered for the void-run check and removed from the area the share is measured on.
    let pinRoom = 0;
    for (const sp of Array.from(el.querySelectorAll(".pin-spacer, .scrollmagic-pin-spacer"))) {
      const outer = sp.parentElement && sp.parentElement.closest(".pin-spacer, .scrollmagic-pin-spacer");
      if (outer && el.contains(outer)) continue; // count nested spacers once, via the outermost
      const sr = sp.getBoundingClientRect();
      const scs = getComputedStyle(sp);
      spans.push([Math.max(sr.top, box.top), Math.min(sr.bottom, box.bottom)]);
      pinRoom += (parseFloat(scs.paddingTop) + parseFloat(scs.paddingBottom)) * Math.min(sr.width, box.width);
    }
    for (const st of Array.from(el.querySelectorAll("*")).slice(0, 300)) {
      if (getComputedStyle(st).position !== "sticky" || !st.parentElement || !el.contains(st.parentElement)) continue;
      const pr = st.parentElement.getBoundingClientRect();
      const sh = st.getBoundingClientRect().height;
      if (pr.height < sh * 1.5) continue;
      spans.push([Math.max(pr.top, box.top), Math.min(pr.bottom, box.bottom)]);
      pinRoom += (pr.height - sh) * Math.min(pr.width, box.width);
    }
    const addRect = (q) => {
      const w = Math.min(q.right, box.right) - Math.max(q.left, box.left);
      const h = Math.min(q.bottom, box.bottom) - Math.max(q.top, box.top);
      if (w > 0 && h > 0) {
        filled += w * h;
        spans.push([Math.max(q.top, box.top), Math.min(q.bottom, box.bottom)]);
      }
    };
    const walker = document.createTreeWalker(el, NodeFilter.SHOW_TEXT);
    let n;
    let guard = 0;
    while ((n = walker.nextNode()) && guard++ < 400) {
      if (!n.textContent.trim() || !n.parentElement || !isVisible(n.parentElement)) continue;
      const rg = document.createRange();
      rg.selectNodeContents(n);
      for (const q of Array.from(rg.getClientRects())) addRect(q);
    }
    for (const m of Array.from(el.querySelectorAll("canvas, video, iframe, img, picture, svg, input, select, textarea, button, table"))) {
      if (m.parentElement && m.parentElement.closest("svg")) continue;
      if (isVisible(m)) addRect(m.getBoundingClientRect());
    }
    // Descendants that paint something count as filled: background art/gradients, and drawn marks (a tinted
    // fill or a visible border) such as a CSS wireframe built from <i>/<span> shapes. A painted box covering
    // most of the panel is an inner wrapper, not a mark, so it does not count.
    const alphaOf = (c) => {
      if (!c || c === "transparent") return 0;
      const parts = c.replace(/rgba?\(|\)/g, "").split(",");
      return parts.length === 4 ? parseFloat(parts[3]) : 1;
    };
    for (const c of Array.from(el.querySelectorAll("*")).slice(0, 300)) {
      const ccs = getComputedStyle(c);
      let painted = ccs.backgroundImage !== "none";
      if (!painted) {
        const cr = c.getBoundingClientRect();
        if (cr.width * cr.height > r.width * r.height * 0.5) continue;
        const fill = alphaOf(ccs.backgroundColor) > 0.5 && ccs.backgroundColor !== cs.backgroundColor;
        const edge = ["Top", "Right", "Bottom", "Left"].some((s) => parseFloat(ccs["border" + s + "Width"]) > 0 && ccs["border" + s + "Style"] !== "none" && ccs["border" + s + "Style"] !== "hidden" && alphaOf(ccs["border" + s + "Color"]) > 0.3);
        painted = fill || edge;
      }
      if (painted && isVisible(c)) addRect(c.getBoundingClientRect());
    }
    const share = filled / Math.max(1, r.width * r.height - pinRoom);
    // Largest run of the panel's height with nothing in it (content bunched at the top of a tall box)
    spans.sort((a, b) => a[0] - b[0]);
    let cursor = r.top;
    let gap = 0;
    for (const s of spans) {
      gap = Math.max(gap, s[0] - cursor);
      cursor = Math.max(cursor, s[1]);
    }
    gap = Math.max(gap, r.bottom - cursor);
    const voidRun = gap > 320 && gap > r.height * 0.45;
    if (share < 0.05 || voidRun) {
      emptyPanels++;
      issue("empty-panel", "warning", describe(el) + " is " + Math.round(r.width) + "x" + Math.round(r.height) + "px; " + (voidRun ? Math.round(gap) + "px of it is empty in one run" : "only " + Math.round(share * 100) + "% holds text or media") + " at " + vw + "px wide (fill it with real content or let it shrink to fit)");
    }
  }

  // --- Ragged control row: a row of pills/chips/filters that wraps into a centred or right-aligned second
  // line, or leaves one orphan control alone on its last line, reads as unplanned. Scroll it or let it shrink.
  let raggedRows = 0;
  for (const row of Array.from(document.querySelectorAll("div, ul, ol, nav, fieldset, form, p"))) {
    if (raggedRows >= 4 || !isVisible(row)) continue;
    const rcs = getComputedStyle(row);
    if (!rcs.display.includes("flex") || rcs.flexWrap !== "wrap" || rcs.flexDirection.startsWith("column")) continue;
    // Absolutely/fixed positioned children are out of flow (orbit, scatter, stacked layers): they never wrap,
    // so a row whose controls are placed that way is not a wrapping row.
    const visKids = Array.from(row.children).filter(isVisible);
    if (visKids.some((k) => ["absolute", "fixed"].includes(getComputedStyle(k).position))) continue;
    const kids = visKids;
    if (kids.length < 3 || kids.length > 16) continue;
    const isChip = (k) => {
      const kc = getComputedStyle(k);
      const tinted = kc.backgroundColor !== "transparent" && !kc.backgroundColor.endsWith(", 0)");
      return (tinted || parseFloat(kc.borderTopWidth) > 0) && k.textContent.trim().length > 0 && k.textContent.trim().length < 48;
    };
    const isControl = (k) => k.matches("a, button, label, input, select") || (k.children.length <= 2 && k.querySelector("a, button, input, label")) || isChip(k);
    if (!kids.every(isControl)) continue;
    const rects = kids.map((k) => k.getBoundingClientRect());
    if (rects.some((q) => q.height > 64 || q.width > 360)) continue;
    const tops = [];
    for (const q of rects) if (!tops.some((t) => Math.abs(t - q.top) < 6)) tops.push(q.top);
    if (tops.length < 2) continue;
    tops.sort((a, b) => a - b);
    const lastCount = rects.filter((q) => Math.abs(q.top - tops[tops.length - 1]) < 6).length;
    const firstCount = rects.filter((q) => Math.abs(q.top - tops[0]) < 6).length;
    const jc = rcs.justifyContent;
    const offStart = !["normal", "flex-start", "start", "left", "stretch"].includes(jc);
    const lefts = tops.map((t) => Math.min(...rects.filter((q) => Math.abs(q.top - t) < 6).map((q) => q.left)));
    const raggedEdge = Math.max(...lefts) - Math.min(...lefts) > 8;
    if ((offStart && (lastCount < firstCount || raggedEdge)) || (lastCount === 1 && kids.length >= 4)) {
      raggedRows++;
      issue("ragged-control-row", "warning", describe(row) + " wraps " + kids.length + " controls into " + tops.length + " rows (" + lastCount + " on the last" + (offStart ? ", justify " + jc : "") + ") at " + vw + "px wide (scroll the row, shrink the controls, or group them so every line is full)");
    }
  }

  // --- Ledger-heavy / visual-starved page (desktop). A section is the largest box around one h2. Sections
  // built as stacked rows (lists, tables, row-cards) with no dominant visual read as a spreadsheet; a page
  // where most sections carry no visual moment (art, 3D, scene, media, type as image) reads as a document.
  if (vw >= 1024) {
    const sections = [];
    for (const h of Array.from(document.querySelectorAll("main h2, body > section h2, body > div h2"))) {
      if (!isVisible(h) || h.closest("header, footer, nav, dialog, [aria-hidden='true']")) continue;
      let s = h;
      while (s.parentElement && s.parentElement !== document.body && s.parentElement.querySelectorAll("h2").length === 1) s = s.parentElement;
      if (!sections.includes(s)) sections.push(s);
    }
    const ledgerSecs = [];
    const starved = [];
    const cardSigs = new Map();
    for (const s of sections.slice(0, 40)) {
      const sr = s.getBoundingClientRect();
      const sArea = Math.max(1, sr.width * sr.height);
      let rows = s.querySelectorAll("tr").length;
      let listArea = 0;
      for (const t of Array.from(s.querySelectorAll("table"))) { const tr = t.getBoundingClientRect(); listArea += tr.width * tr.height; }
      for (const c of Array.from(s.querySelectorAll("ul, ol, dl, div")).slice(0, 400)) {
        const kids = Array.from(c.children).filter(isVisible);
        if (kids.length < 4) continue;
        const rr = kids.map((k) => k.getBoundingClientRect());
        let stacked = 0;
        for (let i = 1; i < rr.length; i++) if (rr[i].top >= rr[i - 1].bottom - 4 && Math.abs(rr[i].left - rr[i - 1].left) < 8) stacked++;
        const w0 = rr[0].width;
        if (stacked < kids.length - 2 || w0 < 200 || rr.some((q) => Math.abs(q.width - w0) > w0 * 0.1 || q.height > 220)) continue;
        rows += kids.length;
        const cr = c.getBoundingClientRect();
        listArea += cr.width * cr.height;
      }
      let visual = false;
      for (const m of Array.from(s.querySelectorAll("canvas, video, img, picture, svg"))) {
        if (m.parentElement && m.parentElement.closest("svg")) continue;
        const mr = m.getBoundingClientRect();
        if (isVisible(m) && (mr.width * mr.height >= sArea * 0.25 || (mr.width >= vw * 0.45 && mr.height >= 280))) { visual = true; break; }
      }
      if (!visual) {
        for (const h of Array.from(s.querySelectorAll("h1, h2, p, span, div, strong, b")).slice(0, 300)) {
          if (h.children.length > 3 || !h.textContent.trim()) continue;
          if (parseFloat(getComputedStyle(h).fontSize) >= 120 && isVisible(h)) { visual = true; break; }
        }
      }
      if (!visual) {
        const bi = getComputedStyle(s).backgroundImage;
        if (bi.includes("url(")) visual = true;
      }
      const label = describe(s);
      if (rows >= 4 && listArea >= sArea * 0.25 && !visual) ledgerSecs.push(label + " (" + rows + " rows)");
      if (!visual) starved.push(label);
      const seen = new Set();
      for (const b of Array.from(s.querySelectorAll("*")).slice(0, 600)) {
        const br = b.getBoundingClientRect();
        if (br.width < 240 || br.height < 140 || br.width > sr.width * 0.9 || !isVisible(b)) continue;
        const bc = getComputedStyle(b);
        const bordered = parseFloat(bc.borderTopWidth) > 0 && bc.borderTopStyle !== "none";
        const tinted = bc.backgroundColor !== "transparent" && !bc.backgroundColor.endsWith(", 0)");
        if (!bordered && !tinted) continue;
        const sig = [bc.borderTopWidth, bc.borderTopColor, bc.backgroundColor, bc.borderTopLeftRadius, bc.boxShadow].join("|");
        if (!seen.has(sig)) { seen.add(sig); cardSigs.set(sig, (cardSigs.get(sig) || 0) + 1); }
      }
    }
    const n = sections.length;
    if (n >= 3 && ledgerSecs.length >= 2 && ledgerSecs.length / n >= 0.3) {
      issue("ledger-heavy-page", "warning", ledgerSecs.length + " of " + n + " sections are stacked rows or tables with no dominant visual: " + ledgerSecs.slice(0, 4).join("; ") + " at " + vw + "px wide (keep data to the 1-2 sections where the visitor needs it; give the rest one visual moment: art, 3D, a scene or type as image)");
    }
    if (n >= 4 && starved.length / n >= 0.6) {
      issue("visual-starved-page", "warning", starved.length + " of " + n + " sections have no visual moment (no art, media, 3D or type-as-image covering a quarter of the section), e.g. " + starved.slice(0, 3).join("; ") + " at " + vw + "px wide (each section needs one dominant visual drawn from its own content)");
    }
    const topSig = Array.from(cardSigs.values()).reduce((a, b) => Math.max(a, b), 0);
    if (n >= 4 && topSig >= 4 && topSig / n >= 0.5) {
      issue("repeated-card-look", "warning", "One card treatment (same border, fill, radius and shadow) is used in " + topSig + " of " + n + " sections at " + vw + "px wide (give each section its own material: open ground, full-bleed colour, inset, art plate; boxes are not a layout)");
    }
  }

  // --- Hero empty band (phones): a tall stretch of the first screen with no text, image or photo reads
  // as a void in the most-seen frame. Gradients alone do not count as content.
  if (vw < 600 && window.scrollY < 4) {
    const vh0 = window.innerHeight;
    const spans = [];
    const add = (q) => {
      const t = Math.max(0, q.top);
      const b = Math.min(vh0, q.bottom);
      if (b - t > 2 && q.width > 8) spans.push([t, b]);
    };
    const tw = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    let tn;
    let tg = 0;
    while ((tn = tw.nextNode()) && tg++ < 1500) {
      if (!tn.textContent.trim() || !tn.parentElement || !isVisible(tn.parentElement)) continue;
      const rg = document.createRange();
      rg.selectNodeContents(tn);
      for (const q of Array.from(rg.getClientRects())) if (q.top < vh0) add(q);
    }
    for (const m of Array.from(document.querySelectorAll("canvas, video, iframe, img, picture, svg, input, select, textarea, button"))) {
      if (m.parentElement && m.parentElement.closest("svg")) continue;
      const q = m.getBoundingClientRect();
      if (!(q.top < vh0 && q.width > 24 && q.height > 24 && isVisible(m))) continue;
      if (m.tagName.toLowerCase() !== "svg") {
        add(q);
        continue;
      }
      // An illustration's box is not its art: count drawn shapes, skip full-size sky/backdrop rects and specks
      const svgArea = q.width * q.height;
      let shapes = 0;
      for (const sh of Array.from(m.querySelectorAll("path, rect, circle, ellipse, polygon, polyline, line, text, image, use")).slice(0, 600)) {
        const sq = sh.getBoundingClientRect();
        if (sq.width * sq.height > svgArea * 0.6 || sq.width * sq.height < 400) continue;
        add(sq);
        shapes++;
      }
      if (!shapes) add(q);
    }
    for (const c of Array.from(document.body.querySelectorAll("*")).slice(0, 1500)) {
      const bi = getComputedStyle(c).backgroundImage;
      if (bi.includes("url(")) {
        const q = c.getBoundingClientRect();
        if (q.top < vh0) add(q);
      }
    }
    spans.sort((a, b) => a[0] - b[0]);
    let cur = 0;
    let gapTop = 0;
    let band = 0;
    for (const s of spans) {
      if (s[0] - cur > band) {
        band = s[0] - cur;
        gapTop = cur;
      }
      cur = Math.max(cur, s[1]);
    }
    if (vh0 - cur > band && cur < document.documentElement.scrollHeight - vh0) {
      band = vh0 - cur;
      gapTop = cur;
    }
    if (band > vh0 * 0.15 && band > 110) {
      issue("hero-empty-band", "warning", Math.round(band) + "px of the first " + vh0 + "px screen (from y " + Math.round(gapTop) + ") holds no text, image or art at " + vw + "px wide (bring the hero art up, crop it into the band, or tighten the spacing)");
    }
  }

  // --- WCAG 2.4.11 Focus Not Obscured: focus each control (browser scrolls it into view like Tab does)
  // and check it is not hidden under a sticky/fixed header. Skip links are designed to overlay the header.
  let focusObscured = 0;
  const bars = Array.from(document.querySelectorAll("header, [role='banner'], body > nav, body > div")).filter((h) => {
    const cs = getComputedStyle(h);
    const r = h.getBoundingClientRect();
    return (cs.position === "sticky" || cs.position === "fixed") && r.top <= 1 && r.height > 0 && r.height < window.innerHeight * 0.4;
  });
  if (bars.length) {
    const startY = window.scrollY;
    const focusables = Array.from(document.querySelectorAll("a[href], button, input:not([type=hidden]), select, textarea, [tabindex]:not([tabindex='-1'])")).slice(0, 200);
    // Forward (Tab) then backward (Shift+Tab): moving up is when elements land under a top bar
    const seenObscured = new Set();
    for (const el of [...focusables, ...focusables.slice().reverse()]) {
      if (seenObscured.has(el) || bars.some((b) => b.contains(el)) || !isVisible(el)) continue;
      if (el.tagName === "A" && /skip/i.test(el.textContent || "") && (el.getAttribute("href") || "").startsWith("#")) continue;
      // Keyboard focus scrolls to the nearest edge (honouring scroll-padding); el.focus() alone would centre it
      el.scrollIntoView({ block: "nearest", inline: "nearest", behavior: "instant" });
      el.focus({ preventScroll: true });
      if (document.activeElement !== el) continue;
      const r = el.getBoundingClientRect();
      const covered = bars.some((b) => {
        const br = b.getBoundingClientRect();
        return r.top < br.bottom - 2 && r.bottom > br.top && r.left < br.right && r.right > br.left;
      });
      if (covered) {
        seenObscured.add(el);
        focusObscured++;
        if (focusObscured <= 5) issue("focus-obscured", "error", describe(el) + " is hidden under the sticky header when focused (add scroll-padding-top)");
      }
    }
    if (document.activeElement && document.activeElement.blur) document.activeElement.blur();
    window.scrollTo({ top: startY, behavior: "instant" });
  }

  // --- Live design tokens (Site DNA from the rendered page) --------------------------------
  const tally = (map, key, w) => { if (key) map.set(key, (map.get(key) || 0) + (w || 1)); };
  const top = (map, n) => Array.from(map.entries()).sort((a, b) => b[1] - a[1]).slice(0, n).map(([value, count]) => ({ value, count }));
  const toHex = (c) => "#" + c.slice(0, 3).map((v) => Math.round(v).toString(16).padStart(2, "0")).join("").toUpperCase();
  const textColors = new Map(), bgColors = new Map(), fontUse = new Map(), radii = new Map(), maxWidths = new Map();
  for (const el of Array.from(document.body.querySelectorAll("*")).slice(0, 4000)) {
    if (!isVisible(el)) continue;
    const cs = getComputedStyle(el);
    const r = el.getBoundingClientRect();
    const area = Math.min(r.width * r.height, vw * window.innerHeight);
    const bgc = parseColor(cs.backgroundColor);
    if (bgc && bgc[3] > 0.5) tally(bgColors, toHex(bgc), area / 1000);
    const hasText = Array.from(el.childNodes).some((n) => n.nodeType === 3 && n.nodeValue.trim());
    if (hasText) {
      const fc = parseColor(cs.color);
      if (fc) tally(textColors, toHex(fc), (el.textContent || "").trim().length);
      tally(fontUse, (cs.fontFamily || "").split(",")[0].replace(/['"]/g, "").trim(), (el.textContent || "").trim().length);
    }
    if (cs.borderTopLeftRadius && cs.borderTopLeftRadius !== "0px") tally(radii, cs.borderTopLeftRadius);
    if (cs.maxWidth && cs.maxWidth !== "none" && cs.maxWidth.endsWith("px")) tally(maxWidths, cs.maxWidth);
  }
  const bodyBg = parseColor(getComputedStyle(document.body).backgroundColor);
  const htmlBg = parseColor(getComputedStyle(document.documentElement).backgroundColor);
  const pageBg = bodyBg && bodyBg[3] > 0 ? bodyBg : htmlBg && htmlBg[3] > 0 ? htmlBg : [255, 255, 255, 1];
  const customProps = {};
  for (const sheet of Array.from(document.styleSheets)) {
    try {
      for (const rule of Array.from(sheet.cssRules || [])) {
        if (rule.selectorText && /(^|,)\\s*(:root|html)\\s*(,|$)/.test(rule.selectorText)) {
          for (const prop of Array.from(rule.style)) {
            if (prop.startsWith("--") && Object.keys(customProps).length < 200) customProps[prop] = rule.style.getPropertyValue(prop).trim();
          }
        }
      }
    } catch (e) {}
  }
  const typeScale = {};
  for (const tag of ["h1", "h2", "h3", "h4", "p", "small"]) {
    const el = Array.from(document.querySelectorAll(tag)).find(isVisible);
    if (el) {
      const cs = getComputedStyle(el);
      typeScale[tag] = { size: cs.fontSize, weight: cs.fontWeight, line_height: cs.lineHeight, letter_spacing: cs.letterSpacing, family: cs.fontFamily.split(",")[0].replace(/['"]/g, "").trim() };
    }
  }
  const html = document.documentElement.outerHTML.slice(0, 400000);
  const frameworkHints = [];
  if (window.__NEXT_DATA__ || document.getElementById("__next")) frameworkHints.push("next.js");
  if (window.__NUXT__ || document.getElementById("__nuxt")) frameworkHints.push("nuxt");
  if (document.querySelector("astro-island")) frameworkHints.push("astro");
  if (document.querySelector("[data-reactroot], #root")) frameworkHints.push("react");
  if (/wp-content|wp-includes/.test(html)) frameworkHints.push("wordpress");
  if (document.querySelector("[data-elementor-type], .elementor")) frameworkHints.push("elementor");
  if (document.querySelector(".brxe-section, [class*='brxe-']")) frameworkHints.push("bricks");
  if (window.Shopify || /cdn\\.shopify\\.com/.test(html)) frameworkHints.push("shopify");
  if (document.querySelector("[class*='svelte-']")) frameworkHints.push("svelte");
  if (window.__VUE__ || document.querySelector("[data-v-app]")) frameworkHints.push("vue");
  const motionLibs = [];
  if (window.gsap) motionLibs.push("GSAP");
  if (window.ScrollTrigger) motionLibs.push("ScrollTrigger");
  if (window.THREE) motionLibs.push("Three.js");
  if (window.Lenis || document.documentElement.classList.contains("lenis")) motionLibs.push("Lenis");
  if (window.anime) motionLibs.push("anime.js");
  if (document.querySelector("[data-framer-name], [data-framer-appear-id]")) motionLibs.push("Framer Motion");
  let keyframeCount = 0;
  for (const sheet of Array.from(document.styleSheets)) {
    try { keyframeCount += Array.from(sheet.cssRules || []).filter((r) => r.type === 7).length; } catch (e) {}
  }
  const siteDna = {
    title: document.title.slice(0, 120),
    lang: document.documentElement.lang || null,
    page_background: toHex(pageBg),
    text_colors: top(textColors, 8),
    background_colors: top(bgColors, 8),
    fonts: top(fontUse, 6),
    type_scale: typeScale,
    radii: top(radii, 6),
    max_widths: top(maxWidths, 6),
    css_custom_properties: customProps,
    framework_hints: frameworkHints,
    motion_libraries: motionLibs,
    css_keyframes: keyframeCount,
  };

  return {
    siteDna,
    horizontalOverflowPx: Number(horizontalOverflowPx.toFixed(1)),
    clippedTextNodes,
    overlappingInteractiveElements,
    brokenImages,
    missingAltCount,
    undersizedTargets,
    minTargetPx: minTargetPx === null ? null : Number(minTargetPx.toFixed(1)),
    unlabeledControls,
    minContrastRatio: minContrast === null ? null : Number(minContrast.toFixed(2)),
    contrastFailures,
    decorativeContrastFailures,
    decorativeSamples,
    contrastChecked,
    contrastIndeterminate,
    contrastUnmeasurable: unmeasurable,
    contrastSamples,
    headingProblems,
    hasMainLandmark: Boolean(document.querySelector("main, [role='main']")),
    h1ToBodyRatio,
    maxLineLengthCh,
    fontFamilies: Array.from(fontSet),
    hasReducedMotionRule,
    hasFocusVisibleRule,
    unreadableStylesheets,
    menuToggleVisible,
    issues,
  };
})()`;

/*
 * ---------------------------------------------------------------------------------------------
 * Design critique ("craft") rules
 *
 * Design critique rules ported from pbakaus/impeccable (Apache-2.0), commit 2f98774
 * (2f98774f86bc05563876643db37e6c37c322850f), crates/core/src/checks/rules.rs,
 * crates/core/src/checks/{text_rules,html_patterns,css_scan,measures}.rs,
 * crates/core/src/browser/{element_checks,page_checks,quality,text_collectors}.rs,
 * crates/foundation/src/{constants,registry,color}.rs and crates/foundation/src/rules/text.rs.
 * Copyright the impeccable authors; licensed under the Apache License, Version 2.0
 * (http://www.apache.org/licenses/LICENSE-2.0). Rule ids, thresholds and gates follow upstream.
 *
 * Modified by Layoza:
 *  - Re-implemented from Rust into one self-contained in-page JavaScript function that runs in real
 *    Chromium against computed styles and live geometry (upstream's file engine reads authored CSS
 *    text and has no layout; its URL engine runs a wasm core). Rules upstream evaluates from CSS
 *    text (marquee, monotonous-spacing, codex-grid-background, repeating-stripes-gradient,
 *    radial-halo, buried-raster wash, organic-clip-path) read computed values or the CSSOM instead.
 *  - Emitted as non-blocking `issues` with rule id `craft-<upstream-id>` and severity "critique";
 *    the runner adds a top-level `craft_critique` {rule: count} summary.
 *  - Upstream's advisory/warning split is not carried: every craft finding is critique-level.
 *  - Some deep corpus exemptions upstream added for live sites (shadow DOM, scripted colour
 *    reveals, pixel-sampled backdrops, transform-scaled boxes, design-system DESIGN.md tokens) are
 *    approximated or omitted; each place is marked "Layoza:" below.
 *  - Brand preservation (not in upstream): rules that ban specific fonts or colours globally can be
 *    opted out of by the page's own brand. Declare it in the page head:
 *
 *      <meta name="layoza-brand-allow" content="font:Inter, palette:cream">
 *
 *    Keys (comma or semicolon separated, case-insensitive):
 *      font:<family>     overused-font (and italic-serif-display) stand down for that family
 *      palette:cream     cream-palette stands down (alias: palette:beige)
 *      palette:purple    ai-color-palette stands down for purple/violet (alias: palette:violet)
 *      palette:cyan      ai-color-palette stands down for cyan
 *      rule:<id>         any craft rule stands down, e.g. rule:em-dash-overuse
 *    Suppressed findings are listed in `craft_critique_suppressed`, never silently dropped.
 *
 * Not ported (and why): low-contrast, skipped-heading, broken-image, script-error, text-overflow,
 * content-hidden-at-rest (Layoza's objective WCAG/geometry checks above already gate on these);
 * text-occlusion, first-viewport-column-overflow, clipped-overflow-container (covered by Layoza's
 * interactive-overlap / text-clip-or-bleed / document-overflow). heading-rhythm is ported in core
 * form (see its block). design-system-font/-color/-radius/-font-size are NOT ported from upstream:
 * Layoza has its own rendered, token-aware implementation in `layozaDesignSystemCheck` below.
 * ---------------------------------------------------------------------------------------------
 */
function layozaCraftCritique() {
  "use strict";
  const D = document;
  const vw = window.innerWidth || 0;
  const vh = window.innerHeight || 0;
  const issues = [];
  const suppressed = [];
  const errors = [];
  if (!D.body) return { issues, suppressed, errors, brand_allow: null };

  // ---- Brand allow-list (Layoza addition) ---------------------------------------------------
  const allow = { fonts: [], palettes: [], rules: [] };
  D.querySelectorAll('meta[name="layoza-brand-allow" i]').forEach((m) => {
    (m.getAttribute("content") || "").split(/[,;]/).forEach((part) => {
      const i = part.indexOf(":");
      if (i < 0) return;
      const k = part.slice(0, i).trim().toLowerCase();
      const v = part.slice(i + 1).trim().toLowerCase().replace(/^['"]|['"]$/g, "");
      if (!v) return;
      if (k === "font") allow.fonts.push(v);
      else if (k === "palette") allow.palettes.push(v === "beige" ? "cream" : v === "violet" ? "purple" : v);
      else if (k === "rule") allow.rules.push(v.replace(/^craft-/, ""));
    });
  });
  const emit = (rule, detail) => {
    const d = String(detail).replace(/\s+/g, " ").slice(0, 300);
    if (allow.rules.includes(rule)) {
      suppressed.push({ rule: "craft-" + rule, allowed_by: "rule:" + rule, detail: d });
      return;
    }
    issues.push({ rule: "craft-" + rule, severity: "critique", detail: d });
  };
  const suppress = (rule, by, detail) => suppressed.push({ rule: "craft-" + rule, allowed_by: by, detail: String(detail).slice(0, 300) });
  const run = (name, fn) => {
    try {
      fn();
    } catch (e) {
      if (errors.length < 20) errors.push(name + ": " + (e && e.message ? e.message : String(e)));
    }
  };

  // ---- Upstream constants (crates/foundation/src/constants.rs, rules/text.rs) ---------------
  const SAFE_TAGS = new Set(["blockquote", "nav", "a", "input", "textarea", "select", "pre", "code", "span", "th", "td", "tr", "li", "label", "button", "hr", "html", "head", "body", "script", "style", "link", "meta", "title", "br", "img", "svg", "path", "circle", "rect", "line", "polyline", "polygon", "g", "defs", "use"]);
  const BORDER_SAFE_TAGS = new Set(Array.from(SAFE_TAGS).filter((t) => t !== "label"));
  const HEADING_TAGS = new Set(["h1", "h2", "h3", "h4", "h5", "h6"]);
  const OVERUSED_FONTS = ["inter", "roboto", "open sans", "lato", "montserrat", "arial", "helvetica", "fraunces", "instrument sans", "instrument serif", "geist", "geist sans", "geist mono", "mona sans", "plus jakarta sans", "space grotesk", "recoleta"];
  const BRAND_FONT_DOMAINS = {
    roboto: ["google.com", "youtube.com", "android.com", "chromium.org", "chrome.com", "web.dev", "gstatic.com", "firebase.google.com"],
    "google sans": ["google.com", "youtube.com", "android.com", "chromium.org", "chrome.com", "web.dev", "gstatic.com", "firebase.google.com"],
    "product sans": ["google.com", "youtube.com", "android.com", "chromium.org", "chrome.com", "web.dev", "gstatic.com", "firebase.google.com"],
    geist: ["vercel.com", "nextjs.org", "v0.app"],
    "geist sans": ["vercel.com", "nextjs.org", "v0.app"],
    "geist mono": ["vercel.com", "nextjs.org", "v0.app"],
    "mona sans": ["github.com", "githubnext.com"],
  };
  const CSS_GENERIC_FONTS = ["serif", "sans-serif", "monospace", "cursive", "fantasy", "inherit", "initial", "unset", "revert"];
  const GENERIC_FONTS = CSS_GENERIC_FONTS.concat(["system-ui", "ui-serif", "ui-sans-serif", "ui-monospace", "ui-rounded", "-apple-system", "blinkmacsystemfont", "segoe ui"]);
  const KNOWN_SERIF_FONTS = ["fraunces", "recoleta", "newsreader", "playfair display", "playfair", "cormorant", "cormorant garamond", "garamond", "eb garamond", "tiempos", "tiempos headline", "tiempos text", "lora", "vollkorn", "spectral", "source serif pro", "source serif 4", "source serif", "ibm plex serif", "merriweather", "libre caslon", "libre baskerville", "baskerville", "georgia", "times new roman", "times", "dm serif display", "dm serif text", "instrument serif", "gt sectra", "ogg", "canela", "freight display", "freight text"];
  const MONOSPACE_FACES = ["andale mono", "cascadia code", "consolas", "courier", "courier new", "fira code", "hack", "inconsolata", "lucida console", "menlo", "monaco", "source code pro"];
  const QUALITY_TEXT_TAGS = new Set(["p", "li", "td", "th", "dd", "blockquote", "figcaption"]);
  const NON_RENDERED_TAGS = new Set(["script", "style", "title", "noscript", "template", "head", "meta", "link", "base", "param", "source", "track", "datalist", "col", "colgroup", "area"]);
  const KICKER_SKIP_SELECTOR = 'nav,form,table,thead,tbody,tfoot,figure,figcaption,ol,ul,li,[role="navigation"],[aria-label*="breadcrumb" i],[class*="breadcrumb" i],[aria-hidden="true"],[data-impeccable-allow-kickers]';
  const KICKER_CARD_CONTEXT_SELECTOR = 'article,button,a,li,[role="listitem"],[role="option"]';
  const APP_CONTEXT = '[role="tabpanel"], [role="dialog"], [role="application"], dialog';
  const NUMBERED_LABEL_TAGS = ["span", "p", "div", "small", "em", "strong", "b"];
  const SR_ONLY_SELECTOR = '.sr-only, .visually-hidden, .visuallyhidden, .screen-reader, .screen-reader-only, .screenreader, .a11y-hidden, .hidden-visually, [class*="sr-only" i], [class*="visually-hidden" i], [class*="visuallyhidden" i], [class*="screen-reader" i], [class*="screenreader" i]';
  const TINY_TEXT_UI_CONTEXT = 'button, a, label, summary, pre, [role="button"], [role="link"], [role="tab"], [role="menuitem"], [role="option"], nav, footer, [aria-hidden="true"], [class*="badge" i], [class*="caption" i], [class*="chip" i], [class*="code" i], [class*="console" i], [class*="diff" i], [class*="label" i], [class*="meta" i], [class*="mock" i], [class*="pill" i], [class*="preview" i], [class*="tag" i], [class*="terminal" i], [class*="writes" i]';
  const EXEMPT_CONTEXT = 'pre, code, kbd, samp, var, svg, [aria-hidden="true"], [class*="terminal" i], [class*="console" i], [class*="code" i], [class*="mock" i], [class*="editor" i], [class*="syntax" i], [class*="diff" i]';
  const INTERACTIVE = 'a[href], button, summary, label, select, textarea, [role="button"], [role="link"], [role="tab"], [role="menuitem"], [role="menuitemcheckbox"], [role="menuitemradio"], [role="option"], [role="checkbox"], [role="radio"], [role="switch"], [role="treeitem"]';
  const FURNITURE = 'nav, [role="navigation"], td, th, [role="gridcell"], [role="cell"], caption, figcaption, dt, dd, footer, [class*="meta" i], [class*="label" i], [class*="badge" i], [class*="chip" i], [class*="pill" i], [class*="tag" i], [class*="kicker" i], [class*="eyebrow" i], [class*="breadcrumb" i], [class*="timestamp" i], [class*="category" i], [class*="caption" i], [class*="nav" i]';
  const SMALLPRINT = 'small, footer, [class*="legal" i], [class*="copyright" i], [class*="fineprint" i], [class*="fine-print" i], [class*="smallprint" i], [class*="small-print" i], [class*="disclaimer" i], [class*="disclosure" i], [class*="footnote" i]';
  const TRACKED_CONTROL_LABEL = 'a[href], button, [role="button"], [role="link"], [role="tab"]';
  const TEXT_EDGE_QUERY = "a, button, code, dd, dt, figcaption, h1, h2, h3, h4, h5, h6, li, p, pre, span, td, th";
  const FLUSH_SKIP_TAGS = new Set(["html", "body", "main", "header", "footer", "nav", "article", "aside", "button", "a", "label", "summary", "code", "pre", "input", "textarea", "select", "form", "figure", "table", "tbody", "thead", "tr", "td", "th"]);
  const LEADING_DISPLAY_TYPE_PX = 24;
  const LEADING_MIN_LINE_BOXES = 1.5;
  const LEADING_HEADING_CONTEXT = 'h1, h2, h3, h4, h5, h6, [role="heading"]';
  const UI_TEXT_FLOOR_PX = 11;
  const SMALLPRINT_TEXT_FLOOR_PX = 10;
  const ALL_CAPS_LONG_RUN = 80;
  const TRACKED_LABEL_MAX_CHARS = 40;
  const LINE_MAX = 80; // BrowserConfig::line_max() default
  const HERO_EYEBROW_TRACKING_PX = 1.6;
  const HERO_EYEBROW_TRACKING_EM = 0.08;
  const FAINT_PAINT_ALPHA = 0.05;
  const CAPTION_MAX_CHARS = 140;
  const EM_DASH_FLOOR = 8;
  const EM_DASH_CHARS_PER_DASH = 500;
  const CONTENT_AREA_EM = 1.2;

  // ---- DOM helpers ---------------------------------------------------------------------------
  const styleCache = new Map();
  const cs = (el) => {
    let s = styleCache.get(el);
    if (!s) {
      s = getComputedStyle(el);
      styleCache.set(el, s);
    }
    return s;
  };
  const rectCache = new Map();
  const rect = (el) => {
    let r = rectCache.get(el);
    if (!r) {
      r = el.getBoundingClientRect();
      rectCache.set(el, r);
    }
    return r;
  };
  const num = (v) => {
    const n = parseFloat(v);
    return Number.isFinite(n) ? n : 0;
  };
  const tagOf = (el) => (el.localName || el.tagName || "").toLowerCase();
  const collapse = (s) => (s || "").replace(/\s+/g, " ").trim();
  const directTextNodes = (el) => Array.from(el.childNodes).filter((n) => n.nodeType === 3).map((n) => n.nodeValue || "");
  const directText = (el) => directTextNodes(el).join("");
  const cleanInlineText = (el) => collapse(directTextNodes(el).join(" "));
  const classSel = (el) => {
    const t = (el.getAttribute("class") || "").trim().split(/\s+/).filter(Boolean);
    return (tagOf(el) || "el") + (t.length ? "." + t.join(".") : "");
  };
  const describe = (el) => {
    const sel = classSel(el).slice(0, 60);
    const text = collapse(el.textContent).slice(0, 40);
    return "<" + sel + ">" + (text ? ' "' + text + '"' : "");
  };
  const fontTokens = (ff) => (ff || "").split(",").map((f) => f.trim().replace(/^['"]|['"]$/g, "").toLowerCase());
  const closest = (el, sel) => {
    try {
      return el.closest(sel);
    } catch (e) {
      return null;
    }
  };
  const matchesOrClosest = (el, sel) => Boolean(closest(el, sel));
  const resolveLen = (value, fontSize) => {
    const v = (value || "").trim().toLowerCase();
    const m = v.match(/^(-?[\d.]+)(px|em|rem|%)?$/);
    if (!m) return null;
    const n = parseFloat(m[1]);
    if (!Number.isFinite(n)) return null;
    if (m[2] === "em") return n * fontSize;
    if (m[2] === "rem") return n * 16;
    if (m[2] === "%") return (n / 100) * fontSize;
    return n;
  };
  const textRectOf = (node) => {
    const range = D.createRange();
    range.selectNodeContents(node);
    const rs = Array.from(range.getClientRects()).filter((r) => r.width > 0.5 && r.height > 0.5);
    if (!rs.length) return null;
    let l = Infinity, t = Infinity, r = -Infinity, b = -Infinity;
    for (const x of rs) {
      l = Math.min(l, x.left);
      t = Math.min(t, x.top);
      r = Math.max(r, x.right);
      b = Math.max(b, x.bottom);
    }
    return { left: l, top: t, right: r, bottom: b, width: r - l, height: b - t };
  };
  const unionRects = (list) => {
    const rs = list.filter(Boolean);
    if (!rs.length) return null;
    let l = Infinity, t = Infinity, r = -Infinity, b = -Infinity;
    for (const x of rs) {
      l = Math.min(l, x.left);
      t = Math.min(t, x.top);
      r = Math.max(r, x.right);
      b = Math.max(b, x.bottom);
    }
    return { left: l, top: t, right: r, bottom: b, width: r - l, height: b - t };
  };
  // Union of the rects of `el`'s own (direct) text nodes
  const directTextRect = (el) =>
    unionRects(Array.from(el.childNodes).filter((n) => n.nodeType === 3 && (n.nodeValue || "").trim()).map(textRectOf));
  // Union of the rects of every text node under `el`
  const allTextRect = (el) => {
    const out = [];
    const walker = D.createTreeWalker(el, NodeFilter.SHOW_TEXT, null);
    let n;
    let guard = 0;
    while ((n = walker.nextNode()) && guard++ < 400) {
      if ((n.nodeValue || "").trim() && n.parentElement && !NON_RENDERED_TAGS.has(tagOf(n.parentElement))) out.push(textRectOf(n));
    }
    return unionRects(out);
  };

  // display / content-visibility / opacity chain; visibility read on the element (it inherits)
  const shownMemo = new Map();
  function shown(el) {
    if (!el || el.nodeType !== 1 || el === D.documentElement) return true;
    if (shownMemo.has(el)) return shownMemo.get(el);
    const s = cs(el);
    const ok = s.display !== "none" && s.contentVisibility !== "hidden";
    const res = ok && !(parseFloat(s.opacity) <= 0.01) && shown(el.parentElement);
    shownMemo.set(el, res);
    return res;
  }
  function visible(el) {
    const s = cs(el);
    if (s.visibility === "hidden" || s.visibility === "collapse") return false;
    if (!shown(el)) return false;
    const r = rect(el);
    return r.width > 0 && r.height > 0;
  }
  // checks.mjs#isRenderedTypeElement / font_text_has_a_box: no hidden attr / display / visibility,
  // and (optionally) no near-zero opacity, on the element or any ancestor
  function hasBox(el, withOpacity) {
    for (let cur = el; cur && cur.nodeType === 1; cur = cur.parentElement) {
      if (cur.hidden) return false;
      const s = cs(cur);
      if (s.display === "none" || s.visibility === "hidden" || s.visibility === "collapse" || s.contentVisibility === "hidden") return false;
      if (withOpacity && parseFloat(s.opacity) <= 0.01) return false;
    }
    return true;
  }
  const effectiveOpacity = (el) => {
    let acc = 1;
    for (let cur = el; cur && cur.nodeType === 1; cur = cur.parentElement) {
      const v = parseFloat(cs(cur).opacity);
      if (Number.isFinite(v)) acc *= Math.min(1, Math.max(0, v));
      if (acc <= 0.02) return 0;
    }
    return acc;
  };
  const ownOpacity = (el) => {
    const v = parseFloat(cs(el).opacity);
    return Number.isFinite(v) ? Math.min(1, Math.max(0, v)) : 1;
  };
  function visuallyHidden(el) {
    if (closest(el, SR_ONLY_SELECTOR)) return true;
    const s = cs(el);
    if (s.position === "absolute" || s.position === "fixed") {
      if (/rect\(\s*0/.test(s.clip || "") || /inset\(\s*(?:50%|99|100%)/.test(s.clipPath || "")) return true;
      const w = parseFloat(s.width), h = parseFloat(s.height);
      if ((w === 1 || h === 1) && (s.overflow === "hidden" || s.overflow === "clip")) return true;
    }
    return false;
  }

  // ---- Colour helpers (crates/foundation/src/color.rs) ---------------------------------------
  const COLOR_TOKEN_RE = /(?:rgba?|hsla?|oklch|oklab|lab|lch|hwb|color-mix|color)\((?:[^()]|\([^()]*\))*\)|#[0-9a-fA-F]{3,8}\b|\btransparent\b/gi;
  const colorCache = new Map();
  let cctx = null;
  function parseColor(str) {
    if (str == null) return null;
    const key = String(str).trim().toLowerCase();
    if (!key || key === "none") return null;
    if (colorCache.has(key)) return colorCache.get(key);
    let out = null;
    if (key === "transparent") out = { r: 0, g: 0, b: 0, a: 0 };
    else {
      const m = key.match(/^rgba?\(\s*(-?[\d.]+)[\s,]+(-?[\d.]+)[\s,]+(-?[\d.]+)\s*(?:[,/]\s*([\d.]+)(%?))?\s*\)$/);
      if (m) {
        let a = m[4] === undefined ? 1 : parseFloat(m[4]);
        if (m[5] === "%") a /= 100;
        out = { r: +m[1], g: +m[2], b: +m[3], a };
      } else if (window.CSS && CSS.supports && CSS.supports("color", key) && !/currentcolor|^(inherit|initial|unset|revert)$/.test(key)) {
        try {
          if (!cctx) {
            const c = D.createElement("canvas");
            c.width = c.height = 1;
            cctx = c.getContext("2d", { willReadFrequently: true });
          }
          cctx.clearRect(0, 0, 1, 1);
          cctx.fillStyle = key;
          cctx.fillRect(0, 0, 1, 1);
          const d = cctx.getImageData(0, 0, 1, 1).data;
          out = { r: d[0], g: d[1], b: d[2], a: d[3] / 255 };
        } catch (e) {
          out = null;
        }
      }
    }
    colorCache.set(key, out);
    return out;
  }
  const colorTokens = (value) => (String(value || "").match(COLOR_TOKEN_RE) || []);
  const gradientColors = (value) => (/gradient\(/i.test(value || "") ? colorTokens(value).map(parseColor).filter(Boolean) : []);
  const spread = (c) => Math.max(c.r, c.g, c.b) - Math.min(c.r, c.g, c.b);
  const hasChroma = (c, t) => Boolean(c) && spread(c) >= (t === undefined ? 30 : t);
  const isTransparent = (str) => {
    const c = parseColor(str);
    return c ? c.a <= 0 : !str || str === "none";
  };
  const chan = (v) => {
    const x = v / 255;
    return x <= 0.04045 ? x / 12.92 : Math.pow((x + 0.055) / 1.055, 2.4);
  };
  const lum = (c) => 0.2126 * chan(c.r) + 0.7152 * chan(c.g) + 0.0722 * chan(c.b);
  const contrastRatio = (a, b) => {
    const l1 = lum(a), l2 = lum(b);
    return (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05);
  };
  const over = (top, bottom) => {
    const a = top.a === undefined ? 1 : top.a;
    return { r: top.r * a + bottom.r * (1 - a), g: top.g * a + bottom.g * (1 - a), b: top.b * a + bottom.b * (1 - a), a: 1 };
  };
  const hueOf = (c) => {
    const r = c.r / 255, g = c.g / 255, b = c.b / 255;
    const max = Math.max(r, g, b), min = Math.min(r, g, b);
    if (max === min) return 0;
    const d = max - min;
    let h;
    if (max === r) h = ((g - b) / d + (g < b ? 6 : 0)) / 6;
    else if (max === g) h = ((b - r) / d + 2) / 6;
    else h = ((r - g) / d + 4) / 6;
    return Math.round(h * 360);
  };
  const lightSat = (c) => {
    const max = Math.max(c.r, c.g, c.b) / 255, min = Math.min(c.r, c.g, c.b) / 255;
    const l = (max + min) / 2, d = max - min;
    const s = d <= 0 || l <= 0 || l >= 1 ? 0 : d / (1 - Math.abs(2 * l - 1));
    return [l, s];
  };
  const isGrayInk = (c) => {
    const [l, s] = lightSat(c);
    return s < 0.2 && l > 0.3 && l < 0.85;
  };
  const hex = (c) => "#" + [c.r, c.g, c.b].map((v) => Math.round(v).toString(16).padStart(2, "0")).join("");
  const isNeutralColor = (str) => {
    if (!str || str === "transparent") return true;
    const c = parseColor(str);
    return c ? spread(c) < 30 : false;
  };
  const isAccentColor = (str) => {
    const c = parseColor(str);
    return c ? spread(c) >= 40 : false;
  };
  const colorsNearlyMatch = (a, b) => {
    const ca = parseColor(a), cb = parseColor(b);
    if (!ca || !cb) return false;
    return Math.abs(ca.a - cb.a) <= 0.03 && Math.max(Math.abs(ca.r - cb.r), Math.abs(ca.g - cb.g), Math.abs(ca.b - cb.b)) <= 3;
  };
  const canvasDark = () => /dark/i.test(cs(D.documentElement).colorScheme || "");
  const CANVAS = () => (canvasDark() ? { r: 18, g: 18, b: 18, a: 1 } : { r: 255, g: 255, b: 255, a: 1 });

  // The surface under `el`: translucent fills composited to the first opaque one. A url() raster
  // makes it unresolved; a gradient stops the walk and is averaged by the caller (gradient_ancestor_average).
  function bgInfo(el) {
    const layers = [];
    for (let cur = el; cur && cur.nodeType === 1; cur = cur.parentElement) {
      const s = cs(cur);
      const img = s.backgroundImage;
      if (img && img !== "none") {
        if (/url\(/i.test(img)) return { color: null, unresolved: true };
        if (/gradient\(/i.test(img)) return { color: null, unresolved: false, gradient: cur };
      }
      const c = parseColor(s.backgroundColor);
      if (c && c.a > 0) {
        layers.push(c);
        if (c.a >= 1) break;
      }
    }
    let base = CANVAS();
    for (let i = layers.length - 1; i >= 0; i--) base = over(layers[i], base);
    return { color: base, unresolved: false };
  }
  function gradientAverage(start) {
    for (let cur = start; cur && cur.nodeType === 1; cur = cur.parentElement) {
      const g = gradientColors(cs(cur).backgroundImage);
      if (g.length) {
        const n = g.length;
        return { r: Math.round(g.reduce((s, c) => s + c.r, 0) / n), g: Math.round(g.reduce((s, c) => s + c.g, 0) / n), b: Math.round(g.reduce((s, c) => s + c.b, 0) / n), a: 1 };
      }
    }
    return null;
  }
  // quality.rs#has_visible_background_boundary
  function hasVisibleBgBoundary(el) {
    const bg = cs(el).backgroundColor;
    if (isTransparent(bg)) return false;
    for (let p = el.parentElement; p; p = p.parentElement) {
      const pb = cs(p).backgroundColor;
      if (!isTransparent(pb)) return !colorsNearlyMatch(bg, pb);
    }
    if (canvasDark()) return true;
    return !colorsNearlyMatch(bg, "rgb(255, 255, 255)");
  }
  const rootBackground = () => {
    const b = parseColor(cs(D.body).backgroundColor);
    if (b && b.a > 0) return over(b, CANVAS());
    const h = parseColor(cs(D.documentElement).backgroundColor);
    if (h && h.a > 0) return over(h, CANVAS());
    return CANVAS();
  };
  const darkPage = lum(rootBackground()) < 0.1;

  // ---- Shadow parsing (foundation css/measures.rs#parse_shadow_layers) ------------------------
  function splitTop(value, sep) {
    const out = [];
    let depth = 0, start = 0;
    for (let i = 0; i < value.length; i++) {
      const ch = value[i];
      if (ch === "(") depth++;
      else if (ch === ")") depth--;
      else if (ch === sep && depth === 0) {
        out.push(value.slice(start, i));
        start = i + 1;
      }
    }
    out.push(value.slice(start));
    return out.map((s) => s.trim()).filter(Boolean);
  }
  function shadowLayers(value) {
    if (!value || value === "none") return [];
    const out = [];
    for (const layer of splitTop(value, ",")) {
      const cm = layer.match(new RegExp(COLOR_TOKEN_RE.source, "i"));
      const color = cm ? parseColor(cm[0]) : null;
      const rest = cm ? layer.replace(cm[0], " ") : layer;
      const nums = (rest.match(/-?\d*\.?\d+/g) || []).map(parseFloat);
      if (nums.length < 2) continue;
      out.push({ color, alpha: color ? color.a : 1, x: nums[0], y: nums[1], blur: nums[2] || 0, spread: nums[3] || 0, inset: /\binset\b/i.test(rest), count: nums.length });
    }
    return out;
  }
  const outerReach = (l) => {
    const base = l.spread + l.blur / 2;
    return [base - l.y, base + l.x, base + l.y, base - l.x];
  };

  // ---- Keyframes (CSSOM) ---------------------------------------------------------------------
  const keyframes = new Map(); // name -> { text, frames: [[prop, value], ...] }
  const authoredCss = [];
  (function collectSheets() {
    const walk = (rules) => {
      for (const rule of Array.from(rules || [])) {
        if (rule.type === 7 || rule.constructor.name === "CSSKeyframesRule") {
          const decls = [];
          for (const fr of Array.from(rule.cssRules || [])) {
            for (const prop of Array.from(fr.style || [])) decls.push([prop, fr.style.getPropertyValue(prop)]);
          }
          keyframes.set(rule.name, { text: rule.cssText, decls });
        } else if (rule.cssRules) {
          walk(rule.cssRules);
        }
        if (rule.type === 1) authoredCss.push(rule.cssText);
      }
    };
    for (const sheet of Array.from(D.styleSheets)) {
      try {
        walk(sheet.cssRules);
      } catch (e) {
        /* cross-origin sheet */
      }
    }
  })();
  const animationNames = (el) => (cs(el).animationName || "").split(",").map((s) => s.trim()).filter((n) => n && n !== "none");
  const iterationCounts = (el) => (cs(el).animationIterationCount || "").split(",").map((s) => s.trim());
  const isInfiniteAt = (el, i) => {
    const counts = iterationCounts(el);
    return (counts[i] !== undefined ? counts[i] : counts[counts.length - 1]) === "infinite";
  };
  // css_scan.rs#keyframe_decl_only_pulses / keyframes_only_pulse
  const scaleWithinUnit = (list) => {
    const parts = list.split(/[,\s]+/).filter(Boolean);
    return parts.length > 0 && parts.every((p) => {
      const v = Number(p);
      return Number.isFinite(v) && v >= 0 && v <= 1;
    });
  };
  function keyframesOnlyPulse(name) {
    const k = keyframes.get(name);
    if (!k || !k.decls.length) return null;
    return k.decls.every(([prop, value]) => {
      const p = prop.replace(/^-webkit-/, "");
      const v = value.replace(/!important$/, "").trim();
      if (p === "opacity") return true;
      if (p === "scale") return v === "none" || scaleWithinUnit(v);
      if (p === "transform") {
        if (v === "none") return true;
        const fns = v.match(/scale(?:[xyz]|3d)?\([^()]*\)/gi);
        if (!fns || v.replace(/scale(?:[xyz]|3d)?\([^()]*\)/gi, "").trim() !== "") return false;
        return fns.every((f) => scaleWithinUnit(f.replace(/^[^(]*\(|\)$/g, "")));
      }
      if (p === "animation-timing-function") return !/linear\(|var\(/.test(v) && !(v.match(/cubic-bezier\(([^)]*)\)/g) || []).some((b) => {
        const n = b.replace(/^[^(]*\(|\)$/g, "").split(",").map(parseFloat);
        return !(n[1] >= 0 && n[1] <= 1) || !(n[3] >= 0 && n[3] <= 1);
      });
      return false;
    });
  }

  const all = Array.from(D.body.querySelectorAll("*")).slice(0, 8000);

  // =================================================================================================
  // Page-level typography: overused-font (page_checks.rs#check_typography)
  // =================================================================================================
  run("overused-font", () => {
    const usage = new Map(), chars = new Map();
    let total = 0, totalChars = 0;
    for (const el of D.body.querySelectorAll("p, h1, h2, h3, h4, h5, h6, li, td, th, dd, blockquote, figcaption, a, button, label, span")) {
      const nodes = directTextNodes(el);
      if (!nodes.some((t) => t.trim())) continue;
      const weight = hasBox(el, false) ? nodes.reduce((n, t) => n + collapse(t).length, 0) : 0;
      const ff = cs(el).fontFamily;
      if (!ff) continue;
      const primary = fontTokens(ff).find((f) => f && !CSS_GENERIC_FONTS.includes(f));
      if (!primary) continue;
      usage.set(primary, (usage.get(primary) || 0) + 1);
      total++;
      if (weight > 0) {
        chars.set(primary, (chars.get(primary) || 0) + weight);
        totalChars += weight;
      }
    }
    if (total < 20) return;
    const ranked = Array.from(usage.entries()).sort((a, b) => b[1] - a[1]);
    const [font, count] = ranked[0];
    if (ranked[1] && ranked[1][1] === count) return;
    const host = (location.hostname || "").toLowerCase();
    const brandDomain = (BRAND_FONT_DOMAINS[font] || []).some((d) => host === d || host.endsWith("." + d));
    if (!OVERUSED_FONTS.includes(font) || brandDomain) return;
    if (totalChars > 0 && (chars.get(font) || 0) / totalChars < 0.05) return;
    const detail = "Primary font: " + font + " (" + Math.round((count / total) * 100) + "% of text)";
    if (allow.fonts.includes(font)) suppress("overused-font", "font:" + font, detail);
    else emit("overused-font", detail);
  });

  // flat-type-hierarchy (rules.rs#check_flat_type_hierarchy_samples)
  run("flat-type-hierarchy", () => {
    const samples = [];
    for (const el of D.body.querySelectorAll("h1,h2,h3,h4,h5,h6,p,li,td,th,dd,blockquote,figcaption")) {
      if (!(el.textContent || "").trim() || !hasBox(el, true)) continue;
      const size = parseFloat(cs(el).fontSize);
      if (!Number.isFinite(size) || size < 8 || size >= 200) continue;
      const t = tagOf(el);
      const w = cs(el).fontWeight;
      const weight = w === "normal" ? 400 : w === "bold" ? 700 : parseFloat(w);
      samples.push({ role: HEADING_TAGS.has(t) ? t : "body", size, weight: Number.isFinite(weight) ? weight : NaN });
    }
    const byRole = new Map();
    for (const s of samples) {
      const size = Math.round(s.size * 10) / 10;
      if (!byRole.has(s.role)) byRole.set(s.role, []);
      byRole.get(s.role).push(size);
    }
    const dominant = (role, sizes) => {
      const counts = new Map();
      for (const z of sizes) counts.set(z, (counts.get(z) || 0) + 1);
      const ranked = Array.from(counts.entries()).sort((a, b) => b[1] - a[1] || a[0] - b[0]);
      if (ranked.length > 1 && ranked[0][1] === ranked[1][1]) {
        if (role !== "h1") return null;
        return Math.max(...ranked.filter((r) => r[1] === ranked[0][1]).map((r) => r[0]));
      }
      return ranked.length ? ranked[0][0] : null;
    };
    let settled = 0, dropped = 0;
    const droppedSizes = [];
    const roles = [];
    for (const [role, sizes] of byRole) {
      const d = dominant(role, sizes);
      if (d !== null) {
        if (role !== "body") settled += sizes.length;
        roles.push([role, d]);
      } else if (role !== "body") {
        dropped += sizes.length;
        droppedSizes.push(...sizes);
      }
    }
    if (roles.length < 3) return;
    const sizeOf = (n) => (roles.find((r) => r[0] === n) || [])[1];
    if (sizeOf("h1") !== undefined && sizeOf("body") !== undefined && sizeOf("h1") < sizeOf("body")) return;
    roles.sort((a, b) => a[1] - b[1] || (a[0] < b[0] ? -1 : 1));
    const largestStep = (sizes) => {
      let step = 1;
      for (let i = 1; i < sizes.length; i++) step = Math.max(step, sizes[i] / sizes[i - 1]);
      return step;
    };
    const ladder = roles.map((r) => r[1]);
    const step = largestStep(ladder);
    if (step >= 1.25) return;
    if (dropped > settled && droppedSizes.some((x) => largestStep(ladder.concat([x]).sort((a, b) => a - b)) >= 1.25)) return;
    emit("flat-type-hierarchy", "Role sizes: " + roles.map((r) => r[0] + " " + r[1] + "px").join(", ") + " (largest adjacent step " + step.toFixed(2) + ":1; target 1.25:1)");
  });

  // cream-palette (page_checks.rs#check_cream_palette, measures.rs#is_cream_color)
  run("cream-palette", () => {
    let bg = parseColor(cs(D.body).backgroundColor);
    if (!bg || bg.a === 0) bg = parseColor(cs(D.documentElement).backgroundColor);
    if (!bg) return;
    const { r, g, b } = bg;
    if (Math.min(r, g, b) < 209 || !(r >= g && g >= b)) return;
    const warmth = r - b;
    if (warmth < 6 || warmth > 48) return;
    const detail = "cream/beige page background rgb(" + r + ", " + g + ", " + b + ")";
    if (allow.palettes.includes("cream")) suppress("cream-palette", "palette:cream", detail);
    else emit("cream-palette", detail);
  });

  // =================================================================================================
  // Cards: nested-cards (page_checks.rs#check_layout)
  // =================================================================================================
  const cardMemo = new Map();
  function edgeSides(el, layers) {
    const s = cs(el);
    const sides = ["Top", "Right", "Bottom", "Left"].map((k) => num(s["border" + k + "Width"]) >= 0.5 && s["border" + k + "Style"] !== "none" && s["border" + k + "Style"] !== "hidden" && (parseColor(s["border" + k + "Color"]) || { a: 0 }).a >= FAINT_PAINT_ALPHA);
    for (const l of layers) {
      if (l.inset || l.alpha < FAINT_PAINT_ALPHA) continue;
      outerReach(l).forEach((reach, i) => {
        if (reach >= 1) sides[i] = true;
      });
    }
    return sides.filter(Boolean).length;
  }
  function fillDiffers(el) {
    const s = cs(el);
    if (s.backgroundImage && s.backgroundImage !== "none") return true;
    if (isTransparent(s.backgroundColor)) return false;
    const surface = el.parentElement ? bgInfo(el.parentElement).color : CANVAS();
    if (!surface) return true;
    const fill = over(parseColor(s.backgroundColor), surface);
    return Math.max(Math.abs(fill.r - surface.r), Math.abs(fill.g - surface.g), Math.abs(fill.b - surface.b)) > 3;
  }
  function isCardLike(el) {
    if (cardMemo.has(el)) return cardMemo.get(el);
    let res = false;
    const t = tagOf(el);
    if (!(SAFE_TAGS.has(t) || ["input", "select", "textarea", "img", "video", "canvas", "picture"].includes(t))) {
      const s = cs(el);
      const layers = shadowLayers(s.boxShadow);
      const casts = layers.some((l) => l.alpha >= FAINT_PAINT_ALPHA && (l.x !== 0 || l.y !== 0 || l.blur > 0 || l.spread !== 0));
      const rounded = (s.borderRadius || "").split(/[\s/]+/).some((p) => parseFloat(p) > 0);
      if (casts || rounded) res = edgeSides(el, layers) >= 3 || fillDiffers(el);
    }
    cardMemo.set(el, res);
    return res;
  }
  function showsBorderOrShadow(el) {
    const s = cs(el);
    if (shadowLayers(s.boxShadow).some((l) => l.alpha >= FAINT_PAINT_ALPHA && (l.x !== 0 || l.y !== 0 || l.blur > 0 || l.spread !== 0))) return true;
    return ["Top", "Right", "Bottom", "Left"].some((k) => num(s["border" + k + "Width"]) >= 0.5 && s["border" + k + "Style"] !== "none" && s["border" + k + "Style"] !== "hidden" && (parseColor(s["border" + k + "Color"]) || { a: 0 }).a >= FAINT_PAINT_ALPHA);
  }
  const overlapArea = (a, b) => Math.max(0, Math.min(a.right, b.right) - Math.max(a.left, b.left)) * Math.max(0, Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top));
  run("nested-cards", () => {
    const flagged = [];
    const isDialog = (el) => tagOf(el) === "dialog" || /\b(dialog|alertdialog)\b/i.test(el.getAttribute("role") || "") || (el.getAttribute("aria-modal") || "").trim().toLowerCase() === "true";
    const dialogCard = (outer) => {
      if (isDialog(outer)) return true;
      for (let a = outer.parentElement; a; a = a.parentElement) {
        if (isDialog(a)) return true;
        if (isCardLike(a)) return false;
      }
      return false;
    };
    for (const el of all) {
      if (!isCardLike(el)) continue;
      const s = cs(el);
      if (s.position === "absolute" || s.position === "fixed") continue;
      if (/(?:^|[^A-Za-z0-9])(?:dropdown|popover|tooltip|menu|modal|dialog)(?:[^A-Za-z0-9]|$)/i.test(el.getAttribute("class") || "")) continue;
      if (/\b(menu|listbox|tablist|radiogroup|toolbar)\b/i.test(el.getAttribute("role") || "")) continue;
      if ((el.textContent || "").trim().length < 10) continue;
      const r = rect(el);
      if (r.width < 50 || r.height < 30) continue;
      if (!showsBorderOrShadow(el)) continue;
      // single-line label box
      const radius = parseFloat(s.borderRadius);
      const fs = parseFloat(s.fontSize);
      const lh = s.lineHeight === "normal" ? fs * 1.2 : parseFloat(s.lineHeight);
      if (Number.isFinite(radius) && r.height > 0 && radius >= r.height / 2 - 0.5) continue;
      if (Number.isFinite(lh) && lh > 0 && r.height - (num(s.paddingTop) + num(s.paddingBottom) + num(s.borderTopWidth) + num(s.borderBottomWidth)) <= lh * 1.5) continue;
      const kids = Array.from(el.children);
      if (kids.length && kids.every((c) => ["input", "select", "textarea"].includes(tagOf(c)))) continue;
      if (s.display === "inline" || s.display === "contents") continue;
      const area = r.width * r.height;
      if (Array.from(el.querySelectorAll("img, picture, video, canvas, iframe")).some((m) => overlapArea(rect(m), r) >= area * 0.6)) continue;
      for (let p = el.parentElement; p; p = p.parentElement) {
        if (!isCardLike(p)) continue;
        const pr = rect(p);
        const ps = cs(p);
        const near = (x, y) => Math.abs(x - y) <= 1.5;
        const shares = [near(r.top, pr.top + num(ps.borderTopWidth)), near(r.right, pr.right - num(ps.borderRightWidth)), near(r.bottom, pr.bottom - num(ps.borderBottomWidth)), near(r.left, pr.left + num(ps.borderLeftWidth))].filter(Boolean).length >= 3;
        // Layoza: figure / media-player / output-block exemptions reduced to their main gates
        const total = collapse(el.textContent).replace(/\s/g, "").length;
        const figure = Array.from(el.querySelectorAll("svg, canvas")).some((f) => overlapArea(rect(f), r) >= area * 0.4 && total - collapse(f.textContent).replace(/\s/g, "").length <= CAPTION_MAX_CHARS && !el.querySelector("button, input, select, textarea, a[href]"));
        const player = Boolean(el.querySelector("audio[controls], video[controls]"));
        const pre = Array.from(el.querySelectorAll("pre, code, samp, kbd, output")).some((c) => collapse(c.textContent).replace(/\s/g, "").length >= total * 0.6);
        if (visible(el) && !shares && !dialogCard(p) && !figure && !player && !pre) flagged.push(el);
        break;
      }
    }
    for (const el of flagged) {
      if (!flagged.some((o) => o !== el && el.contains(o))) emit("nested-cards", "Card inside card " + describe(el));
    }
  });

  // =================================================================================================
  // Headings and labels: italic-serif-display, hero-eyebrow-chip, kicker-above-heading,
  // numbered-section-labels, icon-tile-stack, oversized-h1
  // =================================================================================================
  function resolveSerif(ff) {
    const tokens = fontTokens(ff);
    const primary = tokens.find((f) => f && !GENERIC_FONTS.includes(f));
    if (!primary) return { primary: null, serif: false };
    return { primary, serif: KNOWN_SERIF_FONTS.includes(primary) || tokens.includes("serif") };
  }
  run("italic-serif-display", () => {
    for (const h of D.body.querySelectorAll("h1, h2")) {
      const t = tagOf(h);
      const stack = [h];
      let hit = null;
      while (stack.length && !hit) {
        const node = stack.pop();
        if (!visible(node) && !(cs(node).display === "contents")) continue;
        if (closest(node, '[aria-hidden="true"]')) continue;
        if (directText(node).trim()) {
          const s = cs(node);
          const size = parseFloat(s.fontSize);
          if (s.fontStyle === "italic" && (t === "h1" || size >= 48) && size >= 48) {
            const res = resolveSerif(s.fontFamily);
            if (res.serif) hit = { res, size };
          }
        }
        stack.push(...Array.from(node.children).reverse());
      }
      if (!hit) continue;
      const detail = "italic serif " + t + " (" + (hit.res.primary || "serif") + ") at " + Math.round(hit.size) + 'px "' + collapse(h.textContent).slice(0, 60) + '"';
      if (allow.fonts.includes(hit.res.primary)) suppress("italic-serif-display", "font:" + hit.res.primary, detail);
      else emit("italic-serif-display", detail);
    }
  });

  // text_collectors.rs#label_type_element / label_before_heading / label_near_heading
  function labelTypeElement(label) {
    let el = label;
    for (let i = 0; i < 3; i++) {
      if (cleanInlineText(el)) return el;
      const withText = Array.from(el.children).filter((c) => (c.textContent || "").trim());
      if (withText.length !== 1) return el;
      el = withText[0];
    }
    return el;
  }
  function labelBeforeHeading(heading) {
    let current = heading;
    for (let levels = 0; levels <= 4; levels++) {
      if (current.previousElementSibling) return { label: current.previousElementSibling, levels };
      const parent = current.parentElement;
      if (!parent || parent === D.body || parent === D.documentElement) return null;
      current = parent;
    }
    return null;
  }
  function labelNearHeading(label, heading) {
    const l = rect(label), h = rect(heading);
    if (l.width <= 0 || l.height <= 0 || h.width <= 0 || h.height <= 0) return false;
    if (l.top >= h.bottom) return false;
    return Math.max(h.top - l.bottom, h.left - l.right, 0) <= 96;
  }
  const fontSizeOf = (el) => num(cs(el).fontSize);
  const letterSpacingPx = (el) => {
    const v = resolveLen(cs(el).letterSpacing, fontSizeOf(el));
    return v ? v : 0;
  };
  const YEAR_RE = /\b(19|20)\d{2}\b/;
  function heroEyebrow(h1) {
    const sib = h1.previousElementSibling;
    if (!sib) return null;
    const ty = labelTypeElement(sib);
    const hs = fontSizeOf(h1);
    if (closest(h1, APP_CONTEXT)) return null;
    if (!(hs >= 48)) return null;
    if (HEADING_TAGS.has(tagOf(sib))) return null;
    const text = (sib.textContent || "").trim();
    if (text.length < 2 || text.length > 60) return null;
    const fs = fontSizeOf(ty);
    if (!(fs > 0 && fs <= 14)) return null;
    const ts = cs(ty);
    const upper = ts.textTransform === "uppercase" || (/[A-Z]/.test(text) && !/[a-z]/.test(text));
    const ls = letterSpacingPx(ty);
    const holdsTime = tagOf(sib) === "time" || Boolean(sib.querySelector("time"));
    const dated = ls < HERO_EYEBROW_TRACKING_PX && (holdsTime || YEAR_RE.test(text));
    const tracked = ls >= HERO_EYEBROW_TRACKING_PX || (fs > 0 && ls > 0 && ls >= fs * HERO_EYEBROW_TRACKING_EM - 1e-9);
    const classic = upper && !dated && tracked;
    const weight = Number(ts.fontWeight) || 400;
    const accentBold = weight >= 700 && isAccentColor(ts.color);
    let dash = false;
    for (const which of ["::before", "::after"]) {
      const ps = getComputedStyle(sib, which);
      if (!ps || ps.content === "none" || ps.content === "normal") continue;
      const w = parseFloat(ps.width), hh = parseFloat(ps.height);
      if (!(w >= 8 && w <= 80 && hh >= 1 && hh <= 6)) continue;
      const bg = parseColor(ps.backgroundColor);
      if (bg && bg.a >= 0.1 && spread(bg) >= 30) dash = true;
    }
    if (!classic && !accentBold && !dash) return null;
    const style = classic ? "tracked-caps" : accentBold ? "accent-bold" : "dash-prefix";
    return 'eyebrow chip (' + style + ') "' + text.slice(0, 40) + '" above h1 "' + collapse(h1.textContent).slice(0, 60) + '"';
  }
  run("hero-eyebrow-chip", () => {
    for (const h1 of D.body.querySelectorAll("h1")) {
      if (!visible(h1)) continue;
      const d = heroEyebrow(h1);
      if (d) emit("hero-eyebrow-chip", d);
    }
  });

  const labelCeiling = (headingSize, base) => Math.max(base, Math.min(Number.isFinite(headingSize) && headingSize > 0 ? headingSize * 0.45 : 0, 16));
  function kickerCardContext(heading, kicker) {
    const item = closest(heading, KICKER_CARD_CONTEXT_SELECTOR);
    if (!item) return false;
    const pageScale = vh > 0 && rect(item).height > vh * 2;
    return !pageScale && item.contains(kicker);
  }
  const headingLevel = (h) => {
    const m = tagOf(h).match(/^h([1-6])$/);
    if (m) return Number(m[1]);
    if ((h.getAttribute("role") || "").toLowerCase() !== "heading") return 0;
    const al = parseInt(h.getAttribute("aria-level") || "", 10);
    return Number.isFinite(al) && al >= 1 ? al : 2;
  };
  run("kicker-above-heading", () => {
    const DOC_NUM_RE = /^(§|\d+(\.\d+)+\b|(section|article|clause|appendix|exhibit|schedule|chapter|part|rule|title)\s+([0-9ivxlc]+\b|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b)/i;
    for (const heading of D.body.querySelectorAll('h1, h2, h3, h4, [role="heading"]')) {
      const level = headingLevel(heading);
      if (!level || level > 4) continue;
      if (closest(heading, KICKER_SKIP_SELECTOR) || closest(heading, APP_CONTEXT)) continue;
      const found = labelBeforeHeading(heading);
      if (!found) continue;
      const kicker = found.label;
      if (found.levels > 0 && !labelNearHeading(kicker, heading)) continue;
      if (closest(kicker, KICKER_SKIP_SELECTOR)) continue;
      if (kickerCardContext(heading, kicker)) continue;
      const hfs = fontSizeOf(heading);
      // kicker_in_painted_card
      if (level >= 3 && hfs < LEADING_DISPLAY_TYPE_PX) {
        let inCard = false;
        for (let c = heading.parentElement; c && c !== D.body; c = c.parentElement) {
          if (isCardLike(c)) {
            inCard = rect(c).height < (vh || 800) && c.contains(kicker);
            break;
          }
        }
        if (inCard) continue;
      }
      const headingText = collapse(heading.textContent);
      const kickerText = cleanInlineText(kicker) || collapse(kicker.textContent);
      const kt = labelTypeElement(kicker);
      const kfs = fontSizeOf(kt);
      const kls = letterSpacingPx(kt);
      const ks = cs(kt);
      // is_kicker_candidate
      if (headingText.length < 3 || /^\/[0-9A-Za-z_-]+/.test(headingText.replace(/^"|"$/g, "").trim())) continue;
      if (!(hfs >= 20)) continue;
      const ktag = tagOf(kicker);
      if (HEADING_TAGS.has(ktag) || !["p", "span", "div", "small"].includes(ktag)) continue;
      if (kickerText.length < 2 || kickerText.length > 34) continue;
      if (/^step\s*\d+/i.test(kickerText) || /^\d{1,2}$/.test(kickerText)) continue;
      if (/[·•|]|\s[/›»>]\s|\b(19|20)\d{2}\b/.test(kickerText)) continue;
      if (DOC_NUM_RE.test(kickerText)) continue;
      const smallCaps = /small-caps/.test(ks.fontVariant + " " + ks.fontVariantCaps);
      const upper = ks.textTransform === "uppercase" || (/[A-Z]/.test(kickerText) && !/[a-z]/.test(kickerText)) || smallCaps;
      if (!upper) continue;
      if (!(kfs > 0 && kfs <= labelCeiling(hfs, 14))) continue;
      if (!(kls >= kfs * 0.06)) continue;
      if (tagOf(heading) === "h1" && hfs >= 48 && (kls >= HERO_EYEBROW_TRACKING_PX || (found.levels === 0 && kfs <= 14 && kls > 0 && kls >= kfs * HERO_EYEBROW_TRACKING_EM - 1e-9 && heroEyebrow(heading)))) continue;
      if (!visible(heading) || !visible(kicker)) continue;
      emit("kicker-above-heading", 'kicker "' + kickerText.slice(0, 40) + '" above ' + tagOf(heading) + ' "' + headingText.replace(/^"|"$/g, "").slice(0, 60) + '"');
    }
  });

  const parseNumberedLabel = (raw) => {
    const text = collapse(raw);
    if (!text || text.length > 40) return null;
    const m = text.match(/^(\d{2})$/) || text.match(/^(\d{1,2})\s*[^0-9A-Za-z_\s]\s*\S/);
    if (!m) return null;
    const index = parseInt(m[1], 10);
    if (!Number.isFinite(index) || index > 40) return null;
    return { index, text };
  };
  const isMonoFamily = (ff) => {
    const first = (ff || "").split(",")[0].trim().replace(/^['"]|['"]$/g, "").toLowerCase();
    return MONOSPACE_FACES.includes(first) || first.split(/[^a-z0-9]/).some((w) => ["mono", "monospace", "monospaced", "sfmono"].includes(w));
  };
  run("numbered-section-labels", () => {
    const candidates = [];
    const seen = new Set();
    for (const heading of D.body.querySelectorAll("h2, h3, h4")) {
      if (closest(heading, KICKER_SKIP_SELECTOR)) continue;
      const found = labelBeforeHeading(heading);
      if (!found) continue;
      const label = found.label;
      if (found.levels > 1 && !labelNearHeading(label, heading)) continue;
      if (seen.has(label) || closest(label, KICKER_SKIP_SELECTOR) || HEADING_TAGS.has(tagOf(label))) continue;
      const holdsMedia = Array.from(label.querySelectorAll("img, picture, video, canvas")).length > 0 || Array.from(label.querySelectorAll("svg")).some((svg) => svg.parentElement && parseNumberedLabel(directText(svg.parentElement)));
      if (kickerCardContext(heading, label) || holdsMedia) continue;
      const parsed = parseNumberedLabel(cleanInlineText(label) || collapse(label.textContent));
      if (!parsed) continue;
      const headingText = collapse(heading.textContent);
      const hfs = fontSizeOf(heading);
      const lt = labelTypeElement(label);
      const lfs = fontSizeOf(lt);
      const ls = cs(lt);
      if (headingText.length < 3 || !NUMBERED_LABEL_TAGS.includes(tagOf(label))) continue;
      if (!(lfs > 0 && lfs <= labelCeiling(hfs, 13))) continue;
      if (hfs > 0 && hfs < lfs * 1.3) continue;
      const weight = Number(ls.fontWeight) || 400;
      const spacing = letterSpacingPx(lt);
      if (!(/mono/i.test(ls.fontFamily) || weight >= 600 || spacing >= 0.5 || ls.textTransform === "uppercase" || isAccentColor(ls.color))) continue;
      seen.add(label);
      candidates.push({ index: parsed.index, label: parsed.text.slice(0, 24), tag: tagOf(heading), text: headingText.slice(0, 60) });
    }
    if (candidates.length < 2 || new Set(candidates.map((c) => c.index)).size < 2) return;
    for (const c of candidates) emit("numbered-section-labels", 'tiny numbered label "' + c.label + '" beside ' + c.tag + ' "' + c.text + '" (' + candidates.length + " on page)");
  });

  const EMOJI_ONLY_RE = /^(?:\p{Extended_Pictographic}|\p{Emoji_Component}|\uFE0F|\u200D|\s)+$/u;
  run("icon-tile-stack", () => {
    for (const el of all) {
      const t = tagOf(el);
      let cardTitle = false;
      if (!HEADING_TAGS.has(t)) {
        if (!["div", "span"].includes(t) || !["block", "flow-root"].includes(cs(el).display)) continue;
        const text = directText(el).trim();
        if (text.length < 2 || text.length > 80 || el.children.length) continue;
        if (!(num(cs(el).fontWeight) >= 600 && fontSizeOf(el) >= 16)) continue;
        if (closest(el, 'h1, h2, h3, h4, h5, h6, [role="heading"], a, button, label')) continue;
        cardTitle = true;
      }
      if (!visible(el)) continue;
      const found = labelBeforeHeading(el);
      if (!found) continue;
      // tile_box
      let sib = found.label;
      for (let i = 0; i < 3; i++) {
        const kids = Array.from(sib.children);
        const ss = cs(sib);
        if (ss.display === "contents") {
          if (!kids.length) break;
          sib = kids[kids.length - 1];
          continue;
        }
        const paintsNothing = isTransparent(ss.backgroundColor) && (!ss.backgroundImage || ss.backgroundImage === "none") && (!ss.boxShadow || ss.boxShadow === "none") && !(num(ss.borderTopWidth) > 0);
        if (kids.length === 1 && paintsNothing) {
          const o = rect(sib), n = rect(kids[0]);
          const same = (a, b) => Math.abs(a - b) <= 1;
          if (o.width > 0 && same(o.left, n.left) && same(o.top, n.top) && same(o.width, n.width) && same(o.height, n.height)) {
            sib = kids[0];
            continue;
          }
        }
        break;
      }
      if (found.levels > 0 && !labelNearHeading(sib, el)) continue;
      const st = tagOf(sib);
      if (!st || HEADING_TAGS.has(st)) continue;
      const sr = rect(sib), hr = rect(el);
      const w = sr.width, h = sr.height;
      if (!(w >= 32 && w <= 128 && h >= 32 && h <= 128)) continue;
      if (w / h < 0.7 || w / h > 1.4) continue;
      const ss = cs(sib);
      const bgc = parseColor(ss.backgroundColor);
      const bgVisible = (bgc && bgc.a >= 0.05) || (ss.backgroundImage && ss.backgroundImage !== "none");
      const ring = shadowLayers(ss.boxShadow).filter((l) => l.alpha >= FAINT_PAINT_ALPHA && l.x === 0 && l.y === 0 && l.blur === 0 && l.spread >= 0.5).reduce((m, l) => Math.max(m, l.spread), 0);
      if (!bgVisible && !(Math.max(num(ss.borderTopWidth), ring) > 0)) continue;
      if (num(ss.borderRadius) >= w / 2) continue;
      let icon = sib.querySelector('svg, i[data-lucide], i[class*="fa-"], i[class*="icon"]');
      if (!icon) icon = Array.from(sib.querySelectorAll("*")).find((c) => /url\(/i.test(cs(c).maskImage || cs(c).webkitMaskImage || "")) || null;
      const emoji = !sib.children.length && EMOJI_ONLY_RE.test(directText(sib).trim()) && directText(sib).trim().length > 0;
      if (!icon && !emoji) continue;
      if (icon && rect(icon).width && rect(icon).width >= w * 0.95) continue;
      if (hr.top && sr.bottom && sr.bottom > hr.top + 4) continue;
      emit("icon-tile-stack", Math.round(w) + "x" + Math.round(h) + "px icon tile above " + t + ' "' + (el.textContent || "").trim().slice(0, 60) + '"' + (cardTitle ? " (card title)" : ""));
    }
  });

  run("oversized-h1", () => {
    for (const h1 of D.body.querySelectorAll("h1")) {
      if (!visible(h1)) continue;
      const fs = fontSizeOf(h1);
      const text = collapse(h1.textContent);
      if (fs < 72 || text.length < 40 || !vw || !vh) continue;
      const r = rect(h1);
      const hr = r.height / vh;
      if (hr < 0.28 && (r.width * r.height) / (vw * vh) < 0.25) continue;
      emit("oversized-h1", Math.round(fs) + "px h1, " + text.length + " chars, " + Math.round(hr * 100) + 'vh "' + text.slice(0, 60) + '"');
    }
  });

  // =================================================================================================
  // Per-element paint rules
  // =================================================================================================
  const tellOf = (c) => {
    const h = hueOf(c);
    if (h >= 170 && h <= 197) return lightSat(c)[1] < 0.4 ? null : "cyan";
    if (h >= 260 && h <= 310) return "purple";
    return null;
  };
  const tellLabel = (t) => (t === "cyan" ? "Cyan" : "Purple/violet");
  const aiInk = [];
  const tells = new Set();
  const dedupe = new Set();
  const once = (key) => {
    if (dedupe.has(key)) return false;
    dedupe.add(key);
    return true;
  };
  const isTabContext = (el) => {
    if (closest(el, '[aria-selected="true"], [aria-current]:not([aria-current="false"])')) return true;
    let cur = el;
    for (let d = 0; cur && d < 6; d++, cur = cur.parentElement) {
      if (/(?:^|[\s_-])(?:active|current|selected)(?:$|[\s_-])/i.test(cur.getAttribute("class") || "")) return true;
    }
    return false;
  };
  const cornersOf = (s, width) => {
    const one = (v) => {
      const first = (v || "0").trim().split(/\s+/)[0];
      return first.endsWith("%") ? (parseFloat(first) / 100) * width : num(first);
    };
    return [one(s.borderTopLeftRadius), one(s.borderTopRightRadius), one(s.borderBottomRightRadius), one(s.borderBottomLeftRadius)];
  };
  // rules.rs#is_rounded_away_from_side: the two corners the stripe does not touch
  const roundedAway = (c, i) => {
    const pair = i === 0 ? [c[3], c[2]] : i === 1 ? [c[0], c[3]] : i === 2 ? [c[0], c[1]] : [c[1], c[2]];
    return pair[0] >= 4 && pair[1] >= 4;
  };
  const glyphOnly = (text) => !/[\p{L}\p{N}]/u.test(text);
  const VALID_SHADOW = (l) => l.count >= 3;

  function glowCheck(el) {
    const s = cs(el);
    const box = s.boxShadow && s.boxShadow !== "none" ? s.boxShadow : "";
    let text = s.textShadow && s.textShadow !== "none" ? s.textShadow : "";
    const parent = el.parentElement;
    if (text && parent && cs(parent).textShadow === text) text = "";
    if (!box && !text) return null;
    const info = bgInfo(parent || el);
    const surface = info.color;
    let effective = info.color;
    if (!effective && !info.unresolved) effective = gradientAverage(parent);
    const onDark = effective ? lum(effective) < 0.1 : false;
    const r = rect(el);
    const opacity = ownOpacity(el);
    const perceptible = (blur, spr, alpha, lift) => {
      const extent = blur / 2 + spr;
      if (extent <= 0) return false;
      if (blur * alpha * opacity < 3) return false;
      const area = r.width * r.height;
      if (area <= 0) return false;
      const lit = (r.width + 2 * extent) * (r.height + 2 * extent) - area;
      if (lit > area * 2) return false;
      return lift === null || lift >= 20;
    };
    const scan = (value, prop) => {
      if (!value) return null;
      const layers = shadowLayers(value);
      const lift = surface ? layers.filter((l) => l.color && l.blur * l.alpha * opacity >= 1.5 && hasChroma(l.color, 30)).reduce((sum, l) => sum + 0.5 * l.alpha * opacity * Math.max(Math.abs(l.color.r - surface.r), Math.abs(l.color.g - surface.g), Math.abs(l.color.b - surface.b)), 0) : null;
      for (const l of layers) {
        if (!l.color || !hasChroma(l.color, 30) || l.count < 3 || l.blur <= 4) continue;
        if (!perceptible(l.blur, l.spread, l.alpha, lift)) continue;
        if (l.x === 0 && l.y === 0) return "Zero-offset " + prop + " glow (" + hex(l.color) + ")";
        if (onDark) return "Colored " + prop + " glow (" + hex(l.color) + ") on dark background";
      }
      return null;
    };
    return scan(box, "box-shadow") || scan(text, "text-shadow");
  }

  function radialStops(value) {
    const lower = value.toLowerCase();
    let at = -1;
    for (let i = lower.indexOf("radial-gradient("); i >= 0; i = lower.indexOf("radial-gradient(", i + 1)) {
      if (!lower.slice(Math.max(0, i - 10), i).endsWith("repeating-")) {
        at = i;
        break;
      }
    }
    if (at < 0) return null;
    const open = at + "radial-gradient".length;
    let depth = 0, end = -1;
    for (let i = open; i < value.length; i++) {
      if (value[i] === "(") depth++;
      else if (value[i] === ")") {
        depth--;
        if (depth === 0) {
          end = i;
          break;
        }
      }
    }
    if (end < 0) return null;
    const args = splitTop(value.slice(open + 1, end), ",");
    const stops = [];
    for (const a of args) {
      const m = a.match(new RegExp(COLOR_TOKEN_RE.source, "i"));
      if (!m) continue;
      const transparent = /^transparent$/i.test(m[0]);
      const color = transparent ? { r: 0, g: 0, b: 0, a: 0 } : parseColor(m[0]);
      stops.push({ raw: a, color, transparent: transparent || (color && color.a === 0) });
    }
    return stops;
  }

  // Lazily collected text boxes for radial-spotlight-glow's behind-text gate
  let textBoxes = null;
  const textBoxesAll = () => {
    if (!textBoxes) {
      textBoxes = [];
      for (const el of all) {
        if (textBoxes.length > 5000) break;
        if (cleanInlineText(el) && visible(el)) textBoxes.push(rect(el));
      }
    }
    return textBoxes;
  };

  const marqueeNames = new Set();
  for (const [name, k] of keyframes) {
    const pct = (k.text.match(/\btranslate(?:[xX]|3[dD])?\(\s*(?:calc\(\s*)?(-?[\d.]+)%/gi) || []).map((m) => parseFloat(m.replace(/^.*\(\s*(?:calc\(\s*)?/i, "")));
    if (!pct.length) continue;
    const body = k.text.slice(k.text.indexOf("{") + 1);
    if (pct.length === 1 && /\bscale\(|\bopacity\s*:/i.test(body)) continue;
    const travel = pct.length > 1 ? Math.max(...pct) - Math.min(...pct) : Math.abs(pct[0]);
    if (travel >= 20) marqueeNames.add(name);
  }
  const pulseKeyframes = (name) => {
    const k = keyframes.get(name);
    if (!k) return null;
    const body = k.text.slice(k.text.indexOf("{") + 1);
    return /\bopacity\s*:/i.test(body) || /\bbox-shadow\s*:/i.test(body) || /\btransform\s*:[^;{}]*\bscale/i.test(body);
  };
  const toggleVisibility = (name) => {
    const k = keyframes.get(name);
    if (!k) return false;
    let out = false;
    for (const [prop, value] of k.decls) {
      if (prop === "opacity") {
        if (num(value) <= 0.15) out = true;
      } else if (prop === "visibility") {
        if (/^(hidden|collapse)$/.test(value.trim())) out = true;
      } else if (prop !== "animation-timing-function") return false;
    }
    return out;
  };

  // ---- quality ---------------------------------------------------------------------------------
  function holdsOnlyPhrasing(el) {
    return Array.from(el.children).every((c) => {
      const d = cs(c).display;
      return d === "inline" || d === "contents" || tagOf(c) === "br";
    });
  }
  function lineWidths(el) {
    const range = D.createRange();
    range.selectNodeContents(el);
    const rs = Array.from(range.getClientRects()).filter((x) => x.width > 1 && x.height > 1);
    if (!rs.length) return null;
    const lines = [];
    for (const x of rs) {
      const mid = (x.top + x.bottom) / 2;
      const line = lines.find((l) => mid > l.top && mid < l.bottom);
      if (line) {
        line.left = Math.min(line.left, x.left);
        line.right = Math.max(line.right, x.right);
      } else lines.push({ top: x.top, bottom: x.bottom, left: x.left, right: x.right });
    }
    return lines.map((l) => l.right - l.left);
  }
  function isHeadingText(el) {
    const found = closest(el, LEADING_HEADING_CONTEXT);
    if (!found) return false;
    if (found === el) return true;
    for (let c = el; c && c !== found; c = c.parentElement) if (QUALITY_TEXT_TAGS.has(tagOf(c))) return false;
    return true;
  }
  const textLineCount = (h, pitch, fs) => {
    if (!(Number.isFinite(h) && pitch > 0)) return 1;
    const after = Math.round((h - (fs > 0 ? fs * CONTENT_AREA_EM : 0)) / pitch);
    return 1 + (after > 0 ? after : 0);
  };
  const isCapitalizedRun = (text) => !/\p{Ll}/u.test(text) && /\p{Lu}/u.test(text);
  const letterCount = (text) => (text.match(/\p{L}/gu) || []).length;
  const shortOneLineLabel = (el, lh) => {
    if (letterCount((el.textContent || "").trim()) > TRACKED_LABEL_MAX_CHARS) return false;
    const tr = directTextRect(el);
    return !(lh > 0 && tr && tr.height > lh * 1.5);
  };
  const typedCapsLabel = (el, lh, everyLetter) => {
    const text = (el.textContent || "").trim();
    return isCapitalizedRun(text) && (!everyLetter || !/\p{Ll}|\p{Lo}/u.test(text)) && shortOneLineLabel(el, lh);
  };
  const typedCapsText = (el) => {
    const text = collapse(el.textContent);
    return text.length < ALL_CAPS_LONG_RUN && isCapitalizedRun(text) && !/\p{Lo}/u.test(text);
  };
  const isCjk = (text) => {
    let cjk = 0, scripted = 0;
    for (const ch of Array.from(text).slice(0, 256)) {
      if (/[\u1100-\u11FF\u3040-\u30FF\u3130-\u318F\u31F0-\u31FF\u3400-\u4DBF\u4E00-\u9FFF\uA960-\uA97F\uAC00-\uD7FF\uF900-\uFAFF\uFF66-\uFF9F]/.test(ch)) {
        cjk++;
        scripted++;
      } else if (/\p{L}/u.test(ch)) scripted++;
    }
    return cjk > 0 && cjk * 2 >= scripted;
  };
  function inControl(el) {
    if (closest(el, INTERACTIVE)) return true;
    for (let c = el; c; c = c.parentElement) {
      const ti = c.getAttribute("tabindex");
      if (ti !== null && !(parseFloat(ti) < 0) && collapse(c.textContent).length <= 80) return true;
    }
    return false;
  }
  function textFlushSides(el, r) {
    const flush = [false, false, false, false];
    const edge = 4;
    const smallChip = r.height > 0 && r.height < 27.5;
    for (const node of el.querySelectorAll(TEXT_EDGE_QUERY)) {
      if (!directText(node).trim()) continue;
      const br = rect(node);
      if (br.width <= 0 || br.height <= 0) continue;
      if (br.bottom < r.top || br.top > r.bottom || br.right < r.left || br.left > r.right) continue;
      if (!(br.top - r.top <= edge || r.right - br.right <= edge || r.bottom - br.bottom <= edge || br.left - r.left <= edge)) continue;
      let nr = directTextRect(node) || br;
      nr = { left: Math.max(nr.left, br.left), right: Math.min(nr.right, br.right), top: Math.max(nr.top, br.top), bottom: Math.min(nr.bottom, br.bottom) };
      if (nr.right <= nr.left || nr.bottom <= nr.top) continue;
      const fs = num(cs(node).fontSize);
      const land = smallChip && fs > 0 ? Math.min(4, fs * 0.25) : edge;
      const lands = (gap) => gap <= land && gap >= -edge;
      const sides = [lands(nr.top - r.top), lands(r.right - nr.right), lands(r.bottom - nr.bottom), lands(nr.left - r.left)];
      if (!sides.some(Boolean)) continue;
      if (visuallyHidden(node) || !visible(node)) continue;
      // text_is_the_wrappers
      let ours = true;
      for (let c = node; c && c !== el; c = c.parentElement) {
        const cst = cs(c);
        if (parseFloat(cst.opacity) <= 0.02 || cst.position === "absolute" || cst.position === "fixed" || (cst.display !== "inline" && hasVisibleBgBoundary(c))) {
          ours = false;
          break;
        }
      }
      if (!ours) continue;
      sides.forEach((v, i) => (flush[i] = flush[i] || v));
    }
    return flush;
  }

  function quality(el, t, s, r) {
    const dt = directText(el);
    const hasDirect = dt.trim().length > 10;
    const textLen = (el.textContent || "").trim().length;
    const fs = parseFloat(s.fontSize) || 16;
    const lh = resolveLen(s.lineHeight, fs);
    const ls = resolveLen(s.letterSpacing, fs);
    const isHeading = HEADING_TAGS.has(t);
    const where = " " + describe(el);

    // line-length
    const proseInPhrasing = !hasDirect && ["p", "li", "dd", "blockquote"].includes(t) && holdsOnlyPhrasing(el);
    if ((hasDirect || proseInPhrasing) && QUALITY_TEXT_TAGS.has(t) && r.width > 0 && textLen > LINE_MAX) {
      const widths = lineWidths(el);
      if (widths && widths.length) {
        const total = widths.reduce((a, b) => a + b, 0);
        const renderedLen = collapse(el.textContent).length;
        const chars = (w) => (renderedLen * w) / total;
        const long = widths.filter((w) => chars(w) > LINE_MAX + 5).length;
        if (total > 0 && long >= 2) emit("line-length", "~" + Math.round(chars(Math.max(...widths))) + " chars on " + long + " of " + widths.length + " rendered lines (aim for <" + LINE_MAX + ")" + where);
      }
    }

    // cramped-padding: text too close to its own bounded box
    const sides = ["Top", "Right", "Bottom", "Left"];
    const bw = sides.map((k) => num(s["border" + k + "Width"]));
    const bv = sides.map((k, i) => bw[i] > 0 && !isTransparent(s["border" + k + "Color"]) && s["border" + k + "Style"] !== "none");
    if (!(t === "code" && !closest(el, "pre")) && hasDirect && textLen > 20 && r.width > 100 && s.display !== "inline" && r.height > 30) {
      const hasBg = hasVisibleBgBoundary(el);
      if (bv.filter(Boolean).length >= 2 || hasBg) {
        const tr = directTextRect(el);
        if (tr) {
          const insets = [tr.top - (r.top + bw[0]), r.right - bw[1] - tr.right, r.bottom - bw[2] - tr.bottom, tr.left - (r.left + bw[3])];
          const layoutWidth = D.documentElement.clientWidth > 0 && D.documentElement.clientWidth <= vw ? D.documentElement.clientWidth : vw;
          const fullBleed = hasBg && vw > 0 && r.left <= 2 && r.right >= layoutWidth - 2;
          const vSides = [], hSides = [];
          if (hasBg || bv[0]) vSides.push(0);
          if (hasBg || bv[2]) vSides.push(2);
          if ((hasBg && !fullBleed) || bv[3]) hSides.push(3);
          if ((hasBg && !fullBleed) || bv[1]) hSides.push(1);
          // Judge the box the eye sees: background-clip:content-box paints no colour under the padding,
          // so with no border that padding is not visible space inside the box
          const clipToContent = hasBg && !bv.some(Boolean) && s.backgroundClip === "content-box";
          const pads = sides.map((k) => (clipToContent ? 0 : num(s["padding" + k])));
          if (clipToContent) sides.forEach((k, i) => { insets[i] -= num(s["padding" + k]); });
          const minOver = (list, f) => list.reduce((m, i) => Math.min(m, f(i)), Infinity);
          const vMin = minOver(vSides, (i) => pads[i]), hMin = minOver(hSides, (i) => pads[i]);
          const vInset = minOver(vSides, (i) => insets[i]), hInset = minOver(hSides, (i) => insets[i]);
          const vT = Math.max(4, fs * 0.3), hT = Math.max(8, fs * 0.5);
          if (vMin < vT && vInset < vT) emit("cramped-padding", Math.round(vInset * 10) / 10 + "px of space above and below the text (need ≥" + vT.toFixed(1) + "px for " + fs + "px text)" + where);
          else if (hMin < hT && hInset < hT) emit("cramped-padding", Math.round(hInset * 10) / 10 + "px of space beside the text (need ≥" + hT.toFixed(1) + "px for " + fs + "px text)" + where);
        }
      }
    }
    // cramped-padding: children's text flush against a visible boundary
    const children = Array.from(el.children);
    if (!FLUSH_SKIP_TAGS.has(t) && !hasDirect && s.position !== "fixed" && s.position !== "absolute" && r.width > 0 && r.height > 0 && s.display !== "inline" && children.length) {
      const outlineVisible = num(s.outlineWidth) > 0 && !isTransparent(s.outlineColor) && s.outlineStyle && s.outlineStyle !== "none";
      const bgVisible = hasVisibleBgBoundary(el);
      if (bv.some(Boolean) || outlineVisible || bgVisible) {
        const pad = sides.map((k) => num(s["padding" + k]));
        const overflow = [false, false, false, false], insulate = [false, false, false, false];
        for (const c of children) {
          const cst = cs(c);
          const cr = rect(c);
          if (cr.width > 0 && cr.height > 0) {
            if (r.top - cr.top > 1) overflow[0] = true;
            if (cr.right - r.right > 1) overflow[1] = true;
            if (cr.bottom - r.bottom > 1) overflow[2] = true;
            if (r.left - cr.left > 1) overflow[3] = true;
            if (cr.top - r.top >= 4) insulate[0] = true;
            if (r.right - cr.right >= 4) insulate[1] = true;
            if (r.bottom - cr.bottom >= 4) insulate[2] = true;
            if (cr.left - r.left >= 4) insulate[3] = true;
          }
          sides.forEach((k, i) => {
            if (num(cst["padding" + k]) >= 4 || num(cst["margin" + k]) >= 4) insulate[i] = true;
          });
        }
        const fullBand = vw > 0 && r.width >= vw * 0.94 && bgVisible && !outlineVisible;
        let textFlush = null;
        const flushSides = [];
        const names = ["top", "right", "bottom", "left"];
        for (let i = 0; i < 4; i++) {
          const bounded = bv[i] || outlineVisible || (bgVisible && !(fullBand && (i === 1 || i === 3)));
          if (!bounded || pad[i] > 2 || insulate[i] || overflow[i]) continue;
          if (!textFlush) textFlush = textFlushSides(el, r);
          if (textFlush[i]) flushSides.push(names[i]);
        }
        if (flushSides.length && children.some((c) => (c.textContent || "").trim().length > 4)) {
          const visibleSides = names.filter((_, i) => bv[i]);
          const parts = [];
          if (visibleSides.length === 4) parts.push("border");
          else if (visibleSides.length) parts.push("border-" + visibleSides.join("/"));
          if (outlineVisible) parts.push("outline");
          if (bgVisible) parts.push("bg");
          const cls = ((el.getAttribute("class") || "").trim().split(/\s+/)[0]) || "";
          emit("cramped-padding", "<" + t + ">" + (cls ? ' "' + cls + '"' : "") + ": children flush against " + parts.join("+") + " on " + (flushSides.length === 4 ? "all sides" : flushSides.join("/")) + " (no inset)");
        }
      }
    }

    // body-text-viewport-edge (Layoza: scroller/track exemptions reduced to "no scrolling ancestor")
    if ((t === "p" || t === "li") && (hasDirect || holdsOnlyPhrasing(el)) && textLen > 40 && vw > 0) {
      const hasOwnBg = !isTransparent(s.backgroundColor);
      let inScroller = false;
      for (let c = el.parentElement; c && c !== D.body; c = c.parentElement) if (/auto|scroll/.test(cs(c).overflowX)) inScroller = true;
      if (!closest(el, "nav") && !closest(el, "header") && !hasOwnBg && s.position !== "fixed" && s.position !== "absolute" && r.width / vw > 0.5 && !inScroller) {
        const tr = allTextRect(el);
        let span = null;
        if (tr) {
          const cl = r.left + bw[3] + num(s.paddingLeft), cr = r.right - bw[1] - num(s.paddingRight);
          const pitch = lh > 0 ? lh : fs * 1.2;
          if (textLineCount(tr.height, pitch, fs) >= 2 && tr.width >= (cr - cl) * 0.9) span = [cl, cr];
          else if (s.display === "list-item") span = s.direction === "rtl" ? [tr.left, Math.max(tr.right, cr)] : [Math.min(tr.left, cl), tr.right];
          else span = [tr.left, tr.right];
        } else if (hasDirect) span = [r.left, r.right];
        if (span && span[1] > 0 && span[0] < vw) {
          const floor = vw <= 480 ? 12 : 16;
          const l = Math.round(span[0]), rr = Math.round(vw - span[1]);
          const leftClose = l >= 0 && span[0] < floor, rightClose = rr >= 0 && span[1] > vw - floor;
          if (leftClose || rightClose) emit("body-text-viewport-edge", "<" + t + "> with " + textLen + "-char body bleeds to viewport edge (" + (leftClose && rightClose ? "left " + l + "px / right " + rr + "px" : leftClose ? "left " + l + "px" : "right " + rr + "px") + ")" + where);
        }
      }
    }

    // tight-leading
    if (hasDirect && textLen > 50 && !isHeading && fs > 0 && fs < LEADING_DISPLAY_TYPE_PX && lh !== null) {
      let pitch = lh;
      if (s.display === "inline") {
        for (let p = el.parentElement; p; p = p.parentElement) {
          const d = cs(p).display;
          if (d !== "inline" && d !== "contents") {
            const block = resolveLen(cs(p).lineHeight, num(cs(p).fontSize));
            if (block && block > lh) pitch = block;
            break;
          }
        }
      }
      const ratio = pitch / fs;
      if (ratio > 0 && Math.round(ratio * 100) / 100 < 1.3) {
        const tr = directTextRect(el) || r;
        let itemsFit = false;
        if (/flex|grid/.test(s.display) && el.children.length) itemsFit = Array.from(el.children).every((c) => rect(c).height <= pitch * 1.5);
        const wraps = tr.height >= pitch * LEADING_MIN_LINE_BOXES && !itemsFit;
        const weight = num(s.fontWeight) || 400;
        const clamped = /-webkit-(inline-)?box/.test(s.display) || (!/^(|none)$/.test((s.webkitLineClamp || "").trim()) && /flow-root|inline-block/.test(s.display));
        const boldTitle = weight >= 600 && (clamped || textLineCount(tr.height, pitch, fs) <= 2);
        if (wraps && !visuallyHidden(el) && !isHeadingText(el) && !boldTitle) emit("tight-leading", "line-height " + ratio.toFixed(2) + "x (need >=1.3)" + where);
      }
    }

    // justified-text
    if (hasDirect && s.textAlign === "justify" && r.width > 0 && fs > 0 && (s.hyphens || s.webkitHyphens) !== "auto" && r.width / (fs * 0.5) <= 45 && !isCjk(dt)) emit("justified-text", "text-align: justify without hyphens: auto" + where);

    // tiny-text
    if (hasDirect && textLen > 20 && fs < 12 && !["sub", "sup", "code", "kbd", "samp", "var", "caption", "figcaption"].includes(t) && !closest(el, TINY_TEXT_UI_CONTEXT)) {
      const upper = s.textTransform === "uppercase" || typedCapsLabel(el, lh, true);
      const codeRun = isMonoFamily(s.fontFamily) && /^pre/.test(s.whiteSpace);
      if (!upper && !codeRun) emit("tiny-text", fs + "px body text" + where);
    }

    // undersized-ui-text
    {
      const own = collapse(dt);
      if (fs > 0 && fs < UI_TEXT_FLOOR_PX && own.length >= 2 && !["sub", "sup", "option"].includes(t) && !closest(el, "sub, sup")) {
        const codeRun = isMonoFamily(s.fontFamily) && /^pre/.test(s.whiteSpace) && /[{}[\]<>=;"`\\_]/.test(own);
        if (!codeRun && !matchesOrClosest(el, EXEMPT_CONTEXT) && !visuallyHidden(el)) {
          const interactive = inControl(el);
          const furniture = matchesOrClosest(el, FURNITURE);
          const floor = !interactive && matchesOrClosest(el, SMALLPRINT) ? SMALLPRINT_TEXT_FLOOR_PX : UI_TEXT_FLOOR_PX;
          if (fs < floor - 0.1 && (interactive || furniture || own.length <= 20)) emit("undersized-ui-text", fs + 'px functional text "' + own.slice(0, 40) + '" (below ' + floor + "px floor)");
        }
      }
    }

    // all-caps-body
    if (hasDirect && s.textTransform === "uppercase" && !isHeading && collapse(dt).length >= ALL_CAPS_LONG_RUN) emit("all-caps-body", "text-transform: uppercase on " + collapse(dt).length + " chars of body text" + where);

    // wide-tracking
    if (hasDirect && textLen > 20 && ls !== null && ls > 0 && fs > 0 && ls / fs > 0.05) {
      const controlLabel = Boolean(closest(el, TRACKED_CONTROL_LABEL)) && shortOneLineLabel(el, lh);
      const lowered = s.textTransform === "lowercase";
      const capsLabel = s.textTransform === "uppercase" || (!lowered && (typedCapsText(el) || typedCapsLabel(el, lh, false)));
      if (!capsLabel && !controlLabel) emit("wide-tracking", "letter-spacing: " + (ls / fs).toFixed(2) + "em on body text" + where);
    }

    // extreme-negative-tracking
    if (hasDirect && textLen > 20 && fs > 0 && ls !== null && ls < 0) {
      const em = ls / fs;
      if (em < (fs >= 40 ? -0.09 : -0.07) && !isCjk(collapse(el.textContent))) emit("extreme-negative-tracking", "letter-spacing: " + em.toFixed(2) + "em at " + fs + 'px — "' + collapse(el.textContent).slice(0, 40) + '"');
    }
  }

  const gptPairs = new Map();
  for (const el of all) {
    run("element", () => {
      const t = tagOf(el);
      if (NON_RENDERED_TAGS.has(t)) return;
      const s = cs(el);
      if (s.display === "none") return;
      const isVis = visible(el);
      const r = rect(el);

      // ---- buried-raster (quality.rs, element form) — runs before the visibility gate on purpose
      {
        const op = parseFloat(s.opacity);
        const bgImg = s.backgroundImage || "";
        if (op >= 0 && op < 0.15 && (t === "img" || /url\(/i.test(bgImg)) && shown(el.parentElement)) {
          const svgSource = t === "img" ? /\.svg(?:[?#]|$)|^data:image\/svg\+xml/i.test(el.getAttribute("src") || "") : (bgImg.match(/url\(([^)]*)\)/gi) || []).every((u) => /\.svg(?:[?#"']|\)|$)|data:image\/svg\+xml/i.test(u));
          // Layoza: raster_is_state_layer reduced to its icon-size and blurred-placeholder gates
          const icon = r.width <= 48 && r.height <= 48;
          const blurM = (s.filter || "").match(/blur\(\s*([\d.]+)px/i);
          const blurred = Boolean(blurM) && parseFloat(blurM[1]) >= 4;
          if (!svgSource && !icon && !blurred && r.width > 0 && r.height > 0) {
            const label = t === "img" ? el.getAttribute("alt") || "" : collapse(el.textContent).slice(0, 40);
            emit("buried-raster", (t === "img" ? "<img>" : "raster background") + " at opacity " + op + (label ? ' "' + label + '"' : "") + " " + classSel(el).slice(0, 60));
          }
        }
        // css_scan.rs#scan_css_text_for_buried_raster, on the computed layer list
        if (isVis && /url\(/i.test(bgImg) && /gradient\(/i.test(bgImg) && (s.backgroundBlendMode || "normal").split(",").every((m) => m.trim() === "normal") && (s.mixBlendMode || "normal") === "normal") {
          const firstUrl = bgImg.search(/url\(/i);
          const washes = splitTop(bgImg, ",").filter((layer) => /gradient\(/i.test(layer) && bgImg.indexOf(layer) < firstUrl);
          const opaque = washes.some((g) => {
            const cols = colorTokens(g).map(parseColor).filter(Boolean);
            return cols.length > 0 && cols.every((c) => c.a >= 0.9);
          });
          if (opaque && once("buried-wash:" + bgImg)) emit("buried-raster", "raster under a near-opaque gradient wash: " + bgImg.slice(0, 90));
        }
      }
      // Layoza: motion rules read every laid-out box whatever its opacity (a blink or pulse animates
      // opacity, so the capture can land on a transparent frame)
      if (r.width > 0 && r.height > 0 && hasBox(el, false)) {
        // ---- motion: bounce-easing, pulsing-dot, blinking-cursor, marquee --------------------------
        const names = animationNames(el);
        if (!SAFE_TAGS.has(t)) {
          const bounceName = names.find((n) => /bounce|elastic|wobble|jiggle|spring/i.test(n));
          if (bounceName && keyframesOnlyPulse(bounceName) !== true && once("bounce:" + bounceName)) emit("bounce-easing", "animation: " + bounceName + " " + classSel(el).slice(0, 60));
          if (/(?:^|\s)animate-bounce(?:\s|$)/.test(el.getAttribute("class") || "") && once("bounce:tw")) emit("bounce-easing", "animate-bounce (Tailwind)");
          const tf = [s.animationTimingFunction, s.transitionTimingFunction].filter(Boolean).join(" ");
          for (const m of tf.matchAll(/cubic-bezier\(\s*([\d.-]+)\s*,\s*([\d.-]+)\s*,\s*([\d.-]+)\s*,\s*([\d.-]+)\s*\)/g)) {
            const y1 = parseFloat(m[2]), y2 = parseFloat(m[4]);
            if (y1 < -0.1 || y1 > 1.1 || y2 < -0.1 || y2 > 1.1) {
              // Layoza: one report per distinct curve (upstream reports per element; a curve set on
              // `*` would otherwise repeat on every node)
              if (once("bezier:" + m[0])) emit("bounce-easing", m[0] + " " + classSel(el).slice(0, 60));
              break;
            }
          }
        }
        if (names.length) {
          names.forEach((name, i) => {
            if (!isInfiniteAt(el, i)) return;
            // pulsing-dot (css_scan.rs#scan_css_text_for_pulsing_dot): layout size, not transformed size
            const known = pulseKeyframes(name);
            const pulse = known === null ? /pulse|blink|ping/i.test(name) : known;
            const w = el.offsetWidth, h = el.offsetHeight;
            if (pulse && w >= 2 && h >= 2 && w <= 16 && h <= 16) {
              const rv = (s.borderTopLeftRadius || "").trim().split(/\s+/)[0];
              const round = rv.endsWith("%") ? parseFloat(rv) >= 40 : num(rv) >= 999 || num(rv) >= 0.4 * Math.min(w, h);
              if (round && once("pulse:" + classSel(el) + ":" + name)) {
                const inLandmark = Boolean(closest(el, 'header, nav, [role="banner"], [role="navigation"]'));
                emit("pulsing-dot", classSel(el).slice(0, 60) + " — " + w + "x" + h + 'px dot with infinite "' + name + '" animation' + (inLandmark ? " in header/nav" : ""));
              }
            }
            if (marqueeNames.has(name) && once("marquee:" + classSel(el) + ":" + name)) emit("marquee", classSel(el).slice(0, 60) + ' — infinite horizontal loop animation "' + name + '"');
          });
          // blinking-cursor (element_checks.rs#check_element_blinking_cursor_dom)
          if (!["input", "textarea", "select", "img", "svg"].includes(t) && iterationCounts(el).includes("infinite")) {
            const blink = names.find((n) => /blink|caret|cursor/i.test(n)) || names.find(toggleVisibility);
            if (blink && !el.isContentEditable && !closest(el, '[contenteditable=""], [contenteditable="true"], [role="textbox"]')) {
              const pageTop = r.top + (window.scrollY || 0);
              const text = (el.textContent || "").trim();
              let cursor = text.length === 1 && /^[_|\u2580-\u259F\u25A0\u25AE\u2759\u275A\uFF5C]$/.test(text);
              if (!cursor && !text && !el.children.length) {
                const bg = parseColor(s.backgroundColor);
                const filled = (bg && bg.a > 0.2) || ["Left", "Right", "Bottom"].some((k) => num(s["border" + k + "Width"]) >= 1);
                const vertical = r.width >= 1 && r.width <= 24 && r.height >= 6 && r.height <= 48 && r.height >= r.width;
                const underscore = r.height >= 1 && r.height <= 6 && r.width >= 4 && r.width <= 24;
                cursor = filled && (vertical || underscore) && num(s.borderRadius) < 0.4 * Math.min(r.width, r.height);
              }
              if (cursor && pageTop <= 1200) emit("blinking-cursor", classSel(el).slice(0, 60) + " — " + Math.round(r.width) + "x" + Math.round(r.height) + 'px blinking cursor (animation "' + blink + '") in the first viewport');
            }
          }
        }
        if (t === "marquee" && once("marquee:tag")) emit("marquee", "<marquee> element");

      }
      if (!isVis) return;
      const opacityChain = effectiveOpacity(el);
      if (opacityChain <= 0.02) return;

      // ---- side-tab / border-accent-on-rounded (rules.rs#check_borders) ----------------------
      if (!BORDER_SAFE_TAGS.has(t) && r.width >= 20 && r.height >= 20 && !closest(el, '[role="status"], [role="alert"], [role="alertdialog"], [role="log"], [aria-live="polite"], [aria-live="assertive"]')) {
        const sides = ["Top", "Right", "Bottom", "Left"];
        const widths = sides.map((k) => num(s["border" + k + "Width"]));
        const colors = sides.map((k) => s["border" + k + "Color"]);
        if (widths.some((w) => w > 0)) {
          const corners = cornersOf(s, r.width);
          const radius = num(s.borderRadius);
          const tab = isTabContext(el);
          for (let i = 0; i < 4; i++) {
            const w = widths[i];
            if (w < 1 || isNeutralColor(colors[i]) || s["border" + sides[i] + "Style"] === "none") continue;
            const maxOther = Math.max(...widths.filter((_, j) => j !== i));
            if (!(w >= 2 && (maxOther <= 1 || w >= maxOther * 2))) continue;
            const sn = sides[i].toLowerCase();
            if (i === 1 || i === 3) {
              if (!roundedAway(corners, i)) continue;
              if (radius > 0) emit("side-tab", "border-" + sn + ": " + w + "px + border-radius: " + radius + "px " + describe(el));
              else if (w >= 3) emit("side-tab", "border-" + sn + ": " + w + "px " + describe(el));
              else {
                const pair = i === 1 ? [corners[0], corners[3]] : [corners[1], corners[2]];
                const far = Math.min(pair[0], pair[1]);
                if (far > 0) emit("side-tab", "border-" + sn + ": " + w + "px + border-radius: " + far + "px " + describe(el));
              }
            } else if (radius > 0 && w >= 2) {
              emit("border-accent-on-rounded", "border-" + sn + ": " + w + "px + border-radius: " + radius + "px " + describe(el));
            } else if (!tab && w >= 3 && w <= 12 && roundedAway(corners, i)) {
              emit("side-tab", "border-" + sn + ": " + w + "px " + describe(el));
            }
          }
        }
      }

      // ---- colours: gradient-text, ai-color-palette (purple text), gray-on-color ----------------
      const direct = directText(el);
      const hasDirect = direct.trim().length > 0;
      const bgClip = s.webkitBackgroundClip || s.backgroundClip || "";
      if (hasDirect || el.textContent.trim()) {
        if (/\btext\b/.test(bgClip) && /gradient\(/i.test(s.backgroundImage || "")) {
          // Layoza: upstream reports SAFE_TAGS gradient text through its page-level CSS-text pass;
          // here every element with clipped gradient text is read directly.
          const stops = gradientColors(s.backgroundImage);
          const strongest = stops.reduce((m, c) => Math.max(m, c.a), 0);
          if (!stops.length || strongest * ownOpacity(el) >= 0.15) emit("gradient-text", "background-clip: text + gradient " + describe(el));
        }
      }
      if (hasDirect && !SAFE_TAGS.has(t) && r.height >= 10 && r.width >= 10) {
        const ink = parseColor(s.color);
        if (ink && !glyphOnly(direct) && ink.a > 0.02) {
          if (hasChroma(ink, 50)) {
            const h = hueOf(ink);
            if (h >= 260 && h <= 310 && (["h1", "h2", "h3"].includes(t) || num(s.fontSize) >= 20)) {
              tells.add("purple");
              const d = "Purple/violet text (" + hex(ink) + ") on heading " + describe(el);
              if (allow.palettes.includes("purple")) suppress("ai-color-palette", "palette:purple", d);
              else emit("ai-color-palette", d);
            }
          }
          if (!/\btext\b/.test(bgClip) && isGrayInk(ink)) {
            const info = bgInfo(el);
            const bgs = info.color ? [info.color] : info.unresolved ? [] : gradientColors(cs(info.gradient).backgroundImage).filter((c) => c.a > 0);
            const readsAsColour = (bg) => {
              const L = lum(bg);
              const scale = L >= 0.01 ? 1 : Math.sqrt(0.01 / Math.max(L, 1e-4));
              return hasChroma(bg, 40 * scale);
            };
            if (bgs.length && bgs.every(readsAsColour)) emit("gray-on-color", "text " + hex(ink) + " on bg " + (bgs.length === 1 ? hex(bgs[0]) : "gradient(" + bgs.map(hex).join(", ") + ")") + " " + describe(el));
          }
        }
      }

      // ---- ai-color-palette: gradient and neon ink (element_checks.rs#check_element_ai_palette_dom)
      {
        const stops = gradientColors(s.backgroundImage);
        if (stops.length >= 2 && !stops.every((c) => c.r === stops[0].r && c.g === stops[0].g && c.b === stops[0].b && c.a === stops[0].a) && Math.min(r.width, r.height) >= 8 && r.width * r.height >= 256) {
          const blurM = (s.filter || "").match(/blur\(\s*([\d.]+)px/i);
          const blur = blurM ? parseFloat(blurM[1]) : 0;
          if (blur < 24) {
            const op = ownOpacity(el);
            let painted = 0, inBand = 0, bridge = 0, tell = null, purple = false;
            for (const c of stops) {
              if (c.a * op < 0.15 || !hasChroma(c, 50)) continue;
              painted++;
              const b = tellOf(c);
              if (b) {
                inBand++;
                if (b === "purple") purple = true;
                if (!tell) tell = b;
              } else if (hueOf(c) >= 197 && hueOf(c) < 260) bridge++;
            }
            if (tell && inBand * 2 >= painted && !(!purple && inBand * 2 === painted && bridge < painted - inBand)) {
              tells.add(tell);
              const d = tellLabel(tell) + " gradient background " + classSel(el).slice(0, 60);
              if (allow.palettes.includes(tell)) suppress("ai-color-palette", "palette:" + tell, d);
              else emit("ai-color-palette", d);
            }
          }
        }
        if (directText(el).trim().length >= 2 && !(el instanceof SVGElement)) {
          const tc = parseColor(s.color);
          if (tc && tc.a * ownOpacity(el) >= 0.15 && hasChroma(tc, 80)) {
            const tell = tellOf(tc);
            if (tell) {
              const info = el.parentElement ? bgInfo(el.parentElement) : { color: null };
              let bg = info.color;
              if (!bg && !info.unresolved) bg = gradientAverage(el.parentElement);
              if (bg && lum(bg) < 0.1) {
                tells.add(tell);
                aiInk.push({ tell, detail: tellLabel(tell) + " neon text on dark background " + describe(el) });
              }
            }
          }
        }
      }

      // ---- dark-glow (rules.rs#check_glow) ------------------------------------------------------
      {
        const g = glowCheck(el);
        if (g) emit("dark-glow", g + " " + classSel(el).slice(0, 60));
      }

      // ---- organic-clip-path (css_scan.rs#scan_css_text_for_organic_clip_path, computed) --------
      {
        const cp = s.clipPath || "";
        const pm = cp.match(/^(polygon|path)\((.*)\)$/i);
        if (pm && once("clip:" + cp)) {
          if (pm[1].toLowerCase() === "path") {
            const curves = (pm[2].match(/[CcSsQqTtAa]/g) || []).length;
            if (curves >= 3) emit("organic-clip-path", "clip-path: path() with " + curves + " curve segments " + classSel(el).slice(0, 60));
          } else {
            const points = pm[2].split(",").map((p) => p.trim()).filter(Boolean).filter((p, i) => !(i === 0 && /^(nonzero|evenodd)$/i.test(p)));
            if (points.length >= 10) {
              let offGrid = 0;
              for (const p of points) for (const n of p.match(/-?\d*\.?\d+/g) || []) if (Math.abs(parseFloat(n) - Math.round(parseFloat(n) / 25) * 25) > 0.5) offGrid++;
              const keys = points.map((p) => p.split(/\s+/).map((c) => (parseFloat(c) === 0 ? "0" : c.toLowerCase())));
              const rectilinear = keys.every((a, i) => {
                const b = keys[(i + 1) % keys.length];
                return a.length === 2 && b.length === 2 && (a[0] === b[0] || a[1] === b[1]);
              });
              if (offGrid >= points.length && !rectilinear) emit("organic-clip-path", "clip-path: polygon() with " + points.length + " vertices approximating an organic contour " + classSel(el).slice(0, 60));
            }
          }
        }
      }

      // ---- background tells: radial-halo, radial-spotlight-glow, repeating-stripes, codex-grid --
      const img = s.backgroundImage || "";
      if (img && img !== "none" && /gradient\(/i.test(img)) {
        if (darkPage && !/url\(/i.test(img)) {
          for (const g of img.matchAll(/(repeating-)?radial-gradient\(/gi)) {
            if (g[1]) continue;
            const stops = radialStops(img.slice(g.index));
            if (!stops || stops.length < 2) continue;
            if (stops.some((st) => (st.raw.match(/-?[\d.]+px\b/g) || []).some((x) => Math.abs(parseFloat(x)) <= 24))) continue;
            const first = stops[0], last = stops[stops.length - 1];
            if (!last.color || last.color.a > 0.05 || !first.color || first.transparent || first.color.a < 0.7 || spread(first.color) < 24) continue;
            const snippet = "radial-gradient halo (" + hex(first.color) + " → transparent) on dark page";
            if (once("halo:" + snippet)) emit("radial-halo", snippet);
          }
        }
        // radial-spotlight-glow (measures.rs#radial_spotlight_stops + radial_glow_prominent_stop)
        const rs = radialStops(img);
        if (rs && rs.length >= 2 && r.width >= 240 && r.height >= 160) {
          const last = rs[rs.length - 1];
          const lastA = last.transparent ? 0 : last.color ? last.color.a : 1;
          const colored = rs.filter((st) => !st.transparent && st.color && st.color.a > 0.05);
          if (lastA <= 0.05 && colored.length >= 1 && colored.length <= 2 && !colored.some((st) => st.color.a >= 0.45)) {
            const chromatic = colored.map((st) => st.color).filter((c) => hasChroma(c, 24));
            const op = effectiveOpacity(el);
            const beneath = splitTop(img, ",").slice(1).join(",");
            const backdrop = /gradient\(|url\(/i.test(beneath) ? null : bgInfo(el).color;
            let best = null;
            for (const c of chromatic) {
              const eff = c.a * op;
              if (eff < 0.14) continue;
              let score = eff;
              if (backdrop) {
                const ratio = contrastRatio(over({ r: c.r, g: c.g, b: c.b, a: eff }, backdrop), backdrop);
                if (ratio < 1.3) continue;
                score = ratio;
              }
              if (!best || score > best.score) best = { c, score };
            }
            if (best && textBoxesAll().some((b) => b.left < r.right && b.right > r.left && b.top < r.bottom && b.bottom > r.top)) {
              const label = el.getAttribute("data-name") || el.id || ((el.getAttribute("class") || "").trim().split(/\s+/)[0]) || t;
              emit("radial-spotlight-glow", 'radial-gradient spotlight glow "' + label + '" (' + hex(best.c) + " a" + best.c.a.toFixed(2) + " → transparent) on " + Math.round(r.width) + "x" + Math.round(r.height) + " surface");
            }
          }
        }
        // repeating-stripes-gradient (html_patterns.rs#repeating_gradient_repeats)
        for (const m of img.matchAll(/repeating-(linear|radial|conic)-gradient\(/gi)) {
          const args = img.slice(m.index + m[0].length);
          let depth = 1, end = args.length;
          for (let i = 0; i < args.length; i++) {
            if (args[i] === "(") depth++;
            else if (args[i] === ")" && --depth === 0) {
              end = i;
              break;
            }
          }
          const stops = splitTop(args.slice(0, end), ",");
          const tokens = (stop) => splitTop(stop, " ").map((x) => x.toLowerCase());
          const firstTok = tokens(stops[0] || "")[0] || "";
          const firstIsColor = Boolean(parseColor(firstTok)) || firstTok === "transparent" || firstTok === "currentcolor" || /^(rgba?|hsla?|oklch|oklab|lab|lch|hwb|color)\(/.test(firstTok);
          const fi = firstIsColor ? 0 : 1;
          const pos = (tok) => {
            if (!tok) return null;
            const tt = tok.replace(/\s/g, "").replace(/^calc\((.*)\)$/, "$1");
            const mm = tt.match(/^([+-]?[\d.]+)(%|deg|grad|turn|px|em|rem)?$/);
            if (!mm) return null;
            const n = parseFloat(mm[1]);
            if (n === 0) return 0;
            const full = { "%": 100, deg: 360, grad: 400, turn: 1 }[mm[2] || ""];
            return full ? n / full : null;
          };
          let repeats = true;
          if (stops.length >= fi + 2) {
            let sized = false;
            if (m[1].toLowerCase() === "radial" && fi === 1) sized = tokens(stops[0]).some((x, i, arr) => !["circle", "ellipse", "farthest-corner", "at"].includes(x) && !(arr.indexOf("at") >= 0 && i > arr.indexOf("at")));
            if (!sized) {
              const first = tokens(stops[fi]), last = tokens(stops[stops.length - 1]);
              const startsZero = first.length < 2 || pos(first[first.length - 1]) === 0;
              const endsFull = last.length === 1 || pos(last[last.length - 1]) === 1;
              repeats = !(startsZero && endsFull);
            }
          }
          if (repeats && once("stripes")) {
            emit("repeating-stripes-gradient", "repeating-gradient decorative stripes " + classSel(el).slice(0, 60));
            break;
          }
        }
        // codex-grid-background (css_scan.rs#scan_css_text_for_grid_background, computed)
        const norm = img.replace(/rgba\(\s*0,\s*0,\s*0,\s*0\)/g, "transparent");
        const hairlines = (norm.match(/\b\d{1,3}px\s*,\s*transparent\s+\d{1,3}px/gi) || []).length + (norm.match(/transparent\s+calc\(100%\s*-\s*\d{1,3}px\)/gi) || []).length;
        if (hairlines >= 2 && /\b\d{1,3}px\b/.test(s.backgroundSize || "") && once("grid")) emit("codex-grid-background", "two-axis grid-line gradient background " + classSel(el).slice(0, 60));
      }

      // ---- gpt-thin-border-wide-shadow: collect the hairline + halo pair --------------------------
      {
        const halo = shadowLayers(s.boxShadow).filter((l) => !l.inset && l.alpha >= 0.12 && l.blur + Math.min(l.spread, 0) >= 32);
        if (halo.length) {
          const thin = ["Top", "Right", "Bottom", "Left"].map((k) => [num(s["border" + k + "Width"]), parseColor(s["border" + k + "Color"])]).filter(([w, c]) => w > 0 && w <= 1.5 && c && c.a >= 0.28);
          if (thin.length >= 2) gptPairs.set(el, [Math.max(...thin.map((x) => x[0])), Math.max(...halo.map((l) => l.blur))]);
        }
      }

      // ---- quality.rs#check_quality ---------------------------------------------------------------
      if (el.closest("svg") || /^(claude-|cic-)/.test(el.id || "")) return;
      quality(el, t, s, r);
    });
  }
  if (tells.size >= 2) {
    for (const ink of aiInk) {
      if (allow.palettes.includes(ink.tell)) suppress("ai-color-palette", "palette:" + ink.tell, ink.detail);
      else emit("ai-color-palette", ink.detail);
    }
  }

  run("gpt-thin-border-wide-shadow", () => {
    const close = (x, y) => {
      const m = Math.max(Math.abs(x), Math.abs(y));
      return m > 0 && Math.abs(x - y) / m <= 0.25;
    };
    const reported = new Set();
    for (const [el, pair] of gptPairs) {
      const r = rect(el);
      // A row: siblings (climbing at most two single wrappers) that carry the same pair at a similar size
      let node = el;
      for (let depth = 0; depth <= 2 && node && node.parentElement; depth++) {
        const sibs = Array.from(node.parentElement.children);
        const matching = sibs.filter((sib) => {
          const cands = [sib, ...Array.from(sib.querySelectorAll("*")).slice(0, 32)];
          return cands.some((c) => gptPairs.has(c) && close(rect(c).width, r.width) && close(rect(c).height, r.height));
        });
        if (matching.length >= 3) {
          if (!reported.has(node.parentElement)) {
            reported.add(node.parentElement);
            emit("gpt-thin-border-wide-shadow", pair[0] + "px border + " + pair[1] + "px shadow blur, repeated across the row (" + matching.length + " boxes in " + classSel(node.parentElement).slice(0, 50) + ")");
          }
          break;
        }
        node = node.parentElement;
      }
    }
  });

  // =================================================================================================
  // Page patterns: shape-assembled-illustration, monotonous-spacing, edge-flush-cards,
  // repeated-container-text, and the copy rules
  // =================================================================================================
  run("shape-assembled-illustration", () => {
    for (const svg of D.body.querySelectorAll("svg")) {
      if (svg.parentElement && svg.parentElement.closest("svg")) continue;
      const block = svg.outerHTML;
      const open = (block.match(/^<svg\b[^>]*>/i) || [""])[0];
      if ((block.match(/<(?:text|tspan)\b/gi) || []).length > 2) continue;
      if (/<pattern\b/i.test(block)) continue;
      const primitives = (block.match(/<(?:rect|circle|ellipse|polygon)\b/gi) || []).length;
      if (primitives < 8) continue;
      const vb = open.match(/\bviewBox\s*=\s*["']\s*[-0-9.]+[\s,]+[-0-9.]+[\s,]+([0-9.]+)[\s,]+([0-9.]+)\s*["']/i);
      const dim = (name) => {
        const re = new RegExp(name + "\\s*=\\s*[\"']\\s*([0-9.]+)(?:px)?\\s*[\"']", "gi");
        let m;
        while ((m = re.exec(open))) {
          const prev = open[m.index - 1];
          if (prev && /[-\w]/.test(prev)) continue;
          return parseFloat(m[1]);
        }
        return null;
      };
      let w = dim("width"), h = dim("height");
      if (w === null && vb) w = parseFloat(vb[1]);
      if (h === null && vb) h = parseFloat(vb[2]);
      if (w === null || h === null || w < 200 || h < 200) continue;
      const fills = new Set();
      for (const m of block.matchAll(/\bfill\s*[:=]\s*["']?\s*([^"';>}\s]+)/gi)) {
        const paint = m[1].trim().toLowerCase();
        if (!paint || ["none", "transparent", "currentcolor", "inherit"].includes(paint)) continue;
        fills.add(paint);
      }
      if (fills.size < 3) continue;
      emit("shape-assembled-illustration", "inline <svg> scene: " + primitives + " primitive shapes, ~" + Math.round(w) + "x" + Math.round(h) + "px, " + fills.size + " fill colors");
    }
  });

  run("monotonous-spacing", () => {
    // Layoza: upstream scans authored CSS text; here the CSSOM rule text plus inline style
    // attributes and class lists stand in for it.
    const styleText = authoredCss.concat(Array.from(D.querySelectorAll("[style]")).map((e) => "style=\"" + e.getAttribute("style") + "\"")).join("\n");
    const classText = Array.from(D.querySelectorAll("[class]")).map((e) => e.getAttribute("class")).join("\n");
    const vals = [];
    for (const m of styleText.matchAll(/(?:padding|margin)(?:-(?:top|right|bottom|left))?\s*:\s*(\d+)px/gi)) {
      const v = parseInt(m[1], 10);
      if (v > 0 && v < 200) vals.push(v);
    }
    for (const m of styleText.matchAll(/gap\s*:\s*(\d+)px/gi)) vals.push(parseInt(m[1], 10));
    for (const m of classText.matchAll(/\b(?:p|px|py|pt|pb|pl|pr|m|mx|my|mt|mb|ml|mr|gap)-(\d+)\b/g)) vals.push(parseInt(m[1], 10) * 4);
    for (const m of styleText.matchAll(/(?:padding|margin)(?:-(?:top|right|bottom|left))?\s*:\s*([0-9.]+)rem/gi)) {
      const v = Math.round(parseFloat(m[1]) * 16);
      if (v > 0 && v < 200) vals.push(v);
    }
    const rounded = vals.map((v) => Math.round(v / 4) * 4);
    if (rounded.length < 10) return;
    const counts = new Map();
    for (const v of rounded) counts.set(v, (counts.get(v) || 0) + 1);
    const [dominant, max] = Array.from(counts.entries()).sort((a, b) => b[1] - a[1])[0];
    const pct = max / rounded.length;
    const unique = Array.from(new Set(rounded)).filter((v) => v > 0);
    if (pct > 0.6 && unique.length <= 3) emit("monotonous-spacing", "~" + dominant + "px used " + max + "/" + rounded.length + " times (" + Math.round(pct * 100) + "%)");
  });

  run("edge-flush-cards", () => {
    for (const sc of all) {
      if (!/auto|scroll/.test(cs(sc).overflowX)) continue;
      if (sc.scrollWidth <= sc.clientWidth + 8 || sc.scrollLeft > 4) continue;
      const sr = rect(sc);
      if (sr.width < 120 || sr.height < 60 || sr.top + (window.scrollY || 0) > 2 * (vh || 800)) continue;
      const cl = sr.left + sc.clientLeft, cr = cl + sc.clientWidth;
      const flush = [];
      let firstRight = Infinity, lastLeft = -Infinity;
      for (const card of sc.querySelectorAll("*")) {
        if (!visible(card)) continue;
        let owner = card.parentElement;
        while (owner && owner !== sc && !/auto|scroll/.test(cs(owner).overflowX)) owner = owner.parentElement;
        if (owner !== sc) continue;
        const cr2 = rect(card);
        if (cr2.width < 80 || cr2.height < 40) continue;
        if (/^(td|th|tr|tbody|thead|tfoot|caption)$/.test(tagOf(card)) || /^table-/.test(cs(card).display)) continue;
        const bg = parseColor(cs(card).backgroundColor);
        const borders = ["Top", "Right", "Bottom", "Left"].filter((k) => num(cs(card)["border" + k + "Width"]) > 0).length;
        if (!(bg && bg.a > 0.5) && borders < 2) continue;
        firstRight = Math.min(firstRight, cr2.right);
        lastLeft = Math.max(lastLeft, cr2.left);
        const gutter = cr2.left - cl, gap = cr - cr2.right;
        const fr = gutter >= 6 && gap < 8 && gap > -24, fl = gap >= 6 && gutter < 8 && gutter > -24;
        if (fr || fl) flush.push({ card, edge: fr ? "right" : "left", gap: Math.round(fr ? gap : gutter) });
      }
      if (!flush.length || firstRight > lastLeft + 1) continue;
      const worst = flush.reduce((a, b) => (b.gap < a.gap ? b : a));
      emit("edge-flush-cards", flush.length + " card" + (flush.length === 1 ? "" : "s") + " flush against the " + worst.edge + " edge of " + classSel(sc).slice(0, 50) + " at rest (" + worst.gap + "px gap, e.g. " + classSel(worst.card).slice(0, 40) + ")");
    }
  });

  run("repeated-container-text", () => {
    const SKIP = 'table,select,datalist,nav,menu,[role="navigation"],[role="menu"],[role="menubar"],[role="listbox"],[role="grid"],[role="tablist"],[role="radiogroup"],[aria-hidden="true"]';
    const TAGS = ["div", "section", "article", "aside", "main", "figure", "form", "fieldset", "details", "li"];
    const containers = all.filter((el) => {
      if (!TAGS.includes(tagOf(el)) || closest(el, SKIP)) return false;
      const s = cs(el);
      const shadow = s.boxShadow && s.boxShadow !== "none";
      const border = ["Top", "Right", "Bottom", "Left"].filter((k) => num(s["border" + k + "Width"]) >= 1).length >= 3;
      const radius = num(s.borderRadius) > 0;
      const bg = parseColor(s.backgroundColor);
      return (shadow || border) && (radius || (bg && bg.a > 0.1));
    });
    const cset = new Set(containers);
    for (const container of containers) {
      if (!visible(container)) continue;
      const desc = Array.from(container.querySelectorAll("*"));
      if (desc.length > 250) continue;
      const groups = new Map();
      for (const d of desc) {
        let inner = false;
        for (let a = d.parentElement; a && a !== container; a = a.parentElement) if (cset.has(a)) inner = true;
        if (inner || closest(d, SKIP) || /icon|material-symbols|(?:^|\s)fa[srlbd]?(?:\s|-|$)/i.test(d.getAttribute("class") || "") || !visible(d)) continue;
        const text = cleanInlineText(d);
        if (text.length < 4 || text.length > 48 || !/[a-zA-Z]/.test(text)) continue;
        const sig = [];
        for (let c = d; c && c !== container; c = c.parentElement) {
          const cls = (c.getAttribute("class") || "").trim().split(/\s+/).filter((x) => x && !x.split(/[-_]/).some((p) => p.length >= 4 && /^[0-9a-f]+$/i.test(p) && /\d/.test(p))).sort().join(".");
          sig.push(tagOf(c) + (cls ? "." + cls : ""));
        }
        if (!groups.has(text)) groups.set(text, []);
        groups.get(text).push(sig.join(">"));
      }
      for (const [text, sigs] of groups) {
        if (sigs.length < 3 || new Set(sigs).size < 3) continue;
        emit("repeated-container-text", '"' + text.slice(0, 40) + '" rendered ' + sigs.length + "× in distinct spots inside " + classSel(container).slice(0, 60));
      }
    }
  });

  // =================================================================================================
  // heading-rhythm (page_checks.rs#check_heading_rhythm_dom and its rhythm_* helpers)
  //
  // A heading binds to the content it introduces, so the space above it should exceed the space
  // below it. Core port: thresholds (2-violation page minimum, 200px small-card exemption, 6..160px
  // band for the gap below, 12px minimum deficit, 0.75 crowding ratio, 0.5px even slack, the
  // running-prose test) and the walks (eyebrow fold, edge above / below through display:contents,
  // spacers and unpainted wrappers, landmark stops, separating rules / pictures) follow upstream.
  // Layoza: simplifications, all erring toward silence:
  //  - shadow-DOM content inside a would-be spacer is not inspected (a light-DOM-empty custom
  //    element with a shadow tree is treated as content, never as a spacer);
  //  - the "unpainted at rest" gate is approximated by visible() + effective opacity + the heading
  //    intersecting the viewport horizontally (upstream runs its full paint gate);
  //  - background-image banding reads computed background-repeat / background-size instead of
  //    re-tokenising the "background" shorthand;
  //  - rhythm_repeats compares tag, Dice similarity of the tag outline (>= 0.7) and that the
  //    neighbour holds a heading of the same level, without upstream's child-path / class-chain match;
  //  - running-prose line counts come from Range client rects grouped by line top.
  // =================================================================================================
  run("heading-rhythm", () => {
    const MIN_VIOLATIONS = 2;
    const CARD_EXEMPT_HEIGHT = 200;
    const MAX_BELOW_PX = 160;
    const MIN_DEFICIT_PX = 12;
    const EVEN_SLACK_PX = 0.5;
    const MEDIA = ["img", "picture", "video", "canvas", "svg", "iframe"];
    const CONTROLS = ["input", "textarea", "select", "button", "meter", "progress"];
    const HTAGS = ["h1", "h2", "h3", "h4", "h5", "h6"];
    const body = D.body;
    const fsz = (el) => num(cs(el).fontSize) || 16;
    const trimText = (el) => collapse(el.textContent);
    const hasWords = (el) => directTextNodes(el).some((t) => t.trim());
    const isContents = (el) => cs(el).display === "contents";
    const overlapsX = (a, b) => Math.min(a.right, b.right) - Math.max(a.left, b.left) >= 8;
    const shadowPaints = (v) => shadowLayers(v).some((l) => l.alpha >= FAINT_PAINT_ALPHA);
    function visibleFlow(el) {
      const s = cs(el);
      if (s.display === "none" || s.visibility === "hidden") return false;
      if (num(s.opacity === "" ? "1" : s.opacity) <= 0.05) return false;
      if (s.position === "absolute" || s.position === "fixed" || s.position === "sticky") return false;
      const r = rect(el);
      return r.width >= 1 && r.height >= 1;
    }
    function imageBand(el) {
      const s = cs(el);
      const img = s.backgroundImage;
      if (!img || img === "none") return false;
      if (/text/.test((s.backgroundClip || "") + " " + (s.webkitBackgroundClip || ""))) return false;
      const images = splitTop(img, ",");
      const sizes = splitTop(s.backgroundSize || "auto", ",");
      const repeats = splitTop(s.backgroundRepeat || "repeat", ",");
      return images.some((im, i) => {
        if (im === "none") return false;
        const size = (sizes[i] || sizes[sizes.length - 1] || "").trim();
        if (size === "cover") return true;
        if (/gradient\(/.test(im) && ["auto", "auto auto", "100% 100%"].includes(size)) return true;
        const rep = (repeats[i] || repeats[repeats.length - 1] || "").trim().split(/\s+/);
        const tiles = (w) => w === "repeat" || w === "space" || w === "round";
        return rep.length === 1 ? tiles(rep[0]) : tiles(rep[0]) && tiles(rep[1]);
      });
    }
    function paintsEdge(el, side) {
      if (isContents(el)) return false;
      const bg = parseColor(cs(el).backgroundColor);
      if (bg && bg.a > 0.05) return true;
      if (imageBand(el)) return true;
      if (num(cs(el)["border" + side + "Width"]) > 0) return true;
      return shadowPaints(cs(el).boxShadow);
    }
    function renderedChildren(el) {
      return Array.from(el.children).filter((k) => {
        const s = cs(k);
        if (s.display === "none" || s.position === "absolute" || s.position === "fixed") return false;
        const r = rect(k);
        return r.width >= 1 && r.height >= 1;
      });
    }
    function emptyBox(el, depth) {
      const t = tagOf(el);
      if (paintsEdge(el, "Bottom") || MEDIA.includes(t) || CONTROLS.includes(t)) return false;
      if (t === "hr" || num(cs(el).borderTopWidth) > 0) return false;
      if (el.shadowRoot) return false; // Layoza: shadow trees count as content
      const kids = renderedChildren(el);
      return kids.length === 0 || (depth > 0 && kids.every((k) => emptyBox(k, depth - 1)));
    }
    const isSpacer = (el) => !trimText(el) && emptyBox(el, 4);
    function flowBox(s, r, fromEnd, pick) {
      if (isContents(s)) {
        const kids = Array.from(s.children);
        if (fromEnd) kids.reverse();
        for (const k of kids) {
          const b = flowBox(k, r, fromEnd, pick);
          if (b && !isSpacer(b)) return b;
        }
        return null;
      }
      if (!visibleFlow(s)) return null;
      const sr = rect(s);
      return pick(sr) && overlapsX(sr, r) ? s : null;
    }
    function frames(el) {
      return ["borderTopWidth", "borderLeftWidth", "borderRightWidth"].some((k) => num(cs(el)[k]) > 0);
    }
    function blockSeparates(el) {
      const er = rect(el);
      if (tagOf(el) === "hr" || er.height <= 4) return true;
      if (!trimText(el) && el.querySelector("img, picture, video, canvas, svg, iframe")) return true;
      let cur = el;
      let framed = false;
      for (let i = 0; i < 8 && cur; i++) {
        const cr = rect(cur);
        framed = framed || frames(cur);
        if (!framed && num(cs(cur).borderBottomWidth) > 0 && cr.width >= er.width * 0.9) return true;
        const t = tagOf(cur);
        if (MEDIA.includes(t) && cr.width >= er.width * 0.5) return true;
        if (HTAGS.includes(t)) return true;
        cur = Array.from(cur.children).reverse().find((k) => {
          if (cs(k).display === "none") return false;
          const kr = rect(k);
          return kr.width >= 1 && kr.height >= 1 && kr.bottom >= er.bottom - 2;
        });
      }
      return false;
    }
    const isSideLandmark = (el) => ["footer", "nav", "aside", "header"].includes(tagOf(el)) || /\b(contentinfo|navigation|complementary|banner)\b/i.test(el.getAttribute("role") || "");
    const isMainLandmark = (el) => tagOf(el) === "main" || /\bmain\b/i.test(el.getAttribute("role") || "");
    function lowestContent(el) {
      let cur = el;
      for (let i = 0; i < 8; i++) {
        const r = rect(cur);
        if (paintsEdge(cur, "Bottom") || MEDIA.includes(tagOf(cur))) return r.bottom;
        const kids = renderedChildren(cur).filter((k) => !isSpacer(k));
        if (!kids.length) return r.bottom - Math.max(0, num(cs(cur).paddingBottom));
        cur = kids.reduce((a, b) => (rect(b).bottom > rect(a).bottom ? b : a));
      }
      return rect(cur).bottom;
    }
    const contentBottom = (el) => Math.min(lowestContent(el), rect(el).bottom);
    function subtree(el, limit) {
      const out = [];
      const stack = [el];
      while (stack.length && out.length < limit) {
        const c = stack.pop();
        out.push(c);
        stack.push(...Array.from(c.children).reverse());
      }
      return out;
    }
    const textSize = (el) => fsz(subtree(el, 60).find(hasWords) || el);
    function coloursDiffer(a, b) {
      const ca = parseColor(cs(a).color), cb = parseColor(cs(b).color);
      if (!ca || !cb) return false;
      return Math.abs(ca.r - cb.r) >= 32 || Math.abs(ca.g - cb.g) >= 32 || Math.abs(ca.b - cb.b) >= 32 || Math.abs(ca.a - cb.a) >= 0.25;
    }
    const italic = (el) => /^(italic|oblique)/.test(cs(el).fontStyle || "");
    const weightOf = (el) => {
      const w = cs(el).fontWeight;
      return w === "bold" ? 700 : w === "normal" ? 400 : num(w) || 400;
    };
    function setApart(line, heading, words) {
      const container = line.parentElement;
      if (!container || trimText(line).length > 40) return false;
      const size = fsz(words);
      const lh = resolveLen(cs(words).lineHeight, size);
      const pitch = lh && lh > 0 ? lh : size * 1.2;
      const tr = directTextRect(words);
      if (!(tr && tr.height > 0 && tr.height < Math.max(pitch, size * 1.2) * 1.5)) return false;
      if (coloursDiffer(words, container) && coloursDiffer(words, heading)) return true;
      if (italic(words) && !italic(container) && !italic(heading)) return true;
      return weightOf(words) <= weightOf(container) - 300;
    }
    function readsAsEyebrow(line, heading, tSize, opensGroup) {
      const hSize = fsz(heading);
      const span = Math.max(rect(heading).width, rect(line).width);
      for (const e of subtree(line, 40)) {
        const er = rect(e);
        if (paintsEdge(e, "Bottom") && er.width >= 1 && er.width < span * 0.6 && trimText(e)) return true;
        if (!hasWords(e)) continue;
        const size = fsz(e);
        if (size > hSize) continue;
        if (size < tSize * 0.92 || cs(e).textTransform === "uppercase") return true;
        const tracking = parseFloat(cs(e).letterSpacing);
        if (Number.isFinite(tracking) && tracking >= 0.5) return true;
        const cased = directText(e).split("").filter((c) => c.toLowerCase() !== c.toUpperCase());
        if (cased.length >= 3 && cased.every((c) => c === c.toUpperCase())) return true;
        if (opensGroup && setApart(line, heading, e)) return true;
      }
      return false;
    }
    const painted = (el) => {
      const c = parseColor(cs(el).backgroundColor);
      return c && c.a > 0.05 ? c : null;
    };
    function bandDiffers(el) {
      const band = painted(el);
      if (!band) return false;
      let backdrop = CANVAS();
      for (let c = el.parentElement; c; c = c.parentElement) {
        const bg = painted(c);
        if (bg) {
          backdrop = bg;
          break;
        }
      }
      return Math.abs(band.r - backdrop.r) > 2 || Math.abs(band.g - backdrop.g) > 2 || Math.abs(band.b - backdrop.b) > 2 || Math.abs(band.a - backdrop.a) > 0.02;
    }
    function drawsBottomEdge(el) {
      if (isContents(el)) return false;
      if (num(cs(el).borderBottomWidth) > 0) return true;
      if (shadowPaints(cs(el).boxShadow)) return true;
      return bandDiffers(el);
    }
    function drawsTopEdge(el) {
      if (isContents(el)) return false;
      if (num(cs(el).borderTopWidth) > 0 || imageBand(el)) return true;
      const bs = cs(el).boxShadow;
      if (bs && bs !== "none") return true;
      return bandDiffers(el);
    }
    function shapeTokens(el) {
      const out = [];
      let budget = 24;
      const walk = (e, depth) => {
        out.push(tagOf(e));
        if (depth === 0) return;
        for (const k of renderedChildren(e)) {
          if (budget-- <= 0) break;
          walk(k, depth - 1);
        }
      };
      walk(el, 3);
      return out;
    }
    function dice(a, b) {
      if (!a.length && !b.length) return 1;
      const rest = b.slice();
      let shared = 0;
      for (const t of a) {
        const i = rest.indexOf(t);
        if (i >= 0) {
          rest.splice(i, 1);
          shared++;
        }
      }
      return (2 * shared) / (a.length + b.length);
    }
    function repeats(el, h) {
      const t = tagOf(el);
      const level = tagOf(h);
      const shape = shapeTokens(el);
      const laysOut = (s) => {
        const st = cs(s);
        const r = rect(s);
        return st.display !== "none" && st.position !== "absolute" && st.position !== "fixed" && r.width >= 1 && r.height >= 1 && !isSpacer(s);
      };
      const alike = (s) => tagOf(s) === t && Boolean(s.querySelector(level)) && dice(shape, shapeTokens(s)) >= 0.7;
      for (let s = el.previousElementSibling; s; s = s.previousElementSibling) {
        if (laysOut(s)) {
          if (alike(s)) return true;
          break;
        }
      }
      for (let s = el.nextElementSibling; s; s = s.nextElementSibling) {
        if (laysOut(s)) return alike(s);
      }
      return false;
    }
    function closingText(block) {
      let cur = block;
      for (let i = 0; i < 12 && cur; i++) {
        if (MEDIA.includes(tagOf(cur)) || drawsBottomEdge(cur)) return null;
        if (hasWords(cur)) return cur;
        const kids = renderedChildren(cur).filter((k) => !isSpacer(k));
        if (!kids.length) return null;
        cur = kids.reduce((a, b) => (rect(b).bottom > rect(a).bottom ? b : a));
      }
      return null;
    }
    function lineCountOf(el) {
      const range = D.createRange();
      range.selectNodeContents(el);
      const tops = new Set();
      for (const r of Array.from(range.getClientRects())) if (r.width > 0 && r.height > 0) tops.add(Math.round(r.top));
      return tops.size;
    }
    function endsInProse(block, heading, tSize) {
      const t = closingText(block);
      if (!t || HTAGS.includes(tagOf(t))) return false;
      const size = fsz(t);
      if (size >= fsz(heading) || size > tSize * 1.25) return false;
      if (trimText(t).length < 80) return false;
      if (rect(t).width < rect(heading).width * 0.5) return false;
      return lineCountOf(t) >= 2;
    }
    function previousBox(n) {
      for (let sib = n.previousElementSibling; sib; sib = sib.previousElementSibling) {
        if (isContents(sib)) {
          const inner = Array.from(sib.children).reverse().find((k) => {
            const r = rect(k);
            return r.width >= 1 && r.height >= 1;
          });
          if (inner) return inner;
        } else {
          const r = rect(sib);
          if (cs(sib).display !== "none" && r.width >= 1 && r.height >= 1) return sib;
        }
      }
      return null;
    }
    function clusterTop(h, r, tSize) {
      let topEl = h, top = r.top, cursor = h, folded = 0;
      while (folded < 3) {
        const sib = previousBox(cursor);
        if (!sib) {
          const p = cursor.parentElement;
          if (!p || p === body) break;
          const starts = isContents(p) || (Math.abs(rect(p).top - top) <= 1 && !paintsEdge(p, "Top"));
          if (!starts) break;
          cursor = p;
          continue;
        }
        if (isSpacer(sib)) {
          cursor = sib;
          continue;
        }
        if (!visibleFlow(sib)) break;
        const sr = rect(sib);
        if (!overlapsX(sr, r)) break;
        const gap = top - sr.bottom;
        if (gap < 0 || gap >= 28 || sr.height > 60) break;
        const text = trimText(sib);
        if (!text.length || HTAGS.includes(tagOf(sib))) break;
        const opensGroup = !previousBox(sib);
        if (text.length > 80 || !readsAsEyebrow(sib, h, tSize, opensGroup)) break;
        topEl = sib;
        top = sr.top;
        cursor = sib;
        folded++;
      }
      return { topEl, top };
    }
    function edgeAbove(h, startEl, top, r) {
      const pick = (sr) => sr.bottom <= top + 2;
      const inset = paintsEdge(startEl, "Top") ? 0 : Math.max(0, num(cs(startEl).paddingTop));
      for (let n = startEl; n && n !== body; ) {
        let nearest = null;
        for (let s = n.previousElementSibling; s; s = s.previousElementSibling) {
          const b = flowBox(s, r, true, pick);
          if (b && !isSpacer(b)) {
            const bottom = contentBottom(b);
            if (!nearest || bottom > nearest.bottom) nearest = { bottom, el: b };
          }
        }
        if (nearest) return { bottom: nearest.bottom - inset, el: nearest.el };
        const p = n.parentElement;
        if (!p || p === body) return null;
        if (drawsTopEdge(p) || (paintsEdge(p, "Top") && repeats(p, h))) return null;
        n = p;
      }
      return null;
    }
    function edgeBelow(h, r) {
      const pick = (sr) => sr.top >= r.bottom - 2;
      for (let n = h; n && n !== body; ) {
        let nearest = null;
        for (let s = n.nextElementSibling; s; s = s.nextElementSibling) {
          const b = flowBox(s, r, false, pick);
          if (b && !isSpacer(b)) {
            const t = rect(b).top;
            if (!nearest || t < nearest.top) nearest = { top: t, el: b };
          }
        }
        if (nearest) {
          if (n !== h && isSideLandmark(nearest.el)) return null;
          return nearest;
        }
        const p = n.parentElement;
        if (!p) return null;
        if (p !== body && !isContents(p) && (drawsBottomEdge(p) || repeats(p, h))) return null;
        if (isMainLandmark(p)) return null;
        n = p;
      }
      return null;
    }
    function insideSmallCard(h) {
      for (let c = h.parentElement; c && c !== body; c = c.parentElement) {
        if (isCardLike(c) && rect(c).height < CARD_EXEMPT_HEIGHT) return true;
      }
      return false;
    }

    const cands = [];
    for (const h of Array.from(D.querySelectorAll("h2, h3, h4"))) {
      if (!visibleFlow(h) || !visible(h) || effectiveOpacity(h) <= 0.05) continue;
      const r = rect(h);
      if (r.right <= 0 || r.left >= vw) continue; // Layoza: approximates upstream's paint gate
      const text = trimText(h);
      if (text.length < 3) continue;
      if (paintsEdge(h, "Top")) continue;
      const below = edgeBelow(h, r);
      if (!below) continue;
      const tSize = Math.max(fsz(body), textSize(below.el));
      const ct = clusterTop(h, r, tSize);
      const above = edgeAbove(h, ct.topEl, ct.top, r);
      if (!above) continue;
      if (insideSmallCard(h)) continue;
      const gapAbove = Math.max(0, ct.top - above.bottom);
      const gapBelow = Math.max(0, below.top - r.bottom);
      if (gapBelow < 6 || gapBelow > MAX_BELOW_PX) continue;
      const crowded = gapAbove < gapBelow * 0.75 && gapBelow - gapAbove >= MIN_DEFICIT_PX;
      const evenUnderProse = !crowded && gapAbove <= gapBelow + EVEN_SLACK_PX && endsInProse(above.el, h, tSize);
      if ((crowded || evenUnderProse) && !blockSeparates(above.el)) {
        cands.push({ tag: tagOf(h), text: text.slice(0, 60), above: gapAbove, below: gapBelow });
      }
    }
    if (cands.length < MIN_VIOLATIONS) return;
    for (const c of cands) {
      emit("heading-rhythm", c.tag + ' "' + c.text + '" has ' + Math.round(c.above) + "px above vs " + Math.round(c.below) + "px below; it reads as bound to the block above (" + cands.length + " headings on page)");
    }
  });

  run("copy", () => {
    // em-dash-overuse reads innerText (text_collectors.rs#check_em_dash_overuse_dom), less the
    // lines a rendered <select> writes and table/grid cells that hold only a dash.
    const raw = D.body.innerText || D.body.textContent || "";
    const choices = Array.from(D.querySelectorAll("select option")).map((o) => collapse(o.textContent)).filter(Boolean);
    const lines = raw.split("\n").filter((line) => {
      const k = collapse(line);
      const at = choices.indexOf(k);
      if (at >= 0) {
        choices.splice(at, 1);
        return false;
      }
      return true;
    });
    const text = lines.join("\n").split(/([\n\t])/).filter((seg) => !/^[—–-]+$/.test(seg.trim())).join("");
    const body = text.replace(/\s+/g, " ");
    const dashes = (body.match(/—|--(?=\S)/g) || []).length;
    if (dashes >= EM_DASH_FLOOR && body.length <= dashes * EM_DASH_CHARS_PER_DASH) emit("em-dash-overuse", dashes + " em-dashes in body text");
    // marketing-buzzword / aphoristic-cadence / theater-slop-phrase (detect regex_matchers.rs,
    // html_patterns.rs) on the collapsed page text
    const lower = body.toLowerCase();
    const BUZZ = ["streamline your", "empower your", "supercharge your", "unleash your", "unleash the power", "leverage the power", "built for the modern", "trusted by leading", "trusted by the world", "best-in-class", "industry-leading", "world-class", "enterprise-grade", "next-generation", "cutting-edge", "transform your business", "revolutionize", "game-changer", "game changing", "mission-critical", "best of breed", "future-proof", "future proof", "seamless experience", "seamlessly integrate", "drive engagement", "drive growth", "drive results", "harness the power"];
    let buzz = 0, sample = "";
    for (const phrase of BUZZ) {
      for (let i = lower.indexOf(phrase); i >= 0; i = lower.indexOf(phrase, i + phrase.length)) {
        buzz++;
        if (!sample) sample = body.slice(Math.max(0, i - 12), i + phrase.length + 12).trim();
      }
    }
    if (buzz) emit("marketing-buzzword", buzz + " buzzword phrase" + (buzz === 1 ? "" : "s") + ': "' + sample + '"');
    const aph = (body.match(/\bNot an? [a-z][^.!?]{1,40}[.!]\s+[A-Z][^.!?]{1,60}[.!]/g) || []).concat(body.match(/\b[A-Z][^.!?]{4,80}[.!]\s+(No|Just)\s+[a-z][^.!?]{2,60}[.!]/g) || []);
    if (aph.length >= 3) emit("aphoristic-cadence", aph.length + ' aphoristic constructions: "' + aph[0].trim().slice(0, 80) + '"');
    const HEADS = ["accessibility", "accountability", "agile", "alignment", "compliance", "diversity", "engagement", "governance", "growth", "hiring", "hygiene", "innovation", "inclusion", "kindness", "leadership", "metrics", "privacy", "process", "productivity", "quality", "safety", "security", "strategy", "sustainability", "transparency", "trust", "wellness"];
    for (const m of body.matchAll(/\b(\w+)\s+theater\b/gi)) {
      if (HEADS.includes(m[1].toLowerCase())) {
        emit("theater-slop-phrase", '"' + m[0].trim() + '"');
        break;
      }
    }
  });

  return {
    issues,
    suppressed,
    errors,
    brand_allow: allow.fonts.length || allow.palettes.length || allow.rules.length ? allow : null,
  };
}
const CRAFT_EVAL_SCRIPT = "(" + layozaCraftCritique.toString() + ")()";

/*
 * ---------------------------------------------------------------------------------------------
 * Generic-default tells (Layoza-original code)
 *
 * Ideas from the "calibration" list in anthropics/skills frontend-design (Apache-2.0): the traits
 * generated pages share regardless of subject. No upstream code exists (it is prose guidance); these
 * are Layoza's own rendered-DOM detectors. They complement, not duplicate, the ported impeccable
 * rules above (kicker-above-heading, numbered-section-labels, cream-palette, gradient-text, ...).
 * Each fires only on a repeated pattern or a clear single instance, emits rule `craft-<id>` with
 * severity "critique" (non-blocking), and honours <meta name="layoza-brand-allow" content="rule:<id>">.
 * ---------------------------------------------------------------------------------------------
 */
function layozaGenericTells() {
  "use strict";
  const D = document;
  const vh = window.innerHeight || 0;
  const issues = [];
  const suppressed = [];
  const errors = [];
  if (!D.body) return { issues, suppressed, errors };
  const allowRules = [];
  D.querySelectorAll('meta[name="layoza-brand-allow" i]').forEach((m) =>
    (m.getAttribute("content") || "").split(/[,;]/).forEach((p) => {
      const i = p.indexOf(":");
      if (i > 0 && p.slice(0, i).trim().toLowerCase() === "rule") allowRules.push(p.slice(i + 1).trim().toLowerCase().replace(/^craft-/, ""));
    })
  );
  // Product UI (dashboard, admin, tool) is judged on task clarity, not on landing-page hero rules.
  // Declared with <meta name="layoza-surface" content="app">, or detected: a data table/grid and a
  // navigation rail or tab list both in the first viewport.
  const LANDING_ONLY = ["hero-metric", "hero-art-scale", "hero-art-mobile", "display-scale"];
  const appSurface = (() => {
    if (document.querySelector('meta[name="layoza-surface"][content="app" i]')) return true;
    const inFold = (el) => { const r = el.getBoundingClientRect(); return r.width > 0 && r.height > 0 && r.top < window.innerHeight && r.bottom > 0; };
    const data = Array.from(document.querySelectorAll('table, [role="grid"], [role="table"]')).some(inFold);
    const rail = Array.from(document.querySelectorAll('aside nav, nav[aria-orientation="vertical"], [role="tablist"], [role="toolbar"]')).some(inFold);
    return data && rail;
  })();
  const emit = (rule, detail) => {
    const d = String(detail).replace(/\s+/g, " ").slice(0, 300);
    if (allowRules.includes(rule)) suppressed.push({ rule: "craft-" + rule, allowed_by: "rule:" + rule, detail: d });
    else if (appSurface && LANDING_ONLY.includes(rule)) suppressed.push({ rule: "craft-" + rule, allowed_by: "surface:app", detail: d });
    else issues.push({ rule: "craft-" + rule, severity: "critique", detail: d });
  };
  const run = (name, fn) => {
    try {
      fn();
    } catch (e) {
      if (errors.length < 20) errors.push(name + ": " + (e && e.message ? e.message : String(e)));
    }
  };
  const cs = (el) => getComputedStyle(el);
  const collapse = (s) => String(s || "").replace(/\s+/g, " ").trim();
  const directText = (el) => collapse(Array.from(el.childNodes).filter((n) => n.nodeType === 3).map((n) => n.nodeValue).join(" "));
  const SKIP = '[aria-hidden="true"], .sr-only, .visually-hidden, [class*="sr-only" i], script, style, template, noscript, svg';
  const CODE = 'pre, code, kbd, samp, var, [class*="code" i], [class*="terminal" i], [class*="console" i], [class*="editor" i]';
  const visible = (el) => {
    const r = el.getBoundingClientRect();
    if (r.width < 1 || r.height < 1) return false;
    const s = cs(el);
    return s.visibility !== "hidden" && s.display !== "none" && parseFloat(s.opacity) > 0.05 && !el.closest(SKIP);
  };
  const describe = (el) => {
    const cls = typeof el.className === "string" ? el.className.trim().split(/\s+/).filter(Boolean).slice(0, 2).join(".") : "";
    return el.tagName.toLowerCase() + (el.id ? "#" + el.id : "") + (cls ? "." + cls : "");
  };
  const family = (s) => (s.fontFamily.split(",")[0] || "").trim().replace(/^["']|["']$/g, "").toLowerCase();
  const isMono = (s) => {
    const f = family(s);
    return f === "monospace" || /\bmono\b|code|courier|consolas|menlo|monaco|inconsolata|ui-monospace/.test(f);
  };
  const rgb = (v) => {
    const m = /rgba?\(\s*([\d.]+)[ ,]+([\d.]+)[ ,]+([\d.]+)(?:\s*[,/]\s*([\d.]+%?))?/.exec(v || "");
    if (!m) return null;
    const a = m[4] === undefined ? 1 : m[4].endsWith("%") ? parseFloat(m[4]) / 100 : parseFloat(m[4]);
    return { r: +m[1], g: +m[2], b: +m[3], a };
  };
  const hsl = ({ r, g, b }) => {
    const R = r / 255, G = g / 255, B = b / 255;
    const max = Math.max(R, G, B), min = Math.min(R, G, B), l = (max + min) / 2, d = max - min;
    if (d === 0) return { h: 0, s: 0, l };
    const s = d / (1 - Math.abs(2 * l - 1));
    let h = max === R ? ((G - B) / d) % 6 : max === G ? (B - R) / d + 2 : (R - G) / d + 4;
    h = (h * 60 + 360) % 360;
    return { h, s, l };
  };
  const lum = ({ r, g, b }) => {
    const c = (v) => ((v /= 255) <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4));
    return 0.2126 * c(r) + 0.7152 * c(g) + 0.0722 * c(b);
  };
  const hex = ({ r, g, b }) => "#" + [r, g, b].map((v) => Math.round(v).toString(16).padStart(2, "0")).join("");
  const bodySize = parseFloat(cs(D.body).fontSize) || 16;
  const all = Array.from(D.body.querySelectorAll("*"));

  // arrow-cta: '→' appended to link and button text, repeated across the page
  run("arrow-cta", () => {
    const hits = [];
    for (const el of D.body.querySelectorAll('a[href], button, [role="button"], input[type="submit"], input[type="button"]')) {
      if (!visible(el) || el.closest('[aria-label*="pagination" i], [class*="pagination" i], [class*="carousel" i]')) continue;
      const text = collapse(el.tagName === "INPUT" ? el.value : el.textContent);
      // An arrow-only control (carousel next, back-to-top) is a control, not a label suffix
      if (!text || text.length > 60 || !/\p{L}{2,}/u.test(text)) continue;
      // ↗ on an off-site or new-tab link is an honest "leaves this site" marker, not chrome
      let external = el.target === "_blank";
      try {
        external = external || (el.href && new URL(el.href, location.href).host !== location.host);
      } catch (_) {
        /* unparsable href */
      }
      const after = getComputedStyle(el, "::after").content || "";
      const glyph = (/(→|↗|⟶|➔|➜|➝|»|->)\s*$/.exec(text) || /(→|↗|⟶|➔|➜|➝|»)/.exec(after) || [])[1];
      const arrow = Boolean(glyph) && !(glyph === "↗" && external);
      if (arrow && !hits.includes(text)) hits.push(text);
    }
    if (hits.length >= 2) emit("arrow-cta", hits.length + ' links/buttons end in an arrow glyph, e.g. "' + hits.slice(0, 3).join('", "') + '"');
  });

  // middot-meta: meta strings joined with middle dots ('A · B · C') in body content
  run("middot-meta", () => {
    const hits = [];
    const RE = /\S\s[·•]\s\S[^·•]*\s[·•]\s\S/;
    for (const el of all) {
      if (el.closest("footer, nav, [role='navigation'], [aria-label*='breadcrumb' i]")) continue;
      const t = collapse(el.textContent);
      if (t.length < 5 || t.length > 140 || !RE.test(t)) continue;
      if (Array.from(el.children).some((c) => RE.test(collapse(c.textContent)))) continue;
      if (visible(el) && !hits.includes(t)) hits.push(t);
    }
    if (hits.length >= 2) emit("middot-meta", hits.length + ' meta strings joined with middle dots, e.g. "' + hits.slice(0, 2).join('", "') + '"');
  });

  // caps-dash-label: labels built as 'WORD — fragment' with a spaced em dash
  run("caps-dash-label", () => {
    const hits = [];
    for (const el of all) {
      const own = directText(el);
      if (!own || own.length > 70 || !visible(el)) continue;
      const t = cs(el).textTransform === "uppercase" ? own.toUpperCase() : own;
      if (/^[\p{Lu}\d][\p{Lu}\d&'’.]*(?:\s[\p{Lu}\d&'’.]+){0,2}\s[—–]\s\S/u.test(t) && !hits.includes(own)) hits.push(own);
    }
    if (hits.length >= 2) emit("caps-dash-label", hits.length + ' labels shaped "WORD — fragment", e.g. "' + hits.slice(0, 2).join('", "') + '"');
  });

  // headline-accent-word: one word or short phrase in a headline set in a different colour, style or face
  run("headline-accent-word", () => {
    let n = 0;
    for (const h of D.body.querySelectorAll("h1, h2")) {
      if (n >= 4 || !visible(h)) continue;
      const total = collapse(h.textContent);
      if (total.split(" ").length < 3) continue;
      const hs = cs(h);
      const hc = rgb(hs.color);
      for (const c of h.querySelectorAll("em, i, span, strong, b, mark, u")) {
        const ct = collapse(c.textContent);
        if (!ct || ct.length / total.length >= 0.4 || ct.split(" ").length > 3 || !visible(c)) continue;
        const s = cs(c);
        if (s.display === "block" || s.display === "flex" || s.display === "grid") continue;
        const cc = rgb(s.color);
        const colour = hc && cc && Math.abs(hc.r - cc.r) + Math.abs(hc.g - cc.g) + Math.abs(hc.b - cc.b) > 60;
        const style = s.fontStyle !== hs.fontStyle;
        const face = family(s) !== family(hs);
        const weight = Math.abs((Number(s.fontWeight) || 400) - (Number(hs.fontWeight) || 400)) >= 200;
        const paint = s.backgroundImage !== "none" && s.backgroundImage !== hs.backgroundImage;
        if (colour || style || face || weight || paint) {
          const how = [colour && "colour", style && "italic", face && "typeface", weight && "weight", paint && "fill"].filter(Boolean).join("+");
          emit("headline-accent-word", h.tagName.toLowerCase() + ' "' + total.slice(0, 70) + '" accents only "' + ct.slice(0, 30) + '" (' + how + ")");
          n++;
          break;
        }
      }
    }
  });

  // dark-acid-accent: near-black page with a single acid-green or vermilion accent
  run("dark-acid-accent", () => {
    let bg = rgb(cs(D.body).backgroundColor);
    if (!bg || bg.a < 0.5) bg = rgb(cs(D.documentElement).backgroundColor);
    if (!bg || bg.a < 0.5 || lum(bg) >= 0.03) return;
    const buckets = new Map();
    for (const el of all) {
      if (!visible(el)) continue;
      const s = cs(el);
      for (const v of [s.color, s.backgroundColor]) {
        const c = rgb(v);
        if (!c || c.a < 0.5) continue;
        const x = hsl(c);
        if (x.s < 0.55 || x.l < 0.3 || x.l > 0.8) continue;
        const key = Math.round(x.h / 30) % 12;
        if (!buckets.has(key)) buckets.set(key, { h: x.h, c, n: 0 });
        buckets.get(key).n++;
      }
    }
    if (buckets.size !== 1) return;
    const only = Array.from(buckets.values())[0];
    const acid = only.h >= 60 && only.h <= 120;
    const vermilion = only.h <= 25 || only.h >= 345;
    if ((acid || vermilion) && only.n >= 3) emit("dark-acid-accent", "near-black ground " + hex(bg) + " with one " + (acid ? "acid-green" : "vermilion") + " accent " + hex(only.c) + " on " + only.n + " elements");
  });

  // uniform-radius: one border-radius on everything regardless of hierarchy
  run("uniform-radius", () => {
    const byValue = new Map();
    let totalN = 0;
    for (const el of all) {
      if (!visible(el)) continue;
      const s = cs(el);
      const r = parseFloat(s.borderTopLeftRadius);
      if (!(r >= 3)) continue;
      const box = el.getBoundingClientRect();
      if (r >= Math.min(box.width, box.height) / 2 - 1) continue; // pills and circles are shapes, not a radius scale
      const painted = (rgb(s.backgroundColor) || { a: 0 }).a > 0.05 || parseFloat(s.borderTopWidth) > 0 || s.boxShadow !== "none" || /^(img|video|picture|canvas|iframe)$/i.test(el.tagName);
      if (!painted) continue;
      const tag = el.tagName.toLowerCase();
      const role = /^(button|a)$/.test(tag) || el.getAttribute("role") === "button" ? "control" : /^(input|select|textarea)$/.test(tag) ? "field" : /^(img|video|picture|canvas|iframe)$/.test(tag) ? "media" : "surface";
      const key = Math.round(r);
      if (!byValue.has(key)) byValue.set(key, { n: 0, roles: new Set() });
      const e = byValue.get(key);
      e.n++;
      e.roles.add(role);
      totalN++;
    }
    if (totalN < 10) return;
    const [value, top] = Array.from(byValue.entries()).sort((a, b) => b[1].n - a[1].n)[0];
    if (top.n / totalN >= 0.85 && top.roles.size >= 3) emit("uniform-radius", "border-radius " + value + "px on " + top.n + " of " + totalN + " rounded elements across " + Array.from(top.roles).join(", "));
  });

  // card-kit: content chopped into identical rounded cards with the same soft shadow, section after section
  run("card-kit", () => {
    const groups = [];
    for (const p of all) {
      const kids = Array.from(p.children).filter((k) => visible(k));
      if (kids.length < 3) continue;
      const sig = kids.map((k) => {
        const s = cs(k);
        return s.boxShadow !== "none" && parseFloat(s.borderTopLeftRadius) >= 4 ? s.boxShadow + "|" + s.borderTopLeftRadius : null;
      });
      if (sig.some((x) => !x) || new Set(sig).size !== 1) continue;
      const widths = kids.map((k) => k.getBoundingClientRect().width);
      if (Math.max(...widths) > Math.min(...widths) * 1.06) continue;
      if (kids.some((k) => collapse(k.textContent).length < 20)) continue;
      groups.push(describe(p) + " (" + kids.length + " cards)");
    }
    if (groups.length >= 2) emit("card-kit", groups.length + " sections built from identical shadowed rounded cards: " + groups.slice(0, 3).join(", "));
  });

  // mono-meta-labels: a monospace face as costume for small data labels outside code
  run("mono-meta-labels", () => {
    if (isMono(cs(D.body))) return; // a page set entirely in mono has chosen it
    const hits = [];
    for (const el of all) {
      const own = directText(el);
      // Single tokens (identifiers, branch names, figures) are what mono is for
      if (!own || own.length > 60 || !/\s/.test(own) || el.closest(CODE) || !visible(el)) continue;
      const s = cs(el);
      if (parseFloat(s.fontSize) < 14 && isMono(s)) hits.push(own);
    }
    if (hits.length >= 3) emit("mono-meta-labels", hits.length + ' small labels set in a monospace face outside code, e.g. "' + hits.slice(0, 3).join('", "') + '"');
  });

  // scattered-reveal: the same fade-and-slide entrance on section after section
  run("scattered-reveal", () => {
    const frames = new Set();
    const walk = (rules) => {
      for (const r of rules) {
        if (r.type === 7 /* KEYFRAMES_RULE */) {
          const t = r.cssText;
          if (/opacity\s*:\s*0(?:\.0*)?\s*[;}]/.test(t) && /translate/.test(t)) frames.add(r.name);
        } else if (r.cssRules) walk(r.cssRules);
      }
    };
    for (const sh of D.styleSheets) {
      try {
        walk(sh.cssRules);
      } catch (_) {
        /* cross-origin sheet */
      }
    }
    const REVEAL = '[data-aos], [data-reveal], [data-animate], [data-scroll], .reveal, .fade-up, .fade-in, .animate-on-scroll, [class*="reveal" i], [class*="fade-up" i], [class*="fade-in" i], [class*="slide-up" i]';
    const sections = new Set();
    for (const el of all) {
      const s = cs(el);
      const anim = s.animationName.split(",").some((n) => frames.has(n.trim()));
      const trans = el.matches(REVEAL) && /opacity|transform|all/.test(s.transitionProperty);
      if (!anim && !trans) continue;
      const sec = el.closest("section, article, header, footer, aside") || el;
      sections.add(sec);
    }
    if (sections.size >= 3) emit("scattered-reveal", "fade/slide entrance repeated across " + sections.size + " sections; one orchestrated moment lands better");
  });

  // vague-cta: action text that does not say what happens
  run("vague-cta", () => {
    const counts = {};
    for (const el of D.body.querySelectorAll('a[href], button, [role="button"], input[type="submit"], input[type="button"]')) {
      if (!visible(el)) continue;
      const t = collapse(el.tagName === "INPUT" ? el.value : el.textContent).toLowerCase().replace(/[→↗»>.!\s]+$/, "");
      if (/^(submit|click here|here|learn more|read more|more|go|send|ok|continue)$/.test(t)) counts[t] = (counts[t] || 0) + 1;
    }
    const bad = Object.entries(counts).filter(([t, n]) => /^(submit|click here|here|go|ok)$/.test(t) || n >= 2);
    if (bad.length) emit("vague-cta", "actions named " + bad.map(([t, n]) => '"' + t + '"' + (n > 1 ? " x" + n : "")).join(", ") + "; name what happens (\"Book the slot\", \"Send enquiry\")");
  });

  // centered-prose: multi-line paragraphs centred, so every line starts somewhere new
  run("centered-prose", () => {
    let n = 0, ex = "";
    for (const p of D.body.querySelectorAll("p, li, dd, blockquote")) {
      if (!visible(p)) continue;
      const s = cs(p);
      if (s.textAlign !== "center") continue;
      const lh = s.lineHeight === "normal" ? parseFloat(s.fontSize) * 1.2 : parseFloat(s.lineHeight);
      if (lh > 0 && p.getBoundingClientRect().height >= lh * 3.5) {
        n++;
        if (!ex) ex = collapse(p.textContent).slice(0, 50);
      }
    }
    if (n >= 3) emit("centered-prose", n + ' paragraphs of 4+ lines are centre-aligned, e.g. "' + ex + '"');
  });

  // font-family-sprawl: more typefaces than roles
  run("font-family-sprawl", () => {
    const fams = new Map();
    for (const el of all) {
      if (!directText(el) || el.closest(CODE) || !visible(el)) continue;
      const f = family(cs(el));
      if (f) fams.set(f, (fams.get(f) || 0) + 1);
    }
    if (fams.size >= 4) emit("font-family-sprawl", fams.size + " typefaces render text: " + Array.from(fams.keys()).join(", ") + "; one or two families with clear roles carry a page");
  });

  // hero-metric: big-number + small-label stat block as the first-viewport treatment
  run("hero-metric", () => {
    const hits = [];
    for (const el of all) {
      const own = directText(el);
      if (!own || own.length > 10 || !visible(el)) continue;
      if (!/^[~≈<>+]?\d[\d.,]*\s*(?:%|[kKmMbB]\+?|\+|x|×|★|\/\d+|hrs?|yrs?|min)?\+?$/.test(own)) continue;
      const r = el.getBoundingClientRect();
      if (r.top + window.scrollY > vh) continue;
      if (parseFloat(cs(el).fontSize) >= bodySize * 2.2) hits.push(own);
    }
    if (hits.length >= 2) emit("hero-metric", "first viewport leads with big-number stats " + hits.slice(0, 4).join(", ") + "; open with the subject's most characteristic thing instead");
  });

  // hero-art-scale: the first desktop viewport carries no authored visual at scale, so the page opens
  // as headline + buttons with (at most) a small widget beside them. Layoza-original.
  run("hero-art-scale", () => {
    const vw = window.innerWidth || 0;
    if (vw < 1024 || window.scrollY > 4) return;
    const fold = { l: 0, t: 0, r: vw, b: vh };
    const inFold = (r) => Math.max(0, Math.min(r.right, fold.r) - Math.max(r.left, fold.l)) * Math.max(0, Math.min(r.bottom, fold.b) - Math.max(r.top, fold.t));
    let best = 0, bestEl = null;
    for (const el of all) {
      const tag = el.tagName.toLowerCase();
      const s = cs(el);
      const media = /^(svg|img|canvas|video|picture)$/.test(tag) && !el.closest("svg svg, button, a, header nav");
      const painted = /url\(/.test(s.backgroundImage) || (/gradient/.test(s.backgroundImage) && el.querySelector("svg, img, canvas") === null && el.children.length === 0);
      if (!media && !painted) continue;
      if (tag === "svg" && el.parentElement && el.parentElement.closest("svg")) continue;
      if (el.closest("form, select, input, textarea, button")) continue;
      const r = el.getBoundingClientRect();
      const st = cs(el);
      // Own visibility test: the shared visible() skips every svg on purpose (text rules)
      if (r.width < 48 || r.height < 48 || st.visibility === "hidden" || st.display === "none" || parseFloat(st.opacity) < 0.05 || el.closest('[aria-hidden="true"]:not(svg):not(svg *)') && !/^(svg|img|canvas)$/.test(tag)) continue;
      const a = inFold(r);
      if (a > best) {
        best = a;
        bestEl = el;
      }
    }
    const share = best / (vw * vh);
    if (share < 0.2) emit("hero-art-scale", "largest authored visual in the first viewport covers " + Math.round(share * 100) + "% of it" + (bestEl ? " (" + describe(bestEl) + ")" : "") + "; let the world's artwork carry the first screen, not a widget beside the headline");
  });

  // hero-art-mobile: on phones the signature art shrinks to a thumbnail or falls below the first screen.
  // Judges read the 375px first frame as the brand; the art should still own a real share of it.
  run("hero-art-mobile", () => {
    const vw = window.innerWidth || 0;
    if (vw >= 600 || window.scrollY > 4) return;
    let best = 0, bestEl = null;
    for (const el of all) {
      const tag = el.tagName.toLowerCase();
      const s = cs(el);
      const media = /^(svg|img|canvas|video|picture)$/.test(tag) && !el.closest("svg svg, button, a, header nav");
      const painted = /url\(/.test(s.backgroundImage);
      if (!media && !painted) continue;
      if (tag === "svg" && el.parentElement && el.parentElement.closest("svg")) continue;
      if (el.closest("form, select, input, textarea, button")) continue;
      const r = el.getBoundingClientRect();
      if (r.width < 48 || r.height < 48 || s.visibility === "hidden" || s.display === "none" || parseFloat(s.opacity) < 0.05) continue;
      const a = Math.max(0, Math.min(r.right, vw) - Math.max(r.left, 0)) * Math.max(0, Math.min(r.bottom, vh) - Math.max(r.top, 0));
      if (a > best) { best = a; bestEl = el; }
    }
    const share = best / (vw * vh);
    if (share < 0.15) emit("hero-art-mobile", "largest authored visual in the first 375-class screen covers " + Math.round(share * 100) + "% of it" + (bestEl ? " (" + describe(bestEl) + ")" : "") + "; keep the signature art large on phones (crop it, place it behind the headline, or bring it above the fold)");
  });

  // dead-controls: most buttons answer neither with a transition nor with a :active press state,
  // so clicks feel unacknowledged. An instant :active colour change counts (static briefs). Layoza-original.
  run("dead-controls", () => {
    const activeSelectors = [];
    for (const sheet of document.styleSheets) {
      let rules;
      try { rules = sheet.cssRules; } catch (e) { continue; } // cross-origin font sheets
      const walk = (list) => {
        for (const rule of list) {
          if (rule.cssRules && !rule.selectorText) walk(rule.cssRules);
          else if (rule.selectorText && rule.selectorText.includes(":active")) {
            for (const part of rule.selectorText.split(",")) {
              const base = part.replace(/:active/g, "").trim();
              if (base) activeSelectors.push(base);
            }
          }
        }
      };
      walk(rules);
    }
    const hasActive = (el) => activeSelectors.some((sel) => { try { return el.matches(sel); } catch (e) { return false; } });
    const controls = [...document.querySelectorAll('button, [role="button"], a[href], input[type="submit"]')].filter((el) => {
      const r = el.getBoundingClientRect();
      const s = cs(el);
      if (r.width < 24 || r.height < 20 || s.visibility === "hidden" || s.display === "none") return false;
      // Button-like: a filled or outlined control, not running prose links
      const filled = s.backgroundColor !== "rgba(0, 0, 0, 0)" && s.backgroundColor !== "transparent";
      const outlined = parseFloat(s.borderTopWidth) > 0 && s.borderTopStyle !== "none";
      return el.tagName !== "A" || filled || outlined;
    });
    if (controls.length < 3) return;
    const dead = controls.filter((el) => {
      const durs = cs(el).transitionDuration.split(",").map((d) => parseFloat(d) || 0);
      return Math.max(...durs) === 0 && !hasActive(el);
    });
    if (dead.length / controls.length >= 0.5) emit("dead-controls", dead.length + " of " + controls.length + " buttons have no transition and no :active press state (e.g. " + describe(dead[0]) + "); give every control hover, press and state feedback (layoza-animations 2c)");
  });

  // display-scale: on desktop the largest visible text stays under 5vw, so no type reaches
  // architectural scale and the page reads as a smooth template scale. Layoza-original; threshold
  // informed by Leonxlnx/taste-skill research.md (MIT; model-panel medians 8.4vw vs 5.0vw).
  run("display-scale", () => {
    const vw = window.innerWidth || 0;
    if (vw < 1200) return;
    let max = 0, maxEl = null;
    for (const el of D.body.querySelectorAll("h1, h2, h3, p, span, a, div, strong, em, blockquote, li")) {
      if (!el.firstChild || ![...el.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim().length >= 2)) continue;
      const s = cs(el);
      if (s.visibility === "hidden" || s.display === "none") continue;
      const r = el.getBoundingClientRect();
      if (r.width < 1 || r.height < 1) continue;
      const fs = parseFloat(s.fontSize) || 0;
      if (fs > max) { max = fs; maxEl = el; }
    }
    if (max && max / vw < 0.05) emit("display-scale", "largest text is " + Math.round(max) + "px (" + (max / vw * 100).toFixed(1) + "vw) at " + vw + "px; let one headline or the name reach about 8vw, or declare the small-type direction on purpose");
  });

  return { issues, suppressed, errors };
}
const TELLS_EVAL_SCRIPT = "(" + layozaGenericTells.toString() + ")()";

/*
 * ---------------------------------------------------------------------------------------------
 * Interface hygiene (Layoza-original code)
 *
 * Objective platform checks whose ideas come from vercel-labs/web-interface-guidelines (MIT) and
 * ibelick/ui-skills (MIT); detection is Layoza's own, on rendered computed styles. Each one names a
 * concrete user-visible failure (forced zoom, light scrollbars on a dark page, layout shift, a
 * control the keyboard cannot reach), so they are "warning"/"error" issues, not taste critique.
 *  zoom-blocked       error   viewport meta disables pinch zoom (WCAG 1.4.4 Resize Text)
 *  ios-input-zoom     warning form field font-size < 16px at phone width: iOS Safari zooms on focus
 *  dark-color-scheme  warning dark page without `color-scheme: dark`: native controls and scrollbars stay light
 *  image-no-size      warning <img> with no width/height and no aspect-ratio: shifts layout on load (CLS)
 *  pointer-not-button warning cursor:pointer box with no link/button/role/tabindex: unreachable by keyboard
 *  field-autocomplete warning personal-data field (email/phone/name/address) without `autocomplete`
 *  transition-all     warning `transition: all` animates layout properties and every future change
 * ---------------------------------------------------------------------------------------------
 */
function layozaInterfaceChecks() {
  "use strict";
  const D = document;
  const vw = window.innerWidth || 0;
  const issues = [];
  const errors = [];
  if (!D.body) return { issues, errors };
  const issue = (rule, severity, detail) => issues.push({ rule, severity, detail: String(detail).replace(/\s+/g, " ").slice(0, 300) });
  const run = (name, fn) => {
    try {
      fn();
    } catch (e) {
      if (errors.length < 20) errors.push(name + ": " + (e && e.message ? e.message : String(e)));
    }
  };
  const cs = (el) => getComputedStyle(el);
  const visible = (el) => {
    const r = el.getBoundingClientRect();
    const s = cs(el);
    return r.width >= 1 && r.height >= 1 && s.visibility !== "hidden" && s.display !== "none" && !el.closest('[aria-hidden="true"]');
  };
  const name = (el) => {
    const label = (el.getAttribute("aria-label") || el.getAttribute("name") || el.id || el.textContent || "").trim().replace(/\s+/g, " ").slice(0, 30);
    return "<" + el.tagName.toLowerCase() + (el.id ? "#" + el.id : "") + '> "' + label + '"';
  };
  const rgb = (v) => {
    const m = /rgba?\(\s*([\d.]+)[ ,]+([\d.]+)[ ,]+([\d.]+)(?:\s*[,/]\s*([\d.]+%?))?/.exec(v || "");
    if (!m) return null;
    const a = m[4] === undefined ? 1 : m[4].endsWith("%") ? parseFloat(m[4]) / 100 : parseFloat(m[4]);
    return { r: +m[1], g: +m[2], b: +m[3], a };
  };
  const lum = ({ r, g, b }) => {
    const c = (v) => ((v /= 255) <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4));
    return 0.2126 * c(r) + 0.7152 * c(g) + 0.0722 * c(b);
  };
  const all = Array.from(D.body.querySelectorAll("*"));

  run("zoom-blocked", () => {
    const meta = D.querySelector('meta[name="viewport" i]');
    const c = ((meta && meta.getAttribute("content")) || "").toLowerCase().replace(/\s+/g, "");
    const max = /maximum-scale=([\d.]+)/.exec(c);
    if (/user-scalable=(no|0)\b/.test(c) || (max && parseFloat(max[1]) < 2)) issue("zoom-blocked", "error", 'viewport meta "' + c + '" blocks pinch zoom; remove user-scalable=no / maximum-scale');
  });

  run("ios-input-zoom", () => {
    if (vw > 480) return;
    const small = [];
    for (const el of D.body.querySelectorAll("input:not([type=hidden]):not([type=checkbox]):not([type=radio]):not([type=range]):not([type=submit]):not([type=button]):not([type=color]):not([type=file]), select, textarea")) {
      if (!visible(el)) continue;
      const fs = parseFloat(cs(el).fontSize);
      if (fs < 16) small.push(name(el) + " " + fs + "px");
    }
    if (small.length) issue("ios-input-zoom", "warning", small.length + " form field(s) under 16px at " + vw + "px (iOS zooms on focus): " + small.slice(0, 3).join(", "));
  });

  run("dark-color-scheme", () => {
    let bg = rgb(cs(D.body).backgroundColor);
    if (!bg || bg.a < 0.5) bg = rgb(cs(D.documentElement).backgroundColor);
    if (!bg || bg.a < 0.5 || lum(bg) > 0.06) return;
    const scheme = cs(D.documentElement).colorScheme || "normal";
    const meta = D.querySelector('meta[name="color-scheme" i]');
    if (!/dark/.test(scheme) && !(meta && /dark/i.test(meta.getAttribute("content") || ""))) issue("dark-color-scheme", "warning", "dark page background without color-scheme: dark; scrollbars, form controls and autofill render light");
  });

  run("image-no-size", () => {
    const bad = [];
    for (const img of D.body.querySelectorAll("img")) {
      if (!visible(img)) continue;
      const s = cs(img);
      // Reserved: both attributes, an explicit CSS aspect-ratio, or a fixed box the image is fitted into
      const sized = (img.hasAttribute("width") && img.hasAttribute("height")) || (s.aspectRatio && !/^auto/.test(s.aspectRatio)) || s.objectFit !== "fill" || /absolute|fixed/.test(s.position);
      if (!sized) bad.push((img.getAttribute("src") || "").split("/").pop().slice(0, 40) || "inline image");
    }
    if (bad.length) issue("image-no-size", "warning", bad.length + " image(s) without width/height or aspect-ratio reserve no space before loading: " + bad.slice(0, 3).join(", "));
  });

  run("pointer-not-button", () => {
    const NATIVE = 'a[href], button, input, select, textarea, summary, label, [role], [tabindex], [contenteditable="true"], video, audio, iframe';
    const bad = [];
    for (const el of all) {
      if (bad.length >= 5) break;
      if (el.matches(NATIVE) || el.closest(NATIVE) || el.querySelector(NATIVE)) continue;
      const s = cs(el);
      if (s.cursor !== "pointer" || !visible(el)) continue;
      // Only the outermost pointer box counts; its children inherit the cursor
      if (el.parentElement && cs(el.parentElement).cursor === "pointer") continue;
      bad.push(name(el));
    }
    if (bad.length) issue("pointer-not-button", "warning", "element(s) styled clickable (cursor: pointer) with no link, button, role or tabindex, so keyboard users cannot reach them: " + bad.slice(0, 3).join(", "));
  });

  run("field-autocomplete", () => {
    const bad = [];
    const PERSONAL = /e-?mail|phone|tel|mobile|first.?name|last.?name|full.?name|^name$|surname|address|postcode|postal|zip|city|country/i;
    for (const el of D.body.querySelectorAll("input:not([type=hidden]):not([type=checkbox]):not([type=radio]):not([type=submit]):not([type=button]):not([type=search])")) {
      if (!visible(el) || el.hasAttribute("autocomplete")) continue;
      const lab = (el.labels && el.labels[0] ? el.labels[0].textContent : "") + " " + (el.name || "") + " " + (el.id || "") + " " + (el.type || "") + " " + (el.getAttribute("placeholder") || "");
      if (el.type === "email" || el.type === "tel" || PERSONAL.test(lab.trim())) bad.push(name(el));
    }
    const emailAsText = Array.from(D.body.querySelectorAll("input[type=text], input:not([type])")).some((i) => visible(i) && /e-?mail/i.test((i.name || "") + " " + (i.id || "") + " " + (i.labels && i.labels[0] ? i.labels[0].textContent : "")));
    if (emailAsText) bad.push("an email field typed as text (no email keyboard)");
    if (bad.length) issue("field-autocomplete", "warning", bad.length + " personal-data field(s) without autocomplete or the right type, so people retype what their browser knows: " + bad.slice(0, 3).join(", "));
  });

  run("transition-all", () => {
    let n = 0;
    let ex = null;
    for (const el of all) {
      const s = cs(el);
      if (!/(^|,\s*)all(\s|,|$)/.test(s.transitionProperty)) continue;
      if (!s.transitionDuration.split(",").some((d) => parseFloat(d) > 0)) continue;
      n++;
      if (!ex) ex = name(el);
    }
    if (n >= 3) issue("transition-all", "warning", "transition: all on " + n + " elements (e.g. " + ex + "); list the properties that should animate (transform, opacity, colour)");
  });

  const ratio = (a, b) => {
    const l1 = lum(a), l2 = lum(b);
    return (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05);
  };
  const behind = (el) => {
    for (let p = el.parentElement; p; p = p.parentElement) {
      const c = rgb(cs(p).backgroundColor);
      if (c && c.a >= 0.9) return c;
    }
    return { r: 255, g: 255, b: 255, a: 1 };
  };

  // field-boundary-contrast: WCAG 1.4.11 — a text field's edge (border or fill) needs 3:1 against
  // what surrounds it, or people can't find where to type. Layoza-original.
  run("field-boundary-contrast", () => {
    const bad = [];
    for (const el of D.body.querySelectorAll("input:not([type=hidden]):not([type=checkbox]):not([type=radio]):not([type=range]):not([type=submit]):not([type=button]):not([type=color]):not([type=file]):not([type=image]), select, textarea")) {
      if (!visible(el)) continue;
      const s = cs(el);
      const around = behind(el);
      const fill = rgb(s.backgroundColor);
      const fillOk = fill && fill.a >= 0.9 && ratio(fill, around) >= 3;
      const sides = ["Top", "Right", "Bottom", "Left"].filter((k) => parseFloat(s["border" + k + "Width"]) >= 1 && s["border" + k + "Style"] !== "none");
      const borderOk = sides.some((k) => {
        const c = rgb(s["border" + k + "Color"]);
        return c && c.a >= 0.9 && ratio(c, fill && fill.a >= 0.9 ? fill : around) >= 3 && ratio(c, around) >= 3;
      });
      if (!fillOk && !borderOk) bad.push(name(el));
    }
    if (bad.length) issue("field-boundary-contrast", "warning", bad.length + " form field(s) whose border and fill are both under 3:1 against their surroundings (WCAG 1.4.11), so the field edge is hard to see: " + bad.slice(0, 3).join(", "));
  });

  // render-blocking-fonts: a third-party font stylesheet loaded as a plain <link rel=stylesheet>
  // (a media="print" link whose onload swaps media to all has already loaded non-blocking; skip it)
  // blocks first paint on a cross-origin round trip; the headline (usually LCP) waits. Layoza-original.
  run("render-blocking-fonts", () => {
    const FONT_HOST = /fonts\.googleapis\.com|use\.typekit\.net|fonts\.bunny\.net|api\.fontshare\.com/i;
    const bad = Array.from(D.querySelectorAll('link[rel~="stylesheet"][href]')).filter((l) => FONT_HOST.test(l.href) && !l.closest("noscript") && !/\bmedia\b/.test(l.getAttribute("onload") || "") && (!l.media || /^(all|screen)$/i.test(l.media.trim())) && !l.disabled);
    if (bad.length) issue("render-blocking-fonts", "warning", bad.length + " font stylesheet(s) block first paint (" + bad[0].href.slice(0, 60) + "); preconnect, then load with media=\"print\" onload=\"this.media='all'\" plus a <noscript> copy and metric-matched fallbacks");
  });

  // svg-text-illegible: SVG labels scaled down by their viewBox render far smaller than their CSS
  // font-size says (charts, maps, diagrams on phones). Layoza-original.
  run("svg-text-illegible", () => {
    let n = 0;
    const ex = [];
    for (const t of D.body.querySelectorAll("svg text")) {
      const txt = (t.textContent || "").trim();
      if (!txt || t.closest('[aria-hidden="true"]') && !t.closest("svg[role=img], figure")) continue;
      const r = t.getBoundingClientRect();
      const s = cs(t);
      if (r.width < 1 || r.height < 1 || s.visibility === "hidden" || s.display === "none" || parseFloat(s.opacity) < 0.1) continue;
      const m = t.getScreenCTM && t.getScreenCTM();
      const scale = m ? Math.hypot(m.a, m.b) : 1;
      const px = parseFloat(s.fontSize) * scale;
      if (px > 0 && px < 9) {
        n++;
        if (ex.length < 3) ex.push('"' + txt.slice(0, 20) + '" ' + px.toFixed(1) + "px");
      }
    }
    if (n >= 3) issue("svg-text-illegible", "warning", n + " SVG text label(s) render under 9px at " + vw + "px after viewBox scaling: " + ex.join(", ") + "; enlarge them, hide them at this width, or move the content into HTML");
  });

  // text-offscreen: visible text partly cut at the viewport edge (labels hung off a phone's edge,
  // often hidden from document-overflow by body { overflow-x: hidden }). Skips fully offscreen
  // drawers, marquee tracks, scrollable carousels and visually-hidden text. Layoza-original.
  run("text-offscreen", () => {
    const ex = [];
    const scroller = (el) => { for (let c = el.parentElement; c && c !== D.body; c = c.parentElement) { if (/auto|scroll/.test(cs(c).overflowX) && c.scrollWidth > c.clientWidth + 2) return true; } return false; };
    for (const el of D.body.querySelectorAll("*")) {
      const svgText = el.tagName.toLowerCase() === "text";
      if (!svgText && ![...el.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim().length > 1)) continue;
      if (el.closest('[aria-hidden="true"]') && !svgText) continue;
      const s = cs(el);
      if (s.visibility === "hidden" || s.display === "none" || parseFloat(s.opacity) < 0.1) continue;
      const r = el.getBoundingClientRect();
      if (r.width < 4 || r.height < 4 || r.right <= 0 || r.left >= vw || r.width > vw * 0.9) continue;
      // Deliberate display bleeds (giant words run off the edge on purpose) are art direction, not a cut label.
      if (parseFloat(s.fontSize) >= 48) continue;
      // Tickers and marquees: text riding a track wider than its clipping parent.
      let ticker = false;
      for (let c = el.parentElement; c && c !== D.body; c = c.parentElement) {
        if (/hidden|clip/.test(cs(c).overflowX) && c.scrollWidth > c.clientWidth + 2) { ticker = true; break; }
      }
      if (ticker) continue;
      const cut = Math.max(0, -r.left) + Math.max(0, r.right - vw);
      if (cut <= Math.max(3, r.width * 0.02) || scroller(el)) continue;
      ex.push('"' + (el.textContent || "").trim().slice(0, 24) + '" cut ' + Math.round(cut) + "px");
    }
    if (ex.length) issue("text-offscreen", "warning", ex.length + " text label(s) cut off at the viewport edge at " + vw + "px: " + ex.slice(0, 3).join(", ") + "; keep labels inside the column (anchor them inward, inset the drawing, or re-stage for phones)");
  });

  // small-submit: a form's submit control is shorter than 44px, so the action where the eye ends
  // reads as an afterthought and misses comfortable touch size. Layoza-original.
  run("small-submit", () => {
    const bad = [];
    for (const f of D.body.querySelectorAll("form")) {
      if (!visible(f)) continue;
      const b = f.querySelector('button:not([type=button]):not([type=reset]), input[type=submit]');
      if (!b || !visible(b)) continue;
      const h = b.getBoundingClientRect().height;
      if (h < 44) bad.push(Math.round(h) + "px \"" + ((b.textContent || b.value || "").trim().slice(0, 20)) + "\"");
    }
    if (bad.length) issue("small-submit", "warning", bad.length + " form submit button(s) under 44px tall at " + vw + "px: " + bad.slice(0, 3).join(", ") + "; make the primary form action full-size");
  });

  // dead-strip-below-footer: blank document height after the last visible content, usually a
  // leftover min-height, spacer or pinned-stage padding. Layoza-original.
  run("dead-strip-below-footer", () => {
    const docH = Math.max(D.documentElement.scrollHeight, D.body.scrollHeight);
    let bottom = 0;
    for (const el of D.body.querySelectorAll("*")) {
      const s = cs(el);
      if (s.position === "fixed" || s.display === "none" || s.visibility === "hidden" || parseFloat(s.opacity) < 0.05) continue;
      const r = el.getBoundingClientRect();
      if (r.width < 2 || r.height < 2) continue;
      const hasPaint = (el.childNodes.length && [...el.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim())) || /^(IMG|SVG|CANVAS|VIDEO|svg|INPUT|BUTTON)$/.test(el.tagName) || !/rgba\(0, 0, 0, 0\)|transparent/.test(s.backgroundColor) || parseFloat(s.borderBottomWidth) > 0;
      if (!hasPaint) continue;
      bottom = Math.max(bottom, r.bottom + window.scrollY);
    }
    // Only blank space the visitor has to scroll into counts; a short page simply ends above the fold.
    const gap = docH - Math.max(bottom, window.innerHeight || 0);
    if (bottom && gap > Math.max(80, (window.innerHeight || 0) * 0.12)) issue("dead-strip-below-footer", "warning", Math.round(gap) + "px of blank page after the last content at " + vw + "px; end the document on the footer (check min-height, spacers and pinned-stage padding)");
  });


  return { issues, errors };
}
const INTERFACE_EVAL_SCRIPT = "(" + layozaInterfaceChecks.toString() + ")()";

/**
 * Layoza quality sweep: Layoza's own rule set for generic-default tells, shipping slips and
 * brief-blind style habits. Every finding carries the rule id, a selector, the measured value, why
 * it matters and a fix hint. Severity "warning" is objective (feeds the verifier as a warning, never
 * as a release-blocking error); "critique" is advisory taste and never gates.
 *
 * Style habits (gradient surfaces, glass, resting shadows, hard offset shadows, mono running text,
 * pill controls, floating pill nav) are judged against the brief, never banned: each is a critique
 * that the project silences by naming it in DESIGN.md/BRAND.md (passed as --allow by `layoza qa`),
 * with `--allow <id|group>`, or with <meta name="layoza-brand-allow" content="rule:<id>">.
 */
function layozaQualitySweep(opts) {
  "use strict";
  opts = opts || {};
  const D = document;
  const vw = window.innerWidth || 0;
  const vh = window.innerHeight || 0;
  const issues = [];
  const suppressed = [];
  const errors = [];
  if (!D.body) return { issues, suppressed, errors };

  // Rule registry: id -> [severity, category, group, why, fix]. Groups name the brief-aware styles.
  const RULES = {
    "title-missing": ["warning", "content", "", "Tabs, bookmarks, history and search results show the document title; without one the page is anonymous.", "Write a <title> that names the page first and the brand second."],
    "duplicate-id": ["warning", "accessibility", "", "Labels, aria references and in-page links resolve to the first element with an id, so later duplicates are unreachable or mislabelled.", "Give every id one owner; generate unique ids for repeated components."],
    "positive-tabindex": ["warning", "accessibility", "", "A tabindex above 0 pulls the element ahead of the document order, so keyboard users jump around the page.", "Use tabindex=\"0\" or none, and order the DOM the way it should be read."],
    "placeholder-copy": ["warning", "content", "", "Template filler left in a shipped page tells visitors nobody finished it.", "Replace the filler with real copy, or remove the block."],
    "font-load-failed": ["warning", "typography", "", "A declared web font failed to load, so the page renders in a fallback face the design was never checked in.", "Fix the font URL/format or self-host the file; keep a metric-matched fallback in the stack."],
    "autoplay-sound": ["warning", "accessibility", "", "Media that starts playing with sound startles people and talks over screen readers (WCAG 1.4.2).", "Add muted to autoplaying media, or start playback only on a user action."],
    "video-blank-start": ["warning", "media", "", "A video that does not autoplay and has no poster shows an empty or black frame until someone presses play.", "Add a poster image that previews the footage."],
    "alt-is-filename": ["warning", "accessibility", "", "Screen readers read the alt text aloud; a file name or a bare word like \"image\" says nothing about the picture.", "Describe what the image shows, or use alt=\"\" when it is decorative."],
    "raster-oversized": ["warning", "performance", "", "The browser downloads and decodes far more pixels than the slot shows, which slows LCP on phones.", "Serve srcset/sizes candidates close to the rendered width (x device pixel ratio)."],
    "focus-ring-missing": ["warning", "accessibility", "", "With :focus-visible matched, the control looks identical focused and unfocused, so keyboard users lose their place (WCAG 2.4.7).", "Give :focus-visible a ring that differs from the resting state: outline 2px+ with an offset, in a colour with 3:1 contrast."],
    "blend-mode-text": ["warning", "accessibility", "", "Text painted with difference/exclusion blending takes its colour from whatever is behind it, so its contrast changes from pixel to pixel.", "Use a plain colour for text that must be read; keep blend modes for decoration."],
    "rtl-dir-missing": ["warning", "i18n", "", "The page language is written right to left but the document direction is still left to right, so text, punctuation and layout run the wrong way.", "Set dir=\"rtl\" on <html> (or on the RTL section)."],
    "rtl-physical-props": ["warning", "i18n", "", "Right-to-left pages mirror the layout; physical left/right properties stay pinned to the wrong side.", "Use logical properties: margin-inline-start, padding-inline-end, inset-inline-start, text-align: start, border-inline-start."],
    "sidebar-vanishes": ["warning", "navigation", "", "At this width the sidebar navigation disappears completely and nothing else on screen carries its links.", "Collapse the sidebar to an icon rail with labels on hover/focus, or move its links into a visible menu."],
    "reduced-motion-hides-text": ["warning", "accessibility", "", "With reduced motion requested the reveal animations never run, so this text stays invisible for exactly the people who asked for less motion.", "Under prefers-reduced-motion: reduce, show the end state immediately instead of skipping the animation that reveals it."],
    "straggler-line": ["critique", "typography", "", "A single short word alone on the last line of a heading or paragraph breaks the shape of the block and reads as an accident.", "Add text-wrap: pretty to running text, or text-wrap: balance to headings."],
    "heading-unbalanced": ["critique", "typography", "", "A multi-line heading with one long line and one short line reads lopsided.", "Set text-wrap: balance on headings, or adjust the max-width so the lines are even."],
    "pill-everything": ["critique", "style", "pill", "When every button, chip and field is a fully rounded pill, nothing signals hierarchy and the page looks like a default kit.", "Reserve the pill for one role (tags or the primary action) and give the others a radius that matches the system."],
    "hard-offset-shadow": ["critique", "style", "offset-shadow", "Zero-blur offset shadows on many boxes are a recognisable stock look; fine when the brief asks for it, noise when it does not.", "Keep the offset shadow for the elements the brief wants to pop, or name the style in DESIGN.md."],
    "fx-gradient-surfaces": ["critique", "style", "gradient", "Several large surfaces carry decorative gradients; without a reason in the brief they read as filler rather than identity.", "Use flat surfaces and let content (photos, product, data) carry the colour, or declare the gradient in DESIGN.md."],
    "fx-glass": ["critique", "style", "glass", "Frosted translucent panels on many boxes cost GPU time and muddy text contrast; it is a default look, not a decision.", "Use solid panels unless the brief calls for glass; if it does, keep it to one layer (nav or overlay)."],
    "fx-resting-shadows": ["critique", "style", "shadow", "Most cards float on large soft shadows at rest, which flattens the hierarchy because everything is lifted.", "Reserve elevation for things that sit above the page (menus, dialogs, dragged items); separate cards with space or a quiet border."],
    "fx-text-shadow": ["critique", "style", "glow", "Text shadows and glows on many text blocks blur the letterforms and look like effects for their own sake.", "Remove text-shadow from running text; if text sits on an image, use a scrim behind it instead."],
    "mono-running-text": ["critique", "typography", "mono", "Paragraphs set in a monospace face are slower to read; mono works for code, figures and short labels.", "Set running text in a proportional face and keep mono for code and data."],
    "nav-floating-pill": ["critique", "style", "pill", "A detached rounded navigation capsule floating over the page is a stock template move.", "Dock the navigation to the edge, or name the floating nav in DESIGN.md when the brief wants it."],
    "imagery-scarce": ["critique", "content", "imagery", "A long page with no photographs, product shots or real artwork shows nothing of the subject; it reads as a template.", "Show the actual objects of the trade: product, place, people, work samples, data."],
    "font-size-sprawl": ["critique", "typography", "", "Many unrelated font sizes mean there is no type scale; hierarchy becomes guesswork.", "Pick 5-7 sizes from a ratio-based scale and map every text role to one of them."],
    "radius-sprawl": ["critique", "style", "", "Many different corner radii on boxes of the same kind look unplanned.", "Define 2-4 radius tokens (control, card, overlay) and use only those."],
    "link-default-blue": ["critique", "style", "", "Links still use the browser default blue/purple, which clashes with the palette and looks unfinished.", "Style links from the palette with an underline or other non-colour cue."],
    "emoji-icons": ["critique", "style", "emoji", "Emoji used as interface icons render differently on every platform and look like a placeholder.", "Use one consistent icon set (SVG), sized and coloured from the system."],
    "figures-proportional": ["critique", "typography", "", "Columns of numbers in proportional figures do not line up, so values are hard to compare.", "Set font-variant-numeric: tabular-nums on numeric cells and data."],
    "numbers-left-aligned": ["critique", "typography", "", "Left-aligned numbers put the units digit in a different place on every row.", "Right-align numeric columns (and their headers)."],
    "skip-link-missing": ["critique", "accessibility", "", "Keyboard users have to tab through every header link before reaching the content.", "Add a \"Skip to content\" link as the first focusable element, pointing at <main>."],
    "cta-competition": ["critique", "layout", "", "Several equally loud filled buttons on the first screen compete; the visitor cannot tell which action matters.", "Keep one filled primary action per screen; make the others secondary (outline or text)."],
    "row-heights-ragged": ["critique", "layout", "", "Cards in the same row end at different heights, so the grid looks broken.", "Let the cards stretch to the row height (align-items: stretch, grid) and pin their actions to the bottom."],
    "scroller-hidden": ["critique", "interaction", "", "A horizontal scroller hides its scrollbar and offers no buttons, so most visitors never discover the rest of the items.", "Show part of the next item at the edge, keep a visible scrollbar or add previous/next buttons."],
    "text-below-floor": ["error", "typography", "", "Running text under 14px or interface labels under 12px are hard to read on phones and for anyone with low vision; zoom is not a substitute.", "Set body copy to at least 14px (16px is the safe web default) and labels, buttons and fields to at least 12px."],
    "eyebrow-on-most-headings": ["warning", "template-tell", "eyebrow", "A small all-caps label sits above most section headings; repeated on every section it stops labelling anything and reads as a template habit.", "Keep a label only where it carries information (a category, a date); let most headings stand alone."],
    "numbered-non-sequence": ["warning", "template-tell", "numbering", "Cards numbered 01, 02, 03 imply an order or a process; on content that is not a sequence the numbers are decoration that suggests steps that do not exist.", "Drop the numbers, or rewrite the cards as real steps if the order matters."],
    "illustrated-people": ["warning", "template-tell", "illustration", "Generic vector people standing in for the subject are a stock look; they show no real customer, product or place.", "Use photographs of the real people, work or product, or drop the figure."],
    "violet-blue-hero": ["warning", "template-tell", "gradient", "A violet-to-blue gradient behind the first screen is the most common generated-site backdrop; it says nothing about this subject.", "Build the first screen from the subject's own colours or imagery."],
    "logo-wall-unverified": ["warning", "trust", "", "A row of client logos that look generic or placeholder suggests endorsements that may not exist. Verify this is real.", "Use only logos you have permission to show, with real names in the alt text, or remove the row."],
    "testimonial-unsourced": ["warning", "trust", "", "Testimonials with stock names or no company, role or link cannot be checked and read as invented. Verify this is real.", "Attribute each quote to a real, named person with their role and organisation, ideally with a link or photo they approved."],
    "hover-scale-uniform": ["warning", "motion", "hover-scale", "The same hover zoom on many unrelated elements (cards, buttons, images, logos) is a blanket effect, not feedback designed for each control.", "Give each kind of control its own small, purposeful hover change, and leave non-interactive boxes still."],
    "hover-effects-stacked": ["warning", "motion", "", "A hover that changes three or more things at once (lift, shadow, colour, border, filter) is noisy and slows the eye.", "Pick one or two hover signals per control and keep them consistent across the system."],
    "layout-property-animated": ["warning", "performance", "", "Animating width, height, top/left or margin/padding re-runs layout every frame, which janks on phones and can shift the page.", "Animate transform and opacity instead (scale, translate), or use grid-template-rows / interpolate-size for expanding panels."],
    "focus-ring-animated": ["warning", "accessibility", "", "A focus indicator that fades or grows in is briefly invisible, so keyboard users lose their place while tabbing quickly.", "Show the focus ring instantly: exclude outline and box-shadow from the transition on :focus-visible."],
    "auto-rotate-no-pause": ["warning", "accessibility", "", "Content that moves on its own for more than five seconds needs a way to pause it (WCAG 2.2.2); people reading slowly lose the slide.", "Add a visible pause button, pause on hover and focus, and do not auto-rotate under prefers-reduced-motion."],
    "disabled-unstyled": ["warning", "interaction", "", "A disabled control looks exactly like an enabled one, so people press it and nothing happens.", "Style :disabled / [aria-disabled=true] with reduced contrast and a not-allowed cursor, and explain why it is unavailable."],
    "field-states-missing": ["warning", "forms", "", "The form has no styles for invalid or error states, so a rejected field looks identical to a valid one.", "Style :user-invalid or [aria-invalid=true] with a colour plus an icon or message, and reserve space so the message does not shift the layout."],
    "motion-ignores-reduce": ["warning", "motion", "", "Animations keep running when the visitor has asked the system to reduce motion, which can trigger nausea and vestibular symptoms.", "Wrap decorative keyframes and transforms in @media (prefers-reduced-motion: no-preference), or stop them under reduce."],
    "placeholder-names": ["warning", "trust", "", "Stock names, example.com addresses or 555 phone numbers are template filler. Verify this is real.", "Replace them with the real contact details and names, or remove the block."],
    "claim-unsourced": ["warning", "trust", "", "Headline claims like \"10x faster\" or \"trusted by 50,000+\" with no source nearby read as invented. Verify this is real.", "Link each claim to its source (study, case study, public count) or soften it to what you can prove."],
    "mock-device-chrome": ["warning", "template-tell", "mockup", "Fake browser windows (three coloured dots and a URL pill) or drawn phone frames around screenshots are a template prop.", "Show the real product screenshot on its own, cropped to the part that matters."],
    "neutrals-untinted": ["critique", "color", "neutral", "Every grey on the page is pure (zero chroma); greys tinted toward the brand hue make the palette feel considered.", "Give neutrals a slight tint of the brand hue (chroma about 0.005-0.02 in OKLCH)."],
    "accent-flood": ["critique", "color", "accent", "Saturated accent fill covers a large share of the first screen, so the accent stops pointing at anything.", "Keep the accent to the primary action and a few highlights (roughly 5% of the screen) unless the brief calls for a colour-block look."],
    "measure-narrow": ["warning", "typography", "", "Running text set in very narrow columns breaks into choppy lines and too many hyphen-free ragged ends.", "Widen the column to about 45-75 characters per line (max-width: 65ch) for body copy."],
    "caps-display-tight": ["warning", "typography", "", "Multi-line all-caps display text with line-height under 1.0 makes the rows touch or collide.", "Set line-height to at least 1.0-1.1 for uppercase display type."],
    "sticky-under-header": ["warning", "layout", "", "A sticky element pinned at top: 0 slides under the fixed header, so its top is hidden while it is stuck.", "Set its top to the header height (top: var(--header-h)) or use scroll-margin/scroll-padding."],
    "nav-default-layout": ["warning", "template-tell", "structure", "Wordmark left, four or five centred links and one button right is the default generated navigation; it gives the brand nothing of its own.", "Shape the navigation around how this audience moves through the site (fewer links, a search, a booking entry, a different placement)."],
    "footer-default-columns": ["warning", "template-tell", "structure", "A footer with Product / Company / Resources / Legal columns is the stock template footer.", "Group the footer by what visitors here actually look for (opening hours, locations, contact, cases) and drop empty categories."],
    "page-template-sequence": ["warning", "template-tell", "structure", "Hero, then three equal feature cards, then a call-to-action band, then the footer is the default landing-page outline.", "Order the page around the subject's own story; vary section forms (a list, a table, a case, a timeline)."],
    "lcp-image-lazy": ["warning", "performance", "", "The largest image on the first screen is lazy-loaded, so the browser waits to fetch it and LCP slips.", "Remove loading=lazy from the hero image and add fetchpriority=high."],
    "hero-video-no-poster": ["warning", "performance", "", "An autoplay hero video with no poster shows nothing until the video decodes, which delays the first paint of the hero.", "Add a poster image (the first frame, compressed) and preload=metadata."],
    "icon-sets-mixed": ["warning", "style", "", "Two or more icon libraries on one page bring different stroke widths, grids and corner styles.", "Use one icon set throughout, or redraw the odd icons to match."],
    "graphic-unhidden": ["warning", "accessibility", "", "Decorative canvas or svg graphics without aria-hidden or a label are announced as unlabeled images by some screen readers.", "Add aria-hidden=\"true\" to decorative graphics, or role=img with an aria-label when they carry meaning."],
    "overflow-at-width": ["error", "layout", "", "The page scrolls sideways at this width, so content is cut off and the whole page wobbles on touch.", "Find the element wider than the viewport (fixed widths, 100vw plus padding, long words, wide media) and constrain it with max-width: 100%, minmax(0, 1fr) or overflow-wrap."],
    "stats-too-round": ["warning", "trust", "", "Several headline numbers that are all suspiciously round (10K+, 100%, 500+) read as placeholders. Verify this is real.", "Show the real figure with its source and date, or remove the stat."],
  };

  const allowSet = new Set((opts.allow || []).map((x) => String(x).toLowerCase().trim()).filter(Boolean));
  D.querySelectorAll('meta[name="layoza-brand-allow" i]').forEach((m) =>
    (m.getAttribute("content") || "").split(",").forEach((p) => {
      const v = p.trim().toLowerCase();
      if (v.startsWith("rule:")) allowSet.add(v.slice(5).trim());
      else if (v.startsWith("style:")) allowSet.add(v.slice(6).trim());
    })
  );
  const counts = {};
  const emit = (id, el, measured, detail) => {
    const r = RULES[id];
    if (!r) return;
    counts[id] = (counts[id] || 0) + 1;
    if (counts[id] > 6) return;
    const sel = el ? selectorOf(el) : "document";
    if (allowSet.has(id) || (r[2] && allowSet.has(r[2]))) {
      if (counts[id] === 1) suppressed.push({ rule: id, detail: "allowed by the brief (" + (allowSet.has(id) ? id : r[2]) + ")" });
      return;
    }
    issues.push({
      rule: id,
      severity: r[0],
      category: r[1],
      selector: sel,
      measured: String(measured).slice(0, 160),
      why: r[3],
      fix: r[4],
      detail: (sel + ": " + (detail || measured) + ". " + r[4]).replace(/\s+/g, " ").slice(0, 320),
    });
  };
  const only = Array.isArray(opts.only) ? new Set(opts.only) : null;
  const run = (id, fn) => {
    if (only && !only.has(id)) return;
    try {
      fn();
    } catch (e) {
      if (errors.length < 20) errors.push(id + ": " + (e && e.message ? e.message : String(e)));
    }
  };
  function selectorOf(el) {
    const parts = [];
    for (let cur = el, depth = 0; cur && cur.nodeType === 1 && depth < 3; cur = cur.parentElement, depth++) {
      let p = cur.tagName.toLowerCase();
      if (cur.id && /^[A-Za-z][\w-]*$/.test(cur.id)) { parts.unshift(p + "#" + cur.id); break; }
      const cls = (cur.getAttribute("class") || "").trim().split(/\s+/).filter((c) => /^[A-Za-z_][\w-]*$/.test(c)).slice(0, 2);
      if (cls.length) p += "." + cls.join(".");
      else if (cur.parentElement) {
        const same = Array.from(cur.parentElement.children).filter((c) => c.tagName === cur.tagName);
        if (same.length > 1) p += ":nth-of-type(" + (same.indexOf(cur) + 1) + ")";
      }
      parts.unshift(p);
      if (cur === D.body) break;
    }
    return parts.join(" > ");
  }
  const cs = (el) => getComputedStyle(el);
  const px = (v) => parseFloat(v) || 0;
  const shown = (el) => {
    if (!el || el.closest("[hidden], [inert], dialog:not([open]), template, details:not([open]) > :not(summary)")) return false;
    if (typeof el.checkVisibility === "function" && !el.checkVisibility({ opacityProperty: true, visibilityProperty: true })) return false;
    const r = el.getBoundingClientRect();
    return r.width >= 2 && r.height >= 2;
  };
  const rgba = (v) => {
    const m = /rgba?\(\s*([\d.]+)[ ,]+([\d.]+)[ ,]+([\d.]+)(?:\s*[,/]\s*([\d.]+%?))?/.exec(v || "");
    if (!m) return null;
    const a = m[4] === undefined ? 1 : m[4].endsWith("%") ? parseFloat(m[4]) / 100 : parseFloat(m[4]);
    return [+m[1], +m[2], +m[3], a];
  };
  const textOf = (el) => (el.textContent || "").replace(/\s+/g, " ").trim();
  const ownText = (el) => Array.from(el.childNodes).some((n) => n.nodeType === 3 && n.nodeValue.trim().length > 0);
  const all = Array.from(D.body.querySelectorAll("*")).slice(0, 6000);
  const docH = D.documentElement.scrollHeight;
  const pageTop = window.scrollY;
  const absTop = (el) => el.getBoundingClientRect().top + pageTop;
  const controls = Array.from(D.querySelectorAll("a[href], button, input:not([type=hidden]), select, textarea, [role='button'], [tabindex]"));

  // ---- Shipping slips -------------------------------------------------------------------------
  run("title-missing", () => {
    if (!(D.title || "").trim()) emit("title-missing", D.documentElement, "<title> empty or absent");
  });

  run("duplicate-id", () => {
    const seen = new Map();
    for (const el of D.querySelectorAll("[id]")) {
      const id = el.id;
      if (!id) continue;
      seen.set(id, (seen.get(id) || 0) + 1);
    }
    for (const [id, n] of seen) {
      if (n < 2) continue;
      const esc = window.CSS && CSS.escape ? CSS.escape(id) : id;
      const referenced = D.querySelector('[for="' + esc + '"], [aria-labelledby~="' + esc + '"], [aria-describedby~="' + esc + '"], [aria-controls~="' + esc + '"], a[href="#' + esc + '"]');
      emit("duplicate-id", D.getElementById(id), 'id="' + id + '" on ' + n + " elements" + (referenced ? " (referenced by " + referenced.tagName.toLowerCase() + ")" : ""));
    }
  });

  run("positive-tabindex", () => {
    for (const el of D.querySelectorAll("[tabindex]")) {
      const t = parseInt(el.getAttribute("tabindex"), 10);
      if (t > 0) emit("positive-tabindex", el, "tabindex=" + t);
    }
  });

  run("placeholder-copy", () => {
    const FILLER = /\b(lorem ipsum|dolor sit amet|consectetur adipisc|placeholder text|sample text|your (?:headline|title|text|tagline|company|logo) here|insert (?:text|copy|image|content) here|coming soon\.{3}|todo:|tbd\b|xxx+\b|\[(?:company|name|city|product|insert)[^\]]*\])/i;
    const walker = D.createTreeWalker(D.body, NodeFilter.SHOW_TEXT);
    let hits = 0;
    for (let n = walker.nextNode(); n && hits < 6; n = walker.nextNode()) {
      const el = n.parentElement;
      if (!el || el.closest("script, style, noscript, template, code, pre, textarea")) continue;
      const m = FILLER.exec(n.nodeValue || "");
      if (m && shown(el)) {
        hits++;
        emit("placeholder-copy", el, '"' + m[0] + '"');
      }
    }
    for (const el of D.querySelectorAll("img[src], a[href]")) {
      const v = el.getAttribute("src") || el.getAttribute("href") || "";
      if (/placehold\.(?:co|it)|via\.placeholder|dummyimage\.com|picsum\.photos|placekitten|^#$/.test(v) && shown(el) && el.tagName === "IMG") emit("placeholder-copy", el, "stock placeholder image " + v.slice(0, 60));
    }
  });

  run("font-load-failed", () => {
    if (!D.fonts) return;
    const failed = new Set();
    D.fonts.forEach((f) => {
      if (f.status === "error") failed.add(String(f.family).replace(/["']/g, ""));
    });
    if (!failed.size) return;
    const users = all.filter((el) => ownText(el) && failed.has((cs(el).fontFamily.split(",")[0] || "").replace(/["']/g, "").trim()));
    for (const fam of failed) {
      const el = users.find((u) => cs(u).fontFamily.includes(fam));
      emit("font-load-failed", el || D.body, '"' + fam + '" status error' + (el ? ", used by " + users.filter((u) => cs(u).fontFamily.includes(fam)).length + " text element(s)" : ""));
    }
  });

  run("media", () => {
    for (const m of D.querySelectorAll("video, audio")) {
      const auto = m.autoplay || m.hasAttribute("autoplay");
      const silent = m.muted || m.hasAttribute("muted") || m.volume === 0;
      if (auto && !silent && !(m.tagName === "VIDEO" && m.readyState >= 1 && m.mozHasAudio === false)) emit("autoplay-sound", m, "autoplay without muted");
      if (m.tagName === "VIDEO" && !auto && !m.getAttribute("poster") && shown(m)) emit("video-blank-start", m, "no poster, no autoplay, preload=" + (m.getAttribute("preload") || "auto"));
    }
  });

  run("images", () => {
    const dpr = window.devicePixelRatio || 1;
    for (const img of D.querySelectorAll("img")) {
      const alt = img.getAttribute("alt");
      if (alt && (/\.(?:png|jpe?g|webp|gif|svg|avif|bmp|tiff?)$/i.test(alt.trim()) || /^(?:img|image|photo|picture|pic|dsc|screenshot)[\s_-]*\d*$/i.test(alt.trim()))) emit("alt-is-filename", img, 'alt="' + alt.slice(0, 40) + '"');
      if (!img.complete || !img.naturalWidth || !shown(img)) continue;
      const r = img.getBoundingClientRect();
      const fit = cs(img).objectFit;
      const need = Math.max(r.width, fit === "cover" ? r.height * (img.naturalWidth / img.naturalHeight) : 0) * Math.max(dpr, 1);
      if (img.naturalWidth >= 1600 && need > 0 && img.naturalWidth > need * 2.5 && r.width >= 40) {
        emit("raster-oversized", img, img.naturalWidth + "x" + img.naturalHeight + " source for a " + Math.round(r.width) + "x" + Math.round(r.height) + " slot (" + (img.naturalWidth / need).toFixed(1) + "x the pixels needed at " + dpr + "dppx)");
      }
    }
  });

  run("blend-mode-text", () => {
    for (const el of all) {
      const mb = cs(el).mixBlendMode;
      if (!/difference|exclusion|overlay|screen|color-dodge|lighten|darken/.test(mb)) continue;
      if (textOf(el).length >= 3 && shown(el) && !el.closest("[aria-hidden='true']")) emit("blend-mode-text", el, "mix-blend-mode: " + mb);
    }
  });

  run("focus-ring-missing", () => {
    if (!D.hasFocus || opts.skipFocus) return;
    const prev = D.activeElement;
    const freeze = D.createElement("style");
    freeze.textContent = "*, *::before, *::after { transition: none !important; animation-play-state: paused !important; }";
    D.head.appendChild(freeze);
    const sx = window.scrollX, sy = window.scrollY;
    const PROPS = ["outlineStyle", "outlineWidth", "outlineColor", "boxShadow", "borderTopColor", "borderBottomColor", "borderBottomWidth", "backgroundColor", "color", "textDecorationLine", "transform"];
    const snap = (el) => {
      const s = cs(el);
      const before = getComputedStyle(el, "::before"), after = getComputedStyle(el, "::after");
      return PROPS.map((p) => s[p]).join("|") + "|" + before.content + before.opacity + before.backgroundColor + "|" + after.content + after.opacity + after.backgroundColor;
    };
    let checked = 0;
    try {
      for (const el of controls) {
        if (checked >= 14) break;
        if (!shown(el) || el.disabled || el.getAttribute("tabindex") === "-1") continue;
        const rest = snap(el);
        const parentRest = el.parentElement ? snap(el.parentElement) : "";
        el.focus({ preventScroll: true });
        if (D.activeElement !== el) continue;
        if (!el.matches(":focus-visible")) { el.blur(); continue; }
        checked++;
        const focused = snap(el);
        const parentFocused = el.parentElement ? snap(el.parentElement) : "";
        el.blur();
        if (focused === rest && parentFocused === parentRest) emit("focus-ring-missing", el, "no computed style change between resting and :focus-visible (outline, shadow, border, background, colour, pseudo-elements)");
      }
    } finally {
      freeze.remove();
      if (prev && prev.focus && prev !== D.body) prev.focus({ preventScroll: true });
      else if (D.activeElement && D.activeElement.blur) D.activeElement.blur();
      window.scrollTo(sx, sy);
    }
  });

  run("rtl", () => {
    const lang = (D.documentElement.getAttribute("lang") || "").toLowerCase();
    const rtlLang = /^(ar|he|iw|fa|ur|ps|sd|ug|yi|dv|ckb)(-|$)/.test(lang);
    const dir = (D.documentElement.getAttribute("dir") || D.body.getAttribute("dir") || "").toLowerCase();
    if (rtlLang && dir !== "rtl") emit("rtl-dir-missing", D.documentElement, 'lang="' + lang + '", dir="' + (dir || "unset") + '"');
    const rtlPage = dir === "rtl" || rtlLang || cs(D.body).direction === "rtl";
    if (!rtlPage) return;
    const PHYS = /(?:^|[;\s{])(margin-(?:left|right)|padding-(?:left|right)|border-(?:left|right)(?:-[a-z]+)?|(?:left|right)\s*:|text-align\s*:\s*(?:left|right)|float\s*:\s*(?:left|right))/i;
    const found = [];
    let n = 0;
    const walk = (rules) => {
      for (const r of Array.from(rules || [])) {
        if (r.cssRules && !r.selectorText) { walk(r.cssRules); continue; }
        if (!r.selectorText || !r.style) continue;
        if (/\[dir=["']?ltr|:dir\(ltr\)/i.test(r.selectorText)) continue;
        const txt = r.style.cssText;
        const m = PHYS.exec(" " + txt);
        if (m) {
          n++;
          if (found.length < 4) found.push(r.selectorText.slice(0, 40) + " { " + m[1].replace(/\s*:$/, "") + " }");
        }
      }
    };
    for (const sh of Array.from(D.styleSheets)) {
      try { walk(sh.cssRules); } catch (e) { /* cross-origin sheet */ }
    }
    if (n >= 3) emit("rtl-physical-props", D.documentElement, n + " rule(s) with physical left/right properties", n + " CSS rule(s) use physical left/right on a right-to-left page, e.g. " + found.join("; "));
  });

  run("sidebar-vanishes", () => {
    if (vw < 700 || vw >= 1280) return;
    const cands = Array.from(D.querySelectorAll("aside, nav, [class*='sidebar' i], [class*='side-nav' i], [class*='sidenav' i]")).filter((el) => {
      if (el.parentElement && el.parentElement.closest("aside, [class*='sidebar' i]")) return false;
      return el.querySelectorAll("a[href]").length >= 4 && !el.closest("header, footer, [role='banner'], [role='contentinfo']");
    });
    for (const el of cands) {
      const links = Array.from(el.querySelectorAll("a[href]"));
      const r = el.getBoundingClientRect();
      const s = cs(el);
      const gone = s.display === "none" || s.visibility === "hidden" || r.width < 8 || r.right <= 0 || r.left >= vw || links.every((a) => !shown(a));
      if (!gone) continue;
      const hrefs = new Set(links.map((a) => a.getAttribute("href")));
      const elsewhere = Array.from(D.querySelectorAll("a[href]")).some((a) => !el.contains(a) && hrefs.has(a.getAttribute("href")) && shown(a));
      if (elsewhere) continue;
      emit("sidebar-vanishes", el, links.length + " sidebar links hidden at " + vw + "px, no icon rail and no other copy of the links visible");
    }
  });

  // ---- Typography -------------------------------------------------------------------------------
  run("line-shape", () => {
    let seen = 0;
    for (const el of D.querySelectorAll("h1, h2, h3, p, li, blockquote, figcaption")) {
      if (seen > 160) break;
      if (!shown(el) || el.closest("nav, header, footer, button, a, [aria-hidden='true']")) continue;
      const txt = textOf(el);
      const heading = /^H[1-3]$/.test(el.tagName);
      if (txt.length < (heading ? 12 : 60) || txt.split(" ").length < 4) continue;
      const s = cs(el);
      const wrap = (s.textWrap || s.textWrapStyle || "") + " " + (s.textWrapStyle || "");
      if (/balance|pretty/.test(wrap)) continue;
      seen++;
      const range = D.createRange();
      range.selectNodeContents(el);
      const rects = Array.from(range.getClientRects()).filter((q) => q.width > 1 && q.height > 1);
      // Merge fragments into lines by baseline band
      const lines = [];
      for (const q of rects) {
        const line = lines.find((l) => Math.abs(l.top - q.top) < q.height * 0.5);
        if (line) { line.left = Math.min(line.left, q.left); line.right = Math.max(line.right, q.right); }
        else lines.push({ top: q.top, left: q.left, right: q.right });
      }
      if (lines.length < 2 || lines.length > (heading ? 5 : 14)) continue;
      lines.sort((a, b) => a.top - b.top);
      const widths = lines.map((l) => l.right - l.left);
      const longest = Math.max(...widths);
      const last = widths[widths.length - 1];
      const lastWords = txt.split(" ").slice(-2);
      const lastWordW = (() => {
        const tw = D.createTreeWalker(el, NodeFilter.SHOW_TEXT);
        let lastNode = null;
        for (let n = tw.nextNode(); n; n = tw.nextNode()) if (n.nodeValue.trim()) lastNode = n;
        if (!lastNode) return null;
        const v = lastNode.nodeValue.replace(/\s+$/, "");
        const i = v.lastIndexOf(" ");
        const rr = D.createRange();
        rr.setStart(lastNode, i + 1);
        rr.setEnd(lastNode, v.length);
        return rr.getBoundingClientRect().width;
      })();
      if (lastWordW !== null && Math.abs(last - lastWordW) < 2 && last < longest * 0.25) {
        emit("straggler-line", el, '"' + lastWords[lastWords.length - 1] + '" alone on line ' + lines.length + " (" + Math.round(last) + "px of " + Math.round(longest) + "px)");
      } else if (heading && lines.length <= 4 && last < longest * 0.45) {
        emit("heading-unbalanced", el, lines.length + " lines, last line " + Math.round((last / longest) * 100) + "% of the longest");
      }
    }
  });

  run("font-size-sprawl", () => {
    const sizes = new Map();
    for (const el of all) {
      if (!ownText(el) || !shown(el) || el.closest("svg, code, pre, [aria-hidden='true']")) continue;
      const k = Math.round(px(cs(el).fontSize) * 2) / 2;
      if (!sizes.has(k)) sizes.set(k, el);
    }
    if (sizes.size > 10) emit("font-size-sprawl", D.body, sizes.size + " distinct text sizes: " + Array.from(sizes.keys()).sort((a, b) => a - b).join("/") + "px");
  });

  run("mono-running-text", () => {
    const hits = Array.from(D.querySelectorAll("p, li, dd, blockquote")).filter((el) => shown(el) && !el.closest("pre, code, kbd, samp, table") && textOf(el).length >= 90 && /mono|courier|consolas|menlo|monaco/i.test(cs(el).fontFamily));
    if (hits.length >= 2) emit("mono-running-text", hits[0], hits.length + " paragraph(s) of 90+ characters in " + cs(hits[0]).fontFamily.split(",")[0]);
  });

  run("figures", () => {
    for (const table of Array.from(D.querySelectorAll("table")).slice(0, 8)) {
      if (!shown(table)) continue;
      const cells = Array.from(table.querySelectorAll("td")).filter((td) => /^[\s$€£¥+\-−(]*[\d.,]+\s*%?[)]?\s*[A-Za-z]{0,3}$/.test(textOf(td)) && textOf(td).length >= 2);
      if (cells.length < 6) continue;
      const prop = cells.filter((td) => !/tabular-nums/.test(cs(td).fontVariantNumeric) && !/mono/i.test(cs(td).fontFamily));
      if (prop.length >= cells.length * 0.6) emit("figures-proportional", table, prop.length + " of " + cells.length + " numeric cells in proportional figures");
      const left = cells.filter((td) => /^(left|start|-webkit-left)$/.test(cs(td).textAlign) && cs(D.documentElement).direction !== "rtl");
      if (left.length >= cells.length * 0.6) emit("numbers-left-aligned", table, left.length + " of " + cells.length + " numeric cells left-aligned");
    }
  });

  // ---- Style habits (brief-aware) -------------------------------------------------------------
  const boxes = all.filter((el) => {
    const r = el.getBoundingClientRect();
    return r.width >= 120 && r.height >= 80 && r.width < vw * 0.98 && shown(el);
  });
  run("fx-gradient-surfaces", () => {
    const g = boxes.filter((el) => {
      const s = cs(el);
      return /gradient\(/.test(s.backgroundImage) && !/\btext\b/.test(s.backgroundClip + " " + s.webkitBackgroundClip) && !/url\(/.test(s.backgroundImage);
    });
    const big = g.filter((el) => { const r = el.getBoundingClientRect(); return r.width * r.height >= 40000; });
    if (big.length >= 3) emit("fx-gradient-surfaces", big[0], big.length + " surfaces of 200x200px+ painted with gradients");
  });
  run("fx-glass", () => {
    const g = all.filter((el) => {
      const s = cs(el);
      const bf = s.backdropFilter || s.webkitBackdropFilter || "";
      const bg = rgba(s.backgroundColor);
      return /blur\(\s*[1-9]/.test(bf) && bg && bg[3] < 0.9 && shown(el);
    });
    if (g.length >= 3) emit("fx-glass", g[0], g.length + " translucent blurred panels (backdrop-filter blur, background alpha < 0.9)");
  });
  // Box-shadow layers: [x, y, blur, spread, inset]
  const shadowLayers = (v) => {
    if (!v || v === "none") return [];
    return v.split(/,(?![^(]*\))/).map((layer) => {
      const nums = (layer.replace(/rgba?\([^)]*\)|#[0-9a-f]+|[a-z-]+\([^)]*\)/gi, "").match(/-?[\d.]+px|\b0\b/g) || []).map(px);
      const c = rgba((/rgba?\([^)]*\)/i.exec(layer) || [""])[0]);
      return { x: nums[0] || 0, y: nums[1] || 0, blur: nums[2] || 0, spread: nums[3] || 0, inset: /inset/.test(layer), alpha: c ? c[3] : 1 };
    });
  };
  run("fx-shadows", () => {
    const cards = boxes.filter((el) => {
      const s = cs(el);
      const bg = rgba(s.backgroundColor);
      return (bg && bg[3] > 0.5) || px(s.borderTopLeftRadius) >= 4 || s.boxShadow !== "none";
    });
    const soft = cards.filter((el) => shadowLayers(cs(el).boxShadow).some((l) => !l.inset && l.blur >= 24 && l.alpha >= 0.06));
    if (cards.length >= 4 && soft.length >= 4 && soft.length / cards.length >= 0.5) emit("fx-resting-shadows", soft[0], soft.length + " of " + cards.length + " boxes carry a soft shadow (blur >= 24px) at rest");
    const hard = all.filter((el) => shown(el) && shadowLayers(cs(el).boxShadow).some((l) => !l.inset && l.blur === 0 && l.spread === 0 && Math.max(Math.abs(l.x), Math.abs(l.y)) >= 3 && l.alpha >= 0.5));
    if (hard.length >= 3) emit("hard-offset-shadow", hard[0], hard.length + " elements with zero-blur offset shadows (" + cs(hard[0]).boxShadow.slice(0, 48) + ")");
  });
  run("fx-text-shadow", () => {
    const t = all.filter((el) => ownText(el) && shown(el) && cs(el).textShadow !== "none" && textOf(el).length >= 3);
    if (t.length >= 4) emit("fx-text-shadow", t[0], t.length + " text elements with text-shadow (" + cs(t[0]).textShadow.slice(0, 48) + ")");
  });
  run("pills", () => {
    const ctl = Array.from(D.querySelectorAll("button, a[class*='btn' i], a[class*='button' i], [role='button'], input[type=submit], input[type=button], input[type=text], input[type=email], input[type=search], select, .tag, .chip, .badge")).filter((el) => {
      const r = el.getBoundingClientRect();
      return shown(el) && r.height >= 20 && r.height <= 72 && r.width >= 40;
    });
    if (ctl.length >= 5) {
      const pill = ctl.filter((el) => px(cs(el).borderTopLeftRadius) >= el.getBoundingClientRect().height / 2 - 1);
      const kinds = new Set(pill.map((el) => (el.tagName === "INPUT" || el.tagName === "SELECT" ? "field" : /tag|chip|badge/i.test(el.className) ? "chip" : "button")));
      if (pill.length / ctl.length >= 0.75 && kinds.size >= 2) emit("pill-everything", pill[0], pill.length + " of " + ctl.length + " controls fully rounded across " + Array.from(kinds).join(", "));
    }
    const nav = Array.from(D.querySelectorAll("header, nav, [role='banner']")).find((el) => {
      const s = cs(el);
      const r = el.getBoundingClientRect();
      return /fixed|sticky/.test(s.position) && shown(el) && r.top >= 8 && r.width < vw * 0.92 && r.height <= 96 && px(s.borderTopLeftRadius) >= r.height / 2 - 2;
    });
    if (nav) emit("nav-floating-pill", nav, "detached " + cs(nav).position + " nav, radius " + cs(nav).borderTopLeftRadius + ", " + Math.round(nav.getBoundingClientRect().width) + "px wide");
  });
  run("radius-sprawl", () => {
    const radii = new Map();
    for (const el of boxes) {
      const r = Math.round(px(cs(el).borderTopLeftRadius));
      if (r > 0 && r < 999 && r < el.getBoundingClientRect().height / 2 - 1) radii.set(r, (radii.get(r) || 0) + 1);
    }
    const used = Array.from(radii.entries()).filter(([, n]) => n >= 1);
    if (used.length >= 7) emit("radius-sprawl", D.body, used.length + " different radii on boxes: " + used.map(([r]) => r).sort((a, b) => a - b).join("/") + "px");
  });
  run("link-default-blue", () => {
    const blue = Array.from(D.querySelectorAll("a[href]")).filter((a) => shown(a) && /^rgb\((0, 0, 238|85, 26, 139|0, 0, 255)\)$/.test(cs(a).color) && textOf(a).length > 1);
    if (blue.length >= 2) emit("link-default-blue", blue[0], blue.length + " link(s) in " + cs(blue[0]).color);
  });
  run("emoji-icons", () => {
    const EMOJI = /[\u{1F300}-\u{1FAFF}\u{2600}-\u{27BF}\u{1F000}-\u{1F2FF}]/u;
    const hits = Array.from(D.querySelectorAll("h2, h3, h4, li, button, a, .icon, [class*='icon' i], span, div")).filter((el) => {
      if (!ownText(el) || !shown(el)) return false;
      const t = Array.from(el.childNodes).filter((n) => n.nodeType === 3).map((n) => n.nodeValue).join("").trim();
      return EMOJI.test(t.slice(0, 3)) && !el.closest("blockquote, [class*='review' i], [class*='comment' i]");
    });
    if (hits.length >= 3) emit("emoji-icons", hits[0], hits.length + " elements start with an emoji glyph (" + textOf(hits[0]).slice(0, 2) + ")");
  });

  // ---- Layout and content -----------------------------------------------------------------------
  run("imagery-scarce", () => {
    if (docH < vh * 3 || D.querySelector('meta[name="layoza-surface"][content="app" i]')) return;
    const media = Array.from(D.querySelectorAll("img, picture, video, canvas, svg, iframe, [style*='background-image']")).filter((m) => {
      if (m.parentElement && m.parentElement.closest("svg")) return false;
      const r = m.getBoundingClientRect();
      return r.width >= 160 && r.height >= 120 && shown(m);
    });
    const bgArt = boxes.filter((el) => /url\(/.test(cs(el).backgroundImage)).length;
    if (media.length + bgArt === 0) emit("imagery-scarce", D.body, "page " + Math.round(docH / vh) + " screens tall with no image, video, canvas or artwork of 160x120px+");
  });
  run("cta-competition", () => {
    const pageBg = rgba(cs(D.body).backgroundColor) || [255, 255, 255, 1];
    const filled = Array.from(D.querySelectorAll("a[href], button, [role='button']")).filter((el) => {
      const r = el.getBoundingClientRect();
      if (!shown(el) || r.top + pageTop > vh || r.height < 36 || r.width < 80 || el.closest("nav, header")) return false;
      const bg = rgba(cs(el).backgroundColor);
      return bg && bg[3] > 0.8 && Math.abs(bg[0] - pageBg[0]) + Math.abs(bg[1] - pageBg[1]) + Math.abs(bg[2] - pageBg[2]) > 90;
    });
    const looks = new Set(filled.map((el) => cs(el).backgroundColor));
    if (filled.length >= 3 && looks.size <= 2) emit("cta-competition", filled[0], filled.length + " filled buttons on the first screen");
  });
  run("row-heights-ragged", () => {
    let flagged = 0;
    for (const parent of all) {
      if (flagged >= 3) break;
      const s = cs(parent);
      if (!/grid|flex/.test(s.display) || (s.display.includes("flex") && /column/.test(s.flexDirection))) continue;
      const kids = Array.from(parent.children).filter((c) => shown(c));
      if (kids.length < 3) continue;
      const rows = new Map();
      for (const k of kids) {
        const r = k.getBoundingClientRect();
        if (r.width < 140 || r.height < 120) continue;
        const key = Math.round(r.top / 4);
        if (!rows.has(key)) rows.set(key, []);
        rows.get(key).push({ k, r });
      }
      for (const row of rows.values()) {
        if (row.length < 3) continue;
        const painted = row.filter(({ k }) => { const ks = cs(k); const bg = rgba(ks.backgroundColor); return (bg && bg[3] > 0.3) || px(ks.borderTopWidth) > 0 || ks.boxShadow !== "none"; });
        if (painted.length < 3) continue;
        const hs = painted.map((x) => x.r.height);
        if (Math.max(...hs) > Math.min(...hs) * 1.15 && Math.max(...hs) - Math.min(...hs) > 24) {
          flagged++;
          emit("row-heights-ragged", parent, painted.length + " cards in one row, heights " + Math.round(Math.min(...hs)) + "-" + Math.round(Math.max(...hs)) + "px");
          break;
        }
      }
    }
  });
  run("scroller-hidden", () => {
    for (const el of all) {
      const s = cs(el);
      if (!/auto|scroll/.test(s.overflowX) || el.scrollWidth <= el.clientWidth + 40 || !shown(el) || el === D.body || el === D.documentElement) continue;
      const hiddenBar = s.scrollbarWidth === "none" || el.offsetHeight - el.clientHeight === 0;
      if (!hiddenBar) continue;
      const host = el.parentElement || el;
      const buttons = host.querySelectorAll("button, [role='button']").length;
      if (buttons === 0) emit("scroller-hidden", el, Math.round(el.scrollWidth - el.clientWidth) + "px of hidden horizontal content, no scrollbar, no buttons");
    }
  });
  run("skip-link-missing", () => {
    const main = D.querySelector("main, [role='main']");
    if (!main) return;
    const headerLinks = Array.from(D.querySelectorAll("a[href], button")).filter((a) => shown(a) && main.compareDocumentPosition(a) & Node.DOCUMENT_POSITION_PRECEDING);
    if (headerLinks.length < 6) return;
    const first = Array.from(D.querySelectorAll("a[href]")).slice(0, 3);
    if (!first.some((a) => /^#./.test(a.getAttribute("href") || "") && /skip|main|content/i.test(textOf(a) + " " + a.getAttribute("href")))) emit("skip-link-missing", D.body, headerLinks.length + " focusable items before <main>, no skip link");
  });

  // ---- Legibility floors ------------------------------------------------------------------------
  run("text-below-floor", () => {
    const SKIP = "footer, small, sub, sup, code, kbd, samp, pre, [aria-hidden='true'], [class*='legal' i], [class*='copyright' i], [class*='fine' i], [class*='disclaimer' i]";
    let body = null, bodyN = 0, label = null, labelN = 0;
    for (const el of D.body.querySelectorAll("p, li, dd, blockquote, td")) {
      if (!ownText(el) || !shown(el) || el.closest(SKIP) || textOf(el).length < 60) continue;
      const f = px(cs(el).fontSize);
      if (f > 0 && f < 14) { bodyN++; if (!body || f < px(cs(body).fontSize)) body = el; }
    }
    for (const el of D.body.querySelectorAll("label, button, input:not([type=hidden]), select, textarea, nav a, [role='button'], [role='tab'], legend, th")) {
      if (!shown(el) || el.closest(SKIP)) continue;
      const hasText = el.matches("input, select, textarea") || textOf(el).length >= 2;
      const f = px(cs(el).fontSize);
      if (hasText && f > 0 && f < 12) { labelN++; if (!label || f < px(cs(label).fontSize)) label = el; }
    }
    if (body) emit("text-below-floor", body, bodyN + " running-text block(s) under 14px, smallest " + cs(body).fontSize);
    if (label) emit("text-below-floor", label, labelN + " label/control(s) under 12px, smallest " + cs(label).fontSize);
  });

  // ---- Template decoration ------------------------------------------------------------------------
  const capsLabel = (el, headingPx) => {
    if (!el || !shown(el)) return false;
    const t = textOf(el);
    if (t.length < 2 || t.length > 40 || el.querySelector("h1, h2, h3, img, svg, button")) return false;
    const s = cs(el);
    const upper = s.textTransform === "uppercase" || (t === t.toUpperCase() && t !== t.toLowerCase());
    return upper && px(s.fontSize) <= headingPx * 0.6;
  };
  run("eyebrow-on-most-headings", () => {
    const hs = Array.from(D.body.querySelectorAll("h2")).filter((h) => shown(h) && textOf(h).length > 2);
    if (hs.length < 3) return;
    const with_ = hs.filter((h) => {
      const hp = px(cs(h).fontSize);
      let prev = h.previousElementSibling;
      if (!prev && h.parentElement) prev = h.parentElement.previousElementSibling;
      return capsLabel(prev, hp);
    });
    if (with_.length / hs.length > 0.6) emit("eyebrow-on-most-headings", with_[0], with_.length + " of " + hs.length + " h2 headings carry an all-caps label above them");
  });
  run("numbered-non-sequence", () => {
    const STEP = /\b(step|steps|stage|phase|process|how it works|first|then|next|finally|timeline|roadmap|journey|day \d|week \d|round|chapter|part)\b/i;
    const seen = new Set();
    for (const parent of all) {
      const kids = Array.from(parent.children).filter(shown);
      if (kids.length < 3 || seen.has(parent)) continue;
      // Zero-padded "01", "02", "03" leading each of the first cards, in order
      const lead = kids.map((k) => { const m = /^\s*0([1-9])(?=[\s.):]|$)/.exec(textOf(k)); return m ? Number(m[1]) : null; });
      const seq = [];
      for (const n of lead) { if (n !== seq.length + 1) break; seq.push(n); }
      if (seq.length < 3) continue;
      seen.add(parent);
      const section = parent.closest("section, article, main") || parent;
      const head = section.querySelector("h1, h2, h3");
      const context = (head ? textOf(head) : "") + " " + textOf(parent).slice(0, 600);
      if (STEP.test(context) || parent.closest("ol")) continue;
      emit("numbered-non-sequence", parent, kids.length + " cards numbered 01-" + String(seq.length).padStart(2, "0") + " with no step or process wording nearby");
    }
  });
  run("illustrated-people", () => {
    const WORD = /\b(person|people|man|woman|user|team|character|avatar|illustration|humaaan|figure|worker)\b/i;
    const hits = Array.from(D.body.querySelectorAll("svg, img[src$='.svg' i]")).filter((el) => {
      if (!shown(el)) return false;
      const r = el.getBoundingClientRect();
      if (r.width < 160 || r.height < 160) return false;
      const label = [el.getAttribute("aria-label"), el.getAttribute("alt"), el.getAttribute("class"), el.getAttribute("src"), el.querySelector && el.querySelector("title") ? textOf(el.querySelector("title")) : ""].join(" ");
      if (!WORD.test(label)) return false;
      if (el.tagName.toLowerCase() === "svg") return el.querySelectorAll("path").length >= 12;
      return true;
    });
    if (hits.length) emit("illustrated-people", hits[0], hits.length + " large vector figure(s) of people (" + Math.round(hits[0].getBoundingClientRect().width) + "px)");
  });
  run("violet-blue-hero", () => {
    const hueOf = (rgb) => {
      const [r, g, b] = rgb.map((v) => v / 255);
      const mx = Math.max(r, g, b), mn = Math.min(r, g, b), d = mx - mn;
      if (d < 0.12) return null;
      let h = mx === r ? ((g - b) / d) % 6 : mx === g ? (b - r) / d + 2 : (r - g) / d + 4;
      return (h * 60 + 360) % 360;
    };
    const stopsOf = (bi) => {
      const out = [];
      for (let i = bi.indexOf("rgb"); i >= 0 && out.length < 8; i = bi.indexOf("rgb", i + 3)) {
        const inner = bi.slice(bi.indexOf("(", i) + 1, bi.indexOf(")", i)).split(/[ ,/]+/).filter(Boolean).slice(0, 3).map(Number);
        if (inner.length === 3 && inner.every((n) => n >= 0)) out.push(inner);
      }
      return out;
    };
    for (const el of [D.body, ...all]) {
      const r = el.getBoundingClientRect();
      if (r.top > vh * 0.5 || r.width < vw * 0.6 || r.height < vh * 0.4 || !shown(el)) continue;
      const bi = cs(el).backgroundImage || "";
      if (bi.indexOf("gradient") < 0) continue;
      const hues = stopsOf(bi).map(hueOf).filter((h) => h !== null);
      const violet = hues.some((h) => h >= 255 && h <= 320), blue = hues.some((h) => h >= 190 && h < 255);
      if (violet && blue) { emit("violet-blue-hero", el, "first-screen gradient through hues " + hues.map((h) => Math.round(h)).join("/") + "°"); return; }
    }
  });

  // ---- Fabricated-content heuristics (phrased as "verify this is real") ---------------------------
  run("logo-wall-unverified", () => {
    const GENERIC = /(^|[^a-z])(logo[ _-]?\d+|company[ _-]?\d*|client[ _-]?\d+|brand[ _-]?\d+|partner[ _-]?\d+|acme|placeholder|lorem|logoipsum|your[ _-]?logo|sample)([^a-z]|$)/i;
    for (const parent of all) {
      const imgs = Array.from(parent.children).map((c) => (c.matches("img, svg") ? c : c.querySelector("img, svg"))).filter((m) => m && shown(m));
      if (imgs.length < 4) continue;
      const hs = imgs.map((m) => Math.round(m.getBoundingClientRect().height));
      if (Math.max(...hs) > 120) continue;
      const names = imgs.map((m) => [m.getAttribute("alt"), m.getAttribute("src"), m.getAttribute("aria-label")].join(" "));
      const generic = names.filter((n) => GENERIC.test(n)).length;
      const blank = imgs.filter((m) => m.tagName === "IMG" && !(m.getAttribute("alt") || "").trim()).length;
      if (generic >= 2 || (blank === imgs.length && /logo|client|partner|trusted|brand/i.test(parent.className + " " + textOf(parent.parentElement || parent).slice(0, 200)))) {
        emit("logo-wall-unverified", parent, imgs.length + " logos, " + generic + " generic/placeholder name(s), " + blank + " without alt text");
        return;
      }
    }
  });
  run("testimonial-unsourced", () => {
    const STOCK = /\b(john|jane) (doe|smith)\b|\b(sarah johnson|mike johnson|emily (davis|chen)|alex (morgan|johnson)|david (lee|kim)|lisa (wong|park)|happy customer|satisfied (client|customer)|anonymous)\b/i;
    const cards = Array.from(D.body.querySelectorAll("blockquote, figure, [class*='testimonial' i], [class*='review' i], [class*='quote' i]")).filter((el) => shown(el) && textOf(el).length > 40 && !el.parentElement.closest("[class*='testimonial' i], [class*='review' i], blockquote, figure"));
    if (!cards.length) return;
    const stock = cards.filter((c) => STOCK.test(textOf(c)));
    const bare = cards.filter((c) => !c.querySelector("cite, figcaption, a[href], img, [class*='role' i], [class*='company' i], [class*='title' i], [class*='org' i]") && !/,|—|–|\|/.test(textOf(c).slice(-60)));
    if (stock.length) emit("testimonial-unsourced", stock[0], stock.length + " testimonial(s) attributed to stock names");
    else if (bare.length >= 2) emit("testimonial-unsourced", bare[0], bare.length + " of " + cards.length + " testimonials have no role, company, link or photo");
  });
  run("stats-too-round", () => {
    const STAT = /^\s*[$€£]?\s*(\d{1,4})(?:[.,](\d+))?\s*([kKmMbB]|%|x)?\s*\+?\s*$/;
    const vals = [];
    for (const el of all) {
      if (!ownText(el) || !shown(el)) continue;
      const t = textOf(el);
      if (t.length > 8) continue;
      const m = STAT.exec(t);
      if (!m || px(cs(el).fontSize) < 28) continue;
      const n = Number(m[1]);
      const round = !m[2] && (n % 10 === 0 || n === 99 || n === 24 || n === 7 || (m[3] && n < 10)) && (t.includes("+") || m[3]);
      vals.push({ el, t, round });
    }
    if (vals.length >= 3 && vals.filter((v) => v.round).length / vals.length >= 0.75) emit("stats-too-round", vals[0].el, vals.map((v) => v.t).slice(0, 5).join(", "));
  });

  // ---- Stylesheet-driven motion and state checks ------------------------------------------------
  const cssRules = [];
  const walkRules = (list, media) => {
    for (const r of Array.from(list || [])) {
      if (cssRules.length > 8000) return;
      if (r.cssRules && !r.selectorText && r.type !== 7) walkRules(r.cssRules, media || (r.conditionText || (r.media && r.media.mediaText) || ""));
      else cssRules.push({ r, media: media || "" });
    }
  };
  for (const sh of Array.from(D.styleSheets)) { try { walkRules(sh.cssRules, ""); } catch (e) { /* cross-origin sheet */ } }
  const reduceMedia = (m) => /prefers-reduced-motion/.test(m);
  run("hover-motion", () => {
    const scaleBy = new Map();
    const stacked = [];
    for (const { r, media } of cssRules) {
      const sel = r.selectorText || "";
      if (sel.indexOf(":hover") < 0 || !r.style || reduceMedia(media)) continue;
      const tf = r.style.getPropertyValue("transform") || r.style.getPropertyValue("scale");
      const m = /scale\(\s*([\d.]+)/.exec(tf) || (/^\s*[\d.]+\s*$/.test(r.style.getPropertyValue("scale")) ? [0, r.style.getPropertyValue("scale").trim()] : null);
      if (m && Number(m[1]) !== 1) {
        const key = Number(m[1]).toFixed(3);
        const set = scaleBy.get(key) || new Set();
        sel.split(",").forEach((s) => set.add(s.trim()));
        scaleBy.set(key, set);
      }
      const kinds = new Set();
      for (let i = 0; i < r.style.length; i++) {
        const p = r.style[i];
        if (/^(transform|translate|scale|rotate)$/.test(p)) kinds.add("move");
        else if (/shadow/.test(p)) kinds.add("shadow");
        else if (/^background/.test(p)) kinds.add("fill");
        else if (p === "color") kinds.add("text");
        else if (/^border.*color$|^outline-color$/.test(p)) kinds.add("edge");
        else if (p === "filter" || p === "backdrop-filter") kinds.add("filter");
        else if (p === "opacity") kinds.add("fade");
      }
      if (kinds.size >= 3) stacked.push(sel.slice(0, 60) + " (" + Array.from(kinds).join("+") + ")");
    }
    for (const [k, set] of scaleBy) {
      if (set.size < 4) continue;
      const targets = Array.from(set).map((s) => s.replace(/:hover.*$/, "")).filter(Boolean);
      let hits = 0;
      for (const t of targets) { try { if (D.querySelector(t)) hits++; } catch (e) { /* invalid selector */ } }
      if (hits >= 4) { emit("hover-scale-uniform", D.body, "scale(" + Number(k) + ") on hover for " + hits + " different selectors (" + targets.slice(0, 4).join(", ") + ")"); break; }
    }
    if (stacked.length >= 2) emit("hover-effects-stacked", D.body, stacked.length + " hover rules change 3+ things at once, e.g. " + stacked.slice(0, 2).join("; "));
  });
  run("layout-property-animated", () => {
    const LAYOUT = /^(width|height|top|left|right|bottom|inset|margin|margin-(top|right|bottom|left)|padding|padding-(top|right|bottom|left)|max-height|min-height)$/;
    const hits = [];
    for (const { r, media } of cssRules) {
      if (reduceMedia(media) || r.type !== 7) continue;
      const props = new Set();
      for (const f of Array.from(r.cssRules || [])) for (let i = 0; i < f.style.length; i++) if (LAYOUT.test(f.style[i])) props.add(f.style[i]);
      if (props.size && Array.from(D.getAnimations ? D.getAnimations() : []).some((a) => a.animationName === r.name)) hits.push("@keyframes " + r.name + " (" + Array.from(props).join(", ") + ")");
    }
    let el0 = null;
    for (const el of all) {
      const s = cs(el);
      if (!s.transitionProperty || s.transitionProperty === "all" || s.transitionDuration === "0s") continue;
      const props = s.transitionProperty.split(",").map((p) => p.trim()).filter((p) => LAYOUT.test(p));
      const durs = s.transitionDuration.split(",").map((d) => parseFloat(d) || 0);
      if (props.length && Math.max(...durs) > 0) { if (!el0) el0 = el; if (hits.length < 6) hits.push(selectorOf(el) + " transition: " + props.join(", ")); }
    }
    if (hits.length) emit("layout-property-animated", el0 || D.body, hits.length + " animation(s) on layout properties", hits.slice(0, 3).join("; "));
  });
  run("focus-ring-animated", () => {
    for (const { r } of cssRules) {
      const sel = r.selectorText || "";
      if (!/:focus(-visible)?/.test(sel) || !r.style) continue;
      if (!(r.style.getPropertyValue("outline") || r.style.getPropertyValue("outline-width") || r.style.getPropertyValue("outline-style") || r.style.getPropertyValue("box-shadow"))) continue;
      let el = null;
      try { el = D.querySelector(sel.split(",")[0].replace(/:focus(-visible|-within)?/g, "")); } catch (e) { el = null; }
      if (!el || !shown(el)) continue;
      const s = cs(el);
      const props = s.transitionProperty.split(",").map((p) => p.trim());
      const durs = s.transitionDuration.split(",").map((d) => parseFloat(d) || 0);
      const i = props.findIndex((p) => p === "all" || p.startsWith("outline") || p === "box-shadow");
      const dur = i >= 0 ? durs[i % durs.length] : 0;
      if (dur >= 0.05) { emit("focus-ring-animated", el, "focus style transitions in over " + Math.round(dur * 1000) + "ms (" + props[i] + ")"); return; }
    }
  });
  run("auto-rotate-no-pause", () => {
    const auto = D.querySelector("[data-autoplay]:not([data-autoplay='false']), [data-ride='carousel'], [data-bs-ride='carousel'], [data-swiper-autoplay], [data-flickity*='autoPlay'], [data-slick*='autoplay\":true'], [data-splide*='autoplay'], [data-glide-autoplay]");
    let host = auto;
    if (!host) {
      for (const a of D.getAnimations ? D.getAnimations() : []) {
        const t = a.effect && a.effect.target;
        if (!t || !t.children || t.children.length < 3 || a.effect.getComputedTiming().iterations !== Infinity) continue;
        const r = t.getBoundingClientRect();
        if (r.width > vw * 0.5 && textOf(t).length > 30) { host = t; break; }
      }
    }
    if (!host) return;
    const scope = host.closest("section, [class*='carousel' i], [class*='slider' i], [class*='marquee' i]") || host.parentElement || host;
    const pause = Array.from(scope.querySelectorAll("button, [role='button']")).some((b) => /pause|stop|play/i.test(textOf(b) + " " + (b.getAttribute("aria-label") || "") + " " + (b.getAttribute("class") || "")));
    const hoverPause = cssRules.some(({ r }) => /:hover|:focus-within/.test(r.selectorText || "") && r.style && /paused/.test(r.style.getPropertyValue("animation-play-state") + r.style.getPropertyValue("animation")));
    if (!pause && !hoverPause) emit("auto-rotate-no-pause", host, "auto-moving content with " + host.children.length + " items and no pause control or hover/focus pause");
  });
  run("control-states", () => {
    const disabled = Array.from(D.querySelectorAll("button:disabled, input:disabled, select:disabled, [aria-disabled='true']")).filter(shown);
    for (const d of disabled.slice(0, 6)) {
      const peer = Array.from(D.querySelectorAll(d.tagName)).find((p) => p !== d && !p.disabled && p.getAttribute("aria-disabled") !== "true" && shown(p) && (p.className === d.className));
      if (!peer) continue;
      const a = cs(d), b = cs(peer);
      if (a.opacity === b.opacity && a.color === b.color && a.backgroundColor === b.backgroundColor && a.cursor === b.cursor) { emit("disabled-unstyled", d, "disabled " + d.tagName.toLowerCase() + " matches its enabled twin (opacity " + a.opacity + ", colour " + a.color + ")"); break; }
    }
    const fields = Array.from(D.querySelectorAll("form input:not([type=hidden]):not([type=submit]):not([type=button]), form textarea, form select")).filter(shown);
    if (fields.length >= 2) {
      const errorStyled = cssRules.some(({ r }) => /:invalid|:user-invalid|aria-invalid|\.(is-)?(error|invalid|has-error)|\[data-error|:has\([^)]*invalid/i.test(r.selectorText || ""));
      if (!errorStyled) emit("field-states-missing", fields[0].form || fields[0], fields.length + " form fields, no :user-invalid / [aria-invalid] / .error style in any stylesheet");
    }
  });

  // ---- Content honesty -----------------------------------------------------------------------------
  const bodyText = (D.body.innerText || "").slice(0, 60000);
  run("placeholder-names", () => {
    const m = /\b(john doe|jane doe|john smith|jane smith|acme (corp|inc|co)\b|[\w.]+@example\.(com|org)|\(?555\)?[-. ]\d{3}[-. ]?\d{4}|123 main st)/i.exec(bodyText);
    if (m) emit("placeholder-names", D.body, '"' + m[0] + '" in the page text');
  });
  run("claim-unsourced", () => {
    const RE = /\b(\d+(?:\.\d+)?x (?:faster|more|better|cheaper|growth|roi)|trusted by (?:over |more than )?[\d.,]+\s*[kKmM]?\+?|\d{2,3}% (?:faster|more|less|increase|growth|of (?:customers|users|teams))|#1 (?:rated|choice|platform))/i;
    for (const el of all) {
      if (!ownText(el) || !shown(el)) continue;
      const t = Array.from(el.childNodes).filter((n) => n.nodeType === 3).map((n) => n.nodeValue).join(" ");
      const m = RE.exec(t);
      if (!m) continue;
      const box = el.closest("section, article, li, div") || el;
      if (box.querySelector("sup, cite, a[href*='source' i], a[href*='study' i], a[href*='case' i], [class*='source' i], [class*='footnote' i]") || /source:|according to|\bstudy\b|survey of/i.test(textOf(box))) continue;
      emit("claim-unsourced", el, '"' + m[0] + '" with no source nearby');
      return;
    }
  });
  run("mock-device-chrome", () => {
    for (const el of all) {
      const kids = Array.from(el.children);
      if (kids.length < 3 || kids.length > 4) continue;
      const dots = kids.slice(0, 3).filter((k) => { const r = k.getBoundingClientRect(); const s = cs(k); return r.width >= 6 && r.width <= 16 && Math.abs(r.width - r.height) <= 1 && px(s.borderTopLeftRadius) >= r.width / 2 - 1 && s.backgroundColor !== "rgba(0, 0, 0, 0)"; });
      if (dots.length === 3 && shown(el)) { emit("mock-device-chrome", el, "three " + Math.round(dots[0].getBoundingClientRect().width) + "px window dots drawn in CSS"); return; }
    }
    for (const el of all) {
      const r = el.getBoundingClientRect();
      const s = cs(el);
      if (r.height < 300 || r.width < 140) continue;
      const ratio = r.width / r.height;
      if (ratio > 0.42 && ratio < 0.56 && px(s.borderTopLeftRadius) >= 28 && px(s.borderTopWidth) >= 6 && el.querySelector("img, video") && shown(el)) { emit("mock-device-chrome", el, "drawn phone frame " + Math.round(r.width) + "x" + Math.round(r.height) + "px, " + s.borderTopWidth + " bezel"); return; }
    }
  });

  // ---- Colour, type and layout measures -----------------------------------------------------------
  const rgbOf = (c) => { const m = /rgba?\(([^)]+)\)/.exec(c || ""); if (!m) return null; const p = m[1].split(/[ ,/]+/).filter(Boolean).map(Number); return p.length >= 3 && (p.length < 4 || p[3] > 0.05) ? p : null; };
  run("neutrals-untinted", () => {
    let grey = 0, tinted = 0;
    for (const el of all.slice(0, 2500)) {
      if (!shown(el)) continue;
      const s = cs(el);
      for (const c of [s.backgroundColor, ownText(el) ? s.color : null]) {
        const p = rgbOf(c);
        if (!p) continue;
        const mx = Math.max(p[0], p[1], p[2]), mn = Math.min(p[0], p[1], p[2]);
        if (mx - mn > 24 || mx > 250 || mn < 6) continue;
        if (mx === mn) grey++; else tinted++;
      }
    }
    if (grey >= 8 && tinted === 0) emit("neutrals-untinted", D.body, grey + " grey fills/text colours, all exactly r=g=b");
  });
  run("accent-flood", () => {
    let area = 0;
    for (const el of all) {
      const r = el.getBoundingClientRect();
      if (r.top >= vh || r.bottom <= 0 || r.width * r.height < 400 || el.matches("img, video, canvas, svg, picture") || !shown(el)) continue;
      const p = rgbOf(cs(el).backgroundColor);
      if (!p) continue;
      const mx = Math.max(p[0], p[1], p[2]) / 255, mn = Math.min(p[0], p[1], p[2]) / 255;
      if (mx - mn < 0.45 || mx < 0.35) continue;
      const parent = el.parentElement ? rgbOf(cs(el.parentElement).backgroundColor) : null;
      if (parent && parent.slice(0, 3).join() === p.slice(0, 3).join()) continue;
      const h = Math.max(0, Math.min(r.bottom, vh) - Math.max(r.top, 0)), w = Math.max(0, Math.min(r.right, vw) - Math.max(r.left, 0));
      area += h * w;
    }
    const share = vw && vh ? area / (vw * vh) : 0;
    if (share > 0.25) emit("accent-flood", D.body, Math.round(share * 100) + "% of the first screen is saturated accent fill");
  });
  run("measure-narrow", () => {
    if (vw < 900) return;
    for (const el of D.body.querySelectorAll("p")) {
      const t = textOf(el);
      if (t.length < 280 || !shown(el)) continue;
      const range = D.createRange();
      range.selectNodeContents(el);
      const tops = new Set(Array.from(range.getClientRects()).map((r) => Math.round(r.top)));
      const perLine = tops.size ? t.length / tops.size : 0;
      if (tops.size >= 5 && perLine < 38) { emit("measure-narrow", el, "about " + Math.round(perLine) + " characters per line over " + tops.size + " lines at " + vw + "px"); return; }
    }
  });
  run("caps-display-tight", () => {
    for (const el of D.body.querySelectorAll("h1, h2, h3, [class*='display' i], [class*='hero' i] > *")) {
      if (!ownText(el) || !shown(el)) continue;
      const s = cs(el);
      const fs = px(s.fontSize), lh = s.lineHeight === "normal" ? fs * 1.15 : px(s.lineHeight);
      const t = textOf(el);
      const upper = s.textTransform === "uppercase" || (t === t.toUpperCase() && /[A-Z]/.test(t));
      if (!upper || fs < 32 || !lh) continue;
      const lines = Math.round(el.getBoundingClientRect().height / lh);
      if (lines >= 2 && lh / fs < 1.0) { emit("caps-display-tight", el, "uppercase " + Math.round(fs) + "px over " + lines + " lines at line-height " + (lh / fs).toFixed(2)); return; }
    }
  });
  run("sticky-under-header", () => {
    const header = all.find((el) => { const s = cs(el); const r = el.getBoundingClientRect(); return (s.position === "fixed" || s.position === "sticky") && r.top <= 1 && r.height >= 40 && r.height < vh * 0.3 && r.width > vw * 0.8 && shown(el); });
    if (!header) return;
    const hh = header.getBoundingClientRect().height;
    const under = all.find((el) => el !== header && !header.contains(el) && !el.contains(header) && cs(el).position === "sticky" && px(cs(el).top) < hh - 4 && el.getBoundingClientRect().height > 20 && shown(el) && cs(el).top !== "auto");
    if (under) emit("sticky-under-header", under, "sticky top " + cs(under).top + " under a " + Math.round(hh) + "px " + cs(header).position + " header");
  });

  // ---- Structural fingerprints -----------------------------------------------------------------------
  run("structure", () => {
    const nav = D.querySelector("header nav, header [role='navigation'], body > nav");
    const header = nav && (nav.closest("header") || nav.parentElement);
    if (header && vw >= 1024 && shown(header)) {
      const hr = header.getBoundingClientRect();
      const links = Array.from(header.querySelectorAll("a[href], button")).filter((a) => shown(a) && a.getBoundingClientRect().height < 80);
      const ctas = links.filter((a) => { const s = cs(a); return rgbOf(s.backgroundColor) && px(s.paddingLeft) >= 10; });
      const plain = links.filter((a) => !ctas.includes(a) && !a.querySelector("img, svg") && textOf(a).length > 1);
      const logo = links.find((a) => a.getBoundingClientRect().left < hr.left + hr.width * 0.2);
      if (logo && plain.length >= 4 && plain.length <= 6 && ctas.length === 1 && ctas[0].getBoundingClientRect().right > hr.left + hr.width * 0.8) {
        const mid = (Math.min(...plain.map((a) => a.getBoundingClientRect().left)) + Math.max(...plain.map((a) => a.getBoundingClientRect().right))) / 2;
        if (Math.abs(mid - (hr.left + hr.width / 2)) < hr.width * 0.12) emit("nav-default-layout", header, "logo left, " + plain.length + " centred links, 1 button right");
      }
    }
    const footer = D.querySelector("footer");
    if (footer) {
      const heads = Array.from(footer.querySelectorAll("h2, h3, h4, h5, h6, strong, [class*='title' i], [class*='heading' i]")).map((h) => textOf(h).toLowerCase());
      const stock = ["product", "company", "resources", "legal", "support", "solutions", "about"].filter((w) => heads.includes(w));
      if (stock.length >= 3) emit("footer-default-columns", footer, "footer columns titled " + stock.join(" / "));
    }
    const main = D.querySelector("main") || D.body;
    const secs = Array.from(main.children).filter((s) => shown(s) && s.getBoundingClientRect().height > 120 && !s.matches("header, footer, nav, script, style"));
    if (secs.length >= 3 && secs.length <= 5) {
      const cards = (s) => { for (const el of s.querySelectorAll("*")) { const k = Array.from(el.children).filter(shown); if (k.length === 3 && k.every((c) => { const r = c.getBoundingClientRect(); const r0 = k[0].getBoundingClientRect(); return Math.abs(r.width - r0.width) < 4 && Math.abs(r.top - r0.top) < 4 && r.width > 150; })) return true; } return false; };
      const ctaBand = (s) => s.querySelectorAll("a[href], button").length <= 3 && s.querySelector("h2, h3") && textOf(s).length < 400;
      if (cards(secs[1]) && ctaBand(secs[secs.length - 1])) emit("page-template-sequence", main, secs.length + " sections: hero, 3 equal cards, call-to-action band");
    }
  });

  // ---- Media, icons and graphics ---------------------------------------------------------------------
  run("hero-media", () => {
    const first = Array.from(D.images).filter((im) => { const r = im.getBoundingClientRect(); return r.top < vh && r.bottom > 0 && r.width * r.height > 0.15 * vw * vh && shown(im); });
    const lazy = first.find((im) => (im.getAttribute("loading") || "").toLowerCase() === "lazy");
    if (lazy) { const r = lazy.getBoundingClientRect(); emit("lcp-image-lazy", lazy, "loading=lazy on a " + Math.round(r.width) + "x" + Math.round(r.height) + "px first-screen image"); }
    const vid = Array.from(D.querySelectorAll("video[autoplay]")).find((v) => { const r = v.getBoundingClientRect(); return r.top < vh && r.width * r.height > 0.2 * vw * vh && !v.getAttribute("poster"); });
    if (vid) emit("hero-video-no-poster", vid, "autoplay video " + Math.round(vid.getBoundingClientRect().width) + "px wide with no poster");
  });
  run("icon-sets-mixed", () => {
    const SETS = { "font-awesome": "[class*='fa-'], .fa, .fas, .far, .fab", material: ".material-icons, .material-symbols-outlined, .material-symbols-rounded, [class*='material-icons']", bootstrap: "[class^='bi-'], [class*=' bi-']", lucide: "[class*='lucide'], [data-lucide]", heroicons: "[class*='heroicon']", feather: "[class*='feather'], [data-feather]", phosphor: "[class^='ph-'], [class*=' ph-'], .ph", remix: "[class^='ri-'], [class*=' ri-']", tabler: "[class^='ti-'], [class*=' ti-'], [class*='tabler']", ionicons: "ion-icon", iconify: "iconify-icon, .iconify" };
    const used = Object.entries(SETS).filter(([, q]) => { try { return D.querySelectorAll(q).length >= 2; } catch (e) { return false; } }).map(([k]) => k);
    if (used.length >= 2) emit("icon-sets-mixed", D.body, used.join(" + "));
  });
  run("graphic-unhidden", () => {
    const bad = Array.from(D.querySelectorAll("canvas, svg")).filter((g) => {
      if (g.parentElement && g.parentElement.closest("svg")) return false;
      if (g.closest("[aria-hidden='true'], [role='presentation'], [role='none']")) return false;
      if (g.getAttribute("role") === "img" || g.getAttribute("aria-label") || g.getAttribute("aria-labelledby") || (g.tagName.toLowerCase() === "svg" && g.querySelector("title"))) return false;
      if (g.closest("a, button, [role='button']")) return false;
      const r = g.getBoundingClientRect();
      return r.width >= 24 && r.height >= 24 && shown(g) && !(g.tagName === "CANVAS" && textOf(g));
    });
    if (bad.length) emit("graphic-unhidden", bad[0], bad.length + " canvas/svg graphic(s) with neither aria-hidden nor a label");
  });

  return { issues, suppressed, errors, allow: Array.from(allowSet), rule_count: Object.keys(RULES).length, subset: only ? Array.from(only) : null };
}

const QUALITY_EVAL = (opts) =>
  "(async () => { try { if (document.fonts) await document.fonts.ready; } catch (e) {} " +
  "const t0 = performance.now(); " +
  "while (performance.now() - t0 < 1500) { const busy = (document.getAnimations ? document.getAnimations() : []).filter((a) => a.playState === 'running' && a.effect && isFinite(a.effect.getComputedTiming().endTime)); if (!busy.length) break; await new Promise((r) => setTimeout(r, 100)); } " +
  "return (" + layozaQualitySweep.toString() + ")(" + JSON.stringify(opts || {}) + "); })()";

/**
 * Lists text that is invisible after a scroll through the page (opacity chain < 0.1 or visibility
 * hidden). Run twice on desktop, once normally and once with prefers-reduced-motion: reduce; text
 * hidden only in the reduced run is a reveal that reduced motion skips without showing the end state.
 */
async function layozaHiddenTextProbe() {
  const H = window.innerHeight;
  const total = document.documentElement.scrollHeight;
  const wait = (ms) => new Promise((r) => setTimeout(r, ms));
  for (let y = 0; y < total && y < H * 14; y += Math.round(H * 0.8)) {
    window.scrollTo({ top: y, left: 0, behavior: "instant" });
    await wait(140);
  }
  await wait(500);
  const sel = (el) => {
    const parts = [];
    for (let c = el, d = 0; c && c.nodeType === 1 && d < 3; c = c.parentElement, d++) {
      let p = c.tagName.toLowerCase();
      if (c.id) { parts.unshift(p + "#" + c.id); break; }
      const cls = (c.getAttribute("class") || "").trim().split(/\s+/).filter(Boolean).slice(0, 2);
      if (cls.length) p += "." + cls.join(".");
      else if (c.parentElement) {
        const same = Array.from(c.parentElement.children).filter((x) => x.tagName === c.tagName);
        if (same.length > 1) p += ":nth-of-type(" + (same.indexOf(c) + 1) + ")";
      }
      parts.unshift(p);
    }
    return parts.join(" > ");
  };
  const out = [];
  for (const el of document.querySelectorAll("h1, h2, h3, h4, p, li, blockquote, figcaption, a, button")) {
    if (out.length >= 40) break;
    const txt = (el.textContent || "").replace(/\s+/g, " ").trim();
    if (txt.length < 3) continue;
    if (el.closest("[aria-hidden='true'], [hidden], [inert], dialog, [popover], details:not([open]) > :not(summary), nav, [role='menu'], [role='tooltip'], [role='tabpanel']")) continue;
    const r = el.getBoundingClientRect();
    if (r.width < 2 || r.height < 2) continue;
    let op = 1, vis = true, gone = false;
    for (let c = el; c && c.nodeType === 1; c = c.parentElement) {
      const s = getComputedStyle(c);
      if (s.display === "none") { gone = true; break; }
      if (s.visibility === "hidden") vis = false;
      op *= parseFloat(s.opacity);
    }
    if (gone || (vis && op >= 0.1)) continue;
    out.push({ selector: sel(el), text: txt.slice(0, 40), opacity: Number(op.toFixed(2)), visibility: vis ? "visible" : "hidden" });
  }
  window.scrollTo({ top: 0, left: 0, behavior: "instant" });
  return out;
}
const HIDDEN_TEXT_PROBE = "(" + layozaHiddenTextProbe.toString() + ")()";
/** Long or endless animations still running (used under prefers-reduced-motion: reduce). */
function layozaRunningMotion() {
  return (document.getAnimations ? document.getAnimations() : [])
    .filter((a) => {
      if (a.playState !== "running" || !a.effect || !a.effect.target) return false;
      const t = a.effect.getComputedTiming();
      return t.iterations === Infinity || (Number(t.duration) || 0) > 1000;
    })
    .slice(0, 40)
    .map((a) => {
      const el = a.effect.target;
      const cls = typeof el.className === "string" ? el.className.trim().split(" ")[0] : "";
      return (el.tagName || "").toLowerCase() + (el.id ? "#" + el.id : cls ? "." + cls : "") + " " + (a.animationName || a.transitionProperty || "script animation");
    });
}
const RUNNING_MOTION_PROBE = "(" + layozaRunningMotion.toString() + ")()";
const OVERFLOW_PROBE = "(() => Math.max(0, document.documentElement.scrollWidth - document.documentElement.clientWidth))()";
/** Widths checked for sideways scroll on top of the three main viewports (320 is the WCAG reflow width). */
const EXTRA_WIDTHS = [320, 1024, 1920];

/**
 * Look fingerprint, part 1 (in page): the coarse layout of the first three screens as two 24x18
 * occupancy grids (text runs, media boxes), in page coordinates so scroll position does not matter.
 */
function layozaLayoutVector() {
  const W = document.documentElement.clientWidth || window.innerWidth;
  const H = Math.min(document.documentElement.scrollHeight, window.innerHeight * 3);
  const cols = 24, rows = 18;
  const text = new Array(cols * rows).fill(0), media = new Array(cols * rows).fill(0);
  const y0 = window.scrollY;
  const mark = (grid, r) => {
    const top = r.top + y0, bottom = r.bottom + y0;
    if (bottom <= 0 || top >= H || r.right <= 0 || r.left >= W) return;
    const cx0 = Math.max(0, Math.floor((r.left / W) * cols)), cx1 = Math.min(cols - 1, Math.floor(((r.right - 1) / W) * cols));
    const ry0 = Math.max(0, Math.floor((top / H) * rows)), ry1 = Math.min(rows - 1, Math.floor(((bottom - 1) / H) * rows));
    for (let y = ry0; y <= ry1; y++) for (let x = cx0; x <= cx1; x++) grid[y * cols + x] = 1;
  };
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
  let n = 0;
  for (let t = walker.nextNode(); t && n < 3000; t = walker.nextNode()) {
    if (!(t.nodeValue || "").trim()) continue;
    const el = t.parentElement;
    if (!el || el.closest("script, style, noscript, template, [hidden], [aria-hidden='true']")) continue;
    const range = document.createRange();
    range.selectNodeContents(t);
    for (const r of range.getClientRects()) if (r.width > 2 && r.height > 2) { mark(text, r); n++; }
  }
  for (const m of document.querySelectorAll("img, picture, video, canvas, svg, iframe")) {
    if (m.parentElement && m.parentElement.closest("svg")) continue;
    const r = m.getBoundingClientRect();
    if (r.width >= 24 && r.height >= 24) mark(media, r);
  }
  // Hero summary: the first-screen headline and how much of the first screen is media
  const h1 = document.querySelector("h1");
  const vhPx = window.innerHeight || 1;
  const firstRows = Math.max(1, Math.round((vhPx / H) * rows));
  let mediaCells = 0;
  for (let i = 0; i < firstRows * cols; i++) mediaCells += media[i] || 0;
  const hero = {
    headline: h1 ? (h1.textContent || "").replace(/\s+/g, " ").trim().slice(0, 90) : null,
    headline_px: h1 ? Math.round(parseFloat(getComputedStyle(h1).fontSize) || 0) : null,
    media_share: Number((mediaCells / (firstRows * cols)).toFixed(3)),
  };
  return { cols, rows, height_px: Math.round(H), text, media, hero, sections: layozaSectionPrints() };

  /*
   * Per-section prints: nav, hero, each top-level section and the footer, in page order. Each one
   * carries structure (an 8x6 text/media occupancy grid of the section's own box, its aspect, and
   * counts of headings, media, controls and columns), palette (rendered background, text and accent
   * colour in OKLab) and type (heading and body family and px).
   */
  function layozaSectionPrints() {
    const sy = window.scrollY;
    const visible = (el) => {
      const cs = getComputedStyle(el);
      if (cs.display === "none" || cs.visibility === "hidden") return null;
      const r = el.getBoundingClientRect();
      return r.width >= 160 && r.height >= 40 ? r : null;
    };
    const pick = new Set();
    const add = (el) => { if (el && el !== document.body && el !== document.documentElement && visible(el)) pick.add(el); };
    document.querySelectorAll("body > header, body > nav, [role='banner'], body > footer, [role='contentinfo'], body > section, main > *, body > main > section, body > div > section, body > div > header, body > div > footer, body > div > main > *").forEach(add);
    // Keep the outermost of nested picks, then order by page position
    let blocks = [...pick].filter((el) => ![...pick].some((o) => o !== el && o.contains(el)));
    if (blocks.length <= 1) blocks = [...(blocks[0] || document.body).children].filter((el) => visible(el));
    blocks = blocks
      .map((el) => ({ el, r: el.getBoundingClientRect() }))
      .sort((a, b) => a.r.top - b.r.top)
      .slice(0, 24);
    const toLin = (c) => { c /= 255; return c <= 0.04045 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4); };
    const oklab = (rgb) => {
      const [r, g, b] = rgb.map(toLin);
      const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b);
      const m = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b);
      const s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b);
      return [0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s, 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s, 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s].map((v) => Number(v.toFixed(4)));
    };
    const rgbOf = (str) => {
      const m = String(str || "").match(/rgba?\(([^)]+)\)/);
      if (!m) return null;
      const p = m[1].split(/[ ,/]+/).filter(Boolean).map(Number);
      if (p.length > 3 && p[3] < 0.5) return null;
      return p.slice(0, 3);
    };
    const bgOf = (el) => {
      for (let e = el; e; e = e.parentElement) {
        const c = rgbOf(getComputedStyle(e).backgroundColor);
        if (c) return c;
      }
      return [255, 255, 255];
    };
    const fam = (el) => (el ? getComputedStyle(el).fontFamily.split(",")[0].replace(/["']/g, "").trim().toLowerCase() : null);
    const px = (el) => (el ? Math.round(parseFloat(getComputedStyle(el).fontSize) || 0) : null);
    const h1 = document.querySelector("h1");
    let heroTaken = false;
    return blocks.map(({ el, r }, i) => {
      const tag = el.tagName.toLowerCase(), roleAttr = el.getAttribute("role") || "";
      let role = "section";
      if (tag === "footer" || roleAttr === "contentinfo") role = "footer";
      else if ((tag === "nav" || tag === "header" || roleAttr === "banner") && r.height < 220 && !(h1 && el.contains(h1))) role = "nav";
      else if (!heroTaken && ((h1 && el.contains(h1)) || r.top + sy < (window.innerHeight || 900))) { role = "hero"; heroTaken = true; }
      const gc = 8, gr = 6, top = r.top, W = Math.max(1, r.width), Hs = Math.max(1, r.height);
      const text = new Array(gc * gr).fill(0), media = new Array(gc * gr).fill(0);
      const mark = (grid, b) => {
        if (b.bottom <= top || b.top >= top + Hs || b.right <= r.left || b.left >= r.right) return;
        const x0 = Math.max(0, Math.floor(((b.left - r.left) / W) * gc)), x1 = Math.min(gc - 1, Math.floor(((b.right - r.left - 1) / W) * gc));
        const y0 = Math.max(0, Math.floor(((b.top - top) / Hs) * gr)), y1 = Math.min(gr - 1, Math.floor(((b.bottom - top - 1) / Hs) * gr));
        for (let y = y0; y <= y1; y++) for (let x = x0; x <= x1; x++) grid[y * gc + x] = 1;
      };
      const tw = document.createTreeWalker(el, NodeFilter.SHOW_TEXT);
      let n = 0, bodyEl = null;
      for (let t = tw.nextNode(); t && n < 600; t = tw.nextNode()) {
        if (!(t.nodeValue || "").trim()) continue;
        const pe = t.parentElement;
        if (!pe || pe.closest("script, style, noscript, template, [hidden], [aria-hidden='true']")) continue;
        if (!bodyEl && pe.closest("p, li, dd, td")) bodyEl = pe;
        const range = document.createRange();
        range.selectNodeContents(t);
        for (const b of range.getClientRects()) if (b.width > 2 && b.height > 2) { mark(text, b); n++; }
      }
      const mediaEls = [...el.querySelectorAll("img, picture, video, canvas, svg, iframe")].filter((m) => !(m.parentElement && m.parentElement.closest("svg")));
      for (const m of mediaEls) { const b = m.getBoundingClientRect(); if (b.width >= 24 && b.height >= 24) mark(media, b); }
      // Columns: the most children of one descendant sharing a top edge
      let columns = 1;
      for (const box of [el, ...el.querySelectorAll("*")].slice(0, 400)) {
        const kids = [...box.children].map((k) => k.getBoundingClientRect()).filter((b) => b.width >= 60 && b.height >= 20);
        if (kids.length < 2) continue;
        const tops = {};
        for (const b of kids) { const k = Math.round(b.top / 8); tops[k] = (tops[k] || 0) + 1; }
        columns = Math.max(columns, ...Object.values(tops));
      }
      const heading = el.querySelector("h1, h2, h3");
      const textEl = bodyEl || heading || el;
      const accentEl = el.querySelector("a.button, .btn, button, [class*='button'], [class*='btn']");
      const accent = accentEl ? rgbOf(getComputedStyle(accentEl).backgroundColor) || rgbOf(getComputedStyle(accentEl).color) : null;
      const bg = bgOf(el);
      return {
        index: i,
        role,
        tag,
        label: (heading ? (heading.textContent || "").replace(/\s+/g, " ").trim() : (el.id || tag)).slice(0, 60),
        structure: {
          text,
          media,
          aspect: Number((Hs / W).toFixed(3)),
          headings: el.querySelectorAll("h1, h2, h3, h4").length,
          media_count: mediaEls.length,
          controls: el.querySelectorAll("a[href], button, input, select, textarea").length,
          columns: Math.min(columns, 12),
        },
        palette: { bg: oklab(bg), text: oklab(rgbOf(getComputedStyle(textEl).color) || [0, 0, 0]), accent: accent ? oklab(accent) : null },
        type: { heading_family: fam(heading), heading_px: px(heading), body_family: fam(textEl), body_px: px(textEl) },
      };
    });
  }
}
const LAYOUT_VECTOR_SCRIPT = "(" + layozaLayoutVector.toString() + ")()";

/** Structure similarity of two section prints (0..1): grid cosine, shaded by aspect and column match. */
function sectionStructureSim(a, b) {
  const va = a.structure.text.concat(a.structure.media), vb = b.structure.text.concat(b.structure.media);
  let dot = 0, na = 0, nb = 0;
  for (let i = 0; i < Math.min(va.length, vb.length); i++) { dot += va[i] * vb[i]; na += va[i] * va[i]; nb += vb[i] * vb[i]; }
  const cos = na && nb ? dot / Math.sqrt(na * nb) : 0;
  const aspect = Math.min(a.structure.aspect, b.structure.aspect) / Math.max(1e-6, Math.max(a.structure.aspect, b.structure.aspect));
  const colsSame = a.structure.columns === b.structure.columns ? 1 : 0.7;
  const mediaSame = (a.structure.media_count > 0) === (b.structure.media_count > 0) ? 1 : 0.8;
  return cos * (0.75 + 0.25 * aspect) * colsSame * mediaSame;
}

/**
 * QA-SAME-SECTION-TWICE: content sections (not nav or footer) of one page whose structure repeats.
 * A section counts when it has at least 4 text cells; a pair matches at structure >= 0.93.
 */
function sameSectionTwice(sections) {
  const list = (sections || []).filter((s) => s.role === "section" && s.structure.text.reduce((x, y) => x + y, 0) >= 4);
  const out = [];
  for (let i = 0; i < list.length; i++) {
    for (let j = i + 1; j < list.length; j++) {
      const sim = sectionStructureSim(list[i], list[j]);
      if (sim >= 0.93) out.push({ a: list[i], b: list[j], sim: Number(sim.toFixed(3)) });
    }
  }
  return out;
}

/**
 * Look fingerprint, part 2 (in page, on a screenshot): dominant palette as OKLab k-means clusters
 * (k <= 6, pixel shares), seeded deterministically from a lightness-sorted sample so runs repeat.
 */
async function layozaPalette(dataUrl) {
  const img = new Image();
  await new Promise((res, rej) => { img.onload = res; img.onerror = () => rej(new Error("screenshot decode failed")); img.src = dataUrl; });
  const w = 120, h = Math.max(1, Math.round((img.height / img.width) * 120));
  const c = document.createElement("canvas");
  c.width = w; c.height = h;
  const g = c.getContext("2d", { willReadFrequently: true });
  g.drawImage(img, 0, 0, w, h);
  const d = g.getImageData(0, 0, w, h).data;
  const lin = (v) => { v /= 255; return v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4); };
  const toLab = (r, gg, b) => {
    const R = lin(r), G = lin(gg), B = lin(b);
    const l = Math.cbrt(0.4122214708 * R + 0.5363325363 * G + 0.0514459929 * B);
    const m = Math.cbrt(0.2119034982 * R + 0.6806995451 * G + 0.1073969566 * B);
    const s = Math.cbrt(0.0883024619 * R + 0.2817188376 * G + 0.6299787005 * B);
    return [0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s, 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s, 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s];
  };
  const pts = [];
  for (let i = 0; i < d.length; i += 4) pts.push(toLab(d[i], d[i + 1], d[i + 2]).concat([d[i], d[i + 1], d[i + 2]]));
  const K = Math.min(6, pts.length);
  const sorted = pts.slice().sort((a, b) => a[0] - b[0]);
  let cent = Array.from({ length: K }, (_, i) => sorted[Math.floor(((i + 0.5) / K) * sorted.length)].slice(0, 3));
  const dist = (a, b) => (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 + (a[2] - b[2]) ** 2;
  let assign = new Array(pts.length).fill(0);
  for (let it = 0; it < 12; it++) {
    for (let i = 0; i < pts.length; i++) {
      let best = 0, bd = Infinity;
      for (let k = 0; k < K; k++) { const dd = dist(pts[i], cent[k]); if (dd < bd) { bd = dd; best = k; } }
      assign[i] = best;
    }
    const sum = Array.from({ length: K }, () => [0, 0, 0, 0]);
    for (let i = 0; i < pts.length; i++) { const s = sum[assign[i]]; s[0] += pts[i][0]; s[1] += pts[i][1]; s[2] += pts[i][2]; s[3]++; }
    cent = cent.map((c0, k) => (sum[k][3] ? [sum[k][0] / sum[k][3], sum[k][1] / sum[k][3], sum[k][2] / sum[k][3]] : c0));
  }
  const rgbSum = Array.from({ length: K }, () => [0, 0, 0, 0]);
  for (let i = 0; i < pts.length; i++) { const s = rgbSum[assign[i]]; s[0] += pts[i][3]; s[1] += pts[i][4]; s[2] += pts[i][5]; s[3]++; }
  const hex = (v) => Math.round(v).toString(16).padStart(2, "0");
  return cent
    .map((c0, k) => ({
      hex: rgbSum[k][3] ? "#" + hex(rgbSum[k][0] / rgbSum[k][3]) + hex(rgbSum[k][1] / rgbSum[k][3]) + hex(rgbSum[k][2] / rgbSum[k][3]) : null,
      oklab: c0.map((v) => Number(v.toFixed(4))),
      share: Number((rgbSum[k][3] / pts.length).toFixed(4)),
    }))
    .filter((x) => x.share > 0)
    .sort((a, b) => b.share - a.share);
}
const PALETTE_SCRIPT = layozaPalette.toString();

/**
 * Re-skin verdict between two look fingerprints. layout_similarity: cosine of the text+media grids;
 * palette_distance: share-weighted nearest-cluster OKLab distance, averaged both ways. Same layout
 * (>= 0.85) with a different palette (>= 0.08) is a re-skin; same layout and palette is a near
 * duplicate; anything else is distinct.
 */
function compareFingerprints(a, b) {
  if (!a || !b || !a.layout || !b.layout || !a.palette || !b.palette) return { verdict: "unmeasured", reason: "a report has no look_fingerprint" };
  const va = a.layout.text.concat(a.layout.media), vb = b.layout.text.concat(b.layout.media);
  let dot = 0, na = 0, nb = 0;
  for (let i = 0; i < Math.min(va.length, vb.length); i++) { dot += va[i] * vb[i]; na += va[i] * va[i]; nb += vb[i] * vb[i]; }
  const layout = na && nb ? dot / Math.sqrt(na * nb) : 0;
  const near = (p, q) => p.reduce((s, c) => s + c.share * Math.min(...q.map((o) => Math.hypot(c.oklab[0] - o.oklab[0], c.oklab[1] - o.oklab[1], c.oklab[2] - o.oklab[2]))), 0) / Math.max(1e-9, p.reduce((s, c) => s + c.share, 0));
  const palette = (near(a.palette, b.palette) + near(b.palette, a.palette)) / 2;
  const verdict = layout >= 0.85 ? (palette >= 0.08 ? "re-skin" : "near-duplicate") : "distinct";
  return { verdict, layout_similarity: Number(layout.toFixed(3)), palette_distance: Number(palette.toFixed(4)), thresholds: { layout_same: 0.85, palette_different: 0.08 } };
}

// Lab throttling profile for the slow-device pass (CDP; network emulation does not slow file:// loads)
const THROTTLE = { cpu: 4, latency_ms: 150, down_bytes_s: 200000, up_bytes_s: 93750 };

/** Throttled lab load on the mobile viewport (Node runner). */
async function throttledPerfNode(browser, url) {
  const vp = VIEWPORTS.find((v) => v.label === "mobile");
  const ctx = await browser.newContext({ viewport: { width: vp.width, height: vp.height } });
  try {
    await ctx.addInitScript(PERF_INIT_SCRIPT);
    const page = await ctx.newPage();
    const cdp = await ctx.newCDPSession(page);
    await cdp.send("Network.enable");
    await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: THROTTLE.latency_ms, downloadThroughput: THROTTLE.down_bytes_s, uploadThroughput: THROTTLE.up_bytes_s });
    await cdp.send("Emulation.setCPUThrottlingRate", { rate: THROTTLE.cpu });
    const t0 = Date.now();
    await page.goto(url, { waitUntil: "load", timeout: 60000 });
    const loadMs = Date.now() - t0;
    await page.waitForLoadState("networkidle", { timeout: 4000 }).catch(() => {});
    await fontsReadyNode(page, 3000);
    await settleNode(page, 3000);
    for (let i = 0; i < 3; i++) await page.keyboard.press("Tab");
    await page.waitForTimeout(WAIT.key);
    return { ...(await ev(page, PERF_READ_SCRIPT)), load_event_ms: loadMs };
  } catch (e) {
    return { error: String(e.message || e).split("\n")[0] };
  } finally {
    await ctx.close();
  }
}

/** Reduced-motion hidden-text run on the desktop viewport (Node runner). */
async function reducedMotionNode(browser, url, caps) {
  const vp = VIEWPORTS[0];
  const ctx = await browser.newContext({ viewport: { width: vp.width, height: vp.height }, reducedMotion: "reduce" });
  try {
    const page = await ctx.newPage();
    await page.goto(url, { waitUntil: "load", timeout: 30000 });
    await page.waitForLoadState("networkidle", { timeout: caps.netidle_ms }).catch(() => {});
    await fontsReadyNode(page, 2000);
    await page.waitForTimeout(WAIT.reduced);
    const hidden = await ev(page, HIDDEN_TEXT_PROBE);
    const running = await ev(page, RUNNING_MOTION_PROBE).catch(() => []);
    const widths = [];
    for (const w of EXTRA_WIDTHS) {
      await page.setViewportSize({ width: w, height: 900 });
      await ev(page, FRAMES_SCRIPT);
      widths.push({ width: w, overflow_px: await ev(page, OVERFLOW_PROBE) });
    }
    return { hidden, running, widths };
  } catch (e) {
    return { hidden: [], error: String(e.message || e).split("\n")[0] };
  } finally {
    await ctx.close();
  }
}

/** Palette + layout fingerprint of the current page (Node runner, desktop). */
async function fingerprintNode(page, vp) {
  try {
    const layout = await ev(page, LAYOUT_VECTOR_SCRIPT);
    const shot = await page.screenshot({ fullPage: true, clip: { x: 0, y: 0, width: vp.width, height: Math.max(1, layout.height_px) } });
    const palette = await ev(page, "(" + PALETTE_SCRIPT + ")(" + JSON.stringify("data:image/png;base64," + shot.toString("base64")) + ")");
    return { palette, layout };
  } catch (e) {
    return { error: String(e.message || e).split("\n")[0] };
  }
}

/*
 * ---------------------------------------------------------------------------------------------
 * Design-system conformance (design-system-color / -font / -radius / -font-size)
 *
 * Layoza-original. pbakaus/impeccable (Apache-2.0) has rules with these ids, which inspired the
 * idea; no upstream code is used here. Upstream scans authored CSS literals against DESIGN.md with a
 * per-channel RGB tolerance. This implementation instead:
 *  - checks the project's OWN declared system (DESIGN.md / SITE-DNA.json, passed in by the runner as
 *    `--design-tokens <json>`), so there are no global taste bans: whatever the brand declares is
 *    on-system by definition;
 *  - measures rendered computed styles in real Chromium at all three viewports (so fluid clamp()
 *    type, media-query overrides and inherited UA defaults are caught where they actually render);
 *  - compares colours perceptually with CIEDE2000 (ΔE00) in CIELAB (D65). A rendered colour is
 *    on-system when ΔE00 <= COLOR_DE_MAX (3.0: ~1 JND above the 2.3 "just noticeable" mark, so a
 *    hand-typed #1b1b1b for a #1a1a1a token passes while a different hue or shade does not);
 *  - names the nearest token (name, value, distance) in every finding so the fix is one swap.
 * Tolerances: radius 1px, font-size 0.75px (sub-pixel rounding of rem/em maths); a fluid
 * clamp(min, …, max) font-size token accepts any rendered size within [min, max].
 * Emitted as severity "warning" (objective: the page breaks its own declared system). One issue per
 * distinct off-system value per viewport (with element count and an example element), capped at
 * DS_MAX_PER_RULE per rule. Opt-out: <meta name="layoza-brand-allow" content="rule:design-system-color">.
 * Not checked (by design): border / outline / shadow colours, SVG and media internals, form-control
 * UA chrome, colours with alpha < 0.1, and generic font fallbacks on code/kbd/samp/pre.
 *
 * Token JSON: { "source": "...", "colors": [hex | {name, value}], "fonts": [family | {name, value}],
 *   "radii": [px | {name, value}], "font_sizes": [px | {name, value} | {name, min, max}] }
 * ---------------------------------------------------------------------------------------------
 */
function layozaDesignSystemCheck(tokens) {
  "use strict";
  const D = document;
  const issues = [];
  const errors = [];
  if (!D.body || !tokens) return { issues, errors };
  const COLOR_DE_MAX = 3.0;
  const RADIUS_TOL_PX = 1;
  const FONT_SIZE_TOL_PX = 0.75;
  const DS_MAX_PER_RULE = 12;
  const GENERIC = ["serif", "sans-serif", "monospace", "cursive", "fantasy", "system-ui", "ui-serif", "ui-sans-serif", "ui-monospace", "ui-rounded", "emoji", "math", "fangsong", "-apple-system", "blinkmacsystemfont", "inherit", "initial", "unset", "revert"];
  const SKIP_CONTEXT = "svg, canvas, video, iframe, picture, object, embed, math, noscript, template, input, select, textarea, option, meter, progress";
  const CODE_CONTEXT = "code, kbd, samp, pre, var";

  const allowRules = [];
  D.querySelectorAll('meta[name="layoza-brand-allow" i]').forEach((m) => {
    (m.getAttribute("content") || "").split(/[,;]/).forEach((part) => {
      const i = part.indexOf(":");
      if (i >= 0 && part.slice(0, i).trim().toLowerCase() === "rule") allowRules.push(part.slice(i + 1).trim().toLowerCase());
    });
  });
  const named = (list, label) =>
    (Array.isArray(list) ? list : []).map((t, i) => (t && typeof t === "object" ? { name: t.name || label + "-" + (i + 1), value: t.value, min: t.min, max: t.max } : { name: label + "-" + (i + 1), value: t }));

  // ---- Colour: parse computed rgb()/rgba(), sRGB -> CIELAB (D65) -> CIEDE2000 -------------------
  function hexToRgb(hex) {
    let h = String(hex || "").trim().replace(/^#/, "");
    if (h.length === 3 || h.length === 4) h = h.split("").map((c) => c + c).join("");
    if (!/^[0-9a-f]{6}([0-9a-f]{2})?$/i.test(h)) return null;
    return { r: parseInt(h.slice(0, 2), 16), g: parseInt(h.slice(2, 4), 16), b: parseInt(h.slice(4, 6), 16), a: h.length === 8 ? parseInt(h.slice(6, 8), 16) / 255 : 1 };
  }
  function parseCss(str) {
    const m = String(str || "").match(/^rgba?\(\s*([\d.]+)[\s,]+([\d.]+)[\s,]+([\d.]+)\s*(?:[,/]\s*([\d.]+)(%?))?\s*\)$/i);
    if (m) {
      let a = m[4] === undefined ? 1 : parseFloat(m[4]);
      if (m[5] === "%") a /= 100;
      return { r: +m[1], g: +m[2], b: +m[3], a };
    }
    const hx = hexToRgb(str);
    if (hx) return hx;
    // Any other syntax (oklch, hsl in a token…): let the browser resolve it
    try {
      const probe = D.createElement("span");
      probe.style.color = String(str);
      if (!probe.style.color) return null;
      D.body.appendChild(probe);
      const v = getComputedStyle(probe).color;
      probe.remove();
      const m2 = v.match(/^rgba?\(\s*([\d.]+)[\s,]+([\d.]+)[\s,]+([\d.]+)/i);
      return m2 ? { r: +m2[1], g: +m2[2], b: +m2[3], a: 1 } : null;
    } catch (e) {
      return null;
    }
  }
  const hexOf = (c) => "#" + [c.r, c.g, c.b].map((v) => Math.round(v).toString(16).padStart(2, "0")).join("");
  function toLab(c) {
    const lin = (v) => {
      v /= 255;
      return v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
    };
    const R = lin(c.r), G = lin(c.g), B = lin(c.b);
    const X = (0.4124564 * R + 0.3575761 * G + 0.1804375 * B) / 0.95047;
    const Y = 0.2126729 * R + 0.7151522 * G + 0.072175 * B;
    const Z = (0.0193339 * R + 0.119192 * G + 0.9503041 * B) / 1.08883;
    const f = (t) => (t > 216 / 24389 ? Math.cbrt(t) : (24389 / 27 * t + 16) / 116);
    const fx = f(X), fy = f(Y), fz = f(Z);
    return { L: 116 * fy - 16, a: 500 * (fx - fy), b: 200 * (fy - fz) };
  }
  function deltaE00(l1, l2) {
    const rad = Math.PI / 180;
    const C1 = Math.hypot(l1.a, l1.b), C2 = Math.hypot(l2.a, l2.b);
    const Cb = (C1 + C2) / 2;
    const G = 0.5 * (1 - Math.sqrt(Math.pow(Cb, 7) / (Math.pow(Cb, 7) + Math.pow(25, 7))));
    const a1 = (1 + G) * l1.a, a2 = (1 + G) * l2.a;
    const C1p = Math.hypot(a1, l1.b), C2p = Math.hypot(a2, l2.b);
    const hp = (b, a) => (b === 0 && a === 0 ? 0 : (Math.atan2(b, a) / rad + 360) % 360);
    const h1 = hp(l1.b, a1), h2 = hp(l2.b, a2);
    const dL = l2.L - l1.L, dC = C2p - C1p;
    let dh = 0;
    if (C1p * C2p !== 0) {
      dh = h2 - h1;
      if (dh > 180) dh -= 360;
      else if (dh < -180) dh += 360;
    }
    const dH = 2 * Math.sqrt(C1p * C2p) * Math.sin((dh / 2) * rad);
    const Lb = (l1.L + l2.L) / 2, Cbp = (C1p + C2p) / 2;
    let hb = h1 + h2;
    if (C1p * C2p !== 0) {
      if (Math.abs(h1 - h2) <= 180) hb = (h1 + h2) / 2;
      else hb = h1 + h2 < 360 ? (h1 + h2 + 360) / 2 : (h1 + h2 - 360) / 2;
    }
    const T = 1 - 0.17 * Math.cos((hb - 30) * rad) + 0.24 * Math.cos(2 * hb * rad) + 0.32 * Math.cos((3 * hb + 6) * rad) - 0.2 * Math.cos((4 * hb - 63) * rad);
    const dTheta = 30 * Math.exp(-Math.pow((hb - 275) / 25, 2));
    const Rc = 2 * Math.sqrt(Math.pow(Cbp, 7) / (Math.pow(Cbp, 7) + Math.pow(25, 7)));
    const Sl = 1 + (0.015 * Math.pow(Lb - 50, 2)) / Math.sqrt(20 + Math.pow(Lb - 50, 2));
    const Sc = 1 + 0.045 * Cbp;
    const Sh = 1 + 0.015 * Cbp * T;
    const Rt = -Math.sin(2 * dTheta * rad) * Rc;
    return Math.sqrt(Math.pow(dL / Sl, 2) + Math.pow(dC / Sc, 2) + Math.pow(dH / Sh, 2) + Rt * (dC / Sc) * (dH / Sh));
  }

  const colorTokens = named(tokens.colors, "color")
    .map((t) => {
      const c = parseCss(t.value);
      return c ? { name: t.name, value: hexOf(c), lab: toLab(c) } : null;
    })
    .filter(Boolean);
  const normFont = (f) => String(f || "").trim().replace(/^['"]|['"]$/g, "").trim().toLowerCase();
  const fontTokens = [];
  for (const t of named(tokens.fonts, "font")) {
    for (const fam of String(t.value || "").split(",").map(normFont).filter(Boolean)) {
      if (!fontTokens.some((x) => x.family === fam)) fontTokens.push({ name: t.name, family: fam, generic: GENERIC.includes(fam) });
    }
  }
  const pxOf = (v) => {
    if (typeof v === "number") return v;
    const m = String(v || "").trim().toLowerCase().match(/^(-?[\d.]+)(px|rem|em)?$/);
    if (!m) return null;
    const n = parseFloat(m[1]);
    return m[2] === "rem" || m[2] === "em" ? n * 16 : n;
  };
  const radiusTokens = named(tokens.radii, "radius").map((t) => ({ name: t.name, px: pxOf(t.value) })).filter((t) => t.px != null && Number.isFinite(t.px));
  const sizeTokens = named(tokens.font_sizes, "size")
    .map((t) => {
      if (t.min != null && t.max != null) return { name: t.name, min: pxOf(t.min), max: pxOf(t.max) };
      const px = pxOf(t.value);
      return px == null ? null : { name: t.name, min: px, max: px };
    })
    .filter((t) => t && Number.isFinite(t.min) && Number.isFinite(t.max));

  // ---- Element collection --------------------------------------------------------------------
  const csOf = (el) => getComputedStyle(el);
  function visible(el, s) {
    if (s.display === "none" || s.visibility === "hidden" || s.visibility === "collapse") return false;
    for (let c = el; c && c.nodeType === 1; c = c.parentElement) {
      const cs = getComputedStyle(c);
      if (cs.display === "none" || parseFloat(cs.opacity) <= 0.01) return false;
    }
    const r = el.getBoundingClientRect();
    return r.width > 0 && r.height > 0;
  }
  const ownText = (el) => Array.from(el.childNodes).some((n) => n.nodeType === 3 && (n.nodeValue || "").trim());
  const describe = (el) => {
    const cls = (el.getAttribute("class") || "").trim().split(/\s+/).filter(Boolean).slice(0, 2);
    const sel = el.localName + (el.id ? "#" + el.id : "") + (cls.length ? "." + cls.join(".") : "");
    const text = (el.textContent || "").replace(/\s+/g, " ").trim().slice(0, 30);
    return "<" + sel + ">" + (text ? ' "' + text + '"' : "");
  };
  const groups = {}; // rule -> Map(valueKey -> {count, example, nearest, detail})
  const record = (rule, key, el, build) => {
    if (allowRules.includes(rule)) return;
    const g = (groups[rule] = groups[rule] || new Map());
    const cur = g.get(key);
    if (cur) cur.count++;
    else g.set(key, Object.assign({ count: 1, example: describe(el) }, build()));
  };

  const all = Array.from(D.body.querySelectorAll("*")).slice(0, 8000);
  all.unshift(D.body);
  for (const el of all) {
    try {
      if (el.closest(SKIP_CONTEXT)) continue;
      const s = csOf(el);
      if (!visible(el, s)) continue;
      const text = ownText(el);
      // Colours: text colour where the element sets text; any painted background
      if (colorTokens.length) {
        const checks = [];
        if (text) checks.push(["text", s.color]);
        checks.push(["background", s.backgroundColor]);
        for (const [role, raw] of checks) {
          const c = parseCss(raw);
          if (!c || c.a < 0.1) continue;
          const lab = toLab(c);
          let best = null;
          for (const t of colorTokens) {
            const de = deltaE00(lab, t.lab);
            if (!best || de < best.de) best = { t, de };
          }
          if (best.de <= COLOR_DE_MAX) continue;
          const hex = hexOf(c);
          record("design-system-color", role + ":" + hex, el, () => ({ role, value: hex, nearest_token: best.t.name, nearest_value: best.t.value, distance: Number(best.de.toFixed(1)), unit: "ΔE00" }));
        }
      }
      if (text && fontTokens.length) {
        const stack = s.fontFamily.split(",").map(normFont).filter(Boolean);
        const primary = stack[0] || "";
        const codeGeneric = GENERIC.includes(primary) && el.closest(CODE_CONTEXT);
        if (primary && !codeGeneric && !fontTokens.some((t) => t.family === primary)) {
          // Nearest: a declared face already in the rendered fallback stack, else the token whose role
          // matches the element (display/heading roles for h1-h6, body/text roles otherwise), else the
          // first declared face
          const isHeading = /^h[1-6]$/.test(el.localName) || el.getAttribute("role") === "heading";
          const roleRe = isHeading ? /display|head|title|h[1-6]/i : /body|text|paragraph|base|copy/i;
          const faces = fontTokens.filter((t) => !t.generic);
          const near = faces.find((t) => stack.includes(t.family)) || faces.find((t) => roleRe.test(t.name)) || faces[0] || fontTokens[0];
          record("design-system-font", primary, el, () => ({ value: primary, nearest_token: near.name, nearest_value: near.family }));
        }
      }
      if (text && sizeTokens.length) {
        const px = parseFloat(s.fontSize);
        if (Number.isFinite(px)) {
          let best = null;
          for (const t of sizeTokens) {
            const d = px < t.min ? t.min - px : px > t.max ? px - t.max : 0;
            if (!best || d < best.d) best = { t, d };
          }
          if (best.d > FONT_SIZE_TOL_PX) {
            const v = Number(px.toFixed(2)) + "px";
            const tv = best.t.min === best.t.max ? best.t.min + "px" : "clamp(" + best.t.min + "px…" + best.t.max + "px)";
            record("design-system-font-size", v, el, () => ({ value: v, nearest_token: best.t.name, nearest_value: tv, distance: Number(best.d.toFixed(2)), unit: "px" }));
          }
        }
      }
      if (radiusTokens.length) {
        const r = el.getBoundingClientRect();
        const half = Math.min(r.width, r.height) / 2;
        const corners = [s.borderTopLeftRadius, s.borderTopRightRadius, s.borderBottomRightRadius, s.borderBottomLeftRadius];
        const seen = new Set();
        for (const c of corners) {
          const first = String(c || "0").trim().split(/\s+/)[0];
          if (/%$/.test(first)) continue; // percentage radii scale with the box; not a scale step
          const px = parseFloat(first);
          if (!Number.isFinite(px) || px <= 0 || seen.has(px)) continue;
          seen.add(px);
          // A radius at or past half the short side renders as a pill / circle: any token that also
          // reaches that far (e.g. "full: 9999px") renders identically.
          const pill = px >= half - 0.5;
          let best = null;
          for (const t of radiusTokens) {
            const d = pill && t.px >= half - 0.5 ? 0 : Math.abs(t.px - px);
            if (!best || d < best.d) best = { t, d };
          }
          if (best.d > RADIUS_TOL_PX) {
            const v = Number(px.toFixed(2)) + "px";
            record("design-system-radius", v, el, () => ({ value: v, nearest_token: best.t.name, nearest_value: best.t.px + "px", distance: Number(best.d.toFixed(2)), unit: "px" }));
          }
        }
      }
    } catch (e) {
      if (errors.length < 20) errors.push(String(e && e.message ? e.message : e));
    }
  }

  for (const rule of ["design-system-color", "design-system-font", "design-system-radius", "design-system-font-size"]) {
    const g = groups[rule];
    if (!g) continue;
    const rows = Array.from(g.values()).sort((a, b) => b.count - a.count).slice(0, DS_MAX_PER_RULE);
    for (const row of rows) {
      const what = rule === "design-system-color" ? row.role + " colour " + row.value : rule === "design-system-font" ? "font-family " + row.value : rule === "design-system-radius" ? "border-radius " + row.value : "font-size " + row.value;
      const dist = row.distance != null ? " (" + row.distance + " " + row.unit + " away)" : "";
      issues.push({
        rule,
        severity: "warning",
        detail: what + " on " + row.count + " element" + (row.count === 1 ? "" : "s") + ", e.g. " + row.example + "; nearest token " + row.nearest_token + " = " + row.nearest_value + dist,
        value: row.value,
        element: row.example,
        count: row.count,
        nearest_token: row.nearest_token,
        nearest_value: row.nearest_value,
        distance: row.distance != null ? row.distance : null,
      });
    }
  }
  return { issues, errors };
}

function parseArgs(argv) {
  const args = {
    url: null,
    file: null,
    out: ".layoza/qa-snapshot.json",
    screenshotDir: null,
    designTokens: null,
    allow: [],
    compare: null,
    full: false,
  };
  let outGiven = false;
  for (let i = 2; i < argv.length; i++) {
    const k = argv[i];
    if (k === "--url" && argv[i + 1]) args.url = argv[++i];
    else if (k === "--file" && argv[i + 1]) args.file = argv[++i];
    else if (k === "--out" && argv[i + 1]) { args.out = argv[++i]; outGiven = true; }
    else if (k === "--screenshots" && argv[i + 1]) args.screenshotDir = argv[++i];
    else if (k === "--design-tokens" && argv[i + 1]) args.designTokens = argv[++i];
    else if (k === "--allow" && argv[i + 1]) args.allow.push(...argv[++i].split(",").map((s) => s.trim()).filter(Boolean));
    else if (k === "--compare" && argv[i + 1] && argv[i + 2]) args.compare = [argv[++i], argv[++i]];
    else if (k === "--full") args.full = true;
    else if (k.startsWith("--")) console.error(`Unknown flag ignored: ${k}`);
  }
  args.outGiven = outGiven;
  // Keep screenshots beside the report so QA of one page never overwrites the repo's own captures.
  if (!args.screenshotDir) {
    args.screenshotDir = outGiven ? path.join(path.dirname(args.out), "screenshots") : ".layoza/screenshots";
  }
  if (!args.url && args.file) {
    if (!fs.existsSync(args.file)) {
      console.error(`File not found: ${args.file}`);
      process.exit(1);
    }
    args.url = pathToFileURL(path.resolve(args.file)).href;
  }
  return args;
}

const ART_SCROLL_SCRIPT = `(y) => { window.scrollTo({ top: y, left: 0, behavior: "instant" }); return document.documentElement.scrollHeight; }`;
const artStep = (vp) => (vp.width < 768 ? 700 : 760);

/**
 * art-over-text pass: at the first screen and each scroll step, diffs the viewport with canvas/video
 * art shown vs hidden inside the text line boxes that the art overlaps. Returns hits + errors.
 */
/** Folds one position's text-over-graphic statistics into the running result (shared by both runners). */
function addTextGraphicHits(tg, found, stats, at) {
  found.candidates.forEach((c, i) => {
    const st = stats[i];
    if (!st) return;
    tg.sampled++;
    const required = c.large ? 3 : 4.5;
    const occluded = st.changed <= TG_OCCLUDED_MAX;
    const lowContrast = !occluded && st.p10 < required;
    const busy = !occluded && st.lstd >= TG_BUSY_LSTD;
    if (!occluded && !lowContrast && !busy) return;
    if (tg.hits.some((h) => h.text === c.text)) return;
    tg.hits.push({ text: c.text, el: c.el, graphic: c.graphic, y: at, required, contrast_p10: st.p10, contrast_median: st.median, l_std: st.lstd, changed_when_hidden: st.changed, occluded, low_contrast: lowContrast, busy, halo: Boolean(c.halo) });
  });
}

/** text-over-graphic at the current scroll position (Node runner). */
async function textOverGraphicNode(page, tg) {
  if (tg.positions >= TG_MAX_POSITIONS || tg.sampled >= TG_MAX_TOTAL) return;
  const found = await ev(page, TG_FIND_SCRIPT + "(" + Math.min(TG_MAX_PER_POSITION, TG_MAX_TOTAL - tg.sampled) + ")");
  if (!found.candidates.length) return;
  tg.positions++;
  let hidden;
  const shown = await page.screenshot({ clip: found.clip });
  await ev(page, TG_HIDE_SCRIPT);
  try {
    await page.waitForTimeout(60);
    hidden = await page.screenshot({ clip: found.clip });
  } finally {
    await ev(page, TG_RESTORE_SCRIPT);
  }
  const stats = await ev(page, TG_STATS_SCRIPT, {
    shown: "data:image/png;base64," + shown.toString("base64"),
    hidden: "data:image/png;base64," + hidden.toString("base64"),
    clip: found.clip,
    candidates: found.candidates,
  });
  addTextGraphicHits(tg, found, stats, Math.round(await ev(page, "window.scrollY")));
}

/**
 * Waits (up to capMs) until finite animations are done and the first screen is pixel-stable across
 * two frames SETTLE_STEP_MS apart, with canvas/video/endless loops masked. Returns how it ended.
 */
/** fonts.ready, capped so a font that never resolves cannot hang the run. */
async function fontsReadyNode(page, capMs) {
  await ev(page, FONTS_READY_SCRIPT, capMs).catch(() => {});
}

async function settleNode(page, capMs) {
  const t0 = Date.now();
  let prev = null;
  let last = { pending: 0, loops: 0 };
  for (;;) {
    last = await ev(page, SETTLE_PENDING_SCRIPT + "(" + SETTLE_CAP_MS + ")");
    let shot = null;
    if (last.pending === 0) {
      shot = "data:image/png;base64," + (await page.screenshot({ mask: [page.locator(SETTLE_MASK_SELECTOR)] })).toString("base64");
      if (prev && (await ev(page, PIN_DIFF_SCRIPT, [prev, shot])) <= SETTLE_STABLE_MAX) {
        return { settled: true, ms: Date.now() - t0, pending_animations: 0, endless_loops: last.loops };
      }
    }
    prev = shot;
    if (Date.now() - t0 >= capMs) return { settled: false, ms: Date.now() - t0, pending_animations: last.pending, endless_loops: last.loops };
    await page.waitForTimeout(SETTLE_STEP_MS);
  }
}

async function artOverTextNode(page, vp) {
  const hits = [];
  const errors = [];
  const tg = { positions: 0, sampled: 0, hits: [], errors: [] };
  let positions = 0;
  let diffs = 0;
  const t0 = Date.now();
  try {
    let scrollH = await ev(page, ART_SCROLL_SCRIPT, 0);
    const vh = vp.height;
    for (let y = 0; positions < ART_MAX_SCANS && diffs < ART_MAX_DIFFS && y < Math.max(1, scrollH - vh + artStep(vp)); y += artStep(vp)) {
      scrollH = await ev(page, ART_SCROLL_SCRIPT, Math.min(y, Math.max(0, scrollH - vh)));
      positions++;
      await page.waitForTimeout(WAIT.art_step);
      try {
        await textOverGraphicNode(page, tg);
      } catch (e) {
        if (tg.errors.length < 3) tg.errors.push(String(e.message || e).split(/\r?\n/)[0]);
        await ev(page, TG_RESTORE_SCRIPT).catch(() => {});
      }
      const found = await ev(page, ART_LINES_SCRIPT);
      if (!found.lines.length) continue;
      diffs++;
      // The first screen is where visitors rest longest: sample moving art three times there
      const shown = [await page.screenshot({ clip: found.clip })];
      for (let k = 1; k < (positions === 1 ? 3 : 1); k++) {
        await page.waitForTimeout(WAIT.art_first);
        shown.push(await page.screenshot({ clip: found.clip }));
      }
      await ev(page, ART_HIDE_SCRIPT);
      let hidden;
      try {
        await page.waitForTimeout(60);
        hidden = await page.screenshot({ clip: found.clip });
      } finally {
        await ev(page, ART_RESTORE_SCRIPT);
      }
      const cover = await ev(page, ART_DIFF_SCRIPT, {
        shown: shown.map((b) => "data:image/png;base64," + b.toString("base64")),
        b: "data:image/png;base64," + hidden.toString("base64"),
        lines: found.lines,
        clip: found.clip,
      });
      const at = Math.round(await ev(page, "window.scrollY"));
      found.lines.forEach((l, i) => {
        const c = cover[i];
        if (c.line < ART_COVER_MIN && c.word < ART_WORD_MIN) return;
        const prev = hits.find((h) => h.text === l.text);
        if (!prev) hits.push({ text: l.text, y: at, cover: c.line, word: c.word, word_text: c.wordText });
        else {
          prev.cover = Math.max(prev.cover, c.line);
          if (c.word > prev.word) Object.assign(prev, { word: c.word, word_text: c.wordText });
        }
      });
    }
    await ev(page, ART_SCROLL_SCRIPT, 0);
    await ev(page, FRAMES_SCRIPT);
  } catch (e) {
    errors.push(String(e.message || e).split("\n")[0]);
  }
  return { hits, positions, diffs, ms: Date.now() - t0, errors, tg };
}

// motion-too-sparse + pinned-stage-static (Layoza-original). Desktop only, on a fresh load after the
// other checks (so once-only reveals have not fired yet and the scroll pass is not disturbed).
// Motion density: distinct groups of elements whose transform/opacity/clip/filter/colour change
// across scroll positions 25/50/75/100/0%, plus running or declared CSS/WAAPI animations and
// scroll-timeline links; nested movers count once (top-most) and siblings count as one group (a
// staggered list is one idea). Large canvases add at most 2, interactive transitions at most 2.
// The page declares its level with <meta name="layoza-motion" content="calm|bold|wild|none">.
const MOTION_DENSITY_MIN = { calm: 3, bold: 6, signature: 2 };
const MOTION_PROBE_SCRIPT = `(async () => {
  const meta = document.querySelector('meta[name="layoza-motion"]');
  const declared = ((meta && meta.getAttribute("content")) || document.documentElement.getAttribute("data-motion") || "").trim().toLowerCase();
  const allow = Array.from(document.querySelectorAll('meta[name="layoza-brand-allow"]')).map((m) => m.getAttribute("content") || "").join(",");
  const out = { declared: declared || null, skipped: null, groups: 0, changed: 0, animated: 0, scroll_linked: 0, canvases: 0, interactive: 0, short_page: false, app_surface: false, examples: [] };
  out.app_surface = !!document.querySelector("table, [role=grid], [role=treegrid], [role=application]");
  if (declared === "none" || declared === "static") { out.skipped = "page declares no motion"; return out; }
  if (allow.includes("motion-too-sparse")) { out.skipped = "brand-allow"; return out; }
  const SKIP = new Set(["SCRIPT", "STYLE", "LINK", "META", "NOSCRIPT", "TEMPLATE", "BR", "TITLE", "SOURCE", "TRACK"]);
  const els = Array.from(document.body.querySelectorAll("*")).filter((e) => !SKIP.has(e.tagName)).slice(0, 5000);
  const sig = (e) => { const s = getComputedStyle(e); return [s.transform, s.opacity, s.clipPath, s.filter, s.translate, s.scale, s.rotate, s.strokeDashoffset, s.backgroundPosition, s.color, s.backgroundColor, s.visibility, s.width].join("|"); };
  const go = (y) => window.scrollTo({ top: y, left: 0, behavior: "instant" });
  const wait = (ms) => new Promise((r) => setTimeout(r, ms));
  const maxY = Math.max(0, document.documentElement.scrollHeight - window.innerHeight);
  out.short_page = maxY < window.innerHeight * 0.5;
  const base = new Map(els.map((e) => [e, sig(e)]));
  const moved = new Set();
  for (const f of [0.25, 0.5, 0.75, 1, 0]) {
    go(Math.round(maxY * f));
    await wait(300);
    for (const e of els) if (!moved.has(e) && sig(e) !== base.get(e)) moved.add(e);
  }
  out.changed = moved.size;
  const sized = (e) => { const r = e.getBoundingClientRect(); return r.width > 1 && r.height > 1; };
  const animTargets = new Set();
  try { for (const a of document.getAnimations ? document.getAnimations() : []) { const t = a.effect && a.effect.target; if (t && t.nodeType === 1) animTargets.add(t); } } catch (e) {}
  let interactiveTransitions = 0;
  const transitionKinds = new Set();
  for (const e of els) {
    const s = getComputedStyle(e);
    if (s.animationName && s.animationName !== "none") animTargets.add(e);
    const tl = s.animationTimeline;
    if (tl && tl !== "auto" && tl !== "none" && s.animationName !== "none") { animTargets.add(e); out.scroll_linked++; }
    const dur = (s.transitionDuration || "0s").split(",").some((d) => parseFloat(d) > 0);
    if (dur && e.matches("a, button, summary, label, input, select, [role=button], [role=tab], [tabindex]")) { interactiveTransitions++; transitionKinds.add(s.transitionProperty); }
  }
  out.animated = animTargets.size;
  const all = new Set([...moved, ...animTargets].filter((e) => e.isConnected && sized(e)));
  const top = [...all].filter((e) => { for (let p = e.parentElement; p; p = p.parentElement) if (all.has(p)) return false; return true; });
  const groups = new Set(top.map((e) => e.parentElement));
  out.groups = groups.size;
  for (const e of top.slice(0, 8)) out.examples.push(e.tagName.toLowerCase() + (e.id ? "#" + e.id : "") + (typeof e.className === "string" && e.className.trim() ? "." + e.className.trim().split(" ").slice(0, 2).join(".") : ""));
  const canvases = Array.from(document.querySelectorAll("canvas")).filter((c) => { const r = c.getBoundingClientRect(); return r.width * r.height >= 200 * 150 && !all.has(c); });
  out.canvases = Math.min(2, canvases.length);
  out.interactive = interactiveTransitions === 0 ? 0 : (interactiveTransitions >= 6 && transitionKinds.size >= 2 ? 2 : 1);
  out.groups += out.canvases + out.interactive;
  return out;
})()`;

// Pinned stages: sticky elements at least half a viewport tall (not header/nav/aside) whose parent
// gives at least half a viewport of pinned scroll, and children of library pin spacers (.pin-spacer).
// Each is tagged data-layoza-pin so the collector can screenshot it at 25/50/75% of its range.
const PIN_FIND_SCRIPT = `(() => {
  const vh = window.innerHeight, vw = window.innerWidth;
  const docTop = (el) => el.getBoundingClientRect().top + window.scrollY;
  const name = (e) => e.tagName.toLowerCase() + (e.id ? "#" + e.id : "") + (typeof e.className === "string" && e.className.trim() ? "." + e.className.trim().split(" ").slice(0, 2).join(".") : "");
  const out = [];
  const taken = [];
  const add = (el, start, end) => {
    if (out.length >= 4 || end - start < vh * 0.5) return;
    if (taken.some((t) => t.contains(el) || el.contains(t))) return;
    taken.push(el);
    el.setAttribute("data-layoza-pin", String(out.length));
    out.push({ id: out.length, el: name(el), start: Math.max(0, Math.round(start)), end: Math.round(end) });
  };
  for (const sp of document.querySelectorAll(".pin-spacer")) {
    const c = sp.firstElementChild;
    if (!c) continue;
    const r = c.getBoundingClientRect();
    if (r.height < vh * 0.5 || r.width < vw * 0.4) continue;
    const start = docTop(sp);
    add(c, start, start + sp.getBoundingClientRect().height - r.height);
  }
  for (const el of document.body.querySelectorAll("*")) {
    const s = getComputedStyle(el);
    if (s.position !== "sticky" || el.closest("header, nav, aside, footer, [role=banner], [role=navigation]")) continue;
    const r = el.getBoundingClientRect();
    if (r.height < vh * 0.5 || r.width < vw * 0.4 || !el.parentElement) continue;
    const pr = el.parentElement.getBoundingClientRect();
    const t = parseFloat(s.top) || 0;
    const start = docTop(el.parentElement) - t;
    add(el, start, start + pr.height - r.height);
  }
  return out;
})()`;
// Viewport clip of a tagged pinned stage at the current scroll position
const PIN_CLIP_SCRIPT = `(id) => {
  const el = document.querySelector('[data-layoza-pin="' + id + '"]');
  if (!el) return null;
  const r = el.getBoundingClientRect();
  const x = Math.max(0, r.left), y = Math.max(0, r.top);
  const w = Math.min(window.innerWidth, r.right) - x, h = Math.min(window.innerHeight, r.bottom) - y;
  return w > 40 && h > 40 ? { x: Math.round(x), y: Math.round(y), width: Math.round(w), height: Math.round(h) } : null;
}`;
// Largest share of changed pixels (sum of abs RGB diff > 40) between any two of the stills, compared at
// 320px wide so antialiasing noise does not count
const PIN_DIFF_SCRIPT = `async (shots) => {
  const load = (src) => new Promise((res, rej) => { const im = new Image(); im.onload = () => res(im); im.onerror = () => rej(new Error("screenshot decode failed")); im.src = src; });
  const W = 320;
  const px = (im) => { const H = Math.max(1, Math.round(im.naturalHeight * W / im.naturalWidth)); const c = document.createElement("canvas"); c.width = W; c.height = H; const x = c.getContext("2d", { willReadFrequently: true }); x.drawImage(im, 0, 0, W, H); return x.getImageData(0, 0, W, H); };
  const imgs = [];
  for (const s of shots) imgs.push(px(await load(s)));
  let max = 0;
  for (let i = 0; i < imgs.length; i++) for (let j = i + 1; j < imgs.length; j++) {
    const A = imgs[i], B = imgs[j];
    const n = Math.min(A.data.length, B.data.length) / 4;
    let changed = 0;
    for (let k = 0; k < n; k++) { const q = k * 4; if (Math.abs(A.data[q] - B.data[q]) + Math.abs(A.data[q + 1] - B.data[q + 1]) + Math.abs(A.data[q + 2] - B.data[q + 2]) > 40) changed++; }
    max = Math.max(max, changed / Math.max(1, n));
  }
  return max;
}`;
const PIN_STATIC_MAX = 0.02; // a stage whose stills differ in under 2% of pixels reads as frozen
const PIN_FRACTIONS = [0.25, 0.5, 0.75];

/** Desktop motion pass on a fresh load: density probe, then pinned-stage stills. */
async function motionPassNode(page, url, caps) {
  const res = { density: null, pins: [], errors: [] };
  try {
    await page.goto(url, { waitUntil: "load", timeout: 30000 });
    await page.waitForLoadState("networkidle", { timeout: caps.netidle_ms }).catch(() => {});
    await fontsReadyNode(page, 2000);
    await page.waitForTimeout(WAIT.scroll_anim);
    res.density = await ev(page, MOTION_PROBE_SCRIPT);
    const pins = await ev(page, PIN_FIND_SCRIPT);
    for (const pin of pins) {
      const shots = [];
      let clip = null;
      for (const f of PIN_FRACTIONS) {
        await ev(page, ART_SCROLL_SCRIPT, Math.round(pin.start + (pin.end - pin.start) * f));
        await page.waitForTimeout(WAIT.pin);
        clip = clip || (await ev(page, PIN_CLIP_SCRIPT, pin.id));
        if (!clip) break;
        shots.push("data:image/png;base64," + (await page.screenshot({ clip })).toString("base64"));
      }
      if (shots.length === PIN_FRACTIONS.length) res.pins.push({ ...pin, max_change: await ev(page, PIN_DIFF_SCRIPT, shots) });
    }
    await ev(page, ART_SCROLL_SCRIPT, 0);
  } catch (e) {
    res.errors.push(String(e.message || e).split("\n")[0]);
  }
  return res;
}

/** One viewport pass (Node runner). wide = every check; otherwise layout, overflow and touch only. */
async function viewportPassNode(browser, vp, args, shotDir, caps, wide) {
      const context = await browser.newContext({ viewport: { width: vp.width, height: vp.height } });
      try {
      await context.addInitScript(PERF_INIT_SCRIPT);
      const page = await context.newPage();
      const consoleErrors = [];
      page.on("pageerror", (err) => consoleErrors.push(`PageError: ${err.message}`));
      page.on("console", (msg) => {
        if (msg.type() === "error") consoleErrors.push(`console.error: ${msg.text()}`);
      });

      await page.goto(args.url, { waitUntil: "load", timeout: 30000 });
      await page.waitForLoadState("networkidle", { timeout: caps.netidle_ms }).catch(() => {});
      await fontsReadyNode(page, 3000);

      // Intro animations finish before anything is measured (capped; reported when they never rest)
      const settleLoad = await settleNode(page, caps.settle_ms);
      // Read load-phase LCP/CLS before screenshots: full-page capture resizes the viewport and
      // would otherwise add late LCP candidates and layout shifts that real users never see.
      const loadPerf = await ev(page, PERF_READ_SCRIPT);
      await ev(page, SCROLL_PASS_SCRIPT);
      // Back at the top after the scroll pass: let reveals that re-ran come to rest (shares the cap)
      const settleTop = await settleNode(page, Math.max(SETTLE_STEP_MS * 3, caps.settle_ms - settleLoad.ms));
      const settle = { settled: settleLoad.settled && settleTop.settled, ms: settleLoad.ms + settleTop.ms, load: settleLoad, after_scroll: settleTop };
      const screenshot = path.join(shotDir, `${vp.label}.png`);
      let heroShot = null;
      if (vp.label === "mobile") {
        // 1:1 first phone screen, captured before the full-page capture resizes the viewport
        heroShot = path.join(shotDir, "mobile-hero.png");
        await page.screenshot({ path: heroShot, fullPage: false });
      }
      await page.screenshot({ path: screenshot, fullPage: true });
      const dom = await ev(page, DOM_EVAL_SCRIPT);
      let craft = null;
      if (wide) try {
        craft = await ev(page, CRAFT_EVAL_SCRIPT);
      } catch (e) {
        craft = { issues: [], suppressed: [], errors: [`craft evaluation failed: ${e.message || e}`] };
      }
      // Layoza-original generic-default tells join the craft critique (same severity and opt-out)
      if (wide) try {
        const tells = await ev(page, TELLS_EVAL_SCRIPT);
        craft.issues.push(...tells.issues);
        craft.suppressed = (craft.suppressed || []).concat(tells.suppressed);
        craft.errors = (craft.errors || []).concat(tells.errors);
      } catch (e) {
        craft.errors = (craft.errors || []).concat([`tells evaluation failed: ${e.message || e}`]);
      }
      let ui = null;
      if (wide) try {
        ui = await ev(page, INTERFACE_EVAL_SCRIPT);
      } catch (e) {
        ui = { issues: [], errors: [`interface evaluation failed: ${e.message || e}`] };
      }
      let quality;
      try {
        quality = await ev(page, QUALITY_EVAL({ allow: args.allow, skipFocus: vp.label !== "desktop", only: wide ? null : LAYOUT_ONLY_RULES }));
      } catch (e) {
        quality = { issues: [], suppressed: [], errors: [`quality sweep failed: ${e.message || e}`] };
      }
      let design = null;
      if (args.designEval && wide) {
        try {
          design = await ev(page, args.designEval);
        } catch (e) {
          design = { issues: [], errors: [`design-system evaluation failed: ${e.message || e}`] };
        }
      }
      const fingerprint = vp.label === "desktop" ? await fingerprintNode(page, vp) : null;

      const art = wide ? await artOverTextNode(page, vp) : null;
      const hiddenText = vp.label === "desktop" ? await ev(page, HIDDEN_TEXT_PROBE).catch(() => null) : null;

      // Scripted interactions (keyboard, then the mobile-menu click below) feed Event Timing for the
      // lab interaction proxy; it is read only after every interaction has run
      for (let i = 0; i < 3; i++) await page.keyboard.press("Tab");
      await page.keyboard.press("Shift+Tab");
      await page.waitForTimeout(WAIT.key);

      let menu = null;
      if (dom.menuToggleVisible) {
        const before = await ev(page, VISIBLE_NAV_LINKS_SCRIPT);
        try {
          await page.locator(MENU_TOGGLE_SELECTOR).first().click({ timeout: 3000 });
          // Poll until the drawer shows more links (up to 800ms) instead of a fixed wait
          let after = before;
          for (let ms = 0; ms < 800 && after <= before; ms += 100) {
            await page.waitForTimeout(100);
            after = await ev(page, VISIBLE_NAV_LINKS_SCRIPT);
          }
          await page.waitForTimeout(WAIT.key);
          const menuShot = path.join(shotDir, `${vp.label}-menu-open.png`);
          await page.screenshot({ path: menuShot, fullPage: false });
          menu = { links_before: before, links_after: after, opened: after > before, screenshot: menuShot };
          if (menu.opened) {
            // Escape from wherever focus sits after the click (usually the toggle) must close the menu
            await page.keyboard.press("Escape");
            // Drawers animate out: poll up to 1.5s, and record how long the close took
            menu.escape_closes = false;
            for (let ms = 100; ms <= 1500; ms += 100) {
              await page.waitForTimeout(100);
              if ((await ev(page, VISIBLE_NAV_LINKS_SCRIPT)) <= before) {
                menu.escape_closes = true;
                menu.escape_close_ms = ms;
                break;
              }
            }
          }
        } catch (e) {
          menu = { links_before: before, links_after: null, opened: false, error: String(e.message || e) };
        }
      }
      const inputPerf = await ev(page, PERF_READ_SCRIPT);
      const perf = loadPerf && inputPerf ? { ...loadPerf, inp_proxy_ms: inputPerf.inp_proxy_ms, event_entries: inputPerf.event_entries } : null;
      return { ...vp, checks: wide ? "full" : "layout", dom, craft, design, ui, quality, fingerprint, hiddenText, perf, menu, art, motion: null, consoleErrors, screenshot, heroShot, settle };
      } finally {
        await context.close();
      }
}

/** Desktop motion density + pinned stages on a fresh load in its own context (Node runner). */
async function motionContextNode(browser, url, caps) {
  const vp = VIEWPORTS[0];
  const ctx = await browser.newContext({ viewport: { width: vp.width, height: vp.height } });
  try {
    return await motionPassNode(await ctx.newPage(), url, caps);
  } finally {
    await ctx.close();
  }
}

/**
 * Collects raw per-viewport measurements with the Node `playwright` package: one browser, each
 * viewport and side pass in its own context, all in parallel.
 */
async function collectWithNode(pw, args, shotDir) {
  const full = Boolean(args.full);
  const caps = MODE_CAPS[full ? "full" : "fast"];
  const browser = await pw.chromium.launch({ headless: true });
  try {
    const firstLine = (e) => String((e && e.message) || e).split("\n")[0];
    const [motion, reduced, throttled, ...vps] = await Promise.all([
      motionContextNode(browser, args.url, caps).catch((e) => ({ density: null, pins: [], errors: [firstLine(e)] })),
      reducedMotionNode(browser, args.url, caps),
      full ? throttledPerfNode(browser, args.url) : Promise.resolve(null),
      ...VIEWPORTS.map((vp) => viewportPassNode(browser, vp, args, shotDir, caps, full || vp.label === "desktop")),
    ]);
    const raw = vps;
    raw[0].motion = motion;
    raw.extra = { reduced, throttled };
    raw.mode = full ? "full" : "fast";
    return raw;
  } finally {
    await browser.close();
  }
}

/** Same collection sequence through Python Playwright (falls back to installed Chrome/Edge). */
function collectWithPython(args, shotDir) {
  const pyCode = `
import json, sys, os, base64, time, asyncio
from playwright.async_api import async_playwright

url, raw_out, shot_dir, cfg_path = sys.argv[1:5]
cfg = json.load(open(cfg_path, encoding="utf-8"))
FULL = bool(cfg["full"])
CAPS = cfg["caps"]
WAIT = cfg["wait"]

def b64(png):
    return "data:image/png;base64," + base64.b64encode(png).decode("ascii")

def first_line(e):
    s = str(e).splitlines()
    return s[0] if s else repr(e)

async def quiet(page, cap_ms):
    # Event waits: network idle (capped), then fonts.ready (capped)
    try:
        await page.wait_for_load_state("networkidle", timeout=cap_ms)
    except Exception:
        pass
    try:
        await page.evaluate(cfg["fonts_ready"], 3000)
    except Exception:
        pass

async def settle(page, cap_ms):
    # Mirrors settleNode: finite animations done + first screen stable across two masked frames
    t0 = time.time()
    prev = None
    while True:
        last = await page.evaluate(cfg["settle_pending"] + "(" + str(cfg["settle_cap"]) + ")")
        shot = None
        if last["pending"] == 0:
            shot = b64(await page.screenshot(mask=[page.locator(cfg["settle_mask"])]))
            if prev and await page.evaluate(cfg["pin_diff"], [prev, shot]) <= cfg["settle_stable_max"]:
                return {"settled": True, "ms": round((time.time() - t0) * 1000), "pending_animations": 0, "endless_loops": last["loops"]}
        prev = shot
        elapsed = round((time.time() - t0) * 1000)
        if elapsed >= cap_ms:
            return {"settled": False, "ms": elapsed, "pending_animations": last["pending"], "endless_loops": last["loops"]}
        await page.wait_for_timeout(cfg["settle_step"])

async def text_over_graphic(page, tg):
    # Mirrors textOverGraphicNode; hits are folded in by the Node side (addTextGraphicHits)
    if tg["positions"] >= cfg["tg_max_positions"] or tg["sampled"] >= cfg["tg_max_total"]:
        return
    found = await page.evaluate(cfg["tg_find"] + "(" + str(min(cfg["tg_max_per_position"], cfg["tg_max_total"] - tg["sampled"])) + ")")
    if not found["candidates"]:
        return
    tg["positions"] += 1
    shown = await page.screenshot(clip=found["clip"])
    await page.evaluate(cfg["tg_hide"])
    try:
        await page.wait_for_timeout(60)
        hidden = await page.screenshot(clip=found["clip"])
    finally:
        await page.evaluate(cfg["tg_restore"])
    stats = await page.evaluate(cfg["tg_stats"], {"shown": b64(shown), "hidden": b64(hidden), "clip": found["clip"], "candidates": found["candidates"]})
    tg["sampled"] += len([x for x in stats if x])
    tg["raw"].append({"found": found, "stats": stats, "at": round(await page.evaluate("window.scrollY"))})

async def art_scan(page, vp):
    art = {"hits": [], "positions": 0, "diffs": 0, "errors": [], "tg": {"positions": 0, "sampled": 0, "raw": [], "errors": []}}
    art_t0 = time.time()
    try:
        step = 700 if vp["width"] < 768 else 760
        vh = vp["height"]
        scroll_h = await page.evaluate(cfg["art_scroll"], 0)
        y = 0
        while art["positions"] < cfg["art_max_scans"] and art["diffs"] < cfg["art_max_diffs"] and y < max(1, scroll_h - vh + step):
            scroll_h = await page.evaluate(cfg["art_scroll"], min(y, max(0, scroll_h - vh)))
            y += step
            art["positions"] += 1
            await page.wait_for_timeout(WAIT["art_step"])
            try:
                await text_over_graphic(page, art["tg"])
            except Exception as e:
                if len(art["tg"]["errors"]) < 3:
                    art["tg"]["errors"].append(first_line(e))
                try:
                    await page.evaluate(cfg["tg_restore"])
                except Exception:
                    pass
            found = await page.evaluate(cfg["art_lines"])
            if not found["lines"]:
                continue
            art["diffs"] += 1
            shown = [await page.screenshot(clip=found["clip"])]
            for _ in range(2 if art["positions"] == 1 else 0):
                await page.wait_for_timeout(WAIT["art_first"])
                shown.append(await page.screenshot(clip=found["clip"]))
            await page.evaluate(cfg["art_hide"])
            try:
                await page.wait_for_timeout(60)
                hidden = await page.screenshot(clip=found["clip"])
            finally:
                await page.evaluate(cfg["art_restore"])
            cover = await page.evaluate(cfg["art_diff"], {"shown": [b64(x) for x in shown], "b": b64(hidden), "lines": found["lines"], "clip": found["clip"]})
            at = round(await page.evaluate("window.scrollY"))
            for line, c in zip(found["lines"], cover):
                if c["line"] < cfg["art_cover_min"] and c["word"] < cfg["art_word_min"]:
                    continue
                prev = next((h for h in art["hits"] if h["text"] == line["text"]), None)
                if not prev:
                    art["hits"].append({"text": line["text"], "y": at, "cover": c["line"], "word": c["word"], "word_text": c["wordText"]})
                else:
                    prev["cover"] = max(prev["cover"], c["line"])
                    if c["word"] > prev["word"]:
                        prev["word"] = c["word"]
                        prev["word_text"] = c["wordText"]
        await page.evaluate(cfg["art_scroll"], 0)
        await page.evaluate(cfg["frames"])
    except Exception as e:
        art["errors"].append(first_line(e))
    art["ms"] = round((time.time() - art_t0) * 1000)
    return art

async def viewport_pass(browser, vp, wide):
    ctx = await browser.new_context(viewport={"width": vp["width"], "height": vp["height"]})
    try:
        await ctx.add_init_script(cfg["perf_init"])
        page = await ctx.new_page()
        console_errors = []
        page.on("pageerror", lambda err, ce=console_errors: ce.append(f"PageError: {err}"))
        page.on("console", lambda msg, ce=console_errors: ce.append(f"console.error: {msg.text}") if msg.type == "error" else None)

        await page.goto(url, wait_until="load", timeout=30000)
        await quiet(page, CAPS["netidle_ms"])
        settle_load = await settle(page, CAPS["settle_ms"])
        load_perf = await page.evaluate(cfg["perf_read"])
        await page.evaluate(cfg["scroll_pass"])
        settle_top = await settle(page, max(cfg["settle_step"] * 3, CAPS["settle_ms"] - settle_load["ms"]))
        settle_info = {"settled": settle_load["settled"] and settle_top["settled"], "ms": settle_load["ms"] + settle_top["ms"], "load": settle_load, "after_scroll": settle_top}
        shot = os.path.join(shot_dir, f"{vp['label']}.png")
        hero_shot = None
        if vp["label"] == "mobile":
            hero_shot = os.path.join(shot_dir, "mobile-hero.png")
            await page.screenshot(path=hero_shot, full_page=False)
        await page.screenshot(path=shot, full_page=True)
        dom = await page.evaluate(cfg["dom_eval"])
        craft = None
        ui = None
        design = None
        if wide:
            try:
                craft = await page.evaluate(cfg["craft_eval"])
            except Exception as e:
                craft = {"issues": [], "suppressed": [], "errors": ["craft evaluation failed: " + first_line(e)]}
            try:
                tells = await page.evaluate(cfg["tells_eval"])
                craft["issues"] = craft.get("issues", []) + tells["issues"]
                craft["suppressed"] = craft.get("suppressed", []) + tells["suppressed"]
                craft["errors"] = craft.get("errors", []) + tells["errors"]
            except Exception as e:
                craft["errors"] = craft.get("errors", []) + ["tells evaluation failed: " + first_line(e)]
            try:
                ui = await page.evaluate(cfg["interface_eval"])
            except Exception as e:
                ui = {"issues": [], "errors": ["interface evaluation failed: " + first_line(e)]}
        try:
            q = cfg["quality_desktop"] if vp["label"] == "desktop" else (cfg["quality_other"] if wide else cfg["quality_layout"])
            quality = await page.evaluate(q)
        except Exception as e:
            quality = {"issues": [], "suppressed": [], "errors": ["quality sweep failed: " + first_line(e)]}
        fingerprint = None
        if vp["label"] == "desktop":
            try:
                layout = await page.evaluate(cfg["layout_vector"])
                png = await page.screenshot(full_page=True, clip={"x": 0, "y": 0, "width": vp["width"], "height": max(1, layout["height_px"])})
                fingerprint = {"layout": layout, "palette": await page.evaluate(cfg["palette"], b64(png))}
            except Exception as e:
                fingerprint = {"error": first_line(e)}
        if wide and cfg.get("design_eval"):
            try:
                design = await page.evaluate(cfg["design_eval"])
            except Exception as e:
                design = {"issues": [], "errors": ["design-system evaluation failed: " + first_line(e)]}

        art = await art_scan(page, vp) if wide else None
        hidden_text = None
        if vp["label"] == "desktop":
            try:
                hidden_text = await page.evaluate(cfg["hidden_text"])
            except Exception:
                hidden_text = None

        for _ in range(3):
            await page.keyboard.press("Tab")
        await page.keyboard.press("Shift+Tab")
        await page.wait_for_timeout(WAIT["key"])

        menu = None
        if dom.get("menuToggleVisible"):
            before = await page.evaluate(cfg["nav_links"])
            try:
                await page.locator(cfg["menu_toggle"]).first.click(timeout=3000)
                # Poll until the drawer shows more links (up to 800ms) instead of a fixed wait
                after = before
                ms = 0
                while ms < 800 and after <= before:
                    await page.wait_for_timeout(100)
                    ms += 100
                    after = await page.evaluate(cfg["nav_links"])
                await page.wait_for_timeout(WAIT["key"])
                menu_shot = os.path.join(shot_dir, f"{vp['label']}-menu-open.png")
                await page.screenshot(path=menu_shot, full_page=False)
                menu = {"links_before": before, "links_after": after, "opened": after > before, "screenshot": menu_shot}
                if menu["opened"]:
                    await page.keyboard.press("Escape")
                    # Drawers animate out: poll up to 1.5s, and record how long the close took
                    menu["escape_closes"] = False
                    for step in range(1, 16):
                        await page.wait_for_timeout(100)
                        if await page.evaluate(cfg["nav_links"]) <= before:
                            menu["escape_closes"] = True
                            menu["escape_close_ms"] = step * 100
                            break
            except Exception as e:
                menu = {"links_before": before, "links_after": None, "opened": False, "error": first_line(e)}
        # Lab interaction proxy is read after every scripted interaction, including the menu click
        input_perf = await page.evaluate(cfg["perf_read"])
        perf = {**load_perf, "inp_proxy_ms": input_perf["inp_proxy_ms"], "event_entries": input_perf["event_entries"]} if load_perf and input_perf else None
        return {**vp, "checks": "full" if wide else "layout", "dom": dom, "craft": craft, "design": design, "ui": ui, "quality": quality, "fingerprint": fingerprint, "hiddenText": hidden_text, "perf": perf, "menu": menu, "art": art, "motion": None, "consoleErrors": console_errors, "screenshot": shot, "heroShot": hero_shot, "settle": settle_info}
    finally:
        await ctx.close()

async def motion_pass(browser):
    # Desktop motion density + pinned stages on a fresh load in its own context
    vp = cfg["viewports"][0]
    motion = {"density": None, "pins": [], "errors": []}
    ctx = await browser.new_context(viewport={"width": vp["width"], "height": vp["height"]})
    try:
        page = await ctx.new_page()
        await page.goto(url, wait_until="load", timeout=30000)
        await quiet(page, CAPS["netidle_ms"])
        await page.wait_for_timeout(WAIT["scroll_anim"])
        motion["density"] = await page.evaluate(cfg["motion_probe"])
        for pin in await page.evaluate(cfg["pin_find"]):
            shots, clip = [], None
            for f in cfg["pin_fractions"]:
                await page.evaluate(cfg["art_scroll"], round(pin["start"] + (pin["end"] - pin["start"]) * f))
                await page.wait_for_timeout(WAIT["pin"])
                clip = clip or await page.evaluate(cfg["pin_clip"], pin["id"])
                if not clip:
                    break
                shots.append(b64(await page.screenshot(clip=clip)))
            if len(shots) == len(cfg["pin_fractions"]):
                motion["pins"].append({**pin, "max_change": await page.evaluate(cfg["pin_diff"], shots)})
        await page.evaluate(cfg["art_scroll"], 0)
    except Exception as e:
        motion["errors"].append(first_line(e))
    finally:
        await ctx.close()
    return motion

async def reduced_pass(browser):
    # Reduced-motion run (desktop): text hidden only here is a reveal that reduce skips without its end state
    vp = cfg["viewports"][0]
    rctx = await browser.new_context(viewport={"width": vp["width"], "height": vp["height"]}, reduced_motion="reduce")
    try:
        rpage = await rctx.new_page()
        await rpage.goto(url, wait_until="load", timeout=30000)
        await quiet(rpage, CAPS["netidle_ms"])
        await rpage.wait_for_timeout(WAIT["reduced"])
        hidden = await rpage.evaluate(cfg["hidden_text"])
        try:
            running = await rpage.evaluate(cfg["running_motion"])
        except Exception:
            running = []
        widths = []
        for w in cfg["extra_widths"]:
            await rpage.set_viewport_size({"width": w, "height": 900})
            await rpage.evaluate(cfg["frames"])
            widths.append({"width": w, "overflow_px": await rpage.evaluate(cfg["overflow_probe"])})
        return {"hidden": hidden, "running": running, "widths": widths}
    except Exception as e:
        return {"hidden": [], "error": first_line(e)}
    finally:
        await rctx.close()

async def throttled_pass(browser):
    # Throttled lab load (mobile): CPU 4x + network emulation over CDP
    vp = next(v for v in cfg["viewports"] if v["label"] == "mobile")
    tctx = await browser.new_context(viewport={"width": vp["width"], "height": vp["height"]})
    try:
        await tctx.add_init_script(cfg["perf_init"])
        tpage = await tctx.new_page()
        cdp = await tctx.new_cdp_session(tpage)
        th = cfg["throttle"]
        await cdp.send("Network.enable")
        await cdp.send("Network.emulateNetworkConditions", {"offline": False, "latency": th["latency_ms"], "downloadThroughput": th["down_bytes_s"], "uploadThroughput": th["up_bytes_s"]})
        await cdp.send("Emulation.setCPUThrottlingRate", {"rate": th["cpu"]})
        t0 = time.time()
        await tpage.goto(url, wait_until="load", timeout=60000)
        load_ms = round((time.time() - t0) * 1000)
        await quiet(tpage, 4000)
        await settle(tpage, 3000)
        for _ in range(3):
            await tpage.keyboard.press("Tab")
        await tpage.wait_for_timeout(WAIT["key"])
        return {**((await tpage.evaluate(cfg["perf_read"])) or {}), "load_event_ms": load_ms}
    except Exception as e:
        return {"error": first_line(e)}
    finally:
        await tctx.close()

async def none():
    return None

async def main():
    async with async_playwright() as p:
        browser = None
        errors = []
        for kwargs in [{}, {"channel": "chrome"}, {"channel": "msedge"}]:
            try:
                browser = await p.chromium.launch(headless=True, **kwargs)
                break
            except Exception as e:
                errors.append(first_line(e))
        if browser is None:
            raise RuntimeError("Could not launch headless Chromium, Chrome, or Edge: " + " | ".join(errors))
        try:
            jobs = [viewport_pass(browser, vp, FULL or vp["label"] == "desktop") for vp in cfg["viewports"]]
            res = await asyncio.gather(motion_pass(browser), reduced_pass(browser), throttled_pass(browser) if FULL else none(), *jobs)
        finally:
            await browser.close()
    motion, reduced, throttled = res[0], res[1], res[2]
    raw = list(res[3:])
    raw[0]["motion"] = motion
    with open(raw_out, "w", encoding="utf-8") as f:
        json.dump({"viewports": raw, "extra": {"reduced": reduced, "throttled": throttled}}, f)

asyncio.run(main())
`;

  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "layoza-qa-"));
  const cfgPath = path.join(tmp, "cfg.json");
  const rawPath = path.join(tmp, "raw.json");
  fs.writeFileSync(
    cfgPath,
    JSON.stringify({
      viewports: VIEWPORTS,
      perf_init: PERF_INIT_SCRIPT,
      perf_read: PERF_READ_SCRIPT,
      dom_eval: DOM_EVAL_SCRIPT,
      craft_eval: CRAFT_EVAL_SCRIPT,
      tells_eval: TELLS_EVAL_SCRIPT,
      interface_eval: INTERFACE_EVAL_SCRIPT,
      design_eval: args.designEval || null,
      nav_links: VISIBLE_NAV_LINKS_SCRIPT,
      scroll_pass: SCROLL_PASS_SCRIPT,
      menu_toggle: MENU_TOGGLE_SELECTOR,
      art_scroll: ART_SCROLL_SCRIPT,
      art_lines: ART_LINES_SCRIPT,
      art_hide: ART_HIDE_SCRIPT,
      art_restore: ART_RESTORE_SCRIPT,
      art_diff: ART_DIFF_SCRIPT,
      art_max_scans: ART_MAX_SCANS,
      art_max_diffs: ART_MAX_DIFFS,
      art_cover_min: ART_COVER_MIN,
      art_word_min: ART_WORD_MIN,
      motion_probe: MOTION_PROBE_SCRIPT,
      pin_find: PIN_FIND_SCRIPT,
      pin_clip: PIN_CLIP_SCRIPT,
      pin_diff: PIN_DIFF_SCRIPT,
      pin_fractions: PIN_FRACTIONS,
      settle_pending: SETTLE_PENDING_SCRIPT,
      settle_cap: SETTLE_CAP_MS,
      settle_step: SETTLE_STEP_MS,
      settle_stable_max: SETTLE_STABLE_MAX,
      settle_mask: SETTLE_MASK_SELECTOR,
      tg_find: TG_FIND_SCRIPT,
      tg_hide: TG_HIDE_SCRIPT,
      tg_restore: TG_RESTORE_SCRIPT,
      tg_stats: TG_STATS_SCRIPT,
      tg_max_per_position: TG_MAX_PER_POSITION,
      tg_max_total: TG_MAX_TOTAL,
      tg_max_positions: TG_MAX_POSITIONS,
      quality_desktop: QUALITY_EVAL({ allow: args.allow, skipFocus: false }),
      quality_other: QUALITY_EVAL({ allow: args.allow, skipFocus: true }),
      quality_layout: QUALITY_EVAL({ allow: args.allow, skipFocus: true, only: LAYOUT_ONLY_RULES }),
      full: Boolean(args.full),
      caps: MODE_CAPS[args.full ? "full" : "fast"],
      wait: WAIT,
      frames: FRAMES_SCRIPT,
      fonts_ready: FONTS_READY_SCRIPT,
      layout_vector: LAYOUT_VECTOR_SCRIPT,
      palette: PALETTE_SCRIPT,
      hidden_text: HIDDEN_TEXT_PROBE,
      running_motion: RUNNING_MOTION_PROBE,
      overflow_probe: OVERFLOW_PROBE,
      extra_widths: EXTRA_WIDTHS,
      throttle: THROTTLE,
    }),
    "utf8"
  );
  try {
    // UTF-8 everywhere: Windows consoles default to cp1252, which cannot encode page text
    const res = spawnSync("python", ["-X", "utf8", "-c", pyCode, args.url, rawPath, shotDir, cfgPath], { stdio: "inherit", env: { ...process.env, PYTHONIOENCODING: "utf-8", PYTHONUTF8: "1" } });
    if (res.error || res.status !== 0) {
      throw new Error(
        "Python Playwright fallback failed. Install Node Playwright (`npm i -D playwright && npx playwright install chromium`) " +
          "or Python Playwright (`pip install playwright && playwright install chromium`)."
      );
    }
    const parsed = JSON.parse(fs.readFileSync(rawPath, "utf8"));
    const raw = parsed.viewports;
    raw.extra = parsed.extra;
    raw.mode = args.full ? "full" : "fast";
    // Fold the Python runner's raw text-over-graphic samples with the same rules as the Node runner
    for (const v of raw) {
      const t = v.art && v.art.tg;
      if (!t || !t.raw) continue;
      const tg = { positions: t.positions, sampled: 0, hits: [], errors: t.errors || [] };
      for (const r of t.raw) addTextGraphicHits(tg, r.found, r.stats, r.at);
      v.art.tg = tg;
    }
    return raw;
  } finally {
    fs.rmSync(tmp, { recursive: true, force: true });
  }
}

const maxOf = (vals) => {
  const xs = vals.filter((v) => typeof v === "number");
  return xs.length ? Math.max(...xs) : null;
};
const minOf = (vals) => {
  const xs = vals.filter((v) => typeof v === "number");
  return xs.length ? Math.min(...xs) : null;
};

/**
 * Top-level design-critique summary. `craft_critique` is {rule: count} using the largest count any
 * single viewport produced (rules re-fire per viewport; summing would triple-count page-level rules).
 */
function craftSummary(raw) {
  const perViewport = {};
  const summary = {};
  const suppressed = [];
  const errors = [];
  for (const v of raw) {
    const counts = {};
    for (const i of (v.craft && v.craft.issues) || []) counts[i.rule] = (counts[i.rule] || 0) + 1;
    // A viewport whose craft pass did not run (fast mode at 768/375) reports null, not "no findings"
    perViewport[v.label] = v.craft ? counts : null;
    for (const [rule, n] of Object.entries(counts)) summary[rule] = Math.max(summary[rule] || 0, n);
    for (const s of (v.craft && v.craft.suppressed) || []) {
      if (!suppressed.some((x) => x.rule === s.rule && x.detail === s.detail)) suppressed.push(s);
    }
    for (const e of (v.craft && v.craft.errors) || []) errors.push(`${v.label}: ${e}`);
  }
  const sorted = Object.fromEntries(Object.entries(summary).sort(([a], [b]) => a.localeCompare(b)));
  return {
    craft_critique: sorted,
    craft_critique_by_viewport: perViewport,
    craft_critique_suppressed: suppressed,
    craft_critique_errors: errors.slice(0, 20),
    craft_critique_attribution:
      "Design critique rules ported from pbakaus/impeccable (Apache-2.0), commit 2f98774, plus Layoza-original generic-default tells (ideas: anthropics/skills frontend-design, Apache-2.0); non-blocking (severity \"critique\").",
  };
}

const tokenCount = (v) => (Array.isArray(v) ? v.length : 0);
function designTokenSummary(tokens) {
  return {
    source: tokens.source || null,
    colors: tokenCount(tokens.colors),
    fonts: tokenCount(tokens.fonts),
    radii: tokenCount(tokens.radii),
    font_sizes: tokenCount(tokens.font_sizes),
    color_metric: "CIEDE2000 (CIELAB D65), off-system when ΔE00 > 3.0",
    tolerances_px: { radius: 1, font_size: 0.75 },
  };
}
/** {rule: distinct off-system values}, largest count any single viewport produced. */
function designSystemSummary(raw) {
  const out = {};
  const errors = [];
  for (const v of raw) {
    const counts = {};
    for (const i of (v.design && v.design.issues) || []) counts[i.rule] = (counts[i.rule] || 0) + 1;
    for (const [rule, n] of Object.entries(counts)) out[rule] = Math.max(out[rule] || 0, n);
    for (const e of (v.design && v.design.errors) || []) errors.push(`${v.label}: ${e}`);
  }
  return { findings: Object.fromEntries(Object.entries(out).sort(([a], [b]) => a.localeCompare(b))), errors: errors.slice(0, 20) };
}

/** Builds the report from raw measurements. Unmeasured values stay `null` and are listed in `unmeasured`. */
/**
 * Motion verdict for one viewport's motion pass (desktop only). Threshold: 6 distinct motion ideas for
 * bold/wild pages, 3 for calm ones (declared calm, or an undeclared app surface such as a table/grid
 * dashboard). `none`/`static` and brand-allow skip the density rule; pinned stages are always checked.
 */
function motionVerdict(m) {
  if (!m || !m.density) return m ? { errors: m.errors } : null;
  const d = m.density;
  let level = "bold";
  let source = "default";
  if (d.declared && ["calm", "subtle", "simple", "minimal"].includes(d.declared)) [level, source] = ["calm", "declared"];
  else if (d.declared && ["bold", "wild"].includes(d.declared)) [level, source] = [d.declared, "declared"];
  else if (d.app_surface) [level, source] = ["calm", "undeclared page with table/grid content, calm threshold"];
  const pins = m.pins.map((p) => ({ ...p, max_change: Number(p.max_change.toFixed(4)) }));
  // One scroll-driven set piece plus quiet feedback is a complete motion design, not a sparse one.
  const signature = pins.some((p) => p.max_change >= PIN_STATIC_MAX) || d.scroll_linked > 0;
  const threshold = signature ? MOTION_DENSITY_MIN.signature : level === "calm" ? MOTION_DENSITY_MIN.calm : MOTION_DENSITY_MIN.bold;
  return {
    level,
    level_source: source,
    declared: d.declared,
    skipped: d.skipped || (d.short_page ? "page shorter than 1.5 viewports" : null),
    groups: d.groups,
    threshold,
    signature,
    sparse: !d.skipped && !d.short_page && d.groups < threshold,
    changed_elements: d.changed,
    animated_elements: d.animated,
    scroll_linked: d.scroll_linked,
    examples: d.examples,
    pinned_stages: pins,
    static_pins: pins.filter((p) => p.max_change < PIN_STATIC_MAX),
    errors: m.errors,
  };
}

function assembleReport(url, engine, raw, tokens) {
  const viewports = raw.map((v) => {
    const issues = [...v.dom.issues];
    if (v.menu && v.menu.opened && v.menu.escape_closes === false) {
      issues.push({
        rule: "menu-escape-broken",
        severity: "warning",
        detail: "Pressing Escape with focus on the menu toggle leaves the open menu showing: listen for Escape on the document (or the toggle too), close the menu and return focus to the toggle.",
      });
    }
    if (v.menu && v.menu.escape_closes && v.menu.escape_close_ms > 800) {
      issues.push({
        rule: "menu-close-slow",
        severity: "warning",
        detail: `The menu takes about ${v.menu.escape_close_ms}ms to leave the screen after Escape: keep exit animations near 200-300ms so closing feels instant.`,
      });
    }
    if (v.menu && !v.menu.opened) {
      issues.push({
        rule: "menu-toggle-broken",
        severity: "error",
        detail: v.menu.error
          ? `Menu toggle click failed: ${v.menu.error}`
          : `Menu toggle click revealed no additional nav links (${v.menu.links_before} → ${v.menu.links_after})`,
      });
    }
    const artHits = (v.art && v.art.hits) || [];
    if (artHits.length) {
      const list = artHits
        .slice(0, 6)
        .map((h) => {
          const word = h.cover < ART_COVER_MIN ? `, ${Math.round(h.word * 100)}% of "${h.word_text}"` : "";
          return `"${h.text}" @${h.y}px (${Math.round(h.cover * 100)}% of line${word} covered)`;
        })
        .join("; ");
      issues.push({
        rule: "art-over-text",
        severity: "warning",
        detail: `${artHits.length} text line(s) have canvas/video art painted across them at a resting scroll position: ${list}; give the text its own zone or a solid plate and keep the art beside or behind it, never through it`,
      });
    }
    const tg = (v.art && v.art.tg) || null;
    const tgHits = (tg && tg.hits) || [];
    const tgList = (xs, f) => xs.slice(0, 4).map(f).join("; ");
    const occluded = tgHits.filter((h) => h.occluded);
    if (occluded.length) {
      issues.push({
        rule: "text-occluded",
        severity: "warning",
        detail: `${occluded.length} text label(s) over a graphic are painted over by something above them (hiding the text changes no pixel): ${tgList(occluded, (h) => `"${h.text}" ${h.el} over ${h.graphic} @${h.y}px`)}; move the text clear of the covering layer or raise it above (z-index / DOM order)`,
      });
    }
    const lowC = tgHits.filter((h) => h.low_contrast);
    const hardFail = lowC.filter((h) => h.contrast_median < h.required && !h.halo);
    for (const [list, severity] of [[hardFail, "error"], [lowC.filter((h) => !hardFail.includes(h)), "warning"]]) {
      if (!list.length) continue;
      issues.push({
        rule: "text-over-graphic-contrast",
        severity,
        detail: `${list.length} text element(s) over canvas/video/image art measure below ${list[0].required}:1 against the real pixels behind them${severity === "error" ? " (median pixel fails)" : " (10% of the pixels behind fail)"}: ${tgList(list, (h) => `"${h.text}" ${h.el} over ${h.graphic} @${h.y}px p10 ${h.contrast_p10}:1, median ${h.contrast_median}:1 (needs ${h.required}:1)`)}; put the text on a scrim/plate, move it to a calm part of the art, or change its colour`,
      });
    }
    const busy = tgHits.filter((h) => h.busy && !h.low_contrast);
    if (busy.length) {
      issues.push({
        rule: "text-over-busy-graphic",
        severity: "warning",
        detail: `${busy.length} text element(s) sit on a busy graphic (luminance L* std >= ${TG_BUSY_LSTD} behind the glyphs): ${tgList(busy, (h) => `"${h.text}" over ${h.graphic} @${h.y}px L* std ${h.l_std}`)}; calm the area behind the text (scrim, blur plate, or place the label beside the art)`,
      });
    }
    const motion = motionVerdict(v.motion);
    if (motion && motion.sparse) {
      issues.push({
        rule: "motion-too-sparse",
        severity: "warning",
        detail: `Only ${motion.groups} distinct motion idea(s) found across scroll positions and declared animations (need ${motion.threshold} for a ${motion.level} page${motion.level_source === "declared" ? "" : `, ${motion.level_source}`}): build one scroll- or hand-driven signature set piece from the subject (a pinned stage that changes with scroll counts), or declare <meta name="layoza-motion" content="calm"> if the page is meant to be quiet`,
      });
    }
    for (const pin of (motion && motion.static_pins) || []) {
      issues.push({
        rule: "pinned-stage-static",
        severity: "warning",
        detail: `Pinned stage ${pin.el} holds for ${pin.end - pin.start}px of scroll but its stills at 25/50/75% differ in only ${(pin.max_change * 100).toFixed(1)}% of pixels: map a visible change (step, reveal, transform) to the pinned progress or unpin it`,
      });
    }
    if (v.dom.brokenImages.length) {
      issues.push({ rule: "broken-image", severity: "error", detail: v.dom.brokenImages.slice(0, 5).join(", ") });
    }
    // Non-blocking design critique (severity "critique"), see the impeccable port above
    issues.push(...((v.craft && v.craft.issues) || []));
    // Objective design-system drift (severity "warning"), only when the project declares tokens
    issues.push(...((v.design && v.design.issues) || []));
    // Objective interface hygiene (warning; zoom-blocked is an error), Layoza-original
    issues.push(...((v.ui && v.ui.issues) || []));
    // Layoza quality sweep: warnings are objective, critiques are brief-aware style advice
    issues.push(...((v.quality && v.quality.issues) || []));
    // Text that stays invisible only when the visitor asks for reduced motion: the reveal skipped its end state
    if (v.label === "desktop" && raw.extra && raw.extra.reduced && Array.isArray(raw.extra.reduced.hidden) && Array.isArray(v.hiddenText)) {
      const normal = new Set(v.hiddenText.map((h) => h.selector));
      const only = raw.extra.reduced.hidden.filter((h) => !normal.has(h.selector));
      if (only.length) {
        issues.push({
          rule: "reduced-motion-hides-text",
          severity: "warning",
          category: "motion",
          selector: only[0].selector,
          measured: `${only.length} text block(s) at opacity ${only[0].opacity} / visibility ${only[0].visibility} under prefers-reduced-motion: reduce`,
          why: "Visitors who turn motion off never see this text: the reveal animation is skipped but its final visible state is never applied.",
          fix: "Inside @media (prefers-reduced-motion: reduce), set the end state directly (opacity:1; transform:none) instead of only removing the animation.",
          detail: only.slice(0, 5).map((h) => `${h.selector} "${h.text}"`).join("; "),
        });
      }
    }
    if (v.label === "desktop" && raw.extra && raw.extra.reduced) {
      const red = raw.extra.reduced;
      if (Array.isArray(red.running) && red.running.length) {
        issues.push({
          rule: "motion-ignores-reduce",
          severity: "warning",
          category: "motion",
          selector: red.running[0].split(" ")[0],
          measured: `${red.running.length} long or endless animation(s) still running under prefers-reduced-motion: reduce`,
          why: "Animations keep running when the visitor has asked the system to reduce motion, which can trigger nausea and vestibular symptoms.",
          fix: "Wrap decorative keyframes and transforms in @media (prefers-reduced-motion: no-preference), or stop them under reduce.",
          detail: red.running.slice(0, 5).join("; "),
        });
      }
      for (const w of red.widths || []) {
        if (!(w.overflow_px > 2)) continue;
        issues.push({
          rule: "overflow-at-width",
          severity: "error",
          category: "layout",
          selector: "html",
          measured: `${w.overflow_px}px of sideways scroll at ${w.width}px wide`,
          why: "The page scrolls sideways at this width, so content is cut off and the whole page wobbles on touch.",
          fix: "Find the element wider than the viewport (fixed widths, 100vw plus padding, long words, wide media) and constrain it with max-width: 100%, minmax(0, 1fr) or overflow-wrap.",
          detail: `${w.overflow_px}px horizontal overflow at ${w.width}px (extra width sweep)`,
        });
      }
    }
    return {
      label: v.label,
      width: v.width,
      height: v.height,
      // "full" = every check ran here; "layout" = layout, overflow and touch rules only (fast mode)
      checks: v.checks || "full",
      horizontal_overflow_px: v.dom.horizontalOverflowPx,
      clipped_text_nodes: v.dom.clippedTextNodes,
      overlapping_interactive_elements: v.dom.overlappingInteractiveElements,
      broken_images: v.dom.brokenImages.length,
      issues,
      console_errors: v.consoleErrors,
      min_contrast_ratio: v.dom.minContrastRatio,
      contrast_failures: v.dom.contrastFailures,
      lcp_ms: v.perf ? v.perf.lcp_ms : null,
      cls: v.perf ? v.perf.cls : null,
      inp_proxy_ms: v.perf ? v.perf.inp_proxy_ms : null,
      mobile_menu: v.menu,
      art_over_text: v.art
        ? { positions: v.art.positions, diffs: v.art.diffs, ms: v.art.ms, hits: v.art.hits.map((h) => ({ ...h, cover: Number(h.cover.toFixed(2)), word: Number(h.word.toFixed(2)) })), errors: v.art.errors }
        : null,
      text_over_graphic: tg ? { positions: tg.positions, sampled: tg.sampled, hits: tgHits, errors: tg.errors } : null,
      motion: motion,
      // How long the page took to come to rest before measuring; settled false = hit the cap
      motion_settle: v.settle || null,
      screenshot_path: v.screenshot,
      hero_screenshot_path: v.heroShot || null,
    };
  });

  const desktop = raw.find((v) => v.label === "desktop") || raw[0];
  const d = desktop.dom;
  const contrastSamples = [...new Set(raw.flatMap((v) => v.dom.contrastSamples))].slice(0, 10);

  const report = {
    schema_version: "2.0.0",
    url,
    engine,
    methodology: "laboratory_synthetic",
    methodology_disclosure:
      "Single headless Chromium load per viewport on this machine. LCP/CLS come from PerformanceObserver; inp_ms is a lab interaction proxy (worst Event Timing duration >= 16ms across scripted Tab/Shift+Tab presses and, when a mobile menu toggle exists, its click), not field INP. These are lab diagnostics, not CrUX field data.",
    viewports,
    // Worst value across viewports
    min_contrast_ratio: minOf(raw.map((v) => v.dom.minContrastRatio)),
    contrast_failures: Math.max(...raw.map((v) => v.dom.contrastFailures)),
    contrast_failure_samples: contrastSamples,
    decorative_contrast_failures: Math.max(...raw.map((v) => v.dom.decorativeContrastFailures)),
    decorative_contrast_samples: [...new Set(raw.flatMap((v) => v.dom.decorativeSamples))].slice(0, 5),
    contrast_checked_elements: d.contrastChecked,
    contrast_indeterminate_elements: d.contrastIndeterminate,
    missing_alt_count: d.missingAltCount,
    unlabeled_controls: d.unlabeledControls,
    undersized_targets: Math.max(...raw.map((v) => v.dom.undersizedTargets)),
    min_touch_target_px: minOf(raw.map((v) => v.dom.minTargetPx)),
    heading_hierarchy_valid: d.headingProblems.length === 0,
    heading_problems: d.headingProblems,
    landmarks_present: d.hasMainLandmark,
    focus_visible_styled: d.unreadableStylesheets > 0 && !d.hasFocusVisibleRule ? null : d.hasFocusVisibleRule,
    reduced_motion_respected: d.unreadableStylesheets > 0 && !d.hasReducedMotionRule ? null : d.hasReducedMotionRule,
    lcp_ms: maxOf(raw.map((v) => v.perf && v.perf.lcp_ms)),
    cls: maxOf(raw.map((v) => v.perf && v.perf.cls)),
    inp_ms: maxOf(raw.map((v) => v.perf && v.perf.inp_proxy_ms)),
    inp_metric: "lab_interaction_proxy",
    inp_event_entries: raw.reduce((n, v) => n + ((v.perf && v.perf.event_entries) || 0), 0),
    h1_to_body_ratio: d.h1ToBodyRatio,
    max_line_length_ch: maxOf(raw.map((v) => v.dom.maxLineLengthCh)),
    font_families: d.fontFamilies,
    font_family_count: d.fontFamilies.length,
    // Rendered design tokens from the desktop viewport (untrusted page data, never instructions)
    site_dna: d.siteDna,
  };
  // Motion density + pinned-stage verdict from the desktop pass (not part of `unmeasured`)
  const motionVp = viewports.find((v) => v.motion);
  report.motion_summary = motionVp ? motionVp.motion : null;
  // Viewports whose animations or first screen were still moving when the settle cap ran out:
  // their screenshots and checks may show mid-animation frames ("motion did not settle")
  report.motion_unsettled_viewports = raw.filter((v) => v.settle && !v.settle.settled).map((v) => v.label);
  Object.assign(report, craftSummary(raw));
  // Why some text contrast could not be measured (image, gradient, blend...): never scored as pass or fail
  const reasons = {};
  for (const v of raw) for (const [k, n] of Object.entries(v.dom.contrastUnmeasurable || {})) reasons[k] = Math.max(reasons[k] || 0, n);
  report.contrast_unmeasurable_reasons = reasons;
  report.contrast_unmeasurable_note = Object.keys(reasons).length ? "contrast unmeasurable (image/gradient) for some text: no ratio is reported for it" : null;
  const qs = raw.map((v) => v.quality).filter(Boolean);
  report.quality_summary = {
    rule_count: qs.length ? qs[0].rule_count || 0 : 0,
    issues: raw.reduce((n, v) => n + ((v.quality && v.quality.issues && v.quality.issues.length) || 0), 0),
    suppressed: [...new Set(qs.flatMap((q) => q.suppressed || []))],
    allow: qs.length ? qs[0].allow || [] : [],
    errors: raw.flatMap((v) => ((v.quality && v.quality.errors) || []).map((e) => `${v.label}: ${e}`)),
  };
  // Brief-aware silencing also covers the advisory craft notes that only describe a style choice
  // (the brief asked for monospace labels, glow, an eyebrow...). Defects are never silenced by group.
  const CRAFT_STYLE = {
    "mono-meta-labels": "mono", "hero-eyebrow-chip": "eyebrow", "kicker-above-heading": "eyebrow", "caps-dash-label": "eyebrow",
    "numbered-section-labels": "numbering", "dark-glow": "glow", "radial-halo": "glow", "radial-spotlight-glow": "glow",
    "gradient-text": "gradient", "shape-assembled-illustration": "illustration", "uniform-radius": "pill",
    "gpt-thin-border-wide-shadow": "shadow", "card-kit": "structure", "italic-serif-display": "type",
  };
  const allowSet = new Set(report.quality_summary.allow.map((a) => String(a).toLowerCase()));
  if (allowSet.size) {
    for (const v of report.viewports || []) {
      v.issues = (v.issues || []).filter((i) => {
        const id = String(i.rule || "").replace(/^craft-/, "");
        if (i.severity !== "critique" || !(allowSet.has(id) || allowSet.has(i.rule) || allowSet.has(CRAFT_STYLE[id]))) return true;
        if (!report.quality_summary.suppressed.some((s) => s.rule === i.rule))
          report.quality_summary.suppressed.push({ rule: i.rule, detail: "allowed by the brief (" + (allowSet.has(CRAFT_STYLE[id]) ? CRAFT_STYLE[id] : id) + ")" });
        return false;
      });
    }
  }
  report.look_fingerprint = desktop.fingerprint && !desktop.fingerprint.error ? desktop.fingerprint : null;
  // Repeated section structure inside this page (one WARN per pair, on the desktop viewport)
  const twice = report.look_fingerprint && !allowSet.has("same-section-twice") ? sameSectionTwice(report.look_fingerprint.layout.sections) : [];
  const deskOut = (report.viewports || []).find((v) => v.label === "desktop");
  if (deskOut && Array.isArray(deskOut.issues)) {
    for (const t of twice.slice(0, 6)) {
      deskOut.issues.push({
        rule: "same-section-twice",
        severity: "warning",
        category: "design-quality",
        selector: "section #" + t.a.index + " / #" + t.b.index,
        detail: "Sections #" + t.a.index + " (" + t.a.label + ") and #" + t.b.index + " (" + t.b.label + ") share one structure (similarity " + t.sim + ").",
        measured: "structure similarity " + t.sim + " (flag at >= 0.93)",
        why: "The page repeats one section layout, so the scroll reads as the same block twice and the second one adds no new shape.",
        fix: "Give one of the two sections its own composition (a different column count, media placement or reading order), or merge them.",
      });
    }
  }
  const mobile = raw.find((v) => v.label === "mobile");
  const thr = raw.extra && raw.extra.throttled;
  report.lab_perf = {
    methodology: "laboratory_synthetic",
    viewport: "mobile",
    unthrottled: mobile && mobile.perf ? { lcp_ms: mobile.perf.lcp_ms, cls: mobile.perf.cls } : null,
    throttled: thr && !thr.error ? { lcp_ms: thr.lcp_ms ?? null, cls: thr.cls ?? null, load_event_ms: thr.load_event_ms ?? null } : null,
    throttled_error: thr && thr.error ? thr.error : null,
    throttled_skipped: thr ? null : "skipped (fast mode); run with --full for the throttled pass",
    profile: THROTTLE,
    inp: "lab interaction proxy only (scripted key presses), not field INP",
    note: url.startsWith("file:") ? "file:// loads bypass the network, so only the CPU throttle applies; serve over http for network-throttled numbers" : "CDP network emulation + CPU slowdown; Lab data, not field (CrUX) data",
  };
  // Which passes ran. Nothing is reported from a pass that did not run.
  const mode = raw.mode === "full" ? "full" : "fast";
  const layoutOnly = raw.filter((v) => v.checks === "layout").map((v) => v.width);
  const passes = raw.map((v) => (v.checks === "layout" ? "layout-rules@" : "full-sweep@") + v.width);
  if (raw.extra && raw.extra.reduced && !raw.extra.reduced.error) passes.push("reduced-motion@" + VIEWPORTS[0].width, "extra-widths@" + EXTRA_WIDTHS.join(","));
  if (desktop.motion) passes.push("motion-density@" + desktop.width);
  if (thr) passes.push("throttled-perf@" + (VIEWPORTS.find((v) => v.label === "mobile") || {}).width);
  const skipped = [];
  if (!thr) skipped.push("Throttled Lab LCP/CLS (slow-device pass): skipped in fast mode; run with --full");
  if (layoutOnly.length) skipped.push("Full rule sweep, craft, tells, interface, design-token and art-over-text checks at " + layoutOnly.join("px, ") + "px: fast mode ran layout, overflow and touch rules only there; run with --full");
  if (raw.extra && raw.extra.reduced && raw.extra.reduced.error) skipped.push("Reduced-motion and extra-width pass failed: " + raw.extra.reduced.error);
  report.mode = mode;
  report.passes = passes;
  report.skipped_passes = skipped;
  report.methodology_disclosure += mode === "fast"
    ? " Fast mode: one browser, viewports and side passes in parallel contexts (parallel loads share the CPU, so lab timings can read higher than a lone load)."
    : " Full mode: adds the throttled slow-device pass and the full rule sweep at every width.";
  report.unmeasured = Object.entries(report)
    .filter(([, v]) => v === null)
    .map(([k]) => k);
  // Declared design tokens (DESIGN.md / SITE-DNA.json) the page was checked against; `null` means the
  // project declares none, so no design-system rule ran (not a measurement gap, so not in `unmeasured`).
  report.design_tokens = tokens ? designTokenSummary(tokens) : null;
  report.design_system = tokens ? designSystemSummary(raw) : null;
  return report;
}

async function main() {
  const t0 = Date.now();
  const args = parseArgs(process.argv);
  if (args.compare) {
    const [a, b] = args.compare.map((p) => {
      const r = JSON.parse(fs.readFileSync(p, "utf8").replace(/^\uFEFF/, ""));
      return r.look_fingerprint || null;
    });
    const verdict = compareFingerprints(a, b);
    console.log(JSON.stringify(verdict, null, 2));
    if (args.outGiven) fs.writeFileSync(path.resolve(args.out), JSON.stringify(verdict, null, 2), "utf8");
    process.exit(verdict.verdict === "re-skin" ? 2 : 0);
  }
  if (!args.url) {
    console.error("Usage: node scripts/browser-qa.mjs (--url <http-url> | --file <html-path>) [--out <json-path>] [--screenshots <dir>] [--design-tokens <json-path>] [--allow rule-or-group,...] [--full] | --compare <a.json> <b.json>");
    process.exit(1);
  }
  const outPath = path.resolve(args.out);
  const shotDir = path.resolve(args.screenshotDir);
  fs.mkdirSync(path.dirname(outPath), { recursive: true });
  fs.mkdirSync(shotDir, { recursive: true });
  // Never leave a stale report behind if this run fails
  fs.rmSync(outPath, { force: true });

  // Layoza runs this script from a temp folder, where a bare import cannot see the project's
  // node_modules. Resolve playwright from the working directory (the project) first.
  // LAYOZA_PLAYWRIGHT_DIR may point at a folder holding node_modules/playwright (a scratch
  // install), so the Node runner works without adding playwright to the project.
  let pw = null;
  const bases = [process.env.LAYOZA_PLAYWRIGHT_DIR, process.cwd()].filter(Boolean);
  for (const base of bases) {
    try {
      const req = createRequire(path.join(path.resolve(base), "noop.js"));
      const mod = await import(pathToFileURL(req.resolve("playwright")).href);
      // A CommonJS entry imported by file URL may expose its exports only on `default`
      pw = mod.chromium ? mod : mod.default;
      break;
    } catch {
      pw = null;
    }
  }
  if (!pw && process.env.LAYOZA_QA_ENGINE !== "python") {
    try {
      const mod = await import("playwright");
      pw = mod.chromium ? mod : mod.default;
    } catch {
      pw = null;
    }
  }
  if (process.env.LAYOZA_QA_ENGINE === "python") pw = null;
  let tokens = null;
  if (args.designTokens) {
    try {
      tokens = JSON.parse(fs.readFileSync(args.designTokens, "utf8").replace(/^﻿/, ""));
    } catch (e) {
      console.error(`Cannot read --design-tokens ${args.designTokens}: ${e.message || e}`);
      process.exit(1);
    }
    const empty = ["colors", "fonts", "radii", "font_sizes"].every((k) => tokenCount(tokens[k]) === 0);
    if (empty) tokens = null;
    else args.designEval = "(" + layozaDesignSystemCheck.toString() + ")(" + JSON.stringify(tokens) + ")";
  }
  const engine = pw ? "node-playwright" : "python-playwright";
  const raw = pw ? await collectWithNode(pw, args, shotDir) : collectWithPython(args, shotDir);
  const report = assembleReport(args.url, engine, raw, tokens);
  report.runtime_ms = Date.now() - t0;

  fs.writeFileSync(outPath, JSON.stringify(report, null, 2), "utf8");
  console.log(`Layoza Browser QA report written to ${outPath} (${engine}, real Chromium, ${raw.length} viewports)`);
  if (report.unmeasured.length) console.log(`Unmeasured: ${report.unmeasured.join(", ")}`);
  console.log(`Quality sweep: ${report.quality_summary.rule_count} rules, ${report.quality_summary.issues} finding(s); runtime ${(report.runtime_ms / 1000).toFixed(1)}s`);
}

main().catch((err) => {
  console.error(err && err.stack ? err.stack : err);
  process.exit(1);
});
