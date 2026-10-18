import { useEffect, useState } from 'react';

type Mech = { id: string; name: string; kind: 'hero' | 'section'; tech: string; use: string; size: 'xl' | 'tall' | 'sm' };

// Six real entries from `layoza mechanics list` (76 in the library).
const MECHS: Mech[] = [
  { id: 'hero-odometer-in-headline', name: 'Odometer Inside the Sentence', kind: 'hero', tech: 'css', size: 'xl', use: 'A real number rolls into place inside the headline.' },
  { id: 'hero-seam-split-ink', name: 'Seam-Split Headline', kind: 'hero', tech: 'js', size: 'sm', use: 'The headline parts along a seam to reveal what is underneath.' },
  { id: 'hero-floor-plan-draw', name: 'Floor Plan That Draws Itself', kind: 'hero', tech: 'js', size: 'tall', use: 'Lines ink in, like a plan being drawn for a builder or an architect.' },
  { id: 'pricing-receipt-printer-slider', name: 'Slider-driven receipt printer', kind: 'section', tech: 'js', size: 'tall', use: 'Move the slider and the receipt prints the price, line by line.' },
  { id: 'proof-audit-trail-log', name: 'Audit trail log', kind: 'section', tech: 'css', size: 'sm', use: 'Proof arrives as timestamped log lines, not floating logos.' },
  { id: 'hero-ticket-stub-tear', name: 'Ticket Stub Tear-Off', kind: 'hero', tech: 'js', size: 'sm', use: 'The stub tears away to show the date: for events and bookings.' },
];

const AGENTS = ['Claude Code', 'Cursor', 'Codex', 'OpenCode', 'Windsurf', 'Antigravity'];

function Demo({ id, still }: { id: string; still: boolean }) {
  const [qty, setQty] = useState(3);
  const cls = still ? 'mp-demo still' : 'mp-demo';
  switch (id) {
    case 'hero-odometer-in-headline':
      return (
        <div className={cls}>
          <p className="mp-odo">
            <span className="mp-wheel" aria-hidden="true"><span>0<br />1<br />2<br />3<br />4<br />5<br />6<br />7</span></span>
            <span className="mp-wheel w2" aria-hidden="true"><span>0<br />1<br />2<br />3<br />4<br />5<br />6</span></span>
            <span className="sr-only">76</span> mechanics, dealt to fit the brief.
          </p>
        </div>
      );
    case 'hero-seam-split-ink':
      return (
        <div className={cls}>
          <div className="mp-seam" aria-label="Generic in, designed out">
            <span className="top" aria-hidden="true">Generic in.</span>
            <span className="under">Designed out.</span>
            <span className="bot" aria-hidden="true">Generic in.</span>
          </div>
        </div>
      );
    case 'hero-floor-plan-draw':
      return (
        <div className={cls}>
          <svg className="mp-plan" viewBox="0 0 320 180" role="img" aria-label="A floor plan drawing itself">
            <path d="M20 20 H300 V160 H20 Z M20 90 H140 M140 20 V160 M200 90 H300 M200 90 V160" pathLength={1} />
            <path className="door" d="M140 120 A30 30 0 0 1 170 150" pathLength={1} />
          </svg>
        </div>
      );
    case 'pricing-receipt-printer-slider':
      return (
        <div className={cls}>
          <label className="mp-qty mono">Agents: <b>{qty}</b>
            <input type="range" min={1} max={6} value={qty} onChange={(e) => setQty(Number(e.target.value))} />
          </label>
          <div className="mp-receipt mono" aria-live="polite">
            {AGENTS.slice(0, qty).map((a) => <div key={a} className="ln"><span>{a}</span><span>£0.00</span></div>)}
            <div className="ln tot"><span>Total · MIT</span><span>£0.00</span></div>
          </div>
        </div>
      );
    case 'proof-audit-trail-log':
      return (
        <div className={cls}>
          <ol className="mp-log mono">
            <li><span>qa</span> 1440 · 0 errors</li>
            <li><span>qa</span> 768 · 0 errors</li>
            <li><span>qa</span> 375 · 0 errors</li>
            <li><span>gate</span> reduced motion honoured</li>
          </ol>
        </div>
      );
    default:
      return (
        <div className={cls}>
          <div className="mp-ticket">
            <div className="stub mono">ADMIT ONE</div>
            <div className="body"><b>npx layoza init</b><span className="mono">Any day · any agent</span></div>
          </div>
        </div>
      );
  }
}

export default function MotionPlayground() {
  const [kind, setKind] = useState<'all' | 'hero' | 'section'>('all');
  const [calm, setCalm] = useState(false);
  const [sel, setSel] = useState(MECHS[0].id);
  const [run, setRun] = useState(0);
  const [sysReduced, setSysReduced] = useState(false);

  useEffect(() => {
    const m = window.matchMedia('(prefers-reduced-motion: reduce)');
    setSysReduced(m.matches);
    const on = () => setSysReduced(m.matches);
    m.addEventListener('change', on);
    return () => m.removeEventListener('change', on);
  }, []);

  const pick = (id: string) => { setSel(id); setRun((r) => r + 1); };

  return (
    <div className={`mp${calm ? ' mp-calm' : ''}`}>
      <div className="mp-bar mono">
        <div className="mp-seg" role="radiogroup" aria-label="Mechanic type">
          {(['all', 'hero', 'section'] as const).map((t) => (
            <label key={t}><input type="radio" name="mp-kind" checked={kind === t} onChange={() => setKind(t)} />{t}</label>
          ))}
        </div>
        <div className="mp-seg" role="radiogroup" aria-label="Intensity">
          {([['bold', false], ['calm', true]] as const).map(([t, v]) => (
            <label key={t}><input type="radio" name="mp-int" checked={calm === v} onChange={() => { setCalm(v); setRun((r) => r + 1); }} />{t}</label>
          ))}
        </div>
        <button type="button" className="mp-replay" onClick={() => setRun((r) => r + 1)} disabled={sysReduced}>↻ Replay</button>
      </div>
      <ul className="mp-grid">
        {MECHS.map((m) => {
          const on = sel === m.id;
          const hidden = kind !== 'all' && m.kind !== kind;
          return (
            <li key={m.id} className={`mp-tile mp-${m.size}${on ? ' on' : ''}${hidden ? ' off' : ''}`}>
              <button type="button" className="mp-head" aria-pressed={on} onClick={() => pick(m.id)}>
                <span className="mp-name">{m.name}</span>
                <span className="mono mp-meta">{m.kind} · {m.tech}{on ? ' · playing' : ' · play'}</span>
              </button>
              {/* Only the active tile animates; the rest show their end state. */}
              <Demo key={on ? `${m.id}-${run}` : m.id} id={m.id} still={!on || sysReduced} />
              <p className="mp-use">{m.use}</p>
            </li>
          );
        })}
      </ul>
      {sysReduced && <p className="mp-note mono">Your system asks for reduced motion: every tile shows its end state and nothing moves.</p>}
    </div>
  );
}
