import sys, asyncio
from playwright.sync_api import sync_playwright
import pathlib
f = pathlib.Path(sys.argv[1]).resolve().as_uri()
out = pathlib.Path(sys.argv[2])
with sync_playwright() as p:
    b = p.chromium.launch()
    for name,w,h in [('d',1440,900),('t',768,1024),('m',375,812)]:
        pg = b.new_page(viewport={'width':w,'height':h})
        pg.goto(f); pg.wait_for_timeout(3800)
        pg.screenshot(path=str(out/f'{name}-0.png'))
        H = pg.evaluate('document.documentElement.scrollHeight')
        y=h; i=1
        while y < H:
            pg.evaluate(f'window.scrollTo(0,{y})'); pg.wait_for_timeout(700)
            pg.screenshot(path=str(out/f'{name}-{i}.png')); y+=h; i+=1
        pg.close()
    b.close()
