import asyncio
from playwright.async_api import async_playwright
JS = r"""document.addEventListener('focusout', e => console.log('blur ' + e.target.className + ' ' + new Error().stack.split('\n').slice(1,7).join(' | ')), true)"""
async def m():
    async with async_playwright() as p:
        b=await p.chromium.launch(); pg=await b.new_page(viewport={'width':1440,'height':900})
        await pg.goto('http://127.0.0.1:18942/index.html', wait_until='networkidle'); await pg.wait_for_timeout(1200)
        await pg.evaluate("document.querySelector('#work').scrollIntoView()"); await pg.wait_for_timeout(800)
        msgs=[]; pg.on('console', lambda c: msgs.append(c.text))
        await pg.evaluate(JS)
        await pg.click('[data-case="1"]')
        for t in (50,200,500,900):
            await pg.wait_for_timeout(t)
            print(t, await pg.evaluate("document.activeElement.className"))
        for x in msgs: print(x)
        await b.close()
asyncio.run(m())
