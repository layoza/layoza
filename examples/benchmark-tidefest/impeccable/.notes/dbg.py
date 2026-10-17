import asyncio
from pathlib import Path
from playwright.async_api import async_playwright
url=(Path('.').resolve()/'index.html').as_uri()
async def m():
  async with async_playwright() as p:
    b=await p.chromium.launch(); pg=await b.new_page(viewport={'width':1440,'height':900})
    await pg.goto(url); await pg.wait_for_timeout(800)
    await pg.evaluate("(()=>{const s=document.getElementById('sunrise');scrollTo(0,s.offsetTop+(s.offsetHeight-innerHeight)*0.6)})()"); await pg.wait_for_timeout(300)
    print(await pg.evaluate("(()=>{const s=document.querySelector('.sun');const c=getComputedStyle(s);return [c.transform,JSON.stringify(s.getBoundingClientRect()),c.getPropertyValue('--p')]})()"))
    await b.close()
asyncio.run(m())
