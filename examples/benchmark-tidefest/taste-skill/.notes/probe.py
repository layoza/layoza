import sys, asyncio
from playwright.sync_api import sync_playwright
from pathlib import Path
url = Path(sys.argv[1]).resolve().as_uri()
with sync_playwright() as p:
    b = p.chromium.launch()
    pg = b.new_page(viewport={'width':1440,'height':900})
    errs=[]; pg.on('console', lambda m: errs.append(m.text) if m.type=='error' else None); pg.on('pageerror', lambda e: errs.append(str(e)))
    pg.goto(url); pg.wait_for_timeout(1500)
    print(pg.evaluate("""() => { const f=document.querySelector('.foot').getBoundingClientRect(); return {doc: document.documentElement.scrollHeight, footBottom: f.bottom + scrollY, sw: document.documentElement.scrollWidth,
      tall: [...document.querySelectorAll('body *')].map(e=>[e, e.getBoundingClientRect()]).filter(([e,r])=>r.bottom+scrollY > document.querySelector('.foot').getBoundingClientRect().bottom+scrollY+2).map(([e,r])=>e.tagName+'.'+e.className+' '+Math.round(r.bottom+scrollY)).slice(0,10)} }"""))
    print('errors', errs)
    b.close()
