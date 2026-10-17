import sys
from playwright.sync_api import sync_playwright
from pathlib import Path
url = Path(sys.argv[1]).resolve().as_uri()
with sync_playwright() as p:
    b = p.chromium.launch(); pg = b.new_page(viewport={'width':1440,'height':900})
    pg.goto(url); pg.wait_for_timeout(1200)
    print(pg.evaluate("""() => { const base = document.documentElement.scrollHeight; const out=[];
      for (const e of document.querySelectorAll('main *, footer *')) { const d=e.style.display; e.style.display='none'; const h=document.documentElement.scrollHeight; e.style.display=d;
        if (h < base - 50 && h > base - 200) out.push(e.tagName+'.'+e.className+'#'+e.id+' '+(base-h)); }
      return {base, out: out.slice(-8)}; }"""))
    b.close()
