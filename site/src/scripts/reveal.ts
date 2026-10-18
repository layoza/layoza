/** Scroll reveals and the per-page scroll story. Content is visible without JS;
 *  the lowered start state only exists under html.js-motion (see global.css).
 *  Reveals and one-shot toggles use IntersectionObserver, so nothing waits on GSAP;
 *  the GSAP scenes load once the browser is idle. */
const cleanups: (() => void)[] = [];
let gen = 0;

function observe(sel: string, cls: string, margin: string) {
  const els = [...document.querySelectorAll<HTMLElement>(sel)];
  if (!els.length) return;
  const io = new IntersectionObserver((entries) => {
    for (const e of entries) {
      if (!e.isIntersecting) continue;
      e.target.classList.add(cls);
      io.unobserve(e.target);
    }
  }, { rootMargin: margin });
  els.forEach((el) => io.observe(el));
  cleanups.push(() => io.disconnect());
}

export function initReveals() {
  teardown();
  if (!document.documentElement.classList.contains('js-motion')) return;
  const run = ++gen;

  document.querySelectorAll<HTMLElement>('[data-reveal]').forEach((el) => {
    const d = Number(el.dataset.reveal) || 0;
    if (d) el.style.setProperty('--rd', `${d}s`);
  });
  observe('[data-reveal]', 'in', '0px 0px -12% 0px');
  observe('[data-qa]', 'live', '0px 0px -25% 0px');
  observe('[data-blueprint]', 'on', '0px 0px -25% 0px');
  // Failsafe: never leave text lowered. Text resting inside the observer's bottom margin would
  // otherwise stay faded, so whatever is on screen when scrolling settles is revealed too.
  const showOnScreen = () => {
    document.querySelectorAll('[data-reveal]:not(.in)').forEach((el) => {
      const r = el.getBoundingClientRect();
      if (r.top < window.innerHeight && r.bottom > 0) el.classList.add('in');
    });
  };
  const fail = window.setTimeout(showOnScreen, 2500);
  let settle = 0;
  const onScroll = () => { clearTimeout(settle); settle = window.setTimeout(showOnScreen, 150); };
  window.addEventListener('scroll', onScroll, { passive: true });
  cleanups.push(() => { clearTimeout(fail); clearTimeout(settle); window.removeEventListener('scroll', onScroll); });

  if (!document.querySelector('[data-problem], [data-dna], [data-flow], [data-final], [data-colophon]')) return;
  const idle = (cb: () => void) =>
    'requestIdleCallback' in window ? requestIdleCallback(cb, { timeout: 2000 }) : setTimeout(cb, 600);
  idle(async () => {
    const { startScenes } = await import('./scenes');
    if (run !== gen) return;
    cleanups.push(startScenes());
  });
}

function teardown() {
  gen++;
  cleanups.splice(0).forEach((c) => c());
}
