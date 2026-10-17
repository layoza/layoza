import sys, pathlib
from playwright.sync_api import sync_playwright
f=pathlib.Path(sys.argv[1]).resolve().as_uri(); out=pathlib.Path(sys.argv[2]); out.mkdir(exist_ok=True)
for old in out.glob('*.png'): old.unlink()
with sync_playwright() as p:
    b=p.chromium.launch()
    for w,h,n in [(1440,900,'d'),(768,1024,'t'),(375,812,'m')]:
        pg=b.new_page(viewport={'width':w,'height':h}); pg.goto(f); pg.wait_for_timeout(1200)
        H=pg.evaluate('document.body.scrollHeight'); i=0; y=0
        while y<H:
            pg.evaluate(f'window.scrollTo(0,{y})'); pg.wait_for_timeout(150)
            pg.screenshot(path=str(out/f'{n}{i}.png')); i+=1; y+=h
        pg.close()
    b.close()
