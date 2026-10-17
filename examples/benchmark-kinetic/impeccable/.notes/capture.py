import sys, json, asyncio, os
from playwright.async_api import async_playwright
URL = sys.argv[1]; OUT = sys.argv[2]
os.makedirs(OUT + '/frames', exist_ok=True)
SECTIONS = ['.hero', '#about', '#services', '#work', '.philo', '.process', '.reel', '#insights', '#contact', '.footer']
async def settle(page, ms=900):
    await page.wait_for_timeout(ms)
async def scroll_to(page, y):
    await page.evaluate("""y => { if (window.__lenis) window.__lenis.scrollTo(y, {immediate:true, force:true}); else window.scrollTo(0, y); }""", y)
async def run(pw, w, h, tag, rm=False, frames=True):
    b = await pw.chromium.launch(args=['--use-angle=swiftshader','--enable-unsafe-swiftshader'])
    ctx = await b.new_context(viewport={'width': w, 'height': h}, device_scale_factor=1, reduced_motion='reduce' if rm else 'no-preference', has_touch=(w < 800), is_mobile=(w < 800))
    page = await ctx.new_page()
    logs = []
    page.on('console', lambda m: logs.append(f'{m.type}: {m.text}') if m.type in ('error','warning') else None)
    page.on('pageerror', lambda e: logs.append(f'pageerror: {e}'))
    await page.goto(URL, wait_until='networkidle')
    await page.evaluate('document.fonts.ready')
    await settle(page, 2600)
    info = await page.evaluate("""() => ({sw: document.documentElement.scrollWidth, iw: innerWidth, H: document.documentElement.scrollHeight,
        webgl: !!document.querySelector('#ribbon canvas'), nowebgl: document.documentElement.classList.contains('no-webgl'),
        wide: [...document.querySelectorAll('body *')].filter(e => { const r = e.getBoundingClientRect(); return r.right > innerWidth + 1 && getComputedStyle(e).position !== 'fixed' && !e.closest('.ct-ribbon,.philo-line,.ft-mark,.p4,.ribbon-stage,.fold,.reel-stage'); }).slice(0,8).map(e => e.tagName + '.' + e.className.toString().slice(0,40))})""")
    if frames:
        await page.screenshot(path=f'{OUT}/frames/{tag}-00-hero.png', timeout=120000)
        tops = await page.evaluate("""s => s.map(q => { const e = document.querySelector(q); return e ? [q, e.getBoundingClientRect().top + scrollY, e.offsetHeight] : null }).filter(Boolean)""", SECTIONS[1:])
        n = 1
        for q, top, hgt in tops:
            pts = [top]
            if hgt > h * 1.6: pts = [top + f * (hgt - h) for f in (0.08, 0.45, 0.85)]
            if q == '#work': pts = [top + h*0.1, top + h*1.1, top + h*2.1, top + hgt - h]
            for k, y in enumerate(pts):
                # pass through earlier positions so 'once' triggers fire
                await scroll_to(page, max(0, y - h*0.6)); await settle(page, 350)
                await scroll_to(page, y); await settle(page, 1300)
                name = q.strip('.#') + (f'-{k}' if len(pts) > 1 else '')
                await page.screenshot(path=f'{OUT}/frames/{tag}-{n:02d}-{name}.png'); n += 1
    if rm:
        await page.screenshot(path=f'{OUT}/{tag}.png', full_page=True, timeout=120000)
    await b.close()
    return {'tag': tag, **info, 'logs': logs[:20]}
async def main():
    async with async_playwright() as pw:
        res = []
        res.append(await run(pw, 1440, 900, 'd1440'))
        res.append(await run(pw, 768, 1024, 't768'))
        res.append(await run(pw, 375, 812, 'm375'))
        res.append(await run(pw, 1440, 900, 'desktop', rm=True, frames=False))
        res.append(await run(pw, 390, 844, 'mobile', rm=True, frames=False))
        print(json.dumps(res, indent=1))
asyncio.run(main())
