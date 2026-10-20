#!/usr/bin/env node
/**
 * Layoza render facts (Playwright + Chromium), Layoza-original.
 *
 * Loads a live URL or local HTML file in headless Chromium at 1440x900 and 390x844, scrolls once
 * through the page so scroll reveals run, and measures the rendered facts that `layoza conform`
 * compares with a dealt page shape: typefaces, monospace, eyebrows and tiny tracked caps, index
 * numerals, gradients/glows/glass/blobs, icon-card and identical-card rows, h1 count and lines,
 * statement length, section gaps, real pictures, the live interaction and its fallback, the footer
 * brand line, button shapes and motif hints. Read-only: nothing on the page is clicked or changed,
 * and no request is made besides the page load itself.
 *
 * Uses the Node `playwright` package when installed, otherwise Python `playwright.async_api`
 * (the same resolution order as browser-qa.mjs). Writes `{ url, engine, viewports: [...] }`.
 */

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";

const WIDTHS = [
  { label: "desktop", width: 1440, height: 900 },
  { label: "mobile", width: 390, height: 844 },
];

const FONTS_READY = "(cap) => Promise.race([document.fonts ? document.fonts.ready.then(() => true) : Promise.resolve(true), new Promise((r) => setTimeout(() => r(false), cap))])";

// One visitor pass down the page and back so scroll-triggered reveals have run before measuring.
const SCROLL_THROUGH = `(async () => {
  const go = (top) => window.scrollTo({ top, left: 0, behavior: "instant" });
  const step = Math.max(300, Math.floor(window.innerHeight * 0.8));
  const h = document.documentElement.scrollHeight;
  for (let y = 0; y < h && y < 40000; y += step) { go(y); await new Promise((r) => setTimeout(r, 120)); }
  go(0);
  await new Promise((r) => setTimeout(r, 700));
  return true;
})()`;

