import sys, pathlib
from playwright.sync_api import sync_playwright
root=pathlib.Path(__file__).resolve().parent.parent
w=int(sys.argv[1]); h=int(sys.argv[2]); ids=sys.argv[3].split(',')
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={'width':w,'height':h}); pg.goto((root/'index.html').as_uri())
    pg.add_style_tag(content='.js .reveal{opacity:1!important;transform:none!important}')
    pg.wait_for_timeout(800)
    for i in ids:
        pg.locator(i).screenshot(path=str(root/'.notes'/f'sec-{w}-{i.strip("#.")}.png'))
    b.close()
