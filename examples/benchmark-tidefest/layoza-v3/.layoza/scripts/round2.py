import pathlib, re
p = pathlib.Path(__file__).resolve().parents[1] / 'src' / 'index.template.html'
t = p.read_text(encoding='utf-8')

def R(a, b):
    global t
    assert a in t, a[:90]
    t = t.replace(a, b)

# section rhythm
R('.section{padding-block:clamp(96px,13vw,200px)}', '.section{padding-block:clamp(80px,9vw,140px)}')

# tickets: swallowtail band end + per-row subtotal instead of the clasp (it read as a "0")
t = re.sub(r'\n        <svg class="clasp".*?</svg>', '\n        <p class="sub tab" aria-hidden="true">£0</p>', t)
R('.clasp{position:absolute;right:clamp(14px,2vw,30px);top:50%;width:clamp(34px,4vw,48px);height:auto;transform:translateY(-50%);transition:transform .22s var(--ease-out)}\n.band.bump .clasp{transform:translateY(-50%) rotate(-14deg)}',
  '.band{clip-path:polygon(0 0,100% 0,calc(100% - 26px) 50%,100% 100%,0 100%)}\n.sub{margin:0;min-width:4.5ch;text-align:right;font-weight:900;font-size:clamp(1.3rem,2vw,1.7rem);font-variation-settings:"wdth" 80;opacity:.5;transition:opacity .2s ease,transform .2s var(--ease-out)}\n.sub.on{opacity:1}\n.band.bump .sub{transform:translateY(-3px)}')
R('grid-template-columns:minmax(0,1fr) auto auto;align-items:center;gap:16px clamp(20px,3vw,48px);background:var(--c);color:var(--t);border-radius:28px 12px 12px 28px;',
  'grid-template-columns:minmax(0,1fr) auto auto auto;align-items:center;gap:16px clamp(20px,3vw,48px);background:var(--c);color:var(--t);')
R('  .band{grid-template-columns:1fr auto;padding-right:clamp(64px,16vw,90px)}',
  '  .band{grid-template-columns:1fr auto;padding-right:48px}\n  .band .price{grid-column:1/-1}')
R('.clasp,.stepper button,.menu-toggle{', '.sub,.stepper button,.menu-toggle{')
R('    var span = document.createElement("span"); span.textContent = n; out.replaceChildren(span);',
  '    var span = document.createElement("span"); span.textContent = n; out.replaceChildren(span);\n    sub.textContent = gbp(n * +b.dataset.price); sub.classList.toggle("on", n>0);')
R('var out = $("output", b), minus', 'var sub = $(".sub", b), out = $("output", b), minus')

# countdown: numerals at hero scale standing on a horizon, the sun rising behind them
old = t[t.index('<section class="section countdown"'):t.index('<section class="section tickets"')]
new = '''<section class="section countdown" id="countdown" aria-labelledby="cd-title">
  <div class="wrap">
    <div class="cd-head">
      <h2 id="cd-title">Gates open in</h2>
      <p>12:00 UK time, Saturday 14 August 2027. The first set starts two hours later on the Shoreline stage.</p>
    </div>
    <div class="cd-stage">
      <svg class="cd-sun" viewBox="0 0 1200 420" aria-hidden="true" preserveAspectRatio="xMidYMax meet"><defs><clipPath id="cdClip"><rect width="1200" height="404"/></clipPath></defs><g clip-path="url(#cdClip)"><g id="cdSun"><circle cx="600" cy="404" r="300" fill="#FFC83D"/><circle cx="600" cy="404" r="360" fill="none" stroke="#FFC83D" stroke-width="6" opacity=".55"/></g></g><path d="M0 404 H1200" stroke="#0F1E4A" stroke-width="5"/><path d="M80 418h220M420 418h120M700 418h260M1040 418h90" stroke="#0F1E4A" stroke-width="4" stroke-linecap="round" opacity=".5"/></svg>
      <dl class="cd tab" id="cd" aria-describedby="cd-title">
        <div><dt>days</dt><dd data-u="d"><span>000</span></dd></div>
        <div><dt>hours</dt><dd data-u="h"><span>00</span></dd></div>
        <div><dt>minutes</dt><dd data-u="m"><span>00</span></dd></div>
        <div><dt>seconds</dt><dd data-u="s"><span>00</span></dd></div>
      </dl>
    </div>
    <p class="cd-note" id="cd-done" hidden>Tidefest is open. Head for the sand.</p>
  </div>
</section>

'''
t = t.replace(old, new)
R('.cd-horizon{display:block;width:100%;height:auto;margin-top:-2px}',
  '.cd-head{display:flex;flex-wrap:wrap;justify-content:space-between;align-items:baseline;gap:16px 48px}\n.cd-head h2{font-size:var(--s-mid)}\n.cd-head p{max-width:40ch}\n.cd-stage{position:relative;margin-top:clamp(24px,4vw,56px)}\n.cd-sun{position:absolute;left:0;right:0;bottom:0;width:100%;height:auto;z-index:0}')
