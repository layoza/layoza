import pathlib
from playwright.sync_api import sync_playwright
url=(pathlib.Path('..').resolve()/'index.html').as_uri()
with sync_playwright() as p:
    b=p.chromium.launch()
    for w in (1440,375):
        pg=b.new_page(viewport={'width':w,'height':800}); pg.goto(url,wait_until='networkidle'); pg.wait_for_timeout(1500)
        print(w, pg.evaluate("""[document.documentElement.scrollWidth, document.documentElement.clientWidth, window.innerWidth, document.body.scrollWidth,
        [...document.querySelectorAll('body *')].filter(e=>{const r=e.getBoundingClientRect();return r.right>document.documentElement.clientWidth+1 && !e.closest('.marquee') && !e.closest('#days-track')}).slice(0,8).map(e=>e.tagName+'.'+e.className+':'+Math.round(e.getBoundingClientRect().right))]"""))
    b.close()
