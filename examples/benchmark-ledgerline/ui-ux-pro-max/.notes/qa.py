import sys, json, pathlib
from playwright.sync_api import sync_playwright
here = pathlib.Path(__file__).parent
url = (here.parent / "index.html").resolve().as_uri()
out = {}
with sync_playwright() as p:
    b = p.chromium.launch()
    for name, w, h in [("desktop",1440,900),("tablet",768,1024),("mobile",375,812)]:
        pg = b.new_page(viewport={"width":w,"height":h})
        errs = []
        pg.on("pageerror", lambda e: errs.append(str(e)))
        pg.goto(url); pg.wait_for_timeout(1200)
        pg.screenshot(path=str(here/f"{name}-fold.png"))
        # reveal all
        pg.evaluate("document.querySelectorAll('.reveal').forEach(e=>e.classList.add('in'))")
        pg.screenshot(path=str(here/f"{name}-full.png"), full_page=True)
        sw = pg.evaluate("document.documentElement.scrollWidth")
        out[name] = {"scrollWidth": sw, "overflow": sw > w, "errors": errs}
        if name == "mobile":
            pg.click(".menu-btn"); pg.wait_for_timeout(400)
            pg.screenshot(path=str(here/"mobile-menu.png"))
            out[name]["menuOpen"] = pg.get_attribute(".menu-btn","aria-expanded")
            pg.keyboard.press("Escape")
        if name == "desktop":
            pg.fill("#clients-n","100"); pg.fill("#tx-n","90")
            out[name]["hours_100x90"] = pg.text_content("#hours")  # expect 100
            pg.click("[data-bill=yearly]")
            out[name]["yearly"] = pg.eval_on_selector_all(".plan .v","els=>els.map(e=>e.textContent)")
            out[name]["billed"] = pg.text_content(".plan .billed")
            pg.click("#q2"); pg.wait_for_timeout(400)
            out[name]["faqOpen"] = pg.get_attribute("#q2","aria-expanded")
            pg.click("#submit-btn")
            out[name]["errSummary"] = pg.eval_on_selector_all("#err-list li","els=>els.map(e=>e.textContent)")
            pg.locator("#trial").screenshot(path=str(here/"form-errors.png"))
            pg.fill("#f-name","Test User"); pg.fill("#f-email","test@example.co.uk"); pg.fill("#f-firm","Example Firm"); pg.select_option("#f-size","3-10")
            pg.click("#submit-btn"); pg.wait_for_timeout(1300)
            out[name]["success"] = pg.is_visible("#success")
            pg.locator("#trial").screenshot(path=str(here/"form-success.png"))
        pg.close()
    b.close()
print(json.dumps(out, indent=1))
