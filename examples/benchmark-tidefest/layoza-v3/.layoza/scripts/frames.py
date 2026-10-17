import sys, pathlib
from playwright.sync_api import sync_playwright
root = pathlib.Path(__file__).resolve().parents[2]
url = (root/'index.html').as_uri()
out = root/'.layoza'/'frames'; out.mkdir(exist_ok=True)
w, h = int(sys.argv[1]), int(sys.argv[2])
ys = sys.argv[3].split(',')
tag = sys.argv[4] if len(sys.argv) > 4 else ''
with sync_playwright() as p:
    b = p.chromium.launch()
    pg = b.new_page(viewport={'width': w, 'height': h}, reduced_motion='reduce' if tag=='rm' else 'no-preference')
    pg.goto(url); pg.wait_for_timeout(2200)
    for y in ys:
        if y.lstrip('-').isdigit(): pg.evaluate(f'window.scrollTo(0,{y})')
        else: pg.evaluate(f'window.scrollTo(0, document.querySelector("{y}").getBoundingClientRect().top + scrollY - 72)')
        pg.wait_for_timeout(1400)
        pg.screenshot(path=str(out/f'{w}-{y.strip("#.")}{tag}.png'))
    b.close()
print('done')
