import sys, pathlib
from playwright.sync_api import sync_playwright
f = pathlib.Path(sys.argv[1]).resolve().as_uri()
with sync_playwright() as p:
    b = p.chromium.launch(); pg = b.new_page(viewport={'width':1440,'height':900})
    pg.goto(f); pg.wait_for_timeout(6000)
    print(pg.inner_text('.box[data-b="5"] .bv'))
    pg.click('#reqBtn'); pg.wait_for_timeout(3000)
    print(pg.inner_text('.box[data-b="5"] .bv'), pg.evaluate("(5426.25).toLocaleString('en-GB',{minimumFractionDigits:2,maximumFractionDigits:2})"))
    b.close()
