// Original Layoza composition probe: full-page captures at three widths plus measured
// type scale, viewport fill, section rhythm and media reach. Evidence for review, not a score.
// Full-page captures and measurements run with reduced motion: captureBeyondViewport resizes the
// render surface, which can release pins and replay scroll scenes. Normal-motion first-screen,
// middle and end frames are captured separately at the real viewport size, with no resize.
import fs from 'node:fs';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
import {Browser, sleep} from './lib/cdp.mjs';

if (Number(process.versions.node.split('.')[0]) < 22) { console.error('Composition review requires Node 22+; the base Layoza runtime remains available.'); process.exit(2); }
const target = process.argv[2], out = path.resolve(process.argv[3] || '.layoza/composition-review');
if (!target) { console.error('Usage: node scripts/review-composition.mjs <url-or-html> [output-directory]'); process.exit(2); }
const url = /^(https?|file):\/\//.test(target) ? target : pathToFileURL(path.resolve(target)).href;
fs.mkdirSync(out, {recursive: true});

// Runs in the page. Measures what a visitor sees; no style is judged here.
function measure() {
  const vw = innerWidth, vh = innerHeight;
  const visible = el => { const s = getComputedStyle(el), r = el.getBoundingClientRect(); return s.visibility !== 'hidden' && s.display !== 'none' && +s.opacity > 0.05 && r.width > 0 && r.height > 0; };
  const textEls = [...document.querySelectorAll('h1,h2,h3,h4,p,li,a,span,blockquote,figcaption,label,button,dt,dd,td')]
    .filter(el => visible(el) && [...el.childNodes].some(n => n.nodeType === 3 && n.textContent.trim()));
  // Whole-pixel buckets, flagged above 10 sizes: keep in sync with browser-qa.mjs font-size-sprawl.
  const sizes = textEls.map(el => Math.round(parseFloat(getComputedStyle(el).fontSize)));
  const median = list => { const s = [...list].sort((a, b) => a - b); return s.length ? s[Math.floor(s.length / 2)] : 0; };
  const h1 = document.querySelector('h1');
  const h1Size = h1 ? parseFloat(getComputedStyle(h1).fontSize) : 0;
  const bodySize = median([...document.querySelectorAll('p')].filter(visible).map(p => parseFloat(getComputedStyle(p).fontSize))) || 16;
  // First-screen fill: share of a coarse grid covered by text, media or a non-page background.
  const pageBg = getComputedStyle(document.body).backgroundColor;
  let filled = 0, cells = 0;
  for (let y = 10; y < vh; y += 40) for (let x = 10; x < vw; x += 40) {
    cells++;
    let el = document.elementFromPoint(x, y), hit = false;
    while (el && el !== document.body && el !== document.documentElement && !hit) {
      const s = getComputedStyle(el);
      hit = /^(IMG|VIDEO|CANVAS|SVG|PICTURE)$/i.test(el.tagName) || s.backgroundImage !== 'none'
        || (s.backgroundColor !== 'rgba(0, 0, 0, 0)' && s.backgroundColor !== pageBg && el.getBoundingClientRect().width < vw * 0.98)
        || [...el.childNodes].some(n => n.nodeType === 3 && n.textContent.trim());
      el = el.parentElement;
    }
    if (hit) filled++;
  }
  const sections = [...document.querySelectorAll('main > section, main > article, body > section, main > div > section')].filter(visible)
    .map(el => { const r = el.getBoundingClientRect(); return {id: el.id || el.className || el.tagName, heightVh: +(r.height / vh).toFixed(2), media: el.querySelectorAll('img,video,canvas,svg,picture').length, textChars: el.textContent.trim().length}; });
  const media = [...document.querySelectorAll('img,video,canvas,picture')].filter(visible).map(el => el.getBoundingClientRect().width / vw);
  return {
    viewport: {width: vw, height: vh}, documentHeightVh: +(document.documentElement.scrollHeight / vh).toFixed(2),
    horizontalOverflowPx: Math.max(0, document.documentElement.scrollWidth - vw),
    type: {h1Px: Math.round(h1Size), h1VwPct: +(h1Size / vw * 100).toFixed(1), bodyPx: Math.round(bodySize), headingToBody: +(h1Size / bodySize).toFixed(2), distinctSizes: new Set(sizes).size},
    firstScreenFillPct: Math.round(filled / cells * 100),
    sections, media: {count: media.length, wide: media.filter(w => w >= 0.6).length, fullBleed: media.filter(w => w >= 0.98).length},
  };
}

