import sys, pathlib
from playwright.sync_api import sync_playwright
url = pathlib.Path(sys.argv[1]).resolve().as_uri()
with sync_playwright() as p:
    b = p.chromium.launch()
    pg = b.new_page(viewport={'width':390,'height':812})
    pg.goto(url); pg.wait_for_timeout(500)
    print(pg.evaluate("""() => { const a=document.querySelector('.bakery__arch').getBoundingClientRect(); const l=document.querySelector('.bakery__loaves').getBoundingClientRect(); const s=document.querySelector('.bakery__loaves svg.is-on'); const r=s.getBoundingClientRect(); return {a:[a.x,a.y,a.width,a.height], l:[l.x,l.y,l.width,l.height], s:[r.x,r.y,r.width,r.height], vb:s.getAttribute('viewBox'), errs: window.__errs}; }"""))
    b.close()
