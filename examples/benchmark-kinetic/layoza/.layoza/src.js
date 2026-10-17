const $ = (s, r = document) => r.querySelector(s);
const $$ = (s, r = document) => [...r.querySelectorAll(s)];
const root = document.documentElement;
const reduce = matchMedia('(prefers-reduced-motion: reduce)').matches;
const fine = matchMedia('(hover: hover) and (pointer: fine)').matches;
const harness = new URLSearchParams(location.search).has('t');
const G = window.gsap, ST = window.ScrollTrigger, SPLIT = window.SplitText, M = window.Motion;
const TAU = Math.PI * 2;
const yr = $('#yr'); if (yr) yr.textContent = new Date().getFullYear();

/* ---------- shared signal state: the page-long thread ---------- */
const sig = { p: 0, v: 0, boost: 0, pulse: 0 };
const ACTS = ['Input', 'Modulate', 'Amplify', 'Transmit'];
const readN = $('#readout-n'), readT = $('#readout-t');
let lastAct = -1;
function setAct(a) {
  if (a === lastAct) return; lastAct = a;
  if (readN) { readN.textContent = '0' + (a + 1); readT.textContent = ACTS[a]; }
}
function scrollState() {
  const max = root.scrollHeight - innerHeight;
  sig.p = max > 0 ? Math.min(1, Math.max(0, scrollY / max)) : 0;
}

/* ---------- three.js ribbon (lazy, never blocks the headline) ---------- */
const ribbonApi = { heroP: 0, unfold: null };
async function initRibbon() {
  const canvas = $('#ribbon');
  const probe = document.createElement('canvas');
  const ok = !!(window.WebGLRenderingContext && (probe.getContext('webgl2') || probe.getContext('webgl')));
  if (!ok) { root.classList.add('no-3d'); return; }
  // let the headline and lede paint first (LCP), then build the GPU scene
  await new Promise((res) => { const l = $('.hero__ctas > *'); if (reduce || !l) return res(); l.addEventListener('animationstart', () => setTimeout(res, 120), { once: true }); setTimeout(res, 1700); });
  let THREE;
  try { THREE = await import('https://cdn.jsdelivr.net/npm/three@0.160.0/build/three.module.min.js'); }
  catch (e) { root.classList.add('no-3d'); return; }
  const small = innerWidth < 768;
  let renderer;
  try { renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true, powerPreference: 'high-performance' }); }
  catch (e) { root.classList.add('no-3d'); return; }
  const gl = renderer.getContext();
  const dbg = gl.getExtension('WEBGL_debug_renderer_info');
  const soft = dbg && /swiftshader|llvmpipe|software/i.test(gl.getParameter(dbg.UNMASKED_RENDERER_WEBGL));
  renderer.setPixelRatio(soft ? 0.75 : Math.min(devicePixelRatio, small ? 1.5 : 2));
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 1.05;
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(30, 1, 0.1, 100);
  camera.position.set(0, 0, 10);

  // studio environment: dark room, white softboxes, one cobalt panel, a faint lavender bounce
  const env = new THREE.Scene();
  const disposables = [];
  const roomG = new THREE.BoxGeometry(30, 30, 30), roomM = new THREE.MeshBasicMaterial({ color: 0x07080c, side: THREE.BackSide });
  env.add(new THREE.Mesh(roomG, roomM)); disposables.push(roomG, roomM);
  const panel = (w, h, rgb, pos, look) => {
    const g = new THREE.PlaneGeometry(w, h), m = new THREE.MeshBasicMaterial({ color: new THREE.Color(...rgb), side: THREE.DoubleSide });
    const p = new THREE.Mesh(g, m); p.position.set(...pos); p.lookAt(...look); env.add(p); disposables.push(g, m);
  };
  panel(14, 5, [5, 5, 5.4], [0, 9, 2], [0, 0, 0]);
  panel(2, 14, [9, 9, 9], [9, 1, 4], [0, 0, 0]);
  panel(10, 12, [0.5, 0.75, 5.5], [-10, 0, -1], [0, 0, 0]);
  panel(12, 3, [1.4, 1.2, 3.2], [0, -8, 3], [0, 0, 0]);
  panel(4, 4, [2.2, 2.2, 2.4], [-3, 2, 11], [0, 0, 0]);
  const pmrem = new THREE.PMREMGenerator(renderer);
  const envTex = pmrem.fromScene(env, 0.035).texture;
  scene.environment = envTex;

  const NU = soft ? 170 : small ? 220 : 320, NV = 9;
  const geo = new THREE.BufferGeometry();
  const pos = new Float32Array((NU + 1) * (NV + 1) * 3);
  geo.setAttribute('position', new THREE.BufferAttribute(pos, 3));
  const idx = [];
  for (let i = 0; i < NU; i++) for (let j = 0; j < NV; j++) {
    const a = i * (NV + 1) + j, b = a + NV + 1;
    idx.push(a, b, a + 1, b, b + 1, a + 1);
  }
  geo.setIndex(idx);
  const mat = new THREE.MeshPhysicalMaterial({ color: 0xe4e8f0, metalness: 1, roughness: 0.2, clearcoat: 1, clearcoatRoughness: 0.1, side: THREE.DoubleSide, envMapIntensity: 1.25 });
  const mesh = new THREE.Mesh(geo, mat);
  const group = new THREE.Group(); group.add(mesh); scene.add(group);

  const C = new Float32Array((NU + 1) * 3);
  const tmp = { t: [0, 0, 0], n: [0, 0, 0], b: [0, 0, 0] };
  function centre(u, k, t, out, o) {
    const s = u * 2 - 1;
    const th = Math.PI * 1.18 * s * (0.35 + 0.65 * k);
    out[o] = (2.15 * Math.sin(th) + 0.28 * Math.sin(3 * th + t * 0.6)) * (0.4 + 0.6 * k);
    out[o + 1] = 0.62 * Math.sin(2 * th + 0.4 + t * 0.35) * k + 0.42 * s * k;
    out[o + 2] = 1.25 * Math.cos(th) * k - 0.4;
  }
  function build(k, t) {
    for (let i = 0; i <= NU; i++) centre(i / NU, k, t, C, i * 3);
    let p = 0;
    for (let i = 0; i <= NU; i++) {
      const i0 = Math.max(0, i - 1) * 3, i1 = Math.min(NU, i + 1) * 3, o = i * 3;
      let tx = C[i1] - C[i0], ty = C[i1 + 1] - C[i0 + 1], tz = C[i1 + 2] - C[i0 + 2];
      const tl = Math.hypot(tx, ty, tz) || 1; tx /= tl; ty /= tl; tz /= tl;
      // n = normalize(t x up), b = n x t
      let nx = tz * 0 - 0 * ty, ny = 0, nz = 0;
      nx = -tz; ny = 0; nz = tx; // t x (0,1,0)
      const nl = Math.hypot(nx, ny, nz) || 1; nx /= nl; ny /= nl; nz /= nl;
      const bx = ny * tz - nz * ty, by = nz * tx - nx * tz, bz = nx * ty - ny * tx;
      const u = i / NU;
      const phi = (2.6 * Math.PI * u + 0.5 * Math.sin(t * 0.4 + u * 3)) * (0.5 + 0.5 * k) + 0.3;
      const cw = Math.cos(phi), sw = Math.sin(phi);
      const wx = nx * cw + bx * sw, wy = ny * cw + by * sw, wz = nz * cw + bz * sw;
      // cup direction: perpendicular to t and w
      const cx = ty * wz - tz * wy, cy = tz * wx - tx * wz, cz = tx * wy - ty * wx;
      const width = (0.06 + 0.38 * Math.pow(Math.sin(Math.PI * u), 0.55)) * (0.45 + 0.55 * k);
      for (let j = 0; j <= NV; j++) {
        const v = j / NV * 2 - 1, cup = 0.1 * v * v * width;
        pos[p++] = C[o] + wx * v * width + cx * cup;
        pos[p++] = C[o + 1] + wy * v * width + cy * cup;
        pos[p++] = C[o + 2] + wz * v * width + cz * cup;
      }
    }
    geo.attributes.position.needsUpdate = true;
    geo.computeVertexNormals();
  }

  const state = { k: 0.05, spin: -1.3, scale: 0.35 };
  let W = 0, H = 0, layout = { x: 0, y: 0, s: 1 };
  function resize() {
    const r = canvas.getBoundingClientRect(); W = r.width; H = r.height;
    if (!W || !H) return;
    renderer.setSize(W, H, false);
    camera.aspect = W / H; camera.updateProjectionMatrix();
    const vh = 2 * Math.tan((camera.fov * Math.PI / 180) / 2) * camera.position.z, vw = vh * camera.aspect;
    if (camera.aspect < 0.85) layout = { x: 0, y: vh * 0.02, s: Math.min(vw * 0.98 / 4.8, vh * 0.42 / 2.6) };
    else layout = { x: vw * 0.17, y: vh * 0.1, s: Math.min(vw * 0.58 / 4.8, vh * 0.72 / 2.6) };
  }
  resize();
  const ptr = { x: 0, y: 0, rx: 0, ry: 0 };
  if (fine && !reduce) addEventListener('pointermove', (e) => { ptr.x = e.clientX / innerWidth * 2 - 1; ptr.y = e.clientY / innerHeight * 2 - 1; }, { passive: true });
  let t = 0, visible = true, running = false, last = performance.now();
  function frame(now) {
    const dt = Math.min(0.05, (now - last) / 1000); last = now;
    if (!reduce) t += dt * 0.55;
    ptr.rx += (ptr.y * 0.22 - ptr.rx) * 0.045; ptr.ry += (ptr.x * 0.35 - ptr.ry) * 0.045;
    const hp = ribbonApi.heroP;
    build(state.k, t);
    group.position.set(layout.x * (1 - hp * 0.9), layout.y * (1 - hp) - hp * 0.4, 0);
    const s = layout.s * state.scale * (1 + hp * 0.18);
    group.scale.setScalar(s);
    group.rotation.set(0.28 + ptr.rx + hp * 0.35, -0.35 + state.spin + ptr.ry + hp * 1.25 + (reduce ? 0 : Math.sin(t * 0.25) * 0.12), -0.12 + hp * 0.2);
    renderer.render(scene, camera);
  }
  // adaptive quality: software GL or slow frames render every other frame
  let skip = soft, odd = false, slow = 0;
  function loop(now) {
    if (!running) return;
    odd = !odd;
    if (!skip || odd) { const t0 = performance.now(); frame(now); if (performance.now() - t0 > 14 && ++slow > 20) skip = true; }
    requestAnimationFrame(loop);
  }
  function start() { if (running || reduce) return; running = true; last = performance.now(); requestAnimationFrame(loop); }
  function stop() { running = false; }
  ribbonApi.render = () => frame(performance.now());
  new IntersectionObserver(([e]) => { visible = e.isIntersecting; visible && !document.hidden ? start() : stop(); }).observe(canvas);
  document.addEventListener('visibilitychange', () => { document.hidden ? stop() : visible && start(); });
  addEventListener('resize', () => { resize(); if (!running) ribbonApi.render(); });
  root.classList.add('is-3d');
  // unfold: compressed knot -> open sculpture (0.7s - 1.6s of the entrance, or as soon as ready)
  const elapsed = performance.now() / 1000;
  if (reduce || !G) { state.k = 1; state.spin = 0; state.scale = 1; frame(performance.now()); }
  else {
    G.to(state, { k: 1, spin: 0, scale: 1, duration: 1.2, ease: 'expo.out', delay: Math.max(0, 0.7 - elapsed), onUpdate: () => { if (!running) frame(performance.now()); } });
  }
  start();
  addEventListener('pagehide', () => { stop(); geo.dispose(); mat.dispose(); envTex.dispose(); pmrem.dispose(); disposables.forEach((d) => d.dispose()); renderer.dispose(); });
}
fetch('https://cdn.jsdelivr.net/npm/three@0.160.0/build/three.module.min.js', { mode: 'cors' }).catch(() => {});
initRibbon();

