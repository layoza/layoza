import pathlib
from playwright.sync_api import sync_playwright
u=pathlib.Path('index.html').resolve().as_uri()
with sync_playwright() as p:
    b=p.chromium.launch()
    for rm in ['no-preference','reduce']:
        pg=b.new_page(viewport={'width':375,'height':812},reduced_motion=rm); errs=[]
        pg.on('pageerror',lambda e: errs.append(str(e))); pg.goto(u); pg.wait_for_timeout(1500)
        vis=lambda: pg.eval_on_selector_all('.day',"e=>e.filter(x=>!x.hidden).map(x=>x.dataset.day)")
        r=[vis()]; pg.click('.filter [data-day=sun]'); r.append(vis()); pg.click('.filter [data-day=all]'); r.append(vis())
        pg.click('[aria-label="Add one Weekend ticket"]'); pg.click('[aria-label="Add one Weekend ticket"]'); pg.click('[aria-label="Add one Day ticket"]'); pg.wait_for_timeout(500)
        r.append(pg.inner_text('#basket-total')); r.append(pg.inner_text('#checkout'))
        pg.click('#q2b'); r.append(pg.get_attribute('#q2b','aria-expanded'))
        pg.fill('#email','bad'); pg.click('#news-form button'); r.append(pg.inner_text('#email-err'))
        pg.fill('#email','a@b.co'); pg.click('#news-form button'); r.append(pg.inner_text('#news-box')[:40])
        s1=pg.inner_text('#cd-s'); pg.wait_for_timeout(1100); r.append((s1,pg.inner_text('#cd-s'),pg.inner_text('#cd-d')))
        pg.click('.menu-toggle'); r.append(pg.is_visible('#mobile-menu')); pg.keyboard.press('Escape'); r.append(pg.is_visible('#mobile-menu'))
        print(rm, r, errs)
    b.close()
