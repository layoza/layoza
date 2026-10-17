import sys, pathlib
from playwright.sync_api import sync_playwright
root = pathlib.Path(sys.argv[1]).resolve()
url = (root / 'index.html').as_uri(); out = root / '.notes'
with sync_playwright() as p:
    b = p.chromium.launch()
    errs = []
    pg = b.new_page(viewport={'width':1440,'height':900})
    pg.on('pageerror', lambda e: errs.append(str(e))); pg.on('console', lambda m: m.type=='error' and errs.append(m.text))
    pg.goto(url); pg.wait_for_timeout(1500)
    # filter
    pg.click('[data-filter="VG"]'); print('VG:', pg.inner_text('#filter-status'), pg.eval_on_selector_all('.dish:not([hidden])', 'e=>e.length'))
    pg.click('[data-filter="GF"]'); print('GF:', pg.inner_text('#filter-status'))
    pg.click('[data-filter="V"]'); print('V:', pg.inner_text('#filter-status'))
    pg.click('[data-filter="all"]')
    # bakery
    pg.click('[data-loaf="4"]'); pg.wait_for_timeout(400)
    pg.locator('#bakery').scroll_into_view_if_needed(); pg.wait_for_timeout(1200)
    pg.screenshot(path=str(out/'bakery-croissant.png'))
    print('fact:', pg.inner_text('[data-fact-name]'))
    # scrubber
    pg.locator('#oven').scroll_into_view_if_needed(); pg.fill('#scrub', '420'); pg.dispatch_event('#scrub','input'); pg.wait_for_timeout(1200)
    print('scrub:', pg.inner_text('#scrub-out'), '|', pg.inner_text('#scrub-text'))
    pg.screenshot(path=str(out/'oven-0700.png'))
    # booking: empty submit
    pg.locator('#book').scroll_into_view_if_needed(); pg.wait_for_timeout(900)
    pg.click('#booking button[type=submit]')
    print('errors:', [pg.inner_text('#e-'+k) for k in ['name','email','party','date','time']], 'focused:', pg.evaluate('document.activeElement.id'))
    pg.screenshot(path=str(out/'booking-errors.png'))
    pg.fill('#f-name','Sam Taylor'); pg.fill('#f-email','sam@'); pg.locator('#f-email').blur(); print('email err:', pg.inner_text('#e-email'))
    pg.fill('#f-email','sam@example.com'); pg.select_option('#f-party','4')
    dates = pg.eval_on_selector_all('#f-date option', 'o=>o.map(x=>x.textContent)'); print('dates:', len(dates)-1, dates[1:4])
    pg.select_option('#f-date', index=2); 
    times = pg.eval_on_selector_all('#f-time option', 'o=>o.map(x=>x.value)'); print('times:', times[1:])
    pg.select_option('#f-time','19:30'); pg.fill('#f-notes','One coeliac.')
    pg.click('#booking button[type=submit]'); pg.wait_for_timeout(300)
    print('success:', pg.inner_text('[data-success-text]'))
    pg.screenshot(path=str(out/'booking-success.png'))
    # accordion
    pg.click('details.qa >> nth=1 >> summary'); pg.click('details.qa >> nth=2 >> summary')
    print('open:', pg.eval_on_selector_all('details.qa[open]', 'e=>e.length'))
    print('ERRORS', errs)
    # mobile
    m = b.new_page(viewport={'width':375,'height':812}, is_mobile=True, has_touch=True)
    m.goto(url); m.wait_for_timeout(1500)
    m.screenshot(path=str(out/'m-hero.png'))
    m.click('.menu-toggle'); m.wait_for_timeout(400); m.screenshot(path=str(out/'m-menu.png'))
    m.click('#drawer a[href="#bakery"]'); m.wait_for_timeout(1500)
    print('menu closed:', m.get_attribute('.menu-toggle','aria-expanded'))
    m.screenshot(path=str(out/'m-bakery.png'))
    print('overflow 375:', m.evaluate('document.documentElement.scrollWidth'))
    # reduced motion
    r = b.new_page(viewport={'width':1440,'height':900}, reduced_motion='reduce'); r.goto(url); r.wait_for_timeout(300)
    r.screenshot(path=str(out/'reduced.png'))
    b.close()
