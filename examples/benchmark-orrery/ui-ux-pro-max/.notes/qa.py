import pathlib, json
from playwright.sync_api import sync_playwright
here = pathlib.Path(__file__).parent
url = (here.parent / "index.html").as_uri()
out = {}
with sync_playwright() as p:
    b = p.chromium.launch()
    for name, w, h in [("desktop",1440,900),("tablet",768,1024),("mobile",375,812)]:
        pg = b.new_page(viewport={"width":w,"height":h})
        errs = []
        pg.on("pageerror", lambda e: errs.append(str(e)))
        pg.on("console", lambda m: m.type=="error" and errs.append(m.text))
        pg.goto(url, wait_until="networkidle")
        pg.wait_for_timeout(3300)
        pg.screenshot(path=str(here/f"{name}-fold.png"))
        pg.screenshot(path=str(here/f"{name}-full.png"), full_page=True)
        r = {"errors": errs, "hscroll": pg.evaluate("document.documentElement.scrollWidth > innerWidth")}
        # basket
        pg.locator(".add-btn").nth(0).click(); pg.locator(".add-btn").nth(2).click()
        r["basket"] = pg.inner_text("#basket-count")
        # empty submit
        pg.locator("#submit-btn").click()
        r["errors_shown"] = pg.locator(".field.invalid").count()
        r["summary_focused"] = pg.evaluate("document.activeElement.id")
        pg.fill("#f-name","Ada Lovelace"); pg.fill("#f-email","bad"); pg.locator("#f-email").blur()
        r["email_err"] = pg.inner_text("#f-email-err")
        pg.fill("#f-email","ada@example.com")
        pg.select_option("#f-show","Deep Field")
        r["date_opts"] = pg.locator("#f-date option").count()-1
        pg.select_option("#f-date", index=3)
        pg.locator("label[for=t3]").click()
        pg.fill("#f-tickets","3")
        r["total"] = pg.inner_text("#total")
        pg.locator("#submit-btn").click(); pg.wait_for_timeout(900)
        r["success"] = pg.is_visible("#booking-success")
        r["success_text"] = pg.inner_text("#success-details")
        pg.locator("#booking-success").screenshot(path=str(here/f"{name}-success.png"))
        # faq
        pg.locator(".faq-btn").first.click(); pg.wait_for_timeout(400)
        r["faq"] = pg.get_attribute(".faq-btn >> nth=0","aria-expanded")
        r["fridays"] = pg.locator("#friday-list .date-card strong").all_inner_texts()
        if w < 961:
            pg.evaluate("scrollTo(0,0)")
            pg.locator(".menu-toggle").click(); pg.wait_for_timeout(350)
            r["menu_visible"] = pg.is_visible("#primary-nav a[href='#shop']")
            pg.screenshot(path=str(here/f"{name}-menu.png"))
            pg.keyboard.press("Escape"); pg.wait_for_timeout(300)
            r["menu_closed"] = pg.get_attribute(".menu-toggle","aria-expanded")
        out[name] = r
        pg.close()
    # reduced motion
    ctx = b.new_context(reduced_motion="reduce", viewport={"width":1440,"height":900})
    pg = ctx.new_page(); pg.goto(url, wait_until="networkidle")
    out["reduced_motion_reveal_off"] = not pg.evaluate("document.documentElement.classList.contains('reveal-on')")
    out["reduced_anim"] = pg.evaluate("getComputedStyle(document.querySelector('.s1')).animationName")
    b.close()
print(json.dumps(out, indent=1))
