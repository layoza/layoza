import pathlib, json, re
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").as_uri()
src=open(r'scripts\browser-qa.mjs',encoding='utf-8').read()
m=re.search(r'const SCROLL_PASS_SCRIPT = `(.*?)`;',src,re.S); SP=m.group(1)
CHK="""(()=>{const out=[];for(const el of document.querySelectorAll("h1, h2, h3, p, li, a, button")){if(!(el.textContent||'').trim()||el.closest("[aria-hidden='true'], [hidden]"))continue;const r=el.getBoundingClientRect();if(r.width<=2||r.height<=2)continue;let cur=el,f=false,who='';while(cur&&cur.nodeType===1){const cs=getComputedStyle(cur);if(cs.display==='none'||cs.visibility==='hidden'){f=false;break}if(parseFloat(cs.opacity)===0){f=true;who=cur.className}cur=cur.parentElement}if(f)out.push(el.tagName+':'+el.textContent.trim().slice(0,25)+' <- '+who)}return out})()"""
with sync_playwright() as p:
    b=p.chromium.launch(); c=b.new_context(viewport={"width":1440,"height":900}); pg=c.new_page()
    pg.goto(url,wait_until="load"); pg.evaluate("document.fonts.ready.then(()=>new Promise(r=>setTimeout(r,1000)))")
    pg.evaluate(SP)
    r=pg.evaluate(CHK); print(len(r)); print("\n".join(r[:45]))
    b.close()
