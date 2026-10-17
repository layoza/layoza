from playwright.sync_api import sync_playwright
url = 'examples/benchmark-orrery/taste-skill/index.html'
out = 'examples/benchmark-orrery/taste-skill/.notes/'
with sync_playwright() as p:
    b = p.chromium.launch()
    pg = b.new_page(viewport={'width':1440,'height':900}); errs=[]
    pg.on('pageerror', lambda e: errs.append(str(e)))
    pg.goto(url); pg.wait_for_timeout(800)
    # basket
    adds = pg.locator('.add'); print('add buttons', adds.count())
    adds.nth(0).click(); adds.nth(3).click(); pg.locator('[data-join]').nth(1).click()
    print('count', pg.text_content('#count'))
    pg.click('.basket-btn'); print('basket list', pg.inner_text('#basket-list').replace('\n',' | '), pg.text_content('#basket-sum'))
    pg.keyboard.press('Escape')
    # form: empty submit
    pg.locator('#book').scroll_into_view_if_needed(); pg.click('.submit'); pg.wait_for_timeout(200)
    print('errors shown', [pg.text_content('#e-'+k) for k in ['name','email','show','date','time','qty']])
    print('focused', pg.evaluate('document.activeElement.id'))
    pg.fill('#f-name','Ada'); pg.fill('#f-email','ada@'); pg.locator('#f-email').blur(); print('email err', pg.text_content('#e-email'))
    pg.fill('#f-email','ada@example.com')
    pg.select_option('#f-show','clockwork')
    opts = pg.evaluate("[...document.querySelectorAll('#f-date option')].map(o=>o.textContent+(o.disabled?' [x]':''))"); print(len(opts)-1, 'dates', opts[1], '...', opts[-1])
    pg.select_option('#f-date', index=3); pg.wait_for_timeout(1000); print('orrery label', pg.text_content('#orrery-date'))
    pg.check('input[value="19:30"]', force=True); pg.click('[data-step="1"]')
    print('total', pg.text_content('#f-total'))
    pg.click('.submit'); pg.wait_for_timeout(300)
    print('success visible', pg.is_visible('#success'), pg.text_content('#success-text'))
    # accordion
    pg.locator('#faq summary').first.click(); pg.wait_for_timeout(400); print('acc open', pg.evaluate("document.querySelector('#faq details').open"))
    # fridays
    print('fridays', pg.evaluate("[...document.querySelectorAll('#fridays strong')].map(s=>s.textContent)"))
    # turn
    pg.locator('#top').scroll_into_view_if_needed(); pg.click('[data-turn="1"]'); pg.wait_for_timeout(900); print('turned', pg.text_content('#orrery-date'))
    # mobile
    m = b.new_page(viewport={'width':375,'height':812}, is_mobile=True, has_touch=True)
    m.goto(url); m.wait_for_timeout(1500)
    m.screenshot(path=out+'m-fold.png')
    m.click('.menu-btn'); m.wait_for_timeout(400); print('menu open', m.is_visible('#sheet-nav'), m.evaluate('document.activeElement.textContent'))
    m.screenshot(path=out+'m-menu.png')
    m.click('#sheet-nav a[href="#shop"]'); m.wait_for_timeout(1200); print('menu closed', not m.is_visible('#sheet-nav'))
    m.locator('.add').nth(2).click(); print('m count', m.text_content('#count'))
    print('overflow', m.evaluate('document.documentElement.scrollWidth'), 'errors', errs)
    t = b.new_page(viewport={'width':768,'height':1024}); t.goto(url); t.wait_for_timeout(1500); t.screenshot(path=out+'t-fold.png')
    b.close()
