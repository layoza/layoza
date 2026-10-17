import sys, json
from pathlib import Path
from playwright.sync_api import sync_playwright
D = Path(sys.argv[1]); out = D/'.impeccable'/'review'
url = (D/'index.html').as_uri()
res = {}
with sync_playwright() as p:
    b = p.chromium.launch()
    for name,(w,h) in {'desktop':(1440,900),'tablet':(768,1024),'mobile':(375,812)}.items():
        pg = b.new_page(viewport={'width':w,'height':h}, reduced_motion='reduce')
        errs=[]; pg.on('console', lambda m: errs.append(m.text) if m.type=='error' else None); pg.on('pageerror', lambda e: errs.append(str(e)))
        pg.goto(url); pg.evaluate('document.fonts.ready'); pg.wait_for_timeout(600)
        pg.screenshot(path=str(out/f'{name}-fold.png'))
        pg.screenshot(path=str(out/f'{name}.png'), full_page=True)
        ov = pg.evaluate('''() => { const W=document.documentElement.clientWidth; return [...document.querySelectorAll('body *')].filter(e=>{const r=e.getBoundingClientRect(); return r.width && (r.right>W+1||r.left<-1)}).slice(0,10).map(e=>e.tagName+'.'+e.className+' '+Math.round(e.getBoundingClientRect().right))}''')
        res[name]={'errors':errs,'overflow':ov,'scrollW':pg.evaluate('document.documentElement.scrollWidth')}
        pg.close()
    b.close()
print(json.dumps(res,indent=1))
