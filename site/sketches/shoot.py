import sys, pathlib
from playwright.sync_api import sync_playwright
root = pathlib.Path(sys.argv[1]).resolve()
with sync_playwright() as p:
    b = p.chromium.launch()
    for d in ["v2-a", "v2-b", "v2-c"]:
        for name, w, h in [("desktop", 1440, 900), ("mobile", 375, 812)]:
            pg = b.new_page(viewport={"width": w, "height": h})
            pg.goto((root / d / "index.html").as_uri()); pg.wait_for_timeout(900)
            pg.screenshot(path=str(root / f"{d}-{name}.png"))
            print(d, name, pg.evaluate("document.documentElement.scrollWidth"))
            pg.close()
    b.close()