function observe(width, m) {
  const notes = [], add = (id, message) => notes.push({id, viewport: width, level: 'REVIEW', message});
  if (m.horizontalOverflowPx > 1) add('horizontal-overflow', `Page scrolls sideways by ${m.horizontalOverflowPx}px.`);
  if (!m.type.h1Px) add('no-h1', 'No visible h1 to anchor the first screen.');
  else if (width >= 1200 && m.type.h1VwPct < 6) add('small-display-type', `Hero headline is ${m.type.h1Px}px (${m.type.h1VwPct}vw). Poster-scale pages usually run 7vw or more.`);
  if (m.type.h1Px && m.type.headingToBody < 3) add('flat-type-scale', `Headline is only ${m.type.headingToBody}x body text; the scale reads flat.`);
  if (m.type.distinctSizes > 10) add('too-many-sizes', `${m.type.distinctSizes} distinct text sizes; a tight scale reads more deliberate.`);
  if (m.firstScreenFillPct < 40) add('empty-first-screen', `Only ${m.firstScreenFillPct}% of the first screen carries type, media or a surface. Check it reads as a composed screen, not small items on empty ground.`);
  if (m.media.fullBleed === 0 && width >= 1200) add('no-full-bleed', 'Nothing runs edge to edge; check whether one moment should break the column.');
  const heights = m.sections.map(s => s.heightVh);
  if (heights.length >= 4) {
    const mean = heights.reduce((a, b) => a + b, 0) / heights.length;
    const spread = Math.sqrt(heights.reduce((a, h) => a + (h - mean) ** 2, 0) / heights.length) / mean;
    if (spread < 0.2) add('even-rhythm', `${heights.length} sections of near-equal height (spread ${spread.toFixed(2)}); the stack may read as a template.`);
  }
  return notes;
}

const report = {schema_version: 1, source: 'Layoza composition probe', target,
  method: 'Full-page images and metrics use reduced motion; a full-page capture resizes the render surface and can change pin and scroll states. Normal-motion images are viewport-only frames at the top, middle and end of the page, captured without any resize.',
  images: [], viewports: [], findings: []};
const browser = await Browser.launch();
try {
  for (const width of [1440, 768, 390]) {
    const page = await browser.newPage({width, height: width === 390 ? 844 : 900, reducedMotion: true});
    try {
      await page.goto(url, {settleMs: 800});
      await page.eval('document.fonts.ready.then(() => true)');
      await page.eval(() => { document.documentElement.style.scrollBehavior = 'auto'; document.querySelectorAll('img[loading="lazy"]').forEach(i => { i.loading = 'eager'; }); });
      const total = await page.eval(() => document.documentElement.scrollHeight);
      for (let y = 0; y < total; y += 700) { await page.eval(y => scrollTo(0, y), y); await sleep(60); }
      await page.eval(() => scrollTo(0, 0)); await sleep(400);
      const metrics = await page.eval(measure);
      const {cssContentSize: size} = await page.send('Page.getLayoutMetrics');
      const shot = await page.send('Page.captureScreenshot', {format: 'png', captureBeyondViewport: true, clip: {x: 0, y: 0, width: Math.ceil(size.width), height: Math.min(16000, Math.ceil(size.height)), scale: 1}});
      const file = `full-${width}.png`;
      fs.writeFileSync(path.join(out, file), Buffer.from(shot.data, 'base64'));
      report.images.push({viewport: width, motion: 'reduced', position: 'full-page', path: path.join(out, file)});
      report.viewports.push({width, height: metrics.viewport.height, metrics});
      report.findings.push(...observe(width, metrics));
    } finally { await page.close(); }
    const live = await browser.newPage({width, height: width === 390 ? 844 : 900, reducedMotion: false});
    try {
      await live.goto(url, {settleMs: 800});
      await live.eval('document.fonts.ready.then(() => true)');
      await live.eval(() => { document.documentElement.style.scrollBehavior = 'auto'; });
      for (const [at, label] of [[0, 'top'], [0.5, 'middle'], [1, 'end']]) {
        await live.eval(at => scrollTo(0, (document.documentElement.scrollHeight - innerHeight) * at), at);
        await live.send('Input.dispatchMouseEvent', {type: 'mouseWheel', x: width / 2, y: 400, deltaX: 0, deltaY: 1});
        await sleep(900);
        const shot = await live.send('Page.captureScreenshot', {format: 'png', captureBeyondViewport: false});
        const file = `normal-${width}-${label}.png`;
        fs.writeFileSync(path.join(out, file), Buffer.from(shot.data, 'base64'));
        report.images.push({viewport: width, motion: 'normal', position: label, path: path.join(out, file)});
      }
    } finally { await live.close(); }
  }
} finally { await browser.close(); }
fs.writeFileSync(path.join(out, 'report.json'), JSON.stringify(report, null, 2));
const review = {source: report.source, target, report: path.join(out, 'report.json'), screenshots: report.images, viewports: report.viewports, observations: report.findings,
  verdict: 'Measurements and review prompts only. Look at the images: a flagged page can be right for its brief and a clean one can still be dull. This report neither passes nor fails artistic quality.'};
fs.writeFileSync(path.join(out, 'composition-review.json'), JSON.stringify(review, null, 2));
console.log(JSON.stringify({report: path.join(out, 'composition-review.json'), observations: review.observations.length, verdict: review.verdict}, null, 2));
