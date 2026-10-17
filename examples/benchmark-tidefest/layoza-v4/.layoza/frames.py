import sys, pathlib
from playwright.sync_api import sync_playwright
url = pathlib.Path(sys.argv[1]).resolve().as_uri()
out = pathlib.Path(sys.argv[2]); out.mkdir(parents=True, exist_ok=True)
w, h, step = int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
rm = len(sys.argv) > 6 and sys.argv[6] == 'reduce'
with sync_playwright() as p:
    b = p.chromium.launch()
    ctx = b.new_context(viewport={'width': w, 'height': h}, reduced_motion='reduce' if rm else 'no-preference')
    pg = ctx.new_page(); pg.goto(url); pg.wait_for_timeout(2500)
    total = pg.evaluate('document.documentElement.scrollHeight')
    y, i = 0, 0
    while y < total:
        pg.mouse.wheel(0, 0)
        pg.evaluate(f'window.scrollTo(0,{y})'); pg.wait_for_timeout(1300)
        pg.screenshot(path=str(out / f'{w}-{i:02d}.png')); y += step; i += 1
    print(total, i)
    b.close()
