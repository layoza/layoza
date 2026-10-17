import sys, pathlib
from playwright.sync_api import sync_playwright
root=pathlib.Path(__file__).resolve().parent.parent
url=(root/'index.html').as_uri(); out=root/'.notes'
with sync_playwright() as p:
    b=p.chromium.launch()
    for name,w,h in [('desktop',1440,900),('tablet',768,1024),('mobile',375,812)]:
        pg=b.new_page(viewport={'width':w,'height':h})
        errs=[]; pg.on('pageerror',lambda e: errs.append(str(e))); pg.on('console',lambda m: errs.append(m.text) if m.type=='error' else None)
        pg.goto(url); pg.wait_for_timeout(1200)
        pg.screenshot(path=str(out/f'{name}-fold.png'))
        H=pg.evaluate('document.body.scrollHeight')
        for y in range(0,H,400): pg.evaluate(f'window.scrollTo(0,{y})'); pg.wait_for_timeout(60)
        pg.wait_for_timeout(900)
        sw=pg.evaluate('document.documentElement.scrollWidth')
        pg.screenshot(path=str(out/f'{name}-full.png'),full_page=True)
        print(name,'scrollWidth',sw,'errors',errs)
        if name=='mobile':
            pg.evaluate('window.scrollTo(0,0)'); pg.click('.nav-cta .menu-toggle'); pg.wait_for_timeout(500)
            pg.screenshot(path=str(out/'mobile-menu.png'))
            print('drawer focused:',pg.evaluate('document.activeElement.getAttribute("aria-label")'))
            pg.keyboard.press('Escape'); pg.wait_for_timeout(400)
            print('after esc expanded:',pg.get_attribute('.nav-cta .menu-toggle','aria-expanded'))
        if name=='desktop':
            pg.click('button[data-filter="VG"]'); pg.wait_for_timeout(200)
            print(pg.inner_text('#menu-count'), 'visible dishes', pg.evaluate('[...document.querySelectorAll(".dish")].filter(d=>!d.hidden).length'))
            pg.locator('#menu').screenshot(path=str(out/'menu-vg.png'))
            pg.click('button[data-filter="all"]')
            pg.click('#submit'); pg.wait_for_timeout(300)
            print('summary items', pg.evaluate('document.querySelectorAll("#summary-list li").length'), 'focus', pg.evaluate('document.activeElement.id'))
            pg.locator('#form-card').screenshot(path=str(out/'form-errors.png'))
            pg.fill('#name','Sam Taylor'); pg.fill('#email','sam@example.com'); pg.select_option('#party','4')
            pg.select_option('#date',index=3); 
            opts=pg.evaluate('[...document.querySelectorAll("#time option")].filter(o=>!o.disabled&&o.value).map(o=>o.value)')
            print('date options', pg.evaluate('document.querySelectorAll("#date option").length-1'), 'times', opts)
            pg.select_option('#time',opts[0]); pg.click('#submit'); pg.wait_for_timeout(1000)
            print('success:', pg.inner_text('#success h3'))
            pg.locator('#form-card').screenshot(path=str(out/'form-success.png'))
            pg.click('#q2'); pg.wait_for_timeout(400); print('acc', pg.get_attribute('#q2','aria-expanded'))
        pg.close()
    b.close()
