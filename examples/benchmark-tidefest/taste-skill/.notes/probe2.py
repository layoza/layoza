import sys
from playwright.sync_api import sync_playwright
from pathlib import Path
url = Path(sys.argv[1]).resolve().as_uri()
with sync_playwright() as p:
    b = p.chromium.launch(); pg = b.new_page(viewport={'width':1440,'height':900})
    pg.goto(url); pg.wait_for_timeout(1200)
    pg.evaluate("window.scrollTo(0, 1e6)"); pg.wait_for_timeout(600)
    print(pg.evaluate("""() => ({body: document.body.getBoundingClientRect().bottom + scrollY, doc: document.documentElement.scrollHeight, y: scrollY,
      els: [...document.querySelectorAll('body > *')].map(e => e.tagName + '.' + e.className + ' ' + Math.round(e.getBoundingClientRect().top + scrollY) + '-' + Math.round(e.getBoundingClientRect().bottom + scrollY))})"""))
    b.close()
