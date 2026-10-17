import pathlib, re
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").as_uri()
src=open(r'scripts\browser-qa.mjs',encoding='utf-8').read()
SP=re.search(r'const SCROLL_PASS_SCRIPT = `(.*?)`;',src,re.S).group(1)
CNT="""(()=>{let n=0;const w=[];for(const el of document.querySelectorAll("h1, h2, h3, p, li, a, button")){if(!(el.textContent||'').trim()||el.closest("[aria-hidden='true'], [hidden]"))continue;const r=el.getBoundingClientRect();if(r.width<=2||r.height<=2)continue;let cur=el,f=false;while(cur&&cur.nodeType===1){const cs=getComputedStyle(cur);if(cs.display==='none'||cs.visibility==='hidden'){f=false;break}if(parseFloat(cs.opacity)===0)f=true;cur=cur.parentElement}if(f){n++;w.push(el.tagName+'.'+el.className+':'+el.textContent.trim().slice(0,15))}}return [n,performance.now()|0,w.slice(0,4)]})()"""
with sync_playwright() as p:
    b=p.chromium.launch(); c=b.new_context(viewport={"width":1440,"height":900}); pg=c.new_page()
    pg.goto(url,wait_until="load"); pg.evaluate("document.fonts.ready.then(()=>new Promise(r=>setTimeout(r,1000)))")
    pg.evaluate(SP); print('after pass',pg.evaluate(CNT))
    pg.screenshot(path="fp.png",full_page=True); print('after fullpage',pg.evaluate(CNT))
    pg.wait_for_timeout(1500); print('later',pg.evaluate(CNT))
    b.close()
