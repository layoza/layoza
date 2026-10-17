import pathlib
from playwright.sync_api import sync_playwright
url = pathlib.Path('index.html').resolve().as_uri()
with sync_playwright() as p:
    b = p.chromium.launch(); pg = b.new_page(viewport={'width':1440,'height':900}); errs=[]
    pg.on('pageerror', lambda e: errs.append(str(e))); pg.on('console', lambda m: m.type=='error' and errs.append(m.text))
    pg.goto(url); pg.wait_for_timeout(2000)
    pg.click('label:has(input[value=sat])'); pg.wait_for_timeout(400)
    print('visible days', pg.eval_on_selector_all('.day:not([hidden])', 'e=>e.map(x=>x.dataset.day)'), pg.text_content('#board-status'))
    pg.click('[aria-label="Add one Weekend ticket"]'); pg.click('[aria-label="Add one Weekend ticket"]'); pg.click('[aria-label="Add one Weekend + camping ticket"]'); pg.wait_for_timeout(600)
    print('total', pg.text_content('#total'), '|', pg.text_content('#till-lines'))
    pg.click('#buy'); print('buy', pg.text_content('#till-msg'))
    print('countdown', [pg.text_content(s) for s in ['#cd-d','#cd-h','#cd-m','#cd-s']])
    pg.fill('#email','bad@'); pg.click('#news button'); print('err', pg.text_content('#email-hint'))
    pg.fill('#email','me@example.com'); pg.click('#news button'); pg.wait_for_timeout(300); print('done visible', pg.is_visible('#news-done'))
    pg.click('summary >> nth=0'); pg.wait_for_timeout(400); print('faq open', pg.eval_on_selector('details', 'd=>d.open'))
    m = b.new_page(viewport={'width':375,'height':812}); m.goto(url); m.wait_for_timeout(1500)
    m.click('.site-head .menu-btn'); print('drawer', m.is_visible('#drawer'), m.get_attribute('.site-head .menu-btn','aria-expanded'))
    m.keyboard.press('Escape'); print('closed', not m.is_visible('#drawer'))
    print('errors', errs); b.close()