R('.cd dd{margin:0;font-weight:900;font-size:clamp(4.4rem,10.5vw,10rem);', '.cd dd{margin:0;font-weight:900;font-size:clamp(4.4rem,14vw,13rem);')
R('.cd{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:clamp(8px,2vw,32px);',
  '.cd{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:clamp(8px,2vw,32px);padding:clamp(40px,8vw,120px) 0 clamp(28px,4vw,52px);')
R('.cd div{display:flex;flex-direction:column-reverse}', '.cd div{display:flex;flex-direction:column-reverse;align-items:center;text-align:center}')
R('gsap.fromTo("#cdSun", { y: 70 }, { y: 0, ease: "none", scrollTrigger: { trigger: "#countdown", start: "top 80%", end: "bottom 60%", scrub: .8 } });',
  'gsap.fromTo("#cdSun", { y: 220 }, { y: 0, ease: "none", scrollTrigger: { trigger: "#countdown", start: "top 85%", end: "bottom 75%", scrub: .8 } });')

# camping: map leads at 7/12, heading and the four blocks beside it
old = t[t.index('<section class="section camping"'):t.index('<section class="section faq"')]
i = old.index('      <svg class="map"')
mapsvg = old[i:old.index('</svg>', i) + 6]
j = old.index('      <ol class="info">')
info = old[j:old.index('</ol>', j) + 5]
new = ('<section class="section camping" id="camping" aria-labelledby="camp-title">\n  <div class="wrap camp-grid">\n'
       + mapsvg + '\n    <div>\n      <h2 id="camp-title">Camping &amp; travel</h2>\n'
       '      <p class="camp-lede">The beach sits on the Atlantic coast a short ride from Newquay. Here is how the site fits together.</p>\n'
       + info + '\n    </div>\n  </div>\n</section>\n\n')
t = t.replace(old, new)
R('.camp-grid{display:grid;grid-template-columns:minmax(0,6fr) minmax(0,6fr);gap:clamp(32px,5vw,80px);align-items:start}',
  '.camp-grid{display:grid;grid-template-columns:minmax(0,7fr) minmax(0,5fr);gap:clamp(32px,5vw,72px);align-items:start}\n.camping h2{font-size:var(--s-mid)}\n.camp-lede{margin:18px 0 28px;max-width:44ch}\n@media (min-width:900px){.map{position:sticky;top:calc(var(--header-h) + 24px)}}')

# FAQ: split, heading left, questions right
R('''    <div class="section-head">
      <h2 id="faq-title">Good to know</h2>
      <p>Still wondering about something? Write to <a href="mailto:hello@tidefest.example">hello@tidefest.example</a>.</p>
    </div>
    <div class="faq-list" id="faq">''', '''    <div class="faq-grid">
    <div class="faq-aside">
      <h2 id="faq-title">Good to know</h2>
      <p>Still wondering about something? Write to <a href="mailto:hello@tidefest.example">hello@tidefest.example</a>.</p>
      <svg class="faq-art" viewBox="0 0 400 220" aria-hidden="true"><circle cx="200" cy="200" r="120" fill="#FFC83D"/><rect y="196" width="400" height="24" fill="#182C66"/><path d="M0 198H400" stroke="#FFF3E4" stroke-width="4"/><path d="M30 212c20-8 34-8 54 0s34 8 54 0 34-8 54 0M230 212c20-8 34-8 54 0s34 8 54 0" fill="none" stroke="#FF6A4D" stroke-width="4" stroke-linecap="round"/></svg>
    </div>
    <div class="faq-list" id="faq">''')
