import sys, pathlib, asyncio
from playwright.sync_api import sync_playwright
root = pathlib.Path(__file__).parent
url = (root.parent / "index.html").resolve().as_uri()
out = root / "shots"; out.mkdir(exist_ok=True)
with sync_playwright() as p:
    b = p.chromium.launch()
    for name, w, h in [("d", 1440, 900), ("t", 768, 1024), ("m", 375, 812)]:
        pg = b.new_page(viewport={"width": w, "height": h}, device_scale_factor=1)
        errs = []
        pg.on("console", lambda m: errs.append(m.text) if m.type == "error" else None)
        pg.on("pageerror", lambda e: errs.append(str(e)))
        pg.goto(url); pg.add_style_tag(content="html{scroll-behavior:auto!important}"); pg.wait_for_timeout(1500)
        pg.screenshot(path=str(out / f"{name}-0.png"))
        if name == "d" or "full" in sys.argv:
            total = pg.evaluate("document.body.scrollHeight")
            y = 0; i = 1
            while y < total and i < 14:
                y += h
                pg.evaluate(f"window.scrollTo(0,{y})"); pg.wait_for_timeout(500)
                pg.screenshot(path=str(out / f"{name}-{i}.png")); i += 1
        print(name, "errors:", errs)
    b.close()
