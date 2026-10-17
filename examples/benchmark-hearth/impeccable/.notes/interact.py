import sys
from pathlib import Path
from playwright.sync_api import sync_playwright
D=Path(sys.argv[1]); url=(D/'index.html').as_uri(); out=D/'.impeccable'/'review'
with sync_playwright() as p:
    b=p.chromium.launch()
    pg=b.new_page(viewport={'width':1280,'height':800}); pg.goto(url); pg.evaluate('document.fonts.ready'); pg.wait_for_timeout(500)
    pg.screenshot(path=str(out/'user-1280.png'))
    pg.click('.chip-btn[data-tag=VG]'); print('VG:',pg.inner_text('[data-count]'), pg.locator('.dish:not([hidden])').count())
    pg.click('.chip-btn[data-tag=V]'); print('V:',pg.inner_text('[data-count]'))
    pg.click('#booking button[type=submit]'); print('errors shown:',pg.locator('#booking .err.show').count(), 'focused:',pg.evaluate('document.activeElement.id'))
    pg.fill('#b-name','Sam Taylor'); pg.fill('#b-email','sam@'); pg.locator('#b-email').blur(); print('email err:',pg.inner_text('#e-email'))
    pg.fill('#b-email','sam@example.com'); pg.click('label[for=p4]')
    pg.select_option('#b-date', index=2); pg.select_option('#b-time','19:00')
    pg.click('#booking button[type=submit]'); pg.wait_for_timeout(900)
    print('success:',pg.is_visible('[data-success]'), pg.inner_text('[data-s-head]'))
    pg.locator('[data-success]').screenshot(path=str(D/'.notes'/'success.png'))
    pg.click('#faq2-b'); print('faq expanded:',pg.get_attribute('#faq2-b','aria-expanded'))
    pg.fill('#b-name','x') if False else None
    pg.click('.phases button[data-ph="1"]'); print('oven:',pg.inner_text('[data-clock]'),pg.inner_text('[data-phase-name]'))
    m=b.new_page(viewport={'width':375,'height':812}); m.goto(url); m.wait_for_timeout(300)
    m.click('.head-plate .menu-btn'); m.wait_for_timeout(600); print('drawer open:',m.get_attribute('.head-plate .menu-btn','aria-expanded'))
    m.screenshot(path=str(out/'mobile-menu-open.png'))
    m.keyboard.press('Escape'); m.wait_for_timeout(500); print('drawer after esc:',m.get_attribute('.head-plate .menu-btn','aria-expanded'))
    b.close()
