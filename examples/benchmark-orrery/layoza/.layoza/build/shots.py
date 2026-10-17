import sys, pathlib
from playwright.sync_api import sync_playwright
root = pathlib.Path(__file__).resolve().parents[2]
url = (root/'index.html').as_uri(); out = root/'.layoza'/'shots'
with sync_playwright() as p:
    b = p.chromium.launch()
    for name,w,h in (('d',1440,900),('m',375,812)):
        pg = b.new_page(viewport={'width':w,'height':h}); pg.goto(url); pg.wait_for_timeout(2200)
        pg.screenshot(path=str(out/f'{name}-hero.png'))
        for sec in ('shows','book','telescope','shop'):
            pg.locator('#'+sec).scroll_into_view_if_needed(); pg.evaluate(f"document.getElementById('{sec}').scrollIntoView()"); pg.wait_for_timeout(400)
            pg.screenshot(path=str(out/f'{name}-{sec}.png'))
    b.close()
