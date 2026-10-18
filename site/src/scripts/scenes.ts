import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);

/** Problem: pin the section on wide screens and move the plates sideways with the scroll. */
function problemScene() {
  const sec = document.querySelector<HTMLElement>('[data-problem]');
  if (!sec) return;
  const track = sec.querySelector<HTMLElement>('[data-track]')!;
  const plates = [...sec.querySelectorAll<HTMLElement>('[data-plate]')];
  const count = sec.querySelector<HTMLElement>('[data-count]');
  const bar = sec.querySelector<HTMLElement>('[data-bar]');
  const setActive = (i: number) => {
    plates.forEach((p, j) => p.classList.toggle('on', i === j));
    if (count) count.textContent = String(i + 1).padStart(2, '0');
    if (bar) bar.style.width = `${((i + 1) / plates.length) * 100}%`;
  };
  sec.setAttribute('data-pinned', '');
  setActive(0);
  const dist = () => Math.max(0, track.scrollWidth - window.innerWidth);
  gsap.to(track, {
    x: () => -dist(),
    ease: 'none',
    scrollTrigger: {
      trigger: sec.querySelector('.pin'),
      pin: true,
      start: 'top top',
      end: () => `+=${dist()}`,
      scrub: 0.5,
      // Rest on a whole plate, never between two.
      snap: { snapTo: 1 / (plates.length - 1), duration: { min: 0.2, max: 0.5 }, delay: 0.08, ease: 'power2.inOut' },
      invalidateOnRefresh: true,
      onUpdate: (st) => setActive(Math.min(plates.length - 1, Math.round(st.progress * (plates.length - 1)))),
    },
  });
  return () => {
    sec.removeAttribute('data-pinned');
    plates.forEach((p) => p.classList.remove('on'));
  };
}

/** Site DNA: each extraction step lights up as it reaches the middle of the screen. */
function dnaScene() {
  const sec = document.querySelector<HTMLElement>('[data-dna]');
  if (!sec) return;
  const steps = [...sec.querySelectorAll<HTMLElement>('[data-step]')];
  sec.setAttribute('data-live', '');
  const on = (i: number) => {
    steps.forEach((s, j) => s.classList.toggle('on', i === j));
    sec.dataset.at = String(i);
  };
  on(0);
  steps.forEach((s, i) =>
    ScrollTrigger.create({ trigger: s, start: 'top 60%', end: 'bottom 60%', onEnter: () => on(i), onEnterBack: () => on(i) }),
  );
  return () => {
    sec.removeAttribute('data-live');
    delete sec.dataset.at;
    steps.forEach((s) => s.classList.remove('on'));
  };
}

/** Workflow: a serpentine path through the stage dots, drawn by the scroll. */
function flowScene() {
  const flow = document.querySelector<HTMLElement>('[data-flow]');
  if (!flow) return;
  const svg = flow.querySelector<SVGSVGElement>('svg')!;
  const base = flow.querySelector<SVGPathElement>('[data-flow-base]')!;
  const line = flow.querySelector<SVGPathElement>('[data-flow-line]')!;
  const items = [...flow.querySelectorAll<HTMLElement>('[data-step]')];
  const dots = items.map((li) => li.querySelector<HTMLElement>('[data-dot]')!);
  let stops: number[] = [];

  const build = () => {
    const box = flow.getBoundingClientRect();
    svg.setAttribute('viewBox', `0 0 ${box.width} ${box.height}`);
    const wide = window.innerWidth > 760;
    const amp = wide ? Math.min(96, box.width * 0.06) : 0;
    dots.forEach((d) => (d.style.transform = ''));
    const pts = dots.map((d, i) => {
      const r = d.getBoundingClientRect();
      // On wide screens the curve swings toward the side each stage's text sits on.
      const x = r.left + r.width / 2 - box.left + (wide ? (i % 2 ? amp : -amp) : 0);
      return { x, y: r.top + r.height / 2 - box.top };
    });
    // Dots ride the curve, so move each one to its point.
    dots.forEach((d, i) => (d.style.transform = `translate(calc(-50% + ${wide ? (i % 2 ? amp : -amp) : 0}px), 0)`));
    let p = `M${pts[0].x} ${pts[0].y}`;
    for (let i = 1; i < pts.length; i++) {
      const a = pts[i - 1], b = pts[i], my = (a.y + b.y) / 2;
      p += ` C${a.x} ${my} ${b.x} ${my} ${b.x} ${b.y}`;
    }
    base.setAttribute('d', p);
    line.setAttribute('d', p);
    const len = line.getTotalLength();
    gsap.set(line, { strokeDasharray: len, strokeDashoffset: len });
    // Path length at each dot, so a dot lights when the ink reaches it.
    stops = pts.map((pt, i) => {
      if (i === 0) return 0;
      let lo = 0, hi = len;
      for (let k = 0; k < 24; k++) { const m = (lo + hi) / 2; line.getPointAtLength(m).y < pt.y ? (lo = m) : (hi = m); }
      return hi / len;
    });
  };

  flow.classList.add('drawn');
  build();
  const st = ScrollTrigger.create({
    trigger: flow,
    start: 'top 65%',
    end: 'bottom 65%',
    scrub: 0.4,
    onRefreshInit: build,
    onUpdate: (s) => {
      const len = line.getTotalLength();
      gsap.set(line, { strokeDashoffset: len * (1 - s.progress) });
      items.forEach((li, i) => li.classList.toggle('lit', s.progress >= stops[i] - 0.01));
    },
  });
  items[0].classList.add('lit');
  return () => {
    st.kill();
    flow.classList.remove('drawn');
    base.removeAttribute('d');
    line.removeAttribute('d');
    dots.forEach((d) => (d.style.transform = ''));
    items.forEach((li) => li.classList.remove('lit'));
  };
}

