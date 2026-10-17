import pathlib, json
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").as_uri()
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={"width":1440,"height":900})
    pg.goto(url,wait_until="load"); pg.wait_for_timeout(2500)
    h=pg.evaluate("document.documentElement.scrollHeight")
    for y in range(0,h,400): pg.evaluate(f"scrollTo(0,{y})"); pg.wait_for_timeout(60)
    pg.wait_for_timeout(2000)
    print(pg.evaluate("""[...document.querySelectorAll('body *')].filter(e=>{const cs=getComputedStyle(e);return cs.opacity==='0'&&e.textContent.trim()&&e.children.length<3}).slice(0,50).map(e=>e.tagName+'.'+e.className+':'+e.textContent.trim().slice(0,20))"""))
    b.close()
