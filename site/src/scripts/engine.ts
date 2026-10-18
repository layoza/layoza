import {
  AdditiveBlending, BufferAttribute, BufferGeometry, CanvasTexture, Color, DoubleSide, EdgesGeometry,
  Group, LineBasicMaterial, LineDashedMaterial, LineSegments, Mesh, MeshBasicMaterial, PerspectiveCamera,
  PlaneGeometry, Points, PointsMaterial, Scene, SRGBColorSpace, TextureLoader, Vector3, WebGLRenderer,
  type Texture,
} from 'three';
import { gsap } from 'gsap';

const W = 6.4;
const D = 4; // 16:10, same as the benchmark screenshots
const GAP = 1.25;
const LIME = 0xc8ff61;
const CYAN = 0x6addf0;

function canvasLayer(draw: (g: CanvasRenderingContext2D, w: number, h: number) => void) {
  const c = document.createElement('canvas');
  c.width = 1024;
  c.height = 640;
  const g = c.getContext('2d')!;
  draw(g, c.width, c.height);
  const t = new CanvasTexture(c);
  t.colorSpace = SRGBColorSpace;
  t.anisotropy = 4;
  return t;
}

/** Layer 2 — design DNA: the grid, the type scale and the tokens read from a site. */
function dnaTexture() {
  return canvasLayer((g, w, h) => {
    g.fillStyle = 'rgba(16,21,31,0.62)';
    g.fillRect(0, 0, w, h);
    g.strokeStyle = 'rgba(106,221,240,0.75)';
    g.lineWidth = 2;
    for (let i = 1; i < 6; i++) {
      const x = (w / 6) * i;
      g.beginPath(); g.moveTo(x, 0); g.lineTo(x, h); g.stroke();
    }
    g.fillStyle = '#6ADDF0';
    g.font = '500 30px "IBM Plex Mono", monospace';
    ['--ground   #F4EFE6', '--display  700 / 96', '--grid     12 · 24px', '--accent   #C8FF61'].forEach((s, i) =>
      g.fillText(s, 44, h - 190 + i * 42),
    );
    const sw = ['#F4EFE6', '#141922', '#C8FF61', '#8168FF'];
    sw.forEach((c, i) => { g.fillStyle = c; g.fillRect(w - 300 + i * 66, 48, 54, 54); });
    g.fillStyle = '#F8FAFC';
    g.font = '700 120px "Space Grotesk Variable", sans-serif';
    g.fillText('Aa', 44, 150);
    g.font = '500 26px "IBM Plex Mono", monospace';
    g.fillStyle = '#9CA5B4';
    g.fillText('150 / 96 / 52 / 22 / 17', 210, 136);
  });
}

/** Layer 3 — motion: an easing path and its keyframes. */
function motionTexture() {
  return canvasLayer((g, w, h) => {
    g.clearRect(0, 0, w, h);
    g.strokeStyle = '#C8FF61';
    g.lineWidth = 6;
    g.setLineDash([18, 14]);
    g.beginPath();
    g.moveTo(40, h - 60);
    g.bezierCurveTo(400, h - 60, 420, 70, w - 50, 60);
    g.stroke();
    g.setLineDash([]);
    g.fillStyle = '#C8FF61';
    [[40, h - 60], [300, h - 120], [520, 260], [w - 50, 60]].forEach(([x, y]) => {
      g.beginPath(); g.arc(x, y, 12, 0, Math.PI * 2); g.fill();
    });
    g.strokeStyle = 'rgba(248,250,252,0.5)';
    g.lineWidth = 2;
    for (let i = 0; i <= 20; i++) {
      const x = 40 + ((w - 90) / 20) * i;
      g.beginPath(); g.moveTo(x, h - 24); g.lineTo(x, h - (i % 5 ? 36 : 50)); g.stroke();
    }
  });
}

