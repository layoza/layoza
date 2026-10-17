import pathlib
p = pathlib.Path(__file__).resolve().parents[1] / 'src' / 'index.template.html'
t = p.read_text(encoding='utf-8')

def R(a, b):
    global t
    assert a in t, a[:90]
    t = t.replace(a, b)

R('font-variation-settings:"wdth" 80;opacity:.5;transition:opacity .2s ease,transform .2s var(--ease-out)}\n.sub.on{opacity:1}',
  'font-variation-settings:"wdth" 80;transition:transform .2s var(--ease-out)}')
R('.stepper{display:inline-flex;', '.stepper{justify-self:start;display:inline-flex;')
R('.news h2{font-variation-settings:"wdth" 120}', '.news h2{font-size:clamp(3.25rem,7.4vw,7.5rem);font-variation-settings:"wdth" 120}')
# map: tents on the headland, the two stages on the beach
R('<text x="214" y="372" fill="#0F1E4A" font-size="22">festival beach</text>',
  '<g fill="#0F1E4A" aria-hidden="true"><path d="M300 250l12-20 12 20z M328 256l12-20 12 20z M356 248l12-20 12 20z M314 280l12-20 12 20z M342 284l12-20 12 20z"/></g>'
  '<g fill="none" stroke="#0F1E4A" stroke-width="4" stroke-linecap="round"><path d="M232 330 q22-22 44 0"/><path d="M286 352 v-16 a12 12 0 0 1 24 0 v16"/></g>'
  '<text x="214" y="384" fill="#0F1E4A" font-size="22">festival beach</text>')
p.write_text(t, encoding='utf-8')
print('round3 applied')