/* ---------- everything below needs GSAP; content is already visible without it ---------- */
if (G && ST) {
  G.registerPlugin(ST); if (SPLIT) G.registerPlugin(SPLIT);
  G.defaults({ ease: 'power3.out' });

  /* Lenis smooth scroll, synced to ScrollTrigger */
  let lenis = null;
  if (!reduce && fine && window.Lenis && !harness) {
    lenis = new window.Lenis({ lerp: 0.11, smoothWheel: true });
    root.classList.add('lenis');
    lenis.on('scroll', ST.update);
    G.ticker.add((time) => lenis.raf(time * 1000));
    G.ticker.lagSmoothing(0);
  }
  window.__ks = { lenis };

  /* in-page anchors: smooth, offset under the header, focus moves with the reader */
  $$('a[href^="#"]').forEach((a) => a.addEventListener('click', (e) => {
    const id = a.getAttribute('href'); const el = id.length > 1 ? $(id) : $('#top');
    if (!el) return;
    e.preventDefault(); closeMenu(false);
    const focusIt = () => { if (!el.hasAttribute('tabindex')) el.setAttribute('tabindex', '-1'); el.focus({ preventScroll: true }); };
    if (lenis) lenis.scrollTo(el, { offset: id === '#top' ? 0 : -70, duration: 1.3, onComplete: focusIt });
    else { el.scrollIntoView({ behavior: reduce ? 'auto' : 'smooth' }); focusIt(); }
    history.replaceState(null, '', id);
  }));

  /* header: compact after leaving the top */
  const hdr = $('#hdr');
  ST.create({ start: 40, end: 'max', onToggle: (s) => hdr.classList.toggle('is-compact', s.isActive) });

  /* header trace: the signal line. amplitude and frequency follow page progress, speed follows scroll velocity */
  const tc = $('#trace'), tx = tc.getContext('2d');
  let tw = 0, th = 0, phase = 0;
  const sizeTrace = () => { const d = Math.min(devicePixelRatio, 2); tw = tc.clientWidth; th = tc.clientHeight; tc.width = tw * d; tc.height = th * d; tx.setTransform(d, 0, 0, d, 0, 0); };
  sizeTrace(); addEventListener('resize', sizeTrace);
  function drawTrace(dt) {
    const vel = Math.min(Math.abs(sig.v) / 2500, 1);
    sig.boost += (vel - sig.boost) * 0.08; sig.pulse *= 0.94;
    if (!reduce) phase += dt * (1.4 + sig.boost * 7);
    const amp = (1 + 7 * sig.p) * (1 + sig.boost * 0.7 + sig.pulse);
    const cyc = 3 + 20 * sig.p, mid = th / 2;
    tx.clearRect(0, 0, tw, th);
    tx.lineWidth = 1.5; tx.strokeStyle = '#4865FF'; tx.beginPath();
    for (let x = 0; x <= tw; x += 3) {
      const u = x / tw, env = 0.35 + 0.65 * Math.sin(Math.PI * u);
      const y = mid + Math.sin(TAU * cyc * u - phase) * amp * env;
      x ? tx.lineTo(x, y) : tx.moveTo(x, y);
    }
    tx.stroke();
    const dx = sig.p * tw, du = sig.p;
    const dy = mid + Math.sin(TAU * cyc * du - phase) * amp * (0.35 + 0.65 * Math.sin(Math.PI * du));
    tx.fillStyle = '#D8FF57'; tx.beginPath(); tx.arc(Math.max(4, Math.min(tw - 4, dx)), dy, 3.5, 0, TAU); tx.fill();
  }
  G.ticker.add((time, deltaMs) => drawTrace(deltaMs / 1000));
  $$('.btn').forEach((b) => b.addEventListener('pointerenter', () => { sig.pulse = 1.6; }));

  const mm = G.matchMedia();
  mm.add({ desk: '(min-width: 960px)', mob: '(max-width: 959px)', motion: '(prefers-reduced-motion: no-preference)' }, (ctx) => {
    const { desk, motion } = ctx.conditions;
    if (!motion) return;

    /* HERO scroll: headline drifts, the sculpture turns and moves to centre, CTAs recede */
    G.timeline({ scrollTrigger: { trigger: '.hero', start: 'top top', end: 'bottom top', scrub: true, onUpdate: (s) => { ribbonApi.heroP = s.progress; if (ribbonApi.render && reduce) ribbonApi.render(); } } })
      .to('.headline', { yPercent: desk ? -22 : -10, xPercent: desk ? 4 : 0, ease: 'none' }, 0)
      .to('.hero__bottom', { y: -50, autoAlpha: 0.3, scale: 0.97, transformOrigin: 'left bottom', ease: 'none' }, 0)
      .to('.hero__aside', { opacity: 0.15, y: -30, ease: 'none', duration: 0.4 }, 0);

    /* INTRO: three stages, dividers extend, key phrase goes outline -> solid with scroll */
    G.from('.intro__head', { scaleX: 0.0001, duration: 1.1, ease: 'expo.out', scrollTrigger: { trigger: '.intro', start: 'top 80%', once: true } });
    if (SPLIT) {
      SPLIT.create('.intro__a', { type: 'lines', mask: 'lines', linesClass: 'il', onSplit: (s) => G.from(s.lines, { yPercent: 110, duration: 1, ease: 'expo.out', stagger: 0.09, scrollTrigger: { trigger: '.intro__h', start: 'top 82%', once: true } }) });
    }
    G.from('.intro__b', { y: 40, autoAlpha: 0, duration: 0.9, delay: 0.25, scrollTrigger: { trigger: '.intro__h', start: 'top 82%', once: true } });
    const kp = $('.kp');
    G.fromTo(kp, { '--f': 0 }, { '--f': 1, ease: 'none', scrollTrigger: { trigger: '.intro__h', start: 'top 70%', end: 'bottom 30%', scrub: 0.5 } });
    G.from('.intro__txt mark', { '--hl': '0%', duration: 0.8, stagger: 0.25, ease: 'power2.inOut', scrollTrigger: { trigger: '.intro__txt', start: 'top 75%', once: true } });
    G.from('.disc-list li', { x: 28, autoAlpha: 0, duration: 0.7, stagger: 0.07, scrollTrigger: { trigger: '.disc-list', start: 'top 85%', once: true } });
    G.from('.scope', { y: 60, duration: 1.1, ease: 'expo.out', scrollTrigger: { trigger: '.scope', start: 'top 88%', once: true } });
    // the scope: a flat input line gains its first wave as you read
    const sp = $('#scope-path'), sd = $('#scope-dot');
    const scope = { a: 0 };
    const drawScope = () => {
      let d = ''; const a = scope.a;
      for (let i = 0; i <= 120; i++) {
        const x = i * 5, u = x / 600, gate = u < 0.25 ? 0 : Math.min(1, (u - 0.25) / 0.2);
        const y = 100 - Math.sin(TAU * 3.2 * (u - 0.25)) * 70 * a * gate * (0.6 + 0.4 * Math.sin(Math.PI * u));
        d += (i ? ' L' : 'M') + x + ',' + y.toFixed(1);
      }
      sp.setAttribute('d', d);
    };
    G.to(scope, { a: 1, ease: 'none', onUpdate: drawScope, scrollTrigger: { trigger: '.scope', start: 'top 85%', end: 'bottom 35%', scrub: 0.6 } });
    G.to(sd, { attr: { r: 9 }, repeat: -1, yoyo: true, duration: 0.8, ease: 'sine.inOut' });

    /* SERVICES heading: letter tracking compresses into place */
    G.fromTo('#svc-h', { letterSpacing: '0.12em', autoAlpha: 0.2 }, { letterSpacing: '-0.04em', autoAlpha: 1, ease: 'none', scrollTrigger: { trigger: '.svcs', start: 'top 85%', end: 'top 25%', scrub: 0.6 } });
    G.from('.svcs__sub', { y: 24, autoAlpha: 0, duration: 0.8, scrollTrigger: { trigger: '.svcs__sub', start: 'top 90%', once: true } });

    /* WORK: heading words stagger, media shutters open, captions follow with offset */
    if (SPLIT) SPLIT.create('#work-h', { type: 'words', mask: 'words', onSplit: (s) => G.from(s.words, { yPercent: 105, rotate: 4, duration: 1, ease: 'expo.out', stagger: 0.1, scrollTrigger: { trigger: '#work-h', start: 'top 85%', once: true } }) });
    $$('.proj').forEach((p) => {
      const tl = G.timeline({ scrollTrigger: { trigger: p, start: 'top 82%', once: true } });
      tl.from($('.proj__media', p), { clipPath: 'inset(100% 0% 0% 0%)', duration: 1.2, ease: 'expo.out' })
        .from($('.zm', p), { scale: 1.14, svgOrigin: (() => { const vb = $('svg', p).viewBox.baseVal; return `${vb.width / 2} ${vb.height / 2}`; })(), duration: 1.4, ease: 'expo.out' }, 0)
        .from($$('.proj__name, .proj__info > *', p), { y: 36, duration: 0.9, ease: 'expo.out', stagger: 0.07 }, 0.3);
    });
    if (desk) G.to('.proj--c', { yPercent: -10, ease: 'none', scrollTrigger: { trigger: '.proj--b', start: 'top bottom', end: 'bottom top', scrub: 0.8 } });

    /* PHILOSOPHY: one coordinated timeline. seen -> felt */
    const pp = $('#phil-path'), pd = $('#phil-dot');
    const pulse = { a: 0, x: 0 };
    const drawPulse = () => {
      let d = '';
      for (let i = 0; i <= 160; i++) {
        const u = i / 160, x = u * 1440;
        const beat = Math.exp(-Math.pow((u - pulse.x) / 0.035, 2));
        const y = 60 - pulse.a * (Math.sin(TAU * 6 * u) * 8 + beat * 52 * Math.sin(TAU * 18 * (u - pulse.x)));
        d += (i ? ' L' : 'M') + x.toFixed(1) + ',' + y.toFixed(1);
      }
      pp.setAttribute('d', d); pd.setAttribute('cx', (pulse.x * 1440).toFixed(1));
    };
    const s1 = $('.phil__s1'), s2lines = $$('.phil__s2 .ln > span'), felt = $$('.felt > span');
    if (desk) {
      const tl = G.timeline({ defaults: { ease: 'power2.inOut' }, scrollTrigger: { trigger: '.phil__pin', start: 'top top', end: '+=150%', pin: true, scrub: 0.8, anticipatePin: 1, onToggle: (s) => $('.phil').classList.toggle('is-pinned', s.isActive) } });
      window.__ks.phil = tl;
      const big = Math.min(4.2, (innerWidth * 0.82) / s1.getBoundingClientRect().width);
      tl.addLabel('seen')
        .fromTo(s1, { scale: big, y: () => innerHeight * 0.5 - s1.offsetTop - s1.offsetHeight * big / 2, '--strike': 0 }, { scale: big, y: () => innerHeight * 0.5 - s1.offsetTop - s1.offsetHeight * big / 2, '--strike': 0, duration: 0.15 })
        .fromTo(s2lines, { yPercent: 108 }, { yPercent: 108, duration: 0.15 }, 0)
        .addLabel('shift')
        .to(s1, { scale: 1, y: 0, duration: 0.45 }, 'shift')
        .to(s1, { '--strike': 1, duration: 0.2, ease: 'power1.out' }, 'shift+=0.3')
        .to(s2lines, { yPercent: 0, duration: 0.4, stagger: 0.08, ease: 'power3.out' }, 'shift+=0.2')
        .addLabel('felt')
        .fromTo(felt, { y: 0 }, { keyframes: { y: [0, '-0.08em', '0.05em', 0] }, duration: 0.3, stagger: { each: 0.04, from: 'start' }, ease: 'none' }, 'felt-=0.1')
        .fromTo(pulse, { a: 0, x: 0 }, { a: 1, x: 1, duration: 0.75, ease: 'none', onUpdate: drawPulse }, 'shift-=0.05');
    } else {
      G.from(s2lines, { yPercent: 108, duration: 1, stagger: 0.1, ease: 'expo.out', scrollTrigger: { trigger: '.phil__s2', start: 'top 85%', once: true } });
      G.fromTo(s1, { '--strike': 0 }, { '--strike': 1, duration: 0.6, delay: 0.5, scrollTrigger: { trigger: '.phil__s2', start: 'top 85%', once: true } });
      G.fromTo(pulse, { a: 0, x: 0 }, { a: 1, x: 1, ease: 'none', onUpdate: drawPulse, scrollTrigger: { trigger: '.phil', start: 'top 70%', end: 'bottom 30%', scrub: 0.6 } });
    }

    /* PROCESS: a scroll-driven signal chain. the same line is discovered, defined, created, launched */
    const art = $('.proc__art');
    const states = JSON.parse(art.dataset.states);
    const mPath = $('.m', art), sPath = $('.s', art), rings = $$('.ring', art), steps = $$('.pstep'), bar = $('.rail__bar i');
    const setStep = (i) => steps.forEach((s, k) => { s.classList.toggle('is-active', k === i); s.classList.toggle('is-done', k < i); });
    if (desk) {
      G.set(mPath, { attr: { d: states.m[0] } }); G.set(sPath, { attr: { d: states.s[0] } });
      const tl = G.timeline({ defaults: { ease: 'power2.inOut', duration: 1 }, scrollTrigger: {
        trigger: '.proc__pin', start: 'top top', end: '+=260%', pin: true, scrub: 0.8, anticipatePin: 1, snap: harness ? false : { snapTo: 'labelsDirectional', duration: { min: 0.2, max: 0.6 }, delay: 0.15, ease: 'power1.inOut' },
        onToggle: (s) => $('.proc').classList.toggle('is-pinned', s.isActive),
        onUpdate: (s) => setStep(Math.min(3, Math.floor(s.progress * 3.999))) } });
      window.__ks.proc = tl;
      setStep(0);
      tl.addLabel('st0').fromTo(bar, { '--rp': 0.04 }, { '--rp': 0.04, duration: 0.3 });
      for (let i = 1; i < 4; i++) {
        tl.addLabel(`st${i}`)
          .to(mPath, { attr: { d: states.m[i] } }, `st${i}`)
          .to(sPath, { attr: { d: states.s[i] } }, `st${i}`)
          .to('.proc__track', { yPercent: -25 * i, duration: 0.3, ease: 'power3.inOut' }, `st${i}`)
          .to(bar, { '--rp': [0.04, 0.37, 0.7, 1][i] }, `st${i}`)
          .addLabel(`hold${i}`)
          .to({}, { duration: 0.35 });
      }
      tl.fromTo(rings, { scale: 0.2, autoAlpha: 0, svgOrigin: '960 150' }, { scale: 1, autoAlpha: 1, stagger: 0.08, duration: 0.6 }, 'st3+=0.3').addLabel('end');
    } else {
      setStep(-1);
      G.fromTo(bar, { '--rp': 0 }, { '--rp': 1, ease: 'none', scrollTrigger: { trigger: '.rail', start: 'top 70%', end: 'bottom 60%', scrub: 0.5 } });
      steps.forEach((s, i) => ST.create({ trigger: s, start: 'top 65%', onEnter: () => setStep(i), onLeaveBack: () => setStep(i - 1) }));
    }
    if (SPLIT) SPLIT.create('#proc-h', { type: 'lines', mask: 'lines', onSplit: (s) => G.from(s.lines, { yPercent: 100, rotateX: -55, transformPerspective: 700, transformOrigin: '50% 100%', duration: 1, ease: 'expo.out', stagger: 0.1, scrollTrigger: { trigger: '#proc-h', start: 'top 85%', once: true } }) });

    /* SHOWREEL: the stage expands, the heading slides in from both sides, the outline marquee follows scroll speed */
    G.fromTo('#reel-frame', { clipPath: desk ? 'inset(7% 9% 7% 9% round 28px)' : 'inset(4% 5% 4% 5% round 18px)' }, { clipPath: 'inset(0% 0% 0% 0% round 22px)', ease: 'none', scrollTrigger: { trigger: '#reel-frame', start: 'top 95%', end: 'top 25%', scrub: 0.6 } });
    const rl = $$('.rl');
    G.fromTo(rl[0], { xPercent: -14 }, { xPercent: 0, ease: 'none', scrollTrigger: { trigger: '.reel__head', start: 'top bottom', end: 'top 30%', scrub: 0.6 } });
    G.fromTo(rl[1], { xPercent: 14 }, { xPercent: 0, ease: 'none', scrollTrigger: { trigger: '.reel__head', start: 'top bottom', end: 'top 30%', scrub: 0.6 } });
    const mq = $('.marquee__in'); let mx = 0, dir = 1;
    const mqTick = (t, dms) => {
      const v = sig.v; if (Math.abs(v) > 20) dir = v > 0 ? 1 : -1;
      mx -= dms / 1000 * (40 + Math.min(Math.abs(v), 3000) * 0.12) * dir;
      const w = mq.scrollWidth / 2; if (w) { if (mx < -w) mx += w; if (mx > 0) mx -= w; }
      mq.style.transform = `translate3d(${mx.toFixed(1)}px,0,0)`;
    };
    ST.create({ trigger: '.reel', start: 'top bottom', end: 'bottom top', onToggle: (s) => s.isActive ? G.ticker.add(mqTick) : G.ticker.remove(mqTick) });

    /* INSIGHTS: words of the heading stagger in, rows draw their rules */
    if (SPLIT) SPLIT.create('#ins-h', { type: 'words', onSplit: (s) => G.from(s.words, { y: 30, autoAlpha: 0, duration: 0.7, stagger: 0.06, scrollTrigger: { trigger: '#ins-h', start: 'top 88%', once: true } }) });
    G.from('.ins', { y: 32, ease: 'expo.out', duration: 0.7, stagger: 0.1, scrollTrigger: { trigger: '.ins-list', start: 'top 85%', once: true } });

    /* FINALE: cinematic char stagger, the cobalt ribbon crosses behind the type */
    if (SPLIT) SPLIT.create('.fl', { type: 'chars', onSplit: (s) => G.from(s.chars, { yPercent: 110, duration: 1.1, ease: 'expo.out', stagger: { each: 0.035, from: 'start' }, scrollTrigger: { trigger: '.finale__h', start: 'top 85%', once: true } }) });
    G.fromTo('.finale__ribbon-g', { xPercent: -30 }, { xPercent: 8, ease: 'none', scrollTrigger: { trigger: '.finale', start: 'top bottom', end: 'bottom top', scrub: 0.8 } });
    G.fromTo('.finale__ribbon', { clipPath: 'inset(0% 100% 0% 0%)' }, { clipPath: 'inset(0% 0% 0% 0%)', ease: 'none', scrollTrigger: { trigger: '.finale__stage', start: 'top 90%', end: 'center 45%', scrub: 0.6 } });
    G.from('.finale__left > :not(.magnetic)', { y: 30, ease: 'expo.out', duration: 0.7, stagger: 0.08, scrollTrigger: { trigger: '.finale__grid', start: 'top 85%', once: true } });
    G.from('.form', { y: 60, duration: 1, ease: 'expo.out', scrollTrigger: { trigger: '.finale__grid', start: 'top 85%', once: true } });

    /* FOOTER: the wordmark letters rise; the trace lands on the last letter */
    if (SPLIT) SPLIT.create('.foot__word', { type: 'chars', charsClass: 'ch', onSplit: (s) => G.from(s.chars, { yPercent: 60, autoAlpha: 0, duration: 0.9, ease: 'expo.out', stagger: 0.03, scrollTrigger: { trigger: '.foot__mark', start: 'top 95%', once: true } }) });
    const fp = $('#foot-path'), fd = $('#foot-dot'); const ft = { a: 1 };
    const drawFoot = () => {
      let d = '';
      for (let i = 0; i <= 160; i++) { const u = i / 160; const y = 30 - Math.sin(TAU * 14 * u) * 24 * ft.a * (1 - u * 0.85); d += (i ? ' L' : 'M') + (u * 1440).toFixed(1) + ',' + y.toFixed(1); }
      fp.setAttribute('d', d);
    };
    drawFoot();
    G.fromTo(ft, { a: 1 }, { a: 0.15, ease: 'none', onUpdate: drawFoot, scrollTrigger: { trigger: '.foot', start: 'top bottom', end: 'bottom bottom', scrub: 0.6 } });

    return () => { ribbonApi.heroP = 0; };
  });

  /* page-long thread: progress, velocity, act */
  ST.create({ start: 0, end: 'max', onUpdate: (s) => { sig.p = s.progress; sig.v = s.getVelocity(); } });
  $$('main > section').forEach((sec) => {
    ST.create({ trigger: sec, start: 'top 55%', end: 'bottom 55%', onToggle: (s) => s.isActive && setAct(+sec.dataset.act || 0) });
  });
  const navLinks = $$('.nav a');
  ['work', 'services', 'about', 'insights'].forEach((id) => {
    const sec = $('#' + id); if (!sec) return;
    ST.create({ trigger: sec, start: 'top 50%', end: 'bottom 50%', onToggle: (s) => navLinks.forEach((l) => l.classList.toggle('is-here', s.isActive && l.getAttribute('href') === '#' + id)) });
  });

  /* waveform edges: the boundary between two fields carries the signal at the current act's amplitude */
  $$('.edge').forEach((edge) => {
    const path = $('path', edge), A = +edge.dataset.amp, K = +edge.dataset.k;
    const draw = (p) => {
      const amp = Math.sin(p * Math.PI) * A * 112;
      let d = 'M0,120';
      for (let i = 0; i <= 48; i++) {
        const u = i / 48, x = u * 1440;
        const y = 120 - amp * (0.5 + 0.5 * Math.sin(TAU * K * u + p * 7)) * (0.55 + 0.45 * Math.sin(Math.PI * u));
        d += ` L${x.toFixed(1)},${y.toFixed(1)}`;
      }
      path.setAttribute('d', d + ' L1440,120 Z');
    };
    if (!reduce) ST.create({ trigger: edge, start: 'top bottom', end: 'bottom top', onUpdate: (s) => draw(s.progress) });
  });

  document.addEventListener('focusin', (e) => { for (let n = e.target; n && n !== document.body; n = n.parentElement) G.getTweensOf(n).forEach((t) => { const owner = t.scrollTrigger ? t : t.parent; const st = owner && owner.scrollTrigger; if (st && st.vars.once) owner.progress(1); }); });
  addEventListener('load', () => ST.refresh());
  document.fonts && document.fonts.ready.then(() => ST.refresh());
}

