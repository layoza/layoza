import pathlib
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").as_uri()
JS="""ScrollTrigger.getAll().filter(s=>s.trigger&&s.trigger.classList&&(s.trigger.classList.contains('proj')||s.trigger.classList.contains('ins-list'))).map(s=>[s.trigger.className.slice(0,12),Math.round(s.start),Math.round(s.trigger.getBoundingClientRect().top+scrollY-innerHeight*0.82)])"""
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={"width":1440,"height":900})
    pg.goto(url,wait_until="load")
    for t in (0,1500,4000):
        pg.wait_for_timeout(t); print(t,pg.evaluate(JS), pg.evaluate("document.documentElement.scrollHeight"))
    b.close()
