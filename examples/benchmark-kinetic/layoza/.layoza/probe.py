import pathlib, time
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").as_uri()
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={"width":1440,"height":900})
    pg.goto(url+"?t",wait_until="networkidle"); pg.wait_for_timeout(2000)
    for y in (4500,5000,5400,5800):
        pg.evaluate(f"scrollTo(0,{y})"); pg.wait_for_timeout(800)
        t=time.time(); r=pg.evaluate("new Promise(r=>{let n=0,s=performance.now();function f(){n++; if(n<10) requestAnimationFrame(f); else r(performance.now()-s)} requestAnimationFrame(f)})"); print(y,'10 frames ms',round(r))
        try:
            t=time.time(); pg.screenshot(path=f"probe_{y}.png",timeout=60000); print(' shot',round(time.time()-t,1),'s')
        except Exception as e: print(' fail',str(e)[:80])
    b.close()
