import pathlib
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").resolve().as_uri()
with sync_playwright() as p:
    b=p.chromium.launch();pg=b.new_page(viewport={"width":375,"height":812});pg.goto(url,wait_until="networkidle")
    print(pg.evaluate("""[...document.querySelectorAll('body *')].filter(e=>e.getBoundingClientRect().right>376).slice(0,12).map(e=>e.tagName+'.'+e.className+' '+Math.round(e.getBoundingClientRect().right))"""))
    b.close()