/** In-page collector. Pure reads of computed style and geometry. */
function layozaRenderFacts() {
  const vw = window.innerWidth;
  const sy = window.scrollY;
  const px = (v) => parseFloat(v) || 0;
  const SKIP = new Set(["SCRIPT", "STYLE", "NOSCRIPT", "TEMPLATE", "HEAD", "META", "LINK", "TITLE"]);
  const visCache = new Map();
  const visible = (el) => {
    if (!el || el.nodeType !== 1) return false;
    if (visCache.has(el)) return visCache.get(el);
    let ok;
    if (SKIP.has(el.tagName)) ok = false;
    else if (el.checkVisibility) ok = el.checkVisibility({ opacityProperty: true, visibilityProperty: true });
    else {
      const st = getComputedStyle(el);
      ok = st.display !== "none" && st.visibility !== "hidden" && parseFloat(st.opacity) > 0;
    }
    if (ok) {
      const r = el.getBoundingClientRect();
      ok = r.width > 0 && r.height > 0;
    }
    visCache.set(el, ok);
    return ok;
  };
  const firstFamily = (ff) => (ff || "").split(",")[0].trim().replace(/^["']|["']$/g, "");
  const MONO = /(^|\b)(monospace|ui-monospace|mono|courier|consolas|menlo|monaco|code)(\b|$)/i;
  const ICON_FONT = /icon|awesome|material symbols|glyph|fontello/i;
  const textOf = (el) => (el.innerText || el.textContent || "").replace(/\s+/g, " ").trim();
  const short = (s, n = 60) => (s.length > n ? s.slice(0, n - 1) + "…" : s);
  const sel = (el) => {
    let s = el.tagName.toLowerCase();
    if (el.id) s += "#" + el.id;
    const c = typeof el.className === "string" ? el.className.trim().split(/\s+/).filter(Boolean).slice(0, 2) : [];
    if (c.length) s += "." + c.join(".");
    return s;
  };
  const docRect = (el) => {
    const r = el.getBoundingClientRect();
    return { top: r.top + sy, bottom: r.bottom + sy, left: r.left, right: r.right, width: r.width, height: r.height };
  };
  const parseColor = (s) => {
    const m = /rgba?\(([^)]+)\)/.exec(s || "");
    if (!m) return null;
    const p = m[1].split(/[\s,/]+/).filter(Boolean).map(parseFloat);
    return { r: p[0], g: p[1], b: p[2], a: p.length > 3 ? p[3] : 1 };
  };
  const chroma = (c) => (c ? Math.max(c.r, c.g, c.b) - Math.min(c.r, c.g, c.b) : 0);

  // Text nodes: typefaces by character count, monospace runs, the body size
  const fontChars = new Map();
  const sizeChars = new Map();
  const mono = { chars: 0, samples: [] };
  const walker = document.createTreeWalker(document.body || document.documentElement, NodeFilter.SHOW_TEXT);
  for (let n = walker.nextNode(); n; n = walker.nextNode()) {
    const t = n.nodeValue.replace(/\s+/g, " ").trim();
    if (!t) continue;
    const el = n.parentElement;
    if (!visible(el)) continue;
    const st = getComputedStyle(el);
    const fam = firstFamily(st.fontFamily);
    if (ICON_FONT.test(fam)) continue;
    fontChars.set(fam, (fontChars.get(fam) || 0) + t.length);
    const fs = Math.round(px(st.fontSize));
    sizeChars.set(fs, (sizeChars.get(fs) || 0) + t.length);
    if (MONO.test(fam)) {
      mono.chars += t.length;
      if (mono.samples.length < 5) mono.samples.push({ el: sel(el), family: fam, text: short(t, 40) });
    }
  }
  const fonts = [...fontChars.entries()].sort((a, b) => b[1] - a[1]).map(([family, chars]) => ({ family, chars }));
  const bodyPx = [...sizeChars.entries()].sort((a, b) => b[1] - a[1]).map(([s]) => s)[0] || 16;

  // Eyebrows: a short small label directly before an h1/h2/h3; tiny tracked caps anywhere
  const isUpper = (el, t, st) => st.textTransform === "uppercase" || (/[A-Z]{3}/.test(t) && t === t.toUpperCase() && /[A-Z]/.test(t));
  const trackingEm = (st) => (st.letterSpacing === "normal" ? 0 : px(st.letterSpacing) / Math.max(1, px(st.fontSize)));
  const eyebrows = [];
  for (const h of document.querySelectorAll("h1, h2, h3")) {
    if (!visible(h)) continue;
    let prev = h.previousElementSibling;
    while (prev && !visible(prev)) prev = prev.previousElementSibling;
    if (!prev && h.parentElement && h.parentElement.firstElementChild === h) {
      prev = h.parentElement.previousElementSibling;
      while (prev && !visible(prev)) prev = prev.previousElementSibling;
    }
    if (!prev || /^(H[1-6]|IMG|PICTURE|SVG|VIDEO|FIGURE|NAV|HEADER|UL|OL)$/i.test(prev.tagName)) continue;
    const t = textOf(prev);
    if (!t || t.length > 48 || t.split(" ").length > 7) continue;
    const st = getComputedStyle(prev);
    const size = px(st.fontSize);
    const upper = isUpper(prev, t, st);
    const track = trackingEm(st);
    if (size <= bodyPx * 0.95 && (upper || track > 0.05 || size < bodyPx * 0.8)) {
      eyebrows.push({ text: short(t, 40), before: h.tagName.toLowerCase(), font_px: Math.round(size), upper, tracking_em: Number(track.toFixed(3)) });
    }
  }
  const tinyCaps = { count: 0, samples: [] };
  const numerals = [];
  const NUMERAL = /^\s*0?\d{1,2}\s*[/.—–-](?!\d)/;
  for (const el of document.body.querySelectorAll("*")) {
    if (SKIP.has(el.tagName) || el.childElementCount > 2 || !visible(el)) continue;
    const t = textOf(el);
    if (!t || t.length > 80) continue;
    const st = getComputedStyle(el);
    // Leaf-ish label only: its own text carries the run, not a descendant's
    const own = [...el.childNodes].some((c) => c.nodeType === 3 && c.nodeValue.trim());
    if (!own) continue;
    if (t.length <= 40 && isUpper(el, t, st) && trackingEm(st) > 0.05 && px(st.fontSize) < bodyPx * 0.8) {
      tinyCaps.count++;
      if (tinyCaps.samples.length < 5) tinyCaps.samples.push({ el: sel(el), text: short(t, 40), font_px: Math.round(px(st.fontSize)) });
    }
    const heading = /^H[1-6]$/.test(el.tagName);
    if ((heading || t.length <= 40) && (NUMERAL.test(t) || (/^0\d$/.test(t) && el.childElementCount === 0))) {
      numerals.push({ el: sel(el), text: short(t, 40) });
    }
  }

  // Effects, buttons and motif hints in one walk over visible elements (pseudo elements included)
  const effects = { gradients: [], scrims: 0, glows: [], backdrop: [], blobs: [] };
  const motif = { stepped_polygons: 0, chamfer_polygons: 0, torn_edges: 0, other_clips: 0, radial_masks: 0, masks: 0, grid_lines: 0, hairline_grids: 0, single_side_borders: 0, edge_lines: 0, corner_marks: 0, side_dots: 0, rules: 0, ruled_starts: 0, arch_tops: 0, round_masks: 0, bezel_frames: 0, letterbox_media: 0, lit_grounds: 0, samples: [] };
  const note = (k, el, extra) => { if (motif.samples.length < 12) motif.samples.push({ hint: k, el: sel(el), ...(extra || {}) }); };
  const polyPoints = (cp) => {
    const at = cp.indexOf("polygon(");
    if (at < 0) return null;
    // Split on commas and spaces at depth 0 only, so calc(100% - 15px) stays one coordinate
    const body = cp.slice(at + 8, cp.lastIndexOf(")"));
    const pts = [];
    let cur = [];
    let tok = "";
    let depth = 0;
    const flush = () => { if (tok.trim()) cur.push(tok.trim()); tok = ""; };
    for (const ch of body) {
      if (ch === "(") depth++;
      if (ch === ")") depth--;
      if (depth === 0 && ch === ",") { flush(); pts.push(cur.slice(-2)); cur = []; }
      else if (depth === 0 && ch === " ") flush();
      else tok += ch;
    }
    flush();
    if (cur.length) pts.push(cur.slice(-2));
    return pts;
  };
  const classifyPolygon = (pts) => {
    let axis = 0;
    let diag = 0;
    for (let i = 0; i < pts.length; i++) {
      const [ax, ay] = pts[i];
      const [bx, by] = pts[(i + 1) % pts.length];
      if (ax === bx || ay === by) axis++;
      else diag++;
    }
    if (pts.length >= 12 && diag >= pts.length * 0.5) return "torn";
    if (pts.length >= 8 && axis >= pts.length * 0.75) return "stepped";
    if (pts.length >= 5 && diag >= 1 && diag <= 4) return "chamfer";
    return "other";
  };
  const isMediaFrame = (el) => /^(IMG|PICTURE|VIDEO|CANVAS|FIGURE)$/.test(el.tagName) || !!el.querySelector(":scope > img, :scope > picture, :scope > video, :scope > canvas");
  const shadowBlobs = (s) => {
    const out = [];
    if (!s || s === "none") return out;
    for (const part of s.split(/,(?![^(]*\))/)) {
      const c = parseColor(part);
      const nums = part.replace(/rgba?\([^)]*\)/, "").match(/-?[\d.]+px/g) || [];
      out.push({ blur: nums.length >= 3 ? px(nums[2]) : 0, color: c });
    }
    return out;
  };
  // Craft: authored interaction states. For each rule naming :hover/:active/:focus-visible, keep the
  // compound that carries the state (".btn" from ".btn:hover .arrow"); a button has the state when
  // it matches one. Cross-origin sheets cannot be read and stay silent.
  const stateSel = { hover: [], active: [], focus: [] };
  const STATE_RE = { hover: /:hover/, active: /:active/, focus: /:focus-visible|:focus(?![-\w])/ };
  const walkRules = (rules) => {
    for (const r of rules || []) {
      if (!r.selectorText) {
        try { if (r.cssRules) walkRules(r.cssRules); } catch {}
        continue;
      }
      for (const part of r.selectorText.split(/,(?![^(]*\))/)) {
        const p = part.trim();
        for (const k of Object.keys(STATE_RE)) {
          const m = STATE_RE[k].exec(p);
          if (!m) continue;
          const subject = p.slice(0, m.index).replace(/:(hover|active|focus-visible|focus-within|focus)(?![-\w])/g, "");
          stateSel[k].push(/^\s*$|[\s>+~]$/.test(subject) ? subject + "*" : subject);
        }
      }
    }
  };
  for (const sh of document.styleSheets) {
    try { walkRules(sh.cssRules); } catch {}
  }
  const hasState = (el, k) => stateSel[k].some((s) => { try { return el.matches(s); } catch { return false; } });
  const GLYPH = /[←-⇿➔-➿⬀-⯿✓✔★☆›»]|[\u{1F300}-\u{1FAFF}]/u;

  const buttons = [];
  const buttonEls = [];
  const all = [...document.body.querySelectorAll("*")].slice(0, 6000);
  for (const el of all) {
    if (SKIP.has(el.tagName) || !visible(el)) continue;
    const st = getComputedStyle(el);
    const r = el.getBoundingClientRect();
    const area = r.width * r.height;
    const bg = st.backgroundImage || "none";
    if (/gradient\(/.test(bg)) {
      if (/repeating-(linear|radial)-gradient/.test(bg) || (/linear-gradient/.test(bg) && /\d+px\s+\d+px|1px/.test(st.backgroundSize))) {
        motif.grid_lines++;
        note("grid-lines", el);
      } else {
        const colors = (bg.match(/rgba?\([^)]*\)/g) || []).map(parseColor);
        const neutral = colors.every((c) => !c || chroma(c) < 24 || c.a < 0.05);
        if (neutral) effects.scrims++;
        else if (effects.gradients.length < 20) effects.gradients.push({ el: sel(el), w: Math.round(r.width), h: Math.round(r.height) });
      }
    }
    for (const s of [...shadowBlobs(st.boxShadow), ...shadowBlobs(st.textShadow)]) {
      if (s.blur >= 16 && s.color && chroma(s.color) >= 40 && s.color.a > 0.15) {
        if (effects.glows.length < 20) effects.glows.push({ el: sel(el), blur_px: s.blur });
        break;
      }
    }
    const bdf = st.backdropFilter || st.webkitBackdropFilter;
    if (bdf && bdf !== "none" && effects.backdrop.length < 20) effects.backdrop.push({ el: sel(el), filter: bdf });
    const blur = /blur\(([\d.]+)px\)/.exec(st.filter || "");
    if (blur && px(blur[1]) >= 20 && area > 10000 && effects.blobs.length < 20) effects.blobs.push({ el: sel(el), blur_px: px(blur[1]) });

    // Buttons: semantic ones plus links styled as buttons
    const tag = el.tagName;
    const cls = typeof el.className === "string" ? el.className : "";
    const semantic = tag === "BUTTON" || el.getAttribute("role") === "button" || /(^|\s)(btn|button)([-_\s]|$)/i.test(cls) || (tag === "INPUT" && /^(submit|button)$/i.test(el.type));
    const bgc = parseColor(st.backgroundColor);
    const peFill = (pe) => {
      const ps = getComputedStyle(el, pe);
      const c = parseColor(ps.backgroundColor);
      return ps.content !== "none" && ps.position === "absolute" && !!c && c.a > 0.5 && px(ps.width) >= r.width * 0.8 && px(ps.height) >= r.height * 0.8;
    };
    const filled = (!!bgc && bgc.a > 0.5) || ((tag === "A" || tag === "BUTTON") && (peFill("::before") || peFill("::after")));
    const outlined = px(st.borderTopWidth) >= 1 && px(st.borderLeftWidth) >= 1;
    const looksLikeButton = tag === "A" && (filled || outlined) && px(st.paddingLeft) >= 12 && r.height >= 28 && r.height <= 90 && textOf(el).length <= 40;
    // Skip links, tabs and off-screen controls are not page buttons
    const offscreen = r.bottom + sy <= 0 || r.right <= 0 || r.left >= vw;
    const notButton = offscreen || /skip/i.test(cls) || /^skip to/i.test(textOf(el)) || /^(tab|menuitem|option)$/.test(el.getAttribute("role") || "");
    if ((semantic || looksLikeButton) && !notButton && r.width >= 40 && r.height >= 24 && r.height <= 120 && textOf(el).length > 0) {
      const corner = (v) => (/%/.test(v) ? (px(v) / 100) * Math.min(r.width, r.height) : px(v));
      const radius = Math.max(corner(st.borderTopLeftRadius), corner(st.borderTopRightRadius), corner(st.borderBottomLeftRadius), corner(st.borderBottomRightRadius));
      const before = getComputedStyle(el, "::before");
      const after = getComputedStyle(el, "::after");
      const clip = [st.clipPath, before.clipPath, after.clipPath].find((c) => c && c !== "none") || null;
      const mask = [st.maskImage || st.webkitMaskImage, before.maskImage || before.webkitMaskImage, after.maskImage || after.webkitMaskImage].find((c) => c && c !== "none") || null;
      // Primary: a filled link, submit or .btn (media toggles and menu buttons stay out)
      const primary = filled && (tag === "A" || (tag === "INPUT") || el.type === "submit" || /(^|\s)(btn|button|cta)([-_\s]|$)/i.test(cls));
      // Craft: proportions, label, icon kind, a lone accent side border, authored states
      const bw = (s) => px(st["border" + s + "Width"]);
      const edgeBar = [before, after].some((ps) => {
        const c = parseColor(ps.backgroundColor);
        return ps.content !== "none" && ps.position === "absolute" && !!c && c.a > 0.5 && px(ps.width) >= 2 && px(ps.width) <= 8 && px(ps.height) >= r.height * 0.6;
      });
      const insetEdge = /inset/.test(st.boxShadow || "") && /(^|[\s,(])-?[2-8](\.\d+)?px\s+0px\s+0px/.test(st.boxShadow);
      const sideBorder = ((bw("Left") >= 2 || bw("Right") >= 2) && Math.abs(bw("Left") - bw("Right")) >= 1.5 && bw("Top") < 1.5) || edgeBar || insetEdge;
      const icon = el.querySelector("svg, img, i[class*=icon], span[class*=icon]") ? "svg" : GLYPH.test(textOf(el)) ? "glyph" : null;
      const states = { hover: hasState(el, "hover"), active: hasState(el, "active"), focus_visible: hasState(el, "focus") };
      if (buttons.length < 40) buttonEls.push(el);
      if (buttons.length < 40) buttons.push({ el: sel(el), text: short(textOf(el), 30), radius: Math.round(radius * 10) / 10, height: Math.round(r.height), width: Math.round(r.width), clip, mask, filled, primary, padding_x: Math.round(px(st.paddingLeft)), padding_y: Math.round(px(st.paddingTop)), font_px: Math.round(px(st.fontSize)), font_weight: parseInt(st.fontWeight, 10) || 400, side_border: sideBorder, icon, states });
    }

    // Motif hints
    const cp = st.clipPath || "none";
    if (cp !== "none" && area >= 1600) {
      const pts = polyPoints(cp);
      if (pts) {
        const kind = classifyPolygon(pts);
        if (kind === "torn" && r.width >= vw * 0.5) { motif.torn_edges++; note("torn-edge", el); }
        else if (kind === "stepped") { motif.stepped_polygons++; note("stepped-polygon", el, { points: pts.length }); }
        else if (kind === "chamfer") { motif.chamfer_polygons++; note("chamfer-polygon", el, { points: pts.length }); }
        else motif.other_clips++;
      } else if (/circle\(|ellipse\(/.test(cp) && isMediaFrame(el)) { motif.round_masks++; note("round-mask", el); }
      else motif.other_clips++;
    }
    const mk = st.maskImage || st.webkitMaskImage;
    if (mk && mk !== "none") {
      motif.masks++;
      if (/radial-gradient/.test(mk)) { motif.radial_masks++; note("radial-mask", el); }
      else if (r.width >= vw * 0.5 && /url\(/.test(mk)) { motif.torn_edges++; note("torn-mask", el); }
    }
    const bw = [px(st.borderTopWidth), px(st.borderRightWidth), px(st.borderBottomWidth), px(st.borderLeftWidth)];
    const sides = bw.filter((w) => w > 0).length;
    if (sides === 1 && Math.max(...bw) >= 2 && (isMediaFrame(el) || el.querySelector("img, picture, video, canvas"))) { motif.single_side_borders++; note("single-side-border", el); }
    if (sides === 1 && bw[0] > 0 && bw[0] <= 2 && r.width >= vw * 0.4 && !(isMediaFrame(el))) motif.rules++;
    if (tag === "HR" && r.width >= vw * 0.4) motif.rules++;
    const rTL = corner2(st.borderTopLeftRadius, r);
    const rTR = corner2(st.borderTopRightRadius, r);
    const rB = Math.max(corner2(st.borderBottomLeftRadius, r), corner2(st.borderBottomRightRadius, r));
    if (r.width >= 120 && r.height >= 120 && Math.min(rTL, rTR) >= Math.min(r.width * 0.35, 100) && rB <= 12) { motif.arch_tops++; note("arch-top", el); }
    if (r.width >= 120 && Math.abs(r.width - r.height) <= 4 && Math.min(rTL, rTR) >= r.width * 0.45 && rB >= r.width * 0.45 && isMediaFrame(el)) { motif.round_masks++; note("round-mask", el); }
    if (st.display.includes("grid") && (px(st.columnGap) >= 1 && px(st.columnGap) <= 2) && bgc && bgc.a > 0.5 && el.childElementCount >= 3) { motif.hairline_grids++; note("hairline-grid", el); }
    // Bezel: a frame holding media with an even border or padding and one rounded radius
    const pads = [px(st.paddingTop), px(st.paddingRight), px(st.paddingBottom), px(st.paddingLeft)];
    const evenBorder = sides === 4 && Math.max(...bw) - Math.min(...bw) <= 0.5 && bw[0] >= 1;
    const evenPad = Math.min(...pads) >= 4 && Math.max(...pads) - Math.min(...pads) <= 2 && Math.max(...pads) <= 32 && bgc && bgc.a > 0.3;
    if (r.width >= 160 && Math.min(rTL, rTR, rB) >= 4 && (evenBorder || evenPad) && el.querySelector(":scope > img, :scope > picture, :scope > video, :scope > canvas")) { motif.bezel_frames++; note("bezel-frame", el); }
    // Letterbox: a photograph or film cropped wide (about 2.39:1)
    if (/^(IMG|VIDEO|PICTURE|CANVAS)$/.test(tag) && r.width >= vw * 0.4 && r.height > 0 && r.width / r.height >= 2.15 && r.width / r.height <= 2.7) { motif.letterbox_media++; note("letterbox", el); }
    // Lit ground: a wide band lit by one soft gradient falloff
    if (r.width >= vw * 0.8 && r.height >= 240 && /radial-gradient|linear-gradient/.test(st.backgroundImage || "") && !/repeating-/.test(st.backgroundImage || "")) { motif.lit_grounds++; note("lit-ground", el); }
    for (const pe of ["::before", "::after"]) {
      const ps = getComputedStyle(el, pe);
      if (!ps.content || ps.content === "none" || ps.display === "none") continue;
      const w = px(ps.width);
      const h = px(ps.height);
      const pb = [px(ps.borderTopWidth), px(ps.borderRightWidth), px(ps.borderBottomWidth), px(ps.borderLeftWidth)];
      const pfill = parseColor(ps.backgroundColor);
      const filledPe = !!pfill && pfill.a > 0.3;
      if (ps.position === "absolute" && w > 0 && w <= 40 && h > 0 && h <= 40) {
        const lShape = (pb[0] > 0 && pb[3] > 0 && !pb[1] && !pb[2]) || (pb[0] > 0 && pb[1] > 0 && !pb[2] && !pb[3]) || (pb[2] > 0 && pb[3] > 0 && !pb[0] && !pb[1]) || (pb[1] > 0 && pb[2] > 0 && !pb[0] && !pb[3]);
        if (lShape) { motif.corner_marks++; note("corner-mark", el); }
        if (/50%|9999|999px/.test(ps.borderTopLeftRadius) || px(ps.borderTopLeftRadius) >= w / 2) {
          const top = px(ps.top);
          if ((ps.left !== "auto" && px(ps.left) < 0) || (ps.right !== "auto" && px(ps.right) < 0) || (top > 0 && Math.abs(top + h / 2 - r.height / 2) < 6)) { motif.side_dots++; note("side-notch", el); }
        }
      }
      if (filledPe && ((w > 0 && w <= 4 && h >= 30) || (h > 0 && h <= 4 && w >= 30 && w >= r.width * 0.5)) && isMediaFrame(el)) { motif.edge_lines++; note("edge-line", el); }
      if (filledPe && h >= 2 && h <= 8 && w >= 16 && w <= 160 && r.width >= vw * 0.4) motif.ruled_starts++;
    }
  }
  function corner2(v, r) {
    return /%/.test(v) ? (px(v) / 100) * Math.min(r.width, r.height) : px(v);
  }

  // Rows of three or more same-size siblings: icon cards (small icon + short text) and identical cards
  const iconCardRows = [];
  const identicalRows = [];
  for (const parent of all) {
    if (parent.childElementCount < 3 || !visible(parent) || parent.closest("footer, nav, header, [role=contentinfo]")) continue;
    const kids = [...parent.children].filter((k) => visible(k));
    if (kids.length < 3) continue;
    const groups = new Map();
    for (const k of kids) {
      const r = k.getBoundingClientRect();
      if (r.width < 120 || r.height < 80) continue;
      const key = Math.round(r.width / 3) + "x" + Math.round(r.height / 3);
      if (!groups.has(key)) groups.set(key, []);
      groups.get(key).push(k);
    }
    for (const g of groups.values()) {
      if (g.length < 3) continue;
      // Same row only: equal tops
      const tops = new Map();
      for (const k of g) {
        const t = Math.round(k.getBoundingClientRect().top / 4);
        tops.set(t, (tops.get(t) || 0) + 1);
      }
      if (Math.max(...tops.values()) < 3) continue;
      const withIcon = g.filter((k) => {
        const icon = [...k.querySelectorAll("svg, img, i, [class*='icon']")].some((ic) => {
          const ir = ic.getBoundingClientRect();
          return ir.width >= 12 && ir.width <= 72 && ir.height <= 72;
        });
        return icon && textOf(k).length <= 260;
      });
      const r0 = g[0].getBoundingClientRect();
      const row = { el: sel(parent), count: g.length, w: Math.round(r0.width), h: Math.round(r0.height) };
      if (withIcon.length >= 3) iconCardRows.push(row);
      else if (g.every((k) => textOf(k).length >= 20) && r0.height >= 120 && r0.width >= 160) identicalRows.push(row);
    }
  }

  // h1 count and rendered lines
  const h1s = [...document.querySelectorAll("h1")].filter(visible);
  const h1Lines = h1s.map((h) => {
    const range = document.createRange();
    range.selectNodeContents(h);
    const st = getComputedStyle(h);
    const lh = st.lineHeight === "normal" ? px(st.fontSize) * 1.2 : px(st.lineHeight);
    const tops = [];
    for (const rr of range.getClientRects()) {
      if (rr.width < 1 || rr.height < 1) continue;
      if (!tops.some((t) => Math.abs(t - rr.top) < lh * 0.5)) tops.push(rr.top);
    }
    return tops.length || Math.round(h.getBoundingClientRect().height / Math.max(1, lh));
  });

  // Top-level sections: main's children (descending through single wrappers), else body's
  let root = document.querySelector("main") || document.body;
  while (root.childElementCount === 1 && root.firstElementChild) root = root.firstElementChild;
  const sections = [...root.children].filter((s) => visible(s) && !/^(HEADER|NAV|FOOTER|SCRIPT|STYLE)$/.test(s.tagName) && s.getBoundingClientRect().height > 40);
  const CONTENT = "h1,h2,h3,h4,h5,h6,p,li,img,picture,video,canvas,svg,figure,table,blockquote,button,a,label,input,select,dt,dd,span";
  const bounds = sections.map((s) => {
    let top = Infinity;
    let bottom = -Infinity;
    for (const c of [s, ...s.querySelectorAll(CONTENT)]) {
      if (c === s && !s.matches(CONTENT)) continue;
      if (!visible(c)) continue;
      const r = docRect(c);
      if (r.height < 2) continue;
      top = Math.min(top, r.top);
      bottom = Math.max(bottom, r.bottom);
    }
    return Number.isFinite(top) ? { top, bottom } : null;
  });
  const sectionGaps = [];
  for (let i = 0; i + 1 < bounds.length; i++) {
    if (bounds[i] && bounds[i + 1]) sectionGaps.push(Math.round(bounds[i + 1].top - bounds[i].bottom));
  }

  // Hero and statement paragraphs: word counts
  const words = (t) => (t ? t.split(/\s+/).filter(Boolean).length : 0);
  const heroSection = h1s[0] ? sections.find((s) => s.contains(h1s[0])) || h1s[0].parentElement : null;
  let heroWords = 0;
  let heroSample = "";
  if (heroSection) {
    for (const p of heroSection.querySelectorAll("p")) {
      if (!visible(p)) continue;
      const n = words(textOf(p));
      if (n > heroWords) { heroWords = n; heroSample = short(textOf(p), 80); }
    }
  }
  let statementWords = 0;
  let statementSample = "";
  for (const p of document.querySelectorAll("p, blockquote")) {
    if (!visible(p) || px(getComputedStyle(p).fontSize) < bodyPx * 1.25) continue;
    const n = words(textOf(p));
    if (n > statementWords) { statementWords = n; statementSample = short(textOf(p), 80); }
  }

  // Real pictures: raster img/picture and non-svg background images at least 200px wide
  const rasterSrcs = new Set();
  for (const img of document.querySelectorAll("img")) {
    if (!visible(img)) continue;
    const src = img.currentSrc || img.src || "";
    if (!src || /\.svg(\?|#|$)|^data:image\/svg/i.test(src)) continue;
    if (img.getBoundingClientRect().width >= 200) rasterSrcs.add(src);
  }
  for (const el of all) {
    if (!visible(el)) continue;
    const bg = getComputedStyle(el).backgroundImage;
    if (!bg || !/url\(/.test(bg)) continue;
    for (const m of bg.matchAll(/url\(["']?([^"')]+)["']?\)/g)) {
      if (/\.svg(\?|#|$)|^data:image\/svg/i.test(m[1])) continue;
      if (el.getBoundingClientRect().width >= 200) rasterSrcs.add(m[1]);
    }
  }

  // Craft: the header row(s) and height, utility bars above it included
  const vh = window.innerHeight;
  let headerFacts = null;
  const hdr = [...document.querySelectorAll("header, [role=banner]")].find((h) => visible(h) && docRect(h).top < 120 && !h.closest("main article, section"))
    || [...document.querySelectorAll("nav")].find((n) => visible(n) && docRect(n).top < 120);
  if (hdr) {
    const hr = docRect(hdr);
    const parts = [hdr];
    // Bars stacked above the header (siblings of it or of its wrappers) belong to the same chrome
    for (let n = hdr; n && n !== document.body; n = n.parentElement) {
      for (let s = n.previousElementSibling; s; s = s.previousElementSibling) {
        if (!visible(s) || SKIP.has(s.tagName)) continue;
        const r = docRect(s);
        const sp = getComputedStyle(s).position;
        if (sp === "absolute" || sp === "fixed" || /skip/i.test(typeof s.className === "string" ? s.className : "") || /^skip to/i.test(textOf(s))) continue;
        if (r.height > 0 && r.height <= 80 && r.top >= 0 && r.bottom <= hr.top + 2 && r.top < 120 && textOf(s)) parts.push(s);
      }
    }
    const bands = [];
    for (const p of parts) {
      for (const el of [p, ...p.querySelectorAll("a, button, img, svg, input, select, span, p, li, strong, small")]) {
        if (!visible(el) || (el.tagName !== "IMG" && el.tagName !== "svg" && el.tagName !== "INPUT" && el.querySelector("a, button, img, svg, span, p, li"))) continue;
        if (el === p && el.childElementCount) continue;
        const r = docRect(el);
        if (r.height < 4 || r.height > 160 || r.top > hr.bottom + 2 || r.left >= vw || r.right <= 0) continue;
        bands.push([r.top, r.bottom, sel(el)]);
      }
    }
    bands.sort((a, b) => a[0] - b[0]);
    // Rows: bands of content that do not overlap vertically (a 4px touch still counts as one row)
    let rows = 0;
    let end = -Infinity;
    const rowEls = [];
    for (const [t, b, name] of bands) {
      if (t >= end - 4) { rows++; end = b; rowEls.push(name + "@" + Math.round(t)); } else end = Math.max(end, b);
    }
    const top = Math.min(...parts.map((p) => docRect(p).top));
    const pos = getComputedStyle(hdr).position;
    headerFacts = { el: sel(hdr), height: Math.round(hr.bottom - Math.max(0, top)), rows, row_starts: rowEls.slice(0, 6), utility_bar: parts.length > 1, sticky: pos === "sticky" || pos === "fixed" };
  }

  // Craft: distinct type sizes (px with the characters they carry) and the h1's own style
  const typeSizes = [...sizeChars.entries()].sort((a, b) => b[0] - a[0]).map(([size, chars]) => ({ px: size, chars }));
  let h1Style = null;
  if (h1s[0]) {
    const st = getComputedStyle(h1s[0]);
    const fs = px(st.fontSize);
    h1Style = { font_px: Math.round(fs), tracking_em: Number(trackingEm(st).toFixed(3)), weight: parseInt(st.fontWeight, 10) || 400 };
  }

  // Craft: the longest prose line, in characters (paragraphs of three lines or more)
  const measure = { max_chars: 0, el: null, sample: "", paragraphs: 0 };
  for (const el of document.querySelectorAll("p, li, blockquote, dd, figcaption")) {
    if (!visible(el) || el.querySelector("p, li, div, ul, ol")) continue;
    const t = textOf(el);
    if (t.length < 140) continue;
    const st = getComputedStyle(el);
    const fs = px(st.fontSize);
    const lh = st.lineHeight === "normal" ? fs * 1.2 : px(st.lineHeight);
    const h = el.getBoundingClientRect().height - px(st.paddingTop) - px(st.paddingBottom);
    const lines = Math.round(h / Math.max(1, lh));
    if (lines < 3) continue;
    measure.paragraphs++;
    const cpl = Math.round(t.length / lines);
    if (cpl > measure.max_chars) { measure.max_chars = cpl; measure.el = sel(el); measure.sample = short(t, 50); }
  }

  // Craft: headings per section head (h1/h2 grouped by their nearest sectioning box)
  const owners = new Map();
  for (const h of document.querySelectorAll("h1, h2")) {
    if (!visible(h) || h.closest("footer, nav, aside, dialog, [role=contentinfo]")) continue;
    const owner = h.closest("section, article, header") || sections.find((s) => s.contains(h)) || document.body;
    if (!owners.has(owner)) owners.set(owner, []);
    owners.get(owner).push(short(textOf(h), 40));
  }
  const multiHeads = [...owners.entries()].filter(([, hs]) => hs.length > 1).map(([o, hs]) => ({ el: sel(o), count: hs.length, texts: hs.slice(0, 3) }));

  // Craft: the first real picture (raster img, video, canvas or photo background at half the
  // width or more) and what is drawn over the hero's largest picture
  const media = [];
  for (const el of all) {
    if (!visible(el)) continue;
    const r = docRect(el);
    if (r.width < Math.min(vw * 0.45, 360) || r.height < 120) continue;
    let ok = false;
    if (el.tagName === "VIDEO" || el.tagName === "CANVAS") ok = true;
    else if (el.tagName === "IMG") ok = !/\.svg(\?|#|$)|^data:image\/svg/i.test(el.currentSrc || el.src || "");
    else {
      const bg = getComputedStyle(el).backgroundImage;
      ok = /url\(/.test(bg || "") && !/\.svg/i.test(bg);
    }
    if (ok) media.push({ el, r });
  }
  const firstMedia = media.slice().sort((a, b) => a.r.top - b.r.top)[0] || null;
  const heroMedia = media.filter((m) => m.r.top < vh).sort((a, b) => b.r.width * b.r.height - a.r.width * a.r.height)[0] || null;
  const overlays = [];
  if (heroMedia) {
    const m = heroMedia.r;
    const mArea = m.width * m.height;
    for (const el of all) {
      if (el === heroMedia.el || el.contains(heroMedia.el) || heroMedia.el.contains(el) || !visible(el)) continue;
      if (/^(IMG|VIDEO|CANVAS|PICTURE|SOURCE)$/.test(el.tagName) || el.closest("svg") !== null && el.tagName !== "svg") continue;
      const st = getComputedStyle(el);
      if (st.position !== "absolute" && st.position !== "fixed") continue;
      if (textOf(el)) continue;
      const r = docRect(el);
      const drawing = el.tagName === "svg" && el.querySelectorAll("line, path, polyline, rect, circle").length >= 2;
      if ((r.width * r.height >= mArea * 0.25 && !drawing) || r.width * r.height < 4) continue;
      if (r.right <= m.left || r.left >= m.right || r.bottom <= m.top || r.top >= m.bottom) continue;
      const bgc = parseColor(st.backgroundColor);
      const pseudoPaint = ["::before", "::after"].some((pe) => { const ps = getComputedStyle(el, pe); const c = parseColor(ps.backgroundColor); return ps.content !== "none" && ((c && c.a > 0.1) || px(ps.borderTopWidth) > 0); });
      const painted = el.tagName === "svg" || pseudoPaint || (bgc && bgc.a > 0.1) || px(st.borderTopWidth) + px(st.borderLeftWidth) + px(st.borderRightWidth) + px(st.borderBottomWidth) > 0 || st.backgroundImage !== "none";
      if (painted && overlays.length < 20) overlays.push({ el: sel(el), w: Math.round(r.width), h: Math.round(r.height) });
    }
    // A drawn grid (repeating gradient lines) on the picture's own frame counts as an overlay too
    let fr = heroMedia.el.parentElement;
    for (let k = 0; fr && k < 3; k++, fr = fr.parentElement) {
      const bs = getComputedStyle(fr);
      if (/gradient/.test(bs.backgroundImage) && /\d+px/.test(bs.backgroundSize) && px(bs.backgroundSize) > 0 && px(bs.backgroundSize) <= 80) { overlays.push({ el: sel(fr), w: 0, h: 0, grid: true }); break; }
    }
  }

  // Craft: text blocks in the hero's first viewport
  let heroBlocks = null;
  if (heroSection) {
    heroBlocks = { count: 0, samples: [] };
    for (const el of [heroSection, ...heroSection.querySelectorAll("*")]) {
      if (!visible(el) || el.closest("nav")) continue;
      const own = [...el.childNodes].some((n) => n.nodeType === 3 && /[\p{L}\p{N}]/u.test(n.nodeValue));
      if (!own) continue;
      // An inline run inside a block counts with its block
      const disp = getComputedStyle(el).display;
      if (/^inline/.test(disp) && el.tagName !== "A" && el.tagName !== "BUTTON" && el.parentElement && heroSection.contains(el.parentElement) && el.parentElement !== heroSection) continue;
      if (docRect(el).top >= vh) continue;
      heroBlocks.count++;
      if (heroBlocks.samples.length < 30) heroBlocks.samples.push(short(textOf(el), 30));
    }
  }

  // Craft: the hero's action pair (first primary in the hero and the next action beside it)
  let heroPair = null;
  if (heroSection) {
    const i = buttons.findIndex((b, k) => b.primary && heroSection.contains(buttonEls[k]) && docRect(buttonEls[k]).top < vh);
    if (i >= 0) {
      const pe = buttonEls[i];
      const pr = docRect(pe);
      const j = buttons.findIndex((b, k) => k !== i && heroSection.contains(buttonEls[k]) && !buttonEls[k].contains(pe) && Math.abs(docRect(buttonEls[k]).top - pr.top) < 120 && pe.parentElement && (pe.parentElement.contains(buttonEls[k]) || (pe.parentElement.parentElement || pe).contains(buttonEls[k])));
      heroPair = { primary: { text: buttons[i].text, height: buttons[i].height, padding_x: buttons[i].padding_x }, secondary: j >= 0 ? { text: buttons[j].text, height: buttons[j].height, padding_x: buttons[j].padding_x, filled: buttons[j].filled } : null };
    }
  }

  // Craft: Unicode glyphs or emoji standing in for icons in links and buttons
  const glyphs = { count: 0, samples: [] };
  for (const el of document.querySelectorAll("a, button, [role=button]")) {
    if (!visible(el)) continue;
    const t = textOf(el);
    if (!GLYPH.test(t)) continue;
    glyphs.count++;
    if (glyphs.samples.length < 5) glyphs.samples.push({ el: sel(el), text: short(t, 30) });
  }

  // Live interaction and its no-JS fallback
  const controls = [];
  const addCtl = (kind, el) => { if (visible(el) && controls.length < 10) controls.push({ kind, el: sel(el) }); };
  document.querySelectorAll("input[type=range]").forEach((e) => addCtl("range", e));
  document.querySelectorAll("select").forEach((e) => addCtl("select", e));
  document.querySelectorAll("[role=tablist], [role=tab]").forEach((e) => addCtl("tabs", e));
  document.querySelectorAll("button[aria-controls], [role=button][aria-controls]").forEach((e) => addCtl("aria-controls", e));
  document.querySelectorAll("input[type=radio], input[type=checkbox], input[type=number]").forEach((e) => addCtl("input", e));
  const interaction = { controls, table: document.querySelectorAll("table").length > 0, noscript: document.querySelectorAll("noscript").length > 0 };

  // Footer: the largest text element and its width against the viewport
  const footer = document.querySelector("footer, [role=contentinfo]") || (root.lastElementChild || null);
  let footerFacts = null;
  if (footer && visible(footer)) {
    const runs = [];
    for (const el of [footer, ...footer.querySelectorAll("*")]) {
      if (!visible(el) || SKIP.has(el.tagName)) continue;
      const own = [...el.childNodes].some((c) => c.nodeType === 3 && c.nodeValue.trim());
      if (own) runs.push({ el, size: px(getComputedStyle(el).fontSize) });
    }
    const max = Math.max(0, ...runs.map((x) => x.size));
    // The brand line may be split into spans: measure every run set at (about) the largest size
    const big = runs.filter((x) => x.size >= max * 0.9);
    if (big.length) {
      let left = Infinity;
      let right = -Infinity;
      for (const x of big) {
        const range = document.createRange();
        range.selectNodeContents(x.el);
        for (const rr of range.getClientRects()) {
          if (rr.width < 1) continue;
          left = Math.min(left, rr.left);
          right = Math.max(right, rr.right);
        }
      }
      const w = Number.isFinite(left) ? right - left : 0;
      footerFacts = { el: sel(big[0].el), text: short(big.map((x) => textOf(x.el)).join(" "), 40), font_px: Math.round(max), width_px: Math.round(w), width_ratio: Number((w / vw).toFixed(3)) };
    }
  }

  // Hero ground: the first opaque background behind the first h1, and the h1's own text colour
  // (light text on a photo or video hero reads as a dark hero even with a transparent box)
  const lum = (c) => {
    const ch = (v) => { v /= 255; return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4); };
    return Number((0.2126 * ch(c.r) + 0.7152 * ch(c.g) + 0.0722 * ch(c.b)).toFixed(3));
  };
  let heroGround = null;
  const heroH1 = document.querySelector("h1");
  if (heroH1) {
    let bg = null;
    let bgEl = null;
    for (let el = heroH1; el && el.nodeType === 1; el = el.parentElement) {
      const cs = getComputedStyle(el);
      // A gradient or picture ground has no single colour: leave it to the h1's text colour
      if (cs.backgroundImage && cs.backgroundImage !== "none") { bgEl = el; break; }
      const c = parseColor(cs.backgroundColor);
      if (c && c.a >= 0.5) { bg = c; bgEl = el; break; }
    }
    const text = parseColor(getComputedStyle(heroH1).color);
    heroGround = {
      el: bgEl ? sel(bgEl) : null,
      bg_luminance: bg ? lum(bg) : null,
      h1_text_luminance: text ? lum(text) : null,
    };
  }

  return {
    width: vw,
    body_px: bodyPx,
    fonts,
    mono,
    eyebrows: eyebrows.slice(0, 20),
    tiny_caps: tinyCaps,
    numerals: numerals.slice(0, 20),
    effects,
    icon_card_rows: iconCardRows.slice(0, 10),
    identical_card_rows: identicalRows.slice(0, 10),
    h1: { count: h1s.length, lines: h1Lines },
    statement: { hero_words: heroWords, hero_sample: heroSample, max_words: statementWords, sample: statementSample },
    section_gaps: sectionGaps,
    images: { raster: rasterSrcs.size },
    interaction,
    footer: footerFacts,
    buttons,
    hero_ground: heroGround,
    motif,
    viewport_height: vh,
    header: headerFacts,
    type_sizes: typeSizes,
    h1_style: h1Style,
    measure,
    multi_heading_sections: multiHeads,
    first_media: firstMedia ? { el: sel(firstMedia.el), top: Math.round(firstMedia.r.top), height: Math.round(firstMedia.r.height) } : null,
    hero_overlays: heroMedia ? { media: sel(heroMedia.el), items: overlays } : null,
    hero_blocks: heroBlocks,
    glyph_icons: glyphs,
    hero_pair: heroPair,
  };
}

const FACTS_SCRIPT = "(" + layozaRenderFacts.toString() + ")()";

function parseArgs(argv) {
  const args = { url: null, file: null, out: null };
  for (let i = 2; i < argv.length; i++) {
    const k = argv[i];
    if (k === "--url" && argv[i + 1]) args.url = argv[++i];
    else if (k === "--file" && argv[i + 1]) args.file = argv[++i];
    else if (k === "--out" && argv[i + 1]) args.out = argv[++i];
    else if (k.startsWith("--")) console.error(`Unknown flag ignored: ${k}`);
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

async function collectWithNode(pw, url) {
  // Let file:// pages expose their own stylesheets to cssRules (unique-origin otherwise).
  const browser = await pw.chromium.launch({ headless: true, args: ["--allow-file-access-from-files"] });
  try {
    return await Promise.all(
      WIDTHS.map(async (vp) => {
        const ctx = await browser.newContext({ viewport: { width: vp.width, height: vp.height } });
        try {
          const page = await ctx.newPage();
          await page.goto(url, { waitUntil: "load", timeout: 30000 });
          await page.waitForLoadState("networkidle", { timeout: 3000 }).catch(() => {});
          await page.evaluate("(" + FONTS_READY + ")(3000)").catch(() => {});
          await page.evaluate(SCROLL_THROUGH);
          return { label: vp.label, ...(await page.evaluate(FACTS_SCRIPT)) };
        } finally {
          await ctx.close();
        }
      })
    );
  } finally {
    await browser.close();
  }
}

/** Same collection through Python Playwright (falls back to installed Chrome/Edge). */
function collectWithPython(url) {
  const pyCode = `
import json, sys, asyncio, site
# Run isolated (-I) yet still find a per-user Playwright install
sys.path.append(site.getusersitepackages())
from playwright.async_api import async_playwright

url, raw_out, cfg_path = sys.argv[1:4]
cfg = json.load(open(cfg_path, encoding="utf-8"))

async def one(browser, vp):
    ctx = await browser.new_context(viewport={"width": vp["width"], "height": vp["height"]})
    try:
        page = await ctx.new_page()
        await page.goto(url, wait_until="load", timeout=30000)
        try:
            await page.wait_for_load_state("networkidle", timeout=3000)
        except Exception:
            pass
        try:
            await page.evaluate("(" + cfg["fonts_ready"] + ")(3000)")
        except Exception:
            pass
        await page.evaluate(cfg["scroll"])
        facts = await page.evaluate(cfg["facts"])
        return {"label": vp["label"], **facts}
    finally:
        await ctx.close()

async def main():
    async with async_playwright() as p:
        browser = None
        errors = []
        for kwargs in [{}, {"channel": "chrome"}, {"channel": "msedge"}]:
            try:
                browser = await p.chromium.launch(headless=True, args=["--allow-file-access-from-files"], **kwargs)
                break
            except Exception as e:
                errors.append(str(e).splitlines()[0] if str(e) else repr(e))
        if browser is None:
            raise RuntimeError("Could not launch headless Chromium, Chrome, or Edge: " + " | ".join(errors))
        try:
            res = await asyncio.gather(*[one(browser, vp) for vp in cfg["widths"]])
        finally:
            await browser.close()
    with open(raw_out, "w", encoding="utf-8") as f:
        json.dump(list(res), f)

asyncio.run(main())
`;
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "layoza-facts-"));
  const cfgPath = path.join(tmp, "cfg.json");
  const rawPath = path.join(tmp, "raw.json");
  fs.writeFileSync(cfgPath, JSON.stringify({ widths: WIDTHS, fonts_ready: FONTS_READY, scroll: SCROLL_THROUGH, facts: FACTS_SCRIPT }), "utf8");
  try {
    const res = spawnSync("python", ["-I", "-X", "utf8", "-c", pyCode, url, rawPath, cfgPath], { stdio: "inherit", env: { ...process.env, PYTHONIOENCODING: "utf-8", PYTHONUTF8: "1" } });
    if (res.error || res.status !== 0) {
      throw new Error(
        "Python Playwright fallback failed. Install Node Playwright (`npm i -D playwright && npx playwright install chromium`) " +
          "or Python Playwright (`pip install playwright && playwright install chromium`)."
      );
    }
    return JSON.parse(fs.readFileSync(rawPath, "utf8"));
  } finally {
    fs.rmSync(tmp, { recursive: true, force: true });
  }
}

async function main() {
  const args = parseArgs(process.argv);
  if (!args.url || !args.out) {
    console.error("Usage: node scripts/render-facts.mjs (--url <http-url> | --file <html-path>) --out <json-path>");
    process.exit(1);
  }
  const outPath = path.resolve(args.out);
  fs.mkdirSync(path.dirname(outPath), { recursive: true });
  fs.rmSync(outPath, { force: true });
  // Same Playwright resolution as browser-qa.mjs: LAYOZA_PLAYWRIGHT_DIR, the project, then a bare import
  let pw = null;
  const bases = [process.env.LAYOZA_PLAYWRIGHT_DIR, process.cwd()].filter(Boolean);
  for (const base of bases) {
    try {
      const req = createRequire(path.join(path.resolve(base), "noop.js"));
      const mod = await import(pathToFileURL(req.resolve("playwright")).href);
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
  const engine = pw ? "node-playwright" : "python-playwright";
  const viewports = pw ? await collectWithNode(pw, args.url) : collectWithPython(args.url);
  fs.writeFileSync(outPath, JSON.stringify({ url: args.url, engine, viewports }, null, 2), "utf8");
  console.log(`Layoza render facts written to ${outPath} (${engine}, ${viewports.length} widths)`);
}

main().catch((err) => {
  console.error(err && err.stack ? err.stack : err);
  process.exit(1);
});
