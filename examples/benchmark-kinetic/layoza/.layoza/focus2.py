import pathlib, json
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").as_uri()
JS="""(()=>{const out=[];const bars=[document.querySelector('header')];
const f=[...document.querySelectorAll("a[href], button, input:not([type=hidden]), select, textarea")];
for(const el of [...f,...f.slice().reverse()]){const cs=getComputedStyle(el); if(cs.visibility==='hidden')continue;
el.scrollIntoView({block:'nearest',inline:'nearest',behavior:'instant'});el.focus({preventScroll:true});if(document.activeElement!==el)continue;
const r=el.getBoundingClientRect(),br=bars[0].getBoundingClientRect();
if(r.top<br.bottom-2&&r.bottom>br.top)out.push([el.tagName,(el.textContent||el.id).slice(0,20),Math.round(r.top),Math.round(r.bottom),Math.round(br.bottom),Math.round(scrollY),getComputedStyle(document.documentElement).scrollPaddingTop])}
return out})()"""
with sync_playwright() as p:
    b=p.chromium.launch(); pg=b.new_page(viewport={"width":1440,"height":900})
    pg.goto(url,wait_until="load"); pg.wait_for_timeout(2500)
    h=pg.evaluate("document.documentElement.scrollHeight")
    for y in range(0,h,600): pg.evaluate(f"scrollTo(0,{y})"); pg.wait_for_timeout(120)
    pg.wait_for_timeout(1500)
    for r in pg.evaluate(JS): print(r)
    b.close()
