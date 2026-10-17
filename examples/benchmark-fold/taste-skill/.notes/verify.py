"""Functional check of index.html opened from the file system at the brief's three viewports."""
import pathlib
from playwright.sync_api import sync_playwright

here = pathlib.Path(__file__).resolve().parent
url = (here.parent / "index.html").as_uri()
shots = here / "shots"
shots.mkdir(exist_ok=True)
out = []

with sync_playwright() as p:
    b = p.chromium.launch()
    for w, h in [(1440, 900), (768, 1024), (375, 812)]:
        pg = b.new_page(viewport={"width": w, "height": h})
        errs = []
        pg.on("pageerror", lambda e: errs.append(str(e)))
        pg.on("console", lambda m: errs.append(m.text) if m.type == "error" else None)
        pg.goto(url, wait_until="networkidle")
        pg.screenshot(path=str(shots / f"{w}-fold.png"))
        overflow = pg.evaluate("document.documentElement.scrollWidth - document.documentElement.clientWidth")
        # filter
        pg.click("[data-filter=civic]")
        vis = pg.evaluate("[...document.querySelectorAll('.proj')].filter(x=>!x.hidden).map(x=>x.dataset.type)")
        cnt = pg.inner_text(".count")
        pg.click("[data-filter=all]")
        # hero jump link
        pg.click("[data-jump=extensions]")
        vis2 = pg.evaluate("[...document.querySelectorAll('.proj')].filter(x=>!x.hidden).map(x=>x.dataset.type)")
        pg.click("[data-filter=all]")
        # mobile menu
        menu = None
        if w < 700:
            pg.evaluate("window.scrollTo(0,0)")
            pg.click(".menu-btn")
            menu = pg.is_visible("#site-nav a[href='#studio']")
            pg.screenshot(path=str(shots / f"{w}-menu.png"))
            pg.click("#site-nav a[href='#studio']")
            menu = (menu, pg.get_attribute(".menu-btn", "aria-expanded"))
        # form: empty submit -> errors
        pg.click(".submit")
        errors = pg.evaluate("[...document.querySelectorAll('.err')].filter(e=>e.textContent).length")
        focused = pg.evaluate("document.activeElement.id")
        pg.fill("#f-name", "Ann Example")
        pg.fill("#f-email", "ann@")
        pg.click(".submit")
        email_err = pg.inner_text("#e-email")
        pg.fill("#f-email", "ann@example.com")
        pg.select_option("#f-type", "Timber extension")
        pg.select_option("#f-budget", "Not sure yet")
        pg.fill("#f-msg", "A kitchen extension to a 1930s semi in the south side.")
        pg.screenshot(path=str(shots / f"{w}-form-filled.png"), full_page=False)
        pg.click(".submit")
        done = pg.inner_text("#done") if pg.is_visible("#done") else None
        pg.locator("#done").scroll_into_view_if_needed()
        pg.screenshot(path=str(shots / f"{w}-success.png"))
        pg.screenshot(path=str(shots / f"{w}-full.png"), full_page=True)
        out.append(dict(w=w, overflow=overflow, civic=vis, count=cnt, jump=vis2, menu=menu, errors=errors,
                        focused=focused, email_err=email_err, done=done, console=errs))
        pg.close()
    b.close()
for o in out:
    print(o)
