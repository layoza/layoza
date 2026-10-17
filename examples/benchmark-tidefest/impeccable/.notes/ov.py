import asyncio,sys
from pathlib import Path
from playwright.async_api import async_playwright
url=(Path('.').resolve()/'index.html').as_uri()
async def m():
  async with async_playwright() as p:
    b=await p.chromium.launch()
    for w,h in [(1440,900),(375,812)]:
      pg=await b.new_page(viewport={'width':w,'height':h})
      await pg.goto(url); await pg.evaluate('document.fonts.ready'); await pg.wait_for_timeout(800)
      print(w, await pg.evaluate("""[...document.querySelectorAll('body *')].filter(e=>e.getBoundingClientRect().right>innerWidth+1 && !e.closest('svg')).map(e=>e.tagName+'.'+e.className+':'+Math.round(e.getBoundingClientRect().right)).slice(0,12)"""))
    await b.close()
asyncio.run(m())
