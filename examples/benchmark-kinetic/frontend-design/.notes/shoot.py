import sys, os, json, asyncio
from playwright.async_api import async_playwright
URL = sys.argv[1] if len(sys.argv) > 1 else 'http://127.0.0.1:8765/index.html'
OUT = os.path.join(os.path.dirname(__file__), 'shots'); os.makedirs(OUT, exist_ok=True)
async def run(w, h, name, reduced=False):
    async with async_playwright() as p:
        b = await p.chromium.launch(args=['--use-gl=angle','--enable-webgl','--ignore-gpu-blocklist'])
        ctx = await b.new_context(viewport={'width': w, 'height': h}, reduced_motion='reduce' if reduced else 'no-preference', device_scale_factor=1)
        pg = await ctx.new_page()
        errs = []
        pg.on('console', lambda m: errs.append(m.type + ': ' + m.text) if m.type in ('error','warning') else None)
        pg.on('pageerror', lambda e: errs.append('pageerror: ' + str(e)))
        await pg.goto(URL, wait_until='networkidle')
        await pg.wait_for_timeout(2600)
        await pg.screenshot(path=f'{OUT}/{name}-00-hero.png')
        H = await pg.evaluate('document.documentElement.scrollHeight')
        info = await pg.evaluate('''() => ({ sw: document.documentElement.scrollWidth, cw: document.documentElement.clientWidth, live: !!document.querySelector('.hero-visual.is-live'), lenis: !!document.querySelector('html.lenis') })''')
        stops = [int(x) for x in sys.argv[2].split(',')] if len(sys.argv) > 2 else None
        n = 1; y = 0
        targets = []
        for sel in ['#about', '.svc-scroll', '#work', '.p-2', '.p-4', '.phil-scroll', '.proc-scroll', '.reel', '#insights', '#contact', '.footer']:
            top = await pg.evaluate(f"(() => {{ const e=document.querySelector('{sel}'); return e ? e.getBoundingClientRect().top + scrollY : null }})()")
            targets.append((sel, top))
        extra = []
        for sel, top in targets:
            extra.append((sel, top))
            if sel in ('.svc-scroll', '.phil-scroll', '.proc-scroll'):
                for f in (0.45, 0.8, 1.0):
                    extra.append((sel + f'-{f}', top + f * (await pg.evaluate(f"document.querySelector('{sel}').offsetHeight - innerHeight"))))
        for sel, top in extra:
            if top is None: continue
            # step through to let scrubs settle
            await pg.evaluate(f'window.scrollTo(0, {int(top)})')
            await pg.wait_for_timeout(1400)
            safe = sel.replace('#','').replace('.','').replace(' ','')
            await pg.screenshot(path=f'{OUT}/{name}-{n:02d}-{safe}.png'); n += 1
        print(name, json.dumps(info), 'H=', H)
        for e in errs[:20]: print('  ', e)
        await b.close()
async def main():
    which = os.environ.get('WHICH', 'all')
    if which in ('all','d'): await run(1440, 900, 'd')
    if which in ('all','t'): await run(768, 1024, 't')
    if which in ('all','m'): await run(375, 812, 'm')
    if which in ('all','r'): await run(1440, 900, 'r', reduced=True)
asyncio.run(main())
