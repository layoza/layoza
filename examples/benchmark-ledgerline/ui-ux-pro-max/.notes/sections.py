import pathlib, sys
from playwright.sync_api import sync_playwright
here = pathlib.Path(__file__).parent
url = (here.parent / "index.html").resolve().as_uri()
w,h = int(sys.argv[1]), int(sys.argv[2])
with sync_playwright() as p:
    b = p.chromium.launch(); pg = b.new_page(viewport={"width":w,"height":h})
    pg.goto(url); pg.wait_for_timeout(1000)
    pg.evaluate("document.querySelectorAll('.reveal').forEach(e=>e.classList.add('in'))")
    for sel in ["#product .board","#how","#estimator","#pricing","#security","#customers","#faq","#trial","footer"]:
        pg.locator(sel).screenshot(path=str(here/f"{w}-{sel.strip('#.').replace(' ','_').replace('.','')}.png"))
    b.close()
