import { useEffect, useState } from 'react';
import './InstallTerminal.css';

type Tab = { id: string; label: string; cmd: string; out: string[] };

// Output lines are real, captured from `npx layoza init` / `doctor` in an empty git repo
// (paths shortened to the project root).
const TABS: Tab[] = [
  {
    id: 'init', label: 'Install', cmd: 'npx layoza init',
    out: [
      'Layoza initialized (Project scope) in `./my-site`',
      'Configured agents: antigravity, claude-code, codex, cursor, opencode',
      '  wrote: ./my-site/.claude/skills/layoza',
      '  wrote: ./my-site/.claude/skills/layoza-site-dna',
      '  wrote: ./my-site/.claude/skills/layoza-art-direction',
      '  … 19 skills per agent',
    ],
  },
  {
    id: 'doctor', label: 'Check', cmd: 'npx layoza doctor',
    out: [
      '=== Layoza Doctor ===',
      'Python Required    : false (100% Native Rust Core)',
      'Skills Validated   : 19/19',
      'Smoke Test Passed  : true',
      "  Agent `claude-code `: detected=true, skills_installed=true, mcp_configured=true",
    ],
  },
  {
    id: 'qa', label: 'Browser QA', cmd: 'npm i -D playwright && npx playwright install chromium',
    out: ['# Layoza drives your Chromium for capture_screenshots and layoza qa.', '# Install it once per machine.'],
  },
];

export default function InstallTerminal() {
  const [tab, setTab] = useState(0);
  const [shown, setShown] = useState(99);
  const [copied, setCopied] = useState(false);
  const t = TABS[tab];

  useEffect(() => {
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) { setShown(99); return; }
    setShown(0);
    let i = 0;
    const id = window.setInterval(() => {
      i += 1;
      setShown(i);
      if (i >= t.out.length) window.clearInterval(id);
    }, 260);
    return () => window.clearInterval(id);
  }, [tab]);

  const copy = async () => {
    try { await navigator.clipboard.writeText(t.cmd); setCopied(true); setTimeout(() => setCopied(false), 1600); } catch { /* clipboard blocked: the text stays selectable */ }
  };

  return (
    <div className="term">
      <div className="term-bar" role="tablist" aria-label="Install steps">
        {TABS.map((x, i) => (
          <button key={x.id} type="button" role="tab" aria-selected={i === tab} aria-controls="term-body" onClick={() => setTab(i)}>
            <span className="mono">{String(i + 1).padStart(2, '0')}</span> {x.label}
          </button>
        ))}
      </div>
      <div id="term-body" className="term-body mono" role="tabpanel">
        <div className="term-cmd">
          <code><span className="p">$</span> {t.cmd}</code>
          <button type="button" onClick={copy} aria-label={`Copy command: ${t.cmd}`}>{copied ? 'Copied' : 'Copy'}</button>
        </div>
        {/* Every line is laid out from the start (untyped ones hidden), so typing never moves the page. */}
        <pre aria-live="polite">{t.out.map((l, i) => <span key={i} data-wait={i >= shown ? '' : undefined}>{l}</span>)}</pre>
      </div>
    </div>
  );
}
