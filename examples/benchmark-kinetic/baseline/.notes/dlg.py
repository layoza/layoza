import asyncio
from playwright.async_api import async_playwright
async def run():
    async with async_playwright() as p:
        b=await p.chromium.launch(); pg=await b.new_page(viewport={'width':1440,'height':900})
        await pg.goto('examples/benchmark-kinetic/baseline/index.html',wait_until='networkidle'); await pg.wait_for_timeout(1500)
        await pg.evaluate("document.querySelector('[data-open=\"0\"]').click()"); await pg.wait_for_timeout(900)
        print(await pg.evaluate("(()=>{const d=document.getElementById('caseDialog'),b=d.querySelector('.case__box');const r=d.getBoundingClientRect(),rb=b.getBoundingClientRect();return {d:[r.x,r.y,r.width,r.height],b:[rb.x,rb.y,rb.width,rb.height],op:getComputedStyle(b).opacity,tf:getComputedStyle(b).transform, dv:getComputedStyle(d).visibility, dop:getComputedStyle(d).opacity}})()"))
        await pg.screenshot(path='dlg.png')
        await b.close()
asyncio.run(run())
