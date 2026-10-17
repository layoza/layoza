import sys, pathlib
from playwright.sync_api import sync_playwright

here = pathlib.Path(__file__).resolve().parent
page_url = (here.parent / "index.html").as_uri()
out = here / "shots"
out.mkdir(exist_ok=True)
sizes = {"desktop": (1440, 900), "tablet": (768, 1024), "mobile": (375, 812)}
only = sys.argv[1:] or list(sizes)

with sync_playwright() as p:
    b = p.chromium.launch(args=["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"])
    for name in only:
        w, h = sizes[name]
        pg = b.new_page(viewport={"width": w, "height": h})
        logs = []
        pg.on("console", lambda m: logs.append(f"{m.type}: {m.text}"))
        pg.on("pageerror", lambda e: logs.append(f"pageerror: {e}"))
        pg.goto(page_url, wait_until="networkidle")
        pg.wait_for_timeout(2500)
        print(name, "no-webgl" if pg.evaluate("document.documentElement.classList.contains('no-webgl')") else "webgl",
              "scrollW", pg.evaluate("document.documentElement.scrollWidth"))
        pg.screenshot(path=str(out / f"{name}-0-hero.png"))
        for i, step in enumerate(["battery", "belt", "weight", "colour"], 1):
            pg.evaluate(f"document.querySelector('[data-step={step}]').scrollIntoView({{block:'center', behavior:'instant'}})")
            pg.wait_for_timeout(1800)
            pg.screenshot(path=str(out / f"{name}-{i}-{step}.png"))
        pg.check("#c-clay")
        pg.wait_for_timeout(1200)
        pg.screenshot(path=str(out / f"{name}-5-clay.png"))
        pg.screenshot(path=str(out / f"{name}-full.png"), full_page=True)
        for l in logs:
            print("  ", l)
        pg.close()
    b.close()