/* ---------- services: one active discipline, a stage that changes with it ---------- */
const svcs = $$('.svc'), visuals = $$('.v'), stageI = $('#stage-i'), stageN = $('#stage-n'), progs = $$('.stage__prog i');
const SVC_NAMES = ['Identity', 'Web', 'Growth', 'Motion'];
const loops = [];
let activeSvc = -1;
const deskQ = matchMedia('(min-width: 960px)');
function buildLoops() {
  if (!G) return;
  const v1 = visuals[0], v2 = visuals[1], v3 = visuals[2];
  // brand: construction draws, glyph weight breathes, palette chips set
  loops[0] = G.timeline({ paused: true, repeat: -1, repeatDelay: 0.4, defaults: { ease: 'power3.inOut' } })
    .fromTo($$('.v1-c1, .v1-c2', v1), { attr: { 'stroke-dashoffset': 1 } }, { attr: { 'stroke-dashoffset': 0 }, duration: 1.2, stagger: 0.2 })
    .fromTo($('.v1-base', v1), { scaleX: 0, transformOrigin: '0 0' }, { scaleX: 1, duration: 0.8 }, 0.2)
    .fromTo($('.v1-g', v1), { attr: { 'font-weight': 300, 'letter-spacing': 20 } }, { attr: { 'font-weight': 700, 'letter-spacing': -14 }, duration: 1.4 }, 0.3)
    .fromTo($$('.v1-chips rect', v1), { y: 40, opacity: 0 }, { y: 0, opacity: 1, duration: 0.6, stagger: 0.08 }, 0.8)
    .to($('.v1-g', v1), { attr: { 'font-weight': 400 }, duration: 1.2 }, '+=0.8')
    .to($$('.v1-chips rect', v1), { y: -8, duration: 0.5, stagger: { each: 0.06, yoyo: true, repeat: 1 } }, '<');
  // web: one page, three layouts
  const L = [
    [[64, 100, 250, 26], [64, 136, 180, 26], [64, 186, 392, 150], [64, 350, 190, 88], [266, 350, 190, 88]],
    [[270, 100, 186, 26], [270, 136, 140, 26], [64, 100, 190, 338], [270, 186, 186, 120], [270, 318, 186, 120]],
    [[110, 104, 300, 30], [160, 146, 200, 18], [64, 190, 120, 248], [200, 190, 120, 248], [336, 190, 120, 248]],
  ];
  const bl = $$('.b', v2);
  const t2 = G.timeline({ paused: true, repeat: -1, defaults: { duration: 0.9, ease: 'power3.inOut' } });
  [1, 2, 0].forEach((li) => { t2.to({}, { duration: 1.1 }); bl.forEach((b, k) => t2.to(b, { attr: { x: L[li][k][0], y: L[li][k][1], width: L[li][k][2], height: L[li][k][3] } }, '<' + (k ? 0.05 : 0))); });
  loops[1] = t2;
  // growth: signals draw, the lime cursor rides the strongest line, response bars rise
  const mk = $('#mk-a'), dot = $('.mk-dot', v3), ml = mk.getTotalLength();
  const ride = { u: 0 };
  loops[2] = G.timeline({ paused: true, repeat: -1, repeatDelay: 0.6 })
    .fromTo($$('.mk', v3), { attr: { 'stroke-dashoffset': 1 } }, { attr: { 'stroke-dashoffset': 0 }, duration: 1.6, stagger: 0.18, ease: 'power2.inOut' })
    .fromTo(ride, { u: 0 }, { u: 1, duration: 2.6, ease: 'power1.inOut', onUpdate: () => { const pt = mk.getPointAtLength(ride.u * ml); dot.setAttribute('cx', pt.x); dot.setAttribute('cy', pt.y); } }, 0.3)
    .fromTo($$('.mk-bars rect', v3), { scaleY: 0, transformOrigin: '50% 100%' }, { scaleY: 1, duration: 0.7, stagger: 0.05, ease: 'power3.out' }, 0.5);
  // motion: lissajous ribbons on canvas
  const cv = $('.v4'), c = cv.getContext('2d');
  let on = false, t4 = 0, lastT = 0;
  const draw4 = (now) => {
    if (!on) return;
    const dt = lastT ? (now - lastT) / 1000 : 0; lastT = now; t4 += dt;
    c.clearRect(0, 0, 520, 520);
    for (let k = 0; k < 26; k++) {
      c.beginPath();
      const ph = t4 * 0.6 + k * 0.045;
      for (let i = 0; i <= 220; i++) {
        const a = i / 220 * TAU;
        const x = 260 + Math.sin(a * 2 + ph) * 190 * (0.7 + 0.3 * Math.cos(a + t4 * 0.3));
        const y = 250 + Math.sin(a * 3 + ph * 0.7) * 150;
        i ? c.lineTo(x, y) : c.moveTo(x, y);
      }
      c.strokeStyle = k === 13 ? '#D8FF57' : `rgba(${k < 13 ? '72,101,255' : '169,155,255'},${0.18 + 0.5 * (1 - Math.abs(k - 13) / 13)})`;
      c.lineWidth = k === 13 ? 2 : 1.2; c.stroke();
    }
    requestAnimationFrame(draw4);
  };
  loops[3] = { play() { if (on) return; on = true; lastT = 0; requestAnimationFrame(draw4); }, pause() { on = false; }, progress() {} };
  if (reduce) { loops.forEach((l) => l.progress && l.progress(0.6)); on = true; draw4(performance.now()); on = false; }
}
buildLoops();
function setSvc(i, fromScroll) {
  if (i === activeSvc) return;
  const prev = activeSvc; activeSvc = i;
  svcs.forEach((s, k) => { s.classList.toggle('is-active', k === i); $('.svc__t', s).setAttribute('aria-expanded', String(k === i)); });
  if (stageI) { stageI.textContent = '0' + (i + 1); stageN.textContent = SVC_NAMES[i]; }
  progs.forEach((p, k) => p.style.setProperty('--p', k <= i ? 1 : 0));
  if (deskQ.matches) {
    visuals.forEach((v, k) => v.classList.toggle('is-on', k === i));
    if (M && !reduce) {
      if (prev > -1) M.animate(visuals[prev], { opacity: [1, 0], transform: ['translateY(0px) scale(1)', 'translateY(-24px) scale(.96)'] }, { duration: 0.35, easing: [0.23, 1, 0.32, 1] });
      M.animate(visuals[i], { opacity: [0, 1], transform: ['translateY(28px) scale(1.04)', 'translateY(0px) scale(1)'] }, { duration: 0.6, delay: 0.08, easing: [0.23, 1, 0.32, 1] });
    }
  }
  if (!reduce) loops.forEach((l, k) => l && (k === i ? l.play() : l.pause()));
}
function svcMode() {
  if (deskQ.matches) {
    visuals.forEach((v) => $('#stage').insertBefore(v, $('.stage__prog')));
    activeSvc = -1; setSvc(Math.max(0, activeSvc), false);
  } else {
    visuals.forEach((v, k) => $(`.svc__visual[data-slot="${k}"]`).appendChild(v));
    activeSvc = -1; setSvc(0);
  }
}
svcMode(); deskQ.addEventListener('change', svcMode);
svcs.forEach((s, i) => {
  $('.svc__t', s).addEventListener('click', () => {
    if (deskQ.matches) {
      const tr = $('.svc-track'), top = tr.getBoundingClientRect().top + scrollY - 60, len = tr.offsetHeight - (innerHeight - 60);
      const y = top + len * (i + 0.5) / 4;
      window.__ks && window.__ks.lenis ? window.__ks.lenis.scrollTo(y, { duration: 1 }) : scrollTo({ top: y, behavior: reduce ? 'auto' : 'smooth' });
      setSvc(i);
    } else {
      setSvc(activeSvc === i ? -1 : i);
      if (activeSvc === -1) { svcs.forEach((x) => { x.classList.remove('is-active'); $('.svc__t', x).setAttribute('aria-expanded', 'false'); }); loops.forEach((l) => l && l.pause()); }
    }
  });
});
if (ST) ST.create({ trigger: '.svc-track', start: 'top 60px', end: 'bottom bottom', onUpdate: (st) => deskQ.matches && setSvc(Math.min(3, Math.floor(st.progress * 4))) });

