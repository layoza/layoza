# Dev-only: capture reduced-motion and no-WebGL states.
import pathlib, asyncio
from playwright.async_api import async_playwright
here = pathlib.Path(__file__).parent; url=(here.parent/"index.html").resolve().as_uri(); out=here/"shots"
async def run():
    async with async_playwright() as p:
        b = await p.chromium.launch(args=["--disable-webgl","--disable-3d-apis"])
        pg = await b.new_page(viewport={"width":1440,"height":900})
        errs=[]; pg.on("pageerror", lambda e: errs.append(str(e)))
        await pg.goto(url, wait_until="networkidle"); await pg.wait_for_timeout(1500)
        await pg.screenshot(path=str(out/"nogl-hero.png"))
        await pg.click("label[for=paint-clay]"); await pg.evaluate("document.getElementById('colours').scrollIntoView()"); await pg.wait_for_timeout(800)
        await pg.screenshot(path=str(out/"nogl-colours.png"))
        print("nogl", errs, await pg.evaluate("document.querySelector('.stage').className"))
        await b.close()
        b = await p.chromium.launch(args=["--use-gl=angle","--use-angle=swiftshader","--enable-unsafe-swiftshader"])
        ctx = await b.new_context(viewport={"width":375,"height":812}, reduced_motion="reduce")
        pg = await ctx.new_page(); errs=[]; pg.on("pageerror", lambda e: errs.append(str(e)))
        await pg.goto(url, wait_until="networkidle"); await pg.wait_for_timeout(2500)
        await pg.evaluate("window.scrollTo(0, document.querySelector('[data-chapter=battery]').getBoundingClientRect().top + scrollY - 400)"); await pg.wait_for_timeout(1500)
        await pg.screenshot(path=str(out/"rm-battery-m.png"))
        # form: submit empty then valid
        await pg.evaluate("document.getElementById('test-ride').scrollIntoView()")
        await pg.click("#booking button[type=submit]"); await pg.wait_for_timeout(400)
        await pg.screenshot(path=str(out/"form-errors-m.png"))
        await pg.fill("#f-name","Sam Rider"); await pg.fill("#f-email","sam@example.com"); await pg.select_option("#f-city","Bath")
        mn = await pg.evaluate("document.getElementById('f-date').min"); await pg.fill("#f-date", mn)
        await pg.click("#booking button[type=submit]"); await pg.wait_for_timeout(1200)
        await pg.evaluate("document.getElementById('booking').scrollIntoView()"); await pg.wait_for_timeout(300)
        await pg.screenshot(path=str(out/"form-success-m.png"))
        print("rm", errs)
        await b.close()
asyncio.run(run())
