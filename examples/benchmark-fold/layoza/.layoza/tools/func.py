import sys, pathlib
from playwright.sync_api import sync_playwright
f=pathlib.Path(sys.argv[1]).resolve().as_uri()
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={'width':1440,'height':900}); pg.goto(f)
    for t in ['housing','civic','extensions','all']:
        pg.click(f'.filters [data-filter={t}]')
        print(t, pg.locator('#register li:visible').count(), pg.inner_text('#count'))
    pg.click('.send'); print('errors:', [e for e in pg.locator('.error').all_inner_texts() if e])
    print('focused:', pg.evaluate('document.activeElement.id'))
    pg.fill('#f-name','Ann Test'); pg.fill('#f-email','ann@'); pg.locator('#f-email').blur(); print('email err:', pg.inner_text('#e-email'))
    pg.fill('#f-email','ann@example.com'); pg.select_option('#f-type',index=1); pg.select_option('#f-budget',index=2); pg.fill('#f-msg','A tenement flat in Hillhead.')
    pg.click('.send'); print('success visible:', pg.is_visible('#success'), pg.inner_text('#success-text'))
    m=b.new_page(viewport={'width':375,'height':812}); m.goto(f)
    m.click('.menu-btn'); print('menu open:', m.get_attribute('.menu-btn','aria-expanded'), m.is_visible('#site-nav a[href="#work"]'))
    m.keyboard.press('Escape'); print('after esc:', m.get_attribute('.menu-btn','aria-expanded'), m.is_visible('#site-nav a[href="#work"]'))
    b.close()
