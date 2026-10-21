// Original Layoza GSAP lifecycle controller. Inject a pinned GSAP runtime.
// Three.js still renders the scene; GSAP owns authored state and timing.
export function createGsapMotion({ root, gsap, ScrollTrigger, enabled = true,
  compose = () => {}, render, resolve = () => {} } = {}) {
  const still = () => { resolve(); render?.({ time: 0, deltaMs: 0, static: true }); };
  const inert = status => ({ status, pause() {}, resume() {}, destroy() {}, get active() { return false; } });
  if (!enabled) { still(); return inert('disabled'); }
  if (typeof window === 'undefined' || typeof document === 'undefined') return inert('server');
  if (!gsap?.context || !gsap?.ticker) { still(); return inert('missing-gsap-static'); }
  root ||= document.documentElement;
  if (ScrollTrigger) gsap.registerPlugin(ScrollTrigger);
  const media = window.matchMedia('(prefers-reduced-motion: reduce)');
  let context = null, paused = false, visible = false, disposed = false, ticking = false;
  let listeners = [], disposeComposition = null;
  function clearContext() {
    disposeComposition?.(); disposeComposition = null;
    listeners.forEach(remove => remove()); listeners = [];
    context?.revert(); context = null;
  }
  function tick(time, deltaMs) { render?.({ time, deltaMs, static: false }); }
  function syncTicker() {
    const needsTick = Boolean(render && context && visible && !document.hidden && !disposed);
    if (needsTick && !ticking) { gsap.ticker.add(tick); ticking = true; }
    if (!needsTick && ticking) { gsap.ticker.remove(tick); ticking = false; }
  }
  function rebuild() {
    if (disposed) return;
    clearContext();
    if (!paused && !media.matches) {
      context = gsap.context(() => {
        const cleanup = compose({ gsap, ScrollTrigger, root,
          timeline: options => gsap.timeline(options),
          on(target, type, callback, options) {
            const safe = event => { if (context && !disposed) context.add(() => callback(event)); };
            target.addEventListener(type, safe, options);
            listeners.push(() => target.removeEventListener(type, safe, options));
          }
        });
        if (typeof cleanup === 'function') disposeComposition = cleanup;
      }, root);
    } else still();
    syncTicker();
  }
  const observer = new IntersectionObserver(entries => {
    visible = entries[0].isIntersecting;
    syncTicker();
  }, { rootMargin: '64px' });
  observer.observe(root);
  media.addEventListener('change', rebuild);
  document.addEventListener('visibilitychange', syncTicker);
  rebuild();
  return {
    status: 'gsap',
    get active() { return Boolean(context); },
    pause() { if (!disposed && !paused) { paused = true; rebuild(); } },
    resume() { if (!disposed && paused) { paused = false; rebuild(); } },
    destroy() {
      if (disposed) return;
      disposed = true;
      clearContext(); syncTicker(); observer.disconnect();
      media.removeEventListener('change', rebuild);
      document.removeEventListener('visibilitychange', syncTicker);
    }
  };
}
