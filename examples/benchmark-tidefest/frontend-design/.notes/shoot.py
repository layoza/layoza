import sys, pathlib
from playwright.sync_api import sync_playwright

here = pathlib.Path(__file__).resolve().parent
url = (here.parent / "index.html").as_uri()
out = here / "shots"
out.mkdir(exist_ok=True)

with sync_playwright() as p:
    b = p.chromium.launch()
    errors = []
    for name, w, h in [("desktop", 1440, 900), ("tablet", 768, 1024), ("mobile", 375, 812)]:
        for rm in ["no-preference", "reduce"]:
            pg = b.new_page(viewport={"width": w, "height": h}, reduced_motion=rm)
            pg.on("console", lambda m: m.type == "error" and errors.append(m.text))
            pg.on("pageerror", lambda e: errors.append(str(e)))
            pg.goto(url, wait_until="networkidle")
            pg.wait_for_timeout(600)
            tag = f"{name}-{rm}"
            pg.screenshot(path=str(out / f"{tag}-hero.png"))
            sw = pg.evaluate("document.documentElement.scrollWidth")
            print(tag, "scrollWidth", sw, "viewport", w)
            if rm == "no-preference":
                top = pg.evaluate("document.querySelector('.sunrise').offsetTop")
                hgt = pg.evaluate("document.querySelector('.sunrise').offsetHeight - innerHeight")
                for f in [0.15, 0.5, 0.85]:
                    pg.evaluate(f"window.scrollTo(0,{top + hgt * f})")
                    pg.wait_for_timeout(250)
                    pg.screenshot(path=str(out / f"{tag}-sun{int(f*100)}.png"))
            # interactions
            pg.click(".day-btn[data-day='3']")
            pg.click(".ticket:nth-child(2) [data-step='1']")
            pg.click(".ticket:nth-child(2) [data-step='1']")
            pg.click(".ticket:nth-child(3) [data-step='1']")
            pg.wait_for_timeout(600)
            print(tag, "total", pg.inner_text("#total"), "| visible sets", pg.evaluate("[...document.querySelectorAll('.set')].filter(s=>!s.hidden).length"))
            pg.click("#fq2")
            pg.fill("#email", "bad@")
            pg.click("#news-form button")
            print(tag, "err:", pg.inner_text("#email-msg"))
            pg.screenshot(path=str(out / f"{tag}-full.png"), full_page=True)
            pg.fill("#email", "me@example.com")
            pg.click("#news-form button")
            print(tag, "ok:", pg.inner_text("#news-slot"))
            if name == "mobile":
                pg.evaluate("scrollTo(0,0)")
                pg.click(".menu-btn")
                pg.wait_for_timeout(500)
                pg.screenshot(path=str(out / f"{tag}-menu.png"))
                pg.keyboard.press("Escape")
                print(tag, "menu after esc", pg.get_attribute(".menu-btn", "aria-expanded"))
            pg.close()
    print("errors:", errors)
    b.close()
