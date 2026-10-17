# Dev-only: viewport captures of the 3D story at several widths.
import sys, pathlib, asyncio
from playwright.async_api import async_playwright
here = pathlib.Path(__file__).parent
url = (here.parent / "index.html").resolve().as_uri()
out = here / "shots"; out.mkdir(exist_ok=True)
which = sys.argv[1] if len(sys.argv) > 1 else "all"

async def run():
    async with async_playwright() as p:
        b = await p.chromium.launch(args=["--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"])
        for name, w, h in [("d", 1440, 900), ("t", 768, 1024), ("m", 375, 812)]:
            if which != "all" and which != name: continue
            pg = await b.new_page(viewport={"width": w, "height": h}, device_scale_factor=1)
            msgs = []
            pg.on("console", lambda m: msgs.append(m.type + ": " + m.text))
            pg.on("pageerror", lambda e: msgs.append("PAGEERROR " + str(e)))
            await pg.goto(url, wait_until="networkidle")
            try:
                await pg.wait_for_selector(".stage.is-gl", timeout=15000)
            except Exception:
                msgs.append("NO GL")
            await pg.wait_for_timeout(1500)
            await pg.screenshot(path=str(out / f"{name}-0-hero.png"))
            for ch in ["battery", "belt", "weight", "colours"]:
                await pg.evaluate(f"""(()=>{{const el=document.querySelector('.street__text [data-chapter="{ch}"]');const r=el.getBoundingClientRect();window.scrollTo(0, window.scrollY + r.top - (innerWidth<900? innerHeight*0.5 : 80));}})()""")
                await pg.wait_for_timeout(2600)
                await pg.screenshot(path=str(out / f"{name}-{ch}.png"))
            for sel in ["#specs", ".quotes", "#test-ride", ".faq", ".site-footer"]:
                await pg.evaluate(f"document.querySelector('{sel}').scrollIntoView({{behavior:'instant'}})")
                await pg.wait_for_timeout(500)
                await pg.screenshot(path=str(out / f"{name}-{sel.strip('#.')}.png"))
            print(name, msgs[:12])
            await pg.close()
        await b.close()
asyncio.run(run())
