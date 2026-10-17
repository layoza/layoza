import asyncio, sys
from playwright.async_api import async_playwright
URL='examples/benchmark-kinetic/baseline/index.html'
async def run():
    async with async_playwright() as p:
        b=await p.chromium.launch()
        for w,h,mob in [(375,812,True),(768,1024,False)]:
            ctx=await b.new_context(viewport={'width':w,'height':h},is_mobile=mob,has_touch=mob)
            pg=await ctx.new_page(); await pg.goto(URL,wait_until='networkidle'); await pg.wait_for_timeout(1500)
            print(w, await pg.evaluate('''({iw:innerWidth, cw:document.documentElement.clientWidth, sw:document.documentElement.scrollWidth, tog:document.querySelector('.menu-toggle').getBoundingClientRect().right, bar:document.querySelector('.site-header__bar').getBoundingClientRect().width,
              wide:[...document.querySelectorAll('main *, footer *')].filter(e=>{const r=e.getBoundingClientRect();return r.width>0 && r.right>document.documentElement.clientWidth+1 && !e.closest('svg') && !e.closest('.art')}).slice(0,6).map(e=>(e.className||e.tagName)+':'+Math.round(e.getBoundingClientRect().right))})'''))
            await ctx.close()
        await b.close()
asyncio.run(run())
