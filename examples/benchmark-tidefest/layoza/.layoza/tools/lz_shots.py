import sys, pathlib
from playwright.sync_api import sync_playwright
f=pathlib.Path(sys.argv[1]).resolve().as_uri(); out=pathlib.Path(sys.argv[2]); out.mkdir(exist_ok=True, parents=True)
with sync_playwright() as p:
    b=p.chromium.launch()
    for w,h,tag in [(1440,900,'d'),(375,812,'m')]:
        pg=b.new_page(viewport={'width':w,'height':h}); pg.goto(f); pg.wait_for_timeout(2500)
        pg.screenshot(path=str(out/f'{tag}-00.png'))
        H=pg.evaluate('document.documentElement.scrollHeight'); y=0;i=1
        while y<H:
            y+=h*0.9; pg.mouse.wheel(0,h*0.9); pg.wait_for_timeout(900)
            pg.screenshot(path=str(out/f'{tag}-{i:02d}.png')); i+=1
            H=pg.evaluate('document.documentElement.scrollHeight')
            if i>40: break
    b.close()
