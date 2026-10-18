/** Loads the WebGL hero only when it can help: a wide screen, WebGL present, and the
 *  page fully loaded and idle. Phones keep the CSS stack (it is the same picture and
 *  saves them ~540 KB of three.js). The CSS stack in Hero.astro is the content until then (and forever
 *  if WebGL is missing), so nothing essential waits on this file. */
let dispose: (() => void) | null = null;

function hasWebGL() {
  try {
    const c = document.createElement('canvas');
    return !!(c.getContext('webgl2') || c.getContext('webgl'));
  } catch {
    return false;
  }
}

export function mountEngine() {
  const stage = document.querySelector<HTMLElement>('[data-engine]');
  if (!stage || !window.matchMedia('(min-width: 900px)').matches || !hasWebGL()) return;
  const idle = (cb: () => void) =>
    'requestIdleCallback' in window ? requestIdleCallback(cb, { timeout: 3000 }) : setTimeout(cb, 1500);
  const whenLoaded = (cb: () => void) =>
    document.readyState === 'complete' ? cb() : window.addEventListener('load', cb, { once: true });
  whenLoaded(() => idle(async () => {
    if (!stage.isConnected) return;
    const [{ createEngine }] = await Promise.all([import('./engine'), document.fonts.ready]);
    if (!stage.isConnected) return;
    dispose = createEngine(stage);
  }));
}

// Release the GL context when the page is put away (also covers bfcache).
window.addEventListener('pagehide', () => {
  dispose?.();
  dispose = null;
});
