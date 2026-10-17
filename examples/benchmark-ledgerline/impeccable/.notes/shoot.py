import sys, asyncio
from playwright.async_api import async_playwright
URL = "examples/benchmark-ledgerline/impeccable/index.html"
OUT = "examples/benchmark-ledgerline/impeccable/.impeccable/review/"
async def main():
    async with async_playwright() as p:
        b = await p.chromium.launch()
        for name,w,h in [("desktop",1440,900),("tablet",768,1024),("mobile",375,812),("mobile390",390,844)]:
            pg = await b.new_page(viewport={"width":w,"height":h})
            errs=[]; pg.on("console", lambda m: errs.append(m.text) if m.type=="error" else None); pg.on("pageerror", lambda e: errs.append(str(e)))
            await pg.goto(URL); await pg.evaluate("document.fonts.ready"); await pg.wait_for_timeout(3500)
            ov = await pg.evaluate("document.documentElement.scrollWidth - window.innerWidth")
            await pg.screenshot(path=OUT+name+"-fold.png")
            await pg.screenshot(path=OUT+name+".png", full_page=True)
            print(name, "overflowX", ov, "errors", errs)
            await pg.close()
        await b.close()
asyncio.run(main())