/* ---------- disclosures (project outlines, essay abstracts) ---------- */
function disclosure(btns, panel) {
  const open = btns[0].getAttribute('aria-expanded') !== 'true';
  btns.forEach((b) => b.getAttribute('aria-expanded') !== null && b.setAttribute('aria-expanded', String(open)));
  panel.classList.toggle('is-open', open);
  if (open && M && !reduce) M.animate(panel.firstElementChild.children, { opacity: [0, 1], transform: ['translateY(14px)', 'translateY(0px)'] }, { duration: 0.45, delay: M.stagger(0.06), easing: [0.23, 1, 0.32, 1] });
  setTimeout(() => ST && ST.refresh(), 450);
}
$$('.proj').forEach((p) => {
  const btn = $('.proj__view', p), media = $('.proj__media', p), panel = $('.proj__more', p);
  [btn, media].forEach((el) => el.addEventListener('click', () => disclosure([btn, media], panel)));
});
$$('.ins').forEach((row) => { const b = $('.ins__btn', row); b.addEventListener('click', () => disclosure([b], $('.ins__more', row))); });

/* ---------- media parallax + insight hover previews (fine pointers only) ---------- */
if (fine && !reduce && G) {
  $$('.proj__media').forEach((m) => {
    const layers = $$('.pl', m).map((l) => ({ d: +l.dataset.depth || 1, x: G.quickTo(l, 'x', { duration: 0.8, ease: 'power3' }), y: G.quickTo(l, 'y', { duration: 0.8, ease: 'power3' }) }));
    m.addEventListener('pointermove', (e) => { const r = m.getBoundingClientRect(); const px = (e.clientX - r.left) / r.width - 0.5, py = (e.clientY - r.top) / r.height - 0.5; layers.forEach((l) => { l.x(-px * 26 * l.d); l.y(-py * 18 * l.d); }); });
    m.addEventListener('pointerleave', () => layers.forEach((l) => { l.x(0); l.y(0); }));
  });
  const fl = $('.ins-float'); const fx = G.quickTo(fl, 'left', { duration: 0.5, ease: 'power3' }), fy = G.quickTo(fl, 'top', { duration: 0.5, ease: 'power3' });
  $$('.ins').forEach((row) => {
    const art = $('.ins__art', row).innerHTML;
    row.addEventListener('pointerenter', (e) => { fl.innerHTML = art; G.set(fl, { display: 'block', left: e.clientX, top: e.clientY }); G.to(fl, { opacity: 1, scale: 1, rotate: -3, duration: 0.35, ease: 'power3.out' }); });
    row.addEventListener('pointermove', (e) => { fx(e.clientX + 170); fy(e.clientY - 40); });
    row.addEventListener('pointerleave', () => G.to(fl, { opacity: 0, scale: 0.9, rotate: 0, duration: 0.25, ease: 'power2.out', onComplete: () => G.set(fl, { display: 'none' }) }));
  });
}

