import sys, pathlib
from playwright.sync_api import sync_playwright
url = pathlib.Path('index.html').resolve().as_uri()
out = pathlib.Path('.layoza/tframes'); out.mkdir(exist_ok=True)
with sync_playwright() as p:
    b = p.chromium.launch()
    for t in ['0','0.25','0.5','0.75','1']:
        pg = b.new_page(viewport={'width':1440,'height':900}); pg.goto(url+'?t='+t); pg.wait_for_timeout(2600)
        pg.screenshot(path=str(out/f't{t}.png')); pg.close()
    b.close()
