import asyncio
from pathlib import Path
from playwright.async_api import async_playwright
url=(Path('.').resolve()/'index.html').as_uri()
async def m():
  async with async_playwright() as p:
    b=await p.chromium.launch()
    pg=await b.new_page(viewport={'width':1440,'height':900})
    await pg.goto(url); await pg.evaluate('document.fonts.ready'); await pg.wait_for_timeout(800)
    print(await pg.evaluate("[document.documentElement.scrollWidth, document.body.scrollWidth]"))
    print(await pg.evaluate("""[...document.querySelectorAll('*')].filter(e=>{const r=e.getBoundingClientRect();return r.right>innerWidth+1}).map(e=>e.tagName+'.'+(e.className.baseVal??e.className)+':'+Math.round(e.getBoundingClientRect().right)).slice(0,12)"""))
    await b.close()
asyncio.run(m())