/* ---------- custom cursor + magnetic controls ---------- */
if (fine && G) {
  root.classList.add('has-cursor');
  const cur = $('.cur'), dot = $('.cur__dot'), ring = $('.cur__ring');
  const dx = G.quickTo(dot, 'x', { duration: reduce ? 0.01 : 0.12, ease: 'power3' }), dy = G.quickTo(dot, 'y', { duration: reduce ? 0.01 : 0.12, ease: 'power3' });
  const rx = G.quickTo(ring, 'x', { duration: reduce ? 0.01 : 0.45, ease: 'power3' }), ry = G.quickTo(ring, 'y', { duration: reduce ? 0.01 : 0.45, ease: 'power3' });
  addEventListener('pointermove', (e) => { cur.classList.add('is-live'); dx(e.clientX); dy(e.clientY); rx(e.clientX); ry(e.clientY); }, { passive: true });
  document.addEventListener('pointerover', (e) => {
    const t = e.target;
    cur.classList.toggle('is-view', !!t.closest('.proj__media'));
    cur.classList.toggle('is-link', !t.closest('.proj__media') && !!t.closest('a, button, label, input, textarea'));
    cur.classList.toggle('is-loud', !!t.closest('.finale'));
  });
  document.addEventListener('pointerleave', () => cur.classList.remove('is-live'));
  
  if (!reduce) $$('.magnetic').forEach((m) => {
    const strength = m.closest('.finale') ? 0.42 : 0.28;
    const mx = G.quickTo(m, 'x', { duration: 0.5, ease: 'power3' }), my = G.quickTo(m, 'y', { duration: 0.5, ease: 'power3' });
    m.addEventListener('pointermove', (e) => { const r = m.getBoundingClientRect(); mx((e.clientX - r.left - r.width / 2) * strength); my((e.clientY - r.top - r.height / 2) * strength); });
    m.addEventListener('pointerleave', () => { mx(0); my(0); });
  });
}

