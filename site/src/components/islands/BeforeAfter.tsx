import { useRef, useState, type KeyboardEvent, type PointerEvent } from 'react';

export type Pair = { id: string; label: string; brief: string; before: string; after: string; afterNote: string };

export default function BeforeAfter({ pairs }: { pairs: Pair[] }) {
  const [active, setActive] = useState(0);
  const [pos, setPos] = useState(50);
  const frame = useRef<HTMLDivElement>(null);
  const dragging = useRef(false);
  const pair = pairs[active];

  const fromPointer = (e: PointerEvent) => {
    const r = frame.current!.getBoundingClientRect();
    setPos(Math.max(0, Math.min(100, ((e.clientX - r.left) / r.width) * 100)));
  };
  const onKey = (e: KeyboardEvent) => {
    const step = e.shiftKey ? 10 : 2;
    if (e.key === 'ArrowLeft') setPos((p) => Math.max(0, p - step));
    else if (e.key === 'ArrowRight') setPos((p) => Math.min(100, p + step));
    else if (e.key === 'Home') setPos(0);
    else if (e.key === 'End') setPos(100);
    else return;
    e.preventDefault();
  };

  return (
    <div className="ba">
      <div className="ba-tabs" role="tablist" aria-label="Benchmark brief">
        {pairs.map((p, i) => (
          <button
            key={p.id}
            role="tab"
            type="button"
            id={`ba-tab-${p.id}`}
            aria-selected={i === active}
            aria-controls="ba-panel"
            onClick={() => { setActive(i); setPos(50); }}
          >
            {p.label}
          </button>
        ))}
      </div>
      <div id="ba-panel" role="tabpanel" aria-labelledby={`ba-tab-${pair.id}`}>
        <p className="ba-brief mono">Brief: {pair.brief}</p>
        <div
          className="ba-frame"
          ref={frame}
          onPointerDown={(e) => { dragging.current = true; (e.target as Element).setPointerCapture?.(e.pointerId); fromPointer(e); }}
          onPointerMove={(e) => dragging.current && fromPointer(e)}
          onPointerUp={() => (dragging.current = false)}
          onPointerCancel={() => (dragging.current = false)}
        >
          <img src={pair.after} alt={`Layoza output for the ${pair.label} brief`} width={1440} height={900} loading="lazy" decoding="async" />
          <div className="ba-before" style={{ clipPath: `inset(0 ${100 - pos}% 0 0)` }}>
            <img src={pair.before} alt={`Same model, no skill, for the ${pair.label} brief`} width={1440} height={900} loading="lazy" decoding="async" />
          </div>
          <span className="ba-tag ba-tag-l mono">No skill</span>
          <span className="ba-tag ba-tag-r mono">With Layoza</span>
          <div
            className="ba-handle"
            style={{ left: `${pos}%` }}
            role="slider"
            tabIndex={0}
            aria-label="Reveal the no-skill version"
            aria-valuemin={0}
            aria-valuemax={100}
            aria-valuenow={Math.round(pos)}
            aria-valuetext={`${Math.round(pos)}% no-skill version shown`}
            onKeyDown={onKey}
          >
            <span aria-hidden="true">◀ ▶</span>
          </div>
        </div>
        <p className="ba-note">{pair.afterNote}</p>
      </div>
    </div>
  );
}
