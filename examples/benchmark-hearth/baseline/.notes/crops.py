import pathlib, sys
from playwright.sync_api import sync_playwright
root = pathlib.Path(__file__).resolve().parent
url = (root.parent / "index.html").as_uri()
w = int(sys.argv[1]) if len(sys.argv)>1 else 1440
with sync_playwright() as p:
    b = p.chromium.launch(); pg = b.new_page(viewport={"width":w,"height":900})
    pg.goto(url); pg.wait_for_timeout(800)
    pg.evaluate("document.querySelectorAll('.pre').forEach(e=>e.classList.remove('pre'))"); pg.wait_for_timeout(900)
    for s in ["#menu","#bakery","#oven","#book","#private","#reviews","#faq","#visit"]:
        pg.locator(s).screenshot(path=str(root/f"sec-{w}-{s[1:]}.png"))
    b.close()
