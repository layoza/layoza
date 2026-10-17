import pathlib
from playwright.sync_api import sync_playwright
root = pathlib.Path(__file__).resolve().parents[2]; url=(root/'index.html').as_uri(); out=root/'.layoza'/'shots'
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={'width':1440,'height':900}); errs=[]
    pg.on('console', lambda m: errs.append(m.text) if m.type=='error' else None); pg.on('pageerror', lambda e: errs.append(str(e)))
    pg.goto(url); pg.wait_for_timeout(500)
    pg.click('[data-add="Card orrery kit"]'); pg.click('[data-add="Moon-watching kit"]'); pg.locator('.tier button').first.click()
    print('count', pg.text_content('#basket-count'))
    pg.click('#basket-btn'); pg.wait_for_timeout(300); print('panel', pg.inner_text('#basket-panel')[:300].replace('\n',' | '))
    pg.screenshot(path=str(out/'d-basket.png'))
    pg.keyboard.press('Escape')
    pg.evaluate("document.getElementById('book').scrollIntoView()")
    pg.click('#book-form button[type=submit]'); pg.wait_for_timeout(200)
    print('errors', [pg.text_content('#e-'+k) for k in ['name','email','show','date','time','qty']])
    pg.screenshot(path=str(out/'d-book-err.png'))
    pg.fill('#f-name','Sam Test'); pg.fill('#f-email','sam@example.com'); pg.select_option('#f-show','saturn'); pg.select_option('#f-date', index=3); pg.click('#times label:nth-child(3)'); pg.click('[data-step="1"]')
    pg.wait_for_timeout(900); pg.screenshot(path=str(out/'d-book-fill.png')); print('mini', pg.inner_text('#mini-cap'))
    pg.click('#book-form button[type=submit]'); pg.wait_for_timeout(200); print('ok', pg.inner_text('#book-ok')[:250].replace('\n',' | '))
    pg.locator('.faq summary').nth(1).click(); print('faq open', pg.locator('.faq details').nth(1).get_attribute('open'))
    pg.fill('#sky-time','600'); pg.dispatch_event('#sky-time','input'); print('slider', pg.text_content('#sky-out'), pg.text_content('#sky-stars'))
    print('stubs', [t.replace('\n',' ') for t in pg.locator('.stub').all_inner_texts()])
    print('errs', errs); b.close()