/* ---------- mobile menu (Motion for presence) ---------- */
const menu = $('#menu'), mbtn = $('#menu-btn');
let lastFocus = null;
function openMenu() {
  lastFocus = document.activeElement;
  menu.hidden = false; mbtn.setAttribute('aria-expanded', 'true'); mbtn.setAttribute('aria-label', 'Close menu');
  document.body.style.overflow = 'hidden'; window.__ks && window.__ks.lenis && window.__ks.lenis.stop();
  if (M && !reduce) {
    M.animate(menu, { clipPath: ['circle(0% at calc(100% - 44px) 33px)', 'circle(150% at calc(100% - 44px) 33px)'] }, { duration: 0.6, easing: [0.32, 0.72, 0, 1] });
    M.animate($$('.menu__list a'), { opacity: [0, 1], transform: ['translateY(40px)', 'translateY(0px)'] }, { duration: 0.5, delay: M.stagger(0.06, { startDelay: 0.12 }), easing: [0.23, 1, 0.32, 1] });
    M.animate($('.menu__foot'), { opacity: [0, 1] }, { duration: 0.4, delay: 0.35 });
  }
  setTimeout(() => $('.menu__list a').focus(), 60);
}
function closeMenu(restore = true) {
  if (menu.hidden) return;
  const done = () => { menu.hidden = true; };
  mbtn.setAttribute('aria-expanded', 'false'); mbtn.setAttribute('aria-label', 'Open menu');
  document.body.style.overflow = ''; window.__ks && window.__ks.lenis && window.__ks.lenis.start();
  if (M && !reduce) M.animate(menu, { opacity: [1, 0] }, { duration: 0.25 }).then(() => { done(); menu.style.opacity = ''; });
  else done();
  if (restore) mbtn.focus();
}
mbtn.addEventListener('click', () => (menu.hidden ? openMenu() : closeMenu()));
document.addEventListener('keydown', (e) => {
  if (menu.hidden) return;
  if (e.key === 'Escape') closeMenu();
  if (e.key === 'Tab') {
    const f = [mbtn, ...$$('a, button', menu)];
    const i = f.indexOf(document.activeElement);
    if (e.shiftKey && i <= 0) { e.preventDefault(); f[f.length - 1].focus(); }
    else if (!e.shiftKey && i === f.length - 1) { e.preventDefault(); f[0].focus(); }
  }
});
if (!G) $$('#menu a').forEach((a) => a.addEventListener('click', () => closeMenu(false)));

