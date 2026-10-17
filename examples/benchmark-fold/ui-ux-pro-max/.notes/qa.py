import pathlib, json
from playwright.sync_api import sync_playwright

here = pathlib.Path(__file__).resolve().parent
url = (here.parent / "index.html").as_uri()
out = {}
with sync_playwright() as p:
    b = p.chromium.launch()
    errors = []
    for name, w, h in [("desktop", 1440, 900), ("tablet", 768, 1024), ("mobile", 375, 812)]:
        pg = b.new_page(viewport={"width": w, "height": h})
        pg.on("pageerror", lambda e: errors.append(str(e)))
        pg.on("console", lambda m: m.type == "error" and errors.append(m.text))
        pg.goto(url); pg.wait_for_load_state("networkidle")
        pg.screenshot(path=str(here / f"{name}-fold.png"))
        pg.screenshot(path=str(here / f"{name}-full.png"), full_page=True)
        out[name] = {"hscroll": pg.evaluate("document.documentElement.scrollWidth > innerWidth")}
        if name == "mobile":
            pg.click(".menu-btn"); out[name]["nav_open"] = pg.is_visible("#site-nav a[href='#work']")
            pg.screenshot(path=str(here / "mobile-menu.png"))
            pg.keyboard.press("Escape"); out[name]["nav_closed_esc"] = not pg.is_visible("#site-nav a[href='#work']")
        pg.close()
    pg = b.new_page(viewport={"width": 1440, "height": 900})
    pg.goto(url)
    res = {}
    for f in ["housing", "civic", "extensions", "all"]:
        pg.click(f".chip[data-filter={f}]")
        res[f] = [pg.locator(".project:visible").count(), pg.text_content("#count")]
    out["filter"] = res
    pg.click("#enquiry button[type=submit]")
    out["empty_submit_errors"] = pg.locator(".err:visible").count()
    out["summary_focused"] = pg.evaluate("document.activeElement.id")
    pg.fill("#email", "bad"); pg.locator("#email").blur()
    out["email_err"] = pg.text_content("#email-err")
    pg.fill("#name", "Ann Lee"); pg.fill("#email", "ann@example.com")
    pg.select_option("#ptype", index=1); pg.select_option("#budget", index=2)
    pg.fill("#message", "A tenement flat in Govanhill that needs insulating.")
    pg.click("#enquiry button[type=submit]")
    out["success_visible"] = pg.is_visible("#success")
    out["success_text"] = pg.text_content("#success h3")
    pg.locator("#contact").screenshot(path=str(here / "contact-success.png"))
    out["errors"] = errors
    b.close()
print(json.dumps(out, indent=1))
