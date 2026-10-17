import asyncio, json, sys
from pathlib import Path
from playwright.async_api import async_playwright
root = Path(sys.argv[1]).resolve()
url = (root/'index.html').as_uri()
out = root/'.impeccable'/'review'
async def main():
    async with async_playwright() as p:
        b = await p.chromium.launch()
        res = {}
        for name,(w,h) in {'desktop':(1440,900),'tablet':(768,1024),'mobile':(375,812)}.items():
            ctx = await b.new_context(viewport={'width':w,'height':h}, device_scale_factor=1)
            pg = await ctx.new_page()
            errs=[]
            pg.on('console', lambda m: errs.append(m.text) if m.type=='error' else None)
            pg.on('pageerror', lambda e: errs.append(str(e)))
            await pg.add_init_script("window.__cls=0;new PerformanceObserver(l=>{for(const e of l.getEntries()){if(!e.hadRecentInput)window.__cls+=e.value}}).observe({type:'layout-shift',buffered:true});")
            await pg.goto(url, wait_until='networkidle')
            await pg.evaluate('document.fonts.ready')
            await pg.mouse.move(w*0.4, 200)
            await pg.wait_for_timeout(2500)
            await pg.screenshot(path=str(out/f'{name}-hero.png'))
            ov = await pg.evaluate('document.documentElement.scrollWidth - innerWidth')
            # scroll through page to exercise scroll sequences
            H = await pg.evaluate('document.body.scrollHeight')
            y=0
            while y < H:
                await pg.mouse.wheel(0, 600); y+=600; await pg.wait_for_timeout(60)
            await pg.wait_for_timeout(400)
            cls = await pg.evaluate('window.__cls')
            await pg.evaluate('scrollTo(0,0)'); await pg.wait_for_timeout(500)
            await pg.screenshot(path=str(out/f'{name}.png'), full_page=True)
            res[name]={'overflowX':ov,'cls':round(cls,4),'errors':errs,'height':H}
            await ctx.close()
        print(json.dumps(res, indent=1))
        await b.close()
asyncio.run(main())
