import sys, pathlib
from playwright.sync_api import sync_playwright
f = pathlib.Path(sys.argv[1]).resolve().as_uri(); out = pathlib.Path(sys.argv[2])
with sync_playwright() as p:
    b = p.chromium.launch(); pg = b.new_page(viewport={'width':1440,'height':900})
    errs=[]; pg.on('console', lambda m: errs.append(m.text) if m.type=='error' else None); pg.on('pageerror', lambda e: errs.append(str(e)))
    pg.goto(f); pg.wait_for_timeout(1300); pg.screenshot(path=str(out/'mid-1300.png'))
    pg.wait_for_timeout(3000)
    pg.click('#reqBtn'); pg.wait_for_timeout(1800); pg.screenshot(path=str(out/'flag-resolved.png'))
    print('status', pg.inner_text('#sheetStatus'), '| box5', pg.inner_text('.box[data-b="5"] .bv'))
    # estimator
    pg.fill('#clientsN','300'); pg.fill('#txN','150'); pg.wait_for_timeout(600)
    print('hours', pg.inner_text('#hoursOut'), pg.inner_text('#txOut'), pg.inner_text('#daysOut'))
    pg.check('#by', force=True); pg.wait_for_timeout(500)
    print('prices', [t.inner_text() for t in pg.query_selector_all('.price')])
    pg.click('.qa button'); pg.wait_for_timeout(400); print('faq', pg.get_attribute('.qa button','aria-expanded'))
    pg.click('#tsubmit'); pg.wait_for_timeout(300)
    print('errors', [e.inner_text() for e in pg.query_selector_all('.err')])
    pg.fill('#fname','Sam Patel'); pg.fill('#femail','sam@patelco.co.uk'); pg.fill('#ffirm','Patel & Co'); pg.select_option('#fsize','3–10')
    pg.click('#tsubmit'); pg.wait_for_timeout(1500)
    el=pg.query_selector('#tformWrap'); el.scroll_into_view_if_needed(); el.screenshot(path=str(out/'success.png'))
    print('success', pg.inner_text('#okTitle'), pg.is_visible('#tform'))
    print('console errors', errs)
    b.close()