export function createEngine(stage: HTMLElement) {
  const canvas = stage.querySelector<HTMLCanvasElement>('canvas.gl')!;
  const callouts = [...stage.querySelectorAll<HTMLElement>('[data-layer]')];
  const replay = stage.querySelector<HTMLButtonElement>('[data-replay]');
  const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  const renderer = new WebGLRenderer({ canvas, antialias: true, alpha: true, powerPreference: 'low-power' });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  renderer.outputColorSpace = SRGBColorSpace;

  const scene = new Scene();
  const camera = new PerspectiveCamera(30, 1, 0.1, 100);
  camera.position.set(9.5, 11, 12.5);
  camera.lookAt(0, 2, 0);

  const rig = new Group();
  scene.add(rig);

  const loader = new TextureLoader();
  const textures: Texture[] = [];
  const tex = (t: Texture) => (textures.push(t), t);
  const loads: Promise<unknown>[] = [];
  const img = (url: string) => {
    let t!: Texture;
    loads.push(new Promise((res) => { t = loader.load(url, res, undefined, res); }));
    return tex(t);
  };
  const geo = new PlaneGeometry(W, D);
  const edgeGeo = new EdgesGeometry(geo);

  const specs = [
    { map: img('/hero/before.webp'), color: 0x8a909c, edge: CYAN, opacity: 1 },
    { map: tex(dnaTexture()), color: 0xffffff, edge: CYAN, opacity: 0.92 },
    { map: tex(motionTexture()), color: 0xffffff, edge: CYAN, opacity: 1 },
    { map: img('/hero/after.webp'), color: 0xffffff, edge: LIME, opacity: 1 },
  ];
  textures.forEach((t) => (t.colorSpace = SRGBColorSpace));

  const disposables: { dispose(): void }[] = [geo, edgeGeo];
  const layers = specs.map((s) => {
    const mat = new MeshBasicMaterial({ map: s.map, color: s.color, transparent: true, opacity: s.opacity, side: DoubleSide, depthWrite: false });
    const mesh = new Mesh(geo, mat);
    mesh.rotation.x = -Math.PI / 2;
    const em = new LineBasicMaterial({ color: s.edge, transparent: true, opacity: 0.9 });
    const edge = new LineSegments(edgeGeo, em);
    mesh.add(edge);
    disposables.push(mat, em);
    const layer = new Group();
    layer.add(mesh);
    rig.add(layer);
    return layer;
  });

  // Corner risers: dashed lines tying the sheets together, like a section drawing.
  const riserGeo = new BufferGeometry();
  const corners = [[-W / 2, -D / 2], [W / 2, -D / 2], [W / 2, D / 2], [-W / 2, D / 2]];
  riserGeo.setAttribute('position', new BufferAttribute(new Float32Array(corners.flatMap(([x, z]) => [x, 0, z, x, 1, z])), 3));
  const riserMat = new LineDashedMaterial({ color: CYAN, dashSize: 0.06, gapSize: 0.06, transparent: true, opacity: 0.45 });
  const risers = new LineSegments(riserGeo, riserMat);
  risers.computeLineDistances();
  rig.add(risers);
  disposables.push(riserGeo, riserMat);

  // Particles: the intelligence travelling up through the layers.
  const N = 260;
  const pos = new Float32Array(N * 3);
  const speed = new Float32Array(N);
  for (let i = 0; i < N; i++) {
    pos[i * 3] = (Math.random() - 0.5) * W * 0.92;
    pos[i * 3 + 1] = Math.random() * GAP * 3;
    pos[i * 3 + 2] = (Math.random() - 0.5) * D * 0.92;
    speed[i] = 0.25 + Math.random() * 0.55;
  }
  const pGeo = new BufferGeometry();
  pGeo.setAttribute('position', new BufferAttribute(pos, 3));
  const pMat = new PointsMaterial({ color: new Color(LIME), size: 0.045, transparent: true, opacity: 0.85, blending: AdditiveBlending, depthWrite: false });
  const points = new Points(pGeo, pMat);
  rig.add(points);
  disposables.push(pGeo, pMat);

  // Explode state, driven by GSAP so Replay is one restart().
  const state = { gap: reduced ? GAP : 0, spin: reduced ? 0 : -0.5, particles: reduced ? 0 : 0 };
  const tl = gsap.timeline({ paused: true });
  tl.fromTo(state, { gap: 0, spin: -0.5 }, { gap: GAP, spin: 0, duration: 2.2, ease: 'expo.out' })
    .fromTo(state, { particles: 0 }, { particles: 1, duration: 1 }, 0.9);
  callouts.forEach((c, i) => tl.fromTo(c, { opacity: 0 }, { opacity: 1, duration: 0.4 }, 0.6 + (3 - i) * 0.18));

  const mouse = { x: 0, y: 0, tx: 0, ty: 0 };
  const onMove = (e: PointerEvent) => {
    const r = stage.getBoundingClientRect();
    mouse.tx = ((e.clientX - r.left) / r.width - 0.5) * 2;
    mouse.ty = ((e.clientY - r.top) / r.height - 0.5) * 2;
  };
  if (!reduced) window.addEventListener('pointermove', onMove, { passive: true });

  const v = new Vector3();
  let w = 0, h = 0;
  const resize = () => {
    w = stage.clientWidth;
    h = stage.clientHeight;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.fov = w < 560 ? 36 : 30;
    // Shift the stack left so the label column on the right has clear space.
    camera.setViewOffset(w, h, w * (w < 560 ? 0.2 : 0.21), 0, w, h);
    camera.updateProjectionMatrix();
  };
  const ro = new ResizeObserver(resize);
  ro.observe(stage);
  resize();

  function placeCallouts() {
    rig.updateMatrixWorld();
    // Labels sit in a column at the stage's right edge, each level with its sheet's
    // rightmost corner, pushed apart so they never overlap one another or the sheets.
    const spots: { el: HTMLElement; y: number; x: number }[] = [];
    layers.forEach((layer, i) => {
      const el = callouts.find((c) => c.dataset.layer === String(i + 1));
      if (!el) return;
      let best: { x: number; y: number } | null = null;
      for (const [x, z] of corners) {
        v.set(x, 0, z).applyMatrix4(layer.matrixWorld).project(camera);
        const sx = (v.x * 0.5 + 0.5) * w;
        const sy = (-v.y * 0.5 + 0.5) * h;
        if (!best || sx > best.x) best = { x: sx, y: sy };
      }
      if (best) spots.push({ el, y: best.y, x: best.x });
    });
    spots.sort((a, b) => a.y - b.y);
    let floor = -Infinity;
    for (const s of spots) {
      const eh = s.el.offsetHeight;
      const y = Math.max(s.y - eh / 2, floor);
      floor = y + eh + 10;
      const x = w - s.el.offsetWidth;
      s.el.style.transform = `translate(${Math.round(x)}px, ${Math.round(y)}px)`;
      s.el.style.setProperty('--lead', `${Math.max(0, Math.round(x - s.x - 8))}px`);
    }
  }

  let raf = 0;
  let visible = true;
  let last = performance.now();
  const frame = (now: number) => {
    const dt = Math.min((now - last) / 1000, 0.05);
    last = now;
    mouse.x += (mouse.tx - mouse.x) * 0.06;
    mouse.y += (mouse.ty - mouse.y) * 0.06;
    rig.rotation.y = state.spin + mouse.x * 0.12;
    rig.rotation.x = mouse.y * 0.05;
    layers.forEach((l, i) => (l.position.y = state.gap * i));
    risers.scale.y = Math.max(state.gap * 3, 0.0001);
    points.visible = state.particles > 0.01;
    pMat.opacity = 0.85 * state.particles;
    if (!reduced) {
      const top = state.gap * 3;
      for (let i = 0; i < N; i++) {
        let y = pos[i * 3 + 1] + speed[i] * dt;
        if (y > top) y = 0;
        pos[i * 3 + 1] = y;
      }
      pGeo.attributes.position.needsUpdate = true;
    }
    renderer.render(scene, camera);
    placeCallouts();
    if (!stage.classList.contains('gl-on')) stage.classList.add('gl-on');
    if (visible && !reduced) raf = requestAnimationFrame(frame);
  };

  const io = new IntersectionObserver(([e]) => {
    visible = e.isIntersecting && !document.hidden;
    cancelAnimationFrame(raf);
    if (visible && !reduced) { last = performance.now(); raf = requestAnimationFrame(frame); }
  });
  io.observe(stage);

  let alive = true;
  Promise.all(loads).then(() => {
    if (!alive) return;
    if (reduced) {
      state.gap = GAP; state.spin = 0;
      callouts.forEach((c) => (c.style.opacity = '1'));
      frame(performance.now());
    } else {
      tl.play(0);
      if (replay) {
        replay.hidden = false;
        replay.onclick = () => tl.restart();
      }
    }
  });

  return () => {
    alive = false;
    cancelAnimationFrame(raf);
    tl.kill();
    io.disconnect();
    ro.disconnect();
    window.removeEventListener('pointermove', onMove);
    textures.forEach((t) => t.dispose());
    disposables.forEach((d) => d.dispose());
    renderer.dispose();
    stage.classList.remove('gl-on');
  };
}
