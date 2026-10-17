import pathlib
from playwright.sync_api import sync_playwright
here=pathlib.Path(__file__).parent; url=(here.parent/"index.html").as_uri()+"?t"
out=here/"shots"/"pins"; out.mkdir(parents=True,exist_ok=True)
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={"width":1440,"height":900})
    pg.goto(url,wait_until="networkidle"); pg.wait_for_timeout(3000)
    pg.screenshot(path=str(out/"f00_hero.png"))
    for name in ("phil","proc"):
        s,e=pg.evaluate(f"[window.__ks.{name}.scrollTrigger.start, window.__ks.{name}.scrollTrigger.end]")
        for k,q in enumerate((0,.25,.5,.75,1)):
            pg.evaluate(f"scrollTo(0,{s+(e-s)*q})"); pg.wait_for_timeout(1400)
            pg.screenshot(path=str(out/f"f{1 if name=='phil' else 2}{k}_{name}_{int(q*100)}.png"))
    b.close()