/* ---------- showreel: coded placeholder, muted, paused offscreen ---------- */
(function reel() {
  const cv = $('#reel'), c = cv.getContext('2d'), btn = $('#reel-play'), bt = $('#reel-play-t'), sceneEl = $('#reel-scene'), tcEl = $('#reel-tc'), bars = $$('.reel__bar b');
  const SC = ['Ribbon', 'Type', 'Field', 'Tunnel'], LEN = 4;
  let W = 0, H = 0, t = 0, playing = !reduce, visible = false, last = 0, raf = 0;
  const size = () => { const d = Math.min(devicePixelRatio, 1.5); W = cv.clientWidth; H = cv.clientHeight; cv.width = W * d; cv.height = H * d; c.setTransform(d, 0, 0, d, 0, 0); };
  size(); addEventListener('resize', () => { size(); draw(); });
  const WORDS = ['Idea', 'Signal', 'Form', 'Motion'];
  function ribbonScene(lt) {
    c.fillStyle = '#05060A'; c.fillRect(0, 0, W, H);
    const n = 160, cy = H * 0.5;
    let prev = null;
    for (let i = 0; i <= n; i++) {
      const u = i / n, x = -W * 0.05 + u * W * 1.1;
      const y = cy + Math.sin(TAU * (u * 1.3) + lt * 0.8) * H * 0.2 + Math.sin(TAU * u * 3.1 - lt) * H * 0.04;
      const th = 3.4 * Math.PI * u + lt * 1.2, w = (H * 0.11) * Math.sin(Math.PI * u) + 4, h = w * Math.cos(th);
      const cur = [x, y - h, x, y + h];
      if (prev) {
        const shade = 0.5 + 0.5 * Math.sin(th + 0.8), front = Math.cos(th) >= 0;
        const l = front ? 35 + shade * 58 : 22 + shade * 30;
        c.fillStyle = front ? `hsl(${222 + (1 - shade) * 8},${18 + (1 - shade) * 60}%,${l}%)` : `hsl(229,${70}%,${l}%)`;
        c.beginPath(); c.moveTo(prev[0], prev[1]); c.lineTo(cur[0], cur[1]); c.lineTo(cur[2], cur[3]); c.lineTo(prev[2], prev[3]); c.closePath(); c.fill();
        c.strokeStyle = c.fillStyle; c.lineWidth = 0.8; c.stroke();
      }
      prev = cur;
    }
  }
  function typeScene(lt) {
    c.fillStyle = '#4865FF'; c.fillRect(0, 0, W, H);
    const i = Math.floor(lt / 1) % WORDS.length, f = lt % 1;
    const e = 1 - Math.pow(1 - Math.min(1, f / 0.5), 3);
    c.save(); c.fillStyle = '#F6F5F0'; c.textBaseline = 'middle'; c.textAlign = 'center';
    const fs = Math.min(W * 0.22, H * 0.5);
    c.font = `600 ${fs}px "Space Grotesk", sans-serif`;
    if ('letterSpacing' in c) c.letterSpacing = `${(-0.05 + (1 - e) * 0.25) * fs}px`;
    c.globalAlpha = Math.min(1, f / 0.15) * (f > 0.85 ? (1 - f) / 0.15 : 1);
    c.fillText(WORDS[i].toUpperCase(), W / 2, H / 2 + (1 - e) * H * 0.08);
    c.restore();
    c.strokeStyle = 'rgba(246,245,240,.5)'; c.lineWidth = 1.5; c.beginPath();
    for (let x = 0; x <= W; x += 6) { const y = H * 0.82 + Math.sin(x / W * TAU * 6 - lt * 4) * 10; x ? c.lineTo(x, y) : c.moveTo(x, y); }
    c.stroke();
  }
  function fieldScene(lt) {
    c.fillStyle = '#090B11'; c.fillRect(0, 0, W, H);
    const rows = 34;
    for (let r = 0; r < rows; r++) {
      const y0 = H * 0.12 + r * (H * 0.8 / rows);
      c.beginPath();
      for (let x = 0; x <= W; x += 8) {
        const u = x / W, bump = Math.exp(-Math.pow((u - 0.5 - 0.2 * Math.sin(lt * 0.7)) / 0.16, 2)) * H * 0.12 * Math.sin(Math.PI * r / rows);
        const y = y0 - bump * (1 + 0.3 * Math.sin(lt * 2 + r * 0.3)) + Math.sin(u * 14 + lt * 2 + r * 0.4) * 3;
        x ? c.lineTo(x, y) : c.moveTo(x, y);
      }
      c.strokeStyle = r === 17 ? '#D8FF57' : `rgba(246,245,240,${0.15 + 0.55 * Math.sin(Math.PI * r / rows)})`;
      c.lineWidth = r === 17 ? 2 : 1.1; c.stroke();
    }
  }
  function tunnelScene(lt) {
    c.fillStyle = '#05060A'; c.fillRect(0, 0, W, H);
    for (let k = 0; k < 30; k++) {
      const z = ((k / 30) + lt * 0.12) % 1, r = Math.pow(z, 2.2) * Math.max(W, H) * 0.9 + 6;
      c.beginPath();
      for (let i = 0; i <= 90; i++) {
        const a = i / 90 * TAU, rr = r * (1 + 0.08 * Math.sin(3 * a + lt + k * 0.4));
        const x = W / 2 + Math.cos(a + lt * 0.2) * rr * 1.25, y = H / 2 + Math.sin(a + lt * 0.2) * rr * 0.75;
        i ? c.lineTo(x, y) : c.moveTo(x, y);
      }
      c.strokeStyle = z > 0.55 ? `rgba(169,155,255,${1 - z})` : `rgba(72,101,255,${0.3 + z})`;
      c.lineWidth = 1 + z * 2; c.stroke();
    }
  }
  const scenes = [ribbonScene, typeScene, fieldScene, tunnelScene];
  function draw() {
    if (!W) return;
    const total = LEN * 4, tt = t % total, s = Math.floor(tt / LEN), lt = tt - s * LEN;
    scenes[s](t);
    if (lt > LEN - 0.4) { c.globalAlpha = (lt - (LEN - 0.4)) / 0.4; scenes[(s + 1) % 4](t); c.globalAlpha = 1; }
    sceneEl.textContent = `0${s + 1} ${SC[s]}`;
    const sec = Math.floor(tt); tcEl.textContent = `00:${String(sec).padStart(2, '0')}`;
    bars.forEach((b, k) => { b.style.transform = `scaleX(${k < s ? 1 : k === s ? lt / LEN : 0})`; });
  }
  function loop(now) { if (!playing || !visible || document.hidden) { raf = 0; return; } const dt = last ? Math.min(0.05, (now - last) / 1000) : 0; last = now; t += dt; draw(); raf = requestAnimationFrame(loop); }
  function kick() { if (!raf && playing && visible && !document.hidden) { last = 0; raf = requestAnimationFrame(loop); } }
  function setPlaying(p) {
    playing = p; btn.setAttribute('aria-pressed', String(p)); bt.textContent = p ? 'Pause reel' : 'Play reel';
    $('.ic-pause', btn).style.display = p ? '' : 'none'; $('.ic-play', btn).style.display = p ? 'none' : '';
    kick();
  }
  btn.addEventListener('click', () => setPlaying(!playing));
  new IntersectionObserver(([e]) => { visible = e.isIntersecting; kick(); }, { threshold: 0.05 }).observe(cv);
  document.addEventListener('visibilitychange', kick);
  if (document.fonts) document.fonts.ready.then(draw);
  t = reduce ? 1.6 : 0; draw(); setPlaying(playing);
})();