/** Final: the four logo pieces start scattered and converge into the mark. */
function finalScene() {
  const sec = document.querySelector<HTMLElement>('[data-final]');
  if (!sec) return;
  const frags = sec.querySelectorAll<SVGElement>('[data-frag]');
  const from = [
    { x: -140, y: -90, rotation: -24 },
    { x: 120, y: -40, rotation: 18 },
    { x: -90, y: 80, rotation: 12 },
    { x: 110, y: 110, rotation: -16 },
  ];
  // Smaller scatter on narrow screens so pieces never cross the headline.
  const k = Math.min(1, window.innerWidth / 1200);
  frags.forEach((f, i) => {
    gsap.fromTo(f, { x: from[i].x * k, y: from[i].y * k, rotation: from[i].rotation, opacity: 0.25, transformOrigin: '50% 50%' }, {
      x: 0, y: 0, rotation: 0, opacity: 1, ease: 'power2.out',
      scrollTrigger: { trigger: sec, start: 'top 85%', end: 'center 60%', scrub: 0.6 },
    });
  });
}

/** Starts the scroll scenes; returns their teardown. Loaded lazily by reveal.ts. */
export function startScenes() {
  const ctx = gsap.context(() => {
    // Colophon wordmark rises as the footer arrives.
    const mark = document.querySelector<HTMLElement>('[data-colophon]');
    if (mark) {
      gsap.fromTo(mark, { yPercent: 30 }, {
        yPercent: 0, ease: 'none',
        scrollTrigger: { trigger: mark, start: 'top bottom', end: 'bottom bottom', scrub: true },
      });
    }
  });
  const mm = gsap.matchMedia();
  mm.add('(prefers-reduced-motion: no-preference)', () => {
    const undo = [dnaScene(), flowScene(), finalScene()];
    return () => undo.forEach((u) => u?.());
  });
  // Pinning needs room: short screens get the swipeable row instead.
  mm.add('(min-width: 900px) and (min-height: 620px) and (prefers-reduced-motion: no-preference)', () => {
    const undo = problemScene();
    return () => undo?.();
  });
  let t = 0;
  const onResize = () => { clearTimeout(t); t = window.setTimeout(() => ScrollTrigger.refresh(), 200); };
  window.addEventListener('resize', onResize);
  // Images above a pinned section change its offsets once they load.
  const onLoad = () => ScrollTrigger.refresh();
  if (document.readyState !== 'complete') window.addEventListener('load', onLoad, { once: true });
  return () => {
    window.removeEventListener('resize', onResize);
    window.removeEventListener('load', onLoad);
    clearTimeout(t);
    mm.revert();
    ctx.revert();
  };
}
