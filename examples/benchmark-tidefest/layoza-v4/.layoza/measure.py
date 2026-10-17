import sys, pathlib
from playwright.sync_api import sync_playwright
url = pathlib.Path(sys.argv[1]).resolve().as_uri()
with sync_playwright() as p:
    b = p.chromium.launch()
    pg = b.new_page(viewport={'width':1440,'height':900})
    pg.goto(url); pg.wait_for_timeout(2500)
    print(pg.evaluate("""() => { const h=document.querySelector('.wordmark'); const r=document.createRange(); r.selectNodeContents(h); const w=r.getBoundingClientRect().width; const fs=parseFloat(getComputedStyle(h).fontSize); return {w, fs, ratio:w/fs, cw:h.clientWidth}; }"""))
    b.close()
