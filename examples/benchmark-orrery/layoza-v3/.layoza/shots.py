import sys, pathlib
from playwright.sync_api import sync_playwright
root = pathlib.Path(__file__).resolve().parent
url = (root.parent / 'index.html').as_uri()
out = root / 'shots'
targets = sys.argv[1:] or ['top']
with sync_playwright() as p:
    b = p.chromium.launch()
    for w, h, tag in [(1440, 900, 'd'), (375, 812, 'm')]:
        pg = b.new_page(viewport={'width': w, 'height': h})
        pg.goto(url); pg.wait_for_timeout(3000)
        for t in targets:
            if t != 'top':
                pg.evaluate(f"document.querySelector('#{t}').scrollIntoView()")
            else:
                pg.evaluate("scrollTo(0,0)")
            pg.wait_for_timeout(1300)
            pg.screenshot(path=str(out / f'{tag}-{t}.png'))
    b.close()
