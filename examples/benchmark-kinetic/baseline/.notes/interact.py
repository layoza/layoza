import asyncio
from playwright.async_api import async_playwright
URL='examples/benchmark-kinetic/baseline/index.html'
OUT='examples/benchmark-kinetic/baseline/.notes/'
async def run():
    async with async_playwright() as p:
        b=await p.chromium.launch()
        # mobile menu
        ctx=await b.new_context(viewport={'width':375,'height':812},is_mobile=True,has_touch=True)
        pg=await ctx.new_page(); errs=[]
        pg.on('pageerror',lambda e:errs.append(str(e))); pg.on('console',lambda m:errs.append(m.text) if m.type=='error' else None)
        await pg.goto(URL,wait_until='networkidle'); await pg.wait_for_timeout(2000)
        await pg.click('.menu-toggle'); await pg.wait_for_timeout(1200)
        await pg.screenshot(path=OUT+'i-menu.png')
        print('menu expanded', await pg.get_attribute('.menu-toggle','aria-expanded'))
        await pg.keyboard.press('Escape'); await pg.wait_for_timeout(800)
        print('menu after esc', await pg.get_attribute('.menu-toggle','aria-expanded'), await pg.evaluate("getComputedStyle(document.getElementById('mobileMenu')).visibility"))
        # mobile services tap
        await pg.click('.svc:nth-child(4) .svc__btn'); await pg.wait_for_timeout(1200)
        print('svc active idx', await pg.text_content('#svcIdx'))
        print('errors mobile', errs)
        await ctx.close()
        # desktop
        ctx=await b.new_context(viewport={'width':1440,'height':900})
        pg=await ctx.new_page(); errs=[]
        pg.on('pageerror',lambda e:errs.append(str(e))); pg.on('console',lambda m:errs.append(m.text) if m.type=='error' else None)
        await pg.goto(URL,wait_until='networkidle'); await pg.wait_for_timeout(2000)
        await pg.click('.nav-main a[href="#services"]'); await pg.wait_for_timeout(1800)
        await pg.click('.svc:nth-child(3) .svc__btn'); await pg.wait_for_timeout(1800)
        print('desk svc idx', await pg.text_content('#svcIdx'))
        await pg.screenshot(path=OUT+'i-svc.png')
        await pg.evaluate("document.querySelector('[data-open=\"0\"]').click()"); await pg.wait_for_timeout(900)
        print('dialog open', await pg.evaluate("document.getElementById('caseDialog').open"))
        await pg.screenshot(path=OUT+'i-dialog.png')
        await pg.keyboard.press('Escape'); await pg.wait_for_timeout(400)
        print('dialog after esc', await pg.evaluate("document.getElementById('caseDialog').open"))
        # reel
        await pg.evaluate("document.getElementById('reelStage').scrollIntoView()"); await pg.wait_for_timeout(1500)
        t1=await pg.text_content('#reelTime'); await pg.wait_for_timeout(1200); t2=await pg.text_content('#reelTime')
        print('reel autoplay', t1, t2, await pg.get_attribute('#reelPlay','aria-pressed'))
        await pg.click('#reelPlay'); await pg.wait_for_timeout(300); t3=await pg.text_content('#reelTime'); await pg.wait_for_timeout(1200)
        print('reel paused', t3, await pg.text_content('#reelTime'), await pg.get_attribute('#reelPlay','aria-pressed'))
        await pg.click('[data-ch="1"]'); await pg.wait_for_timeout(300); print('chapter2', await pg.text_content('#reelTime'))
        await pg.screenshot(path=OUT+'i-reel.png')
        # insights accordion
        await pg.evaluate("document.querySelector('#insights').scrollIntoView()"); await pg.wait_for_timeout(1000)
        await pg.click('.ins:nth-child(1) .ins__btn'); await pg.wait_for_timeout(900)
        print('ins expanded', await pg.get_attribute('.ins:nth-child(1) .ins__btn','aria-expanded'), await pg.evaluate("document.getElementById('insp0').offsetHeight"))
        # form
        await pg.click('.nav-main a[href="#insights"]'); await pg.wait_for_timeout(500)
        await pg.click('.header-cta .btn'); await pg.wait_for_timeout(1900)
        print('focused after CTA', await pg.evaluate("document.activeElement.id"))
        await pg.click('#startProject'); await pg.wait_for_timeout(300)
        print('validation', await pg.text_content('#e-name'), '|', await pg.text_content('#e-email'), '|', await pg.evaluate("document.activeElement.id"))
        await pg.fill('#f-name','Test Person'); await pg.fill('#f-email','test@example.org'); await pg.fill('#f-msg','We need a new website for a product launch.')
        await pg.click('.chip:nth-child(2)')
        nav=[]
        pg.on('request', lambda r: nav.append(r.url))
        await pg.evaluate("window.__href=null; const d=Object.getOwnPropertyDescriptor(window,'location');")
        # intercept mailto by checking status + listening for navigation attempts
        await pg.click('#startProject'); await pg.wait_for_timeout(600)
        print('status', await pg.text_content('#formStatus'))
        await pg.screenshot(path=OUT+'i-contact.png')
        print('errors desk', errs)
        await ctx.close()
        # reduced motion
        ctx=await b.new_context(viewport={'width':1440,'height':900}, reduced_motion='reduce')
        pg=await ctx.new_page(); errs=[]
        pg.on('pageerror',lambda e:errs.append(str(e)))
        await pg.goto(URL,wait_until='networkidle'); await pg.wait_for_timeout(800)
        await pg.screenshot(path=OUT+'i-rm-hero.png')
        print('rm preload removed', await pg.evaluate("!document.documentElement.classList.contains('preload')"), 'pinned process', await pg.evaluate("document.getElementById('process').classList.contains('is-pinned')"))
        await pg.evaluate("document.getElementById('philosophy').scrollIntoView()"); await pg.wait_for_timeout(500)
        await pg.screenshot(path=OUT+'i-rm-philo.png')
        print('errors rm', errs)
        await ctx.close(); await b.close()
asyncio.run(run())
