// Layoza spatial work stage. MIT. Supply your own artwork and design-system colours.
// THREE is injected: use the project's pinned module, never a hidden network dependency.
export function mountSpatialWorkStage(THREE, host, options) {
  const { artworks, background, frame, onState = () => {}, onError = () => {} } = options;
  if (!host || !artworks?.length || !background || !frame) throw new Error('A host, real artwork and design-system colours are required');
  const abort = new AbortController(), signal = abort.signal;
  const media = matchMedia('(prefers-reduced-motion: reduce)');
  let renderer, disposed = false, lost = false, visible = false, paused = false, raf = 0;
  let progress = 0, intro = 0, targetX = 0, targetY = 0, pointerX = 0, pointerY = 0, last = 0;
  const scene = new THREE.Scene(), camera = new THREE.PerspectiveCamera(38, 1, .1, 100);
  const resources = new Set(), cards = [], group = new THREE.Group();
  scene.add(group);
  const own = r => { resources.add(r); return r; };
  const canvas = document.createElement('canvas'); canvas.setAttribute('aria-hidden', 'true');
  try { renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: false }); }
  catch (error) { onError(error); return { setProgress() {}, setPaused() {}, dispose() {}, webgl: false }; }
  renderer.setPixelRatio(Math.min(devicePixelRatio || 1, 2));
  renderer.setClearColor(background); renderer.outputColorSpace = THREE.SRGBColorSpace;
  host.append(canvas);
  const loader = new THREE.TextureLoader();
  const ready = Promise.all(artworks.map((art, i) => new Promise(resolve => {
    loader.load(art.src, texture => {
      if (disposed) { texture.dispose(); resolve(false); return; }
      own(texture); texture.colorSpace = THREE.SRGBColorSpace;
      const width = 3.2, height = width / (art.aspect || 4 / 3);
      const geometry = own(new THREE.PlaneGeometry(width, height, 48, 24));
      const base = geometry.attributes.position.array.slice();
      const material = own(new THREE.MeshBasicMaterial({ map: texture, side: THREE.DoubleSide }));
      const plate = new THREE.Group(), mesh = new THREE.Mesh(geometry, material);
      const backing = new THREE.Mesh(own(new THREE.BoxGeometry(width + .07, height + .07, .065)), own(new THREE.MeshBasicMaterial({ color: frame })));
      backing.position.z = -.055; plate.add(backing, mesh); group.add(plate);
      cards[i] = { plate, geometry, base, backing, width }; resolve(true);
    }, undefined, error => { onError(error); resolve(false); });
  }))).then(loaded => {
    if (disposed) return false;
    // Keep the HTML poster if even one required texture is missing.
    if (loaded.every(Boolean)) host.dataset.webglReady = 'true';
    draw(); wake(); return loaded.every(Boolean);
  });
  const clamp = x => Math.max(0, Math.min(1, x));
  const smooth = x => { x = clamp(x); return x * x * (3 - 2 * x); };
  function draw() {
    if (disposed || lost) return;
    const staticMode = paused || media.matches;
    const p = staticMode ? .52 : Math.max(progress, intro * .38);
    const unfold = smooth(p / .5), select = smooth((p - .55) / .45), phone = host.clientWidth < 700;
    cards.forEach((card, i) => {
      const { plate, geometry, base, backing, width } = card, a = geometry.attributes.position;
      const curl = (1 - unfold) * .65;
      for (let n = 0; n < a.count; n++) {
        const x = base[n * 3], y = base[n * 3 + 1];
        a.setXYZ(n, x, y + Math.sin(x / width * Math.PI) * curl * .12, (x * x / width) * curl);
      }
      a.needsUpdate = true;
      backing.visible = unfold > .9;
      if (phone) {
        // One complete feature above two recognizable supporting works, not a
        // stack whose front plate hides every other project's identity.
        plate.position.set(i === 0 ? 0 : (i === 1 ? -.95 : .95), i === 0 ? .55 - select * .1 : -1.22 - select * .13, i === 0 ? .4 : -.15);
        plate.rotation.set(0, (i === 0 ? -.1 : (i === 1 ? .12 : -.12)) * (1 - unfold), (i === 1 ? -.04 : i === 2 ? .04 : 0) * (1 - select));
        plate.scale.setScalar(i === 0 ? .82 + select * .21 : .55 - select * .09);
      } else {
        const x = (i - 1) * 3.35;
        plate.position.set(x * (1 - select) + (i === 0 ? 0 : i === 1 ? -4.7 : 4.7) * select, (i === 1 ? .18 : -.1) * (1 - unfold), (i === 0 ? select * 1.5 : -select * 1.3));
        plate.rotation.set((1 - unfold) * (i === 1 ? -.1 : .1), (1 - unfold) * (1 - i) * .5, (1 - unfold) * (i - 1) * .12);
        plate.scale.setScalar(i === 0 ? 1 + select * .7 : 1 - select * .18);
      }
    });
    group.rotation.set(staticMode ? 0 : pointerY * .035, staticMode ? 0 : pointerX * .06, 0);
    renderer.render(scene, camera);
    onState({ progress: p, selected: select > .65 ? 0 : null, paused: staticMode, webgl: host.dataset.webglReady === 'true' });
  }
  function eligible() { return !disposed && !lost && visible && !document.hidden && !paused && !media.matches; }
  function tick(now) {
    raf = 0; if (!eligible()) return;
    intro = Math.min(1, intro + Math.min(.05, (now - (last || now)) / 1000) / 1.7); last = now;
    pointerX += (targetX - pointerX) * .08; pointerY += (targetY - pointerY) * .08;
    draw(); raf = requestAnimationFrame(tick);
  }
  function stop() { cancelAnimationFrame(raf); raf = 0; last = 0; }
  function wake() { if (eligible() && !raf) raf = requestAnimationFrame(tick); }
  function sync() { stop(); draw(); wake(); }
  const resize = () => {
    if (disposed || lost) return;
    const w = Math.max(1, host.clientWidth), h = Math.max(1, host.clientHeight);
    // Fit the complete 10-unit triptych horizontally, including at tablet widths.
    camera.aspect = w / h; camera.position.set(0, 0, w < 700 ? 6.3 : Math.max(5.2, 16.3 / camera.aspect));
    camera.updateProjectionMatrix(); renderer.setSize(w, h, false); draw();
  };
  const ro = new ResizeObserver(resize); ro.observe(host);
  const io = new IntersectionObserver(([entry]) => { visible = entry.isIntersecting; sync(); }); io.observe(host);
  document.addEventListener('visibilitychange', sync, { signal }); media.addEventListener('change', sync, { signal });
  host.addEventListener('pointermove', e => { if (e.pointerType !== 'mouse') return; const r = host.getBoundingClientRect(); targetX = (e.clientX - r.left) / r.width * 2 - 1; targetY = (e.clientY - r.top) / r.height * 2 - 1; }, { signal });
  host.addEventListener('pointerleave', () => { targetX = targetY = 0; }, { signal });
  canvas.addEventListener('webglcontextlost', e => { e.preventDefault(); lost = true; stop(); delete host.dataset.webglReady; onError(new Error('WebGL context lost; HTML poster retained')); }, { signal });
  canvas.addEventListener('webglcontextrestored', () => { if (disposed) return; lost = false; ready.then(ok => { if (ok) host.dataset.webglReady = 'true'; resize(); sync(); }); }, { signal });
  resize();
  return { webgl: true, ready,
    setProgress(value) { progress = clamp(value); if (!paused && !media.matches) draw(); },
    setPaused(value) { paused = Boolean(value); sync(); },
    diagnostics() { return { raf: Boolean(raf), visible, paused: paused || media.matches, lost, disposed, textures: cards.filter(Boolean).length }; },
    dispose() { if (disposed) return; disposed = true; stop(); abort.abort(); ro.disconnect(); io.disconnect(); resources.forEach(r => r.dispose()); renderer.dispose(); canvas.remove(); delete host.dataset.webglReady; }
  };
}
