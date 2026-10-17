import pathlib, json
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").as_uri()
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={"width":1440,"height":900})
    pg.goto(url,wait_until="load"); pg.wait_for_timeout(2500)
    for sel in ['.ins__btn','#reel-play','#f-name']:
        pg.evaluate("scrollTo(0,0)"); pg.wait_for_timeout(600)
        pg.focus(sel); 
        for t in (50,400,1200):
            pg.wait_for_timeout(t)
            print(sel,t,pg.evaluate(f"(()=>{{const r=document.querySelector('{sel}').getBoundingClientRect();return [Math.round(r.top),Math.round(scrollY),document.documentElement.classList.contains('lenis')]}})()"))
    b.close()