/* ---------- contact form: validated, then handed to the visitor's mail app ---------- */
(function form() {
  const f = $('#brief'), ok = $('#form-ok'), fields = $('.form__fields');
  const name = $('#f-name'), mail = $('#f-mail');
  const err = (inp, el, msg) => { $(el).textContent = msg; inp.setAttribute('aria-invalid', msg ? 'true' : 'false'); };
  f.addEventListener('submit', (e) => {
    e.preventDefault();
    let bad = null;
    if (!name.value.trim()) { err(name, '#e-name', 'Add your name so we know who to reply to.'); bad = bad || name; } else err(name, '#e-name', '');
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(mail.value.trim())) { err(mail, '#e-mail', 'Add an email address we can reply to, like name@company.com.'); bad = bad || mail; } else err(mail, '#e-mail', '');
    if (bad) {
      bad.focus();
      if (M && !reduce) M.animate(bad.closest('.field'), { transform: ['translateX(0px)', 'translateX(-4px)', 'translateX(4px)', 'translateX(0px)'] }, { duration: 0.3 });
      return;
    }
    const need = (new FormData(f).get('need')) || '';
    const body = `Name: ${name.value.trim()}\nEmail: ${mail.value.trim()}\nNeed: ${need}\n\n${$('#f-msg').value.trim()}`;
    const href = `mailto:hello@yourstudio.example?subject=${encodeURIComponent('New project brief: ' + need)}&body=${encodeURIComponent(body)}`;
    fields.hidden = true; ok.hidden = false; ok.focus();
    // success: the brief leaves as a transmitted waveform
    const p = $('#ok-path'); const s = { a: 0 };
    const drawOk = () => { let d = ''; for (let i = 0; i <= 120; i++) { const u = i / 120; const y = 35 - Math.sin(TAU * 8 * u) * 26 * s.a * Math.sin(Math.PI * u); d += (i ? ' L' : 'M') + (u * 600).toFixed(1) + ',' + y.toFixed(1); } p.setAttribute('d', d); };
    if (G && !reduce) G.to(s, { a: 1, duration: 0.9, ease: 'expo.out', onUpdate: drawOk }); else { s.a = 1; drawOk(); }
    sig.pulse = 3;
    window.location.href = href;
  });
})();

/* ---------- reduced motion: static, complete frames ---------- */
if (reduce && G) {
  // thread still reflects position, without travel
  const sp = $('#scope-path'); if (sp) { let d = ''; for (let i = 0; i <= 120; i++) { const x = i * 5, u = x / 600, g = u < 0.25 ? 0 : Math.min(1, (u - 0.25) / 0.2); d += (i ? ' L' : 'M') + x + ',' + (100 - Math.sin(TAU * 3.2 * (u - 0.25)) * 70 * g).toFixed(1); } sp.setAttribute('d', d); }
}
