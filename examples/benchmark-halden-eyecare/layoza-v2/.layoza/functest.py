import pathlib
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).resolve().parent.parent/"index.html").as_uri()
out=pathlib.Path(__file__).resolve().parent/"shots"
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={"width":1440,"height":900})
    errs=[]; pg.on("pageerror",lambda e:errs.append(str(e)))
    pg.goto(url,wait_until="networkidle")
    pg.click(".frame >> nth=0 >> [data-add]"); pg.click(".frame >> nth=3 >> [data-add]"); pg.click(".pkg >> nth=1 >> [data-choose]")
    pg.wait_for_timeout(800); print("bag count", pg.inner_text("#bag-count"))
    pg.click(".pkg >> nth=2 >> [data-choose]"); print("after switching lens", pg.inner_text("#bag-count"))
    pg.click("#bag-btn"); pg.wait_for_timeout(300); pg.screenshot(path=str(out/"f-bag.png")); print("bag items", pg.inner_text("#bag-items").replace("\n"," | "))
    pg.keyboard.press("Escape")
    pg.click("#book-form button[type=submit]"); pg.wait_for_timeout(200)
    print("errors shown", [pg.inner_text("#e-"+k) for k in ["name","email","type","day","slot"]])
    pg.screenshot(path=str(out/"f-form-err.png"))
    pg.click(".slot >> nth=0"); pg.wait_for_timeout(900)
    print("prefill", pg.input_value("#f-type"), pg.input_value("#f-day"))
    pg.fill("#f-name","Test Person"); pg.fill("#f-email","bad")
    pg.click("#book-form button[type=submit]"); print("email err", pg.inner_text("#e-email"))
    pg.fill("#f-email","test@example.com"); pg.click("#book-form button[type=submit]"); pg.wait_for_timeout(500)
    print("success visible", pg.is_visible("#success")); pg.screenshot(path=str(out/"f-success.png"))
    # evening on non-thursday
    pg.click("#book-again"); pg.select_option("#f-type","nhs")
    opts=pg.eval_on_selector_all("#f-day option","os=>os.filter(o=>!o.disabled&&o.value).map(o=>[o.value,o.textContent])")
    nonthu=[o for o in opts if "evenings" not in o[1]][0][0]
    pg.select_option("#f-day",nonthu); pg.check("input[value=evening]",force=True); print("evening err", pg.inner_text("#e-slot"))
    q=pg.locator(".faq-q").nth(1); print("faq before", q.get_attribute("aria-expanded")); q.click(); print("faq after", q.get_attribute("aria-expanded"))
    print("page errors", errs)
    # no JS
    c=b.new_context(java_script_enabled=False,viewport={"width":1440,"height":900}); n=c.new_page(); n.goto(url,wait_until="networkidle")
    n.screenshot(path=str(out/"nojs-full.png"),full_page=True)
    print("nojs faq visible", n.is_visible("#fa3 p"), "h1", n.is_visible("h1"))
    b.close()
