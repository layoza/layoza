import sys
from playwright.sync_api import sync_playwright
url=sys.argv[1]; out=sys.argv[2]
sizes=[("desktop",1440,900),("tablet",768,1024),("mobile",375,812)]
with sync_playwright() as p:
    b=p.chromium.launch()
    for name,w,h in sizes:
        pg=b.new_page(viewport={"width":w,"height":h})
        errs=[]; pg.on("console",lambda m: errs.append(m.text) if m.type=="error" else None); pg.on("pageerror",lambda e: errs.append(str(e)))
        pg.goto(url); pg.evaluate("document.fonts.ready.then(()=>true)"); pg.wait_for_timeout(400)
        pg.screenshot(path=f"{out}/{name}-fold.png")
        pg.screenshot(path=f"{out}/{name}-full.png",full_page=True)
        sw=pg.evaluate("document.documentElement.scrollWidth")
        print(name,"scrollWidth",sw,"errors",errs)
        pg.close()
    b.close()