k = t.index('<div class="faq-list" id="faq">')
end = t.index('</section>', k)
seg = t[k:end]
seg2 = seg.rstrip()
assert seg2.endswith('</div>\n  </div>')
seg2 = seg2[:-len('</div>\n  </div>')] + '</div>\n    </div>\n  </div>\n'
t = t[:k] + seg2 + t[end:]
R('.faq-list{border-top:2px solid var(--foam);max-width:1000px}',
  '.faq-grid{display:grid;grid-template-columns:minmax(0,5fr) minmax(0,7fr);gap:clamp(32px,5vw,80px);align-items:start}\n.faq-aside{position:sticky;top:calc(var(--header-h) + 24px)}\n.faq-aside h2{font-size:var(--s-mid);max-width:8ch}\n.faq-aside p{margin-top:20px;color:var(--foam-2);max-width:34ch}\n.faq-art{width:min(100%,360px);height:auto;margin-top:40px}\n.faq-list{border-top:2px solid var(--foam)}\n@media (max-width:899px){.faq-grid{grid-template-columns:1fr}.faq-aside{position:static}.faq-art{display:none}}')

# newsletter: copy + form left, sunrise field right
R('''    <div class="news-copy">
      <h2 id="news-title">Hear it first</h2>
      <p>New names, stage times and who plays each sunrise set, sent when they land. Twice a month at most.</p>
    </div>
    <div>''', '''    <div>
      <div class="news-copy">
        <h2 id="news-title">Hear it first</h2>
        <p>New names, stage times and who plays each sunrise set, sent when they land. Twice a month at most.</p>
      </div>''')
R('''      </div>
    </div>
  </div>
</section>
</main>''', '''      </div>
    </div>
    <svg class="news-art" viewBox="0 0 500 420" aria-hidden="true"><defs><clipPath id="newsClip"><rect width="500" height="300"/></clipPath></defs><g clip-path="url(#newsClip)"><circle id="newsSun" cx="250" cy="300" r="190" fill="#FFC83D"/></g><rect y="300" width="500" height="120" fill="#0F1E4A"/><path d="M0 300H500" stroke="#FFF3E4" stroke-width="4"/><g fill="#FFC83D"><rect x="150" y="318" width="200" height="6" rx="3"/><rect x="180" y="340" width="140" height="6" rx="3" opacity=".8"/><rect x="205" y="362" width="90" height="6" rx="3" opacity=".6"/><rect x="228" y="384" width="44" height="6" rx="3" opacity=".4"/></g></svg>
  </div>
</section>
</main>''')
R('.news-grid{display:grid;grid-template-columns:minmax(0,7fr) minmax(0,5fr);gap:clamp(32px,5vw,80px);align-items:end}',
  '.news-grid{display:grid;grid-template-columns:minmax(0,6fr) minmax(0,6fr);gap:clamp(32px,5vw,80px);align-items:center}\n.news .signup,.news .success{margin-top:36px;max-width:560px}\n.news-art{width:100%;height:auto;border-radius:20px}')
R('@media (max-width:899px){.news-grid{grid-template-columns:1fr}}', '@media (max-width:899px){.news-grid{grid-template-columns:1fr}.news-art{max-width:420px}}')
R('$("#okSun").animate(', '$("#newsSun").animate([{transform:"translateY(0)"},{transform:"translateY(-70px)"}],{duration:1200,easing:"cubic-bezier(.23,1,.32,1)",fill:"forwards"}); $("#okSun").animate(')
R('    // footer: the sun sets', '    gsap.fromTo("#newsSun", { y: 160 }, { y: 0, ease: "none", scrollTrigger: { trigger: "#news", start: "top 85%", end: "center 55%", scrub: .8 } });\n    // footer: the sun sets')

# calendar note beside the day filter
R('        </fieldset>', '        </fieldset>\n        <p class="day-note">14 August 2027 falls on a Saturday, so Tidefest runs Saturday to Monday.</p>')
R('.stage-key{', '.day-note{margin-top:14px;font-size:1rem;color:var(--foam-2);max-width:48ch}\n.stage-key{')

p.write_text(t, encoding='utf-8')
print('round2 applied')
