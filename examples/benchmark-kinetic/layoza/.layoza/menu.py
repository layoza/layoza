import pathlib
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").as_uri()
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={"width":375,"height":812})
    pg.goto(url,wait_until="networkidle"); pg.wait_for_timeout(2500)
    pg.click('#menu-btn'); pg.wait_for_timeout(1200); pg.screenshot(path="shots/menu.png")
    print(pg.evaluate("(()=>{const b=document.querySelector('.menu__foot .btn');const s=getComputedStyle(b);const t=b.querySelector('.btn__t');return [s.opacity,s.color,s.backgroundColor,t.getBoundingClientRect().height,getComputedStyle(document.querySelector('.menu__foot')).opacity]})()"))
    b.close()
