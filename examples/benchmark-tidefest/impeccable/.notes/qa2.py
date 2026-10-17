import asyncio, json
from pathlib import Path
from playwright.async_api import async_playwright
root=Path('.').resolve(); url=(root/'index.html').as_uri(); out=root/'.notes'/'shots'; out.mkdir(exist_ok=True)
async def m():
  async with async_playwright() as p:
    b=await p.chromium.launch(); r={}
    pg=await b.new_page(viewport={'width':1440,'height':900})
    await pg.goto(url); await pg.add_style_tag(content='html{scroll-behavior:auto!important}'); await pg.evaluate('document.fonts.ready'); await pg.wait_for_timeout(600)
    # sunrise mid
    for f in (0.3,0.6,0.95):
      await pg.evaluate(f"(()=>{{const s=document.getElementById('sunrise');scrollTo(0,s.offsetTop+(s.offsetHeight-innerHeight)*{f})}})()"); await pg.wait_for_timeout(700)
      r[f'hdr-sun{f}']=await pg.evaluate("document.getElementById('bar').getBoundingClientRect().top")
      await pg.screenshot(path=str(out/f'd-sunrise-{f}.png'))
    await pg.click('.days button[data-day=sat]'); await pg.wait_for_timeout(800)
    r['sat']=await pg.evaluate("[...document.querySelectorAll('.act')].filter(a=>!a.hidden).map(a=>a.querySelector('.act__name').textContent)")
    await pg.evaluate("document.getElementById('lineup').scrollIntoView({behavior:'instant'})"); await pg.wait_for_timeout(600); await pg.screenshot(path=str(out/'d-lineup.png'))
    await pg.click('.board--wknd [data-d="1"]'); await pg.click('.board--wknd [data-d="1"]'); await pg.click('.board--camp [data-d="1"]')
    r['total']=await pg.text_content('#total')
    await pg.evaluate("document.getElementById('tickets').scrollIntoView({behavior:'instant'})"); await pg.wait_for_timeout(600); await pg.screenshot(path=str(out/'d-tickets.png'))
    await pg.click('#qb3'); await pg.wait_for_timeout(500)
    r['faq']=await pg.evaluate("[document.getElementById('qb3').getAttribute('aria-expanded'), document.getElementById('qa3').hidden]")
    await pg.evaluate("document.getElementById('camping').scrollIntoView({behavior:'instant'})"); await pg.wait_for_timeout(600); await pg.screenshot(path=str(out/'d-camping.png'))
    await pg.fill('#email','nope@'); await pg.click('#news-form button'); r['err']=await pg.text_content('#email-err')
    await pg.fill('#email','sam@example.com'); await pg.click('#news-form button'); r['ok']=await pg.text_content('#news-box')
    await pg.evaluate("document.querySelector('.news').scrollIntoView({behavior:'instant'})"); await pg.wait_for_timeout(600); await pg.screenshot(path=str(out/'d-news.png'))
    # hero picker
    await pg.evaluate('scrollTo(0,0)'); await pg.click('input[value=camp]'); await pg.click('#hero-buy'); await pg.wait_for_timeout(900)
    r['heroBuy']=await pg.evaluate("[document.activeElement.getAttribute('aria-label'), document.getElementById('total').textContent]")
    # mobile
    mp=await b.new_page(viewport={'width':375,'height':812}, is_mobile=True, has_touch=True)
    await mp.goto(url); await mp.add_style_tag(content='html{scroll-behavior:auto!important}'); await mp.evaluate('document.fonts.ready'); await mp.wait_for_timeout(600)
    await mp.click('.bar .menu-btn'); await mp.wait_for_timeout(600); await mp.screenshot(path=str(out/'m-menu.png'))
    r['menuFocus']=await mp.evaluate("document.activeElement.getAttribute('aria-label')")
    await mp.keyboard.press('Escape'); await mp.wait_for_timeout(400)
    r['menuClosed']=await mp.evaluate("document.querySelector('.bar .menu-btn').getAttribute('aria-expanded')")
    for sel in ['.count','#lineup','#tickets','#camping','.news']:
      await mp.evaluate(f"document.querySelector('{sel}').scrollIntoView({{behavior:'instant'}})"); await mp.wait_for_timeout(700)
      r['hdr-'+sel]=await mp.evaluate("document.getElementById('bar').getBoundingClientRect().top")
      await mp.screenshot(path=str(out/f"m-{sel.strip('#.')}.png"))
    # reduced motion
    rc=await b.new_context(viewport={'width':1440,'height':900}, reduced_motion='reduce'); rp=await rc.new_page()
    await rp.goto(url); await rp.add_style_tag(content='html{scroll-behavior:auto!important}'); await rp.evaluate('document.fonts.ready'); await rp.wait_for_timeout(500)
    r['rm']=await rp.evaluate("[document.documentElement.classList.contains('is-static'), getComputedStyle(document.querySelector('.sunrise__stage')).position, document.querySelector('[data-motion-toggle]').hidden]")
    await rp.evaluate("document.getElementById('sunrise').scrollIntoView({behavior:'instant'})"); await rp.wait_for_timeout(600); await rp.screenshot(path=str(out/'rm-sunrise.png'))
    await rp.evaluate("document.querySelector('.news').scrollIntoView({behavior:'instant'})"); await rp.wait_for_timeout(300)
    await rp.fill('#email','bad'); await rp.click('#news-form button'); await rp.screenshot(path=str(out/'d-news-error.png'))
    await rp.fill('#email','sam@example.com'); await rp.click('#news-form button'); await rp.wait_for_timeout(200); await rp.screenshot(path=str(out/'d-news-success.png'))
    print(json.dumps(r,indent=1)); await b.close()
asyncio.run(m())
