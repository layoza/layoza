import sys, asyncio, json
from playwright.async_api import async_playwright
URL = 'examples/benchmark-kinetic/baseline/index.html'
OUT = 'examples/benchmark-kinetic/baseline/.notes/'
async def run():
    async with async_playwright() as p:
        b = await p.chromium.launch(args=['--use-gl=angle','--enable-webgl','--ignore-gpu-blocklist'])
        for w,h in [(1440,900),(768,1024),(375,812)]:
            ctx = await b.new_context(viewport={'width':w,'height':h}, device_scale_factor=1, has_touch=(w<500), is_mobile=(w<500))
            pg = await ctx.new_page()
            errs=[]
            pg.on('console', lambda m: errs.append(m.type+': '+m.text) if m.type in ('error','warning') else None)
            pg.on('pageerror', lambda e: errs.append('PAGEERROR '+str(e)))
            await pg.goto(URL, wait_until='networkidle')
            await pg.wait_for_timeout(2800)
            await pg.screenshot(path=f'{OUT}{w}-hero.png')
            H = await pg.evaluate('document.documentElement.scrollHeight')
            ov = await pg.evaluate('document.documentElement.scrollWidth - innerWidth')
            n = 0; y = 0
            while y < H:
                y += int(h*0.9)
                await pg.mouse.wheel(0, int(h*0.9))
                await pg.wait_for_timeout(900)
                n+=1
                if n % 1 == 0 and n <= 40:
                    await pg.screenshot(path=f'{OUT}{w}-s{n:02d}.png')
                H = await pg.evaluate('document.documentElement.scrollHeight')
            wide = await pg.evaluate('''[...document.querySelectorAll('body *')].filter(e=>{const r=e.getBoundingClientRect();return r.right>innerWidth+1 && getComputedStyle(e).position!=='fixed'}).slice(0,8).map(e=>e.className+':'+Math.round(e.getBoundingClientRect().right))''')
            print(w, 'H', H, 'overflowX', ov, 'shots', n, 'wide', wide)
            print('\n'.join(errs[:15]))
            await ctx.close()
        await b.close()
asyncio.run(run())
