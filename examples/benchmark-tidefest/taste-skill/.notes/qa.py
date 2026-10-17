import sys
from playwright.sync_api import sync_playwright
from pathlib import Path
here = Path(__file__).parent
url = (here.parent / 'index.html').resolve().as_uri()
with sync_playwright() as p:
    b = p.chromium.launch()
    for (w,h,name) in [(1440,900,'d'),(768,1024,'t'),(375,812,'m')]:
        pg = b.new_page(viewport={'width':w,'height':h})
        errs=[]; pg.on('pageerror', lambda e: errs.append(str(e))); pg.on('console', lambda m: errs.append(m.text) if m.type=='error' else None)
        pg.goto(url); pg.wait_for_timeout(1500)
        pg.screenshot(path=str(here/f'{name}-fold.png'))
        # day filter
        pg.click('.days button[data-day="sat"]'); pg.wait_for_timeout(300)
        vis = pg.evaluate("[...document.querySelectorAll('.day')].map(d=>d.dataset.day+':'+!d.hidden)")
        # tickets
        pg.locator('#tickets').scroll_into_view_if_needed(); pg.wait_for_timeout(500)
        for sel in ['.type[data-type=weekend] [data-step="1"]','.type[data-type=weekend] [data-step="1"]','.type[data-type=camping] [data-step="1"]','.type[data-type=day] [data-step="1"]']:
            pg.click(sel)
        pg.wait_for_timeout(700)
        tot = pg.text_content('#total'); summ = pg.text_content('#summary')
        pg.screenshot(path=str(here/f'{name}-tickets.png'))
        # faq
        pg.click('.q:nth-child(3) summary'); pg.wait_for_timeout(300)
        opens = pg.evaluate("[...document.querySelectorAll('.q')].map(q=>q.open)")
        # newsletter
        pg.fill('#email','bad@'); pg.click('#news button'); msg1 = pg.text_content('#news-msg')
        pg.fill('#email','me@example.com'); pg.click('#news button'); pg.wait_for_timeout(200)
        done = pg.text_content('.news__done')
        # countdown
        cd = pg.text_content('#cd-days'), pg.text_content('#cd-tides')
        # panel
        pg.locator('[data-scene=rise]').scroll_into_view_if_needed(); pg.wait_for_timeout(900)
        pg.screenshot(path=str(here/f'{name}-rise.png'))
        menu=None
        if w < 821:
            pg.evaluate('scrollTo(0,0)'); pg.click('.menu-btn'); pg.wait_for_timeout(400)
            menu = pg.get_attribute('.menu-btn','aria-expanded'), pg.evaluate('document.activeElement.textContent')
            pg.screenshot(path=str(here/f'{name}-menu.png'))
            pg.keyboard.press('Escape'); menu += (pg.get_attribute('.menu-btn','aria-expanded'),)
        sw = pg.evaluate('document.documentElement.scrollWidth')
        print(name, vis, tot, summ, opens, msg1, done, cd, menu, 'sw', sw, 'errs', errs)
        pg.close()
    # reduced motion
    ctx = b.new_context(viewport={'width':1440,'height':900}, reduced_motion='reduce')
    pg = ctx.new_page(); pg.goto(url); pg.wait_for_timeout(1200)
    pg.screenshot(path=str(here/'d-reduced.png'))
    pg.evaluate('scrollTo(0, document.querySelector("#countdown").offsetTop)'); pg.wait_for_timeout(500)
    print('reduced: hidden reveal count', pg.evaluate("[...document.querySelectorAll('[data-reveal]')].filter(e=>getComputedStyle(e).opacity==='0' && e.getBoundingClientRect().top < innerHeight && e.getBoundingClientRect().bottom>0).length"))
    b.close()
