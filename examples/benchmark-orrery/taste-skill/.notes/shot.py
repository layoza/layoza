import sys, asyncio
from playwright.sync_api import sync_playwright
url = 'examples/benchmark-orrery/taste-skill/index.html'
out = 'examples/benchmark-orrery/taste-skill/.notes/'
with sync_playwright() as p:
    b = p.chromium.launch()
    pg = b.new_page(viewport={'width':1440,'height':900})
    errs=[]; pg.on('console', lambda m: errs.append(m.text) if m.type=='error' else None); pg.on('pageerror', lambda e: errs.append(str(e)))
    pg.goto(url); pg.wait_for_timeout(3000)
    print('seats', pg.evaluate("document.querySelectorAll('#seats rect').length"))
    for sel in sys.argv[1:]:
        pg.evaluate(f"document.querySelector('{sel}').scrollIntoView()"); pg.wait_for_timeout(2200)
        pg.screenshot(path=out+'s-'+sel.strip('#.').replace(' ','_')+'.png')
    print('errors', errs)
    b.close()
