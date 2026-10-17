import sys
from playwright.sync_api import sync_playwright
with sync_playwright() as p:
    b=p.chromium.launch(args=["--use-gl=angle","--ignore-gpu-blocklist"]); pg=b.new_page(viewport={"width":1440,"height":900})
    pg.on("pageerror", lambda e: print("ERR", e.stack if hasattr(e,'stack') else e))
    pg.goto(sys.argv[1], wait_until="networkidle"); pg.wait_for_timeout(2500)
    for y in range(0, 30000, 600):
        pg.evaluate(f"window.scrollTo(0,{y})"); pg.wait_for_timeout(120)
    pg.wait_for_timeout(1000); b.close()
